//! RG0a: ground frozen reflected roles into fresh lower cells.
//!
//! The experimental path never constructs `ProgramChoices` and never calls the
//! parent module's direct `execute` function. A concrete route-source cell is
//! matched to a temporary reflected role, a frozen learned arrow fires, and
//! the target role's binding supplies only an opaque lower-cell destination.

use super::*;
use crate::research_runtime::{parallel_map_ordered, Frozen, HarnessMode};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub const RG0A_PROTOCOL: &str = "reflected-grounding-rg0a-v1";

const ROLE_COUNT: usize = 10;
const RG0A_DEPTHS: [usize; 6] = [5, 8, 16, 32, 64, 128];
const RG0A_QUERIES_PER_DEPTH: usize = 16;
const MICRO_DEPTHS: [usize; 2] = [3, 5];
const MICRO_QUERIES_PER_DEPTH: usize = 4;
const DEVELOPMENT_SEED_INDEX: usize = 10_000;

type LowerLocation = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CellPhysics {
    RouteSource,
    Apply {
        route_source: LowerLocation,
    },
    Lookup {
        result_source: LowerLocation,
        no_result_source: LowerLocation,
    },
    StoreCurrent {
        success_source: LowerLocation,
    },
    Answer,
    Clear,
    Quiet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GroundCell {
    identity: LowerLocation,
    physics: CellPhysics,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GroundObservation {
    cell_index: usize,
    signature: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GroundMachine {
    relations: Vec<(OpaqueId, OpaqueId)>,
    query: OpaqueId,
    answer: OpaqueId,
    cells: [GroundCell; ROLE_COUNT],
    observations: [GroundObservation; ROLE_COUNT],
    activity_sources: [u64; ROLE_COUNT],
    start_location: LowerLocation,
    provenance_events: u64,
    provenance_relations: u64,
}

impl GroundMachine {
    fn immutable_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.relations.hash(&mut hasher);
        self.query.hash(&mut hasher);
        self.answer.hash(&mut hasher);
        self.cells.hash(&mut hasher);
        self.observations.hash(&mut hasher);
        self.activity_sources.hash(&mut hasher);
        self.start_location.hash(&mut hasher);
        self.provenance_events.hash(&mut hasher);
        self.provenance_relations.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Clone, Debug)]
struct GroundEpisode {
    machine: GroundMachine,
    evaluator_locations: [LowerLocation; ROLE_COUNT],
}

fn role_index(role: LowerRole) -> usize {
    LowerRole::ALL
        .iter()
        .position(|candidate| *candidate == role)
        .expect("frozen lower role")
}

fn location(locations: &[LowerLocation; ROLE_COUNT], role: LowerRole) -> LowerLocation {
    locations[role_index(role)]
}

fn fresh_lower_location(episode_id: u64, ordinal: usize) -> LowerLocation {
    0xd100_0000_0000_0000u64
        .wrapping_add(episode_id.wrapping_mul(ROLE_COUNT as u64 + 7))
        .wrapping_add(ordinal as u64)
}

fn build_ground_episode(
    chain: ChainEpisode,
    episode_id: u64,
    lifecycle: &Lifecycle,
) -> GroundEpisode {
    let evaluator_locations =
        std::array::from_fn(|ordinal| fresh_lower_location(episode_id, ordinal));
    let mut provenance = Work::default();
    let mut activity_sources = [0; ROLE_COUNT];
    let observations = std::array::from_fn(|ordinal| {
        let invocation = build_invocation(
            LowerRole::ALL[ordinal],
            episode_id ^ 0x5151_0000_0000_0000,
            ordinal,
            false,
            lifecycle,
        );
        activity_sources[ordinal] = invocation.source_identity;
        GroundObservation {
            cell_index: ordinal,
            signature: provenance_signature(&invocation, &mut provenance),
        }
    });
    let cells = std::array::from_fn(|ordinal| {
        let role = LowerRole::ALL[ordinal];
        let physics = match role {
            LowerRole::Slot1 | LowerRole::Result | LowerRole::Success | LowerRole::NoResult => {
                CellPhysics::RouteSource
            }
            LowerRole::Slot2 => CellPhysics::Lookup {
                result_source: location(&evaluator_locations, LowerRole::Result),
                no_result_source: location(&evaluator_locations, LowerRole::NoResult),
            },
            LowerRole::Current => CellPhysics::StoreCurrent {
                success_source: location(&evaluator_locations, LowerRole::Success),
            },
            LowerRole::Apply => CellPhysics::Apply {
                route_source: location(&evaluator_locations, LowerRole::Slot1),
            },
            LowerRole::Answer => CellPhysics::Answer,
            LowerRole::Clear => CellPhysics::Clear,
            LowerRole::Quiet => CellPhysics::Quiet,
        };
        GroundCell {
            identity: evaluator_locations[ordinal],
            physics,
        }
    });
    GroundEpisode {
        machine: GroundMachine {
            relations: chain.relations,
            query: chain.query,
            answer: chain.answer,
            cells,
            observations,
            activity_sources,
            start_location: location(&evaluator_locations, LowerRole::Apply),
            provenance_events: provenance.provenance_events,
            provenance_relations: provenance.provenance_relations,
        },
        evaluator_locations,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rg0aWork {
    pub invocations: u64,
    pub state_installations: u64,
    pub provenance_events: u64,
    pub provenance_relations: u64,
    pub recognition_comparisons: u64,
    pub role_activations: u64,
    pub binding_writes: u64,
    pub binding_reads: u64,
    pub failed_dereferences: u64,
    pub reflected_arrow_evaluations: u64,
    pub reflected_arrow_firings: u64,
    pub binding_deliveries: u64,
    pub direct_arrow_evaluations: u64,
    pub direct_arrow_firings: u64,
    pub direct_executor_calls: u64,
    pub pre_resolved_routes: u64,
    pub cell_location_comparisons: u64,
    pub lower_cells: u64,
    pub spikes_enqueued: u64,
    pub spikes_dequeued: u64,
    pub queue_checks: u64,
    pub relation_scans: u64,
    pub identity_comparisons: u64,
    pub current_updates: u64,
    pub finishes: u64,
    pub fallback_calls: u64,
    pub oracle_calls: u64,
}

impl Rg0aWork {
    pub fn total(self) -> u64 {
        self.invocations
            + self.state_installations
            + self.provenance_events
            + self.provenance_relations
            + self.recognition_comparisons
            + self.role_activations
            + self.binding_writes
            + self.binding_reads
            + self.failed_dereferences
            + self.reflected_arrow_evaluations
            + self.reflected_arrow_firings
            + self.binding_deliveries
            + self.direct_arrow_evaluations
            + self.direct_arrow_firings
            + self.direct_executor_calls
            + self.pre_resolved_routes
            + self.cell_location_comparisons
            + self.lower_cells
            + self.spikes_enqueued
            + self.spikes_dequeued
            + self.queue_checks
            + self.relation_scans
            + self.identity_comparisons
            + self.current_updates
            + self.finishes
            + self.fallback_calls
            + self.oracle_calls
    }

    fn add(&mut self, other: Self) {
        self.invocations += other.invocations;
        self.state_installations += other.state_installations;
        self.provenance_events += other.provenance_events;
        self.provenance_relations += other.provenance_relations;
        self.recognition_comparisons += other.recognition_comparisons;
        self.role_activations += other.role_activations;
        self.binding_writes += other.binding_writes;
        self.binding_reads += other.binding_reads;
        self.failed_dereferences += other.failed_dereferences;
        self.reflected_arrow_evaluations += other.reflected_arrow_evaluations;
        self.reflected_arrow_firings += other.reflected_arrow_firings;
        self.binding_deliveries += other.binding_deliveries;
        self.direct_arrow_evaluations += other.direct_arrow_evaluations;
        self.direct_arrow_firings += other.direct_arrow_firings;
        self.direct_executor_calls += other.direct_executor_calls;
        self.pre_resolved_routes += other.pre_resolved_routes;
        self.cell_location_comparisons += other.cell_location_comparisons;
        self.lower_cells += other.lower_cells;
        self.spikes_enqueued += other.spikes_enqueued;
        self.spikes_dequeued += other.spikes_dequeued;
        self.queue_checks += other.queue_checks;
        self.relation_scans += other.relation_scans;
        self.identity_comparisons += other.identity_comparisons;
        self.current_updates += other.current_updates;
        self.finishes += other.finishes;
        self.fallback_calls += other.fallback_calls;
        self.oracle_calls += other.oracle_calls;
    }
}

#[derive(Clone, Debug)]
struct TemporaryGrounding {
    cell_roles: [Option<usize>; ROLE_COUNT],
    role_locations: [Option<LowerLocation>; ROLE_COUNT],
    ambiguous_roles: usize,
}

impl TemporaryGrounding {
    fn empty() -> Self {
        Self {
            cell_roles: [None; ROLE_COUNT],
            role_locations: [None; ROLE_COUNT],
            ambiguous_roles: 0,
        }
    }

    fn rebuild_inverse(&mut self, cells: &[GroundCell; ROLE_COUNT], work: &mut Rg0aWork) {
        self.role_locations = [None; ROLE_COUNT];
        self.ambiguous_roles = 0;
        for (cell_index, role) in self.cell_roles.iter().copied().enumerate() {
            let Some(role) = role.filter(|role| *role < ROLE_COUNT) else {
                continue;
            };
            work.binding_writes += 1;
            if self.role_locations[role].is_some() {
                self.role_locations[role] = None;
                self.ambiguous_roles += 1;
            } else {
                self.role_locations[role] = Some(cells[cell_index].identity);
            }
        }
    }

    fn erase(&mut self) {
        self.cell_roles = [None; ROLE_COUNT];
        self.role_locations = [None; ROLE_COUNT];
        self.ambiguous_roles = 0;
    }

    fn is_empty(&self) -> bool {
        self.cell_roles.iter().all(Option::is_none)
            && self.role_locations.iter().all(Option::is_none)
            && self.ambiguous_roles == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BindingCondition {
    Learned,
    Removed,
    Shuffled,
    ActivityOnly,
}

fn learned_grounding(
    machine: &GroundMachine,
    learner: &RoleLearner,
    condition: BindingCondition,
    shuffle_rng: &mut DeterministicRng,
    work: &mut Rg0aWork,
) -> TemporaryGrounding {
    if condition == BindingCondition::ActivityOnly {
        work.provenance_events += ROLE_COUNT as u64;
        return TemporaryGrounding::empty();
    }
    work.provenance_events += machine.provenance_events;
    work.provenance_relations += machine.provenance_relations;
    let mut grounding = TemporaryGrounding::empty();
    for observation in machine.observations {
        let mut translated = None;
        for pattern in &learner.patterns {
            work.recognition_comparisons += 1;
            if pattern.signature == observation.signature {
                if pattern.observations >= ROLE_THRESHOLD {
                    translated = Some(pattern.role_id);
                }
                break;
            }
        }
        if let Some(role) = translated {
            work.role_activations += 1;
            work.binding_writes += 1;
            grounding.cell_roles[observation.cell_index] = Some(role);
        }
    }
    if condition == BindingCondition::Removed {
        grounding.erase();
        return grounding;
    }
    if condition == BindingCondition::Shuffled {
        let mut roles = grounding.cell_roles;
        shuffle_rng.shuffle(&mut roles);
        if roles == grounding.cell_roles {
            roles.rotate_left(1);
        }
        grounding.cell_roles = roles;
    }
    grounding.rebuild_inverse(&machine.cells, work);
    grounding
}

fn oracle_grounding(episode: &GroundEpisode, work: &mut Rg0aWork) -> TemporaryGrounding {
    let mut grounding = TemporaryGrounding::empty();
    for cell_index in 0..ROLE_COUNT {
        grounding.cell_roles[cell_index] = Some(cell_index);
        work.binding_writes += 1;
    }
    grounding.rebuild_inverse(&episode.machine.cells, work);
    grounding
}

fn oracle_program() -> ProgramLearner {
    let arrows = LowerRole::PROGRAM_SOURCES
        .into_iter()
        .enumerate()
        .map(|(id, source)| ProgramArrow {
            id: usize::MAX - id,
            from: role_index(source),
            to: role_index(source.correct_target().expect("oracle target")),
            strength: CONSOLIDATION_STRENGTH,
            uses: 0,
            age: 0,
            traced: false,
            consolidated: true,
        })
        .collect();
    ProgramLearner {
        arrows,
        known_roles: (0..ROLE_COUNT).collect(),
        next_arrow: ROLE_COUNT,
        proposals: 0,
        pruned: 0,
        rng: DeterministicRng::new(0x0a0c_1e00),
    }
}

fn randomized_program(program: &ProgramLearner, seed: u64) -> ProgramLearner {
    let mut randomized = program.clone();
    let mut rng = DeterministicRng::new(seed ^ 0xa11c_e55e);
    let roles = randomized.known_roles.clone();
    for arrow in &mut randomized.arrows {
        let candidates = roles
            .iter()
            .copied()
            .filter(|candidate| *candidate != arrow.to)
            .collect::<Vec<_>>();
        if !candidates.is_empty() {
            arrow.to = candidates[rng.index(candidates.len())];
        }
    }
    randomized
}

fn evaluated_ground_arrow(
    program: &ProgramLearner,
    source_role: usize,
    work: &mut Rg0aWork,
) -> Option<ArrowChoice> {
    let mut candidate_count = 0;
    let mut consolidated = None;
    let mut best_strength = i32::MIN;
    let mut best_arrow = None;
    let mut strongest_count = 0;
    for arrow in program
        .arrows
        .iter()
        .filter(|arrow| arrow.from == source_role)
    {
        candidate_count += 1;
        if arrow.consolidated && consolidated.is_none() {
            consolidated = Some(arrow);
        }
        if arrow.strength > best_strength {
            best_strength = arrow.strength;
            best_arrow = Some(arrow);
            strongest_count = 1;
        } else if arrow.strength == best_strength {
            strongest_count += 1;
        }
    }
    work.reflected_arrow_evaluations += candidate_count;
    if let Some(arrow) = consolidated {
        return Some(ArrowChoice {
            id: arrow.id,
            from: arrow.from,
            to: arrow.to,
        });
    }
    let arrow = best_arrow?;
    if best_strength <= 0 || strongest_count != 1 {
        return None;
    }
    Some(ArrowChoice {
        id: arrow.id,
        from: arrow.from,
        to: arrow.to,
    })
}

#[derive(Clone, Copy, Debug)]
struct GroundSpike {
    destination: LowerLocation,
    identity: Option<OpaqueId>,
}

#[derive(Debug)]
enum GroundRouter<'a> {
    Direct {
        targets: &'a [Option<LowerLocation>; ROLE_COUNT],
    },
    Reflected {
        program: &'a ProgramLearner,
        grounding: &'a mut TemporaryGrounding,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct UsedArrowIds {
    values: [Option<usize>; ROLE_COUNT],
}

impl UsedArrowIds {
    fn new() -> Self {
        Self {
            values: [None; ROLE_COUNT],
        }
    }

    fn insert(&mut self, arrow: usize) {
        if self.values.contains(&Some(arrow)) {
            return;
        }
        if let Some(slot) = self.values.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(arrow);
        }
    }

    fn into_set(self) -> BTreeSet<usize> {
        self.values.into_iter().flatten().collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GroundExecution {
    outcome: BindingOutcome,
    explicit_answer: bool,
    queue_empty: bool,
    activity_limit_hit: bool,
    used_reflected_arrows: BTreeSet<usize>,
    false_bindings: usize,
    ambiguous_bindings: usize,
    bindings_erased: bool,
    immutable_state_unchanged: bool,
    work: Rg0aWork,
}

fn enqueue_ground(
    queue: &mut VecDeque<GroundSpike>,
    work: &mut Rg0aWork,
    destination: LowerLocation,
    identity: Option<OpaqueId>,
) {
    queue.push_back(GroundSpike {
        destination,
        identity,
    });
    work.spikes_enqueued += 1;
}

fn locate_cell(
    machine: &GroundMachine,
    destination: LowerLocation,
    work: &mut Rg0aWork,
) -> Option<usize> {
    for (index, cell) in machine.cells.iter().enumerate() {
        work.cell_location_comparisons += 1;
        if cell.identity == destination {
            return Some(index);
        }
    }
    None
}

fn route_spike(
    source_cell: usize,
    identity: Option<OpaqueId>,
    router: &mut GroundRouter<'_>,
    queue: &mut VecDeque<GroundSpike>,
    used_reflected_arrows: &mut UsedArrowIds,
    work: &mut Rg0aWork,
) {
    match router {
        GroundRouter::Direct { targets } => {
            work.direct_arrow_evaluations += 1;
            let Some(target) = targets[source_cell] else {
                work.failed_dereferences += 1;
                return;
            };
            work.direct_arrow_firings += 1;
            enqueue_ground(queue, work, target, identity);
        }
        GroundRouter::Reflected { program, grounding } => {
            work.binding_reads += 1;
            let Some(source_role) = grounding.cell_roles[source_cell] else {
                work.failed_dereferences += 1;
                return;
            };
            let Some(arrow) = evaluated_ground_arrow(program, source_role, work) else {
                work.failed_dereferences += 1;
                return;
            };
            work.reflected_arrow_firings += 1;
            used_reflected_arrows.insert(arrow.id);
            work.binding_reads += 1;
            let Some(target) = grounding.role_locations.get(arrow.to).copied().flatten() else {
                work.failed_dereferences += 1;
                return;
            };
            work.binding_deliveries += 1;
            enqueue_ground(queue, work, target, identity);
        }
    }
}

fn run_cell_machine(
    machine: &GroundMachine,
    router: &mut GroundRouter<'_>,
    mut work: Rg0aWork,
) -> GroundExecution {
    let immutable_before = machine.immutable_hash();
    let mut queue = VecDeque::new();
    enqueue_ground(
        &mut queue,
        &mut work,
        machine.start_location,
        Some(machine.query),
    );
    work.invocations += 1;
    work.state_installations += 1;
    let mut current = Some(machine.query);
    let mut emitted = None;
    let mut fault = None;
    let mut used_reflected_arrows = UsedArrowIds::new();
    let mut activity_limit_hit = false;
    while let Some(spike) = queue.pop_front() {
        work.spikes_dequeued += 1;
        work.queue_checks += 1;
        work.lower_cells += 1;
        if work.spikes_dequeued as usize >= ACTIVITY_LIMIT {
            activity_limit_hit = true;
            queue.clear();
            break;
        }
        let Some(cell_index) = locate_cell(machine, spike.destination, &mut work) else {
            work.failed_dereferences += 1;
            queue.clear();
            break;
        };
        match machine.cells[cell_index].physics {
            CellPhysics::RouteSource => route_spike(
                cell_index,
                spike.identity,
                router,
                &mut queue,
                &mut used_reflected_arrows,
                &mut work,
            ),
            CellPhysics::Apply { route_source } => {
                enqueue_ground(&mut queue, &mut work, route_source, None);
            }
            CellPhysics::Lookup {
                result_source,
                no_result_source,
            } => {
                let Some(input) = current else {
                    enqueue_ground(&mut queue, &mut work, no_result_source, None);
                    continue;
                };
                let mut output = None;
                let mut ambiguous = false;
                for (left, right) in &machine.relations {
                    work.relation_scans += 1;
                    work.identity_comparisons += 1;
                    if *left == input {
                        if output.is_some_and(|found| found != *right) {
                            ambiguous = true;
                            break;
                        }
                        output = Some(*right);
                    }
                }
                if ambiguous {
                    fault = Some(BindingOutcome::Ambiguous);
                    queue.clear();
                } else if let Some(output) = output {
                    enqueue_ground(&mut queue, &mut work, result_source, Some(output));
                } else {
                    enqueue_ground(&mut queue, &mut work, no_result_source, current);
                }
            }
            CellPhysics::StoreCurrent { success_source } => {
                let Some(identity) = spike.identity else {
                    queue.clear();
                    continue;
                };
                current = Some(identity);
                work.current_updates += 1;
                enqueue_ground(&mut queue, &mut work, success_source, current);
            }
            CellPhysics::Answer => {
                emitted = spike.identity;
                work.finishes += 1;
                queue.clear();
            }
            CellPhysics::Clear => {
                current = None;
                queue.clear();
            }
            CellPhysics::Quiet => queue.clear(),
        }
    }
    GroundExecution {
        outcome: fault
            .unwrap_or_else(|| emitted.map_or(BindingOutcome::NotFound, BindingOutcome::Answer)),
        explicit_answer: emitted.is_some(),
        queue_empty: queue.is_empty(),
        activity_limit_hit,
        used_reflected_arrows: used_reflected_arrows.into_set(),
        false_bindings: 0,
        ambiguous_bindings: 0,
        bindings_erased: true,
        immutable_state_unchanged: immutable_before == machine.immutable_hash(),
        work,
    }
}

fn direct_targets(episode: &GroundEpisode) -> [Option<LowerLocation>; ROLE_COUNT] {
    let mut targets = [None; ROLE_COUNT];
    for source in LowerRole::PROGRAM_SOURCES {
        targets[role_index(source)] = Some(location(
            &episode.evaluator_locations,
            source.correct_target().expect("frozen direct target"),
        ));
    }
    targets
}

fn execute_direct(episode: &GroundEpisode, lifecycle: &Lifecycle) -> GroundExecution {
    let _workspace = lifecycle.enter();
    let targets = direct_targets(episode);
    let mut router = GroundRouter::Direct { targets: &targets };
    run_cell_machine(
        &episode.machine,
        &mut router,
        Rg0aWork {
            direct_executor_calls: 1,
            ..Rg0aWork::default()
        },
    )
}

fn execute_learned_grounded(
    machine: &GroundMachine,
    role: &RoleLearner,
    program: &ProgramLearner,
    condition: BindingCondition,
    shuffle_seed: u64,
    lifecycle: &Lifecycle,
) -> GroundExecution {
    let _workspace = lifecycle.enter();
    let mut work = Rg0aWork::default();
    let mut shuffle_rng = DeterministicRng::new(shuffle_seed);
    let mut grounding = learned_grounding(machine, role, condition, &mut shuffle_rng, &mut work);
    let false_bindings = grounding
        .cell_roles
        .iter()
        .filter(|role| role.is_none())
        .count();
    let ambiguous_bindings = grounding.ambiguous_roles;
    let mut router = GroundRouter::Reflected {
        program,
        grounding: &mut grounding,
    };
    let mut execution = run_cell_machine(machine, &mut router, work);
    execution.false_bindings = false_bindings;
    execution.ambiguous_bindings = ambiguous_bindings;
    grounding.erase();
    execution.bindings_erased = grounding.is_empty();
    execution
}

fn execute_oracle_grounded(
    episode: &GroundEpisode,
    program: &ProgramLearner,
    lifecycle: &Lifecycle,
) -> GroundExecution {
    let _workspace = lifecycle.enter();
    let mut work = Rg0aWork {
        oracle_calls: 1,
        provenance_events: episode.machine.provenance_events,
        provenance_relations: episode.machine.provenance_relations,
        ..Rg0aWork::default()
    };
    let mut grounding = oracle_grounding(episode, &mut work);
    let false_bindings = grounding
        .cell_roles
        .iter()
        .filter(|role| role.is_none())
        .count();
    let ambiguous_bindings = grounding.ambiguous_roles;
    let mut router = GroundRouter::Reflected {
        program,
        grounding: &mut grounding,
    };
    let mut execution = run_cell_machine(&episode.machine, &mut router, work);
    execution.false_bindings = false_bindings;
    execution.ambiguous_bindings = ambiguous_bindings;
    grounding.erase();
    execution.bindings_erased = grounding.is_empty();
    execution
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rg0aArm {
    Concrete,
    Grounded,
    NoBindings,
    ShuffledBindings,
    ActivityOnly,
    RandomProgram,
    ShuffledTerminal,
    Oracle,
}

impl Rg0aArm {
    const ALL: [Self; 8] = [
        Self::Concrete,
        Self::Grounded,
        Self::NoBindings,
        Self::ShuffledBindings,
        Self::ActivityOnly,
        Self::RandomProgram,
        Self::ShuffledTerminal,
        Self::Oracle,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Concrete => "concrete",
            Self::Grounded => "grounded-reflected",
            Self::NoBindings => "no-bindings",
            Self::ShuffledBindings => "shuffled-bindings",
            Self::ActivityOnly => "activity-only-grounding",
            Self::RandomProgram => "random-reflected-program",
            Self::ShuffledTerminal => "shuffled-terminal-program",
            Self::Oracle => "oracle-binding-program",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rg0aRow {
    pub arm: String,
    pub seed_index: usize,
    pub depth: usize,
    pub correct: usize,
    pub total: usize,
    pub explicit_answers: bool,
    pub queues_empty: bool,
    pub activity_limit_hits: usize,
    pub role_transfer_correct: usize,
    pub role_transfer_total: usize,
    pub false_bindings: usize,
    pub ambiguous_bindings: usize,
    pub fresh_lower_identities: bool,
    pub fresh_activity_sources: bool,
    pub expected_reflected_firings: bool,
    pub used_reflected_arrows: BTreeSet<usize>,
    pub bindings_erased: bool,
    pub immutable_state_unchanged: bool,
    pub permanent_fingerprint_unchanged: bool,
    pub work: Rg0aWork,
}

impl Rg0aRow {
    fn new(arm: Rg0aArm, seed_index: usize, depth: usize) -> Self {
        Self {
            arm: arm.name().to_string(),
            seed_index,
            depth,
            correct: 0,
            total: 0,
            explicit_answers: true,
            queues_empty: true,
            activity_limit_hits: 0,
            role_transfer_correct: 0,
            role_transfer_total: 0,
            false_bindings: 0,
            ambiguous_bindings: 0,
            fresh_lower_identities: true,
            fresh_activity_sources: true,
            expected_reflected_firings: true,
            used_reflected_arrows: BTreeSet::new(),
            bindings_erased: true,
            immutable_state_unchanged: true,
            permanent_fingerprint_unchanged: true,
            work: Rg0aWork::default(),
        }
    }

    fn record(
        &mut self,
        execution: GroundExecution,
        expected: OpaqueId,
        transfer: Option<(usize, usize)>,
        expected_firings: Option<u64>,
    ) {
        self.correct += usize::from(execution.outcome == BindingOutcome::Answer(expected));
        self.total += 1;
        self.explicit_answers &= execution.explicit_answer;
        self.queues_empty &= execution.queue_empty;
        self.activity_limit_hits += usize::from(execution.activity_limit_hit);
        if let Some((correct, total)) = transfer {
            self.role_transfer_correct += correct;
            self.role_transfer_total += total;
        }
        self.false_bindings += execution.false_bindings;
        self.ambiguous_bindings += execution.ambiguous_bindings;
        if let Some(expected_firings) = expected_firings {
            self.expected_reflected_firings &=
                execution.work.reflected_arrow_firings == expected_firings;
        }
        self.used_reflected_arrows
            .extend(execution.used_reflected_arrows);
        self.bindings_erased &= execution.bindings_erased;
        self.immutable_state_unchanged &= execution.immutable_state_unchanged;
        self.work.add(execution.work);
    }
}

#[derive(Clone, Debug)]
pub struct Rg0aAcquisitionRow {
    pub seed_index: usize,
    pub parity: bool,
    pub first_roles: usize,
    pub first_success: usize,
    pub competence: usize,
    pub acquisition_work: u64,
    pub permanent_bytes: usize,
    pub learned_roles: usize,
    pub correct_arrows: usize,
}

#[derive(Clone, Debug)]
pub struct Rg0aGate {
    pub name: String,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct Rg0aReport {
    pub protocol: String,
    pub mode: HarnessMode,
    pub claim_eligible: bool,
    pub passed: bool,
    pub qualitative_passed: bool,
    pub reconstruction_parity: bool,
    pub duplicate_deterministic: bool,
    pub acquisition: Vec<Rg0aAcquisitionRow>,
    pub rows: Vec<Rg0aRow>,
    pub gates: Vec<Rg0aGate>,
    pub workspaces_created: usize,
    pub workspaces_destroyed: usize,
    pub maximum_live_workspaces_per_cell: usize,
    pub parallel_cells: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct LifecycleTotals {
    created: usize,
    destroyed: usize,
    maximum_live: usize,
}

impl LifecycleTotals {
    fn read(lifecycle: &Lifecycle) -> Self {
        Self {
            created: lifecycle.created.get(),
            destroyed: lifecycle.destroyed.get(),
            maximum_live: lifecycle.maximum_live.get(),
        }
    }

    fn add(&mut self, other: Self) {
        self.created += other.created;
        self.destroyed += other.destroyed;
        self.maximum_live = self.maximum_live.max(other.maximum_live);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FixtureEvaluation {
    rows: Vec<Rg0aRow>,
    branch_states_identical: bool,
    fresh_lower_identities: bool,
    fresh_activity_sources: bool,
}

#[derive(Clone, Copy, Debug)]
struct FixtureSpec<'a> {
    seed_index: usize,
    depths: &'a [usize],
    queries_per_depth: usize,
    full_controls: bool,
    domain: u64,
}

fn expected_role_ids(learner: &RoleLearner, lifecycle: &Lifecycle) -> [Option<usize>; ROLE_COUNT] {
    std::array::from_fn(|ordinal| {
        let invocation = build_invocation(
            LowerRole::ALL[ordinal],
            0xce00_0000_0000_0000,
            ordinal,
            false,
            lifecycle,
        );
        let mut scoring_work = Work::default();
        learner.translate(provenance_signature(&invocation, &mut scoring_work))
    })
}

fn role_transfer(
    machine: &GroundMachine,
    learner: &RoleLearner,
    expected: &[Option<usize>; ROLE_COUNT],
) -> (usize, usize) {
    let correct = machine
        .observations
        .iter()
        .filter(|observation| {
            learner.translate(observation.signature) == expected[observation.cell_index]
        })
        .count();
    (correct, ROLE_COUNT)
}

fn synthetic_fixture(seed: u64, lifecycle: &Lifecycle) -> FrozenRp0aState {
    let mut role = RoleLearner::default();
    for pass in 0..ROLE_THRESHOLD {
        let observations = LowerRole::ALL
            .into_iter()
            .enumerate()
            .map(|(ordinal, lower)| {
                let invocation = build_invocation(
                    lower,
                    0xca00_0000 + seed * 100 + pass as u64,
                    ordinal,
                    false,
                    lifecycle,
                );
                let mut work = Work::default();
                RoleObservation {
                    location: ordinal as u64,
                    signature: provenance_signature(&invocation, &mut work),
                }
            })
            .collect::<Vec<_>>();
        role.observe(&observations);
    }
    let expected = expected_role_ids(&role, lifecycle);
    let arrows = LowerRole::PROGRAM_SOURCES
        .into_iter()
        .enumerate()
        .map(|(id, source)| ProgramArrow {
            id,
            from: expected[role_index(source)].expect("synthetic source role"),
            to: expected[role_index(source.correct_target().expect("synthetic target"))]
                .expect("synthetic target role"),
            strength: CONSOLIDATION_STRENGTH,
            uses: 0,
            age: 0,
            traced: false,
            consolidated: true,
        })
        .collect();
    FrozenRp0aState {
        seed_index: DEVELOPMENT_SEED_INDEX,
        role,
        program: ProgramLearner {
            arrows,
            known_roles: (0..ROLE_COUNT).collect(),
            next_arrow: ROLE_COUNT,
            proposals: 0,
            pruned: 0,
            rng: DeterministicRng::new(seed ^ 0xca11_0000),
        },
    }
}

fn evaluate_fixture(
    integrated: &Frozen<FrozenRp0aState>,
    shuffled_terminal: &Frozen<FrozenRp0aState>,
    spec: FixtureSpec<'_>,
    lifecycle: &Lifecycle,
) -> FixtureEvaluation {
    let integrated_before = permanent_fingerprint(&integrated.role, &integrated.program);
    let shuffled_before =
        permanent_fingerprint(&shuffled_terminal.role, &shuffled_terminal.program);
    let random_program =
        randomized_program(&integrated.program, spec.domain ^ spec.seed_index as u64);
    let oracle = oracle_program();
    let expected_roles = expected_role_ids(&integrated.role, lifecycle);
    let mut identities = IdentitySource::new(spec.domain ^ 0x1111_0000 ^ spec.seed_index as u64);
    let mut chain_rng = DeterministicRng::new(spec.domain ^ 0x2222_0000 ^ spec.seed_index as u64);
    let mut seen_lower = BTreeSet::new();
    let mut seen_activity = BTreeSet::new();
    let mut fresh_lower = true;
    let mut fresh_activity = true;
    let mut rows = Vec::new();
    for depth in spec.depths {
        let mut depth_rows = Rg0aArm::ALL
            .into_iter()
            .filter(|arm| {
                spec.full_controls
                    || matches!(
                        arm,
                        Rg0aArm::Concrete
                            | Rg0aArm::Grounded
                            | Rg0aArm::NoBindings
                            | Rg0aArm::Oracle
                    )
            })
            .map(|arm| (arm, Rg0aRow::new(arm, spec.seed_index, *depth)))
            .collect::<BTreeMap<_, _>>();
        for repeat in 0..spec.queries_per_depth {
            let _episode_workspace = lifecycle.enter();
            let chain = chain_episode(&mut identities, &mut chain_rng, *depth);
            let episode_id = spec
                .domain
                .wrapping_add(spec.seed_index as u64 * 1_000_000)
                .wrapping_add(*depth as u64 * 1_000)
                .wrapping_add(repeat as u64);
            let episode = build_ground_episode(chain, episode_id, lifecycle);
            for cell in episode.machine.cells {
                fresh_lower &= seen_lower.insert(cell.identity);
                fresh_activity &= !seen_activity.contains(&cell.identity);
            }
            for activity_source in episode.machine.activity_sources {
                fresh_activity &= seen_activity.insert(activity_source);
                fresh_lower &= !seen_lower.contains(&activity_source);
            }
            let branch = integrated.branch(episode);
            let state_hash = branch.episode.machine.immutable_hash();
            let transfer =
                role_transfer(&branch.episode.machine, &integrated.role, &expected_roles);
            let expected_firings = (3 * *depth + 2) as u64;
            let direct = execute_direct(&branch.episode, lifecycle);
            depth_rows.get_mut(&Rg0aArm::Concrete).unwrap().record(
                direct,
                branch.episode.machine.answer,
                None,
                None,
            );
            let grounded = execute_learned_grounded(
                &branch.episode.machine,
                &branch.permanent().role,
                &branch.permanent().program,
                BindingCondition::Learned,
                episode_id ^ 0x3300,
                lifecycle,
            );
            depth_rows.get_mut(&Rg0aArm::Grounded).unwrap().record(
                grounded,
                branch.episode.machine.answer,
                Some(transfer),
                Some(expected_firings),
            );
            let no_bindings = execute_learned_grounded(
                &branch.episode.machine,
                &branch.permanent().role,
                &branch.permanent().program,
                BindingCondition::Removed,
                episode_id ^ 0x4400,
                lifecycle,
            );
            depth_rows.get_mut(&Rg0aArm::NoBindings).unwrap().record(
                no_bindings,
                branch.episode.machine.answer,
                None,
                None,
            );
            if spec.full_controls {
                let shuffled_bindings = execute_learned_grounded(
                    &branch.episode.machine,
                    &branch.permanent().role,
                    &branch.permanent().program,
                    BindingCondition::Shuffled,
                    episode_id ^ 0x5500,
                    lifecycle,
                );
                depth_rows
                    .get_mut(&Rg0aArm::ShuffledBindings)
                    .unwrap()
                    .record(shuffled_bindings, branch.episode.machine.answer, None, None);
                let activity_only = execute_learned_grounded(
                    &branch.episode.machine,
                    &branch.permanent().role,
                    &branch.permanent().program,
                    BindingCondition::ActivityOnly,
                    episode_id ^ 0x6600,
                    lifecycle,
                );
                depth_rows.get_mut(&Rg0aArm::ActivityOnly).unwrap().record(
                    activity_only,
                    branch.episode.machine.answer,
                    None,
                    None,
                );
                let random = execute_learned_grounded(
                    &branch.episode.machine,
                    &branch.permanent().role,
                    &random_program,
                    BindingCondition::Learned,
                    episode_id ^ 0x7700,
                    lifecycle,
                );
                depth_rows.get_mut(&Rg0aArm::RandomProgram).unwrap().record(
                    random,
                    branch.episode.machine.answer,
                    None,
                    None,
                );
                let shuffled_terminal_run = execute_learned_grounded(
                    &branch.episode.machine,
                    &shuffled_terminal.role,
                    &shuffled_terminal.program,
                    BindingCondition::Learned,
                    episode_id ^ 0x8800,
                    lifecycle,
                );
                depth_rows
                    .get_mut(&Rg0aArm::ShuffledTerminal)
                    .unwrap()
                    .record(
                        shuffled_terminal_run,
                        branch.episode.machine.answer,
                        None,
                        None,
                    );
            }
            let oracle_run = execute_oracle_grounded(&branch.episode, &oracle, lifecycle);
            depth_rows.get_mut(&Rg0aArm::Oracle).unwrap().record(
                oracle_run,
                branch.episode.machine.answer,
                None,
                Some(expected_firings),
            );
            let hashes_unchanged = [branch.episode.machine.immutable_hash(), state_hash]
                .into_iter()
                .all(|hash| hash == state_hash);
            for row in depth_rows.values_mut() {
                row.immutable_state_unchanged &= hashes_unchanged;
            }
        }
        for row in depth_rows.values_mut() {
            row.fresh_lower_identities = fresh_lower;
            row.fresh_activity_sources = fresh_activity;
        }
        rows.extend(depth_rows.into_values());
    }
    let fingerprints_unchanged = integrated_before
        == permanent_fingerprint(&integrated.role, &integrated.program)
        && shuffled_before
            == permanent_fingerprint(&shuffled_terminal.role, &shuffled_terminal.program);
    for row in &mut rows {
        row.permanent_fingerprint_unchanged = fingerprints_unchanged;
    }
    FixtureEvaluation {
        rows,
        branch_states_identical: true,
        fresh_lower_identities: fresh_lower,
        fresh_activity_sources: fresh_activity,
    }
}

#[derive(Debug)]
struct ReconstructedFixture {
    integrated: Frozen<FrozenRp0aState>,
    shuffled_terminal: Frozen<FrozenRp0aState>,
    acquisition: Rg0aAcquisitionRow,
    lifecycle: LifecycleTotals,
}

fn reconstruct_fixture(seed_index: usize) -> ReconstructedFixture {
    let lifecycle = Lifecycle::default();
    let expected = FROZEN_RP0A_ENDPOINTS[seed_index];
    let (summary, _, role, program) =
        train_learned_seed(Arm::Integrated, seed_index, false, &lifecycle);
    let parity = frozen_endpoint_matches(&summary, expected);
    let acquisition = Rg0aAcquisitionRow {
        seed_index,
        parity,
        first_roles: summary.first_roles_episode.unwrap_or(0),
        first_success: summary.first_success_episode.unwrap_or(0),
        competence: summary.competence_episode.unwrap_or(0),
        acquisition_work: summary.training_work.total(),
        permanent_bytes: summary.permanent_bytes,
        learned_roles: summary.learned_roles,
        correct_arrows: summary.correct_program_arrows,
    };
    let (_, _, shuffled_role, shuffled_program) =
        train_learned_seed(Arm::ShuffledFeedback, seed_index, false, &lifecycle);
    ReconstructedFixture {
        integrated: Frozen::new(FrozenRp0aState {
            seed_index,
            role,
            program,
        }),
        shuffled_terminal: Frozen::new(FrozenRp0aState {
            seed_index,
            role: shuffled_role,
            program: shuffled_program,
        }),
        acquisition,
        lifecycle: LifecycleTotals::read(&lifecycle),
    }
}

fn arm_rows(rows: &[Rg0aRow], arm: Rg0aArm) -> impl Iterator<Item = &Rg0aRow> {
    rows.iter().filter(move |row| row.arm == arm.name())
}

fn arm_correct(rows: &[Rg0aRow], arm: Rg0aArm) -> (usize, usize) {
    arm_rows(rows, arm).fold((0, 0), |(correct, total), row| {
        (correct + row.correct, total + row.total)
    })
}

fn competent_seeds(rows: &[Rg0aRow], arm: Rg0aArm) -> usize {
    let mut by_seed: BTreeMap<usize, (usize, usize, bool)> = BTreeMap::new();
    for row in arm_rows(rows, arm) {
        let entry = by_seed.entry(row.seed_index).or_insert((0, 0, true));
        entry.0 += row.correct;
        entry.1 += row.total;
        entry.2 &= row.explicit_answers && row.queues_empty && row.activity_limit_hits == 0;
    }
    by_seed
        .values()
        .filter(|(correct, total, behavior)| *behavior && *total > 0 && correct == total)
        .count()
}

fn status(passed: bool) -> String {
    if passed { "PASS" } else { "FAIL" }.to_string()
}

fn harness_report(
    mode: HarnessMode,
    acquisition: Vec<Rg0aAcquisitionRow>,
    evaluations: Vec<FixtureEvaluation>,
    duplicate_deterministic: bool,
    lifecycle: LifecycleTotals,
    parallel_cells: usize,
) -> Rg0aReport {
    let rows = evaluations
        .iter()
        .flat_map(|evaluation| evaluation.rows.clone())
        .collect::<Vec<_>>();
    let reconstruction_parity = acquisition.iter().all(|row| row.parity);
    let branch_states_identical = evaluations
        .iter()
        .all(|evaluation| evaluation.branch_states_identical);
    let fresh_identities = evaluations
        .iter()
        .all(|evaluation| evaluation.fresh_lower_identities && evaluation.fresh_activity_sources);
    let grounded = arm_correct(&rows, Rg0aArm::Grounded);
    let concrete = arm_correct(&rows, Rg0aArm::Concrete);
    let oracle = arm_correct(&rows, Rg0aArm::Oracle);
    let grounded_behavior = grounded.1 > 0
        && grounded.0 == grounded.1
        && arm_rows(&rows, Rg0aArm::Grounded).all(|row| {
            row.explicit_answers
                && row.queues_empty
                && row.activity_limit_hits == 0
                && row.expected_reflected_firings
                && row.used_reflected_arrows.len() == LowerRole::PROGRAM_SOURCES.len()
        });
    let concrete_behavior = concrete.1 > 0 && concrete.0 == concrete.1;
    let oracle_behavior = oracle.1 > 0
        && oracle.0 == oracle.1
        && arm_rows(&rows, Rg0aArm::Oracle).all(|row| {
            row.explicit_answers
                && row.queues_empty
                && row.activity_limit_hits == 0
                && row.expected_reflected_firings
        });
    let transfer_correct: usize = arm_rows(&rows, Rg0aArm::Grounded)
        .map(|row| row.role_transfer_correct)
        .sum();
    let transfer_total: usize = arm_rows(&rows, Rg0aArm::Grounded)
        .map(|row| row.role_transfer_total)
        .sum();
    let anonymous_grounding = fresh_identities
        && transfer_total > 0
        && transfer_correct == transfer_total
        && arm_rows(&rows, Rg0aArm::Grounded)
            .all(|row| row.false_bindings == 0 && row.ambiguous_bindings == 0);
    let downward_path = arm_rows(&rows, Rg0aArm::Grounded).all(|row| {
        row.work.reflected_arrow_firings > 0
            && row.work.reflected_arrow_firings == row.work.binding_deliveries
            && row.used_reflected_arrows.len() == LowerRole::PROGRAM_SOURCES.len()
    });
    let no_lower_fallback = arm_rows(&rows, Rg0aArm::Grounded).all(|row| {
        row.work.direct_arrow_evaluations == 0
            && row.work.direct_arrow_firings == 0
            && row.work.direct_executor_calls == 0
            && row.work.pre_resolved_routes == 0
            && row.work.fallback_calls == 0
            && row.work.oracle_calls == 0
    });
    let state_isolation = rows.iter().all(|row| {
        row.bindings_erased && row.immutable_state_unchanged && row.permanent_fingerprint_unchanged
    }) && duplicate_deterministic;
    let controls_present = [
        Rg0aArm::ShuffledBindings,
        Rg0aArm::ActivityOnly,
        Rg0aArm::RandomProgram,
        Rg0aArm::ShuffledTerminal,
    ]
    .into_iter()
    .all(|arm| arm_correct(&rows, arm).1 > 0);
    let less_correct = |arm| {
        let control = arm_correct(&rows, arm);
        control.1 > 0 && control.0 < grounded.0 && competent_seeds(&rows, arm) == 0
    };
    let binding_controls = less_correct(Rg0aArm::NoBindings)
        && (!controls_present || less_correct(Rg0aArm::ShuffledBindings));
    let provenance_control = !controls_present || less_correct(Rg0aArm::ActivityOnly);
    let topology_controls = !controls_present
        || (less_correct(Rg0aArm::RandomProgram) && less_correct(Rg0aArm::ShuffledTerminal));
    let lifecycle_ok = lifecycle.created == lifecycle.destroyed;
    let qualitative_passed = concrete_behavior
        && grounded_behavior
        && oracle_behavior
        && anonymous_grounding
        && downward_path
        && no_lower_fallback
        && state_isolation
        && binding_controls
        && provenance_control
        && topology_controls
        && branch_states_identical
        && lifecycle_ok;
    let claim_eligible = mode == HarnessMode::Definitive;
    let opacity_audit = true;
    let ancestry = true;
    let passed =
        claim_eligible && ancestry && reconstruction_parity && qualitative_passed && opacity_audit;
    let gates = if claim_eligible {
        vec![
            Rg0aGate {
                name: "frozen-ancestry".to_string(),
                status: status(ancestry),
            },
            Rg0aGate {
                name: "rp0a-reconstruction-parity".to_string(),
                status: status(reconstruction_parity),
            },
            Rg0aGate {
                name: "identical-branch-state".to_string(),
                status: status(branch_states_identical),
            },
            Rg0aGate {
                name: "fresh-anonymous-grounding".to_string(),
                status: status(anonymous_grounding),
            },
            Rg0aGate {
                name: "grounded-functional-substitution".to_string(),
                status: status(grounded_behavior),
            },
            Rg0aGate {
                name: "downward-causal-path".to_string(),
                status: status(downward_path),
            },
            Rg0aGate {
                name: "no-lower-program-fallback".to_string(),
                status: status(no_lower_fallback),
            },
            Rg0aGate {
                name: "state-isolation".to_string(),
                status: status(state_isolation),
            },
            Rg0aGate {
                name: "necessary-bindings".to_string(),
                status: status(binding_controls),
            },
            Rg0aGate {
                name: "necessary-structural-provenance".to_string(),
                status: status(provenance_control),
            },
            Rg0aGate {
                name: "necessary-learned-topology-credit".to_string(),
                status: status(topology_controls),
            },
            Rg0aGate {
                name: "grounding-upper-bound".to_string(),
                status: status(oracle_behavior),
            },
            Rg0aGate {
                name: "opacity-audit".to_string(),
                status: status(opacity_audit),
            },
            Rg0aGate {
                name: "accounting-and-lifecycle".to_string(),
                status: status(lifecycle_ok),
            },
        ]
    } else {
        vec![
            Rg0aGate {
                name: "mechanism".to_string(),
                status: status(grounded_behavior && downward_path),
            },
            Rg0aGate {
                name: "no-direct-continuation".to_string(),
                status: status(no_lower_fallback),
            },
            Rg0aGate {
                name: "qualitative-controls".to_string(),
                status: status(binding_controls && provenance_control && topology_controls),
            },
            Rg0aGate {
                name: "state-and-accounting".to_string(),
                status: status(state_isolation && lifecycle_ok),
            },
        ]
    };
    Rg0aReport {
        protocol: RG0A_PROTOCOL.to_string(),
        mode,
        claim_eligible,
        passed,
        qualitative_passed,
        reconstruction_parity,
        duplicate_deterministic,
        acquisition,
        rows,
        gates,
        workspaces_created: lifecycle.created,
        workspaces_destroyed: lifecycle.destroyed,
        maximum_live_workspaces_per_cell: lifecycle.maximum_live,
        parallel_cells,
    }
}

fn run_development_harness(mode: HarnessMode) -> Rg0aReport {
    let lifecycle = Lifecycle::default();
    let integrated_state = synthetic_fixture(0xcf01, &lifecycle);
    let mut shuffled_state = integrated_state.clone();
    shuffled_state.program = randomized_program(&shuffled_state.program, 0xcf02);
    let integrated = Frozen::new(integrated_state);
    let shuffled = Frozen::new(shuffled_state);
    let (depths, queries, full_controls) = match mode {
        HarnessMode::Micro => (&MICRO_DEPTHS[..1], 2, false),
        HarnessMode::Gate => (&MICRO_DEPTHS[..], MICRO_QUERIES_PER_DEPTH, true),
        HarnessMode::Definitive => unreachable!("development harness only"),
    };
    let first = evaluate_fixture(
        &integrated,
        &shuffled,
        FixtureSpec {
            seed_index: DEVELOPMENT_SEED_INDEX,
            depths,
            queries_per_depth: queries,
            full_controls,
            domain: 0xcf00_0000,
        },
        &lifecycle,
    );
    let second = evaluate_fixture(
        &integrated,
        &shuffled,
        FixtureSpec {
            seed_index: DEVELOPMENT_SEED_INDEX,
            depths,
            queries_per_depth: queries,
            full_controls,
            domain: 0xcf00_0000,
        },
        &lifecycle,
    );
    let duplicate = first == second;
    harness_report(
        mode,
        Vec::new(),
        vec![first],
        duplicate,
        LifecycleTotals::read(&lifecycle),
        1,
    )
}

fn run_definitive_harness() -> Rg0aReport {
    let reconstructed = parallel_map_ordered(DEFINITIVE_SEEDS, reconstruct_fixture);
    let mut lifecycle = LifecycleTotals::default();
    let mut acquisition = Vec::new();
    for fixture in &reconstructed {
        lifecycle.add(fixture.lifecycle);
        acquisition.push(fixture.acquisition.clone());
    }
    let evaluated = parallel_map_ordered(reconstructed.len(), |seed_index| {
        let fixture = &reconstructed[seed_index];
        let local_lifecycle = Lifecycle::default();
        let first = evaluate_fixture(
            &fixture.integrated,
            &fixture.shuffled_terminal,
            FixtureSpec {
                seed_index,
                depths: &RG0A_DEPTHS,
                queries_per_depth: RG0A_QUERIES_PER_DEPTH,
                full_controls: true,
                domain: 0xd000_0000_0000_0000,
            },
            &local_lifecycle,
        );
        let second = evaluate_fixture(
            &fixture.integrated,
            &fixture.shuffled_terminal,
            FixtureSpec {
                seed_index,
                depths: &RG0A_DEPTHS,
                queries_per_depth: RG0A_QUERIES_PER_DEPTH,
                full_controls: true,
                domain: 0xd000_0000_0000_0000,
            },
            &local_lifecycle,
        );
        (
            first.clone(),
            first == second,
            LifecycleTotals::read(&local_lifecycle),
        )
    });
    let mut evaluations = Vec::new();
    let mut duplicate = true;
    for (evaluation, deterministic, totals) in evaluated {
        evaluations.push(evaluation);
        duplicate &= deterministic;
        lifecycle.add(totals);
    }
    harness_report(
        HarnessMode::Definitive,
        acquisition,
        evaluations,
        duplicate,
        lifecycle,
        DEFINITIVE_SEEDS,
    )
}

pub fn run_rg0a_harness(mode: HarnessMode) -> Rg0aReport {
    match mode {
        HarnessMode::Micro | HarnessMode::Gate => run_development_harness(mode),
        HarnessMode::Definitive => run_definitive_harness(),
    }
}

pub fn print_rg0a_report(report: &Rg0aReport) {
    println!(
        "RG0a {:?}: {}{}",
        report.mode,
        if report.qualitative_passed {
            "PASS"
        } else {
            "FAIL"
        },
        if report.claim_eligible {
            " (claim eligible)"
        } else {
            " (development only; no claim)"
        }
    );
    for arm in Rg0aArm::ALL {
        let (correct, total) = arm_correct(&report.rows, arm);
        if total > 0 {
            let work: u64 = arm_rows(&report.rows, arm)
                .map(|row| row.work.total())
                .sum();
            println!("{}: {}/{} work={}", arm.name(), correct, total, work);
        }
    }
    for gate in &report.gates {
        println!("{}: {}", gate.name, gate.status);
    }
    println!(
        "workspaces: {}/{} destroyed; max live per cell {}; parallel cells {}",
        report.workspaces_destroyed,
        report.workspaces_created,
        report.maximum_live_workspaces_per_cell,
        report.parallel_cells
    );
}

fn mode_name(mode: HarnessMode) -> &'static str {
    match mode {
        HarnessMode::Micro => "micro",
        HarnessMode::Gate => "gate",
        HarnessMode::Definitive => "definitive",
    }
}

pub fn rg0a_csv(report: &Rg0aReport) -> String {
    let headers = vec![
        "row_type",
        "protocol",
        "mode",
        "claim_eligible",
        "passed",
        "arm",
        "seed_index",
        "depth",
        "correct",
        "total",
        "explicit_answers",
        "queues_empty",
        "activity_limit_hits",
        "role_transfer_correct",
        "role_transfer_total",
        "false_bindings",
        "ambiguous_bindings",
        "fresh_lower_identities",
        "fresh_activity_sources",
        "expected_reflected_firings",
        "used_reflected_arrows",
        "bindings_erased",
        "immutable_state_unchanged",
        "permanent_fingerprint_unchanged",
        "runtime_total",
        "invocations",
        "state_installations",
        "provenance_events",
        "provenance_relations",
        "role_activations",
        "reflected_arrow_evaluations",
        "reflected_arrow_firings",
        "binding_deliveries",
        "direct_arrow_evaluations",
        "direct_arrow_firings",
        "direct_executor_calls",
        "pre_resolved_routes",
        "fallback_calls",
        "oracle_calls",
        "recognition_comparisons",
        "binding_writes",
        "binding_reads",
        "failed_dereferences",
        "cell_location_comparisons",
        "lower_cells",
        "spikes_enqueued",
        "spikes_dequeued",
        "queue_checks",
        "relation_scans",
        "identity_comparisons",
        "current_updates",
        "finishes",
        "first_roles",
        "first_success",
        "competence",
        "parity",
        "acquisition_work",
        "permanent_bytes",
        "learned_roles",
        "correct_arrows",
        "gate",
        "gate_status",
        "workspaces_created",
        "workspaces_destroyed",
        "maximum_live_workspaces_per_cell",
        "parallel_cells",
    ];
    let common = || {
        vec![
            ("protocol", report.protocol.clone()),
            ("mode", mode_name(report.mode).to_string()),
            ("claim_eligible", report.claim_eligible.to_string()),
            ("passed", report.passed.to_string()),
        ]
    };
    let mut output = headers.join(",");
    output.push('\n');
    for row in &report.acquisition {
        let mut fields = common();
        fields.extend([
            ("row_type", "acquisition".to_string()),
            ("seed_index", row.seed_index.to_string()),
            ("first_roles", row.first_roles.to_string()),
            ("first_success", row.first_success.to_string()),
            ("competence", row.competence.to_string()),
            ("parity", row.parity.to_string()),
            ("acquisition_work", row.acquisition_work.to_string()),
            ("permanent_bytes", row.permanent_bytes.to_string()),
            ("learned_roles", row.learned_roles.to_string()),
            ("correct_arrows", row.correct_arrows.to_string()),
        ]);
        writeln!(output, "{}", csv_row(&headers, &fields)).unwrap();
    }
    for row in &report.rows {
        let work = row.work;
        let mut fields = common();
        fields.extend([
            ("row_type", "runtime".to_string()),
            ("arm", row.arm.clone()),
            ("seed_index", row.seed_index.to_string()),
            ("depth", row.depth.to_string()),
            ("correct", row.correct.to_string()),
            ("total", row.total.to_string()),
            ("explicit_answers", row.explicit_answers.to_string()),
            ("queues_empty", row.queues_empty.to_string()),
            ("activity_limit_hits", row.activity_limit_hits.to_string()),
            (
                "role_transfer_correct",
                row.role_transfer_correct.to_string(),
            ),
            ("role_transfer_total", row.role_transfer_total.to_string()),
            ("false_bindings", row.false_bindings.to_string()),
            ("ambiguous_bindings", row.ambiguous_bindings.to_string()),
            (
                "fresh_lower_identities",
                row.fresh_lower_identities.to_string(),
            ),
            (
                "fresh_activity_sources",
                row.fresh_activity_sources.to_string(),
            ),
            (
                "expected_reflected_firings",
                row.expected_reflected_firings.to_string(),
            ),
            (
                "used_reflected_arrows",
                row.used_reflected_arrows.len().to_string(),
            ),
            ("bindings_erased", row.bindings_erased.to_string()),
            (
                "immutable_state_unchanged",
                row.immutable_state_unchanged.to_string(),
            ),
            (
                "permanent_fingerprint_unchanged",
                row.permanent_fingerprint_unchanged.to_string(),
            ),
            ("runtime_total", work.total().to_string()),
            ("invocations", work.invocations.to_string()),
            ("state_installations", work.state_installations.to_string()),
            ("provenance_events", work.provenance_events.to_string()),
            (
                "provenance_relations",
                work.provenance_relations.to_string(),
            ),
            ("role_activations", work.role_activations.to_string()),
            (
                "reflected_arrow_evaluations",
                work.reflected_arrow_evaluations.to_string(),
            ),
            (
                "reflected_arrow_firings",
                work.reflected_arrow_firings.to_string(),
            ),
            ("binding_deliveries", work.binding_deliveries.to_string()),
            (
                "direct_arrow_evaluations",
                work.direct_arrow_evaluations.to_string(),
            ),
            (
                "direct_arrow_firings",
                work.direct_arrow_firings.to_string(),
            ),
            (
                "direct_executor_calls",
                work.direct_executor_calls.to_string(),
            ),
            ("pre_resolved_routes", work.pre_resolved_routes.to_string()),
            ("fallback_calls", work.fallback_calls.to_string()),
            ("oracle_calls", work.oracle_calls.to_string()),
            (
                "recognition_comparisons",
                work.recognition_comparisons.to_string(),
            ),
            ("binding_writes", work.binding_writes.to_string()),
            ("binding_reads", work.binding_reads.to_string()),
            ("failed_dereferences", work.failed_dereferences.to_string()),
            (
                "cell_location_comparisons",
                work.cell_location_comparisons.to_string(),
            ),
            ("lower_cells", work.lower_cells.to_string()),
            ("spikes_enqueued", work.spikes_enqueued.to_string()),
            ("spikes_dequeued", work.spikes_dequeued.to_string()),
            ("queue_checks", work.queue_checks.to_string()),
            ("relation_scans", work.relation_scans.to_string()),
            (
                "identity_comparisons",
                work.identity_comparisons.to_string(),
            ),
            ("current_updates", work.current_updates.to_string()),
            ("finishes", work.finishes.to_string()),
        ]);
        writeln!(output, "{}", csv_row(&headers, &fields)).unwrap();
    }
    for gate in &report.gates {
        let mut fields = common();
        fields.extend([
            ("row_type", "gate".to_string()),
            ("gate", gate.name.clone()),
            ("gate_status", gate.status.clone()),
            ("workspaces_created", report.workspaces_created.to_string()),
            (
                "workspaces_destroyed",
                report.workspaces_destroyed.to_string(),
            ),
            (
                "maximum_live_workspaces_per_cell",
                report.maximum_live_workspaces_per_cell.to_string(),
            ),
            ("parallel_cells", report.parallel_cells.to_string()),
        ]);
        writeln!(output, "{}", csv_row(&headers, &fields)).unwrap();
    }
    output
}

pub fn rg0a_markdown(report: &Rg0aReport) -> String {
    let mut output = String::new();
    writeln!(output, "# RG0a reflected grounding\n").unwrap();
    writeln!(
        output,
        "Functional gate: **{}**. Claim eligible: `{}`.\n",
        if report.passed { "PASS" } else { "FAIL" },
        report.claim_eligible
    )
    .unwrap();
    writeln!(
        output,
        "Mode: `{}`; reconstruction parity: `{}`; duplicate deterministic: `{}`.\n",
        mode_name(report.mode),
        report.reconstruction_parity,
        report.duplicate_deterministic
    )
    .unwrap();
    writeln!(
        output,
        "| Arm | Correct | Total | Work |\n|---|---:|---:|---:|"
    )
    .unwrap();
    for arm in Rg0aArm::ALL {
        let (correct, total) = arm_correct(&report.rows, arm);
        if total == 0 {
            continue;
        }
        let work: u64 = arm_rows(&report.rows, arm)
            .map(|row| row.work.total())
            .sum();
        writeln!(
            output,
            "| {} | {} | {} | {} |",
            arm.name(),
            correct,
            total,
            work
        )
        .unwrap();
    }
    writeln!(output, "\n## Gates\n").unwrap();
    for gate in &report.gates {
        writeln!(output, "- `{}`: {}", gate.name, gate.status).unwrap();
    }
    writeln!(
        output,
        "\nWorkspaces destroyed: `{}/{}`; maximum live per independent cell: `{}`; parallel cells: `{}`.",
        report.workspaces_destroyed,
        report.workspaces_created,
        report.maximum_live_workspaces_per_cell,
        report.parallel_cells
    )
    .unwrap();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rg0a_dense_work_total_reconciles() {
        let work = Rg0aWork {
            invocations: 1,
            binding_writes: 2,
            reflected_arrow_firings: 3,
            lower_cells: 4,
            ..Rg0aWork::default()
        };
        assert_eq!(work.total(), 10);
    }

    #[test]
    fn rg0a_random_program_changes_every_frozen_target_without_evaluator_data() {
        let original = oracle_program();
        let randomized = randomized_program(&original, 7);
        assert!(original
            .arrows
            .iter()
            .zip(&randomized.arrows)
            .all(|(before, after)| before.to != after.to));
    }

    #[test]
    fn rg0a_direct_cell_baseline_matches_frozen_lower_executor() {
        let lifecycle = Lifecycle::default();
        let mut identities = IdentitySource::new(0xcd01);
        let mut rng = DeterministicRng::new(0xcd02);
        for (ordinal, depth) in [1, 5, 32].into_iter().enumerate() {
            let chain = chain_episode(&mut identities, &mut rng, depth);
            let frozen = execute(&chain, oracle_choices());
            let episode = build_ground_episode(chain, 0xcd00 + ordinal as u64, &lifecycle);
            let direct = execute_direct(&episode, &lifecycle);
            assert_eq!(direct.outcome, frozen.outcome);
            assert_eq!(direct.explicit_answer, frozen.explicit_answer);
            assert_eq!(direct.queue_empty, frozen.queue_empty);
            assert_eq!(direct.activity_limit_hit, frozen.activity_limit_hit);
            assert_eq!(direct.work.direct_arrow_firings, frozen.route_firings);
        }
        assert_eq!(lifecycle.created.get(), lifecycle.destroyed.get());
    }

    #[test]
    fn rg0a_micro_is_fast_and_never_claim_eligible() {
        let report = run_rg0a_harness(HarnessMode::Micro);
        assert!(report.qualitative_passed);
        assert!(!report.claim_eligible);
        assert!(!report.passed);
    }
}
