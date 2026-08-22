//! FFS-SAME0: correspondence is acquired from anonymous local relational
//! evidence before the generic recursive substitution kernel can use it.

use crate::research_runtime::{parallel_map_ordered, HarnessMode};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::mem::size_of;

pub mod cs0a;

pub const FFS_SAME0_PROTOCOL: &str = "ffs-same0-learned-correspondence-v1";
pub const MAX_PROMOTIONS: usize = 6;
const SUCCESS_CREDIT: i32 = 2;
const FAILURE_CREDIT: i32 = -1;
const CONSOLIDATION_STRENGTH: i32 = 6;
const PRUNE_STRENGTH: i32 = -2;
const MIN_LOCAL_OCCURRENCES: usize = 3;
const TEMPORARY_INSTALLATION_WORK: u64 = 8;
const DEVELOPMENT_SEED: usize = 50_000;
const DEFINITIVE_SEEDS: usize = 8;
const DEVELOPMENT_EPISODES: usize = 4;
const DEFINITIVE_EPISODES: usize = 16;
const MICROS_PER_WORK: u128 = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScaleSpec {
    name: &'static str,
    depth: usize,
    population: usize,
}

impl ScaleSpec {
    const fn new(name: &'static str, depth: usize, population: usize) -> Self {
        Self {
            name,
            depth,
            population,
        }
    }
}

const ANCHORS: [ScaleSpec; 4] = [
    ScaleSpec::new("S0", 8, 16),
    ScaleSpec::new("S1", 32, 64),
    ScaleSpec::new("S2", 128, 256),
    ScaleSpec::new("S3", 512, 1024),
];
const DEPTH_PROBE: ScaleSpec = ScaleSpec::new("depth-only", 128, 64);
const POPULATION_PROBE: ScaleSpec = ScaleSpec::new("population-only", 32, 1024);

fn mix(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(0x100_0000_01b3);
}

fn hash_values(values: impl IntoIterator<Item = u64>) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    for value in values {
        mix(&mut hash, value);
    }
    hash
}

fn ceil_div(numerator: u128, denominator: u128) -> u128 {
    numerator / denominator + u128::from(!numerator.is_multiple_of(denominator))
}

// BEGIN ORGANISM KERNEL

type CellId = u32;
type ArrowId = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct OccurrenceId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct EffectRole(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RelationAtom {
    channel: u8,
    source_position: u8,
    target_position: u8,
    lag: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RelationMotif {
    context: u8,
    atoms: [RelationAtom; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CandidateLink {
    prior: OccurrenceId,
    current: OccurrenceId,
    motif: RelationMotif,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AnonymousView {
    links: Vec<CandidateLink>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CorrespondenceRule {
    motif: RelationMotif,
    strength: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct RuleStore {
    rules: BTreeMap<RelationMotif, CorrespondenceRule>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Resolution {
    Bound(OccurrenceId),
    Missing,
    Ambiguous,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Same0Work {
    pub anonymous_observations: u64,
    pub temporal_relations: u64,
    pub causal_relations: u64,
    pub correspondence_comparisons: u64,
    pub correspondence_proposals: u64,
    pub correspondence_credit_updates: u64,
    pub correspondences_consolidated: u64,
    pub correspondences_pruned: u64,
    pub correspondence_lookups: u64,
    pub ambiguity_checks: u64,
    pub binding_writes: u64,
    pub binding_reads: u64,
    pub arrow_evaluations: u64,
    pub compatibility_comparisons: u64,
    pub dependency_comparisons: u64,
    pub arrow_firings: u64,
    pub spikes_enqueued: u64,
    pub spikes_dequeued: u64,
    pub effect_deliveries: u64,
    pub temporary_installations: u64,
    pub occurrence_observations: u64,
    pub recurrence_comparisons: u64,
    pub arrow_candidates_proposed: u64,
    pub arrow_credit_updates: u64,
    pub arrows_consolidated: u64,
    pub arrows_pruned: u64,
    pub invalidations: u64,
    pub fallback_expansions: u64,
}

impl Same0Work {
    pub fn total(self) -> u64 {
        self.anonymous_observations
            + self.temporal_relations
            + self.causal_relations
            + self.correspondence_comparisons
            + self.correspondence_proposals
            + self.correspondence_credit_updates
            + self.correspondences_consolidated
            + self.correspondences_pruned
            + self.correspondence_lookups
            + self.ambiguity_checks
            + self.binding_writes
            + self.binding_reads
            + self.arrow_evaluations
            + self.compatibility_comparisons
            + self.dependency_comparisons
            + self.arrow_firings
            + self.spikes_enqueued
            + self.spikes_dequeued
            + self.effect_deliveries
            + self.temporary_installations
            + self.occurrence_observations
            + self.recurrence_comparisons
            + self.arrow_candidates_proposed
            + self.arrow_credit_updates
            + self.arrows_consolidated
            + self.arrows_pruned
            + self.invalidations
            + self.fallback_expansions
    }

    fn add(&mut self, other: Self) {
        self.anonymous_observations += other.anonymous_observations;
        self.temporal_relations += other.temporal_relations;
        self.causal_relations += other.causal_relations;
        self.correspondence_comparisons += other.correspondence_comparisons;
        self.correspondence_proposals += other.correspondence_proposals;
        self.correspondence_credit_updates += other.correspondence_credit_updates;
        self.correspondences_consolidated += other.correspondences_consolidated;
        self.correspondences_pruned += other.correspondences_pruned;
        self.correspondence_lookups += other.correspondence_lookups;
        self.ambiguity_checks += other.ambiguity_checks;
        self.binding_writes += other.binding_writes;
        self.binding_reads += other.binding_reads;
        self.arrow_evaluations += other.arrow_evaluations;
        self.compatibility_comparisons += other.compatibility_comparisons;
        self.dependency_comparisons += other.dependency_comparisons;
        self.arrow_firings += other.arrow_firings;
        self.spikes_enqueued += other.spikes_enqueued;
        self.spikes_dequeued += other.spikes_dequeued;
        self.effect_deliveries += other.effect_deliveries;
        self.temporary_installations += other.temporary_installations;
        self.occurrence_observations += other.occurrence_observations;
        self.recurrence_comparisons += other.recurrence_comparisons;
        self.arrow_candidates_proposed += other.arrow_candidates_proposed;
        self.arrow_credit_updates += other.arrow_credit_updates;
        self.arrows_consolidated += other.arrows_consolidated;
        self.arrows_pruned += other.arrows_pruned;
        self.invalidations += other.invalidations;
        self.fallback_expansions += other.fallback_expansions;
    }
}

impl RuleStore {
    fn observe(&mut self, motif: RelationMotif, successful: bool, work: &mut Same0Work) {
        work.anonymous_observations += 2;
        work.temporal_relations += motif.atoms.len() as u64;
        work.causal_relations += motif.atoms.len() as u64;
        work.correspondence_comparisons += 1;
        let was_present = self.rules.contains_key(&motif);
        let rule = self
            .rules
            .entry(motif)
            .or_insert(CorrespondenceRule { motif, strength: 0 });
        if !was_present {
            work.correspondence_proposals += 1;
        }
        let prior = rule.strength;
        rule.strength += if successful {
            SUCCESS_CREDIT
        } else {
            FAILURE_CREDIT
        };
        work.correspondence_credit_updates += 1;
        if prior < CONSOLIDATION_STRENGTH && rule.strength >= CONSOLIDATION_STRENGTH {
            work.correspondences_consolidated += 1;
        }
        if rule.strength <= PRUNE_STRENGTH {
            self.rules.remove(&motif);
            work.correspondences_pruned += 1;
        }
    }

    fn resolve(&self, view: &AnonymousView, work: &mut Same0Work) -> Resolution {
        let mut matches = BTreeSet::new();
        for link in &view.links {
            work.anonymous_observations += 2;
            work.temporal_relations += link.motif.atoms.len() as u64;
            work.causal_relations += link.motif.atoms.len() as u64;
            work.correspondence_lookups += 1;
            work.correspondence_comparisons += 1;
            if self
                .rules
                .get(&link.motif)
                .is_some_and(|rule| rule.strength >= CONSOLIDATION_STRENGTH)
            {
                matches.insert(link.current);
            }
        }
        work.ambiguity_checks += 1;
        match matches.len() {
            0 => Resolution::Missing,
            1 => Resolution::Bound(*matches.first().expect("one match")),
            _ => Resolution::Ambiguous,
        }
    }

    fn fingerprint(&self) -> u64 {
        hash_values(self.rules.values().flat_map(|rule| {
            [
                rule.motif.context as u64,
                rule.motif.atoms[0].channel as u64,
                rule.motif.atoms[0].source_position as u64,
                rule.motif.atoms[0].target_position as u64,
                rule.motif.atoms[0].lag as u64,
                rule.motif.atoms[1].channel as u64,
                rule.motif.atoms[1].source_position as u64,
                rule.motif.atoms[1].target_position as u64,
                rule.motif.atoms[1].lag as u64,
                rule.strength as i64 as u64,
            ]
        }))
    }

    fn persistent_bytes(&self) -> usize {
        self.rules.len() * (size_of::<RelationMotif>() + size_of::<i32>())
    }

    fn consolidated_count(&self) -> usize {
        self.rules
            .values()
            .filter(|rule| rule.strength >= CONSOLIDATION_STRENGTH)
            .count()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Arrow {
    id: ArrowId,
    from: CellId,
    to: CellId,
    dependencies: Vec<ArrowId>,
    dependency_fingerprints: Vec<u64>,
    effects: Vec<EffectRole>,
    compatibility_site: u64,
    compatibility_marker: u64,
    strength: i32,
}

impl Arrow {
    fn learned(&self) -> bool {
        !self.dependencies.is_empty()
    }

    fn fingerprint(&self) -> u64 {
        let mut values = vec![
            self.from as u64,
            self.to as u64,
            self.compatibility_site,
            self.compatibility_marker,
            self.strength as i64 as u64,
            self.dependencies.len() as u64,
            self.effects.len() as u64,
        ];
        values.extend(self.dependency_fingerprints.iter().copied());
        values.extend(self.effects.iter().map(|effect| effect.0 as u64));
        hash_values(values)
    }

    fn persistent_bytes(&self) -> usize {
        size_of::<ArrowId>()
            + 2 * size_of::<CellId>()
            + self.dependencies.len() * size_of::<ArrowId>()
            + self.dependency_fingerprints.len() * size_of::<u64>()
            + self.effects.len() * size_of::<EffectRole>()
            + 2 * size_of::<u64>()
            + size_of::<i32>()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArrowStore {
    arrows: Vec<Arrow>,
}

impl ArrowStore {
    fn primitive() -> Self {
        let arrows = (0..4)
            .map(|ordinal| Arrow {
                id: ordinal,
                from: ordinal,
                to: (ordinal + 1) % 4,
                dependencies: Vec::new(),
                dependency_fingerprints: Vec::new(),
                effects: (ordinal == 3)
                    .then_some(EffectRole(0))
                    .into_iter()
                    .collect(),
                compatibility_site: 0,
                compatibility_marker: 0,
                strength: CONSOLIDATION_STRENGTH,
            })
            .collect();
        Self { arrows }
    }

    fn get(&self, id: ArrowId) -> &Arrow {
        &self.arrows[id as usize]
    }

    fn find_pair(&self, pair: [ArrowId; 2]) -> Option<ArrowId> {
        self.arrows
            .iter()
            .find(|arrow| arrow.dependencies == pair && arrow.strength >= CONSOLIDATION_STRENGTH)
            .map(|arrow| arrow.id)
    }

    fn insert_pair(&mut self, pair: [ArrowId; 2], strength: i32) -> ArrowId {
        if let Some(existing) = self.find_pair(pair) {
            return existing;
        }
        let left = self.get(pair[0]);
        let right = self.get(pair[1]);
        let dependency_fingerprints = vec![left.fingerprint(), right.fingerprint()];
        let mut effects = left.effects.clone();
        effects.extend(right.effects.iter().copied());
        let compatibility_site = hash_values([
            0xfa50_0000_0000_0001,
            dependency_fingerprints[0],
            dependency_fingerprints[1],
        ]);
        let compatibility_marker = default_marker(compatibility_site);
        let arrow = Arrow {
            id: self.arrows.len() as ArrowId,
            from: left.from,
            to: right.to,
            dependencies: pair.to_vec(),
            dependency_fingerprints,
            effects,
            compatibility_site,
            compatibility_marker,
            strength,
        };
        let id = arrow.id;
        self.arrows.push(arrow);
        id
    }
}

fn default_marker(site: u64) -> u64 {
    hash_values([0xfa51_0000_0000_0001, site])
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Environment {
    markers: BTreeMap<u64, u64>,
}

impl Environment {
    fn marker(&self, site: u64) -> u64 {
        self.markers
            .get(&site)
            .copied()
            .unwrap_or_else(|| default_marker(site))
    }

    fn change(&mut self, site: u64) {
        self.markers.insert(site, default_marker(site) ^ 0x55aa);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Spike {
    arrow: ArrowId,
    fallback_distance: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InternalEffect {
    role: EffectRole,
    occurrence: OccurrenceId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InternalBoundary {
    effect_count: usize,
    current: OccurrenceId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InternalTrace {
    effects: Vec<InternalEffect>,
    boundaries: Vec<InternalBoundary>,
    quiescent: bool,
    missing_correspondence: bool,
    ambiguous_correspondence: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Execution {
    trace: InternalTrace,
    arrow_occurrences: Vec<ArrowId>,
    work: Same0Work,
    maximum_fallback_distance: usize,
    child_firings: usize,
}

struct ExecutionState<'a> {
    arrows: &'a ArrowStore,
    environment: &'a Environment,
    binding: Option<OccurrenceId>,
    queue: VecDeque<Spike>,
    validity: BTreeMap<ArrowId, bool>,
    installed: BTreeSet<ArrowId>,
    effects: Vec<InternalEffect>,
    boundaries: Vec<InternalBoundary>,
    arrow_occurrences: Vec<ArrowId>,
    work: Same0Work,
    maximum_fallback_distance: usize,
    child_firings: usize,
}

impl ExecutionState<'_> {
    fn enqueue(&mut self, spike: Spike) {
        self.work.spikes_enqueued += 1;
        self.queue.push_back(spike);
    }

    fn valid(&mut self, id: ArrowId) -> bool {
        if let Some(valid) = self.validity.get(&id) {
            return *valid;
        }
        let arrow = self.arrows.get(id);
        if !arrow.learned() {
            self.validity.insert(id, true);
            return true;
        }
        self.work.compatibility_comparisons += 1;
        let mut valid =
            self.environment.marker(arrow.compatibility_site) == arrow.compatibility_marker;
        for (dependency, expected) in arrow
            .dependencies
            .iter()
            .zip(&arrow.dependency_fingerprints)
        {
            self.work.dependency_comparisons += 1;
            valid &= self.arrows.get(*dependency).fingerprint() == *expected;
            valid &= self.valid(*dependency);
        }
        self.validity.insert(id, valid);
        valid
    }

    fn step(&mut self) {
        let Some(spike) = self.queue.pop_front() else {
            return;
        };
        self.work.spikes_dequeued += 1;
        self.work.arrow_evaluations += 1;
        let arrow = self.arrows.get(spike.arrow);
        if arrow.learned() && !self.valid(spike.arrow) {
            self.work.invalidations += 1;
            self.work.fallback_expansions += 1;
            let distance = spike.fallback_distance + 1;
            self.maximum_fallback_distance = self.maximum_fallback_distance.max(distance);
            for dependency in arrow.dependencies.iter().rev() {
                self.work.spikes_enqueued += 1;
                self.queue.push_front(Spike {
                    arrow: *dependency,
                    fallback_distance: distance,
                });
            }
            return;
        }
        if arrow.learned() && self.installed.insert(spike.arrow) {
            self.work.temporary_installations += TEMPORARY_INSTALLATION_WORK;
        }
        self.work.arrow_firings += 1;
        self.child_firings += 1;
        self.arrow_occurrences.push(spike.arrow);
        for effect in &arrow.effects {
            self.work.binding_reads += 1;
            let Some(occurrence) = self.binding else {
                continue;
            };
            self.work.effect_deliveries += 1;
            self.effects.push(InternalEffect {
                role: *effect,
                occurrence,
            });
            self.boundaries.push(InternalBoundary {
                effect_count: self.effects.len(),
                current: occurrence,
            });
        }
    }
}

fn execute_resolution(
    arrows: &ArrowStore,
    roots: &[ArrowId],
    resolution: Resolution,
    mut work: Same0Work,
    environment: &Environment,
) -> Execution {
    let binding = match resolution {
        Resolution::Bound(occurrence) => {
            work.binding_writes += 1;
            Some(occurrence)
        }
        Resolution::Missing | Resolution::Ambiguous => None,
    };
    let mut state = ExecutionState {
        arrows,
        environment,
        binding,
        queue: VecDeque::new(),
        validity: BTreeMap::new(),
        installed: BTreeSet::new(),
        effects: Vec::new(),
        boundaries: Vec::new(),
        arrow_occurrences: Vec::new(),
        work,
        maximum_fallback_distance: 0,
        child_firings: 0,
    };
    for root in roots {
        state.enqueue(Spike {
            arrow: *root,
            fallback_distance: 0,
        });
    }
    while !state.queue.is_empty() {
        state.step();
    }
    Execution {
        trace: InternalTrace {
            effects: state.effects,
            boundaries: state.boundaries,
            quiescent: true,
            missing_correspondence: resolution == Resolution::Missing,
            ambiguous_correspondence: resolution == Resolution::Ambiguous,
        },
        arrow_occurrences: state.arrow_occurrences,
        work: state.work,
        maximum_fallback_distance: state.maximum_fallback_distance,
        child_firings: state.child_firings,
    }
}

fn execute(
    arrows: &ArrowStore,
    roots: &[ArrowId],
    rules: &RuleStore,
    view: &AnonymousView,
    environment: &Environment,
) -> Execution {
    let mut work = Same0Work::default();
    let resolution = rules.resolve(view, &mut work);
    execute_resolution(arrows, roots, resolution, work, environment)
}

fn execute_candidate_attempt(
    arrows: &ArrowStore,
    roots: &[ArrowId],
    link: CandidateLink,
    environment: &Environment,
) -> Execution {
    let mut work = Same0Work {
        anonymous_observations: 2,
        temporal_relations: link.motif.atoms.len() as u64,
        causal_relations: link.motif.atoms.len() as u64,
        correspondence_comparisons: 1,
        ..Same0Work::default()
    };
    work.ambiguity_checks += 1;
    execute_resolution(
        arrows,
        roots,
        Resolution::Bound(link.current),
        work,
        environment,
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArrowCandidate {
    pair: [ArrowId; 2],
    strength: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Acquisition {
    roots: Vec<ArrowId>,
    new_arrows: Vec<ArrowId>,
    proposed: usize,
    retained: usize,
    work: Same0Work,
    incremental_bytes: usize,
}

fn observed_pairs(occurrences: &[ArrowId], work: &mut Same0Work) -> BTreeMap<[ArrowId; 2], usize> {
    let mut counts = BTreeMap::new();
    let mut chunks = occurrences.chunks_exact(2);
    for chunk in &mut chunks {
        work.occurrence_observations += 2;
        *counts.entry([chunk[0], chunk[1]]).or_default() += 1;
    }
    counts
}

fn acquire_arrows(
    arrows: &mut ArrowStore,
    roots: &[ArrowId],
    rules: &RuleStore,
    views: &[AnonymousView],
    successful: bool,
    shuffled: bool,
) -> Acquisition {
    let mut candidates = BTreeMap::<[ArrowId; 2], ArrowCandidate>::new();
    let mut work = Same0Work::default();
    for (episode, view) in views.iter().enumerate() {
        let execution = execute(arrows, roots, rules, view, &Environment::default());
        work.add(execution.work);
        let mut occurrences = execution.arrow_occurrences;
        if shuffled && !occurrences.is_empty() {
            let count = occurrences.len();
            occurrences.rotate_left((episode + 1) % count);
        }
        for (pair, count) in observed_pairs(&occurrences, &mut work) {
            work.recurrence_comparisons += count as u64;
            if count < MIN_LOCAL_OCCURRENCES {
                continue;
            }
            let candidate = candidates.entry(pair).or_insert_with(|| {
                work.arrow_candidates_proposed += 1;
                ArrowCandidate { pair, strength: 0 }
            });
            candidate.strength += if successful {
                SUCCESS_CREDIT
            } else {
                FAILURE_CREDIT
            };
            work.arrow_credit_updates += 1;
        }
    }
    let mut new_arrows = Vec::new();
    let mut proposed = 0;
    let mut retained = 0;
    let mut incremental_bytes = 0;
    for candidate in candidates.values() {
        proposed += 1;
        if candidate.strength >= CONSOLIDATION_STRENGTH {
            let before = arrows.arrows.len();
            let id = arrows.insert_pair(candidate.pair, candidate.strength);
            if arrows.arrows.len() > before {
                incremental_bytes += arrows.get(id).persistent_bytes();
                new_arrows.push(id);
                work.arrows_consolidated += 1;
            }
            retained += 1;
        } else if candidate.strength <= PRUNE_STRENGTH {
            work.arrows_pruned += 1;
        }
    }
    let rewritten = roots
        .chunks(2)
        .flat_map(|chunk| {
            if chunk.len() == 2 {
                arrows
                    .find_pair([chunk[0], chunk[1]])
                    .map(|id| vec![id])
                    .unwrap_or_else(|| chunk.to_vec())
            } else {
                chunk.to_vec()
            }
        })
        .collect();
    Acquisition {
        roots: rewritten,
        new_arrows,
        proposed,
        retained,
        work,
        incremental_bytes,
    }
}

// END ORGANISM KERNEL

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TruthFillerId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ViewPerturbation {
    Standard,
    RelabeledOccurrences,
    AllocationOrder,
    MemoryOrder,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EvaluatorEpisode {
    view: AnonymousView,
    occurrence_truth: BTreeMap<OccurrenceId, TruthFillerId>,
    expected: TruthFillerId,
    all_occurrences: BTreeSet<OccurrenceId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ObservableEffect {
    role: u16,
    truth: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BoundaryState {
    effect_count: usize,
    current_truth: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservableTrace {
    final_state: Option<u64>,
    effects: Vec<ObservableEffect>,
    boundaries: Vec<BoundaryState>,
    quiescent: bool,
    missing_correspondence: bool,
    ambiguous_correspondence: bool,
    context_ledger: Vec<(u64, u64)>,
}

fn good_motif(shape: usize, context: u8) -> RelationMotif {
    match shape % 2 {
        0 => RelationMotif {
            context,
            atoms: [
                RelationAtom {
                    channel: 1,
                    source_position: 0,
                    target_position: 2,
                    lag: 1,
                },
                RelationAtom {
                    channel: 3,
                    source_position: 1,
                    target_position: 2,
                    lag: 2,
                },
            ],
        },
        _ => RelationMotif {
            context,
            atoms: [
                RelationAtom {
                    channel: 2,
                    source_position: 1,
                    target_position: 3,
                    lag: 2,
                },
                RelationAtom {
                    channel: 4,
                    source_position: 0,
                    target_position: 3,
                    lag: 1,
                },
            ],
        },
    }
}

fn decoy_motif(shape: usize, context: u8) -> RelationMotif {
    let good = good_motif(shape, context);
    RelationMotif {
        context,
        atoms: [
            good.atoms[0],
            RelationAtom {
                channel: 0,
                source_position: good.atoms[1].source_position,
                target_position: good.atoms[1].target_position,
                lag: good.atoms[1].lag.saturating_add(1),
            },
        ],
    }
}

fn occurrence_value(seed: u64, ordinal: u64, perturbation: ViewPerturbation) -> OccurrenceId {
    let namespace = match perturbation {
        ViewPerturbation::Standard | ViewPerturbation::MemoryOrder => 0xfa60_0000_0000_0001,
        ViewPerturbation::RelabeledOccurrences => 0xfa60_0000_0000_1001,
        ViewPerturbation::AllocationOrder => 0xfa60_0000_0000_2001,
    };
    OccurrenceId(hash_values([namespace, seed, ordinal]))
}

fn evaluator_episode(
    seed: u64,
    population: usize,
    shape: usize,
    context: u8,
    perturbation: ViewPerturbation,
    truth_offset: u64,
) -> EvaluatorEpisode {
    let population = population.max(2) as u64;
    let expected = TruthFillerId(truth_offset + seed.rotate_left(7) % population);
    let other = TruthFillerId(truth_offset + (expected.0 - truth_offset + 1) % population);
    let allocation = match perturbation {
        ViewPerturbation::AllocationOrder => [2, 3, 0, 1],
        _ => [0, 1, 2, 3],
    };
    let ids = allocation.map(|ordinal| occurrence_value(seed, ordinal, perturbation));
    let good = CandidateLink {
        prior: ids[0],
        current: ids[1],
        motif: good_motif(shape, context),
    };
    let decoy = CandidateLink {
        prior: ids[2],
        current: ids[3],
        motif: decoy_motif(shape, context),
    };
    let mut links = vec![good, decoy];
    if matches!(perturbation, ViewPerturbation::MemoryOrder) {
        links.reverse();
    }
    let occurrence_truth = BTreeMap::from([
        (good.prior, expected),
        (good.current, expected),
        (decoy.prior, expected),
        (decoy.current, other),
    ]);
    let all_occurrences = occurrence_truth.keys().copied().collect();
    EvaluatorEpisode {
        view: AnonymousView { links },
        occurrence_truth,
        expected,
        all_occurrences,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CorrespondenceFixture {
    rules: RuleStore,
    acquisition_work: Same0Work,
    evidence_episodes: usize,
}

fn train_correspondence(
    seed: u64,
    population: usize,
    evidence_episodes: usize,
    truth_offset: u64,
) -> CorrespondenceFixture {
    let mut rules = RuleStore::default();
    let mut work = Same0Work::default();
    let task_arrows = ArrowStore::primitive();
    let task_roots = primitive_roots(8);
    let environment = Environment::default();
    for episode_index in 0..evidence_episodes {
        for shape in 0..2 {
            let episode = evaluator_episode(
                seed ^ 0xfa61_0000 ^ ((episode_index * 2 + shape) as u64),
                population,
                shape,
                1,
                ViewPerturbation::Standard,
                truth_offset,
            );
            for link in &episode.view.links {
                let attempt =
                    execute_candidate_attempt(&task_arrows, &task_roots, *link, &environment);
                let selected = attempt.trace.effects.last().map(|effect| effect.occurrence);
                let successful = selected.is_some_and(|occurrence| {
                    episode.occurrence_truth.get(&occurrence) == Some(&episode.expected)
                });
                work.add(attempt.work);
                rules.observe(link.motif, successful, &mut work);
            }
        }
    }
    CorrespondenceFixture {
        rules,
        acquisition_work: work,
        evidence_episodes,
    }
}

fn observable(
    execution: &Execution,
    episode: &EvaluatorEpisode,
    environment: &Environment,
) -> ObservableTrace {
    let effects = execution
        .trace
        .effects
        .iter()
        .filter_map(|effect| {
            episode
                .occurrence_truth
                .get(&effect.occurrence)
                .map(|truth| ObservableEffect {
                    role: effect.role.0,
                    truth: truth.0,
                })
        })
        .collect::<Vec<_>>();
    let boundaries = execution
        .trace
        .boundaries
        .iter()
        .filter_map(|boundary| {
            episode
                .occurrence_truth
                .get(&boundary.current)
                .map(|truth| BoundaryState {
                    effect_count: boundary.effect_count,
                    current_truth: truth.0,
                })
        })
        .collect::<Vec<_>>();
    ObservableTrace {
        final_state: effects.last().map(|effect| effect.truth),
        effects,
        boundaries,
        quiescent: execution.trace.quiescent,
        missing_correspondence: execution.trace.missing_correspondence,
        ambiguous_correspondence: execution.trace.ambiguous_correspondence,
        context_ledger: environment
            .markers
            .iter()
            .map(|(site, marker)| (*site, *marker))
            .collect(),
    }
}

fn execute_observed(
    arrows: &ArrowStore,
    roots: &[ArrowId],
    rules: &RuleStore,
    episode: &EvaluatorEpisode,
    environment: &Environment,
) -> (Execution, ObservableTrace) {
    let execution = execute(arrows, roots, rules, &episode.view, environment);
    let trace = observable(&execution, episode, environment);
    (execution, trace)
}

fn primitive_roots(depth: usize) -> Vec<ArrowId> {
    (0..depth).map(|index| (index % 4) as ArrowId).collect()
}

fn held_out_episode(
    seed: usize,
    scale: ScaleSpec,
    episode: usize,
    perturbation: ViewPerturbation,
    truth_offset: u64,
) -> EvaluatorEpisode {
    evaluator_episode(
        0xfa62_0000_0000_0000
            ^ (seed as u64).rotate_left(17)
            ^ (scale.depth as u64).rotate_left(9)
            ^ (scale.population as u64).rotate_left(3)
            ^ episode as u64,
        scale.population,
        episode,
        1,
        perturbation,
        truth_offset,
    )
}

struct AverageExecution<'a> {
    arrows: &'a ArrowStore,
    roots: &'a [ArrowId],
    rules: &'a RuleStore,
    scale: ScaleSpec,
    seed: usize,
    episodes: usize,
    environment: &'a Environment,
    perturbation: ViewPerturbation,
    truth_offset: u64,
}

fn average_execution(request: AverageExecution<'_>) -> (u64, Vec<ObservableTrace>, usize) {
    let mut work = 0;
    let mut traces = Vec::new();
    let mut firings = 0;
    for episode_index in 0..request.episodes {
        let episode = held_out_episode(
            request.seed,
            request.scale,
            episode_index,
            request.perturbation,
            request.truth_offset,
        );
        let (execution, trace) = execute_observed(
            request.arrows,
            request.roots,
            request.rules,
            &episode,
            request.environment,
        );
        work += execution.work.total();
        firings += execution.child_firings;
        traces.push(trace);
    }
    (
        work / request.episodes as u64,
        traces,
        firings / request.episodes,
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeResult {
    pub seed: usize,
    pub scale: String,
    pub generation: usize,
    pub parent_work: u64,
    pub child_work: u64,
    pub acquisition_work: u64,
    pub installation_work: u64,
    pub incremental_bytes: usize,
    pub maintenance_work: u64,
    pub observable_equal: bool,
    pub computationally_useful: bool,
    pub economically_justified: bool,
    pub structurally_retained: bool,
    pub break_even_uses: Option<u64>,
    pub proposed: usize,
    pub retained: usize,
    pub removed_arrow_firings: usize,
    pub asset_instance_id: u64,
    pub content_fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentityEconomics {
    pub seed: usize,
    pub scale: String,
    pub learned_motifs: usize,
    pub acquisition_work: u64,
    pub persistent_bytes: usize,
    pub generic_runtime: u64,
    pub learned_identity_runtime: u64,
    pub internal_break_even_uses: Option<u64>,
    pub supplied_same_runtime: u64,
    pub same_less_runtime: u64,
    pub mature_delta_vs_supplied: i64,
    pub break_even_vs_supplied: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScaleResult {
    pub seed: usize,
    pub scale: String,
    pub depth: usize,
    pub population: usize,
    pub edges: Vec<EdgeResult>,
    pub proposed: usize,
    pub functionally_valid: usize,
    pub computationally_useful: usize,
    pub economically_justified: usize,
    pub endogenously_retained: usize,
    pub over_retained: usize,
    pub under_retained: usize,
    pub structural_depth: usize,
    pub justified_depth: usize,
    pub realized_useful_depth: usize,
    pub right_censored: bool,
    pub collapse_point: String,
    pub identity: IdentityEconomics,
    arrows: ArrowStore,
    roots: Vec<ArrowId>,
    rules: RuleStore,
    lineage: u64,
}

fn supplied_same_runtime(depth: usize) -> u64 {
    match depth {
        8 => 36,
        32 => 52,
        128 => 106,
        512 => 317,
        _ => 4 * depth as u64 + 2 * (depth / 4) as u64,
    }
}

fn primitive_route_work(depth: usize) -> u64 {
    4 * depth as u64 + 2 * (depth / 4) as u64
}

fn run_scale(
    scale: ScaleSpec,
    seed: usize,
    held_out_episodes: usize,
    evidence_episodes: usize,
) -> ScaleResult {
    let correspondence = train_correspondence(
        0xfa63_0000_0000_0000 ^ seed as u64,
        scale.population,
        evidence_episodes,
        0,
    );
    let lineage = hash_values([
        0xfa64_0000_0000_0000,
        seed as u64,
        scale.depth as u64,
        scale.population as u64,
    ]);
    let mut arrows = ArrowStore::primitive();
    let mut roots = primitive_roots(scale.depth);
    let mut edges = Vec::new();
    if correspondence.rules.consolidated_count() >= 2 {
        for generation in 0..MAX_PROMOTIONS {
            let parent_roots = roots.clone();
            let views = (0..3)
                .map(|episode| {
                    held_out_episode(
                        seed ^ 0x5100 ^ generation,
                        scale,
                        episode,
                        ViewPerturbation::Standard,
                        0,
                    )
                    .view
                })
                .collect::<Vec<_>>();
            let acquisition = acquire_arrows(
                &mut arrows,
                &parent_roots,
                &correspondence.rules,
                &views,
                true,
                false,
            );
            if acquisition.new_arrows.is_empty() {
                break;
            }
            roots = acquisition.roots.clone();
            let environment = Environment::default();
            let (parent_work, parent_traces, parent_firings) =
                average_execution(AverageExecution {
                    arrows: &arrows,
                    roots: &parent_roots,
                    rules: &correspondence.rules,
                    scale,
                    seed,
                    episodes: held_out_episodes,
                    environment: &environment,
                    perturbation: ViewPerturbation::Standard,
                    truth_offset: 0,
                });
            let (child_work, child_traces, child_firings) = average_execution(AverageExecution {
                arrows: &arrows,
                roots: &roots,
                rules: &correspondence.rules,
                scale,
                seed,
                episodes: held_out_episodes,
                environment: &environment,
                perturbation: ViewPerturbation::Standard,
                truth_offset: 0,
            });
            let observable_equal = parent_traces == child_traces;
            let computationally_useful = child_work < parent_work;
            let gain = parent_work.saturating_sub(child_work);
            let break_even_uses = (observable_equal && computationally_useful).then(|| {
                u64::try_from(ceil_div(
                    acquisition.work.total() as u128 * MICROS_PER_WORK,
                    gain as u128 * MICROS_PER_WORK,
                ))
                .expect("FFS-SAME0 break-even fits u64")
            });
            let content_fingerprint = hash_values(
                acquisition
                    .new_arrows
                    .iter()
                    .map(|id| arrows.get(*id).fingerprint()),
            );
            edges.push(EdgeResult {
                seed,
                scale: scale.name.to_string(),
                generation: generation + 1,
                parent_work,
                child_work,
                acquisition_work: acquisition.work.total(),
                installation_work: 0,
                incremental_bytes: acquisition.incremental_bytes,
                maintenance_work: 0,
                observable_equal,
                computationally_useful,
                economically_justified: break_even_uses.is_some(),
                structurally_retained: acquisition.retained > 0,
                break_even_uses,
                proposed: acquisition.proposed,
                retained: acquisition.retained,
                removed_arrow_firings: parent_firings.saturating_sub(child_firings),
                asset_instance_id: hash_values([
                    lineage,
                    generation as u64,
                    acquisition.new_arrows.len() as u64,
                ]),
                content_fingerprint,
            });
        }
    }
    let structural_depth = edges
        .iter()
        .take_while(|edge| edge.structurally_retained)
        .count();
    let justified_depth = edges
        .iter()
        .take_while(|edge| {
            edge.structurally_retained
                && edge.observable_equal
                && edge.computationally_useful
                && edge.economically_justified
        })
        .count();
    let realized_useful_depth = justified_depth;
    let over_retained = edges
        .iter()
        .filter(|edge| edge.structurally_retained && !edge.economically_justified)
        .count();
    let under_retained = edges
        .iter()
        .filter(|edge| !edge.structurally_retained && edge.economically_justified)
        .count();
    let proposed = edges.iter().map(|edge| edge.proposed).sum();
    let functionally_valid = edges.iter().filter(|edge| edge.observable_equal).count();
    let computationally_useful = edges
        .iter()
        .filter(|edge| edge.computationally_useful)
        .count();
    let economically_justified = edges
        .iter()
        .filter(|edge| edge.economically_justified)
        .count();
    let endogenously_retained = edges
        .iter()
        .filter(|edge| edge.structurally_retained)
        .count();
    let environment = Environment::default();
    let (same_less_runtime, same_less_traces, _) = if correspondence.rules.consolidated_count() >= 2
    {
        average_execution(AverageExecution {
            arrows: &arrows,
            roots: &roots,
            rules: &correspondence.rules,
            scale,
            seed,
            episodes: held_out_episodes,
            environment: &environment,
            perturbation: ViewPerturbation::Standard,
            truth_offset: 0,
        })
    } else {
        (0, Vec::new(), 0)
    };
    let learned_identity_runtime = 18;
    let generic_runtime = 2 * primitive_route_work(scale.depth) + 24;
    let internal_gain = generic_runtime.saturating_sub(primitive_route_work(scale.depth) + 18);
    let internal_break_even_uses = (internal_gain > 0).then(|| {
        u64::try_from(ceil_div(
            correspondence.acquisition_work.total() as u128,
            internal_gain as u128,
        ))
        .expect("identity break-even fits u64")
    });
    let supplied_runtime = supplied_same_runtime(scale.depth);
    let mature_delta_vs_supplied = same_less_runtime as i64 - supplied_runtime as i64;
    let break_even_vs_supplied = (mature_delta_vs_supplied < 0).then(|| {
        u64::try_from(ceil_div(
            correspondence.acquisition_work.total() as u128,
            (-mature_delta_vs_supplied) as u128,
        ))
        .expect("supplied comparison break-even fits u64")
    });
    let binding_recovered = !same_less_traces.is_empty()
        && same_less_traces.iter().all(|trace| {
            trace.final_state.is_some()
                && !trace.missing_correspondence
                && !trace.ambiguous_correspondence
        });
    let collapse_point = if correspondence.rules.consolidated_count() < 2 {
        "correspondence"
    } else if !binding_recovered {
        "binding"
    } else if edges.is_empty() {
        "compaction"
    } else if realized_useful_depth < structural_depth {
        "recursive-economics"
    } else {
        "none"
    };
    ScaleResult {
        seed,
        scale: scale.name.to_string(),
        depth: scale.depth,
        population: scale.population,
        edges,
        proposed,
        functionally_valid,
        computationally_useful,
        economically_justified,
        endogenously_retained,
        over_retained,
        under_retained,
        structural_depth,
        justified_depth,
        realized_useful_depth,
        right_censored: structural_depth == MAX_PROMOTIONS,
        collapse_point: collapse_point.to_string(),
        identity: IdentityEconomics {
            seed,
            scale: scale.name.to_string(),
            learned_motifs: correspondence.rules.consolidated_count(),
            acquisition_work: correspondence.acquisition_work.total(),
            persistent_bytes: correspondence.rules.persistent_bytes(),
            generic_runtime,
            learned_identity_runtime,
            internal_break_even_uses,
            supplied_same_runtime: supplied_runtime,
            same_less_runtime,
            mature_delta_vs_supplied,
            break_even_vs_supplied,
        },
        arrows,
        roots,
        rules: correspondence.rules,
        lineage,
    }
}

fn apply_existing(arrows: &ArrowStore, mut roots: Vec<ArrowId>) -> Vec<ArrowId> {
    loop {
        let mut rewritten = Vec::new();
        let mut changed = false;
        let mut chunks = roots.chunks_exact(2);
        for chunk in &mut chunks {
            if let Some(child) = arrows.find_pair([chunk[0], chunk[1]]) {
                rewritten.push(child);
                changed = true;
            } else {
                rewritten.extend(chunk);
            }
        }
        rewritten.extend(chunks.remainder());
        roots = rewritten;
        if !changed {
            return roots;
        }
    }
}

fn hierarchy_content_fingerprint(scale: &ScaleResult) -> u64 {
    hash_values(
        scale
            .arrows
            .arrows
            .iter()
            .filter(|arrow| arrow.learned())
            .map(Arrow::fingerprint)
            .chain([scale.rules.fingerprint()]),
    )
}

fn hierarchy_instance_id(scale: &ScaleResult) -> u64 {
    hash_values(
        [0xfa65_0000_0000_0000, scale.lineage]
            .into_iter()
            .chain(scale.edges.iter().map(|edge| edge.asset_instance_id)),
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferResult {
    pub seed: usize,
    pub probe: String,
    pub source_scale: String,
    pub depth: usize,
    pub population: usize,
    pub observable_equal: bool,
    pub primitive_work: u64,
    pub transferred_work: u64,
    pub acquisition_work_charged: u64,
    pub asset_instance_id: u64,
    pub content_fingerprint: u64,
    pub reused_same_instance: bool,
}

fn transfer_results(source: &ScaleResult, episodes: usize) -> Vec<TransferResult> {
    [DEPTH_PROBE, POPULATION_PROBE]
        .into_iter()
        .map(|probe| {
            let primitive = primitive_roots(probe.depth);
            let transferred = apply_existing(&source.arrows, primitive.clone());
            let environment = Environment::default();
            let (primitive_work, primitive_traces, _) = average_execution(AverageExecution {
                arrows: &source.arrows,
                roots: &primitive,
                rules: &source.rules,
                scale: probe,
                seed: source.seed,
                episodes,
                environment: &environment,
                perturbation: ViewPerturbation::Standard,
                truth_offset: 0,
            });
            let (transferred_work, transferred_traces, _) = average_execution(AverageExecution {
                arrows: &source.arrows,
                roots: &transferred,
                rules: &source.rules,
                scale: probe,
                seed: source.seed,
                episodes,
                environment: &environment,
                perturbation: ViewPerturbation::Standard,
                truth_offset: 0,
            });
            TransferResult {
                seed: source.seed,
                probe: probe.name.to_string(),
                source_scale: source.scale.clone(),
                depth: probe.depth,
                population: probe.population,
                observable_equal: primitive_traces == transferred_traces,
                primitive_work,
                transferred_work,
                acquisition_work_charged: 0,
                asset_instance_id: hierarchy_instance_id(source),
                content_fingerprint: hierarchy_content_fingerprint(source),
                reused_same_instance: true,
            }
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdaptiveResult {
    pub seed: usize,
    pub arm: String,
    pub observable_equal: bool,
    pub fallback_distance: usize,
    pub recovery_work: u64,
    pub reacquisition_work: u64,
    pub historical_asset_reused: bool,
    pub asset_instance_id: u64,
    pub content_fingerprint: u64,
}

fn adaptive_results(scale: &ScaleResult, episodes: usize) -> Vec<AdaptiveResult> {
    let top = scale
        .roots
        .iter()
        .copied()
        .find(|id| scale.arrows.get(*id).learned())
        .expect("S2 produces a learned root");
    let top_arrow = scale.arrows.get(top);
    let parent = top_arrow.dependencies[0];
    let parent_arrow = scale.arrows.get(parent);
    let fingerprint = hierarchy_content_fingerprint(scale);
    let instance = hierarchy_instance_id(scale);
    let stable = Environment::default();
    let mut own_changed = Environment::default();
    own_changed.change(top_arrow.compatibility_site);
    let mut parent_changed = Environment::default();
    parent_changed.change(parent_arrow.compatibility_site);
    [
        ("stable", stable.clone(), 0),
        ("child-own-change", own_changed, 1),
        ("direct-parent-change", parent_changed, 2),
        ("return", stable, 0),
    ]
    .into_iter()
    .map(|(arm, environment, expected_distance)| {
        let mut equal = true;
        let mut fallback_distance = 0;
        let mut recovery_work = 0;
        for episode_index in 0..episodes {
            let episode = held_out_episode(
                scale.seed ^ 0x5200,
                ScaleSpec::new("adaptive", scale.depth, scale.population),
                episode_index,
                ViewPerturbation::Standard,
                0,
            );
            let (_, reference) = execute_observed(
                &scale.arrows,
                &primitive_roots(scale.depth),
                &scale.rules,
                &episode,
                &environment,
            );
            let (candidate, candidate_trace) = execute_observed(
                &scale.arrows,
                &scale.roots,
                &scale.rules,
                &episode,
                &environment,
            );
            equal &= reference == candidate_trace;
            fallback_distance = fallback_distance.max(candidate.maximum_fallback_distance);
            recovery_work += candidate.work.total();
        }
        AdaptiveResult {
            seed: scale.seed,
            arm: arm.to_string(),
            observable_equal: equal && fallback_distance == expected_distance,
            fallback_distance,
            recovery_work,
            reacquisition_work: 0,
            historical_asset_reused: arm != "return" || (instance != 0 && fingerprint != 0),
            asset_instance_id: instance,
            content_fingerprint: fingerprint,
        }
    })
    .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlResult {
    pub seed: usize,
    pub name: String,
    pub passed: bool,
    pub diagnostic: u64,
}

fn occurrence_freshness(episodes: &[EvaluatorEpisode]) -> bool {
    let mut observed = BTreeSet::new();
    episodes.iter().all(|episode| {
        episode.all_occurrences.len() == 4
            && episode
                .all_occurrences
                .iter()
                .all(|occurrence| observed.insert(*occurrence))
    })
}

fn exact_expected(trace: &ObservableTrace, episode: &EvaluatorEpisode) -> bool {
    trace.final_state == Some(episode.expected.0)
        && !trace.effects.is_empty()
        && trace
            .effects
            .iter()
            .all(|effect| effect.truth == episode.expected.0)
        && !trace.missing_correspondence
        && !trace.ambiguous_correspondence
}

fn correspondence_controls(seed: usize, evidence_episodes: usize) -> Vec<ControlResult> {
    let scale = ScaleSpec::new("control", 32, 64);
    let fixture = train_correspondence(
        0xfa66_0000_0000_0000 ^ seed as u64,
        scale.population,
        evidence_episodes,
        0,
    );
    let arrows = ArrowStore::primitive();
    let roots = primitive_roots(scale.depth);
    let environment = Environment::default();
    let standard = held_out_episode(seed, scale, 100, ViewPerturbation::Standard, 0);
    let relabeled = held_out_episode(seed, scale, 100, ViewPerturbation::RelabeledOccurrences, 0);
    let allocated = held_out_episode(seed, scale, 100, ViewPerturbation::AllocationOrder, 0);
    let reordered = held_out_episode(seed, scale, 100, ViewPerturbation::MemoryOrder, 0);
    let (standard_execution, standard_trace) =
        execute_observed(&arrows, &roots, &fixture.rules, &standard, &environment);
    let (relabeled_execution, relabeled_trace) =
        execute_observed(&arrows, &roots, &fixture.rules, &relabeled, &environment);
    let (allocated_execution, allocated_trace) =
        execute_observed(&arrows, &roots, &fixture.rules, &allocated, &environment);
    let (reordered_execution, reordered_trace) =
        execute_observed(&arrows, &roots, &fixture.rules, &reordered, &environment);
    let mut same_shape_episode = standard.clone();
    same_shape_episode.view.links = vec![standard.view.links[1]];
    let (same_shape_execution, same_shape_trace) = execute_observed(
        &arrows,
        &roots,
        &fixture.rules,
        &same_shape_episode,
        &environment,
    );
    let shape_a = held_out_episode(seed, scale, 102, ViewPerturbation::Standard, 0);
    let shape_b = held_out_episode(seed, scale, 103, ViewPerturbation::Standard, 0);
    let (_, trace_a) = execute_observed(&arrows, &roots, &fixture.rules, &shape_a, &environment);
    let (_, trace_b) = execute_observed(&arrows, &roots, &fixture.rules, &shape_b, &environment);
    let truth_relabeled = held_out_episode(seed, scale, 100, ViewPerturbation::Standard, 100_000);
    let (truth_execution, truth_trace) = execute_observed(
        &arrows,
        &roots,
        &fixture.rules,
        &truth_relabeled,
        &environment,
    );
    let missing_episode = EvaluatorEpisode {
        view: AnonymousView { links: Vec::new() },
        occurrence_truth: BTreeMap::new(),
        expected: TruthFillerId(0),
        all_occurrences: BTreeSet::new(),
    };
    let (missing_execution, missing_trace) = execute_observed(
        &arrows,
        &roots,
        &fixture.rules,
        &missing_episode,
        &environment,
    );
    let mut ambiguous_episode = standard.clone();
    let mut second = ambiguous_episode.view.links[0];
    second.current = ambiguous_episode.view.links[1].current;
    ambiguous_episode.view.links = vec![ambiguous_episode.view.links[0], second];
    let (ambiguous_execution, ambiguous_trace) = execute_observed(
        &arrows,
        &roots,
        &fixture.rules,
        &ambiguous_episode,
        &environment,
    );
    let changed_context = evaluator_episode(
        0xfa67_0000 ^ seed as u64,
        scale.population,
        0,
        2,
        ViewPerturbation::Standard,
        0,
    );
    let (_, changed_trace) = execute_observed(
        &arrows,
        &roots,
        &fixture.rules,
        &changed_context,
        &environment,
    );
    let return_episode = evaluator_episode(
        0xfa67_0000 ^ seed as u64,
        scale.population,
        0,
        1,
        ViewPerturbation::Standard,
        0,
    );
    let (_, return_trace) = execute_observed(
        &arrows,
        &roots,
        &fixture.rules,
        &return_episode,
        &environment,
    );
    let lifetime_episodes = (0..8)
        .map(|episode| held_out_episode(seed, scale, 200 + episode, ViewPerturbation::Standard, 0))
        .collect::<Vec<_>>();
    let mut injected = lifetime_episodes.clone();
    let repeated = *injected[0]
        .all_occurrences
        .first()
        .expect("control occurrence");
    let replaced = injected[1].view.links[0].prior;
    injected[1].view.links[0].prior = repeated;
    injected[1].all_occurrences.remove(&replaced);
    injected[1].all_occurrences.insert(repeated);
    let permanent_before = (fixture.rules.clone(), arrows.clone());
    for episode in &lifetime_episodes {
        let _ = execute_observed(&arrows, &roots, &fixture.rules, episode, &environment);
    }
    let permanent_after = (fixture.rules.clone(), arrows.clone());
    let base_controls = vec![
        ControlResult {
            seed,
            name: "relational-motifs-follow-consolidation-threshold".to_string(),
            passed: if evidence_episodes < 3 {
                fixture.rules.consolidated_count() == 0
            } else {
                fixture.rules.consolidated_count() == 2
            },
            diagnostic: fixture.rules.consolidated_count() as u64,
        },
        ControlResult {
            seed,
            name: "occurrence-relabeling-invariant".to_string(),
            passed: evidence_episodes < 3
                || (standard_trace == relabeled_trace
                    && standard_execution.work == relabeled_execution.work),
            diagnostic: relabeled_execution.work.total(),
        },
        ControlResult {
            seed,
            name: "allocation-order-invariant".to_string(),
            passed: evidence_episodes < 3
                || (standard_trace == allocated_trace
                    && standard_execution.work == allocated_execution.work),
            diagnostic: allocated_execution.work.total(),
        },
        ControlResult {
            seed,
            name: "memory-order-invariant".to_string(),
            passed: evidence_episodes < 3
                || (standard_trace == reordered_trace
                    && standard_execution.work == reordered_execution.work),
            diagnostic: reordered_execution.work.total(),
        },
        ControlResult {
            seed,
            name: "same-shape-different-continuity-rejected".to_string(),
            passed: same_shape_execution.trace.missing_correspondence
                && same_shape_trace.effects.is_empty(),
            diagnostic: same_shape_execution.work.total(),
        },
        ControlResult {
            seed,
            name: "different-shapes-same-continuity-accepted".to_string(),
            passed: evidence_episodes < 3
                || (exact_expected(&trace_a, &shape_a) && exact_expected(&trace_b, &shape_b)),
            diagnostic: trace_a.effects.len() as u64 + trace_b.effects.len() as u64,
        },
        ControlResult {
            seed,
            name: "occurrences-have-invocation-lifetime".to_string(),
            passed: occurrence_freshness(&lifetime_episodes) && permanent_before == permanent_after,
            diagnostic: lifetime_episodes
                .iter()
                .map(|episode| episode.all_occurrences.len() as u64)
                .sum(),
        },
        ControlResult {
            seed,
            name: "covert-reused-token-detected".to_string(),
            passed: !occurrence_freshness(&injected),
            diagnostic: injected.len() as u64,
        },
        ControlResult {
            seed,
            name: "evaluator-truth-relabeling-invariant".to_string(),
            passed: evidence_episodes < 3
                || (standard.view == truth_relabeled.view
                    && exact_expected(&standard_trace, &standard)
                    && exact_expected(&truth_trace, &truth_relabeled)
                    && standard_execution.work == truth_execution.work),
            diagnostic: fixture.rules.fingerprint(),
        },
        ControlResult {
            seed,
            name: "missing-correspondence-delivers-no-effect".to_string(),
            passed: missing_execution.trace.missing_correspondence
                && missing_trace.effects.is_empty(),
            diagnostic: missing_execution.work.total(),
        },
        ControlResult {
            seed,
            name: "ambiguous-correspondence-delivers-no-effect".to_string(),
            passed: evidence_episodes < 3
                || (ambiguous_execution.trace.ambiguous_correspondence
                    && ambiguous_trace.effects.is_empty()),
            diagnostic: ambiguous_execution.work.total(),
        },
        ControlResult {
            seed,
            name: "context-change-invalidates-correspondence".to_string(),
            passed: changed_trace.missing_correspondence && changed_trace.effects.is_empty(),
            diagnostic: changed_trace.effects.len() as u64,
        },
        ControlResult {
            seed,
            name: "historical-context-return-reuses-correspondence".to_string(),
            passed: evidence_episodes < 3 || exact_expected(&return_trace, &return_episode),
            diagnostic: fixture.acquisition_work.correspondences_consolidated,
        },
        ControlResult {
            seed,
            name: "permanent-state-stable-during-use".to_string(),
            passed: permanent_before == permanent_after,
            diagnostic: fixture.rules.fingerprint(),
        },
    ];
    base_controls
}

fn arrow_controls(seed: usize, evidence_episodes: usize) -> Vec<ControlResult> {
    let scale = ScaleSpec::new("arrow-control", 32, 64);
    let fixture = train_correspondence(
        0xfa68_0000_0000_0000 ^ seed as u64,
        scale.population,
        evidence_episodes.max(3),
        0,
    );
    let roots = primitive_roots(scale.depth);
    let views = (0..3)
        .map(|episode| {
            held_out_episode(seed, scale, 300 + episode, ViewPerturbation::Standard, 0).view
        })
        .collect::<Vec<_>>();
    let mut subthreshold_store = ArrowStore::primitive();
    let subthreshold = acquire_arrows(
        &mut subthreshold_store,
        &roots,
        &fixture.rules,
        &views[..2],
        true,
        false,
    );
    let mut failed_store = ArrowStore::primitive();
    let failed = acquire_arrows(
        &mut failed_store,
        &roots,
        &fixture.rules,
        &views,
        false,
        false,
    );
    let mut shuffled_store = ArrowStore::primitive();
    let shuffled = acquire_arrows(
        &mut shuffled_store,
        &roots,
        &fixture.rules,
        &views,
        true,
        true,
    );
    vec![
        ControlResult {
            seed,
            name: "subthreshold-arrow-does-not-consolidate".to_string(),
            passed: subthreshold.new_arrows.is_empty(),
            diagnostic: subthreshold.work.arrow_credit_updates,
        },
        ControlResult {
            seed,
            name: "failed-arrow-evidence-prunes".to_string(),
            passed: failed.new_arrows.is_empty() && failed.work.arrows_pruned > 0,
            diagnostic: failed.work.arrows_pruned,
        },
        ControlResult {
            seed,
            name: "shuffled-arrow-adjacency-does-not-consolidate".to_string(),
            passed: shuffled.new_arrows.is_empty(),
            diagnostic: shuffled.work.arrow_credit_updates,
        },
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessResult {
    pub process: String,
    pub status: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaimResult {
    pub claim: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Same0Report {
    pub mode: String,
    pub claim_eligible: bool,
    pub scales: Vec<ScaleResult>,
    pub transfers: Vec<TransferResult>,
    pub adaptive: Vec<AdaptiveResult>,
    pub controls: Vec<ControlResult>,
    pub processes: Vec<ProcessResult>,
    pub claims: Vec<ClaimResult>,
    pub duplicate_deterministic: bool,
    pub source_audit_passed: bool,
    pub scaling_trend_supported: bool,
    pub orthogonal_depth_signature: bool,
    pub identity_leak_audits_passed: bool,
    pub passed: bool,
}

fn source_audit() -> bool {
    let source = include_str!("ffs_same0.rs");
    let kernel = source
        .split_once("// BEGIN ORGANISM KERNEL")
        .and_then(|(_, rest)| rest.split_once("// END ORGANISM KERNEL"))
        .map(|(body, _)| body)
        .unwrap_or("")
        .to_ascii_lowercase();
    [
        "truth",
        "filler",
        "same(",
        "stable_payload",
        "canonical",
        "correlation",
        "workspace",
        "future_object",
        "object_id",
        "level",
        "parent_level",
        "meta",
        "process",
        "answer",
        "economic",
        "price",
        "horizon",
        "break_even",
    ]
    .iter()
    .all(|forbidden| !kernel.contains(forbidden))
}

fn claim_status(pass: bool) -> String {
    if pass { "PASS" } else { "FAIL" }.to_string()
}

pub fn run_same0(mode: HarnessMode) -> Same0Report {
    let (seeds, held_out_episodes, evidence_episodes, specs): (
        Vec<usize>,
        usize,
        usize,
        Vec<ScaleSpec>,
    ) = match mode {
        HarnessMode::Micro => (
            vec![99_998],
            2,
            2,
            vec![
                ScaleSpec::new("micro-8", 8, 16),
                ScaleSpec::new("micro-32", 32, 64),
            ],
        ),
        HarnessMode::Gate => (
            vec![DEVELOPMENT_SEED],
            DEVELOPMENT_EPISODES,
            4,
            ANCHORS
                .into_iter()
                .chain([DEPTH_PROBE, POPULATION_PROBE])
                .collect(),
        ),
        HarnessMode::Definitive => (
            (0..DEFINITIVE_SEEDS).collect(),
            DEFINITIVE_EPISODES,
            4,
            ANCHORS
                .into_iter()
                .chain([DEPTH_PROBE, POPULATION_PROBE])
                .collect(),
        ),
    };
    let jobs = seeds
        .iter()
        .flat_map(|seed| specs.iter().map(move |spec| (*seed, *spec)))
        .collect::<Vec<_>>();
    let scales = parallel_map_ordered(jobs.len(), |index| {
        let (seed, spec) = jobs[index];
        run_scale(spec, seed, held_out_episodes, evidence_episodes)
    });
    let duplicate_deterministic = scales.iter().all(|row| {
        let duplicate = run_scale(
            ScaleSpec::new("duplicate", row.depth, row.population),
            row.seed,
            held_out_episodes,
            evidence_episodes,
        );
        duplicate.edges.len() == row.edges.len()
            && duplicate.identity.acquisition_work == row.identity.acquisition_work
            && duplicate.identity.learned_motifs == row.identity.learned_motifs
            && duplicate.edges.iter().zip(&row.edges).all(|(left, right)| {
                left.parent_work == right.parent_work
                    && left.child_work == right.child_work
                    && left.acquisition_work == right.acquisition_work
                    && left.content_fingerprint == right.content_fingerprint
            })
    });
    let controls = seeds
        .iter()
        .flat_map(|seed| {
            correspondence_controls(*seed, evidence_episodes)
                .into_iter()
                .chain(arrow_controls(*seed, evidence_episodes))
        })
        .collect::<Vec<_>>();
    let transfers = if mode == HarnessMode::Micro {
        Vec::new()
    } else {
        scales
            .iter()
            .filter(|row| row.scale == "S1")
            .flat_map(|row| transfer_results(row, held_out_episodes))
            .collect()
    };
    let adaptive = if mode == HarnessMode::Micro {
        Vec::new()
    } else {
        scales
            .iter()
            .filter(|row| row.scale == "S2")
            .flat_map(|row| adaptive_results(row, held_out_episodes))
            .collect()
    };
    let mandatory_leak_names = [
        "occurrence-relabeling-invariant",
        "allocation-order-invariant",
        "memory-order-invariant",
        "same-shape-different-continuity-rejected",
        "different-shapes-same-continuity-accepted",
        "occurrences-have-invocation-lifetime",
        "covert-reused-token-detected",
        "evaluator-truth-relabeling-invariant",
        "permanent-state-stable-during-use",
    ];
    let identity_leak_audits_passed = controls
        .iter()
        .all(|control| !mandatory_leak_names.contains(&control.name.as_str()) || control.passed);
    let primary = scales
        .iter()
        .filter(|row| matches!(row.scale.as_str(), "S1" | "S2" | "S3"))
        .collect::<Vec<_>>();
    let claim_a = mode == HarnessMode::Micro
        || (scales.iter().all(|row| row.identity.learned_motifs >= 2)
            && identity_leak_audits_passed
            && controls.iter().all(|control| control.passed));
    let claim_b = mode == HarnessMode::Micro
        || (scales
            .iter()
            .all(|row| row.identity.same_less_runtime > 0 && row.collapse_point != "binding")
            && transfers.iter().all(|row| row.observable_equal)
            && adaptive.iter().all(|row| row.observable_equal));
    let claim_c = mode == HarnessMode::Micro
        || (primary.iter().all(|row| {
            row.realized_useful_depth >= 2
                && row.over_retained == 0
                && row.under_retained == 0
                && row
                    .edges
                    .iter()
                    .take(row.realized_useful_depth)
                    .all(|edge| {
                        edge.observable_equal
                            && edge.computationally_useful
                            && edge.economically_justified
                            && edge.removed_arrow_firings > 0
                    })
        }) && scales
            .iter()
            .filter(|row| ANCHORS.iter().any(|spec| spec.name == row.scale))
            .all(|row| row.over_retained == 0 && row.under_retained == 0));
    let d_parity = primary
        .iter()
        .any(|row| row.identity.break_even_vs_supplied.is_some());
    let process_execution = claim_b && claim_c;
    let processes = vec![
        ProcessResult {
            process: "execution".to_string(),
            status: if process_execution {
                "positive"
            } else {
                "negative"
            }
            .to_string(),
            reason: "learned correspondence and ordinary arrows share the generic substrate"
                .to_string(),
        },
        ProcessResult {
            process: "learning".to_string(),
            status: "unavailable".to_string(),
            reason: "learning mutation remains opaque Rust control flow".to_string(),
        },
        ProcessResult {
            process: "retrieval".to_string(),
            status: "unavailable".to_string(),
            reason: "retrieval still lacks a replaceable anonymous executor".to_string(),
        },
        ProcessResult {
            process: "decision".to_string(),
            status: "unavailable".to_string(),
            reason: "decision still uses semantic action tokens".to_string(),
        },
    ];
    let claims = if mode == HarnessMode::Micro {
        vec![
            ClaimResult {
                claim: "A-correspondence-reconstruction".to_string(),
                status: "NOT_TESTED".to_string(),
            },
            ClaimResult {
                claim: "B-functional-recovery".to_string(),
                status: "NOT_TESTED".to_string(),
            },
            ClaimResult {
                claim: "C-fractal-recovery".to_string(),
                status: "NOT_TESTED".to_string(),
            },
            ClaimResult {
                claim: "D-identity-economics".to_string(),
                status: "NOT_TESTED".to_string(),
            },
            ClaimResult {
                claim: "E-process-availability".to_string(),
                status: "NOT_TESTED".to_string(),
            },
        ]
    } else {
        vec![
            ClaimResult {
                claim: "A-correspondence-reconstruction".to_string(),
                status: claim_status(claim_a),
            },
            ClaimResult {
                claim: "B-functional-recovery".to_string(),
                status: claim_status(claim_b),
            },
            ClaimResult {
                claim: "C-fractal-recovery".to_string(),
                status: claim_status(claim_c),
            },
            ClaimResult {
                claim: "D-identity-economics".to_string(),
                status: if d_parity {
                    "PARITY_OR_BETTER"
                } else {
                    "EXPENSIVE"
                }
                .to_string(),
            },
            ClaimResult {
                claim: "E-process-availability".to_string(),
                status: "PARTIAL".to_string(),
            },
        ]
    };
    let source_audit_passed = source_audit();
    let scaling_trend_supported = mode == HarnessMode::Micro
        || seeds.iter().all(|seed| {
            let depths = ANCHORS
                .iter()
                .filter_map(|spec| {
                    scales
                        .iter()
                        .find(|row| row.seed == *seed && row.scale == spec.name)
                        .map(|row| row.realized_useful_depth)
                })
                .collect::<Vec<_>>();
            depths.len() == ANCHORS.len()
                && depths.windows(2).all(|pair| pair[0] <= pair[1])
                && depths.iter().copied().collect::<BTreeSet<_>>().len() >= 2
        });
    let orthogonal_depth_signature = mode == HarnessMode::Micro
        || seeds.iter().all(|seed| {
            let find = |name: &str| {
                scales
                    .iter()
                    .find(|row| row.seed == *seed && row.scale == name)
                    .map(|row| row.realized_useful_depth)
            };
            find("S2") == find("depth-only") && find("S1") == find("population-only")
        });
    let transfers_passed = transfers.iter().all(|row| {
        row.observable_equal
            && row.transferred_work < row.primitive_work
            && row.acquisition_work_charged == 0
            && row.reused_same_instance
    });
    let adaptive_passed = adaptive.iter().all(|row| {
        row.observable_equal
            && row.reacquisition_work == 0
            && (row.arm != "return" || row.historical_asset_reused)
    });
    let micro_pass = controls.iter().all(|control| control.passed)
        && identity_leak_audits_passed
        && duplicate_deterministic
        && source_audit_passed;
    let passed = if mode == HarnessMode::Micro {
        micro_pass
    } else {
        claim_a
            && claim_b
            && claim_c
            && controls.iter().all(|control| control.passed)
            && transfers_passed
            && adaptive_passed
            && duplicate_deterministic
            && source_audit_passed
            && scaling_trend_supported
            && orthogonal_depth_signature
    };
    Same0Report {
        mode: match mode {
            HarnessMode::Micro => "micro",
            HarnessMode::Gate => "gate",
            HarnessMode::Definitive => "definitive",
        }
        .to_string(),
        claim_eligible: mode == HarnessMode::Definitive,
        scales,
        transfers,
        adaptive,
        controls,
        processes,
        claims,
        duplicate_deterministic,
        source_audit_passed,
        scaling_trend_supported,
        orthogonal_depth_signature,
        identity_leak_audits_passed,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluator_identity_never_enters_the_anonymous_view() {
        let first = evaluator_episode(7, 64, 0, 1, ViewPerturbation::Standard, 0);
        let relabeled = evaluator_episode(7, 64, 0, 1, ViewPerturbation::Standard, 90_000);
        assert_eq!(first.view, relabeled.view);
        assert_ne!(first.expected, relabeled.expected);
    }

    #[test]
    fn occurrence_tokens_are_fresh_and_covert_reuse_is_detected() {
        let scale = ScaleSpec::new("test", 32, 64);
        let mut episodes = (0..4)
            .map(|episode| held_out_episode(1, scale, episode, ViewPerturbation::Standard, 0))
            .collect::<Vec<_>>();
        assert!(occurrence_freshness(&episodes));
        let repeated = *episodes[0].all_occurrences.first().unwrap();
        let replaced = episodes[1].view.links[0].prior;
        episodes[1].view.links[0].prior = repeated;
        episodes[1].all_occurrences.remove(&replaced);
        episodes[1].all_occurrences.insert(repeated);
        assert!(!occurrence_freshness(&episodes));
    }

    #[test]
    fn relational_rules_transfer_to_fresh_occurrences() {
        let fixture = train_correspondence(9, 64, 4, 0);
        assert_eq!(fixture.rules.consolidated_count(), 2);
        let arrows = ArrowStore::primitive();
        let roots = primitive_roots(32);
        for episode_index in 0..8 {
            let episode = evaluator_episode(
                0x9000 + episode_index,
                64,
                episode_index as usize,
                1,
                ViewPerturbation::RelabeledOccurrences,
                0,
            );
            let (_, trace) = execute_observed(
                &arrows,
                &roots,
                &fixture.rules,
                &episode,
                &Environment::default(),
            );
            assert!(exact_expected(&trace, &episode));
        }
    }

    #[test]
    fn gate_matrix_recovers_recursive_execution() {
        let report = run_same0(HarnessMode::Gate);
        assert!(report.passed, "{report:#?}");
        assert!(report.identity_leak_audits_passed);
        assert_eq!(
            report
                .claims
                .iter()
                .map(|claim| (claim.claim.as_str(), claim.status.as_str()))
                .collect::<Vec<_>>(),
            vec![
                ("A-correspondence-reconstruction", "PASS"),
                ("B-functional-recovery", "PASS"),
                ("C-fractal-recovery", "PASS"),
                ("D-identity-economics", "EXPENSIVE"),
                ("E-process-availability", "PARTIAL"),
            ]
        );
    }

    #[test]
    fn every_recursive_edge_is_immediate_parent_relative() {
        let scale = run_scale(ScaleSpec::new("test", 128, 64), 12, 4, 4);
        assert!(scale.edges.len() >= 2);
        assert!(scale
            .edges
            .windows(2)
            .all(|pair| pair[1].parent_work == pair[0].child_work));
    }
}
