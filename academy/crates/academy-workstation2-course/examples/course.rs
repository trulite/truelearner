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
        "steps={steps} shift={} replay={} first_failure={:?}",
        run.probe_keyboard_shift, run.exact_replay, run.first_failure
    );
    for c in Capability::ALL {
        println!("  {c:?}: {:?}", run.state(c));
    }
    println!("  dev:   {:?}", run.development);
    println!("  probe: {:?}", run.shifted_probe);
}
