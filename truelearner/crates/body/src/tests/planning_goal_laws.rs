//! Unit-test ladder for the physical prerequisites of planning and goal discovery.
//!
//! The active tests name laws the body already supports. Ignored tests freeze the
//! next unsupported transitions without adding a planner, goal object, reward,
//! episode verdict, or evaluator state to the learner.

use crate::{
    attach,
    harness::{attach_outcome_component, attach_sensor, effect, motor, reading, schedule, Motor},
    verify_choice_contract, Arrival, ArrowKind, ArrowState, Body, BodyCheckpoint, ChoiceWarrant,
    Junction, JunctionId, Link, LinkId, OpenBody, PhysicalEvent, ReturnDecision, ReturnStatus,
    TraceEvent, WitnessKind,
};

fn run(body: &mut Body) -> (Vec<PhysicalEvent>, Vec<TraceEvent>) {
    let mut events = Vec::new();
    let mut trace = Vec::new();
    body.run_traced(256, |event| events.push(event), |event| trace.push(event))
        .unwrap();
    assert!(body.is_quiet());
    verify_choice_contract(&trace).unwrap();
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
    body.mark_witness(
        witness,
        WitnessKind::Closure {
            offers_choice: true,
        },
    )
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
        TraceEvent::Choice(choice) if choice.warrant == Some(ChoiceWarrant::Reentry)
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

fn chosen_warrant(trace: &[TraceEvent]) -> Option<ChoiceWarrant> {
    trace.iter().find_map(|event| match event {
        TraceEvent::Choice(choice) if choice.sent => choice.warrant,
        _ => None,
    })
}

#[test]
fn reentry_crosses_no_downstream_motor_and_opens_only_the_actual_return() {
    let mut world = PlanningWorld::new();
    prepare_route(&mut world);
    let strengths = world
        .outcome_witnesses
        .map(|link| world.body.arrows[link.slot()].strength());

    let (events, trace) = world.probe_shared_with_present_condition(40, 4, 92);

    assert_eq!(effect(&events, &world.motors), [PlanningWorld::ROUTE]);
    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
    assert_eq!(world.body.returns.live_count, 1);
    assert_eq!(
        world
            .outcome_witnesses
            .map(|link| world.body.arrows[link.slot()].strength()),
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
    assert_eq!(
        world
            .body
            .arrows
            .iter()
            .filter(|state| matches!(
                state.kind(),
                ArrowKind::Return {
                    status: ReturnStatus::Ambiguous { at: 18 },
                    ..
                }
            ))
            .count(),
        2
    );
}

#[test]
fn changed_support_invalidates_reentry_before_choice() {
    let mut world = PlanningWorld::new();
    prepare_route(&mut world);
    assert!(world.body.reentry_state().closed_steps > 0);
    world
        .body
        .mark_witness(
            world.outcome_witnesses[2],
            WitnessKind::Closure {
                offers_choice: false,
            },
        )
        .unwrap();
    assert_eq!(world.body.reentry_state().closed_steps, 2);

    let (events, trace) = world.probe_shared_with_present_condition(40, 4, 92);

    assert_eq!(effect(&events, &world.motors), [PlanningWorld::DEAD_END]);
    assert_ne!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
}

#[test]
fn two_reaching_candidates_make_no_reentry_claim() {
    let mut world = PlanningWorld::new();
    prepare_route(&mut world);
    world
        .body
        .replace_arrow_state(
            world.outcome_witnesses[PlanningWorld::DEAD_END],
            ArrowState::drive(),
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

    assert_ne!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
    assert_eq!(effect(&events, &world.motors).len(), 1);
    verify_choice_contract(&trace).unwrap();
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
        .filter(|link| body.arrows[link.slot()].factors().is_some())
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
    assert_eq!(chosen_warrant(&full_trace), Some(ChoiceWarrant::Reentry));
    assert_eq!(
        chosen_warrant(&shortcut_trace),
        Some(ChoiceWarrant::Reentry)
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
    assert_eq!(chosen_warrant(&plain_trace), chosen_warrant(&product_trace));
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

    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
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
    assert_eq!(chosen_warrant(&left_trace), chosen_warrant(&right_trace));
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
                    if choice.warrant == Some(ChoiceWarrant::Reentry)
            ))
            .count(),
        2
    );
}

#[derive(Clone)]
struct BranchingWorld {
    body: Body,
    shared: JunctionId,
    x: JunctionId,
    y: JunctionId,
    dead_x: JunctionId,
    dead_y: JunctionId,
    goal: JunctionId,
    motors: [Motor; 6],
    values: [i32; 5],
}

impl BranchingWorld {
    const A: usize = 0;
    const B: usize = 1;
    const X_DEAD: usize = 2;
    const X_GOAL: usize = 3;
    const Y_DEAD: usize = 4;
    const Y_GOAL: usize = 5;

    const X: usize = 0;
    const Y: usize = 1;
    const DEAD_X: usize = 2;
    const DEAD_Y: usize = 3;
    const GOAL: usize = 4;

    fn new(reverse_motor_identities: bool) -> Self {
        let mut body = Body::default();
        let raw_motors = std::array::from_fn(|_| motor(&mut body));
        let motors = if reverse_motor_identities {
            std::array::from_fn(|index| raw_motors[5 - index])
        } else {
            raw_motors
        };
        let shared = attach_sensor(
            &mut body,
            Junction::integrating(1),
            &[(motors[Self::A].opportunity, 1)],
        );
        let x = attach_sensor(
            &mut body,
            Junction::sampled(1_000),
            &[(motors[Self::X_DEAD].opportunity, 1)],
        );
        let y = attach_sensor(
            &mut body,
            Junction::sampled(1_000),
            &[(motors[Self::Y_DEAD].opportunity, 1)],
        );
        let dead_x = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        let dead_y = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        let goal = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        for (source, output) in [
            (x, Self::A),
            (y, Self::B),
            (dead_x, Self::X_DEAD),
            (goal, Self::X_GOAL),
            (dead_y, Self::Y_DEAD),
            (goal, Self::Y_GOAL),
        ] {
            outcome_witness(&mut body, source, motors[output].opportunity);
        }
        schedule(
            &mut body,
            0,
            &[
                Arrival::caused(x, 0, 0),
                Arrival::caused(y, 0, 0),
                Arrival::caused(dead_x, 0, 0),
                Arrival::caused(dead_y, 0, 0),
                Arrival::caused(goal, 0, 0),
            ],
        );
        run(&mut body);
        Self {
            body,
            shared,
            x,
            y,
            dead_x,
            dead_y,
            goal,
            motors,
            values: [0; 5],
        }
    }

    fn add_action(&mut self, surface: JunctionId, output: usize) {
        self.body
            .add_link(Link::new(surface, self.motors[output].opportunity, 1, 0))
            .unwrap();
    }

    fn choose(
        &mut self,
        surface: JunctionId,
        value: i32,
        outputs: &[usize],
        at: u64,
        cause: u64,
    ) -> (Vec<PhysicalEvent>, Vec<TraceEvent>) {
        schedule(&mut self.body, at, &[reading(surface, 0, value, cause)]);
        let opportunities = outputs
            .iter()
            .map(|output| Arrival::caused(self.motors[*output].opportunity, 1, cause))
            .collect::<Vec<_>>();
        schedule(&mut self.body, at + 1, &opportunities);
        run(&mut self.body)
    }

    fn return_from(&mut self, source: usize, at: u64, cause: u64) {
        self.values[source] += 1;
        let junction = match source {
            Self::X => self.x,
            Self::Y => self.y,
            Self::DEAD_X => self.dead_x,
            Self::DEAD_Y => self.dead_y,
            Self::GOAL => self.goal,
            _ => unreachable!("known branching-world source"),
        };
        schedule(
            &mut self.body,
            at,
            &[reading(junction, 0, self.values[source], cause)],
        );
        let (_, trace) = run(&mut self.body);
        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.decision == ReturnDecision::Accepted
                    && returned.source == junction
                    && returned.return_cause == Some(cause)
        )));
    }

    fn train(&mut self) {
        let (events, _) = self.choose(self.shared, 1, &[Self::A], 10, 1);
        assert_eq!(effect(&events, &self.motors), [Self::A]);
        self.return_from(Self::X, 14, 1);

        self.add_action(self.shared, Self::B);
        let (events, _) = self.choose(self.shared, 1, &[Self::A, Self::B], 20, 2);
        assert_eq!(effect(&events, &self.motors), [Self::B]);
        self.return_from(Self::Y, 24, 2);

        self.values[Self::X] += 1;
        let (events, _) = self.choose(self.x, self.values[Self::X], &[Self::X_DEAD], 30, 3);
        assert_eq!(effect(&events, &self.motors), [Self::X_DEAD]);
        self.return_from(Self::DEAD_X, 34, 3);

        self.add_action(self.x, Self::X_GOAL);
        self.values[Self::X] += 1;
        let (events, _) = self.choose(
            self.x,
            self.values[Self::X],
            &[Self::X_DEAD, Self::X_GOAL],
            40,
            4,
        );
        assert_eq!(effect(&events, &self.motors), [Self::X_GOAL]);
        self.return_from(Self::GOAL, 44, 4);

        self.values[Self::Y] += 1;
        let (events, _) = self.choose(self.y, self.values[Self::Y], &[Self::Y_DEAD], 50, 5);
        assert_eq!(effect(&events, &self.motors), [Self::Y_DEAD]);
        self.return_from(Self::DEAD_Y, 54, 5);
    }

    fn add_goal_below_y(&mut self) {
        self.add_action(self.y, Self::Y_GOAL);
        self.values[Self::Y] += 1;
        let (events, _) = self.choose(
            self.y,
            self.values[Self::Y],
            &[Self::Y_DEAD, Self::Y_GOAL],
            60,
            6,
        );
        assert_eq!(effect(&events, &self.motors), [Self::Y_GOAL]);
        self.return_from(Self::GOAL, 64, 6);
    }

    fn probe(&mut self, at: u64, cause: u64) -> (Vec<PhysicalEvent>, Vec<TraceEvent>) {
        self.values[Self::GOAL] += 1;
        schedule(
            &mut self.body,
            at,
            &[
                reading(self.shared, 0, 1, cause),
                reading(self.goal, 0, self.values[Self::GOAL], cause + 100),
            ],
        );
        schedule(
            &mut self.body,
            at + 1,
            &[
                Arrival::caused(self.motors[Self::A].opportunity, 1, cause),
                Arrival::caused(self.motors[Self::B].opportunity, 1, cause),
            ],
        );
        run(&mut self.body)
    }
}

#[test]
fn reentry_looks_through_a_remembered_branch_without_enacting_it() {
    for reverse_motor_identities in [false, true] {
        let mut world = BranchingWorld::new(reverse_motor_identities);
        world.train();
        let closed_steps = world.body.reentry_state().closed_steps;

        let (events, trace) = world.probe(70, 7);

        assert_eq!(effect(&events, &world.motors), [BranchingWorld::A]);
        assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
        let reentry = trace.iter().find_map(|event| match event {
            TraceEvent::Candidate(candidate)
                if candidate.path.output == world.motors[BranchingWorld::A].opportunity
                    && candidate.reentries.len() == 1 =>
            {
                Some(&candidate.reentries[0])
            }
            _ => None,
        });
        let reentry = reentry.expect("A has one remembered continuation to the present goal");
        assert_eq!(reentry.condition, world.goal);
        assert_eq!(reentry.steps.len(), 2);
        assert_eq!(reentry.steps[0].returned_source, world.x);
        assert_eq!(reentry.steps[1].returned_source, world.goal);
        assert!(events.iter().all(|event| ![
            world.x,
            world.y,
            world.dead_x,
            world.dead_y,
            world.motors[BranchingWorld::X_DEAD].effect,
            world.motors[BranchingWorld::X_GOAL].effect,
            world.motors[BranchingWorld::Y_DEAD].effect,
            world.motors[BranchingWorld::Y_GOAL].effect,
        ]
        .contains(&event.junction)));
        assert_eq!(world.body.reentry_state().closed_steps, closed_steps);
        assert!(!trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
        ) || matches!(event, TraceEvent::Strengthened(_))));
    }
}

#[test]
fn two_remembered_goal_branches_make_no_unique_planning_claim() {
    let mut world = BranchingWorld::new(false);
    world.train();
    world.add_goal_below_y();

    let (events, trace) = world.probe(70, 7);

    assert_eq!(effect(&events, &world.motors).len(), 1);
    assert_ne!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
}

#[derive(Clone)]
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
        .filter(|link| body.arrows[link.slot()].is_membership())
        .filter_map(|link| body.arena.link(link).map(|physical| physical.from));
    let parent = parents.next();
    assert!(parents.all(|candidate| Some(candidate) == parent));
    parent
}

fn composed_motifs(body: &Body) -> Vec<(LinkId, LinkId)> {
    body.arrows
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
    let mut parents = body.arrows.iter().enumerate().filter_map(|(slot, memory)| {
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
    assert_eq!(chosen_warrant(trace), Some(ChoiceWarrant::Reentry));
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
    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::LocalIncidence));
}

#[test]
fn reentry_without_an_accepted_second_return_forms_no_shared_membership() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);

    let (_, trace) = world.probe_with_present_condition_and_returns(0, 30, 3, 90, &[]);

    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
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

    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
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
        .arrows
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
    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
    assert_eq!(
        trace
            .iter()
            .filter_map(|event| match event {
                TraceEvent::Candidate(candidate) if !candidate.motif_reentries.is_empty() => {
                    Some(candidate)
                }
                _ => None,
            })
            .count(),
        1
    );
    assert_eq!(world.body.reentry_state().closed_steps, 2);
    assert!(!trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
    ) || matches!(event, TraceEvent::Strengthened(_))));
}

#[test]
fn one_demonstration_cannot_reenter_a_fresh_causal_instance() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);

    let (events, trace) = world.probe(2, 30, 3);

    assert_eq!(events, [ClosureWorld::output(2, ClosureWorld::PROXY)]);
    assert_ne!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
    assert!(trace.iter().all(|event| !matches!(
        event,
        TraceEvent::Candidate(candidate) if !candidate.motif_reentries.is_empty()
    )));
}

#[test]
fn a_changed_fresh_path_form_cannot_receive_motif_reentry() {
    let mut world = ClosureWorld::with_forms([false; 3], [1, 1, 2]);
    world.demonstrate(0, 10, 1);
    world.demonstrate(1, 30, 3);

    let (events, trace) = world.probe(2, 50, 5);

    assert_eq!(events, [ClosureWorld::output(2, ClosureWorld::PROXY)]);
    assert_ne!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
}

#[test]
fn changed_fresh_outcome_roles_make_no_motif_choice() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    world.demonstrate(1, 30, 3);
    attach_outcome_component(
        &mut world.body,
        world.closures[2],
        [world.motors[ClosureWorld::output(2, ClosureWorld::PROXY)].opportunity],
    );

    let (events, trace) = world.probe(2, 50, 5);

    assert_eq!(events, [ClosureWorld::output(2, ClosureWorld::PROXY)]);
    assert_ne!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
    assert!(trace.iter().all(|event| !matches!(
        event,
        TraceEvent::Candidate(candidate) if !candidate.motif_reentries.is_empty()
    )));
}

#[test]
fn motif_reentry_is_invariant_to_renaming_and_construction_order() {
    let mut world = ClosureWorld::with_reversed_order([false, false, true]);
    world.demonstrate(0, 10, 1);
    world.demonstrate(1, 30, 3);

    let (events, trace) = world.probe(2, 50, 5);

    assert_eq!(events, [ClosureWorld::output(2, ClosureWorld::CLOSER)]);
    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
}

#[test]
fn disconnected_matching_motifs_add_support_without_selecting_a_parent() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    world.demonstrate(1, 30, 3);
    for _ in 0..4 {
        let mut dormant = ClosureWorld::new();
        dormant.demonstrate(0, 10, 1);
        dormant.demonstrate(1, 30, 3);
        let part = OpenBody::new(dormant.body, Vec::new()).unwrap();
        attach(&mut world.body, part, &[]).unwrap();
    }

    let at = world.body.now() + 10;
    let (events, trace) = world.probe(2, at, 5);
    let support = trace.iter().find_map(|event| match event {
        TraceEvent::Candidate(candidate) if !candidate.motif_reentries.is_empty() => {
            Some(&candidate.motif_reentries)
        }
        _ => None,
    });

    assert_eq!(events, [ClosureWorld::output(2, ClosureWorld::CLOSER)]);
    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
    assert!(support.is_some_and(|support| support.len() > 1));
}

#[test]
fn an_overlarge_motif_search_fails_closed_without_a_choice_claim() {
    let mut world = ClosureWorld::new();
    world.demonstrate(0, 10, 1);
    world.demonstrate(1, 30, 3);
    for _ in 0..300 {
        let from = world.body.add_junction(Junction::integrating(1)).unwrap();
        let to = world.body.add_junction(Junction::integrating(1)).unwrap();
        world.body.add_link(Link::new(from, to, 1, 1)).unwrap();
    }

    let (events, trace) = world.probe(2, 50, 5);

    assert_eq!(events, [ClosureWorld::output(2, ClosureWorld::PROXY)]);
    assert_ne!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Candidate(candidate)
            if candidate.new_path
                && candidate.motif_reentries.is_empty()
                && candidate.reentry_incidence_visits == 256
    )));
}

#[test]
fn checkpoint_replays_the_exact_fresh_motif_choice() {
    let mut plain = ClosureWorld::new();
    plain.demonstrate(0, 10, 1);
    plain.demonstrate(1, 30, 3);
    let bytes = plain.body.checkpoint().unwrap().canonical_bytes().unwrap();
    let mut restored = plain.clone();
    restored.body = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();

    let plain_result = plain.probe(2, 50, 5);
    let restored_result = restored.probe(2, 50, 5);

    assert_eq!(plain_result, restored_result);
    assert_eq!(
        chosen_warrant(&plain_result.1),
        Some(ChoiceWarrant::Reentry)
    );
}

#[test]
fn only_a_fresh_exact_return_confirms_tentative_motif_reentry() {
    let mut learned = ClosureWorld::new();
    learned.demonstrate(0, 10, 1);
    learned.demonstrate(1, 30, 3);
    assert_eq!(
        learned.probe(2, 50, 5).0,
        [ClosureWorld::output(2, ClosureWorld::CLOSER)]
    );
    assert_eq!(learned.body.reentry_state().closed_steps, 2);

    let mut exact = learned.clone();
    let trace = exact.return_closure(2, 54, 5);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned)
            if returned.decision == ReturnDecision::Accepted
                && returned.return_cause == Some(5)
    )));
    assert_eq!(exact.body.reentry_state().closed_steps, 3);

    let mut ambiguous = learned.clone();
    let closer = ClosureWorld::output(2, ClosureWorld::CLOSER);
    let extra = attach_sensor(
        &mut ambiguous.body,
        Junction::integrating(1),
        &[(ambiguous.motors[closer].opportunity, 1)],
    );
    assert_eq!(ambiguous.act_from(extra, &[closer], 54, 6).0, [closer]);
    let trace = ambiguous.return_closure(2, 58, 99);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Ambiguous
    )));
    assert_eq!(ambiguous.body.reentry_state().closed_steps, 2);

    let mut wrong_cause = learned;
    let trace = wrong_cause.return_closure(2, 54, 99);
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned)
            if returned.decision == ReturnDecision::Accepted
                && returned.return_cause == Some(5)
    )));
    assert_eq!(wrong_cause.body.reentry_state().closed_steps, 2);
}

#[derive(Clone)]
struct MotifChainWorld {
    body: Body,
    motors: Vec<Motor>,
    starts: [JunctionId; 3],
    intermediate: [JunctionId; 4],
    outcomes: [JunctionId; 4],
}

impl MotifChainWorld {
    const FIRST_PROXY_0: usize = 0;
    const FIRST_CLOSER_0: usize = 1;
    const FIRST_PROXY_1: usize = 2;
    const FIRST_CLOSER_1: usize = 3;
    const SECOND_PROXY_0: usize = 4;
    const SECOND_CLOSER_0: usize = 5;
    const SECOND_PROXY_1: usize = 6;
    const SECOND_CLOSER_1: usize = 7;
    const FRESH_PROXY: usize = 8;
    const FRESH_LEFT: usize = 9;
    const FRESH_RIGHT: usize = 10;
    const LEFT_PROXY: usize = 11;
    const LEFT_CLOSER: usize = 12;
    const RIGHT_PROXY: usize = 13;
    const RIGHT_CLOSER: usize = 14;

    const MID_0: usize = 0;
    const MID_1: usize = 1;
    const LEFT: usize = 2;
    const RIGHT: usize = 3;

    const GOAL_0: usize = 0;
    const GOAL_1: usize = 1;
    const FRESH_GOAL: usize = 2;
    const DEAD: usize = 3;

    fn new() -> Self {
        Self::with_setup(false, false)
    }

    fn with_setup(dormant_prefix: bool, reverse_fresh_construction: bool) -> Self {
        let mut body = Body::default();
        if dormant_prefix {
            let from = body.add_junction(Junction::integrating(1)).unwrap();
            let to = body.add_junction(Junction::integrating(1)).unwrap();
            body.add_link(Link::new(from, to, 1, 1)).unwrap();
        }
        let motors = (0..15).map(|_| motor(&mut body)).collect::<Vec<_>>();
        let starts =
            std::array::from_fn(|_| attach_sensor(&mut body, Junction::integrating(1), &[]));
        let intermediate =
            std::array::from_fn(|_| attach_sensor(&mut body, Junction::integrating(1), &[]));
        let outcomes =
            std::array::from_fn(|_| attach_sensor(&mut body, Junction::sampled(1_000), &[]));

        for (surface, output) in [
            (starts[0], Self::FIRST_PROXY_0),
            (starts[0], Self::FIRST_CLOSER_0),
            (starts[1], Self::FIRST_PROXY_1),
            (starts[1], Self::FIRST_CLOSER_1),
        ] {
            body.add_link(Link::new(surface, motors[output].opportunity, 1, 0))
                .unwrap();
        }
        let fresh = if reverse_fresh_construction {
            [Self::FRESH_RIGHT, Self::FRESH_LEFT]
        } else {
            [Self::FRESH_LEFT, Self::FRESH_RIGHT]
        };
        for output in [Self::FRESH_PROXY, fresh[0], fresh[1]] {
            body.add_link(Link::new(starts[2], motors[output].opportunity, 1, 0))
                .unwrap();
        }
        for (source, output) in [
            (intermediate[Self::MID_0], Self::FIRST_CLOSER_0),
            (intermediate[Self::MID_1], Self::FIRST_CLOSER_1),
            (intermediate[Self::LEFT], Self::FRESH_LEFT),
            (intermediate[Self::RIGHT], Self::FRESH_RIGHT),
        ] {
            outcome_witness(&mut body, source, motors[output].opportunity);
        }
        schedule(
            &mut body,
            0,
            &outcomes.map(|outcome| Arrival::caused(outcome, 0, 0)),
        );
        run(&mut body);
        Self {
            body,
            motors,
            starts,
            intermediate,
            outcomes,
        }
    }

    fn act(
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

    fn demonstrate(
        &mut self,
        surface: JunctionId,
        proxy: usize,
        closer: usize,
        outcome: JunctionId,
        at: u64,
        cause: u64,
    ) {
        assert_eq!(self.act(surface, &[proxy], at, cause).0, [proxy]);
        assert_eq!(self.act(surface, &[closer], at + 10, cause + 1).0, [closer]);
        schedule(
            &mut self.body,
            at + 14,
            &[reading(outcome, 0, 1, cause + 1)],
        );
        let (_, trace) = run(&mut self.body);
        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.source == outcome
                    && returned.return_cause == Some(cause + 1)
                    && returned.decision == ReturnDecision::Accepted
        )));
    }

    fn train_first_motif(&mut self) {
        self.demonstrate(
            self.starts[0],
            Self::FIRST_PROXY_0,
            Self::FIRST_CLOSER_0,
            self.intermediate[Self::MID_0],
            10,
            1,
        );
        self.demonstrate(
            self.starts[1],
            Self::FIRST_PROXY_1,
            Self::FIRST_CLOSER_1,
            self.intermediate[Self::MID_1],
            30,
            3,
        );
        for (surface, output) in [
            (self.intermediate[Self::MID_0], Self::SECOND_PROXY_0),
            (self.intermediate[Self::MID_0], Self::SECOND_CLOSER_0),
            (self.intermediate[Self::MID_1], Self::SECOND_PROXY_1),
            (self.intermediate[Self::MID_1], Self::SECOND_CLOSER_1),
            (self.intermediate[Self::LEFT], Self::LEFT_PROXY),
            (self.intermediate[Self::LEFT], Self::LEFT_CLOSER),
            (self.intermediate[Self::RIGHT], Self::RIGHT_PROXY),
            (self.intermediate[Self::RIGHT], Self::RIGHT_CLOSER),
        ] {
            self.body
                .add_link(Link::new(surface, self.motors[output].opportunity, 2, 0))
                .unwrap();
        }
        for (source, output) in [
            (self.outcomes[Self::GOAL_0], Self::SECOND_CLOSER_0),
            (self.outcomes[Self::GOAL_1], Self::SECOND_CLOSER_1),
            (self.outcomes[Self::FRESH_GOAL], Self::LEFT_CLOSER),
            (self.outcomes[Self::DEAD], Self::RIGHT_CLOSER),
        ] {
            outcome_witness(&mut self.body, source, self.motors[output].opportunity);
        }
    }

    fn train_one_second_example(&mut self) {
        self.demonstrate(
            self.intermediate[Self::MID_0],
            Self::SECOND_PROXY_0,
            Self::SECOND_CLOSER_0,
            self.outcomes[Self::GOAL_0],
            50,
            5,
        );
    }

    fn train_second_motif(&mut self) {
        self.train_one_second_example();
        self.demonstrate(
            self.intermediate[Self::MID_1],
            Self::SECOND_PROXY_1,
            Self::SECOND_CLOSER_1,
            self.outcomes[Self::GOAL_1],
            70,
            7,
        );
    }

    fn train(&mut self) {
        self.train_first_motif();
        self.train_second_motif();
        assert_eq!(composed_motifs(&self.body).len(), 2);
    }

    fn make_right_reach_goal(&mut self) {
        let output = self.motors[Self::RIGHT_CLOSER].opportunity;
        let dead = self.outcomes[Self::DEAD];
        let witness = self
            .body
            .arena
            .incoming(output)
            .find(|link| {
                self.body.arrows[link.slot()].witness_kind()
                    == Some(WitnessKind::Closure {
                        offers_choice: true,
                    })
                    && self
                        .body
                        .arena
                        .link(*link)
                        .is_some_and(|physical| physical.from == dead)
            })
            .expect("right closer has its dead outcome witness");
        self.body
            .mark_witness(
                witness,
                WitnessKind::Closure {
                    offers_choice: false,
                },
            )
            .unwrap();
        outcome_witness(&mut self.body, self.outcomes[Self::FRESH_GOAL], output);
    }

    fn make_left_outcome_ambiguous(&mut self) {
        outcome_witness(
            &mut self.body,
            self.outcomes[Self::DEAD],
            self.motors[Self::LEFT_CLOSER].opportunity,
        );
    }

    fn path_entries_from(&self, surface: JunctionId) -> usize {
        let mut count = 0;
        let mut next = self
            .body
            .arena
            .junction(surface)
            .and_then(|junction| junction.outgoing_head);
        while let Some(link) = next {
            let physical = self.body.arena.link(link).expect("live path incidence");
            next = physical.next;
            count += usize::from(self.body.arrows[link.slot()].is_entry());
        }
        count
    }

    fn remap_junctions(&mut self, base: usize) {
        let remap = |junction: JunctionId| {
            JunctionId::new(base + junction.slot()).expect("validated attachment identity")
        };
        self.starts = self.starts.map(remap);
        self.intermediate = self.intermediate.map(remap);
        self.outcomes = self.outcomes.map(remap);
        for motor in &mut self.motors {
            motor.opportunity = remap(motor.opportunity);
            motor.effect = remap(motor.effect);
        }
    }

    fn probe(&mut self, at: u64, cause: u64) -> (Vec<usize>, Vec<TraceEvent>) {
        schedule(
            &mut self.body,
            at,
            &[
                reading(self.starts[2], 0, 1, cause),
                reading(self.outcomes[Self::FRESH_GOAL], 0, 1, cause + 100),
            ],
        );
        schedule(
            &mut self.body,
            at + 1,
            &[
                Arrival::caused(self.motors[Self::FRESH_PROXY].opportunity, 1, cause),
                Arrival::caused(self.motors[Self::FRESH_LEFT].opportunity, 1, cause),
                Arrival::caused(self.motors[Self::FRESH_RIGHT].opportunity, 1, cause),
            ],
        );
        let (events, trace) = run(&mut self.body);
        (effect(&events, &self.motors), trace)
    }
}

#[test]
fn two_identity_free_motifs_compose_to_select_the_only_reaching_first_step() {
    let mut world = MotifChainWorld::new();
    world.train();
    let closed_before = world.body.reentry_state().closed_steps;
    assert_eq!(
        world.path_entries_from(world.intermediate[MotifChainWorld::LEFT]),
        0
    );

    let (events, trace) = world.probe(90, 9);

    assert_eq!(
        events,
        [MotifChainWorld::FRESH_LEFT],
        "choices={:#?}",
        trace
            .iter()
            .filter(|event| matches!(event, TraceEvent::Candidate(_) | TraceEvent::Choice(_)))
            .collect::<Vec<_>>()
    );
    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
    let route = trace.iter().find_map(|event| match event {
        TraceEvent::Candidate(candidate)
            if candidate.path.output == world.motors[MotifChainWorld::FRESH_LEFT].opportunity
                && candidate.motif_routes.len() == 1 =>
        {
            Some(&candidate.motif_routes[0])
        }
        _ => None,
    });
    let route = route.expect("the left candidate has one composed motif route");
    assert_eq!(route.condition, world.outcomes[MotifChainWorld::FRESH_GOAL]);
    assert_eq!(route.steps.len(), 1);
    assert_eq!(
        route.steps[0].surface,
        world.intermediate[MotifChainWorld::LEFT]
    );
    assert_eq!(
        route.steps[0].output,
        world.motors[MotifChainWorld::LEFT_CLOSER].opportunity
    );
    assert_eq!(route.steps[0].outcome_source, route.condition);
    assert!(!route.steps[0].supports.is_empty());
    assert_eq!(
        world.path_entries_from(world.intermediate[MotifChainWorld::LEFT]),
        0
    );
    assert_eq!(world.body.reentry_state().closed_steps, closed_before);
    assert!(!trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
    ) || matches!(event, TraceEvent::Strengthened(_))));
}

#[test]
fn a_composed_motif_route_waits_for_the_real_intermediate_before_continuing() {
    let mut world = MotifChainWorld::new();
    world.train();
    assert_eq!(world.probe(90, 9).0, [MotifChainWorld::FRESH_LEFT]);

    schedule(
        &mut world.body,
        94,
        &[reading(world.intermediate[MotifChainWorld::LEFT], 0, 1, 9)],
    );
    let (events, trace) = run(&mut world.body);
    assert!(effect(&events, &world.motors).is_empty());
    assert!(trace.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned)
            if returned.source == world.intermediate[MotifChainWorld::LEFT]
                && returned.decision == ReturnDecision::Accepted
    )));

    let at = world.body.now() + 1;
    schedule(
        &mut world.body,
        at,
        &[
            Arrival::caused(world.motors[MotifChainWorld::LEFT_PROXY].opportunity, 1, 9),
            Arrival::caused(world.motors[MotifChainWorld::LEFT_CLOSER].opportunity, 1, 9),
        ],
    );
    let (events, _) = run(&mut world.body);
    assert_eq!(
        effect(&events, &world.motors),
        [MotifChainWorld::LEFT_CLOSER]
    );
}

#[test]
fn two_reaching_motif_routes_make_no_unique_composed_choice() {
    let mut world = MotifChainWorld::new();
    world.train();
    world.make_right_reach_goal();

    let (_, trace) = world.probe(90, 9);

    assert_eq!(
        trace
            .iter()
            .filter(|event| matches!(
                event,
                TraceEvent::Candidate(candidate) if candidate.motif_routes.len() == 1
            ))
            .count(),
        2
    );
    assert_ne!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
}

#[test]
fn one_downstream_example_cannot_supply_a_composed_motif_route() {
    let mut world = MotifChainWorld::new();
    world.train_first_motif();
    world.train_one_second_example();

    let (_, trace) = world.probe(90, 9);

    assert!(trace.iter().all(|event| !matches!(
        event,
        TraceEvent::Candidate(candidate) if !candidate.motif_routes.is_empty()
    )));
    assert_ne!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
}

#[test]
fn ambiguous_downstream_outcome_makes_no_composed_motif_route() {
    let mut world = MotifChainWorld::new();
    world.train();
    world.make_left_outcome_ambiguous();

    let (_, trace) = world.probe(90, 9);

    assert!(trace.iter().all(|event| !matches!(
        event,
        TraceEvent::Candidate(candidate) if !candidate.motif_routes.is_empty()
    )));
    assert_ne!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
}

#[test]
fn composed_motif_route_is_invariant_to_identity_and_construction_order() {
    for (dormant_prefix, reverse_fresh_construction) in
        [(false, false), (true, true), (false, true)]
    {
        let mut world = MotifChainWorld::with_setup(dormant_prefix, reverse_fresh_construction);
        world.train();

        let (events, trace) = world.probe(90, 9);

        assert_eq!(events, [MotifChainWorld::FRESH_LEFT]);
        assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
    }
}

#[test]
fn checkpoint_and_attachment_preserve_composed_motif_routes() {
    let mut plain = MotifChainWorld::new();
    plain.train();
    let bytes = plain.body.checkpoint().unwrap().canonical_bytes().unwrap();
    let mut restored = plain.clone();
    restored.body = BodyCheckpoint::decode(&bytes).unwrap().restore().unwrap();

    assert_eq!(plain.probe(90, 9), restored.probe(90, 9));

    let mut attached = MotifChainWorld::new();
    attached.train();
    let mut host = Body::default();
    let from = host.add_junction(Junction::integrating(1)).unwrap();
    let to = host.add_junction(Junction::integrating(1)).unwrap();
    host.add_link(Link::new(from, to, 1, 1)).unwrap();
    let junction_base = host.arena.junction_count();
    let part = OpenBody::new(attached.body, Vec::new()).unwrap();
    attach(&mut host, part, &[]).unwrap();
    attached.body = host;
    attached.remap_junctions(junction_base);

    let (events, trace) = attached.probe(90, 9);
    assert_eq!(events, [MotifChainWorld::FRESH_LEFT]);
    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
}

#[test]
fn an_independent_trained_product_does_not_change_the_composed_motif_choice() {
    let mut world = MotifChainWorld::new();
    world.train();
    let mut dormant = MotifChainWorld::new();
    dormant.train();
    let part = OpenBody::new(dormant.body, Vec::new()).unwrap();
    attach(&mut world.body, part, &[]).unwrap();

    let (events, trace) = world.probe(90, 9);

    assert_eq!(events, [MotifChainWorld::FRESH_LEFT]);
    assert_eq!(chosen_warrant(&trace), Some(ChoiceWarrant::Reentry));
}
