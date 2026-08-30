use crate::{
    Cause, JunctionId, JunctionRef, LinkId, LinkRef, Outcome, Path, PhysicalEvent, Run, Time,
};
use std::{cmp::Reverse, error::Error, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraceArrival {
    pub at: Time,
    pub target: JunctionId,
    pub impulse: i32,
    pub cause: Cause,
    pub via: Option<LinkId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TracePath {
    pub surface: JunctionId,
    pub middle: JunctionRef,
    pub output: JunctionId,
    pub first: LinkRef,
    pub second: LinkRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateTrace {
    pub at: Time,
    pub cause: Cause,
    pub group: usize,
    pub path: TracePath,
    pub connected_outcomes: Vec<JunctionId>,
    pub executable: bool,
    pub return_cause: Option<Cause>,
    pub unanswered: bool,
    pub outcome: Option<Outcome>,
    pub participation: u64,
    pub strength: i64,
    pub drive: u16,
    pub stable_order: u32,
    pub new_path: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChoiceBasis {
    CurrentReturn,
    AvailableOutcome,
    LatestOutcome,
    UntriedOutputRelease,
    ParticipationStrengthAndDrive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChoiceTrace {
    pub at: Time,
    pub group: usize,
    pub alternatives: usize,
    pub winner: Option<TracePath>,
    pub basis: Option<ChoiceBasis>,
    pub construction: bool,
    pub sent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChoiceLaw {
    CandidateAccounting,
    Eligibility,
    CurrentReturn,
    CurrentSurfaceLocality,
    UntriedOutputRelease,
    AvailableOutcome,
    LatestOutcome,
    ParticipationStrengthAndDrive,
    Delivery,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChoiceLawViolation {
    MissingChoice {
        at: Time,
        group: usize,
    },
    CandidateCount {
        at: Time,
        group: usize,
        recorded: usize,
        observed: usize,
    },
    WinnerNotCandidate {
        at: Time,
        group: usize,
        winner: TracePath,
    },
    MissingWinner {
        at: Time,
        group: usize,
        law: ChoiceLaw,
        expected: TracePath,
    },
    UnexpectedWinner {
        at: Time,
        group: usize,
        winner: TracePath,
    },
    WrongWinner {
        at: Time,
        group: usize,
        law: ChoiceLaw,
        expected: TracePath,
        observed: TracePath,
    },
    WrongBasis {
        at: Time,
        group: usize,
        law: ChoiceLaw,
        expected: Option<ChoiceBasis>,
        observed: Option<ChoiceBasis>,
    },
    CurrentSurfaceLocality {
        at: Time,
        group: usize,
        winner: TracePath,
        winner_drive: u16,
        strongest_drive: u16,
    },
    Delivery {
        at: Time,
        group: usize,
        expected_sent: bool,
        observed_sent: bool,
    },
}

impl ChoiceLawViolation {
    pub const fn law(&self) -> ChoiceLaw {
        match self {
            Self::MissingChoice { .. }
            | Self::CandidateCount { .. }
            | Self::WinnerNotCandidate { .. } => ChoiceLaw::CandidateAccounting,
            Self::MissingWinner { law, .. }
            | Self::WrongWinner { law, .. }
            | Self::WrongBasis { law, .. } => *law,
            Self::UnexpectedWinner { .. } => ChoiceLaw::Eligibility,
            Self::CurrentSurfaceLocality { .. } => ChoiceLaw::CurrentSurfaceLocality,
            Self::Delivery { .. } => ChoiceLaw::Delivery,
        }
    }
}

impl fmt::Display for ChoiceLawViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingChoice { at, group } => {
                write!(formatter, "choice group {group} at {at} has candidates but no choice")
            }
            Self::CandidateCount {
                at,
                group,
                recorded,
                observed,
            } => write!(
                formatter,
                "choice group {group} at {at} records {recorded} alternatives but has {observed} candidates"
            ),
            Self::WinnerNotCandidate { at, group, .. } => write!(
                formatter,
                "choice group {group} at {at} selected a path that was not a candidate"
            ),
            Self::MissingWinner { at, group, law, .. } => write!(
                formatter,
                "choice group {group} at {at} violated {law:?}: the required candidate was not selected"
            ),
            Self::UnexpectedWinner { at, group, .. } => write!(
                formatter,
                "choice group {group} at {at} violated Eligibility: an ineligible candidate was selected"
            ),
            Self::WrongWinner { at, group, law, .. } => write!(
                formatter,
                "choice group {group} at {at} violated {law:?}: it selected the wrong candidate"
            ),
            Self::WrongBasis {
                at,
                group,
                law,
                expected,
                observed,
            } => write!(
                formatter,
                "choice group {group} at {at} violated {law:?}: expected basis {expected:?}, observed {observed:?}"
            ),
            Self::CurrentSurfaceLocality {
                at,
                group,
                winner_drive,
                strongest_drive,
                ..
            } => write!(
                formatter,
                "choice group {group} at {at} violated CurrentSurfaceLocality: winner drive {winner_drive}, strongest current drive {strongest_drive}"
            ),
            Self::Delivery {
                at,
                group,
                expected_sent,
                observed_sent,
            } => write!(
                formatter,
                "choice group {group} at {at} violated Delivery: expected sent={expected_sent}, observed sent={observed_sent}"
            ),
        }
    }
}

impl Error for ChoiceLawViolation {}

#[derive(Clone, Copy)]
struct ExpectedChoice<'a> {
    candidate: &'a CandidateTrace,
    basis: ChoiceBasis,
    law: ChoiceLaw,
}

/// Verifies recorded choice decisions without running or changing the body.
pub fn verify_choice_laws(events: &[TraceEvent]) -> Result<(), ChoiceLawViolation> {
    let mut pending = Vec::<&CandidateTrace>::new();
    for event in events {
        match event {
            TraceEvent::Candidate(candidate) => pending.push(candidate),
            TraceEvent::Choice(choice) => {
                let candidates = pending
                    .iter()
                    .copied()
                    .filter(|candidate| {
                        candidate.at == choice.at && candidate.group == choice.group
                    })
                    .collect::<Vec<_>>();
                verify_choice(choice, &candidates)?;
                pending.retain(|candidate| {
                    candidate.at != choice.at || candidate.group != choice.group
                });
            }
            _ => {}
        }
    }
    if let Some(candidate) = pending.first() {
        return Err(ChoiceLawViolation::MissingChoice {
            at: candidate.at,
            group: candidate.group,
        });
    }
    Ok(())
}

fn verify_choice(
    choice: &ChoiceTrace,
    candidates: &[&CandidateTrace],
) -> Result<(), ChoiceLawViolation> {
    if choice.alternatives != candidates.len() {
        return Err(ChoiceLawViolation::CandidateCount {
            at: choice.at,
            group: choice.group,
            recorded: choice.alternatives,
            observed: candidates.len(),
        });
    }
    let observed = match choice.winner {
        Some(winner) => Some(
            candidates
                .iter()
                .copied()
                .find(|candidate| candidate.path == winner)
                .ok_or(ChoiceLawViolation::WinnerNotCandidate {
                    at: choice.at,
                    group: choice.group,
                    winner,
                })?,
        ),
        None => None,
    };
    let expected = expected_choice(candidates, choice.construction);
    match (expected, observed) {
        (None, Some(winner)) => {
            return Err(ChoiceLawViolation::UnexpectedWinner {
                at: choice.at,
                group: choice.group,
                winner: winner.path,
            });
        }
        (Some(expected), None) => {
            return Err(ChoiceLawViolation::MissingWinner {
                at: choice.at,
                group: choice.group,
                law: expected.law,
                expected: expected.candidate.path,
            });
        }
        (Some(expected), Some(observed)) => {
            if expected.law != ChoiceLaw::CurrentReturn {
                let strongest_drive = candidates
                    .iter()
                    .copied()
                    .filter(|candidate| choice.construction || candidate.executable)
                    .map(|candidate| candidate.drive)
                    .max()
                    .expect("an expected choice has an eligible candidate");
                if observed.drive != strongest_drive {
                    return Err(ChoiceLawViolation::CurrentSurfaceLocality {
                        at: choice.at,
                        group: choice.group,
                        winner: observed.path,
                        winner_drive: observed.drive,
                        strongest_drive,
                    });
                }
            }
            if expected.candidate.path != observed.path {
                return Err(ChoiceLawViolation::WrongWinner {
                    at: choice.at,
                    group: choice.group,
                    law: expected.law,
                    expected: expected.candidate.path,
                    observed: observed.path,
                });
            }
            if choice.basis != Some(expected.basis) {
                return Err(ChoiceLawViolation::WrongBasis {
                    at: choice.at,
                    group: choice.group,
                    law: expected.law,
                    expected: Some(expected.basis),
                    observed: choice.basis,
                });
            }
        }
        (None, None) => {
            if choice.basis.is_some() {
                return Err(ChoiceLawViolation::WrongBasis {
                    at: choice.at,
                    group: choice.group,
                    law: ChoiceLaw::Eligibility,
                    expected: None,
                    observed: choice.basis,
                });
            }
        }
    }
    let expected_sent = choice.winner.is_some() && !choice.construction;
    if choice.sent != expected_sent {
        return Err(ChoiceLawViolation::Delivery {
            at: choice.at,
            group: choice.group,
            expected_sent,
            observed_sent: choice.sent,
        });
    }
    Ok(())
}

fn expected_choice<'a>(
    candidates: &'a [&'a CandidateTrace],
    construction: bool,
) -> Option<ExpectedChoice<'a>> {
    let eligible = candidates
        .iter()
        .copied()
        .filter(|candidate| construction || candidate.executable)
        .collect::<Vec<_>>();
    let exact_returns = eligible
        .iter()
        .copied()
        .filter(|candidate| {
            candidate.return_cause.is_some() && candidate.return_cause == Some(candidate.cause)
        })
        .collect::<Vec<_>>();
    if let [candidate] = exact_returns.as_slice() {
        return Some(ExpectedChoice {
            candidate,
            basis: ChoiceBasis::CurrentReturn,
            law: ChoiceLaw::CurrentReturn,
        });
    }

    let strongest_drive = eligible.iter().map(|candidate| candidate.drive).max()?;
    let active = eligible
        .iter()
        .copied()
        .filter(|candidate| candidate.drive == strongest_drive)
        .collect::<Vec<_>>();
    let output_is_tried = |output| {
        active
            .iter()
            .any(|candidate| candidate.path.output == output && candidate.participation > 0)
    };
    let has_tried_output = active
        .iter()
        .any(|candidate| output_is_tried(candidate.path.output));
    let has_untried_output = active
        .iter()
        .any(|candidate| !output_is_tried(candidate.path.output));
    if has_tried_output && has_untried_output {
        let candidate = active
            .iter()
            .copied()
            .filter(|candidate| !output_is_tried(candidate.path.output))
            .max_by_key(|candidate| preference(candidate))?;
        return Some(ExpectedChoice {
            candidate,
            basis: ChoiceBasis::UntriedOutputRelease,
            law: ChoiceLaw::UntriedOutputRelease,
        });
    }
    if let Some(candidate) = unique_latest(&active, true) {
        return Some(ExpectedChoice {
            candidate,
            basis: ChoiceBasis::AvailableOutcome,
            law: ChoiceLaw::AvailableOutcome,
        });
    }
    if let Some(candidate) = unique_latest(&active, false) {
        return Some(ExpectedChoice {
            candidate,
            basis: ChoiceBasis::LatestOutcome,
            law: ChoiceLaw::LatestOutcome,
        });
    }
    active
        .iter()
        .copied()
        .max_by_key(|candidate| preference(candidate))
        .map(|candidate| ExpectedChoice {
            candidate,
            basis: ChoiceBasis::ParticipationStrengthAndDrive,
            law: ChoiceLaw::ParticipationStrengthAndDrive,
        })
}

fn preference(candidate: &CandidateTrace) -> (u64, i64, u16, Reverse<u32>) {
    (
        candidate.participation,
        candidate.strength,
        candidate.drive,
        Reverse(candidate.stable_order),
    )
}

fn unique_latest<'a>(
    candidates: &[&'a CandidateTrace],
    available_only: bool,
) -> Option<&'a CandidateTrace> {
    let latest = candidates
        .iter()
        .filter_map(|candidate| {
            candidate
                .outcome
                .filter(|outcome| !available_only || outcome.available_until_choice)
        })
        .map(|outcome| outcome.at)
        .max()?;
    let mut latest_candidates = candidates.iter().copied().filter(|candidate| {
        candidate.outcome.is_some_and(|outcome| {
            outcome.at == latest && (!available_only || outcome.available_until_choice)
        })
    });
    let candidate = latest_candidates.next()?;
    latest_candidates.next().is_none().then_some(candidate)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReturnDecision {
    BlockedByReadyPath,
    NoOpenPath,
    Ambiguous,
    BeforeReturnOpened,
    Accepted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReturnTrace {
    pub at: Time,
    pub source: JunctionId,
    pub incoming_cause: Cause,
    pub path: Option<Path>,
    pub return_cause: Option<Cause>,
    pub return_opened_at: Option<Time>,
    pub open_paths: usize,
    pub exact_paths: usize,
    pub decision: ReturnDecision,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StrengthTrace {
    pub at: Time,
    pub link: LinkId,
    pub before: i64,
    pub after: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraceEvent {
    Arrival(TraceArrival),
    Transition(PhysicalEvent),
    Candidate(CandidateTrace),
    Choice(ChoiceTrace),
    Return(ReturnTrace),
    Strengthened(StrengthTrace),
    Quiet(Run),
}

pub(crate) trait TraceSink {
    const ENABLED: bool;

    fn record(&mut self, event: TraceEvent);
}

pub(crate) struct NoTrace;

impl TraceSink for NoTrace {
    const ENABLED: bool = false;

    #[inline]
    fn record(&mut self, _event: TraceEvent) {}
}

pub(crate) struct ObserveTrace<F>(F);

impl<F> ObserveTrace<F> {
    pub(crate) const fn new(observe: F) -> Self {
        Self(observe)
    }
}

impl<F: FnMut(TraceEvent)> TraceSink for ObserveTrace<F> {
    const ENABLED: bool = true;

    fn record(&mut self, event: TraceEvent) {
        (self.0)(event);
    }
}

#[cfg(test)]
mod tests {
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
            strength: 1,
            drive,
            stable_order: index as u32,
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
}
