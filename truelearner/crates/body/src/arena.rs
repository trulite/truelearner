use crate::physics::*;

#[repr(align(32))]
#[derive(Clone, Copy, Debug)]
pub(crate) struct LinkSlot {
    pub(crate) delay: Time,
    pub(crate) trigger: Trigger,
    pub(crate) from: JunctionId,
    pub(crate) to: JunctionId,
    pub(crate) next: Option<LinkId>,
    pub(crate) impulse: Impulse,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Arena {
    junctions: Vec<JunctionSlot>,
    links: Vec<LinkSlot>,
    outgoing_tail: Vec<Option<LinkId>>,
    incoming_head: Vec<Option<LinkId>>,
    incoming_tail: Vec<Option<LinkId>>,
    incoming_next: Vec<Option<LinkId>>,
}

impl Arena {
    pub(crate) fn junction_count(&self) -> usize {
        self.junctions.len()
    }

    pub(crate) fn link_count(&self) -> usize {
        self.links.len()
    }

    pub(crate) fn reserve(&mut self, junctions: usize, links: usize) {
        self.junctions.reserve(junctions);
        self.outgoing_tail.reserve(junctions);
        self.incoming_head.reserve(junctions);
        self.incoming_tail.reserve(junctions);
        self.links.reserve(links);
        self.incoming_next.reserve(links);
    }

    pub(crate) fn add_junction(&mut self, law: Junction) -> Result<JunctionId, BuildError> {
        let id = JunctionId::new(self.junctions.len()).ok_or(BuildError::CapacityExhausted)?;
        self.junctions.push(JunctionSlot::new(law));
        self.outgoing_tail.push(None);
        self.incoming_head.push(None);
        self.incoming_tail.push(None);
        Ok(id)
    }

    pub(crate) fn add_link(&mut self, law: Link) -> Result<LinkId, BuildError> {
        self.require(law.from)
            .map_err(BuildError::UnknownJunction)?;
        self.require(law.to).map_err(BuildError::UnknownJunction)?;
        let id = LinkId::new(self.links.len()).ok_or(BuildError::CapacityExhausted)?;
        self.links.push(LinkSlot {
            delay: law.delay,
            trigger: law.trigger,
            from: law.from,
            to: law.to,
            next: None,
            impulse: law.impulse,
        });
        self.incoming_next.push(None);
        let source = law.from.slot();
        if let Some(tail) = self.outgoing_tail[source] {
            self.links[tail.slot()].next = Some(id);
        } else {
            self.junctions[source].outgoing_head = Some(id);
        }
        self.outgoing_tail[source] = Some(id);
        let target = law.to.slot();
        if let Some(tail) = self.incoming_tail[target] {
            self.incoming_next[tail.slot()] = Some(id);
        } else {
            self.incoming_head[target] = Some(id);
        }
        self.incoming_tail[target] = Some(id);
        Ok(id)
    }

    pub(crate) fn require(&self, junction: JunctionId) -> Result<(), JunctionId> {
        self.junctions
            .get(junction.slot())
            .map(|_| ())
            .ok_or(junction)
    }

    pub(crate) fn junction(&self, id: JunctionId) -> Option<&JunctionSlot> {
        self.junctions.get(id.slot())
    }

    pub(crate) fn junction_mut(&mut self, id: JunctionId) -> Option<&mut JunctionSlot> {
        self.junctions.get_mut(id.slot())
    }

    pub(crate) fn link(&self, id: LinkId) -> Option<&LinkSlot> {
        self.links.get(id.slot())
    }

    pub(crate) fn link_mut(&mut self, id: LinkId) -> Option<&mut LinkSlot> {
        self.links.get_mut(id.slot())
    }

    pub(crate) fn incoming(&self, junction: JunctionId) -> Incoming<'_> {
        Incoming {
            arena: self,
            next: self.incoming_head.get(junction.slot()).copied().flatten(),
        }
    }

    pub(crate) fn has_link_capacity(&self, additional: usize) -> bool {
        self.links
            .len()
            .checked_add(additional)
            .is_some_and(|len| len <= u32::MAX as usize)
    }

    pub(crate) fn has_junction_capacity(&self, additional: usize) -> bool {
        self.junctions
            .len()
            .checked_add(additional)
            .is_some_and(|len| len <= u32::MAX as usize)
    }

    pub(crate) fn append(&mut self, mut other: Self) -> (usize, usize) {
        let junction_base = self.junctions.len();
        let link_base = self.links.len();
        for junction in &mut other.junctions {
            junction.outgoing_head = junction.outgoing_head.map(|id| remap_link(id, link_base));
        }
        for link in &mut other.links {
            link.from = remap_junction(link.from, junction_base);
            link.to = remap_junction(link.to, junction_base);
            link.next = link.next.map(|id| remap_link(id, link_base));
        }
        for tail in &mut other.outgoing_tail {
            *tail = tail.map(|id| remap_link(id, link_base));
        }
        for head in &mut other.incoming_head {
            *head = head.map(|id| remap_link(id, link_base));
        }
        for tail in &mut other.incoming_tail {
            *tail = tail.map(|id| remap_link(id, link_base));
        }
        for next in &mut other.incoming_next {
            *next = next.map(|id| remap_link(id, link_base));
        }
        self.junctions.append(&mut other.junctions);
        self.links.append(&mut other.links);
        self.outgoing_tail.append(&mut other.outgoing_tail);
        self.incoming_head.append(&mut other.incoming_head);
        self.incoming_tail.append(&mut other.incoming_tail);
        self.incoming_next.append(&mut other.incoming_next);
        debug_assert!(self.incoming_is_consistent());
        (junction_base, link_base)
    }

    fn incoming_is_consistent(&self) -> bool {
        let mut links = 0_usize;
        for slot in 0..self.junctions.len() {
            let junction = JunctionId::new(slot).expect("live junction identity");
            for link in self.incoming(junction) {
                if self.links[link.slot()].to != junction {
                    return false;
                }
                links += 1;
            }
        }
        links == self.links.len()
    }
}

pub(crate) struct Incoming<'a> {
    arena: &'a Arena,
    next: Option<LinkId>,
}

impl Iterator for Incoming<'_> {
    type Item = LinkId;

    fn next(&mut self) -> Option<Self::Item> {
        let link = self.next?;
        self.next = self.arena.incoming_next[link.slot()];
        Some(link)
    }
}

fn remap_junction(id: JunctionId, base: usize) -> JunctionId {
    JunctionId::new(base + id.slot()).expect("validated appended junction identity")
}

fn remap_link(id: LinkId, base: usize) -> LinkId {
    LinkId::new(base + id.slot()).expect("validated appended link identity")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn propagation_slots_are_compact() {
        eprintln!(
            "junction_slot={} junction_spec={} retention={} option_time={} option_link={} link_slot={} link_spec={} trigger={}",
            size_of::<JunctionSlot>(),
            size_of::<Junction>(),
            size_of::<Retention>(),
            size_of::<Option<Time>>(),
            size_of::<Option<LinkId>>(),
            size_of::<LinkSlot>(),
            size_of::<Link>(),
            size_of::<Trigger>()
        );
        assert_eq!(size_of::<JunctionSlot>(), 32);
        assert_eq!(size_of::<LinkSlot>(), 32);
        assert_eq!(std::mem::align_of::<JunctionSlot>(), 32);
        assert_eq!(std::mem::align_of::<LinkSlot>(), 32);
        assert_eq!(size_of::<Option<JunctionId>>(), size_of::<JunctionId>());
        assert_eq!(size_of::<Option<LinkId>>(), size_of::<LinkId>());
    }

    #[test]
    fn incoming_incidence_preserves_source_and_construction_order() {
        let mut arena = Arena::default();
        let left = arena.add_junction(Junction::integrating(1)).unwrap();
        let right = arena.add_junction(Junction::integrating(1)).unwrap();
        let target = arena.add_junction(Junction::integrating(1)).unwrap();
        let first = arena.add_link(Link::new(left, target, 1, 2)).unwrap();
        let second = arena.add_link(Link::new(right, target, 3, 4)).unwrap();

        assert_eq!(arena.incoming(target).collect::<Vec<_>>(), [first, second]);
        assert_eq!(arena.link(first).unwrap().from, left);
        assert_eq!(arena.link(second).unwrap().from, right);
        assert!(arena.incoming_is_consistent());
    }

    #[test]
    fn append_remaps_both_ends_and_incoming_incidence() {
        let mut host = Arena::default();
        host.add_junction(Junction::integrating(1)).unwrap();

        let mut part = Arena::default();
        let left = part.add_junction(Junction::integrating(1)).unwrap();
        let right = part.add_junction(Junction::integrating(1)).unwrap();
        let target = part.add_junction(Junction::integrating(1)).unwrap();
        let first = part.add_link(Link::new(left, target, 1, 1)).unwrap();
        let second = part.add_link(Link::new(right, target, 1, 1)).unwrap();

        let (junction_base, link_base) = host.append(part);
        let left = remap_junction(left, junction_base);
        let right = remap_junction(right, junction_base);
        let target = remap_junction(target, junction_base);
        let first = remap_link(first, link_base);
        let second = remap_link(second, link_base);

        assert_eq!(host.incoming(target).collect::<Vec<_>>(), [first, second]);
        assert_eq!(host.link(first).unwrap().from, left);
        assert_eq!(host.link(second).unwrap().from, right);
        assert!(host.incoming_is_consistent());
    }
}
