use crate::{DeviceEvent, DeviceState, SessionCheckpoint, WorkstationWorld, WorldError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[cfg(feature = "research")]
use truelearner_workstation::{ResearchHarnessConfig, ResearchOpportunityIncidence};
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
        Ok(Self {
            harness: WorkstationHarness::new(seed)?,
            world: WorkstationWorld::new()?,
            sequence: 0,
        })
    }

    #[cfg(feature = "research")]
    pub fn new_research(seed: u64, config: ResearchHarnessConfig) -> Result<Self, WorldError> {
        Ok(Self {
            harness: WorkstationHarness::new_research(seed, config)?,
            world: WorkstationWorld::new()?,
            sequence: 0,
        })
    }

    pub fn step(&mut self) -> Result<SessionObservation, WorldError> {
        let mut next = self.clone();
        let read = next.harness.read()?;
        let sample = next.world.sense(&read.state)?;
        let body = next.harness.step(sample.clone())?;
        let device_events = next.world.advance(&body.state_before, &body.state_after);
        let sequence = next.sequence;
        next.sequence = next.sequence.saturating_add(1);
        let observation = SessionObservation {
            sequence,
            sample,
            body,
            device_events,
            device_after: next.world.device().clone(),
            world_fingerprint: next.world.fingerprint()?,
            session_fingerprint: next.fingerprint()?,
        };
        *self = next;
        Ok(observation)
    }

    pub fn read(&self) -> Result<SessionRead, WorldError> {
        Ok(SessionRead {
            sequence: self.sequence,
            body: self.harness.read()?,
            device: self.world.device().clone(),
            world_fingerprint: self.world.fingerprint()?,
            session_fingerprint: self.fingerprint()?,
        })
    }

    pub fn save(&self) -> Result<SessionCheckpoint, WorldError> {
        Ok(SessionCheckpoint::new(
            self.harness.save()?.canonical_bytes()?,
            self.world.device_clone(),
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
            world: WorkstationWorld::from_parts(payload.device, payload.asset_digest)?,
            sequence: payload.sequence,
        })
    }

    #[cfg(feature = "research")]
    pub fn restore_research(
        checkpoint: SessionCheckpoint,
        opportunity_incidence: ResearchOpportunityIncidence,
    ) -> Result<Self, WorldError> {
        let payload = checkpoint.open()?;
        let harness_checkpoint =
            truelearner_workstation::WorkstationCheckpoint::decode(&payload.harness)?;
        Ok(Self {
            harness: WorkstationHarness::restore_research(
                harness_checkpoint,
                opportunity_incidence,
            )?,
            world: WorkstationWorld::from_parts(payload.device, payload.asset_digest)?,
            sequence: payload.sequence,
        })
    }

    #[cfg(feature = "research")]
    pub fn restore_research_config(
        checkpoint: SessionCheckpoint,
        config: ResearchHarnessConfig,
    ) -> Result<Self, WorldError> {
        let payload = checkpoint.open()?;
        let harness_checkpoint =
            truelearner_workstation::WorkstationCheckpoint::decode(&payload.harness)?;
        Ok(Self {
            harness: WorkstationHarness::restore_research_config(harness_checkpoint, config)?,
            world: WorkstationWorld::from_parts(payload.device, payload.asset_digest)?,
            sequence: payload.sequence,
        })
    }

    fn fingerprint(&self) -> Result<String, WorldError> {
        let digest = Sha256::digest(self.save()?.canonical_bytes()?);
        Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
    }
}
