use academy_workstation::{SessionCheckpoint, WorkstationSession, KEY_COUNT};
use truelearner_workstation::Eye;
#[cfg(feature = "research")]
use truelearner_workstation::{
    BodyAxis, Protocol, ResearchHarnessConfig, ResearchOpportunityIncidence,
    ResearchTransitionOpportunity,
};

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

#[cfg(feature = "research")]
#[test]
fn shared_opportunity_wave_exposes_separate_full_morphology_movements() {
    let config = ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::GenericOnly,
    };
    let mut default_session = WorkstationSession::new(82_001).unwrap();
    let mut session = WorkstationSession::new_research(82_001, config).unwrap();
    assert_eq!(
        default_session.save().unwrap().canonical_bytes().unwrap(),
        session.save().unwrap().canonical_bytes().unwrap()
    );
    let mut isolated_finger_steps = 0_u64;
    let mut five_finger_steps = 0_u64;
    let mut moved_digits = Vec::new();
    for _ in 0..48 {
        let default_observation = default_session.step().unwrap();
        let observation = session.step().unwrap();
        assert!(
            default_observation == observation,
            "default behavior diverged from the authorized shared behavior at sequence {}",
            observation.sequence
        );
        let changed_fingers = observation
            .body
            .movements
            .iter()
            .filter_map(|movement| match movement.axis {
                BodyAxis::FingerFlexion { digit } if movement.changed => Some(digit),
                _ => None,
            })
            .collect::<Vec<_>>();
        if changed_fingers.len() == 1 {
            isolated_finger_steps += 1;
            if !moved_digits.contains(&changed_fingers[0]) {
                moved_digits.push(changed_fingers[0]);
            }
        }
        if changed_fingers.len() == 5 {
            five_finger_steps += 1;
        }
        assert!(observation.body.naturally_quiescent);
    }
    assert!(isolated_finger_steps > 0);
    assert!(moved_digits.len() >= 2, "moved digits: {moved_digits:?}");
    assert_eq!(five_finger_steps, 0);
    assert_eq!(default_session.save().unwrap(), session.save().unwrap());
}
