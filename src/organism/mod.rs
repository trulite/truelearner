//! Clean post-M8 organism surface.
//!
//! The historical experiment modules remain untouched as immutable scientific
//! provenance. New capability work should use this module: [`substrate`] is
//! the semantic-free physical runtime, and [`conformance`] checks it together
//! with the cumulative M8 development fingerprint.

pub mod conformance;
pub mod substrate;

pub use substrate::{
    ArrowId, ArrowSpec, CellId, CellSpec, Crossing, Execution, SpikeInput, Substrate, TraceEntry,
    WorkLedger,
};
