use crate::checkpoint::WorkstationCheckpoint;
use crate::state::{ActuatorFrame, BodyControl, Direction};
use crate::{
    BodyAxis, BodyMovement, Digit, Eye, Point, WorkstationError, WorkstationState, WorldSample,
    AXIS_COUNT, BODY_MAX, TOUCH_SITES,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_body::{
    harness::{attach_outcome_component, attach_sensor, motor},
    Arrival, Body, BodyCheckpoint, BodyCheckpointError, Junction, JunctionId, Run,
    TraceEvent as BodyTraceEvent, Work,
};

const CONTROL_COUNT: usize = AXIS_COUNT * 2;
const RECEPTOR_SIDE: usize = 9;
const RECEPTORS_PER_EYE: usize = RECEPTOR_SIDE * RECEPTOR_SIDE;
const VISUAL_SENSOR_COUNT: usize = Eye::ALL.len() * RECEPTORS_PER_EYE;
const CONTACT_FIELDS: usize = 2;
const PROPRIOCEPTIVE_FIELDS: usize = 6;
const SENSOR_COUNT: usize =
    VISUAL_SENSOR_COUNT + TOUCH_SITES * CONTACT_FIELDS + AXIS_COUNT * PROPRIOCEPTIVE_FIELDS;
const SENSOR_LIFETIME: u64 = u64::MAX;
const SENSOR_PRIME: i32 = i32::MIN;
const OUTCOME_COMPONENTS: usize = 3;
const MOMENT_LIMIT: usize = 512;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Handles {
    pub(crate) vision: [Vec<JunctionId>; 2],
    pub(crate) contacts: [[JunctionId; CONTACT_FIELDS]; TOUCH_SITES],
    pub(crate) proprioception: [[JunctionId; PROPRIOCEPTIVE_FIELDS]; AXIS_COUNT],
    pub(crate) outcomes: [JunctionId; OUTCOME_COMPONENTS],
    pub(crate) opportunities: Vec<JunctionId>,
    #[serde(with = "outward_checkpoint")]
    pub(crate) outward: Vec<(JunctionId, BodyControl)>,
}

impl Handles {
    fn valid_for(&self, body: &Body) -> bool {
        if self
            .vision
            .iter()
            .any(|receptors| receptors.len() != RECEPTORS_PER_EYE)
            || self.opportunities.len() != CONTROL_COUNT
            || self.outward.len() != CONTROL_COUNT
        {
            return false;
        }
        let controls_are_canonical = self
            .outward
            .iter()
            .map(|(_, body_control)| *body_control)
            .eq(BodyAxis::ALL.into_iter().flat_map(|axis| {
                [Direction::Decrease, Direction::Increase]
                    .into_iter()
                    .map(move |direction| control(axis, direction))
            }));
        controls_are_canonical
            && self
                .vision
                .iter()
                .flatten()
                .chain(self.contacts.iter().flatten())
                .chain(self.proprioception.iter().flatten())
                .chain(&self.outcomes)
                .chain(&self.opportunities)
                .chain(self.outward.iter().map(|(junction, _)| junction))
                .all(|junction| body.held(*junction).is_some())
    }
}

mod outward_checkpoint {
    use super::{control, BodyAxis, BodyControl, Direction, JunctionId, CONTROL_COUNT};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(
        outward: &[(JunctionId, BodyControl)],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        outward
            .iter()
            .map(|(junction, body_control)| {
                let axis = body_control.axis().index();
                let direction = usize::from(body_control.direction() == Direction::Increase);
                (
                    *junction,
                    u16::try_from(axis * 2 + direction).unwrap_or(u16::MAX),
                )
            })
            .collect::<Vec<_>>()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<(JunctionId, BodyControl)>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<(JunctionId, u16)>::deserialize(deserializer)?
            .into_iter()
            .map(|(junction, encoded)| {
                let encoded = usize::from(encoded);
                if encoded >= CONTROL_COUNT {
                    return Err(D::Error::custom("invalid checkpoint body control"));
                }
                let direction = if encoded % 2 == 0 {
                    Direction::Decrease
                } else {
                    Direction::Increase
                };
                Ok((junction, control(BodyAxis::ALL[encoded / 2], direction)))
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotorEffect {
    pub at: u64,
    pub control: BodyControl,
    pub impulse: i32,
    pub cause: u64,
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
    pub physical_trace_events: u64,
}

impl StepMetrics {
    fn from_run(run: Run, resident_bytes: usize, physical_trace_events: u64) -> Self {
        Self {
            physical_work: total_work(run.work),
            drive_deliveries: run.work.emissions,
            modulatory_deliveries: 0,
            plasticity_updates: run.work.changes,
            structural_proposals: run.work.changes,
            junction_proposals: 0,
            resident_bytes,
            physical_trace_events,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationStepObservation {
    pub sequence: u64,
    pub state_before: WorkstationState,
    pub state_after: WorkstationState,
    pub pose_changed: bool,
    pub admitted_inputs: usize,
    pub crossings: Vec<MotorEffect>,
    pub movements: Vec<BodyMovement>,
    pub returned_transitions: Vec<BodyAxis>,
    pub pending_transitions: Vec<BodyAxis>,
    pub metrics: StepMetrics,
    pub naturally_quiescent: bool,
    pub body_fingerprint: String,
    pub physical_tick: i64,
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

#[derive(Clone, Debug)]
pub struct WorkstationHarness {
    body: Body,
    handles: Handles,
    state: WorkstationState,
    sequence: u64,
    physical_tick: u64,
    pending_transitions: [bool; AXIS_COUNT],
    history: Vec<WorldSample>,
}

impl PartialEq for WorkstationHarness {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.sequence == other.sequence
            && self.physical_tick == other.physical_tick
            && self.pending_transitions == other.pending_transitions
            && self.history == other.history
    }
}

impl Eq for WorkstationHarness {}

impl WorkstationHarness {
    pub fn new(_seed: u64) -> Result<Self, WorkstationError> {
        Self::fresh()
    }

    fn fresh() -> Result<Self, WorkstationError> {
        let mut body = Body::default();
        body.reserve(OUTCOME_COMPONENTS + SENSOR_COUNT + CONTROL_COUNT * 2, 1_024);
        let mut opportunities = Vec::with_capacity(CONTROL_COUNT);
        let mut outward = Vec::with_capacity(CONTROL_COUNT);
        for axis in BodyAxis::ALL {
            for direction in [Direction::Decrease, Direction::Increase] {
                let attached = motor(&mut body);
                opportunities.push(attached.opportunity);
                outward.push((attached.effect, control(axis, direction)));
            }
        }
        let mut prime = Vec::with_capacity(SENSOR_COUNT);
        let vision = Eye::ALL.map(|eye| {
            (0..RECEPTORS_PER_EYE)
                .map(|receptor| {
                    let nearby = eye_nearness(&opportunities, eye, receptor);
                    attach_sampled_sensor(&mut body, &nearby, &mut prime)
                })
                .collect()
        });
        let contacts = std::array::from_fn(|site| {
            let nearby = contact_nearness(&opportunities, site);
            std::array::from_fn(|_| attach_sampled_sensor(&mut body, &nearby, &mut prime))
        });
        let proprioception = BodyAxis::ALL.map(|axis| {
            let nearby = axis_nearness(&opportunities, axis);
            std::array::from_fn(|_| attach_sampled_sensor(&mut body, &nearby, &mut prime))
        });
        let outcomes =
            std::array::from_fn(|_| attach_sensor(&mut body, Junction::integrating(1), &[]));
        for (component, motors) in [
            &opportunities[..4],
            &opportunities[4..8],
            &opportunities[8..],
        ]
        .into_iter()
        .enumerate()
        {
            attach_outcome_component(&mut body, outcomes[component], motors.iter().copied());
        }
        body.inputs(0, &prime).map_err(body_error)?;
        body.run(MOMENT_LIMIT, |_| {}).map_err(body_error)?;
        Ok(Self {
            body,
            handles: Handles {
                vision,
                contacts,
                proprioception,
                outcomes,
                opportunities,
                outward,
            },
            state: WorkstationState::default(),
            sequence: 0,
            physical_tick: 0,
            pending_transitions: [false; AXIS_COUNT],
            history: Vec::new(),
        })
    }

    pub fn step(
        &mut self,
        sample: WorldSample,
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        let (next, observation) = self.transition(sample)?;
        *self = next;
        Ok(observation)
    }

    pub fn step_traced(
        &mut self,
        sample: WorldSample,
    ) -> Result<(WorkstationStepObservation, Vec<BodyTraceEvent>), WorkstationError> {
        sample.validate()?;
        let mut next = self.clone();
        let mut trace = Vec::new();
        let observation = next.step_in_place_with_trace(sample, Some(&mut trace))?;
        *self = next;
        Ok((observation, trace))
    }

    pub fn transition(
        &self,
        sample: WorldSample,
    ) -> Result<(Self, WorkstationStepObservation), WorkstationError> {
        sample.validate()?;
        let mut next = self.clone();
        let observation = next.step_in_place(sample)?;
        Ok((next, observation))
    }

    fn step_in_place(
        &mut self,
        sample: WorldSample,
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        self.step_in_place_with_trace(sample, None)
    }

    fn step_in_place_with_trace(
        &mut self,
        sample: WorldSample,
        trace: Option<&mut Vec<BodyTraceEvent>>,
    ) -> Result<WorkstationStepObservation, WorkstationError> {
        sample.validate()?;
        let state_before = self.state.clone();
        let returned_transitions = self.pending_axes();
        let cause = self.sequence.saturating_add(1);
        let at = self.physical_tick.saturating_add(1);
        let mut first_wave = self.sensory_wave(&sample, cause);
        let mut returned_components = [false; OUTCOME_COMPONENTS];
        for axis in &returned_transitions {
            returned_components[outcome_component(*axis)] = true;
        }
        for (component, returned) in returned_components.into_iter().enumerate() {
            if returned {
                first_wave.push(Arrival::caused(self.handles.outcomes[component], 1, cause));
            }
        }
        self.body.inputs(at, &first_wave).map_err(body_error)?;
        let opportunity_at = at.saturating_add(1);
        let opportunity_wave = self
            .handles
            .opportunities
            .iter()
            .copied()
            .map(|target| Arrival::caused(target, 1, cause))
            .collect::<Vec<_>>();
        self.body
            .inputs(opportunity_at, &opportunity_wave)
            .map_err(body_error)?;

        let outward = &self.handles.outward;
        let mut crossings = Vec::new();
        let mut latest = opportunity_at;
        let mut observe = |event: truelearner_body::PhysicalEvent| {
            latest = latest.max(event.at);
            if let Some((_, control)) = outward
                .iter()
                .find(|(junction, _)| *junction == event.junction)
            {
                crossings.push(MotorEffect {
                    at: event.at,
                    control: *control,
                    impulse: event
                        .impulse
                        .clamp(i64::from(i32::MIN), i64::from(i32::MAX))
                        as i32,
                    cause: event.cause,
                });
            }
        };
        let run = match trace {
            Some(trace) => self
                .body
                .run_traced(MOMENT_LIMIT, &mut observe, |event| trace.push(event)),
            None => self.body.run(MOMENT_LIMIT, &mut observe),
        }
        .map_err(body_error)?;
        self.physical_tick = latest.max(self.body.now());

        let mut frame = ActuatorFrame::default();
        for effect in &crossings {
            let effort = effect
                .impulse
                .unsigned_abs()
                .min(u32::from(BODY_MAX as u16)) as u16;
            frame.activate(effect.control.axis(), effect.control.direction(), effort);
        }
        let movements = self.state.integrate(frame);
        self.pending_transitions = std::array::from_fn(|index| {
            movements
                .iter()
                .any(|movement| movement.axis.index() == index && movement.changed)
        });
        let pending_transitions = self.pending_axes();
        let pose_changed = !self.state.same_pose(&state_before);
        let sequence = self.sequence;
        self.sequence = self.sequence.saturating_add(1);
        self.history.push(sample);
        let resident_bytes = self.resident_bytes();
        let body_fingerprint = self.fingerprint()?;
        let metrics = StepMetrics::from_run(
            run,
            resident_bytes,
            u64::try_from(crossings.len()).unwrap_or(u64::MAX),
        );
        Ok(WorkstationStepObservation {
            sequence,
            state_before,
            state_after: self.state.clone(),
            pose_changed,
            admitted_inputs: first_wave.len().saturating_add(opportunity_wave.len()),
            crossings,
            movements,
            returned_transitions,
            pending_transitions,
            metrics,
            naturally_quiescent: self.body.is_quiet(),
            body_fingerprint,
            physical_tick: i64::try_from(self.physical_tick).unwrap_or(i64::MAX),
        })
    }

    fn sensory_wave(&self, sample: &WorldSample, cause: u64) -> Vec<Arrival> {
        let mut wave = Vec::with_capacity(SENSOR_COUNT);
        for eye in Eye::ALL {
            for (receptor, target) in self.handles.vision[eye.index()].iter().copied().enumerate() {
                wave.push(Arrival::caused(
                    target,
                    i32::from(sample.eye(eye).sample(receptor_position(receptor))),
                    cause,
                ));
            }
        }
        for (site, contact) in sample.contacts().iter().copied().enumerate() {
            let [pressure, slip] = self.handles.contacts[site];
            wave.push(Arrival::caused(
                pressure,
                i32::from(contact.pressure()),
                cause,
            ));
            wave.push(Arrival::caused(slip, i32::from(contact.slip()), cause));
        }
        for sense in self.state.proprioception() {
            let [position, velocity, decrease, increase, lower, upper] =
                self.handles.proprioception[sense.axis.index()];
            wave.extend([
                Arrival::caused(position, i32::from(sense.position), cause),
                Arrival::caused(velocity, i32::from(sense.velocity), cause),
                Arrival::caused(decrease, i32::from(sense.decrease_effort), cause),
                Arrival::caused(increase, i32::from(sense.increase_effort), cause),
                Arrival::caused(lower, i32::from(sense.at_lower_limit), cause),
                Arrival::caused(upper, i32::from(sense.at_upper_limit), cause),
            ]);
        }
        debug_assert_eq!(wave.len(), SENSOR_COUNT);
        wave
    }

    pub const fn state(&self) -> &WorkstationState {
        &self.state
    }

    pub fn read(&self) -> Result<WorkstationRead, WorkstationError> {
        Ok(WorkstationRead {
            state: self.state.clone(),
            body_fingerprint: self.fingerprint()?,
            physical_tick: i64::try_from(self.physical_tick).unwrap_or(i64::MAX),
            return_path_count: 0,
            resident_bytes: self.resident_bytes(),
            pending_transitions: self.pending_axes(),
        })
    }

    pub fn save(&self) -> Result<WorkstationCheckpoint, WorkstationError> {
        if !self.body.is_quiet() {
            return Err(WorkstationError::InvalidCheckpoint);
        }
        let body = self
            .body
            .checkpoint()
            .and_then(|checkpoint| checkpoint.canonical_bytes())
            .map_err(body_checkpoint_error)?;
        Ok(WorkstationCheckpoint::new(
            body,
            self.handles.clone(),
            self.state.clone(),
            self.sequence,
            self.physical_tick,
            self.pending_transitions,
            self.history.clone(),
        ))
    }

    pub fn restore(checkpoint: WorkstationCheckpoint) -> Result<Self, WorkstationError> {
        let payload = checkpoint.open();
        let body = BodyCheckpoint::decode(&payload.body)
            .and_then(BodyCheckpoint::restore)
            .map_err(body_checkpoint_error)?;
        if !payload.handles.valid_for(&body) {
            return Err(WorkstationError::InvalidCheckpoint);
        }
        Ok(Self {
            body,
            handles: payload.handles,
            state: payload.state,
            sequence: payload.sequence,
            physical_tick: payload.physical_tick,
            pending_transitions: payload.pending_transitions,
            history: payload.history,
        })
    }

    fn pending_axes(&self) -> Vec<BodyAxis> {
        BodyAxis::ALL
            .into_iter()
            .filter(|axis| self.pending_transitions[axis.index()])
            .collect()
    }

    fn resident_bytes(&self) -> usize {
        std::mem::size_of::<Self>().saturating_add(
            self.history
                .iter()
                .map(|sample| bincode::serialized_size(sample).unwrap_or(0) as usize)
                .sum::<usize>(),
        )
    }

    fn fingerprint(&self) -> Result<String, WorkstationError> {
        let mut digest = Sha256::new();
        digest.update(b"truelearner-compact-workstation-v1");
        digest.update(
            bincode::serialize(&self.history).map_err(|_| WorkstationError::InvalidCheckpoint)?,
        );
        digest.update(
            bincode::serialize(&self.state).map_err(|_| WorkstationError::InvalidCheckpoint)?,
        );
        Ok(digest
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }
}

const fn control(axis: BodyAxis, direction: Direction) -> BodyControl {
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

const fn outcome_component(axis: BodyAxis) -> usize {
    match axis {
        BodyAxis::EyeHorizontal { eye: Eye::Left } | BodyAxis::EyeVertical { eye: Eye::Left } => 0,
        BodyAxis::EyeHorizontal { eye: Eye::Right } | BodyAxis::EyeVertical { eye: Eye::Right } => {
            1
        }
        _ => 2,
    }
}

const fn total_work(work: Work) -> u64 {
    work.arrivals
        .saturating_add(work.meetings)
        .saturating_add(work.changes)
        .saturating_add(work.link_visits)
        .saturating_add(work.emissions)
}

fn body_error(error: truelearner_body::RunError) -> WorkstationError {
    WorkstationError::Body(format!("{error:?}"))
}

fn body_checkpoint_error(error: BodyCheckpointError) -> WorkstationError {
    WorkstationError::Body(error.to_string())
}

fn attach_sampled_sensor(
    body: &mut Body,
    nearby: &[(JunctionId, u64)],
    prime: &mut Vec<Arrival>,
) -> JunctionId {
    let sensor = attach_sensor(body, Junction::sampled(SENSOR_LIFETIME), nearby);
    prime.push(Arrival::new(sensor, SENSOR_PRIME));
    sensor
}

fn axis_nearness(opportunities: &[JunctionId], axis: BodyAxis) -> Vec<(JunctionId, u64)> {
    let start = axis.index() * 2;
    vec![(opportunities[start], 1), (opportunities[start + 1], 1)]
}

fn eye_nearness(opportunities: &[JunctionId], eye: Eye, receptor: usize) -> Vec<(JunctionId, u64)> {
    let mut nearby = Vec::with_capacity(2);
    let column = receptor % RECEPTOR_SIDE;
    let row = receptor / RECEPTOR_SIDE;
    extend_directional_nearness(
        &mut nearby,
        opportunities,
        BodyAxis::EyeHorizontal { eye },
        column,
    );
    extend_directional_nearness(
        &mut nearby,
        opportunities,
        BodyAxis::EyeVertical { eye },
        row,
    );
    nearby
}

fn extend_directional_nearness(
    nearby: &mut Vec<(JunctionId, u64)>,
    opportunities: &[JunctionId],
    axis: BodyAxis,
    position: usize,
) {
    let center = RECEPTOR_SIDE / 2;
    let mut push = |direction| {
        let offset = usize::from(direction == Direction::Increase);
        nearby.push((opportunities[axis.index() * 2 + offset], 1));
    };
    match position.cmp(&center) {
        std::cmp::Ordering::Less => push(Direction::Decrease),
        std::cmp::Ordering::Equal => {
            push(Direction::Decrease);
            push(Direction::Increase);
        }
        std::cmp::Ordering::Greater => push(Direction::Increase),
    }
}

fn contact_nearness(opportunities: &[JunctionId], site: usize) -> Vec<(JunctionId, u64)> {
    let axes = if site == 0 {
        vec![
            BodyAxis::PalmHorizontal,
            BodyAxis::PalmVertical,
            BodyAxis::PalmDepth,
            BodyAxis::Wrist,
            BodyAxis::Spread,
        ]
    } else {
        let digit = Digit::ALL[site - 1];
        let mut axes = vec![BodyAxis::PalmDepth, BodyAxis::FingerFlexion { digit }];
        if digit == Digit::Thumb {
            axes.push(BodyAxis::ThumbOpposition);
        }
        axes
    };
    axes.into_iter()
        .flat_map(|axis| axis_nearness(opportunities, axis))
        .collect()
}

fn receptor_position(receptor: usize) -> Point {
    let column = receptor % RECEPTOR_SIDE;
    let row = receptor / RECEPTOR_SIDE;
    let coordinate = |index: usize| {
        let numerator = index * BODY_MAX as usize + RECEPTOR_SIDE - 2;
        i16::try_from(numerator / (RECEPTOR_SIDE - 1)).expect("receptor position is bounded")
    };
    Point::new(coordinate(column), coordinate(row)).expect("receptor position is bounded")
}

#[cfg(test)]
fn sensory_values(wave: &[Arrival]) -> Vec<i32> {
    wave.iter().map(|arrival| arrival.impulse).collect()
}

#[cfg(test)]
fn visual_range(eye: Eye) -> std::ops::Range<usize> {
    let start = eye.index() * RECEPTORS_PER_EYE;
    start..start + RECEPTORS_PER_EYE
}

#[cfg(test)]
fn contact_range() -> std::ops::Range<usize> {
    VISUAL_SENSOR_COUNT..VISUAL_SENSOR_COUNT + TOUCH_SITES * CONTACT_FIELDS
}

#[cfg(test)]
fn proprioception_range() -> std::ops::Range<usize> {
    let start = VISUAL_SENSOR_COUNT + TOUCH_SITES * CONTACT_FIELDS;
    start..start + AXIS_COUNT * PROPRIOCEPTIVE_FIELDS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContactSample, LightField, TOUCH_SITES};

    fn sample() -> WorldSample {
        WorldSample::new(
            [
                LightField::filled(3, 3, 1).unwrap(),
                LightField::filled(3, 3, 2).unwrap(),
            ],
            [ContactSample::default(); TOUCH_SITES],
        )
        .unwrap()
    }

    fn field(value: u8) -> LightField {
        LightField::filled(9, 9, value).unwrap()
    }

    fn centered(value: u8) -> LightField {
        let mut pixels = vec![0; 9 * 9];
        pixels[4 * 9 + 4] = value;
        LightField::new(9, 9, pixels).unwrap()
    }

    fn with_fields(
        left: LightField,
        right: LightField,
        contacts: [ContactSample; TOUCH_SITES],
    ) -> WorldSample {
        WorldSample::new([left, right], contacts).unwrap()
    }

    #[test]
    fn target_intensity_is_an_ordinary_local_reading() {
        let harness = WorkstationHarness::new(1).unwrap();
        let high = harness.sensory_wave(
            &with_fields(
                centered(255),
                field(0),
                [ContactSample::default(); TOUCH_SITES],
            ),
            1,
        );
        let lower = harness.sensory_wave(
            &with_fields(
                centered(254),
                field(0),
                [ContactSample::default(); TOUCH_SITES],
            ),
            1,
        );

        assert_eq!(
            high.iter()
                .map(|arrival| arrival.target)
                .collect::<Vec<_>>(),
            lower
                .iter()
                .map(|arrival| arrival.target)
                .collect::<Vec<_>>()
        );
        let differences = sensory_values(&high)
            .into_iter()
            .zip(sensory_values(&lower))
            .enumerate()
            .filter_map(|(index, (left, right))| (left != right).then_some(index))
            .collect::<Vec<_>>();
        assert_eq!(differences, [RECEPTORS_PER_EYE / 2]);
    }

    #[test]
    fn retinal_position_is_near_only_its_matching_eye_directions() {
        let harness = WorkstationHarness::new(1).unwrap();
        let opportunities = &harness.handles.opportunities;
        let nearby = |axis: BodyAxis, direction: Direction| {
            let offset = usize::from(direction == Direction::Increase);
            (opportunities[axis.index() * 2 + offset], 1)
        };
        let horizontal = BodyAxis::EyeHorizontal { eye: Eye::Left };
        let vertical = BodyAxis::EyeVertical { eye: Eye::Left };
        let left = eye_nearness(opportunities, Eye::Left, RECEPTOR_SIDE * 4);
        let center = eye_nearness(opportunities, Eye::Left, RECEPTOR_SIDE * 4 + 4);
        let lower_right = eye_nearness(opportunities, Eye::Left, RECEPTORS_PER_EYE - 1);

        assert_eq!(
            left,
            vec![
                nearby(horizontal, Direction::Decrease),
                nearby(vertical, Direction::Decrease),
                nearby(vertical, Direction::Increase),
            ]
        );
        assert_eq!(
            center,
            vec![
                nearby(horizontal, Direction::Decrease),
                nearby(horizontal, Direction::Increase),
                nearby(vertical, Direction::Decrease),
                nearby(vertical, Direction::Increase),
            ]
        );
        assert_eq!(
            lower_right,
            vec![
                nearby(horizontal, Direction::Increase),
                nearby(vertical, Direction::Increase),
            ]
        );
    }

    #[test]
    fn changing_the_right_eye_changes_only_right_receptors() {
        let harness = WorkstationHarness::new(2).unwrap();
        let before = sensory_values(&harness.sensory_wave(
            &with_fields(field(1), field(2), [ContactSample::default(); TOUCH_SITES]),
            1,
        ));
        let after = sensory_values(&harness.sensory_wave(
            &with_fields(field(1), field(3), [ContactSample::default(); TOUCH_SITES]),
            1,
        ));

        assert_eq!(
            &before[visual_range(Eye::Left)],
            &after[visual_range(Eye::Left)]
        );
        assert_ne!(
            &before[visual_range(Eye::Right)],
            &after[visual_range(Eye::Right)]
        );
        assert_eq!(&before[contact_range()], &after[contact_range()]);
        assert_eq!(
            &before[proprioception_range()],
            &after[proprioception_range()]
        );
    }

    #[test]
    fn changing_one_touch_site_changes_only_that_touch_reading() {
        let harness = WorkstationHarness::new(3).unwrap();
        let mut contacts = [ContactSample::default(); TOUCH_SITES];
        contacts[2] = ContactSample::new(7, -3).unwrap();
        let before = sensory_values(&harness.sensory_wave(
            &with_fields(field(1), field(2), [ContactSample::default(); TOUCH_SITES]),
            1,
        ));
        let after =
            sensory_values(&harness.sensory_wave(&with_fields(field(1), field(2), contacts), 1));
        let differences = before
            .iter()
            .zip(&after)
            .enumerate()
            .filter_map(|(index, (left, right))| (left != right).then_some(index))
            .collect::<Vec<_>>();
        let start = VISUAL_SENSOR_COUNT + 2 * CONTACT_FIELDS;
        assert_eq!(differences, [start, start + 1]);
    }

    #[test]
    fn transition_is_transactional_and_restore_replays() {
        let harness = WorkstationHarness::new(1).unwrap();
        let (mut candidate, expected) = harness.transition(sample()).unwrap();
        assert_eq!(harness.read().unwrap().physical_tick, 0);
        assert!(expected.naturally_quiescent);

        let checkpoint = candidate.save().unwrap();
        let next_expected = candidate.step(sample()).unwrap();
        let mut restored = WorkstationHarness::restore(checkpoint).unwrap();
        assert_eq!(restored.step(sample()).unwrap(), next_expected);
    }

    #[test]
    fn traced_step_preserves_the_body_and_its_continuation() {
        let mut plain = WorkstationHarness::new(4).unwrap();
        let mut traced = plain.clone();

        let plain_observation = plain.step(sample()).unwrap();
        let (traced_observation, trace) = traced.step_traced(sample()).unwrap();

        assert_eq!(plain_observation, traced_observation);
        assert_eq!(format!("{:?}", plain.body), format!("{:?}", traced.body));
        assert!(trace
            .iter()
            .any(|event| matches!(event, BodyTraceEvent::Choice(_))));
        assert!(matches!(trace.last(), Some(BodyTraceEvent::Quiet(_))));

        assert_eq!(
            plain.step(sample()).unwrap(),
            traced.step(sample()).unwrap()
        );
        assert_eq!(format!("{:?}", plain.body), format!("{:?}", traced.body));
    }
}
