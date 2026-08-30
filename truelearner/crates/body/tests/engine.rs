use truelearner_body::{Arrival, Body, Junction, Link, RunError};

#[test]
fn quiet_step_is_identity_and_reports_no_frontier() {
    let mut body = Body::default();

    assert_eq!(body.step(|_| {}).unwrap(), None);
    assert!(body.is_quiet());
}

#[test]
fn batch_meeting_is_independent_of_arrival_order() {
    fn episode(impulses: [i32; 3]) -> Vec<(i64, i32, i32)> {
        let mut body = Body::default();
        let junction = body.add_junction(Junction::integrating(i32::MAX)).unwrap();
        let arrivals = impulses.map(|impulse| Arrival::new(junction, impulse));
        body.inputs(0, &arrivals).unwrap();
        let mut changes = Vec::new();
        body.step(|change| changes.push((change.impulse, change.before, change.after)))
            .unwrap();
        changes
    }

    let expected = vec![(i64::from(i32::MAX), 0, i32::MAX)];
    assert_eq!(episode([i32::MAX, 1, -1]), expected);
    assert_eq!(episode([-1, 1, i32::MAX]), expected);
}

#[test]
fn run_is_exactly_repeated_step() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(3)).unwrap();
    let target = body.add_junction(Junction::integrating(1)).unwrap();
    body.add_link(Link::new(source, target, 2, 1)).unwrap();
    body.inputs(4, &[Arrival::new(source, 1), Arrival::new(source, 2)])
        .unwrap();
    let mut stepped = body.clone();

    let mut run_changes = Vec::new();
    let run = body.run(2, |change| run_changes.push(change)).unwrap();
    let mut step_changes = Vec::new();
    let mut steps = Vec::new();
    while let Some(step) = stepped.step(|change| step_changes.push(change)).unwrap() {
        steps.push(step);
    }

    assert_eq!(run_changes, step_changes);
    assert_eq!(steps.len(), 2);
    assert_eq!(run.moments, steps.len() as u64);
    assert_eq!(
        run.work.arrivals,
        steps.iter().map(|step| step.work.arrivals).sum()
    );
    assert_eq!(
        run.work.emissions,
        steps.iter().map(|step| step.work.emissions).sum()
    );
    assert!(body.is_quiet() && stepped.is_quiet());
}

#[test]
fn step_reports_its_actual_physical_work() {
    let mut body = Body::default();
    let integrating = body.add_junction(Junction::integrating(1)).unwrap();
    let sampled = body.add_junction(Junction::sampled(8)).unwrap();
    body.inputs(0, &[Arrival::new(integrating, 1), Arrival::new(sampled, 4)])
        .unwrap();

    let step = body.step(|_| {}).unwrap().unwrap();

    assert_eq!(step.work.arrivals, 2);
    assert_eq!(step.work.meetings, 2);
    assert_eq!(step.work.changes, 1);
}

#[test]
fn arrivals_meet_before_a_junction_fires() {
    let mut body = Body::default();
    let input = body.add_junction(Junction::integrating(3)).unwrap();
    let output = body.add_junction(Junction::integrating(1)).unwrap();
    body.add_link(Link::new(input, output, 2, 1)).unwrap();
    body.input(4, input, 1).unwrap();
    body.input(4, input, 2).unwrap();

    let mut changes = Vec::new();
    let run = body.run(10, |change| changes.push(change)).unwrap();

    assert_eq!(
        (run.moments, run.work.arrivals, run.work.meetings),
        (2, 3, 2)
    );
    assert_eq!(changes.len(), 2);
    assert_eq!(
        (changes[0].at, changes[0].arrivals, changes[0].impulse),
        (4, 2, 3)
    );
    assert_eq!(changes[1].at, 6);
    assert!(body.is_quiet());
}

#[test]
fn dormant_body_size_does_not_change_active_work() {
    let mut body = Body::default();
    let active = body.add_junction(Junction::integrating(1)).unwrap();
    for _ in 0..10_000 {
        body.add_junction(Junction::integrating(1)).unwrap();
    }
    body.input(0, active, 1).unwrap();

    let run = body.run(2, |_| {}).unwrap();

    assert_eq!(
        (
            run.work.arrivals,
            run.work.meetings,
            run.work.changes,
            run.work.link_visits
        ),
        (1, 1, 1, 0)
    );
}

#[test]
fn arena_growth_does_not_move_an_existing_identity() {
    let mut body = Body::default();
    let original = body.add_junction(Junction::integrating(1)).unwrap();
    body.input(0, original, 1).unwrap();
    body.run(1, |_| {}).unwrap();

    for _ in 0..100_000 {
        body.add_junction(Junction::integrating(1)).unwrap();
    }
    body.input(1, original, 1).unwrap();
    let mut changes = Vec::new();
    body.run(1, |change| changes.push(change)).unwrap();

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].junction, original);
}

#[test]
fn reservation_and_slot_renaming_preserve_behavior() {
    fn episode(dormant_first: bool, reserve: bool) -> Vec<(u64, i32, i32)> {
        let mut body = Body::default();
        if reserve {
            body.reserve(128, 8);
        }
        if dormant_first {
            body.add_junction(Junction::integrating(1)).unwrap();
        }
        let source = body.add_junction(Junction::integrating(1)).unwrap();
        let target = body.add_junction(Junction::integrating(1)).unwrap();
        body.add_link(Link::new(source, target, 3, 1)).unwrap();
        body.input(7, source, 1).unwrap();

        let mut observed = Vec::new();
        body.run(2, |change| {
            observed.push((change.at, change.before, change.after))
        })
        .unwrap();
        observed
    }

    assert_eq!(episode(false, false), episode(false, true));
    assert_eq!(episode(false, false), episode(true, false));
}

#[test]
fn sparse_large_times_do_not_scan_empty_ticks() {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(1)).unwrap();
    body.input(1_000_000_000, junction, 1).unwrap();

    let run = body.run(1, |_| {}).unwrap();

    assert_eq!((run.moments, run.work.arrivals), (1, 1));
}

#[test]
fn link_delay_is_not_currently_bounded_to_u16() {
    let mut body = Body::default();
    let source = body.add_junction(Junction::integrating(1)).unwrap();
    let target = body.add_junction(Junction::integrating(1)).unwrap();
    let delay = u16::MAX as u64 + 1;
    body.add_link(Link::new(source, target, delay, 1)).unwrap();
    body.input(3, source, 1).unwrap();

    let mut times = Vec::new();
    body.run(2, |change| times.push(change.at)).unwrap();

    assert_eq!(times, [3, 3 + delay]);
}

#[test]
fn schedule_preserves_time_order_across_sparse_buckets() {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(1)).unwrap();
    for at in [65_537, 2, u32::MAX as u64, 65_536, 3] {
        body.input(at, junction, 1).unwrap();
    }

    let mut times = Vec::new();
    body.run(5, |change| times.push(change.at)).unwrap();

    assert_eq!(times, [2, 3, 65_536, 65_537, u32::MAX as u64]);
}

#[test]
fn time_cannot_move_backward_between_runs() {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(1)).unwrap();
    body.input(10, junction, 1).unwrap();
    body.run(1, |_| {}).unwrap();

    assert_eq!(
        body.input(9, junction, 1),
        Err(RunError::TimeWentBackward {
            now: 10,
            requested: 9
        })
    );
}

#[test]
fn a_loop_stops_at_the_explicit_physical_limit() {
    let mut body = Body::default();
    let junction = body.add_junction(Junction::integrating(1)).unwrap();
    body.add_link(Link::new(junction, junction, 0, 1)).unwrap();
    body.input(0, junction, 1).unwrap();

    assert_eq!(body.run(4, |_| {}), Err(RunError::MomentLimitReached));
    assert!(!body.is_quiet());
}
