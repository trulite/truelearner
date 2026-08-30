use truelearner_body::{Body, Junction, Link, Trigger};

#[test]
fn integrating_memory_holds_subthreshold_input() {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(3)).unwrap();
    body.input(0, junction, 1).unwrap();
    body.input(1, junction, 2).unwrap();

    let mut changes = Vec::new();
    body.run(10, |change| changes.push(change)).unwrap();

    assert_eq!(changes.len(), 1);
    assert_eq!((changes[0].before, changes[0].after), (1, 3));
    assert_eq!(body.held(junction), Some(0));
}

#[test]
fn sampled_memory_ignores_a_repeat_and_fires_on_real_change() {
    let mut body = Body::default();
    let sensor = body.add_junction(Junction::sampled(10)).unwrap();
    body.input(0, sensor, 7).unwrap();
    body.input(1, sensor, 7).unwrap();
    body.input(2, sensor, 9).unwrap();

    let mut changes = Vec::new();
    body.run(10, |change| changes.push(change)).unwrap();

    assert_eq!(changes.len(), 1);
    assert_eq!((changes[0].before, changes[0].after), (7, 9));
}

#[test]
fn sampled_memory_preserves_a_short_change_across_u32_wrap() {
    let mut body = Body::default();
    let sensor = body.add_junction(Junction::sampled(10)).unwrap();
    body.input(u32::MAX as u64 - 2, sensor, 7).unwrap();
    body.input(u32::MAX as u64 + 2, sensor, 9).unwrap();

    let mut changes = Vec::new();
    body.run(2, |change| changes.push(change)).unwrap();

    assert_eq!(changes.len(), 1);
    assert_eq!((changes[0].before, changes[0].after), (7, 9));
}

#[test]
fn sampled_memory_forgets_across_a_whole_u32_epoch() {
    let mut body = Body::default();
    let sensor = body.add_junction(Junction::sampled(10)).unwrap();
    body.input(1, sensor, 7).unwrap();
    body.input(1 + (1_u64 << 32) + 2, sensor, 9).unwrap();

    let mut changes = Vec::new();
    body.run(2, |change| changes.push(change)).unwrap();

    assert!(changes.is_empty());
}

#[test]
fn integrating_memory_supports_the_declared_i32_signal_range() {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(i32::MAX)).unwrap();
    body.input(0, junction, i32::MAX).unwrap();

    let mut changes = Vec::new();
    body.run(1, |change| changes.push(change)).unwrap();

    assert_eq!(changes[0].after, i32::MAX);
}

#[test]
fn directional_links_turn_sensor_change_into_distinct_actions() {
    let mut body = Body::default();
    let sensor = body.add_junction(Junction::sampled(10)).unwrap();
    let up = body.add_junction(Junction::integrating(1)).unwrap();
    let down = body.add_junction(Junction::integrating(1)).unwrap();
    body.add_link(Link::new(sensor, up, 0, 1).when(Trigger::RisesThrough(5)))
        .unwrap();
    body.add_link(Link::new(sensor, down, 0, 1).when(Trigger::FallsThrough(5)))
        .unwrap();

    for (at, sample) in [(0, 4), (1, 6), (2, 3)] {
        body.input(at, sensor, sample).unwrap();
    }
    let mut fired = Vec::new();
    body.run(10, |change| fired.push(change.junction)).unwrap();

    assert_eq!(fired, vec![sensor, up, sensor, down]);
}
