use crate::{Arc3ActionArguments, Arc3ActionCall};
use academy_workstation2::{
    BezelControl, DeviceEvent, Workstation2, Workstation2Observation, Workstation2Session,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fmt;
use truelearner_workstation::{
    verify_choice_contract, BodyTraceEvent, ColorField, Eye, Rgb, WorkstationCheckpoint,
    WorkstationError, WorkstationStepObservation, WorldSample,
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
}

#[derive(Clone, Debug)]
pub struct Arc3Sensorimotor {
    session: Workstation2Session,
    sequence: u64,
    previous_frame: Option<Vec<u8>>,
}

impl Arc3Sensorimotor {
    pub fn restore(body_checkpoint: &[u8]) -> Result<Self, Arc3Error> {
        let checkpoint = WorkstationCheckpoint::decode(body_checkpoint)?;
        let frame = ColorField::filled(
            ARC3_FRAME_SIDE as u16,
            ARC3_FRAME_SIDE as u16,
            palette_rgb(0),
        )?;
        Ok(Self {
            session: Workstation2Session::with_world(
                checkpoint,
                Workstation2::with_game_surface(frame, false, &[])?,
            )?,
            sequence: 0,
            previous_frame: None,
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
        })
    }

    pub fn observe(
        &mut self,
        frame: Vec<u8>,
        point_enabled: bool,
        enabled: &[BezelControl],
    ) -> Result<Arc3SensorimotorObservation, Arc3Error> {
        validate_frame(&frame)?;
        let application_frame = application_frame(&frame)?;
        let frame_changed = self
            .previous_frame
            .as_ref()
            .map(|previous| previous != &frame);

        let mut next = self.clone();
        next.session
            .replace_game_surface(application_frame, point_enabled, enabled)?;
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
            let inputs = session
                .device_events
                .iter()
                .filter_map(|event| activation_input_for(event).transpose())
                .collect::<Result<Vec<_>, _>>()?;
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

fn application_frame(frame: &[u8]) -> Result<ColorField, Arc3Error> {
    Ok(ColorField::new(
        ARC3_FRAME_SIDE as u16,
        ARC3_FRAME_SIDE as u16,
        frame.iter().copied().map(palette_rgb).collect(),
    )?)
}

fn palette_rgb(value: u8) -> Rgb {
    const PALETTE: [Rgb; ARC3_PALETTE_SIZE as usize] = [
        Rgb::new(255, 255, 255),
        Rgb::new(204, 204, 204),
        Rgb::new(153, 153, 153),
        Rgb::new(102, 102, 102),
        Rgb::new(51, 51, 51),
        Rgb::new(0, 0, 0),
        Rgb::new(229, 58, 163),
        Rgb::new(255, 123, 204),
        Rgb::new(249, 60, 49),
        Rgb::new(30, 147, 255),
        Rgb::new(136, 216, 241),
        Rgb::new(255, 220, 0),
        Rgb::new(255, 133, 27),
        Rgb::new(146, 18, 49),
        Rgb::new(79, 204, 48),
        Rgb::new(163, 86, 214),
    ];
    PALETTE[usize::from(value.min(ARC3_PALETTE_SIZE - 1))]
}

fn frame_fingerprint(frame: &[u8]) -> String {
    hex_digest(Sha256::digest(frame))
}

fn sample_fingerprint(sample: &WorldSample) -> String {
    let mut digest = Sha256::new();
    digest.update(b"academy-workstation2-application-sample-v3");
    for eye in Eye::ALL {
        let field = sample.eye(eye);
        for channel in [field.global(), field.foveal()] {
            digest.update(channel.width().to_le_bytes());
            digest.update(channel.height().to_le_bytes());
            digest.update(channel.pixels());
        }
        for signal in field.foveal_chromatic().pixels() {
            digest.update(signal.red_green().to_le_bytes());
            digest.update(signal.blue_yellow().to_le_bytes());
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

fn activation_input_for(event: &DeviceEvent) -> Result<Option<Arc3DeviceInput>, Arc3Error> {
    match *event {
        DeviceEvent::ContentActivated {
            column,
            row,
            touch: _,
        } => {
            let x = u8::try_from(column)
                .ok()
                .filter(|value| *value < ARC3_FRAME_SIDE as u8)
                .ok_or_else(|| Arc3Error::boundary("content column is outside the ARC frame"))?;
            let y = u8::try_from(row)
                .ok()
                .filter(|value| *value < ARC3_FRAME_SIDE as u8)
                .ok_or_else(|| Arc3Error::boundary("content row is outside the ARC frame"))?;
            Ok(Some(Arc3DeviceInput {
                event: *event,
                call: Arc3ActionCall {
                    id: 6,
                    arguments: Arc3ActionArguments::Point { x, y },
                },
            }))
        }
        DeviceEvent::ControlActivated { control, .. } => {
            Ok(control_action(control).map(|id| Arc3DeviceInput {
                event: *event,
                call: Arc3ActionCall {
                    id,
                    arguments: Arc3ActionArguments::Unit,
                },
            }))
        }
        DeviceEvent::TouchStarted { .. }
        | DeviceEvent::TouchMoved { .. }
        | DeviceEvent::TouchEnded { .. } => Ok(None),
    }
}

fn control_action(control: BezelControl) -> Option<u8> {
    match control {
        BezelControl::North => Some(1),
        BezelControl::South => Some(2),
        BezelControl::West => Some(3),
        BezelControl::East => Some(4),
        BezelControl::Primary => Some(5),
        BezelControl::Back => Some(7),
        BezelControl::Reset => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use academy_workstation2::{ScreenPoint, TouchId};

    fn sensorimotor_checkpoint() -> Vec<u8> {
        truelearner_workstation::WorkstationHarness::new(31_001)
            .unwrap()
            .save()
            .unwrap()
            .canonical_bytes()
            .unwrap()
    }

    fn touch() -> TouchId {
        TouchId::new(0).unwrap()
    }

    #[test]
    fn palette_becomes_screen_rgb() {
        let frame = (0..16).cycle().take(ARC3_FRAME_PIXELS).collect::<Vec<_>>();
        let converted = application_frame(&frame).unwrap();
        assert_eq!(converted.width(), 64);
        assert_eq!(converted.height(), 64);
        assert_eq!(
            &converted.pixels()[..16],
            &[
                Rgb::new(255, 255, 255),
                Rgb::new(204, 204, 204),
                Rgb::new(153, 153, 153),
                Rgb::new(102, 102, 102),
                Rgb::new(51, 51, 51),
                Rgb::new(0, 0, 0),
                Rgb::new(229, 58, 163),
                Rgb::new(255, 123, 204),
                Rgb::new(249, 60, 49),
                Rgb::new(30, 147, 255),
                Rgb::new(136, 216, 241),
                Rgb::new(255, 220, 0),
                Rgb::new(255, 133, 27),
                Rgb::new(146, 18, 49),
                Rgb::new(79, 204, 48),
                Rgb::new(163, 86, 214),
            ]
        );
    }

    #[test]
    fn every_palette_entry_has_a_distinct_retinal_code() {
        let codes = (0..ARC3_PALETTE_SIZE)
            .map(|value| {
                let colour = palette_rgb(value);
                let opponents = colour.opponents();
                (
                    colour.luminance(),
                    opponents.red_green(),
                    opponents.blue_yellow(),
                )
            })
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(codes.len(), ARC3_PALETTE_SIZE as usize);
    }

    #[test]
    fn invalid_frame_is_atomic() {
        let mut sensorimotor = Arc3Sensorimotor::restore(&sensorimotor_checkpoint()).unwrap();
        let before = sensorimotor.snapshot().unwrap();
        assert!(sensorimotor
            .observe(vec![16; ARC3_FRAME_PIXELS], false, &[])
            .is_err());
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
            first
                .observe(frame.clone(), true, &[BezelControl::Primary])
                .unwrap(),
            replay
                .observe(frame, true, &[BezelControl::Primary])
                .unwrap()
        );
        assert_eq!(first.snapshot().unwrap(), replay.snapshot().unwrap());
    }

    #[test]
    fn content_activation_maps_exact_coordinates_to_action_six() {
        let event = DeviceEvent::ContentActivated {
            touch: touch(),
            column: 12,
            row: 34,
        };
        let input = activation_input_for(&event).unwrap().unwrap();
        assert_eq!(input.event, event);
        assert_eq!(input.call.id, 6);
        assert_eq!(
            input.call.arguments,
            Arc3ActionArguments::Point { x: 12, y: 34 }
        );
    }

    #[test]
    fn generic_controls_map_to_all_six_unit_actions() {
        for (control, expected) in [
            (BezelControl::North, 1),
            (BezelControl::South, 2),
            (BezelControl::West, 3),
            (BezelControl::East, 4),
            (BezelControl::Primary, 5),
            (BezelControl::Back, 7),
        ] {
            let event = DeviceEvent::ControlActivated {
                touch: touch(),
                control,
            };
            let input = activation_input_for(&event).unwrap().unwrap();
            assert_eq!(input.call.id, expected);
            assert_eq!(input.call.arguments, Arc3ActionArguments::Unit);
        }
    }

    #[test]
    fn raw_touch_and_reset_do_not_emit_arc_actions() {
        let raw = DeviceEvent::TouchStarted {
            touch: touch(),
            at: ScreenPoint { x: 512, y: 512 },
        };
        let reset = DeviceEvent::ControlActivated {
            touch: touch(),
            control: BezelControl::Reset,
        };
        assert_eq!(activation_input_for(&raw).unwrap(), None);
        assert_eq!(activation_input_for(&reset).unwrap(), None);
    }

    #[test]
    fn out_of_frame_content_activation_is_rejected() {
        let event = DeviceEvent::ContentActivated {
            touch: touch(),
            column: 64,
            row: 0,
        };
        assert!(activation_input_for(&event).is_err());
    }
}
