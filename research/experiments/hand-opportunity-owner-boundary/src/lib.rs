#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    FreshOpportunityEvaluationEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{
    FreshOpportunityDecision, JunctionId, LearnerId, LearnerObservation,
    LearnerOwnershipRelation, Protocol, classify_learner_ownership_relation,
};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    RelationClassifierControls,
    OrganismToRootHandBoundary,
}

impl Arm {
    pub const ALL: [Self; 2] = [Self::RelationClassifierControls, Self::OrganismToRootHandBoundary];

    pub const fn id(self) -> &'static str {
        match self {
            Self::RelationClassifierControls => "relation-classifier-controls",
            Self::OrganismToRootHandBoundary => "organism-to-root-hand-boundary",
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
struct Evidence {
    classifier_relations: Vec<LearnerOwnershipRelation>,
    evaluations: Vec<FreshOpportunityEvaluationEvidence>,
    actual_position_changes: usize,
    emitted_physical: BTreeSet<u64>,
    escaped_upper: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    propagation_budget_exhaustions: u64,
}

fn classifier_relations() -> Vec<LearnerOwnershipRelation> {
    let learner = |id: u64, parent: Option<u64>| LearnerObservation {
        id: LearnerId(id),
        parent: parent.map(LearnerId),
        surface: JunctionId(0),
        output: JunctionId(0),
        junctions: Vec::new(),
        links: Vec::new(),
    };
    let learners = [
        learner(1, None),
        learner(2, None),
        learner(3, Some(1)),
        learner(4, Some(1)),
    ];
    [
        (None, None),
        (Some(1), Some(1)),
        (None, Some(1)),
        (Some(1), None),
        (Some(1), Some(3)),
        (Some(3), Some(1)),
        (Some(3), Some(4)),
        (Some(1), Some(2)),
        (Some(9), Some(9)),
    ]
    .into_iter()
    .map(|(donor, recipient)| {
        classify_learner_ownership_relation(
            donor.map(LearnerId),
            recipient.map(LearnerId),
            &learners,
        )
    })
    .collect()
}

fn measure() -> Evidence {
    let hand = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerFreshOpportunity,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    Evidence {
        classifier_relations: classifier_relations(),
        evaluations: hand
            .trajectory
            .iter()
            .flat_map(|step| step.fresh_opportunity_evaluations.iter().cloned())
            .collect(),
        actual_position_changes: hand
            .trajectory
            .iter()
            .filter(|step| step.position_before != step.position_after)
            .count(),
        emitted_physical: hand
            .trajectory
            .iter()
            .flat_map(|step| step.emitted_outputs.iter().copied())
            .collect(),
        escaped_upper: hand.escaped_upper,
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
        propagation_budget_exhaustions: hand
            .trajectory
            .iter()
            .map(|step| step.propagation_budget_exhaustions)
            .sum(),
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
        schema: "hand-opportunity-owner-boundary/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations: serde_json::to_value(evidence).expect("evidence serializes"),
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: evidence.exact_replay,
        naturally_quiescent: evidence.naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    let controls = [
        LearnerOwnershipRelation::SameOwner,
        LearnerOwnershipRelation::SameOwner,
        LearnerOwnershipRelation::OrganismToRoot,
        LearnerOwnershipRelation::RootToOrganism,
        LearnerOwnershipRelation::ParentToChild,
        LearnerOwnershipRelation::ChildToParent,
        LearnerOwnershipRelation::Siblings,
        LearnerOwnershipRelation::Unrelated,
        LearnerOwnershipRelation::Unrelated,
    ];
    let integrity = evidence.actual_position_changes == 4
        && evidence.emitted_physical == BTreeSet::from([20_001])
        && !evidence.escaped_upper
        && evidence.propagation_budget_exhaustions == 0
        && evidence.exact_replay
        && evidence.naturally_quiescent;
    match arm {
        Arm::RelationClassifierControls => result(
            arm,
            evidence.classifier_relations == controls,
            "the pure ancestry classifier aliased an adjacent, sibling, unrelated, or unknown owner pair",
            evidence,
        ),
        Arm::OrganismToRootHandBoundary => result(
            arm,
            integrity
                && evidence.evaluations.len() == 5
                && evidence.evaluations.iter().all(|evaluation| {
                    evaluation.ownership_relation == LearnerOwnershipRelation::OrganismToRoot
                })
                && evidence.evaluations.iter().filter(|evaluation| {
                    evaluation.decision == FreshOpportunityDecision::RejectedOwnerMismatch
                }).count() == 4
                && evidence.evaluations.iter().filter(|evaluation| {
                    evaluation.decision == FreshOpportunityDecision::RejectedRecentDonor
                }).count() == 1,
            "a failed hand pair was not the exact organism-to-root adjacent relation",
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
    fn classifier_and_hand_boundary_follow_frozen_predicates() {
        let controls = run(Arm::RelationClassifierControls);
        let hand = run(Arm::OrganismToRootHandBoundary);
        assert_eq!(controls.outcome, "survived", "{controls:#?}");
        assert_eq!(hand.outcome, "survived", "{hand:#?}");
    }
}
