#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    ReflectedHandProtocolEvidence, ReflectedHandStepEvidence, run_reflected_hand_with_protocol,
};
use serde::Serialize;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{CandidateOwnership, Protocol};

const OWNER_AMBIGUITY: &str = "mixed-owner motor candidate resolution";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    BoundaryCandidateLocalization,
    TemporalMatchedControl,
    CompleteLocalization,
}

impl Arm {
    pub const ALL: [Self; 3] = [
        Self::BoundaryCandidateLocalization,
        Self::TemporalMatchedControl,
        Self::CompleteLocalization,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::BoundaryCandidateLocalization => "boundary-candidate-localization",
            Self::TemporalMatchedControl => "temporal-matched-control",
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StepLocalization {
    pub index: usize,
    pub position_before: i16,
    pub position_after: i16,
    pub delivered_surfaces: usize,
    pub owner_writes: u64,
    pub owned_surface_firings_with_paths: usize,
    pub maximum_retained_paths: u32,
    pub maximum_consequential_paths: u32,
    pub motor_candidates: usize,
    pub motor_candidates_reached_by_paths: usize,
    pub ambiguous_motor_candidates: usize,
    pub owned_motor_candidates: usize,
    pub executable_motor_candidates: usize,
    pub consequential_candidate_reads: usize,
    pub strongest_admitted_drive: u64,
}

impl StepLocalization {
    fn from_step(step: &ReflectedHandStepEvidence) -> Self {
        let owned_paths = step
            .surface_paths
            .iter()
            .filter(|path| path.owner.is_some() && path.complete_paths > 0)
            .collect::<Vec<_>>();
        let motor_candidates = step
            .output_candidates
            .iter()
            .filter(|candidate| candidate.is_motor)
            .collect::<Vec<_>>();
        Self {
            index: step.index,
            position_before: step.position_before,
            position_after: step.position_after,
            delivered_surfaces: step.delivered_surface_count,
            owner_writes: step.owner_writes,
            owned_surface_firings_with_paths: owned_paths.len(),
            maximum_retained_paths: owned_paths
                .iter()
                .map(|path| path.complete_paths)
                .max()
                .unwrap_or(0),
            maximum_consequential_paths: owned_paths
                .iter()
                .map(|path| path.consequential_paths)
                .max()
                .unwrap_or(0),
            motor_candidates: motor_candidates.len(),
            motor_candidates_reached_by_paths: motor_candidates
                .iter()
                .filter(|candidate| candidate.path_inputs > 0)
                .count(),
            ambiguous_motor_candidates: motor_candidates
                .iter()
                .filter(|candidate| candidate.ownership == CandidateOwnership::Ambiguous)
                .count(),
            owned_motor_candidates: motor_candidates
                .iter()
                .filter(|candidate| matches!(candidate.ownership, CandidateOwnership::Owned(_)))
                .count(),
            executable_motor_candidates: motor_candidates
                .iter()
                .filter(|candidate| candidate.executable)
                .count(),
            consequential_candidate_reads: motor_candidates
                .iter()
                .filter(|candidate| candidate.consequence_tick.is_some())
                .count(),
            strongest_admitted_drive: motor_candidates
                .iter()
                .map(|candidate| candidate.admitted_drive.unsigned_abs())
                .max()
                .unwrap_or(0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ProtocolLocalization {
    pub protocol: Protocol,
    pub changed_steps: usize,
    pub last_changed_step: Option<usize>,
    pub terminal_step: Option<StepLocalization>,
    pub first_post_stall_step: Option<StepLocalization>,
    pub first_missing_transition: &'static str,
    pub path_lifetime_survived: bool,
    pub path_traversal_survived: bool,
    pub exact_owned_candidate_present: bool,
    pub mixed_owner_block_observed: bool,
    pub exact_replay: bool,
    pub naturally_quiescent: bool,
}

fn localize(hand: &ReflectedHandProtocolEvidence) -> ProtocolLocalization {
    let last_changed_step = hand
        .trajectory
        .iter()
        .rfind(|step| step.position_before != step.position_after)
        .map(|step| step.index);
    let terminal_index = last_changed_step.map(|index| index.saturating_add(1));
    let terminal_step = terminal_index
        .and_then(|index| hand.trajectory.get(index))
        .map(StepLocalization::from_step);
    let first_post_stall_step = terminal_index
        .and_then(|index| hand.trajectory.get(index.saturating_add(1)))
        .map(StepLocalization::from_step);
    let observed = first_post_stall_step.as_ref().or(terminal_step.as_ref());
    let path_lifetime_survived = observed.is_some_and(|step| {
        step.owned_surface_firings_with_paths > 0 && step.maximum_consequential_paths > 0
    });
    let path_traversal_survived = observed.is_some_and(|step| {
        step.motor_candidates_reached_by_paths > 0 && step.strongest_admitted_drive > 0
    });
    let exact_owned_candidate_present =
        observed.is_some_and(|step| step.owned_motor_candidates > 0);
    let mixed_owner_block_observed = observed.is_some_and(|step| {
        step.motor_candidates_reached_by_paths > 0
            && step.ambiguous_motor_candidates == step.motor_candidates_reached_by_paths
            && step.executable_motor_candidates == 0
    });
    let first_missing_transition = if !path_lifetime_survived {
        "retained reverse-path lifetime"
    } else if !path_traversal_survived {
        "reverse-path traversal into a motor candidate"
    } else if mixed_owner_block_observed {
        OWNER_AMBIGUITY
    } else if exact_owned_candidate_present
        && observed.is_some_and(|step| step.consequential_candidate_reads == 0)
    {
        "owner-local consequence lookup"
    } else {
        "motor candidate selection or output"
    };
    ProtocolLocalization {
        protocol: hand.protocol,
        changed_steps: hand.changed_steps,
        last_changed_step,
        terminal_step,
        first_post_stall_step,
        first_missing_transition,
        path_lifetime_survived,
        path_traversal_survived,
        exact_owned_candidate_present,
        mixed_owner_block_observed,
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
    }
}

#[derive(Clone, Debug)]
struct Evidence {
    candidate: ProtocolLocalization,
    reference: ProtocolLocalization,
    behavior_equal: bool,
    diagnosis_equal: bool,
}

fn behavior_equal(
    left: &ReflectedHandProtocolEvidence,
    right: &ReflectedHandProtocolEvidence,
) -> bool {
    left.trajectory
        .iter()
        .zip(&right.trajectory)
        .all(|(left, right)| {
            left.position_before == right.position_before
                && left.position_after == right.position_after
                && left.direction == right.direction
                && left.emitted_outputs == right.emitted_outputs
        })
        && left.trajectory.len() == right.trajectory.len()
}

fn measure() -> Evidence {
    let candidate = run_reflected_hand_with_protocol(Protocol::RecursiveLearnerBoundaryNovelty);
    let reference =
        run_reflected_hand_with_protocol(Protocol::RecursiveLearnerEligibleReturnClosure);
    let behavior_equal = behavior_equal(&candidate, &reference);
    let candidate = localize(&candidate);
    let reference = localize(&reference);
    let diagnosis_equal = candidate.first_missing_transition == reference.first_missing_transition
        && candidate.path_lifetime_survived == reference.path_lifetime_survived
        && candidate.path_traversal_survived == reference.path_traversal_survived
        && candidate.mixed_owner_block_observed == reference.mixed_owner_block_observed;
    Evidence {
        candidate,
        reference,
        behavior_equal,
        diagnosis_equal,
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
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "hand-autonomous-reuse-localization/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations,
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay,
        naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    match arm {
        Arm::BoundaryCandidateLocalization => {
            let observed = &evidence.candidate;
            let survived = observed.path_lifetime_survived
                && observed.path_traversal_survived
                && observed.mixed_owner_block_observed
                && observed.first_missing_transition == OWNER_AMBIGUITY
                && observed.exact_replay
                && observed.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::to_value(observed).expect("candidate localization serializes"),
                "the candidate trace did not order retained path, traversal, ownership, and executability",
                observed.exact_replay,
                observed.naturally_quiescent,
            )
        }
        Arm::TemporalMatchedControl => {
            let observed = &evidence.reference;
            let survived = observed.first_missing_transition == OWNER_AMBIGUITY
                && evidence.behavior_equal
                && evidence.diagnosis_equal
                && observed.exact_replay
                && observed.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({
                    "reference": observed,
                    "behavior_equal": evidence.behavior_equal,
                    "diagnosis_equal": evidence.diagnosis_equal,
                }),
                "the temporal reference did not preserve the candidate behavior and first missing transition",
                observed.exact_replay,
                observed.naturally_quiescent,
            )
        }
        Arm::CompleteLocalization => {
            let survived = evidence.behavior_equal
                && evidence.diagnosis_equal
                && evidence.candidate.first_missing_transition == OWNER_AMBIGUITY
                && evidence.reference.first_missing_transition == OWNER_AMBIGUITY
                && evidence.candidate.exact_replay
                && evidence.reference.exact_replay
                && evidence.candidate.naturally_quiescent
                && evidence.reference.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({
                    "candidate": evidence.candidate,
                    "reference": evidence.reference,
                    "behavior_equal": evidence.behavior_equal,
                    "diagnosis_equal": evidence.diagnosis_equal,
                    "retained_path_hypothesis": "falsified",
                    "exact_owned_selection_reentry_hypothesis": "falsified-prerequisite",
                    "next_hypothesis": "factor simultaneous motor inputs by their existing physical learner owner before evaluating executability",
                }),
                "the one-pass diagnostic did not isolate one matched first missing transition",
                evidence.candidate.exact_replay && evidence.reference.exact_replay,
                evidence.candidate.naturally_quiescent && evidence.reference.naturally_quiescent,
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
    fn boundary_candidate_localizes_mixed_owner_block_after_live_traversal() {
        let result = run(Arm::BoundaryCandidateLocalization);
        assert_eq!(result.outcome, "survived");
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
    }

    #[test]
    fn temporal_reference_matches_behavior_and_diagnosis() {
        let result = run(Arm::TemporalMatchedControl);
        assert_eq!(result.outcome, "survived");
    }

    #[test]
    fn complete_localization_is_compact_and_deterministic() {
        let first = serde_json::to_vec(&run(Arm::CompleteLocalization)).unwrap();
        let second = serde_json::to_vec(&run(Arm::CompleteLocalization)).unwrap();
        assert_eq!(first, second);
        assert!(first.len() < 16_384);
    }
}
