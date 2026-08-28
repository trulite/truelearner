#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    CoherentEffectEvidence, EffectComposition, ReflectedHandProtocolEvidence,
    run_reflected_hand_bounded_with_effect_composition,
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
    SequentialWorldComposition,
    CoherentUnresolvedEffect,
    CombinedComposition,
    FrozenBatchedParent,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::SequentialWorldComposition,
        Self::CoherentUnresolvedEffect,
        Self::CombinedComposition,
        Self::FrozenBatchedParent,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::SequentialWorldComposition => "sequential-world-composition",
            Self::CoherentUnresolvedEffect => "coherent-unresolved-effect",
            Self::CombinedComposition => "combined-composition",
            Self::FrozenBatchedParent => "frozen-batched-parent",
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
    phase_directions: Vec<i8>,
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
    coherent_effects: Vec<CoherentEffectEvidence>,
    coherent_admissions: usize,
    coherent_rejections: usize,
    propagation_budget_exhaustions: u64,
    stopped: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
}

impl HandSummary {
    fn integral(&self) -> bool {
        !self.stopped
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
    let coherent_effects = hand
        .trajectory
        .iter()
        .flat_map(|step| step.coherent_effects.iter().cloned())
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
        phase_directions: hand
            .trajectory
            .iter()
            .flat_map(|step| step.phase_directions.iter().copied())
            .collect(),
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
        coherent_admissions: coherent_effects
            .iter()
            .filter(|effect| effect.admitted)
            .count(),
        coherent_rejections: coherent_effects
            .iter()
            .filter(|effect| !effect.admitted)
            .count(),
        coherent_effects,
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

fn measure_hand(protocol: Protocol, effect_composition: EffectComposition) -> HandSummary {
    summarize(run_reflected_hand_bounded_with_effect_composition(
        protocol,
        JUNCTION_CAPACITY,
        LINK_CAPACITY,
        MAX_MOMENTS_PER_SEND,
        effect_composition,
    ))
}

#[derive(Clone, Debug)]
struct Evidence {
    sequential: HandSummary,
    coherent: HandSummary,
    combined: HandSummary,
    parent: HandSummary,
}

fn measure() -> Evidence {
    std::thread::scope(|scope| {
        let sequential = scope.spawn(|| {
            measure_hand(
                Protocol::RecursiveLearnerRootFreshOpportunity,
                EffectComposition::QuiescentPhaseSequential,
            )
        });
        let coherent = scope.spawn(|| {
            measure_hand(
                Protocol::RecursiveLearnerCoherentEffect,
                EffectComposition::Batched,
            )
        });
        let combined = scope.spawn(|| {
            measure_hand(
                Protocol::RecursiveLearnerCoherentEffect,
                EffectComposition::QuiescentPhaseSequential,
            )
        });
        let parent = scope.spawn(|| {
            measure_hand(
                Protocol::RecursiveLearnerRootFreshOpportunity,
                EffectComposition::Batched,
            )
        });
        Evidence {
            sequential: sequential.join().expect("sequential arm completes"),
            coherent: coherent.join().expect("coherent arm completes"),
            combined: combined.join().expect("combined arm completes"),
            parent: parent.join().expect("parent control completes"),
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

fn frozen_parent_exact(parent: &HandSummary) -> bool {
    parent.integral()
        && parent.effect_composition == EffectComposition::Batched
        && parent.actual_position_changes == 9
        && parent.opposing_output_steps == 6
        && parent.reached_upper
        && parent.escaped_upper
        && !parent.reached_lower
        && !parent.escaped_lower
        && parent.directions == BTreeSet::from([-1, 1])
}

fn result(arm: Arm, survived: bool, falsifier: &'static str, evidence: &Evidence) -> ProbeResult {
    let candidate = match arm {
        Arm::SequentialWorldComposition => &evidence.sequential,
        Arm::CoherentUnresolvedEffect => &evidence.coherent,
        Arm::CombinedComposition => &evidence.combined,
        Arm::FrozenBatchedParent => &evidence.parent,
    };
    ProbeResult {
        schema: "hand-effect-composition-laws/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations: serde_json::json!({
            "candidate": candidate,
            "frozen_batched_parent": evidence.parent,
        }),
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay: candidate.exact_replay,
        naturally_quiescent: candidate.naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    let parent_exact = frozen_parent_exact(&evidence.parent);
    match arm {
        Arm::SequentialWorldComposition => result(
            arm,
            parent_exact
                && evidence.sequential.integral()
                && evidence.sequential.improves_on(&evidence.parent),
            "sequential world composition did not improve travel or reduce opposing cancellation",
            evidence,
        ),
        Arm::CoherentUnresolvedEffect => result(
            arm,
            parent_exact
                && evidence.coherent.integral()
                && evidence.coherent.coherent_admissions > 0
                && evidence.coherent.coherent_rejections > 0
                && evidence.coherent.improves_on(&evidence.parent),
            "unresolved-effect coherence did not both admit and release or did not improve the hand trajectory",
            evidence,
        ),
        Arm::CombinedComposition => result(
            arm,
            parent_exact
                && evidence.combined.integral()
                && evidence.combined.reached_lower
                && evidence.combined.escaped_lower
                && evidence.combined.reached_upper
                && evidence.combined.escaped_upper
                && evidence.combined.primary_closed
                && evidence.combined.perturbation_recovered,
            "the composed laws did not produce complete reflected-joint control",
            evidence,
        ),
        Arm::FrozenBatchedParent => result(
            arm,
            parent_exact,
            "the frozen batched parent trajectory changed",
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
    fn frozen_parent_and_all_candidate_integrity_predicates_hold() {
        let measured = evidence();
        assert!(
            frozen_parent_exact(&measured.parent),
            "{:#?}",
            measured.parent
        );
        assert!(measured.sequential.integral());
        assert!(measured.coherent.integral());
        assert!(measured.combined.integral());
    }

    #[test]
    fn results_are_formal_survival_or_falsification() {
        for (_, observed) in run_all() {
            assert!(matches!(observed.outcome, "survived" | "falsified"));
            assert!(observed.exact_replay);
            assert!(observed.naturally_quiescent);
        }
    }
}
