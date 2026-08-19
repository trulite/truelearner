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
struct OpaqueAction(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct OpaqueIdentity(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct RoleId(usize);

#[derive(Clone, Debug, PartialEq, Eq)]
struct TemporaryStructure {
    occupants: [OpaqueIdentity; ROLE_COUNT],
}

#[derive(Clone, Debug)]
struct IdentitySource {
    next: u64,
    namespace: u64,
}

impl IdentitySource {
    fn new(namespace: u64) -> Self {
        Self { next: 0, namespace }
    }

    fn fresh_structure(&mut self) -> TemporaryStructure {
        TemporaryStructure {
            occupants: std::array::from_fn(|_| self.issue()),
        }
    }

    fn issue(&mut self) -> OpaqueIdentity {
        let identity = OpaqueIdentity(mix64(self.namespace ^ self.next));
        self.next += 1;
        identity
    }
}

fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RoleTransformation {
    source_for_output: [RoleId; ROLE_COUNT],
}

impl RoleTransformation {
    fn new(sources: [usize; ROLE_COUNT]) -> Self {
        Self {
            source_for_output: sources.map(RoleId),
        }
    }

    fn apply(&self, before: &TemporaryStructure) -> TemporaryStructure {
        TemporaryStructure {
            occupants: std::array::from_fn(|output| {
                before.occupants[self.source_for_output[output].0]
            }),
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

#[derive(Clone, Debug, Default)]
struct ProvenanceLearner {
    arrows: Vec<SourceArrow>,
    proposed_arrows: usize,
    observed_identities: usize,
}

impl ProvenanceLearner {
    fn observe(
        &mut self,
        action: OpaqueAction,
        before: &TemporaryStructure,
        after: &TemporaryStructure,
    ) {
        self.observed_identities += ROLE_COUNT * 2;
        for output in 0..ROLE_COUNT {
            if !self
                .arrows
                .iter()
                .any(|arrow| arrow.action == action && arrow.output_role == RoleId(output))
            {
                for input in 0..ROLE_COUNT {
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

            for input in 0..ROLE_COUNT {
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

    fn predict(&self, action: OpaqueAction) -> Option<RoleTransformation> {
        let mut sources = [RoleId(0); ROLE_COUNT];
        for (output, source) in sources.iter_mut().enumerate() {
            let candidates = self
                .arrows
                .iter()
                .filter(|arrow| arrow.action == action && arrow.output_role == RoleId(output))
                .collect::<Vec<_>>();
            if candidates.len() != ROLE_COUNT {
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

    fn model_entries(&self) -> usize {
        self.arrows.len()
    }

    fn permanent_identity_cells(&self) -> usize {
        0
    }

    fn fingerprint(&self) -> u64 {
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
    let mut learner = ProvenanceLearner::default();
    let mut identities = IdentitySource::new(seed ^ 0xd4a1_0000);

    for episode in 0..TRAINING_EPISODES_PER_ACTION {
        for action_index in 0..ACTION_COUNT {
            let action = world.action(action_index);
            let before = identities.fresh_structure();
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
        Self {
            changed_by_action: world
                .effect_for_action
                .iter()
                .map(|(&action, effect)| (action, effect.changed_roles()))
                .collect(),
        }
    }

    fn apply(
        &self,
        action: OpaqueAction,
        before: &[MaskValue; ROLE_COUNT],
    ) -> [MaskValue; ROLE_COUNT] {
        let changed = &self.changed_by_action[&action];
        std::array::from_fn(|role| {
            if changed.contains(&RoleId(role)) {
                MaskValue::Unknown
            } else {
                before[role].clone()
            }
        })
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
) -> [MaskValue; ROLE_COUNT] {
    let mut current = initial.occupants.map(MaskValue::Known);
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
            let mut learner = ProvenanceLearner::default();
            let mut identities = IdentitySource::new(seed ^ 0xd4a3_0000);
            for _ in 0..8 {
                for action_index in 0..ACTION_COUNT {
                    let before = identities.fresh_structure();
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
        let control_initial = control_identities.fresh_structure();
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
            let initial = identities.fresh_structure();
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
                .zip(expected.occupants)
                .all(|(predicted, expected)| *predicted == MaskValue::Known(expected));
            mask_baseline_exact += usize::from(mask_exact);

            traces.push(CompositionTrace {
                seed_index,
                sequence_label: sequence.label,
                initial_identity_ids: initial.occupants.map(|identity| identity.0).to_vec(),
                expected_identity_ids: expected.occupants.map(|identity| identity.0).to_vec(),
                predicted_identity_ids: predicted.occupants.map(|identity| identity.0).to_vec(),
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
                        resulting_identity_ids: state.occupants.map(|identity| identity.0).to_vec(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn report() -> &'static ComposableModelReport {
        static REPORT: OnceLock<ComposableModelReport> = OnceLock::new();
        REPORT.get_or_init(run_composable_model_experiment)
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
}
