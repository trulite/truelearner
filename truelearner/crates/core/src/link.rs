use crate::prelude::*;

pub(crate) const LOCAL_RETURN_STRENGTH: u32 = 3;
pub(crate) const LOCAL_VARIATION_RADIUS: i32 = 2;
pub(crate) const PARTICIPATION_IMPULSE: u64 = 1_u64 << 32;
pub(crate) const UNIT: i64 = 1_i64 << 32;
pub(crate) const UNIT_U64: u64 = 1_u64 << 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct LinkSlot(pub(crate) usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransmissionMode {
    Drive,
    Modulatory,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransmissionTrigger {
    #[default]
    SourceFires,
    QualifiedLocalParticipation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Link {
    pub from: JunctionId,
    pub to: JunctionId,
    pub delay: i64,
    pub phase: i32,
    pub coupling: i32,
    pub resistance: u32,
    pub mode: TransmissionMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LinkState {
    pub(crate) id: LinkId,
    pub(crate) from: JunctionId,
    pub(crate) to: JunctionId,
    pub(crate) delay: i64,
    pub(crate) phase: i32,
    pub(crate) coupling: i32,
    pub(crate) source_generation: Generation,
    pub(crate) target_generation: Generation,
    pub(crate) generation: Generation,
    pub(crate) resistance: u32,
    pub(crate) live: bool,
    pub(crate) participation_level: u64,
    pub(crate) plastic_support: u64,
    pub(crate) decay_load: u64,
    pub(crate) mode: TransmissionMode,
    pub(crate) trigger: TransmissionTrigger,
}

impl LinkState {
    pub(crate) fn retire(&mut self) {
        self.resistance = 0;
        self.live = false;
        self.participation_level = 0;
        self.plastic_support = 0;
        self.decay_load = 0;
        self.generation = Generation(self.generation.0.wrapping_add(1).max(1));
    }
}

impl Arena {
    pub(crate) fn add_link(&mut self, spec: Link) -> LinkId {
        self.require_junction(spec.from);
        self.require_junction(spec.to);
        assert!(spec.delay >= 0, "delay must not run backward in time");
        let source_slot = self.junction_slot(spec.from).unwrap();
        let target_slot = self.junction_slot(spec.to).unwrap();
        let source_generation = self.junctions[source_slot.0].generation;
        let target_generation = self.junctions[target_slot.0].generation;
        let reusable = self.links.iter().position(|link| !link.live);
        assert!(
            reusable.is_some() || self.link_slots.len() < self.link_capacity as usize,
            "arena has no free link identity"
        );
        let (id, slot, generation, prior) = reusable.map_or_else(
            || {
                (
                    LinkId(self.link_slots.len() as u64),
                    LinkSlot(self.links.len()),
                    Generation(1),
                    None,
                )
            },
            |index| {
                let prior = &self.links[index];
                (
                    prior.id,
                    LinkSlot(index),
                    prior.generation,
                    Some((prior.from, prior.to)),
                )
            },
        );
        let link = LinkState {
            id,
            from: spec.from,
            to: spec.to,
            delay: spec.delay,
            phase: spec.phase,
            coupling: spec.coupling,
            source_generation,
            target_generation,
            generation,
            resistance: spec.resistance,
            live: spec.resistance > 0,
            participation_level: 0,
            plastic_support: 0,
            decay_load: 0,
            mode: spec.mode,
            trigger: TransmissionTrigger::SourceFires,
        };
        if slot.0 < self.links.len() {
            if let Some((from, to)) = prior {
                self.outgoing_index[from.0 as usize].retain(|candidate| *candidate != id);
                self.incoming_index[to.0 as usize].retain(|candidate| *candidate != id);
            }
            self.links[slot.0] = link;
            self.link_slots[id.0 as usize] = Some(slot);
        } else {
            self.links.push(link);
            self.link_slots.push(Some(slot));
        }
        self.outgoing_index[spec.from.0 as usize].push(id);
        self.incoming_index[spec.to.0 as usize].push(id);
        if self.junctions[source_slot.0].region != self.junctions[target_slot.0].region {
            self.output_junctions[spec.from.0 as usize] = true;
        }
        let index = id.0 as usize;
        if self.strength.len() <= index {
            self.strength.resize(index + 1, 0);
            self.life.resize(index + 1, 0);
            self.decay_remainder.resize(index + 1, 0);
        }
        self.strength[index] = i64::from(spec.coupling).saturating_mul(UNIT);
        self.life[index] = u64::from(spec.resistance).saturating_mul(UNIT_U64);
        self.decay_remainder[index] = 0;
        if spec.resistance > 0 && spec.delay == 0 {
            self.zero_delay_live_links = self.zero_delay_live_links.saturating_add(1);
        }
        id
    }
}

impl Body {
    pub(crate) fn form_link(
        &mut self,
        from: JunctionId,
        to: JunctionId,
        strength: i64,
        delay: i64,
        work: &mut Work,
    ) -> LinkId {
        let id = self.add_link(Link {
            from,
            to,
            delay,
            phase: 0,
            coupling: strength.signum() as i32,
            resistance: 1,
            mode: TransmissionMode::Drive,
        });
        self.arena.strength[id.0 as usize] = strength;
        work.total = work.total.saturating_add(1);
        work.local_structural_proposals = work.local_structural_proposals.saturating_add(1);
        id
    }
}
