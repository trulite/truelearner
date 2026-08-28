#![forbid(unsafe_code)]

use serde::Serialize;
use std::any::Any;
use std::collections::BTreeSet;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::str::FromStr;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use truelearner_core::{
    Checkpoint, Harness, HarnessBuilder, Input, Junction, JunctionId, Link, PhysicalEvent,
    Protocol, Run, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const LOWER: i16 = -4;
const UPPER: i16 = 4;
const PRIMARY_STEPS: usize = 16;
const DEVELOPMENT_STEPS: usize = 8;
const RECOVERY_STEPS: usize = 16;
const JUNCTION_CAPACITY: u32 = 16_384;
const LINK_CAPACITY: u32 = 65_536;
const WARM_LIMIT: Duration = Duration::from_secs(10);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    InheritedIntegrityControl,
    SyntheticMonolithicReference,
    TruthfulProprioceptionOnly,
    RecursiveSyntheticProvenance,
    TruthfulRecursiveComposition,
    ProvenanceMemoryFactorialLocalization,
}

impl Arm {
    pub const ALL: [Self; 6] = [
        Self::TruthfulRecursiveComposition,
        Self::InheritedIntegrityControl,
        Self::SyntheticMonolithicReference,
        Self::TruthfulProprioceptionOnly,
        Self::RecursiveSyntheticProvenance,
        Self::ProvenanceMemoryFactorialLocalization,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::InheritedIntegrityControl => "inherited-integrity-control",
            Self::SyntheticMonolithicReference => "synthetic-monolithic-reference",
            Self::TruthfulProprioceptionOnly => "truthful-proprioception-only",
            Self::RecursiveSyntheticProvenance => "recursive-synthetic-provenance",
            Self::TruthfulRecursiveComposition => "truthful-recursive-composition",
            Self::ProvenanceMemoryFactorialLocalization => {
                "provenance-memory-factorial-localization"
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

fn result(
    arm: Arm,
    outcome: &'static str,
    observations: serde_json::Value,
    falsifier: Option<String>,
    exact_replay: bool,
    naturally_quiescent: bool,
) -> ProbeResult {
    ProbeResult {
        schema: "developmental-hand-proprioception-decomposition/v1",
        arm: arm.id(),
        outcome,
        observations,
        falsifier,
        exact_replay,
        naturally_quiescent,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Provenance {
    Synthetic,
    Truthful,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum LearnerPhysics {
    Monolithic,
    Recursive,
}

impl LearnerPhysics {
    const fn protocol(self) -> Protocol {
        match self {
            Self::Monolithic => Protocol::SensorimotorSynthesis,
            Self::Recursive => Protocol::RecursiveLearnerConstruction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
struct FactorialCell {
    provenance: Provenance,
    learner_physics: LearnerPhysics,
}

impl FactorialCell {
    const SYNTHETIC_MONOLITHIC: Self = Self {
        provenance: Provenance::Synthetic,
        learner_physics: LearnerPhysics::Monolithic,
    };
    const TRUTHFUL_MONOLITHIC: Self = Self {
        provenance: Provenance::Truthful,
        learner_physics: LearnerPhysics::Monolithic,
    };
    const SYNTHETIC_RECURSIVE: Self = Self {
        provenance: Provenance::Synthetic,
        learner_physics: LearnerPhysics::Recursive,
    };
    const TRUTHFUL_RECURSIVE: Self = Self {
        provenance: Provenance::Truthful,
        learner_physics: LearnerPhysics::Recursive,
    };
}

#[derive(Clone)]
struct WorldCheckpoint {
    harness: Checkpoint,
    position: i16,
    pending: Vec<JunctionId>,
    sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum PhysicalStop {
    JunctionCapacity,
    LinkCapacity,
    WarmRuntime,
}

fn panic_text(payload: &(dyn Any + Send)) -> Option<&str> {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
}

fn classify_capacity(payload: &(dyn Any + Send)) -> Option<PhysicalStop> {
    match panic_text(payload) {
        Some("arena has no free junction slot") => Some(PhysicalStop::JunctionCapacity),
        Some("arena has no free link identity") => Some(PhysicalStop::LinkCapacity),
        _ => None,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
struct EventSummary {
    drive_incidence: u64,
    proposals: u64,
    traversals: u64,
    choices: u64,
    outputs: u64,
    return_scheduling: u64,
    return_admissions: u64,
    closure_observations: u64,
    constructions: u64,
    owner_writes: u64,
    owner_reads: u64,
    consequential_owner_reads: u64,
}

impl EventSummary {
    fn observe(&mut self, run: &Run) {
        for transition in &run.physical_trace {
            match transition.event {
                PhysicalEvent::DriveIncidence { .. } => self.drive_incidence += 1,
                PhysicalEvent::JunctionProposal { .. } | PhysicalEvent::Proposal { .. } => {
                    self.proposals += 1;
                }
                PhysicalEvent::QualifiedLocalTraversal { .. } => self.traversals += 1,
                PhysicalEvent::PathChosen { .. } | PhysicalEvent::CandidateSelection { .. } => {
                    self.choices += 1
                }
                PhysicalEvent::Output(_) => self.outputs += 1,
                PhysicalEvent::ReturnScheduling { .. } => self.return_scheduling += 1,
                PhysicalEvent::ReturnOriginAdmission { .. } => self.return_admissions += 1,
                PhysicalEvent::CausalClosureObserved { .. } => self.closure_observations += 1,
                PhysicalEvent::LearnerConstructed { .. } => self.constructions += 1,
                PhysicalEvent::LearnerConsequenceRecorded { .. } => self.owner_writes += 1,
                PhysicalEvent::LearnerCandidatePreference {
                    consequence_tick, ..
                } => {
                    self.owner_reads += 1;
                    if consequence_tick.is_some() {
                        self.consequential_owner_reads += 1;
                    }
                }
                _ => {}
            }
        }
    }

    fn merge(&mut self, other: &Self) {
        self.drive_incidence += other.drive_incidence;
        self.proposals += other.proposals;
        self.traversals += other.traversals;
        self.choices += other.choices;
        self.outputs += other.outputs;
        self.return_scheduling += other.return_scheduling;
        self.return_admissions += other.return_admissions;
        self.closure_observations += other.closure_observations;
        self.constructions += other.constructions;
        self.owner_writes += other.owner_writes;
        self.owner_reads += other.owner_reads;
        self.consequential_owner_reads += other.consequential_owner_reads;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct JointStep {
    index: usize,
    position_before: i16,
    position_after: i16,
    effort: [i32; 2],
    direction: i8,
    emitted_outputs: Vec<u64>,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    learners: usize,
    junctions: usize,
    links: usize,
    naturally_quiescent: bool,
    local_return_updates: u64,
    comparisons: u64,
    scans: u64,
    events: EventSummary,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
struct TrialAggregate {
    steps: usize,
    changed_steps: usize,
    directions: BTreeSet<i8>,
    reached_lower: bool,
    reached_upper: bool,
    escaped_lower: bool,
    escaped_upper: bool,
    maximum_same_direction_run: usize,
    final_position: i16,
    learners: usize,
    junctions: usize,
    links: usize,
    comparisons: u64,
    scans: u64,
    events: EventSummary,
}

impl TrialAggregate {
    fn from_history(history: &[JointStep]) -> Self {
        let mut aggregate = Self {
            steps: history.len(),
            final_position: history.last().map_or(0, |step| step.position_after),
            learners: history.last().map_or(0, |step| step.learners),
            junctions: history.last().map_or(0, |step| step.junctions),
            links: history.last().map_or(0, |step| step.links),
            ..Self::default()
        };
        let mut previous_direction = 0;
        let mut direction_run = 0;
        for step in history {
            if step.direction != 0 {
                aggregate.changed_steps += 1;
                aggregate.directions.insert(step.direction);
                if step.direction == previous_direction {
                    direction_run += 1;
                } else {
                    previous_direction = step.direction;
                    direction_run = 1;
                }
                aggregate.maximum_same_direction_run =
                    aggregate.maximum_same_direction_run.max(direction_run);
            }
            aggregate.reached_lower |= step.reached_lower;
            aggregate.reached_upper |= step.reached_upper;
            aggregate.escaped_lower |= step.escaped_lower;
            aggregate.escaped_upper |= step.escaped_upper;
            aggregate.comparisons += step.comparisons;
            aggregate.scans += step.scans;
            aggregate.events.merge(&step.events);
        }
        aggregate
    }

    fn closes_joint(&self) -> bool {
        self.steps == PRIMARY_STEPS
            && self.directions.len() == 2
            && self.reached_lower
            && self.reached_upper
            && self.escaped_lower
            && self.escaped_upper
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct CompletedTrial {
    history: Vec<JointStep>,
    aggregate: TrialAggregate,
    exact_replay: bool,
    naturally_quiescent: bool,
    elapsed_millis: u128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct StoppedTrial {
    completed_prefix: Vec<JointStep>,
    aggregate: TrialAggregate,
    stop: PhysicalStop,
    stopped_step: usize,
    exact_replay: bool,
    naturally_quiescent: bool,
    elapsed_millis: u128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
enum TrialOutcome {
    Completed(CompletedTrial),
    Stopped(StoppedTrial),
}

impl TrialOutcome {
    fn aggregate(&self) -> &TrialAggregate {
        match self {
            Self::Completed(trial) => &trial.aggregate,
            Self::Stopped(trial) => &trial.aggregate,
        }
    }

    fn history(&self) -> &[JointStep] {
        match self {
            Self::Completed(trial) => &trial.history,
            Self::Stopped(trial) => &trial.completed_prefix,
        }
    }

    fn stop(&self) -> Option<PhysicalStop> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(trial) => Some(trial.stop),
        }
    }

    fn exact_replay(&self) -> bool {
        match self {
            Self::Completed(trial) => trial.exact_replay,
            Self::Stopped(trial) => trial.exact_replay,
        }
    }

    fn naturally_quiescent(&self) -> bool {
        match self {
            Self::Completed(trial) => trial.naturally_quiescent,
            Self::Stopped(trial) => trial.naturally_quiescent,
        }
    }

    fn completed(&self) -> Option<&CompletedTrial> {
        match self {
            Self::Completed(trial) => Some(trial),
            Self::Stopped(_) => None,
        }
    }
}

struct JointWorld {
    harness: Harness,
    cell: FactorialCell,
    sensors: Vec<JunctionId>,
    sensor_physical: Vec<u64>,
    motors: [JunctionId; 2],
    motor_physical: [u64; 2],
    outcomes: [JunctionId; 2],
    outcome_physical: [u64; 2],
    position: i16,
    pending: Vec<JunctionId>,
    sequence: u64,
}

impl JointWorld {
    fn new(cell: FactorialCell) -> Self {
        Self::with_capacity(cell, JUNCTION_CAPACITY, LINK_CAPACITY)
    }

    fn with_capacity(cell: FactorialCell, junction_capacity: u32, link_capacity: u32) -> Self {
        let mut builder =
            HarnessBuilder::with_capacity(junction_capacity, link_capacity, OUTWARD_REGION);
        builder.set_protocol(cell.learner_physics.protocol());
        builder.set_physical_tracing(true);
        let anchor = add_junction(&mut builder, 90_000, 10_000, 0, 99);
        let sensor_physical = (0..9)
            .map(|channel| 10_000 + channel as u64)
            .collect::<Vec<_>>();
        let sensors = sensor_physical
            .iter()
            .map(|physical| {
                let sensor = add_junction(&mut builder, *physical, 10, 0, 1);
                add_link(&mut builder, anchor, sensor);
                sensor
            })
            .collect::<Vec<_>>();
        let motor_physical = [20_000, 20_001];
        let motors = [
            add_junction(&mut builder, motor_physical[0], 9, 0, 2),
            add_junction(&mut builder, motor_physical[1], 11, 0, 2),
        ];
        let sinks = [
            add_junction(&mut builder, 30_000, 9, OUTWARD_REGION, 1),
            add_junction(&mut builder, 30_001, 11, OUTWARD_REGION, 1),
        ];
        for index in 0..2 {
            add_link(&mut builder, motors[index], sinks[index]);
        }
        let outcome_physical = [40_000, 40_001];
        let outcomes = [
            add_junction(&mut builder, outcome_physical[0], 1_000, 0, 1),
            add_junction(&mut builder, outcome_physical[1], 1_001, 0, 1),
        ];
        for outcome in outcomes {
            add_link(&mut builder, anchor, outcome);
        }
        for index in 0..2 {
            builder.set_outcome_source_for_output(motors[index], outcomes[index]);
        }
        Self {
            harness: builder.build(),
            cell,
            sensors,
            sensor_physical,
            motors,
            motor_physical,
            outcomes,
            outcome_physical,
            position: 0,
            pending: Vec::new(),
            sequence: 0,
        }
    }

    fn checkpoint(&self) -> WorldCheckpoint {
        WorldCheckpoint {
            harness: self.harness.save().expect("joint checkpoint saves"),
            position: self.position,
            pending: self.pending.clone(),
            sequence: self.sequence,
        }
    }

    fn restore(cell: FactorialCell, checkpoint: WorldCheckpoint) -> Self {
        let mut world = Self::new(cell);
        world.harness = Harness::restore(checkpoint.harness).expect("joint checkpoint restores");
        world.position = checkpoint.position;
        world.pending = checkpoint.pending;
        world.sequence = checkpoint.sequence;
        world
    }

    fn active_channels(&self) -> Vec<usize> {
        let mut active = vec![2];
        if self.position < 0 {
            active.push(0);
        } else if self.position > 0 {
            active.push(1);
        }
        if self.position == LOWER {
            active.push(3);
        }
        if self.position == UPPER {
            active.push(4);
        }
        active
    }

    fn sensor_origin(&self, channel: usize) -> u64 {
        match self.cell.provenance {
            Provenance::Synthetic => 70_000 + self.sequence * 1_000 + channel as u64,
            Provenance::Truthful => self.sensor_physical[channel],
        }
    }

    fn motor_origin(&self, index: usize) -> u64 {
        match self.cell.provenance {
            Provenance::Synthetic => 75_000 + self.sequence * 1_000 + index as u64,
            Provenance::Truthful => self.outcome_physical[index],
        }
    }

    fn pending_origin(&self, target: JunctionId, index: usize) -> u64 {
        match self.cell.provenance {
            Provenance::Synthetic => 60_000 + self.sequence * 100 + index as u64,
            Provenance::Truthful => self
                .outcomes
                .iter()
                .position(|outcome| *outcome == target)
                .map_or(0, |direction| self.outcome_physical[direction]),
        }
    }

    fn checked_send(&mut self, inputs: &[Input]) -> Result<Run, PhysicalStop> {
        match catch_unwind(AssertUnwindSafe(|| self.harness.send(inputs))) {
            Ok(run) => Ok(run),
            Err(payload) => match classify_capacity(payload.as_ref()) {
                Some(stop) => Err(stop),
                None => resume_unwind(payload),
            },
        }
    }

    fn step(&mut self) -> Result<JointStep, PhysicalStop> {
        let position_before = self.position;
        let mut quiet = true;
        let mut outputs = Vec::new();
        let mut events = EventSummary::default();
        let mut local_return_updates = 0;
        let mut comparisons = 0;
        let mut scans = 0;

        if !self.pending.is_empty() {
            let tick = self.harness.read().clock.tick.saturating_add(1);
            let pending = self.pending.clone();
            let inputs = pending
                .iter()
                .enumerate()
                .map(|(index, target)| {
                    physical_input(*target, tick, self.pending_origin(*target, index))
                })
                .collect::<Vec<_>>();
            let returned = self.checked_send(&inputs)?;
            self.pending.clear();
            quiet &= returned.naturally_quiescent;
            events.observe(&returned);
            local_return_updates += returned.work.local_return_updates;
            comparisons += returned.execution_cost.comparisons;
            scans += returned.execution_cost.scans;
            outputs.extend(returned.outputs);
        }

        let tick = self.harness.read().clock.tick.saturating_add(1);
        let mut inputs = self
            .active_channels()
            .into_iter()
            .map(|channel| physical_input(self.sensors[channel], tick, self.sensor_origin(channel)))
            .collect::<Vec<_>>();
        for index in 0..2 {
            inputs.push(physical_input(
                self.motors[index],
                tick.saturating_add(2),
                self.motor_origin(index),
            ));
        }
        let moved = self.checked_send(&inputs)?;
        quiet &= moved.naturally_quiescent;
        events.observe(&moved);
        local_return_updates += moved.work.local_return_updates;
        comparisons += moved.execution_cost.comparisons;
        scans += moved.execution_cost.scans;
        outputs.extend(moved.outputs);

        let mut effort = [0_i32; 2];
        let emitted_outputs = outputs
            .iter()
            .map(|output| output.from_physical)
            .collect::<Vec<_>>();
        for output in outputs {
            if output.from_physical == self.motor_physical[0] {
                effort[0] = effort[0].saturating_add(output.impulse.abs());
            } else if output.from_physical == self.motor_physical[1] {
                effort[1] = effort[1].saturating_add(output.impulse.abs());
            }
        }
        let direction = match effort[1].cmp(&effort[0]) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };
        let next = self
            .position
            .saturating_add(i16::from(direction))
            .clamp(LOWER, UPPER);
        let escaped_lower = self.position == LOWER && next > LOWER;
        let escaped_upper = self.position == UPPER && next < UPPER;
        self.position = next;
        let reached_lower = next == LOWER && position_before != LOWER;
        let reached_upper = next == UPPER && position_before != UPPER;
        self.pending = if next != position_before {
            vec![if direction < 0 {
                self.outcomes[0]
            } else {
                self.outcomes[1]
            }]
        } else {
            Vec::new()
        };
        let observation = self.harness.read();
        let index = usize::try_from(self.sequence).unwrap_or(usize::MAX);
        self.sequence = self.sequence.saturating_add(1);
        Ok(JointStep {
            index,
            position_before,
            position_after: next,
            effort,
            direction,
            emitted_outputs,
            reached_lower,
            reached_upper,
            escaped_lower,
            escaped_upper,
            learners: observation.learners.len(),
            junctions: observation.junctions.len(),
            links: observation.links.len(),
            naturally_quiescent: quiet,
            local_return_updates,
            comparisons,
            scans,
            events,
        })
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

fn physical_input(target: JunctionId, tick: i64, origin_physical: u64) -> Input {
    Input {
        arrival_tick: tick,
        phase: 0,
        origin_physical,
        target,
        impulse: 1,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Execution {
    history: Vec<JointStep>,
    stop: Option<PhysicalStop>,
    stopped_step: usize,
    canonical_bytes: Vec<u8>,
}

fn execute(world: &mut JointWorld, steps: usize) -> Execution {
    let mut history = Vec::new();
    let mut stop = None;
    for step in 0..steps {
        match world.step() {
            Ok(observed) => history.push(observed),
            Err(reason) => {
                stop = Some(reason);
                return Execution {
                    history,
                    stop,
                    stopped_step: step,
                    canonical_bytes: world
                        .harness
                        .save()
                        .and_then(|checkpoint| checkpoint.canonical_bytes())
                        .expect("stopped checkpoint encodes"),
                };
            }
        }
    }
    Execution {
        history,
        stop,
        stopped_step: steps,
        canonical_bytes: world
            .harness
            .save()
            .and_then(|checkpoint| checkpoint.canonical_bytes())
            .expect("completed checkpoint encodes"),
    }
}

fn trial_with_capacity(cell: FactorialCell, junctions: u32, links: u32) -> TrialOutcome {
    let started = Instant::now();
    let mut direct = JointWorld::with_capacity(cell, junctions, links);
    let checkpoint = direct.checkpoint();
    let direct_execution = execute(&mut direct, PRIMARY_STEPS);
    let mut replay = if junctions == JUNCTION_CAPACITY && links == LINK_CAPACITY {
        JointWorld::restore(cell, checkpoint)
    } else {
        let mut restored = JointWorld::with_capacity(cell, junctions, links);
        restored.harness = Harness::restore(checkpoint.harness).expect("checkpoint restores");
        restored.position = checkpoint.position;
        restored.pending = checkpoint.pending;
        restored.sequence = checkpoint.sequence;
        restored
    };
    let replay_execution = execute(&mut replay, PRIMARY_STEPS);
    let elapsed = started.elapsed();
    let exact_replay = direct_execution == replay_execution;
    let quiet = direct_execution
        .history
        .iter()
        .all(|step| step.naturally_quiescent);
    let aggregate = TrialAggregate::from_history(&direct_execution.history);
    if elapsed >= WARM_LIMIT {
        return TrialOutcome::Stopped(StoppedTrial {
            completed_prefix: direct_execution.history,
            aggregate,
            stop: PhysicalStop::WarmRuntime,
            stopped_step: direct_execution.stopped_step,
            exact_replay,
            naturally_quiescent: quiet,
            elapsed_millis: elapsed.as_millis(),
        });
    }
    match direct_execution.stop {
        Some(stop) => TrialOutcome::Stopped(StoppedTrial {
            completed_prefix: direct_execution.history,
            aggregate,
            stop,
            stopped_step: direct_execution.stopped_step,
            exact_replay,
            naturally_quiescent: quiet,
            elapsed_millis: elapsed.as_millis(),
        }),
        None => TrialOutcome::Completed(CompletedTrial {
            history: direct_execution.history,
            aggregate,
            exact_replay,
            naturally_quiescent: quiet,
            elapsed_millis: elapsed.as_millis(),
        }),
    }
}

fn trial(cell: FactorialCell) -> TrialOutcome {
    trial_with_capacity(cell, JUNCTION_CAPACITY, LINK_CAPACITY)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct PerturbationTrial {
    development_steps: usize,
    imposed_position: i16,
    recovery_steps: usize,
    left_upper: bool,
    reached_lower: bool,
    left_lower: bool,
    both_signs: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    stop: Option<PhysicalStop>,
}

fn execute_perturbation(cell: FactorialCell, imposed: i16) -> PerturbationTrial {
    fn once(cell: FactorialCell, imposed: i16) -> (Execution, Execution) {
        let mut world = JointWorld::new(cell);
        let development = execute(&mut world, DEVELOPMENT_STEPS);
        if development.stop.is_none() {
            world.position = imposed;
        }
        let recovery = if development.stop.is_none() {
            execute(&mut world, RECOVERY_STEPS)
        } else {
            Execution {
                history: Vec::new(),
                stop: development.stop,
                stopped_step: 0,
                canonical_bytes: development.canonical_bytes.clone(),
            }
        };
        (development, recovery)
    }
    let started = Instant::now();
    let (development, recovery) = once(cell, imposed);
    let (replayed_development, replayed_recovery) = once(cell, imposed);
    let elapsed = started.elapsed();
    let exact_replay = development == replayed_development && recovery == replayed_recovery;
    let aggregate = TrialAggregate::from_history(&recovery.history);
    let stop = if elapsed >= WARM_LIMIT {
        Some(PhysicalStop::WarmRuntime)
    } else {
        development.stop.or(recovery.stop)
    };
    PerturbationTrial {
        development_steps: DEVELOPMENT_STEPS,
        imposed_position: imposed,
        recovery_steps: RECOVERY_STEPS,
        left_upper: recovery
            .history
            .iter()
            .any(|step| step.position_before == UPPER && step.position_after < UPPER),
        reached_lower: aggregate.reached_lower,
        left_lower: aggregate.escaped_lower,
        both_signs: aggregate.directions.len() == 2,
        exact_replay,
        naturally_quiescent: development
            .history
            .iter()
            .chain(&recovery.history)
            .all(|step| step.naturally_quiescent),
        stop,
    }
}

impl PerturbationTrial {
    fn recovered(&self) -> bool {
        self.stop.is_none()
            && self.left_upper
            && self.reached_lower
            && self.left_lower
            && self.both_signs
            && self.exact_replay
            && self.naturally_quiescent
    }
}

#[derive(Clone, Debug, Serialize)]
struct IntegrityEvidence {
    synthesis: Vec<(&'static str, &'static str)>,
    recursive: Vec<(&'static str, &'static str)>,
    old_reference_exact: bool,
    survived: bool,
}

fn old_reference_exact(reference: &sensorimotor_synthesis_ladder::ProbeResult) -> bool {
    let observed = &reference.observations["observations"];
    reference.outcome == "falsified"
        && observed["steps"] == 16
        && observed["changed_steps"] == 12
        && observed["directions"] == serde_json::json!([-1, 1])
        && observed["reached_lower"] == false
        && observed["reached_upper"] == false
        && observed["escaped_lower"] == false
        && observed["escaped_upper"] == false
        && reference.exact_replay
        && reference.naturally_quiescent
}

fn integrity() -> IntegrityEvidence {
    use recursive_learner_proprioceptive_control::{Arm as RecursiveArm, run as recursive_run};
    use sensorimotor_synthesis_ladder::{Arm as SynthesisArm, run as synthesis_run};
    let synthesis_prefix = SynthesisArm::ALL[..=4]
        .iter()
        .copied()
        .map(|arm| (arm, synthesis_run(arm)))
        .collect::<Vec<_>>();
    let one_joint_failed = synthesis_prefix
        .last()
        .is_some_and(|(_, result)| result.outcome == "falsified");
    let old_reference_exact = synthesis_prefix
        .last()
        .map(|(_, result)| old_reference_exact(result))
        .expect("the inherited prefix includes the one-joint reference");
    let mut synthesis = synthesis_prefix
        .into_iter()
        .map(|(arm, result)| (arm.id(), result.outcome))
        .collect::<Vec<_>>();
    synthesis.extend(SynthesisArm::ALL[5..].iter().copied().map(|arm| {
        if one_joint_failed {
            (arm.id(), "inconclusive")
        } else {
            (arm.id(), synthesis_run(arm).outcome)
        }
    }));
    let recursive = RecursiveArm::ALL
        .into_iter()
        .map(|arm| (arm.id(), recursive_run(arm).outcome))
        .collect::<Vec<_>>();
    let expected_synthesis = [
        "survived",
        "survived",
        "survived",
        "survived",
        "falsified",
        "inconclusive",
        "inconclusive",
        "inconclusive",
        "inconclusive",
        "inconclusive",
        "inconclusive",
    ];
    let expected_recursive = ["survived", "falsified", "survived", "survived"];
    let survived = synthesis
        .iter()
        .map(|(_, outcome)| *outcome)
        .eq(expected_synthesis)
        && recursive
            .iter()
            .map(|(_, outcome)| *outcome)
            .eq(expected_recursive)
        && old_reference_exact;
    IntegrityEvidence {
        synthesis,
        recursive,
        old_reference_exact,
        survived,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum TransitionStage {
    ExternalIncidence,
    ProposalOrTraversal,
    CandidateAdmission,
    MotorOutput,
    WorldChange,
    ReturnAdmission,
    CausalClosure,
    OwnerConsequenceWrite,
    OwnerConsequenceRead,
    Continuation,
    Reversal,
    Release,
    PhysicalCost,
    Complete,
}

#[derive(Clone, Debug, Serialize)]
struct FactorialLocalization {
    provenance_effect_monolithic: bool,
    provenance_effect_recursive: bool,
    recursion_effect_synthetic: bool,
    recursion_effect_truthful: bool,
    interaction_observed: bool,
    first_synthetic_provenance_difference: Option<usize>,
    first_truthful_provenance_difference: Option<usize>,
    earliest_incomplete_transition: TransitionStage,
    explanation: &'static str,
}

fn first_difference(left: &TrialOutcome, right: &TrialOutcome) -> Option<usize> {
    left.history()
        .iter()
        .zip(right.history())
        .position(|(left, right)| {
            left.position_after != right.position_after
                || left.direction != right.direction
                || left.emitted_outputs != right.emitted_outputs
        })
        .or_else(|| {
            (left.stop() != right.stop()).then_some(left.history().len().min(right.history().len()))
        })
}

fn differs(left: &TrialOutcome, right: &TrialOutcome) -> bool {
    first_difference(left, right).is_some()
        || left.aggregate().final_position != right.aggregate().final_position
        || left.aggregate().directions != right.aggregate().directions
}

fn incomplete_transition(complete: &TrialOutcome) -> (TransitionStage, &'static str) {
    if complete.stop().is_some() {
        return (
            TransitionStage::PhysicalCost,
            "the complete candidate hit a fixed physical-cost boundary",
        );
    }
    let aggregate = complete.aggregate();
    let events = &aggregate.events;
    if events.drive_incidence == 0 {
        (TransitionStage::ExternalIncidence, "no external incidence")
    } else if events.proposals == 0 && events.traversals == 0 {
        (
            TransitionStage::ProposalOrTraversal,
            "incidence did not produce local proposal or traversal",
        )
    } else if events.choices == 0 {
        (
            TransitionStage::CandidateAdmission,
            "paths existed without candidate admission",
        )
    } else if aggregate.changed_steps == 0 {
        (TransitionStage::MotorOutput, "no physical motor change")
    } else if events.return_scheduling == 0 && events.return_admissions == 0 {
        (
            TransitionStage::ReturnAdmission,
            "movement did not admit a physical return",
        )
    } else if events.constructions == 0 {
        (TransitionStage::CausalClosure, "no causal owner formed")
    } else if events.owner_writes == 0 {
        (
            TransitionStage::OwnerConsequenceWrite,
            "the owner received no private consequence write",
        )
    } else if events.consequential_owner_reads == 0 {
        (
            TransitionStage::OwnerConsequenceRead,
            "later selection did not read private consequence",
        )
    } else if aggregate.maximum_same_direction_run < 4 {
        (
            TransitionStage::Continuation,
            "owner-local reads did not sustain boundary travel",
        )
    } else if aggregate.directions.len() < 2 {
        (
            TransitionStage::Reversal,
            "the opposite sign did not emerge",
        )
    } else if (aggregate.reached_lower && !aggregate.escaped_lower)
        || (aggregate.reached_upper && !aggregate.escaped_upper)
    {
        (TransitionStage::Release, "a reached limit was not left")
    } else if !aggregate.reached_lower || !aggregate.reached_upper {
        (
            TransitionStage::WorldChange,
            "both signs occurred without reaching both boundaries",
        )
    } else {
        (TransitionStage::Complete, "the primary trace closed")
    }
}

#[derive(Clone, Debug, Serialize)]
struct FactorialEvidence {
    integrity: IntegrityEvidence,
    synthetic_monolithic: TrialOutcome,
    truthful_monolithic: TrialOutcome,
    synthetic_recursive: TrialOutcome,
    truthful_recursive: TrialOutcome,
    perturbation: Option<PerturbationTrial>,
    localization: FactorialLocalization,
}

fn measure_factorial() -> FactorialEvidence {
    let truthful_recursive = trial(FactorialCell::TRUTHFUL_RECURSIVE);
    let (integrity, synthetic_monolithic, truthful_monolithic, synthetic_recursive) =
        std::thread::scope(|scope| {
            let integrity = scope.spawn(integrity);
            let synthetic_monolithic = scope.spawn(|| trial(FactorialCell::SYNTHETIC_MONOLITHIC));
            let truthful_monolithic = scope.spawn(|| trial(FactorialCell::TRUTHFUL_MONOLITHIC));
            let synthetic_recursive = scope.spawn(|| trial(FactorialCell::SYNTHETIC_RECURSIVE));
            (
                integrity.join().expect("integrity measurement completes"),
                synthetic_monolithic
                    .join()
                    .expect("synthetic-monolithic measurement completes"),
                truthful_monolithic
                    .join()
                    .expect("truthful-monolithic measurement completes"),
                synthetic_recursive
                    .join()
                    .expect("synthetic-recursive measurement completes"),
            )
        });
    let perturbation = truthful_recursive
        .completed()
        .filter(|trial| trial.aggregate.closes_joint())
        .map(|_| execute_perturbation(FactorialCell::TRUTHFUL_RECURSIVE, UPPER));
    let provenance_effect_monolithic = differs(&synthetic_monolithic, &truthful_monolithic);
    let provenance_effect_recursive = differs(&synthetic_recursive, &truthful_recursive);
    let recursion_effect_synthetic = differs(&synthetic_monolithic, &synthetic_recursive);
    let recursion_effect_truthful = differs(&truthful_monolithic, &truthful_recursive);
    let (earliest_incomplete_transition, explanation) = incomplete_transition(&truthful_recursive);
    let localization = FactorialLocalization {
        provenance_effect_monolithic,
        provenance_effect_recursive,
        recursion_effect_synthetic,
        recursion_effect_truthful,
        interaction_observed: provenance_effect_monolithic != provenance_effect_recursive
            || recursion_effect_synthetic != recursion_effect_truthful,
        first_synthetic_provenance_difference: first_difference(
            &synthetic_monolithic,
            &synthetic_recursive,
        ),
        first_truthful_provenance_difference: first_difference(
            &truthful_monolithic,
            &truthful_recursive,
        ),
        earliest_incomplete_transition,
        explanation,
    };
    FactorialEvidence {
        integrity,
        synthetic_monolithic,
        truthful_monolithic,
        synthetic_recursive,
        truthful_recursive,
        perturbation,
        localization,
    }
}

static EVIDENCE: OnceLock<FactorialEvidence> = OnceLock::new();

fn evidence() -> &'static FactorialEvidence {
    EVIDENCE.get_or_init(measure_factorial)
}

fn reference_matches(outcome: &TrialOutcome) -> bool {
    let aggregate = outcome.aggregate();
    outcome.stop().is_none()
        && aggregate.steps == 16
        && aggregate.changed_steps == 12
        && aggregate.directions == BTreeSet::from([-1, 1])
        && !aggregate.reached_lower
        && !aggregate.reached_upper
        && !aggregate.escaped_lower
        && !aggregate.escaped_upper
        && outcome.exact_replay()
        && outcome.naturally_quiescent()
}

fn inherited_result(evidence: &FactorialEvidence) -> ProbeResult {
    result(
        Arm::InheritedIntegrityControl,
        if evidence.integrity.survived {
            "survived"
        } else {
            "falsified"
        },
        serde_json::to_value(&evidence.integrity).expect("integrity serializes"),
        (!evidence.integrity.survived)
            .then(|| "an inherited classification or old reference changed".to_string()),
        evidence.integrity.survived,
        evidence.integrity.survived,
    )
}

fn reference_result(evidence: &FactorialEvidence) -> ProbeResult {
    if !evidence.integrity.survived {
        return result(
            Arm::SyntheticMonolithicReference,
            "inconclusive",
            serde_json::json!({"integrity": "failed"}),
            Some("the integrity prerequisite failed".to_string()),
            false,
            false,
        );
    }
    let survived = reference_matches(&evidence.synthetic_monolithic);
    result(
        Arm::SyntheticMonolithicReference,
        if survived { "survived" } else { "falsified" },
        serde_json::to_value(&evidence.synthetic_monolithic).expect("reference serializes"),
        (!survived).then(|| "the common synthetic-monolithic cell drifted from 12/16".to_string()),
        evidence.synthetic_monolithic.exact_replay(),
        evidence.synthetic_monolithic.naturally_quiescent(),
    )
}

fn truthful_only_result(evidence: &FactorialEvidence) -> ProbeResult {
    let changed = differs(
        &evidence.synthetic_monolithic,
        &evidence.truthful_monolithic,
    );
    let survived = evidence.integrity.survived
        && changed
        && evidence.truthful_monolithic.stop().is_none()
        && evidence.truthful_monolithic.exact_replay()
        && evidence.truthful_monolithic.naturally_quiescent();
    let falsifier = if !evidence.integrity.survived {
        "the integrity prerequisite failed"
    } else if evidence.truthful_monolithic.stop().is_some() {
        "truthful proprioception reached a fixed physical-cost boundary"
    } else if !changed {
        "truthful proprioception did not change the synthetic trajectory"
    } else {
        "truthful proprioception lost replay or quiescence"
    };
    result(
        Arm::TruthfulProprioceptionOnly,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "synthetic": evidence.synthetic_monolithic,
            "truthful": evidence.truthful_monolithic,
            "trajectory_changed": changed,
        }),
        (!survived).then(|| falsifier.to_string()),
        evidence.truthful_monolithic.exact_replay(),
        evidence.truthful_monolithic.naturally_quiescent(),
    )
}

fn recursive_synthetic_result(evidence: &FactorialEvidence) -> ProbeResult {
    let survived = evidence.integrity.survived
        && evidence.synthetic_recursive.stop().is_none()
        && evidence.synthetic_recursive.exact_replay()
        && evidence.synthetic_recursive.naturally_quiescent();
    result(
        Arm::RecursiveSyntheticProvenance,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "synthetic_monolithic": evidence.synthetic_monolithic,
            "synthetic_recursive": evidence.synthetic_recursive,
            "trajectory_changed": differs(&evidence.synthetic_monolithic, &evidence.synthetic_recursive),
            "constructed_learners": evidence.synthetic_recursive.aggregate().learners,
        }),
        (!survived).then(|| {
            "the synthetic-recursive diagnostic hit cost, replay, quiescence, or integrity failure"
                .to_string()
        }),
        evidence.synthetic_recursive.exact_replay(),
        evidence.synthetic_recursive.naturally_quiescent(),
    )
}

fn composition_result(evidence: &FactorialEvidence) -> ProbeResult {
    let primary_closed = evidence
        .truthful_recursive
        .completed()
        .is_some_and(|trial| trial.aggregate.closes_joint());
    let recovered = evidence
        .perturbation
        .as_ref()
        .is_some_and(PerturbationTrial::recovered);
    let survived = evidence.integrity.survived
        && primary_closed
        && recovered
        && evidence.truthful_recursive.exact_replay()
        && evidence.truthful_recursive.naturally_quiescent();
    let falsifier = if evidence.truthful_recursive.stop().is_some() {
        "the complete candidate hit a fixed physical-cost boundary"
    } else if !primary_closed {
        "the complete candidate did not reach and leave both limits in sixteen steps"
    } else if !recovered {
        "the primary survivor did not recover from the fixed perturbation"
    } else {
        "the complete candidate lost integrity, replay, or quiescence"
    };
    result(
        Arm::TruthfulRecursiveComposition,
        if survived { "survived" } else { "falsified" },
        serde_json::json!({
            "primary": evidence.truthful_recursive,
            "perturbation": evidence.perturbation,
            "primary_closed": primary_closed,
            "perturbation_recovered": recovered,
        }),
        (!survived).then(|| falsifier.to_string()),
        evidence.truthful_recursive.exact_replay()
            && evidence
                .perturbation
                .as_ref()
                .is_none_or(|trial| trial.exact_replay),
        evidence.truthful_recursive.naturally_quiescent()
            && evidence
                .perturbation
                .as_ref()
                .is_none_or(|trial| trial.naturally_quiescent),
    )
}

fn localization_result(evidence: &FactorialEvidence) -> ProbeResult {
    let survived = evidence.integrity.survived
        && evidence.synthetic_monolithic.exact_replay()
        && evidence.truthful_monolithic.exact_replay()
        && evidence.synthetic_recursive.exact_replay()
        && evidence.truthful_recursive.exact_replay();
    result(
        Arm::ProvenanceMemoryFactorialLocalization,
        if survived { "survived" } else { "inconclusive" },
        serde_json::json!({
            "localization": evidence.localization,
            "synthetic_monolithic": evidence.synthetic_monolithic.aggregate(),
            "truthful_monolithic": evidence.truthful_monolithic.aggregate(),
            "synthetic_recursive": evidence.synthetic_recursive.aggregate(),
            "truthful_recursive": evidence.truthful_recursive.aggregate(),
            "stops": {
                "synthetic_monolithic": evidence.synthetic_monolithic.stop(),
                "truthful_monolithic": evidence.truthful_monolithic.stop(),
                "synthetic_recursive": evidence.synthetic_recursive.stop(),
                "truthful_recursive": evidence.truthful_recursive.stop(),
            },
        }),
        (!survived).then(|| "factorial lineage or replay was not interpretable".to_string()),
        survived,
        [
            &evidence.synthetic_monolithic,
            &evidence.truthful_monolithic,
            &evidence.synthetic_recursive,
            &evidence.truthful_recursive,
        ]
        .into_iter()
        .all(TrialOutcome::naturally_quiescent),
    )
}

pub fn run(arm: Arm) -> ProbeResult {
    let measured = evidence();
    match arm {
        Arm::InheritedIntegrityControl => inherited_result(measured),
        Arm::SyntheticMonolithicReference => reference_result(measured),
        Arm::TruthfulProprioceptionOnly => truthful_only_result(measured),
        Arm::RecursiveSyntheticProvenance => recursive_synthetic_result(measured),
        Arm::TruthfulRecursiveComposition => composition_result(measured),
        Arm::ProvenanceMemoryFactorialLocalization => localization_result(measured),
    }
}

pub fn run_all() -> Vec<(Arm, ProbeResult)> {
    Arm::ALL.into_iter().map(|arm| (arm, run(arm))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_cells_change_exactly_one_axis() {
        assert_eq!(
            FactorialCell::SYNTHETIC_MONOLITHIC.provenance,
            FactorialCell::SYNTHETIC_RECURSIVE.provenance
        );
        assert_ne!(
            FactorialCell::SYNTHETIC_MONOLITHIC.learner_physics,
            FactorialCell::SYNTHETIC_RECURSIVE.learner_physics
        );
        assert_eq!(
            FactorialCell::SYNTHETIC_MONOLITHIC.learner_physics,
            FactorialCell::TRUTHFUL_MONOLITHIC.learner_physics
        );
        assert_ne!(
            FactorialCell::SYNTHETIC_MONOLITHIC.provenance,
            FactorialCell::TRUTHFUL_MONOLITHIC.provenance
        );
    }

    #[test]
    fn synthetic_monolithic_reference() {
        let observed = run(Arm::SyntheticMonolithicReference);
        assert_eq!(observed.outcome, "survived", "{observed:#?}");
    }

    #[test]
    fn known_capacity_exhaustion_is_a_physical_stop() {
        for cell in [
            FactorialCell::SYNTHETIC_MONOLITHIC,
            FactorialCell::TRUTHFUL_MONOLITHIC,
        ] {
            let observed = trial_with_capacity(cell, 64, 64);
            assert!(matches!(
                observed.stop(),
                Some(PhysicalStop::JunctionCapacity | PhysicalStop::LinkCapacity)
            ));
            assert!(observed.exact_replay());
        }
    }

    #[test]
    fn factorial_arm_classifications_follow_frozen_predicates() {
        let measured = evidence();
        let truth = run(Arm::TruthfulProprioceptionOnly);
        let expected_truth = measured.truthful_monolithic.stop().is_none()
            && differs(
                &measured.synthetic_monolithic,
                &measured.truthful_monolithic,
            );
        assert_eq!(truth.outcome == "survived", expected_truth);

        let composition = run(Arm::TruthfulRecursiveComposition);
        let expected_composition = measured
            .truthful_recursive
            .completed()
            .is_some_and(|trial| trial.aggregate.closes_joint())
            && measured
                .perturbation
                .as_ref()
                .is_some_and(PerturbationTrial::recovered);
        assert_eq!(composition.outcome == "survived", expected_composition);
    }

    #[test]
    fn provenance_memory_factorial_localization() {
        let observed = run(Arm::ProvenanceMemoryFactorialLocalization);
        assert_eq!(observed.outcome, "survived", "{observed:#?}");
        assert!(
            observed.observations["localization"]["explanation"]
                .as_str()
                .is_some_and(|text| !text.is_empty())
        );
    }

    #[test]
    fn fixed_perturbation_is_conditional_and_external() {
        let measured = evidence();
        let primary_closed = measured
            .truthful_recursive
            .completed()
            .is_some_and(|trial| trial.aggregate.closes_joint());
        assert_eq!(measured.perturbation.is_some(), primary_closed);
        if let Some(perturbation) = &measured.perturbation {
            assert_eq!(perturbation.development_steps, 8);
            assert_eq!(perturbation.imposed_position, 4);
            assert_eq!(perturbation.recovery_steps, 16);
        }
    }

    #[test]
    fn mirrored_perturbation_is_held_out_and_deterministic_when_primary_closes() {
        let measured = evidence();
        if measured
            .truthful_recursive
            .completed()
            .is_some_and(|trial| trial.aggregate.closes_joint())
        {
            let first = execute_perturbation(FactorialCell::TRUTHFUL_RECURSIVE, LOWER);
            let second = execute_perturbation(FactorialCell::TRUTHFUL_RECURSIVE, LOWER);
            assert_eq!(first, second);
        }
    }
}
