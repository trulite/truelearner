use crate::checkpoint::CheckpointError;
use crate::prelude::*;

impl Arena {
    pub(crate) fn body(&self, version: u64) -> ArenaBody {
        let minimum_position = self
            .junctions
            .iter()
            .map(|junction| junction.position)
            .min()
            .unwrap_or(0);
        let maximum_position = self
            .junctions
            .iter()
            .map(|junction| junction.position)
            .max()
            .unwrap_or(0);
        ArenaBody {
            arena: self.id,
            version,
            minimum_position,
            maximum_position,
            cell_capacity: self.junction_capacity,
            arrow_capacity: self.link_capacity,
            cells: self
                .junctions
                .iter()
                .map(|junction| DurableJunction {
                    id: junction.id,
                    generation: junction.generation,
                    physical_id: junction.physical_id,
                    position: junction.position,
                    region: junction.region,
                    threshold: junction.threshold,
                    resistance: junction.resistance,
                    live: junction.live,
                })
                .collect(),
            arrows: self
                .links
                .iter()
                .map(|link| DurableLink {
                    id: link.id,
                    generation: link.generation,
                    from: JunctionRef {
                        arena: self.id,
                        id: link.from,
                        generation: link.source_generation,
                    },
                    to: JunctionRef {
                        arena: self.id,
                        id: link.to,
                        generation: link.target_generation,
                    },
                    delay: link.delay,
                    phase: link.phase,
                    coupling: link.coupling,
                    resistance: link.resistance,
                    transmission_mode: mode_byte(link.mode),
                    live: link.live,
                })
                .collect(),
        }
    }

    pub(crate) fn from_body(mut body: ArenaBody) -> Result<Self, CheckpointError> {
        body.validate()?;
        body.cells.sort_by_key(|junction| junction.id);
        body.arrows.sort_by_key(|link| link.id);
        let mut arena = Self::new(body.arena, body.cell_capacity, body.arrow_capacity);
        let last_junction = body.cells.iter().map(|junction| junction.id.0).max();
        arena.junction_slots = last_junction
            .map(|last| vec![None; last as usize + 1])
            .unwrap_or_default();
        for stored in body.cells {
            if stored.threshold <= 0 {
                return Err(CheckpointError::InvalidPhysicalBody);
            }
            let slot = JunctionSlot(arena.junctions.len());
            arena.junction_slots[stored.id.0 as usize] = stored.live.then_some(slot);
            arena.junctions.push(JunctionState {
                id: stored.id,
                physical_id: stored.physical_id,
                position: stored.position,
                region: stored.region,
                threshold: stored.threshold,
                state: 0,
                last_update_tick: 0,
                refractory_until: 0,
                generation: stored.generation,
                resistance: stored.resistance,
                live: stored.live,
                decay_load: 0,
            });
        }
        arena.activation = vec![0; arena.junction_slots.len()];
        arena.outgoing_index = vec![Vec::new(); arena.junction_slots.len()];
        arena.incoming_index = vec![Vec::new(); arena.junction_slots.len()];
        arena.output_junctions = vec![false; arena.junction_slots.len()];

        let last_link = body.arrows.iter().map(|link| link.id.0).max();
        arena.link_slots = last_link
            .map(|last| vec![None; last as usize + 1])
            .unwrap_or_default();
        arena.strength = vec![0; arena.link_slots.len()];
        arena.life = vec![0; arena.link_slots.len()];
        arena.decay_remainder = vec![0; arena.link_slots.len()];
        for stored in body.arrows {
            if stored.from.arena != arena.id || stored.to.arena != arena.id {
                return Err(CheckpointError::MissingJunction(stored.from.id));
            }
            let from = arena
                .junctions
                .iter()
                .find(|junction| junction.id == stored.from.id)
                .ok_or(CheckpointError::MissingJunction(stored.from.id))?;
            let to = arena
                .junctions
                .iter()
                .find(|junction| junction.id == stored.to.id)
                .ok_or(CheckpointError::MissingJunction(stored.to.id))?;
            if stored.live
                && (from.generation != stored.from.generation
                    || to.generation != stored.to.generation)
            {
                return Err(CheckpointError::StaleJunctionReference(
                    if from.generation != stored.from.generation {
                        stored.from
                    } else {
                        stored.to
                    },
                ));
            }
            if stored.live && (!from.live || !to.live) {
                return Err(CheckpointError::InvalidPhysicalBody);
            }
            if stored.delay < 0 || stored.live != (stored.resistance > 0) {
                return Err(CheckpointError::InvalidPhysicalBody);
            }
            let slot = LinkSlot(arena.links.len());
            arena.link_slots[stored.id.0 as usize] = Some(slot);
            arena.strength[stored.id.0 as usize] = i64::from(stored.coupling) * UNIT;
            arena.life[stored.id.0 as usize] =
                u64::from(stored.resistance).saturating_mul(UNIT_U64);
            arena.links.push(LinkState {
                id: stored.id,
                from: stored.from.id,
                to: stored.to.id,
                delay: stored.delay,
                phase: stored.phase,
                coupling: stored.coupling,
                source_generation: stored.from.generation,
                target_generation: stored.to.generation,
                generation: stored.generation,
                resistance: stored.resistance,
                live: stored.live,
                participation_level: 0,
                plastic_support: 0,
                decay_load: 0,
                mode: mode(stored.transmission_mode)?,
                trigger: TransmissionTrigger::SourceFires,
            });
        }
        arena.rebuild_indexes();
        Ok(arena)
    }

    pub(crate) fn version(&self, version: u64) -> Result<BodyVersion, CheckpointError> {
        let body = self.body(version);
        Ok(BodyVersion {
            version,
            parent: None,
            arenas: vec![ArenaVersion {
                arena: self.id,
                block: body.content_hash()?,
            }],
        })
    }
}

impl Body {
    pub fn arena_body(&self, version: u64) -> ArenaBody {
        self.arena.body(version)
    }

    pub fn canonical_body_bytes(&self, version: u64) -> Result<Vec<u8>, FormatError> {
        self.arena_body(version).canonical_bytes()
    }

    pub fn from_body_bytes(bytes: &[u8]) -> Result<Self, CheckpointError> {
        Self::from_arena_body(ArenaBody::decode(bytes)?)
    }

    pub fn from_arena_body(body: ArenaBody) -> Result<Self, CheckpointError> {
        Ok(Self::from_arena(Arena::from_body(body)?))
    }
}

fn mode_byte(mode: TransmissionMode) -> u8 {
    match mode {
        TransmissionMode::Drive => 0,
        TransmissionMode::Modulatory => 1,
    }
}

fn mode(mode: u8) -> Result<TransmissionMode, CheckpointError> {
    match mode {
        0 => Ok(TransmissionMode::Drive),
        1 => Ok(TransmissionMode::Modulatory),
        other => Err(CheckpointError::UnsupportedTransmissionMode(other)),
    }
}
