use truelearner_core::{
    AttachError, AttachmentSite, ComponentJunction, ComponentLink, ComponentSpecError, Harness,
    HarnessBuilder, Input, Junction, Link, PhysicalComponentSpec, PhysicalEvent, PhysicalIncidence,
    PhysicalInput, Protocol, TransmissionMode,
};

fn junction(physical_id: u64, position: i32, region: i16, threshold: i32) -> Junction {
    Junction {
        physical_id,
        position,
        region,
        threshold,
        resistance: u32::MAX,
    }
}

fn component(
    junctions: Vec<ComponentJunction>,
    links: Vec<ComponentLink>,
) -> PhysicalComponentSpec {
    PhysicalComponentSpec::new(junctions, links, vec![0]).unwrap()
}

fn one_port_component() -> PhysicalComponentSpec {
    component(
        vec![
            ComponentJunction::ordinary(0, 1),
            ComponentJunction::ordinary(0, 2),
        ],
        vec![ComponentLink::ordinary(0, 1, 1)],
    )
}

fn port_input(attachment: &truelearner_core::PhysicalAttachment, tick: i64) -> PhysicalInput {
    let port = attachment.port(0).unwrap();
    PhysicalInput {
        input: Input {
            target: port.target(),
            arrival_tick: tick,
            phase: 0,
            impulse: 1,
            origin_physical: port.origin_physical(),
        },
        incidence: PhysicalIncidence::Sample,
    }
}

#[test]
fn runtime_attachment_is_atomic_quiet_and_checkpointed() {
    let mut builder = HarnessBuilder::with_capacity(8, 8, 1);
    builder.set_physical_tracing(true);
    let mut harness = builder.build();
    let before = harness.save().unwrap().canonical_bytes().unwrap();

    let attachment = harness
        .attach_physical(AttachmentSite::new(7, 0), &one_port_component())
        .unwrap();

    assert_eq!(harness.read().clock.tick, 0);
    assert_eq!(harness.read().junctions.len(), 2);
    assert_eq!(harness.read().links.len(), 1);
    assert_eq!(attachment.len(), 1);
    assert!(!attachment.is_empty());
    assert_ne!(harness.save().unwrap().canonical_bytes().unwrap(), before);

    let checkpoint = harness.save().unwrap();
    let mut restored = Harness::restore(checkpoint).unwrap();
    let input = port_input(&attachment, 1);
    let original_run = harness.send_physical(&[input]);
    let restored_run = restored.send_physical(&[input]);

    assert!(original_run.naturally_quiescent);
    assert_eq!(restored_run, original_run);
    assert_eq!(restored.read(), harness.read());
}

#[test]
fn runtime_attachment_capacity_failure_rolls_back_exactly() {
    let mut builder = HarnessBuilder::with_capacity(1, 0, 1);
    builder.add_junction(junction(1, 0, 0, 1));
    let mut harness = builder.build();
    let before = harness.save().unwrap().canonical_bytes().unwrap();

    assert_eq!(
        harness.attach_physical(AttachmentSite::new(2, 0), &one_port_component()),
        Err(AttachError::JunctionCapacity {
            needed: 2,
            available: 0,
        })
    );
    assert_eq!(harness.save().unwrap().canonical_bytes().unwrap(), before);
}

#[test]
fn runtime_attachment_refuses_pending_activity_without_mutation() {
    let mut builder = HarnessBuilder::with_capacity(4, 2, 1);
    let target = builder.add_junction(junction(1, 0, 0, 1));
    let mut harness = builder.build();
    let exhausted = harness.send_bounded(
        &[Input {
            target,
            arrival_tick: 1,
            phase: 0,
            impulse: 1,
            origin_physical: 1,
        }],
        0,
    );
    assert!(!exhausted.naturally_quiescent);
    let before = harness.save().unwrap().canonical_bytes().unwrap();

    assert_eq!(
        harness.attach_physical(AttachmentSite::new(2, 0), &one_port_component()),
        Err(AttachError::BodyNotQuiescent)
    );
    assert_eq!(harness.save().unwrap().canonical_bytes().unwrap(), before);
}

#[test]
fn runtime_attachment_spec_makes_invalid_local_topology_unrepresentable() {
    assert_eq!(
        PhysicalComponentSpec::new(Vec::new(), Vec::new(), Vec::new()),
        Err(ComponentSpecError::NoJunctions)
    );
    assert_eq!(
        PhysicalComponentSpec::new(
            vec![ComponentJunction::ordinary(0, 1)],
            Vec::new(),
            Vec::new(),
        ),
        Err(ComponentSpecError::NoPorts)
    );
    assert_eq!(
        PhysicalComponentSpec::new(vec![ComponentJunction::ordinary(0, 1)], Vec::new(), vec![1],),
        Err(ComponentSpecError::UnknownPort {
            port: 1,
            junctions: 1,
        })
    );
    assert_eq!(
        PhysicalComponentSpec::new(
            vec![ComponentJunction::ordinary(0, 1)],
            Vec::new(),
            vec![0, 0],
        ),
        Err(ComponentSpecError::DuplicatePort(0))
    );
    assert_eq!(
        PhysicalComponentSpec::new(vec![ComponentJunction::ordinary(0, 1)], Vec::new(), vec![0],),
        Err(ComponentSpecError::UnlinkedJunction(0))
    );
    assert_eq!(
        ComponentJunction::new(0, 0, 1),
        Err(ComponentSpecError::NonPositiveThreshold)
    );
    assert_eq!(
        ComponentJunction::new(0, 1, 0),
        Err(ComponentSpecError::NonPositiveResistance)
    );
    assert_eq!(
        ComponentLink::new(0, 1, -1, 0, 1, 1, TransmissionMode::Drive),
        Err(ComponentSpecError::NegativeDelay)
    );
}

#[test]
fn runtime_attachment_adds_no_motor_map_and_uses_ordinary_formation() {
    let mut builder = HarnessBuilder::with_capacity(16, 32, 1);
    let motor = builder.add_junction(junction(10, 1, 0, 2));
    let outside = builder.add_junction(junction(11, 1, 1, 1));
    builder.add_link(Link {
        from: motor,
        to: outside,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: u32::MAX,
        mode: TransmissionMode::Drive,
    });
    let mut harness = builder.build();

    let attachment = harness
        .attach_physical(AttachmentSite::new(0, 0), &one_port_component())
        .unwrap();
    let attached = harness.read();
    assert_eq!(attached.links.len(), 2);
    assert!(attached.links.iter().all(|link| {
        !((link.from == attachment.port(0).unwrap().target() && link.to == motor)
            || (link.from == motor && link.to == attachment.port(0).unwrap().target()))
    }));

    let run = harness.send_physical(&[port_input(&attachment, 1)]);
    assert!(run.naturally_quiescent);
    assert!(harness.read().links.len() > attached.links.len());
}

fn physical_cycle_body(
    attach_unrelated_first: bool,
) -> (Harness, truelearner_core::PhysicalAttachment) {
    let mut builder = HarnessBuilder::with_capacity(128, 512, 1);
    builder.set_physical_tracing(true);
    builder.set_protocol(
        Protocol::RecursiveLearnerCausalTopologyProductCompositionNaturalCycleClosure,
    );
    let motor = builder.add_junction(junction(10, 1, 0, 1));
    let outside = builder.add_junction(junction(11, 1, 1, 1));
    builder.add_link(Link {
        from: motor,
        to: outside,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: u32::MAX,
        mode: TransmissionMode::Drive,
    });
    let mut body = builder.build();
    if attach_unrelated_first {
        body.attach_physical(AttachmentSite::new(20, 0), &one_port_component())
            .unwrap();
    }
    let attachment = body
        .attach_physical(AttachmentSite::new(0, 0), &one_port_component())
        .unwrap();
    if !attach_unrelated_first {
        body.attach_physical(AttachmentSite::new(20, 0), &one_port_component())
            .unwrap();
    }
    (body, attachment)
}

fn physical_input(
    attachment: &truelearner_core::PhysicalAttachment,
    tick: i64,
    origin_physical: u64,
    incidence: PhysicalIncidence,
) -> PhysicalInput {
    PhysicalInput {
        input: Input {
            target: attachment.port(0).unwrap().target(),
            arrival_tick: tick,
            phase: 0,
            impulse: 1,
            origin_physical,
        },
        incidence,
    }
}

fn open_physical_cycle(
    body: &mut Harness,
    attachment: &truelearner_core::PhysicalAttachment,
    tick: i64,
) -> u64 {
    let run = body.send_physical(&[
        physical_input(
            attachment,
            tick,
            attachment.port(0).unwrap().origin_physical(),
            PhysicalIncidence::Transition,
        ),
        motor_opportunity(body, tick.saturating_add(2)),
    ]);
    assert!(run.naturally_quiescent);
    assert_eq!(run.outputs.len(), 1, "{:#?}", run.physical_trace);
    run.outputs[0].from_physical
}

fn motor_opportunity(body: &Harness, tick: i64) -> PhysicalInput {
    motor_opportunity_with_impulse(body, tick, 1)
}

fn motor_opportunity_with_impulse(body: &Harness, tick: i64, impulse: i32) -> PhysicalInput {
    opportunity_for(body, tick, 10, impulse)
}

fn opportunity_for(body: &Harness, tick: i64, physical_id: u64, impulse: i32) -> PhysicalInput {
    let motor = body
        .read()
        .junctions
        .iter()
        .find(|junction| junction.physical_id == physical_id)
        .unwrap()
        .id;
    PhysicalInput {
        input: Input {
            target: motor,
            arrival_tick: tick,
            phase: 0,
            impulse,
            origin_physical: 40_000,
        },
        incidence: PhysicalIncidence::Sample,
    }
}

#[test]
fn physical_cycle_closure_returns_on_the_used_path_before_another_action() {
    let (mut body, attachment) = physical_cycle_body(false);
    let output_origin = open_physical_cycle(&mut body, &attachment, 1);
    let checkpoint = body.save().unwrap();
    let return_tick = body.read().clock.tick.saturating_add(1);
    let returned = physical_input(
        &attachment,
        return_tick,
        output_origin,
        PhysicalIncidence::Transition,
    );

    let run = body.send_physical(&[returned]);
    let mut replay = Harness::restore(checkpoint).unwrap();
    let replay_run = replay.send_physical(&[returned]);

    assert!(run.outputs.is_empty(), "{:#?}", run.physical_trace);
    assert!(run.physical_trace.iter().any(|transition| matches!(
        transition.event,
        PhysicalEvent::NaturalCycleClosed { output, .. }
            if body.read().junction(output).is_some_and(|junction| junction.physical_id == output_origin)
    )));
    assert!(body
        .read()
        .links
        .iter()
        .any(|link| link.last_consequence_tick == Some(return_tick)));
    assert_eq!(replay_run, run);
    assert_eq!(replay.read(), body.read());
    assert!(run.naturally_quiescent);
}

#[test]
fn physical_cycle_closure_treats_samples_and_unrelated_causes_as_identity() {
    for (origin, incidence) in [
        (10, PhysicalIncidence::Sample),
        (999_999, PhysicalIncidence::Transition),
    ] {
        let (mut body, attachment) = physical_cycle_body(false);
        open_physical_cycle(&mut body, &attachment, 1);
        let before = body
            .read()
            .links
            .iter()
            .filter(|link| link.last_consequence_tick.is_some())
            .count();
        let tick = body.read().clock.tick.saturating_add(1);
        let run = body.send_physical(&[physical_input(&attachment, tick, origin, incidence)]);

        assert!(run.physical_trace.iter().any(|transition| matches!(
            transition.event,
            PhysicalEvent::Fire { junction }
                if junction == attachment.port(0).unwrap().target()
        )));
        assert_eq!(
            body.read()
                .links
                .iter()
                .filter(|link| link.last_consequence_tick.is_some())
                .count(),
            before
        );
        assert!(!run.physical_trace.iter().any(|transition| matches!(
            transition.event,
            PhysicalEvent::NaturalCycleClosed { .. }
        )));
        assert!(run.naturally_quiescent);
    }
}

#[test]
fn physical_cycle_closure_rejects_ambiguous_participating_paths() {
    let mut builder = HarnessBuilder::with_capacity(128, 512, 1);
    builder.set_physical_tracing(true);
    builder.set_protocol(
        Protocol::RecursiveLearnerCausalTopologyProductCompositionNaturalCycleClosure,
    );
    let surface = builder.add_junction(junction(41, 0, 0, 1));
    let motor = builder.add_junction(junction(10, 1, 0, 2));
    let sink = builder.add_junction(junction(11, 1, 1, 1));
    for intermediate_physical in [42, 43] {
        let intermediate = builder.add_junction(junction(intermediate_physical, 0, 0, 1));
        builder.add_link(Link {
            from: surface,
            to: intermediate,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: 1,
            mode: TransmissionMode::Drive,
        });
        builder.add_link(Link {
            from: intermediate,
            to: motor,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: 1,
            mode: TransmissionMode::Drive,
        });
    }
    builder.add_link(Link {
        from: motor,
        to: sink,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: u32::MAX,
        mode: TransmissionMode::Drive,
    });
    let mut body = builder.build();
    let opened = body.send_physical(&[
        PhysicalInput {
            input: Input {
                target: surface,
                arrival_tick: 1,
                phase: 0,
                impulse: 1,
                origin_physical: 41,
            },
            incidence: PhysicalIncidence::Transition,
        },
        opportunity_for(&body, 3, 10, 1),
    ]);
    assert_eq!(opened.outputs.len(), 1, "{:#?}", opened.physical_trace);

    let return_tick = body.read().clock.tick.saturating_add(1);
    let returned = body.send_physical(&[PhysicalInput {
        input: Input {
            target: surface,
            arrival_tick: return_tick,
            phase: 0,
            impulse: 1,
            origin_physical: opened.outputs[0].from_physical,
        },
        incidence: PhysicalIncidence::Transition,
    }]);

    assert!(
        returned.physical_trace.iter().any(|transition| matches!(
            transition.event,
            PhysicalEvent::NaturalCycleClosureEvaluated { matching_paths, .. }
                if matching_paths > 1
        )),
        "{:#?}",
        returned.physical_trace
    );
    assert!(!returned
        .physical_trace
        .iter()
        .any(|transition| matches!(transition.event, PhysicalEvent::NaturalCycleClosed { .. })));
    assert!(body
        .read()
        .links
        .iter()
        .all(|link| link.last_consequence_tick != Some(return_tick)));
    assert!(returned.naturally_quiescent);
}

fn close_two_cycles(attach_unrelated_first: bool) -> (Vec<u64>, usize, usize) {
    let (mut body, attachment) = physical_cycle_body(attach_unrelated_first);
    let mut outputs = Vec::new();
    for cycle in 0..2_i64 {
        let open_tick = body.read().clock.tick.saturating_add(1);
        let output = open_physical_cycle(&mut body, &attachment, open_tick);
        outputs.push(output);
        let close_tick = body.read().clock.tick.saturating_add(1);
        let run = body.send_physical(&[physical_input(
            &attachment,
            close_tick,
            output,
            PhysicalIncidence::Transition,
        )]);
        assert!(
            run.outputs.is_empty(),
            "cycle {cycle}: {:#?}",
            run.physical_trace
        );
        assert!(run.naturally_quiescent);
    }
    let observation = body.read();
    let owned_links = observation
        .learners
        .first()
        .map_or(0, |learner| learner.links.len());
    (outputs, observation.learners.len(), owned_links)
}

#[test]
fn physical_cycle_composition_forms_one_existing_learner_and_ignores_attachment_order() {
    let after = close_two_cycles(false);
    let before = close_two_cycles(true);

    assert_eq!(after.0, vec![10, 10]);
    assert_eq!(before.0, after.0);
    assert_eq!(after.1, 1);
    assert_eq!(before.1, after.1);
    assert!(after.2 >= 2);
    assert_eq!(before.2, after.2);
}
