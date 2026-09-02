use truelearner_body::{attach, Arrival, AttachError, Body, Join, Junction, Link, OpenBody};

fn events(body: &mut Body) -> Vec<truelearner_body::PhysicalEvent> {
    let mut events = Vec::new();
    body.run(32, |event| events.push(event)).unwrap();
    events
}

#[test]
fn inward_attachment_preserves_memory_links_and_time() {
    let mut host = Body::default();
    let clock = host.add_junction(Junction::integrating(1)).unwrap();
    let received = host.add_junction(Junction::integrating(1)).unwrap();
    host.input(10, clock, 1).unwrap();
    events(&mut host);

    let mut sensor = Body::default();
    let sample = sensor.add_junction(Junction::sampled(20)).unwrap();
    let local = sensor.add_junction(Junction::integrating(1)).unwrap();
    let blocked = sensor.add_junction(Junction::integrating(1)).unwrap();
    sensor.add_link(Link::new(sample, local, 0, 1)).unwrap();
    let blocked_link = sensor.add_link(Link::new(sample, blocked, 0, 1)).unwrap();
    sensor.mark_path_entry(blocked_link).unwrap();
    sensor.input(1, sample, 7).unwrap();
    events(&mut sensor);
    let sensor = OpenBody::new(sensor, vec![sample, blocked]).unwrap();
    let port = sensor.port(0).unwrap();
    let blocked_port = sensor.port(1).unwrap();

    let attachment = attach(&mut host, sensor, &[Join::into_host(received, port, 0, 1)]).unwrap();
    let attached_sample = attachment.port(port).unwrap();
    let attached_blocked = attachment.port(blocked_port).unwrap();

    host.input(11, attached_sample, 7).unwrap();
    assert!(events(&mut host).is_empty());

    host.inputs(12, &[Arrival::new(attached_sample, 9)])
        .unwrap();
    let changed = events(&mut host);
    assert_eq!(changed.len(), 3);
    assert_eq!(changed[0].junction, attached_sample);
    assert_eq!(changed[1].at, 12);
    assert_eq!(changed[2].junction, received);
    assert_eq!(changed[2].at, 12);
    assert!(!changed
        .iter()
        .any(|event| event.junction == attached_blocked));
}

#[test]
fn outward_attachment_preserves_declared_direction() {
    let mut host = Body::default();
    let action = host.add_junction(Junction::integrating(1)).unwrap();
    let mut hand = Body::default();
    let movement = hand.add_junction(Junction::integrating(1)).unwrap();
    let hand = OpenBody::new(hand, vec![movement]).unwrap();
    let port = hand.port(0).unwrap();

    let attachment = attach(&mut host, hand, &[Join::into_part(action, port, 2, 1)]).unwrap();
    let movement = attachment.port(port).unwrap();
    host.inputs(3, &[Arrival::new(action, 1)]).unwrap();

    let fired = events(&mut host);
    assert_eq!(
        fired
            .iter()
            .map(|event| (event.at, event.junction))
            .collect::<Vec<_>>(),
        [(3, action), (5, movement)]
    );
}

#[test]
fn attachment_adds_no_undeclared_motor_mapping() {
    let mut host = Body::default();
    let tissue = host.add_junction(Junction::integrating(1)).unwrap();
    let motor = host.add_junction(Junction::integrating(1)).unwrap();
    let effect = host.add_junction(Junction::integrating(1)).unwrap();
    host.add_link(Link::new(motor, effect, 0, 1)).unwrap();

    let mut sensor = Body::default();
    let sample = sensor.add_junction(Junction::integrating(1)).unwrap();
    let sensor = OpenBody::new(sensor, vec![sample]).unwrap();
    let port = sensor.port(0).unwrap();
    let attachment = attach(&mut host, sensor, &[Join::into_host(tissue, port, 0, 1)]).unwrap();

    host.input(1, attachment.port(port).unwrap(), 1).unwrap();
    let fired = events(&mut host);
    assert!(fired.iter().any(|event| event.junction == tissue));
    assert!(!fired.iter().any(|event| event.junction == motor));
    assert!(!fired.iter().any(|event| event.junction == effect));
}

#[test]
fn attaching_an_empty_body_is_identity() {
    let mut attached = Body::default();
    let source = attached.add_junction(Junction::integrating(1)).unwrap();
    let target = attached.add_junction(Junction::integrating(1)).unwrap();
    attached.add_link(Link::new(source, target, 1, 1)).unwrap();
    let mut untouched = attached.clone();

    let empty = OpenBody::new(Body::default(), Vec::new()).unwrap();
    let result = attach(&mut attached, empty, &[]).unwrap();
    assert!(result.is_empty());

    attached.input(2, source, 1).unwrap();
    untouched.input(2, source, 1).unwrap();
    assert_eq!(events(&mut attached), events(&mut untouched));
}

#[test]
fn active_host_failure_is_atomic_and_returns_the_part() {
    let mut host = Body::default();
    let host_input = host.add_junction(Junction::integrating(1)).unwrap();
    host.input(1, host_input, 1).unwrap();
    let mut expected = host.clone();

    let mut part = Body::default();
    let part_port = part.add_junction(Junction::integrating(1)).unwrap();
    let part = OpenBody::new(part, vec![part_port]).unwrap();
    let port = part.port(0).unwrap();
    let failure = attach(&mut host, part, &[Join::into_host(host_input, port, 0, 1)]).unwrap_err();

    assert_eq!(failure.error(), AttachError::HostActive);
    assert!(failure.into_part().port(0).is_some());
    assert_eq!(events(&mut host), events(&mut expected));
}

#[test]
fn active_part_failure_is_atomic() {
    let mut host = Body::default();
    let host_port = host.add_junction(Junction::integrating(1)).unwrap();
    let mut part = Body::default();
    let part_port = part.add_junction(Junction::integrating(1)).unwrap();
    part.input(1, part_port, 1).unwrap();
    let part = OpenBody::new(part, vec![part_port]).unwrap();
    let port = part.port(0).unwrap();

    let failure = attach(&mut host, part, &[Join::into_host(host_port, port, 0, 1)]).unwrap_err();

    assert_eq!(failure.error(), AttachError::PartActive);
    assert!(host.is_quiet());
    assert!(!failure.into_part().into_body().is_quiet());
}

#[test]
fn invalid_ports_fail_before_host_mutation() {
    let mut donor = Body::default();
    donor.add_junction(Junction::integrating(1)).unwrap();
    let foreign_host = donor.add_junction(Junction::integrating(1)).unwrap();

    let mut host = Body::default();
    let host_port = host.add_junction(Junction::integrating(1)).unwrap();
    let mut expected = host.clone();

    let mut part = Body::default();
    let part_port = part.add_junction(Junction::integrating(1)).unwrap();
    let part = OpenBody::new(part, vec![part_port]).unwrap();
    let port = part.port(0).unwrap();
    let failure = attach(
        &mut host,
        part,
        &[Join::into_host(foreign_host, port, 0, 1)],
    )
    .unwrap_err();
    assert_eq!(failure.error(), AttachError::UnknownHost(foreign_host));
    failure.into_part();

    host.input(1, host_port, 1).unwrap();
    expected.input(1, host_port, 1).unwrap();
    assert_eq!(events(&mut host), events(&mut expected));

    let mut two_ports = Body::default();
    let first = two_ports.add_junction(Junction::integrating(1)).unwrap();
    let second = two_ports.add_junction(Junction::integrating(1)).unwrap();
    let two_ports = OpenBody::new(two_ports, vec![first, second]).unwrap();
    let foreign_part_port = two_ports.port(1).unwrap();
    let mut one_port = Body::default();
    let only = one_port.add_junction(Junction::integrating(1)).unwrap();
    let one_port = OpenBody::new(one_port, vec![only]).unwrap();
    let failure = attach(
        &mut host,
        one_port,
        &[Join::into_host(host_port, foreign_part_port, 0, 1)],
    )
    .unwrap_err();
    assert_eq!(
        failure.error(),
        AttachError::UnknownPartPort(foreign_part_port)
    );
}

#[test]
fn duplicate_open_ports_are_rejected() {
    let mut body = Body::default();
    let port = body.add_junction(Junction::integrating(1)).unwrap();

    assert!(matches!(
        OpenBody::new(body, vec![port, port]),
        Err(truelearner_body::OpenBodyError::DuplicatePort(id)) if id == port
    ));
}

#[test]
fn disconnected_attachment_order_preserves_behavior() {
    fn episode(reverse: bool) -> Vec<usize> {
        let mut host = Body::default();
        let targets = [
            host.add_junction(Junction::integrating(1)).unwrap(),
            host.add_junction(Junction::integrating(1)).unwrap(),
        ];
        let mut attached = [None, None];
        let order = if reverse { [1, 0] } else { [0, 1] };
        for index in order {
            let mut sensor = Body::default();
            let input = sensor.add_junction(Junction::integrating(1)).unwrap();
            let sensor = OpenBody::new(sensor, vec![input]).unwrap();
            let port = sensor.port(0).unwrap();
            let attachment = attach(
                &mut host,
                sensor,
                &[Join::into_host(targets[index], port, 0, 1)],
            )
            .unwrap();
            attached[index] = attachment.port(port);
        }
        for input in attached {
            host.inputs(1, &[Arrival::new(input.unwrap(), 1)]).unwrap();
        }
        let mut result = events(&mut host)
            .into_iter()
            .filter_map(|event| targets.iter().position(|target| *target == event.junction))
            .collect::<Vec<_>>();
        result.sort_unstable();
        result
    }

    assert_eq!(episode(false), [0, 1]);
    assert_eq!(episode(true), [0, 1]);
}

#[test]
fn grouping_disconnected_attachments_preserves_behavior() {
    fn sensor() -> (
        OpenBody,
        truelearner_body::Port,
        truelearner_body::JunctionId,
    ) {
        let mut body = Body::default();
        let input = body.add_junction(Junction::integrating(1)).unwrap();
        let body = OpenBody::new(body, vec![input]).unwrap();
        let port = body.port(0).unwrap();
        (body, port, input)
    }

    fn direct() -> Vec<usize> {
        let mut host = Body::default();
        let targets = [
            host.add_junction(Junction::integrating(1)).unwrap(),
            host.add_junction(Junction::integrating(1)).unwrap(),
        ];
        let mut inputs = Vec::new();
        for target in targets {
            let (sensor, port, _) = sensor();
            let attachment =
                attach(&mut host, sensor, &[Join::into_host(target, port, 0, 1)]).unwrap();
            inputs.push(Arrival::new(attachment.port(port).unwrap(), 1));
        }
        host.inputs(1, &inputs).unwrap();
        let mut result = events(&mut host)
            .into_iter()
            .filter_map(|event| targets.iter().position(|target| *target == event.junction))
            .collect::<Vec<_>>();
        result.sort_unstable();
        result
    }

    fn grouped() -> Vec<usize> {
        let (left, _, left_junction) = sensor();
        let left_body = left.into_body();
        let (right, right_port, _) = sensor();
        let mut group = left_body;
        let right_attachment = attach(&mut group, right, &[]).unwrap();
        let right_junction = right_attachment.port(right_port).unwrap();
        let group = OpenBody::new(group, vec![left_junction, right_junction]).unwrap();
        let group_left = group.port(0).unwrap();
        let group_right = group.port(1).unwrap();

        let mut host = Body::default();
        let targets = [
            host.add_junction(Junction::integrating(1)).unwrap(),
            host.add_junction(Junction::integrating(1)).unwrap(),
        ];
        let attachment = attach(
            &mut host,
            group,
            &[
                Join::into_host(targets[0], group_left, 0, 1),
                Join::into_host(targets[1], group_right, 0, 1),
            ],
        )
        .unwrap();
        host.inputs(
            1,
            &[
                Arrival::new(attachment.port(group_left).unwrap(), 1),
                Arrival::new(attachment.port(group_right).unwrap(), 1),
            ],
        )
        .unwrap();
        let mut result = events(&mut host)
            .into_iter()
            .filter_map(|event| targets.iter().position(|target| *target == event.junction))
            .collect::<Vec<_>>();
        result.sort_unstable();
        result
    }

    assert_eq!(direct(), grouped());
}
