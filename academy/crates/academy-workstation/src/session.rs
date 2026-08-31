use crate::{
    checkpoint::CheckpointParents, DeviceEvent, DeviceState, KeyId, SessionCheckpoint,
    WorkstationPresentation, WorkstationWorld, WorldError, WorldTransition,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_workstation::{
    BodyTraceEvent, MotorEffect, WorkstationCheckpoint, WorkstationHarness, WorkstationRead,
    WorkstationStepObservation, WorldSample,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionObservation {
    pub sequence: u64,
    pub sample: WorldSample,
    pub body: WorkstationStepObservation,
    pub device_events: Vec<DeviceEvent>,
    pub device_after: DeviceState,
    pub world_fingerprint: String,
    pub session_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRead {
    pub sequence: u64,
    pub body: WorkstationRead,
    pub device: DeviceState,
    pub world_fingerprint: String,
    pub session_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkstationSession {
    harness: WorkstationHarness,
    world: WorkstationWorld,
    sequence: u64,
    boundary_parents: Vec<MotorEffect>,
    application_parents: Vec<MotorEffect>,
    progress_parents: Vec<MotorEffect>,
}

impl WorkstationSession {
    pub fn new(seed: u64) -> Result<Self, WorldError> {
        Self::new_with_presentation(seed, WorkstationPresentation::default())
    }

    pub fn new_with_presentation(
        seed: u64,
        presentation: WorkstationPresentation,
    ) -> Result<Self, WorldError> {
        Ok(Self {
            harness: WorkstationHarness::new(seed)?,
            world: WorkstationWorld::new_with_presentation(presentation)?,
            sequence: 0,
            boundary_parents: Vec::new(),
            application_parents: Vec::new(),
            progress_parents: Vec::new(),
        })
    }

    pub fn from_body_checkpoint(
        checkpoint: WorkstationCheckpoint,
        presentation: WorkstationPresentation,
    ) -> Result<Self, WorldError> {
        Self::from_body_checkpoint_with_key_depths(
            checkpoint,
            presentation,
            crate::KEY_PRESS_DEPTH,
            crate::KEY_RELEASE_DEPTH,
        )
    }

    pub fn from_body_checkpoint_with_key_depths(
        checkpoint: WorkstationCheckpoint,
        presentation: WorkstationPresentation,
        key_press_depth: i16,
        key_release_depth: i16,
    ) -> Result<Self, WorldError> {
        Ok(Self {
            harness: WorkstationHarness::restore(checkpoint)?,
            world: WorkstationWorld::new_with_presentation_and_key_depths(
                presentation,
                key_press_depth,
                key_release_depth,
            )?,
            sequence: 0,
            boundary_parents: Vec::new(),
            application_parents: Vec::new(),
            progress_parents: Vec::new(),
        })
    }

    pub fn step(&mut self) -> Result<SessionObservation, WorldError> {
        let (observation, _) = self.step_internal(false)?;
        Ok(observation)
    }

    pub fn step_traced(&mut self) -> Result<(SessionObservation, Vec<BodyTraceEvent>), WorldError> {
        self.step_internal(true)
    }

    /// Lets an already-caused world return arrive without opening a fresh
    /// chance to move.
    pub fn settle(&mut self) -> Result<SessionObservation, WorldError> {
        let sample = self.world.sense(self.harness.state())?;
        let mut harness = self.harness.clone();
        let body = harness.settle_with_causal_parents(
            sample.clone(),
            &self.boundary_parents,
            &self.progress_parents,
        )?;
        let mut world = self.world.clone();
        let transition = world.advance_observation(&body);
        let sequence = self.sequence;
        let next_sequence = sequence.saturating_add(1);
        let world_fingerprint = world.fingerprint()?;
        let session_fingerprint =
            composed_session_fingerprint(next_sequence, &body.body_fingerprint, &world_fingerprint);
        let observation = SessionObservation {
            sequence,
            sample,
            body,
            device_events: transition.events,
            device_after: world.device().clone(),
            world_fingerprint,
            session_fingerprint,
        };
        *self = Self {
            harness,
            world,
            sequence: next_sequence,
            boundary_parents: transition.boundary_parents,
            application_parents: transition.application_parents,
            progress_parents: transition.progress_parents,
        };
        Ok(observation)
    }

    /// Advances a visibly external key transition for an Academy
    /// demonstration. The learner's simultaneous motor output does not cause
    /// this device event, so the transition returns with no motor parent.
    pub fn step_with_external_key(
        &mut self,
        key: KeyId,
        pressed_after: bool,
    ) -> Result<SessionObservation, WorldError> {
        let sample = self.world.sense(self.harness.state())?;
        let mut harness = self.harness.clone();
        let body = harness.step_with_causal_parents(
            sample.clone(),
            &self.boundary_parents,
            &self.progress_parents,
        )?;
        let mut world = self.world.clone();
        let pressed_before = world.device().keys_down().any(|candidate| candidate == key);
        let events = world.advance_external_key(key, pressed_before, pressed_after)?;
        let transition = WorldTransition::external(events);
        let sequence = self.sequence;
        let next_sequence = sequence.saturating_add(1);
        let world_fingerprint = world.fingerprint()?;
        let session_fingerprint =
            composed_session_fingerprint(next_sequence, &body.body_fingerprint, &world_fingerprint);
        let observation = SessionObservation {
            sequence,
            sample,
            body,
            device_events: transition.events,
            device_after: world.device().clone(),
            world_fingerprint,
            session_fingerprint,
        };
        *self = Self {
            harness,
            world,
            sequence: next_sequence,
            boundary_parents: transition.boundary_parents,
            application_parents: transition.application_parents,
            progress_parents: transition.progress_parents,
        };
        Ok(observation)
    }

    fn step_internal(
        &mut self,
        traced: bool,
    ) -> Result<(SessionObservation, Vec<BodyTraceEvent>), WorldError> {
        let sample = self.world.sense(self.harness.state())?;
        let mut harness = self.harness.clone();
        let (body, trace) = if traced {
            harness.step_traced_with_causal_parents(
                sample.clone(),
                &self.boundary_parents,
                &self.progress_parents,
            )?
        } else {
            (
                harness.step_with_causal_parents(
                    sample.clone(),
                    &self.boundary_parents,
                    &self.progress_parents,
                )?,
                Vec::new(),
            )
        };
        let mut world = self.world.clone();
        let transition = world.advance_observation(&body);
        let sequence = self.sequence;
        let next_sequence = self.sequence.saturating_add(1);
        let world_fingerprint = world.fingerprint()?;
        let session_fingerprint =
            composed_session_fingerprint(next_sequence, &body.body_fingerprint, &world_fingerprint);
        let observation = SessionObservation {
            sequence,
            sample,
            body,
            device_events: transition.events,
            device_after: world.device().clone(),
            world_fingerprint,
            session_fingerprint,
        };
        *self = Self {
            harness,
            world,
            sequence: next_sequence,
            boundary_parents: transition.boundary_parents,
            application_parents: transition.application_parents,
            progress_parents: transition.progress_parents,
        };
        Ok((observation, trace))
    }

    pub fn read(&self) -> Result<SessionRead, WorldError> {
        let body = self.harness.read()?;
        let world_fingerprint = self.world.fingerprint()?;
        let session_fingerprint =
            composed_session_fingerprint(self.sequence, &body.body_fingerprint, &world_fingerprint);
        Ok(SessionRead {
            sequence: self.sequence,
            body,
            device: self.world.device().clone(),
            world_fingerprint,
            session_fingerprint,
        })
    }

    pub fn set_presentation(
        &mut self,
        presentation: WorkstationPresentation,
    ) -> Result<(), WorldError> {
        let mut next = self.clone();
        let changed = next.world.presentation() != &presentation;
        next.world.set_presentation(presentation)?;
        if changed {
            next.boundary_parents.clone_from(&next.application_parents);
        }
        next.application_parents.clear();
        *self = next;
        Ok(())
    }

    pub fn save(&self) -> Result<SessionCheckpoint, WorldError> {
        Ok(SessionCheckpoint::new(
            self.harness.save()?.canonical_bytes()?,
            self.world.device_clone(),
            self.world.presentation().clone(),
            (self.world.key_press_depth(), self.world.key_release_depth()),
            self.sequence,
            self.world.asset_digest(),
            CheckpointParents {
                boundary: self.boundary_parents.clone(),
                application: self.application_parents.clone(),
                progress: self.progress_parents.clone(),
            },
        ))
    }

    pub fn restore(checkpoint: SessionCheckpoint) -> Result<Self, WorldError> {
        let payload = checkpoint.open()?;
        let harness_checkpoint =
            truelearner_workstation::WorkstationCheckpoint::decode(&payload.harness)?;
        Ok(Self {
            harness: WorkstationHarness::restore(harness_checkpoint)?,
            world: WorkstationWorld::from_parts(
                payload.device,
                payload.presentation,
                payload.key_press_depth,
                payload.key_release_depth,
                payload.asset_digest,
            )?,
            sequence: payload.sequence,
            boundary_parents: payload.boundary_parents,
            application_parents: payload.application_parents,
            progress_parents: payload.progress_parents,
        })
    }

    pub fn body_checkpoint(&self) -> Result<WorkstationCheckpoint, WorldError> {
        Ok(self.harness.save()?)
    }
}

fn composed_session_fingerprint(sequence: u64, body: &str, world: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(b"truelearner-workstation-session-v7");
    digest.update(sequence.to_le_bytes());
    digest.update(body.as_bytes());
    digest.update(world.as_bytes());
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MonitorFrame;
    use truelearner_workstation::{BodyControl, Direction};

    #[test]
    fn presentation_change_clears_stale_ancestry_when_no_exact_parent_exists() {
        let mut session = WorkstationSession::new(71_015).unwrap();
        session.boundary_parents.push(MotorEffect {
            at: 1,
            control: BodyControl::PalmDepth {
                direction: Direction::Increase,
            },
            impulse: 16,
            cause: 2,
        });

        session
            .set_presentation(WorkstationPresentation::with_monitor_frame(
                MonitorFrame::new(8, 8, vec![177; 64]).unwrap(),
            ))
            .unwrap();

        assert!(session.boundary_parents.is_empty());
    }
}
