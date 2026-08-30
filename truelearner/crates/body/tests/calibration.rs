use std::cell::Cell;

use truelearner_body::{attach, calibrate, Arrival, Body, Join, Junction, OpenBody, Residual};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Band {
    low: i16,
    high: i16,
}

fn distance(body: &Band, value: &i16) -> Residual {
    let amount = if *value < body.low {
        body.low.abs_diff(*value)
    } else if *value > body.high {
        value.abs_diff(body.high)
    } else {
        0
    };
    Residual::new(u32::from(amount))
}

#[test]
fn calibration_curries_body_context() {
    let mut normalizer = calibrate(Band { low: 4, high: 6 }, distance);

    assert_eq!(normalizer.body(), &Band { low: 4, high: 6 });
    assert_eq!(normalizer.step(Some(9)), Some(Residual::new(3)));
    assert_eq!(normalizer.step(Some(5)), Some(Residual::ZERO));
}

#[test]
fn unavailable_reading_stays_unavailable_without_calling_the_relation() {
    let calls = Cell::new(0_u32);
    let mut normalizer = calibrate(7_i16, |body: &i16, value: &i16| {
        calls.set(calls.get().saturating_add(1));
        Residual::new(u32::from(body.abs_diff(*value)))
    });

    assert_eq!(normalizer.step::<i16>(None), None);
    assert_eq!(calls.get(), 0);
}

#[test]
fn residual_has_identity_association_and_saturation() {
    let a = Residual::new(2);
    let b = Residual::new(3);
    let c = Residual::new(5);

    assert_eq!(Residual::ZERO.combine(a), a);
    assert_eq!(a.combine(Residual::ZERO), a);
    assert_eq!(a.combine(b).combine(c), a.combine(b.combine(c)));
    assert!(Residual::ZERO.is_quiet());
    assert!(!a.is_quiet());
    assert_eq!(Residual::new(u32::MAX).combine(a).amount(), u32::MAX);
}

#[test]
fn calibration_transfers_to_a_structured_reading() {
    let mut normalizer = calibrate([10_u8, 20, 30, 40], |body: &[u8; 4], value: &[u8; 4]| {
        Residual::new(
            body.iter()
                .zip(value)
                .map(|(normal, actual)| u32::from(normal.abs_diff(*actual)))
                .sum(),
        )
    });

    assert_eq!(
        normalizer.step(Some([12, 20, 27, 41])),
        Some(Residual::new(6))
    );
}

#[test]
fn calibrated_reading_crosses_an_attachment_without_losing_its_cause() {
    let mut host = Body::default();
    let received = host.add_junction(Junction::integrating(1)).unwrap();
    let mut sensor = Body::default();
    let residual = sensor.add_junction(Junction::integrating(3)).unwrap();
    let sensor = OpenBody::new(sensor, vec![residual]).unwrap();
    let port = sensor.port(0).unwrap();
    let attachment = attach(&mut host, sensor, &[Join::into_host(received, port, 0, 1)]).unwrap();

    let mut normalizer = calibrate(Band { low: 4, high: 6 }, distance);
    let impulse = normalizer.step(Some(9)).unwrap().amount() as i32;
    host.inputs(
        1,
        &[Arrival::caused(attachment.port(port).unwrap(), impulse, 55)],
    )
    .unwrap();
    let mut fired = Vec::new();
    host.run(4, |event| fired.push(event)).unwrap();

    assert_eq!(fired.len(), 2);
    assert_eq!(fired[1].junction, received);
    assert_eq!(fired[1].cause, 55);
}
