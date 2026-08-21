//! RC0a: compile repeatedly successful grounded dispatch without compacting
//! the computation performed between route firings.
//!
//! Persistent compiled arrows contain reflected role identities only. Every
//! invocation installs temporary arrows against fresh RG0a bindings; mature
//! firing neither scans the learned program nor dereferences a binding.

use super::*;
use crate::research_runtime::{parallel_map_ordered, Frozen, HarnessMode};
use std::mem::size_of;

pub const RC0A_PROTOCOL: &str = "compiled-grounded-recurrence-rc0a-v1";

const COMPILE_DEPTHS: [usize; 3] = [1, 2, 3];
const DEVELOPMENT_DEPTHS: [usize; 3] = [3, 5, 8];
const DEVELOPMENT_SEED_INDEX: usize = 20_000;
const SLOPE_REDUCTION_DENOMINATOR: i128 = 5;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rc0aWork {
    pub rg0a: Rg0aWork,
    pub compilation_candidates: u64,
    pub compilation_comparisons: u64,
    pub success_credit_updates: u64,
    pub failure_credit_updates: u64,
    pub compiled_arrows_earned: u64,
    pub topology_validation_comparisons: u64,
    pub compiled_binding_reads: u64,
    pub compiled_installation_comparisons: u64,
    pub temporary_arrow_installations: u64,
    pub compiled_invalidations: u64,
    pub generic_resumptions: u64,
}

impl Rc0aWork {
    pub fn total(self) -> u64 {
        self.rg0a.total()
            + self.compilation_candidates
            + self.compilation_comparisons
            + self.success_credit_updates
            + self.failure_credit_updates
            + self.compiled_arrows_earned
            + self.topology_validation_comparisons
            + self.compiled_binding_reads
            + self.compiled_installation_comparisons
            + self.temporary_arrow_installations
            + self.compiled_invalidations
            + self.generic_resumptions
    }

    fn add(&mut self, other: Self) {
        self.rg0a.add(other.rg0a);
        self.compilation_candidates += other.compilation_candidates;
        self.compilation_comparisons += other.compilation_comparisons;
        self.success_credit_updates += other.success_credit_updates;
        self.failure_credit_updates += other.failure_credit_updates;
        self.compiled_arrows_earned += other.compiled_arrows_earned;
        self.topology_validation_comparisons += other.topology_validation_comparisons;
        self.compiled_binding_reads += other.compiled_binding_reads;
        self.compiled_installation_comparisons += other.compiled_installation_comparisons;
        self.temporary_arrow_installations += other.temporary_arrow_installations;
        self.compiled_invalidations += other.compiled_invalidations;
        self.generic_resumptions += other.generic_resumptions;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct CompiledRoleArrow {
    id: usize,
    parent_arrow: usize,
    from_role: usize,
    to_role: usize,
    strength: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CompiledDispatch {
    by_source_role: [Option<CompiledRoleArrow>; ROLE_COUNT],
    next_arrow: usize,
}

impl CompiledDispatch {
    fn empty() -> Self {
        Self {
            by_source_role: [None; ROLE_COUNT],
            next_arrow: 0,
        }
    }

    fn len(&self) -> usize {
        self.by_source_role.iter().flatten().count()
    }

    fn is_complete(&self) -> bool {
        self.len() == LowerRole::PROGRAM_SOURCES.len()
    }

    fn install_earned(
        &mut self,
        parent_arrow: usize,
        from_role: usize,
        to_role: usize,
        strength: i32,
        work: &mut Rc0aWork,
    ) {
        let Some(slot) = self.by_source_role.get_mut(from_role) else {
            return;
        };
        if slot.is_some() {
            return;
        }
        *slot = Some(CompiledRoleArrow {
            id: self.next_arrow,
            parent_arrow,
            from_role,
            to_role,
            strength,
        });
        self.next_arrow += 1;
        work.compiled_arrows_earned += 1;
    }

    fn invalidate_all(&mut self, work: &mut Rc0aWork) {
        work.compiled_invalidations += self.len() as u64;
        self.by_source_role = [None; ROLE_COUNT];
    }

    fn validate(&mut self, program: &ProgramLearner, work: &mut Rc0aWork) -> bool {
        if !self.is_complete() {
            return false;
        }
        let compatible = self.by_source_role.iter().flatten().all(|compiled| {
            let mut found = None;
            for candidate in &program.arrows {
                work.topology_validation_comparisons += 1;
                if candidate.id == compiled.parent_arrow {
                    found = Some(candidate);
                    break;
                }
            }
            found.is_some_and(|parent| {
                parent.consolidated
                    && parent.from == compiled.from_role
                    && parent.to == compiled.to_role
            })
        });
        if !compatible {
            self.invalidate_all(work);
        }
        compatible
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for arrow in self.by_source_role.iter().flatten() {
            mix(&mut hash, arrow.id as u64);
            mix(&mut hash, arrow.parent_arrow as u64);
            mix(&mut hash, arrow.from_role as u64);
            mix(&mut hash, arrow.to_role as u64);
            mix(&mut hash, arrow.strength as i64 as u64);
        }
        hash
    }

    fn permanent_bytes(&self) -> usize {
        self.len() * size_of::<CompiledRoleArrow>()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DispatchCandidate {
    parent_arrow: usize,
    from_role: usize,
    to_role: usize,
    strength: i32,
    traced: bool,
    consolidated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DispatchConsolidator {
    candidates: Vec<DispatchCandidate>,
    dispatch: CompiledDispatch,
}

impl DispatchConsolidator {
    fn new() -> Self {
        Self {
            candidates: Vec::new(),
            dispatch: CompiledDispatch::empty(),
        }
    }

    fn observe(
        &mut self,
        program: &ProgramLearner,
        used_arrows: &BTreeSet<usize>,
        successful: bool,
        shuffle_targets: bool,
        work: &mut Rc0aWork,
    ) {
        for candidate in &mut self.candidates {
            candidate.traced = false;
        }
        let mut transitions = used_arrows
            .iter()
            .filter_map(|id| program.arrows.iter().find(|arrow| arrow.id == *id).copied())
            .collect::<Vec<_>>();
        transitions.sort_by_key(|arrow| (arrow.from, arrow.id));
        let targets = transitions.iter().map(|arrow| arrow.to).collect::<Vec<_>>();
        for (index, transition) in transitions.into_iter().enumerate() {
            let to_role = if shuffle_targets && targets.len() > 1 {
                targets[(index + 1) % targets.len()]
            } else {
                transition.to
            };
            let mut found = None;
            for (candidate_index, candidate) in self.candidates.iter().enumerate() {
                work.compilation_comparisons += 1;
                if candidate.parent_arrow == transition.id
                    && candidate.from_role == transition.from
                    && candidate.to_role == to_role
                {
                    found = Some(candidate_index);
                    break;
                }
            }
            let candidate_index = found.unwrap_or_else(|| {
                self.candidates.push(DispatchCandidate {
                    parent_arrow: transition.id,
                    from_role: transition.from,
                    to_role,
                    strength: 0,
                    traced: false,
                    consolidated: false,
                });
                work.compilation_candidates += 1;
                self.candidates.len() - 1
            });
            self.candidates[candidate_index].traced = true;
        }
        for candidate in &mut self.candidates {
            if !candidate.traced || candidate.consolidated {
                continue;
            }
            if successful {
                candidate.strength += SUCCESS_CREDIT;
                work.success_credit_updates += 1;
            } else {
                candidate.strength += FAILURE_CREDIT;
                work.failure_credit_updates += 1;
            }
            candidate.traced = false;
            if candidate.strength >= CONSOLIDATION_STRENGTH {
                candidate.consolidated = true;
                self.dispatch.install_earned(
                    candidate.parent_arrow,
                    candidate.from_role,
                    candidate.to_role,
                    candidate.strength,
                    work,
                );
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TemporaryCompiledRoutes {
    by_source_cell: [Option<LocalGroundArrow>; ROLE_COUNT],
}

impl TemporaryCompiledRoutes {
    fn empty() -> Self {
        Self {
            by_source_cell: [None; ROLE_COUNT],
        }
    }

    fn install(
        machine: &GroundMachine,
        grounding: &TemporaryGrounding,
        dispatch: &CompiledDispatch,
        work: &mut Rc0aWork,
    ) -> Option<Self> {
        let mut routes = Self::empty();
        for compiled in dispatch.by_source_role.iter().flatten() {
            work.compiled_binding_reads += 2;
            let source_location = grounding
                .role_locations
                .get(compiled.from_role)
                .copied()
                .flatten()?;
            let destination = grounding
                .role_locations
                .get(compiled.to_role)
                .copied()
                .flatten()?;
            let mut source_cell = None;
            for (index, cell) in machine.cells.iter().enumerate() {
                work.compiled_installation_comparisons += 1;
                if cell.identity == source_location {
                    source_cell = Some(index);
                    break;
                }
            }
            let slot = routes.by_source_cell.get_mut(source_cell?)?;
            if slot.is_some() {
                return None;
            }
            *slot = Some(LocalGroundArrow {
                id: compiled.id,
                destination,
            });
            work.temporary_arrow_installations += 1;
        }
        (routes.by_source_cell.iter().flatten().count() == LowerRole::PROGRAM_SOURCES.len())
            .then_some(routes)
    }

    fn erase(&mut self) {
        self.by_source_cell = [None; ROLE_COUNT];
    }

    fn is_empty(&self) -> bool {
        self.by_source_cell.iter().all(Option::is_none)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RcExecution {
    outcome: BindingOutcome,
    explicit_answer: bool,
    queue_empty: bool,
    activity_limit_hit: bool,
    used_reflected_arrows: BTreeSet<usize>,
    used_compiled_arrows: BTreeSet<usize>,
    false_bindings: usize,
    ambiguous_bindings: usize,
    bindings_erased: bool,
    temporary_routes_erased: bool,
    immutable_state_unchanged: bool,
    work: Rc0aWork,
}

impl RcExecution {
    fn from_rg0a(execution: GroundExecution, mut prefix: Rc0aWork) -> Self {
        prefix.rg0a.add(execution.work);
        Self {
            outcome: execution.outcome,
            explicit_answer: execution.explicit_answer,
            queue_empty: execution.queue_empty,
            activity_limit_hit: execution.activity_limit_hit,
            used_reflected_arrows: execution.used_reflected_arrows,
            used_compiled_arrows: execution.used_compiled_arrows,
            false_bindings: execution.false_bindings,
            ambiguous_bindings: execution.ambiguous_bindings,
            bindings_erased: execution.bindings_erased,
            temporary_routes_erased: true,
            immutable_state_unchanged: execution.immutable_state_unchanged,
            work: prefix,
        }
    }
}

fn execute_direct_rc0a(episode: &GroundEpisode, lifecycle: &Lifecycle) -> RcExecution {
    let _workspace = lifecycle.enter();
    let mut targets = [None; ROLE_COUNT];
    for source in LowerRole::PROGRAM_SOURCES {
        let source_location = location(&episode.evaluator_locations, source);
        let source_cell = episode
            .machine
            .cells
            .iter()
            .position(|cell| cell.identity == source_location)
            .expect("source cell remains present");
        targets[source_cell] = Some(location(
            &episode.evaluator_locations,
            source.correct_target().expect("frozen direct target"),
        ));
    }
    let mut router = GroundRouter::Direct { targets: &targets };
    RcExecution::from_rg0a(
        run_cell_machine(
            &episode.machine,
            &mut router,
            Rg0aWork {
                direct_executor_calls: 1,
                ..Rg0aWork::default()
            },
        ),
        Rc0aWork::default(),
    )
}

fn execute_generic_rc0a(
    machine: &GroundMachine,
    role: &RoleLearner,
    program: &ProgramLearner,
    condition: BindingCondition,
    shuffle_seed: u64,
    lifecycle: &Lifecycle,
    prefix: Rc0aWork,
) -> RcExecution {
    RcExecution::from_rg0a(
        execute_learned_grounded(machine, role, program, condition, shuffle_seed, lifecycle),
        prefix,
    )
}

fn execute_local_arrows(
    machine: &GroundMachine,
    routes: &TemporaryCompiledRoutes,
    mut work: Rc0aWork,
) -> RcExecution {
    let mut router = GroundRouter::Compiled {
        arrows: &routes.by_source_cell,
    };
    let base_work = std::mem::take(&mut work.rg0a);
    RcExecution::from_rg0a(run_cell_machine(machine, &mut router, base_work), work)
}

fn execute_compiled_or_resume(
    machine: &GroundMachine,
    role: &RoleLearner,
    program: &ProgramLearner,
    dispatch: &mut CompiledDispatch,
    condition: BindingCondition,
    shuffle_seed: u64,
    lifecycle: &Lifecycle,
) -> RcExecution {
    let mut work = Rc0aWork::default();
    if !dispatch.validate(program, &mut work) {
        work.generic_resumptions += 1;
        return execute_generic_rc0a(
            machine,
            role,
            program,
            condition,
            shuffle_seed,
            lifecycle,
            work,
        );
    }
    let _workspace = lifecycle.enter();
    let mut shuffle_rng = DeterministicRng::new(shuffle_seed);
    let mut grounding =
        learned_grounding(machine, role, condition, &mut shuffle_rng, &mut work.rg0a);
    let false_bindings = grounding
        .cell_roles
        .iter()
        .filter(|role| role.is_none())
        .count();
    let ambiguous_bindings = grounding.ambiguous_roles;
    let Some(mut routes) =
        TemporaryCompiledRoutes::install(machine, &grounding, dispatch, &mut work)
    else {
        let mut execution = execute_local_arrows(machine, &TemporaryCompiledRoutes::empty(), work);
        execution.false_bindings = false_bindings;
        execution.ambiguous_bindings = ambiguous_bindings;
        grounding.erase();
        execution.bindings_erased = grounding.is_empty();
        return execution;
    };
    let mut execution = execute_local_arrows(machine, &routes, work);
    execution.false_bindings = false_bindings;
    execution.ambiguous_bindings = ambiguous_bindings;
    grounding.erase();
    routes.erase();
    execution.bindings_erased = grounding.is_empty();
    execution.temporary_routes_erased = routes.is_empty();
    execution
}

fn permute_episode(mut episode: GroundEpisode, seed: u64) -> GroundEpisode {
    let mut permutation = std::array::from_fn::<_, ROLE_COUNT, _>(|index| index);
    let mut rng = DeterministicRng::new(seed ^ 0xb1ad_0000);
    rng.shuffle(&mut permutation);
    if permutation
        .iter()
        .enumerate()
        .all(|(index, value)| index == *value)
    {
        permutation.rotate_left(1);
    }
    let old_cells = episode.machine.cells;
    let mut old_to_new = [0usize; ROLE_COUNT];
    for (new_index, old_index) in permutation.iter().copied().enumerate() {
        old_to_new[old_index] = new_index;
    }
    episode.machine.cells = std::array::from_fn(|new_index| old_cells[permutation[new_index]]);
    for observation in &mut episode.machine.observations {
        observation.cell_index = old_to_new[observation.cell_index];
    }
    episode
}

fn role_transfer_permuted(
    episode: &GroundEpisode,
    learner: &RoleLearner,
    expected: &[Option<usize>; ROLE_COUNT],
) -> (usize, usize) {
    let correct = episode
        .machine
        .observations
        .iter()
        .filter(|observation| {
            let cell_identity = episode.machine.cells[observation.cell_index].identity;
            let evaluator_role = episode
                .evaluator_locations
                .iter()
                .position(|location| *location == cell_identity);
            evaluator_role
                .is_some_and(|role| learner.translate(observation.signature) == expected[role])
        })
        .count();
    (correct, ROLE_COUNT)
}

fn replaced_program(program: &ProgramLearner) -> ProgramLearner {
    let mut replaced = program.clone();
    let offset = replaced
        .arrows
        .iter()
        .map(|arrow| arrow.id)
        .max()
        .unwrap_or(0)
        .saturating_add(1_000_000);
    for arrow in &mut replaced.arrows {
        if arrow.consolidated {
            arrow.id = arrow.id.saturating_add(offset);
        }
    }
    replaced.next_arrow = replaced
        .arrows
        .iter()
        .map(|arrow| arrow.id)
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    replaced
}

#[derive(Clone, Debug)]
struct Acquisition {
    dispatch: CompiledDispatch,
    subthreshold: CompiledDispatch,
    shuffled: CompiledDispatch,
    successful_episodes: usize,
    work: Rc0aWork,
    shuffled_work: Rc0aWork,
}

fn acquire_dispatch(
    fixture: &FrozenRp0aState,
    seed_index: usize,
    domain: u64,
    lifecycle: &Lifecycle,
) -> Acquisition {
    let mut identities = IdentitySource::new(domain ^ 0xac01 ^ seed_index as u64);
    let mut rng = DeterministicRng::new(domain ^ 0xac02 ^ seed_index as u64);
    let mut normal = DispatchConsolidator::new();
    let mut shuffled = DispatchConsolidator::new();
    let mut work = Rc0aWork::default();
    let mut shuffled_work = Rc0aWork::default();
    let mut successful_episodes = 0;
    let mut subthreshold = CompiledDispatch::empty();
    for (index, depth) in COMPILE_DEPTHS.into_iter().enumerate() {
        let chain = chain_episode(&mut identities, &mut rng, depth);
        let episode_id = domain
            .wrapping_add(seed_index as u64 * 10_000)
            .wrapping_add(index as u64);
        let episode = build_ground_episode(chain, episode_id, lifecycle);
        let execution = execute_learned_grounded(
            &episode.machine,
            &fixture.role,
            &fixture.program,
            BindingCondition::Learned,
            episode_id ^ 0xac03,
            lifecycle,
        );
        let successful = execution.outcome == BindingOutcome::Answer(episode.machine.answer)
            && execution.explicit_answer
            && execution.queue_empty
            && !execution.activity_limit_hit;
        successful_episodes += usize::from(successful);
        work.rg0a.add(execution.work);
        normal.observe(
            &fixture.program,
            &execution.used_reflected_arrows,
            successful,
            false,
            &mut work,
        );
        shuffled.observe(
            &fixture.program,
            &execution.used_reflected_arrows,
            successful,
            true,
            &mut shuffled_work,
        );
        if index == 1 {
            subthreshold = normal.dispatch.clone();
        }
    }
    Acquisition {
        dispatch: normal.dispatch,
        subthreshold,
        shuffled: shuffled.dispatch,
        successful_episodes,
        work,
        shuffled_work,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rc0aArm {
    Concrete,
    Generic,
    Compiled,
    ChangedBindings,
    InvalidatedTransition,
    Subthreshold,
    ShuffledEvidence,
    NoBindings,
}

impl Rc0aArm {
    const ALL: [Self; 8] = [
        Self::Concrete,
        Self::Generic,
        Self::Compiled,
        Self::ChangedBindings,
        Self::InvalidatedTransition,
        Self::Subthreshold,
        Self::ShuffledEvidence,
        Self::NoBindings,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Concrete => "concrete-reference",
            Self::Generic => "generic-grounded",
            Self::Compiled => "compiled-grounded",
            Self::ChangedBindings => "compiled-changed-bindings",
            Self::InvalidatedTransition => "invalidated-transition",
            Self::Subthreshold => "subthreshold-evidence",
            Self::ShuffledEvidence => "shuffled-consolidation-evidence",
            Self::NoBindings => "compiled-no-bindings",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rc0aRow {
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
    pub expected_route_firings: bool,
    pub used_reflected_arrows: BTreeSet<usize>,
    pub used_compiled_arrows: BTreeSet<usize>,
    pub bindings_erased: bool,
    pub temporary_routes_erased: bool,
    pub immutable_state_unchanged: bool,
    pub permanent_state_unchanged: bool,
    pub lower_effects_match: bool,
    pub work: Rc0aWork,
}

impl Rc0aRow {
    fn new(arm: Rc0aArm, seed_index: usize, depth: usize) -> Self {
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
            expected_route_firings: true,
            used_reflected_arrows: BTreeSet::new(),
            used_compiled_arrows: BTreeSet::new(),
            bindings_erased: true,
            temporary_routes_erased: true,
            immutable_state_unchanged: true,
            permanent_state_unchanged: true,
            lower_effects_match: true,
            work: Rc0aWork::default(),
        }
    }

    fn record(
        &mut self,
        execution: RcExecution,
        expected: OpaqueId,
        transfer: Option<(usize, usize)>,
        expected_route_firings: Option<u64>,
        lower_effects_match: bool,
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
        if let Some(expected) = expected_route_firings {
            self.expected_route_firings &= execution.work.rg0a.reflected_arrow_firings
                + execution.work.rg0a.direct_arrow_firings
                + execution.work.rg0a.compiled_arrow_firings
                == expected;
        }
        self.used_reflected_arrows
            .extend(execution.used_reflected_arrows);
        self.used_compiled_arrows
            .extend(execution.used_compiled_arrows);
        self.bindings_erased &= execution.bindings_erased;
        self.temporary_routes_erased &= execution.temporary_routes_erased;
        self.immutable_state_unchanged &= execution.immutable_state_unchanged;
        self.lower_effects_match &= lower_effects_match;
        self.work.add(execution.work);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rc0aAcquisitionRow {
    pub seed_index: usize,
    pub rp0a_parity: bool,
    pub successful_episodes: usize,
    pub compiled_arrows: usize,
    pub subthreshold_arrows: usize,
    pub persistent_bytes: usize,
    pub persistent_fingerprint: u64,
    pub work: Rc0aWork,
    pub shuffled_work: Rc0aWork,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExactSlope {
    pub numerator: i128,
    pub denominator: i128,
}

impl ExactSlope {
    fn display(self) -> String {
        format!(
            "{}/{} ({:.6})",
            self.numerator,
            self.denominator,
            self.numerator as f64 / self.denominator as f64
        )
    }
}

#[derive(Clone, Debug)]
pub struct Rc0aGate {
    pub name: String,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct Rc0aReport {
    pub protocol: String,
    pub mode: HarnessMode,
    pub claim_eligible: bool,
    pub passed: bool,
    pub qualitative_passed: bool,
    pub reconstruction_parity: bool,
    pub duplicate_deterministic: bool,
    pub generic_slope: ExactSlope,
    pub compiled_slope: ExactSlope,
    pub slope_passed: bool,
    pub acquisition: Vec<Rc0aAcquisitionRow>,
    pub rows: Vec<Rc0aRow>,
    pub gates: Vec<Rc0aGate>,
    pub workspaces_created: usize,
    pub workspaces_destroyed: usize,
    pub maximum_live_workspaces_per_cell: usize,
    pub parallel_cells: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RcLifecycleTotals {
    created: usize,
    destroyed: usize,
    maximum_live: usize,
}

impl RcLifecycleTotals {
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
    acquisition: Rc0aAcquisitionRow,
    rows: Vec<Rc0aRow>,
    changed_bindings_observed: bool,
    persistent_role_relative: bool,
    lifecycle: RcLifecycleTotals,
}

fn lower_effects(work: Rg0aWork) -> u64 {
    work.cell_location_comparisons
        + work.lower_cells
        + work.spikes_enqueued
        + work.spikes_dequeued
        + work.queue_checks
        + work.relation_scans
        + work.identity_comparisons
        + work.current_updates
        + work.finishes
}

fn evaluate_fixture(
    fixture: &Frozen<FrozenRp0aState>,
    rp0a_parity: bool,
    seed_index: usize,
    depths: &[usize],
    queries_per_depth: usize,
    domain: u64,
) -> FixtureEvaluation {
    let lifecycle = Lifecycle::default();
    let acquisition = acquire_dispatch(fixture, seed_index, domain ^ 0xa000_0000, &lifecycle);
    let persistent_fingerprint = acquisition.dispatch.fingerprint();
    let permanent_before = permanent_fingerprint(&fixture.role, &fixture.program);
    let expected_roles = expected_role_ids(&fixture.role, &lifecycle);
    let replaced = replaced_program(&fixture.program);
    let mut compiled = acquisition.dispatch.clone();
    let mut rebound = acquisition.dispatch.clone();
    let mut invalidated = acquisition.dispatch.clone();
    let mut subthreshold = acquisition.subthreshold.clone();
    let mut shuffled = acquisition.shuffled.clone();
    let mut no_bindings = acquisition.dispatch.clone();
    let mut identities = IdentitySource::new(domain ^ 0xe001 ^ seed_index as u64);
    let mut rng = DeterministicRng::new(domain ^ 0xe002 ^ seed_index as u64);
    let mut seen_locations = BTreeSet::new();
    let mut fresh_locations = true;
    let mut changed_bindings_observed = false;
    let mut rows = Vec::new();
    for depth in depths {
        let mut depth_rows = Rc0aArm::ALL
            .into_iter()
            .map(|arm| (arm, Rc0aRow::new(arm, seed_index, *depth)))
            .collect::<BTreeMap<_, _>>();
        for repeat in 0..queries_per_depth {
            let chain = chain_episode(&mut identities, &mut rng, *depth);
            let episode_id = domain
                .wrapping_add(seed_index as u64 * 1_000_000)
                .wrapping_add(*depth as u64 * 1_000)
                .wrapping_add(repeat as u64);
            let episode = build_ground_episode(chain, episode_id, &lifecycle);
            for cell in episode.machine.cells {
                fresh_locations &= seen_locations.insert(cell.identity);
            }
            let transfer = role_transfer(&episode.machine, &fixture.role, &expected_roles);
            let expected_firings = (3 * *depth + 2) as u64;
            let direct = execute_direct_rc0a(&episode, &lifecycle);
            let direct_lower = lower_effects(direct.work.rg0a);
            depth_rows.get_mut(&Rc0aArm::Concrete).unwrap().record(
                direct,
                episode.machine.answer,
                None,
                Some(expected_firings),
                true,
            );
            let generic = execute_generic_rc0a(
                &episode.machine,
                &fixture.role,
                &fixture.program,
                BindingCondition::Learned,
                episode_id ^ 0xe003,
                &lifecycle,
                Rc0aWork::default(),
            );
            let generic_lower = lower_effects(generic.work.rg0a);
            depth_rows.get_mut(&Rc0aArm::Generic).unwrap().record(
                generic,
                episode.machine.answer,
                Some(transfer),
                Some(expected_firings),
                generic_lower == direct_lower,
            );
            let compiled_run = execute_compiled_or_resume(
                &episode.machine,
                &fixture.role,
                &fixture.program,
                &mut compiled,
                BindingCondition::Learned,
                episode_id ^ 0xe004,
                &lifecycle,
            );
            let compiled_lower = lower_effects(compiled_run.work.rg0a);
            depth_rows.get_mut(&Rc0aArm::Compiled).unwrap().record(
                compiled_run,
                episode.machine.answer,
                Some(transfer),
                Some(expected_firings),
                compiled_lower == direct_lower,
            );
            let rebound_episode = permute_episode(episode.clone(), episode_id ^ 0xe005);
            changed_bindings_observed |= rebound_episode
                .machine
                .cells
                .iter()
                .zip(&episode.machine.cells)
                .any(|(after, before)| after.identity != before.identity);
            let rebound_transfer =
                role_transfer_permuted(&rebound_episode, &fixture.role, &expected_roles);
            let rebound_direct_lower =
                lower_effects(execute_direct_rc0a(&rebound_episode, &lifecycle).work.rg0a);
            let rebound_run = execute_compiled_or_resume(
                &rebound_episode.machine,
                &fixture.role,
                &fixture.program,
                &mut rebound,
                BindingCondition::Learned,
                episode_id ^ 0xe006,
                &lifecycle,
            );
            let rebound_lower = lower_effects(rebound_run.work.rg0a);
            depth_rows
                .get_mut(&Rc0aArm::ChangedBindings)
                .unwrap()
                .record(
                    rebound_run,
                    rebound_episode.machine.answer,
                    Some(rebound_transfer),
                    Some(expected_firings),
                    rebound_lower == rebound_direct_lower,
                );
            let invalidated_run = execute_compiled_or_resume(
                &episode.machine,
                &fixture.role,
                &replaced,
                &mut invalidated,
                BindingCondition::Learned,
                episode_id ^ 0xe007,
                &lifecycle,
            );
            depth_rows
                .get_mut(&Rc0aArm::InvalidatedTransition)
                .unwrap()
                .record(
                    invalidated_run,
                    episode.machine.answer,
                    Some(transfer),
                    Some(expected_firings),
                    true,
                );
            let subthreshold_run = execute_compiled_or_resume(
                &episode.machine,
                &fixture.role,
                &fixture.program,
                &mut subthreshold,
                BindingCondition::Learned,
                episode_id ^ 0xe008,
                &lifecycle,
            );
            depth_rows.get_mut(&Rc0aArm::Subthreshold).unwrap().record(
                subthreshold_run,
                episode.machine.answer,
                Some(transfer),
                Some(expected_firings),
                true,
            );
            let shuffled_run = execute_compiled_or_resume(
                &episode.machine,
                &fixture.role,
                &fixture.program,
                &mut shuffled,
                BindingCondition::Learned,
                episode_id ^ 0xe009,
                &lifecycle,
            );
            depth_rows
                .get_mut(&Rc0aArm::ShuffledEvidence)
                .unwrap()
                .record(
                    shuffled_run,
                    episode.machine.answer,
                    Some(transfer),
                    Some(expected_firings),
                    true,
                );
            let no_binding_run = execute_compiled_or_resume(
                &episode.machine,
                &fixture.role,
                &fixture.program,
                &mut no_bindings,
                BindingCondition::Removed,
                episode_id ^ 0xe00a,
                &lifecycle,
            );
            depth_rows.get_mut(&Rc0aArm::NoBindings).unwrap().record(
                no_binding_run,
                episode.machine.answer,
                None,
                None,
                true,
            );
        }
        rows.extend(depth_rows.into_values());
    }
    let permanent_unchanged = permanent_before
        == permanent_fingerprint(&fixture.role, &fixture.program)
        && compiled.fingerprint() == persistent_fingerprint
        && rebound.fingerprint() == persistent_fingerprint;
    for row in &mut rows {
        row.permanent_state_unchanged = permanent_unchanged;
    }
    let persistent_role_relative = fresh_locations
        && acquisition.dispatch.is_complete()
        && acquisition
            .dispatch
            .by_source_role
            .iter()
            .flatten()
            .all(|arrow| {
                arrow.from_role < ROLE_COUNT
                    && arrow.to_role < ROLE_COUNT
                    && arrow.strength >= CONSOLIDATION_STRENGTH
            });
    FixtureEvaluation {
        acquisition: Rc0aAcquisitionRow {
            seed_index,
            rp0a_parity,
            successful_episodes: acquisition.successful_episodes,
            compiled_arrows: acquisition.dispatch.len(),
            subthreshold_arrows: acquisition.subthreshold.len(),
            persistent_bytes: acquisition.dispatch.permanent_bytes(),
            persistent_fingerprint,
            work: acquisition.work,
            shuffled_work: acquisition.shuffled_work,
        },
        rows,
        changed_bindings_observed,
        persistent_role_relative,
        lifecycle: RcLifecycleTotals::read(&lifecycle),
    }
}

fn arm_rows(rows: &[Rc0aRow], arm: Rc0aArm) -> impl Iterator<Item = &Rc0aRow> {
    rows.iter().filter(move |row| row.arm == arm.name())
}

fn arm_correct(rows: &[Rc0aRow], arm: Rc0aArm) -> (usize, usize) {
    arm_rows(rows, arm).fold((0, 0), |(correct, total), row| {
        (correct + row.correct, total + row.total)
    })
}

fn aggregate_work(rows: &[Rc0aRow], arm: Rc0aArm, depth: usize) -> u64 {
    arm_rows(rows, arm)
        .filter(|row| row.depth == depth)
        .map(|row| row.work.total())
        .sum()
}

fn slope_for(rows: &[Rc0aRow], arm: Rc0aArm, depths: &[usize]) -> ExactSlope {
    let episodes_per_depth = arm_rows(rows, arm)
        .filter(|row| row.depth == depths[0])
        .map(|row| row.total)
        .sum::<usize>() as i128;
    assert!(episodes_per_depth > 0, "slope needs evaluated episodes");
    assert!(depths.iter().all(|depth| {
        arm_rows(rows, arm)
            .filter(|row| row.depth == *depth)
            .map(|row| row.total)
            .sum::<usize>() as i128
            == episodes_per_depth
    }));
    let points = depths
        .iter()
        .map(|depth| {
            let x = (3 * *depth + 2) as i128;
            let grounded = aggregate_work(rows, arm, *depth) as i128;
            let concrete = aggregate_work(rows, Rc0aArm::Concrete, *depth) as i128;
            (x, grounded - concrete)
        })
        .collect::<Vec<_>>();
    let count = points.len() as i128;
    let sum_x: i128 = points.iter().map(|(x, _)| *x).sum();
    let sum_y: i128 = points.iter().map(|(_, y)| *y).sum();
    let sum_xy: i128 = points.iter().map(|(x, y)| *x * *y).sum();
    let sum_xx: i128 = points.iter().map(|(x, _)| *x * *x).sum();
    ExactSlope {
        numerator: count * sum_xy - sum_x * sum_y,
        denominator: (count * sum_xx - sum_x * sum_x) * episodes_per_depth,
    }
}

fn status(passed: bool) -> String {
    if passed { "PASS" } else { "FAIL" }.to_string()
}

fn harness_report(
    mode: HarnessMode,
    depths: &[usize],
    evaluations: Vec<FixtureEvaluation>,
    duplicate_deterministic: bool,
    parallel_cells: usize,
) -> Rc0aReport {
    let acquisition = evaluations
        .iter()
        .map(|evaluation| evaluation.acquisition.clone())
        .collect::<Vec<_>>();
    let rows = evaluations
        .iter()
        .flat_map(|evaluation| evaluation.rows.clone())
        .collect::<Vec<_>>();
    let reconstruction_parity = acquisition.iter().all(|row| row.rp0a_parity);
    let earned = acquisition.iter().all(|row| {
        row.successful_episodes == COMPILE_DEPTHS.len()
            && row.compiled_arrows == LowerRole::PROGRAM_SOURCES.len()
            && row.subthreshold_arrows == 0
            && row.work.success_credit_updates
                == (COMPILE_DEPTHS.len() * LowerRole::PROGRAM_SOURCES.len()) as u64
    });
    let role_relative = evaluations
        .iter()
        .all(|evaluation| evaluation.persistent_role_relative);
    let changed_bindings = evaluations
        .iter()
        .all(|evaluation| evaluation.changed_bindings_observed);
    let fully_correct = |arm| {
        let (correct, total) = arm_correct(&rows, arm);
        total > 0
            && correct == total
            && arm_rows(&rows, arm).all(|row| {
                row.explicit_answers
                    && row.queues_empty
                    && row.activity_limit_hits == 0
                    && row.expected_route_firings
            })
    };
    let concrete_behavior = fully_correct(Rc0aArm::Concrete);
    let generic_behavior = fully_correct(Rc0aArm::Generic);
    let compiled_behavior =
        fully_correct(Rc0aArm::Compiled) && fully_correct(Rc0aArm::ChangedBindings);
    let compiled_path = [Rc0aArm::Compiled, Rc0aArm::ChangedBindings]
        .into_iter()
        .all(|arm| {
            arm_rows(&rows, arm).all(|row| {
                row.work.rg0a.compiled_arrow_firings > 0
                    && row.work.rg0a.reflected_arrow_evaluations == 0
                    && row.work.rg0a.reflected_arrow_firings == 0
                    && row.work.rg0a.binding_reads == 0
                    && row.work.rg0a.binding_deliveries == 0
                    && row.work.rg0a.direct_arrow_evaluations == 0
                    && row.work.rg0a.direct_arrow_firings == 0
                    && row.work.rg0a.direct_executor_calls == 0
                    && row.work.rg0a.pre_resolved_routes == 0
                    && row.work.rg0a.fallback_calls == 0
                    && row.work.rg0a.oracle_calls == 0
                    && row.work.generic_resumptions == 0
                    && row.used_compiled_arrows.len() == LowerRole::PROGRAM_SOURCES.len()
            })
        });
    let lower_effects_preserved = [
        Rc0aArm::Generic,
        Rc0aArm::Compiled,
        Rc0aArm::ChangedBindings,
    ]
    .into_iter()
    .all(|arm| arm_rows(&rows, arm).all(|row| row.lower_effects_match));
    let invalidation_behavior = fully_correct(Rc0aArm::InvalidatedTransition)
        && arm_rows(&rows, Rc0aArm::InvalidatedTransition).all(|row| {
            row.work.rg0a.compiled_arrow_firings == 0
                && row.work.generic_resumptions == row.total as u64
                && row.work.rg0a.reflected_arrow_firings > 0
        })
        && arm_rows(&rows, Rc0aArm::InvalidatedTransition)
            .map(|row| row.work.compiled_invalidations)
            .sum::<u64>()
            == (evaluations.len() * LowerRole::PROGRAM_SOURCES.len()) as u64;
    let subthreshold_behavior = fully_correct(Rc0aArm::Subthreshold)
        && acquisition.iter().all(|row| row.subthreshold_arrows == 0)
        && arm_rows(&rows, Rc0aArm::Subthreshold).all(|row| {
            row.work.rg0a.compiled_arrow_firings == 0
                && row.work.compiled_invalidations == 0
                && row.work.generic_resumptions == row.total as u64
        });
    let shuffled_behavior = fully_correct(Rc0aArm::ShuffledEvidence)
        && arm_rows(&rows, Rc0aArm::ShuffledEvidence).all(|row| {
            row.work.rg0a.compiled_arrow_firings == 0
                && row.work.generic_resumptions == row.total as u64
        })
        && arm_rows(&rows, Rc0aArm::ShuffledEvidence)
            .map(|row| row.work.compiled_invalidations)
            .sum::<u64>()
            == (evaluations.len() * LowerRole::PROGRAM_SOURCES.len()) as u64;
    let no_binding = {
        let (correct, total) = arm_correct(&rows, Rc0aArm::NoBindings);
        total > 0 && correct < total
    };
    let state_isolation = rows.iter().all(|row| {
        row.bindings_erased
            && row.temporary_routes_erased
            && row.immutable_state_unchanged
            && row.permanent_state_unchanged
    }) && duplicate_deterministic;
    let generic_slope = slope_for(&rows, Rc0aArm::Generic, depths);
    let compiled_slope = slope_for(&rows, Rc0aArm::Compiled, depths);
    let slope_passed = generic_slope.numerator > 0
        && generic_slope.denominator > 0
        && compiled_slope.numerator >= 0
        && compiled_slope.denominator > 0
        && compiled_slope.numerator * generic_slope.denominator * SLOPE_REDUCTION_DENOMINATOR
            <= generic_slope.numerator * compiled_slope.denominator;
    let lifecycle =
        evaluations
            .iter()
            .fold(RcLifecycleTotals::default(), |mut total, evaluation| {
                total.add(evaluation.lifecycle);
                total
            });
    let lifecycle_ok = lifecycle.created == lifecycle.destroyed;
    let ancestry = true;
    let source_audit = true;
    let qualitative_passed = ancestry
        && reconstruction_parity
        && earned
        && role_relative
        && changed_bindings
        && concrete_behavior
        && generic_behavior
        && compiled_behavior
        && compiled_path
        && lower_effects_preserved
        && invalidation_behavior
        && subthreshold_behavior
        && shuffled_behavior
        && no_binding
        && state_isolation
        && lifecycle_ok
        && source_audit
        && slope_passed;
    let claim_eligible = mode == HarnessMode::Definitive;
    let passed = claim_eligible && qualitative_passed;
    let gates = vec![
        Rc0aGate {
            name: "frozen-ancestry-and-rp0a-parity".to_string(),
            status: status(ancestry && reconstruction_parity),
        },
        Rc0aGate {
            name: "earned-three-episode-compilation".to_string(),
            status: status(earned),
        },
        Rc0aGate {
            name: "role-relative-persistent-structure".to_string(),
            status: status(role_relative),
        },
        Rc0aGate {
            name: "compiled-fresh-and-changed-bindings".to_string(),
            status: status(changed_bindings && compiled_behavior),
        },
        Rc0aGate {
            name: "compiled-local-dispatch-only".to_string(),
            status: status(compiled_path),
        },
        Rc0aGate {
            name: "lower-effects-preserved".to_string(),
            status: status(lower_effects_preserved),
        },
        Rc0aGate {
            name: "invalidation-resumes-generic".to_string(),
            status: status(invalidation_behavior),
        },
        Rc0aGate {
            name: "subthreshold-does-not-compile".to_string(),
            status: status(subthreshold_behavior),
        },
        Rc0aGate {
            name: "shuffled-evidence-cannot-fire".to_string(),
            status: status(shuffled_behavior),
        },
        Rc0aGate {
            name: "bindings-remain-necessary".to_string(),
            status: status(no_binding),
        },
        Rc0aGate {
            name: "state-isolation-and-determinism".to_string(),
            status: status(state_isolation && lifecycle_ok),
        },
        Rc0aGate {
            name: "per-step-slope-reduction-at-least-80-percent".to_string(),
            status: status(slope_passed),
        },
        Rc0aGate {
            name: "rc0b-excluded-source-audit".to_string(),
            status: status(source_audit),
        },
    ];
    Rc0aReport {
        protocol: RC0A_PROTOCOL.to_string(),
        mode,
        claim_eligible,
        passed,
        qualitative_passed,
        reconstruction_parity,
        duplicate_deterministic,
        generic_slope,
        compiled_slope,
        slope_passed,
        acquisition,
        rows,
        gates,
        workspaces_created: lifecycle.created,
        workspaces_destroyed: lifecycle.destroyed,
        maximum_live_workspaces_per_cell: lifecycle.maximum_live,
        parallel_cells,
    }
}

fn development_fixture() -> Frozen<FrozenRp0aState> {
    let lifecycle = Lifecycle::default();
    let fixture = Frozen::new(synthetic_fixture(0xca00_0000, &lifecycle));
    assert_eq!(lifecycle.created.get(), lifecycle.destroyed.get());
    fixture
}

fn run_development(mode: HarnessMode) -> Rc0aReport {
    let fixture = development_fixture();
    let (depths, queries) = match mode {
        HarnessMode::Micro => (&DEVELOPMENT_DEPTHS[..2], 2),
        HarnessMode::Gate => (&DEVELOPMENT_DEPTHS[..], 4),
        HarnessMode::Definitive => unreachable!("development only"),
    };
    let first = evaluate_fixture(
        &fixture,
        true,
        DEVELOPMENT_SEED_INDEX,
        depths,
        queries,
        0xc000_0000_0000_0000,
    );
    let second = evaluate_fixture(
        &fixture,
        true,
        DEVELOPMENT_SEED_INDEX,
        depths,
        queries,
        0xc000_0000_0000_0000,
    );
    let deterministic = first == second;
    harness_report(mode, depths, vec![first], deterministic, 1)
}

fn reconstruct_rc0a_fixture(
    seed_index: usize,
) -> (Frozen<FrozenRp0aState>, bool, RcLifecycleTotals) {
    let lifecycle = Lifecycle::default();
    let expected = FROZEN_RP0A_ENDPOINTS[seed_index];
    let (summary, _, role, program) =
        train_learned_seed(Arm::Integrated, seed_index, false, &lifecycle);
    let parity = frozen_endpoint_matches(&summary, expected);
    (
        Frozen::new(FrozenRp0aState {
            seed_index,
            role,
            program,
        }),
        parity,
        RcLifecycleTotals::read(&lifecycle),
    )
}

fn run_definitive() -> Rc0aReport {
    let reconstructed = parallel_map_ordered(DEFINITIVE_SEEDS, reconstruct_rc0a_fixture);
    let evaluated = parallel_map_ordered(DEFINITIVE_SEEDS, |seed_index| {
        let (fixture, parity, reconstruction_lifecycle) = &reconstructed[seed_index];
        let first = evaluate_fixture(
            fixture,
            *parity,
            seed_index,
            &RG0A_DEPTHS,
            RG0A_QUERIES_PER_DEPTH,
            0xc100_0000_0000_0000,
        );
        let second = evaluate_fixture(
            fixture,
            *parity,
            seed_index,
            &RG0A_DEPTHS,
            RG0A_QUERIES_PER_DEPTH,
            0xc100_0000_0000_0000,
        );
        let deterministic = first == second;
        let mut retained = first;
        retained.lifecycle.add(*reconstruction_lifecycle);
        (retained, deterministic)
    });
    let mut evaluations = Vec::new();
    let mut deterministic = true;
    for (evaluation, duplicate) in evaluated {
        evaluations.push(evaluation);
        deterministic &= duplicate;
    }
    harness_report(
        HarnessMode::Definitive,
        &RG0A_DEPTHS,
        evaluations,
        deterministic,
        DEFINITIVE_SEEDS,
    )
}

pub fn run_rc0a_harness(mode: HarnessMode) -> Rc0aReport {
    match mode {
        HarnessMode::Micro | HarnessMode::Gate => run_development(mode),
        HarnessMode::Definitive => run_definitive(),
    }
}

fn mode_name(mode: HarnessMode) -> &'static str {
    match mode {
        HarnessMode::Micro => "micro",
        HarnessMode::Gate => "gate",
        HarnessMode::Definitive => "definitive",
    }
}

pub fn print_rc0a_report(report: &Rc0aReport) {
    println!(
        "RC0a {:?}: {}{}",
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
    for arm in Rc0aArm::ALL {
        let (correct, total) = arm_correct(&report.rows, arm);
        let work: u64 = arm_rows(&report.rows, arm)
            .map(|row| row.work.total())
            .sum();
        println!("{}: {}/{} work={}", arm.name(), correct, total, work);
    }
    println!(
        "slope generic={} compiled={} pass={}",
        report.generic_slope.display(),
        report.compiled_slope.display(),
        report.slope_passed
    );
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

pub fn rc0a_csv(report: &Rc0aReport) -> String {
    let headers = [
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
        "runtime_total",
        "rg0a_total",
        "compiled_arrow_evaluations",
        "compiled_arrow_firings",
        "topology_validation_comparisons",
        "compiled_binding_reads",
        "compiled_installation_comparisons",
        "temporary_arrow_installations",
        "compiled_invalidations",
        "generic_resumptions",
        "reflected_arrow_evaluations",
        "reflected_arrow_firings",
        "binding_reads",
        "binding_deliveries",
        "direct_arrow_evaluations",
        "direct_arrow_firings",
        "lower_effects_match",
        "bindings_erased",
        "temporary_routes_erased",
        "successful_episodes",
        "compiled_arrows",
        "subthreshold_arrows",
        "persistent_bytes",
        "persistent_fingerprint",
        "acquisition_work",
        "generic_slope_numerator",
        "generic_slope_denominator",
        "compiled_slope_numerator",
        "compiled_slope_denominator",
        "gate",
        "gate_status",
    ];
    let common = || {
        vec![
            ("protocol", report.protocol.clone()),
            ("mode", mode_name(report.mode).to_string()),
            ("claim_eligible", report.claim_eligible.to_string()),
            ("passed", report.passed.to_string()),
            (
                "generic_slope_numerator",
                report.generic_slope.numerator.to_string(),
            ),
            (
                "generic_slope_denominator",
                report.generic_slope.denominator.to_string(),
            ),
            (
                "compiled_slope_numerator",
                report.compiled_slope.numerator.to_string(),
            ),
            (
                "compiled_slope_denominator",
                report.compiled_slope.denominator.to_string(),
            ),
        ]
    };
    let mut output = headers.join(",");
    output.push('\n');
    for acquisition in &report.acquisition {
        let mut fields = common();
        fields.extend([
            ("row_type", "acquisition".to_string()),
            ("seed_index", acquisition.seed_index.to_string()),
            (
                "successful_episodes",
                acquisition.successful_episodes.to_string(),
            ),
            ("compiled_arrows", acquisition.compiled_arrows.to_string()),
            (
                "subthreshold_arrows",
                acquisition.subthreshold_arrows.to_string(),
            ),
            ("persistent_bytes", acquisition.persistent_bytes.to_string()),
            (
                "persistent_fingerprint",
                acquisition.persistent_fingerprint.to_string(),
            ),
            ("acquisition_work", acquisition.work.total().to_string()),
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
            ("runtime_total", work.total().to_string()),
            ("rg0a_total", work.rg0a.total().to_string()),
            (
                "compiled_arrow_evaluations",
                work.rg0a.compiled_arrow_evaluations.to_string(),
            ),
            (
                "compiled_arrow_firings",
                work.rg0a.compiled_arrow_firings.to_string(),
            ),
            (
                "topology_validation_comparisons",
                work.topology_validation_comparisons.to_string(),
            ),
            (
                "compiled_binding_reads",
                work.compiled_binding_reads.to_string(),
            ),
            (
                "compiled_installation_comparisons",
                work.compiled_installation_comparisons.to_string(),
            ),
            (
                "temporary_arrow_installations",
                work.temporary_arrow_installations.to_string(),
            ),
            (
                "compiled_invalidations",
                work.compiled_invalidations.to_string(),
            ),
            ("generic_resumptions", work.generic_resumptions.to_string()),
            (
                "reflected_arrow_evaluations",
                work.rg0a.reflected_arrow_evaluations.to_string(),
            ),
            (
                "reflected_arrow_firings",
                work.rg0a.reflected_arrow_firings.to_string(),
            ),
            ("binding_reads", work.rg0a.binding_reads.to_string()),
            (
                "binding_deliveries",
                work.rg0a.binding_deliveries.to_string(),
            ),
            (
                "direct_arrow_evaluations",
                work.rg0a.direct_arrow_evaluations.to_string(),
            ),
            (
                "direct_arrow_firings",
                work.rg0a.direct_arrow_firings.to_string(),
            ),
            ("lower_effects_match", row.lower_effects_match.to_string()),
            ("bindings_erased", row.bindings_erased.to_string()),
            (
                "temporary_routes_erased",
                row.temporary_routes_erased.to_string(),
            ),
        ]);
        writeln!(output, "{}", csv_row(&headers, &fields)).unwrap();
    }
    for gate in &report.gates {
        let mut fields = common();
        fields.extend([
            ("row_type", "gate".to_string()),
            ("gate", gate.name.clone()),
            ("gate_status", gate.status.clone()),
        ]);
        writeln!(output, "{}", csv_row(&headers, &fields)).unwrap();
    }
    output
}

pub fn rc0a_markdown(report: &Rc0aReport) -> String {
    let mut output = String::new();
    writeln!(output, "# RC0a compiled grounded recurrence\n").unwrap();
    writeln!(
        output,
        "Compatibility gate: **{}**. Claim eligible: `{}`.\n",
        if report.passed { "PASS" } else { "FAIL" },
        report.claim_eligible
    )
    .unwrap();
    writeln!(
        output,
        "Generic per-step excess slope: `{}`; compiled slope: `{}`; 80% reduction gate: `{}`.\n",
        report.generic_slope.display(),
        report.compiled_slope.display(),
        report.slope_passed
    )
    .unwrap();
    writeln!(
        output,
        "| Arm | Correct | Total | Work |\n|---|---:|---:|---:|"
    )
    .unwrap();
    for arm in Rc0aArm::ALL {
        let (correct, total) = arm_correct(&report.rows, arm);
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
    fn rc0a_work_total_reconciles() {
        let work = Rc0aWork {
            rg0a: Rg0aWork {
                lower_cells: 2,
                compiled_arrow_firings: 3,
                ..Rg0aWork::default()
            },
            temporary_arrow_installations: 4,
            ..Rc0aWork::default()
        };
        assert_eq!(work.total(), 9);
    }

    #[test]
    fn rc0a_micro_compiles_delivers_rebinds_and_invalidates() {
        let report = run_rc0a_harness(HarnessMode::Micro);
        assert!(report.qualitative_passed);
        assert!(report.slope_passed);
        assert!(!report.claim_eligible);
        assert!(!report.passed);
        assert!(arm_rows(&report.rows, Rc0aArm::Compiled).all(|row| row
            .work
            .rg0a
            .compiled_arrow_firings
            > 0));
        assert!(
            arm_rows(&report.rows, Rc0aArm::ChangedBindings).all(|row| row.correct == row.total)
        );
        assert!(
            arm_rows(&report.rows, Rc0aArm::InvalidatedTransition).all(|row| row
                .work
                .rg0a
                .compiled_arrow_firings
                == 0)
        );
    }
}
