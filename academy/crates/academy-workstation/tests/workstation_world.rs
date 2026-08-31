use academy_workstation::{
    KeyId, MonitorFrame, SessionCheckpoint, WorkstationPresentation, WorkstationSession,
    WorkstationWorld, KEY_COUNT,
};
use truelearner_workstation::{Eye, WorkstationHarness};

#[test]
fn public_session_uses_real_monitor_pixels_and_proper_keyboard_geometry() {
    let mut session = WorkstationSession::new(71_001).unwrap();
    let observation = session.step().unwrap();
    assert_eq!(KEY_COUNT, 104);
    for eye in Eye::ALL {
        let field = observation.sample.eye(eye);
        assert_eq!((field.width(), field.height()), (640, 360));
        let minimum = field.pixels().iter().copied().min().unwrap();
        let maximum = field.pixels().iter().copied().max().unwrap();
        assert!(minimum < maximum);
    }
    assert_ne!(
        observation.sample.eye(Eye::Left),
        observation.sample.eye(Eye::Right)
    );
    assert!(observation.body.naturally_quiescent);
}

#[test]
fn checkpoint_restores_the_exact_next_world_and_body_step() {
    let mut session = WorkstationSession::new(71_002).unwrap();
    session.step().unwrap();
    session.step().unwrap();
    let read = session.read().unwrap();
    assert_eq!(session.read().unwrap(), read);

    let checkpoint = session.save().unwrap();
    let bytes = checkpoint.canonical_bytes().unwrap();
    let mut restored =
        WorkstationSession::restore(SessionCheckpoint::decode(&bytes).unwrap()).unwrap();
    assert_eq!(restored.read().unwrap(), session.read().unwrap());
    assert_eq!(restored.step().unwrap(), session.step().unwrap());
    assert_eq!(restored.save().unwrap(), session.save().unwrap());
}

#[test]
fn organism_sample_contains_no_device_or_evaluator_fields() {
    let mut session = WorkstationSession::new(71_003).unwrap();
    let observation = session.step().unwrap();
    let wire = serde_json::to_string(&observation.sample).unwrap();
    for forbidden in [
        "key",
        "cursor",
        "click",
        "character",
        "target",
        "expected",
        "score",
        "capability",
        "action",
        "image",
    ] {
        assert!(!wire.contains(forbidden), "leaked {forbidden}: {wire}");
    }
}

#[test]
fn corrupt_session_checkpoint_fails_closed() {
    let session = WorkstationSession::new(71_004).unwrap();
    let before = session.read().unwrap();
    let mut bytes = session.save().unwrap().canonical_bytes().unwrap();
    bytes[31] ^= 1;
    assert!(SessionCheckpoint::decode(&bytes).is_err());
    assert_eq!(session.read().unwrap(), before);
}

#[test]
fn illuminated_real_keys_change_only_the_external_pixels() {
    let left = WorkstationPresentation::with_illuminated_key(KeyId(17));
    let right = WorkstationPresentation::with_illuminated_key(KeyId(29));
    let body = truelearner_workstation::WorkstationState::default();
    let left_sample = WorkstationWorld::new_with_presentation(left)
        .unwrap()
        .sense(&body)
        .unwrap();
    let right_sample = WorkstationWorld::new_with_presentation(right)
        .unwrap()
        .sense(&body)
        .unwrap();

    for eye in Eye::ALL {
        assert_ne!(left_sample.eye(eye), right_sample.eye(eye));
    }
    for sample in [left_sample, right_sample] {
        let wire = serde_json::to_string(&sample).unwrap();
        for forbidden in ["key", "target", "coordinate", "direction", "score"] {
            assert!(!wire.contains(forbidden), "leaked {forbidden}: {wire}");
        }
    }
}

#[test]
fn illuminated_key_presentation_restores_the_exact_next_step() {
    let presentation = WorkstationPresentation::with_illuminated_key(KeyId(29));
    let mut session = WorkstationSession::new_with_presentation(71_005, presentation).unwrap();
    session.step().unwrap();
    let checkpoint = session.save().unwrap();
    let mut restored = WorkstationSession::restore(checkpoint).unwrap();

    assert_eq!(restored.read().unwrap(), session.read().unwrap());
    assert_eq!(restored.step().unwrap(), session.step().unwrap());
}

#[test]
fn invalid_illuminated_key_fails_closed() {
    let presentation = WorkstationPresentation::with_illuminated_key(KeyId(KEY_COUNT as u16));
    assert_eq!(
        WorkstationWorld::new_with_presentation(presentation).unwrap_err(),
        academy_workstation::WorldError::InvalidPresentation
    );
}

#[test]
fn monitor_cue_changes_only_pixels_and_preserves_the_body_until_the_next_step() {
    let mut blank = WorkstationSession::new(71_007).unwrap();
    let before = blank.read().unwrap();
    let blank_sample = WorkstationWorld::new()
        .unwrap()
        .sense(&before.body.state)
        .unwrap();

    blank
        .set_presentation(WorkstationPresentation::with_monitor_glyph('1'))
        .unwrap();
    let after = blank.read().unwrap();
    let cue_sample =
        WorkstationWorld::new_with_presentation(WorkstationPresentation::with_monitor_glyph('1'))
            .unwrap()
            .sense(&after.body.state)
            .unwrap();

    assert_eq!(after.sequence, before.sequence);
    assert_eq!(after.body, before.body);
    assert_ne!(after.world_fingerprint, before.world_fingerprint);
    assert_ne!(after.session_fingerprint, before.session_fingerprint);
    for eye in Eye::ALL {
        assert_ne!(cue_sample.eye(eye), blank_sample.eye(eye));
    }
    let wire = serde_json::to_string(&cue_sample).unwrap();
    for forbidden in ["glyph", "character", "key", "target", "answer", "score"] {
        assert!(!wire.contains(forbidden), "leaked {forbidden}: {wire}");
    }
}

#[test]
fn monitor_cue_update_is_transactional_and_restores_the_exact_next_step() {
    let mut session = WorkstationSession::new(71_008).unwrap();
    session
        .set_presentation(WorkstationPresentation::with_monitor_glyph('9'))
        .unwrap();
    let checkpoint = session.save().unwrap();
    let mut restored = WorkstationSession::restore(checkpoint).unwrap();

    assert_eq!(restored.read().unwrap(), session.read().unwrap());
    assert_eq!(restored.step().unwrap(), session.step().unwrap());

    let before = restored.read().unwrap();
    assert_eq!(
        restored
            .set_presentation(WorkstationPresentation::with_monitor_glyph('\n'))
            .unwrap_err(),
        academy_workstation::WorldError::InvalidPresentation
    );
    assert_eq!(restored.read().unwrap(), before);
}

#[test]
fn absent_monitor_cue_is_the_exact_default_identity() {
    let body = truelearner_workstation::WorkstationState::default();
    let default_world = WorkstationWorld::new().unwrap();
    let explicit_world =
        WorkstationWorld::new_with_presentation(WorkstationPresentation::default()).unwrap();
    assert_eq!(default_world, explicit_world);
    assert_eq!(
        default_world.sense(&body).unwrap(),
        explicit_world.sense(&body).unwrap()
    );
}

#[test]
fn application_raster_reaches_the_monitor_and_restores_exactly() {
    let mut pixels = vec![0; 64 * 64];
    pixels[32 * 64 + 32] = 255;
    let presentation =
        WorkstationPresentation::with_monitor_frame(MonitorFrame::new(64, 64, pixels).unwrap());
    let mut session = WorkstationSession::new_with_presentation(71_009, presentation).unwrap();
    let checkpoint = session.save().unwrap();
    let mut restored = WorkstationSession::restore(checkpoint).unwrap();

    assert_eq!(restored.read().unwrap(), session.read().unwrap());
    assert_eq!(restored.step().unwrap(), session.step().unwrap());
}

#[test]
fn invalid_application_raster_fails_before_a_world_exists() {
    assert_eq!(
        MonitorFrame::new(64, 64, vec![0; 63 * 64]).unwrap_err(),
        academy_workstation::WorldError::InvalidPresentation
    );
}

#[test]
fn session_can_attach_a_new_world_to_an_existing_body_checkpoint() {
    let harness = WorkstationHarness::new(71_010).unwrap();
    let body_before = harness.read().unwrap();
    let checkpoint = harness.save().unwrap();
    let session =
        WorkstationSession::from_body_checkpoint(checkpoint, WorkstationPresentation::default())
            .unwrap();

    assert_eq!(session.read().unwrap().body, body_before);
}
