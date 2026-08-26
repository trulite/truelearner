use crate::prelude::*;

/// Two live links joined at one junction.
#[derive(Clone, Copy)]
pub(crate) struct Path {
    pub(crate) first: LinkId,
    pub(crate) junction: JunctionId,
    pub(crate) second: LinkId,
}

impl Arena {
    pub(crate) fn paths(&self) -> Vec<Path> {
        self.links
            .iter()
            .filter(|second| self.path_link(second) && self.is_output_junction(second.to))
            .flat_map(|second| {
                let junction = self.junction_by_id(second.from).unwrap();
                self.incoming_index[junction.id.0 as usize]
                    .iter()
                    .filter_map(move |id| {
                        let first = self.link_by_id(*id)?;
                        let source = self.junction_by_id(first.from)?;
                        (self.path_link(first)
                            && first.to == junction.id
                            && first.from != junction.id
                            && source.position == junction.position)
                            .then_some(Path {
                                first: first.id,
                                junction: junction.id,
                                second: second.id,
                            })
                    })
            })
            .collect()
    }

    pub(crate) fn completes_path(&self, link: LinkId) -> bool {
        let Some(second) = self.link_by_id(link) else {
            return false;
        };
        if !self.path_link(second) || !self.is_output_junction(second.to) {
            return false;
        }
        let Some(junction) = self.junction_by_id(second.from) else {
            return false;
        };
        self.incoming_index[junction.id.0 as usize]
            .iter()
            .any(|id| {
                self.link_by_id(*id).is_some_and(|first| {
                    self.path_link(first)
                        && first.from != junction.id
                        && self
                            .junction_by_id(first.from)
                            .is_some_and(|source| source.position == junction.position)
                })
            })
    }

    fn path_link(&self, link: &LinkState) -> bool {
        link.live
            && link.mode == TransmissionMode::Drive
            && link.trigger == TransmissionTrigger::SourceFires
            && self.junction_by_id(link.from).is_some()
            && self.junction_by_id(link.to).is_some()
    }
}

impl Body {
    pub(crate) fn form_from(&mut self, fired: &Fired, moment: &Moment, run: &mut RunState) {
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::Fire {
                    junction: fired.junction,
                },
            });
        }
        run.work.total = run.work.total.saturating_add(1);
        if fired.external {
            self.form(
                fired.junction,
                &mut run.work,
                &mut run.cost,
                moment.phase,
                &mut run.trace,
            );
        }
    }

    /// Form complete source → junction → output paths in one physical event.
    pub(crate) fn form(
        &mut self,
        source: JunctionId,
        work: &mut Work,
        cost: &mut ExecutionCost,
        phase: i32,
        trace: &mut Vec<PhysicalTransition>,
    ) {
        let source_state = self
            .arena
            .junction_snapshot(self.arena.junction_slot(source).unwrap().0);
        let existing = self.arena.paths();
        cost.allocations = cost.allocations.saturating_add(2);
        cost.touch::<JunctionState>(self.arena.junctions.len());
        cost.touch::<LinkState>(self.arena.links.len());

        let mut outputs = self
            .arena
            .junctions
            .iter()
            .filter_map(|target| {
                let distance = target.position.saturating_sub(source_state.position).abs();
                (target.live
                    && target.id != source
                    && self.arena.is_output_junction(target.id)
                    && (1..=LOCAL_VARIATION_RADIUS).contains(&distance))
                .then_some((distance, target.position, target.id))
            })
            .collect::<Vec<_>>();
        outputs.sort_by_key(|target| (target.0, target.1));

        for (distance, _, output) in outputs {
            for sign in [1_i64, -1_i64] {
                let exists = existing.iter().any(|path| {
                    let first = self
                        .arena
                        .link_snapshot(self.arena.link_slot(path.first).unwrap().0);
                    let second = self
                        .arena
                        .link_snapshot(self.arena.link_slot(path.second).unwrap().0);
                    first.from == source
                        && second.to == output
                        && self.arena.strength[path.second.0 as usize].signum() == sign
                });
                if exists {
                    continue;
                }
                let physical_id = self
                    .arena
                    .junctions
                    .iter()
                    .map(|junction| junction.physical_id)
                    .max()
                    .unwrap_or(0)
                    .checked_add(1)
                    .expect("junction identity exhausted");
                let junction = self.add_junction(Junction {
                    physical_id,
                    position: source_state.position,
                    region: source_state.region,
                    threshold: 1,
                    resistance: 1,
                });
                work.total = work.total.saturating_add(1);
                work.local_junction_proposals = work.local_junction_proposals.saturating_add(1);
                if self.trace_physics {
                    trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase,
                        event: PhysicalEvent::JunctionProposal {
                            junction,
                            source,
                            target: output,
                        },
                    });
                }
                self.form_link(source, junction, UNIT, 1, work);
                self.form_link(
                    junction,
                    output,
                    UNIT.saturating_mul(sign),
                    i64::from(distance.max(1)),
                    work,
                );
            }
        }
    }
}
