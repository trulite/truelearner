use truelearner_body::{
    attach, harness::*, verify_choice_contract, Arrival, Body, ChoiceWarrant, Join, Junction,
    JunctionId, Link, OpenBody, TraceEvent, Work,
};

struct LocalWorld {
    body: Body,
    surface: JunctionId,
    motors: Vec<Motor>,
}

impl LocalWorld {
    fn new(motor_distances: &[u64]) -> Self {
        let mut body = Body::default();
        let motors = motor_distances
            .iter()
            .map(|_| motor(&mut body))
            .collect::<Vec<_>>();
        let nearby_outputs = motors
            .iter()
            .zip(motor_distances)
            .map(|(motor, distance)| (motor.opportunity, *distance))
            .collect::<Vec<_>>();
        let surface = attach_sensor(&mut body, Junction::integrating(1), &nearby_outputs);
        Self {
            body,
            surface,
            motors,
        }
    }

    fn act(&mut self, at: u64) -> Trace {
        schedule(&mut self.body, at, &[reading(self.surface, 0, 1)]);
        let opportunities = self
            .motors
            .iter()
            .map(|motor| Arrival::new(motor.opportunity, 1))
            .collect::<Vec<_>>();
        schedule(&mut self.body, at + 1, &opportunities);
        finish(&mut self.body)
    }
}

struct CompetitionWorld {
    body: Body,
    surfaces: [JunctionId; 2],
    motors: [Motor; 2],
    consequence: JunctionId,
    consequence_value: i32,
}

impl CompetitionWorld {
    fn new(reverse_construction: bool) -> Self {
        let mut body = Body::default();
        let component = |body: &mut Body| {
            let motor = motor(body);
            let surface = attach_sensor(body, Junction::integrating(1), &[(motor.opportunity, 1)]);
            (surface, motor)
        };
        let (surfaces, motors) = if reverse_construction {
            let (right_surface, right_motor) = component(&mut body);
            let (left_surface, left_motor) = component(&mut body);
            ([left_surface, right_surface], [left_motor, right_motor])
        } else {
            let (left_surface, left_motor) = component(&mut body);
            let (right_surface, right_motor) = component(&mut body);
            ([left_surface, right_surface], [left_motor, right_motor])
        };
        let consequence = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        attach_outcome_component(
            &mut body,
            consequence,
            motors.map(|motor| motor.opportunity),
        );
        schedule(&mut body, 0, &[reading(consequence, 0, 0)]);
        finish(&mut body);
        Self {
            body,
            surfaces,
            motors,
            consequence,
            consequence_value: 0,
        }
    }

    fn act(&mut self, which: usize, at: u64) -> Trace {
        schedule(&mut self.body, at, &[reading(self.surfaces[which], 0, 1)]);
        schedule(
            &mut self.body,
            at + 1,
            &[Arrival::new(self.motors[which].opportunity, 1)],
        );
        finish(&mut self.body)
    }

    fn compete(&mut self, at: u64) -> Trace {
        self.compete_traced(at).0
    }

    fn compete_traced(&mut self, at: u64) -> (Trace, Vec<TraceEvent>) {
        schedule(
            &mut self.body,
            at,
            &self.surfaces.map(|surface| reading(surface, 0, 1)),
        );
        schedule(
            &mut self.body,
            at + 1,
            &self.motors.map(|motor| Arrival::new(motor.opportunity, 1)),
        );
        let mut events = Vec::new();
        let mut trace = Vec::new();
        let run = self
            .body
            .run_traced(256, |event| events.push(event), |event| trace.push(event))
            .unwrap();
        assert!(self.body.is_quiet());
        (Trace { run, events }, trace)
    }

    fn return_change(&mut self, at: u64) -> Trace {
        self.consequence_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.consequence, 0, self.consequence_value)],
        );
        finish(&mut self.body)
    }

    fn return_repeat(&mut self, at: u64) -> Trace {
        schedule(
            &mut self.body,
            at,
            &[reading(self.consequence, 0, self.consequence_value)],
        );
        finish(&mut self.body)
    }

    fn completed_cycle(&mut self, which: usize, at: u64) {
        let action = self.act(which, at);
        assert_eq!(effect(&action.events, &self.motors), [which]);
        self.return_change(at + 2);
    }
}

struct LearnerWorld {
    body: Body,
    members: [JunctionId; 3],
    motor: Motor,
    consequence: JunctionId,
    consequence_value: i32,
}

impl LearnerWorld {
    fn new() -> Self {
        let mut body = Body::default();
        let motor = motor(&mut body);
        let members = [
            attach_sensor(
                &mut body,
                Junction::integrating(1),
                &[(motor.opportunity, 1)],
            ),
            attach_sensor(&mut body, Junction::integrating(1), &[]),
            attach_sensor(&mut body, Junction::integrating(1), &[]),
        ];
        let consequence = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        attach_outcome_component(&mut body, consequence, [motor.opportunity]);
        schedule(&mut body, 0, &[reading(consequence, 0, 0)]);
        finish(&mut body);
        Self {
            body,
            members,
            motor,
            consequence,
            consequence_value: 0,
        }
    }

    fn close(&mut self, member_count: usize, at: u64, live_change: bool) -> Trace {
        let arrivals = self.members[..member_count]
            .iter()
            .map(|member| reading(*member, 0, 1))
            .collect::<Vec<_>>();
        schedule(&mut self.body, at, &arrivals);
        if live_change {
            self.consequence_value += 1;
            schedule(
                &mut self.body,
                at,
                &[reading(self.consequence, 0, self.consequence_value)],
            );
        }
        finish(&mut self.body)
    }

    fn probe(&mut self, at: u64) -> Trace {
        schedule(&mut self.body, at, &[reading(self.members[0], 0, 1)]);
        schedule(
            &mut self.body,
            at + 1,
            &[Arrival::new(self.motor.opportunity, 1)],
        );
        finish(&mut self.body)
    }
}

#[test]
fn quiet_is_identity_and_run_is_repeated_step() {
    let mut body = Body::default();
    assert_eq!(body.step(|_| {}).unwrap(), None);

    let source = integrating(&mut body, 1);
    let output = integrating(&mut body, 1);
    body.add_link(Link::new(source, output, 2, 1)).unwrap();
    schedule(&mut body, 4, &[Arrival::new(source, 1)]);
    let mut stepped = body.clone();

    let run = finish(&mut body);
    let mut step_events = Vec::new();
    while stepped
        .step(|event| step_events.push(event))
        .unwrap()
        .is_some()
    {}

    assert_eq!(run.events, step_events);
    assert!(body.is_quiet() && stepped.is_quiet());
}

#[test]
fn equal_time_arrivals_meet_independently_of_input_order() {
    fn episode(impulses: [i32; 3]) -> Vec<(i64, i32, i32)> {
        let mut body = Body::default();
        let junction = integrating(&mut body, i32::MAX);
        schedule(
            &mut body,
            0,
            &impulses.map(|impulse| Arrival::new(junction, impulse)),
        );
        finish(&mut body)
            .events
            .into_iter()
            .map(|event| (event.impulse, event.before, event.after))
            .collect()
    }

    let expected = vec![(i64::from(i32::MAX), 0, i32::MAX)];
    assert_eq!(episode([i32::MAX, 1, -1]), expected);
    assert_eq!(episode([-1, 1, i32::MAX]), expected);
}

#[test]
fn repeated_sensor_sample_is_identity_and_real_change_fires() {
    let mut body = Body::default();
    let sensor = attach_sensor(&mut body, Junction::sampled(10), &[]);
    for (at, value) in [(0, 7), (1, 7), (2, 9)] {
        schedule(&mut body, at, &[reading(sensor, 0, value)]);
    }

    let trace = finish(&mut body);
    assert_eq!(trace.events.len(), 1);
    assert_eq!(trace.events[0].junction, sensor);
    assert_eq!((trace.events[0].before, trace.events[0].after), (7, 9));
}

#[test]
fn expired_sensor_memory_does_not_invent_a_transition() {
    let mut body = Body::default();
    let sensor = attach_sensor(&mut body, Junction::sampled(4), &[]);
    schedule(&mut body, 1, &[reading(sensor, 0, 7)]);
    schedule(&mut body, 5, &[reading(sensor, 0, 9)]);

    assert!(finish(&mut body).events.is_empty());
}

#[test]
fn a_surface_without_a_nearby_output_changes_no_later_action() {
    let mut world = LocalWorld::new(&[]);
    let first = world.act(10);
    let second = world.act(20);

    assert!(effect(&first.events, &world.motors).is_empty());
    assert!(effect(&second.events, &world.motors).is_empty());
}

#[test]
fn a_local_surface_forms_one_reusable_choice_without_duplicate_growth() {
    let mut world = LocalWorld::new(&[2]);
    let first = world.act(10);
    let second = world.act(20);
    let third = world.act(30);

    assert_eq!(effect(&first.events, &world.motors), [0]);
    assert_eq!(effect(&second.events, &world.motors), [0]);
    assert_eq!(effect(&third.events, &world.motors), [0]);
    assert_eq!(second.run.work, third.run.work);
}

#[test]
fn formation_is_local_and_does_not_cross_distance_three() {
    let mut world = LocalWorld::new(&[3]);
    let trace = world.act(10);

    assert!(effect(&trace.events, &world.motors).is_empty());
}

#[test]
fn an_outward_effect_does_not_form_a_reentry_choice() {
    let mut body = Body::default();
    let returned = integrating(&mut body, 1);
    let motor = motor(&mut body);
    let mut part = Body::default();
    let local_outside = integrating(&mut part, 1);
    let part = OpenBody::new(part, vec![local_outside]).unwrap();
    let port = part.port(0).unwrap();
    let outside = attach(&mut body, part, &[Join::into_host(returned, port, 0, 1)])
        .unwrap()
        .port(port)
        .unwrap();

    schedule(&mut body, 10, &[reading(outside, 0, 1)]);
    schedule(&mut body, 11, &[Arrival::new(motor.opportunity, 1)]);
    let trace = finish(&mut body);

    assert_eq!(event_count(&trace.events, returned), 1);
    assert_eq!(event_count(&trace.events, motor.effect), 0);
}

#[test]
fn one_connected_world_chooses_exactly_one_action() {
    let mut world = LocalWorld::new(&[1, 2]);
    let trace = world.act(10);

    assert_eq!(effect(&trace.events, &world.motors).len(), 1);
}

#[test]
fn latest_available_consequence_precedes_old_strength() {
    let mut world = CompetitionWorld::new(false);
    for at in [10, 20, 30] {
        world.completed_cycle(0, at);
    }
    world.completed_cycle(1, 40);

    let chosen = world.compete(50);
    assert_eq!(effect(&chosen.events, &world.motors), [1]);
}

#[test]
fn ambiguous_current_returns_do_not_create_a_false_preference() {
    let mut world = CompetitionWorld::new(false);
    for at in [10, 20, 30] {
        world.completed_cycle(0, at);
    }
    world.act(0, 40);
    world.act(1, 42);

    let chosen = world.compete(50);
    assert_eq!(effect(&chosen.events, &world.motors), [0]);
}

#[test]
fn an_unanswered_action_gives_an_alternative_the_next_chance() {
    let mut world = CompetitionWorld::new(false);

    let first = world.act(0, 10);
    assert_eq!(effect(&first.events, &world.motors), [0]);

    let (next, trace) = world.compete_traced(20);
    assert_eq!(effect(&next.events, &world.motors), [1]);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Choice(choice) if choice.warrant == Some(ChoiceWarrant::Exploration)
    )));
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Candidate(candidate)
            if candidate.path.output == world.motors[0].opportunity && candidate.unanswered
    )));
}

#[test]
fn a_previously_successful_action_without_a_new_return_releases_to_an_alternative() {
    let mut world = CompetitionWorld::new(false);
    world.completed_cycle(0, 10);
    assert_eq!(effect(&world.act(1, 20).events, &world.motors), [1]);

    let (first, first_trace) = world.compete_traced(30);
    assert_eq!(effect(&first.events, &world.motors), [0]);
    assert!(first_trace.iter().any(|event| matches!(
        event,
        TraceEvent::Choice(choice) if choice.warrant == Some(ChoiceWarrant::RetainedContinuation)
    )));

    let (next, trace) = world.compete_traced(40);
    let choices = trace
        .iter()
        .filter_map(|event| match event {
            TraceEvent::Choice(choice) => Some(choice),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        effect(&next.events, &world.motors),
        [1],
        "first={:#?}\nnext={choices:#?}",
        first_trace
            .iter()
            .filter_map(|event| match event {
                TraceEvent::Candidate(candidate) => Some(candidate),
                _ => None,
            })
            .collect::<Vec<_>>()
    );
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Choice(choice)
            if choice.warrant == Some(ChoiceWarrant::Exploration)
    )));
    verify_choice_contract(&first_trace).unwrap();
    verify_choice_contract(&trace).unwrap();
}

#[test]
fn a_completed_action_gives_an_untried_alternative_the_next_chance() {
    let mut world = CompetitionWorld::new(false);
    world.completed_cycle(0, 10);

    let (next, trace) = world.compete_traced(20);
    assert_eq!(effect(&next.events, &world.motors), [1]);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Choice(choice) if choice.warrant == Some(ChoiceWarrant::Exploration)
    )));
}

#[test]
fn a_fresh_return_preserves_the_acted_output_for_untried_release() {
    let mut body = Body::default();
    let motors: [Motor; 3] = std::array::from_fn(|_| motor(&mut body));
    let surface = attach_sensor(
        &mut body,
        Junction::integrating(1),
        &[(motors[0].opportunity, 1)],
    );
    let local_bridge = attach_sensor(
        &mut body,
        Junction::integrating(1),
        &[(motors[0].opportunity, 1), (motors[1].opportunity, 1)],
    );
    let outcome = attach_sensor(&mut body, Junction::sampled(100), &[]);
    attach_outcome_component(&mut body, outcome, motors.map(|motor| motor.opportunity));
    schedule(&mut body, 0, &[reading(outcome, 0, 0)]);
    finish(&mut body);

    schedule(&mut body, 10, &[reading(surface, 0, 1)]);
    schedule(&mut body, 11, &[Arrival::new(motors[0].opportunity, 1)]);
    assert_eq!(effect(&finish(&mut body).events, &motors), [0]);

    schedule(&mut body, 20, &[reading(surface, 0, 1)]);
    schedule(
        &mut body,
        21,
        &motors.map(|motor| Arrival::new(motor.opportunity, 1)),
    );
    let mut events = Vec::new();
    let mut trace = Vec::new();
    body.run_traced(256, |event| events.push(event), |event| trace.push(event))
        .unwrap();

    assert_eq!(effect(&events, &motors), [1]);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Candidate(candidate)
            if candidate.fresh_opportunity.is_some_and(|fresh| {
                fresh.output == motors[1].opportunity
            })
    )));
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Choice(choice) if choice.warrant == Some(ChoiceWarrant::RetainedContinuation)
    )));
    verify_choice_contract(&trace).unwrap();

    schedule(
        &mut body,
        30,
        &[reading(local_bridge, 0, 1), reading(outcome, 0, 1)],
    );
    schedule(
        &mut body,
        31,
        &motors.map(|motor| Arrival::new(motor.opportunity, 1)),
    );
    let mut events = Vec::new();
    let mut trace = Vec::new();
    body.run_traced(256, |event| events.push(event), |event| trace.push(event))
        .unwrap();

    assert_eq!(effect(&events, &motors), [0]);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Candidate(candidate)
            if candidate.path.output == motors[1].opportunity
                && candidate.output_participated
    )));
    assert!(trace.iter().all(|event| !matches!(
        event,
        TraceEvent::Candidate(candidate)
            if candidate.path.output != motors[1].opportunity
                && candidate.output_participated
    )));
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Choice(choice)
            if choice.warrant == Some(ChoiceWarrant::Exploration)
    )));
    verify_choice_contract(&trace).unwrap();
}

#[test]
fn a_unique_ordinary_return_continues_after_both_outputs_were_tried() {
    let mut world = CompetitionWorld::new(false);
    world.completed_cycle(0, 10);
    world.completed_cycle(1, 20);
    assert_eq!(effect(&world.act(1, 30).events, &world.motors), [1]);

    world.consequence_value += 1;
    schedule(
        &mut world.body,
        40,
        &[
            reading(world.consequence, 0, world.consequence_value),
            reading(world.surfaces[0], 0, 1),
            reading(world.surfaces[1], 0, 1),
        ],
    );
    schedule(
        &mut world.body,
        41,
        &world.motors.map(|motor| Arrival::new(motor.opportunity, 1)),
    );
    let mut events = Vec::new();
    let mut trace = Vec::new();
    world
        .body
        .run_traced(256, |event| events.push(event), |event| trace.push(event))
        .unwrap();

    assert_eq!(effect(&events, &world.motors), [1]);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Candidate(candidate)
            if candidate.path.output == world.motors[1].opportunity
                && candidate.output_participated
    )));
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Choice(choice)
            if choice.warrant == Some(ChoiceWarrant::ReturnedConsequence)
    )));
    verify_choice_contract(&trace).unwrap();
}

#[test]
fn a_fresh_external_opportunity_crosses_only_into_the_root_learner() {
    let mut root = LearnerWorld::new();
    root.close(2, 10, true);
    let root_probe = root.probe(20);

    let mut child = LearnerWorld::new();
    child.close(2, 10, true);
    child.close(3, 20, true);
    let child_probe = child.probe(30);

    assert_eq!(event_count(&root_probe.events, root.motor.effect), 1);
    assert_eq!(event_count(&child_probe.events, child.motor.effect), 0);
}

#[test]
fn disconnected_physical_components_choose_independently() {
    let mut body = Body::default();
    let motors = [motor(&mut body), motor(&mut body)];
    let surfaces = [
        attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motors[0].opportunity, 1)],
        ),
        attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motors[1].opportunity, 1)],
        ),
    ];
    let consequences = [
        attach_sensor(&mut body, Junction::sampled(100), &[]),
        attach_sensor(&mut body, Junction::sampled(100), &[]),
    ];
    for index in 0..2 {
        attach_outcome_component(&mut body, consequences[index], [motors[index].opportunity]);
    }
    schedule(
        &mut body,
        0,
        &consequences.map(|sensor| reading(sensor, 0, 0)),
    );
    finish(&mut body);

    for index in 0..2 {
        schedule(
            &mut body,
            10 + index as u64 * 10,
            &[reading(surfaces[index], 0, 1)],
        );
        schedule(
            &mut body,
            11 + index as u64 * 10,
            &[Arrival::new(motors[index].opportunity, 1)],
        );
        finish(&mut body);
        schedule(
            &mut body,
            12 + index as u64 * 10,
            &[reading(consequences[index], 0, 1)],
        );
        finish(&mut body);
    }

    schedule(
        &mut body,
        40,
        &[reading(surfaces[0], 0, 1), reading(surfaces[1], 0, 1)],
    );
    schedule(
        &mut body,
        41,
        &[
            Arrival::new(motors[0].opportunity, 1),
            Arrival::new(motors[1].opportunity, 1),
        ],
    );
    let trace = finish(&mut body);

    assert_eq!(effect(&trace.events, &motors), [0, 1]);
}

#[test]
fn connected_choice_is_independent_of_construction_order() {
    let mut forward = CompetitionWorld::new(false);
    let mut reverse = CompetitionWorld::new(true);
    for world in [&mut forward, &mut reverse] {
        world.completed_cycle(0, 10);
        world.completed_cycle(1, 20);
    }

    let forward_trace = forward.compete(30);
    let reverse_trace = reverse.compete(30);

    let forward_positions = effect(&forward_trace.events, &forward.motors);
    let reverse_positions = effect(&reverse_trace.events, &reverse.motors);
    assert_eq!(forward_positions, reverse_positions);
}

#[test]
fn a_later_consequence_credits_only_the_action_that_physically_happened() {
    let mut world = CompetitionWorld::new(false);
    world.act(1, 10);
    world.act(0, 20);
    world.return_change(22);

    let chosen = world.compete(30);
    assert_eq!(effect(&chosen.events, &world.motors), [0]);
}

#[test]
fn preopening_and_repeated_samples_close_nothing_and_credit_nothing() {
    let mut before = CompetitionWorld::new(false);
    before.return_change(10);
    before.act(0, 20);

    let mut repeated = CompetitionWorld::new(false);
    repeated.act(0, 10);
    repeated.return_repeat(12);

    assert_eq!(
        effect(&before.compete(40).events, &before.motors),
        effect(&repeated.compete(40).events, &repeated.motors)
    );
}

#[test]
fn one_physical_consequence_has_one_learning_effect() {
    let mut once = CompetitionWorld::new(false);
    once.act(0, 10);
    once.return_change(12);

    let mut repeated = once.body.clone();
    schedule(
        &mut repeated,
        13,
        &[reading(once.consequence, 0, once.consequence_value)],
    );
    finish(&mut repeated);

    let once_trace = once.compete(20);
    schedule(
        &mut repeated,
        20,
        &once.surfaces.map(|surface| reading(surface, 0, 1)),
    );
    schedule(
        &mut repeated,
        21,
        &once.motors.map(|motor| Arrival::new(motor.opportunity, 1)),
    );
    let repeated_trace = finish(&mut repeated);

    assert_eq!(
        physical_trace(&once_trace.events),
        physical_trace(&repeated_trace.events)
    );
}

#[test]
fn a_returned_consequence_is_available_until_one_choice_then_consumed() {
    let mut world = CompetitionWorld::new(false);
    for at in [10, 20, 30] {
        world.completed_cycle(1, at);
    }
    world.completed_cycle(0, 40);

    let first = world.compete(50);
    let second = world.compete(60);
    assert_eq!(effect(&first.events, &world.motors), [0]);
    assert_eq!(effect(&second.events, &world.motors), [1]);
}

#[test]
fn one_completed_physical_cycle_strengthens_its_participating_action() {
    let mut completed = CompetitionWorld::new(false);
    let mut unchanged = CompetitionWorld::new(false);
    for world in [&mut completed, &mut unchanged] {
        for at in [10, 20, 30] {
            world.completed_cycle(1, at);
        }
        world.act(0, 40);
    }
    completed.return_change(42);
    unchanged.return_repeat(42);

    assert_eq!(
        effect(&completed.compete(50).events, &completed.motors),
        [0]
    );
    assert_eq!(
        effect(&unchanged.compete(50).events, &unchanged.motors),
        [1]
    );
}

#[test]
fn learner_construction_requires_new_physical_membership() {
    let mut repeated = LearnerWorld::new();
    repeated.close(2, 10, true);
    repeated.close(2, 20, true);
    let repeated_probe = repeated.probe(30);

    let mut expanded = LearnerWorld::new();
    expanded.close(2, 10, true);
    expanded.close(3, 20, true);
    let expanded_probe = expanded.probe(30);

    assert_eq!(
        event_count(&repeated_probe.events, repeated.motor.effect),
        1
    );
    assert_eq!(
        event_count(&expanded_probe.events, expanded.motor.effect),
        0
    );
}

#[test]
fn construction_composes_only_a_same_moment_live_consequence() {
    let mut same = LearnerWorld::new();
    same.close(2, 10, true);
    let same_probe = same.probe(20);

    let mut stale = LearnerWorld::new();
    stale.close(2, 10, false);
    stale.consequence_value += 1;
    schedule(
        &mut stale.body,
        11,
        &[reading(stale.consequence, 0, stale.consequence_value)],
    );
    finish(&mut stale.body);
    let stale_probe = stale.probe(20);

    assert_eq!(event_count(&same_probe.events, same.motor.effect), 1);
    assert_eq!(event_count(&stale_probe.events, stale.motor.effect), 0);
}

#[test]
fn observing_events_cannot_change_the_body() {
    let mut silent = LocalWorld::new(&[1]);
    let mut observed = LocalWorld {
        body: silent.body.clone(),
        surface: silent.surface,
        motors: silent.motors.clone(),
    };

    schedule(&mut silent.body, 10, &[reading(silent.surface, 0, 1)]);
    schedule(
        &mut silent.body,
        11,
        &[Arrival::new(silent.motors[0].opportunity, 1)],
    );
    silent.body.run(256, |_| {}).unwrap();
    let trace = observed.act(10);

    assert_eq!(
        silent.body.held(silent.surface),
        observed.body.held(observed.surface)
    );
    assert_eq!(
        silent.body.held(silent.motors[0].effect),
        observed.body.held(observed.motors[0].effect)
    );
    assert_eq!(event_count(&trace.events, observed.motors[0].effect), 1);
    assert!(silent.body.is_quiet() && observed.body.is_quiet());
}

#[test]
fn dormant_body_size_does_not_change_active_work() {
    fn episode(dormant: usize) -> Work {
        let mut body = Body::default();
        let active = integrating(&mut body, 1);
        for _ in 1..=dormant {
            integrating(&mut body, 1);
        }
        schedule(&mut body, 0, &[Arrival::new(active, 1)]);
        finish(&mut body).run.work
    }

    assert_eq!(episode(0), episode(10_000));
}

#[test]
fn arena_growth_preserves_existing_physical_identity() {
    let mut body = Body::default();
    let original = integrating(&mut body, 1);
    for _ in 1..=100_000 {
        integrating(&mut body, 1);
    }
    schedule(&mut body, 0, &[Arrival::new(original, 1)]);

    assert_eq!(
        finish(&mut body)
            .events
            .into_iter()
            .map(|event| event.junction)
            .collect::<Vec<_>>(),
        [original]
    );
}
