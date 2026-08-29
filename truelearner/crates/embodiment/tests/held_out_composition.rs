use truelearner_embodiment::{
    parallel, BoundedAxis, ChangeDetector, Driver, DriverBank, Incidence, OpposedEffort, Origin,
    Signal,
};

fn sample(origin: u64, value: u8) -> Signal<u8> {
    Signal::new(Origin(origin), Incidence::Sample, value)
}

fn effort(origin: u64, decrease: u16, increase: u16) -> Signal<OpposedEffort> {
    Signal::new(
        Origin(origin),
        Incidence::Sample,
        OpposedEffort::new(decrease, increase),
    )
}

#[test]
fn three_parallel_sensor_factors_retain_independent_histories() {
    let mut sensors = parallel(
        parallel(ChangeDetector::default(), ChangeDetector::default()),
        ChangeDetector::default(),
    );
    sensors.step(((sample(1, 10), sample(2, 20)), sample(3, 30)));
    let next = sensors.step(((sample(1, 10), sample(2, 21)), sample(3, 30)));

    assert_eq!(next.0 .0.incidence, Incidence::Sample);
    assert_eq!(next.0 .1.incidence, Incidence::Transition);
    assert_eq!(next.1.incidence, Incidence::Sample);
    assert_eq!(next.0 .1.origin, Origin(2));
}

#[test]
fn seven_actuator_factors_preserve_local_change_and_feedback() {
    let mut axes = DriverBank::new(
        (0..7)
            .map(|_| BoundedAxis::new(0, -5, 5).unwrap())
            .collect(),
    );
    let outputs = axes
        .step(
            (0..7)
                .map(|index| {
                    if index == 5 {
                        effort(100 + index, 3, 0)
                    } else {
                        effort(100 + index, 0, 0)
                    }
                })
                .collect(),
        )
        .unwrap();

    assert_eq!(
        outputs
            .iter()
            .map(|output| output.actual)
            .collect::<Vec<_>>(),
        vec![0, 0, 0, 0, 0, -3, 0]
    );
    assert_eq!(outputs[5].feedback.origin, Origin(105));
    assert_eq!(outputs[5].feedback.incidence, Incidence::Transition);
}
