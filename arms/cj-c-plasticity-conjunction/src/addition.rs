// ORGANISM-VISIBLE CANDIDATE ADDITION START

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArrowState {
    pub from_physical: u64,
    pub to_physical: u64,
    pub coupling: i32,
    pub resistance: u32,
    pub generation: u32,
    pub live: bool,
}

impl PlasticSubstrate {
    pub fn propagate_cj_c(&mut self) -> Execution {
        let start_fingerprint = self.complete_fingerprint();
        let mut trace = Vec::new();
        let mut crossings = Vec::new();
        let mut work = WorkLedger::default();

        while !self.pending.is_empty() {
            let mut first = 0;
            for candidate in 1..self.pending.len() {
                work.queue_comparisons += 1;
                if self.spike_order(candidate, first) == Ordering::Less {
                    first = candidate;
                }
            }
            let spike = self.pending.remove(first);
            let external_arrival = spike.arrow.is_none();
            self.elapse_to(spike.arrival_tick, &mut work);
            self.tick = spike.arrival_tick;
            work.spikes_delivered += 1;
            work.generation_checks += 1;

            if let Some((arrow_id, generation)) = spike.arrow {
                let arrow = &self.arrows[arrow_id.0];
                if !arrow.live || arrow.generation != generation {
                    continue;
                }
            }
            let target = &self.cells[spike.target.0];
            if !target.live || target.generation != spike.target_generation {
                continue;
            }

            self.apply_local_return(spike.target, self.tick, &mut work);
            self.decay_cell(spike.target, self.tick);
            let target = &mut self.cells[spike.target.0];
            target.state = target.state.saturating_add(spike.impulse);
            work.state_updates += 1;
            work.threshold_checks += 1;
            let fires = self.tick >= target.refractory_until && target.state >= target.threshold;
            trace.push(TraceEntry {
                tick: self.tick,
                target_physical: target.physical_id,
                impulse: spike.impulse,
                fired: fires,
            });
            if !fires {
                continue;
            }

            target.state = 0;
            target.refractory_until = self.tick.saturating_add(1);
            work.firings += 1;
            let source = spike.target;
            let origin_physical = target.physical_id;
            let source_generation = target.generation;

            if !external_arrival {
                self.expose_from_current_firings(source, self.tick, &mut work);
            }
            self.apply_incident_support(source, self.tick, &mut work);
            if external_arrival {
                self.propose_local_arrows(source, &mut work);
            }
            let outgoing = self
                .arrows
                .iter()
                .enumerate()
                .map(|(index, arrow)| (ArrowId(index), arrow.clone()))
                .collect::<Vec<_>>();
            for (arrow_id, arrow) in outgoing {
                work.arrow_checks += 1;
                if !arrow.live
                    || arrow.from != source
                    || arrow.source_generation != source_generation
                {
                    continue;
                }
                let from = &self.cells[arrow.from.0];
                let to = &self.cells[arrow.to.0];
                if from.region != to.region {
                    crossings.push(Crossing {
                        tick: self.tick,
                        from_physical: from.physical_id,
                        to_physical: to.physical_id,
                        from_region: from.region,
                        to_region: to.region,
                        impulse: arrow.coupling,
                    });
                }
                let live_arrow = &mut self.arrows[arrow_id.0];
                live_arrow.eligible_until = Some(self.tick.saturating_add(LOCAL_WINDOW));
                work.local_eligibility_writes += 1;
                self.pending.push(Spike {
                    arrival_tick: self.tick.saturating_add(arrow.delay),
                    phase: arrow.phase,
                    origin_physical,
                    target: arrow.to,
                    target_generation: to.generation,
                    impulse: arrow.coupling,
                    serial: self.next_serial,
                    arrow: Some((arrow_id, arrow.generation)),
                });
                self.next_serial = self.next_serial.wrapping_add(1);
                work.spikes_emitted += 1;
            }
        }

        Execution {
            start_fingerprint,
            end_fingerprint: self.complete_fingerprint(),
            permanent_fingerprint: self.permanent_fingerprint(),
            trace,
            crossings,
            work,
            naturally_quiescent: self.pending.is_empty(),
        }
    }

    fn apply_incident_support(&mut self, cell: CellId, tick: i64, work: &mut WorkLedger) {
        let active = self
            .arrows
            .iter()
            .enumerate()
            .filter_map(|(index, arrow)| {
                (arrow.live
                    && arrow.to == cell
                    && arrow.eligible_until.is_some_and(|end| tick <= end))
                .then_some((ArrowId(index), arrow.from))
            })
            .collect::<Vec<_>>();
        let mut sources = active.iter().map(|(_, source)| *source).collect::<Vec<_>>();
        sources.sort_by_key(|source| self.cells[source.0].physical_id);
        sources.dedup();
        if sources.len() < 2 {
            return;
        }
        for (arrow_id, _) in active {
            let arrow = &mut self.arrows[arrow_id.0];
            arrow.resistance = arrow
                .resistance
                .saturating_add(LOCAL_RETURN_STRENGTH);
            arrow.eligible_until = None;
            work.local_return_updates += 1;
        }
    }

    fn expose_from_current_firings(
        &mut self,
        source: CellId,
        tick: i64,
        work: &mut WorkLedger,
    ) {
        let position = self.cells[source.0].position;
        let mut others = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(index, cell)| {
                (CellId(index) != source
                    && cell.live
                    && cell.position == position
                    && cell.refractory_until > tick
                    && self.arrows.iter().any(|arrow| {
                        arrow.live
                            && arrow.to == CellId(index)
                            && arrow.eligible_until.is_some_and(|end| tick <= end)
                    }))
                    .then_some(CellId(index))
            })
            .collect::<Vec<_>>();
        others.sort_by_key(|other| self.cells[other.0].physical_id);
        for other in others {
            let mut sources = [source, other];
            sources.sort_by_key(|item| self.cells[item.0].physical_id);
            if self.has_common_live_target(sources) {
                continue;
            }
            let Some(target) = self.first_uncommitted_at(position, sources) else {
                continue;
            };
            for from in sources {
                let generation = u32::try_from(self.arrows.len())
                    .unwrap_or(u32::MAX)
                    .saturating_add(2);
                self.arrows.push(Arrow {
                    from,
                    to: target,
                    delay: 1,
                    phase: 0,
                    coupling: 2,
                    source_generation: self.cells[from.0].generation,
                    generation,
                    resistance: 1,
                    live: true,
                    eligible_until: None,
                });
                work.local_structural_proposals += 1;
            }
        }
    }

    fn has_common_live_target(&self, sources: [CellId; 2]) -> bool {
        self.cells.iter().enumerate().any(|(index, _)| {
            let target = CellId(index);
            sources.iter().all(|source| {
                self.arrows.iter().any(|arrow| {
                    arrow.live && arrow.from == *source && arrow.to == target
                })
            })
        })
    }

    fn first_uncommitted_at(&self, position: i32, sources: [CellId; 2]) -> Option<CellId> {
        let mut cells = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(index, cell)| {
                let candidate = CellId(index);
                (cell.live
                    && cell.position == position
                    && !sources.contains(&candidate)
                    && cell.refractory_until <= self.tick
                    && !self.arrows.iter().any(|arrow| {
                        arrow.live && (arrow.from == candidate || arrow.to == candidate)
                    }))
                .then_some((cell.physical_id, candidate))
            })
            .collect::<Vec<_>>();
        cells.sort_by_key(|(physical_id, _)| *physical_id);
        cells.first().map(|(_, cell)| *cell)
    }

    pub fn arrow_states(&self) -> Vec<ArrowState> {
        self.arrows
            .iter()
            .map(|arrow| ArrowState {
                from_physical: self.cells[arrow.from.0].physical_id,
                to_physical: self.cells[arrow.to.0].physical_id,
                coupling: arrow.coupling,
                resistance: arrow.resistance,
                generation: arrow.generation,
                live: arrow.live,
            })
            .collect()
    }

    pub fn tick(&self) -> i64 {
        self.tick
    }
}

// ORGANISM-VISIBLE CANDIDATE ADDITION END
