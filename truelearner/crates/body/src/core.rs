//! What changes because of one observed physical event.

use crate::{
    arena::Arena,
    engine::PhysicalMoment,
    physics::opens,
    trace::{
        CandidateTrace, ChoiceBasis, ChoiceTrace, FreshOpportunityTrace, MotifReentryTrace,
        MotifRouteStepTrace, MotifRouteTrace, NoTrace, ReentryStepTrace, ReentryTrace,
        ReturnCandidateTrace, ReturnDecision, ReturnTrace, StrengthTrace, TraceEvent, TracePath,
        TraceSink,
    },
    Body, BuildError, Impulse, Junction, JunctionId, Link, LinkId, Retention, RunError, Time,
    Trigger,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Reverse, collections::HashMap};

// Ordered fragments of one core module. Keeping one privacy boundary makes this
// split organizational only: it neither exposes internals nor changes a law.
include!("core/model.rs");
include!("core/reaction.rs");
include!("core/automaticity.rs");
include!("core/closure.rs");
include!("core/candidates.rs");
include!("core/reentry.rs");
include!("core/choice.rs");
include!("core/state.rs");

#[cfg(test)]
#[path = "tests/core.rs"]
mod tests;
