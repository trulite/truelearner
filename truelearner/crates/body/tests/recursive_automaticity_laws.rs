#![deny(warnings)]

use truelearner_body::{
    attach,
    harness::{attach_outcome_component, attach_sensor, effect, motor, reading, schedule, Motor},
    Arrival, AutomaticityWork, Body, BodyCheckpoint, Junction, JunctionId, Link, LinkId, OpenBody,
    PhysicalEvent, Run, Work,
};

struct ChainWorld {
    body: Body,
    surface: JunctionId,
    motor: Motor,
    outcome: JunctionId,
    terminal: JunctionId,
    chain: Vec<LinkId>,
    internal: Vec<JunctionId>,
    outcome_value: i32,
}

impl ChainWorld {
    fn new(depth: usize) -> Self {
        assert!(depth >= 2);
        Self::with_delays(&vec![1; depth])
    }

    fn with_delays(delays: &[u64]) -> Self {
        assert!(delays.len() >= 2);
        let mut body = Body::default();
        let motor = motor(&mut body);
        let surface = attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motor.opportunity, 1)],
        );
        let outcome = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);

        let mut from = motor.effect;
        let mut chain = Vec::new();
        let mut internal = Vec::new();
        for delay in delays.iter().copied() {
            let next = body.add_junction(Junction::integrating(1)).unwrap();
            chain.push(body.add_link(Link::new(from, next, delay, 1)).unwrap());
            internal.push(next);
            from = next;
        }
        schedule(&mut body, 0, &[reading(outcome, 0, 0)]);
        body.run(512, |_| {}).unwrap();
        Self {
            body,
            surface,
            motor,
            outcome,
            terminal: from,
            chain,
            internal,
            outcome_value: 0,
        }
    }

    fn act(&mut self, at: u64) -> (Run, Vec<PhysicalEvent>) {
        schedule(&mut self.body, at, &[reading(self.surface, 0, 1)]);
        schedule(
            &mut self.body,
            at + 1,
            &[Arrival::new(self.motor.opportunity, 1)],
        );
        let mut events = Vec::new();
        let run = self.body.run(512, |event| events.push(event)).unwrap();
        assert!(self.body.is_quiet());
        assert_eq!(effect(&events, &[self.motor]), [0], "action at {at}");
        (run, events)
    }

    fn close(&mut self, at: u64) {
        self.outcome_value += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(self.outcome, 0, self.outcome_value)],
        );
        self.body.run(512, |_| {}).unwrap();
        assert!(self.body.is_quiet());
    }

    fn complete(&mut self, at: u64) -> (Run, Vec<PhysicalEvent>) {
        let result = self.act(at);
        assert!(result.1.iter().any(|event| event.junction == self.terminal));
        self.close(at + 32);
        result
    }
}

#[test]
fn heterogeneous_delays_survive_every_recursive_level() {
    let mut world = ChainWorld::with_delays(&[1, 3, 2, 4, 1, 2, 3, 1]);
    world.complete(10);
    let (ordinary, ordinary_events) = world.complete(60);
    for at in [110, 160, 210, 260, 310, 360, 410] {
        world.complete(at);
    }
    let (automatic, automatic_events) = world.act(460);

    let ordinary_terminal = terminal_event(&ordinary_events, world.terminal);
    let automatic_terminal = terminal_event(&automatic_events, world.terminal);
    assert_eq!(ordinary_terminal.at - 60, 19);
    assert_eq!(automatic_terminal.at - 460, ordinary_terminal.at - 60);
    assert_eq!(automatic_terminal.impulse, ordinary_terminal.impulse);
    assert!(total(automatic.work) < total(ordinary.work));
    assert!(world.body.automaticity_work().composites_formed >= 7);
}

fn total(work: Work) -> u64 {
    work.arrivals + work.meetings + work.changes + work.link_visits + work.emissions
}

fn terminal_event(events: &[PhysicalEvent], terminal: JunctionId) -> PhysicalEvent {
    *events
        .iter()
        .find(|event| event.junction == terminal)
        .expect("the terminal physical effect occurs")
}

fn delta(after: AutomaticityWork, before: AutomaticityWork) -> AutomaticityWork {
    AutomaticityWork {
        pair_observations: after.pair_observations - before.pair_observations,
        supported_closure_updates: after.supported_closure_updates
            - before.supported_closure_updates,
        composites_formed: after.composites_formed - before.composites_formed,
    }
}

#[test]
fn retained_links_reenter_the_same_law_and_form_a_recursive_hierarchy() {
    let mut world = ChainWorld::new(8);
    let before = world.body.automaticity_work();

    world.complete(10);
    let (ordinary, ordinary_events) = world.complete(60);
    world.complete(110);
    let (level_one, level_one_events) = world.complete(160);
    world.complete(210);
    world.complete(260);
    let (level_two, level_two_events) = world.complete(310);
    world.complete(360);
    world.complete(410);
    let (level_three, level_three_events) = world.act(460);

    let ordinary_terminal = terminal_event(&ordinary_events, world.terminal);
    for (started, events) in [
        (160, &level_one_events),
        (310, &level_two_events),
        (460, &level_three_events),
    ] {
        let terminal = terminal_event(events, world.terminal);
        assert_eq!(terminal.at - started, ordinary_terminal.at - 60);
        assert_eq!(terminal.impulse, ordinary_terminal.impulse);
        assert_eq!((terminal.before, terminal.after), (0, 1));
    }

    assert!(total(level_one.work) < total(ordinary.work));
    assert!(total(level_two.work) < total(level_one.work));
    assert!(total(level_three.work) < total(level_two.work));
    let acquisition = delta(world.body.automaticity_work(), before);
    assert!(acquisition.composites_formed >= 7);
    assert!(acquisition.total() > acquisition.composites_formed);
}

#[test]
fn a_visible_branch_prevents_a_shortcut_from_erasing_it() {
    let mut world = ChainWorld::new(8);
    let fork = world.internal[2];
    let branch = world.body.add_junction(Junction::integrating(1)).unwrap();
    world.body.add_link(Link::new(fork, branch, 1, 1)).unwrap();

    let (ordinary_run, ordinary_events) = world.complete(10);
    for (index, at) in [10, 60, 110, 160, 210, 260, 310].into_iter().enumerate() {
        if index != 0 {
            world.complete(at);
        }
    }
    let (automatic, events) = world.act(360);
    assert!(events.iter().any(|event| event.junction == fork));
    assert!(events.iter().any(|event| event.junction == branch));
    assert!(events.iter().any(|event| event.junction == world.terminal));
    assert!(ordinary_events.iter().any(|event| event.junction == fork));
    assert!(total(automatic.work) < total(ordinary_run.work));
}

#[test]
fn a_changed_leaf_invalidates_every_dependent_level_before_it_fires() {
    let mut world = ChainWorld::new(8);
    for at in [10, 60, 110, 160, 210, 260, 310, 360, 410] {
        world.complete(at);
    }
    let (automatic, _) = world.act(460);
    world.close(492);

    let changed = world.chain[3];
    world.body.set_link_impulse(changed, -1).unwrap();
    let (_, interrupted) = world.act(520);
    assert_eq!(effect(&interrupted, &[world.motor]), [0]);
    assert!(!interrupted
        .iter()
        .any(|event| event.junction == world.terminal));
    world.close(552);

    world.body.set_link_impulse(changed, 1).unwrap();
    let (restored, restored_events) = world.act(580);
    assert!(restored_events
        .iter()
        .any(|event| event.junction == world.terminal));
    assert_eq!(restored.work, automatic.work);
}

#[test]
fn a_pending_input_into_an_omitted_interior_forces_local_fallback() {
    let mut world = ChainWorld::new(8);
    for at in [10, 60, 110, 160, 210, 260, 310, 360, 410] {
        world.complete(at);
    }
    let interrupted = world.internal[3];
    schedule(&mut world.body, 526, &[Arrival::new(interrupted, 1)]);
    let (_, events) = world.act(520);

    let meeting = events
        .iter()
        .find(|event| event.junction == interrupted)
        .expect("the interrupted physical interior must remain present");
    assert_eq!(meeting.arrivals, 2);
    assert!(events.iter().any(|event| event.junction == world.terminal));
}

#[test]
fn recursive_automaticity_survives_checkpoint_and_continuous_time() {
    let mut world = ChainWorld::new(8);
    for at in [10, 60, 110, 160, 210, 260, 310, 360, 410] {
        world.complete(at);
    }
    let bytes = world.body.checkpoint().unwrap().canonical_bytes().unwrap();
    let restored = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();
    let mut replay = ChainWorld {
        body: restored,
        surface: world.surface,
        motor: world.motor,
        outcome: world.outcome,
        terminal: world.terminal,
        chain: world.chain.clone(),
        internal: world.internal.clone(),
        outcome_value: world.outcome_value,
    };

    let (plain_run, plain_events) = world.act(100_000);
    let (replay_run, replay_events) = replay.act(100_000);
    assert_eq!(plain_run, replay_run);
    assert_eq!(plain_events, replay_events);
    assert_eq!(
        world.body.automaticity_work(),
        replay.body.automaticity_work()
    );
}

#[test]
fn recursive_dependencies_are_remapped_when_the_learned_body_is_attached() {
    let mut world = ChainWorld::new(8);
    for at in [10, 60, 110, 160, 210, 260, 310, 360, 410] {
        world.complete(at);
    }
    let mut expected = world.body.clone();
    schedule(&mut expected, 460, &[reading(world.surface, 0, 1)]);
    schedule(
        &mut expected,
        461,
        &[Arrival::new(world.motor.opportunity, 1)],
    );
    let mut expected_events = Vec::new();
    let expected_run = expected
        .run(512, |event| expected_events.push(event))
        .unwrap();

    let part = OpenBody::new(
        world.body,
        vec![
            world.surface,
            world.motor.opportunity,
            world.motor.effect,
            world.terminal,
        ],
    )
    .unwrap();
    let ports = [
        part.port(0).unwrap(),
        part.port(1).unwrap(),
        part.port(2).unwrap(),
        part.port(3).unwrap(),
    ];
    let mut host = Body::default();
    let dormant_from = host.add_junction(Junction::integrating(1)).unwrap();
    let dormant_to = host.add_junction(Junction::integrating(1)).unwrap();
    host.add_link(Link::new(dormant_from, dormant_to, 1, 1))
        .unwrap();
    let attached = attach(&mut host, part, &[]).unwrap();
    let [surface, opportunity, effect, terminal] = ports.map(|port| attached.port(port).unwrap());
    schedule(&mut host, 460, &[reading(surface, 0, 1)]);
    schedule(&mut host, 461, &[Arrival::new(opportunity, 1)]);
    let mut events = Vec::new();
    let run = host.run(512, |event| events.push(event)).unwrap();

    assert_eq!(run, expected_run);
    assert_eq!(
        events
            .iter()
            .filter(|event| event.junction == effect || event.junction == terminal)
            .map(|event| (event.at, event.impulse, event.before, event.after,))
            .collect::<Vec<_>>(),
        expected_events
            .iter()
            .filter(|event| {
                event.junction == world.motor.effect || event.junction == world.terminal
            })
            .map(|event| (event.at, event.impulse, event.before, event.after,))
            .collect::<Vec<_>>()
    );
}

#[derive(Clone, Copy)]
struct ProductPart {
    surface: JunctionId,
    motor: Motor,
    outcome: JunctionId,
    terminal: JunctionId,
    outcome_value: i32,
}

struct ProductWorld {
    body: Body,
    parts: [ProductPart; 2],
}

impl ProductWorld {
    fn new(reverse_construction: bool) -> Self {
        fn add_part(body: &mut Body) -> ProductPart {
            let motor = motor(body);
            let surface = attach_sensor(body, Junction::integrating(1), &[(motor.opportunity, 1)]);
            let outcome = attach_sensor(body, Junction::sampled(1_000), &[]);
            attach_outcome_component(body, outcome, [motor.opportunity]);
            let mut terminal = motor.effect;
            for _ in 0..8 {
                let next = body.add_junction(Junction::integrating(1)).unwrap();
                body.add_link(Link::new(terminal, next, 1, 1)).unwrap();
                terminal = next;
            }
            ProductPart {
                surface,
                motor,
                outcome,
                terminal,
                outcome_value: 0,
            }
        }

        let mut body = Body::default();
        let parts = if reverse_construction {
            let second = add_part(&mut body);
            let first = add_part(&mut body);
            [first, second]
        } else {
            [add_part(&mut body), add_part(&mut body)]
        };
        schedule(&mut body, 0, &parts.map(|part| reading(part.outcome, 0, 0)));
        body.run(1_024, |_| {}).unwrap();
        Self { body, parts }
    }

    fn act(&mut self, at: u64) -> (Run, Vec<PhysicalEvent>) {
        schedule(
            &mut self.body,
            at,
            &std::array::from_fn::<_, 2, _>(|index| reading(self.parts[index].surface, 0, 1)),
        );
        schedule(
            &mut self.body,
            at + 1,
            &std::array::from_fn::<_, 2, _>(|index| {
                Arrival::new(self.parts[index].motor.opportunity, 1)
            }),
        );
        let mut events = Vec::new();
        let run = self.body.run(1_024, |event| events.push(event)).unwrap();
        for part in self.parts {
            assert!(events
                .iter()
                .any(|event| event.junction == part.motor.effect));
            assert!(events.iter().any(|event| event.junction == part.terminal));
        }
        (run, events)
    }

    fn close(&mut self, at: u64) {
        for part in &mut self.parts {
            part.outcome_value += 1;
        }
        schedule(
            &mut self.body,
            at,
            &std::array::from_fn::<_, 2, _>(|index| {
                reading(
                    self.parts[index].outcome,
                    0,
                    self.parts[index].outcome_value,
                )
            }),
        );
        self.body.run(1_024, |_| {}).unwrap();
    }

    fn complete(&mut self, at: u64) -> (Run, Vec<PhysicalEvent>) {
        let result = self.act(at);
        self.close(at + 32);
        result
    }
}

#[test]
fn independent_simultaneous_skills_consolidate_as_a_product() {
    fn climb(reverse: bool) -> (Run, Run, AutomaticityWork) {
        let mut world = ProductWorld::new(reverse);
        world.complete(10);
        let ordinary = world.complete(60).0;
        for at in [110, 160, 210, 260, 310, 360, 410] {
            world.complete(at);
        }
        let automatic = world.act(460).0;
        (ordinary, automatic, world.body.automaticity_work())
    }

    let forward = climb(false);
    let reversed = climb(true);
    assert!(total(forward.1.work) < total(forward.0.work));
    assert!(forward.2.composites_formed >= 16);
    assert_eq!(forward, reversed);
}

#[test]
fn formation_cost_is_finite_and_reuse_can_amortize_it() {
    let mut world = ChainWorld::new(8);
    let before = world.body.automaticity_work();
    world.complete(10);
    let ordinary = world.complete(60).0;
    for at in [110, 160, 210, 260, 310, 360, 410] {
        world.complete(at);
    }
    let acquired = delta(world.body.automaticity_work(), before);
    let (automatic, _) = world.act(460);
    world.close(492);
    let saving = total(ordinary.work) - total(automatic.work);
    assert!(saving > 0);
    let break_even_uses = acquired.total().div_ceil(saving);
    assert!(break_even_uses > 0);

    let formed = world.body.automaticity_work().composites_formed;
    let state = world.body.automaticity_state();
    let mut repaid = 0_u64;
    for use_index in 0..break_even_uses {
        let at = 550 + use_index * 50;
        let run = world.complete(at).0;
        repaid = repaid.saturating_add(total(ordinary.work) - total(run.work));
    }
    assert!(repaid >= acquired.total());
    assert_eq!(world.body.automaticity_work().composites_formed, formed);
    assert_eq!(world.body.automaticity_state(), state);
}

#[test]
fn silence_preserves_probation_but_a_closed_invalid_context_forgets_it_locally() {
    let mut world = ChainWorld::new(8);
    world.complete(10);
    world.complete(60);
    let probation = world.body.automaticity_state();
    assert!(probation.candidate_pairs > 0);
    assert!(!probation.has_recursive_composites);

    let distractor = world.body.add_junction(Junction::integrating(1)).unwrap();
    schedule(&mut world.body, 500, &[Arrival::new(distractor, 1)]);
    world.body.run(32, |_| {}).unwrap();
    assert_eq!(world.body.automaticity_state(), probation);

    for link in &world.chain {
        world.body.set_link_impulse(*link, -1).unwrap();
    }
    let (_, interrupted) = world.act(600);
    assert!(!interrupted
        .iter()
        .any(|event| event.junction == world.terminal));
    assert_eq!(world.body.automaticity_state().open_witnesses, 0);
    world.close(632);
    assert_eq!(world.body.automaticity_state().open_witnesses, 0);
    assert_eq!(world.body.automaticity_state().candidate_pairs, 0);

    for link in &world.chain {
        world.body.set_link_impulse(*link, 1).unwrap();
    }
    world.complete(700);
    assert!(world.body.automaticity_state().candidate_pairs > 0);
    assert!(!world.body.automaticity_state().has_recursive_composites);
    world.complete(750);
    world.complete(800);
    assert!(world.body.automaticity_state().has_recursive_composites);
}
