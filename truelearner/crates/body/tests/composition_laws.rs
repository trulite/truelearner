#![deny(warnings)]

use truelearner_body::{
    harness::{attach_outcome_component, attach_sensor, effect, finish, motor, reading, schedule},
    verify_choice_contract, Arrival, Body, Junction, PhysicalEvent, ReturnDecision, TraceEvent,
};

struct TwoStepWorld {
    body: Body,
    start: truelearner_body::JunctionId,
    intermediate: truelearner_body::JunctionId,
    final_outcome: truelearner_body::JunctionId,
    motors: [truelearner_body::harness::Motor; 2],
    intermediate_value: i32,
    final_value: i32,
}

impl TwoStepWorld {
    fn new(reverse_motor_construction: bool) -> Self {
        let mut body = Body::default();
        let motors = if reverse_motor_construction {
            let second = motor(&mut body);
            let first = motor(&mut body);
            [first, second]
        } else {
            [motor(&mut body), motor(&mut body)]
        };
        let start = attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motors[0].opportunity, 1)],
        );
        let intermediate = attach_sensor(
            &mut body,
            Junction::sampled(1_000),
            &[(motors[1].opportunity, 1)],
        );
        let final_outcome = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        attach_outcome_component(&mut body, intermediate, [motors[0].opportunity]);
        attach_outcome_component(&mut body, final_outcome, [motors[1].opportunity]);
        schedule(
            &mut body,
            0,
            &[
                Arrival::new(intermediate, 0),
                Arrival::new(final_outcome, 0),
            ],
        );
        finish(&mut body);
        Self {
            body,
            start,
            intermediate,
            final_outcome,
            motors,
            intermediate_value: 0,
            final_value: 0,
        }
    }

    fn run_traced(&mut self) -> (Vec<PhysicalEvent>, Vec<TraceEvent>) {
        let mut events = Vec::new();
        let mut trace = Vec::new();
        self.body
            .run_traced(256, |event| events.push(event), |event| trace.push(event))
            .unwrap();
        assert!(self.body.is_quiet());
        verify_choice_contract(&trace).unwrap();
        (events, trace)
    }

    fn teach_first_step(&mut self, at: u64) {
        schedule(&mut self.body, at, &[reading(self.start, 0, 1)]);
        schedule(
            &mut self.body,
            at + 1,
            &[Arrival::new(self.motors[0].opportunity, 1)],
        );
        let (action, _) = self.run_traced();
        assert_eq!(effect(&action, &self.motors), [0]);

        self.intermediate_value += 1;
        schedule(
            &mut self.body,
            at + 4,
            &[reading(self.intermediate, 0, self.intermediate_value)],
        );
        let (returned, trace) = self.run_traced();
        assert!(effect(&returned, &self.motors).is_empty());
        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.decision == ReturnDecision::Accepted
                    && returned.source == self.intermediate
        )));
    }

    fn teach_second_step(&mut self, at: u64) {
        self.intermediate_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.intermediate, 0, self.intermediate_value)],
        );
        schedule(
            &mut self.body,
            at + 1,
            &[Arrival::new(self.motors[1].opportunity, 1)],
        );
        let (action, _) = self.run_traced();
        assert_eq!(effect(&action, &self.motors), [1]);

        self.final_value += 1;
        schedule(
            &mut self.body,
            at + 4,
            &[reading(self.final_outcome, 0, self.final_value)],
        );
        let (returned, trace) = self.run_traced();
        assert!(effect(&returned, &self.motors).is_empty());
        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.decision == ReturnDecision::Accepted
                    && returned.source == self.final_outcome
        )));
    }

    fn start_probe(&mut self, at: u64) -> Vec<PhysicalEvent> {
        schedule(&mut self.body, at, &[reading(self.start, 0, 1)]);
        self.run_traced().0
    }

    fn return_intermediate(&mut self, at: u64) -> (Vec<PhysicalEvent>, Vec<TraceEvent>) {
        self.intermediate_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.intermediate, 0, self.intermediate_value)],
        );
        self.run_traced()
    }
}

#[test]
fn separately_learned_steps_compose_through_one_physical_intermediate() {
    let mut world = TwoStepWorld::new(false);
    world.teach_first_step(10);
    world.teach_second_step(20);

    let first = world.start_probe(30);
    assert_eq!(effect(&first, &world.motors), [0]);

    let (second, trace) = world.return_intermediate(34);
    assert_eq!(effect(&second, &world.motors), [1]);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned)
            if returned.decision == ReturnDecision::Accepted
                && returned.source == world.intermediate
    )));
}

#[test]
fn an_untrained_second_step_is_not_invented() {
    let mut world = TwoStepWorld::new(false);
    world.teach_first_step(10);

    let first = world.start_probe(20);
    assert_eq!(effect(&first, &world.motors), [0]);
    let (second, _) = world.return_intermediate(24);
    assert!(effect(&second, &world.motors).is_empty());
}

#[test]
fn composition_is_independent_of_motor_construction_order() {
    let composed_effects = |reverse_motor_construction| {
        let mut world = TwoStepWorld::new(reverse_motor_construction);
        world.teach_first_step(10);
        world.teach_second_step(20);
        let first = effect(&world.start_probe(30), &world.motors);
        let second = effect(&world.return_intermediate(34).0, &world.motors);
        (first, second)
    };

    assert_eq!(composed_effects(false), composed_effects(true));
}

#[test]
fn automatic_internal_paths_do_not_skip_the_real_world_intermediate() {
    let mut world = TwoStepWorld::new(false);
    for at in [10, 20, 30] {
        world.teach_first_step(at);
    }
    for at in [40, 50, 60] {
        world.teach_second_step(at);
    }

    let first = world.start_probe(70);
    assert_eq!(effect(&first, &world.motors), [0]);
    assert_eq!(
        first
            .iter()
            .filter(|event| event.junction == world.intermediate)
            .count(),
        0,
        "an internal composite must not invent its world return"
    );

    let (second, trace) = world.return_intermediate(74);
    assert_eq!(effect(&second, &world.motors), [1]);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned)
            if returned.decision == ReturnDecision::Accepted
                && returned.source == world.intermediate
    )));
}
