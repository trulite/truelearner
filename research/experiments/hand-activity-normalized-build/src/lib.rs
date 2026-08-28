#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    ReflectedHandProtocolEvidence, run_reflected_hand_bounded,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, Link, Protocol, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const JUNCTION_CAPACITY: u32 = 512;
const LINK_CAPACITY: u32 = 2_048;
const MAX_MOMENTS_PER_SEND: u64 = 256;

const PREDECESSOR_HASHES: [(&str, &str); 7] = [
    (
        "attribution-conservation.json",
        "0931453840810efb426703cacc365c65adef4ea6d1aaff7b29d1f5e4b62dd3f1",
    ),
    (
        "boundary-input-activity.json",
        "036d0102636dc863e137a86954dc1d87a9918c850ce0b065f810fb30fad084b1",
    ),
    (
        "finite-activity-decision.json",
        "bb36dfc38f090ccb986f7f83e808400b13a8ce7b439c0ed426f8b0f01b5b2b1f",
    ),
    (
        "learner-construction-activity.json",
        "f1584a924db8269f1d0fe24198d33a8eb2a148e9cbb14106c0343dd707d3d5fa",
    ),
    (
        "evidence.json",
        "641c939772ce19bec962d71f80638493e9cfdde81889a665d0fc8b01fc3584c0",
    ),
    (
        "adjudication.toml",
        "91670c66a6d899be93a062a5c96b388f08381e29226bcc646e26d5792b61c0e7",
    ),
    (
        "convergence.toml",
        "6e97b6268a4ca45f0992454377f31ef6ac0d57f2c174ac566afe7a2013a5dc77",
    ),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    ActivityNormalizedScaling,
    ParentBehaviorPreservation,
    ReturnBearingComposition,
    CompleteOneJoint,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::ActivityNormalizedScaling,
        Self::ParentBehaviorPreservation,
        Self::ReturnBearingComposition,
        Self::CompleteOneJoint,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::ActivityNormalizedScaling => "activity-normalized-scaling",
            Self::ParentBehaviorPreservation => "parent-behavior-preservation",
            Self::ReturnBearingComposition => "return-bearing-composition",
            Self::CompleteOneJoint => "complete-one-joint",
        }
    }
}

pub struct PredecessorBytes<'a> {
    pub files: [(&'static str, &'a [u8]); 7],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct DigestControl {
    name: &'static str,
    expected_sha256: &'static str,
    observed_sha256: String,
    matched: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ScalePoint {
    fanout: usize,
    batched_items: u64,
    batch_max: u64,
    comparisons: u64,
    scans: u64,
    comparisons_per_item_milli: u64,
    exact_replay: bool,
    naturally_quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct HandSummary {
    protocol: Protocol,
    positions: Vec<(i16, i16)>,
    phase_outputs: Vec<Vec<Vec<u64>>>,
    comparisons: u64,
    scans: u64,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    primary_closed: bool,
    perturbation_recovered: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    stopped: bool,
    propagation_budget_exhaustions: u64,
}

#[derive(Clone, Debug, Serialize)]
struct Evidence {
    predecessor_controls: Vec<DigestControl>,
    predecessor_controls_survived: bool,
    scaling: Vec<ScalePoint>,
    scaling_survived: bool,
    parent: HandSummary,
    parent_behavior_preserved: bool,
    candidate: HandSummary,
    step_six_composed: bool,
    step_nine_composed: bool,
    complete_one_joint: bool,
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

fn junction(
    builder: &mut HarnessBuilder,
    physical_id: u64,
    threshold: i32,
) -> truelearner_core::JunctionId {
    builder.add_junction(Junction {
        physical_id,
        position: 0,
        region: 0,
        threshold,
        resistance: u32::MAX,
    })
}

fn fanout_fixture(fanout: usize) -> (Harness, truelearner_core::JunctionId) {
    let capacity = u32::try_from(fanout.saturating_add(4)).unwrap_or(u32::MAX);
    let mut builder = HarnessBuilder::with_capacity(capacity, capacity, OUTWARD_REGION);
    builder.set_protocol(Protocol::Physical);
    let source = junction(&mut builder, 10_000, 1);
    for index in 0..fanout {
        let target = junction(&mut builder, 20_000 + index as u64, i32::MAX);
        builder.add_link(Link {
            from: source,
            to: target,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: u32::MAX,
            mode: TransmissionMode::Drive,
        });
    }
    (builder.build(), source)
}

fn scale_point(fanout: usize) -> ScalePoint {
    let (mut harness, source) = fanout_fixture(fanout);
    let checkpoint = harness.save().expect("scale checkpoint saves");
    let inputs = [Input {
        arrival_tick: 1,
        phase: 0,
        origin_physical: 10_000,
        target: source,
        impulse: 1,
    }];
    let run = harness.send(&inputs);
    let replay = Harness::restore(checkpoint)
        .expect("scale checkpoint restores")
        .send(&inputs);
    ScalePoint {
        fanout,
        batched_items: run.execution_cost.batched_items,
        batch_max: run.execution_cost.batch_max,
        comparisons: run.execution_cost.comparisons,
        scans: run.execution_cost.scans,
        comparisons_per_item_milli: run.execution_cost.comparisons.saturating_mul(1_000)
            / run.execution_cost.batched_items.max(1),
        exact_replay: run == replay,
        naturally_quiescent: run.naturally_quiescent,
    }
}

fn scaling_evidence() -> Vec<ScalePoint> {
    [4, 8, 16, 32, 64].into_iter().map(scale_point).collect()
}

fn scaling_survived(points: &[ScalePoint]) -> bool {
    points.iter().all(|point| {
        point.batch_max == point.fanout as u64
            && point.comparisons <= point.batched_items.saturating_mul(2)
            && point.scans <= point.batched_items.saturating_mul(24)
            && point.exact_replay
            && point.naturally_quiescent
    }) && points.windows(2).all(|pair| {
        pair[1].comparisons_per_item_milli
            <= pair[0].comparisons_per_item_milli.saturating_add(1_000)
    })
}

fn summarize(hand: &ReflectedHandProtocolEvidence) -> HandSummary {
    HandSummary {
        protocol: hand.protocol,
        positions: hand
            .trajectory
            .iter()
            .map(|step| (step.position_before, step.position_after))
            .collect(),
        phase_outputs: hand
            .trajectory
            .iter()
            .map(|step| {
                step.phase_work
                    .iter()
                    .map(|phase| phase.emitted_outputs.clone())
                    .collect()
            })
            .collect(),
        comparisons: hand.comparisons,
        scans: hand.scans,
        reached_lower: hand.reached_lower,
        reached_upper: hand.reached_upper,
        escaped_lower: hand.escaped_lower,
        escaped_upper: hand.escaped_upper,
        primary_closed: hand.primary_closed,
        perturbation_recovered: hand.perturbation_recovered,
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

fn parent_behavior_preserved(parent: &HandSummary) -> bool {
    parent.protocol == Protocol::RecursiveLearnerBoundedConstructionContinuation
        && parent.positions
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
        && parent.exact_replay
        && parent.naturally_quiescent
        && !parent.stopped
        && parent.propagation_budget_exhaustions == 0
}

fn one_direction(step: &[Vec<u64>]) -> bool {
    let mut outputs = step.iter().flatten().copied();
    let Some(first) = outputs.next() else {
        return false;
    };
    outputs.all(|output| output == first)
}

fn complete_one_joint(candidate: &HandSummary) -> bool {
    candidate.protocol == Protocol::RecursiveLearnerReturnBearingContinuation
        && candidate.reached_lower
        && candidate.reached_upper
        && candidate.escaped_lower
        && candidate.escaped_upper
        && candidate.primary_closed
        && candidate.perturbation_recovered
        && candidate.exact_replay
        && candidate.naturally_quiescent
        && !candidate.stopped
        && candidate.propagation_budget_exhaustions == 0
}

fn digest_controls(predecessor: &PredecessorBytes<'_>) -> Vec<DigestControl> {
    PREDECESSOR_HASHES
        .into_iter()
        .zip(predecessor.files)
        .map(|((expected_name, expected), (name, bytes))| {
            let observed_sha256 = format!("{:x}", Sha256::digest(bytes));
            DigestControl {
                name,
                expected_sha256: expected,
                matched: name == expected_name && observed_sha256 == expected,
                observed_sha256,
            }
        })
        .collect()
}

pub fn run_all(predecessor: &PredecessorBytes<'_>) -> Vec<(Arm, ProbeResult)> {
    let predecessor_controls = digest_controls(predecessor);
    let predecessor_controls_survived = predecessor_controls.iter().all(|item| item.matched);
    let scaling = scaling_evidence();
    let scaling_survived = scaling_survived(&scaling);
    let parent = summarize(&run_reflected_hand_bounded(
        Protocol::RecursiveLearnerBoundedConstructionContinuation,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    ));
    let candidate = summarize(&run_reflected_hand_bounded(
        Protocol::RecursiveLearnerReturnBearingContinuation,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    ));
    let parent_behavior_preserved = parent_behavior_preserved(&parent);
    let step_six_composed = candidate.phase_outputs.get(6).is_some_and(|step| {
        one_direction(step) && candidate.positions[6].0 != candidate.positions[6].1
    });
    let step_nine_composed = candidate.phase_outputs.get(9).is_some_and(|step| {
        one_direction(step) && candidate.positions[9].0 != candidate.positions[9].1
    });
    let complete_one_joint = complete_one_joint(&candidate);
    let evidence = Evidence {
        predecessor_controls,
        predecessor_controls_survived,
        scaling,
        scaling_survived,
        parent,
        parent_behavior_preserved,
        candidate,
        step_six_composed,
        step_nine_composed,
        complete_one_joint,
    };

    Arm::ALL
        .into_iter()
        .map(|arm| {
            let survived = match arm {
                Arm::ActivityNormalizedScaling => {
                    evidence.predecessor_controls_survived && evidence.scaling_survived
                }
                Arm::ParentBehaviorPreservation => evidence.parent_behavior_preserved,
                Arm::ReturnBearingComposition => {
                    evidence.step_six_composed && evidence.step_nine_composed
                }
                Arm::CompleteOneJoint => evidence.complete_one_joint,
            };
            let falsifier = match arm {
                Arm::ActivityNormalizedScaling => "comparison or scan cost is superlinear in scheduled causal activity, replay differs, or predecessor lineage changed",
                Arm::ParentBehaviorPreservation => "the opt-in parent hand behavior, replay, quiescence, or exhaustion changed",
                Arm::ReturnBearingComposition => "either known opposing-output cancellation remains or the step does not move",
                Arm::CompleteOneJoint => "the joint fails either limit, either escape, perturbation recovery, replay, quiescence, or bounded execution",
            };
            (arm, ProbeResult {
                schema: "hand-activity-normalized-build/v1",
                arm: arm.id(),
                outcome: if survived { "survived" } else { "falsified" },
                observations: serde_json::to_value(&evidence).expect("evidence serializes"),
                falsifier: (!survived).then(|| falsifier.to_string()),
                exact_replay: evidence.parent.exact_replay && evidence.candidate.exact_replay,
                naturally_quiescent: evidence.parent.naturally_quiescent && evidence.candidate.naturally_quiescent,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_normalized_scaling_is_linear_and_replayable() {
        let points = scaling_evidence();
        assert!(scaling_survived(&points), "{points:#?}");
    }

    #[test]
    fn complete_return_bearing_hand_closes_and_recovers() {
        let hand = summarize(&run_reflected_hand_bounded(
            Protocol::RecursiveLearnerReturnBearingContinuation,
            JUNCTION_CAPACITY,
            LINK_CAPACITY,
            MAX_MOMENTS_PER_SEND,
        ));
        assert!(complete_one_joint(&hand), "{hand:#?}");
        assert!(one_direction(&hand.phase_outputs[6]));
        assert!(one_direction(&hand.phase_outputs[9]));
    }
}
