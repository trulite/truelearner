use truelearner_embodiment::{
    Availability, ChangeDetector, CommandCollector, Driver, DriverBank, EffectMode, Incidence,
    OpposedEffort, Origin, Port, Signal,
};

fn observed<T>(origin: u64, value: T) -> Signal<T> {
    Signal::new(Origin(origin), Incidence::Sample, value)
}

#[test]
fn availability_maps_and_pairs_without_inventing_an_occluded_value() {
    let available = Availability::Available(12_i16);
    assert_eq!(
        available.map(|value| value * 2),
        Availability::Available(24)
    );
    assert_eq!(
        Availability::Available(12_i16).zip(Availability::Available(5_i16)),
        Availability::Available((12, 5))
    );
    assert_eq!(
        Availability::Available(12_i16).zip(Availability::<i16>::Unavailable),
        Availability::Unavailable
    );
    assert_eq!(
        Availability::<i16>::Unavailable.map(|value| value * 2),
        Availability::Unavailable
    );
}

#[test]
fn disappearance_and_reappearance_are_honest_sensor_transitions() {
    let mut detector = ChangeDetector::default();

    let present = detector.step(observed(10, Availability::Available(80_u8)));
    let occluded = detector.step(observed(10, Availability::Unavailable));
    let still_occluded = detector.step(observed(10, Availability::Unavailable));
    let visible_again = detector.step(observed(10, Availability::Available(80_u8)));

    assert_eq!(present.incidence, Incidence::Sample);
    assert_eq!(occluded.incidence, Incidence::Transition);
    assert_eq!(occluded.value, Availability::Unavailable);
    assert_eq!(still_occluded.incidence, Incidence::Sample);
    assert_eq!(visible_again.incidence, Incidence::Transition);
}

#[test]
fn a_sensor_field_preserves_multiple_simultaneous_candidates() {
    let detectors = (0..3).map(|_| ChangeDetector::default()).collect();
    let mut field = DriverBank::new(detectors);
    let output = field
        .step(vec![
            observed(20, Availability::Available(41_u8)),
            observed(21, Availability::Unavailable),
            observed(22, Availability::Available(173_u8)),
        ])
        .unwrap();

    assert_eq!(output.len(), 3);
    assert_eq!(output[0].origin, Origin(20));
    assert_eq!(output[0].value, Availability::Available(41));
    assert_eq!(output[1].origin, Origin(21));
    assert_eq!(output[1].value, Availability::Unavailable);
    assert_eq!(output[2].origin, Origin(22));
    assert_eq!(output[2].value, Availability::Available(173));
}

#[test]
fn binocular_relation_routes_a_command_only_while_both_views_are_available() {
    let relation = Availability::Available(18_i16)
        .zip(Availability::Available(11_i16))
        .map(|(left, right)| left - right);
    let occluded_relation = Availability::Available(18_i16)
        .zip(Availability::<i16>::Unavailable)
        .map(|(left, right)| left - right);

    let mut commands = CommandCollector::<OpposedEffort, 1>::new();
    if let Availability::Available(disparity) = relation {
        commands
            .add(
                Port(0),
                OpposedEffort::new(0, disparity.unsigned_abs()),
                |left, right| left.combine_bounded(right, 32),
            )
            .unwrap();
    }
    if let Availability::Available(disparity) = occluded_relation {
        commands
            .add(
                Port(0),
                OpposedEffort::new(0, disparity.unsigned_abs()),
                |left, right| left.combine_bounded(right, 32),
            )
            .unwrap();
    }

    assert_eq!(
        commands.finish().command(Port(0)).unwrap(),
        Some(&OpposedEffort::new(0, 7))
    );
}

#[test]
fn all_eye_sources_join_before_the_finished_effect_becomes_identity() {
    let mut commands = CommandCollector::<OpposedEffort, 2>::new();
    commands
        .add(Port(0), OpposedEffort::new(0, 3), |left, right| {
            left.combine_bounded(right, 16)
        })
        .unwrap();
    commands
        .add(Port(0), OpposedEffort::new(0, 5), |left, right| {
            left.combine_bounded(right, 16)
        })
        .unwrap();
    commands
        .add(Port(1), OpposedEffort::new(4, 0), |left, right| {
            left.combine_bounded(right, 16)
        })
        .unwrap();

    let mut frame = commands.finish();
    assert_eq!(
        frame.command(Port(0)).unwrap(),
        Some(&OpposedEffort::new(0, 8))
    );
    frame.constrain(Port(0), EffectMode::Identity).unwrap();

    assert_eq!(frame.command(Port(0)).unwrap(), None);
    assert_eq!(
        frame.command(Port(1)).unwrap(),
        Some(&OpposedEffort::new(4, 0))
    );
}

#[test]
fn the_same_finished_effect_constraint_is_local_across_five_fingers() {
    let mut commands = CommandCollector::<OpposedEffort, 5>::new();
    for port in 0..5 {
        commands
            .add(
                Port(port),
                OpposedEffort::new(0, port as u16 + 1),
                |left, right| left.combine_bounded(right, 16),
            )
            .unwrap();
    }

    let mut frame = commands.finish();
    frame.constrain(Port(2), EffectMode::Identity).unwrap();
    frame.constrain(Port(3), EffectMode::Apply).unwrap();

    assert_eq!(frame.command(Port(2)).unwrap(), None);
    assert_eq!(
        frame.command(Port(3)).unwrap(),
        Some(&OpposedEffort::new(0, 4))
    );
    assert!(frame
        .commands()
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != 2)
        .all(|(_, command)| command.is_some()));
}

#[test]
fn bounded_command_join_has_identity_and_association() {
    let identity = OpposedEffort::default();
    let first = OpposedEffort::new(2, 3);
    let second = OpposedEffort::new(5, 7);
    let third = OpposedEffort::new(11, 13);

    assert_eq!(first.combine_bounded(identity, 16), first);
    assert_eq!(identity.combine_bounded(first, 16), first);
    assert_eq!(
        first.combine_bounded(second, 16).combine_bounded(third, 16),
        first.combine_bounded(second.combine_bounded(third, 16), 16)
    );
}

#[test]
fn invalid_port_does_not_change_the_command_collector() {
    let mut commands = CommandCollector::<OpposedEffort, 2>::new();
    let before = commands.clone();
    assert!(commands
        .add(Port(2), OpposedEffort::new(0, 1), |left, right| {
            left.combine_bounded(right, 16)
        })
        .is_err());
    assert_eq!(commands, before);
}

#[test]
fn held_out_seven_port_frame_preserves_unconstrained_neighbors() {
    let mut commands = CommandCollector::<u16, 7>::new();
    for port in 0..7 {
        commands
            .add(Port(port), port as u16 + 10, |left, right| {
                left.saturating_add(right)
            })
            .unwrap();
    }
    let mut frame = commands.finish();
    frame.constrain(Port(5), EffectMode::Identity).unwrap();

    assert_eq!(frame.command(Port(5)).unwrap(), None);
    assert_eq!(frame.command(Port(4)).unwrap(), Some(&14));
    assert_eq!(frame.command(Port(6)).unwrap(), Some(&16));
}
