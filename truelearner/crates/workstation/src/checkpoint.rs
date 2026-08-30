use crate::{WorkstationError, WorldSample};
use bincode::Options;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const MAGIC: &[u8; 8] = b"TLWORK02";
const VERSION: u16 = 1;
const HEADER_LEN: usize = 50;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Payload {
    history: Vec<WorldSample>,
}

/// Opaque durable history from which the deterministic body is rebuilt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkstationCheckpoint {
    payload: Payload,
}

impl WorkstationCheckpoint {
    pub(crate) fn new(history: Vec<WorldSample>) -> Self {
        Self {
            payload: Payload { history },
        }
    }

    pub(crate) fn into_history(self) -> Vec<WorldSample> {
        self.payload.history
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, WorkstationError> {
        let payload = options()
            .serialize(&self.payload)
            .map_err(|_| WorkstationError::InvalidCheckpoint)?;
        let length =
            u64::try_from(payload.len()).map_err(|_| WorkstationError::InvalidCheckpoint)?;
        let mut bytes = Vec::with_capacity(HEADER_LEN.saturating_add(payload.len()));
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes.extend_from_slice(&Sha256::digest(&payload));
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, WorkstationError> {
        if bytes.len() < HEADER_LEN {
            return Err(WorkstationError::TruncatedCheckpoint);
        }
        if &bytes[..8] != MAGIC {
            return Err(WorkstationError::WrongCheckpointMagic);
        }
        let version = u16::from_le_bytes(
            bytes[8..10]
                .try_into()
                .map_err(|_| WorkstationError::InvalidCheckpoint)?,
        );
        if version != VERSION {
            return Err(WorkstationError::UnsupportedCheckpointVersion(version));
        }
        let length = usize::try_from(u64::from_le_bytes(
            bytes[10..18]
                .try_into()
                .map_err(|_| WorkstationError::InvalidCheckpoint)?,
        ))
        .map_err(|_| WorkstationError::InvalidCheckpoint)?;
        let expected = HEADER_LEN
            .checked_add(length)
            .ok_or(WorkstationError::InvalidCheckpoint)?;
        match bytes.len().cmp(&expected) {
            std::cmp::Ordering::Less => return Err(WorkstationError::TruncatedCheckpoint),
            std::cmp::Ordering::Greater => return Err(WorkstationError::TrailingCheckpointBytes),
            std::cmp::Ordering::Equal => {}
        }
        let payload = &bytes[HEADER_LEN..];
        if Sha256::digest(payload).as_slice() != &bytes[18..HEADER_LEN] {
            return Err(WorkstationError::CheckpointChecksum);
        }
        let payload = options()
            .deserialize(payload)
            .map_err(|_| WorkstationError::InvalidCheckpoint)?;
        Ok(Self { payload })
    }
}

fn options() -> impl Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .reject_trailing_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContactSample, LightField, TOUCH_SITES};

    fn sample() -> WorldSample {
        WorldSample::new(
            [
                LightField::filled(1, 1, 1).unwrap(),
                LightField::filled(1, 1, 2).unwrap(),
            ],
            [ContactSample::default(); TOUCH_SITES],
        )
        .unwrap()
    }

    #[test]
    fn round_trip_and_corruption_are_explicit() {
        let checkpoint = WorkstationCheckpoint::new(vec![sample()]);
        let bytes = checkpoint.canonical_bytes().unwrap();
        assert_eq!(WorkstationCheckpoint::decode(&bytes).unwrap(), checkpoint);

        let mut corrupt = bytes.clone();
        corrupt[HEADER_LEN] ^= 1;
        assert_eq!(
            WorkstationCheckpoint::decode(&corrupt),
            Err(WorkstationError::CheckpointChecksum)
        );
        assert_eq!(
            WorkstationCheckpoint::decode(&bytes[..HEADER_LEN - 1]),
            Err(WorkstationError::TruncatedCheckpoint)
        );
        let mut trailing = bytes;
        trailing.push(0);
        assert_eq!(
            WorkstationCheckpoint::decode(&trailing),
            Err(WorkstationError::TrailingCheckpointBytes)
        );
    }
}
