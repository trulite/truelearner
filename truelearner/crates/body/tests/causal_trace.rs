use truelearner_body::{
    harness::{attach_outcome_component, attach_sensor, finish, motor, reading, schedule},
    verify_choice_contract, Arrival, Body, Junction, ReturnDecision, TraceEvent,
};

struct World {
    body: Body,
    sensor: truelearner_body::JunctionId,
    outcome: truelearner_body::JunctionId,
    opportunity: truelearner_body::JunctionId,
    effect: truelearner_body::JunctionId,
}

impl World {
    fn new() -> Self {
        let mut body = Body::default();
        let motor = motor(&mut body);
        let sensor = attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        );
        let outcome = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        schedule(&mut body, 0, &[Arrival::new(outcome, 0)]);
        finish(&mut body);
        Self {
            body,
            sensor,
            outcome,
            opportunity: motor.opportunity,
            effect: motor.effect,
        }
    }

    fn act(&mut self, at: u64) -> (truelearner_body::Run, Vec<TraceEvent>) {
        schedule(&mut self.body, at, &[reading(self.sensor, 0, 1)]);
        schedule(&mut self.body, at + 1, &[Arrival::new(self.opportunity, 1)]);
        let mut trace = Vec::new();
        let run = self
            .body
            .run_traced(256, |_| {}, |event| trace.push(event))
            .unwrap();
        (run, trace)
    }
}

#[test]
fn trace_keeps_the_whole_choice_no_effect_and_learning_chain() {
    let mut world = World::new();
    let (_, action) = world.act(10);

    verify_choice_contract(&action).unwrap();

    let candidates = action
        .iter()
        .filter(|event| matches!(event, TraceEvent::Candidate(_)))
        .count();
    assert!(candidates >= 2, "the losing path must remain visible");
    assert!(action.iter().any(|event| matches!(
        event,
        TraceEvent::Choice(choice)
            if choice.alternatives >= 2 && choice.winner.is_some()
    )));
    assert!(action.iter().any(|event| matches!(
        event,
        TraceEvent::Transition(change) if change.junction == world.effect
    )));

    schedule(&mut world.body, 20, &[Arrival::new(world.outcome, 0)]);
    let mut no_effect = Vec::new();
    world
        .body
        .run_traced(256, |_| {}, |event| no_effect.push(event))
        .unwrap();
    assert!(no_effect.iter().any(|event| matches!(
        event,
        TraceEvent::Arrival(arrival) if arrival.target == world.outcome
    )));
    assert!(!no_effect.iter().any(|event| matches!(
        event,
        TraceEvent::Transition(change) if change.junction == world.outcome
    )));
    assert!(!no_effect
        .iter()
        .any(|event| matches!(event, TraceEvent::Return(_) | TraceEvent::Strengthened(_))));

    schedule(&mut world.body, 30, &[Arrival::new(world.outcome, 1)]);
    let mut learned = Vec::new();
    world
        .body
        .run_traced(256, |_| {}, |event| learned.push(event))
        .unwrap();
    assert!(learned.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
    )));
    assert_eq!(
        learned
            .iter()
            .filter(|event| matches!(event, TraceEvent::Strengthened(_)))
            .count(),
        2
    );
}

#[test]
fn trace_records_a_real_rejected_return_reason() {
    let mut body = Body::default();
    let motor = motor(&mut body);
    let action = attach_sensor(
        &mut body,
        Junction::integrating(1),
        &[(motor.opportunity, 1)],
    );
    let outcome = attach_sensor(
        &mut body,
        Junction::sampled(1_000),
        &[(motor.opportunity, 1)],
    );
    attach_outcome_component(&mut body, outcome, [motor.opportunity]);
    schedule(&mut body, 0, &[Arrival::new(outcome, 0)]);
    finish(&mut body);
    schedule(&mut body, 10, &[reading(action, 0, 1)]);
    schedule(&mut body, 11, &[Arrival::new(motor.opportunity, 1)]);
    finish(&mut body);

    schedule(&mut body, 20, &[Arrival::new(outcome, 1)]);
    body.run_traced(256, |_| {}, |_| {}).unwrap();
    schedule(&mut body, 30, &[Arrival::new(outcome, 2)]);
    let mut rejected = Vec::new();
    body.run_traced(256, |_| {}, |event| rejected.push(event))
        .unwrap();

    assert!(
        rejected.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.decision == ReturnDecision::BlockedByCandidatePath
        )),
        "{rejected:#?}"
    );
}

#[test]
fn a_return_matches_only_the_component_that_returned_it() {
    let mut body = Body::default();
    let motors = [motor(&mut body), motor(&mut body)];
    let sensors = motors.map(|motor| {
        attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        )
    });
    let outcomes = [
        attach_sensor(&mut body, Junction::sampled(1_000), &[]),
        attach_sensor(&mut body, Junction::sampled(1_000), &[]),
    ];
    for index in 0..2 {
        attach_outcome_component(&mut body, outcomes[index], [motors[index].opportunity]);
    }
    schedule(
        &mut body,
        0,
        &outcomes.map(|outcome| Arrival::new(outcome, 0)),
    );
    finish(&mut body);

    schedule(
        &mut body,
        10,
        &sensors.map(|sensor| Arrival::new(sensor, 1)),
    );
    schedule(
        &mut body,
        11,
        &motors.map(|motor| Arrival::new(motor.opportunity, 1)),
    );
    finish(&mut body);

    for (at, outcome) in outcomes.into_iter().enumerate() {
        schedule(&mut body, 20 + at as u64, &[Arrival::new(outcome, 1)]);
        let mut trace = Vec::new();
        body.run_traced(256, |_| {}, |event| trace.push(event))
            .unwrap();

        assert!(
            trace.iter().any(|event| matches!(
                event,
                TraceEvent::Return(returned)
                    if returned.source == outcome
                        && returned.decision == ReturnDecision::Accepted
            )),
            "{trace:#?}"
        );
        assert_eq!(
            trace
                .iter()
                .filter(|event| matches!(event, TraceEvent::Strengthened(_)))
                .count(),
            2,
            "{trace:#?}"
        );
    }
}

#[test]
fn traced_run_is_the_same_body_arrow_as_untraced_run() {
    let mut plain = World::new();
    let mut traced = World {
        body: plain.body.clone(),
        sensor: plain.sensor,
        outcome: plain.outcome,
        opportunity: plain.opportunity,
        effect: plain.effect,
    };
    for world in [&mut plain, &mut traced] {
        schedule(&mut world.body, 10, &[reading(world.sensor, 0, 1)]);
        schedule(&mut world.body, 11, &[Arrival::new(world.opportunity, 1)]);
    }

    let mut plain_events = Vec::new();
    let plain_run = plain
        .body
        .run(256, |event| plain_events.push(event))
        .unwrap();
    let mut traced_events = Vec::new();
    let mut trace = Vec::new();
    let traced_run = traced
        .body
        .run_traced(
            256,
            |event| traced_events.push(event),
            |event| trace.push(event),
        )
        .unwrap();

    assert_eq!(plain_run, traced_run);
    assert_eq!(plain_events, traced_events);
    assert_eq!(format!("{:?}", plain.body), format!("{:?}", traced.body));
    assert!(matches!(trace.last(), Some(TraceEvent::Quiet(_))));

    for world in [&mut plain, &mut traced] {
        schedule(&mut world.body, 20, &[reading(world.sensor, 0, 1)]);
        schedule(&mut world.body, 21, &[Arrival::new(world.opportunity, 1)]);
    }
    let mut plain_next = Vec::new();
    let mut traced_next = Vec::new();
    let plain_run = plain.body.run(256, |event| plain_next.push(event)).unwrap();
    let traced_run = traced
        .body
        .run_traced(256, |event| traced_next.push(event), |_| {})
        .unwrap();
    assert_eq!(plain_run, traced_run);
    assert_eq!(plain_next, traced_next);
    assert_eq!(format!("{:?}", plain.body), format!("{:?}", traced.body));
}
