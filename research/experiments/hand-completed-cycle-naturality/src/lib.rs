#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    EffectComposition, OutputChoiceResolutionEvidence, ReflectedHandProtocolEvidence,
    run_reflected_hand_bounded,
};
use serde::Serialize;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{CompletedCycleState, JunctionId, LearnerId, OutputAdmission, Protocol};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    ChoiceTraceInertness,
    CompletedCycleFirstArrowChange,
}

impl Arm {
    pub const ALL: [Self; 2] = [
        Self::ChoiceTraceInertness,
        Self::CompletedCycleFirstArrowChange,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::ChoiceTraceInertness => "choice-trace-inertness",
            Self::CompletedCycleFirstArrowChange => "completed-cycle-first-arrow-change",
        }
    }
}

impl FromStr for Arm {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|arm| arm.id() == value)
            .ok_or(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct HandSummary {
    protocol: Protocol,
    effect_composition: EffectComposition,
    actual_position_changes: usize,
    opposing_output_steps: usize,
    final_position: i16,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    completed_cycle_admissions: usize,
    cross_view_admissions: usize,
    output_choice_resolutions: usize,
    propagation_budget_exhaustions: u64,
    stopped: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
}

impl HandSummary {
    fn exact_parent(&self) -> bool {
        self.protocol == Protocol::RecursiveLearnerCompletedCycle
            && self.effect_composition == EffectComposition::Batched
            && self.actual_position_changes == 12
            && self.opposing_output_steps == 4
            && self.final_position == -2
            && !self.reached_lower
            && !self.reached_upper
            && !self.escaped_lower
            && !self.escaped_upper
            && self.completed_cycle_admissions == 9
            && self.cross_view_admissions == 2
            && self.output_choice_resolutions > 0
            && self.propagation_budget_exhaustions == 0
            && !self.stopped
            && self.exact_replay
            && self.naturally_quiescent
    }
}

fn summarize(hand: &ReflectedHandProtocolEvidence) -> HandSummary {
    let completed = hand
        .trajectory
        .iter()
        .flat_map(|step| &step.completed_cycle_continuations)
        .collect::<Vec<_>>();
    HandSummary {
        protocol: hand.protocol,
        effect_composition: hand.effect_composition,
        actual_position_changes: hand.actual_position_changes,
        opposing_output_steps: hand
            .trajectory
            .iter()
            .filter(|step| {
                step.emitted_outputs.contains(&20_000) && step.emitted_outputs.contains(&20_001)
            })
            .count(),
        final_position: hand.final_position,
        reached_lower: hand.reached_lower,
        reached_upper: hand.reached_upper,
        escaped_lower: hand.escaped_lower,
        escaped_upper: hand.escaped_upper,
        completed_cycle_admissions: completed.iter().filter(|effect| effect.admitted).count(),
        cross_view_admissions: completed
            .iter()
            .filter(|effect| effect.admitted && effect.crosses_ownership_view)
            .count(),
        output_choice_resolutions: hand
            .trajectory
            .iter()
            .map(|step| step.output_choice_resolutions.len())
            .sum(),
        propagation_budget_exhaustions: hand
            .trajectory
            .iter()
            .map(|step| step.propagation_budget_exhaustions)
            .sum(),
        stopped: hand.stopped,
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ChoicePoint {
    ordinal: usize,
    hand_step: usize,
    tick: i64,
    phase: i32,
    admitted: Vec<OutputAdmission>,
    admission_basis: truelearner_core::OutputChoiceBasis,
    completed_cycle_state: CompletedCycleState,
    crosses_ownership_view: bool,
}

impl ChoicePoint {
    fn unique(&self) -> Option<OutputAdmission> {
        (self.admitted.len() == 1).then(|| self.admitted[0])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
enum CategoricalFailureKind {
    MultiTargetRelation,
    NonNaturalOwnershipChange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct CategoricalFailure {
    kind: CategoricalFailureKind,
    previous_ordinal: Option<usize>,
    current_ordinal: usize,
    previous_target: Option<JunctionId>,
    current_targets: Vec<JunctionId>,
    previous_owner: Option<LearnerId>,
    current_owners: Vec<Option<LearnerId>>,
    admission_basis: truelearner_core::OutputChoiceBasis,
    completed_cycle_state: CompletedCycleState,
    tick: i64,
    phase: i32,
    hand_step: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
struct NaturalityAnalysis {
    ordered_choices: Vec<ChoicePoint>,
    multi_target_admissions: usize,
    ownership_changes_preserving_target: usize,
    ownership_changes_changing_target: usize,
    target_changes_without_ownership_change: usize,
    first_multi_target: Option<CategoricalFailure>,
    first_non_natural: Option<CategoricalFailure>,
    first_failure: Option<CategoricalFailure>,
}

fn analyze_choices<'a>(
    choices: impl Iterator<Item = (usize, &'a OutputChoiceResolutionEvidence)>,
) -> NaturalityAnalysis {
    let ordered_choices = choices
        .enumerate()
        .map(|(ordinal, (hand_step, choice))| ChoicePoint {
            ordinal,
            hand_step,
            tick: choice.tick,
            phase: choice.phase,
            admitted: choice.admitted.clone(),
            admission_basis: choice.admission_basis,
            completed_cycle_state: choice.completed_cycle_state,
            crosses_ownership_view: choice.crosses_ownership_view,
        })
        .collect::<Vec<_>>();
    let mut analysis = NaturalityAnalysis {
        ordered_choices: ordered_choices.clone(),
        ..NaturalityAnalysis::default()
    };
    let mut previous: Option<&ChoicePoint> = None;
    for current in &ordered_choices {
        let Some(current_admission) = current.unique() else {
            analysis.multi_target_admissions += 1;
            let failure = CategoricalFailure {
                kind: CategoricalFailureKind::MultiTargetRelation,
                previous_ordinal: previous.map(|point| point.ordinal),
                current_ordinal: current.ordinal,
                previous_target: previous
                    .and_then(ChoicePoint::unique)
                    .map(|item| item.target),
                current_targets: current.admitted.iter().map(|item| item.target).collect(),
                previous_owner: previous
                    .and_then(ChoicePoint::unique)
                    .and_then(|item| item.owner),
                current_owners: current.admitted.iter().map(|item| item.owner).collect(),
                admission_basis: current.admission_basis,
                completed_cycle_state: current.completed_cycle_state,
                tick: current.tick,
                phase: current.phase,
                hand_step: current.hand_step,
            };
            analysis.first_multi_target.get_or_insert(failure.clone());
            analysis.first_failure.get_or_insert(failure);
            previous = None;
            continue;
        };
        if let Some(previous_point) = previous {
            let previous_admission = previous_point
                .unique()
                .expect("previous choice is functional");
            let owner_changed = previous_admission.owner != current_admission.owner;
            let target_changed = previous_admission.target != current_admission.target;
            match (owner_changed, target_changed) {
                (true, false) => analysis.ownership_changes_preserving_target += 1,
                (true, true) => {
                    analysis.ownership_changes_changing_target += 1;
                    let failure = CategoricalFailure {
                        kind: CategoricalFailureKind::NonNaturalOwnershipChange,
                        previous_ordinal: Some(previous_point.ordinal),
                        current_ordinal: current.ordinal,
                        previous_target: Some(previous_admission.target),
                        current_targets: vec![current_admission.target],
                        previous_owner: previous_admission.owner,
                        current_owners: vec![current_admission.owner],
                        admission_basis: current.admission_basis,
                        completed_cycle_state: current.completed_cycle_state,
                        tick: current.tick,
                        phase: current.phase,
                        hand_step: current.hand_step,
                    };
                    analysis.first_non_natural.get_or_insert(failure.clone());
                    analysis.first_failure.get_or_insert(failure);
                }
                (false, true) => analysis.target_changes_without_ownership_change += 1,
                (false, false) => {}
            }
        }
        previous = Some(current);
    }
    analysis
}

#[derive(Clone, Debug)]
struct Evidence {
    summary: HandSummary,
    analysis: NaturalityAnalysis,
}

fn measure() -> Evidence {
    let hand = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerCompletedCycle,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let analysis = analyze_choices(hand.trajectory.iter().flat_map(|step| {
        step.output_choice_resolutions
            .iter()
            .map(move |choice| (step.index, choice))
    }));
    Evidence {
        summary: summarize(&hand),
        analysis,
    }
}

static EVIDENCE: OnceLock<Evidence> = OnceLock::new();

fn evidence() -> &'static Evidence {
    EVIDENCE.get_or_init(measure)
}

#[derive(Clone, Debug, Serialize)]
pub struct ProbeResult {
    schema: &'static str,
    pub arm: &'static str,
    pub outcome: &'static str,
    pub observations: serde_json::Value,
    pub falsifier: Option<String>,
    pub exact_replay: bool,
    pub naturally_quiescent: bool,
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    let survived = match arm {
        Arm::ChoiceTraceInertness => evidence.summary.exact_parent(),
        Arm::CompletedCycleFirstArrowChange => {
            evidence.summary.exact_parent() && evidence.analysis.first_failure.is_some()
        }
    };
    let falsifier = match arm {
        Arm::ChoiceTraceInertness => "the diagnostic changed the frozen hand summary or integrity",
        Arm::CompletedCycleFirstArrowChange => {
            "no first multi-target or non-natural single-target failure was observable"
        }
    };
    ProbeResult {
        schema: "hand-completed-cycle-naturality/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations: serde_json::json!({
            "frozen_parent_summary": evidence.summary,
            "naturality": evidence.analysis,
        }),
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: evidence.summary.exact_replay,
        naturally_quiescent: evidence.summary.naturally_quiescent,
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use truelearner_core::OutputChoiceBasis;

    fn choice(tick: i64, target: u64, owner: Option<u64>) -> OutputChoiceResolutionEvidence {
        OutputChoiceResolutionEvidence {
            tick,
            phase: 0,
            ordinary_target: JunctionId(target),
            current_transition_target: None,
            coherent_effect_target: None,
            completed_cycle_target: Some(JunctionId(target)),
            computed_winner_target: JunctionId(target),
            admitted: vec![OutputAdmission {
                target: JunctionId(target),
                owner: owner.map(LearnerId),
            }],
            computed_winner_basis: OutputChoiceBasis::CompletedCycle,
            admission_basis: OutputChoiceBasis::CompletedCycle,
            completed_cycle_state: CompletedCycleState::Unique,
            crosses_ownership_view: owner.is_some(),
        }
    }

    #[test]
    fn pure_fold_distinguishes_commuting_and_non_natural_owner_changes() {
        let choices = [
            choice(1, 10, None),
            choice(2, 10, Some(1)),
            choice(3, 11, Some(2)),
        ];
        let observed = analyze_choices(choices.iter().enumerate());
        assert_eq!(observed.ownership_changes_preserving_target, 1);
        assert_eq!(observed.ownership_changes_changing_target, 1);
        assert_eq!(
            observed.first_failure.as_ref().map(|failure| failure.kind),
            Some(CategoricalFailureKind::NonNaturalOwnershipChange)
        );
    }

    #[test]
    fn pure_fold_reports_multi_target_admission_before_naturality() {
        let first = choice(1, 10, None);
        let mut relation = choice(2, 10, Some(1));
        relation.admitted.push(OutputAdmission {
            target: JunctionId(11),
            owner: Some(LearnerId(2)),
        });
        relation.admission_basis = OutputChoiceBasis::RecentCohort;
        let choices = [first, relation];
        let observed = analyze_choices(choices.iter().enumerate());
        assert_eq!(
            observed.first_failure.as_ref().map(|failure| failure.kind),
            Some(CategoricalFailureKind::MultiTargetRelation)
        );
    }
}
