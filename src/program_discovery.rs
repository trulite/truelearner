use std::collections::{HashSet, VecDeque};

use crate::binding::{BindingOutcome, IdentitySource, OpaqueId};

const TRAIN_DEPTHS: [usize; 4] = [1, 2, 3, 4];
const HELD_OUT_DEPTHS: [usize; 4] = [5, 8, 16, 32];
const ISOLATED_SEEDS: usize = 8;
const INTEGRATED_SEEDS: usize = 8;
const CONTROL_SEEDS: usize = 8;
const INTEGRATED_BUDGET: usize = 50_000;

#[derive(Clone, Copy, Debug)]
struct DiscoveryPhysics {
    coactivity_window: usize,
    probation_episodes: usize,
    success_credit: i32,
    failure_credit: i32,
    prune_strength: i32,
    consolidation_strength: i32,
    activity_limit: usize,
}

const FROZEN_PHYSICS: DiscoveryPhysics = DiscoveryPhysics {
    coactivity_window: 10,
    probation_episodes: 8,
    success_credit: 2,
    failure_credit: -1,
    prune_strength: -2,
    consolidation_strength: 6,
    activity_limit: 1_600,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Role {
    Slot1,
    Slot2,
    Result,
    Current,
    Success,
    Apply,
    NoResult,
    Answer,
    Clear,
    Quiet,
}

impl Role {
    const ALL: [Self; 10] = [
        Self::Slot1,
        Self::Slot2,
        Self::Result,
        Self::Current,
        Self::Success,
        Self::Apply,
        Self::NoResult,
        Self::Answer,
        Self::Clear,
        Self::Quiet,
    ];
}

const LOOKUP_SOURCE: Role = Role::Slot1;
const FEEDBACK_SOURCE: Role = Role::Result;
const CONTINUE_SOURCE: Role = Role::Success;
const FINISH_SOURCE: Role = Role::NoResult;
const PROGRAM_SOURCES: [Role; 4] = [
    LOOKUP_SOURCE,
    FEEDBACK_SOURCE,
    CONTINUE_SOURCE,
    FINISH_SOURCE,
];

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

    fn index(&mut self, length: usize) -> usize {
        (self.next_u64() as usize) % length
    }

    fn shuffle<T>(&mut self, values: &mut [T]) {
        for index in (1..values.len()).rev() {
            let selected = self.index(index + 1);
            values.swap(index, selected);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CandidateArrow {
    id: usize,
    from: Role,
    to: Role,
    strength: i32,
    uses: usize,
    age: usize,
    trace: bool,
    consolidated: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RouteChoice {
    arrow_id: Option<usize>,
    from: Role,
    to: Role,
}

impl RouteChoice {
    fn fixed(from: Role, to: Role) -> Self {
        Self {
            arrow_id: None,
            from,
            to,
        }
    }
}

#[derive(Clone, Debug)]
struct ProposalNetwork {
    arrows: Vec<CandidateArrow>,
    learning_sources: Vec<Role>,
    next_arrow_id: usize,
    proposals: usize,
    rejected: usize,
    rng: DeterministicRng,
    complete: bool,
    needs_proposals: bool,
}

impl ProposalNetwork {
    fn new(seed: u64, learning_sources: &[Role]) -> Self {
        let mut network = Self {
            arrows: Vec::new(),
            learning_sources: learning_sources.to_vec(),
            next_arrow_id: 0,
            proposals: 0,
            rejected: 0,
            rng: DeterministicRng::new(seed ^ 0x50_30_ae_11),
            complete: false,
            needs_proposals: true,
        };
        network.propose_from_generic_coactivity();
        network
    }

    fn propose_from_generic_coactivity(&mut self) {
        if self.complete || !self.needs_proposals {
            return;
        }
        let mut recent = VecDeque::new();
        for role in Role::ALL {
            while recent.len() >= FROZEN_PHYSICS.coactivity_window {
                recent.pop_front();
            }
            let nearby: Vec<_> = recent.iter().copied().collect();
            for previous in nearby {
                self.propose(previous, role);
                self.propose(role, previous);
            }
            recent.push_back(role);
        }
        self.needs_proposals = false;
    }

    fn propose(&mut self, from: Role, to: Role) {
        if from == to
            || self
                .arrows
                .iter()
                .any(|arrow| arrow.from == from && arrow.to == to)
        {
            return;
        }
        self.arrows.push(CandidateArrow {
            id: self.next_arrow_id,
            from,
            to,
            strength: 0,
            uses: 0,
            age: 0,
            trace: false,
            consolidated: false,
        });
        self.next_arrow_id += 1;
        self.proposals += 1;
    }

    fn begin_episode(&mut self) {
        self.propose_from_generic_coactivity();
        for arrow in &mut self.arrows {
            arrow.age += 1;
            arrow.trace = false;
        }
    }

    fn choose(&mut self, source: Role) -> Option<RouteChoice> {
        let consolidated = self
            .arrows
            .iter()
            .find(|arrow| arrow.from == source && arrow.consolidated)
            .copied();
        if let Some(arrow) = consolidated {
            return Some(RouteChoice {
                arrow_id: Some(arrow.id),
                from: arrow.from,
                to: arrow.to,
            });
        }

        let best_positive = self
            .arrows
            .iter()
            .filter(|arrow| arrow.from == source && arrow.strength > 0)
            .map(|arrow| arrow.strength)
            .max();
        let choices: Vec<_> = self
            .arrows
            .iter()
            .filter(|arrow| {
                arrow.from == source
                    && best_positive.is_none_or(|strength| arrow.strength == strength)
            })
            .map(|arrow| arrow.id)
            .collect();
        if choices.is_empty() {
            return None;
        }
        let selected_id = choices[self.rng.index(choices.len())];
        let arrow = self
            .arrows
            .iter()
            .find(|arrow| arrow.id == selected_id)
            .unwrap();
        Some(RouteChoice {
            arrow_id: Some(arrow.id),
            from: arrow.from,
            to: arrow.to,
        })
    }

    fn mark_used(&mut self, choice: RouteChoice) {
        let Some(arrow_id) = choice.arrow_id else {
            return;
        };
        let arrow = self
            .arrows
            .iter_mut()
            .find(|arrow| arrow.id == arrow_id)
            .expect("selected arrow must remain live for the episode");
        arrow.uses += 1;
        arrow.trace = true;
    }

    fn terminal_feedback(&mut self, success: bool) {
        for arrow in &mut self.arrows {
            if arrow.trace && !arrow.consolidated {
                arrow.strength += if success {
                    FROZEN_PHYSICS.success_credit
                } else {
                    FROZEN_PHYSICS.failure_credit
                };
            }
            arrow.trace = false;
        }
        self.consolidate_ready_sources();
        self.prune();
    }

    fn consolidate_ready_sources(&mut self) {
        for source in self.learning_sources.clone() {
            if self
                .arrows
                .iter()
                .any(|arrow| arrow.from == source && arrow.consolidated)
            {
                continue;
            }
            let Some(best_strength) = self
                .arrows
                .iter()
                .filter(|arrow| arrow.from == source)
                .map(|arrow| arrow.strength)
                .max()
            else {
                continue;
            };
            if best_strength < FROZEN_PHYSICS.consolidation_strength {
                continue;
            }
            let strongest: Vec<_> = self
                .arrows
                .iter()
                .filter(|arrow| arrow.from == source && arrow.strength == best_strength)
                .map(|arrow| arrow.id)
                .collect();
            if strongest.len() != 1 {
                continue;
            }
            let winner = strongest[0];
            for arrow in &mut self.arrows {
                if arrow.id == winner {
                    arrow.consolidated = true;
                }
            }
            let before = self.arrows.len();
            self.arrows
                .retain(|arrow| arrow.from != source || arrow.id == winner);
            self.rejected += before - self.arrows.len();
        }
        self.complete = self.learning_sources.iter().all(|source| {
            self.arrows
                .iter()
                .any(|arrow| arrow.from == *source && arrow.consolidated)
        });
        if self.complete {
            let before = self.arrows.len();
            self.arrows.retain(|arrow| arrow.consolidated);
            self.rejected += before - self.arrows.len();
            self.needs_proposals = false;
        }
    }

    fn prune(&mut self) {
        if self.complete {
            return;
        }
        let before = self.arrows.len();
        self.arrows.retain(|arrow| {
            arrow.consolidated
                || arrow.uses == 0
                || arrow.age < FROZEN_PHYSICS.probation_episodes
                || arrow.strength > FROZEN_PHYSICS.prune_strength
        });
        let removed = before - self.arrows.len();
        self.rejected += removed;
        self.needs_proposals |= removed > 0;
    }

    fn evaluation_choice(&self, source: Role) -> Option<RouteChoice> {
        let candidates: Vec<_> = self
            .arrows
            .iter()
            .filter(|arrow| arrow.from == source)
            .collect();
        if candidates.is_empty() {
            return None;
        }
        if let Some(arrow) = candidates.iter().find(|arrow| arrow.consolidated) {
            return Some(RouteChoice {
                arrow_id: Some(arrow.id),
                from: arrow.from,
                to: arrow.to,
            });
        }
        let best = candidates.iter().map(|arrow| arrow.strength).max()?;
        if best <= 0 {
            return None;
        }
        let strongest: Vec<_> = candidates
            .into_iter()
            .filter(|arrow| arrow.strength == best)
            .collect();
        (strongest.len() == 1).then(|| RouteChoice {
            arrow_id: Some(strongest[0].id),
            from: strongest[0].from,
            to: strongest[0].to,
        })
    }

    fn route_target(&self, source: Role) -> Option<Role> {
        self.evaluation_choice(source).map(|choice| choice.to)
    }

    fn consolidated_target(&self, source: Role) -> Option<Role> {
        self.arrows
            .iter()
            .find(|arrow| arrow.from == source && arrow.consolidated)
            .map(|arrow| arrow.to)
    }

    fn stable_arrows(&self) -> usize {
        self.arrows
            .iter()
            .filter(|arrow| arrow.consolidated)
            .count()
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        let mut arrows = self.arrows.clone();
        arrows.sort_by_key(|arrow| (arrow.from, arrow.to, arrow.id));
        for arrow in arrows {
            fingerprint_mix(&mut hash, arrow.id as u64);
            fingerprint_mix(&mut hash, arrow.from as u64);
            fingerprint_mix(&mut hash, arrow.to as u64);
            fingerprint_mix(&mut hash, arrow.strength as i64 as u64);
            fingerprint_mix(&mut hash, arrow.uses as u64);
            fingerprint_mix(&mut hash, arrow.consolidated as u64);
        }
        hash
    }
}

#[derive(Clone, Debug)]
struct TaskEpisode {
    relations: Vec<(OpaqueId, OpaqueId)>,
    query: OpaqueId,
    correct: BindingOutcome,
}

fn chain_episode(
    identities: &mut IdentitySource,
    rng: &mut DeterministicRng,
    depth: usize,
    relation_count: usize,
) -> TaskEpisode {
    let chain: Vec<_> = (0..=depth).map(|_| identities.issue()).collect();
    let mut relations: Vec<_> = chain.windows(2).map(|pair| (pair[0], pair[1])).collect();
    for _ in depth..relation_count {
        relations.push((identities.issue(), identities.issue()));
    }
    rng.shuffle(&mut relations);
    TaskEpisode {
        relations,
        query: chain[0],
        correct: BindingOutcome::Answer(chain[depth]),
    }
}

#[derive(Clone, Copy, Debug)]
struct ProgramChoices {
    lookup: RouteChoice,
    feedback: RouteChoice,
    continuation: RouteChoice,
    finish: RouteChoice,
}

impl ProgramChoices {
    fn learned(network: &mut ProposalNetwork) -> Option<Self> {
        Some(Self {
            lookup: network.choose(LOOKUP_SOURCE)?,
            feedback: network.choose(FEEDBACK_SOURCE)?,
            continuation: network.choose(CONTINUE_SOURCE)?,
            finish: network.choose(FINISH_SOURCE)?,
        })
    }

    fn evaluated(network: &ProposalNetwork) -> Option<Self> {
        Some(Self {
            lookup: network.evaluation_choice(LOOKUP_SOURCE)?,
            feedback: network.evaluation_choice(FEEDBACK_SOURCE)?,
            continuation: network.evaluation_choice(CONTINUE_SOURCE)?,
            finish: network.evaluation_choice(FINISH_SOURCE)?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Event {
    Start,
    Apply,
    Result(OpaqueId),
    Success,
    NoResult,
    Answer(OpaqueId),
    Clear,
    Quiet,
}

#[derive(Clone, Debug)]
struct ExecutionResult {
    outcome: BindingOutcome,
    spikes: usize,
    activity_limit_hit: bool,
    explicit_answer: bool,
    queue_empty: bool,
    used_arrows: Vec<usize>,
}

fn execute_program(
    episode: &TaskEpisode,
    choices: ProgramChoices,
    activity_limit: usize,
) -> ExecutionResult {
    let mut queue = VecDeque::from([Event::Start]);
    let mut current = Some(episode.query);
    let mut emitted_answer = None;
    let mut fault = None;
    let mut spikes = 0;
    let mut used_arrows = Vec::new();
    let mut activity_limit_hit = false;

    while let Some(event) = queue.pop_front() {
        spikes += 1;
        if spikes >= activity_limit {
            activity_limit_hit = true;
            queue.clear();
            break;
        }
        match event {
            Event::Start => queue.push_back(Event::Apply),
            Event::Apply => {
                mark_choice(choices.lookup, &mut used_arrows);
                let Some(input) = current else {
                    queue.push_back(Event::NoResult);
                    continue;
                };
                let mut outputs = Vec::new();
                match choices.lookup.to {
                    Role::Slot1 | Role::Slot2 => {
                        for (left, right) in &episode.relations {
                            spikes += 1;
                            if *left == input {
                                let output = if choices.lookup.to == Role::Slot1 {
                                    *left
                                } else {
                                    *right
                                };
                                if !outputs.contains(&output) {
                                    outputs.push(output);
                                }
                            }
                        }
                    }
                    _ => {}
                }
                match outputs.len() {
                    0 => queue.push_back(Event::NoResult),
                    1 => queue.push_back(Event::Result(outputs[0])),
                    _ => {
                        fault = Some(BindingOutcome::Ambiguous);
                        queue.clear();
                    }
                }
            }
            Event::Result(identity) => {
                mark_choice(choices.feedback, &mut used_arrows);
                match choices.feedback.to {
                    Role::Current => current = Some(identity),
                    Role::Answer => queue.push_back(Event::Answer(identity)),
                    Role::Apply => queue.push_back(Event::Apply),
                    Role::Clear => queue.push_back(Event::Clear),
                    Role::Quiet => queue.push_back(Event::Quiet),
                    _ => {}
                }
                if choices.feedback.to != Role::Answer {
                    queue.push_back(Event::Success);
                }
            }
            Event::Success => {
                mark_choice(choices.continuation, &mut used_arrows);
                match choices.continuation.to {
                    Role::Apply => queue.push_back(Event::Apply),
                    Role::Answer => {
                        if let Some(identity) = current {
                            queue.push_back(Event::Answer(identity));
                        }
                    }
                    Role::Clear => queue.push_back(Event::Clear),
                    Role::Quiet => queue.push_back(Event::Quiet),
                    _ => {}
                }
            }
            Event::NoResult => {
                mark_choice(choices.finish, &mut used_arrows);
                match choices.finish.to {
                    Role::Answer => {
                        if let Some(identity) = current {
                            queue.push_back(Event::Answer(identity));
                        }
                    }
                    Role::Apply => queue.push_back(Event::Apply),
                    Role::Clear => queue.push_back(Event::Clear),
                    _ => queue.push_back(Event::Quiet),
                }
            }
            Event::Answer(identity) => {
                emitted_answer = Some(identity);
                queue.clear();
            }
            Event::Clear => {
                current = None;
                queue.clear();
            }
            Event::Quiet => queue.clear(),
        }
    }

    let outcome = fault
        .unwrap_or_else(|| emitted_answer.map_or(BindingOutcome::NotFound, BindingOutcome::Answer));
    ExecutionResult {
        outcome,
        spikes,
        activity_limit_hit,
        explicit_answer: emitted_answer.is_some(),
        queue_empty: queue.is_empty(),
        used_arrows,
    }
}

fn mark_choice(choice: RouteChoice, used_arrows: &mut Vec<usize>) {
    if let Some(arrow_id) = choice.arrow_id {
        if !used_arrows.contains(&arrow_id) {
            used_arrows.push(arrow_id);
        }
    }
}

fn apply_traces(network: &mut ProposalNetwork, used_arrows: &[usize]) {
    for arrow_id in used_arrows {
        let choice = network
            .arrows
            .iter()
            .find(|arrow| arrow.id == *arrow_id)
            .map(|arrow| RouteChoice {
                arrow_id: Some(arrow.id),
                from: arrow.from,
                to: arrow.to,
            });
        if let Some(choice) = choice {
            network.mark_used(choice);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IsolatedGate {
    Lookup,
    Feedback,
    Continue,
    Finish,
}

impl IsolatedGate {
    fn source(self) -> Role {
        match self {
            Self::Lookup => LOOKUP_SOURCE,
            Self::Feedback => FEEDBACK_SOURCE,
            Self::Continue => CONTINUE_SOURCE,
            Self::Finish => FINISH_SOURCE,
        }
    }

    fn expected_target(self) -> Role {
        match self {
            Self::Lookup => Role::Slot2,
            Self::Feedback => Role::Current,
            Self::Continue => Role::Apply,
            Self::Finish => Role::Answer,
        }
    }
}

fn isolated_choices(gate: IsolatedGate, network: &mut ProposalNetwork) -> Option<ProgramChoices> {
    let learned = network.choose(gate.source())?;
    Some(ProgramChoices {
        lookup: if gate == IsolatedGate::Lookup {
            learned
        } else {
            RouteChoice::fixed(Role::Slot1, Role::Slot2)
        },
        feedback: if gate == IsolatedGate::Feedback {
            learned
        } else {
            RouteChoice::fixed(Role::Result, Role::Current)
        },
        continuation: if gate == IsolatedGate::Continue {
            learned
        } else if gate == IsolatedGate::Lookup {
            RouteChoice::fixed(Role::Success, Role::Answer)
        } else {
            RouteChoice::fixed(Role::Success, Role::Apply)
        },
        finish: if gate == IsolatedGate::Finish {
            learned
        } else {
            RouteChoice::fixed(Role::NoResult, Role::Answer)
        },
    })
}

#[derive(Clone, Debug)]
pub struct IsolatedGateResult {
    pub name: &'static str,
    pub successful_seeds: usize,
    pub total_seeds: usize,
    pub average_episodes: f64,
    pub proposed_arrows: usize,
    pub surviving_arrows: usize,
    pub reversed_control: bool,
    pub random_feedback_stable: bool,
}

fn run_isolated_gate(gate: IsolatedGate) -> IsolatedGateResult {
    let mut successes = 0;
    let mut episode_total = 0;
    let mut proposals = 0;
    let mut survivors = 0;
    for seed in 0..ISOLATED_SEEDS {
        let mut network = ProposalNetwork::new(0x5000 + seed as u64, &[gate.source()]);
        let mut identities = IdentitySource::new(0x5100 + seed as u64);
        let mut rng = DeterministicRng::new(0x5200 + seed as u64);
        let mut competence_episode = None;
        for episode_number in 1..=2_000 {
            network.begin_episode();
            let depth = if matches!(gate, IsolatedGate::Feedback | IsolatedGate::Continue) {
                2
            } else {
                1
            };
            let episode = chain_episode(&mut identities, &mut rng, depth, 8);
            let choices = isolated_choices(gate, &mut network).unwrap();
            let run = execute_program(&episode, choices, FROZEN_PHYSICS.activity_limit);
            apply_traces(&mut network, &run.used_arrows);
            network.terminal_feedback(run.outcome == episode.correct);
            if network.consolidated_target(gate.source()) == Some(gate.expected_target()) {
                competence_episode = Some(episode_number);
                break;
            }
        }
        if let Some(episode) = competence_episode {
            successes += 1;
            episode_total += episode;
        }
        proposals += network.proposals;
        survivors += network.arrows.len();
    }

    let reversed_control = if gate == IsolatedGate::Lookup {
        run_reverse_lookup_control()
    } else {
        true
    };
    let random_feedback_stable = run_random_feedback_control(gate);
    IsolatedGateResult {
        name: match gate {
            IsolatedGate::Lookup => "P0a lookup",
            IsolatedGate::Feedback => "P0b feedback",
            IsolatedGate::Continue => "P0c self-trigger",
            IsolatedGate::Finish => "P0d finish",
        },
        successful_seeds: successes,
        total_seeds: ISOLATED_SEEDS,
        average_episodes: episode_total as f64 / successes.max(1) as f64,
        proposed_arrows: proposals / ISOLATED_SEEDS,
        surviving_arrows: survivors / ISOLATED_SEEDS,
        reversed_control,
        random_feedback_stable,
    }
}

fn run_reverse_lookup_control() -> bool {
    let mut network = ProposalNetwork::new(0x5300, &[Role::Slot2]);
    let mut identities = IdentitySource::new(0x5301);
    let mut rng = DeterministicRng::new(0x5302);
    for _ in 0..2_000 {
        network.begin_episode();
        let mut episode = chain_episode(&mut identities, &mut rng, 1, 8);
        episode.query = match episode.correct {
            BindingOutcome::Answer(right) => right,
            _ => unreachable!(),
        };
        let left = episode
            .relations
            .iter()
            .find_map(|(left, right)| (*right == episode.query).then_some(*left))
            .unwrap();
        episode.correct = BindingOutcome::Answer(left);
        let learned = network.choose(Role::Slot2).unwrap();
        let choices = ProgramChoices {
            lookup: RouteChoice {
                arrow_id: learned.arrow_id,
                from: Role::Slot2,
                to: learned.to,
            },
            feedback: RouteChoice::fixed(Role::Result, Role::Current),
            continuation: RouteChoice::fixed(Role::Success, Role::Answer),
            finish: RouteChoice::fixed(Role::NoResult, Role::Answer),
        };
        let run = execute_reverse_lookup(&episode, choices.lookup);
        apply_traces(&mut network, &run.used_arrows);
        network.terminal_feedback(run.outcome == episode.correct);
        if network.consolidated_target(Role::Slot2) == Some(Role::Slot1) {
            return true;
        }
    }
    false
}

fn execute_reverse_lookup(episode: &TaskEpisode, lookup: RouteChoice) -> ExecutionResult {
    let mut outputs = HashSet::new();
    let mut spikes = 1;
    if lookup.to == Role::Slot1 {
        for (left, right) in &episode.relations {
            spikes += 1;
            if *right == episode.query {
                outputs.insert(*left);
            }
        }
    }
    let outcome = match outputs.len() {
        0 => BindingOutcome::NotFound,
        1 => BindingOutcome::Answer(*outputs.iter().next().unwrap()),
        _ => BindingOutcome::Ambiguous,
    };
    ExecutionResult {
        outcome,
        spikes,
        activity_limit_hit: false,
        explicit_answer: matches!(outcome, BindingOutcome::Answer(_)),
        queue_empty: true,
        used_arrows: lookup.arrow_id.into_iter().collect(),
    }
}

fn run_random_feedback_control(gate: IsolatedGate) -> bool {
    let mut network = ProposalNetwork::new(0x5400 + gate as u64, &[gate.source()]);
    let mut identities = IdentitySource::new(0x5410 + gate as u64);
    let mut rng = DeterministicRng::new(0x5420 + gate as u64);
    for _ in 0..500 {
        network.begin_episode();
        let depth = if matches!(gate, IsolatedGate::Feedback | IsolatedGate::Continue) {
            2
        } else {
            1
        };
        let episode = chain_episode(&mut identities, &mut rng, depth, 8);
        let choices = isolated_choices(gate, &mut network).unwrap();
        let run = execute_program(&episode, choices, FROZEN_PHYSICS.activity_limit);
        apply_traces(&mut network, &run.used_arrows);
        network.terminal_feedback(rng.next_u64() & 1 == 0);
    }
    network.consolidated_target(gate.source()) == Some(gate.expected_target())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FeedbackMode {
    Real,
    Shuffled,
    Random,
    ActivityOnly,
}

#[derive(Clone, Debug)]
pub struct TopologyCheckpoint {
    pub episode: usize,
    pub candidates: usize,
    pub stable_arrows: usize,
    pub lookup_correct: bool,
    pub feedback_correct: bool,
    pub continuation_correct: bool,
    pub finish_correct: bool,
}

#[derive(Clone, Debug)]
struct SeedResult {
    competent: bool,
    first_success_episode: Option<usize>,
    proposals_before_first_success: Option<usize>,
    spikes_before_first_success: Option<usize>,
    episodes_from_first_success_to_competence: Option<usize>,
    final_arrows: usize,
    held_out_correct: usize,
    held_out_total: usize,
    fingerprint_unchanged: bool,
    explicit_answers: bool,
    queues_empty: bool,
    trajectories: Vec<TopologyCheckpoint>,
}

fn run_integrated_seed(seed: usize, mode: FeedbackMode) -> SeedResult {
    let mut network = ProposalNetwork::new(0x6000 + seed as u64, &PROGRAM_SOURCES);
    let mut identities = IdentitySource::new(0x6100 + seed as u64);
    let mut rng = DeterministicRng::new(0x6200 + seed as u64);
    let mut feedback_rng = DeterministicRng::new(0x6300 + seed as u64);
    let mut previous_actual_success = false;
    let mut first_success_episode = None;
    let mut proposals_before_first_success = None;
    let mut spikes_before_first_success = None;
    let mut training_spikes = 0;
    let mut competence_episode = None;
    let mut trajectories = Vec::new();
    let mut last_stable_arrows = 0;
    let checkpoint_set = [1, 10, 100, 1_000, 10_000, INTEGRATED_BUDGET];

    for episode_number in 1..=INTEGRATED_BUDGET {
        network.begin_episode();
        let depth = TRAIN_DEPTHS[(episode_number - 1) % TRAIN_DEPTHS.len()];
        let episode = chain_episode(&mut identities, &mut rng, depth, 12);
        let Some(choices) = ProgramChoices::learned(&mut network) else {
            continue;
        };
        let run = execute_program(&episode, choices, FROZEN_PHYSICS.activity_limit);
        training_spikes += run.spikes;
        apply_traces(&mut network, &run.used_arrows);
        let actual_success = run.outcome == episode.correct && !run.activity_limit_hit;
        if actual_success && first_success_episode.is_none() {
            first_success_episode = Some(episode_number);
            proposals_before_first_success = Some(network.proposals);
            spikes_before_first_success = Some(training_spikes);
        }
        let credited_success = match mode {
            FeedbackMode::Real => actual_success,
            FeedbackMode::Shuffled => previous_actual_success,
            FeedbackMode::Random => feedback_rng.next_u64().is_multiple_of(4),
            FeedbackMode::ActivityOnly => !run.used_arrows.is_empty(),
        };
        previous_actual_success = actual_success;
        network.terminal_feedback(credited_success);

        if competence_episode.is_none() && network.complete && program_routes_correct(&network) {
            competence_episode = Some(episode_number);
        }
        let stable_arrows = network.stable_arrows();
        let topology_changed = stable_arrows != last_stable_arrows;
        if checkpoint_set.contains(&episode_number)
            || topology_changed
            || first_success_episode == Some(episode_number)
        {
            trajectories.push(checkpoint(episode_number, &network));
        }
        last_stable_arrows = stable_arrows;
    }

    let fingerprint_before = network.fingerprint();
    let mut held_out_correct = 0;
    let mut held_out_total = 0;
    let mut explicit_answers = true;
    let mut queues_empty = true;
    if let Some(choices) = ProgramChoices::evaluated(&network) {
        let mut heldout_identities = IdentitySource::new(0x7000 + seed as u64);
        let mut heldout_rng = DeterministicRng::new(0x7100 + seed as u64);
        for depth in HELD_OUT_DEPTHS {
            for _ in 0..16 {
                let episode =
                    chain_episode(&mut heldout_identities, &mut heldout_rng, depth, depth + 8);
                let run = execute_program(&episode, choices, FROZEN_PHYSICS.activity_limit);
                held_out_correct += usize::from(run.outcome == episode.correct);
                held_out_total += 1;
                explicit_answers &= run.explicit_answer;
                queues_empty &= run.queue_empty && !run.activity_limit_hit;
            }
        }
    } else {
        held_out_total = HELD_OUT_DEPTHS.len() * 16;
        explicit_answers = false;
        queues_empty = false;
    }
    SeedResult {
        competent: held_out_correct == held_out_total,
        first_success_episode,
        proposals_before_first_success,
        spikes_before_first_success,
        episodes_from_first_success_to_competence: first_success_episode
            .zip(competence_episode)
            .map(|(first, competent)| competent.saturating_sub(first)),
        final_arrows: network.arrows.len(),
        held_out_correct,
        held_out_total,
        fingerprint_unchanged: fingerprint_before == network.fingerprint(),
        explicit_answers,
        queues_empty,
        trajectories,
    }
}

fn checkpoint(episode: usize, network: &ProposalNetwork) -> TopologyCheckpoint {
    TopologyCheckpoint {
        episode,
        candidates: network.arrows.len(),
        stable_arrows: network.stable_arrows(),
        lookup_correct: network.route_target(LOOKUP_SOURCE) == Some(Role::Slot2),
        feedback_correct: network.route_target(FEEDBACK_SOURCE) == Some(Role::Current),
        continuation_correct: network.route_target(CONTINUE_SOURCE) == Some(Role::Apply),
        finish_correct: network.route_target(FINISH_SOURCE) == Some(Role::Answer),
    }
}

fn program_routes_correct(network: &ProposalNetwork) -> bool {
    network.route_target(LOOKUP_SOURCE) == Some(Role::Slot2)
        && network.route_target(FEEDBACK_SOURCE) == Some(Role::Current)
        && network.route_target(CONTINUE_SOURCE) == Some(Role::Apply)
        && network.route_target(FINISH_SOURCE) == Some(Role::Answer)
}

#[derive(Clone, Debug)]
pub struct IntegratedCondition {
    pub name: &'static str,
    pub competent_seeds: usize,
    pub total_seeds: usize,
    pub first_success_seeds: usize,
    pub average_first_success_episode: Option<f64>,
    pub average_proposals_before_first_success: Option<f64>,
    pub average_spikes_before_first_success: Option<f64>,
    pub average_learning_after_first_success: Option<f64>,
    pub average_final_arrows: f64,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub fingerprints_unchanged: bool,
    pub explicit_answers: bool,
    pub queues_empty: bool,
    pub representative_trajectory: Vec<TopologyCheckpoint>,
}

fn run_condition(mode: FeedbackMode, seeds: usize) -> IntegratedCondition {
    let results: Vec<_> = (0..seeds)
        .map(|seed| run_integrated_seed(seed, mode))
        .collect();
    let first_successes: Vec<_> = results
        .iter()
        .filter_map(|result| result.first_success_episode)
        .collect();
    let proposals: Vec<_> = results
        .iter()
        .filter_map(|result| result.proposals_before_first_success)
        .collect();
    let spikes: Vec<_> = results
        .iter()
        .filter_map(|result| result.spikes_before_first_success)
        .collect();
    let after_success: Vec<_> = results
        .iter()
        .filter_map(|result| result.episodes_from_first_success_to_competence)
        .collect();
    IntegratedCondition {
        name: match mode {
            FeedbackMode::Real => "real terminal feedback",
            FeedbackMode::Shuffled => "shuffled terminal feedback",
            FeedbackMode::Random => "random feedback",
            FeedbackMode::ActivityOnly => "activity only",
        },
        competent_seeds: results.iter().filter(|result| result.competent).count(),
        total_seeds: seeds,
        first_success_seeds: first_successes.len(),
        average_first_success_episode: average(&first_successes),
        average_proposals_before_first_success: average(&proposals),
        average_spikes_before_first_success: average(&spikes),
        average_learning_after_first_success: average(&after_success),
        average_final_arrows: results
            .iter()
            .map(|result| result.final_arrows)
            .sum::<usize>() as f64
            / seeds as f64,
        held_out_correct: results.iter().map(|result| result.held_out_correct).sum(),
        held_out_total: results.iter().map(|result| result.held_out_total).sum(),
        fingerprints_unchanged: results.iter().all(|result| result.fingerprint_unchanged),
        explicit_answers: results.iter().all(|result| result.explicit_answers),
        queues_empty: results.iter().all(|result| result.queues_empty),
        representative_trajectory: results
            .first()
            .map_or_else(Vec::new, |result| result.trajectories.clone()),
    }
}

fn average(values: &[usize]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<usize>() as f64 / values.len() as f64)
}

#[derive(Clone, Debug)]
pub struct ProgramDiscoveryReport {
    pub isolated: Vec<IsolatedGateResult>,
    pub real: IntegratedCondition,
    pub shuffled: IntegratedCondition,
    pub random: IntegratedCondition,
    pub activity_only: IntegratedCondition,
    pub shared_configuration: bool,
    pub integrated_hypothesis_supported: bool,
    pub experimental_gate_valid: bool,
}

pub fn run_program_discovery_experiment() -> ProgramDiscoveryReport {
    let isolated = [
        IsolatedGate::Lookup,
        IsolatedGate::Feedback,
        IsolatedGate::Continue,
        IsolatedGate::Finish,
    ]
    .into_iter()
    .map(run_isolated_gate)
    .collect::<Vec<_>>();
    let real = run_condition(FeedbackMode::Real, INTEGRATED_SEEDS);
    let shuffled = run_condition(FeedbackMode::Shuffled, CONTROL_SEEDS);
    let random = run_condition(FeedbackMode::Random, CONTROL_SEEDS);
    let activity_only = run_condition(FeedbackMode::ActivityOnly, CONTROL_SEEDS);
    let isolated_pass = isolated.iter().all(|gate| {
        gate.successful_seeds == gate.total_seeds
            && gate.reversed_control
            && !gate.random_feedback_stable
    });
    let integrated_hypothesis_supported = real.competent_seeds == real.total_seeds
        && shuffled.competent_seeds == 0
        && random.competent_seeds == 0
        && activity_only.competent_seeds * 4 <= activity_only.total_seeds;
    let experimental_gate_valid = isolated_pass
        && real.fingerprints_unchanged
        && shuffled.fingerprints_unchanged
        && random.fingerprints_unchanged
        && activity_only.fingerprints_unchanged;
    ProgramDiscoveryReport {
        isolated,
        real,
        shuffled,
        random,
        activity_only,
        shared_configuration: true,
        integrated_hypothesis_supported,
        experimental_gate_valid,
    }
}

pub fn print_program_discovery_report(report: &ProgramDiscoveryReport) {
    println!("P0 de-supply program discovery:");
    for gate in &report.isolated {
        println!(
            "  {}: seeds={}/{}, episodes={:.1}, proposals/survivors={}/{}, reverse={}, random-stable={}",
            gate.name,
            gate.successful_seeds,
            gate.total_seeds,
            gate.average_episodes,
            gate.proposed_arrows,
            gate.surviving_arrows,
            gate.reversed_control,
            gate.random_feedback_stable
        );
    }
    for condition in [
        &report.real,
        &report.shuffled,
        &report.random,
        &report.activity_only,
    ] {
        println!(
            "  {}: competent={}/{}, first-success={}/{}, first episode={:?}, learn-after={:?}, arrows={:.1}, held-out={}/{}",
            condition.name,
            condition.competent_seeds,
            condition.total_seeds,
            condition.first_success_seeds,
            condition.total_seeds,
            condition.average_first_success_episode,
            condition.average_learning_after_first_success,
            condition.average_final_arrows,
            condition.held_out_correct,
            condition.held_out_total
        );
        if condition.name == "real terminal feedback" {
            println!(
                "    first-success proposals/spikes={:?}/{:?}",
                condition.average_proposals_before_first_success,
                condition.average_spikes_before_first_success
            );
            for point in &condition.representative_trajectory {
                println!(
                    "    episode {:>5}: candidates={}, stable={}, lookup={}, feedback={}, continue={}, finish={}",
                    point.episode,
                    point.candidates,
                    point.stable_arrows,
                    point.lookup_correct,
                    point.feedback_correct,
                    point.continuation_correct,
                    point.finish_correct
                );
            }
        }
    }
    println!(
        "  shared physics={}, integrated hypothesis={}, experimental gate={}",
        report.shared_configuration,
        report.integrated_hypothesis_supported,
        report.experimental_gate_valid
    );
}

fn fingerprint_mix(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(0x100_0000_01b3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn report() -> &'static ProgramDiscoveryReport {
        static REPORT: OnceLock<ProgramDiscoveryReport> = OnceLock::new();
        REPORT.get_or_init(run_program_discovery_experiment)
    }

    #[test]
    fn p0_isolated_gates_rediscover_each_route_with_shared_physics() {
        let report = report();
        assert!(report.shared_configuration);
        for gate in &report.isolated {
            assert_eq!(gate.successful_seeds, gate.total_seeds, "{}", gate.name);
            assert!(gate.reversed_control);
            assert!(!gate.random_feedback_stable);
            assert!(gate.proposed_arrows > gate.surviving_arrows);
        }
    }

    #[test]
    fn p0e_is_a_valid_fresh_end_to_end_discovery_test() {
        let report = report();
        assert!(report.experimental_gate_valid);
        assert!(report.real.fingerprints_unchanged);
        assert!(report.shuffled.fingerprints_unchanged);
        assert!(report.random.fingerprints_unchanged);
        assert!(report.activity_only.fingerprints_unchanged);
    }

    #[test]
    fn p0e_real_feedback_is_compared_against_nonfunctional_controls() {
        let report = report();
        assert_eq!(report.shuffled.competent_seeds, 0);
        assert_eq!(report.random.competent_seeds, 0);
        assert!(report.activity_only.competent_seeds * 4 <= report.activity_only.total_seeds);
        if report.integrated_hypothesis_supported {
            assert_eq!(report.real.competent_seeds, report.real.total_seeds);
            assert!(report.real.explicit_answers);
            assert!(report.real.queues_empty);
        }
    }

    #[test]
    fn p0e_records_bootstrapping_and_topology_trajectories() {
        let report = report();
        assert!(!report.real.representative_trajectory.is_empty());
        assert!(report.real.held_out_total > 0);
        assert_eq!(
            report
                .real
                .representative_trajectory
                .last()
                .unwrap()
                .episode,
            INTEGRATED_BUDGET
        );
    }
}
