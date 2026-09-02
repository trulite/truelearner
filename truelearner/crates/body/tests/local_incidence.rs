use truelearner_body::{Arrival, Body, Junction};

#[test]
fn locally_near_arrivals_integrate_without_episode_identity() {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(10)).unwrap();

    body.input(10, junction, 5).unwrap();
    body.run(8, |_| {}).unwrap();
    body.input(14, junction, 5).unwrap();
    let mut fired = Vec::new();
    body.run(8, |event| fired.push(event)).unwrap();

    assert_eq!(fired.len(), 1);
    assert_eq!(fired[0].junction, junction);
}

#[test]
fn arrivals_outside_the_physical_window_do_not_share_potential() {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(10)).unwrap();

    body.input(10, junction, 5).unwrap();
    body.run(8, |_| {}).unwrap();
    body.input(15, junction, 7).unwrap();
    let mut fired = Vec::new();
    body.run(8, |event| fired.push(event)).unwrap();

    assert!(fired.is_empty());
    assert_eq!(body.held(junction), Some(7));
}

#[test]
fn one_moment_combines_all_arrivals_at_one_physical_junction() {
    let mut body = Body::default();
    let left = body.add_junction(Junction::integrating(2)).unwrap();
    let right = body.add_junction(Junction::integrating(2)).unwrap();

    body.inputs(
        10,
        &[
            Arrival::new(left, 1),
            Arrival::new(right, 1),
            Arrival::new(left, 1),
        ],
    )
    .unwrap();
    let mut fired = Vec::new();
    body.run(8, |event| fired.push(event)).unwrap();

    assert_eq!(fired.len(), 1);
    assert_eq!(fired[0].junction, left);
    assert_eq!(fired[0].arrivals, 2);
    assert_eq!(body.held(right), Some(1));
}
