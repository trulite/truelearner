use crate::prelude::*;

const CONSTRUCTION_EVIDENCE: u32 = 2;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LearnerState {
    pub(crate) id: LearnerId,
    pub(crate) parent: Option<LearnerId>,
    pub(crate) surface: JunctionId,
    pub(crate) output: JunctionId,
    pub(crate) junctions: Vec<JunctionId>,
    pub(crate) links: Vec<LinkId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CausalClosureState {
    pub(crate) parent: Option<LearnerId>,
    pub(crate) surface: JunctionId,
    pub(crate) output: JunctionId,
    pub(crate) evidence: u32,
    pub(crate) constructed: Option<LearnerId>,
}

impl Body {
    pub(crate) fn observe_causal_closure(
        &mut self,
        surface: JunctionId,
        output: JunctionId,
        lineage: &[LinkId],
        moment: &Moment,
        run: &mut RunState,
    ) {
        if self.protocol != Protocol::RecursiveLearnerConstruction {
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

        let existing = self
            .causal_closures
            .iter()
            .position(|closure| closure.surface == surface && closure.output == output);
        let closure_index = existing.unwrap_or_else(|| {
            let parent = self
                .learners
                .iter()
                .rev()
                .find(|learner| learner.junctions.binary_search(&surface).is_ok())
                .map(|learner| learner.id);
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
        let id = LearnerId(self.next_learner_id);
        self.next_learner_id = self.next_learner_id.saturating_add(1);
        self.learners.push(LearnerState {
            id,
            parent,
            surface,
            output,
            junctions,
            links,
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
    }

    pub(crate) fn recursive_learning_bytes(&self) -> usize {
        let learner_members = self.learners.iter().fold(0usize, |total, learner| {
            total
                .saturating_add(learner.junctions.capacity() * std::mem::size_of::<JunctionId>())
                .saturating_add(learner.links.capacity() * std::mem::size_of::<LinkId>())
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
