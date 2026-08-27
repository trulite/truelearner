use crate::checkpoint::CheckpointError;
use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct BodySnapshot {
    arena: ArenaSnapshot,
    clock: PhysicalClock,
    pending: Vec<Firing>,
    protocol: Protocol,
    trace_physics: bool,
    next_serial: u64,
    outcome_source: Option<JunctionId>,
    local_outcome_sources: Vec<(JunctionId, JunctionId)>,
    output_wave_open: bool,
    learners: Vec<LearnerState>,
    causal_closures: Vec<CausalClosureState>,
    next_learner_id: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ArenaSnapshot {
    junction_capacity: u32,
    link_capacity: u32,
    junction_identity_count: u64,
    link_identity_count: u64,
    junctions: Vec<JunctionState>,
    links: Vec<LinkState>,
    activation: Vec<i64>,
    strength: Vec<i64>,
    life: Vec<u64>,
    decay_remainder: Vec<u64>,
    aging_links: Vec<LinkId>,
    active_junctions: Vec<JunctionId>,
}

impl Body {
    pub(crate) fn snapshot(&self) -> Result<BodySnapshot, CheckpointError> {
        Ok(BodySnapshot {
            arena: ArenaSnapshot {
                junction_capacity: self.arena.junction_capacity,
                link_capacity: self.arena.link_capacity,
                junction_identity_count: u64::try_from(self.arena.junction_slots.len())
                    .map_err(|_| CheckpointError::InvalidCheckpoint)?,
                link_identity_count: u64::try_from(self.arena.link_slots.len())
                    .map_err(|_| CheckpointError::InvalidCheckpoint)?,
                junctions: self.arena.junctions.clone(),
                links: self.arena.links.clone(),
                activation: self.arena.activation.clone(),
                strength: self.arena.strength.clone(),
                life: self.arena.life.clone(),
                decay_remainder: self.arena.decay_remainder.clone(),
                aging_links: self.arena.aging_links.iter().copied().collect(),
                active_junctions: {
                    let mut active = self
                        .arena
                        .active_junctions
                        .iter()
                        .copied()
                        .collect::<Vec<_>>();
                    active.sort_unstable();
                    active
                },
            },
            clock: self.clock(),
            pending: self.pending.canonical(),
            protocol: self.protocol,
            trace_physics: self.trace_physics,
            next_serial: self.next_serial,
            outcome_source: self.outcome_source,
            local_outcome_sources: self.local_outcome_sources.clone(),
            output_wave_open: self.output_wave_open,
            learners: self.learners.clone(),
            causal_closures: self.causal_closures.clone(),
            next_learner_id: self.next_learner_id,
        })
    }

    pub(crate) fn from_snapshot(snapshot: BodySnapshot) -> Result<Self, CheckpointError> {
        let mut body = Self::from_arena(snapshot.arena.open()?);
        body.tick = snapshot.clock.tick;
        body.protocol = snapshot.protocol;
        body.trace_physics = snapshot.trace_physics;
        body.pressure_tick = pressure_epoch(snapshot.clock.tick);
        body.outcome_source = snapshot.outcome_source;
        if let Some(source) = body.outcome_source {
            if body.arena.junction_slot(source).is_none() {
                return Err(CheckpointError::MissingJunction(source));
            }
        }
        let mut previous_output = None;
        for (output, source) in &snapshot.local_outcome_sources {
            if body.arena.junction_slot(*output).is_none() {
                return Err(CheckpointError::MissingJunction(*output));
            }
            if body.arena.junction_slot(*source).is_none() {
                return Err(CheckpointError::MissingJunction(*source));
            }
            if !body.arena.is_output_junction(*output)
                || previous_output.is_some_and(|previous| previous >= *output)
            {
                return Err(CheckpointError::InvalidCheckpoint);
            }
            previous_output = Some(*output);
        }
        body.local_outcome_sources = snapshot.local_outcome_sources;
        body.output_wave_open = snapshot.output_wave_open;
        let mut learner_ids = HashSet::new();
        let mut previous_id = LearnerId(0);
        for learner in &snapshot.learners {
            if learner.id.0 == 0
                || !learner_ids.insert(learner.id)
                || learner.id <= previous_id
                || learner
                    .parent
                    .is_some_and(|parent| !learner_ids.contains(&parent))
                || learner.junctions.is_empty()
                || learner.links.is_empty()
                || !strictly_sorted(&learner.junctions)
                || !strictly_sorted(&learner.links)
                || learner
                    .junctions
                    .iter()
                    .any(|junction| body.arena.junction_slot(*junction).is_none())
                || learner
                    .links
                    .iter()
                    .any(|link| body.arena.link_slot(*link).is_none())
                || learner.junctions.binary_search(&learner.surface).is_err()
                || learner.junctions.binary_search(&learner.output).is_err()
            {
                return Err(CheckpointError::InvalidCheckpoint);
            }
            previous_id = learner.id;
        }
        let mut closure_keys = HashSet::new();
        for closure in &snapshot.causal_closures {
            if !closure_keys.insert((closure.surface, closure.output))
                || closure.evidence == 0
                || body.arena.junction_slot(closure.surface).is_none()
                || body.arena.junction_slot(closure.output).is_none()
                || closure
                    .parent
                    .is_some_and(|parent| !learner_ids.contains(&parent))
                || closure
                    .constructed
                    .is_some_and(|learner| !learner_ids.contains(&learner))
            {
                return Err(CheckpointError::InvalidCheckpoint);
            }
        }
        if snapshot.next_learner_id == 0
            || learner_ids
                .iter()
                .any(|learner| learner.0 >= snapshot.next_learner_id)
        {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        body.learners = snapshot.learners;
        body.causal_closures = snapshot.causal_closures;
        body.next_learner_id = snapshot.next_learner_id;
        for firing in &snapshot.pending {
            if firing.arrival_tick < snapshot.clock.tick {
                return Err(CheckpointError::InvalidCheckpoint);
            }
            let slot = body
                .arena
                .junction_slot(firing.target)
                .ok_or(CheckpointError::MissingJunction(firing.target))?;
            if body.arena.junctions[slot.0].generation != firing.target_generation {
                return Err(CheckpointError::InvalidCheckpoint);
            }
            if let Some((link, generation)) = firing.link {
                let slot = body
                    .arena
                    .link_slot(link)
                    .ok_or(CheckpointError::MissingLink(link))?;
                if body.arena.links[slot.0].generation != generation {
                    return Err(CheckpointError::StaleLinkReference(link));
                }
            }
        }
        body.pending = Schedule::from_canonical(snapshot.clock.tick, snapshot.pending);
        body.next_serial = snapshot.next_serial;
        Ok(body)
    }
}

fn strictly_sorted<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

impl ArenaSnapshot {
    fn open(self) -> Result<Arena, CheckpointError> {
        let aging_links = self.aging_links.iter().copied().collect::<BTreeSet<_>>();
        if aging_links.len() != self.aging_links.len() {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        let active_junctions = self
            .active_junctions
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        if active_junctions.len() != self.active_junctions.len() {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        let junction_identity_count = usize::try_from(self.junction_identity_count)
            .map_err(|_| CheckpointError::InvalidCheckpoint)?;
        let link_identity_count = usize::try_from(self.link_identity_count)
            .map_err(|_| CheckpointError::InvalidCheckpoint)?;
        if self.junctions.len() > self.junction_capacity as usize
            || self.links.len() > self.link_capacity as usize
            || self.activation.len() != junction_identity_count
            || self.strength.len() != link_identity_count
            || self.life.len() != link_identity_count
            || self.decay_remainder.len() != link_identity_count
        {
            return Err(CheckpointError::InvalidCheckpoint);
        }

        let mut arena = Arena::new(self.junction_capacity, self.link_capacity);
        arena.junction_slots = vec![None; junction_identity_count];
        let mut physical_ids = HashSet::new();
        let mut junction_ids = HashSet::new();
        for (index, junction) in self.junctions.iter().enumerate() {
            let identity =
                usize::try_from(junction.id.0).map_err(|_| CheckpointError::InvalidCheckpoint)?;
            if identity >= junction_identity_count
                || !junction_ids.insert(junction.id)
                || !physical_ids.insert(junction.physical_id)
            {
                return Err(CheckpointError::InvalidCheckpoint);
            }
            if junction.live {
                if junction.resistance == 0 || arena.junction_slots[identity].is_some() {
                    return Err(CheckpointError::InvalidCheckpoint);
                }
                arena.junction_slots[identity] = Some(JunctionSlot(index));
            }
        }

        arena.link_slots = vec![None; link_identity_count];
        for (index, link) in self.links.iter().enumerate() {
            let identity =
                usize::try_from(link.id.0).map_err(|_| CheckpointError::InvalidCheckpoint)?;
            if identity >= link_identity_count
                || arena.link_slots[identity].is_some()
                || link.delay < 0
                || link.live != (link.resistance > 0)
            {
                return Err(CheckpointError::InvalidCheckpoint);
            }
            arena.link_slots[identity] = Some(LinkSlot(index));
            if link.live {
                let source_identity =
                    usize::try_from(link.from.0).map_err(|_| CheckpointError::InvalidCheckpoint)?;
                let target_identity =
                    usize::try_from(link.to.0).map_err(|_| CheckpointError::InvalidCheckpoint)?;
                let source = arena
                    .junction_slots
                    .get(source_identity)
                    .and_then(|slot| *slot)
                    .ok_or(CheckpointError::MissingJunction(link.from))?;
                let target = arena
                    .junction_slots
                    .get(target_identity)
                    .and_then(|slot| *slot)
                    .ok_or(CheckpointError::MissingJunction(link.to))?;
                if self.junctions[source.0].generation != link.source_generation
                    || self.junctions[target.0].generation != link.target_generation
                {
                    return Err(CheckpointError::StaleLinkReference(link.id));
                }
            }
        }

        arena.junctions = self.junctions;
        arena.links = self.links;
        arena.activation = self.activation;
        arena.strength = self.strength;
        arena.life = self.life;
        arena.decay_remainder = self.decay_remainder;
        arena.rebuild_indexes();
        if aging_links
            .iter()
            .any(|id| arena.link_by_id(*id).is_none_or(|link| !link.live))
        {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        arena.aging_links = aging_links;
        if active_junctions.iter().any(|id| {
            arena
                .junction_by_id(*id)
                .is_none_or(|junction| !junction.live)
        }) {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        arena.active_junctions = active_junctions;
        Ok(arena)
    }
}
