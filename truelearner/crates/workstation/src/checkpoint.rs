use crate::{
    harness::{Handles, VisualAttention},
    MotorEffect, WorkstationError, WorkstationState, WorldSample, AXIS_COUNT,
};
use crate::{ContactSample, LightField, TOUCH_SITES};
use bincode::Options;
use memmap2::MmapOptions;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
use truelearner_body::{BodyCheckpoint, JunctionId};

const MAGIC: &[u8; 8] = b"TLWORK02";
const VERSION: u16 = 16;
const LEGACY_ATTENTION_VERSION: u16 = 15;
const LEGACY_VISION_VERSION: u16 = 14;
const HEADER_LEN: usize = 50;
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Payload {
    pub(crate) body: Vec<u8>,
    pub(crate) handles: Handles,
    pub(crate) state: WorkstationState,
    pub(crate) sequence: u64,
    pub(crate) physical_tick: u64,
    pub(crate) pending_transitions: [bool; AXIS_COUNT],
    pub(crate) pending_stops: Vec<MotorEffect>,
    pub(crate) reach_strain: [i32; 2],
    pub(crate) vergence_strain: i32,
    pub(crate) visual_attention: VisualAttention,
    pub(crate) history: Vec<WorldSample>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkstationCheckpoint {
    payload: Payload,
    legacy_visual_migration: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyHandles {
    vision: [Vec<JunctionId>; 2],
    salience: [Vec<JunctionId>; 2],
    value: [Vec<u32>; 2],
    contacts: [[JunctionId; 2]; TOUCH_SITES],
    proprioception: [[JunctionId; 6]; AXIS_COUNT],
    exploration: [JunctionId; 4],
    competition_outcomes: [JunctionId; 4],
    outcomes: [JunctionId; AXIS_COUNT],
    resisted_progress: [JunctionId; AXIS_COUNT],
    opportunities: Vec<JunctionId>,
    outward: Vec<(JunctionId, crate::BodyControl)>,
}

impl From<LegacyHandles> for Handles {
    fn from(value: LegacyHandles) -> Self {
        Self {
            vision: value.vision,
            global_vision: [Vec::new(), Vec::new()],
            visual_transients: [Vec::new(), Vec::new()],
            foveal_vision: [Vec::new(), Vec::new()],
            salience: value.salience,
            value: value.value,
            contacts: value.contacts,
            proprioception: value.proprioception,
            exploration: value.exploration,
            competition_outcomes: value.competition_outcomes,
            outcomes: value.outcomes,
            resisted_progress: value.resisted_progress,
            opportunities: value.opportunities,
            outward: value.outward,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyWorldSample {
    eyes: [LightField; 2],
    contacts: [ContactSample; TOUCH_SITES],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyPayloadV14 {
    body: Vec<u8>,
    handles: LegacyHandles,
    state: WorkstationState,
    sequence: u64,
    physical_tick: u64,
    pending_transitions: [bool; AXIS_COUNT],
    pending_stops: Vec<MotorEffect>,
    reach_strain: [i32; 2],
    vergence_strain: i32,
    history: Vec<LegacyWorldSample>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyPayloadV15 {
    body: Vec<u8>,
    handles: Handles,
    state: WorkstationState,
    sequence: u64,
    physical_tick: u64,
    pending_transitions: [bool; AXIS_COUNT],
    pending_stops: Vec<MotorEffect>,
    reach_strain: [i32; 2],
    vergence_strain: i32,
    history: Vec<WorldSample>,
}

impl WorkstationCheckpoint {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        body: Vec<u8>,
        handles: Handles,
        state: WorkstationState,
        sequence: u64,
        physical_tick: u64,
        pending_transitions: [bool; AXIS_COUNT],
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
            legacy_visual_migration: false,
        }
    }

    pub(crate) fn open(self) -> (Payload, bool) {
        (self.payload, self.legacy_visual_migration)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, WorkstationError> {
        if self.legacy_visual_migration {
            return Err(WorkstationError::InvalidCheckpoint);
        }
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
        if ![VERSION, LEGACY_ATTENTION_VERSION, LEGACY_VISION_VERSION].contains(&version) {
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
        let (payload, legacy_visual_migration) = match version {
            VERSION => (
                options()
                    .deserialize(payload)
                    .map_err(|_| WorkstationError::InvalidCheckpoint)?,
                false,
            ),
            LEGACY_ATTENTION_VERSION => {
                let legacy: LegacyPayloadV15 = options()
                    .deserialize(payload)
                    .map_err(|_| WorkstationError::InvalidCheckpoint)?;
                let visual_attention = VisualAttention::from_history(&legacy.history);
                (
                    Payload {
                        body: legacy.body,
                        handles: legacy.handles,
                        state: legacy.state,
                        sequence: legacy.sequence,
                        physical_tick: legacy.physical_tick,
                        pending_transitions: legacy.pending_transitions,
                        pending_stops: legacy.pending_stops,
                        reach_strain: legacy.reach_strain,
                        vergence_strain: legacy.vergence_strain,
                        visual_attention,
                        history: legacy.history,
                    },
                    false,
                )
            }
            LEGACY_VISION_VERSION => {
                let legacy: LegacyPayloadV14 = options()
                    .deserialize(payload)
                    .map_err(|_| WorkstationError::InvalidCheckpoint)?;
                let history = legacy
                    .history
                    .into_iter()
                    .map(|sample| WorldSample::new(sample.eyes, sample.contacts))
                    .collect::<Result<Vec<_>, _>>()?;
                let visual_attention = VisualAttention::from_history(&history);
                (
                    Payload {
                        body: legacy.body,
                        handles: legacy.handles.into(),
                        state: legacy.state,
                        sequence: legacy.sequence,
                        physical_tick: legacy.physical_tick,
                        pending_transitions: legacy.pending_transitions,
                        pending_stops: legacy.pending_stops,
                        reach_strain: legacy.reach_strain,
                        vergence_strain: legacy.vergence_strain,
                        visual_attention,
                        history,
                    },
                    true,
                )
            }
            _ => unreachable!("checkpoint version was validated"),
        };
        BodyCheckpoint::decode(&payload.body).map_err(|_| WorkstationError::InvalidCheckpoint)?;
        Ok(Self {
            payload,
            legacy_visual_migration,
        })
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

    fn version_14_bytes(harness: &WorkstationHarness) -> Vec<u8> {
        let body = harness
            .body
            .checkpoint()
            .unwrap()
            .canonical_bytes()
            .unwrap();
        let handles = LegacyHandles {
            vision: harness.handles.vision.clone(),
            salience: harness.handles.salience.clone(),
            value: harness.handles.value.clone(),
            contacts: harness.handles.contacts,
            proprioception: harness.handles.proprioception,
            exploration: harness.handles.exploration,
            competition_outcomes: harness.handles.competition_outcomes,
            outcomes: harness.handles.outcomes,
            resisted_progress: harness.handles.resisted_progress,
            opportunities: harness.handles.opportunities.clone(),
            outward: harness.handles.outward.clone(),
        };
        let payload = LegacyPayloadV14 {
            body,
            handles,
            state: harness.state.clone(),
            sequence: harness.sequence,
            physical_tick: harness.physical_tick,
            pending_transitions: harness.pending_transitions,
            pending_stops: harness.pending_stops.clone(),
            reach_strain: harness.reach_strain,
            vergence_strain: harness.vergence_strain,
            history: Vec::new(),
        };
        let payload = options().serialize(&payload).unwrap();
        let mut bytes = Vec::with_capacity(HEADER_LEN + payload.len());
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&LEGACY_VISION_VERSION.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&Sha256::digest(&payload));
        bytes.extend_from_slice(&payload);
        bytes
    }

    fn version_15_bytes(harness: &WorkstationHarness) -> Vec<u8> {
        let payload = LegacyPayloadV15 {
            body: harness
                .body
                .checkpoint()
                .unwrap()
                .canonical_bytes()
                .unwrap(),
            handles: harness.handles.clone(),
            state: harness.state.clone(),
            sequence: harness.sequence,
            physical_tick: harness.physical_tick,
            pending_transitions: harness.pending_transitions,
            pending_stops: harness.pending_stops.clone(),
            reach_strain: harness.reach_strain,
            vergence_strain: harness.vergence_strain,
            history: harness.history.clone(),
        };
        let payload = options().serialize(&payload).unwrap();
        let mut bytes = Vec::with_capacity(HEADER_LEN + payload.len());
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&LEGACY_ATTENTION_VERSION.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&Sha256::digest(&payload));
        bytes.extend_from_slice(&payload);
        bytes
    }

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
        obsolete[8..10].copy_from_slice(&9_u16.to_le_bytes());
        assert_eq!(
            WorkstationCheckpoint::decode(&obsolete),
            Err(WorkstationError::UnsupportedCheckpointVersion(9))
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

    #[test]
    fn version_14_migration_matches_fresh_version_16_construction_and_replay() {
        let legacy = WorkstationHarness::legacy_for_test().unwrap();
        let decoded = WorkstationCheckpoint::decode(&version_14_bytes(&legacy)).unwrap();
        assert_eq!(
            decoded.canonical_bytes(),
            Err(WorkstationError::InvalidCheckpoint)
        );

        let mut migrated = WorkstationHarness::restore(decoded).unwrap();
        let mut fresh = WorkstationHarness::new(1).unwrap();
        assert_eq!(migrated.read().unwrap(), fresh.read().unwrap());

        let sample = crate::WorldSample::new(
            [
                LightField::filled(9, 9, 17).unwrap(),
                LightField::filled(9, 9, 23).unwrap(),
            ],
            [ContactSample::default(); TOUCH_SITES],
        )
        .unwrap();
        assert_eq!(
            migrated.step(sample.clone()).unwrap(),
            fresh.step(sample).unwrap()
        );
        let migrated_bytes = migrated.save().unwrap().canonical_bytes().unwrap();
        assert_eq!(
            WorkstationCheckpoint::decode(&migrated_bytes)
                .unwrap()
                .canonical_bytes()
                .unwrap(),
            migrated_bytes
        );
    }

    #[test]
    fn version_15_migration_reconstructs_attention_and_replays() {
        let mut global = vec![12; crate::GLOBAL_VISION_FIELDS];
        global[8] = 200;
        global[14] = 200;
        let visual = || {
            crate::VisualField::new(
                LightField::new(8, 8, global.clone()).unwrap(),
                vec![0; crate::GLOBAL_VISION_FIELDS * crate::GLOBAL_CHANGE_SUBREGIONS],
                LightField::filled(17, 17, 0).unwrap(),
            )
            .unwrap()
        };
        let sample = crate::WorldSample::new_visual(
            [visual(), visual()],
            [ContactSample::default(); TOUCH_SITES],
        )
        .unwrap();
        let mut original = WorkstationHarness::new(7).unwrap();
        original.observe(sample.clone()).unwrap();

        let checkpoint = WorkstationCheckpoint::decode(&version_15_bytes(&original)).unwrap();
        let mut migrated = WorkstationHarness::restore(checkpoint).unwrap();
        assert_eq!(migrated.visual_attention, original.visual_attention);
        assert_eq!(
            migrated.step(sample.clone()).unwrap(),
            original.step(sample).unwrap()
        );
        assert_eq!(
            WorkstationCheckpoint::decode(&migrated.save().unwrap().canonical_bytes().unwrap())
                .unwrap(),
            migrated.save().unwrap()
        );
    }
}
