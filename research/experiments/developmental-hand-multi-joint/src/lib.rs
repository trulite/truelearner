#![forbid(unsafe_code)]

use developmental_hand_construction_admission::run_reflected_hand_bounded;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, Link, Output, OutputCompetitionBasis,
    PhysicalEvent, PhysicalIncidence, PhysicalInput, Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const LOWER: i16 = -4;
const UPPER: i16 = 4;
const PRIMARY_STEPS: usize = 16;
const DEVELOPMENT_STEPS: usize = 8;
const RECOVERY_STEPS: usize = 16;
const MAX_MOMENTS_PER_SEND: u64 = 256;
const JUNCTION_CAPACITY: u32 = 16_384;
const LINK_CAPACITY: u32 = 65_536;
const PHYSICAL_BASE: u64 = 1_000_000;
const PHYSICAL_STRIDE: u64 = 100_000;

type MotorOriginSets = Vec<[Vec<u64>; 2]>;
type PublicHistoryEntry<'a> = (&'a [i16], &'a [i16], &'a [i8], &'a [Vec<u64>]);

const PREDECESSOR_HASHES: [(&str, &str); 2] = [
    (
        "complete-one-joint.json",
        "60245d1ab700e879f41d8011c8f4ba39a27815d301bce9d62a1163f059db4053",
    ),
    (
        "convergence.toml",
        "a83340005269f22cefea49bb0985551c6597a69175aec25ccf2a0da0df34396a",
    ),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum BuildOrder {
    Forward,
    Reverse,
}

#[derive(Clone, Debug)]
struct JointModule {
    label: usize,
    sensors: Vec<JunctionId>,
    sensor_physical: Vec<u64>,
    motors: [JunctionId; 2],
    motor_physical: [u64; 2],
    outcome_physical: [u64; 2],
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct StepDiagnostics {
    pub physical_inputs: u64,
    pub drive_events: u64,
    pub motor_drive_events: u64,
    pub foreign_motor_drive_events: u64,
    pub output_events: u64,
    pub return_scheduling: u64,
    pub return_admissions: u64,
    pub consequence_writes: u64,
    pub owner_reads: u64,
    pub consequential_owner_reads: u64,
    pub selections: u64,
    pub propagation_budget_exhaustions: u64,
}

impl StepDiagnostics {
    fn observe(&mut self, run: &Run, modules: &[JointModule]) {
        for transition in &run.physical_trace {
            match transition.event {
                PhysicalEvent::PhysicalIncidenceObserved { .. } => self.physical_inputs += 1,
                PhysicalEvent::DriveIncidence { .. } => self.drive_events += 1,
                PhysicalEvent::DriveProvenanceObserved {
                    target,
                    carried_origin,
                    ..
                } => {
                    if let Some(target_module) = module_for_motor(modules, target) {
                        self.motor_drive_events += 1;
                        if physical_module(carried_origin, modules.len())
                            .is_some_and(|origin_module| origin_module != target_module)
                        {
                            self.foreign_motor_drive_events += 1;
                        }
                    }
                }
                PhysicalEvent::Output(_) => self.output_events += 1,
                PhysicalEvent::ReturnScheduling { .. } => self.return_scheduling += 1,
                PhysicalEvent::ReturnOriginAdmission { .. } => self.return_admissions += 1,
                PhysicalEvent::LearnerConsequenceRecorded { .. } => self.consequence_writes += 1,
                PhysicalEvent::LearnerCandidatePreference {
                    consequence_tick, ..
                } => {
                    self.owner_reads += 1;
                    if consequence_tick.is_some() {
                        self.consequential_owner_reads += 1;
                    }
                }
                PhysicalEvent::CandidateSelection { .. } => self.selections += 1,
                PhysicalEvent::PropagationBudgetExhausted { .. } => {
                    self.propagation_budget_exhaustions += 1;
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MultiJointStep {
    pub index: usize,
    pub positions_before: Vec<i16>,
    pub positions_after: Vec<i16>,
    pub serial_pose_after: Vec<i16>,
    pub directions: Vec<i8>,
    pub phase_outputs: Vec<Vec<u64>>,
    pub emitted_outputs: Vec<u64>,
    pub motor_origins: Vec<[Vec<u64>; 2]>,
    pub motor_path_origins: Vec<[Vec<u64>; 2]>,
    pub output_competition: Vec<MotorCompetitionComponent>,
    pub motor_candidates: Vec<MotorCandidateEvaluation>,
    pub naturally_quiescent: bool,
    pub comparisons: u64,
    pub scans: u64,
    pub batched_items: u64,
    pub work: u64,
    pub physical_work: u64,
    pub learners: usize,
    pub junctions: usize,
    pub links: usize,
    pub diagnostics: StepDiagnostics,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct MotorCompetitionComponent {
    pub phase: usize,
    pub module: usize,
    pub side: usize,
    pub component: Option<u64>,
    pub basis: OutputCompetitionBasis,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct MotorCandidateEvaluation {
    pub phase: usize,
    pub module: usize,
    pub side: usize,
    pub path_inputs: u32,
    pub distinct_path_origins: u32,
    pub distinct_path_owners: u32,
    pub positive_path_strength: u64,
    pub negative_path_strength: u64,
    pub opportunity: i64,
    pub supplied_opportunity: i64,
    pub admitted_drive: i64,
    pub projected_drive: i64,
    pub threshold: i64,
    pub unanswered_returns: u32,
    pub executable: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct JointCapability {
    pub label: usize,
    pub reached_lower: bool,
    pub reached_upper: bool,
    pub escaped_lower: bool,
    pub escaped_upper: bool,
    pub directions: BTreeSet<i8>,
    pub changed_steps: usize,
    pub closed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ChainRun {
    pub joint_count: usize,
    pub history: Vec<MultiJointStep>,
    pub joints: Vec<JointCapability>,
    pub exact_replay: bool,
    pub naturally_quiescent: bool,
    pub propagation_budget_exhaustions: u64,
    pub comparisons: u64,
    pub scans: u64,
    pub batched_items: u64,
    pub work: u64,
    pub physical_work: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PerturbationEvidence {
    pub imposed_joint: usize,
    pub imposed_position: i16,
    pub development: Vec<MultiJointStep>,
    pub recovery: Vec<MultiJointStep>,
    pub joint_capabilities: Vec<JointCapability>,
    pub proximal_left_imposed_limit: bool,
    pub distal_control_preserved: bool,
    pub serial_pose_changed: bool,
    pub exact_replay: bool,
    pub naturally_quiescent: bool,
    pub propagation_budget_exhaustions: u64,
    pub recovered: bool,
}

#[derive(Clone, Debug, Serialize)]
struct DigestControl {
    name: &'static str,
    expected_sha256: &'static str,
    observed_sha256: String,
    matched: bool,
}

pub struct PredecessorBytes<'a> {
    pub complete_one_joint: &'a [u8],
    pub convergence: &'a [u8],
}

#[derive(Clone, Debug, Serialize)]
pub struct DevelopmentEvidence {
    pub protocol: Protocol,
    predecessor_controls: Vec<DigestControl>,
    pub predecessor_controls_survived: bool,
    pub one_module: ChainRun,
    pub one_module_identity: bool,
    pub two_forward: ChainRun,
    pub two_reverse: ChainRun,
    pub two_registration_order_invariant: bool,
    pub two_perturbation: PerturbationEvidence,
    pub two_joint_survived: bool,
    pub five_forward: Option<ChainRun>,
    pub five_reverse: Option<ChainRun>,
    pub five_registration_order_invariant: Option<bool>,
    pub five_perturbation: Option<PerturbationEvidence>,
    pub five_joint_survived: Option<bool>,
    pub five_rung_executed: bool,
    pub first_failure: Option<FirstFailure>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FirstFailure {
    pub rung: usize,
    pub joint: Option<usize>,
    pub step: Option<usize>,
    pub stage: &'static str,
    pub explanation: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    OneModuleIdentity,
    TwoJointSerialComposition,
    RegistrationOrderInvariance,
    ProximalChangeIsolation,
    ConditionalFiveJoint,
}

impl Arm {
    pub const ALL: [Self; 5] = [
        Self::OneModuleIdentity,
        Self::TwoJointSerialComposition,
        Self::RegistrationOrderInvariance,
        Self::ProximalChangeIsolation,
        Self::ConditionalFiveJoint,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::OneModuleIdentity => "one-module-identity",
            Self::TwoJointSerialComposition => "two-joint-serial-composition",
            Self::RegistrationOrderInvariance => "registration-order-invariance",
            Self::ProximalChangeIsolation => "proximal-change-isolation",
            Self::ConditionalFiveJoint => "conditional-five-joint",
        }
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

struct MultiJointWorld {
    harness: Harness,
    modules: Vec<JointModule>,
    positions: Vec<i16>,
    pending: Vec<(usize, Vec<usize>)>,
    sequence: usize,
}

impl MultiJointWorld {
    fn new(joint_count: usize, order: BuildOrder, protocol: Protocol) -> Self {
        let mut builder =
            HarnessBuilder::with_capacity(JUNCTION_CAPACITY, LINK_CAPACITY, OUTWARD_REGION);
        builder.set_protocol(protocol);
        builder.set_physical_tracing(true);
        let labels = match order {
            BuildOrder::Forward => (0..joint_count).collect::<Vec<_>>(),
            BuildOrder::Reverse => (0..joint_count).rev().collect::<Vec<_>>(),
        };
        let mut modules = labels
            .into_iter()
            .map(|label| build_module(&mut builder, label))
            .collect::<Vec<_>>();
        modules.sort_by_key(|module| module.label);
        Self {
            harness: builder.build(),
            modules,
            positions: vec![0; joint_count],
            pending: Vec::new(),
            sequence: 0,
        }
    }

    fn send(&mut self, inputs: &[Input]) -> Run {
        self.harness.send_bounded(inputs, MAX_MOMENTS_PER_SEND)
    }

    fn send_physical(&mut self, inputs: &[PhysicalInput]) -> Run {
        self.harness
            .send_physical_bounded(inputs, MAX_MOMENTS_PER_SEND)
    }

    fn step(&mut self) -> MultiJointStep {
        let positions_before = self.positions.clone();
        let mut runs = Vec::new();
        let mut phase_outputs = Vec::new();

        if !self.pending.is_empty() {
            let tick = self.harness.read().clock.tick.saturating_add(1);
            let pending = std::mem::take(&mut self.pending);
            let mut inputs = Vec::new();
            for (label, channels) in pending {
                let module = &self.modules[label];
                for channel in channels {
                    inputs.push(PhysicalInput {
                        input: physical_input(
                            module.sensors[channel],
                            tick,
                            module.sensor_physical[channel],
                        ),
                        incidence: PhysicalIncidence::Transition,
                    });
                }
            }
            let returned = self.send_physical(&inputs);
            phase_outputs.push(output_sources(&returned.outputs));
            runs.push(returned);
        }

        let tick = self.harness.read().clock.tick.saturating_add(1);
        let mut inputs = Vec::new();
        for module in &self.modules {
            for channel in active_channels(self.positions[module.label]) {
                inputs.push(physical_input(
                    module.sensors[channel],
                    tick,
                    module.sensor_physical[channel],
                ));
            }
            for index in 0..2 {
                inputs.push(physical_input(
                    module.motors[index],
                    tick.saturating_add(2),
                    module.outcome_physical[index],
                ));
            }
        }
        let current = self.send(&inputs);
        phase_outputs.push(output_sources(&current.outputs));
        runs.push(current);

        let outputs = runs
            .iter()
            .flat_map(|run| run.outputs.iter().copied())
            .collect::<Vec<_>>();
        let directions = self.apply_outputs(&outputs);
        let emitted_outputs = output_sources(&outputs);
        let (motor_origins, motor_path_origins) = motor_origin_sets(&runs, &self.modules);
        let output_competition = motor_competition_components(&runs, &self.modules);
        let motor_candidates = motor_candidate_evaluations(&runs, &self.modules);
        let naturally_quiescent = runs.iter().all(|run| run.naturally_quiescent);
        let comparisons = runs.iter().map(|run| run.execution_cost.comparisons).sum();
        let scans = runs.iter().map(|run| run.execution_cost.scans).sum();
        let batched_items = runs
            .iter()
            .map(|run| run.execution_cost.batched_items)
            .sum();
        let work = runs.iter().map(|run| run.work.total()).sum();
        let physical_work = runs.iter().map(|run| run.work.physical_total()).sum();
        let mut diagnostics = StepDiagnostics::default();
        for run in &runs {
            diagnostics.observe(run, &self.modules);
        }
        let observation = self.harness.read();
        let step = MultiJointStep {
            index: self.sequence,
            positions_before,
            positions_after: self.positions.clone(),
            serial_pose_after: serial_pose(&self.positions),
            directions,
            phase_outputs,
            emitted_outputs,
            motor_origins,
            motor_path_origins,
            output_competition,
            motor_candidates,
            naturally_quiescent,
            comparisons,
            scans,
            batched_items,
            work,
            physical_work,
            learners: observation.learners.len(),
            junctions: observation.junctions.len(),
            links: observation.links.len(),
            diagnostics,
        };
        self.sequence += 1;
        step
    }

    fn apply_outputs(&mut self, outputs: &[Output]) -> Vec<i8> {
        let mut efforts = vec![[0_i64; 2]; self.modules.len()];
        for output in outputs {
            for module in &self.modules {
                if output.from_physical == module.motor_physical[0] {
                    efforts[module.label][0] += i64::from(output.impulse).abs();
                } else if output.from_physical == module.motor_physical[1] {
                    efforts[module.label][1] += i64::from(output.impulse).abs();
                }
            }
        }
        let mut directions = Vec::with_capacity(self.modules.len());
        for module in &self.modules {
            let label = module.label;
            let direction = match efforts[label][1].cmp(&efforts[label][0]) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            let before = self.positions[label];
            let after = before
                .saturating_add(i16::from(direction))
                .clamp(LOWER, UPPER);
            self.positions[label] = after;
            if after != before {
                self.pending.push((label, active_channels(after)));
            }
            directions.push(direction);
        }
        directions
    }
}

fn build_module(builder: &mut HarnessBuilder, label: usize) -> JointModule {
    let physical = PHYSICAL_BASE + label as u64 * PHYSICAL_STRIDE;
    let position = 10 + i32::try_from(label).unwrap_or(i32::MAX).saturating_mul(4);
    let anchor = add_junction(builder, physical + 90_000, 10_000 + position, 0, 99);
    let sensor_physical = (0..9)
        .map(|channel| physical + 10_000 + channel)
        .collect::<Vec<_>>();
    let sensors = sensor_physical
        .iter()
        .map(|source| {
            let sensor = add_junction(builder, *source, position, 0, 1);
            add_link(builder, anchor, sensor, 0);
            sensor
        })
        .collect::<Vec<_>>();
    let motor_physical = [physical + 20_000, physical + 20_001];
    let motors = [
        add_junction(builder, motor_physical[0], position - 1, 0, 2),
        add_junction(builder, motor_physical[1], position + 1, 0, 2),
    ];
    let sinks = [
        add_junction(builder, physical + 30_000, position - 1, OUTWARD_REGION, 1),
        add_junction(builder, physical + 30_001, position + 1, OUTWARD_REGION, 1),
    ];
    for index in 0..2 {
        add_link(builder, motors[index], sinks[index], 0);
    }
    let outcome_physical = [physical + 40_000, physical + 40_001];
    let outcomes = [
        add_junction(builder, outcome_physical[0], 1_000 + position, 0, 1),
        add_junction(builder, outcome_physical[1], 1_001 + position, 0, 1),
    ];
    for outcome in outcomes {
        add_link(builder, anchor, outcome, 0);
    }
    for sensor in &sensors {
        for outcome in outcomes {
            add_link(builder, *sensor, outcome, 3);
        }
    }
    for index in 0..2 {
        builder.set_outcome_source_for_output(motors[index], outcomes[index]);
    }
    JointModule {
        label,
        sensors,
        sensor_physical,
        motors,
        motor_physical,
        outcome_physical,
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

fn add_link(builder: &mut HarnessBuilder, from: JunctionId, to: JunctionId, delay: i64) {
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

fn physical_input(target: JunctionId, arrival_tick: i64, origin_physical: u64) -> Input {
    Input {
        arrival_tick,
        phase: 0,
        origin_physical,
        target,
        impulse: 1,
    }
}

fn active_channels(position: i16) -> Vec<usize> {
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
    active
}

fn serial_pose(positions: &[i16]) -> Vec<i16> {
    positions
        .iter()
        .scan(0_i16, |pose, position| {
            *pose = pose.saturating_add(*position);
            Some(*pose)
        })
        .collect()
}

fn output_sources(outputs: &[Output]) -> Vec<u64> {
    outputs.iter().map(|output| output.from_physical).collect()
}

fn module_for_motor(modules: &[JointModule], target: JunctionId) -> Option<usize> {
    modules
        .iter()
        .find(|module| module.motors.contains(&target))
        .map(|module| module.label)
}

fn motor_for_target(modules: &[JointModule], target: JunctionId) -> Option<(usize, usize)> {
    modules.iter().find_map(|module| {
        module
            .motors
            .iter()
            .position(|motor| *motor == target)
            .map(|side| (module.label, side))
    })
}

fn motor_origin_sets(runs: &[Run], modules: &[JointModule]) -> (MotorOriginSets, MotorOriginSets) {
    let mut origins = vec![[Vec::new(), Vec::new()]; modules.len()];
    let mut path_origins = vec![[Vec::new(), Vec::new()]; modules.len()];
    for transition in runs.iter().flat_map(|run| &run.physical_trace) {
        match transition.event {
            PhysicalEvent::CausalLineageMemberObserved {
                target,
                origin_physical,
                ..
            } => {
                if let Some((module, side)) = motor_for_target(modules, target) {
                    origins[module][side].push(origin_physical);
                }
            }
            PhysicalEvent::DriveProvenanceObserved {
                target,
                carried_origin,
                completes_path: true,
                ..
            } => {
                if let Some((module, side)) = motor_for_target(modules, target) {
                    path_origins[module][side].push(carried_origin);
                }
            }
            _ => {}
        }
    }
    for sides in origins.iter_mut().chain(&mut path_origins) {
        for side in sides {
            side.sort_unstable();
            side.dedup();
        }
    }
    (origins, path_origins)
}

fn motor_competition_components(
    runs: &[Run],
    modules: &[JointModule],
) -> Vec<MotorCompetitionComponent> {
    runs.iter()
        .enumerate()
        .flat_map(|(phase, run)| {
            run.physical_trace.iter().filter_map(move |transition| {
                if let PhysicalEvent::OutputCompetitionComponent {
                    target,
                    component,
                    basis,
                    ..
                } = transition.event
                {
                    motor_for_target(modules, target).map(|(module, side)| {
                        MotorCompetitionComponent {
                            phase,
                            module,
                            side,
                            component,
                            basis,
                        }
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn motor_candidate_evaluations(
    runs: &[Run],
    modules: &[JointModule],
) -> Vec<MotorCandidateEvaluation> {
    runs.iter()
        .enumerate()
        .flat_map(|(phase, run)| {
            run.physical_trace.iter().filter_map(move |transition| {
                if let PhysicalEvent::OutputCandidateEvaluated {
                    target,
                    path_inputs,
                    distinct_path_origins,
                    distinct_path_owners,
                    positive_path_strength,
                    negative_path_strength,
                    opportunity,
                    supplied_opportunity,
                    admitted_drive,
                    projected_drive,
                    threshold,
                    unanswered_returns,
                    executable,
                    ..
                } = transition.event
                {
                    motor_for_target(modules, target).map(|(module, side)| {
                        MotorCandidateEvaluation {
                            phase,
                            module,
                            side,
                            path_inputs,
                            distinct_path_origins,
                            distinct_path_owners,
                            positive_path_strength,
                            negative_path_strength,
                            opportunity,
                            supplied_opportunity,
                            admitted_drive,
                            projected_drive,
                            threshold,
                            unanswered_returns,
                            executable,
                        }
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn physical_module(physical: u64, joint_count: usize) -> Option<usize> {
    let offset = physical.checked_sub(PHYSICAL_BASE)?;
    let module = usize::try_from(offset / PHYSICAL_STRIDE).ok()?;
    (module < joint_count).then_some(module)
}

fn capabilities(history: &[MultiJointStep], joint_count: usize) -> Vec<JointCapability> {
    (0..joint_count)
        .map(|label| {
            let mut capability = JointCapability {
                label,
                ..JointCapability::default()
            };
            for step in history {
                let before = step.positions_before[label];
                let after = step.positions_after[label];
                let direction = step.directions[label];
                if direction != 0 {
                    capability.directions.insert(direction);
                }
                capability.changed_steps += usize::from(before != after);
                capability.reached_lower |= before != LOWER && after == LOWER;
                capability.reached_upper |= before != UPPER && after == UPPER;
                capability.escaped_lower |= before == LOWER && after > LOWER;
                capability.escaped_upper |= before == UPPER && after < UPPER;
            }
            capability.closed = capability.reached_lower
                && capability.reached_upper
                && capability.escaped_lower
                && capability.escaped_upper
                && capability.directions == BTreeSet::from([-1, 1]);
            capability
        })
        .collect()
}

fn execute(world: &mut MultiJointWorld, steps: usize) -> Vec<MultiJointStep> {
    (0..steps).map(|_| world.step()).collect()
}

fn run_history(joint_count: usize, order: BuildOrder, protocol: Protocol) -> Vec<MultiJointStep> {
    let mut world = MultiJointWorld::new(joint_count, order, protocol);
    execute(&mut world, PRIMARY_STEPS)
}

fn summarize_run(
    joint_count: usize,
    order: BuildOrder,
    protocol: Protocol,
    history: Vec<MultiJointStep>,
) -> ChainRun {
    let replay = run_history(joint_count, order, protocol);
    let exact_replay = history == replay;
    let naturally_quiescent = history.iter().all(|step| step.naturally_quiescent);
    let propagation_budget_exhaustions = history
        .iter()
        .map(|step| step.diagnostics.propagation_budget_exhaustions)
        .sum();
    ChainRun {
        joint_count,
        joints: capabilities(&history, joint_count),
        comparisons: history.iter().map(|step| step.comparisons).sum(),
        scans: history.iter().map(|step| step.scans).sum(),
        batched_items: history.iter().map(|step| step.batched_items).sum(),
        work: history.iter().map(|step| step.work).sum(),
        physical_work: history.iter().map(|step| step.physical_work).sum(),
        history,
        exact_replay,
        naturally_quiescent,
        propagation_budget_exhaustions,
    }
}

fn run_chain(joint_count: usize, order: BuildOrder, protocol: Protocol) -> ChainRun {
    let history = run_history(joint_count, order, protocol);
    summarize_run(joint_count, order, protocol, history)
}

fn run_perturbation_history(
    joint_count: usize,
    protocol: Protocol,
) -> (Vec<MultiJointStep>, Vec<MultiJointStep>) {
    let mut world = MultiJointWorld::new(joint_count, BuildOrder::Forward, protocol);
    let development = execute(&mut world, DEVELOPMENT_STEPS);
    world.positions[0] = UPPER;
    world.pending.clear();
    let recovery = execute(&mut world, RECOVERY_STEPS);
    (development, recovery)
}

fn run_perturbation(joint_count: usize, protocol: Protocol) -> PerturbationEvidence {
    let (development, recovery) = run_perturbation_history(joint_count, protocol);
    let replay = run_perturbation_history(joint_count, protocol);
    let mut complete_history = development.clone();
    complete_history.extend(recovery.iter().cloned());
    let joint_capabilities = capabilities(&complete_history, joint_count);
    let proximal_left_imposed_limit = recovery
        .iter()
        .any(|step| step.positions_before[0] == UPPER && step.positions_after[0] < UPPER);
    let distal_control_preserved = joint_capabilities.iter().skip(1).all(|joint| joint.closed);
    let serial_pose_changed = recovery.iter().any(|step| {
        step.serial_pose_after
            .windows(2)
            .any(|pair| pair[0] != pair[1])
    });
    let exact_replay = (development.clone(), recovery.clone()) == replay;
    let naturally_quiescent = complete_history.iter().all(|step| step.naturally_quiescent);
    let propagation_budget_exhaustions = complete_history
        .iter()
        .map(|step| step.diagnostics.propagation_budget_exhaustions)
        .sum();
    let recovered = joint_capabilities.iter().all(|joint| joint.closed)
        && proximal_left_imposed_limit
        && distal_control_preserved
        && serial_pose_changed
        && exact_replay
        && naturally_quiescent
        && propagation_budget_exhaustions == 0;
    PerturbationEvidence {
        imposed_joint: 0,
        imposed_position: UPPER,
        development,
        recovery,
        joint_capabilities,
        proximal_left_imposed_limit,
        distal_control_preserved,
        serial_pose_changed,
        exact_replay,
        naturally_quiescent,
        propagation_budget_exhaustions,
        recovered,
    }
}

fn public_history(run: &ChainRun) -> Vec<PublicHistoryEntry<'_>> {
    run.history
        .iter()
        .map(|step| {
            (
                step.positions_before.as_slice(),
                step.positions_after.as_slice(),
                step.directions.as_slice(),
                step.phase_outputs.as_slice(),
            )
        })
        .collect()
}

fn chain_survived(run: &ChainRun, one: &ChainRun) -> bool {
    let activity_bound = u64::try_from(run.joint_count).unwrap_or(u64::MAX);
    run.joints.iter().all(|joint| joint.closed)
        && run.exact_replay
        && run.naturally_quiescent
        && run.propagation_budget_exhaustions == 0
        && run.comparisons
            <= one
                .comparisons
                .saturating_mul(activity_bound)
                .saturating_mul(2)
        && run.scans <= one.scans.saturating_mul(activity_bound).saturating_mul(2)
}

fn identity_survived(one: &ChainRun) -> bool {
    let reference = run_reflected_hand_bounded(
        Protocol::RecursiveLearnerReturnBearingContinuation,
        512,
        2_048,
        MAX_MOMENTS_PER_SEND,
    );
    let observed_outputs = one
        .history
        .iter()
        .map(|step| {
            step.phase_outputs
                .iter()
                .map(|phase| {
                    phase
                        .iter()
                        .filter_map(|physical| match *physical {
                            value if value == PHYSICAL_BASE + 20_000 => Some(0),
                            value if value == PHYSICAL_BASE + 20_001 => Some(1),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let reference_outputs = reference
        .trajectory
        .iter()
        .map(|step| {
            step.phase_work
                .iter()
                .map(|phase| {
                    phase
                        .emitted_outputs
                        .iter()
                        .filter_map(|physical| match *physical {
                            20_000 => Some(0),
                            20_001 => Some(1),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    one.history
        .iter()
        .map(|step| (step.positions_before[0], step.positions_after[0]))
        .eq(reference
            .trajectory
            .iter()
            .map(|step| (step.position_before, step.position_after)))
        && observed_outputs == reference_outputs
        && one.joints.iter().all(|joint| joint.closed)
        && one.exact_replay
        && one.naturally_quiescent
        && one.propagation_budget_exhaustions == 0
}

fn registration_invariant(forward: &ChainRun, reverse: &ChainRun) -> bool {
    public_history(forward) == public_history(reverse)
        && forward.joints == reverse.joints
        && forward.naturally_quiescent == reverse.naturally_quiescent
        && forward.propagation_budget_exhaustions == reverse.propagation_budget_exhaustions
}

fn first_failure(
    rung: usize,
    run: &ChainRun,
    identity: &ChainRun,
    perturbation: &PerturbationEvidence,
) -> FirstFailure {
    if let Some(step) = run
        .history
        .iter()
        .find(|step| step.diagnostics.propagation_budget_exhaustions > 0)
    {
        return FirstFailure {
            rung,
            joint: None,
            step: Some(step.index),
            stage: "physical-cost",
            explanation: "the shared causal wave exhausted its propagation budget".to_string(),
        };
    }
    for (step, identity_step) in run.history.iter().zip(&identity.history) {
        for joint in 0..run.joint_count {
            let expected = identity_step.directions[0];
            let observed = step.directions[joint];
            if observed == expected {
                continue;
            }
            let physical = PHYSICAL_BASE + joint as u64 * PHYSICAL_STRIDE;
            let emitted = step
                .emitted_outputs
                .iter()
                .any(|output| matches!(*output, value if value == physical + 20_000 || value == physical + 20_001));
            let (stage, explanation) = if !emitted {
                (
                    "output-choice",
                    "an independently driven local motor produced no outward output",
                )
            } else if observed == 0 {
                (
                    "motor-integration",
                    "opposing local outputs cancelled before physical movement",
                )
            } else {
                (
                    "continued-arrow",
                    "the component emitted a different physical arrow from the one-module identity",
                )
            };
            return FirstFailure {
                rung,
                joint: Some(joint),
                step: Some(step.index),
                stage,
                explanation: explanation.to_string(),
            };
        }
    }
    if let Some(joint) = run.joints.iter().find(|joint| !joint.closed) {
        let (stage, explanation) = if joint.directions.len() < 2 {
            ("reversal", "the joint did not emit both physical signs")
        } else if !joint.reached_lower || !joint.reached_upper {
            (
                "continuation",
                "the joint did not reach both reflected limits",
            )
        } else {
            ("release", "the joint reached a limit but did not leave it")
        };
        return FirstFailure {
            rung,
            joint: Some(joint.label),
            step: None,
            stage,
            explanation: explanation.to_string(),
        };
    }
    if !perturbation.recovered {
        return FirstFailure {
            rung,
            joint: Some(0),
            step: None,
            stage: "perturbation-recovery",
            explanation: "the product state did not recover after the proximal component changed"
                .to_string(),
        };
    }
    FirstFailure {
        rung,
        joint: None,
        step: None,
        stage: "representation-invariance",
        explanation: "forward and reverse module registration changed physical behavior"
            .to_string(),
    }
}

fn digest_controls(predecessor: &PredecessorBytes<'_>) -> Vec<DigestControl> {
    let files = [
        ("complete-one-joint.json", predecessor.complete_one_joint),
        ("convergence.toml", predecessor.convergence),
    ];
    PREDECESSOR_HASHES
        .into_iter()
        .zip(files)
        .map(|((expected_name, expected), (name, bytes))| {
            let observed_sha256 = format!("{:x}", Sha256::digest(bytes));
            DigestControl {
                name,
                expected_sha256: expected,
                matched: name == expected_name && observed_sha256 == expected,
                observed_sha256,
            }
        })
        .collect()
}

pub fn measure(predecessor: &PredecessorBytes<'_>) -> DevelopmentEvidence {
    measure_with_protocol(
        predecessor,
        Protocol::RecursiveLearnerReturnBearingContinuation,
    )
}

pub fn measure_with_protocol(
    predecessor: &PredecessorBytes<'_>,
    protocol: Protocol,
) -> DevelopmentEvidence {
    let predecessor_controls = digest_controls(predecessor);
    let predecessor_controls_survived = predecessor_controls.iter().all(|item| item.matched);
    let one_module = run_chain(1, BuildOrder::Forward, protocol);
    let one_module_identity = identity_survived(&one_module);
    let two_forward = run_chain(2, BuildOrder::Forward, protocol);
    let two_reverse = run_chain(2, BuildOrder::Reverse, protocol);
    let two_registration_order_invariant = registration_invariant(&two_forward, &two_reverse);
    let two_perturbation = run_perturbation(2, protocol);
    let two_joint_survived = predecessor_controls_survived
        && one_module_identity
        && chain_survived(&two_forward, &one_module)
        && two_registration_order_invariant
        && two_perturbation.recovered;

    let (five_forward, five_reverse, five_registration_order_invariant, five_perturbation) =
        if two_joint_survived {
            let forward = run_chain(5, BuildOrder::Forward, protocol);
            let reverse = run_chain(5, BuildOrder::Reverse, protocol);
            let invariant = registration_invariant(&forward, &reverse);
            let perturbation = run_perturbation(5, protocol);
            (
                Some(forward),
                Some(reverse),
                Some(invariant),
                Some(perturbation),
            )
        } else {
            (None, None, None, None)
        };
    let five_joint_survived = five_forward.as_ref().map(|forward| {
        chain_survived(forward, &one_module)
            && five_registration_order_invariant == Some(true)
            && five_perturbation
                .as_ref()
                .is_some_and(|perturbation| perturbation.recovered)
    });
    let first_failure = if !two_joint_survived {
        Some(first_failure(
            2,
            &two_forward,
            &one_module,
            &two_perturbation,
        ))
    } else if five_joint_survived == Some(false) {
        Some(first_failure(
            5,
            five_forward.as_ref().expect("five-joint run exists"),
            &one_module,
            five_perturbation
                .as_ref()
                .expect("five-joint perturbation exists"),
        ))
    } else {
        None
    };
    DevelopmentEvidence {
        protocol,
        predecessor_controls,
        predecessor_controls_survived,
        one_module,
        one_module_identity,
        two_forward,
        two_reverse,
        two_registration_order_invariant,
        two_perturbation,
        two_joint_survived,
        five_forward,
        five_reverse,
        five_registration_order_invariant,
        five_perturbation,
        five_joint_survived,
        five_rung_executed: two_joint_survived,
        first_failure,
    }
}

pub fn run_all(predecessor: &PredecessorBytes<'_>) -> Vec<(Arm, ProbeResult)> {
    run_all_with_protocol(
        predecessor,
        Protocol::RecursiveLearnerReturnBearingContinuation,
    )
}

pub fn run_all_with_protocol(
    predecessor: &PredecessorBytes<'_>,
    protocol: Protocol,
) -> Vec<(Arm, ProbeResult)> {
    let evidence = measure_with_protocol(predecessor, protocol);
    Arm::ALL
        .into_iter()
        .map(|arm| {
            let (outcome, survived, falsifier) = match arm {
                Arm::OneModuleIdentity => (
                    if evidence.one_module_identity {
                        "survived"
                    } else {
                        "falsified"
                    },
                    evidence.one_module_identity,
                    "the one-module product world differs from the established one-joint hand",
                ),
                Arm::TwoJointSerialComposition => (
                    if evidence.two_joint_survived {
                        "survived"
                    } else {
                        "falsified"
                    },
                    evidence.two_joint_survived,
                    "either anonymous component loses closed reflected control or shared work is unbounded",
                ),
                Arm::RegistrationOrderInvariance => (
                    if evidence.two_registration_order_invariant {
                        "survived"
                    } else {
                        "falsified"
                    },
                    evidence.two_registration_order_invariant,
                    "forward and reverse physical module registration change source-mapped behavior",
                ),
                Arm::ProximalChangeIsolation => (
                    if evidence.two_perturbation.recovered {
                        "survived"
                    } else {
                        "falsified"
                    },
                    evidence.two_perturbation.recovered,
                    "changing the proximal component destroys distal control or recovery",
                ),
                Arm::ConditionalFiveJoint => match evidence.five_joint_survived {
                    Some(true) => (
                        "survived",
                        true,
                        "one or more of five components loses control, invariance, recovery, or bounded work",
                    ),
                    Some(false) => (
                        "falsified",
                        false,
                        "one or more of five components loses control, invariance, recovery, or bounded work",
                    ),
                    None => (
                        "inconclusive",
                        false,
                        "the five-joint rung is stopped because the two-joint prerequisite failed",
                    ),
                },
            };
            let result = ProbeResult {
                schema: "developmental-hand-multi-joint/v1",
                arm: arm.id(),
                outcome,
                observations: serde_json::to_value(&evidence).expect("evidence serializes"),
                falsifier: (!survived).then(|| falsifier.to_string()),
                exact_replay: evidence.one_module.exact_replay
                    && evidence.two_forward.exact_replay
                    && evidence.two_perturbation.exact_replay
                    && evidence
                        .five_forward
                        .as_ref()
                        .is_none_or(|run| run.exact_replay)
                    && evidence
                        .five_perturbation
                        .as_ref()
                        .is_none_or(|run| run.exact_replay),
                naturally_quiescent: evidence.one_module.naturally_quiescent
                    && evidence.two_forward.naturally_quiescent
                    && evidence.two_perturbation.naturally_quiescent
                    && evidence
                        .five_forward
                        .as_ref()
                        .is_none_or(|run| run.naturally_quiescent)
                    && evidence
                        .five_perturbation
                        .as_ref()
                        .is_none_or(|run| run.naturally_quiescent),
            };
            (arm, result)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn predecessor() -> PredecessorBytes<'static> {
        PredecessorBytes {
            complete_one_joint: include_bytes!(
                "../../../campaigns/hand-activity-normalized-build-v1/artifacts/complete-one-joint.json"
            ),
            convergence: include_bytes!(
                "../../../campaigns/hand-activity-normalized-build-v1/convergence.toml"
            ),
        }
    }

    fn product_evidence() -> DevelopmentEvidence {
        measure_with_protocol(
            &predecessor(),
            Protocol::RecursiveLearnerCausalTopologyProductComposition,
        )
    }

    #[test]
    fn immediate_origin_product_is_falsified_by_one_joint_identity() {
        let evidence = measure_with_protocol(
            &predecessor(),
            Protocol::RecursiveLearnerCausalOriginProductComposition,
        );
        assert!(!evidence.one_module_identity);
        assert!(!evidence.two_joint_survived);
        assert!(!evidence.five_rung_executed);
    }

    #[test]
    fn causal_path_product_is_falsified_by_one_joint_identity() {
        let evidence = measure_with_protocol(
            &predecessor(),
            Protocol::RecursiveLearnerCausalPathProductComposition,
        );
        assert!(!evidence.one_module_identity);
        assert!(!evidence.two_joint_survived);
        assert!(!evidence.five_rung_executed);
    }

    #[test]
    fn one_module_matches_existing_hand() {
        let evidence = measure(&predecessor());
        assert!(evidence.one_module_identity, "{:#?}", evidence.one_module);
    }

    #[test]
    fn two_joint_serial_composition() {
        let evidence = measure(&predecessor());
        assert!(
            evidence.two_joint_survived || evidence.first_failure.is_some(),
            "a failed rung must retain its first physical break"
        );
    }

    #[test]
    fn registration_order_is_inert() {
        let evidence = measure(&predecessor());
        assert!(evidence.two_registration_order_invariant);
    }

    #[test]
    fn proximal_change_preserves_distal_control() {
        let evidence = measure(&predecessor());
        assert!(
            evidence.two_perturbation.recovered || evidence.first_failure.is_some(),
            "failed recovery must retain the earlier compositional break"
        );
    }

    #[test]
    fn five_joint_composition() {
        let evidence = measure(&predecessor());
        if evidence.two_joint_survived {
            assert!(evidence.five_joint_survived.is_some());
            assert!(evidence.five_rung_executed);
        } else {
            assert_eq!(evidence.five_joint_survived, None);
            assert!(!evidence.five_rung_executed);
        }
    }

    #[test]
    fn product_output_preserves_the_one_joint_identity() {
        let evidence = product_evidence();
        assert!(evidence.one_module_identity, "{:#?}", evidence.one_module);
    }

    #[test]
    fn product_output_closes_two_joints_before_five() {
        let evidence = product_evidence();
        assert!(
            evidence.two_joint_survived,
            "failure: {:#?}; joints: {:#?}",
            evidence.first_failure, evidence.two_forward.joints
        );
        assert!(evidence.five_rung_executed);
        assert_eq!(evidence.five_joint_survived, Some(true));
    }

    #[test]
    fn topology_ablation_removing_output_factorization_restores_step_zero_suppression() {
        let evidence = measure_with_protocol(
            &predecessor(),
            Protocol::RecursiveLearnerCausalTopologyOpportunityComposition,
        );
        assert!(evidence.one_module_identity, "{:#?}", evidence.one_module);
        assert!(!evidence.two_joint_survived);
        assert_eq!(
            evidence.first_failure,
            Some(FirstFailure {
                rung: 2,
                joint: Some(0),
                step: Some(0),
                stage: "output-choice",
                explanation: "an independently driven local motor produced no outward output"
                    .to_string(),
            })
        );
        assert!(!evidence.five_rung_executed);
        assert_eq!(evidence.five_joint_survived, None);
    }

    #[test]
    fn topology_ablation_removing_opportunity_factorization_restores_shared_reversal_asymmetry() {
        let evidence = measure_with_protocol(
            &predecessor(),
            Protocol::RecursiveLearnerCausalTopologyOutputComposition,
        );
        assert!(evidence.one_module_identity, "{:#?}", evidence.one_module);
        assert!(!evidence.two_joint_survived);
        assert_eq!(
            evidence.first_failure,
            Some(FirstFailure {
                rung: 2,
                joint: Some(1),
                step: Some(5),
                stage: "continued-arrow",
                explanation:
                    "the component emitted a different physical arrow from the one-module identity"
                        .to_string(),
            })
        );
        assert!(!evidence.five_rung_executed);
        assert_eq!(evidence.five_joint_survived, None);
    }
}
