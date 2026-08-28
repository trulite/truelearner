#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    OutputCandidateEvidence, ReflectedHandProtocolEvidence, run_reflected_hand_with_protocol,
};
use serde::Serialize;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{
    Checkpoint, Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent,
    Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    UnrelatedMixedOwnerTransfer,
    UnownedOriginDiscriminator,
    ConditionalReflectedHand,
    CompleteComposition,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::UnrelatedMixedOwnerTransfer,
        Self::UnownedOriginDiscriminator,
        Self::ConditionalReflectedHand,
        Self::CompleteComposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::UnrelatedMixedOwnerTransfer => "unrelated-mixed-owner-transfer",
            Self::UnownedOriginDiscriminator => "unowned-origin-discriminator",
            Self::ConditionalReflectedHand => "conditional-reflected-hand",
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

struct MixedOwnerFixture {
    harness: Harness,
    action: JunctionId,
    surfaces: [JunctionId; 2],
    motor: JunctionId,
}

impl MixedOwnerFixture {
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

    fn mixed(&mut self, reversed: bool) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let mut inputs = vec![
            input(self.surfaces[0], tick, 80_001),
            input(self.surfaces[1], tick, 80_002),
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

#[derive(Clone, Debug, Serialize)]
struct TransferEvidence {
    reference_motor_outputs: usize,
    candidate_motor_outputs: usize,
    selected_owner_group: bool,
    one_effect: bool,
    input_order_behavior_equal: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn motor_outputs(run: &Run) -> usize {
    run.outputs
        .iter()
        .filter(|output| output.from_physical == 80_010)
        .count()
}

fn transfer_evidence() -> TransferEvidence {
    let mut reference = MixedOwnerFixture::new(Protocol::RecursiveLearnerBoundaryNovelty);
    let reference_setup = reference.prepare();
    let reference_run = reference.mixed(false);
    let reference_motor_outputs = motor_outputs(&reference_run);

    let protocol = Protocol::RecursiveLearnerOwnerFactorization;
    let mut candidate = MixedOwnerFixture::new(protocol);
    let setup = candidate.prepare();
    let checkpoint = candidate.harness.save().expect("candidate fixture saves");
    let mut replay = MixedOwnerFixture::restore(protocol, checkpoint.clone());
    let mut reversed = MixedOwnerFixture::restore(protocol, checkpoint);
    let candidate_run = candidate.mixed(false);
    let replayed = replay.mixed(false);
    let reordered = reversed.mixed(true);
    let candidate_motor_outputs = motor_outputs(&candidate_run);
    let selected_owner_group = candidate_run.physical_trace.iter().any(|transition| {
        matches!(
            transition.event,
            PhysicalEvent::MixedOwnerCandidateResolved {
                owner_count: 2,
                selected_owner: Some(_),
                ..
            }
        )
    });
    let one_effect = candidate_motor_outputs == 1;
    let input_order_behavior_equal = candidate_run.outputs == reordered.outputs;
    let exact_replay = candidate_run == replayed
        && candidate.harness.save().unwrap().canonical_bytes().unwrap()
            == replay.harness.save().unwrap().canonical_bytes().unwrap();
    let naturally_quiescent = reference_setup
        .iter()
        .chain(&setup)
        .chain([&reference_run, &candidate_run, &replayed, &reordered])
        .all(|run| run.naturally_quiescent);
    let survived = reference_motor_outputs == 0
        && one_effect
        && selected_owner_group
        && input_order_behavior_equal
        && exact_replay
        && naturally_quiescent;
    TransferEvidence {
        reference_motor_outputs,
        candidate_motor_outputs,
        selected_owner_group,
        one_effect,
        input_order_behavior_equal,
        exact_replay,
        naturally_quiescent,
        survived,
    }
}

#[derive(Clone, Debug, Serialize)]
struct DiscriminatorEvidence {
    step: usize,
    motor_candidates: usize,
    candidates_with_two_origins: usize,
    candidates_with_one_known_owner: usize,
    ambiguous_candidates: usize,
    owner_only_prerequisite_present: bool,
    causal_origin_prerequisite_present: bool,
    common_boundary_prerequisite_present: bool,
    survived: bool,
}

fn stalled_motor_candidates(
    hand: &ReflectedHandProtocolEvidence,
) -> (usize, Vec<&OutputCandidateEvidence>) {
    let step = hand
        .trajectory
        .iter()
        .rfind(|step| step.position_before != step.position_after)
        .map(|step| step.index.saturating_add(2))
        .unwrap_or(0);
    let candidates = hand
        .trajectory
        .get(step)
        .map(|step| {
            step.output_candidates
                .iter()
                .filter(|candidate| candidate.is_motor && candidate.path_inputs > 0)
                .collect()
        })
        .unwrap_or_default();
    (step, candidates)
}

fn discriminator_evidence(hand: &ReflectedHandProtocolEvidence) -> DiscriminatorEvidence {
    let (step, candidates) = stalled_motor_candidates(hand);
    let candidates_with_two_origins = candidates
        .iter()
        .filter(|candidate| candidate.distinct_path_origins == 2)
        .count();
    let candidates_with_one_known_owner = candidates
        .iter()
        .filter(|candidate| candidate.distinct_path_owners == 1)
        .count();
    let ambiguous_candidates = candidates
        .iter()
        .filter(|candidate| candidate.ownership == truelearner_core::CandidateOwnership::Ambiguous)
        .count();
    let motor_candidates = candidates.len();
    let owner_only_prerequisite_present = motor_candidates > 0
        && candidates
            .iter()
            .all(|candidate| candidate.distinct_path_owners >= 2);
    let causal_origin_prerequisite_present = motor_candidates > 0
        && candidates_with_two_origins == motor_candidates
        && candidates_with_one_known_owner == motor_candidates;
    let common_boundary_prerequisite_present = owner_only_prerequisite_present;
    let survived = motor_candidates == 2
        && ambiguous_candidates == 2
        && !owner_only_prerequisite_present
        && causal_origin_prerequisite_present
        && !common_boundary_prerequisite_present;
    DiscriminatorEvidence {
        step,
        motor_candidates,
        candidates_with_two_origins,
        candidates_with_one_known_owner,
        ambiguous_candidates,
        owner_only_prerequisite_present,
        causal_origin_prerequisite_present,
        common_boundary_prerequisite_present,
        survived,
    }
}

#[derive(Clone, Debug)]
struct Evidence {
    transfer: TransferEvidence,
    baseline: ReflectedHandProtocolEvidence,
    hand: ReflectedHandProtocolEvidence,
    discriminator: DiscriminatorEvidence,
}

fn measure() -> Evidence {
    let transfer = transfer_evidence();
    let baseline = run_reflected_hand_with_protocol(Protocol::RecursiveLearnerBoundaryNovelty);
    let hand = run_reflected_hand_with_protocol(Protocol::RecursiveLearnerOwnerFactorization);
    let discriminator = discriminator_evidence(&hand);
    Evidence {
        transfer,
        baseline,
        hand,
        discriminator,
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
        schema: "hand-owner-local-candidate-factorization/v1",
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
        Arm::UnrelatedMixedOwnerTransfer => result(
            arm,
            evidence.transfer.survived,
            serde_json::to_value(&evidence.transfer).unwrap(),
            "owner-only factorization did not create exactly one private effect in the unrelated fixture",
            evidence.transfer.exact_replay,
            evidence.transfer.naturally_quiescent,
        ),
        Arm::UnownedOriginDiscriminator => result(
            arm,
            evidence.discriminator.survived,
            serde_json::to_value(&evidence.discriminator).unwrap(),
            "the stalled hand did not distinguish owner-only, causal-origin, and common-boundary prerequisites",
            evidence.hand.exact_replay,
            evidence.hand.naturally_quiescent,
        ),
        Arm::ConditionalReflectedHand => {
            let improved = evidence.hand.changed_steps > evidence.baseline.changed_steps;
            result(
                arm,
                improved,
                serde_json::json!({
                    "baseline_changed_steps": evidence.baseline.changed_steps,
                    "candidate_changed_steps": evidence.hand.changed_steps,
                    "baseline_final_position": evidence.baseline.final_position,
                    "candidate_final_position": evidence.hand.final_position,
                    "primary_closed": evidence.hand.primary_closed,
                    "perturbation_recovered": evidence.hand.perturbation_recovered,
                }),
                "the candidate passed its fixture but did not improve the unchanged hand",
                evidence.hand.exact_replay,
                evidence.hand.naturally_quiescent,
            )
        }
        Arm::CompleteComposition => {
            let hand_improved = evidence.hand.changed_steps > evidence.baseline.changed_steps;
            let survived =
                evidence.transfer.survived && evidence.discriminator.survived && hand_improved;
            result(
                arm,
                survived,
                serde_json::json!({
                    "fixture_survived": evidence.transfer.survived,
                    "hand_improved": hand_improved,
                    "owner_only_prerequisite_present": evidence.discriminator.owner_only_prerequisite_present,
                    "causal_origin_prerequisite_present": evidence.discriminator.causal_origin_prerequisite_present,
                    "common_boundary_prerequisite_present": evidence.discriminator.common_boundary_prerequisite_present,
                    "next_candidate": "causal-origin-first ownership-domain factorization",
                }),
                "the owner-only law did not earn both unrelated transfer and hand improvement",
                evidence.transfer.exact_replay && evidence.hand.exact_replay,
                evidence.transfer.naturally_quiescent && evidence.hand.naturally_quiescent,
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
    fn owner_only_candidate_earns_unrelated_transfer() {
        assert_eq!(run(Arm::UnrelatedMixedOwnerTransfer).outcome, "survived");
    }

    #[test]
    fn stalled_hand_exposes_unowned_origin_not_second_owner() {
        assert_eq!(run(Arm::UnownedOriginDiscriminator).outcome, "survived");
    }

    #[test]
    fn complete_candidate_is_total_and_preserves_negative_result() {
        let result = run(Arm::CompleteComposition);
        assert!(matches!(result.outcome, "survived" | "falsified"));
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
    }
}
