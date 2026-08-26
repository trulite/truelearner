use truelearner_human::{
    BodyAxis, ContactSample, HumanCheckpoint, HumanHarness, LightField, Side, WorldSample,
    AXIS_COUNT, TOUCH_SITES,
};

fn centered_light() -> LightField {
    let width = 33_u16;
    let height = 33_u16;
    let mut pixels = vec![0_u8; usize::from(width) * usize::from(height)];
    pixels[usize::from(height / 2) * usize::from(width) + usize::from(width / 2)] = 255;
    LightField::new(width, height, pixels).unwrap()
}

fn sample() -> WorldSample {
    WorldSample::new(
        [centered_light(), centered_light()],
        [[ContactSample::default(); TOUCH_SITES]; 2],
    )
    .unwrap()
}

#[test]
fn repeated_physical_input_forms_balanced_paths_without_a_fake_outcome() {
    let mut body = HumanHarness::new(91).unwrap();
    let formed = body.step(sample()).unwrap();
    assert!(formed.metrics.structural_proposals > 0);
    let balanced = if formed.crossings.is_empty() {
        body.step(sample()).unwrap()
    } else {
        formed
    };
    assert!(!balanced.crossings.is_empty());
    assert!(!balanced.movements.is_empty());
    assert!(!balanced.pose_changed);
    assert!(!balanced.pending_outcome);
    assert!(balanced.movements.iter().all(|movement| !movement.changed
        && movement.velocity == 0
        && movement.net_impulse == 0
        && movement.decrease_effort == movement.increase_effort));
    assert!(balanced.naturally_quiescent);

    let without_outcome = body.step(sample()).unwrap();
    assert_eq!(without_outcome.metrics.plasticity_updates, 0);
    assert!(!without_outcome.pending_outcome);
    assert!(without_outcome.naturally_quiescent);
}

#[test]
fn reads_are_inert_and_checkpoint_restores_the_exact_next_step() {
    let mut body = HumanHarness::new(92).unwrap();
    body.step(sample()).unwrap();
    body.step(sample()).unwrap();
    let read = body.read().unwrap();
    assert_eq!(body.read().unwrap(), read);

    let checkpoint = body.save().unwrap();
    let bytes = checkpoint.canonical_bytes().unwrap();
    let mut restored = HumanHarness::restore(HumanCheckpoint::decode(&bytes).unwrap()).unwrap();
    assert_eq!(restored.read().unwrap(), body.read().unwrap());
    assert_eq!(
        restored.step(sample()).unwrap(),
        body.step(sample()).unwrap()
    );
    assert_eq!(restored.save().unwrap(), body.save().unwrap());
}

#[test]
fn corrupt_and_invalid_external_values_fail_closed() {
    let body = HumanHarness::new(93).unwrap();
    let before = body.save().unwrap();
    let mut bytes = before.canonical_bytes().unwrap();
    bytes[30] ^= 1;
    assert!(HumanCheckpoint::decode(&bytes).is_err());

    let invalid = r#"{"eyes":[{"width":1,"height":1,"pixels":[]},{"width":1,"height":1,"pixels":[0]}],"contacts":[[{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0}],[{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0}]]}"#;
    let sample = serde_json::from_str(invalid).unwrap();
    let mut candidate = body.clone();
    assert!(candidate.step(sample).is_err());
    assert_eq!(candidate.save().unwrap(), before);
}

#[test]
fn two_eyes_and_ten_fingertips_have_stable_public_positions() {
    let body = HumanHarness::new(94).unwrap();
    let state = body.read().unwrap().state;
    assert_eq!(
        state.eyes().focus(Side::Left),
        state.eyes().focus(Side::Right)
    );
    let mut fingertip_count = 0;
    for side in [Side::Left, Side::Right] {
        let hand = state.hand(side);
        let tips = truelearner_human::Digit::ALL.map(|digit| hand.fingertip(digit));
        assert_eq!(tips.len(), 5);
        fingertip_count += tips.len();
        assert!(tips.iter().all(|point| {
            (0..=truelearner_human::BODY_MAX).contains(&point.x())
                && (0..=truelearner_human::BODY_MAX).contains(&point.y())
        }));
    }
    assert_eq!(fingertip_count, 10);

    let proprioception = state.proprioception();
    assert_eq!(proprioception.len(), AXIS_COUNT);
    assert_eq!(proprioception.map(|sense| sense.axis), BodyAxis::ALL);
    assert!(proprioception
        .iter()
        .all(|sense| sense.position == 0 && sense.velocity == 0));
}
