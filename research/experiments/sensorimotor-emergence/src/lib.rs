#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent, Protocol,
    TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const AXIS_SPACING: i32 = 8;
const LOWER: i16 = -4;
const UPPER: i16 = 4;
const STEPS: usize = 40;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arm {
    GlobalReturn,
    CausalLocalReturn,
    ShuffledLocalReturn,
    CausalLocalReference,
    LocalReturnDeferral,
    LocalReturnReplacement,
    ShuffledLocalReference,
}

impl Arm {
    pub const ALL: [Self; 7] = [
        Self::GlobalReturn,
        Self::CausalLocalReturn,
        Self::ShuffledLocalReturn,
        Self::CausalLocalReference,
        Self::LocalReturnDeferral,
        Self::LocalReturnReplacement,
        Self::ShuffledLocalReference,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::GlobalReturn => "global-return-reference",
            Self::CausalLocalReturn => "causal-local-return",
            Self::ShuffledLocalReturn => "shuffled-local-return",
            Self::CausalLocalReference => "causal-local-reference",
            Self::LocalReturnDeferral => "local-return-deferral",
            Self::LocalReturnReplacement => "local-return-replacement",
            Self::ShuffledLocalReference => "shuffled-local-reference",
        }
    }

    const fn wiring(self) -> OutcomeWiring {
        match self {
            Self::GlobalReturn => OutcomeWiring::Global,
            Self::ShuffledLocalReturn | Self::ShuffledLocalReference => OutcomeWiring::Shuffled,
            Self::CausalLocalReturn
            | Self::CausalLocalReference
            | Self::LocalReturnDeferral
            | Self::LocalReturnReplacement => OutcomeWiring::Local,
        }
    }

    const fn protocol(self) -> Protocol {
        match self {
            Self::LocalReturnDeferral => Protocol::UnansweredReturnDeferral,
            Self::LocalReturnReplacement => Protocol::UnansweredReturnReplacement,
            _ => Protocol::Physical,
        }
    }

    const fn expected_credited_path(self) -> Option<usize> {
        match self.wiring() {
            OutcomeWiring::Global => None,
            OutcomeWiring::Local => Some(0),
            OutcomeWiring::Shuffled => Some(1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OutcomeWiring {
    Global,
    Local,
    Shuffled,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageStatus {
    Passed,
    Failed,
    NotRun,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageResult {
    pub stage: String,
    pub status: StageStatus,
    pub observations: serde_json::Value,
    pub falsifier: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub schema: String,
    pub arm: String,
    pub outcome: String,
    pub falsifier: Option<String>,
    pub stages: Vec<StageResult>,
    pub exact_replay: bool,
    pub naturally_quiescent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Surface {
    Proprioceptive,
    Binocular,
    VocalAuditory,
    Composition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CandidateSurface {
    Proprioceptive,
    Binocular,
    VocalAuditory,
    Composition,
}

impl From<CandidateSurface> for Surface {
    fn from(surface: CandidateSurface) -> Self {
        match surface {
            CandidateSurface::Proprioceptive => Self::Proprioceptive,
            CandidateSurface::Binocular => Self::Binocular,
            CandidateSurface::VocalAuditory => Self::VocalAuditory,
            CandidateSurface::Composition => Self::Composition,
        }
    }
}

pub fn run_candidate_control(name: &str, axes: usize, surface: CandidateSurface) -> StageResult {
    run_candidate_control_steps(name, axes, surface, STEPS)
}

pub fn run_candidate_control_steps(
    name: &str,
    axes: usize,
    surface: CandidateSurface,
    steps: usize,
) -> StageResult {
    control_stage_with_protocol(
        name,
        axes,
        surface.into(),
        Arm::LocalReturnReplacement,
        Protocol::SensorimotorSynthesis,
        steps,
    )
}

pub fn run(arm: Arm, full: bool) -> ExperimentResult {
    let isolation = isolation_stage(arm);
    let mut stages = vec![isolation.clone()];
    let mut falsifier = isolation.falsifier.clone();
    let isolation_expected = match arm.expected_credited_path() {
        None => isolation.observations["credited_paths"] == 2,
        Some(path) => {
            isolation.status == StageStatus::Passed
                && isolation.observations["credited_path"] == path
        }
    };
    if !isolation_expected {
        falsifier.get_or_insert_with(|| "the preregistered isolation trace differed".to_string());
    }

    if matches!(arm, Arm::LocalReturnDeferral | Arm::LocalReturnReplacement) && isolation_expected {
        let opportunity = opportunity_stage(arm);
        if opportunity.status == StageStatus::Failed {
            falsifier = opportunity.falsifier.clone();
        }
        stages.push(opportunity);
    }

    let runs_joint = matches!(
        arm,
        Arm::CausalLocalReturn
            | Arm::CausalLocalReference
            | Arm::LocalReturnDeferral
            | Arm::LocalReturnReplacement
    );
    if full && runs_joint && isolation_expected && falsifier.is_none() {
        let single = control_stage("single_joint", 1, Surface::Proprioceptive, arm);
        let advance = single.status == StageStatus::Passed;
        if !advance {
            falsifier = single.falsifier.clone();
        }
        stages.push(single);
        let later = [
            ("repeated_axes", 4, Surface::Proprioceptive),
            ("digit_control", 10, Surface::Proprioceptive),
            ("binocular_control", 1, Surface::Binocular),
            ("vocal_auditory_control", 4, Surface::VocalAuditory),
            ("multimodal_composition", 7, Surface::Composition),
        ];
        if advance && arm == Arm::LocalReturnReplacement {
            let mut open = true;
            for (name, axes, surface) in later {
                if open {
                    let result = control_stage(name, axes, surface, arm);
                    open = result.status == StageStatus::Passed;
                    if !open && falsifier.is_none() {
                        falsifier = result.falsifier.clone();
                    }
                    stages.push(result);
                } else {
                    stages.push(not_run(name));
                }
            }
        } else {
            stages.extend(later.into_iter().map(|(name, _, _)| not_run(name)));
        }
    }

    let scientific_outcome = match arm {
        Arm::LocalReturnReplacement if full && falsifier.is_some() => "falsified",
        Arm::LocalReturnReplacement if isolation_expected => "survived",
        Arm::LocalReturnDeferral if full && falsifier.is_some() => "falsified",
        Arm::LocalReturnDeferral if isolation_expected => "survived",
        Arm::CausalLocalReturn if full && falsifier.is_some() => "falsified",
        Arm::CausalLocalReturn if isolation_expected => "survived",
        Arm::GlobalReturn
        | Arm::ShuffledLocalReturn
        | Arm::CausalLocalReference
        | Arm::ShuffledLocalReference
            if isolation_expected =>
        {
            "control-confirmed"
        }
        _ => "inconclusive",
    };
    ExperimentResult {
        schema: "sensorimotor-emergence/v1".to_string(),
        arm: arm.id().to_string(),
        outcome: scientific_outcome.to_string(),
        falsifier,
        exact_replay: stages
            .iter()
            .all(|stage| stage.observations["exact_replay"].as_bool().unwrap_or(true)),
        naturally_quiescent: stages.iter().all(|stage| {
            stage.observations["naturally_quiescent"]
                .as_bool()
                .unwrap_or(true)
        }),
        stages,
    }
}

fn not_run(stage: &str) -> StageResult {
    StageResult {
        stage: stage.to_string(),
        status: StageStatus::NotRun,
        observations: serde_json::json!({}),
        falsifier: None,
    }
}

fn isolation_stage(arm: Arm) -> StageResult {
    let mut world = IsolationWorld::new(arm, false);
    let participated = world.participate();
    let before = world.used_strengths();
    let checkpoint = world.harness.save().expect("fixture checkpoint saves");
    let returned = world.return_first_consequence();
    let after = world.used_strengths();
    let credited = (0..2)
        .filter(|index| after[*index] > before[*index])
        .collect::<Vec<_>>();

    let mut replay = Harness::restore(checkpoint).expect("fixture checkpoint restores");
    let tick = replay.read().clock.tick.saturating_add(1);
    let replayed = replay.send(&[physical_input(world.outcomes[0], tick, 80_000)]);
    let exact_replay = returned.outputs == replayed.outputs
        && returned.work == replayed.work
        && world
            .harness
            .save()
            .and_then(|checkpoint| checkpoint.canonical_bytes())
            == replay
                .save()
                .and_then(|checkpoint| checkpoint.canonical_bytes());

    let expected = match arm.expected_credited_path() {
        None => credited == [0, 1],
        Some(path) => credited == [path],
    };
    StageResult {
        stage: "causal_isolation".to_string(),
        status: if expected {
            StageStatus::Passed
        } else {
            StageStatus::Failed
        },
        observations: serde_json::json!({
            "outputs": participated.outputs.len(),
            "credited_paths": credited.len(),
            "credited_path": credited.first().copied(),
            "local_return_updates": returned.work.local_return_updates,
            "exact_replay": exact_replay,
            "naturally_quiescent": participated.naturally_quiescent && returned.naturally_quiescent,
        }),
        falsifier: (!expected)
            .then(|| "the physical return credited an unexpected path".to_string()),
    }
}

fn opportunity_stage(arm: Arm) -> StageResult {
    let mut world = OpportunityWorld::new(arm);
    let first = world.stimulate();
    let first_output = first.outputs[0].from_physical;
    let initial_return = world.return_for(first_output);
    let reused = world.stimulate();
    let reused_output = reused.outputs[0].from_physical;
    let before = world.used_strength(reused_output);
    let stimulus = world.stimulus();
    let checkpoint = world.harness.save().expect("opportunity checkpoint saves");
    let alternative = world.harness.send(&stimulus);
    let alternative_output = alternative.outputs[0].from_physical;
    let return_count = world.harness.read().return_path_count;
    let superseded = alternative
        .physical_trace
        .iter()
        .filter(|transition| matches!(transition.event, PhysicalEvent::ReturnSuperseded { .. }))
        .count();
    let returned = world.return_for(alternative_output);
    let displaced_unchanged = world.used_strength(reused_output) == before;

    let mut replay = Harness::restore(checkpoint).expect("opportunity checkpoint restores");
    let replayed = replay.send(&stimulus);
    let exact_replay = alternative.outputs == replayed.outputs
        && alternative.work == replayed.work
        && alternative.naturally_quiescent == replayed.naturally_quiescent;
    let expected = match arm {
        Arm::LocalReturnDeferral => return_count == 2 && superseded == 0,
        Arm::LocalReturnReplacement => return_count == 1 && superseded == 1,
        _ => false,
    } && first.outputs.len() == 1
        && initial_return.work.local_return_updates == 2
        && reused_output == first_output
        && alternative.outputs.len() == 1
        && alternative_output != reused_output
        && returned.work.local_return_updates == 2
        && displaced_unchanged
        && exact_replay;
    StageResult {
        stage: "local_opportunity".to_string(),
        status: if expected {
            StageStatus::Passed
        } else {
            StageStatus::Failed
        },
        observations: serde_json::json!({
            "reused_output": reused_output,
            "alternative_output": alternative_output,
            "return_paths_before_alternative_outcome": return_count,
            "superseded_returns": superseded,
            "alternative_return_updates": returned.work.local_return_updates,
            "displaced_strength_unchanged": displaced_unchanged,
            "exact_replay": exact_replay,
            "naturally_quiescent": first.naturally_quiescent
                && initial_return.naturally_quiescent
                && reused.naturally_quiescent
                && alternative.naturally_quiescent
                && returned.naturally_quiescent,
        }),
        falsifier: (!expected).then(|| {
            "the unanswered return did not produce the preregistered bounded local opportunity"
                .to_string()
        }),
    }
}

fn control_stage(name: &str, axes: usize, surface: Surface, arm: Arm) -> StageResult {
    control_stage_with_protocol(name, axes, surface, arm, arm.protocol(), STEPS)
}

fn control_stage_with_protocol(
    name: &str,
    axes: usize,
    surface: Surface,
    arm: Arm,
    protocol: Protocol,
    steps: usize,
) -> StageResult {
    let mut world = AxisWorld::new_with_protocol(axes, surface, arm, protocol);
    let checkpoint = world.harness.save().expect("stage checkpoint saves");
    let history = world.run(steps);
    let mut replay = AxisWorld::restore_with_protocol(axes, surface, arm, protocol, checkpoint);
    let replayed = replay.run(steps);
    let exact_replay = history == replayed
        && world
            .harness
            .save()
            .and_then(|checkpoint| checkpoint.canonical_bytes())
            == replay
                .harness
                .save()
                .and_then(|checkpoint| checkpoint.canonical_bytes());
    let changed_steps = history
        .iter()
        .filter(|step| !step.changed.is_empty())
        .count();
    let isolated_axes = history
        .iter()
        .filter_map(|step| match step.changed.as_slice() {
            [axis] => Some(*axis),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let directions = history
        .iter()
        .flat_map(|step| step.directions.iter().copied())
        .collect::<BTreeSet<_>>();
    let reached_lower = history.iter().any(|step| step.reached_lower);
    let reached_upper = history.iter().any(|step| step.reached_upper);
    let escaped_lower = history.iter().any(|step| step.escaped_lower);
    let escaped_upper = history.iter().any(|step| step.escaped_upper);
    let bilateral_changes = history.iter().filter(|step| step.bilateral_change).count();
    let delayed_acoustic_changes = history
        .iter()
        .filter(|step| step.delayed_acoustic_change)
        .count();
    let passed = match surface {
        Surface::Proprioceptive if axes == 1 => {
            directions.len() == 2
                && reached_lower
                && reached_upper
                && escaped_lower
                && escaped_upper
                && changed_steps >= 4
        }
        Surface::Proprioceptive => isolated_axes.len() >= 2,
        Surface::Binocular => directions.len() == 2 && bilateral_changes >= 2,
        Surface::VocalAuditory => isolated_axes.len() >= 2 && delayed_acoustic_changes >= 2,
        Surface::Composition => isolated_axes.len() >= 3 && bilateral_changes >= 2,
    } && exact_replay
        && history.iter().all(|step| step.naturally_quiescent);
    StageResult {
        stage: name.to_string(),
        status: if passed {
            StageStatus::Passed
        } else {
            StageStatus::Failed
        },
        observations: serde_json::json!({
            "steps": history.len(),
            "changed_steps": changed_steps,
            "directions": directions,
            "isolated_axes": isolated_axes,
            "reached_lower": reached_lower,
            "reached_upper": reached_upper,
            "escaped_lower": escaped_lower,
            "escaped_upper": escaped_upper,
            "bilateral_changes": bilateral_changes,
            "delayed_acoustic_changes": delayed_acoustic_changes,
            "exact_replay": exact_replay,
            "naturally_quiescent": history.iter().all(|step| step.naturally_quiescent),
        }),
        falsifier: (!passed).then(|| match surface {
            Surface::Proprioceptive if axes == 1 => {
                "causal locality isolated credit but did not produce bidirectional limit recovery"
                    .to_string()
            }
            _ => format!("{name} did not satisfy its preregistered emergence predicate"),
        }),
    }
}

fn physical_input(target: JunctionId, tick: i64, origin: u64) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target,
        impulse: 1,
    }
}

fn add_junction(
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

fn add_link(builder: &mut HarnessBuilder, from: JunctionId, to: JunctionId) {
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

struct IsolationWorld {
    harness: Harness,
    sensors: [JunctionId; 2],
    motors: [JunctionId; 2],
    outcomes: [JunctionId; 2],
}

struct OpportunityWorld {
    harness: Harness,
    input: JunctionId,
    motors: [JunctionId; 2],
    outcomes: [JunctionId; 2],
    physical_outputs: [u64; 2],
}

impl OpportunityWorld {
    fn new(arm: Arm) -> Self {
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_physical_tracing(true);
        builder.set_protocol(arm.protocol());
        let input = add_junction(&mut builder, 600, 0, 0, 1);
        let physical_outputs = [601, 602];
        let motors = [
            add_junction(&mut builder, physical_outputs[0], -1, 0, 2),
            add_junction(&mut builder, physical_outputs[1], 1, 0, 2),
        ];
        let sinks = [
            add_junction(&mut builder, 603, -1, OUTWARD_REGION, 1),
            add_junction(&mut builder, 604, 1, OUTWARD_REGION, 1),
        ];
        let outcomes = [
            add_junction(&mut builder, 605, 50, 0, 1),
            add_junction(&mut builder, 606, 60, 0, 1),
        ];
        let anchor = add_junction(&mut builder, 607, 1_000, 0, 99);
        for target in [input, outcomes[0], outcomes[1]] {
            add_link(&mut builder, anchor, target);
        }
        for index in 0..2 {
            add_link(&mut builder, motors[index], sinks[index]);
        }
        wire_outcomes(&mut builder, arm, &motors, &outcomes);
        Self {
            harness: builder.build(),
            input,
            motors,
            outcomes,
            physical_outputs,
        }
    }

    fn stimulus(&self) -> Vec<Input> {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        vec![
            physical_input(self.input, tick, 2_000 + tick as u64),
            physical_input(self.motors[0], tick + 1, 3_000 + tick as u64),
            physical_input(self.motors[1], tick + 1, 4_000 + tick as u64),
        ]
    }

    fn stimulate(&mut self) -> truelearner_core::Run {
        self.harness.send(&self.stimulus())
    }

    fn return_for(&mut self, output: u64) -> truelearner_core::Run {
        let index = self
            .physical_outputs
            .iter()
            .position(|physical| *physical == output)
            .expect("output belongs to opportunity fixture");
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[physical_input(
            self.outcomes[index],
            tick,
            5_000 + tick as u64,
        )])
    }

    fn used_strength(&self, output: u64) -> i64 {
        let index = self
            .physical_outputs
            .iter()
            .position(|physical| *physical == output)
            .expect("output belongs to opportunity fixture");
        self.harness
            .read()
            .links
            .into_iter()
            .filter(|link| link.live && link.to == self.motors[index] && link.participation > 0)
            .map(|link| link.strength)
            .max()
            .expect("used output has a participating path")
    }
}

impl IsolationWorld {
    fn new(arm: Arm, reflected: bool) -> Self {
        let positions = if reflected { [10, 0] } else { [0, 10] };
        let mut builder = HarnessBuilder::with_capacity(64, 128, OUTWARD_REGION);
        builder.set_physical_tracing(true);
        let sensors = std::array::from_fn(|index| {
            add_junction(&mut builder, 100 + index as u64, positions[index], 0, 1)
        });
        let motors = std::array::from_fn(|index| {
            add_junction(&mut builder, 200 + index as u64, positions[index] + 1, 0, 2)
        });
        let sinks: [JunctionId; 2] = std::array::from_fn(|index| {
            add_junction(
                &mut builder,
                300 + index as u64,
                positions[index] + 1,
                OUTWARD_REGION,
                1,
            )
        });
        let outcomes =
            std::array::from_fn(|index| add_junction(&mut builder, 400 + index as u64, 100, 0, 1));
        let anchor = add_junction(&mut builder, 500, 1_000, 0, 99);
        for target in sensors.into_iter().chain(outcomes) {
            add_link(&mut builder, anchor, target);
        }
        for index in 0..2 {
            add_link(&mut builder, motors[index], sinks[index]);
        }
        wire_outcomes(&mut builder, arm, &motors, &outcomes);
        Self {
            harness: builder.build(),
            sensors,
            motors,
            outcomes,
        }
    }

    fn participate(&mut self) -> truelearner_core::Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness.send(&[
            physical_input(self.sensors[0], tick, 1_000),
            physical_input(self.sensors[1], tick, 1_001),
            physical_input(self.motors[0], tick + 1, 1_002),
            physical_input(self.motors[1], tick + 1, 1_003),
        ])
    }

    fn return_first_consequence(&mut self) -> truelearner_core::Run {
        let tick = self.harness.read().clock.tick.saturating_add(1);
        self.harness
            .send(&[physical_input(self.outcomes[0], tick, 1_004)])
    }

    fn used_strengths(&self) -> [i64; 2] {
        let observation = self.harness.read();
        std::array::from_fn(|index| {
            observation
                .links
                .iter()
                .filter(|link| link.live && link.to == self.motors[index] && link.participation > 0)
                .map(|link| link.strength)
                .max()
                .expect("fixture output participates")
        })
    }
}

fn wire_outcomes(
    builder: &mut HarnessBuilder,
    arm: Arm,
    motors: &[JunctionId],
    outcomes: &[JunctionId],
) {
    match arm.wiring() {
        OutcomeWiring::Global => builder.set_outcome_source(outcomes[0]),
        OutcomeWiring::Local => {
            for (motor, outcome) in motors.iter().zip(outcomes) {
                builder.set_outcome_source_for_output(*motor, *outcome);
            }
        }
        OutcomeWiring::Shuffled => {
            for (index, motor) in motors.iter().enumerate() {
                builder
                    .set_outcome_source_for_output(*motor, outcomes[(index + 1) % outcomes.len()]);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AxisStep {
    changed: Vec<usize>,
    directions: Vec<i8>,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    bilateral_change: bool,
    delayed_acoustic_change: bool,
    naturally_quiescent: bool,
}

struct AxisWorld {
    harness: Harness,
    protocol: Protocol,
    arm: Arm,
    surface: Surface,
    sensors: Vec<Vec<JunctionId>>,
    motors: Vec<[JunctionId; 2]>,
    motor_physical: Vec<[u64; 2]>,
    outcomes: Vec<[JunctionId; 2]>,
    positions: Vec<i16>,
    pending: Vec<JunctionId>,
    prior_bilateral: [u8; 2],
    prior_acoustic: [u8; 2],
    sequence: u64,
}

impl AxisWorld {
    fn new_with_protocol(axes: usize, surface: Surface, arm: Arm, protocol: Protocol) -> Self {
        let (junction_capacity, link_capacity) = if matches!(
            protocol,
            Protocol::SensorimotorCandidate | Protocol::SensorimotorSynthesis
        ) {
            (65_536, 262_144)
        } else {
            (1_024, 4_096)
        };
        let mut builder =
            HarnessBuilder::with_capacity(junction_capacity, link_capacity, OUTWARD_REGION);
        builder.set_protocol(protocol);
        let mut sensors = Vec::with_capacity(axes);
        let mut motors = Vec::with_capacity(axes);
        let mut motor_physical = Vec::with_capacity(axes);
        let mut outcomes = Vec::with_capacity(axes);
        let anchor = add_junction(&mut builder, 90_000, 10_000, 0, 99);
        for axis in 0..axes {
            let center = 10 + i32::try_from(axis).unwrap_or(0) * AXIS_SPACING;
            let local_sensors = (0..9)
                .map(|channel| {
                    let sensor = add_junction(
                        &mut builder,
                        10_000 + (axis * 16 + channel) as u64,
                        center,
                        0,
                        1,
                    );
                    add_link(&mut builder, anchor, sensor);
                    sensor
                })
                .collect::<Vec<_>>();
            sensors.push(local_sensors);
            let physical = [20_000 + (axis * 2) as u64, 20_001 + (axis * 2) as u64];
            let pair = [
                add_junction(&mut builder, physical[0], center - 1, 0, 2),
                add_junction(&mut builder, physical[1], center + 1, 0, 2),
            ];
            let sinks = [
                add_junction(
                    &mut builder,
                    30_000 + (axis * 2) as u64,
                    center - 1,
                    OUTWARD_REGION,
                    1,
                ),
                add_junction(
                    &mut builder,
                    30_001 + (axis * 2) as u64,
                    center + 1,
                    OUTWARD_REGION,
                    1,
                ),
            ];
            add_link(&mut builder, pair[0], sinks[0]);
            add_link(&mut builder, pair[1], sinks[1]);
            let returns = [
                add_junction(
                    &mut builder,
                    40_000 + (axis * 2) as u64,
                    1_000 + axis as i32 * 2,
                    0,
                    1,
                ),
                add_junction(
                    &mut builder,
                    40_001 + (axis * 2) as u64,
                    1_001 + axis as i32 * 2,
                    0,
                    1,
                ),
            ];
            add_link(&mut builder, anchor, returns[0]);
            add_link(&mut builder, anchor, returns[1]);
            motors.push(pair);
            motor_physical.push(physical);
            outcomes.push(returns);
        }
        let flat_motors = motors.iter().flatten().copied().collect::<Vec<_>>();
        let flat_outcomes = outcomes.iter().flatten().copied().collect::<Vec<_>>();
        wire_outcomes(&mut builder, arm, &flat_motors, &flat_outcomes);
        Self {
            harness: builder.build(),
            protocol,
            arm,
            surface,
            sensors,
            motors,
            motor_physical,
            outcomes,
            positions: vec![0; axes],
            pending: Vec::new(),
            prior_bilateral: [0; 2],
            prior_acoustic: [0; 2],
            sequence: 0,
        }
    }

    fn restore_with_protocol(
        axes: usize,
        surface: Surface,
        arm: Arm,
        protocol: Protocol,
        checkpoint: truelearner_core::Checkpoint,
    ) -> Self {
        let template = Self::new_with_protocol(axes, surface, arm, protocol);
        Self {
            harness: Harness::restore(checkpoint).expect("stage checkpoint restores"),
            ..template
        }
    }

    fn run(&mut self, steps: usize) -> Vec<AxisStep> {
        (0..steps).map(|_| self.step()).collect()
    }

    fn step(&mut self) -> AxisStep {
        let before = self.positions.clone();
        let bilateral_before = self.bilateral_signal();
        let mut naturally_quiescent = true;
        let mut outputs = Vec::new();
        if !self.pending.is_empty() {
            let tick = self.harness.read().clock.tick.saturating_add(1);
            let inputs = self
                .pending
                .drain(..)
                .enumerate()
                .map(|(index, target)| {
                    physical_input(target, tick, 60_000 + self.sequence * 100 + index as u64)
                })
                .collect::<Vec<_>>();
            let returned = self.harness.send(&inputs);
            naturally_quiescent &= returned.naturally_quiescent;
            outputs.extend(returned.outputs);
        }

        let tick = self.harness.read().clock.tick.saturating_add(1);
        let proprioceptive_tick = if self.protocol == Protocol::SensorimotorSynthesis {
            tick.saturating_add(2)
        } else {
            tick.saturating_add(1)
        };
        let mut inputs = Vec::new();
        for axis in 0..self.positions.len() {
            for channel in self.active_channels(axis) {
                inputs.push(physical_input(
                    self.sensors[axis][channel],
                    tick,
                    70_000 + self.sequence * 1_000 + (axis * 16 + channel) as u64,
                ));
            }
            inputs.push(physical_input(
                self.motors[axis][0],
                proprioceptive_tick,
                75_000 + self.sequence * 1_000 + (axis * 2) as u64,
            ));
            inputs.push(physical_input(
                self.motors[axis][1],
                proprioceptive_tick,
                75_001 + self.sequence * 1_000 + (axis * 2) as u64,
            ));
        }
        let run = self.harness.send(&inputs);
        naturally_quiescent &= run.naturally_quiescent;
        outputs.extend(run.outputs);

        let mut efforts = vec![[0_i32; 2]; self.positions.len()];
        for output in outputs {
            for (axis, physical) in self.motor_physical.iter().enumerate() {
                if output.from_physical == physical[0] {
                    efforts[axis][0] = efforts[axis][0].saturating_add(output.impulse.abs());
                } else if output.from_physical == physical[1] {
                    efforts[axis][1] = efforts[axis][1].saturating_add(output.impulse.abs());
                }
            }
        }

        let mut changed = Vec::new();
        let mut directions = Vec::new();
        let mut reached_lower = false;
        let mut reached_upper = false;
        let mut escaped_lower = false;
        let mut escaped_upper = false;
        let mut causal_returns = Vec::new();
        for (axis, effort) in efforts.iter().enumerate() {
            let direction = effort[1].cmp(&effort[0]);
            let delta = match direction {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            let next = self.positions[axis]
                .saturating_add(delta)
                .clamp(LOWER, UPPER);
            if next != self.positions[axis] {
                escaped_lower |= self.positions[axis] == LOWER && delta > 0;
                escaped_upper |= self.positions[axis] == UPPER && delta < 0;
                self.positions[axis] = next;
                reached_lower |= next == LOWER;
                reached_upper |= next == UPPER;
                changed.push(axis);
                directions.push(delta as i8);
                causal_returns.push(if delta < 0 {
                    self.outcomes[axis][0]
                } else {
                    self.outcomes[axis][1]
                });
            }
        }
        self.pending = match self.arm.wiring() {
            OutcomeWiring::Global if !changed.is_empty() => vec![self.outcomes[0][0]],
            OutcomeWiring::Global => Vec::new(),
            OutcomeWiring::Local | OutcomeWiring::Shuffled => causal_returns,
        };
        let bilateral_after = self.bilateral_signal();
        let acoustic_after = self.acoustic_signal();
        let bilateral_change = bilateral_before != bilateral_after;
        let delayed_acoustic_change = self.prior_acoustic != acoustic_after;
        self.prior_bilateral = bilateral_after;
        self.prior_acoustic = acoustic_after;
        self.sequence = self.sequence.saturating_add(1);
        debug_assert_eq!(before.len(), self.positions.len());
        AxisStep {
            changed,
            directions,
            reached_lower,
            reached_upper,
            escaped_lower,
            escaped_upper,
            bilateral_change,
            delayed_acoustic_change,
            naturally_quiescent,
        }
    }

    fn active_channels(&self, axis: usize) -> Vec<usize> {
        let position = self.positions[axis];
        let mut active = vec![2];
        if position < 0 {
            active.push(0);
        } else if position > 0 {
            active.push(1);
        }
        if position == LOWER {
            active.push(3);
        }
        if position == UPPER {
            active.push(4);
        }
        let bilateral = self.bilateral_signal();
        if matches!(self.surface, Surface::Binocular | Surface::Composition) {
            active.push(5 + usize::from(bilateral[0] >= bilateral[1]));
        }
        let acoustic = self.prior_acoustic;
        if matches!(self.surface, Surface::VocalAuditory | Surface::Composition)
            && acoustic[0].saturating_add(acoustic[1]) > 0
        {
            active.push(7 + usize::from(acoustic[0] < acoustic[1]));
        }
        active
    }

    fn bilateral_signal(&self) -> [u8; 2] {
        if !matches!(self.surface, Surface::Binocular | Surface::Composition) {
            return [0; 2];
        }
        let vergence = self.positions.first().copied().unwrap_or(0);
        let depth = 3_i16;
        [
            u8::try_from((depth - vergence).unsigned_abs()).unwrap_or(u8::MAX),
            u8::try_from((depth + vergence).unsigned_abs()).unwrap_or(u8::MAX),
        ]
    }

    fn acoustic_signal(&self) -> [u8; 2] {
        if !matches!(self.surface, Surface::VocalAuditory | Surface::Composition) {
            return [0; 2];
        }
        let energy = self.positions.iter().fold(0_u16, |sum, position| {
            sum.saturating_add(position.unsigned_abs())
        });
        let timbre = self.positions.get(1).copied().unwrap_or(0);
        [
            u8::try_from(energy.saturating_add(timbre.unsigned_abs())).unwrap_or(u8::MAX),
            u8::try_from(energy.saturating_add((-timbre).unsigned_abs())).unwrap_or(u8::MAX),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_return_reference_records_shared_credit() {
        let result = run(Arm::GlobalReturn, false);
        assert_eq!(result.outcome, "control-confirmed");
        assert_eq!(result.stages[0].observations["credited_paths"], 2);
        assert!(result.exact_replay);
        assert!(result.naturally_quiescent);
    }

    #[test]
    fn causal_local_return_records_exact_credit() {
        let result = run(Arm::CausalLocalReturn, false);
        assert_eq!(result.outcome, "survived");
        assert_eq!(result.stages[0].observations["credited_paths"], 1);
        assert_eq!(result.stages[0].observations["credited_path"], 0);
        assert!(result.exact_replay);
    }

    #[test]
    fn shuffled_local_return_records_wrong_credit() {
        let result = run(Arm::ShuffledLocalReturn, false);
        assert_eq!(result.outcome, "control-confirmed");
        assert_eq!(result.stages[0].observations["credited_path"], 1);
    }

    #[test]
    fn full_ladder_stops_after_the_first_failed_emergence_stage() {
        let result = run(Arm::CausalLocalReturn, true);
        let failed = result
            .stages
            .iter()
            .position(|stage| stage.status == StageStatus::Failed)
            .expect("the current candidate has a preserved first failure");
        assert!(result.stages[failed + 1..]
            .iter()
            .all(|stage| stage.status == StageStatus::NotRun));
    }

    #[test]
    fn causal_local_reference_preserves_saturation() {
        let result = run(Arm::CausalLocalReference, true);
        assert_eq!(result.outcome, "control-confirmed");
        assert_eq!(result.stages[1].status, StageStatus::Failed);
        assert_eq!(
            result.stages[1].observations["directions"],
            serde_json::json!([1])
        );
    }

    #[test]
    fn unanswered_local_return_defers_to_one_fresh_neighbor() {
        let result = run(Arm::LocalReturnDeferral, false);
        assert_eq!(result.stages[1].status, StageStatus::Passed);
        assert_eq!(result.stages[1].observations["superseded_returns"], 0);
        assert_eq!(
            result.stages[1].observations["return_paths_before_alternative_outcome"],
            2
        );
    }

    #[test]
    fn superseded_local_return_is_replaced() {
        let result = run(Arm::LocalReturnReplacement, false);
        assert_eq!(result.stages[1].status, StageStatus::Passed);
        assert_eq!(result.stages[1].observations["superseded_returns"], 1);
        assert_eq!(
            result.stages[1].observations["return_paths_before_alternative_outcome"],
            1
        );
    }

    #[test]
    fn shuffled_local_reference_preserves_wrong_credit() {
        let result = run(Arm::ShuffledLocalReference, false);
        assert_eq!(result.outcome, "control-confirmed");
        assert_eq!(result.stages[0].observations["credited_path"], 1);
    }
}
