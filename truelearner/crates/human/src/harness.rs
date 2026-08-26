use crate::checkpoint::HumanCheckpoint;
use crate::state::{ActuatorFrame, BodyControl, Direction};
use crate::{
    BodyAxis, BodyMovement, ContactSample, Digit, HumanError, HumanState, Side, WorldSample,
    AXIS_COUNT, DIGIT_COUNT,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_core::{
    Harness, HarnessBuilder, Input, Junction, JunctionId, Link, Output, TransmissionMode,
};

const OUTWARD_REGION: i16 = 1;
const EXTERNAL_FEATURE_COUNT: usize = 36;
const RECEPTORS_PER_AXIS: usize = 9;
const FEATURE_COUNT: usize = EXTERNAL_FEATURE_COUNT + AXIS_COUNT * RECEPTORS_PER_AXIS;
const BINS: usize = 4;
const CONTROL_COUNT: usize = 50;
const AXIS_POSITION_BASE: i32 = 10;
const AXIS_POSITION_STRIDE: i32 = 8;
const SENSOR_PHYSICAL_BASE: u64 = 10_000_000;
const CONTROL_PHYSICAL_BASE: u64 = 20_000_000;
const SINK_PHYSICAL_BASE: u64 = 30_000_000;
const RETURN_PHYSICAL: u64 = 40_000_000;
const ANCHOR_PHYSICAL: u64 = 40_000_001;
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
    returning: JunctionId,
}

impl Sites {
    pub(crate) fn validate(&self) -> Result<(), HumanError> {
        if self.sensors.len() == FEATURE_COUNT && self.motors.len() == CONTROL_COUNT {
            Ok(())
        } else {
            Err(HumanError::InvalidCheckpoint)
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
pub struct HumanStepObservation {
    pub sequence: u64,
    pub state_before: HumanState,
    pub state_after: HumanState,
    pub pose_changed: bool,
    pub admitted_inputs: usize,
    pub crossings: Vec<Output>,
    pub movements: Vec<BodyMovement>,
    pub metrics: StepMetrics,
    pub naturally_quiescent: bool,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub pending_outcome: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HumanRead {
    pub state: HumanState,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub return_path_count: usize,
    pub resident_bytes: usize,
    pub pending_outcome: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HumanHarness {
    boundary: Harness,
    state: HumanState,
    sites: Sites,
    sequence: u64,
    pending_outcome: bool,
}

impl HumanHarness {
    pub fn new(_seed: u64) -> Result<Self, HumanError> {
        let (boundary, sites) = build_harness();
        Ok(Self {
            boundary,
            state: HumanState::default(),
            sites,
            sequence: 0,
            pending_outcome: false,
        })
    }

    pub fn step(&mut self, sample: WorldSample) -> Result<HumanStepObservation, HumanError> {
        sample.validate()?;
        let mut next = self.clone();
        let state_before = next.state.clone();
        let mut metrics = StepMetrics::default();
        let mut naturally_quiescent = true;
        let mut crossings = Vec::new();

        if next.pending_outcome {
            let tick = next.boundary.read().clock.tick.saturating_add(1);
            let returned = next.boundary.send(&[Input {
                arrival_tick: tick,
                phase: 30_000,
                origin_physical: EXTERNAL_PHYSICAL_BASE
                    .saturating_add(next.sequence.saturating_mul(1_000))
                    .saturating_add(900),
                target: next.sites.returning,
                impulse: 1,
            }]);
            metrics.add_run(&returned);
            naturally_quiescent &= returned.naturally_quiescent;
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
                        .saturating_add(next.sequence.saturating_mul(1_000))
                        .saturating_add(u64::try_from(feature).unwrap_or(0)),
                    target: next.sites.sensors[feature][bin],
                    impulse: 1,
                }
            })
            .collect::<Vec<_>>();
        let opportunity_tick = tick.saturating_add(1);
        inputs.extend(next.sites.motors.iter().enumerate().map(|(index, target)| {
            Input {
                arrival_tick: opportunity_tick,
                phase: 20_000_i32.saturating_add(i32::try_from(index).unwrap_or(0)),
                origin_physical: EXTERNAL_PHYSICAL_BASE
                    .saturating_add(next.sequence.saturating_mul(1_000))
                    .saturating_add(500)
                    .saturating_add(u64::try_from(index).unwrap_or(0)),
                target: *target,
                impulse: 1,
            }
        }));
        let run = next.boundary.send(&inputs);
        metrics.add_run(&run);
        naturally_quiescent &= run.naturally_quiescent;
        crossings.extend(run.outputs);

        let mut frame = ActuatorFrame::default();
        for crossing in &crossings {
            let index = crossing
                .from_physical
                .checked_sub(CONTROL_PHYSICAL_BASE)
                .and_then(|value| usize::try_from(value).ok())
                .filter(|index| *index < CONTROL_COUNT)
                .ok_or(HumanError::UnknownOutput(crossing.from_physical))?;
            let control = control(index);
            let impulse = u16::try_from(crossing.impulse.unsigned_abs())
                .unwrap_or(u16::MAX)
                .min(crate::BODY_MAX as u16);
            frame.activate(control.axis(), control.direction(), impulse);
        }
        let movements = next.state.integrate(frame);
        let pose_changed = !next.state.same_pose(&state_before);
        next.pending_outcome = pose_changed;
        next.sequence = next.sequence.saturating_add(1);
        let observation = HumanStepObservation {
            sequence: self.sequence,
            state_before,
            state_after: next.state.clone(),
            pose_changed,
            admitted_inputs: inputs.len(),
            crossings,
            movements,
            metrics,
            naturally_quiescent,
            body_fingerprint: next.fingerprint()?,
            physical_tick: next.boundary.read().clock.tick,
            pending_outcome: next.pending_outcome,
        };
        *self = next;
        Ok(observation)
    }

    pub fn read(&self) -> Result<HumanRead, HumanError> {
        let observation = self.boundary.read();
        Ok(HumanRead {
            state: self.state.clone(),
            body_fingerprint: self.fingerprint()?,
            physical_tick: observation.clock.tick,
            return_path_count: observation.return_path_count,
            resident_bytes: observation.resident_bytes,
            pending_outcome: self.pending_outcome,
        })
    }

    pub fn save(&self) -> Result<HumanCheckpoint, HumanError> {
        let core = self
            .boundary
            .save()
            .map_err(|error| HumanError::CoreCheckpoint(format!("{error:?}")))?
            .canonical_bytes()
            .map_err(|error| HumanError::CoreCheckpoint(format!("{error:?}")))?;
        Ok(HumanCheckpoint::new(
            core,
            self.state.clone(),
            self.sites.clone(),
            self.sequence,
            self.pending_outcome,
        ))
    }

    pub fn restore(checkpoint: HumanCheckpoint) -> Result<Self, HumanError> {
        let payload = checkpoint.open()?;
        let core = truelearner_core::Checkpoint::decode(&payload.core)
            .map_err(|error| HumanError::CoreCheckpoint(format!("{error:?}")))?;
        let boundary = Harness::restore(core)
            .map_err(|error| HumanError::CoreCheckpoint(format!("{error:?}")))?;
        for sensors in &payload.sites.sensors {
            for sensor in sensors {
                if boundary.read().junction(*sensor).is_none() {
                    return Err(HumanError::InvalidCheckpoint);
                }
            }
        }
        for motor in &payload.sites.motors {
            if boundary.read().junction(*motor).is_none() {
                return Err(HumanError::InvalidCheckpoint);
            }
        }
        if boundary.read().junction(payload.sites.returning).is_none() {
            return Err(HumanError::InvalidCheckpoint);
        }
        Ok(Self {
            boundary,
            state: payload.state,
            sites: payload.sites,
            sequence: payload.sequence,
            pending_outcome: payload.pending_outcome,
        })
    }

    fn fingerprint(&self) -> Result<String, HumanError> {
        let checkpoint = self.save()?;
        let digest = Sha256::digest(checkpoint.canonical_bytes()?);
        Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
    }
}

fn build_harness() -> (Harness, Sites) {
    let mut builder = HarnessBuilder::with_capacity(8_192, 16_384, OUTWARD_REGION);
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
    let returning = builder.add_junction(junction(RETURN_PHYSICAL, 2_000, 0, 1));
    let anchor = builder.add_junction(junction(ANCHOR_PHYSICAL, 2_020, 0, 99));
    builder.add_link(link(anchor, returning, 1));
    for sensor in sensors.iter().flatten() {
        builder.add_link(link(anchor, *sensor, 1));
    }
    builder.set_outcome_source(returning);
    (
        builder.build(),
        Sites {
            sensors,
            motors,
            returning,
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
    if feature < 24 {
        let (dx, dy) = RETINA_OFFSETS[feature % RETINA_OFFSETS.len()];
        return if dx == 0 && dy == 0 {
            BodyAxis::Vergence
        } else if dx.unsigned_abs() >= dy.unsigned_abs() {
            BodyAxis::GazeHorizontal
        } else {
            BodyAxis::GazeVertical
        };
    }
    if feature < EXTERNAL_FEATURE_COUNT {
        let contact = feature - 24;
        let side = if contact < crate::TOUCH_SITES {
            Side::Left
        } else {
            Side::Right
        };
        let local = contact % crate::TOUCH_SITES;
        return if local == 0 {
            BodyAxis::ContactForce { side }
        } else {
            BodyAxis::FingerFlexion {
                side,
                digit: Digit::ALL[local - 1],
            }
        };
    }
    let axis = (feature - EXTERNAL_FEATURE_COUNT) / RECEPTORS_PER_AXIS;
    BodyAxis::ALL[axis]
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

fn sensory_features(sample: &WorldSample, state: &HumanState) -> [u8; FEATURE_COUNT] {
    let mut values = [0_u8; FEATURE_COUNT];
    let mut cursor = 0;
    for side in [Side::Left, Side::Right] {
        let focus = state.eyes().focus(side);
        for (dx, dy) in RETINA_OFFSETS {
            values[cursor] = sample.eye(side).sample(focus.offset(dx, dy));
            cursor += 1;
        }
    }
    for side in [Side::Left, Side::Right] {
        for contact in sample.contacts(side) {
            values[cursor] = contact_value(*contact);
            cursor += 1;
        }
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
    match index {
        0 => BodyControl::GazeHorizontal {
            direction: Direction::Decrease,
        },
        1 => BodyControl::GazeHorizontal {
            direction: Direction::Increase,
        },
        2 => BodyControl::GazeVertical {
            direction: Direction::Decrease,
        },
        3 => BodyControl::GazeVertical {
            direction: Direction::Increase,
        },
        4 => BodyControl::Vergence {
            direction: Direction::Decrease,
        },
        5 => BodyControl::Vergence {
            direction: Direction::Increase,
        },
        rest => hand_control(rest - 6),
    }
}

fn hand_control(index: usize) -> BodyControl {
    let side = if index < 22 { Side::Left } else { Side::Right };
    let local = index % 22;
    let direction = if local.is_multiple_of(2) {
        Direction::Decrease
    } else {
        Direction::Increase
    };
    match local / 2 {
        0 => BodyControl::PalmHorizontal { side, direction },
        1 => BodyControl::PalmVertical { side, direction },
        2 => BodyControl::Wrist { side, direction },
        3 => BodyControl::ContactForce { side, direction },
        4 => BodyControl::Spread { side, direction },
        5 => BodyControl::ThumbOpposition { side, direction },
        digit => BodyControl::FingerFlexion {
            side,
            digit: Digit::ALL[(digit - 6).min(DIGIT_COUNT - 1)],
            direction,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dark_sample() -> WorldSample {
        WorldSample::new(
            [
                crate::LightField::filled(1, 1, 0).unwrap(),
                crate::LightField::filled(1, 1, 0).unwrap(),
            ],
            [[ContactSample::default(); crate::TOUCH_SITES]; 2],
        )
        .unwrap()
    }

    #[test]
    fn every_control_changes_only_its_owned_state() {
        let initial = HumanState::default();
        let mut state = initial.clone();
        let gaze = BodyControl::GazeHorizontal {
            direction: Direction::Decrease,
        };
        let mut frame = ActuatorFrame::default();
        frame.activate(gaze.axis(), gaze.direction(), 1);
        assert!(state.integrate(frame)[0].changed);
        assert_ne!(state.eyes(), initial.eyes());
        assert_eq!(state.hand(Side::Left), initial.hand(Side::Left));
        assert_eq!(state.hand(Side::Right), initial.hand(Side::Right));

        let before = state.clone();
        let finger = BodyControl::FingerFlexion {
            side: Side::Right,
            digit: Digit::Index,
            direction: Direction::Increase,
        };
        let mut frame = ActuatorFrame::default();
        frame.activate(finger.axis(), finger.direction(), 1);
        assert!(state.integrate(frame)[0].changed);
        assert_eq!(state.eyes(), before.eyes());
        assert_eq!(state.hand(Side::Left), before.hand(Side::Left));
        assert_ne!(state.hand(Side::Right), before.hand(Side::Right));
    }

    #[test]
    fn control_pairs_and_disjoint_controls_obey_the_expected_laws() {
        let initial = HumanState::default();
        let mut identity = initial.clone();
        let decrease = BodyControl::GazeHorizontal {
            direction: Direction::Decrease,
        };
        let increase = BodyControl::GazeHorizontal {
            direction: Direction::Increase,
        };
        let mut frame = ActuatorFrame::default();
        frame.activate(decrease.axis(), decrease.direction(), 1);
        frame.activate(increase.axis(), increase.direction(), 1);
        let movement = identity.integrate(frame);
        assert!(identity.same_pose(&initial));
        assert!(!movement[0].changed);

        let left = BodyControl::PalmHorizontal {
            side: Side::Left,
            direction: Direction::Decrease,
        };
        let right = BodyControl::PalmVertical {
            side: Side::Right,
            direction: Direction::Increase,
        };
        let mut first = initial.clone();
        let mut first_frame = ActuatorFrame::default();
        first_frame.activate(left.axis(), left.direction(), 1);
        first_frame.activate(right.axis(), right.direction(), 1);
        first.integrate(first_frame);
        let mut second = initial;
        let mut second_frame = ActuatorFrame::default();
        second_frame.activate(right.axis(), right.direction(), 1);
        second_frame.activate(left.axis(), left.direction(), 1);
        second.integrate(second_frame);
        assert_eq!(first, second);
    }

    #[test]
    fn proprioceptive_receptors_preserve_position_velocity_and_effort_sign() {
        let neutral = sensory_features(&dark_sample(), &HumanState::default());
        assert_eq!(&neutral[36..45], &[0, 0, u8::MAX, 0, 0, 0, 0, 0, 0]);

        let mut state = HumanState::default();
        let mut increase = ActuatorFrame::default();
        increase.activate(BodyAxis::GazeHorizontal, Direction::Increase, 1);
        state.integrate(increase);
        let positive = sensory_features(&dark_sample(), &state);
        assert_eq!(&positive[36..45], &[0, 4, 0, 0, 16, 0, 1, 0, 0]);

        let mut decrease = ActuatorFrame::default();
        decrease.activate(BodyAxis::GazeHorizontal, Direction::Decrease, 2);
        state.integrate(decrease);
        let negative = sensory_features(&dark_sample(), &state);
        assert_eq!(&negative[36..45], &[4, 0, 0, 32, 0, 2, 0, 0, 0]);

        state.integrate(ActuatorFrame::default());
        let still = sensory_features(&dark_sample(), &state);
        assert_eq!(&still[36..45], &[4, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn every_neutral_axis_has_one_truthful_position_presence_receptor() {
        let features = sensory_features(&dark_sample(), &HumanState::default());
        for axis in BodyAxis::ALL {
            let first = EXTERNAL_FEATURE_COUNT + axis.index() * RECEPTORS_PER_AXIS;
            assert_eq!(features[first], 0);
            assert_eq!(features[first + 1], 0);
            assert_eq!(features[first + 2], u8::MAX);
            assert_eq!(
                features[first..first + 3]
                    .iter()
                    .filter(|value| **value > 0)
                    .count(),
                1,
                "axis {axis:?}"
            );
        }
    }

    #[test]
    fn every_receptor_is_local_only_to_its_neutral_anatomical_axis() {
        for feature in 0..FEATURE_COUNT {
            let axis = receptor_axis(feature);
            let receptor = receptor_position(feature);
            let local = (0..CONTROL_COUNT)
                .filter(|index| motor_position(*index).abs_diff(receptor) <= 2)
                .collect::<Vec<_>>();
            assert_eq!(local.len(), 2, "feature {feature} local motors {local:?}");
            assert!(local.iter().all(|index| control(*index).axis() == axis));
            assert_eq!(
                motor_position(local[0]).abs_diff(receptor),
                motor_position(local[1]).abs_diff(receptor)
            );
        }
    }

    #[test]
    fn receptor_ownership_follows_eye_touch_and_proprioceptive_anatomy() {
        assert!((0..24).all(|feature| matches!(
            receptor_axis(feature),
            BodyAxis::GazeHorizontal | BodyAxis::GazeVertical | BodyAxis::Vergence
        )));
        assert_eq!(
            receptor_axis(24),
            BodyAxis::ContactForce { side: Side::Left }
        );
        assert_eq!(
            receptor_axis(25),
            BodyAxis::FingerFlexion {
                side: Side::Left,
                digit: Digit::Thumb
            }
        );
        assert_eq!(
            receptor_axis(35),
            BodyAxis::FingerFlexion {
                side: Side::Right,
                digit: Digit::Little
            }
        );
        for axis in BodyAxis::ALL {
            let first = EXTERNAL_FEATURE_COUNT + axis.index() * RECEPTORS_PER_AXIS;
            assert!(
                (first..first + RECEPTORS_PER_AXIS).all(|feature| receptor_axis(feature) == axis)
            );
        }
    }
}
