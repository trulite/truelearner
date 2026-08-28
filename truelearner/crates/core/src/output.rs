use crate::prelude::*;

impl Arena {
    pub(crate) fn find_output_junctions(&mut self) {
        self.output_junctions = vec![false; self.junction_slots.len()];
        self.outputs_by_position.clear();
        for link in &self.links {
            let Some(from) = self.junction_slot(link.from) else {
                continue;
            };
            let Some(to) = self.junction_slot(link.to) else {
                continue;
            };
            if self.junctions[from.0].region != self.junctions[to.0].region {
                self.output_junctions[link.from.0 as usize] = true;
            }
        }
        for junction in &self.junctions {
            if junction.live && self.is_output_junction(junction.id) {
                self.outputs_by_position
                    .entry(junction.position)
                    .or_default()
                    .push(junction.id);
            }
        }
        for outputs in self.outputs_by_position.values_mut() {
            outputs.sort_unstable();
            outputs.dedup();
        }
    }

    pub(crate) fn is_output_junction(&self, junction: JunctionId) -> bool {
        self.output_junctions
            .get(junction.0 as usize)
            .copied()
            .unwrap_or(false)
    }

    pub(crate) fn local_outputs(&self, position: i32) -> Vec<JunctionId> {
        let lower = position.saturating_sub(LOCAL_VARIATION_RADIUS);
        let upper = position.saturating_add(LOCAL_VARIATION_RADIUS);
        self.outputs_by_position
            .range(lower..=upper)
            .flat_map(|(_, outputs)| outputs.iter().copied())
            .filter(|output| {
                self.junction_by_id(*output)
                    .is_some_and(|junction| junction.live && junction.position != position)
            })
            .collect()
    }
}

impl Body {
    /// Fire output along every live link leaving this junction.
    pub(crate) fn fire_output_from(&mut self, fired: Fired, moment: &Moment, run: &mut RunState) {
        let source = fired.junction;
        run.cost.allocations = run.cost.allocations.saturating_add(1);
        run.cost.adjacency_accesses = run.cost.adjacency_accesses.saturating_add(
            u64::try_from(self.arena.outgoing_index[source.0 as usize].len()).unwrap_or(u64::MAX),
        );
        let outgoing = self.arena.outgoing_index[source.0 as usize]
            .iter()
            .filter_map(|id| {
                let slot = self.arena.link_slot(*id)?;
                run.cost.scans = run.cost.scans.saturating_add(1);
                run.cost.touch::<LinkState>(1);
                Some((*id, self.arena.link_snapshot(slot.0)))
            })
            .collect::<Vec<_>>();

        for (link_id, link) in outgoing {
            run.cost.touch::<LinkState>(1);
            if !link.live
                || link.from != source
                || link.source_generation != fired.state.generation
                || link.trigger != TransmissionTrigger::SourceFires
            {
                continue;
            }
            let Some(from_slot) = self.arena.junction_slot(link.from) else {
                continue;
            };
            let Some(to_slot) = self.arena.junction_slot(link.to) else {
                continue;
            };
            let from = self.arena.junction_snapshot(from_slot.0);
            let to = self.arena.junction_snapshot(to_slot.0);
            run.cost.touch::<JunctionState>(2);
            if from.id != link.from
                || !from.live
                || from.generation != link.source_generation
                || to.id != link.to
                || !to.live
                || to.generation != link.target_generation
            {
                continue;
            }

            run.work.total = run.work.total.saturating_add(2);
            if from.region != to.region {
                let output = Output {
                    tick: self.tick,
                    from_physical: from.physical_id,
                    to_physical: to.physical_id,
                    from_region: from.region,
                    to_region: to.region,
                    impulse: link.coupling,
                };
                if self.trace_physics {
                    run.trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase: moment.phase,
                        event: PhysicalEvent::Output(output),
                    });
                }
                run.outputs.push(output);
            }
            if !self.arena.completes_path(link_id) {
                self.reuse(link_id);
            }

            run.cost.touch::<LinkState>(1);
            run.cost.arena_lookups = run.cost.arena_lookups.saturating_add(2);
            let next_tick = self.tick.saturating_add(link.delay);
            let next_wave = if link.delay == 0 && link.phase == moment.phase {
                moment.causal.saturating_add(1)
            } else {
                0
            };
            let origin_physical = if self.protocol.is_sensorimotor() {
                fired.causal_origin
            } else {
                fired.state.physical_id
            };
            self.pending.push(
                Firing {
                    arrival_tick: next_tick,
                    phase: link.phase,
                    causal_wave: next_wave,
                    origin_physical,
                    causal_lineage: self.protocol.preserves_causal_lineage().then(|| {
                        fired
                            .causal_lineage
                            .clone()
                            .unwrap_or_else(|| CausalLineage::singleton(origin_physical, self.tick))
                    }),
                    physical_incidence: PhysicalIncidence::Sample,
                    target_physical: to.physical_id,
                    target: link.to,
                    target_generation: to.generation,
                    impulse: link.coupling,
                    strength: self.arena.strength[link_id.0 as usize],
                    serial: self.next_serial,
                    link: Some((link_id, link.generation)),
                },
                &mut run.cost,
            );
            self.next_serial = self.next_serial.wrapping_add(1);
        }
    }

    /// Clear held output after the complete output wave has finished.
    pub(crate) fn finish_output_wave(
        &mut self,
        work: &mut Work,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) {
        if !self.output_wave_open {
            return;
        }
        self.output_wave_open = false;
        for index in 0..self.arena.output_junctions.len() {
            if !self.arena.output_junctions[index] {
                continue;
            }
            let activation = self.arena.activation.get(index).copied().unwrap_or(0);
            if activation == 0 {
                continue;
            }
            let target = JunctionId(u64::try_from(index).unwrap_or(u64::MAX));
            let Some(slot) = self.arena.junction_slot(target) else {
                continue;
            };
            self.arena.activation[index] = 0;
            let tick = self.tick;
            self.arena.edit_junction(slot.0, |junction| {
                junction.state = 0;
                junction.last_update_tick = tick;
            });
            self.arena.active_junctions.remove(&target);
            work.total = work.total.saturating_add(1);
            if self.trace_physics {
                physical_trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: 0,
                    event: PhysicalEvent::OutputWaveFinished { target, activation },
                });
            }
        }
    }
}
