#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    ReflectedHandProtocolEvidence, run_reflected_hand_bounded, run_reflected_hand_with_protocol,
};
use serde::Serialize;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{
    Checkpoint, Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent,
    Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const MAX_MOMENTS_PER_SEND: u64 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    UnrelatedOriginTransfer,
    SameOriginMixedOwnerControl,
    BoundedReflectedHand,
    CompleteComposition,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::UnrelatedOriginTransfer,
        Self::SameOriginMixedOwnerControl,
        Self::BoundedReflectedHand,
        Self::CompleteComposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::UnrelatedOriginTransfer => "unrelated-origin-transfer",
            Self::SameOriginMixedOwnerControl => "same-origin-mixed-owner-control",
            Self::BoundedReflectedHand => "bounded-reflected-hand",
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

struct OriginFixture {
    harness: Harness,
    action: JunctionId,
    surfaces: [JunctionId; 2],
    motor: JunctionId,
}

impl OriginFixture {
    fn new(protocol: Protocol) -> Self {
        let mut builder = HarnessBuilder::with_capacity(96, 256, OUTWARD_REGION);
        builder.set_protocol(protocol);
        builder.set_physical_tracing(true);
        let action = junction(&mut builder, 80_000, 0, 0, 1);
        let surfaces = [
            junction(&mut builder, 80_001, 2, 0, 1),
            junction(&mut builder, 80_002, 2, 0, 1),
        ];
        let motor = junction(&mut builder, 80_010, 1, 0, 2);
        let sink = junction(&mut builder, 80_011, 1, OUTWARD_REGION, 1);
        let outcome = junction(&mut builder, 80_012, 50, 0, 1);
        let anchor = junction(&mut builder, 80_013, 100, 0, 99);
        for target in [action, surfaces[0], surfaces[1], outcome] {
            link(&mut builder, anchor, target, 0);
        }
        for surface in surfaces {
            link(&mut builder, surface, outcome, 3);
        }
        link(&mut builder, motor, sink, 0);
        builder.set_outcome_source_for_output(motor, outcome);
        Self {
            harness: builder.build(),
            action,
            surfaces,
            motor,
        }
    }

    fn restore(protocol: Protocol, checkpoint: Checkpoint) -> Self {
        let template = Self::new(protocol);
        Self {
            harness: Harness::restore(checkpoint).expect("fixture checkpoint restores"),
            ..template
        }
    }

    fn round(&mut self, index: usize) -> Vec<Run> {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let action = self.harness.send(&[
            input(self.action, tick, 80_000),
            input(self.motor, tick.saturating_add(2), 80_010),
        ]);
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let consequence =
            self.harness
                .send(&[input(self.surfaces[index], tick, 80_001 + index as u64)]);
        vec![action, consequence]
    }

    fn prepare(&mut self) -> Vec<Run> {
        let mut runs = Vec::new();
        for index in 0..2 {
            runs.extend(self.round(index));
            runs.extend(self.round(index));
        }
        runs.extend(self.round(0));
        runs.extend(self.round(0));
        runs
    }

    fn mixed(&mut self, origins: [u64; 2], reversed: bool) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let mut inputs = vec![
            input(self.surfaces[0], tick, origins[0]),
            input(self.surfaces[1], tick, origins[1]),
        ];
        if reversed {
            inputs.reverse();
        }
        self.harness.send(&inputs)
    }
}

fn junction(
    builder: &mut HarnessBuilder,
    physical_id: u64,
    position: i32,
    region: i16,
    threshold: i32,
) -> JunctionId {
    builder.add_junction(Junction {
        physical_id,
        position,
        region,
        threshold,
        resistance: u32::MAX,
    })
}

fn link(builder: &mut HarnessBuilder, from: JunctionId, to: JunctionId, delay: i64) {
    builder.add_link(Link {
        from,
        to,
        delay,
        phase: 0,
        coupling: 1,
        resistance: u32::MAX,
        mode: TransmissionMode::Drive,
    });
}

fn input(target: JunctionId, tick: i64, origin_physical: u64) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical,
        target,
        impulse: 1,
    }
}

fn motor_outputs(run: &Run) -> usize {
    run.outputs
        .iter()
        .filter(|output| output.from_physical == 80_010)
        .count()
}

#[derive(Clone, Debug, Serialize)]
struct TransferEvidence {
    reference_motor_outputs: usize,
    candidate_motor_outputs: usize,
    selected_origin: bool,
    input_order_behavior_equal: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

#[derive(Clone, Debug, Serialize)]
struct SameOriginControlEvidence {
    motor_outputs: usize,
    origin_factorization_activated: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn fixture_evidence() -> (TransferEvidence, SameOriginControlEvidence) {
    let mut reference = OriginFixture::new(Protocol::RecursiveLearnerBoundaryNovelty);
    let reference_setup = reference.prepare();
    let reference_run = reference.mixed([80_001, 80_002], false);

    let protocol = Protocol::RecursiveLearnerCausalOriginFactorization;
    let mut candidate = OriginFixture::new(protocol);
    let setup = candidate.prepare();
    let checkpoint = candidate.harness.save().expect("candidate fixture saves");
    let mut replay = OriginFixture::restore(protocol, checkpoint.clone());
    let mut reordered = OriginFixture::restore(protocol, checkpoint.clone());
    let mut same_origin = OriginFixture::restore(protocol, checkpoint.clone());
    let mut same_origin_replay = OriginFixture::restore(protocol, checkpoint);
    let candidate_run = candidate.mixed([80_001, 80_002], false);
    let replayed = replay.mixed([80_001, 80_002], false);
    let reversed = reordered.mixed([80_001, 80_002], true);
    let same_origin_run = same_origin.mixed([80_099, 80_099], false);
    let same_origin_replayed = same_origin_replay.mixed([80_099, 80_099], false);

    let selected_origin = candidate_run.physical_trace.iter().any(|transition| {
        matches!(
            transition.event,
            PhysicalEvent::CausalOriginCandidateResolved {
                origin_count: 2,
                selected_origin: Some(_),
                ..
            }
        )
    });
    let exact_replay = candidate_run == replayed
        && candidate.harness.save().unwrap().canonical_bytes().unwrap()
            == replay.harness.save().unwrap().canonical_bytes().unwrap();
    let naturally_quiescent = reference_setup
        .iter()
        .chain(&setup)
        .chain([&reference_run, &candidate_run, &replayed, &reversed])
        .all(|run| run.naturally_quiescent);
    let reference_motor_outputs = motor_outputs(&reference_run);
    let candidate_motor_outputs = motor_outputs(&candidate_run);
    let input_order_behavior_equal = candidate_run.outputs == reversed.outputs;
    let transfer_survived = reference_motor_outputs == 0
        && candidate_motor_outputs == 1
        && selected_origin
        && input_order_behavior_equal
        && exact_replay
        && naturally_quiescent;

    let same_origin_outputs = motor_outputs(&same_origin_run);
    let origin_factorization_activated = same_origin_run.physical_trace.iter().any(|transition| {
        matches!(
            transition.event,
            PhysicalEvent::CausalOriginCandidateResolved { .. }
        )
    });
    let same_origin_exact_replay = same_origin_run == same_origin_replayed
        && same_origin
            .harness
            .save()
            .unwrap()
            .canonical_bytes()
            .unwrap()
            == same_origin_replay
                .harness
                .save()
                .unwrap()
                .canonical_bytes()
                .unwrap();
    let same_origin_quiescent =
        same_origin_run.naturally_quiescent && same_origin_replayed.naturally_quiescent;
    let control_survived = same_origin_outputs == 0
        && !origin_factorization_activated
        && same_origin_exact_replay
        && same_origin_quiescent;

    (
        TransferEvidence {
            reference_motor_outputs,
            candidate_motor_outputs,
            selected_origin,
            input_order_behavior_equal,
            exact_replay,
            naturally_quiescent,
            survived: transfer_survived,
        },
        SameOriginControlEvidence {
            motor_outputs: same_origin_outputs,
            origin_factorization_activated,
            exact_replay: same_origin_exact_replay,
            naturally_quiescent: same_origin_quiescent,
            survived: control_survived,
        },
    )
}

#[derive(Clone, Debug, Serialize)]
struct HandEvidence {
    baseline_changed_steps: usize,
    candidate_changed_steps: usize,
    baseline_final_position: i16,
    candidate_final_position: i16,
    first_exhaustion_step: Option<usize>,
    first_exhaustion_motor_candidates: Option<usize>,
    propagation_budget_exhaustions: u64,
    primary_closed: bool,
    perturbation_recovered: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    movement_improved: bool,
    survived: bool,
}

fn hand_evidence(
    baseline: &ReflectedHandProtocolEvidence,
    candidate: &ReflectedHandProtocolEvidence,
) -> HandEvidence {
    let first_exhaustion = candidate
        .trajectory
        .iter()
        .find(|step| step.propagation_budget_exhaustions > 0);
    let propagation_budget_exhaustions = candidate
        .trajectory
        .iter()
        .map(|step| step.propagation_budget_exhaustions)
        .sum();
    let movement_improved = candidate.changed_steps > baseline.changed_steps;
    let survived = movement_improved
        && candidate.naturally_quiescent
        && candidate.primary_closed
        && candidate.perturbation_recovered;
    HandEvidence {
        baseline_changed_steps: baseline.changed_steps,
        candidate_changed_steps: candidate.changed_steps,
        baseline_final_position: baseline.final_position,
        candidate_final_position: candidate.final_position,
        first_exhaustion_step: first_exhaustion.map(|step| step.index),
        first_exhaustion_motor_candidates: first_exhaustion.map(|step| {
            step.output_candidates
                .iter()
                .filter(|candidate| candidate.is_motor)
                .count()
        }),
        propagation_budget_exhaustions,
        primary_closed: candidate.primary_closed,
        perturbation_recovered: candidate.perturbation_recovered,
        exact_replay: candidate.exact_replay,
        naturally_quiescent: candidate.naturally_quiescent,
        movement_improved,
        survived,
    }
}

#[derive(Clone, Debug)]
struct Evidence {
    transfer: TransferEvidence,
    same_origin: SameOriginControlEvidence,
    hand: HandEvidence,
}

fn measure() -> Evidence {
    let (transfer, same_origin) = fixture_evidence();
    let baseline = run_reflected_hand_with_protocol(Protocol::RecursiveLearnerBoundaryNovelty);
    let candidate = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerCausalOriginFactorization,
        512,
        2_048,
        MAX_MOMENTS_PER_SEND,
    );
    Evidence {
        transfer,
        same_origin,
        hand: hand_evidence(&baseline, &candidate),
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
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "hand-causal-origin-ownership-factorization/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations,
        falsifier: (!survived).then(|| falsifier.to_owned()),
        exact_replay,
        naturally_quiescent,
    }
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    match arm {
        Arm::UnrelatedOriginTransfer => result(
            arm,
            evidence.transfer.survived,
            serde_json::to_value(&evidence.transfer).unwrap(),
            "origin factorization did not produce exactly one private effect in the unrelated fixture",
            evidence.transfer.exact_replay,
            evidence.transfer.naturally_quiescent,
        ),
        Arm::SameOriginMixedOwnerControl => result(
            arm,
            evidence.same_origin.survived,
            serde_json::to_value(&evidence.same_origin).unwrap(),
            "a shared carried-origin label laundered evidence across two learner paths",
            evidence.same_origin.exact_replay,
            evidence.same_origin.naturally_quiescent,
        ),
        Arm::BoundedReflectedHand => result(
            arm,
            evidence.hand.survived,
            serde_json::to_value(&evidence.hand).unwrap(),
            "the hand gained movement but exhausted the propagation budget and failed natural quiescence",
            evidence.hand.exact_replay,
            evidence.hand.naturally_quiescent,
        ),
        Arm::CompleteComposition => {
            let survived = evidence.transfer.survived
                && evidence.same_origin.survived
                && evidence.hand.survived;
            result(
                arm,
                survived,
                serde_json::json!({
                    "unrelated_transfer": evidence.transfer.survived,
                    "same_origin_control": evidence.same_origin.survived,
                    "safe_hand_improvement": evidence.hand.survived,
                    "movement_improved": evidence.hand.movement_improved,
                    "naturally_quiescent": evidence.hand.naturally_quiescent,
                    "first_exhaustion_step": evidence.hand.first_exhaustion_step,
                    "next_question": "what feedback transition re-enters the selected origin and creates the self-sustaining wave",
                }),
                "causal-origin factorization did not earn transfer, boundary safety, and a naturally quiet hand together",
                evidence.transfer.exact_replay
                    && evidence.same_origin.exact_replay
                    && evidence.hand.exact_replay,
                evidence.transfer.naturally_quiescent
                    && evidence.same_origin.naturally_quiescent
                    && evidence.hand.naturally_quiescent,
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
    fn causal_origin_candidate_earns_unrelated_transfer() {
        assert_eq!(run(Arm::UnrelatedOriginTransfer).outcome, "survived");
    }

    #[test]
    fn same_origin_laundering_failure_is_preserved() {
        let result = run(Arm::SameOriginMixedOwnerControl);
        assert_eq!(result.outcome, "falsified");
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
    }

    #[test]
    fn bounded_hand_preserves_the_quiescence_failure() {
        let result = run(Arm::BoundedReflectedHand);
        assert_eq!(result.outcome, "falsified");
        assert!(result.exact_replay);
        assert!(!result.naturally_quiescent);
    }

    #[test]
    fn complete_candidate_is_rejected_after_quiescence_failure() {
        assert_eq!(run(Arm::CompleteComposition).outcome, "falsified");
    }
}
