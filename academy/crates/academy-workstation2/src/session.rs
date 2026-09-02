use crate::{BezelControl, DeviceEvent, Workstation2};
use serde::{Deserialize, Serialize};
use truelearner_workstation::{
    BodyTraceEvent, ColorField, LightField, WorkstationCheckpoint, WorkstationError,
    WorkstationHarness, WorkstationRead, WorkstationStepObservation, WorldSample,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workstation2Observation {
    pub sequence: u64,
    pub sample: WorldSample,
    pub body: WorkstationStepObservation,
    pub device_events: Vec<DeviceEvent>,
    pub text: String,
    pub scale: i16,
}

#[derive(Clone, Debug)]
pub struct Workstation2Session {
    harness: WorkstationHarness,
    world: Workstation2,
    sequence: u64,
}

impl Workstation2Session {
    pub fn cold_control(seed: u64, keyboard_shift: i16) -> Result<Self, WorkstationError> {
        Ok(Self {
            harness: WorkstationHarness::new(seed)?,
            world: Workstation2::new(keyboard_shift),
            sequence: 0,
        })
    }

    pub fn from_checkpoint(
        checkpoint: WorkstationCheckpoint,
        keyboard_shift: i16,
    ) -> Result<Self, WorkstationError> {
        Self::with_world(checkpoint, Workstation2::new(keyboard_shift))
    }

    pub fn with_world(
        checkpoint: WorkstationCheckpoint,
        world: Workstation2,
    ) -> Result<Self, WorkstationError> {
        Ok(Self {
            harness: WorkstationHarness::restore(checkpoint)?,
            world,
            sequence: 0,
        })
    }

    pub fn world(&self) -> &Workstation2 {
        &self.world
    }

    pub fn replace_application_frame(&mut self, frame: LightField) -> Result<(), WorkstationError> {
        self.world.replace_pixels(frame)
    }

    pub fn replace_color_application_frame(
        &mut self,
        frame: ColorField,
    ) -> Result<(), WorkstationError> {
        self.world.replace_color_pixels(frame)
    }

    pub fn replace_game_surface(
        &mut self,
        frame: ColorField,
        point_enabled: bool,
        enabled: &[BezelControl],
    ) -> Result<(), WorkstationError> {
        self.world
            .replace_game_surface(frame, point_enabled, enabled)
    }

    pub fn step(&mut self) -> Result<Workstation2Observation, WorkstationError> {
        // This is the complete organism input. Device events and application
        // state never cross this call boundary.
        let sample = self.world.sense(self.harness.state())?;
        let body = self.harness.step(sample.clone())?;
        let device_events = self.world.advance(&body.state_after);
        let observation = Workstation2Observation {
            sequence: self.sequence,
            sample,
            body,
            device_events,
            text: self.world.text().to_owned(),
            scale: self.world.scale(),
        };
        self.sequence = self.sequence.saturating_add(1);
        Ok(observation)
    }

    /// Observer-equivalent form of [`Self::step`] with the physical body trace.
    pub fn step_traced(
        &mut self,
    ) -> Result<(Workstation2Observation, Vec<BodyTraceEvent>), WorkstationError> {
        let sample = self.world.sense(self.harness.state())?;
        let (body, trace) = self.harness.step_traced(sample.clone())?;
        let device_events = self.world.advance(&body.state_after);
        let observation = Workstation2Observation {
            sequence: self.sequence,
            sample,
            body,
            device_events,
            text: self.world.text().to_owned(),
            scale: self.world.scale(),
        };
        self.sequence = self.sequence.saturating_add(1);
        Ok((observation, trace))
    }

    pub fn body_checkpoint(&self) -> Result<WorkstationCheckpoint, WorkstationError> {
        self.harness.save()
    }

    pub fn body_read(&self) -> Result<WorkstationRead, WorkstationError> {
        self.harness.read()
    }
}
