use crate::prelude::*;

/// The physical topology present in one arena.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arena {
    pub(crate) junctions: Vec<JunctionState>,
    pub(crate) junction_slots: Vec<Option<JunctionSlot>>,
    pub(crate) links: Vec<LinkState>,
    pub(crate) link_slots: Vec<Option<LinkSlot>>,
    pub(crate) junction_capacity: u32,
    pub(crate) link_capacity: u32,
    pub(crate) outgoing_index: Vec<Vec<LinkId>>,
    pub(crate) incoming_index: Vec<Vec<LinkId>>,
    pub(crate) zero_delay_live_links: usize,
    pub(crate) output_junctions: Vec<bool>,
    pub(crate) outputs_by_position: BTreeMap<i32, Vec<JunctionId>>,
    pub(crate) activation: Vec<i64>,
    pub(crate) strength: Vec<i64>,
    pub(crate) life: Vec<u64>,
    pub(crate) decay_remainder: Vec<u64>,
    pub(crate) active_junctions: HashSet<JunctionId>,
    pub(crate) aging_links: BTreeSet<LinkId>,
}

impl Arena {
    pub(crate) fn new(junction_capacity: u32, link_capacity: u32) -> Self {
        Self {
            junctions: Vec::new(),
            junction_slots: Vec::new(),
            links: Vec::new(),
            link_slots: Vec::new(),
            junction_capacity,
            link_capacity,
            outgoing_index: Vec::new(),
            incoming_index: Vec::new(),
            zero_delay_live_links: 0,
            output_junctions: Vec::new(),
            outputs_by_position: BTreeMap::new(),
            activation: Vec::new(),
            strength: Vec::new(),
            life: Vec::new(),
            decay_remainder: Vec::new(),
            active_junctions: HashSet::new(),
            aging_links: BTreeSet::new(),
        }
    }

    pub(crate) fn junction_snapshot(&self, slot: usize) -> JunctionState {
        self.junctions[slot].clone()
    }

    pub(crate) fn edit_junction<R>(
        &mut self,
        slot: usize,
        edit: impl FnOnce(&mut JunctionState) -> R,
    ) -> R {
        edit(&mut self.junctions[slot])
    }

    pub(crate) fn link_snapshot(&self, slot: usize) -> LinkState {
        self.links[slot].clone()
    }

    pub(crate) fn edit_link<R>(
        &mut self,
        slot: usize,
        edit: impl FnOnce(&mut LinkState) -> R,
    ) -> R {
        edit(&mut self.links[slot])
    }

    pub(crate) fn require_junction(&self, id: JunctionId) {
        let valid = self.junction_slot(id).is_some_and(|slot| {
            let junction = &self.junctions[slot.0];
            junction.id == id && junction.live
        });
        assert!(valid, "junction must be live in this body");
    }

    pub(crate) fn junction_slot(&self, id: JunctionId) -> Option<JunctionSlot> {
        usize::try_from(id.0)
            .ok()
            .and_then(|index| self.junction_slots.get(index))
            .copied()
            .flatten()
    }

    pub(crate) fn link_slot(&self, id: LinkId) -> Option<LinkSlot> {
        usize::try_from(id.0)
            .ok()
            .and_then(|index| self.link_slots.get(index))
            .copied()
            .flatten()
    }

    pub(crate) fn rebuild_indexes(&mut self) {
        self.outgoing_index = vec![Vec::new(); self.junction_slots.len()];
        self.incoming_index = vec![Vec::new(); self.junction_slots.len()];
        for link in &self.links {
            self.outgoing_index[link.from.0 as usize].push(link.id);
            self.incoming_index[link.to.0 as usize].push(link.id);
        }
        self.active_junctions = self
            .junctions
            .iter()
            .filter(|junction| junction.state != 0)
            .map(|junction| junction.id)
            .collect();
        self.zero_delay_live_links = self
            .links
            .iter()
            .filter(|link| link.live && link.delay == 0)
            .count();
        self.aging_links = self
            .links
            .iter()
            .filter(|link| link.live && link.resistance < u32::MAX)
            .map(|link| link.id)
            .collect();
        self.find_output_junctions();
    }

    pub(crate) fn memory_bytes(&self) -> usize {
        self.junctions.len() * std::mem::size_of::<JunctionState>()
            + self.links.len() * std::mem::size_of::<LinkState>()
    }

    pub(crate) fn allocated_bytes(&self) -> usize {
        self.junctions.capacity() * std::mem::size_of::<JunctionState>()
            + self.links.capacity() * std::mem::size_of::<LinkState>()
            + self.junction_slots.len() * std::mem::size_of::<Option<JunctionSlot>>()
            + self.link_slots.len() * std::mem::size_of::<Option<LinkSlot>>()
            + indexed_bytes(&self.outgoing_index)
            + indexed_bytes(&self.incoming_index)
            + self.active_junctions.len()
                * (std::mem::size_of::<JunctionId>() + 3 * std::mem::size_of::<usize>())
            + self.output_junctions.len() * std::mem::size_of::<bool>()
    }

    pub(crate) fn junction_by_id(&self, id: JunctionId) -> Option<&JunctionState> {
        let slot = self.junction_slots.get(id.0 as usize)?.as_ref()?;
        self.junctions.get(slot.0)
    }

    pub(crate) fn link_by_id(&self, id: LinkId) -> Option<&LinkState> {
        let slot = self.link_slots.get(id.0 as usize)?.as_ref()?;
        self.links.get(slot.0)
    }
}

fn indexed_bytes(index: &[Vec<LinkId>]) -> usize {
    std::mem::size_of_val(index)
        + index
            .iter()
            .map(|links| links.len() * std::mem::size_of::<LinkId>())
            .sum::<usize>()
}
