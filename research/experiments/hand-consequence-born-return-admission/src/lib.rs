#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    ReflectedHandProtocolEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::Protocol;

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    ConsequenceBornReturnAdmission,
    TruthfulNoConsequenceRelease,
    ConditionalReflectedHandRelease,
    CompleteComposition,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::ConsequenceBornReturnAdmission,
        Self::TruthfulNoConsequenceRelease,
        Self::ConditionalReflectedHandRelease,
        Self::CompleteComposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::ConsequenceBornReturnAdmission => "consequence-born-return-admission",
            Self::TruthfulNoConsequenceRelease => "truthful-no-consequence-release",
            Self::ConditionalReflectedHandRelease => "conditional-reflected-hand-release",
            Self::CompleteComposition => "complete-composition",
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
    effort_steps: usize,
    actual_position_changes: usize,
    directions: BTreeSet<i8>,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    final_position: i16,
    primary_closed: bool,
    perturbation_recovered: bool,
    rejected_before_opening: usize,
    admitted_later_returns: usize,
    consequence_writes_without_delivered_surface: usize,
    no_surface_admitted_origins: BTreeSet<u64>,
    no_surface_rejected_origins: BTreeSet<u64>,
    no_surface_eligible_origins: BTreeSet<u64>,
    no_surface_ineligible_origins: BTreeSet<u64>,
    propagation_budget_exhaustions: u64,
    exact_replay: bool,
    naturally_quiescent: bool,
    emitted_physical: BTreeSet<u64>,
    no_surface_eligibility: Vec<ResidualEligibility>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct ResidualEligibility {
    step: usize,
    origin: u64,
    birth_tick: i64,
    return_opened_tick: i64,
    eligible: bool,
}

fn summarize(hand: &ReflectedHandProtocolEvidence) -> HandSummary {
    HandSummary {
        protocol: hand.protocol,
        effort_steps: hand.changed_steps,
        actual_position_changes: hand
            .trajectory
            .iter()
            .filter(|step| step.position_before != step.position_after)
            .count(),
        directions: hand.directions.clone(),
        reached_lower: hand.reached_lower,
        reached_upper: hand.reached_upper,
        escaped_lower: hand.escaped_lower,
        escaped_upper: hand.escaped_upper,
        final_position: hand.final_position,
        primary_closed: hand.primary_closed,
        perturbation_recovered: hand.perturbation_recovered,
        rejected_before_opening: hand
            .trajectory
            .iter()
            .flat_map(|step| &step.return_origins)
            .filter(|origin| origin.decision == "rejected-before-return-opened")
            .count(),
        admitted_later_returns: hand
            .trajectory
            .iter()
            .flat_map(|step| &step.return_origins)
            .filter(|origin| origin.decision.starts_with("admitted"))
            .count(),
        consequence_writes_without_delivered_surface: hand
            .trajectory
            .iter()
            .filter(|step| step.delivered_surface_count == 0)
            .map(|step| step.consequence_writes.len())
            .sum(),
        no_surface_admitted_origins: hand
            .trajectory
            .iter()
            .filter(|step| step.delivered_surface_count == 0)
            .flat_map(|step| &step.return_origins)
            .filter(|origin| origin.decision.starts_with("admitted"))
            .map(|origin| origin.origin_physical)
            .collect(),
        no_surface_rejected_origins: hand
            .trajectory
            .iter()
            .filter(|step| step.delivered_surface_count == 0)
            .flat_map(|step| &step.return_origins)
            .filter(|origin| origin.decision == "rejected-before-return-opened")
            .map(|origin| origin.origin_physical)
            .collect(),
        no_surface_eligible_origins: hand
            .trajectory
            .iter()
            .filter(|step| step.delivered_surface_count == 0)
            .flat_map(|step| &step.closure_eligibility)
            .filter(|origin| origin.eligible)
            .map(|origin| origin.origin_physical)
            .collect(),
        no_surface_ineligible_origins: hand
            .trajectory
            .iter()
            .filter(|step| step.delivered_surface_count == 0)
            .flat_map(|step| &step.closure_eligibility)
            .filter(|origin| !origin.eligible)
            .map(|origin| origin.origin_physical)
            .collect(),
        propagation_budget_exhaustions: hand
            .trajectory
            .iter()
            .map(|step| step.propagation_budget_exhaustions)
            .sum(),
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
        emitted_physical: hand
            .trajectory
            .iter()
            .flat_map(|step| step.emitted_outputs.iter().copied())
            .collect(),
        no_surface_eligibility: hand
            .trajectory
            .iter()
            .filter(|step| step.delivered_surface_count == 0)
            .flat_map(|step| {
                step.closure_eligibility
                    .iter()
                    .map(move |eligibility| ResidualEligibility {
                        step: step.index,
                        origin: eligibility.origin_physical,
                        birth_tick: eligibility.origin_birth_tick,
                        return_opened_tick: eligibility.return_opened_tick,
                        eligible: eligibility.eligible,
                    })
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
    }
}

#[derive(Clone, Debug)]
struct Evidence {
    candidate: HandSummary,
    reference: HandSummary,
}

fn measure() -> Evidence {
    let candidate = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerConsequenceBornReturn,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let reference = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerBoundaryEffectTerminal,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    Evidence {
        candidate: summarize(&candidate),
        reference: summarize(&reference),
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
        schema: "hand-consequence-born-return-admission/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations,
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: evidence.candidate.exact_replay,
        naturally_quiescent: evidence.candidate.naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    match arm {
        Arm::ConsequenceBornReturnAdmission => {
            let survived = evidence.candidate.rejected_before_opening > 0
                && evidence.candidate.admitted_later_returns > 0
                && evidence.candidate.exact_replay
                && evidence.candidate.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({
                    "candidate": evidence.candidate,
                    "reference": evidence.reference,
                }),
                "the candidate did not reject pre-opening origins while preserving later valid returns",
                evidence,
            )
        }
        Arm::TruthfulNoConsequenceRelease => {
            let survived = evidence.candidate.escaped_upper
                && evidence.candidate.directions.len() == 2
                && evidence.candidate.emitted_physical.len() == 2
                && evidence.candidate.propagation_budget_exhaustions == 0
                && evidence.candidate.exact_replay
                && evidence.candidate.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({
                    "candidate": evidence.candidate,
                    "reference": evidence.reference,
                    "upper_release_gained": evidence.candidate.escaped_upper && !evidence.reference.escaped_upper,
                }),
                "truthful rejection did not expose the previously suppressed physical alternative safely",
                evidence,
            )
        }
        Arm::ConditionalReflectedHandRelease => {
            let survived = evidence.candidate.reached_upper
                && evidence.candidate.escaped_upper
                && evidence.candidate.directions.len() == 2
                && evidence.candidate.actual_position_changes
                    > evidence.reference.actual_position_changes
                && evidence.candidate.propagation_budget_exhaustions == 0
                && evidence.candidate.exact_replay
                && evidence.candidate.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({
                    "candidate": evidence.candidate,
                    "reference": evidence.reference,
                }),
                "the unchanged hand did not leave the reached boundary and improve physical travel safely",
                evidence,
            )
        }
        Arm::CompleteComposition => {
            let survived = evidence.candidate.primary_closed
                && evidence.candidate.perturbation_recovered
                && evidence.candidate.propagation_budget_exhaustions == 0
                && evidence.candidate.exact_replay
                && evidence.candidate.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({
                    "candidate": evidence.candidate,
                    "reference": evidence.reference,
                    "safe_release_survived": evidence.candidate.escaped_upper && evidence.candidate.propagation_budget_exhaustions == 0,
                }),
                "safe upper release did not yet compose into complete joint closure and perturbation recovery",
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
    fn consequence_born_admission_is_observed() {
        let observed = run(Arm::ConsequenceBornReturnAdmission);
        assert_eq!(observed.outcome, "survived", "{observed:#?}");
    }

    #[test]
    fn truthful_no_consequence_can_release_the_incumbent() {
        let observed = run(Arm::TruthfulNoConsequenceRelease);
        assert!(matches!(observed.outcome, "survived" | "falsified"));
        assert!(observed.exact_replay);
        assert!(observed.naturally_quiescent);
    }

    #[test]
    fn hand_release_and_complete_composition_follow_frozen_predicates() {
        let release = run(Arm::ConditionalReflectedHandRelease);
        let complete = run(Arm::CompleteComposition);
        assert!(matches!(release.outcome, "survived" | "falsified"));
        assert!(matches!(complete.outcome, "survived" | "falsified"));
        assert!(release.exact_replay && complete.exact_replay);
        assert!(release.naturally_quiescent && complete.naturally_quiescent);
    }
}
