use crate::{DeviceState, WorkstationPresentation, WorldError};
use bincode::Options;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const MAGIC: &[u8; 8] = b"TLWSES01";
const LEGACY_VERSION: u16 = 1;
const VERSION: u16 = 2;
const LEGACY_LAYOUT_VERSION: u16 = 1;
const LAYOUT_VERSION: u16 = 2;
const HEADER_LEN: usize = 50;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CheckpointPayload {
    pub harness: Vec<u8>,
    pub device: DeviceState,
    pub presentation: WorkstationPresentation,
    pub sequence: u64,
    pub asset_digest: [u8; 32],
    pub layout_version: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyCheckpointPayload {
    harness: Vec<u8>,
    device: DeviceState,
    sequence: u64,
    asset_digest: [u8; 32],
    layout_version: u16,
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
    ) -> Self {
        Self {
            payload: CheckpointPayload {
                harness,
                device,
                presentation,
                sequence,
                asset_digest,
                layout_version: LAYOUT_VERSION,
            },
        }
    }

    pub(crate) fn open(self) -> Result<CheckpointPayload, WorldError> {
        validate_payload(&self.payload)?;
        Ok(self.payload)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, WorldError> {
        validate_payload(&self.payload)?;
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
        if !matches!(version, LEGACY_VERSION | VERSION) {
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
        let payload = if version == LEGACY_VERSION {
            let legacy: LegacyCheckpointPayload = options()
                .deserialize(encoded)
                .map_err(|_| WorldError::InvalidCheckpoint)?;
            validate_legacy_payload(&legacy)?;
            CheckpointPayload {
                harness: legacy.harness,
                device: legacy.device,
                presentation: WorkstationPresentation::default(),
                sequence: legacy.sequence,
                asset_digest: legacy.asset_digest,
                layout_version: LAYOUT_VERSION,
            }
        } else {
            options()
                .deserialize(encoded)
                .map_err(|_| WorldError::InvalidCheckpoint)?
        };
        validate_payload(&payload)?;
        Ok(Self { payload })
    }
}

fn validate_legacy_payload(payload: &LegacyCheckpointPayload) -> Result<(), WorldError> {
    if payload.layout_version != LEGACY_LAYOUT_VERSION {
        return Err(WorldError::InvalidCheckpoint);
    }
    payload.device.validate()?;
    truelearner_workstation::WorkstationCheckpoint::decode(&payload.harness)?;
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_one_checkpoint_migrates_to_default_presentation() {
        let harness = truelearner_workstation::WorkstationHarness::new(71_006)
            .unwrap()
            .save()
            .unwrap()
            .canonical_bytes()
            .unwrap();
        let legacy = LegacyCheckpointPayload {
            harness,
            device: DeviceState::default(),
            sequence: 7,
            asset_digest: [9; 32],
            layout_version: LEGACY_LAYOUT_VERSION,
        };
        let payload = options().serialize(&legacy).unwrap();
        let mut encoded = Vec::new();
        encoded.extend_from_slice(MAGIC);
        encoded.extend_from_slice(&LEGACY_VERSION.to_le_bytes());
        encoded.extend_from_slice(&u64::try_from(payload.len()).unwrap().to_le_bytes());
        encoded.extend_from_slice(&Sha256::digest(&payload));
        encoded.extend_from_slice(&payload);

        let migrated = SessionCheckpoint::decode(&encoded).unwrap().open().unwrap();
        assert_eq!(migrated.presentation, WorkstationPresentation::default());
        assert_eq!(migrated.sequence, 7);
        assert_eq!(migrated.layout_version, LAYOUT_VERSION);
    }
}
