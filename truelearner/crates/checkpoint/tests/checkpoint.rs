use truelearner_body::{Body, Junction, Link, LinkRole, RunError};
use truelearner_checkpoint::capture;

#[test]
fn capturing_an_already_quiet_body_is_identity() {
    let mut body = Body::default();

    let captured = capture(&mut body, 0, |_| {}).unwrap();
    let restored = captured.checkpoint.restore();

    assert_eq!(captured.drain.moments, 0);
    assert!(body.is_quiet());
    assert!(restored.is_quiet());
}

#[test]
fn capture_forwards_every_event_while_draining_to_quiet() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(1)).unwrap();
    let target = body.add_junction(Junction::integrating(1)).unwrap();
    body.add_link(Link::new(source, target, 2, 1)).unwrap();
    body.input(4, source, 1).unwrap();
    let mut events = Vec::new();

    let captured = capture(&mut body, 4, |event| events.push(event)).unwrap();

    assert_eq!(events.len(), 2);
    assert_eq!((events[0].at, events[0].junction), (4, source));
    assert_eq!((events[1].at, events[1].junction), (6, target));
    assert_eq!(captured.drain.moments, 2);
    assert!(body.is_quiet());
    assert!(captured.checkpoint.restore().is_quiet());
}

#[test]
fn restored_continuation_equals_uninterrupted_continuation() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(1)).unwrap();
    let target = body.add_junction(Junction::integrating(1)).unwrap();
    body.add_link(Link::new(source, target, 1, 1)).unwrap();
    body.input(2, source, 1).unwrap();
    let checkpoint = capture(&mut body, 4, |_| {}).unwrap().checkpoint;
    let mut restored = checkpoint.restore();

    body.input(8, source, 1).unwrap();
    restored.input(8, source, 1).unwrap();
    let mut uninterrupted_events = Vec::new();
    let uninterrupted = body
        .run(4, |event| uninterrupted_events.push(event))
        .unwrap();
    let mut restored_events = Vec::new();
    let replayed = restored
        .run(4, |event| restored_events.push(event))
        .unwrap();

    assert_eq!(replayed, uninterrupted);
    assert_eq!(restored_events, uninterrupted_events);
    assert_eq!(restored.held(source), body.held(source));
    assert_eq!(restored.held(target), body.held(target));
}

#[test]
fn sensor_memory_survives_the_quiet_cut() {
    let mut body = Body::default();
    let sensor = body.add_junction(Junction::sampled(10)).unwrap();
    body.input(1, sensor, 7).unwrap();
    let checkpoint = capture(&mut body, 2, |_| {}).unwrap().checkpoint;
    let mut restored = checkpoint.restore();

    restored.input(2, sensor, 7).unwrap();
    let mut repeated = Vec::new();
    restored.run(2, |event| repeated.push(event)).unwrap();
    assert!(repeated.is_empty());

    restored.input(3, sensor, 9).unwrap();
    let mut changed = Vec::new();
    restored.run(2, |event| changed.push(event)).unwrap();
    assert_eq!(changed.len(), 1);
    assert_eq!((changed[0].before, changed[0].after), (7, 9));
}

#[test]
fn link_learning_role_survives_the_quiet_cut() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(1)).unwrap();
    let target = body.add_junction(Junction::integrating(1)).unwrap();
    let link = body.add_link(Link::new(source, target, 0, 1)).unwrap();
    body.set_link_role(link, LinkRole::PathEntry).unwrap();
    let checkpoint = capture(&mut body, 0, |_| {}).unwrap().checkpoint;
    let mut restored = checkpoint.restore();

    restored.input(1, source, 1).unwrap();
    let mut events = Vec::new();
    restored.run(2, |event| events.push(event)).unwrap();

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].junction, source);
}

#[test]
fn body_time_survives_the_quiet_cut() {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(1)).unwrap();
    body.input(10, junction, 1).unwrap();
    let checkpoint = capture(&mut body, 2, |_| {}).unwrap().checkpoint;
    let mut restored = checkpoint.restore();

    assert_eq!(
        restored.input(9, junction, 1),
        Err(RunError::TimeWentBackward {
            now: 10,
            requested: 9,
        })
    );
}

#[test]
fn repeated_restores_are_independent() {
    let mut body = Body::default();
    let sensor = body.add_junction(Junction::sampled(10)).unwrap();
    body.input(1, sensor, 7).unwrap();
    let checkpoint = capture(&mut body, 2, |_| {}).unwrap().checkpoint;
    let mut left = checkpoint.restore();
    let right = checkpoint.restore();

    left.input(2, sensor, 9).unwrap();
    left.run(2, |_| {}).unwrap();

    assert_eq!(left.held(sensor), Some(9));
    assert_eq!(right.held(sensor), Some(7));
    assert_eq!(checkpoint.restore().held(sensor), Some(7));
}

#[test]
fn failed_drain_returns_no_checkpoint_and_releases_body_access() {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(1)).unwrap();
    let loop_link = body.add_link(Link::new(junction, junction, 0, 1)).unwrap();
    body.input(0, junction, 1).unwrap();

    assert!(matches!(
        capture(&mut body, 4, |_| {}),
        Err(RunError::MomentLimitReached)
    ));
    assert!(!body.is_quiet());

    body.set_link_role(loop_link, LinkRole::PathEntry).unwrap();
    body.run(2, |_| {}).unwrap();
    assert!(body.is_quiet());
}

#[test]
fn consuming_a_checkpoint_yields_the_captured_body() {
    let mut body = Body::default();
    let sensor = body.add_junction(Junction::sampled(10)).unwrap();
    body.input(1, sensor, 4).unwrap();

    let restored = capture(&mut body, 2, |_| {})
        .unwrap()
        .checkpoint
        .into_body();

    assert_eq!(restored.held(sensor), Some(4));
    assert!(restored.is_quiet());
}
