use std::collections::{HashSet, VecDeque};

use crate::binding::{BindingOutcome, IdentitySource, OpaqueId};

const COACTIVITY_WINDOW: usize = 2;
const CONSOLIDATION_STRENGTH: i32 = 4;

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

    fn terminal_feedback(&mut self, success: bool, episode_number: usize) {
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

    fn train_episode(&mut self, episode: &Episode, episode_number: usize) -> bool {
        let mut work = self.present(episode, true);
        let proposal = self.propose_answer(episode, &mut work);
        let success = proposal == episode.correct;
        self.terminal_feedback(success, episode_number);
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
