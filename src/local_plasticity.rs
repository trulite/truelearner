use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::OnceLock;

use crate::binding::{BindingOutcome, IdentitySource, OpaqueId};

const SEEDS: usize = 8;
const ROLE_THRESHOLD: i32 = 4;
const CONSOLIDATION_STRENGTH: i32 = 6;
const SUCCESS_CREDIT: i32 = 2;
const FAILURE_CREDIT: i32 = -1;
const ACTIVITY_LIMIT: usize = 1_600;
const TRAIN_DEPTHS: [usize; 4] = [1, 2, 3, 4];
const HELD_OUT_DEPTHS: [usize; 4] = [5, 8, 16, 32];

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
enum EncodingFamily {
    Training,
    Transferred,
    Symmetric,
}

#[derive(Clone, Debug)]
struct RawNode {
    receptor: u64,
    identity: Option<OpaqueId>,
    outgoing: Vec<u64>,
}

#[derive(Clone, Debug)]
struct RawEpisode {
    nodes: Vec<RawNode>,
    signatures: HashMap<u64, u64>,
}

#[derive(Clone, Debug)]
struct LogicalEpisode {
    relations: Vec<(OpaqueId, OpaqueId)>,
    query: OpaqueId,
    correct: BindingOutcome,
}

fn chain_episode(
    identities: &mut IdentitySource,
    rng: &mut DeterministicRng,
    depth: usize,
    relation_count: usize,
) -> LogicalEpisode {
    let chain: Vec<_> = (0..=depth).map(|_| identities.issue()).collect();
    let mut relations: Vec<_> = chain.windows(2).map(|pair| (pair[0], pair[1])).collect();
    for _ in depth..relation_count {
        relations.push((identities.issue(), identities.issue()));
    }
    rng.shuffle(&mut relations);
    LogicalEpisode {
        relations,
        query: chain[0],
        correct: BindingOutcome::Answer(chain[depth]),
    }
}

fn encode_episode(episode: &LogicalEpisode, family: EncodingFamily, seed: u64) -> RawEpisode {
    let mut rng = DeterministicRng::new(seed ^ 0x51_10_aa_19);
    let mut next_receptor = || rng.next_u64();
    let mut nodes = Vec::new();

    for &(left, right) in &episode.relations {
        let container = next_receptor();
        let first = next_receptor();
        let second = next_receptor();
        match family {
            EncodingFamily::Training | EncodingFamily::Transferred => {
                nodes.push(RawNode {
                    receptor: container,
                    identity: None,
                    outgoing: vec![first],
                });
                nodes.push(RawNode {
                    receptor: first,
                    identity: Some(left),
                    outgoing: vec![second],
                });
                nodes.push(RawNode {
                    receptor: second,
                    identity: Some(right),
                    outgoing: Vec::new(),
                });
            }
            EncodingFamily::Symmetric => {
                nodes.push(RawNode {
                    receptor: container,
                    identity: None,
                    outgoing: vec![first, second],
                });
                nodes.push(RawNode {
                    receptor: first,
                    identity: Some(left),
                    outgoing: Vec::new(),
                });
                nodes.push(RawNode {
                    receptor: second,
                    identity: Some(right),
                    outgoing: Vec::new(),
                });
            }
        }
    }

    nodes.push(RawNode {
        receptor: next_receptor(),
        identity: Some(episode.query),
        outgoing: Vec::new(),
    });

    match family {
        EncodingFamily::Training => {}
        EncodingFamily::Transferred => {
            nodes.reverse();
            rng.shuffle(&mut nodes);
            for node in &mut nodes {
                node.outgoing.reverse();
            }
        }
        EncodingFamily::Symmetric => rng.shuffle(&mut nodes),
    }
    let mut raw = RawEpisode {
        nodes,
        signatures: HashMap::new(),
    };
    raw.signatures = compute_structural_signatures(&raw);
    raw
}

fn structural_signatures(episode: &RawEpisode) -> &HashMap<u64, u64> {
    &episode.signatures
}

fn compute_structural_signatures(episode: &RawEpisode) -> HashMap<u64, u64> {
    let index: HashMap<_, _> = episode
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.receptor, index))
        .collect();
    let mut incoming = vec![Vec::new(); episode.nodes.len()];
    for (source_index, node) in episode.nodes.iter().enumerate() {
        for target in &node.outgoing {
            let target_index = index[target];
            incoming[target_index].push(source_index);
        }
    }

    let mut labels: Vec<_> = episode
        .nodes
        .iter()
        .enumerate()
        .map(|(node_index, node)| {
            hash_words(&[
                u64::from(node.identity.is_some()),
                incoming[node_index].len() as u64,
                node.outgoing.len() as u64,
            ])
        })
        .collect();

    for _ in 0..3 {
        let previous = labels.clone();
        for (node_index, node) in episode.nodes.iter().enumerate() {
            let mut sources: Vec<_> = incoming[node_index]
                .iter()
                .map(|source| previous[*source])
                .collect();
            let mut targets: Vec<_> = node
                .outgoing
                .iter()
                .map(|target| previous[index[target]])
                .collect();
            sources.sort_unstable();
            targets.sort_unstable();
            let mut words = vec![
                previous[node_index],
                sources.len() as u64,
                targets.len() as u64,
            ];
            words.extend(sources);
            words.push(u64::MAX);
            words.extend(targets);
            labels[node_index] = hash_words(&words);
        }
    }

    episode
        .nodes
        .iter()
        .zip(labels)
        .map(|(node, label)| (node.receptor, label))
        .collect()
}

fn hash_words(words: &[u64]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for word in words {
        fingerprint_mix(&mut hash, *word);
    }
    hash
}

#[derive(Clone, Debug)]
struct RolePattern {
    cell: usize,
    signature: u64,
    evidence: i32,
    consolidated: bool,
}

#[derive(Clone, Debug, Default)]
struct SensoryRoleLearner {
    patterns: Vec<RolePattern>,
    next_cell: usize,
    observations: usize,
}

impl SensoryRoleLearner {
    fn observe(&mut self, episode: &RawEpisode) {
        let signatures = structural_signatures(episode);
        let mut unique: Vec<_> = episode
            .nodes
            .iter()
            .filter(|node| node.identity.is_some())
            .map(|node| signatures[&node.receptor])
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        unique.sort_unstable();
        for signature in unique {
            if let Some(pattern) = self
                .patterns
                .iter_mut()
                .find(|pattern| pattern.signature == signature)
            {
                pattern.evidence += 1;
                pattern.consolidated |= pattern.evidence >= ROLE_THRESHOLD;
            } else {
                self.patterns.push(RolePattern {
                    cell: self.next_cell,
                    signature,
                    evidence: 1,
                    consolidated: false,
                });
                self.next_cell += 1;
            }
        }
        self.observations += 1;
    }

    fn cell_for_signature(&self, signature: u64) -> Option<usize> {
        self.patterns
            .iter()
            .find(|pattern| pattern.signature == signature)
            .map(|pattern| pattern.cell)
    }

    fn translate(&self, episode: &RawEpisode) -> Option<TranslatedEpisode> {
        let signatures = structural_signatures(episode);
        let index: HashMap<_, _> = episode
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.receptor, index))
            .collect();
        let mut undirected = vec![Vec::new(); episode.nodes.len()];
        for (source, node) in episode.nodes.iter().enumerate() {
            for target in &node.outgoing {
                let target = index[target];
                undirected[source].push(target);
                undirected[target].push(source);
            }
        }

        let mut seen = vec![false; episode.nodes.len()];
        let mut relations = Vec::new();
        let mut query = None;
        for start in 0..episode.nodes.len() {
            if seen[start] {
                continue;
            }
            let mut stack = vec![start];
            seen[start] = true;
            let mut occurrences = Vec::new();
            while let Some(node_index) = stack.pop() {
                let node = &episode.nodes[node_index];
                if let Some(identity) = node.identity {
                    let signature = signatures[&node.receptor];
                    occurrences.push(RoleOccurrence {
                        role_cell: self.cell_for_signature(signature)?,
                        identity,
                    });
                }
                for neighbor in &undirected[node_index] {
                    if !seen[*neighbor] {
                        seen[*neighbor] = true;
                        stack.push(*neighbor);
                    }
                }
            }
            match occurrences.len() {
                1 => {
                    if query.replace(occurrences[0]).is_some() {
                        return None;
                    }
                }
                2 => relations.push(TranslatedRelation { occurrences }),
                _ => return None,
            }
        }
        Some(TranslatedEpisode {
            relations,
            query: query?,
        })
    }

    fn consolidated_cells(&self) -> Vec<usize> {
        let mut cells: Vec<_> = self
            .patterns
            .iter()
            .filter(|pattern| pattern.consolidated)
            .map(|pattern| pattern.cell)
            .collect();
        cells.sort_unstable();
        cells
    }

    fn permanent_receptor_ids(&self) -> usize {
        0
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        let mut patterns = self.patterns.clone();
        patterns.sort_by_key(|pattern| pattern.signature);
        for pattern in patterns {
            fingerprint_mix(&mut hash, pattern.cell as u64);
            fingerprint_mix(&mut hash, pattern.signature);
            fingerprint_mix(&mut hash, pattern.evidence as i64 as u64);
            fingerprint_mix(&mut hash, pattern.consolidated as u64);
        }
        hash
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RoleOccurrence {
    role_cell: usize,
    identity: OpaqueId,
}

#[derive(Clone, Debug)]
struct TranslatedRelation {
    occurrences: Vec<RoleOccurrence>,
}

#[derive(Clone, Debug)]
struct TranslatedEpisode {
    relations: Vec<TranslatedRelation>,
    query: RoleOccurrence,
}

fn roles_for_logical_pair(
    translated: &TranslatedEpisode,
    pair: (OpaqueId, OpaqueId),
) -> Option<(usize, usize)> {
    translated.relations.iter().find_map(|relation| {
        let left = relation
            .occurrences
            .iter()
            .find(|occurrence| occurrence.identity == pair.0)?;
        let right = relation
            .occurrences
            .iter()
            .find(|occurrence| occurrence.identity == pair.1)?;
        Some((left.role_cell, right.role_cell))
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LookupArrow {
    id: usize,
    from: usize,
    to: usize,
    strength: i32,
    uses: usize,
    consolidated: bool,
}

fn execute_lookup(
    episode: &TranslatedEpisode,
    route: LookupArrow,
    query: OpaqueId,
) -> BindingOutcome {
    let mut outputs = HashSet::new();
    for relation in &episode.relations {
        let matches = relation
            .occurrences
            .iter()
            .any(|occurrence| occurrence.role_cell == route.from && occurrence.identity == query);
        if !matches {
            continue;
        }
        for occurrence in &relation.occurrences {
            if occurrence.role_cell == route.to {
                outputs.insert(occurrence.identity);
            }
        }
    }
    match outputs.len() {
        0 => BindingOutcome::NotFound,
        1 => BindingOutcome::Answer(*outputs.iter().next().unwrap()),
        _ => BindingOutcome::Ambiguous,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum InternalRole {
    Result,
    Current,
    Success,
    Apply,
    NoResult,
    Answer,
    Clear,
    Quiet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Unit {
    Sensory(usize),
    Internal(InternalRole),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RouteClass {
    Lookup,
    Feedback,
    Continue,
    Finish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProgramArrow {
    id: usize,
    from: Unit,
    to: Unit,
    strength: i32,
    uses: usize,
    consolidated: bool,
}

fn unit_code(unit: Unit) -> u64 {
    match unit {
        Unit::Sensory(cell) => cell as u64,
        Unit::Internal(role) => 1_000 + role as u64,
    }
}

#[derive(Clone, Copy, Debug)]
struct ProgramChoices {
    lookup: ProgramArrow,
    feedback: ProgramArrow,
    continuation: ProgramArrow,
    finish: ProgramArrow,
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
    activity_limit_hit: bool,
    explicit_answer: bool,
    queue_empty: bool,
    used_arrows: Vec<usize>,
}

fn execute_program(episode: &TranslatedEpisode, choices: ProgramChoices) -> ExecutionResult {
    let mut queue = VecDeque::from([Event::Start]);
    let mut current = Some(episode.query.identity);
    let mut answer = None;
    let mut fault = None;
    let mut spikes = 0;
    let mut used_arrows = Vec::new();
    let mut activity_limit_hit = false;

    while let Some(event) = queue.pop_front() {
        spikes += 1;
        if spikes >= ACTIVITY_LIMIT {
            activity_limit_hit = true;
            queue.clear();
            break;
        }
        match event {
            Event::Start => queue.push_back(Event::Apply),
            Event::Apply => {
                mark_used(choices.lookup.id, &mut used_arrows);
                let (Unit::Sensory(from), Unit::Sensory(to)) =
                    (choices.lookup.from, choices.lookup.to)
                else {
                    queue.push_back(Event::NoResult);
                    continue;
                };
                let Some(input) = current else {
                    queue.push_back(Event::NoResult);
                    continue;
                };
                let route = LookupArrow {
                    id: choices.lookup.id,
                    from,
                    to,
                    strength: choices.lookup.strength,
                    uses: choices.lookup.uses,
                    consolidated: choices.lookup.consolidated,
                };
                spikes += episode.relations.len();
                match execute_lookup(episode, route, input) {
                    BindingOutcome::Answer(output) => queue.push_back(Event::Result(output)),
                    BindingOutcome::NotFound => queue.push_back(Event::NoResult),
                    BindingOutcome::Ambiguous => {
                        fault = Some(BindingOutcome::Ambiguous);
                        queue.clear();
                    }
                }
            }
            Event::Result(identity) => {
                mark_used(choices.feedback.id, &mut used_arrows);
                match choices.feedback.to {
                    Unit::Internal(InternalRole::Current) => current = Some(identity),
                    Unit::Internal(InternalRole::Answer) => {
                        queue.push_back(Event::Answer(identity))
                    }
                    Unit::Internal(InternalRole::Apply) => queue.push_back(Event::Apply),
                    Unit::Internal(InternalRole::Clear) => queue.push_back(Event::Clear),
                    _ => queue.push_back(Event::Quiet),
                }
                if choices.feedback.to != Unit::Internal(InternalRole::Answer) {
                    queue.push_back(Event::Success);
                }
            }
            Event::Success => {
                mark_used(choices.continuation.id, &mut used_arrows);
                match choices.continuation.to {
                    Unit::Internal(InternalRole::Apply) => queue.push_back(Event::Apply),
                    Unit::Internal(InternalRole::Answer) => {
                        if let Some(identity) = current {
                            queue.push_back(Event::Answer(identity));
                        }
                    }
                    Unit::Internal(InternalRole::Clear) => queue.push_back(Event::Clear),
                    _ => queue.push_back(Event::Quiet),
                }
            }
            Event::NoResult => {
                mark_used(choices.finish.id, &mut used_arrows);
                match choices.finish.to {
                    Unit::Internal(InternalRole::Answer) => {
                        if let Some(identity) = current {
                            queue.push_back(Event::Answer(identity));
                        }
                    }
                    Unit::Internal(InternalRole::Apply) => queue.push_back(Event::Apply),
                    Unit::Internal(InternalRole::Clear) => queue.push_back(Event::Clear),
                    _ => queue.push_back(Event::Quiet),
                }
            }
            Event::Answer(identity) => {
                answer = Some(identity);
                queue.clear();
            }
            Event::Clear => {
                current = None;
                queue.clear();
            }
            Event::Quiet => queue.clear(),
        }
    }

    ExecutionResult {
        outcome: fault
            .unwrap_or_else(|| answer.map_or(BindingOutcome::NotFound, BindingOutcome::Answer)),
        activity_limit_hit,
        explicit_answer: answer.is_some(),
        queue_empty: queue.is_empty(),
        used_arrows,
    }
}

fn mark_used(arrow: usize, used: &mut Vec<usize>) {
    if !used.contains(&arrow) {
        used.push(arrow);
    }
}

#[derive(Clone, Debug)]
pub struct RoleDiscoveryReport {
    pub successful_seeds: usize,
    pub total_seeds: usize,
    pub learned_role_cells: usize,
    pub transferred_encodings: usize,
    pub transferred_total: usize,
    pub permanent_receptor_ids: usize,
    pub fingerprints_unchanged: bool,
    pub symmetric_field_roles_distinct: bool,
    pub passed: bool,
}

fn run_role_discovery() -> RoleDiscoveryReport {
    let mut successful_seeds = 0;
    let mut transferred = 0;
    let mut transferred_total = 0;
    let mut receptor_ids = 0;
    let mut fingerprints_unchanged = true;
    let mut role_cells = 0;
    for seed in 0..SEEDS {
        let mut learner = SensoryRoleLearner::default();
        let mut ids = IdentitySource::new(0x8100 + seed as u64);
        let mut rng = DeterministicRng::new(0x8200 + seed as u64);
        for episode_index in 0..16 {
            let episode = chain_episode(&mut ids, &mut rng, 1, 8);
            learner.observe(&encode_episode(
                &episode,
                EncodingFamily::Training,
                0x8300 + seed as u64 * 100 + episode_index,
            ));
        }
        role_cells = learner.consolidated_cells().len();
        receptor_ids += learner.permanent_receptor_ids();
        let before = learner.fingerprint();
        let mut seed_correct = true;
        for episode_index in 0..32 {
            let episode = chain_episode(&mut ids, &mut rng, 1, 8);
            let raw = encode_episode(
                &episode,
                EncodingFamily::Transferred,
                0x8400 + seed as u64 * 100 + episode_index,
            );
            let translated = learner.translate(&raw).unwrap();
            let roles = roles_for_logical_pair(&translated, episode.relations[0]).unwrap();
            let query_role = translated.query.role_cell;
            let correct = roles.0 != roles.1
                && roles.0 != query_role
                && roles.1 != query_role
                && learner.consolidated_cells().contains(&roles.0)
                && learner.consolidated_cells().contains(&roles.1);
            transferred += usize::from(correct);
            transferred_total += 1;
            seed_correct &= correct;
        }
        fingerprints_unchanged &= before == learner.fingerprint();
        successful_seeds += usize::from(seed_correct && role_cells == 3);
    }

    let mut symmetric = SensoryRoleLearner::default();
    let mut ids = IdentitySource::new(0x8500);
    let mut rng = DeterministicRng::new(0x8501);
    let mut field_roles_distinct = false;
    for episode_index in 0..16 {
        let episode = chain_episode(&mut ids, &mut rng, 1, 8);
        let raw = encode_episode(&episode, EncodingFamily::Symmetric, 0x8502 + episode_index);
        symmetric.observe(&raw);
        let translated = symmetric.translate(&raw).unwrap();
        let pair = roles_for_logical_pair(&translated, episode.relations[0]).unwrap();
        field_roles_distinct |= pair.0 != pair.1;
    }

    RoleDiscoveryReport {
        successful_seeds,
        total_seeds: SEEDS,
        learned_role_cells: role_cells,
        transferred_encodings: transferred,
        transferred_total,
        permanent_receptor_ids: receptor_ids,
        fingerprints_unchanged,
        symmetric_field_roles_distinct: field_roles_distinct,
        passed: successful_seeds == SEEDS
            && transferred == transferred_total
            && receptor_ids == 0
            && fingerprints_unchanged
            && !field_roles_distinct,
    }
}

fn trained_role_learner(seed: usize) -> SensoryRoleLearner {
    let mut learner = SensoryRoleLearner::default();
    let mut ids = IdentitySource::new(0x8600 + seed as u64);
    let mut rng = DeterministicRng::new(0x8610 + seed as u64);
    for episode_index in 0..16 {
        let episode = chain_episode(&mut ids, &mut rng, 1, 8);
        learner.observe(&encode_episode(
            &episode,
            EncodingFamily::Training,
            0x8620 + seed as u64 * 100 + episode_index,
        ));
    }
    learner
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FeedbackMode {
    Real,
    Shuffled,
    Random,
}

fn average(values: &[usize]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<usize>() as f64 / values.len() as f64)
}

fn fingerprint_mix(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(0x100_0000_01b3);
    }
}

const P2_PRIMARY_SLOTS: usize = 8;
const P2_PRIMARY_DISTRACTORS: usize = 8;
const P2_INTEGRATED_BUDGET: usize = 50_000;
const P2_PROBATION_EPISODES: usize = 6;
const P2_PRUNE_STRENGTH: i32 = -2;
const P2_ELIGIBILITY_CAPACITY: usize = 64;
const P2_LOCAL_RADIUS: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum PlasticUnit {
    Program(Unit),
    Irrelevant(usize),
}

impl PlasticUnit {
    fn program(self) -> Option<Unit> {
        match self {
            Self::Program(unit) => Some(unit),
            Self::Irrelevant(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct EncounterSignature {
    first: u64,
    second: u64,
}

#[derive(Clone, Copy, Debug)]
struct EncounterContext {
    left: PlasticUnit,
    right: PlasticUnit,
    signature: EncounterSignature,
    snapshot: PreCouplingSnapshot,
    value_context: EncounterValueContext,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EncounterAdmission {
    admit: bool,
    exploratory: bool,
}

impl EncounterSignature {
    fn between(left: PlasticUnit, right: PlasticUnit) -> Self {
        let mut features = [plastic_unit_feature(left), plastic_unit_feature(right)];
        features.sort_unstable();
        Self {
            first: features[0],
            second: features[1],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct EndpointSnapshot {
    persists_across_episodes: bool,
    receives_external_activity: bool,
    receives_queued_activity: bool,
    is_temporary_activity: bool,
}

impl EndpointSnapshot {
    fn from_unit(unit: PlasticUnit) -> Self {
        match unit {
            PlasticUnit::Program(Unit::Sensory(_)) => Self {
                persists_across_episodes: true,
                receives_external_activity: true,
                receives_queued_activity: false,
                is_temporary_activity: false,
            },
            PlasticUnit::Program(Unit::Internal(_)) => Self {
                persists_across_episodes: true,
                receives_external_activity: false,
                receives_queued_activity: true,
                is_temporary_activity: false,
            },
            PlasticUnit::Irrelevant(_) => Self {
                persists_across_episodes: false,
                receives_external_activity: false,
                receives_queued_activity: false,
                is_temporary_activity: true,
            },
        }
    }

    fn code(self) -> u8 {
        u8::from(self.persists_across_episodes)
            | (u8::from(self.receives_external_activity) << 1)
            | (u8::from(self.receives_queued_activity) << 2)
            | (u8::from(self.is_temporary_activity) << 3)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct PreCouplingSnapshot {
    first: EndpointSnapshot,
    second: EndpointSnapshot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct EndpointValueContext {
    occupied_slots: u8,
    has_consolidated_outgoing: bool,
    has_consolidated_incoming: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct EncounterValueContext {
    first: EndpointValueContext,
    second: EndpointValueContext,
}

impl PreCouplingSnapshot {
    fn between(left: PlasticUnit, right: PlasticUnit) -> Self {
        let mut endpoints = [
            EndpointSnapshot::from_unit(left),
            EndpointSnapshot::from_unit(right),
        ];
        endpoints.sort_unstable();
        Self {
            first: endpoints[0],
            second: endpoints[1],
        }
    }

    fn code(self) -> u16 {
        u16::from(self.first.code()) | (u16::from(self.second.code()) << 8)
    }
}

fn plastic_unit_feature(unit: PlasticUnit) -> u64 {
    match unit {
        PlasticUnit::Program(Unit::Sensory(_)) => 100,
        PlasticUnit::Program(Unit::Internal(_)) => 1_000,
        PlasticUnit::Irrelevant(_) => 10_000,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PlasticArrow {
    id: usize,
    from: PlasticUnit,
    to: PlasticUnit,
    strength: i32,
    uses: usize,
    consolidated: bool,
    probation_left: usize,
    last_touched: usize,
    opportunity: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PlasticityOutcome {
    snapshot: PreCouplingSnapshot,
    context: EncounterValueContext,
    exploratory: bool,
    useful: bool,
}

#[derive(Clone, Debug)]
struct PlasticOpportunity {
    snapshot: PreCouplingSnapshot,
    context: EncounterValueContext,
    exploratory: bool,
    arrows: HashSet<usize>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct EligibilityEntry {
    arrow: usize,
    uses: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlasticityMetrics {
    pub active_relevant_touches: usize,
    pub active_irrelevant_touches: usize,
    pub dormant_touches: usize,
    pub local_encounters: usize,
    pub directional_couplings_created: usize,
    pub directional_couplings_released: usize,
    pub eligibility_registrations: usize,
    pub eligibility_updates: usize,
    pub eligibility_evictions: usize,
    pub ever_used_couplings: usize,
    pub surviving_couplings: usize,
    pub peak_probationary_couplings: usize,
    pub gate_evaluations: usize,
    pub gate_admissions: usize,
}

#[derive(Clone, Debug)]
struct LocalPlasticity {
    arrows: HashMap<usize, PlasticArrow>,
    slots: HashMap<PlasticUnit, Vec<usize>>,
    next_id: usize,
    next_opportunity_id: usize,
    slot_count: usize,
    episode: usize,
    rng: DeterministicRng,
    eligibility: VecDeque<EligibilityEntry>,
    ever_used: HashSet<usize>,
    encountered_signatures: HashMap<EncounterSignature, usize>,
    track_plasticity_outcomes: bool,
    opportunities: HashMap<usize, PlasticOpportunity>,
    pending_outcomes: Vec<PlasticityOutcome>,
    metrics: PlasticityMetrics,
}

impl LocalPlasticity {
    fn new(seed: u64, slot_count: usize) -> Self {
        Self::with_outcome_tracking(seed, slot_count, false)
    }

    fn with_outcome_tracking(seed: u64, slot_count: usize, track_outcomes: bool) -> Self {
        Self {
            arrows: HashMap::new(),
            slots: HashMap::new(),
            next_id: 0,
            next_opportunity_id: 0,
            slot_count,
            episode: 0,
            rng: DeterministicRng::new(seed),
            eligibility: VecDeque::new(),
            ever_used: HashSet::new(),
            encountered_signatures: HashMap::new(),
            track_plasticity_outcomes: track_outcomes,
            opportunities: HashMap::new(),
            pending_outcomes: Vec::new(),
            metrics: PlasticityMetrics::default(),
        }
    }

    fn expose_activity(&mut self, sensory_cells: &[usize], active_irrelevant: usize) {
        self.expose_activity_with_gate(sensory_cells, active_irrelevant, |_| true);
    }

    fn expose_activity_with_gate(
        &mut self,
        sensory_cells: &[usize],
        active_irrelevant: usize,
        mut admit: impl FnMut(EncounterContext) -> bool,
    ) {
        self.expose_activity_with_admission_gate(sensory_cells, active_irrelevant, |context| {
            EncounterAdmission {
                admit: admit(context),
                exploratory: false,
            }
        });
    }

    fn expose_activity_with_admission_gate(
        &mut self,
        sensory_cells: &[usize],
        active_irrelevant: usize,
        mut decide: impl FnMut(EncounterContext) -> EncounterAdmission,
    ) {
        self.episode += 1;
        let mut active: Vec<_> = sensory_cells
            .iter()
            .copied()
            .map(Unit::Sensory)
            .chain(
                [
                    InternalRole::Result,
                    InternalRole::Current,
                    InternalRole::Success,
                    InternalRole::Apply,
                    InternalRole::NoResult,
                    InternalRole::Answer,
                    InternalRole::Clear,
                    InternalRole::Quiet,
                ]
                .into_iter()
                .map(Unit::Internal),
            )
            .map(PlasticUnit::Program)
            .collect();
        active.extend((0..active_irrelevant).map(PlasticUnit::Irrelevant));
        self.metrics.active_relevant_touches += sensory_cells.len() + 8;
        self.metrics.active_irrelevant_touches += active_irrelevant;
        self.rng.shuffle(&mut active);
        self.age_active(&active);

        for source_index in 0..active.len() {
            for offset in 1..=P2_LOCAL_RADIUS.min(active.len().saturating_sub(1)) {
                let target_index = (source_index + offset) % active.len();
                self.metrics.local_encounters += 1;
                let left = active[source_index];
                let right = active[target_index];
                let signature = EncounterSignature::between(left, right);
                let snapshot = PreCouplingSnapshot::between(left, right);
                let value_context = self.value_context(left, right);
                *self.encountered_signatures.entry(signature).or_default() += 1;
                let needs_growth = self.find_arrow(left, right).is_none()
                    || self.find_arrow(right, left).is_none();
                if !needs_growth {
                    continue;
                }
                self.metrics.gate_evaluations += 1;
                let admission = decide(EncounterContext {
                    left,
                    right,
                    signature,
                    snapshot,
                    value_context,
                });
                if admission.admit {
                    self.metrics.gate_admissions += 1;
                    self.open_opportunity(
                        left,
                        right,
                        snapshot,
                        value_context,
                        admission.exploratory,
                    );
                }
            }
        }
        self.update_peak();
    }

    fn age_active(&mut self, active: &[PlasticUnit]) {
        let mut release = Vec::new();
        for source in active {
            let Some(slots) = self.slots.get(source) else {
                continue;
            };
            for id in slots {
                let Some(arrow) = self.arrows.get_mut(id) else {
                    continue;
                };
                if arrow.consolidated {
                    continue;
                }
                if arrow.probation_left > 0 {
                    arrow.probation_left -= 1;
                }
                if arrow.probation_left == 0 && arrow.uses == 0 {
                    release.push(*id);
                }
            }
        }
        release.sort_unstable();
        release.dedup();
        for id in release {
            self.release(id);
        }
    }

    fn open_opportunity(
        &mut self,
        left: PlasticUnit,
        right: PlasticUnit,
        snapshot: PreCouplingSnapshot,
        context: EncounterValueContext,
        exploratory: bool,
    ) {
        if !self.track_plasticity_outcomes {
            self.open_direction(left, right, None);
            self.open_direction(right, left, None);
            return;
        }
        let opportunity = self.next_opportunity_id;
        self.next_opportunity_id += 1;
        self.opportunities.insert(
            opportunity,
            PlasticOpportunity {
                snapshot,
                context,
                exploratory,
                arrows: HashSet::new(),
            },
        );
        let first = self.open_direction(left, right, Some(opportunity));
        let second = self.open_direction(right, left, Some(opportunity));
        if first.is_none() && second.is_none() {
            self.opportunities.remove(&opportunity);
        }
    }

    fn endpoint_value_context(&self, unit: PlasticUnit) -> EndpointValueContext {
        let occupied_slots = self.slots.get(&unit).map_or(0, Vec::len).min(2) as u8;
        let has_consolidated_outgoing = self.slots.get(&unit).is_some_and(|slots| {
            slots
                .iter()
                .any(|id| self.arrows.get(id).is_some_and(|arrow| arrow.consolidated))
        });
        let has_consolidated_incoming = self
            .arrows
            .values()
            .any(|arrow| arrow.to == unit && arrow.consolidated);
        EndpointValueContext {
            occupied_slots,
            has_consolidated_outgoing,
            has_consolidated_incoming,
        }
    }

    fn value_context(&self, left: PlasticUnit, right: PlasticUnit) -> EncounterValueContext {
        let mut endpoints = [
            (
                EndpointSnapshot::from_unit(left),
                self.endpoint_value_context(left),
            ),
            (
                EndpointSnapshot::from_unit(right),
                self.endpoint_value_context(right),
            ),
        ];
        endpoints.sort_unstable();
        EncounterValueContext {
            first: endpoints[0].1,
            second: endpoints[1].1,
        }
    }

    fn open_direction(
        &mut self,
        from: PlasticUnit,
        to: PlasticUnit,
        opportunity: Option<usize>,
    ) -> Option<usize> {
        if from == to || self.find_arrow(from, to).is_some() {
            return None;
        }
        if self.slot_count == 0 {
            return None;
        }
        let occupied = self.slots.get(&from).map_or(0, Vec::len);
        if occupied >= self.slot_count {
            let replace = self.slots.get(&from).and_then(|ids| {
                ids.iter()
                    .filter_map(|id| self.arrows.get(id))
                    .filter(|arrow| !arrow.consolidated)
                    .min_by_key(|arrow| {
                        (
                            arrow.strength,
                            usize::from(arrow.uses > 0),
                            arrow.last_touched,
                        )
                    })
                    .map(|arrow| arrow.id)
            });
            let replace = replace?;
            self.release(replace);
        }
        let id = self.next_id;
        self.next_id += 1;
        self.arrows.insert(
            id,
            PlasticArrow {
                id,
                from,
                to,
                strength: 0,
                uses: 0,
                consolidated: false,
                probation_left: P2_PROBATION_EPISODES,
                last_touched: self.episode,
                opportunity,
            },
        );
        self.slots.entry(from).or_default().push(id);
        if let Some(opportunity) = opportunity {
            if let Some(trace) = self.opportunities.get_mut(&opportunity) {
                trace.arrows.insert(id);
            }
        }
        self.metrics.directional_couplings_created += 1;
        Some(id)
    }

    fn release(&mut self, id: usize) {
        let Some(arrow) = self.arrows.remove(&id) else {
            return;
        };
        if let Some(slots) = self.slots.get_mut(&arrow.from) {
            slots.retain(|slot| *slot != id);
        }
        self.eligibility.retain(|entry| entry.arrow != id);
        self.metrics.directional_couplings_released += 1;
        if let Some(opportunity) = arrow.opportunity {
            let mut rejected = None;
            if let Some(trace) = self.opportunities.get_mut(&opportunity) {
                trace.arrows.remove(&id);
                if trace.arrows.is_empty() {
                    rejected = Some((trace.snapshot, trace.context, trace.exploratory));
                }
            }
            if let Some((snapshot, context, exploratory)) = rejected {
                self.opportunities.remove(&opportunity);
                self.pending_outcomes.push(PlasticityOutcome {
                    snapshot,
                    context,
                    exploratory,
                    useful: false,
                });
            }
        }
    }

    fn find_arrow(&self, from: PlasticUnit, to: PlasticUnit) -> Option<usize> {
        self.slots.get(&from)?.iter().find_map(|id| {
            self.arrows
                .get(id)
                .filter(|arrow| arrow.to == to)
                .map(|arrow| arrow.id)
        })
    }

    fn class(arrow: PlasticArrow) -> Option<RouteClass> {
        let from = arrow.from.program()?;
        let to = arrow.to.program()?;
        match (from, to) {
            (Unit::Sensory(_), Unit::Sensory(_)) => Some(RouteClass::Lookup),
            (Unit::Internal(InternalRole::Result), _) => Some(RouteClass::Feedback),
            (Unit::Internal(InternalRole::Success), _) => Some(RouteClass::Continue),
            (Unit::Internal(InternalRole::NoResult), _) => Some(RouteClass::Finish),
            _ => None,
        }
    }

    fn candidates(&self, class: RouteClass) -> Vec<PlasticArrow> {
        let mut candidates: Vec<_> = self
            .arrows
            .values()
            .copied()
            .filter(|arrow| Self::class(*arrow) == Some(class))
            .collect();
        candidates.sort_by_key(|arrow| arrow.id);
        candidates
    }

    fn choose(&mut self, class: RouteClass) -> Option<PlasticArrow> {
        let candidates = self.candidates(class);
        if let Some(arrow) = candidates.iter().find(|arrow| arrow.consolidated) {
            return Some(*arrow);
        }
        let best = candidates.iter().map(|arrow| arrow.strength).max()?;
        let choices: Vec<_> = candidates
            .iter()
            .filter(|arrow| arrow.strength == best)
            .map(|arrow| arrow.id)
            .collect();
        let id = choices[self.rng.index(choices.len())];
        self.arrows.get(&id).copied()
    }

    fn evaluated(&self, class: RouteClass) -> Option<PlasticArrow> {
        let candidates = self.candidates(class);
        if let Some(arrow) = candidates.iter().find(|arrow| arrow.consolidated) {
            return Some(*arrow);
        }
        let best = candidates.iter().map(|arrow| arrow.strength).max()?;
        let strongest: Vec<_> = candidates
            .iter()
            .filter(|arrow| arrow.strength == best && best > 0)
            .collect();
        if strongest.len() == 1 {
            Some(*strongest[0])
        } else {
            None
        }
    }

    fn choices(&mut self) -> Option<ProgramChoices> {
        Some(ProgramChoices {
            lookup: self.choose(RouteClass::Lookup)?.as_program()?,
            feedback: self.choose(RouteClass::Feedback)?.as_program()?,
            continuation: self.choose(RouteClass::Continue)?.as_program()?,
            finish: self.choose(RouteClass::Finish)?.as_program()?,
        })
    }

    fn evaluated_choices(&self) -> Option<ProgramChoices> {
        Some(ProgramChoices {
            lookup: self.evaluated(RouteClass::Lookup)?.as_program()?,
            feedback: self.evaluated(RouteClass::Feedback)?.as_program()?,
            continuation: self.evaluated(RouteClass::Continue)?.as_program()?,
            finish: self.evaluated(RouteClass::Finish)?.as_program()?,
        })
    }

    fn register_used(&mut self, used: &[usize]) {
        for id in used {
            let Some(arrow) = self.arrows.get_mut(id) else {
                continue;
            };
            arrow.uses += 1;
            arrow.last_touched = self.episode;
            arrow.probation_left = P2_PROBATION_EPISODES;
            self.ever_used.insert(*id);
            if let Some(entry) = self.eligibility.iter_mut().find(|entry| entry.arrow == *id) {
                entry.uses += 1;
            } else {
                if self.eligibility.len() == P2_ELIGIBILITY_CAPACITY {
                    self.eligibility.pop_front();
                    self.metrics.eligibility_evictions += 1;
                }
                self.eligibility.push_back(EligibilityEntry {
                    arrow: *id,
                    uses: 1,
                });
                self.metrics.eligibility_registrations += 1;
            }
        }
    }

    fn terminal_feedback(&mut self, success: bool) {
        let entries: Vec<_> = self.eligibility.drain(..).collect();
        let mut consolidate = Vec::new();
        let mut release = Vec::new();
        let mut useful_opportunities = Vec::new();
        for entry in entries {
            let Some(arrow) = self.arrows.get_mut(&entry.arrow) else {
                continue;
            };
            self.metrics.eligibility_updates += 1;
            if success {
                if let Some(opportunity) = arrow.opportunity {
                    useful_opportunities.push(opportunity);
                }
            }
            if !arrow.consolidated {
                arrow.strength += if success {
                    SUCCESS_CREDIT
                } else {
                    FAILURE_CREDIT
                };
            }
            if arrow.strength >= CONSOLIDATION_STRENGTH {
                consolidate.push(arrow.id);
            } else if arrow.strength <= P2_PRUNE_STRENGTH {
                release.push(arrow.id);
            }
        }
        useful_opportunities.sort_unstable();
        useful_opportunities.dedup();
        for opportunity in useful_opportunities {
            self.report_useful(opportunity);
        }
        for id in release {
            self.release(id);
        }
        for id in consolidate {
            self.consolidate(id);
        }
        if self.complete() {
            self.release_unconsolidated();
        }
        self.update_peak();
    }

    fn report_useful(&mut self, opportunity: usize) {
        let Some(trace) = self.opportunities.remove(&opportunity) else {
            return;
        };
        for arrow in &trace.arrows {
            if let Some(arrow) = self.arrows.get_mut(arrow) {
                arrow.opportunity = None;
            }
        }
        self.pending_outcomes.push(PlasticityOutcome {
            snapshot: trace.snapshot,
            context: trace.context,
            exploratory: trace.exploratory,
            useful: true,
        });
    }

    fn drain_plasticity_outcomes(&mut self) -> Vec<PlasticityOutcome> {
        std::mem::take(&mut self.pending_outcomes)
    }

    fn consolidate(&mut self, id: usize) {
        let Some(class) = self.arrows.get(&id).copied().and_then(Self::class) else {
            return;
        };
        if self
            .arrows
            .values()
            .any(|arrow| arrow.consolidated && Self::class(*arrow) == Some(class))
        {
            return;
        }
        if let Some(arrow) = self.arrows.get_mut(&id) {
            arrow.consolidated = true;
        }
        let competitors: Vec<_> = self
            .arrows
            .values()
            .filter(|arrow| arrow.id != id && Self::class(**arrow) == Some(class))
            .map(|arrow| arrow.id)
            .collect();
        for competitor in competitors {
            self.release(competitor);
        }
    }

    fn complete(&self) -> bool {
        [
            RouteClass::Lookup,
            RouteClass::Feedback,
            RouteClass::Continue,
            RouteClass::Finish,
        ]
        .into_iter()
        .all(|class| {
            self.arrows
                .values()
                .any(|arrow| arrow.consolidated && Self::class(*arrow) == Some(class))
        })
    }

    fn consolidated_count(&self) -> usize {
        self.arrows
            .values()
            .filter(|arrow| arrow.consolidated)
            .count()
    }

    fn release_unconsolidated(&mut self) {
        let release: Vec<_> = self
            .arrows
            .values()
            .filter(|arrow| !arrow.consolidated)
            .map(|arrow| arrow.id)
            .collect();
        for id in release {
            self.release(id);
        }
    }

    fn update_peak(&mut self) {
        let probationary = self
            .arrows
            .values()
            .filter(|arrow| !arrow.consolidated)
            .count();
        self.metrics.peak_probationary_couplings =
            self.metrics.peak_probationary_couplings.max(probationary);
        self.metrics.ever_used_couplings = self.ever_used.len();
        self.metrics.surviving_couplings = self.arrows.len();
    }

    fn fingerprint(&self) -> u64 {
        let mut arrows: Vec<_> = self.arrows.values().copied().collect();
        arrows.sort_by_key(|arrow| (arrow.from, arrow.to));
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for arrow in arrows {
            fingerprint_mix(&mut hash, plastic_unit_code(arrow.from));
            fingerprint_mix(&mut hash, plastic_unit_code(arrow.to));
            fingerprint_mix(&mut hash, arrow.strength as i64 as u64);
            fingerprint_mix(&mut hash, arrow.consolidated as u64);
        }
        hash
    }

    fn reset_metrics(&mut self) {
        self.metrics = PlasticityMetrics::default();
    }

    fn useful_signatures(&self) -> HashSet<EncounterSignature> {
        self.arrows
            .values()
            .filter(|arrow| arrow.consolidated)
            .map(|arrow| EncounterSignature::between(arrow.from, arrow.to))
            .collect()
    }

    fn useful_pairs(&self) -> HashSet<(PlasticUnit, PlasticUnit)> {
        self.arrows
            .values()
            .filter(|arrow| arrow.consolidated)
            .map(|arrow| ordered_pair(arrow.from, arrow.to))
            .collect()
    }
}

fn ordered_pair(first: PlasticUnit, second: PlasticUnit) -> (PlasticUnit, PlasticUnit) {
    if first <= second {
        (first, second)
    } else {
        (second, first)
    }
}

impl PlasticArrow {
    fn as_program(self) -> Option<ProgramArrow> {
        Some(ProgramArrow {
            id: self.id,
            from: self.from.program()?,
            to: self.to.program()?,
            strength: self.strength,
            uses: self.uses,
            consolidated: self.consolidated,
        })
    }
}

fn plastic_unit_code(unit: PlasticUnit) -> u64 {
    match unit {
        PlasticUnit::Program(unit) => unit_code(unit),
        PlasticUnit::Irrelevant(id) => 10_000 + id as u64,
    }
}

#[derive(Clone, Debug)]
pub struct P2LookupReport {
    pub forward_seeds: usize,
    pub reverse_seeds: usize,
    pub total_seeds: usize,
    pub random_feedback_stable: bool,
    pub average_created: usize,
    pub average_used: usize,
    pub average_surviving: usize,
    pub passed: bool,
}

fn train_local_lookup(seed: usize, reverse: bool, slots: usize) -> (bool, PlasticityMetrics) {
    train_local_lookup_with_gate(seed, reverse, slots, &mut |_| true)
}

fn train_local_lookup_with_gate(
    seed: usize,
    reverse: bool,
    slots: usize,
    admit: &mut impl FnMut(EncounterContext) -> bool,
) -> (bool, PlasticityMetrics) {
    let roles = trained_role_learner(seed);
    let cells = roles.consolidated_cells();
    let mut network = LocalPlasticity::new(0xa100 + seed as u64 * 100 + reverse as u64, slots);
    let mut ids = IdentitySource::new(0xa200 + seed as u64 + reverse as u64 * 100);
    let mut rng = DeterministicRng::new(0xa300 + seed as u64 + reverse as u64 * 100);
    for episode_index in 0..10_000 {
        network.expose_activity_with_gate(&cells, 4, &mut *admit);
        let Some(route) = network.choose(RouteClass::Lookup) else {
            continue;
        };
        let mut episode = chain_episode(&mut ids, &mut rng, 1, 8);
        if reverse {
            let pair = episode.relations[0];
            episode.query = pair.1;
            episode.correct = BindingOutcome::Answer(pair.0);
        }
        let raw = encode_episode(
            &episode,
            EncodingFamily::Training,
            0xa400 + seed as u64 * 20_000 + episode_index,
        );
        let translated = roles.translate(&raw).unwrap();
        let program = route.as_program().unwrap();
        let lookup = LookupArrow {
            id: program.id,
            from: match program.from {
                Unit::Sensory(cell) => cell,
                Unit::Internal(_) => unreachable!(),
            },
            to: match program.to {
                Unit::Sensory(cell) => cell,
                Unit::Internal(_) => unreachable!(),
            },
            strength: program.strength,
            uses: program.uses,
            consolidated: program.consolidated,
        };
        let outcome = execute_lookup(&translated, lookup, episode.query);
        network.register_used(&[route.id]);
        network.terminal_feedback(outcome == episode.correct);
        if network
            .evaluated(RouteClass::Lookup)
            .is_some_and(|arrow| arrow.consolidated)
        {
            break;
        }
    }
    let learned = network
        .evaluated(RouteClass::Lookup)
        .and_then(PlasticArrow::as_program);
    let mut correct = 0;
    if let Some(route) = learned {
        for episode_index in 0..32 {
            let mut episode = chain_episode(&mut ids, &mut rng, 1, 8);
            if reverse {
                let pair = episode.relations[0];
                episode.query = pair.1;
                episode.correct = BindingOutcome::Answer(pair.0);
            }
            let raw = encode_episode(
                &episode,
                EncodingFamily::Transferred,
                0xa500 + seed as u64 * 100 + episode_index,
            );
            let translated = roles.translate(&raw).unwrap();
            let lookup = LookupArrow {
                id: route.id,
                from: match route.from {
                    Unit::Sensory(cell) => cell,
                    Unit::Internal(_) => unreachable!(),
                },
                to: match route.to {
                    Unit::Sensory(cell) => cell,
                    Unit::Internal(_) => unreachable!(),
                },
                strength: route.strength,
                uses: route.uses,
                consolidated: route.consolidated,
            };
            correct +=
                usize::from(execute_lookup(&translated, lookup, episode.query) == episode.correct);
        }
    }
    if learned.is_some_and(|route| route.consolidated) {
        network.release_unconsolidated();
    }
    network.update_peak();
    (correct == 32, network.metrics)
}

fn local_lookup_random_control() -> bool {
    let roles = trained_role_learner(70);
    let cells = roles.consolidated_cells();
    let mut network = LocalPlasticity::new(0xa600, P2_PRIMARY_SLOTS);
    let mut rng = DeterministicRng::new(0xa601);
    for _ in 0..5_000 {
        network.expose_activity(&cells, 4);
        let Some(route) = network.choose(RouteClass::Lookup) else {
            continue;
        };
        network.register_used(&[route.id]);
        network.terminal_feedback(rng.next_u64().is_multiple_of(4));
    }
    network
        .evaluated(RouteClass::Lookup)
        .is_some_and(|arrow| arrow.consolidated)
}

fn run_p2_lookup() -> P2LookupReport {
    let mut forward = 0;
    let mut reverse = 0;
    let mut created = 0;
    let mut used = 0;
    let mut surviving = 0;
    for seed in 0..SEEDS {
        let (forward_ok, forward_metrics) = train_local_lookup(seed, false, P2_PRIMARY_SLOTS);
        let (reverse_ok, _) = train_local_lookup(seed, true, P2_PRIMARY_SLOTS);
        forward += usize::from(forward_ok);
        reverse += usize::from(reverse_ok);
        created += forward_metrics.directional_couplings_created;
        used += forward_metrics.ever_used_couplings;
        surviving += forward_metrics.surviving_couplings;
    }
    let random_feedback_stable = local_lookup_random_control();
    P2LookupReport {
        forward_seeds: forward,
        reverse_seeds: reverse,
        total_seeds: SEEDS,
        random_feedback_stable,
        average_created: created / SEEDS,
        average_used: used / SEEDS,
        average_surviving: surviving / SEEDS,
        passed: forward == SEEDS && reverse == SEEDS && !random_feedback_stable,
    }
}

#[derive(Clone, Debug)]
struct P2IntegratedSeed {
    competent: bool,
    held_out_correct: usize,
    held_out_total: usize,
    roles: usize,
    surviving_program: usize,
    fingerprint_unchanged: bool,
    explicit_answers: bool,
    queues_empty: bool,
    first_success_episode: Option<usize>,
    competence_episode: Option<usize>,
    metrics: PlasticityMetrics,
    network: LocalPlasticity,
}

fn run_p2_integrated_seed(
    seed: usize,
    mode: FeedbackMode,
    active_irrelevant: usize,
    slots: usize,
) -> P2IntegratedSeed {
    run_p2_integrated_seed_with_gate(seed, mode, active_irrelevant, slots, &mut |_| true)
}

fn run_p2_integrated_seed_with_gate(
    seed: usize,
    mode: FeedbackMode,
    active_irrelevant: usize,
    slots: usize,
    admit: &mut impl FnMut(EncounterContext) -> bool,
) -> P2IntegratedSeed {
    let mut roles = SensoryRoleLearner::default();
    let mut network = LocalPlasticity::new(0xb100 + seed as u64, slots);
    let mut ids = IdentitySource::new(0xb200 + seed as u64);
    let mut rng = DeterministicRng::new(0xb300 + seed as u64);
    let mut feedback_rng = DeterministicRng::new(0xb400 + seed as u64);
    let mut previous_success = false;
    let mut first_success = None;
    let mut competence = None;

    for episode_index in 1..=P2_INTEGRATED_BUDGET {
        let depth = TRAIN_DEPTHS[(episode_index - 1) % TRAIN_DEPTHS.len()];
        let episode = chain_episode(&mut ids, &mut rng, depth, 12);
        let raw = encode_episode(
            &episode,
            EncodingFamily::Training,
            0xb500 + seed as u64 * 100_000 + episode_index as u64,
        );
        roles.observe(&raw);
        let Some(translated) = roles.translate(&raw) else {
            continue;
        };
        let cells: Vec<_> = roles.patterns.iter().map(|pattern| pattern.cell).collect();
        network.expose_activity_with_gate(&cells, active_irrelevant, &mut *admit);
        let Some(choices) = network.choices() else {
            continue;
        };
        let run = execute_program(&translated, choices);
        let actual_success =
            run.outcome == episode.correct && !run.activity_limit_hit && run.explicit_answer;
        if actual_success && first_success.is_none() {
            first_success = Some(episode_index);
        }
        network.register_used(&run.used_arrows);
        let credit = match mode {
            FeedbackMode::Real => actual_success,
            FeedbackMode::Shuffled => previous_success,
            FeedbackMode::Random => feedback_rng.next_u64().is_multiple_of(4),
        };
        previous_success = actual_success;
        network.terminal_feedback(credit);
        if competence.is_none() && network.complete() && roles.consolidated_cells().len() == 3 {
            competence = Some(episode_index);
        }
        if competence.is_some() && mode == FeedbackMode::Real {
            break;
        }
    }

    let fingerprint_before = hash_words(&[roles.fingerprint(), network.fingerprint()]);
    let mut held_out_correct = 0;
    let mut held_out_total = 0;
    let mut explicit_answers = true;
    let mut queues_empty = true;
    if let Some(choices) = network.evaluated_choices() {
        let mut heldout_ids = IdentitySource::new(0xb600 + seed as u64);
        let mut heldout_rng = DeterministicRng::new(0xb700 + seed as u64);
        for depth in HELD_OUT_DEPTHS {
            for episode_index in 0..16 {
                let episode = chain_episode(&mut heldout_ids, &mut heldout_rng, depth, depth + 8);
                let raw = encode_episode(
                    &episode,
                    EncodingFamily::Transferred,
                    0xb800 + seed as u64 * 10_000 + depth as u64 * 100 + episode_index,
                );
                let translated = roles.translate(&raw).unwrap();
                let run = execute_program(&translated, choices);
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
    let fingerprint_after = hash_words(&[roles.fingerprint(), network.fingerprint()]);
    network.update_peak();
    P2IntegratedSeed {
        competent: held_out_correct == held_out_total,
        held_out_correct,
        held_out_total,
        roles: roles.consolidated_cells().len(),
        surviving_program: network.consolidated_count(),
        fingerprint_unchanged: fingerprint_before == fingerprint_after,
        explicit_answers,
        queues_empty,
        first_success_episode: first_success,
        competence_episode: competence,
        metrics: network.metrics.clone(),
        network,
    }
}

#[derive(Clone, Debug)]
pub struct P2IntegratedReport {
    pub competent_seeds: usize,
    pub total_seeds: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub average_roles: f64,
    pub average_surviving_program: f64,
    pub fingerprints_unchanged: bool,
    pub explicit_answers: bool,
    pub queues_empty: bool,
    pub average_first_success_episode: Option<f64>,
    pub average_competence_episode: Option<f64>,
    pub average_created: usize,
    pub average_released: usize,
    pub average_used: usize,
    pub average_peak_probationary: usize,
    pub average_eligibility_updates: usize,
    pub total_eligibility_evictions: usize,
}

fn run_p2_integrated_condition(mode: FeedbackMode) -> P2IntegratedReport {
    let results: Vec<_> = (0..SEEDS)
        .map(|seed| run_p2_integrated_seed(seed, mode, P2_PRIMARY_DISTRACTORS, P2_PRIMARY_SLOTS))
        .collect();
    let first: Vec<_> = results
        .iter()
        .filter_map(|result| result.first_success_episode)
        .collect();
    let competence: Vec<_> = results
        .iter()
        .filter_map(|result| result.competence_episode)
        .collect();
    P2IntegratedReport {
        competent_seeds: results.iter().filter(|result| result.competent).count(),
        total_seeds: SEEDS,
        held_out_correct: results.iter().map(|result| result.held_out_correct).sum(),
        held_out_total: results.iter().map(|result| result.held_out_total).sum(),
        average_roles: results.iter().map(|result| result.roles).sum::<usize>() as f64
            / SEEDS as f64,
        average_surviving_program: results
            .iter()
            .map(|result| result.surviving_program)
            .sum::<usize>() as f64
            / SEEDS as f64,
        fingerprints_unchanged: results.iter().all(|result| result.fingerprint_unchanged),
        explicit_answers: results.iter().all(|result| result.explicit_answers),
        queues_empty: results.iter().all(|result| result.queues_empty),
        average_first_success_episode: average(&first),
        average_competence_episode: average(&competence),
        average_created: results
            .iter()
            .map(|result| result.metrics.directional_couplings_created)
            .sum::<usize>()
            / SEEDS,
        average_released: results
            .iter()
            .map(|result| result.metrics.directional_couplings_released)
            .sum::<usize>()
            / SEEDS,
        average_used: results
            .iter()
            .map(|result| result.metrics.ever_used_couplings)
            .sum::<usize>()
            / SEEDS,
        average_peak_probationary: results
            .iter()
            .map(|result| result.metrics.peak_probationary_couplings)
            .sum::<usize>()
            / SEEDS,
        average_eligibility_updates: results
            .iter()
            .map(|result| result.metrics.eligibility_updates)
            .sum::<usize>()
            / SEEDS,
        total_eligibility_evictions: results
            .iter()
            .map(|result| result.metrics.eligibility_evictions)
            .sum(),
    }
}

#[derive(Clone, Debug)]
pub struct DormantScalingPoint {
    pub total_cells: usize,
    pub active_touches: usize,
    pub local_encounters: usize,
    pub dormant_touches: usize,
    pub global_possible_arrows: u64,
    pub held_out_correct: usize,
    pub held_out_total: usize,
}

#[derive(Clone, Debug)]
pub struct ActiveScalingPoint {
    pub active_irrelevant: usize,
    pub active_touches: usize,
    pub local_encounters: usize,
    pub created: usize,
    pub released: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
}

fn evaluate_frozen_network(
    roles: &SensoryRoleLearner,
    network: &LocalPlasticity,
    seed: u64,
) -> (usize, usize) {
    let Some(choices) = network.evaluated_choices() else {
        return (0, 16);
    };
    let mut ids = IdentitySource::new(seed);
    let mut rng = DeterministicRng::new(seed ^ 0x55aa);
    let mut correct = 0;
    for episode_index in 0..16 {
        let episode = chain_episode(&mut ids, &mut rng, 8, 16);
        let raw = encode_episode(
            &episode,
            EncodingFamily::Transferred,
            seed + episode_index as u64,
        );
        let translated = roles.translate(&raw).unwrap();
        correct += usize::from(execute_program(&translated, choices).outcome == episode.correct);
    }
    (correct, 16)
}

fn trained_p2_for_scaling() -> (SensoryRoleLearner, LocalPlasticity) {
    let seed = run_p2_integrated_seed(41, FeedbackMode::Real, 8, P2_PRIMARY_SLOTS);
    let roles = trained_role_learner(41);
    (roles, seed.network)
}

fn run_dormant_scaling() -> Vec<DormantScalingPoint> {
    let (roles, network) = trained_p2_for_scaling();
    [10usize, 100, 1_000, 10_000]
        .into_iter()
        .map(|total_cells| {
            let mut trial = network.clone();
            trial.reset_metrics();
            let cells = roles.consolidated_cells();
            for _ in 0..64 {
                trial.expose_activity(&cells, 8);
            }
            let (correct, total) =
                evaluate_frozen_network(&roles, &trial, 0xc000 + total_cells as u64);
            DormantScalingPoint {
                total_cells,
                active_touches: trial.metrics.active_relevant_touches
                    + trial.metrics.active_irrelevant_touches,
                local_encounters: trial.metrics.local_encounters,
                dormant_touches: trial.metrics.dormant_touches,
                global_possible_arrows: (total_cells as u64)
                    .saturating_mul(total_cells.saturating_sub(1) as u64),
                held_out_correct: correct,
                held_out_total: total,
            }
        })
        .collect()
}

fn run_active_scaling() -> Vec<ActiveScalingPoint> {
    let (roles, network) = trained_p2_for_scaling();
    [0usize, 10, 100, 1_000]
        .into_iter()
        .map(|active_irrelevant| {
            let mut trial = network.clone();
            trial.reset_metrics();
            let cells = roles.consolidated_cells();
            for _ in 0..64 {
                trial.expose_activity(&cells, active_irrelevant);
            }
            let (correct, total) =
                evaluate_frozen_network(&roles, &trial, 0xd000 + active_irrelevant as u64);
            ActiveScalingPoint {
                active_irrelevant,
                active_touches: trial.metrics.active_relevant_touches
                    + trial.metrics.active_irrelevant_touches,
                local_encounters: trial.metrics.local_encounters,
                created: trial.metrics.directional_couplings_created,
                released: trial.metrics.directional_couplings_released,
                held_out_correct: correct,
                held_out_total: total,
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct SlotDiagnostic {
    pub slots_per_cell: usize,
    pub successful_lookup_seeds: usize,
}

fn run_slot_diagnostic() -> Vec<SlotDiagnostic> {
    [1usize, 2, 4, 8]
        .into_iter()
        .map(|slots| SlotDiagnostic {
            slots_per_cell: slots,
            successful_lookup_seeds: (0..SEEDS)
                .filter(|seed| train_local_lookup(*seed, false, slots).0)
                .count(),
        })
        .collect()
}

const P21_TRAINING_SEEDS: usize = 4;
const P21_EVALUATION_SEEDS: usize = 8;
const P21_EXPLORATION_DIVISOR: u64 = 4_096;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PlasticityValueEvidence {
    useful_runs: usize,
    rejected_runs: usize,
    encounters: usize,
}

#[derive(Clone, Debug, Default)]
struct PlasticityValueModel {
    evidence: HashMap<EncounterSignature, PlasticityValueEvidence>,
}

impl PlasticityValueModel {
    fn observe_completed_run(&mut self, network: &LocalPlasticity) {
        let useful = network.useful_signatures();
        for (signature, encounters) in &network.encountered_signatures {
            let evidence = self.evidence.entry(*signature).or_default();
            evidence.encounters += *encounters;
            if useful.contains(signature) {
                evidence.useful_runs += 1;
            } else {
                evidence.rejected_runs += 1;
            }
        }
    }

    fn predicts_useful(&self, signature: EncounterSignature) -> bool {
        self.evidence
            .get(&signature)
            .is_some_and(|evidence| evidence.useful_runs >= 2 && evidence.rejected_runs == 0)
    }

    fn useful_signatures(&self) -> Vec<EncounterSignature> {
        let mut signatures: Vec<_> = self
            .evidence
            .keys()
            .copied()
            .filter(|signature| self.predicts_useful(*signature))
            .collect();
        signatures.sort_unstable();
        signatures
    }

    fn shuffled(&self) -> Self {
        let valuable = self.useful_signatures().len();
        let useful: HashSet<_> = self.useful_signatures().into_iter().collect();
        let mut replacements: Vec<_> = self
            .evidence
            .keys()
            .copied()
            .filter(|signature| !useful.contains(signature))
            .collect();
        replacements.sort_unstable();
        let replacements: HashSet<_> = replacements.into_iter().take(valuable).collect();
        let mut shuffled = self.clone();
        for (signature, evidence) in &mut shuffled.evidence {
            if replacements.contains(signature) {
                evidence.useful_runs = P21_TRAINING_SEEDS;
                evidence.rejected_runs = 0;
            } else {
                evidence.useful_runs = 0;
                evidence.rejected_runs = P21_TRAINING_SEEDS;
            }
        }
        shuffled
    }

    fn fingerprint(&self) -> u64 {
        let mut entries: Vec<_> = self.evidence.iter().collect();
        entries.sort_by_key(|(signature, _)| **signature);
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for (signature, evidence) in entries {
            fingerprint_mix(&mut hash, signature.first);
            fingerprint_mix(&mut hash, signature.second);
            fingerprint_mix(&mut hash, evidence.useful_runs as u64);
            fingerprint_mix(&mut hash, evidence.rejected_runs as u64);
            fingerprint_mix(&mut hash, evidence.encounters as u64);
        }
        hash
    }
}

fn train_plasticity_value_model() -> (PlasticityValueModel, HashSet<(PlasticUnit, PlasticUnit)>) {
    let mut model = PlasticityValueModel::default();
    let mut oracle_pairs = HashSet::new();
    for seed in 0..P21_TRAINING_SEEDS {
        let result = run_p2_integrated_seed(
            1_000 + seed,
            FeedbackMode::Real,
            P2_PRIMARY_DISTRACTORS,
            P2_PRIMARY_SLOTS,
        );
        assert!(result.competent);
        model.observe_completed_run(&result.network);
        oracle_pairs.extend(result.network.useful_pairs());
    }
    (model, oracle_pairs)
}

#[derive(Clone, Debug)]
struct P21SeedResult {
    competent: bool,
    held_out_correct: usize,
    held_out_total: usize,
    competence_episode: Option<usize>,
    metrics: PlasticityMetrics,
    useful_signatures: HashSet<EncounterSignature>,
    encountered_signatures: HashSet<EncounterSignature>,
}

fn summarize_p21_seed(result: P2IntegratedSeed) -> P21SeedResult {
    P21SeedResult {
        competent: result.competent,
        held_out_correct: result.held_out_correct,
        held_out_total: result.held_out_total,
        competence_episode: result.competence_episode,
        metrics: result.metrics,
        useful_signatures: result.network.useful_signatures(),
        encountered_signatures: result
            .network
            .encountered_signatures
            .keys()
            .copied()
            .collect(),
    }
}

fn run_p21_always_seed(seed: usize) -> P21SeedResult {
    summarize_p21_seed(run_p2_integrated_seed(
        2_000 + seed,
        FeedbackMode::Real,
        P2_PRIMARY_DISTRACTORS,
        P2_PRIMARY_SLOTS,
    ))
}

fn cached_p21_always_results() -> &'static [P21SeedResult] {
    static RESULTS: OnceLock<Vec<P21SeedResult>> = OnceLock::new();
    RESULTS.get_or_init(|| (0..P21_EVALUATION_SEEDS).map(run_p21_always_seed).collect())
}

fn run_p21_model_seed(
    seed: usize,
    model: &PlasticityValueModel,
    exploration_divisor: u64,
) -> P21SeedResult {
    let mut exploration = DeterministicRng::new(0xe100 + seed as u64);
    let mut gate = |context: EncounterContext| {
        model.predicts_useful(context.signature)
            || exploration.next_u64().is_multiple_of(exploration_divisor)
    };
    summarize_p21_seed(run_p2_integrated_seed_with_gate(
        3_000 + seed,
        FeedbackMode::Real,
        P2_PRIMARY_DISTRACTORS,
        P2_PRIMARY_SLOTS,
        &mut gate,
    ))
}

fn run_p21_random_seed(
    seed: usize,
    admission_numerator: u64,
    admission_denominator: u64,
) -> P21SeedResult {
    let mut random = DeterministicRng::new(0xe200 + seed as u64);
    let mut gate = |_| random.next_u64() % admission_denominator < admission_numerator;
    summarize_p21_seed(run_p2_integrated_seed_with_gate(
        4_000 + seed,
        FeedbackMode::Real,
        P2_PRIMARY_DISTRACTORS,
        P2_PRIMARY_SLOTS,
        &mut gate,
    ))
}

fn run_p21_oracle_seed(
    seed: usize,
    useful_pairs: &HashSet<(PlasticUnit, PlasticUnit)>,
) -> P21SeedResult {
    let mut gate = |context: EncounterContext| {
        useful_pairs.contains(&ordered_pair(context.left, context.right))
    };
    summarize_p21_seed(run_p2_integrated_seed_with_gate(
        5_000 + seed,
        FeedbackMode::Real,
        P2_PRIMARY_DISTRACTORS,
        P2_PRIMARY_SLOTS,
        &mut gate,
    ))
}

fn deterministic_discovery_work(metrics: &PlasticityMetrics, gate_cost: bool) -> usize {
    metrics.active_relevant_touches
        + metrics.active_irrelevant_touches
        + metrics.local_encounters
        + metrics.directional_couplings_created
        + metrics.eligibility_updates
        + usize::from(gate_cost) * metrics.gate_evaluations
}

#[derive(Clone, Debug)]
pub struct P21PredictorReport {
    pub entries: usize,
    pub predicted_useful: usize,
    pub useful_recalled: usize,
    pub useful_total: usize,
    pub rejected_correctly: usize,
    pub rejected_total: usize,
    pub shuffled_useful_recalled: usize,
    pub forward_lookup_seeds: usize,
    pub reverse_lookup_seeds: usize,
    pub fingerprint_unchanged: bool,
    pub passed: bool,
}

#[derive(Clone, Debug)]
pub struct P21ConditionReport {
    pub competent_seeds: usize,
    pub total_seeds: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub average_created: usize,
    pub average_released: usize,
    pub average_gate_evaluations: usize,
    pub average_gate_admissions: usize,
    pub average_active_touches: usize,
    pub average_local_encounters: usize,
    pub average_eligibility_updates: usize,
    pub average_competence_episode: Option<f64>,
    pub average_work: usize,
}

fn summarize_p21_condition(results: &[P21SeedResult], gate_cost: bool) -> P21ConditionReport {
    let competence: Vec<_> = results
        .iter()
        .filter_map(|result| result.competence_episode)
        .collect();
    P21ConditionReport {
        competent_seeds: results.iter().filter(|result| result.competent).count(),
        total_seeds: results.len(),
        held_out_correct: results.iter().map(|result| result.held_out_correct).sum(),
        held_out_total: results.iter().map(|result| result.held_out_total).sum(),
        average_created: results
            .iter()
            .map(|result| result.metrics.directional_couplings_created)
            .sum::<usize>()
            / results.len(),
        average_released: results
            .iter()
            .map(|result| result.metrics.directional_couplings_released)
            .sum::<usize>()
            / results.len(),
        average_gate_evaluations: results
            .iter()
            .map(|result| result.metrics.gate_evaluations)
            .sum::<usize>()
            / results.len(),
        average_gate_admissions: results
            .iter()
            .map(|result| result.metrics.gate_admissions)
            .sum::<usize>()
            / results.len(),
        average_active_touches: results
            .iter()
            .map(|result| {
                result.metrics.active_relevant_touches + result.metrics.active_irrelevant_touches
            })
            .sum::<usize>()
            / results.len(),
        average_local_encounters: results
            .iter()
            .map(|result| result.metrics.local_encounters)
            .sum::<usize>()
            / results.len(),
        average_eligibility_updates: results
            .iter()
            .map(|result| result.metrics.eligibility_updates)
            .sum::<usize>()
            / results.len(),
        average_competence_episode: average(&competence),
        average_work: results
            .iter()
            .map(|result| deterministic_discovery_work(&result.metrics, gate_cost))
            .sum::<usize>()
            / results.len(),
    }
}

#[derive(Clone, Debug)]
pub struct P21Report {
    pub predictor: P21PredictorReport,
    pub always: P21ConditionReport,
    pub learned: P21ConditionReport,
    pub random: P21ConditionReport,
    pub shuffled: P21ConditionReport,
    pub oracle: P21ConditionReport,
    pub coupling_reduction: f64,
    pub work_reduction: f64,
    pub passed: bool,
}

pub fn run_p2_1_experiment() -> P21Report {
    let (model, oracle_pairs) = train_plasticity_value_model();
    let before = model.fingerprint();
    let shuffled_model = model.shuffled();
    let mut forward_lookup_seeds = 0;
    let mut reverse_lookup_seeds = 0;
    for seed in 0..SEEDS {
        let mut forward_rng = DeterministicRng::new(0xe300 + seed as u64);
        let mut forward_gate = |context: EncounterContext| {
            model.predicts_useful(context.signature)
                || forward_rng
                    .next_u64()
                    .is_multiple_of(P21_EXPLORATION_DIVISOR)
        };
        forward_lookup_seeds += usize::from(
            train_local_lookup_with_gate(500 + seed, false, P2_PRIMARY_SLOTS, &mut forward_gate).0,
        );
        let mut reverse_rng = DeterministicRng::new(0xe400 + seed as u64);
        let mut reverse_gate = |context: EncounterContext| {
            model.predicts_useful(context.signature)
                || reverse_rng
                    .next_u64()
                    .is_multiple_of(P21_EXPLORATION_DIVISOR)
        };
        reverse_lookup_seeds += usize::from(
            train_local_lookup_with_gate(600 + seed, true, P2_PRIMARY_SLOTS, &mut reverse_gate).0,
        );
    }

    let always_results = cached_p21_always_results();
    let mut useful_recalled = 0;
    let mut useful_total = 0;
    let mut rejected_correctly = 0;
    let mut rejected_total = 0;
    let mut shuffled_useful_recalled = 0;
    for result in always_results {
        for signature in &result.useful_signatures {
            useful_total += 1;
            useful_recalled += usize::from(model.predicts_useful(*signature));
            shuffled_useful_recalled += usize::from(shuffled_model.predicts_useful(*signature));
        }
        for signature in result
            .encountered_signatures
            .difference(&result.useful_signatures)
        {
            rejected_total += 1;
            rejected_correctly += usize::from(!model.predicts_useful(*signature));
        }
    }
    let predictor = P21PredictorReport {
        entries: model.evidence.len(),
        predicted_useful: model.useful_signatures().len(),
        useful_recalled,
        useful_total,
        rejected_correctly,
        rejected_total,
        shuffled_useful_recalled,
        forward_lookup_seeds,
        reverse_lookup_seeds,
        fingerprint_unchanged: before == model.fingerprint(),
        passed: useful_recalled == useful_total
            && rejected_correctly == rejected_total
            && useful_total > 0
            && shuffled_useful_recalled < useful_total
            && forward_lookup_seeds == SEEDS
            && reverse_lookup_seeds == SEEDS
            && before == model.fingerprint(),
    };

    let learned_results: Vec<_> = (0..P21_EVALUATION_SEEDS)
        .map(|seed| run_p21_model_seed(seed, &model, P21_EXPLORATION_DIVISOR))
        .collect();
    let learned_admissions: usize = learned_results
        .iter()
        .map(|result| result.metrics.gate_admissions)
        .sum();
    let learned_evaluations: usize = learned_results
        .iter()
        .map(|result| result.metrics.gate_evaluations)
        .sum();
    let denominator = learned_evaluations.max(1) as u64;
    let numerator = learned_admissions.max(1) as u64;
    let random_results: Vec<_> = (0..P21_EVALUATION_SEEDS)
        .map(|seed| run_p21_random_seed(seed, numerator, denominator))
        .collect();
    let shuffled_results: Vec<_> = (0..P21_EVALUATION_SEEDS)
        .map(|seed| run_p21_model_seed(100 + seed, &shuffled_model, P21_EXPLORATION_DIVISOR))
        .collect();
    let oracle_results: Vec<_> = (0..P21_EVALUATION_SEEDS)
        .map(|seed| run_p21_oracle_seed(seed, &oracle_pairs))
        .collect();

    let always = summarize_p21_condition(always_results, false);
    let learned = summarize_p21_condition(&learned_results, true);
    let random = summarize_p21_condition(&random_results, true);
    let shuffled = summarize_p21_condition(&shuffled_results, true);
    let oracle = summarize_p21_condition(&oracle_results, true);
    let coupling_reduction = 1.0 - learned.average_created as f64 / always.average_created as f64;
    let work_reduction = 1.0 - learned.average_work as f64 / always.average_work as f64;
    let passed = predictor.passed
        && learned.competent_seeds == P21_EVALUATION_SEEDS
        && learned.held_out_correct == learned.held_out_total
        && learned.average_created < always.average_created
        && learned.average_created < random.average_created
        && learned.average_created < shuffled.average_created
        && learned.average_work < always.average_work
        && oracle.competent_seeds == P21_EVALUATION_SEEDS;

    P21Report {
        predictor,
        always,
        learned,
        random,
        shuffled,
        oracle,
        coupling_reduction,
        work_reduction,
        passed,
    }
}

pub fn print_p2_1_report(report: &P21Report) {
    println!("P2.1 selective structural plasticity:");
    println!(
        "  predictor: entries={}, predicted-useful={}, useful={}/{}, rejected={}/{}, shuffled-useful={}/{}, lookup forward/reverse={}/{}, frozen={}",
        report.predictor.entries,
        report.predictor.predicted_useful,
        report.predictor.useful_recalled,
        report.predictor.useful_total,
        report.predictor.rejected_correctly,
        report.predictor.rejected_total,
        report.predictor.shuffled_useful_recalled,
        report.predictor.useful_total,
        report.predictor.forward_lookup_seeds,
        report.predictor.reverse_lookup_seeds,
        report.predictor.fingerprint_unchanged
    );
    println!(
        "  always: competent={}/{}, created={}, work={}",
        report.always.competent_seeds,
        report.always.total_seeds,
        report.always.average_created,
        report.always.average_work
    );
    println!(
        "  learned gate: competent={}/{}, created={}, gate={}/{}, work={}, coupling/work reduction={:.1}%/{:.1}%",
        report.learned.competent_seeds,
        report.learned.total_seeds,
        report.learned.average_created,
        report.learned.average_gate_admissions,
        report.learned.average_gate_evaluations,
        report.learned.average_work,
        report.coupling_reduction * 100.0,
        report.work_reduction * 100.0
    );
    println!(
        "  controls: random competent={}/{}, created={}; shuffled competent={}/{}, created={}; oracle competent={}/{}, created={}; passed={}",
        report.random.competent_seeds,
        report.random.total_seeds,
        report.random.average_created,
        report.shuffled.competent_seeds,
        report.shuffled.total_seeds,
        report.shuffled.average_created,
        report.oracle.competent_seeds,
        report.oracle.total_seeds,
        report.oracle.average_created,
        report.passed
    );
}

const P22_REPRESENTATION_THRESHOLD: usize = 4;
const P22_EXPLORATION_DIVISOR: u64 = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EncounterPrototype {
    id: usize,
    snapshot: PreCouplingSnapshot,
    observations: usize,
}

#[derive(Clone, Debug, Default)]
struct EncounterEncoder {
    prototypes: Vec<EncounterPrototype>,
}

impl EncounterEncoder {
    fn observe(&mut self, snapshot: PreCouplingSnapshot) -> usize {
        self.observe_with_work(snapshot).0
    }

    fn observe_with_work(&mut self, snapshot: PreCouplingSnapshot) -> (usize, usize) {
        let mut comparisons = 0;
        for prototype in &mut self.prototypes {
            comparisons += 1;
            if prototype.snapshot == snapshot {
                prototype.observations += 1;
                return (prototype.id, comparisons);
            }
        }
        let id = self.prototypes.len();
        self.prototypes.push(EncounterPrototype {
            id,
            snapshot,
            observations: 1,
        });
        (id, comparisons + 1)
    }

    fn encode(&self, snapshot: PreCouplingSnapshot) -> Option<usize> {
        self.encode_with_work(snapshot).0
    }

    fn encode_with_work(&self, snapshot: PreCouplingSnapshot) -> (Option<usize>, usize) {
        let mut comparisons = 0;
        for prototype in &self.prototypes {
            comparisons += 1;
            if prototype.snapshot == snapshot
                && prototype.observations >= P22_REPRESENTATION_THRESHOLD
            {
                return (Some(prototype.id), comparisons);
            }
        }
        (None, comparisons)
    }

    fn fingerprint(&self) -> u64 {
        let mut prototypes = self.prototypes.clone();
        prototypes.sort_by_key(|prototype| prototype.snapshot);
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for prototype in prototypes {
            fingerprint_mix(&mut hash, prototype.snapshot.code() as u64);
            fingerprint_mix(&mut hash, prototype.observations as u64);
        }
        hash
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct EncounterValueEvidence {
    useful: usize,
    rejected: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct EncounterValueKey {
    representation: usize,
    context: EncounterValueContext,
}

#[derive(Clone, Debug, Default)]
struct EncounterValueModel {
    aggregate: HashMap<usize, EncounterValueEvidence>,
    contextual: HashMap<EncounterValueKey, EncounterValueEvidence>,
}

impl EncounterValueModel {
    fn record(evidence: &mut EncounterValueEvidence, useful: bool) {
        if useful {
            evidence.useful += 1;
        } else {
            evidence.rejected += 1;
        }
    }

    fn observe(&mut self, representation: usize, context: EncounterValueContext, useful: bool) {
        Self::record(self.aggregate.entry(representation).or_default(), useful);
        Self::record(
            self.contextual
                .entry(EncounterValueKey {
                    representation,
                    context,
                })
                .or_default(),
            useful,
        );
    }

    fn predicts_representation_useful(&self, representation: usize) -> bool {
        self.aggregate
            .get(&representation)
            .is_some_and(|evidence| evidence.useful >= 2)
    }

    fn predicts_useful(&self, representation: usize, context: EncounterValueContext) -> bool {
        if let Some(evidence) = self.contextual.get(&EncounterValueKey {
            representation,
            context,
        }) {
            if evidence.useful >= 2 {
                return true;
            }
            if evidence.rejected >= 2 && evidence.useful == 0 {
                return false;
            }
        }
        self.predicts_representation_useful(representation)
    }

    fn admits_plasticity(&self, representation: usize, context: EncounterValueContext) -> bool {
        self.predicts_representation_useful(representation)
            || self.predicts_useful(representation, context)
    }

    fn has_useful_value(&self) -> bool {
        self.aggregate
            .keys()
            .copied()
            .any(|representation| self.predicts_representation_useful(representation))
    }

    fn useful_representations(&self) -> Vec<usize> {
        let mut representations: Vec<_> = self
            .aggregate
            .keys()
            .copied()
            .filter(|representation| self.predicts_representation_useful(*representation))
            .collect();
        representations.sort_unstable();
        representations
    }

    fn shuffled(&self, representation_count: usize) -> Self {
        let useful = self.useful_representations();
        let useful_set: HashSet<_> = useful.iter().copied().collect();
        let replacements: HashSet<_> = (0..representation_count)
            .filter(|representation| !useful_set.contains(representation))
            .take(useful.len())
            .collect();
        let mut shuffled = Self::default();
        for representation in 0..representation_count {
            if replacements.contains(&representation) {
                shuffled.aggregate.insert(
                    representation,
                    EncounterValueEvidence {
                        useful: P21_TRAINING_SEEDS,
                        rejected: 0,
                    },
                );
            } else {
                shuffled.aggregate.insert(
                    representation,
                    EncounterValueEvidence {
                        useful: 0,
                        rejected: P21_TRAINING_SEEDS,
                    },
                );
            }
        }
        for (key, evidence) in &self.contextual {
            let replacement = if useful_set.contains(&key.representation) {
                (0..representation_count)
                    .find(|representation| replacements.contains(representation))
                    .unwrap_or(key.representation)
            } else if replacements.contains(&key.representation) {
                useful.first().copied().unwrap_or(key.representation)
            } else {
                key.representation
            };
            shuffled.contextual.insert(
                EncounterValueKey {
                    representation: replacement,
                    context: key.context,
                },
                *evidence,
            );
        }
        shuffled
    }

    fn fingerprint(&self) -> u64 {
        let mut evidence: Vec<_> = self.aggregate.iter().collect();
        evidence.sort_by_key(|(representation, _)| **representation);
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for (representation, evidence) in evidence {
            fingerprint_mix(&mut hash, *representation as u64);
            fingerprint_mix(&mut hash, evidence.useful as u64);
            fingerprint_mix(&mut hash, evidence.rejected as u64);
        }
        let mut contextual: Vec<_> = self.contextual.iter().collect();
        contextual.sort_by_key(|(key, _)| **key);
        for (key, evidence) in contextual {
            fingerprint_mix(&mut hash, key.representation as u64);
            fingerprint_mix(&mut hash, key.context.first.occupied_slots as u64);
            fingerprint_mix(
                &mut hash,
                key.context.first.has_consolidated_outgoing as u64,
            );
            fingerprint_mix(
                &mut hash,
                key.context.first.has_consolidated_incoming as u64,
            );
            fingerprint_mix(&mut hash, key.context.second.occupied_slots as u64);
            fingerprint_mix(
                &mut hash,
                key.context.second.has_consolidated_outgoing as u64,
            );
            fingerprint_mix(
                &mut hash,
                key.context.second.has_consolidated_incoming as u64,
            );
            fingerprint_mix(&mut hash, evidence.useful as u64);
            fingerprint_mix(&mut hash, evidence.rejected as u64);
        }
        hash
    }
}

#[derive(Clone, Debug)]
struct EncounterObservation {
    pair: (PlasticUnit, PlasticUnit),
    snapshot: PreCouplingSnapshot,
    context: EncounterValueContext,
}

#[derive(Clone, Debug)]
struct EncounterTrainingRun {
    observations: Vec<EncounterObservation>,
    useful_pairs: HashSet<(PlasticUnit, PlasticUnit)>,
}

fn collect_encounter_training_run(seed: usize) -> EncounterTrainingRun {
    let mut observed = HashSet::new();
    let mut observations = Vec::new();
    let mut gate = |context: EncounterContext| {
        let key = (
            ordered_pair(context.left, context.right),
            context.snapshot,
            context.value_context,
        );
        if observed.insert(key) {
            observations.push(EncounterObservation {
                pair: key.0,
                snapshot: key.1,
                context: key.2,
            });
        }
        true
    };
    let result = run_p2_integrated_seed_with_gate(
        6_000 + seed,
        FeedbackMode::Real,
        P2_PRIMARY_DISTRACTORS,
        P2_PRIMARY_SLOTS,
        &mut gate,
    );
    assert!(result.competent);
    EncounterTrainingRun {
        observations,
        useful_pairs: result.network.useful_pairs(),
    }
}

fn train_encounter_model() -> (
    EncounterEncoder,
    EncounterValueModel,
    usize,
    Vec<EncounterTrainingRun>,
) {
    let runs: Vec<_> = (0..P21_TRAINING_SEEDS)
        .map(collect_encounter_training_run)
        .collect();
    let mut encoder = EncounterEncoder::default();
    let mut snapshots: Vec<_> = runs
        .iter()
        .flat_map(|run| {
            run.observations
                .iter()
                .map(|observation| observation.snapshot)
        })
        .collect();
    snapshots.sort_unstable();
    for snapshot in snapshots {
        encoder.observe(snapshot);
    }
    let mut value = EncounterValueModel::default();
    let mut mixed_representations: HashSet<usize> = HashSet::new();
    for run in &runs {
        let mut encountered = HashSet::new();
        let mut useful = HashSet::new();
        let mut rejected = HashSet::new();
        for observation in &run.observations {
            let representation = encoder.encode(observation.snapshot).unwrap();
            encountered.insert(representation);
            if run.useful_pairs.contains(&observation.pair) {
                useful.insert(representation);
            } else {
                rejected.insert(representation);
            }
        }
        mixed_representations.extend(useful.intersection(&rejected).copied());
        for representation in encountered {
            let contexts: HashSet<_> = run
                .observations
                .iter()
                .filter(|observation| encoder.encode(observation.snapshot) == Some(representation))
                .map(|observation| observation.context)
                .collect();
            for context in contexts {
                let context_useful = run.observations.iter().any(|observation| {
                    encoder.encode(observation.snapshot) == Some(representation)
                        && observation.context == context
                        && run.useful_pairs.contains(&observation.pair)
                });
                value.observe(representation, context, context_useful);
            }
        }
    }
    (encoder, value, mixed_representations.len(), runs)
}

fn run_p22_frozen_seed(
    seed: usize,
    encoder: &EncounterEncoder,
    value: &EncounterValueModel,
    exploration_divisor: u64,
) -> (P21SeedResult, usize) {
    let mut exploration = DeterministicRng::new(0xf100 + seed as u64);
    let mut recognition_work = 0;
    let mut gate = |context: EncounterContext| {
        let (representation, comparisons) = encoder.encode_with_work(context.snapshot);
        recognition_work += comparisons;
        representation.is_some_and(|representation| {
            value.admits_plasticity(representation, context.value_context)
        }) || exploration.next_u64().is_multiple_of(exploration_divisor)
    };
    let result = summarize_p21_seed(run_p2_integrated_seed_with_gate(
        7_000 + seed,
        FeedbackMode::Real,
        P2_PRIMARY_DISTRACTORS,
        P2_PRIMARY_SLOTS,
        &mut gate,
    ));
    (result, recognition_work)
}

#[derive(Clone, Debug)]
struct AdaptiveEncounterGate {
    encoder: EncounterEncoder,
    value: EncounterValueModel,
    exploration: DeterministicRng,
    gate_rejections: usize,
    exploration_admissions: usize,
    useful_from_exploration: usize,
    first_useful_episode: Option<usize>,
    representation_work: usize,
    value_updates: usize,
}

impl AdaptiveEncounterGate {
    fn new(seed: u64) -> Self {
        Self {
            encoder: EncounterEncoder::default(),
            value: EncounterValueModel::default(),
            exploration: DeterministicRng::new(seed),
            gate_rejections: 0,
            exploration_admissions: 0,
            useful_from_exploration: 0,
            first_useful_episode: None,
            representation_work: 0,
            value_updates: 0,
        }
    }

    fn decide(
        &mut self,
        snapshot: PreCouplingSnapshot,
        context: EncounterValueContext,
    ) -> EncounterAdmission {
        let (representation, work) = self.encoder.observe_with_work(snapshot);
        self.representation_work += work;
        if !self.value.has_useful_value() {
            return EncounterAdmission {
                admit: true,
                exploratory: false,
            };
        }
        if self.value.admits_plasticity(representation, context) {
            return EncounterAdmission {
                admit: true,
                exploratory: false,
            };
        }
        if self
            .exploration
            .next_u64()
            .is_multiple_of(P22_EXPLORATION_DIVISOR)
        {
            self.exploration_admissions += 1;
            return EncounterAdmission {
                admit: true,
                exploratory: true,
            };
        }
        self.gate_rejections += 1;
        EncounterAdmission {
            admit: false,
            exploratory: false,
        }
    }

    fn observe_outcomes(&mut self, episode: usize, outcomes: Vec<PlasticityOutcome>) {
        for outcome in outcomes {
            let (representation, work) = self.encoder.observe_with_work(outcome.snapshot);
            self.representation_work += work;
            if outcome.useful && outcome.exploratory {
                self.useful_from_exploration += 1;
            }
            self.value
                .observe(representation, outcome.context, outcome.useful);
            self.value_updates += 1;
        }
        if self.value.has_useful_value() && self.first_useful_episode.is_none() {
            self.first_useful_episode = Some(episode);
        }
    }

    fn fingerprint(&self) -> u64 {
        hash_words(&[self.encoder.fingerprint(), self.value.fingerprint()])
    }
}

#[derive(Clone, Debug)]
struct P22AdaptiveSeed {
    seed: P21SeedResult,
    representations: usize,
    valued_representations: usize,
    first_useful_episode: Option<usize>,
    created_at_first_useful: usize,
    gate_rejections: usize,
    exploration_admissions: usize,
    useful_from_exploration: usize,
    encoder_value_fingerprint_unchanged: bool,
    representation_work: usize,
    value_updates: usize,
}

fn run_p22_adaptive_seed(seed: usize) -> P22AdaptiveSeed {
    let mut roles = SensoryRoleLearner::default();
    let mut network =
        LocalPlasticity::with_outcome_tracking(0xf200 + seed as u64, P2_PRIMARY_SLOTS, true);
    let mut gate = AdaptiveEncounterGate::new(0xf300 + seed as u64);
    let mut ids = IdentitySource::new(0xf400 + seed as u64);
    let mut rng = DeterministicRng::new(0xf500 + seed as u64);
    let mut first_success = None;
    let mut competence = None;
    let mut created_at_first_useful = 0;

    for episode_index in 1..=P2_INTEGRATED_BUDGET {
        let depth = TRAIN_DEPTHS[(episode_index - 1) % TRAIN_DEPTHS.len()];
        let episode = chain_episode(&mut ids, &mut rng, depth, 12);
        let raw = encode_episode(
            &episode,
            EncodingFamily::Training,
            0xf600 + seed as u64 * 100_000 + episode_index as u64,
        );
        roles.observe(&raw);
        let Some(translated) = roles.translate(&raw) else {
            continue;
        };
        let cells: Vec<_> = roles.patterns.iter().map(|pattern| pattern.cell).collect();
        network.expose_activity_with_admission_gate(&cells, P2_PRIMARY_DISTRACTORS, |context| {
            gate.decide(context.snapshot, context.value_context)
        });
        if let Some(choices) = network.choices() {
            let run = execute_program(&translated, choices);
            let success =
                run.outcome == episode.correct && !run.activity_limit_hit && run.explicit_answer;
            if success && first_success.is_none() {
                first_success = Some(episode_index);
            }
            network.register_used(&run.used_arrows);
            network.terminal_feedback(success);
        }
        let before_useful = gate.first_useful_episode.is_none();
        gate.observe_outcomes(episode_index, network.drain_plasticity_outcomes());
        if before_useful && gate.first_useful_episode.is_some() {
            created_at_first_useful = network.metrics.directional_couplings_created;
        }
        if competence.is_none() && network.complete() && roles.consolidated_cells().len() == 3 {
            competence = Some(episode_index);
            break;
        }
    }

    let learning_fingerprint = gate.fingerprint();
    let fingerprint_before = hash_words(&[
        roles.fingerprint(),
        network.fingerprint(),
        learning_fingerprint,
    ]);
    let mut held_out_correct = 0;
    let mut held_out_total = 0;
    let mut explicit_answers = true;
    let mut queues_empty = true;
    if let Some(choices) = network.evaluated_choices() {
        let mut heldout_ids = IdentitySource::new(0xf700 + seed as u64);
        let mut heldout_rng = DeterministicRng::new(0xf800 + seed as u64);
        for depth in HELD_OUT_DEPTHS {
            for episode_index in 0..16 {
                let episode = chain_episode(&mut heldout_ids, &mut heldout_rng, depth, depth + 8);
                let raw = encode_episode(
                    &episode,
                    EncodingFamily::Transferred,
                    0xf900 + seed as u64 * 10_000 + depth as u64 * 100 + episode_index,
                );
                let translated = roles.translate(&raw).unwrap();
                let run = execute_program(&translated, choices);
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
    let fingerprint_after = hash_words(&[
        roles.fingerprint(),
        network.fingerprint(),
        gate.fingerprint(),
    ]);
    network.update_peak();
    let result = P2IntegratedSeed {
        competent: held_out_correct == held_out_total,
        held_out_correct,
        held_out_total,
        roles: roles.consolidated_cells().len(),
        surviving_program: network.consolidated_count(),
        fingerprint_unchanged: fingerprint_before == fingerprint_after,
        explicit_answers,
        queues_empty,
        first_success_episode: first_success,
        competence_episode: competence,
        metrics: network.metrics.clone(),
        network,
    };
    P22AdaptiveSeed {
        seed: summarize_p21_seed(result),
        representations: gate.encoder.prototypes.len(),
        valued_representations: gate.value.useful_representations().len(),
        first_useful_episode: gate.first_useful_episode,
        created_at_first_useful,
        gate_rejections: gate.gate_rejections,
        exploration_admissions: gate.exploration_admissions,
        useful_from_exploration: gate.useful_from_exploration,
        encoder_value_fingerprint_unchanged: fingerprint_before == fingerprint_after,
        representation_work: gate.representation_work,
        value_updates: gate.value_updates,
    }
}

#[derive(Clone, Debug)]
pub struct P22RepresentationReport {
    pub representations: usize,
    pub valued_representations: usize,
    pub mixed_outcome_representations: usize,
    pub context_sensitive_representations: usize,
    pub useful_recalled: usize,
    pub useful_total: usize,
    pub rejected_correctly: usize,
    pub rejected_total: usize,
    pub shuffled_useful_recalled: usize,
    pub frozen: bool,
    pub passed: bool,
}

#[derive(Clone, Debug)]
pub struct P22AdaptiveReport {
    pub condition: P21ConditionReport,
    pub average_representations: f64,
    pub average_valued_representations: f64,
    pub average_first_useful_episode: Option<f64>,
    pub average_created_at_first_useful: usize,
    pub average_gate_rejections: usize,
    pub average_exploration_admissions: usize,
    pub useful_from_exploration: usize,
    pub average_representation_work: usize,
    pub average_value_updates: usize,
    pub early_created_per_episode: f64,
    pub late_created_per_episode: f64,
    pub fingerprints_unchanged: bool,
}

#[derive(Clone, Debug)]
pub struct P22Report {
    pub representation: P22RepresentationReport,
    pub always: P21ConditionReport,
    pub frozen_gate: P21ConditionReport,
    pub random: P21ConditionReport,
    pub shuffled: P21ConditionReport,
    pub oracle: P21ConditionReport,
    pub adaptive: P22AdaptiveReport,
    pub frozen_coupling_reduction: f64,
    pub frozen_work_reduction: f64,
    pub adaptive_coupling_reduction: f64,
    pub adaptive_work_reduction: f64,
    pub passed: bool,
}

pub fn run_p2_2_experiment() -> P22Report {
    let (encoder, value, mixed_outcomes, training_runs) = train_encounter_model();
    let oracle_pairs: HashSet<_> = training_runs
        .iter()
        .flat_map(|run| run.useful_pairs.iter().copied())
        .collect();
    let encoder_before = encoder.fingerprint();
    let value_before = value.fingerprint();
    let shuffled_value = value.shuffled(encoder.prototypes.len());

    let always_results = cached_p21_always_results();
    let mut useful_recalled = 0;
    let mut useful_total = 0;
    let mut rejected_correctly = 0;
    let mut rejected_total = 0;
    let mut shuffled_useful_recalled = 0;
    let mut predictions_by_representation: HashMap<usize, HashSet<bool>> = HashMap::new();
    for run in &training_runs {
        let mut encountered_keys = HashSet::new();
        let mut useful_keys = HashSet::new();
        for observation in &run.observations {
            let representation = encoder.encode(observation.snapshot).unwrap();
            let key = EncounterValueKey {
                representation,
                context: observation.context,
            };
            encountered_keys.insert(key);
            if run.useful_pairs.contains(&observation.pair) {
                useful_keys.insert(key);
            }
        }
        for key in &useful_keys {
            useful_total += 1;
            let prediction = value.predicts_useful(key.representation, key.context);
            predictions_by_representation
                .entry(key.representation)
                .or_default()
                .insert(prediction);
            useful_recalled += usize::from(prediction);
            shuffled_useful_recalled +=
                usize::from(shuffled_value.predicts_useful(key.representation, key.context));
        }
        for key in encountered_keys.difference(&useful_keys) {
            rejected_total += 1;
            let prediction = value.predicts_useful(key.representation, key.context);
            predictions_by_representation
                .entry(key.representation)
                .or_default()
                .insert(prediction);
            rejected_correctly += usize::from(!prediction);
        }
    }
    let context_sensitive_representations = predictions_by_representation
        .values()
        .filter(|predictions| predictions.len() > 1)
        .count();
    let representation = P22RepresentationReport {
        representations: encoder.prototypes.len(),
        valued_representations: value.useful_representations().len(),
        mixed_outcome_representations: mixed_outcomes,
        context_sensitive_representations,
        useful_recalled,
        useful_total,
        rejected_correctly,
        rejected_total,
        shuffled_useful_recalled,
        frozen: encoder_before == encoder.fingerprint() && value_before == value.fingerprint(),
        passed: encoder.prototypes.len() > 1
            && mixed_outcomes > 0
            && context_sensitive_representations > 0
            && useful_recalled == useful_total
            && rejected_correctly * 4 >= rejected_total * 3
            && shuffled_useful_recalled < useful_total
            && encoder_before == encoder.fingerprint()
            && value_before == value.fingerprint(),
    };

    let frozen_runs: Vec<_> = (0..P21_EVALUATION_SEEDS)
        .map(|seed| run_p22_frozen_seed(seed, &encoder, &value, P22_EXPLORATION_DIVISOR))
        .collect();
    let frozen_results: Vec<_> = frozen_runs
        .iter()
        .map(|(result, _)| result.clone())
        .collect();
    let frozen_recognition_work: usize = frozen_runs.iter().map(|(_, work)| work).sum();
    let frozen_admissions: usize = frozen_results
        .iter()
        .map(|result| result.metrics.gate_admissions)
        .sum();
    let frozen_evaluations: usize = frozen_results
        .iter()
        .map(|result| result.metrics.gate_evaluations)
        .sum();
    let random_results: Vec<_> = (0..P21_EVALUATION_SEEDS)
        .map(|seed| {
            run_p21_random_seed(
                200 + seed,
                frozen_admissions.max(1) as u64,
                frozen_evaluations.max(1) as u64,
            )
        })
        .collect();
    let shuffled_runs: Vec<_> = (0..P21_EVALUATION_SEEDS)
        .map(|seed| {
            run_p22_frozen_seed(
                100 + seed,
                &encoder,
                &shuffled_value,
                P22_EXPLORATION_DIVISOR,
            )
        })
        .collect();
    let shuffled_results: Vec<_> = shuffled_runs
        .iter()
        .map(|(result, _)| result.clone())
        .collect();
    let shuffled_recognition_work: usize = shuffled_runs.iter().map(|(_, work)| work).sum();
    let oracle_results: Vec<_> = (0..P21_EVALUATION_SEEDS)
        .map(|seed| run_p21_oracle_seed(200 + seed, &oracle_pairs))
        .collect();
    let adaptive_results: Vec<_> = (0..P21_EVALUATION_SEEDS)
        .map(run_p22_adaptive_seed)
        .collect();
    let adaptive_seed_results: Vec<_> = adaptive_results
        .iter()
        .map(|result| result.seed.clone())
        .collect();

    let always = summarize_p21_condition(always_results, false);
    let mut frozen_gate = summarize_p21_condition(&frozen_results, true);
    frozen_gate.average_work += frozen_recognition_work / frozen_runs.len();
    let random = summarize_p21_condition(&random_results, true);
    let mut shuffled = summarize_p21_condition(&shuffled_results, true);
    shuffled.average_work += shuffled_recognition_work / shuffled_runs.len();
    let oracle = summarize_p21_condition(&oracle_results, true);
    let mut adaptive_condition = summarize_p21_condition(&adaptive_seed_results, true);
    let adaptive_representation_work: usize = adaptive_results
        .iter()
        .map(|result| result.representation_work)
        .sum();
    let adaptive_value_updates: usize = adaptive_results
        .iter()
        .map(|result| result.value_updates)
        .sum();
    adaptive_condition.average_work +=
        (adaptive_representation_work + adaptive_value_updates) / adaptive_results.len();
    let first_useful: Vec<_> = adaptive_results
        .iter()
        .filter_map(|result| result.first_useful_episode)
        .collect();
    let early_created: usize = adaptive_results
        .iter()
        .map(|result| result.created_at_first_useful)
        .sum();
    let early_episodes: usize = adaptive_results
        .iter()
        .filter_map(|result| result.first_useful_episode)
        .sum();
    let late_created: usize = adaptive_results
        .iter()
        .map(|result| {
            result
                .seed
                .metrics
                .directional_couplings_created
                .saturating_sub(result.created_at_first_useful)
        })
        .sum();
    let late_episodes: usize = adaptive_results
        .iter()
        .map(|result| {
            result
                .seed
                .competence_episode
                .unwrap_or(P2_INTEGRATED_BUDGET)
                .saturating_sub(result.first_useful_episode.unwrap_or(0))
        })
        .sum();
    let adaptive = P22AdaptiveReport {
        condition: adaptive_condition,
        average_representations: adaptive_results
            .iter()
            .map(|result| result.representations)
            .sum::<usize>() as f64
            / adaptive_results.len() as f64,
        average_valued_representations: adaptive_results
            .iter()
            .map(|result| result.valued_representations)
            .sum::<usize>() as f64
            / adaptive_results.len() as f64,
        average_first_useful_episode: average(&first_useful),
        average_created_at_first_useful: early_created / adaptive_results.len(),
        average_gate_rejections: adaptive_results
            .iter()
            .map(|result| result.gate_rejections)
            .sum::<usize>()
            / adaptive_results.len(),
        average_exploration_admissions: adaptive_results
            .iter()
            .map(|result| result.exploration_admissions)
            .sum::<usize>()
            / adaptive_results.len(),
        useful_from_exploration: adaptive_results
            .iter()
            .map(|result| result.useful_from_exploration)
            .sum(),
        average_representation_work: adaptive_representation_work / adaptive_results.len(),
        average_value_updates: adaptive_value_updates / adaptive_results.len(),
        early_created_per_episode: early_created as f64 / early_episodes.max(1) as f64,
        late_created_per_episode: late_created as f64 / late_episodes.max(1) as f64,
        fingerprints_unchanged: adaptive_results
            .iter()
            .all(|result| result.encoder_value_fingerprint_unchanged),
    };
    let frozen_coupling_reduction =
        1.0 - frozen_gate.average_created as f64 / always.average_created as f64;
    let frozen_work_reduction = 1.0 - frozen_gate.average_work as f64 / always.average_work as f64;
    let adaptive_coupling_reduction =
        1.0 - adaptive.condition.average_created as f64 / always.average_created as f64;
    let adaptive_work_reduction =
        1.0 - adaptive.condition.average_work as f64 / always.average_work as f64;
    let passed = representation.passed
        && frozen_gate.competent_seeds == P21_EVALUATION_SEEDS
        && frozen_gate.held_out_correct == frozen_gate.held_out_total
        && frozen_gate.average_created < always.average_created
        && frozen_gate.average_created < random.average_created
        && shuffled.competent_seeds < P21_EVALUATION_SEEDS
        && oracle.competent_seeds == P21_EVALUATION_SEEDS
        && adaptive.condition.competent_seeds == P21_EVALUATION_SEEDS
        && adaptive.condition.held_out_correct == adaptive.condition.held_out_total
        && adaptive.condition.average_created < always.average_created
        && adaptive.condition.average_work < always.average_work
        && adaptive.late_created_per_episode < adaptive.early_created_per_episode
        && adaptive.fingerprints_unchanged;

    P22Report {
        representation,
        always,
        frozen_gate,
        random,
        shuffled,
        oracle,
        adaptive,
        frozen_coupling_reduction,
        frozen_work_reduction,
        adaptive_coupling_reduction,
        adaptive_work_reduction,
        passed,
    }
}

pub fn print_p2_2_report(report: &P22Report) {
    println!("P2.2 learned encounter representations:");
    println!(
        "  representation: learned={}, valued={}, mixed={}, context-sensitive={}, useful={}/{}, rejected={}/{}, shuffled-useful={}/{}, frozen={}",
        report.representation.representations,
        report.representation.valued_representations,
        report.representation.mixed_outcome_representations,
        report.representation.context_sensitive_representations,
        report.representation.useful_recalled,
        report.representation.useful_total,
        report.representation.rejected_correctly,
        report.representation.rejected_total,
        report.representation.shuffled_useful_recalled,
        report.representation.useful_total,
        report.representation.frozen
    );
    println!(
        "  frozen gate: competent={}/{}, created={}, work={}, reduction={:.1}%/{:.1}%",
        report.frozen_gate.competent_seeds,
        report.frozen_gate.total_seeds,
        report.frozen_gate.average_created,
        report.frozen_gate.average_work,
        report.frozen_coupling_reduction * 100.0,
        report.frozen_work_reduction * 100.0
    );
    println!(
        "  fresh adaptive: competent={}/{}, created={}, work={}, recognition/value={}/{}, first useful={:?}, competence={:?}, early/late creation={:.2}/{:.2}, exploration={}/{}, reduction={:.1}%/{:.1}%",
        report.adaptive.condition.competent_seeds,
        report.adaptive.condition.total_seeds,
        report.adaptive.condition.average_created,
        report.adaptive.condition.average_work,
        report.adaptive.average_representation_work,
        report.adaptive.average_value_updates,
        report.adaptive.average_first_useful_episode,
        report.adaptive.condition.average_competence_episode,
        report.adaptive.early_created_per_episode,
        report.adaptive.late_created_per_episode,
        report.adaptive.useful_from_exploration,
        report.adaptive.average_exploration_admissions,
        report.adaptive_coupling_reduction * 100.0,
        report.adaptive_work_reduction * 100.0
    );
    println!(
        "  controls: random competent={}/{}, created={}; shuffled competent={}/{}, created={}; oracle competent={}/{}, created={}; passed={}",
        report.random.competent_seeds,
        report.random.total_seeds,
        report.random.average_created,
        report.shuffled.competent_seeds,
        report.shuffled.total_seeds,
        report.shuffled.average_created,
        report.oracle.competent_seeds,
        report.oracle.total_seeds,
        report.oracle.average_created,
        report.passed
    );
}

#[derive(Clone, Debug)]
pub struct P2Report {
    pub lookup: P2LookupReport,
    pub roles: RoleDiscoveryReport,
    pub integrated: P2IntegratedReport,
    pub shuffled: P2IntegratedReport,
    pub random: P2IntegratedReport,
    pub dormant_scaling: Vec<DormantScalingPoint>,
    pub active_scaling: Vec<ActiveScalingPoint>,
    pub slot_diagnostic: Vec<SlotDiagnostic>,
    pub passed: bool,
}

pub fn run_p2_experiment() -> P2Report {
    let lookup = run_p2_lookup();
    let roles = run_role_discovery();
    let integrated = run_p2_integrated_condition(FeedbackMode::Real);
    let shuffled = run_p2_integrated_condition(FeedbackMode::Shuffled);
    let random = run_p2_integrated_condition(FeedbackMode::Random);
    let dormant_scaling = run_dormant_scaling();
    let active_scaling = run_active_scaling();
    let slot_diagnostic = run_slot_diagnostic();
    let dormant_flat = dormant_scaling.windows(2).all(|pair| {
        pair[0].active_touches == pair[1].active_touches
            && pair[0].local_encounters == pair[1].local_encounters
            && pair[1].dormant_touches == 0
    });
    let active_grows = active_scaling
        .windows(2)
        .all(|pair| pair[1].local_encounters > pair[0].local_encounters);
    let scaling_accurate = dormant_scaling
        .iter()
        .all(|point| point.held_out_correct == point.held_out_total)
        && active_scaling
            .iter()
            .all(|point| point.held_out_correct == point.held_out_total);
    let passed = lookup.passed
        && roles.passed
        && integrated.competent_seeds == SEEDS
        && integrated.held_out_correct == integrated.held_out_total
        && integrated.fingerprints_unchanged
        && integrated.explicit_answers
        && integrated.queues_empty
        && shuffled.competent_seeds == 0
        && random.competent_seeds == 0
        && dormant_flat
        && active_grows
        && scaling_accurate;
    P2Report {
        lookup,
        roles,
        integrated,
        shuffled,
        random,
        dormant_scaling,
        active_scaling,
        slot_diagnostic,
        passed,
    }
}

pub fn print_p2_report(report: &P2Report) {
    println!("P2 local structural plasticity:");
    println!(
        "  P2a lookup: forward={}/{}, reverse={}/{}, created/used/survive={}/{}/{}, random-stable={}",
        report.lookup.forward_seeds,
        report.lookup.total_seeds,
        report.lookup.reverse_seeds,
        report.lookup.total_seeds,
        report.lookup.average_created,
        report.lookup.average_used,
        report.lookup.average_surviving,
        report.lookup.random_feedback_stable
    );
    println!(
        "  P2b roles: seeds={}/{}, transfer={}/{}, cells={}",
        report.roles.successful_seeds,
        report.roles.total_seeds,
        report.roles.transferred_encodings,
        report.roles.transferred_total,
        report.roles.learned_role_cells
    );
    println!(
        "  P2c integrated: competent={}/{}, held-out={}/{}, roles={:.1}, program={:.1}, created/released/used/peak={}/{}/{}/{}, eligibility updates/evictions={}/{}, first/competent={:?}/{:?}",
        report.integrated.competent_seeds,
        report.integrated.total_seeds,
        report.integrated.held_out_correct,
        report.integrated.held_out_total,
        report.integrated.average_roles,
        report.integrated.average_surviving_program,
        report.integrated.average_created,
        report.integrated.average_released,
        report.integrated.average_used,
        report.integrated.average_peak_probationary,
        report.integrated.average_eligibility_updates,
        report.integrated.total_eligibility_evictions,
        report.integrated.average_first_success_episode,
        report.integrated.average_competence_episode
    );
    let dormant_first = &report.dormant_scaling[0];
    let dormant_last = report.dormant_scaling.last().unwrap();
    let active_first = &report.active_scaling[0];
    let active_last = report.active_scaling.last().unwrap();
    println!(
        "  dormant scaling: cells={}→{}, touches={}→{}, encounters={}→{}, dormant-touches={}",
        dormant_first.total_cells,
        dormant_last.total_cells,
        dormant_first.active_touches,
        dormant_last.active_touches,
        dormant_first.local_encounters,
        dormant_last.local_encounters,
        dormant_last.dormant_touches
    );
    println!(
        "  active distractors: {}→{}, touches={}→{}, encounters={}→{}",
        active_first.active_irrelevant,
        active_last.active_irrelevant,
        active_first.active_touches,
        active_last.active_touches,
        active_first.local_encounters,
        active_last.local_encounters
    );
    println!(
        "  controls: shuffled={}/{}, random={}/{}, passed={}",
        report.shuffled.competent_seeds,
        report.shuffled.total_seeds,
        report.random.competent_seeds,
        report.random.total_seeds,
        report.passed
    );
}

#[cfg(test)]
mod p2_tests {
    use super::*;
    use std::sync::OnceLock;

    fn report() -> &'static P2Report {
        static REPORT: OnceLock<P2Report> = OnceLock::new();
        REPORT.get_or_init(run_p2_experiment)
    }

    #[test]
    fn p2a_discovers_lookup_direction_from_local_bidirectional_plasticity() {
        let report = report();
        assert!(report.lookup.passed);
        assert_eq!(report.lookup.forward_seeds, SEEDS);
        assert_eq!(report.lookup.reverse_seeds, SEEDS);
        assert!(!report.lookup.random_feedback_stable);
    }

    #[test]
    fn p2b_preserves_identity_independent_role_discovery() {
        let report = report();
        assert!(report.roles.passed);
        assert_eq!(report.roles.learned_role_cells, 3);
        assert_eq!(report.roles.permanent_receptor_ids, 0);
    }

    #[test]
    fn p2c_discovers_roles_and_program_from_a_fresh_local_substrate() {
        let report = report();
        assert!(report.passed);
        assert_eq!(report.integrated.competent_seeds, SEEDS);
        assert_eq!(
            report.integrated.held_out_correct,
            report.integrated.held_out_total
        );
        assert!(report.integrated.fingerprints_unchanged);
        assert!(report.integrated.explicit_answers);
        assert!(report.integrated.queues_empty);
    }

    #[test]
    fn p2_cost_tracks_active_structure_not_dormant_capacity() {
        let report = report();
        assert!(report.dormant_scaling.iter().all(|point| {
            point.dormant_touches == 0 && point.held_out_correct == point.held_out_total
        }));
        assert!(report.dormant_scaling.windows(2).all(|pair| {
            pair[0].active_touches == pair[1].active_touches
                && pair[0].local_encounters == pair[1].local_encounters
        }));
        assert!(report
            .active_scaling
            .windows(2)
            .all(|pair| pair[1].local_encounters > pair[0].local_encounters));
    }

    #[test]
    fn p2_credit_updates_only_the_bounded_eligibility_queue() {
        let report = report();
        assert!(report.integrated.average_peak_probationary > 0);
        assert_eq!(report.integrated.average_surviving_program, 4.0);
        assert!(report.integrated.average_eligibility_updates > 0);
        assert_eq!(report.integrated.total_eligibility_evictions, 0);
        assert_eq!(report.shuffled.competent_seeds, 0);
        assert_eq!(report.random.competent_seeds, 0);
    }
}

#[cfg(test)]
mod p21_tests {
    use super::*;
    use std::sync::OnceLock;

    fn report() -> &'static P21Report {
        static REPORT: OnceLock<P21Report> = OnceLock::new();
        REPORT.get_or_init(run_p2_1_experiment)
    }

    #[test]
    fn p21a_learns_role_relative_plasticity_value() {
        let report = report();
        assert!(report.predictor.passed);
        assert_eq!(
            report.predictor.useful_recalled,
            report.predictor.useful_total
        );
        assert_eq!(
            report.predictor.rejected_correctly,
            report.predictor.rejected_total
        );
        assert!(report.predictor.shuffled_useful_recalled < report.predictor.useful_total);
        assert!(report.predictor.fingerprint_unchanged);
    }

    #[test]
    fn p21b_reduces_probationary_churn_without_losing_the_program() {
        let report = report();
        assert!(report.passed);
        assert_eq!(report.learned.competent_seeds, P21_EVALUATION_SEEDS);
        assert_eq!(
            report.learned.held_out_correct,
            report.learned.held_out_total
        );
        assert!(report.learned.average_created < report.always.average_created);
        assert!(report.learned.average_created < report.random.average_created);
        assert!(report.learned.average_created < report.shuffled.average_created);
        assert!(report.learned.average_work < report.always.average_work);
        assert_eq!(report.oracle.competent_seeds, P21_EVALUATION_SEEDS);
    }

    #[test]
    fn p21_counts_the_gate_economics_separately() {
        let report = report();
        assert!(report.learned.average_gate_evaluations > 0);
        assert!(report.learned.average_gate_admissions > 0);
        assert!(report.learned.average_gate_admissions < report.learned.average_gate_evaluations);
        assert!(report.always.average_work > 0);
        assert!(report.learned.average_work > 0);
    }
}

#[cfg(test)]
mod p22_tests {
    use super::*;
    use std::sync::OnceLock;

    fn report() -> &'static P22Report {
        static REPORT: OnceLock<P22Report> = OnceLock::new();
        REPORT.get_or_init(run_p2_2_experiment)
    }

    #[test]
    fn p22a_learns_pre_coupling_representations_separately_from_value() {
        let report = report();
        assert!(report.representation.passed);
        assert!(report.representation.representations > 1);
        assert!(report.representation.mixed_outcome_representations > 0);
        assert!(report.representation.frozen);
    }

    #[test]
    fn p22b_frozen_representations_gate_plasticity_without_supplied_classes() {
        let report = report();
        assert_eq!(report.frozen_gate.competent_seeds, P21_EVALUATION_SEEDS);
        assert_eq!(
            report.frozen_gate.held_out_correct,
            report.frozen_gate.held_out_total
        );
        assert!(report.frozen_gate.average_created < report.always.average_created);
        assert!(report.shuffled.competent_seeds < P21_EVALUATION_SEEDS);
    }

    #[test]
    fn p22c_fresh_lifetime_learning_becomes_selective() {
        let report = report();
        assert!(report.passed);
        assert_eq!(
            report.adaptive.condition.competent_seeds,
            P21_EVALUATION_SEEDS
        );
        assert_eq!(
            report.adaptive.condition.held_out_correct,
            report.adaptive.condition.held_out_total
        );
        assert!(report.adaptive.condition.average_created < report.always.average_created);
        assert!(report.adaptive.condition.average_work < report.always.average_work);
        assert!(
            report.adaptive.late_created_per_episode < report.adaptive.early_created_per_episode
        );
        assert!(report.adaptive.fingerprints_unchanged);
    }
}
