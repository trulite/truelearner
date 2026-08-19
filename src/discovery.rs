use std::cell::Cell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use crate::binding::{BindingOutcome, IdentitySource, OpaqueId};

const COACTIVITY_WINDOW: usize = 2;
const CONSOLIDATION_STRENGTH: i32 = 4;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Role {
    Query,
    Same,
    Relation,
    Slot1,
    Slot2,
    Answer,
    Cue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    LeftToRight,
    RightToLeft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CandidateArrow {
    from: Role,
    to: Role,
    strength: i32,
    uses: usize,
    trace: bool,
}

#[derive(Clone, Debug)]
struct Relation {
    left: OpaqueId,
    right: OpaqueId,
    cued: bool,
}

#[derive(Clone, Debug)]
struct Episode {
    relations: Vec<Relation>,
    query: OpaqueId,
    correct: BindingOutcome,
}

#[derive(Clone, Copy, Debug, Default)]
struct EpisodeWork {
    sensory_spikes: usize,
    answer_spikes: usize,
    peak_temporary_cells: usize,
    peak_temporary_arrows: usize,
}

impl EpisodeWork {
    fn total_spikes(self) -> usize {
        self.sensory_spikes + self.answer_spikes
    }
}

#[derive(Clone, Debug)]
struct DiscoveryLearner {
    candidates: Vec<CandidateArrow>,
    consolidated: Option<CandidateArrow>,
    recent_roles: VecDeque<(usize, Role)>,
    clock: usize,
    exploration_cursor: usize,
    proposals: usize,
    rejected: usize,
    training_spikes: usize,
    episodes_to_competence: Option<usize>,
    last_used_route: Option<(Role, Role)>,
    last_episode_spikes: usize,
}

impl DiscoveryLearner {
    fn new(seed: u64) -> Self {
        Self {
            candidates: Vec::new(),
            consolidated: None,
            recent_roles: VecDeque::new(),
            clock: 0,
            exploration_cursor: seed as usize,
            proposals: 0,
            rejected: 0,
            training_spikes: 0,
            episodes_to_competence: None,
            last_used_route: None,
            last_episode_spikes: 0,
        }
    }

    fn present(&mut self, episode: &Episode, allow_proposals: bool) -> EpisodeWork {
        self.recent_roles.clear();
        let mut work = EpisodeWork {
            peak_temporary_cells: episode.relations.len() * 3 + 1,
            peak_temporary_arrows: episode.relations.len() * 2,
            ..EpisodeWork::default()
        };

        for relation in &episode.relations {
            self.activate(Role::Relation, allow_proposals, &mut work);
            if relation.cued {
                self.activate(Role::Cue, allow_proposals, &mut work);
            }
            self.activate(Role::Slot1, allow_proposals, &mut work);
            self.activate(Role::Slot2, allow_proposals, &mut work);
        }
        self.activate(Role::Query, allow_proposals, &mut work);
        self.activate(Role::Same, allow_proposals, &mut work);
        self.activate(Role::Answer, allow_proposals, &mut work);
        work
    }

    fn activate(&mut self, role: Role, allow_proposals: bool, work: &mut EpisodeWork) {
        self.clock += 1;
        work.sensory_spikes += 1;
        while self
            .recent_roles
            .front()
            .is_some_and(|(time, _)| self.clock - *time > COACTIVITY_WINDOW)
        {
            self.recent_roles.pop_front();
        }
        if allow_proposals && self.consolidated.is_none() {
            let nearby: Vec<_> = self.recent_roles.iter().map(|(_, role)| *role).collect();
            for previous in nearby {
                self.propose(previous, role);
                self.propose(role, previous);
            }
        }
        self.recent_roles.push_back((self.clock, role));
    }

    fn propose(&mut self, from: Role, to: Role) {
        if from == to
            || self
                .candidates
                .iter()
                .any(|arrow| arrow.from == from && arrow.to == to)
        {
            return;
        }
        self.candidates.push(CandidateArrow {
            from,
            to,
            strength: 0,
            uses: 0,
            trace: false,
        });
        self.proposals += 1;
        self.candidates.sort_by_key(|arrow| (arrow.from, arrow.to));
    }

    fn propose_answer(&mut self, episode: &Episode, work: &mut EpisodeWork) -> BindingOutcome {
        let selected = if let Some(route) = self.consolidated {
            route
        } else {
            let Some(index) = self.exploration_choice() else {
                return BindingOutcome::NotFound;
            };
            self.candidates[index].uses += 1;
            self.candidates[index].trace = true;
            self.candidates[index]
        };
        self.last_used_route = Some((selected.from, selected.to));
        work.answer_spikes += 1;
        execute_arrow(selected, episode, work)
    }

    fn exploration_choice(&mut self) -> Option<usize> {
        let minimum_uses = self.candidates.iter().map(|arrow| arrow.uses).min()?;
        let choices: Vec<_> = self
            .candidates
            .iter()
            .enumerate()
            .filter_map(|(index, arrow)| (arrow.uses == minimum_uses).then_some(index))
            .collect();
        let selected = choices[self.exploration_cursor % choices.len()];
        self.exploration_cursor = self.exploration_cursor.wrapping_add(1);
        Some(selected)
    }

    fn terminal_feedback(
        &mut self,
        success: bool,
        episode_number: usize,
        allow_consolidation: bool,
    ) {
        if self.consolidated.is_some() {
            return;
        }
        for arrow in &mut self.candidates {
            if arrow.trace {
                if success {
                    arrow.strength += 1;
                } else {
                    arrow.strength -= 1;
                }
                arrow.trace = false;
            }
        }
        if allow_consolidation {
            self.consolidate_if_ready(episode_number);
        }
    }

    fn consolidate_if_ready(&mut self, episode_number: usize) {
        if self.consolidated.is_some() {
            return;
        }
        if let Some(winner) = self
            .candidates
            .iter()
            .copied()
            .find(|arrow| arrow.strength >= CONSOLIDATION_STRENGTH)
        {
            self.rejected += self.candidates.len().saturating_sub(1);
            self.candidates.clear();
            self.consolidated = Some(winner);
            self.episodes_to_competence = Some(episode_number);
        }
    }

    fn consolidate_unique_if_ready(&mut self, episode_number: usize) {
        if self.consolidated.is_some() {
            return;
        }
        let Some(best_strength) = self.candidates.iter().map(|arrow| arrow.strength).max() else {
            return;
        };
        if best_strength < CONSOLIDATION_STRENGTH {
            return;
        }
        let mut strongest = self
            .candidates
            .iter()
            .copied()
            .filter(|arrow| arrow.strength == best_strength);
        let Some(winner) = strongest.next() else {
            return;
        };
        if strongest.next().is_some() {
            return;
        }
        self.rejected += self.candidates.len().saturating_sub(1);
        self.candidates.clear();
        self.consolidated = Some(winner);
        self.episodes_to_competence = Some(episode_number);
    }

    fn train_episode(&mut self, episode: &Episode, episode_number: usize) -> bool {
        self.train_episode_internal(episode, episode_number, true)
    }

    fn train_episode_deferred(&mut self, episode: &Episode, episode_number: usize) -> bool {
        self.train_episode_internal(episode, episode_number, false)
    }

    fn train_episode_internal(
        &mut self,
        episode: &Episode,
        episode_number: usize,
        allow_consolidation: bool,
    ) -> bool {
        let mut work = self.present(episode, true);
        let proposal = self.propose_answer(episode, &mut work);
        let success = proposal == episode.correct;
        self.terminal_feedback(success, episode_number, allow_consolidation);
        self.last_episode_spikes = work.total_spikes();
        self.training_spikes += self.last_episode_spikes;
        success
    }

    fn evaluate(&mut self, episode: &Episode) -> (BindingOutcome, EpisodeWork) {
        let mut work = self.present(episode, false);
        let answer = self
            .evaluation_route()
            .map_or(BindingOutcome::NotFound, |route| {
                work.answer_spikes += 1;
                execute_arrow(route, episode, &mut work)
            });
        (answer, work)
    }

    fn evaluation_route(&self) -> Option<CandidateArrow> {
        if let Some(route) = self.consolidated {
            return Some(route);
        }
        let best_strength = self.candidates.iter().map(|arrow| arrow.strength).max()?;
        if best_strength <= 0 {
            return None;
        }
        let mut strongest = self
            .candidates
            .iter()
            .copied()
            .filter(|arrow| arrow.strength == best_strength);
        let route = strongest.next()?;
        strongest.next().is_none().then_some(route)
    }

    fn permanent_fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        let arrows = self
            .consolidated
            .iter()
            .chain(self.candidates.iter())
            .copied();
        for arrow in arrows {
            fingerprint_mix(&mut hash, arrow.from as u64);
            fingerprint_mix(&mut hash, arrow.to as u64);
            fingerprint_mix(&mut hash, arrow.strength as i64 as u64);
            fingerprint_mix(&mut hash, arrow.uses as u64);
        }
        hash
    }

    fn selected_route(&self) -> Option<(Role, Role)> {
        self.consolidated.map(|arrow| (arrow.from, arrow.to))
    }

    fn live_candidates(&self) -> usize {
        self.candidates.len()
    }

    fn stable_arrows(&self) -> usize {
        usize::from(self.consolidated.is_some())
    }

    fn route_strength(&self, from: Role, to: Role) -> Option<i32> {
        self.consolidated
            .filter(|arrow| arrow.from == from && arrow.to == to)
            .map(|arrow| arrow.strength)
            .or_else(|| {
                self.candidates
                    .iter()
                    .find(|arrow| arrow.from == from && arrow.to == to)
                    .map(|arrow| arrow.strength)
            })
    }

    fn plausible_routes(&self) -> Vec<(Role, Role)> {
        if let Some(route) = self.consolidated {
            return vec![(route.from, route.to)];
        }
        self.candidates
            .iter()
            .filter_map(|arrow| (arrow.strength > 0).then_some((arrow.from, arrow.to)))
            .collect()
    }
}

fn execute_arrow(
    arrow: CandidateArrow,
    episode: &Episode,
    work: &mut EpisodeWork,
) -> BindingOutcome {
    let mut outputs = HashSet::new();
    match (arrow.from, arrow.to) {
        (Role::Slot1, Role::Slot1 | Role::Slot2) | (Role::Slot2, Role::Slot1 | Role::Slot2) => {
            for relation in &episode.relations {
                work.answer_spikes += 1;
                let matched = match arrow.from {
                    Role::Slot1 => relation.left == episode.query,
                    Role::Slot2 => relation.right == episode.query,
                    _ => unreachable!(),
                };
                if matched {
                    work.answer_spikes += 2;
                    outputs.insert(match arrow.to {
                        Role::Slot1 => relation.left,
                        Role::Slot2 => relation.right,
                        _ => unreachable!(),
                    });
                }
            }
        }
        (Role::Cue, Role::Slot1 | Role::Slot2) => {
            for relation in &episode.relations {
                work.answer_spikes += 1;
                if relation.cued {
                    work.answer_spikes += 1;
                    outputs.insert(match arrow.to {
                        Role::Slot1 => relation.left,
                        Role::Slot2 => relation.right,
                        _ => unreachable!(),
                    });
                }
            }
        }
        _ => {}
    }
    match outputs.len() {
        0 => BindingOutcome::NotFound,
        1 => BindingOutcome::Answer(*outputs.iter().next().unwrap()),
        _ => BindingOutcome::Ambiguous,
    }
}

fn fingerprint_mix(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(0x100_0000_01b3);
    }
}

fn normal_episode(
    identities: &mut IdentitySource,
    relation_count: usize,
    target: usize,
    direction: Direction,
    cue_target: bool,
) -> Episode {
    let mut relations = Vec::with_capacity(relation_count);
    for index in 0..relation_count {
        relations.push(Relation {
            left: identities.issue(),
            right: identities.issue(),
            cued: cue_target && index == target,
        });
    }
    let (query, answer) = match direction {
        Direction::LeftToRight => (relations[target].left, relations[target].right),
        Direction::RightToLeft => (relations[target].right, relations[target].left),
    };
    rotate_relations(&mut relations, target);
    Episode {
        relations,
        query,
        correct: BindingOutcome::Answer(answer),
    }
}

fn random_label_episode(
    identities: &mut IdentitySource,
    relation_count: usize,
    target: usize,
    label: usize,
) -> Episode {
    let mut episode = normal_episode(
        identities,
        relation_count,
        target,
        Direction::LeftToRight,
        false,
    );
    let answer = episode.relations[label % relation_count].right;
    episode.correct = BindingOutcome::Answer(answer);
    episode
}

fn rotate_relations(relations: &mut [Relation], amount: usize) {
    if !relations.is_empty() {
        let length = relations.len();
        relations.rotate_left(amount % length);
    }
}

#[derive(Clone, Debug)]
pub struct DiscoveryCheckpoint {
    pub episodes: usize,
    pub live_candidates: usize,
    pub stable_arrows: usize,
    pub validation_correct: usize,
    pub validation_total: usize,
}

#[derive(Clone, Debug)]
pub struct SeedResult {
    pub seed: u64,
    pub correct: usize,
    pub total: usize,
    pub route: String,
    pub episodes_to_competence: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct DiscoveryReport {
    pub checkpoints: Vec<DiscoveryCheckpoint>,
    pub forward_seeds: Vec<SeedResult>,
    pub reverse_seeds: Vec<SeedResult>,
    pub forward_route: String,
    pub reverse_route: String,
    pub proposed_arrows: usize,
    pub rejected_arrows: usize,
    pub stable_arrows: usize,
    pub training_spikes: usize,
    pub peak_temporary_cells: usize,
    pub peak_temporary_arrows: usize,
    pub held_out_fingerprint_unchanged: bool,
    pub random_label_training_correct: usize,
    pub random_label_training_total: usize,
    pub random_label_stable_arrows: usize,
    pub shortcut_runs: usize,
    pub shortcut_failures_without_cue: usize,
    pub passed: bool,
}

fn route_name(route: Option<(Role, Role)>) -> String {
    route.map_or_else(
        || "none".to_string(),
        |(from, to)| format!("{from:?}->{to:?}"),
    )
}

fn train_direction(seed: u64, direction: Direction, episodes: usize) -> DiscoveryLearner {
    let mut learner = DiscoveryLearner::new(seed);
    let mut identities = IdentitySource::new(seed ^ 0xd000_1000);
    for episode_number in 1..=episodes {
        let target = ((seed as usize).wrapping_add(episode_number * 7)) % 10;
        let episode = normal_episode(&mut identities, 10, target, direction, false);
        learner.train_episode(&episode, episode_number);
    }
    learner
}

fn held_out_accuracy(
    learner: &mut DiscoveryLearner,
    seed: u64,
    direction: Direction,
    count: usize,
    cue_target: bool,
) -> (usize, usize, usize) {
    let mut identities = IdentitySource::new(seed ^ 0xd000_2000);
    let mut correct = 0;
    let mut peak_cells = 0;
    let mut peak_arrows = 0;
    for index in 0..count {
        let episode = normal_episode(
            &mut identities,
            10,
            (index * 3 + seed as usize) % 10,
            direction,
            cue_target,
        );
        let (answer, work) = learner.evaluate(&episode);
        correct += usize::from(answer == episode.correct);
        peak_cells = peak_cells.max(work.peak_temporary_cells);
        peak_arrows = peak_arrows.max(work.peak_temporary_arrows);
    }
    (correct, peak_cells, peak_arrows)
}

pub fn run_experiment() -> DiscoveryReport {
    let checkpoints_at = [10, 100, 1_000, 10_000];
    let mut representative = DiscoveryLearner::new(0xd000_0001);
    let mut training_ids = IdentitySource::new(0xd000_0002);
    let mut checkpoints = Vec::new();
    for episode_number in 1..=10_000 {
        let episode = normal_episode(
            &mut training_ids,
            10,
            episode_number * 7 % 10,
            Direction::LeftToRight,
            false,
        );
        representative.train_episode(&episode, episode_number);
        if checkpoints_at.contains(&episode_number) {
            let before = representative.permanent_fingerprint();
            let (correct, _, _) = held_out_accuracy(
                &mut representative,
                0xd000_0100 + episode_number as u64,
                Direction::LeftToRight,
                100,
                false,
            );
            assert_eq!(before, representative.permanent_fingerprint());
            checkpoints.push(DiscoveryCheckpoint {
                episodes: episode_number,
                live_candidates: representative.live_candidates(),
                stable_arrows: representative.stable_arrows(),
                validation_correct: correct,
                validation_total: 100,
            });
        }
    }

    let mut forward_seeds = Vec::new();
    let mut reverse_seeds = Vec::new();
    for seed in 1..=8 {
        let mut forward = train_direction(0xd010_0000 + seed, Direction::LeftToRight, 1_000);
        let (correct, _, _) = held_out_accuracy(
            &mut forward,
            0xd011_0000 + seed,
            Direction::LeftToRight,
            500,
            false,
        );
        forward_seeds.push(SeedResult {
            seed,
            correct,
            total: 500,
            route: route_name(forward.selected_route()),
            episodes_to_competence: forward.episodes_to_competence,
        });

        let mut reverse = train_direction(0xd020_0000 + seed, Direction::RightToLeft, 1_000);
        let (correct, _, _) = held_out_accuracy(
            &mut reverse,
            0xd021_0000 + seed,
            Direction::RightToLeft,
            500,
            false,
        );
        reverse_seeds.push(SeedResult {
            seed,
            correct,
            total: 500,
            route: route_name(reverse.selected_route()),
            episodes_to_competence: reverse.episodes_to_competence,
        });
    }

    let fingerprint_before = representative.permanent_fingerprint();
    let (_, peak_temporary_cells, peak_temporary_arrows) = held_out_accuracy(
        &mut representative,
        0xd030_0001,
        Direction::LeftToRight,
        20_000,
        false,
    );
    let held_out_fingerprint_unchanged =
        fingerprint_before == representative.permanent_fingerprint();

    let mut random_labels = DiscoveryLearner::new(0xd040_0001);
    let mut random_ids = IdentitySource::new(0xd040_0002);
    let mut random_label_training_correct = 0;
    for episode_number in 1..=10_000 {
        let target = episode_number * 7 % 10;
        let label = episode_number * 3 + 1;
        let episode = random_label_episode(&mut random_ids, 10, target, label);
        random_label_training_correct +=
            usize::from(random_labels.train_episode(&episode, episode_number));
    }

    let shortcut_runs = 32;
    let mut shortcut_failures_without_cue = 0;
    for seed in 0..shortcut_runs {
        let mut learner = DiscoveryLearner::new(0xd050_0000 + seed as u64);
        let mut identities = IdentitySource::new(0xd051_0000 + seed as u64);
        for episode_number in 1..=1_000 {
            let episode = normal_episode(
                &mut identities,
                10,
                episode_number * 7 % 10,
                Direction::LeftToRight,
                true,
            );
            learner.train_episode(&episode, episode_number);
        }
        let (correct, _, _) = held_out_accuracy(
            &mut learner,
            0xd052_0000 + seed as u64,
            Direction::LeftToRight,
            100,
            false,
        );
        shortcut_failures_without_cue += usize::from(correct < 100);
    }

    let forward_route = route_name(representative.selected_route());
    let reverse_route = reverse_seeds
        .first()
        .map_or_else(|| "none".to_string(), |seed| seed.route.clone());
    let all_forward_pass = forward_seeds
        .iter()
        .all(|result| result.correct == result.total && result.route == "Slot1->Slot2");
    let all_reverse_pass = reverse_seeds
        .iter()
        .all(|result| result.correct == result.total && result.route == "Slot2->Slot1");
    let random_control_pass =
        random_labels.stable_arrows() == 0 && random_label_training_correct < 1_500;
    let passed = all_forward_pass
        && all_reverse_pass
        && forward_route == "Slot1->Slot2"
        && reverse_route == "Slot2->Slot1"
        && held_out_fingerprint_unchanged
        && representative.stable_arrows() == 1
        && representative.proposals > representative.stable_arrows()
        && random_control_pass
        && shortcut_failures_without_cue > 0;

    DiscoveryReport {
        checkpoints,
        forward_seeds,
        reverse_seeds,
        forward_route,
        reverse_route,
        proposed_arrows: representative.proposals,
        rejected_arrows: representative.rejected,
        stable_arrows: representative.stable_arrows(),
        training_spikes: representative.training_spikes,
        peak_temporary_cells,
        peak_temporary_arrows,
        held_out_fingerprint_unchanged,
        random_label_training_correct,
        random_label_training_total: 10_000,
        random_label_stable_arrows: random_labels.stable_arrows(),
        shortcut_runs,
        shortcut_failures_without_cue,
        passed,
    }
}

pub fn print_report(report: &DiscoveryReport) {
    println!("d0 discovered routing topology:");
    for checkpoint in &report.checkpoints {
        println!(
            "  episodes {:>5}: live candidates={}, stable arrows={}, validation={}/{}",
            checkpoint.episodes,
            checkpoint.live_candidates,
            checkpoint.stable_arrows,
            checkpoint.validation_correct,
            checkpoint.validation_total
        );
    }
    println!(
        "  learned forward/reverse routes: {} / {}",
        report.forward_route, report.reverse_route
    );
    println!(
        "  proposed/rejected/stable arrows: {}/{}/{}",
        report.proposed_arrows, report.rejected_arrows, report.stable_arrows
    );
    println!(
        "  random-label successes={}/{}, stable arrows={}",
        report.random_label_training_correct,
        report.random_label_training_total,
        report.random_label_stable_arrows
    );
    println!(
        "  shortcut failures after cue removal: {}/{}",
        report.shortcut_failures_without_cue, report.shortcut_runs
    );
    println!(
        "  peak temporary cells/arrows={}/{}, held-out fingerprint unchanged={}",
        report.peak_temporary_cells,
        report.peak_temporary_arrows,
        report.held_out_fingerprint_unchanged
    );
}

const INTERVENTION_BASES: usize = 18;
const INTERVENTION_STATES: usize = 22;
const INTERVENTION_BLOCKS: usize = 2;
const INTERVENTION_BUDGET: usize = INTERVENTION_BASES * INTERVENTION_STATES * INTERVENTION_BLOCKS;

#[derive(Clone, Debug)]
struct ContrastBase {
    episode: Episode,
    changed_answer: OpaqueId,
}

#[derive(Clone, Debug, Default)]
struct CurriculumAudit {
    cue_offsets: [usize; 11],
    unchanged_answers: usize,
    changed_answers: usize,
}

#[derive(Clone, Debug)]
pub struct StrengthPoint {
    pub episode: usize,
    pub real_route_strength: Option<i32>,
    pub shortcut_route_strength: Option<i32>,
    pub used_route: String,
    pub success: bool,
    pub live_candidates: usize,
    pub spikes: usize,
}

#[derive(Clone, Debug)]
pub struct InterventionSeedResult {
    pub seed: u64,
    pub observation_route: String,
    pub observation_correct: usize,
    pub contrasting_route: String,
    pub contrasting_correct: usize,
    pub total: usize,
    pub contrasting_episodes_to_competence: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct InterventionReport {
    pub training_budget: usize,
    pub seed_results: Vec<InterventionSeedResult>,
    pub observation_shortcut_failures: usize,
    pub contrasting_shortcut_failures: usize,
    pub contrasting_forward_routes: usize,
    pub cue_offset_counts: Vec<usize>,
    pub unchanged_answer_episodes: usize,
    pub changed_answer_episodes: usize,
    pub representative_history: Vec<StrengthPoint>,
    pub representative_competence_episode: Option<usize>,
    pub representative_real_peak: i32,
    pub representative_shortcut_peak: i32,
    pub representative_shortcut_last: i32,
    pub held_out_fingerprint_unchanged: bool,
    pub random_label_stable_arrows: usize,
    pub random_label_training_correct: usize,
    pub passed: bool,
}

fn intervention_states() -> Vec<(usize, bool)> {
    let mut states = vec![(0, false), (1, false), (1, true), (10, true)];
    for changed in [false, true] {
        for cue_offset in 0..=10 {
            if !states.contains(&(cue_offset, changed)) {
                states.push((cue_offset, changed));
            }
        }
    }
    debug_assert_eq!(states.len(), INTERVENTION_STATES);
    states
}

fn make_contrast_bases(
    identities: &mut IdentitySource,
    seed: u64,
    block: usize,
) -> Vec<ContrastBase> {
    (0..INTERVENTION_BASES)
        .map(|index| {
            let target = (index * 7 + block * 3 + seed as usize) % 10;
            let mut episode = normal_episode(identities, 10, target, Direction::LeftToRight, false);
            let shift = (index * 3 + block * 7 + seed as usize) % episode.relations.len();
            episode.relations.rotate_left(shift);
            if (index + block + seed as usize).is_multiple_of(2) {
                episode.relations.reverse();
            }
            ContrastBase {
                episode,
                changed_answer: identities.issue(),
            }
        })
        .collect()
}

fn contrast_variant(base: &ContrastBase, cue_offset: usize, changed: bool) -> Episode {
    let mut episode = base.episode.clone();
    for relation in &mut episode.relations {
        relation.cued = false;
    }
    let target = episode
        .relations
        .iter()
        .position(|relation| relation.left == episode.query)
        .expect("query identity must occur in slot one");
    if changed {
        episode.relations[target].right = base.changed_answer;
        episode.correct = BindingOutcome::Answer(base.changed_answer);
    }
    if cue_offset < episode.relations.len() {
        let cued = (target + cue_offset) % episode.relations.len();
        episode.relations[cued].cued = true;
    }
    episode
}

fn intervention_curricula(seed: u64) -> (Vec<Episode>, Vec<Episode>, CurriculumAudit) {
    let mut identities = IdentitySource::new(seed ^ 0xd100_1000);
    let states = intervention_states();
    let mut observation = Vec::with_capacity(INTERVENTION_BUDGET);
    let mut contrasting = Vec::with_capacity(INTERVENTION_BUDGET);
    let mut audit = CurriculumAudit::default();

    for block in 0..INTERVENTION_BLOCKS {
        let bases = make_contrast_bases(&mut identities, seed, block);
        for (state_index, &(cue_offset, changed)) in states.iter().enumerate() {
            let start = (state_index * 5 + block * 7 + seed as usize) % bases.len();
            for offset in 0..bases.len() {
                let base = &bases[(start + offset) % bases.len()];
                contrasting.push(contrast_variant(base, cue_offset, changed));
                observation.push(contrast_variant(base, 0, changed));
                audit.cue_offsets[cue_offset] += 1;
                if changed {
                    audit.changed_answers += 1;
                } else {
                    audit.unchanged_answers += 1;
                }
            }
        }
    }
    debug_assert_eq!(observation.len(), INTERVENTION_BUDGET);
    debug_assert_eq!(contrasting.len(), INTERVENTION_BUDGET);
    (observation, contrasting, audit)
}

fn train_with_history(
    seed: u64,
    curriculum: &[Episode],
    record_history: bool,
) -> (DiscoveryLearner, Vec<StrengthPoint>) {
    let mut learner = DiscoveryLearner::new(seed);
    let mut history = Vec::with_capacity(usize::from(record_history) * curriculum.len());
    for (index, episode) in curriculum.iter().enumerate() {
        let success = learner.train_episode(episode, index + 1);
        if record_history {
            history.push(StrengthPoint {
                episode: index + 1,
                real_route_strength: learner.route_strength(Role::Slot1, Role::Slot2),
                shortcut_route_strength: learner.route_strength(Role::Cue, Role::Slot2),
                used_route: route_name(learner.last_used_route),
                success,
                live_candidates: learner.live_candidates(),
                spikes: learner.last_episode_spikes,
            });
        }
    }
    (learner, history)
}

fn d1_random_label_control() -> (usize, usize) {
    let mut learner = DiscoveryLearner::new(0xd140_0001);
    let mut identities = IdentitySource::new(0xd140_0002);
    let mut correct = 0;
    for episode_number in 1..=10_000 {
        let episode = random_label_episode(
            &mut identities,
            10,
            episode_number * 7 % 10,
            episode_number * 3 + 1,
        );
        correct += usize::from(learner.train_episode(&episode, episode_number));
    }
    (learner.stable_arrows(), correct)
}

pub fn run_intervention_experiment() -> InterventionReport {
    let mut seed_results = Vec::new();
    let mut observation_shortcut_failures = 0;
    let mut contrasting_shortcut_failures = 0;
    let mut contrasting_forward_routes = 0;
    let mut representative_history = Vec::new();
    let mut representative_learner = None;
    let mut representative_audit = None;

    for seed in 0..32 {
        let learner_seed = 0xd110_0000 + seed as u64;
        let (observation, contrasting, audit) = intervention_curricula(0xd120_0000 + seed as u64);
        let (mut observation_learner, _) = train_with_history(learner_seed, &observation, false);
        let (mut contrasting_learner, history) =
            train_with_history(learner_seed, &contrasting, seed == 0);

        let (observation_correct, _, _) = held_out_accuracy(
            &mut observation_learner,
            0xd130_0000 + seed as u64,
            Direction::LeftToRight,
            500,
            false,
        );
        let (contrasting_correct, _, _) = held_out_accuracy(
            &mut contrasting_learner,
            0xd131_0000 + seed as u64,
            Direction::LeftToRight,
            500,
            false,
        );
        observation_shortcut_failures += usize::from(observation_correct < 500);
        contrasting_shortcut_failures += usize::from(contrasting_correct < 500);
        contrasting_forward_routes +=
            usize::from(contrasting_learner.selected_route() == Some((Role::Slot1, Role::Slot2)));
        seed_results.push(InterventionSeedResult {
            seed: seed as u64,
            observation_route: route_name(observation_learner.selected_route()),
            observation_correct,
            contrasting_route: route_name(contrasting_learner.selected_route()),
            contrasting_correct,
            total: 500,
            contrasting_episodes_to_competence: contrasting_learner.episodes_to_competence,
        });
        if seed == 0 {
            representative_history = history;
            representative_learner = Some(contrasting_learner);
            representative_audit = Some(audit);
        }
    }

    let mut representative_learner =
        representative_learner.expect("representative learner must exist");
    let fingerprint_before = representative_learner.permanent_fingerprint();
    let _ = held_out_accuracy(
        &mut representative_learner,
        0xd150_0001,
        Direction::LeftToRight,
        20_000,
        false,
    );
    let held_out_fingerprint_unchanged =
        fingerprint_before == representative_learner.permanent_fingerprint();
    let audit = representative_audit.expect("representative audit must exist");
    let (random_label_stable_arrows, random_label_training_correct) = d1_random_label_control();
    let representative_real_peak = representative_history
        .iter()
        .filter_map(|point| point.real_route_strength)
        .max()
        .unwrap_or(0);
    let representative_shortcut_peak = representative_history
        .iter()
        .filter_map(|point| point.shortcut_route_strength)
        .max()
        .unwrap_or(0);
    let representative_shortcut_last = representative_history
        .iter()
        .rev()
        .find_map(|point| point.shortcut_route_strength)
        .unwrap_or(0);
    let representative_competence_episode = representative_learner.episodes_to_competence;

    let exactly_counterbalanced = audit
        .cue_offsets
        .iter()
        .all(|count| *count == audit.cue_offsets[0])
        && audit.unchanged_answers == audit.changed_answers;
    let passed = observation_shortcut_failures > 0
        && contrasting_shortcut_failures == 0
        && contrasting_forward_routes == 32
        && exactly_counterbalanced
        && held_out_fingerprint_unchanged
        && random_label_stable_arrows == 0
        && random_label_training_correct < 1_500;

    InterventionReport {
        training_budget: INTERVENTION_BUDGET,
        seed_results,
        observation_shortcut_failures,
        contrasting_shortcut_failures,
        contrasting_forward_routes,
        cue_offset_counts: audit.cue_offsets.to_vec(),
        unchanged_answer_episodes: audit.unchanged_answers,
        changed_answer_episodes: audit.changed_answers,
        representative_history,
        representative_competence_episode,
        representative_real_peak,
        representative_shortcut_peak,
        representative_shortcut_last,
        held_out_fingerprint_unchanged,
        random_label_stable_arrows,
        random_label_training_correct,
        passed,
    }
}

pub fn print_intervention_report(report: &InterventionReport) {
    println!("d1 intervention-robust topology discovery:");
    println!(
        "  observation/contrasting shortcut failures: {}/32 / {}/32",
        report.observation_shortcut_failures, report.contrasting_shortcut_failures
    );
    println!(
        "  contrasting forward routes: {}/32, budget per learner={}",
        report.contrasting_forward_routes, report.training_budget
    );
    println!(
        "  cue offsets={}, unchanged/changed answers={}/{}, fingerprint unchanged={}",
        report
            .cue_offset_counts
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join("/"),
        report.unchanged_answer_episodes,
        report.changed_answer_episodes,
        report.held_out_fingerprint_unchanged
    );
    println!(
        "  representative real peak={}, shortcut peak/last={}/{}, competence episode={:?}",
        report.representative_real_peak,
        report.representative_shortcut_peak,
        report.representative_shortcut_last,
        report.representative_competence_episode
    );
    println!(
        "  random labels successes= {}/10000, stable arrows={}",
        report.random_label_training_correct, report.random_label_stable_arrows
    );
}

const NO_ACTION_INDEX: usize = 3;
const ACTION_COST: i32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActionEffect {
    Informative,
    Disruptive,
    Inert,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InformationResult {
    Informative,
    Disruptive,
    Uninformative,
}

#[derive(Clone, Copy, Debug)]
struct RouteSnapshot {
    route: (Role, Role),
    strength_before: i32,
    tested: bool,
}

#[derive(Clone, Debug)]
struct ActionTrace {
    action_index: usize,
    routes: Vec<RouteSnapshot>,
}

impl ActionTrace {
    fn new(action_index: usize, learner: &DiscoveryLearner) -> Self {
        let routes = learner
            .plausible_routes()
            .into_iter()
            .map(|route| RouteSnapshot {
                route,
                strength_before: learner.route_strength(route.0, route.1).unwrap(),
                tested: false,
            })
            .collect();
        Self {
            action_index,
            routes,
        }
    }

    fn observe_route(&mut self, route: Option<(Role, Role)>) {
        let Some(route) = route else {
            return;
        };
        if let Some(snapshot) = self
            .routes
            .iter_mut()
            .find(|snapshot| snapshot.route == route)
        {
            snapshot.tested = true;
        }
    }

    fn complete(&self) -> bool {
        self.routes.iter().all(|snapshot| snapshot.tested)
    }

    fn classify(&self, learner: &DiscoveryLearner) -> InformationResult {
        let changes: Vec<_> = self
            .routes
            .iter()
            .map(|snapshot| {
                learner
                    .route_strength(snapshot.route.0, snapshot.route.1)
                    .unwrap_or(i32::MIN)
                    - snapshot.strength_before
            })
            .collect();
        let weakened = changes.iter().filter(|change| **change < 0).count();
        let supported = changes.iter().filter(|change| **change >= 0).count();
        if weakened > 0 && supported > 0 {
            InformationResult::Informative
        } else if weakened == changes.len() {
            InformationResult::Disruptive
        } else {
            InformationResult::Uninformative
        }
    }
}

#[derive(Clone, Debug)]
struct ActionPolicy {
    values: [i32; 4],
    tried: [bool; 4],
    exploration_cursor: usize,
}

impl ActionPolicy {
    fn new(seed: u64) -> Self {
        Self {
            values: [0; 4],
            tried: [false; 4],
            exploration_cursor: seed as usize % 4,
        }
    }

    fn choose(&mut self, unresolved: bool) -> usize {
        if !unresolved {
            return NO_ACTION_INDEX;
        }
        if let Some((index, _)) = self
            .values
            .iter()
            .take(3)
            .enumerate()
            .filter(|(_, value)| **value > 0)
            .max_by_key(|(_, value)| **value)
        {
            return index;
        }
        for offset in 0..4 {
            let index = (self.exploration_cursor + offset) % 4;
            if !self.tried[index] {
                self.exploration_cursor = (index + 1) % 4;
                return index;
            }
        }
        self.values
            .iter()
            .enumerate()
            .max_by_key(|(index, value)| (**value, usize::from(*index == NO_ACTION_INDEX)))
            .map(|(index, _)| index)
            .unwrap()
    }

    fn learn(&mut self, action_index: usize, result: InformationResult) {
        self.tried[action_index] = true;
        let information_value = match result {
            InformationResult::Informative => 3,
            InformationResult::Disruptive => -2,
            InformationResult::Uninformative => 0,
        };
        let cost = i32::from(action_index != NO_ACTION_INDEX) * ACTION_COST;
        self.values[action_index] += information_value - cost;
    }
}

#[derive(Clone, Copy, Debug)]
enum ActionStrategy {
    Learned,
    Random,
    Fixed,
    NoAction,
}

#[derive(Clone, Debug)]
pub struct ActionDecision {
    pub decision: usize,
    pub action_id: usize,
    pub actual_effect: String,
    pub result: String,
    pub real_before: i32,
    pub real_after: i32,
    pub shortcut_before: i32,
    pub shortcut_after: i32,
    pub policy_value_after: i32,
}

#[derive(Clone, Debug)]
struct ActionRun {
    correct: usize,
    total: usize,
    selected_route: String,
    informative_action_id: usize,
    disruptive_action_id: usize,
    informative_preferred: bool,
    disruptive_preferred: bool,
    decisions: usize,
    paid_actions: usize,
    post_resolution_paid_actions: usize,
    information_results: [usize; 3],
    policy_values: [i32; 4],
    trace: Vec<ActionDecision>,
}

fn shuffled_action_effects(seed: u64) -> [ActionEffect; 3] {
    let mut effects = [
        ActionEffect::Informative,
        ActionEffect::Disruptive,
        ActionEffect::Inert,
    ];
    let mut rng = DeterministicRng::new(seed ^ 0xd200_1000);
    rng.shuffle(&mut effects);
    effects
}

fn action_episode(
    identities: &mut IdentitySource,
    episode_number: usize,
    effect: Option<ActionEffect>,
) -> Episode {
    let target = episode_number * 7 % 10;
    let mut episode = normal_episode(identities, 10, target, Direction::LeftToRight, true);
    let target_index = episode
        .relations
        .iter()
        .position(|relation| relation.left == episode.query)
        .unwrap();
    match effect {
        Some(ActionEffect::Informative) => {
            for relation in &mut episode.relations {
                relation.cued = false;
            }
            let wrong = (target_index + 1) % episode.relations.len();
            episode.relations[wrong].cued = true;
        }
        Some(ActionEffect::Disruptive) => {
            episode.correct = BindingOutcome::Answer(identities.issue());
        }
        Some(ActionEffect::Inert) | None => {}
    }
    episode
}

fn prepare_unresolved_topology(
    learner: &mut DiscoveryLearner,
    identities: &mut IdentitySource,
    episode_number: &mut usize,
) {
    let mut recovery_steps = 0;
    while learner.consolidated.is_none()
        && (!learner
            .plausible_routes()
            .contains(&(Role::Slot1, Role::Slot2))
            || !learner
                .plausible_routes()
                .contains(&(Role::Cue, Role::Slot2)))
    {
        *episode_number += 1;
        recovery_steps += 1;
        let episode = action_episode(identities, *episode_number, None);
        learner.train_episode_deferred(&episode, *episode_number);
        assert!(
            recovery_steps < 5_000,
            "failed to create unresolved topology: real={:?}, cue={:?}, plausible={:?}, candidates={:?}",
            learner.route_strength(Role::Slot1, Role::Slot2),
            learner.route_strength(Role::Cue, Role::Slot2),
            learner.plausible_routes(),
            learner
                .candidates
                .iter()
                .map(|arrow| (arrow.from, arrow.to, arrow.strength, arrow.uses))
                .collect::<Vec<_>>()
        );
    }
}

fn choose_strategy_action(
    strategy: ActionStrategy,
    policy: &mut ActionPolicy,
    unresolved: bool,
    rng: &mut DeterministicRng,
) -> usize {
    match strategy {
        ActionStrategy::Learned => policy.choose(unresolved),
        ActionStrategy::Random if unresolved => (rng.next_u64() as usize) % 4,
        ActionStrategy::Fixed if unresolved => 0,
        ActionStrategy::NoAction | ActionStrategy::Random | ActionStrategy::Fixed => {
            NO_ACTION_INDEX
        }
    }
}

fn run_action_strategy(seed: u64, strategy: ActionStrategy, keep_trace: bool) -> ActionRun {
    let effects = shuffled_action_effects(seed);
    let informative_action_id = effects
        .iter()
        .position(|effect| *effect == ActionEffect::Informative)
        .unwrap();
    let disruptive_action_id = effects
        .iter()
        .position(|effect| *effect == ActionEffect::Disruptive)
        .unwrap();
    let mut learner = DiscoveryLearner::new(seed ^ 0xd210_0000);
    let mut policy = ActionPolicy::new(seed ^ 0xd211_0000);
    let mut rng = DeterministicRng::new(seed ^ 0xd212_0000);
    let mut identities = IdentitySource::new(seed ^ 0xd213_0000);
    let mut episode_number = 0;
    let mut decisions = 0;
    let mut paid_actions = 0;
    let mut information_results = [0; 3];
    let mut trace_log = Vec::new();

    prepare_unresolved_topology(&mut learner, &mut identities, &mut episode_number);
    while learner.consolidated.is_none() && decisions < 16 {
        prepare_unresolved_topology(&mut learner, &mut identities, &mut episode_number);
        if learner.consolidated.is_some() {
            break;
        }
        let plausible = learner.plausible_routes();
        if plausible.len() < 2 {
            continue;
        }
        let action_index = choose_strategy_action(strategy, &mut policy, true, &mut rng);
        let effect = (action_index != NO_ACTION_INDEX).then(|| effects[action_index]);
        decisions += 1;
        paid_actions += usize::from(action_index != NO_ACTION_INDEX);
        let mut trace = ActionTrace::new(action_index, &learner);
        let real_before = learner
            .route_strength(Role::Slot1, Role::Slot2)
            .unwrap_or(0);
        let shortcut_before = learner.route_strength(Role::Cue, Role::Slot2).unwrap_or(0);
        let mut window_steps = 0;
        while !trace.complete() {
            episode_number += 1;
            window_steps += 1;
            let episode = action_episode(&mut identities, episode_number, effect);
            learner.train_episode_deferred(&episode, episode_number);
            trace.observe_route(learner.last_used_route);
            assert!(window_steps < 200, "action evidence window did not close");
        }
        let result = trace.classify(&learner);
        information_results[result as usize] += 1;
        if matches!(strategy, ActionStrategy::Learned) {
            policy.learn(trace.action_index, result);
        }
        let real_after = learner
            .route_strength(Role::Slot1, Role::Slot2)
            .unwrap_or(0);
        let shortcut_after = learner.route_strength(Role::Cue, Role::Slot2).unwrap_or(0);
        if keep_trace {
            trace_log.push(ActionDecision {
                decision: decisions,
                action_id: action_index,
                actual_effect: format!("{:?}", effect.unwrap_or(ActionEffect::Inert)),
                result: format!("{result:?}"),
                real_before,
                real_after,
                shortcut_before,
                shortcut_after,
                policy_value_after: policy.values[action_index],
            });
        }
        learner.consolidate_unique_if_ready(episode_number);
    }

    let mut post_resolution_paid_actions = 0;
    if learner.consolidated.is_some() {
        for _ in 0..100 {
            let action = choose_strategy_action(strategy, &mut policy, false, &mut rng);
            post_resolution_paid_actions += usize::from(action != NO_ACTION_INDEX);
        }
    }
    let (correct, _, _) = held_out_accuracy(
        &mut learner,
        seed ^ 0xd214_0000,
        Direction::LeftToRight,
        500,
        false,
    );
    let informative_preferred = policy.values[informative_action_id]
        > policy
            .values
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != informative_action_id)
            .map(|(_, value)| *value)
            .max()
            .unwrap();
    let disruptive_preferred = policy.values[disruptive_action_id]
        > policy
            .values
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != disruptive_action_id)
            .map(|(_, value)| *value)
            .max()
            .unwrap();

    ActionRun {
        correct,
        total: 500,
        selected_route: route_name(learner.selected_route()),
        informative_action_id,
        disruptive_action_id,
        informative_preferred,
        disruptive_preferred,
        decisions,
        paid_actions,
        post_resolution_paid_actions,
        information_results,
        policy_values: policy.values,
        trace: trace_log,
    }
}

fn random_label_action_control() -> (usize, usize) {
    let mut learner = DiscoveryLearner::new(0xd240_0001);
    let mut policy = ActionPolicy::new(0xd240_0002);
    let mut identities = IdentitySource::new(0xd240_0003);
    for episode_number in 1..=10_000 {
        let episode = random_label_episode(
            &mut identities,
            10,
            episode_number * 7 % 10,
            episode_number * 3 + 1,
        );
        learner.train_episode(&episode, episode_number);
        if learner.plausible_routes().len() > 1 {
            let action = policy.choose(true);
            policy.learn(action, InformationResult::Uninformative);
        }
    }
    (
        policy.values.iter().filter(|value| **value > 0).count(),
        learner.stable_arrows(),
    )
}

#[derive(Clone, Debug)]
pub struct ActiveDiscoveryReport {
    pub learned_correct_runs: usize,
    pub learned_forward_routes: usize,
    pub informative_preferred_runs: usize,
    pub disruptive_preferred_runs: usize,
    pub post_resolution_paid_actions: usize,
    pub random_correct_runs: usize,
    pub fixed_correct_runs: usize,
    pub no_action_correct_runs: usize,
    pub average_learned_decisions: f64,
    pub average_learned_paid_actions: f64,
    pub average_random_decisions: f64,
    pub average_random_paid_actions: f64,
    pub learned_information_results: Vec<usize>,
    pub action_permutations_seen: usize,
    pub random_label_positive_actions: usize,
    pub random_label_stable_arrows: usize,
    pub representative_informative_action_id: usize,
    pub representative_disruptive_action_id: usize,
    pub representative_policy_values: Vec<i32>,
    pub representative_trace: Vec<ActionDecision>,
    pub passed: bool,
}

pub fn run_active_discovery_experiment() -> ActiveDiscoveryReport {
    let mut learned_correct_runs = 0;
    let mut learned_forward_routes = 0;
    let mut informative_preferred_runs = 0;
    let mut disruptive_preferred_runs = 0;
    let mut post_resolution_paid_actions = 0;
    let mut random_correct_runs = 0;
    let mut fixed_correct_runs = 0;
    let mut no_action_correct_runs = 0;
    let mut total_learned_decisions = 0;
    let mut total_learned_paid_actions = 0;
    let mut total_random_decisions = 0;
    let mut total_random_paid_actions = 0;
    let mut learned_information_results = [0; 3];
    let mut permutations = HashSet::new();
    let mut representative = None;

    for seed in 0..32 {
        let seed = 0xd220_0000 + seed;
        let learned = run_action_strategy(seed, ActionStrategy::Learned, seed == 0xd220_0000);
        let random = run_action_strategy(seed, ActionStrategy::Random, false);
        let fixed = run_action_strategy(seed, ActionStrategy::Fixed, false);
        let no_action = run_action_strategy(seed, ActionStrategy::NoAction, false);
        learned_correct_runs += usize::from(learned.correct == learned.total);
        learned_forward_routes += usize::from(learned.selected_route == "Slot1->Slot2");
        informative_preferred_runs += usize::from(learned.informative_preferred);
        disruptive_preferred_runs += usize::from(learned.disruptive_preferred);
        post_resolution_paid_actions += learned.post_resolution_paid_actions;
        random_correct_runs += usize::from(random.correct == random.total);
        fixed_correct_runs += usize::from(fixed.correct == fixed.total);
        no_action_correct_runs += usize::from(no_action.correct == no_action.total);
        total_learned_decisions += learned.decisions;
        total_learned_paid_actions += learned.paid_actions;
        total_random_decisions += random.decisions;
        total_random_paid_actions += random.paid_actions;
        for (total, count) in learned_information_results
            .iter_mut()
            .zip(learned.information_results)
        {
            *total += count;
        }
        permutations.insert((learned.informative_action_id, learned.disruptive_action_id));
        if representative.is_none() {
            representative = Some(learned);
        }
    }
    let representative = representative.unwrap();
    let (random_label_positive_actions, random_label_stable_arrows) = random_label_action_control();
    let passed = learned_correct_runs == 32
        && informative_preferred_runs == 32
        && disruptive_preferred_runs == 0
        && post_resolution_paid_actions == 0
        && learned_forward_routes == 32
        && fixed_correct_runs < learned_correct_runs
        && no_action_correct_runs < learned_correct_runs
        && permutations.len() >= 6
        && random_label_positive_actions == 0
        && random_label_stable_arrows == 0;

    ActiveDiscoveryReport {
        learned_correct_runs,
        learned_forward_routes,
        informative_preferred_runs,
        disruptive_preferred_runs,
        post_resolution_paid_actions,
        random_correct_runs,
        fixed_correct_runs,
        no_action_correct_runs,
        average_learned_decisions: total_learned_decisions as f64 / 32.0,
        average_learned_paid_actions: total_learned_paid_actions as f64 / 32.0,
        average_random_decisions: total_random_decisions as f64 / 32.0,
        average_random_paid_actions: total_random_paid_actions as f64 / 32.0,
        learned_information_results: learned_information_results.to_vec(),
        action_permutations_seen: permutations.len(),
        random_label_positive_actions,
        random_label_stable_arrows,
        representative_informative_action_id: representative.informative_action_id,
        representative_disruptive_action_id: representative.disruptive_action_id,
        representative_policy_values: representative.policy_values.to_vec(),
        representative_trace: representative.trace,
        passed,
    }
}

pub fn print_active_discovery_report(report: &ActiveDiscoveryReport) {
    println!("d2 learned epistemic action:");
    println!(
        "  learned/informative-preferred/disruptive-preferred: {}/{}/{} of 32",
        report.learned_correct_runs,
        report.informative_preferred_runs,
        report.disruptive_preferred_runs
    );
    println!(
        "  random/fixed/no-action correct runs: {}/{}/{} of 32",
        report.random_correct_runs, report.fixed_correct_runs, report.no_action_correct_runs
    );
    println!(
        "  learned/random decisions={:.1}/{:.1}, paid actions={:.1}/{:.1}, post-resolution paid actions={}",
        report.average_learned_decisions,
        report.average_random_decisions,
        report.average_learned_paid_actions,
        report.average_random_paid_actions,
        report.post_resolution_paid_actions
    );
    println!(
        "  learned informative/disruptive/uninformative windows={}/{}/{}",
        report.learned_information_results[0],
        report.learned_information_results[1],
        report.learned_information_results[2]
    );
    println!(
        "  action permutations={}, random-label positive actions/stable arrows={}/{}",
        report.action_permutations_seen,
        report.random_label_positive_actions,
        report.random_label_stable_arrows
    );
    println!(
        "  representative informative/disruptive ids={}/{}, policy values={:?}",
        report.representative_informative_action_id,
        report.representative_disruptive_action_id,
        report.representative_policy_values
    );
    for decision in &report.representative_trace {
        println!(
            "    decision {} action {} effect={} result={} real {}->{}, shortcut {}->{}, action value={}",
            decision.decision,
            decision.action_id,
            decision.actual_effect,
            decision.result,
            decision.real_before,
            decision.real_after,
            decision.shortcut_before,
            decision.shortcut_after,
            decision.policy_value_after
        );
    }
}

const AMORTIZATION_PROBLEMS: usize = 100;
const AMORTIZATION_SEEDS: usize = 8;
const AMORTIZATION_STRENGTH: i32 = CONSOLIDATION_STRENGTH - 1;

#[derive(Clone, Debug)]
struct ScalableActionPolicy {
    values: Vec<i32>,
    tried: Vec<bool>,
    exploration_cursor: usize,
}

impl ScalableActionPolicy {
    fn new(choice_count: usize, seed: u64) -> Self {
        Self {
            values: vec![0; choice_count],
            tried: vec![false; choice_count],
            exploration_cursor: seed as usize % choice_count,
        }
    }

    fn choose(&mut self, unresolved: bool) -> usize {
        let no_action = self.values.len() - 1;
        if !unresolved {
            return no_action;
        }
        if let Some((index, _)) = self
            .values
            .iter()
            .take(no_action)
            .enumerate()
            .filter(|(_, value)| **value > 0)
            .max_by_key(|(_, value)| **value)
        {
            return index;
        }
        for offset in 0..self.values.len() {
            let index = (self.exploration_cursor + offset) % self.values.len();
            if !self.tried[index] {
                self.exploration_cursor = (index + 1) % self.values.len();
                return index;
            }
        }
        self.values
            .iter()
            .enumerate()
            .max_by_key(|(index, value)| (**value, usize::from(*index == no_action)))
            .map(|(index, _)| index)
            .unwrap()
    }

    fn learn(&mut self, action_index: usize, result: InformationResult) {
        self.tried[action_index] = true;
        let information_value = match result {
            InformationResult::Informative => 3,
            InformationResult::Disruptive => -2,
            InformationResult::Uninformative => 0,
        };
        let no_action = self.values.len() - 1;
        let cost = i32::from(action_index != no_action) * ACTION_COST;
        self.values[action_index] += information_value - cost;
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for value in &self.values {
            fingerprint_mix(&mut hash, *value as i64 as u64);
        }
        for tried in &self.tried {
            fingerprint_mix(&mut hash, u64::from(*tried));
        }
        fingerprint_mix(&mut hash, self.exploration_cursor as u64);
        hash
    }

    fn retained_problem_identities(&self) -> usize {
        0
    }

    fn preferred_paid_action(&self) -> Option<usize> {
        let no_action = self.values.len() - 1;
        self.values
            .iter()
            .take(no_action)
            .enumerate()
            .filter(|(_, value)| **value > 0)
            .max_by_key(|(_, value)| **value)
            .map(|(index, _)| index)
    }
}

#[derive(Clone, Copy, Debug)]
enum AmortizationStrategy {
    Learned,
    Random,
    Oracle,
}

#[derive(Clone, Debug)]
struct FreshWorkspace {
    learner: DiscoveryLearner,
    identities: IdentitySource,
    episode_number: usize,
    live_counter: Rc<Cell<usize>>,
}

impl FreshWorkspace {
    fn new(seed: u64, live_counter: Rc<Cell<usize>>) -> Self {
        assert_eq!(
            live_counter.get(),
            0,
            "previous topology workspace is still live"
        );
        live_counter.set(1);
        Self {
            learner: DiscoveryLearner::new(seed ^ 0xd310_0000),
            identities: IdentitySource::new(seed ^ 0xd311_0000),
            episode_number: 0,
            live_counter,
        }
    }
}

impl Drop for FreshWorkspace {
    fn drop(&mut self) {
        self.live_counter.set(self.live_counter.get() - 1);
    }
}

#[derive(Clone, Debug)]
struct AmortizedProblemResult {
    paid_actions: usize,
    action_decisions: usize,
    correct: bool,
    spikes: usize,
    episodes: usize,
    workspace_fingerprint: u64,
    workspace_arrows_before_drop: usize,
    live_workspaces_after_drop: usize,
    policy_fingerprint_before: u64,
    policy_fingerprint_after: u64,
}

fn scalable_action_effects(choice_count: usize, seed: u64) -> Vec<ActionEffect> {
    assert!(choice_count >= 4);
    let mut effects = vec![ActionEffect::Inert; choice_count - 1];
    effects[0] = ActionEffect::Informative;
    effects[1] = ActionEffect::Disruptive;
    let mut rng = DeterministicRng::new(seed ^ 0xd320_0000);
    rng.shuffle(&mut effects);
    effects
}

fn prepare_workspace_competitors(workspace: &mut FreshWorkspace) {
    let mut preparation_steps = 0;
    loop {
        let real = workspace
            .learner
            .route_strength(Role::Slot1, Role::Slot2)
            .unwrap_or(i32::MIN);
        let shortcut = workspace
            .learner
            .route_strength(Role::Cue, Role::Slot2)
            .unwrap_or(i32::MIN);
        if real >= AMORTIZATION_STRENGTH && shortcut >= AMORTIZATION_STRENGTH {
            return;
        }
        workspace.episode_number += 1;
        preparation_steps += 1;
        let episode = action_episode(&mut workspace.identities, workspace.episode_number, None);
        workspace
            .learner
            .train_episode_deferred(&episode, workspace.episode_number);
        assert!(
            preparation_steps < 10_000,
            "failed to prepare equally plausible routes"
        );
    }
}

fn solve_amortized_problem(
    policy: &mut ScalableActionPolicy,
    effects: &[ActionEffect],
    strategy: AmortizationStrategy,
    seed: u64,
    live_counter: Rc<Cell<usize>>,
) -> AmortizedProblemResult {
    let policy_fingerprint_before = policy.fingerprint();
    let no_action = effects.len();
    let informative_action = effects
        .iter()
        .position(|effect| *effect == ActionEffect::Informative)
        .unwrap();
    let mut random_choices: Vec<_> = (0..=no_action).collect();
    let mut rng = DeterministicRng::new(seed ^ 0xd330_0000);
    rng.shuffle(&mut random_choices);
    let mut random_cursor = 0;
    let mut workspace = FreshWorkspace::new(seed, live_counter.clone());
    prepare_workspace_competitors(&mut workspace);
    let mut paid_actions = 0;
    let mut action_decisions = 0;

    while workspace.learner.consolidated.is_none() && action_decisions < effects.len() + 1 {
        prepare_workspace_competitors(&mut workspace);
        let action_index = match strategy {
            AmortizationStrategy::Learned => policy.choose(true),
            AmortizationStrategy::Random => {
                let action = random_choices[random_cursor];
                random_cursor += 1;
                action
            }
            AmortizationStrategy::Oracle => informative_action,
        };
        action_decisions += 1;
        paid_actions += usize::from(action_index != no_action);
        let effect = (action_index != no_action).then(|| effects[action_index]);
        let mut trace = ActionTrace::new(action_index, &workspace.learner);
        let mut evidence_steps = 0;
        while !trace.complete() {
            workspace.episode_number += 1;
            evidence_steps += 1;
            let episode =
                action_episode(&mut workspace.identities, workspace.episode_number, effect);
            workspace
                .learner
                .train_episode_deferred(&episode, workspace.episode_number);
            trace.observe_route(workspace.learner.last_used_route);
            assert!(
                evidence_steps < 500,
                "amortization evidence window did not close"
            );
        }
        let result = trace.classify(&workspace.learner);
        if matches!(strategy, AmortizationStrategy::Learned) {
            policy.learn(action_index, result);
        }
        workspace
            .learner
            .consolidate_unique_if_ready(workspace.episode_number);
    }

    let (correct, _, _) = held_out_accuracy(
        &mut workspace.learner,
        seed ^ 0xd340_0000,
        Direction::LeftToRight,
        20,
        false,
    );
    let workspace_fingerprint = workspace.learner.permanent_fingerprint();
    let workspace_arrows_before_drop =
        workspace.learner.live_candidates() + workspace.learner.stable_arrows();
    let spikes = workspace.learner.training_spikes;
    let episodes = workspace.episode_number;
    let result = AmortizedProblemResult {
        paid_actions,
        action_decisions,
        correct: correct == 20,
        spikes,
        episodes,
        workspace_fingerprint,
        workspace_arrows_before_drop,
        live_workspaces_after_drop: usize::MAX,
        policy_fingerprint_before,
        policy_fingerprint_after: policy.fingerprint(),
    };
    drop(workspace);
    AmortizedProblemResult {
        live_workspaces_after_drop: live_counter.get(),
        ..result
    }
}

#[derive(Clone, Debug)]
pub struct AmortizationPoint {
    pub choice_count: usize,
    pub first_problem_learned_actions: f64,
    pub mature_learned_actions: f64,
    pub mature_random_actions: f64,
    pub mature_oracle_actions: f64,
    pub break_even_problem: Option<usize>,
    pub learned_correct: usize,
    pub random_correct: usize,
    pub oracle_correct: usize,
    pub total_problems: usize,
    pub average_learned_spikes: f64,
    pub average_random_spikes: f64,
    pub average_learned_episodes: f64,
    pub average_random_episodes: f64,
}

#[derive(Clone, Debug)]
pub struct AmortizationReport {
    pub points: Vec<AmortizationPoint>,
    pub workspaces_created: usize,
    pub workspaces_destroyed: usize,
    pub maximum_live_workspaces_after_problem: usize,
    pub policy_retained_problem_identities: usize,
    pub workspace_fingerprints_observed: usize,
    pub policy_only_state_crosses_problems: bool,
    pub passed: bool,
}

pub fn run_amortization_experiment() -> AmortizationReport {
    let choice_counts = [4, 8, 16, 32, 64];
    let mut points = Vec::new();
    let mut workspaces_created = 0;
    let mut workspaces_destroyed = 0;
    let mut maximum_live_workspaces_after_problem = 0;
    let mut workspace_fingerprints = HashSet::new();
    let mut policy_retained_problem_identities = 0;

    for choice_count in choice_counts {
        let mut learned_by_problem = vec![0usize; AMORTIZATION_PROBLEMS];
        let mut random_by_problem = vec![0usize; AMORTIZATION_PROBLEMS];
        let mut oracle_by_problem = vec![0usize; AMORTIZATION_PROBLEMS];
        let mut learned_correct = 0;
        let mut random_correct = 0;
        let mut oracle_correct = 0;
        let mut learned_spikes = 0;
        let mut random_spikes = 0;
        let mut learned_episodes = 0;
        let mut random_episodes = 0;

        for seed_index in 0..AMORTIZATION_SEEDS {
            let seed = 0xd350_0000 + choice_count as u64 * 1_000 + seed_index as u64;
            let effects = scalable_action_effects(choice_count, seed);
            let mut learned_policy = ScalableActionPolicy::new(choice_count, seed ^ 0xd351_0000);
            let mut random_policy = ScalableActionPolicy::new(choice_count, seed ^ 0xd352_0000);
            let mut oracle_policy = ScalableActionPolicy::new(choice_count, seed ^ 0xd353_0000);
            let live_counter = Rc::new(Cell::new(0));

            for problem_index in 0..AMORTIZATION_PROBLEMS {
                let problem_seed = seed + problem_index as u64 * 10_000;
                let learned = solve_amortized_problem(
                    &mut learned_policy,
                    &effects,
                    AmortizationStrategy::Learned,
                    problem_seed,
                    live_counter.clone(),
                );
                let random = solve_amortized_problem(
                    &mut random_policy,
                    &effects,
                    AmortizationStrategy::Random,
                    problem_seed ^ 0xd354_0000,
                    live_counter.clone(),
                );
                let oracle = solve_amortized_problem(
                    &mut oracle_policy,
                    &effects,
                    AmortizationStrategy::Oracle,
                    problem_seed ^ 0xd355_0000,
                    live_counter.clone(),
                );
                workspaces_created += 3;
                workspaces_destroyed += usize::from(learned.live_workspaces_after_drop == 0)
                    + usize::from(random.live_workspaces_after_drop == 0)
                    + usize::from(oracle.live_workspaces_after_drop == 0);
                maximum_live_workspaces_after_problem = maximum_live_workspaces_after_problem.max(
                    learned
                        .live_workspaces_after_drop
                        .max(random.live_workspaces_after_drop)
                        .max(oracle.live_workspaces_after_drop),
                );
                workspace_fingerprints.insert(learned.workspace_fingerprint);
                workspace_fingerprints.insert(random.workspace_fingerprint);
                workspace_fingerprints.insert(oracle.workspace_fingerprint);
                debug_assert!(learned.workspace_arrows_before_drop > 0);
                debug_assert!(random.workspace_arrows_before_drop > 0);
                debug_assert!(oracle.workspace_arrows_before_drop > 0);
                debug_assert!(
                    learned.policy_fingerprint_before != learned.policy_fingerprint_after
                        || problem_index > 0
                );
                learned_by_problem[problem_index] += learned.paid_actions;
                random_by_problem[problem_index] += random.paid_actions;
                oracle_by_problem[problem_index] += oracle.paid_actions;
                learned_correct += usize::from(learned.correct);
                random_correct += usize::from(random.correct);
                oracle_correct += usize::from(oracle.correct);
                learned_spikes += learned.spikes;
                random_spikes += random.spikes;
                learned_episodes += learned.episodes;
                random_episodes += random.episodes;
                debug_assert!(learned.episodes > 0);
                debug_assert!(random.episodes > 0);
                debug_assert!(oracle.action_decisions > 0);
            }
            policy_retained_problem_identities += learned_policy.retained_problem_identities();
        }

        let seed_count = AMORTIZATION_SEEDS as f64;
        let first_problem_learned_actions = learned_by_problem[0] as f64 / seed_count;
        let mature_start = AMORTIZATION_PROBLEMS - 20;
        let mature_divisor = (20 * AMORTIZATION_SEEDS) as f64;
        let mature_learned_actions =
            learned_by_problem[mature_start..].iter().sum::<usize>() as f64 / mature_divisor;
        let mature_random_actions =
            random_by_problem[mature_start..].iter().sum::<usize>() as f64 / mature_divisor;
        let mature_oracle_actions =
            oracle_by_problem[mature_start..].iter().sum::<usize>() as f64 / mature_divisor;
        let mut learned_cumulative = 0;
        let mut random_cumulative = 0;
        let mut break_even_problem = None;
        for problem_index in 0..AMORTIZATION_PROBLEMS {
            learned_cumulative += learned_by_problem[problem_index];
            random_cumulative += random_by_problem[problem_index];
            if learned_cumulative < random_cumulative {
                break_even_problem = Some(problem_index + 1);
                break;
            }
        }
        let total_problems = AMORTIZATION_PROBLEMS * AMORTIZATION_SEEDS;
        points.push(AmortizationPoint {
            choice_count,
            first_problem_learned_actions,
            mature_learned_actions,
            mature_random_actions,
            mature_oracle_actions,
            break_even_problem,
            learned_correct,
            random_correct,
            oracle_correct,
            total_problems,
            average_learned_spikes: learned_spikes as f64 / total_problems as f64,
            average_random_spikes: random_spikes as f64 / total_problems as f64,
            average_learned_episodes: learned_episodes as f64 / total_problems as f64,
            average_random_episodes: random_episodes as f64 / total_problems as f64,
        });
    }

    let accuracy_pass = points.iter().all(|point| {
        point.learned_correct == point.total_problems
            && point.random_correct == point.total_problems
            && point.oracle_correct == point.total_problems
    });
    let mature_policy_pass = points.iter().all(|point| {
        (point.mature_learned_actions - 1.0).abs() < 0.01
            && (point.mature_oracle_actions - 1.0).abs() < 0.01
    });
    let random_scaling_pass = points
        .windows(2)
        .all(|pair| pair[1].mature_random_actions > pair[0].mature_random_actions);
    let amortization_pass = points.iter().skip(1).all(|point| {
        point.mature_learned_actions < point.mature_random_actions
            && point.break_even_problem.is_some()
    });
    let policy_only_state_crosses_problems = workspaces_created == workspaces_destroyed
        && maximum_live_workspaces_after_problem == 0
        && policy_retained_problem_identities == 0;
    let passed = accuracy_pass
        && mature_policy_pass
        && random_scaling_pass
        && amortization_pass
        && policy_only_state_crosses_problems;

    AmortizationReport {
        points,
        workspaces_created,
        workspaces_destroyed,
        maximum_live_workspaces_after_problem,
        policy_retained_problem_identities,
        workspace_fingerprints_observed: workspace_fingerprints.len(),
        policy_only_state_crosses_problems,
        passed,
    }
}

pub fn print_amortization_report(report: &AmortizationReport) {
    println!("d2.1 epistemic-action amortization:");
    for point in &report.points {
        println!(
            "  choices {:>2}: first learned={:.1}, mature learned/random/oracle={:.1}/{:.1}/{:.1}, break-even={:?}, spikes={:.0}/{:.0}, episodes={:.0}/{:.0}, accuracy={}/{}/{}/{}",
            point.choice_count,
            point.first_problem_learned_actions,
            point.mature_learned_actions,
            point.mature_random_actions,
            point.mature_oracle_actions,
            point.break_even_problem,
            point.average_learned_spikes,
            point.average_random_spikes,
            point.average_learned_episodes,
            point.average_random_episodes,
            point.learned_correct,
            point.random_correct,
            point.oracle_correct,
            point.total_problems
        );
    }
    println!(
        "  workspaces created/destroyed={}/{}, max live after problem={}, policy identities={}, workspace fingerprints={}",
        report.workspaces_created,
        report.workspaces_destroyed,
        report.maximum_live_workspaces_after_problem,
        report.policy_retained_problem_identities,
        report.workspace_fingerprints_observed
    );
}

const REMAP_MAX_PROBLEMS: usize = 500;
const REMAP_SUCCESS_STREAK: usize = 5;
const REMAP_SEEDS: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RemapCase {
    Untried,
    Rejected,
}

impl RemapCase {
    fn name(self) -> &'static str {
        match self {
            Self::Untried => "untried",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Debug)]
struct RemapProblemResult {
    paid_actions: usize,
    correct: bool,
    selected_actions: Vec<usize>,
    live_workspaces_after_drop: usize,
}

fn remap_initial_effects(
    choice_count: usize,
    policy: &ScalableActionPolicy,
) -> (Vec<ActionEffect>, usize, usize, usize) {
    let no_action = choice_count - 1;
    let mut paid_order = Vec::with_capacity(choice_count - 1);
    for offset in 0..choice_count {
        let action = (policy.exploration_cursor + offset) % choice_count;
        if action != no_action {
            paid_order.push(action);
        }
    }
    let rejected_candidate = paid_order[0];
    let informative_action = paid_order[2];
    let untried_candidate = *paid_order.last().unwrap();
    let mut effects = vec![ActionEffect::Inert; choice_count - 1];
    effects[informative_action] = ActionEffect::Informative;
    let disruptive_action = paid_order[paid_order.len() / 2];
    if disruptive_action != informative_action && disruptive_action != untried_candidate {
        effects[disruptive_action] = ActionEffect::Disruptive;
    }
    (
        effects,
        informative_action,
        rejected_candidate,
        untried_candidate,
    )
}

fn remapped_effects(
    original: &[ActionEffect],
    old_informative: usize,
    new_informative: usize,
) -> Vec<ActionEffect> {
    let mut effects = original.to_vec();
    effects[old_informative] = ActionEffect::Inert;
    effects[new_informative] = ActionEffect::Informative;
    effects
}

fn solve_remap_problem(
    policy: &mut ScalableActionPolicy,
    effects: &[ActionEffect],
    seed: u64,
    live_counter: Rc<Cell<usize>>,
) -> RemapProblemResult {
    let no_action = effects.len();
    let mut workspace = FreshWorkspace::new(seed, live_counter.clone());
    prepare_workspace_competitors(&mut workspace);
    let mut paid_actions = 0;
    let mut selected_actions = Vec::new();

    while workspace.learner.consolidated.is_none() && selected_actions.len() < effects.len() + 1 {
        prepare_workspace_competitors(&mut workspace);
        let action_index = policy.choose(true);
        selected_actions.push(action_index);
        paid_actions += usize::from(action_index != no_action);
        let effect = (action_index != no_action).then(|| effects[action_index]);
        let mut trace = ActionTrace::new(action_index, &workspace.learner);
        let mut evidence_steps = 0;
        while !trace.complete() {
            workspace.episode_number += 1;
            evidence_steps += 1;
            let episode =
                action_episode(&mut workspace.identities, workspace.episode_number, effect);
            workspace
                .learner
                .train_episode_deferred(&episode, workspace.episode_number);
            trace.observe_route(workspace.learner.last_used_route);
            assert!(evidence_steps < 500, "remap evidence window did not close");
        }
        let information = trace.classify(&workspace.learner);
        let value_before = policy.values[action_index];
        policy.learn(action_index, information);
        workspace
            .learner
            .consolidate_unique_if_ready(workspace.episode_number);

        // Once no-action leaves both topology and policy unchanged, repeating
        // it cannot resolve this fresh problem.
        if action_index == no_action
            && information == InformationResult::Uninformative
            && policy.values[action_index] == value_before
        {
            break;
        }
    }

    let (correct, _, _) = held_out_accuracy(
        &mut workspace.learner,
        seed ^ 0xd440_0000,
        Direction::LeftToRight,
        20,
        false,
    );
    let result = RemapProblemResult {
        paid_actions,
        correct: workspace.learner.consolidated.is_some() && correct == 20,
        selected_actions,
        live_workspaces_after_drop: usize::MAX,
    };
    drop(workspace);
    RemapProblemResult {
        live_workspaces_after_drop: live_counter.get(),
        ..result
    }
}

#[derive(Clone, Debug)]
pub struct RemapTrajectoryPoint {
    pub choice_count: usize,
    pub maturity: usize,
    pub remap_case: String,
    pub seed_index: usize,
    pub policy_kind: String,
    pub problem: usize,
    pub old_action: usize,
    pub new_action: usize,
    pub old_action_value: i32,
    pub new_action_value: i32,
    pub preferred_action: Option<usize>,
    pub selected_actions: Vec<usize>,
    pub paid_actions: usize,
    pub correct: bool,
    pub cumulative_cost: usize,
}

#[derive(Clone, Debug)]
pub struct RemapPoint {
    pub choice_count: usize,
    pub maturity: usize,
    pub remap_case: String,
    pub mature_adapted_runs: usize,
    pub fresh_adapted_runs: usize,
    pub total_runs: usize,
    pub mature_average_problems: Option<f64>,
    pub fresh_average_problems: Option<f64>,
    pub mature_average_cost: Option<f64>,
    pub fresh_average_cost: Option<f64>,
    pub mature_average_observed_cost: f64,
    pub fresh_average_observed_cost: f64,
    pub mature_average_old_collapse_problem: Option<f64>,
    pub mature_average_exploration_resume_problem: Option<f64>,
    pub mature_average_new_preferred_problem: Option<f64>,
    pub mature_old_value_before: f64,
    pub mature_new_value_before: f64,
}

#[derive(Clone, Debug)]
pub struct RemapReport {
    pub points: Vec<RemapPoint>,
    pub trajectories: Vec<RemapTrajectoryPoint>,
    pub workspaces_created: usize,
    pub workspaces_destroyed: usize,
    pub maximum_live_workspaces_after_problem: usize,
    pub mature_slower_than_scratch_cases: usize,
    pub mature_failed_cases: usize,
    pub passed: bool,
}

#[derive(Clone, Debug)]
struct AdaptationRun {
    adapted_at: Option<usize>,
    cumulative_cost: usize,
    old_collapsed_at: Option<usize>,
    exploration_resumed_at: Option<usize>,
    new_preferred_at: Option<usize>,
}

struct AdaptationSpec<'a> {
    choice_count: usize,
    maturity: usize,
    remap_case: RemapCase,
    seed_index: usize,
    policy_kind: &'a str,
    old_action: usize,
    new_action: usize,
}

fn run_adaptation(
    policy: &mut ScalableActionPolicy,
    effects: &[ActionEffect],
    spec: &AdaptationSpec<'_>,
    seed: u64,
    live_counter: Rc<Cell<usize>>,
    trajectories: &mut Vec<RemapTrajectoryPoint>,
    workspace_audit: &mut (usize, usize, usize),
) -> AdaptationRun {
    let mut cumulative_cost = 0;
    let mut success_streak = 0;
    let mut adapted_at = None;
    let mut old_collapsed_at = None;
    let mut exploration_resumed_at = None;
    let mut new_preferred_at = None;

    for problem in 1..=REMAP_MAX_PROBLEMS {
        let problem_seed = seed + problem as u64 * 100_000;
        let result = solve_remap_problem(policy, effects, problem_seed, live_counter.clone());
        workspace_audit.0 += 1;
        workspace_audit.1 += usize::from(result.live_workspaces_after_drop == 0);
        workspace_audit.2 = workspace_audit.2.max(result.live_workspaces_after_drop);
        cumulative_cost += result.paid_actions;
        let new_is_preferred = policy.preferred_paid_action() == Some(spec.new_action);
        if old_collapsed_at.is_none() && policy.preferred_paid_action() != Some(spec.old_action) {
            old_collapsed_at = Some(problem);
        }
        if exploration_resumed_at.is_none()
            && result
                .selected_actions
                .iter()
                .any(|action| *action != spec.old_action && *action < effects.len())
        {
            exploration_resumed_at = Some(problem);
        }
        if new_preferred_at.is_none() && new_is_preferred {
            new_preferred_at = Some(problem);
        }
        let clean_success = result.correct
            && result.paid_actions == 1
            && result.selected_actions == [spec.new_action]
            && new_is_preferred;
        success_streak = if clean_success { success_streak + 1 } else { 0 };
        trajectories.push(RemapTrajectoryPoint {
            choice_count: spec.choice_count,
            maturity: spec.maturity,
            remap_case: spec.remap_case.name().to_string(),
            seed_index: spec.seed_index,
            policy_kind: spec.policy_kind.to_string(),
            problem,
            old_action: spec.old_action,
            new_action: spec.new_action,
            old_action_value: policy.values[spec.old_action],
            new_action_value: policy.values[spec.new_action],
            preferred_action: policy.preferred_paid_action(),
            selected_actions: result.selected_actions,
            paid_actions: result.paid_actions,
            correct: result.correct,
            cumulative_cost,
        });
        if success_streak == REMAP_SUCCESS_STREAK {
            adapted_at = Some(problem);
            break;
        }
    }

    AdaptationRun {
        adapted_at,
        cumulative_cost,
        old_collapsed_at,
        exploration_resumed_at,
        new_preferred_at,
    }
}

fn average_recorded_problem(values: &[Option<usize>]) -> Option<f64> {
    let recorded: Vec<_> = values.iter().flatten().copied().collect();
    (!recorded.is_empty()).then_some(recorded.iter().sum::<usize>() as f64 / recorded.len() as f64)
}

pub fn run_remap_experiment() -> RemapReport {
    let choice_counts = [16, 64];
    let maturities = [10, 50, 100];
    let remap_cases = [RemapCase::Untried, RemapCase::Rejected];
    let mut points = Vec::new();
    let mut trajectories = Vec::new();
    let live_counter = Rc::new(Cell::new(0));
    let mut workspace_audit = (0usize, 0usize, 0usize);
    let mut mature_slower_than_scratch_cases = 0;
    let mut mature_failed_cases = 0;

    for choice_count in choice_counts {
        for maturity in maturities {
            for remap_case in remap_cases {
                let mut mature_adapted = 0;
                let mut fresh_adapted = 0;
                let mut mature_problems = 0;
                let mut fresh_problems = 0;
                let mut mature_cost = 0;
                let mut fresh_cost = 0;
                let mut mature_observed_cost = 0;
                let mut fresh_observed_cost = 0;
                let mut mature_old_collapse = Vec::new();
                let mut mature_exploration_resume = Vec::new();
                let mut mature_new_preferred = Vec::new();
                let mut mature_old_before = 0;
                let mut mature_new_before = 0;

                for seed_index in 0..REMAP_SEEDS {
                    let seed = 0xd410_0000
                        + choice_count as u64 * 100_000
                        + maturity as u64 * 1_000
                        + seed_index as u64;
                    let mut mature_policy =
                        ScalableActionPolicy::new(choice_count, seed ^ 0xd411_0000);
                    let (original_effects, old_action, rejected_action, untried_action) =
                        remap_initial_effects(choice_count, &mature_policy);
                    for problem in 0..maturity {
                        let result = solve_amortized_problem(
                            &mut mature_policy,
                            &original_effects,
                            AmortizationStrategy::Learned,
                            seed + problem as u64 * 10_000,
                            live_counter.clone(),
                        );
                        workspace_audit.0 += 1;
                        workspace_audit.1 += usize::from(result.live_workspaces_after_drop == 0);
                        workspace_audit.2 =
                            workspace_audit.2.max(result.live_workspaces_after_drop);
                        assert!(result.correct, "maturity problem did not resolve");
                    }
                    let new_action = match remap_case {
                        RemapCase::Untried => untried_action,
                        RemapCase::Rejected => rejected_action,
                    };
                    assert!(mature_policy.values[old_action] > 0);
                    match remap_case {
                        RemapCase::Untried => assert!(!mature_policy.tried[new_action]),
                        RemapCase::Rejected => {
                            assert!(mature_policy.tried[new_action]);
                            assert!(mature_policy.values[new_action] < 0);
                        }
                    }
                    let effects = remapped_effects(&original_effects, old_action, new_action);
                    mature_old_before += mature_policy.values[old_action];
                    mature_new_before += mature_policy.values[new_action];
                    let mut fresh_policy =
                        ScalableActionPolicy::new(choice_count, seed ^ 0xd411_0000);
                    let mature_spec = AdaptationSpec {
                        choice_count,
                        maturity,
                        remap_case,
                        seed_index,
                        policy_kind: "mature",
                        old_action,
                        new_action,
                    };
                    let fresh_spec = AdaptationSpec {
                        policy_kind: "fresh",
                        ..mature_spec
                    };
                    let mature_run = run_adaptation(
                        &mut mature_policy,
                        &effects,
                        &mature_spec,
                        seed ^ 0xd412_0000,
                        live_counter.clone(),
                        &mut trajectories,
                        &mut workspace_audit,
                    );
                    let fresh_run = run_adaptation(
                        &mut fresh_policy,
                        &effects,
                        &fresh_spec,
                        seed ^ 0xd412_0000,
                        live_counter.clone(),
                        &mut trajectories,
                        &mut workspace_audit,
                    );
                    mature_observed_cost += mature_run.cumulative_cost;
                    fresh_observed_cost += fresh_run.cumulative_cost;
                    mature_old_collapse.push(mature_run.old_collapsed_at);
                    mature_exploration_resume.push(mature_run.exploration_resumed_at);
                    mature_new_preferred.push(mature_run.new_preferred_at);
                    if let Some(problem) = mature_run.adapted_at {
                        mature_adapted += 1;
                        mature_problems += problem;
                        mature_cost += mature_run.cumulative_cost;
                    } else {
                        mature_failed_cases += 1;
                    }
                    if let Some(problem) = fresh_run.adapted_at {
                        fresh_adapted += 1;
                        fresh_problems += problem;
                        fresh_cost += fresh_run.cumulative_cost;
                    }
                    if match (mature_run.adapted_at, fresh_run.adapted_at) {
                        (Some(mature), Some(fresh)) => {
                            mature > fresh || mature_run.cumulative_cost > fresh_run.cumulative_cost
                        }
                        (None, Some(_)) => true,
                        _ => false,
                    } {
                        mature_slower_than_scratch_cases += 1;
                    }
                }

                points.push(RemapPoint {
                    choice_count,
                    maturity,
                    remap_case: remap_case.name().to_string(),
                    mature_adapted_runs: mature_adapted,
                    fresh_adapted_runs: fresh_adapted,
                    total_runs: REMAP_SEEDS,
                    mature_average_problems: (mature_adapted > 0)
                        .then_some(mature_problems as f64 / mature_adapted as f64),
                    fresh_average_problems: (fresh_adapted > 0)
                        .then_some(fresh_problems as f64 / fresh_adapted as f64),
                    mature_average_cost: (mature_adapted > 0)
                        .then_some(mature_cost as f64 / mature_adapted as f64),
                    fresh_average_cost: (fresh_adapted > 0)
                        .then_some(fresh_cost as f64 / fresh_adapted as f64),
                    mature_average_observed_cost: mature_observed_cost as f64 / REMAP_SEEDS as f64,
                    fresh_average_observed_cost: fresh_observed_cost as f64 / REMAP_SEEDS as f64,
                    mature_average_old_collapse_problem: average_recorded_problem(
                        &mature_old_collapse,
                    ),
                    mature_average_exploration_resume_problem: average_recorded_problem(
                        &mature_exploration_resume,
                    ),
                    mature_average_new_preferred_problem: average_recorded_problem(
                        &mature_new_preferred,
                    ),
                    mature_old_value_before: mature_old_before as f64 / REMAP_SEEDS as f64,
                    mature_new_value_before: mature_new_before as f64 / REMAP_SEEDS as f64,
                });
            }
        }
    }

    let all_fresh_adapt = points
        .iter()
        .all(|point| point.fresh_adapted_runs == point.total_runs);
    let all_workspaces_destroyed = workspace_audit.0 == workspace_audit.1 && workspace_audit.2 == 0;
    let diagnostic_found_rigidity = mature_slower_than_scratch_cases > 0;
    let passed = all_fresh_adapt && all_workspaces_destroyed && diagnostic_found_rigidity;

    RemapReport {
        points,
        trajectories,
        workspaces_created: workspace_audit.0,
        workspaces_destroyed: workspace_audit.1,
        maximum_live_workspaces_after_problem: workspace_audit.2,
        mature_slower_than_scratch_cases,
        mature_failed_cases,
        passed,
    }
}

pub fn print_remap_report(report: &RemapReport) {
    println!("d2.2 silent action remapping:");
    for point in &report.points {
        println!(
            "  choices {:>2}, maturity {:>3}, {:>8}: adapted mature/fresh={}/{}, problems={:?}/{:?}, adapted cost={:?}/{:?}, observed cost={:.1}/{:.1}, collapse/explore/new={:?}/{:?}/{:?}, initial old/new={:.1}/{:.1}",
            point.choice_count,
            point.maturity,
            point.remap_case,
            point.mature_adapted_runs,
            point.fresh_adapted_runs,
            point.mature_average_problems,
            point.fresh_average_problems,
            point.mature_average_cost,
            point.fresh_average_cost,
            point.mature_average_observed_cost,
            point.fresh_average_observed_cost,
            point.mature_average_old_collapse_problem,
            point.mature_average_exploration_resume_problem,
            point.mature_average_new_preferred_problem,
            point.mature_old_value_before,
            point.mature_new_value_before
        );
    }
    println!(
        "  rigidity cases={}, no-adaptation cases={}, workspaces created/destroyed={}/{}, max live={}",
        report.mature_slower_than_scratch_cases,
        report.mature_failed_cases,
        report.workspaces_created,
        report.workspaces_destroyed,
        report.maximum_live_workspaces_after_problem
    );
}

const REOPEN_VIOLATIONS: usize = 3;
const RECONSOLIDATE_SUCCESSES: usize = 2;
const PLASTICITY_SEEDS: usize = 2;
const PLASTICITY_MAX_PROBLEMS: usize = 100;

#[derive(Clone, Debug, Default)]
struct HistoricalActionEvidence {
    informative: usize,
    disruptive: usize,
    uninformative: usize,
}

impl HistoricalActionEvidence {
    fn record(&mut self, result: InformationResult) {
        match result {
            InformationResult::Informative => self.informative += 1,
            InformationResult::Disruptive => self.disruptive += 1,
            InformationResult::Uninformative => self.uninformative += 1,
        }
    }

    fn preference(&self) -> i64 {
        self.informative as i64 * 3 - self.disruptive as i64 * 2 - self.uninformative as i64
    }

    fn total(&self) -> usize {
        self.informative + self.disruptive + self.uninformative
    }
}

#[derive(Clone, Debug)]
struct RegimeActionPolicy {
    historical: Vec<HistoricalActionEvidence>,
    current_values: Vec<i32>,
    current_tried: Vec<bool>,
    exploration_cursor: usize,
    trusted_action: Option<usize>,
    trusted: bool,
    violation_streak: usize,
    reconsolidating_action: Option<usize>,
    reconsolidation_streak: usize,
    violated_action: Option<usize>,
    reopen_count: usize,
    last_reopen_violation_count: Option<usize>,
}

impl RegimeActionPolicy {
    fn new(choice_count: usize, seed: u64) -> Self {
        Self {
            historical: vec![HistoricalActionEvidence::default(); choice_count],
            current_values: vec![0; choice_count],
            current_tried: vec![false; choice_count],
            exploration_cursor: seed as usize % choice_count,
            trusted_action: None,
            trusted: false,
            violation_streak: 0,
            reconsolidating_action: None,
            reconsolidation_streak: 0,
            violated_action: None,
            reopen_count: 0,
            last_reopen_violation_count: None,
        }
    }

    fn choose(&mut self, unresolved: bool) -> usize {
        let no_action = self.current_values.len() - 1;
        if !unresolved {
            return no_action;
        }
        if self.trusted {
            return self.trusted_action.expect("trusted action");
        }
        if let Some((index, _)) = self
            .current_values
            .iter()
            .take(no_action)
            .enumerate()
            .filter(|(_, value)| **value > 0)
            .max_by_key(|(_, value)| **value)
        {
            return index;
        }

        let mut candidates = Vec::new();
        for offset in 0..self.current_values.len() {
            let index = (self.exploration_cursor + offset) % self.current_values.len();
            if index != no_action && !self.current_tried[index] {
                candidates.push((index, offset));
            }
        }
        if let Some((index, _)) = candidates
            .into_iter()
            .max_by_key(|(index, offset)| (self.historical[*index].preference(), -(*offset as i64)))
        {
            self.exploration_cursor = (index + 1) % self.current_values.len();
            return index;
        }
        no_action
    }

    fn learn(&mut self, action: usize, result: InformationResult) {
        self.historical[action].record(result);
        if self.trusted {
            debug_assert_eq!(self.trusted_action, Some(action));
            if result == InformationResult::Informative {
                self.violation_streak = 0;
            } else {
                self.violation_streak += 1;
                if self.violation_streak >= REOPEN_VIOLATIONS {
                    self.reopen(action);
                }
            }
            return;
        }

        self.current_tried[action] = true;
        let no_action = self.current_values.len() - 1;
        let information_value = match result {
            InformationResult::Informative => 3,
            InformationResult::Disruptive => -2,
            InformationResult::Uninformative => 0,
        };
        let cost = i32::from(action != no_action) * ACTION_COST;
        self.current_values[action] =
            (self.current_values[action] + information_value - cost).clamp(-4, 4);

        if result == InformationResult::Informative {
            if self.reconsolidating_action == Some(action) {
                self.reconsolidation_streak += 1;
            } else {
                self.reconsolidating_action = Some(action);
                self.reconsolidation_streak = 1;
            }
            self.trusted_action = Some(action);
            if self.reconsolidation_streak >= RECONSOLIDATE_SUCCESSES {
                self.trusted = true;
                self.violation_streak = 0;
                self.violated_action = None;
            }
        } else if self.reconsolidating_action == Some(action) {
            self.reconsolidating_action = None;
            self.reconsolidation_streak = 0;
        }
    }

    fn reopen(&mut self, violated_action: usize) {
        self.trusted = false;
        self.trusted_action = None;
        self.last_reopen_violation_count = Some(self.violation_streak);
        self.violation_streak = 0;
        self.reconsolidating_action = None;
        self.reconsolidation_streak = 0;
        self.current_values.fill(0);
        self.current_tried.fill(false);
        self.current_values[violated_action] = -1;
        self.current_tried[violated_action] = true;
        self.violated_action = Some(violated_action);
        self.exploration_cursor = (violated_action + 1) % self.current_values.len();
        self.reopen_count += 1;
    }

    fn preferred_action(&self) -> Option<usize> {
        if self.trusted {
            return self.trusted_action;
        }
        let no_action = self.current_values.len() - 1;
        self.current_values
            .iter()
            .take(no_action)
            .enumerate()
            .filter(|(_, value)| **value > 0)
            .max_by_key(|(_, value)| **value)
            .map(|(index, _)| index)
    }

    fn historical_total(&self) -> usize {
        self.historical
            .iter()
            .map(HistoricalActionEvidence::total)
            .sum()
    }
}

#[derive(Clone, Debug)]
struct RegimeProblemResult {
    paid_actions: usize,
    correct: bool,
    selected_actions: Vec<usize>,
    reopened: bool,
    live_workspaces_after_drop: usize,
}

fn solve_regime_problem(
    policy: &mut RegimeActionPolicy,
    effects: &[ActionEffect],
    seed: u64,
    live_counter: Rc<Cell<usize>>,
) -> RegimeProblemResult {
    let no_action = effects.len();
    let mut workspace = FreshWorkspace::new(seed, live_counter.clone());
    prepare_workspace_competitors(&mut workspace);
    let reopen_before = policy.reopen_count;
    let mut paid_actions = 0;
    let mut selected_actions = Vec::new();

    while workspace.learner.consolidated.is_none()
        && selected_actions.len() < effects.len() + REOPEN_VIOLATIONS
    {
        prepare_workspace_competitors(&mut workspace);
        let was_trusted = policy.trusted;
        let action = policy.choose(true);
        selected_actions.push(action);
        paid_actions += usize::from(action != no_action);
        let effect = (action != no_action).then(|| effects[action]);
        let mut trace = ActionTrace::new(action, &workspace.learner);
        let mut evidence_steps = 0;
        while !trace.complete() {
            workspace.episode_number += 1;
            evidence_steps += 1;
            let episode =
                action_episode(&mut workspace.identities, workspace.episode_number, effect);
            workspace
                .learner
                .train_episode_deferred(&episode, workspace.episode_number);
            trace.observe_route(workspace.learner.last_used_route);
            assert!(
                evidence_steps < 500,
                "plasticity evidence window did not close"
            );
        }
        let information = trace.classify(&workspace.learner);
        policy.learn(action, information);
        workspace
            .learner
            .consolidate_unique_if_ready(workspace.episode_number);

        // A trusted mapping receives one independent expectation test per
        // problem. Exploration proceeds immediately only after reopening.
        if was_trusted && policy.trusted && workspace.learner.consolidated.is_none() {
            break;
        }
        if action == no_action && information == InformationResult::Uninformative {
            break;
        }
    }

    let (correct, _, _) = held_out_accuracy(
        &mut workspace.learner,
        seed ^ 0xd540_0000,
        Direction::LeftToRight,
        20,
        false,
    );
    let result = RegimeProblemResult {
        paid_actions,
        correct: workspace.learner.consolidated.is_some() && correct == 20,
        selected_actions,
        reopened: policy.reopen_count > reopen_before,
        live_workspaces_after_drop: usize::MAX,
    };
    drop(workspace);
    RegimeProblemResult {
        live_workspaces_after_drop: live_counter.get(),
        ..result
    }
}

#[derive(Clone, Debug)]
pub struct PlasticityTrajectoryPoint {
    pub choice_count: usize,
    pub maturity: usize,
    pub seed_index: usize,
    pub phase: String,
    pub problem: usize,
    pub old_action: usize,
    pub new_action: usize,
    pub preferred_action: Option<usize>,
    pub trusted: bool,
    pub violation_streak: usize,
    pub reopen_count: usize,
    pub selected_actions: Vec<usize>,
    pub paid_actions: usize,
    pub correct: bool,
    pub historical_evidence: usize,
}

#[derive(Clone, Debug)]
pub struct PlasticityPoint {
    pub choice_count: usize,
    pub maturity: usize,
    pub adapted_runs: usize,
    pub total_runs: usize,
    pub average_violations_to_reopen: f64,
    pub average_problems_to_adapt: f64,
    pub average_paid_actions: f64,
    pub reset_average_problems: f64,
    pub false_reopenings: usize,
}

#[derive(Clone, Debug)]
pub struct SwitchDiagnostic {
    pub phase: String,
    pub problems_to_adapt: usize,
    pub paid_actions: usize,
    pub violations_to_reopen: usize,
}

#[derive(Clone, Debug)]
pub struct PlasticityReport {
    pub points: Vec<PlasticityPoint>,
    pub trajectories: Vec<PlasticityTrajectoryPoint>,
    pub switch_diagnostic: Vec<SwitchDiagnostic>,
    pub noisy_false_reopenings: usize,
    pub unchanged_false_reopenings: usize,
    pub historical_evidence_preserved: bool,
    pub workspaces_created: usize,
    pub workspaces_destroyed: usize,
    pub maximum_live_workspaces_after_problem: usize,
    pub passed: bool,
}

#[derive(Clone, Debug)]
struct PlasticityAdaptation {
    adapted_at: Option<usize>,
    paid_actions: usize,
    violations_to_reopen: Option<usize>,
}

struct PlasticitySpec<'a> {
    choice_count: usize,
    maturity: usize,
    seed_index: usize,
    phase: &'a str,
    old_action: usize,
    new_action: usize,
}

fn run_plasticity_adaptation(
    policy: &mut RegimeActionPolicy,
    effects: &[ActionEffect],
    spec: &PlasticitySpec<'_>,
    seed: u64,
    live_counter: Rc<Cell<usize>>,
    trajectories: &mut Vec<PlasticityTrajectoryPoint>,
    workspace_audit: &mut (usize, usize, usize),
) -> PlasticityAdaptation {
    let mut paid_actions = 0;
    let mut success_streak = 0;
    let mut adapted_at = None;
    let mut violations_to_reopen = None;

    for problem in 1..=PLASTICITY_MAX_PROBLEMS {
        let result = solve_regime_problem(
            policy,
            effects,
            seed + problem as u64 * 100_000,
            live_counter.clone(),
        );
        workspace_audit.0 += 1;
        workspace_audit.1 += usize::from(result.live_workspaces_after_drop == 0);
        workspace_audit.2 = workspace_audit.2.max(result.live_workspaces_after_drop);
        paid_actions += result.paid_actions;
        if result.reopened && violations_to_reopen.is_none() {
            violations_to_reopen = policy.last_reopen_violation_count;
        }
        let clean_success = result.correct
            && result.paid_actions == 1
            && result.selected_actions == [spec.new_action]
            && policy.trusted
            && policy.preferred_action() == Some(spec.new_action);
        success_streak = if clean_success { success_streak + 1 } else { 0 };
        trajectories.push(PlasticityTrajectoryPoint {
            choice_count: spec.choice_count,
            maturity: spec.maturity,
            seed_index: spec.seed_index,
            phase: spec.phase.to_string(),
            problem,
            old_action: spec.old_action,
            new_action: spec.new_action,
            preferred_action: policy.preferred_action(),
            trusted: policy.trusted,
            violation_streak: policy.violation_streak,
            reopen_count: policy.reopen_count,
            selected_actions: result.selected_actions,
            paid_actions: result.paid_actions,
            correct: result.correct,
            historical_evidence: policy.historical_total(),
        });
        if success_streak == REMAP_SUCCESS_STREAK {
            adapted_at = Some(problem);
            break;
        }
    }

    PlasticityAdaptation {
        adapted_at,
        paid_actions,
        violations_to_reopen,
    }
}

fn mature_regime_policy(
    choice_count: usize,
    maturity: usize,
    seed: u64,
    live_counter: Rc<Cell<usize>>,
    workspace_audit: &mut (usize, usize, usize),
) -> (RegimeActionPolicy, Vec<ActionEffect>, usize, usize) {
    let template = ScalableActionPolicy::new(choice_count, seed ^ 0xd511_0000);
    let (effects, informative, rejected, _) = remap_initial_effects(choice_count, &template);
    let mut policy = RegimeActionPolicy::new(choice_count, seed ^ 0xd511_0000);
    for problem in 0..maturity {
        let result = solve_regime_problem(
            &mut policy,
            &effects,
            seed + problem as u64 * 10_000,
            live_counter.clone(),
        );
        workspace_audit.0 += 1;
        workspace_audit.1 += usize::from(result.live_workspaces_after_drop == 0);
        workspace_audit.2 = workspace_audit.2.max(result.live_workspaces_after_drop);
        assert!(result.correct, "maturity problem did not resolve");
    }
    assert!(policy.trusted);
    assert_eq!(policy.preferred_action(), Some(informative));
    (policy, effects, informative, rejected)
}

fn run_noise_control(
    policy: &mut RegimeActionPolicy,
    effects: &[ActionEffect],
    seed: u64,
    live_counter: Rc<Cell<usize>>,
    workspace_audit: &mut (usize, usize, usize),
    noisy: bool,
) -> usize {
    let reopen_before = policy.reopen_count;
    for problem in 1..=60 {
        let mut problem_effects = effects.to_vec();
        if noisy && problem % 10 == 0 {
            let preferred = policy.preferred_action().unwrap();
            problem_effects[preferred] = ActionEffect::Inert;
        }
        let result = solve_regime_problem(
            policy,
            &problem_effects,
            seed + problem as u64 * 100_000,
            live_counter.clone(),
        );
        workspace_audit.0 += 1;
        workspace_audit.1 += usize::from(result.live_workspaces_after_drop == 0);
        workspace_audit.2 = workspace_audit.2.max(result.live_workspaces_after_drop);
    }
    policy.reopen_count - reopen_before
}

pub fn run_plasticity_experiment() -> PlasticityReport {
    let choice_counts = [16, 64];
    let maturities = [10, 50, 100];
    let live_counter = Rc::new(Cell::new(0));
    let mut workspace_audit = (0usize, 0usize, 0usize);
    let mut points = Vec::new();
    let mut trajectories = Vec::new();
    let mut noisy_false_reopenings = 0;
    let mut unchanged_false_reopenings = 0;
    let mut historical_evidence_preserved = true;

    for choice_count in choice_counts {
        for maturity in maturities {
            let mut adapted_runs = 0;
            let mut violations = 0;
            let mut problems = 0;
            let mut actions = 0;
            let mut reset_problems = 0;

            for seed_index in 0..PLASTICITY_SEEDS {
                let seed = 0xd510_0000 + choice_count as u64 * 100_000 + seed_index as u64;
                let (mut policy, original, old_action, rejected_action) = mature_regime_policy(
                    choice_count,
                    maturity,
                    seed,
                    live_counter.clone(),
                    &mut workspace_audit,
                );
                let history_before = policy.historical[old_action].informative;
                let remapped = remapped_effects(&original, old_action, rejected_action);
                let spec = PlasticitySpec {
                    choice_count,
                    maturity,
                    seed_index,
                    phase: "first-remap",
                    old_action,
                    new_action: rejected_action,
                };
                let adaptation = run_plasticity_adaptation(
                    &mut policy,
                    &remapped,
                    &spec,
                    seed ^ 0xd512_0000,
                    live_counter.clone(),
                    &mut trajectories,
                    &mut workspace_audit,
                );
                if let Some(adapted) = adaptation.adapted_at {
                    adapted_runs += 1;
                    problems += adapted;
                }
                violations += adaptation.violations_to_reopen.unwrap_or(0);
                actions += adaptation.paid_actions;
                historical_evidence_preserved &=
                    policy.historical[old_action].informative >= history_before;

                let mut reset_policy = RegimeActionPolicy::new(choice_count, seed ^ 0xd511_0000);
                let reset_spec = PlasticitySpec {
                    phase: "full-reset",
                    ..spec
                };
                let reset = run_plasticity_adaptation(
                    &mut reset_policy,
                    &remapped,
                    &reset_spec,
                    seed ^ 0xd512_0000,
                    live_counter.clone(),
                    &mut Vec::new(),
                    &mut workspace_audit,
                );
                reset_problems += reset.adapted_at.unwrap_or(PLASTICITY_MAX_PROBLEMS);

                if maturity == 100 {
                    let (mut unchanged, effects, _, _) = mature_regime_policy(
                        choice_count,
                        maturity,
                        seed ^ 0xd513_0000,
                        live_counter.clone(),
                        &mut workspace_audit,
                    );
                    unchanged_false_reopenings += run_noise_control(
                        &mut unchanged,
                        &effects,
                        seed ^ 0xd514_0000,
                        live_counter.clone(),
                        &mut workspace_audit,
                        false,
                    );
                    let (mut noisy, effects, _, _) = mature_regime_policy(
                        choice_count,
                        maturity,
                        seed ^ 0xd515_0000,
                        live_counter.clone(),
                        &mut workspace_audit,
                    );
                    noisy_false_reopenings += run_noise_control(
                        &mut noisy,
                        &effects,
                        seed ^ 0xd516_0000,
                        live_counter.clone(),
                        &mut workspace_audit,
                        true,
                    );
                }
            }

            points.push(PlasticityPoint {
                choice_count,
                maturity,
                adapted_runs,
                total_runs: PLASTICITY_SEEDS,
                average_violations_to_reopen: violations as f64 / PLASTICITY_SEEDS as f64,
                average_problems_to_adapt: problems as f64 / PLASTICITY_SEEDS as f64,
                average_paid_actions: actions as f64 / PLASTICITY_SEEDS as f64,
                reset_average_problems: reset_problems as f64 / PLASTICITY_SEEDS as f64,
                false_reopenings: 0,
            });
        }
    }

    let diagnostic_seed = 0xd590_0001;
    let (mut switch_policy, original, action_a, action_b) = mature_regime_policy(
        16,
        50,
        diagnostic_seed,
        live_counter.clone(),
        &mut workspace_audit,
    );
    let mut switch_diagnostic = Vec::new();
    let phases = [
        (
            "second-regime",
            action_a,
            action_b,
            remapped_effects(&original, action_a, action_b),
        ),
        ("first-regime", action_b, action_a, original.clone()),
        (
            "second-regime-again",
            action_a,
            action_b,
            remapped_effects(&original, action_a, action_b),
        ),
    ];
    for (phase_index, (phase, old_action, new_action, effects)) in phases.into_iter().enumerate() {
        let spec = PlasticitySpec {
            choice_count: 16,
            maturity: 50,
            seed_index: 0,
            phase,
            old_action,
            new_action,
        };
        let adaptation = run_plasticity_adaptation(
            &mut switch_policy,
            &effects,
            &spec,
            diagnostic_seed ^ 0xd591_0000 ^ phase_index as u64,
            live_counter.clone(),
            &mut trajectories,
            &mut workspace_audit,
        );
        switch_diagnostic.push(SwitchDiagnostic {
            phase: phase.to_string(),
            problems_to_adapt: adaptation.adapted_at.unwrap_or(PLASTICITY_MAX_PROBLEMS),
            paid_actions: adaptation.paid_actions,
            violations_to_reopen: adaptation.violations_to_reopen.unwrap_or(0),
        });
    }

    let maturity_independent = choice_counts.iter().all(|choice_count| {
        let matching: Vec<_> = points
            .iter()
            .filter(|point| point.choice_count == *choice_count)
            .collect();
        let (minimum, maximum) = matching
            .iter()
            .map(|point| point.average_problems_to_adapt)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), value| {
                (min.min(value), max.max(value))
            });
        matching
            .iter()
            .all(|point| point.average_violations_to_reopen == REOPEN_VIOLATIONS as f64)
            && maximum - minimum <= 1.0
    });
    let all_adapt = points
        .iter()
        .all(|point| point.adapted_runs == point.total_runs);
    let all_workspaces_destroyed = workspace_audit.0 == workspace_audit.1 && workspace_audit.2 == 0;
    let switch_back_pass = switch_diagnostic
        .iter()
        .all(|point| point.problems_to_adapt < PLASTICITY_MAX_PROBLEMS)
        && switch_diagnostic[1].paid_actions < switch_diagnostic[0].paid_actions;
    let passed = all_adapt
        && maturity_independent
        && noisy_false_reopenings == 0
        && unchanged_false_reopenings == 0
        && historical_evidence_preserved
        && switch_back_pass
        && all_workspaces_destroyed;

    PlasticityReport {
        points,
        trajectories,
        switch_diagnostic,
        noisy_false_reopenings,
        unchanged_false_reopenings,
        historical_evidence_preserved,
        workspaces_created: workspace_audit.0,
        workspaces_destroyed: workspace_audit.1,
        maximum_live_workspaces_after_problem: workspace_audit.2,
        passed,
    }
}

pub fn print_plasticity_report(report: &PlasticityReport) {
    println!("d2.3 expectation-triggered reopening:");
    for point in &report.points {
        println!(
            "  choices {:>2}, maturity {:>3}: adapted={}/{}, violations={:.1}, problems={:.1}, paid actions={:.1}, reset problems={:.1}",
            point.choice_count,
            point.maturity,
            point.adapted_runs,
            point.total_runs,
            point.average_violations_to_reopen,
            point.average_problems_to_adapt,
            point.average_paid_actions,
            point.reset_average_problems
        );
    }
    for point in &report.switch_diagnostic {
        println!(
            "  switch {}: violations={}, problems={}, paid actions={}",
            point.phase, point.violations_to_reopen, point.problems_to_adapt, point.paid_actions
        );
    }
    println!(
        "  false reopenings noisy/unchanged={}/{}, history preserved={}, workspaces created/destroyed={}/{}, max live={}",
        report.noisy_false_reopenings,
        report.unchanged_false_reopenings,
        report.historical_evidence_preserved,
        report.workspaces_created,
        report.workspaces_destroyed,
        report.maximum_live_workspaces_after_problem
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn amortization_report() -> &'static AmortizationReport {
        static REPORT: OnceLock<AmortizationReport> = OnceLock::new();
        REPORT.get_or_init(run_amortization_experiment)
    }

    fn remap_report() -> &'static RemapReport {
        static REPORT: OnceLock<RemapReport> = OnceLock::new();
        REPORT.get_or_init(run_remap_experiment)
    }

    fn plasticity_report() -> &'static PlasticityReport {
        static REPORT: OnceLock<PlasticityReport> = OnceLock::new();
        REPORT.get_or_init(run_plasticity_experiment)
    }

    #[test]
    fn d0_discovers_forward_topology_without_supplied_route_candidates() {
        let report = run_experiment();
        assert_eq!(report.forward_route, "Slot1->Slot2");
        assert_eq!(report.stable_arrows, 1);
        assert!(report.proposed_arrows > report.stable_arrows);
        assert!(report
            .forward_seeds
            .iter()
            .all(|seed| seed.correct == seed.total));
    }

    #[test]
    fn d0_same_prior_discovers_reverse_topology_from_reverse_experience() {
        let report = run_experiment();
        assert_eq!(report.reverse_route, "Slot2->Slot1");
        assert!(report
            .reverse_seeds
            .iter()
            .all(|seed| seed.correct == seed.total));
    }

    #[test]
    fn d0_random_labels_do_not_create_a_stable_route() {
        let report = run_experiment();
        assert_eq!(report.random_label_stable_arrows, 0);
        assert!(report.random_label_training_correct < 1_500);
    }

    #[test]
    fn d0_held_out_renamings_do_not_change_permanent_topology() {
        let report = run_experiment();
        assert!(report.held_out_fingerprint_unchanged);
        assert!(report.passed);
    }

    #[test]
    fn d0_records_shortcut_vulnerability() {
        let report = run_experiment();
        assert!(report.shortcut_failures_without_cue > 0);
    }

    #[test]
    fn d1_contrasting_experience_removes_the_observation_only_shortcut() {
        let report = run_intervention_experiment();
        assert!(report.observation_shortcut_failures > 0);
        assert_eq!(report.contrasting_shortcut_failures, 0);
        assert_eq!(report.contrasting_forward_routes, 32);
    }

    #[test]
    fn d1_counterbalances_every_observable_cue_location_and_answer_change() {
        let report = run_intervention_experiment();
        assert!(report
            .cue_offset_counts
            .iter()
            .all(|count| *count == report.cue_offset_counts[0]));
        assert_eq!(
            report.unchanged_answer_episodes,
            report.changed_answer_episodes
        );
    }

    #[test]
    fn d1_strength_history_records_shortcut_reversal_before_consolidation() {
        let report = run_intervention_experiment();
        let shortcut_peak = report
            .representative_history
            .iter()
            .filter_map(|point| point.shortcut_route_strength)
            .max()
            .unwrap();
        let shortcut_last = report
            .representative_history
            .iter()
            .rev()
            .find_map(|point| point.shortcut_route_strength)
            .unwrap_or(0);
        assert!(shortcut_peak > shortcut_last);
        assert!(report
            .representative_history
            .iter()
            .any(|point| point.real_route_strength == Some(CONSOLIDATION_STRENGTH)));
    }

    #[test]
    fn d1_held_out_and_random_label_controls_remain_clean() {
        let report = run_intervention_experiment();
        assert!(report.held_out_fingerprint_unchanged);
        assert_eq!(report.random_label_stable_arrows, 0);
        assert!(report.passed);
    }

    #[test]
    fn d2_learns_the_informative_action_across_opaque_permutations() {
        let report = run_active_discovery_experiment();
        assert_eq!(report.learned_correct_runs, 32);
        assert_eq!(report.informative_preferred_runs, 32);
        assert!(report.action_permutations_seen >= 6);
    }

    #[test]
    fn d2_rejects_disruption_and_stops_paying_after_resolution() {
        let report = run_active_discovery_experiment();
        assert_eq!(report.disruptive_preferred_runs, 0);
        assert_eq!(report.post_resolution_paid_actions, 0);
    }

    #[test]
    fn d2_baselines_expose_random_search_and_reject_fixed_or_passive_habits() {
        let report = run_active_discovery_experiment();
        assert_eq!(report.random_correct_runs, report.learned_correct_runs);
        assert!(report.average_random_paid_actions < report.average_learned_paid_actions);
        assert!(report.fixed_correct_runs < report.learned_correct_runs);
        assert!(report.no_action_correct_runs < report.learned_correct_runs);
    }

    #[test]
    fn d2_random_labels_create_neither_topology_nor_action_preference() {
        let report = run_active_discovery_experiment();
        assert_eq!(report.random_label_positive_actions, 0);
        assert_eq!(report.random_label_stable_arrows, 0);
        assert!(report.passed);
    }

    #[test]
    fn d2_1_mature_policy_reuses_one_informative_action() {
        let report = amortization_report();
        assert!(report.points.iter().all(|point| {
            (point.mature_learned_actions - 1.0).abs() < 0.01
                && (point.mature_oracle_actions - 1.0).abs() < 0.01
        }));
    }

    #[test]
    fn d2_1_random_search_cost_grows_with_action_space() {
        let report = amortization_report();
        assert!(report
            .points
            .windows(2)
            .all(|pair| pair[1].mature_random_actions > pair[0].mature_random_actions));
        assert!(report.points.iter().skip(1).all(|point| {
            point.mature_learned_actions < point.mature_random_actions
                && point.break_even_problem.is_some()
        }));
    }

    #[test]
    fn d2_1_destroys_every_problem_workspace() {
        let report = amortization_report();
        assert_eq!(report.workspaces_created, report.workspaces_destroyed);
        assert_eq!(report.maximum_live_workspaces_after_problem, 0);
        assert_eq!(report.policy_retained_problem_identities, 0);
        assert!(report.policy_only_state_crosses_problems);
    }

    #[test]
    fn d2_1_preserves_accuracy_for_learned_random_and_oracle_policies() {
        let report = amortization_report();
        assert!(report.points.iter().all(|point| {
            point.learned_correct == point.total_problems
                && point.random_correct == point.total_problems
                && point.oracle_correct == point.total_problems
        }));
        assert!(report.passed);
    }

    #[test]
    fn d2_2_fresh_policy_adapts_under_every_new_mapping() {
        let report = remap_report();
        assert!(report
            .points
            .iter()
            .all(|point| point.fresh_adapted_runs == point.total_runs));
    }

    #[test]
    fn d2_2_exposes_rigidity_without_adding_plasticity() {
        let report = remap_report();
        assert!(report.mature_slower_than_scratch_cases > 0);
        assert!(report.mature_failed_cases > 0);
    }

    #[test]
    fn d2_2_destroys_every_adaptation_workspace() {
        let report = remap_report();
        assert_eq!(report.workspaces_created, report.workspaces_destroyed);
        assert_eq!(report.maximum_live_workspaces_after_problem, 0);
    }

    #[test]
    fn d2_2_records_complete_old_and_new_value_trajectories() {
        let report = remap_report();
        assert!(!report.trajectories.is_empty());
        assert!(report
            .trajectories
            .iter()
            .any(|point| point.policy_kind == "mature"));
        assert!(report
            .trajectories
            .iter()
            .any(|point| point.policy_kind == "fresh"));
        assert!(report.passed);
    }

    #[test]
    fn d2_3_reopens_rejected_actions_after_a_fixed_violation_streak() {
        let report = plasticity_report();
        assert!(report.points.iter().all(|point| {
            point.adapted_runs == point.total_runs
                && point.average_violations_to_reopen == REOPEN_VIOLATIONS as f64
        }));
    }

    #[test]
    fn d2_3_adaptation_time_is_independent_of_prior_maturity() {
        let report = plasticity_report();
        for choice_count in [16, 64] {
            let values: Vec<_> = report
                .points
                .iter()
                .filter(|point| point.choice_count == choice_count)
                .map(|point| point.average_problems_to_adapt)
                .collect();
            let minimum = values.iter().copied().fold(f64::INFINITY, f64::min);
            let maximum = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            assert!(maximum - minimum <= 1.0);
        }
    }

    #[test]
    fn d2_3_ignores_isolated_noise_and_preserves_history() {
        let report = plasticity_report();
        assert_eq!(report.noisy_false_reopenings, 0);
        assert_eq!(report.unchanged_false_reopenings, 0);
        assert!(report.historical_evidence_preserved);
    }

    #[test]
    fn d2_3_switches_back_without_leaking_problem_workspaces() {
        let report = plasticity_report();
        assert_eq!(report.switch_diagnostic.len(), 3);
        assert!(report
            .switch_diagnostic
            .iter()
            .all(|point| point.problems_to_adapt < PLASTICITY_MAX_PROBLEMS));
        assert!(
            report.switch_diagnostic[1].paid_actions < report.switch_diagnostic[0].paid_actions
        );
        assert_eq!(report.workspaces_created, report.workspaces_destroyed);
        assert_eq!(report.maximum_live_workspaces_after_problem, 0);
        assert!(report.passed);
    }
}
