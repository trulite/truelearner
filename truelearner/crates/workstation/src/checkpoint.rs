use crate::harness::{FocusedVision, ResearchFocusedActionProjection, RetinalState, Sites};
use crate::{WorkstationError, WorkstationState, AXIS_COUNT};
use bincode::Options;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use truelearner_core::Checkpoint;

const MAGIC: &[u8; 8] = b"TLWORK01";
const LEGACY_VERSION: u16 = 1;
const VERSION_TWO: u16 = 2;
const VERSION_THREE: u16 = 3;
const VERSION: u16 = 4;
const LEGACY_LAYOUT_VERSION: u16 = 1;
const VERSION_TWO_LAYOUT: u16 = 2;
const VERSION_THREE_LAYOUT: u16 = 3;
const LAYOUT_VERSION: u16 = 4;
const HEADER_LEN: usize = 50;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CheckpointPayload {
    pub core: Vec<u8>,
    pub state: WorkstationState,
    pub sites: Sites,
    pub sequence: u64,
    pub pending_transitions: [bool; AXIS_COUNT],
    pub retinal_state: RetinalState,
    pub focused_vision: Option<FocusedVision>,
    pub layout_version: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct VersionTwoCheckpointPayload {
    core: Vec<u8>,
    state: WorkstationState,
    sites: Sites,
    sequence: u64,
    pending_transitions: [bool; AXIS_COUNT],
    retinal_state: RetinalState,
    layout_version: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct VersionThreeFocusedVision {
    sensors: Vec<[truelearner_core::JunctionId; 3]>,
    relays: Vec<truelearner_core::JunctionId>,
    previous: Option<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct VersionThreeCheckpointPayload {
    core: Vec<u8>,
    state: WorkstationState,
    sites: Sites,
    sequence: u64,
    pending_transitions: [bool; AXIS_COUNT],
    retinal_state: RetinalState,
    focused_vision: Option<VersionThreeFocusedVision>,
    layout_version: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyCheckpointPayload {
    core: Vec<u8>,
    state: WorkstationState,
    sites: Sites,
    sequence: u64,
    pending_transitions: [bool; AXIS_COUNT],
    layout_version: u16,
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
        retinal_state: RetinalState,
        focused_vision: Option<FocusedVision>,
    ) -> Self {
        Self {
            payload: CheckpointPayload {
                core,
                state,
                sites,
                sequence,
                pending_transitions,
                retinal_state,
                focused_vision,
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
        if !matches!(
            version,
            LEGACY_VERSION | VERSION_TWO | VERSION_THREE | VERSION
        ) {
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
        let payload = match version {
            LEGACY_VERSION => {
                let legacy: LegacyCheckpointPayload = options()
                    .deserialize(encoded)
                    .map_err(|_| WorkstationError::InvalidCheckpoint)?;
                validate_legacy_payload(&legacy)?;
                CheckpointPayload {
                    core: legacy.core,
                    state: legacy.state,
                    sites: legacy.sites,
                    sequence: legacy.sequence,
                    pending_transitions: legacy.pending_transitions,
                    retinal_state: RetinalState::default(),
                    focused_vision: None,
                    layout_version: LAYOUT_VERSION,
                }
            }
            VERSION_TWO => {
                let previous: VersionTwoCheckpointPayload = options()
                    .deserialize(encoded)
                    .map_err(|_| WorkstationError::InvalidCheckpoint)?;
                validate_version_two_payload(&previous)?;
                CheckpointPayload {
                    core: previous.core,
                    state: previous.state,
                    sites: previous.sites,
                    sequence: previous.sequence,
                    pending_transitions: previous.pending_transitions,
                    retinal_state: previous.retinal_state,
                    focused_vision: None,
                    layout_version: LAYOUT_VERSION,
                }
            }
            VERSION_THREE => {
                let previous: VersionThreeCheckpointPayload = options()
                    .deserialize(encoded)
                    .map_err(|_| WorkstationError::InvalidCheckpoint)?;
                validate_version_three_payload(&previous)?;
                CheckpointPayload {
                    core: previous.core,
                    state: previous.state,
                    sites: previous.sites,
                    sequence: previous.sequence,
                    pending_transitions: previous.pending_transitions,
                    retinal_state: previous.retinal_state,
                    focused_vision: previous.focused_vision.map(|focused| FocusedVision {
                        sensors: focused.sensors,
                        relays: focused.relays,
                        previous: focused.previous,
                        action_projection: ResearchFocusedActionProjection::Isolated,
                    }),
                    layout_version: LAYOUT_VERSION,
                }
            }
            VERSION => options()
                .deserialize(encoded)
                .map_err(|_| WorkstationError::InvalidCheckpoint)?,
            _ => return Err(WorkstationError::UnsupportedCheckpointVersion(version)),
        };
        validate_payload(&payload)?;
        Ok(Self { payload })
    }
}

fn validate_version_three_payload(
    payload: &VersionThreeCheckpointPayload,
) -> Result<(), WorkstationError> {
    if payload.layout_version != VERSION_THREE_LAYOUT {
        return Err(WorkstationError::InvalidCheckpoint);
    }
    payload.state.validate()?;
    payload.sites.validate()?;
    if let Some(focused) = &payload.focused_vision {
        FocusedVision {
            sensors: focused.sensors.clone(),
            relays: focused.relays.clone(),
            previous: focused.previous.clone(),
            action_projection: ResearchFocusedActionProjection::Isolated,
        }
        .validate()?;
    }
    Checkpoint::decode(&payload.core)
        .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
    Ok(())
}

fn validate_version_two_payload(
    payload: &VersionTwoCheckpointPayload,
) -> Result<(), WorkstationError> {
    if payload.layout_version != VERSION_TWO_LAYOUT {
        return Err(WorkstationError::InvalidCheckpoint);
    }
    payload.state.validate()?;
    payload.sites.validate()?;
    Checkpoint::decode(&payload.core)
        .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
    Ok(())
}

fn validate_legacy_payload(payload: &LegacyCheckpointPayload) -> Result<(), WorkstationError> {
    if payload.layout_version != LEGACY_LAYOUT_VERSION {
        return Err(WorkstationError::InvalidCheckpoint);
    }
    payload.state.validate()?;
    payload.sites.validate()?;
    Checkpoint::decode(&payload.core)
        .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
    Ok(())
}

fn validate_payload(payload: &CheckpointPayload) -> Result<(), WorkstationError> {
    if payload.layout_version != LAYOUT_VERSION {
        return Err(WorkstationError::InvalidCheckpoint);
    }
    payload.state.validate()?;
    payload.sites.validate()?;
    if let Some(focused_vision) = &payload.focused_vision {
        focused_vision.validate()?;
    }
    Checkpoint::decode(&payload.core)
        .map_err(|error| WorkstationError::CoreCheckpoint(format!("{error:?}")))?;
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
    fn version_one_checkpoint_migrates_with_empty_retinal_state() {
        let current = WorkstationHarness::new(1)
            .unwrap()
            .save()
            .unwrap()
            .open()
            .unwrap();
        let legacy = LegacyCheckpointPayload {
            core: current.core,
            state: current.state,
            sites: current.sites,
            sequence: 7,
            pending_transitions: current.pending_transitions,
            layout_version: LEGACY_LAYOUT_VERSION,
        };
        let payload = options().serialize(&legacy).unwrap();
        let mut encoded = Vec::with_capacity(HEADER_LEN + payload.len());
        encoded.extend_from_slice(MAGIC);
        encoded.extend_from_slice(&LEGACY_VERSION.to_le_bytes());
        encoded.extend_from_slice(&u64::try_from(payload.len()).unwrap().to_le_bytes());
        encoded.extend_from_slice(&Sha256::digest(&payload));
        encoded.extend_from_slice(&payload);

        let migrated = WorkstationCheckpoint::decode(&encoded)
            .unwrap()
            .open()
            .unwrap();
        assert_eq!(migrated.sequence, 7);
        assert_eq!(migrated.retinal_state, RetinalState::default());
        assert!(migrated.focused_vision.is_none());
        assert_eq!(migrated.layout_version, LAYOUT_VERSION);
    }

    #[test]
    fn version_two_checkpoint_migrates_without_a_focused_organ() {
        let current = WorkstationHarness::new(2)
            .unwrap()
            .save()
            .unwrap()
            .open()
            .unwrap();
        let previous = VersionTwoCheckpointPayload {
            core: current.core,
            state: current.state,
            sites: current.sites,
            sequence: 11,
            pending_transitions: current.pending_transitions,
            retinal_state: current.retinal_state,
            layout_version: VERSION_TWO_LAYOUT,
        };
        let payload = options().serialize(&previous).unwrap();
        let mut encoded = Vec::with_capacity(HEADER_LEN + payload.len());
        encoded.extend_from_slice(MAGIC);
        encoded.extend_from_slice(&VERSION_TWO.to_le_bytes());
        encoded.extend_from_slice(&u64::try_from(payload.len()).unwrap().to_le_bytes());
        encoded.extend_from_slice(&Sha256::digest(&payload));
        encoded.extend_from_slice(&payload);

        let migrated = WorkstationCheckpoint::decode(&encoded)
            .unwrap()
            .open()
            .unwrap();
        assert_eq!(migrated.sequence, 11);
        assert_eq!(migrated.retinal_state, previous.retinal_state);
        assert!(migrated.focused_vision.is_none());
        assert_eq!(migrated.layout_version, LAYOUT_VERSION);
    }

    #[cfg(feature = "research")]
    #[test]
    fn version_three_focused_checkpoint_migrates_as_isolated() {
        use crate::{
            Protocol, ResearchHarnessConfig, ResearchOpportunityIncidence,
            ResearchTransitionOpportunity, ResearchVisualComposition,
        };

        let current = WorkstationHarness::new_research_composed(
            3,
            ResearchHarnessConfig {
                protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
                opportunity_incidence: ResearchOpportunityIncidence::Independent,
                transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
            },
            ResearchVisualComposition::default().with_focused_sensor_field(true),
        )
        .unwrap()
        .save()
        .unwrap()
        .open()
        .unwrap();
        let focused = current.focused_vision.unwrap();
        let previous = VersionThreeCheckpointPayload {
            core: current.core,
            state: current.state,
            sites: current.sites,
            sequence: 13,
            pending_transitions: current.pending_transitions,
            retinal_state: current.retinal_state,
            focused_vision: Some(VersionThreeFocusedVision {
                sensors: focused.sensors,
                relays: focused.relays,
                previous: focused.previous,
            }),
            layout_version: VERSION_THREE_LAYOUT,
        };
        let payload = options().serialize(&previous).unwrap();
        let mut encoded = Vec::with_capacity(HEADER_LEN + payload.len());
        encoded.extend_from_slice(MAGIC);
        encoded.extend_from_slice(&VERSION_THREE.to_le_bytes());
        encoded.extend_from_slice(&u64::try_from(payload.len()).unwrap().to_le_bytes());
        encoded.extend_from_slice(&Sha256::digest(&payload));
        encoded.extend_from_slice(&payload);

        let migrated = WorkstationCheckpoint::decode(&encoded)
            .unwrap()
            .open()
            .unwrap();
        assert_eq!(migrated.sequence, 13);
        assert_eq!(
            migrated.focused_vision.unwrap().action_projection,
            ResearchFocusedActionProjection::Isolated
        );
        assert_eq!(migrated.layout_version, LAYOUT_VERSION);
    }
}
