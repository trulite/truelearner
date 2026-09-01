use crate::{Automaticity, AutomaticityV7, Body, Junction, Link, LinkMemory};
use bincode::Options;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

const MAGIC: &[u8; 8] = b"TLBODY01";
const VERSION: u16 = 8;
const PREVIOUS_VERSION: u16 = 7;
const HEADER_LEN: usize = 50;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct JunctionRecord {
    law: Junction,
    stamp: u64,
    value: i32,
    sampled_known: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LinkRecord {
    law: Link,
    memory: LinkMemory,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Payload {
    now: u64,
    junctions: Vec<JunctionRecord>,
    links: Vec<LinkRecord>,
    automaticity: Option<Box<Automaticity>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct PayloadV7 {
    now: u64,
    junctions: Vec<JunctionRecord>,
    links: Vec<LinkRecord>,
    automaticity: Option<Box<AutomaticityV7>>,
}

impl From<PayloadV7> for Payload {
    fn from(previous: PayloadV7) -> Self {
        Self {
            now: previous.now,
            junctions: previous.junctions,
            links: previous.links,
            automaticity: previous
                .automaticity
                .map(|automaticity| Box::new((*automaticity).into())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyCheckpoint {
    payload: Payload,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyCheckpointError {
    BodyNotQuiet,
    Truncated,
    WrongMagic,
    UnsupportedVersion(u16),
    Checksum,
    TrailingBytes,
    Invalid,
}

impl fmt::Display for BodyCheckpointError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BodyNotQuiet => formatter.write_str("body checkpoint requires a quiet body"),
            Self::Truncated => formatter.write_str("body checkpoint is truncated"),
            Self::WrongMagic => formatter.write_str("body checkpoint has the wrong magic"),
            Self::UnsupportedVersion(version) => {
                write!(formatter, "unsupported body checkpoint version {version}")
            }
            Self::Checksum => formatter.write_str("body checkpoint checksum differs"),
            Self::TrailingBytes => formatter.write_str("body checkpoint has trailing bytes"),
            Self::Invalid => formatter.write_str("body checkpoint is invalid"),
        }
    }
}

impl std::error::Error for BodyCheckpointError {}

impl Body {
    pub fn checkpoint(&self) -> Result<BodyCheckpoint, BodyCheckpointError> {
        if !self.is_quiet() {
            return Err(BodyCheckpointError::BodyNotQuiet);
        }
        let junctions = self
            .arena
            .junctions()
            .map(|slot| {
                let (stamp, value, sampled_known) = slot.checkpoint_state();
                JunctionRecord {
                    law: slot.checkpoint_law(),
                    stamp,
                    value,
                    sampled_known,
                }
            })
            .collect();
        let links = self
            .arena
            .links()
            .zip(&self.link_memory)
            .map(|(slot, memory)| LinkRecord {
                law: slot.checkpoint_law(),
                memory: memory.clone(),
            })
            .collect();
        Ok(BodyCheckpoint {
            payload: Payload {
                now: self.now(),
                junctions,
                links,
                automaticity: self.automaticity.clone(),
            },
        })
    }
}

impl BodyCheckpoint {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, BodyCheckpointError> {
        let payload = options()
            .serialize(&self.payload)
            .map_err(|_| BodyCheckpointError::Invalid)?;
        let length = u64::try_from(payload.len()).map_err(|_| BodyCheckpointError::Invalid)?;
        let mut bytes = Vec::with_capacity(HEADER_LEN.saturating_add(payload.len()));
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes.extend_from_slice(&Sha256::digest(&payload));
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, BodyCheckpointError> {
        if bytes.len() < HEADER_LEN {
            return Err(BodyCheckpointError::Truncated);
        }
        if &bytes[..8] != MAGIC {
            return Err(BodyCheckpointError::WrongMagic);
        }
        let version = u16::from_le_bytes(
            bytes[8..10]
                .try_into()
                .map_err(|_| BodyCheckpointError::Invalid)?,
        );
        if version != VERSION && version != PREVIOUS_VERSION {
            return Err(BodyCheckpointError::UnsupportedVersion(version));
        }
        let length = usize::try_from(u64::from_le_bytes(
            bytes[10..18]
                .try_into()
                .map_err(|_| BodyCheckpointError::Invalid)?,
        ))
        .map_err(|_| BodyCheckpointError::Invalid)?;
        let expected = HEADER_LEN
            .checked_add(length)
            .ok_or(BodyCheckpointError::Invalid)?;
        match bytes.len().cmp(&expected) {
            std::cmp::Ordering::Less => return Err(BodyCheckpointError::Truncated),
            std::cmp::Ordering::Greater => return Err(BodyCheckpointError::TrailingBytes),
            std::cmp::Ordering::Equal => {}
        }
        let payload = &bytes[HEADER_LEN..];
        if Sha256::digest(payload).as_slice() != &bytes[18..HEADER_LEN] {
            return Err(BodyCheckpointError::Checksum);
        }
        let payload = if version == VERSION {
            options()
                .deserialize(payload)
                .map_err(|_| BodyCheckpointError::Invalid)?
        } else {
            let previous: PayloadV7 = options()
                .deserialize(payload)
                .map_err(|_| BodyCheckpointError::Invalid)?;
            previous.into()
        };
        Ok(Self { payload })
    }

    pub fn restore(self) -> Result<Body, BodyCheckpointError> {
        let mut body = Body::default();
        body.reserve(self.payload.junctions.len(), self.payload.links.len());
        for record in self.payload.junctions {
            let id = body
                .add_junction(record.law)
                .map_err(|_| BodyCheckpointError::Invalid)?;
            body.arena
                .junction_mut(id)
                .expect("new checkpoint junction exists")
                .restore_state(record.stamp, record.value, record.sampled_known);
        }
        for record in self.payload.links {
            let id = body
                .add_link(record.law)
                .map_err(|_| BodyCheckpointError::Invalid)?;
            body.link_memory[id.slot()] = record.memory;
        }
        body.restore_checkpoint_time(self.payload.now);
        body.automaticity = self.payload.automaticity;
        body.rebuild_live_returns();
        Ok(body)
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
    use crate::{
        harness::{attach_outcome_component, attach_sensor, finish, motor, reading, schedule},
        Arrival, Junction, TraceEvent, Trigger,
    };

    #[derive(Serialize)]
    enum VersionEightTrigger {
        SourceFires,
        RisesThrough(i32),
        FallsThrough(i32),
    }

    #[derive(Serialize)]
    struct VersionEightLink {
        from: crate::JunctionId,
        to: crate::JunctionId,
        delay: u64,
        impulse: i32,
        trigger: VersionEightTrigger,
    }

    #[derive(Serialize)]
    struct VersionEightLinkRecord {
        law: VersionEightLink,
        memory: LinkMemory,
    }

    #[derive(Serialize)]
    struct VersionEightPayload {
        now: u64,
        junctions: Vec<JunctionRecord>,
        links: Vec<VersionEightLinkRecord>,
        automaticity: Option<Box<Automaticity>>,
    }

    fn body_with_open_return() -> (Body, crate::JunctionId) {
        let mut body = Body::default();
        let motor = motor(&mut body);
        let sensor = attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        );
        let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        schedule(&mut body, 0, &[Arrival::caused(outcome, 0, 0)]);
        finish(&mut body);
        schedule(&mut body, 1, &[reading(sensor, 0, 1, 1)]);
        schedule(&mut body, 2, &[Arrival::caused(motor.opportunity, 1, 1)]);
        finish(&mut body);
        (body, outcome)
    }

    #[test]
    fn checkpoint_restores_the_exact_next_wave() {
        let (body, outcome) = body_with_open_return();
        let bytes = body.checkpoint().unwrap().canonical_bytes().unwrap();
        let mut plain = body;
        let mut restored = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();
        let mut histories = Vec::new();
        for candidate in [&mut plain, &mut restored] {
            candidate
                .inputs(candidate.now() + 1, &[Arrival::caused(outcome, 1, 1)])
                .unwrap();
            let mut physical = Vec::new();
            let mut trace = Vec::<TraceEvent>::new();
            candidate
                .run_traced(8, |event| physical.push(event), |event| trace.push(event))
                .unwrap();
            histories.push((physical, trace));
        }
        assert_eq!(histories[0], histories[1]);
        assert_eq!(
            plain.checkpoint().unwrap().canonical_bytes().unwrap(),
            restored.checkpoint().unwrap().canonical_bytes().unwrap()
        );
    }

    #[test]
    fn version_eight_threshold_triggers_keep_their_checkpoint_meaning() {
        let first = crate::JunctionId::new(0).unwrap();
        let second = crate::JunctionId::new(1).unwrap();
        let payload = VersionEightPayload {
            now: 0,
            junctions: vec![
                JunctionRecord {
                    law: Junction::sampled(100),
                    stamp: 0,
                    value: 0,
                    sampled_known: false,
                },
                JunctionRecord {
                    law: Junction::integrating(1),
                    stamp: 0,
                    value: 0,
                    sampled_known: false,
                },
            ],
            links: vec![
                VersionEightLinkRecord {
                    law: VersionEightLink {
                        from: first,
                        to: second,
                        delay: 0,
                        impulse: 1,
                        trigger: VersionEightTrigger::SourceFires,
                    },
                    memory: LinkMemory::default(),
                },
                VersionEightLinkRecord {
                    law: VersionEightLink {
                        from: first,
                        to: second,
                        delay: 1,
                        impulse: 1,
                        trigger: VersionEightTrigger::RisesThrough(5),
                    },
                    memory: LinkMemory::default(),
                },
                VersionEightLinkRecord {
                    law: VersionEightLink {
                        from: first,
                        to: second,
                        delay: 2,
                        impulse: -1,
                        trigger: VersionEightTrigger::FallsThrough(-5),
                    },
                    memory: LinkMemory::default(),
                },
            ],
            automaticity: None,
        };
        let payload = options().serialize(&payload).unwrap();
        let mut bytes = Vec::with_capacity(HEADER_LEN + payload.len());
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&Sha256::digest(&payload));
        bytes.extend_from_slice(&payload);

        let body = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();
        let triggers = body
            .arena
            .links()
            .map(|link| link.checkpoint_law().trigger)
            .collect::<Vec<_>>();

        assert_eq!(
            triggers,
            [
                Trigger::SourceFires,
                Trigger::RisesThrough(5),
                Trigger::FallsThrough(-5)
            ]
        );
    }

    #[test]
    fn version_seven_checkpoint_restores_with_empty_dependent_thought_state() {
        let payload = PayloadV7 {
            now: 0,
            junctions: Vec::new(),
            links: Vec::new(),
            automaticity: Some(Box::new(AutomaticityV7::default())),
        };
        let payload = options().serialize(&payload).unwrap();
        let mut bytes = Vec::with_capacity(HEADER_LEN + payload.len());
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&PREVIOUS_VERSION.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&Sha256::digest(&payload));
        bytes.extend_from_slice(&payload);

        let body = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();

        assert_eq!(body.reentry_state().thought_shortcuts, 0);
        assert_eq!(body.automaticity_work(), crate::AutomaticityWork::default());
    }

    #[test]
    fn corruption_and_nonquiet_save_fail_closed() {
        let (mut body, outcome) = body_with_open_return();
        let bytes = body.checkpoint().unwrap().canonical_bytes().unwrap();
        body.input(body.now() + 1, outcome, 1).unwrap();
        assert_eq!(body.checkpoint(), Err(BodyCheckpointError::BodyNotQuiet));

        let mut corrupt = bytes.clone();
        corrupt[HEADER_LEN] ^= 1;
        assert_eq!(
            BodyCheckpoint::decode(&corrupt),
            Err(BodyCheckpointError::Checksum)
        );
        assert_eq!(
            BodyCheckpoint::decode(&bytes[..HEADER_LEN - 1]),
            Err(BodyCheckpointError::Truncated)
        );
    }
}
