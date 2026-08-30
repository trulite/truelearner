use crate::{
    Cause, JunctionId, JunctionRef, LinkId, LinkRef, Outcome, Path, PhysicalEvent, Run, Time,
};

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
    pub new_path: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChoiceBasis {
    CurrentReturn,
    AvailableOutcome,
    LatestOutcome,
    UntriedOutputRelease,
    ParticipationAndStrength,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChoiceTrace {
    pub at: Time,
    pub group: usize,
    pub alternatives: usize,
    pub winner: Option<TracePath>,
    pub basis: Option<ChoiceBasis>,
    pub sent: bool,
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
