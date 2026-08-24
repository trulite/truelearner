//! Clean post-M8 organism surface.
//!
//! The historical experiment modules remain untouched as immutable scientific
//! provenance. New capability work should use this module: [`substrate`] is
//! the semantic-free physical runtime, and [`conformance`] checks it together
//! with the cumulative M8 development fingerprint.

pub mod conformance;

/// Compatibility path for the historical experiment crate.
///
/// New development must depend on the standalone
/// `frozen-organism-v1-physics` crate instead of importing the research
/// archive. The complete M0--M8 learner will wrap this surface behind a
/// separately conformance-checked host API.
pub mod substrate {
    pub use frozen_organism_v1_physics::*;
}

pub use substrate::{
    ArrowId, ArrowSpec, CellId, CellSpec, Crossing, Execution, SpikeInput, Substrate, TraceEntry,
    WorkLedger,
};
