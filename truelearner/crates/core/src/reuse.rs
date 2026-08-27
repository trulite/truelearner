use crate::prelude::*;

impl Body {
    /// Record participation and open the physical way back when a path completes.
    pub(crate) fn reuse(&mut self, link: LinkId) {
        let Some(slot) = self.arena.link_slot(link) else {
            return;
        };
        self.arena.edit_link(slot.0, |state| {
            state.participation_level = state
                .participation_level
                .saturating_add(PARTICIPATION_IMPULSE);
        });
        let Some(path) = self.arena.path_for_second(link) else {
            return;
        };
        let Some(output) = self.arena.link_by_id(path.second).map(|link| link.to) else {
            return;
        };
        let Some(outcome) = self.outcome_source_for_output(output) else {
            return;
        };
        if self.arena.return_links(&[outcome]).iter().any(|id| {
            self.arena
                .link_snapshot(self.arena.link_slot(*id).unwrap().0)
                .to
                == path.junction
        }) {
            return;
        }
        self.add_link(Link {
            from: outcome,
            to: path.junction,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: if self.protocol.is_sensorimotor() {
                LOCAL_RETURN_STRENGTH
            } else {
                u32::MAX
            },
            mode: TransmissionMode::Modulatory,
        });
    }
}
