use crate::prelude::*;

impl Body {
    pub fn propagate(&mut self) -> RunResult {
        let protocol = self.protocol.bindings();
        crate::core::run(self, protocol)
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
