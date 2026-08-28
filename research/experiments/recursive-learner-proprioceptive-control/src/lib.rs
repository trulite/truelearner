use serde::Serialize;
use std::str::FromStr;
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent, Protocol, Run,
    TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    CurrentProprioceptiveOwnerGating,
    GlobalConsequenceReference,
    OwnerLocalRecentConsequence,
    RecursiveProprioceptiveControlComposition,
}

impl Arm {
    pub const ALL: [Self; 4] = [
        Self::CurrentProprioceptiveOwnerGating,
        Self::GlobalConsequenceReference,
        Self::OwnerLocalRecentConsequence,
        Self::RecursiveProprioceptiveControlComposition,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::CurrentProprioceptiveOwnerGating => "current-proprioceptive-owner-gating",
            Self::GlobalConsequenceReference => "global-consequence-reference",
            Self::OwnerLocalRecentConsequence => "owner-local-recent-consequence",
            Self::RecursiveProprioceptiveControlComposition => {
                "recursive-proprioceptive-control-composition"
            }
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
    let trial = proprioceptive_control_trial();
    match arm {
        Arm::CurrentProprioceptiveOwnerGating => gating_result(trial),
        Arm::GlobalConsequenceReference => global_reference_result(trial),
        Arm::OwnerLocalRecentConsequence => consequence_result(trial),
        Arm::RecursiveProprioceptiveControlComposition => composition_result(trial),
    }
}

fn result(
    arm: Arm,
    outcome: &'static str,
    observations: serde_json::Value,
    falsifier: Option<&str>,
    replay: bool,
    quiet: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "recursive-learner-proprioceptive-control/v1",
        arm: arm.id(),
        outcome,
        observations,
        falsifier: falsifier.map(str::to_owned),
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

fn input(target: JunctionId, tick: i64, physical: u64, impulse: i32) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: physical,
        target,
        impulse,
    }
}

struct ControlWorld {
    harness: Harness,
    action: JunctionId,
    surface: JunctionId,
    motor: JunctionId,
    controlled: JunctionId,
    controlled_outcome: JunctionId,
}

impl ControlWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_protocol(Protocol::RecursiveLearnerConstruction);
        builder.set_physical_tracing(true);
        let action = junction(&mut builder, 76_000, 0, 0, 1);
        let surface = junction(&mut builder, 76_001, 2, 0, 1);
        let unrelated = junction(&mut builder, 76_002, 20, 0, 1);
        let motor = junction(&mut builder, 76_010, 1, 0, 3);
        let sink = junction(&mut builder, 76_011, 1, OUTWARD_REGION, 1);
        let outcome = junction(&mut builder, 76_012, 50, 0, 1);
        let anchor = junction(&mut builder, 76_013, 100, 0, 99);
        let controlled = junction(&mut builder, 76_020, 30, 0, 2);
        let controlled_sink = junction(&mut builder, 76_021, 30, OUTWARD_REGION, 1);
        let controlled_path = junction(&mut builder, 76_022, 2, 0, 1);
        let controlled_outcome = junction(&mut builder, 76_023, 60, 0, 1);
        for target in [action, surface, unrelated, outcome, controlled_outcome] {
            link(&mut builder, anchor, target, 0);
        }
        for source in [surface, unrelated] {
            link(&mut builder, source, outcome, 3);
        }
        link(&mut builder, motor, sink, 0);
        link(&mut builder, surface, controlled_path, 0);
        link(&mut builder, controlled_path, controlled, 2);
        link(&mut builder, controlled, controlled_sink, 0);
        builder.set_outcome_source_for_output(motor, outcome);
        builder.set_outcome_source_for_output(controlled, controlled_outcome);
        Self {
            harness: builder.build(),
            action,
            surface,
            motor,
            controlled,
            controlled_outcome,
        }
    }

    fn observe_closure(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.action, tick, 76_000, 1),
            input(self.motor, tick.saturating_add(2), 76_010, 2),
        ]);
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[input(self.surface, tick, 76_001, 1)])
    }

    fn stimulate(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.surface, tick, 76_001, 1),
            input(self.controlled, tick.saturating_add(2), 76_001, 1),
        ])
    }

    fn controlled_consequence(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness
            .send(&[input(self.controlled_outcome, tick, 76_001, 1)])
    }
}

fn has_output(run: &Run, physical: u64) -> bool {
    run.outputs
        .iter()
        .any(|output| output.from_physical == physical)
}

fn stimulate_restored(
    harness: &mut Harness,
    surface: JunctionId,
    controlled: JunctionId,
    opportunity_origin: u64,
) -> Run {
    let tick = harness.read().clock.tick.saturating_add(1);
    harness.send(&[
        input(surface, tick, 76_001, 1),
        input(controlled, tick.saturating_add(2), opportunity_origin, 1),
    ])
}

#[derive(Clone, Copy)]
struct Trial {
    root_constructed: bool,
    current_admitted: bool,
    absent_rejected: bool,
    shifted_rejected: bool,
    unrelated_rejected: bool,
    proprioceptive_trace: bool,
    global_history_present_at_birth: bool,
    fresh_owner_ignored_global_history: bool,
    private_write: bool,
    private_read: bool,
    bounded_release: bool,
    depth: usize,
    adjacent_ancestry: bool,
    deepest_owner_write: bool,
    deepest_owner_read: bool,
    exact_replay: bool,
    quiet: bool,
}

fn proprioceptive_control_trial() -> Trial {
    let mut world = ControlWorld::new();
    let first = world.observe_closure();
    let second = world.observe_closure();
    let root = world.harness.read().learners[0].id;
    let birth = world.harness.save().expect("birth checkpoint saves");
    let global_history_present_at_birth = world
        .harness
        .read()
        .links
        .iter()
        .any(|link| link.last_consequence_tick.is_some());

    let current = world.stimulate();
    let current_admitted = has_output(&current, 76_020);
    let proprioceptive_trace = current.physical_trace.iter().any(|transition| {
        matches!(
            transition.event,
            PhysicalEvent::ProprioceptiveOpportunity {
                owner,
                origin_physical: 76_001,
                admitted: true,
                ..
            } if owner == root
        )
    });
    let fresh_owner_ignored_global_history = current_admitted
        && current.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::LearnerCandidatePreference {
                    owner,
                    consequence_tick: None,
                    admitted: true,
                    ..
                } if owner == root
            )
        });

    let mut absent = Harness::restore(birth.clone()).expect("absent restores");
    let tick = absent.read().clock.tick.saturating_add(1);
    let absent_run = absent.send(&[input(world.surface, tick, 76_001, 1)]);
    let absent_rejected = !has_output(&absent_run, 76_020);

    let mut shifted = Harness::restore(birth.clone()).expect("shifted restores");
    let tick = shifted.read().clock.tick.saturating_add(1);
    let shifted_run = shifted.send(&[
        input(world.controlled, tick, 76_001, 1),
        input(world.surface, tick.saturating_add(1), 76_001, 1),
    ]);
    let shifted_rejected = !has_output(&shifted_run, 76_020);

    let mut unrelated = Harness::restore(birth).expect("unrelated restores");
    let unrelated_run = stimulate_restored(&mut unrelated, world.surface, world.controlled, 76_002);
    let unrelated_rejected = !has_output(&unrelated_run, 76_020)
        && unrelated_run.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::ProprioceptiveOpportunity {
                    owner,
                    origin_physical: 76_002,
                    admitted: false,
                    ..
                } if owner == root
            )
        });

    let consequence = world.controlled_consequence();
    let private_tick =
        consequence
            .physical_trace
            .iter()
            .find_map(|transition| match transition.event {
                PhysicalEvent::LearnerConsequenceRecorded { owner, tick, .. } if owner == root => {
                    Some(tick)
                }
                _ => None,
            });
    let private_write = private_tick.is_some();
    let learned = world.harness.save().expect("learned checkpoint saves");
    let recent = world.stimulate();
    let private_read = has_output(&recent, 76_020)
        && recent.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::LearnerCandidatePreference {
                    owner,
                    consequence_tick: Some(tick),
                    admitted: true,
                    ..
                } if owner == root && Some(tick) == private_tick
            )
        });

    let mut released_outputs = Vec::new();
    let mut released_quiet = true;
    for offset in 6..10 {
        let mut released = Harness::restore(learned.clone()).expect("release restores");
        released.advance_to(released.read().clock.tick.saturating_add(offset));
        let run = stimulate_restored(&mut released, world.surface, world.controlled, 76_001);
        released_outputs.extend(run.outputs.iter().map(|output| output.from_physical));
        released_quiet &= run.naturally_quiescent;
    }
    let bounded_release = released_outputs.contains(&76_010);

    let mut replay = Harness::restore(learned.clone()).expect("replay restores");
    let mut direct = Harness::restore(learned).expect("direct restores");
    let replay_run = stimulate_restored(&mut replay, world.surface, world.controlled, 76_001);
    let direct_run = stimulate_restored(&mut direct, world.surface, world.controlled, 76_001);
    let exact_replay = replay_run == direct_run
        && replay.save().unwrap().canonical_bytes().unwrap()
            == direct.save().unwrap().canonical_bytes().unwrap();

    let depth_trial = recursive_depth_trial();
    Trial {
        root_constructed: first.work.causal_closure_observations == 1
            && second.work.learner_constructions == 1,
        current_admitted,
        absent_rejected,
        shifted_rejected,
        unrelated_rejected,
        proprioceptive_trace,
        global_history_present_at_birth,
        fresh_owner_ignored_global_history,
        private_write,
        private_read,
        bounded_release,
        depth: depth_trial.depth,
        adjacent_ancestry: depth_trial.adjacent_ancestry,
        deepest_owner_write: depth_trial.deepest_owner_write,
        deepest_owner_read: depth_trial.deepest_owner_read,
        exact_replay,
        quiet: [
            &first,
            &second,
            &current,
            &absent_run,
            &shifted_run,
            &unrelated_run,
            &consequence,
            &recent,
            &replay_run,
            &direct_run,
        ]
        .iter()
        .all(|run| run.naturally_quiescent)
            && released_quiet
            && depth_trial.quiet,
    }
}

#[derive(Clone, Copy)]
struct DepthTrial {
    depth: usize,
    adjacent_ancestry: bool,
    deepest_owner_write: bool,
    deepest_owner_read: bool,
    quiet: bool,
}

fn recursive_depth_trial() -> DepthTrial {
    let mut world = ControlWorld::new();
    let mut runs = Vec::new();
    for _ in 0..6 {
        runs.push(world.observe_closure());
    }
    let learners = world.harness.read().learners;
    let depth = learners.len();
    let adjacent_ancestry = depth == 3
        && learners[0].parent.is_none()
        && learners[1].parent == Some(learners[0].id)
        && learners[2].parent == Some(learners[1].id);
    let deepest = learners.last().map(|learner| learner.id);
    let consequence = world.observe_closure();
    let deepest_owner_write = deepest.is_some_and(|deepest| {
        consequence.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::LearnerConsequenceRecorded { owner, .. } if owner == deepest
            )
        })
    });
    let read = world.stimulate();
    let deepest_owner_read = deepest.is_some_and(|deepest| {
        read.physical_trace.iter().any(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::LearnerCandidatePreference { owner, .. } if owner == deepest
            )
        })
    });
    let quiet = runs
        .iter()
        .chain([&consequence, &read])
        .all(|run| run.naturally_quiescent);
    DepthTrial {
        depth,
        adjacent_ancestry,
        deepest_owner_write,
        deepest_owner_read,
        quiet,
    }
}

fn gating_result(trial: Trial) -> ProbeResult {
    let survived = trial.root_constructed
        && trial.current_admitted
        && trial.absent_rejected
        && trial.shifted_rejected
        && trial.unrelated_rejected
        && trial.proprioceptive_trace
        && trial.exact_replay
        && trial.quiet;
    result(
        Arm::CurrentProprioceptiveOwnerGating,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "root_constructed": trial.root_constructed,
            "current_same_owner_admitted": trial.current_admitted,
            "absent_rejected": trial.absent_rejected,
            "one_tick_shifted_rejected": trial.shifted_rejected,
            "unrelated_owner_rejected": trial.unrelated_rejected,
            "owner_bearing_opportunity_trace": trial.proprioceptive_trace,
        }),
        (!survived).then_some(
            "current owner-local opportunity gating failed a timing or causal-origin control",
        ),
        trial.exact_replay,
        trial.quiet,
    )
}

fn global_reference_result(trial: Trial) -> ProbeResult {
    let falsified =
        trial.global_history_present_at_birth && trial.fresh_owner_ignored_global_history;
    result(
        Arm::GlobalConsequenceReference,
        if falsified {
            "falsified"
        } else {
            "inconclusive"
        },
        serde_json::json!({
            "global_link_history_present_at_learner_birth": trial.global_history_present_at_birth,
            "fresh_owner_selected_with_no_private_tick": trial.fresh_owner_ignored_global_history,
            "global_history_sufficient_for_isolated_child_preference": false,
        }),
        falsified.then_some(
            "global link consequence history cannot provide fresh owner-local preference isolation",
        ),
        trial.exact_replay,
        trial.quiet,
    )
}

fn consequence_result(trial: Trial) -> ProbeResult {
    let survived = trial.private_write
        && trial.private_read
        && trial.bounded_release
        && trial.exact_replay
        && trial.quiet;
    result(
        Arm::OwnerLocalRecentConsequence,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "accepted_participating_return_wrote_private_generation": trial.private_write,
            "later_owned_selection_read_private_tick": trial.private_read,
            "preference_released_after_recent_window": trial.bounded_release,
            "checkpoint_replay_exact": trial.exact_replay,
        }),
        (!survived).then_some("owner-local consequence write, read, release, or replay failed"),
        trial.exact_replay,
        trial.quiet,
    )
}

fn composition_result(trial: Trial) -> ProbeResult {
    let survived = trial.depth == 3
        && trial.adjacent_ancestry
        && trial.deepest_owner_write
        && trial.deepest_owner_read
        && trial.current_admitted
        && trial.private_write
        && trial.private_read
        && trial.bounded_release
        && trial.exact_replay
        && trial.quiet;
    result(
        Arm::RecursiveProprioceptiveControlComposition,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "constructed_depth": trial.depth,
            "adjacent_ancestry": trial.adjacent_ancestry,
            "deepest_owner_consequence_write": trial.deepest_owner_write,
            "deepest_owner_preference_read": trial.deepest_owner_read,
            "primitive_gating_preserved": trial.current_admitted,
            "bounded_local_control_preserved": trial.private_write && trial.private_read && trial.bounded_release,
        }),
        (!survived).then_some("recursive composition lost a primitive, adjacent owner, replay, or quiescence invariant"),
        trial.exact_replay,
        trial.quiet,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_current_proprioceptive_owner_gating() {
        let result = run(Arm::CurrentProprioceptiveOwnerGating);
        assert_eq!(result.outcome, "survived", "{result:#?}");
    }

    #[test]
    fn probe_global_consequence_reference() {
        let result = run(Arm::GlobalConsequenceReference);
        assert_eq!(result.outcome, "falsified", "{result:#?}");
    }

    #[test]
    fn probe_owner_local_recent_consequence() {
        let result = run(Arm::OwnerLocalRecentConsequence);
        assert_eq!(result.outcome, "survived", "{result:#?}");
    }

    #[test]
    fn probe_recursive_proprioceptive_control_composition() {
        let result = run(Arm::RecursiveProprioceptiveControlComposition);
        assert_eq!(result.outcome, "survived", "{result:#?}");
    }
}
