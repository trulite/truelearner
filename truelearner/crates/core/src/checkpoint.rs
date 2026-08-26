use crate::prelude::*;
use crate::snapshot::BodySnapshot;
use bincode::Options;

const VERSION: u16 = 2;
const MAGIC: &[u8; 8] = b"TLCHKP01";
const HEADER_LEN: usize = 50;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkpoint {
    body: BodySnapshot,
    outward_region: i16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckpointError {
    Format(FormatError),
    UnsupportedTransmissionMode(u8),
    MissingJunction(JunctionId),
    MissingLink(LinkId),
    ManifestMismatch,
    InvalidPhysicalBody,
    StaleJunctionReference(JunctionRef),
    Truncated,
    WrongMagic,
    UnsupportedCheckpointVersion(u16),
    InvalidCheckpoint,
    Checksum,
    TrailingBytes,
}

impl From<FormatError> for CheckpointError {
    fn from(error: FormatError) -> Self {
        Self::Format(error)
    }
}

impl Checkpoint {
    pub(crate) fn new(body: BodySnapshot, outward_region: i16) -> Self {
        Self {
            body,
            outward_region,
        }
    }

    pub(crate) fn open(self) -> (BodySnapshot, i16) {
        (self.body, self.outward_region)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CheckpointError> {
        validate_manifest(&self.body.body_version, &self.body.body)?;
        let payload = options()
            .serialize(self)
            .map_err(|_| CheckpointError::InvalidCheckpoint)?;
        let length =
            u64::try_from(payload.len()).map_err(|_| CheckpointError::InvalidCheckpoint)?;
        let mut bytes = Vec::with_capacity(HEADER_LEN + payload.len());
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes.extend_from_slice(ContentHash::of(&payload).as_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, CheckpointError> {
        if bytes.len() < HEADER_LEN {
            return Err(CheckpointError::Truncated);
        }
        if &bytes[..MAGIC.len()] != MAGIC {
            return Err(CheckpointError::WrongMagic);
        }
        let version = u16::from_le_bytes(bytes[8..10].try_into().unwrap());
        if version != VERSION {
            return Err(CheckpointError::UnsupportedCheckpointVersion(version));
        }
        let length = usize::try_from(u64::from_le_bytes(bytes[10..18].try_into().unwrap()))
            .map_err(|_| CheckpointError::InvalidCheckpoint)?;
        let expected = HEADER_LEN
            .checked_add(length)
            .ok_or(CheckpointError::InvalidCheckpoint)?;
        match bytes.len().cmp(&expected) {
            std::cmp::Ordering::Less => return Err(CheckpointError::Truncated),
            std::cmp::Ordering::Greater => return Err(CheckpointError::TrailingBytes),
            std::cmp::Ordering::Equal => {}
        }
        let checksum = ContentHash(bytes[18..HEADER_LEN].try_into().unwrap());
        let payload = &bytes[HEADER_LEN..];
        if ContentHash::of(payload) != checksum {
            return Err(CheckpointError::Checksum);
        }
        let checkpoint: Self = options()
            .deserialize(payload)
            .map_err(|_| CheckpointError::InvalidCheckpoint)?;
        validate_manifest(&checkpoint.body.body_version, &checkpoint.body.body)?;
        Ok(checkpoint)
    }
}

fn options() -> impl Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .reject_trailing_bytes()
}

pub(crate) fn validate_manifest(
    manifest: &BodyVersion,
    body: &ArenaBody,
) -> Result<(), CheckpointError> {
    let hash = body.content_hash()?;
    let matches = manifest.arenas.len() == 1
        && manifest.arenas[0].arena == body.arena
        && manifest.arenas[0].block == hash;
    matches
        .then_some(())
        .ok_or(CheckpointError::ManifestMismatch)
}
