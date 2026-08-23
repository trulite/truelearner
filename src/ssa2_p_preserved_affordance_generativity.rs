//! SSA2-P development: long physical trajectories from preserved affordances.

#[allow(dead_code)]
mod composed {
    use crate::organism::Execution;

    include!(concat!(
        env!("OUT_DIR"),
        "/ssa1_learned_variation_control_frozen.rs"
    ));
    include!("ssa2_p_experiment.inc.rs");
}

pub use composed::{
    run_ssa2_p_gate, run_ssa2_p_micro, run_ssa2_p_probe, Ssa2pCell, Ssa2pReport, Ssa2pTrajectory,
};
