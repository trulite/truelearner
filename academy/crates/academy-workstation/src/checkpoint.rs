use crate::{DeviceState, WorkstationPresentation, WorldError};
use bincode::Options;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_workstation::MotorEffect;

const MAGIC: &[u8; 8] = b"TLWSES02";
const VERSION: u16 = 4;
const LAYOUT_VERSION: u16 = 4;
const HEADER_LEN: usize = 50;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CheckpointPayload {
    pub harness: Vec<u8>,
    pub device: DeviceState,
    pub presentation: WorkstationPresentation,
    pub sequence: u64,
    pub asset_digest: [u8; 32],
    pub boundary_parents: Vec<MotorEffect>,
    pub progress_parents: Vec<MotorEffect>,
    pub layout_version: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionCheckpoint {
    payload: CheckpointPayload,
}

impl SessionCheckpoint {
    pub(crate) fn new(
        harness: Vec<u8>,
        device: DeviceState,
        presentation: WorkstationPresentation,
        sequence: u64,
        asset_digest: [u8; 32],
        boundary_parents: Vec<MotorEffect>,
        progress_parents: Vec<MotorEffect>,
    ) -> Self {
        Self {
            payload: CheckpointPayload {
                harness,
                device,
                presentation,
                sequence,
                asset_digest,
                boundary_parents,
                progress_parents,
                layout_version: LAYOUT_VERSION,
            },
        }
    }

    pub(crate) fn open(self) -> Result<CheckpointPayload, WorldError> {
        Ok(self.payload)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, WorldError> {
        let payload = options()
            .serialize(&self.payload)
            .map_err(|_| WorldError::InvalidCheckpoint)?;
        let length = u64::try_from(payload.len()).map_err(|_| WorldError::InvalidCheckpoint)?;
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
            return Err(WorldError::TruncatedCheckpoint);
        }
        if &bytes[..8] != MAGIC {
            return Err(WorldError::WrongCheckpointMagic);
        }
        let version = u16::from_le_bytes(bytes[8..10].try_into().unwrap());
        if version != VERSION {
            return Err(WorldError::UnsupportedCheckpointVersion(version));
        }
        let length = usize::try_from(u64::from_le_bytes(bytes[10..18].try_into().unwrap()))
            .map_err(|_| WorldError::InvalidCheckpoint)?;
        let expected = HEADER_LEN
            .checked_add(length)
            .ok_or(WorldError::InvalidCheckpoint)?;
        match bytes.len().cmp(&expected) {
            std::cmp::Ordering::Less => return Err(WorldError::TruncatedCheckpoint),
            std::cmp::Ordering::Greater => return Err(WorldError::TrailingCheckpointBytes),
            std::cmp::Ordering::Equal => {}
        }
        let checksum: [u8; 32] = bytes[18..HEADER_LEN].try_into().unwrap();
        let encoded = &bytes[HEADER_LEN..];
        if <[u8; 32]>::from(Sha256::digest(encoded)) != checksum {
            return Err(WorldError::CheckpointChecksum);
        }
        let payload = options()
            .deserialize(encoded)
            .map_err(|_| WorldError::InvalidCheckpoint)?;
        validate_payload(&payload)?;
        Ok(Self { payload })
    }
}

fn validate_payload(payload: &CheckpointPayload) -> Result<(), WorldError> {
    if payload.layout_version != LAYOUT_VERSION {
        return Err(WorldError::InvalidCheckpoint);
    }
    payload.device.validate()?;
    payload
        .presentation
        .validate(&crate::WorldGeometry::standard_ansi_104()?)?;
    truelearner_workstation::WorkstationCheckpoint::decode(&payload.harness)?;
    Ok(())
}

fn options() -> impl Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .reject_trailing_bytes()
}
