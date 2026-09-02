use crate::{
    harness::{Handles, VisualAttention},
    MotorEffect, WorkstationError, WorkstationState, WorldSample, AXIS_COUNT,
};
use bincode::Options;
use memmap2::MmapOptions;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
use truelearner_body::BodyCheckpoint;

const MAGIC: &[u8; 8] = b"TLWORK02";
const VERSION: u16 = 17;
const HEADER_LEN: usize = 50;
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Payload {
    pub(crate) body: Vec<u8>,
    pub(crate) handles: Handles,
    pub(crate) state: WorkstationState,
    pub(crate) sequence: u64,
    pub(crate) physical_tick: u64,
    pub(crate) pending_transitions: [Option<crate::Direction>; AXIS_COUNT],
    pub(crate) pending_stops: Vec<MotorEffect>,
    pub(crate) reach_strain: [i32; 2],
    pub(crate) vergence_strain: i32,
    pub(crate) visual_attention: VisualAttention,
    pub(crate) history: Vec<WorldSample>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkstationCheckpoint {
    payload: Payload,
}

impl WorkstationCheckpoint {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        body: Vec<u8>,
        handles: Handles,
        state: WorkstationState,
        sequence: u64,
        physical_tick: u64,
        pending_transitions: [Option<crate::Direction>; AXIS_COUNT],
        pending_stops: Vec<MotorEffect>,
        reach_strain: [i32; 2],
        vergence_strain: i32,
        visual_attention: VisualAttention,
        history: Vec<WorldSample>,
    ) -> Self {
        Self {
            payload: Payload {
                body,
                handles,
                state,
                sequence,
                physical_tick,
                pending_transitions,
                pending_stops,
                reach_strain,
                vergence_strain,
                visual_attention,
                history,
            },
        }
    }

    pub(crate) fn open(self) -> Payload {
        self.payload
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
        let payload: Payload = options()
            .deserialize(payload)
            .map_err(|_| WorkstationError::InvalidCheckpoint)?;
        BodyCheckpoint::decode(&payload.body).map_err(|_| WorkstationError::InvalidCheckpoint)?;
        Ok(Self { payload })
    }

    pub fn write_mmap(&self, path: impl AsRef<Path>) -> Result<(), WorkstationError> {
        let bytes = self.canonical_bytes()?;
        let destination = path.as_ref();
        let (temporary, file) = create_temporary(destination)?;
        let result = write_and_replace(&bytes, &file, &temporary, destination);
        if result.is_err() {
            let _ = std::fs::remove_file(&temporary);
        }
        result
    }
}

#[allow(unsafe_code)]
fn write_and_replace(
    bytes: &[u8],
    file: &File,
    temporary: &Path,
    destination: &Path,
) -> Result<(), WorkstationError> {
    file.set_len(u64::try_from(bytes.len()).map_err(|_| WorkstationError::CheckpointIo)?)
        .map_err(|_| WorkstationError::CheckpointIo)?;
    // SAFETY: this process created the file exclusively, fixes its length before
    // mapping, and neither aliases nor truncates it until the mapping is dropped.
    let mut mapped = unsafe { MmapOptions::new().len(bytes.len()).map_mut(file) }
        .map_err(|_| WorkstationError::CheckpointIo)?;
    mapped.copy_from_slice(bytes);
    mapped.flush().map_err(|_| WorkstationError::CheckpointIo)?;
    drop(mapped);
    file.sync_all()
        .map_err(|_| WorkstationError::CheckpointIo)?;
    std::fs::rename(temporary, destination).map_err(|_| WorkstationError::CheckpointIo)?;
    sync_parent(destination)?;
    Ok(())
}

fn create_temporary(destination: &Path) -> Result<(PathBuf, File), WorkstationError> {
    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    let name = destination
        .file_name()
        .ok_or(WorkstationError::CheckpointIo)?;
    for _ in 0..16 {
        let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let mut temporary = name.to_os_string();
        temporary.push(format!(".{}.{}.tmp", std::process::id(), serial));
        let path = parent.join(temporary);
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(file) => return Ok((path, file)),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => return Err(WorkstationError::CheckpointIo),
        }
    }
    Err(WorkstationError::CheckpointIo)
}

#[cfg(unix)]
fn sync_parent(destination: &Path) -> Result<(), WorkstationError> {
    File::open(destination.parent().unwrap_or_else(|| Path::new(".")))
        .and_then(|directory| directory.sync_all())
        .map_err(|_| WorkstationError::CheckpointIo)
}

#[cfg(not(unix))]
fn sync_parent(_destination: &Path) -> Result<(), WorkstationError> {
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
    use crate::WorkstationHarness;

    #[test]
    fn round_trip_corruption_and_mmap_write_are_explicit() {
        let checkpoint = WorkstationHarness::new(1).unwrap().save().unwrap();
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

        let mut obsolete = bytes.clone();
        obsolete[8..10].copy_from_slice(&16_u16.to_le_bytes());
        assert_eq!(
            WorkstationCheckpoint::decode(&obsolete),
            Err(WorkstationError::UnsupportedCheckpointVersion(16))
        );

        let path = std::env::temp_dir().join(format!(
            "truelearner-checkpoint-{}-{}.bin",
            std::process::id(),
            NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
        ));
        checkpoint.write_mmap(&path).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        std::fs::remove_file(path).unwrap();
    }
}
