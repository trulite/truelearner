use super::*;

fn path(index: usize) -> TracePath {
    TracePath {
        surface: JunctionId::new(index * 3).unwrap(),
        middle: JunctionId::new(index * 3 + 1).unwrap().into(),
        output: JunctionId::new(index * 3 + 2).unwrap(),
        first: LinkId::new(index * 2).unwrap().into(),
        second: LinkId::new(index * 2 + 1).unwrap().into(),
    }
}

fn candidate(index: usize, drive: u16) -> CandidateTrace {
    CandidateTrace {
        at: 7,
        cause: 11,
        group: 0,
        path: path(index),
        connected_outcomes: Vec::new(),
        executable: true,
        return_cause: None,
        unanswered: false,
        outcome: None,
        participation: 0,
        participated_at: 0,
        output_participated: false,
        outcome_source: None,
        progress_source: None,
        resisted_progress: false,
        boundary_open: false,
        boundary_inhibited: false,
        strength: 1,
        drive,
        stable_order: index as u32,
        fresh_opportunity: None,
        present_sources: Vec::new(),
        reentries: Vec::new(),
        motif_reentries: Vec::new(),
        motif_routes: Vec::new(),
        reentry_incidence_visits: 0,
        reentry_shortcut_hits: 0,
        reentry_failed: false,
        motif_route_failed: false,
        new_path: false,
    }
}

#[test]
fn offline_verifier_accepts_a_valid_choice() {
    let weak = candidate(0, 44);
    let strong = candidate(1, 1_023);
    let events = [
        TraceEvent::Candidate(weak),
        TraceEvent::Candidate(strong.clone()),
        TraceEvent::Choice(ChoiceTrace {
            at: 7,
            group: 0,
            alternatives: 2,
            winner: Some(strong.path),
            basis: Some(ChoiceBasis::ParticipationStrengthAndDrive),
            construction: false,
            sent: true,
        }),
    ];

    assert_eq!(verify_choice_laws(&events), Ok(()));
}

#[test]
fn offline_verifier_checks_unique_reentry_receipt_shape() {
    let condition = JunctionId::new(30).unwrap();
    let mut reaching = candidate(0, 512);
    let (JunctionRef::Existing(middle), LinkRef::Existing(first), LinkRef::Existing(second)) = (
        reaching.path.middle,
        reaching.path.first,
        reaching.path.second,
    ) else {
        unreachable!()
    };
    reaching.present_sources.push(condition);
    reaching.reentries.push(ReentryTrace {
        condition,
        steps: vec![ReentryStepTrace {
            path: Path {
                surface: reaching.path.surface,
                middle,
                output: reaching.path.output,
                first,
                second,
            },
            returned_source: condition,
            outcome_witness: LinkId::new(40).unwrap(),
            outcome_target: reaching.path.output,
        }],
    });
    reaching.reentry_incidence_visits = 1;
    reaching.outcome = Some(Outcome {
        at: 5,
        caused_transition: true,
        available_until_choice: true,
    });
    let stale = candidate(1, 512);
    let mut events = vec![
        TraceEvent::Candidate(reaching.clone()),
        TraceEvent::Candidate(stale),
        TraceEvent::Choice(ChoiceTrace {
            at: 7,
            group: 0,
            alternatives: 2,
            winner: Some(reaching.path),
            basis: Some(ChoiceBasis::UniqueReentry),
            construction: false,
            sent: true,
        }),
    ];

    verify_choice_laws(&events).unwrap();

    let TraceEvent::Candidate(candidate) = &mut events[0] else {
        unreachable!()
    };
    candidate.reentry_incidence_visits = 0;
    candidate.reentry_shortcut_hits = 1;
    verify_choice_laws(&events).unwrap();

    let TraceEvent::Candidate(candidate) = &mut events[0] else {
        unreachable!()
    };
    candidate.reentries[0].steps[0].outcome_target = JunctionId::new(99).unwrap();
    assert!(verify_choice_laws(&events).is_err());
}

#[test]
fn offline_verifier_checks_unique_motif_reentry_receipt_shape() {
    let mut reaching = candidate(0, 512);
    reaching.new_path = true;
    reaching.motif_reentries.push(MotifReentryTrace {
        witness: LinkId::new(40).unwrap(),
        parent: LinkId::new(41).unwrap(),
    });
    reaching.reentry_incidence_visits = 1;
    let stale = candidate(1, 512);
    let mut events = vec![
        TraceEvent::Candidate(reaching.clone()),
        TraceEvent::Candidate(stale),
        TraceEvent::Choice(ChoiceTrace {
            at: 7,
            group: 0,
            alternatives: 2,
            winner: Some(reaching.path),
            basis: Some(ChoiceBasis::UniqueMotifReentry),
            construction: false,
            sent: true,
        }),
    ];

    verify_choice_laws(&events).unwrap();

    let TraceEvent::Candidate(candidate) = &mut events[0] else {
        unreachable!()
    };
    candidate.motif_reentries[0].parent = candidate.motif_reentries[0].witness;
    assert!(verify_choice_laws(&events).is_err());
}

#[test]
fn offline_verifier_checks_composed_motif_route_receipt_shape() {
    let intermediate = JunctionId::new(50).unwrap();
    let condition = JunctionId::new(60).unwrap();
    let mut reaching = candidate(0, 512);
    reaching.new_path = true;
    reaching.outcome_source = Some(intermediate);
    reaching.present_sources.push(condition);
    reaching.motif_reentries.push(MotifReentryTrace {
        witness: LinkId::new(40).unwrap(),
        parent: LinkId::new(41).unwrap(),
    });
    reaching.motif_routes.push(MotifRouteTrace {
        condition,
        steps: vec![MotifRouteStepTrace {
            surface: intermediate,
            output: JunctionId::new(51).unwrap(),
            through: LinkId::new(42).unwrap(),
            impulse: 1,
            outcome_source: condition,
            supports: vec![MotifReentryTrace {
                witness: LinkId::new(43).unwrap(),
                parent: LinkId::new(44).unwrap(),
            }],
        }],
    });
    reaching.reentry_incidence_visits = 1;
    let mut local_only = candidate(1, 512);
    local_only.new_path = true;
    local_only.motif_reentries.push(MotifReentryTrace {
        witness: LinkId::new(45).unwrap(),
        parent: LinkId::new(46).unwrap(),
    });
    local_only.reentry_incidence_visits = 1;
    let mut events = vec![
        TraceEvent::Candidate(reaching.clone()),
        TraceEvent::Candidate(local_only),
        TraceEvent::Choice(ChoiceTrace {
            at: 7,
            group: 0,
            alternatives: 2,
            winner: Some(reaching.path),
            basis: Some(ChoiceBasis::UniqueMotifReentry),
            construction: false,
            sent: true,
        }),
    ];

    verify_choice_laws(&events).unwrap();

    let TraceEvent::Candidate(candidate) = &mut events[0] else {
        unreachable!()
    };
    candidate.motif_routes[0].steps[0].surface = JunctionId::new(99).unwrap();
    assert!(verify_choice_laws(&events).is_err());
}

#[test]
fn offline_verifier_checks_unanswered_output_release() {
    let mut unanswered = candidate(0, 512);
    unanswered.at = 30;
    unanswered.participation = 2;
    unanswered.participated_at = 20;
    unanswered.unanswered = true;
    unanswered.strength = 3;
    unanswered.outcome = Some(Outcome {
        at: 10,
        caused_transition: true,
        available_until_choice: false,
    });
    let mut alternative = candidate(1, 512);
    alternative.at = 30;
    alternative.participation = 1;
    alternative.participated_at = 15;
    let mut events = vec![
        TraceEvent::Candidate(unanswered),
        TraceEvent::Candidate(alternative.clone()),
        TraceEvent::Choice(ChoiceTrace {
            at: 30,
            group: 0,
            alternatives: 2,
            winner: Some(alternative.path),
            basis: Some(ChoiceBasis::UnansweredOutputRelease),
            construction: false,
            sent: true,
        }),
    ];

    verify_choice_laws(&events).unwrap();

    let TraceEvent::Choice(choice) = events.last_mut().unwrap() else {
        unreachable!()
    };
    choice.basis = Some(ChoiceBasis::LatestOutcome);
    let failure = verify_choice_laws(&events).unwrap_err();
    assert_eq!(failure.law(), ChoiceLaw::UnansweredOutputRelease);
}

#[test]
fn offline_verifier_accepts_local_boundary_release() {
    let local = JunctionId::new(30).unwrap();
    let mut completed = candidate(0, 512);
    completed.outcome_source = Some(local);
    completed.boundary_inhibited = true;
    let mut antagonist = candidate(1, 1);
    antagonist.outcome_source = Some(local);
    let mut unrelated = candidate(2, 1_023);
    unrelated.outcome_source = Some(JunctionId::new(31).unwrap());
    let events = [
        TraceEvent::Candidate(completed),
        TraceEvent::Candidate(antagonist.clone()),
        TraceEvent::Candidate(unrelated),
        TraceEvent::Choice(ChoiceTrace {
            at: 7,
            group: 0,
            alternatives: 3,
            winner: Some(antagonist.path),
            basis: Some(ChoiceBasis::BoundaryRelease),
            construction: false,
            sent: true,
        }),
    ];

    verify_choice_laws(&events).unwrap();
}

#[test]
fn offline_verifier_names_the_old_surface_locality_failure() {
    let mut weak = candidate(0, 44);
    weak.outcome = Some(Outcome {
        at: 3,
        caused_transition: true,
        available_until_choice: true,
    });
    let strong = candidate(1, 1_023);
    let events = [
        TraceEvent::Candidate(weak.clone()),
        TraceEvent::Candidate(strong),
        TraceEvent::Choice(ChoiceTrace {
            at: 7,
            group: 0,
            alternatives: 2,
            winner: Some(weak.path),
            basis: Some(ChoiceBasis::AvailableOutcome),
            construction: false,
            sent: true,
        }),
    ];

    let failure = verify_choice_laws(&events).unwrap_err();
    assert_eq!(failure.law(), ChoiceLaw::CurrentSurfaceLocality);
    assert_eq!(
        failure.to_string(),
        "choice group 0 at 7 violated CurrentSurfaceLocality: winner drive 44, strongest current drive 1023"
    );
}

#[test]
fn exact_current_return_precedes_current_surface_locality() {
    let mut returning = candidate(0, 44);
    returning.return_cause = Some(returning.cause);
    let strong = candidate(1, 1_023);
    let events = [
        TraceEvent::Candidate(returning.clone()),
        TraceEvent::Candidate(strong),
        TraceEvent::Choice(ChoiceTrace {
            at: 7,
            group: 0,
            alternatives: 2,
            winner: Some(returning.path),
            basis: Some(ChoiceBasis::CurrentReturn),
            construction: false,
            sent: true,
        }),
    ];

    assert_eq!(verify_choice_laws(&events), Ok(()));
}
