use truelearner_embodiment::{
    parallel, BoundedAxis, ChangeDetector, Driver, DriverBank, Incidence, OpportunityGate,
    OpposedEffort, Origin, Signal,
};

fn sample(origin: u64, value: u8) -> Signal<u8> {
    Signal::new(Origin(origin), Incidence::Sample, value)
}

#[test]
fn opportunity_gate_composes_an_event_or_the_identity() {
    let mut gate = OpportunityGate;
    assert_eq!(gate.step((true, sample(30, 7))), Some(sample(30, 7)));
    assert_eq!(gate.step((false, sample(31, 9))), None);
}

fn effort(origin: u64, decrease: u16, increase: u16) -> Signal<OpposedEffort> {
    Signal::new(
        Origin(origin),
        Incidence::Sample,
        OpposedEffort::new(decrease, increase),
    )
}

#[test]
fn binocular_drivers_keep_two_views_and_two_histories() {
    let mut eyes = parallel(ChangeDetector::default(), ChangeDetector::default());
    let first = eyes.step((sample(10, 80), sample(11, 120)));
    assert_eq!(
        (first.0.incidence, first.1.incidence),
        (Incidence::Sample, Incidence::Sample)
    );

    let second = eyes.step((sample(10, 80), sample(11, 121)));
    assert_eq!(second.0.incidence, Incidence::Sample);
    assert_eq!(second.1.incidence, Incidence::Transition);
    assert_eq!((second.0.origin, second.1.origin), (Origin(10), Origin(11)));
}

#[test]
fn five_finger_axes_move_independently_and_return_their_causes() {
    let axes = (0..5)
        .map(|_| BoundedAxis::new(5, 0, 10).unwrap())
        .collect();
    let mut hand = DriverBank::new(axes);
    let outputs = hand
        .step(vec![
            effort(20, 0, 0),
            effort(21, 0, 0),
            effort(22, 0, 2),
            effort(23, 0, 0),
            effort(24, 0, 0),
        ])
        .unwrap();

    assert_eq!(
        outputs
            .iter()
            .map(|output| output.actual)
            .collect::<Vec<_>>(),
        vec![0, 0, 2, 0, 0]
    );
    assert_eq!(outputs[2].feedback.origin, Origin(22));
    assert_eq!(outputs[2].feedback.incidence, Incidence::Transition);
    assert!(outputs
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != 2)
        .all(|(_, output)| output.feedback.incidence == Incidence::Sample));
}

#[test]
fn wrong_hand_arity_fails_before_any_finger_moves() {
    let axes = (0..5)
        .map(|_| BoundedAxis::new(5, 0, 10).unwrap())
        .collect();
    let mut hand = DriverBank::new(axes);
    let before = hand.clone();
    assert!(hand.step(vec![effort(1, 0, 1)]).is_err());
    assert_eq!(hand, before);
}
