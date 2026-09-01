//! Small physical laws and the engine that composes them through time.

mod arena;
mod attachment;
mod calibration;
mod checkpoint;
mod core;
mod engine;
pub mod harness;
#[cfg(test)]
#[path = "tests/motif_laws.rs"]
mod motif_laws;
mod physics;
#[cfg(test)]
#[path = "tests/planning_goal_laws.rs"]
mod planning_goal_laws;
mod trace;

pub use crate::attachment::{
    attach, AttachError, AttachFailure, Attachment, Join, OpenBody, OpenBodyError, Port,
};
pub use crate::calibration::{calibrate, Normalizer, Residual};
pub use crate::checkpoint::{BodyCheckpoint, BodyCheckpointError};
pub use crate::core::{
    ApplyError, AutomaticityState, AutomaticityWork, Cause, JunctionRef, LinkRef, Outcome, Path,
    ReentryState,
};
#[cfg(test)]
pub(crate) use crate::core::{ArrowKind, ReturnStatus};
pub(crate) use crate::core::{ArrowState, Consolidation, ReentryCache, WitnessKind};
pub use crate::engine::Body;
pub use crate::physics::{
    Arrival, BuildError, Event as PhysicalEvent, Impulse, Junction, JunctionId, Link, LinkId,
    Retention, Run, RunError, Step, Time, Trigger, Work, DRIVE_MAX,
};
pub use crate::trace::{
    verify_choice_laws, CandidateTrace, ChoiceBasis, ChoiceLaw, ChoiceLawViolation, ChoiceTrace,
    MotifReentryTrace, MotifRouteStepTrace, MotifRouteTrace, ReentryStepTrace, ReentryTrace,
    ReturnCandidateTrace, ReturnDecision, ReturnTrace, StrengthTrace, TraceArrival, TraceEvent,
    TracePath,
};
