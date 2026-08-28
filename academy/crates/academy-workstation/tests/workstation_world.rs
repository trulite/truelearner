use academy_workstation::{SessionCheckpoint, WorkstationSession, KEY_COUNT};
use truelearner_workstation::Eye;

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
