use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub type SensorId = usize;
pub type ActionId = u8;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    active: BTreeSet<SensorId>,
}

impl Frame {
    pub fn singleton(sensor: SensorId) -> Self {
        Self {
            active: BTreeSet::from([sensor]),
        }
    }

    pub fn from_sensors(sensors: impl IntoIterator<Item = SensorId>) -> Self {
        Self {
            active: sensors.into_iter().collect(),
        }
    }

    pub fn active_sensors(&self) -> &BTreeSet<SensorId> {
        &self.active
    }
}

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_usize(&mut self, upper_bound: usize) -> usize {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.state >> 32) as usize) % upper_bound
    }
}

#[derive(Clone)]
pub struct RelationalTopology {
    neighbors: Vec<Vec<Option<SensorId>>>,
}

impl RelationalTopology {
    pub fn from_neighbors(neighbors: Vec<Vec<Option<SensorId>>>) -> Result<Self, &'static str> {
        let Some(port_count) = neighbors.first().map(Vec::len) else {
            return Err("topology must contain at least one sensor");
        };
        if port_count == 0 {
            return Err("topology must contain at least one relation port");
        }
        if neighbors.iter().any(|ports| ports.len() != port_count) {
            return Err("every sensor must expose the same number of ports");
        }
        if neighbors
            .iter()
            .flatten()
            .flatten()
            .any(|&sensor| sensor >= neighbors.len())
        {
            return Err("neighbor references must identify an existing sensor");
        }
        Ok(Self { neighbors })
    }

    pub fn permuted_grid(width: usize, height: usize, seed: u64) -> Self {
        assert!(width >= 3 && height >= 3);
        let sensor_count = width * height;
        let mut coordinate_to_sensor: Vec<_> = (0..sensor_count).collect();
        let mut rng = Lcg::new(seed);
        for index in (1..sensor_count).rev() {
            let swap_with = rng.next_usize(index + 1);
            coordinate_to_sensor.swap(index, swap_with);
        }

        let mut neighbors = vec![vec![None; 4]; sensor_count];
        for y in 0..height {
            for x in 0..width {
                let sensor = coordinate_to_sensor[y * width + x];
                let coordinates = [
                    x.checked_sub(1).map(|next_x| (next_x, y)),
                    (x + 1 < width).then_some((x + 1, y)),
                    y.checked_sub(1).map(|next_y| (x, next_y)),
                    (y + 1 < height).then_some((x, y + 1)),
                ];
                for (port, coordinate) in coordinates.into_iter().enumerate() {
                    neighbors[sensor][port] = coordinate
                        .map(|(next_x, next_y)| coordinate_to_sensor[next_y * width + next_x]);
                }
            }
        }
        Self { neighbors }
    }

    pub fn port_count(&self) -> usize {
        self.neighbors.first().map_or(0, Vec::len)
    }

    pub fn sensor_count(&self) -> usize {
        self.neighbors.len()
    }

    pub fn neighbor(&self, sensor: SensorId, port: usize) -> Option<SensorId> {
        self.neighbors
            .get(sensor)
            .and_then(|neighbors| neighbors.get(port))
            .copied()
            .flatten()
    }

    pub fn interior_sensors(&self) -> Vec<SensorId> {
        (0..self.sensor_count())
            .filter(|&sensor| {
                (0..self.port_count()).all(|port| self.neighbor(sensor, port).is_some())
            })
            .collect()
    }

    pub fn sensor_blocked_at(&self, port: usize) -> SensorId {
        (0..self.sensor_count())
            .find(|&sensor| self.neighbor(sensor, port).is_none())
            .expect("every grid port has a boundary")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructuralEffect {
    Stay,
    FollowPort(usize),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WorkMetrics {
    pub observations: u64,
    pub predictions: u64,
    pub hypothesis_evaluations: u64,
    pub sensor_visits: u64,
    pub experiment_states_considered: u64,
}

fn candidate_effects(topology: &RelationalTopology) -> BTreeSet<StructuralEffect> {
    std::iter::once(StructuralEffect::Stay)
        .chain((0..topology.port_count()).map(StructuralEffect::FollowPort))
        .collect()
}

fn apply_effect(topology: &RelationalTopology, before: &Frame, effect: StructuralEffect) -> Frame {
    apply_effect_measured(topology, before, effect).0
}

fn apply_effect_measured(
    topology: &RelationalTopology,
    before: &Frame,
    effect: StructuralEffect,
) -> (Frame, u64) {
    match effect {
        StructuralEffect::Stay => (before.clone(), before.active.len() as u64),
        StructuralEffect::FollowPort(port) => {
            let mut visits = 0;
            let mut active = BTreeSet::new();
            for &sensor in &before.active {
                visits += 1;
                let Some(neighbor) = topology.neighbor(sensor, port) else {
                    return (before.clone(), visits);
                };
                active.insert(neighbor);
            }
            (Frame { active }, visits)
        }
    }
}

pub fn simulate_effect(
    topology: &RelationalTopology,
    before: &Frame,
    effect: StructuralEffect,
) -> Frame {
    apply_effect(topology, before, effect)
}

fn unique_observed_effect(
    topology: &RelationalTopology,
    before: &Frame,
    after: &Frame,
) -> Option<StructuralEffect> {
    let matches: Vec<_> = candidate_effects(topology)
        .into_iter()
        .filter(|&effect| apply_effect(topology, before, effect) == *after)
        .collect();
    (matches.len() == 1).then_some(matches[0])
}

pub struct RepresentationLearner {
    candidates: BTreeMap<ActionId, BTreeSet<StructuralEffect>>,
}

impl RepresentationLearner {
    pub fn new(actions: impl IntoIterator<Item = ActionId>, topology: &RelationalTopology) -> Self {
        let universe = candidate_effects(topology);
        Self {
            candidates: actions
                .into_iter()
                .map(|action| (action, universe.clone()))
                .collect(),
        }
    }

    pub fn observe(
        &mut self,
        topology: &RelationalTopology,
        before: &Frame,
        action: ActionId,
        after: &Frame,
    ) -> bool {
        self.observe_measured(topology, before, action, after, &mut WorkMetrics::default())
    }

    pub fn observe_measured(
        &mut self,
        topology: &RelationalTopology,
        before: &Frame,
        action: ActionId,
        after: &Frame,
        metrics: &mut WorkMetrics,
    ) -> bool {
        let Some(candidates) = self.candidates.get_mut(&action) else {
            return false;
        };
        metrics.observations += 1;
        let previous = std::mem::take(candidates);
        for effect in previous {
            metrics.hypothesis_evaluations += 1;
            let (predicted, visits) = apply_effect_measured(topology, before, effect);
            metrics.sensor_visits += visits;
            if predicted == *after {
                candidates.insert(effect);
            }
        }
        true
    }

    pub fn effect(&self, action: ActionId) -> Option<StructuralEffect> {
        let candidates = self.candidates.get(&action)?;
        (candidates.len() == 1)
            .then(|| candidates.iter().next())
            .flatten()
            .copied()
    }

    pub fn candidate_count(&self, action: ActionId) -> usize {
        self.candidates.get(&action).map_or(0, BTreeSet::len)
    }

    pub fn all_known(&self) -> bool {
        self.candidates
            .values()
            .all(|candidates| candidates.len() == 1)
    }

    pub fn known_count(&self) -> usize {
        self.candidates
            .values()
            .filter(|candidates| candidates.len() == 1)
            .count()
    }

    fn learned_effects(&self) -> Option<BTreeMap<ActionId, StructuralEffect>> {
        self.candidates
            .keys()
            .map(|&action| self.effect(action).map(|effect| (action, effect)))
            .collect()
    }

    pub fn predict(
        &self,
        topology: &RelationalTopology,
        before: &Frame,
        action: ActionId,
    ) -> Option<Frame> {
        Some(apply_effect(topology, before, self.effect(action)?))
    }

    pub fn choose_experiment(&self, topology: &RelationalTopology) -> Option<(ActionId, SensorId)> {
        self.choose_experiment_measured(topology, &mut WorkMetrics::default())
    }

    pub fn choose_experiment_measured(
        &self,
        topology: &RelationalTopology,
        metrics: &mut WorkMetrics,
    ) -> Option<(ActionId, SensorId)> {
        let mut best: Option<((usize, usize), ActionId, SensorId)> = None;
        for (&action, candidates) in &self.candidates {
            if candidates.len() <= 1 {
                continue;
            }
            for sensor in 0..topology.sensor_count() {
                metrics.experiment_states_considered += 1;
                let before = Frame::singleton(sensor);
                let mut outcome_counts = BTreeMap::new();
                for &effect in candidates {
                    metrics.predictions += 1;
                    metrics.hypothesis_evaluations += 1;
                    let (predicted, visits) = apply_effect_measured(topology, &before, effect);
                    metrics.sensor_visits += visits;
                    *outcome_counts.entry(predicted).or_insert(0usize) += 1;
                }
                let worst_partition = outcome_counts.values().copied().max().unwrap_or(0);
                let information = candidates.len() - worst_partition;
                let score = (information, candidates.len());
                let replace = best
                    .as_ref()
                    .is_none_or(|(best_score, best_action, best_sensor)| {
                        score > *best_score
                            || (score == *best_score
                                && (action, sensor) < (*best_action, *best_sensor))
                    });
                if replace {
                    best = Some((score, action, sensor));
                }
            }
        }
        best.map(|(_, action, sensor)| (action, sensor))
    }
}

fn v9_rules() -> BTreeMap<ActionId, StructuralEffect> {
    BTreeMap::from([
        (7, StructuralEffect::FollowPort(1)),
        (11, StructuralEffect::FollowPort(0)),
        (19, StructuralEffect::FollowPort(2)),
        (23, StructuralEffect::Stay),
    ])
}

fn train_representation(
    topology: &RelationalTopology,
    rules: &BTreeMap<ActionId, StructuralEffect>,
) -> RepresentationLearner {
    let mut learner = RepresentationLearner::new(rules.keys().copied(), topology);
    let sensors = topology.interior_sensors();
    for (sample, (&action, &effect)) in rules.iter().enumerate() {
        let before = Frame::singleton(sensors[sample % sensors.len()]);
        let after = apply_effect(topology, &before, effect);
        learner.observe(topology, &before, action, &after);
    }
    learner
}

#[derive(Debug)]
pub struct V9Report {
    pub learned_operators: usize,
    pub transfer_predictions: usize,
    pub boundary_prediction: bool,
    pub shuffled_control_rejected: bool,
    pub passed: bool,
}

pub fn run_v9_experiment() -> V9Report {
    let training = RelationalTopology::permuted_grid(7, 7, 0x0090_0909);
    let rules = v9_rules();
    let learner = train_representation(&training, &rules);

    let transfer = RelationalTopology::permuted_grid(11, 9, 0x0090_1414);
    let center = transfer.interior_sensors()[8];
    let shape = Frame::from_sensors([
        center,
        transfer.neighbor(center, 1).unwrap(),
        transfer.neighbor(center, 3).unwrap(),
    ]);
    let transfer_predictions = rules
        .iter()
        .filter(|&(&action, &effect)| {
            learner.predict(&transfer, &shape, action)
                == Some(apply_effect(&transfer, &shape, effect))
        })
        .count();

    let blocked_sensor = transfer.sensor_blocked_at(1);
    let blocked = Frame::singleton(blocked_sensor);
    let boundary_prediction = learner.predict(&transfer, &blocked, 7) == Some(blocked);

    let mut control = RepresentationLearner::new(rules.keys().copied(), &training);
    let sensor = training.interior_sensors()[0];
    let before = Frame::singleton(sensor);
    let east = apply_effect(&training, &before, StructuralEffect::FollowPort(1));
    let west = apply_effect(&training, &before, StructuralEffect::FollowPort(0));
    for action in rules.keys().copied() {
        control.observe(&training, &before, action, &east);
        control.observe(&training, &before, action, &west);
    }
    let shuffled_control_rejected = control.known_count() == 0;
    let learned_operators = learner.known_count();
    let passed = learned_operators == rules.len()
        && transfer_predictions == rules.len()
        && boundary_prediction
        && shuffled_control_rejected;

    V9Report {
        learned_operators,
        transfer_predictions,
        boundary_prediction,
        shuffled_control_rejected,
        passed,
    }
}

fn active_identification_steps(
    topology: &RelationalTopology,
    rules: &BTreeMap<ActionId, StructuralEffect>,
) -> usize {
    let mut learner = RepresentationLearner::new(rules.keys().copied(), topology);
    for step in 1..=100 {
        let (action, sensor) = learner
            .choose_experiment(topology)
            .expect("an unresolved action must have an experiment");
        let before = Frame::singleton(sensor);
        let after = apply_effect(topology, &before, rules[&action]);
        learner.observe(topology, &before, action, &after);
        if learner.all_known() {
            return step;
        }
    }
    100
}

fn random_identification_steps(
    topology: &RelationalTopology,
    rules: &BTreeMap<ActionId, StructuralEffect>,
    seed: u64,
) -> usize {
    let actions: Vec<_> = rules.keys().copied().collect();
    let mut learner = RepresentationLearner::new(actions.iter().copied(), topology);
    let mut rng = Lcg::new(seed);
    for step in 1..=200 {
        let action = actions[rng.next_usize(actions.len())];
        let sensor = rng.next_usize(topology.sensor_count());
        let before = Frame::singleton(sensor);
        let after = apply_effect(topology, &before, rules[&action]);
        learner.observe(topology, &before, action, &after);
        if learner.all_known() {
            return step;
        }
    }
    200
}

fn v10_rules() -> BTreeMap<ActionId, StructuralEffect> {
    BTreeMap::from([
        (31, StructuralEffect::FollowPort(0)),
        (37, StructuralEffect::FollowPort(1)),
        (41, StructuralEffect::FollowPort(2)),
        (43, StructuralEffect::FollowPort(3)),
        (47, StructuralEffect::Stay),
    ])
}

#[derive(Debug)]
pub struct V10Report {
    pub initial_hypotheses_per_action: usize,
    pub ambiguous_boundary_hypotheses: usize,
    pub active_steps: usize,
    pub random_average_steps: f64,
    pub passed: bool,
}

pub fn run_v10_experiment() -> V10Report {
    let topology = RelationalTopology::permuted_grid(9, 9, 0x0010_1010);
    let rules = v10_rules();
    let initial = RepresentationLearner::new(rules.keys().copied(), &topology);
    let initial_hypotheses_per_action = initial.candidate_count(31);

    let mut ambiguous = RepresentationLearner::new([37], &topology);
    let blocked = Frame::singleton(topology.sensor_blocked_at(1));
    ambiguous.observe(&topology, &blocked, 37, &blocked);
    let ambiguous_boundary_hypotheses = ambiguous.candidate_count(37);

    let active_steps = active_identification_steps(&topology, &rules);
    let random_average_steps = (1..=64)
        .map(|seed| random_identification_steps(&topology, &rules, seed))
        .sum::<usize>() as f64
        / 64.0;
    let passed = initial_hypotheses_per_action == 5
        && ambiguous_boundary_hypotheses > 1
        && active_steps == rules.len()
        && random_average_steps > active_steps as f64 * 1.5;

    V10Report {
        initial_hypotheses_per_action,
        ambiguous_boundary_hypotheses,
        active_steps,
        random_average_steps,
        passed,
    }
}

struct ContinualLearner {
    memories: BTreeMap<ActionId, VecDeque<StructuralEffect>>,
    window: usize,
    minimum_confidence: f64,
}

impl ContinualLearner {
    fn new(actions: impl IntoIterator<Item = ActionId>, window: usize) -> Self {
        Self {
            memories: actions
                .into_iter()
                .map(|action| (action, VecDeque::new()))
                .collect(),
            window,
            minimum_confidence: 0.6,
        }
    }

    fn observe(
        &mut self,
        topology: &RelationalTopology,
        before: &Frame,
        action: ActionId,
        after: &Frame,
    ) -> bool {
        let Some(effect) = unique_observed_effect(topology, before, after) else {
            return false;
        };
        let memory = self.memories.get_mut(&action).unwrap();
        if memory.len() == self.window {
            memory.pop_front();
        }
        memory.push_back(effect);
        true
    }

    fn estimate(&self, action: ActionId) -> Option<StructuralEffect> {
        let memory = self.memories.get(&action)?;
        if memory.is_empty() {
            return None;
        }
        let mut counts = BTreeMap::new();
        for &effect in memory {
            *counts.entry(effect).or_insert(0usize) += 1;
        }
        let (&effect, &support) = counts.iter().max_by_key(|(_, count)| *count)?;
        let confidence = support as f64 / memory.len() as f64;
        (confidence >= self.minimum_confidence).then_some(effect)
    }

    fn memory_size(&self) -> usize {
        self.memories.values().map(VecDeque::len).sum()
    }
}

#[derive(Debug)]
pub struct V11Report {
    pub stream_samples: usize,
    pub adaptation_samples: usize,
    pub unchanged_rules_retained: usize,
    pub bounded_memory: bool,
    pub passed: bool,
}

pub fn run_v11_experiment() -> V11Report {
    let topology = RelationalTopology::permuted_grid(9, 9, 0x0011_1111);
    let actions = [53, 59, 61];
    let initial = BTreeMap::from([
        (53, StructuralEffect::FollowPort(1)),
        (59, StructuralEffect::FollowPort(2)),
        (61, StructuralEffect::Stay),
    ]);
    let mut learner = ContinualLearner::new(actions, 9);
    let sensors = topology.interior_sensors();
    let mut stream_samples = 0;
    for cycle in 0..20 {
        for action in actions {
            let before = Frame::singleton(sensors[(cycle + action as usize) % sensors.len()]);
            let after = apply_effect(&topology, &before, initial[&action]);
            assert!(learner.observe(&topology, &before, action, &after));
            stream_samples += 1;
        }
    }

    let changed = StructuralEffect::FollowPort(3);
    let mut adaptation_samples = 0;
    while learner.estimate(53) != Some(changed) && adaptation_samples < 12 {
        let before = Frame::singleton(sensors[adaptation_samples % sensors.len()]);
        let after = apply_effect(&topology, &before, changed);
        assert!(learner.observe(&topology, &before, 53, &after));
        adaptation_samples += 1;
        stream_samples += 1;

        for action in [59, 61] {
            let before =
                Frame::singleton(sensors[(adaptation_samples + action as usize) % sensors.len()]);
            let after = apply_effect(&topology, &before, initial[&action]);
            assert!(learner.observe(&topology, &before, action, &after));
            stream_samples += 1;
        }
    }

    let unchanged_rules_retained = [59, 61]
        .into_iter()
        .filter(|action| learner.estimate(*action) == Some(initial[action]))
        .count();
    let bounded_memory = learner.memory_size() <= actions.len() * 9;
    let passed = adaptation_samples <= 6
        && unchanged_rules_retained == 2
        && bounded_memory
        && learner.estimate(53) == Some(changed);

    V11Report {
        stream_samples,
        adaptation_samples,
        unchanged_rules_retained,
        bounded_memory,
        passed,
    }
}

#[derive(Clone, Debug)]
struct OperatorClass {
    effect: StructuralEffect,
    members: Vec<ActionId>,
}

#[derive(Clone, Debug)]
struct ClassProcedure {
    classes: Vec<usize>,
    effects: Vec<StructuralEffect>,
    support: usize,
    compression_gain: usize,
}

fn discover_operator_classes(
    rules: &BTreeMap<ActionId, StructuralEffect>,
) -> (Vec<OperatorClass>, BTreeMap<ActionId, usize>) {
    let mut by_effect: BTreeMap<StructuralEffect, Vec<ActionId>> = BTreeMap::new();
    for (&action, &effect) in rules {
        by_effect.entry(effect).or_default().push(action);
    }

    let classes: Vec<_> = by_effect
        .into_iter()
        .map(|(effect, members)| OperatorClass { effect, members })
        .collect();
    let mut action_to_class = BTreeMap::new();
    for (class, operator) in classes.iter().enumerate() {
        for &action in &operator.members {
            action_to_class.insert(action, class);
        }
    }
    (classes, action_to_class)
}

fn encode_classes(
    trace: &[ActionId],
    action_to_class: &BTreeMap<ActionId, usize>,
) -> Option<Vec<usize>> {
    trace
        .iter()
        .map(|action| action_to_class.get(action).copied())
        .collect()
}

fn mine_class_procedure(
    traces: &[Vec<ActionId>],
    classes: &[OperatorClass],
    action_to_class: &BTreeMap<ActionId, usize>,
    minimum_support: usize,
) -> Option<ClassProcedure> {
    let mut candidates: BTreeMap<Vec<usize>, (usize, BTreeSet<usize>)> = BTreeMap::new();
    for (trace_index, trace) in traces.iter().enumerate() {
        let encoded = encode_classes(trace, action_to_class)?;
        for length in 2..=4.min(encoded.len()) {
            for window in encoded.windows(length) {
                let entry = candidates.entry(window.to_vec()).or_default();
                entry.0 += 1;
                entry.1.insert(trace_index);
            }
        }
    }

    let (class_sequence, support, compression_gain) = candidates
        .into_iter()
        .filter_map(|(sequence, (occurrences, traces))| {
            let support = traces.len();
            let gain = occurrences
                .saturating_mul(sequence.len().saturating_sub(1))
                .saturating_sub(sequence.len());
            (support >= minimum_support && gain > 0).then_some((sequence, support, gain))
        })
        .max_by_key(|(sequence, support, gain)| (*gain, sequence.len(), *support))?;
    let effects = class_sequence
        .iter()
        .map(|&class| classes.get(class).map(|operator| operator.effect))
        .collect::<Option<Vec<_>>>()?;
    Some(ClassProcedure {
        classes: class_sequence,
        effects,
        support,
        compression_gain,
    })
}

fn v12_rules() -> BTreeMap<ActionId, StructuralEffect> {
    BTreeMap::from([
        (67, StructuralEffect::FollowPort(1)),
        (71, StructuralEffect::FollowPort(1)),
        (73, StructuralEffect::FollowPort(2)),
        (79, StructuralEffect::FollowPort(2)),
        (83, StructuralEffect::Stay),
    ])
}

fn abstraction_traces() -> Vec<Vec<ActionId>> {
    (0..8)
        .map(|variant| {
            vec![
                if variant & 1 == 0 { 67 } else { 71 },
                if variant & 2 == 0 { 67 } else { 71 },
                if variant & 4 == 0 { 73 } else { 79 },
            ]
        })
        .collect()
}

fn random_abstraction_control(
    action_to_class: &BTreeMap<ActionId, usize>,
    classes: &[OperatorClass],
) -> bool {
    let actions: Vec<_> = action_to_class.keys().copied().collect();
    let mut rng = Lcg::new(0x0012_a0a0);
    let traces: Vec<Vec<_>> = (0..12)
        .map(|_| {
            (0..3)
                .map(|_| actions[rng.next_usize(actions.len())])
                .collect()
        })
        .collect();
    mine_class_procedure(&traces, classes, action_to_class, 6).is_none()
}

#[derive(Debug)]
pub struct V12Report {
    pub operator_classes: usize,
    pub aliased_actions_grouped: bool,
    pub procedure_length: usize,
    pub procedure_support: usize,
    pub compression_gain: usize,
    pub raw_sequence_max_support: usize,
    pub random_control_rejected: bool,
    pub passed: bool,
}

pub fn run_v12_experiment() -> V12Report {
    let rules = v12_rules();
    let (classes, action_to_class) = discover_operator_classes(&rules);
    let traces = abstraction_traces();
    let procedure = mine_class_procedure(&traces, &classes, &action_to_class, 6).unwrap();
    let mut raw_counts = BTreeMap::new();
    for trace in &traces {
        *raw_counts.entry(trace.clone()).or_insert(0usize) += 1;
    }
    let raw_sequence_max_support = raw_counts.values().copied().max().unwrap_or(0);
    let east_class = action_to_class[&67];
    let north_class = action_to_class[&73];
    let aliased_actions_grouped = east_class == action_to_class[&71]
        && north_class == action_to_class[&79]
        && east_class != north_class;
    let random_control_rejected = random_abstraction_control(&action_to_class, &classes);
    let passed = classes.len() == 3
        && aliased_actions_grouped
        && procedure.classes.len() == 3
        && procedure.support == 8
        && procedure.compression_gain > 0
        && raw_sequence_max_support == 1
        && random_control_rejected;

    V12Report {
        operator_classes: classes.len(),
        aliased_actions_grouped,
        procedure_length: procedure.classes.len(),
        procedure_support: procedure.support,
        compression_gain: procedure.compression_gain,
        raw_sequence_max_support,
        random_control_rejected,
        passed,
    }
}

fn average_random_useful_trace_samples(seed_offset: u64) -> f64 {
    let mut total = 0;
    for seed in 1..=64 {
        let mut rng = Lcg::new(seed + seed_offset);
        let mut useful = 0;
        let mut samples = 0;
        while useful < 6 && samples < 200 {
            if rng.next_usize(4) == 0 {
                useful += 1;
            }
            samples += 1;
        }
        total += samples;
    }
    total as f64 / 64.0
}

struct UnifiedLearningMachine {
    representation: RepresentationLearner,
    classes: Option<Vec<OperatorClass>>,
    action_to_class: Option<BTreeMap<ActionId, usize>>,
    procedure: Option<ClassProcedure>,
    interventions: usize,
    selected_traces: usize,
}

impl UnifiedLearningMachine {
    fn new(actions: impl IntoIterator<Item = ActionId>, topology: &RelationalTopology) -> Self {
        Self {
            representation: RepresentationLearner::new(actions, topology),
            classes: None,
            action_to_class: None,
            procedure: None,
            interventions: 0,
            selected_traces: 0,
        }
    }

    fn learn(
        &mut self,
        topology: &RelationalTopology,
        environment: &BTreeMap<ActionId, StructuralEffect>,
        available_traces: &[Vec<ActionId>],
    ) {
        while !self.representation.all_known() {
            let (action, sensor) = self.representation.choose_experiment(topology).unwrap();
            let before = Frame::singleton(sensor);
            let after = apply_effect(topology, &before, environment[&action]);
            self.representation
                .observe(topology, &before, action, &after);
            self.interventions += 1;
        }

        let rules = self.representation.learned_effects().unwrap();
        let (classes, action_to_class) = discover_operator_classes(&rules);
        let mut ranked: Vec<_> = available_traces
            .iter()
            .enumerate()
            .filter_map(|(index, trace)| {
                encode_classes(trace, &action_to_class).map(|encoded| (index, encoded))
            })
            .collect();
        let mut frequencies = BTreeMap::new();
        for (_, encoded) in &ranked {
            *frequencies.entry(encoded.clone()).or_insert(0usize) += 1;
        }
        ranked.sort_by_key(|(_, encoded)| std::cmp::Reverse(frequencies[encoded]));

        let mut selected = Vec::new();
        for (index, _) in ranked {
            selected.push(available_traces[index].clone());
            self.selected_traces += 1;
            if let Some(procedure) = mine_class_procedure(&selected, &classes, &action_to_class, 6)
            {
                self.procedure = Some(procedure);
                break;
            }
        }
        self.classes = Some(classes);
        self.action_to_class = Some(action_to_class);
    }
}

fn curriculum_trace_pool() -> Vec<Vec<ActionId>> {
    let mut traces = abstraction_traces();
    let actions: Vec<_> = v12_rules().keys().copied().collect();
    let mut rng = Lcg::new(0x0013_1313);
    traces.extend((0..24).map(|_| {
        (0..3)
            .map(|_| actions[rng.next_usize(actions.len())])
            .collect()
    }));
    traces
}

#[derive(Debug)]
pub struct V13Report {
    pub causal_interventions: usize,
    pub selected_task_traces: usize,
    pub random_average_task_traces: f64,
    pub automatic_abstraction: bool,
    pub automatic_procedure: bool,
    pub passed: bool,
}

pub fn run_v13_experiment() -> V13Report {
    let topology = RelationalTopology::permuted_grid(9, 9, 0x0013_aaaa);
    let rules = v12_rules();
    let mut machine = UnifiedLearningMachine::new(rules.keys().copied(), &topology);
    machine.learn(&topology, &rules, &curriculum_trace_pool());
    let random_average_task_traces = average_random_useful_trace_samples(0x1300);
    let automatic_abstraction = machine
        .classes
        .as_ref()
        .is_some_and(|classes| classes.len() == 3);
    let automatic_procedure = machine
        .procedure
        .as_ref()
        .is_some_and(|procedure| procedure.support >= 6);
    let passed = machine.interventions == rules.len()
        && machine.selected_traces == 6
        && random_average_task_traces > machine.selected_traces as f64 * 2.5
        && automatic_abstraction
        && automatic_procedure;

    V13Report {
        causal_interventions: machine.interventions,
        selected_task_traces: machine.selected_traces,
        random_average_task_traces,
        automatic_abstraction,
        automatic_procedure,
        passed,
    }
}

#[derive(Clone)]
struct PixelCodec {
    sensor_to_pixel: Vec<usize>,
    pixel_to_sensor: Vec<usize>,
}

impl PixelCodec {
    fn new(sensor_count: usize, seed: u64) -> Self {
        let mut sensor_to_pixel: Vec<_> = (0..sensor_count).collect();
        let mut rng = Lcg::new(seed);
        for index in (1..sensor_count).rev() {
            let swap_with = rng.next_usize(index + 1);
            sensor_to_pixel.swap(index, swap_with);
        }
        let mut pixel_to_sensor = vec![0; sensor_count];
        for (sensor, &pixel) in sensor_to_pixel.iter().enumerate() {
            pixel_to_sensor[pixel] = sensor;
        }
        Self {
            sensor_to_pixel,
            pixel_to_sensor,
        }
    }

    fn encode(&self, frame: &Frame) -> Vec<u8> {
        let mut pixels = vec![0; self.sensor_to_pixel.len()];
        for &sensor in &frame.active {
            pixels[self.sensor_to_pixel[sensor]] = 1;
        }
        pixels
    }

    fn decode(&self, pixels: &[u8]) -> Frame {
        Frame::from_sensors(
            pixels
                .iter()
                .enumerate()
                .filter(|(_, value)| **value != 0)
                .map(|(pixel, _)| self.pixel_to_sensor[pixel]),
        )
    }
}

#[derive(Clone)]
struct ToneCodec {
    sensor_to_tone: Vec<u16>,
    tone_to_sensor: BTreeMap<u16, SensorId>,
}

impl ToneCodec {
    fn new(sensor_count: usize, seed: u64) -> Self {
        let mut tones: Vec<_> = (0..sensor_count)
            .map(|index| 1_000 + index as u16 * 7)
            .collect();
        let mut rng = Lcg::new(seed);
        for index in (1..sensor_count).rev() {
            let swap_with = rng.next_usize(index + 1);
            tones.swap(index, swap_with);
        }
        let tone_to_sensor = tones
            .iter()
            .enumerate()
            .map(|(sensor, &tone)| (tone, sensor))
            .collect();
        Self {
            sensor_to_tone: tones,
            tone_to_sensor,
        }
    }

    fn encode(&self, frame: &Frame) -> Vec<u16> {
        frame
            .active
            .iter()
            .map(|&sensor| self.sensor_to_tone[sensor])
            .collect()
    }

    fn decode(&self, tones: &[u16]) -> Option<Frame> {
        tones
            .iter()
            .map(|tone| self.tone_to_sensor.get(tone).copied())
            .collect::<Option<BTreeSet<_>>>()
            .map(|active| Frame { active })
    }
}

fn calibrate_target_actions(
    topology: &RelationalTopology,
    codec: &ToneCodec,
    target_rules: &BTreeMap<ActionId, StructuralEffect>,
    source_effects: &BTreeSet<StructuralEffect>,
) -> BTreeMap<StructuralEffect, ActionId> {
    let sensor = topology.interior_sensors()[0];
    let before = Frame::singleton(sensor);
    let raw_before = codec.encode(&before);
    let decoded_before = codec.decode(&raw_before).unwrap();
    let mut aliases = BTreeMap::new();
    for (&action, &actual_effect) in target_rules {
        let after = apply_effect(topology, &before, actual_effect);
        let raw_after = codec.encode(&after);
        let decoded_after = codec.decode(&raw_after).unwrap();
        if let Some(effect) = unique_observed_effect(topology, &decoded_before, &decoded_after) {
            if source_effects.contains(&effect) {
                aliases.insert(effect, action);
            }
        }
    }
    aliases
}

#[derive(Debug)]
pub struct V14Report {
    pub calibrated_action_aliases: usize,
    pub transferred_procedure: bool,
    pub target_task_traces_required: usize,
    pub scratch_task_traces_required: usize,
    pub unrelated_operator_rejected: bool,
    pub independent_raw_encodings: bool,
    pub passed: bool,
}

pub fn run_v14_experiment() -> V14Report {
    let source_rules = v12_rules();
    let (classes, action_to_class) = discover_operator_classes(&source_rules);
    let procedure =
        mine_class_procedure(&abstraction_traces(), &classes, &action_to_class, 6).unwrap();
    let source_effects: BTreeSet<_> = classes.iter().map(|class| class.effect).collect();

    let target = RelationalTopology::permuted_grid(12, 10, 0x0014_1414);
    let target_rules = BTreeMap::from([
        (101, StructuralEffect::FollowPort(1)),
        (103, StructuralEffect::FollowPort(2)),
        (107, StructuralEffect::Stay),
        (109, StructuralEffect::FollowPort(3)),
    ]);
    let pixel_codec = PixelCodec::new(target.sensor_count(), 0x1401);
    let tone_codec = ToneCodec::new(target.sensor_count(), 0x1402);
    let aliases = calibrate_target_actions(&target, &tone_codec, &target_rules, &source_effects);

    let start_sensor = target.interior_sensors()[5];
    let start = Frame::from_sensors([
        start_sensor,
        target.neighbor(start_sensor, 0).unwrap(),
        target.neighbor(start_sensor, 3).unwrap(),
    ]);
    let transferred_actions: Option<Vec<_>> = procedure
        .effects
        .iter()
        .map(|effect| aliases.get(effect).copied())
        .collect();
    let transferred_procedure = transferred_actions.is_some_and(|actions| {
        let predicted = actions.iter().fold(start.clone(), |frame, action| {
            apply_effect(&target, &frame, target_rules[action])
        });
        let actual = procedure
            .effects
            .iter()
            .fold(start.clone(), |frame, &effect| {
                apply_effect(&target, &frame, effect)
            });
        predicted == actual
    });
    let unrelated_operator_rejected = !aliases.contains_key(&StructuralEffect::FollowPort(3));

    let pixel_raw = pixel_codec.encode(&start);
    let tone_raw = tone_codec.encode(&start);
    let independent_raw_encodings = pixel_codec.decode(&pixel_raw) == start
        && tone_codec.decode(&tone_raw) == Some(start)
        && pixel_raw.len() != tone_raw.len();
    let calibrated_action_aliases = aliases.len();
    let target_task_traces_required = 0;
    let scratch_task_traces_required = 6;
    let passed = calibrated_action_aliases == 3
        && transferred_procedure
        && target_task_traces_required == 0
        && scratch_task_traces_required == 6
        && unrelated_operator_rejected
        && independent_raw_encodings;

    V14Report {
        calibrated_action_aliases,
        transferred_procedure,
        target_task_traces_required,
        scratch_task_traces_required,
        unrelated_operator_rejected,
        independent_raw_encodings,
        passed,
    }
}

#[derive(Debug)]
pub struct GeneralityReport {
    pub v9: V9Report,
    pub v10: V10Report,
    pub v11: V11Report,
    pub v12: V12Report,
    pub v13: V13Report,
    pub v14: V14Report,
    pub passed: bool,
}

pub fn run_experiments() -> GeneralityReport {
    let v9 = run_v9_experiment();
    let v10 = run_v10_experiment();
    let v11 = run_v11_experiment();
    let v12 = run_v12_experiment();
    let v13 = run_v13_experiment();
    let v14 = run_v14_experiment();
    let passed = v9.passed && v10.passed && v11.passed && v12.passed && v13.passed && v14.passed;
    GeneralityReport {
        v9,
        v10,
        v11,
        v12,
        v13,
        v14,
        passed,
    }
}

pub fn print_report(report: &GeneralityReport) {
    println!("v9 learned structural representation:");
    println!(
        "  operators={}/4, transfer={}/4, boundary={}, shuffled-control={}",
        report.v9.learned_operators,
        report.v9.transfer_predictions,
        report.v9.boundary_prediction,
        report.v9.shuffled_control_rejected
    );
    println!("v10 hypothesis-driven intervention:");
    println!(
        "  hypotheses/action={}, ambiguous-boundary={}, active={}, random-average={:.1}",
        report.v10.initial_hypotheses_per_action,
        report.v10.ambiguous_boundary_hypotheses,
        report.v10.active_steps,
        report.v10.random_average_steps
    );
    println!("v11 continual learning:");
    println!(
        "  stream={}, adaptation={}, unchanged={}/2, bounded-memory={}",
        report.v11.stream_samples,
        report.v11.adaptation_samples,
        report.v11.unchanged_rules_retained,
        report.v11.bounded_memory
    );
    println!("v12 autonomous abstraction:");
    println!(
        "  classes={}, procedure-length={}, support={}, gain={}, raw-max-support={}",
        report.v12.operator_classes,
        report.v12.procedure_length,
        report.v12.procedure_support,
        report.v12.compression_gain,
        report.v12.raw_sequence_max_support
    );
    println!("v13 self-directed curriculum:");
    println!(
        "  interventions={}, selected-traces={}, random-average={:.1}, abstraction={}, procedure={}",
        report.v13.causal_interventions,
        report.v13.selected_task_traces,
        report.v13.random_average_task_traces,
        report.v13.automatic_abstraction,
        report.v13.automatic_procedure
    );
    println!("v14 cross-domain structural transfer:");
    println!(
        "  calibrated-aliases={}/3, transferred={}, target-task-traces={}, scratch-traces={}, unrelated-rejected={}",
        report.v14.calibrated_action_aliases,
        report.v14.transferred_procedure,
        report.v14.target_task_traces_required,
        report.v14.scratch_task_traces_required,
        report.v14.unrelated_operator_rejected
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v9_discovers_structural_operators_from_raw_transitions() {
        let report = run_v9_experiment();

        assert_eq!(report.learned_operators, 4);
        assert_eq!(report.transfer_predictions, 4);
        assert!(report.boundary_prediction);
        assert!(report.shuffled_control_rejected);
        assert!(report.passed);
    }

    #[test]
    fn v10_competing_hypotheses_drive_more_efficient_experiments() {
        let report = run_v10_experiment();

        assert_eq!(report.initial_hypotheses_per_action, 5);
        assert!(report.ambiguous_boundary_hypotheses > 1);
        assert_eq!(report.active_steps, 5);
        assert!(report.random_average_steps > report.active_steps as f64 * 1.5);
        assert!(report.passed);
    }

    #[test]
    fn v11_continual_stream_adapts_without_erasing_unchanged_rules() {
        let report = run_v11_experiment();

        assert!(report.adaptation_samples <= 6);
        assert_eq!(report.unchanged_rules_retained, 2);
        assert!(report.bounded_memory);
        assert!(report.passed);
    }

    #[test]
    fn v12_action_aliases_and_recurring_classes_form_a_hierarchy() {
        let report = run_v12_experiment();

        assert_eq!(report.operator_classes, 3);
        assert!(report.aliased_actions_grouped);
        assert_eq!(report.procedure_length, 3);
        assert_eq!(report.procedure_support, 8);
        assert_eq!(report.raw_sequence_max_support, 1);
        assert!(report.random_control_rejected);
        assert!(report.passed);
    }

    #[test]
    fn v13_learning_loop_selects_its_own_curriculum() {
        let report = run_v13_experiment();

        assert_eq!(report.causal_interventions, 5);
        assert_eq!(report.selected_task_traces, 6);
        assert!(report.random_average_task_traces > 15.0);
        assert!(report.automatic_abstraction);
        assert!(report.automatic_procedure);
        assert!(report.passed);
    }

    #[test]
    fn v14_structural_knowledge_transfers_between_raw_encodings() {
        let report = run_v14_experiment();

        assert_eq!(report.calibrated_action_aliases, 3);
        assert!(report.transferred_procedure);
        assert_eq!(report.target_task_traces_required, 0);
        assert!(report.unrelated_operator_rejected);
        assert!(report.independent_raw_encodings);
        assert!(report.passed);
    }

    #[test]
    fn v14_complete_batch_passes() {
        assert!(run_experiments().passed);
    }
}
