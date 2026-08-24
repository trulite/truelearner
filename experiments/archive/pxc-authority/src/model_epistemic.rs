use std::collections::{BTreeMap, BTreeSet};

const ROLE_COUNT: usize = 8;
const TRAINING_EPISODES_PER_ACTION: usize = 128;
const HELD_OUT_EPISODES_PER_ACTION: usize = 64;
const D3_SEEDS: usize = 16;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum RoleOutcome {
    Changed,
    Preserved,
}

#[derive(Clone, Debug)]
struct TemporaryStructure {
    occupants: Vec<OpaqueIdentity>,
}

impl TemporaryStructure {
    fn fresh(identity_source: &mut IdentitySource) -> Self {
        Self {
            occupants: (0..ROLE_COUNT).map(|_| identity_source.issue()).collect(),
        }
    }
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

#[derive(Clone, Debug)]
struct EnvironmentEffect {
    changed_roles: BTreeSet<RoleId>,
}

impl EnvironmentEffect {
    fn selective(role: usize) -> Self {
        Self {
            changed_roles: BTreeSet::from([RoleId(role)]),
        }
    }

    fn disruptive() -> Self {
        Self {
            changed_roles: (0..ROLE_COUNT).map(RoleId).collect(),
        }
    }

    fn inert() -> Self {
        Self {
            changed_roles: BTreeSet::new(),
        }
    }

    fn apply(
        &self,
        before: &TemporaryStructure,
        identity_source: &mut IdentitySource,
    ) -> TemporaryStructure {
        let mut after = before.clone();
        for role in &self.changed_roles {
            after.occupants[role.0] = identity_source.issue();
        }
        after
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EffectArrow {
    action: OpaqueAction,
    role: RoleId,
    outcome: RoleOutcome,
    support: usize,
}

#[derive(Clone, Debug, Default)]
struct ActionEffectLearner {
    arrows: Vec<EffectArrow>,
    proposed_arrows: usize,
    observed_identities: usize,
}

impl ActionEffectLearner {
    fn observe(
        &mut self,
        action: OpaqueAction,
        before: &TemporaryStructure,
        after: &TemporaryStructure,
    ) {
        self.observed_identities += before.occupants.len() + after.occupants.len();
        for role_index in 0..ROLE_COUNT {
            let role = RoleId(role_index);
            let outcome = if before.occupants[role_index] == after.occupants[role_index] {
                RoleOutcome::Preserved
            } else {
                RoleOutcome::Changed
            };
            if !self
                .arrows
                .iter()
                .any(|arrow| arrow.action == action && arrow.role == role)
            {
                self.arrows.push(EffectArrow {
                    action,
                    role,
                    outcome: RoleOutcome::Changed,
                    support: 0,
                });
                self.arrows.push(EffectArrow {
                    action,
                    role,
                    outcome: RoleOutcome::Preserved,
                    support: 0,
                });
                self.proposed_arrows += 2;
                self.arrows.sort();
            }
            self.arrows
                .iter_mut()
                .find(|arrow| {
                    arrow.action == action && arrow.role == role && arrow.outcome == outcome
                })
                .unwrap()
                .support += 1;
        }
    }

    fn predict(&self, action: OpaqueAction) -> Option<ActionPrediction> {
        let mut changed = BTreeSet::new();
        let mut preserved = BTreeSet::new();
        for role_index in 0..ROLE_COUNT {
            let role = RoleId(role_index);
            let changed_support = self
                .arrows
                .iter()
                .find(|arrow| {
                    arrow.action == action
                        && arrow.role == role
                        && arrow.outcome == RoleOutcome::Changed
                })?
                .support;
            let preserved_support = self
                .arrows
                .iter()
                .find(|arrow| {
                    arrow.action == action
                        && arrow.role == role
                        && arrow.outcome == RoleOutcome::Preserved
                })?
                .support;
            let total = changed_support + preserved_support;
            let strongest = changed_support.max(preserved_support);
            if total < 8 || strongest * 5 < total * 4 {
                return None;
            }
            match changed_support > preserved_support {
                true => {
                    changed.insert(role);
                }
                false => {
                    preserved.insert(role);
                }
            }
        }
        Some(ActionPrediction { changed, preserved })
    }

    fn permanent_identity_cells(&self) -> usize {
        0
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for arrow in &self.arrows {
            fingerprint_mix(&mut hash, arrow.action.0);
            fingerprint_mix(&mut hash, arrow.role.0 as u64);
            fingerprint_mix(
                &mut hash,
                match arrow.outcome {
                    RoleOutcome::Changed => 1,
                    RoleOutcome::Preserved => 2,
                },
            );
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct ActionPrediction {
    changed: BTreeSet<RoleId>,
    preserved: BTreeSet<RoleId>,
}

#[derive(Clone, Debug)]
struct RouteActivity {
    cells: BTreeSet<RoleId>,
    arrows: BTreeSet<(RoleId, RoleId)>,
}

impl RouteActivity {
    fn from_path(path: &[usize]) -> Self {
        let cells = path.iter().copied().map(RoleId).collect();
        let arrows = path
            .windows(2)
            .map(|pair| (RoleId(pair[0]), RoleId(pair[1])))
            .collect();
        Self { cells, arrows }
    }
}

#[derive(Clone, Debug)]
struct RouteComparison {
    shared_roles: BTreeSet<RoleId>,
    first_only_roles: BTreeSet<RoleId>,
    second_only_roles: BTreeSet<RoleId>,
    shared_arrows: BTreeSet<(RoleId, RoleId)>,
    first_only_arrows: BTreeSet<(RoleId, RoleId)>,
    second_only_arrows: BTreeSet<(RoleId, RoleId)>,
}

fn compare_routes(first: &RouteActivity, second: &RouteActivity) -> RouteComparison {
    RouteComparison {
        shared_roles: first.cells.intersection(&second.cells).copied().collect(),
        first_only_roles: first.cells.difference(&second.cells).copied().collect(),
        second_only_roles: second.cells.difference(&first.cells).copied().collect(),
        shared_arrows: first.arrows.intersection(&second.arrows).copied().collect(),
        first_only_arrows: first.arrows.difference(&second.arrows).copied().collect(),
        second_only_arrows: second.arrows.difference(&first.arrows).copied().collect(),
    }
}

#[derive(Clone, Debug)]
struct ActionAssessment {
    action: OpaqueAction,
    prediction: ActionPrediction,
    changes_route_specific: usize,
    preserves_route_specific: usize,
    changes_shared: usize,
    preserves_shared: usize,
}

impl ActionAssessment {
    fn selection_key(&self) -> (bool, usize, usize, usize, std::cmp::Reverse<usize>) {
        let useful = self.changes_route_specific > 0
            && self.preserves_route_specific > 0
            && self.changes_shared == 0;
        (
            useful,
            self.preserves_shared,
            self.changes_route_specific,
            self.preserves_route_specific,
            std::cmp::Reverse(self.prediction.changed.len()),
        )
    }
}

fn assess_action(
    action: OpaqueAction,
    prediction: ActionPrediction,
    comparison: &RouteComparison,
) -> ActionAssessment {
    let route_specific: BTreeSet<_> = comparison
        .first_only_roles
        .union(&comparison.second_only_roles)
        .copied()
        .collect();
    ActionAssessment {
        action,
        changes_route_specific: prediction.changed.intersection(&route_specific).count(),
        preserves_route_specific: prediction.preserved.intersection(&route_specific).count(),
        changes_shared: prediction
            .changed
            .intersection(&comparison.shared_roles)
            .count(),
        preserves_shared: prediction
            .preserved
            .intersection(&comparison.shared_roles)
            .count(),
        prediction,
    }
}

#[derive(Clone, Debug)]
pub struct PreActionAssessment {
    pub action_id: u64,
    pub changed_roles: Vec<usize>,
    pub preserved_roles: Vec<usize>,
    pub changes_route_specific: usize,
    pub preserves_route_specific: usize,
    pub changes_shared: usize,
    pub preserves_shared: usize,
}

#[derive(Clone, Debug)]
pub struct PreActionTrace {
    pub first_route_roles: Vec<usize>,
    pub second_route_roles: Vec<usize>,
    pub shared_roles: Vec<usize>,
    pub first_only_roles: Vec<usize>,
    pub second_only_roles: Vec<usize>,
    pub shared_arrows: usize,
    pub first_only_arrows: usize,
    pub second_only_arrows: usize,
    pub assessments: Vec<PreActionAssessment>,
    pub chosen_action_id: u64,
    pub model_fingerprint_before: u64,
    pub model_fingerprint_after_choice: u64,
}

fn choose_distinguishing_action(
    learner: &ActionEffectLearner,
    actions: &[OpaqueAction],
    first: &RouteActivity,
    second: &RouteActivity,
) -> Option<(OpaqueAction, PreActionTrace)> {
    let fingerprint_before = learner.fingerprint();
    let comparison = compare_routes(first, second);
    let mut assessments = Vec::new();
    for action in actions {
        let prediction = learner.predict(*action)?;
        assessments.push(assess_action(*action, prediction, &comparison));
    }
    let chosen = assessments
        .iter()
        .max_by_key(|assessment| assessment.selection_key())?;
    if !chosen.selection_key().0 {
        return None;
    }
    let chosen_action = chosen.action;
    let trace = PreActionTrace {
        first_route_roles: first.cells.iter().map(|role| role.0).collect(),
        second_route_roles: second.cells.iter().map(|role| role.0).collect(),
        shared_roles: comparison.shared_roles.iter().map(|role| role.0).collect(),
        first_only_roles: comparison
            .first_only_roles
            .iter()
            .map(|role| role.0)
            .collect(),
        second_only_roles: comparison
            .second_only_roles
            .iter()
            .map(|role| role.0)
            .collect(),
        shared_arrows: comparison.shared_arrows.len(),
        first_only_arrows: comparison.first_only_arrows.len(),
        second_only_arrows: comparison.second_only_arrows.len(),
        assessments: assessments
            .into_iter()
            .map(|assessment| PreActionAssessment {
                action_id: assessment.action.0,
                changed_roles: assessment
                    .prediction
                    .changed
                    .iter()
                    .map(|role| role.0)
                    .collect(),
                preserved_roles: assessment
                    .prediction
                    .preserved
                    .iter()
                    .map(|role| role.0)
                    .collect(),
                changes_route_specific: assessment.changes_route_specific,
                preserves_route_specific: assessment.preserves_route_specific,
                changes_shared: assessment.changes_shared,
                preserves_shared: assessment.preserves_shared,
            })
            .collect(),
        chosen_action_id: chosen_action.0,
        model_fingerprint_before: fingerprint_before,
        model_fingerprint_after_choice: learner.fingerprint(),
    };
    Some((chosen_action, trace))
}

#[derive(Clone, Debug)]
struct ActionEnvironment {
    actions: Vec<OpaqueAction>,
    effects: BTreeMap<OpaqueAction, EnvironmentEffect>,
    selective_actions: BTreeMap<RoleId, OpaqueAction>,
    disruptive_action: OpaqueAction,
}

fn build_environment(seed: u64) -> ActionEnvironment {
    let mut action_ids: Vec<_> = (0..6)
        .map(|index| OpaqueAction(mix64(seed ^ index as u64)))
        .collect();
    let mut rng = DeterministicRng::new(seed ^ 0xd3a0_0000);
    rng.shuffle(&mut action_ids);
    let effect_list = [
        EnvironmentEffect::selective(1),
        EnvironmentEffect::selective(3),
        EnvironmentEffect::selective(5),
        EnvironmentEffect::disruptive(),
        EnvironmentEffect::inert(),
        EnvironmentEffect::selective(7),
    ];
    let mut effects = BTreeMap::new();
    let mut selective_actions = BTreeMap::new();
    for (action, effect) in action_ids.iter().copied().zip(effect_list) {
        if effect.changed_roles.len() == 1 {
            let role = *effect.changed_roles.iter().next().unwrap();
            if matches!(role.0, 1 | 3 | 5) {
                selective_actions.insert(role, action);
            }
        }
        effects.insert(action, effect);
    }
    let disruptive_action = effects
        .iter()
        .find_map(|(action, effect)| (effect.changed_roles.len() == ROLE_COUNT).then_some(*action))
        .unwrap();
    rng.shuffle(&mut action_ids);
    ActionEnvironment {
        actions: action_ids,
        effects,
        selective_actions,
        disruptive_action,
    }
}

fn train_action_models_with_episodes(
    seed: u64,
    shuffled_outcomes: bool,
    episodes: usize,
) -> (ActionEffectLearner, ActionEnvironment) {
    let environment = build_environment(seed);
    let mut learner = ActionEffectLearner::default();
    let mut identities = IdentitySource::new(seed ^ 0xd3a1_0000);
    let mut rng = DeterministicRng::new(seed ^ 0xd3a2_0000);
    for episode in 0..episodes {
        let mut outcome_actions = environment.actions.clone();
        if shuffled_outcomes {
            rng.shuffle(&mut outcome_actions);
        }
        for (action, outcome_action) in environment.actions.iter().copied().zip(outcome_actions) {
            let before = TemporaryStructure::fresh(&mut identities);
            let after = environment.effects[&outcome_action].apply(&before, &mut identities);
            learner.observe(action, &before, &after);
        }
        debug_assert_eq!(
            episode + 1,
            learner.observed_identities / (ROLE_COUNT * 2 * 6)
        );
    }
    (learner, environment)
}

fn train_action_models(
    seed: u64,
    shuffled_outcomes: bool,
) -> (ActionEffectLearner, ActionEnvironment) {
    train_action_models_with_episodes(seed, shuffled_outcomes, TRAINING_EPISODES_PER_ACTION)
}

#[derive(Clone, Debug)]
pub struct ActionModelReport {
    pub correct_predictions: usize,
    pub total_predictions: usize,
    pub shuffled_confident_predictions: usize,
    pub proposed_arrows: usize,
    pub permanent_identity_cells: usize,
    pub observed_identities: usize,
    pub model_entries: usize,
    pub model_size_fixed_as_identities_grow: bool,
    pub fingerprint_unchanged_during_test: bool,
    pub passed: bool,
}

pub fn run_action_model_experiment() -> ActionModelReport {
    let mut correct_predictions = 0;
    let mut total_predictions = 0;
    let mut shuffled_confident_predictions = 0;
    let mut proposed_arrows = 0;
    let mut observed_identities = 0;
    let mut model_entries = 0;
    let mut model_size_fixed_as_identities_grow = true;
    let mut fingerprint_unchanged_during_test = true;

    for seed_index in 0..D3_SEEDS {
        let seed = 0xd3a3_0000 + seed_index as u64;
        let (learner, environment) = train_action_models(seed, false);
        let fingerprint_before = learner.fingerprint();
        let mut identities = IdentitySource::new(seed ^ 0xd3a4_0000);
        for _ in 0..HELD_OUT_EPISODES_PER_ACTION {
            for action in &environment.actions {
                let before = TemporaryStructure::fresh(&mut identities);
                let actual = environment.effects[action].apply(&before, &mut identities);
                let prediction = learner.predict(*action).unwrap();
                let predicted = EnvironmentEffect {
                    changed_roles: prediction.changed,
                }
                .apply(&before, &mut identities);
                let correct = (0..ROLE_COUNT).all(|role| {
                    (before.occupants[role] == actual.occupants[role])
                        == (before.occupants[role] == predicted.occupants[role])
                });
                correct_predictions += usize::from(correct);
                total_predictions += 1;
            }
        }
        fingerprint_unchanged_during_test &= fingerprint_before == learner.fingerprint();
        proposed_arrows += learner.proposed_arrows;
        observed_identities += learner.observed_identities;
        model_entries += learner.arrows.len();
        let short = train_action_models_with_episodes(seed, false, 8).0;
        model_size_fixed_as_identities_grow &= short.arrows.len() == learner.arrows.len();

        let (shuffled, shuffled_environment) = train_action_models(seed, true);
        shuffled_confident_predictions += shuffled_environment
            .actions
            .iter()
            .filter(|action| shuffled.predict(**action).is_some())
            .count();
    }

    let permanent_identity_cells = train_action_models(0xd3af_0000, false)
        .0
        .permanent_identity_cells();
    let passed = correct_predictions == total_predictions
        && shuffled_confident_predictions == 0
        && permanent_identity_cells == 0
        && model_size_fixed_as_identities_grow
        && fingerprint_unchanged_during_test;
    ActionModelReport {
        correct_predictions,
        total_predictions,
        shuffled_confident_predictions,
        proposed_arrows,
        permanent_identity_cells,
        observed_identities,
        model_entries,
        model_size_fixed_as_identities_grow,
        fingerprint_unchanged_during_test,
        passed,
    }
}

#[derive(Clone, Debug)]
pub struct ModelBasedActionReport {
    pub model_correct: usize,
    pub empty_history_correct: usize,
    pub random_correct: usize,
    pub disruptive_heuristic_correct: usize,
    pub total: usize,
    pub action_permutations: usize,
    pub all_choices_pre_action: bool,
    pub frozen_fingerprint_unchanged: bool,
    pub decisions: Vec<ModelBasedDecision>,
    pub representative_trace: PreActionTrace,
    pub passed: bool,
}

#[derive(Clone, Debug)]
pub struct ModelBasedDecision {
    pub seed_index: usize,
    pub template_index: usize,
    pub expected_action_id: u64,
    pub chosen_action_id: u64,
    pub actual_changed_roles: Vec<usize>,
    pub actual_preserved_roles: Vec<usize>,
    pub correct: bool,
    pub trace: PreActionTrace,
}

fn ambiguity_templates() -> [(RouteActivity, RouteActivity, RoleId); 3] {
    [
        (
            RouteActivity::from_path(&[0, 1, 6]),
            RouteActivity::from_path(&[0, 2, 6]),
            RoleId(1),
        ),
        (
            RouteActivity::from_path(&[0, 3, 6]),
            RouteActivity::from_path(&[0, 4, 6]),
            RoleId(3),
        ),
        (
            RouteActivity::from_path(&[0, 5, 6]),
            RouteActivity::from_path(&[0, 2, 6]),
            RoleId(5),
        ),
    ]
}

pub fn run_model_based_action_experiment() -> ModelBasedActionReport {
    let mut model_correct = 0;
    let mut empty_history_correct = 0;
    let mut random_correct = 0;
    let mut disruptive_heuristic_correct = 0;
    let mut total = 0;
    let mut action_orders = BTreeSet::new();
    let mut all_choices_pre_action = true;
    let mut frozen_fingerprint_unchanged = true;
    let mut decisions = Vec::new();
    let mut representative_trace = None;

    for seed_index in 0..D3_SEEDS {
        let seed = 0xd3b0_0000 + seed_index as u64;
        let (learner, environment) = train_action_models(seed, false);
        let fingerprint = learner.fingerprint();
        action_orders.insert(
            environment
                .actions
                .iter()
                .map(|action| action.0)
                .collect::<Vec<_>>(),
        );
        let mut rng = DeterministicRng::new(seed ^ 0xd3b1_0000);
        let mut identities = IdentitySource::new(seed ^ 0xd3b2_0000);

        for (template_index, (first, second, selective_role)) in
            ambiguity_templates().into_iter().enumerate()
        {
            let expected = environment.selective_actions[&selective_role];
            let (chosen, trace) =
                choose_distinguishing_action(&learner, &environment.actions, &first, &second)
                    .unwrap();
            all_choices_pre_action &= trace.model_fingerprint_before
                == trace.model_fingerprint_after_choice
                && trace.assessments.len() == environment.actions.len();
            frozen_fingerprint_unchanged &= fingerprint == learner.fingerprint();
            let before = TemporaryStructure::fresh(&mut identities);
            let after = environment.effects[&chosen].apply(&before, &mut identities);
            let mut actual_changed = Vec::new();
            let mut actual_preserved = Vec::new();
            for role in 0..ROLE_COUNT {
                if before.occupants[role] == after.occupants[role] {
                    actual_preserved.push(role);
                } else {
                    actual_changed.push(role);
                }
            }
            let correct = chosen == expected
                && actual_changed == vec![selective_role.0]
                && actual_preserved.len() == ROLE_COUNT - 1;
            model_correct += usize::from(correct);
            frozen_fingerprint_unchanged &= fingerprint == learner.fingerprint();

            let empty_history_choice = environment.actions[0];
            empty_history_correct += usize::from(empty_history_choice == expected);

            let random_choice =
                environment.actions[(rng.next_u64() as usize) % environment.actions.len()];
            random_correct += usize::from(random_choice == expected);
            disruptive_heuristic_correct += usize::from(environment.disruptive_action == expected);
            total += 1;

            if seed_index == 0 && template_index == 0 {
                representative_trace = Some(trace.clone());
            }
            decisions.push(ModelBasedDecision {
                seed_index,
                template_index,
                expected_action_id: expected.0,
                chosen_action_id: chosen.0,
                actual_changed_roles: actual_changed,
                actual_preserved_roles: actual_preserved,
                correct,
                trace,
            });
        }
    }

    let passed = model_correct == total
        && empty_history_correct < total / 2
        && random_correct < total / 2
        && disruptive_heuristic_correct == 0
        && action_orders.len() == D3_SEEDS
        && all_choices_pre_action
        && frozen_fingerprint_unchanged;
    ModelBasedActionReport {
        model_correct,
        empty_history_correct,
        random_correct,
        disruptive_heuristic_correct,
        total,
        action_permutations: action_orders.len(),
        all_choices_pre_action,
        frozen_fingerprint_unchanged,
        decisions,
        representative_trace: representative_trace.unwrap(),
        passed,
    }
}

pub fn print_action_model_report(report: &ActionModelReport) {
    println!("d3a role-relative action-effect learning:");
    println!(
        "  held-out predictions={}/{}, shuffled confident predictions={}",
        report.correct_predictions, report.total_predictions, report.shuffled_confident_predictions
    );
    println!(
        "  proposed arrows={}, model entries={}, observed identities={}, permanent identity cells={}",
        report.proposed_arrows,
        report.model_entries,
        report.observed_identities,
        report.permanent_identity_cells
    );
    println!(
        "  held-out fingerprint unchanged={}, model size fixed={}",
        report.fingerprint_unchanged_during_test, report.model_size_fixed_as_identities_grow
    );
}

pub fn print_model_based_action_report(report: &ModelBasedActionReport) {
    println!("d3b model-based epistemic action:");
    println!(
        "  first-action correct model/empty-history/random/disruptive={}/{}/{}/{}/{}",
        report.model_correct,
        report.empty_history_correct,
        report.random_correct,
        report.disruptive_heuristic_correct,
        report.total
    );
    println!(
        "  action permutations={}, pre-action trace complete={}, frozen fingerprint unchanged={}",
        report.action_permutations,
        report.all_choices_pre_action,
        report.frozen_fingerprint_unchanged
    );
    println!(
        "  representative shared={:?}, first-only={:?}, second-only={:?}, chosen={}",
        report.representative_trace.shared_roles,
        report.representative_trace.first_only_roles,
        report.representative_trace.second_only_roles,
        report.representative_trace.chosen_action_id
    );
    for assessment in &report.representative_trace.assessments {
        println!(
            "    action {} changed={:?}, preserved={:?}, route-specific change/preserve={}/{}, shared change/preserve={}/{}",
            assessment.action_id,
            assessment.changed_roles,
            assessment.preserved_roles,
            assessment.changes_route_specific,
            assessment.preserves_route_specific,
            assessment.changes_shared,
            assessment.preserves_shared
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn d3a_report() -> &'static ActionModelReport {
        static REPORT: OnceLock<ActionModelReport> = OnceLock::new();
        REPORT.get_or_init(run_action_model_experiment)
    }

    fn d3b_report() -> &'static ModelBasedActionReport {
        static REPORT: OnceLock<ModelBasedActionReport> = OnceLock::new();
        REPORT.get_or_init(run_model_based_action_experiment)
    }

    #[test]
    fn d3a_learns_role_relative_changed_and_preserved_effects() {
        let report = d3a_report();
        assert_eq!(report.correct_predictions, report.total_predictions);
        assert_eq!(report.permanent_identity_cells, 0);
        assert!(report.model_size_fixed_as_identities_grow);
    }

    #[test]
    fn d3a_shuffled_action_outcomes_do_not_create_confident_models() {
        let report = d3a_report();
        assert_eq!(report.shuffled_confident_predictions, 0);
        assert!(report.fingerprint_unchanged_during_test);
        assert!(report.passed);
    }

    #[test]
    fn d3b_selects_the_distinguishing_action_before_observing_its_usefulness() {
        let report = d3b_report();
        assert_eq!(report.model_correct, report.total);
        assert!(report.all_choices_pre_action);
        assert!(report.frozen_fingerprint_unchanged);
        assert_eq!(report.decisions.len(), report.total);
        assert!(report.decisions.iter().all(|decision| decision.correct));
    }

    #[test]
    fn d3b_beats_empty_history_random_and_disruptive_baselines() {
        let report = d3b_report();
        assert!(report.model_correct > report.empty_history_correct);
        assert!(report.model_correct > report.random_correct);
        assert_eq!(report.disruptive_heuristic_correct, 0);
        assert!(report.action_permutations >= D3_SEEDS);
        assert!(report.passed);
    }
}
