use crate::prelude::*;

const PARTICIPATION_RELAX_NUMERATOR: u64 = 15;
const PARTICIPATION_RELAX_DENOMINATOR: u64 = 16;

impl Arena {
    pub(crate) fn held_links(&self, outcome: Option<JunctionId>) -> HashSet<LinkId> {
        let paths = self.paths();
        let junctions = paths
            .iter()
            .map(|path| path.junction)
            .collect::<HashSet<_>>();
        let returns = self
            .links
            .iter()
            .filter(|link| {
                link.live
                    && link.mode == TransmissionMode::Modulatory
                    && Some(link.from) == outcome
                    && junctions.contains(&link.to)
            })
            .map(|link| link.id)
            .collect::<Vec<_>>();
        let open = returns
            .iter()
            .filter_map(|id| self.link_by_id(*id).map(|link| link.to))
            .collect::<HashSet<_>>();
        paths
            .into_iter()
            .filter(|path| open.contains(&path.junction))
            .flat_map(|path| [path.first, path.second])
            .chain(returns)
            .collect()
    }
}

impl Body {
    pub(crate) fn decay_links_to(
        &mut self,
        tick: i64,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        physical_trace: Option<&mut Vec<PhysicalTransition>>,
        phase: i32,
    ) {
        let mut physical_trace = physical_trace;
        let elapsed = tick.saturating_sub(self.tick);
        let elapsed_u64 = u64::try_from(elapsed).unwrap_or(u64::MAX);
        let held = self.arena.held_links(self.outcome_source);
        for index in 0..self.arena.links.len() {
            execution_cost.scans = execution_cost.scans.saturating_add(1);
            let snapshot = self.arena.link_snapshot(index);
            if !snapshot.live {
                continue;
            }
            if held.contains(&snapshot.id) {
                self.arena.edit_link(index, |link| {
                    link.participation_level =
                        relax_participation(link.participation_level, elapsed);
                });
                work.total = work.total.saturating_add(elapsed_u64);
                execution_cost.touch::<LinkState>(1);
                continue;
            }
            let index = snapshot.id.0 as usize;
            let before = self.arena.life[index];
            let decay_numerator = elapsed_u64
                .saturating_mul(UNIT_U64)
                .saturating_add(self.arena.decay_remainder[index]);
            let loss = decay_numerator / 10;
            self.arena.decay_remainder[index] = decay_numerator % 10;
            let active_ticks =
                elapsed_u64.min(before.saturating_mul(10).saturating_add(UNIT_U64 - 1) / UNIT_U64);
            let after = before.saturating_sub(loss);
            self.arena.life[index] = after;
            let deallocated = before > 0 && after == 0;
            self.arena.edit_link(index, |link| {
                link.participation_level = relax_participation(link.participation_level, elapsed);
                if deallocated {
                    link.retire();
                } else {
                    let observer = after.saturating_add(UNIT_U64 - 1) / UNIT_U64;
                    link.resistance = u32::try_from(observer).unwrap_or(u32::MAX);
                }
            });
            work.total = work.total.saturating_add(active_ticks);
            execution_cost.touch::<LinkState>(1);
            if deallocated && snapshot.delay == 0 {
                self.arena.zero_delay_live_links =
                    self.arena.zero_delay_live_links.saturating_sub(1);
            }
            if deallocated {
                work.total = work.total.saturating_add(1);
                work.physical_deallocations = work.physical_deallocations.saturating_add(1);
                if let Some(trace) = physical_trace.as_deref_mut() {
                    trace.push(PhysicalTransition {
                        tick,
                        phase,
                        event: PhysicalEvent::Deallocate { link: snapshot.id },
                    });
                }
            }
        }
    }

    /// Hold new input at a junction.
    pub(crate) fn hold_input(
        &mut self,
        junction: JunctionId,
        strength: i64,
    ) -> (JunctionState, i64) {
        self.decay_held_input(junction, self.tick);
        let slot = self.arena.junction_slot(junction).unwrap();
        let index = junction.0 as usize;
        self.arena.activation[index] = self.arena.activation[index].saturating_add(strength);
        let held = self.arena.activation[index];
        self.arena
            .edit_junction(slot.0, |target| target.state = observed(held));
        if held != 0 {
            self.arena.active_junctions.insert(junction);
        }
        (self.arena.junction_snapshot(slot.0), held)
    }

    /// Let held input weaken as physical time advances.
    pub(crate) fn elapse_activation_to(&mut self, tick: i64, cost: &mut ExecutionCost) {
        cost.observe_frontier(self.arena.active_junctions.len());
        cost.allocations = cost.allocations.saturating_add(1);
        let junctions = self
            .arena
            .active_junctions
            .iter()
            .copied()
            .collect::<Vec<_>>();
        for junction in junctions {
            if self.arena.junction_slot(junction).is_none() {
                continue;
            }
            cost.scans = cost.scans.saturating_add(1);
            cost.touch::<JunctionState>(1);
            self.decay_held_input(junction, tick);
        }
    }

    fn decay_held_input(&mut self, junction: JunctionId, tick: i64) {
        let slot = self
            .arena
            .junction_slot(junction)
            .expect("junction must resolve");
        let elapsed = tick.saturating_sub(self.arena.junction_snapshot(slot.0).last_update_tick);
        if elapsed <= 0 {
            return;
        }
        let index = junction.0 as usize;
        if self.output_wave_open
            && self
                .arena
                .output_junctions
                .get(index)
                .copied()
                .unwrap_or(false)
            && self.arena.activation.get(index).copied().unwrap_or(0) != 0
        {
            self.arena
                .edit_junction(slot.0, |target| target.last_update_tick = tick);
            return;
        }

        let loss = elapsed.saturating_mul(UNIT);
        let held = self.arena.activation[index];
        self.arena.activation[index] = if held > 0 {
            held.saturating_sub(loss).max(0)
        } else {
            held.saturating_add(loss).min(0)
        };
        let held = self.arena.activation[index];
        self.arena.edit_junction(slot.0, |target| {
            target.state = observed(held);
            target.last_update_tick = tick;
        });
        if held == 0 {
            self.arena.active_junctions.remove(&junction);
        } else {
            self.arena.active_junctions.insert(junction);
        }
    }
}

fn observed(held: i64) -> i32 {
    let value = held / UNIT;
    i32::try_from(value).unwrap_or_else(|_| {
        if value.is_negative() {
            i32::MIN
        } else {
            i32::MAX
        }
    })
}

fn relax_participation(mut level: u64, elapsed: i64) -> u64 {
    for _ in 0..elapsed.max(0) {
        level =
            level.saturating_mul(PARTICIPATION_RELAX_NUMERATOR) / PARTICIPATION_RELAX_DENOMINATOR;
    }
    level
}
