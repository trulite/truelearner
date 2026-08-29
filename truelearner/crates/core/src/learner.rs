use crate::prelude::*;

const CONSTRUCTION_EVIDENCE: u32 = 2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) enum ConsequenceLifetime {
    #[default]
    Ordinary,
    HeldForFirstChoice,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct LearnerReturnMemory {
    pub(crate) link: LinkId,
    pub(crate) generation: Generation,
    pub(crate) origin_physical: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct LearnerConsequenceMemory {
    pub(crate) link: LinkId,
    pub(crate) generation: Generation,
    pub(crate) last_consequence_tick: i64,
    #[serde(default)]
    pub(crate) lifetime: ConsequenceLifetime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LearnerState {
    pub(crate) id: LearnerId,
    pub(crate) parent: Option<LearnerId>,
    pub(crate) surface: JunctionId,
    pub(crate) output: JunctionId,
    pub(crate) junctions: Vec<JunctionId>,
    pub(crate) links: Vec<LinkId>,
    pub(crate) return_memory: Vec<LearnerReturnMemory>,
    pub(crate) consequence_memory: Vec<LearnerConsequenceMemory>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CausalClosureState {
    pub(crate) parent: Option<LearnerId>,
    pub(crate) surface: JunctionId,
    pub(crate) output: JunctionId,
    pub(crate) evidence: u32,
    pub(crate) constructed: Option<LearnerId>,
}

fn same_tick_consequences(arena: &Arena, links: &[LinkId], tick: i64) -> Vec<(LinkId, Generation)> {
    links
        .iter()
        .filter_map(|link| {
            arena.link_by_id(*link).and_then(|state| {
                (state.live && state.last_consequence_tick == Some(tick))
                    .then_some((*link, state.generation))
            })
        })
        .collect()
}

impl Body {
    pub(crate) fn deepest_learner_owning(&self, surface: JunctionId) -> Option<LearnerId> {
        self.learners
            .iter()
            .rev()
            .find(|learner| learner.junctions.binary_search(&surface).is_ok())
            .map(|learner| learner.id)
    }

    fn deepest_learner_owning_link_index(&self, link: LinkId) -> Option<usize> {
        self.learners
            .iter()
            .rposition(|learner| learner.links.binary_search(&link).is_ok())
    }

    pub(crate) fn return_memory_owner(&self, link: LinkId) -> Option<LearnerId> {
        self.protocol
            .constructs_learners()
            .then(|| self.deepest_learner_owning_link_index(link))
            .flatten()
            .map(|index| self.learners[index].id)
    }

    pub(crate) fn learner_owner_for_origin(&self, origin_physical: u64) -> Option<LearnerId> {
        if !self.protocol.constructs_learners() {
            return None;
        }
        let origin = self
            .arena
            .junctions
            .iter()
            .find(|junction| junction.live && junction.physical_id == origin_physical)?
            .id;
        self.deepest_learner_owning(origin)
    }

    pub(crate) fn learner_consequence_tick(
        &self,
        owner: LearnerId,
        link: LinkId,
        generation: Generation,
    ) -> Option<i64> {
        let live = self
            .arena
            .link_by_id(link)
            .is_some_and(|state| state.live && state.generation == generation);
        live.then(|| {
            self.learners
                .iter()
                .find(|learner| learner.id == owner)?
                .consequence_memory
                .iter()
                .find(|memory| memory.link == link && memory.generation == generation)
                .map(|memory| memory.last_consequence_tick)
        })
        .flatten()
    }

    pub(crate) fn held_learner_consequence_tick(
        &self,
        owner: LearnerId,
        link: LinkId,
        generation: Generation,
    ) -> Option<i64> {
        let live = self
            .arena
            .link_by_id(link)
            .is_some_and(|state| state.live && state.generation == generation);
        live.then(|| {
            self.learners
                .iter()
                .find(|learner| learner.id == owner)?
                .consequence_memory
                .iter()
                .find(|memory| {
                    memory.link == link
                        && memory.generation == generation
                        && memory.lifetime == ConsequenceLifetime::HeldForFirstChoice
                })
                .map(|memory| memory.last_consequence_tick)
        })
        .flatten()
    }

    fn remember_learner_consequence_with_lifetime(
        &mut self,
        owner: LearnerId,
        link: LinkId,
        generation: Generation,
        tick: i64,
        lifetime: ConsequenceLifetime,
    ) -> bool {
        if self
            .arena
            .link_by_id(link)
            .is_none_or(|state| !state.live || state.generation != generation)
        {
            return false;
        }
        let Some(learner) = self.learners.iter_mut().find(|learner| learner.id == owner) else {
            return false;
        };
        if let Some(memory) = learner
            .consequence_memory
            .iter_mut()
            .find(|memory| memory.link == link && memory.generation == generation)
        {
            if memory.last_consequence_tick == tick && memory.lifetime == lifetime {
                return false;
            }
            memory.last_consequence_tick = tick;
            memory.lifetime = lifetime;
        } else {
            learner.consequence_memory.push(LearnerConsequenceMemory {
                link,
                generation,
                last_consequence_tick: tick,
                lifetime,
            });
            learner.consequence_memory.sort_unstable();
        }
        true
    }

    pub(crate) fn remember_learner_consequence(
        &mut self,
        owner: LearnerId,
        link: LinkId,
        generation: Generation,
        tick: i64,
    ) -> bool {
        self.remember_learner_consequence_with_lifetime(
            owner,
            link,
            generation,
            tick,
            ConsequenceLifetime::Ordinary,
        )
    }

    pub(crate) fn remember_construction_consequence(
        &mut self,
        owner: LearnerId,
        link: LinkId,
        generation: Generation,
        tick: i64,
    ) -> bool {
        self.remember_learner_consequence_with_lifetime(
            owner,
            link,
            generation,
            tick,
            ConsequenceLifetime::HeldForFirstChoice,
        )
    }

    pub(crate) fn consume_held_learner_consequence(
        &mut self,
        owner: LearnerId,
        link: LinkId,
        generation: Generation,
    ) -> Option<i64> {
        let live = self
            .arena
            .link_by_id(link)
            .is_some_and(|state| state.live && state.generation == generation);
        if !live {
            return None;
        }
        let memory = self
            .learners
            .iter_mut()
            .find(|learner| learner.id == owner)?
            .consequence_memory
            .iter_mut()
            .find(|memory| {
                memory.link == link
                    && memory.generation == generation
                    && memory.lifetime == ConsequenceLifetime::HeldForFirstChoice
            })?;
        memory.lifetime = ConsequenceLifetime::Ordinary;
        Some(memory.last_consequence_tick)
    }

    pub(crate) fn record_learner_consequence(
        &mut self,
        owner: LearnerId,
        link: LinkId,
        generation: Generation,
        tick: i64,
        phase: i32,
        run: &mut RunState,
    ) {
        if !self.remember_learner_consequence(owner, link, generation, tick) {
            return;
        }
        run.work.total = run.work.total.saturating_add(1);
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase,
                event: PhysicalEvent::LearnerConsequenceRecorded {
                    owner,
                    link,
                    generation: generation.0,
                    tick,
                },
            });
        }
    }

    pub(crate) fn return_is_available(&self, link: LinkId) -> bool {
        let Some(state) = self.arena.link_by_id(link) else {
            return false;
        };
        let Some(index) = self
            .return_memory_owner(link)
            .and_then(|owner| self.learners.iter().position(|learner| learner.id == owner))
        else {
            return state.return_origins.is_empty();
        };
        !self.learners[index]
            .return_memory
            .iter()
            .any(|memory| memory.link == link && memory.generation == state.generation)
    }

    pub(crate) fn remember_return_origin(
        &mut self,
        owner: Option<LearnerId>,
        link: LinkId,
        generation: Generation,
        origin_physical: u64,
    ) -> Option<LearnerId> {
        let Some(index) =
            owner.and_then(|owner| self.learners.iter().position(|learner| learner.id == owner))
        else {
            let slot = self
                .arena
                .link_slot(link)
                .expect("validated return link exists");
            self.arena.edit_link(slot.0, |state| {
                state.return_origins.push(origin_physical);
                state.return_origins.sort_unstable();
                state.return_origins.dedup();
            });
            return None;
        };
        let learner = &mut self.learners[index];
        learner.return_memory.push(LearnerReturnMemory {
            link,
            generation,
            origin_physical,
        });
        learner.return_memory.sort_unstable();
        learner.return_memory.dedup();
        Some(learner.id)
    }

    pub(crate) fn return_origin_is_available(
        &self,
        owner: Option<LearnerId>,
        link: LinkId,
        generation: Generation,
        origin_physical: u64,
    ) -> bool {
        let Some(index) =
            owner.and_then(|owner| self.learners.iter().position(|learner| learner.id == owner))
        else {
            return self
                .arena
                .link_by_id(link)
                .is_some_and(|state| !state.return_origins.contains(&origin_physical));
        };
        !self.learners[index].return_memory.iter().any(|memory| {
            memory.link == link
                && memory.generation == generation
                && memory.origin_physical == origin_physical
        })
    }

    pub(crate) fn observe_causal_closure(
        &mut self,
        surface: JunctionId,
        output: JunctionId,
        lineage: &[LinkId],
        moment: &Moment,
        run: &mut RunState,
    ) {
        if !self.protocol.constructs_learners() {
            return;
        }
        let mut links = lineage.to_vec();
        links.sort_unstable();
        links.dedup();
        if links.is_empty()
            || links
                .iter()
                .any(|link| self.arena.link_by_id(*link).is_none_or(|state| !state.live))
        {
            return;
        }

        let mut junctions = vec![surface, output];
        for link in &links {
            let state = self
                .arena
                .link_by_id(*link)
                .expect("validated causal lineage remains live");
            junctions.extend([state.from, state.to]);
        }
        junctions.sort_unstable();
        junctions.dedup();
        let parent = self.deepest_learner_owning(surface);
        if self.protocol.requires_boundary_novelty() {
            let novel_members = parent
                .and_then(|parent| self.learners.iter().find(|learner| learner.id == parent))
                .map_or(junctions.len(), |parent| {
                    junctions
                        .iter()
                        .filter(|junction| parent.junctions.binary_search(junction).is_err())
                        .count()
                });
            let eligible = parent.is_none() || novel_members > 0;
            if self.trace_physics {
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::BoundaryNoveltyEvaluated {
                        parent,
                        surface,
                        output,
                        proposed_members: u32::try_from(junctions.len()).unwrap_or(u32::MAX),
                        novel_members: u32::try_from(novel_members).unwrap_or(u32::MAX),
                        eligible,
                    },
                });
            }
            if !eligible {
                return;
            }
        }
        let existing = self.causal_closures.iter().position(|closure| {
            closure.parent == parent && closure.surface == surface && closure.output == output
        });
        let closure_index = existing.unwrap_or_else(|| {
            self.causal_closures.push(CausalClosureState {
                parent,
                surface,
                output,
                evidence: 0,
                constructed: None,
            });
            self.causal_closures.len() - 1
        });

        let closure = &mut self.causal_closures[closure_index];
        if closure.constructed.is_some() {
            return;
        }
        closure.evidence = closure.evidence.saturating_add(1);
        let evidence = closure.evidence;
        let parent = closure.parent;
        run.work.total = run.work.total.saturating_add(1);
        run.work.causal_closure_observations =
            run.work.causal_closure_observations.saturating_add(1);
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::CausalClosureObserved {
                    parent,
                    surface,
                    output,
                    evidence,
                },
            });
        }
        if evidence < CONSTRUCTION_EVIDENCE || self.learners.len() >= self.arena.junctions.len() {
            return;
        }

        let id = LearnerId(self.next_learner_id);
        self.next_learner_id = self.next_learner_id.saturating_add(1);
        self.learners.push(LearnerState {
            id,
            parent,
            surface,
            output,
            junctions,
            links,
            return_memory: Vec::new(),
            consequence_memory: Vec::new(),
        });
        self.causal_closures[closure_index].constructed = Some(id);
        run.work.total = run.work.total.saturating_add(1);
        run.work.learner_constructions = run.work.learner_constructions.saturating_add(1);
        if self.trace_physics {
            let learner = self.learners.last().expect("constructed learner exists");
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::LearnerConstructed {
                    learner: id,
                    parent,
                    surface,
                    output,
                    junction_count: u32::try_from(learner.junctions.len()).unwrap_or(u32::MAX),
                    link_count: u32::try_from(learner.links.len()).unwrap_or(u32::MAX),
                },
            });
        }
        if self.protocol.composes_construction_outcome() {
            let consequences = same_tick_consequences(
                &self.arena,
                &self
                    .learners
                    .last()
                    .expect("constructed learner exists")
                    .links,
                self.tick,
            );
            for (link, generation) in consequences {
                if self.protocol.holds_construction_outcome_for_first_choice() {
                    if self.remember_construction_consequence(id, link, generation, self.tick) {
                        run.work.total = run.work.total.saturating_add(1);
                        if self.trace_physics {
                            run.trace.push(PhysicalTransition {
                                tick: self.tick,
                                phase: moment.phase,
                                event: PhysicalEvent::LearnerConsequenceRecorded {
                                    owner: id,
                                    link,
                                    generation: generation.0,
                                    tick: self.tick,
                                },
                            });
                        }
                    }
                } else {
                    self.record_learner_consequence(
                        id,
                        link,
                        generation,
                        self.tick,
                        moment.phase,
                        run,
                    );
                }
            }
        }
    }

    pub(crate) fn recursive_learning_bytes(&self) -> usize {
        let learner_members = self.learners.iter().fold(0usize, |total, learner| {
            total
                .saturating_add(learner.junctions.capacity() * std::mem::size_of::<JunctionId>())
                .saturating_add(learner.links.capacity() * std::mem::size_of::<LinkId>())
                .saturating_add(
                    learner.return_memory.capacity() * std::mem::size_of::<LearnerReturnMemory>(),
                )
                .saturating_add(
                    learner.consequence_memory.capacity()
                        * std::mem::size_of::<LearnerConsequenceMemory>(),
                )
        });
        self.learners
            .capacity()
            .saturating_mul(std::mem::size_of::<LearnerState>())
            .saturating_add(
                self.causal_closures
                    .capacity()
                    .saturating_mul(std::mem::size_of::<CausalClosureState>()),
            )
            .saturating_add(learner_members)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn junction(physical_id: u64) -> Junction {
        Junction {
            physical_id,
            position: i32::try_from(physical_id).unwrap(),
            region: 0,
            threshold: 1,
            resistance: u32::MAX,
        }
    }

    fn link(from: JunctionId, to: JunctionId) -> Link {
        Link {
            from,
            to,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: u32::MAX,
            mode: TransmissionMode::Drive,
        }
    }

    #[test]
    fn construction_outcome_composition_selects_only_live_same_tick_lineage() {
        let mut body = Body::with_capacity(8, 8);
        let source = body.add_junction(junction(1));
        let target = body.add_junction(junction(2));
        let other = body.add_junction(junction(3));
        let current = body.add_link(link(source, target));
        let older = body.add_link(link(target, source));
        let unrelated = body.add_link(link(source, other));
        let dead = body.add_link(link(other, target));
        let tick = 7;

        for (id, consequence_tick) in [(current, tick), (older, tick - 1), (unrelated, tick)] {
            let slot = body.arena.link_slot(id).unwrap();
            body.arena.edit_link(slot.0, |state| {
                state.last_consequence_tick = Some(consequence_tick)
            });
        }
        let dead_slot = body.arena.link_slot(dead).unwrap();
        body.arena.edit_link(dead_slot.0, |state| {
            state.live = false;
            state.last_consequence_tick = Some(tick);
        });

        let selected = same_tick_consequences(&body.arena, &[current, older, dead], tick);
        let generation = body.arena.link_by_id(current).unwrap().generation;
        assert_eq!(selected, vec![(current, generation)]);
        assert!(!selected.iter().any(|(link, _)| *link == unrelated));
    }

    #[test]
    fn bounded_construction_continuation_consumes_exact_memory_without_refreshing_tick() {
        let mut body = Body::with_capacity(4, 4);
        let source = body.add_junction(junction(1));
        let target = body.add_junction(junction(2));
        let link = body.add_link(link(source, target));
        let generation = body.arena.link_by_id(link).unwrap().generation;
        let owner = LearnerId(1);
        body.learners.push(LearnerState {
            id: owner,
            parent: None,
            surface: source,
            output: target,
            junctions: vec![source, target],
            links: vec![link],
            return_memory: Vec::new(),
            consequence_memory: Vec::new(),
        });

        assert!(body.remember_construction_consequence(owner, link, generation, 7));
        assert_eq!(
            body.held_learner_consequence_tick(owner, link, generation),
            Some(7)
        );
        assert_eq!(
            body.consume_held_learner_consequence(owner, link, generation),
            Some(7)
        );
        assert_eq!(
            body.held_learner_consequence_tick(owner, link, generation),
            None
        );
        assert_eq!(
            body.learner_consequence_tick(owner, link, generation),
            Some(7)
        );
        assert_eq!(
            body.consume_held_learner_consequence(owner, link, generation),
            None
        );
    }
}
