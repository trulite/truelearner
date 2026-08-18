use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::inertia::{GridTopology, Point};

const MINIMUM_RULE_SUPPORT: usize = 3;
const MINIMUM_RULE_CONFIDENCE: f64 = 0.8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    PushLeft,
    PushRight,
    Rotate,
    Wait,
}

impl Action {
    const ALL: [Self; 4] = [Self::PushLeft, Self::PushRight, Self::Rotate, Self::Wait];
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Transform {
    dx: i32,
    dy: i32,
    quarter_turns: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Shape {
    points: Vec<Point>,
}

impl Shape {
    fn from_points(points: &[Point]) -> Self {
        let minimum_x = points.iter().map(|point| point.x).min().unwrap_or(0);
        let minimum_y = points.iter().map(|point| point.y).min().unwrap_or(0);
        let mut normalized: Vec<_> = points
            .iter()
            .map(|point| Point {
                x: point.x - minimum_x,
                y: point.y - minimum_y,
            })
            .collect();
        normalized.sort();
        normalized.dedup();
        Self { points: normalized }
    }

    fn rotated(&self, quarter_turns: u8) -> Self {
        let mut points = self.points.clone();
        for _ in 0..quarter_turns % 4 {
            points = points
                .into_iter()
                .map(|point| Point {
                    x: -point.y,
                    y: point.x,
                })
                .collect();
        }
        Self::from_points(&points)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RawFrame {
    pixels: Vec<u8>,
}

impl RawFrame {
    fn render(topology: &GridTopology, shape: &Shape, anchor: Point) -> Self {
        Self::try_render(topology, shape, anchor)
            .expect("rendered object must remain inside the topology")
    }

    fn try_render(topology: &GridTopology, shape: &Shape, anchor: Point) -> Option<Self> {
        let mut pixels = vec![0; topology.sensor_count()];
        for relative in &shape.points {
            let point = Point {
                x: anchor.x + relative.x,
                y: anchor.y + relative.y,
            };
            let sensor = topology.sensor_at(point)?;
            pixels[sensor] = 1;
        }
        Some(Self { pixels })
    }

    fn decode(&self, topology: &GridTopology) -> Option<(Shape, Point)> {
        let points: Vec<_> = self
            .pixels
            .iter()
            .enumerate()
            .filter_map(|(sensor, &value)| {
                (value != 0).then(|| topology.point_of(sensor)).flatten()
            })
            .collect();
        if points.is_empty() {
            return None;
        }

        let anchor = Point {
            x: points.iter().map(|point| point.x).min()?,
            y: points.iter().map(|point| point.y).min()?,
        };
        Some((Shape::from_points(&points), anchor))
    }
}

fn apply_transform(
    topology: &GridTopology,
    before: &RawFrame,
    transform: Transform,
) -> Option<RawFrame> {
    let (shape, anchor) = before.decode(topology)?;
    let transformed_shape = shape.rotated(transform.quarter_turns);
    let transformed_anchor = Point {
        x: anchor.x + transform.dx,
        y: anchor.y + transform.dy,
    };
    RawFrame::try_render(topology, &transformed_shape, transformed_anchor)
}

fn infer_transform(
    topology: &GridTopology,
    before: &RawFrame,
    after: &RawFrame,
) -> Option<Transform> {
    let (before_shape, before_anchor) = before.decode(topology)?;
    let (after_shape, after_anchor) = after.decode(topology)?;
    let matching_turns: Vec<_> = (0..4)
        .filter(|&turns| before_shape.rotated(turns) == after_shape)
        .collect();
    if matching_turns.len() != 1 {
        return None;
    }

    Some(Transform {
        dx: after_anchor.x - before_anchor.x,
        dy: after_anchor.y - before_anchor.y,
        quarter_turns: matching_turns[0],
    })
}

#[derive(Clone, Copy, Debug)]
struct RuleEstimate {
    transform: Transform,
    support: usize,
    confidence: f64,
}

struct CausalLearner {
    memories: BTreeMap<Action, VecDeque<Transform>>,
    history_window: usize,
}

#[derive(Clone, Debug)]
struct SearchResult {
    actions: Vec<Action>,
    expanded_states: usize,
}

impl CausalLearner {
    fn new(history_window: usize) -> Self {
        assert!(history_window >= MINIMUM_RULE_SUPPORT);
        Self {
            memories: Action::ALL
                .into_iter()
                .map(|action| (action, VecDeque::new()))
                .collect(),
            history_window,
        }
    }

    fn observe(
        &mut self,
        topology: &GridTopology,
        before: &RawFrame,
        action: Action,
        after: &RawFrame,
    ) -> bool {
        let Some(transform) = infer_transform(topology, before, after) else {
            return false;
        };
        let memory = self
            .memories
            .get_mut(&action)
            .expect("all actions have a memory");
        if memory.len() == self.history_window {
            memory.pop_front();
        }
        memory.push_back(transform);
        true
    }

    fn estimate(&self, action: Action) -> Option<RuleEstimate> {
        let memory = self.memories.get(&action)?;
        if memory.len() < MINIMUM_RULE_SUPPORT {
            return None;
        }
        let mut counts = BTreeMap::new();
        for &transform in memory {
            *counts.entry(transform).or_insert(0usize) += 1;
        }
        let (&transform, &support) = counts.iter().max_by_key(|(_, count)| *count)?;
        let confidence = support as f64 / memory.len() as f64;
        (confidence >= MINIMUM_RULE_CONFIDENCE).then_some(RuleEstimate {
            transform,
            support,
            confidence,
        })
    }

    fn predict(
        &self,
        topology: &GridTopology,
        before: &RawFrame,
        action: Action,
    ) -> Option<RawFrame> {
        let rule = self.estimate(action)?;
        apply_transform(topology, before, rule.transform)
    }

    fn all_rules_known(&self) -> bool {
        Action::ALL
            .into_iter()
            .all(|action| self.estimate(action).is_some())
    }

    fn uncertainty(&self, action: Action) -> f64 {
        let memory = self
            .memories
            .get(&action)
            .expect("all actions have a memory");
        let support_deficit = MINIMUM_RULE_SUPPORT.saturating_sub(memory.len()) as f64;
        if memory.is_empty() {
            return support_deficit * 10.0 + 1.0;
        }

        let mut counts = BTreeMap::new();
        for &transform in memory {
            *counts.entry(transform).or_insert(0usize) += 1;
        }
        let dominant = counts.values().copied().max().unwrap_or(0);
        let disagreement = 1.0 - dominant as f64 / memory.len() as f64;
        support_deficit * 10.0 + disagreement
    }

    fn choose_experiment(&self) -> Action {
        Action::ALL
            .into_iter()
            .max_by(|left, right| {
                self.uncertainty(*left)
                    .total_cmp(&self.uncertainty(*right))
                    .then_with(|| right.cmp(left))
            })
            .expect("the action set is non-empty")
    }

    fn plan(
        &self,
        topology: &GridTopology,
        start: &RawFrame,
        target: &RawFrame,
        maximum_depth: usize,
    ) -> Option<Vec<Action>> {
        self.plan_with_stats(topology, start, target, maximum_depth)
            .map(|result| result.actions)
    }

    fn plan_with_stats(
        &self,
        topology: &GridTopology,
        start: &RawFrame,
        target: &RawFrame,
        maximum_depth: usize,
    ) -> Option<SearchResult> {
        if start == target {
            return Some(SearchResult {
                actions: Vec::new(),
                expanded_states: 0,
            });
        }

        let mut queue = VecDeque::from([(start.clone(), Vec::new())]);
        let mut visited = BTreeSet::from([start.clone()]);
        let mut expanded_states = 0;
        while let Some((frame, path)) = queue.pop_front() {
            expanded_states += 1;
            if path.len() == maximum_depth {
                continue;
            }
            for action in Action::ALL {
                let Some(next) = self.predict(topology, &frame, action) else {
                    continue;
                };
                if !visited.insert(next.clone()) {
                    continue;
                }

                let mut next_path = path.clone();
                next_path.push(action);
                if &next == target {
                    return Some(SearchResult {
                        actions: next_path,
                        expanded_states,
                    });
                }
                queue.push_back((next, next_path));
            }
        }
        None
    }
}

fn initial_rule(action: Action) -> Transform {
    match action {
        Action::PushLeft => Transform {
            dx: -1,
            ..Transform::default()
        },
        Action::PushRight => Transform {
            dx: 1,
            ..Transform::default()
        },
        Action::Rotate => Transform {
            quarter_turns: 1,
            ..Transform::default()
        },
        Action::Wait => Transform::default(),
    }
}

fn training_shape() -> Shape {
    Shape::from_points(&[
        Point { x: 0, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: 2 },
        Point { x: 1, y: 2 },
        Point { x: 2, y: 2 },
    ])
}

fn novel_shape() -> Shape {
    Shape::from_points(&[
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 1, y: 2 },
    ])
}

fn transition(
    topology: &GridTopology,
    shape: &Shape,
    anchor: Point,
    transform: Transform,
) -> (RawFrame, RawFrame) {
    let before = RawFrame::render(topology, shape, anchor);
    let after = apply_transform(topology, &before, transform)
        .expect("training transform must remain inside the topology");
    (before, after)
}

fn observe_action_sample(
    learner: &mut CausalLearner,
    topology: &GridTopology,
    action: Action,
    transform: Transform,
    sample_index: usize,
) {
    let anchor = Point {
        x: 4 + (sample_index % 3) as i32,
        y: 4 + ((sample_index / 3) % 3) as i32,
    };
    let (before, after) = transition(topology, &training_shape(), anchor, transform);
    assert!(learner.observe(topology, &before, action, &after));
}

fn train_complete_learner(history_window: usize, samples_per_action: usize) -> CausalLearner {
    let topology = GridTopology::permuted(14, 14, 0xca05a1);
    let mut learner = CausalLearner::new(history_window);
    for action in Action::ALL {
        for sample in 0..samples_per_action {
            observe_action_sample(
                &mut learner,
                &topology,
                action,
                initial_rule(action),
                sample,
            );
        }
    }
    learner
}

fn active_coverage_steps() -> usize {
    let topology = GridTopology::permuted(14, 14, 0xac71ce);
    let mut learner = CausalLearner::new(8);
    for step in 1..=100 {
        let action = learner.choose_experiment();
        observe_action_sample(&mut learner, &topology, action, initial_rule(action), step);
        if learner.all_rules_known() {
            return step;
        }
    }
    100
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

fn random_coverage_steps(seed: u64) -> usize {
    let topology = GridTopology::permuted(14, 14, seed ^ 0x5e1150);
    let mut learner = CausalLearner::new(8);
    let mut rng = Lcg::new(seed);
    for step in 1..=200 {
        let action = Action::ALL[rng.next_usize(Action::ALL.len())];
        observe_action_sample(&mut learner, &topology, action, initial_rule(action), step);
        if learner.all_rules_known() {
            return step;
        }
    }
    200
}

fn average_random_coverage_steps() -> f64 {
    (1..=64).map(random_coverage_steps).sum::<usize>() as f64 / 64.0
}

fn passive_discovers_all_rules() -> bool {
    let topology = GridTopology::permuted(14, 14, 0x9a551e);
    let mut learner = CausalLearner::new(8);
    for sample in 0..32 {
        observe_action_sample(
            &mut learner,
            &topology,
            Action::Wait,
            initial_rule(Action::Wait),
            sample,
        );
    }
    learner.all_rules_known()
}

fn transfer_accuracy(learner: &CausalLearner) -> usize {
    let topology = GridTopology::permuted(20, 16, 0x7a11ca05);
    let shape = novel_shape();
    Action::ALL
        .into_iter()
        .filter(|&action| {
            let before = RawFrame::render(&topology, &shape, Point { x: 8, y: 6 });
            let actual = apply_transform(&topology, &before, initial_rule(action)).unwrap();
            learner.predict(&topology, &before, action) == Some(actual)
        })
        .count()
}

fn adapt_changed_rule(learner: &mut CausalLearner) -> (usize, bool) {
    let topology = GridTopology::permuted(14, 14, 0xada971);
    let changed = Transform {
        dy: 1,
        ..Transform::default()
    };

    observe_action_sample(learner, &topology, Action::PushLeft, changed, 100);
    let mut samples = 1;
    while learner
        .estimate(Action::PushLeft)
        .is_none_or(|estimate| estimate.transform != changed)
        && samples < 16
    {
        if learner.choose_experiment() != Action::PushLeft {
            return (samples, false);
        }
        observe_action_sample(learner, &topology, Action::PushLeft, changed, 100 + samples);
        samples += 1;
    }

    let before = RawFrame::render(&topology, &novel_shape(), Point { x: 7, y: 6 });
    let actual = apply_transform(&topology, &before, changed).unwrap();
    let adapted = learner.predict(&topology, &before, Action::PushLeft) == Some(actual);
    (samples, adapted)
}

fn rectangle_shape() -> Shape {
    Shape::from_points(&[
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
        Point { x: 2, y: 1 },
    ])
}

fn execute_actions(
    topology: &GridTopology,
    start: &RawFrame,
    actions: &[Action],
    changed_rule: Option<(Action, Transform)>,
) -> Option<RawFrame> {
    let mut frame = start.clone();
    for &action in actions {
        let transform = changed_rule
            .filter(|(changed_action, _)| *changed_action == action)
            .map_or_else(|| initial_rule(action), |(_, transform)| transform);
        frame = apply_transform(topology, &frame, transform)?;
    }
    Some(frame)
}

struct HeldOutPlanCase {
    start: RawFrame,
    target: RawFrame,
    expected_length: usize,
}

fn held_out_plan_cases(topology: &GridTopology) -> Vec<HeldOutPlanCase> {
    let shape = novel_shape();
    let specifications = [
        (
            Point { x: 6, y: 6 },
            vec![
                Action::PushRight,
                Action::PushRight,
                Action::PushRight,
                Action::PushRight,
                Action::Rotate,
            ],
        ),
        (
            Point { x: 12, y: 6 },
            vec![
                Action::PushLeft,
                Action::PushLeft,
                Action::PushLeft,
                Action::Rotate,
                Action::Rotate,
            ],
        ),
        (
            Point { x: 5, y: 6 },
            vec![
                Action::PushRight,
                Action::PushRight,
                Action::PushRight,
                Action::PushRight,
                Action::PushRight,
                Action::Rotate,
                Action::Rotate,
                Action::Rotate,
            ],
        ),
    ];

    specifications
        .into_iter()
        .map(|(anchor, actions)| {
            let start = RawFrame::render(topology, &shape, anchor);
            let target = execute_actions(topology, &start, &actions, None)
                .expect("held-out target must be reachable");
            HeldOutPlanCase {
                start,
                target,
                expected_length: actions.len(),
            }
        })
        .collect()
}

fn evaluate_held_out_plans(learner: &CausalLearner) -> (usize, usize, f64) {
    let topology = GridTopology::permuted(22, 18, 0x91a7_7a11);
    let cases = held_out_plan_cases(&topology);
    let mut solved = 0;
    let mut optimal = 0;
    let mut random_successes = 0;
    let random_trials_per_case = 512;
    let mut rng = Lcg::new(0x05e9_e0ce);

    for case in &cases {
        if let Some(plan) = learner.plan(&topology, &case.start, &case.target, 8) {
            if execute_actions(&topology, &case.start, &plan, None) == Some(case.target.clone()) {
                solved += 1;
            }
            if plan.len() == case.expected_length {
                optimal += 1;
            }
        }

        for _ in 0..random_trials_per_case {
            let actions: Vec<_> = (0..case.expected_length)
                .map(|_| Action::ALL[rng.next_usize(Action::ALL.len())])
                .collect();
            if execute_actions(&topology, &case.start, &actions, None) == Some(case.target.clone())
            {
                random_successes += 1;
            }
        }
    }

    let random_success_rate =
        random_successes as f64 / (cases.len() * random_trials_per_case) as f64;
    (solved, optimal, random_success_rate)
}

fn boundary_order_is_respected(learner: &CausalLearner) -> bool {
    let topology = GridTopology::permuted(22, 10, 0x00b0_ada7);
    let start = RawFrame::render(&topology, &rectangle_shape(), Point { x: 19, y: 4 });
    let required = [Action::Rotate, Action::PushRight];
    let target = execute_actions(&topology, &start, &required, None).unwrap();

    learner.plan(&topology, &start, &target, 4) == Some(required.to_vec())
}

fn unreachable_target_is_reported(learner: &CausalLearner) -> bool {
    let topology = GridTopology::permuted(18, 14, 0x00ff_11ae);
    let shape = novel_shape();
    let start = RawFrame::render(&topology, &shape, Point { x: 7, y: 5 });
    let target = RawFrame::render(&topology, &shape, Point { x: 7, y: 6 });

    learner.plan(&topology, &start, &target, 8).is_none()
}

fn replan_after_rule_change() -> (bool, usize) {
    let topology = GridTopology::permuted(20, 12, 0x007e_91a0);
    let mut learner = train_complete_learner(3, 3);
    let shape = novel_shape();
    let start = RawFrame::render(&topology, &shape, Point { x: 10, y: 5 });
    let target = RawFrame::render(&topology, &shape, Point { x: 6, y: 5 });
    let original_plan = learner.plan(&topology, &start, &target, 6).unwrap();
    if original_plan.len() != 4 || original_plan[0] != Action::PushLeft {
        return (false, 0);
    }

    let after_first = apply_transform(&topology, &start, initial_rule(original_plan[0])).unwrap();
    let changed = Transform {
        dx: -2,
        ..Transform::default()
    };
    let after_surprise = apply_transform(&topology, &after_first, changed).unwrap();
    if !learner.observe(&topology, &after_first, Action::PushLeft, &after_surprise) {
        return (false, 0);
    }

    for sample in 0..2 {
        if learner.choose_experiment() != Action::PushLeft {
            return (false, 0);
        }
        observe_action_sample(
            &mut learner,
            &topology,
            Action::PushLeft,
            changed,
            200 + sample,
        );
    }
    if learner
        .estimate(Action::PushLeft)
        .is_none_or(|estimate| estimate.transform != changed)
    {
        return (false, 0);
    }

    let Some(replanned) = learner.plan(&topology, &after_surprise, &target, 6) else {
        return (false, 0);
    };
    let reached = execute_actions(
        &topology,
        &after_surprise,
        &replanned,
        Some((Action::PushLeft, changed)),
    ) == Some(target);
    (reached, replanned.len())
}

#[derive(Debug)]
pub struct PlanningReport {
    pub held_out_solved: usize,
    pub held_out_optimal: usize,
    pub random_success_rate: f64,
    pub boundary_order_respected: bool,
    pub unreachable_reported: bool,
    pub replanned_after_change: bool,
    pub replan_length: usize,
    pub passed: bool,
}

pub fn run_planning_experiment() -> PlanningReport {
    let learner = train_complete_learner(6, 6);
    let (held_out_solved, held_out_optimal, random_success_rate) =
        evaluate_held_out_plans(&learner);
    let boundary_order_respected = boundary_order_is_respected(&learner);
    let unreachable_reported = unreachable_target_is_reported(&learner);
    let (replanned_after_change, replan_length) = replan_after_rule_change();
    let passed = held_out_solved == 3
        && held_out_optimal == 3
        && random_success_rate < 0.02
        && boundary_order_respected
        && unreachable_reported
        && replanned_after_change
        && replan_length == 2;

    PlanningReport {
        held_out_solved,
        held_out_optimal,
        random_success_rate,
        boundary_order_respected,
        unreachable_reported,
        replanned_after_change,
        replan_length,
        passed,
    }
}

pub fn print_planning_report(report: &PlanningReport) {
    println!("causal composition and ANSWER planning:");
    println!(
        "  held-out targets solved optimally: {}/3 solved, {}/3 shortest",
        report.held_out_solved, report.held_out_optimal
    );
    println!(
        "  matched-length random action success: {:.2}%",
        report.random_success_rate * 100.0
    );
    println!(
        "  boundary-sensitive action order respected: {}",
        report.boundary_order_respected
    );
    println!(
        "  unreachable target reported: {}",
        report.unreachable_reported
    );
    println!(
        "  replanned after changed action rule: {} (new plan length={})",
        report.replanned_after_change, report.replan_length
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ProcedureToken {
    Primitive(Action),
    Procedure(usize),
}

#[derive(Clone, Debug)]
struct Procedure {
    tokens: Vec<ProcedureToken>,
    actions: Vec<Action>,
    support: usize,
    compression_gain: usize,
    dependencies: BTreeMap<Action, Transform>,
}

#[derive(Debug)]
struct ProcedurePlan {
    actions: Vec<Action>,
    expanded_states: usize,
    procedure_steps: usize,
    used_primitive_fallback: bool,
}

struct ProcedureMemory {
    demonstrations: Vec<Vec<Action>>,
    procedures: BTreeMap<usize, Procedure>,
    next_id: usize,
}

impl ProcedureMemory {
    fn new() -> Self {
        Self {
            demonstrations: Vec::new(),
            procedures: BTreeMap::new(),
            next_id: 0,
        }
    }

    fn observe_success(&mut self, actions: &[Action]) {
        if !actions.is_empty() {
            self.demonstrations.push(actions.to_vec());
        }
    }

    fn consolidate_base(
        &mut self,
        learner: &CausalLearner,
        maximum_length: usize,
        minimum_support: usize,
    ) -> Option<usize> {
        let mut candidates: BTreeMap<Vec<Action>, (usize, BTreeSet<usize>)> = BTreeMap::new();
        for (trace_index, trace) in self.demonstrations.iter().enumerate() {
            for length in 3..=maximum_length.min(trace.len()) {
                for window in trace.windows(length) {
                    let entry = candidates.entry(window.to_vec()).or_default();
                    entry.0 += 1;
                    entry.1.insert(trace_index);
                }
            }
        }

        let best = candidates
            .into_iter()
            .filter_map(|(actions, (occurrences, traces))| {
                let support = traces.len();
                let compression_gain = occurrences
                    .saturating_mul(actions.len().saturating_sub(1))
                    .saturating_sub(actions.len());
                (support >= minimum_support && compression_gain > 0).then_some((
                    actions,
                    support,
                    compression_gain,
                ))
            })
            .filter(|(actions, _, _)| {
                self.procedures
                    .values()
                    .all(|procedure| procedure.actions != *actions)
            })
            .max_by(|left, right| {
                (left.2, left.0.len(), left.1).cmp(&(right.2, right.0.len(), right.1))
            })?;

        let tokens = best
            .0
            .iter()
            .copied()
            .map(ProcedureToken::Primitive)
            .collect();
        self.insert(tokens, best.0, best.1, best.2, learner)
    }

    fn consolidate_nested(
        &mut self,
        learner: &CausalLearner,
        maximum_tokens: usize,
        minimum_support: usize,
    ) -> Option<usize> {
        let encoded: Vec<_> = self
            .demonstrations
            .iter()
            .map(|trace| self.encode(trace))
            .collect();
        let mut candidates: BTreeMap<Vec<ProcedureToken>, (usize, BTreeSet<usize>)> =
            BTreeMap::new();
        for (trace_index, trace) in encoded.iter().enumerate() {
            for length in 2..=maximum_tokens.min(trace.len()) {
                for window in trace.windows(length) {
                    if !window
                        .iter()
                        .any(|token| matches!(token, ProcedureToken::Procedure(_)))
                    {
                        continue;
                    }
                    let entry = candidates.entry(window.to_vec()).or_default();
                    entry.0 += 1;
                    entry.1.insert(trace_index);
                }
            }
        }

        let best = candidates
            .into_iter()
            .filter_map(|(tokens, (occurrences, traces))| {
                let support = traces.len();
                let compression_gain = occurrences
                    .saturating_mul(tokens.len().saturating_sub(1))
                    .saturating_sub(tokens.len());
                let actions = self.flatten(&tokens)?;
                (support >= minimum_support && compression_gain > 0).then_some((
                    tokens,
                    actions,
                    support,
                    compression_gain,
                ))
            })
            .filter(|(_, actions, _, _)| {
                self.procedures
                    .values()
                    .all(|procedure| procedure.actions != *actions)
            })
            .max_by(|left, right| {
                (left.3, left.1.len(), left.2).cmp(&(right.3, right.1.len(), right.2))
            })?;

        self.insert(best.0, best.1, best.2, best.3, learner)
    }

    fn insert(
        &mut self,
        tokens: Vec<ProcedureToken>,
        actions: Vec<Action>,
        support: usize,
        compression_gain: usize,
        learner: &CausalLearner,
    ) -> Option<usize> {
        let dependencies = procedure_dependencies(&actions, learner)?;
        let id = self.next_id;
        self.next_id += 1;
        self.procedures.insert(
            id,
            Procedure {
                tokens,
                actions,
                support,
                compression_gain,
                dependencies,
            },
        );
        Some(id)
    }

    fn encode(&self, actions: &[Action]) -> Vec<ProcedureToken> {
        let mut procedures: Vec<_> = self.procedures.iter().collect();
        procedures.sort_by_key(|(_, procedure)| std::cmp::Reverse(procedure.actions.len()));

        let mut encoded = Vec::new();
        let mut offset = 0;
        while offset < actions.len() {
            if let Some((&id, procedure)) = procedures
                .iter()
                .find(|(_, procedure)| actions[offset..].starts_with(procedure.actions.as_slice()))
            {
                encoded.push(ProcedureToken::Procedure(id));
                offset += procedure.actions.len();
            } else {
                encoded.push(ProcedureToken::Primitive(actions[offset]));
                offset += 1;
            }
        }
        encoded
    }

    fn flatten(&self, tokens: &[ProcedureToken]) -> Option<Vec<Action>> {
        let mut actions = Vec::new();
        for token in tokens {
            match *token {
                ProcedureToken::Primitive(action) => actions.push(action),
                ProcedureToken::Procedure(id) => {
                    actions.extend_from_slice(&self.procedures.get(&id)?.actions);
                }
            }
        }
        Some(actions)
    }

    fn retain_valid(&mut self, learner: &CausalLearner) -> usize {
        let original_count = self.procedures.len();
        self.procedures.retain(|_, procedure| {
            procedure.dependencies.iter().all(|(&action, transform)| {
                learner
                    .estimate(action)
                    .is_some_and(|estimate| estimate.transform == *transform)
            })
        });

        loop {
            let valid_ids: BTreeSet<_> = self.procedures.keys().copied().collect();
            let previous_count = self.procedures.len();
            self.procedures.retain(|_, procedure| {
                procedure.tokens.iter().all(|token| match token {
                    ProcedureToken::Primitive(_) => true,
                    ProcedureToken::Procedure(id) => valid_ids.contains(id),
                })
            });
            if self.procedures.len() == previous_count {
                break;
            }
        }
        self.demonstrations.clear();
        original_count - self.procedures.len()
    }

    fn plan(
        &self,
        learner: &CausalLearner,
        topology: &GridTopology,
        start: &RawFrame,
        target: &RawFrame,
        maximum_primitive_depth: usize,
    ) -> Option<ProcedurePlan> {
        if start == target {
            return Some(ProcedurePlan {
                actions: Vec::new(),
                expanded_states: 0,
                procedure_steps: 0,
                used_primitive_fallback: false,
            });
        }

        let mut operators: Vec<_> = self.procedures.values().collect();
        operators.sort_by_key(|procedure| std::cmp::Reverse(procedure.actions.len()));
        let mut queue = VecDeque::from([(start.clone(), Vec::new(), 0usize)]);
        let mut visited = BTreeSet::from([start.clone()]);
        let mut expanded_states = 0;
        while let Some((frame, path, steps)) = queue.pop_front() {
            expanded_states += 1;
            for procedure in &operators {
                if path.len() + procedure.actions.len() > maximum_primitive_depth {
                    continue;
                }
                let Some(next) = predict_actions(learner, topology, &frame, &procedure.actions)
                else {
                    continue;
                };
                if !visited.insert(next.clone()) {
                    continue;
                }

                let mut next_path = path.clone();
                next_path.extend_from_slice(&procedure.actions);
                if &next == target {
                    return Some(ProcedurePlan {
                        actions: next_path,
                        expanded_states,
                        procedure_steps: steps + 1,
                        used_primitive_fallback: false,
                    });
                }
                queue.push_back((next, next_path, steps + 1));
            }
        }

        let primitive =
            learner.plan_with_stats(topology, start, target, maximum_primitive_depth)?;
        Some(ProcedurePlan {
            actions: primitive.actions,
            expanded_states: expanded_states + primitive.expanded_states,
            procedure_steps: 0,
            used_primitive_fallback: true,
        })
    }

    fn procedure_count(&self) -> usize {
        self.procedures.len()
    }

    fn nested_count(&self) -> usize {
        self.procedures
            .values()
            .filter(|procedure| {
                procedure
                    .tokens
                    .iter()
                    .any(|token| matches!(token, ProcedureToken::Procedure(_)))
            })
            .count()
    }

    fn best_support(&self) -> usize {
        self.procedures
            .values()
            .map(|procedure| procedure.support)
            .max()
            .unwrap_or(0)
    }

    fn total_compression_gain(&self) -> usize {
        self.procedures
            .values()
            .map(|procedure| procedure.compression_gain)
            .sum()
    }
}

fn procedure_dependencies(
    actions: &[Action],
    learner: &CausalLearner,
) -> Option<BTreeMap<Action, Transform>> {
    actions
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|action| {
            learner
                .estimate(action)
                .map(|estimate| (action, estimate.transform))
        })
        .collect()
}

fn predict_actions(
    learner: &CausalLearner,
    topology: &GridTopology,
    start: &RawFrame,
    actions: &[Action],
) -> Option<RawFrame> {
    let mut frame = start.clone();
    for &action in actions {
        frame = learner.predict(topology, &frame, action)?;
    }
    Some(frame)
}

fn recurring_procedure() -> Vec<Action> {
    vec![
        Action::PushRight,
        Action::PushRight,
        Action::PushRight,
        Action::Rotate,
    ]
}

fn learn_procedure_memory(learner: &CausalLearner) -> ProcedureMemory {
    let topology = GridTopology::permuted(24, 18, 0x0088_1ea4);
    let shape = training_shape();
    let recurring = recurring_procedure();
    let mut memory = ProcedureMemory::new();

    for sample in 0..10 {
        let start = RawFrame::render(
            &topology,
            &shape,
            Point {
                x: 4 + (sample % 4),
                y: 4 + (sample / 4),
            },
        );
        let target = execute_actions(&topology, &start, &recurring, None).unwrap();
        let plan = learner
            .plan(&topology, &start, &target, recurring.len())
            .unwrap();
        assert_eq!(plan, recurring);
        memory.observe_success(&plan);
    }
    memory
        .consolidate_base(learner, recurring.len(), 6)
        .expect("the recurring primitive sequence must compress");

    let repeated: Vec<_> = recurring.iter().copied().cycle().take(8).collect();
    for sample in 0..8 {
        let start = RawFrame::render(
            &topology,
            &shape,
            Point {
                x: 3 + (sample % 3),
                y: 9 + (sample / 3),
            },
        );
        let target = execute_actions(&topology, &start, &repeated, None).unwrap();
        let plan = memory
            .plan(learner, &topology, &start, &target, repeated.len())
            .unwrap();
        assert!(!plan.used_primitive_fallback);
        memory.observe_success(&plan.actions);
    }
    memory
        .consolidate_nested(learner, 2, 6)
        .expect("repeated procedures must form a higher-level procedure");
    memory
}

fn random_traces_learn_no_procedure(learner: &CausalLearner) -> bool {
    let mut memory = ProcedureMemory::new();
    let mut rng = Lcg::new(0x00a0_15e5);
    for _ in 0..12 {
        let trace: Vec<_> = (0..4)
            .map(|_| Action::ALL[rng.next_usize(Action::ALL.len())])
            .collect();
        memory.observe_success(&trace);
    }
    memory.consolidate_base(learner, 4, 6).is_none() && memory.procedure_count() == 0
}

fn evaluate_procedure_transfer(
    learner: &CausalLearner,
    memory: &ProcedureMemory,
) -> (bool, usize, usize, usize, bool) {
    let topology = GridTopology::permuted(34, 22, 0x0078_a45f);
    let start = RawFrame::render(&topology, &novel_shape(), Point { x: 7, y: 8 });
    let recurring = recurring_procedure();
    let target_actions: Vec<_> = recurring.iter().copied().cycle().take(12).collect();
    let target = execute_actions(&topology, &start, &target_actions, None).unwrap();
    let primitive = learner
        .plan_with_stats(&topology, &start, &target, target_actions.len())
        .unwrap();
    let hierarchical = memory
        .plan(learner, &topology, &start, &target, target_actions.len())
        .unwrap();
    let transferred = !hierarchical.used_primitive_fallback
        && hierarchical.actions.len() == primitive.actions.len()
        && execute_actions(&topology, &start, &hierarchical.actions, None) == Some(target);

    let fallback_target = execute_actions(&topology, &start, &[Action::PushLeft], None).unwrap();
    let fallback = memory
        .plan(learner, &topology, &start, &fallback_target, 2)
        .unwrap();
    let fallback_works = fallback.used_primitive_fallback
        && execute_actions(&topology, &start, &fallback.actions, None) == Some(fallback_target);

    (
        transferred,
        primitive.expanded_states,
        hierarchical.expanded_states,
        hierarchical.procedure_steps,
        fallback_works,
    )
}

fn repair_procedure_after_rule_change() -> (usize, bool) {
    let topology = GridTopology::permuted(24, 18, 0x00c4_a63e);
    let mut learner = train_complete_learner(3, 3);
    let mut memory = learn_procedure_memory(&learner);
    let changed = Transform {
        dx: 2,
        ..Transform::default()
    };
    for sample in 0..3 {
        observe_action_sample(
            &mut learner,
            &topology,
            Action::PushRight,
            changed,
            300 + sample,
        );
    }
    assert_eq!(
        learner.estimate(Action::PushRight).unwrap().transform,
        changed
    );

    let invalidated = memory.retain_valid(&learner);
    let replacement = [
        Action::PushRight,
        Action::PushRight,
        Action::PushLeft,
        Action::Rotate,
    ];
    for _ in 0..8 {
        memory.observe_success(&replacement);
    }
    let repaired_id = memory
        .consolidate_base(&learner, replacement.len(), 6)
        .expect("new successful traces must repair the stale abstraction");
    let repaired_actions = &memory.procedures.get(&repaired_id).unwrap().actions;

    let start = RawFrame::render(&topology, &novel_shape(), Point { x: 7, y: 7 });
    let target = execute_actions(
        &topology,
        &start,
        &replacement,
        Some((Action::PushRight, changed)),
    )
    .unwrap();
    let plan = memory
        .plan(&learner, &topology, &start, &target, replacement.len())
        .unwrap();
    let reached = !plan.used_primitive_fallback
        && repaired_actions == &replacement
        && execute_actions(
            &topology,
            &start,
            &plan.actions,
            Some((Action::PushRight, changed)),
        ) == Some(target);
    (invalidated, reached)
}

#[derive(Debug)]
pub struct ProcedureReport {
    pub learned_procedures: usize,
    pub nested_procedures: usize,
    pub best_support: usize,
    pub compression_gain: usize,
    pub transferred: bool,
    pub primitive_expansions: usize,
    pub hierarchical_expansions: usize,
    pub hierarchical_steps: usize,
    pub expansion_reduction: f64,
    pub primitive_fallback_works: bool,
    pub random_control_rejected: bool,
    pub invalidated_after_change: usize,
    pub repaired_after_change: bool,
    pub passed: bool,
}

pub fn run_procedure_experiment() -> ProcedureReport {
    let learner = train_complete_learner(6, 6);
    let memory = learn_procedure_memory(&learner);
    let learned_procedures = memory.procedure_count();
    let nested_procedures = memory.nested_count();
    let best_support = memory.best_support();
    let compression_gain = memory.total_compression_gain();
    let (transferred, primitive_expansions, hierarchical_expansions, hierarchical_steps, fallback) =
        evaluate_procedure_transfer(&learner, &memory);
    let expansion_reduction = primitive_expansions as f64 / hierarchical_expansions.max(1) as f64;
    let random_control_rejected = random_traces_learn_no_procedure(&learner);
    let (invalidated_after_change, repaired_after_change) = repair_procedure_after_rule_change();
    let passed = learned_procedures >= 2
        && nested_procedures >= 1
        && best_support >= 6
        && compression_gain > 0
        && transferred
        && expansion_reduction >= 5.0
        && fallback
        && random_control_rejected
        && invalidated_after_change >= 2
        && repaired_after_change;

    ProcedureReport {
        learned_procedures,
        nested_procedures,
        best_support,
        compression_gain,
        transferred,
        primitive_expansions,
        hierarchical_expansions,
        hierarchical_steps,
        expansion_reduction,
        primitive_fallback_works: fallback,
        random_control_rejected,
        invalidated_after_change,
        repaired_after_change,
        passed,
    }
}

pub fn print_procedure_report(report: &ProcedureReport) {
    println!("procedure discovery and hierarchical planning:");
    println!(
        "  learned procedures: {} (nested={}, best support={}, compression gain={})",
        report.learned_procedures,
        report.nested_procedures,
        report.best_support,
        report.compression_gain
    );
    println!(
        "  novel-shape/new-grid transfer: {} ({} procedure steps)",
        report.transferred, report.hierarchical_steps
    );
    println!(
        "  search expansions: primitive={}, hierarchical={} ({:.1}x reduction)",
        report.primitive_expansions, report.hierarchical_expansions, report.expansion_reduction
    );
    println!(
        "  primitive fallback: {}, random-trace rejection: {}",
        report.primitive_fallback_works, report.random_control_rejected
    );
    println!(
        "  causal change invalidated {} procedures; repaired from new experience: {}",
        report.invalidated_after_change, report.repaired_after_change
    );
}

#[derive(Debug)]
pub struct CausalReport {
    pub learned_rules: usize,
    pub minimum_rule_support: usize,
    pub minimum_rule_confidence: f64,
    pub active_coverage_steps: usize,
    pub random_average_coverage_steps: f64,
    pub passive_complete: bool,
    pub transfer_predictions: usize,
    pub adaptation_samples: usize,
    pub adapted_after_change: bool,
    pub passed: bool,
}

pub fn run_experiment() -> CausalReport {
    let mut learner = train_complete_learner(6, 6);
    let learned_estimates: Vec<_> = Action::ALL
        .into_iter()
        .filter_map(|action| learner.estimate(action).map(|estimate| (action, estimate)))
        .collect();
    let learned_rules = learned_estimates.len();
    let minimum_rule_support = learned_estimates
        .iter()
        .map(|(_, estimate)| estimate.support)
        .min()
        .unwrap_or(0);
    let minimum_rule_confidence = learned_estimates
        .iter()
        .map(|(_, estimate)| estimate.confidence)
        .fold(1.0, f64::min);
    let active_coverage_steps = active_coverage_steps();
    let random_average_coverage_steps = average_random_coverage_steps();
    let passive_complete = passive_discovers_all_rules();
    let transfer_predictions = transfer_accuracy(&learner);
    let (adaptation_samples, adapted_after_change) = adapt_changed_rule(&mut learner);
    let passed = learned_rules == Action::ALL.len()
        && minimum_rule_support >= MINIMUM_RULE_SUPPORT
        && minimum_rule_confidence >= MINIMUM_RULE_CONFIDENCE
        && active_coverage_steps < random_average_coverage_steps as usize
        && !passive_complete
        && transfer_predictions == Action::ALL.len()
        && adaptation_samples <= 6
        && adapted_after_change;

    CausalReport {
        learned_rules,
        minimum_rule_support,
        minimum_rule_confidence,
        active_coverage_steps,
        random_average_coverage_steps,
        passive_complete,
        transfer_predictions,
        adaptation_samples,
        adapted_after_change,
        passed,
    }
}

pub fn print_report(report: &CausalReport) {
    println!("active causal learning:");
    println!(
        "  learned action rules: {}/4 (minimum support={}, confidence={:.1}%)",
        report.learned_rules,
        report.minimum_rule_support,
        report.minimum_rule_confidence * 100.0
    );
    println!(
        "  experiments to learn all rules: active={}, random average={:.1}, passive complete={}",
        report.active_coverage_steps, report.random_average_coverage_steps, report.passive_complete
    );
    println!(
        "  novel-shape/new-grid action predictions: {}/4",
        report.transfer_predictions
    );
    println!(
        "  changed rule relearned: {} after {} contradictory samples",
        report.adapted_after_change, report.adaptation_samples
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_visual_transitions_recover_all_action_rules() {
        let learner = train_complete_learner(6, 6);
        for action in Action::ALL {
            assert_eq!(
                learner.estimate(action).unwrap().transform,
                initial_rule(action),
                "{action:?}"
            );
        }
    }

    #[test]
    fn uncertainty_directed_experiments_beat_random_and_passive_observation() {
        let active = active_coverage_steps();
        let random = average_random_coverage_steps();

        assert_eq!(active, 12);
        assert!(active < random as usize);
        assert!(!passive_discovers_all_rules());
    }

    #[test]
    fn learned_action_rules_transfer_to_a_novel_shape_and_sensor_layout() {
        let learner = train_complete_learner(6, 6);

        assert_eq!(transfer_accuracy(&learner), 4);
    }

    #[test]
    fn uncertainty_focuses_experiments_on_a_changed_rule_until_it_adapts() {
        let mut learner = train_complete_learner(6, 6);
        let (samples, adapted) = adapt_changed_rule(&mut learner);

        assert!(samples <= 6);
        assert!(adapted);
    }

    #[test]
    fn complete_causal_experiment_passes() {
        assert!(run_experiment().passed);
    }

    #[test]
    fn learned_model_composes_shortest_plans_for_held_out_targets() {
        let report = run_planning_experiment();

        assert_eq!(report.held_out_solved, 3);
        assert_eq!(report.held_out_optimal, 3);
        assert!(report.random_success_rate < 0.02);
    }

    #[test]
    fn planner_respects_boundaries_and_reports_unreachable_targets() {
        let learner = train_complete_learner(6, 6);

        assert!(boundary_order_is_respected(&learner));
        assert!(unreachable_target_is_reported(&learner));
    }

    #[test]
    fn prediction_failure_triggers_learning_and_replanning() {
        let (reached, plan_length) = replan_after_rule_change();

        assert!(reached);
        assert_eq!(plan_length, 2);
    }

    #[test]
    fn complete_planning_experiment_passes() {
        assert!(run_planning_experiment().passed);
    }

    #[test]
    fn recurring_actions_compress_into_nested_procedures() {
        let learner = train_complete_learner(6, 6);
        let memory = learn_procedure_memory(&learner);

        assert!(memory.procedure_count() >= 2);
        assert!(memory.nested_count() >= 1);
        assert!(memory.best_support() >= 6);
        assert!(memory.total_compression_gain() > 0);
    }

    #[test]
    fn procedures_transfer_and_reduce_search_with_primitive_fallback() {
        let learner = train_complete_learner(6, 6);
        let memory = learn_procedure_memory(&learner);
        let (transferred, primitive, hierarchical, steps, fallback) =
            evaluate_procedure_transfer(&learner, &memory);

        assert!(transferred);
        assert!(primitive >= hierarchical * 5);
        assert!(steps <= 2);
        assert!(fallback);
    }

    #[test]
    fn random_traces_do_not_become_procedures() {
        let learner = train_complete_learner(6, 6);

        assert!(random_traces_learn_no_procedure(&learner));
    }

    #[test]
    fn causal_change_invalidates_and_repairs_procedures() {
        let (invalidated, repaired) = repair_procedure_after_rule_change();

        assert!(invalidated >= 2);
        assert!(repaired);
    }

    #[test]
    fn complete_procedure_experiment_passes() {
        assert!(run_procedure_experiment().passed);
    }
}
