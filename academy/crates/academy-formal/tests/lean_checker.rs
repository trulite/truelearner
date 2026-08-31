use academy_formal::{
    project_ambiguous_return, project_closed_return, CausalCheckRequest, CausalClaim, CausalEvent,
    ClosureWitness, LeanChecker,
};
use truelearner_workstation::{
    BodyJunctionId, BodyLinkId, BodyPath, BodyPhysicalEvent, BodyReturnCandidateTrace,
    BodyReturnDecision, BodyReturnTrace, BodyRun, BodyTraceEvent, ContactSample, LightField,
    WorkstationHarness, WorldSample, TOUCH_SITES,
};

fn checker() -> Option<LeanChecker> {
    std::env::var_os("TRUELEARNER_LEAN_CHECKER").map(LeanChecker::new)
}

fn physical_sample() -> WorldSample {
    let light = |value| {
        let width = 33_u16;
        let height = 33_u16;
        let mut pixels = vec![0_u8; usize::from(width) * usize::from(height)];
        pixels[usize::from(height / 2) * usize::from(width) + usize::from(width / 2)] = value;
        LightField::new(width, height, pixels).unwrap()
    };
    WorldSample::new(
        [light(255), light(192)],
        [ContactSample::default(); TOUCH_SITES],
    )
    .unwrap()
}

fn frozen_trace_with_closed_return() -> (Vec<BodyTraceEvent>, usize) {
    let mut harness = WorkstationHarness::new(91).unwrap();
    let mut frozen = Vec::new();
    for _ in 0..8 {
        let (_, step) = harness.step_traced(physical_sample()).unwrap();
        let return_in_step = step.iter().position(|event| {
            matches!(
                event,
                BodyTraceEvent::Return(returned)
                    if returned.decision == BodyReturnDecision::Accepted
            )
        });
        let offset = frozen.len();
        frozen.extend(step);
        if let Some(return_in_step) = return_in_step {
            return (frozen, offset + return_in_step);
        }
    }
    panic!("workstation episode produced no accepted return: {frozen:#?}");
}

fn body_junction(id: u32) -> BodyJunctionId {
    serde_json::from_value(serde_json::json!(id)).unwrap()
}

fn body_link(id: u32) -> BodyLinkId {
    serde_json::from_value(serde_json::json!(id)).unwrap()
}

fn retained_ambiguous_trace_shape() -> (Vec<BodyTraceEvent>, usize) {
    let first = BodyPath {
        surface: body_junction(3),
        middle: body_junction(8),
        output: body_junction(1),
        first: body_link(7),
        second: body_link(8),
    };
    let second = BodyPath {
        surface: body_junction(6),
        middle: body_junction(10),
        output: body_junction(4),
        first: body_link(12),
        second: body_link(13),
    };
    let events = vec![
        BodyTraceEvent::Transition(BodyPhysicalEvent {
            at: 12,
            junction: first.output,
            arrivals: 1,
            impulse: 1,
            before: 0,
            after: 1,
            cause: 7,
        }),
        BodyTraceEvent::Transition(BodyPhysicalEvent {
            at: 22,
            junction: second.output,
            arrivals: 1,
            impulse: 1,
            before: 0,
            after: 1,
            cause: 7,
        }),
        BodyTraceEvent::Return(BodyReturnTrace {
            at: 22,
            source: body_junction(7),
            incoming_cause: 7,
            path: None,
            return_cause: None,
            return_opened_at: None,
            open_paths: 2,
            exact_paths: 2,
            candidates: vec![
                BodyReturnCandidateTrace {
                    path: first,
                    cause: 7,
                    opened_at: 12,
                },
                BodyReturnCandidateTrace {
                    path: second,
                    cause: 7,
                    opened_at: 22,
                },
            ],
            decision: BodyReturnDecision::Ambiguous,
        }),
        BodyTraceEvent::Quiet(BodyRun::default()),
    ];
    (events, 2)
}

fn closed_request(claimed_witness: Option<u64>) -> CausalCheckRequest {
    CausalCheckRequest::new(
        vec![
            CausalEvent {
                id: 1,
                time: 10,
                parents: Vec::new(),
            },
            CausalEvent {
                id: 2,
                time: 12,
                parents: vec![1],
            },
        ],
        vec![ClosureWitness {
            id: 7,
            crossing: 1,
            support: vec![11, 12],
            opened_at: 10,
            expires_at: 20,
        }],
        2,
        CausalClaim {
            resolution: "closed".to_string(),
            witness: claimed_witness,
        },
    )
}

fn ambiguous_request() -> CausalCheckRequest {
    CausalCheckRequest::new(
        vec![
            CausalEvent {
                id: 1,
                time: 10,
                parents: Vec::new(),
            },
            CausalEvent {
                id: 2,
                time: 10,
                parents: Vec::new(),
            },
            CausalEvent {
                id: 3,
                time: 12,
                parents: vec![1, 2],
            },
        ],
        vec![
            ClosureWitness {
                id: 7,
                crossing: 1,
                support: vec![11, 12],
                opened_at: 10,
                expires_at: 20,
            },
            ClosureWitness {
                id: 8,
                crossing: 2,
                support: vec![21, 22],
                opened_at: 10,
                expires_at: 20,
            },
        ],
        3,
        CausalClaim {
            resolution: "ambiguous".to_string(),
            witness: None,
        },
    )
}

#[test]
fn rust_calls_lean_and_accepts_an_entailed_claim() {
    let Some(checker) = checker() else {
        return;
    };
    let receipt = checker.check(&closed_request(Some(7))).unwrap();

    assert!(receipt.accepted);
    assert_eq!(receipt.resolution, "closed");
    assert_eq!(receipt.witness, Some(7));
    assert_eq!(receipt.explanations, [7]);
    assert_eq!(receipt.persistent_links, [11, 12]);
}

#[test]
fn rust_preserves_a_lean_falsification_as_evidence() {
    let Some(checker) = checker() else {
        return;
    };
    let receipt = checker.check(&closed_request(Some(8))).unwrap();

    assert!(!receipt.accepted);
    assert_eq!(receipt.resolution, "closed");
    assert_eq!(receipt.witness, Some(7));
    assert_eq!(receipt.persistent_links, [11, 12]);
}

#[test]
fn ambiguous_return_persists_nothing() {
    let Some(checker) = checker() else {
        return;
    };
    let receipt = checker.check(&ambiguous_request()).unwrap();

    assert!(receipt.accepted);
    assert_eq!(receipt.resolution, "ambiguous");
    assert_eq!(receipt.witness, None);
    assert_eq!(receipt.explanations, [7, 8]);
    assert!(receipt.persistent_links.is_empty());
}

#[test]
fn retained_ambiguous_paths_project_without_persistence() {
    let (trace, returned) = retained_ambiguous_trace_shape();
    let projection = project_ambiguous_return(&trace, returned).unwrap();

    assert_eq!(projection.request.events.len(), 3);
    assert_eq!(projection.request.witnesses.len(), 2);
    assert_eq!(projection.request.events[2].parents, [1, 2]);
    assert_eq!(projection.physical_support.len(), 4);
    assert_eq!(projection.request.claim.resolution, "ambiguous");
    assert_eq!(projection.request.claim.witness, None);
}

#[test]
fn retained_ambiguous_paths_are_accepted_by_lean() {
    let Some(checker) = checker() else {
        return;
    };
    let (trace, returned) = retained_ambiguous_trace_shape();
    let projection = project_ambiguous_return(&trace, returned).unwrap();
    let receipt = checker.check(&projection.request).unwrap();

    assert!(receipt.accepted);
    assert_eq!(receipt.resolution, "ambiguous");
    assert_eq!(receipt.explanations, [1, 2]);
    assert!(receipt.persistent_links.is_empty());
}

#[test]
fn timing_without_ancestry_makes_no_closure_claim() {
    let Some(checker) = checker() else {
        return;
    };
    let mut request = closed_request(None);
    request.events[1].parents.clear();
    request.claim.resolution = "no_claim".to_string();

    let receipt = checker.check(&request).unwrap();

    assert!(receipt.accepted);
    assert_eq!(receipt.resolution, "no_claim");
    assert!(receipt.explanations.is_empty());
    assert!(receipt.persistent_links.is_empty());
}

#[test]
fn actual_workstation_trace_projects_one_complete_closed_cycle() {
    let (trace, returned) = frozen_trace_with_closed_return();
    let projection = project_closed_return(&trace, returned).unwrap();

    assert_eq!(projection.request.events.len(), 2);
    assert_eq!(projection.request.events[1].parents, [1]);
    assert_eq!(projection.request.witnesses[0].support, [1, 2]);
    assert_ne!(
        projection.physical_support[0].1,
        projection.physical_support[1].1
    );
}

#[test]
fn actual_workstation_trace_is_accepted_by_lean() {
    let Some(checker) = checker() else {
        return;
    };
    let (trace, returned) = frozen_trace_with_closed_return();
    let projection = project_closed_return(&trace, returned).unwrap();
    let receipt = checker.check(&projection.request).unwrap();

    assert!(receipt.accepted);
    assert_eq!(receipt.resolution, "closed");
    assert_eq!(receipt.persistent_links, [1, 2]);
}

#[test]
fn projection_rejects_a_trace_missing_the_output_crossing() {
    let (mut trace, returned_index) = frozen_trace_with_closed_return();
    let returned = match &trace[returned_index] {
        BodyTraceEvent::Return(returned) => returned,
        _ => unreachable!("fixture selected a return"),
    };
    let path = returned.path.unwrap();
    let cause = returned.return_cause.unwrap();
    let opened_at = returned.return_opened_at.unwrap();
    let crossing = trace[..returned_index]
        .iter()
        .position(|event| {
            matches!(
                event,
                BodyTraceEvent::Transition(transition)
                    if transition.at == opened_at
                        && transition.junction == path.output
                        && transition.cause == cause
            )
        })
        .unwrap();
    trace.remove(crossing);

    assert!(matches!(
        project_closed_return(&trace, returned_index - 1),
        Err(academy_formal::TraceProjectionError::OutputCrossingCount { observed: 0, .. })
    ));
}

#[test]
fn projection_makes_no_claim_for_an_ambiguous_return() {
    let (mut trace, returned_index) = frozen_trace_with_closed_return();
    let BodyTraceEvent::Return(returned) = &mut trace[returned_index] else {
        unreachable!("fixture selected a return")
    };
    returned.decision = BodyReturnDecision::Ambiguous;

    assert!(matches!(
        project_closed_return(&trace, returned_index),
        Err(academy_formal::TraceProjectionError::ReturnNotClosed {
            decision: BodyReturnDecision::Ambiguous,
            ..
        })
    ));
}

#[test]
fn frozen_closure_projection_replays_exactly() {
    let (left_trace, left_return) = frozen_trace_with_closed_return();
    let (right_trace, right_return) = frozen_trace_with_closed_return();

    assert_eq!(left_return, right_return);
    assert_eq!(left_trace, right_trace);
    assert_eq!(
        project_closed_return(&left_trace, left_return).unwrap(),
        project_closed_return(&right_trace, right_return).unwrap()
    );
}
