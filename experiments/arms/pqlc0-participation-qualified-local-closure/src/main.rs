#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig,
    PhysicalEvent, PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode,
    TransmissionTrigger,
};

const ROOTS: [u64; 2] = [2_500_000, 2_600_000];
const EXPECTED_PHYSICAL_CASES: usize = 200;
const EXPECTED_MECHANICS_ROWS: usize = 400;
const CYCLE_WORK_CEILING: u64 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum World {
    PositiveOneHop,
    NeverParticipated,
    ExpiredParticipation,
    WrongPath,
    UnrelatedActivity,
    TwoUpstreamOneParticipated,
    ContactFanout,
    NoPrematureForward,
    DriveNotConsequence,
    ClosureCycle,
}

impl World {
    const ALL: [Self; 10] = [
        Self::PositiveOneHop,
        Self::NeverParticipated,
        Self::ExpiredParticipation,
        Self::WrongPath,
        Self::UnrelatedActivity,
        Self::TwoUpstreamOneParticipated,
        Self::ContactFanout,
        Self::NoPrematureForward,
        Self::DriveNotConsequence,
        Self::ClosureCycle,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::PositiveOneHop => "positive_one_hop",
            Self::NeverParticipated => "never_participated",
            Self::ExpiredParticipation => "expired_participation",
            Self::WrongPath => "wrong_path",
            Self::UnrelatedActivity => "unrelated_activity",
            Self::TwoUpstreamOneParticipated => "two_upstream_one_participated",
            Self::ContactFanout => "contact_fanout",
            Self::NoPrematureForward => "no_premature_forward",
            Self::DriveNotConsequence => "drive_not_consequence",
            Self::ClosureCycle => "closure_cycle",
        }
    }
}

enum Step {
    Arrive(Vec<SpikeInput>),
    Advance(i64),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    updates: u64,
    qlp: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<PhysicalTransition>,
    participation: Vec<u64>,
    support: Vec<u64>,
    triggers: Vec<TransmissionTrigger>,
    work: WorkTotals,
    qlp_events: u64,
    source_fires: u64,
    final_tick: i64,
    pressure_phase: i64,
    body_hash: String,
    live: bool,
    quiescent: bool,
}

struct Geometry {
    body: PlasticSubstrate,
    contacts: Vec<ArrowId>,
    qlp_arrows: Vec<ArrowId>,
    steps: Vec<Step>,
}

struct Builder {
    body: PlasticSubstrate,
    root: u64,
    phase: i64,
    next_physical: u64,
    next_position: i32,
}

impl Builder {
    fn new(root: u64, phase: i64, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root + 500), 64, 128, mechanics);
        body.set_physical_tracing(true);
        if phase > 0 {
            body.advance_time(phase);
        }
        Self {
            body,
            root,
            phase,
            next_physical: root + 1,
            next_position: 0,
        }
    }

    fn cell(&mut self, threshold: i32) -> CellId {
        let id = self.body.add_cell(CellSpec {
            physical_id: self.next_physical,
            position: self.next_position,
            region: 0,
            threshold,
            resistance: 100_000,
        });
        self.next_physical += 1;
        self.next_position += 10;
        id
    }

    fn arrow(&mut self, from: CellId, to: CellId, mode: TransmissionMode) -> ArrowId {
        self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: 100_000,
            mode,
        })
    }

    fn qlp(&mut self, from: CellId, to: CellId, delay: i64) -> ArrowId {
        self.body.add_arrow_with_trigger(
            ArrowSpec {
                from,
                to,
                delay,
                phase: 0,
                coupling: 1,
                resistance: 100_000,
                mode: TransmissionMode::Modulatory,
            },
            TransmissionTrigger::QualifiedLocalParticipation,
        )
    }

    fn source(&mut self, target: CellId) -> CellId {
        let source = self.cell(1);
        self.arrow(source, target, TransmissionMode::Drive);
        source
    }

    fn spike(&self, target: CellId, offset: i64, ordinal: u64, impulse: i32) -> SpikeInput {
        SpikeInput {
            arrival_tick: self.phase + offset,
            phase: 0,
            origin_physical: self.root + 50_000 + ordinal,
            target,
            impulse,
        }
    }

    fn finish(self, contacts: Vec<ArrowId>, qlp_arrows: Vec<ArrowId>, steps: Vec<Step>) -> Geometry {
        Geometry {
            body: self.body,
            contacts,
            qlp_arrows,
            steps,
        }
    }
}

fn positive_geometry(mut builder: Builder, include_consequence: bool) -> Geometry {
    let c1 = builder.cell(1);
    let c2 = builder.cell(1);
    let effect = builder.cell(2);
    let source = builder.source(c1);
    let f1 = builder.arrow(c1, c2, TransmissionMode::Drive);
    let f2 = builder.arrow(c2, effect, TransmissionMode::Drive);
    builder.arrow(effect, c2, TransmissionMode::Modulatory);
    let qlp = builder.qlp(c2, c1, 0);
    let mut steps = vec![Step::Arrive(vec![builder.spike(source, 0, 1, 1)])];
    if include_consequence {
        steps.push(Step::Arrive(vec![builder.spike(effect, 1, 2, 2)]));
    }
    builder.finish(vec![f1, f2], vec![qlp], steps)
}

fn geometry(root: u64, phase: i64, world: World, mechanics: MechanicalConfig) -> Geometry {
    let mut builder = Builder::new(root, phase, mechanics);
    match world {
        World::PositiveOneHop => positive_geometry(builder, true),
        World::NoPrematureForward => positive_geometry(builder, false),
        World::DriveNotConsequence => {
            let c1 = builder.cell(1);
            let c2 = builder.cell(2);
            let dummy = builder.cell(2);
            let f2 = builder.arrow(c2, dummy, TransmissionMode::Drive);
            let qlp = builder.qlp(c2, c1, 0);
            let participate = builder.spike(c2, 0, 1, 2);
            let drive_only = builder.spike(c2, 1, 2, 1);
            builder.finish(
                vec![f2],
                vec![qlp],
                vec![
                    Step::Arrive(vec![participate]),
                    Step::Arrive(vec![drive_only]),
                ],
            )
        }
        World::NeverParticipated => {
            let c1 = builder.cell(1);
            let c2 = builder.cell(1);
            let effect = builder.cell(1);
            let dummy1 = builder.cell(2);
            let dummy2 = builder.cell(2);
            let f1 = builder.arrow(c1, dummy1, TransmissionMode::Drive);
            let f2 = builder.arrow(c2, dummy2, TransmissionMode::Drive);
            builder.arrow(effect, c2, TransmissionMode::Modulatory);
            let qlp = builder.qlp(c2, c1, 0);
            let effect_input = builder.spike(effect, 0, 3, 1);
            builder.finish(
                vec![f1, f2],
                vec![qlp],
                vec![Step::Arrive(vec![effect_input])],
            )
        }
        World::ExpiredParticipation => {
            let c1 = builder.cell(1);
            let c2 = builder.cell(1);
            let effect = builder.cell(2);
            let source = builder.source(c1);
            let f1 = builder.arrow(c1, c2, TransmissionMode::Drive);
            let f2 = builder.arrow(c2, effect, TransmissionMode::Drive);
            builder.arrow(effect, c2, TransmissionMode::Modulatory);
            let qlp = builder.qlp(c2, c1, 0);
            let start = builder.spike(source, 0, 4, 1);
            let consequence = builder.spike(effect, 1024, 5, 2);
            builder.finish(
                vec![f1, f2],
                vec![qlp],
                vec![
                    Step::Arrive(vec![start]),
                    Step::Advance(phase + 1024),
                    Step::Arrive(vec![consequence]),
                ],
            )
        }
        World::WrongPath | World::UnrelatedActivity => {
            let c1 = builder.cell(1);
            let c2 = builder.cell(if world == World::UnrelatedActivity { 2 } else { 1 });
            let effect = builder.cell(1);
            let d1 = builder.cell(2);
            let d2 = builder.cell(2);
            let other = builder.cell(1);
            let s1 = builder.source(c1);
            let so = builder.source(other);
            let f1 = builder.arrow(c1, d1, TransmissionMode::Drive);
            let required = builder.arrow(c2, d2, TransmissionMode::Drive);
            let other_contact = builder.arrow(other, d2, TransmissionMode::Drive);
            builder.arrow(effect, c2, TransmissionMode::Modulatory);
            let qlp = builder.qlp(c2, c1, 0);
            let mut inputs = vec![builder.spike(s1, 0, 6, 1), builder.spike(so, 0, 7, 1)];
            if world == World::UnrelatedActivity {
                inputs.push(builder.spike(c2, 0, 8, 1));
            }
            inputs.push(builder.spike(effect, 1, 9, 1));
            builder.finish(
                vec![f1, required, other_contact],
                vec![qlp],
                vec![Step::Arrive(inputs)],
            )
        }
        World::TwoUpstreamOneParticipated => {
            let c1a = builder.cell(1);
            let c1b = builder.cell(1);
            let c2 = builder.cell(1);
            let effect = builder.cell(2);
            let da = builder.cell(2);
            let db = builder.cell(2);
            let ua = builder.cell(1);
            let ub = builder.cell(1);
            let sa = builder.source(c1a);
            let s2 = builder.source(c2);
            let fa = builder.arrow(c1a, da, TransmissionMode::Drive);
            let fb = builder.arrow(c1b, db, TransmissionMode::Drive);
            let f2 = builder.arrow(c2, effect, TransmissionMode::Drive);
            builder.arrow(effect, c2, TransmissionMode::Modulatory);
            let q2a = builder.qlp(c2, c1a, 0);
            let q2b = builder.qlp(c2, c1b, 0);
            let qa = builder.qlp(c1a, ua, 0);
            let qb = builder.qlp(c1b, ub, 0);
            let first = vec![builder.spike(sa, 0, 10, 1), builder.spike(s2, 0, 11, 1)];
            let consequence = builder.spike(effect, 1, 12, 2);
            builder.finish(
                vec![f2, fa, fb],
                vec![q2a, q2b, qa, qb],
                vec![Step::Arrive(first), Step::Arrive(vec![consequence])],
            )
        }
        World::ContactFanout => {
            let c1 = builder.cell(1);
            let c2 = builder.cell(1);
            let effect = builder.cell(2);
            let dummy = builder.cell(2);
            let source = builder.source(c2);
            let f2a = builder.arrow(c2, effect, TransmissionMode::Drive);
            let f2b = builder.arrow(c2, dummy, TransmissionMode::Drive);
            builder.arrow(effect, c2, TransmissionMode::Modulatory);
            let qlp = builder.qlp(c2, c1, 0);
            let start = builder.spike(source, 0, 13, 1);
            let consequence = builder.spike(effect, 1, 14, 2);
            builder.finish(
                vec![f2a, f2b],
                vec![qlp],
                vec![Step::Arrive(vec![start]), Step::Arrive(vec![consequence])],
            )
        }
        World::ClosureCycle => {
            let c1 = builder.cell(1);
            let c2 = builder.cell(1);
            let d1 = builder.cell(2);
            let d2 = builder.cell(2);
            let effect = builder.cell(1);
            let s1 = builder.source(c1);
            let s2 = builder.source(c2);
            let f1 = builder.arrow(c1, d1, TransmissionMode::Drive);
            let f2 = builder.arrow(c2, d2, TransmissionMode::Drive);
            builder.arrow(effect, c1, TransmissionMode::Modulatory);
            let q12 = builder.qlp(c1, c2, 1);
            let q21 = builder.qlp(c2, c1, 1);
            let start = vec![builder.spike(s1, 0, 15, 1), builder.spike(s2, 0, 16, 1)];
            let consequence = builder.spike(effect, 1, 17, 1);
            builder.finish(
                vec![f1, f2],
                vec![q12, q21],
                vec![Step::Arrive(start), Step::Arrive(vec![consequence])],
            )
        }
    }
}

fn execute(mut geometry: Geometry) -> Observation {
    let mut trace = Vec::new();
    let mut work = WorkTotals::default();
    let mut quiescent = true;
    for step in geometry.steps {
        match step {
            Step::Arrive(inputs) => {
                let result = geometry.body.arrive(&inputs, 256);
                work.physical = work.physical.saturating_add(result.work.physical_total());
                work.drive = work.drive.saturating_add(result.work.drive_deliveries);
                work.modulation = work
                    .modulation
                    .saturating_add(result.work.modulatory_deliveries);
                work.updates = work
                    .updates
                    .saturating_add(result.work.local_return_updates);
                work.qlp = work
                    .qlp
                    .saturating_add(result.work.qualified_local_traversals);
                quiescent &= result.naturally_quiescent;
                trace.extend(result.physical_trace);
                if work.physical > CYCLE_WORK_CEILING {
                    break;
                }
            }
            Step::Advance(tick) => {
                let elapsed = geometry.body.advance_time(tick);
                work.physical = work.physical.saturating_add(elapsed.physical_total());
            }
        }
    }
    let participation = geometry
        .contacts
        .iter()
        .map(|arrow| geometry.body.local_participation(*arrow))
        .collect::<Vec<_>>();
    let support = geometry
        .contacts
        .iter()
        .map(|arrow| geometry.body.local_plastic_support(*arrow))
        .collect::<Vec<_>>();
    let triggers = geometry
        .qlp_arrows
        .iter()
        .map(|arrow| geometry.body.transmission_trigger(*arrow))
        .collect::<Vec<_>>();
    let qlp_events = trace
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::QualifiedLocalTraversal { .. }))
        .count() as u64;
    let source_fires = trace
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::Fire { .. }))
        .count() as u64;
    let body = geometry.body.arena_body(1);
    let live = geometry.contacts.iter().chain(&geometry.qlp_arrows).all(|id| {
        body.arrows
            .iter()
            .find(|arrow| arrow.id == *id)
            .is_some_and(|arrow| arrow.live)
    });
    Observation {
        trace,
        participation,
        support,
        triggers,
        work,
        qlp_events,
        source_fires,
        final_tick: geometry.body.clock().tick,
        pressure_phase: geometry.body.clock().pressure_phase(),
        body_hash: ContentHash::of(&geometry.body.canonical_body_bytes(1).unwrap()).to_string(),
        live,
        quiescent: quiescent && work.physical <= CYCLE_WORK_CEILING,
    }
}

fn run(root: u64, phase: i64, world: World, mechanics: MechanicalConfig) -> Observation {
    execute(geometry(root, phase, world, mechanics))
}

fn predicate(world: World, observation: &Observation) -> bool {
    let positive = |index: usize| observation.support.get(index).is_some_and(|value| *value > 0);
    let zero = |index: usize| observation.support.get(index).is_some_and(|value| *value == 0);
    match world {
        World::PositiveOneHop => positive(0) && positive(1) && observation.qlp_events == 1,
        World::NeverParticipated | World::ExpiredParticipation => {
            observation.support.iter().all(|value| *value == 0) && observation.qlp_events == 0
        }
        World::WrongPath | World::UnrelatedActivity => {
            zero(1) && observation.qlp_events == 0
        }
        World::TwoUpstreamOneParticipated => {
            positive(0) && positive(1) && zero(2) && observation.qlp_events == 3
        }
        World::ContactFanout => positive(0) && positive(1) && observation.qlp_events == 1,
        World::NoPrematureForward => {
            observation.support.iter().all(|value| *value == 0) && observation.qlp_events == 0
        }
        World::DriveNotConsequence => {
            observation.support.iter().all(|value| *value == 0) && observation.qlp_events == 0
        }
        World::ClosureCycle => {
            positive(0)
                && positive(1)
                && observation.qlp_events > 2
                && observation.work.physical <= CYCLE_WORK_CEILING
                && observation.quiescent
        }
    }
}

fn mechanics_name(mechanics: MechanicalConfig) -> &'static str {
    if mechanics == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn vector(values: &[u64]) -> String {
    values
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn trigger_vector(values: &[TransmissionTrigger]) -> String {
    values
        .iter()
        .map(|value| match value {
            TransmissionTrigger::SourceFires => "source",
            TransmissionTrigger::QualifiedLocalParticipation => "qualified",
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn write_row(
    csv: &mut String,
    case_id: usize,
    root: u64,
    phase: i64,
    world: World,
    mechanics: MechanicalConfig,
    observation: &Observation,
) {
    writeln!(
        csv,
        "{case_id},{root},{phase},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        world.name(),
        mechanics_name(mechanics),
        vector(&observation.participation),
        vector(&observation.support),
        trigger_vector(&observation.triggers),
        u8::from(predicate(world, observation)),
        observation.qlp_events,
        observation.source_fires,
        observation.work.physical,
        observation.work.drive,
        observation.work.modulation,
        observation.work.updates,
        observation.work.qlp,
        observation.final_tick,
        observation.pressure_phase,
        ContentHash::of(format!("{:?}", observation.trace).as_bytes()),
        observation.body_hash,
        u8::from(observation.live),
        u8::from(observation.quiescent),
    )
    .unwrap();
}

fn write_checksums(output: &Path) {
    let mut sums = String::new();
    for name in ["matrix.csv", "report.md"] {
        let bytes = fs::read(output.join(name)).unwrap();
        writeln!(sums, "{}  {name}", ContentHash::of(&bytes)).unwrap();
    }
    fs::write(output.join("SHA256SUMS"), sums).unwrap();
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from("results/pqlc0_participation_qualified_local_closure_v1")
        });
    fs::create_dir_all(&output).unwrap();
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];
    let mut csv = String::from(
        "case_id,root,initial_phase,world,mechanics,participation,support,triggers,predicate_pass,qlp_events,source_fires,physical_work,drive_deliveries,modulatory_deliveries,local_updates,qlp_work,final_tick,pressure_phase,trace_hash,body_hash,live,quiescent\n",
    );
    let mut physical_cases = 0_usize;
    let mut world_complete = [true; 10];
    let mut maximum_cycle_work = 0_u64;
    let mut maximum_cycle_events = 0_u64;
    for (world_index, world) in World::ALL.into_iter().enumerate() {
        for root in ROOTS {
            for phase in 0..10 {
                physical_cases += 1;
                let reference = run(root, phase, world, mechanics[0]);
                let reference_replay = run(root, phase, world, mechanics[0]);
                assert_eq!(reference_replay, reference);
                let production = run(root, phase, world, mechanics[1]);
                let production_replay = run(root, phase, world, mechanics[1]);
                assert_eq!(production_replay, production);
                assert_eq!(production, reference);
                assert!(reference.live);
                assert!(
                    reference
                        .triggers
                        .iter()
                        .all(|trigger| *trigger
                            == TransmissionTrigger::QualifiedLocalParticipation)
                );
                let pass = predicate(world, &reference);
                world_complete[world_index] &= pass;
                if world == World::ClosureCycle {
                    maximum_cycle_work = maximum_cycle_work.max(reference.work.physical);
                    maximum_cycle_events = maximum_cycle_events.max(reference.qlp_events);
                }
                for (kind, observation) in
                    [(mechanics[0], &reference), (mechanics[1], &production)]
                {
                    write_row(
                        &mut csv,
                        physical_cases,
                        root,
                        phase,
                        world,
                        kind,
                        observation,
                    );
                }
            }
        }
    }
    assert_eq!(physical_cases, EXPECTED_PHYSICAL_CASES);
    let mechanics_rows = physical_cases * 2;
    assert_eq!(mechanics_rows, EXPECTED_MECHANICS_ROWS);
    let development_positive = world_complete.iter().all(|value| *value);
    let report = format!(
        "# PQLC0 participation-qualified local closure result v1\n\n\
         - physical cases: `{physical_cases}/{EXPECTED_PHYSICAL_CASES}`\n\
         - mechanics rows: `{mechanics_rows}/{EXPECTED_MECHANICS_ROWS}`\n\
         - exact same-mechanics reconstruction: `{}/{}` runs\n\
         - exact ordered Reference/Production histories: `{physical_cases}/{EXPECTED_PHYSICAL_CASES}`\n\
         - world-complete signature: `{world_complete:?}`\n\
         - maximum cycle physical work: `{maximum_cycle_work}/{CYCLE_WORK_CEILING}`\n\
         - maximum cycle QLP traversals: `{maximum_cycle_events}`\n\
         - PQLC0 development positive: `{development_positive}`\n\
         - pressure, durable-learning, ARC, or authority changes: `0`\n",
        physical_cases * 4,
        physical_cases * 4,
    );
    fs::write(output.join("matrix.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!(
        "PQLC0_COMPLETE physical_cases={physical_cases} positive={development_positive} cycle_work={maximum_cycle_work}"
    );
}
