#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [6_100_000, 6_200_000];
const START_TICKS: [i64; 3] = [0, 3, 7];
const POSITION_ORIGINS: [i32; 2] = [0, 10_000];
const RETAIN_AGE: i64 = 14;
const WEAK_LIFETIME: i64 = 10;
const WORK_CEILING: u64 = 256;
const EXPECTED_CASES: usize = 72;
const EXPECTED_ROWS: usize = 144;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Family {
    SymmetryNoConsequence,
    PositiveUseful,
    NegativeUseful,
    NeitherUsefulRepeated,
    BothUseful,
    BoundedReproposal,
}

impl Family {
    const ALL: [Self; 6] = [
        Self::SymmetryNoConsequence,
        Self::PositiveUseful,
        Self::NegativeUseful,
        Self::NeitherUsefulRepeated,
        Self::BothUseful,
        Self::BoundedReproposal,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::SymmetryNoConsequence => "symmetry_no_consequence",
            Self::PositiveUseful => "positive_useful",
            Self::NegativeUseful => "negative_useful",
            Self::NeitherUsefulRepeated => "neither_useful_repeated",
            Self::BothUseful => "both_useful",
            Self::BoundedReproposal => "bounded_reproposal",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    updates: u64,
    proposals: u64,
    deallocations: u64,
    qlp: u64,
}

impl WorkTotals {
    fn add(&mut self, work: Work) {
        self.physical = self.physical.saturating_add(work.physical_total());
        self.drive = self.drive.saturating_add(work.drive_deliveries);
        self.modulation = self
            .modulation
            .saturating_add(work.modulatory_deliveries);
        self.updates = self.updates.saturating_add(work.local_return_updates);
        self.proposals = self
            .proposals
            .saturating_add(work.local_structural_proposals);
        self.deallocations = self
            .deallocations
            .saturating_add(work.physical_deallocations);
        self.qlp = self.qlp.saturating_add(work.qualified_local_traversals);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CandidateState {
    id: ArrowId,
    coupling: i32,
    delay: i64,
    phase: i32,
    resistance: u32,
    live: bool,
    participation: u64,
    plastic_support: u64,
    decay_load: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    pre_trace: Vec<PhysicalTransition>,
    trace: Vec<PhysicalTransition>,
    initial: Vec<CandidateState>,
    after_consequence: Vec<CandidateState>,
    final_candidates: Vec<CandidateState>,
    initial_crossings: Vec<i32>,
    death_ages: Vec<(i32, i64)>,
    selected_crossing_present: bool,
    peak_live_candidates: usize,
    work: WorkTotals,
    final_tick: i64,
    pre_body_hash: String,
    body_hash: String,
    naturally_quiescent: bool,
}

impl Observation {
    fn candidate(&self, coupling: i32) -> CandidateState {
        *self
            .final_candidates
            .iter()
            .find(|candidate| candidate.coupling == coupling)
            .expect("SV0 signed candidate must exist")
    }
}

struct World {
    body: PlasticSubstrate,
    source: CellId,
    target: CellId,
    consequence_cells: [CellId; 2],
}

fn build_world(
    root: u64,
    start_tick: i64,
    position_origin: i32,
    mechanics: MechanicalConfig,
) -> World {
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 16, 32, mechanics);
    body.set_physical_tracing(true);
    if start_tick > 0 {
        body.advance_time(start_tick);
    }
    let source = body.add_cell(CellSpec {
        physical_id: root + 1,
        position: position_origin,
        region: 0,
        threshold: 1,
        resistance: 1_000,
    });
    let target = body.add_cell(CellSpec {
        physical_id: root + 2,
        position: position_origin.saturating_add(1),
        region: 1,
        threshold: 100,
        resistance: 1_000,
    });
    let consequence_cells = [100, 200].map(|offset| {
        body.add_cell(CellSpec {
            physical_id: root + u64::try_from(offset).unwrap(),
            position: position_origin.saturating_add(offset),
            region: 0,
            threshold: 1,
            resistance: 1_000,
        })
    });
    for consequence in consequence_cells {
        body.add_arrow(ArrowSpec {
            from: consequence,
            to: source,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 1_000,
            mode: TransmissionMode::Modulatory,
        });
    }
    World {
        body,
        source,
        target,
        consequence_cells,
    }
}

fn enter_drive(body: &mut PlasticSubstrate, target: CellId, tick: i64, origin: u64) {
    body.enter(SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target,
        impulse: 1,
    });
}

fn candidate_states(body: &PlasticSubstrate, source: CellId, target: CellId) -> Vec<CandidateState> {
    let mut candidates = body
        .arena_body(1)
        .arrows
        .into_iter()
        .filter(|arrow| arrow.from.id == source && arrow.to.id == target)
        .map(|arrow| CandidateState {
            id: arrow.id,
            coupling: arrow.coupling,
            delay: arrow.delay,
            phase: arrow.phase,
            resistance: arrow.resistance,
            live: arrow.live,
            participation: body.local_participation(arrow.id),
            plastic_support: body.local_plastic_support(arrow.id),
            decay_load: body.local_decay_load(arrow.id),
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|candidate| candidate.coupling);
    candidates
}

fn live_candidates(body: &PlasticSubstrate, source: CellId, target: CellId) -> usize {
    candidate_states(body, source, target)
        .iter()
        .filter(|candidate| candidate.live)
        .count()
}

fn body_hash(body: &PlasticSubstrate) -> String {
    ContentHash::of(&body.canonical_body_bytes(1).unwrap()).to_string()
}

fn extend_run(
    body: &mut PlasticSubstrate,
    trace: &mut Vec<PhysicalTransition>,
    work: &mut WorkTotals,
) -> bool {
    let run = body.propagate();
    trace.extend(run.physical_trace);
    work.add(run.work);
    run.naturally_quiescent
}

fn run_case(
    root: u64,
    start_tick: i64,
    position_origin: i32,
    family: Family,
    mechanics: MechanicalConfig,
) -> Observation {
    let mut world = build_world(root, start_tick, position_origin, mechanics);
    let mut trace = Vec::new();
    let mut work = WorkTotals::default();
    let mut naturally_quiescent = true;

    enter_drive(
        &mut world.body,
        world.source,
        start_tick,
        root + 900_000,
    );
    let first = world.body.propagate();
    naturally_quiescent &= first.naturally_quiescent;
    let initial_crossings = first
        .crossings
        .iter()
        .filter(|crossing| crossing.from_region == 0 && crossing.to_region == 1)
        .map(|crossing| crossing.impulse)
        .collect::<Vec<_>>();
    let pre_trace = first.physical_trace.clone();
    trace.extend(first.physical_trace);
    work.add(first.work);
    let initial = candidate_states(&world.body, world.source, world.target);
    let pre_body_hash = body_hash(&world.body);
    let mut peak_live_candidates = live_candidates(&world.body, world.source, world.target);

    let selected_crossing_present = match family {
        Family::PositiveUseful => initial_crossings.contains(&1),
        Family::NegativeUseful => initial_crossings.contains(&-1),
        Family::BothUseful => initial_crossings.contains(&1) && initial_crossings.contains(&-1),
        _ => true,
    };

    match family {
        Family::PositiveUseful | Family::NegativeUseful => {
            let tick = world.body.clock().tick.saturating_add(1);
            enter_drive(
                &mut world.body,
                world.consequence_cells[0],
                tick,
                root + 910_000,
            );
            naturally_quiescent &= extend_run(&mut world.body, &mut trace, &mut work);
        }
        Family::BothUseful => {
            let tick = world.body.clock().tick.saturating_add(1);
            for (index, consequence) in world.consequence_cells.into_iter().enumerate() {
                enter_drive(
                    &mut world.body,
                    consequence,
                    tick,
                    root + 910_000 + u64::try_from(index).unwrap(),
                );
            }
            naturally_quiescent &= extend_run(&mut world.body, &mut trace, &mut work);
        }
        _ => {}
    }
    let after_consequence = candidate_states(&world.body, world.source, world.target);
    peak_live_candidates = peak_live_candidates.max(live_candidates(
        &world.body,
        world.source,
        world.target,
    ));

    let mut death_ages = Vec::new();
    match family {
        Family::SymmetryNoConsequence => {
            let mut previous = candidate_states(&world.body, world.source, world.target);
            for tick in start_tick.saturating_add(2)..=start_tick.saturating_add(WEAK_LIFETIME) {
                work.add(world.body.advance_time(tick));
                let current = candidate_states(&world.body, world.source, world.target);
                for (before, after) in previous.iter().zip(&current) {
                    if before.live && !after.live {
                        death_ages.push((after.coupling, tick.saturating_sub(start_tick)));
                    }
                }
                previous = current;
            }
        }
        Family::NeitherUsefulRepeated => {
            for offset in [3, 5, 7] {
                enter_drive(
                    &mut world.body,
                    world.source,
                    start_tick.saturating_add(offset),
                    root + 920_000 + u64::try_from(offset).unwrap(),
                );
                naturally_quiescent &= extend_run(&mut world.body, &mut trace, &mut work);
                peak_live_candidates = peak_live_candidates.max(live_candidates(
                    &world.body,
                    world.source,
                    world.target,
                ));
            }
            work.add(
                world
                    .body
                    .advance_time(start_tick.saturating_add(WEAK_LIFETIME)),
            );
        }
        Family::BoundedReproposal => {
            enter_drive(
                &mut world.body,
                world.source,
                start_tick.saturating_add(3),
                root + 930_000,
            );
            naturally_quiescent &= extend_run(&mut world.body, &mut trace, &mut work);
            peak_live_candidates = peak_live_candidates.max(live_candidates(
                &world.body,
                world.source,
                world.target,
            ));
            work.add(
                world
                    .body
                    .advance_time(start_tick.saturating_add(WEAK_LIFETIME)),
            );
            enter_drive(
                &mut world.body,
                world.source,
                start_tick.saturating_add(11),
                root + 930_001,
            );
            naturally_quiescent &= extend_run(&mut world.body, &mut trace, &mut work);
            peak_live_candidates = peak_live_candidates.max(live_candidates(
                &world.body,
                world.source,
                world.target,
            ));
            work.add(world.body.advance_time(start_tick.saturating_add(21)));
        }
        Family::PositiveUseful | Family::NegativeUseful | Family::BothUseful => {
            work.add(
                world
                    .body
                    .advance_time(start_tick.saturating_add(RETAIN_AGE)),
            );
        }
    }

    let final_candidates = candidate_states(&world.body, world.source, world.target);
    Observation {
        pre_trace,
        trace,
        initial,
        after_consequence,
        final_candidates,
        initial_crossings,
        death_ages,
        selected_crossing_present,
        peak_live_candidates,
        work,
        final_tick: world.body.clock().tick,
        pre_body_hash,
        body_hash: body_hash(&world.body),
        naturally_quiescent,
    }
}

fn symmetric_initial(observation: &Observation) -> bool {
    if observation.initial.len() != 2 {
        return false;
    }
    let negative = observation.initial[0];
    let positive = observation.initial[1];
    negative.coupling == -1
        && positive.coupling == 1
        && negative.delay == positive.delay
        && negative.phase == positive.phase
        && negative.resistance == 1
        && positive.resistance == 1
        && negative.live
        && positive.live
        && negative.participation == positive.participation
        && negative.plastic_support == positive.plastic_support
        && negative.decay_load == positive.decay_load
        && observation.initial_crossings == [1, -1]
}

fn predicate(family: Family, observation: &Observation) -> bool {
    let base = symmetric_initial(observation)
        && observation.selected_crossing_present
        && observation.naturally_quiescent
        && observation.work.physical <= WORK_CEILING
        && observation.work.qlp == 0
        && observation
            .final_candidates
            .iter()
            .all(|candidate| candidate.coupling == -1 || candidate.coupling == 1);
    if !base {
        return false;
    }
    let negative = observation.candidate(-1);
    let positive = observation.candidate(1);
    match family {
        Family::SymmetryNoConsequence => {
            observation.work.proposals == 2
                && observation.work.updates == 0
                && observation.work.modulation == 0
                && observation.work.deallocations == 2
                && observation.death_ages == [(-1, WEAK_LIFETIME), (1, WEAK_LIFETIME)]
                && !negative.live
                && !positive.live
        }
        Family::PositiveUseful => {
            observation.work.proposals == 2
                && observation.work.updates == 1
                && positive.live
                && positive.resistance > 1
                && !negative.live
                && negative.resistance == 0
        }
        Family::NegativeUseful => {
            observation.work.proposals == 2
                && observation.work.updates == 1
                && negative.live
                && negative.resistance > 1
                && !positive.live
                && positive.resistance == 0
        }
        Family::NeitherUsefulRepeated => {
            observation.work.proposals == 2
                && observation.work.updates == 0
                && observation.work.modulation == 0
                && observation.work.deallocations == 2
                && !negative.live
                && !positive.live
        }
        Family::BothUseful => {
            observation.work.proposals == 2
                && observation.work.updates == 4
                && negative.live
                && positive.live
                && negative.resistance > 1
                && positive.resistance > 1
        }
        Family::BoundedReproposal => {
            observation.work.proposals == 4
                && observation.work.updates == 0
                && observation.work.deallocations == 4
                && observation.peak_live_candidates == 2
                && !negative.live
                && !positive.live
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

fn states_text(states: &[CandidateState]) -> String {
    states
        .iter()
        .map(|state| {
            format!(
                "{}:{}/{}/{}/{}/{}",
                state.coupling,
                u8::from(state.live),
                state.resistance,
                state.participation,
                state.plastic_support,
                state.decay_load
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn selection_text(observation: &Observation) -> &'static str {
    match (observation.candidate(-1).live, observation.candidate(1).live) {
        (false, false) => "neither",
        (false, true) => "positive_only",
        (true, false) => "negative_only",
        (true, true) => "both",
    }
}

struct CaseRecord {
    case_id: usize,
    root: u64,
    start_tick: i64,
    position_origin: i32,
    family: Family,
    reference: Observation,
    production: Observation,
    reference_replay_equal: bool,
    production_replay_equal: bool,
    mechanics_equal: bool,
    reference_pass: bool,
    production_pass: bool,
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/sv0_symmetric_sign_variation_v1"));
    fs::create_dir_all(&output).expect("create SV0 output directory");

    let mut cases = Vec::new();
    let mut case_id = 0_usize;
    for root in ROOTS {
        for start_tick in START_TICKS {
            for position_origin in POSITION_ORIGINS {
                for family in Family::ALL {
                    case_id += 1;
                    let reference = run_case(
                        root,
                        start_tick,
                        position_origin,
                        family,
                        MechanicalConfig::REFERENCE,
                    );
                    let reference_replay = run_case(
                        root,
                        start_tick,
                        position_origin,
                        family,
                        MechanicalConfig::REFERENCE,
                    );
                    let production = run_case(
                        root,
                        start_tick,
                        position_origin,
                        family,
                        MechanicalConfig::PRODUCTION,
                    );
                    let production_replay = run_case(
                        root,
                        start_tick,
                        position_origin,
                        family,
                        MechanicalConfig::PRODUCTION,
                    );
                    let reference_replay_equal = reference == reference_replay;
                    let production_replay_equal = production == production_replay;
                    let mechanics_equal = reference == production;
                    let reference_pass = predicate(family, &reference);
                    let production_pass = predicate(family, &production);
                    cases.push(CaseRecord {
                        case_id,
                        root,
                        start_tick,
                        position_origin,
                        family,
                        reference,
                        production,
                        reference_replay_equal,
                        production_replay_equal,
                        mechanics_equal,
                        reference_pass,
                        production_pass,
                    });
                }
            }
        }
    }

    let mut permutation_by_variant = BTreeMap::new();
    for root in ROOTS {
        for start_tick in START_TICKS {
            for position_origin in POSITION_ORIGINS {
                let positive = cases
                    .iter()
                    .find(|case| {
                        case.root == root
                            && case.start_tick == start_tick
                            && case.position_origin == position_origin
                            && case.family == Family::PositiveUseful
                    })
                    .unwrap();
                let negative = cases
                    .iter()
                    .find(|case| {
                        case.root == root
                            && case.start_tick == start_tick
                            && case.position_origin == position_origin
                            && case.family == Family::NegativeUseful
                    })
                    .unwrap();
                let pass = positive.reference.pre_trace == negative.reference.pre_trace
                    && positive.reference.pre_body_hash == negative.reference.pre_body_hash
                    && positive.reference.initial == negative.reference.initial
                    && selection_text(&positive.reference) == "positive_only"
                    && selection_text(&negative.reference) == "negative_only";
                permutation_by_variant.insert((root, start_tick, position_origin), pass);
            }
        }
    }

    let mut csv = String::from(
        "case_id,root,start_tick,position_origin,family,mechanics,initial_states,after_consequence_states,final_states,initial_crossings,death_ages,selected_crossing_present,selection,proposals,updates,deallocations,physical_work,peak_live_candidates,final_tick,naturally_quiescent,replay_equal,mechanics_equal,predicate_pass,permutation_pass,case_pass,pre_body_hash,body_hash\n",
    );
    let mut rows = 0_usize;
    let mut maximum_work = 0_u64;
    let mut all_replay = true;
    let mut all_mechanics = true;
    let mut all_predicates = true;
    let mut all_permutation = true;
    let mut all_pass = true;
    let mut gate_pass = BTreeMap::<char, bool>::from([
        ('A', true),
        ('B', true),
        ('C', true),
        ('D', true),
        ('E', true),
        ('F', true),
        ('G', true),
    ]);

    for case in &cases {
        let permutation_pass = *permutation_by_variant
            .get(&(case.root, case.start_tick, case.position_origin))
            .unwrap();
        let case_pass = case.reference_replay_equal
            && case.production_replay_equal
            && case.mechanics_equal
            && case.reference_pass
            && case.production_pass
            && permutation_pass;
        all_replay &= case.reference_replay_equal && case.production_replay_equal;
        all_mechanics &= case.mechanics_equal;
        all_predicates &= case.reference_pass && case.production_pass;
        all_permutation &= permutation_pass;
        all_pass &= case_pass;
        let gate = match case.family {
            Family::SymmetryNoConsequence => 'A',
            Family::PositiveUseful => 'B',
            Family::NegativeUseful => 'C',
            Family::NeitherUsefulRepeated => 'E',
            Family::BothUseful => 'F',
            Family::BoundedReproposal => 'G',
        };
        *gate_pass.get_mut(&gate).unwrap() &= case.reference_pass && case.production_pass;
        *gate_pass.get_mut(&'D').unwrap() &= permutation_pass;

        for (mechanics, observation, replay_equal, predicate_pass) in [
            (
                MechanicalConfig::REFERENCE,
                &case.reference,
                case.reference_replay_equal,
                case.reference_pass,
            ),
            (
                MechanicalConfig::PRODUCTION,
                &case.production,
                case.production_replay_equal,
                case.production_pass,
            ),
        ] {
            rows += 1;
            maximum_work = maximum_work.max(observation.work.physical);
            writeln!(
                csv,
                "{},{},{},{},{},{},{},{},{},{:?},{:?},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                case.case_id,
                case.root,
                case.start_tick,
                case.position_origin,
                case.family.name(),
                mechanics_name(mechanics),
                states_text(&observation.initial),
                states_text(&observation.after_consequence),
                states_text(&observation.final_candidates),
                observation.initial_crossings,
                observation.death_ages,
                observation.selected_crossing_present,
                selection_text(observation),
                observation.work.proposals,
                observation.work.updates,
                observation.work.deallocations,
                observation.work.physical,
                observation.peak_live_candidates,
                observation.final_tick,
                observation.naturally_quiescent,
                replay_equal,
                case.mechanics_equal,
                predicate_pass,
                permutation_pass,
                case_pass,
                observation.pre_body_hash,
                observation.body_hash,
            )
            .unwrap();
        }
    }

    assert_eq!(case_id, EXPECTED_CASES);
    assert_eq!(rows, EXPECTED_ROWS);
    let matrix_path = output.join("matrix.csv");
    fs::write(&matrix_path, csv).expect("write SV0 matrix");
    let matrix_hash = ContentHash::of(&fs::read(&matrix_path).unwrap()).to_string();
    let first_failed_gate = ['A', 'B', 'C', 'D', 'E', 'F', 'G']
        .into_iter()
        .find(|gate| !gate_pass[gate]);
    let classification = first_failed_gate.map_or_else(
        || "SV0 positive — symmetric signed variation is selectable".to_string(),
        |gate| format!("SV0 negative — first failed gate {gate}"),
    );
    let report = format!(
        "# SV0 symmetric sign variation v1\n\n\
         - cases: `{case_id}/{EXPECTED_CASES}`\n\
         - mechanics rows: `{rows}/{EXPECTED_ROWS}`\n\
         - exact same-mechanics replay: `{all_replay}`\n\
         - exact Reference/Production agreement: `{all_mechanics}`\n\
         - all family predicates: `{all_predicates}`\n\
         - sign-usefulness permutation: `{all_permutation}`\n\
         - Gate A symmetry: `{}`\n\
         - Gate B positive selection: `{}`\n\
         - Gate C negative selection: `{}`\n\
         - Gate D sign permutation: `{}`\n\
         - Gate E neither useful: `{}`\n\
         - Gate F both useful: `{}`\n\
         - Gate G bounded variation: `{}`\n\
         - maximum PhysicalWork: `{maximum_work}/{WORK_CEILING}`\n\
         - classification: `{classification}`\n\
         - matrix SHA-256: `{matrix_hash}`\n",
        gate_pass[&'A'],
        gate_pass[&'B'],
        gate_pass[&'C'],
        gate_pass[&'D'],
        gate_pass[&'E'],
        gate_pass[&'F'],
        gate_pass[&'G'],
    );
    fs::write(output.join("report.md"), report).expect("write SV0 report");
    assert!(all_pass, "SV0 symmetric sign variation gate failed");
    println!("SV0_SYMMETRIC_SIGN_VARIATION_V1_PASS");
    println!("classification={classification}");
    println!("matrix_sha256={matrix_hash}");
}
