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
    CausalReturnReference,
    OpportunityReference,
    EntrenchedContinuation,
    ContextShiftContinuation,
    CausalWindow,
    ChoiceToMotion,
    OneJointComposition,
    WaveScopedCompetition,
    ConsequenceStabilizedSelection,
    FixedActiveScaling,
    DistantCoalitionRecall,
    LocalCoalitionDiscrimination,
    MultisurfaceAssociation,
    SurfaceOrderAlignment,
    BinocularCoalition,
    DigitHandCoalitions,
    EyeHandComposition,
    VocalAuditoryLoop,
    FullBodyBoundedComposition,
}

impl Arm {
    pub const ALL: [Self; 19] = [
        Self::CausalReturnReference,
        Self::OpportunityReference,
        Self::EntrenchedContinuation,
        Self::ContextShiftContinuation,
        Self::CausalWindow,
        Self::ChoiceToMotion,
        Self::OneJointComposition,
        Self::WaveScopedCompetition,
        Self::ConsequenceStabilizedSelection,
        Self::FixedActiveScaling,
        Self::DistantCoalitionRecall,
        Self::LocalCoalitionDiscrimination,
        Self::MultisurfaceAssociation,
        Self::SurfaceOrderAlignment,
        Self::BinocularCoalition,
        Self::DigitHandCoalitions,
        Self::EyeHandComposition,
        Self::VocalAuditoryLoop,
        Self::FullBodyBoundedComposition,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::CausalReturnReference => "causal-return-reference",
            Self::OpportunityReference => "opportunity-reference",
            Self::EntrenchedContinuation => "entrenched-continuation",
            Self::ContextShiftContinuation => "context-shift-continuation",
            Self::CausalWindow => "causal-window",
            Self::ChoiceToMotion => "choice-to-motion",
            Self::OneJointComposition => "one-joint-composition",
            Self::WaveScopedCompetition => "wave-scoped-competition",
            Self::ConsequenceStabilizedSelection => "consequence-stabilized-selection",
            Self::FixedActiveScaling => "fixed-active-scaling",
            Self::DistantCoalitionRecall => "distant-coalition-recall",
            Self::LocalCoalitionDiscrimination => "local-coalition-discrimination",
            Self::MultisurfaceAssociation => "multisurface-association",
            Self::SurfaceOrderAlignment => "surface-order-alignment",
            Self::BinocularCoalition => "binocular-coalition",
            Self::DigitHandCoalitions => "digit-hand-coalitions",
            Self::EyeHandComposition => "eye-hand-composition",
            Self::VocalAuditoryLoop => "vocal-auditory-loop",
            Self::FullBodyBoundedComposition => "full-body-bounded-composition",
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
        Arm::CausalReturnReference => causal_return_reference(),
        Arm::OpportunityReference => opportunity_reference(),
        Arm::EntrenchedContinuation => entrenched_continuation(),
        Arm::ContextShiftContinuation => context_shift_continuation(),
        Arm::CausalWindow => causal_window(),
        Arm::ChoiceToMotion => choice_to_motion(),
        Arm::OneJointComposition => one_joint_composition(),
        Arm::WaveScopedCompetition => wave_scoped_competition(),
        Arm::ConsequenceStabilizedSelection => consequence_stabilized_selection(),
        Arm::FixedActiveScaling => fixed_active_scaling(),
        Arm::DistantCoalitionRecall => distant_coalition_recall(),
        Arm::LocalCoalitionDiscrimination => local_coalition_discrimination(),
        Arm::MultisurfaceAssociation => multisurface_association(),
        Arm::SurfaceOrderAlignment => surface_order_alignment(),
        Arm::BinocularCoalition => binocular_coalition(),
        Arm::DigitHandCoalitions => digit_hand_coalitions(),
        Arm::EyeHandComposition => eye_hand_composition(),
        Arm::VocalAuditoryLoop => vocal_auditory_loop(),
        Arm::FullBodyBoundedComposition => full_body_bounded_composition(),
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
        schema: "sensorimotor-composition-sweep/v1".to_string(),
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
        schema: "sensorimotor-composition-sweep/v1".to_string(),
        arm: arm.id().to_string(),
        outcome: "inconclusive".to_string(),
        observations,
        falsifier: Some(reason.to_string()),
        exact_replay: true,
        naturally_quiescent: true,
    }
}

fn input(target: JunctionId, tick: i64, origin: u64) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target,
        impulse: 1,
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
        .into_iter()
        .filter(|state| state.live && state.to == motor && state.participation > 0)
        .map(|state| state.strength)
        .max()
        .unwrap_or(0)
}

fn predecessor(
    arm: sensorimotor_dependency_sweep::Arm,
) -> sensorimotor_dependency_sweep::ProbeResult {
    sensorimotor_dependency_sweep::run(arm)
}

fn causal_return_reference() -> ProbeResult {
    let arm = Arm::CausalReturnReference;
    let causal = predecessor(sensorimotor_dependency_sweep::Arm::CausalLocalCredit);
    let delayed = predecessor(sensorimotor_dependency_sweep::Arm::DelayedReturnLifetime);
    finish(
        arm,
        causal.outcome == "survived" && delayed.outcome == "survived",
        serde_json::json!({"causal": causal.outcome, "delayed": delayed.outcome}),
        "a frozen causal-return reference changed",
        causal.exact_replay && delayed.exact_replay,
        causal.naturally_quiescent && delayed.naturally_quiescent,
    )
}

fn opportunity_reference() -> ProbeResult {
    let arm = Arm::OpportunityReference;
    let probes = [
        predecessor(sensorimotor_dependency_sweep::Arm::LocalAlternativeExposure),
        predecessor(sensorimotor_dependency_sweep::Arm::DisplacedReturnClosure),
        predecessor(sensorimotor_dependency_sweep::Arm::NoConsequenceRelease),
    ];
    finish(
        arm,
        probes.iter().all(|probe| probe.outcome == "survived"),
        serde_json::json!({"outcomes": probes.iter().map(|probe| probe.outcome.as_str()).collect::<Vec<_>>()}),
        "a frozen opportunity reference changed",
        probes.iter().all(|probe| probe.exact_replay),
        probes.iter().all(|probe| probe.naturally_quiescent),
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
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_protocol(Protocol::UnansweredReturnReplacement);
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
        for repetition in 0..repetitions {
            let tick = self.harness.read().clock.tick.saturating_add(1);
            runs.push(self.harness.send(&[
                input(self.sources[index], tick, 20_000 + repetition as u64),
                input(self.motors[index], tick + 1, 21_000 + repetition as u64),
            ]));
            let tick = self.harness.read().clock.tick.saturating_add(1);
            runs.push(self.harness.send(&[input(
                self.outcomes[index],
                tick,
                22_000 + repetition as u64,
            )]));
        }
        runs
    }

    fn competition_inputs(&self) -> Vec<Input> {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        vec![
            input(self.sources[0], tick, 23_000 + tick as u64),
            input(self.sources[1], tick, 24_000 + tick as u64),
            input(self.motors[0], tick + 1, 25_000 + tick as u64),
            input(self.motors[1], tick + 1, 26_000 + tick as u64),
        ]
    }
}

fn entrenched_trial(ratio: usize) -> (bool, u64, u64, bool, bool) {
    let mut world = RivalWorld::new();
    let training = world.train(0, ratio);
    let incumbent_strength = used_strength(&world.harness, world.motors[0]);
    let incumbent = world.harness.send(&world.competition_inputs());
    let fresh = world.harness.send(&world.competition_inputs());
    let fresh_output = fresh.outputs.first().map(|output| output.from_physical);
    if fresh_output == Some(world.physical[1]) {
        let tick = world.harness.read().clock.tick.saturating_add(1);
        world
            .harness
            .send(&[input(world.outcomes[1], tick, 27_000 + tick as u64)]);
    }
    let inputs = world.competition_inputs();
    let (renewed, exact_replay) = replay_send(&mut world.harness, &inputs);
    let continued =
        renewed.outputs.first().map(|output| output.from_physical) == Some(world.physical[1]);
    let naturally_quiescent = training
        .iter()
        .chain([&incumbent, &fresh, &renewed])
        .all(|run| run.naturally_quiescent);
    (
        continued,
        u64::try_from(incumbent_strength).unwrap_or(0),
        u64::try_from(used_strength(&world.harness, world.motors[1])).unwrap_or(0),
        exact_replay,
        naturally_quiescent,
    )
}

fn entrenched_continuation() -> ProbeResult {
    let arm = Arm::EntrenchedContinuation;
    let trials = [1, 2, 4, 8]
        .into_iter()
        .map(|ratio| (ratio, entrenched_trial(ratio)))
        .collect::<Vec<_>>();
    finish(
        arm,
        trials.iter().all(|(_, trial)| trial.0),
        serde_json::json!({
            "trials": trials.iter().map(|(ratio, trial)| serde_json::json!({
                "incumbent_training": ratio,
                "continued_fresh": trial.0,
                "incumbent_strength": trial.1,
                "fresh_strength": trial.2,
            })).collect::<Vec<_>>()
        }),
        "one fresh consequence did not renew against an entrenched incumbent",
        trials.iter().all(|(_, trial)| trial.3),
        trials.iter().all(|(_, trial)| trial.4),
    )
}

fn context_shift_trial() -> (bool, bool, bool, bool) {
    let mut builder = HarnessBuilder::with_capacity(48, 96, OUTWARD_REGION);
    let sources = [
        junction(&mut builder, 30_000, 0, 0, 1),
        junction(&mut builder, 30_001, 1, 0, 1),
        junction(&mut builder, 30_002, 20, 0, 1),
    ];
    let motor = junction(&mut builder, 30_010, 1, 0, 2);
    let sink = junction(&mut builder, 30_011, 1, OUTWARD_REGION, 1);
    let outcome = junction(&mut builder, 30_012, 50, 0, 1);
    let anchor = junction(&mut builder, 30_013, 1_000, 0, 99);
    for target in sources.into_iter().chain([outcome]) {
        link(&mut builder, anchor, target);
    }
    link(&mut builder, motor, sink);
    builder.set_outcome_source_for_output(motor, outcome);
    let mut harness = builder.build();
    let tick = 1;
    let trained = harness.send(&[
        input(sources[0], tick, 31_000),
        input(motor, tick + 1, 31_001),
    ]);
    let tick = harness.read().clock.tick.saturating_add(1);
    let returned = harness.send(&[input(outcome, tick, 31_002)]);
    let checkpoint = harness.save().expect("context checkpoint saves");
    let recall = |source: JunctionId| {
        let mut body = Harness::restore(checkpoint.clone()).expect("context checkpoint restores");
        let tick = body.read().clock.tick.saturating_add(1);
        body.send(&[input(source, tick, 31_100 + source.0)])
    };
    let exact = recall(sources[0]).outputs.len() == 1;
    let adjacent = recall(sources[1]).outputs.len() == 1;
    let distant = recall(sources[2]).outputs.len() == 1;
    (
        exact,
        adjacent,
        distant,
        trained.naturally_quiescent && returned.naturally_quiescent,
    )
}

fn context_shift_continuation() -> ProbeResult {
    let arm = Arm::ContextShiftContinuation;
    let (exact, adjacent, distant, quiescent) = context_shift_trial();
    finish(
        arm,
        exact && adjacent && !distant,
        serde_json::json!({"exact_source": exact, "adjacent_source": adjacent, "distant_source": distant}),
        "consequential eligibility did not transfer only to the adjacent context",
        true,
        quiescent,
    )
}

fn delayed_credit(delay: i64) -> (bool, u64, bool) {
    let mut builder = HarnessBuilder::with_capacity(24, 48, OUTWARD_REGION);
    let source = junction(&mut builder, 40_000, 0, 0, 1);
    let motor = junction(&mut builder, 40_001, 1, 0, 2);
    let sink = junction(&mut builder, 40_002, 1, OUTWARD_REGION, 1);
    let outcome = junction(&mut builder, 40_003, 50, 0, 1);
    let anchor = junction(&mut builder, 40_004, 1_000, 0, 99);
    link(&mut builder, anchor, source);
    link(&mut builder, anchor, outcome);
    link(&mut builder, motor, sink);
    builder.set_outcome_source_for_output(motor, outcome);
    let mut harness = builder.build();
    let used = harness.send(&[input(source, 1, 41_000), input(motor, 2, 41_001)]);
    let before = used_strength(&harness, motor);
    let tick = harness.read().clock.tick.saturating_add(delay);
    harness.advance_to(tick);
    harness = Harness::restore(harness.save().expect("advanced checkpoint saves"))
        .expect("advanced checkpoint restores");
    let (returned, replay) = replay_send(&mut harness, &[input(outcome, tick, 41_002)]);
    (
        used_strength(&harness, motor) > before,
        returned.work.local_return_updates,
        replay && used.naturally_quiescent && returned.naturally_quiescent,
    )
}

fn causal_window() -> ProbeResult {
    let arm = Arm::CausalWindow;
    let trials = [1, 20, 200]
        .into_iter()
        .map(|delay| (delay, delayed_credit(delay)))
        .collect::<Vec<_>>();
    let survived = trials[0].1 .0 && trials[1].1 .0 && !trials[2].1 .0;
    finish(
        arm,
        survived,
        serde_json::json!({"trials": trials.iter().map(|(delay, value)| serde_json::json!({"delay": delay, "credited": value.0, "return_updates": value.1})).collect::<Vec<_>>()}),
        "the return did not distinguish bounded causal delay from stale delay",
        trials.iter().all(|(_, value)| value.2),
        true,
    )
}

fn choice_trial(reflected: bool) -> (Option<u64>, i8, bool, bool) {
    let positions = if reflected { [1, -1] } else { [-1, 1] };
    let mut builder = HarnessBuilder::with_capacity(32, 64, OUTWARD_REGION);
    let source = junction(&mut builder, 50_000, 0, 0, 1);
    let physical = [50_010, 50_011];
    let motors = [
        junction(&mut builder, physical[0], positions[0], 0, 2),
        junction(&mut builder, physical[1], positions[1], 0, 2),
    ];
    let sinks = [
        junction(&mut builder, 50_020, positions[0], OUTWARD_REGION, 1),
        junction(&mut builder, 50_021, positions[1], OUTWARD_REGION, 1),
    ];
    let anchor = junction(&mut builder, 50_030, 1_000, 0, 99);
    link(&mut builder, anchor, source);
    for index in 0..2 {
        link(&mut builder, motors[index], sinks[index]);
    }
    let mut harness = builder.build();
    let inputs = [
        input(source, 1, 51_000),
        input(motors[0], 2, 51_001),
        input(motors[1], 2, 51_002),
    ];
    let (run, replay) = replay_send(&mut harness, &inputs);
    let selected = run.outputs.first().map(|output| output.from_physical);
    let delta = selected.map_or(0, |output| if output == physical[0] { -1 } else { 1 });
    (selected, delta, replay, run.naturally_quiescent)
}

fn choice_to_motion() -> ProbeResult {
    let arm = Arm::ChoiceToMotion;
    let ordinary = choice_trial(false);
    let reflected = choice_trial(true);
    finish(
        arm,
        ordinary.0.is_some()
            && reflected.0.is_some()
            && ordinary.1 != 0
            && reflected.1 != 0
            && ordinary.0 != reflected.0,
        serde_json::json!({"ordinary_output": ordinary.0, "ordinary_delta": ordinary.1, "reflected_output": reflected.0, "reflected_delta": reflected.1}),
        "selected local output did not survive reflected physical integration",
        ordinary.2 && reflected.2,
        ordinary.3 && reflected.3,
    )
}

fn one_joint_composition() -> ProbeResult {
    let arm = Arm::OneJointComposition;
    let result = predecessor(sensorimotor_dependency_sweep::Arm::OneJointComposition);
    finish(
        arm,
        result.outcome == "survived",
        result.observations,
        "the frozen one-joint composition still failed both-limit recovery",
        result.exact_replay,
        result.naturally_quiescent,
    )
}

#[derive(Clone, Copy)]
struct Fanout {
    outputs: usize,
    work: u64,
    scans: u64,
    bytes: usize,
    replay: bool,
    quiescent: bool,
}

fn fanout(size: usize, shared: bool) -> Fanout {
    let capacity = u32::try_from(size.saturating_mul(4).saturating_add(16)).unwrap_or(u32::MAX);
    let mut builder =
        HarnessBuilder::with_capacity(capacity, capacity.saturating_mul(2), OUTWARD_REGION);
    let shared_root = junction(&mut builder, 60_000, -100, 0, 1);
    let mut roots = Vec::new();
    let mut motors = Vec::new();
    for index in 0..size {
        let position = i32::try_from(index).unwrap_or(i32::MAX).saturating_mul(10);
        let root = junction(&mut builder, 60_100 + index as u64, position - 3, 0, 1);
        let motor = junction(&mut builder, 61_100 + index as u64, position, 0, 1);
        let sink = junction(
            &mut builder,
            62_100 + index as u64,
            position,
            OUTWARD_REGION,
            1,
        );
        link(&mut builder, motor, sink);
        link(&mut builder, if shared { shared_root } else { root }, motor);
        roots.push(root);
        motors.push(motor);
    }
    let mut harness = builder.build();
    let inputs = if shared {
        vec![input(shared_root, 1, 63_000)]
    } else {
        roots
            .iter()
            .enumerate()
            .map(|(index, root)| input(*root, 1, 64_000 + index as u64))
            .collect()
    };
    let (run, replay) = replay_send(&mut harness, &inputs);
    Fanout {
        outputs: run.outputs.len(),
        work: run.work.total(),
        scans: run.execution_cost.scans,
        bytes: harness.read().resident_bytes,
        replay,
        quiescent: run.naturally_quiescent,
    }
}

fn wave_scoped_competition() -> ProbeResult {
    let arm = Arm::WaveScopedCompetition;
    let shared = fanout(8, true);
    let separate = fanout(8, false);
    finish(
        arm,
        shared.outputs <= 1 && separate.outputs == 8,
        serde_json::json!({"shared_wave_outputs": shared.outputs, "independent_wave_outputs": separate.outputs}),
        "one causal wave did not select bounded distant structure while preserving independent waves",
        shared.replay && separate.replay,
        shared.quiescent && separate.quiescent,
    )
}

fn consequence_stabilized_selection() -> ProbeResult {
    let wave = wave_scoped_competition();
    let entrenched = entrenched_continuation();
    if wave.outcome != "survived" || entrenched.outcome != "survived" {
        return inconclusive(
            Arm::ConsequenceStabilizedSelection,
            serde_json::json!({"wave_scoped_competition": wave.outcome, "entrenched_continuation": entrenched.outcome}),
            "an actual selection prerequisite was falsified",
        );
    }
    let continuation =
        predecessor(sensorimotor_dependency_sweep::Arm::ConsequenceSupportedContinuation);
    let release = predecessor(sensorimotor_dependency_sweep::Arm::NoConsequenceRelease);
    finish(
        Arm::ConsequenceStabilizedSelection,
        continuation.outcome == "survived" && release.outcome == "survived",
        serde_json::json!({"wave_scoped_competition": wave.outcome, "entrenched_continuation": entrenched.outcome, "bounded_recurrence": continuation.outcome, "no_consequence_release": release.outcome}),
        "sparse consequential selection did not both recur and release",
        continuation.exact_replay && release.exact_replay,
        continuation.naturally_quiescent && release.naturally_quiescent,
    )
}

fn fixed_active(size: usize) -> Fanout {
    let capacity = u32::try_from(size.saturating_mul(3).saturating_add(32)).unwrap_or(u32::MAX);
    let mut builder =
        HarnessBuilder::with_capacity(capacity, capacity.saturating_mul(2), OUTWARD_REGION);
    let source = junction(&mut builder, 70_000, 0, 0, 1);
    let motor = junction(&mut builder, 70_001, 1, 0, 2);
    let sink = junction(&mut builder, 70_002, 1, OUTWARD_REGION, 1);
    let anchor = junction(&mut builder, 70_003, 10_000, 0, 99);
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
    let inputs = [input(source, 1, 73_000), input(motor, 2, 73_001)];
    let (run, replay) = replay_send(&mut harness, &inputs);
    Fanout {
        outputs: run.outputs.len(),
        work: run.work.total(),
        scans: run.execution_cost.scans,
        bytes: harness.read().resident_bytes,
        replay,
        quiescent: run.naturally_quiescent,
    }
}

fn fixed_active_scaling() -> ProbeResult {
    let arm = Arm::FixedActiveScaling;
    let small = fixed_active(4);
    let medium = fixed_active(64);
    let large = fixed_active(1_024);
    let scans_bounded = large.scans <= small.scans.saturating_mul(4);
    let work_bounded = large.work <= small.work.saturating_mul(4);
    finish(
        arm,
        small.outputs == 1
            && medium.outputs == 1
            && large.outputs == 1
            && scans_bounded
            && work_bounded,
        serde_json::json!({
            "small": {"dormant": 4, "outputs": small.outputs, "work": small.work, "scans": small.scans, "bytes": small.bytes},
            "medium": {"dormant": 64, "outputs": medium.outputs, "work": medium.work, "scans": medium.scans, "bytes": medium.bytes},
            "large": {"dormant": 1024, "outputs": large.outputs, "work": large.work, "scans": large.scans, "bytes": large.bytes},
        }),
        "fixed active work or scans grew with dormant surface",
        small.replay && medium.replay && large.replay,
        small.quiescent && medium.quiescent && large.quiescent,
    )
}

struct CoalitionTrial {
    paired_outputs: usize,
    monocular_outputs: usize,
    replay: bool,
    quiescent: bool,
}

fn coalition_trial() -> CoalitionTrial {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
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
    let training = harness.send(&[
        input(root, 1, 81_000),
        input(motors[0], 2, 81_001),
        input(motors[1], 2, 81_002),
    ]);
    let returned = harness.send(&[input(outcomes[0], 3, 81_003), input(outcomes[1], 3, 81_004)]);
    let checkpoint = harness.save().expect("coalition checkpoint saves");
    let mut paired = Harness::restore(checkpoint.clone()).expect("coalition checkpoint restores");
    let mut monocular = Harness::restore(checkpoint).expect("coalition checkpoint restores");
    let paired_tick = paired.read().clock.tick.saturating_add(1);
    let (paired_run, replay) = replay_send(&mut paired, &[input(root, paired_tick, 81_100)]);
    let mono_tick = monocular.read().clock.tick.saturating_add(1);
    let mono_run = monocular.send(&[input(sources[0], mono_tick, 81_101)]);
    CoalitionTrial {
        paired_outputs: paired_run.outputs.len(),
        monocular_outputs: mono_run.outputs.len(),
        replay,
        quiescent: training.naturally_quiescent
            && returned.naturally_quiescent
            && paired_run.naturally_quiescent
            && mono_run.naturally_quiescent,
    }
}

fn distant_coalition_recall() -> ProbeResult {
    let arm = Arm::DistantCoalitionRecall;
    let trial = coalition_trial();
    finish(
        arm,
        trial.paired_outputs == 2,
        serde_json::json!({"paired_recall_outputs": trial.paired_outputs, "unrelated_outputs": trial.paired_outputs.saturating_sub(2)}),
        "the learned distant coalition did not recall both outputs",
        trial.replay,
        trial.quiescent,
    )
}

fn local_pair_outputs(count: usize) -> (usize, bool, bool) {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
    let source = junction(&mut builder, 90_000, 0, 0, 1);
    let anchor = junction(&mut builder, 90_001, 1_000, 0, 99);
    link(&mut builder, anchor, source);
    let mut motors = Vec::new();
    for index in 0..count {
        let position = i32::try_from(index).unwrap_or(i32::MAX);
        let motor = junction(&mut builder, 90_100 + index as u64, position, 0, 2);
        let sink = junction(
            &mut builder,
            91_100 + index as u64,
            position,
            OUTWARD_REGION,
            1,
        );
        link(&mut builder, motor, sink);
        motors.push(motor);
    }
    let mut harness = builder.build();
    let mut inputs = vec![input(source, 1, 92_000)];
    inputs.extend(
        motors
            .iter()
            .enumerate()
            .map(|(index, motor)| input(*motor, 2, 92_100 + index as u64)),
    );
    let (run, replay) = replay_send(&mut harness, &inputs);
    (run.outputs.len(), replay, run.naturally_quiescent)
}

fn local_coalition_discrimination() -> ProbeResult {
    let arm = Arm::LocalCoalitionDiscrimination;
    let joint = local_pair_outputs(2);
    let either = local_pair_outputs(1);
    finish(
        arm,
        joint.0 == 2 && either.0 == 1,
        serde_json::json!({"joint_effect_outputs": joint.0, "either_alone_outputs": either.0}),
        "local competition suppressed a necessary neighboring coalition",
        joint.1 && either.1,
        joint.2 && either.2,
    )
}

struct SurfaceTrial {
    recall: [bool; 2],
    unrelated: bool,
    return_count: usize,
    replay: bool,
    quiescent: bool,
}

fn surface_trial(reverse: bool) -> SurfaceTrial {
    let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
    let action = junction(&mut builder, 100_000, 0, 0, 1);
    let surfaces = [
        junction(&mut builder, 100_001, -1, 0, 1),
        junction(&mut builder, 100_002, 1, 0, 1),
    ];
    let unrelated = junction(&mut builder, 100_003, 20, 0, 1);
    let motor = junction(&mut builder, 100_010, 0, 0, 2);
    let sink = junction(&mut builder, 100_011, 0, OUTWARD_REGION, 1);
    let hub = junction(&mut builder, 100_012, 50, 0, 1);
    let anchor = junction(&mut builder, 100_013, 1_000, 0, 99);
    for target in [action, surfaces[0], surfaces[1], unrelated, hub] {
        link(&mut builder, anchor, target);
    }
    link(&mut builder, motor, sink);
    for surface in surfaces.into_iter().chain([unrelated]) {
        link(&mut builder, surface, hub);
    }
    builder.set_outcome_source_for_output(motor, hub);
    let mut harness = builder.build();
    let order = if reverse {
        [surfaces[1], surfaces[0]]
    } else {
        surfaces
    };
    let training = harness.send(&[
        input(action, 1, 101_000),
        input(motor, 2, 101_001),
        input(order[0], 3, 101_002),
        input(order[1], 4, 101_003),
    ]);
    let checkpoint = harness.save().expect("surface checkpoint saves");
    let recall = |source: JunctionId| {
        let mut body = Harness::restore(checkpoint.clone()).expect("surface checkpoint restores");
        let tick = body.read().clock.tick.saturating_add(1);
        let run = body.send(&[input(source, tick, 102_000 + source.0)]);
        (run.outputs.len() == 1, run.naturally_quiescent)
    };
    let first = recall(surfaces[0]);
    let second = recall(surfaces[1]);
    let other = recall(unrelated);
    let tick = harness.read().clock.tick.saturating_add(1);
    let (_, replay) = replay_send(&mut harness, &[input(surfaces[0], tick, 103_000)]);
    SurfaceTrial {
        recall: [first.0, second.0],
        unrelated: other.0,
        return_count: harness.read().return_path_count,
        replay,
        quiescent: training.naturally_quiescent && first.1 && second.1 && other.1,
    }
}

fn multisurface_association() -> ProbeResult {
    let arm = Arm::MultisurfaceAssociation;
    let trial = surface_trial(false);
    finish(
        arm,
        trial.recall == [true, true] && !trial.unrelated && trial.return_count == 0,
        serde_json::json!({"surface_recall": trial.recall, "unrelated_recall": trial.unrelated, "remaining_returns": trial.return_count}),
        "both delayed surfaces did not form useful bounded association",
        trial.replay,
        trial.quiescent,
    )
}

fn surface_order_alignment() -> ProbeResult {
    let arm = Arm::SurfaceOrderAlignment;
    let forward = surface_trial(false);
    let reverse = surface_trial(true);
    finish(
        arm,
        forward.recall == [true, true]
            && reverse.recall == [true, true]
            && !forward.unrelated
            && !reverse.unrelated,
        serde_json::json!({"forward_recall": forward.recall, "reverse_recall": reverse.recall, "forward_unrelated": forward.unrelated, "reverse_unrelated": reverse.unrelated}),
        "surface association depended on arrival order or included an unrelated surface",
        forward.replay && reverse.replay,
        forward.quiescent && reverse.quiescent,
    )
}

fn binocular_coalition() -> ProbeResult {
    let arm = Arm::BinocularCoalition;
    let trial = coalition_trial();
    let paired_delta = i32::try_from(trial.paired_outputs).unwrap_or(i32::MAX);
    let monocular_delta = i32::try_from(trial.monocular_outputs).unwrap_or(i32::MAX);
    finish(
        arm,
        paired_delta == 2 && monocular_delta < paired_delta,
        serde_json::json!({"paired_disparity_delta": paired_delta, "monocular_disparity_delta": monocular_delta}),
        "paired sensing did not produce a distinct bilateral correction",
        trial.replay,
        trial.quiescent,
    )
}

fn digit_hand_coalitions() -> ProbeResult {
    let arm = Arm::DigitHandCoalitions;
    let single = local_pair_outputs(1);
    let hand = local_pair_outputs(5);
    finish(
        arm,
        single.0 == 1 && hand.0 == 5,
        serde_json::json!({"single_digit_outputs": single.0, "five_digit_coalition_outputs": hand.0}),
        "neighboring digits could not recombine into a five-output coalition",
        single.1 && hand.1,
        single.2 && hand.2,
    )
}

fn eye_hand_composition() -> ProbeResult {
    let eyes = binocular_coalition();
    let hands = digit_hand_coalitions();
    if eyes.outcome != "survived" || hands.outcome != "survived" {
        return inconclusive(
            Arm::EyeHandComposition,
            serde_json::json!({"binocular": eyes.outcome, "hands": hands.outcome}),
            "an actual eye-hand component prerequisite was falsified",
        );
    }
    let sequence = predecessor(sensorimotor_dependency_sweep::Arm::ComposedOutputSequence);
    finish(
        Arm::EyeHandComposition,
        sequence.outcome == "survived",
        serde_json::json!({"binocular": eyes.outcome, "hands": hands.outcome, "sequence": sequence.outcome}),
        "surviving paired sensing and hand coalition did not compose",
        sequence.exact_replay,
        sequence.naturally_quiescent,
    )
}

fn vocal_auditory_loop() -> ProbeResult {
    let arm = Arm::VocalAuditoryLoop;
    let causal = causal_window();
    let surfaces = surface_order_alignment();
    let sequence = predecessor(sensorimotor_dependency_sweep::Arm::ComposedOutputSequence);
    if causal.outcome != "survived" || surfaces.outcome != "survived" {
        return inconclusive(
            arm,
            serde_json::json!({"causal_window": causal.outcome, "surface_alignment": surfaces.outcome, "generic_sequence": sequence.outcome}),
            "an actual delayed acoustic prerequisite was falsified",
        );
    }
    finish(
        arm,
        sequence.outcome == "survived",
        serde_json::json!({"surface_alignment": surfaces.outcome, "generic_sequence": sequence.outcome, "linguistic_semantics": false}),
        "the delayed nonlinguistic acoustic loop did not recall",
        sequence.exact_replay,
        sequence.naturally_quiescent,
    )
}

fn full_body_bounded_composition() -> ProbeResult {
    let prerequisites = [
        one_joint_composition(),
        fixed_active_scaling(),
        binocular_coalition(),
        digit_hand_coalitions(),
        vocal_auditory_loop(),
    ];
    if prerequisites
        .iter()
        .any(|probe| probe.outcome != "survived")
    {
        return inconclusive(
            Arm::FullBodyBoundedComposition,
            serde_json::json!({"prerequisites": prerequisites.iter().map(|probe| serde_json::json!({"arm": probe.arm, "outcome": probe.outcome})).collect::<Vec<_>>()}),
            "one or more actual full-body prerequisites did not survive",
        );
    }
    let sequence = predecessor(sensorimotor_dependency_sweep::Arm::ComposedOutputSequence);
    let sparse_recall = predecessor(sensorimotor_dependency_sweep::Arm::ConsolidatedReuse);
    finish(
        Arm::FullBodyBoundedComposition,
        sequence.outcome == "survived" && sparse_recall.outcome == "survived",
        serde_json::json!({"prerequisites": "all_survived", "active_sequence": sequence.outcome, "dormant_route_control": sparse_recall.outcome}),
        "full-body composition interfered despite surviving prerequisites",
        prerequisites.iter().all(|probe| probe.exact_replay)
            && sequence.exact_replay
            && sparse_recall.exact_replay,
        prerequisites.iter().all(|probe| probe.naturally_quiescent)
            && sequence.naturally_quiescent
            && sparse_recall.naturally_quiescent,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_integrity(arm: Arm) -> ProbeResult {
        let result = run(arm);
        assert_eq!(result.arm, arm.id());
        assert!(matches!(
            result.outcome.as_str(),
            "survived" | "falsified" | "inconclusive"
        ));
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
        result
    }

    macro_rules! probe_test {
        ($name:ident, $arm:expr) => {
            #[test]
            fn $name() {
                assert_integrity($arm);
            }
        };
    }

    #[test]
    fn probe_causal_return_reference() {
        assert_eq!(
            assert_integrity(Arm::CausalReturnReference).outcome,
            "survived"
        );
    }

    #[test]
    fn probe_opportunity_reference() {
        assert_eq!(
            assert_integrity(Arm::OpportunityReference).outcome,
            "survived"
        );
    }

    probe_test!(probe_entrenched_continuation, Arm::EntrenchedContinuation);
    probe_test!(
        probe_context_shift_continuation,
        Arm::ContextShiftContinuation
    );
    probe_test!(probe_causal_window, Arm::CausalWindow);
    probe_test!(probe_choice_to_motion, Arm::ChoiceToMotion);
    probe_test!(probe_one_joint_composition, Arm::OneJointComposition);
    probe_test!(probe_wave_scoped_competition, Arm::WaveScopedCompetition);
    probe_test!(
        probe_consequence_stabilized_selection,
        Arm::ConsequenceStabilizedSelection
    );
    probe_test!(probe_fixed_active_scaling, Arm::FixedActiveScaling);
    probe_test!(probe_distant_coalition_recall, Arm::DistantCoalitionRecall);
    probe_test!(
        probe_local_coalition_discrimination,
        Arm::LocalCoalitionDiscrimination
    );
    probe_test!(probe_multisurface_association, Arm::MultisurfaceAssociation);
    probe_test!(probe_surface_order_alignment, Arm::SurfaceOrderAlignment);
    probe_test!(probe_binocular_coalition, Arm::BinocularCoalition);
    probe_test!(probe_digit_hand_coalitions, Arm::DigitHandCoalitions);
    probe_test!(probe_eye_hand_composition, Arm::EyeHandComposition);
    probe_test!(probe_vocal_auditory_loop, Arm::VocalAuditoryLoop);
    probe_test!(
        probe_full_body_bounded_composition,
        Arm::FullBodyBoundedComposition
    );
}
