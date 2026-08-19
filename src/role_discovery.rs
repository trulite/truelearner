use std::collections::{HashMap, HashSet, VecDeque};

use crate::binding::{BindingOutcome, IdentitySource, OpaqueId};

const SEEDS: usize = 8;
const ROLE_THRESHOLD: i32 = 4;
const CONSOLIDATION_STRENGTH: i32 = 6;
const SUCCESS_CREDIT: i32 = 2;
const FAILURE_CREDIT: i32 = -1;
const ACTIVITY_LIMIT: usize = 1_600;
const INTEGRATED_BUDGET: usize = 50_000;
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

#[derive(Clone, Debug)]
struct LookupDiscovery {
    arrows: Vec<LookupArrow>,
    rng: DeterministicRng,
    proposals: usize,
    ever_used: HashSet<usize>,
}

impl LookupDiscovery {
    fn new(role_cells: &[usize], seed: u64) -> Self {
        let mut arrows = Vec::new();
        for &from in role_cells {
            for &to in role_cells {
                if from != to {
                    let id = arrows.len();
                    arrows.push(LookupArrow {
                        id,
                        from,
                        to,
                        strength: 0,
                        uses: 0,
                        consolidated: false,
                    });
                }
            }
        }
        Self {
            proposals: arrows.len(),
            arrows,
            rng: DeterministicRng::new(seed),
            ever_used: HashSet::new(),
        }
    }

    fn choose(&mut self) -> Option<LookupArrow> {
        if let Some(arrow) = self.arrows.iter().find(|arrow| arrow.consolidated) {
            return Some(*arrow);
        }
        let best = self
            .arrows
            .iter()
            .map(|arrow| arrow.strength)
            .max()
            .unwrap_or(0);
        let candidates: Vec<_> = self
            .arrows
            .iter()
            .filter(|arrow| arrow.strength == best)
            .map(|arrow| arrow.id)
            .collect();
        let id = candidates[self.rng.index(candidates.len())];
        self.arrows.iter().find(|arrow| arrow.id == id).copied()
    }

    fn evaluated(&self) -> Option<LookupArrow> {
        if let Some(arrow) = self.arrows.iter().find(|arrow| arrow.consolidated) {
            return Some(*arrow);
        }
        let best = self.arrows.iter().map(|arrow| arrow.strength).max()?;
        let candidates: Vec<_> = self
            .arrows
            .iter()
            .filter(|arrow| arrow.strength == best && best > 0)
            .collect();
        if candidates.len() == 1 {
            Some(*candidates[0])
        } else {
            None
        }
    }

    fn feedback(&mut self, used: LookupArrow, success: bool) {
        self.ever_used.insert(used.id);
        let arrow = self
            .arrows
            .iter_mut()
            .find(|arrow| arrow.id == used.id)
            .unwrap();
        arrow.uses += 1;
        arrow.strength += if success {
            SUCCESS_CREDIT
        } else {
            FAILURE_CREDIT
        };
        let best = self
            .arrows
            .iter()
            .map(|arrow| arrow.strength)
            .max()
            .unwrap_or(0);
        let winners: Vec<_> = self
            .arrows
            .iter()
            .filter(|arrow| arrow.strength == best)
            .map(|arrow| arrow.id)
            .collect();
        if best >= CONSOLIDATION_STRENGTH && winners.len() == 1 {
            let winner = winners[0];
            self.arrows.retain(|arrow| arrow.id == winner);
            self.arrows[0].consolidated = true;
        }
    }
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

impl Unit {
    fn is_sensory(self) -> bool {
        matches!(self, Self::Sensory(_))
    }
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

#[derive(Clone, Debug)]
struct ProgramDiscovery {
    arrows: Vec<ProgramArrow>,
    next_id: usize,
    proposals: usize,
    ever_used: HashSet<usize>,
    known_units: HashSet<Unit>,
    rng: DeterministicRng,
}

impl ProgramDiscovery {
    fn new(seed: u64) -> Self {
        Self {
            arrows: Vec::new(),
            next_id: 0,
            proposals: 0,
            ever_used: HashSet::new(),
            known_units: HashSet::new(),
            rng: DeterministicRng::new(seed),
        }
    }

    fn sync_units(&mut self, sensory_cells: &[usize]) {
        let mut units: Vec<_> = sensory_cells.iter().copied().map(Unit::Sensory).collect();
        units.extend(
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
        );
        let current: HashSet<_> = units.iter().copied().collect();
        if current == self.known_units {
            return;
        }
        self.known_units = current;
        for &from in &units {
            for &to in &units {
                if from == to
                    || self
                        .arrows
                        .iter()
                        .any(|arrow| arrow.from == from && arrow.to == to)
                {
                    continue;
                }
                self.arrows.push(ProgramArrow {
                    id: self.next_id,
                    from,
                    to,
                    strength: 0,
                    uses: 0,
                    consolidated: false,
                });
                self.next_id += 1;
                self.proposals += 1;
            }
        }
    }

    fn candidates(&self, class: RouteClass) -> Vec<&ProgramArrow> {
        self.arrows
            .iter()
            .filter(|arrow| match class {
                RouteClass::Lookup => arrow.from.is_sensory() && arrow.to.is_sensory(),
                RouteClass::Feedback => arrow.from == Unit::Internal(InternalRole::Result),
                RouteClass::Continue => arrow.from == Unit::Internal(InternalRole::Success),
                RouteClass::Finish => arrow.from == Unit::Internal(InternalRole::NoResult),
            })
            .collect()
    }

    fn choose(&mut self, class: RouteClass) -> Option<ProgramArrow> {
        let candidates = self.candidates(class);
        if let Some(arrow) = candidates.iter().find(|arrow| arrow.consolidated) {
            return Some(**arrow);
        }
        let best = candidates.iter().map(|arrow| arrow.strength).max()?;
        let choices: Vec<_> = candidates
            .iter()
            .filter(|arrow| arrow.strength == best)
            .map(|arrow| arrow.id)
            .collect();
        let id = choices[self.rng.index(choices.len())];
        self.arrows.iter().find(|arrow| arrow.id == id).copied()
    }

    fn evaluated(&self, class: RouteClass) -> Option<ProgramArrow> {
        let candidates = self.candidates(class);
        if let Some(arrow) = candidates.iter().find(|arrow| arrow.consolidated) {
            return Some(**arrow);
        }
        let best = candidates.iter().map(|arrow| arrow.strength).max()?;
        let strongest: Vec<_> = candidates
            .into_iter()
            .filter(|arrow| arrow.strength == best && best > 0)
            .collect();
        if strongest.len() == 1 {
            Some(*strongest[0])
        } else {
            None
        }
    }

    fn feedback(&mut self, used: &[usize], success: bool) {
        for arrow_id in used {
            self.ever_used.insert(*arrow_id);
            if let Some(arrow) = self.arrows.iter_mut().find(|arrow| arrow.id == *arrow_id) {
                arrow.uses += 1;
                if !arrow.consolidated {
                    arrow.strength += if success {
                        SUCCESS_CREDIT
                    } else {
                        FAILURE_CREDIT
                    };
                }
            }
        }
        for class in [
            RouteClass::Lookup,
            RouteClass::Feedback,
            RouteClass::Continue,
            RouteClass::Finish,
        ] {
            if self
                .candidates(class)
                .iter()
                .any(|arrow| arrow.consolidated)
            {
                continue;
            }
            let candidates = self.candidates(class);
            let Some(best) = candidates.iter().map(|arrow| arrow.strength).max() else {
                continue;
            };
            let winners: Vec<_> = candidates
                .iter()
                .filter(|arrow| arrow.strength == best)
                .map(|arrow| arrow.id)
                .collect();
            if best >= CONSOLIDATION_STRENGTH && winners.len() == 1 {
                let winner = winners[0];
                if let Some(arrow) = self.arrows.iter_mut().find(|arrow| arrow.id == winner) {
                    arrow.consolidated = true;
                }
            }
        }
        if self.complete() {
            self.arrows.retain(|arrow| arrow.consolidated);
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
            self.candidates(class)
                .iter()
                .any(|arrow| arrow.consolidated)
        })
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        let mut arrows = self.arrows.clone();
        arrows.sort_by_key(|arrow| (arrow.from, arrow.to));
        for arrow in arrows {
            fingerprint_mix(&mut hash, arrow.source_code());
            fingerprint_mix(&mut hash, arrow.target_code());
            fingerprint_mix(&mut hash, arrow.strength as i64 as u64);
            fingerprint_mix(&mut hash, arrow.consolidated as u64);
        }
        hash
    }
}

impl ProgramArrow {
    fn source_code(self) -> u64 {
        unit_code(self.from)
    }

    fn target_code(self) -> u64 {
        unit_code(self.to)
    }
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

impl ProgramChoices {
    fn learned(network: &mut ProgramDiscovery) -> Option<Self> {
        Some(Self {
            lookup: network.choose(RouteClass::Lookup)?,
            feedback: network.choose(RouteClass::Feedback)?,
            continuation: network.choose(RouteClass::Continue)?,
            finish: network.choose(RouteClass::Finish)?,
        })
    }

    fn evaluated(network: &ProgramDiscovery) -> Option<Self> {
        Some(Self {
            lookup: network.evaluated(RouteClass::Lookup)?,
            feedback: network.evaluated(RouteClass::Feedback)?,
            continuation: network.evaluated(RouteClass::Continue)?,
            finish: network.evaluated(RouteClass::Finish)?,
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

#[derive(Clone, Debug)]
pub struct LookupDiscoveryReport {
    pub forward_seeds: usize,
    pub reverse_seeds: usize,
    pub total_seeds: usize,
    pub transferred_correct: usize,
    pub transferred_total: usize,
    pub role_fingerprints_unchanged: bool,
    pub random_feedback_stable: bool,
    pub proposed_arrows: usize,
    pub ever_used_arrows: usize,
    pub surviving_arrows: usize,
    pub passed: bool,
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

fn train_lookup_direction(seed: usize, reverse: bool) -> (bool, usize, usize, usize, bool) {
    let roles = trained_role_learner(seed);
    let role_fingerprint = roles.fingerprint();
    let mut network = LookupDiscovery::new(
        &roles.consolidated_cells(),
        0x8700 + seed as u64 + reverse as u64,
    );
    let mut ids = IdentitySource::new(0x8710 + seed as u64 + reverse as u64 * 100);
    let mut rng = DeterministicRng::new(0x8720 + seed as u64 + reverse as u64 * 100);
    for episode_index in 0..2_000 {
        let mut episode = chain_episode(&mut ids, &mut rng, 1, 8);
        if reverse {
            let pair = episode.relations[0];
            episode.query = pair.1;
            episode.correct = BindingOutcome::Answer(pair.0);
        }
        let raw = encode_episode(
            &episode,
            EncodingFamily::Training,
            0x8730 + seed as u64 * 10_000 + episode_index,
        );
        let translated = roles.translate(&raw).unwrap();
        let route = network.choose().unwrap();
        let outcome = execute_lookup(&translated, route, episode.query);
        network.feedback(route, outcome == episode.correct);
        if network.arrows.len() == 1 && network.arrows[0].consolidated {
            break;
        }
    }
    let mut transferred_correct = 0;
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
            0x8740 + seed as u64 * 10_000 + episode_index,
        );
        let translated = roles.translate(&raw).unwrap();
        let outcome = network
            .evaluated()
            .map_or(BindingOutcome::NotFound, |route| {
                execute_lookup(&translated, route, episode.query)
            });
        transferred_correct += usize::from(outcome == episode.correct);
    }
    (
        transferred_correct == 32,
        network.proposals,
        network.ever_used.len(),
        network.arrows.len(),
        role_fingerprint == roles.fingerprint(),
    )
}

fn random_lookup_control() -> bool {
    let roles = trained_role_learner(99);
    let mut network = LookupDiscovery::new(&roles.consolidated_cells(), 0x8790);
    let mut ids = IdentitySource::new(0x8791);
    let mut rng = DeterministicRng::new(0x8792);
    for episode_index in 0..1_000 {
        let episode = chain_episode(&mut ids, &mut rng, 1, 8);
        let raw = encode_episode(&episode, EncodingFamily::Training, 0x8793 + episode_index);
        let translated = roles.translate(&raw).unwrap();
        let route = network.choose().unwrap();
        let _ = execute_lookup(&translated, route, episode.query);
        network.feedback(route, rng.next_u64().is_multiple_of(4));
    }
    network.arrows.len() == 1 && network.arrows[0].consolidated
}

fn run_lookup_discovery() -> LookupDiscoveryReport {
    let mut forward = 0;
    let mut reverse = 0;
    let mut proposals = 0;
    let mut used = 0;
    let mut survivors = 0;
    let mut fingerprints = true;
    for seed in 0..SEEDS {
        let forward_result = train_lookup_direction(seed, false);
        let reverse_result = train_lookup_direction(seed, true);
        forward += usize::from(forward_result.0);
        reverse += usize::from(reverse_result.0);
        proposals += forward_result.1;
        used += forward_result.2;
        survivors += forward_result.3;
        fingerprints &= forward_result.4 && reverse_result.4;
    }
    let random_feedback_stable = random_lookup_control();
    LookupDiscoveryReport {
        forward_seeds: forward,
        reverse_seeds: reverse,
        total_seeds: SEEDS,
        transferred_correct: (forward + reverse) * 32,
        transferred_total: SEEDS * 2 * 32,
        role_fingerprints_unchanged: fingerprints,
        random_feedback_stable,
        proposed_arrows: proposals / SEEDS,
        ever_used_arrows: used / SEEDS,
        surviving_arrows: survivors / SEEDS,
        passed: forward == SEEDS && reverse == SEEDS && fingerprints && !random_feedback_stable,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FeedbackMode {
    Real,
    Shuffled,
    Random,
}

#[derive(Clone, Debug)]
struct IntegratedSeed {
    competent: bool,
    held_out_correct: usize,
    held_out_total: usize,
    roles: usize,
    possible_proposals: usize,
    actual_proposals: usize,
    used_proposals: usize,
    surviving_arrows: usize,
    fingerprint_unchanged: bool,
    explicit_answers: bool,
    queues_empty: bool,
    first_success_episode: Option<usize>,
    competence_episode: Option<usize>,
}

fn run_integrated_seed(seed: usize, mode: FeedbackMode) -> IntegratedSeed {
    let mut roles = SensoryRoleLearner::default();
    let mut program = ProgramDiscovery::new(0x9000 + seed as u64);
    let mut ids = IdentitySource::new(0x9010 + seed as u64);
    let mut rng = DeterministicRng::new(0x9020 + seed as u64);
    let mut feedback_rng = DeterministicRng::new(0x9030 + seed as u64);
    let mut previous_success = false;
    let mut first_success = None;
    let mut competence = None;

    for episode_index in 1..=INTEGRATED_BUDGET {
        let depth = TRAIN_DEPTHS[(episode_index - 1) % TRAIN_DEPTHS.len()];
        let episode = chain_episode(&mut ids, &mut rng, depth, 12);
        let raw = encode_episode(
            &episode,
            EncodingFamily::Training,
            0x9040 + seed as u64 * 100_000 + episode_index as u64,
        );
        roles.observe(&raw);
        let Some(translated) = roles.translate(&raw) else {
            continue;
        };
        program.sync_units(
            &roles
                .patterns
                .iter()
                .map(|pattern| pattern.cell)
                .collect::<Vec<_>>(),
        );
        let Some(choices) = ProgramChoices::learned(&mut program) else {
            continue;
        };
        let run = execute_program(&translated, choices);
        let actual_success =
            run.outcome == episode.correct && !run.activity_limit_hit && run.explicit_answer;
        if actual_success && first_success.is_none() {
            first_success = Some(episode_index);
        }
        let credit = match mode {
            FeedbackMode::Real => actual_success,
            FeedbackMode::Shuffled => previous_success,
            FeedbackMode::Random => feedback_rng.next_u64().is_multiple_of(4),
        };
        previous_success = actual_success;
        program.feedback(&run.used_arrows, credit);
        if competence.is_none() && program.complete() && roles.consolidated_cells().len() == 3 {
            competence = Some(episode_index);
        }
    }

    let fingerprint_before = hash_words(&[roles.fingerprint(), program.fingerprint()]);
    let mut held_out_correct = 0;
    let mut held_out_total = 0;
    let mut explicit_answers = true;
    let mut queues_empty = true;
    if let Some(choices) = ProgramChoices::evaluated(&program) {
        let mut heldout_ids = IdentitySource::new(0x9100 + seed as u64);
        let mut heldout_rng = DeterministicRng::new(0x9110 + seed as u64);
        for depth in HELD_OUT_DEPTHS {
            for episode_index in 0..16 {
                let episode = chain_episode(&mut heldout_ids, &mut heldout_rng, depth, depth + 8);
                let raw = encode_episode(
                    &episode,
                    EncodingFamily::Transferred,
                    0x9120 + seed as u64 * 10_000 + depth as u64 * 100 + episode_index,
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
    let fingerprint_after = hash_words(&[roles.fingerprint(), program.fingerprint()]);
    let unit_count = roles.patterns.len() + 8;
    IntegratedSeed {
        competent: held_out_correct == held_out_total,
        held_out_correct,
        held_out_total,
        roles: roles.consolidated_cells().len(),
        possible_proposals: unit_count * unit_count.saturating_sub(1),
        actual_proposals: program.proposals,
        used_proposals: program.ever_used.len(),
        surviving_arrows: program.arrows.len(),
        fingerprint_unchanged: fingerprint_before == fingerprint_after,
        explicit_answers,
        queues_empty,
        first_success_episode: first_success,
        competence_episode: competence,
    }
}

#[derive(Clone, Debug)]
pub struct IntegratedRoleProgramReport {
    pub competent_seeds: usize,
    pub total_seeds: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub average_roles: f64,
    pub possible_proposals: usize,
    pub actual_proposals: usize,
    pub used_proposals: usize,
    pub surviving_arrows: usize,
    pub fingerprints_unchanged: bool,
    pub explicit_answers: bool,
    pub queues_empty: bool,
    pub average_first_success_episode: Option<f64>,
    pub average_competence_episode: Option<f64>,
}

fn run_integrated_condition(mode: FeedbackMode) -> IntegratedRoleProgramReport {
    let results: Vec<_> = (0..SEEDS)
        .map(|seed| run_integrated_seed(seed, mode))
        .collect();
    let first: Vec<_> = results
        .iter()
        .filter_map(|result| result.first_success_episode)
        .collect();
    let competence: Vec<_> = results
        .iter()
        .filter_map(|result| result.competence_episode)
        .collect();
    IntegratedRoleProgramReport {
        competent_seeds: results.iter().filter(|result| result.competent).count(),
        total_seeds: SEEDS,
        held_out_correct: results.iter().map(|result| result.held_out_correct).sum(),
        held_out_total: results.iter().map(|result| result.held_out_total).sum(),
        average_roles: results.iter().map(|result| result.roles).sum::<usize>() as f64
            / SEEDS as f64,
        possible_proposals: results[0].possible_proposals,
        actual_proposals: results
            .iter()
            .map(|result| result.actual_proposals)
            .sum::<usize>()
            / SEEDS,
        used_proposals: results
            .iter()
            .map(|result| result.used_proposals)
            .sum::<usize>()
            / SEEDS,
        surviving_arrows: results
            .iter()
            .map(|result| result.surviving_arrows)
            .sum::<usize>()
            / SEEDS,
        fingerprints_unchanged: results.iter().all(|result| result.fingerprint_unchanged),
        explicit_answers: results.iter().all(|result| result.explicit_answers),
        queues_empty: results.iter().all(|result| result.queues_empty),
        average_first_success_episode: average(&first),
        average_competence_episode: average(&competence),
    }
}

fn average(values: &[usize]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<usize>() as f64 / values.len() as f64)
}

#[derive(Clone, Debug)]
pub struct P1Report {
    pub roles: RoleDiscoveryReport,
    pub lookup: LookupDiscoveryReport,
    pub integrated: IntegratedRoleProgramReport,
    pub shuffled: IntegratedRoleProgramReport,
    pub random: IntegratedRoleProgramReport,
    pub passed: bool,
}

pub fn run_experiment() -> P1Report {
    let roles = run_role_discovery();
    let lookup = run_lookup_discovery();
    let integrated = run_integrated_condition(FeedbackMode::Real);
    let shuffled = run_integrated_condition(FeedbackMode::Shuffled);
    let random = run_integrated_condition(FeedbackMode::Random);
    let passed = roles.passed
        && lookup.passed
        && integrated.competent_seeds == integrated.total_seeds
        && integrated.held_out_correct == integrated.held_out_total
        && integrated.fingerprints_unchanged
        && integrated.explicit_answers
        && integrated.queues_empty
        && shuffled.competent_seeds == 0
        && random.competent_seeds == 0;
    P1Report {
        roles,
        lookup,
        integrated,
        shuffled,
        random,
        passed,
    }
}

pub fn print_report(report: &P1Report) {
    println!("P1 learned sensory roles and program:");
    println!(
        "  P1a roles: seeds={}/{}, transfer={}/{}, cells={}, receptor-ids={}, symmetric-distinct={}",
        report.roles.successful_seeds,
        report.roles.total_seeds,
        report.roles.transferred_encodings,
        report.roles.transferred_total,
        report.roles.learned_role_cells,
        report.roles.permanent_receptor_ids,
        report.roles.symmetric_field_roles_distinct
    );
    println!(
        "  P1b lookup: forward={}/{}, reverse={}/{}, transfer={}/{}, proposals/used/survive={}/{}/{}",
        report.lookup.forward_seeds,
        report.lookup.total_seeds,
        report.lookup.reverse_seeds,
        report.lookup.total_seeds,
        report.lookup.transferred_correct,
        report.lookup.transferred_total,
        report.lookup.proposed_arrows,
        report.lookup.ever_used_arrows,
        report.lookup.surviving_arrows
    );
    println!(
        "  P1c integrated: competent={}/{}, held-out={}/{}, roles={:.1}, proposals possible/actual/used/survive={}/{}/{}/{}, first/competent={:?}/{:?}",
        report.integrated.competent_seeds,
        report.integrated.total_seeds,
        report.integrated.held_out_correct,
        report.integrated.held_out_total,
        report.integrated.average_roles,
        report.integrated.possible_proposals,
        report.integrated.actual_proposals,
        report.integrated.used_proposals,
        report.integrated.surviving_arrows,
        report.integrated.average_first_success_episode,
        report.integrated.average_competence_episode
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

    fn report() -> &'static P1Report {
        static REPORT: OnceLock<P1Report> = OnceLock::new();
        REPORT.get_or_init(run_experiment)
    }

    #[test]
    fn p1a_discovers_encoding_invariant_positional_structures() {
        let report = report();
        assert!(report.roles.passed);
        assert_eq!(report.roles.learned_role_cells, 3);
        assert_eq!(report.roles.permanent_receptor_ids, 0);
        assert!(!report.roles.symmetric_field_roles_distinct);
        assert!(report.roles.fingerprints_unchanged);
    }

    #[test]
    fn p1b_discovers_both_lookup_directions_through_learned_roles() {
        let report = report();
        assert!(report.lookup.passed);
        assert_eq!(report.lookup.forward_seeds, report.lookup.total_seeds);
        assert_eq!(report.lookup.reverse_seeds, report.lookup.total_seeds);
        assert!(report.lookup.role_fingerprints_unchanged);
        assert!(!report.lookup.random_feedback_stable);
    }

    #[test]
    fn p1c_starts_fresh_and_transfers_the_complete_program_to_new_encodings() {
        let report = report();
        assert!(report.passed);
        assert_eq!(
            report.integrated.competent_seeds,
            report.integrated.total_seeds
        );
        assert_eq!(
            report.integrated.held_out_correct,
            report.integrated.held_out_total
        );
        assert!(report.integrated.fingerprints_unchanged);
        assert!(report.integrated.explicit_answers);
        assert!(report.integrated.queues_empty);
        assert_eq!(report.integrated.surviving_arrows, 4);
    }

    #[test]
    fn p1c_requires_terminal_correctness_for_reliable_program_construction() {
        let report = report();
        assert_eq!(report.shuffled.competent_seeds, 0);
        assert_eq!(report.random.competent_seeds, 0);
    }
}
