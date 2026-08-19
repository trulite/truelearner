use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap},
};

use crate::composable_models::{
    mix64, IdentitySource, OpaqueAction, OpaqueIdentity, ProvenanceLearner, RoleId,
    RoleTransformation, TemporaryStructure,
};

const ROLE_COUNT: usize = 13;
const ACTION_COUNT: usize = 3;
const TRAINING_EPISODES_PER_ACTION: usize = 64;
const EVALUATION_SEEDS: usize = 8;
const DEPTHS: [usize; 5] = [1, 2, 3, 4, 8];
const TRAINING_CHECKPOINTS: [usize; 8] = [0, 32, 128, 512, 2_048, 8_192, 32_768, 65_536];

#[derive(Clone, Debug)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.state
    }

    fn shuffle<T>(&mut self, values: &mut [T]) {
        for index in (1..values.len()).rev() {
            let selected = (self.next_u64() as usize) % (index + 1);
            values.swap(index, selected);
        }
    }
}

#[derive(Clone, Debug)]
struct SearchWorld {
    action_for_effect: [OpaqueAction; ACTION_COUNT],
    effect_for_action: BTreeMap<OpaqueAction, RoleTransformation>,
}

impl SearchWorld {
    fn new(seed: u64) -> Self {
        let mut shift = (0..ROLE_COUNT).collect::<Vec<_>>();
        shift[0] = 8;
        for (role, source) in shift.iter_mut().enumerate().take(9).skip(1) {
            *source = role - 1;
        }
        let mut reveal = (0..ROLE_COUNT).collect::<Vec<_>>();
        reveal[9] = 8;
        let identity = (0..ROLE_COUNT).collect::<Vec<_>>();
        let transformations = [
            RoleTransformation::new(shift),
            RoleTransformation::new(reveal),
            RoleTransformation::new(identity),
        ];

        let mut rng = DeterministicRng::new(seed ^ 0x5000_0000);
        let mut actions =
            std::array::from_fn(|index| OpaqueAction(mix64(seed ^ 0x5100_0000 ^ index as u64)));
        rng.shuffle(&mut actions);
        let effect_for_action = actions
            .iter()
            .copied()
            .zip(transformations)
            .collect::<BTreeMap<_, _>>();
        Self {
            action_for_effect: actions,
            effect_for_action,
        }
    }

    fn action(&self, effect_index: usize) -> OpaqueAction {
        self.action_for_effect[effect_index]
    }

    fn apply(&self, action: OpaqueAction, state: &TemporaryStructure) -> TemporaryStructure {
        self.effect_for_action[&action].apply(state)
    }
}

fn train_action_models(seed: u64) -> (SearchWorld, ProvenanceLearner) {
    let world = SearchWorld::new(seed);
    let mut learner = ProvenanceLearner::new(ROLE_COUNT);
    let mut identities = IdentitySource::new(seed ^ 0x5200_0000);
    for _ in 0..TRAINING_EPISODES_PER_ACTION {
        for action_index in 0..ACTION_COUNT {
            let action = world.action(action_index);
            let before = identities.fresh_structure(ROLE_COUNT);
            let after = world.apply(action, &before);
            learner.observe(action, &before, &after);
        }
    }
    (world, learner)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SearchGoal {
    first_roles: BTreeSet<RoleId>,
    second_roles: BTreeSet<RoleId>,
}

impl SearchGoal {
    fn reachable() -> Self {
        Self {
            first_roles: BTreeSet::from([RoleId(9), RoleId(12)]),
            second_roles: BTreeSet::from([RoleId(10), RoleId(12)]),
        }
    }

    fn unreachable() -> Self {
        Self {
            first_roles: BTreeSet::from([RoleId(11), RoleId(12)]),
            second_roles: BTreeSet::from([RoleId(10), RoleId(12)]),
        }
    }

    fn shared(&self) -> BTreeSet<RoleId> {
        self.first_roles
            .intersection(&self.second_roles)
            .copied()
            .collect()
    }

    fn first_only(&self) -> BTreeSet<RoleId> {
        self.first_roles
            .difference(&self.second_roles)
            .copied()
            .collect()
    }

    fn second_only(&self) -> BTreeSet<RoleId> {
        self.second_roles
            .difference(&self.first_roles)
            .copied()
            .collect()
    }

    fn output(&self, state: &TemporaryStructure, first: bool) -> Vec<OpaqueIdentity> {
        let roles = if first {
            &self.first_roles
        } else {
            &self.second_roles
        };
        roles.iter().map(|role| state.occupants[role.0]).collect()
    }

    fn distinguishes(&self, initial: &TemporaryStructure, predicted: &TemporaryStructure) -> bool {
        let count_changes = |roles: BTreeSet<RoleId>| {
            roles
                .iter()
                .filter(|role| initial.occupants[role.0] != predicted.occupants[role.0])
                .count()
        };
        let first_changes = count_changes(self.first_only());
        let second_changes = count_changes(self.second_only());
        count_changes(self.shared()) == 0
            && (first_changes > 0) != (second_changes > 0)
            && self.output(predicted, true) != self.output(predicted, false)
    }
}

fn initial_state(
    identities: &mut IdentitySource,
    required_depth: Option<usize>,
) -> TemporaryStructure {
    let base = identities.issue();
    let marker = identities.issue();
    let mut occupants = vec![base; ROLE_COUNT];
    if let Some(depth) = required_depth {
        occupants[9 - depth] = marker;
    }
    TemporaryStructure { occupants }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SearchValue {
    Low,
    Medium,
    High,
}

impl SearchValue {
    fn rank(self) -> i64 {
        match self {
            Self::Low => 1,
            Self::Medium => 2,
            Self::High => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ValueKey {
    equality_pattern: Vec<usize>,
    first_roles: Vec<usize>,
    second_roles: Vec<usize>,
    remaining_budget: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StateSignature([u8; ROLE_COUNT]);

impl StateSignature {
    fn from_state(state: &TemporaryStructure) -> Self {
        Self::from_classes(equality_pattern(state))
    }

    fn from_classes(classes: impl IntoIterator<Item = usize>) -> Self {
        let classes = classes.into_iter().collect::<Vec<_>>();
        assert_eq!(classes.len(), ROLE_COUNT);
        let mut canonical = BTreeMap::<usize, u8>::new();
        let mut next_class = 0u8;
        let mut signature = [0u8; ROLE_COUNT];
        for (role, class) in classes.into_iter().enumerate() {
            signature[role] = *canonical.entry(class).or_insert_with(|| {
                let assigned = next_class;
                next_class += 1;
                assigned
            });
        }
        Self(signature)
    }

    fn apply(self, transformation: &RoleTransformation) -> Self {
        Self::from_classes(
            transformation
                .source_for_output
                .iter()
                .map(|source| self.0[source.0] as usize),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CompiledValueKey {
    state: StateSignature,
    first_roles: u16,
    second_roles: u16,
    remaining_budget: u8,
}

fn role_mask(roles: impl IntoIterator<Item = usize>) -> u16 {
    roles
        .into_iter()
        .fold(0u16, |mask, role| mask | (1u16 << role))
}

impl CompiledValueKey {
    fn from_value_key(key: &ValueKey) -> Self {
        Self {
            state: StateSignature::from_classes(key.equality_pattern.iter().copied()),
            first_roles: role_mask(key.first_roles.iter().copied()),
            second_roles: role_mask(key.second_roles.iter().copied()),
            remaining_budget: key.remaining_budget as u8,
        }
    }

    fn from_parts(state: StateSignature, goal: &SearchGoal, remaining_budget: usize) -> Self {
        Self {
            state,
            first_roles: role_mask(goal.first_roles.iter().map(|role| role.0)),
            second_roles: role_mask(goal.second_roles.iter().map(|role| role.0)),
            remaining_budget: remaining_budget as u8,
        }
    }
}

fn equality_pattern(state: &TemporaryStructure) -> Vec<usize> {
    let mut classes = BTreeMap::<OpaqueIdentity, usize>::new();
    let mut next_class = 0;
    state
        .occupants
        .iter()
        .map(|identity| {
            *classes.entry(*identity).or_insert_with(|| {
                let class = next_class;
                next_class += 1;
                class
            })
        })
        .collect()
}

fn value_key(state: &TemporaryStructure, goal: &SearchGoal, remaining_budget: usize) -> ValueKey {
    ValueKey {
        equality_pattern: equality_pattern(state),
        first_roles: goal.first_roles.iter().map(|role| role.0).collect(),
        second_roles: goal.second_roles.iter().map(|role| role.0).collect(),
        remaining_budget,
    }
}

#[derive(Clone, Debug, Default)]
struct ValueCounts {
    low: usize,
    medium: usize,
    high: usize,
}

impl ValueCounts {
    fn winner(&self) -> Option<SearchValue> {
        let candidates = [
            (SearchValue::Low, self.low),
            (SearchValue::Medium, self.medium),
            (SearchValue::High, self.high),
        ];
        let strongest = candidates.iter().map(|(_, count)| count).max()?;
        let winners = candidates
            .iter()
            .filter(|(_, count)| count == strongest)
            .collect::<Vec<_>>();
        (winners.len() == 1).then_some(winners[0].0)
    }
}

#[derive(Clone, Debug, Default)]
struct SearchValueLearner {
    values: BTreeMap<ValueKey, ValueCounts>,
    examples: usize,
}

impl SearchValueLearner {
    fn observe(&mut self, key: ValueKey, value: SearchValue) {
        let counts = self.values.entry(key).or_default();
        match value {
            SearchValue::Low => counts.low += 1,
            SearchValue::Medium => counts.medium += 1,
            SearchValue::High => counts.high += 1,
        }
        self.examples += 1;
    }

    fn predict(
        &self,
        state: &TemporaryStructure,
        goal: &SearchGoal,
        remaining_budget: usize,
    ) -> Option<SearchValue> {
        self.values
            .get(&value_key(state, goal, remaining_budget))?
            .winner()
    }

    fn entries(&self) -> usize {
        self.values.len()
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for (key, counts) in &self.values {
            for class in &key.equality_pattern {
                mix_fingerprint(&mut hash, *class as u64);
            }
            for role in &key.first_roles {
                mix_fingerprint(&mut hash, *role as u64);
            }
            for role in &key.second_roles {
                mix_fingerprint(&mut hash, *role as u64);
            }
            mix_fingerprint(&mut hash, key.remaining_budget as u64);
            mix_fingerprint(&mut hash, counts.low as u64);
            mix_fingerprint(&mut hash, counts.medium as u64);
            mix_fingerprint(&mut hash, counts.high as u64);
        }
        hash
    }
}

#[derive(Clone, Debug)]
struct CompiledValueTable {
    values: HashMap<CompiledValueKey, Option<SearchValue>>,
    collision_free: bool,
    compilation_work: usize,
}

impl CompiledValueTable {
    fn compile(learner: &SearchValueLearner) -> Self {
        let mut values = HashMap::new();
        let mut collision_free = true;
        let mut compilation_work = 0;
        for (key, counts) in &learner.values {
            let compiled = CompiledValueKey::from_value_key(key);
            let value = counts.winner();
            if values
                .insert(compiled, value)
                .is_some_and(|previous| previous != value)
            {
                collision_free = false;
            }
            compilation_work +=
                key.equality_pattern.len() + key.first_roles.len() + key.second_roles.len() + 4;
        }
        Self {
            values,
            collision_free,
            compilation_work,
        }
    }

    fn predict(
        &self,
        state: StateSignature,
        goal: &SearchGoal,
        remaining_budget: usize,
    ) -> Option<SearchValue> {
        self.values
            .get(&CompiledValueKey::from_parts(state, goal, remaining_budget))
            .copied()
            .flatten()
    }

    fn entries(&self) -> usize {
        self.values.len()
    }
}

#[derive(Clone, Debug)]
struct LocalValueNetwork {
    entry_cells: HashMap<CompiledValueKey, usize>,
    arrows: BTreeMap<usize, usize>,
    cells: usize,
    compilation_work: usize,
}

impl LocalValueNetwork {
    fn compile(table: &CompiledValueTable) -> Self {
        let mut entry_cells = HashMap::new();
        let mut arrows = BTreeMap::new();
        let mut entries = table
            .values
            .iter()
            .map(|(&key, &value)| (key, value))
            .collect::<Vec<_>>();
        entries.sort_by_key(|(key, _)| *key);
        for (index, (key, value)) in entries.into_iter().enumerate() {
            let entry_cell = index + 3;
            entry_cells.insert(key, entry_cell);
            if let Some(value) = value {
                arrows.insert(entry_cell, value as usize);
            }
        }
        let confident = arrows.len();
        Self {
            entry_cells,
            arrows,
            cells: table.values.len() + 3,
            compilation_work: table.compilation_work + table.values.len() + confident,
        }
    }

    fn predict(
        &self,
        state: StateSignature,
        goal: &SearchGoal,
        remaining_budget: usize,
    ) -> Option<SearchValue> {
        let entry_cell = self
            .entry_cells
            .get(&CompiledValueKey::from_parts(state, goal, remaining_budget))
            .copied()?;
        match self.arrows.get(&entry_cell).copied()? {
            0 => Some(SearchValue::Low),
            1 => Some(SearchValue::Medium),
            2 => Some(SearchValue::High),
            _ => None,
        }
    }
}

fn mix_fingerprint(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(0x100_0000_01b3);
}

fn minimum_remaining_steps(
    transformations: &BTreeMap<OpaqueAction, RoleTransformation>,
    initial: &TemporaryStructure,
    goal: &SearchGoal,
    maximum_depth: usize,
) -> Option<usize> {
    if goal.distinguishes(initial, initial) {
        return Some(0);
    }
    let mut frontier = vec![initial.clone()];
    let mut visited = BTreeSet::from([initial.occupants.clone()]);
    for depth in 1..=maximum_depth {
        let mut next = Vec::new();
        for state in frontier {
            for transformation in transformations.values() {
                let predicted = transformation.apply(&state);
                if goal.distinguishes(initial, &predicted) {
                    return Some(depth);
                }
                if visited.insert(predicted.occupants.clone()) {
                    next.push(predicted);
                }
            }
        }
        frontier = next;
        if frontier.is_empty() {
            break;
        }
    }
    None
}

fn value_from_minimum(minimum: Option<usize>) -> SearchValue {
    match minimum {
        Some(0..=2) => SearchValue::High,
        Some(_) => SearchValue::Medium,
        None => SearchValue::Low,
    }
}

fn train_value_models(
    maximum_examples: usize,
) -> (
    BTreeMap<usize, SearchValueLearner>,
    SearchValueLearner,
    Vec<Vec<OpaqueAction>>,
) {
    let mut learner = SearchValueLearner::default();
    let mut shuffled = SearchValueLearner::default();
    let mut snapshots = BTreeMap::new();
    snapshots.insert(0, learner.clone());
    let mut memorized_paths = Vec::new();
    let mut rng = DeterministicRng::new(0x5300_0000);

    for example in 0..maximum_examples {
        let world_seed = 0x5400_0000 + (example / 64) as u64;
        let world = SearchWorld::new(world_seed);
        let depth = DEPTHS[example % DEPTHS.len()];
        let goal = if example % 7 == 6 {
            SearchGoal::unreachable()
        } else {
            SearchGoal::reachable()
        };
        let mut identities = IdentitySource::new(0x5500_0000 + example as u64);
        let initial = initial_state(&mut identities, Some(depth));
        let successful = std::iter::repeat_n(world.action(0), depth.saturating_sub(1))
            .chain(std::iter::once(world.action(1)))
            .collect::<Vec<_>>();
        if example % 64 == 0 {
            memorized_paths.push(successful.clone());
        }

        let prefix_length = (rng.next_u64() as usize) % (depth + 1);
        let sequence = if example % 2 == 0 {
            successful[..prefix_length].to_vec()
        } else {
            (0..prefix_length)
                .map(|_| world.action((rng.next_u64() as usize) % ACTION_COUNT))
                .collect()
        };
        let state = sequence.iter().fold(initial.clone(), |state, &action| {
            world.apply(action, &state)
        });
        let remaining = depth - prefix_length;
        let minimum = minimum_remaining_steps(&world.effect_for_action, &state, &goal, remaining);
        let value = value_from_minimum(minimum);
        let key = value_key(&state, &goal, remaining);
        learner.observe(key.clone(), value);
        let shuffled_value = match rng.next_u64() % 3 {
            0 => SearchValue::Low,
            1 => SearchValue::Medium,
            _ => SearchValue::High,
        };
        shuffled.observe(key, shuffled_value);

        let learned_examples = example + 1;
        if TRAINING_CHECKPOINTS.contains(&learned_examples) {
            snapshots.insert(learned_examples, learner.clone());
        }
    }
    (snapshots, shuffled, memorized_paths)
}

#[derive(Clone, Copy, Debug)]
enum PriorityPolicy<'a> {
    Exhaustive,
    Random(u64),
    Learned(&'a SearchValueLearner),
    Compiled(&'a CompiledValueTable),
    Local(&'a LocalValueNetwork),
    ZeroCost(&'a CompiledValueTable),
    Oracle(&'a BTreeMap<OpaqueAction, RoleTransformation>),
    Shuffled(&'a SearchValueLearner),
    Memorized(&'a [Vec<OpaqueAction>]),
}

impl PriorityPolicy<'_> {
    fn uses_signature(self) -> bool {
        matches!(self, Self::Compiled(_) | Self::Local(_) | Self::ZeroCost(_))
    }

    fn score(
        self,
        state: &TemporaryStructure,
        signature: Option<StateSignature>,
        goal: &SearchGoal,
        remaining: usize,
        sequence: &[OpaqueAction],
    ) -> (i64, bool) {
        match self {
            Self::Exhaustive => (1, false),
            Self::Random(seed) => {
                let sequence_hash = sequence
                    .iter()
                    .fold(seed, |hash, action| mix64(hash ^ action.0));
                ((sequence_hash & i64::MAX as u64) as i64, false)
            }
            Self::Learned(learner) | Self::Shuffled(learner) => (
                learner
                    .predict(state, goal, remaining)
                    .map_or(1, SearchValue::rank),
                true,
            ),
            Self::Compiled(table) | Self::ZeroCost(table) => (
                table
                    .predict(signature.expect("compiled signature"), goal, remaining)
                    .map_or(1, SearchValue::rank),
                true,
            ),
            Self::Local(network) => (
                network
                    .predict(signature.expect("local signature"), goal, remaining)
                    .map_or(1, SearchValue::rank),
                true,
            ),
            Self::Oracle(transformations) => (
                minimum_remaining_steps(transformations, state, goal, remaining)
                    .map_or(0, |steps| 1_000 - steps as i64),
                true,
            ),
            Self::Memorized(paths) => (
                if paths.iter().any(|path| path.starts_with(sequence)) {
                    3
                } else {
                    1
                },
                true,
            ),
        }
    }

    fn evaluation_cost(self, scored: bool) -> EvaluationCost {
        if !scored {
            return EvaluationCost::default();
        }
        match self {
            Self::Compiled(_) => EvaluationCost {
                signature_update_work: ROLE_COUNT,
                value_retrieval_work: 1,
                ..EvaluationCost::default()
            },
            Self::Local(_) => EvaluationCost {
                signature_update_work: ROLE_COUNT,
                local_activation_spikes: 2,
                ..EvaluationCost::default()
            },
            Self::ZeroCost(_) => EvaluationCost::default(),
            _ => EvaluationCost {
                value_retrieval_work: ROLE_COUNT + 5,
                ..EvaluationCost::default()
            },
        }
    }
}

#[derive(Clone, Debug)]
struct FrontierNode {
    priority: i64,
    depth: usize,
    insertion_priority: usize,
    sequence: Vec<OpaqueAction>,
    state: TemporaryStructure,
    signature: Option<StateSignature>,
}

#[derive(Clone, Copy, Debug, Default)]
struct EvaluationCost {
    signature_update_work: usize,
    value_retrieval_work: usize,
    local_activation_spikes: usize,
}

impl EvaluationCost {
    fn total(self) -> usize {
        self.signature_update_work + self.value_retrieval_work + self.local_activation_spikes
    }
}

impl PartialEq for FrontierNode {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
            && self.depth == other.depth
            && self.insertion_priority == other.insertion_priority
    }
}

impl Eq for FrontierNode {}

impl PartialOrd for FrontierNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FrontierNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.depth.cmp(&other.depth))
            .then_with(|| self.insertion_priority.cmp(&other.insertion_priority))
    }
}

#[derive(Clone, Debug, Default)]
struct SearchWork {
    partial_generated: usize,
    partial_scored: usize,
    partial_expanded: usize,
    model_applications: usize,
    complete_candidates: usize,
    heuristic_evaluations: usize,
    model_work: usize,
    heuristic_work: usize,
    signature_update_work: usize,
    value_retrieval_work: usize,
    local_activation_spikes: usize,
    signature_mismatches: usize,
}

impl SearchWork {
    fn add(&mut self, other: &Self) {
        self.partial_generated += other.partial_generated;
        self.partial_scored += other.partial_scored;
        self.partial_expanded += other.partial_expanded;
        self.model_applications += other.model_applications;
        self.complete_candidates += other.complete_candidates;
        self.heuristic_evaluations += other.heuristic_evaluations;
        self.model_work += other.model_work;
        self.heuristic_work += other.heuristic_work;
        self.signature_update_work += other.signature_update_work;
        self.value_retrieval_work += other.value_retrieval_work;
        self.local_activation_spikes += other.local_activation_spikes;
        self.signature_mismatches += other.signature_mismatches;
    }

    fn total_work(&self) -> usize {
        self.model_work + self.heuristic_work + self.complete_candidates * 4
    }
}

#[derive(Clone, Debug)]
struct ExactLengthResult {
    sequence: Option<Vec<OpaqueAction>>,
    predicted: Option<TemporaryStructure>,
    work: SearchWork,
}

fn search_exact_length(
    models: &BTreeMap<OpaqueAction, RoleTransformation>,
    initial: &TemporaryStructure,
    goal: &SearchGoal,
    target_length: usize,
    policy: PriorityPolicy<'_>,
) -> ExactLengthResult {
    let actions = models.keys().copied().collect::<Vec<_>>();
    let mut frontier = BinaryHeap::new();
    let root_signature = policy
        .uses_signature()
        .then(|| StateSignature::from_state(initial));
    let (root_priority, root_scored) =
        policy.score(initial, root_signature, goal, target_length, &[]);
    let root_cost = policy.evaluation_cost(root_scored);
    let mut work = SearchWork {
        partial_generated: 1,
        partial_scored: usize::from(root_scored),
        heuristic_evaluations: usize::from(root_scored),
        heuristic_work: root_cost.total(),
        signature_update_work: root_cost.signature_update_work,
        value_retrieval_work: root_cost.value_retrieval_work,
        local_activation_spikes: root_cost.local_activation_spikes,
        ..SearchWork::default()
    };
    frontier.push(FrontierNode {
        priority: root_priority,
        depth: 0,
        insertion_priority: usize::MAX,
        sequence: Vec::new(),
        state: initial.clone(),
        signature: root_signature,
    });
    let mut insertion = 0;

    while let Some(node) = frontier.pop() {
        work.partial_expanded += 1;
        if node.depth == target_length {
            work.complete_candidates += 1;
            if goal.distinguishes(initial, &node.state) {
                return ExactLengthResult {
                    sequence: Some(node.sequence),
                    predicted: Some(node.state),
                    work,
                };
            }
            continue;
        }

        for &action in &actions {
            let predicted = models[&action].apply(&node.state);
            let signature = node
                .signature
                .map(|signature| signature.apply(&models[&action]));
            let mut sequence = node.sequence.clone();
            sequence.push(action);
            let remaining = target_length - sequence.len();
            let (priority, scored) =
                policy.score(&predicted, signature, goal, remaining, &sequence);
            let evaluation_cost = policy.evaluation_cost(scored);
            work.partial_generated += 1;
            work.partial_scored += usize::from(scored);
            work.heuristic_evaluations += usize::from(scored);
            work.model_applications += 1;
            work.model_work += ROLE_COUNT;
            work.heuristic_work += evaluation_cost.total();
            work.signature_update_work += evaluation_cost.signature_update_work;
            work.value_retrieval_work += evaluation_cost.value_retrieval_work;
            work.local_activation_spikes += evaluation_cost.local_activation_spikes;
            work.signature_mismatches += usize::from(
                signature
                    .is_some_and(|signature| signature != StateSignature::from_state(&predicted)),
            );
            insertion += 1;
            frontier.push(FrontierNode {
                priority,
                depth: sequence.len(),
                insertion_priority: usize::MAX - insertion,
                sequence,
                state: predicted,
                signature,
            });
        }
    }

    ExactLengthResult {
        sequence: None,
        predicted: None,
        work,
    }
}

#[derive(Clone, Debug)]
struct PlanResult {
    sequence: Option<Vec<OpaqueAction>>,
    predicted: Option<TemporaryStructure>,
    work: SearchWork,
}

fn plan_shortest(
    models: &BTreeMap<OpaqueAction, RoleTransformation>,
    initial: &TemporaryStructure,
    goal: &SearchGoal,
    maximum_depth: usize,
    policy: PriorityPolicy<'_>,
) -> PlanResult {
    let mut work = SearchWork::default();
    for target_length in 1..=maximum_depth {
        let result = search_exact_length(models, initial, goal, target_length, policy);
        work.add(&result.work);
        if result.sequence.is_some() {
            return PlanResult {
                sequence: result.sequence,
                predicted: result.predicted,
                work,
            };
        }
    }
    PlanResult {
        sequence: None,
        predicted: None,
        work,
    }
}

#[derive(Clone, Debug, Default)]
struct AggregateWork {
    cases: usize,
    correct: usize,
    shortest: usize,
    real_execution_correct: usize,
    partial_generated: usize,
    partial_scored: usize,
    partial_expanded: usize,
    model_applications: usize,
    complete_candidates: usize,
    heuristic_evaluations: usize,
    model_work: usize,
    heuristic_work: usize,
    signature_update_work: usize,
    value_retrieval_work: usize,
    local_activation_spikes: usize,
    signature_mismatches: usize,
    total_work: usize,
}

impl AggregateWork {
    fn record(
        &mut self,
        result: &PlanResult,
        expected_depth: Option<usize>,
        initial: &TemporaryStructure,
        goal: &SearchGoal,
        world: &SearchWorld,
    ) {
        self.cases += 1;
        let correct = match expected_depth {
            Some(depth) => result
                .sequence
                .as_ref()
                .is_some_and(|sequence| sequence.len() == depth),
            None => result.sequence.is_none(),
        };
        self.correct += usize::from(correct);
        self.shortest += usize::from(correct);
        if let Some(sequence) = &result.sequence {
            let real = sequence.iter().fold(initial.clone(), |state, &action| {
                world.apply(action, &state)
            });
            self.real_execution_correct += usize::from(
                result.predicted.as_ref() == Some(&real) && goal.distinguishes(initial, &real),
            );
        } else if expected_depth.is_none() {
            self.real_execution_correct += 1;
        }
        self.partial_generated += result.work.partial_generated;
        self.partial_scored += result.work.partial_scored;
        self.partial_expanded += result.work.partial_expanded;
        self.model_applications += result.work.model_applications;
        self.complete_candidates += result.work.complete_candidates;
        self.heuristic_evaluations += result.work.heuristic_evaluations;
        self.model_work += result.work.model_work;
        self.heuristic_work += result.work.heuristic_work;
        self.signature_update_work += result.work.signature_update_work;
        self.value_retrieval_work += result.work.value_retrieval_work;
        self.local_activation_spikes += result.work.local_activation_spikes;
        self.signature_mismatches += result.work.signature_mismatches;
        self.total_work += result.work.total_work();
    }
}

#[derive(Clone, Debug)]
pub struct SearchDepthReport {
    pub required_depth: usize,
    pub cases: usize,
    pub exhaustive_generated: usize,
    pub learned_generated: usize,
    pub learned_scored: usize,
    pub exhaustive_expanded: usize,
    pub learned_expanded: usize,
    pub random_expanded: usize,
    pub oracle_expanded: usize,
    pub shuffled_expanded: usize,
    pub memorizer_expanded: usize,
    pub exhaustive_complete: usize,
    pub learned_complete: usize,
    pub exhaustive_model_applications: usize,
    pub learned_model_applications: usize,
    pub learned_heuristic_evaluations: usize,
    pub learned_model_work: usize,
    pub learned_heuristic_work: usize,
    pub exhaustive_total_work: usize,
    pub learned_total_work: usize,
    pub learned_correct: usize,
    pub learned_shortest: usize,
    pub real_execution_correct: usize,
}

#[derive(Clone, Debug)]
pub struct TrainingCheckpointReport {
    pub examples: usize,
    pub heuristic_entries: usize,
    pub held_out_expanded: usize,
    pub held_out_model_applications: usize,
    pub held_out_total_work: usize,
}

#[derive(Clone, Debug)]
pub struct SearchValueReport {
    pub identity_invariant: bool,
    pub goal_conditional: bool,
    pub checkpoints: Vec<TrainingCheckpointReport>,
    pub entries_plateau: bool,
    pub mature_entries: usize,
    pub depth_reports: Vec<SearchDepthReport>,
    pub reachable_exhaustive_expanded: usize,
    pub reachable_learned_expanded: usize,
    pub reachable_exhaustive_model_applications: usize,
    pub reachable_learned_model_applications: usize,
    pub reachable_exhaustive_total_work: usize,
    pub reachable_learned_total_work: usize,
    pub economic_advantage_demonstrated: bool,
    pub unreachable_exhaustive_expanded: usize,
    pub unreachable_learned_expanded: usize,
    pub unreachable_exhaustive_total_work: usize,
    pub unreachable_learned_total_work: usize,
    pub unreachable_correct: usize,
    pub shuffled_no_advantage: bool,
    pub memorizer_no_transfer: bool,
    pub frozen_fingerprint_unchanged: bool,
    pub permanent_action_model_entries: usize,
    pub passed: bool,
}

#[derive(Clone, Debug)]
pub struct CompiledValueDepthReport {
    pub required_depth: usize,
    pub cases: usize,
    pub exhaustive_expanded: usize,
    pub current_expanded: usize,
    pub compiled_expanded: usize,
    pub local_expanded: usize,
    pub zero_cost_expanded: usize,
    pub exhaustive_total_work: usize,
    pub current_total_work: usize,
    pub compiled_total_work: usize,
    pub local_total_work: usize,
    pub zero_cost_total_work: usize,
    pub compiled_signature_work: usize,
    pub compiled_retrieval_work: usize,
    pub local_signature_work: usize,
    pub local_activation_spikes: usize,
    pub signature_mismatches: usize,
}

#[derive(Clone, Debug)]
pub struct CompiledSearchValueReport {
    pub learned_entries: usize,
    pub compiled_entries: usize,
    pub local_cells: usize,
    pub local_arrows: usize,
    pub value_checks: usize,
    pub values_identical: bool,
    pub signatures_path_independent: bool,
    pub convergent_paths_checked: usize,
    pub collision_free: bool,
    pub search_order_identical: bool,
    pub signature_mismatches: usize,
    pub depth_reports: Vec<CompiledValueDepthReport>,
    pub reachable_cases: usize,
    pub reachable_exhaustive_total_work: usize,
    pub reachable_current_total_work: usize,
    pub reachable_compiled_total_work: usize,
    pub reachable_local_total_work: usize,
    pub reachable_zero_cost_total_work: usize,
    pub reachable_compiled_signature_work: usize,
    pub reachable_compiled_retrieval_work: usize,
    pub reachable_local_signature_work: usize,
    pub reachable_local_activation_spikes: usize,
    pub unreachable_cases: usize,
    pub unreachable_exhaustive_total_work: usize,
    pub unreachable_current_total_work: usize,
    pub unreachable_compiled_total_work: usize,
    pub unreachable_local_total_work: usize,
    pub direct_compilation_work: usize,
    pub local_compilation_work: usize,
    pub direct_saving_per_problem: usize,
    pub local_saving_per_problem: usize,
    pub direct_break_even_problems: usize,
    pub local_break_even_problems: usize,
    pub frozen_fingerprint_unchanged: bool,
    pub economic_advantage_demonstrated: bool,
    pub passed: bool,
}

fn evaluate_policy(
    learner: &SearchValueLearner,
    shuffled: &SearchValueLearner,
    memorized_paths: &[Vec<OpaqueAction>],
) -> (
    Vec<SearchDepthReport>,
    AggregateWork,
    AggregateWork,
    bool,
    usize,
) {
    let mut by_depth = DEPTHS
        .iter()
        .map(|&required_depth| {
            (
                required_depth,
                AggregateWork::default(),
                AggregateWork::default(),
                AggregateWork::default(),
                AggregateWork::default(),
                AggregateWork::default(),
                AggregateWork::default(),
            )
        })
        .collect::<Vec<_>>();
    let mut unreachable_exhaustive = AggregateWork::default();
    let mut unreachable_learned = AggregateWork::default();
    let mut frozen = true;
    let mut permanent_entries = 0;

    for seed_index in 0..EVALUATION_SEEDS {
        let seed = 0x5600_0000 + seed_index as u64;
        let (world, action_learner) = train_action_models(seed);
        let models = (0..ACTION_COUNT)
            .map(|index| {
                let action = world.action(index);
                (action, action_learner.predict(action).unwrap())
            })
            .collect::<BTreeMap<_, _>>();
        let fingerprint = action_learner.fingerprint();
        permanent_entries = action_learner.model_entries();

        for (required_depth, exhaustive, learned, random, oracle, shuffled_work, memorizer) in
            &mut by_depth
        {
            let mut identities = IdentitySource::new(seed ^ 0x5700_0000 ^ *required_depth as u64);
            let initial = initial_state(&mut identities, Some(*required_depth));
            let goal = SearchGoal::reachable();
            let exhaustive_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::Exhaustive,
            );
            let learned_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::Learned(learner),
            );
            let random_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::Random(seed ^ *required_depth as u64),
            );
            let oracle_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::Oracle(&world.effect_for_action),
            );
            let shuffled_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::Shuffled(shuffled),
            );
            let memorizer_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::Memorized(memorized_paths),
            );
            exhaustive.record(
                &exhaustive_result,
                Some(*required_depth),
                &initial,
                &goal,
                &world,
            );
            learned.record(
                &learned_result,
                Some(*required_depth),
                &initial,
                &goal,
                &world,
            );
            random.record(
                &random_result,
                Some(*required_depth),
                &initial,
                &goal,
                &world,
            );
            oracle.record(
                &oracle_result,
                Some(*required_depth),
                &initial,
                &goal,
                &world,
            );
            shuffled_work.record(
                &shuffled_result,
                Some(*required_depth),
                &initial,
                &goal,
                &world,
            );
            memorizer.record(
                &memorizer_result,
                Some(*required_depth),
                &initial,
                &goal,
                &world,
            );
        }

        let mut identities = IdentitySource::new(seed ^ 0x5800_0000);
        let initial = initial_state(&mut identities, None);
        let goal = SearchGoal::unreachable();
        let exhaustive_result =
            plan_shortest(&models, &initial, &goal, 8, PriorityPolicy::Exhaustive);
        let learned_result = plan_shortest(
            &models,
            &initial,
            &goal,
            8,
            PriorityPolicy::Learned(learner),
        );
        unreachable_exhaustive.record(&exhaustive_result, None, &initial, &goal, &world);
        unreachable_learned.record(&learned_result, None, &initial, &goal, &world);
        frozen &= action_learner.fingerprint() == fingerprint;
    }

    let depth_reports = by_depth
        .into_iter()
        .map(
            |(required_depth, exhaustive, learned, random, oracle, shuffled, memorizer)| {
                SearchDepthReport {
                    required_depth,
                    cases: learned.cases,
                    exhaustive_generated: exhaustive.partial_generated,
                    learned_generated: learned.partial_generated,
                    learned_scored: learned.partial_scored,
                    exhaustive_expanded: exhaustive.partial_expanded,
                    learned_expanded: learned.partial_expanded,
                    random_expanded: random.partial_expanded,
                    oracle_expanded: oracle.partial_expanded,
                    shuffled_expanded: shuffled.partial_expanded,
                    memorizer_expanded: memorizer.partial_expanded,
                    exhaustive_complete: exhaustive.complete_candidates,
                    learned_complete: learned.complete_candidates,
                    exhaustive_model_applications: exhaustive.model_applications,
                    learned_model_applications: learned.model_applications,
                    learned_heuristic_evaluations: learned.heuristic_evaluations,
                    learned_model_work: learned.model_work,
                    learned_heuristic_work: learned.heuristic_work,
                    exhaustive_total_work: exhaustive.total_work,
                    learned_total_work: learned.total_work,
                    learned_correct: learned.correct,
                    learned_shortest: learned.shortest,
                    real_execution_correct: learned.real_execution_correct,
                }
            },
        )
        .collect();

    (
        depth_reports,
        unreachable_exhaustive,
        unreachable_learned,
        frozen,
        permanent_entries,
    )
}

fn audit_value_equivalence(
    learner: &SearchValueLearner,
    compiled: &CompiledValueTable,
    local: &LocalValueNetwork,
) -> (usize, bool) {
    let mut checks = 0;
    let mut identical = true;
    for seed_index in 0..EVALUATION_SEEDS {
        let seed = 0x5b00_0000 + seed_index as u64;
        let (world, action_learner) = train_action_models(seed);
        let models = (0..ACTION_COUNT)
            .map(|index| {
                let action = world.action(index);
                (action, action_learner.predict(action).unwrap())
            })
            .collect::<BTreeMap<_, _>>();
        for required_depth in DEPTHS {
            let mut identities = IdentitySource::new(seed ^ 0x5c00_0000 ^ required_depth as u64);
            let initial = initial_state(&mut identities, Some(required_depth));
            let mut states = BTreeMap::from([(initial.occupants.clone(), initial)]);
            for _ in 0..=8 {
                for state in states.values() {
                    let signature = StateSignature::from_state(state);
                    for goal in [SearchGoal::reachable(), SearchGoal::unreachable()] {
                        for remaining in 0..=8 {
                            let current = learner.predict(state, &goal, remaining);
                            let direct = compiled.predict(signature, &goal, remaining);
                            let activated = local.predict(signature, &goal, remaining);
                            checks += 1;
                            identical &= current == direct && direct == activated;
                        }
                    }
                }
                let new_states = states
                    .values()
                    .flat_map(|state| {
                        models
                            .values()
                            .map(|transformation| transformation.apply(state))
                    })
                    .collect::<Vec<_>>();
                for state in new_states {
                    states.entry(state.occupants.clone()).or_insert(state);
                }
            }
        }
    }
    (checks, identical)
}

fn audit_path_independence() -> (bool, usize) {
    let (world, action_learner) = train_action_models(0x5d00_0000);
    let models = (0..ACTION_COUNT)
        .map(|index| {
            let action = world.action(index);
            (action, action_learner.predict(action).unwrap())
        })
        .collect::<BTreeMap<_, _>>();
    let mut identities = IdentitySource::new(0x5e00_0000);
    let initial = initial_state(&mut identities, Some(8));
    let initial_signature = StateSignature::from_state(&initial);
    let mut frontier = vec![(Vec::<OpaqueAction>::new(), initial, initial_signature)];
    let mut path_independent = true;
    let mut convergent_paths = 0;

    for depth in 1..=8 {
        let mut next = Vec::new();
        let mut reached =
            BTreeMap::<Vec<OpaqueIdentity>, (StateSignature, Vec<OpaqueAction>)>::new();
        for (sequence, state, signature) in frontier {
            for (&action, transformation) in &models {
                let predicted = transformation.apply(&state);
                let predicted_signature = signature.apply(transformation);
                path_independent &= predicted_signature == StateSignature::from_state(&predicted);
                let mut next_sequence = sequence.clone();
                next_sequence.push(action);
                if let Some((previous_signature, previous_sequence)) =
                    reached.get(&predicted.occupants)
                {
                    if previous_sequence != &next_sequence {
                        convergent_paths += 1;
                        path_independent &= previous_signature == &predicted_signature;
                    }
                } else {
                    reached.insert(
                        predicted.occupants.clone(),
                        (predicted_signature, next_sequence.clone()),
                    );
                }
                next.push((next_sequence, predicted, predicted_signature));
            }
        }
        frontier = next;
        if depth == 8 {
            break;
        }
    }
    (path_independent, convergent_paths)
}

type CompiledDepthAggregates = (
    usize,
    AggregateWork,
    AggregateWork,
    AggregateWork,
    AggregateWork,
    AggregateWork,
);

fn evaluate_compiled_policies(
    learner: &SearchValueLearner,
    compiled: &CompiledValueTable,
    local: &LocalValueNetwork,
) -> (
    Vec<CompiledValueDepthReport>,
    AggregateWork,
    AggregateWork,
    AggregateWork,
    AggregateWork,
    bool,
    bool,
    usize,
) {
    let mut by_depth = DEPTHS
        .iter()
        .map(|&depth| {
            (
                depth,
                AggregateWork::default(),
                AggregateWork::default(),
                AggregateWork::default(),
                AggregateWork::default(),
                AggregateWork::default(),
            )
        })
        .collect::<Vec<CompiledDepthAggregates>>();
    let mut unreachable_exhaustive = AggregateWork::default();
    let mut unreachable_current = AggregateWork::default();
    let mut unreachable_compiled = AggregateWork::default();
    let mut unreachable_local = AggregateWork::default();
    let mut ordering_identical = true;
    let mut frozen = true;
    let mut permanent_entries = 0;

    for seed_index in 0..EVALUATION_SEEDS {
        let seed = 0x5600_0000 + seed_index as u64;
        let (world, action_learner) = train_action_models(seed);
        let models = (0..ACTION_COUNT)
            .map(|index| {
                let action = world.action(index);
                (action, action_learner.predict(action).unwrap())
            })
            .collect::<BTreeMap<_, _>>();
        let fingerprint = action_learner.fingerprint();
        permanent_entries = action_learner.model_entries();

        for (required_depth, exhaustive, current, direct, activated, zero_cost) in &mut by_depth {
            let mut identities = IdentitySource::new(seed ^ 0x5700_0000 ^ *required_depth as u64);
            let initial = initial_state(&mut identities, Some(*required_depth));
            let goal = SearchGoal::reachable();
            let exhaustive_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::Exhaustive,
            );
            let current_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::Learned(learner),
            );
            let compiled_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::Compiled(compiled),
            );
            let local_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::Local(local),
            );
            let zero_result = plan_shortest(
                &models,
                &initial,
                &goal,
                *required_depth,
                PriorityPolicy::ZeroCost(compiled),
            );
            ordering_identical &= current_result.sequence == compiled_result.sequence
                && compiled_result.sequence == local_result.sequence
                && local_result.sequence == zero_result.sequence
                && current_result.predicted == compiled_result.predicted
                && compiled_result.predicted == local_result.predicted
                && current_result.work.partial_expanded == compiled_result.work.partial_expanded
                && compiled_result.work.partial_expanded == local_result.work.partial_expanded
                && local_result.work.partial_expanded == zero_result.work.partial_expanded
                && current_result.work.complete_candidates
                    == compiled_result.work.complete_candidates
                && compiled_result.work.complete_candidates
                    == local_result.work.complete_candidates
                && local_result.work.complete_candidates == zero_result.work.complete_candidates;
            exhaustive.record(
                &exhaustive_result,
                Some(*required_depth),
                &initial,
                &goal,
                &world,
            );
            current.record(
                &current_result,
                Some(*required_depth),
                &initial,
                &goal,
                &world,
            );
            direct.record(
                &compiled_result,
                Some(*required_depth),
                &initial,
                &goal,
                &world,
            );
            activated.record(
                &local_result,
                Some(*required_depth),
                &initial,
                &goal,
                &world,
            );
            zero_cost.record(&zero_result, Some(*required_depth), &initial, &goal, &world);
        }

        let mut identities = IdentitySource::new(seed ^ 0x5800_0000);
        let initial = initial_state(&mut identities, None);
        let goal = SearchGoal::unreachable();
        let exhaustive_result =
            plan_shortest(&models, &initial, &goal, 8, PriorityPolicy::Exhaustive);
        let current_result = plan_shortest(
            &models,
            &initial,
            &goal,
            8,
            PriorityPolicy::Learned(learner),
        );
        let compiled_result = plan_shortest(
            &models,
            &initial,
            &goal,
            8,
            PriorityPolicy::Compiled(compiled),
        );
        let local_result = plan_shortest(&models, &initial, &goal, 8, PriorityPolicy::Local(local));
        ordering_identical &= current_result.sequence == compiled_result.sequence
            && compiled_result.sequence == local_result.sequence
            && current_result.work.partial_expanded == compiled_result.work.partial_expanded
            && compiled_result.work.partial_expanded == local_result.work.partial_expanded;
        unreachable_exhaustive.record(&exhaustive_result, None, &initial, &goal, &world);
        unreachable_current.record(&current_result, None, &initial, &goal, &world);
        unreachable_compiled.record(&compiled_result, None, &initial, &goal, &world);
        unreachable_local.record(&local_result, None, &initial, &goal, &world);
        frozen &= action_learner.fingerprint() == fingerprint;
    }

    let depth_reports = by_depth
        .into_iter()
        .map(
            |(required_depth, exhaustive, current, compiled, local, zero_cost)| {
                CompiledValueDepthReport {
                    required_depth,
                    cases: current.cases,
                    exhaustive_expanded: exhaustive.partial_expanded,
                    current_expanded: current.partial_expanded,
                    compiled_expanded: compiled.partial_expanded,
                    local_expanded: local.partial_expanded,
                    zero_cost_expanded: zero_cost.partial_expanded,
                    exhaustive_total_work: exhaustive.total_work,
                    current_total_work: current.total_work,
                    compiled_total_work: compiled.total_work,
                    local_total_work: local.total_work,
                    zero_cost_total_work: zero_cost.total_work,
                    compiled_signature_work: compiled.signature_update_work,
                    compiled_retrieval_work: compiled.value_retrieval_work,
                    local_signature_work: local.signature_update_work,
                    local_activation_spikes: local.local_activation_spikes,
                    signature_mismatches: compiled.signature_mismatches
                        + local.signature_mismatches
                        + zero_cost.signature_mismatches,
                }
            },
        )
        .collect();

    (
        depth_reports,
        unreachable_exhaustive,
        unreachable_current,
        unreachable_compiled,
        unreachable_local,
        ordering_identical,
        frozen,
        permanent_entries,
    )
}

fn divide_rounding_up(numerator: usize, denominator: usize) -> usize {
    if numerator == 0 {
        0
    } else if denominator == 0 {
        usize::MAX
    } else {
        numerator.div_ceil(denominator)
    }
}

pub fn run_compiled_experiment() -> CompiledSearchValueReport {
    let (snapshots, _, _) = train_value_models(*TRAINING_CHECKPOINTS.last().unwrap());
    let learner = snapshots[TRAINING_CHECKPOINTS.last().unwrap()].clone();
    let original_fingerprint = learner.fingerprint();
    let compiled = CompiledValueTable::compile(&learner);
    let local = LocalValueNetwork::compile(&compiled);
    let (value_checks, values_identical) = audit_value_equivalence(&learner, &compiled, &local);
    let (signatures_path_independent, convergent_paths_checked) = audit_path_independence();
    let (
        depth_reports,
        unreachable_exhaustive,
        unreachable_current,
        unreachable_compiled,
        unreachable_local,
        search_order_identical,
        action_models_frozen,
        _,
    ) = evaluate_compiled_policies(&learner, &compiled, &local);

    let reachable_cases = depth_reports.iter().map(|depth| depth.cases).sum::<usize>();
    let reachable_exhaustive_total_work = depth_reports
        .iter()
        .map(|depth| depth.exhaustive_total_work)
        .sum::<usize>();
    let reachable_current_total_work = depth_reports
        .iter()
        .map(|depth| depth.current_total_work)
        .sum::<usize>();
    let reachable_compiled_total_work = depth_reports
        .iter()
        .map(|depth| depth.compiled_total_work)
        .sum::<usize>();
    let reachable_local_total_work = depth_reports
        .iter()
        .map(|depth| depth.local_total_work)
        .sum::<usize>();
    let reachable_zero_cost_total_work = depth_reports
        .iter()
        .map(|depth| depth.zero_cost_total_work)
        .sum::<usize>();

    let compiled_saving =
        reachable_exhaustive_total_work.saturating_sub(reachable_compiled_total_work);
    let local_saving = reachable_exhaustive_total_work.saturating_sub(reachable_local_total_work);
    let direct_saving_per_problem = compiled_saving / reachable_cases;
    let local_saving_per_problem = local_saving / reachable_cases;
    let direct_break_even_problems =
        divide_rounding_up(compiled.compilation_work, direct_saving_per_problem);
    let local_break_even_problems =
        divide_rounding_up(local.compilation_work, local_saving_per_problem);
    let signature_mismatches = depth_reports
        .iter()
        .map(|depth| depth.signature_mismatches)
        .sum::<usize>()
        + unreachable_compiled.signature_mismatches
        + unreachable_local.signature_mismatches;
    let reachable_compiled_signature_work = depth_reports
        .iter()
        .map(|depth| depth.compiled_signature_work)
        .sum::<usize>();
    let reachable_compiled_retrieval_work = depth_reports
        .iter()
        .map(|depth| depth.compiled_retrieval_work)
        .sum::<usize>();
    let reachable_local_signature_work = depth_reports
        .iter()
        .map(|depth| depth.local_signature_work)
        .sum::<usize>();
    let reachable_local_activation_spikes = depth_reports
        .iter()
        .map(|depth| depth.local_activation_spikes)
        .sum();
    let economic_advantage_demonstrated = reachable_compiled_total_work
        < reachable_exhaustive_total_work
        && reachable_local_total_work < reachable_exhaustive_total_work;
    let frozen_fingerprint_unchanged =
        action_models_frozen && learner.fingerprint() == original_fingerprint;
    let passed = values_identical
        && signatures_path_independent
        && convergent_paths_checked > 0
        && compiled.collision_free
        && search_order_identical
        && signature_mismatches == 0
        && economic_advantage_demonstrated
        && unreachable_compiled.correct == unreachable_compiled.cases
        && unreachable_local.correct == unreachable_local.cases
        && unreachable_compiled.partial_expanded == unreachable_exhaustive.partial_expanded
        && unreachable_local.partial_expanded == unreachable_exhaustive.partial_expanded
        && frozen_fingerprint_unchanged;

    CompiledSearchValueReport {
        learned_entries: learner.entries(),
        compiled_entries: compiled.entries(),
        local_cells: local.cells,
        local_arrows: local.arrows.len(),
        value_checks,
        values_identical,
        signatures_path_independent,
        convergent_paths_checked,
        collision_free: compiled.collision_free,
        search_order_identical,
        signature_mismatches,
        depth_reports,
        reachable_cases,
        reachable_exhaustive_total_work,
        reachable_current_total_work,
        reachable_compiled_total_work,
        reachable_local_total_work,
        reachable_zero_cost_total_work,
        reachable_compiled_signature_work,
        reachable_compiled_retrieval_work,
        reachable_local_signature_work,
        reachable_local_activation_spikes,
        unreachable_cases: unreachable_current.cases,
        unreachable_exhaustive_total_work: unreachable_exhaustive.total_work,
        unreachable_current_total_work: unreachable_current.total_work,
        unreachable_compiled_total_work: unreachable_compiled.total_work,
        unreachable_local_total_work: unreachable_local.total_work,
        direct_compilation_work: compiled.compilation_work,
        local_compilation_work: local.compilation_work,
        direct_saving_per_problem,
        local_saving_per_problem,
        direct_break_even_problems,
        local_break_even_problems,
        frozen_fingerprint_unchanged,
        economic_advantage_demonstrated,
        passed,
    }
}

pub fn run_experiment() -> SearchValueReport {
    let (snapshots, shuffled, memorized_paths) =
        train_value_models(*TRAINING_CHECKPOINTS.last().unwrap());
    let mature = snapshots[TRAINING_CHECKPOINTS.last().unwrap()].clone();

    let mut checkpoint_reports = Vec::new();
    for checkpoint in TRAINING_CHECKPOINTS {
        let learner = &snapshots[&checkpoint];
        let (depths, _, _, _, _) = evaluate_policy(learner, &shuffled, &memorized_paths);
        checkpoint_reports.push(TrainingCheckpointReport {
            examples: checkpoint,
            heuristic_entries: learner.entries(),
            held_out_expanded: depths.iter().map(|depth| depth.learned_expanded).sum(),
            held_out_model_applications: depths
                .iter()
                .map(|depth| depth.learned_model_applications)
                .sum(),
            held_out_total_work: depths.iter().map(|depth| depth.learned_total_work).sum(),
        });
    }

    let mature_fingerprint = mature.fingerprint();
    let (depth_reports, unreachable_exhaustive, unreachable_learned, frozen, action_entries) =
        evaluate_policy(&mature, &shuffled, &memorized_paths);

    let mut identities_a = IdentitySource::new(0x5900_0000);
    let mut identities_b = IdentitySource::new(0x5900_0001);
    let state_a = initial_state(&mut identities_a, Some(4));
    let state_b = initial_state(&mut identities_b, Some(4));
    let identity_invariant = value_key(&state_a, &SearchGoal::reachable(), 4)
        == value_key(&state_b, &SearchGoal::reachable(), 4);
    let world = SearchWorld::new(0x5a00_0000);
    let reachable_value = value_from_minimum(minimum_remaining_steps(
        &world.effect_for_action,
        &state_a,
        &SearchGoal::reachable(),
        4,
    ));
    let unreachable_value = value_from_minimum(minimum_remaining_steps(
        &world.effect_for_action,
        &state_a,
        &SearchGoal::unreachable(),
        4,
    ));
    let goal_conditional = reachable_value != unreachable_value;
    let entries_plateau = checkpoint_reports
        .iter()
        .rev()
        .take(2)
        .map(|checkpoint| checkpoint.heuristic_entries)
        .collect::<BTreeSet<_>>()
        .len()
        == 1;
    let shuffled_no_advantage = depth_reports
        .iter()
        .map(|depth| depth.shuffled_expanded)
        .sum::<usize>()
        >= depth_reports
            .iter()
            .map(|depth| depth.learned_expanded)
            .sum::<usize>();
    let memorizer_no_transfer = depth_reports
        .iter()
        .map(|depth| depth.memorizer_expanded)
        .sum::<usize>()
        >= depth_reports
            .iter()
            .map(|depth| depth.learned_expanded)
            .sum::<usize>();
    let all_cases = EVALUATION_SEEDS;
    let reachable_exhaustive_expanded = depth_reports
        .iter()
        .map(|depth| depth.exhaustive_expanded)
        .sum();
    let reachable_learned_expanded = depth_reports
        .iter()
        .map(|depth| depth.learned_expanded)
        .sum();
    let reachable_exhaustive_model_applications = depth_reports
        .iter()
        .map(|depth| depth.exhaustive_model_applications)
        .sum();
    let reachable_learned_model_applications = depth_reports
        .iter()
        .map(|depth| depth.learned_model_applications)
        .sum();
    let reachable_exhaustive_total_work = depth_reports
        .iter()
        .map(|depth| depth.exhaustive_total_work)
        .sum();
    let reachable_learned_total_work = depth_reports
        .iter()
        .map(|depth| depth.learned_total_work)
        .sum();
    let economic_advantage_demonstrated =
        reachable_learned_total_work < reachable_exhaustive_total_work;
    let passed = identity_invariant
        && goal_conditional
        && entries_plateau
        && depth_reports.iter().all(|depth| {
            depth.learned_correct == all_cases
                && depth.learned_shortest == all_cases
                && depth.real_execution_correct == all_cases
                && depth.learned_expanded <= depth.exhaustive_expanded
        })
        && depth_reports
            .last()
            .is_some_and(|depth| depth.learned_expanded < depth.exhaustive_expanded)
        && unreachable_exhaustive.correct == all_cases
        && unreachable_learned.correct == all_cases
        && unreachable_learned.partial_expanded == unreachable_exhaustive.partial_expanded
        && shuffled_no_advantage
        && memorizer_no_transfer
        && frozen;

    SearchValueReport {
        identity_invariant,
        goal_conditional,
        checkpoints: checkpoint_reports,
        entries_plateau,
        mature_entries: mature.entries(),
        depth_reports,
        reachable_exhaustive_expanded,
        reachable_learned_expanded,
        reachable_exhaustive_model_applications,
        reachable_learned_model_applications,
        reachable_exhaustive_total_work,
        reachable_learned_total_work,
        economic_advantage_demonstrated,
        unreachable_exhaustive_expanded: unreachable_exhaustive.partial_expanded,
        unreachable_learned_expanded: unreachable_learned.partial_expanded,
        unreachable_exhaustive_total_work: unreachable_exhaustive.total_work,
        unreachable_learned_total_work: unreachable_learned.total_work,
        unreachable_correct: unreachable_learned.correct,
        shuffled_no_advantage,
        memorizer_no_transfer,
        frozen_fingerprint_unchanged: frozen && mature.fingerprint() == mature_fingerprint,
        permanent_action_model_entries: action_entries,
        passed,
    }
}

pub fn print_report(report: &SearchValueReport) {
    println!("s0/s1 learned search value and lazy supplied search:");
    println!(
        "  identity invariant={}, goal conditional={}, heuristic entries={}, plateau={}",
        report.identity_invariant,
        report.goal_conditional,
        report.mature_entries,
        report.entries_plateau
    );
    for checkpoint in &report.checkpoints {
        println!(
            "  training examples {:4}: entries={}, held-out expanded={}, applications={}, total work={}",
            checkpoint.examples,
            checkpoint.heuristic_entries,
            checkpoint.held_out_expanded,
            checkpoint.held_out_model_applications,
            checkpoint.held_out_total_work
        );
    }
    println!(
        "  reachable aggregate expanded={}/{}, applications={}/{}, total work={}/{}, economic advantage={}",
        report.reachable_exhaustive_expanded,
        report.reachable_learned_expanded,
        report.reachable_exhaustive_model_applications,
        report.reachable_learned_model_applications,
        report.reachable_exhaustive_total_work,
        report.reachable_learned_total_work,
        report.economic_advantage_demonstrated
    );
    for depth in &report.depth_reports {
        println!(
            "  depth {}: generated/scored/expanded learned={:.1}/{:.1}/{:.1}, exhaustive expanded={:.1}, random/oracle/shuffled/memorizer={:.1}/{:.1}/{:.1}/{:.1}, complete={:.1}/{:.1}, model/heuristic work={:.1}/{:.1}, total work={:.1}/{:.1}",
            depth.required_depth,
            depth.learned_generated as f64 / depth.cases as f64,
            depth.learned_scored as f64 / depth.cases as f64,
            depth.learned_expanded as f64 / depth.cases as f64,
            depth.exhaustive_expanded as f64 / depth.cases as f64,
            depth.random_expanded as f64 / depth.cases as f64,
            depth.oracle_expanded as f64 / depth.cases as f64,
            depth.shuffled_expanded as f64 / depth.cases as f64,
            depth.memorizer_expanded as f64 / depth.cases as f64,
            depth.exhaustive_complete as f64 / depth.cases as f64,
            depth.learned_complete as f64 / depth.cases as f64,
            depth.learned_model_work as f64 / depth.cases as f64,
            depth.learned_heuristic_work as f64 / depth.cases as f64,
            depth.exhaustive_total_work as f64 / depth.cases as f64,
            depth.learned_total_work as f64 / depth.cases as f64
        );
    }
    println!(
        "  unreachable expanded exhaustive/learned={}/{}, total work={}/{}, correct={}/{}, frozen={}",
        report.unreachable_exhaustive_expanded,
        report.unreachable_learned_expanded,
        report.unreachable_exhaustive_total_work,
        report.unreachable_learned_total_work,
        report.unreachable_correct,
        EVALUATION_SEEDS,
        report.frozen_fingerprint_unchanged
    );
}

pub fn print_compiled_report(report: &CompiledSearchValueReport) {
    println!("s1.1 compiled search value:");
    println!(
        "  entries learned/compiled={}/{}, local cells/arrows={}/{}, value checks={}, identical={}, collision free={}",
        report.learned_entries,
        report.compiled_entries,
        report.local_cells,
        report.local_arrows,
        report.value_checks,
        report.values_identical,
        report.collision_free
    );
    println!(
        "  path independent={}, convergent paths={}, ordering identical={}, signature mismatches={}",
        report.signatures_path_independent,
        report.convergent_paths_checked,
        report.search_order_identical,
        report.signature_mismatches
    );
    println!(
        "  reachable total work exhaustive/current/compiled/local/zero={}/{}/{}/{}/{}, economic advantage={}",
        report.reachable_exhaustive_total_work,
        report.reachable_current_total_work,
        report.reachable_compiled_total_work,
        report.reachable_local_total_work,
        report.reachable_zero_cost_total_work,
        report.economic_advantage_demonstrated
    );
    println!(
        "  compiled signature/retrieval work={}/{}, local signature/spikes={}/{}, compilation direct/local={}/{}, break even={}/{} problems",
        report.reachable_compiled_signature_work,
        report.reachable_compiled_retrieval_work,
        report.reachable_local_signature_work,
        report.reachable_local_activation_spikes,
        report.direct_compilation_work,
        report.local_compilation_work,
        report.direct_break_even_problems,
        report.local_break_even_problems
    );
    for depth in &report.depth_reports {
        println!(
            "  depth {}: expanded exhaustive/guided={:.1}/{:.1}, total work exhaustive/current/compiled/local/zero={:.1}/{:.1}/{:.1}/{:.1}/{:.1}",
            depth.required_depth,
            depth.exhaustive_expanded as f64 / depth.cases as f64,
            depth.compiled_expanded as f64 / depth.cases as f64,
            depth.exhaustive_total_work as f64 / depth.cases as f64,
            depth.current_total_work as f64 / depth.cases as f64,
            depth.compiled_total_work as f64 / depth.cases as f64,
            depth.local_total_work as f64 / depth.cases as f64,
            depth.zero_cost_total_work as f64 / depth.cases as f64
        );
    }
    println!(
        "  unreachable total work exhaustive/current/compiled/local={}/{}/{}/{}, correct={}",
        report.unreachable_exhaustive_total_work,
        report.unreachable_current_total_work,
        report.unreachable_compiled_total_work,
        report.unreachable_local_total_work,
        report.unreachable_cases
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn report() -> &'static SearchValueReport {
        static REPORT: OnceLock<SearchValueReport> = OnceLock::new();
        REPORT.get_or_init(run_experiment)
    }

    fn compiled_report() -> &'static CompiledSearchValueReport {
        static REPORT: OnceLock<CompiledSearchValueReport> = OnceLock::new();
        REPORT.get_or_init(run_compiled_experiment)
    }

    #[test]
    fn s0_value_is_identity_invariant_and_goal_conditional() {
        let report = report();
        assert!(report.identity_invariant);
        assert!(report.goal_conditional);
        assert!(report.entries_plateau);
    }

    #[test]
    fn s1_preserves_accuracy_shortest_plans_and_real_execution() {
        let report = report();
        assert!(report.depth_reports.iter().all(|depth| {
            depth.learned_correct == depth.cases
                && depth.learned_shortest == depth.cases
                && depth.real_execution_correct == depth.cases
        }));
        assert!(report.frozen_fingerprint_unchanged);
    }

    #[test]
    fn s1_reduces_held_out_search_work_after_learning() {
        let report = report();
        let first = report.checkpoints.first().unwrap();
        let mature = report.checkpoints.last().unwrap();
        assert!(mature.held_out_expanded < first.held_out_expanded);
        assert!(mature.held_out_model_applications < first.held_out_model_applications);
    }

    #[test]
    fn s1_controls_reject_shuffled_values_and_path_memory() {
        let report = report();
        assert!(report.shuffled_no_advantage);
        assert!(report.memorizer_no_transfer);
        assert!(report.passed);
    }

    #[test]
    fn s1_unreachable_search_remains_complete_and_more_expensive() {
        let report = report();
        assert_eq!(
            report.unreachable_learned_expanded,
            report.unreachable_exhaustive_expanded
        );
        assert!(report.unreachable_learned_total_work > report.unreachable_exhaustive_total_work);
        assert!(!report.economic_advantage_demonstrated);
    }

    #[test]
    fn s1_1_compiled_values_are_exact_and_path_independent() {
        let report = compiled_report();
        assert!(report.values_identical);
        assert!(report.collision_free);
        assert!(report.signatures_path_independent);
        assert!(report.convergent_paths_checked > 0);
        assert_eq!(report.signature_mismatches, 0);
    }

    #[test]
    fn s1_1_all_value_paths_preserve_search_order() {
        let report = compiled_report();
        assert!(report.search_order_identical);
        assert!(report.depth_reports.iter().all(|depth| {
            depth.current_expanded == depth.compiled_expanded
                && depth.compiled_expanded == depth.local_expanded
                && depth.local_expanded == depth.zero_cost_expanded
        }));
        assert!(report.frozen_fingerprint_unchanged);
    }

    #[test]
    fn s1_1_compilation_makes_guided_reachable_search_cheaper() {
        let report = compiled_report();
        assert!(report.reachable_compiled_total_work < report.reachable_exhaustive_total_work);
        assert!(report.reachable_local_total_work < report.reachable_exhaustive_total_work);
        assert!(report.economic_advantage_demonstrated);
        assert!(report.direct_break_even_problems > 0);
        assert!(report.local_break_even_problems > 0);
    }

    #[test]
    fn s1_1_unreachable_search_remains_complete_with_overhead() {
        let report = compiled_report();
        assert!(report.unreachable_compiled_total_work > report.unreachable_exhaustive_total_work);
        assert!(report.unreachable_local_total_work > report.unreachable_exhaustive_total_work);
        assert!(report.passed);
    }
}
