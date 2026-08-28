use serde::Serialize;
use std::str::FromStr;
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, LearnerId, Link, LinkId, PhysicalEvent,
    Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    ReturnEligibilityLifetimeLocalization,
    ConstructionScopedReturnReopening,
    OwnerParticipationReturnEligibility,
    ReturnReentryComposition,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::ReturnEligibilityLifetimeLocalization,
        Self::ConstructionScopedReturnReopening,
        Self::OwnerParticipationReturnEligibility,
        Self::ReturnReentryComposition,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::ReturnEligibilityLifetimeLocalization => {
                "return-eligibility-lifetime-localization"
            }
            Self::ConstructionScopedReturnReopening => "construction-scoped-return-reopening",
            Self::OwnerParticipationReturnEligibility => "owner-participation-return-eligibility",
            Self::ReturnReentryComposition => "return-reentry-composition",
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
    let trial = fresh_memory_trial();
    match arm {
        Arm::ReturnEligibilityLifetimeLocalization => diagnostic_result(trial),
        Arm::ConstructionScopedReturnReopening => construction_scoped_result(trial),
        Arm::OwnerParticipationReturnEligibility => owner_memory_result(trial),
        Arm::ReturnReentryComposition => composition_result(trial),
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
        schema: "recursive-learner-fresh-memory/v1",
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

struct ClosureWorld {
    harness: Harness,
    action: JunctionId,
    surface: JunctionId,
    unrelated: JunctionId,
    motor: JunctionId,
}

impl ClosureWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_protocol(Protocol::RecursiveLearnerConstruction);
        builder.set_physical_tracing(true);
        let action = junction(&mut builder, 75_000, 0, 0, 1);
        let surface = junction(&mut builder, 75_001, 2, 0, 1);
        let unrelated = junction(&mut builder, 75_002, 20, 0, 1);
        let motor = junction(&mut builder, 75_010, 1, 0, 2);
        let sink = junction(&mut builder, 75_011, 1, OUTWARD_REGION, 1);
        let outcome = junction(&mut builder, 75_012, 50, 0, 1);
        let anchor = junction(&mut builder, 75_013, 100, 0, 99);
        for target in [action, surface, unrelated, outcome] {
            link(&mut builder, anchor, target, 0);
        }
        for source in [surface, unrelated] {
            link(&mut builder, source, outcome, 3);
        }
        link(&mut builder, motor, sink, 0);
        builder.set_outcome_source_for_output(motor, outcome);
        Self {
            harness: builder.build(),
            action,
            surface,
            unrelated,
            motor,
        }
    }

    fn observe(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.action, tick, 75_000),
            input(self.motor, tick.saturating_add(2), 75_010),
        ]);
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[input(self.surface, tick, 75_001)])
    }

    fn unrelated(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[input(self.unrelated, tick, 75_002)])
    }

    fn expire_returns(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(64);
        self.harness.send(&[input(self.unrelated, tick, 75_002)])
    }
}

fn accepted_owner(run: &Run) -> Option<(LearnerId, LinkId, u32)> {
    run.physical_trace
        .iter()
        .find_map(|transition| match transition.event {
            PhysicalEvent::ReturnOriginAdmission {
                owner: Some(owner),
                link,
                generation,
                admitted: true,
                ..
            } => Some((owner, link, generation)),
            _ => None,
        })
}

fn rejected_tuple(run: &Run, expected: (LearnerId, LinkId, u32)) -> bool {
    run.physical_trace.iter().any(|transition| {
        matches!(
            transition.event,
            PhysicalEvent::ReturnOriginAdmission {
                owner: Some(owner),
                link,
                generation,
                admitted: false,
                ..
            } if (owner, link, generation) == expected
        )
    })
}

#[derive(Clone, Copy)]
struct Trial {
    root_constructed: bool,
    scheduling_owner_visible: bool,
    child_admission: bool,
    child_reverse_consolidation: bool,
    parent_history_preserved: bool,
    duplicate_rejected: bool,
    distinct_second_return: bool,
    renewed_generation_admitted: bool,
    unrelated_rejected: bool,
    constructed_depth: usize,
    adjacent_ancestry: bool,
    exact_replay: bool,
    quiet: bool,
}

fn fresh_memory_trial() -> Trial {
    let mut world = ClosureWorld::new();
    let root_first = world.observe();
    let root_second = world.observe();
    let root = world.harness.read().learners[0].id;
    let parent_history = world
        .harness
        .read()
        .links
        .iter()
        .map(|link| (link.id, link.return_origins.clone()))
        .collect::<Vec<_>>();
    let checkpoint = world.harness.save().expect("root checkpoint saves");
    let replay = Harness::restore(checkpoint).expect("root checkpoint restores");

    let child_first = world.observe();
    let mut replay_world = ClosureWorld {
        harness: replay,
        action: world.action,
        surface: world.surface,
        unrelated: world.unrelated,
        motor: world.motor,
    };
    let child_first_replayed = replay_world.observe();
    let first = accepted_owner(&child_first).expect("child admits a fresh return");
    let parent_history_preserved = parent_history.iter().all(|(id, origins)| {
        world
            .harness
            .read()
            .links
            .iter()
            .find(|link| link.id == *id)
            .is_some_and(|link| link.return_origins == *origins)
    });
    let child_second = world.observe();
    let child_second_replayed = replay_world.observe();
    let second = accepted_owner(&child_second).expect("second fresh return is admitted");
    let checkpoint_equal = world.harness.save().unwrap().canonical_bytes().unwrap()
        == replay_world
            .harness
            .save()
            .unwrap()
            .canonical_bytes()
            .unwrap();

    let grandchild_first = world.observe();
    let grandchild_second = world.observe();
    let unrelated = world.unrelated();
    let expired = world.expire_returns();
    let renewed = world.observe();
    let renewed_owner = accepted_owner(&renewed);
    let learners = world.harness.read().learners;
    let adjacent_ancestry = learners.len() == 3
        && learners[0].parent.is_none()
        && learners[1].parent == Some(learners[0].id)
        && learners[2].parent == Some(learners[1].id);
    let quiet = [
        &root_first,
        &root_second,
        &child_first,
        &child_second,
        &grandchild_first,
        &grandchild_second,
        &unrelated,
        &expired,
        &renewed,
    ]
    .iter()
    .all(|run| run.naturally_quiescent);

    Trial {
        root_constructed: root_second.work.learner_constructions == 1,
        scheduling_owner_visible: child_first.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::ReturnScheduling {
                    owner: Some(owner),
                    ..
                } if owner == root
            )
        }),
        child_admission: first.0 == root,
        child_reverse_consolidation: child_first.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::ReversePathConsolidated { .. }
            )
        }),
        parent_history_preserved,
        duplicate_rejected: rejected_tuple(&child_second, first),
        distinct_second_return: second.0 == root && (second.1, second.2) != (first.1, first.2),
        renewed_generation_admitted: renewed_owner
            .is_some_and(|(owner, _, generation)| owner == learners[2].id && generation > 1),
        unrelated_rejected: unrelated.work.causal_closure_observations == 0,
        constructed_depth: learners.len(),
        adjacent_ancestry,
        exact_replay: child_first == child_first_replayed
            && child_second == child_second_replayed
            && checkpoint_equal,
        quiet,
    }
}

fn diagnostic_result(trial: Trial) -> ProbeResult {
    let survived = trial.root_constructed
        && trial.scheduling_owner_visible
        && trial.child_admission
        && trial.child_reverse_consolidation;
    result(
        Arm::ReturnEligibilityLifetimeLocalization,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "root_constructed": trial.root_constructed,
            "owner_bearing_scheduling": trial.scheduling_owner_visible,
            "child_owned_admission": trial.child_admission,
            "post_admission_reverse_consolidation": trial.child_reverse_consolidation,
            "localized_transition": "owner-local physical-origin return admission",
        }),
        "the trace did not localize return eligibility between modulatory incidence and reverse consolidation",
        trial.exact_replay,
        trial.quiet,
    )
}

fn construction_scoped_result(trial: Trial) -> ProbeResult {
    result(
        Arm::ConstructionScopedReturnReopening,
        "falsified",
        serde_json::json!({
            "first_child_return_admitted": trial.child_admission,
            "same_owner_link_generation_origin_rejected": trial.duplicate_rejected,
            "distinct_second_return_admitted": trial.distinct_second_return,
            "persistent_owner_memory_required": true,
        }),
        "construction-only reopening cannot both remember the rejected tuple and admit a distinct later participation",
        trial.exact_replay,
        trial.quiet,
    )
}

fn owner_memory_result(trial: Trial) -> ProbeResult {
    let survived = trial.child_admission
        && trial.parent_history_preserved
        && trial.duplicate_rejected
        && trial.distinct_second_return
        && trial.renewed_generation_admitted;
    result(
        Arm::OwnerParticipationReturnEligibility,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "child_owned_admission": trial.child_admission,
            "parent_history_preserved": trial.parent_history_preserved,
            "duplicate_rejected": trial.duplicate_rejected,
            "distinct_second_return": trial.distinct_second_return,
            "renewed_generation_admitted": trial.renewed_generation_admitted,
        }),
        "owner-local memory failed freshness, duplicate, parent-preservation, or renewal",
        trial.exact_replay,
        trial.quiet,
    )
}

fn composition_result(trial: Trial) -> ProbeResult {
    let survived = trial.constructed_depth == 3
        && trial.adjacent_ancestry
        && trial.unrelated_rejected
        && trial.exact_replay
        && trial.quiet;
    result(
        Arm::ReturnReentryComposition,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "constructed_depth": trial.constructed_depth,
            "adjacent_ancestry": trial.adjacent_ancestry,
            "unrelated_return_rejected": trial.unrelated_rejected,
            "exact_replay": trial.exact_replay,
            "naturally_quiescent": trial.quiet,
        }),
        "fresh return memory did not compose through exact adjacent depth three",
        trial.exact_replay,
        trial.quiet,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_return_eligibility_lifetime_localization() {
        let result = run(Arm::ReturnEligibilityLifetimeLocalization);
        assert_eq!(result.outcome, "survived");
        assert_eq!(result.observations["child_owned_admission"], true);
    }

    #[test]
    fn probe_construction_scoped_return_reopening() {
        let result = run(Arm::ConstructionScopedReturnReopening);
        assert_eq!(result.outcome, "falsified");
        assert_eq!(
            result.observations["persistent_owner_memory_required"],
            true
        );
    }

    #[test]
    fn probe_owner_participation_return_eligibility() {
        let result = run(Arm::OwnerParticipationReturnEligibility);
        assert_eq!(result.outcome, "survived");
        assert_eq!(result.observations["duplicate_rejected"], true);
        assert_eq!(result.observations["renewed_generation_admitted"], true);
    }

    #[test]
    fn probe_return_reentry_composition() {
        let result = run(Arm::ReturnReentryComposition);
        assert_eq!(result.outcome, "survived");
        assert_eq!(result.observations["constructed_depth"], 3);
        assert_eq!(result.observations["adjacent_ancestry"], true);
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
