use crate::{SessionCheckpoint, SessionObservation, SessionRead, WorkstationSession, WorldError};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};

const MAGIC: &[u8; 8] = b"TLWREC01";
const VERSION: u16 = 4;
const HEADER_LEN: usize = 50;
pub const MAX_RECORDING_STEPS: usize = 120;
const MAX_DECODED_BYTES: u64 = 256 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedStep {
    pub before: SessionRead,
    pub observation: SessionObservation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationRecording {
    schema_version: u16,
    seed: u64,
    initial_checkpoint: Vec<u8>,
    steps: Vec<RecordedStep>,
}

impl WorkstationRecording {
    pub fn capture(seed: u64, step_count: usize) -> Result<Self, WorldError> {
        if step_count == 0 {
            return Err(WorldError::InvalidRecording);
        }
        if step_count > MAX_RECORDING_STEPS {
            return Err(WorldError::RecordingTooLong);
        }

        let mut session = WorkstationSession::new(seed)?;
        let initial_checkpoint = session.save()?.canonical_bytes()?;
        let mut steps = Vec::with_capacity(step_count);
        for _ in 0..step_count {
            steps.push(RecordedStep {
                before: session.read()?,
                observation: session.step()?,
            });
        }
        let recording = Self {
            schema_version: VERSION,
            seed,
            initial_checkpoint,
            steps,
        };
        recording.validate()?;
        recording.verify_exact_replay()?;
        Ok(recording)
    }

    pub const fn seed(&self) -> u64 {
        self.seed
    }

    pub fn steps(&self) -> &[RecordedStep] {
        &self.steps
    }

    pub fn initial_checkpoint(&self) -> &[u8] {
        &self.initial_checkpoint
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, WorldError> {
        self.validate()?;
        let json = serde_json::to_vec(self).map_err(|_| WorldError::InvalidRecording)?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&json)
            .map_err(|_| WorldError::InvalidRecording)?;
        let payload = encoder.finish().map_err(|_| WorldError::InvalidRecording)?;
        let length = u64::try_from(payload.len()).map_err(|_| WorldError::InvalidRecording)?;
        let mut bytes = Vec::with_capacity(HEADER_LEN.saturating_add(payload.len()));
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes.extend_from_slice(&Sha256::digest(&payload));
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, WorldError> {
        if bytes.len() < HEADER_LEN {
            return Err(WorldError::TruncatedRecording);
        }
        if &bytes[..8] != MAGIC {
            return Err(WorldError::WrongRecordingMagic);
        }
        let version = u16::from_le_bytes(bytes[8..10].try_into().unwrap());
        if version != VERSION {
            return Err(WorldError::UnsupportedRecordingVersion(version));
        }
        let length = usize::try_from(u64::from_le_bytes(bytes[10..18].try_into().unwrap()))
            .map_err(|_| WorldError::InvalidRecording)?;
        let expected = HEADER_LEN
            .checked_add(length)
            .ok_or(WorldError::InvalidRecording)?;
        match bytes.len().cmp(&expected) {
            std::cmp::Ordering::Less => return Err(WorldError::TruncatedRecording),
            std::cmp::Ordering::Greater => return Err(WorldError::TrailingRecordingBytes),
            std::cmp::Ordering::Equal => {}
        }
        let checksum: [u8; 32] = bytes[18..HEADER_LEN].try_into().unwrap();
        let payload = &bytes[HEADER_LEN..];
        if <[u8; 32]>::from(Sha256::digest(payload)) != checksum {
            return Err(WorldError::RecordingChecksum);
        }
        let mut decoded = Vec::new();
        GzDecoder::new(payload)
            .take(MAX_DECODED_BYTES.saturating_add(1))
            .read_to_end(&mut decoded)
            .map_err(|_| WorldError::InvalidRecording)?;
        if u64::try_from(decoded.len()).unwrap_or(u64::MAX) > MAX_DECODED_BYTES {
            return Err(WorldError::RecordingTooLong);
        }
        let recording: Self =
            serde_json::from_slice(&decoded).map_err(|_| WorldError::InvalidRecording)?;
        recording.validate()?;
        Ok(recording)
    }

    pub fn verify_exact_replay(&self) -> Result<(), WorldError> {
        self.validate()?;
        let checkpoint = SessionCheckpoint::decode(&self.initial_checkpoint)?;
        let mut session = WorkstationSession::restore(checkpoint)?;
        for recorded in &self.steps {
            let sequence = recorded.observation.sequence;
            if session.read()? != recorded.before || session.step()? != recorded.observation {
                return Err(WorldError::RecordingReplayDiverged(sequence));
            }
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), WorldError> {
        if self.schema_version != VERSION || self.steps.is_empty() {
            return Err(WorldError::InvalidRecording);
        }
        if self.steps.len() > MAX_RECORDING_STEPS {
            return Err(WorldError::RecordingTooLong);
        }
        SessionCheckpoint::decode(&self.initial_checkpoint)?;

        for (index, recorded) in self.steps.iter().enumerate() {
            let sequence = recorded.observation.sequence;
            if recorded.before.sequence != sequence
                || recorded.observation.body.sequence != sequence
                || recorded.before.body.state != recorded.observation.body.state_before
                || sequence
                    != self.steps[0]
                        .observation
                        .sequence
                        .saturating_add(index as u64)
            {
                return Err(WorldError::InvalidRecording);
            }
            if let Some(next) = self.steps.get(index + 1) {
                if next.before.session_fingerprint != recorded.observation.session_fingerprint
                    || next.before.device != recorded.observation.device_after
                    || next.before.body.state != recorded.observation.body.state_after
                {
                    return Err(WorldError::InvalidRecording);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recording_round_trip_preserves_complete_steps_and_exact_replay() {
        let recording = WorkstationRecording::capture(81_001, 3).unwrap();
        let bytes = recording.canonical_bytes().unwrap();
        let decoded = WorkstationRecording::decode(&bytes).unwrap();

        assert_eq!(decoded, recording);
        decoded.verify_exact_replay().unwrap();
        assert_eq!(decoded.steps().len(), 3);
    }

    #[test]
    fn recording_is_an_inert_view_of_the_ordinary_session() {
        let recording = WorkstationRecording::capture(81_002, 2).unwrap();
        let checkpoint = SessionCheckpoint::decode(recording.initial_checkpoint()).unwrap();
        let mut unrecorded = WorkstationSession::restore(checkpoint).unwrap();

        for expected in recording.steps() {
            assert_eq!(unrecorded.read().unwrap(), expected.before);
            assert_eq!(unrecorded.step().unwrap(), expected.observation);
        }
    }

    #[test]
    fn corrupt_truncated_and_trailing_recordings_fail_closed() {
        let recording = WorkstationRecording::capture(81_003, 1).unwrap();
        let bytes = recording.canonical_bytes().unwrap();

        let mut corrupt = bytes.clone();
        let last = corrupt.len() - 1;
        corrupt[last] ^= 1;
        assert_eq!(
            WorkstationRecording::decode(&corrupt).unwrap_err(),
            WorldError::RecordingChecksum
        );
        assert_eq!(
            WorkstationRecording::decode(&bytes[..bytes.len() - 1]).unwrap_err(),
            WorldError::TruncatedRecording
        );
        let mut trailing = bytes;
        trailing.push(0);
        assert_eq!(
            WorkstationRecording::decode(&trailing).unwrap_err(),
            WorldError::TrailingRecordingBytes
        );
    }

    #[test]
    fn recording_bounds_are_explicit() {
        assert_eq!(
            WorkstationRecording::capture(81_004, 0).unwrap_err(),
            WorldError::InvalidRecording
        );
        assert_eq!(
            WorkstationRecording::capture(81_004, MAX_RECORDING_STEPS + 1).unwrap_err(),
            WorldError::RecordingTooLong
        );
    }
}
