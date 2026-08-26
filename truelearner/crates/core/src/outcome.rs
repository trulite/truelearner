use crate::prelude::*;

impl Arena {
    pub(crate) fn return_links(&self, outcome: Option<JunctionId>) -> Vec<LinkId> {
        let junctions = self
            .paths()
            .into_iter()
            .map(|path| path.junction)
            .collect::<HashSet<_>>();
        self.links
            .iter()
            .filter(|link| {
                link.live
                    && link.mode == TransmissionMode::Modulatory
                    && Some(link.from) == outcome
                    && junctions.contains(&link.to)
            })
            .map(|link| link.id)
            .collect()
    }
}

impl Body {
    /// Remove the temporary path after its outcome has physically returned.
    pub(crate) fn return_outcome(&mut self, id: LinkId) {
        let Some(slot) = self.arena.link_slot(id) else {
            return;
        };
        let link = self.arena.link_snapshot(slot.0);
        if !self.arena.return_links(self.outcome_source).contains(&id) {
            return;
        }
        if link.delay == 0 {
            self.arena.zero_delay_live_links = self.arena.zero_delay_live_links.saturating_sub(1);
        }
        let index = id.0 as usize;
        self.arena.life[index] = 0;
        self.arena.decay_remainder[index] = 0;
        self.arena.edit_link(slot.0, LinkState::retire);
    }

    pub(crate) fn outcomes_return(&mut self, moment: &Moment, run: &mut RunState) {
        for incidence in &moment.incidences {
            if incidence.outcomes.is_empty() {
                continue;
            }
            let arrivals = u32::try_from(incidence.outcomes.len()).unwrap_or(u32::MAX);
            let impulse = incidence
                .outcomes
                .iter()
                .fold(0_i32, |sum, firing| sum.saturating_add(firing.impulse));
            let count = u64::try_from(incidence.outcomes.len()).unwrap_or(u64::MAX);
            run.work.total = run.work.total.saturating_add(count.saturating_mul(2));
            run.work.modulatory_deliveries = run.work.modulatory_deliveries.saturating_add(count);
            if self.trace_physics {
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::ModulatoryIncidence {
                        target: incidence.junction,
                        arrivals,
                        impulse,
                        causal_wave: moment.causal,
                    },
                });
            }
        }
    }

    pub(crate) fn strengthen_outcomes(&mut self, moment: &Moment, run: &mut RunState) {
        for incidence in &moment.incidences {
            for outcome in &incidence.outcomes {
                self.apply_outcome(incidence.junction, moment, run);
                if let Some((link, _)) = outcome.link {
                    self.return_outcome(link);
                }
            }
        }
    }

    pub(crate) fn apply_outcome(
        &mut self,
        junction: JunctionId,
        moment: &Moment,
        run: &mut RunState,
    ) {
        let return_updates_before = run.work.local_return_updates;
        let candidates = {
            run.cost.allocations = run.cost.allocations.saturating_add(1);
            run.cost.scans = run
                .cost
                .scans
                .saturating_add(u64::try_from(self.arena.links.len()).unwrap_or(u64::MAX));
            run.cost.touch::<LinkState>(self.arena.links.len());
            self.arena
                .links
                .iter()
                .map(|link| link.id)
                .collect::<Vec<_>>()
        };
        let qualified_local = candidates.iter().any(|id| {
            let slot = self
                .arena
                .link_slot(*id)
                .expect("indexed LINK must resolve");
            let link = self.arena.link_snapshot(slot.0);
            link.live
                && link.from == junction
                && link.mode == TransmissionMode::Drive
                && link.participation_level > 0
        });
        for id in candidates {
            run.cost.scans = run.cost.scans.saturating_add(1);
            let slot = self.arena.link_slot(id).expect("indexed LINK must resolve");
            let link = self.arena.link_snapshot(slot.0);
            let local_participating_structure = link.live
                && link.mode == TransmissionMode::Drive
                && (link.from == junction || link.to == junction);
            if local_participating_structure && link.participation_level > 0 {
                let index = id.0 as usize;
                let participation = link.participation_level;
                let coupling_before = self.arena.strength[index];
                let resistance_before = self.arena.life[index];
                let sign = coupling_before.signum();
                self.arena.strength[index] = coupling_before.saturating_add(
                    sign.saturating_mul(i64::try_from(participation).unwrap_or(i64::MAX)),
                );
                self.arena.life[index] = resistance_before
                    .saturating_add(participation.saturating_mul(u64::from(LOCAL_RETURN_STRENGTH)));
                let coupling_observer = self.arena.strength[index] / UNIT;
                let resistance_observer =
                    self.arena.life[index].saturating_add(UNIT_U64.saturating_sub(1)) / UNIT_U64;
                self.arena.edit_link(slot.0, |live_link| {
                    live_link.coupling = i32::try_from(coupling_observer).unwrap_or_else(|_| {
                        if coupling_observer.is_negative() {
                            i32::MIN
                        } else {
                            i32::MAX
                        }
                    });
                    live_link.resistance = u32::try_from(resistance_observer).unwrap_or(u32::MAX);
                    live_link.decay_load = 0;
                });
                run.work.total = run.work.total.saturating_add(4);
                run.work.local_return_updates = run.work.local_return_updates.saturating_add(1);
            }
            run.cost.touch::<LinkState>(1);
        }
        if run.work.local_return_updates > return_updates_before {
            self.strengthen(junction, &mut run.work, moment.phase, &mut run.trace);
        }
        if qualified_local {
            self.propagate_qualified_local(junction, moment, run);
        }
    }

    fn propagate_qualified_local(
        &mut self,
        junction: JunctionId,
        moment: &Moment,
        run: &mut RunState,
    ) {
        let outgoing = self.arena.outgoing_index[junction.0 as usize].clone();
        for id in outgoing {
            let slot = self.arena.link_slot(id).expect("indexed LINK must resolve");
            let link = self.arena.link_snapshot(slot.0);
            run.cost.scans = run.cost.scans.saturating_add(1);
            run.cost.touch::<LinkState>(1);
            if !link.live
                || link.from != junction
                || link.trigger != TransmissionTrigger::QualifiedLocalParticipation
            {
                continue;
            }
            assert_eq!(link.mode, TransmissionMode::Modulatory);
            let Some(source_slot) = self.arena.junction_slot(link.from) else {
                continue;
            };
            let Some(target_slot) = self.arena.junction_slot(link.to) else {
                continue;
            };
            let source = self.arena.junction_snapshot(source_slot.0);
            let target = self.arena.junction_snapshot(target_slot.0);
            if source.id != link.from
                || !source.live
                || source.generation != link.source_generation
                || target.id != link.to
                || !target.live
                || target.generation != link.target_generation
            {
                continue;
            }
            let arrival_tick = self.tick.saturating_add(link.delay);
            let arrival_phase = link.phase;
            let generation = link.generation;
            let coupling = link.coupling;
            let target_generation = target.generation;
            let target_id = link.to;
            let origin_physical = source.physical_id;
            self.arena.edit_link(slot.0, |live_link| {
                live_link.participation_level = live_link
                    .participation_level
                    .saturating_add(PARTICIPATION_IMPULSE);
            });
            run.work.total = run.work.total.saturating_add(1);
            run.work.qualified_local_traversals =
                run.work.qualified_local_traversals.saturating_add(1);
            if self.trace_physics {
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::QualifiedLocalTraversal { link: id },
                });
            }
            self.pending.push(
                Firing {
                    arrival_tick,
                    phase: arrival_phase,
                    causal_wave: if link.delay == 0 && arrival_phase == moment.phase {
                        moment.causal.saturating_add(1)
                    } else {
                        0
                    },
                    origin_physical,
                    target_physical: target.physical_id,
                    target: target_id,
                    target_generation,
                    impulse: coupling,
                    strength: self.arena.strength[id.0 as usize],
                    serial: self.next_serial,
                    link: Some((id, generation)),
                },
                &mut run.cost,
            );
            self.next_serial = self.next_serial.wrapping_add(1);
        }
    }
}
