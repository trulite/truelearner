#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    ReflectedHandProtocolEvidence, ReflectedHandStepEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{CandidateOwnership, JunctionId, Protocol};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    UpperBoundaryOppositeCandidateLocalization,
    PreBoundaryMatchedControl,
    CompleteLocalization,
}

impl Arm {
    pub const ALL: [Self; 3] = [
        Self::UpperBoundaryOppositeCandidateLocalization,
        Self::PreBoundaryMatchedControl,
        Self::CompleteLocalization,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::UpperBoundaryOppositeCandidateLocalization => {
                "upper-boundary-opposite-candidate-localization"
            }
            Self::PreBoundaryMatchedControl => "pre-boundary-matched-control",
            Self::CompleteLocalization => "complete-localization",
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct MotorStageEvidence {
    pub target: Option<JunctionId>,
    pub physical: u64,
    pub path_firings: usize,
    pub complete_path_firings: usize,
    pub carried_origins: BTreeSet<u64>,
    pub origin_owners: BTreeSet<Option<u64>>,
    pub path_owners: BTreeSet<Option<u64>>,
    pub candidate_evaluations: usize,
    pub candidates_with_paths: usize,
    pub executable_candidates: usize,
    pub maximum_admitted_drive: i64,
    pub maximum_projected_drive: i64,
    pub minimum_threshold: Option<i64>,
    pub consequential_candidates: usize,
    pub selection_checks: usize,
    pub maximum_executable_origin_groups: u32,
    pub selected_origins: BTreeSet<u64>,
    pub selected_path_inputs: u32,
    pub selection_evaluations: usize,
    pub admitted_selections: usize,
    pub rejected_selections: usize,
    pub selection_consequence_ticks: BTreeSet<i64>,
    pub emitted_outputs: usize,
}

impl MotorStageEvidence {
    fn owner_key(owner: Option<truelearner_core::LearnerId>) -> Option<u64> {
        owner.map(|owner| owner.0)
    }

    fn stage(&self) -> &'static str {
        if self.path_firings == 0 || self.candidates_with_paths == 0 {
            "no-live-path"
        } else if self.maximum_projected_drive < self.minimum_threshold.unwrap_or(i64::MAX) {
            "subthreshold"
        } else if self.executable_candidates == 0 {
            "candidate-not-executable"
        } else if self.selection_evaluations > 0 && self.admitted_selections == 0 {
            "rejected-by-output-competition"
        } else if self.admitted_selections > 0 && self.emitted_outputs == 0 {
            "admitted-but-not-emitted"
        } else if self.emitted_outputs == 0 {
            "executable-without-local-competition"
        } else {
            "emitted"
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StepEvidence {
    pub index: usize,
    pub position_before: i16,
    pub position_after: i16,
    pub direction: i8,
    pub delivered_surface_count: usize,
    pub motors: Vec<MotorStageEvidence>,
    pub stage_by_physical: BTreeMap<u64, &'static str>,
    pub admitted_return_origins: BTreeSet<u64>,
    pub admitted_return_count: usize,
    pub consequence_write_ticks: BTreeSet<i64>,
    pub consequence_write_junctions: BTreeSet<JunctionId>,
    pub consequence_born_eligible_origins: BTreeSet<u64>,
    pub consequence_born_ineligible_origins: BTreeSet<u64>,
    pub propagation_budget_exhaustions: u64,
}

fn motor_targets(step: &ReflectedHandStepEvidence) -> BTreeMap<JunctionId, u64> {
    step.drive_provenance
        .iter()
        .filter(|drive| drive.is_motor)
        .map(|drive| (drive.target, drive.target_physical))
        .collect()
}

fn summarize(step: &ReflectedHandStepEvidence) -> StepEvidence {
    let targets = motor_targets(step);
    let mut motors = targets
        .iter()
        .map(|(target, physical)| {
            let mut evidence = MotorStageEvidence {
                target: Some(*target),
                physical: *physical,
                ..MotorStageEvidence::default()
            };
            for drive in step
                .drive_provenance
                .iter()
                .filter(|drive| drive.target == *target)
            {
                evidence.path_firings += usize::from(drive.link.is_some());
                evidence.complete_path_firings += usize::from(drive.completes_path);
                evidence.carried_origins.insert(drive.carried_origin);
                evidence
                    .origin_owners
                    .insert(MotorStageEvidence::owner_key(drive.origin_owner));
                evidence
                    .path_owners
                    .insert(MotorStageEvidence::owner_key(drive.path_owner));
            }
            for candidate in step
                .output_candidates
                .iter()
                .filter(|candidate| candidate.target == *target)
            {
                evidence.candidate_evaluations += 1;
                evidence.candidates_with_paths += usize::from(candidate.path_inputs > 0);
                evidence.executable_candidates += usize::from(candidate.executable);
                evidence.maximum_admitted_drive = evidence
                    .maximum_admitted_drive
                    .max(candidate.admitted_drive);
                evidence.maximum_projected_drive = evidence
                    .maximum_projected_drive
                    .max(candidate.projected_drive);
                evidence.minimum_threshold = Some(
                    evidence
                        .minimum_threshold
                        .map_or(candidate.threshold, |old| old.min(candidate.threshold)),
                );
                evidence.consequential_candidates +=
                    usize::from(candidate.consequence_tick.is_some());
            }
            for selection in step
                .causal_origin_selection
                .iter()
                .filter(|selection| selection.target == *target)
            {
                evidence.selection_checks += 1;
                evidence.maximum_executable_origin_groups = evidence
                    .maximum_executable_origin_groups
                    .max(selection.executable_groups);
                evidence.selected_origins.extend(selection.selected_origin);
                evidence.selected_path_inputs = evidence
                    .selected_path_inputs
                    .max(selection.selected_path_inputs);
                if let Some(CandidateOwnership::Owned(owner)) = selection.selected_ownership {
                    evidence
                        .path_owners
                        .insert(MotorStageEvidence::owner_key(Some(owner)));
                }
            }
            for selection in step
                .candidate_selection
                .iter()
                .filter(|selection| selection.target == *target)
            {
                evidence.selection_evaluations += 1;
                evidence.admitted_selections += usize::from(selection.admitted);
                evidence.rejected_selections += usize::from(!selection.admitted);
                evidence
                    .selection_consequence_ticks
                    .extend(selection.consequence_tick);
            }
            evidence.emitted_outputs = step
                .emitted_outputs
                .iter()
                .filter(|output| **output == *physical)
                .count();
            evidence
        })
        .collect::<Vec<_>>();
    motors.sort_by_key(|motor| motor.physical);
    let stage_by_physical = motors
        .iter()
        .map(|motor| (motor.physical, motor.stage()))
        .collect();
    let admitted_return_origins = step
        .return_origins
        .iter()
        .filter(|origin| origin.decision.starts_with("admitted"))
        .map(|origin| origin.origin_physical)
        .collect();
    let admitted_return_count = step
        .return_origins
        .iter()
        .filter(|origin| origin.decision.starts_with("admitted"))
        .count();
    StepEvidence {
        index: step.index,
        position_before: step.position_before,
        position_after: step.position_after,
        direction: step.direction,
        delivered_surface_count: step.delivered_surface_count,
        motors,
        stage_by_physical,
        admitted_return_origins,
        admitted_return_count,
        consequence_write_ticks: step
            .consequence_writes
            .iter()
            .map(|write| write.tick)
            .collect(),
        consequence_write_junctions: step
            .consequence_writes
            .iter()
            .map(|write| write.junction)
            .collect(),
        consequence_born_eligible_origins: step
            .closure_eligibility
            .iter()
            .filter(|eligibility| eligibility.eligible)
            .map(|eligibility| eligibility.origin_physical)
            .collect(),
        consequence_born_ineligible_origins: step
            .closure_eligibility
            .iter()
            .filter(|eligibility| !eligibility.eligible)
            .map(|eligibility| eligibility.origin_physical)
            .collect(),
        propagation_budget_exhaustions: step.propagation_budget_exhaustions,
    }
}

#[derive(Clone, Debug)]
struct Evidence {
    hand: ReflectedHandProtocolEvidence,
    last_moving: StepEvidence,
    first_clamped: StepEvidence,
    first_no_consequence_clamp: Option<StepEvidence>,
    post_clamp: StepEvidence,
    clamped_steps: Vec<StepEvidence>,
}

fn measure() -> Evidence {
    let hand = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerBoundaryEffectTerminal,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let last_moving_index = hand
        .trajectory
        .iter()
        .rposition(|step| step.position_before != step.position_after)
        .expect("terminal hand reaches a boundary");
    let first_clamped_index = last_moving_index.saturating_add(1);
    let post_clamp_index = first_clamped_index.saturating_add(1);
    let first_no_consequence_clamp = hand.trajectory[first_clamped_index..]
        .iter()
        .find(|step| {
            step.position_before == step.position_after
                && step.candidate_selection.iter().any(|selection| {
                    selection.is_motor && selection.admitted && selection.consequence_tick.is_none()
                })
        })
        .map(summarize);
    let clamped_steps = hand.trajectory[first_clamped_index..]
        .iter()
        .map(summarize)
        .collect();
    Evidence {
        last_moving: summarize(&hand.trajectory[last_moving_index]),
        first_clamped: summarize(&hand.trajectory[first_clamped_index]),
        first_no_consequence_clamp,
        post_clamp: summarize(&hand.trajectory[post_clamp_index]),
        clamped_steps,
        hand,
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

fn result(
    arm: Arm,
    survived: bool,
    observations: serde_json::Value,
    falsifier: &'static str,
    evidence: &Evidence,
) -> ProbeResult {
    ProbeResult {
        schema: "hand-upper-boundary-release-localization/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations,
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: evidence.hand.exact_replay,
        naturally_quiescent: evidence.hand.naturally_quiescent,
    }
}

fn complete_trace(evidence: &Evidence) -> bool {
    [&evidence.last_moving, &evidence.first_clamped]
        .into_iter()
        .all(|step| {
            step.motors.len() == 2 && step.motors.iter().all(|motor| motor.target.is_some())
        })
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    match arm {
        Arm::UpperBoundaryOppositeCandidateLocalization => {
            let emitted = evidence
                .first_clamped
                .motors
                .iter()
                .filter(|motor| motor.emitted_outputs > 0)
                .count();
            let silent = evidence
                .first_clamped
                .motors
                .iter()
                .filter(|motor| motor.emitted_outputs == 0)
                .count();
            let survived = complete_trace(evidence)
                && emitted == 1
                && silent == 1
                && evidence.first_clamped.propagation_budget_exhaustions == 0
                && evidence.hand.exact_replay
                && evidence.hand.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({
                    "first_clamped": evidence.first_clamped,
                    "emitted_motor_count": emitted,
                    "silent_motor_count": silent,
                }),
                "the existing trace did not distinguish one emitted and one silent motor at first clamp",
                evidence,
            )
        }
        Arm::PreBoundaryMatchedControl => {
            let same_endpoints = evidence
                .last_moving
                .stage_by_physical
                .keys()
                .eq(evidence.first_clamped.stage_by_physical.keys());
            let survived = complete_trace(evidence)
                && same_endpoints
                && evidence.last_moving.position_before != evidence.last_moving.position_after
                && evidence.first_clamped.position_before == evidence.first_clamped.position_after
                && evidence.last_moving.propagation_budget_exhaustions == 0
                && evidence.hand.exact_replay
                && evidence.hand.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({
                    "last_moving": evidence.last_moving,
                    "first_clamped": evidence.first_clamped,
                    "same_physical_endpoints": same_endpoints,
                }),
                "the last moving step was not a valid matched physical-endpoint control",
                evidence,
            )
        }
        Arm::CompleteLocalization => {
            let first = &evidence.first_clamped.stage_by_physical;
            let stable = evidence
                .first_no_consequence_clamp
                .as_ref()
                .is_none_or(|later| first == &later.stage_by_physical);
            let silent_stage = evidence
                .first_clamped
                .motors
                .iter()
                .find(|motor| motor.emitted_outputs == 0)
                .map(MotorStageEvidence::stage);
            let survived = complete_trace(evidence)
                && stable
                && silent_stage.is_some()
                && evidence
                    .post_clamp
                    .consequence_born_ineligible_origins
                    .is_superset(&evidence.post_clamp.admitted_return_origins)
                && !evidence.post_clamp.consequence_write_ticks.is_empty()
                && evidence.hand.exact_replay
                && evidence.hand.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({
                    "last_moving": evidence.last_moving,
                    "first_clamped": evidence.first_clamped,
                    "post_clamp": evidence.post_clamp,
                    "first_no_consequence_clamp": evidence.first_no_consequence_clamp,
                    "clamped_steps": evidence.clamped_steps,
                    "silent_motor_first_failed_stage": silent_stage,
                    "failure_stage_survives_consequence_expiry": stable,
                    "consequence_expired_while_clamped": evidence.first_no_consequence_clamp.is_some(),
                    "ineligible_returns_still_write_consequence": evidence.post_clamp.consequence_born_ineligible_origins.is_superset(&evidence.post_clamp.admitted_return_origins) && !evidence.post_clamp.consequence_write_ticks.is_empty(),
                    "first_missing_transition": "consequence-born eligibility is observed for closure but not enforced for consequence admission",
                    "core_diagnostic_sufficient": survived,
                    "new_core_event_required": false,
                }),
                "the existing diagnostic could not produce one stable first failed stage",
                evidence,
            )
        }
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_diagnostic_distinguishes_both_motor_endpoints() {
        let observed = run(Arm::UpperBoundaryOppositeCandidateLocalization);
        assert_eq!(observed.outcome, "survived", "{observed:#?}");
    }

    #[test]
    fn last_moving_step_is_a_matched_control() {
        let observed = run(Arm::PreBoundaryMatchedControl);
        assert_eq!(observed.outcome, "survived", "{observed:#?}");
    }

    #[test]
    fn one_pass_localizes_a_stable_first_failed_stage() {
        let observed = run(Arm::CompleteLocalization);
        assert_eq!(observed.outcome, "survived", "{observed:#?}");
        assert_eq!(observed.observations["core_diagnostic_sufficient"], true);
        assert_eq!(observed.observations["new_core_event_required"], false);
    }
}
