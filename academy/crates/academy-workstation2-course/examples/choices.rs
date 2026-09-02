//! Causally inert choice-level trace of a Workstation2 session window.
use academy_workstation2::Workstation2;
use truelearner_workstation::{
    BodyAxis, BodyTraceEvent, WorkstationCheckpoint, WorkstationHarness,
};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let path = &args[0];
    let from: usize = args[1].parse().unwrap();
    let to: usize = args[2].parse().unwrap();
    let axes_only = args.get(3).is_none_or(|s| s != "all");
    let checkpoint = if path == "fresh" {
        WorkstationHarness::new(11).unwrap().save().unwrap()
    } else {
        WorkstationCheckpoint::decode(&std::fs::read(path).unwrap()).unwrap()
    };
    let mut harness = WorkstationHarness::restore(checkpoint).unwrap();
    let mut world = Workstation2::new(0);
    for step in 0..to {
        let sample = world.sense(harness.state()).unwrap();
        let (obs, trace) = harness.step_traced(sample).unwrap();
        let _ = world.advance(&obs.state_after);
        if step < from {
            continue;
        }
        let palm = obs.state_after.hand().palm();
        println!(
            "== step {step} tick {} palm=({},{},{}) crossings={:?}",
            obs.physical_tick,
            palm.x(),
            palm.y(),
            palm.depth(),
            obs.crossings
                .iter()
                .map(|c| format!("{:?}/{:?}", c.control.axis(), c.control.direction()))
                .collect::<Vec<_>>()
        );
        for event in &trace {
            match event {
                BodyTraceEvent::Candidate(c) => {
                    let control = harness.control_for_trace_output(c.path.output);
                    let hand_axis = control.is_some_and(|k| {
                        matches!(
                            k.axis(),
                            BodyAxis::PalmHorizontal | BodyAxis::PalmVertical | BodyAxis::PalmDepth
                        )
                    });
                    if axes_only && !hand_axis {
                        continue;
                    }
                    println!(
                        "  cand t{} g{} {:?} exec={} part={}@{} outpart={} unans={} outcome={:?} src={:?} bopen={} binh={} resist={} str={} drive={} new={}",
                        c.at,
                        c.group,
                        control.map(|k| format!("{:?}/{:?}", k.axis(), k.direction())),
                        c.executable,
                        c.participation,
                        c.participated_at,
                        c.output_participated,
                        c.unanswered,
                        c.outcome.map(|o| (o.at, o.changed_world, o.available_until_choice)),
                        c.outcome_source.map(|j| format!("{j:?}")),
                        c.boundary_open,
                        c.boundary_inhibited,
                        c.resisted_progress,
                        c.strength,
                        c.drive,
                        c.new_path,
                    );
                }
                BodyTraceEvent::Choice(ch) => {
                    let control = ch
                        .winner
                        .and_then(|w| harness.control_for_trace_output(w.output));
                    let hand_axis = control.is_some_and(|k| {
                        matches!(
                            k.axis(),
                            BodyAxis::PalmHorizontal | BodyAxis::PalmVertical | BodyAxis::PalmDepth
                        )
                    });
                    if axes_only && !hand_axis {
                        continue;
                    }
                    println!(
                        "  CHOICE t{} g{} alts={} winner={:?} warrant={:?} sent={}",
                        ch.at,
                        ch.group,
                        ch.alternatives,
                        control.map(|k| format!("{:?}/{:?}", k.axis(), k.direction())),
                        ch.warrant,
                        ch.sent
                    );
                }
                BodyTraceEvent::Return(r) => {
                    println!("  RETURN {:?}", r);
                }
                _ => {}
            }
        }
    }
}
