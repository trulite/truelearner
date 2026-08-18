use std::collections::{HashMap, HashSet};

type CellId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpaqueId {
    first: u64,
    second: u64,
}

#[derive(Clone, Debug)]
pub(crate) struct IdentitySource {
    next: u64,
    seed: u64,
}

impl IdentitySource {
    pub(crate) fn new(seed: u64) -> Self {
        Self { next: 0, seed }
    }

    pub(crate) fn issue(&mut self) -> OpaqueId {
        let value = self.next;
        self.next += 1;
        OpaqueId {
            first: mix64(self.seed),
            second: mix64(value),
        }
    }
}

fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
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

    fn shuffle<T>(&mut self, values: &mut [T]) {
        for index in (1..values.len()).rev() {
            let selected = (self.next_u64() as usize) % (index + 1);
            values.swap(index, selected);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum SlotRole {
    Slot1,
    Slot2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PermanentCellKind {
    Query,
    Same,
    Relation,
    Slot1,
    Slot2,
    Answer,
}

#[derive(Clone, Debug)]
struct PermanentCell {
    kind: PermanentCellKind,
    threshold: u32,
}

#[derive(Clone, Debug)]
struct PermanentArrow {
    match_role: SlotRole,
    output_role: SlotRole,
    strength: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TempCellKind {
    Relation,
    Identity {
        identity: OpaqueId,
        role: SlotRole,
        relation: usize,
    },
    Query {
        identity: OpaqueId,
    },
}

#[derive(Clone, Debug)]
struct TempCell {
    kind: TempCellKind,
}

#[derive(Clone, Copy, Debug)]
struct TempArrow {
    from: CellId,
    to: CellId,
    role: SlotRole,
}

#[derive(Clone, Copy, Debug)]
struct TempRelation {
    relation_cell: CellId,
    slot1_cell: CellId,
    slot2_cell: CellId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindingOutcome {
    Answer(OpaqueId),
    NotFound,
    Ambiguous,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct EpisodeMetrics {
    pub(crate) temporary_cells: usize,
    pub(crate) temporary_arrows: usize,
    pub(crate) spikes: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct BindingLearner {
    permanent_cells: Vec<PermanentCell>,
    permanent_arrows: Vec<PermanentArrow>,
    training_examples: u64,
    temporary_cells: Vec<TempCell>,
    temporary_arrows: Vec<TempArrow>,
    temporary_relations: Vec<TempRelation>,
    query_cell: Option<CellId>,
}

impl BindingLearner {
    pub(crate) fn new() -> Self {
        Self {
            permanent_cells: Vec::new(),
            permanent_arrows: Vec::new(),
            training_examples: 0,
            temporary_cells: Vec::new(),
            temporary_arrows: Vec::new(),
            temporary_relations: Vec::new(),
            query_cell: None,
        }
    }

    fn ensure_permanent_structure(&mut self) {
        if !self.permanent_cells.is_empty() {
            return;
        }
        self.permanent_cells = [
            PermanentCellKind::Query,
            PermanentCellKind::Same,
            PermanentCellKind::Relation,
            PermanentCellKind::Slot1,
            PermanentCellKind::Slot2,
            PermanentCellKind::Answer,
        ]
        .into_iter()
        .map(|kind| PermanentCell { kind, threshold: 1 })
        .collect();
        for match_role in [SlotRole::Slot1, SlotRole::Slot2] {
            for output_role in [SlotRole::Slot1, SlotRole::Slot2] {
                self.permanent_arrows.push(PermanentArrow {
                    match_role,
                    output_role,
                    strength: 0,
                });
            }
        }
    }

    pub(crate) fn begin_episode(&mut self) {
        self.erase_temporary();
    }

    /// The parser exposes that two opaque identities occupied two different
    /// slots in one relation. Both structural arrows point from the relation
    /// cell to its slot occurrence; neither is an answer route.
    pub(crate) fn observe_relation(&mut self, slot1: OpaqueId, slot2: OpaqueId) {
        let relation_index = self.temporary_relations.len();
        let relation_cell = self.push_temp(TempCellKind::Relation);
        let slot1_cell = self.push_temp(TempCellKind::Identity {
            identity: slot1,
            role: SlotRole::Slot1,
            relation: relation_index,
        });
        let slot2_cell = self.push_temp(TempCellKind::Identity {
            identity: slot2,
            role: SlotRole::Slot2,
            relation: relation_index,
        });
        self.temporary_arrows.push(TempArrow {
            from: relation_cell,
            to: slot1_cell,
            role: SlotRole::Slot1,
        });
        self.temporary_arrows.push(TempArrow {
            from: relation_cell,
            to: slot2_cell,
            role: SlotRole::Slot2,
        });
        self.temporary_relations.push(TempRelation {
            relation_cell,
            slot1_cell,
            slot2_cell,
        });
    }

    fn observe_query(&mut self, identity: OpaqueId) {
        assert!(
            self.query_cell.is_none(),
            "one query is permitted per episode"
        );
        self.query_cell = Some(self.push_temp(TempCellKind::Query { identity }));
    }

    fn answer(&self) -> (BindingOutcome, EpisodeMetrics) {
        let Some((_, route)) = self.selected_route() else {
            return (
                BindingOutcome::NotFound,
                self.episode_metrics(self.temporary_identity_count() + 1),
            );
        };
        self.execute_route(route.match_role, route.output_role)
    }

    /// Applies only terminal supervision. Candidate routes receive credit or
    /// blame according to the complete output identity they produced.
    fn learn_from_terminal(&mut self, correct: BindingOutcome) {
        self.ensure_permanent_structure();
        let candidates: Vec<_> = self
            .permanent_arrows
            .iter()
            .map(|route| {
                let (outcome, _) = self.execute_route(route.match_role, route.output_role);
                outcome
            })
            .collect();
        for (arrow, outcome) in self.permanent_arrows.iter_mut().zip(candidates) {
            if outcome == correct {
                arrow.strength = arrow.strength.saturating_add(1);
            } else {
                arrow.strength = arrow.strength.saturating_sub(1);
            }
        }
        self.training_examples += 1;
    }

    fn selected_route(&self) -> Option<(usize, &PermanentArrow)> {
        let mut best = None;
        let mut tied = false;
        for (index, arrow) in self.permanent_arrows.iter().enumerate() {
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
        }
    }

    pub(crate) fn lookup_identity(
        &self,
        identity: OpaqueId,
    ) -> (BindingOutcome, EpisodeMetrics, Option<usize>) {
        let Some((arrow_id, route)) = self.selected_route() else {
            return (
                BindingOutcome::NotFound,
                self.episode_metrics(self.temporary_identity_count() + 1),
                None,
            );
        };
        let (outcome, metrics) =
            self.execute_identity(identity, route.match_role, route.output_role);
        (outcome, metrics, Some(arrow_id))
    }

    pub(crate) fn selected_route_id(&self) -> Option<usize> {
        self.selected_route().map(|(arrow_id, _)| arrow_id)
    }

    pub(crate) fn temporary_identity_cells(&self) -> Vec<usize> {
        self.temporary_cells
            .iter()
            .enumerate()
            .filter_map(|(cell_id, cell)| {
                matches!(cell.kind, TempCellKind::Identity { .. }).then_some(cell_id)
            })
            .collect()
    }

    /// Performs the local work of one temporary identity occurrence. The
    /// caller is responsible for delivering the comparison as a queued spike.
    pub(crate) fn compare_temporary_identity(
        &self,
        query_identity: OpaqueId,
        cell_id: usize,
    ) -> Option<OpaqueId> {
        let (_, route) = self.selected_route()?;
        let TempCellKind::Identity {
            identity,
            role,
            relation,
        } = self.temporary_cells.get(cell_id)?.kind
        else {
            return None;
        };
        if role != route.match_role || identity != query_identity {
            return None;
        }
        let relation = self.temporary_relations[relation];
        let output_cell = match route.output_role {
            SlotRole::Slot1 => relation.slot1_cell,
            SlotRole::Slot2 => relation.slot2_cell,
        };
        let TempCellKind::Identity {
            identity: output, ..
        } = self.temporary_cells[output_cell].kind
        else {
            unreachable!("relation slot has identity kind")
        };
        Some(output)
    }

    fn execute_route(
        &self,
        match_role: SlotRole,
        output_role: SlotRole,
    ) -> (BindingOutcome, EpisodeMetrics) {
        let Some(query_cell) = self.query_cell else {
            return (BindingOutcome::NotFound, self.episode_metrics(0));
        };
        let TempCellKind::Query {
            identity: query_identity,
        } = self.temporary_cells[query_cell].kind
        else {
            unreachable!("query cell has query kind")
        };

        self.execute_identity(query_identity, match_role, output_role)
    }

    fn execute_identity(
        &self,
        query_identity: OpaqueId,
        match_role: SlotRole,
        output_role: SlotRole,
    ) -> (BindingOutcome, EpisodeMetrics) {
        let mut outputs = HashSet::new();
        let mut spikes = 1;
        for cell in &self.temporary_cells {
            let TempCellKind::Identity {
                identity,
                role,
                relation,
            } = cell.kind
            else {
                continue;
            };
            spikes += 1;
            if role != match_role || identity != query_identity {
                continue;
            }
            spikes += 2;
            let relation = self.temporary_relations[relation];
            let relation_cell = self.temporary_cells[relation.relation_cell].kind;
            debug_assert_eq!(relation_cell, TempCellKind::Relation);
            let output_cell = match output_role {
                SlotRole::Slot1 => relation.slot1_cell,
                SlotRole::Slot2 => relation.slot2_cell,
            };
            let TempCellKind::Identity {
                identity: output, ..
            } = self.temporary_cells[output_cell].kind
            else {
                unreachable!("relation slot has identity kind")
            };
            outputs.insert(output);
        }

        let outcome = match outputs.len() {
            0 => BindingOutcome::NotFound,
            1 => BindingOutcome::Answer(*outputs.iter().next().unwrap()),
            _ => BindingOutcome::Ambiguous,
        };
        (outcome, self.episode_metrics(spikes))
    }

    fn episode_metrics(&self, spikes: usize) -> EpisodeMetrics {
        EpisodeMetrics {
            temporary_cells: self.temporary_cells.len(),
            temporary_arrows: self.temporary_arrows.len(),
            spikes,
        }
    }

    fn temporary_identity_count(&self) -> usize {
        self.temporary_cells
            .iter()
            .filter(|cell| matches!(cell.kind, TempCellKind::Identity { .. }))
            .count()
    }

    fn push_temp(&mut self, kind: TempCellKind) -> CellId {
        let id = self.temporary_cells.len();
        self.temporary_cells.push(TempCell { kind });
        id
    }

    pub(crate) fn erase_temporary(&mut self) {
        self.temporary_cells = Vec::new();
        self.temporary_arrows = Vec::new();
        self.temporary_relations = Vec::new();
        self.query_cell = None;
    }

    pub(crate) fn temporary_counts(&self) -> (usize, usize) {
        (self.temporary_cells.len(), self.temporary_arrows.len())
    }

    pub(crate) fn temporary_capacities(&self) -> (usize, usize, usize) {
        (
            self.temporary_cells.capacity(),
            self.temporary_arrows.capacity(),
            self.temporary_relations.capacity(),
        )
    }

    fn permanent_identity_cells(&self) -> usize {
        0
    }

    pub(crate) fn permanent_fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        fingerprint_mix(&mut hash, self.training_examples);
        fingerprint_mix(&mut hash, self.permanent_cells.len() as u64);
        for cell in &self.permanent_cells {
            fingerprint_mix(&mut hash, cell.kind as u64);
            fingerprint_mix(&mut hash, cell.threshold as u64);
        }
        fingerprint_mix(&mut hash, self.permanent_arrows.len() as u64);
        for arrow in &self.permanent_arrows {
            fingerprint_mix(&mut hash, arrow.match_role as u64);
            fingerprint_mix(&mut hash, arrow.output_role as u64);
            fingerprint_mix(&mut hash, arrow.strength as i64 as u64);
        }
        hash
    }

    pub(crate) fn permanent_counts(&self) -> (usize, usize) {
        (self.permanent_cells.len(), self.permanent_arrows.len())
    }

    fn route_strengths(&self) -> Vec<(SlotRole, SlotRole, i32)> {
        self.permanent_arrows
            .iter()
            .map(|arrow| (arrow.match_role, arrow.output_role, arrow.strength))
            .collect()
    }

    fn structural_arrows_are_symmetric(&self) -> bool {
        self.temporary_relations.iter().all(|relation| {
            let arrows: Vec<_> = self
                .temporary_arrows
                .iter()
                .filter(|arrow| arrow.from == relation.relation_cell)
                .collect();
            arrows.len() == 2
                && arrows
                    .iter()
                    .any(|arrow| arrow.to == relation.slot1_cell && arrow.role == SlotRole::Slot1)
                && arrows
                    .iter()
                    .any(|arrow| arrow.to == relation.slot2_cell && arrow.role == SlotRole::Slot2)
        })
    }
}

fn fingerprint_mix(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(0x100_0000_01b3);
    }
}

#[derive(Clone, Debug)]
struct BindingEpisode {
    relations: Vec<(OpaqueId, OpaqueId)>,
    query: OpaqueId,
    correct: BindingOutcome,
    distinct_identities: usize,
}

fn normal_episode(
    identities: &mut IdentitySource,
    rng: &mut DeterministicRng,
    relation_count: usize,
) -> BindingEpisode {
    assert!(relation_count > 0);
    let mut relations = Vec::with_capacity(relation_count);
    for _ in 0..relation_count {
        relations.push((identities.issue(), identities.issue()));
    }
    let target = (rng.next_u64() as usize) % relation_count;
    let query = relations[target].0;
    let answer = relations[target].1;
    rng.shuffle(&mut relations);
    BindingEpisode {
        relations,
        query,
        correct: BindingOutcome::Answer(answer),
        distinct_identities: relation_count * 2,
    }
}

fn present(learner: &mut BindingLearner, episode: &BindingEpisode) {
    learner.begin_episode();
    for &(slot1, slot2) in &episode.relations {
        learner.observe_relation(slot1, slot2);
    }
    learner.observe_query(episode.query);
}

fn evaluate_episode(
    learner: &mut BindingLearner,
    episode: &BindingEpisode,
) -> (bool, EpisodeMetrics) {
    present(learner, episode);
    let (outcome, metrics) = learner.answer();
    let correct = outcome == episode.correct;
    learner.erase_temporary();
    (correct, metrics)
}

pub(crate) fn frozen_lookup_operation() -> BindingLearner {
    let mut learner = BindingLearner::new();
    let mut identities = IdentitySource::new(0x1900_f001);
    let mut rng = DeterministicRng::new(0x1900_f002);
    for _ in 0..32 {
        let episode = normal_episode(&mut identities, &mut rng, 6);
        present(&mut learner, &episode);
        let _proposal = learner.answer();
        learner.learn_from_terminal(episode.correct);
        learner.erase_temporary();
    }
    learner
}

#[derive(Clone, Debug, Default)]
struct PersistentMemorizer {
    bindings: HashMap<OpaqueId, OpaqueId>,
}

impl PersistentMemorizer {
    fn train(&mut self, episode: &BindingEpisode) {
        for &(left, right) in &episode.relations {
            self.bindings.insert(left, right);
        }
    }

    fn answer(&self, query: OpaqueId) -> BindingOutcome {
        self.bindings
            .get(&query)
            .copied()
            .map_or(BindingOutcome::NotFound, BindingOutcome::Answer)
    }
}

#[derive(Clone, Debug)]
pub struct BindingCheckpoint {
    pub training_episodes: usize,
    pub permanent_cells: usize,
    pub permanent_arrows: usize,
    pub validation_correct: usize,
    pub validation_total: usize,
}

#[derive(Clone, Debug)]
pub struct BindingReport {
    pub checkpoints: Vec<BindingCheckpoint>,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub held_out_distinct_identities: usize,
    pub memorizer_entries: usize,
    pub memorizer_held_out_correct: usize,
    pub reverse_is_not_found: bool,
    pub missing_is_not_found: bool,
    pub conflict_is_ambiguous: bool,
    pub duplicate_is_single_answer: bool,
    pub peak_temporary_cells: usize,
    pub peak_temporary_arrows: usize,
    pub average_spikes: f64,
    pub residual_temporary_cells: usize,
    pub residual_temporary_arrows: usize,
    pub temporary_capacity_released: bool,
    pub permanent_identity_cells: usize,
    pub permanent_fingerprint_unchanged: bool,
    pub ten_thousandth_fingerprint_unchanged: bool,
    pub permanent_route_strengths: Vec<(String, String, i32)>,
    pub passed: bool,
}

fn validation_accuracy(
    learner: &mut BindingLearner,
    identities: &mut IdentitySource,
    rng: &mut DeterministicRng,
    count: usize,
) -> usize {
    (0..count)
        .filter(|_| {
            let episode = normal_episode(identities, rng, 10);
            evaluate_episode(learner, &episode).0
        })
        .count()
}

fn control_episodes(
    identities: &mut IdentitySource,
) -> (
    BindingEpisode,
    BindingEpisode,
    BindingEpisode,
    BindingEpisode,
) {
    let a = identities.issue();
    let b = identities.issue();
    let c = identities.issue();
    let d = identities.issue();
    let e = identities.issue();
    let reverse = BindingEpisode {
        relations: vec![(a, b), (c, d)],
        query: b,
        correct: BindingOutcome::NotFound,
        distinct_identities: 4,
    };
    let missing = BindingEpisode {
        relations: vec![(a, b), (c, d)],
        query: e,
        correct: BindingOutcome::NotFound,
        distinct_identities: 5,
    };
    let conflict = BindingEpisode {
        relations: vec![(a, b), (a, c), (d, e)],
        query: a,
        correct: BindingOutcome::Ambiguous,
        distinct_identities: 5,
    };
    let duplicate = BindingEpisode {
        relations: vec![(a, b), (a, b), (c, d)],
        query: a,
        correct: BindingOutcome::Answer(b),
        distinct_identities: 4,
    };
    (reverse, missing, conflict, duplicate)
}

pub fn run_experiment() -> BindingReport {
    let mut learner = BindingLearner::new();
    let mut memorizer = PersistentMemorizer::default();
    let mut training_ids = IdentitySource::new(0x1900_0001);
    let mut training_rng = DeterministicRng::new(0x1900_0002);
    let mut validation_ids = IdentitySource::new(0x1900_1001);
    let mut validation_rng = DeterministicRng::new(0x1900_1002);
    let checkpoints_at = [10, 100, 1_000, 10_000];
    let mut checkpoints = Vec::new();

    for episode_index in 1..=10_000 {
        let episode = normal_episode(&mut training_ids, &mut training_rng, 10);
        present(&mut learner, &episode);
        let _proposal = learner.answer();
        learner.learn_from_terminal(episode.correct);
        assert!(learner.structural_arrows_are_symmetric());
        learner.erase_temporary();
        memorizer.train(&episode);

        if checkpoints_at.contains(&episode_index) {
            let fingerprint = learner.permanent_fingerprint();
            let correct =
                validation_accuracy(&mut learner, &mut validation_ids, &mut validation_rng, 100);
            assert_eq!(fingerprint, learner.permanent_fingerprint());
            let (permanent_cells, permanent_arrows) = learner.permanent_counts();
            checkpoints.push(BindingCheckpoint {
                training_episodes: episode_index,
                permanent_cells,
                permanent_arrows,
                validation_correct: correct,
                validation_total: 100,
            });
        }
    }

    let fingerprint_before = learner.permanent_fingerprint();
    let mut held_out_ids = IdentitySource::new(0x1900_2001);
    let mut held_out_rng = DeterministicRng::new(0x1900_2002);
    let mut held_out_correct = 0;
    let mut held_out_distinct_identities = 0;
    let mut memorizer_held_out_correct = 0;
    let mut peak_temporary_cells = 0;
    let mut peak_temporary_arrows = 0;
    let mut total_spikes = 0;
    let mut ten_thousandth_fingerprint_unchanged = false;

    for episode_index in 1..=20_000 {
        let episode = normal_episode(&mut held_out_ids, &mut held_out_rng, 10);
        held_out_distinct_identities += episode.distinct_identities;
        let killer_before = (episode_index == 10_000).then(|| learner.permanent_fingerprint());
        present(&mut learner, &episode);
        let (outcome, metrics) = learner.answer();
        held_out_correct += usize::from(outcome == episode.correct);
        memorizer_held_out_correct +=
            usize::from(memorizer.answer(episode.query) == episode.correct);
        peak_temporary_cells = peak_temporary_cells.max(metrics.temporary_cells);
        peak_temporary_arrows = peak_temporary_arrows.max(metrics.temporary_arrows);
        total_spikes += metrics.spikes;
        learner.erase_temporary();
        assert_eq!(learner.temporary_counts(), (0, 0));
        assert_eq!(learner.temporary_capacities(), (0, 0, 0));
        if let Some(killer_before) = killer_before {
            ten_thousandth_fingerprint_unchanged = killer_before == learner.permanent_fingerprint();
        }
    }

    let mut control_ids = IdentitySource::new(0x1900_3001);
    let (reverse, missing, conflict, duplicate) = control_episodes(&mut control_ids);
    let reverse_is_not_found = evaluate_episode(&mut learner, &reverse).0;
    let missing_is_not_found = evaluate_episode(&mut learner, &missing).0;
    let conflict_is_ambiguous = evaluate_episode(&mut learner, &conflict).0;
    let duplicate_is_single_answer = evaluate_episode(&mut learner, &duplicate).0;

    let (residual_temporary_cells, residual_temporary_arrows) = learner.temporary_counts();
    let temporary_capacity_released = learner.temporary_capacities() == (0, 0, 0);
    let permanent_identity_cells = learner.permanent_identity_cells();
    let permanent_fingerprint_unchanged = fingerprint_before == learner.permanent_fingerprint();
    let permanent_route_strengths = learner
        .route_strengths()
        .into_iter()
        .map(|(from, to, strength)| (format!("{from:?}"), format!("{to:?}"), strength))
        .collect::<Vec<_>>();
    let structure_plateaued = checkpoints.windows(2).all(|pair| {
        pair[0].permanent_cells == pair[1].permanent_cells
            && pair[0].permanent_arrows == pair[1].permanent_arrows
    });
    let checkpoint_accuracy = checkpoints
        .iter()
        .all(|checkpoint| checkpoint.validation_correct == checkpoint.validation_total);
    let passed = checkpoint_accuracy
        && structure_plateaued
        && held_out_correct == 20_000
        && memorizer_held_out_correct == 0
        && reverse_is_not_found
        && missing_is_not_found
        && conflict_is_ambiguous
        && duplicate_is_single_answer
        && peak_temporary_cells > 0
        && peak_temporary_arrows > 0
        && residual_temporary_cells == 0
        && residual_temporary_arrows == 0
        && temporary_capacity_released
        && permanent_identity_cells == 0
        && permanent_fingerprint_unchanged
        && ten_thousandth_fingerprint_unchanged;

    BindingReport {
        checkpoints,
        held_out_correct,
        held_out_total: 20_000,
        held_out_distinct_identities,
        memorizer_entries: memorizer.bindings.len(),
        memorizer_held_out_correct,
        reverse_is_not_found,
        missing_is_not_found,
        conflict_is_ambiguous,
        duplicate_is_single_answer,
        peak_temporary_cells,
        peak_temporary_arrows,
        average_spikes: total_spikes as f64 / 20_000.0,
        residual_temporary_cells,
        residual_temporary_arrows,
        temporary_capacity_released,
        permanent_identity_cells,
        permanent_fingerprint_unchanged,
        ten_thousandth_fingerprint_unchanged,
        permanent_route_strengths,
        passed,
    }
}

pub fn print_report(report: &BindingReport) {
    println!("v19 temporary binding:");
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
    println!(
        "  held-out fresh identities: {}/{}, distinct identities={}, memorizer={}/{} with {} entries",
        report.held_out_correct,
        report.held_out_total,
        report.held_out_distinct_identities,
        report.memorizer_held_out_correct,
        report.held_out_total,
        report.memorizer_entries
    );
    println!(
        "  controls reverse/missing/conflict/duplicate={}/{}/{}/{}, peak temporary cells/arrows={}/{}, residual={}/{}, capacity released={}",
        report.reverse_is_not_found,
        report.missing_is_not_found,
        report.conflict_is_ambiguous,
        report.duplicate_is_single_answer,
        report.peak_temporary_cells,
        report.peak_temporary_arrows,
        report.residual_temporary_cells,
        report.residual_temporary_arrows,
        report.temporary_capacity_released
    );
    println!(
        "  permanent identity cells={}, average spikes/query={:.1}, fingerprint unchanged={}, 10000th unchanged={}",
        report.permanent_identity_cells,
        report.average_spikes,
        report.permanent_fingerprint_unchanged,
        report.ten_thousandth_fingerprint_unchanged
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v19_learns_direction_from_terminal_supervision() {
        let report = run_experiment();
        let slot1_to_slot2 = report
            .permanent_route_strengths
            .iter()
            .find(|(from, to, _)| from == "Slot1" && to == "Slot2")
            .unwrap()
            .2;
        let alternatives = report
            .permanent_route_strengths
            .iter()
            .filter(|(from, to, _)| !(from == "Slot1" && to == "Slot2"))
            .map(|(_, _, strength)| *strength)
            .max()
            .unwrap();

        assert!(slot1_to_slot2 > alternatives);
        assert!(report
            .checkpoints
            .iter()
            .all(|point| point.validation_correct == point.validation_total));
    }

    #[test]
    fn v19_fresh_identities_use_temporary_not_permanent_memory() {
        let report = run_experiment();

        assert_eq!(report.held_out_correct, report.held_out_total);
        assert_eq!(report.held_out_distinct_identities, 400_000);
        assert_eq!(report.permanent_identity_cells, 0);
        assert_eq!(report.residual_temporary_cells, 0);
        assert_eq!(report.residual_temporary_arrows, 0);
        assert!(report.temporary_capacity_released);
    }

    #[test]
    fn v19_controls_distinguish_direction_conflict_and_duplicates() {
        let report = run_experiment();

        assert!(report.reverse_is_not_found);
        assert!(report.missing_is_not_found);
        assert!(report.conflict_is_ambiguous);
        assert!(report.duplicate_is_single_answer);
    }

    #[test]
    fn v19_held_out_evaluation_is_permanently_read_only() {
        let report = run_experiment();

        assert!(report.permanent_fingerprint_unchanged);
        assert!(report.ten_thousandth_fingerprint_unchanged);
        assert_eq!(report.memorizer_held_out_correct, 0);
        assert!(report.passed);
    }
}
