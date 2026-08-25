#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode,
};

const ROOTS: [u64; 2] = [2_100_000, 2_200_000];
const EXPECTED_PHYSICAL_CASES: usize = 360;
const EXPECTED_MECHANICS_ROWS: usize = 720;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    LocalModulation,
    AdjacentRelay,
}

impl Arm {
    const ALL: [Self; 2] = [Self::LocalModulation, Self::AdjacentRelay];

    fn name(self) -> &'static str {
        match self {
            Self::LocalModulation => "local_modulation",
            Self::AdjacentRelay => "adjacent_relay",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum World {
    OneContact,
    TwoContacts,
    ThreeContacts,
    BrokenChain,
    UnusedIntermediate,
    ParallelDistractor,
    BranchBoth,
    BranchOne,
    TemporalBreak,
}

impl World {
    const ALL: [Self; 9] = [
        Self::OneContact,
        Self::TwoContacts,
        Self::ThreeContacts,
        Self::BrokenChain,
        Self::UnusedIntermediate,
        Self::ParallelDistractor,
        Self::BranchBoth,
        Self::BranchOne,
        Self::TemporalBreak,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::OneContact => "one_contact",
            Self::TwoContacts => "two_contacts",
            Self::ThreeContacts => "three_contacts",
            Self::BrokenChain => "broken_chain",
            Self::UnusedIntermediate => "unused_intermediate",
            Self::ParallelDistractor => "parallel_distractor",
            Self::BranchBoth => "branch_both",
            Self::BranchOne => "branch_one",
            Self::TemporalBreak => "temporal_break",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    updates: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    trace: Vec<PhysicalTransition>,
    participation: Vec<u64>,
    support: Vec<u64>,
    distractor_support: Vec<u64>,
    modulation_at_contacts: Vec<u64>,
    work: WorkTotals,
    final_tick: i64,
    pressure_phase: i64,
    body_hash: String,
    live: bool,
    quiescent: bool,
}

struct Geometry {
    body: PlasticSubstrate,
    effect: CellId,
    contacts: Vec<CellId>,
    contact_arrows: Vec<ArrowId>,
    distractor_arrows: Vec<ArrowId>,
    initial_inputs: Vec<CellId>,
    temporal_reentry: Option<CellId>,
}

struct Builder {
    body: PlasticSubstrate,
    root: u64,
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

    fn source(&mut self, target: CellId) -> CellId {
        let source = self.cell(1);
        self.arrow(source, target, TransmissionMode::Drive);
        source
    }

    fn add_return(
        &mut self,
        arm: Arm,
        effect: CellId,
        contacts: &[CellId],
        local_indices: &[usize],
    ) {
        match arm {
            Arm::LocalModulation => {
                for index in local_indices {
                    self.arrow(effect, contacts[*index], TransmissionMode::Modulatory);
                }
            }
            Arm::AdjacentRelay => {
                if contacts.is_empty() {
                    return;
                }
                if local_indices.len() > 1 {
                    let relay = self.cell(1);
                    self.arrow(effect, relay, TransmissionMode::Drive);
                    for index in local_indices {
                        self.arrow(relay, contacts[*index], TransmissionMode::Modulatory);
                    }
                    return;
                }
                let relays = (0..contacts.len())
                    .map(|_| self.cell(1))
                    .collect::<Vec<_>>();
                self.arrow(effect, relays[contacts.len() - 1], TransmissionMode::Drive);
                for index in (0..contacts.len()).rev() {
                    self.arrow(relays[index], contacts[index], TransmissionMode::Modulatory);
                    if index > 0 {
                        self.arrow(relays[index], relays[index - 1], TransmissionMode::Drive);
                    }
                }
            }
        }
    }

    fn finish(
        self,
        effect: CellId,
        contacts: Vec<CellId>,
        contact_arrows: Vec<ArrowId>,
        distractor_arrows: Vec<ArrowId>,
        initial_inputs: Vec<CellId>,
        temporal_reentry: Option<CellId>,
    ) -> Geometry {
        let _ = self.root;
        Geometry {
            body: self.body,
            effect,
            contacts,
            contact_arrows,
            distractor_arrows,
            initial_inputs,
            temporal_reentry,
        }
    }
}

fn build_chain(
    builder: &mut Builder,
    arm: Arm,
    depth: usize,
) -> (CellId, Vec<CellId>, Vec<ArrowId>, Vec<CellId>) {
    let contacts = (0..depth).map(|_| builder.cell(1)).collect::<Vec<_>>();
    let effect = builder.cell(2);
    let source = builder.source(contacts[0]);
    let arrows = (0..depth)
        .map(|index| {
            let target = if index + 1 < depth {
                contacts[index + 1]
            } else {
                effect
            };
            builder.arrow(contacts[index], target, TransmissionMode::Drive)
        })
        .collect::<Vec<_>>();
    builder.add_return(arm, effect, &contacts, &[depth - 1]);
    (effect, contacts, arrows, vec![source])
}

fn geometry(
    root: u64,
    phase: i64,
    arm: Arm,
    world: World,
    mechanics: MechanicalConfig,
) -> Geometry {
    let mut builder = Builder::new(root, phase, mechanics);
    match world {
        World::OneContact | World::TwoContacts | World::ThreeContacts => {
            let depth = match world {
                World::OneContact => 1,
                World::TwoContacts => 2,
                World::ThreeContacts => 3,
                _ => unreachable!(),
            };
            let (effect, contacts, arrows, inputs) = build_chain(&mut builder, arm, depth);
            builder.finish(effect, contacts, arrows, Vec::new(), inputs, None)
        }
        World::BrokenChain => {
            let c1 = builder.cell(1);
            let dead_end = builder.cell(2);
            let c2 = builder.cell(1);
            let c3 = builder.cell(1);
            let effect = builder.cell(2);
            let s1 = builder.source(c1);
            let s2 = builder.source(c2);
            let f1 = builder.arrow(c1, dead_end, TransmissionMode::Drive);
            let f2 = builder.arrow(c2, c3, TransmissionMode::Drive);
            let f3 = builder.arrow(c3, effect, TransmissionMode::Drive);
            let contacts = vec![c1, c2, c3];
            builder.add_return(arm, effect, &contacts, &[2]);
            builder.finish(
                effect,
                contacts,
                vec![f1, f2, f3],
                Vec::new(),
                vec![s1, s2],
                None,
            )
        }
        World::UnusedIntermediate => {
            let c1 = builder.cell(1);
            let c2 = builder.cell(2);
            let c3 = builder.cell(1);
            let effect = builder.cell(2);
            let s1 = builder.source(c1);
            let s3 = builder.source(c3);
            let f1 = builder.arrow(c1, c2, TransmissionMode::Drive);
            let f2 = builder.arrow(c2, c3, TransmissionMode::Drive);
            let f3 = builder.arrow(c3, effect, TransmissionMode::Drive);
            let contacts = vec![c1, c2, c3];
            builder.add_return(arm, effect, &contacts, &[2]);
            builder.finish(
                effect,
                contacts,
                vec![f1, f2, f3],
                Vec::new(),
                vec![s1, s3],
                None,
            )
        }
        World::ParallelDistractor => {
            let (effect, contacts, arrows, mut inputs) = build_chain(&mut builder, arm, 2);
            let d1 = builder.cell(1);
            let d2 = builder.cell(1);
            let d_effect = builder.cell(2);
            let d_source = builder.source(d1);
            let d_a = builder.arrow(d1, d2, TransmissionMode::Drive);
            let d_b = builder.arrow(d2, d_effect, TransmissionMode::Drive);
            inputs.push(d_source);
            builder.finish(effect, contacts, arrows, vec![d_a, d_b], inputs, None)
        }
        World::BranchBoth | World::BranchOne => {
            let a = builder.cell(1);
            let b = builder.cell(1);
            let effect = builder.cell(3);
            let sa = builder.source(a);
            let sb = builder.source(b);
            let fa = builder.arrow(a, effect, TransmissionMode::Drive);
            let fb = builder.arrow(b, effect, TransmissionMode::Drive);
            let contacts = vec![a, b];
            builder.add_return(arm, effect, &contacts, &[0, 1]);
            let inputs = if world == World::BranchBoth {
                vec![sa, sb]
            } else {
                vec![sa]
            };
            builder.finish(effect, contacts, vec![fa, fb], Vec::new(), inputs, None)
        }
        World::TemporalBreak => {
            let (effect, contacts, arrows, inputs) = build_chain(&mut builder, arm, 3);
            let reentry = contacts[2];
            builder.finish(effect, contacts, arrows, Vec::new(), inputs, Some(reentry))
        }
    }
}

fn input(target: CellId, tick: i64, origin: u64, impulse: i32) -> SpikeInput {
    SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target,
        impulse,
    }
}

fn admit(
    geometry: &mut Geometry,
    inputs: &[SpikeInput],
    trace: &mut Vec<PhysicalTransition>,
    totals: &mut WorkTotals,
) {
    let result = geometry.body.arrive(inputs, 256);
    assert!(result.naturally_quiescent);
    totals.physical = totals.physical.saturating_add(result.work.physical_total());
    totals.drive = totals.drive.saturating_add(result.work.drive_deliveries);
    totals.modulation = totals
        .modulation
        .saturating_add(result.work.modulatory_deliveries);
    totals.updates = totals
        .updates
        .saturating_add(result.work.local_return_updates);
    trace.extend(result.physical_trace);
}

fn run(root: u64, phase: i64, arm: Arm, world: World, mechanics: MechanicalConfig) -> Observation {
    let mut geometry = geometry(root, phase, arm, world, mechanics);
    let mut trace = Vec::new();
    let mut work = WorkTotals::default();
    let initial = geometry
        .initial_inputs
        .iter()
        .enumerate()
        .map(|(index, target)| input(*target, phase, root + 10_000 + index as u64, 1))
        .collect::<Vec<_>>();
    admit(&mut geometry, &initial, &mut trace, &mut work);

    let consequence_tick = if world == World::TemporalBreak {
        let tick = phase + 1024;
        let advance = geometry.body.advance_time(tick);
        work.physical = work.physical.saturating_add(advance.physical_total());
        let target = geometry.temporal_reentry.unwrap();
        admit(
            &mut geometry,
            &[input(target, tick, root + 20_001, 1)],
            &mut trace,
            &mut work,
        );
        tick
    } else {
        phase + 1
    };
    let effect = geometry.effect;
    let consequence_impulse = if world == World::BranchBoth || world == World::BranchOne {
        3
    } else if world == World::TemporalBreak {
        1
    } else {
        2
    };
    admit(
        &mut geometry,
        &[input(
            effect,
            consequence_tick,
            root + 20_002,
            consequence_impulse,
        )],
        &mut trace,
        &mut work,
    );

    let participation = geometry
        .contact_arrows
        .iter()
        .map(|arrow| geometry.body.local_participation(*arrow))
        .collect::<Vec<_>>();
    let support = geometry
        .contact_arrows
        .iter()
        .map(|arrow| geometry.body.local_plastic_support(*arrow))
        .collect::<Vec<_>>();
    let distractor_support = geometry
        .distractor_arrows
        .iter()
        .map(|arrow| geometry.body.local_plastic_support(*arrow))
        .collect::<Vec<_>>();
    let modulation_at_contacts = geometry
        .contacts
        .iter()
        .map(|contact| {
            trace
                .iter()
                .filter(|transition| {
                    matches!(
                        transition.event,
                        PhysicalEvent::Deliver {
                            mode: TransmissionMode::Modulatory,
                            target,
                            ..
                        } if target == *contact
                    )
                })
                .count() as u64
        })
        .collect::<Vec<_>>();
    let body = geometry.body.arena_body(1);
    let live = geometry.contact_arrows.iter().all(|arrow| {
        body.arrows
            .iter()
            .find(|candidate| candidate.id == *arrow)
            .is_some_and(|candidate| candidate.live)
    });
    Observation {
        trace,
        participation,
        support,
        distractor_support,
        modulation_at_contacts,
        work,
        final_tick: geometry.body.clock().tick,
        pressure_phase: geometry.body.clock().pressure_phase(),
        body_hash: ContentHash::of(&geometry.body.canonical_body_bytes(1).unwrap()).to_string(),
        live,
        quiescent: true,
    }
}

fn predicate(world: World, observation: &Observation) -> bool {
    let positive = |index: usize| {
        observation
            .support
            .get(index)
            .is_some_and(|value| *value > 0)
    };
    let zero = |index: usize| {
        observation
            .support
            .get(index)
            .is_some_and(|value| *value == 0)
    };
    let no_modulation = |index: usize| {
        observation
            .modulation_at_contacts
            .get(index)
            .is_some_and(|value| *value == 0)
    };
    match world {
        World::OneContact => positive(0),
        World::TwoContacts => positive(0) && positive(1),
        World::ThreeContacts => positive(0) && positive(1) && positive(2),
        World::BrokenChain => zero(0) && no_modulation(0) && positive(2),
        World::UnusedIntermediate => {
            zero(0) && zero(1) && no_modulation(0) && no_modulation(1) && positive(2)
        }
        World::ParallelDistractor => {
            positive(0)
                && positive(1)
                && observation
                    .distractor_support
                    .iter()
                    .all(|value| *value == 0)
        }
        World::BranchBoth => positive(0) && positive(1),
        World::BranchOne => positive(0) && zero(1),
        World::TemporalBreak => {
            zero(0) && zero(1) && positive(2) && no_modulation(0) && no_modulation(1)
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

fn write_row(
    csv: &mut String,
    case_id: usize,
    root: u64,
    phase: i64,
    arm: Arm,
    world: World,
    mechanics: MechanicalConfig,
    observation: &Observation,
) {
    writeln!(
        csv,
        "{case_id},{root},{phase},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        arm.name(),
        world.name(),
        mechanics_name(mechanics),
        vector(&observation.participation),
        vector(&observation.support),
        vector(&observation.distractor_support),
        vector(&observation.modulation_at_contacts),
        u8::from(predicate(world, observation)),
        observation.work.physical,
        observation.work.drive,
        observation.work.modulation,
        observation.work.updates,
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
        .unwrap_or_else(|| PathBuf::from("results/cpc2_adjacent_local_causal_closure_v1"));
    fs::create_dir_all(&output).unwrap();
    let mut csv = String::from(
        "case_id,root,initial_phase,arm,world,mechanics,participation,support,distractor_support,modulation_at_contacts,predicate_pass,physical_work,drive_deliveries,modulatory_deliveries,local_updates,final_tick,pressure_phase,trace_hash,body_hash,live,quiescent\n",
    );
    let mechanics = [MechanicalConfig::REFERENCE, MechanicalConfig::PRODUCTION];
    let mut physical_cases = 0_usize;
    let mut local_passes = 0_usize;
    let mut relay_passes = 0_usize;
    let mut local_worlds = [true; 9];
    let mut relay_worlds = [true; 9];
    for arm in Arm::ALL {
        for (world_index, world) in World::ALL.into_iter().enumerate() {
            for root in ROOTS {
                for phase in 0..10 {
                    physical_cases += 1;
                    let reference = run(root, phase, arm, world, mechanics[0]);
                    let reference_replay = run(root, phase, arm, world, mechanics[0]);
                    assert_eq!(reference_replay, reference);
                    let production = run(root, phase, arm, world, mechanics[1]);
                    let production_replay = run(root, phase, arm, world, mechanics[1]);
                    assert_eq!(production_replay, production);
                    assert_eq!(production, reference);
                    assert!(reference.live && reference.quiescent);
                    let pass = predicate(world, &reference);
                    match arm {
                        Arm::LocalModulation => {
                            local_passes += usize::from(pass);
                            local_worlds[world_index] &= pass;
                        }
                        Arm::AdjacentRelay => {
                            relay_passes += usize::from(pass);
                            relay_worlds[world_index] &= pass;
                        }
                    }
                    for (kind, observation) in
                        [(mechanics[0], &reference), (mechanics[1], &production)]
                    {
                        write_row(
                            &mut csv,
                            physical_cases,
                            root,
                            phase,
                            arm,
                            world,
                            kind,
                            observation,
                        );
                    }
                }
            }
        }
    }
    assert_eq!(physical_cases, EXPECTED_PHYSICAL_CASES);
    let mechanics_rows = physical_cases * 2;
    assert_eq!(mechanics_rows, EXPECTED_MECHANICS_ROWS);
    let local_complete = local_worlds.iter().all(|value| *value);
    let relay_complete = relay_worlds.iter().all(|value| *value);
    let development_positive = local_complete || relay_complete;
    let report = format!(
        "# CPC2 adjacent local causal closure result v1\n\n\
         - physical cases: `{physical_cases}/{EXPECTED_PHYSICAL_CASES}`\n\
         - mechanics rows: `{mechanics_rows}/{EXPECTED_MECHANICS_ROWS}`\n\
         - exact same-mechanics replay: `{}/{}` runs\n\
         - exact ordered Reference/Production histories: `{physical_cases}/{EXPECTED_PHYSICAL_CASES}`\n\
         - local-Modulation predicate cases: `{local_passes}/180`\n\
         - adjacent-relay predicate cases: `{relay_passes}/180`\n\
         - local-Modulation complete arm: `{local_complete}`\n\
         - adjacent-relay complete arm: `{relay_complete}`\n\
         - CPC2 development positive: `{development_positive}`\n\
         - runtime or substrate-law changes: `0`\n\n\
         World-complete signatures:\n\
         local-Modulation: `{local_worlds:?}`\n\
         adjacent-relay: `{relay_worlds:?}`\n",
        physical_cases * 4,
        physical_cases * 4,
    );
    fs::write(output.join("matrix.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    write_checksums(&output);
    println!(
        "CPC2_COMPLETE physical_cases={physical_cases} local_complete={local_complete} relay_complete={relay_complete}"
    );
}
