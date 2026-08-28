use crate::checkpoint::WorkstationCheckpoint;
use crate::state::{ActuatorFrame, BodyControl, Direction};
use crate::{
    BodyAxis, BodyMovement, ContactSample, Digit, Eye, WorkstationError, WorkstationState,
    WorldSample, AXIS_COUNT, BODY_MAX,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, Link, Output, PhysicalIncidence,
    PhysicalInput, Protocol, TransmissionMode,
};
#[cfg(feature = "research")]
use truelearner_core::{PhysicalEvent, PhysicalTransition};

const OUTWARD_REGION: i16 = 1;
const RETINA_FEATURES: usize = 24;
const EXTERNAL_FEATURE_COUNT: usize = RETINA_FEATURES + crate::TOUCH_SITES;
const RECEPTORS_PER_AXIS: usize = 9;
const FEATURE_COUNT: usize = EXTERNAL_FEATURE_COUNT + AXIS_COUNT * RECEPTORS_PER_AXIS;
const BINS: usize = 4;
const CONTROL_COUNT: usize = AXIS_COUNT * 2;
const AXIS_POSITION_BASE: i32 = 10;
const AXIS_POSITION_STRIDE: i32 = 8;
const SENSOR_PHYSICAL_BASE: u64 = 10_000_000;
const CONTROL_PHYSICAL_BASE: u64 = 20_000_000;
const SINK_PHYSICAL_BASE: u64 = 30_000_000;
const OUTCOME_PHYSICAL_BASE: u64 = 40_000_000;
const ANCHOR_PHYSICAL_BASE: u64 = 41_000_000;
const EXTERNAL_PHYSICAL_BASE: u64 = 50_000_000;
const RETINA_OFFSETS: [(i16, i16); 12] = [
    (0, 0),
    (8, 0),
    (-8, 0),
    (0, 8),
    (0, -8),
    (24, 24),
    (-24, 24),
    (24, -24),
    (-24, -24),
    (128, 0),
    (-128, 0),
    (0, 128),
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Sites {
    sensors: Vec<[JunctionId; BINS]>,
    motors: Vec<JunctionId>,
    outcomes: Vec<JunctionId>,
}

impl Sites {
    pub(crate) fn validate(&self) -> Result<(), WorkstationError> {
        if self.sensors.len() == FEATURE_COUNT
            && self.motors.len() == CONTROL_COUNT
            && self.outcomes.len() == AXIS_COUNT
        {
            Ok(())
        } else {
            Err(WorkstationError::InvalidCheckpoint)
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepMetrics {
    pub physical_work: u64,
    pub drive_deliveries: u64,
    pub modulatory_deliveries: u64,
    pub plasticity_updates: u64,
    pub structural_proposals: u64,
    pub junction_proposals: u64,
    pub resident_bytes: usize,
}

impl StepMetrics {
    fn add_run(&mut self, run: &truelearner_core::Run) {
        self.physical_work = self.physical_work.saturating_add(run.work.physical_total());
        self.drive_deliveries = self
            .drive_deliveries
            .saturating_add(run.work.drive_deliveries);
        self.modulatory_deliveries = self
            .modulatory_deliveries
            .saturating_add(run.work.modulatory_deliveries);
        self.plasticity_updates = self
            .plasticity_updates
            .saturating_add(run.work.local_return_updates);
        self.structural_proposals = self
            .structural_proposals
            .saturating_add(run.work.local_structural_proposals);
        self.junction_proposals = self
            .junction_proposals
            .saturating_add(run.work.local_junction_proposals);
        self.resident_bytes = self.resident_bytes.max(run.memory_bytes);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationStepObservation {
    pub sequence: u64,
    pub state_before: WorkstationState,
    pub state_after: WorkstationState,
    pub pose_changed: bool,
    pub admitted_inputs: usize,
    pub crossings: Vec<Output>,
    pub movements: Vec<BodyMovement>,
    pub returned_transitions: Vec<BodyAxis>,
    pub pending_transitions: Vec<BodyAxis>,
    pub metrics: StepMetrics,
    pub naturally_quiescent: bool,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    #[cfg(feature = "research")]
    pub choice_diagnostics: Vec<ResearchChoiceDiagnostic>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationRead {
    pub state: WorkstationState,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub return_path_count: usize,
    pub resident_bytes: usize,
    pub pending_transitions: Vec<BodyAxis>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkstationHarness {
    boundary: Harness,
    state: WorkstationState,
    sites: Sites,
    sequence: u64,
    pending_transitions: [bool; AXIS_COUNT],
    #[cfg(feature = "research")]
    opportunity_incidence: ResearchOpportunityIncidence,
    #[cfg(feature = "research")]
    transition_opportunity: ResearchTransitionOpportunity,
}

#[cfg(feature = "research")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResearchOpportunityIncidence {
    Independent,
    SharedWave,
}

#[cfg(feature = "research")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResearchTransitionOpportunity {
    GenericOnly,
    LocalAfterTransition,
    ComposedWithReturn,
}

#[cfg(feature = "research")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResearchHarnessConfig {
    pub protocol: Protocol,
    pub opportunity_incidence: ResearchOpportunityIncidence,
    pub transition_opportunity: ResearchTransitionOpportunity,
}

#[cfg(feature = "research")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "diagnostic", rename_all = "snake_case")]
pub enum ResearchChoiceDiagnostic {
    Candidate {
        tick: i64,
        phase: i32,
        control: BodyControl,
        ownership: String,
        path_inputs: u32,
        positive_path_strength: u64,
        negative_path_strength: u64,
        opportunity: i64,
        supplied_opportunity: i64,
        admitted_drive: i64,
        projected_drive: i64,
        threshold: i64,
        consequence_tick: Option<i64>,
        unanswered_returns: u32,
        executable: bool,
    },
    TransitionContinuation {
        tick: i64,
        phase: i32,
        control: BodyControl,
        current_owner_transition: bool,
        unanswered_returns: u32,
        admitted: bool,
    },
    Choice {
        tick: i64,
        phase: i32,
        ordinary_control: Option<BodyControl>,
        current_transition_control: Option<BodyControl>,
        computed_winner_control: Option<BodyControl>,
        admitted_controls: Vec<BodyControl>,
        computed_winner_basis: String,
        admission_basis: String,
    },
}

impl WorkstationHarness {
    pub fn new(_seed: u64) -> Result<Self, WorkstationError> {
        let (boundary, sites) =
            build_harness(Protocol::RecursiveLearnerCausalTopologyProductComposition);
        Ok(Self {
            boundary,
            state: WorkstationState::default(),
            sites,
            sequence: 0,
            pending_transitions: [false; AXIS_COUNT],
            #[cfg(feature = "research")]
            opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
            #[cfg(feature = "research")]
            transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
        })
    }

    #[cfg(feature = "research")]
    fn new_with(
        protocol: Protocol,
        opportunity_incidence: ResearchOpportunityIncidence,
        transition_opportunity: ResearchTransitionOpportunity,
    ) -> Result<Self, WorkstationError> {
        let (boundary, sites) = build_harness(protocol);
        Ok(Self {
            boundary,
            state: WorkstationState::default(),
            sites,
            sequence: 0,
            pending_transitions: [false; AXIS_COUNT],
            opportunity_incidence,
            transition_opportunity,
        })
    }

    #[cfg(feature = "research")]
    pub fn new_research(
        _seed: u64,
        config: ResearchHarnessConfig,
    ) -> Result<Self, WorkstationError> {
        Self::new_with(
            config.protocol,
            config.opportunity_incidence,
            config.transition_opportunity,
        )
    }

    pub fn step(
        &mut self,
        sample: WorldSample,
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        sample.validate()?;
        let mut next = self.clone();
        let state_before = next.state.clone();
        let returned_transitions = next.pending_axes();
        let mut metrics = StepMetrics::default();
        let mut naturally_quiescent = true;
        let mut crossings = Vec::new();
        let mut admitted_inputs = 0;
        #[cfg(feature = "research")]
        let mut choice_diagnostics = Vec::new();

        if !returned_transitions.is_empty() {
            let tick = next.boundary.read().clock.tick.saturating_add(1);
            let returns = returned_transitions
                .iter()
                .enumerate()
                .map(|(order, axis)| PhysicalInput {
                    input: Input {
                        arrival_tick: tick,
                        phase: 30_000_i32.saturating_add(i32::try_from(order).unwrap_or(0)),
                        origin_physical: transition_origin(next.sequence, *axis),
                        target: next.sites.outcomes[axis.index()],
                        impulse: 1,
                    },
                    incidence: PhysicalIncidence::Transition,
                })
                .collect::<Vec<_>>();
            #[cfg(feature = "research")]
            let returns = if next.transition_opportunity
                == ResearchTransitionOpportunity::ComposedWithReturn
            {
                let mut returns = returns;
                let opportunity_tick = tick.saturating_add(1);
                for (order, axis) in returned_transitions.iter().enumerate() {
                    let first_motor = axis.index() * 2;
                    let phase = 30_000_i32.saturating_add(i32::try_from(order).unwrap_or(0));
                    let origin_physical = transition_opportunity_origin(next.sequence, *axis);
                    for target in &next.sites.motors[first_motor..first_motor + 2] {
                        returns.push(PhysicalInput {
                            input: Input {
                                arrival_tick: opportunity_tick,
                                phase,
                                origin_physical,
                                target: *target,
                                impulse: 1,
                            },
                            incidence: PhysicalIncidence::Sample,
                        });
                    }
                }
                returns
            } else {
                returns
            };
            admitted_inputs += returns.len();
            let returned = next.boundary.send_physical(&returns);
            metrics.add_run(&returned);
            naturally_quiescent &= returned.naturally_quiescent;
            #[cfg(feature = "research")]
            choice_diagnostics.extend(project_choice_diagnostics(
                &returned.physical_trace,
                &next.sites,
            ));
            crossings.extend(returned.outputs);
        }

        let features = sensory_features(&sample, &next.state);
        let tick = next.boundary.read().clock.tick.saturating_add(1);
        let mut inputs = features
            .iter()
            .enumerate()
            .filter(|(_, value)| **value > 0)
            .map(|(feature, value)| {
                let bin = usize::from(*value / 64).min(BINS - 1);
                Input {
                    arrival_tick: tick,
                    phase: i32::try_from(feature).unwrap_or(0),
                    origin_physical: EXTERNAL_PHYSICAL_BASE
                        .saturating_add(next.sequence.saturating_mul(10_000))
                        .saturating_add(u64::try_from(feature).unwrap_or(0)),
                    target: next.sites.sensors[feature][bin],
                    impulse: 1,
                }
            })
            .collect::<Vec<_>>();
        let opportunity_tick = tick.saturating_add(1);
        inputs.extend(next.sites.motors.iter().enumerate().map(|(index, target)| {
            let (phase, origin_offset) = next.opportunity_coordinates(index);
            Input {
                arrival_tick: opportunity_tick,
                phase,
                origin_physical: EXTERNAL_PHYSICAL_BASE
                    .saturating_add(next.sequence.saturating_mul(10_000))
                    .saturating_add(origin_offset),
                target: *target,
                impulse: 1,
            }
        }));
        #[cfg(feature = "research")]
        if next.transition_opportunity == ResearchTransitionOpportunity::LocalAfterTransition {
            for (order, axis) in returned_transitions.iter().enumerate() {
                let first_motor = axis.index() * 2;
                let phase = 30_000_i32.saturating_add(i32::try_from(order).unwrap_or(0));
                let origin_physical = transition_origin(next.sequence, *axis);
                for target in &next.sites.motors[first_motor..first_motor + 2] {
                    inputs.push(Input {
                        arrival_tick: opportunity_tick,
                        phase,
                        origin_physical,
                        target: *target,
                        impulse: 1,
                    });
                }
            }
        }
        admitted_inputs += inputs.len();
        let run = next.boundary.send(&inputs);
        metrics.add_run(&run);
        naturally_quiescent &= run.naturally_quiescent;
        #[cfg(feature = "research")]
        choice_diagnostics.extend(project_choice_diagnostics(&run.physical_trace, &next.sites));
        crossings.extend(run.outputs);

        let mut frame = ActuatorFrame::default();
        for crossing in &crossings {
            let index = crossing
                .from_physical
                .checked_sub(CONTROL_PHYSICAL_BASE)
                .and_then(|value| usize::try_from(value).ok())
                .filter(|index| *index < CONTROL_COUNT)
                .ok_or(WorkstationError::UnknownOutput(crossing.from_physical))?;
            let control = control(index);
            let impulse = u16::try_from(crossing.impulse.unsigned_abs())
                .unwrap_or(u16::MAX)
                .min(BODY_MAX as u16);
            frame.activate(control.axis(), control.direction(), impulse);
        }
        let movements = next.state.integrate(frame);
        let pose_changed = !next.state.same_pose(&state_before);
        next.pending_transitions = [false; AXIS_COUNT];
        for movement in &movements {
            if movement.changed {
                next.pending_transitions[movement.axis.index()] = true;
            }
        }
        next.sequence = next.sequence.saturating_add(1);
        let observation = WorkstationStepObservation {
            sequence: self.sequence,
            state_before,
            state_after: next.state.clone(),
            pose_changed,
            admitted_inputs,
            crossings,
            movements,
            returned_transitions,
            pending_transitions: next.pending_axes(),
            metrics,
            naturally_quiescent,
            body_fingerprint: next.fingerprint()?,
            physical_tick: next.boundary.read().clock.tick,
            #[cfg(feature = "research")]
            choice_diagnostics,
        };
        *self = next;
        Ok(observation)
    }

    pub fn read(&self) -> Result<WorkstationRead, WorkstationError> {
        let observation = self.boundary.read();
        Ok(WorkstationRead {
            state: self.state.clone(),
            body_fingerprint: self.fingerprint()?,
            physical_tick: observation.clock.tick,
            return_path_count: observation.return_path_count,
            resident_bytes: observation.resident_bytes,
            pending_transitions: self.pending_axes(),
        })
    }

    pub fn save(&self) -> Result<WorkstationCheckpoint, WorkstationError> {
        let core = self
            .boundary
            .save()
            .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?
            .canonical_bytes()
            .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
        Ok(WorkstationCheckpoint::new(
            core,
            self.state.clone(),
            self.sites.clone(),
            self.sequence,
            self.pending_transitions,
        ))
    }

    pub fn restore(checkpoint: WorkstationCheckpoint) -> Result<Self, WorkstationError> {
        let payload = checkpoint.open()?;
        let core = truelearner_core::Checkpoint::decode(&payload.core)
            .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
        let boundary = Harness::restore(core)
            .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
        for sensors in &payload.sites.sensors {
            for sensor in sensors {
                if boundary.read().junction(*sensor).is_none() {
                    return Err(WorkstationError::InvalidCheckpoint);
                }
            }
        }
        for site in payload.sites.motors.iter().chain(&payload.sites.outcomes) {
            if boundary.read().junction(*site).is_none() {
                return Err(WorkstationError::InvalidCheckpoint);
            }
        }
        Ok(Self {
            boundary,
            state: payload.state,
            sites: payload.sites,
            sequence: payload.sequence,
            pending_transitions: payload.pending_transitions,
            #[cfg(feature = "research")]
            opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
            #[cfg(feature = "research")]
            transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
        })
    }

    #[cfg(feature = "research")]
    pub fn restore_research(
        checkpoint: WorkstationCheckpoint,
        opportunity_incidence: ResearchOpportunityIncidence,
    ) -> Result<Self, WorkstationError> {
        let mut restored = Self::restore(checkpoint)?;
        restored.opportunity_incidence = opportunity_incidence;
        Ok(restored)
    }

    #[cfg(feature = "research")]
    pub fn restore_research_config(
        checkpoint: WorkstationCheckpoint,
        config: ResearchHarnessConfig,
    ) -> Result<Self, WorkstationError> {
        let mut restored = Self::restore(checkpoint)?;
        restored.opportunity_incidence = config.opportunity_incidence;
        restored.transition_opportunity = config.transition_opportunity;
        Ok(restored)
    }

    fn opportunity_coordinates(&self, _index: usize) -> (i32, u64) {
        #[cfg(feature = "research")]
        if self.opportunity_incidence == ResearchOpportunityIncidence::Independent {
            return (
                20_000_i32.saturating_add(i32::try_from(_index).unwrap_or(0)),
                5_000_u64.saturating_add(u64::try_from(_index).unwrap_or(0)),
            );
        }

        (20_000, 5_000)
    }

    fn pending_axes(&self) -> Vec<BodyAxis> {
        BodyAxis::ALL
            .into_iter()
            .filter(|axis| self.pending_transitions[axis.index()])
            .collect()
    }

    fn fingerprint(&self) -> Result<String, WorkstationError> {
        let digest = Sha256::digest(self.save()?.canonical_bytes()?);
        Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
    }
}

fn transition_origin(sequence: u64, axis: BodyAxis) -> u64 {
    EXTERNAL_PHYSICAL_BASE
        .saturating_add(sequence.saturating_mul(10_000))
        .saturating_add(9_000)
        .saturating_add(u64::try_from(axis.index()).unwrap_or(0))
}

#[cfg(feature = "research")]
fn transition_opportunity_origin(sequence: u64, axis: BodyAxis) -> u64 {
    EXTERNAL_PHYSICAL_BASE
        .saturating_add(sequence.saturating_mul(10_000))
        .saturating_add(9_500)
        .saturating_add(u64::try_from(axis.index()).unwrap_or(0))
}

#[cfg(feature = "research")]
fn project_choice_diagnostics(
    trace: &[PhysicalTransition],
    sites: &Sites,
) -> Vec<ResearchChoiceDiagnostic> {
    trace
        .iter()
        .filter_map(|transition| match &transition.event {
            PhysicalEvent::OutputCandidateEvaluated {
                target,
                ownership,
                path_inputs,
                positive_path_strength,
                negative_path_strength,
                opportunity,
                supplied_opportunity,
                admitted_drive,
                projected_drive,
                threshold,
                consequence_tick,
                unanswered_returns,
                executable,
                ..
            } => Some(ResearchChoiceDiagnostic::Candidate {
                tick: transition.tick,
                phase: transition.phase,
                control: control_for_target(sites, *target)?,
                ownership: format!("{ownership:?}"),
                path_inputs: *path_inputs,
                positive_path_strength: *positive_path_strength,
                negative_path_strength: *negative_path_strength,
                opportunity: *opportunity,
                supplied_opportunity: *supplied_opportunity,
                admitted_drive: *admitted_drive,
                projected_drive: *projected_drive,
                threshold: *threshold,
                consequence_tick: *consequence_tick,
                unanswered_returns: *unanswered_returns,
                executable: *executable,
            }),
            PhysicalEvent::PhysicalTransitionContinuationEvaluated {
                target,
                current_owner_transition,
                unanswered_returns,
                admitted,
                ..
            } => Some(ResearchChoiceDiagnostic::TransitionContinuation {
                tick: transition.tick,
                phase: transition.phase,
                control: control_for_target(sites, *target)?,
                current_owner_transition: *current_owner_transition,
                unanswered_returns: *unanswered_returns,
                admitted: *admitted,
            }),
            PhysicalEvent::OutputChoiceResolved {
                ordinary_target,
                current_transition_target,
                computed_winner_target,
                admitted,
                computed_winner_basis,
                admission_basis,
                ..
            } => Some(ResearchChoiceDiagnostic::Choice {
                tick: transition.tick,
                phase: transition.phase,
                ordinary_control: control_for_target(sites, *ordinary_target),
                current_transition_control: current_transition_target
                    .and_then(|target| control_for_target(sites, target)),
                computed_winner_control: control_for_target(sites, *computed_winner_target),
                admitted_controls: admitted
                    .iter()
                    .filter_map(|admission| control_for_target(sites, admission.target))
                    .collect(),
                computed_winner_basis: format!("{computed_winner_basis:?}"),
                admission_basis: format!("{admission_basis:?}"),
            }),
            _ => None,
        })
        .collect()
}

#[cfg(feature = "research")]
fn control_for_target(sites: &Sites, target: JunctionId) -> Option<BodyControl> {
    sites
        .motors
        .iter()
        .position(|candidate| *candidate == target)
        .map(control)
}

fn build_harness(protocol: Protocol) -> (Harness, Sites) {
    let mut builder = HarnessBuilder::with_capacity(8_192, 16_384, OUTWARD_REGION);
    builder.set_protocol(protocol);
    builder.set_physical_tracing(true);

    let motors = (0..CONTROL_COUNT)
        .map(|index| {
            let position = motor_position(index);
            let motor = builder.add_junction(junction(
                CONTROL_PHYSICAL_BASE + u64::try_from(index).unwrap_or(0),
                position,
                0,
                2,
            ));
            let sink = builder.add_junction(junction(
                SINK_PHYSICAL_BASE + u64::try_from(index).unwrap_or(0),
                position,
                OUTWARD_REGION,
                1,
            ));
            builder.add_link(link(motor, sink, 1));
            motor
        })
        .collect::<Vec<_>>();
    let sensors = (0..FEATURE_COUNT)
        .map(|feature| {
            std::array::from_fn(|bin| {
                let position = receptor_position(feature);
                builder.add_junction(junction(
                    SENSOR_PHYSICAL_BASE
                        + u64::try_from(feature.saturating_mul(BINS).saturating_add(bin))
                            .unwrap_or(0),
                    position,
                    0,
                    1,
                ))
            })
        })
        .collect::<Vec<_>>();
    let outcomes = BodyAxis::ALL
        .into_iter()
        .map(|axis| {
            builder.add_junction(junction(
                OUTCOME_PHYSICAL_BASE + u64::try_from(axis.index()).unwrap_or(0),
                2_000 + i32::try_from(axis.index()).unwrap_or(0) * 8,
                0,
                1,
            ))
        })
        .collect::<Vec<_>>();

    for axis in BodyAxis::ALL {
        let anchor = builder.add_junction(junction(
            ANCHOR_PHYSICAL_BASE + u64::try_from(axis.index()).unwrap_or(0),
            3_000 + i32::try_from(axis.index()).unwrap_or(0) * 8,
            0,
            99,
        ));
        builder.add_link(link(anchor, outcomes[axis.index()], 1));
        for (feature, bins) in sensors.iter().enumerate() {
            if receptor_axis(feature) == axis {
                for sensor in bins {
                    builder.add_link(link(anchor, *sensor, 1));
                }
            }
        }
        let first_motor = axis.index() * 2;
        builder.set_outcome_source_for_output(motors[first_motor], outcomes[axis.index()]);
        builder.set_outcome_source_for_output(motors[first_motor + 1], outcomes[axis.index()]);
    }

    (
        builder.build(),
        Sites {
            sensors,
            motors,
            outcomes,
        },
    )
}

fn motor_position(index: usize) -> i32 {
    let control = control(index);
    let center = axis_position(control.axis());
    match control.direction() {
        Direction::Decrease => center - 1,
        Direction::Increase => center + 1,
    }
}

fn receptor_position(feature: usize) -> i32 {
    axis_position(receptor_axis(feature))
}

fn axis_position(axis: BodyAxis) -> i32 {
    AXIS_POSITION_BASE
        .saturating_add(i32::try_from(axis.index()).unwrap_or(0) * AXIS_POSITION_STRIDE)
}

fn receptor_axis(feature: usize) -> BodyAxis {
    debug_assert!(feature < FEATURE_COUNT);
    if feature < RETINA_FEATURES {
        let eye = Eye::ALL[feature / RETINA_OFFSETS.len()];
        let (dx, dy) = RETINA_OFFSETS[feature % RETINA_OFFSETS.len()];
        return if dx.unsigned_abs() >= dy.unsigned_abs() {
            BodyAxis::EyeHorizontal { eye }
        } else {
            BodyAxis::EyeVertical { eye }
        };
    }
    if feature < EXTERNAL_FEATURE_COUNT {
        let contact = feature - RETINA_FEATURES;
        return if contact == 0 {
            BodyAxis::PalmDepth
        } else {
            BodyAxis::FingerFlexion {
                digit: Digit::ALL[contact - 1],
            }
        };
    }
    BodyAxis::ALL[(feature - EXTERNAL_FEATURE_COUNT) / RECEPTORS_PER_AXIS]
}

fn junction(physical_id: u64, position: i32, region: i16, threshold: i32) -> Junction {
    Junction {
        physical_id,
        position,
        region,
        threshold,
        resistance: u32::MAX,
    }
}

fn link(from: JunctionId, to: JunctionId, coupling: i32) -> Link {
    Link {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling,
        resistance: u32::MAX,
        mode: TransmissionMode::Drive,
    }
}

fn sensory_features(sample: &WorldSample, state: &WorkstationState) -> [u8; FEATURE_COUNT] {
    let mut values = [0_u8; FEATURE_COUNT];
    let mut cursor = 0;
    for eye in Eye::ALL {
        let focus = state.eye(eye).gaze();
        for (dx, dy) in RETINA_OFFSETS {
            values[cursor] = sample.eye(eye).sample(focus.offset(dx, dy));
            cursor += 1;
        }
    }
    for contact in sample.contacts() {
        values[cursor] = contact_value(*contact);
        cursor += 1;
    }
    debug_assert_eq!(cursor, EXTERNAL_FEATURE_COUNT);
    for sense in state.proprioception() {
        for value in signed_channels(sense.position / 4)
            .into_iter()
            .chain([if sense.position == 0 { u8::MAX } else { 0 }])
            .chain(signed_channels(sense.velocity))
            .chain([
                magnitude(sense.decrease_effort),
                magnitude(sense.increase_effort),
                if sense.at_lower_limit { u8::MAX } else { 0 },
                if sense.at_upper_limit { u8::MAX } else { 0 },
            ])
        {
            values[cursor] = value;
            cursor += 1;
        }
    }
    debug_assert_eq!(cursor, FEATURE_COUNT);
    values
}

fn contact_value(contact: ContactSample) -> u8 {
    let pressure = contact.pressure() / 4;
    let slip = contact.slip().unsigned_abs() / 4;
    u8::try_from(pressure.saturating_add(slip).min(u16::from(u8::MAX))).unwrap_or(u8::MAX)
}

fn signed_channels(value: i16) -> [u8; 2] {
    if value.is_negative() {
        [magnitude(value.unsigned_abs()), 0]
    } else {
        [0, magnitude(value.unsigned_abs())]
    }
}

fn magnitude(value: u16) -> u8 {
    u8::try_from(value.min(u16::from(u8::MAX))).unwrap_or(u8::MAX)
}

fn control(index: usize) -> BodyControl {
    let axis = BodyAxis::ALL[index / 2];
    let direction = if index.is_multiple_of(2) {
        Direction::Decrease
    } else {
        Direction::Increase
    };
    control_for_axis(axis, direction)
}

const fn control_for_axis(axis: BodyAxis, direction: Direction) -> BodyControl {
    match axis {
        BodyAxis::EyeHorizontal { eye } => BodyControl::EyeHorizontal { eye, direction },
        BodyAxis::EyeVertical { eye } => BodyControl::EyeVertical { eye, direction },
        BodyAxis::PalmHorizontal => BodyControl::PalmHorizontal { direction },
        BodyAxis::PalmVertical => BodyControl::PalmVertical { direction },
        BodyAxis::PalmDepth => BodyControl::PalmDepth { direction },
        BodyAxis::Wrist => BodyControl::Wrist { direction },
        BodyAxis::Spread => BodyControl::Spread { direction },
        BodyAxis::ThumbOpposition => BodyControl::ThumbOpposition { direction },
        BodyAxis::FingerFlexion { digit } => BodyControl::FingerFlexion { digit, direction },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn dark_sample() -> WorldSample {
        WorldSample::new(
            [
                crate::LightField::filled(1, 1, 0).unwrap(),
                crate::LightField::filled(1, 1, 0).unwrap(),
            ],
            [ContactSample::default(); crate::TOUCH_SITES],
        )
        .unwrap()
    }

    #[test]
    fn every_control_maps_to_one_axis_and_the_two_directions() {
        for axis in BodyAxis::ALL {
            let pair = [control(axis.index() * 2), control(axis.index() * 2 + 1)];
            assert_eq!(pair.map(BodyControl::axis), [axis, axis]);
            assert_eq!(
                pair.map(BodyControl::direction),
                [Direction::Decrease, Direction::Increase]
            );
        }
    }

    #[test]
    fn each_axis_has_one_distinct_local_outcome_component() {
        let (boundary, sites) =
            build_harness(Protocol::RecursiveLearnerCausalTopologyProductComposition);
        assert_eq!(
            boundary.read().protocol,
            Protocol::RecursiveLearnerCausalTopologyProductComposition
        );
        assert_eq!(sites.outcomes.len(), AXIS_COUNT);
        assert_eq!(
            sites
                .outcomes
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            AXIS_COUNT
        );
    }

    #[test]
    fn every_receptor_is_local_to_one_anatomical_axis() {
        for feature in 0..FEATURE_COUNT {
            let axis = receptor_axis(feature);
            let receptor = receptor_position(feature);
            let local = (0..CONTROL_COUNT)
                .filter(|index| motor_position(*index).abs_diff(receptor) <= 2)
                .collect::<Vec<_>>();
            assert_eq!(local.len(), 2, "feature {feature} local motors {local:?}");
            assert!(local.iter().all(|index| control(*index).axis() == axis));
        }
    }

    #[test]
    fn neutral_proprioception_keeps_every_axis_physically_present() {
        let features = sensory_features(&dark_sample(), &WorkstationState::default());
        for axis in BodyAxis::ALL {
            let first = EXTERNAL_FEATURE_COUNT + axis.index() * RECEPTORS_PER_AXIS;
            assert_eq!(&features[first..first + 3], &[0, 0, u8::MAX]);
        }
    }
}
