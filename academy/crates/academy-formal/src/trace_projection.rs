use crate::{CausalCheckRequest, CausalClaim, CausalEvent, ClosureWitness};
use std::{collections::BTreeMap, error::Error, fmt};
use truelearner_workstation::{
    verify_choice_laws, BodyLinkId, BodyReturnDecision, BodyReturnTrace, BodyTraceEvent,
};

const CROSSING_EVENT_ID: u64 = 1;
const RETURN_EVENT_ID: u64 = 2;
const WITNESS_ID: u64 = 1;
const FIRST_SUPPORT_ID: u64 = 1;
const SECOND_SUPPORT_ID: u64 = 2;

/// A formal request together with the exact physical links represented by its
/// local support identifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClosureProjection {
    pub request: CausalCheckRequest,
    pub return_event_index: usize,
    pub physical_support: Vec<(u64, BodyLinkId)>,
}

/// Projects one accepted return from an already-frozen body trace.
///
/// The projection is deliberately strict. It first verifies all recorded
/// choices, then requires the output transition, accepted return naming the
/// resolved physical path, and both strengthening events to be present. Timing
/// without these arrows is rejected.
pub fn project_closed_return(
    events: &[BodyTraceEvent],
    return_event_index: usize,
) -> Result<ClosureProjection, TraceProjectionError> {
    let returned = validated_return(events, return_event_index)?;
    if returned.decision != BodyReturnDecision::Accepted {
        return Err(TraceProjectionError::ReturnNotClosed {
            event: return_event_index,
            decision: returned.decision,
            open_paths: returned.open_paths,
            exact_paths: returned.exact_paths,
        });
    }
    let path = returned
        .path
        .ok_or(TraceProjectionError::AcceptedReturnMissingPath(
            return_event_index,
        ))?;
    let cause = returned
        .return_cause
        .ok_or(TraceProjectionError::AcceptedReturnMissingCause(
            return_event_index,
        ))?;
    let opened_at =
        returned
            .return_opened_at
            .ok_or(TraceProjectionError::AcceptedReturnMissingOpening(
                return_event_index,
            ))?;

    let crossings = events[..return_event_index]
        .iter()
        .filter(|event| {
            matches!(
                event,
                BodyTraceEvent::Transition(transition)
                    if transition.at == opened_at
                        && transition.junction == path.output
                        && transition.cause == cause
            )
        })
        .count();
    if crossings != 1 {
        return Err(TraceProjectionError::OutputCrossingCount {
            event: return_event_index,
            observed: crossings,
        });
    }

    for link in [path.first, path.second] {
        let strengthened = events[return_event_index + 1..]
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    BodyTraceEvent::Strengthened(strength)
                        if strength.at == returned.at
                            && strength.link == link
                            && strength.after == strength.before.saturating_add(1)
                )
            })
            .count();
        if strengthened != 1 {
            return Err(TraceProjectionError::StrengtheningCount {
                event: return_event_index,
                link,
                observed: strengthened,
            });
        }
    }

    Ok(ClosureProjection {
        request: CausalCheckRequest::new(
            vec![
                CausalEvent {
                    id: CROSSING_EVENT_ID,
                    time: opened_at,
                    parents: Vec::new(),
                },
                CausalEvent {
                    id: RETURN_EVENT_ID,
                    time: returned.at,
                    parents: vec![CROSSING_EVENT_ID],
                },
            ],
            vec![ClosureWitness {
                id: WITNESS_ID,
                crossing: CROSSING_EVENT_ID,
                support: vec![FIRST_SUPPORT_ID, SECOND_SUPPORT_ID],
                opened_at,
                expires_at: returned.at,
            }],
            RETURN_EVENT_ID,
            CausalClaim {
                resolution: "closed".to_string(),
                witness: Some(WITNESS_ID),
            },
        ),
        return_event_index,
        physical_support: vec![
            (FIRST_SUPPORT_ID, path.first),
            (SECOND_SUPPORT_ID, path.second),
        ],
    })
}

/// Projects every causally eligible contender of one ambiguous return.
///
/// A same-tick contender is admitted only when its output transition occurs
/// earlier in the trace. Candidate counts without the paths themselves are not
/// enough to construct the request.
pub fn project_ambiguous_return(
    events: &[BodyTraceEvent],
    return_event_index: usize,
) -> Result<ClosureProjection, TraceProjectionError> {
    let returned = validated_return(events, return_event_index)?;
    if returned.decision != BodyReturnDecision::Ambiguous {
        return Err(TraceProjectionError::ReturnNotAmbiguous {
            event: return_event_index,
            decision: returned.decision,
        });
    }
    if returned.candidates.len() != returned.open_paths {
        return Err(TraceProjectionError::CandidateCount {
            event: return_event_index,
            recorded: returned.open_paths,
            retained: returned.candidates.len(),
        });
    }
    let candidates = returned
        .candidates
        .iter()
        .filter(|candidate| candidate.opened_at <= returned.at)
        .collect::<Vec<_>>();
    if candidates.len() < 2 {
        return Err(TraceProjectionError::InsufficientCausalCandidates {
            event: return_event_index,
            retained: candidates.len(),
        });
    }

    for (index, candidate) in candidates.iter().enumerate() {
        if candidates[..index]
            .iter()
            .any(|earlier| earlier.path == candidate.path)
        {
            return Err(TraceProjectionError::DuplicateCandidatePath {
                event: return_event_index,
                candidate: index,
            });
        }
        let crossings = events[..return_event_index]
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    BodyTraceEvent::Transition(transition)
                        if transition.at == candidate.opened_at
                            && transition.junction == candidate.path.output
                            && transition.cause == candidate.cause
                )
            })
            .count();
        if crossings != 1 {
            return Err(TraceProjectionError::CandidateCrossingCount {
                event: return_event_index,
                candidate: index,
                observed: crossings,
            });
        }
        let strengthening = events[return_event_index + 1..]
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    BodyTraceEvent::Strengthened(strength)
                        if strength.at == returned.at
                            && [candidate.path.first, candidate.path.second]
                                .contains(&strength.link)
                )
            })
            .count();
        if strengthening != 0 {
            return Err(TraceProjectionError::AmbiguousReturnStrengthened {
                event: return_event_index,
                candidate: index,
                observed: strengthening,
            });
        }
    }

    let mut formal_links = BTreeMap::new();
    let mut physical_support = Vec::new();
    let mut support_id = |link: BodyLinkId| {
        if let Some(id) = formal_links.get(&link) {
            return *id;
        }
        let id = u64::try_from(physical_support.len() + 1)
            .expect("bounded trace support fits in a formal identifier");
        formal_links.insert(link, id);
        physical_support.push((id, link));
        id
    };
    let candidate_count = u64::try_from(candidates.len())
        .map_err(|_| TraceProjectionError::TooManyCandidates(return_event_index))?;
    let returned_id = candidate_count + 1;
    let mut causal_events = Vec::with_capacity(candidates.len() + 1);
    let mut witnesses = Vec::with_capacity(candidates.len());
    for (index, candidate) in candidates.iter().enumerate() {
        let witness_id = u64::try_from(index + 1)
            .map_err(|_| TraceProjectionError::TooManyCandidates(return_event_index))?;
        causal_events.push(CausalEvent {
            id: witness_id,
            time: candidate.opened_at,
            parents: Vec::new(),
        });
        witnesses.push(ClosureWitness {
            id: witness_id,
            crossing: witness_id,
            support: vec![
                support_id(candidate.path.first),
                support_id(candidate.path.second),
            ],
            opened_at: candidate.opened_at,
            expires_at: returned.at,
        });
    }
    causal_events.push(CausalEvent {
        id: returned_id,
        time: returned.at,
        parents: (1..=candidate_count).collect(),
    });

    Ok(ClosureProjection {
        request: CausalCheckRequest::new(
            causal_events,
            witnesses,
            returned_id,
            CausalClaim {
                resolution: "ambiguous".to_string(),
                witness: None,
            },
        ),
        return_event_index,
        physical_support,
    })
}

fn validated_return(
    events: &[BodyTraceEvent],
    return_event_index: usize,
) -> Result<&BodyReturnTrace, TraceProjectionError> {
    if !matches!(events.last(), Some(BodyTraceEvent::Quiet(_))) {
        return Err(TraceProjectionError::NotNaturallyQuiet);
    }
    verify_choice_laws(events)
        .map_err(|error| TraceProjectionError::ChoiceLaw(error.to_string()))?;
    match events.get(return_event_index) {
        Some(BodyTraceEvent::Return(returned)) => Ok(returned),
        Some(_) => Err(TraceProjectionError::NotReturnEvent(return_event_index)),
        None => Err(TraceProjectionError::EventOutsideTrace(return_event_index)),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraceProjectionError {
    NotNaturallyQuiet,
    ChoiceLaw(String),
    EventOutsideTrace(usize),
    NotReturnEvent(usize),
    ReturnNotClosed {
        event: usize,
        decision: BodyReturnDecision,
        open_paths: usize,
        exact_paths: usize,
    },
    ReturnNotAmbiguous {
        event: usize,
        decision: BodyReturnDecision,
    },
    CandidateCount {
        event: usize,
        recorded: usize,
        retained: usize,
    },
    InsufficientCausalCandidates {
        event: usize,
        retained: usize,
    },
    DuplicateCandidatePath {
        event: usize,
        candidate: usize,
    },
    CandidateCrossingCount {
        event: usize,
        candidate: usize,
        observed: usize,
    },
    AmbiguousReturnStrengthened {
        event: usize,
        candidate: usize,
        observed: usize,
    },
    TooManyCandidates(usize),
    AcceptedReturnMissingPath(usize),
    AcceptedReturnMissingCause(usize),
    AcceptedReturnMissingOpening(usize),
    OutputCrossingCount {
        event: usize,
        observed: usize,
    },
    StrengtheningCount {
        event: usize,
        link: BodyLinkId,
        observed: usize,
    },
}

impl fmt::Display for TraceProjectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotNaturallyQuiet => formatter.write_str("frozen trace does not end at natural quiet"),
            Self::ChoiceLaw(message) => write!(formatter, "choice trace is invalid: {message}"),
            Self::EventOutsideTrace(event) => write!(formatter, "event {event} is outside the frozen trace"),
            Self::NotReturnEvent(event) => write!(formatter, "event {event} is not a return"),
            Self::ReturnNotClosed {
                event,
                decision,
                open_paths,
                exact_paths,
            } => write!(
                formatter,
                "return {event} is {decision:?}, with {open_paths} open and {exact_paths} exact paths"
            ),
            Self::ReturnNotAmbiguous { event, decision } => {
                write!(formatter, "return {event} is {decision:?}, not ambiguous")
            }
            Self::CandidateCount {
                event,
                recorded,
                retained,
            } => write!(
                formatter,
                "return {event} records {recorded} candidates but retains {retained}"
            ),
            Self::InsufficientCausalCandidates { event, retained } => write!(
                formatter,
                "return {event} retains only {retained} causally eligible candidates"
            ),
            Self::DuplicateCandidatePath { event, candidate } => write!(
                formatter,
                "return {event} repeats the physical path at candidate {candidate}"
            ),
            Self::CandidateCrossingCount {
                event,
                candidate,
                observed,
            } => write!(
                formatter,
                "return {event} candidate {candidate} has {observed} matching output transitions instead of one"
            ),
            Self::AmbiguousReturnStrengthened {
                event,
                candidate,
                observed,
            } => write!(
                formatter,
                "ambiguous return {event} candidate {candidate} has {observed} strengthening events"
            ),
            Self::TooManyCandidates(event) => {
                write!(formatter, "return {event} has too many candidates to identify")
            }
            Self::AcceptedReturnMissingPath(event) => {
                write!(formatter, "accepted return {event} names no path")
            }
            Self::AcceptedReturnMissingCause(event) => {
                write!(formatter, "accepted return {event} names no cause")
            }
            Self::AcceptedReturnMissingOpening(event) => {
                write!(formatter, "accepted return {event} names no opening")
            }
            Self::OutputCrossingCount { event, observed } => write!(
                formatter,
                "return {event} has {observed} matching output transitions instead of one"
            ),
            Self::StrengtheningCount {
                event,
                link,
                observed,
            } => write!(
                formatter,
                "return {event} has {observed} strengthening events for physical link {link:?} instead of one"
            ),
        }
    }
}

impl Error for TraceProjectionError {}
