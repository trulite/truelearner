#![forbid(unsafe_code)]
//! A deterministic external keyboard, touchpad, and monitor world.

mod checkpoint;
mod geometry;
mod recording;
mod render;
mod session;
mod world;

pub use checkpoint::SessionCheckpoint;
pub use geometry::{Key, KeyId, Rect, WorldGeometry, KEY_COUNT};
pub use recording::{RecordedStep, WorkstationRecording, MAX_RECORDING_STEPS};
pub use session::{SessionObservation, SessionRead, WorkstationSession};
pub use world::{
    DeviceEvent, DeviceState, MonitorFrame, ScreenPoint, WorkstationPresentation, WorkstationWorld,
    WorldTransition, CONTACT_DEPTH, KEY_PRESS_DEPTH, KEY_RELEASE_DEPTH, LONG_PRESS_STEPS,
};

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorldError {
    AssetDecode,
    AssetDigest,
    InvalidGeometry,
    InvalidPresentation,
    InvalidKeyDepths,
    InvalidState,
    InvalidCheckpoint,
    TruncatedCheckpoint,
    WrongCheckpointMagic,
    UnsupportedCheckpointVersion(u16),
    CheckpointChecksum,
    TrailingCheckpointBytes,
    InvalidRecording,
    RecordingTooLong,
    TruncatedRecording,
    WrongRecordingMagic,
    UnsupportedRecordingVersion(u16),
    RecordingChecksum,
    TrailingRecordingBytes,
    RecordingReplayDiverged(u64),
    Body(String),
}

impl fmt::Display for WorldError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AssetDecode => formatter.write_str("monitor image could not be decoded"),
            Self::AssetDigest => formatter.write_str("monitor image digest differs"),
            Self::InvalidGeometry => formatter.write_str("workstation geometry is invalid"),
            Self::InvalidPresentation => formatter.write_str("workstation presentation is invalid"),
            Self::InvalidKeyDepths => formatter.write_str("workstation key depths are invalid"),
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
            Self::InvalidRecording => formatter.write_str("workstation recording is invalid"),
            Self::RecordingTooLong => formatter.write_str("workstation recording is too long"),
            Self::TruncatedRecording => formatter.write_str("workstation recording is truncated"),
            Self::WrongRecordingMagic => {
                formatter.write_str("workstation recording has the wrong magic")
            }
            Self::UnsupportedRecordingVersion(version) => {
                write!(
                    formatter,
                    "unsupported workstation recording version {version}"
                )
            }
            Self::RecordingChecksum => {
                formatter.write_str("workstation recording checksum differs")
            }
            Self::TrailingRecordingBytes => {
                formatter.write_str("workstation recording has trailing bytes")
            }
            Self::RecordingReplayDiverged(sequence) => {
                write!(
                    formatter,
                    "workstation recording diverged at sequence {sequence}"
                )
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
