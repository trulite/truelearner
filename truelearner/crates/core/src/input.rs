use crate::prelude::*;

impl Body {
    pub(crate) fn enter(&mut self, input: Input) {
        self.enter_physical(PhysicalInput {
            input,
            incidence: PhysicalIncidence::Sample,
        });
    }

    pub(crate) fn enter_physical(&mut self, physical: PhysicalInput) {
        let input = physical.input;
        self.arena.require_junction(input.target);
        assert!(
            input.arrival_tick >= self.tick,
            "physical arrivals cannot precede current body time"
        );
        let target = self
            .arena
            .junction_snapshot(self.arena.junction_slot(input.target).unwrap().0);
        let mut ignored = ExecutionCost::default();
        self.pending.push(
            Firing {
                arrival_tick: input.arrival_tick,
                phase: input.phase,
                causal_wave: 0,
                origin_physical: input.origin_physical,
                causal_lineage: self.protocol.preserves_causal_lineage().then(|| {
                    match physical.incidence {
                        PhysicalIncidence::Sample => {
                            CausalLineage::singleton(input.origin_physical, input.arrival_tick)
                        }
                        PhysicalIncidence::Transition => {
                            CausalLineage::transitioned(input.origin_physical, input.arrival_tick)
                        }
                    }
                }),
                physical_incidence: physical.incidence,
                target_physical: target.physical_id,
                target: input.target,
                target_generation: target.generation,
                impulse: input.impulse,
                strength: i64::from(input.impulse).saturating_mul(UNIT),
                serial: self.next_serial,
                link: None,
            },
            &mut ignored,
        );
        self.next_serial = self.next_serial.wrapping_add(1);
    }

    pub(crate) fn meet_links(&mut self, run: &mut RunState) -> Option<Moment> {
        let batch = self.pending.next_wave(&mut run.cost)?;
        let first = &batch[0];
        run.cost.observe_batch(batch.len());
        let (tick, phase, causal) = (first.arrival_tick, first.phase, first.causal_wave);
        self.elapse_to_observed(tick, &mut run.work, &mut run.cost, phase, &mut run.trace);
        self.tick = tick;

        let mut incidences = Vec::new();
        for firing in batch {
            run.cost.touch::<Firing>(1);
            let (mode, source, source_physical, source_region, link, completes_path, path_owner) =
                if let Some((link_id, generation)) = firing.link {
                    let Some(link_slot) = self.arena.link_slot(link_id) else {
                        continue;
                    };
                    let link = self.arena.link_snapshot(link_slot.0);
                    run.cost.touch::<LinkState>(1);
                    if !link.live || link.generation != generation {
                        continue;
                    }
                    (
                        link.mode,
                        Some(link.from),
                        self.arena
                            .junction_slot(link.from)
                            .map(|slot| self.arena.junction_snapshot(slot.0).physical_id),
                        self.arena
                            .junction_slot(link.from)
                            .map(|slot| self.arena.junction_snapshot(slot.0).region),
                        Some(link_id),
                        self.arena.completes_path(link_id),
                        self.return_memory_owner(link_id),
                    )
                } else {
                    (TransmissionMode::Drive, None, None, None, None, false, None)
                };
            let Some(target_slot) = self.arena.junction_slot(firing.target) else {
                continue;
            };
            let target = self.arena.junction_snapshot(target_slot.0);
            run.cost.touch::<JunctionState>(1);
            if target.id != firing.target
                || !target.live
                || target.generation != firing.target_generation
            {
                continue;
            }
            if self.trace_physics && mode == TransmissionMode::Drive {
                if link.is_none() {
                    run.trace.push(PhysicalTransition {
                        tick,
                        phase,
                        event: PhysicalEvent::PhysicalIncidenceObserved {
                            target: firing.target,
                            origin_physical: firing.origin_physical,
                            incidence: firing.physical_incidence,
                            causal_wave: causal,
                        },
                    });
                }
                run.trace.push(PhysicalTransition {
                    tick,
                    phase,
                    event: PhysicalEvent::DriveProvenanceObserved {
                        source,
                        target: firing.target,
                        source_physical,
                        target_physical: target.physical_id,
                        source_region,
                        target_region: target.region,
                        link,
                        completes_path,
                        carried_origin: firing.origin_physical,
                        origin_owner: self.learner_owner_for_origin(firing.origin_physical),
                        path_owner,
                        strength: firing.strength,
                        causal_wave: causal,
                    },
                });
            }
            add_incidence(&mut incidences, firing, mode);
        }
        Some(Moment {
            phase,
            causal,
            incidences,
        })
    }
}

fn add_incidence(incidences: &mut Vec<Incidence>, firing: Firing, mode: TransmissionMode) {
    if let Some(incidence) = incidences
        .iter_mut()
        .find(|incidence| incidence.junction == firing.target)
    {
        match mode {
            TransmissionMode::Drive => incidence.inputs.push(firing),
            TransmissionMode::Modulatory => incidence.outcomes.push(firing),
        }
        return;
    }

    let junction = firing.target;
    let (inputs, outcomes) = match mode {
        TransmissionMode::Drive => (vec![firing], Vec::new()),
        TransmissionMode::Modulatory => (Vec::new(), vec![firing]),
    };
    incidences.push(Incidence {
        junction,
        inputs,
        outcomes,
        supplied_opportunity: 0,
    });
}
