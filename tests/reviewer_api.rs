use organism_v0::review::{
    simulate_effect, Frame, RelationalTopology, RepresentationLearner, StructuralEffect,
    UnifiedLearner,
};

fn reviewer_ring() -> RelationalTopology {
    RelationalTopology::from_neighbors(vec![
        vec![Some(4), Some(1)],
        vec![Some(0), Some(2)],
        vec![Some(1), Some(3)],
        vec![Some(2), Some(4)],
        vec![Some(3), Some(0)],
    ])
    .unwrap()
}

#[test]
fn reviewer_can_train_on_private_transitions_and_hold_out_a_pattern() {
    let topology = reviewer_ring();
    let mut learner = RepresentationLearner::new([201, 203], &topology);

    let training = Frame::singleton(0);
    let moved = simulate_effect(&topology, &training, StructuralEffect::FollowPort(1));
    let stayed = simulate_effect(&topology, &training, StructuralEffect::Stay);
    assert!(learner.observe(&topology, &training, 201, &moved));
    assert!(learner.observe(&topology, &training, 203, &stayed));

    let held_out = Frame::from_sensors([1, 2]);
    let expected = Frame::from_sensors([2, 3]);
    assert_eq!(learner.predict(&topology, &held_out, 201), Some(expected));
    assert_eq!(learner.predict(&topology, &held_out, 203), Some(held_out));
}

#[test]
fn reviewer_can_expose_an_unsupported_or_contradictory_action() {
    let topology = reviewer_ring();
    let mut learner = RepresentationLearner::new([211], &topology);
    let before = Frame::singleton(0);
    let clockwise = simulate_effect(&topology, &before, StructuralEffect::FollowPort(1));
    let counterclockwise = simulate_effect(&topology, &before, StructuralEffect::FollowPort(0));

    assert!(learner.observe(&topology, &before, 211, &clockwise));
    assert!(learner.observe(&topology, &before, 211, &counterclockwise));
    assert_eq!(learner.candidate_count(211), 0);
    assert_eq!(learner.predict(&topology, &before, 211), None);
    assert!(!learner.observe(&topology, &before, 255, &clockwise));
}

#[test]
fn reviewer_can_probe_the_unified_learner_with_hidden_tokens() {
    let mut learner = UnifiedLearner::new(256, 6, 512);
    let hidden_pairs = [(17, 203), (41, 199), (89, 197), (113, 193)];

    learner.reset_activity();
    for &(key, value) in &hidden_pairs {
        learner.absorb(250);
        learner.absorb(key);
        learner.absorb(value);
    }

    for (key, value) in hidden_pairs {
        learner.reset_activity();
        learner.absorb(key);
        assert_eq!(learner.answer(), Some(value));
    }
}
