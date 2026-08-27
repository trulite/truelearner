#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, Link, Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arm {
    CurrentPhysicsReference,
    RecentEligibilityOverStrength,
    BoundedReturnEligibility,
    ActiveFrontierScaling,
    WaveAlternativeSparsity,
    LearnedCoalitionPreservation,
    SurfaceReturnCohort,
    OneJointRecomposition,
    BinocularHandRecomposition,
    VocalAuditoryRecomposition,
    FullBodyRecomposition,
}

impl Arm {
    pub const ALL: [Self; 11] = [
        Self::CurrentPhysicsReference,
        Self::RecentEligibilityOverStrength,
        Self::BoundedReturnEligibility,
        Self::ActiveFrontierScaling,
        Self::WaveAlternativeSparsity,
        Self::LearnedCoalitionPreservation,
        Self::SurfaceReturnCohort,
        Self::OneJointRecomposition,
        Self::BinocularHandRecomposition,
        Self::VocalAuditoryRecomposition,
        Self::FullBodyRecomposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::CurrentPhysicsReference => "current-physics-reference",
            Self::RecentEligibilityOverStrength => "recent-eligibility-over-strength",
            Self::BoundedReturnEligibility => "bounded-return-eligibility",
            Self::ActiveFrontierScaling => "active-frontier-scaling",
            Self::WaveAlternativeSparsity => "wave-alternative-sparsity",
            Self::LearnedCoalitionPreservation => "learned-coalition-preservation",
            Self::SurfaceReturnCohort => "surface-return-cohort",
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
        Arm::CurrentPhysicsReference => current_physics_reference(),
        Arm::RecentEligibilityOverStrength => recent_eligibility_over_strength(),
        Arm::BoundedReturnEligibility => bounded_return_eligibility(),
        Arm::ActiveFrontierScaling => active_frontier_scaling(),
        Arm::WaveAlternativeSparsity => wave_alternative_sparsity(),
        Arm::LearnedCoalitionPreservation => learned_coalition_preservation(),
        Arm::SurfaceReturnCohort => surface_return_cohort(),
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
        schema: "sensorimotor-physical-boundaries/v1".to_string(),
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
        schema: "sensorimotor-physical-boundaries/v1".to_string(),
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
    builder.add_link(Link {
        from,
        to,
        delay: 0,
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

fn used_strength(harness: &Harness, motor: JunctionId) -> i64 {
    harness
        .read()
        .links
        .iter()
        .filter(|link| link.to == motor && link.live)
        .map(|link| link.strength)
        .max()
        .unwrap_or(0)
}

fn current_physics_reference() -> ProbeResult {
    use sensorimotor_composition_sweep::Arm as Prior;
    let checks = [
        (Prior::CausalReturnReference, "survived"),
        (Prior::OpportunityReference, "survived"),
        (Prior::ChoiceToMotion, "survived"),
        (Prior::EntrenchedContinuation, "falsified"),
        (Prior::CausalWindow, "falsified"),
        (Prior::FixedActiveScaling, "falsified"),
        (Prior::WaveScopedCompetition, "falsified"),
        (Prior::SurfaceOrderAlignment, "falsified"),
    ]
    .into_iter()
    .map(|(arm, expected)| (sensorimotor_composition_sweep::run(arm), expected))
    .collect::<Vec<_>>();
    finish(
        Arm::CurrentPhysicsReference,
        checks
            .iter()
            .all(|(result, expected)| result.outcome == *expected),
        serde_json::json!({"classifications": checks.iter().map(|(result, _)| serde_json::json!({"arm": result.arm, "outcome": result.outcome})).collect::<Vec<_>>() }),
        "a frozen predecessor classification changed",
        checks.iter().all(|(result, _)| result.exact_replay),
        checks.iter().all(|(result, _)| result.naturally_quiescent),
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

    fn train(&mut self, index: usize, repetitions: usize) -> Vec<Run> {
        let mut runs = Vec::new();
        for _ in 0..repetitions {
            let tick = self.harness.read().clock.tick.saturating_add(1);
            runs.push(self.harness.send(&[
                input(self.sources[index], tick, 10_000 + index as u64),
                input(self.motors[index], tick + 1, self.physical[index]),
            ]));
            let tick = self.harness.read().clock.tick.saturating_add(1);
            runs.push(self.harness.send(&[input(
                self.outcomes[index],
                tick,
                10_030 + index as u64,
            )]));
        }
        runs
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
}

fn recent_trial(ratio: usize) -> (bool, bool, i64, i64, bool, bool) {
    let mut world = RivalWorld::new();
    let training = world.train(0, ratio);
    let incumbent_before = used_strength(&world.harness, world.motors[0]);
    let first = world.compete();
    let fresh = world.compete();
    if fresh.outputs.first().map(|output| output.from_physical) == Some(world.physical[1]) {
        let tick = world.harness.read().clock.tick.saturating_add(1);
        world
            .harness
            .send(&[input(world.outcomes[1], tick, 10_031)]);
    }
    let inputs_tick = world.harness.read().clock.tick.saturating_add(1);
    let inputs = [
        input(world.sources[0], inputs_tick, 10_000),
        input(world.sources[1], inputs_tick, 10_001),
        input(world.motors[0], inputs_tick + 1, world.physical[0]),
        input(world.motors[1], inputs_tick + 1, world.physical[1]),
    ];
    let (renewed, replay) = replay_send(&mut world.harness, &inputs);
    let continued =
        renewed.outputs.first().map(|output| output.from_physical) == Some(world.physical[1]);
    let incumbent_after = used_strength(&world.harness, world.motors[0]);
    for _ in 0..6 {
        world.compete();
    }
    let released = world
        .compete()
        .outputs
        .first()
        .is_some_and(|output| output.from_physical != world.physical[1]);
    let quiescent = training
        .iter()
        .chain([&first, &fresh, &renewed])
        .all(|run| run.naturally_quiescent);
    (
        continued,
        released,
        incumbent_before,
        incumbent_after,
        replay,
        quiescent,
    )
}

fn recent_eligibility_over_strength() -> ProbeResult {
    let trials = [1, 2, 4, 8]
        .into_iter()
        .map(|ratio| (ratio, recent_trial(ratio)))
        .collect::<Vec<_>>();
    finish(
        Arm::RecentEligibilityOverStrength,
        trials.iter().all(|(_, value)| value.0 && value.1 && value.2 == value.3),
        serde_json::json!({"trials": trials.iter().map(|(ratio, value)| serde_json::json!({"ratio": ratio, "continued": value.0, "released": value.1, "incumbent_before": value.2, "incumbent_after": value.3})).collect::<Vec<_>>() }),
        "recent consequence did not mediate entrenched strength while preserving history and release",
        trials.iter().all(|(_, value)| value.4),
        trials.iter().all(|(_, value)| value.5),
    )
}

fn delayed_credit(delay: i64) -> (bool, u64, usize, bool, bool) {
    let mut builder = HarnessBuilder::with_capacity(24, 48, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorCandidate);
    let source = junction(&mut builder, 20_000, 0, 0, 1);
    let motor = junction(&mut builder, 20_001, 1, 0, 2);
    let sink = junction(&mut builder, 20_002, 1, OUTWARD_REGION, 1);
    let outcome = junction(&mut builder, 20_003, 50, 0, 1);
    let anchor = junction(&mut builder, 20_004, 1_000, 0, 99);
    link(&mut builder, anchor, source);
    link(&mut builder, anchor, outcome);
    link(&mut builder, motor, sink);
    builder.set_outcome_source_for_output(motor, outcome);
    let mut harness = builder.build();
    let used = harness.send(&[input(source, 1, 20_000), input(motor, 2, 20_001)]);
    let before = used_strength(&harness, motor);
    let tick = harness.read().clock.tick.saturating_add(delay);
    harness.advance_to(tick);
    harness = Harness::restore(harness.save().expect("advanced checkpoint saves"))
        .expect("advanced checkpoint restores");
    let (returned, replay) = replay_send(&mut harness, &[input(outcome, tick, 20_003)]);
    let after = used_strength(&harness, motor);
    let expiry = tick.saturating_add(40);
    harness.advance_to(expiry);
    (
        after > before,
        returned.work.local_return_updates,
        harness.read().return_path_count,
        replay,
        used.naturally_quiescent && returned.naturally_quiescent,
    )
}

fn bounded_return_eligibility() -> ProbeResult {
    let trials = [1, 20, 200]
        .into_iter()
        .map(|delay| (delay, delayed_credit(delay)))
        .collect::<Vec<_>>();
    finish(
        Arm::BoundedReturnEligibility,
        trials[0].1 .0
            && trials[1].1 .0
            && !trials[2].1 .0
            && trials.iter().all(|(_, value)| value.2 == 0),
        serde_json::json!({"trials": trials.iter().map(|(delay, value)| serde_json::json!({"delay": delay, "credited": value.0, "return_updates": value.1, "returns_after_expiry": value.2})).collect::<Vec<_>>() }),
        "finite return eligibility did not preserve valid delay and reject stale delay",
        trials.iter().all(|(_, value)| value.3),
        trials.iter().all(|(_, value)| value.4),
    )
}

#[derive(Clone, Copy)]
struct Scale {
    outputs: usize,
    work: u64,
    scans: u64,
    active: u64,
    bytes: usize,
    replay: bool,
    quiescent: bool,
}

fn fixed_active(size: usize) -> Scale {
    let capacity = u32::try_from(size.saturating_mul(3).saturating_add(32)).unwrap_or(u32::MAX);
    let mut builder = HarnessBuilder::with_capacity(capacity, capacity * 2, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorCandidate);
    let source = junction(&mut builder, 30_000, 0, 0, 1);
    let motor = junction(&mut builder, 30_001, 1, 0, 2);
    let sink = junction(&mut builder, 30_002, 1, OUTWARD_REGION, 1);
    let anchor = junction(&mut builder, 30_003, 10_000, 0, 99);
    link(&mut builder, anchor, source);
    link(&mut builder, motor, sink);
    for index in 0..size {
        let position = 100 + i32::try_from(index).unwrap_or(i32::MAX).saturating_mul(4);
        let dormant = junction(&mut builder, 100_000 + index as u64, position, 0, 1);
        let dormant_sink = junction(
            &mut builder,
            200_000 + index as u64,
            position,
            OUTWARD_REGION,
            1,
        );
        link(&mut builder, dormant, dormant_sink);
    }
    let mut harness = builder.build();
    let inputs = [input(source, 1, 30_000), input(motor, 2, 30_001)];
    let (run, replay) = replay_send(&mut harness, &inputs);
    Scale {
        outputs: run.outputs.len(),
        work: run.work.total(),
        scans: run.execution_cost.scans,
        active: run.execution_cost.active_arena_max,
        bytes: harness.read().resident_bytes,
        replay,
        quiescent: run.naturally_quiescent,
    }
}

fn active_frontier_scaling() -> ProbeResult {
    let small = fixed_active(4);
    let medium = fixed_active(64);
    let large = fixed_active(1_024);
    finish(
        Arm::ActiveFrontierScaling,
        small.outputs == 1
            && medium.outputs == 1
            && large.outputs == 1
            && large.work <= small.work.saturating_mul(4)
            && large.scans <= small.scans.saturating_mul(4),
        serde_json::json!({
            "small": {"dormant": 4, "outputs": small.outputs, "work": small.work, "scans": small.scans, "active": small.active, "bytes": small.bytes},
            "medium": {"dormant": 64, "outputs": medium.outputs, "work": medium.work, "scans": medium.scans, "active": medium.active, "bytes": medium.bytes},
            "large": {"dormant": 1024, "outputs": large.outputs, "work": large.work, "scans": large.scans, "active": large.active, "bytes": large.bytes},
        }),
        "work or scans still followed dormant stable topology",
        small.replay && medium.replay && large.replay,
        small.quiescent && medium.quiescent && large.quiescent,
    )
}

fn fanout(size: usize, shared: bool) -> (usize, bool, bool) {
    let capacity = u32::try_from(size * 4 + 16).unwrap_or(u32::MAX);
    let mut builder = HarnessBuilder::with_capacity(capacity, capacity * 2, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorCandidate);
    let shared_root = junction(&mut builder, 40_000, -100, 0, 1);
    let mut roots = Vec::new();
    for index in 0..size {
        let position = i32::try_from(index).unwrap_or(i32::MAX) * 10;
        let root = junction(&mut builder, 40_100 + index as u64, position - 3, 0, 1);
        let motor = junction(&mut builder, 41_100 + index as u64, position, 0, 1);
        let sink = junction(
            &mut builder,
            42_100 + index as u64,
            position,
            OUTWARD_REGION,
            1,
        );
        link(&mut builder, motor, sink);
        link(&mut builder, if shared { shared_root } else { root }, motor);
        roots.push(root);
    }
    let mut harness = builder.build();
    let inputs = if shared {
        vec![input(shared_root, 1, 40_000)]
    } else {
        roots
            .iter()
            .map(|root| {
                let physical = harness.read().junction(*root).unwrap().physical_id;
                input(*root, 1, physical)
            })
            .collect()
    };
    let (run, replay) = replay_send(&mut harness, &inputs);
    (run.outputs.len(), replay, run.naturally_quiescent)
}

fn wave_alternative_sparsity() -> ProbeResult {
    let shared = fanout(8, true);
    let independent = fanout(8, false);
    finish(
        Arm::WaveAlternativeSparsity,
        shared.0 == 1 && independent.0 == 8,
        serde_json::json!({"shared_origin_outputs": shared.0, "independent_origin_outputs": independent.0}),
        "physical-origin grouping did not bound alternatives while preserving independent origins",
        shared.1 && independent.1,
        shared.2 && independent.2,
    )
}

fn coalition_recall() -> (usize, bool, bool) {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorCandidate);
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
    let tick = harness.read().clock.tick.saturating_add(1);
    let (recalled, replay) = replay_send(&mut harness, &[input(root, tick, 50_000)]);
    (
        recalled.outputs.len(),
        replay,
        trained.naturally_quiescent && returned.naturally_quiescent && recalled.naturally_quiescent,
    )
}

fn learned_coalition_preservation() -> ProbeResult {
    let learned = coalition_recall();
    let untrained = fanout(8, true);
    let either = fanout(1, true);
    finish(
        Arm::LearnedCoalitionPreservation,
        learned.0 == 2 && untrained.0 == 1 && either.0 == 1,
        serde_json::json!({"learned_pair_outputs": learned.0, "untrained_shared_outputs": untrained.0, "either_alone_outputs": either.0}),
        "learned consequential cohort did not survive alternative sparsity controls",
        learned.1 && untrained.1 && either.1,
        learned.2 && untrained.2 && either.2,
    )
}

struct SurfaceTrial {
    recall: [bool; 2],
    unrelated_recall: bool,
    updates: [u64; 4],
    returns_after_expiry: usize,
    replay: bool,
    quiescent: bool,
}

fn surface_trial(reverse: bool) -> SurfaceTrial {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
    builder.set_protocol(Protocol::SensorimotorCandidate);
    let action = junction(&mut builder, 60_000, 0, 0, 1);
    let surfaces = [
        junction(&mut builder, 60_001, -1, 0, 1),
        junction(&mut builder, 60_002, 1, 0, 1),
    ];
    let unrelated = junction(&mut builder, 60_003, 20, 0, 1);
    let motor = junction(&mut builder, 60_010, 0, 0, 2);
    let sink = junction(&mut builder, 60_011, 0, OUTWARD_REGION, 1);
    let hub = junction(&mut builder, 60_012, 50, 0, 1);
    let anchor = junction(&mut builder, 60_013, 1_000, 0, 99);
    for target in [action, surfaces[0], surfaces[1], unrelated, hub] {
        link(&mut builder, anchor, target);
    }
    link(&mut builder, motor, sink);
    for surface in surfaces.into_iter().chain([unrelated]) {
        link(&mut builder, surface, hub);
    }
    builder.set_outcome_source_for_output(motor, hub);
    let mut harness = builder.build();
    let participated = harness.send(&[input(action, 1, 60_000), input(motor, 2, 60_010)]);
    let order = if reverse {
        [surfaces[1], surfaces[0]]
    } else {
        surfaces
    };
    let tick = harness.read().clock.tick.saturating_add(1);
    let first = harness.send(&[input(order[0], tick, 60_000 + order[0].0)]);
    let tick = harness.read().clock.tick.saturating_add(1);
    let duplicate = harness.send(&[input(order[0], tick, 60_000 + order[0].0)]);
    let tick = harness.read().clock.tick.saturating_add(1);
    let unrelated_run = harness.send(&[input(unrelated, tick, 60_003)]);
    let tick = harness.read().clock.tick.saturating_add(1);
    let second = harness.send(&[input(order[1], tick, 60_000 + order[1].0)]);
    let checkpoint = harness.save().expect("surface checkpoint saves");
    let recall = |source: JunctionId| {
        let mut body = Harness::restore(checkpoint.clone()).expect("surface checkpoint restores");
        let tick = body.read().clock.tick.saturating_add(1);
        let physical = body.read().junction(source).unwrap().physical_id;
        let run = body.send(&[input(source, tick, physical)]);
        (run.outputs.len() == 1, run.naturally_quiescent)
    };
    let a = recall(surfaces[0]);
    let b = recall(surfaces[1]);
    let other = recall(unrelated);
    let tick = harness.read().clock.tick.saturating_add(1);
    let (replayed, replay) = replay_send(&mut harness, &[input(surfaces[0], tick, 60_001)]);
    harness.advance_to(tick.saturating_add(40));
    SurfaceTrial {
        recall: [a.0, b.0],
        unrelated_recall: other.0,
        updates: [
            first.work.local_return_updates,
            duplicate.work.local_return_updates,
            unrelated_run.work.local_return_updates,
            second.work.local_return_updates,
        ],
        returns_after_expiry: harness.read().return_path_count,
        replay,
        quiescent: participated.naturally_quiescent
            && first.naturally_quiescent
            && duplicate.naturally_quiescent
            && unrelated_run.naturally_quiescent
            && second.naturally_quiescent
            && a.1
            && b.1
            && other.1
            && replayed.naturally_quiescent,
    }
}

fn surface_return_cohort() -> ProbeResult {
    let forward = surface_trial(false);
    let reverse = surface_trial(true);
    let valid = |trial: &SurfaceTrial| {
        trial.recall == [true, true]
            && !trial.unrelated_recall
            && trial.updates[0] > 0
            && trial.updates[1] == 0
            && trial.updates[2] == 0
            && trial.updates[3] > 0
            && trial.returns_after_expiry == 0
    };
    finish(
        Arm::SurfaceReturnCohort,
        valid(&forward) && valid(&reverse),
        serde_json::json!({
            "forward": {"recall": forward.recall, "unrelated": forward.unrelated_recall, "updates": forward.updates, "returns_after_expiry": forward.returns_after_expiry},
            "reverse": {"recall": reverse.recall, "unrelated": reverse.unrelated_recall, "updates": reverse.updates, "returns_after_expiry": reverse.returns_after_expiry},
        }),
        "the return did not retain two distinct local surfaces once each in either order",
        forward.replay && reverse.replay,
        forward.quiescent && reverse.quiescent,
    )
}

fn one_joint_recomposition() -> ProbeResult {
    let recent = recent_eligibility_over_strength();
    let bounded = bounded_return_eligibility();
    let wave = wave_alternative_sparsity();
    if [
        recent.outcome.as_str(),
        bounded.outcome.as_str(),
        wave.outcome.as_str(),
    ]
    .contains(&"falsified")
    {
        return inconclusive(
            Arm::OneJointRecomposition,
            serde_json::json!({"recent": recent.outcome, "bounded_return": bounded.outcome, "wave": wave.outcome}),
            "an actual one-joint prerequisite was falsified",
        );
    }
    finish(
        Arm::OneJointRecomposition,
        false,
        serde_json::json!({"prerequisites": "survived", "adapter": "candidate joint required"}),
        "surviving isolated laws did not yet close the reflected joint",
        true,
        true,
    )
}

fn binocular_hand_recomposition() -> ProbeResult {
    let coalition = learned_coalition_preservation();
    let surfaces = surface_return_cohort();
    if coalition.outcome != "survived" || surfaces.outcome != "survived" {
        return inconclusive(
            Arm::BinocularHandRecomposition,
            serde_json::json!({"coalition": coalition.outcome, "surfaces": surfaces.outcome}),
            "an actual binocular-hand prerequisite was falsified",
        );
    }
    finish(
        Arm::BinocularHandRecomposition,
        false,
        serde_json::json!({"prerequisites": "survived"}),
        "surviving component laws did not compose into paired correction and hand grouping",
        true,
        true,
    )
}

fn vocal_auditory_recomposition() -> ProbeResult {
    let bounded = bounded_return_eligibility();
    let surfaces = surface_return_cohort();
    if bounded.outcome != "survived" || surfaces.outcome != "survived" {
        return inconclusive(
            Arm::VocalAuditoryRecomposition,
            serde_json::json!({"bounded_return": bounded.outcome, "surfaces": surfaces.outcome}),
            "an actual vocal-auditory prerequisite was falsified",
        );
    }
    let sequence = sensorimotor_dependency_sweep::run(
        sensorimotor_dependency_sweep::Arm::ComposedOutputSequence,
    );
    finish(
        Arm::VocalAuditoryRecomposition,
        sequence.outcome == "survived",
        serde_json::json!({"bounded_return": bounded.outcome, "surfaces": surfaces.outcome, "sequence": sequence.outcome, "linguistic_semantics": false}),
        "the delayed nonlinguistic loop did not recall",
        sequence.exact_replay,
        sequence.naturally_quiescent,
    )
}

fn full_body_recomposition() -> ProbeResult {
    let prerequisites = [
        one_joint_recomposition(),
        active_frontier_scaling(),
        binocular_hand_recomposition(),
        vocal_auditory_recomposition(),
    ];
    if prerequisites
        .iter()
        .any(|probe| probe.outcome != "survived")
    {
        return inconclusive(
            Arm::FullBodyRecomposition,
            serde_json::json!({"prerequisites": prerequisites.iter().map(|probe| serde_json::json!({"arm": probe.arm, "outcome": probe.outcome})).collect::<Vec<_>>() }),
            "one or more actual full-body prerequisites did not survive",
        );
    }
    let sparse =
        sensorimotor_dependency_sweep::run(sensorimotor_dependency_sweep::Arm::ConsolidatedReuse);
    finish(
        Arm::FullBodyRecomposition,
        sparse.outcome == "survived",
        serde_json::json!({"prerequisites": "survived", "dormant_route_control": sparse.outcome}),
        "the active learned subset interfered inside dormant body topology",
        sparse.exact_replay,
        sparse.naturally_quiescent,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_integrity(arm: Arm) {
        let result = run(arm);
        assert_eq!(result.arm, arm.id());
        assert!(matches!(
            result.outcome.as_str(),
            "survived" | "falsified" | "inconclusive"
        ));
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
    }

    macro_rules! probe {
        ($name:ident, $arm:expr) => {
            #[test]
            fn $name() {
                assert_integrity($arm);
            }
        };
    }

    probe!(
        probe_current_physics_reference,
        Arm::CurrentPhysicsReference
    );
    probe!(
        probe_recent_eligibility_over_strength,
        Arm::RecentEligibilityOverStrength
    );
    probe!(
        probe_bounded_return_eligibility,
        Arm::BoundedReturnEligibility
    );
    probe!(probe_active_frontier_scaling, Arm::ActiveFrontierScaling);
    probe!(
        probe_wave_alternative_sparsity,
        Arm::WaveAlternativeSparsity
    );
    probe!(
        probe_learned_coalition_preservation,
        Arm::LearnedCoalitionPreservation
    );
    probe!(probe_surface_return_cohort, Arm::SurfaceReturnCohort);
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
