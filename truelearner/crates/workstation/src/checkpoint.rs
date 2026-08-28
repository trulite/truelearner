use crate::harness::Sites;
use crate::{WorkstationError, WorkstationState, AXIS_COUNT};
use bincode::Options;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_core::Checkpoint;

const MAGIC: &[u8; 8] = b"TLWORK01";
const VERSION: u16 = 1;
const LAYOUT_VERSION: u16 = 1;
const HEADER_LEN: usize = 50;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CheckpointPayload {
    pub core: Vec<u8>,
    pub state: WorkstationState,
    pub sites: Sites,
    pub sequence: u64,
    pub pending_transitions: [bool; AXIS_COUNT],
    pub layout_version: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkstationCheckpoint {
    payload: CheckpointPayload,
}

impl WorkstationCheckpoint {
    pub(crate) fn new(
        core: Vec<u8>,
        state: WorkstationState,
        sites: Sites,
        sequence: u64,
        pending_transitions: [bool; AXIS_COUNT],
    ) -> Self {
        Self {
            payload: CheckpointPayload {
                core,
                state,
                sites,
                sequence,
                pending_transitions,
                layout_version: LAYOUT_VERSION,
            },
        }
    }

    pub(crate) fn open(self) -> Result<CheckpointPayload, WorkstationError> {
        validate_payload(&self.payload)?;
        Ok(self.payload)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, WorkstationError> {
        validate_payload(&self.payload)?;
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
        let version = u16::from_le_bytes(bytes[8..10].try_into().unwrap());
        if version != VERSION {
            return Err(WorkstationError::UnsupportedCheckpointVersion(version));
        }
        let length = usize::try_from(u64::from_le_bytes(bytes[10..18].try_into().unwrap()))
            .map_err(|_| WorkstationError::InvalidCheckpoint)?;
        let expected = HEADER_LEN
            .checked_add(length)
            .ok_or(WorkstationError::InvalidCheckpoint)?;
        match bytes.len().cmp(&expected) {
            std::cmp::Ordering::Less => return Err(WorkstationError::TruncatedCheckpoint),
            std::cmp::Ordering::Greater => return Err(WorkstationError::TrailingCheckpointBytes),
            std::cmp::Ordering::Equal => {}
        }
        let checksum: [u8; 32] = bytes[18..HEADER_LEN].try_into().unwrap();
        let encoded = &bytes[HEADER_LEN..];
        if <[u8; 32]>::from(Sha256::digest(encoded)) != checksum {
            return Err(WorkstationError::CheckpointChecksum);
        }
        let payload = options()
            .deserialize(encoded)
            .map_err(|_| WorkstationError::InvalidCheckpoint)?;
        validate_payload(&payload)?;
        Ok(Self { payload })
    }
}

fn validate_payload(payload: &CheckpointPayload) -> Result<(), WorkstationError> {
    if payload.layout_version != LAYOUT_VERSION {
        return Err(WorkstationError::InvalidCheckpoint);
    }
    payload.state.validate()?;
    payload.sites.validate()?;
    Checkpoint::decode(&payload.core)
        .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
    Ok(())
}

fn options() -> impl Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .reject_trailing_bytes()
}
