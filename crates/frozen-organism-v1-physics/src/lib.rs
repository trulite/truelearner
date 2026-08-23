#![forbid(unsafe_code)]
//! Frozen Organism v1 retained-physics surface.
//!
//! This crate contains only retained CELL/ARROW/SPIKE physics. It has no
//! dependency on the historical experiment crate, its evaluators, or its
//! semantic fixtures. It is not, by itself, the complete M0--M8 learner.

mod substrate;

pub use substrate::{
    ArrowId, ArrowSpec, CellId, CellSpec, Crossing, Execution, SpikeInput, Substrate, TraceEntry,
    WorkLedger,
};
