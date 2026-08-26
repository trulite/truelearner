use crate::prelude::*;

impl Body {
    /// Successful outcome makes both physical links strong and executable.
    pub(crate) fn strengthen(
        &mut self,
        junction: JunctionId,
        work: &mut Work,
        phase: i32,
        trace: &mut Vec<PhysicalTransition>,
    ) {
        let mut strengthened = HashSet::new();
        for path in self
            .arena
            .paths()
            .into_iter()
            .filter(|path| path.junction == junction)
        {
            let first = self
                .arena
                .link_snapshot(self.arena.link_slot(path.first).unwrap().0);
            let second = self
                .arena
                .link_snapshot(self.arena.link_slot(path.second).unwrap().0);
            if first.participation_level > 0 && second.participation_level > 0 {
                self.strengthen_link(path.first, work, phase, trace);
                self.strengthen_link(path.second, work, phase, trace);
                strengthened.extend([path.first, path.second]);
            }
        }
        let remaining = self
            .arena
            .links
            .iter()
            .filter(|link| {
                link.live
                    && link.from == junction
                    && link.mode == TransmissionMode::Drive
                    && link.trigger == TransmissionTrigger::SourceFires
                    && link.participation_level > 0
                    && !strengthened.contains(&link.id)
            })
            .map(|link| link.id)
            .collect::<Vec<_>>();
        for link in remaining {
            self.strengthen_link(link, work, phase, trace);
        }
    }

    /// Make one used link strong enough to fire its target junction later.
    pub(crate) fn strengthen_link(
        &mut self,
        id: LinkId,
        work: &mut Work,
        phase: i32,
        trace: &mut Vec<PhysicalTransition>,
    ) {
        let slot = self.arena.link_slot(id).unwrap();
        let link = self.arena.link_snapshot(slot.0);
        let threshold = self
            .arena
            .junction_snapshot(self.arena.junction_slot(link.to).unwrap().0)
            .threshold;
        let before = self.arena.strength[id.0 as usize];
        let sign = before.signum();
        if sign == 0 {
            return;
        }
        let required = i64::from(threshold)
            .saturating_mul(UNIT)
            .saturating_mul(sign);
        let after = if before.unsigned_abs() < required.unsigned_abs() {
            required
        } else {
            before
        };
        let coupling = i32::try_from(after / UNIT).unwrap_or_else(|_| {
            if after.is_negative() {
                i32::MIN
            } else {
                i32::MAX
            }
        });

        self.arena.strength[id.0 as usize] = after;
        self.arena.life[id.0 as usize] = u64::from(u32::MAX).saturating_mul(UNIT_U64);
        self.arena.decay_remainder[id.0 as usize] = 0;
        self.arena.edit_link(slot.0, |state| {
            state.coupling = coupling;
            state.resistance = u32::MAX;
            state.decay_load = 0;
        });
        work.total = work.total.saturating_add(3);

        if self.trace_physics {
            trace.push(PhysicalTransition {
                tick: self.tick,
                phase,
                event: PhysicalEvent::LinkStrengthened {
                    link: id,
                    from: link.from,
                    to: link.to,
                    coupling_before: link.coupling,
                    coupling_after: coupling,
                },
            });
        }
    }
}
