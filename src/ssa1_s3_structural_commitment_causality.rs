//! SSA1-S3 development: causal tests of S2's physical commitment classifier.

#[allow(dead_code)]
mod composed {
    include!(concat!(env!("OUT_DIR"), "/ssa1_s3_composition.rs"));
}

pub use composed::{run_s3_gate, run_s3_micro, run_s3_probe, S3Arm, S3Cell, S3Report};
