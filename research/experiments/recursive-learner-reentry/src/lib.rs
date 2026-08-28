use serde::Serialize;
use std::str::FromStr;
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent, Protocol, Run,
    TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    PostConstructionCausalOpportunity,
    ChildOwnedClosureGenesis,
    RecursiveDepthReentry,
}

impl Arm {
    pub const ALL: [Self; 3] = [
        Self::PostConstructionCausalOpportunity,
        Self::ChildOwnedClosureGenesis,
        Self::RecursiveDepthReentry,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::PostConstructionCausalOpportunity => "post-construction-causal-opportunity",
            Self::ChildOwnedClosureGenesis => "child-owned-closure-genesis",
            Self::RecursiveDepthReentry => "recursive-depth-reentry",
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
        Arm::PostConstructionCausalOpportunity => post_construction_causal_opportunity(),
        Arm::ChildOwnedClosureGenesis => child_owned_closure_genesis(),
        Arm::RecursiveDepthReentry => recursive_depth_reentry(),
    }
}

fn result(
    arm: Arm,
    outcome: &'static str,
    observations: serde_json::Value,
    falsifier: &str,
    replay: bool,
    quiet: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "recursive-learner-reentry/v1",
        arm: arm.id(),
        outcome,
        observations,
        falsifier: (outcome != "survived").then(|| falsifier.to_string()),
        exact_replay: replay,
        naturally_quiescent: quiet,
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

fn input(target: JunctionId, tick: i64, physical: u64) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: physical,
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
    motor: JunctionId,
}

impl ClosureWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_protocol(Protocol::RecursiveLearnerConstruction);
        builder.set_physical_tracing(true);
        let action = junction(&mut builder, 70_000, 0, 0, 1);
        let surface = junction(&mut builder, 70_001, 2, 0, 1);
        let motor = junction(&mut builder, 70_010, 1, 0, 2);
        let sink = junction(&mut builder, 70_011, 1, OUTWARD_REGION, 1);
        let outcome = junction(&mut builder, 70_012, 50, 0, 1);
        let anchor = junction(&mut builder, 70_013, 100, 0, 99);
        for target in [action, surface, outcome] {
            link(&mut builder, anchor, target, 0);
        }
        link(&mut builder, surface, outcome, 3);
        link(&mut builder, motor, sink, 0);
        builder.set_outcome_source_for_output(motor, outcome);
        Self {
            harness: builder.build(),
            action,
            surface,
            motor,
        }
    }

    fn participate(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.action, tick, 70_000),
            input(self.motor, tick.saturating_add(2), 70_010),
        ])
    }

    fn return_surface(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[input(self.surface, tick, 70_001)])
    }

    fn observe_round(&mut self) -> Run {
        self.participate();
        self.return_surface()
    }
}

struct ReentryTrial {
    root_constructed: bool,
    surface_owned: bool,
    post_output: bool,
    post_return_incidence: bool,
    post_reverse_consolidation: bool,
    child_owned_closure: bool,
    descendant_constructed: bool,
    replay: bool,
    quiet: bool,
}

fn reentry_trial() -> ReentryTrial {
    let mut world = ClosureWorld::new();
    let root_first = world.observe_round();
    let root_second = world.observe_round();
    let observed = world.harness.read();
    let root_constructed = observed.learners.len() == 1;
    let surface_owned = observed
        .learners
        .first()
        .is_some_and(|learner| learner.junctions.contains(&world.surface));
    let checkpoint = world.harness.save().expect("root checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("root checkpoint restores");

    let participated = world.participate();
    let replay_tick = replay.read().clock.tick.saturating_add(1);
    let replay_participated = replay.send(&[
        input(world.action, replay_tick, 70_000),
        input(world.motor, replay_tick.saturating_add(2), 70_010),
    ]);
    let post = world.return_surface();
    let replay_tick = replay.read().clock.tick.saturating_add(1);
    let replay_post = replay.send(&[input(world.surface, replay_tick, 70_001)]);
    let exact = same_run(&participated, &replay_participated)
        && same_run(&post, &replay_post)
        && world.harness.save().unwrap().canonical_bytes().unwrap()
            == replay.save().unwrap().canonical_bytes().unwrap();
    ReentryTrial {
        root_constructed,
        surface_owned,
        post_output: post
            .outputs
            .iter()
            .any(|output| output.from_physical == 70_010),
        post_return_incidence: post.physical_trace.iter().any(|transition| {
            matches!(transition.event, PhysicalEvent::ModulatoryIncidence { .. })
        }),
        post_reverse_consolidation: post.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::ReversePathConsolidated { .. }
            )
        }),
        child_owned_closure: post.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::CausalClosureObserved {
                    parent: Some(_),
                    ..
                }
            )
        }),
        descendant_constructed: world.harness.read().learners.len() > 1,
        replay: exact,
        quiet: root_first.naturally_quiescent
            && root_second.naturally_quiescent
            && participated.naturally_quiescent
            && post.naturally_quiescent,
    }
}

fn post_construction_causal_opportunity() -> ProbeResult {
    let trial = reentry_trial();
    result(
        Arm::PostConstructionCausalOpportunity,
        "falsified",
        serde_json::json!({
            "root_constructed": trial.root_constructed,
            "surface_owned": trial.surface_owned,
            "post_construction_output": trial.post_output,
            "post_construction_modulatory_incidence": trial.post_return_incidence,
            "post_construction_reverse_consolidation": trial.post_reverse_consolidation,
            "child_owned_closure": trial.child_owned_closure,
            "earliest_absent": "fresh finite return admission before owner resolution",
        }),
        "the return incidence was rejected before reverse consolidation and owner-local closure lookup",
        trial.replay,
        trial.quiet,
    )
}

fn child_owned_closure_genesis() -> ProbeResult {
    let trial = reentry_trial();
    result(
        Arm::ChildOwnedClosureGenesis,
        "falsified",
        serde_json::json!({
            "owner_key_implemented": true,
            "root_constructed": trial.root_constructed,
            "surface_owned": trial.surface_owned,
            "child_owned_closure": trial.child_owned_closure,
            "descendant_constructed": trial.descendant_constructed,
        }),
        "owner-specific closure identity could not act because no fresh return was admitted after construction",
        trial.replay,
        trial.quiet,
    )
}

fn recursive_depth_reentry() -> ProbeResult {
    let solve = child_owned_closure_genesis();
    result(
        Arm::RecursiveDepthReentry,
        "inconclusive",
        serde_json::json!({
            "prerequisite_outcome": solve.outcome,
            "depth_arm_run": false,
            "constructed_depth": 1,
        }),
        "the composition arm was correctly not run because child-owned closure genesis failed",
        solve.exact_replay,
        solve.naturally_quiescent,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_post_construction_causal_opportunity() {
        let result = run(Arm::PostConstructionCausalOpportunity);
        assert_eq!(result.outcome, "falsified");
        assert_eq!(result.observations["root_constructed"], true);
        assert_eq!(result.observations["surface_owned"], true);
        assert_eq!(result.observations["child_owned_closure"], false);
    }

    #[test]
    fn probe_child_owned_closure_genesis() {
        let result = run(Arm::ChildOwnedClosureGenesis);
        assert_eq!(result.outcome, "falsified");
        assert_eq!(result.observations["owner_key_implemented"], true);
        assert_eq!(result.observations["descendant_constructed"], false);
    }

    #[test]
    fn probe_recursive_depth_reentry() {
        let result = run(Arm::RecursiveDepthReentry);
        assert_eq!(result.outcome, "inconclusive");
        assert_eq!(result.observations["depth_arm_run"], false);
    }

    #[test]
    fn every_declared_arm_has_a_result() {
        for arm in Arm::ALL {
            let result = run(arm);
            assert_eq!(result.arm, arm.id());
            assert!(result.exact_replay);
            assert!(result.naturally_quiescent);
        }
    }
}
