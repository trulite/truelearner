#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent, Protocol, Run,
    TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arm {
    CurrentCandidateReference,
    EligibilityWriteReadLocalization,
    SurfaceReturnOpeningLocalization,
    CoalitionLineageLocalization,
    ParticipationContinuitySolves,
    SurvivingBoundariesComposition,
    OneJointRecomposition,
    BinocularHandRecomposition,
    VocalAuditoryRecomposition,
    FullBodyRecomposition,
}

impl Arm {
    pub const ALL: [Self; 10] = [
        Self::CurrentCandidateReference,
        Self::EligibilityWriteReadLocalization,
        Self::SurfaceReturnOpeningLocalization,
        Self::CoalitionLineageLocalization,
        Self::ParticipationContinuitySolves,
        Self::SurvivingBoundariesComposition,
        Self::OneJointRecomposition,
        Self::BinocularHandRecomposition,
        Self::VocalAuditoryRecomposition,
        Self::FullBodyRecomposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::CurrentCandidateReference => "current-candidate-reference",
            Self::EligibilityWriteReadLocalization => "eligibility-write-read-localization",
            Self::SurfaceReturnOpeningLocalization => "surface-return-opening-localization",
            Self::CoalitionLineageLocalization => "coalition-lineage-localization",
            Self::ParticipationContinuitySolves => "participation-continuity-solves",
            Self::SurvivingBoundariesComposition => "surviving-boundaries-composition",
            Self::OneJointRecomposition => "one-joint-recomposition",
            Self::BinocularHandRecomposition => "binocular-hand-recomposition",
            Self::VocalAuditoryRecomposition => "vocal-auditory-recomposition",
            Self::FullBodyRecomposition => "full-body-recomposition",
        }
    }
}

impl FromStr for Arm {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|arm| arm.id() == value)
            .ok_or_else(|| format!("unknown arm {value:?}"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeResult {
    pub schema: String,
    pub arm: String,
    pub outcome: String,
    pub observations: serde_json::Value,
    pub falsifier: Option<String>,
    pub exact_replay: bool,
    pub naturally_quiescent: bool,
}

pub fn run(arm: Arm) -> ProbeResult {
    match arm {
        Arm::CurrentCandidateReference => current_candidate_reference(),
        Arm::EligibilityWriteReadLocalization => eligibility_write_read_localization(),
        Arm::SurfaceReturnOpeningLocalization => surface_return_opening_localization(),
        Arm::CoalitionLineageLocalization => coalition_lineage_localization(),
        Arm::ParticipationContinuitySolves => participation_continuity_solves(),
        Arm::SurvivingBoundariesComposition => surviving_boundaries_composition(),
        Arm::OneJointRecomposition => one_joint_recomposition(),
        Arm::BinocularHandRecomposition => binocular_hand_recomposition(),
        Arm::VocalAuditoryRecomposition => vocal_auditory_recomposition(),
        Arm::FullBodyRecomposition => full_body_recomposition(),
    }
}

fn finish(
    arm: Arm,
    survived: bool,
    observations: serde_json::Value,
    falsifier: &str,
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    let passed = survived && exact_replay && naturally_quiescent;
    ProbeResult {
        schema: "sensorimotor-participation-continuity/v1".to_string(),
        arm: arm.id().to_string(),
        outcome: if passed { "survived" } else { "falsified" }.to_string(),
        observations,
        falsifier: (!passed).then(|| {
            if !exact_replay {
                "exact checkpoint replay differed".to_string()
            } else if !naturally_quiescent {
                "the fixture did not quiesce naturally".to_string()
            } else {
                falsifier.to_string()
            }
        }),
        exact_replay,
        naturally_quiescent,
    }
}

fn inconclusive(arm: Arm, observations: serde_json::Value, reason: &str) -> ProbeResult {
    ProbeResult {
        schema: "sensorimotor-participation-continuity/v1".to_string(),
        arm: arm.id().to_string(),
        outcome: "inconclusive".to_string(),
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

fn link(builder: &mut HarnessBuilder, from: JunctionId, to: JunctionId) {
    delayed_link(builder, from, to, 0);
}

fn delayed_link(builder: &mut HarnessBuilder, from: JunctionId, to: JunctionId, delay: i64) {
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

struct CleanRivalWorld {
    harness: Harness,
    sources: [JunctionId; 2],
    motors: [JunctionId; 2],
    outcomes: [JunctionId; 2],
    physical: [u64; 2],
}

impl CleanRivalWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(128, 512, OUTWARD_REGION);
        builder.set_protocol(Protocol::SensorimotorCandidate);
        builder.set_physical_tracing(true);
        let sources = [
            junction(&mut builder, 70_000, -2, 0, 1),
            junction(&mut builder, 70_001, 2, 0, 1),
        ];
        let physical = [70_010, 70_011];
        let motors = [
            junction(&mut builder, physical[0], -1, 0, 2),
            junction(&mut builder, physical[1], 1, 0, 2),
        ];
        let sinks = [
            junction(&mut builder, 70_020, -1, OUTWARD_REGION, 1),
            junction(&mut builder, 70_021, 1, OUTWARD_REGION, 1),
        ];
        let outcomes = [
            junction(&mut builder, 70_030, 50, 0, 1),
            junction(&mut builder, 70_031, 60, 0, 1),
        ];
        let anchor = junction(&mut builder, 70_040, 1_000, 0, 99);
        for target in sources.into_iter().chain(outcomes) {
            link(&mut builder, anchor, target);
        }
        for index in 0..2 {
            link(&mut builder, motors[index], sinks[index]);
            builder.set_outcome_source_for_output(motors[index], outcomes[index]);
        }
        Self {
            harness: builder.build(),
            sources,
            motors,
            outcomes,
            physical,
        }
    }

    fn participate(&mut self, index: usize) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.motors[index], tick, self.physical[index]),
            input(self.sources[index], tick + 1, 70_000 + index as u64),
        ])
    }

    fn return_outcome(&mut self, index: usize) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness
            .send(&[input(self.outcomes[index], tick, 70_030 + index as u64)])
    }

    fn train(&mut self, index: usize, repetitions: usize) -> bool {
        let mut quiet = true;
        for _ in 0..repetitions {
            quiet &= self.participate(index).naturally_quiescent;
            quiet &= self.return_outcome(index).naturally_quiescent;
        }
        quiet
    }

    fn compete_inputs(&self) -> [Input; 2] {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        [
            input(self.sources[0], tick, 70_000),
            input(self.sources[1], tick, 70_001),
        ]
    }
}

fn clean_recent_trial(ratio: usize) -> (bool, bool, bool, bool, serde_json::Value) {
    let mut world = CleanRivalWorld::new();
    let mut quiet = world.train(0, ratio);
    let incumbent_before = marked_links(&world.harness, world.motors[0]);
    quiet &= world.train(1, 1);
    let fresh_marks = marked_links(&world.harness, world.motors[1]);
    let inputs = world.compete_inputs();
    let (renewed, replay) = replay_send(&mut world.harness, &inputs);
    quiet &= renewed.naturally_quiescent;
    let continued = renewed
        .outputs
        .iter()
        .any(|output| output.from_physical == world.physical[1]);
    let incumbent_after = marked_links(&world.harness, world.motors[0]);
    world
        .harness
        .advance_to(world.harness.read().clock.tick.saturating_add(8));
    let release_inputs = world.compete_inputs();
    let (released_run, release_replay) = replay_send(&mut world.harness, &release_inputs);
    quiet &= released_run.naturally_quiescent;
    let released = !released_run
        .outputs
        .iter()
        .any(|output| output.from_physical == world.physical[1]);
    (
        continued,
        released,
        replay && release_replay,
        quiet,
        serde_json::json!({
            "ratio": ratio,
            "continued": continued,
            "released": released,
            "incumbent_before": incumbent_before,
            "incumbent_after": incumbent_after,
            "fresh_marks": fresh_marks,
            "renewed_outputs": renewed.outputs.iter().map(|output| output.from_physical).collect::<Vec<_>>(),
            "candidate_reads": candidate_trace(&renewed),
        }),
    )
}

fn clean_recent_solve() -> (bool, bool, bool, serde_json::Value) {
    let trials = [1, 2, 4, 8]
        .into_iter()
        .map(clean_recent_trial)
        .collect::<Vec<_>>();
    (
        trials.iter().all(|trial| trial.0 && trial.1),
        trials.iter().all(|trial| trial.2),
        trials.iter().all(|trial| trial.3),
        serde_json::json!({"trials": trials.iter().map(|trial| &trial.4).collect::<Vec<_>>() }),
    )
}

fn clean_coalition_solve() -> (bool, bool, bool, serde_json::Value) {
    let mut builder = HarnessBuilder::with_capacity(128, 512, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorCandidate);
    builder.set_physical_tracing(true);
    let root = junction(&mut builder, 80_000, -100, 0, 1);
    let sources = [
        junction(&mut builder, 80_001, 0, 0, 1),
        junction(&mut builder, 80_002, 10, 0, 1),
    ];
    let motors = [
        junction(&mut builder, 80_010, 1, 0, 2),
        junction(&mut builder, 80_011, 11, 0, 2),
    ];
    let sinks = [
        junction(&mut builder, 80_020, 1, OUTWARD_REGION, 1),
        junction(&mut builder, 80_021, 11, OUTWARD_REGION, 1),
    ];
    let outcomes = [
        junction(&mut builder, 80_030, 50, 0, 1),
        junction(&mut builder, 80_031, 60, 0, 1),
    ];
    let anchor = junction(&mut builder, 80_040, 1_000, 0, 99);
    for target in [root, sources[0], sources[1], outcomes[0], outcomes[1]] {
        link(&mut builder, anchor, target);
    }
    link(&mut builder, root, sources[0]);
    link(&mut builder, root, sources[1]);
    for index in 0..2 {
        link(&mut builder, motors[index], sinks[index]);
        builder.set_outcome_source_for_output(motors[index], outcomes[index]);
    }
    let mut harness = builder.build();
    let tick = harness.read().clock.tick.saturating_add(1);
    let trained = harness.send(&[
        input(motors[0], tick, 80_010),
        input(motors[1], tick, 80_011),
        input(sources[0], tick + 1, 80_001),
        input(sources[1], tick + 1, 80_002),
    ]);
    let tick = harness.read().clock.tick.saturating_add(1);
    let returned = harness.send(&[
        input(outcomes[0], tick, 80_030),
        input(outcomes[1], tick, 80_031),
    ]);
    let marks = [
        marked_links(&harness, motors[0]),
        marked_links(&harness, motors[1]),
    ];
    let tick = harness.read().clock.tick.saturating_add(1);
    let (recalled, replay) = replay_send(&mut harness, &[input(root, tick, 80_000)]);
    let outputs = recalled
        .outputs
        .iter()
        .map(|output| output.from_physical)
        .collect::<Vec<_>>();
    (
        trained.outputs.len() == 2
            && returned.work.local_return_updates >= 4
            && outputs.contains(&80_010)
            && outputs.contains(&80_011),
        replay,
        trained.naturally_quiescent && returned.naturally_quiescent && recalled.naturally_quiescent,
        serde_json::json!({
            "training_outputs": trained.outputs.iter().map(|output| output.from_physical).collect::<Vec<_>>(),
            "return_updates": returned.work.local_return_updates,
            "per_member_links": marks,
            "recalled_outputs": outputs,
            "candidate_reads": candidate_trace(&recalled),
        }),
    )
}

struct SurfaceCohort {
    recall: [bool; 2],
    unrelated: bool,
    updates: [u64; 4],
    returns_after_expiry: usize,
    consequence_links: Vec<serde_json::Value>,
    return_origins: Vec<Vec<u64>>,
    recall_reads: [Vec<serde_json::Value>; 2],
    replay: bool,
    quiet: bool,
}

fn surface_cohort(reverse: bool) -> SurfaceCohort {
    let mut builder = HarnessBuilder::with_capacity(128, 512, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorCandidate);
    let action = junction(&mut builder, 90_000, 0, 0, 1);
    let surfaces = [
        junction(&mut builder, 90_001, 0, 0, 1),
        junction(&mut builder, 90_002, 2, 0, 1),
    ];
    let unrelated = junction(&mut builder, 90_003, 20, 0, 1);
    let motor = junction(&mut builder, 90_010, 1, 0, 2);
    let sink = junction(&mut builder, 90_011, 1, OUTWARD_REGION, 1);
    let hub = junction(&mut builder, 90_012, 50, 0, 1);
    let anchor = junction(&mut builder, 90_013, 1_000, 0, 99);
    for target in [action, surfaces[0], surfaces[1], unrelated, hub] {
        link(&mut builder, anchor, target);
    }
    link(&mut builder, motor, sink);
    for surface in surfaces.into_iter().chain([unrelated]) {
        delayed_link(&mut builder, surface, hub, 3);
    }
    builder.set_outcome_source_for_output(motor, hub);
    let mut harness = builder.build();
    let trained = harness.send(&[input(motor, 1, 90_010), input(action, 2, 90_000)]);
    let order = if reverse {
        [surfaces[1], surfaces[0]]
    } else {
        surfaces
    };
    let tick = harness.read().clock.tick.saturating_add(1);
    let first = harness.send(&[input(order[0], tick, 90_000 + order[0].0)]);
    let tick = harness.read().clock.tick.saturating_add(1);
    let duplicate = harness.send(&[input(order[0], tick, 90_000 + order[0].0)]);
    let tick = harness.read().clock.tick.saturating_add(1);
    let other = harness.send(&[input(unrelated, tick, 90_003)]);
    let tick = harness.read().clock.tick.saturating_add(1);
    let second = harness.send(&[input(order[1], tick, 90_000 + order[1].0)]);
    let checkpoint = harness.save().expect("surface checkpoint saves");
    let recall = |source: JunctionId| {
        let mut replay = Harness::restore(checkpoint.clone()).expect("surface checkpoint restores");
        let tick = replay.read().clock.tick.saturating_add(1);
        let physical = replay.read().junction(source).unwrap().physical_id;
        let run = replay.send(&[input(source, tick, physical)]);
        (
            run.outputs
                .iter()
                .any(|output| output.from_physical == 90_010),
            run.naturally_quiescent,
            candidate_trace(&run),
        )
    };
    let a = recall(surfaces[0]);
    let b = recall(surfaces[1]);
    let unrelated_recall = recall(unrelated);
    let consequence_links = harness
        .read()
        .links
        .into_iter()
        .filter(|link| link.last_consequence_tick.is_some())
        .map(|link| {
            serde_json::json!({
                "link": link.id.0,
                "from": link.from.0,
                "to": link.to.0,
                "strength": link.strength,
                "last_consequence_tick": link.last_consequence_tick,
            })
        })
        .collect();
    let return_origins = harness
        .read()
        .links
        .into_iter()
        .filter(|link| link.live && link.mode == TransmissionMode::Modulatory)
        .map(|link| link.return_origins)
        .collect();
    let tick = harness.read().clock.tick.saturating_add(1);
    let (_, replay) = replay_send(&mut harness, &[input(surfaces[0], tick, 90_001)]);
    harness.advance_to(tick.saturating_add(40));
    SurfaceCohort {
        recall: [a.0, b.0],
        unrelated: unrelated_recall.0,
        updates: [
            first.work.local_return_updates,
            duplicate.work.local_return_updates,
            other.work.local_return_updates,
            second.work.local_return_updates,
        ],
        returns_after_expiry: harness.read().return_path_count,
        consequence_links,
        return_origins,
        recall_reads: [a.2, b.2],
        replay,
        quiet: trained.naturally_quiescent
            && first.naturally_quiescent
            && duplicate.naturally_quiescent
            && other.naturally_quiescent
            && second.naturally_quiescent
            && a.1
            && b.1
            && unrelated_recall.1,
    }
}

fn clean_surface_solve() -> (bool, bool, bool, serde_json::Value) {
    let forward = surface_cohort(false);
    let reverse = surface_cohort(true);
    let valid = |trial: &SurfaceCohort| {
        trial.recall == [true, true]
            && !trial.unrelated
            && trial.updates[0] > 0
            && trial.updates[1] == 0
            && trial.updates[2] == 0
            && trial.updates[3] > 0
            && trial.returns_after_expiry == 0
    };
    (
        valid(&forward) && valid(&reverse),
        forward.replay && reverse.replay,
        forward.quiet && reverse.quiet,
        serde_json::json!({
            "forward": {"recall": forward.recall, "unrelated": forward.unrelated, "updates": forward.updates, "returns_after_expiry": forward.returns_after_expiry, "consequence_links": forward.consequence_links, "return_origins": forward.return_origins, "recall_reads": forward.recall_reads},
            "reverse": {"recall": reverse.recall, "unrelated": reverse.unrelated, "updates": reverse.updates, "returns_after_expiry": reverse.returns_after_expiry, "consequence_links": reverse.consequence_links, "return_origins": reverse.return_origins, "recall_reads": reverse.recall_reads},
        }),
    )
}

fn clean_surface_association() -> (bool, bool, bool, serde_json::Value) {
    let forward = surface_cohort(false);
    let reverse = surface_cohort(true);
    let associated = |trial: &SurfaceCohort| {
        trial.updates[0] > 0
            && trial.updates[1] == 0
            && trial.updates[2] == 0
            && trial.updates[3] > 0
            && trial.return_origins.iter().any(|origins| {
                origins.contains(&90_001) && origins.contains(&90_002) && !origins.contains(&90_003)
            })
    };
    (
        associated(&forward) && associated(&reverse),
        forward.replay && reverse.replay,
        forward.quiet && reverse.quiet,
        serde_json::json!({
            "forward_updates": forward.updates,
            "reverse_updates": reverse.updates,
            "forward_return_origins": forward.return_origins,
            "reverse_return_origins": reverse.return_origins,
            "reverse_recall_established": false,
        }),
    )
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

fn canonical(harness: &Harness) -> Vec<u8> {
    harness
        .save()
        .and_then(|checkpoint| checkpoint.canonical_bytes())
        .expect("fixture checkpoint is canonical")
}

fn same_run(left: &Run, right: &Run) -> bool {
    left.outputs == right.outputs
        && left.work == right.work
        && left.naturally_quiescent == right.naturally_quiescent
        && left.memory_bytes == right.memory_bytes
        && left.execution_cost == right.execution_cost
}

fn replay_send(harness: &mut Harness, inputs: &[Input]) -> (Run, bool) {
    let checkpoint = harness.save().expect("fixture checkpoint saves");
    let mut replay = Harness::restore(checkpoint).expect("fixture checkpoint restores");
    let observed = harness.send(inputs);
    let replayed = replay.send(inputs);
    let exact = same_run(&observed, &replayed) && canonical(harness) == canonical(&replay);
    (observed, exact)
}

fn current_candidate_reference() -> ProbeResult {
    use sensorimotor_physical_boundaries::Arm as Parent;
    let checks = Parent::ALL
        .into_iter()
        .map(sensorimotor_physical_boundaries::run)
        .collect::<Vec<_>>();
    let expected = [
        "survived",
        "falsified",
        "survived",
        "survived",
        "survived",
        "falsified",
        "falsified",
        "inconclusive",
        "inconclusive",
        "inconclusive",
        "inconclusive",
    ];
    finish(
        Arm::CurrentCandidateReference,
        checks
            .iter()
            .zip(expected)
            .all(|(result, expected)| result.outcome == expected),
        serde_json::json!({"classifications": checks.iter().map(|result| serde_json::json!({"arm": result.arm, "outcome": result.outcome})).collect::<Vec<_>>() }),
        "a frozen parent classification changed",
        checks.iter().all(|result| result.exact_replay),
        checks.iter().all(|result| result.naturally_quiescent),
    )
}

struct RivalWorld {
    harness: Harness,
    sources: [JunctionId; 2],
    motors: [JunctionId; 2],
    outcomes: [JunctionId; 2],
    physical: [u64; 2],
}

impl RivalWorld {
    fn new() -> Self {
        let mut builder = HarnessBuilder::with_capacity(8_192, 32_768, OUTWARD_REGION);
        builder.set_protocol(Protocol::SensorimotorCandidate);
        builder.set_physical_tracing(true);
        let sources = [
            junction(&mut builder, 10_000, -2, 0, 1),
            junction(&mut builder, 10_001, 2, 0, 1),
        ];
        let physical = [10_010, 10_011];
        let motors = [
            junction(&mut builder, physical[0], -1, 0, 2),
            junction(&mut builder, physical[1], 1, 0, 2),
        ];
        let sinks = [
            junction(&mut builder, 10_020, -1, OUTWARD_REGION, 1),
            junction(&mut builder, 10_021, 1, OUTWARD_REGION, 1),
        ];
        let outcomes = [
            junction(&mut builder, 10_030, 50, 0, 1),
            junction(&mut builder, 10_031, 60, 0, 1),
        ];
        let anchor = junction(&mut builder, 10_040, 1_000, 0, 99);
        for target in sources.into_iter().chain(outcomes) {
            link(&mut builder, anchor, target);
        }
        for index in 0..2 {
            link(&mut builder, motors[index], sinks[index]);
            builder.set_outcome_source_for_output(motors[index], outcomes[index]);
        }
        Self {
            harness: builder.build(),
            sources,
            motors,
            outcomes,
            physical,
        }
    }

    fn compete(&mut self) -> Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            input(self.sources[0], tick, 10_000),
            input(self.sources[1], tick, 10_001),
            input(self.motors[0], tick + 1, self.physical[0]),
            input(self.motors[1], tick + 1, self.physical[1]),
        ])
    }

    fn train_incumbent(&mut self, repetitions: usize) -> bool {
        let mut quiet = true;
        for _ in 0..repetitions {
            quiet &= self.compete().naturally_quiescent;
            let tick = self.harness.read().clock.tick.saturating_add(1);
            quiet &= self
                .harness
                .send(&[input(self.outcomes[0], tick, 10_030)])
                .naturally_quiescent;
        }
        quiet
    }
}

fn candidate_trace(run: &Run) -> Vec<serde_json::Value> {
    run.physical_trace
        .iter()
        .filter_map(|transition| match transition.event {
            PhysicalEvent::CandidateSelection {
                target,
                origin_scope,
                consequence_tick,
                admitted,
            } => Some(serde_json::json!({
                "target": target.0,
                "origin_scope": origin_scope,
                "consequence_tick": consequence_tick,
                "admitted": admitted,
            })),
            _ => None,
        })
        .collect()
}

fn marked_links(harness: &Harness, target: JunctionId) -> Vec<serde_json::Value> {
    harness
        .read()
        .links
        .into_iter()
        .filter(|link| link.live && link.to == target && link.participation > 0)
        .map(|link| {
            serde_json::json!({
                "link": link.id.0,
                "from": link.from.0,
                "strength": link.strength,
                "participation": link.participation,
                "last_consequence_tick": link.last_consequence_tick,
            })
        })
        .collect()
}

fn eligibility_trial(ratio: usize) -> (serde_json::Value, bool, bool) {
    let mut world = RivalWorld::new();
    let mut quiet = world.train_incumbent(ratio);
    let first = world.compete();
    let fresh = world.compete();
    quiet &= first.naturally_quiescent && fresh.naturally_quiescent;
    let fresh_output = fresh.outputs.first().map(|output| output.from_physical);
    let fresh_index = fresh_output.and_then(|physical| {
        world
            .physical
            .iter()
            .position(|candidate| *candidate == physical)
    });
    let returned = fresh_index.map(|index| {
        let tick = world.harness.read().clock.tick.saturating_add(1);
        world
            .harness
            .send(&[input(world.outcomes[index], tick, 10_030 + index as u64)])
    });
    if let Some(run) = &returned {
        quiet &= run.naturally_quiescent;
    }
    let marks = [
        marked_links(&world.harness, world.motors[0]),
        marked_links(&world.harness, world.motors[1]),
    ];
    let tick = world.harness.read().clock.tick.saturating_add(1);
    let inputs = [
        input(world.sources[0], tick, 10_000),
        input(world.sources[1], tick, 10_001),
        input(world.motors[0], tick + 1, world.physical[0]),
        input(world.motors[1], tick + 1, world.physical[1]),
    ];
    let (renewed, replay) = replay_send(&mut world.harness, &inputs);
    quiet &= renewed.naturally_quiescent;
    let renewed_output = renewed.outputs.first().map(|output| output.from_physical);
    (
        serde_json::json!({
            "ratio": ratio,
            "first_output": first.outputs.first().map(|output| output.from_physical),
            "fresh_output": fresh_output,
            "return_updates": returned.as_ref().map_or(0, |run| run.work.local_return_updates),
            "marked_links": marks,
            "renewed_output": renewed_output,
            "continued": renewed_output == fresh_output,
            "candidate_reads": candidate_trace(&renewed),
        }),
        replay,
        quiet,
    )
}

fn eligibility_write_read_localization() -> ProbeResult {
    let trials = [1, 2, 4, 8]
        .into_iter()
        .map(eligibility_trial)
        .collect::<Vec<_>>();
    let localized = trials.iter().all(|(value, _, _)| {
        value["return_updates"].as_u64().unwrap_or(0) > 0
            && value["marked_links"].as_array().is_some_and(|motors| {
                motors
                    .iter()
                    .any(|links| !links.as_array().unwrap().is_empty())
            })
            && !value["candidate_reads"].as_array().unwrap().is_empty()
    });
    finish(
        Arm::EligibilityWriteReadLocalization,
        localized,
        serde_json::json!({"trials": trials.iter().map(|trial| &trial.0).collect::<Vec<_>>() }),
        "the local return, consequence write, and next candidate read were not all observable",
        trials.iter().all(|trial| trial.1),
        trials.iter().all(|trial| trial.2),
    )
}

struct SurfaceObservation {
    path_after_action: bool,
    return_after_action: usize,
    updates: u64,
    recorded: usize,
    replay: bool,
    quiet: bool,
}

fn surface_opening(motor_position: i32) -> SurfaceObservation {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorCandidate);
    builder.set_physical_tracing(true);
    let action = junction(&mut builder, 60_000, 0, 0, 1);
    let surface = junction(&mut builder, 60_001, 0, 0, 1);
    let motor = junction(&mut builder, 60_010, motor_position, 0, 2);
    let sink = junction(&mut builder, 60_011, motor_position, OUTWARD_REGION, 1);
    let hub = junction(&mut builder, 60_012, 50, 0, 1);
    let anchor = junction(&mut builder, 60_013, 1_000, 0, 99);
    for target in [action, surface, hub] {
        link(&mut builder, anchor, target);
    }
    link(&mut builder, motor, sink);
    link(&mut builder, surface, hub);
    builder.set_outcome_source_for_output(motor, hub);
    let mut harness = builder.build();
    let action_run = harness.send(&[input(action, 1, 60_000), input(motor, 2, 60_010)]);
    let observation = harness.read();
    let path_after_action = observation
        .links
        .iter()
        .any(|link| link.live && link.to == motor && link.participation > 0);
    let return_after_action = observation.return_path_count;
    let tick = observation.clock.tick.saturating_add(1);
    let (surface_run, replay) = replay_send(&mut harness, &[input(surface, tick, 60_001)]);
    let recorded = surface_run
        .physical_trace
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::ConsequenceRecorded { .. }))
        .count();
    SurfaceObservation {
        path_after_action,
        return_after_action,
        updates: surface_run.work.local_return_updates,
        recorded,
        replay,
        quiet: action_run.naturally_quiescent && surface_run.naturally_quiescent,
    }
}

fn surface_return_opening_localization() -> ProbeResult {
    let invalid = surface_opening(0);
    let valid = surface_opening(1);
    finish(
        Arm::SurfaceReturnOpeningLocalization,
        !invalid.path_after_action
            && invalid.return_after_action == 0
            && invalid.updates == 0
            && valid.path_after_action
            && valid.return_after_action == 1
            && valid.updates > 0
            && valid.recorded > 0,
        serde_json::json!({
            "distance_zero": {"path": invalid.path_after_action, "returns": invalid.return_after_action, "updates": invalid.updates, "records": invalid.recorded},
            "distance_one": {"path": valid.path_after_action, "returns": valid.return_after_action, "updates": valid.updates, "records": valid.recorded},
        }),
        "path-forming separation did not restore return opening and delivery",
        invalid.replay && valid.replay,
        invalid.quiet && valid.quiet,
    )
}

struct CoalitionObservation {
    outputs: usize,
    marks: [Vec<serde_json::Value>; 2],
    reads: Vec<serde_json::Value>,
    replay: bool,
    quiet: bool,
}

fn coalition_observation() -> CoalitionObservation {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorCandidate);
    builder.set_physical_tracing(true);
    let root = junction(&mut builder, 50_000, -100, 0, 1);
    let sources = [
        junction(&mut builder, 50_001, 0, 0, 1),
        junction(&mut builder, 50_002, 10, 0, 1),
    ];
    let motors = [
        junction(&mut builder, 50_010, 1, 0, 2),
        junction(&mut builder, 50_011, 11, 0, 2),
    ];
    let sinks = [
        junction(&mut builder, 50_020, 1, OUTWARD_REGION, 1),
        junction(&mut builder, 50_021, 11, OUTWARD_REGION, 1),
    ];
    let outcomes = [
        junction(&mut builder, 50_030, 50, 0, 1),
        junction(&mut builder, 50_031, 60, 0, 1),
    ];
    let anchor = junction(&mut builder, 50_040, 1_000, 0, 99);
    for target in [root, sources[0], sources[1], outcomes[0], outcomes[1]] {
        link(&mut builder, anchor, target);
    }
    link(&mut builder, root, sources[0]);
    link(&mut builder, root, sources[1]);
    for index in 0..2 {
        link(&mut builder, motors[index], sinks[index]);
        builder.set_outcome_source_for_output(motors[index], outcomes[index]);
    }
    let mut harness = builder.build();
    let trained = harness.send(&[
        input(root, 1, 50_000),
        input(motors[0], 2, 50_010),
        input(motors[1], 2, 50_011),
    ]);
    let returned = harness.send(&[input(outcomes[0], 3, 50_030), input(outcomes[1], 3, 50_031)]);
    let marks = [
        marked_links(&harness, motors[0]),
        marked_links(&harness, motors[1]),
    ];
    let tick = harness.read().clock.tick.saturating_add(1);
    let (recalled, replay) = replay_send(&mut harness, &[input(root, tick, 50_000)]);
    CoalitionObservation {
        outputs: recalled.outputs.len(),
        marks,
        reads: candidate_trace(&recalled),
        replay,
        quiet: trained.naturally_quiescent
            && returned.naturally_quiescent
            && recalled.naturally_quiescent,
    }
}

fn coalition_lineage_localization() -> ProbeResult {
    let observed = coalition_observation();
    let executable = observed.marks.iter().all(|links| !links.is_empty());
    let marked = observed.marks.iter().all(|links| {
        links
            .iter()
            .any(|link| link["last_consequence_tick"].as_i64().is_some())
    });
    finish(
        Arm::CoalitionLineageLocalization,
        executable && marked && !observed.reads.is_empty(),
        serde_json::json!({"recalled_outputs": observed.outputs, "per_member_links": observed.marks, "candidate_reads": observed.reads}),
        "both executable members did not expose consequence lineage into selection",
        observed.replay,
        observed.quiet,
    )
}

fn participation_continuity_solves() -> ProbeResult {
    let recent = clean_recent_solve();
    let coalition = clean_coalition_solve();
    let surface = clean_surface_solve();
    finish(
        Arm::ParticipationContinuitySolves,
        recent.0 && coalition.0 && surface.0,
        serde_json::json!({"recent": recent.3, "coalition": coalition.3, "surface": surface.3}),
        "one or more localized participation-continuity predicates remain unsolved",
        recent.1 && coalition.1 && surface.1,
        recent.2 && coalition.2 && surface.2,
    )
}

fn surviving_boundaries_composition() -> ProbeResult {
    use sensorimotor_physical_boundaries::Arm as Parent;
    let probes = [
        sensorimotor_physical_boundaries::run(Parent::BoundedReturnEligibility),
        sensorimotor_physical_boundaries::run(Parent::ActiveFrontierScaling),
        sensorimotor_physical_boundaries::run(Parent::WaveAlternativeSparsity),
    ];
    finish(
        Arm::SurvivingBoundariesComposition,
        probes.iter().all(|probe| probe.outcome == "survived"),
        serde_json::json!({"components": probes.iter().map(|probe| serde_json::json!({"arm": probe.arm, "outcome": probe.outcome, "observations": probe.observations})).collect::<Vec<_>>() }),
        "a parent-surviving boundary changed under neutral fan-in",
        probes.iter().all(|probe| probe.exact_replay),
        probes.iter().all(|probe| probe.naturally_quiescent),
    )
}

fn embodied_prerequisites() -> (bool, bool, bool, serde_json::Value) {
    let recent = clean_recent_solve();
    let coalition = clean_coalition_solve();
    let surface = clean_surface_association();
    (
        recent.0 && coalition.0 && surface.0,
        recent.1 && coalition.1 && surface.1,
        recent.2 && coalition.2 && surface.2,
        serde_json::json!({
            "recent_continuation": recent.0,
            "coalition": coalition.0,
            "action_surface_association": surface.0,
            "surface_observations": surface.3,
        }),
    )
}

fn one_joint_recomposition() -> ProbeResult {
    let prerequisites = embodied_prerequisites();
    let boundaries = surviving_boundaries_composition();
    if !prerequisites.0 || boundaries.outcome != "survived" {
        return inconclusive(
            Arm::OneJointRecomposition,
            serde_json::json!({"continuity": prerequisites.3, "boundaries": boundaries.outcome}),
            "an actual one-joint prerequisite was falsified",
        );
    }
    let stage = sensorimotor_emergence::run_candidate_control_steps(
        "single_joint",
        1,
        sensorimotor_emergence::CandidateSurface::Proprioceptive,
        4,
    );
    if stage.status != sensorimotor_emergence::StageStatus::Passed {
        return inconclusive(
            Arm::OneJointRecomposition,
            serde_json::json!({"prerequisites": prerequisites.3, "four_step_preflight": stage}),
            "the full reflected one-joint run exceeded the development budget after topology growth",
        );
    }
    finish(
        Arm::OneJointRecomposition,
        stage.status == sensorimotor_emergence::StageStatus::Passed,
        serde_json::json!({"prerequisites": prerequisites.3, "stage": stage}),
        "the reflected one-joint stage did not close",
        prerequisites.1
            && stage.observations["exact_replay"]
                .as_bool()
                .unwrap_or(false),
        prerequisites.2
            && stage.observations["naturally_quiescent"]
                .as_bool()
                .unwrap_or(false),
    )
}

fn binocular_hand_recomposition() -> ProbeResult {
    let joint = one_joint_recomposition();
    if joint.outcome != "survived" {
        return inconclusive(
            Arm::BinocularHandRecomposition,
            serde_json::json!({"one_joint": joint.outcome}),
            "an actual binocular-hand prerequisite failed",
        );
    }
    let binocular = sensorimotor_emergence::run_candidate_control(
        "binocular_control",
        1,
        sensorimotor_emergence::CandidateSurface::Binocular,
    );
    let hand = sensorimotor_emergence::run_candidate_control(
        "digit_control",
        10,
        sensorimotor_emergence::CandidateSurface::Proprioceptive,
    );
    let passed = binocular.status == sensorimotor_emergence::StageStatus::Passed
        && hand.status == sensorimotor_emergence::StageStatus::Passed;
    finish(
        Arm::BinocularHandRecomposition,
        passed,
        serde_json::json!({"binocular": binocular, "hand": hand}),
        "binocular correction and hand grouping did not compose",
        binocular.observations["exact_replay"]
            .as_bool()
            .unwrap_or(false)
            && hand.observations["exact_replay"].as_bool().unwrap_or(false),
        binocular.observations["naturally_quiescent"]
            .as_bool()
            .unwrap_or(false)
            && hand.observations["naturally_quiescent"]
                .as_bool()
                .unwrap_or(false),
    )
}

fn vocal_auditory_recomposition() -> ProbeResult {
    let prerequisites = embodied_prerequisites();
    if !prerequisites.0 {
        return inconclusive(
            Arm::VocalAuditoryRecomposition,
            serde_json::json!({"continuity": prerequisites.3}),
            "an actual vocal-auditory prerequisite failed",
        );
    }
    let stage = sensorimotor_emergence::run_candidate_control_steps(
        "vocal_auditory_control",
        4,
        sensorimotor_emergence::CandidateSurface::VocalAuditory,
        4,
    );
    if stage.status != sensorimotor_emergence::StageStatus::Passed {
        return inconclusive(
            Arm::VocalAuditoryRecomposition,
            serde_json::json!({"prerequisites": prerequisites.3, "four_step_preflight": stage}),
            "the vocal-auditory preflight produced no admissible movement before the full-run cost gate",
        );
    }
    finish(
        Arm::VocalAuditoryRecomposition,
        stage.status == sensorimotor_emergence::StageStatus::Passed,
        serde_json::json!({"prerequisites": prerequisites.3, "stage": stage}),
        "the delayed nonlinguistic acoustic loop did not close",
        prerequisites.1
            && stage.observations["exact_replay"]
                .as_bool()
                .unwrap_or(false),
        prerequisites.2
            && stage.observations["naturally_quiescent"]
                .as_bool()
                .unwrap_or(false),
    )
}

fn full_body_recomposition() -> ProbeResult {
    let probes = [
        one_joint_recomposition(),
        binocular_hand_recomposition(),
        vocal_auditory_recomposition(),
        surviving_boundaries_composition(),
    ];
    if probes.iter().any(|probe| probe.outcome != "survived") {
        return inconclusive(
            Arm::FullBodyRecomposition,
            serde_json::json!({"prerequisites": probes.iter().map(|probe| serde_json::json!({"arm": probe.arm, "outcome": probe.outcome})).collect::<Vec<_>>() }),
            "one or more actual full-body prerequisites failed",
        );
    }
    let stage = sensorimotor_emergence::run_candidate_control(
        "multimodal_composition",
        7,
        sensorimotor_emergence::CandidateSurface::Composition,
    );
    finish(
        Arm::FullBodyRecomposition,
        stage.status == sensorimotor_emergence::StageStatus::Passed,
        serde_json::json!({"prerequisites": "survived", "stage": stage}),
        "the full-body composition interfered",
        stage.observations["exact_replay"]
            .as_bool()
            .unwrap_or(false),
        stage.observations["naturally_quiescent"]
            .as_bool()
            .unwrap_or(false),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! probe {
        ($name:ident, $arm:expr) => {
            #[test]
            fn $name() {
                let result = run($arm);
                assert!(matches!(
                    result.outcome.as_str(),
                    "survived" | "falsified" | "inconclusive"
                ));
                assert!(result.exact_replay);
                assert!(result.naturally_quiescent);
            }
        };
    }

    probe!(
        probe_current_candidate_reference,
        Arm::CurrentCandidateReference
    );
    probe!(
        probe_eligibility_write_read_localization,
        Arm::EligibilityWriteReadLocalization
    );
    probe!(
        probe_surface_return_opening_localization,
        Arm::SurfaceReturnOpeningLocalization
    );
    probe!(
        probe_coalition_lineage_localization,
        Arm::CoalitionLineageLocalization
    );
    probe!(
        probe_participation_continuity_solves,
        Arm::ParticipationContinuitySolves
    );
    probe!(
        probe_surviving_boundaries_composition,
        Arm::SurvivingBoundariesComposition
    );
    probe!(probe_one_joint_recomposition, Arm::OneJointRecomposition);
    probe!(
        probe_binocular_hand_recomposition,
        Arm::BinocularHandRecomposition
    );
    probe!(
        probe_vocal_auditory_recomposition,
        Arm::VocalAuditoryRecomposition
    );
    probe!(probe_full_body_recomposition, Arm::FullBodyRecomposition);
}
