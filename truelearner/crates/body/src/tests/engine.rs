use super::*;

fn participant_links(body: &Body, change: usize) -> Vec<Option<LinkId>> {
    let change = &body.activity.moment.changes[change];
    body.activity
        .moment
        .participants(change)
        .map(|participant| participant.via)
        .collect()
}

#[test]
fn frontier_preserves_boundary_and_link_participants() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(2)).unwrap();
    let middle = body.add_junction(Junction::integrating(1)).unwrap();
    let target = body.add_junction(Junction::integrating(1)).unwrap();
    let first = body.add_link(Link::new(source, middle, 1, 1)).unwrap();
    let second = body.add_link(Link::new(middle, target, 1, 1)).unwrap();
    body.inputs(
        3,
        &[Arrival::caused(source, 1, 7), Arrival::caused(source, 1, 7)],
    )
    .unwrap();

    body.step(|_| {}).unwrap();
    assert_eq!(body.activity.moment.changes[0].event.cause, 7);
    assert_eq!(participant_links(&body, 0), [None, None]);
    assert!(body.link_memory[first.slot()].transmitted);
    assert_eq!(body.link_memory[first.slot()].cause, 7);
    assert_eq!(body.link_memory[first.slot()].participated_at, 3);

    body.step(|_| {}).unwrap();
    assert_eq!(body.activity.moment.changes[0].event.junction, middle);
    assert_eq!(participant_links(&body, 0), [Some(first)]);
    assert!(body.link_memory[second.slot()].transmitted);
    assert_eq!(body.link_memory[second.slot()].cause, 7);
    assert_eq!(body.link_memory[second.slot()].participated_at, 4);

    body.step(|_| {}).unwrap();
    assert_eq!(body.activity.moment.changes[0].event.junction, target);
    assert_eq!(participant_links(&body, 0), [Some(second)]);
}

#[test]
fn meeting_cause_is_order_independent_and_accumulated_once() {
    fn episode(causes: [u64; 3]) -> u64 {
        let mut body = Body::default();
        let junction = body.add_junction(Junction::integrating(3)).unwrap();
        body.inputs(0, &causes.map(|cause| Arrival::caused(junction, 1, cause)))
            .unwrap();
        body.step(|_| {}).unwrap();
        body.activity.moment.changes[0].event.cause
    }

    assert_eq!(episode([8, 8, 8]), 8);
    assert_eq!(episode([8, 9, 8]), 0);
    assert_eq!(episode([9, 8, 8]), 0);
}

#[test]
fn boundary_input_and_failed_enqueue_record_no_link_transmission() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(1)).unwrap();
    let target = body.add_junction(Junction::integrating(1)).unwrap();
    let clock = body.add_junction(Junction::integrating(1)).unwrap();
    let link = body.add_link(Link::new(source, target, 0, 1)).unwrap();
    body.set_link_role(link, crate::core::LinkRole::PathEntry)
        .unwrap();

    body.input(10, clock, 1).unwrap();
    body.step(|_| {}).unwrap();
    assert!(!body.link_memory[link.slot()].transmitted);

    assert_eq!(
        body.send_through(9, link, 3),
        Err(RunError::TimeWentBackward {
            now: 10,
            requested: 9,
        })
    );
    assert!(!body.link_memory[link.slot()].transmitted);
}

#[test]
fn irrelevant_waves_do_not_touch_reaction_workspace() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(1)).unwrap();
    let target = body.add_junction(Junction::integrating(1)).unwrap();
    body.add_link(Link::new(source, target, 0, 1)).unwrap();

    body.input(1, source, 1).unwrap();
    body.run(4, |_| {}).unwrap();
    assert!(body.activity.reaction.is_clear());
    assert_eq!(body.activity.reaction.fact_capacity(), 0);
}

#[test]
fn relevant_wave_clears_and_reuses_reaction_workspace() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(1)).unwrap();
    let output = body.add_junction(Junction::integrating(1)).unwrap();
    body.add_link(Link::new(source, output, 1, 0)).unwrap();

    body.input(1, source, 1).unwrap();
    body.step(|_| {}).unwrap();
    assert!(body.activity.reaction.is_clear());
    assert!(body.activity.reaction.fact_capacity() > 0);
}
