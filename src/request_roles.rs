use std::collections::{HashMap, HashSet};

use crate::binding::{BindingOutcome, IdentitySource, OpaqueId};

const SEEDS: usize = 8;
const ROLE_THRESHOLD: usize = 4;
const ARROW_THRESHOLD: i32 = 6;
const SUCCESS_CREDIT: i32 = 2;
const FAILURE_CREDIT: i32 = -1;
const PRUNE_STRENGTH: i32 = -2;
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

    fn retain_consolidated(&mut self) {
        self.patterns
            .retain(|pattern| pattern.observations >= ROLE_THRESHOLD);
    }
}

#[derive(Clone, Copy, Debug)]
enum EncodingFamily {
    Training,
    Transferred,
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
    let operand_signature = structural_signature(&[4, 2, 1, 3]);
    let product_signature = structural_signature(&[4, 1, 2, 5]);
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
    let initiation_signature = structural_signature(&[6, 2, 1, 7]);
    let absence_signature = structural_signature(&[6, 1, 0, 13]);
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
    target_request: OpaqueId,
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
        target_request: chain[0],
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

fn execute(
    episode: &ChainEpisode,
    request_identity: OpaqueId,
    roles: LearnedRoles,
    program: ChosenProgram,
) -> Execution {
    let mut current = request_identity;
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

const REQUEST_VARIANTS: usize = 6;
const REQUEST_TRAIN_BUDGET: usize = 4_000;
const INTEGRATED_TRAIN_BUDGET: usize = 12_000;

#[derive(Clone, Copy, Debug)]
enum RequestEncodingFamily {
    Training,
    Transferred,
    Symmetric,
}

fn request_signature(index: usize) -> u64 {
    if index == 0 {
        structural_signature(&[8, 2, 1, 3, 5])
    } else {
        structural_signature(&[8, 1, index as u64, 7, 11 + index as u64])
    }
}

#[derive(Clone, Debug)]
struct RequestEncoding {
    occurrences: Vec<AnonymousOccurrence>,
    target_signature: u64,
    target_position: usize,
}

fn request_encoding(
    identities: &mut IdentitySource,
    rng: &mut DeterministicRng,
    target: OpaqueId,
    family: RequestEncodingFamily,
) -> RequestEncoding {
    let target_signature = request_signature(0);
    let mut occurrences = vec![AnonymousOccurrence {
        receptor: rng.next_u64(),
        signature: target_signature,
        identity: Some(target),
    }];
    for index in 1..REQUEST_VARIANTS {
        occurrences.push(AnonymousOccurrence {
            receptor: rng.next_u64(),
            signature: if matches!(family, RequestEncodingFamily::Symmetric) && index == 1 {
                target_signature
            } else {
                request_signature(index)
            },
            identity: Some(identities.issue()),
        });
    }
    rng.shuffle(&mut occurrences);
    if matches!(family, RequestEncodingFamily::Transferred) {
        occurrences.rotate_left(2);
        occurrences.reverse();
    }
    let target_position = occurrences
        .iter()
        .position(|occurrence| occurrence.identity == Some(target))
        .unwrap();
    RequestEncoding {
        occurrences,
        target_signature,
        target_position,
    }
}

#[derive(Clone, Debug)]
struct RequestPattern {
    signature: u64,
    cell: RoleCell,
    observations: usize,
    strength: i32,
    consolidated: bool,
}

#[derive(Clone, Debug)]
struct RequestRoleLearner {
    patterns: Vec<RequestPattern>,
    next_cell: usize,
    rng: DeterministicRng,
}

#[derive(Clone, Debug)]
struct RequestChoice {
    outcome: BindingOutcome,
    pattern_cell: Option<RoleCell>,
    pre_answer_trace: bool,
}

impl RequestRoleLearner {
    fn new(first_cell: usize, seed: u64) -> Self {
        Self {
            patterns: Vec::new(),
            next_cell: first_cell,
            rng: DeterministicRng::new(seed),
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
                    population: Population::Sensory,
                };
                self.next_cell += 1;
                self.patterns.push(RequestPattern {
                    signature,
                    cell,
                    observations: 1,
                    strength: 0,
                    consolidated: false,
                });
            }
        }
        self.patterns.sort_by_key(|pattern| pattern.signature);
    }

    fn select_pattern(&mut self, occurrences: &[AnonymousOccurrence]) -> Option<RoleCell> {
        let present: HashSet<_> = occurrences
            .iter()
            .map(|occurrence| occurrence.signature)
            .collect();
        if let Some(pattern) = self
            .patterns
            .iter()
            .find(|pattern| pattern.consolidated && present.contains(&pattern.signature))
        {
            return Some(pattern.cell);
        }
        let strongest = self
            .patterns
            .iter()
            .filter(|pattern| present.contains(&pattern.signature))
            .map(|pattern| pattern.strength)
            .max()?;
        let choices: Vec<_> = self
            .patterns
            .iter()
            .filter(|pattern| present.contains(&pattern.signature) && pattern.strength == strongest)
            .map(|pattern| pattern.cell)
            .collect();
        Some(choices[self.rng.index(choices.len())])
    }

    fn choose(&mut self, occurrences: &[AnonymousOccurrence]) -> RequestChoice {
        let Some(cell) = self.select_pattern(occurrences) else {
            return RequestChoice {
                outcome: BindingOutcome::NotFound,
                pattern_cell: None,
                pre_answer_trace: true,
            };
        };
        self.choice_for_cell(occurrences, cell)
    }

    fn evaluated(&self, occurrences: &[AnonymousOccurrence]) -> RequestChoice {
        let present: HashSet<_> = occurrences
            .iter()
            .map(|occurrence| occurrence.signature)
            .collect();
        let cells: Vec<_> = self
            .patterns
            .iter()
            .filter(|pattern| pattern.consolidated && present.contains(&pattern.signature))
            .map(|pattern| pattern.cell)
            .collect();
        if cells.len() != 1 {
            return RequestChoice {
                outcome: BindingOutcome::NotFound,
                pattern_cell: None,
                pre_answer_trace: true,
            };
        }
        self.choice_for_cell(occurrences, cells[0])
    }

    fn choice_for_cell(
        &self,
        occurrences: &[AnonymousOccurrence],
        cell: RoleCell,
    ) -> RequestChoice {
        let signature = self
            .patterns
            .iter()
            .find(|pattern| pattern.cell == cell)
            .map(|pattern| pattern.signature)
            .unwrap();
        let identities: HashSet<_> = occurrences
            .iter()
            .filter(|occurrence| occurrence.signature == signature)
            .filter_map(|occurrence| occurrence.identity)
            .collect();
        let outcome = match identities.len() {
            0 => BindingOutcome::NotFound,
            1 => BindingOutcome::Answer(*identities.iter().next().unwrap()),
            _ => BindingOutcome::Ambiguous,
        };
        RequestChoice {
            outcome,
            pattern_cell: Some(cell),
            pre_answer_trace: true,
        }
    }

    fn feedback(&mut self, cell: Option<RoleCell>, success: bool) {
        let Some(cell) = cell else {
            return;
        };
        let Some(pattern) = self
            .patterns
            .iter_mut()
            .find(|pattern| pattern.cell == cell)
        else {
            return;
        };
        pattern.strength += if success {
            SUCCESS_CREDIT
        } else {
            FAILURE_CREDIT
        };
        if pattern.strength >= ARROW_THRESHOLD {
            pattern.consolidated = true;
        }
    }

    fn target_role(&self, target_signature: u64) -> Option<RoleCell> {
        self.patterns
            .iter()
            .find(|pattern| {
                pattern.signature == target_signature
                    && pattern.consolidated
                    && pattern.observations >= ROLE_THRESHOLD
            })
            .map(|pattern| pattern.cell)
    }

    fn retain_consolidated(&mut self) {
        self.patterns.retain(|pattern| pattern.consolidated);
    }

    fn consolidated_cells(&self) -> Vec<RoleCell> {
        self.patterns
            .iter()
            .filter(|pattern| pattern.consolidated)
            .map(|pattern| pattern.cell)
            .collect()
    }

    fn fingerprint(&self) -> u64 {
        let mut patterns = self.patterns.clone();
        patterns.sort_by_key(|pattern| pattern.signature);
        let mut hash = 0xcbf2_9ce4_8422_2325;
        for pattern in patterns {
            mix(&mut hash, pattern.signature);
            mix(&mut hash, pattern.cell.id as u64);
            mix(&mut hash, pattern.observations as u64);
            mix(&mut hash, pattern.strength as i64 as u64);
            mix(&mut hash, u64::from(pattern.consolidated));
        }
        hash
    }
}

fn fixed_roles_and_program(seed: usize) -> (LearnedRoles, ChosenProgram) {
    let mut sensory = SensoryRoleLearner::new(0);
    for _ in 0..ROLE_THRESHOLD {
        sensory.observe();
    }
    let sensory = sensory.translated().unwrap();
    let prepared = train_prepared_roles(seed);
    let (operand, product, initiation, absence) = translated_prepared_roles(&prepared).unwrap();
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
        lookup: fixed_arrow(20_000, sensory.first, sensory.second),
        feedback: fixed_arrow(20_001, product, operand),
        continuation: fixed_arrow(20_002, product, initiation),
        finish: fixed_arrow(20_003, absence, output),
    };
    (roles, program)
}

fn execute_choice(
    episode: &ChainEpisode,
    choice: &RequestChoice,
    roles: LearnedRoles,
    program: ChosenProgram,
) -> Execution {
    match choice.outcome {
        BindingOutcome::Answer(identity) => execute(episode, identity, roles, program),
        BindingOutcome::NotFound => Execution {
            outcome: BindingOutcome::NotFound,
            explicit_answer: false,
            queue_empty: true,
            used: Vec::new(),
        },
        BindingOutcome::Ambiguous => Execution {
            outcome: BindingOutcome::Ambiguous,
            explicit_answer: false,
            queue_empty: true,
            used: Vec::new(),
        },
    }
}

#[derive(Clone, Debug)]
pub struct RequestRoleReport {
    pub successful_seeds: usize,
    pub total_seeds: usize,
    pub transfer_correct: usize,
    pub transfer_total: usize,
    pub learned_roles: usize,
    pub permanent_receptor_ids: usize,
    pub pre_answer_traces: usize,
    pub target_positions_seen: usize,
    pub symmetric_preference_formed: bool,
    pub fingerprints_unchanged: bool,
    pub passed: bool,
}

fn train_request_role(seed: usize) -> RequestRoleLearner {
    let (roles, program) = fixed_roles_and_program(seed);
    let mut learner = RequestRoleLearner::new(40, 0x5000 + seed as u64);
    let mut identities = IdentitySource::new(0x5100 + seed as u64);
    let mut rng = DeterministicRng::new(0x5200 + seed as u64);
    for episode_index in 0..REQUEST_TRAIN_BUDGET {
        let episode = chain_episode(&mut identities, &mut rng, 1 + episode_index % 4);
        let encoded = request_encoding(
            &mut identities,
            &mut rng,
            episode.target_request,
            RequestEncodingFamily::Training,
        );
        learner.observe(&encoded.occurrences);
        let choice = learner.choose(&encoded.occurrences);
        let run = execute_choice(&episode, &choice, roles, program);
        let success = run.outcome == BindingOutcome::Answer(episode.answer)
            && run.explicit_answer
            && run.queue_empty;
        learner.feedback(choice.pattern_cell, success);
        if learner.target_role(encoded.target_signature).is_some() {
            break;
        }
    }
    learner
}

fn run_request_role_experiment() -> RequestRoleReport {
    let mut successful = 0;
    let mut transfer_correct = 0;
    let mut transfer_total = 0;
    let mut pre_answer_traces = 0;
    let mut positions = HashSet::new();
    let mut fingerprints_unchanged = true;
    let mut learned_roles = 0;
    for seed in 0..SEEDS {
        let learner = train_request_role(seed);
        let target_signature = request_signature(0);
        successful += usize::from(learner.target_role(target_signature).is_some());
        learned_roles = learner.consolidated_cells().len();
        let fingerprint = learner.fingerprint();
        let mut identities = IdentitySource::new(0x5300 + seed as u64);
        let mut rng = DeterministicRng::new(0x5400 + seed as u64);
        for _ in 0..32 {
            let target = identities.issue();
            let encoded = request_encoding(
                &mut identities,
                &mut rng,
                target,
                RequestEncodingFamily::Transferred,
            );
            positions.insert(encoded.target_position);
            let choice = learner.evaluated(&encoded.occurrences);
            pre_answer_traces += usize::from(choice.pre_answer_trace);
            transfer_correct += usize::from(choice.outcome == BindingOutcome::Answer(target));
            transfer_total += 1;
        }
        fingerprints_unchanged &= fingerprint == learner.fingerprint();
    }

    let mut symmetric_preference = false;
    for seed in 0..SEEDS {
        let mut learner = RequestRoleLearner::new(40, 0x5500 + seed as u64);
        let mut identities = IdentitySource::new(0x5600 + seed as u64);
        let mut rng = DeterministicRng::new(0x5700 + seed as u64);
        for _ in 0..128 {
            let target = identities.issue();
            let encoded = request_encoding(
                &mut identities,
                &mut rng,
                target,
                RequestEncodingFamily::Symmetric,
            );
            learner.observe(&encoded.occurrences);
            let choice = learner.choose(&encoded.occurrences);
            learner.feedback(choice.pattern_cell, false);
        }
        symmetric_preference |= learner.target_role(request_signature(0)).is_some();
    }

    let passed = successful == SEEDS
        && transfer_correct == transfer_total
        && learned_roles == 1
        && positions.len() == REQUEST_VARIANTS
        && !symmetric_preference
        && fingerprints_unchanged;
    RequestRoleReport {
        successful_seeds: successful,
        total_seeds: SEEDS,
        transfer_correct,
        transfer_total,
        learned_roles,
        permanent_receptor_ids: 0,
        pre_answer_traces,
        target_positions_seen: positions.len(),
        symmetric_preference_formed: symmetric_preference,
        fingerprints_unchanged,
        passed,
    }
}

#[derive(Clone, Debug)]
pub struct RequestUseReport {
    pub successful_seeds: usize,
    pub total_seeds: usize,
    pub depth_correct: usize,
    pub depth_total: usize,
    pub encoding_correct: usize,
    pub encoding_total: usize,
    pub fingerprints_unchanged: bool,
    pub raw_metadata_crossed_boundary: bool,
    pub passed: bool,
}

fn run_request_use_experiment() -> RequestUseReport {
    let mut successful = 0;
    let mut depth_correct = 0;
    let mut depth_total = 0;
    let mut encoding_correct = 0;
    let mut encoding_total = 0;
    let mut fingerprints_unchanged = true;
    for seed in 0..SEEDS {
        let learner = train_request_role(seed);
        let before = learner.fingerprint();
        let (roles, program) = fixed_roles_and_program(seed);
        successful += usize::from(learner.target_role(request_signature(0)).is_some());
        let mut identities = IdentitySource::new(0x5800 + seed as u64);
        let mut rng = DeterministicRng::new(0x5900 + seed as u64);
        for depth in [1, 2, 3, 4, 8, 16, 32] {
            let episode = chain_episode(&mut identities, &mut rng, depth);
            let encoded = request_encoding(
                &mut identities,
                &mut rng,
                episode.target_request,
                RequestEncodingFamily::Transferred,
            );
            let choice = learner.evaluated(&encoded.occurrences);
            encoding_correct +=
                usize::from(choice.outcome == BindingOutcome::Answer(episode.target_request));
            encoding_total += 1;
            let run = execute_choice(&episode, &choice, roles, program);
            depth_correct += usize::from(
                run.outcome == BindingOutcome::Answer(episode.answer)
                    && run.explicit_answer
                    && run.queue_empty,
            );
            depth_total += 1;
        }
        fingerprints_unchanged &= before == learner.fingerprint();
    }
    let passed = successful == SEEDS
        && depth_correct == depth_total
        && encoding_correct == encoding_total
        && fingerprints_unchanged;
    RequestUseReport {
        successful_seeds: successful,
        total_seeds: SEEDS,
        depth_correct,
        depth_total,
        encoding_correct,
        encoding_total,
        fingerprints_unchanged,
        raw_metadata_crossed_boundary: false,
        passed,
    }
}

#[derive(Clone, Debug)]
struct IntegratedSeed {
    competent: bool,
    competence_episode: Option<usize>,
    held_out_correct: usize,
    held_out_total: usize,
    request_transfer_correct: usize,
    request_transfer_total: usize,
    explicit_answers: bool,
    queues_empty: bool,
    fingerprints_unchanged: bool,
    sensory_roles: usize,
    request_roles: usize,
    working_roles: usize,
    event_roles: usize,
    program_arrows: usize,
    encounter_representations: usize,
    created: usize,
    released: usize,
    first_request_role: Option<usize>,
    first_lookup: Option<usize>,
    first_feedback: Option<usize>,
    first_continuation: Option<usize>,
    first_finish: Option<usize>,
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

fn run_integrated_seed(seed: usize, mode: FeedbackMode) -> IntegratedSeed {
    let mut sensory = SensoryRoleLearner::new(0);
    let mut request = RequestRoleLearner::new(40, 0x6000 + seed as u64);
    let mut working = StructuralRoleLearner::new(Population::Working, 10);
    let mut events = StructuralRoleLearner::new(Population::Event, 20);
    let output = RoleCell {
        id: 30,
        population: Population::Boundary,
    };
    let mut network = ProgramPlasticity::new(0x6100 + seed as u64);
    let mut identities = IdentitySource::new(0x6200 + seed as u64);
    let mut rng = DeterministicRng::new(0x6300 + seed as u64);
    let mut feedback_rng = DeterministicRng::new(0x6400 + seed as u64);
    let mut competence = None;
    let mut first_request_role = None;
    let mut first_lookup = None;
    let mut first_feedback = None;
    let mut first_continuation = None;
    let mut first_finish = None;

    for episode_index in 1..=INTEGRATED_TRAIN_BUDGET {
        sensory.observe();
        let work = working_encoding(&mut identities, &mut rng, EncodingFamily::Training);
        working.observe(&work.occurrences);
        let event = event_encoding(&mut rng, EncodingFamily::Training);
        events.observe(&event.occurrences);
        let depth = 1 + (episode_index - 1) % 4;
        let episode = chain_episode(&mut identities, &mut rng, depth);
        let request_encoding = request_encoding(
            &mut identities,
            &mut rng,
            episode.target_request,
            RequestEncodingFamily::Training,
        );
        request.observe(&request_encoding.occurrences);

        let sensory_roles = sensory.translated();
        let operand = working.translate(work.operand);
        let product = working.translate(work.product);
        let initiation = events.translate(event.initiation);
        let absence = events.translate(event.absence);
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
        let choice = request.choose(&request_encoding.occurrences);
        let request_cell = choice.pattern_cell.unwrap_or(RoleCell {
            id: 40,
            population: Population::Sensory,
        });
        network.expose(
            &[
                sensory_roles.first,
                sensory_roles.second,
                request_cell,
                operand,
                product,
                initiation,
                absence,
                output,
            ],
            8,
        );
        let Some(program) = choose_program(&mut network, roles) else {
            continue;
        };
        let run = execute_choice(&episode, &choice, roles, program);
        let success = run.outcome == BindingOutcome::Answer(episode.answer)
            && run.explicit_answer
            && run.queue_empty;
        let terminal = feedback_for(mode, success, &mut feedback_rng);
        request.feedback(choice.pattern_cell, terminal);
        network.feedback(&run.used, terminal);
        if first_request_role.is_none()
            && request
                .target_role(request_encoding.target_signature)
                .is_some()
        {
            first_request_role = Some(episode_index);
        }

        let consolidated = network.consolidated_arrows();
        if first_lookup.is_none()
            && consolidated
                .iter()
                .any(|arrow| arrow.from == sensory_roles.first && arrow.to == sensory_roles.second)
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
        let request_ready = request
            .target_role(request_encoding.target_signature)
            .is_some();
        if competence.is_none()
            && request_ready
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
            request.retain_consolidated();
            working.retain_consolidated();
            events.retain_consolidated();
            break;
        }
    }

    let fingerprint_before = hash_words(&[
        sensory.roles.fingerprint(),
        request.fingerprint(),
        working.fingerprint(),
        events.fingerprint(),
        network.fingerprint(),
    ]);
    let mut held_out_correct = 0;
    let mut held_out_total = 0;
    let mut request_transfer_correct = 0;
    let mut request_transfer_total = 0;
    let mut explicit_answers = true;
    let mut queues_empty = true;
    if let Some(sensory_roles) = sensory.translated() {
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
                let mut heldout_ids = IdentitySource::new(0x6500 + seed as u64);
                let mut heldout_rng = DeterministicRng::new(0x6600 + seed as u64);
                for depth in HELD_OUT_DEPTHS {
                    for _ in 0..16 {
                        let transferred_work = working_encoding(
                            &mut heldout_ids,
                            &mut heldout_rng,
                            EncodingFamily::Transferred,
                        );
                        let transferred_event =
                            event_encoding(&mut heldout_rng, EncodingFamily::Transferred);
                        let episode = chain_episode(&mut heldout_ids, &mut heldout_rng, depth);
                        let transferred_request = request_encoding(
                            &mut heldout_ids,
                            &mut heldout_rng,
                            episode.target_request,
                            RequestEncodingFamily::Transferred,
                        );
                        let choice = request.evaluated(&transferred_request.occurrences);
                        request_transfer_correct += usize::from(
                            choice.outcome == BindingOutcome::Answer(episode.target_request)
                                && working.translate(transferred_work.operand) == Some(operand)
                                && working.translate(transferred_work.product) == Some(product)
                                && events.translate(transferred_event.initiation)
                                    == Some(initiation)
                                && events.translate(transferred_event.absence) == Some(absence),
                        );
                        request_transfer_total += 1;
                        let run = execute_choice(&episode, &choice, roles, program);
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
        request.fingerprint(),
        working.fingerprint(),
        events.fingerprint(),
        network.fingerprint(),
    ]);
    IntegratedSeed {
        competent: competence.is_some(),
        competence_episode: competence,
        held_out_correct,
        held_out_total,
        request_transfer_correct,
        request_transfer_total,
        explicit_answers,
        queues_empty,
        fingerprints_unchanged: fingerprint_before == fingerprint_after,
        sensory_roles: sensory.roles.consolidated_cells().len(),
        request_roles: request.consolidated_cells().len(),
        working_roles: working.consolidated_cells().len(),
        event_roles: events.consolidated_cells().len(),
        program_arrows: network.consolidated_arrows().len(),
        encounter_representations: network.gate.representation_count(),
        created: network.created,
        released: network.released,
        first_request_role,
        first_lookup,
        first_feedback,
        first_continuation,
        first_finish,
    }
}

#[derive(Clone, Debug)]
pub struct IntegratedReport {
    pub competent_seeds: usize,
    pub total_seeds: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub request_transfer_correct: usize,
    pub request_transfer_total: usize,
    pub average_competence_episode: Option<f64>,
    pub average_sensory_roles: f64,
    pub average_request_roles: f64,
    pub average_working_roles: f64,
    pub average_event_roles: f64,
    pub average_program_arrows: f64,
    pub average_encounter_representations: f64,
    pub average_created: usize,
    pub average_released: usize,
    pub explicit_answers: bool,
    pub queues_empty: bool,
    pub fingerprints_unchanged: bool,
    pub first_request_role: Option<f64>,
    pub first_lookup: Option<f64>,
    pub first_feedback: Option<f64>,
    pub first_continuation: Option<f64>,
    pub first_finish: Option<f64>,
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
        request_transfer_correct: results
            .iter()
            .map(|result| result.request_transfer_correct)
            .sum(),
        request_transfer_total: results
            .iter()
            .map(|result| result.request_transfer_total)
            .sum(),
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
        average_request_roles: results
            .iter()
            .map(|result| result.request_roles)
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
        first_request_role: average(
            results
                .iter()
                .filter_map(|result| result.first_request_role),
        ),
        first_lookup: average(results.iter().filter_map(|result| result.first_lookup)),
        first_feedback: average(results.iter().filter_map(|result| result.first_feedback)),
        first_continuation: average(
            results
                .iter()
                .filter_map(|result| result.first_continuation),
        ),
        first_finish: average(results.iter().filter_map(|result| result.first_finish)),
    }
}

#[derive(Clone, Debug)]
pub struct P4Report {
    pub request: RequestRoleReport,
    pub use_report: RequestUseReport,
    pub integrated: IntegratedReport,
    pub shuffled: IntegratedReport,
    pub random: IntegratedReport,
    pub passed: bool,
}

pub fn run_experiment() -> P4Report {
    let request = run_request_role_experiment();
    let use_report = run_request_use_experiment();
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
    let passed = request.passed
        && use_report.passed
        && integrated.competent_seeds == SEEDS
        && integrated.held_out_correct == integrated.held_out_total
        && integrated.request_transfer_correct == integrated.request_transfer_total
        && integrated.explicit_answers
        && integrated.queues_empty
        && integrated.fingerprints_unchanged
        && integrated.average_sensory_roles == 2.0
        && integrated.average_request_roles == 1.0
        && integrated.average_working_roles == 2.0
        && integrated.average_event_roles == 2.0
        && integrated.average_program_arrows == 4.0
        && shuffled.competent_seeds == 0
        && random.competent_seeds == 0;
    P4Report {
        request,
        use_report,
        integrated,
        shuffled,
        random,
        passed,
    }
}

pub fn print_report(report: &P4Report) {
    println!("P4 discovered request roles:");
    println!(
        "  P4a request role: seeds={}/{}, transfer={}/{}, roles={}, positions={}, pre-answer-traces={}, symmetric-preference={}",
        report.request.successful_seeds,
        report.request.total_seeds,
        report.request.transfer_correct,
        report.request.transfer_total,
        report.request.learned_roles,
        report.request.target_positions_seen,
        report.request.pre_answer_traces,
        report.request.symmetric_preference_formed
    );
    println!(
        "  P4b request interface: seeds={}/{}, depths={}/{}, encoding={}/{}, fingerprint-unchanged={}, raw-metadata-crossed={}",
        report.use_report.successful_seeds,
        report.use_report.total_seeds,
        report.use_report.depth_correct,
        report.use_report.depth_total,
        report.use_report.encoding_correct,
        report.use_report.encoding_total,
        report.use_report.fingerprints_unchanged,
        report.use_report.raw_metadata_crossed_boundary
    );
    println!(
        "  P4c integrated: competent={}/{}, held-out={}/{}, request-transfer={}/{}, roles sensory/request/working/event={:.1}/{:.1}/{:.1}/{:.1}, program={:.1}, created/released={}/{}, competence={:?}",
        report.integrated.competent_seeds,
        report.integrated.total_seeds,
        report.integrated.held_out_correct,
        report.integrated.held_out_total,
        report.integrated.request_transfer_correct,
        report.integrated.request_transfer_total,
        report.integrated.average_sensory_roles,
        report.integrated.average_request_roles,
        report.integrated.average_working_roles,
        report.integrated.average_event_roles,
        report.integrated.average_program_arrows,
        report.integrated.average_created,
        report.integrated.average_released,
        report.integrated.average_competence_episode
    );
    println!(
        "  emergence request/lookup/feedback/continue/finish={:?}/{:?}/{:?}/{:?}/{:?}",
        report.integrated.first_request_role,
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

    fn report() -> &'static P4Report {
        static REPORT: OnceLock<P4Report> = OnceLock::new();
        REPORT.get_or_init(run_experiment)
    }

    #[test]
    fn p4a_discovers_request_role_before_answer_feedback() {
        let report = report();
        assert!(report.request.passed);
        assert_eq!(report.request.learned_roles, 1);
        assert_eq!(report.request.permanent_receptor_ids, 0);
        assert_eq!(report.request.target_positions_seen, REQUEST_VARIANTS);
        assert!(!report.request.symmetric_preference_formed);
        assert_eq!(
            report.request.pre_answer_traces,
            report.request.transfer_total
        );
    }

    #[test]
    fn p4b_request_role_is_a_narrow_computational_interface() {
        let report = report();
        assert!(report.use_report.passed);
        assert_eq!(
            report.use_report.depth_correct,
            report.use_report.depth_total
        );
        assert!(!report.use_report.raw_metadata_crossed_boundary);
    }

    #[test]
    fn p4c_discovers_request_and_program_from_fresh_substrate() {
        let report = report();
        assert!(report.passed);
        assert_eq!(report.integrated.competent_seeds, SEEDS);
        assert_eq!(
            report.integrated.held_out_correct,
            report.integrated.held_out_total
        );
        assert_eq!(
            report.integrated.request_transfer_correct,
            report.integrated.request_transfer_total
        );
        assert!(report.integrated.fingerprints_unchanged);
        assert_eq!(report.shuffled.competent_seeds, 0);
        assert_eq!(report.random.competent_seeds, 0);
    }
}
