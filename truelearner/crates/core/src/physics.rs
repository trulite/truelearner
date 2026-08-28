use crate::prelude::*;

impl Body {
    pub(crate) fn propagate(&mut self) -> RunResult {
        let protocol = self.protocol.bindings();
        crate::core::run(self, protocol)
    }

    pub(crate) fn propagate_bounded(&mut self, max_moments: u64) -> RunResult {
        let protocol = self.protocol.bindings();
        crate::core::run_bounded(self, protocol, max_moments)
    }
}

// Bind each line of the algorithm to one coherent physical implementation.
impl Protocol {
    pub(crate) fn bindings(self) -> Bindings {
        match self {
            Self::Physical => Bindings {
                start,
                links_meet: Body::meet_links,
                choose: Body::choose_at,
                outcome_returns: Body::outcomes_return,
                strengthen: Body::strengthen_outcomes,
                fire_junction: Body::fire,
                form_paths: Body::form_from,
                fire_output: Body::fire_output_from,
                hold,
                finish,
            },
            Self::UnansweredReturnDeferral => Bindings {
                start,
                links_meet: Body::meet_links,
                choose: Body::choose_after_unanswered_return,
                outcome_returns: Body::outcomes_return,
                strengthen: Body::strengthen_outcomes,
                fire_junction: Body::fire,
                form_paths: Body::form_from,
                fire_output: Body::fire_output_from,
                hold,
                finish,
            },
            Self::UnansweredReturnReplacement => Bindings {
                start,
                links_meet: Body::meet_links,
                choose: Body::choose_and_replace_unanswered_return,
                outcome_returns: Body::outcomes_return,
                strengthen: Body::strengthen_outcomes,
                fire_junction: Body::fire,
                form_paths: Body::form_from,
                fire_output: Body::fire_output_from,
                hold,
                finish,
            },
            Self::SensorimotorCandidate => Bindings {
                start,
                links_meet: Body::meet_links,
                choose: Body::choose_sensorimotor_candidate,
                outcome_returns: Body::outcomes_return,
                strengthen: Body::strengthen_candidate_outcomes,
                fire_junction: Body::fire,
                form_paths: Body::form_from_participation,
                fire_output: Body::fire_output_from,
                hold,
                finish,
            },
            Self::SensorimotorSynthesis => Bindings {
                start,
                links_meet: Body::meet_links,
                choose: Body::choose_sensorimotor_candidate,
                outcome_returns: Body::outcomes_return,
                strengthen: Body::strengthen_candidate_outcomes,
                fire_junction: Body::fire,
                form_paths: Body::form_from_participation,
                fire_output: Body::fire_output_from,
                hold,
                finish,
            },
            Self::RecursiveLearnerConstruction
            | Self::RecursiveLearnerCausalLineage
            | Self::RecursiveLearnerConsequenceBornClosure
            | Self::RecursiveLearnerConsequenceCohortClosure
            | Self::RecursiveLearnerEligibleReturnClosure
            | Self::RecursiveLearnerBoundaryNovelty
            | Self::RecursiveLearnerOwnerFactorization
            | Self::RecursiveLearnerCausalOriginFactorization
            | Self::RecursiveLearnerRegionalPathClosure
            | Self::RecursiveLearnerBoundaryEffectTerminal
            | Self::RecursiveLearnerConsequenceBornReturn
            | Self::RecursiveLearnerPhysicalTransitionReturn
            | Self::RecursiveLearnerFreshOpportunity
            | Self::RecursiveLearnerRootFreshOpportunity
            | Self::RecursiveLearnerTransitionContinuation
            | Self::RecursiveLearnerCoherentEffect
            | Self::RecursiveLearnerCompletedCycle
            | Self::RecursiveLearnerConstructionOutcomeComposition
            | Self::RecursiveLearnerBoundedConstructionContinuation
            | Self::RecursiveLearnerReturnBearingContinuation
            | Self::RecursiveLearnerCausalOriginProductComposition
            | Self::RecursiveLearnerCausalPathProductComposition
            | Self::RecursiveLearnerCausalTopologyOutputComposition
            | Self::RecursiveLearnerCausalTopologyOpportunityComposition
            | Self::RecursiveLearnerCausalTopologyProductComposition => Bindings {
                start,
                links_meet: Body::meet_links,
                choose: Body::choose_sensorimotor_candidate,
                outcome_returns: Body::outcomes_return,
                strengthen: Body::strengthen_candidate_outcomes,
                fire_junction: Body::fire,
                form_paths: Body::form_from_participation,
                fire_output: Body::fire_output_from,
                hold,
                finish,
            },
        }
    }
}

fn start(body: &mut Body) -> RunState {
    body.output_wave_open = body.arena.output_junctions.iter().any(|junction| *junction);
    let mut cost = ExecutionCost::default();
    cost.observe_memory_bytes(body.working_bytes());
    RunState {
        outputs: Vec::new(),
        work: Work::default(),
        cost,
        trace: Vec::new(),
    }
}

fn hold(body: &mut Body, run: &mut RunState) {
    run.cost.observe_memory_bytes(body.working_bytes());
}

fn finish(body: &mut Body, mut run: RunState) -> RunResult {
    if body.pending.is_empty() {
        body.finish_output_wave(&mut run.work, &mut run.trace);
    }
    RunResult {
        outputs: run.outputs,
        work: run.work,
        naturally_quiescent: body.pending.is_empty(),
        memory_bytes: body.arena.memory_bytes(),
        execution_cost: run.cost,
        physical_trace: run.trace,
    }
}
