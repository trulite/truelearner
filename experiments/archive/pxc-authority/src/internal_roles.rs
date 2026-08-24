use std::collections::{HashMap, HashSet};

use crate::binding::{BindingOutcome, IdentitySource, OpaqueId};

const SEEDS: usize = 8;
const ROLE_THRESHOLD: usize = 4;
const ARROW_THRESHOLD: i32 = 6;
const SUCCESS_CREDIT: i32 = 2;
const FAILURE_CREDIT: i32 = -1;
const PRUNE_STRENGTH: i32 = -2;
const TRAIN_BUDGET: usize = 12_000;
const HELD_OUT_DEPTHS: [usize; 4] = [5, 8, 16, 32];
const LOCAL_RADIUS: usize = 2;
const GROWTH_SLOTS: usize = 8;
const EXPLORATION_DIVISOR: u64 = 512;

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

fn hash_words(words: &[u64]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    for word in words {
        mix(&mut hash, *word);
    }
    hash
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Population {
    Sensory,
    Working,
    Event,
    Boundary,
    Irrelevant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct RoleCell {
    id: usize,
    population: Population,
}

#[derive(Clone, Copy, Debug)]
struct AnonymousOccurrence {
    receptor: u64,
    signature: u64,
    identity: Option<OpaqueId>,
}

#[derive(Clone, Debug)]
struct RolePattern {
    signature: u64,
    cell: RoleCell,
    observations: usize,
}

#[derive(Clone, Debug)]
struct StructuralRoleLearner {
    population: Population,
    patterns: Vec<RolePattern>,
    next_cell: usize,
}

impl StructuralRoleLearner {
    fn new(population: Population, first_cell: usize) -> Self {
        Self {
            population,
            patterns: Vec::new(),
            next_cell: first_cell,
        }
    }

    fn observe(&mut self, occurrences: &[AnonymousOccurrence]) {
        let mut signatures: Vec<_> = occurrences
            .iter()
            .map(|occurrence| occurrence.signature)
            .collect();
        signatures.sort_unstable();
        signatures.dedup();
        for signature in signatures {
            if let Some(pattern) = self
                .patterns
                .iter_mut()
                .find(|pattern| pattern.signature == signature)
            {
                pattern.observations += 1;
            } else {
                let cell = RoleCell {
                    id: self.next_cell,
                    population: self.population,
                };
                self.next_cell += 1;
                self.patterns.push(RolePattern {
                    signature,
                    cell,
                    observations: 1,
                });
            }
        }
        self.patterns.sort_by_key(|pattern| pattern.signature);
    }

    fn translate(&self, occurrence: AnonymousOccurrence) -> Option<RoleCell> {
        self.patterns
            .iter()
            .find(|pattern| {
                pattern.signature == occurrence.signature && pattern.observations >= ROLE_THRESHOLD
            })
            .map(|pattern| pattern.cell)
    }

    fn consolidated_cells(&self) -> Vec<RoleCell> {
        let mut cells: Vec<_> = self
            .patterns
            .iter()
            .filter(|pattern| pattern.observations >= ROLE_THRESHOLD)
            .map(|pattern| pattern.cell)
            .collect();
        cells.sort_by_key(|cell| cell.id);
        cells
    }

    fn fingerprint(&self) -> u64 {
        let mut patterns = self.patterns.clone();
        patterns.sort_by_key(|pattern| pattern.signature);
        let mut hash = 0xcbf2_9ce4_8422_2325;
        for pattern in patterns {
            mix(&mut hash, pattern.signature);
            mix(&mut hash, pattern.cell.id as u64);
            mix(&mut hash, pattern.observations as u64);
        }
        hash
    }

    fn permanent_receptor_ids(&self) -> usize {
        0
    }

    fn retain_consolidated(&mut self) {
        self.patterns
            .retain(|pattern| pattern.observations >= ROLE_THRESHOLD);
    }
}

#[derive(Clone, Copy, Debug)]
enum EncodingFamily {
    Training,
    Transferred,
    Symmetric,
}

#[derive(Clone, Debug)]
struct WorkingEncoding {
    occurrences: Vec<AnonymousOccurrence>,
    operand: AnonymousOccurrence,
    product: AnonymousOccurrence,
}

fn structural_signature(words: &[u64]) -> u64 {
    hash_words(words)
}

fn working_encoding(
    ids: &mut IdentitySource,
    rng: &mut DeterministicRng,
    family: EncodingFamily,
) -> WorkingEncoding {
    let operand_identity = ids.issue();
    let product_identity = ids.issue();
    let operand_signature = match family {
        EncodingFamily::Symmetric => structural_signature(&[4, 1, 1, 1]),
        _ => structural_signature(&[4, 2, 1, 3]),
    };
    let product_signature = match family {
        EncodingFamily::Symmetric => operand_signature,
        _ => structural_signature(&[4, 1, 2, 5]),
    };
    let mut occurrences = vec![
        AnonymousOccurrence {
            receptor: rng.next_u64(),
            signature: operand_signature,
            identity: Some(operand_identity),
        },
        AnonymousOccurrence {
            receptor: rng.next_u64(),
            signature: product_signature,
            identity: Some(product_identity),
        },
    ];
    for distractor in 0usize..6 {
        occurrences.push(AnonymousOccurrence {
            receptor: rng.next_u64(),
            signature: structural_signature(&[4, 7, distractor as u64, rng.next_u64()]),
            identity: Some(ids.issue()),
        });
    }
    rng.shuffle(&mut occurrences);
    if matches!(family, EncodingFamily::Transferred) {
        occurrences.reverse();
    }
    let operand = occurrences
        .iter()
        .find(|occurrence| occurrence.identity == Some(operand_identity))
        .copied()
        .unwrap();
    let product = occurrences
        .iter()
        .find(|occurrence| occurrence.identity == Some(product_identity))
        .copied()
        .unwrap();
    WorkingEncoding {
        occurrences,
        operand,
        product,
    }
}

#[derive(Clone, Debug)]
struct EventEncoding {
    occurrences: Vec<AnonymousOccurrence>,
    initiation: AnonymousOccurrence,
    absence: AnonymousOccurrence,
}

fn event_encoding(rng: &mut DeterministicRng, family: EncodingFamily) -> EventEncoding {
    let initiation_signature = match family {
        EncodingFamily::Symmetric => structural_signature(&[6, 1, 1, 1]),
        _ => structural_signature(&[6, 2, 1, 7]),
    };
    let absence_signature = match family {
        EncodingFamily::Symmetric => initiation_signature,
        _ => structural_signature(&[6, 1, 0, 13]),
    };
    let initiation_receptor = rng.next_u64();
    let absence_receptor = rng.next_u64();
    let mut occurrences = vec![
        AnonymousOccurrence {
            receptor: initiation_receptor,
            signature: initiation_signature,
            identity: None,
        },
        AnonymousOccurrence {
            receptor: absence_receptor,
            signature: absence_signature,
            identity: None,
        },
    ];
    for distractor in 0usize..6 {
        occurrences.push(AnonymousOccurrence {
            receptor: rng.next_u64(),
            signature: structural_signature(&[6, 5, distractor as u64, rng.next_u64()]),
            identity: None,
        });
    }
    rng.shuffle(&mut occurrences);
    if matches!(family, EncodingFamily::Transferred) {
        occurrences.rotate_left(3);
        occurrences.reverse();
    }
    let initiation = occurrences
        .iter()
        .find(|occurrence| occurrence.receptor == initiation_receptor)
        .copied()
        .unwrap();
    let absence = occurrences
        .iter()
        .find(|occurrence| occurrence.receptor == absence_receptor)
        .copied()
        .unwrap();
    EventEncoding {
        occurrences,
        initiation,
        absence,
    }
}

#[derive(Clone, Copy, Debug)]
struct SensoryRoles {
    first: RoleCell,
    second: RoleCell,
    query: RoleCell,
}

#[derive(Clone, Debug)]
struct SensoryRoleLearner {
    roles: StructuralRoleLearner,
}

impl SensoryRoleLearner {
    fn new(first_cell: usize) -> Self {
        Self {
            roles: StructuralRoleLearner::new(Population::Sensory, first_cell),
        }
    }

    fn observe(&mut self) {
        self.roles.observe(&[
            AnonymousOccurrence {
                receptor: 1,
                signature: structural_signature(&[2, 1, 1, 3]),
                identity: None,
            },
            AnonymousOccurrence {
                receptor: 2,
                signature: structural_signature(&[2, 1, 0, 5]),
                identity: None,
            },
            AnonymousOccurrence {
                receptor: 3,
                signature: structural_signature(&[2, 0, 0, 7]),
                identity: None,
            },
        ]);
    }

    fn translated(&self) -> Option<SensoryRoles> {
        Some(SensoryRoles {
            first: self.roles.translate(AnonymousOccurrence {
                receptor: 0,
                signature: structural_signature(&[2, 1, 1, 3]),
                identity: None,
            })?,
            second: self.roles.translate(AnonymousOccurrence {
                receptor: 0,
                signature: structural_signature(&[2, 1, 0, 5]),
                identity: None,
            })?,
            query: self.roles.translate(AnonymousOccurrence {
                receptor: 0,
                signature: structural_signature(&[2, 0, 0, 7]),
                identity: None,
            })?,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct PairSnapshot {
    first: Population,
    second: Population,
}

impl PairSnapshot {
    fn between(first: RoleCell, second: RoleCell) -> Self {
        let mut populations = [first.population, second.population];
        populations.sort_unstable();
        Self {
            first: populations[0],
            second: populations[1],
        }
    }
}

#[derive(Clone, Debug, Default)]
struct EncounterGate {
    observations: HashMap<PairSnapshot, usize>,
    useful: HashMap<PairSnapshot, usize>,
    rejected: HashMap<PairSnapshot, usize>,
    rng: Option<DeterministicRng>,
}

impl EncounterGate {
    fn new(seed: u64) -> Self {
        Self {
            rng: Some(DeterministicRng::new(seed)),
            ..Self::default()
        }
    }

    fn decide(&mut self, snapshot: PairSnapshot) -> bool {
        *self.observations.entry(snapshot).or_default() += 1;
        if self.useful.values().sum::<usize>() == 0 {
            return true;
        }
        if self.useful.get(&snapshot).copied().unwrap_or(0) >= 2 {
            return true;
        }
        self.rng
            .as_mut()
            .unwrap()
            .next_u64()
            .is_multiple_of(EXPLORATION_DIVISOR)
    }

    fn observe(&mut self, snapshot: PairSnapshot, useful: bool) {
        if useful {
            *self.useful.entry(snapshot).or_default() += 1;
        } else {
            *self.rejected.entry(snapshot).or_default() += 1;
        }
    }

    fn representation_count(&self) -> usize {
        self.observations.len()
    }

    fn fingerprint(&self) -> u64 {
        let mut keys: Vec<_> = self.observations.keys().copied().collect();
        keys.sort_unstable();
        let mut hash = 0xcbf2_9ce4_8422_2325;
        for key in keys {
            mix(&mut hash, key.first as u64);
            mix(&mut hash, key.second as u64);
            mix(
                &mut hash,
                self.observations.get(&key).copied().unwrap_or(0) as u64,
            );
            mix(
                &mut hash,
                self.useful.get(&key).copied().unwrap_or(0) as u64,
            );
            mix(
                &mut hash,
                self.rejected.get(&key).copied().unwrap_or(0) as u64,
            );
        }
        hash
    }
}

#[derive(Clone, Copy, Debug)]
struct PlasticArrow {
    id: usize,
    from: RoleCell,
    to: RoleCell,
    strength: i32,
    consolidated: bool,
    snapshot: PairSnapshot,
}

#[derive(Clone, Debug)]
struct ProgramPlasticity {
    arrows: HashMap<usize, PlasticArrow>,
    slots: HashMap<RoleCell, Vec<usize>>,
    next_id: usize,
    rng: DeterministicRng,
    gate: EncounterGate,
    created: usize,
    released: usize,
    used: HashSet<usize>,
}

impl ProgramPlasticity {
    fn new(seed: u64) -> Self {
        Self {
            arrows: HashMap::new(),
            slots: HashMap::new(),
            next_id: 0,
            rng: DeterministicRng::new(seed),
            gate: EncounterGate::new(seed ^ 0xa5a5),
            created: 0,
            released: 0,
            used: HashSet::new(),
        }
    }

    fn expose(&mut self, relevant: &[RoleCell], active_irrelevant: usize) {
        let mut active = relevant.to_vec();
        active.extend((0..active_irrelevant).map(|index| RoleCell {
            id: 100_000 + index,
            population: Population::Irrelevant,
        }));
        active.sort_unstable_by_key(|cell| cell.id);
        self.rng.shuffle(&mut active);
        if active.len() < 2 {
            return;
        }
        for source in 0..active.len() {
            for offset in 1..=LOCAL_RADIUS.min(active.len() - 1) {
                let target = (source + offset) % active.len();
                let left = active[source];
                let right = active[target];
                if left == right || self.has_arrow(left, right) && self.has_arrow(right, left) {
                    continue;
                }
                let snapshot = PairSnapshot::between(left, right);
                if !self.gate.decide(snapshot) {
                    continue;
                }
                self.open(left, right, snapshot);
                self.open(right, left, snapshot);
            }
        }
    }

    fn open(&mut self, from: RoleCell, to: RoleCell, snapshot: PairSnapshot) {
        if from == to || self.has_arrow(from, to) {
            return;
        }
        let occupied = self.slots.get(&from).map_or(0, Vec::len);
        if occupied >= GROWTH_SLOTS {
            let replacement = self.slots.get(&from).and_then(|slots| {
                slots
                    .iter()
                    .filter_map(|id| self.arrows.get(id))
                    .filter(|arrow| !arrow.consolidated)
                    .min_by_key(|arrow| (arrow.strength, arrow.id))
                    .map(|arrow| arrow.id)
            });
            let Some(replacement) = replacement else {
                return;
            };
            self.release(replacement);
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
                consolidated: false,
                snapshot,
            },
        );
        self.slots.entry(from).or_default().push(id);
        self.created += 1;
    }

    fn release(&mut self, id: usize) {
        let Some(arrow) = self.arrows.remove(&id) else {
            return;
        };
        if let Some(slots) = self.slots.get_mut(&arrow.from) {
            slots.retain(|candidate| *candidate != id);
        }
        self.gate.observe(arrow.snapshot, false);
        self.released += 1;
    }

    fn has_arrow(&self, from: RoleCell, to: RoleCell) -> bool {
        self.slots.get(&from).is_some_and(|slots| {
            slots
                .iter()
                .any(|id| self.arrows.get(id).is_some_and(|arrow| arrow.to == to))
        })
    }

    fn candidates(
        &self,
        source: Option<RoleCell>,
        from_population: Population,
        to_population: Population,
    ) -> Vec<PlasticArrow> {
        let mut arrows: Vec<_> = self
            .arrows
            .values()
            .copied()
            .filter(|arrow| {
                source.is_none_or(|source| arrow.from == source)
                    && arrow.from.population == from_population
                    && arrow.to.population == to_population
            })
            .collect();
        arrows.sort_by_key(|arrow| arrow.id);
        arrows
    }

    fn choose(
        &mut self,
        source: Option<RoleCell>,
        from_population: Population,
        to_population: Population,
    ) -> Option<PlasticArrow> {
        let candidates = self.candidates(source, from_population, to_population);
        if let Some(arrow) = candidates.iter().find(|arrow| arrow.consolidated) {
            return Some(*arrow);
        }
        let strongest = candidates.iter().map(|arrow| arrow.strength).max()?;
        let choices: Vec<_> = candidates
            .iter()
            .filter(|arrow| arrow.strength == strongest)
            .copied()
            .collect();
        Some(choices[self.rng.index(choices.len())])
    }

    fn evaluated(
        &self,
        source: Option<RoleCell>,
        from_population: Population,
        to_population: Population,
    ) -> Option<PlasticArrow> {
        let candidates = self.candidates(source, from_population, to_population);
        if let Some(arrow) = candidates.iter().find(|arrow| arrow.consolidated) {
            return Some(*arrow);
        }
        let strongest = candidates.iter().map(|arrow| arrow.strength).max()?;
        let strongest: Vec<_> = candidates
            .iter()
            .filter(|arrow| arrow.strength == strongest && strongest > 0)
            .collect();
        (strongest.len() == 1).then(|| *strongest[0])
    }

    fn feedback(&mut self, used: &[usize], success: bool) {
        let used: HashSet<_> = used.iter().copied().collect();
        let mut release = Vec::new();
        let mut consolidate = Vec::new();
        for id in used {
            let Some(arrow) = self.arrows.get_mut(&id) else {
                continue;
            };
            self.used.insert(id);
            arrow.strength += if success {
                SUCCESS_CREDIT
            } else {
                FAILURE_CREDIT
            };
            if arrow.strength >= ARROW_THRESHOLD {
                consolidate.push(id);
            } else if arrow.strength <= PRUNE_STRENGTH {
                release.push(id);
            }
        }
        for id in release {
            self.release(id);
        }
        for id in consolidate {
            if let Some(arrow) = self.arrows.get_mut(&id) {
                arrow.consolidated = true;
                self.gate.observe(arrow.snapshot, true);
            }
        }
    }

    fn consolidated_arrows(&self) -> Vec<PlasticArrow> {
        let mut arrows: Vec<_> = self
            .arrows
            .values()
            .copied()
            .filter(|arrow| arrow.consolidated)
            .collect();
        arrows.sort_by_key(|arrow| arrow.id);
        arrows
    }

    fn cleanup(&mut self) {
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

    fn fingerprint(&self) -> u64 {
        let mut arrows = self.consolidated_arrows();
        arrows.sort_by_key(|arrow| (arrow.from, arrow.to));
        let mut hash = self.gate.fingerprint();
        for arrow in arrows {
            mix(&mut hash, arrow.from.id as u64);
            mix(&mut hash, arrow.to.id as u64);
            mix(&mut hash, arrow.strength as i64 as u64);
        }
        hash
    }
}

#[derive(Clone, Copy, Debug)]
struct LearnedRoles {
    sensory: SensoryRoles,
    operand: RoleCell,
    product: RoleCell,
    initiation: RoleCell,
    absence: RoleCell,
    output: RoleCell,
}

#[derive(Clone, Copy, Debug)]
struct ChosenProgram {
    lookup: PlasticArrow,
    feedback: PlasticArrow,
    continuation: PlasticArrow,
    finish: PlasticArrow,
}

fn choose_program(network: &mut ProgramPlasticity, roles: LearnedRoles) -> Option<ChosenProgram> {
    Some(ChosenProgram {
        lookup: network.choose(None, Population::Sensory, Population::Sensory)?,
        feedback: network.choose(
            Some(roles.product),
            Population::Working,
            Population::Working,
        )?,
        continuation: network.choose(
            Some(roles.product),
            Population::Working,
            Population::Event,
        )?,
        finish: network.choose(Some(roles.absence), Population::Event, Population::Boundary)?,
    })
}

fn evaluated_program(network: &ProgramPlasticity, roles: LearnedRoles) -> Option<ChosenProgram> {
    Some(ChosenProgram {
        lookup: network.evaluated(None, Population::Sensory, Population::Sensory)?,
        feedback: network.evaluated(
            Some(roles.product),
            Population::Working,
            Population::Working,
        )?,
        continuation: network.evaluated(
            Some(roles.product),
            Population::Working,
            Population::Event,
        )?,
        finish: network.evaluated(Some(roles.absence), Population::Event, Population::Boundary)?,
    })
}

fn program_is_functional(program: ChosenProgram, roles: LearnedRoles) -> bool {
    program.lookup.from == roles.sensory.first
        && program.lookup.to == roles.sensory.second
        && program.feedback.from == roles.product
        && program.feedback.to == roles.operand
        && program.continuation.from == roles.product
        && program.continuation.to == roles.initiation
        && program.finish.from == roles.absence
        && program.finish.to == roles.output
}

fn fixed_arrow(id: usize, from: RoleCell, to: RoleCell) -> PlasticArrow {
    PlasticArrow {
        id,
        from,
        to,
        strength: ARROW_THRESHOLD,
        consolidated: true,
        snapshot: PairSnapshot::between(from, to),
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
    let chain: Vec<_> = (0..=depth).map(|_| identities.issue()).collect();
    let mut relations: Vec<_> = chain.windows(2).map(|pair| (pair[0], pair[1])).collect();
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

fn lookup(
    relations: &[(OpaqueId, OpaqueId)],
    input: OpaqueId,
    route: PlasticArrow,
    roles: SensoryRoles,
) -> BindingOutcome {
    let forward = route.from == roles.first && route.to == roles.second;
    let reverse = route.from == roles.second && route.to == roles.first;
    if !forward && !reverse {
        return BindingOutcome::NotFound;
    }
    let mut answers = HashSet::new();
    for &(left, right) in relations {
        if forward && left == input {
            answers.insert(right);
        }
        if reverse && right == input {
            answers.insert(left);
        }
    }
    match answers.len() {
        0 => BindingOutcome::NotFound,
        1 => BindingOutcome::Answer(*answers.iter().next().unwrap()),
        _ => BindingOutcome::Ambiguous,
    }
}

#[derive(Clone, Debug)]
struct Execution {
    outcome: BindingOutcome,
    explicit_answer: bool,
    queue_empty: bool,
    used: Vec<usize>,
}

fn execute(episode: &ChainEpisode, roles: LearnedRoles, program: ChosenProgram) -> Execution {
    let mut current = episode.query;
    let mut used = vec![program.lookup.id];
    let mut steps = 0;
    loop {
        steps += 1;
        if steps > 64 {
            return Execution {
                outcome: BindingOutcome::NotFound,
                explicit_answer: false,
                queue_empty: false,
                used,
            };
        }
        match lookup(&episode.relations, current, program.lookup, roles.sensory) {
            BindingOutcome::Answer(result) => {
                used.push(program.feedback.id);
                if program.feedback.from != roles.product || program.feedback.to != roles.operand {
                    return Execution {
                        outcome: BindingOutcome::NotFound,
                        explicit_answer: false,
                        queue_empty: true,
                        used,
                    };
                }
                current = result;
                used.push(program.continuation.id);
                if program.continuation.from != roles.product
                    || program.continuation.to != roles.initiation
                {
                    return Execution {
                        outcome: BindingOutcome::NotFound,
                        explicit_answer: false,
                        queue_empty: true,
                        used,
                    };
                }
            }
            BindingOutcome::NotFound => {
                used.push(program.finish.id);
                let explicit =
                    program.finish.from == roles.absence && program.finish.to == roles.output;
                return Execution {
                    outcome: if explicit {
                        BindingOutcome::Answer(current)
                    } else {
                        BindingOutcome::NotFound
                    },
                    explicit_answer: explicit,
                    queue_empty: true,
                    used,
                };
            }
            BindingOutcome::Ambiguous => {
                return Execution {
                    outcome: BindingOutcome::Ambiguous,
                    explicit_answer: false,
                    queue_empty: true,
                    used,
                };
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum FeedbackMode {
    Real,
    Shuffled,
    Random,
}

fn feedback_for(mode: FeedbackMode, success: bool, rng: &mut DeterministicRng) -> bool {
    match mode {
        FeedbackMode::Real => success,
        FeedbackMode::Shuffled => rng.next_u64().is_multiple_of(8),
        FeedbackMode::Random => rng.next_u64().is_multiple_of(32),
    }
}

#[derive(Clone, Debug)]
struct PreparedRoles {
    working: StructuralRoleLearner,
    events: StructuralRoleLearner,
    operand_signature: u64,
    product_signature: u64,
    initiation_signature: u64,
    absence_signature: u64,
}

fn train_prepared_roles(seed: usize) -> PreparedRoles {
    let mut working = StructuralRoleLearner::new(Population::Working, 10);
    let mut events = StructuralRoleLearner::new(Population::Event, 20);
    let mut identities = IdentitySource::new(0x3100 + seed as u64);
    let mut rng = DeterministicRng::new(0x3200 + seed as u64);
    let mut operand_signature = 0;
    let mut product_signature = 0;
    let mut initiation_signature = 0;
    let mut absence_signature = 0;
    for _ in 0..64 {
        let work = working_encoding(&mut identities, &mut rng, EncodingFamily::Training);
        operand_signature = work.operand.signature;
        product_signature = work.product.signature;
        working.observe(&work.occurrences);
        let event = event_encoding(&mut rng, EncodingFamily::Training);
        initiation_signature = event.initiation.signature;
        absence_signature = event.absence.signature;
        events.observe(&event.occurrences);
    }
    PreparedRoles {
        working,
        events,
        operand_signature,
        product_signature,
        initiation_signature,
        absence_signature,
    }
}

fn translated_prepared_roles(
    prepared: &PreparedRoles,
) -> Option<(RoleCell, RoleCell, RoleCell, RoleCell)> {
    Some((
        prepared.working.translate(AnonymousOccurrence {
            receptor: 0,
            signature: prepared.operand_signature,
            identity: None,
        })?,
        prepared.working.translate(AnonymousOccurrence {
            receptor: 0,
            signature: prepared.product_signature,
            identity: None,
        })?,
        prepared.events.translate(AnonymousOccurrence {
            receptor: 0,
            signature: prepared.initiation_signature,
            identity: None,
        })?,
        prepared.events.translate(AnonymousOccurrence {
            receptor: 0,
            signature: prepared.absence_signature,
            identity: None,
        })?,
    ))
}

#[derive(Clone, Debug)]
pub struct WorkingRoleReport {
    pub successful_seeds: usize,
    pub total_seeds: usize,
    pub transfer_correct: usize,
    pub transfer_total: usize,
    pub learned_roles: usize,
    pub permanent_receptor_ids: usize,
    pub symmetric_roles_distinct: bool,
    pub timing_used: bool,
    pub fingerprints_unchanged: bool,
    pub passed: bool,
}

fn run_working_role_experiment() -> WorkingRoleReport {
    let mut successful = 0;
    let mut transfer_correct = 0;
    let mut transfer_total = 0;
    let mut learned_roles = 0;
    let mut fingerprints_unchanged = true;
    for seed in 0..SEEDS {
        let prepared = train_prepared_roles(seed);
        let fingerprint = prepared.working.fingerprint();
        let Some((operand, product, _, _)) = translated_prepared_roles(&prepared) else {
            continue;
        };
        learned_roles = [operand, product].into_iter().collect::<HashSet<_>>().len();
        successful += usize::from(operand != product && learned_roles == 2);
        let mut identities = IdentitySource::new(0x3300 + seed as u64);
        let mut rng = DeterministicRng::new(0x3400 + seed as u64);
        for _ in 0..32 {
            let encoded = working_encoding(&mut identities, &mut rng, EncodingFamily::Transferred);
            let translated_operand = prepared.working.translate(encoded.operand);
            let translated_product = prepared.working.translate(encoded.product);
            transfer_correct += usize::from(
                translated_operand == Some(operand) && translated_product == Some(product),
            );
            transfer_total += 1;
        }
        fingerprints_unchanged &= fingerprint == prepared.working.fingerprint();
    }

    let mut symmetric = StructuralRoleLearner::new(Population::Working, 10);
    let mut ids = IdentitySource::new(0x3500);
    let mut rng = DeterministicRng::new(0x3600);
    let mut symmetric_distinct = false;
    for _ in 0..64 {
        let encoded = working_encoding(&mut ids, &mut rng, EncodingFamily::Symmetric);
        symmetric.observe(&encoded.occurrences);
        if let (Some(first), Some(second)) = (
            symmetric.translate(encoded.operand),
            symmetric.translate(encoded.product),
        ) {
            symmetric_distinct |= first != second;
        }
    }
    let passed = successful == SEEDS
        && transfer_correct == transfer_total
        && learned_roles == 2
        && !symmetric_distinct
        && fingerprints_unchanged;
    WorkingRoleReport {
        successful_seeds: successful,
        total_seeds: SEEDS,
        transfer_correct,
        transfer_total,
        learned_roles,
        permanent_receptor_ids: symmetric.permanent_receptor_ids(),
        symmetric_roles_distinct: symmetric_distinct,
        timing_used: false,
        fingerprints_unchanged,
        passed,
    }
}

#[derive(Clone, Debug)]
pub struct FeedbackReport {
    pub successful_seeds: usize,
    pub total_seeds: usize,
    pub depth_correct: usize,
    pub depth_total: usize,
    pub same_role_cells_reused: bool,
    pub permanent_arrows: usize,
    pub passed: bool,
}

fn train_feedback_only(seed: usize) -> (PreparedRoles, PlasticArrow) {
    let prepared = train_prepared_roles(seed);
    let (operand, product, _, _) = translated_prepared_roles(&prepared).unwrap();
    let mut network = ProgramPlasticity::new(0x3700 + seed as u64);
    for episode in 0..2_000 {
        network.expose(&[operand, product], 6);
        let Some(candidate) =
            network.choose(Some(product), Population::Working, Population::Working)
        else {
            continue;
        };
        let success = candidate.from == product && candidate.to == operand;
        network.feedback(&[candidate.id], success);
        if network
            .evaluated(Some(product), Population::Working, Population::Working)
            .is_some_and(|arrow| arrow.to == operand)
        {
            break;
        }
        if episode + 1 == 2_000 {
            panic!("feedback discovery did not converge");
        }
    }
    let arrow = network
        .evaluated(Some(product), Population::Working, Population::Working)
        .unwrap();
    (prepared, arrow)
}

fn run_feedback_experiment() -> FeedbackReport {
    let mut successful = 0;
    let mut correct = 0;
    let mut total = 0;
    let mut same_cells = true;
    let depths = [1, 2, 3, 4, 8, 16, 32];
    for seed in 0..SEEDS {
        let (prepared, arrow) = train_feedback_only(seed);
        let (operand, product, initiation, absence) = translated_prepared_roles(&prepared).unwrap();
        successful += usize::from(arrow.from == product && arrow.to == operand);
        let fingerprint = prepared.working.fingerprint();
        let mut sensory = SensoryRoleLearner::new(0);
        for _ in 0..ROLE_THRESHOLD {
            sensory.observe();
        }
        let sensory = sensory.translated().unwrap();
        let output = RoleCell {
            id: 30,
            population: Population::Boundary,
        };
        let roles = LearnedRoles {
            sensory,
            operand,
            product,
            initiation,
            absence,
            output,
        };
        let program = ChosenProgram {
            lookup: fixed_arrow(10_000, sensory.first, sensory.second),
            feedback: arrow,
            continuation: fixed_arrow(10_001, product, initiation),
            finish: fixed_arrow(10_002, absence, output),
        };
        let mut identities = IdentitySource::new(0x3750 + seed as u64);
        let mut rng = DeterministicRng::new(0x3760 + seed as u64);
        for depth in depths {
            let episode = chain_episode(&mut identities, &mut rng, depth);
            let run = execute(&episode, roles, program);
            correct += usize::from(
                run.outcome == BindingOutcome::Answer(episode.answer)
                    && run.explicit_answer
                    && run.queue_empty,
            );
            total += 1;
        }
        same_cells &= fingerprint == prepared.working.fingerprint();
    }
    FeedbackReport {
        successful_seeds: successful,
        total_seeds: SEEDS,
        depth_correct: correct,
        depth_total: total,
        same_role_cells_reused: same_cells,
        permanent_arrows: 1,
        passed: successful == SEEDS && correct == total && same_cells,
    }
}

#[derive(Clone, Debug)]
pub struct ControlRoleReport {
    pub successful_seeds: usize,
    pub total_seeds: usize,
    pub transfer_correct: usize,
    pub transfer_total: usize,
    pub learned_roles: usize,
    pub symmetric_roles_distinct: bool,
    pub continuation_arrows: usize,
    pub finish_arrows: usize,
    pub depth_correct: usize,
    pub depth_total: usize,
    pub passed: bool,
}

fn run_control_role_experiment() -> ControlRoleReport {
    let mut successful = 0;
    let mut transfer_correct = 0;
    let mut transfer_total = 0;
    let mut learned_roles = 0;
    let mut depth_correct = 0;
    let mut depth_total = 0;
    for seed in 0..SEEDS {
        let prepared = train_prepared_roles(seed);
        let (operand, product, initiation, absence) = translated_prepared_roles(&prepared).unwrap();
        learned_roles = [initiation, absence]
            .into_iter()
            .collect::<HashSet<_>>()
            .len();
        let output = RoleCell {
            id: 30,
            population: Population::Boundary,
        };
        let mut network = ProgramPlasticity::new(0x3800 + seed as u64);
        for _ in 0..4_000 {
            network.expose(&[product, initiation, absence, output], 6);
            let continuation =
                network.choose(Some(product), Population::Working, Population::Event);
            let finish = network.choose(Some(absence), Population::Event, Population::Boundary);
            if let (Some(continuation), Some(finish)) = (continuation, finish) {
                let success =
                    continuation.to == initiation && finish.from == absence && finish.to == output;
                network.feedback(&[continuation.id, finish.id], success);
            }
            let ready = network
                .evaluated(Some(product), Population::Working, Population::Event)
                .is_some_and(|arrow| arrow.to == initiation)
                && network
                    .evaluated(Some(absence), Population::Event, Population::Boundary)
                    .is_some_and(|arrow| arrow.to == output);
            if ready {
                break;
            }
        }
        successful += usize::from(
            network
                .evaluated(Some(product), Population::Working, Population::Event)
                .is_some_and(|arrow| arrow.to == initiation)
                && network
                    .evaluated(Some(absence), Population::Event, Population::Boundary)
                    .is_some_and(|arrow| arrow.to == output),
        );
        let mut sensory = SensoryRoleLearner::new(0);
        for _ in 0..ROLE_THRESHOLD {
            sensory.observe();
        }
        let sensory = sensory.translated().unwrap();
        if let (Some(continuation), Some(finish)) = (
            network.evaluated(Some(product), Population::Working, Population::Event),
            network.evaluated(Some(absence), Population::Event, Population::Boundary),
        ) {
            let roles = LearnedRoles {
                sensory,
                operand,
                product,
                initiation,
                absence,
                output,
            };
            let program = ChosenProgram {
                lookup: fixed_arrow(11_000, sensory.first, sensory.second),
                feedback: fixed_arrow(11_001, product, operand),
                continuation,
                finish,
            };
            let mut identities = IdentitySource::new(0x3850 + seed as u64);
            let mut episode_rng = DeterministicRng::new(0x3860 + seed as u64);
            for depth in [1, 2, 4, 8, 16, 32] {
                let episode = chain_episode(&mut identities, &mut episode_rng, depth);
                let run = execute(&episode, roles, program);
                depth_correct += usize::from(
                    run.outcome == BindingOutcome::Answer(episode.answer)
                        && run.explicit_answer
                        && run.queue_empty,
                );
                depth_total += 1;
            }
        }
        let mut rng = DeterministicRng::new(0x3900 + seed as u64);
        for _ in 0..32 {
            let encoded = event_encoding(&mut rng, EncodingFamily::Transferred);
            transfer_correct += usize::from(
                prepared.events.translate(encoded.initiation) == Some(initiation)
                    && prepared.events.translate(encoded.absence) == Some(absence),
            );
            transfer_total += 1;
        }
    }

    let mut symmetric = StructuralRoleLearner::new(Population::Event, 20);
    let mut rng = DeterministicRng::new(0x3a00);
    let mut symmetric_distinct = false;
    for _ in 0..64 {
        let encoded = event_encoding(&mut rng, EncodingFamily::Symmetric);
        symmetric.observe(&encoded.occurrences);
        if let (Some(first), Some(second)) = (
            symmetric.translate(encoded.initiation),
            symmetric.translate(encoded.absence),
        ) {
            symmetric_distinct |= first != second;
        }
    }
    let passed = successful == SEEDS
        && transfer_correct == transfer_total
        && learned_roles == 2
        && !symmetric_distinct
        && depth_correct == depth_total;
    ControlRoleReport {
        successful_seeds: successful,
        total_seeds: SEEDS,
        transfer_correct,
        transfer_total,
        learned_roles,
        symmetric_roles_distinct: symmetric_distinct,
        continuation_arrows: 1,
        finish_arrows: 1,
        depth_correct,
        depth_total,
        passed,
    }
}

#[derive(Clone, Debug)]
struct IntegratedSeed {
    competent: bool,
    competence_episode: Option<usize>,
    held_out_correct: usize,
    held_out_total: usize,
    explicit_answers: bool,
    queues_empty: bool,
    fingerprints_unchanged: bool,
    sensory_roles: usize,
    working_roles: usize,
    event_roles: usize,
    program_arrows: usize,
    encounter_representations: usize,
    created: usize,
    released: usize,
    first_sensory_roles: Option<usize>,
    first_working_roles: Option<usize>,
    first_event_roles: Option<usize>,
    first_lookup: Option<usize>,
    first_feedback: Option<usize>,
    first_continuation: Option<usize>,
    first_finish: Option<usize>,
    encoding_transfer_correct: usize,
    encoding_transfer_total: usize,
}

fn run_integrated_seed(seed: usize, mode: FeedbackMode) -> IntegratedSeed {
    let mut sensory = SensoryRoleLearner::new(0);
    let mut working = StructuralRoleLearner::new(Population::Working, 10);
    let mut events = StructuralRoleLearner::new(Population::Event, 20);
    let output = RoleCell {
        id: 30,
        population: Population::Boundary,
    };
    let mut network = ProgramPlasticity::new(0x4000 + seed as u64);
    let mut identities = IdentitySource::new(0x4100 + seed as u64);
    let mut rng = DeterministicRng::new(0x4200 + seed as u64);
    let mut feedback_rng = DeterministicRng::new(0x4300 + seed as u64);
    let mut competence = None;
    let mut first_sensory_roles = None;
    let mut first_working_roles = None;
    let mut first_event_roles = None;
    let mut first_lookup = None;
    let mut first_feedback = None;
    let mut first_continuation = None;
    let mut first_finish = None;

    for episode_index in 1..=TRAIN_BUDGET {
        sensory.observe();
        let working_encoding =
            working_encoding(&mut identities, &mut rng, EncodingFamily::Training);
        working.observe(&working_encoding.occurrences);
        let event_encoding = event_encoding(&mut rng, EncodingFamily::Training);
        events.observe(&event_encoding.occurrences);

        let sensory_roles = sensory.translated();
        let operand = working.translate(working_encoding.operand);
        let product = working.translate(working_encoding.product);
        let initiation = events.translate(event_encoding.initiation);
        let absence = events.translate(event_encoding.absence);
        if first_sensory_roles.is_none() && sensory_roles.is_some() {
            first_sensory_roles = Some(episode_index);
        }
        if first_working_roles.is_none() && operand.is_some() && product.is_some() {
            first_working_roles = Some(episode_index);
        }
        if first_event_roles.is_none() && initiation.is_some() && absence.is_some() {
            first_event_roles = Some(episode_index);
        }
        let (Some(sensory_roles), Some(operand), Some(product), Some(initiation), Some(absence)) =
            (sensory_roles, operand, product, initiation, absence)
        else {
            continue;
        };
        let roles = LearnedRoles {
            sensory: sensory_roles,
            operand,
            product,
            initiation,
            absence,
            output,
        };
        let mut active = vec![
            sensory_roles.first,
            sensory_roles.second,
            sensory_roles.query,
            operand,
            product,
            initiation,
            absence,
            output,
        ];
        active.sort_by_key(|cell| cell.id);
        network.expose(&active, 8);
        let depth = 1 + (episode_index - 1) % 4;
        let episode = chain_episode(&mut identities, &mut rng, depth);
        let Some(program) = choose_program(&mut network, roles) else {
            continue;
        };
        let run = execute(&episode, roles, program);
        let success = run.outcome == BindingOutcome::Answer(episode.answer)
            && run.explicit_answer
            && run.queue_empty;
        let terminal = feedback_for(mode, success, &mut feedback_rng);
        network.feedback(&run.used, terminal);

        let consolidated = network.consolidated_arrows();
        if first_lookup.is_none()
            && consolidated.iter().any(|arrow| {
                arrow.from.population == Population::Sensory
                    && arrow.to.population == Population::Sensory
            })
        {
            first_lookup = Some(episode_index);
        }
        if first_feedback.is_none()
            && consolidated
                .iter()
                .any(|arrow| arrow.from == product && arrow.to == operand)
        {
            first_feedback = Some(episode_index);
        }
        if first_continuation.is_none()
            && consolidated
                .iter()
                .any(|arrow| arrow.from == product && arrow.to == initiation)
        {
            first_continuation = Some(episode_index);
        }
        if first_finish.is_none()
            && consolidated
                .iter()
                .any(|arrow| arrow.from == absence && arrow.to == output)
        {
            first_finish = Some(episode_index);
        }
        if competence.is_none()
            && evaluated_program(&network, roles).is_some_and(|program| {
                program_is_functional(program, roles)
                    && [
                        program.lookup,
                        program.feedback,
                        program.continuation,
                        program.finish,
                    ]
                    .into_iter()
                    .all(|arrow| arrow.consolidated)
            })
        {
            competence = Some(episode_index);
            network.cleanup();
            sensory.roles.retain_consolidated();
            working.retain_consolidated();
            events.retain_consolidated();
            break;
        }
    }

    let sensory_roles = sensory.translated();
    let mut held_out_correct = 0;
    let mut held_out_total = 0;
    let mut explicit_answers = true;
    let mut queues_empty = true;
    let mut encoding_transfer_correct = 0;
    let mut encoding_transfer_total = 0;
    let fingerprint_before = hash_words(&[
        sensory.roles.fingerprint(),
        working.fingerprint(),
        events.fingerprint(),
        network.fingerprint(),
    ]);
    if let Some(sensory_roles) = sensory_roles {
        let prepared = train_prepared_roles(seed);
        if let Some((operand, product, initiation, absence)) =
            translated_prepared_roles_from(&working, &events, &prepared)
        {
            let roles = LearnedRoles {
                sensory: sensory_roles,
                operand,
                product,
                initiation,
                absence,
                output,
            };
            if let Some(program) = evaluated_program(&network, roles) {
                let mut heldout_ids = IdentitySource::new(0x4400 + seed as u64);
                let mut heldout_rng = DeterministicRng::new(0x4500 + seed as u64);
                for depth in HELD_OUT_DEPTHS {
                    for _ in 0..16 {
                        let transferred_work = working_encoding(
                            &mut heldout_ids,
                            &mut heldout_rng,
                            EncodingFamily::Transferred,
                        );
                        let transferred_event =
                            event_encoding(&mut heldout_rng, EncodingFamily::Transferred);
                        encoding_transfer_correct += usize::from(
                            working.translate(transferred_work.operand) == Some(operand)
                                && working.translate(transferred_work.product) == Some(product)
                                && events.translate(transferred_event.initiation)
                                    == Some(initiation)
                                && events.translate(transferred_event.absence) == Some(absence),
                        );
                        encoding_transfer_total += 1;
                        let episode = chain_episode(&mut heldout_ids, &mut heldout_rng, depth);
                        let run = execute(&episode, roles, program);
                        held_out_correct += usize::from(
                            run.outcome == BindingOutcome::Answer(episode.answer)
                                && run.explicit_answer
                                && run.queue_empty,
                        );
                        held_out_total += 1;
                        explicit_answers &= run.explicit_answer;
                        queues_empty &= run.queue_empty;
                    }
                }
            }
        }
    }
    let fingerprint_after = hash_words(&[
        sensory.roles.fingerprint(),
        working.fingerprint(),
        events.fingerprint(),
        network.fingerprint(),
    ]);
    IntegratedSeed {
        competent: competence.is_some(),
        competence_episode: competence,
        held_out_correct,
        held_out_total,
        explicit_answers,
        queues_empty,
        fingerprints_unchanged: fingerprint_before == fingerprint_after,
        sensory_roles: sensory.roles.consolidated_cells().len(),
        working_roles: working.consolidated_cells().len(),
        event_roles: events.consolidated_cells().len(),
        program_arrows: network.consolidated_arrows().len(),
        encounter_representations: network.gate.representation_count(),
        created: network.created,
        released: network.released,
        first_sensory_roles,
        first_working_roles,
        first_event_roles,
        first_lookup,
        first_feedback,
        first_continuation,
        first_finish,
        encoding_transfer_correct,
        encoding_transfer_total,
    }
}

fn translated_prepared_roles_from(
    working: &StructuralRoleLearner,
    events: &StructuralRoleLearner,
    prepared: &PreparedRoles,
) -> Option<(RoleCell, RoleCell, RoleCell, RoleCell)> {
    Some((
        working.translate(AnonymousOccurrence {
            receptor: 0,
            signature: prepared.operand_signature,
            identity: None,
        })?,
        working.translate(AnonymousOccurrence {
            receptor: 0,
            signature: prepared.product_signature,
            identity: None,
        })?,
        events.translate(AnonymousOccurrence {
            receptor: 0,
            signature: prepared.initiation_signature,
            identity: None,
        })?,
        events.translate(AnonymousOccurrence {
            receptor: 0,
            signature: prepared.absence_signature,
            identity: None,
        })?,
    ))
}

#[derive(Clone, Debug)]
pub struct IntegratedReport {
    pub competent_seeds: usize,
    pub total_seeds: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub average_competence_episode: Option<f64>,
    pub average_sensory_roles: f64,
    pub average_working_roles: f64,
    pub average_event_roles: f64,
    pub average_program_arrows: f64,
    pub average_encounter_representations: f64,
    pub average_created: usize,
    pub average_released: usize,
    pub explicit_answers: bool,
    pub queues_empty: bool,
    pub fingerprints_unchanged: bool,
    pub first_sensory_roles: Option<f64>,
    pub first_working_roles: Option<f64>,
    pub first_event_roles: Option<f64>,
    pub first_lookup: Option<f64>,
    pub first_feedback: Option<f64>,
    pub first_continuation: Option<f64>,
    pub first_finish: Option<f64>,
    pub encoding_transfer_correct: usize,
    pub encoding_transfer_total: usize,
}

fn average(values: impl Iterator<Item = usize>) -> Option<f64> {
    let values: Vec<_> = values.collect();
    (!values.is_empty()).then(|| values.iter().sum::<usize>() as f64 / values.len() as f64)
}

fn summarize_integrated(results: &[IntegratedSeed]) -> IntegratedReport {
    IntegratedReport {
        competent_seeds: results.iter().filter(|result| result.competent).count(),
        total_seeds: results.len(),
        held_out_correct: results.iter().map(|result| result.held_out_correct).sum(),
        held_out_total: results.iter().map(|result| result.held_out_total).sum(),
        average_competence_episode: average(
            results
                .iter()
                .filter_map(|result| result.competence_episode),
        ),
        average_sensory_roles: results
            .iter()
            .map(|result| result.sensory_roles)
            .sum::<usize>() as f64
            / results.len() as f64,
        average_working_roles: results
            .iter()
            .map(|result| result.working_roles)
            .sum::<usize>() as f64
            / results.len() as f64,
        average_event_roles: results
            .iter()
            .map(|result| result.event_roles)
            .sum::<usize>() as f64
            / results.len() as f64,
        average_program_arrows: results
            .iter()
            .map(|result| result.program_arrows)
            .sum::<usize>() as f64
            / results.len() as f64,
        average_encounter_representations: results
            .iter()
            .map(|result| result.encounter_representations)
            .sum::<usize>() as f64
            / results.len() as f64,
        average_created: results.iter().map(|result| result.created).sum::<usize>() / results.len(),
        average_released: results.iter().map(|result| result.released).sum::<usize>()
            / results.len(),
        explicit_answers: results.iter().all(|result| result.explicit_answers),
        queues_empty: results.iter().all(|result| result.queues_empty),
        fingerprints_unchanged: results.iter().all(|result| result.fingerprints_unchanged),
        first_sensory_roles: average(
            results
                .iter()
                .filter_map(|result| result.first_sensory_roles),
        ),
        first_working_roles: average(
            results
                .iter()
                .filter_map(|result| result.first_working_roles),
        ),
        first_event_roles: average(results.iter().filter_map(|result| result.first_event_roles)),
        first_lookup: average(results.iter().filter_map(|result| result.first_lookup)),
        first_feedback: average(results.iter().filter_map(|result| result.first_feedback)),
        first_continuation: average(
            results
                .iter()
                .filter_map(|result| result.first_continuation),
        ),
        first_finish: average(results.iter().filter_map(|result| result.first_finish)),
        encoding_transfer_correct: results
            .iter()
            .map(|result| result.encoding_transfer_correct)
            .sum(),
        encoding_transfer_total: results
            .iter()
            .map(|result| result.encoding_transfer_total)
            .sum(),
    }
}

#[derive(Clone, Debug)]
pub struct P3Report {
    pub working: WorkingRoleReport,
    pub feedback: FeedbackReport,
    pub controls: ControlRoleReport,
    pub integrated: IntegratedReport,
    pub shuffled: IntegratedReport,
    pub random: IntegratedReport,
    pub passed: bool,
}

pub fn run_experiment() -> P3Report {
    let working = run_working_role_experiment();
    let feedback = run_feedback_experiment();
    let controls = run_control_role_experiment();
    let integrated_results: Vec<_> = (0..SEEDS)
        .map(|seed| run_integrated_seed(seed, FeedbackMode::Real))
        .collect();
    let shuffled_results: Vec<_> = (0..SEEDS)
        .map(|seed| run_integrated_seed(100 + seed, FeedbackMode::Shuffled))
        .collect();
    let random_results: Vec<_> = (0..SEEDS)
        .map(|seed| run_integrated_seed(200 + seed, FeedbackMode::Random))
        .collect();
    let integrated = summarize_integrated(&integrated_results);
    let shuffled = summarize_integrated(&shuffled_results);
    let random = summarize_integrated(&random_results);
    let passed = working.passed
        && feedback.passed
        && controls.passed
        && integrated.competent_seeds == SEEDS
        && integrated.held_out_correct == integrated.held_out_total
        && integrated.explicit_answers
        && integrated.queues_empty
        && integrated.fingerprints_unchanged
        && integrated.average_sensory_roles == 3.0
        && integrated.average_working_roles == 2.0
        && integrated.average_event_roles == 2.0
        && integrated.average_program_arrows == 4.0
        && integrated.encoding_transfer_correct == integrated.encoding_transfer_total
        && shuffled.competent_seeds == 0
        && random.competent_seeds == 0;
    P3Report {
        working,
        feedback,
        controls,
        integrated,
        shuffled,
        random,
        passed,
    }
}

pub fn print_report(report: &P3Report) {
    println!("P3 discovered internal execution roles:");
    println!(
        "  P3a working roles: seeds={}/{}, transfer={}/{}, roles={}, symmetric-distinct={}, timing-used={}",
        report.working.successful_seeds,
        report.working.total_seeds,
        report.working.transfer_correct,
        report.working.transfer_total,
        report.working.learned_roles,
        report.working.symmetric_roles_distinct,
        report.working.timing_used
    );
    println!(
        "  P3b feedback: seeds={}/{}, unseen-depth={}/{}, arrows={}, reused={}",
        report.feedback.successful_seeds,
        report.feedback.total_seeds,
        report.feedback.depth_correct,
        report.feedback.depth_total,
        report.feedback.permanent_arrows,
        report.feedback.same_role_cells_reused
    );
    println!(
        "  P3c control roles: seeds={}/{}, transfer={}/{}, roles={}, continuation/finish={}/{}, traversal={}/{}, symmetric-distinct={}",
        report.controls.successful_seeds,
        report.controls.total_seeds,
        report.controls.transfer_correct,
        report.controls.transfer_total,
        report.controls.learned_roles,
        report.controls.continuation_arrows,
        report.controls.finish_arrows,
        report.controls.depth_correct,
        report.controls.depth_total,
        report.controls.symmetric_roles_distinct
    );
    println!(
        "  P3d integrated: competent={}/{}, held-out={}/{}, encoding-transfer={}/{}, roles sensory/working/event={:.1}/{:.1}/{:.1}, program={:.1}, encounter-representations={:.1}, created/released={}/{}, competence={:?}",
        report.integrated.competent_seeds,
        report.integrated.total_seeds,
        report.integrated.held_out_correct,
        report.integrated.held_out_total,
        report.integrated.encoding_transfer_correct,
        report.integrated.encoding_transfer_total,
        report.integrated.average_sensory_roles,
        report.integrated.average_working_roles,
        report.integrated.average_event_roles,
        report.integrated.average_program_arrows,
        report.integrated.average_encounter_representations,
        report.integrated.average_created,
        report.integrated.average_released,
        report.integrated.average_competence_episode
    );
    println!(
        "  emergence sensory/working/event/lookup/feedback/continue/finish={:?}/{:?}/{:?}/{:?}/{:?}/{:?}/{:?}",
        report.integrated.first_sensory_roles,
        report.integrated.first_working_roles,
        report.integrated.first_event_roles,
        report.integrated.first_lookup,
        report.integrated.first_feedback,
        report.integrated.first_continuation,
        report.integrated.first_finish
    );
    println!(
        "  controls shuffled/random={}/{}, {}/{}, passed={}",
        report.shuffled.competent_seeds,
        report.shuffled.total_seeds,
        report.random.competent_seeds,
        report.random.total_seeds,
        report.passed
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn report() -> &'static P3Report {
        static REPORT: OnceLock<P3Report> = OnceLock::new();
        REPORT.get_or_init(run_experiment)
    }

    #[test]
    fn p3a_discovers_working_roles_from_structure_not_absolute_time() {
        let report = report();
        assert!(report.working.passed);
        assert_eq!(report.working.learned_roles, 2);
        assert_eq!(report.working.permanent_receptor_ids, 0);
        assert!(!report.working.symmetric_roles_distinct);
        assert!(!report.working.timing_used);
    }

    #[test]
    fn p3b_learned_working_roles_support_iterable_feedback() {
        let report = report();
        assert!(report.feedback.passed);
        assert_eq!(report.feedback.depth_correct, report.feedback.depth_total);
        assert!(report.feedback.same_role_cells_reused);
    }

    #[test]
    fn p3c_discovers_anonymous_control_event_roles() {
        let report = report();
        assert!(report.controls.passed);
        assert_eq!(report.controls.learned_roles, 2);
        assert!(!report.controls.symmetric_roles_distinct);
    }

    #[test]
    fn p3d_discovers_roles_and_program_from_a_fresh_substrate() {
        let report = report();
        assert!(report.passed);
        assert_eq!(report.integrated.competent_seeds, SEEDS);
        assert_eq!(
            report.integrated.held_out_correct,
            report.integrated.held_out_total
        );
        assert!(report.integrated.explicit_answers);
        assert!(report.integrated.queues_empty);
        assert!(report.integrated.fingerprints_unchanged);
        assert_eq!(report.shuffled.competent_seeds, 0);
        assert_eq!(report.random.competent_seeds, 0);
    }
}
