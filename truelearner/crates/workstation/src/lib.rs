#![forbid(unsafe_code)]
//! A bounded visual-touch workstation body around the public TrueLearner Harness.

mod checkpoint;
mod harness;
mod state;

pub use checkpoint::WorkstationCheckpoint;
#[cfg(feature = "research")]
pub use harness::{
    ResearchChoiceDiagnostic, ResearchHarnessConfig, ResearchOpportunityIncidence,
    ResearchTransitionOpportunity,
};
pub use harness::{StepMetrics, WorkstationHarness, WorkstationRead, WorkstationStepObservation};
pub use state::{
    AxisProprioception, BodyAxis, BodyControl, BodyMovement, ContactSample, Digit, DigitState,
    Direction, Eye, EyeState, HandPoint, HandState, LightField, Point, WorkstationState,
    WorldSample, AXIS_COUNT, BODY_MAX, DIGIT_COUNT, TOUCH_SITES,
};
#[cfg(feature = "research")]
pub use truelearner_core::Protocol;

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkstationError {
    EmptyLightField,
    LightFieldTooLarge,
    LightLength,
    ContactOutsideRange,
    InvalidState,
    TruncatedCheckpoint,
    WrongCheckpointMagic,
    UnsupportedCheckpointVersion(u16),
    InvalidCheckpoint,
    CheckpointChecksum,
    TrailingCheckpointBytes,
    UnknownOutput(u64),
    CoreCheckpoint(String),
}

impl fmt::Display for WorkstationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLightField => formatter.write_str("light field dimensions must be positive"),
            Self::LightFieldTooLarge => {
                formatter.write_str("light field exceeds the bounded pixel capacity")
            }
            Self::LightLength => {
                formatter.write_str("light field length does not match its dimensions")
            }
            Self::ContactOutsideRange => {
                formatter.write_str("contact sample is outside the physical range")
            }
            Self::InvalidState => formatter.write_str("workstation body state is invalid"),
            Self::TruncatedCheckpoint => formatter.write_str("workstation checkpoint is truncated"),
            Self::WrongCheckpointMagic => {
                formatter.write_str("workstation checkpoint has the wrong magic")
            }
            Self::UnsupportedCheckpointVersion(version) => {
                write!(
                    formatter,
                    "unsupported workstation checkpoint version {version}"
                )
            }
            Self::InvalidCheckpoint => formatter.write_str("workstation checkpoint is invalid"),
            Self::CheckpointChecksum => {
                formatter.write_str("workstation checkpoint checksum differs")
            }
            Self::TrailingCheckpointBytes => {
                formatter.write_str("workstation checkpoint has trailing bytes")
            }
            Self::UnknownOutput(physical) => {
                write!(formatter, "unknown outward physical output {physical}")
            }
            Self::CoreCheckpoint(message) => write!(formatter, "core checkpoint failed: {message}"),
        }
    }
}

impl std::error::Error for WorkstationError {}
