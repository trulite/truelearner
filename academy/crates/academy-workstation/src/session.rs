use crate::{
    DeviceEvent, DeviceState, SessionCheckpoint, WorkstationPresentation, WorkstationWorld,
    WorldError,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_workstation::{
    WorkstationHarness, WorkstationRead, WorkstationStepObservation, WorldSample,
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
        })
    }

    pub fn step(&mut self) -> Result<SessionObservation, WorldError> {
        let sample = self.world.sense(self.harness.state())?;
        let (harness, body) = self.harness.transition(sample.clone())?;
        let mut world = self.world.clone();
        let device_events = world.advance(&body.state_before, &body.state_after);
        let sequence = self.sequence;
        let next_sequence = self.sequence.saturating_add(1);
        let world_fingerprint = world.fingerprint()?;
        let session_fingerprint =
            composed_session_fingerprint(next_sequence, &body.body_fingerprint, &world_fingerprint);
        let observation = SessionObservation {
            sequence,
            sample,
            body,
            device_events,
            device_after: world.device().clone(),
            world_fingerprint,
            session_fingerprint,
        };
        *self = Self {
            harness,
            world,
            sequence: next_sequence,
        };
        Ok(observation)
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
        next.world.set_presentation(presentation)?;
        *self = next;
        Ok(())
    }

    pub fn save(&self) -> Result<SessionCheckpoint, WorldError> {
        Ok(SessionCheckpoint::new(
            self.harness.save()?.canonical_bytes()?,
            self.world.device_clone(),
            self.world.presentation(),
            self.sequence,
            self.world.asset_digest(),
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
                payload.asset_digest,
            )?,
            sequence: payload.sequence,
        })
    }
}

fn composed_session_fingerprint(sequence: u64, body: &str, world: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(b"truelearner-workstation-session-v2");
    digest.update(sequence.to_le_bytes());
    digest.update(body.as_bytes());
    digest.update(world.as_bytes());
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
