use crate::prelude::*;

impl Arena {
    pub(crate) fn return_links(&self, outcomes: &[JunctionId]) -> Vec<LinkId> {
        outcomes
            .iter()
            .flat_map(|outcome| {
                self.outgoing_index
                    .get(outcome.0 as usize)
                    .into_iter()
                    .flatten()
                    .copied()
            })
            .filter(|id| {
                self.link_by_id(*id).is_some_and(|link| {
                    link.live
                        && link.mode == TransmissionMode::Modulatory
                        && self.is_path_junction(link.to)
                })
            })
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
        if !self
            .arena
            .return_links(&self.outcome_sources())
            .contains(&id)
        {
            return;
        }
        if link.delay == 0 {
            self.arena.zero_delay_live_links = self.arena.zero_delay_live_links.saturating_sub(1);
        }
        let index = id.0 as usize;
        self.arena.life[index] = 0;
        self.arena.decay_remainder[index] = 0;
        self.arena.edit_link(slot.0, LinkState::retire);
        self.arena.aging_links.remove(&id);
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

    pub(crate) fn strengthen_candidate_outcomes(&mut self, moment: &Moment, run: &mut RunState) {
        for incidence in &moment.incidences {
            for outcome in &incidence.outcomes {
                let Some((link, _)) = outcome.link else {
                    continue;
                };
                if self.accept_return_origin(link, outcome.origin_physical) {
                    self.apply_outcome(incidence.junction, moment, run);
                    if self.protocol == Protocol::SensorimotorSynthesis {
                        self.consolidate_reverse_path(link, outcome.origin_physical, moment, run);
                    }
                }
            }
        }
    }

    fn consolidate_reverse_path(
        &mut self,
        return_link: LinkId,
        origin: u64,
        moment: &Moment,
        run: &mut RunState,
    ) {
        let Some(return_state) = self.arena.link_by_id(return_link).cloned() else {
            return;
        };
        let Some(source) = self
            .arena
            .junctions
            .iter()
            .find(|junction| junction.live && junction.physical_id == origin)
            .map(|junction| junction.id)
        else {
            return;
        };
        if source == return_state.from {
            return;
        }
        let action = self
            .arena
            .paths_through(return_state.to)
            .into_iter()
            .filter_map(|path| {
                let second = self.arena.link_by_id(path.second)?;
                (second.participation_level > 0).then_some((
                    path,
                    second.participation_level,
                    self.arena.strength[path.second.0 as usize],
                    self.arena.life[path.second.0 as usize],
                ))
            })
            .max_by_key(|(_, participation, strength, _)| {
                (*participation, strength.unsigned_abs())
            });
        let Some((action, _, action_strength, action_life)) = action else {
            return;
        };
        let Some(action_second) = self.arena.link_by_id(action.second) else {
            return;
        };
        let output = action_second.to;
        let sign = action_strength.signum();
        if sign == 0 {
            return;
        }
        let Some(reverse) = self.arena.paths_from(source).into_iter().find(|path| {
            self.arena.link_by_id(path.second).is_some_and(|second| {
                second.to == output && self.arena.strength[path.second.0 as usize].signum() == sign
            })
        }) else {
            return;
        };
        let reverse_index = reverse.second.0 as usize;
        let consolidated = action_strength
            .unsigned_abs()
            .max(UNIT_U64.saturating_mul(2));
        self.arena.strength[reverse_index] = if sign < 0 {
            -i64::try_from(consolidated).unwrap_or(i64::MAX)
        } else {
            i64::try_from(consolidated).unwrap_or(i64::MAX)
        };
        self.arena.life[reverse_index] = self.arena.life[reverse_index].max(action_life);
        let observed = self.arena.strength[reverse_index] / UNIT;
        let resistance =
            self.arena.life[reverse_index].saturating_add(UNIT_U64.saturating_sub(1)) / UNIT_U64;
        let tick = self.tick;
        let slot = self
            .arena
            .link_slot(reverse.second)
            .expect("reverse path link resolves");
        self.arena.edit_link(slot.0, |link| {
            link.coupling = i32::try_from(observed).unwrap_or_else(|_| {
                if observed.is_negative() {
                    i32::MIN
                } else {
                    i32::MAX
                }
            });
            link.resistance = u32::try_from(resistance).unwrap_or(u32::MAX);
            link.last_consequence_tick = Some(tick);
        });
        run.work.total = run.work.total.saturating_add(1);
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::ReversePathConsolidated {
                    source,
                    output,
                    link: reverse.second,
                },
            });
        }
    }

    fn accept_return_origin(&mut self, id: LinkId, origin: u64) -> bool {
        let Some(slot) = self.arena.link_slot(id) else {
            return false;
        };
        let state = self.arena.link_snapshot(slot.0);
        if !state.live
            || state.mode != TransmissionMode::Modulatory
            || state.return_origins.contains(&origin)
        {
            return false;
        }
        let Some(source) = self.arena.junction_by_id(state.from) else {
            return false;
        };
        let Some(target) = self.arena.junction_by_id(state.to) else {
            return false;
        };
        let direct = source.physical_id == origin;
        let local = self
            .arena
            .junctions
            .iter()
            .find(|junction| junction.live && junction.physical_id == origin)
            .is_some_and(|junction| {
                junction.position.saturating_sub(target.position).abs() <= LOCAL_VARIATION_RADIUS
            });
        if !direct && !local {
            return false;
        }
        self.arena.edit_link(slot.0, |link| {
            link.return_origins.push(origin);
            link.return_origins.sort_unstable();
            link.return_origins.dedup();
        });
        true
    }

    pub(crate) fn apply_outcome(
        &mut self,
        junction: JunctionId,
        moment: &Moment,
        run: &mut RunState,
    ) {
        let return_updates_before = run.work.local_return_updates;
        let candidates = if self.protocol.is_sensorimotor() {
            let mut local = self.arena.incoming_index[junction.0 as usize].clone();
            local.extend_from_slice(&self.arena.outgoing_index[junction.0 as usize]);
            local.sort_unstable();
            local.dedup();
            run.cost.allocations = run.cost.allocations.saturating_add(1);
            run.cost.adjacency_accesses = run
                .cost
                .adjacency_accesses
                .saturating_add(u64::try_from(local.len()).unwrap_or(u64::MAX));
            local
        } else {
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
                    if self.protocol.is_sensorimotor() {
                        live_link.last_consequence_tick = Some(self.tick);
                    }
                });
                if self.trace_physics && self.protocol.is_sensorimotor() {
                    run.trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase: moment.phase,
                        event: PhysicalEvent::ConsequenceRecorded { link: id, junction },
                    });
                }
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
