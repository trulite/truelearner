use std::cell::Cell;

use truelearner_embodiment::{
    calibrate, parallel, Availability, Driver, DriverBank, EffectMode, Incidence, Normalizer,
    Origin, Residual, Signal, SpatialField,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Band {
    low: i16,
    high: i16,
}

fn band_residual(body: &Band, value: &i16) -> Residual {
    let amount = if *value < body.low {
        body.low.abs_diff(*value)
    } else if *value > body.high {
        value.abs_diff(body.high)
    } else {
        0
    };
    Residual::new(u32::from(amount))
}

fn observed<T>(origin: u64, incidence: Incidence, value: T) -> Signal<Availability<T>> {
    Signal::new(Origin(origin), incidence, Availability::Available(value))
}

#[test]
fn calibration_laws_curries_body_context_and_preserves_signal() {
    let mut normalizer = calibrate(Band { low: 4, high: 6 }, band_residual);
    let output = normalizer.step(observed(41, Incidence::Transition, 9));

    assert_eq!(normalizer.body(), &Band { low: 4, high: 6 });
    assert_eq!(output.origin, Origin(41));
    assert_eq!(output.incidence, Incidence::Transition);
    assert_eq!(output.value, Availability::Available(Residual::new(3)));
}

#[test]
fn calibration_laws_unavailable_never_evaluates_relation() {
    let calls = Cell::new(0_u32);
    let mut normalizer = calibrate(7_i16, |body: &i16, value: &i16| {
        calls.set(calls.get().saturating_add(1));
        Residual::new(u32::from(body.abs_diff(*value)))
    });

    let output = normalizer.step(Signal::new(
        Origin(8),
        Incidence::Sample,
        Availability::<i16>::Unavailable,
    ));

    assert_eq!(calls.get(), 0);
    assert_eq!(output.value, Availability::Unavailable);
}

#[test]
fn calibration_laws_unavailable_body_context_does_not_construct_a_normalizer() {
    let context = Availability::<Band>::Unavailable;
    let normalizer = context.map(|body| calibrate(body, band_residual));

    assert!(matches!(normalizer, Availability::Unavailable));
}

#[test]
fn calibration_laws_residual_identity_association_and_effect() {
    let a = Residual::new(2);
    let b = Residual::new(3);
    let c = Residual::new(5);

    assert_eq!(Residual::ZERO.combine(a), a);
    assert_eq!(a.combine(Residual::ZERO), a);
    assert_eq!(a.combine(b).combine(c), a.combine(b.combine(c)));
    assert_eq!(Residual::ZERO.effect_mode(), EffectMode::Identity);
    assert!(!Residual::ZERO.has_opportunity());
    assert_eq!(a.effect_mode(), EffectMode::Apply);
    assert!(a.has_opportunity());
    assert_eq!(Residual::new(u32::MAX).combine(a).amount(), u32::MAX);
}

#[test]
fn calibration_laws_parallel_composition_preserves_local_factors() {
    let mut pair = parallel(
        calibrate(Band { low: -1, high: 1 }, band_residual),
        calibrate(Band { low: 9, high: 11 }, band_residual),
    );

    let (left, right) = pair.step((
        observed(1, Incidence::Sample, -4),
        observed(2, Incidence::Transition, 10),
    ));

    assert_eq!(left.value, Availability::Available(Residual::new(3)));
    assert_eq!(right.value, Availability::Available(Residual::ZERO));
    assert_eq!((left.origin, right.origin), (Origin(1), Origin(2)));
    assert_eq!(right.incidence, Incidence::Transition);
}

#[test]
fn calibration_ladder_finger_pressure_uses_body_supplied_normal() {
    let mut finger = calibrate(Band { low: 18, high: 22 }, band_residual);

    assert_eq!(
        finger.step(observed(101, Incidence::Sample, 20)).value,
        Availability::Available(Residual::ZERO)
    );
    assert_eq!(
        finger.step(observed(101, Incidence::Transition, 27)).value,
        Availability::Available(Residual::new(5))
    );
}

#[test]
fn calibration_ladder_hand_keeps_five_fingers_independent() {
    type FingerNormalizer = Normalizer<Band, fn(&Band, &i16) -> Residual>;
    let limits = [
        Band { low: 4, high: 6 },
        Band { low: 5, high: 7 },
        Band { low: 6, high: 8 },
        Band { low: 7, high: 9 },
        Band { low: 8, high: 10 },
    ];
    let mut hand = DriverBank::<FingerNormalizer>::new(
        limits
            .into_iter()
            .map(|limit| calibrate(limit, band_residual as fn(&Band, &i16) -> Residual))
            .collect(),
    );

    let output = hand
        .step(vec![
            observed(200, Incidence::Sample, 5),
            observed(201, Incidence::Sample, 6),
            observed(202, Incidence::Sample, 12),
            observed(203, Incidence::Sample, 8),
            observed(204, Incidence::Sample, 9),
        ])
        .unwrap();

    assert_eq!(output[2].value, Availability::Available(Residual::new(4)));
    assert!(output
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != 2)
        .all(|(_, signal)| signal.value == Availability::Available(Residual::ZERO)));
    assert_eq!(output[2].origin, Origin(202));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EyeContext {
    resting_offset: i16,
}

fn eye_residual(body: &EyeContext, offset: &i16) -> Residual {
    Residual::new(u32::from(body.resting_offset.abs_diff(*offset)))
}

#[test]
fn calibration_ladder_binocular_eye_composes_available_and_missing_views() {
    let mut eyes = parallel(
        calibrate(EyeContext { resting_offset: -2 }, eye_residual),
        calibrate(EyeContext { resting_offset: 2 }, eye_residual),
    );

    let (left, right) = eyes.step((
        observed(301, Incidence::Transition, 1),
        Signal::new(
            Origin(302),
            Incidence::Sample,
            Availability::<i16>::Unavailable,
        ),
    ));

    assert_eq!(left.value, Availability::Available(Residual::new(3)));
    assert_eq!(right.value, Availability::Unavailable);
    assert_eq!(right.origin, Origin(302));
}

#[test]
fn calibration_ladder_ear_does_not_turn_silence_or_dropout_into_a_measurement() {
    let mut ear = calibrate(Band { low: 28, high: 32 }, band_residual);

    let quiet = ear.step(observed(401, Incidence::Sample, 30));
    let loud = ear.step(observed(401, Incidence::Transition, 41));
    let dropout = ear.step(Signal::new(
        Origin(401),
        Incidence::Transition,
        Availability::<i16>::Unavailable,
    ));

    assert_eq!(quiet.value, Availability::Available(Residual::ZERO));
    assert_eq!(loud.value, Availability::Available(Residual::new(9)));
    assert_eq!(dropout.value, Availability::Unavailable);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct VoiceContext {
    resting_spectrum: [u8; 4],
}

fn voice_residual(body: &VoiceContext, spectrum: &[u8; 4]) -> Residual {
    let amount = body
        .resting_spectrum
        .iter()
        .zip(spectrum)
        .map(|(normal, actual)| u32::from(normal.abs_diff(*actual)))
        .fold(0_u32, u32::saturating_add);
    Residual::new(amount)
}

#[test]
fn calibration_ladder_voice_accepts_structured_delayed_feedback() {
    let mut voice = calibrate(
        VoiceContext {
            resting_spectrum: [10, 20, 30, 40],
        },
        voice_residual,
    );

    let output = voice.step(observed(501, Incidence::Transition, [12, 20, 27, 41]));

    assert_eq!(output.value, Availability::Available(Residual::new(6)));
    assert_eq!(output.origin, Origin(501));
    assert_eq!(output.incidence, Incidence::Transition);
}

#[test]
fn calibration_ladder_mixed_modalities_compose_without_a_shared_value_type() {
    let mut sensorium = parallel(
        calibrate(Band { low: 1, high: 2 }, band_residual),
        calibrate(
            VoiceContext {
                resting_spectrum: [1, 1, 1, 1],
            },
            voice_residual,
        ),
    );

    let (touch, voice) = sensorium.step((
        observed(601, Incidence::Sample, -2_i16),
        observed(602, Incidence::Transition, [1, 3, 1, 4]),
    ));

    assert_eq!(touch.value, Availability::Available(Residual::new(3)));
    assert_eq!(voice.value, Availability::Available(Residual::new(5)));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FieldContext {
    resting_level: u8,
}

fn field_residual(body: &FieldContext, field: &SpatialField<u8, 2>) -> Residual {
    field
        .cells()
        .iter()
        .fold(Residual::ZERO, |sum, cell| match cell {
            Availability::Available(value) => sum.combine(Residual::new(u32::from(
                body.resting_level.abs_diff(*value),
            ))),
            Availability::Unavailable => sum,
        })
}

#[test]
fn calibration_ladder_held_out_spatial_field_uses_the_same_transformation() {
    let field = SpatialField::new(
        [2, 2],
        vec![
            Availability::Available(4),
            Availability::Available(7),
            Availability::Unavailable,
            Availability::Available(5),
        ],
    )
    .unwrap();
    let mut held_out = calibrate(FieldContext { resting_level: 5 }, field_residual);

    let output = held_out.step(observed(701, Incidence::Transition, field));

    assert_eq!(output.value, Availability::Available(Residual::new(3)));
    assert_eq!(output.origin, Origin(701));
}
