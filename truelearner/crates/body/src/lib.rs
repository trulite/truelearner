//! Small physical laws and the engine that composes them through time.

mod arena;
mod attachment;
mod calibration;
mod checkpoint;
pub mod core;
mod engine;
pub mod harness;
mod physics;
mod timeline;
mod trace;

pub use crate::attachment::{
    attach, AttachError, AttachFailure, Attachment, Join, OpenBody, OpenBodyError, Port,
};
pub use crate::calibration::{calibrate, Normalizer, Residual};
pub use crate::checkpoint::{BodyCheckpoint, BodyCheckpointError};
pub use crate::core::*;
pub use crate::engine::Body;
pub use crate::physics::{
    Arrival, BuildError, Event as PhysicalEvent, Impulse, Junction, JunctionId, Link, LinkId,
    Retention, Run, RunError, Step, Time, Trigger, Work,
};
pub use crate::timeline::{MomentKey, QueueWork, Timeline, TimelineItem};
pub use crate::trace::{
    CandidateTrace, ChoiceBasis, ChoiceTrace, ReturnDecision, ReturnTrace, StrengthTrace,
    TraceArrival, TraceEvent, TracePath,
};
