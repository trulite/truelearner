use crate::{
    Cause, JunctionId, JunctionRef, LinkId, LinkRef, Outcome, Path, PhysicalEvent, Run, Time,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Reverse, error::Error, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct TraceArrival {
    pub at: Time,
    pub target: JunctionId,
    pub impulse: i32,
    pub cause: Cause,
    pub via: Option<LinkId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct TracePath {
    pub surface: JunctionId,
    pub middle: JunctionRef,
    pub output: JunctionId,
    pub first: LinkRef,
    pub second: LinkRef,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct FreshOpportunityTrace {
    pub source: JunctionId,
    pub output: JunctionId,
    pub through: LinkId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReentryStepTrace {
    pub path: Path,
    pub returned_source: JunctionId,
    pub outcome_witness: LinkId,
    pub outcome_target: JunctionId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReentryTrace {
    pub condition: JunctionId,
    pub steps: Vec<ReentryStepTrace>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct MotifReentryTrace {
    pub witness: LinkId,
    pub parent: LinkId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MotifRouteStepTrace {
    pub surface: JunctionId,
    pub output: JunctionId,
    pub through: LinkId,
    pub impulse: i32,
    pub outcome_source: JunctionId,
    pub supports: Vec<MotifReentryTrace>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MotifRouteTrace {
    pub condition: JunctionId,
    pub steps: Vec<MotifRouteStepTrace>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
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
    pub participated_at: Time,
    pub output_participated: bool,
    pub outcome_source: Option<JunctionId>,
    pub progress_source: Option<JunctionId>,
    pub resisted_progress: bool,
    pub boundary_open: bool,
    pub boundary_inhibited: bool,
    pub strength: i64,
    pub drive: u16,
    pub stable_order: u32,
    pub fresh_opportunity: Option<FreshOpportunityTrace>,
    pub present_sources: Vec<JunctionId>,
    pub reentries: Vec<ReentryTrace>,
    pub motif_reentries: Vec<MotifReentryTrace>,
    pub motif_routes: Vec<MotifRouteTrace>,
    /// Local graph incidences actually examined by the bounded reentry resolver.
    pub reentry_incidence_visits: u16,
    /// Previously compiled, dependency-checked reentry searches reused this wave.
    pub reentry_shortcut_hits: u16,
    pub reentry_failed: bool,
    pub motif_route_failed: bool,
    pub new_path: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ChoiceBasis {
    CurrentReturn,
    BoundaryRelease,
    RetainedProgress,
    FreshOpportunity,
    UniqueReentry,
    UniqueMotifReentry,
    AvailableOutcome,
    UnansweredOutputRelease,
    LatestOutcome,
    UntriedOutputRelease,
    ParticipationStrengthAndDrive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
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
    BoundaryRelease,
    RetainedProgress,
    CurrentSurfaceLocality,
    FreshOpportunity,
    UniqueReentry,
    UniqueMotifReentry,
    UntriedOutputRelease,
    AvailableOutcome,
    UnansweredOutputRelease,
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

/// Verifies recorded choice decisions and receipt shape without changing the body.
///
/// Historical exact-return and motif-form support are established by the body
/// before projection; this observer has no retained topology from which to
/// reconstruct that ancestry.
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
            if !matches!(
                expected.law,
                ChoiceLaw::CurrentReturn | ChoiceLaw::BoundaryRelease
            ) {
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
    if !construction {
        let mut inhibited = candidates
            .iter()
            .copied()
            .filter(|candidate| candidate.executable && candidate.boundary_inhibited);
        if let Some(first) = inhibited.next() {
            let source = first.outcome_source?;
            if inhibited.any(|candidate| candidate.outcome_source != Some(source)) {
                return None;
            }
            let local = candidates
                .iter()
                .copied()
                .filter(|candidate| {
                    candidate.executable
                        && !candidate.boundary_inhibited
                        && candidate.outcome_source == Some(source)
                })
                .collect::<Vec<_>>();
            return unique_output(&local).map(|candidate| ExpectedChoice {
                candidate,
                basis: ChoiceBasis::BoundaryRelease,
                law: ChoiceLaw::BoundaryRelease,
            });
        }
    }
    let eligible = candidates
        .iter()
        .copied()
        .filter(|candidate| construction || (candidate.executable && !candidate.boundary_inhibited))
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
    let fresh = eligible
        .iter()
        .copied()
        .filter(|candidate| candidate.fresh_opportunity.is_some())
        .collect::<Vec<_>>();
    if let [candidate] = fresh.as_slice() {
        return Some(ExpectedChoice {
            candidate,
            basis: ChoiceBasis::FreshOpportunity,
            law: ChoiceLaw::FreshOpportunity,
        });
    }
    if let Some(candidate) = unique_retained_progress(&active) {
        return Some(ExpectedChoice {
            candidate,
            basis: ChoiceBasis::RetainedProgress,
            law: ChoiceLaw::RetainedProgress,
        });
    }
    let output_is_tried = |output| {
        active.iter().any(|candidate| {
            candidate.path.output == output
                && (candidate.participation > 0 || candidate.output_participated)
        })
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
    if let Some(candidate) = unique_returned_output(&active) {
        return Some(ExpectedChoice {
            candidate,
            basis: ChoiceBasis::CurrentReturn,
            law: ChoiceLaw::CurrentReturn,
        });
    }
    if let Some(candidate) = unique_reentry(&active) {
        return Some(ExpectedChoice {
            candidate,
            basis: ChoiceBasis::UniqueReentry,
            law: ChoiceLaw::UniqueReentry,
        });
    }
    if let Some(candidate) = unique_motif_reentry(&active) {
        return Some(ExpectedChoice {
            candidate,
            basis: ChoiceBasis::UniqueMotifReentry,
            law: ChoiceLaw::UniqueMotifReentry,
        });
    }
    if let Some(candidate) = unique_latest(&active, true) {
        return Some(ExpectedChoice {
            candidate,
            basis: ChoiceBasis::AvailableOutcome,
            law: ChoiceLaw::AvailableOutcome,
        });
    }
    let latest_unanswered = latest_unanswered_output(&active);
    if let Some(unanswered) = latest_unanswered {
        if let Some(candidate) = active
            .iter()
            .copied()
            .filter(|candidate| candidate.path.output != unanswered)
            .max_by_key(|candidate| preference(candidate))
        {
            return Some(ExpectedChoice {
                candidate,
                basis: ChoiceBasis::UnansweredOutputRelease,
                law: ChoiceLaw::UnansweredOutputRelease,
            });
        }
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

fn unique_reentry<'a>(candidates: &[&'a CandidateTrace]) -> Option<&'a CandidateTrace> {
    if candidates.iter().copied().any(|candidate| {
        candidate.reentry_failed
            || candidate.reentries.len() > 1
            || candidate
                .reentries
                .iter()
                .any(|reentry| !valid_reentry(candidate, reentry))
    }) {
        return None;
    }
    let mut reaching = candidates
        .iter()
        .copied()
        .filter(|candidate| candidate.reentries.len() == 1);
    let candidate = reaching.next()?;
    reaching.next().is_none().then_some(candidate)
}

fn unique_motif_reentry<'a>(candidates: &[&'a CandidateTrace]) -> Option<&'a CandidateTrace> {
    if candidates.iter().copied().any(|candidate| {
        candidate.reentry_failed
            || (!candidate.motif_reentries.is_empty() && candidate.reentry_incidence_visits == 0)
            || candidate
                .motif_reentries
                .iter()
                .any(|support| support.witness == support.parent || !candidate.new_path)
            || candidate
                .motif_routes
                .iter()
                .any(|route| !valid_motif_route(candidate, route))
    }) {
        return None;
    }
    if candidates
        .iter()
        .any(|candidate| !candidate.motif_routes.is_empty())
    {
        if candidates
            .iter()
            .any(|candidate| candidate.motif_route_failed || candidate.motif_routes.len() > 1)
        {
            return None;
        }
        let mut reaching = candidates
            .iter()
            .copied()
            .filter(|candidate| candidate.motif_routes.len() == 1);
        let candidate = reaching.next()?;
        return reaching.next().is_none().then_some(candidate);
    }
    let mut reaching = candidates
        .iter()
        .copied()
        .filter(|candidate| !candidate.motif_reentries.is_empty());
    let candidate = reaching.next()?;
    reaching.next().is_none().then_some(candidate)
}

fn valid_motif_route(candidate: &CandidateTrace, route: &MotifRouteTrace) -> bool {
    let Some(first) = route.steps.first() else {
        return false;
    };
    if !candidate.new_path
        || candidate.motif_reentries.is_empty()
        || candidate.outcome_source != Some(first.surface)
        || route.condition == candidate.path.surface
        || !candidate.present_sources.contains(&route.condition)
        || route.steps.last().map(|step| step.outcome_source) != Some(route.condition)
        || candidate.reentry_incidence_visits == 0
    {
        return false;
    }
    for (index, step) in route.steps.iter().enumerate() {
        if step.impulse == 0
            || step.supports.is_empty()
            || step
                .supports
                .iter()
                .any(|support| support.witness == support.parent)
            || (index > 0 && step.surface != route.steps[index - 1].outcome_source)
            || route.steps[..index]
                .iter()
                .any(|earlier| earlier.surface == step.surface)
        {
            return false;
        }
    }
    true
}

fn valid_reentry(candidate: &CandidateTrace, reentry: &ReentryTrace) -> bool {
    let Some(first) = reentry.steps.first() else {
        return false;
    };
    if candidate.path
        != (TracePath {
            surface: first.path.surface,
            middle: first.path.middle.into(),
            output: first.path.output,
            first: first.path.first.into(),
            second: first.path.second.into(),
        })
        || !candidate.present_sources.contains(&reentry.condition)
        || reentry.steps.last().map(|step| step.returned_source) != Some(reentry.condition)
    {
        return false;
    }
    for (index, step) in reentry.steps.iter().enumerate() {
        if step.outcome_target != step.path.middle && step.outcome_target != step.path.output {
            return false;
        }
        if index > 0 && step.path.surface != reentry.steps[index - 1].returned_source {
            return false;
        }
        if reentry.steps[..index].iter().any(|earlier| {
            earlier.path == step.path || earlier.outcome_witness == step.outcome_witness
        }) {
            return false;
        }
    }
    candidate.reentry_incidence_visits > 0 || candidate.reentry_shortcut_hits > 0
}

fn latest_unanswered_output(candidates: &[&CandidateTrace]) -> Option<JunctionId> {
    let mut latest = None;
    let mut output = None;
    let mut ambiguous = false;
    for candidate in candidates
        .iter()
        .copied()
        .filter(|candidate| candidate.unanswered && candidate.participation > 0)
    {
        match latest {
            None => {
                latest = Some(candidate.participated_at);
                output = Some(candidate.path.output);
            }
            Some(at) if candidate.participated_at > at => {
                latest = Some(candidate.participated_at);
                output = Some(candidate.path.output);
                ambiguous = false;
            }
            Some(at)
                if candidate.participated_at == at && output != Some(candidate.path.output) =>
            {
                ambiguous = true;
            }
            Some(_) => {}
        }
    }
    if ambiguous {
        None
    } else {
        output
    }
}

fn unique_returned_output<'a>(candidates: &[&'a CandidateTrace]) -> Option<&'a CandidateTrace> {
    unique_output(
        &candidates
            .iter()
            .copied()
            .filter(|candidate| candidate.output_participated)
            .collect::<Vec<_>>(),
    )
}

fn unique_output<'a>(candidates: &[&'a CandidateTrace]) -> Option<&'a CandidateTrace> {
    let mut output: Option<JunctionId> = None;
    let mut winner: Option<&CandidateTrace> = None;
    for candidate in candidates.iter().copied() {
        match output {
            None => output = Some(candidate.path.output),
            Some(current) if current != candidate.path.output => return None,
            Some(_) => {}
        }
        if winner.is_none_or(|selected| preference(candidate) > preference(selected)) {
            winner = Some(candidate);
        }
    }
    winner
}

fn unique_retained_progress<'a>(candidates: &[&'a CandidateTrace]) -> Option<&'a CandidateTrace> {
    let progressing = candidates
        .iter()
        .copied()
        .filter(|candidate| {
            candidate.resisted_progress
                && candidate.boundary_open
                && candidate.strength > 1
                && candidate.participation > 0
        })
        .collect::<Vec<_>>();
    unique_output(&progressing)
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ReturnDecision {
    BlockedByReadyPath,
    NoOpenPath,
    Ambiguous,
    BeforeReturnOpened,
    Accepted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct ReturnCandidateTrace {
    pub path: Path,
    pub cause: Cause,
    pub opened_at: Time,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReturnTrace {
    pub at: Time,
    pub source: JunctionId,
    pub incoming_cause: Cause,
    pub path: Option<Path>,
    pub return_cause: Option<Cause>,
    pub return_opened_at: Option<Time>,
    pub offers_choice: Option<bool>,
    pub open_paths: usize,
    pub exact_paths: usize,
    pub candidates: Vec<ReturnCandidateTrace>,
    pub decision: ReturnDecision,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct StrengthTrace {
    pub at: Time,
    pub link: LinkId,
    pub before: i64,
    pub after: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
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
}
