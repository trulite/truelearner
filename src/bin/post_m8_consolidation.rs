use organism_v0::organism::conformance::{replay_affordance_law, replay_m8_gate};

fn main() {
    let affordance = replay_affordance_law();
    assert!(affordance.passed(), "{affordance:#?}");
    println!("affordance causal-window fingerprint: PASS");

    let mode = std::env::args().nth(1);
    if mode.as_deref() != Some("--m8-gate") {
        println!("M8 replay: skipped (pass --m8-gate for the six-seed development GATE)");
        return;
    }

    let m8 = replay_m8_gate();
    assert!(m8.matches_frozen_gate(), "{m8:#?}");
    println!("M8 cumulative development fingerprint: PASS");
}
