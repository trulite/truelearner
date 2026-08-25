#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, Core0Profile, MechanicalConfig,
    PhysicalEvent, PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode,
    TransmissionTrigger,
};

const HIGH_RESISTANCE: u32 = 10_000;
const EXPERIENCES: usize = 2;
const DEPTHS: [usize; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const LIFETIMES: [u32; 6] = [1, 2, 4, 8, 16, 32];
const DELAYS: [i64; 12] = [0, 1, 2, 4, 8, 12, 16, 24, 32, 48, 64, 96];
const NOISE_PAIRS: [usize; 8] = [0, 1, 2, 4, 8, 16, 32, 64];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Depth,
    Lifetime,
    Delay,
    Variation,
}

impl Family {
    fn name(self) -> &'static str {
        match self {
            Self::Depth => "depth",
            Self::Lifetime => "lifetime",
            Self::Delay => "delay",
            Self::Variation => "variation",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Case {
    family: Family,
    depth: usize,
    resistance: u32,
    delay: i64,
    noise_pairs: usize,
}

fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    cases.extend(DEPTHS.map(|depth| Case {
        family: Family::Depth,
        depth,
        resistance: 8,
        delay: 4,
        noise_pairs: 0,
    }));
    cases.extend(LIFETIMES.map(|resistance| Case {
        family: Family::Lifetime,
        depth: 8,
        resistance,
        delay: 12,
        noise_pairs: 0,
    }));
    cases.extend(DELAYS.map(|delay| Case {
        family: Family::Delay,
        depth: 8,
        resistance: 8,
        delay,
        noise_pairs: 0,
    }));
    cases.extend(NOISE_PAIRS.map(|noise_pairs| Case {
        family: Family::Variation,
        depth: 8,
        resistance: 8,
        delay: 4,
        noise_pairs,
    }));
    cases
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Experience {
    action_crossings: usize,
    desired_live_before: usize,
    qlp_live_before: usize,
    participating_before: usize,
    desired_supported: usize,
    backward_supported_depth: usize,
    desired_qlp_traversals: usize,
    noise_supported: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    experiences: Vec<Experience>,
    full_closure: bool,
    work: u64,
    deallocations: u64,
    final_tick: i64,
    pending: usize,
    quiescent: bool,
    body_hash: String,
    trace_hash: String,
    trace: Vec<PhysicalTransition>,
}

struct Geometry {
    body: PlasticSubstrate,
    source: CellId,
    returning: CellId,
    desired_drive: Vec<ArrowId>,
    desired_qlp: Vec<ArrowId>,
    noise_drive: Vec<ArrowId>,
}

struct Builder {
    body: PlasticSubstrate,
    next_physical: u64,
    next_position: i32,
}

impl Builder {
    fn new(root: u64, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 1024, 8192, mechanics);
        body.set_core0_profile(Core0Profile::GenericExternal);
        body.set_physical_tracing(true);
        Self {
            body,
            next_physical: root.saturating_add(1),
            next_position: 0,
        }
    }

    fn cell(&mut self, region: i16, threshold: i32) -> CellId {
        let id = self.body.add_cell(CellSpec {
            physical_id: self.next_physical,
            position: self.next_position,
            region,
            threshold,
            resistance: HIGH_RESISTANCE,
        });
        self.next_physical = self.next_physical.saturating_add(1);
        self.next_position = self.next_position.saturating_add(10);
        id
    }

    fn arrow(
        &mut self,
        from: CellId,
        to: CellId,
        coupling: i32,
        resistance: u32,
        mode: TransmissionMode,
    ) -> ArrowId {
        self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay: 0,
            phase: 0,
            coupling,
            resistance,
            mode,
        })
    }

    fn qlp(&mut self, from: CellId, to: CellId, resistance: u32) -> ArrowId {
        self.body.add_arrow_with_trigger(
            ArrowSpec {
                from,
                to,
                delay: 0,
                phase: 0,
                coupling: 1,
                resistance,
                mode: TransmissionMode::Modulatory,
            },
            TransmissionTrigger::QualifiedLocalParticipation,
        )
    }
}

fn build(root: u64, mechanics: MechanicalConfig, case: Case) -> Geometry {
    let mut builder = Builder::new(root, mechanics);
    let anchor = builder.cell(0, 100);
    let source = builder.cell(0, 1);
    let contacts = (0..case.depth)
        .map(|_| builder.cell(0, 1))
        .collect::<Vec<_>>();
    let motor = builder.cell(0, 1);
    let output = builder.cell(1, 1);
    let returning = builder.cell(0, 1);

    builder.arrow(anchor, source, 1, HIGH_RESISTANCE, TransmissionMode::Drive);
    builder.arrow(
        anchor,
        returning,
        1,
        HIGH_RESISTANCE,
        TransmissionMode::Drive,
    );
    builder.arrow(motor, output, 1, HIGH_RESISTANCE, TransmissionMode::Drive);
    builder.arrow(
        returning,
        motor,
        1,
        HIGH_RESISTANCE,
        TransmissionMode::Modulatory,
    );

    let mut desired_drive = Vec::with_capacity(case.depth.saturating_add(1));
    desired_drive.push(builder.arrow(
        source,
        contacts[0],
        1,
        case.resistance,
        TransmissionMode::Drive,
    ));
    for index in 0..case.depth {
        let target = if index + 1 < case.depth {
            contacts[index + 1]
        } else {
            motor
        };
        desired_drive.push(builder.arrow(
            contacts[index],
            target,
            1,
            case.resistance,
            TransmissionMode::Drive,
        ));
    }

    let mut desired_qlp = Vec::with_capacity(case.depth.saturating_add(1));
    desired_qlp.push(builder.qlp(motor, contacts[case.depth - 1], case.resistance));
    for index in (1..case.depth).rev() {
        desired_qlp.push(builder.qlp(contacts[index], contacts[index - 1], case.resistance));
    }
    desired_qlp.push(builder.qlp(contacts[0], source, case.resistance));

    let mut noise_drive = Vec::with_capacity(case.noise_pairs.saturating_mul(4));
    for _ in 0..case.noise_pairs {
        for sign in [1, -1] {
            let contact = builder.cell(0, 1);
            noise_drive.push(builder.arrow(
                source,
                contact,
                1,
                case.resistance,
                TransmissionMode::Drive,
            ));
            noise_drive.push(builder.arrow(
                contact,
                motor,
                sign,
                case.resistance,
                TransmissionMode::Drive,
            ));
            builder.qlp(motor, contact, case.resistance);
            builder.qlp(contact, source, case.resistance);
        }
    }

    Geometry {
        body: builder.body,
        source,
        returning,
        desired_drive,
        desired_qlp,
        noise_drive,
    }
}

fn pulse(
    body: &mut PlasticSubstrate,
    target: CellId,
    tick: i64,
    origin: u64,
) -> truelearner_core::RunResult {
    body.arrive(
        &[SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: origin,
            target,
            impulse: 1,
        }],
        1,
    )
}

fn live(body: &PlasticSubstrate, id: ArrowId) -> bool {
    body.arena_body(0)
        .arrows
        .iter()
        .find(|arrow| arrow.id == id)
        .is_some_and(|arrow| arrow.live)
}

fn count_qlp(trace: &[PhysicalTransition], ids: &[ArrowId]) -> usize {
    trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::QualifiedLocalTraversal { arrow } if ids.contains(&arrow)
            )
        })
        .count()
}

fn execute(root: u64, mechanics: MechanicalConfig, case: Case) -> Observation {
    let mut geometry = build(root, mechanics, case);
    let mut experiences = Vec::with_capacity(EXPERIENCES);
    let mut trace = Vec::new();
    let mut work = 0_u64;
    let mut deallocations = 0_u64;
    let mut quiescent = true;

    for experience in 0..EXPERIENCES {
        if experience > 0 {
            let recovery = geometry.body.clock().tick.saturating_add(1);
            geometry.body.advance_time(recovery);
        }
        let start = geometry.body.clock().tick;
        let forward = pulse(
            &mut geometry.body,
            geometry.source,
            start,
            root.saturating_add(80_000 + u64::try_from(experience).unwrap_or(0) * 10),
        );
        let action_crossings = forward.crossings.len();
        work = work.saturating_add(forward.work.physical_total());
        deallocations = deallocations.saturating_add(forward.work.physical_deallocations);
        quiescent &= forward.naturally_quiescent;
        trace.extend(forward.physical_trace);

        let consequence_tick = geometry.body.clock().tick.saturating_add(case.delay);
        geometry.body.advance_time(consequence_tick);
        let desired_live_before = geometry
            .desired_drive
            .iter()
            .filter(|id| live(&geometry.body, **id))
            .count();
        let qlp_live_before = geometry
            .desired_qlp
            .iter()
            .filter(|id| live(&geometry.body, **id))
            .count();
        let participating_before = geometry
            .desired_drive
            .iter()
            .filter(|id| live(&geometry.body, **id) && geometry.body.local_participation(**id) > 0)
            .count();
        let resistance_before = geometry
            .desired_drive
            .iter()
            .map(|id| geometry.body.core0_resistance_material(*id))
            .collect::<Vec<_>>();
        let noise_before = geometry
            .noise_drive
            .iter()
            .map(|id| geometry.body.core0_resistance_material(*id))
            .collect::<Vec<_>>();

        let consequence = pulse(
            &mut geometry.body,
            geometry.returning,
            consequence_tick,
            root.saturating_add(90_000 + u64::try_from(experience).unwrap_or(0) * 10),
        );
        work = work.saturating_add(consequence.work.physical_total());
        deallocations = deallocations.saturating_add(consequence.work.physical_deallocations);
        quiescent &= consequence.naturally_quiescent;
        let desired_supported_flags = geometry
            .desired_drive
            .iter()
            .zip(&resistance_before)
            .map(|(id, before)| geometry.body.core0_resistance_material(*id) > *before)
            .collect::<Vec<_>>();
        let desired_supported = desired_supported_flags
            .iter()
            .filter(|value| **value)
            .count();
        let backward_supported_depth = desired_supported_flags
            .iter()
            .rev()
            .take_while(|supported| **supported)
            .count();
        let desired_qlp_traversals = count_qlp(&consequence.physical_trace, &geometry.desired_qlp);
        let noise_supported = geometry
            .noise_drive
            .iter()
            .zip(noise_before)
            .filter(|(id, before)| geometry.body.core0_resistance_material(**id) > *before)
            .count();
        trace.extend(consequence.physical_trace);
        experiences.push(Experience {
            action_crossings,
            desired_live_before,
            qlp_live_before,
            participating_before,
            desired_supported,
            backward_supported_depth,
            desired_qlp_traversals,
            noise_supported,
        });
    }

    let desired_count = geometry.desired_drive.len();
    let qlp_count = geometry.desired_qlp.len();
    let full_closure = experiences.iter().all(|experience| {
        experience.action_crossings == 1
            && experience.desired_live_before == desired_count
            && experience.qlp_live_before == qlp_count
            && experience.participating_before == desired_count
            && experience.desired_supported == desired_count
            && experience.backward_supported_depth == desired_count
            && experience.desired_qlp_traversals == qlp_count
    }) && quiescent
        && geometry.body.pending_physical_activity() == 0;
    let trace_hash = ContentHash::of(format!("{trace:?}").as_bytes()).to_string();
    Observation {
        experiences,
        full_closure,
        work,
        deallocations,
        final_tick: geometry.body.clock().tick,
        pending: geometry.body.pending_physical_activity(),
        quiescent,
        body_hash: ContentHash::of(&geometry.body.canonical_body_bytes(0).unwrap()).to_string(),
        trace_hash,
        trace,
    }
}

fn list(values: impl IntoIterator<Item = usize>) -> String {
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == "--preflight") {
        let case = Case {
            family: Family::Depth,
            depth: 64,
            resistance: 8,
            delay: 4,
            noise_pairs: 0,
        };
        let observation = execute(91_000_000, MechanicalConfig::REFERENCE, case);
        println!(
            "family={} depth={} full={} backward={} qlp={} work={} tick={}",
            case.family.name(),
            case.depth,
            observation.full_closure,
            list(
                observation
                    .experiences
                    .iter()
                    .map(|experience| experience.backward_supported_depth)
            ),
            list(
                observation
                    .experiences
                    .iter()
                    .map(|experience| experience.desired_qlp_traversals)
            ),
            observation.work,
            observation.final_tick
        );
        return;
    }

    eprintln!("CORE1_E13D_COMPOSITION_BOUNDARY_V1_EVIDENCE_SPENT");
    let destination = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_e13d_boundary_v1"));
    fs::create_dir_all(&destination).expect("create E13-D destination");
    let mut csv =
        BufWriter::new(File::create(destination.join("matrix.csv")).expect("create matrix"));
    writeln!(
        csv,
        "family,depth,resistance,delay,noise_pairs,full_closure,action_crossings,desired_live_before,qlp_live_before,participating_before,desired_supported,backward_supported_depth,desired_qlp_traversals,noise_supported,physical_work,deallocations,final_tick,pending,quiescent,replay_exact,mechanics_exact,body_hash,trace_hash"
    )
    .expect("write header");

    let mut report = String::from(
        "# CORE1 E13-D composition boundary result\n\n| Family | Value | Full closure | Backward depth | Work | Replay | Mechanics |\n|---|---:|---:|---|---:|---:|---:|\n",
    );
    for (index, case) in cases().into_iter().enumerate() {
        let root = 92_000_000_u64.saturating_add(u64::try_from(index).unwrap_or(0) * 10_000);
        let reference = execute(root, MechanicalConfig::REFERENCE, case);
        let replay = execute(root, MechanicalConfig::REFERENCE, case);
        let production = execute(root, MechanicalConfig::PRODUCTION, case);
        let replay_exact = reference == replay;
        let mechanics_exact = reference == production;
        let value = match case.family {
            Family::Depth => case.depth.to_string(),
            Family::Lifetime => case.resistance.to_string(),
            Family::Delay => case.delay.to_string(),
            Family::Variation => case.noise_pairs.to_string(),
        };
        writeln!(
            csv,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            case.family.name(),
            case.depth,
            case.resistance,
            case.delay,
            case.noise_pairs,
            reference.full_closure,
            list(
                reference
                    .experiences
                    .iter()
                    .map(|item| item.action_crossings)
            ),
            list(
                reference
                    .experiences
                    .iter()
                    .map(|item| item.desired_live_before)
            ),
            list(
                reference
                    .experiences
                    .iter()
                    .map(|item| item.qlp_live_before)
            ),
            list(
                reference
                    .experiences
                    .iter()
                    .map(|item| item.participating_before)
            ),
            list(
                reference
                    .experiences
                    .iter()
                    .map(|item| item.desired_supported)
            ),
            list(
                reference
                    .experiences
                    .iter()
                    .map(|item| item.backward_supported_depth)
            ),
            list(
                reference
                    .experiences
                    .iter()
                    .map(|item| item.desired_qlp_traversals)
            ),
            list(
                reference
                    .experiences
                    .iter()
                    .map(|item| item.noise_supported)
            ),
            reference.work,
            reference.deallocations,
            reference.final_tick,
            reference.pending,
            reference.quiescent,
            replay_exact,
            mechanics_exact,
            reference.body_hash,
            reference.trace_hash,
        )
        .expect("write row");
        csv.flush().expect("flush row");
        writeln!(
            report,
            "| {} | {} | {} | {} | {} | {} | {} |",
            case.family.name(),
            value,
            reference.full_closure,
            list(
                reference
                    .experiences
                    .iter()
                    .map(|item| item.backward_supported_depth)
            ),
            reference.work,
            replay_exact,
            mechanics_exact,
        )
        .expect("write report row");
        fs::write(destination.join("report.md"), &report).expect("stream report");
    }
    println!("CORE1_E13D_COMPOSITION_BOUNDARY_V1_COMPLETE cases=34 experiences=2");
}
