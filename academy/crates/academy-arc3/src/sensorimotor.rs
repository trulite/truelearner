use crate::{Arc3ActionArguments, Arc3ActionCall, Arc3ActionCatalog};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fmt;
use truelearner_workstation::{
    verify_choice_laws, BodyControl, BodyTraceEvent, ContactSample, Digit, Direction, LightField,
    MotorEffect, Point, WorkstationError, WorkstationHarness, WorkstationStepObservation,
    WorldSample, BODY_MAX, TOUCH_SITES,
};

pub const ARC3_FRAME_SIDE: usize = 64;
pub const ARC3_FRAME_PIXELS: usize = ARC3_FRAME_SIDE * ARC3_FRAME_SIDE;
pub const ARC3_PALETTE_SIZE: u8 = 16;
const RETINA_SIDE: usize = 9;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3ActionWitness {
    pub crossing: MotorEffect,
    pub action: u8,
    pub offered: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3SensorimotorObservation {
    pub sequence: u64,
    pub frame_sha256: String,
    pub frame_changed: Option<bool>,
    pub retina: Vec<u8>,
    pub retina_changed: Option<bool>,
    pub returned_previous_action: bool,
    pub admitted_inputs: usize,
    pub outward_crossings: usize,
    pub action_witnesses: Vec<Arc3ActionWitness>,
    pub call: Option<Arc3ActionCall>,
    pub physical_work: u64,
    pub plasticity_updates: u64,
    pub resident_bytes: usize,
    pub naturally_quiescent: bool,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub observation: WorkstationStepObservation,
    pub trace: Vec<BodyTraceEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3SensorimotorSnapshot {
    pub sequence: u64,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub resident_bytes: usize,
    pub previous_frame_sha256: Option<String>,
    pub pending_action: Option<MotorEffect>,
}

#[derive(Clone)]
pub struct Arc3Sensorimotor {
    harness: WorkstationHarness,
    sequence: u64,
    previous_frame: Option<Vec<u8>>,
    previous_retina: Option<Vec<u8>>,
    previous_action: Option<MotorEffect>,
}

impl Arc3Sensorimotor {
    pub fn new(seed: u64) -> Result<Self, Arc3Error> {
        Ok(Self {
            harness: WorkstationHarness::new(seed)?,
            sequence: 0,
            previous_frame: None,
            previous_retina: None,
            previous_action: None,
        })
    }

    pub fn snapshot(&self) -> Result<Arc3SensorimotorSnapshot, Arc3Error> {
        let read = self.harness.read()?;
        Ok(Arc3SensorimotorSnapshot {
            sequence: self.sequence,
            body_fingerprint: checkpoint_fingerprint(&self.harness)?,
            physical_tick: read.physical_tick,
            resident_bytes: read.resident_bytes,
            previous_frame_sha256: self.previous_frame.as_deref().map(frame_fingerprint),
            pending_action: self.previous_action,
        })
    }

    pub fn observe(
        &mut self,
        frame: Vec<u8>,
        actions: &Arc3ActionCatalog,
    ) -> Result<Arc3SensorimotorObservation, Arc3Error> {
        validate_frame(&frame)?;
        actions.validate()?;
        let field = LightField::new(64, 64, frame.clone())?;
        let retina = retinal_samples(&field);
        let sample = WorldSample::new(
            [field.clone(), field],
            [ContactSample::default(); TOUCH_SITES],
        )?;
        let frame_changed = self
            .previous_frame
            .as_ref()
            .map(|previous| previous != &frame);
        let retina_changed = self
            .previous_retina
            .as_ref()
            .map(|previous| previous != &retina);
        let boundary_parents = if frame_changed == Some(true) {
            self.previous_action.iter().copied().collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let mut next = self.harness.clone();
        let (observation, trace) =
            next.step_traced_with_boundary_parents(sample, &boundary_parents)?;
        verify_choice_laws(&trace)
            .map_err(|error| Arc3Error::boundary(format!("choice trace failed: {error}")))?;
        let action_witnesses = observation
            .crossings
            .iter()
            .filter_map(|crossing| {
                physical_action(crossing.control).map(|action| Arc3ActionWitness {
                    crossing: *crossing,
                    action,
                    offered: actions.contains(action),
                })
            })
            .collect::<Vec<_>>();
        let offered = action_witnesses
            .iter()
            .filter(|witness| witness.offered)
            .collect::<Vec<_>>();
        let call = match offered.as_slice() {
            [] => None,
            [witness] => Some(action_call(witness.action, &observation)),
            _ => {
                return Err(Arc3Error::boundary(
                    "several physical ARC actuators crossed in one turn",
                ))
            }
        };
        let next_parent = call.and_then(|call| {
            offered
                .iter()
                .find(|witness| witness.action == call.id)
                .map(|witness| witness.crossing)
        });
        let body_fingerprint = checkpoint_fingerprint(&next)?;
        let result = Arc3SensorimotorObservation {
            sequence: self.sequence,
            frame_sha256: frame_fingerprint(&frame),
            frame_changed,
            retina,
            retina_changed,
            returned_previous_action: !boundary_parents.is_empty(),
            admitted_inputs: observation.admitted_inputs,
            outward_crossings: observation.crossings.len(),
            action_witnesses,
            call,
            physical_work: observation.metrics.physical_work,
            plasticity_updates: observation.metrics.plasticity_updates,
            resident_bytes: observation.metrics.resident_bytes,
            naturally_quiescent: observation.naturally_quiescent,
            body_fingerprint,
            physical_tick: observation.physical_tick,
            observation,
            trace,
        };
        self.harness = next;
        self.sequence = self.sequence.saturating_add(1);
        self.previous_frame = Some(frame);
        self.previous_retina = Some(result.retina.clone());
        self.previous_action = next_parent;
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

fn frame_fingerprint(frame: &[u8]) -> String {
    hex_digest(Sha256::digest(frame))
}

fn checkpoint_fingerprint(harness: &WorkstationHarness) -> Result<String, Arc3Error> {
    Ok(hex_digest(Sha256::digest(
        harness.save()?.canonical_bytes()?,
    )))
}

fn hex_digest(digest: impl AsRef<[u8]>) -> String {
    digest
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn retinal_samples(field: &LightField) -> Vec<u8> {
    (0..RETINA_SIDE)
        .flat_map(|row| {
            (0..RETINA_SIDE).map(move |column| {
                let coordinate = |index: usize| {
                    let numerator = index * BODY_MAX as usize + RETINA_SIDE - 2;
                    i16::try_from(numerator / (RETINA_SIDE - 1))
                        .expect("retinal coordinate is bounded")
                };
                field.sample(
                    Point::new(coordinate(column), coordinate(row))
                        .expect("retinal point is bounded"),
                )
            })
        })
        .collect()
}

fn physical_action(control: BodyControl) -> Option<u8> {
    match control {
        BodyControl::PalmVertical {
            direction: Direction::Decrease,
        }
        | BodyControl::Wrist {
            direction: Direction::Decrease,
        } => Some(1),
        BodyControl::PalmVertical {
            direction: Direction::Increase,
        }
        | BodyControl::Wrist {
            direction: Direction::Increase,
        } => Some(2),
        BodyControl::PalmHorizontal {
            direction: Direction::Decrease,
        }
        | BodyControl::Spread {
            direction: Direction::Decrease,
        } => Some(3),
        BodyControl::PalmHorizontal {
            direction: Direction::Increase,
        }
        | BodyControl::Spread {
            direction: Direction::Increase,
        } => Some(4),
        BodyControl::FingerFlexion {
            digit: Digit::Index | Digit::Middle | Digit::Ring | Digit::Little,
            direction: Direction::Increase,
        } => Some(5),
        BodyControl::PalmDepth {
            direction: Direction::Increase,
        } => Some(6),
        BodyControl::PalmDepth {
            direction: Direction::Decrease,
        }
        | BodyControl::FingerFlexion {
            digit: Digit::Index | Digit::Middle | Digit::Ring | Digit::Little,
            direction: Direction::Decrease,
        } => Some(7),
        BodyControl::EyeHorizontal { .. }
        | BodyControl::EyeVertical { .. }
        | BodyControl::ThumbOpposition { .. }
        | BodyControl::FingerFlexion {
            digit: Digit::Thumb,
            ..
        } => None,
    }
}

fn action_call(action: u8, observation: &WorkstationStepObservation) -> Arc3ActionCall {
    let arguments = if action == 6 {
        let palm = observation.state_after.hand().palm();
        Arc3ActionArguments::Point {
            x: scale_point(palm.x()),
            y: scale_point(palm.y()),
        }
    } else {
        Arc3ActionArguments::Unit
    };
    Arc3ActionCall {
        id: action,
        arguments,
    }
}

fn scale_point(value: i16) -> u8 {
    let scaled = i32::from(value) * 63 / i32::from(BODY_MAX);
    u8::try_from(scaled).unwrap_or(63).min(63)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Arc3ActionOffer, Arc3ActionSchema};

    fn catalog() -> Arc3ActionCatalog {
        Arc3ActionCatalog {
            offers: (1..=7)
                .map(|id| Arc3ActionOffer {
                    id,
                    schema: if id == 6 {
                        Arc3ActionSchema::Point {
                            width: 64,
                            height: 64,
                        }
                    } else {
                        Arc3ActionSchema::Unit
                    },
                })
                .collect(),
        }
    }

    #[test]
    fn invalid_frame_is_atomic() {
        let mut sensorimotor = Arc3Sensorimotor::new(205).unwrap();
        let before = sensorimotor.snapshot().unwrap();
        assert!(sensorimotor
            .observe(vec![16; ARC3_FRAME_PIXELS], &catalog())
            .is_err());
        assert_eq!(sensorimotor.snapshot().unwrap(), before);
    }

    #[test]
    fn frozen_requests_replay_exactly() {
        let mut first = Arc3Sensorimotor::new(205).unwrap();
        let mut replay = Arc3Sensorimotor::new(205).unwrap();
        let mut frames = vec![vec![0; ARC3_FRAME_PIXELS]; 3];
        frames[1][32 * ARC3_FRAME_SIDE + 32] = 9;
        frames[2][32 * ARC3_FRAME_SIDE + 40] = 5;
        for frame in frames {
            assert_eq!(
                first.observe(frame.clone(), &catalog()).unwrap(),
                replay.observe(frame, &catalog()).unwrap()
            );
        }
        assert_eq!(first.snapshot().unwrap(), replay.snapshot().unwrap());
    }

    #[test]
    fn sampled_retina_is_an_explicit_projection_of_the_full_frame() {
        let mut frame = vec![0; ARC3_FRAME_PIXELS];
        frame[63 * ARC3_FRAME_SIDE + 63] = 15;
        let field = LightField::new(64, 64, frame).unwrap();
        let retina = retinal_samples(&field);
        assert_eq!(retina.len(), RETINA_SIDE * RETINA_SIDE);
        assert_eq!(retina[RETINA_SIDE * RETINA_SIDE - 1], 15);
    }

    #[test]
    fn changed_frame_returns_only_to_the_actual_previous_arc_crossing() {
        let mut sensorimotor = Arc3Sensorimotor::new(205).unwrap();
        let first = sensorimotor
            .observe(vec![0; ARC3_FRAME_PIXELS], &catalog())
            .unwrap();
        let call = first
            .call
            .expect("the frozen body emits one offered action");
        let parent = first
            .action_witnesses
            .iter()
            .find(|witness| witness.action == call.id)
            .expect("the call has a physical crossing witness")
            .crossing;

        let mut changed = vec![0; ARC3_FRAME_PIXELS];
        changed[ARC3_FRAME_PIXELS / 2] = 1;
        let returned = sensorimotor.observe(changed.clone(), &catalog()).unwrap();
        assert!(returned.returned_previous_action);
        assert_eq!(returned.observation.boundary_parents, vec![parent]);

        let repeated = sensorimotor.observe(changed, &catalog()).unwrap();
        assert!(!repeated.returned_previous_action);
        assert!(repeated.observation.boundary_parents.is_empty());
    }
}
