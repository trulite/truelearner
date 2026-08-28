#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    CandidateSelectionEvidence, OutputCandidateEvidence, ReflectedHandProtocolEvidence,
    ReturnOriginEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{PhysicalIncidence, Protocol};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    ExistingLineageDiscriminator,
    StableSurfaceActivationEdge,
    PhysicalTransitionReturn,
    ConditionalReflectedHandRelease,
    CompleteComposition,
}

impl Arm {
    pub const ALL: [Self; 5] = [
        Self::ExistingLineageDiscriminator,
        Self::StableSurfaceActivationEdge,
        Self::PhysicalTransitionReturn,
        Self::ConditionalReflectedHandRelease,
        Self::CompleteComposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::ExistingLineageDiscriminator => "existing-lineage-discriminator",
            Self::StableSurfaceActivationEdge => "stable-surface-activation-edge",
            Self::PhysicalTransitionReturn => "physical-transition-return",
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
    transition_inputs: usize,
    sample_inputs: usize,
    matched_sampled_and_transitioned_origins: BTreeSet<u64>,
    admitted_transition_origins: BTreeSet<u64>,
    rejected_sample_origins: BTreeSet<u64>,
    unchanged_sample_rejections: usize,
    stable_transition_set_repetitions: usize,
    upper_clamp_decisions: Vec<BoundaryDecision>,
    propagation_budget_exhaustions: u64,
    exact_replay: bool,
    naturally_quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct BoundaryDecision {
    step: usize,
    emitted_outputs: Vec<u64>,
    return_scheduling: u64,
    return_admissions: u64,
    rejected_returns: u64,
    consequence_writes: usize,
    owner_reads: u64,
    consequential_owner_reads: u64,
    return_origins: Vec<ReturnOriginEvidence>,
    candidate_selection: Vec<CandidateSelectionEvidence>,
    output_candidates: Vec<OutputCandidateEvidence>,
}

fn summarize(hand: &ReflectedHandProtocolEvidence) -> HandSummary {
    let transitioned = hand
        .trajectory
        .iter()
        .flat_map(|step| &step.physical_incidences)
        .filter(|event| event.incidence == PhysicalIncidence::Transition)
        .map(|event| event.origin_physical)
        .collect::<BTreeSet<_>>();
    let sampled = hand
        .trajectory
        .iter()
        .flat_map(|step| &step.physical_incidences)
        .filter(|event| event.incidence == PhysicalIncidence::Sample)
        .map(|event| event.origin_physical)
        .collect::<BTreeSet<_>>();
    let transition_sets = hand
        .trajectory
        .iter()
        .map(|step| {
            step.physical_incidences
                .iter()
                .filter(|event| event.incidence == PhysicalIncidence::Transition)
                .map(|event| event.origin_physical)
                .collect::<BTreeSet<_>>()
        })
        .filter(|origins| !origins.is_empty())
        .collect::<Vec<_>>();
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
        transition_inputs: hand
            .trajectory
            .iter()
            .flat_map(|step| &step.physical_incidences)
            .filter(|event| event.incidence == PhysicalIncidence::Transition)
            .count(),
        sample_inputs: hand
            .trajectory
            .iter()
            .flat_map(|step| &step.physical_incidences)
            .filter(|event| event.incidence == PhysicalIncidence::Sample)
            .count(),
        matched_sampled_and_transitioned_origins: transitioned
            .intersection(&sampled)
            .copied()
            .collect(),
        admitted_transition_origins: hand
            .trajectory
            .iter()
            .flat_map(|step| &step.transition_eligibility)
            .filter(|event| event.eligible && event.transition_tick.is_some())
            .map(|event| event.origin_physical)
            .collect(),
        rejected_sample_origins: hand
            .trajectory
            .iter()
            .flat_map(|step| &step.transition_eligibility)
            .filter(|event| !event.eligible && event.transition_tick.is_none())
            .map(|event| event.origin_physical)
            .collect(),
        unchanged_sample_rejections: hand
            .trajectory
            .iter()
            .flat_map(|step| &step.return_origins)
            .filter(|event| event.decision == "rejected-unchanged-sample")
            .count(),
        stable_transition_set_repetitions: transition_sets
            .windows(2)
            .filter(|pair| pair[0] == pair[1])
            .count(),
        upper_clamp_decisions: hand
            .trajectory
            .iter()
            .filter(|step| step.position_before == 4 && step.position_after == 4)
            .map(|step| BoundaryDecision {
                step: step.index,
                emitted_outputs: step.emitted_outputs.clone(),
                return_scheduling: step.return_scheduling,
                return_admissions: step.return_admissions,
                rejected_returns: step.rejected_returns,
                consequence_writes: step.consequence_writes.len(),
                owner_reads: step.owner_reads,
                consequential_owner_reads: step.consequential_owner_reads,
                return_origins: step.return_origins.clone(),
                candidate_selection: step.candidate_selection.clone(),
                output_candidates: step
                    .output_candidates
                    .iter()
                    .filter(|candidate| candidate.is_motor)
                    .cloned()
                    .collect(),
            })
            .collect(),
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
            Protocol::RecursiveLearnerPhysicalTransitionReturn,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            MAX_MOMENTS_PER_SEND,
        )),
        parent: summarize(&run_reflected_hand_bounded(
            Protocol::RecursiveLearnerConsequenceBornReturn,
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

fn result(
    arm: Arm,
    survived: bool,
    observations: serde_json::Value,
    falsifier: &'static str,
    evidence: &Evidence,
) -> ProbeResult {
    ProbeResult {
        schema: "hand-physical-change-resampling/v1",
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
        Arm::ExistingLineageDiscriminator => result(
            arm,
            evidence.parent.unchanged_sample_rejections > 0,
            serde_json::json!({ "parent": evidence.parent }),
            "birth timing and origin identity still admitted later unchanged samples",
            evidence,
        ),
        Arm::StableSurfaceActivationEdge => result(
            arm,
            evidence.candidate.stable_transition_set_repetitions == 0,
            serde_json::json!({
                "candidate": evidence.candidate,
                "aliased_repetitions": evidence.candidate.stable_transition_set_repetitions,
            }),
            "real movement repeated the same stable active-origin set",
            evidence,
        ),
        Arm::PhysicalTransitionReturn => {
            let survived = evidence.candidate.transition_inputs > 0
                && evidence.candidate.sample_inputs > 0
                && !evidence
                    .candidate
                    .matched_sampled_and_transitioned_origins
                    .is_empty()
                && !evidence.candidate.admitted_transition_origins.is_empty()
                && !evidence.candidate.rejected_sample_origins.is_empty()
                && evidence.candidate.unchanged_sample_rejections > 0
                && evidence.candidate.propagation_budget_exhaustions == 0
                && evidence.candidate.exact_replay
                && evidence.candidate.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({ "candidate": evidence.candidate, "parent": evidence.parent }),
                "the same anonymous origins were not admitted on transition and rejected on sampling",
                evidence,
            )
        }
        Arm::ConditionalReflectedHandRelease => {
            let survived = evidence.candidate.reached_upper
                && evidence.candidate.escaped_upper
                && evidence.candidate.directions.len() == 2
                && evidence.candidate.emitted_physical.len() == 2
                && evidence.candidate.actual_position_changes
                    > evidence.parent.actual_position_changes
                && evidence.candidate.propagation_budget_exhaustions == 0
                && evidence.candidate.exact_replay
                && evidence.candidate.naturally_quiescent;
            result(
                arm,
                survived,
                serde_json::json!({ "candidate": evidence.candidate, "parent": evidence.parent }),
                "physical transition admission did not safely expose and execute the opposite motor",
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
                serde_json::json!({ "candidate": evidence.candidate, "parent": evidence.parent }),
                "safe boundary release did not compose into complete reflected-joint control",
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
    fn existing_lineage_and_stable_surface_arms_preserve_counterexamples() {
        let lineage = run(Arm::ExistingLineageDiscriminator);
        let surface = run(Arm::StableSurfaceActivationEdge);
        assert_eq!(lineage.outcome, "falsified", "{lineage:#?}");
        assert_eq!(surface.outcome, "falsified", "{surface:#?}");
    }

    #[test]
    fn physical_transition_discriminator_survives_matched_origins() {
        let observed = run(Arm::PhysicalTransitionReturn);
        assert_eq!(observed.outcome, "survived", "{observed:#?}");
    }

    #[test]
    fn release_and_complete_composition_follow_frozen_predicates() {
        let release = run(Arm::ConditionalReflectedHandRelease);
        let complete = run(Arm::CompleteComposition);
        assert!(matches!(release.outcome, "survived" | "falsified"));
        assert!(matches!(complete.outcome, "survived" | "falsified"));
        assert!(release.exact_replay && complete.exact_replay);
        assert!(release.naturally_quiescent && complete.naturally_quiescent);
    }
}
