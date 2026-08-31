#![forbid(unsafe_code)]
//! Teaching-free ARC-AGI-3 application on the public physical workstation.

mod protocol;
mod sensorimotor;

pub use protocol::{
    Arc3ActionArguments, Arc3ActionCall, Arc3ActionCatalog, Arc3ActionOffer, Arc3ActionSchema,
    Arc3CapstoneAgent, Arc3CapstoneCommand, Arc3CapstoneObservation, Arc3CapstoneResponse,
};
pub use sensorimotor::{
    Arc3ActionWitness, Arc3DeviceInput, Arc3Error, Arc3PhysicalStep, Arc3Sensorimotor,
    Arc3SensorimotorObservation, Arc3SensorimotorSnapshot, ARC3_FRAME_PIXELS, ARC3_FRAME_SIDE,
    ARC3_PALETTE_SIZE, WORKSTATION_STEPS_PER_OBSERVATION,
};
