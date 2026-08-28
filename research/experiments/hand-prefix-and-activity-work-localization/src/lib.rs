#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    ExistingWitnessEvent, OutputChoiceResolutionEvidence, ReflectedHandProtocolEvidence,
    ReflectedHandStepEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use truelearner_core::{JunctionId, LearnerId, LinkId, OutputChoiceBasis, Protocol};

const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;
const FIRST_REPAIRED_STEP: usize = 3;

const FIXTURES_SHA256: &str = "c2143100494b7b2839e14dc600faf1d79f76350ad03967fc4de9b82a125cfe0a";
const FIRST_WALL_SHA256: &str = "20dbcb67da31289cc053e5a0f6e44ece3f6a1fb921a2441d5e445c7430478c7d";
const CONTROLS_SHA256: &str = "32d6b8f657b271b2fbbd1227eab6978418671d27ea4ddfbbcd487f49b2293cfc";
const EVIDENCE_SHA256: &str = "65d8e225e042c20a87eb0d8df2cac2584838bea09d0f035ad83ba7a27ddeba84";
const ADJUDICATION_SHA256: &str =
    "1d38b2caf45a5d6062cd5090d25b6ac7738984c31093ae004c52315610c1e2bf";
const CONVERGENCE_SHA256: &str = "a2bf55a62aa9e6d06165bc118d6f1983b6d70c4c74e4b7c62a2c054fee489989";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    PredecessorAndReplayControl,
    SharedPrefixWork,
    PostChoiceActivity,
}

impl Arm {
    pub const ALL: [Self; 3] = [
        Self::PredecessorAndReplayControl,
        Self::SharedPrefixWork,
        Self::PostChoiceActivity,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::PredecessorAndReplayControl => "predecessor-and-replay-control",
            Self::SharedPrefixWork => "shared-prefix-work",
            Self::PostChoiceActivity => "post-choice-activity",
        }
    }
}

pub struct PredecessorBytes<'a> {
    pub fixtures: &'a [u8],
    pub first_wall: &'a [u8],
    pub controls: &'a [u8],
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct Consumption {
    tick: i64,
    target: JunctionId,
    owner: LearnerId,
    link: LinkId,
    generation: u32,
    consequence_tick: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct StepWork {
    index: usize,
    position_before: i16,
    position_after: i16,
    comparisons: u64,
    scans: u64,
    comparisons_per_live_link_milli: u64,
    scans_per_live_link_milli: u64,
    learners: usize,
    junctions: usize,
    live_links: usize,
    emitted_outputs: Vec<u64>,
    choices: Vec<OutputChoiceResolutionEvidence>,
    consumptions: Vec<Consumption>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Region {
    SharedPrefix,
    FirstRepairedChoice,
    DivergentTrajectory,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct AlignedStep {
    index: usize,
    region: Region,
    same_input_position: bool,
    same_output_position: bool,
    comparison_delta: i64,
    scan_delta: i64,
    parent: StepWork,
    candidate: StepWork,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
struct RegionWork {
    parent_comparisons: u64,
    candidate_comparisons: u64,
    comparison_delta: i64,
    parent_scans: u64,
    candidate_scans: u64,
    scan_delta: i64,
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
struct Localization {
    first_repaired_step: usize,
    first_output_position_divergence_step: Option<usize>,
    first_input_position_divergence_step: Option<usize>,
    first_comparison_divergence_step: Option<usize>,
    first_scan_divergence_step: Option<usize>,
    shared_prefix: RegionWork,
    first_repaired_choice: RegionWork,
    divergent_trajectory: RegionWork,
    aligned_steps: Vec<AlignedStep>,
    parent_step_sums_match: bool,
    candidate_step_sums_match: bool,
    shared_prefix_physics_and_work_match: bool,
    repaired_step_work_matches: bool,
    first_comparison_delta_has_divergent_input: bool,
    direct_law_overhead_supported: bool,
    trajectory_induced_work_supported: bool,
}

#[derive(Clone, Debug, Serialize)]
struct Evidence {
    predecessor_controls: Vec<DigestControl>,
    predecessor_controls_survived: bool,
    live_parent_reproduced: bool,
    live_candidate_reproduced: bool,
    parent: RunSummary,
    candidate: RunSummary,
    localization: Localization,
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
            "bounded-first-use-fixtures",
            bytes.fixtures,
            FIXTURES_SHA256,
        ),
        digest_control(
            "reflected-hand-first-wall",
            bytes.first_wall,
            FIRST_WALL_SHA256,
        ),
        digest_control(
            "parent-and-lifetime-controls",
            bytes.controls,
            CONTROLS_SHA256,
        ),
        digest_control("evidence", bytes.evidence, EVIDENCE_SHA256),
        digest_control("adjudication", bytes.adjudication, ADJUDICATION_SHA256),
        digest_control("convergence", bytes.convergence, CONVERGENCE_SHA256),
    ]
}

fn per_live_link_milli(work: u64, links: usize) -> u64 {
    let denominator = u64::try_from(links).unwrap_or(u64::MAX).max(1);
    work.saturating_mul(1_000) / denominator
}

fn step_work(step: &ReflectedHandStepEvidence) -> StepWork {
    let consumptions = step
        .existing_witness_trace
        .iter()
        .filter_map(|entry| match entry.event {
            ExistingWitnessEvent::ConstructionContinuationConsumed {
                target,
                owner,
                link,
                generation,
                consequence_tick,
            } => Some(Consumption {
                tick: entry.tick,
                target,
                owner,
                link,
                generation,
                consequence_tick,
            }),
            _ => None,
        })
        .collect();
    StepWork {
        index: step.index,
        position_before: step.position_before,
        position_after: step.position_after,
        comparisons: step.comparisons,
        scans: step.scans,
        comparisons_per_live_link_milli: per_live_link_milli(step.comparisons, step.links),
        scans_per_live_link_milli: per_live_link_milli(step.scans, step.links),
        learners: step.learners,
        junctions: step.junctions,
        live_links: step.links,
        emitted_outputs: step.emitted_outputs.clone(),
        choices: step.output_choice_resolutions.clone(),
        consumptions,
    }
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

fn delta(candidate: u64, parent: u64) -> i64 {
    let candidate = i64::try_from(candidate).unwrap_or(i64::MAX);
    let parent = i64::try_from(parent).unwrap_or(i64::MAX);
    candidate.saturating_sub(parent)
}

impl RegionWork {
    fn observe(&mut self, parent: &StepWork, candidate: &StepWork) {
        self.parent_comparisons = self.parent_comparisons.saturating_add(parent.comparisons);
        self.candidate_comparisons = self
            .candidate_comparisons
            .saturating_add(candidate.comparisons);
        self.parent_scans = self.parent_scans.saturating_add(parent.scans);
        self.candidate_scans = self.candidate_scans.saturating_add(candidate.scans);
        self.comparison_delta = delta(self.candidate_comparisons, self.parent_comparisons);
        self.scan_delta = delta(self.candidate_scans, self.parent_scans);
    }
}

fn localize(
    parent: &ReflectedHandProtocolEvidence,
    candidate: &ReflectedHandProtocolEvidence,
    first_repaired_step: usize,
) -> Localization {
    let aligned_steps = parent
        .trajectory
        .iter()
        .zip(&candidate.trajectory)
        .map(|(parent, candidate)| {
            let parent = step_work(parent);
            let candidate = step_work(candidate);
            let region = match parent.index.cmp(&first_repaired_step) {
                std::cmp::Ordering::Less => Region::SharedPrefix,
                std::cmp::Ordering::Equal => Region::FirstRepairedChoice,
                std::cmp::Ordering::Greater => Region::DivergentTrajectory,
            };
            AlignedStep {
                index: parent.index,
                region,
                same_input_position: parent.position_before == candidate.position_before,
                same_output_position: parent.position_after == candidate.position_after,
                comparison_delta: delta(candidate.comparisons, parent.comparisons),
                scan_delta: delta(candidate.scans, parent.scans),
                parent,
                candidate,
            }
        })
        .collect::<Vec<_>>();

    let mut shared_prefix = RegionWork::default();
    let mut first_repaired_choice = RegionWork::default();
    let mut divergent_trajectory = RegionWork::default();
    for step in &aligned_steps {
        match step.region {
            Region::SharedPrefix => shared_prefix.observe(&step.parent, &step.candidate),
            Region::FirstRepairedChoice => {
                first_repaired_choice.observe(&step.parent, &step.candidate);
            }
            Region::DivergentTrajectory => {
                divergent_trajectory.observe(&step.parent, &step.candidate);
            }
        }
    }

    let first_output_position_divergence_step = aligned_steps
        .iter()
        .find(|step| !step.same_output_position)
        .map(|step| step.index);
    let first_input_position_divergence_step = aligned_steps
        .iter()
        .find(|step| !step.same_input_position)
        .map(|step| step.index);
    let first_comparison_divergence_step = aligned_steps
        .iter()
        .find(|step| step.comparison_delta != 0)
        .map(|step| step.index);
    let first_scan_divergence_step = aligned_steps
        .iter()
        .find(|step| step.scan_delta != 0)
        .map(|step| step.index);
    let parent_step_sums_match = aligned_steps
        .iter()
        .map(|step| step.parent.comparisons)
        .sum::<u64>()
        == parent.comparisons
        && aligned_steps
            .iter()
            .map(|step| step.parent.scans)
            .sum::<u64>()
            == parent.scans;
    let candidate_step_sums_match = aligned_steps
        .iter()
        .map(|step| step.candidate.comparisons)
        .sum::<u64>()
        == candidate.comparisons
        && aligned_steps
            .iter()
            .map(|step| step.candidate.scans)
            .sum::<u64>()
            == candidate.scans;
    let shared_prefix_physics_and_work_match = aligned_steps
        .iter()
        .filter(|step| step.region == Region::SharedPrefix)
        .all(|step| {
            step.same_input_position
                && step.same_output_position
                && step.comparison_delta == 0
                && step.scan_delta == 0
                && step.parent.learners == step.candidate.learners
                && step.parent.junctions == step.candidate.junctions
                && step.parent.live_links == step.candidate.live_links
                && step.parent.emitted_outputs == step.candidate.emitted_outputs
        });
    let repaired_step_work_matches = aligned_steps
        .iter()
        .find(|step| step.region == Region::FirstRepairedChoice)
        .is_some_and(|step| {
            step.same_input_position && step.comparison_delta == 0 && step.scan_delta == 0
        });
    let first_comparison_delta_has_divergent_input = first_comparison_divergence_step
        .zip(first_input_position_divergence_step)
        .is_some_and(|(cost, input)| cost >= input);
    let direct_law_overhead_supported =
        first_comparison_divergence_step.is_some_and(|step| step <= first_repaired_step);
    let trajectory_induced_work_supported = shared_prefix_physics_and_work_match
        && repaired_step_work_matches
        && first_output_position_divergence_step == Some(first_repaired_step)
        && first_comparison_delta_has_divergent_input
        && divergent_trajectory.comparison_delta
            == delta(candidate.comparisons, parent.comparisons);

    Localization {
        first_repaired_step,
        first_output_position_divergence_step,
        first_input_position_divergence_step,
        first_comparison_divergence_step,
        first_scan_divergence_step,
        shared_prefix,
        first_repaired_choice,
        divergent_trajectory,
        aligned_steps,
        parent_step_sums_match,
        candidate_step_sums_match,
        shared_prefix_physics_and_work_match,
        repaired_step_work_matches,
        first_comparison_delta_has_divergent_input,
        direct_law_overhead_supported,
        trajectory_induced_work_supported,
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

fn repaired_choice_survived(localization: &Localization) -> bool {
    localization
        .aligned_steps
        .iter()
        .find(|step| step.index == FIRST_REPAIRED_STEP)
        .is_some_and(|step| {
            step.candidate.consumptions.iter().any(|consumption| {
                consumption.tick == 23
                    && consumption.target == JunctionId(11)
                    && consumption.owner == LearnerId(2)
                    && consumption.link == LinkId(36)
                    && consumption.generation == 1
                    && consumption.consequence_tick == 16
            }) && step.candidate.choices.iter().any(|choice| {
                choice.tick == 23
                    && choice.computed_winner_target == JunctionId(11)
                    && choice.computed_winner_basis == OutputChoiceBasis::CompletedCycle
            })
        })
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
    let localization = localize(&parent_hand, &candidate_hand, FIRST_REPAIRED_STEP);
    let evidence = Evidence {
        predecessor_controls,
        predecessor_controls_survived,
        live_parent_reproduced,
        live_candidate_reproduced,
        parent,
        candidate,
        localization,
    };

    Arm::ALL
        .into_iter()
        .map(|arm| {
            let survived = match arm {
                Arm::PredecessorAndReplayControl => {
                    evidence.predecessor_controls_survived
                        && evidence.live_parent_reproduced
                        && evidence.live_candidate_reproduced
                }
                Arm::SharedPrefixWork => {
                    evidence.localization.parent_step_sums_match
                        && evidence.localization.candidate_step_sums_match
                        && evidence.localization.shared_prefix_physics_and_work_match
                        && evidence.localization.repaired_step_work_matches
                        && repaired_choice_survived(&evidence.localization)
                }
                Arm::PostChoiceActivity => {
                    evidence.localization.trajectory_induced_work_supported
                        && !evidence.localization.direct_law_overhead_supported
                }
            };
            let falsifier = match arm {
                Arm::PredecessorAndReplayControl => {
                    "immutable predecessor or live replay changed under diagnostic retention"
                }
                Arm::SharedPrefixWork => {
                    "work diverged before or at first use, or per-step work did not preserve totals"
                }
                Arm::PostChoiceActivity => {
                    "comparison excess began before the physical input trajectory diverged"
                }
            };
            (
                arm,
                ProbeResult {
                    schema: "hand-prefix-and-activity-work-localization/v1",
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

    fn empty_hand(
        protocol: Protocol,
        work: &[(u64, u64, i16, i16)],
    ) -> ReflectedHandProtocolEvidence {
        let trajectory = work
            .iter()
            .enumerate()
            .map(
                |(index, &(comparisons, scans, position_before, position_after))| {
                    ReflectedHandStepEvidence {
                        index,
                        position_before,
                        position_after,
                        direction: 0,
                        phase_directions: vec![],
                        actual_position_changes: usize::from(position_before != position_after),
                    comparisons,
                    scans,
                    work: Default::default(),
                    execution_cost: Default::default(),
                    phase_work: vec![],
                        emitted_outputs: vec![],
                        delivered_surface_count: 0,
                        learners: 0,
                        junctions: 2,
                        links: 1,
                        return_scheduling: 0,
                        return_admissions: 0,
                        rejected_returns: 0,
                        reverse_consolidations: 0,
                        closure_observations: 0,
                        constructions: 0,
                        boundary_novelty_checks: 0,
                        boundary_novelty_rejections: 0,
                        owner_writes: 0,
                        owner_reads: 0,
                        consequential_owner_reads: 0,
                        surface_paths: vec![],
                        output_candidates: vec![],
                        fresh_opportunities: vec![],
                        fresh_opportunity_evaluations: vec![],
                        physical_transition_continuations: vec![],
                        coherent_effects: vec![],
                        completed_cycle_continuations: vec![],
                        output_choice_resolutions: vec![],
                        existing_witness_trace: vec![],
                        superseded_returns: vec![],
                        drive_provenance: vec![],
                        causal_origin_selection: vec![],
                        candidate_selection: vec![],
                        return_origins: vec![],
                        consequence_writes: vec![],
                        closure_eligibility: vec![],
                        physical_incidences: vec![],
                        transition_eligibility: vec![],
                        mixed_owner_checks: 0,
                        mixed_owner_selections: 0,
                        causal_origin_checks: 0,
                        causal_origin_selections: 0,
                        propagation_budget_exhaustions: 0,
                    }
                },
            )
            .collect::<Vec<_>>();
        ReflectedHandProtocolEvidence {
            protocol,
            effect_composition:
                developmental_hand_construction_admission::EffectComposition::Batched,
            steps: trajectory.len(),
            changed_steps: 0,
            actual_position_changes: 0,
            comparisons: trajectory.iter().map(|step| step.comparisons).sum(),
            scans: trajectory.iter().map(|step| step.scans).sum(),
            directions: Default::default(),
            reached_lower: false,
            reached_upper: false,
            escaped_lower: false,
            escaped_upper: false,
            final_position: trajectory.last().map_or(0, |step| step.position_after),
            learners: 0,
            closure_observations: 0,
            constructions: 0,
            primary_closed: false,
            perturbation_recovered: false,
            stopped: false,
            exact_replay: true,
            naturally_quiescent: true,
            trajectory,
        }
    }

    #[test]
    fn fold_separates_shared_repaired_and_divergent_regions() {
        let parent = empty_hand(
            Protocol::RecursiveLearnerConstructionOutcomeComposition,
            &[(10, 5, 0, 1), (20, 6, 1, 2), (30, 7, 2, 1), (40, 8, 1, 0)],
        );
        let candidate = empty_hand(
            Protocol::RecursiveLearnerBoundedConstructionContinuation,
            &[(10, 5, 0, 1), (20, 6, 1, 2), (30, 7, 2, 3), (55, 11, 3, 4)],
        );

        let observed = localize(&parent, &candidate, 2);

        assert_eq!(observed.first_output_position_divergence_step, Some(2));
        assert_eq!(observed.first_input_position_divergence_step, Some(3));
        assert_eq!(observed.first_comparison_divergence_step, Some(3));
        assert_eq!(observed.shared_prefix.comparison_delta, 0);
        assert_eq!(observed.first_repaired_choice.comparison_delta, 0);
        assert_eq!(observed.divergent_trajectory.comparison_delta, 15);
        assert!(observed.trajectory_induced_work_supported);
        assert!(!observed.direct_law_overhead_supported);
    }

    #[test]
    fn fold_exposes_direct_overhead_at_first_use() {
        let parent = empty_hand(
            Protocol::RecursiveLearnerConstructionOutcomeComposition,
            &[(10, 5, 0, 1), (20, 6, 1, 2)],
        );
        let candidate = empty_hand(
            Protocol::RecursiveLearnerBoundedConstructionContinuation,
            &[(10, 5, 0, 1), (21, 6, 1, 2)],
        );

        let observed = localize(&parent, &candidate, 1);

        assert_eq!(observed.first_comparison_divergence_step, Some(1));
        assert!(observed.direct_law_overhead_supported);
        assert!(!observed.trajectory_induced_work_supported);
    }

    #[test]
    fn normalization_is_total_for_zero_links() {
        assert_eq!(per_live_link_milli(7, 0), 7_000);
    }
}
