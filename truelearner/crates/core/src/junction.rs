use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct JunctionSlot(pub(crate) usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Junction {
    pub physical_id: u64,
    pub position: i32,
    pub region: i16,
    pub threshold: i32,
    pub resistance: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct JunctionState {
    pub(crate) id: JunctionId,
    pub(crate) physical_id: u64,
    pub(crate) position: i32,
    pub(crate) region: i16,
    pub(crate) threshold: i32,
    pub(crate) state: i32,
    pub(crate) last_update_tick: i64,
    pub(crate) refractory_until: i64,
    pub(crate) generation: Generation,
    pub(crate) resistance: u32,
    pub(crate) live: bool,
    pub(crate) decay_load: u64,
}

impl Arena {
    pub(crate) fn add_junction(&mut self, spec: Junction, tick: i64) -> JunctionId {
        assert!(spec.threshold > 0, "threshold must be physically positive");
        assert!(
            self.junctions
                .iter()
                .all(|junction| junction.physical_id != spec.physical_id),
            "physical junction identity must be unique"
        );
        let reusable = self.junctions.iter().position(|junction| !junction.live);
        assert!(
            reusable.is_some() || self.junctions.len() < self.junction_capacity as usize,
            "arena has no free junction slot"
        );
        let id = JunctionId(self.junction_slots.len() as u64);
        let (slot, generation) = reusable.map_or_else(
            || (JunctionSlot(self.junctions.len()), Generation(1)),
            |index| (JunctionSlot(index), self.junctions[index].generation),
        );
        let junction = JunctionState {
            id,
            physical_id: spec.physical_id,
            position: spec.position,
            region: spec.region,
            threshold: spec.threshold,
            state: 0,
            last_update_tick: tick,
            refractory_until: tick,
            generation,
            resistance: spec.resistance,
            live: spec.resistance > 0,
            decay_load: 0,
        };
        if slot.0 < self.junctions.len() {
            self.junctions[slot.0] = junction;
        } else {
            self.junctions.push(junction);
        }
        self.junction_slots
            .push((spec.resistance > 0).then_some(slot));
        self.outgoing_index.push(Vec::new());
        self.incoming_index.push(Vec::new());
        self.output_junctions.push(false);
        self.activation.push(0);
        id
    }
}

impl Body {
    pub(crate) fn fire(
        &mut self,
        incidence: Incidence,
        moment: &Moment,
        run: &mut RunState,
    ) -> Option<Fired> {
        if incidence.inputs.is_empty() {
            return None;
        }
        let arrivals = incidence.arrivals();
        let impulse = incidence.impulse();
        let strength = incidence.strength();
        let external = incidence.external();
        let count = u64::try_from(incidence.inputs.len()).unwrap_or(u64::MAX);
        run.work.total = run.work.total.saturating_add(count.saturating_mul(5));
        run.work.drive_deliveries = run.work.drive_deliveries.saturating_add(count);
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::DriveIncidence {
                    target: incidence.junction,
                    arrivals,
                    impulse,
                    causal_wave: moment.causal,
                },
            });
        }

        let causal_origin = if self.protocol.is_sensorimotor() {
            let mut origins = incidence
                .inputs
                .iter()
                .map(|firing| firing.origin_physical)
                .collect::<Vec<_>>();
            origins.sort_unstable();
            origins.dedup();
            if origins.len() == 1 {
                origins[0]
            } else {
                self.arena
                    .junction_snapshot(self.arena.junction_slot(incidence.junction).unwrap().0)
                    .physical_id
            }
        } else {
            0
        };
        let (state, held) = self.hold_input(incidence.junction, strength);
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::MaterialDriveIncidence {
                    target: incidence.junction,
                    impulse: strength,
                    activation_after: held,
                    causal_wave: moment.causal,
                },
            });
        }
        run.cost.touch::<JunctionState>(1);
        if !state.can_fire(self.tick, held) {
            return None;
        }
        self.clear_after_fire(incidence.junction);
        Some(Fired {
            junction: incidence.junction,
            state,
            external,
            causal_origin,
        })
    }

    fn clear_after_fire(&mut self, junction: JunctionId) {
        let slot = self.arena.junction_slot(junction).unwrap();
        let refractory_until = self.tick.saturating_add(1);
        self.arena.edit_junction(slot.0, |target| {
            target.state = 0;
            target.refractory_until = refractory_until;
        });
        self.arena.activation[junction.0 as usize] = 0;
        self.arena.active_junctions.remove(&junction);
    }
}

impl Incidence {
    fn arrivals(&self) -> u32 {
        u32::try_from(self.inputs.len()).unwrap_or(u32::MAX)
    }

    fn impulse(&self) -> i32 {
        self.inputs
            .iter()
            .fold(0, |sum, firing| sum.saturating_add(firing.impulse))
    }

    fn strength(&self) -> i64 {
        self.inputs
            .iter()
            .fold(0, |sum, firing| sum.saturating_add(firing.strength))
    }

    fn external(&self) -> bool {
        self.inputs.iter().any(|firing| firing.link.is_none())
    }
}

impl JunctionState {
    fn can_fire(&self, tick: i64, held: i64) -> bool {
        tick >= self.refractory_until && held >= i64::from(self.threshold).saturating_mul(UNIT)
    }

    fn retire(&mut self) -> (Generation, Generation) {
        let before = self.generation;
        let after = Generation(before.0.wrapping_add(1));
        self.live = false;
        self.generation = after;
        self.state = 0;
        self.refractory_until = 0;
        self.decay_load = 0;
        (before, after)
    }
}

impl Body {
    pub(crate) fn retire_unlinked_junctions(
        &mut self,
        tick: i64,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        physical_trace: Option<&mut Vec<PhysicalTransition>>,
        phase: i32,
    ) {
        let mut physical_trace = physical_trace;
        let required = self
            .arena
            .links
            .iter()
            .filter(|link| link.live)
            .flat_map(|link| [link.from, link.to])
            .collect::<HashSet<_>>();
        execution_cost.scans = execution_cost
            .scans
            .saturating_add(u64::try_from(self.arena.links.len()).unwrap_or(u64::MAX));
        execution_cost.touch::<JunctionState>(self.arena.junctions.len());
        execution_cost.touch::<LinkState>(self.arena.links.len());

        for index in 0..self.arena.junctions.len() {
            let junction = self.arena.junction_snapshot(index);
            if !junction.live || required.contains(&junction.id) {
                continue;
            }
            let (before_generation, after_generation) =
                self.arena.edit_junction(index, JunctionState::retire);
            if let Some(mapping) = self.arena.junction_slots.get_mut(junction.id.0 as usize) {
                *mapping = None;
            }
            self.arena.active_junctions.remove(&junction.id);
            work.total = work.total.saturating_add(1);
            work.junction_deallocations = work.junction_deallocations.saturating_add(1);
            if let Some(trace) = physical_trace.as_deref_mut() {
                trace.push(PhysicalTransition {
                    tick,
                    phase,
                    event: PhysicalEvent::JunctionDeallocate {
                        junction: junction.id,
                        before_generation: before_generation.0,
                        after_generation: after_generation.0,
                    },
                });
            }
        }
    }
}
