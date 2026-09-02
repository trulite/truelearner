use crate::{
    ChoiceWarrant, JunctionId, JunctionRef, LinkId, LinkRef, Outcome, Path, PhysicalEvent, Run,
    Time,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Reverse, error::Error, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct TraceArrival {
    pub at: Time,
    pub target: JunctionId,
    pub impulse: i32,
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
    pub group: usize,
    pub path: TracePath,
    pub connected_outcomes: Vec<JunctionId>,
    pub executable: bool,
    pub return_present: bool,
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
pub struct ChoiceTrace {
    pub at: Time,
    pub group: usize,
    pub alternatives: usize,
    pub winner: Option<TracePath>,
    pub warrant: Option<ChoiceWarrant>,
    pub construction: bool,
    pub sent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChoiceCheck {
    CandidateAccounting,
    Eligibility,
    LocalResolution,
    Delivery,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChoiceContractViolation {
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
        check: ChoiceCheck,
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
        check: ChoiceCheck,
        expected: TracePath,
        observed: TracePath,
    },
    WrongWarrant {
        at: Time,
        group: usize,
        check: ChoiceCheck,
        expected: Option<ChoiceWarrant>,
        observed: Option<ChoiceWarrant>,
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

impl ChoiceContractViolation {
    pub const fn check(&self) -> ChoiceCheck {
        match self {
            Self::MissingChoice { .. }
            | Self::CandidateCount { .. }
            | Self::WinnerNotCandidate { .. } => ChoiceCheck::CandidateAccounting,
            Self::MissingWinner { check, .. }
            | Self::WrongWinner { check, .. }
            | Self::WrongWarrant { check, .. } => *check,
            Self::UnexpectedWinner { .. } => ChoiceCheck::Eligibility,
            Self::CurrentSurfaceLocality { .. } => ChoiceCheck::LocalResolution,
            Self::Delivery { .. } => ChoiceCheck::Delivery,
        }
    }
}

impl fmt::Display for ChoiceContractViolation {
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
            Self::MissingWinner {
                at, group, check, ..
            } => write!(
                formatter,
                "choice group {group} at {at} violated {check:?}: the required candidate was not selected"
            ),
            Self::UnexpectedWinner { at, group, .. } => write!(
                formatter,
                "choice group {group} at {at} violated Eligibility: an ineligible candidate was selected"
            ),
            Self::WrongWinner {
                at, group, check, ..
            } => write!(
                formatter,
                "choice group {group} at {at} violated {check:?}: it selected the wrong candidate"
            ),
            Self::WrongWarrant {
                at,
                group,
                check,
                expected,
                observed,
            } => write!(
                formatter,
                "choice group {group} at {at} violated {check:?}: expected warrant {expected:?}, observed {observed:?}"
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

impl Error for ChoiceContractViolation {}

#[derive(Clone, Copy)]
struct ExpectedChoice<'a> {
    candidate: &'a CandidateTrace,
    warrant: ChoiceWarrant,
}

/// Verifies recorded choice decisions and receipt shape without changing the body.
///
/// Historical exact-return and motif-form support are established by the body
/// before projection; this observer has no retained topology from which to
/// reconstruct that ancestry.
pub fn verify_choice_contract(events: &[TraceEvent]) -> Result<(), ChoiceContractViolation> {
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
        return Err(ChoiceContractViolation::MissingChoice {
            at: candidate.at,
            group: candidate.group,
        });
    }
    Ok(())
}

fn verify_choice(
    choice: &ChoiceTrace,
    candidates: &[&CandidateTrace],
) -> Result<(), ChoiceContractViolation> {
    if choice.alternatives != candidates.len() {
        return Err(ChoiceContractViolation::CandidateCount {
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
                .ok_or(ChoiceContractViolation::WinnerNotCandidate {
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
            return Err(ChoiceContractViolation::UnexpectedWinner {
                at: choice.at,
                group: choice.group,
                winner: winner.path,
            });
        }
        (Some(expected), None) => {
            return Err(ChoiceContractViolation::MissingWinner {
                at: choice.at,
                group: choice.group,
                check: ChoiceCheck::LocalResolution,
                expected: expected.candidate.path,
            });
        }
        (Some(expected), Some(observed)) => {
            if expected.warrant != ChoiceWarrant::ReturnedConsequence {
                let strongest_drive = candidates
                    .iter()
                    .copied()
                    .filter(|candidate| {
                        choice.construction
                            || (candidate.executable && !candidate.boundary_inhibited)
                    })
                    .map(|candidate| candidate.drive)
                    .max()
                    .expect("an expected choice has an eligible candidate");
                if observed.drive != strongest_drive {
                    return Err(ChoiceContractViolation::CurrentSurfaceLocality {
                        at: choice.at,
                        group: choice.group,
                        winner: observed.path,
                        winner_drive: observed.drive,
                        strongest_drive,
                    });
                }
            }
            if expected.candidate.path != observed.path {
                return Err(ChoiceContractViolation::WrongWinner {
                    at: choice.at,
                    group: choice.group,
                    check: ChoiceCheck::LocalResolution,
                    expected: expected.candidate.path,
                    observed: observed.path,
                });
            }
            if choice.warrant != Some(expected.warrant) {
                return Err(ChoiceContractViolation::WrongWarrant {
                    at: choice.at,
                    group: choice.group,
                    check: ChoiceCheck::LocalResolution,
                    expected: Some(expected.warrant),
                    observed: choice.warrant,
                });
            }
        }
        (None, None) => {
            if choice.warrant.is_some() {
                return Err(ChoiceContractViolation::WrongWarrant {
                    at: choice.at,
                    group: choice.group,
                    check: ChoiceCheck::Eligibility,
                    expected: None,
                    observed: choice.warrant,
                });
            }
        }
    }
    let expected_sent = choice.winner.is_some() && !choice.construction;
    if choice.sent != expected_sent {
        return Err(ChoiceContractViolation::Delivery {
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
        let inhibited = candidates
            .iter()
            .copied()
            .filter(|candidate| candidate.executable && candidate.boundary_inhibited)
            .collect::<Vec<_>>();
        if let Some(first) = inhibited.first() {
            let source = first.outcome_source?;
            if inhibited
                .iter()
                .any(|candidate| candidate.outcome_source != Some(source))
            {
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
            if !local.is_empty() {
                if let Some(candidate) = unique_output(&local) {
                    return Some(ExpectedChoice {
                        candidate,
                        warrant: ChoiceWarrant::ReturnedConsequence,
                    });
                }
            } else {
                let completed = latest_participated_output(&inhibited)?;
                let local = candidates
                    .iter()
                    .copied()
                    .filter(|candidate| {
                        candidate.executable
                            && candidate.outcome_source == Some(source)
                            && candidate.path.output != completed
                    })
                    .collect::<Vec<_>>();
                return unique_output(&local).map(|candidate| ExpectedChoice {
                    candidate,
                    warrant: ChoiceWarrant::ReturnedConsequence,
                });
            }
        }
    }
    let eligible = candidates
        .iter()
        .copied()
        .filter(|candidate| construction || (candidate.executable && !candidate.boundary_inhibited))
        .collect::<Vec<_>>();
    let returned = eligible
        .iter()
        .copied()
        .filter(|candidate| candidate.return_present)
        .collect::<Vec<_>>();
    if let [candidate] = returned.as_slice() {
        return Some(ExpectedChoice {
            candidate,
            warrant: ChoiceWarrant::ReturnedConsequence,
        });
    }

    let strongest_drive = eligible.iter().map(|candidate| candidate.drive).max()?;
    let active = eligible
        .iter()
        .copied()
        .filter(|candidate| candidate.drive == strongest_drive)
        .collect::<Vec<_>>();
    if let Some(candidate) = unique_retained_progress(&active) {
        return Some(ExpectedChoice {
            candidate,
            warrant: ChoiceWarrant::RetainedContinuation,
        });
    }
    let fresh = active
        .iter()
        .copied()
        .filter(|candidate| candidate.fresh_opportunity.is_some())
        .collect::<Vec<_>>();
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
        if let [candidate] = fresh.as_slice() {
            return Some(ExpectedChoice {
                candidate,
                warrant: ChoiceWarrant::RetainedContinuation,
            });
        }
        let candidate = active
            .iter()
            .copied()
            .filter(|candidate| !output_is_tried(candidate.path.output))
            .max_by_key(|candidate| preference(candidate))?;
        return Some(ExpectedChoice {
            candidate,
            warrant: ChoiceWarrant::Exploration,
        });
    }
    if let Some(candidate) = unique_returned_output(&active) {
        return Some(ExpectedChoice {
            candidate,
            warrant: ChoiceWarrant::ReturnedConsequence,
        });
    }
    if let Some(candidate) = unique_reentry(&active) {
        return Some(ExpectedChoice {
            candidate,
            warrant: ChoiceWarrant::Reentry,
        });
    }
    if let Some(candidate) = unique_motif_reentry(&active) {
        return Some(ExpectedChoice {
            candidate,
            warrant: ChoiceWarrant::Reentry,
        });
    }
    if let [candidate] = fresh.as_slice() {
        return Some(ExpectedChoice {
            candidate,
            warrant: ChoiceWarrant::RetainedContinuation,
        });
    }
    if let Some(candidate) = unique_latest(&active, true) {
        return Some(ExpectedChoice {
            candidate,
            warrant: ChoiceWarrant::RetainedContinuation,
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
                warrant: ChoiceWarrant::Exploration,
            });
        }
    }
    if let Some(candidate) = unique_latest(&active, false) {
        return Some(ExpectedChoice {
            candidate,
            warrant: ChoiceWarrant::LocalIncidence,
        });
    }
    active
        .iter()
        .copied()
        .max_by_key(|candidate| preference(candidate))
        .map(|candidate| ExpectedChoice {
            candidate,
            warrant: ChoiceWarrant::LocalIncidence,
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

fn latest_participated_output(candidates: &[&CandidateTrace]) -> Option<JunctionId> {
    let mut latest = None;
    let mut output = None;
    let mut ambiguous = false;
    for candidate in candidates
        .iter()
        .copied()
        .filter(|candidate| candidate.participation > 0)
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
    (!ambiguous).then_some(output).flatten()
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
    BlockedByCandidatePath,
    NoOpenPath,
    Ambiguous,
    BeforeReturnOpened,
    Accepted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct ReturnCandidateTrace {
    pub path: Path,
    pub opened_at: Time,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReturnTrace {
    pub at: Time,
    pub source: JunctionId,
    pub path: Option<Path>,
    pub return_opened_at: Option<Time>,
    pub offers_choice: Option<bool>,
    pub open_paths: usize,
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
#[path = "tests/trace.rs"]
mod tests;
