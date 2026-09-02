#![deny(warnings)]

use truelearner_body::{
    attach,
    harness::{attach_outcome_component, attach_sensor, effect, motor, reading, schedule, Motor},
    verify_choice_contract, Arrival, Body, BodyCheckpoint, Junction, JunctionId, Link, OpenBody,
    Path, PhysicalEvent, ReturnDecision, Run, TraceEvent, Work,
};

struct RepeatedPathWorld {
    body: Body,
    surface: JunctionId,
    motor: Motor,
    outcome: JunctionId,
    outcome_value: i32,
}

impl RepeatedPathWorld {
    fn new() -> Self {
        let mut body = Body::default();
        let motor = motor(&mut body);
        let surface = attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        );
        let outcome = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);
        schedule(&mut body, 0, &[reading(outcome, 0, 0)]);
        body.run(256, |_| {}).unwrap();
        Self {
            body,
            surface,
            motor,
            outcome,
            outcome_value: 0,
        }
    }

    fn act(&mut self, at: u64) -> (Run, Vec<PhysicalEvent>, Vec<TraceEvent>) {
        schedule(&mut self.body, at, &[reading(self.surface, 0, 1)]);
        schedule(
            &mut self.body,
            at + 1,
            &[Arrival::new(self.motor.opportunity, 1)],
        );
        self.run_traced()
    }

    fn close(&mut self, at: u64) -> (Run, Vec<PhysicalEvent>, Vec<TraceEvent>) {
        self.outcome_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.outcome, 0, self.outcome_value)],
        );
        self.run_traced()
    }

    fn complete(&mut self, at: u64) -> (Run, Vec<PhysicalEvent>, Path) {
        let (run, events, _) = self.act(at);
        assert_eq!(effect(&events, &[self.motor]), [0]);
        let (_, returned_events, trace) = self.close(at + 4);
        assert!(effect(&returned_events, &[self.motor]).is_empty());
        let path = trace
            .iter()
            .find_map(|event| match event {
                TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted => {
                    returned.path
                }
                _ => None,
            })
            .expect("a unique returned consequence closes the path");
        (run, events, path)
    }

    fn run_traced(&mut self) -> (Run, Vec<PhysicalEvent>, Vec<TraceEvent>) {
        let mut events = Vec::new();
        let mut trace = Vec::new();
        let run = self
            .body
            .run_traced(256, |event| events.push(event), |event| trace.push(event))
            .unwrap();
        assert!(self.body.is_quiet());
        verify_choice_contract(&trace).unwrap();
        (run, events, trace)
    }
}

fn total(work: Work) -> u64 {
    work.arrivals + work.meetings + work.changes + work.link_visits + work.emissions
}

fn motor_event(events: &[PhysicalEvent], motor: Motor) -> PhysicalEvent {
    *events
        .iter()
        .find(|event| event.junction == motor.effect)
        .expect("the motor effect occurs")
}

#[test]
fn three_supported_closed_uses_make_the_same_effect_with_less_internal_work() {
    let mut world = RepeatedPathWorld::new();
    world.complete(10);
    let (subthreshold, _, _) = world.complete(20);
    let (ordinary, ordinary_events, _) = world.complete(30);
    assert_eq!(subthreshold.work, ordinary.work);

    let (automatic, automatic_events, _) = world.act(40);
    assert_eq!(effect(&automatic_events, &[world.motor]), [0]);

    let ordinary_effect = motor_event(&ordinary_events, world.motor);
    let automatic_effect = motor_event(&automatic_events, world.motor);
    assert_eq!(ordinary_effect.at - 30, automatic_effect.at - 40);
    assert_eq!(ordinary_effect.impulse, automatic_effect.impulse);
    assert_eq!((ordinary_effect.before, ordinary_effect.after), (0, 1));
    assert_eq!((automatic_effect.before, automatic_effect.after), (0, 1));
    assert!(
        total(automatic.work) < total(ordinary.work),
        "automatic={:?}, ordinary={:?}",
        automatic.work,
        ordinary.work
    );
}

#[test]
fn changing_a_parent_invalidates_the_stale_composite_before_it_fires() {
    let mut world = RepeatedPathWorld::new();
    world.complete(10);
    world.complete(20);
    let (_, _, path) = world.complete(30);
    world.body.set_link_impulse(path.second, -1).unwrap();

    let (_, events, _) = world.act(40);
    assert!(effect(&events, &[world.motor]).is_empty());
}

#[test]
fn a_new_effect_of_the_omitted_middle_forces_the_full_path() {
    let mut world = RepeatedPathWorld::new();
    world.complete(10);
    world.complete(20);
    let (_, _, path) = world.complete(30);

    let newly_visible = world.body.add_junction(Junction::integrating(1)).unwrap();
    world
        .body
        .add_link(Link::new(path.middle, newly_visible, 0, 1))
        .unwrap();

    let (_, events, _) = world.act(40);
    assert_eq!(effect(&events, &[world.motor]), [0]);
    assert!(events.iter().any(|event| event.junction == newly_visible));
}

#[test]
fn automaticity_survives_a_checkpoint_round_trip() {
    let mut world = RepeatedPathWorld::new();
    world.complete(10);
    world.complete(20);
    let (ordinary, _, _) = world.complete(30);
    let bytes = world.body.checkpoint().unwrap().canonical_bytes().unwrap();
    let restored = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();
    let mut restored_world = RepeatedPathWorld {
        body: restored,
        surface: world.surface,
        motor: world.motor,
        outcome: world.outcome,
        outcome_value: world.outcome_value,
    };

    let (plain, plain_events, plain_trace) = world.act(40);
    let (replayed, replayed_events, replayed_trace) = restored_world.act(40);
    assert_eq!(plain, replayed);
    assert_eq!(plain_events, replayed_events);
    assert_eq!(plain_trace, replayed_trace);
    assert!(total(plain.work) < total(ordinary.work));
}

#[test]
fn attaching_an_automatic_part_remaps_its_physical_support() {
    let mut world = RepeatedPathWorld::new();
    world.complete(10);
    world.complete(20);
    world.complete(30);
    let mut expected = world.body.clone();
    schedule(&mut expected, 40, &[reading(world.surface, 0, 1)]);
    schedule(
        &mut expected,
        41,
        &[Arrival::new(world.motor.opportunity, 1)],
    );
    let expected_run = expected.run(256, |_| {}).unwrap();

    let part = OpenBody::new(
        world.body,
        vec![world.surface, world.motor.opportunity, world.motor.effect],
    )
    .unwrap();
    let surface_port = part.port(0).unwrap();
    let opportunity_port = part.port(1).unwrap();
    let effect_port = part.port(2).unwrap();
    let mut host = Body::default();
    let dormant_from = host.add_junction(Junction::integrating(1)).unwrap();
    let dormant_to = host.add_junction(Junction::integrating(1)).unwrap();
    host.add_link(Link::new(dormant_from, dormant_to, 1, 1))
        .unwrap();
    let attachment = attach(&mut host, part, &[]).unwrap();
    let surface = attachment.port(surface_port).unwrap();
    let opportunity = attachment.port(opportunity_port).unwrap();
    let attached_effect = attachment.port(effect_port).unwrap();
    schedule(&mut host, 40, &[reading(surface, 0, 1)]);
    schedule(&mut host, 41, &[Arrival::new(opportunity, 1)]);
    let mut events = Vec::new();
    let attached_run = host.run(256, |event| events.push(event)).unwrap();

    assert_eq!(attached_run, expected_run);
    assert!(events.iter().any(|event| event.junction == attached_effect));
}
