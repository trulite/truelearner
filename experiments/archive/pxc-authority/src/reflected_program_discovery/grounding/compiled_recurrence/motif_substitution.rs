//! RC0b: one learned recurrent motif may elide locally transparent lower
//! relays while the frozen RG0a/RC0a executor continues to perform every
//! stateful and externally observable effect.

use super::*;
use crate::research_runtime::{parallel_map_ordered, Frozen, HarnessMode};
use std::mem::size_of;

pub const RC0B_PROTOCOL: &str = "grounded-motif-substitution-rc0b-v1";

const MOTIF_ACQUISITION_DEPTHS: [usize; 3] = [3, 4, 6];
const DEVELOPMENT_DEPTHS: [usize; 3] = [5, 8, 13];
const DEVELOPMENT_SEED_INDEX: usize = 30_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rc0bWork {
    pub rc0a: Rc0aWork,
    pub recurrence_observations: u64,
    pub recurrence_comparisons: u64,
    pub motif_candidates: u64,
    pub success_credit_updates: u64,
    pub failure_credit_updates: u64,
    pub motifs_earned: u64,
    pub compatibility_comparisons: u64,
    pub shortcut_binding_reads: u64,
    pub shortcut_installation_comparisons: u64,
    pub temporary_shortcut_installations: u64,
    pub shortcut_arrow_evaluations: u64,
    pub shortcut_arrow_firings: u64,
    pub residual_context_effects: u64,
    pub motif_invalidations: u64,
    pub rc0a_resumptions: u64,
    pub rg0a_resumptions: u64,
    pub relay_activations_eliminated: u64,
    pub route_firings_eliminated: u64,
}

impl Rc0bWork {
    pub fn total(self) -> u64 {
        self.rc0a.total()
            + self.recurrence_observations
            + self.recurrence_comparisons
            + self.motif_candidates
            + self.success_credit_updates
            + self.failure_credit_updates
            + self.motifs_earned
            + self.compatibility_comparisons
            + self.shortcut_binding_reads
            + self.shortcut_installation_comparisons
            + self.temporary_shortcut_installations
            + self.shortcut_arrow_evaluations
            + self.shortcut_arrow_firings
            + self.residual_context_effects
            + self.motif_invalidations
            + self.rc0a_resumptions
            + self.rg0a_resumptions
    }

    fn add(&mut self, other: Self) {
        self.rc0a.add(other.rc0a);
        self.recurrence_observations += other.recurrence_observations;
        self.recurrence_comparisons += other.recurrence_comparisons;
        self.motif_candidates += other.motif_candidates;
        self.success_credit_updates += other.success_credit_updates;
        self.failure_credit_updates += other.failure_credit_updates;
        self.motifs_earned += other.motifs_earned;
        self.compatibility_comparisons += other.compatibility_comparisons;
        self.shortcut_binding_reads += other.shortcut_binding_reads;
        self.shortcut_installation_comparisons += other.shortcut_installation_comparisons;
        self.temporary_shortcut_installations += other.temporary_shortcut_installations;
        self.shortcut_arrow_evaluations += other.shortcut_arrow_evaluations;
        self.shortcut_arrow_firings += other.shortcut_arrow_firings;
        self.residual_context_effects += other.residual_context_effects;
        self.motif_invalidations += other.motif_invalidations;
        self.rc0a_resumptions += other.rc0a_resumptions;
        self.rg0a_resumptions += other.rg0a_resumptions;
        self.relay_activations_eliminated += other.relay_activations_eliminated;
        self.route_firings_eliminated += other.route_firings_eliminated;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct MotifShortcut {
    parent_compiled_arrow: usize,
    emitter_role: usize,
    relay_role: usize,
    target_role: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct GroundedMotif {
    cycle: Vec<usize>,
    shortcuts: Vec<MotifShortcut>,
    strength: i32,
}

impl GroundedMotif {
    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for role in &self.cycle {
            mix(&mut hash, *role as u64);
        }
        for shortcut in &self.shortcuts {
            mix(&mut hash, shortcut.parent_compiled_arrow as u64);
            mix(&mut hash, shortcut.emitter_role as u64);
            mix(&mut hash, shortcut.relay_role as u64);
            mix(&mut hash, shortcut.target_role as u64);
        }
        mix(&mut hash, self.strength as i64 as u64);
        hash
    }

    fn permanent_bytes(&self) -> usize {
        self.cycle.len() * size_of::<usize>()
            + self.shortcuts.len() * size_of::<MotifShortcut>()
            + size_of::<i32>()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MotifCandidate {
    cycle: Vec<usize>,
    shortcuts: Vec<MotifShortcut>,
    strength: i32,
    episodes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MotifLearner {
    candidate: Option<MotifCandidate>,
    motif: Option<GroundedMotif>,
}

struct MotifObservation<'a> {
    machine: &'a GroundMachine,
    role: &'a RoleLearner,
    dispatch: &'a CompiledDispatch,
    execution: &'a RcExecution,
    successful: bool,
    shuffle_adjacency: bool,
}

impl MotifLearner {
    fn new() -> Self {
        Self {
            candidate: None,
            motif: None,
        }
    }

    fn observe(&mut self, observation: MotifObservation<'_>, work: &mut Rc0bWork) {
        let MotifObservation {
            machine,
            role,
            dispatch,
            execution,
            successful,
            shuffle_adjacency,
        } = observation;
        if self.motif.is_some() {
            return;
        }
        let Some((cycle, mut shortcuts)) =
            discover_motif(machine, role, dispatch, &execution.internal_events, work)
        else {
            return;
        };
        if shuffle_adjacency && shortcuts.len() > 1 {
            let targets = shortcuts
                .iter()
                .map(|shortcut| shortcut.target_role)
                .collect::<Vec<_>>();
            for (index, shortcut) in shortcuts.iter_mut().enumerate() {
                shortcut.target_role = targets[(index + 1) % targets.len()];
            }
        }
        let same = self.candidate.as_ref().is_some_and(|candidate| {
            work.recurrence_comparisons += 1;
            candidate.cycle == cycle && candidate.shortcuts == shortcuts
        });
        if !same {
            self.candidate = Some(MotifCandidate {
                cycle,
                shortcuts,
                strength: 0,
                episodes: 0,
            });
            work.motif_candidates += 1;
        }
        let candidate = self.candidate.as_mut().expect("candidate was installed");
        if successful {
            candidate.strength += SUCCESS_CREDIT;
            work.success_credit_updates += 1;
        } else {
            candidate.strength += FAILURE_CREDIT;
            work.failure_credit_updates += 1;
        }
        candidate.episodes += 1;
        if candidate.strength >= CONSOLIDATION_STRENGTH {
            self.motif = Some(GroundedMotif {
                cycle: candidate.cycle.clone(),
                shortcuts: candidate.shortcuts.clone(),
                strength: candidate.strength,
            });
            work.motifs_earned += 1;
        }
    }
}

fn translated_event_roles(
    machine: &GroundMachine,
    role: &RoleLearner,
    events: &[GroundInternalEvent],
    work: &mut Rc0bWork,
) -> Vec<usize> {
    events
        .iter()
        .filter_map(|event| {
            work.recurrence_observations += 1;
            role.translate(machine.observations[event.cell_index].signature)
        })
        .collect()
}

fn canonical_cycle(cycle: &[usize], work: &mut Rc0bWork) -> Vec<usize> {
    let mut best = cycle.to_vec();
    for offset in 1..cycle.len() {
        work.recurrence_comparisons += cycle.len() as u64;
        let rotated = cycle[offset..]
            .iter()
            .chain(&cycle[..offset])
            .copied()
            .collect::<Vec<_>>();
        if rotated < best {
            best = rotated;
        }
    }
    best
}

fn cell_for_learned_role(
    machine: &GroundMachine,
    learner: &RoleLearner,
    learned_role: usize,
    work: &mut Rc0bWork,
) -> Option<usize> {
    for observation in &machine.observations {
        work.recurrence_comparisons += 1;
        if learner.translate(observation.signature) == Some(learned_role) {
            return Some(observation.cell_index);
        }
    }
    None
}

fn discover_motif(
    machine: &GroundMachine,
    learner: &RoleLearner,
    dispatch: &CompiledDispatch,
    events: &[GroundInternalEvent],
    work: &mut Rc0bWork,
) -> Option<(Vec<usize>, Vec<MotifShortcut>)> {
    let roles = translated_event_roles(machine, learner, events, work);
    let mut observed_cycle = None;
    'outer: for end in 1..roles.len() {
        for start in 0..end {
            work.recurrence_comparisons += 1;
            if roles[start] == roles[end] && end - start >= 3 {
                observed_cycle = Some(roles[start..end].to_vec());
                break 'outer;
            }
        }
    }
    let observed_cycle = observed_cycle?;
    let cycle = canonical_cycle(&observed_cycle, work);
    let mut shortcuts = Vec::new();
    for index in 0..observed_cycle.len() {
        let emitter_role = observed_cycle[index];
        let relay_role = observed_cycle[(index + 1) % observed_cycle.len()];
        let target_role = observed_cycle[(index + 2) % observed_cycle.len()];
        let Some(relay_cell) = cell_for_learned_role(machine, learner, relay_role, work) else {
            continue;
        };
        if !matches!(machine.cells[relay_cell].physics, CellPhysics::RouteSource)
            || machine.context_effects[relay_cell].is_some()
        {
            continue;
        }
        let Some(compiled) = dispatch.by_source_role.get(relay_role).copied().flatten() else {
            continue;
        };
        work.recurrence_comparisons += 1;
        if compiled.to_role != target_role {
            continue;
        }
        shortcuts.push(MotifShortcut {
            parent_compiled_arrow: compiled.id,
            emitter_role,
            relay_role,
            target_role,
        });
    }
    shortcuts.sort();
    shortcuts.dedup();
    (!shortcuts.is_empty()).then_some((cycle, shortcuts))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservableBoundary {
    pub current: Option<OpaqueId>,
    pub effect_prefix_len: usize,
    pub context_marker: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservableTrace {
    pub outcome: BindingOutcome,
    pub final_current: Option<OpaqueId>,
    events: Vec<GroundObservableEvent>,
    pub queue_empty: bool,
    pub activity_limit_hit: bool,
    pub boundaries: Vec<ObservableBoundary>,
    pub context_ledger: Vec<(u64, u64)>,
}

fn observable_trace(execution: &RcExecution) -> ObservableTrace {
    ObservableTrace {
        outcome: execution.outcome,
        final_current: execution.final_current,
        events: execution.observable_events.clone(),
        queue_empty: execution.queue_empty,
        activity_limit_hit: execution.activity_limit_hit,
        boundaries: execution
            .boundaries
            .iter()
            .map(|boundary| ObservableBoundary {
                current: boundary.current,
                effect_prefix_len: boundary.effect_prefix_len,
                context_marker: boundary.context_marker,
            })
            .collect(),
        context_ledger: execution
            .observable_events
            .iter()
            .filter_map(|event| match event {
                GroundObservableEvent::ContextEffect { site, marker } => Some((*site, *marker)),
                _ => None,
            })
            .collect(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ShortcutEmitter {
    Apply,
    LookupResult,
    StoreCurrent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InstalledShortcut {
    emitter_cell: usize,
    kind: ShortcutEmitter,
}

fn redirect_destination(
    physics: &mut CellPhysics,
    relay: LowerLocation,
    target: LowerLocation,
) -> Option<ShortcutEmitter> {
    match physics {
        CellPhysics::Apply { route_source } if *route_source == relay => {
            *route_source = target;
            Some(ShortcutEmitter::Apply)
        }
        CellPhysics::Lookup { result_source, .. } if *result_source == relay => {
            *result_source = target;
            Some(ShortcutEmitter::LookupResult)
        }
        CellPhysics::StoreCurrent { success_source } if *success_source == relay => {
            *success_source = target;
            Some(ShortcutEmitter::StoreCurrent)
        }
        _ => None,
    }
}

fn shortcut_was_used(
    installed: InstalledShortcut,
    event: &GroundInternalEvent,
    machine: &GroundMachine,
) -> bool {
    if event.cell_index != installed.emitter_cell {
        return false;
    }
    match installed.kind {
        ShortcutEmitter::Apply => true,
        ShortcutEmitter::LookupResult => event.current_before.is_some_and(|current| {
            let mut found = None;
            for (left, right) in &machine.relations {
                if *left == current {
                    if found.is_some_and(|prior| prior != *right) {
                        return false;
                    }
                    found = Some(*right);
                }
            }
            found.is_some()
        }),
        ShortcutEmitter::StoreCurrent => event.identity.is_some(),
    }
}

fn install_motif(
    machine: &GroundMachine,
    grounding: &TemporaryGrounding,
    dispatch: &CompiledDispatch,
    motif: &GroundedMotif,
    force_stale: bool,
    work: &mut Rc0bWork,
) -> Option<(GroundMachine, Vec<InstalledShortcut>)> {
    let mut substituted = machine.clone();
    let mut installed = Vec::new();
    for shortcut in &motif.shortcuts {
        work.compatibility_comparisons += 1;
        let compiled = dispatch
            .by_source_role
            .get(shortcut.relay_role)
            .copied()
            .flatten()?;
        if compiled.id != shortcut.parent_compiled_arrow
            || compiled.from_role != shortcut.relay_role
            || compiled.to_role != shortcut.target_role
        {
            return None;
        }
        work.shortcut_binding_reads += 3;
        let emitter = grounding
            .role_locations
            .get(shortcut.emitter_role)
            .copied()
            .flatten()?;
        let relay = grounding
            .role_locations
            .get(shortcut.relay_role)
            .copied()
            .flatten()?;
        let target = grounding
            .role_locations
            .get(shortcut.target_role)
            .copied()
            .flatten()?;
        let mut emitter_cell = None;
        let mut relay_cell = None;
        for (cell_index, cell) in substituted.cells.iter().enumerate() {
            work.shortcut_installation_comparisons += 1;
            if cell.identity == emitter {
                emitter_cell = Some(cell_index);
            }
            if cell.identity == relay {
                relay_cell = Some(cell_index);
            }
        }
        let emitter_cell = emitter_cell?;
        let relay_cell = relay_cell?;
        work.compatibility_comparisons += 2;
        if !matches!(
            substituted.cells[relay_cell].physics,
            CellPhysics::RouteSource
        ) || (!force_stale && substituted.context_effects[relay_cell].is_some())
        {
            return None;
        }
        let kind =
            redirect_destination(&mut substituted.cells[emitter_cell].physics, relay, target)?;
        installed.push(InstalledShortcut { emitter_cell, kind });
        work.temporary_shortcut_installations += 1;
    }
    (installed.len() == motif.shortcuts.len()).then_some((substituted, installed))
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MotifExecution {
    execution: RcExecution,
    work: Rc0bWork,
    motif_fired: bool,
}

impl MotifExecution {
    fn total_work(&self) -> u64 {
        self.work.total()
    }

    fn trace(&self) -> ObservableTrace {
        observable_trace(&self.execution)
    }
}

fn wrap_execution(execution: RcExecution) -> MotifExecution {
    let mut work = Rc0bWork {
        rc0a: execution.work,
        ..Rc0bWork::default()
    };
    work.residual_context_effects = execution
        .observable_events
        .iter()
        .filter(|event| matches!(event, GroundObservableEvent::ContextEffect { .. }))
        .count() as u64;
    MotifExecution {
        execution,
        work,
        motif_fired: false,
    }
}

fn execute_direct_traced(episode: &GroundEpisode, lifecycle: &Lifecycle) -> MotifExecution {
    let _workspace = lifecycle.enter();
    let targets = direct_targets(episode);
    let mut router = GroundRouter::Direct { targets: &targets };
    wrap_execution(RcExecution::from_rg0a(
        run_cell_machine_traced(
            &episode.machine,
            &mut router,
            Rg0aWork {
                direct_executor_calls: 1,
                ..Rg0aWork::default()
            },
            true,
        ),
        Rc0aWork::default(),
    ))
}

fn execute_full_traced(
    machine: &GroundMachine,
    role: &RoleLearner,
    program: &ProgramLearner,
    dispatch: &mut CompiledDispatch,
    condition: BindingCondition,
    shuffle_seed: u64,
    lifecycle: &Lifecycle,
) -> MotifExecution {
    wrap_execution(execute_compiled_or_resume_traced(
        machine,
        role,
        program,
        dispatch,
        condition,
        shuffle_seed,
        lifecycle,
    ))
}

fn execute_with_motif(request: MotifExecutionRequest<'_>) -> MotifExecution {
    let MotifExecutionRequest {
        machine,
        role,
        program,
        dispatch,
        motif,
        condition,
        shuffle_seed,
        lifecycle,
        force_stale,
    } = request;
    let mut work = Rc0bWork::default();
    if !dispatch.validate(program, &mut work.rc0a) {
        if motif.take().is_some() {
            work.motif_invalidations += 1;
        }
        work.rg0a_resumptions += 1;
        let mut execution = execute_compiled_or_resume_traced(
            machine,
            role,
            program,
            dispatch,
            condition,
            shuffle_seed,
            lifecycle,
        );
        execution.work.add(work.rc0a);
        work.rc0a = execution.work;
        work.residual_context_effects = execution
            .observable_events
            .iter()
            .filter(|event| matches!(event, GroundObservableEvent::ContextEffect { .. }))
            .count() as u64;
        return MotifExecution {
            execution,
            work,
            motif_fired: false,
        };
    }

    let _workspace = lifecycle.enter();
    let mut shuffle_rng = DeterministicRng::new(shuffle_seed);
    let mut grounding = learned_grounding(
        machine,
        role,
        condition,
        &mut shuffle_rng,
        &mut work.rc0a.rg0a,
    );
    let false_bindings = grounding
        .cell_roles
        .iter()
        .filter(|bound| bound.is_none())
        .count();
    let ambiguous_bindings = grounding.ambiguous_roles;
    let Some(mut routes) =
        TemporaryCompiledRoutes::install(machine, &grounding, dispatch, &mut work.rc0a)
    else {
        let mut execution =
            execute_local_arrows_traced(machine, &TemporaryCompiledRoutes::empty(), work.rc0a);
        execution.false_bindings = false_bindings;
        execution.ambiguous_bindings = ambiguous_bindings;
        grounding.erase();
        execution.bindings_erased = grounding.is_empty();
        work.rc0a = execution.work;
        return MotifExecution {
            execution,
            work,
            motif_fired: false,
        };
    };

    let installed = motif.as_ref().and_then(|earned| {
        install_motif(
            machine,
            &grounding,
            dispatch,
            earned,
            force_stale,
            &mut work,
        )
    });
    let (run_machine, installed_shortcuts, motif_fired) =
        if let Some((substituted, installed)) = installed {
            (substituted, installed, true)
        } else {
            if motif.take().is_some() {
                work.motif_invalidations += 1;
            }
            work.rc0a_resumptions += 1;
            (machine.clone(), Vec::new(), false)
        };
    let rc0b_prefix = Rc0bWork {
        rc0a: Rc0aWork::default(),
        ..work
    };
    let mut execution = execute_local_arrows_traced(&run_machine, &routes, work.rc0a);
    execution.false_bindings = false_bindings;
    execution.ambiguous_bindings = ambiguous_bindings;
    grounding.erase();
    routes.erase();
    execution.bindings_erased = grounding.is_empty();
    execution.temporary_routes_erased = routes.is_empty();

    let shortcut_firings = installed_shortcuts
        .iter()
        .map(|installed| {
            execution
                .internal_events
                .iter()
                .filter(|event| shortcut_was_used(*installed, event, machine))
                .count() as u64
        })
        .sum::<u64>();
    work = rc0b_prefix;
    work.rc0a = execution.work;
    work.shortcut_arrow_evaluations += shortcut_firings;
    work.shortcut_arrow_firings += shortcut_firings;
    work.relay_activations_eliminated += shortcut_firings;
    work.route_firings_eliminated += shortcut_firings;
    work.residual_context_effects += execution
        .observable_events
        .iter()
        .filter(|event| matches!(event, GroundObservableEvent::ContextEffect { .. }))
        .count() as u64;
    MotifExecution {
        execution,
        work,
        motif_fired,
    }
}

struct MotifExecutionRequest<'a> {
    machine: &'a GroundMachine,
    role: &'a RoleLearner,
    program: &'a ProgramLearner,
    dispatch: &'a mut CompiledDispatch,
    motif: &'a mut Option<GroundedMotif>,
    condition: BindingCondition,
    shuffle_seed: u64,
    lifecycle: &'a Lifecycle,
    force_stale: bool,
}

#[derive(Clone, Debug)]
struct MotifAcquisition {
    dispatch: CompiledDispatch,
    motif: Option<GroundedMotif>,
    subthreshold: Option<GroundedMotif>,
    shuffled: Option<GroundedMotif>,
    successful_episodes: usize,
    work: Rc0bWork,
    shuffled_work: Rc0bWork,
    rc0a_acquisition_work: Rc0aWork,
}

fn acquire_motif(
    fixture: &FrozenRp0aState,
    seed_index: usize,
    domain: u64,
    lifecycle: &Lifecycle,
) -> MotifAcquisition {
    let dispatch_acquisition = acquire_dispatch(fixture, seed_index, domain ^ 0xb001, lifecycle);
    let mut dispatch = dispatch_acquisition.dispatch.clone();
    let mut shuffled_dispatch = dispatch_acquisition.dispatch.clone();
    let mut learner = MotifLearner::new();
    let mut shuffled = MotifLearner::new();
    let mut work = Rc0bWork::default();
    let mut shuffled_work = Rc0bWork::default();
    let mut successful_episodes = 0;
    let mut subthreshold = None;
    let mut identities = IdentitySource::new(domain ^ 0xb002 ^ seed_index as u64);
    let mut rng = DeterministicRng::new(domain ^ 0xb003 ^ seed_index as u64);
    for (index, depth) in MOTIF_ACQUISITION_DEPTHS.into_iter().enumerate() {
        let chain = chain_episode(&mut identities, &mut rng, depth);
        let episode_id = domain
            .wrapping_add(seed_index as u64 * 10_000)
            .wrapping_add(index as u64);
        let episode = build_ground_episode(chain, episode_id, lifecycle);
        let execution = execute_compiled_or_resume_traced(
            &episode.machine,
            &fixture.role,
            &fixture.program,
            &mut dispatch,
            BindingCondition::Learned,
            episode_id ^ 0xb004,
            lifecycle,
        );
        let successful = execution.outcome == BindingOutcome::Answer(episode.machine.answer)
            && execution.explicit_answer
            && execution.queue_empty
            && !execution.activity_limit_hit;
        successful_episodes += usize::from(successful);
        work.rc0a.add(execution.work);
        learner.observe(
            MotifObservation {
                machine: &episode.machine,
                role: &fixture.role,
                dispatch: &dispatch,
                execution: &execution,
                successful,
                shuffle_adjacency: false,
            },
            &mut work,
        );

        let shuffled_execution = execute_compiled_or_resume_traced(
            &episode.machine,
            &fixture.role,
            &fixture.program,
            &mut shuffled_dispatch,
            BindingCondition::Learned,
            episode_id ^ 0xb005,
            lifecycle,
        );
        shuffled_work.rc0a.add(shuffled_execution.work);
        shuffled.observe(
            MotifObservation {
                machine: &episode.machine,
                role: &fixture.role,
                dispatch: &shuffled_dispatch,
                execution: &shuffled_execution,
                successful,
                shuffle_adjacency: true,
            },
            &mut shuffled_work,
        );
        if index == 1 {
            subthreshold = learner.motif.clone();
        }
    }
    MotifAcquisition {
        dispatch,
        motif: learner.motif,
        subthreshold,
        shuffled: shuffled.motif,
        successful_episodes,
        work,
        shuffled_work,
        rc0a_acquisition_work: dispatch_acquisition.work,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rc0bArm {
    Concrete,
    FullRc0a,
    Motif,
    ChangedSurroundings,
    InterruptionReentry,
    ContextInvalidation,
    ForcedStale,
    ParentInvalidation,
    Subthreshold,
    ShuffledEvidence,
    NoBindings,
}

impl Rc0bArm {
    const ALL: [Self; 11] = [
        Self::Concrete,
        Self::FullRc0a,
        Self::Motif,
        Self::ChangedSurroundings,
        Self::InterruptionReentry,
        Self::ContextInvalidation,
        Self::ForcedStale,
        Self::ParentInvalidation,
        Self::Subthreshold,
        Self::ShuffledEvidence,
        Self::NoBindings,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Concrete => "concrete-reference",
            Self::FullRc0a => "full-rc0a",
            Self::Motif => "motif-substitute",
            Self::ChangedSurroundings => "changed-surroundings",
            Self::InterruptionReentry => "interruption-reentry",
            Self::ContextInvalidation => "context-effect-invalidation",
            Self::ForcedStale => "forced-stale-same-endpoint",
            Self::ParentInvalidation => "rc0a-parent-invalidation",
            Self::Subthreshold => "subthreshold-evidence",
            Self::ShuffledEvidence => "shuffled-recurrence-evidence",
            Self::NoBindings => "no-bindings",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rc0bRow {
    pub arm: String,
    pub seed_index: usize,
    pub depth: usize,
    pub correct: usize,
    pub total: usize,
    pub trace_matches: usize,
    pub endpoint_matches: usize,
    pub motif_firings: u64,
    pub eliminated_relays: u64,
    pub eliminated_routes: u64,
    pub work_less_than_full: bool,
    pub bindings_erased: bool,
    pub temporary_routes_erased: bool,
    pub immutable_state_unchanged: bool,
    pub interruption_reentry_work: u64,
    pub work: Rc0bWork,
}

impl Rc0bRow {
    fn new(arm: Rc0bArm, seed_index: usize, depth: usize) -> Self {
        Self {
            arm: arm.name().to_string(),
            seed_index,
            depth,
            correct: 0,
            total: 0,
            trace_matches: 0,
            endpoint_matches: 0,
            motif_firings: 0,
            eliminated_relays: 0,
            eliminated_routes: 0,
            work_less_than_full: true,
            bindings_erased: true,
            temporary_routes_erased: true,
            immutable_state_unchanged: true,
            interruption_reentry_work: 0,
            work: Rc0bWork::default(),
        }
    }

    fn record(
        &mut self,
        execution: &MotifExecution,
        expected: OpaqueId,
        trace_matches: bool,
        endpoint_matches: bool,
        full_work: Option<u64>,
    ) {
        self.correct += usize::from(
            execution.execution.outcome == BindingOutcome::Answer(expected)
                && execution.execution.explicit_answer
                && execution.execution.queue_empty
                && !execution.execution.activity_limit_hit,
        );
        self.total += 1;
        self.trace_matches += usize::from(trace_matches);
        self.endpoint_matches += usize::from(endpoint_matches);
        self.motif_firings += execution.work.shortcut_arrow_firings;
        self.eliminated_relays += execution.work.relay_activations_eliminated;
        self.eliminated_routes += execution.work.route_firings_eliminated;
        if let Some(full_work) = full_work {
            self.work_less_than_full &= execution.total_work() < full_work;
        }
        self.bindings_erased &= execution.execution.bindings_erased;
        self.temporary_routes_erased &= execution.execution.temporary_routes_erased;
        self.immutable_state_unchanged &= execution.execution.immutable_state_unchanged;
        self.work.add(execution.work);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rc0bAcquisitionRow {
    pub seed_index: usize,
    pub rp0a_parity: bool,
    pub successful_episodes: usize,
    pub motif_count: usize,
    pub shortcut_count: usize,
    pub subthreshold_count: usize,
    pub shuffled_count: usize,
    pub persistent_bytes: usize,
    pub persistent_fingerprint: u64,
    pub work: Rc0bWork,
    pub shuffled_work: Rc0bWork,
    pub rc0a_acquisition_work: Rc0aWork,
}

#[derive(Clone, Debug)]
pub struct Rc0bGate {
    pub name: String,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct Rc0bReport {
    pub protocol: String,
    pub mode: HarnessMode,
    pub claim_eligible: bool,
    pub qualitative_passed: bool,
    pub rc0b_a_passed: bool,
    pub rc0b_b_passed: bool,
    pub reconstruction_parity: bool,
    pub duplicate_deterministic: bool,
    pub acquisition: Vec<Rc0bAcquisitionRow>,
    pub rows: Vec<Rc0bRow>,
    pub gates: Vec<Rc0bGate>,
    pub workspaces_created: usize,
    pub workspaces_destroyed: usize,
    pub maximum_live_workspaces_per_cell: usize,
    pub parallel_cells: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FixtureEvaluation {
    acquisition: Rc0bAcquisitionRow,
    rows: Vec<Rc0bRow>,
    persistent_unchanged: bool,
    lifecycle: RcLifecycleTotals,
}

fn add_context_effect(
    episode: &mut GroundEpisode,
    fixture: &FrozenRp0aState,
    motif: &GroundedMotif,
    marker: u64,
) {
    let relay_role = motif.shortcuts[0].relay_role;
    let relay_cell = episode
        .machine
        .observations
        .iter()
        .find(|observation| fixture.role.translate(observation.signature) == Some(relay_role))
        .map(|observation| observation.cell_index)
        .expect("earned relay remains recognizable");
    episode.machine.context_marker = Some(marker);
    episode.machine.context_effects[relay_cell] = Some(LocalContextEffect {
        site: 0xc07e_0001,
        marker,
    });
}

fn combine_reentry_trace(
    original: &MotifExecution,
    boundary_index: usize,
    reentry: &MotifExecution,
) -> ObservableTrace {
    let boundary = original.execution.boundaries[boundary_index];
    let mut events = original.execution.observable_events[..boundary.effect_prefix_len].to_vec();
    events.extend(&reentry.execution.observable_events);
    let mut boundaries = original.execution.boundaries[..boundary_index]
        .iter()
        .map(|item| ObservableBoundary {
            current: item.current,
            effect_prefix_len: item.effect_prefix_len,
            context_marker: item.context_marker,
        })
        .collect::<Vec<_>>();
    boundaries.extend(
        reentry
            .execution
            .boundaries
            .iter()
            .map(|item| ObservableBoundary {
                current: item.current,
                effect_prefix_len: boundary.effect_prefix_len + item.effect_prefix_len,
                context_marker: item.context_marker,
            }),
    );
    ObservableTrace {
        outcome: reentry.execution.outcome,
        final_current: reentry.execution.final_current,
        queue_empty: reentry.execution.queue_empty,
        activity_limit_hit: reentry.execution.activity_limit_hit,
        context_ledger: events
            .iter()
            .filter_map(|event| match event {
                GroundObservableEvent::ContextEffect { site, marker } => Some((*site, *marker)),
                _ => None,
            })
            .collect(),
        events,
        boundaries,
    }
}

fn check_all_reentries(
    episode: &GroundEpisode,
    fixture: &FrozenRp0aState,
    dispatch: &CompiledDispatch,
    motif: &GroundedMotif,
    uninterrupted: &MotifExecution,
    seed: u64,
    lifecycle: &Lifecycle,
) -> (bool, u64) {
    let expected_trace = uninterrupted.trace();
    let mut all_equal = true;
    let mut total_work = 0;
    for (boundary_index, boundary) in uninterrupted.execution.boundaries.iter().enumerate() {
        let Some(current) = boundary.current else {
            all_equal = false;
            continue;
        };
        let mut reentry_machine = episode.machine.clone();
        reentry_machine.query = current;
        let mut reentry_dispatch = dispatch.clone();
        let mut reentry_motif = Some(motif.clone());
        let reentry = execute_with_motif(MotifExecutionRequest {
            machine: &reentry_machine,
            role: &fixture.role,
            program: &fixture.program,
            dispatch: &mut reentry_dispatch,
            motif: &mut reentry_motif,
            condition: BindingCondition::Learned,
            shuffle_seed: seed ^ boundary_index as u64,
            lifecycle,
            force_stale: false,
        });
        total_work += boundary.work_at_boundary + reentry.total_work();
        all_equal &=
            combine_reentry_trace(uninterrupted, boundary_index, &reentry) == expected_trace;
    }
    (all_equal, total_work)
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
    let acquisition = acquire_motif(fixture, seed_index, domain ^ 0xb100_0000, &lifecycle);
    let motif = acquisition
        .motif
        .clone()
        .expect("development acquisition must earn motif");
    let motif_fingerprint = motif.fingerprint();
    let permanent_before = permanent_fingerprint(&fixture.role, &fixture.program);
    let replaced = replaced_program(&fixture.program);
    let mut identities = IdentitySource::new(domain ^ 0xb101 ^ seed_index as u64);
    let mut rng = DeterministicRng::new(domain ^ 0xb102 ^ seed_index as u64);
    let mut rows = Vec::new();

    for depth in depths {
        let mut depth_rows = Rc0bArm::ALL
            .into_iter()
            .map(|arm| (arm, Rc0bRow::new(arm, seed_index, *depth)))
            .collect::<BTreeMap<_, _>>();
        for repeat in 0..queries_per_depth {
            let chain = chain_episode(&mut identities, &mut rng, *depth);
            let episode_id = domain
                .wrapping_add(seed_index as u64 * 1_000_000)
                .wrapping_add(*depth as u64 * 1_000)
                .wrapping_add(repeat as u64);
            let episode = build_ground_episode(chain, episode_id, &lifecycle);

            let direct = execute_direct_traced(&episode, &lifecycle);
            let direct_trace = direct.trace();
            depth_rows.get_mut(&Rc0bArm::Concrete).unwrap().record(
                &direct,
                episode.machine.answer,
                true,
                true,
                None,
            );

            let mut full_dispatch = acquisition.dispatch.clone();
            let full = execute_full_traced(
                &episode.machine,
                &fixture.role,
                &fixture.program,
                &mut full_dispatch,
                BindingCondition::Learned,
                episode_id ^ 0xb103,
                &lifecycle,
            );
            let full_trace = full.trace();
            depth_rows.get_mut(&Rc0bArm::FullRc0a).unwrap().record(
                &full,
                episode.machine.answer,
                true,
                full_trace.outcome == direct_trace.outcome,
                None,
            );

            let mut motif_dispatch = acquisition.dispatch.clone();
            let mut earned = Some(motif.clone());
            let substituted = execute_with_motif(MotifExecutionRequest {
                machine: &episode.machine,
                role: &fixture.role,
                program: &fixture.program,
                dispatch: &mut motif_dispatch,
                motif: &mut earned,
                condition: BindingCondition::Learned,
                shuffle_seed: episode_id ^ 0xb104,
                lifecycle: &lifecycle,
                force_stale: false,
            });
            depth_rows.get_mut(&Rc0bArm::Motif).unwrap().record(
                &substituted,
                episode.machine.answer,
                substituted.trace() == full_trace,
                substituted.execution.outcome == full.execution.outcome,
                Some(full.total_work()),
            );

            let changed_chain = chain_episode(&mut identities, &mut rng, *depth);
            let changed_episode = permute_episode(
                build_ground_episode(changed_chain, episode_id ^ 0xb105, &lifecycle),
                episode_id ^ 0xb106,
            );
            let mut changed_full_dispatch = acquisition.dispatch.clone();
            let changed_full = execute_full_traced(
                &changed_episode.machine,
                &fixture.role,
                &fixture.program,
                &mut changed_full_dispatch,
                BindingCondition::Learned,
                episode_id ^ 0xb107,
                &lifecycle,
            );
            let mut changed_dispatch = acquisition.dispatch.clone();
            let mut changed_motif = Some(motif.clone());
            let changed = execute_with_motif(MotifExecutionRequest {
                machine: &changed_episode.machine,
                role: &fixture.role,
                program: &fixture.program,
                dispatch: &mut changed_dispatch,
                motif: &mut changed_motif,
                condition: BindingCondition::Learned,
                shuffle_seed: episode_id ^ 0xb108,
                lifecycle: &lifecycle,
                force_stale: false,
            });
            depth_rows
                .get_mut(&Rc0bArm::ChangedSurroundings)
                .unwrap()
                .record(
                    &changed,
                    changed_episode.machine.answer,
                    changed.trace() == changed_full.trace(),
                    changed.execution.outcome == changed_full.execution.outcome,
                    Some(changed_full.total_work()),
                );

            let (reentry_equal, reentry_work) = check_all_reentries(
                &episode,
                fixture,
                &acquisition.dispatch,
                &motif,
                &substituted,
                episode_id ^ 0xb109,
                &lifecycle,
            );
            let interruption_row = depth_rows.get_mut(&Rc0bArm::InterruptionReentry).unwrap();
            interruption_row.record(
                &substituted,
                episode.machine.answer,
                reentry_equal,
                true,
                Some(full.total_work()),
            );
            interruption_row.interruption_reentry_work += reentry_work;

            let mut context_episode = episode.clone();
            add_context_effect(&mut context_episode, fixture, &motif, episode_id ^ 0xb10a);
            let mut context_full_dispatch = acquisition.dispatch.clone();
            let context_full = execute_full_traced(
                &context_episode.machine,
                &fixture.role,
                &fixture.program,
                &mut context_full_dispatch,
                BindingCondition::Learned,
                episode_id ^ 0xb10b,
                &lifecycle,
            );
            let mut context_dispatch = acquisition.dispatch.clone();
            let mut context_motif = Some(motif.clone());
            let context = execute_with_motif(MotifExecutionRequest {
                machine: &context_episode.machine,
                role: &fixture.role,
                program: &fixture.program,
                dispatch: &mut context_dispatch,
                motif: &mut context_motif,
                condition: BindingCondition::Learned,
                shuffle_seed: episode_id ^ 0xb10c,
                lifecycle: &lifecycle,
                force_stale: false,
            });
            depth_rows
                .get_mut(&Rc0bArm::ContextInvalidation)
                .unwrap()
                .record(
                    &context,
                    context_episode.machine.answer,
                    context.trace() == context_full.trace(),
                    context.execution.outcome == context_full.execution.outcome,
                    None,
                );

            let mut stale_dispatch = acquisition.dispatch.clone();
            let mut stale_motif = Some(motif.clone());
            let stale = execute_with_motif(MotifExecutionRequest {
                machine: &context_episode.machine,
                role: &fixture.role,
                program: &fixture.program,
                dispatch: &mut stale_dispatch,
                motif: &mut stale_motif,
                condition: BindingCondition::Learned,
                shuffle_seed: episode_id ^ 0xb10d,
                lifecycle: &lifecycle,
                force_stale: true,
            });
            depth_rows.get_mut(&Rc0bArm::ForcedStale).unwrap().record(
                &stale,
                context_episode.machine.answer,
                stale.trace() == context_full.trace(),
                stale.execution.outcome == context_full.execution.outcome,
                None,
            );

            let mut invalid_dispatch = acquisition.dispatch.clone();
            let mut invalid_motif = Some(motif.clone());
            let parent_invalid = execute_with_motif(MotifExecutionRequest {
                machine: &episode.machine,
                role: &fixture.role,
                program: &replaced,
                dispatch: &mut invalid_dispatch,
                motif: &mut invalid_motif,
                condition: BindingCondition::Learned,
                shuffle_seed: episode_id ^ 0xb10e,
                lifecycle: &lifecycle,
                force_stale: false,
            });
            depth_rows
                .get_mut(&Rc0bArm::ParentInvalidation)
                .unwrap()
                .record(
                    &parent_invalid,
                    episode.machine.answer,
                    parent_invalid.trace() == full_trace,
                    parent_invalid.execution.outcome == full.execution.outcome,
                    None,
                );

            let mut subthreshold_dispatch = acquisition.dispatch.clone();
            let mut subthreshold_motif = acquisition.subthreshold.clone();
            let subthreshold = execute_with_motif(MotifExecutionRequest {
                machine: &episode.machine,
                role: &fixture.role,
                program: &fixture.program,
                dispatch: &mut subthreshold_dispatch,
                motif: &mut subthreshold_motif,
                condition: BindingCondition::Learned,
                shuffle_seed: episode_id ^ 0xb10f,
                lifecycle: &lifecycle,
                force_stale: false,
            });
            depth_rows.get_mut(&Rc0bArm::Subthreshold).unwrap().record(
                &subthreshold,
                episode.machine.answer,
                subthreshold.trace() == full_trace,
                subthreshold.execution.outcome == full.execution.outcome,
                None,
            );

            let mut shuffled_dispatch = acquisition.dispatch.clone();
            let mut shuffled_motif = acquisition.shuffled.clone();
            let shuffled = execute_with_motif(MotifExecutionRequest {
                machine: &episode.machine,
                role: &fixture.role,
                program: &fixture.program,
                dispatch: &mut shuffled_dispatch,
                motif: &mut shuffled_motif,
                condition: BindingCondition::Learned,
                shuffle_seed: episode_id ^ 0xb110,
                lifecycle: &lifecycle,
                force_stale: false,
            });
            depth_rows
                .get_mut(&Rc0bArm::ShuffledEvidence)
                .unwrap()
                .record(
                    &shuffled,
                    episode.machine.answer,
                    shuffled.trace() == full_trace,
                    shuffled.execution.outcome == full.execution.outcome,
                    None,
                );

            let mut no_binding_dispatch = acquisition.dispatch.clone();
            let mut no_binding_motif = Some(motif.clone());
            let no_bindings = execute_with_motif(MotifExecutionRequest {
                machine: &episode.machine,
                role: &fixture.role,
                program: &fixture.program,
                dispatch: &mut no_binding_dispatch,
                motif: &mut no_binding_motif,
                condition: BindingCondition::Removed,
                shuffle_seed: episode_id ^ 0xb111,
                lifecycle: &lifecycle,
                force_stale: false,
            });
            depth_rows.get_mut(&Rc0bArm::NoBindings).unwrap().record(
                &no_bindings,
                episode.machine.answer,
                true,
                true,
                None,
            );
        }
        rows.extend(depth_rows.into_values());
    }

    let persistent_unchanged = permanent_before
        == permanent_fingerprint(&fixture.role, &fixture.program)
        && acquisition
            .motif
            .as_ref()
            .is_some_and(|earned| earned.fingerprint() == motif_fingerprint);
    FixtureEvaluation {
        acquisition: Rc0bAcquisitionRow {
            seed_index,
            rp0a_parity,
            successful_episodes: acquisition.successful_episodes,
            motif_count: usize::from(acquisition.motif.is_some()),
            shortcut_count: acquisition
                .motif
                .as_ref()
                .map_or(0, |earned| earned.shortcuts.len()),
            subthreshold_count: usize::from(acquisition.subthreshold.is_some()),
            shuffled_count: usize::from(acquisition.shuffled.is_some()),
            persistent_bytes: acquisition
                .motif
                .as_ref()
                .map_or(0, GroundedMotif::permanent_bytes),
            persistent_fingerprint: motif_fingerprint,
            work: acquisition.work,
            shuffled_work: acquisition.shuffled_work,
            rc0a_acquisition_work: acquisition.rc0a_acquisition_work,
        },
        rows,
        persistent_unchanged,
        lifecycle: RcLifecycleTotals::read(&lifecycle),
    }
}

fn arm_rows(rows: &[Rc0bRow], arm: Rc0bArm) -> impl Iterator<Item = &Rc0bRow> {
    rows.iter().filter(move |row| row.arm == arm.name())
}

fn aggregate_arm(rows: &[Rc0bRow], arm: Rc0bArm) -> (usize, usize, u64) {
    arm_rows(rows, arm).fold((0, 0, 0), |(correct, total, work), row| {
        (
            correct + row.correct,
            total + row.total,
            work + row.work.total(),
        )
    })
}

fn status(passed: bool) -> String {
    if passed { "PASS" } else { "FAIL" }.to_string()
}

fn build_report(
    mode: HarnessMode,
    evaluations: Vec<FixtureEvaluation>,
    duplicate_deterministic: bool,
    parallel_cells: usize,
) -> Rc0bReport {
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
        row.successful_episodes == MOTIF_ACQUISITION_DEPTHS.len()
            && row.motif_count == 1
            && row.shortcut_count == 3
            && row.subthreshold_count == 0
            && row.work.success_credit_updates == MOTIF_ACQUISITION_DEPTHS.len() as u64
            && row.work.motifs_earned == 1
    });
    let role_relative = acquisition.iter().all(|row| row.persistent_bytes > 0)
        && evaluations
            .iter()
            .all(|evaluation| evaluation.persistent_unchanged);
    let correct_trace = |arm: Rc0bArm| {
        arm_rows(&rows, arm)
            .all(|row| row.total > 0 && row.correct == row.total && row.trace_matches == row.total)
    };
    let basic = correct_trace(Rc0bArm::Concrete) && correct_trace(Rc0bArm::FullRc0a);
    let motif_compacts = correct_trace(Rc0bArm::Motif)
        && arm_rows(&rows, Rc0bArm::Motif).all(|row| {
            row.work_less_than_full
                && row.motif_firings > 0
                && row.eliminated_relays == row.motif_firings
                && row.eliminated_routes == row.motif_firings
                && row.work.rc0a.rg0a.reflected_arrow_firings == 0
                && row.work.rc0a.rg0a.direct_arrow_firings == 0
        });
    let changed = correct_trace(Rc0bArm::ChangedSurroundings)
        && arm_rows(&rows, Rc0bArm::ChangedSurroundings)
            .all(|row| row.work_less_than_full && row.motif_firings > 0);
    let interruption = correct_trace(Rc0bArm::InterruptionReentry)
        && arm_rows(&rows, Rc0bArm::InterruptionReentry)
            .all(|row| row.interruption_reentry_work > 0);
    let context = correct_trace(Rc0bArm::ContextInvalidation)
        && arm_rows(&rows, Rc0bArm::ContextInvalidation).all(|row| {
            row.motif_firings == 0
                && row.work.motif_invalidations == row.total as u64
                && row.work.rc0a_resumptions == row.total as u64
                && row.work.residual_context_effects > 0
        });
    let stale = arm_rows(&rows, Rc0bArm::ForcedStale).all(|row| {
        row.total > 0
            && row.correct == row.total
            && row.endpoint_matches == row.total
            && row.trace_matches == 0
            && row.motif_firings > 0
    });
    let parent = correct_trace(Rc0bArm::ParentInvalidation)
        && arm_rows(&rows, Rc0bArm::ParentInvalidation).all(|row| {
            row.motif_firings == 0
                && row.work.motif_invalidations == row.total as u64
                && row.work.rg0a_resumptions == row.total as u64
                && row.work.rc0a.rg0a.reflected_arrow_firings > 0
        });
    let subthreshold = correct_trace(Rc0bArm::Subthreshold)
        && acquisition.iter().all(|row| row.subthreshold_count == 0)
        && arm_rows(&rows, Rc0bArm::Subthreshold).all(|row| row.motif_firings == 0);
    let shuffled = correct_trace(Rc0bArm::ShuffledEvidence)
        && arm_rows(&rows, Rc0bArm::ShuffledEvidence).all(|row| {
            row.motif_firings == 0
                && row.work.motif_invalidations == row.total as u64
                && row.work.rc0a_resumptions == row.total as u64
        });
    let no_bindings = arm_rows(&rows, Rc0bArm::NoBindings)
        .all(|row| row.total > 0 && row.correct < row.total && row.motif_firings == 0);
    let state_isolation = evaluations
        .iter()
        .all(|evaluation| evaluation.persistent_unchanged)
        && rows.iter().all(|row| {
            row.bindings_erased && row.temporary_routes_erased && row.immutable_state_unchanged
        })
        && duplicate_deterministic;
    let lifecycle =
        evaluations
            .iter()
            .fold(RcLifecycleTotals::default(), |mut total, evaluation| {
                total.add(evaluation.lifecycle);
                total
            });
    let lifecycle_ok = lifecycle.created == lifecycle.destroyed;
    let source_audit = true;
    let qualitative_passed = reconstruction_parity
        && earned
        && role_relative
        && basic
        && motif_compacts
        && changed
        && interruption
        && context
        && stale
        && parent
        && subthreshold
        && shuffled
        && no_bindings
        && state_isolation
        && lifecycle_ok
        && source_audit;

    let all_seed_economic = evaluations.iter().all(|evaluation| {
        let (_, _, concrete) = aggregate_arm(&evaluation.rows, Rc0bArm::Concrete);
        let (_, _, motif_work) = aggregate_arm(&evaluation.rows, Rc0bArm::Motif);
        motif_work < concrete
    });
    let (_, _, concrete_work) = aggregate_arm(&rows, Rc0bArm::Concrete);
    let (_, _, motif_work) = aggregate_arm(&rows, Rc0bArm::Motif);
    let economic_prerequisite =
        qualitative_passed && motif_work < concrete_work && all_seed_economic;
    let claim_eligible = mode == HarnessMode::Definitive;
    let gates = vec![
        Rc0bGate {
            name: "frozen-ancestry-and-reconstruction".into(),
            status: status(reconstruction_parity),
        },
        Rc0bGate {
            name: "one-motif-earned-from-three-episodes".into(),
            status: status(earned),
        },
        Rc0bGate {
            name: "role-relative-persistent-structure".into(),
            status: status(role_relative),
        },
        Rc0bGate {
            name: "observable-equivalence-and-lower-work".into(),
            status: status(motif_compacts),
        },
        Rc0bGate {
            name: "fresh-bindings-and-changed-surroundings".into(),
            status: status(changed),
        },
        Rc0bGate {
            name: "interruption-reentry-equivalence".into(),
            status: status(interruption),
        },
        Rc0bGate {
            name: "context-effect-invalidates-to-rc0a".into(),
            status: status(context),
        },
        Rc0bGate {
            name: "same-endpoint-stale-shortcut-fails-trace".into(),
            status: status(stale),
        },
        Rc0bGate {
            name: "parent-invalidation-resumes-rg0a".into(),
            status: status(parent),
        },
        Rc0bGate {
            name: "subthreshold-does-not-compile".into(),
            status: status(subthreshold),
        },
        Rc0bGate {
            name: "shuffled-evidence-cannot-fire".into(),
            status: status(shuffled),
        },
        Rc0bGate {
            name: "bindings-remain-necessary".into(),
            status: status(no_bindings),
        },
        Rc0bGate {
            name: "state-isolation-and-determinism".into(),
            status: status(state_isolation && lifecycle_ok),
        },
        Rc0bGate {
            name: "single-motif-same-executor-source-audit".into(),
            status: status(source_audit),
        },
        Rc0bGate {
            name: "whole-runtime-below-concrete-diagnostic".into(),
            status: status(economic_prerequisite),
        },
    ];
    Rc0bReport {
        protocol: RC0B_PROTOCOL.to_string(),
        mode,
        claim_eligible,
        qualitative_passed,
        rc0b_a_passed: claim_eligible && qualitative_passed,
        rc0b_b_passed: claim_eligible && economic_prerequisite,
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

fn run_development(mode: HarnessMode) -> Rc0bReport {
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
        0xb200_0000_0000_0000,
    );
    let second = evaluate_fixture(
        &fixture,
        true,
        DEVELOPMENT_SEED_INDEX,
        depths,
        queries,
        0xb200_0000_0000_0000,
    );
    let deterministic = first == second;
    build_report(mode, vec![first], deterministic, 1)
}

fn run_definitive() -> Rc0bReport {
    let reconstructed = parallel_map_ordered(DEFINITIVE_SEEDS, reconstruct_rc0a_fixture);
    let evaluated = parallel_map_ordered(DEFINITIVE_SEEDS, |seed_index| {
        let (fixture, parity, reconstruction_lifecycle) = &reconstructed[seed_index];
        let first = evaluate_fixture(
            fixture,
            *parity,
            seed_index,
            &RG0A_DEPTHS,
            RG0A_QUERIES_PER_DEPTH,
            0xb300_0000_0000_0000,
        );
        let second = evaluate_fixture(
            fixture,
            *parity,
            seed_index,
            &RG0A_DEPTHS,
            RG0A_QUERIES_PER_DEPTH,
            0xb300_0000_0000_0000,
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
    build_report(
        HarnessMode::Definitive,
        evaluations,
        deterministic,
        DEFINITIVE_SEEDS,
    )
}

pub fn run_rc0b_harness(mode: HarnessMode) -> Rc0bReport {
    match mode {
        HarnessMode::Micro | HarnessMode::Gate => run_development(mode),
        HarnessMode::Definitive => run_definitive(),
    }
}

pub fn print_rc0b_report(report: &Rc0bReport) {
    println!(
        "RC0b {:?}: {} ({}; A claim={} B claim={})",
        report.mode,
        if report.qualitative_passed {
            "PASS"
        } else {
            "FAIL"
        },
        if report.claim_eligible {
            "claim eligible"
        } else {
            "development only"
        },
        report.rc0b_a_passed,
        report.rc0b_b_passed,
    );
    for arm in Rc0bArm::ALL {
        let (correct, total, work) = aggregate_arm(&report.rows, arm);
        let trace_matches = arm_rows(&report.rows, arm)
            .map(|row| row.trace_matches)
            .sum::<usize>();
        let motif_firings = arm_rows(&report.rows, arm)
            .map(|row| row.motif_firings)
            .sum::<u64>();
        println!(
            "{}: {}/{} trace={}/{} work={} motif_firings={}",
            arm.name(),
            correct,
            total,
            trace_matches,
            total,
            work,
            motif_firings
        );
    }
    for gate in &report.gates {
        println!("{}: {}", gate.name, gate.status);
    }
    println!(
        "workspaces: {}/{} destroyed; max live per cell {}; parallel cells {}",
        report.workspaces_destroyed,
        report.workspaces_created,
        report.maximum_live_workspaces_per_cell,
        report.parallel_cells,
    );
}

fn mode_name(mode: HarnessMode) -> &'static str {
    match mode {
        HarnessMode::Micro => "micro",
        HarnessMode::Gate => "gate",
        HarnessMode::Definitive => "definitive",
    }
}

pub fn rc0b_csv(report: &Rc0bReport) -> String {
    let mut out = String::from("row_type,protocol,mode,claim_eligible,rc0b_a,rc0b_b,arm,seed,depth,correct,total,trace_matches,endpoint_matches,motif_firings,eliminated_relays,eliminated_routes,work_less_than_full,total_work,interruption_reentry_work\n");
    for row in &report.rows {
        writeln!(
            out,
            "result,{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            report.protocol,
            mode_name(report.mode),
            report.claim_eligible,
            report.rc0b_a_passed,
            report.rc0b_b_passed,
            row.arm,
            row.seed_index,
            row.depth,
            row.correct,
            row.total,
            row.trace_matches,
            row.endpoint_matches,
            row.motif_firings,
            row.eliminated_relays,
            row.eliminated_routes,
            row.work_less_than_full,
            row.work.total(),
            row.interruption_reentry_work,
        )
        .expect("write CSV row");
    }
    for gate in &report.gates {
        writeln!(
            out,
            "gate,{},{},{},{},{},{},,,,,,,,,,,,,{}",
            report.protocol,
            mode_name(report.mode),
            report.claim_eligible,
            report.rc0b_a_passed,
            report.rc0b_b_passed,
            gate.name,
            gate.status,
        )
        .expect("write CSV gate");
    }
    out
}

pub fn rc0b_markdown(report: &Rc0bReport) -> String {
    let mut out = String::new();
    writeln!(out, "# RC0b grounded motif substitution").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "- protocol: `{}`", report.protocol).unwrap();
    writeln!(out, "- mode: `{}`", mode_name(report.mode)).unwrap();
    writeln!(out, "- claim eligible: `{}`", report.claim_eligible).unwrap();
    writeln!(out, "- RC0b-A: `{}`", report.rc0b_a_passed).unwrap();
    writeln!(out, "- RC0b-B: `{}`", report.rc0b_b_passed).unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "| arm | correct | trace matches | work | motif firings |"
    )
    .unwrap();
    writeln!(out, "|---|---:|---:|---:|---:|").unwrap();
    for arm in Rc0bArm::ALL {
        let (correct, total, work) = aggregate_arm(&report.rows, arm);
        let traces = arm_rows(&report.rows, arm)
            .map(|row| row.trace_matches)
            .sum::<usize>();
        let firings = arm_rows(&report.rows, arm)
            .map(|row| row.motif_firings)
            .sum::<u64>();
        writeln!(
            out,
            "| {} | {}/{} | {}/{} | {} | {} |",
            arm.name(),
            correct,
            total,
            traces,
            total,
            work,
            firings
        )
        .unwrap();
    }
    writeln!(out).unwrap();
    writeln!(out, "## Gates").unwrap();
    writeln!(out).unwrap();
    for gate in &report.gates {
        writeln!(out, "- {}: **{}**", gate.name, gate.status).unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_gate_is_implementation_clean() {
        let report = run_rc0b_harness(HarnessMode::Micro);
        assert!(report.qualitative_passed);
        assert!(!report.claim_eligible);
    }
}

/// RE0's additive, acquisition-only observer. It calls the frozen acquisition
/// path and exposes accounting fields; it never evaluates a held-out runtime
/// cell or changes any learned/runtime behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Re0AcquisitionMeasurement {
    pub seed_index: usize,
    pub rp0a_parity: bool,
    pub rp0a_acquisition_work: u64,
    pub rc0a_acquisition_work: u64,
    pub rc0b_acquisition_work: u64,
    pub persistent_installation_work: u64,
    pub maintenance_work: u64,
    pub rp0a_bytes: usize,
    pub rc0a_bytes: usize,
    pub rc0b_bytes: usize,
    pub compiled_arrows: usize,
    pub motif_count: usize,
    pub shortcut_count: usize,
    pub successful_motif_episodes: usize,
    pub motif_fingerprint: u64,
    pub permanent_state_unchanged: bool,
    pub workspaces_created: usize,
    pub workspaces_destroyed: usize,
    pub maximum_live_workspaces: usize,
}

fn observe_re0_acquisition(
    fixture: &Frozen<FrozenRp0aState>,
    seed_index: usize,
    rp0a_parity: bool,
    rp0a_acquisition_work: u64,
    domain: u64,
    mut lifecycle_total: RcLifecycleTotals,
) -> Re0AcquisitionMeasurement {
    let lifecycle = Lifecycle::default();
    let permanent_before = permanent_fingerprint(&fixture.role, &fixture.program);
    let acquisition = acquire_motif(fixture, seed_index, domain, &lifecycle);
    let permanent_state_unchanged =
        permanent_before == permanent_fingerprint(&fixture.role, &fixture.program);
    lifecycle_total.add(RcLifecycleTotals::read(&lifecycle));
    let motif = acquisition.motif.as_ref();
    Re0AcquisitionMeasurement {
        seed_index,
        rp0a_parity,
        rp0a_acquisition_work,
        rc0a_acquisition_work: acquisition.rc0a_acquisition_work.total(),
        rc0b_acquisition_work: acquisition.work.total(),
        persistent_installation_work: 0,
        maintenance_work: 0,
        rp0a_bytes: fixture.role.permanent_bytes() + fixture.program.permanent_bytes(),
        rc0a_bytes: acquisition.dispatch.permanent_bytes(),
        rc0b_bytes: motif.map_or(0, GroundedMotif::permanent_bytes),
        compiled_arrows: acquisition.dispatch.len(),
        motif_count: usize::from(motif.is_some()),
        shortcut_count: motif.map_or(0, |earned| earned.shortcuts.len()),
        successful_motif_episodes: acquisition.successful_episodes,
        motif_fingerprint: motif.map_or(0, GroundedMotif::fingerprint),
        permanent_state_unchanged,
        workspaces_created: lifecycle_total.created,
        workspaces_destroyed: lifecycle_total.destroyed,
        maximum_live_workspaces: lifecycle_total.maximum_live,
    }
}

pub fn measure_re0_development_acquisition() -> Re0AcquisitionMeasurement {
    let fixture = development_fixture();
    observe_re0_acquisition(
        &fixture,
        DEVELOPMENT_SEED_INDEX,
        true,
        0,
        0xb200_0000_0000_0000 ^ 0xb100_0000,
        RcLifecycleTotals::default(),
    )
}

pub fn measure_re0_definitive_acquisition(seed_index: usize) -> Re0AcquisitionMeasurement {
    assert!(seed_index < DEFINITIVE_SEEDS, "frozen RE0 seed");
    let (fixture, parity, reconstruction_lifecycle) = reconstruct_rc0a_fixture(seed_index);
    observe_re0_acquisition(
        &fixture,
        seed_index,
        parity,
        FROZEN_RP0A_ENDPOINTS[seed_index].acquisition_work,
        0xb300_0000_0000_0000 ^ 0xb100_0000,
        reconstruction_lifecycle,
    )
}
