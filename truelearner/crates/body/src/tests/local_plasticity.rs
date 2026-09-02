use crate::{
    attach,
    core::LOCAL_PLASTICITY_WINDOW,
    harness::{attach_outcome_component, integrating, motor, Motor},
    ApplyError, Arrival, Body, JunctionId, Link, LinkId, OpenBody, ReturnDecision, TraceEvent,
};

struct LocalMeeting {
    body: Body,
    inputs: [JunctionId; 3],
    active: [LinkId; 2],
    inactive: LinkId,
    remote: LinkId,
    motor: Motor,
    outcome: JunctionId,
}

impl LocalMeeting {
    fn new(reverse_links: bool) -> Self {
        let mut body = Body::default();
        let motor = motor(&mut body);
        let path_surface = integrating(&mut body, 1);
        let path_entry = body
            .add_link(Link::new(path_surface, motor.opportunity, 1, 1))
            .unwrap();
        body.mark_path_entry(path_entry).unwrap();

        let inputs = std::array::from_fn(|_| integrating(&mut body, 1));
        let mut links = Vec::new();
        let order: &[usize] = if reverse_links {
            &[2, 1, 0]
        } else {
            &[0, 1, 2]
        };
        for index in order {
            let link = body
                .add_link(Link::new(inputs[*index], motor.opportunity, 1, 1))
                .unwrap();
            body.mark_locally_plastic(link).unwrap();
            links.push((*index, link));
        }
        links.sort_unstable_by_key(|(index, _)| *index);
        let active = [links[0].1, links[1].1];
        let inactive = links[2].1;

        let remote_source = integrating(&mut body, 1);
        let remote_target = integrating(&mut body, 1);
        let remote = body
            .add_link(Link::new(remote_source, remote_target, 1, 1))
            .unwrap();
        body.mark_locally_plastic(remote).unwrap();
        let outcome = integrating(&mut body, 1);
        attach_outcome_component(&mut body, outcome, [motor.opportunity]);

        Self {
            body,
            inputs,
            active,
            inactive,
            remote,
            motor,
            outcome,
        }
    }

    fn act(&mut self, at: u64, causes: [u64; 2], include_remote: bool) -> Vec<TraceEvent> {
        let mut arrivals = vec![
            Arrival::caused(self.inputs[0], 1, causes[0]),
            Arrival::caused(self.inputs[1], 1, causes[1]),
        ];
        if include_remote {
            let source = self.body.arena.link(self.remote).unwrap().from;
            arrivals.push(Arrival::caused(source, 1, 77));
        }
        self.body.inputs(at, &arrivals).unwrap();
        self.run()
    }

    fn return_outcome(&mut self, at: u64, cause: u64) -> Vec<TraceEvent> {
        self.body
            .inputs(at, &[Arrival::caused(self.outcome, 1, cause)])
            .unwrap();
        self.run()
    }

    fn require_general_reaction_path(&mut self) {
        let quiet = integrating(&mut self.body, 1);
        let entry = self
            .body
            .add_link(Link::new(self.outcome, quiet, 1, 1))
            .unwrap();
        self.body.mark_path_entry(entry).unwrap();
    }

    fn run(&mut self) -> Vec<TraceEvent> {
        let mut trace = Vec::new();
        self.body
            .run_traced(256, |_| {}, |event| trace.push(event))
            .unwrap();
        assert!(self.body.is_quiet());
        trace
    }

    fn strength(&self, link: LinkId) -> i64 {
        self.body.arrows[link.slot()].strength()
    }
}

fn accepted(trace: &[TraceEvent]) -> bool {
    trace.iter().any(|event| {
        matches!(
            event,
            TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted
        )
    })
}

#[test]
fn a_local_return_strengthens_every_recent_input_to_the_returned_meeting() {
    let mut world = LocalMeeting::new(false);
    let action = world.act(10, [41, 42], true);
    assert!(action.iter().any(|event| matches!(
        event,
        TraceEvent::Transition(change) if change.junction == world.motor.effect
    )));

    let returned = world.return_outcome(14, 99);
    assert!(accepted(&returned));
    assert_eq!(world.strength(world.active[0]), 2);
    assert_eq!(world.strength(world.active[1]), 2);
    assert_eq!(world.strength(world.inactive), 1);
    assert_eq!(world.strength(world.remote), 1);
}

#[test]
fn a_late_return_does_not_strengthen_expired_local_activity() {
    let mut world = LocalMeeting::new(false);
    world.act(10, [41, 42], false);
    let returned = world.return_outcome(10 + LOCAL_PLASTICITY_WINDOW + 1, 99);

    assert!(accepted(&returned));
    assert_eq!(world.strength(world.active[0]), 1);
    assert_eq!(world.strength(world.active[1]), 1);
}

#[test]
fn the_window_boundary_is_eligible() {
    let mut world = LocalMeeting::new(false);
    world.act(10, [41, 42], false);
    let returned = world.return_outcome(10 + LOCAL_PLASTICITY_WINDOW, 99);

    assert!(accepted(&returned));
    assert_eq!(world.strength(world.active[0]), 2);
    assert_eq!(world.strength(world.active[1]), 2);
}

#[test]
fn activity_without_a_return_does_not_strengthen() {
    let mut world = LocalMeeting::new(false);
    world.act(10, [41, 42], false);

    assert_eq!(world.strength(world.active[0]), 1);
    assert_eq!(world.strength(world.active[1]), 1);
}

#[test]
fn general_reaction_and_single_return_paths_obey_the_same_law() {
    let episode = |general| {
        let mut world = LocalMeeting::new(false);
        if general {
            world.require_general_reaction_path();
        }
        world.act(10, [41, 42], false);
        let returned = world.return_outcome(14, 99);
        assert!(accepted(&returned));
        [
            world.strength(world.active[0]),
            world.strength(world.active[1]),
        ]
    };

    assert_eq!(episode(false), [2, 2]);
    assert_eq!(episode(true), [2, 2]);
}

#[test]
fn a_recent_active_predecessor_in_the_local_backward_cone_strengthens() {
    let mut world = LocalMeeting::new(false);
    let predecessor_source = integrating(&mut world.body, 1);
    let predecessor = world
        .body
        .add_link(Link::new(predecessor_source, world.inputs[0], 0, 1))
        .unwrap();
    world.body.mark_locally_plastic(predecessor).unwrap();
    world
        .body
        .inputs(
            10,
            &[
                Arrival::caused(predecessor_source, 1, 41),
                Arrival::caused(world.inputs[1], 1, 42),
            ],
        )
        .unwrap();
    world.run();

    assert!(accepted(&world.return_outcome(14, 99)));
    assert_eq!(world.strength(predecessor), 2);
    assert_eq!(world.strength(world.active[0]), 2);
    assert_eq!(world.strength(world.active[1]), 2);
}

#[test]
fn a_fixed_link_carries_eligibility_without_changing() {
    let mut world = LocalMeeting::new(false);
    let fixed_source = integrating(&mut world.body, 1);
    let fixed = world
        .body
        .add_link(Link::new(fixed_source, world.inputs[0], 0, 1))
        .unwrap();
    world
        .body
        .inputs(
            10,
            &[
                Arrival::caused(fixed_source, 1, 41),
                Arrival::caused(world.inputs[1], 1, 42),
            ],
        )
        .unwrap();
    world.run();

    assert!(accepted(&world.return_outcome(14, 99)));
    assert_eq!(world.strength(fixed), 1);
    assert_eq!(world.strength(world.active[0]), 2);
}

#[test]
fn local_plasticity_saturates_after_one_change() {
    let mut world = LocalMeeting::new(false);
    world.act(10, [41, 42], false);
    assert!(accepted(&world.return_outcome(14, 99)));
    world.act(20, [51, 52], false);
    assert!(accepted(&world.return_outcome(24, 100)));

    assert_eq!(world.strength(world.active[0]), 2);
    assert_eq!(world.strength(world.active[1]), 2);
}

#[test]
fn an_ambiguous_return_strengthens_no_recent_input() {
    let mut world = LocalMeeting::new(false);
    world.act(10, [41, 42], false);
    world.act(20, [51, 52], false);
    let returned = world.return_outcome(24, 99);

    assert!(returned.iter().any(|event| matches!(
        event,
        TraceEvent::Return(returned) if returned.decision == ReturnDecision::Ambiguous
    )));
    assert_eq!(world.strength(world.active[0]), 1);
    assert_eq!(world.strength(world.active[1]), 1);
}

#[test]
fn checkpoint_restart_preserves_pending_local_eligibility() {
    let mut world = LocalMeeting::new(false);
    world.act(10, [41, 42], false);
    let checkpoint = world.body.checkpoint().unwrap();
    world.body = checkpoint.restore().unwrap();

    assert!(accepted(&world.return_outcome(14, 99)));
    assert_eq!(world.strength(world.active[0]), 2);
    assert_eq!(world.strength(world.active[1]), 2);
}

#[test]
fn attachment_preserves_the_local_plasticity_mark() {
    let mut host = Body::default();
    let host_source = integrating(&mut host, 1);
    let host_target = integrating(&mut host, 1);
    host.add_link(Link::new(host_source, host_target, 1, 1))
        .unwrap();
    let link_base = host.arena.link_count();

    let mut part = Body::default();
    let source = integrating(&mut part, 1);
    let target = integrating(&mut part, 1);
    let plastic = part.add_link(Link::new(source, target, 1, 1)).unwrap();
    part.mark_locally_plastic(plastic).unwrap();
    let part = OpenBody::new(part, vec![source]).unwrap();

    attach(&mut host, part, &[]).unwrap();
    let attached = LinkId::new(link_base + plastic.slot()).unwrap();
    assert!(host.arrows[attached.slot()].locally_plastic());
}

#[test]
fn only_drive_links_can_be_marked_locally_plastic() {
    let mut body = Body::default();
    let source = integrating(&mut body, 1);
    let target = integrating(&mut body, 1);
    let entry = body.add_link(Link::new(source, target, 1, 1)).unwrap();
    body.mark_path_entry(entry).unwrap();

    assert_eq!(
        body.mark_locally_plastic(entry),
        Err(ApplyError::InvalidLinkRole(entry))
    );
}

#[test]
fn link_construction_order_does_not_change_local_plasticity() {
    let episode = |reverse_links| {
        let mut world = LocalMeeting::new(reverse_links);
        world.act(10, [41, 42], true);
        world.return_outcome(14, 99);
        [
            world.strength(world.active[0]),
            world.strength(world.active[1]),
            world.strength(world.inactive),
            world.strength(world.remote),
        ]
    };

    assert_eq!(episode(false), episode(true));
}
