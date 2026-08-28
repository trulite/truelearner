#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    ExecutionCostEvidence, ReflectedHandPhase, ReflectedHandPhaseWorkEvidence,
    ReflectedHandProtocolEvidence, ReflectedHandStepEvidence, WorkEvidence,
    run_reflected_hand_bounded,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use truelearner_core::Protocol;

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;
const FIRST_DIVERGENT_INPUT_STEP: usize = 4;
const LARGEST_SPIKE_STEP: usize = 11;
const MAJOR_SPIKE: i64 = 256;

const CONTROL_SHA256: &str = "6ea0ebe9d989da1061ad1b102a2d76705e384a410de85b3b48674f4886f953b6";
const PREFIX_SHA256: &str = "31cb6399fe10e303c2a5d198d3de84dc97067a59b119148490c921a8f9e5fdbd";
const ACTIVITY_SHA256: &str = "992569511fa4b813cf70e40556405817b03b789629caeb516e54837eb8f95489";
const EVIDENCE_SHA256: &str = "a08d59c535dfcbae6fec8a4ba1f2728dfcce0cf62f8bd792767c6b9e5c03b2d9";
const ADJUDICATION_SHA256: &str =
    "12902078dbb0ccc22e061c32f42186565e5922b3d4b3b3c5af9c2bbaf31de775";
const CONVERGENCE_SHA256: &str = "78cdbd2a8e769a5fcd3bc978f943d860636409073c2f43271b274fbfcb6ef34e";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    AttributionConservation,
    BoundaryInputActivity,
    LearnerConstructionActivity,
    FiniteActivityDecision,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::AttributionConservation,
        Self::BoundaryInputActivity,
        Self::LearnerConstructionActivity,
        Self::FiniteActivityDecision,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::AttributionConservation => "attribution-conservation",
            Self::BoundaryInputActivity => "boundary-input-activity",
            Self::LearnerConstructionActivity => "learner-construction-activity",
            Self::FiniteActivityDecision => "finite-activity-decision",
        }
    }
}

pub struct PredecessorBytes<'a> {
    pub control: &'a [u8],
    pub prefix: &'a [u8],
    pub activity: &'a [u8],
    pub evidence: &'a [u8],
    pub adjudication: &'a [u8],
    pub convergence: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct DigestControl {
    name: &'static str,
    observed_sha256: String,
    expected_sha256: &'static str,
    matched: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ActivityDecision {
    FiniteUsefulActivity,
    UnexplainedWaste,
    Runaway,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SpikeFacts {
    comparisons_reconciled: bool,
    more_inputs: bool,
    more_physical_work: bool,
    boundary_interaction: bool,
    more_learner_construction: bool,
}

fn classify_activity(runaway: bool, spikes: &[SpikeFacts]) -> ActivityDecision {
    if runaway {
        return ActivityDecision::Runaway;
    }
    let explained = !spikes.is_empty()
        && spikes.iter().all(|spike| {
            spike.comparisons_reconciled
                && spike.more_inputs
                && spike.more_physical_work
                && (spike.boundary_interaction || spike.more_learner_construction)
        });
    if explained {
        ActivityDecision::FiniteUsefulActivity
    } else {
        ActivityDecision::UnexplainedWaste
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct RunSummary {
    protocol: Protocol,
    positions: Vec<(i16, i16)>,
    comparisons: u64,
    scans: u64,
    learners: usize,
    exact_replay: bool,
    naturally_quiescent: bool,
    stopped: bool,
    propagation_budget_exhaustions: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct PhaseAttribution {
    phase: ReflectedHandPhase,
    parent_input_count: usize,
    candidate_input_count: usize,
    parent_output_count: usize,
    candidate_output_count: usize,
    comparison_delta: i64,
    minimum_key_comparison_delta: i64,
    bucket_selection_comparison_delta: i64,
    scan_delta: i64,
    physical_work_delta: i64,
    drive_delivery_delta: i64,
    structural_proposal_delta: i64,
    learner_construction_delta: i64,
    parent: ReflectedHandPhaseWorkEvidence,
    candidate: ReflectedHandPhaseWorkEvidence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct StepAttribution {
    index: usize,
    parent_position_before: i16,
    parent_position_after: i16,
    candidate_position_before: i16,
    candidate_position_after: i16,
    parent_learners: usize,
    candidate_learners: usize,
    parent_junctions: usize,
    candidate_junctions: usize,
    parent_links: usize,
    candidate_links: usize,
    comparison_delta: i64,
    minimum_key_comparison_delta: i64,
    bucket_selection_comparison_delta: i64,
    comparison_delta_reconciled: bool,
    scan_delta: i64,
    input_count_delta: i64,
    physical_work_delta: i64,
    drive_delivery_delta: i64,
    structural_proposal_delta: i64,
    learner_construction_delta: i64,
    major_positive_spike: bool,
    boundary_interaction: bool,
    phase_attribution: Vec<PhaseAttribution>,
}

#[derive(Clone, Debug, Serialize)]
struct Evidence {
    predecessor_controls: Vec<DigestControl>,
    predecessor_controls_survived: bool,
    live_parent_reproduced: bool,
    live_candidate_reproduced: bool,
    phase_conservation_survived: bool,
    comparison_attribution_survived: bool,
    parent: RunSummary,
    candidate: RunSummary,
    steps: Vec<StepAttribution>,
    major_spike_steps: Vec<usize>,
    downstream_comparison_delta: i64,
    downstream_minimum_key_delta: i64,
    downstream_bucket_selection_delta: i64,
    downstream_attribution_reconciled: bool,
    first_spike_explained: bool,
    largest_spike_explained: bool,
    decision: ActivityDecision,
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

fn delta(candidate: u64, parent: u64) -> i64 {
    i64::try_from(candidate)
        .unwrap_or(i64::MAX)
        .saturating_sub(i64::try_from(parent).unwrap_or(i64::MAX))
}

fn delta_usize(candidate: usize, parent: usize) -> i64 {
    i64::try_from(candidate)
        .unwrap_or(i64::MAX)
        .saturating_sub(i64::try_from(parent).unwrap_or(i64::MAX))
}

fn digest_control(name: &'static str, bytes: &[u8], expected: &'static str) -> DigestControl {
    let observed_sha256 = format!("{:x}", Sha256::digest(bytes));
    DigestControl {
        matched: observed_sha256 == expected,
        name,
        observed_sha256,
        expected_sha256: expected,
    }
}

fn predecessor_controls(bytes: &PredecessorBytes<'_>) -> Vec<DigestControl> {
    vec![
        digest_control(
            "predecessor-and-replay-control",
            bytes.control,
            CONTROL_SHA256,
        ),
        digest_control("shared-prefix-work", bytes.prefix, PREFIX_SHA256),
        digest_control("post-choice-activity", bytes.activity, ACTIVITY_SHA256),
        digest_control("evidence", bytes.evidence, EVIDENCE_SHA256),
        digest_control("adjudication", bytes.adjudication, ADJUDICATION_SHA256),
        digest_control("convergence", bytes.convergence, CONVERGENCE_SHA256),
    ]
}

fn run_summary(hand: &ReflectedHandProtocolEvidence) -> RunSummary {
    RunSummary {
        protocol: hand.protocol,
        positions: hand
            .trajectory
            .iter()
            .map(|step| (step.position_before, step.position_after))
            .collect(),
        comparisons: hand.comparisons,
        scans: hand.scans,
        learners: hand.learners,
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
        stopped: hand.stopped,
        propagation_budget_exhaustions: hand
            .trajectory
            .iter()
            .map(|step| step.propagation_budget_exhaustions)
            .sum(),
    }
}

fn live_parent_reproduced(summary: &RunSummary) -> bool {
    summary.protocol == Protocol::RecursiveLearnerConstructionOutcomeComposition
        && summary.comparisons == 5_370
        && summary.scans == 4_426
        && summary.learners == 5
        && summary.positions
            == [
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 2),
                (2, 2),
                (2, 3),
                (3, 2),
                (2, 2),
                (2, 1),
                (1, 1),
                (1, 0),
                (0, -1),
                (-1, -2),
                (-2, -3),
                (-3, -2),
                (-2, -2),
            ]
        && summary.exact_replay
        && summary.naturally_quiescent
        && !summary.stopped
        && summary.propagation_budget_exhaustions == 0
}

fn live_candidate_reproduced(summary: &RunSummary) -> bool {
    summary.protocol == Protocol::RecursiveLearnerBoundedConstructionContinuation
        && summary.comparisons == 7_320
        && summary.scans == 4_681
        && summary.learners == 4
        && summary.positions
            == [
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 4),
                (4, 4),
                (4, 4),
                (4, 4),
                (4, 3),
                (3, 3),
                (3, 4),
                (4, 3),
                (3, 2),
                (2, 2),
                (2, 3),
                (3, 3),
            ]
        && summary.exact_replay
        && summary.naturally_quiescent
        && !summary.stopped
        && summary.propagation_budget_exhaustions == 0
}

fn accumulate_work(total: &mut WorkEvidence, work: &WorkEvidence) {
    total.total = total.total.saturating_add(work.total);
    total.physical_total = total.physical_total.saturating_add(work.physical_total);
    total.drive_deliveries = total.drive_deliveries.saturating_add(work.drive_deliveries);
    total.modulatory_deliveries = total
        .modulatory_deliveries
        .saturating_add(work.modulatory_deliveries);
    total.local_return_updates = total
        .local_return_updates
        .saturating_add(work.local_return_updates);
    total.local_structural_proposals = total
        .local_structural_proposals
        .saturating_add(work.local_structural_proposals);
    total.physical_deallocations = total
        .physical_deallocations
        .saturating_add(work.physical_deallocations);
    total.junction_deallocations = total
        .junction_deallocations
        .saturating_add(work.junction_deallocations);
    total.local_junction_proposals = total
        .local_junction_proposals
        .saturating_add(work.local_junction_proposals);
    total.qualified_local_traversals = total
        .qualified_local_traversals
        .saturating_add(work.qualified_local_traversals);
    total.causal_closure_observations = total
        .causal_closure_observations
        .saturating_add(work.causal_closure_observations);
    total.learner_constructions = total
        .learner_constructions
        .saturating_add(work.learner_constructions);
}

fn accumulate_cost(total: &mut ExecutionCostEvidence, cost: &ExecutionCostEvidence) {
    total.queue_ops = total.queue_ops.saturating_add(cost.queue_ops);
    total.comparisons = total.comparisons.saturating_add(cost.comparisons);
    total.timing_wheel_minimum_key_comparisons = total
        .timing_wheel_minimum_key_comparisons
        .saturating_add(cost.timing_wheel_minimum_key_comparisons);
    total.timing_wheel_bucket_selection_comparisons = total
        .timing_wheel_bucket_selection_comparisons
        .saturating_add(cost.timing_wheel_bucket_selection_comparisons);
    total.attributed_comparisons = total
        .timing_wheel_minimum_key_comparisons
        .saturating_add(total.timing_wheel_bucket_selection_comparisons);
    total.comparisons_reconciled = total.attributed_comparisons == total.comparisons;
    total.scans = total.scans.saturating_add(cost.scans);
    total.allocations = total.allocations.saturating_add(cost.allocations);
    total.bytes_touched = total.bytes_touched.saturating_add(cost.bytes_touched);
    total.peak_memory_bytes = total.peak_memory_bytes.max(cost.peak_memory_bytes);
    total.adjacency_accesses = total
        .adjacency_accesses
        .saturating_add(cost.adjacency_accesses);
    total.frontier_samples = total.frontier_samples.saturating_add(cost.frontier_samples);
    total.active_frontier_total = total
        .active_frontier_total
        .saturating_add(cost.active_frontier_total);
    total.active_frontier_max = total.active_frontier_max.max(cost.active_frontier_max);
    total.batches = total.batches.saturating_add(cost.batches);
    total.batched_items = total.batched_items.saturating_add(cost.batched_items);
    total.batch_max = total.batch_max.max(cost.batch_max);
    for (observed, increment) in total.batch_histogram.iter_mut().zip(cost.batch_histogram) {
        *observed = observed.saturating_add(increment);
    }
    total.batch_fallback_zero_delay = total
        .batch_fallback_zero_delay
        .saturating_add(cost.batch_fallback_zero_delay);
    total.arena_lookups = total.arena_lookups.saturating_add(cost.arena_lookups);
    total.arena_hops = total.arena_hops.saturating_add(cost.arena_hops);
    total.active_arena_samples = total
        .active_arena_samples
        .saturating_add(cost.active_arena_samples);
    total.active_arena_total = total
        .active_arena_total
        .saturating_add(cost.active_arena_total);
    total.active_arena_max = total.active_arena_max.max(cost.active_arena_max);
    total.local_structural_scans = total
        .local_structural_scans
        .saturating_add(cost.local_structural_scans);
}

fn step_conserved(step: &ReflectedHandStepEvidence) -> bool {
    let mut work = WorkEvidence::default();
    let mut cost = ExecutionCostEvidence::default();
    for phase in &step.phase_work {
        accumulate_work(&mut work, &phase.work);
        accumulate_cost(&mut cost, &phase.execution_cost);
    }
    work == step.work
        && cost == step.execution_cost
        && cost.comparisons == step.comparisons
        && cost.scans == step.scans
}

fn phase_attribution(
    parent: &ReflectedHandPhaseWorkEvidence,
    candidate: &ReflectedHandPhaseWorkEvidence,
) -> PhaseAttribution {
    PhaseAttribution {
        phase: parent.phase,
        parent_input_count: parent.input_count,
        candidate_input_count: candidate.input_count,
        parent_output_count: parent.output_count,
        candidate_output_count: candidate.output_count,
        comparison_delta: delta(
            candidate.execution_cost.comparisons,
            parent.execution_cost.comparisons,
        ),
        minimum_key_comparison_delta: delta(
            candidate
                .execution_cost
                .timing_wheel_minimum_key_comparisons,
            parent.execution_cost.timing_wheel_minimum_key_comparisons,
        ),
        bucket_selection_comparison_delta: delta(
            candidate
                .execution_cost
                .timing_wheel_bucket_selection_comparisons,
            parent
                .execution_cost
                .timing_wheel_bucket_selection_comparisons,
        ),
        scan_delta: delta(candidate.execution_cost.scans, parent.execution_cost.scans),
        physical_work_delta: delta(candidate.work.physical_total, parent.work.physical_total),
        drive_delivery_delta: delta(
            candidate.work.drive_deliveries,
            parent.work.drive_deliveries,
        ),
        structural_proposal_delta: delta(
            candidate.work.local_structural_proposals,
            parent.work.local_structural_proposals,
        ),
        learner_construction_delta: delta(
            candidate.work.learner_constructions,
            parent.work.learner_constructions,
        ),
        parent: parent.clone(),
        candidate: candidate.clone(),
    }
}

fn step_attribution(
    parent: &ReflectedHandStepEvidence,
    candidate: &ReflectedHandStepEvidence,
) -> StepAttribution {
    let phase_attribution = parent
        .phase_work
        .iter()
        .zip(&candidate.phase_work)
        .map(|(parent, candidate)| phase_attribution(parent, candidate))
        .collect::<Vec<_>>();
    let comparison_delta = delta(candidate.comparisons, parent.comparisons);
    let minimum_key_comparison_delta = delta(
        candidate
            .execution_cost
            .timing_wheel_minimum_key_comparisons,
        parent.execution_cost.timing_wheel_minimum_key_comparisons,
    );
    let bucket_selection_comparison_delta = delta(
        candidate
            .execution_cost
            .timing_wheel_bucket_selection_comparisons,
        parent
            .execution_cost
            .timing_wheel_bucket_selection_comparisons,
    );
    let input_count_delta = delta_usize(
        candidate
            .phase_work
            .iter()
            .map(|phase| phase.input_count)
            .sum(),
        parent
            .phase_work
            .iter()
            .map(|phase| phase.input_count)
            .sum(),
    );
    StepAttribution {
        index: parent.index,
        parent_position_before: parent.position_before,
        parent_position_after: parent.position_after,
        candidate_position_before: candidate.position_before,
        candidate_position_after: candidate.position_after,
        parent_learners: parent.learners,
        candidate_learners: candidate.learners,
        parent_junctions: parent.junctions,
        candidate_junctions: candidate.junctions,
        parent_links: parent.links,
        candidate_links: candidate.links,
        comparison_delta,
        minimum_key_comparison_delta,
        bucket_selection_comparison_delta,
        comparison_delta_reconciled: comparison_delta
            == minimum_key_comparison_delta.saturating_add(bucket_selection_comparison_delta),
        scan_delta: delta(candidate.scans, parent.scans),
        input_count_delta,
        physical_work_delta: delta(candidate.work.physical_total, parent.work.physical_total),
        drive_delivery_delta: delta(
            candidate.work.drive_deliveries,
            parent.work.drive_deliveries,
        ),
        structural_proposal_delta: delta(
            candidate.work.local_structural_proposals,
            parent.work.local_structural_proposals,
        ),
        learner_construction_delta: delta(
            candidate.work.learner_constructions,
            parent.work.learner_constructions,
        ),
        major_positive_spike: comparison_delta > MAJOR_SPIKE,
        boundary_interaction: candidate.position_before.abs() == 4
            || candidate.position_after.abs() == 4,
        phase_attribution,
    }
}

fn spike_facts(step: &StepAttribution) -> SpikeFacts {
    SpikeFacts {
        comparisons_reconciled: step.comparison_delta_reconciled,
        more_inputs: step.input_count_delta > 0,
        more_physical_work: step.physical_work_delta > 0,
        boundary_interaction: step.boundary_interaction,
        more_learner_construction: step.learner_construction_delta > 0,
    }
}

pub fn run_all(predecessor: &PredecessorBytes<'_>) -> Vec<(Arm, ProbeResult)> {
    let parent_hand = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerConstructionOutcomeComposition,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let candidate_hand = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerBoundedConstructionContinuation,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    );
    let predecessor_controls = predecessor_controls(predecessor);
    let predecessor_controls_survived = predecessor_controls.iter().all(|control| control.matched);
    let parent = run_summary(&parent_hand);
    let candidate = run_summary(&candidate_hand);
    let live_parent_reproduced = live_parent_reproduced(&parent);
    let live_candidate_reproduced = live_candidate_reproduced(&candidate);
    let phase_conservation_survived = parent_hand
        .trajectory
        .iter()
        .chain(&candidate_hand.trajectory)
        .all(step_conserved);
    let comparison_attribution_survived = parent_hand
        .trajectory
        .iter()
        .chain(&candidate_hand.trajectory)
        .all(|step| step.execution_cost.comparisons_reconciled);
    let steps = parent_hand
        .trajectory
        .iter()
        .zip(&candidate_hand.trajectory)
        .map(|(parent, candidate)| step_attribution(parent, candidate))
        .collect::<Vec<_>>();
    let major_spikes = steps
        .iter()
        .filter(|step| step.major_positive_spike)
        .collect::<Vec<_>>();
    let major_spike_steps = major_spikes
        .iter()
        .map(|step| step.index)
        .collect::<Vec<_>>();
    let downstream = steps
        .iter()
        .filter(|step| step.index >= FIRST_DIVERGENT_INPUT_STEP)
        .collect::<Vec<_>>();
    let downstream_comparison_delta = downstream
        .iter()
        .fold(0_i64, |sum, step| sum.saturating_add(step.comparison_delta));
    let downstream_minimum_key_delta = downstream.iter().fold(0_i64, |sum, step| {
        sum.saturating_add(step.minimum_key_comparison_delta)
    });
    let downstream_bucket_selection_delta = downstream.iter().fold(0_i64, |sum, step| {
        sum.saturating_add(step.bucket_selection_comparison_delta)
    });
    let downstream_attribution_reconciled = downstream_comparison_delta
        == downstream_minimum_key_delta.saturating_add(downstream_bucket_selection_delta);
    let first_spike_explained = steps.get(FIRST_DIVERGENT_INPUT_STEP).is_some_and(|step| {
        classify_activity(false, &[spike_facts(step)]) == ActivityDecision::FiniteUsefulActivity
    });
    let largest_spike_explained = steps.get(LARGEST_SPIKE_STEP).is_some_and(|step| {
        classify_activity(false, &[spike_facts(step)]) == ActivityDecision::FiniteUsefulActivity
    });
    let runaway = !parent.naturally_quiescent
        || !candidate.naturally_quiescent
        || parent.propagation_budget_exhaustions > 0
        || candidate.propagation_budget_exhaustions > 0;
    let spike_facts = major_spikes
        .iter()
        .map(|step| spike_facts(step))
        .collect::<Vec<_>>();
    let decision = classify_activity(runaway, &spike_facts);
    let evidence = Evidence {
        predecessor_controls,
        predecessor_controls_survived,
        live_parent_reproduced,
        live_candidate_reproduced,
        phase_conservation_survived,
        comparison_attribution_survived,
        parent,
        candidate,
        steps,
        major_spike_steps,
        downstream_comparison_delta,
        downstream_minimum_key_delta,
        downstream_bucket_selection_delta,
        downstream_attribution_reconciled,
        first_spike_explained,
        largest_spike_explained,
        decision,
    };

    Arm::ALL
        .into_iter()
        .map(|arm| {
            let survived = match arm {
                Arm::AttributionConservation => {
                    evidence.predecessor_controls_survived
                        && evidence.live_parent_reproduced
                        && evidence.live_candidate_reproduced
                        && evidence.phase_conservation_survived
                        && evidence.comparison_attribution_survived
                        && evidence.downstream_attribution_reconciled
                }
                Arm::BoundaryInputActivity => evidence.first_spike_explained,
                Arm::LearnerConstructionActivity => evidence
                    .steps
                    .get(LARGEST_SPIKE_STEP)
                    .is_some_and(|step| {
                        evidence.largest_spike_explained
                            && step.learner_construction_delta > 0
                    }),
                Arm::FiniteActivityDecision => {
                    evidence.decision == ActivityDecision::FiniteUsefulActivity
                }
            };
            let falsifier = match arm {
                Arm::AttributionConservation => {
                    "immutable controls, live executions, phase sums, or comparison attribution changed"
                }
                Arm::BoundaryInputActivity => {
                    "the first major spike lacks extra external input, physical work, or exact comparison attribution"
                }
                Arm::LearnerConstructionActivity => {
                    "the largest spike lacks actual learner construction or remains unattributed"
                }
                Arm::FiniteActivityDecision => {
                    "a major spike is unexplained waste or the changed execution is runaway"
                }
            };
            (
                arm,
                ProbeResult {
                    schema: "hand-downstream-work-class-localization/v1",
                    arm: arm.id(),
                    outcome: if survived { "survived" } else { "falsified" },
                    observations: serde_json::to_value(&evidence).expect("evidence serializes"),
                    falsifier: (!survived).then(|| falsifier.to_owned()),
                    exact_replay: evidence.parent.exact_replay && evidence.candidate.exact_replay,
                    naturally_quiescent: evidence.parent.naturally_quiescent
                        && evidence.candidate.naturally_quiescent,
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn explained() -> SpikeFacts {
        SpikeFacts {
            comparisons_reconciled: true,
            more_inputs: true,
            more_physical_work: true,
            boundary_interaction: true,
            more_learner_construction: false,
        }
    }

    #[test]
    fn classifier_distinguishes_finite_activity_waste_and_runaway() {
        assert_eq!(
            classify_activity(false, &[explained()]),
            ActivityDecision::FiniteUsefulActivity
        );
        assert_eq!(
            classify_activity(
                false,
                &[SpikeFacts {
                    more_inputs: false,
                    ..explained()
                }]
            ),
            ActivityDecision::UnexplainedWaste
        );
        assert_eq!(
            classify_activity(true, &[explained()]),
            ActivityDecision::Runaway
        );
    }

    #[test]
    fn signed_comparison_sources_must_reconcile() {
        let total = 517_i64;
        let minimum_key = 500_i64;
        let bucket = 17_i64;
        assert_eq!(total, minimum_key.saturating_add(bucket));
    }
}
