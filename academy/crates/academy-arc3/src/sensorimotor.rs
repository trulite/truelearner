use crate::{Arc3ActionArguments, Arc3ActionCall};
use academy_workstation::{
    DeviceEvent, DeviceState, MonitorFrame, SessionObservation, WorkstationPresentation,
    WorkstationSession, WorldError, WorldGeometry,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fmt;
use truelearner_workstation::{
    verify_choice_laws, BodyTraceEvent, Eye, WorkstationCheckpoint, WorkstationError,
    WorkstationStepObservation, WorldSample,
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

impl From<WorldError> for Arc3Error {
    fn from(value: WorldError) -> Self {
        Self(format!("workstation application failed: {value}"))
    }
}

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
    pub device_after: DeviceState,
    pub world_fingerprint: String,
    pub session_fingerprint: String,
    pub trace: Vec<BodyTraceEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3SensorimotorObservation {
    pub sequence: u64,
    pub frame_sha256: String,
    pub frame_changed: Option<bool>,
    pub returned_previous_device_input: bool,
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
    pub session_fingerprint: String,
    pub physical_tick: i64,
    pub steps: Vec<Arc3PhysicalStep>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3SensorimotorSnapshot {
    pub sequence: u64,
    pub body_fingerprint: String,
    pub session_fingerprint: String,
    pub physical_tick: i64,
    pub resident_bytes: usize,
    pub previous_frame_sha256: Option<String>,
    pub pending_device_input: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arc3Sensorimotor {
    session: WorkstationSession,
    sequence: u64,
    previous_frame: Option<Vec<u8>>,
    previous_device_input: bool,
}

impl Arc3Sensorimotor {
    pub fn restore(body_checkpoint: &[u8]) -> Result<Self, Arc3Error> {
        let checkpoint = WorkstationCheckpoint::decode(body_checkpoint)?;
        Ok(Self {
            session: WorkstationSession::from_body_checkpoint(
                checkpoint,
                WorkstationPresentation::default(),
            )?,
            sequence: 0,
            previous_frame: None,
            previous_device_input: false,
        })
    }

    pub fn snapshot(&self) -> Result<Arc3SensorimotorSnapshot, Arc3Error> {
        let read = self.session.read()?;
        Ok(Arc3SensorimotorSnapshot {
            sequence: self.sequence,
            body_fingerprint: read.body.body_fingerprint,
            session_fingerprint: read.session_fingerprint,
            physical_tick: read.body.physical_tick,
            resident_bytes: read.body.resident_bytes,
            previous_frame_sha256: self.previous_frame.as_deref().map(frame_fingerprint),
            pending_device_input: self.previous_device_input,
        })
    }

    pub fn observe(&mut self, frame: Vec<u8>) -> Result<Arc3SensorimotorObservation, Arc3Error> {
        validate_frame(&frame)?;
        let presentation = WorkstationPresentation::with_monitor_frame(MonitorFrame::new(
            ARC3_FRAME_SIDE as u16,
            ARC3_FRAME_SIDE as u16,
            frame.iter().copied().map(palette_luminance).collect(),
        )?);
        let frame_changed = self
            .previous_frame
            .as_ref()
            .map(|previous| previous != &frame);

        let mut next = self.clone();
        next.session.set_presentation(presentation)?;
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
            verify_choice_laws(&trace)
                .map_err(|error| Arc3Error::boundary(format!("choice trace failed: {error}")))?;
            admitted_inputs = admitted_inputs.saturating_add(session.body.admitted_inputs);
            outward_crossings = outward_crossings.saturating_add(session.body.crossings.len());
            physical_work = physical_work.saturating_add(session.body.metrics.physical_work);
            plasticity_updates =
                plasticity_updates.saturating_add(session.body.metrics.plasticity_updates);
            naturally_quiescent &= session.body.naturally_quiescent;
            device_events.extend(session.device_events.iter().cloned());
            let inputs = application_inputs_for(&session.device_events, &session.device_after)?;
            steps.push(compact_step(session, trace));
            match inputs.as_slice() {
                [] => {}
                [input] => {
                    application_input = Some(input.clone());
                    break;
                }
                _ => {
                    return Err(Arc3Error::boundary(
                        "several workstation device inputs reached the application together",
                    ))
                }
            }
        }

        let read = next.session.read()?;
        let returned_previous_device_input = next.previous_device_input
            && steps
                .first()
                .is_some_and(|step| !step.body.boundary_parents.is_empty());
        let result = Arc3SensorimotorObservation {
            sequence: self.sequence,
            frame_sha256: frame_fingerprint(&frame),
            frame_changed,
            returned_previous_device_input,
            workstation_steps: steps.len(),
            admitted_inputs,
            outward_crossings,
            device_events,
            application_input,
            physical_work,
            plasticity_updates,
            resident_bytes: read.body.resident_bytes,
            naturally_quiescent,
            body_fingerprint: read.body.body_fingerprint,
            session_fingerprint: read.session_fingerprint,
            physical_tick: read.body.physical_tick,
            steps,
        };
        next.sequence = next.sequence.saturating_add(1);
        next.previous_frame = Some(frame);
        next.previous_device_input = result.application_input.is_some();
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

fn palette_luminance(value: u8) -> u8 {
    value.saturating_mul(17)
}

fn frame_fingerprint(frame: &[u8]) -> String {
    hex_digest(Sha256::digest(frame))
}

fn sample_fingerprint(sample: &WorldSample) -> String {
    let mut digest = Sha256::new();
    digest.update(b"truelearner-workstation-application-sample-v1");
    for eye in Eye::ALL {
        let field = sample.eye(eye);
        digest.update(field.width().to_le_bytes());
        digest.update(field.height().to_le_bytes());
        digest.update(field.pixels());
    }
    for contact in sample.contacts() {
        digest.update(contact.pressure().to_le_bytes());
        digest.update(contact.slip().to_le_bytes());
    }
    hex_digest(digest.finalize())
}

fn compact_step(session: SessionObservation, trace: Vec<BodyTraceEvent>) -> Arc3PhysicalStep {
    Arc3PhysicalStep {
        sequence: session.sequence,
        sample_sha256: sample_fingerprint(&session.sample),
        body: session.body,
        device_events: session.device_events,
        device_after: session.device_after,
        world_fingerprint: session.world_fingerprint,
        session_fingerprint: session.session_fingerprint,
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

fn application_inputs_for(
    events: &[DeviceEvent],
    device: &DeviceState,
) -> Result<Vec<Arc3DeviceInput>, Arc3Error> {
    events
        .iter()
        .filter_map(|event| {
            action_call_for(event, device)
                .transpose()
                .map(|call| call.map(|call| (event, call)))
        })
        .map(|result| {
            let (event, call) = result?;
            Ok(Arc3DeviceInput {
                event: event.clone(),
                call,
            })
        })
        .collect()
}

fn action_call_for(
    event: &DeviceEvent,
    device: &DeviceState,
) -> Result<Option<Arc3ActionCall>, Arc3Error> {
    let id = match event {
        DeviceEvent::KeyPressed { key } => key_action(*key)?,
        DeviceEvent::Clicked { .. } => Some(6),
        _ => None,
    };
    Ok(id.map(|id| Arc3ActionCall {
        id,
        arguments: if id == 6 {
            let cursor = device.cursor();
            Arc3ActionArguments::Point {
                x: scale_point(cursor.x),
                y: scale_point(cursor.y),
            }
        } else {
            Arc3ActionArguments::Unit
        },
    }))
}

fn key_action(key: u16) -> Result<Option<u8>, Arc3Error> {
    let geometry = WorldGeometry::standard_ansi_104()?;
    let key = geometry
        .key(academy_workstation::KeyId(key))
        .ok_or_else(|| Arc3Error::boundary("workstation emitted an unknown key"))?;
    Ok(match key.label.as_str() {
        "Up" => Some(1),
        "Down" => Some(2),
        "Left" => Some(3),
        "Right" => Some(4),
        "Space" => Some(5),
        "Esc" => Some(7),
        _ => None,
    })
}

fn scale_point(value: i16) -> u8 {
    let scaled = i32::from(value) * 63 / i32::from(truelearner_workstation::BODY_MAX);
    u8::try_from(scaled).unwrap_or(63).min(63)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn completed_body_checkpoint() -> Vec<u8> {
        static CHECKPOINT: OnceLock<Vec<u8>> = OnceLock::new();
        CHECKPOINT
            .get_or_init(|| {
                academy_body::BodyCourse::new(31_001)
                    .unwrap()
                    .run()
                    .unwrap()
                    .body_checkpoint
            })
            .clone()
    }

    #[test]
    fn invalid_frame_is_atomic() {
        let mut sensorimotor = Arc3Sensorimotor::restore(&completed_body_checkpoint()).unwrap();
        let before = sensorimotor.snapshot().unwrap();
        assert!(sensorimotor.observe(vec![16; ARC3_FRAME_PIXELS]).is_err());
        assert_eq!(sensorimotor.snapshot().unwrap(), before);
    }

    #[test]
    fn frozen_requests_replay_exactly_from_the_completed_body() {
        let checkpoint = completed_body_checkpoint();
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
    fn only_device_events_can_form_application_calls() {
        let geometry = WorldGeometry::standard_ansi_104().unwrap();
        let up = geometry
            .keys()
            .iter()
            .find(|key| key.label == "Up")
            .unwrap();
        let call = action_call_for(
            &DeviceEvent::KeyPressed { key: up.id.0 },
            &DeviceState::default(),
        )
        .unwrap();
        assert_eq!(
            call,
            Some(Arc3ActionCall {
                id: 1,
                arguments: Arc3ActionArguments::Unit,
            })
        );
        assert_eq!(
            action_call_for(&DeviceEvent::TextChanged, &DeviceState::default()).unwrap(),
            None
        );
    }
}
