#![forbid(unsafe_code)]

use developmental_hand_construction_admission::{
    ReflectedHandProtocolEvidence, run_reflected_hand_with_protocol,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::OnceLock;
use truelearner_core::{
    Checkpoint, Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent,
    Protocol, ReturnOriginDecision, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const EXPECTED_PARENT_SHA256: &str =
    "b8e0518d323c576923bdb2c677f0db1dff2c3452fb0a51beae9451f5c3fae5c6";
const FROZEN_PARENT: &str = include_str!(
    "../../../campaigns/hand-consequence-born-closure-eligibility-v1/convergence.toml"
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    InheritedTemporalReference,
    ExactBoundaryRepetition,
    SiblingBreadthAndLocality,
    AdjacentExpansionTransfer,
    CompleteBoundaryComposition,
    ConditionalReflectedHand,
    HandTransitionLocalization,
}

impl Arm {
    pub const ALL: [Self; 7] = [
        Self::InheritedTemporalReference,
        Self::ExactBoundaryRepetition,
        Self::SiblingBreadthAndLocality,
        Self::AdjacentExpansionTransfer,
        Self::CompleteBoundaryComposition,
        Self::ConditionalReflectedHand,
        Self::HandTransitionLocalization,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::InheritedTemporalReference => "inherited-temporal-reference",
            Self::ExactBoundaryRepetition => "exact-boundary-repetition",
            Self::SiblingBreadthAndLocality => "sibling-breadth-and-locality",
            Self::AdjacentExpansionTransfer => "adjacent-expansion-transfer",
            Self::CompleteBoundaryComposition => "complete-boundary-composition",
            Self::ConditionalReflectedHand => "conditional-reflected-hand",
            Self::HandTransitionLocalization => "hand-transition-localization",
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
    outcome: &'static str,
    observations: serde_json::Value,
    falsifier: Option<&str>,
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "hand-physical-boundary-member-novelty/v1",
        arm: arm.id(),
        outcome,
        observations,
        falsifier: falsifier.map(str::to_owned),
        exact_replay,
        naturally_quiescent,
    }
}

struct BoundaryFixture {
    harness: Harness,
    action: JunctionId,
    surfaces: [JunctionId; 3],
    motor: JunctionId,
}

impl BoundaryFixture {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(128, 512, OUTWARD_REGION);
        builder.set_protocol(Protocol::RecursiveLearnerBoundaryNovelty);
        builder.set_physical_tracing(true);
        let action = junction(&mut builder, 70_000, 0, 0, 1);
        let surfaces = [
            junction(&mut builder, 70_001, 2, 0, 1),
            junction(&mut builder, 70_002, 2, 0, 1),
            junction(&mut builder, 70_003, 3, 0, 1),
        ];
        let motor = junction(&mut builder, 70_010, 1, 0, 2);
        let sink = junction(&mut builder, 70_011, 1, OUTWARD_REGION, 1);
        let outcome = junction(&mut builder, 70_012, 50, 0, 1);
        let anchor = junction(&mut builder, 70_013, 100, 0, 99);
        for target in [action, surfaces[0], surfaces[1], surfaces[2], outcome] {
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

    fn restore(checkpoint: Checkpoint) -> Self {
        let template = Self::new();
        Self {
            harness: Harness::restore(checkpoint).expect("boundary checkpoint restores"),
            ..template
        }
    }

    fn round(&mut self, index: usize) -> [Run; 2] {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let action = self.harness.send(&[
            input(self.action, tick, 70_000),
            input(self.motor, tick.saturating_add(2), 70_010),
        ]);
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let consequence =
            self.harness
                .send(&[input(self.surfaces[index], tick, 70_001 + index as u64)]);
        [action, consequence]
    }
}

fn run_equal(left: &Run, right: &Run) -> bool {
    left.outputs == right.outputs
        && left.work == right.work
        && left.execution_cost == right.execution_cost
        && left.physical_trace == right.physical_trace
        && left.naturally_quiescent == right.naturally_quiescent
}

fn quiet(runs: &[Run]) -> bool {
    runs.iter().all(|run| run.naturally_quiescent)
}

#[derive(Clone, Debug, Serialize)]
struct RepetitionEvidence {
    repetitions: usize,
    learners_before: usize,
    learners_after: usize,
    rejected_observations: u64,
    regenerated_links_rejected_as_novelty: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn repetition_evidence() -> RepetitionEvidence {
    const REPETITIONS: usize = 12;
    let mut fixture = BoundaryFixture::new();
    let mut setup = Vec::new();
    setup.extend(fixture.round(0));
    setup.extend(fixture.round(0));
    let learners_before = fixture.harness.read().learners.len();
    let checkpoint = fixture.harness.save().expect("repetition checkpoint saves");
    let mut replay = BoundaryFixture::restore(checkpoint);
    let mut runs = Vec::new();
    let mut replayed = Vec::new();
    for _ in 0..REPETITIONS {
        runs.extend(fixture.round(0));
        replayed.extend(replay.round(0));
    }
    let rejected_observations = runs
        .iter()
        .flat_map(|run| &run.physical_trace)
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::BoundaryNoveltyEvaluated {
                    parent: Some(_),
                    novel_members: 0,
                    eligible: false,
                    ..
                }
            )
        })
        .count() as u64;
    let regenerated_links_rejected_as_novelty = runs.iter().any(|run| {
        run.work.local_structural_proposals > 0
            && run.physical_trace.iter().any(|transition| {
                matches!(
                    transition.event,
                    PhysicalEvent::BoundaryNoveltyEvaluated {
                        novel_members: 0,
                        eligible: false,
                        ..
                    }
                )
            })
    });
    let exact_replay = runs
        .iter()
        .zip(&replayed)
        .all(|(left, right)| run_equal(left, right))
        && fixture
            .harness
            .save()
            .expect("fixture saves")
            .canonical_bytes()
            .expect("fixture encodes")
            == replay
                .harness
                .save()
                .expect("replay saves")
                .canonical_bytes()
                .expect("replay encodes");
    let learners_after = fixture.harness.read().learners.len();
    let naturally_quiescent = quiet(&setup) && quiet(&runs) && quiet(&replayed);
    let survived = learners_before == 1
        && learners_after == 1
        && rejected_observations == REPETITIONS as u64
        && regenerated_links_rejected_as_novelty
        && exact_replay
        && naturally_quiescent;
    RepetitionEvidence {
        repetitions: REPETITIONS,
        learners_before,
        learners_after,
        rejected_observations,
        regenerated_links_rejected_as_novelty,
        exact_replay,
        naturally_quiescent,
        survived,
    }
}

#[derive(Clone, Debug, Serialize)]
struct SiblingEvidence {
    forward_surfaces: usize,
    reversed_surfaces: usize,
    order_independent: bool,
    distance_three_first_closure: u64,
    distance_three_second_closure: u64,
    distance_three_rejected_nonlocal: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn learned_surfaces(fixture: &BoundaryFixture) -> BTreeSet<JunctionId> {
    fixture
        .harness
        .read()
        .learners
        .iter()
        .map(|learner| learner.surface)
        .collect()
}

fn sibling_order(order: [usize; 2]) -> (BTreeSet<JunctionId>, Vec<Run>, bool) {
    let initial = BoundaryFixture::new();
    let checkpoint = initial.harness.save().expect("sibling checkpoint saves");
    let mut fixture = BoundaryFixture::restore(checkpoint.clone());
    let mut replay = BoundaryFixture::restore(checkpoint);
    let mut runs = Vec::new();
    let mut replayed = Vec::new();
    for index in order {
        for _ in 0..2 {
            runs.extend(fixture.round(index));
            replayed.extend(replay.round(index));
        }
    }
    let exact_replay = runs
        .iter()
        .zip(&replayed)
        .all(|(left, right)| run_equal(left, right));
    (learned_surfaces(&fixture), runs, exact_replay)
}

fn sibling_evidence() -> SiblingEvidence {
    let (forward, forward_runs, forward_replay) = sibling_order([0, 1]);
    let (reversed, reversed_runs, reversed_replay) = sibling_order([1, 0]);
    let order_independent = forward == reversed;

    let mut nonlocal = BoundaryFixture::new();
    let mut nonlocal_runs = Vec::new();
    nonlocal_runs.extend(nonlocal.round(0));
    nonlocal_runs.extend(nonlocal.round(0));
    let first = nonlocal.round(2);
    let second = nonlocal.round(2);
    let distance_three_first_closure = first
        .iter()
        .map(|run| run.work.causal_closure_observations)
        .sum();
    let distance_three_second_closure = second
        .iter()
        .map(|run| run.work.causal_closure_observations)
        .sum();
    let distance_three_rejected_nonlocal = second.iter().any(|run| {
        run.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::ReturnOriginEvaluated {
                    distance: Some(3),
                    decision: ReturnOriginDecision::RejectedNonLocal,
                    ..
                }
            )
        })
    });
    nonlocal_runs.extend(first);
    nonlocal_runs.extend(second);
    let naturally_quiescent =
        quiet(&forward_runs) && quiet(&reversed_runs) && quiet(&nonlocal_runs);
    let exact_replay = forward_replay && reversed_replay;
    let survived = forward.len() == 2
        && reversed.len() == 2
        && order_independent
        && distance_three_first_closure == 1
        && distance_three_second_closure == 0
        && distance_three_rejected_nonlocal
        && exact_replay
        && naturally_quiescent;
    SiblingEvidence {
        forward_surfaces: forward.len(),
        reversed_surfaces: reversed.len(),
        order_independent,
        distance_three_first_closure,
        distance_three_second_closure,
        distance_three_rejected_nonlocal,
        exact_replay,
        naturally_quiescent,
        survived,
    }
}

struct ExpansionFixture {
    harness: Harness,
    root_action: JunctionId,
    surface: JunctionId,
    root_motor: JunctionId,
    controlled: JunctionId,
    controlled_outcome: JunctionId,
}

impl ExpansionFixture {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(96, 256, OUTWARD_REGION);
        builder.set_protocol(Protocol::RecursiveLearnerBoundaryNovelty);
        builder.set_physical_tracing(true);
        let root_action = junction(&mut builder, 71_000, 0, 0, 1);
        let surface = junction(&mut builder, 71_001, 2, 0, 1);
        let root_motor = junction(&mut builder, 71_010, 1, 0, 3);
        let root_sink = junction(&mut builder, 71_011, 1, OUTWARD_REGION, 1);
        let root_outcome = junction(&mut builder, 71_012, 50, 0, 1);
        let anchor = junction(&mut builder, 71_013, 100, 0, 99);
        let controlled = junction(&mut builder, 71_020, 30, 0, 2);
        let controlled_sink = junction(&mut builder, 71_021, 30, OUTWARD_REGION, 1);
        let controlled_path = junction(&mut builder, 71_022, 2, 0, 1);
        let controlled_outcome = junction(&mut builder, 71_023, 60, 0, 1);
        for target in [root_action, surface, root_outcome, controlled_outcome] {
            link(&mut builder, anchor, target, 0);
        }
        link(&mut builder, surface, root_outcome, 3);
        link(&mut builder, root_motor, root_sink, 0);
        link(&mut builder, surface, controlled_path, 0);
        link(&mut builder, controlled_path, controlled, 2);
        link(&mut builder, controlled, controlled_sink, 0);
        builder.set_outcome_source_for_output(root_motor, root_outcome);
        builder.set_outcome_source_for_output(controlled, controlled_outcome);
        Self {
            harness: builder.build(),
            root_action,
            surface,
            root_motor,
            controlled,
            controlled_outcome,
        }
    }

    fn restore(checkpoint: Checkpoint) -> Self {
        let template = Self::new();
        Self {
            harness: Harness::restore(checkpoint).expect("expansion checkpoint restores"),
            ..template
        }
    }

    fn root_round(&mut self) -> Vec<Run> {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let action = self.harness.send(&[
            input(self.root_action, tick, 71_000),
            Input {
                impulse: 2,
                ..input(self.root_motor, tick.saturating_add(2), 71_010)
            },
        ]);
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let consequence = self.harness.send(&[input(self.surface, tick, 71_001)]);
        vec![action, consequence]
    }

    fn expansion_round(&mut self) -> Vec<Run> {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let action = self.harness.send(&[
            input(self.surface, tick, 71_001),
            input(self.controlled, tick.saturating_add(2), 71_001),
        ]);
        let tick = self.harness.read().clock.tick.saturating_add(1);
        let consequence = self
            .harness
            .send(&[input(self.controlled_outcome, tick, 71_001)]);
        vec![action, consequence]
    }
}

#[derive(Clone, Debug, Serialize)]
struct ExpansionEvidence {
    learners: usize,
    child_parented: bool,
    child_output_is_new: bool,
    novel_members_observed: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    survived: bool,
}

fn expansion_evidence() -> ExpansionEvidence {
    let mut fixture = ExpansionFixture::new();
    let mut setup = fixture.root_round();
    setup.extend(fixture.root_round());
    let root = fixture.harness.read().learners[0].id;
    let checkpoint = fixture.harness.save().expect("expansion checkpoint saves");
    let mut replay = ExpansionFixture::restore(checkpoint);
    let mut runs = Vec::new();
    let mut replayed = Vec::new();
    for _ in 0..2 {
        runs.extend(fixture.expansion_round());
        replayed.extend(replay.expansion_round());
    }
    let observed = fixture.harness.read();
    let child = observed.learners.get(1);
    let child_parented = child.is_some_and(|learner| learner.parent == Some(root));
    let child_output_is_new = child.is_some_and(|learner| learner.output == fixture.controlled);
    let novel_members_observed = runs.iter().any(|run| {
        run.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::BoundaryNoveltyEvaluated {
                    parent: Some(parent),
                    novel_members,
                    eligible: true,
                    ..
                } if parent == root && novel_members > 0
            )
        })
    });
    let exact_replay = runs
        .iter()
        .zip(&replayed)
        .all(|(left, right)| run_equal(left, right));
    let naturally_quiescent = quiet(&setup) && quiet(&runs) && quiet(&replayed);
    let learners = observed.learners.len();
    let survived = learners == 2
        && child_parented
        && child_output_is_new
        && novel_members_observed
        && exact_replay
        && naturally_quiescent;
    ExpansionEvidence {
        learners,
        child_parented,
        child_output_is_new,
        novel_members_observed,
        exact_replay,
        naturally_quiescent,
        survived,
    }
}

#[derive(Clone, Debug, Serialize)]
struct Evidence {
    parent_sha256: String,
    parent_intact: bool,
    repetition: RepetitionEvidence,
    siblings: SiblingEvidence,
    expansion: ExpansionEvidence,
    boundary_gate_survived: bool,
    hand_reference: Option<ReflectedHandProtocolEvidence>,
    hand: Option<ReflectedHandProtocolEvidence>,
}

#[derive(Clone, Debug, Serialize)]
struct HandLocalization {
    first_behavior_difference: Option<usize>,
    candidate_last_changed_step: Option<usize>,
    terminal_stall_step: Option<usize>,
    terminal_surface_write_without_owner_read: bool,
    first_post_stall_autonomous_step_without_motor_output: Option<usize>,
    reference_changed_steps: usize,
    candidate_changed_steps: usize,
    earliest_missing_transition: &'static str,
    interpretable: bool,
}

fn hand_localization(
    reference: &ReflectedHandProtocolEvidence,
    candidate: &ReflectedHandProtocolEvidence,
) -> HandLocalization {
    let first_behavior_difference = reference
        .trajectory
        .iter()
        .zip(&candidate.trajectory)
        .position(|(reference, candidate)| {
            reference.position_after != candidate.position_after
                || reference.direction != candidate.direction
                || reference.emitted_outputs != candidate.emitted_outputs
        });
    let candidate_last_changed_step = candidate
        .trajectory
        .iter()
        .rfind(|step| step.position_after != step.position_before)
        .map(|step| step.index);
    let terminal_stall_step = candidate_last_changed_step.map(|step| step.saturating_add(1));
    let terminal_surface_write_without_owner_read = terminal_stall_step
        .and_then(|index| candidate.trajectory.get(index))
        .is_some_and(|step| {
            step.delivered_surface_count > 0
                && step.owner_writes > 0
                && step.owner_reads == 0
                && step.direction == 0
        });
    let first_post_stall_autonomous_step_without_motor_output =
        terminal_stall_step.and_then(|terminal| {
            candidate.trajectory.iter().find_map(|step| {
                (step.index > terminal
                    && step.delivered_surface_count == 0
                    && step.direction == 0
                    && !step
                        .emitted_outputs
                        .iter()
                        .any(|physical| matches!(*physical, 20_000 | 20_001)))
                .then_some(step.index)
            })
        });
    let interpretable = reference.exact_replay
        && reference.naturally_quiescent
        && candidate.exact_replay
        && candidate.naturally_quiescent
        && first_behavior_difference.is_none()
        && terminal_surface_write_without_owner_read
        && first_post_stall_autonomous_step_without_motor_output.is_some();
    HandLocalization {
        first_behavior_difference,
        candidate_last_changed_step,
        terminal_stall_step,
        terminal_surface_write_without_owner_read,
        first_post_stall_autonomous_step_without_motor_output,
        reference_changed_steps: reference.changed_steps,
        candidate_changed_steps: candidate.changed_steps,
        earliest_missing_transition: "post-construction surface-to-owner autonomous reuse",
        interpretable,
    }
}

fn measure() -> Evidence {
    let parent_sha256 = format!("{:x}", Sha256::digest(FROZEN_PARENT.as_bytes()));
    let parent_intact = parent_sha256 == EXPECTED_PARENT_SHA256
        && FROZEN_PARENT.contains("eligible-return-composition")
        && FROZEN_PARENT.contains("physical-boundary-member-novelty");
    let repetition = repetition_evidence();
    let siblings = sibling_evidence();
    let expansion = expansion_evidence();
    let boundary_gate_survived =
        parent_intact && repetition.survived && siblings.survived && expansion.survived;
    let hand_reference = parent_intact
        .then(|| run_reflected_hand_with_protocol(Protocol::RecursiveLearnerEligibleReturnClosure));
    let hand = boundary_gate_survived
        .then(|| run_reflected_hand_with_protocol(Protocol::RecursiveLearnerBoundaryNovelty));
    Evidence {
        parent_sha256,
        parent_intact,
        repetition,
        siblings,
        expansion,
        boundary_gate_survived,
        hand_reference,
        hand,
    }
}

static EVIDENCE: OnceLock<Evidence> = OnceLock::new();

fn evidence() -> &'static Evidence {
    EVIDENCE.get_or_init(measure)
}

pub fn run(arm: Arm) -> ProbeResult {
    let evidence = evidence();
    match arm {
        Arm::InheritedTemporalReference => result(
            arm,
            if evidence.parent_intact {
                "survived"
            } else {
                "inconclusive"
            },
            serde_json::json!({
                "parent_sha256": evidence.parent_sha256,
                "parent_intact": evidence.parent_intact,
                "temporal_composition": "survived",
                "next_frontier": "physical-boundary-member-novelty",
            }),
            (!evidence.parent_intact).then_some("the immutable temporal parent changed"),
            true,
            true,
        ),
        Arm::ExactBoundaryRepetition => result(
            arm,
            if evidence.repetition.survived {
                "survived"
            } else {
                "falsified"
            },
            serde_json::to_value(&evidence.repetition).expect("repetition serializes"),
            (!evidence.repetition.survived).then_some(
                "an already owned physical boundary still earned closure or learner depth",
            ),
            evidence.repetition.exact_replay,
            evidence.repetition.naturally_quiescent,
        ),
        Arm::SiblingBreadthAndLocality => result(
            arm,
            if evidence.siblings.survived {
                "survived"
            } else {
                "falsified"
            },
            serde_json::to_value(&evidence.siblings).expect("siblings serialize"),
            (!evidence.siblings.survived).then_some(
                "two distinct local siblings did not construct in both orders or inherited distance-three locality changed",
            ),
            evidence.siblings.exact_replay,
            evidence.siblings.naturally_quiescent,
        ),
        Arm::AdjacentExpansionTransfer => result(
            arm,
            if evidence.expansion.survived {
                "survived"
            } else {
                "falsified"
            },
            serde_json::to_value(&evidence.expansion).expect("expansion serializes"),
            (!evidence.expansion.survived).then_some(
                "an owned surface could not construct an adjacent child after adding a new physical member",
            ),
            evidence.expansion.exact_replay,
            evidence.expansion.naturally_quiescent,
        ),
        Arm::CompleteBoundaryComposition => result(
            arm,
            if evidence.boundary_gate_survived {
                "survived"
            } else {
                "falsified"
            },
            serde_json::json!({
                "parent_intact": evidence.parent_intact,
                "exact_repetition": evidence.repetition.survived,
                "sibling_breadth_and_locality": evidence.siblings.survived,
                "adjacent_expansion": evidence.expansion.survived,
                "boundary_gate_survived": evidence.boundary_gate_survived,
            }),
            (!evidence.boundary_gate_survived)
                .then_some("the complete boundary-member novelty gate failed a prerequisite"),
            evidence.repetition.exact_replay
                && evidence.siblings.exact_replay
                && evidence.expansion.exact_replay,
            evidence.repetition.naturally_quiescent
                && evidence.siblings.naturally_quiescent
                && evidence.expansion.naturally_quiescent,
        ),
        Arm::ConditionalReflectedHand => {
            let Some(hand) = &evidence.hand else {
                return result(
                    arm,
                    "inconclusive",
                    serde_json::json!({"boundary_gate_survived": false, "hand": null}),
                    Some("the unchanged hand was not run because the boundary gate failed"),
                    false,
                    false,
                );
            };
            let survived = hand.primary_closed && hand.perturbation_recovered;
            result(
                arm,
                if survived { "survived" } else { "falsified" },
                serde_json::json!({
                    "boundary_gate_survived": true,
                    "hand": hand,
                }),
                (!survived).then_some(
                    "the unchanged reflected hand did not close both limits and recover after the boundary gate",
                ),
                hand.exact_replay,
                hand.naturally_quiescent,
            )
        }
        Arm::HandTransitionLocalization => {
            let Some((reference, candidate)) = evidence
                .hand_reference
                .as_ref()
                .zip(evidence.hand.as_ref())
            else {
                return result(
                    arm,
                    "inconclusive",
                    serde_json::json!({
                        "reference_available": evidence.hand_reference.is_some(),
                        "candidate_available": evidence.hand.is_some(),
                    }),
                    Some("the parent or boundary gate failed before hand localization"),
                    false,
                    false,
                );
            };
            let localization = hand_localization(reference, candidate);
            result(
                arm,
                if localization.interpretable {
                    "survived"
                } else {
                    "inconclusive"
                },
                serde_json::json!({
                    "localization": localization,
                    "reference": reference,
                    "candidate": candidate,
                }),
                (!localization.interpretable)
                    .then_some("the hand trace could not localize a replayable first reuse failure"),
                reference.exact_replay && candidate.exact_replay,
                reference.naturally_quiescent && candidate.naturally_quiescent,
            )
        }
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
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

fn input(target: JunctionId, arrival_tick: i64, origin_physical: u64) -> Input {
    Input {
        arrival_tick,
        phase: 0,
        origin_physical,
        target,
        impulse: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_repetition_does_not_turn_regenerated_links_into_novelty() {
        let evidence = repetition_evidence();
        assert!(evidence.survived, "{evidence:#?}");
    }

    #[test]
    fn local_siblings_and_nonlocal_control_are_both_preserved() {
        let evidence = sibling_evidence();
        assert!(evidence.survived, "{evidence:#?}");
    }

    #[test]
    fn adjacent_expansion_adds_a_real_child_member() {
        let evidence = expansion_evidence();
        assert!(evidence.survived, "{evidence:#?}");
    }

    #[test]
    fn hand_execution_is_strictly_conditional_on_the_complete_gate() {
        let evidence = evidence();
        assert!(evidence.boundary_gate_survived);
        assert!(evidence.hand.is_some());
    }

    #[test]
    fn every_arm_is_total_and_settles() {
        for arm in Arm::ALL {
            let result = run(arm);
            assert_eq!(result.arm, arm.id());
            assert!(matches!(
                result.outcome,
                "survived" | "falsified" | "inconclusive"
            ));
            assert!(result.exact_replay);
            assert!(result.naturally_quiescent);
        }
    }
}
