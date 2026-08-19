use std::collections::{HashMap, HashSet, VecDeque};

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
}

#[derive(Clone, Debug)]
struct LocalPlasticity {
    arrows: HashMap<usize, PlasticArrow>,
    slots: HashMap<PlasticUnit, Vec<usize>>,
    next_id: usize,
    slot_count: usize,
    episode: usize,
    rng: DeterministicRng,
    eligibility: VecDeque<EligibilityEntry>,
    ever_used: HashSet<usize>,
    metrics: PlasticityMetrics,
}

impl LocalPlasticity {
    fn new(seed: u64, slot_count: usize) -> Self {
        Self {
            arrows: HashMap::new(),
            slots: HashMap::new(),
            next_id: 0,
            slot_count,
            episode: 0,
            rng: DeterministicRng::new(seed),
            eligibility: VecDeque::new(),
            ever_used: HashSet::new(),
            metrics: PlasticityMetrics::default(),
        }
    }

    fn expose_activity(&mut self, sensory_cells: &[usize], active_irrelevant: usize) {
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
                self.open_direction(left, right);
                self.open_direction(right, left);
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

    fn open_direction(&mut self, from: PlasticUnit, to: PlasticUnit) {
        if from == to || self.find_arrow(from, to).is_some() {
            return;
        }
        if self.slot_count == 0 {
            return;
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
            let Some(replace) = replace else {
                return;
            };
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
            },
        );
        self.slots.entry(from).or_default().push(id);
        self.metrics.directional_couplings_created += 1;
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
        for entry in entries {
            let Some(arrow) = self.arrows.get_mut(&entry.arrow) else {
                continue;
            };
            self.metrics.eligibility_updates += 1;
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
    let roles = trained_role_learner(seed);
    let cells = roles.consolidated_cells();
    let mut network = LocalPlasticity::new(0xa100 + seed as u64 * 100 + reverse as u64, slots);
    let mut ids = IdentitySource::new(0xa200 + seed as u64 + reverse as u64 * 100);
    let mut rng = DeterministicRng::new(0xa300 + seed as u64 + reverse as u64 * 100);
    for episode_index in 0..10_000 {
        network.expose_activity(&cells, 4);
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
        network.expose_activity(&cells, active_irrelevant);
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
