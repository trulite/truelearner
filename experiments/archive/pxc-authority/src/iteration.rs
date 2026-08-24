use crate::binding::{
    frozen_lookup_operation, BindingLearner, BindingOutcome, IdentitySource, OpaqueId,
};

const APPLY_CELL_ID: usize = 6;
const FEEDBACK_ARROW_OFFSET: usize = 4;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FeedbackRoute {
    UseResult,
    Keep,
    Clear,
}

#[derive(Clone, Debug)]
struct FeedbackArrow {
    route: FeedbackRoute,
    strength: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ApplyTrace {
    apply_cell_id: usize,
    lookup_arrow_id: Option<usize>,
    feedback_arrow_id: Option<usize>,
    current_before: OpaqueId,
    result: BindingOutcome,
    current_after: Option<OpaqueId>,
    lookup_spikes: usize,
}

#[derive(Clone, Debug)]
struct IterationLearner {
    lookup: BindingLearner,
    feedback_arrows: Vec<FeedbackArrow>,
    training_examples: u64,
    episode_query: Option<OpaqueId>,
    current: Option<OpaqueId>,
    result: Option<OpaqueId>,
    fault: Option<BindingOutcome>,
    trace: Vec<ApplyTrace>,
}

#[derive(Clone, Debug)]
pub(crate) struct FrozenIterableOperation {
    pub(crate) lookup: BindingLearner,
    pub(crate) feedback_arrow_id: usize,
    pub(crate) permanent_cells: usize,
    pub(crate) permanent_arrows: usize,
    pub(crate) permanent_fingerprint: u64,
}

impl IterationLearner {
    fn new() -> Self {
        Self {
            lookup: frozen_lookup_operation(),
            feedback_arrows: [
                FeedbackRoute::UseResult,
                FeedbackRoute::Keep,
                FeedbackRoute::Clear,
            ]
            .into_iter()
            .map(|route| FeedbackArrow { route, strength: 0 })
            .collect(),
            training_examples: 0,
            episode_query: None,
            current: None,
            result: None,
            fault: None,
            trace: Vec::new(),
        }
    }

    fn with_supplied_feedback() -> Self {
        let mut learner = Self::new();
        for arrow in &mut learner.feedback_arrows {
            arrow.strength = if arrow.route == FeedbackRoute::UseResult {
                1
            } else {
                -1
            };
        }
        learner
    }

    fn begin_episode(&mut self, query: OpaqueId) {
        self.erase_temporary();
        self.lookup.begin_episode();
        self.episode_query = Some(query);
        self.current = Some(query);
    }

    fn observe_relation(&mut self, slot1: OpaqueId, slot2: OpaqueId) {
        self.lookup.observe_relation(slot1, slot2);
    }

    /// An apply pulse contains no pulse number and receives no identity from
    /// the host. The current identity is read and updated inside the machine.
    fn apply_pulse(&mut self) {
        if self.fault.is_some() {
            return;
        }
        let Some(current_before) = self.current else {
            self.fault = Some(BindingOutcome::NotFound);
            return;
        };
        let (outcome, metrics, lookup_arrow_id) = self.lookup.lookup_identity(current_before);
        self.result = match outcome {
            BindingOutcome::Answer(identity) => Some(identity),
            BindingOutcome::NotFound | BindingOutcome::Ambiguous => None,
        };

        let mut feedback_arrow_id = None;
        match outcome {
            BindingOutcome::Answer(identity) => {
                if let Some((arrow_id, route)) = self.selected_feedback() {
                    feedback_arrow_id = Some(FEEDBACK_ARROW_OFFSET + arrow_id);
                    match route {
                        FeedbackRoute::UseResult => self.current = Some(identity),
                        FeedbackRoute::Keep => {}
                        FeedbackRoute::Clear => self.current = None,
                    }
                }
            }
            BindingOutcome::NotFound | BindingOutcome::Ambiguous => {
                self.fault = Some(outcome);
            }
        }

        self.trace.push(ApplyTrace {
            apply_cell_id: APPLY_CELL_ID,
            lookup_arrow_id,
            feedback_arrow_id,
            current_before,
            result: outcome,
            current_after: self.current,
            lookup_spikes: metrics.spikes,
        });
    }

    fn read(&self) -> BindingOutcome {
        if let Some(fault) = self.fault {
            return fault;
        }
        self.current
            .map_or(BindingOutcome::NotFound, BindingOutcome::Answer)
    }

    /// Terminal supervision compares only the complete final outcome. It does
    /// not identify the feedback route or any intermediate identity.
    fn learn_from_terminal(&mut self, correct: BindingOutcome, apply_pulses: usize) {
        let outcomes: Vec<_> = self
            .feedback_arrows
            .iter()
            .map(|arrow| self.simulate(arrow.route, apply_pulses))
            .collect();
        for (arrow, outcome) in self.feedback_arrows.iter_mut().zip(outcomes) {
            if outcome == correct {
                arrow.strength = arrow.strength.saturating_add(1);
            } else {
                arrow.strength = arrow.strength.saturating_sub(1);
            }
        }
        self.training_examples += 1;
    }

    fn simulate(&self, route: FeedbackRoute, apply_pulses: usize) -> BindingOutcome {
        let Some(mut current) = self.episode_query else {
            return BindingOutcome::NotFound;
        };
        for _ in 0..apply_pulses {
            let (outcome, _, _) = self.lookup.lookup_identity(current);
            let BindingOutcome::Answer(result) = outcome else {
                return outcome;
            };
            match route {
                FeedbackRoute::UseResult => current = result,
                FeedbackRoute::Keep => {}
                FeedbackRoute::Clear => return BindingOutcome::NotFound,
            }
        }
        BindingOutcome::Answer(current)
    }

    fn selected_feedback(&self) -> Option<(usize, FeedbackRoute)> {
        let mut best: Option<(usize, &FeedbackArrow)> = None;
        let mut tied = false;
        for (index, arrow) in self.feedback_arrows.iter().enumerate() {
            match best {
                None => {
                    best = Some((index, arrow));
                    tied = false;
                }
                Some((_, current)) if arrow.strength > current.strength => {
                    best = Some((index, arrow));
                    tied = false;
                }
                Some((_, current)) if arrow.strength == current.strength => tied = true,
                Some(_) => {}
            }
        }
        if tied {
            None
        } else {
            best.filter(|(_, arrow)| arrow.strength > 0)
                .map(|(index, arrow)| (index, arrow.route))
        }
    }

    fn erase_temporary(&mut self) {
        self.lookup.erase_temporary();
        self.episode_query = None;
        self.current = None;
        self.result = None;
        self.fault = None;
        self.trace = Vec::new();
    }

    fn temporary_counts(&self) -> (usize, usize) {
        let (lookup_cells, lookup_arrows) = self.lookup.temporary_counts();
        (
            lookup_cells
                + usize::from(self.episode_query.is_some())
                + usize::from(self.current.is_some())
                + usize::from(self.result.is_some()),
            lookup_arrows,
        )
    }

    fn temporary_capacities(&self) -> (usize, usize, usize, usize) {
        let (cells, arrows, relations) = self.lookup.temporary_capacities();
        (cells, arrows, relations, self.trace.capacity())
    }

    fn permanent_counts(&self) -> (usize, usize) {
        let (lookup_cells, lookup_arrows) = self.lookup.permanent_counts();
        (lookup_cells + 4, lookup_arrows + self.feedback_arrows.len())
    }

    fn permanent_fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        fingerprint_mix(&mut hash, self.lookup.permanent_fingerprint());
        fingerprint_mix(&mut hash, self.training_examples);
        fingerprint_mix(&mut hash, APPLY_CELL_ID as u64);
        for arrow in &self.feedback_arrows {
            fingerprint_mix(&mut hash, arrow.route as u64);
            fingerprint_mix(&mut hash, arrow.strength as i64 as u64);
        }
        hash
    }

    fn feedback_strengths(&self) -> Vec<(String, i32)> {
        self.feedback_arrows
            .iter()
            .map(|arrow| (format!("{:?}", arrow.route), arrow.strength))
            .collect()
    }
}

fn fingerprint_mix(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(0x100_0000_01b3);
    }
}

#[derive(Clone, Debug)]
struct IterationEpisode {
    relations: Vec<(OpaqueId, OpaqueId)>,
    query: OpaqueId,
    correct: BindingOutcome,
    distinct_identities: usize,
}

fn chain_episode(
    identities: &mut IdentitySource,
    rng: &mut DeterministicRng,
    depth: usize,
    relation_count: usize,
) -> IterationEpisode {
    assert!(depth > 0);
    assert!(relation_count >= depth);
    let chain: Vec<_> = (0..=depth).map(|_| identities.issue()).collect();
    let mut relations: Vec<_> = chain.windows(2).map(|pair| (pair[0], pair[1])).collect();
    for _ in depth..relation_count {
        relations.push((identities.issue(), identities.issue()));
    }
    rng.shuffle(&mut relations);
    IterationEpisode {
        relations,
        query: chain[0],
        correct: BindingOutcome::Answer(chain[depth]),
        distinct_identities: depth + 1 + (relation_count - depth) * 2,
    }
}

fn present(learner: &mut IterationLearner, episode: &IterationEpisode) {
    learner.begin_episode(episode.query);
    for &(left, right) in &episode.relations {
        learner.observe_relation(left, right);
    }
}

pub(crate) fn frozen_iterable_operation() -> FrozenIterableOperation {
    let mut learner = IterationLearner::new();
    let mut identities = IdentitySource::new(0x2000_f001);
    let mut rng = DeterministicRng::new(0x2000_f002);
    for _ in 0..32 {
        let episode = chain_episode(&mut identities, &mut rng, 2, 10);
        present(&mut learner, &episode);
        learner.apply_pulse();
        learner.apply_pulse();
        learner.learn_from_terminal(episode.correct, 2);
        learner.erase_temporary();
    }
    let (feedback_index, route) = learner
        .selected_feedback()
        .expect("v20 feedback route must be learned before freezing");
    assert_eq!(route, FeedbackRoute::UseResult);
    let (permanent_cells, permanent_arrows) = learner.permanent_counts();
    let permanent_fingerprint = learner.permanent_fingerprint();
    FrozenIterableOperation {
        lookup: learner.lookup,
        feedback_arrow_id: FEEDBACK_ARROW_OFFSET + feedback_index,
        permanent_cells,
        permanent_arrows,
        permanent_fingerprint,
    }
}

fn evaluate(
    learner: &mut IterationLearner,
    episode: &IterationEpisode,
    apply_pulses: usize,
) -> (bool, usize, usize, Vec<ApplyTrace>) {
    present(learner, episode);
    let mut peak_cells = 0;
    let mut peak_arrows = 0;
    for _ in 0..apply_pulses {
        learner.apply_pulse();
        let (cells, arrows) = learner.temporary_counts();
        peak_cells = peak_cells.max(cells);
        peak_arrows = peak_arrows.max(arrows);
    }
    let correct = learner.read() == episode.correct;
    let trace = learner.trace.clone();
    learner.erase_temporary();
    assert_eq!(learner.temporary_counts(), (0, 0));
    assert_eq!(learner.temporary_capacities(), (0, 0, 0, 0));
    (correct, peak_cells, peak_arrows, trace)
}

#[derive(Clone, Debug)]
struct UnrolledTwoStage {
    first_lookup: BindingLearner,
    second_lookup: BindingLearner,
    current: Option<OpaqueId>,
    stage: usize,
    fault: Option<BindingOutcome>,
}

impl UnrolledTwoStage {
    fn new() -> Self {
        Self {
            first_lookup: frozen_lookup_operation(),
            second_lookup: frozen_lookup_operation(),
            current: None,
            stage: 0,
            fault: None,
        }
    }

    fn present(&mut self, episode: &IterationEpisode) {
        self.erase_temporary();
        self.first_lookup.begin_episode();
        self.second_lookup.begin_episode();
        for &(left, right) in &episode.relations {
            self.first_lookup.observe_relation(left, right);
            self.second_lookup.observe_relation(left, right);
        }
        self.current = Some(episode.query);
    }

    fn apply_pulse(&mut self) {
        if self.fault.is_some() || self.stage >= 2 {
            return;
        }
        let Some(current) = self.current else {
            self.fault = Some(BindingOutcome::NotFound);
            return;
        };
        let lookup = if self.stage == 0 {
            &self.first_lookup
        } else {
            &self.second_lookup
        };
        let (outcome, _, _) = lookup.lookup_identity(current);
        match outcome {
            BindingOutcome::Answer(result) => self.current = Some(result),
            BindingOutcome::NotFound | BindingOutcome::Ambiguous => self.fault = Some(outcome),
        }
        self.stage += 1;
    }

    fn read(&self) -> BindingOutcome {
        if let Some(fault) = self.fault {
            return fault;
        }
        self.current
            .map_or(BindingOutcome::NotFound, BindingOutcome::Answer)
    }

    fn erase_temporary(&mut self) {
        self.first_lookup.erase_temporary();
        self.second_lookup.erase_temporary();
        self.current = None;
        self.stage = 0;
        self.fault = None;
    }
}

fn evaluate_unrolled(
    baseline: &mut UnrolledTwoStage,
    episode: &IterationEpisode,
    apply_pulses: usize,
) -> bool {
    baseline.present(episode);
    for _ in 0..apply_pulses {
        baseline.apply_pulse();
    }
    let correct = baseline.read() == episode.correct;
    baseline.erase_temporary();
    correct
}

#[derive(Clone, Debug)]
pub struct IterationCheckpoint {
    pub training_episodes: usize,
    pub permanent_cells: usize,
    pub permanent_arrows: usize,
    pub validation_correct: usize,
    pub validation_total: usize,
}

#[derive(Clone, Debug)]
pub struct DepthResult {
    pub depth: usize,
    pub learner_correct: usize,
    pub supplied_feedback_correct: usize,
    pub unrolled_correct: usize,
    pub total: usize,
    pub average_lookup_spikes: f64,
}

#[derive(Clone, Debug)]
pub struct IterationReport {
    pub checkpoints: Vec<IterationCheckpoint>,
    pub depth_results: Vec<DepthResult>,
    pub held_out_distinct_identities: usize,
    pub trace_reuses_apply_cell: bool,
    pub trace_reuses_lookup_arrow: bool,
    pub trace_reuses_feedback_arrow: bool,
    pub trace_feeds_result_to_next_current: bool,
    pub trace_apply_cell_id: usize,
    pub trace_lookup_arrow_id: usize,
    pub trace_feedback_arrow_id: usize,
    pub missing_intermediate_is_not_found: bool,
    pub ambiguous_intermediate_is_ambiguous: bool,
    pub duplicate_relation_is_single_answer: bool,
    pub permanent_cells: usize,
    pub permanent_arrows: usize,
    pub feedback_strengths: Vec<(String, i32)>,
    pub peak_temporary_cells: usize,
    pub peak_temporary_arrows: usize,
    pub residual_temporary_cells: usize,
    pub residual_temporary_arrows: usize,
    pub temporary_capacity_released: bool,
    pub permanent_fingerprint_unchanged: bool,
    pub passed: bool,
}

fn control_episodes(
    identities: &mut IdentitySource,
) -> (IterationEpisode, IterationEpisode, IterationEpisode) {
    let a = identities.issue();
    let b = identities.issue();
    let c = identities.issue();
    let d = identities.issue();
    let e = identities.issue();
    let missing = IterationEpisode {
        relations: vec![(a, b), (c, d)],
        query: a,
        correct: BindingOutcome::NotFound,
        distinct_identities: 4,
    };
    let ambiguous = IterationEpisode {
        relations: vec![(a, b), (b, c), (b, d)],
        query: a,
        correct: BindingOutcome::Ambiguous,
        distinct_identities: 4,
    };
    let duplicate = IterationEpisode {
        relations: vec![(a, b), (b, e), (b, e), (c, d)],
        query: a,
        correct: BindingOutcome::Answer(e),
        distinct_identities: 5,
    };
    (missing, ambiguous, duplicate)
}

pub fn run_experiment() -> IterationReport {
    let mut learner = IterationLearner::new();
    let mut training_ids = IdentitySource::new(0x2000_0001);
    let mut training_rng = DeterministicRng::new(0x2000_0002);
    let mut validation_ids = IdentitySource::new(0x2000_1001);
    let mut validation_rng = DeterministicRng::new(0x2000_1002);
    let checkpoints_at = [10, 100, 1_000];
    let mut checkpoints = Vec::new();

    for episode_index in 1..=1_000 {
        let episode = chain_episode(&mut training_ids, &mut training_rng, 2, 10);
        present(&mut learner, &episode);
        learner.apply_pulse();
        learner.apply_pulse();
        let _proposal = learner.read();
        learner.learn_from_terminal(episode.correct, 2);
        learner.erase_temporary();

        if checkpoints_at.contains(&episode_index) {
            let fingerprint = learner.permanent_fingerprint();
            let correct = (0..100)
                .filter(|_| {
                    let episode = chain_episode(&mut validation_ids, &mut validation_rng, 2, 10);
                    evaluate(&mut learner, &episode, 2).0
                })
                .count();
            assert_eq!(fingerprint, learner.permanent_fingerprint());
            let (permanent_cells, permanent_arrows) = learner.permanent_counts();
            checkpoints.push(IterationCheckpoint {
                training_episodes: episode_index,
                permanent_cells,
                permanent_arrows,
                validation_correct: correct,
                validation_total: 100,
            });
        }
    }

    let permanent_fingerprint = learner.permanent_fingerprint();
    let mut supplied = IterationLearner::with_supplied_feedback();
    let mut unrolled = UnrolledTwoStage::new();
    let mut held_out_ids = IdentitySource::new(0x2000_2001);
    let mut held_out_rng = DeterministicRng::new(0x2000_2002);
    let mut depth_results = Vec::new();
    let mut held_out_distinct_identities = 0;
    let mut peak_temporary_cells = 0;
    let mut peak_temporary_arrows = 0;
    let mut representative_trace = Vec::new();

    for depth in 1..=4 {
        let total = 1_000;
        let mut learner_correct = 0;
        let mut supplied_feedback_correct = 0;
        let mut unrolled_correct = 0;
        let mut learner_lookup_spikes = 0;
        for episode_index in 0..total {
            let episode = chain_episode(&mut held_out_ids, &mut held_out_rng, depth, 10);
            held_out_distinct_identities += episode.distinct_identities;
            let (correct, cells, arrows, trace) = evaluate(&mut learner, &episode, depth);
            learner_correct += usize::from(correct);
            peak_temporary_cells = peak_temporary_cells.max(cells);
            peak_temporary_arrows = peak_temporary_arrows.max(arrows);
            supplied_feedback_correct += usize::from(evaluate(&mut supplied, &episode, depth).0);
            unrolled_correct += usize::from(evaluate_unrolled(&mut unrolled, &episode, depth));
            learner_lookup_spikes += trace.iter().map(|step| step.lookup_spikes).sum::<usize>();
            if depth == 4 && episode_index == 0 {
                representative_trace = trace;
            }
        }
        depth_results.push(DepthResult {
            depth,
            learner_correct,
            supplied_feedback_correct,
            unrolled_correct,
            total,
            average_lookup_spikes: learner_lookup_spikes as f64 / total as f64,
        });
    }

    let trace_reuses_apply_cell = representative_trace
        .iter()
        .all(|step| step.apply_cell_id == APPLY_CELL_ID);
    let trace_lookup_arrow_id = representative_trace[0].lookup_arrow_id.unwrap();
    let trace_feedback_arrow_id = representative_trace[0].feedback_arrow_id.unwrap();
    let trace_reuses_lookup_arrow = representative_trace
        .iter()
        .all(|step| step.lookup_arrow_id == Some(trace_lookup_arrow_id));
    let trace_reuses_feedback_arrow = representative_trace
        .iter()
        .all(|step| step.feedback_arrow_id == Some(trace_feedback_arrow_id));
    let every_result_becomes_current = representative_trace.iter().all(|step| {
        let BindingOutcome::Answer(result) = step.result else {
            return false;
        };
        step.current_after == Some(result)
    });
    let next_pulse_reads_previous_result = representative_trace.windows(2).all(|pair| {
        let BindingOutcome::Answer(result) = pair[0].result else {
            return false;
        };
        pair[1].current_before == result
    });
    let trace_feeds_result_to_next_current =
        every_result_becomes_current && next_pulse_reads_previous_result;

    let mut control_ids = IdentitySource::new(0x2000_3001);
    let (missing, ambiguous, duplicate) = control_episodes(&mut control_ids);
    let missing_intermediate_is_not_found = evaluate(&mut learner, &missing, 2).0;
    let ambiguous_intermediate_is_ambiguous = evaluate(&mut learner, &ambiguous, 2).0;
    let duplicate_relation_is_single_answer = evaluate(&mut learner, &duplicate, 2).0;

    let (residual_temporary_cells, residual_temporary_arrows) = learner.temporary_counts();
    let temporary_capacity_released = learner.temporary_capacities() == (0, 0, 0, 0);
    let permanent_fingerprint_unchanged = permanent_fingerprint == learner.permanent_fingerprint();
    let (permanent_cells, permanent_arrows) = learner.permanent_counts();
    let feedback_strengths = learner.feedback_strengths();
    let checkpoint_accuracy = checkpoints
        .iter()
        .all(|point| point.validation_correct == point.validation_total);
    let structure_plateaued = checkpoints.windows(2).all(|pair| {
        pair[0].permanent_cells == pair[1].permanent_cells
            && pair[0].permanent_arrows == pair[1].permanent_arrows
    });
    let learner_all_depths = depth_results
        .iter()
        .all(|point| point.learner_correct == point.total);
    let supplied_all_depths = depth_results
        .iter()
        .all(|point| point.supplied_feedback_correct == point.total);
    let unrolled_signature = depth_results.iter().all(|point| match point.depth {
        1 | 2 => point.unrolled_correct == point.total,
        3 | 4 => point.unrolled_correct == 0,
        _ => false,
    });
    let passed = checkpoint_accuracy
        && structure_plateaued
        && learner_all_depths
        && supplied_all_depths
        && unrolled_signature
        && trace_reuses_apply_cell
        && trace_reuses_lookup_arrow
        && trace_reuses_feedback_arrow
        && trace_feeds_result_to_next_current
        && missing_intermediate_is_not_found
        && ambiguous_intermediate_is_ambiguous
        && duplicate_relation_is_single_answer
        && residual_temporary_cells == 0
        && residual_temporary_arrows == 0
        && temporary_capacity_released
        && permanent_fingerprint_unchanged;

    IterationReport {
        checkpoints,
        depth_results,
        held_out_distinct_identities,
        trace_reuses_apply_cell,
        trace_reuses_lookup_arrow,
        trace_reuses_feedback_arrow,
        trace_feeds_result_to_next_current,
        trace_apply_cell_id: APPLY_CELL_ID,
        trace_lookup_arrow_id,
        trace_feedback_arrow_id,
        missing_intermediate_is_not_found,
        ambiguous_intermediate_is_ambiguous,
        duplicate_relation_is_single_answer,
        permanent_cells,
        permanent_arrows,
        feedback_strengths,
        peak_temporary_cells,
        peak_temporary_arrows,
        residual_temporary_cells,
        residual_temporary_arrows,
        temporary_capacity_released,
        permanent_fingerprint_unchanged,
        passed,
    }
}

pub fn print_report(report: &IterationReport) {
    println!("v20 iterable lookup:");
    print!("  checkpoints episodes:cells/arrows/accuracy:");
    for checkpoint in &report.checkpoints {
        print!(
            " {}:{}/{}/{}/{}",
            checkpoint.training_episodes,
            checkpoint.permanent_cells,
            checkpoint.permanent_arrows,
            checkpoint.validation_correct,
            checkpoint.validation_total
        );
    }
    println!();
    for depth in &report.depth_results {
        println!(
            "  depth {} learner/supplied/unrolled: {}/{}, {}/{}, {}/{}, lookup spikes/query={:.1}",
            depth.depth,
            depth.learner_correct,
            depth.total,
            depth.supplied_feedback_correct,
            depth.total,
            depth.unrolled_correct,
            depth.total,
            depth.average_lookup_spikes
        );
    }
    println!(
        "  trace apply/lookup/feedback ids={}/{}/{}, reused={}/{}/{}, feedback chain={}",
        report.trace_apply_cell_id,
        report.trace_lookup_arrow_id,
        report.trace_feedback_arrow_id,
        report.trace_reuses_apply_cell,
        report.trace_reuses_lookup_arrow,
        report.trace_reuses_feedback_arrow,
        report.trace_feeds_result_to_next_current
    );
    println!(
        "  controls missing/ambiguous/duplicate={}/{}/{}, permanent cells/arrows={}/{}, held-out identities={}",
        report.missing_intermediate_is_not_found,
        report.ambiguous_intermediate_is_ambiguous,
        report.duplicate_relation_is_single_answer,
        report.permanent_cells,
        report.permanent_arrows,
        report.held_out_distinct_identities
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v20_selects_feedback_from_terminal_supervision() {
        let report = run_experiment();
        let selected = report
            .feedback_strengths
            .iter()
            .find(|(route, _)| route == "UseResult")
            .unwrap()
            .1;
        let strongest_alternative = report
            .feedback_strengths
            .iter()
            .filter(|(route, _)| route != "UseResult")
            .map(|(_, strength)| *strength)
            .max()
            .unwrap();

        assert!(selected > strongest_alternative);
        assert!(report
            .checkpoints
            .iter()
            .all(|point| point.validation_correct == point.validation_total));
    }

    #[test]
    fn v20_reuses_one_lookup_and_feedback_route_at_every_depth() {
        let report = run_experiment();

        assert!(report
            .depth_results
            .iter()
            .all(|point| point.learner_correct == point.total));
        assert!(report.trace_reuses_apply_cell);
        assert!(report.trace_reuses_lookup_arrow);
        assert!(report.trace_reuses_feedback_arrow);
        assert!(report.trace_feeds_result_to_next_current);
    }

    #[test]
    fn v20_baselines_distinguish_iteration_from_unrolling() {
        let report = run_experiment();

        for point in &report.depth_results {
            assert_eq!(point.supplied_feedback_correct, point.total);
            if point.depth <= 2 {
                assert_eq!(point.unrolled_correct, point.total);
            } else {
                assert_eq!(point.unrolled_correct, 0);
            }
        }
    }

    #[test]
    fn v20_controls_and_held_out_evaluation_preserve_state() {
        let report = run_experiment();

        assert!(report.missing_intermediate_is_not_found);
        assert!(report.ambiguous_intermediate_is_ambiguous);
        assert!(report.duplicate_relation_is_single_answer);
        assert_eq!(report.residual_temporary_cells, 0);
        assert_eq!(report.residual_temporary_arrows, 0);
        assert!(report.temporary_capacity_released);
        assert!(report.permanent_fingerprint_unchanged);
        assert!(report.passed);
    }
}
