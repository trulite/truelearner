use crate::checkpoint::{validate_manifest, CheckpointError};
use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct BodySnapshot {
    pub(crate) body_version: BodyVersion,
    pub(crate) body: ArenaBody,
    clock: PhysicalClock,
    junctions: Vec<JunctionRuntime>,
    links: Vec<LinkRuntime>,
    pending: Vec<Firing>,
    protocol: Protocol,
    next_serial: u64,
    outcome_source: Option<JunctionId>,
    output_wave_open: bool,
    activation: Vec<i64>,
    strength: Vec<i64>,
    life: Vec<u64>,
    decay_remainder: Vec<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct JunctionRuntime {
    id: JunctionId,
    state: i32,
    last_update_tick: i64,
    refractory_until: i64,
    decay_load: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LinkRuntime {
    id: LinkId,
    participation_level: u64,
    plastic_support: u64,
    decay_load: u64,
    trigger: TransmissionTrigger,
}

impl Body {
    pub(crate) fn snapshot(&self, version: u64) -> Result<BodySnapshot, CheckpointError> {
        Ok(BodySnapshot {
            body_version: self.arena.version(version)?,
            body: self.arena.body(version),
            clock: self.clock(),
            junctions: self
                .arena
                .junctions
                .iter()
                .map(|junction| JunctionRuntime {
                    id: junction.id,
                    state: junction.state,
                    last_update_tick: junction.last_update_tick,
                    refractory_until: junction.refractory_until,
                    decay_load: junction.decay_load,
                })
                .collect(),
            links: self
                .arena
                .links
                .iter()
                .map(|link| LinkRuntime {
                    id: link.id,
                    participation_level: link.participation_level,
                    plastic_support: link.plastic_support,
                    decay_load: link.decay_load,
                    trigger: link.trigger,
                })
                .collect(),
            pending: self.pending.canonical(),
            protocol: self.protocol,
            next_serial: self.next_serial,
            outcome_source: self.outcome_source,
            output_wave_open: self.output_wave_open,
            activation: self.arena.activation.clone(),
            strength: self.arena.strength.clone(),
            life: self.arena.life.clone(),
            decay_remainder: self.arena.decay_remainder.clone(),
        })
    }

    pub(crate) fn from_snapshot(snapshot: BodySnapshot) -> Result<Self, CheckpointError> {
        validate_manifest(&snapshot.body_version, &snapshot.body)?;
        let mut body = Self::from_arena_body(snapshot.body)?;
        body.tick = snapshot.clock.tick;
        body.protocol = snapshot.protocol;
        body.pressure_tick = pressure_epoch(snapshot.clock.tick);
        for stored in snapshot.junctions {
            let slot = body
                .arena
                .junctions
                .iter()
                .position(|junction| junction.id == stored.id)
                .ok_or(CheckpointError::MissingJunction(stored.id))?;
            body.arena.edit_junction(slot, |junction| {
                junction.state = stored.state;
                junction.last_update_tick = stored.last_update_tick;
                junction.refractory_until = stored.refractory_until;
                junction.decay_load = stored.decay_load;
            });
        }
        for stored in snapshot.links {
            let slot = body
                .arena
                .link_slot(stored.id)
                .ok_or(CheckpointError::MissingLink(stored.id))?;
            body.arena.edit_link(slot.0, |link| {
                link.participation_level = stored.participation_level;
                link.plastic_support = stored.plastic_support;
                link.decay_load = stored.decay_load;
                link.trigger = stored.trigger;
            });
        }
        if snapshot.activation.len() != body.arena.junction_slots.len()
            || snapshot.strength.len() != body.arena.link_slots.len()
            || snapshot.life.len() != body.arena.link_slots.len()
            || snapshot.decay_remainder.len() != body.arena.link_slots.len()
        {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        body.outcome_source = snapshot.outcome_source;
        body.output_wave_open = snapshot.output_wave_open;
        body.arena.activation = snapshot.activation;
        body.arena.strength = snapshot.strength;
        body.arena.life = snapshot.life;
        body.arena.decay_remainder = snapshot.decay_remainder;
        body.pending = Schedule::from_canonical(snapshot.clock.tick, snapshot.pending);
        body.next_serial = snapshot.next_serial;
        body.arena.active_junctions = body
            .arena
            .junctions
            .iter()
            .filter(|junction| junction.state != 0)
            .map(|junction| junction.id)
            .collect();
        Ok(body)
    }
}
