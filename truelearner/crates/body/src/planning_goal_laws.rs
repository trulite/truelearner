//! Unit-test ladder for the physical prerequisites of planning and goal discovery.
//!
//! The active tests name laws the body already supports. Ignored tests freeze the
//! next unsupported transitions without adding a planner, goal object, reward,
//! episode verdict, or evaluator state to the learner.

use crate::{
    attach,
    harness::{attach_outcome_component, attach_sensor, effect, motor, reading, schedule, Motor},
    verify_choice_laws, Arrival, Body, BodyCheckpoint, ChoiceBasis, Junction, JunctionId, Link,
    LinkId, LinkRole, OpenBody, PhysicalEvent, ReturnDecision, TraceEvent,
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

#[derive(Clone)]
struct PlanningWorld {
    body: Body,
    starts: [JunctionId; 2],
    route: JunctionId,
    dead_end: JunctionId,
    closure: JunctionId,
    motors: [Motor; 3],
    outcome_witnesses: [LinkId; 3],
    route_value: i32,
    dead_end_value: i32,
    condition_value: i32,
}

impl PlanningWorld {
    const ROUTE: usize = 0;
    const DEAD_END: usize = 1;
    const TERMINAL: usize = 2;
    const ROUTE_START: usize = 0;
    const SHARED_START: usize = 1;

    fn new() -> Self {
        Self::with_dormant_prefix(false)
    }

    fn with_dormant_prefix(prefix: bool) -> Self {
        let mut body = Body::default();
        if prefix {
            let from = body.add_junction(Junction::integrating(1)).unwrap();
            let to = body.add_junction(Junction::integrating(1)).unwrap();
            body.add_link(Link::new(from, to, 1, 1)).unwrap();
        }
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
        let outcome_witnesses = [
            outcome_witness(&mut body, route, motors[Self::ROUTE].opportunity),
            outcome_witness(&mut body, dead_end, motors[Self::DEAD_END].opportunity),
            outcome_witness(&mut body, closure, motors[Self::TERMINAL].opportunity),
        ];
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
            outcome_witnesses,
            route_value: 0,
            dead_end_value: 0,
            condition_value: 0,
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
        assert_eq!(
            effect(&run(&mut self.body).0, &self.motors),
            [output],
            "start={start} output={output} at={at} cause={cause}"
        );
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
        let condition = self.body.held(self.closure).unwrap();
        assert_ne!(condition, 0);
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
        assert_eq!(self.body.held(self.closure), Some(condition));
    }

    fn return_closure(&mut self, at: u64, cause: u64) {
        assert_ne!(self.body.held(self.closure), Some(0));
        schedule(
            &mut self.body,
            at,
            &[Arrival::caused(self.closure, 0, cause)],
        );
        let (_, trace) = run(&mut self.body);
        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.decision == ReturnDecision::Accepted
                    && returned.source == self.closure
                    && returned.return_cause == Some(cause)
        )));
        self.condition_value = 0;
        assert_eq!(self.body.held(self.closure), Some(0));
    }

    fn open_condition(&mut self, at: u64, cause: u64) {
        self.condition_value = if self.condition_value == 1 { 2 } else { 1 };
        schedule(
            &mut self.body,
            at,
            &[Arrival::caused(self.closure, self.condition_value, cause)],
        );
        let (events, trace) = run(&mut self.body);
        assert!(effect(&events, &self.motors).is_empty());
        assert!(!trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
        )));
        assert_eq!(self.body.held(self.closure), Some(self.condition_value));
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

    fn probe_shared_with_present_condition(
        &mut self,
        at: u64,
        cause: u64,
        condition_cause: u64,
    ) -> (Vec<PhysicalEvent>, Vec<TraceEvent>) {
        self.condition_value = if self.condition_value == 1 { 2 } else { 1 };
        schedule(
            &mut self.body,
            at,
            &[
                reading(self.starts[Self::SHARED_START], 0, 1, cause),
                Arrival::caused(self.closure, self.condition_value, condition_cause),
            ],
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

fn outcome_witness(body: &mut Body, source: JunctionId, output: JunctionId) -> LinkId {
    let witness = body.add_link(Link::new(source, output, 0, 1)).unwrap();
    body.set_link_role(witness, LinkRole::OutcomeWitness)
        .unwrap();
    witness
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

    let (first, trace) = world.probe_shared_with_present_condition(40, 4, 92);
    assert_eq!(
        effect(&first, &world.motors),
        [PlanningWorld::ROUTE],
        "choices={:#?}",
        trace
            .iter()
            .filter(|event| matches!(event, TraceEvent::Choice(_)))
            .collect::<Vec<_>>()
    );
    let reentry = trace.iter().find_map(|event| match event {
        TraceEvent::Candidate(candidate) if candidate.reentries.len() == 1 => {
            Some(&candidate.reentries[0])
        }
        _ => None,
    });
    let reentry = reentry.expect("one executable continuation reaches the present condition");
    assert_eq!(reentry.condition, world.closure);
    assert_eq!(reentry.steps.len(), 2);
    assert_eq!(reentry.steps[0].returned_source, world.route);
    assert_eq!(reentry.steps[1].returned_source, world.closure);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Choice(choice) if choice.basis == Some(ChoiceBasis::UniqueReentry)
    )));
    assert!(!trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
    ) || matches!(event, TraceEvent::Strengthened(_))));
    assert_eq!(world.body.reentry_state().closed_steps, 3);
    assert_eq!(world.body.returns.live_count, 1);

    let terminal = world.return_route(44, 4);
    assert_eq!(effect(&terminal, &world.motors), [PlanningWorld::TERMINAL]);
    world.return_closure(48, 4);
    assert_eq!(world.body.held(world.closure), Some(0));
}

fn prepare_route(world: &mut PlanningWorld) {
    world.open_condition(5, 90);
    world.teach_start(PlanningWorld::SHARED_START, PlanningWorld::ROUTE, 10, 1);
    assert!(effect(&world.return_route(14, 1), &world.motors).is_empty());
    world.teach_terminal(20, 2);
    world.open_condition(30, 91);
    world.teach_start(PlanningWorld::SHARED_START, PlanningWorld::DEAD_END, 32, 3);
    world.return_dead_end(36, 3);
}

fn chosen_basis(trace: &[TraceEvent]) -> Option<ChoiceBasis> {
    trace.iter().find_map(|event| match event {
        TraceEvent::Choice(choice) if choice.sent => choice.basis,
        _ => None,
    })
}

#[test]
fn reentry_crosses_no_downstream_motor_and_opens_only_the_actual_return() {
    let mut world = PlanningWorld::new();
    prepare_route(&mut world);
    let strengths = world
        .outcome_witnesses
        .map(|link| world.body.link_memory[link.slot()].strength);

    let (events, trace) = world.probe_shared_with_present_condition(40, 4, 92);

    assert_eq!(effect(&events, &world.motors), [PlanningWorld::ROUTE]);
    assert_eq!(chosen_basis(&trace), Some(ChoiceBasis::UniqueReentry));
    assert_eq!(world.body.returns.live_count, 1);
    assert_eq!(
        world
            .outcome_witnesses
            .map(|link| world.body.link_memory[link.slot()].strength),
        strengths
    );
    assert!(!trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
    ) || matches!(event, TraceEvent::Strengthened(_))));
}

#[test]
fn unclosed_and_wrong_cause_steps_retain_no_reentry() {
    let mut unclosed = PlanningWorld::new();
    unclosed.teach_start(PlanningWorld::SHARED_START, PlanningWorld::ROUTE, 10, 1);
    assert_eq!(unclosed.body.reentry_state().closed_steps, 0);

    let mut wrong = PlanningWorld::new();
    wrong.teach_start(PlanningWorld::SHARED_START, PlanningWorld::ROUTE, 10, 1);
    wrong.return_route(14, 99);
    assert_eq!(wrong.body.reentry_state().closed_steps, 0);
}

#[test]
fn passive_timing_and_ambiguous_support_retain_no_reentry() {
    let mut passive = PlanningWorld::new();
    passive.teach_start(PlanningWorld::SHARED_START, PlanningWorld::ROUTE, 10, 1);
    passive.return_route(14, 0);
    assert_eq!(passive.body.reentry_state().closed_steps, 0);

    let mut ambiguous = PlanningWorld::new();
    outcome_witness(
        &mut ambiguous.body,
        ambiguous.route,
        ambiguous.motors[PlanningWorld::ROUTE].opportunity,
    );
    ambiguous.teach_start(PlanningWorld::SHARED_START, PlanningWorld::ROUTE, 10, 1);
    ambiguous.return_route(14, 1);
    assert_eq!(ambiguous.body.reentry_state().closed_steps, 0);
}

#[test]
fn ambiguous_returns_retain_no_reentry() {
    let mut world = PlanningWorld::new();
    world.teach_start(PlanningWorld::ROUTE_START, PlanningWorld::ROUTE, 10, 1);
    world.teach_start(PlanningWorld::ROUTE_START, PlanningWorld::ROUTE, 14, 2);
    world.route_value += 1;
    schedule(
        &mut world.body,
        18,
        &[reading(world.route, 0, world.route_value, 99)],
    );
    let (_, trace) = run(&mut world.body);

    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned)
            if returned.decision == ReturnDecision::Ambiguous && returned.open_paths == 2
    )));
    assert_eq!(world.body.reentry_state().closed_steps, 0);
}

#[test]
fn changed_support_invalidates_reentry_before_choice() {
    let mut world = PlanningWorld::new();
    prepare_route(&mut world);
    assert!(world.body.reentry_state().closed_steps > 0);
    world
        .body
        .set_link_role(world.outcome_witnesses[2], LinkRole::BoundaryWitness)
        .unwrap();
    assert_eq!(world.body.reentry_state().closed_steps, 2);

    let (events, trace) = world.probe_shared_with_present_condition(40, 4, 92);

    assert_eq!(effect(&events, &world.motors), [PlanningWorld::DEAD_END]);
    assert_ne!(chosen_basis(&trace), Some(ChoiceBasis::UniqueReentry));
}

#[test]
fn two_reaching_candidates_make_no_reentry_claim() {
    let mut world = PlanningWorld::new();
    prepare_route(&mut world);
    world
        .body
        .set_link_role(
            world.outcome_witnesses[PlanningWorld::DEAD_END],
            LinkRole::Drive,
        )
        .unwrap();
    outcome_witness(
        &mut world.body,
        world.closure,
        world.motors[PlanningWorld::DEAD_END].opportunity,
    );
    world.teach_start(PlanningWorld::SHARED_START, PlanningWorld::DEAD_END, 40, 5);
    world.return_closure(44, 5);
    assert_eq!(world.body.reentry_state().closed_steps, 3);

    let (events, trace) = world.probe_shared_with_present_condition(50, 6, 92);

    assert_ne!(chosen_basis(&trace), Some(ChoiceBasis::UniqueReentry));
    assert_eq!(effect(&events, &world.motors).len(), 1);
    verify_choice_laws(&trace).unwrap();
}

fn prepare_shortcut_route(world: &mut PlanningWorld) {
    world.open_condition(5, 90);
    world.teach_start(PlanningWorld::SHARED_START, PlanningWorld::ROUTE, 10, 1);
    world.return_route(14, 1);
    for (at, cause, condition_cause) in [(20, 2, 90), (40, 3, 91), (60, 4, 92)] {
        if at != 20 {
            world.open_condition(at - 2, condition_cause);
        }
        world.teach_terminal(at, cause);
    }
    world.open_condition(70, 94);
    world.teach_start(PlanningWorld::SHARED_START, PlanningWorld::DEAD_END, 72, 5);
    world.return_dead_end(76, 5);
}

fn composites(body: &Body) -> Vec<LinkId> {
    (0..body.arena.link_count())
        .filter_map(LinkId::new)
        .filter(|link| {
            matches!(
                body.link_memory[link.slot()].role,
                LinkRole::Composite { .. }
            )
        })
        .collect()
}

fn selected_reentry_work(trace: &[TraceEvent]) -> u16 {
    trace
        .iter()
        .find_map(|event| match event {
            TraceEvent::Candidate(candidate) if candidate.reentries.len() == 1 => {
                Some(candidate.reentry_incidence_visits)
            }
            _ => None,
        })
        .expect("the selected candidate has one witnessed continuation")
}

#[test]
fn an_existing_valid_shortcut_is_reused_without_changing_choice() {
    let mut shortcut = PlanningWorld::new();
    prepare_shortcut_route(&mut shortcut);
    let shortcut_links = composites(&shortcut.body);
    assert!(!shortcut_links.is_empty());
    let mut full = shortcut.clone();
    for link in composites(&full.body) {
        full.body.set_link_impulse(link, 0).unwrap();
    }

    let (full_events, full_trace) = full.probe_shared_with_present_condition(80, 6, 95);
    let (shortcut_events, shortcut_trace) = shortcut.probe_shared_with_present_condition(80, 6, 95);

    assert_eq!(effect(&full_events, &full.motors), [PlanningWorld::ROUTE]);
    assert_eq!(
        effect(&shortcut_events, &shortcut.motors),
        [PlanningWorld::ROUTE]
    );
    assert_eq!(chosen_basis(&full_trace), Some(ChoiceBasis::UniqueReentry));
    assert_eq!(
        chosen_basis(&shortcut_trace),
        Some(ChoiceBasis::UniqueReentry)
    );
    full.route_value += 1;
    schedule(
        &mut full.body,
        84,
        &[reading(full.route, 0, full.route_value, 6)],
    );
    let (full_return, full_reuse_trace) = run(&mut full.body);
    shortcut.route_value += 1;
    schedule(
        &mut shortcut.body,
        84,
        &[reading(shortcut.route, 0, shortcut.route_value, 6)],
    );
    let (shortcut_return, shortcut_reuse_trace) = run(&mut shortcut.body);
    assert_eq!(
        effect(&full_return, &full.motors),
        [PlanningWorld::TERMINAL]
    );
    assert_eq!(
        effect(&shortcut_return, &shortcut.motors),
        [PlanningWorld::TERMINAL]
    );
    assert!(full_reuse_trace.iter().all(|event| !matches!(
        event,
        TraceEvent::Arrival(arrival) if arrival.via.is_some_and(|via| shortcut_links.contains(&via))
    )));
    assert!(shortcut_reuse_trace.iter().any(|event| matches!(
        event,
        TraceEvent::Arrival(arrival) if arrival.via.is_some_and(|via| shortcut_links.contains(&via))
    )));
}

#[test]
fn many_disconnected_retained_histories_do_not_change_choice_or_local_reentry_work() {
    let mut plain = PlanningWorld::new();
    prepare_route(&mut plain);
    let mut product = plain.clone();
    for _ in 0..24 {
        let mut dormant = PlanningWorld::new();
        prepare_route(&mut dormant);
        let part = OpenBody::new(dormant.body, Vec::new()).unwrap();
        attach(&mut product.body, part, &[]).unwrap();
    }

    let (plain_events, plain_trace) = plain.probe_shared_with_present_condition(40, 4, 92);
    let (product_events, product_trace) = product.probe_shared_with_present_condition(40, 4, 92);

    assert_eq!(effect(&plain_events, &plain.motors), [PlanningWorld::ROUTE]);
    assert_eq!(
        effect(&product_events, &product.motors),
        [PlanningWorld::ROUTE]
    );
    assert_eq!(chosen_basis(&plain_trace), chosen_basis(&product_trace));
    assert_eq!(
        selected_reentry_work(&plain_trace),
        selected_reentry_work(&product_trace)
    );
}

#[test]
fn reentry_neither_creates_nor_repairs_a_shortcut() {
    let mut world = PlanningWorld::new();
    prepare_shortcut_route(&mut world);
    let composite = composites(&world.body)[0];
    world.body.set_link_impulse(composite, 0).unwrap();
    let formed = world.body.automaticity_work().composites_formed;

    let (_, trace) = world.probe_shared_with_present_condition(80, 6, 95);

    assert_eq!(chosen_basis(&trace), Some(ChoiceBasis::UniqueReentry));
    assert_eq!(world.body.automaticity_work().composites_formed, formed);
    assert_eq!(world.body.arena.link(composite).unwrap().impulse, 0);
}

#[test]
fn reentry_choice_is_invariant_to_renaming_and_independent_construction_order() {
    let mut semantic_first = PlanningWorld::new();
    prepare_route(&mut semantic_first);
    let dormant_from = semantic_first
        .body
        .add_junction(Junction::integrating(1))
        .unwrap();
    let dormant_to = semantic_first
        .body
        .add_junction(Junction::integrating(1))
        .unwrap();
    semantic_first
        .body
        .add_link(Link::new(dormant_from, dormant_to, 1, 1))
        .unwrap();

    let mut independent_first = PlanningWorld::with_dormant_prefix(true);
    prepare_route(&mut independent_first);

    let (left_events, left_trace) = semantic_first.probe_shared_with_present_condition(40, 4, 92);
    let (right_events, right_trace) =
        independent_first.probe_shared_with_present_condition(40, 4, 92);

    assert_eq!(
        effect(&left_events, &semantic_first.motors),
        effect(&right_events, &independent_first.motors)
    );
    assert_eq!(chosen_basis(&left_trace), chosen_basis(&right_trace));
    assert_eq!(
        semantic_first.body.reentry_state(),
        independent_first.body.reentry_state()
    );
}

#[test]
fn checkpoint_replays_the_exact_reentry_choice_and_trace() {
    let mut world = PlanningWorld::new();
    prepare_route(&mut world);
    let bytes = world.body.checkpoint().unwrap().canonical_bytes().unwrap();
    let restored = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();
    let mut replay = world.clone();
    replay.body = restored;

    let plain = world.probe_shared_with_present_condition(40, 4, 92);
    let repeated = replay.probe_shared_with_present_condition(40, 4, 92);

    assert_eq!(plain, repeated);
    assert_eq!(
        world.body.checkpoint().unwrap().canonical_bytes().unwrap(),
        replay.body.checkpoint().unwrap().canonical_bytes().unwrap()
    );
}

#[derive(Clone, Copy)]
struct AttachedPlanning {
    shared: JunctionId,
    condition: JunctionId,
    route_opportunity: JunctionId,
    dead_opportunity: JunctionId,
    route_effect: JunctionId,
    dead_effect: JunctionId,
}

fn attach_planning(host: &mut Body, world: PlanningWorld) -> AttachedPlanning {
    let ids = [
        world.starts[PlanningWorld::SHARED_START],
        world.closure,
        world.motors[PlanningWorld::ROUTE].opportunity,
        world.motors[PlanningWorld::DEAD_END].opportunity,
        world.motors[PlanningWorld::ROUTE].effect,
        world.motors[PlanningWorld::DEAD_END].effect,
    ];
    let part = OpenBody::new(world.body, ids.to_vec()).unwrap();
    let ports = std::array::from_fn::<_, 6, _>(|index| part.port(index).unwrap());
    let attached = attach(host, part, &[]).unwrap();
    let ids = ports.map(|port| attached.port(port).unwrap());
    AttachedPlanning {
        shared: ids[0],
        condition: ids[1],
        route_opportunity: ids[2],
        dead_opportunity: ids[3],
        route_effect: ids[4],
        dead_effect: ids[5],
    }
}

#[test]
fn attachment_remaps_reentry_support_and_independent_parts_remain_a_product() {
    let mut first = PlanningWorld::new();
    prepare_route(&mut first);
    let mut second = PlanningWorld::new();
    prepare_route(&mut second);
    let mut host = Body::default();
    let left = attach_planning(&mut host, first);
    let right = attach_planning(&mut host, second);
    assert_eq!(host.reentry_state().closed_steps, 6);

    schedule(
        &mut host,
        40,
        &[
            reading(left.shared, 0, 1, 4),
            Arrival::caused(left.condition, 2, 92),
            reading(right.shared, 0, 1, 5),
            Arrival::caused(right.condition, 2, 93),
        ],
    );
    schedule(
        &mut host,
        41,
        &[
            Arrival::caused(left.route_opportunity, 1, 4),
            Arrival::caused(left.dead_opportunity, 1, 4),
            Arrival::caused(right.route_opportunity, 1, 5),
            Arrival::caused(right.dead_opportunity, 1, 5),
        ],
    );
    let (events, trace) = run(&mut host);

    assert!(events
        .iter()
        .any(|event| event.junction == left.route_effect));
    assert!(events
        .iter()
        .any(|event| event.junction == right.route_effect));
    assert!(!events
        .iter()
        .any(|event| event.junction == left.dead_effect));
    assert!(!events
        .iter()
        .any(|event| event.junction == right.dead_effect));
    assert_eq!(
        trace
            .iter()
            .filter(|event| matches!(
                event,
                TraceEvent::Choice(choice)
                    if choice.basis == Some(ChoiceBasis::UniqueReentry)
            ))
            .count(),
        2
    );
}

struct ClosureWorld {
    body: Body,
    surfaces: [JunctionId; 3],
    motors: [Motor; 6],
    closures: [JunctionId; 3],
    closure_values: [i32; 3],
}

impl ClosureWorld {
    const PROXY: usize = 0;
    const CLOSER: usize = 1;

    const fn output(problem: usize, role: usize) -> usize {
        problem * 2 + role
    }

    fn new() -> Self {
        Self::with_reversed_order([false; 3])
    }

    fn with_reversed_order(reversed: [bool; 3]) -> Self {
        Self::with_forms(reversed, [1; 3])
    }

    fn with_forms(reversed: [bool; 3], distances: [u64; 3]) -> Self {
        Self::with_setup(reversed, distances, false)
    }

    fn with_dormant_prefix(prefix: bool) -> Self {
        Self::with_setup([false; 3], [1; 3], prefix)
    }

    fn with_setup(reversed: [bool; 3], distances: [u64; 3], prefix: bool) -> Self {
        let mut body = Body::default();
        if prefix {
            let from = body.add_junction(Junction::integrating(1)).unwrap();
            let to = body.add_junction(Junction::integrating(1)).unwrap();
            body.add_link(Link::new(from, to, 1, 1)).unwrap();
        }
        let motors = std::array::from_fn(|_| motor(&mut body));
        let surfaces = std::array::from_fn(|problem| {
            let roles = if reversed[problem] {
                [Self::CLOSER, Self::PROXY]
            } else {
                [Self::PROXY, Self::CLOSER]
            };
            attach_sensor(
                &mut body,
                Junction::integrating(1),
                &[
                    (
                        motors[Self::output(problem, roles[0])].opportunity,
                        distances[problem],
                    ),
                    (
                        motors[Self::output(problem, roles[1])].opportunity,
                        distances[problem],
                    ),
                ],
            )
        });
        let closures = std::array::from_fn(|problem| {
            let closure = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
            attach_outcome_component(
                &mut body,
                closure,
                [motors[Self::output(problem, Self::CLOSER)].opportunity],
            );
            closure
        });
        schedule(
            &mut body,
            0,
            &closures.map(|closure| Arrival::caused(closure, 0, 0)),
        );
        run(&mut body);
        Self {
            body,
            surfaces,
            motors,
            closures,
            closure_values: [0; 3],
        }
    }

    fn act(
        &mut self,
        problem: usize,
        roles: &[usize],
        at: u64,
        cause: u64,
    ) -> (Vec<usize>, Vec<TraceEvent>) {
        let outputs = roles
            .iter()
            .map(|role| Self::output(problem, *role))
            .collect::<Vec<_>>();
        self.act_from(self.surfaces[problem], &outputs, at, cause)
    }

    fn act_from(
        &mut self,
        surface: JunctionId,
        outputs: &[usize],
        at: u64,
        cause: u64,
    ) -> (Vec<usize>, Vec<TraceEvent>) {
        schedule(&mut self.body, at, &[reading(surface, 0, 1, cause)]);
        schedule(
            &mut self.body,
            at + 1,
            &outputs
                .iter()
                .map(|output| Arrival::caused(self.motors[*output].opportunity, 1, cause))
                .collect::<Vec<_>>(),
        );
        let (events, trace) = run(&mut self.body);
        (effect(&events, &self.motors), trace)
    }

    fn return_closure(&mut self, problem: usize, at: u64, cause: u64) -> Vec<TraceEvent> {
        self.closure_values[problem] += 1;
        schedule(
            &mut self.body,
            at,
            &[reading(
                self.closures[problem],
                0,
                self.closure_values[problem],
                cause,
            )],
        );
        let (_, trace) = run(&mut self.body);
        trace
    }

    fn demonstrate(&mut self, problem: usize, at: u64, cause: u64) {
        assert_eq!(
            self.act(problem, &[Self::PROXY], at, cause).0,
            [Self::output(problem, Self::PROXY)]
        );
        assert_eq!(
            self.act(problem, &[Self::CLOSER], at + 10, cause + 1).0,
            [Self::output(problem, Self::CLOSER)]
        );
        let return_at = (at + 12).max(self.body.now().saturating_add(1));
        let trace = self.return_closure(problem, return_at, cause + 1);
        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.decision == ReturnDecision::Accepted
                    && returned.source == self.closures[problem]
                    && returned.return_cause == Some(cause + 1)
        )));
    }

    fn probe(&mut self, problem: usize, at: u64, cause: u64) -> (Vec<usize>, Vec<TraceEvent>) {
        self.act(problem, &[Self::PROXY, Self::CLOSER], at, cause)
    }

    fn probe_with_present_condition_and_returns(
        &mut self,
        problem: usize,
        at: u64,
        cause: u64,
        condition_cause: u64,
        returns: &[(usize, u64)],
    ) -> (Vec<usize>, Vec<TraceEvent>) {
        self.closure_values[problem] += 1;
        let mut arrivals = vec![
            reading(self.surfaces[problem], 0, 1, cause),
            reading(
                self.closures[problem],
                0,
                self.closure_values[problem],
                condition_cause,
            ),
        ];
        for (returned_problem, return_cause) in returns {
            self.closure_values[*returned_problem] += 1;
            arrivals.push(reading(
                self.closures[*returned_problem],
                0,
                self.closure_values[*returned_problem],
                *return_cause,
            ));
        }
        schedule(&mut self.body, at, &arrivals);
        schedule(
            &mut self.body,
            at + 1,
            &[
                Arrival::caused(
                    self.motors[Self::output(problem, Self::PROXY)].opportunity,
                    1,
                    cause,
                ),
                Arrival::caused(
                    self.motors[Self::output(problem, Self::CLOSER)].opportunity,
                    1,
                    cause,
                ),
            ],
        );
        let (events, trace) = run(&mut self.body);
        (effect(&events, &self.motors), trace)
    }
}

fn direct_membership_parent(body: &Body, member: JunctionId) -> Option<JunctionId> {
    let mut parents = body
        .arena
        .incoming(member)
        .filter(|link| {
            body.link_memory[link.slot()].live
                && body.link_memory[link.slot()].role == LinkRole::Membership
        })
        .filter_map(|link| body.arena.link(link).map(|physical| physical.from));
    let parent = parents.next();
    assert!(parents.all(|candidate| Some(candidate) == parent));
    parent
}

fn composed_motifs(body: &Body) -> Vec<(LinkId, LinkId)> {
    body.link_memory
        .iter()
        .enumerate()
        .filter_map(|(slot, memory)| {
            memory
                .motif_parent()
                .map(|parent| (LinkId::new(slot).expect("live motif witness"), parent))
        })
        .collect()
}

fn motif_parent_from_output(body: &Body, output: JunctionId) -> Option<LinkId> {
    let mut parents = body
        .link_memory
        .iter()
        .enumerate()
        .filter_map(|(slot, memory)| {
            let link = LinkId::new(slot)?;
            (body.arena.link(link)?.from == output)
                .then(|| memory.motif_parent())
                .flatten()
        });
    let parent = parents.next();
    assert!(parents.next().is_none());
    parent
}

fn assert_unique_reentry_and_return(
    trace: &[TraceEvent],
    returned_source: JunctionId,
    decision: ReturnDecision,
) {
    assert_eq!(chosen_basis(trace), Some(ChoiceBasis::UniqueReentry));
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned)
            if returned.source == returned_source && returned.decision == decision
    )));
}

#[test]
fn one_unique_self_caused_closure_changes_later_choice_on_the_same_surface() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);

    assert_eq!(
        world.probe(0, 30, 3).0,
        [ClosureWorld::output(0, ClosureWorld::CLOSER)]
    );
}

#[test]
fn a_passive_closure_sample_creates_no_later_action_preference() {
    let mut world = ClosureWorld::new();
    world.return_closure(0, 10, 9);

    assert_eq!(
        world.probe(0, 20, 1).0,
        [ClosureWorld::output(0, ClosureWorld::PROXY)]
    );
}

#[test]
fn passive_timing_cannot_supply_the_second_example_for_generalization() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    let trace = world.return_closure(1, 30, 0);

    assert!(!trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
    )));
    assert_eq!(world.body.reentry_state().closed_steps, 1);
    assert!(composed_motifs(&world.body).is_empty());
    assert_eq!(
        world.probe(2, 40, 5).0,
        [ClosureWorld::output(2, ClosureWorld::PROXY)]
    );
}

#[test]
fn ambiguous_return_cannot_supply_the_second_example_for_generalization() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    let closer = ClosureWorld::output(1, ClosureWorld::CLOSER);
    let first = attach_sensor(
        &mut world.body,
        Junction::integrating(1),
        &[(world.motors[closer].opportunity, 1)],
    );
    let second = attach_sensor(
        &mut world.body,
        Junction::integrating(1),
        &[(world.motors[closer].opportunity, 1)],
    );
    assert_eq!(world.act_from(first, &[closer], 30, 3).0, [closer]);
    assert_eq!(world.act_from(second, &[closer], 34, 4).0, [closer]);
    let trace = world.return_closure(1, 38, 99);

    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Ambiguous
    )));
    assert_eq!(world.body.reentry_state().closed_steps, 1);
    assert!(composed_motifs(&world.body).is_empty());
    assert_eq!(
        world.probe(2, 40, 6).0,
        [ClosureWorld::output(2, ClosureWorld::PROXY)]
    );
}

#[test]
fn reversed_experience_order_does_not_create_a_generalization_claim() {
    let mut world = ClosureWorld::with_reversed_order([false, true, false]);
    world.demonstrate(0, 10, 1);
    let (events, first_trace) = world.act(1, &[ClosureWorld::CLOSER], 30, 3);
    assert_eq!(
        events,
        [ClosureWorld::output(1, ClosureWorld::CLOSER)],
        "trace={first_trace:#?}"
    );
    let trace = world.return_closure(1, 32, 3);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
    )));
    assert_eq!(
        world.act(1, &[ClosureWorld::PROXY], 40, 4).0,
        [ClosureWorld::output(1, ClosureWorld::PROXY)]
    );
    assert_eq!(world.body.reentry_state().closed_steps, 2);
    assert!(composed_motifs(&world.body).is_empty());

    let (events, trace) = world.probe(2, 50, 5);
    assert_eq!(events, [ClosureWorld::output(2, ClosureWorld::PROXY)]);
    assert_eq!(
        chosen_basis(&trace),
        Some(ChoiceBasis::ParticipationStrengthAndDrive)
    );
}

#[test]
fn reentry_without_an_accepted_second_return_forms_no_shared_membership() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);

    let (_, trace) = world.probe_with_present_condition_and_returns(0, 30, 3, 90, &[]);

    assert_eq!(chosen_basis(&trace), Some(ChoiceBasis::UniqueReentry));
    assert!(!trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned)
            if returned.source == world.closures[1]
                && returned.decision == ReturnDecision::Accepted
    )));
    assert_eq!(
        direct_membership_parent(&world.body, world.surfaces[0]),
        None
    );
    assert_eq!(
        direct_membership_parent(&world.body, world.surfaces[1]),
        None
    );
}

#[test]
fn passive_timing_during_reentry_forms_no_shared_membership() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);

    let (_, trace) = world.probe_with_present_condition_and_returns(0, 30, 3, 90, &[(1, 91)]);

    assert_eq!(chosen_basis(&trace), Some(ChoiceBasis::UniqueReentry));
    assert!(!trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned)
            if returned.source == world.closures[1]
                && returned.decision == ReturnDecision::Accepted
    )));
    assert_eq!(
        direct_membership_parent(&world.body, world.surfaces[0]),
        None
    );
    assert_eq!(
        direct_membership_parent(&world.body, world.surfaces[1]),
        None
    );
}

#[test]
fn ambiguous_return_during_reentry_forms_no_shared_membership() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    let closer = ClosureWorld::output(1, ClosureWorld::CLOSER);
    let first = attach_sensor(
        &mut world.body,
        Junction::integrating(1),
        &[(world.motors[closer].opportunity, 1)],
    );
    let second = attach_sensor(
        &mut world.body,
        Junction::integrating(1),
        &[(world.motors[closer].opportunity, 1)],
    );
    assert_eq!(world.act_from(first, &[closer], 30, 3).0, [closer]);
    assert_eq!(world.act_from(second, &[closer], 34, 4).0, [closer]);

    let (_, trace) = world.probe_with_present_condition_and_returns(0, 40, 5, 90, &[(1, 99)]);

    assert_unique_reentry_and_return(&trace, world.closures[1], ReturnDecision::Ambiguous);
    assert_eq!(
        direct_membership_parent(&world.body, world.surfaces[0]),
        None
    );
    assert_eq!(
        direct_membership_parent(&world.body, world.surfaces[1]),
        None
    );
}

#[test]
fn an_unrelated_exact_return_during_unique_reentry_forms_no_shared_membership() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    let closer = ClosureWorld::output(1, ClosureWorld::CLOSER);
    assert_eq!(
        world.act(1, &[ClosureWorld::PROXY], 26, 2).0,
        [ClosureWorld::output(1, ClosureWorld::PROXY)]
    );
    assert_eq!(world.act(1, &[ClosureWorld::CLOSER], 30, 3).0, [closer]);

    let (_, trace) = world.probe_with_present_condition_and_returns(0, 40, 4, 90, &[(1, 3)]);

    assert_unique_reentry_and_return(&trace, world.closures[1], ReturnDecision::Accepted);
    assert_eq!(
        direct_membership_parent(&world.body, world.surfaces[0]),
        None
    );
    assert_eq!(
        direct_membership_parent(&world.body, world.surfaces[1]),
        None
    );
}

#[test]
fn an_exact_return_that_is_the_reentry_condition_forms_shared_causal_membership() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    let closer = ClosureWorld::output(1, ClosureWorld::CLOSER);
    attach_outcome_component(
        &mut world.body,
        world.closures[0],
        [world.motors[closer].opportunity],
    );
    assert_eq!(
        world.act(1, &[ClosureWorld::PROXY], 26, 2).0,
        [ClosureWorld::output(1, ClosureWorld::PROXY)]
    );
    assert_eq!(world.act(1, &[ClosureWorld::CLOSER], 30, 3).0, [closer]);

    let (_, trace) = world.probe_with_present_condition_and_returns(0, 40, 4, 3, &[]);

    assert_unique_reentry_and_return(&trace, world.closures[0], ReturnDecision::Accepted);
    let first = direct_membership_parent(&world.body, world.surfaces[0]);
    let second = direct_membership_parent(&world.body, world.surfaces[1]);
    assert!(first.is_some(), "trace={trace:#?}");
    assert_eq!(first, second, "trace={trace:#?}");
}

#[test]
fn two_renamed_switch_then_close_histories_form_one_shared_causal_motif() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    assert!(world
        .body
        .link_memory
        .iter()
        .all(|memory| memory.motif_parent().is_none()));
    world.demonstrate(1, 30, 3);

    let composed = composed_motifs(&world.body);
    let [(second, first)] = composed.as_slice() else {
        panic!("expected one composed motif witness, got {composed:#?}");
    };
    assert_eq!(
        world.body.arena.link(*first).expect("first witness").from,
        world.motors[ClosureWorld::output(0, ClosureWorld::CLOSER)].opportunity
    );
    assert_eq!(
        world.body.arena.link(*second).expect("second witness").from,
        world.motors[ClosureWorld::output(1, ClosureWorld::CLOSER)].opportunity
    );
    assert_eq!(
        direct_membership_parent(&world.body, world.surfaces[0]),
        None
    );
    assert_eq!(
        direct_membership_parent(&world.body, world.surfaces[1]),
        None
    );
}

#[test]
fn motif_composition_ignores_identity_and_independent_construction() {
    let episode = |prefix| {
        let mut world = ClosureWorld::with_dormant_prefix(prefix);
        world.demonstrate(0, 10, 1);
        world.demonstrate(1, 30, 3);
        composed_motifs(&world.body).len()
    };

    assert_eq!(episode(false), 1);
    assert_eq!(episode(true), 1);
}

#[test]
fn a_changed_physical_path_form_does_not_compose_a_motif() {
    let mut world = ClosureWorld::with_forms([false; 3], [1, 2, 1]);
    world.demonstrate(0, 10, 1);
    world.demonstrate(1, 30, 3);

    assert!(composed_motifs(&world.body).is_empty());
}

#[test]
fn checkpoint_and_attachment_preserve_composed_motif_links() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    world.demonstrate(1, 30, 3);
    let expected = composed_motifs(&world.body);

    let bytes = world.body.checkpoint().unwrap().canonical_bytes().unwrap();
    let restored = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();
    assert_eq!(composed_motifs(&restored), expected);

    let mut host = Body::default();
    let part = OpenBody::new(restored, Vec::new()).unwrap();
    attach(&mut host, part, &[]).unwrap();
    let attached = composed_motifs(&host);
    assert_eq!(attached.len(), 1);
    assert!(host.arena.link(attached[0].0).is_some());
    assert!(host.arena.link(attached[0].1).is_some());
}

#[test]
fn disconnected_matching_histories_do_not_block_local_motif_composition() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    for _ in 0..8 {
        let mut dormant = ClosureWorld::new();
        dormant.demonstrate(0, 10, 1);
        dormant.demonstrate(1, 30, 3);
        let part = OpenBody::new(dormant.body, Vec::new()).unwrap();
        attach(&mut world.body, part, &[]).unwrap();
    }
    world.demonstrate(1, 50, 3);

    let output = world.motors[ClosureWorld::output(1, ClosureWorld::CLOSER)].opportunity;
    assert!(motif_parent_from_output(&world.body, output).is_some());
}

#[test]
#[ignore = "frontier: a retained renamed motif does not yet reenter a fresh causal instance"]
fn two_renamed_demonstrations_generalize_to_a_third_causal_instance() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    world.demonstrate(1, 30, 3);
    assert_eq!(world.body.reentry_state().closed_steps, 2);

    let (events, trace) = world.probe(2, 50, 5);
    assert_eq!(
        events,
        [ClosureWorld::output(2, ClosureWorld::CLOSER)],
        "choices={:#?}",
        trace
            .iter()
            .filter(|event| matches!(event, TraceEvent::Candidate(_) | TraceEvent::Choice(_)))
            .collect::<Vec<_>>()
    );
    assert_eq!(world.body.reentry_state().closed_steps, 2);
    assert!(!trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
    ) || matches!(event, TraceEvent::Strengthened(_))));
}
