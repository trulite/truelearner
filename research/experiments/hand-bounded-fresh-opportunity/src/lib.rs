#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    FreshOpportunityEvaluationEvidence, FreshOpportunityEvidence, OutputCandidateEvidence,
    ReflectedHandProtocolEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{FreshOpportunityDecision, LinkId, Protocol};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    ConstructedBoundedFreshOpportunity,
    LifetimeLocalityOwnershipControls,
    ConditionalReflectedHandRelease,
    CompleteComposition,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::ConstructedBoundedFreshOpportunity,
        Self::LifetimeLocalityOwnershipControls,
        Self::ConditionalReflectedHandRelease,
        Self::CompleteComposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::ConstructedBoundedFreshOpportunity => "constructed-bounded-fresh-opportunity",
            Self::LifetimeLocalityOwnershipControls => "lifetime-locality-ownership-controls",
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
    actual_position_changes: usize,
    directions: BTreeSet<i8>,
    emitted_physical: BTreeSet<u64>,
    reached_upper: bool,
    escaped_upper: bool,
    reached_lower: bool,
    escaped_lower: bool,
    final_position: i16,
    primary_closed: bool,
    perturbation_recovered: bool,
    fresh_opportunities: Vec<FreshOpportunityEvidence>,
    fresh_opportunity_evaluations: Vec<FreshOpportunityEvaluationEvidence>,
    supplied_candidates: Vec<OutputCandidateEvidence>,
    superseded_returns: BTreeSet<LinkId>,
    unique_transfer_returns: bool,
    every_transfer_superseded: bool,
    propagation_budget_exhaustions: u64,
    exact_replay: bool,
    naturally_quiescent: bool,
}

fn summarize(hand: &ReflectedHandProtocolEvidence) -> HandSummary {
    let fresh_opportunities = hand
        .trajectory
        .iter()
        .flat_map(|step| step.fresh_opportunities.iter().cloned())
        .collect::<Vec<_>>();
    let transfer_returns = fresh_opportunities
        .iter()
        .map(|transfer| transfer.return_link)
        .collect::<BTreeSet<_>>();
    let superseded_returns = hand
        .trajectory
        .iter()
        .flat_map(|step| step.superseded_returns.iter().copied())
        .collect::<BTreeSet<_>>();
    HandSummary {
        protocol: hand.protocol,
        actual_position_changes: hand
            .trajectory
            .iter()
            .filter(|step| step.position_before != step.position_after)
            .count(),
        directions: hand.directions.clone(),
        emitted_physical: hand
            .trajectory
            .iter()
            .flat_map(|step| step.emitted_outputs.iter().copied())
            .collect(),
        reached_upper: hand.reached_upper,
        escaped_upper: hand.escaped_upper,
        reached_lower: hand.reached_lower,
        escaped_lower: hand.escaped_lower,
        final_position: hand.final_position,
        primary_closed: hand.primary_closed,
        perturbation_recovered: hand.perturbation_recovered,
        fresh_opportunity_evaluations: hand
            .trajectory
            .iter()
            .flat_map(|step| step.fresh_opportunity_evaluations.iter().cloned())
            .collect(),
        supplied_candidates: hand
            .trajectory
            .iter()
            .flat_map(|step| step.output_candidates.iter())
            .filter(|candidate| candidate.is_motor && candidate.supplied_opportunity > 0)
            .cloned()
            .collect(),
        unique_transfer_returns: transfer_returns.len() == fresh_opportunities.len(),
        every_transfer_superseded: transfer_returns.is_subset(&superseded_returns),
        fresh_opportunities,
        superseded_returns,
        propagation_budget_exhaustions: hand
            .trajectory
            .iter()
            .map(|step| step.propagation_budget_exhaustions)
            .sum(),
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
    }
}

#[derive(Clone, Debug)]
struct Evidence {
    candidate: HandSummary,
    parent: HandSummary,
}

fn measure() -> Evidence {
    Evidence {
        candidate: summarize(&run_reflected_hand_bounded(
            Protocol::RecursiveLearnerFreshOpportunity,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            MAX_MOMENTS_PER_SEND,
        )),
        parent: summarize(&run_reflected_hand_bounded(
            Protocol::RecursiveLearnerPhysicalTransitionReturn,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            MAX_MOMENTS_PER_SEND,
        )),
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

fn result(arm: Arm, survived: bool, falsifier: &'static str, evidence: &Evidence) -> ProbeResult {
    ProbeResult {
        schema: "hand-bounded-fresh-opportunity/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations: serde_json::json!({
            "candidate": evidence.candidate,
            "parent": evidence.parent,
        }),
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: evidence.candidate.exact_replay,
        naturally_quiescent: evidence.candidate.naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    let integrity = evidence.candidate.propagation_budget_exhaustions == 0
        && evidence.candidate.exact_replay
        && evidence.candidate.naturally_quiescent;
    match arm {
        Arm::ConstructedBoundedFreshOpportunity => result(
            arm,
            integrity
                && !evidence.candidate.fresh_opportunities.is_empty()
                && evidence.candidate.fresh_opportunities.len()
                    == evidence.candidate.supplied_candidates.len()
                && evidence.candidate.unique_transfer_returns
                && evidence.candidate.every_transfer_superseded,
            "a transfer did not produce one executable supplied candidate and exact donor supersession",
            evidence,
        ),
        Arm::LifetimeLocalityOwnershipControls => result(
            arm,
            integrity
                && evidence.parent.fresh_opportunities.is_empty()
                && evidence
                    .candidate
                    .fresh_opportunity_evaluations
                    .iter()
                    .any(|evaluation| {
                        evaluation.decision == FreshOpportunityDecision::RejectedOwnerMismatch
                    })
                && evidence
                    .candidate
                    .fresh_opportunity_evaluations
                    .iter()
                    .any(|evaluation| {
                        evaluation.decision == FreshOpportunityDecision::RejectedRecentDonor
                    })
                && evidence.parent.actual_position_changes == 4
                && evidence.parent.emitted_physical == BTreeSet::from([20_001])
                && evidence.candidate.unique_transfer_returns
                && evidence.candidate.every_transfer_superseded,
            "old protocol behavior changed or one exact donor return transferred repeatedly",
            evidence,
        ),
        Arm::ConditionalReflectedHandRelease => result(
            arm,
            integrity
                && evidence.candidate.reached_upper
                && evidence.candidate.escaped_upper
                && evidence.candidate.directions.len() == 2
                && evidence.candidate.emitted_physical == BTreeSet::from([20_000, 20_001])
                && evidence.candidate.actual_position_changes
                    > evidence.parent.actual_position_changes,
            "the local transfer did not safely execute the second motor and leave upper contact",
            evidence,
        ),
        Arm::CompleteComposition => result(
            arm,
            integrity
                && evidence.candidate.reached_lower
                && evidence.candidate.escaped_lower
                && evidence.candidate.primary_closed
                && evidence.candidate.perturbation_recovered,
            "boundary release did not compose into complete reflected-joint control",
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
    fn local_transfer_and_controls_follow_frozen_predicates() {
        let transfer = run(Arm::ConstructedBoundedFreshOpportunity);
        let controls = run(Arm::LifetimeLocalityOwnershipControls);
        assert!(matches!(transfer.outcome, "survived" | "falsified"));
        assert_eq!(controls.outcome, "survived", "{controls:#?}");
    }

    #[test]
    fn hand_release_and_complete_composition_preserve_integrity() {
        let release = run(Arm::ConditionalReflectedHandRelease);
        let complete = run(Arm::CompleteComposition);
        assert!(matches!(release.outcome, "survived" | "falsified"));
        assert!(matches!(complete.outcome, "survived" | "falsified"));
        assert!(release.exact_replay && complete.exact_replay);
        assert!(release.naturally_quiescent && complete.naturally_quiescent);
    }
}
