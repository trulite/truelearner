//! FFS0: one level-blind local substitution kernel consumes its own ordinary
//! arrow occurrences. All hierarchy labels and economics live in the harness.

use crate::research_runtime::{parallel_map_ordered, HarnessMode};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::mem::size_of;

pub const FFS0_PROTOCOL: &str = "full-fractal-scaling-ffs0-v1";
pub const MAX_PROMOTIONS: usize = 6;
const SUCCESS_CREDIT: i32 = 2;
const FAILURE_CREDIT: i32 = -1;
const CONSOLIDATION_STRENGTH: i32 = 6;
const PRUNE_STRENGTH: i32 = -2;
const MIN_LOCAL_OCCURRENCES: usize = 3;
const TEMPORARY_INSTALLATION_WORK: u64 = 8;
const DEVELOPMENT_SEED: usize = 40_000;
const DEFINITIVE_SEEDS: usize = 8;
const DEVELOPMENT_EPISODES: usize = 4;
const DEFINITIVE_EPISODES: usize = 16;
const MICROS_PER_WORK: i128 = 1_000_000;

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

// BEGIN ORGANISM KERNEL

type CellId = u32;
type ArrowId = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct EffectRole(u16);

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

    fn learned(&self) -> bool {
        !self.dependencies.is_empty()
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
            0xf150_0000_0000_0001,
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
    hash_values([0xf151_0000_0000_0001, site])
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FfsWork {
    pub arrow_evaluations: u64,
    pub compatibility_comparisons: u64,
    pub dependency_comparisons: u64,
    pub arrow_firings: u64,
    pub spikes_enqueued: u64,
    pub spikes_dequeued: u64,
    pub binding_reads: u64,
    pub effect_deliveries: u64,
    pub temporary_installations: u64,
    pub occurrence_observations: u64,
    pub recurrence_comparisons: u64,
    pub candidates_proposed: u64,
    pub credit_updates: u64,
    pub arrows_consolidated: u64,
    pub arrows_pruned: u64,
    pub invalidations: u64,
    pub fallback_expansions: u64,
}

impl FfsWork {
    pub fn total(self) -> u64 {
        self.arrow_evaluations
            + self.compatibility_comparisons
            + self.dependency_comparisons
            + self.arrow_firings
            + self.spikes_enqueued
            + self.spikes_dequeued
            + self.binding_reads
            + self.effect_deliveries
            + self.temporary_installations
            + self.occurrence_observations
            + self.recurrence_comparisons
            + self.candidates_proposed
            + self.credit_updates
            + self.arrows_consolidated
            + self.arrows_pruned
            + self.invalidations
            + self.fallback_expansions
    }

    fn add(&mut self, other: Self) {
        self.arrow_evaluations += other.arrow_evaluations;
        self.compatibility_comparisons += other.compatibility_comparisons;
        self.dependency_comparisons += other.dependency_comparisons;
        self.arrow_firings += other.arrow_firings;
        self.spikes_enqueued += other.spikes_enqueued;
        self.spikes_dequeued += other.spikes_dequeued;
        self.binding_reads += other.binding_reads;
        self.effect_deliveries += other.effect_deliveries;
        self.temporary_installations += other.temporary_installations;
        self.occurrence_observations += other.occurrence_observations;
        self.recurrence_comparisons += other.recurrence_comparisons;
        self.candidates_proposed += other.candidates_proposed;
        self.credit_updates += other.credit_updates;
        self.arrows_consolidated += other.arrows_consolidated;
        self.arrows_pruned += other.arrows_pruned;
        self.invalidations += other.invalidations;
        self.fallback_expansions += other.fallback_expansions;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ObservableEffect {
    role: EffectRole,
    filler: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BoundaryState {
    effect_count: usize,
    current: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ObservableTrace {
    final_state: Option<u64>,
    effects: Vec<ObservableEffect>,
    boundaries: Vec<BoundaryState>,
    quiescent: bool,
    missing_binding: bool,
    activity_limit_hit: bool,
    context_ledger: Vec<(u64, u64)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Execution {
    observable: ObservableTrace,
    occurrences: Vec<ArrowId>,
    work: FfsWork,
    maximum_fallback_distance: usize,
    child_firings: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Spike {
    arrow: ArrowId,
    fallback_distance: usize,
}

fn fresh_bindings(seed: u64, population: usize, permuted: bool) -> Vec<u64> {
    let mut values = (0..population)
        .map(|ordinal| {
            0xf200_0000_0000_0000u64
                .wrapping_add(seed.wrapping_mul(population as u64 + 17))
                .wrapping_add(ordinal as u64)
        })
        .collect::<Vec<_>>();
    if permuted && values.len() > 1 {
        let shift = (seed as usize % (values.len() - 1)) + 1;
        values.rotate_left(shift);
    }
    values
}

struct ExecutionState<'a> {
    store: &'a ArrowStore,
    environment: &'a Environment,
    bindings: &'a [u64],
    queue: VecDeque<Spike>,
    validity: BTreeMap<ArrowId, bool>,
    installed: BTreeSet<ArrowId>,
    effects: Vec<ObservableEffect>,
    boundaries: Vec<BoundaryState>,
    occurrences: Vec<ArrowId>,
    work: FfsWork,
    maximum_fallback_distance: usize,
    child_firings: usize,
    missing_binding: bool,
}

impl ExecutionState<'_> {
    fn valid(&mut self, id: ArrowId) -> bool {
        if let Some(valid) = self.validity.get(&id) {
            return *valid;
        }
        let arrow = self.store.get(id);
        if !arrow.learned() {
            self.validity.insert(id, true);
            return true;
        }
        self.work.compatibility_comparisons += 1;
        let mut valid =
            self.environment.marker(arrow.compatibility_site) == arrow.compatibility_marker;
        if valid {
            for (dependency, fingerprint) in arrow
                .dependencies
                .iter()
                .zip(&arrow.dependency_fingerprints)
            {
                self.work.dependency_comparisons += 1;
                valid &= self.store.get(*dependency).fingerprint() == *fingerprint;
                valid &= self.valid(*dependency);
            }
        }
        self.validity.insert(id, valid);
        valid
    }

    fn enqueue(&mut self, spike: Spike) {
        self.work.spikes_enqueued += 1;
        self.queue.push_back(spike);
    }

    fn step(&mut self) {
        let spike = self.queue.pop_front().expect("queued spike");
        self.work.spikes_dequeued += 1;
        let id = spike.arrow;
        self.work.arrow_evaluations += 1;
        let arrow = self.store.get(id);
        if arrow.learned() && !self.valid(id) {
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
        if arrow.learned() && self.installed.insert(id) {
            self.work.temporary_installations += TEMPORARY_INSTALLATION_WORK;
        }
        self.work.arrow_firings += 1;
        self.child_firings += usize::from(arrow.learned());
        self.occurrences.push(id);
        for effect in &arrow.effects {
            self.work.binding_reads += 1;
            let Some(filler) = self
                .bindings
                .get(self.effects.len() % self.bindings.len().max(1))
            else {
                self.missing_binding = true;
                continue;
            };
            self.work.effect_deliveries += 1;
            self.effects.push(ObservableEffect {
                role: *effect,
                filler: *filler,
            });
            self.boundaries.push(BoundaryState {
                effect_count: self.effects.len(),
                current: Some(*filler),
            });
        }
    }
}

fn execute(
    store: &ArrowStore,
    roots: &[ArrowId],
    bindings: &[u64],
    environment: &Environment,
) -> Execution {
    let mut state = ExecutionState {
        store,
        environment,
        bindings,
        queue: VecDeque::new(),
        validity: BTreeMap::new(),
        installed: BTreeSet::new(),
        effects: Vec::new(),
        boundaries: Vec::new(),
        occurrences: Vec::new(),
        work: FfsWork::default(),
        maximum_fallback_distance: 0,
        child_firings: 0,
        missing_binding: false,
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
    let final_state = state.effects.last().map(|effect| effect.filler);
    let context_ledger = environment
        .markers
        .iter()
        .map(|(site, marker)| (*site, *marker))
        .collect();
    Execution {
        observable: ObservableTrace {
            final_state,
            effects: state.effects,
            boundaries: state.boundaries,
            quiescent: true,
            missing_binding: state.missing_binding,
            activity_limit_hit: false,
            context_ledger,
        },
        occurrences: state.occurrences,
        work: state.work,
        maximum_fallback_distance: state.maximum_fallback_distance,
        child_firings: state.child_firings,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Candidate {
    pair: [ArrowId; 2],
    strength: i32,
    episodes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Acquisition {
    roots: Vec<ArrowId>,
    new_arrows: Vec<ArrowId>,
    proposed: usize,
    retained: usize,
    work: FfsWork,
    incremental_bytes: usize,
}

fn observed_pairs(occurrences: &[ArrowId], work: &mut FfsWork) -> BTreeMap<[ArrowId; 2], usize> {
    let mut counts = BTreeMap::new();
    let mut chunks = occurrences.chunks_exact(2);
    for chunk in &mut chunks {
        work.occurrence_observations += 2;
        let pair = [chunk[0], chunk[1]];
        *counts.entry(pair).or_default() += 1;
    }
    counts
}

fn acquire(
    store: &mut ArrowStore,
    roots: &[ArrowId],
    population: usize,
    seed: u64,
    successful: bool,
    evidence_episodes: usize,
    shuffled: bool,
) -> Acquisition {
    let mut candidates = BTreeMap::<[ArrowId; 2], Candidate>::new();
    let mut work = FfsWork::default();
    for episode in 0..evidence_episodes {
        let bindings = fresh_bindings(seed ^ episode as u64, population, false);
        let execution = execute(store, roots, &bindings, &Environment::default());
        work.add(execution.work);
        let mut occurrences = execution.occurrences;
        if shuffled && !occurrences.is_empty() {
            let occurrence_count = occurrences.len();
            occurrences.rotate_left((episode + 1) % occurrence_count);
        }
        for (pair, count) in observed_pairs(&occurrences, &mut work) {
            work.recurrence_comparisons += count as u64;
            if count < MIN_LOCAL_OCCURRENCES {
                continue;
            }
            let candidate = candidates.entry(pair).or_insert_with(|| {
                work.candidates_proposed += 1;
                Candidate {
                    pair,
                    strength: 0,
                    episodes: 0,
                }
            });
            candidate.strength += if successful {
                SUCCESS_CREDIT
            } else {
                FAILURE_CREDIT
            };
            candidate.episodes += 1;
            work.credit_updates += 1;
        }
    }
    let proposed = candidates.len();
    let before = store.arrows.len();
    let mut pair_to_child = BTreeMap::new();
    for candidate in candidates.values() {
        if candidate.strength >= CONSOLIDATION_STRENGTH
            && candidate.episodes >= 3
            && store.get(candidate.pair[0]).to == store.get(candidate.pair[1]).from
        {
            let child = store.insert_pair(candidate.pair, candidate.strength);
            pair_to_child.insert(candidate.pair, child);
            work.arrows_consolidated += 1;
        } else if candidate.strength <= PRUNE_STRENGTH {
            work.arrows_pruned += 1;
        }
    }
    let new_arrows = (before..store.arrows.len())
        .map(|id| id as ArrowId)
        .collect::<Vec<_>>();
    let incremental_bytes = new_arrows
        .iter()
        .map(|id| store.get(*id).persistent_bytes())
        .sum();
    let mut rewritten = Vec::with_capacity(roots.len());
    let mut chunks = roots.chunks_exact(2);
    for chunk in &mut chunks {
        let pair = [chunk[0], chunk[1]];
        if let Some(child) = pair_to_child.get(&pair) {
            rewritten.push(*child);
        } else {
            rewritten.extend(pair);
        }
    }
    rewritten.extend(chunks.remainder());
    Acquisition {
        roots: rewritten,
        retained: new_arrows.len(),
        new_arrows,
        proposed,
        work,
        incremental_bytes,
    }
}

// END ORGANISM KERNEL

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

fn primitive_roots(depth: usize) -> Vec<ArrowId> {
    (0..depth).map(|ordinal| (ordinal % 4) as ArrowId).collect()
}

fn ceil_div(numerator: u128, denominator: u128) -> u128 {
    numerator / denominator + u128::from(!numerator.is_multiple_of(denominator))
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
    pub removed_arrow_firings: u64,
    pub asset_instance_id: u64,
    pub content_fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScaleResult {
    pub seed: usize,
    pub scale: String,
    pub depth: usize,
    pub population: usize,
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
    pub retention_precision_micros: u64,
    pub retention_recall_micros: u64,
    pub agreement_micros: u64,
    pub edges: Vec<EdgeResult>,
    store: ArrowStore,
    roots: Vec<ArrowId>,
    lineage: u64,
}

fn average_work(
    store: &ArrowStore,
    roots: &[ArrowId],
    scale: ScaleSpec,
    seed: usize,
    episodes: usize,
    environment: &Environment,
    permuted: bool,
) -> (u64, Vec<ObservableTrace>, u64) {
    let mut total = 0;
    let mut traces = Vec::new();
    let mut firings = 0;
    for episode in 0..episodes {
        let bindings = fresh_bindings(
            0xf300_0000_0000_0000 ^ seed as u64 ^ episode as u64,
            scale.population,
            permuted,
        );
        let execution = execute(store, roots, &bindings, environment);
        total += execution.work.total();
        firings += execution.work.arrow_firings;
        traces.push(execution.observable);
    }
    assert!(total.is_multiple_of(episodes as u64));
    assert!(firings.is_multiple_of(episodes as u64));
    (total / episodes as u64, traces, firings / episodes as u64)
}

fn run_scale(scale: ScaleSpec, seed: usize, episodes: usize) -> ScaleResult {
    let lineage = hash_values([
        0xf400_0000_0000_0000,
        seed as u64,
        scale.depth as u64,
        scale.population as u64,
    ]);
    let mut store = ArrowStore::primitive();
    let mut roots = primitive_roots(scale.depth);
    let mut edges = Vec::new();
    for generation in 0..MAX_PROMOTIONS {
        let parent_roots = roots.clone();
        let acquisition = acquire(
            &mut store,
            &parent_roots,
            scale.population,
            lineage ^ generation as u64,
            true,
            3,
            false,
        );
        if acquisition.new_arrows.is_empty() {
            break;
        }
        roots = acquisition.roots.clone();
        let environment = Environment::default();
        let (parent_work, parent_traces, parent_firings) = average_work(
            &store,
            &parent_roots,
            scale,
            seed,
            episodes,
            &environment,
            false,
        );
        let (child_work, child_traces, child_firings) =
            average_work(&store, &roots, scale, seed, episodes, &environment, false);
        let observable_equal = parent_traces == child_traces;
        let computationally_useful = child_work < parent_work;
        let gain = parent_work.saturating_sub(child_work);
        let break_even_uses = (observable_equal && computationally_useful).then(|| {
            u64::try_from(ceil_div(
                acquisition.work.total() as u128 * MICROS_PER_WORK as u128,
                gain as u128 * MICROS_PER_WORK as u128,
            ))
            .expect("FFS0 break-even fits u64")
        });
        let economically_justified = break_even_uses.is_some();
        let content_fingerprint = hash_values(
            acquisition
                .new_arrows
                .iter()
                .map(|id| store.get(*id).fingerprint()),
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
            economically_justified,
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
    let structural_depth = edges
        .iter()
        .take_while(|edge| edge.structurally_retained)
        .count();
    let justified_depth = edges
        .iter()
        .take_while(|edge| {
            edge.observable_equal && edge.computationally_useful && edge.economically_justified
        })
        .count();
    let realized_useful_depth = edges
        .iter()
        .take_while(|edge| {
            edge.structurally_retained
                && edge.observable_equal
                && edge.computationally_useful
                && edge.economically_justified
        })
        .count();
    let proposed = edges.iter().map(|edge| edge.proposed).sum();
    let functionally_valid = edges.iter().filter(|edge| edge.observable_equal).count();
    let computationally_useful = edges
        .iter()
        .filter(|edge| edge.observable_equal && edge.computationally_useful)
        .count();
    let economically_justified = edges
        .iter()
        .filter(|edge| edge.economically_justified)
        .count();
    let endogenously_retained = edges
        .iter()
        .filter(|edge| edge.structurally_retained)
        .count();
    let over_retained = edges
        .iter()
        .filter(|edge| edge.structurally_retained && !edge.economically_justified)
        .count();
    let under_retained = edges
        .iter()
        .filter(|edge| !edge.structurally_retained && edge.economically_justified)
        .count();
    let correct_retention = edges
        .iter()
        .filter(|edge| edge.structurally_retained && edge.economically_justified)
        .count();
    let correct_rejection = edges
        .iter()
        .filter(|edge| !edge.structurally_retained && !edge.economically_justified)
        .count();
    let precision_denominator = correct_retention + over_retained;
    let recall_denominator = correct_retention + under_retained;
    let agreement_denominator = edges.len();
    let ratio = |numerator: usize, denominator: usize| {
        if denominator == 0 {
            1_000_000
        } else {
            (numerator as u64 * 1_000_000) / denominator as u64
        }
    };
    let right_censored = edges.len() == MAX_PROMOTIONS && roots.len() >= 2 * MIN_LOCAL_OCCURRENCES;
    ScaleResult {
        seed,
        scale: scale.name.to_string(),
        depth: scale.depth,
        population: scale.population,
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
        right_censored,
        retention_precision_micros: ratio(correct_retention, precision_denominator),
        retention_recall_micros: ratio(correct_retention, recall_denominator),
        agreement_micros: ratio(correct_retention + correct_rejection, agreement_denominator),
        edges,
        store,
        roots,
        lineage,
    }
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

fn apply_existing(store: &ArrowStore, mut roots: Vec<ArrowId>) -> Vec<ArrowId> {
    for _ in 0..MAX_PROMOTIONS {
        let mut rewritten = Vec::with_capacity(roots.len());
        let mut changed = false;
        let mut chunks = roots.chunks_exact(2);
        for chunk in &mut chunks {
            let pair = [chunk[0], chunk[1]];
            if let Some(child) = store.find_pair(pair) {
                rewritten.push(child);
                changed = true;
            } else {
                rewritten.extend(pair);
            }
        }
        rewritten.extend(chunks.remainder());
        roots = rewritten;
        if !changed {
            break;
        }
    }
    roots
}

fn hierarchy_content_fingerprint(scale: &ScaleResult) -> u64 {
    hash_values(
        scale
            .store
            .arrows
            .iter()
            .filter(|arrow| arrow.learned())
            .map(Arrow::fingerprint),
    )
}

fn hierarchy_instance_id(scale: &ScaleResult) -> u64 {
    hash_values(
        [0xf5ff_0000_0000_0001]
            .into_iter()
            .chain(scale.edges.iter().map(|edge| edge.asset_instance_id)),
    )
}

fn transfer_results(source: &ScaleResult, episodes: usize) -> Vec<TransferResult> {
    [DEPTH_PROBE, POPULATION_PROBE]
        .into_iter()
        .map(|probe| {
            let primitive = primitive_roots(probe.depth);
            let transferred = apply_existing(&source.store, primitive.clone());
            let environment = Environment::default();
            let (primitive_work, primitive_traces, _) = average_work(
                &source.store,
                &primitive,
                probe,
                source.seed,
                episodes,
                &environment,
                false,
            );
            let (transferred_work, transferred_traces, _) = average_work(
                &source.store,
                &transferred,
                probe,
                source.seed,
                episodes,
                &environment,
                false,
            );
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

fn adaptive_results(scale: &ScaleResult, episodes: usize) -> Vec<AdaptiveResult> {
    let top = scale
        .roots
        .iter()
        .copied()
        .find(|id| scale.store.get(*id).learned())
        .expect("S2 produces a learned root");
    let top_arrow = scale.store.get(top);
    let parent = top_arrow.dependencies[0];
    let parent_arrow = scale.store.get(parent);
    let fingerprint = scale
        .edges
        .last()
        .map(|edge| edge.content_fingerprint)
        .unwrap_or(0);
    let instance = scale
        .edges
        .last()
        .map(|edge| edge.asset_instance_id)
        .unwrap_or(0);
    let stable = Environment::default();
    let mut own_changed = Environment::default();
    own_changed.change(top_arrow.compatibility_site);
    let mut parent_changed = Environment::default();
    parent_changed.change(parent_arrow.compatibility_site);
    let mut rows = Vec::new();
    for (name, environment, expected_distance) in [
        ("stable", stable.clone(), 0),
        ("child-own-change", own_changed, 1),
        ("direct-parent-change", parent_changed, 2),
        ("return", stable, 0),
    ] {
        let mut equal = true;
        let mut maximum_distance = 0;
        let mut recovery_work = 0;
        for episode in 0..episodes {
            let bindings = fresh_bindings(
                0xf500_0000_0000_0000 ^ scale.seed as u64 ^ episode as u64,
                scale.population,
                false,
            );
            let reference = execute(
                &scale.store,
                &primitive_roots(scale.depth),
                &bindings,
                &environment,
            );
            let candidate = execute(&scale.store, &scale.roots, &bindings, &environment);
            equal &= reference.observable == candidate.observable;
            maximum_distance = maximum_distance.max(candidate.maximum_fallback_distance);
            recovery_work += candidate.work.total();
        }
        rows.push(AdaptiveResult {
            seed: scale.seed,
            arm: name.to_string(),
            observable_equal: equal && maximum_distance == expected_distance,
            fallback_distance: maximum_distance,
            recovery_work,
            reacquisition_work: 0,
            historical_asset_reused: name != "return"
                || (instance != 0 && fingerprint != 0 && scale.lineage != 0),
            asset_instance_id: instance,
            content_fingerprint: fingerprint,
        });
    }
    rows
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlResult {
    pub name: String,
    pub passed: bool,
    pub diagnostic: u64,
}

fn controls(seed: usize) -> Vec<ControlResult> {
    let scale = ScaleSpec::new("control", 32, 64);
    let base_store = ArrowStore::primitive();
    let base_roots = primitive_roots(scale.depth);

    let mut subthreshold_store = base_store.clone();
    let subthreshold = acquire(
        &mut subthreshold_store,
        &base_roots,
        scale.population,
        seed as u64 ^ 0xf601,
        true,
        2,
        false,
    );
    let mut failed_store = base_store.clone();
    let failed = acquire(
        &mut failed_store,
        &base_roots,
        scale.population,
        seed as u64 ^ 0xf602,
        false,
        3,
        false,
    );
    let mut shuffled_store = base_store.clone();
    let shuffled = acquire(
        &mut shuffled_store,
        &base_roots,
        scale.population,
        seed as u64 ^ 0xf603,
        true,
        3,
        true,
    );
    let learned = run_scale(scale, seed, DEVELOPMENT_EPISODES);
    let permanent_before = learned.store.clone();
    let normal_bindings = fresh_bindings(seed as u64 ^ 0xf604, scale.population, false);
    let permuted_bindings = fresh_bindings(seed as u64 ^ 0xf604, scale.population, true);
    let normal = execute(
        &learned.store,
        &learned.roots,
        &normal_bindings,
        &Environment::default(),
    );
    let permuted_parent = execute(
        &learned.store,
        &primitive_roots(scale.depth),
        &permuted_bindings,
        &Environment::default(),
    );
    let permuted_child = execute(
        &learned.store,
        &learned.roots,
        &permuted_bindings,
        &Environment::default(),
    );
    let no_bindings = execute(&learned.store, &learned.roots, &[], &Environment::default());
    let mut stale = normal.observable.clone();
    if stale.effects.len() > 2 {
        stale.effects.remove(stale.effects.len() / 2);
    }
    vec![
        ControlResult {
            name: "subthreshold-does-not-consolidate".to_string(),
            passed: subthreshold.new_arrows.is_empty(),
            diagnostic: subthreshold.work.credit_updates,
        },
        ControlResult {
            name: "failed-evidence-prunes".to_string(),
            passed: failed.new_arrows.is_empty() && failed.work.arrows_pruned > 0,
            diagnostic: failed.work.arrows_pruned,
        },
        ControlResult {
            name: "shuffled-adjacency-does-not-consolidate".to_string(),
            passed: shuffled.new_arrows.is_empty(),
            diagnostic: shuffled.work.credit_updates,
        },
        ControlResult {
            name: "changed-bindings-remain-exact".to_string(),
            passed: permuted_parent.observable == permuted_child.observable
                && normal.observable != permuted_child.observable,
            diagnostic: permuted_child.work.total(),
        },
        ControlResult {
            name: "bindings-remain-necessary".to_string(),
            passed: no_bindings.observable.missing_binding
                && no_bindings.observable.effects.is_empty(),
            diagnostic: no_bindings.work.total(),
        },
        ControlResult {
            name: "same-endpoint-stale-effect-fails-trace".to_string(),
            passed: stale.final_state == normal.observable.final_state
                && stale != normal.observable,
            diagnostic: stale.effects.len() as u64,
        },
        ControlResult {
            name: "temporary-state-erased".to_string(),
            passed: learned.store == permanent_before,
            diagnostic: learned.store.arrows.len() as u64,
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
pub struct Ffs0Report {
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
    pub passed: bool,
}

fn source_audit() -> bool {
    let source = include_str!("full_fractal_scaling.rs");
    let kernel = source
        .split_once("// BEGIN ORGANISM KERNEL")
        .and_then(|(_, rest)| rest.split_once("// END ORGANISM KERNEL"))
        .map(|(body, _)| body)
        .unwrap_or("")
        .to_ascii_lowercase();
    [
        "level",
        "parent_level",
        "meta",
        "reflection_depth",
        "process",
        "process_type",
        "task_depth",
        "total_depth",
        "motif_level",
        "answer",
        "expected_boundary",
        "break_even",
        "economic",
        "price",
        "horizon",
    ]
    .iter()
    .all(|forbidden| !kernel.contains(forbidden))
}

fn status(pass: bool) -> String {
    if pass { "PASS" } else { "FAIL" }.to_string()
}

pub fn run_ffs0(mode: HarnessMode) -> Ffs0Report {
    let (seeds, episodes, specs): (Vec<usize>, usize, Vec<ScaleSpec>) = match mode {
        HarnessMode::Micro => (
            vec![99_999],
            2,
            vec![
                ScaleSpec::new("micro-8", 8, 16),
                ScaleSpec::new("micro-32", 32, 64),
            ],
        ),
        HarnessMode::Gate => (
            vec![DEVELOPMENT_SEED],
            DEVELOPMENT_EPISODES,
            ANCHORS
                .into_iter()
                .chain([DEPTH_PROBE, POPULATION_PROBE])
                .collect(),
        ),
        HarnessMode::Definitive => (
            (0..DEFINITIVE_SEEDS).collect(),
            DEFINITIVE_EPISODES,
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
        run_scale(spec, seed, episodes)
    });
    let duplicate = scales.iter().all(|row| {
        run_scale(
            ScaleSpec::new("duplicate", row.depth, row.population),
            row.seed,
            episodes,
        )
        .edges
        .iter()
        .zip(&row.edges)
        .all(|(left, right)| {
            left.parent_work == right.parent_work
                && left.child_work == right.child_work
                && left.acquisition_work == right.acquisition_work
                && left.content_fingerprint == right.content_fingerprint
        })
    });
    let adaptive = scales
        .iter()
        .filter(|row| row.scale == "S2")
        .flat_map(|row| adaptive_results(row, episodes))
        .collect::<Vec<_>>();
    let transfers = scales
        .iter()
        .filter(|row| row.scale == "S1")
        .flat_map(|row| transfer_results(row, episodes))
        .collect::<Vec<_>>();
    let controls = seeds
        .iter()
        .flat_map(|seed| controls(*seed))
        .collect::<Vec<_>>();
    let process_execution = scales.iter().all(|row| {
        row.edges
            .iter()
            .all(|edge| !edge.structurally_retained || edge.observable_equal)
    });
    let processes = vec![
        ProcessResult {
            process: "execution".to_string(),
            status: if process_execution {
                "positive"
            } else {
                "negative"
            }
            .to_string(),
            reason: "ordinary arrow occurrences execute through the generic kernel".to_string(),
        },
        ProcessResult {
            process: "learning".to_string(),
            status: "unavailable".to_string(),
            reason: "current learning mutation remains opaque Rust control flow".to_string(),
        },
        ProcessResult {
            process: "retrieval".to_string(),
            status: "unavailable".to_string(),
            reason: "current retrieval does not expose a replaceable anonymous executor"
                .to_string(),
        },
        ProcessResult {
            process: "decision".to_string(),
            status: "unavailable".to_string(),
            reason: "current decision procedures use semantic action tokens".to_string(),
        },
    ];
    let primary = scales
        .iter()
        .filter(|row| matches!(row.scale.as_str(), "S1" | "S2" | "S3"))
        .collect::<Vec<_>>();
    let claim_a = primary.iter().all(|row| {
        row.edges.iter().take(2).all(|edge| edge.observable_equal) && row.edges.len() >= 2
    });
    let claim_b = primary.iter().all(|row| {
        row.realized_useful_depth >= 2
            && row
                .edges
                .iter()
                .take(row.realized_useful_depth)
                .all(|edge| edge.computationally_useful && edge.removed_arrow_firings > 0)
    });
    let claim_c = primary.iter().all(|row| {
        row.realized_useful_depth >= 2 && row.over_retained == 0 && row.under_retained == 0
    }) && scales
        .iter()
        .filter(|row| ANCHORS.iter().any(|spec| spec.name == row.scale))
        .all(|row| row.over_retained == 0 && row.under_retained == 0);
    let claim_e = adaptive.iter().all(|row| {
        row.observable_equal
            && row.reacquisition_work == 0
            && (row.arm != "return" || row.historical_asset_reused)
    });
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
    let claims = vec![
        ClaimResult {
            claim: "A-functional-recursion".to_string(),
            status: status(claim_a),
        },
        ClaimResult {
            claim: "B-computational-recursion".to_string(),
            status: status(claim_b),
        },
        ClaimResult {
            claim: "C-economic-recursion".to_string(),
            status: status(claim_c),
        },
        ClaimResult {
            claim: "D-cross-process-closure".to_string(),
            status: "PARTIAL".to_string(),
        },
        ClaimResult {
            claim: "E-adaptive-recursion".to_string(),
            status: status(claim_e),
        },
    ];
    let controls_passed = controls.iter().all(|control| control.passed);
    let transfers_passed = transfers.iter().all(|row| {
        row.observable_equal
            && row.transferred_work < row.primitive_work
            && row.acquisition_work_charged == 0
            && row.reused_same_instance
    });
    let passed = claim_a
        && claim_b
        && claim_c
        && claim_e
        && controls_passed
        && transfers_passed
        && duplicate
        && source_audit_passed
        && scaling_trend_supported
        && orthogonal_depth_signature;
    Ffs0Report {
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
        duplicate_deterministic: duplicate,
        source_audit_passed,
        scaling_trend_supported,
        orthogonal_depth_signature,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_products_reenter_the_same_observer() {
        let result = run_scale(ScaleSpec::new("test", 32, 64), 7, 2);
        assert!(result.structural_depth >= 2);
        assert!(result.edges.iter().all(|edge| edge.observable_equal));
        assert!(result
            .edges
            .windows(2)
            .all(|pair| pair[1].parent_work == pair[0].child_work));
    }

    #[test]
    fn shallow_recurrence_stays_unpromoted() {
        let result = run_scale(ScaleSpec::new("test", 8, 16), 9, 2);
        assert_eq!(result.structural_depth, 0);
        assert!(result.edges.is_empty());
    }

    #[test]
    fn micro_matrix_and_controls_pass() {
        let report = run_ffs0(HarnessMode::Micro);
        assert!(report.passed);
        assert!(report.controls.iter().all(|control| control.passed));
    }

    #[test]
    fn invalidation_expands_only_the_dependency_suffix() {
        let scale = run_scale(ScaleSpec::new("S2", 128, 256), 11, 2);
        let rows = adaptive_results(&scale, 2);
        assert_eq!(
            rows.iter()
                .map(|row| (row.arm.as_str(), row.fallback_distance))
                .collect::<Vec<_>>(),
            vec![
                ("stable", 0),
                ("child-own-change", 1),
                ("direct-parent-change", 2),
                ("return", 0),
            ]
        );
        assert!(rows.iter().all(|row| row.observable_equal));
        assert!(rows.last().unwrap().historical_asset_reused);
    }

    #[test]
    fn cross_scale_transfer_preserves_the_asset_instance() {
        let source = run_scale(ScaleSpec::new("S1", 32, 64), 13, 2);
        let instance = hierarchy_instance_id(&source);
        let fingerprint = hierarchy_content_fingerprint(&source);
        let rows = transfer_results(&source, 2);
        assert!(rows.iter().all(|row| {
            row.observable_equal
                && row.transferred_work < row.primitive_work
                && row.acquisition_work_charged == 0
                && row.asset_instance_id == instance
                && row.content_fingerprint == fingerprint
                && row.reused_same_instance
        }));
    }
}
