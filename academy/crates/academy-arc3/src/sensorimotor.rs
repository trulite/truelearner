use crate::{Arc3ActionArguments, Arc3ActionCall};
use academy_workstation2::{
    DeviceEvent, ScreenPoint, TouchId, Viewport, Workstation2, Workstation2Observation,
    Workstation2Session, TAP_TRAVEL,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fmt;
use truelearner_workstation::{
    verify_choice_contract, BodyTraceEvent, Eye, LightField, WorkstationCheckpoint,
    WorkstationError, WorkstationStepObservation, WorldSample, BODY_MAX,
};

pub const ARC3_FRAME_SIDE: usize = 64;
pub const ARC3_FRAME_PIXELS: usize = ARC3_FRAME_SIDE * ARC3_FRAME_SIDE;
pub const ARC3_PALETTE_SIZE: u8 = 16;
pub const WORKSTATION_STEPS_PER_OBSERVATION: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arc3Error(String);

impl Arc3Error {
    pub fn boundary(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for Arc3Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Arc3Error {}

impl From<WorkstationError> for Arc3Error {
    fn from(value: WorkstationError) -> Self {
        Self(format!("workstation Harness failed: {value}"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3ActionWitness {
    pub event: DeviceEvent,
    pub call: Arc3ActionCall,
    pub offered: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3DeviceInput {
    pub event: DeviceEvent,
    pub call: Arc3ActionCall,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3PhysicalStep {
    pub sequence: u64,
    pub sample_sha256: String,
    pub body: WorkstationStepObservation,
    pub device_events: Vec<DeviceEvent>,
    pub trace: Vec<BodyTraceEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3SensorimotorObservation {
    pub sequence: u64,
    pub frame_sha256: String,
    pub frame_changed: Option<bool>,
    pub workstation_steps: usize,
    pub admitted_inputs: usize,
    pub outward_crossings: usize,
    pub device_events: Vec<DeviceEvent>,
    pub application_input: Option<Arc3DeviceInput>,
    pub physical_work: u64,
    pub plasticity_updates: u64,
    pub resident_bytes: usize,
    pub naturally_quiescent: bool,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub steps: Vec<Arc3PhysicalStep>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3SensorimotorSnapshot {
    pub sequence: u64,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub resident_bytes: usize,
    pub previous_frame_sha256: Option<String>,
    pub active_touch_tracks: usize,
}

#[derive(Clone, Debug)]
pub struct Arc3Sensorimotor {
    session: Workstation2Session,
    sequence: u64,
    previous_frame: Option<Vec<u8>>,
    active_touches: Vec<TouchTrack>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TouchTrack {
    touch: TouchId,
    start: ScreenPoint,
    current: ScreenPoint,
    travel: u32,
    last_segment: Option<(ScreenPoint, ScreenPoint)>,
}

impl Arc3Sensorimotor {
    pub fn restore(body_checkpoint: &[u8]) -> Result<Self, Arc3Error> {
        let checkpoint = WorkstationCheckpoint::decode(body_checkpoint)?;
        let frame = LightField::filled(
            ARC3_FRAME_SIDE as u16,
            ARC3_FRAME_SIDE as u16,
            palette_luminance(0),
        )?;
        Ok(Self {
            session: Workstation2Session::with_world(
                checkpoint,
                Workstation2::with_pixels_in_viewport(frame, Viewport::arc())?,
            )?,
            sequence: 0,
            previous_frame: None,
            active_touches: Vec::new(),
        })
    }

    pub fn snapshot(&self) -> Result<Arc3SensorimotorSnapshot, Arc3Error> {
        let read = self.session.body_read()?;
        Ok(Arc3SensorimotorSnapshot {
            sequence: self.sequence,
            body_fingerprint: read.body_fingerprint,
            physical_tick: read.physical_tick,
            resident_bytes: read.resident_bytes,
            previous_frame_sha256: self.previous_frame.as_deref().map(frame_fingerprint),
            active_touch_tracks: self.active_touches.len(),
        })
    }

    pub fn observe(&mut self, frame: Vec<u8>) -> Result<Arc3SensorimotorObservation, Arc3Error> {
        validate_frame(&frame)?;
        let application_frame = application_frame(&frame)?;
        let frame_changed = self
            .previous_frame
            .as_ref()
            .map(|previous| previous != &frame);

        let mut next = self.clone();
        next.session.replace_application_frame(application_frame)?;
        let mut steps = Vec::with_capacity(WORKSTATION_STEPS_PER_OBSERVATION);
        let mut device_events = Vec::new();
        let mut application_input = None;
        let mut admitted_inputs = 0_usize;
        let mut outward_crossings = 0_usize;
        let mut physical_work = 0_u64;
        let mut plasticity_updates = 0_u64;
        let mut naturally_quiescent = true;

        for _ in 0..WORKSTATION_STEPS_PER_OBSERVATION {
            let (session, trace) = next.session.step_traced()?;
            verify_choice_contract(&trace)
                .map_err(|error| Arc3Error::boundary(format!("choice trace failed: {error}")))?;
            admitted_inputs = admitted_inputs.saturating_add(session.body.admitted_inputs);
            outward_crossings = outward_crossings.saturating_add(session.body.crossings.len());
            physical_work = physical_work.saturating_add(session.body.metrics.physical_work);
            plasticity_updates =
                plasticity_updates.saturating_add(session.body.metrics.plasticity_updates);
            naturally_quiescent &= session.body.naturally_quiescent;
            device_events.extend(session.device_events.iter().copied());
            let mut inputs = Vec::new();
            for event in &session.device_events {
                if let Some(input) = gesture_input_for(&mut next.active_touches, event)? {
                    inputs.push(input);
                }
            }
            steps.push(compact_step(session, trace));
            match inputs.as_slice() {
                [] => {}
                [input] => {
                    application_input = Some(input.clone());
                    break;
                }
                _ => {
                    return Err(Arc3Error::boundary(
                        "several complete touch gestures reached the application together",
                    ));
                }
            }
        }

        let read = next.session.body_read()?;
        let result = Arc3SensorimotorObservation {
            sequence: self.sequence,
            frame_sha256: frame_fingerprint(&frame),
            frame_changed,
            workstation_steps: steps.len(),
            admitted_inputs,
            outward_crossings,
            device_events,
            application_input,
            physical_work,
            plasticity_updates,
            resident_bytes: read.resident_bytes,
            naturally_quiescent,
            body_fingerprint: read.body_fingerprint,
            physical_tick: read.physical_tick,
            steps,
        };
        next.sequence = next.sequence.saturating_add(1);
        next.previous_frame = Some(frame);
        *self = next;
        Ok(result)
    }
}

fn validate_frame(frame: &[u8]) -> Result<(), Arc3Error> {
    if frame.len() != ARC3_FRAME_PIXELS {
        return Err(Arc3Error::boundary(format!(
            "ARC frame has {} cells; expected {ARC3_FRAME_PIXELS}",
            frame.len()
        )));
    }
    if frame.iter().any(|value| *value >= ARC3_PALETTE_SIZE) {
        return Err(Arc3Error::boundary(
            "ARC frame contains a value outside the 16-color palette",
        ));
    }
    Ok(())
}

fn application_frame(frame: &[u8]) -> Result<LightField, Arc3Error> {
    Ok(LightField::new(
        ARC3_FRAME_SIDE as u16,
        ARC3_FRAME_SIDE as u16,
        frame.iter().copied().map(palette_luminance).collect(),
    )?)
}

fn palette_luminance(value: u8) -> u8 {
    value.saturating_mul(17)
}

fn frame_fingerprint(frame: &[u8]) -> String {
    hex_digest(Sha256::digest(frame))
}

fn sample_fingerprint(sample: &WorldSample) -> String {
    let mut digest = Sha256::new();
    digest.update(b"academy-workstation2-application-sample-v2");
    for eye in Eye::ALL {
        let field = sample.eye(eye);
        for channel in [field.global(), field.foveal()] {
            digest.update(channel.width().to_le_bytes());
            digest.update(channel.height().to_le_bytes());
            digest.update(channel.pixels());
        }
        digest.update(field.changed_values());
    }
    for contact in sample.contacts() {
        digest.update(contact.pressure().to_le_bytes());
        digest.update(contact.slip().to_le_bytes());
    }
    hex_digest(digest.finalize())
}

fn compact_step(session: Workstation2Observation, trace: Vec<BodyTraceEvent>) -> Arc3PhysicalStep {
    Arc3PhysicalStep {
        sequence: session.sequence,
        sample_sha256: sample_fingerprint(&session.sample),
        body: session.body,
        device_events: session.device_events,
        trace,
    }
}

fn hex_digest(digest: impl AsRef<[u8]>) -> String {
    digest
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn gesture_input_for(
    active: &mut Vec<TouchTrack>,
    event: &DeviceEvent,
) -> Result<Option<Arc3DeviceInput>, Arc3Error> {
    match *event {
        DeviceEvent::TouchStarted { touch, at } => {
            validate_point(at)?;
            if active.iter().any(|track| track.touch == touch) {
                return Err(Arc3Error::boundary("touch started twice"));
            }
            active.push(TouchTrack {
                touch,
                start: at,
                current: at,
                travel: 0,
                last_segment: None,
            });
            Ok(None)
        }
        DeviceEvent::TouchMoved { touch, from, to } => {
            validate_point(from)?;
            validate_point(to)?;
            let track = active
                .iter_mut()
                .find(|track| track.touch == touch)
                .ok_or_else(|| Arc3Error::boundary("touch moved before it started"))?;
            if track.current != from {
                return Err(Arc3Error::boundary("touch movement track is discontinuous"));
            }
            track.travel = track.travel.saturating_add(point_distance(from, to));
            track.current = to;
            if from != to {
                track.last_segment = Some((from, to));
            }
            Ok(None)
        }
        DeviceEvent::TouchEnded { touch, at } => {
            validate_point(at)?;
            let index = active
                .iter()
                .position(|track| track.touch == touch)
                .ok_or_else(|| Arc3Error::boundary("touch ended before it started"))?;
            let track = active.remove(index);
            let start = track.start;
            let current = track.current;
            let mut last_segment = track.last_segment;
            let travel = track.travel.saturating_add(point_distance(current, at));
            if current != at {
                last_segment = Some((current, at));
            }
            let call = if travel <= u32::try_from(TAP_TRAVEL).expect("tap travel is positive") {
                Arc3ActionCall {
                    id: 6,
                    arguments: Arc3ActionArguments::Point {
                        x: scale_point(at.x),
                        y: scale_point(at.y),
                    },
                }
            } else {
                Arc3ActionCall {
                    id: swipe_action(start, at, last_segment),
                    arguments: Arc3ActionArguments::Unit,
                }
            };
            Ok(Some(Arc3DeviceInput {
                event: *event,
                call,
            }))
        }
    }
}

fn validate_point(point: ScreenPoint) -> Result<(), Arc3Error> {
    if (0..=BODY_MAX).contains(&point.x) && (0..=BODY_MAX).contains(&point.y) {
        Ok(())
    } else {
        Err(Arc3Error::boundary("touch point is outside the screen"))
    }
}

fn point_distance(from: ScreenPoint, to: ScreenPoint) -> u32 {
    u32::from(from.x.abs_diff(to.x)) + u32::from(from.y.abs_diff(to.y))
}

fn swipe_action(
    start: ScreenPoint,
    end: ScreenPoint,
    last_segment: Option<(ScreenPoint, ScreenPoint)>,
) -> u8 {
    let mut dx = i32::from(end.x) - i32::from(start.x);
    let mut dy = i32::from(end.y) - i32::from(start.y);
    if dx == 0 && dy == 0 {
        if let Some((from, to)) = last_segment {
            dx = i32::from(to.x) - i32::from(from.x);
            dy = i32::from(to.y) - i32::from(from.y);
        }
    }
    if dx.abs() >= dy.abs() {
        if dx < 0 {
            3
        } else {
            4
        }
    } else if dy < 0 {
        1
    } else {
        2
    }
}

fn scale_point(value: i16) -> u8 {
    let scaled = i32::from(value) * 63 / i32::from(BODY_MAX);
    u8::try_from(scaled).unwrap_or(63).min(63)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sensorimotor_checkpoint() -> Vec<u8> {
        truelearner_workstation::WorkstationHarness::new(31_001)
            .unwrap()
            .save()
            .unwrap()
            .canonical_bytes()
            .unwrap()
    }

    fn gesture(events: &[DeviceEvent]) -> Vec<Arc3DeviceInput> {
        let mut active = Vec::new();
        events
            .iter()
            .filter_map(|event| gesture_input_for(&mut active, event).unwrap())
            .collect()
    }

    fn point(x: i16, y: i16) -> ScreenPoint {
        ScreenPoint { x, y }
    }

    fn touch() -> TouchId {
        TouchId::new(0).unwrap()
    }

    #[test]
    fn palette_becomes_screen_luminance() {
        let frame = (0..16).cycle().take(ARC3_FRAME_PIXELS).collect::<Vec<_>>();
        let converted = application_frame(&frame).unwrap();
        assert_eq!(converted.width(), 64);
        assert_eq!(converted.height(), 64);
        assert_eq!(
            &converted.pixels()[..16],
            &[0, 17, 34, 51, 68, 85, 102, 119, 136, 153, 170, 187, 204, 221, 238, 255]
        );
    }

    #[test]
    fn invalid_frame_is_atomic() {
        let mut sensorimotor = Arc3Sensorimotor::restore(&sensorimotor_checkpoint()).unwrap();
        let before = sensorimotor.snapshot().unwrap();
        assert!(sensorimotor.observe(vec![16; ARC3_FRAME_PIXELS]).is_err());
        assert_eq!(sensorimotor.snapshot().unwrap(), before);
    }

    #[test]
    fn frozen_requests_replay_exactly_from_a_checkpoint() {
        let checkpoint = sensorimotor_checkpoint();
        let mut first = Arc3Sensorimotor::restore(&checkpoint).unwrap();
        let mut replay = Arc3Sensorimotor::restore(&checkpoint).unwrap();
        let mut frame = vec![0; ARC3_FRAME_PIXELS];
        frame[32 * ARC3_FRAME_SIDE + 32] = 9;
        assert_eq!(
            first.observe(frame.clone()).unwrap(),
            replay.observe(frame).unwrap()
        );
        assert_eq!(first.snapshot().unwrap(), replay.snapshot().unwrap());
    }

    #[test]
    fn tap_and_sub_threshold_motion_emit_one_point_call_on_release() {
        for events in [
            vec![
                DeviceEvent::TouchStarted {
                    touch: touch(),
                    at: point(512, 512),
                },
                DeviceEvent::TouchEnded {
                    touch: touch(),
                    at: point(512, 512),
                },
            ],
            vec![
                DeviceEvent::TouchStarted {
                    touch: touch(),
                    at: point(500, 500),
                },
                DeviceEvent::TouchMoved {
                    touch: touch(),
                    from: point(500, 500),
                    to: point(516, 500),
                },
                DeviceEvent::TouchEnded {
                    touch: touch(),
                    at: point(516, 500),
                },
            ],
        ] {
            let calls = gesture(&events);
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].call.id, 6);
            assert!(matches!(calls[0].event, DeviceEvent::TouchEnded { .. }));
        }
    }

    #[test]
    fn four_dominant_swipe_directions_emit_one_unit_call_each() {
        for (end, expected) in [
            (point(512, 400), 1),
            (point(512, 624), 2),
            (point(400, 512), 3),
            (point(624, 512), 4),
        ] {
            let calls = gesture(&[
                DeviceEvent::TouchStarted {
                    touch: touch(),
                    at: point(512, 512),
                },
                DeviceEvent::TouchMoved {
                    touch: touch(),
                    from: point(512, 512),
                    to: end,
                },
                DeviceEvent::TouchEnded {
                    touch: touch(),
                    at: end,
                },
            ]);
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].call.id, expected);
            assert_eq!(calls[0].call.arguments, Arc3ActionArguments::Unit);
        }
    }

    #[test]
    fn equal_displacement_prefers_the_horizontal_sign() {
        assert_eq!(swipe_action(point(512, 512), point(400, 400), None), 3);
        assert_eq!(swipe_action(point(512, 512), point(624, 624), None), 4);
    }

    #[test]
    fn movement_never_emits_before_the_one_release_call() {
        let mut active = Vec::new();
        let started = DeviceEvent::TouchStarted {
            touch: touch(),
            at: point(512, 512),
        };
        let moved = DeviceEvent::TouchMoved {
            touch: touch(),
            from: point(512, 512),
            to: point(600, 512),
        };
        let ended = DeviceEvent::TouchEnded {
            touch: touch(),
            at: point(600, 512),
        };
        assert_eq!(gesture_input_for(&mut active, &started).unwrap(), None);
        assert_eq!(gesture_input_for(&mut active, &moved).unwrap(), None);
        assert!(gesture_input_for(&mut active, &ended).unwrap().is_some());
        assert!(active.is_empty());
    }

    #[test]
    fn invalid_touch_sequence_is_rejected() {
        let mut active = Vec::new();
        let ended = DeviceEvent::TouchEnded {
            touch: touch(),
            at: point(512, 512),
        };
        assert!(gesture_input_for(&mut active, &ended).is_err());
    }
}
