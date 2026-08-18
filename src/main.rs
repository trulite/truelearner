use std::collections::VecDeque;

use organism_v0::{
    causal, consolidation, generality, inertia, scaling, stability, tracking, unified, vision,
};

type CellId = usize;
type ArrowId = usize;
type WorldTime = u64;

#[derive(Clone, Debug)]
struct Cell {
    state: i32,
    threshold: i32,
    outgoing: Vec<ArrowId>,
    last_fired: Option<WorldTime>,
}

#[derive(Clone, Debug)]
struct Arrow {
    from: CellId,
    to: CellId,
    weight: i32,
}

#[derive(Clone, Copy, Debug)]
struct Spike {
    from: CellId,
    to: CellId,
    time: WorldTime,
    value: i32,
}

struct Mind {
    cells: Vec<Cell>,
    arrows: Vec<Arrow>,
    queue: VecDeque<Spike>,
    spikes_processed: u64,
}

impl Mind {
    fn new() -> Self {
        Self {
            cells: Vec::new(),
            arrows: Vec::new(),
            queue: VecDeque::new(),
            spikes_processed: 0,
        }
    }

    fn add_cell(&mut self, threshold: i32) -> CellId {
        let id = self.cells.len();
        self.cells.push(Cell {
            state: 0,
            threshold,
            outgoing: Vec::new(),
            last_fired: None,
        });
        id
    }

    fn add_arrow(&mut self, from: CellId, to: CellId, weight: i32) -> ArrowId {
        let id = self.arrows.len();
        self.arrows.push(Arrow { from, to, weight });
        self.cells[from].outgoing.push(id);
        id
    }

    fn inject(&mut self, to: CellId, time: WorldTime, value: i32) {
        self.queue.push_back(Spike {
            from: to,
            to,
            time,
            value,
        });
    }

    fn settle(&mut self) {
        while let Some(spike) = self.queue.pop_front() {
            debug_assert!(spike.from < self.cells.len());
            self.spikes_processed += 1;
            let to = spike.to;
            self.cells[to].state += spike.value;

            if self.cells[to].state >= self.cells[to].threshold {
                self.cells[to].state -= self.cells[to].threshold;
                self.cells[to].last_fired = Some(spike.time);

                let outgoing = self.cells[to].outgoing.clone();
                for arrow_id in outgoing {
                    let arrow = &self.arrows[arrow_id];
                    debug_assert_eq!(arrow.from, to);
                    self.queue.push_back(Spike {
                        from: to,
                        to: arrow.to,
                        time: spike.time,
                        value: arrow.weight,
                    });
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LearnedArrow {
    from: CellId,
    to: CellId,
    evidence: u32,
}

struct LocalTransitionLearner {
    receptor_count: usize,
    max_world_gap: WorldTime,
    last_seen: Vec<Option<WorldTime>>,
    arrows: Vec<LearnedArrow>,
}

impl LocalTransitionLearner {
    fn new(receptor_count: usize, max_world_gap: WorldTime) -> Self {
        assert!(receptor_count > 1);
        assert!(max_world_gap > 0);
        Self {
            receptor_count,
            max_world_gap,
            last_seen: vec![None; receptor_count],
            arrows: Vec::new(),
        }
    }

    fn begin_episode(&mut self) {
        self.last_seen.fill(None);
    }

    fn observe_episode(&mut self, stream: &[(WorldTime, CellId)]) {
        self.begin_episode();
        for &(time, receptor) in stream {
            self.observe(time, receptor);
        }
    }

    fn observe(&mut self, time: WorldTime, receptor: CellId) {
        assert!(receptor < self.receptor_count);

        // LOCAL is the only spatial prior: activity can associate with an
        // immediately neighboring receptor, but no direction is preferred.
        if receptor > 0 {
            self.learn_if_eligible(receptor - 1, receptor, time);
        }
        if receptor + 1 < self.receptor_count {
            self.learn_if_eligible(receptor + 1, receptor, time);
        }

        self.last_seen[receptor] = Some(time);
    }

    fn learn_if_eligible(&mut self, from: CellId, to: CellId, time: WorldTime) {
        let Some(previous_time) = self.last_seen[from] else {
            return;
        };
        let Some(gap) = time.checked_sub(previous_time) else {
            return;
        };
        if gap == 0 || gap > self.max_world_gap {
            return;
        }

        // Recent activity leaves a short trace. Immediate transitions receive
        // stronger evidence; an occluded transition can still receive less.
        let evidence = (self.max_world_gap + 1 - gap) as u32;
        self.strengthen(from, to, evidence);
    }

    fn strengthen(&mut self, from: CellId, to: CellId, evidence: u32) {
        if let Some(arrow) = self
            .arrows
            .iter_mut()
            .find(|arrow| arrow.from == from && arrow.to == to)
        {
            arrow.evidence += evidence;
            return;
        }

        self.arrows.push(LearnedArrow { from, to, evidence });
    }

    fn evidence(&self, from: CellId, to: CellId) -> u32 {
        self.arrows
            .iter()
            .find(|arrow| arrow.from == from && arrow.to == to)
            .map_or(0, |arrow| arrow.evidence)
    }

    fn strongest_target(&self, from: CellId) -> Option<CellId> {
        let mut best: Option<(CellId, u32)> = None;
        let mut tied = false;

        for arrow in self.arrows.iter().filter(|arrow| arrow.from == from) {
            match best {
                None => {
                    best = Some((arrow.to, arrow.evidence));
                    tied = false;
                }
                Some((_, best_evidence)) if arrow.evidence > best_evidence => {
                    best = Some((arrow.to, arrow.evidence));
                    tied = false;
                }
                Some((_, best_evidence)) if arrow.evidence == best_evidence => {
                    tied = true;
                }
                Some(_) => {}
            }
        }

        if tied {
            None
        } else {
            best.map(|(to, _)| to)
        }
    }

    fn prediction_score(&self, stream: &[(WorldTime, CellId)]) -> PredictionScore {
        let mut score = PredictionScore::default();
        for transition in stream.windows(2) {
            score.total += 1;
            if self.strongest_target(transition[0].1) == Some(transition[1].1) {
                score.correct += 1;
            }
        }
        score
    }

    fn direction_evidence(&self) -> (u32, u32) {
        self.arrows.iter().fold((0, 0), |(right, left), arrow| {
            if arrow.to == arrow.from + 1 {
                (right + arrow.evidence, left)
            } else if arrow.from == arrow.to + 1 {
                (right, left + arrow.evidence)
            } else {
                (right, left)
            }
        })
    }

    fn directionality(&self) -> f64 {
        let (right, left) = self.direction_evidence();
        let total = right + left;
        if total == 0 {
            0.0
        } else {
            (right as f64 - left as f64) / total as f64
        }
    }

    fn sorted_arrows(&self) -> Vec<LearnedArrow> {
        let mut arrows = self.arrows.clone();
        arrows.sort_by_key(|arrow| (arrow.from, arrow.to));
        arrows
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MotionConcept {
    relative_step: isize,
    evidence: u32,
}

impl MotionConcept {
    fn discover(learner: &LocalTransitionLearner, minimum_directionality: f64) -> Option<Self> {
        let directionality = learner.directionality();
        if directionality.abs() < minimum_directionality {
            return None;
        }

        let (right, left) = learner.direction_evidence();
        if right > left {
            Some(Self {
                relative_step: 1,
                evidence: right,
            })
        } else if left > right {
            Some(Self {
                relative_step: -1,
                evidence: left,
            })
        } else {
            None
        }
    }

    fn predict(&self, receptor: CellId, receptor_count: usize) -> Option<CellId> {
        let predicted = if self.relative_step > 0 {
            receptor.checked_add(self.relative_step as usize)
        } else {
            receptor.checked_sub((-self.relative_step) as usize)
        };
        predicted.filter(|&candidate| candidate < receptor_count)
    }

    fn prediction_score(
        &self,
        receptor_count: usize,
        stream: &[(WorldTime, CellId)],
    ) -> PredictionScore {
        let mut score = PredictionScore::default();
        for transition in stream.windows(2) {
            score.total += 1;
            if self.predict(transition[0].1, receptor_count) == Some(transition[1].1) {
                score.correct += 1;
            }
        }
        score
    }

    fn direction_name(&self) -> &'static str {
        if self.relative_step > 0 {
            "right"
        } else {
            "left"
        }
    }
}

struct MotionConceptBank {
    concepts: Vec<MotionConcept>,
}

impl MotionConceptBank {
    fn new(concepts: Vec<MotionConcept>) -> Self {
        Self { concepts }
    }

    fn select_from_context(&self, previous: CellId, current: CellId) -> Option<MotionConcept> {
        let observed_step = current as isize - previous as isize;
        self.concepts
            .iter()
            .copied()
            .find(|concept| concept.relative_step == observed_step)
    }

    fn continuation_score(
        &self,
        receptor_count: usize,
        stream: &[(WorldTime, CellId)],
    ) -> PredictionScore {
        if stream.len() < 3 {
            return PredictionScore::default();
        }

        let Some(concept) = self.select_from_context(stream[0].1, stream[1].1) else {
            return PredictionScore {
                correct: 0,
                total: stream.len() - 2,
            };
        };

        concept.prediction_score(receptor_count, &stream[1..])
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PredictionScore {
    correct: usize,
    total: usize,
}

impl PredictionScore {
    fn accuracy(self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.correct as f64 / self.total as f64
        }
    }

    fn add(&mut self, other: Self) {
        self.correct += other.correct;
        self.total += other.total;
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

fn rightward_stream() -> Vec<(WorldTime, CellId)> {
    vec![
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        // The point is occluded at world times 4 and 5.
        (6, 4),
        (7, 5),
        (8, 6),
        (9, 7),
    ]
}

fn held_out_rightward_stream() -> Vec<(WorldTime, CellId)> {
    vec![
        (0, 0),
        (2, 1),
        (4, 2),
        (6, 3),
        (9, 4),
        (11, 5),
        (13, 6),
        (15, 7),
    ]
}

fn windowed_motion_training_stream(
    relative_step: isize,
    variant: usize,
) -> Vec<(WorldTime, CellId)> {
    let times = if variant.is_multiple_of(2) {
        [0, 1, 2, 3]
    } else {
        [0, 2, 3, 5]
    };
    let receptors = if relative_step > 0 {
        [0, 1, 2, 3]
    } else {
        [7, 6, 5, 4]
    };

    times.into_iter().zip(receptors).collect()
}

fn unseen_rightward_stream() -> Vec<(WorldTime, CellId)> {
    vec![(0, 4), (2, 5), (9, 6), (11, 7)]
}

fn unseen_leftward_stream() -> Vec<(WorldTime, CellId)> {
    vec![(0, 3), (3, 2), (10, 1), (12, 0)]
}

fn random_stream(rng: &mut Lcg, receptor_count: usize, length: usize) -> Vec<(WorldTime, CellId)> {
    (0..length)
        .map(|time| (time as WorldTime, rng.next_usize(receptor_count)))
        .collect()
}

fn stationary_stream(receptor: CellId, length: usize) -> Vec<(WorldTime, CellId)> {
    (0..length)
        .map(|time| (time as WorldTime, receptor))
        .collect()
}

fn train_motion(episodes: usize) -> LocalTransitionLearner {
    let mut learner = LocalTransitionLearner::new(8, 3);
    let stream = rightward_stream();
    for _ in 0..episodes {
        learner.observe_episode(&stream);
    }
    learner
}

fn train_windowed_motion(relative_step: isize, episodes: usize) -> LocalTransitionLearner {
    assert!(relative_step == -1 || relative_step == 1);
    let mut learner = LocalTransitionLearner::new(8, 3);
    for episode in 0..episodes {
        learner.observe_episode(&windowed_motion_training_stream(relative_step, episode));
    }
    learner
}

fn train_random(episodes: usize, episode_length: usize, seed: u64) -> LocalTransitionLearner {
    let mut learner = LocalTransitionLearner::new(8, 3);
    let mut rng = Lcg::new(seed);
    for _ in 0..episodes {
        learner.observe_episode(&random_stream(&mut rng, 8, episode_length));
    }
    learner
}

fn score_random(
    learner: &LocalTransitionLearner,
    episodes: usize,
    episode_length: usize,
    seed: u64,
) -> PredictionScore {
    let mut score = PredictionScore::default();
    let mut rng = Lcg::new(seed);
    for _ in 0..episodes {
        score.add(learner.prediction_score(&random_stream(&mut rng, 8, episode_length)));
    }
    score
}

fn runtime_smoke_check() -> u64 {
    let mut mind = Mind::new();
    let source = mind.add_cell(1);
    let accumulator = mind.add_cell(2);
    mind.add_arrow(source, accumulator, 1);

    mind.inject(source, 0, 1);
    mind.settle();
    mind.inject(source, 1, 1);
    mind.settle();

    assert_eq!(mind.cells[accumulator].last_fired, Some(1));
    mind.spikes_processed
}

fn main() {
    const MOTION_EPISODES: usize = 64;
    const RANDOM_EPISODES: usize = 256;
    const RANDOM_EPISODE_LENGTH: usize = 16;

    let motion = train_motion(MOTION_EPISODES);
    let motion_score = motion.prediction_score(&held_out_rightward_stream());
    let (motion_right, motion_left) = motion.direction_evidence();

    let random = train_random(RANDOM_EPISODES, RANDOM_EPISODE_LENGTH, 0x5eed);
    let random_score = score_random(&random, 128, RANDOM_EPISODE_LENGTH, 0xc0ffee);
    let (random_right, random_left) = random.direction_evidence();

    let mut stationary = LocalTransitionLearner::new(8, 3);
    for _ in 0..MOTION_EPISODES {
        stationary.observe_episode(&stationary_stream(3, 8));
    }

    let right_training = train_windowed_motion(1, MOTION_EPISODES);
    let left_training = train_windowed_motion(-1, MOTION_EPISODES);
    let right_concept =
        MotionConcept::discover(&right_training, 0.8).expect("right concept should emerge");
    let left_concept =
        MotionConcept::discover(&left_training, 0.8).expect("left concept should emerge");
    let unseen_right = unseen_rightward_stream();
    let unseen_left = unseen_leftward_stream();
    let right_generalization = right_concept.prediction_score(8, &unseen_right);
    let left_generalization = left_concept.prediction_score(8, &unseen_left);
    let right_rejects_left = right_concept.prediction_score(8, &unseen_left);
    let left_rejects_right = left_concept.prediction_score(8, &unseen_right);
    let concept_bank = MotionConceptBank::new(vec![right_concept, left_concept]);
    let automatic_right = concept_bank.continuation_score(8, &unseen_right);
    let automatic_left = concept_bank.continuation_score(8, &unseen_left);
    let random_concept = MotionConcept::discover(&random, 0.8);
    let stationary_concept = MotionConcept::discover(&stationary, 0.8);

    println!("organism-v14.6 learned-stability experiment");
    println!();
    println!(
        "queue runtime smoke check: {} spikes processed",
        runtime_smoke_check()
    );
    println!("training: {MOTION_EPISODES} repeated rightward sweeps; no direction labels");
    println!("learned arrows:");
    for arrow in motion.sorted_arrows() {
        println!(
            "  {} -> {}  evidence={}",
            arrow.from, arrow.to, arrow.evidence
        );
    }
    println!(
        "motion direction evidence: right={motion_right}, left={motion_left}, score={:.3}",
        motion.directionality()
    );
    println!(
        "occlusion bridge evidence: {} (immediate transition evidence: {})",
        motion.evidence(3, 4),
        motion.evidence(2, 3)
    );
    println!(
        "held-out motion prediction: {}/{} ({:.1}%)",
        motion_score.correct,
        motion_score.total,
        motion_score.accuracy() * 100.0
    );
    println!();
    println!(
        "random control: right={random_right}, left={random_left}, score={:.3}, prediction={:.1}%",
        random.directionality(),
        random_score.accuracy() * 100.0
    );
    println!(
        "stationary control: learned directional arrows={}",
        stationary.arrows.len()
    );
    println!();

    println!("compressed position-independent concepts:");
    println!(
        "  {} concept: relative_step={:+}, evidence={}",
        right_concept.direction_name(),
        right_concept.relative_step,
        right_concept.evidence
    );
    println!(
        "  {} concept: relative_step={:+}, evidence={}",
        left_concept.direction_name(),
        left_concept.relative_step,
        left_concept.evidence
    );
    println!(
        "unseen-position right prediction: {}/{} ({:.1}%), world gaps=[2, 7, 2]",
        right_generalization.correct,
        right_generalization.total,
        right_generalization.accuracy() * 100.0
    );
    println!(
        "unseen-position left prediction: {}/{} ({:.1}%), world gaps=[3, 7, 2]",
        left_generalization.correct,
        left_generalization.total,
        left_generalization.accuracy() * 100.0
    );
    println!(
        "cross-direction predictions: right-on-left={:.1}%, left-on-right={:.1}%",
        right_rejects_left.accuracy() * 100.0,
        left_rejects_right.accuracy() * 100.0
    );
    println!(
        "automatic concept selection: right={}/{} ({:.1}%), left={}/{} ({:.1}%)",
        automatic_right.correct,
        automatic_right.total,
        automatic_right.accuracy() * 100.0,
        automatic_left.correct,
        automatic_left.total,
        automatic_left.accuracy() * 100.0
    );
    println!(
        "concept controls: random={}, stationary={}",
        if random_concept.is_some() {
            "false positive"
        } else {
            "none"
        },
        if stationary_concept.is_some() {
            "false positive"
        } else {
            "none"
        }
    );
    println!();

    let v2_passed = motion.directionality() > 0.9
        && motion_score.accuracy() > 0.9
        && random.directionality().abs() < 0.2
        && random_score.accuracy() < 0.3
        && stationary.arrows.is_empty()
        && right_concept.relative_step == 1
        && left_concept.relative_step == -1
        && right_generalization.accuracy() == 1.0
        && left_generalization.accuracy() == 1.0
        && right_rejects_left.accuracy() == 0.0
        && left_rejects_right.accuracy() == 0.0
        && automatic_right.accuracy() == 1.0
        && automatic_left.accuracy() == 1.0
        && random_concept.is_none()
        && stationary_concept.is_none();
    let inertia_report = inertia::run_experiment();
    inertia::print_report(&inertia_report);
    println!();

    let tracking_report = tracking::run_experiment(inertia_report.concept);
    tracking::print_report(&tracking_report);
    println!();

    let vision_report = vision::run_experiment();
    vision::print_report(&vision_report);
    println!();

    let causal_report = causal::run_experiment();
    causal::print_report(&causal_report);
    println!();

    let planning_report = causal::run_planning_experiment();
    causal::print_planning_report(&planning_report);
    println!();

    let procedure_report = causal::run_procedure_experiment();
    causal::print_procedure_report(&procedure_report);
    println!();

    let generality_report = generality::run_experiments();
    generality::print_report(&generality_report);
    println!();

    let stability_report = stability::run_experiment();
    stability::print_report(&stability_report);
    println!();

    let scaling_report = scaling::run_experiment();
    scaling::print_report(&scaling_report);
    println!();

    let unified_report = unified::run_experiment();
    unified::print_report(&unified_report);
    println!();

    let consolidation_report = consolidation::run_experiment();
    consolidation::print_report(&consolidation_report);
    println!();

    let passed = v2_passed
        && inertia_report.passed
        && tracking_report.passed
        && vision_report.passed
        && causal_report.passed
        && planning_report.passed
        && procedure_report.passed
        && generality_report.passed
        && stability_report.passed
        && scaling_report.passed
        && unified_report.passed
        && consolidation_report.passed;
    println!(
        "RESULT: {}",
        if passed {
            "PASS - identical rest compresses both memories, and the trie remains smaller while all earlier experiments pass"
        } else {
            "FAIL - at least one learning, scaling, capacity, transfer, or control test failed"
        }
    );
    println!(
        "LIMIT: recurrence pruning is not an event-substrate advantage; the equally rested trie uses fewer links, less estimated storage, and less query work."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spikes_propagate_and_settle() {
        let mut mind = Mind::new();
        let a = mind.add_cell(1);
        let b = mind.add_cell(2);
        mind.add_arrow(a, b, 1);

        mind.inject(a, 0, 1);
        mind.settle();
        assert_eq!(mind.cells[b].state, 1);
        assert_eq!(mind.cells[b].last_fired, None);

        mind.inject(a, 1, 1);
        mind.settle();
        assert_eq!(mind.cells[b].state, 0);
        assert_eq!(mind.cells[b].last_fired, Some(1));
    }

    #[test]
    fn repeated_motion_learns_a_directional_path() {
        let learner = train_motion(64);
        let (right, left) = learner.direction_evidence();
        let score = learner.prediction_score(&held_out_rightward_stream());

        assert!(right > 0);
        assert_eq!(left, 0);
        assert_eq!(learner.directionality(), 1.0);
        assert_eq!(
            score,
            PredictionScore {
                correct: 7,
                total: 7
            }
        );
    }

    #[test]
    fn occluded_transition_is_retained_with_weaker_evidence() {
        let learner = train_motion(1);

        assert_eq!(learner.evidence(2, 3), 3);
        assert_eq!(learner.evidence(3, 4), 1);
        assert_eq!(learner.strongest_target(3), Some(4));
    }

    #[test]
    fn random_flashes_do_not_create_a_strong_direction() {
        let learner = train_random(256, 16, 0x5eed);
        let score = score_random(&learner, 128, 16, 0xc0ffee);

        assert!(learner.directionality().abs() < 0.2);
        assert!(score.accuracy() < 0.3);
    }

    #[test]
    fn stationary_input_does_not_create_motion_arrows() {
        let mut learner = LocalTransitionLearner::new(8, 3);
        for _ in 0..64 {
            learner.observe_episode(&stationary_stream(3, 8));
        }

        assert!(learner.arrows.is_empty());
    }

    #[test]
    fn compression_discovers_both_relative_directions() {
        let right = MotionConcept::discover(&train_windowed_motion(1, 64), 0.8).unwrap();
        let left = MotionConcept::discover(&train_windowed_motion(-1, 64), 0.8).unwrap();

        assert_eq!(right.relative_step, 1);
        assert_eq!(left.relative_step, -1);
    }

    #[test]
    fn concepts_generalize_to_unseen_positions_speeds_and_occlusion_lengths() {
        let right = MotionConcept::discover(&train_windowed_motion(1, 64), 0.8).unwrap();
        let left = MotionConcept::discover(&train_windowed_motion(-1, 64), 0.8).unwrap();

        assert_eq!(
            right.prediction_score(8, &unseen_rightward_stream()),
            PredictionScore {
                correct: 3,
                total: 3
            }
        );
        assert_eq!(
            left.prediction_score(8, &unseen_leftward_stream()),
            PredictionScore {
                correct: 3,
                total: 3
            }
        );
    }

    #[test]
    fn concepts_reject_the_opposite_direction() {
        let right = MotionConcept::discover(&train_windowed_motion(1, 64), 0.8).unwrap();
        let left = MotionConcept::discover(&train_windowed_motion(-1, 64), 0.8).unwrap();

        assert_eq!(
            right.prediction_score(8, &unseen_leftward_stream()).correct,
            0
        );
        assert_eq!(
            left.prediction_score(8, &unseen_rightward_stream()).correct,
            0
        );
    }

    #[test]
    fn controls_do_not_compress_into_motion_concepts() {
        let random = train_random(256, 16, 0x5eed);
        let mut stationary = LocalTransitionLearner::new(8, 3);
        for _ in 0..64 {
            stationary.observe_episode(&stationary_stream(3, 8));
        }

        assert!(MotionConcept::discover(&random, 0.8).is_none());
        assert!(MotionConcept::discover(&stationary, 0.8).is_none());
    }

    #[test]
    fn concept_bank_selects_direction_from_observed_context() {
        let right = MotionConcept::discover(&train_windowed_motion(1, 64), 0.8).unwrap();
        let left = MotionConcept::discover(&train_windowed_motion(-1, 64), 0.8).unwrap();
        let bank = MotionConceptBank::new(vec![right, left]);

        assert_eq!(
            bank.continuation_score(8, &unseen_rightward_stream()),
            PredictionScore {
                correct: 2,
                total: 2
            }
        );
        assert_eq!(
            bank.continuation_score(8, &unseen_leftward_stream()),
            PredictionScore {
                correct: 2,
                total: 2
            }
        );
    }
}
