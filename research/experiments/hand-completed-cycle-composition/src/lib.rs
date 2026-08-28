#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    CompletedCycleContinuationEvidence, EffectComposition, ReflectedHandProtocolEvidence,
    run_reflected_hand_bounded,
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
    CompletedCycleSuccessor,
    LifecycleControls,
    OfficialBatchedHand,
    CompleteComposition,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::CompletedCycleSuccessor,
        Self::LifecycleControls,
        Self::OfficialBatchedHand,
        Self::CompleteComposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::CompletedCycleSuccessor => "completed-cycle-successor",
            Self::LifecycleControls => "lifecycle-controls",
            Self::OfficialBatchedHand => "official-batched-hand",
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
    effect_composition: EffectComposition,
    actual_position_changes: usize,
    net_changed_steps: usize,
    directions: BTreeSet<i8>,
    emitted_physical: BTreeSet<u64>,
    opposing_output_steps: usize,
    reached_upper: bool,
    escaped_upper: bool,
    reached_lower: bool,
    escaped_lower: bool,
    final_position: i16,
    primary_closed: bool,
    perturbation_recovered: bool,
    completed_cycle_continuations: Vec<CompletedCycleContinuationEvidence>,
    completed_cycle_admissions: usize,
    completed_cycle_rejections: usize,
    cross_view_admissions: usize,
    propagation_budget_exhaustions: u64,
    stopped: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
}

impl HandSummary {
    fn integral(&self) -> bool {
        self.effect_composition == EffectComposition::Batched
            && !self.stopped
            && self.propagation_budget_exhaustions == 0
            && self.exact_replay
            && self.naturally_quiescent
    }

    fn improves_on(&self, parent: &Self) -> bool {
        self.actual_position_changes > parent.actual_position_changes
            || self.opposing_output_steps < parent.opposing_output_steps
    }
}

fn summarize(hand: ReflectedHandProtocolEvidence) -> HandSummary {
    let completed_cycle_continuations = hand
        .trajectory
        .iter()
        .flat_map(|step| step.completed_cycle_continuations.iter().cloned())
        .collect::<Vec<_>>();
    HandSummary {
        protocol: hand.protocol,
        effect_composition: hand.effect_composition,
        actual_position_changes: hand.actual_position_changes,
        net_changed_steps: hand
            .trajectory
            .iter()
            .filter(|step| step.position_before != step.position_after)
            .count(),
        directions: hand.directions,
        emitted_physical: hand
            .trajectory
            .iter()
            .flat_map(|step| step.emitted_outputs.iter().copied())
            .collect(),
        opposing_output_steps: hand
            .trajectory
            .iter()
            .filter(|step| {
                step.emitted_outputs.contains(&20_000) && step.emitted_outputs.contains(&20_001)
            })
            .count(),
        reached_upper: hand.reached_upper,
        escaped_upper: hand.escaped_upper,
        reached_lower: hand.reached_lower,
        escaped_lower: hand.escaped_lower,
        final_position: hand.final_position,
        primary_closed: hand.primary_closed,
        perturbation_recovered: hand.perturbation_recovered,
        completed_cycle_admissions: completed_cycle_continuations
            .iter()
            .filter(|decision| decision.admitted)
            .count(),
        completed_cycle_rejections: completed_cycle_continuations
            .iter()
            .filter(|decision| !decision.admitted)
            .count(),
        cross_view_admissions: completed_cycle_continuations
            .iter()
            .filter(|decision| decision.admitted && decision.crosses_ownership_view)
            .count(),
        completed_cycle_continuations,
        propagation_budget_exhaustions: hand
            .trajectory
            .iter()
            .map(|step| step.propagation_budget_exhaustions)
            .sum(),
        stopped: hand.stopped,
        exact_replay: hand.exact_replay,
        naturally_quiescent: hand.naturally_quiescent,
    }
}

fn measure_hand(protocol: Protocol) -> HandSummary {
    summarize(run_reflected_hand_bounded(
        protocol,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
    ))
}

#[derive(Clone, Debug)]
struct Evidence {
    candidate: HandSummary,
    coherent_parent: HandSummary,
    root_parent: HandSummary,
}

fn measure() -> Evidence {
    std::thread::scope(|scope| {
        let candidate = scope.spawn(|| measure_hand(Protocol::RecursiveLearnerCompletedCycle));
        let coherent_parent =
            scope.spawn(|| measure_hand(Protocol::RecursiveLearnerCoherentEffect));
        let root_parent =
            scope.spawn(|| measure_hand(Protocol::RecursiveLearnerRootFreshOpportunity));
        Evidence {
            candidate: candidate.join().expect("candidate arm completes"),
            coherent_parent: coherent_parent.join().expect("coherent parent completes"),
            root_parent: root_parent.join().expect("root parent completes"),
        }
    })
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

fn root_parent_exact(parent: &HandSummary) -> bool {
    parent.integral()
        && parent.actual_position_changes == 9
        && parent.opposing_output_steps == 6
        && parent.reached_upper
        && parent.escaped_upper
        && !parent.reached_lower
        && !parent.escaped_lower
        && parent.directions == BTreeSet::from([-1, 1])
        && parent.completed_cycle_continuations.is_empty()
}

fn coherent_parent_exact(parent: &HandSummary) -> bool {
    parent.integral()
        && parent.actual_position_changes == 11
        && parent.opposing_output_steps == 4
        && parent.reached_upper
        && parent.escaped_upper
        && !parent.reached_lower
        && !parent.escaped_lower
        && parent.directions == BTreeSet::from([-1, 1])
        && parent.completed_cycle_continuations.is_empty()
}

fn result(arm: Arm, survived: bool, falsifier: &'static str, evidence: &Evidence) -> ProbeResult {
    ProbeResult {
        schema: "hand-completed-cycle-composition/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations: serde_json::json!({
            "candidate": evidence.candidate,
            "coherent_parent": evidence.coherent_parent,
            "root_fresh_parent": evidence.root_parent,
        }),
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: evidence.candidate.exact_replay,
        naturally_quiescent: evidence.candidate.naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    let parents_exact = coherent_parent_exact(&evidence.coherent_parent)
        && root_parent_exact(&evidence.root_parent);
    match arm {
        Arm::CompletedCycleSuccessor => result(
            arm,
            parents_exact
                && evidence.candidate.integral()
                && evidence.candidate.completed_cycle_admissions > 0
                && evidence.candidate.cross_view_admissions > 0,
            "no unique recent completed cycle was admitted across an ownership view",
            evidence,
        ),
        Arm::LifecycleControls => result(
            arm,
            parents_exact
                && evidence.candidate.integral()
                && evidence.candidate.completed_cycle_rejections > 0,
            "protocol isolation, ambiguity/release evidence, or an integrity control failed",
            evidence,
        ),
        Arm::OfficialBatchedHand => result(
            arm,
            parents_exact
                && evidence.candidate.integral()
                && evidence.candidate.emitted_physical == BTreeSet::from([20_000, 20_001])
                && evidence.candidate.reached_upper
                && evidence.candidate.escaped_upper
                && evidence.candidate.improves_on(&evidence.coherent_parent),
            "completed-cycle composition did not improve the official batched hand over its coherent parent",
            evidence,
        ),
        Arm::CompleteComposition => result(
            arm,
            parents_exact
                && evidence.candidate.integral()
                && evidence.candidate.primary_closed
                && evidence.candidate.reached_upper
                && evidence.candidate.escaped_upper
                && evidence.candidate.reached_lower
                && evidence.candidate.escaped_lower
                && evidence.candidate.perturbation_recovered,
            "the hand did not reach and leave both limits and recover from perturbation",
            evidence,
        ),
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}
