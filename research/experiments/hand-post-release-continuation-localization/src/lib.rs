#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    ReflectedHandProtocolEvidence, ReflectedHandStepEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::Protocol;

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;
const UPPER: i16 = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    TrajectoryIntegrity,
    FirstContinuationFailure,
}

impl Arm {
    pub const ALL: [Self; 2] = [Self::TrajectoryIntegrity, Self::FirstContinuationFailure];

    pub const fn id(self) -> &'static str {
        match self {
            Self::TrajectoryIntegrity => "trajectory-integrity",
            Self::FirstContinuationFailure => "first-continuation-failure",
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum FailureClass {
    ReversedMotorOutput,
    NoMotorCandidate,
    BlockedMotorCandidate,
    ExecutableCandidateWithoutOutput,
    MotorOutputWithoutWorldChange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ContinuationFailure {
    upper_escape_step: usize,
    continuation_direction: i8,
    first_failure_step: usize,
    failure_class: FailureClass,
    escape: ReflectedHandStepEvidence,
    failure: ReflectedHandStepEvidence,
}

fn continuation_failure(hand: &ReflectedHandProtocolEvidence) -> Option<ContinuationFailure> {
    let escape_index = hand.trajectory.iter().position(|step| {
        step.position_before == UPPER && step.position_after < step.position_before
    })?;
    let escape = hand.trajectory.get(escape_index)?;
    let failure = hand
        .trajectory
        .iter()
        .skip(escape_index.saturating_add(1))
        .find(|step| step.direction != escape.direction)?;
    let motor_candidates = failure
        .output_candidates
        .iter()
        .filter(|candidate| candidate.is_motor)
        .collect::<Vec<_>>();
    let failure_class = if failure.direction == -escape.direction && failure.direction != 0 {
        FailureClass::ReversedMotorOutput
    } else if motor_candidates.is_empty() {
        FailureClass::NoMotorCandidate
    } else if motor_candidates
        .iter()
        .all(|candidate| !candidate.executable)
    {
        FailureClass::BlockedMotorCandidate
    } else if failure.emitted_outputs.is_empty() {
        FailureClass::ExecutableCandidateWithoutOutput
    } else {
        FailureClass::MotorOutputWithoutWorldChange
    };
    Some(ContinuationFailure {
        upper_escape_step: escape.index,
        continuation_direction: escape.direction,
        first_failure_step: failure.index,
        failure_class,
        escape: escape.clone(),
        failure: failure.clone(),
    })
}

#[derive(Clone, Debug)]
struct Evidence {
    candidate: ReflectedHandProtocolEvidence,
    strict: ReflectedHandProtocolEvidence,
    failure: Option<ContinuationFailure>,
}

fn measure() -> Evidence {
    let candidate = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerRootFreshOpportunity,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let failure = continuation_failure(&candidate);
    Evidence {
        candidate,
        strict: run_reflected_hand_bounded(
            Protocol::RecursiveLearnerFreshOpportunity,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            MAX_MOMENTS_PER_SEND,
        ),
        failure,
    }
}

static EVIDENCE: OnceLock<Evidence> = OnceLock::new();

fn evidence() -> &'static Evidence {
    EVIDENCE.get_or_init(measure)
}

fn propagation_exhaustions(hand: &ReflectedHandProtocolEvidence) -> u64 {
    hand.trajectory
        .iter()
        .map(|step| step.propagation_budget_exhaustions)
        .sum()
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

fn result(arm: Arm, survived: bool, falsifier: &'static str, evidence: &Evidence) -> ProbeResult {
    ProbeResult {
        schema: "hand-post-release-continuation-localization/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations: serde_json::json!({
            "candidate": evidence.candidate,
            "strict": evidence.strict,
            "first_continuation_failure": evidence.failure,
        }),
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: evidence.candidate.exact_replay,
        naturally_quiescent: evidence.candidate.naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    let integrity = evidence.candidate.reached_upper
        && evidence.candidate.escaped_upper
        && !evidence.candidate.reached_lower
        && evidence.candidate.exact_replay
        && evidence.candidate.naturally_quiescent
        && propagation_exhaustions(&evidence.candidate) == 0
        && evidence.strict.reached_upper
        && !evidence.strict.escaped_upper
        && evidence.strict.changed_steps == 4
        && evidence.strict.exact_replay
        && evidence.strict.naturally_quiescent
        && propagation_exhaustions(&evidence.strict) == 0;
    match arm {
        Arm::TrajectoryIntegrity => result(
            arm,
            integrity,
            "candidate release, strict-parent clamp, replay, quiescence, or propagation did not reproduce",
            evidence,
        ),
        Arm::FirstContinuationFailure => {
            let localized = evidence.failure.as_ref().is_some_and(|failure| {
                failure.first_failure_step > failure.upper_escape_step
                    && (!failure.failure.physical_incidences.is_empty()
                        || !failure.failure.output_candidates.is_empty()
                        || !failure.failure.surface_paths.is_empty())
            });
            result(
                arm,
                integrity && localized,
                "no unique first loss of post-release continuation was visible in the complete causal trace",
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
    fn unchanged_trajectories_reproduce_with_integrity() {
        assert_eq!(run(Arm::TrajectoryIntegrity).outcome, "survived");
    }

    #[test]
    fn first_post_release_loss_is_localized_once() {
        let result = run(Arm::FirstContinuationFailure);
        assert_eq!(result.outcome, "survived", "{result:#?}");
    }
}
