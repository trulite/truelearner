use crate::prelude::*;

impl Body {
    pub(crate) fn enter(&mut self, input: Input) {
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
            let mode = if let Some((link_id, generation)) = firing.link {
                let Some(link_slot) = self.arena.link_slot(link_id) else {
                    continue;
                };
                let link = self.arena.link_snapshot(link_slot.0);
                run.cost.touch::<LinkState>(1);
                if !link.live || link.generation != generation {
                    continue;
                }
                link.mode
            } else {
                TransmissionMode::Drive
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
    });
}
