use truelearner_body::{
    attach, harness::*, Arrival, Body, ChoiceBasis, Join, Junction, JunctionId, Link, OpenBody,
    ReturnDecision, TraceEvent, Work,
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

    fn act(&mut self, at: u64, cause: u64) -> Trace {
        schedule(&mut self.body, at, &[reading(self.surface, 0, 1, cause)]);
        let opportunities = self
            .motors
            .iter()
            .map(|motor| Arrival::caused(motor.opportunity, 1, cause))
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
        schedule(&mut body, 0, &[reading(consequence, 0, 0, 0)]);
        finish(&mut body);
        Self {
            body,
            surfaces,
            motors,
            consequence,
            consequence_value: 0,
        }
    }

    fn act(&mut self, which: usize, at: u64, cause: u64) -> Trace {
        schedule(
            &mut self.body,
            at,
            &[reading(self.surfaces[which], 0, 1, cause)],
        );
        schedule(
            &mut self.body,
            at + 1,
            &[Arrival::caused(self.motors[which].opportunity, 1, cause)],
        );
        finish(&mut self.body)
    }

    fn compete(&mut self, at: u64, cause: u64) -> Trace {
        self.compete_traced(at, cause).0
    }

    fn compete_traced(&mut self, at: u64, cause: u64) -> (Trace, Vec<TraceEvent>) {
        schedule(
            &mut self.body,
            at,
            &self.surfaces.map(|surface| reading(surface, 0, 1, cause)),
        );
        schedule(
            &mut self.body,
            at + 1,
            &self
                .motors
                .map(|motor| Arrival::caused(motor.opportunity, 1, cause)),
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

    fn return_change(&mut self, at: u64, cause: u64) -> Trace {
        self.consequence_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.consequence, 0, self.consequence_value, cause)],
        );
        finish(&mut self.body)
    }

    fn return_repeat(&mut self, at: u64, cause: u64) -> Trace {
        schedule(
            &mut self.body,
            at,
            &[reading(self.consequence, 0, self.consequence_value, cause)],
        );
        finish(&mut self.body)
    }

    fn completed_cycle(&mut self, which: usize, at: u64, cause: u64) {
        let action = self.act(which, at, cause);
        assert_eq!(effect(&action.events, &self.motors), [which]);
        self.return_change(at + 2, cause);
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
        schedule(&mut body, 0, &[reading(consequence, 0, 0, 0)]);
        finish(&mut body);
        Self {
            body,
            members,
            motor,
            consequence,
            consequence_value: 0,
        }
    }

    fn close(&mut self, member_count: usize, at: u64, cause: u64, live_change: bool) -> Trace {
        let arrivals = self.members[..member_count]
            .iter()
            .map(|member| reading(*member, 0, 1, cause))
            .collect::<Vec<_>>();
        schedule(&mut self.body, at, &arrivals);
        if live_change {
            self.consequence_value += 1;
            schedule(
                &mut self.body,
                at,
                &[reading(self.consequence, 0, self.consequence_value, cause)],
            );
        }
        finish(&mut self.body)
    }

    fn probe(&mut self, at: u64, cause: u64) -> Trace {
        schedule(&mut self.body, at, &[reading(self.members[0], 0, 1, cause)]);
        schedule(
            &mut self.body,
            at + 1,
            &[Arrival::caused(self.motor.opportunity, 1, cause)],
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
        schedule(&mut body, at, &[reading(sensor, 0, value, 0)]);
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
    schedule(&mut body, 1, &[reading(sensor, 0, 7, 0)]);
    schedule(&mut body, 5, &[reading(sensor, 0, 9, 0)]);

    assert!(finish(&mut body).events.is_empty());
}

#[test]
fn causal_identity_composes_only_when_all_meeting_arrivals_agree() {
    fn cause(left: u64, right: u64) -> u64 {
        let mut body = Body::default();
        let junction = integrating(&mut body, 2);
        schedule(
            &mut body,
            0,
            &[
                Arrival::caused(junction, 1, left),
                Arrival::caused(junction, 1, right),
            ],
        );
        finish(&mut body).events[0].cause
    }

    assert_eq!(cause(8, 8), 8);
    assert_eq!(cause(8, 9), 0);
    assert_eq!(cause(9, 8), 0);
}

#[test]
fn a_surface_without_a_nearby_output_changes_no_later_action() {
    let mut world = LocalWorld::new(&[]);
    let first = world.act(10, 1);
    let second = world.act(20, 2);

    assert!(effect(&first.events, &world.motors).is_empty());
    assert!(effect(&second.events, &world.motors).is_empty());
}

#[test]
fn a_local_surface_forms_one_reusable_choice_without_duplicate_growth() {
    let mut world = LocalWorld::new(&[2]);
    let first = world.act(10, 1);
    let second = world.act(20, 2);
    let third = world.act(30, 3);

    assert_eq!(effect(&first.events, &world.motors), [0]);
    assert_eq!(effect(&second.events, &world.motors), [0]);
    assert_eq!(effect(&third.events, &world.motors), [0]);
    assert_eq!(second.run.work, third.run.work);
}

#[test]
fn formation_is_local_and_does_not_cross_distance_three() {
    let mut world = LocalWorld::new(&[3]);
    let trace = world.act(10, 1);

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

    schedule(&mut body, 10, &[reading(outside, 0, 1, 7)]);
    schedule(&mut body, 11, &[Arrival::caused(motor.opportunity, 1, 7)]);
    let trace = finish(&mut body);

    assert_eq!(event_count(&trace.events, returned), 1);
    assert_eq!(event_count(&trace.events, motor.effect), 0);
}

#[test]
fn one_connected_world_chooses_exactly_one_action() {
    let mut world = LocalWorld::new(&[1, 2]);
    let trace = world.act(10, 3);

    assert_eq!(effect(&trace.events, &world.motors).len(), 1);
}

#[test]
fn latest_available_consequence_precedes_old_strength() {
    let mut world = CompetitionWorld::new(false);
    for (at, cause) in [(10, 1), (20, 2), (30, 3)] {
        world.completed_cycle(0, at, cause);
    }
    world.completed_cycle(1, 40, 4);

    let chosen = world.compete(50, 5);
    assert_eq!(effect(&chosen.events, &world.motors), [1]);
}

#[test]
fn an_exact_current_return_precedes_every_other_action() {
    let mut world = CompetitionWorld::new(false);
    for (at, cause) in [(10, 1), (20, 2), (30, 3)] {
        world.completed_cycle(1, at, cause);
    }
    let opened = world.act(0, 40, 42);
    assert_eq!(effect(&opened.events, &world.motors), [0]);

    let chosen = world.compete(50, 42);
    assert_eq!(effect(&chosen.events, &world.motors), [0]);
}

#[test]
fn ambiguous_current_returns_do_not_create_a_false_preference() {
    let mut world = CompetitionWorld::new(false);
    for (at, cause) in [(10, 1), (20, 2), (30, 3)] {
        world.completed_cycle(0, at, cause);
    }
    world.act(0, 40, 7);
    world.act(1, 42, 7);

    let chosen = world.compete(50, 7);
    assert_eq!(effect(&chosen.events, &world.motors), [0]);
}

#[test]
fn an_unanswered_action_gives_an_alternative_the_next_chance() {
    let mut world = CompetitionWorld::new(false);

    let first = world.act(0, 10, 1);
    assert_eq!(effect(&first.events, &world.motors), [0]);

    let (next, trace) = world.compete_traced(20, 2);
    assert_eq!(effect(&next.events, &world.motors), [1]);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Choice(choice) if choice.basis == Some(ChoiceBasis::UnansweredOutputRelease)
    )));
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Candidate(candidate)
            if candidate.path.output == world.motors[0].opportunity && candidate.unanswered
    )));
}

#[test]
fn a_fresh_external_opportunity_crosses_only_into_the_root_learner() {
    let mut root = LearnerWorld::new();
    root.close(2, 10, 1, true);
    let root_probe = root.probe(20, 2);

    let mut child = LearnerWorld::new();
    child.close(2, 10, 1, true);
    child.close(3, 20, 2, true);
    let child_probe = child.probe(30, 3);

    assert_eq!(event_count(&root_probe.events, root.motor.effect), 1);
    assert_eq!(event_count(&child_probe.events, child.motor.effect), 0);
}

#[test]
fn disconnected_causal_components_choose_independently() {
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
        &consequences.map(|sensor| reading(sensor, 0, 0, 0)),
    );
    finish(&mut body);

    for index in 0..2 {
        let cause = index as u64 + 1;
        schedule(
            &mut body,
            10 + index as u64 * 10,
            &[reading(surfaces[index], 0, 1, cause)],
        );
        schedule(
            &mut body,
            11 + index as u64 * 10,
            &[Arrival::caused(motors[index].opportunity, 1, cause)],
        );
        finish(&mut body);
        schedule(
            &mut body,
            12 + index as u64 * 10,
            &[reading(consequences[index], 0, 1, cause)],
        );
        finish(&mut body);
    }

    schedule(
        &mut body,
        40,
        &[
            reading(surfaces[0], 0, 1, 30),
            reading(surfaces[1], 0, 1, 40),
        ],
    );
    schedule(
        &mut body,
        41,
        &[
            Arrival::caused(motors[0].opportunity, 1, 30),
            Arrival::caused(motors[1].opportunity, 1, 40),
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
        world.completed_cycle(0, 10, 1);
        world.completed_cycle(1, 20, 2);
    }

    let forward_trace = forward.compete(30, 3);
    let reverse_trace = reverse.compete(30, 3);

    let forward_positions = effect(&forward_trace.events, &forward.motors);
    let reverse_positions = effect(&reverse_trace.events, &reverse.motors);
    assert_eq!(forward_positions, reverse_positions);
}

#[test]
fn an_actual_output_opens_one_physical_return() {
    let mut world = CompetitionWorld::new(false);
    let action = world.act(0, 10, 11);
    assert_eq!(effect(&action.events, &world.motors), [0]);
    world.return_change(12, 11);

    let returned = world.compete(20, 11);
    assert_eq!(effect(&returned.events, &world.motors), [0]);
}

#[test]
fn a_later_consequence_credits_only_the_action_that_physically_happened() {
    let mut world = CompetitionWorld::new(false);
    world.act(1, 10, 8);
    world.act(0, 20, 9);
    world.return_change(22, 9);

    let chosen = world.compete(30, 10);
    assert_eq!(effect(&chosen.events, &world.motors), [0]);
}

#[test]
fn preopening_and_repeated_samples_close_nothing_and_credit_nothing() {
    let mut before = CompetitionWorld::new(false);
    before.return_change(10, 30);
    before.act(0, 20, 30);

    let mut repeated = CompetitionWorld::new(false);
    repeated.act(0, 10, 30);
    repeated.return_repeat(12, 30);

    assert_eq!(
        effect(&before.compete(40, 40).events, &before.motors),
        effect(&repeated.compete(40, 40).events, &repeated.motors)
    );
}

#[test]
fn one_physical_consequence_has_one_learning_effect() {
    let mut once = CompetitionWorld::new(false);
    once.act(0, 10, 4);
    once.return_change(12, 4);

    let mut repeated = once.body.clone();
    schedule(
        &mut repeated,
        13,
        &[reading(once.consequence, 0, once.consequence_value, 4)],
    );
    finish(&mut repeated);

    let once_trace = once.compete(20, 5);
    schedule(
        &mut repeated,
        20,
        &once.surfaces.map(|surface| reading(surface, 0, 1, 5)),
    );
    schedule(
        &mut repeated,
        21,
        &once
            .motors
            .map(|motor| Arrival::caused(motor.opportunity, 1, 5)),
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
    for (at, cause) in [(10, 1), (20, 2), (30, 3)] {
        world.completed_cycle(1, at, cause);
    }
    world.completed_cycle(0, 40, 4);

    let first = world.compete(50, 5);
    let second = world.compete(60, 6);
    assert_eq!(effect(&first.events, &world.motors), [0]);
    assert_eq!(effect(&second.events, &world.motors), [1]);
}

#[test]
fn one_completed_physical_cycle_strengthens_its_participating_action() {
    let mut completed = CompetitionWorld::new(false);
    let mut unchanged = CompetitionWorld::new(false);
    for world in [&mut completed, &mut unchanged] {
        for (at, cause) in [(10, 1), (20, 2), (30, 3)] {
            world.completed_cycle(1, at, cause);
        }
        world.act(0, 40, 4);
    }
    completed.return_change(42, 4);
    unchanged.return_repeat(42, 4);

    assert_eq!(
        effect(&completed.compete(50, 5).events, &completed.motors),
        [0]
    );
    assert_eq!(
        effect(&unchanged.compete(50, 5).events, &unchanged.motors),
        [1]
    );
}

#[test]
fn an_ambiguous_return_does_not_strengthen_a_path() {
    let mut ambiguous = CompetitionWorld::new(false);
    ambiguous.act(0, 10, 7);
    ambiguous.act(1, 20, 7);
    ambiguous.consequence_value += 1;
    schedule(
        &mut ambiguous.body,
        22,
        &[reading(
            ambiguous.consequence,
            0,
            ambiguous.consequence_value,
            7,
        )],
    );
    let mut trace = Vec::new();
    ambiguous
        .body
        .run_traced(256, |_| {}, |event| trace.push(event))
        .unwrap();

    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Ambiguous
    )));
    assert!(!trace
        .iter()
        .any(|event| matches!(event, TraceEvent::Strengthened(_))));
}

#[test]
fn learner_construction_requires_new_physical_membership() {
    let mut repeated = LearnerWorld::new();
    repeated.close(2, 10, 1, true);
    repeated.close(2, 20, 2, true);
    let repeated_probe = repeated.probe(30, 3);

    let mut expanded = LearnerWorld::new();
    expanded.close(2, 10, 1, true);
    expanded.close(3, 20, 2, true);
    let expanded_probe = expanded.probe(30, 3);

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
    same.close(2, 10, 1, true);
    let same_probe = same.probe(20, 2);

    let mut stale = LearnerWorld::new();
    stale.close(2, 10, 1, false);
    stale.consequence_value += 1;
    schedule(
        &mut stale.body,
        11,
        &[reading(stale.consequence, 0, stale.consequence_value, 1)],
    );
    finish(&mut stale.body);
    let stale_probe = stale.probe(20, 2);

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

    schedule(&mut silent.body, 10, &[reading(silent.surface, 0, 1, 1)]);
    schedule(
        &mut silent.body,
        11,
        &[Arrival::caused(silent.motors[0].opportunity, 1, 1)],
    );
    silent.body.run(256, |_| {}).unwrap();
    let trace = observed.act(10, 1);

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
