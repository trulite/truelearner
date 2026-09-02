//! Runs the aimed-tap rung from a checkpoint and prints its evidence.
use academy_workstation2_course::screen_use::{ScreenUseCourse, TapEvidence};
use truelearner_workstation::{WorkstationCheckpoint, WorkstationHarness};

fn show(name: &str, e: &TapEvidence) {
    println!(
        "{name:>8}: taps={:>3} on_target={:>3} hits={:>3} rate={:.3} chance={:.3} x{:.1} gaze={} seen_target={}/{} seen_hand={}/{} contact_steps={} palm_x={:?} palm_y={:?} depth={:?} quiet={}",
        e.taps,
        e.target_taps,
        e.hits,
        e.rate(),
        e.chance,
        if e.chance > 0.0 { e.rate() / e.chance } else { 0.0 },
        e.gaze_changes,
        e.target_seen_steps,
        e.target_foveal_steps,
        e.hand_seen_steps,
        e.hand_foveal_steps,
        e.contact_steps,
        e.palm_x,
        e.palm_y,
        e.depth,
        e.naturally_quiescent
    );
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let steps: usize = args[0].parse().unwrap();
    let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(11);
    let checkpoint = if args[1] == "fresh" {
        WorkstationHarness::new(seed).unwrap().save().unwrap()
    } else {
        WorkstationCheckpoint::decode(&std::fs::read(&args[1]).unwrap()).unwrap()
    };
    let run = ScreenUseCourse::new(steps)
        .aimed_tap(checkpoint, seed)
        .unwrap();
    println!(
        "aimed tap: state={:?} replay={}",
        run.state, run.exact_replay
    );
    show("dev", &run.development);
    for (i, p) in run.probes.iter().enumerate() {
        show(&format!("probe{i}"), p);
    }
    for (i, c) in run.blind_controls.iter().enumerate() {
        show(&format!("blind{i}"), c);
    }
}
