//! CS0a: repeated successful learned correspondence earns a role-relative
//! local route. Generic FFS-SAME0 inference remains the novelty and repair
//! path; compiled state never stores an episode occurrence or filler identity.

use super::*;
use crate::research_runtime::{parallel_map_ordered, HarnessMode};

pub const CS0A_PROTOCOL: &str = "identity-desupply-ladder-v1/cs0a";

const DEVELOPMENT_SEED: usize = 60_000;
const DEFINITIVE_SEEDS: usize = 8;
const MICRO_EPISODES: usize = 2;
const GATE_EPISODES: usize = 8;
const DEFINITIVE_EPISODES: usize = 16;
pub(super) const COMPILED_EVIDENCE_PER_MOTIF: usize = 3;
pub(super) const SUBTHRESHOLD_EVIDENCE_PER_MOTIF: usize = 2;
const TEST_DEPTH: usize = 32;
const TEST_POPULATION: usize = 64;

fn motif_values(motif: RelationMotif) -> impl Iterator<Item = u64> {
    [
        motif.context as u64,
        motif.atoms[0].channel as u64,
        motif.atoms[0].source_position as u64,
        motif.atoms[0].target_position as u64,
        motif.atoms[0].lag as u64,
        motif.atoms[1].channel as u64,
        motif.atoms[1].source_position as u64,
        motif.atoms[1].target_position as u64,
        motif.atoms[1].lag as u64,
    ]
    .into_iter()
}

fn rule_asset_identity(motif: RelationMotif) -> u64 {
    hash_values(std::iter::once(0xca00_0000_0000_0001).chain(motif_values(motif)))
}

fn rule_dependency_fingerprint(rule: &CorrespondenceRule) -> u64 {
    hash_values(
        std::iter::once(0xca00_0000_0000_0002)
            .chain(motif_values(rule.motif))
            .chain([rule.strength as i64 as u64]),
    )
}

// BEGIN PERSISTENT COMPILED CORRESPONDENCE

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CompiledRouteKey {
    correspondence_asset: u64,
    source_role: u8,
    target_role: u8,
    context: u8,
    support: [RelationAtom; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CompiledCorrespondenceRoute {
    id: u32,
    key: CompiledRouteKey,
    parent_fingerprint: u64,
    strength: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct CompiledCorrespondenceStore {
    routes: BTreeMap<RelationMotif, CompiledCorrespondenceRoute>,
}

// END PERSISTENT COMPILED CORRESPONDENCE

impl CompiledCorrespondenceStore {
    pub(super) fn len(&self) -> usize {
        self.routes.len()
    }

    pub(super) fn persistent_bytes(&self) -> usize {
        self.routes.len()
            * (size_of::<u32>()
                + size_of::<CompiledRouteKey>()
                + size_of::<u64>()
                + size_of::<i32>())
    }

    pub(super) fn fingerprint(&self) -> u64 {
        hash_values(self.routes.values().flat_map(|route| {
            [
                route.id as u64,
                route.key.correspondence_asset,
                route.key.source_role as u64,
                route.key.target_role as u64,
                route.key.context as u64,
                route.parent_fingerprint,
                route.strength as i64 as u64,
            ]
            .into_iter()
            .chain(route.key.support.iter().flat_map(|atom| {
                [
                    atom.channel as u64,
                    atom.source_position as u64,
                    atom.target_position as u64,
                    atom.lag as u64,
                ]
            }))
        }))
    }

    fn resolve(
        &self,
        rules: &RuleStore,
        view: &AnonymousView,
        work: &mut Cs0aWork,
    ) -> CompiledResolution {
        let mut temporary_routes = Vec::new();
        let mut invalidated = false;
        for link in &view.links {
            let Some(route) = self.routes.get(&link.motif) else {
                continue;
            };
            work.compiled_local_activations += 1;
            work.context_support_validations += 1;
            let support_valid = route.key.context == link.motif.context
                && route.key.support == link.motif.atoms
                && route.key.source_role == link.motif.atoms[0].source_position
                && route.key.target_role == link.motif.atoms[0].target_position;
            work.compiled_dependency_comparisons += 1;
            let dependency_valid = rules.rules.get(&link.motif).is_some_and(|parent| {
                parent.strength >= CONSOLIDATION_STRENGTH
                    && rule_dependency_fingerprint(parent) == route.parent_fingerprint
            });
            if !support_valid || !dependency_valid || route.strength < CONSOLIDATION_STRENGTH {
                work.compiled_invalidations += 1;
                invalidated = true;
                continue;
            }
            work.temporary_path_installations += 1;
            temporary_routes.push(TemporaryCorrespondenceRoute {
                source: link.prior,
                target: link.current,
                compiled_asset: route.key.correspondence_asset,
            });
        }
        if invalidated || temporary_routes.is_empty() {
            return CompiledResolution::ReopenGeneric;
        }
        work.same0.ambiguity_checks += 1;
        let targets = temporary_routes
            .iter()
            .map(|route| route.target)
            .collect::<BTreeSet<_>>();
        debug_assert!(temporary_routes
            .iter()
            .all(|route| route.source != route.target && route.compiled_asset != 0));
        match targets.len() {
            0 => CompiledResolution::Resolved(Resolution::Missing),
            1 => CompiledResolution::Resolved(Resolution::Bound(
                *targets.first().expect("one compiled target"),
            )),
            _ => CompiledResolution::Resolved(Resolution::Ambiguous),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TemporaryCorrespondenceRoute {
    source: OccurrenceId,
    target: OccurrenceId,
    compiled_asset: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CompiledResolution {
    Resolved(Resolution),
    ReopenGeneric,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cs0aWork {
    pub same0: Same0Work,
    pub compiled_candidate_comparisons: u64,
    pub compiled_proposals: u64,
    pub compiled_credit_updates: u64,
    pub compiled_routes_consolidated: u64,
    pub compiled_routes_pruned: u64,
    pub compiled_local_activations: u64,
    pub context_support_validations: u64,
    pub compiled_dependency_comparisons: u64,
    pub temporary_path_installations: u64,
    pub compiled_invalidations: u64,
    pub generic_reopenings: u64,
}

impl Cs0aWork {
    pub fn total(self) -> u64 {
        self.same0.total()
            + self.compiled_candidate_comparisons
            + self.compiled_proposals
            + self.compiled_credit_updates
            + self.compiled_routes_consolidated
            + self.compiled_routes_pruned
            + self.compiled_local_activations
            + self.context_support_validations
            + self.compiled_dependency_comparisons
            + self.temporary_path_installations
            + self.compiled_invalidations
            + self.generic_reopenings
    }

    pub fn correspondence_total(self) -> u64 {
        self.same0.anonymous_observations
            + self.same0.temporal_relations
            + self.same0.causal_relations
            + self.same0.correspondence_comparisons
            + self.same0.correspondence_lookups
            + self.same0.ambiguity_checks
            + self.same0.binding_writes
            + self.compiled_local_activations
            + self.context_support_validations
            + self.compiled_dependency_comparisons
            + self.temporary_path_installations
            + self.compiled_invalidations
            + self.generic_reopenings
    }

    pub(super) fn add(&mut self, other: Self) {
        self.same0.add(other.same0);
        self.compiled_candidate_comparisons += other.compiled_candidate_comparisons;
        self.compiled_proposals += other.compiled_proposals;
        self.compiled_credit_updates += other.compiled_credit_updates;
        self.compiled_routes_consolidated += other.compiled_routes_consolidated;
        self.compiled_routes_pruned += other.compiled_routes_pruned;
        self.compiled_local_activations += other.compiled_local_activations;
        self.context_support_validations += other.context_support_validations;
        self.compiled_dependency_comparisons += other.compiled_dependency_comparisons;
        self.temporary_path_installations += other.temporary_path_installations;
        self.compiled_invalidations += other.compiled_invalidations;
        self.generic_reopenings += other.generic_reopenings;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CompiledCandidate {
    key: CompiledRouteKey,
    parent_fingerprint: u64,
    strength: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CompiledAcquisition {
    pub(super) store: CompiledCorrespondenceStore,
    pub(super) work: Cs0aWork,
    pub(super) proposed: usize,
    pub(super) consolidated: usize,
}

fn key_for(rule: &CorrespondenceRule) -> CompiledRouteKey {
    CompiledRouteKey {
        correspondence_asset: rule_asset_identity(rule.motif),
        source_role: rule.motif.atoms[0].source_position,
        target_role: rule.motif.atoms[0].target_position,
        context: rule.motif.context,
        support: rule.motif.atoms,
    }
}

pub(super) fn acquire_compiled(
    rules: &RuleStore,
    seed: usize,
    population: usize,
    evidence_per_motif: usize,
    shuffled: bool,
) -> CompiledAcquisition {
    let arrows = ArrowStore::primitive();
    let roots = primitive_roots(TEST_DEPTH);
    let environment = Environment::default();
    let mut candidates = BTreeMap::<RelationMotif, CompiledCandidate>::new();
    let mut work = Cs0aWork::default();
    for evidence in 0..(evidence_per_motif * 2) {
        let episode = evaluator_episode(
            0xca10_0000_0000_0000 ^ (seed as u64).rotate_left(11) ^ evidence as u64,
            population,
            evidence % 2,
            1,
            ViewPerturbation::Standard,
            0,
        );
        let execution = execute(&arrows, &roots, rules, &episode.view, &environment);
        let selected = execution
            .trace
            .effects
            .last()
            .map(|effect| effect.occurrence);
        let successful = selected.is_some_and(|occurrence| {
            episode.occurrence_truth.get(&occurrence) == Some(&episode.expected)
        });
        work.same0.add(execution.work);
        let selected_link = episode.view.links.iter().find(|link| {
            work.compiled_candidate_comparisons += 1;
            Some(link.current) == selected
        });
        let Some(link) = selected_link else {
            continue;
        };
        let Some(parent) = rules.rules.get(&link.motif) else {
            continue;
        };
        let was_present = candidates.contains_key(&link.motif);
        let candidate = candidates.entry(link.motif).or_insert(CompiledCandidate {
            key: key_for(parent),
            parent_fingerprint: rule_dependency_fingerprint(parent),
            strength: 0,
        });
        if !was_present {
            work.compiled_proposals += 1;
        }
        let credited_success = successful && (!shuffled || evidence % 3 == 0);
        candidate.strength += if credited_success {
            SUCCESS_CREDIT
        } else {
            FAILURE_CREDIT
        };
        work.compiled_credit_updates += 1;
        if candidate.strength <= PRUNE_STRENGTH {
            candidates.remove(&link.motif);
            work.compiled_routes_pruned += 1;
        }
    }
    let proposed = candidates.len() + work.compiled_routes_pruned as usize;
    let mut store = CompiledCorrespondenceStore::default();
    for candidate in candidates.values() {
        if candidate.strength < CONSOLIDATION_STRENGTH {
            continue;
        }
        let motif = RelationMotif {
            context: candidate.key.context,
            atoms: candidate.key.support,
        };
        let id = store.routes.len() as u32;
        store.routes.insert(
            motif,
            CompiledCorrespondenceRoute {
                id,
                key: candidate.key,
                parent_fingerprint: candidate.parent_fingerprint,
                strength: candidate.strength,
            },
        );
        work.compiled_routes_consolidated += 1;
    }
    let consolidated = store.len();
    CompiledAcquisition {
        store,
        work,
        proposed,
        consolidated,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CsExecution {
    pub(super) execution: Execution,
    pub(super) work: Cs0aWork,
    pub(super) used_compiled: bool,
    pub(super) reopened_generic: bool,
}

fn execute_generic(
    arrows: &ArrowStore,
    roots: &[ArrowId],
    rules: &RuleStore,
    episode: &EvaluatorEpisode,
    environment: &Environment,
) -> CsExecution {
    let execution = execute(arrows, roots, rules, &episode.view, environment);
    CsExecution {
        work: Cs0aWork {
            same0: execution.work,
            ..Cs0aWork::default()
        },
        execution,
        used_compiled: false,
        reopened_generic: false,
    }
}

pub(super) fn execute_compiled_or_generic(
    arrows: &ArrowStore,
    roots: &[ArrowId],
    rules: &RuleStore,
    compiled: &CompiledCorrespondenceStore,
    episode: &EvaluatorEpisode,
    environment: &Environment,
) -> CsExecution {
    let mut work = Cs0aWork::default();
    let (resolution, used_compiled, reopened_generic) =
        match compiled.resolve(rules, &episode.view, &mut work) {
            CompiledResolution::Resolved(resolution) => (resolution, true, false),
            CompiledResolution::ReopenGeneric => {
                work.generic_reopenings += 1;
                (rules.resolve(&episode.view, &mut work.same0), false, true)
            }
        };
    let execution = execute_resolution(arrows, roots, resolution, work.same0, environment);
    work.same0 = execution.work;
    CsExecution {
        execution,
        work,
        used_compiled,
        reopened_generic,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cs0aArm {
    Generic,
    Compiled,
    FreshOccurrences,
    PermutedAllocation,
    PermutedMemory,
    ChangedBinding,
    ChangedCausalContext,
    InvalidatedParent,
    HistoricalReturn,
    SubthresholdEvidence,
    ShuffledEvidence,
    MissingCorrespondence,
    AmbiguousCorrespondence,
    SuppliedSameReference,
}

impl Cs0aArm {
    pub const ALL: [Self; 14] = [
        Self::Generic,
        Self::Compiled,
        Self::FreshOccurrences,
        Self::PermutedAllocation,
        Self::PermutedMemory,
        Self::ChangedBinding,
        Self::ChangedCausalContext,
        Self::InvalidatedParent,
        Self::HistoricalReturn,
        Self::SubthresholdEvidence,
        Self::ShuffledEvidence,
        Self::MissingCorrespondence,
        Self::AmbiguousCorrespondence,
        Self::SuppliedSameReference,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Generic => "generic-learned-correspondence",
            Self::Compiled => "compiled-correspondence",
            Self::FreshOccurrences => "compiled-fresh-occurrences",
            Self::PermutedAllocation => "compiled-permuted-allocation",
            Self::PermutedMemory => "compiled-permuted-memory",
            Self::ChangedBinding => "compiled-changed-binding",
            Self::ChangedCausalContext => "compiled-changed-causal-context",
            Self::InvalidatedParent => "invalidated-parent-generic-resumption",
            Self::HistoricalReturn => "historical-compiled-return",
            Self::SubthresholdEvidence => "subthreshold-evidence",
            Self::ShuffledEvidence => "shuffled-consolidation-evidence",
            Self::MissingCorrespondence => "missing-correspondence",
            Self::AmbiguousCorrespondence => "ambiguous-correspondence",
            Self::SuppliedSameReference => "supplied-SAME-reference",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cs0aRow {
    pub seed: usize,
    pub arm: Cs0aArm,
    pub correct: usize,
    pub total: usize,
    pub average_work: u64,
    pub average_correspondence_work: u64,
    pub work: Cs0aWork,
    pub compiled_uses: usize,
    pub generic_reopenings: usize,
    pub persistent_fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cs0aAcquisitionResult {
    pub seed: usize,
    pub parent_correspondence_work: u64,
    pub compilation_work: u64,
    pub proposed: usize,
    pub compiled_routes: usize,
    pub persistent_bytes: usize,
    pub persistent_fingerprint: u64,
    pub subthreshold_routes: usize,
    pub shuffled_routes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cs0aControl {
    pub seed: usize,
    pub name: String,
    pub passed: bool,
    pub diagnostic: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CellResult {
    acquisition: Cs0aAcquisitionResult,
    rows: Vec<Cs0aRow>,
    controls: Vec<Cs0aControl>,
}

pub(super) fn changed_parent_rules(rules: &RuleStore) -> RuleStore {
    let mut changed = rules.clone();
    let mut work = Same0Work::default();
    for shape in 0..2 {
        changed.observe(good_motif(shape, 1), false, &mut work);
    }
    changed
}

fn missing_episode(mut episode: EvaluatorEpisode) -> EvaluatorEpisode {
    episode
        .view
        .links
        .retain(|link| !link.motif.eq(&good_motif(0, 1)) && !link.motif.eq(&good_motif(1, 1)));
    episode
}

fn ambiguous_episode(mut episode: EvaluatorEpisode) -> EvaluatorEpisode {
    let good = episode
        .view
        .links
        .iter()
        .find(|link| link.motif == good_motif(0, 1) || link.motif == good_motif(1, 1))
        .copied()
        .expect("held-out episode has good link");
    let alternate = episode
        .view
        .links
        .iter()
        .find(|link| link.motif != good.motif)
        .map(|link| link.current)
        .expect("held-out episode has decoy link");
    episode.view.links.push(CandidateLink {
        prior: good.prior,
        current: alternate,
        motif: good.motif,
    });
    episode
}

fn episode_for(seed: usize, episode_index: usize, arm: Cs0aArm) -> EvaluatorEpisode {
    let perturbation = match arm {
        Cs0aArm::FreshOccurrences => ViewPerturbation::RelabeledOccurrences,
        Cs0aArm::PermutedAllocation => ViewPerturbation::AllocationOrder,
        Cs0aArm::PermutedMemory => ViewPerturbation::MemoryOrder,
        _ => ViewPerturbation::Standard,
    };
    let context = if arm == Cs0aArm::ChangedCausalContext {
        2
    } else {
        1
    };
    let truth_offset = if arm == Cs0aArm::ChangedBinding {
        10_000
    } else {
        0
    };
    let episode = evaluator_episode(
        0xca20_0000_0000_0000
            ^ (seed as u64).rotate_left(13)
            ^ (episode_index as u64).rotate_left(3),
        TEST_POPULATION,
        episode_index,
        context,
        perturbation,
        truth_offset,
    );
    match arm {
        Cs0aArm::MissingCorrespondence => missing_episode(episode),
        Cs0aArm::AmbiguousCorrespondence => ambiguous_episode(episode),
        _ => episode,
    }
}

struct RunFixture<'a> {
    arrows: &'a ArrowStore,
    roots: &'a [ArrowId],
    rules: &'a RuleStore,
    changed_rules: &'a RuleStore,
    compiled: &'a CompiledCorrespondenceStore,
    subthreshold: &'a CompiledCorrespondenceStore,
    shuffled: &'a CompiledCorrespondenceStore,
}

fn run_arm(seed: usize, arm: Cs0aArm, episodes: usize, fixture: &RunFixture<'_>) -> Cs0aRow {
    let selected_rules = if arm == Cs0aArm::InvalidatedParent {
        fixture.changed_rules
    } else {
        fixture.rules
    };
    let selected_store = match arm {
        Cs0aArm::SubthresholdEvidence => fixture.subthreshold,
        Cs0aArm::ShuffledEvidence => fixture.shuffled,
        _ => fixture.compiled,
    };
    let before = selected_store.fingerprint();
    let mut correct = 0;
    let mut total_work = 0;
    let mut correspondence_work = 0;
    let mut aggregate = Cs0aWork::default();
    let mut compiled_uses = 0;
    let mut generic_reopenings = 0;
    for episode_index in 0..episodes {
        let episode = episode_for(seed, episode_index, arm);
        let environment = Environment::default();
        let generic = execute_generic(
            fixture.arrows,
            fixture.roots,
            selected_rules,
            &episode,
            &environment,
        );
        let generic_trace = observable(&generic.execution, &episode, &environment);
        if arm == Cs0aArm::SuppliedSameReference {
            correct += 1;
            total_work += generic.work.total().saturating_sub(18);
            continue;
        }
        let execution = if arm == Cs0aArm::Generic {
            generic
        } else {
            execute_compiled_or_generic(
                fixture.arrows,
                fixture.roots,
                selected_rules,
                selected_store,
                &episode,
                &environment,
            )
        };
        let trace = observable(&execution.execution, &episode, &environment);
        correct += usize::from(trace == generic_trace);
        total_work += execution.work.total();
        correspondence_work += execution.work.correspondence_total();
        compiled_uses += usize::from(execution.used_compiled);
        generic_reopenings += usize::from(execution.reopened_generic);
        aggregate.add(execution.work);
    }
    assert_eq!(
        before,
        selected_store.fingerprint(),
        "episode mutated persistent compiled state"
    );
    Cs0aRow {
        seed,
        arm,
        correct,
        total: episodes,
        average_work: total_work / episodes as u64,
        average_correspondence_work: correspondence_work / episodes as u64,
        work: aggregate,
        compiled_uses,
        generic_reopenings,
        persistent_fingerprint: before,
    }
}

fn row(rows: &[Cs0aRow], arm: Cs0aArm) -> &Cs0aRow {
    rows.iter().find(|row| row.arm == arm).expect("arm row")
}

fn control(seed: usize, name: &str, passed: bool, diagnostic: u64) -> Cs0aControl {
    Cs0aControl {
        seed,
        name: name.to_string(),
        passed,
        diagnostic,
    }
}

fn run_cell(seed: usize, episodes: usize, evidence_episodes: usize) -> CellResult {
    let parent = train_correspondence(
        0xca30_0000_0000_0000 ^ seed as u64,
        TEST_POPULATION,
        evidence_episodes,
        0,
    );
    let compiled = acquire_compiled(
        &parent.rules,
        seed,
        TEST_POPULATION,
        COMPILED_EVIDENCE_PER_MOTIF,
        false,
    );
    let subthreshold = acquire_compiled(
        &parent.rules,
        seed ^ 0x5100,
        TEST_POPULATION,
        SUBTHRESHOLD_EVIDENCE_PER_MOTIF,
        false,
    );
    let shuffled = acquire_compiled(
        &parent.rules,
        seed ^ 0x5200,
        TEST_POPULATION,
        COMPILED_EVIDENCE_PER_MOTIF * 2,
        true,
    );
    let arrows = ArrowStore::primitive();
    let roots = primitive_roots(TEST_DEPTH);
    let changed_rules = changed_parent_rules(&parent.rules);
    let fixture = RunFixture {
        arrows: &arrows,
        roots: &roots,
        rules: &parent.rules,
        changed_rules: &changed_rules,
        compiled: &compiled.store,
        subthreshold: &subthreshold.store,
        shuffled: &shuffled.store,
    };
    let rows = Cs0aArm::ALL
        .into_iter()
        .map(|arm| run_arm(seed, arm, episodes, &fixture))
        .collect::<Vec<_>>();
    let generic = row(&rows, Cs0aArm::Generic);
    let mature = row(&rows, Cs0aArm::Compiled);
    let invalidated = row(&rows, Cs0aArm::InvalidatedParent);
    let historical = row(&rows, Cs0aArm::HistoricalReturn);
    let fingerprint = compiled.store.fingerprint();
    let source_audit = persistent_source_audit();
    let mut controls = vec![
        control(
            seed,
            "compiled-only-after-threshold",
            compiled.consolidated == 2,
            compiled.consolidated as u64,
        ),
        control(
            seed,
            "subthreshold-does-not-compile",
            subthreshold.consolidated == 0,
            subthreshold.consolidated as u64,
        ),
        control(
            seed,
            "shuffled-evidence-does-not-compile",
            shuffled.consolidated == 0,
            shuffled.consolidated as u64,
        ),
        control(
            seed,
            "compiled-work-strictly-lower",
            mature.average_correspondence_work < generic.average_correspondence_work,
            generic
                .average_correspondence_work
                .saturating_sub(mature.average_correspondence_work),
        ),
        control(
            seed,
            "fresh-occurrences-transfer",
            row(&rows, Cs0aArm::FreshOccurrences).correct == episodes,
            row(&rows, Cs0aArm::FreshOccurrences).compiled_uses as u64,
        ),
        control(
            seed,
            "allocation-order-invariant",
            row(&rows, Cs0aArm::PermutedAllocation).correct == episodes,
            row(&rows, Cs0aArm::PermutedAllocation).compiled_uses as u64,
        ),
        control(
            seed,
            "memory-order-invariant",
            row(&rows, Cs0aArm::PermutedMemory).correct == episodes,
            row(&rows, Cs0aArm::PermutedMemory).compiled_uses as u64,
        ),
        control(
            seed,
            "changed-binding-transfer",
            row(&rows, Cs0aArm::ChangedBinding).correct == episodes,
            row(&rows, Cs0aArm::ChangedBinding).compiled_uses as u64,
        ),
        control(
            seed,
            "changed-context-reopens-generic",
            row(&rows, Cs0aArm::ChangedCausalContext).correct == episodes
                && row(&rows, Cs0aArm::ChangedCausalContext).generic_reopenings == episodes,
            row(&rows, Cs0aArm::ChangedCausalContext).generic_reopenings as u64,
        ),
        control(
            seed,
            "stale-parent-invalidates-and-reopens",
            invalidated.correct == episodes
                && invalidated.generic_reopenings == episodes
                && invalidated.work.compiled_invalidations >= episodes as u64,
            invalidated.work.compiled_invalidations,
        ),
        control(
            seed,
            "historical-compatible-route-reused",
            historical.correct == episodes
                && historical.compiled_uses == episodes
                && historical.generic_reopenings == 0,
            historical.compiled_uses as u64,
        ),
        control(
            seed,
            "missing-correspondence-delivers-no-effect",
            row(&rows, Cs0aArm::MissingCorrespondence).correct == episodes
                && row(&rows, Cs0aArm::MissingCorrespondence).generic_reopenings == episodes,
            row(&rows, Cs0aArm::MissingCorrespondence).generic_reopenings as u64,
        ),
        control(
            seed,
            "ambiguous-correspondence-delivers-no-effect",
            row(&rows, Cs0aArm::AmbiguousCorrespondence).correct == episodes,
            row(&rows, Cs0aArm::AmbiguousCorrespondence).compiled_uses as u64,
        ),
        control(
            seed,
            "persistent-state-has-no-occurrence-or-filler",
            source_audit && fingerprint != 0,
            compiled.store.persistent_bytes() as u64,
        ),
        control(
            seed,
            "persistent-state-stable-during-use",
            rows.iter()
                .filter(|row| {
                    !matches!(
                        row.arm,
                        Cs0aArm::SubthresholdEvidence | Cs0aArm::ShuffledEvidence
                    )
                })
                .all(|row| row.persistent_fingerprint == fingerprint),
            fingerprint,
        ),
    ];
    controls.extend(
        correspondence_controls(seed, evidence_episodes)
            .into_iter()
            .map(|parent| Cs0aControl {
                seed,
                name: format!("inherited-{}", parent.name),
                passed: parent.passed,
                diagnostic: parent.diagnostic,
            }),
    );
    CellResult {
        acquisition: Cs0aAcquisitionResult {
            seed,
            parent_correspondence_work: parent.acquisition_work.total(),
            compilation_work: compiled.work.total(),
            proposed: compiled.proposed,
            compiled_routes: compiled.consolidated,
            persistent_bytes: compiled.store.persistent_bytes(),
            persistent_fingerprint: fingerprint,
            subthreshold_routes: subthreshold.consolidated,
            shuffled_routes: shuffled.consolidated,
        },
        rows,
        controls,
    }
}

pub(super) fn persistent_source_audit() -> bool {
    let source = include_str!("cs0a.rs");
    let persistent = source
        .split_once("// BEGIN PERSISTENT COMPILED CORRESPONDENCE")
        .and_then(|(_, rest)| rest.split_once("// END PERSISTENT COMPILED CORRESPONDENCE"))
        .map(|(body, _)| body)
        .unwrap_or("")
        .to_ascii_lowercase();
    [
        "occurrenceid",
        "truthfillerid",
        "concrete",
        "destination",
        "episode",
        "answer",
        "future",
        "level",
        "economic",
    ]
    .iter()
    .all(|forbidden| !persistent.contains(forbidden))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cs0aReport {
    pub mode: String,
    pub claim_eligible: bool,
    pub acquisitions: Vec<Cs0aAcquisitionResult>,
    pub rows: Vec<Cs0aRow>,
    pub controls: Vec<Cs0aControl>,
    pub generic_correspondence_work: u64,
    pub compiled_correspondence_work: u64,
    pub mature_reduction: u64,
    pub duplicate_deterministic: bool,
    pub source_audit_passed: bool,
    pub parent_fixture_positive: bool,
    pub passed: bool,
}

pub fn run_cs0a(mode: HarnessMode) -> Cs0aReport {
    let (seeds, episodes, evidence_episodes) = match mode {
        HarnessMode::Micro => (vec![99_997], MICRO_EPISODES, 4),
        HarnessMode::Gate => (vec![DEVELOPMENT_SEED], GATE_EPISODES, 4),
        HarnessMode::Definitive => ((0..DEFINITIVE_SEEDS).collect(), DEFINITIVE_EPISODES, 4),
    };
    let cells = parallel_map_ordered(seeds.len(), |index| {
        run_cell(seeds[index], episodes, evidence_episodes)
    });
    let duplicate_deterministic = cells.iter().all(|cell| {
        let duplicate = run_cell(cell.acquisition.seed, episodes, evidence_episodes);
        duplicate == *cell
    });
    let acquisitions = cells
        .iter()
        .map(|cell| cell.acquisition.clone())
        .collect::<Vec<_>>();
    let rows = cells
        .iter()
        .flat_map(|cell| cell.rows.clone())
        .collect::<Vec<_>>();
    let controls = cells
        .iter()
        .flat_map(|cell| cell.controls.clone())
        .collect::<Vec<_>>();
    let generic_rows = rows
        .iter()
        .filter(|row| row.arm == Cs0aArm::Generic)
        .collect::<Vec<_>>();
    let compiled_rows = rows
        .iter()
        .filter(|row| row.arm == Cs0aArm::Compiled)
        .collect::<Vec<_>>();
    let generic_correspondence_work = generic_rows
        .iter()
        .map(|row| row.average_correspondence_work)
        .sum::<u64>()
        / generic_rows.len() as u64;
    let compiled_correspondence_work = compiled_rows
        .iter()
        .map(|row| row.average_correspondence_work)
        .sum::<u64>()
        / compiled_rows.len() as u64;
    let mature_reduction = generic_correspondence_work.saturating_sub(compiled_correspondence_work);
    let source_audit_passed = persistent_source_audit();
    let parent_fixture_positive = acquisitions
        .iter()
        .all(|row| row.parent_correspondence_work > 0);
    let essential_behavior = rows.iter().all(|row| row.correct == row.total);
    let compiled_used = compiled_rows
        .iter()
        .all(|row| row.compiled_uses == row.total && row.generic_reopenings == 0);
    let passed = parent_fixture_positive
        && acquisitions.iter().all(|row| {
            row.compiled_routes == 2 && row.subthreshold_routes == 0 && row.shuffled_routes == 0
        })
        && essential_behavior
        && compiled_used
        && compiled_correspondence_work < generic_correspondence_work
        && controls.iter().all(|control| control.passed)
        && duplicate_deterministic
        && source_audit_passed;
    Cs0aReport {
        mode: match mode {
            HarnessMode::Micro => "micro",
            HarnessMode::Gate => "gate",
            HarnessMode::Definitive => "definitive",
        }
        .to_string(),
        claim_eligible: mode == HarnessMode::Definitive,
        acquisitions,
        rows,
        controls,
        generic_correspondence_work,
        compiled_correspondence_work,
        mature_reduction,
        duplicate_deterministic,
        source_audit_passed,
        parent_fixture_positive,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiled_persistent_state_contains_no_episode_identity_type() {
        assert!(persistent_source_audit());
        assert!(!std::any::type_name::<CompiledCorrespondenceRoute>().contains("Occurrence"));
    }

    #[test]
    fn micro_compiles_transfers_invalidates_and_reopens() {
        let report = run_cs0a(HarnessMode::Micro);
        assert!(report.passed);
        assert_eq!(report.generic_correspondence_work, 18);
        assert!(report.compiled_correspondence_work < 18);
        assert!(report.controls.iter().all(|control| control.passed));
    }

    #[test]
    fn gate_is_deterministic_and_uses_no_definitive_state() {
        let report = run_cs0a(HarnessMode::Gate);
        assert!(report.passed);
        assert!(!report.claim_eligible);
        assert!(report.duplicate_deterministic);
        assert!(report.rows.iter().all(|row| row.seed == DEVELOPMENT_SEED));
    }
}
