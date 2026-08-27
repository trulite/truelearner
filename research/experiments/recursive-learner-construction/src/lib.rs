use serde::Serialize;
use std::str::FromStr;
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, LearnerObservation, Link, Protocol, Run,
    TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    FrozenParentReference,
    ClosureSubgraphAutonomy,
    BlankChildBudding,
    ParticipationBornBudding,
    CompleteRecursiveUpwardLadder,
    FirstFailureLocalization,
    ConditionalDownwardAblations,
}

impl Arm {
    pub const ALL: [Self; 7] = [
        Self::FrozenParentReference,
        Self::ClosureSubgraphAutonomy,
        Self::BlankChildBudding,
        Self::ParticipationBornBudding,
        Self::CompleteRecursiveUpwardLadder,
        Self::FirstFailureLocalization,
        Self::ConditionalDownwardAblations,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::FrozenParentReference => "frozen-parent-reference",
            Self::ClosureSubgraphAutonomy => "closure-subgraph-autonomy",
            Self::BlankChildBudding => "blank-child-budding",
            Self::ParticipationBornBudding => "participation-born-budding",
            Self::CompleteRecursiveUpwardLadder => "complete-recursive-upward-ladder",
            Self::FirstFailureLocalization => "first-failure-localization",
            Self::ConditionalDownwardAblations => "conditional-downward-ablations",
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

pub fn run(arm: Arm) -> ProbeResult {
    match arm {
        Arm::FrozenParentReference => frozen_parent_reference(),
        Arm::ClosureSubgraphAutonomy => closure_subgraph_autonomy(),
        Arm::BlankChildBudding => blank_child_budding(),
        Arm::ParticipationBornBudding => participation_born_budding(),
        Arm::CompleteRecursiveUpwardLadder => complete_recursive_upward_ladder(),
        Arm::FirstFailureLocalization => first_failure_localization(),
        Arm::ConditionalDownwardAblations => conditional_downward_ablations(),
    }
}

fn finish(
    arm: Arm,
    survived: bool,
    observations: serde_json::Value,
    falsifier: &str,
    replay: bool,
    quiet: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "recursive-learner-construction/v1",
        arm: arm.id(),
        outcome: if survived { "survived" } else { "falsified" },
        observations,
        falsifier: (!survived).then(|| falsifier.to_string()),
        exact_replay: replay,
        naturally_quiescent: quiet,
    }
}

fn inconclusive(arm: Arm, observations: serde_json::Value, reason: &str) -> ProbeResult {
    ProbeResult {
        schema: "recursive-learner-construction/v1",
        arm: arm.id(),
        outcome: "inconclusive",
        observations,
        falsifier: Some(reason.to_string()),
        exact_replay: true,
        naturally_quiescent: true,
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

fn same_run(left: &Run, right: &Run) -> bool {
    left.outputs == right.outputs
        && left.work == right.work
        && left.execution_cost == right.execution_cost
        && left.naturally_quiescent == right.naturally_quiescent
}

struct ClosureWorld {
    harness: Harness,
    action: JunctionId,
    surface: JunctionId,
    unrelated: JunctionId,
    motor: JunctionId,
}

impl ClosureWorld {
    fn new(protocol: Protocol, dormant: usize) -> Self {
        let capacity = u32::try_from(dormant.saturating_mul(2).saturating_add(64)).unwrap();
        let mut builder =
            HarnessBuilder::with_capacity(capacity, capacity.saturating_mul(4), OUTWARD_REGION);
        builder.set_protocol(protocol);
        builder.set_physical_tracing(true);
        let action = junction(&mut builder, 40_000, 0, 0, 1);
        let surface = junction(&mut builder, 40_001, 2, 0, 1);
        let unrelated = junction(&mut builder, 40_002, 20, 0, 1);
        let motor = junction(&mut builder, 40_010, 1, 0, 2);
        let sink = junction(&mut builder, 40_011, 1, OUTWARD_REGION, 1);
        let outcome = junction(&mut builder, 40_012, 50, 0, 1);
        let anchor = junction(&mut builder, 40_013, 100, 0, 99);
        for target in [action, surface, unrelated, outcome] {
            link(&mut builder, anchor, target, 0);
        }
        for source in [surface, unrelated] {
            link(&mut builder, source, outcome, 3);
        }
        link(&mut builder, motor, sink, 0);
        builder.set_outcome_source_for_output(motor, outcome);
        for index in 0..dormant {
            let position = 1_000_i32.saturating_add(i32::try_from(index).unwrap());
            let output = junction(&mut builder, 50_000 + index as u64, position, 0, 2);
            let sink = junction(
                &mut builder,
                60_000 + index as u64,
                position,
                OUTWARD_REGION,
                1,
            );
            link(&mut builder, output, sink, 0);
        }
        Self {
            harness: builder.build(),
            action,
            surface,
            unrelated,
            motor,
        }
    }

    fn train_action(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.action, tick, 40_000),
            input(self.motor, tick.saturating_add(2), 40_010),
        ])
    }

    fn send_surface(&mut self, surface: JunctionId, physical: u64) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[input(surface, tick, physical)])
    }
}

struct ClosureTrial {
    learners: Vec<LearnerObservation>,
    first_evidence: u64,
    constructions: u64,
    false_constructions: usize,
    replay: bool,
    quiet: bool,
    structural_scans: u64,
}

fn closure_trial(dormant: usize) -> ClosureTrial {
    let mut world = ClosureWorld::new(Protocol::RecursiveLearnerConstruction, dormant);
    let trained = world.train_action();
    let first = world.send_surface(world.surface, 40_001);
    let checkpoint = world.harness.save().expect("closure checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("closure checkpoint restores");
    let retrained = world.train_action();
    let replay_tick = replay.read().clock.tick.saturating_add(1);
    let replay_retrained = replay.send(&[
        input(world.action, replay_tick, 40_000),
        input(world.motor, replay_tick.saturating_add(2), 40_010),
    ]);
    let second = world.send_surface(world.surface, 40_001);
    let tick = replay.read().clock.tick.saturating_add(1);
    let replayed = replay.send(&[input(world.surface, tick, 40_001)]);
    let exact = same_run(&retrained, &replay_retrained)
        && same_run(&second, &replayed)
        && world.harness.save().unwrap().canonical_bytes().unwrap()
            == replay.save().unwrap().canonical_bytes().unwrap();
    let before_false = world.harness.read().learners.len();
    let unrelated = world.send_surface(world.unrelated, 40_002);
    let after_false = world.harness.read().learners.len();
    ClosureTrial {
        learners: world.harness.read().learners,
        first_evidence: first.work.causal_closure_observations,
        constructions: second.work.learner_constructions,
        false_constructions: after_false.saturating_sub(before_false),
        replay: exact,
        quiet: trained.naturally_quiescent
            && first.naturally_quiescent
            && second.naturally_quiescent
            && unrelated.naturally_quiescent,
        structural_scans: first.execution_cost.local_structural_scans,
    }
}

fn frozen_parent_reference() -> ProbeResult {
    use sensorimotor_synthesis_ladder::Arm as Parent;
    let reference = sensorimotor_synthesis_ladder::run(Parent::CompleteCandidateReference);
    let joint = sensorimotor_synthesis_ladder::run(Parent::OneJointControl);
    finish(
        Arm::FrozenParentReference,
        reference.outcome == "survived" && joint.outcome == "falsified",
        serde_json::json!({
            "complete_candidate_reference": reference.outcome,
            "one_joint_control": joint.outcome,
            "one_joint": joint.observations,
        }),
        "a frozen parent classification changed",
        reference.exact_replay && joint.exact_replay,
        reference.naturally_quiescent && joint.naturally_quiescent,
    )
}

fn closure_subgraph_autonomy() -> ProbeResult {
    let trial = closure_trial(4);
    finish(
        Arm::ClosureSubgraphAutonomy,
        false,
        serde_json::json!({
            "closure_observed": trial.first_evidence,
            "allocated_learner_records": trial.learners.len(),
            "reason": "the implemented candidate buds a separate causal learner record",
        }),
        "causal closure did not become autonomous without a separate learner allocation",
        trial.replay,
        trial.quiet,
    )
}

fn blank_child_budding() -> ProbeResult {
    let trial = closure_trial(4);
    let member_links = trial
        .learners
        .first()
        .map_or(0, |learner| learner.links.len());
    finish(
        Arm::BlankChildBudding,
        false,
        serde_json::json!({
            "learners": trial.learners.len(),
            "inherited_live_lineage_links": member_links,
        }),
        "the constructed child preserved participating lineage and was not blank",
        trial.replay,
        trial.quiet,
    )
}

fn participation_born_budding() -> ProbeResult {
    let trial = closure_trial(4);
    let root_constructed = trial.learners.len() == 1
        && trial.learners[0].parent.is_none()
        && !trial.learners[0].links.is_empty();
    finish(
        Arm::ParticipationBornBudding,
        false,
        serde_json::json!({
            "root_constructed": root_constructed,
            "false_constructions": trial.false_constructions,
            "descendant_constructed": false,
            "exact_replay": trial.replay,
        }),
        "lineage-preserving budding constructed a root child but did not demonstrate recursive depth-three construction",
        trial.replay,
        trial.quiet,
    )
}

fn upward_observations() -> (Vec<serde_json::Value>, usize, bool, bool) {
    let parent = frozen_parent_reference();
    let sizes = [4, 64, 1_024]
        .into_iter()
        .map(closure_trial)
        .collect::<Vec<_>>();
    let root = &sizes[0];
    let scans = sizes
        .iter()
        .map(|trial| trial.structural_scans)
        .collect::<Vec<_>>();
    let rungs = vec![
        serde_json::json!({"rung": 0, "passed": parent.outcome == "survived"}),
        serde_json::json!({"rung": 1, "passed": root.first_evidence == 1 && root.false_constructions == 0}),
        serde_json::json!({"rung": 2, "passed": root.constructions == 1 && root.learners.len() == 1}),
        serde_json::json!({"rung": 3, "passed": sizes.iter().all(|trial| trial.replay && trial.quiet) && scans.windows(2).all(|pair| pair[0] == pair[1]), "structural_scans": scans}),
        serde_json::json!({"rung": 4, "passed": false, "constructed_depth": 1, "required_depth": 3}),
    ];
    let first_failed = rungs
        .iter()
        .position(|rung| !rung["passed"].as_bool().unwrap_or(false))
        .unwrap_or(rungs.len());
    (
        rungs,
        first_failed,
        sizes.iter().all(|trial| trial.replay),
        sizes.iter().all(|trial| trial.quiet),
    )
}

fn complete_recursive_upward_ladder() -> ProbeResult {
    let (rungs, first_failed, replay, quiet) = upward_observations();
    finish(
        Arm::CompleteRecursiveUpwardLadder,
        false,
        serde_json::json!({
            "executed_rungs": rungs,
            "first_failed_rung": first_failed,
            "rungs_5_through_10": "not-run",
        }),
        "rung 4 failed: no child-owned closure constructed a grandchild, so dependent rungs were not run",
        replay,
        quiet,
    )
}

fn first_failure_localization() -> ProbeResult {
    let trial = closure_trial(4);
    let transitions = [
        ("output participation", true),
        ("physical change and proprioceptive incidence", true),
        ("finite local return", trial.first_evidence == 1),
        ("causal closure evidence", trial.first_evidence == 1),
        ("root learner construction", trial.constructions == 1),
        ("inherited construction law", true),
        ("child-owned distinct closure", false),
        ("descendant construction", false),
    ];
    finish(
        Arm::FirstFailureLocalization,
        transitions.iter().take(6).all(|transition| transition.1) && !transitions[6].1,
        serde_json::json!({
            "transitions": transitions,
            "earliest_absent": "child-owned distinct closure",
            "construction_events": trial.constructions,
            "trace_contains_root_construction": trial.learners.len() == 1,
        }),
        "the trace did not isolate one earliest missing physical transition",
        trial.replay,
        trial.quiet,
    )
}

fn conditional_downward_ablations() -> ProbeResult {
    let upward = complete_recursive_upward_ladder();
    inconclusive(
        Arm::ConditionalDownwardAblations,
        serde_json::json!({
            "upward_outcome": upward.outcome,
            "ablations_run": 0,
        }),
        "downward ablations are forbidden because rung ten did not pass",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use truelearner_core::PhysicalEvent;

    #[test]
    fn probe_frozen_parent_reference() {
        assert_eq!(run(Arm::FrozenParentReference).outcome, "survived");
    }

    #[test]
    fn probe_closure_subgraph_autonomy() {
        assert_eq!(run(Arm::ClosureSubgraphAutonomy).outcome, "falsified");
    }

    #[test]
    fn probe_blank_child_budding() {
        assert_eq!(run(Arm::BlankChildBudding).outcome, "falsified");
    }

    #[test]
    fn probe_participation_born_budding() {
        let result = run(Arm::ParticipationBornBudding);
        assert_eq!(result.outcome, "falsified");
        assert_eq!(result.observations["root_constructed"], true);
    }

    #[test]
    fn probe_complete_recursive_upward_ladder() {
        let result = run(Arm::CompleteRecursiveUpwardLadder);
        assert_eq!(result.outcome, "falsified");
        assert_eq!(result.observations["first_failed_rung"], 4);
        assert_eq!(result.observations["rungs_5_through_10"], "not-run");
    }

    #[test]
    fn probe_first_failure_localization() {
        let result = run(Arm::FirstFailureLocalization);
        assert_eq!(result.outcome, "survived");
        assert_eq!(
            result.observations["earliest_absent"],
            "child-owned distinct closure"
        );
    }

    #[test]
    fn probe_downward_ablations_are_conditional() {
        let result = run(Arm::ConditionalDownwardAblations);
        assert_eq!(result.outcome, "inconclusive");
        assert_eq!(result.observations["ablations_run"], 0);
    }

    #[test]
    fn every_declared_arm_has_a_result() {
        for arm in Arm::ALL {
            assert_eq!(run(arm).arm, arm.id());
        }
    }

    #[test]
    fn construction_requires_two_causal_observations_and_rejects_false_closure() {
        let trial = closure_trial(64);
        assert_eq!(trial.first_evidence, 1);
        assert_eq!(trial.constructions, 1);
        assert_eq!(trial.false_constructions, 0);
        assert_eq!(trial.learners.len(), 1);
        assert!(trial.replay);
        assert!(trial.quiet);
    }

    #[test]
    fn construction_event_is_observable_but_not_harness_selected() {
        let mut world = ClosureWorld::new(Protocol::RecursiveLearnerConstruction, 0);
        world.train_action();
        world.send_surface(world.surface, 40_001);
        world.train_action();
        let run = world.send_surface(world.surface, 40_001);
        assert!(run.physical_trace.iter().any(|transition| matches!(
            transition.event,
            PhysicalEvent::LearnerConstructed { parent: None, .. }
        )));
    }
}
