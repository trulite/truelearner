#![forbid(unsafe_code)]
//! A deterministic external keyboard, touchpad, and monitor world.

mod checkpoint;
mod geometry;
mod render;
mod session;
mod world;

pub use checkpoint::SessionCheckpoint;
pub use geometry::{Key, KeyId, Rect, WorldGeometry, KEY_COUNT};
pub use session::{SessionObservation, SessionRead, WorkstationSession};
pub use world::{DeviceEvent, DeviceState, ScreenPoint, WorkstationWorld};

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorldError {
    AssetDecode,
    AssetDigest,
    InvalidGeometry,
    InvalidState,
    InvalidCheckpoint,
    TruncatedCheckpoint,
    WrongCheckpointMagic,
    UnsupportedCheckpointVersion(u16),
    CheckpointChecksum,
    TrailingCheckpointBytes,
    Body(String),
}

impl fmt::Display for WorldError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AssetDecode => formatter.write_str("monitor image could not be decoded"),
            Self::AssetDigest => formatter.write_str("monitor image digest differs"),
            Self::InvalidGeometry => formatter.write_str("workstation geometry is invalid"),
            Self::InvalidState => formatter.write_str("workstation device state is invalid"),
            Self::InvalidCheckpoint => formatter.write_str("workstation checkpoint is invalid"),
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
            Self::CheckpointChecksum => {
                formatter.write_str("workstation checkpoint checksum differs")
            }
            Self::TrailingCheckpointBytes => {
                formatter.write_str("workstation checkpoint has trailing bytes")
            }
            Self::Body(message) => write!(formatter, "workstation body failed: {message}"),
        }
    }
}

impl std::error::Error for WorldError {}

impl From<truelearner_workstation::WorkstationError> for WorldError {
    fn from(value: truelearner_workstation::WorkstationError) -> Self {
        Self::Body(value.to_string())
    }
}
