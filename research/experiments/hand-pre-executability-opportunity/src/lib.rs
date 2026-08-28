#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    OutputCandidateEvidence, ReflectedHandProtocolEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{CandidateOwnership, Protocol};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    BalancedDriveWithoutOpportunity,
    OwnershipRejection,
    NoLivePathDrive,
}

impl Arm {
    pub const ALL: [Self; 3] = [
        Self::BalancedDriveWithoutOpportunity,
        Self::OwnershipRejection,
        Self::NoLivePathDrive,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::BalancedDriveWithoutOpportunity => "balanced-drive-without-opportunity",
            Self::OwnershipRejection => "ownership-rejection",
            Self::NoLivePathDrive => "no-live-path-drive",
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
struct DiagnosticSnapshot {
    step: usize,
    position: i16,
    consequence_writes: usize,
    emitted_outputs: Vec<u64>,
    silent_candidates: Vec<OutputCandidateEvidence>,
    executable_incumbents: Vec<OutputCandidateEvidence>,
}

#[derive(Clone, Debug)]
struct Evidence {
    snapshot: Option<DiagnosticSnapshot>,
    actual_position_changes: usize,
    escaped_upper: bool,
    propagation_budget_exhaustions: u64,
    exact_replay: bool,
    naturally_quiescent: bool,
}

fn summarize(hand: ReflectedHandProtocolEvidence) -> Evidence {
    let actual_position_changes = hand
        .trajectory
        .iter()
        .filter(|step| step.position_before != step.position_after)
        .count();
    let snapshot = hand
        .trajectory
        .iter()
        .find(|step| {
            step.position_before == 4
                && step.position_after == 4
                && step.consequence_writes.is_empty()
                && step.output_candidates.iter().any(|candidate| {
                    candidate.is_motor && candidate.path_inputs > 0 && !candidate.executable
                })
                && step
                    .output_candidates
                    .iter()
                    .any(|candidate| candidate.is_motor && candidate.executable)
        })
        .map(|step| DiagnosticSnapshot {
            step: step.index,
            position: step.position_before,
            consequence_writes: step.consequence_writes.len(),
            emitted_outputs: step.emitted_outputs.clone(),
            silent_candidates: step
                .output_candidates
                .iter()
                .filter(|candidate| {
                    candidate.is_motor && candidate.path_inputs > 0 && !candidate.executable
                })
                .cloned()
                .collect(),
            executable_incumbents: step
                .output_candidates
                .iter()
                .filter(|candidate| candidate.is_motor && candidate.executable)
                .cloned()
                .collect(),
        });
    Evidence {
        snapshot,
        actual_position_changes,
        escaped_upper: hand.escaped_upper,
        propagation_budget_exhaustions: hand
            .trajectory
            .iter()
            .map(|step| step.propagation_budget_exhaustions)
            .sum(),
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
    }
}

fn measure() -> Evidence {
    summarize(run_reflected_hand_bounded(
        Protocol::RecursiveLearnerPhysicalTransitionReturn,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    ))
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

fn result(arm: Arm, survived: bool, falsifier: &'static str, evidence: &Evidence) -> ProbeResult {
    ProbeResult {
        schema: "hand-pre-executability-opportunity/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations: serde_json::json!({
            "snapshot": evidence.snapshot,
            "actual_position_changes": evidence.actual_position_changes,
            "escaped_upper": evidence.escaped_upper,
            "propagation_budget_exhaustions": evidence.propagation_budget_exhaustions,
        }),
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: evidence.exact_replay,
        naturally_quiescent: evidence.naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    let snapshot = evidence.snapshot.as_ref();
    let silent = snapshot.and_then(|snapshot| snapshot.silent_candidates.first());
    let live_incumbent = snapshot.is_some_and(|snapshot| {
        snapshot
            .executable_incumbents
            .iter()
            .any(|candidate| candidate.unanswered_returns > 0)
    });
    let integrity = evidence.actual_position_changes == 4
        && !evidence.escaped_upper
        && evidence.propagation_budget_exhaustions == 0
        && evidence.exact_replay
        && evidence.naturally_quiescent;

    match arm {
        Arm::BalancedDriveWithoutOpportunity => result(
            arm,
            integrity
                && live_incumbent
                && silent.is_some_and(|candidate| {
                    candidate.positive_path_strength > 0
                        && candidate.positive_path_strength == candidate.negative_path_strength
                        && candidate.opportunity == 0
                        && candidate.admitted_drive == 0
                        && candidate.projected_drive == 0
                }),
            "the silent drive was not balanced and opportunity-starved alongside a live unanswered incumbent",
            evidence,
        ),
        Arm::OwnershipRejection => result(
            arm,
            integrity
                && silent.is_some_and(|candidate| {
                    candidate.ownership == CandidateOwnership::Ambiguous
                        && candidate.positive_path_strength != candidate.negative_path_strength
                        && (candidate.positive_path_strength > 0
                            || candidate.negative_path_strength > 0)
                        && candidate.admitted_drive != 0
                        && candidate.projected_drive == 0
                }),
            "balanced cancellation or absent live drive occurred before an ownership-only rejection",
            evidence,
        ),
        Arm::NoLivePathDrive => result(
            arm,
            integrity
                && silent.is_some_and(|candidate| {
                    candidate.path_inputs > 0
                        && candidate.positive_path_strength == 0
                        && candidate.negative_path_strength == 0
                }),
            "at least one live signed path strength reached the silent candidate",
            evidence,
        ),
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_explanations_are_mutually_discriminated() {
        let results = run_all();
        assert_eq!(
            results
                .iter()
                .filter(|(_, result)| result.outcome == "survived")
                .count(),
            1,
            "{results:#?}"
        );
    }

    #[test]
    fn diagnostics_preserve_parent_behavior_replay_and_quiescence() {
        let observed = evidence();
        assert_eq!(observed.actual_position_changes, 4, "{observed:#?}");
        assert!(!observed.escaped_upper, "{observed:#?}");
        assert_eq!(observed.propagation_budget_exhaustions, 0);
        assert!(observed.exact_replay);
        assert!(observed.naturally_quiescent);
    }
}
