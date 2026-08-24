use std::collections::{BTreeMap, BTreeSet};

const ROLE_COUNT: usize = 6;
const ACTION_COUNT: usize = 6;
const TRAINING_EPISODES_PER_ACTION: usize = 64;
const HELD_OUT_SEEDS: usize = 16;

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
pub(crate) struct OpaqueAction(pub(crate) u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct OpaqueIdentity(pub(crate) u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct RoleId(pub(crate) usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TemporaryStructure {
    pub(crate) occupants: Vec<OpaqueIdentity>,
}

#[derive(Clone, Debug)]
pub(crate) struct IdentitySource {
    next: u64,
    namespace: u64,
}

impl IdentitySource {
    pub(crate) fn new(namespace: u64) -> Self {
        Self { next: 0, namespace }
    }

    pub(crate) fn fresh_structure(&mut self, role_count: usize) -> TemporaryStructure {
        TemporaryStructure {
            occupants: (0..role_count).map(|_| self.issue()).collect(),
        }
    }

    pub(crate) fn issue(&mut self) -> OpaqueIdentity {
        let identity = OpaqueIdentity(mix64(self.namespace ^ self.next));
        self.next += 1;
        identity
    }
}

pub(crate) fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RoleTransformation {
    pub(crate) source_for_output: Vec<RoleId>,
}

impl RoleTransformation {
    pub(crate) fn new(sources: impl IntoIterator<Item = usize>) -> Self {
        Self {
            source_for_output: sources.into_iter().map(RoleId).collect(),
        }
    }

    pub(crate) fn apply(&self, before: &TemporaryStructure) -> TemporaryStructure {
        TemporaryStructure {
            occupants: self
                .source_for_output
                .iter()
                .map(|source| before.occupants[source.0])
                .collect(),
        }
    }

    fn changed_roles(&self) -> BTreeSet<RoleId> {
        self.source_for_output
            .iter()
            .enumerate()
            .filter_map(|(output, source)| (output != source.0).then_some(RoleId(output)))
            .collect()
    }
}

fn environment_transformations() -> [RoleTransformation; ACTION_COUNT] {
    [
        RoleTransformation::new([0, 1, 2, 3, 4, 5]),
        RoleTransformation::new([1, 0, 2, 3, 4, 5]),
        RoleTransformation::new([1, 2, 0, 3, 4, 5]),
        RoleTransformation::new([2, 0, 1, 3, 4, 5]),
        RoleTransformation::new([0, 0, 2, 3, 4, 5]),
        RoleTransformation::new([0, 1, 2, 3, 5, 4]),
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SourceArrow {
    action: OpaqueAction,
    output_role: RoleId,
    input_role: RoleId,
    support: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct ProvenanceLearner {
    role_count: usize,
    arrows: Vec<SourceArrow>,
    proposed_arrows: usize,
    observed_identities: usize,
}

impl ProvenanceLearner {
    pub(crate) fn new(role_count: usize) -> Self {
        Self {
            role_count,
            arrows: Vec::new(),
            proposed_arrows: 0,
            observed_identities: 0,
        }
    }

    pub(crate) fn observe(
        &mut self,
        action: OpaqueAction,
        before: &TemporaryStructure,
        after: &TemporaryStructure,
    ) {
        assert_eq!(before.occupants.len(), self.role_count);
        assert_eq!(after.occupants.len(), self.role_count);
        self.observed_identities += self.role_count * 2;
        for output in 0..self.role_count {
            if !self
                .arrows
                .iter()
                .any(|arrow| arrow.action == action && arrow.output_role == RoleId(output))
            {
                for input in 0..self.role_count {
                    self.arrows.push(SourceArrow {
                        action,
                        output_role: RoleId(output),
                        input_role: RoleId(input),
                        support: 0,
                    });
                    self.proposed_arrows += 1;
                }
                self.arrows.sort();
            }

            for input in 0..self.role_count {
                if after.occupants[output] == before.occupants[input] {
                    self.arrows
                        .iter_mut()
                        .find(|arrow| {
                            arrow.action == action
                                && arrow.output_role == RoleId(output)
                                && arrow.input_role == RoleId(input)
                        })
                        .unwrap()
                        .support += 1;
                }
            }
        }
    }

    pub(crate) fn predict(&self, action: OpaqueAction) -> Option<RoleTransformation> {
        let mut sources = vec![RoleId(0); self.role_count];
        for (output, source) in sources.iter_mut().enumerate() {
            let candidates = self
                .arrows
                .iter()
                .filter(|arrow| arrow.action == action && arrow.output_role == RoleId(output))
                .collect::<Vec<_>>();
            if candidates.len() != self.role_count {
                return None;
            }
            let total = candidates.iter().map(|arrow| arrow.support).sum::<usize>();
            let strongest = candidates.iter().map(|arrow| arrow.support).max()?;
            let winners = candidates
                .iter()
                .filter(|arrow| arrow.support == strongest)
                .collect::<Vec<_>>();
            if total < 8 || strongest * 8 < total * 7 || winners.len() != 1 {
                return None;
            }
            *source = winners[0].input_role;
        }
        Some(RoleTransformation {
            source_for_output: sources,
        })
    }

    pub(crate) fn model_entries(&self) -> usize {
        self.arrows.len()
    }

    fn permanent_identity_cells(&self) -> usize {
        0
    }

    pub(crate) fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for arrow in &self.arrows {
            fingerprint_mix(&mut hash, arrow.action.0);
            fingerprint_mix(&mut hash, arrow.output_role.0 as u64);
            fingerprint_mix(&mut hash, arrow.input_role.0 as u64);
            fingerprint_mix(&mut hash, arrow.support as u64);
        }
        fingerprint_mix(&mut hash, self.proposed_arrows as u64);
        hash
    }
}

fn fingerprint_mix(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(0x100_0000_01b3);
}

#[derive(Clone, Debug)]
struct ActionWorld {
    action_for_effect: [OpaqueAction; ACTION_COUNT],
    effect_for_action: BTreeMap<OpaqueAction, RoleTransformation>,
}

impl ActionWorld {
    fn new(seed: u64) -> Self {
        let mut rng = DeterministicRng::new(seed ^ 0xd4a0_0000);
        let mut actions =
            std::array::from_fn(|index| OpaqueAction(mix64(seed ^ 0xa400_0000 ^ index as u64)));
        rng.shuffle(&mut actions);
        let transformations = environment_transformations();
        let effect_for_action = actions
            .iter()
            .copied()
            .zip(transformations)
            .collect::<BTreeMap<_, _>>();
        Self {
            action_for_effect: actions,
            effect_for_action,
        }
    }

    fn apply(&self, action: OpaqueAction, state: &TemporaryStructure) -> TemporaryStructure {
        self.effect_for_action[&action].apply(state)
    }

    fn action(&self, effect_index: usize) -> OpaqueAction {
        self.action_for_effect[effect_index]
    }
}

fn train_models(seed: u64, shuffled_outcomes: bool) -> (ActionWorld, ProvenanceLearner) {
    let world = ActionWorld::new(seed);
    let mut learner = ProvenanceLearner::new(ROLE_COUNT);
    let mut identities = IdentitySource::new(seed ^ 0xd4a1_0000);

    for episode in 0..TRAINING_EPISODES_PER_ACTION {
        for action_index in 0..ACTION_COUNT {
            let action = world.action(action_index);
            let before = identities.fresh_structure(ROLE_COUNT);
            let observed_action_index = if shuffled_outcomes {
                (action_index + episode) % ACTION_COUNT
            } else {
                action_index
            };
            let after = world.apply(world.action(observed_action_index), &before);
            learner.observe(action, &before, &after);
        }
    }
    (world, learner)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum MaskValue {
    Known(OpaqueIdentity),
    Unknown,
}

#[derive(Clone, Debug)]
struct ChangedMaskModel {
    changed_by_action: BTreeMap<OpaqueAction, BTreeSet<RoleId>>,
}

impl ChangedMaskModel {
    fn from_world(world: &ActionWorld) -> Self {
        Self::from_effects(&world.effect_for_action)
    }

    fn from_effects(effects: &BTreeMap<OpaqueAction, RoleTransformation>) -> Self {
        Self {
            changed_by_action: effects
                .iter()
                .map(|(&action, effect)| (action, effect.changed_roles()))
                .collect(),
        }
    }

    fn apply(&self, action: OpaqueAction, before: &[MaskValue]) -> Vec<MaskValue> {
        let changed = &self.changed_by_action[&action];
        before
            .iter()
            .enumerate()
            .map(|(role, value)| {
                if changed.contains(&RoleId(role)) {
                    MaskValue::Unknown
                } else {
                    value.clone()
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
struct SequenceTemplate {
    effect_indices: Vec<usize>,
    label: String,
}

fn held_out_sequences() -> Vec<SequenceTemplate> {
    let mut sequences = Vec::new();
    for action in 0..ACTION_COUNT {
        sequences.push(SequenceTemplate {
            effect_indices: vec![action],
            label: format!("single-{action}"),
        });
        sequences.push(SequenceTemplate {
            effect_indices: vec![action, action],
            label: format!("repeat-{action}"),
        });
    }
    for first in 0..ACTION_COUNT {
        for second in 0..ACTION_COUNT {
            if first != second {
                sequences.push(SequenceTemplate {
                    effect_indices: vec![first, second],
                    label: format!("pair-{first}-{second}"),
                });
            }
        }
    }
    for effects in [
        vec![1, 2, 5],
        vec![2, 4, 3],
        vec![3, 1, 4],
        vec![4, 2, 1],
        vec![5, 3, 2],
        vec![2, 3, 2, 3],
        vec![1, 5, 1, 5],
        vec![4, 2, 3, 5],
        vec![2, 4, 3, 1, 5, 2, 4, 3],
        vec![3, 2, 5, 4, 1, 3, 2, 5],
        vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 2, 3, 1, 4, 5, 2],
    ] {
        sequences.push(SequenceTemplate {
            label: format!("length-{}", effects.len()),
            effect_indices: effects,
        });
    }
    sequences
}

fn apply_frozen_sequence(
    models: &BTreeMap<OpaqueAction, RoleTransformation>,
    initial: &TemporaryStructure,
    actions: &[OpaqueAction],
) -> Option<(TemporaryStructure, Vec<TemporaryStructure>)> {
    let mut current = initial.clone();
    let mut steps = Vec::with_capacity(actions.len());
    for action in actions {
        current = models.get(action)?.apply(&current);
        steps.push(current.clone());
    }
    Some((current, steps))
}

fn apply_mask_sequence(
    model: &ChangedMaskModel,
    initial: &TemporaryStructure,
    actions: &[OpaqueAction],
) -> Vec<MaskValue> {
    let mut current = initial
        .occupants
        .iter()
        .copied()
        .map(MaskValue::Known)
        .collect::<Vec<_>>();
    for &action in actions {
        current = model.apply(action, &current);
    }
    current
}

#[derive(Clone, Debug)]
pub struct CompositionStepTrace {
    pub step: usize,
    pub action_id: u64,
    pub source_roles: Vec<usize>,
    pub resulting_identity_ids: Vec<u64>,
}

#[derive(Clone, Debug)]
pub struct CompositionTrace {
    pub seed_index: usize,
    pub sequence_label: String,
    pub initial_identity_ids: Vec<u64>,
    pub expected_identity_ids: Vec<u64>,
    pub predicted_identity_ids: Vec<u64>,
    pub exact: bool,
    pub model_fingerprint_before: u64,
    pub model_fingerprint_after: u64,
    pub steps: Vec<CompositionStepTrace>,
}

#[derive(Clone, Debug)]
pub struct ComposableModelReport {
    pub exact_predictions: usize,
    pub total_predictions: usize,
    pub mask_baseline_exact: usize,
    pub shuffled_confident_models: usize,
    pub matched_mask_pairs: usize,
    pub provenance_distinguishable_pairs: usize,
    pub mask_distinguishable_pairs: usize,
    pub swap_twice_correct: usize,
    pub order_sensitive_pairs: usize,
    pub proposed_arrows: usize,
    pub model_entries: usize,
    pub observed_identities: usize,
    pub permanent_identity_cells: usize,
    pub model_size_fixed: bool,
    pub frozen_fingerprint_unchanged: bool,
    pub exact_by_sequence_length: BTreeMap<usize, (usize, usize)>,
    pub model_applications: usize,
    pub maximum_temporary_roles: usize,
    pub traces: Vec<CompositionTrace>,
    pub passed: bool,
}

pub fn run_composable_model_experiment() -> ComposableModelReport {
    let mut exact_predictions = 0;
    let mut total_predictions = 0;
    let mut mask_baseline_exact = 0;
    let mut shuffled_confident_models = 0;
    let mut matched_mask_pairs = 0;
    let mut provenance_distinguishable_pairs = 0;
    let mut mask_distinguishable_pairs = 0;
    let mut swap_twice_correct = 0;
    let mut order_sensitive_pairs = 0;
    let mut proposed_arrows = 0;
    let mut model_entries = 0;
    let mut observed_identities = 0;
    let mut exact_by_sequence_length = BTreeMap::<usize, (usize, usize)>::new();
    let mut model_applications = 0;
    let mut model_size_fixed = true;
    let mut frozen_fingerprint_unchanged = true;
    let mut traces = Vec::new();

    for seed_index in 0..HELD_OUT_SEEDS {
        let seed = 0xd4a2_0000 + seed_index as u64;
        let (world, learner) = train_models(seed, false);
        let (_, shuffled) = train_models(seed, true);
        proposed_arrows += learner.proposed_arrows;
        model_entries += learner.model_entries();
        observed_identities += learner.observed_identities;
        shuffled_confident_models += (0..ACTION_COUNT)
            .filter(|&index| shuffled.predict(world.action(index)).is_some())
            .count();

        let (_, small_learner) = {
            let world = ActionWorld::new(seed);
            let mut learner = ProvenanceLearner::new(ROLE_COUNT);
            let mut identities = IdentitySource::new(seed ^ 0xd4a3_0000);
            for _ in 0..8 {
                for action_index in 0..ACTION_COUNT {
                    let before = identities.fresh_structure(ROLE_COUNT);
                    let action = world.action(action_index);
                    let after = world.apply(action, &before);
                    learner.observe(action, &before, &after);
                }
            }
            (world, learner)
        };
        model_size_fixed &= small_learner.model_entries() == learner.model_entries();

        let models = (0..ACTION_COUNT)
            .map(|index| {
                let action = world.action(index);
                (action, learner.predict(action).unwrap())
            })
            .collect::<BTreeMap<_, _>>();
        let model_fingerprint = learner.fingerprint();
        let mask_model = ChangedMaskModel::from_world(&world);
        let mut control_identities = IdentitySource::new(seed ^ 0xd4a5_0000);
        let control_initial = control_identities.fresh_structure(ROLE_COUNT);
        let swap = world.action(1);
        let (swap_twice, _) =
            apply_frozen_sequence(&models, &control_initial, &[swap, swap]).unwrap();
        swap_twice_correct += usize::from(swap_twice == control_initial);

        let first_order = [world.action(1), world.action(2)];
        let second_order = [world.action(2), world.action(1)];
        let first_expected = first_order
            .iter()
            .fold(control_initial.clone(), |state, &action| {
                world.apply(action, &state)
            });
        let second_expected = second_order
            .iter()
            .fold(control_initial.clone(), |state, &action| {
                world.apply(action, &state)
            });
        let (first_predicted, _) =
            apply_frozen_sequence(&models, &control_initial, &first_order).unwrap();
        let (second_predicted, _) =
            apply_frozen_sequence(&models, &control_initial, &second_order).unwrap();
        order_sensitive_pairs += usize::from(
            first_expected != second_expected
                && first_predicted == first_expected
                && second_predicted == second_expected,
        );

        let left_rotation = models[&world.action(2)].clone();
        let right_rotation = models[&world.action(3)].clone();
        matched_mask_pairs += 1;
        provenance_distinguishable_pairs += usize::from(left_rotation != right_rotation);
        mask_distinguishable_pairs += usize::from(
            world.effect_for_action[&world.action(2)].changed_roles()
                != world.effect_for_action[&world.action(3)].changed_roles(),
        );

        let mut identities = IdentitySource::new(seed ^ 0xd4a4_0000);
        for sequence in held_out_sequences() {
            let initial = identities.fresh_structure(ROLE_COUNT);
            let actions = sequence
                .effect_indices
                .iter()
                .map(|&index| world.action(index))
                .collect::<Vec<_>>();
            let expected = actions.iter().fold(initial.clone(), |state, &action| {
                world.apply(action, &state)
            });
            let (predicted, steps) = apply_frozen_sequence(&models, &initial, &actions).unwrap();
            let exact = predicted == expected;
            exact_predictions += usize::from(exact);
            total_predictions += 1;
            model_applications += actions.len();
            let entry = exact_by_sequence_length
                .entry(actions.len())
                .or_insert((0, 0));
            entry.0 += usize::from(exact);
            entry.1 += 1;

            let mask_prediction = apply_mask_sequence(&mask_model, &initial, &actions);
            let mask_exact = mask_prediction
                .iter()
                .zip(&expected.occupants)
                .all(|(predicted, expected)| *predicted == MaskValue::Known(*expected));
            mask_baseline_exact += usize::from(mask_exact);

            traces.push(CompositionTrace {
                seed_index,
                sequence_label: sequence.label,
                initial_identity_ids: initial
                    .occupants
                    .iter()
                    .map(|identity| identity.0)
                    .collect(),
                expected_identity_ids: expected
                    .occupants
                    .iter()
                    .map(|identity| identity.0)
                    .collect(),
                predicted_identity_ids: predicted
                    .occupants
                    .iter()
                    .map(|identity| identity.0)
                    .collect(),
                exact,
                model_fingerprint_before: model_fingerprint,
                model_fingerprint_after: learner.fingerprint(),
                steps: actions
                    .iter()
                    .zip(steps)
                    .enumerate()
                    .map(|(step, (&action, state))| CompositionStepTrace {
                        step: step + 1,
                        action_id: action.0,
                        source_roles: models[&action]
                            .source_for_output
                            .iter()
                            .map(|role| role.0)
                            .collect(),
                        resulting_identity_ids: state
                            .occupants
                            .iter()
                            .map(|identity| identity.0)
                            .collect(),
                    })
                    .collect(),
            });
        }
        frozen_fingerprint_unchanged &= traces
            .iter()
            .filter(|trace| trace.seed_index == seed_index)
            .all(|trace| trace.model_fingerprint_before == trace.model_fingerprint_after);
    }

    let permanent_identity_cells = train_models(0xd4af_0000, false)
        .1
        .permanent_identity_cells();
    let passed = exact_predictions == total_predictions
        && shuffled_confident_models == 0
        && provenance_distinguishable_pairs == matched_mask_pairs
        && mask_distinguishable_pairs == 0
        && swap_twice_correct == HELD_OUT_SEEDS
        && order_sensitive_pairs == HELD_OUT_SEEDS
        && mask_baseline_exact < total_predictions
        && permanent_identity_cells == 0
        && model_size_fixed
        && frozen_fingerprint_unchanged
        && exact_by_sequence_length
            .iter()
            .all(|(_, &(correct, total))| correct == total);

    ComposableModelReport {
        exact_predictions,
        total_predictions,
        mask_baseline_exact,
        shuffled_confident_models,
        matched_mask_pairs,
        provenance_distinguishable_pairs,
        mask_distinguishable_pairs,
        swap_twice_correct,
        order_sensitive_pairs,
        proposed_arrows,
        model_entries,
        observed_identities,
        permanent_identity_cells,
        model_size_fixed,
        frozen_fingerprint_unchanged,
        exact_by_sequence_length,
        model_applications,
        maximum_temporary_roles: ROLE_COUNT,
        traces,
        passed,
    }
}

pub fn print_report(report: &ComposableModelReport) {
    println!("d4a composable role-relative transformations:");
    println!(
        "  exact sequence predictions={}/{}, changed-mask baseline={}/{}",
        report.exact_predictions,
        report.total_predictions,
        report.mask_baseline_exact,
        report.total_predictions
    );
    println!(
        "  matched-mask provenance pairs={}/{}, mask distinguishes={}",
        report.provenance_distinguishable_pairs,
        report.matched_mask_pairs,
        report.mask_distinguishable_pairs
    );
    println!(
        "  swap-twice recovery={}/{}, order-sensitive pairs={}/{}",
        report.swap_twice_correct, HELD_OUT_SEEDS, report.order_sensitive_pairs, HELD_OUT_SEEDS
    );
    println!(
        "  proposed arrows={}, model entries={}, observed identities={}, permanent identity cells={}",
        report.proposed_arrows,
        report.model_entries,
        report.observed_identities,
        report.permanent_identity_cells
    );
    println!(
        "  applications={}, temporary roles={}, frozen fingerprint unchanged={}",
        report.model_applications,
        report.maximum_temporary_roles,
        report.frozen_fingerprint_unchanged
    );
    for (length, (correct, total)) in &report.exact_by_sequence_length {
        println!("    sequence length {length}: {correct}/{total}");
    }
}

const D4B_ROLE_COUNT: usize = 13;
const D4B_ACTION_COUNT: usize = 3;
const D4B_SEEDS: usize = 8;
const D4B_DEPTHS: [usize; 5] = [1, 2, 3, 4, 8];

#[derive(Clone, Debug)]
struct PlanningWorld {
    action_for_effect: [OpaqueAction; D4B_ACTION_COUNT],
    effect_for_action: BTreeMap<OpaqueAction, RoleTransformation>,
}

impl PlanningWorld {
    fn new(seed: u64) -> Self {
        let mut shift = (0..D4B_ROLE_COUNT).collect::<Vec<_>>();
        shift[0] = 8;
        for (role, source) in shift.iter_mut().enumerate().take(9).skip(1) {
            *source = role - 1;
        }

        let mut reveal = (0..D4B_ROLE_COUNT).collect::<Vec<_>>();
        reveal[9] = 8;
        let identity = (0..D4B_ROLE_COUNT).collect::<Vec<_>>();
        let transformations = [
            RoleTransformation::new(shift),
            RoleTransformation::new(reveal),
            RoleTransformation::new(identity),
        ];

        let mut rng = DeterministicRng::new(seed ^ 0xd4b0_0000);
        let mut actions =
            std::array::from_fn(|index| OpaqueAction(mix64(seed ^ 0xb400_0000 ^ index as u64)));
        rng.shuffle(&mut actions);
        let effect_for_action = actions
            .iter()
            .copied()
            .zip(transformations)
            .collect::<BTreeMap<_, _>>();
        Self {
            action_for_effect: actions,
            effect_for_action,
        }
    }

    fn action(&self, effect_index: usize) -> OpaqueAction {
        self.action_for_effect[effect_index]
    }

    fn apply(&self, action: OpaqueAction, state: &TemporaryStructure) -> TemporaryStructure {
        self.effect_for_action[&action].apply(state)
    }
}

fn train_planning_models(seed: u64) -> (PlanningWorld, ProvenanceLearner) {
    let world = PlanningWorld::new(seed);
    let mut learner = ProvenanceLearner::new(D4B_ROLE_COUNT);
    let mut identities = IdentitySource::new(seed ^ 0xd4b1_0000);
    for _ in 0..TRAINING_EPISODES_PER_ACTION {
        for action_index in 0..D4B_ACTION_COUNT {
            let action = world.action(action_index);
            let before = identities.fresh_structure(D4B_ROLE_COUNT);
            let after = world.apply(action, &before);
            learner.observe(action, &before, &after);
        }
    }
    (world, learner)
}

#[derive(Clone, Debug)]
struct RouteStructure {
    dependencies: BTreeSet<RoleId>,
}

#[derive(Clone, Debug)]
struct RouteComparison {
    shared: BTreeSet<RoleId>,
    first_only: BTreeSet<RoleId>,
    second_only: BTreeSet<RoleId>,
}

fn compare_route_structures(first: &RouteStructure, second: &RouteStructure) -> RouteComparison {
    RouteComparison {
        shared: first
            .dependencies
            .intersection(&second.dependencies)
            .copied()
            .collect(),
        first_only: first
            .dependencies
            .difference(&second.dependencies)
            .copied()
            .collect(),
        second_only: second
            .dependencies
            .difference(&first.dependencies)
            .copied()
            .collect(),
    }
}

fn route_output(state: &TemporaryStructure, route: &RouteStructure) -> Vec<OpaqueIdentity> {
    route
        .dependencies
        .iter()
        .map(|role| state.occupants[role.0])
        .collect()
}

fn changes_roles(
    before: &TemporaryStructure,
    after: &TemporaryStructure,
    roles: &BTreeSet<RoleId>,
) -> usize {
    roles
        .iter()
        .filter(|role| before.occupants[role.0] != after.occupants[role.0])
        .count()
}

fn is_distinguishing_state(
    initial: &TemporaryStructure,
    predicted: &TemporaryStructure,
    first: &RouteStructure,
    second: &RouteStructure,
    comparison: &RouteComparison,
) -> bool {
    let first_changes = changes_roles(initial, predicted, &comparison.first_only);
    let second_changes = changes_roles(initial, predicted, &comparison.second_only);
    let shared_changes = changes_roles(initial, predicted, &comparison.shared);
    shared_changes == 0
        && (first_changes > 0) != (second_changes > 0)
        && route_output(predicted, first) != route_output(predicted, second)
}

fn planning_routes() -> (RouteStructure, RouteStructure, RouteComparison) {
    let first = RouteStructure {
        dependencies: BTreeSet::from([RoleId(9), RoleId(12)]),
    };
    let second = RouteStructure {
        dependencies: BTreeSet::from([RoleId(10), RoleId(12)]),
    };
    let comparison = compare_route_structures(&first, &second);
    (first, second, comparison)
}

fn planning_initial(
    identities: &mut IdentitySource,
    required_depth: Option<usize>,
) -> (TemporaryStructure, OpaqueIdentity) {
    let base = identities.issue();
    let marker = identities.issue();
    let mut occupants = vec![base; D4B_ROLE_COUNT];
    if let Some(depth) = required_depth {
        occupants[9 - depth] = marker;
    }
    (TemporaryStructure { occupants }, marker)
}

fn sorted_actions(models: &BTreeMap<OpaqueAction, RoleTransformation>) -> Vec<OpaqueAction> {
    models.keys().copied().collect()
}

fn sequence_from_index(
    actions: &[OpaqueAction],
    length: usize,
    mut index: usize,
) -> Vec<OpaqueAction> {
    let mut sequence = vec![actions[0]; length];
    for position in (0..length).rev() {
        sequence[position] = actions[index % actions.len()];
        index /= actions.len();
    }
    sequence
}

#[derive(Clone, Debug)]
pub struct PlanningCandidateTrace {
    pub seed_index: usize,
    pub required_depth: usize,
    pub reachable: bool,
    pub candidate_index: usize,
    pub action_ids: Vec<u64>,
    pub predicted_marker_roles: Vec<usize>,
    pub distinguishes: bool,
    pub before_real_action: bool,
}

#[derive(Clone, Debug)]
struct SearchResult {
    selected: Option<Vec<OpaqueAction>>,
    predicted: Option<TemporaryStructure>,
    candidates_examined: usize,
    model_applications: usize,
    traces: Vec<PlanningCandidateTrace>,
}

struct SearchContext<'a> {
    seed_index: usize,
    required_depth: usize,
    reachable: bool,
    marker: OpaqueIdentity,
    first: &'a RouteStructure,
    second: &'a RouteStructure,
    comparison: &'a RouteComparison,
}

fn bounded_model_search(
    models: &BTreeMap<OpaqueAction, RoleTransformation>,
    initial: &TemporaryStructure,
    maximum_depth: usize,
    context: &SearchContext<'_>,
    record_traces: bool,
) -> SearchResult {
    let actions = sorted_actions(models);
    let mut candidates_examined = 0;
    let mut model_applications = 0;
    let mut traces = Vec::new();

    for length in 1..=maximum_depth {
        let candidate_count = actions.len().pow(length as u32);
        for candidate_in_length in 0..candidate_count {
            let sequence = sequence_from_index(&actions, length, candidate_in_length);
            let (predicted, _) = apply_frozen_sequence(models, initial, &sequence).unwrap();
            let distinguishes = is_distinguishing_state(
                initial,
                &predicted,
                context.first,
                context.second,
                context.comparison,
            );
            candidates_examined += 1;
            model_applications += sequence.len();
            if record_traces {
                traces.push(PlanningCandidateTrace {
                    seed_index: context.seed_index,
                    required_depth: context.required_depth,
                    reachable: context.reachable,
                    candidate_index: candidates_examined,
                    action_ids: sequence.iter().map(|action| action.0).collect(),
                    predicted_marker_roles: predicted
                        .occupants
                        .iter()
                        .enumerate()
                        .filter_map(|(role, identity)| {
                            (*identity == context.marker).then_some(role)
                        })
                        .collect(),
                    distinguishes,
                    before_real_action: true,
                });
            }
            if distinguishes {
                return SearchResult {
                    selected: Some(sequence),
                    predicted: Some(predicted),
                    candidates_examined,
                    model_applications,
                    traces,
                };
            }
        }
    }

    SearchResult {
        selected: None,
        predicted: None,
        candidates_examined,
        model_applications,
        traces,
    }
}

fn mask_search(
    model: &ChangedMaskModel,
    actions: &[OpaqueAction],
    initial: &TemporaryStructure,
    maximum_depth: usize,
    first: &RouteStructure,
    second: &RouteStructure,
) -> Option<Vec<OpaqueAction>> {
    for length in 1..=maximum_depth {
        for candidate in 0..actions.len().pow(length as u32) {
            let sequence = sequence_from_index(actions, length, candidate);
            let predicted = apply_mask_sequence(model, initial, &sequence);
            let route_known = |route: &RouteStructure| {
                route
                    .dependencies
                    .iter()
                    .map(|role| predicted[role.0].clone())
                    .collect::<Vec<_>>()
            };
            let first_output = route_known(first);
            let second_output = route_known(second);
            if first_output
                .iter()
                .all(|value| matches!(value, MaskValue::Known(_)))
                && second_output
                    .iter()
                    .all(|value| matches!(value, MaskValue::Known(_)))
                && first_output != second_output
            {
                return Some(sequence);
            }
        }
    }
    None
}

fn random_search_with_budget(
    models: &BTreeMap<OpaqueAction, RoleTransformation>,
    initial: &TemporaryStructure,
    maximum_depth: usize,
    candidate_budget: usize,
    seed: u64,
    context: &SearchContext<'_>,
) -> Option<Vec<OpaqueAction>> {
    let actions = sorted_actions(models);
    let mut candidates = Vec::new();
    for length in 1..=maximum_depth {
        for candidate in 0..actions.len().pow(length as u32) {
            candidates.push(sequence_from_index(&actions, length, candidate));
        }
    }
    let mut rng = DeterministicRng::new(seed);
    rng.shuffle(&mut candidates);
    for sequence in candidates.into_iter().take(candidate_budget) {
        let (predicted, _) = apply_frozen_sequence(models, initial, &sequence).unwrap();
        if is_distinguishing_state(
            initial,
            &predicted,
            context.first,
            context.second,
            context.comparison,
        ) {
            return Some(sequence);
        }
    }
    None
}

#[derive(Clone, Debug)]
pub struct PlanningDepthReport {
    pub required_depth: usize,
    pub cases: usize,
    pub correct: usize,
    pub real_execution_correct: usize,
    pub shortest_correct: usize,
    pub oracle_correct: usize,
    pub one_step_correct: usize,
    pub mask_planner_correct: usize,
    pub random_correct: usize,
    pub candidates_examined: usize,
    pub model_applications: usize,
    pub role_transfers: usize,
}

#[derive(Clone, Debug)]
pub struct PlanningReport {
    pub depth_reports: Vec<PlanningDepthReport>,
    pub unreachable_correct: usize,
    pub unreachable_total: usize,
    pub order_sensitive_correct: usize,
    pub order_sensitive_total: usize,
    pub permanent_model_entries: usize,
    pub permanent_model_size_flat: bool,
    pub frozen_fingerprint_unchanged: bool,
    pub all_planning_before_action: bool,
    pub training_sequences: usize,
    pub traces: Vec<PlanningCandidateTrace>,
    pub passed: bool,
}

pub fn run_planning_experiment() -> PlanningReport {
    let (first, second, comparison) = planning_routes();
    let mut depth_reports = D4B_DEPTHS
        .iter()
        .map(|&required_depth| PlanningDepthReport {
            required_depth,
            cases: 0,
            correct: 0,
            real_execution_correct: 0,
            shortest_correct: 0,
            oracle_correct: 0,
            one_step_correct: 0,
            mask_planner_correct: 0,
            random_correct: 0,
            candidates_examined: 0,
            model_applications: 0,
            role_transfers: 0,
        })
        .collect::<Vec<_>>();
    let mut unreachable_correct = 0;
    let mut order_sensitive_correct = 0;
    let mut permanent_model_entries = 0;
    let mut permanent_model_size_flat = true;
    let mut frozen_fingerprint_unchanged = true;
    let mut all_planning_before_action = true;
    let mut traces = Vec::new();

    for seed_index in 0..D4B_SEEDS {
        let seed = 0xd4b2_0000 + seed_index as u64;
        let (world, learner) = train_planning_models(seed);
        let models = (0..D4B_ACTION_COUNT)
            .map(|index| {
                let action = world.action(index);
                (action, learner.predict(action).unwrap())
            })
            .collect::<BTreeMap<_, _>>();
        let fingerprint = learner.fingerprint();
        let entries = learner.model_entries();
        if seed_index == 0 {
            permanent_model_entries = entries;
        } else {
            permanent_model_size_flat &= entries == permanent_model_entries;
        }
        let actions = sorted_actions(&models);
        let mask_model = ChangedMaskModel::from_effects(&world.effect_for_action);

        for (depth_index, &required_depth) in D4B_DEPTHS.iter().enumerate() {
            let mut identities = IdentitySource::new(seed ^ 0xd4b3_0000 ^ required_depth as u64);
            let (initial, marker) = planning_initial(&mut identities, Some(required_depth));
            let context = SearchContext {
                seed_index,
                required_depth,
                reachable: true,
                marker,
                first: &first,
                second: &second,
                comparison: &comparison,
            };
            let result = bounded_model_search(&models, &initial, required_depth, &context, true);
            let selected = result.selected.clone();
            let predicted = result.predicted.clone();
            let correct = selected.as_ref().is_some_and(|sequence| {
                sequence.len() == required_depth
                    && predicted.as_ref().is_some_and(|state| {
                        is_distinguishing_state(&initial, state, &first, &second, &comparison)
                    })
            });
            let real = selected.as_ref().map(|sequence| {
                sequence.iter().fold(initial.clone(), |state, &action| {
                    world.apply(action, &state)
                })
            });
            let real_execution_correct = real.as_ref().is_some_and(|state| {
                predicted.as_ref() == Some(state)
                    && is_distinguishing_state(&initial, state, &first, &second, &comparison)
            });

            let oracle = bounded_model_search(
                &world.effect_for_action,
                &initial,
                required_depth,
                &context,
                false,
            );
            let one_step = bounded_model_search(&models, &initial, 1, &context, false).selected;
            let mask = mask_search(
                &mask_model,
                &actions,
                &initial,
                required_depth,
                &first,
                &second,
            );
            let random = random_search_with_budget(
                &models,
                &initial,
                required_depth,
                result.candidates_examined,
                seed ^ 0xd4b4_0000 ^ required_depth as u64,
                &context,
            );

            let report = &mut depth_reports[depth_index];
            report.cases += 1;
            report.correct += usize::from(correct);
            report.real_execution_correct += usize::from(real_execution_correct);
            report.shortest_correct += usize::from(
                selected
                    .as_ref()
                    .is_some_and(|sequence| sequence.len() == required_depth),
            );
            report.oracle_correct += usize::from(oracle.selected == selected);
            report.one_step_correct += usize::from(one_step.is_some());
            report.mask_planner_correct += usize::from(mask.is_some());
            report.random_correct += usize::from(random.is_some());
            report.candidates_examined += result.candidates_examined;
            report.model_applications += result.model_applications;
            report.role_transfers += result.model_applications * D4B_ROLE_COUNT;
            all_planning_before_action &=
                result.traces.iter().all(|trace| trace.before_real_action);
            traces.extend(result.traces);

            if required_depth == 2 {
                let shift_then_reveal = [world.action(0), world.action(1)];
                let reveal_then_shift = [world.action(1), world.action(0)];
                let first_result = apply_frozen_sequence(&models, &initial, &shift_then_reveal)
                    .unwrap()
                    .0;
                let second_result = apply_frozen_sequence(&models, &initial, &reveal_then_shift)
                    .unwrap()
                    .0;
                order_sensitive_correct += usize::from(
                    is_distinguishing_state(&initial, &first_result, &first, &second, &comparison)
                        && !is_distinguishing_state(
                            &initial,
                            &second_result,
                            &first,
                            &second,
                            &comparison,
                        ),
                );
            }
            frozen_fingerprint_unchanged &= learner.fingerprint() == fingerprint;
        }

        let mut identities = IdentitySource::new(seed ^ 0xd4b5_0000);
        let (initial, marker) = planning_initial(&mut identities, None);
        let context = SearchContext {
            seed_index,
            required_depth: 8,
            reachable: false,
            marker,
            first: &first,
            second: &second,
            comparison: &comparison,
        };
        let unreachable = bounded_model_search(&models, &initial, 8, &context, true);
        unreachable_correct += usize::from(unreachable.selected.is_none());
        all_planning_before_action &= unreachable
            .traces
            .iter()
            .all(|trace| trace.before_real_action);
        traces.extend(unreachable.traces);
        frozen_fingerprint_unchanged &= learner.fingerprint() == fingerprint;
    }

    let candidate_growth = depth_reports
        .windows(2)
        .all(|pair| pair[0].candidates_examined < pair[1].candidates_examined);
    let work_growth = depth_reports
        .windows(2)
        .all(|pair| pair[0].model_applications < pair[1].model_applications);
    let reachable_cases = D4B_SEEDS;
    let passed = depth_reports.iter().all(|report| {
        report.correct == reachable_cases
            && report.real_execution_correct == reachable_cases
            && report.shortest_correct == reachable_cases
            && report.oracle_correct == reachable_cases
            && report.mask_planner_correct == 0
            && if report.required_depth == 1 {
                report.one_step_correct == reachable_cases
            } else {
                report.one_step_correct == 0
            }
    }) && depth_reports
        .iter()
        .map(|report| report.random_correct)
        .sum::<usize>()
        < D4B_DEPTHS.len() * D4B_SEEDS
        && unreachable_correct == D4B_SEEDS
        && order_sensitive_correct == D4B_SEEDS
        && permanent_model_size_flat
        && frozen_fingerprint_unchanged
        && all_planning_before_action
        && candidate_growth
        && work_growth;

    PlanningReport {
        depth_reports,
        unreachable_correct,
        unreachable_total: D4B_SEEDS,
        order_sensitive_correct,
        order_sensitive_total: D4B_SEEDS,
        permanent_model_entries,
        permanent_model_size_flat,
        frozen_fingerprint_unchanged,
        all_planning_before_action,
        training_sequences: 0,
        traces,
        passed,
    }
}

pub fn print_planning_report(report: &PlanningReport) {
    println!("d4b counterfactual search over learned transformations:");
    for depth in &report.depth_reports {
        println!(
            "  depth {}: model/real/oracle/random/one-step/mask={}/{}/{}/{}/{}/{} of {}, candidates={:.1}, applications={:.1}, role transfers={:.1}",
            depth.required_depth,
            depth.correct,
            depth.real_execution_correct,
            depth.oracle_correct,
            depth.random_correct,
            depth.one_step_correct,
            depth.mask_planner_correct,
            depth.cases,
            depth.candidates_examined as f64 / depth.cases as f64,
            depth.model_applications as f64 / depth.cases as f64,
            depth.role_transfers as f64 / depth.cases as f64
        );
    }
    println!(
        "  unreachable={}/{}, order-sensitive={}/{}, model entries={}, frozen={}, all planning pre-action={}",
        report.unreachable_correct,
        report.unreachable_total,
        report.order_sensitive_correct,
        report.order_sensitive_total,
        report.permanent_model_entries,
        report.frozen_fingerprint_unchanged,
        report.all_planning_before_action
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn report() -> &'static ComposableModelReport {
        static REPORT: OnceLock<ComposableModelReport> = OnceLock::new();
        REPORT.get_or_init(run_composable_model_experiment)
    }

    fn planning_report() -> &'static PlanningReport {
        static REPORT: OnceLock<PlanningReport> = OnceLock::new();
        REPORT.get_or_init(run_planning_experiment)
    }

    #[test]
    fn d4a_learns_input_role_provenance_without_identity_memory() {
        let report = report();
        assert_eq!(report.permanent_identity_cells, 0);
        assert!(report.model_size_fixed);
        assert_eq!(report.shuffled_confident_models, 0);
    }

    #[test]
    fn d4a_composes_frozen_models_for_unseen_sequences() {
        let report = report();
        assert_eq!(report.exact_predictions, report.total_predictions);
        assert!(report
            .exact_by_sequence_length
            .iter()
            .all(|(_, &(correct, total))| correct == total));
        assert!(report.frozen_fingerprint_unchanged);
    }

    #[test]
    fn d4a_provenance_beats_changed_role_masks() {
        let report = report();
        assert_eq!(
            report.provenance_distinguishable_pairs,
            report.matched_mask_pairs
        );
        assert_eq!(report.mask_distinguishable_pairs, 0);
        assert!(report.mask_baseline_exact < report.total_predictions);
        assert_eq!(report.swap_twice_correct, HELD_OUT_SEEDS);
        assert_eq!(report.order_sensitive_pairs, HELD_OUT_SEEDS);
    }

    #[test]
    fn d4a_work_grows_with_sequence_length_while_model_stays_fixed() {
        let report = report();
        assert!(report.exact_by_sequence_length.contains_key(&1));
        assert!(report.exact_by_sequence_length.contains_key(&8));
        assert!(report.exact_by_sequence_length.contains_key(&16));
        assert!(report.model_applications > report.total_predictions);
        assert_eq!(report.maximum_temporary_roles, ROLE_COUNT);
        assert!(report.passed);
    }

    #[test]
    fn d4b_selects_shortest_sequences_before_acting() {
        let report = planning_report();
        assert!(report.depth_reports.iter().all(|depth| {
            depth.correct == depth.cases
                && depth.real_execution_correct == depth.cases
                && depth.shortest_correct == depth.cases
                && depth.oracle_correct == depth.cases
        }));
        assert!(report.all_planning_before_action);
        assert_eq!(report.training_sequences, 0);
    }

    #[test]
    fn d4b_requires_composition_and_provenance() {
        let report = planning_report();
        assert_eq!(
            report
                .depth_reports
                .iter()
                .find(|depth| depth.required_depth == 1)
                .unwrap()
                .one_step_correct,
            D4B_SEEDS
        );
        assert!(report
            .depth_reports
            .iter()
            .filter(|depth| depth.required_depth > 1)
            .all(|depth| depth.one_step_correct == 0));
        assert!(report
            .depth_reports
            .iter()
            .all(|depth| depth.mask_planner_correct == 0));
        assert_eq!(report.order_sensitive_correct, report.order_sensitive_total);
    }

    #[test]
    fn d4b_reports_unreachable_and_preserves_frozen_models() {
        let report = planning_report();
        assert_eq!(report.unreachable_correct, report.unreachable_total);
        assert!(report.permanent_model_size_flat);
        assert!(report.frozen_fingerprint_unchanged);
    }

    #[test]
    fn d4b_exposes_enumeration_growth_with_depth() {
        let report = planning_report();
        assert!(report
            .depth_reports
            .windows(2)
            .all(|pair| pair[0].candidates_examined < pair[1].candidates_examined));
        assert!(report
            .depth_reports
            .windows(2)
            .all(|pair| pair[0].model_applications < pair[1].model_applications));
        assert!(report.passed);
    }
}
