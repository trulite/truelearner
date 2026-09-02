//! Runs the Workstation2 course from a checkpoint and prints its evidence.
use academy_workstation2_course::{Capability, Workstation2Course};
use truelearner_workstation::{WorkstationCheckpoint, WorkstationHarness};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let steps: usize = args.first().and_then(|s| s.parse().ok()).unwrap_or(96);
    let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(11);
    let checkpoint = match args.get(1) {
        Some(path) if path != "fresh" => {
            WorkstationCheckpoint::decode(&std::fs::read(path).expect("checkpoint file")).unwrap()
        }
        _ => WorkstationHarness::new(seed).unwrap().save().unwrap(),
    };
    let run = Workstation2Course::new(steps)
        .run(checkpoint, seed)
        .unwrap();
    println!(
        "steps={steps} replay={} first_failure={:?}",
        run.exact_replay, run.first_failure
    );
    for c in Capability::ALL {
        println!("  {c:?}: {:?}", run.state(c));
    }
    println!("  gaze:  {:?}", run.gaze);
    println!("  touch: {:?}", run.touch);
    for outcome in &run.rungs {
        println!("  {:?}: state={:?}", outcome.kind, outcome.run.state);
        if let Some(dev) = &outcome.run.development {
            println!("    dev:    {dev:?}");
        }
        for (i, probe) in outcome.run.probes.iter().enumerate() {
            println!("    probe{i}: {probe:?}");
        }
        for (i, control) in outcome.run.controls.iter().enumerate() {
            println!("    ctrl{i}:  {control:?}");
        }
    }
    let tap = &run.aimed_tap;
    println!("  tap: state={:?} replay={}", tap.state, tap.exact_replay);
    println!("    dev:    {:?}", tap.development);
    for (i, p) in tap.probes.iter().enumerate() {
        println!("    probe{i}: {p:?}");
    }
    for (i, c) in tap.blind_controls.iter().enumerate() {
        println!("    blind{i}: {c:?}");
    }
}
