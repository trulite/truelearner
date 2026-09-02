//! What changes because of one observed physical event.

use crate::{
    arena::Arena,
    engine::PhysicalMoment,
    physics::opens,
    trace::{
        CandidateTrace, ChoiceTrace, FreshOpportunityTrace, MotifReentryTrace, MotifRouteStepTrace,
        MotifRouteTrace, ReentryStepTrace, ReentryTrace, ReturnCandidateTrace, ReturnDecision,
        ReturnTrace, StrengthTrace, TraceEvent, TracePath, TraceSink,
    },
    Body, BuildError, Impulse, Junction, JunctionId, Link, LinkId, Retention, RunError, Time,
    Trigger,
};
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
pub(crate) use truelearner_core::{ArrowState, ClosedSupport, Occurrence, WitnessKind};
pub use truelearner_core::{Outcome, Path};

// Ordered fragments of one core module. Keeping one privacy boundary makes this
// split organizational only: it neither exposes internals nor changes a law.
include!("core/model.rs");
include!("core/reaction.rs");
include!("core/consolidation.rs");
include!("core/closure.rs");
include!("core/candidates.rs");
include!("core/reentry.rs");
include!("core/choice.rs");
include!("core/state.rs");
