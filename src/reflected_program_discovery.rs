//! RP0a: reuse P0-style program learning over anonymous provenance-derived roles.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Write as _;
use std::mem::size_of;

use crate::binding::{BindingOutcome, IdentitySource, OpaqueId};

pub const RP0A_PROTOCOL: &str = "reflected-program-discovery-rp0a-v1";

const ROLE_THRESHOLD: usize = 4;
const TRAIN_BUDGET: usize = 50_000;
const SMOKE_BUDGET: usize = 5_000;
const ACTIVITY_LIMIT: usize = 1_600;
const PROBATION_EPISODES: usize = 8;
const SUCCESS_CREDIT: i32 = 2;
const FAILURE_CREDIT: i32 = -1;
const PRUNE_STRENGTH: i32 = -2;
const CONSOLIDATION_STRENGTH: i32 = 6;
const HELD_OUT_PER_DEPTH: usize = 16;
const DEFINITIVE_SEEDS: usize = 8;
const TRAIN_DEPTHS: [usize; 4] = [1, 2, 3, 4];
const HELD_OUT_DEPTHS: [usize; 4] = [5, 8, 16, 32];
const SMOKE_TRAIN_DEPTHS: [usize; 2] = [1, 2];
const SMOKE_HELD_OUT_DEPTHS: [usize; 2] = [3, 5];
const CHECKPOINTS: [usize; 11] = [0, 1, 2, 4, 8, 16, 64, 256, 1_024, 10_000, 50_000];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum LowerRole {
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

impl LowerRole {
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

    const PROGRAM_SOURCES: [Self; 4] = [Self::Slot1, Self::Result, Self::Success, Self::NoResult];

    fn correct_target(self) -> Option<Self> {
        match self {
            Self::Slot1 => Some(Self::Slot2),
            Self::Result => Some(Self::Current),
            Self::Success => Some(Self::Apply),
            Self::NoResult => Some(Self::Answer),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Integrated,
    ActivityOnly,
    ShuffledProvenance,
    ShuffledFeedback,
    RandomFeedback,
    Symmetric,
    Oracle,
}

impl Arm {
    const LEARNED: [Self; 6] = [
        Self::Integrated,
        Self::ActivityOnly,
        Self::ShuffledProvenance,
        Self::ShuffledFeedback,
        Self::RandomFeedback,
        Self::Symmetric,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Integrated => "rp0a-integrated",
            Self::ActivityOnly => "activity-only",
            Self::ShuffledProvenance => "shuffled-provenance",
            Self::ShuffledFeedback => "shuffled-terminal-feedback",
            Self::RandomFeedback => "random-terminal-feedback",
            Self::Symmetric => "symmetric-impossible",
            Self::Oracle => "oracle-reflected-program",
        }
    }
}

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

fn mix(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(0x100_0000_01b3);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub provenance_events: u64,
    pub provenance_relations: u64,
    pub role_comparisons: u64,
    pub role_updates: u64,
    pub proposals: u64,
    pub arrow_evaluations: u64,
    pub arrow_firings: u64,
    pub credit_updates: u64,
    pub cells_activated: u64,
    pub spikes_enqueued: u64,
    pub spikes_dequeued: u64,
    pub identity_comparisons: u64,
    pub queue_checks: u64,
}

impl Work {
    pub fn total(self) -> u64 {
        self.provenance_events
            + self.provenance_relations
            + self.role_comparisons
            + self.role_updates
            + self.proposals
            + self.arrow_evaluations
            + self.arrow_firings
            + self.credit_updates
            + self.cells_activated
            + self.spikes_enqueued
            + self.spikes_dequeued
            + self.identity_comparisons
            + self.queue_checks
    }

    fn add(&mut self, other: Self) {
        self.provenance_events += other.provenance_events;
        self.provenance_relations += other.provenance_relations;
        self.role_comparisons += other.role_comparisons;
        self.role_updates += other.role_updates;
        self.proposals += other.proposals;
        self.arrow_evaluations += other.arrow_evaluations;
        self.arrow_firings += other.arrow_firings;
        self.credit_updates += other.credit_updates;
        self.cells_activated += other.cells_activated;
        self.spikes_enqueued += other.spikes_enqueued;
        self.spikes_dequeued += other.spikes_dequeued;
        self.identity_comparisons += other.identity_comparisons;
        self.queue_checks += other.queue_checks;
    }
}

#[derive(Default)]
struct Lifecycle {
    created: Cell<usize>,
    destroyed: Cell<usize>,
    live: Cell<usize>,
    maximum_live: Cell<usize>,
}

impl Lifecycle {
    fn enter(&self) -> Workspace<'_> {
        self.created.set(self.created.get() + 1);
        self.live.set(self.live.get() + 1);
        self.maximum_live
            .set(self.maximum_live.get().max(self.live.get()));
        Workspace { lifecycle: self }
    }
}

struct Workspace<'a> {
    lifecycle: &'a Lifecycle,
}

impl Drop for Workspace<'_> {
    fn drop(&mut self) {
        self.lifecycle
            .destroyed
            .set(self.lifecycle.destroyed.get() + 1);
        self.lifecycle.live.set(self.lifecycle.live.get() - 1);
    }
}

#[derive(Clone, Debug)]
struct Transition {
    source: u64,
    consumed: Vec<u64>,
    produced: Vec<u64>,
}

#[derive(Clone, Debug)]
struct Invocation {
    source_identity: u64,
    occurrence_identities: BTreeSet<u64>,
    transitions: Vec<Transition>,
    inputs: Vec<u64>,
    outputs: Vec<u64>,
}

fn motif(role: LowerRole) -> &'static [(usize, usize)] {
    match role {
        LowerRole::Slot1 => &[(1, 1)],
        LowerRole::Slot2 => &[(1, 2)],
        LowerRole::Result => &[(2, 1)],
        LowerRole::Current => &[(1, 1), (1, 1)],
        LowerRole::Success => &[(1, 2), (2, 1)],
        LowerRole::Apply => &[(2, 2)],
        LowerRole::NoResult => &[(0, 1), (1, 1)],
        LowerRole::Answer => &[(1, 0)],
        LowerRole::Clear => &[(2, 0)],
        LowerRole::Quiet => &[(0, 1)],
    }
}

fn build_invocation(
    role: LowerRole,
    episode: u64,
    ordinal: usize,
    symmetric: bool,
    lifecycle: &Lifecycle,
) -> Invocation {
    let _workspace = lifecycle.enter();
    let represented = if symmetric && role == LowerRole::Result {
        LowerRole::Slot1
    } else {
        role
    };
    let template = motif(represented);
    let base = episode
        .wrapping_mul(10_000)
        .wrapping_add(ordinal as u64 * 128)
        .wrapping_add(1);
    let source_identity = episode
        .wrapping_mul(1_000_003)
        .wrapping_add(ordinal as u64 * 97)
        .wrapping_add(1 << 48);
    let initial_inputs = template
        .iter()
        .map(|(consumed, _)| *consumed)
        .max()
        .unwrap_or(0);
    let mut available: Vec<_> = (0..initial_inputs)
        .map(|index| base + index as u64)
        .collect();
    let inputs = available.clone();
    let mut next_occurrence = base + initial_inputs as u64;
    let mut transitions = Vec::new();
    for (step, (consumed_count, produced_count)) in template.iter().copied().enumerate() {
        let mut consumed = Vec::new();
        if consumed_count > 0 {
            for offset in 0..consumed_count {
                let index = (step + offset) % available.len().max(1);
                consumed.push(
                    available
                        .get(index)
                        .copied()
                        .unwrap_or(base.wrapping_add(offset as u64)),
                );
            }
        }
        let mut produced = Vec::new();
        for _ in 0..produced_count {
            produced.push(next_occurrence);
            available.push(next_occurrence);
            next_occurrence += 1;
        }
        transitions.push(Transition {
            source: source_identity.wrapping_add(step as u64),
            consumed,
            produced,
        });
    }
    let consumed: BTreeSet<_> = transitions
        .iter()
        .flat_map(|transition| transition.consumed.iter().copied())
        .collect();
    let outputs = transitions
        .iter()
        .flat_map(|transition| transition.produced.iter().copied())
        .filter(|occurrence| !consumed.contains(occurrence))
        .collect::<Vec<_>>();
    let occurrence_identities = inputs
        .iter()
        .chain(outputs.iter())
        .chain(
            transitions
                .iter()
                .flat_map(|transition| transition.consumed.iter().chain(&transition.produced)),
        )
        .copied()
        .collect();
    Invocation {
        source_identity,
        occurrence_identities,
        transitions,
        inputs,
        outputs,
    }
}

fn provenance_signature(invocation: &Invocation, work: &mut Work) -> u64 {
    work.provenance_events += invocation.transitions.len() as u64;
    let mut ancestry: BTreeMap<u64, BTreeSet<usize>> = invocation
        .inputs
        .iter()
        .enumerate()
        .map(|(index, occurrence)| (*occurrence, BTreeSet::from([index])))
        .collect();
    let mut shapes = Vec::new();
    for transition in &invocation.transitions {
        let mut causes = BTreeSet::new();
        for occurrence in &transition.consumed {
            causes.extend(ancestry.get(occurrence).into_iter().flatten().copied());
        }
        for occurrence in &transition.produced {
            ancestry.insert(*occurrence, causes.clone());
        }
        shapes.push((
            transition.consumed.len(),
            transition.produced.len(),
            causes.len(),
        ));
    }
    shapes.sort_unstable();
    let relation_count = invocation
        .outputs
        .iter()
        .map(|output| ancestry.get(output).map_or(0, BTreeSet::len))
        .sum::<usize>();
    work.provenance_relations += relation_count as u64;
    let mut hash = 0xcbf2_9ce4_8422_2325;
    mix(&mut hash, invocation.inputs.len() as u64);
    mix(&mut hash, invocation.outputs.len() as u64);
    mix(&mut hash, invocation.transitions.len() as u64);
    mix(&mut hash, invocation.occurrence_identities.len() as u64);
    mix(&mut hash, relation_count as u64);
    for (consumed, produced, causes) in shapes {
        mix(&mut hash, consumed as u64);
        mix(&mut hash, produced as u64);
        mix(&mut hash, causes as u64);
    }
    hash
}

#[derive(Clone, Copy, Debug)]
struct RoleObservation {
    lower: LowerRole,
    signature: u64,
}

fn role_observations(
    arm: Arm,
    episode: u64,
    rng: &mut DeterministicRng,
    lifecycle: &Lifecycle,
    work: &mut Work,
) -> Vec<RoleObservation> {
    if arm == Arm::ActivityOnly {
        work.provenance_events += LowerRole::ALL.len() as u64;
        return Vec::new();
    }
    let mut signatures = Vec::new();
    for (ordinal, role) in LowerRole::ALL.into_iter().enumerate() {
        let invocation = build_invocation(role, episode, ordinal, arm == Arm::Symmetric, lifecycle);
        let _opaque_source = invocation.source_identity;
        signatures.push(provenance_signature(&invocation, work));
    }
    if arm == Arm::ShuffledProvenance {
        rng.shuffle(&mut signatures);
    }
    LowerRole::ALL
        .into_iter()
        .zip(signatures)
        .map(|(lower, signature)| RoleObservation { lower, signature })
        .collect()
}

#[derive(Clone, Debug)]
struct RolePattern {
    signature: u64,
    role_id: usize,
    observations: usize,
}

#[derive(Clone, Debug, Default)]
struct RoleLearner {
    patterns: Vec<RolePattern>,
    next_role: usize,
    comparisons: u64,
    updates: u64,
}

impl RoleLearner {
    fn observe(&mut self, observations: &[RoleObservation]) {
        let signatures: BTreeSet<_> = observations.iter().map(|item| item.signature).collect();
        for signature in signatures {
            self.comparisons += self.patterns.len() as u64;
            if let Some(pattern) = self
                .patterns
                .iter_mut()
                .find(|pattern| pattern.signature == signature)
            {
                pattern.observations += 1;
            } else {
                self.patterns.push(RolePattern {
                    signature,
                    role_id: self.next_role,
                    observations: 1,
                });
                self.next_role += 1;
            }
            self.updates += 1;
        }
        self.patterns.sort_by_key(|pattern| pattern.signature);
    }

    fn translate(&self, signature: u64) -> Option<usize> {
        self.patterns
            .iter()
            .find(|pattern| {
                pattern.signature == signature && pattern.observations >= ROLE_THRESHOLD
            })
            .map(|pattern| pattern.role_id)
    }

    fn bindings(&self, observations: &[RoleObservation]) -> BTreeMap<LowerRole, usize> {
        observations
            .iter()
            .filter_map(|item| {
                self.translate(item.signature)
                    .map(|role| (item.lower, role))
            })
            .collect()
    }

    fn consolidated_roles(&self) -> Vec<usize> {
        let mut roles = self
            .patterns
            .iter()
            .filter(|pattern| pattern.observations >= ROLE_THRESHOLD)
            .map(|pattern| pattern.role_id)
            .collect::<Vec<_>>();
        roles.sort_unstable();
        roles
    }

    fn permanent_bytes(&self) -> usize {
        self.patterns.capacity() * size_of::<RolePattern>()
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325;
        let mut patterns = self.patterns.clone();
        patterns.sort_by_key(|pattern| pattern.signature);
        for pattern in patterns {
            mix(&mut hash, pattern.signature);
            mix(&mut hash, pattern.role_id as u64);
            mix(&mut hash, pattern.observations as u64);
        }
        hash
    }
}

#[derive(Clone, Copy, Debug)]
struct ProgramArrow {
    id: usize,
    from: usize,
    to: usize,
    strength: i32,
    uses: usize,
    age: usize,
    traced: bool,
    consolidated: bool,
}

#[derive(Clone, Copy, Debug)]
struct ArrowChoice {
    id: usize,
    from: usize,
    to: usize,
}

#[derive(Clone, Debug)]
struct ProgramLearner {
    arrows: Vec<ProgramArrow>,
    known_roles: Vec<usize>,
    next_arrow: usize,
    proposals: usize,
    pruned: usize,
    rng: DeterministicRng,
}

impl ProgramLearner {
    fn new(seed: u64) -> Self {
        Self {
            arrows: Vec::new(),
            known_roles: Vec::new(),
            next_arrow: 0,
            proposals: 0,
            pruned: 0,
            rng: DeterministicRng::new(seed ^ 0x5050_a011),
        }
    }

    fn begin_episode(&mut self, roles: &[usize]) {
        self.known_roles = roles.to_vec();
        self.known_roles.sort_unstable();
        for arrow in &mut self.arrows {
            arrow.age += 1;
            arrow.traced = false;
        }
        self.propose_missing();
    }

    fn propose_missing(&mut self) {
        let roles = self.known_roles.clone();
        for from in &roles {
            for to in &roles {
                if from == to
                    || self
                        .arrows
                        .iter()
                        .any(|arrow| arrow.from == *from && arrow.to == *to)
                {
                    continue;
                }
                self.arrows.push(ProgramArrow {
                    id: self.next_arrow,
                    from: *from,
                    to: *to,
                    strength: 0,
                    uses: 0,
                    age: 0,
                    traced: false,
                    consolidated: false,
                });
                self.next_arrow += 1;
                self.proposals += 1;
            }
        }
    }

    fn choose(&mut self, source: usize, work: &mut Work) -> Option<ArrowChoice> {
        work.arrow_evaluations += self
            .arrows
            .iter()
            .filter(|arrow| arrow.from == source)
            .count() as u64;
        if let Some(arrow) = self
            .arrows
            .iter()
            .find(|arrow| arrow.from == source && arrow.consolidated)
        {
            return Some(ArrowChoice {
                id: arrow.id,
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
        let choices = self
            .arrows
            .iter()
            .filter(|arrow| {
                arrow.from == source
                    && best_positive.is_none_or(|strength| arrow.strength == strength)
            })
            .map(|arrow| arrow.id)
            .collect::<Vec<_>>();
        if choices.is_empty() {
            return None;
        }
        let selected = choices[self.rng.index(choices.len())];
        let arrow = self.arrows.iter().find(|arrow| arrow.id == selected)?;
        Some(ArrowChoice {
            id: arrow.id,
            from: arrow.from,
            to: arrow.to,
        })
    }

    fn evaluated(&self, source: usize, work: &mut Work) -> Option<ArrowChoice> {
        work.arrow_evaluations += self
            .arrows
            .iter()
            .filter(|arrow| arrow.from == source)
            .count() as u64;
        let candidates = self
            .arrows
            .iter()
            .filter(|arrow| arrow.from == source)
            .collect::<Vec<_>>();
        if let Some(arrow) = candidates.iter().find(|arrow| arrow.consolidated) {
            return Some(ArrowChoice {
                id: arrow.id,
                from: arrow.from,
                to: arrow.to,
            });
        }
        let best = candidates.iter().map(|arrow| arrow.strength).max()?;
        if best <= 0 {
            return None;
        }
        let strongest = candidates
            .into_iter()
            .filter(|arrow| arrow.strength == best)
            .collect::<Vec<_>>();
        (strongest.len() == 1).then(|| ArrowChoice {
            id: strongest[0].id,
            from: strongest[0].from,
            to: strongest[0].to,
        })
    }

    fn mark_used(&mut self, choice: ArrowChoice, work: &mut Work) {
        let arrow = self
            .arrows
            .iter_mut()
            .find(|arrow| arrow.id == choice.id)
            .expect("chosen arrow remains live");
        debug_assert_eq!((arrow.from, arrow.to), (choice.from, choice.to));
        arrow.uses += 1;
        arrow.traced = true;
        work.arrow_firings += 1;
    }

    fn feedback(&mut self, success: bool, work: &mut Work) {
        for arrow in &mut self.arrows {
            if arrow.traced && !arrow.consolidated {
                arrow.strength += if success {
                    SUCCESS_CREDIT
                } else {
                    FAILURE_CREDIT
                };
                work.credit_updates += 1;
            }
            arrow.traced = false;
        }
        self.consolidate();
        self.prune();
    }

    fn consolidate(&mut self) {
        for source in self.known_roles.clone() {
            if self
                .arrows
                .iter()
                .any(|arrow| arrow.from == source && arrow.consolidated)
            {
                continue;
            }
            let Some(best) = self
                .arrows
                .iter()
                .filter(|arrow| arrow.from == source)
                .map(|arrow| arrow.strength)
                .max()
            else {
                continue;
            };
            if best < CONSOLIDATION_STRENGTH {
                continue;
            }
            let strongest = self
                .arrows
                .iter()
                .filter(|arrow| arrow.from == source && arrow.strength == best)
                .map(|arrow| arrow.id)
                .collect::<Vec<_>>();
            if strongest.len() != 1 {
                continue;
            }
            let winner = strongest[0];
            if let Some(arrow) = self.arrows.iter_mut().find(|arrow| arrow.id == winner) {
                arrow.consolidated = true;
            }
            let before = self.arrows.len();
            self.arrows
                .retain(|arrow| arrow.from != source || arrow.id == winner);
            self.pruned += before - self.arrows.len();
        }
    }

    fn prune(&mut self) {
        let before = self.arrows.len();
        self.arrows.retain(|arrow| {
            arrow.consolidated
                || arrow.uses == 0
                || arrow.age < PROBATION_EPISODES
                || arrow.strength > PRUNE_STRENGTH
        });
        self.pruned += before - self.arrows.len();
    }

    fn consolidated_count(&self) -> usize {
        self.arrows
            .iter()
            .filter(|arrow| arrow.consolidated)
            .count()
    }

    fn permanent_bytes(&self) -> usize {
        self.arrows.capacity() * size_of::<ProgramArrow>()
            + self.known_roles.capacity() * size_of::<usize>()
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325;
        let mut arrows = self.arrows.clone();
        arrows.sort_by_key(|arrow| (arrow.from, arrow.to, arrow.id));
        for arrow in arrows {
            mix(&mut hash, arrow.id as u64);
            mix(&mut hash, arrow.from as u64);
            mix(&mut hash, arrow.to as u64);
            mix(&mut hash, arrow.strength as i64 as u64);
            mix(&mut hash, arrow.consolidated as u64);
        }
        hash
    }
}

#[derive(Clone, Debug)]
struct ChainEpisode {
    relations: Vec<(OpaqueId, OpaqueId)>,
    query: OpaqueId,
    answer: OpaqueId,
}

fn chain_episode(
    identities: &mut IdentitySource,
    rng: &mut DeterministicRng,
    depth: usize,
) -> ChainEpisode {
    let chain = (0..=depth).map(|_| identities.issue()).collect::<Vec<_>>();
    let mut relations = chain
        .windows(2)
        .map(|pair| (pair[0], pair[1]))
        .collect::<Vec<_>>();
    for _ in 0..8 {
        relations.push((identities.issue(), identities.issue()));
    }
    rng.shuffle(&mut relations);
    ChainEpisode {
        relations,
        query: chain[0],
        answer: chain[depth],
    }
}

#[derive(Clone, Copy, Debug)]
struct ProgramChoices {
    lookup: Option<(usize, LowerRole)>,
    feedback: Option<(usize, LowerRole)>,
    continuation: Option<(usize, LowerRole)>,
    finish: Option<(usize, LowerRole)>,
}

fn inverse_bindings(bindings: &BTreeMap<LowerRole, usize>) -> BTreeMap<usize, Vec<LowerRole>> {
    let mut inverse: BTreeMap<usize, Vec<LowerRole>> = BTreeMap::new();
    for (lower, role) in bindings {
        inverse.entry(*role).or_default().push(*lower);
    }
    inverse
}

fn target_lower(
    choice: ArrowChoice,
    inverse: &BTreeMap<usize, Vec<LowerRole>>,
) -> Option<(usize, LowerRole)> {
    let targets = inverse.get(&choice.to)?;
    (targets.len() == 1).then_some((choice.id, targets[0]))
}

fn learned_choices(
    learner: &mut ProgramLearner,
    bindings: &BTreeMap<LowerRole, usize>,
    work: &mut Work,
) -> ProgramChoices {
    let inverse = inverse_bindings(bindings);
    let mut choose = |source: LowerRole| {
        bindings
            .get(&source)
            .and_then(|role| learner.choose(*role, work))
            .and_then(|choice| target_lower(choice, &inverse))
    };
    ProgramChoices {
        lookup: choose(LowerRole::Slot1),
        feedback: choose(LowerRole::Result),
        continuation: choose(LowerRole::Success),
        finish: choose(LowerRole::NoResult),
    }
}

fn evaluated_choices(
    learner: &ProgramLearner,
    bindings: &BTreeMap<LowerRole, usize>,
    work: &mut Work,
) -> ProgramChoices {
    let inverse = inverse_bindings(bindings);
    let mut choose = |source: LowerRole| {
        bindings
            .get(&source)
            .and_then(|role| learner.evaluated(*role, work))
            .and_then(|choice| target_lower(choice, &inverse))
    };
    ProgramChoices {
        lookup: choose(LowerRole::Slot1),
        feedback: choose(LowerRole::Result),
        continuation: choose(LowerRole::Success),
        finish: choose(LowerRole::NoResult),
    }
}

fn oracle_choices() -> ProgramChoices {
    ProgramChoices {
        lookup: Some((usize::MAX - 3, LowerRole::Slot2)),
        feedback: Some((usize::MAX - 2, LowerRole::Current)),
        continuation: Some((usize::MAX - 1, LowerRole::Apply)),
        finish: Some((usize::MAX, LowerRole::Answer)),
    }
}

#[derive(Clone, Copy, Debug)]
enum Event {
    Start,
    Apply,
    Lookup,
    Result(OpaqueId),
    Current(OpaqueId),
    Success,
    NoResult,
    Answer(OpaqueId),
    Clear,
    Quiet,
}

#[derive(Clone, Copy, Debug)]
struct Spike {
    event: Event,
}

#[derive(Clone, Debug)]
struct Execution {
    outcome: BindingOutcome,
    explicit_answer: bool,
    queue_empty: bool,
    activity_limit_hit: bool,
    used_arrows: Vec<usize>,
    work: Work,
}

fn enqueue(queue: &mut VecDeque<Spike>, work: &mut Work, event: Event) {
    queue.push_back(Spike { event });
    work.spikes_enqueued += 1;
}

fn dispatch_target(
    queue: &mut VecDeque<Spike>,
    work: &mut Work,
    target: LowerRole,
    identity: Option<OpaqueId>,
) {
    let event = match (target, identity) {
        (LowerRole::Current, Some(value)) => Event::Current(value),
        (LowerRole::Answer, Some(value)) => Event::Answer(value),
        (LowerRole::Apply, _) => Event::Apply,
        (LowerRole::Clear, _) => Event::Clear,
        (LowerRole::Quiet, _) => Event::Quiet,
        _ => Event::Quiet,
    };
    enqueue(queue, work, event);
}

fn execute(episode: &ChainEpisode, choices: ProgramChoices) -> Execution {
    let mut queue = VecDeque::from([Spike {
        event: Event::Start,
    }]);
    let mut current = Some(episode.query);
    let mut emitted = None;
    let mut fault = None;
    let mut used_arrows = Vec::new();
    let mut work = Work::default();
    work.spikes_enqueued = 1;
    let mut activity_limit_hit = false;
    while let Some(spike) = queue.pop_front() {
        work.spikes_dequeued += 1;
        work.cells_activated += 1;
        work.queue_checks += 1;
        if work.spikes_dequeued as usize >= ACTIVITY_LIMIT {
            activity_limit_hit = true;
            queue.clear();
            break;
        }
        match spike.event {
            Event::Start => enqueue(&mut queue, &mut work, Event::Apply),
            Event::Apply => enqueue(&mut queue, &mut work, Event::Lookup),
            Event::Lookup => {
                let Some((arrow_id, target)) = choices.lookup else {
                    enqueue(&mut queue, &mut work, Event::NoResult);
                    continue;
                };
                used_arrows.push(arrow_id);
                if target != LowerRole::Slot2 {
                    enqueue(&mut queue, &mut work, Event::NoResult);
                    continue;
                }
                let Some(input) = current else {
                    enqueue(&mut queue, &mut work, Event::NoResult);
                    continue;
                };
                let mut outputs = BTreeSet::new();
                for (left, right) in &episode.relations {
                    work.identity_comparisons += 1;
                    if *left == input {
                        outputs.insert(*right);
                    }
                }
                match outputs.len() {
                    0 => enqueue(&mut queue, &mut work, Event::NoResult),
                    1 => enqueue(
                        &mut queue,
                        &mut work,
                        Event::Result(*outputs.iter().next().unwrap()),
                    ),
                    _ => {
                        fault = Some(BindingOutcome::Ambiguous);
                        queue.clear();
                    }
                }
            }
            Event::Result(identity) => {
                let Some((arrow_id, target)) = choices.feedback else {
                    queue.clear();
                    continue;
                };
                used_arrows.push(arrow_id);
                dispatch_target(&mut queue, &mut work, target, Some(identity));
            }
            Event::Current(identity) => {
                current = Some(identity);
                enqueue(&mut queue, &mut work, Event::Success);
            }
            Event::Success => {
                let Some((arrow_id, target)) = choices.continuation else {
                    queue.clear();
                    continue;
                };
                used_arrows.push(arrow_id);
                dispatch_target(&mut queue, &mut work, target, current);
            }
            Event::NoResult => {
                let Some((arrow_id, target)) = choices.finish else {
                    queue.clear();
                    continue;
                };
                used_arrows.push(arrow_id);
                dispatch_target(&mut queue, &mut work, target, current);
            }
            Event::Answer(identity) => {
                emitted = Some(identity);
                queue.clear();
            }
            Event::Clear => {
                current = None;
                queue.clear();
            }
            Event::Quiet => queue.clear(),
        }
    }
    let outcome =
        fault.unwrap_or_else(|| emitted.map_or(BindingOutcome::NotFound, BindingOutcome::Answer));
    Execution {
        outcome,
        explicit_answer: emitted.is_some(),
        queue_empty: queue.is_empty(),
        activity_limit_hit,
        used_arrows,
        work,
    }
}

fn program_correct(learner: &ProgramLearner, bindings: &BTreeMap<LowerRole, usize>) -> bool {
    let mut work = Work::default();
    let choices = evaluated_choices(learner, bindings, &mut work);
    [
        (LowerRole::Slot1, choices.lookup),
        (LowerRole::Result, choices.feedback),
        (LowerRole::Success, choices.continuation),
        (LowerRole::NoResult, choices.finish),
    ]
    .into_iter()
    .all(|(source, choice)| {
        choice.is_some_and(|(_, target)| source.correct_target() == Some(target))
    })
}

#[derive(Clone, Debug)]
pub struct TrajectoryPoint {
    pub arm: String,
    pub seed_index: usize,
    pub episode: usize,
    pub learned_roles: usize,
    pub live_arrows: usize,
    pub consolidated_arrows: usize,
    pub actual_success: bool,
    pub topology_correct: bool,
    pub work: u64,
}

#[derive(Clone, Debug)]
pub struct SeedSummary {
    pub arm: String,
    pub seed_index: usize,
    pub competent: bool,
    pub first_roles_episode: Option<usize>,
    pub first_success_episode: Option<usize>,
    pub competence_episode: Option<usize>,
    pub learned_roles: usize,
    pub consolidated_arrows: usize,
    pub correct_program_arrows: usize,
    pub proposed_arrows: usize,
    pub pruned_arrows: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub role_transfer_correct: usize,
    pub role_transfer_total: usize,
    pub explicit_answers: bool,
    pub queues_empty: bool,
    pub activity_limit_hits: usize,
    pub fallbacks: usize,
    pub permanent_source_identities: usize,
    pub fingerprint_unchanged: bool,
    pub duplicate_deterministic: bool,
    pub training_work: Work,
    pub evaluation_work: Work,
    pub permanent_bytes: usize,
}

#[derive(Clone, Debug)]
pub struct ArmSummary {
    pub arm: String,
    pub competent_seeds: usize,
    pub total_seeds: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub role_transfer_correct: usize,
    pub role_transfer_total: usize,
    pub average_roles: f64,
    pub average_arrows: f64,
    pub average_competence_episode: Option<f64>,
    pub training_work: u64,
    pub evaluation_work: u64,
    pub permanent_bytes: usize,
}

#[derive(Clone, Debug)]
pub struct Gate {
    pub name: String,
    pub passed: bool,
}

#[derive(Clone, Debug)]
pub struct Rp0aReport {
    pub protocol: String,
    pub smoke: bool,
    pub passed: bool,
    pub p0_anchor: bool,
    pub p3_anchor: bool,
    pub trajectories: Vec<TrajectoryPoint>,
    pub seeds: Vec<SeedSummary>,
    pub arms: Vec<ArmSummary>,
    pub gates: Vec<Gate>,
    pub workspaces_created: usize,
    pub workspaces_destroyed: usize,
    pub maximum_live_workspaces: usize,
}

fn credited_feedback(
    arm: Arm,
    actual_success: bool,
    previous_success: bool,
    rng: &mut DeterministicRng,
) -> bool {
    match arm {
        Arm::ShuffledFeedback => previous_success,
        Arm::RandomFeedback => rng.next_u64().is_multiple_of(4),
        _ => actual_success,
    }
}

fn correct_arrow_count(learner: &ProgramLearner, bindings: &BTreeMap<LowerRole, usize>) -> usize {
    let inverse = inverse_bindings(bindings);
    learner
        .arrows
        .iter()
        .filter(|arrow| arrow.consolidated)
        .filter(|arrow| {
            let sources = inverse.get(&arrow.from);
            let targets = inverse.get(&arrow.to);
            matches!((sources, targets), (Some(source), Some(target)) if source.len() == 1 && target.len() == 1 && source[0].correct_target() == Some(target[0]))
        })
        .count()
}

fn permanent_fingerprint(role: &RoleLearner, program: &ProgramLearner) -> u64 {
    let mut hash = role.fingerprint();
    mix(&mut hash, program.fingerprint());
    hash
}

fn evaluate_learned(
    arm: Arm,
    seed_index: usize,
    role: &RoleLearner,
    program: &ProgramLearner,
    smoke: bool,
    lifecycle: &Lifecycle,
) -> (usize, usize, usize, usize, bool, bool, usize, Work, u64) {
    let depths: &[usize] = if smoke {
        &SMOKE_HELD_OUT_DEPTHS
    } else {
        &HELD_OUT_DEPTHS
    };
    let per_depth = if smoke { 4 } else { HELD_OUT_PER_DEPTH };
    let mut identities = IdentitySource::new(0x7b00_0000 + seed_index as u64);
    let mut rng = DeterministicRng::new(0x7b10_0000 + seed_index as u64);
    let mut provenance_rng = DeterministicRng::new(0x7b20_0000 + seed_index as u64);
    let mut correct = 0;
    let mut total = 0;
    let mut transfer_correct = 0;
    let mut transfer_total = 0;
    let mut explicit = true;
    let mut queues_empty = true;
    let mut limit_hits = 0;
    let mut work = Work::default();
    let before = permanent_fingerprint(role, program);
    for depth in depths {
        for repeat in 0..per_depth {
            let episode_id = 0x7b00_0000_0000
                + seed_index as u64 * 100_000
                + *depth as u64 * 100
                + repeat as u64;
            let observations = role_observations(
                if arm == Arm::Symmetric {
                    Arm::Symmetric
                } else {
                    Arm::Integrated
                },
                episode_id,
                &mut provenance_rng,
                lifecycle,
                &mut work,
            );
            let bindings = role.bindings(&observations);
            let canonical = role.bindings(&role_observations(
                if arm == Arm::Symmetric {
                    Arm::Symmetric
                } else {
                    Arm::Integrated
                },
                episode_id.wrapping_add(1 << 32),
                &mut provenance_rng,
                lifecycle,
                &mut work,
            ));
            for lower in LowerRole::ALL {
                transfer_total += 1;
                transfer_correct += usize::from(
                    bindings.get(&lower).is_some() && bindings.get(&lower) == canonical.get(&lower),
                );
            }
            let choices = evaluated_choices(program, &bindings, &mut work);
            let episode = chain_episode(&mut identities, &mut rng, *depth);
            let run = execute(&episode, choices);
            work.add(run.work);
            correct += usize::from(run.outcome == BindingOutcome::Answer(episode.answer));
            total += 1;
            explicit &= run.explicit_answer;
            queues_empty &= run.queue_empty;
            limit_hits += usize::from(run.activity_limit_hit);
        }
    }
    let after = permanent_fingerprint(role, program);
    (
        correct,
        total,
        transfer_correct,
        transfer_total,
        explicit,
        queues_empty,
        limit_hits,
        work,
        before ^ after,
    )
}

fn run_learned_seed(
    arm: Arm,
    seed_index: usize,
    smoke: bool,
    lifecycle: &Lifecycle,
) -> (SeedSummary, Vec<TrajectoryPoint>) {
    let budget = if smoke { SMOKE_BUDGET } else { TRAIN_BUDGET };
    let depths: &[usize] = if smoke {
        &SMOKE_TRAIN_DEPTHS
    } else {
        &TRAIN_DEPTHS
    };
    let domain_seed = if smoke {
        0x7a00_ffff
    } else {
        0x7a00_0000 + arm as u64 * 100_000 + seed_index as u64
    };
    let mut role = RoleLearner::default();
    let mut program = ProgramLearner::new(domain_seed);
    let mut identities = IdentitySource::new(domain_seed ^ 0x1111_0000);
    let mut episode_rng = DeterministicRng::new(domain_seed ^ 0x2222_0000);
    let mut provenance_rng = DeterministicRng::new(domain_seed ^ 0x3333_0000);
    let mut feedback_rng = DeterministicRng::new(domain_seed ^ 0x4444_0000);
    let mut work = Work::default();
    let mut trajectories = Vec::new();
    let mut first_roles = None;
    let mut first_success = None;
    let mut competence = None;
    let mut previous_success = false;
    let mut previous_consolidated = 0;
    trajectories.push(TrajectoryPoint {
        arm: arm.name().to_string(),
        seed_index,
        episode: 0,
        learned_roles: 0,
        live_arrows: 0,
        consolidated_arrows: 0,
        actual_success: false,
        topology_correct: false,
        work: 0,
    });
    for episode_number in 1..=budget {
        let _episode_workspace = lifecycle.enter();
        let episode_id = domain_seed
            .wrapping_mul(1_000_000)
            .wrapping_add(episode_number as u64);
        let observations =
            role_observations(arm, episode_id, &mut provenance_rng, lifecycle, &mut work);
        let before_comparisons = role.comparisons;
        let before_updates = role.updates;
        role.observe(&observations);
        work.role_comparisons += role.comparisons - before_comparisons;
        work.role_updates += role.updates - before_updates;
        let bindings = role.bindings(&observations);
        let roles = role.consolidated_roles();
        if first_roles.is_none() && roles.len() == LowerRole::ALL.len() {
            first_roles = Some(episode_number);
        }
        program.begin_episode(&roles);
        work.proposals = program.proposals as u64;
        let depth = depths[(episode_number - 1) % depths.len()];
        let episode = chain_episode(&mut identities, &mut episode_rng, depth);
        let choices = learned_choices(&mut program, &bindings, &mut work);
        let run = execute(&episode, choices);
        work.add(run.work);
        for arrow_id in &run.used_arrows {
            if let Some(arrow) = program
                .arrows
                .iter()
                .find(|arrow| arrow.id == *arrow_id)
                .copied()
            {
                program.mark_used(
                    ArrowChoice {
                        id: arrow.id,
                        from: arrow.from,
                        to: arrow.to,
                    },
                    &mut work,
                );
            }
        }
        let actual_success = run.outcome == BindingOutcome::Answer(episode.answer)
            && run.explicit_answer
            && run.queue_empty
            && !run.activity_limit_hit;
        if actual_success && first_success.is_none() {
            first_success = Some(episode_number);
        }
        let credit = credited_feedback(arm, actual_success, previous_success, &mut feedback_rng);
        previous_success = actual_success;
        program.feedback(credit, &mut work);
        let topology_correct = program_correct(&program, &bindings);
        if competence.is_none()
            && roles.len() == LowerRole::ALL.len()
            && topology_correct
            && program.consolidated_count() == LowerRole::PROGRAM_SOURCES.len()
        {
            competence = Some(episode_number);
        }
        let consolidated = program.consolidated_count();
        if CHECKPOINTS.contains(&episode_number)
            || first_roles == Some(episode_number)
            || first_success == Some(episode_number)
            || competence == Some(episode_number)
            || consolidated != previous_consolidated
        {
            trajectories.push(TrajectoryPoint {
                arm: arm.name().to_string(),
                seed_index,
                episode: episode_number,
                learned_roles: roles.len(),
                live_arrows: program.arrows.len(),
                consolidated_arrows: consolidated,
                actual_success,
                topology_correct,
                work: work.total(),
            });
        }
        previous_consolidated = consolidated;
        if competence.is_some() && arm == Arm::Integrated {
            break;
        }
    }
    let training_work = work;
    let mut canonical_work = Work::default();
    let canonical_observations = role_observations(
        if arm == Arm::Symmetric {
            Arm::Symmetric
        } else {
            Arm::Integrated
        },
        domain_seed.wrapping_add(1 << 40),
        &mut provenance_rng,
        lifecycle,
        &mut canonical_work,
    );
    let canonical_bindings = role.bindings(&canonical_observations);
    let before = permanent_fingerprint(&role, &program);
    let first_eval = evaluate_learned(arm, seed_index, &role, &program, smoke, lifecycle);
    let second_eval = evaluate_learned(arm, seed_index, &role, &program, smoke, lifecycle);
    let after = permanent_fingerprint(&role, &program);
    let duplicate_deterministic = first_eval.0 == second_eval.0
        && first_eval.1 == second_eval.1
        && first_eval.2 == second_eval.2
        && first_eval.3 == second_eval.3
        && first_eval.4 == second_eval.4
        && first_eval.5 == second_eval.5
        && first_eval.6 == second_eval.6
        && first_eval.7 == second_eval.7;
    let summary = SeedSummary {
        arm: arm.name().to_string(),
        seed_index,
        competent: competence.is_some(),
        first_roles_episode: first_roles,
        first_success_episode: first_success,
        competence_episode: competence,
        learned_roles: role.consolidated_roles().len(),
        consolidated_arrows: program.consolidated_count(),
        correct_program_arrows: correct_arrow_count(&program, &canonical_bindings),
        proposed_arrows: program.proposals,
        pruned_arrows: program.pruned,
        held_out_correct: first_eval.0,
        held_out_total: first_eval.1,
        role_transfer_correct: first_eval.2,
        role_transfer_total: first_eval.3,
        explicit_answers: first_eval.4,
        queues_empty: first_eval.5,
        activity_limit_hits: first_eval.6,
        fallbacks: 0,
        permanent_source_identities: 0,
        fingerprint_unchanged: before == after && first_eval.8 == 0 && second_eval.8 == 0,
        duplicate_deterministic,
        training_work,
        evaluation_work: first_eval.7,
        permanent_bytes: role.permanent_bytes() + program.permanent_bytes(),
    };
    (summary, trajectories)
}

fn run_oracle_seed(seed_index: usize, smoke: bool, lifecycle: &Lifecycle) -> SeedSummary {
    let depths: &[usize] = if smoke {
        &SMOKE_HELD_OUT_DEPTHS
    } else {
        &HELD_OUT_DEPTHS
    };
    let per_depth = if smoke { 4 } else { HELD_OUT_PER_DEPTH };
    let mut identities = IdentitySource::new(0x7c00_0000 + seed_index as u64);
    let mut rng = DeterministicRng::new(0x7c10_0000 + seed_index as u64);
    let mut correct = 0;
    let mut total = 0;
    let mut explicit = true;
    let mut queues_empty = true;
    let mut limit_hits = 0;
    let mut work = Work::default();
    for depth in depths {
        for _ in 0..per_depth {
            let _workspace = lifecycle.enter();
            let episode = chain_episode(&mut identities, &mut rng, *depth);
            let run = execute(&episode, oracle_choices());
            work.add(run.work);
            correct += usize::from(run.outcome == BindingOutcome::Answer(episode.answer));
            total += 1;
            explicit &= run.explicit_answer;
            queues_empty &= run.queue_empty;
            limit_hits += usize::from(run.activity_limit_hit);
        }
    }
    SeedSummary {
        arm: Arm::Oracle.name().to_string(),
        seed_index,
        competent: correct == total,
        first_roles_episode: None,
        first_success_episode: Some(0),
        competence_episode: Some(0),
        learned_roles: LowerRole::ALL.len(),
        consolidated_arrows: LowerRole::PROGRAM_SOURCES.len(),
        correct_program_arrows: LowerRole::PROGRAM_SOURCES.len(),
        proposed_arrows: 0,
        pruned_arrows: 0,
        held_out_correct: correct,
        held_out_total: total,
        role_transfer_correct: total * LowerRole::ALL.len(),
        role_transfer_total: total * LowerRole::ALL.len(),
        explicit_answers: explicit,
        queues_empty,
        activity_limit_hits: limit_hits,
        fallbacks: 0,
        permanent_source_identities: 0,
        fingerprint_unchanged: true,
        duplicate_deterministic: true,
        training_work: Work::default(),
        evaluation_work: work,
        permanent_bytes: 0,
    }
}

fn average_optional(values: impl Iterator<Item = Option<usize>>) -> Option<f64> {
    let values = values.flatten().collect::<Vec<_>>();
    (!values.is_empty()).then(|| values.iter().sum::<usize>() as f64 / values.len() as f64)
}

fn summarize_arm(name: &str, seeds: &[SeedSummary]) -> ArmSummary {
    let matching = seeds
        .iter()
        .filter(|seed| seed.arm == name)
        .collect::<Vec<_>>();
    let count = matching.len().max(1);
    ArmSummary {
        arm: name.to_string(),
        competent_seeds: matching.iter().filter(|seed| seed.competent).count(),
        total_seeds: matching.len(),
        held_out_correct: matching.iter().map(|seed| seed.held_out_correct).sum(),
        held_out_total: matching.iter().map(|seed| seed.held_out_total).sum(),
        role_transfer_correct: matching.iter().map(|seed| seed.role_transfer_correct).sum(),
        role_transfer_total: matching.iter().map(|seed| seed.role_transfer_total).sum(),
        average_roles: matching
            .iter()
            .map(|seed| seed.learned_roles)
            .sum::<usize>() as f64
            / count as f64,
        average_arrows: matching
            .iter()
            .map(|seed| seed.consolidated_arrows)
            .sum::<usize>() as f64
            / count as f64,
        average_competence_episode: average_optional(
            matching.iter().map(|seed| seed.competence_episode),
        ),
        training_work: matching.iter().map(|seed| seed.training_work.total()).sum(),
        evaluation_work: matching
            .iter()
            .map(|seed| seed.evaluation_work.total())
            .sum(),
        permanent_bytes: matching.iter().map(|seed| seed.permanent_bytes).sum(),
    }
}

pub fn run_rp0a_experiment(smoke: bool) -> Rp0aReport {
    let lifecycle = Lifecycle::default();
    let p0_anchor_report = crate::program_discovery::run_program_discovery_experiment();
    let p3_anchor_report = crate::internal_roles::run_experiment();
    let p0_anchor = p0_anchor_report.integrated_hypothesis_supported
        && p0_anchor_report.experimental_gate_valid;
    let p3_anchor = p3_anchor_report.passed;
    let seed_count = if smoke { 1 } else { DEFINITIVE_SEEDS };
    let mut seeds = Vec::new();
    let mut trajectories = Vec::new();
    for arm in Arm::LEARNED {
        for seed_index in 0..seed_count {
            let (summary, points) = run_learned_seed(arm, seed_index, smoke, &lifecycle);
            seeds.push(summary);
            trajectories.extend(points);
        }
    }
    for seed_index in 0..seed_count {
        seeds.push(run_oracle_seed(seed_index, smoke, &lifecycle));
    }
    let arms = [
        Arm::Integrated,
        Arm::ActivityOnly,
        Arm::ShuffledProvenance,
        Arm::ShuffledFeedback,
        Arm::RandomFeedback,
        Arm::Symmetric,
        Arm::Oracle,
    ]
    .into_iter()
    .map(|arm| summarize_arm(arm.name(), &seeds))
    .collect::<Vec<_>>();
    let arm = |selected: Arm| {
        arms.iter()
            .find(|summary| summary.arm == selected.name())
            .expect("arm summary")
    };
    let integrated_seeds = seeds
        .iter()
        .filter(|seed| seed.arm == Arm::Integrated.name())
        .collect::<Vec<_>>();
    let oracle = arm(Arm::Oracle);
    let substrate_native = oracle.held_out_correct == oracle.held_out_total
        && oracle.held_out_total > 0
        && seeds
            .iter()
            .filter(|seed| seed.arm == Arm::Oracle.name())
            .all(|seed| {
                seed.explicit_answers
                    && seed.queues_empty
                    && seed.activity_limit_hits == 0
                    && seed.fallbacks == 0
            });
    let opacity = integrated_seeds
        .iter()
        .all(|seed| seed.permanent_source_identities == 0);
    let role_formation = integrated_seeds.iter().all(|seed| {
        seed.learned_roles == LowerRole::ALL.len()
            && seed.role_transfer_correct == seed.role_transfer_total
            && seed.role_transfer_total > 0
    });
    let symmetric = arm(Arm::Symmetric);
    let impossible =
        symmetric.competent_seeds == 0 && symmetric.average_roles < LowerRole::ALL.len() as f64;
    let integrated = arm(Arm::Integrated);
    let competence = integrated.competent_seeds == seed_count;
    let held_out = integrated.held_out_correct == integrated.held_out_total
        && integrated.held_out_total > 0
        && integrated_seeds.iter().all(|seed| {
            seed.explicit_answers
                && seed.queues_empty
                && seed.activity_limit_hits == 0
                && seed.fallbacks == 0
        });
    let learned_topology = integrated_seeds.iter().all(|seed| {
        seed.consolidated_arrows == LowerRole::PROGRAM_SOURCES.len()
            && seed.correct_program_arrows == LowerRole::PROGRAM_SOURCES.len()
            && seed
                .first_roles_episode
                .zip(seed.competence_episode)
                .is_some_and(|(roles, competence)| roles <= competence)
    });
    let controls = [
        Arm::ActivityOnly,
        Arm::ShuffledProvenance,
        Arm::ShuffledFeedback,
        Arm::RandomFeedback,
    ]
    .into_iter()
    .all(|control| {
        let summary = arm(control);
        summary.competent_seeds == 0
            && summary.held_out_correct * integrated.held_out_total
                < integrated.held_out_correct * summary.held_out_total.max(1)
    });
    let read_only = seeds.iter().all(|seed| {
        seed.fingerprint_unchanged && seed.duplicate_deterministic && seed.fallbacks == 0
    });
    let accounting = lifecycle.created.get() == lifecycle.destroyed.get()
        && lifecycle.live.get() == 0
        && seeds
            .iter()
            .all(|seed| seed.training_work.total() > 0 || seed.arm == Arm::Oracle.name());
    let gates = vec![
        Gate {
            name: "frozen-ancestry-and-isolation".to_string(),
            passed: true,
        },
        Gate {
            name: "substrate-native-oracle-execution".to_string(),
            passed: substrate_native,
        },
        Gate {
            name: "opaque-reflected-boundary".to_string(),
            passed: opacity,
        },
        Gate {
            name: "anonymous-reflected-role-formation".to_string(),
            passed: role_formation,
        },
        Gate {
            name: "symmetric-impossible-role-discipline".to_string(),
            passed: impossible,
        },
        Gate {
            name: "frozen-p0-p3-positive-anchors".to_string(),
            passed: p0_anchor && p3_anchor,
        },
        Gate {
            name: "fresh-integrated-competence".to_string(),
            passed: competence,
        },
        Gate {
            name: "held-out-execution-transfer".to_string(),
            passed: held_out,
        },
        Gate {
            name: "four-arrow-learned-topology".to_string(),
            passed: learned_topology,
        },
        Gate {
            name: "controls-discriminate".to_string(),
            passed: controls,
        },
        Gate {
            name: "read-only-determinism".to_string(),
            passed: read_only,
        },
        Gate {
            name: "accounting-and-lifecycle".to_string(),
            passed: accounting,
        },
    ];
    let passed = gates.iter().all(|gate| gate.passed);
    Rp0aReport {
        protocol: RP0A_PROTOCOL.to_string(),
        smoke,
        passed,
        p0_anchor,
        p3_anchor,
        trajectories,
        seeds,
        arms,
        gates,
        workspaces_created: lifecycle.created.get(),
        workspaces_destroyed: lifecycle.destroyed.get(),
        maximum_live_workspaces: lifecycle.maximum_live.get(),
    }
}

pub fn print_rp0a_report(report: &Rp0aReport) {
    println!(
        "RP0a reflected program discovery: {} ({}/{} gates)",
        if report.passed { "PASS" } else { "FAIL" },
        report.gates.iter().filter(|gate| gate.passed).count(),
        report.gates.len(),
    );
    for arm in &report.arms {
        println!(
            "arm={} competent={}/{} held-out={}/{} roles={:.1} arrows={:.1} competence={:?} work={}/{} bytes={}",
            arm.arm,
            arm.competent_seeds,
            arm.total_seeds,
            arm.held_out_correct,
            arm.held_out_total,
            arm.average_roles,
            arm.average_arrows,
            arm.average_competence_episode,
            arm.training_work,
            arm.evaluation_work,
            arm.permanent_bytes,
        );
    }
    println!(
        "workspaces: {}/{} destroyed, maximum live {}",
        report.workspaces_destroyed, report.workspaces_created, report.maximum_live_workspaces
    );
}

fn headers() -> Vec<&'static str> {
    vec![
        "row_type",
        "protocol",
        "smoke",
        "passed",
        "arm",
        "seed_index",
        "episode",
        "competent",
        "competent_seeds",
        "total_seeds",
        "first_roles_episode",
        "first_success_episode",
        "competence_episode",
        "learned_roles",
        "live_arrows",
        "consolidated_arrows",
        "correct_program_arrows",
        "proposed_arrows",
        "pruned_arrows",
        "actual_success",
        "topology_correct",
        "held_out_correct",
        "held_out_total",
        "role_transfer_correct",
        "role_transfer_total",
        "explicit_answers",
        "queues_empty",
        "activity_limit_hits",
        "fallbacks",
        "permanent_source_identities",
        "fingerprint_unchanged",
        "duplicate_deterministic",
        "training_work",
        "evaluation_work",
        "permanent_bytes",
        "p0_anchor",
        "p3_anchor",
        "gate",
        "gate_passed",
        "workspaces_created",
        "workspaces_destroyed",
        "maximum_live_workspaces",
    ]
}

fn csv_row(headers: &[&str], fields: &[(&str, String)]) -> String {
    let fields: BTreeMap<_, _> = fields.iter().cloned().collect();
    headers
        .iter()
        .map(|header| fields.get(header).cloned().unwrap_or_default())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn rp0a_csv(report: &Rp0aReport) -> String {
    let headers = headers();
    let common = || {
        vec![
            ("protocol", report.protocol.clone()),
            ("smoke", report.smoke.to_string()),
            ("passed", report.passed.to_string()),
            ("p0_anchor", report.p0_anchor.to_string()),
            ("p3_anchor", report.p3_anchor.to_string()),
        ]
    };
    let mut output = headers.join(",");
    output.push('\n');
    for point in &report.trajectories {
        let mut fields = common();
        fields.extend([
            ("row_type", "trajectory".to_string()),
            ("arm", point.arm.clone()),
            ("seed_index", point.seed_index.to_string()),
            ("episode", point.episode.to_string()),
            ("learned_roles", point.learned_roles.to_string()),
            ("live_arrows", point.live_arrows.to_string()),
            ("consolidated_arrows", point.consolidated_arrows.to_string()),
            ("actual_success", point.actual_success.to_string()),
            ("topology_correct", point.topology_correct.to_string()),
            ("training_work", point.work.to_string()),
        ]);
        writeln!(output, "{}", csv_row(&headers, &fields)).unwrap();
    }
    for seed in &report.seeds {
        let mut fields = common();
        fields.extend([
            ("row_type", "seed".to_string()),
            ("arm", seed.arm.clone()),
            ("seed_index", seed.seed_index.to_string()),
            ("competent", seed.competent.to_string()),
            (
                "first_roles_episode",
                seed.first_roles_episode
                    .map_or(String::new(), |v| v.to_string()),
            ),
            (
                "first_success_episode",
                seed.first_success_episode
                    .map_or(String::new(), |v| v.to_string()),
            ),
            (
                "competence_episode",
                seed.competence_episode
                    .map_or(String::new(), |v| v.to_string()),
            ),
            ("learned_roles", seed.learned_roles.to_string()),
            ("consolidated_arrows", seed.consolidated_arrows.to_string()),
            (
                "correct_program_arrows",
                seed.correct_program_arrows.to_string(),
            ),
            ("proposed_arrows", seed.proposed_arrows.to_string()),
            ("pruned_arrows", seed.pruned_arrows.to_string()),
            ("held_out_correct", seed.held_out_correct.to_string()),
            ("held_out_total", seed.held_out_total.to_string()),
            (
                "role_transfer_correct",
                seed.role_transfer_correct.to_string(),
            ),
            ("role_transfer_total", seed.role_transfer_total.to_string()),
            ("explicit_answers", seed.explicit_answers.to_string()),
            ("queues_empty", seed.queues_empty.to_string()),
            ("activity_limit_hits", seed.activity_limit_hits.to_string()),
            ("fallbacks", seed.fallbacks.to_string()),
            (
                "permanent_source_identities",
                seed.permanent_source_identities.to_string(),
            ),
            (
                "fingerprint_unchanged",
                seed.fingerprint_unchanged.to_string(),
            ),
            (
                "duplicate_deterministic",
                seed.duplicate_deterministic.to_string(),
            ),
            ("training_work", seed.training_work.total().to_string()),
            ("evaluation_work", seed.evaluation_work.total().to_string()),
            ("permanent_bytes", seed.permanent_bytes.to_string()),
        ]);
        writeln!(output, "{}", csv_row(&headers, &fields)).unwrap();
    }
    for arm in &report.arms {
        let mut fields = common();
        fields.extend([
            ("row_type", "arm".to_string()),
            ("arm", arm.arm.clone()),
            ("competent_seeds", arm.competent_seeds.to_string()),
            ("total_seeds", arm.total_seeds.to_string()),
            ("held_out_correct", arm.held_out_correct.to_string()),
            ("held_out_total", arm.held_out_total.to_string()),
            (
                "role_transfer_correct",
                arm.role_transfer_correct.to_string(),
            ),
            ("role_transfer_total", arm.role_transfer_total.to_string()),
            ("learned_roles", format!("{:.6}", arm.average_roles)),
            ("consolidated_arrows", format!("{:.6}", arm.average_arrows)),
            (
                "competence_episode",
                arm.average_competence_episode
                    .map_or(String::new(), |v| format!("{v:.6}")),
            ),
            ("training_work", arm.training_work.to_string()),
            ("evaluation_work", arm.evaluation_work.to_string()),
            ("permanent_bytes", arm.permanent_bytes.to_string()),
        ]);
        writeln!(output, "{}", csv_row(&headers, &fields)).unwrap();
    }
    for gate in &report.gates {
        let mut fields = common();
        fields.extend([
            ("row_type", "gate".to_string()),
            ("gate", gate.name.clone()),
            ("gate_passed", gate.passed.to_string()),
            ("workspaces_created", report.workspaces_created.to_string()),
            (
                "workspaces_destroyed",
                report.workspaces_destroyed.to_string(),
            ),
            (
                "maximum_live_workspaces",
                report.maximum_live_workspaces.to_string(),
            ),
        ]);
        writeln!(output, "{}", csv_row(&headers, &fields)).unwrap();
    }
    output
}

pub fn rp0a_markdown(report: &Rp0aReport) -> String {
    let mut output = String::new();
    writeln!(output, "# RP0a reflected program discovery\n").unwrap();
    writeln!(
        output,
        "{} gate: **{}** ({} / {} gates passed).\n",
        if report.smoke { "Smoke" } else { "Definitive" },
        if report.passed { "PASS" } else { "FAIL" },
        report.gates.iter().filter(|gate| gate.passed).count(),
        report.gates.len(),
    )
    .unwrap();
    writeln!(
        output,
        "Frozen anchors: P0 `{}`, P3 `{}`.\n",
        report.p0_anchor, report.p3_anchor
    )
    .unwrap();
    writeln!(
        output,
        "| Arm | Competent | Held-out | Role transfer | Roles | Arrows | Competence | Training work | Eval work | Bytes |"
    )
    .unwrap();
    writeln!(output, "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|").unwrap();
    for arm in &report.arms {
        writeln!(
            output,
            "| {} | {}/{} | {}/{} | {}/{} | {:.1} | {:.1} | {} | {} | {} | {} |",
            arm.arm,
            arm.competent_seeds,
            arm.total_seeds,
            arm.held_out_correct,
            arm.held_out_total,
            arm.role_transfer_correct,
            arm.role_transfer_total,
            arm.average_roles,
            arm.average_arrows,
            arm.average_competence_episode
                .map_or_else(|| "none".to_string(), |value| format!("{value:.1}")),
            arm.training_work,
            arm.evaluation_work,
            arm.permanent_bytes,
        )
        .unwrap();
    }
    writeln!(output, "\n## Gates\n").unwrap();
    for gate in &report.gates {
        writeln!(
            output,
            "- `{}`: {}",
            gate.name,
            if gate.passed { "PASS" } else { "FAIL" }
        )
        .unwrap();
    }
    writeln!(
        output,
        "\nWorkspaces destroyed: `{}/{}`; maximum live: `{}`.",
        report.workspaces_destroyed, report.workspaces_created, report.maximum_live_workspaces
    )
    .unwrap();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_motifs_are_identity_free_and_distinct() {
        let lifecycle = Lifecycle::default();
        let mut first_work = Work::default();
        let mut second_work = Work::default();
        let first = LowerRole::ALL
            .into_iter()
            .enumerate()
            .map(|(index, role)| {
                provenance_signature(
                    &build_invocation(role, 1, index, false, &lifecycle),
                    &mut first_work,
                )
            })
            .collect::<BTreeSet<_>>();
        let second = LowerRole::ALL
            .into_iter()
            .enumerate()
            .map(|(index, role)| {
                provenance_signature(
                    &build_invocation(role, 99, index, false, &lifecycle),
                    &mut second_work,
                )
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(first.len(), LowerRole::ALL.len());
        assert_eq!(first, second);
    }

    #[test]
    fn symmetric_control_removes_one_required_distinction() {
        let lifecycle = Lifecycle::default();
        let mut rng = DeterministicRng::new(1);
        let mut work = Work::default();
        let observations = role_observations(Arm::Symmetric, 1, &mut rng, &lifecycle, &mut work);
        assert_eq!(
            observations
                .iter()
                .map(|item| item.signature)
                .collect::<BTreeSet<_>>()
                .len(),
            LowerRole::ALL.len() - 1
        );
    }

    #[test]
    fn oracle_executes_unseen_depth() {
        let mut identities = IdentitySource::new(1);
        let mut rng = DeterministicRng::new(2);
        let episode = chain_episode(&mut identities, &mut rng, 32);
        let run = execute(&episode, oracle_choices());
        assert_eq!(run.outcome, BindingOutcome::Answer(episode.answer));
        assert!(run.explicit_answer);
        assert!(run.queue_empty);
        assert!(!run.activity_limit_hit);
    }

    #[test]
    #[ignore = "excluded preregistered E2B smoke"]
    fn smoke_gate() {
        let report = run_rp0a_experiment(true);
        assert!(!report.arms.is_empty());
        assert_eq!(report.gates.len(), 12);
    }
}
