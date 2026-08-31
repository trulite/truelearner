//! Unit-test ladder for the physical prerequisites of planning and goal discovery.
//!
//! The active tests name laws the body already supports. Ignored tests freeze the
//! next unsupported transitions without adding a planner, goal object, reward,
//! episode verdict, or evaluator state to the learner.

use crate::{
    harness::{attach_outcome_component, attach_sensor, effect, motor, reading, schedule, Motor},
    verify_choice_laws, Arrival, Body, Junction, JunctionId, PhysicalEvent, ReturnDecision,
    TraceEvent,
};

fn run(body: &mut Body) -> (Vec<PhysicalEvent>, Vec<TraceEvent>) {
    let mut events = Vec::new();
    let mut trace = Vec::new();
    body.run_traced(256, |event| events.push(event), |event| trace.push(event))
        .unwrap();
    assert!(body.is_quiet());
    verify_choice_laws(&trace).unwrap();
    (events, trace)
}

struct PlanningWorld {
    body: Body,
    starts: [JunctionId; 2],
    route: JunctionId,
    dead_end: JunctionId,
    closure: JunctionId,
    motors: [Motor; 3],
    route_value: i32,
    dead_end_value: i32,
    closure_value: i32,
    condition_open: bool,
}

impl PlanningWorld {
    const ROUTE: usize = 0;
    const DEAD_END: usize = 1;
    const TERMINAL: usize = 2;
    const ROUTE_START: usize = 0;
    const SHARED_START: usize = 1;

    fn new() -> Self {
        let mut body = Body::default();
        let motors = std::array::from_fn(|_| motor(&mut body));
        let starts = [
            attach_sensor(
                &mut body,
                Junction::integrating(1),
                &[(motors[Self::ROUTE].opportunity, 1)],
            ),
            attach_sensor(
                &mut body,
                Junction::integrating(1),
                &[
                    (motors[Self::ROUTE].opportunity, 1),
                    (motors[Self::DEAD_END].opportunity, 1),
                ],
            ),
        ];
        let route = attach_sensor(
            &mut body,
            Junction::sampled(1_000),
            &[(motors[Self::TERMINAL].opportunity, 1)],
        );
        let dead_end = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        let closure = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        attach_outcome_component(&mut body, route, [motors[Self::ROUTE].opportunity]);
        attach_outcome_component(&mut body, dead_end, [motors[Self::DEAD_END].opportunity]);
        attach_outcome_component(&mut body, closure, [motors[Self::TERMINAL].opportunity]);
        schedule(
            &mut body,
            0,
            &[
                Arrival::caused(route, 0, 0),
                Arrival::caused(dead_end, 0, 0),
                Arrival::caused(closure, 0, 0),
            ],
        );
        run(&mut body);
        Self {
            body,
            starts,
            route,
            dead_end,
            closure,
            motors,
            route_value: 0,
            dead_end_value: 0,
            closure_value: 0,
            condition_open: false,
        }
    }

    fn teach_start(&mut self, start: usize, output: usize, at: u64, cause: u64) {
        schedule(
            &mut self.body,
            at,
            &[reading(self.starts[start], 0, 1, cause)],
        );
        schedule(
            &mut self.body,
            at + 1,
            &[Arrival::caused(self.motors[output].opportunity, 1, cause)],
        );
        assert_eq!(effect(&run(&mut self.body).0, &self.motors), [output]);
    }

    fn return_route(&mut self, at: u64, cause: u64) -> Vec<PhysicalEvent> {
        self.route_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.route, 0, self.route_value, cause)],
        );
        run(&mut self.body).0
    }

    fn return_dead_end(&mut self, at: u64, cause: u64) {
        assert!(self.condition_open);
        self.dead_end_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.dead_end, 0, self.dead_end_value, cause)],
        );
        let (_, trace) = run(&mut self.body);
        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.decision == ReturnDecision::Accepted
                    && returned.source == self.dead_end
                    && returned.return_cause == Some(cause)
        )));
        assert!(self.condition_open);
    }

    fn return_closure(&mut self, at: u64, cause: u64) {
        assert!(self.condition_open);
        self.closure_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.closure, 0, self.closure_value, cause)],
        );
        let (_, trace) = run(&mut self.body);
        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.decision == ReturnDecision::Accepted
                    && returned.source == self.closure
                    && returned.return_cause == Some(cause)
        )));
        self.condition_open = false;
    }

    fn open_condition(&mut self, at: u64, cause: u64) {
        self.closure_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.closure, 0, self.closure_value, cause)],
        );
        let (events, trace) = run(&mut self.body);
        assert!(effect(&events, &self.motors).is_empty());
        assert!(!trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
        )));
        self.condition_open = true;
    }

    fn teach_route(&mut self, at: u64, cause: u64) {
        self.teach_start(Self::ROUTE_START, Self::ROUTE, at, cause);
        assert!(effect(&self.return_route(at + 4, cause), &self.motors).is_empty());
    }

    fn teach_terminal(&mut self, at: u64, cause: u64) {
        self.route_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.route, 0, self.route_value, cause)],
        );
        schedule(
            &mut self.body,
            at + 1,
            &[Arrival::caused(
                self.motors[Self::TERMINAL].opportunity,
                1,
                cause,
            )],
        );
        assert_eq!(
            effect(&run(&mut self.body).0, &self.motors),
            [Self::TERMINAL]
        );
        self.return_closure(at + 4, cause);
    }

    fn probe_shared(&mut self, at: u64, cause: u64) -> (Vec<PhysicalEvent>, Vec<TraceEvent>) {
        schedule(
            &mut self.body,
            at,
            &[reading(self.starts[Self::SHARED_START], 0, 1, cause)],
        );
        schedule(
            &mut self.body,
            at + 1,
            &[
                Arrival::caused(self.motors[Self::ROUTE].opportunity, 1, cause),
                Arrival::caused(self.motors[Self::DEAD_END].opportunity, 1, cause),
            ],
        );
        run(&mut self.body)
    }

    fn probe_learned_route(&mut self, at: u64, cause: u64) -> Vec<PhysicalEvent> {
        schedule(
            &mut self.body,
            at,
            &[reading(self.starts[Self::ROUTE_START], 0, 1, cause)],
        );
        run(&mut self.body).0
    }
}

#[test]
fn separately_closed_steps_compose_only_after_the_real_intermediate_returns() {
    let mut world = PlanningWorld::new();
    world.open_condition(5, 90);
    world.teach_route(10, 1);
    world.teach_terminal(20, 2);

    let first = world.probe_learned_route(30, 3);
    assert_eq!(effect(&first, &world.motors), [PlanningWorld::ROUTE]);

    let terminal = world.return_route(34, 3);
    assert_eq!(effect(&terminal, &world.motors), [PlanningWorld::TERMINAL]);
}

#[test]
#[ignore = "frontier: an open condition does not yet select its learned multi-step route"]
fn an_open_condition_selects_the_learned_route_that_can_close_it() {
    let mut world = PlanningWorld::new();
    world.open_condition(5, 90);

    let (route, _) = world.probe_shared(10, 1);
    assert_eq!(effect(&route, &world.motors), [PlanningWorld::ROUTE]);
    assert!(effect(&world.return_route(14, 1), &world.motors).is_empty());
    world.teach_terminal(20, 2);

    world.open_condition(30, 91);
    let (dead_end, _) = world.probe_shared(32, 3);
    assert_eq!(effect(&dead_end, &world.motors), [PlanningWorld::DEAD_END]);
    world.return_dead_end(36, 3);

    world.open_condition(39, 92);
    let (first, trace) = world.probe_shared(40, 4);
    assert_eq!(
        effect(&first, &world.motors),
        [PlanningWorld::ROUTE],
        "choices={:#?}",
        trace
            .iter()
            .filter(|event| matches!(event, TraceEvent::Choice(_)))
            .collect::<Vec<_>>()
    );

    let terminal = world.return_route(44, 4);
    assert_eq!(effect(&terminal, &world.motors), [PlanningWorld::TERMINAL]);
    world.return_closure(48, 4);
    assert!(!world.condition_open);
}

struct ClosureWorld {
    body: Body,
    surfaces: [JunctionId; 2],
    motors: [Motor; 4],
    closure: JunctionId,
    closure_value: i32,
}

impl ClosureWorld {
    const OLD_PROXY: usize = 0;
    const OLD_CLOSER: usize = 1;
    const NEW_PROXY: usize = 2;
    const NEW_CLOSER: usize = 3;

    fn new() -> Self {
        let mut body = Body::default();
        let motors = std::array::from_fn(|_| motor(&mut body));
        let surfaces = [
            attach_sensor(
                &mut body,
                Junction::integrating(1),
                &[
                    (motors[Self::OLD_PROXY].opportunity, 1),
                    (motors[Self::OLD_CLOSER].opportunity, 1),
                ],
            ),
            attach_sensor(
                &mut body,
                Junction::integrating(1),
                &[
                    (motors[Self::NEW_PROXY].opportunity, 1),
                    (motors[Self::NEW_CLOSER].opportunity, 1),
                ],
            ),
        ];
        let closure = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        attach_outcome_component(
            &mut body,
            closure,
            [
                motors[Self::OLD_CLOSER].opportunity,
                motors[Self::NEW_CLOSER].opportunity,
            ],
        );
        schedule(&mut body, 0, &[Arrival::caused(closure, 0, 0)]);
        run(&mut body);
        Self {
            body,
            surfaces,
            motors,
            closure,
            closure_value: 0,
        }
    }

    fn act(&mut self, surface: usize, outputs: &[usize], at: u64, cause: u64) -> Vec<usize> {
        schedule(
            &mut self.body,
            at,
            &[reading(self.surfaces[surface], 0, 1, cause)],
        );
        schedule(
            &mut self.body,
            at + 1,
            &outputs
                .iter()
                .map(|output| Arrival::caused(self.motors[*output].opportunity, 1, cause))
                .collect::<Vec<_>>(),
        );
        effect(&run(&mut self.body).0, &self.motors)
    }

    fn return_closure(&mut self, at: u64, cause: u64) {
        self.closure_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.closure, 0, self.closure_value, cause)],
        );
        let (_, trace) = run(&mut self.body);
        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.decision == ReturnDecision::Accepted
                    && returned.source == self.closure
                    && returned.return_cause == Some(cause)
        )));
    }

    fn discover_old_closure(&mut self) {
        assert_eq!(self.act(0, &[Self::OLD_PROXY], 10, 1), [Self::OLD_PROXY]);
        assert_eq!(self.act(0, &[Self::OLD_CLOSER], 20, 2), [Self::OLD_CLOSER]);
        self.return_closure(22, 2);
    }
}

#[test]
fn one_unique_self_caused_closure_changes_later_choice_on_the_same_surface() {
    let mut world = ClosureWorld::new();
    world.discover_old_closure();

    assert_eq!(
        world.act(
            0,
            &[ClosureWorld::OLD_PROXY, ClosureWorld::OLD_CLOSER],
            30,
            3,
        ),
        [ClosureWorld::OLD_CLOSER]
    );
}

#[test]
fn a_passive_closure_sample_creates_no_later_action_preference() {
    let mut world = ClosureWorld::new();
    world.closure_value += 1;
    schedule(
        &mut world.body,
        10,
        &[reading(world.closure, 0, world.closure_value, 9)],
    );
    run(&mut world.body);

    assert_eq!(
        world.act(
            0,
            &[ClosureWorld::OLD_PROXY, ClosureWorld::OLD_CLOSER],
            20,
            1,
        ),
        [ClosureWorld::OLD_PROXY]
    );
}

#[test]
#[ignore = "frontier: a closure class does not yet transfer to fresh surface and output identities"]
fn a_discovered_closure_transfers_to_a_fresh_equivalent_surface_and_output() {
    let mut world = ClosureWorld::new();
    world.discover_old_closure();

    assert_eq!(
        world.act(
            1,
            &[ClosureWorld::NEW_PROXY, ClosureWorld::NEW_CLOSER],
            30,
            3,
        ),
        [ClosureWorld::NEW_CLOSER]
    );
}
