use truelearner_workstation::{
    BodyAxis, ContactSample, Digit, Eye, LightField, WorkstationCheckpoint, WorkstationHarness,
    WorldSample, AXIS_COUNT, BODY_MAX, TOUCH_SITES,
};

fn centered_light(value: u8) -> LightField {
    let width = 33_u16;
    let height = 33_u16;
    let mut pixels = vec![0_u8; usize::from(width) * usize::from(height)];
    pixels[usize::from(height / 2) * usize::from(width) + usize::from(width / 2)] = value;
    LightField::new(width, height, pixels).unwrap()
}

fn sample() -> WorldSample {
    WorldSample::new(
        [centered_light(255), centered_light(192)],
        [ContactSample::default(); TOUCH_SITES],
    )
    .unwrap()
}

#[test]
fn physical_exploration_returns_only_actual_changed_axes() {
    let mut body = WorkstationHarness::new(91).unwrap();
    let (candidate, projected) = body.transition(sample()).unwrap();
    let first = body.step(sample()).unwrap();
    assert_eq!(projected, first);
    assert_eq!(candidate.save().unwrap(), body.save().unwrap());
    assert!(first.returned_transitions.is_empty());
    assert!(first.naturally_quiescent);

    let mut changed = first;
    for _ in 0..3 {
        if !changed.pending_transitions.is_empty() {
            break;
        }
        changed = body.step(sample()).unwrap();
    }
    assert!(changed.metrics.structural_proposals > 0);
    assert!(!changed.pending_transitions.is_empty());
    assert!(changed.pose_changed);
    assert!(changed
        .pending_transitions
        .iter()
        .all(|axis| changed.movements.iter().any(|movement| {
            movement.axis == *axis && movement.changed && movement.velocity != 0
        })));

    let expected = changed.pending_transitions.clone();
    let returned = body.step(sample()).unwrap();
    assert_eq!(returned.returned_transitions, expected);
    assert!(returned.naturally_quiescent);
}

#[test]
fn reads_are_inert_and_checkpoint_restores_the_exact_next_step() {
    let mut body = WorkstationHarness::new(92).unwrap();
    body.step(sample()).unwrap();
    body.step(sample()).unwrap();
    let read = body.read().unwrap();
    assert_eq!(body.read().unwrap(), read);

    let checkpoint = body.save().unwrap();
    let bytes = checkpoint.canonical_bytes().unwrap();
    let mut restored =
        WorkstationHarness::restore(WorkstationCheckpoint::decode(&bytes).unwrap()).unwrap();
    assert_eq!(restored.read().unwrap(), body.read().unwrap());
    assert_eq!(
        restored.step(sample()).unwrap(),
        body.step(sample()).unwrap()
    );
    assert_eq!(restored.save().unwrap(), body.save().unwrap());
}

#[test]
fn corrupt_and_invalid_external_values_fail_closed() {
    let body = WorkstationHarness::new(93).unwrap();
    let before = body.save().unwrap();
    let mut bytes = before.canonical_bytes().unwrap();
    bytes[30] ^= 1;
    assert!(WorkstationCheckpoint::decode(&bytes).is_err());

    let invalid = r#"{"eyes":[{"width":1,"height":1,"pixels":[]},{"width":1,"height":1,"pixels":[0]}],"contacts":[{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0},{"pressure":0,"slip":0}]}"#;
    let sample = serde_json::from_str(invalid).unwrap();
    let mut candidate = body.clone();
    assert!(candidate.step(sample).is_err());
    assert_eq!(candidate.save().unwrap(), before);
}

#[test]
fn two_eyes_one_hand_and_five_fingertips_have_stable_positions() {
    let body = WorkstationHarness::new(94).unwrap();
    let state = body.read().unwrap().state;
    assert_eq!(state.eye(Eye::Left).gaze(), state.eye(Eye::Right).gaze());

    let fingertips = Digit::ALL.map(|digit| state.hand().fingertip(digit));
    assert_eq!(fingertips.len(), 5);
    assert!(fingertips.iter().all(|point| {
        (0..=BODY_MAX).contains(&point.x())
            && (0..=BODY_MAX).contains(&point.y())
            && (0..=BODY_MAX).contains(&point.depth())
    }));

    let proprioception = state.proprioception();
    assert_eq!(proprioception.len(), AXIS_COUNT);
    assert_eq!(proprioception.map(|sense| sense.axis), BodyAxis::ALL);
    assert!(proprioception
        .iter()
        .all(|sense| sense.position == 0 && sense.velocity == 0));
}
