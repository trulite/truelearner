//! Causally inert focused trace for one generic two-patch target phase.
use academy_workstation2::{ScreenPoint, TargetApp, Workstation2, Workstation2Session};
use truelearner_workstation::{Eye, WorkstationCheckpoint, WorkstationHarness};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let steps: usize = args
        .first()
        .and_then(|value| value.parse().ok())
        .unwrap_or(64);
    let checkpoint = match args.get(1) {
        Some(path) if path != "fresh" => {
            WorkstationCheckpoint::decode(&std::fs::read(path).expect("checkpoint file")).unwrap()
        }
        _ => WorkstationHarness::new(11).unwrap().save().unwrap(),
    };
    let seed = args
        .get(2)
        .and_then(|value| value.parse().ok())
        .unwrap_or(11);
    let app = match args.get(3).map(String::as_str) {
        Some("swapped") => TargetApp::dual(seed).swapped(),
        _ => TargetApp::dual(seed),
    };
    let initial_layout = app.layout();
    let mut session =
        Workstation2Session::with_world(checkpoint, Workstation2::with_target(app)).unwrap();

    println!(
        "target={:?} decoy={:?}",
        initial_layout.target, initial_layout.decoy
    );
    let mut previous_target = initial_layout.target;
    for step in 0..steps {
        let observation = session.step().unwrap();
        let layout = session.world().target().unwrap().layout();
        let gaze = observation.body.state_after.eye(Eye::Left).gaze();
        let right_gaze = observation.body.state_after.eye(Eye::Right).gaze();
        let palm = observation.body.state_after.hand().palm();
        let at = ScreenPoint {
            x: palm.x(),
            y: palm.y(),
        };
        if step % 8 == 0
            || !observation.device_events.is_empty()
            || layout.target != previous_target
        {
            let planar = observation
                .body
                .movements
                .iter()
                .filter(|movement| {
                    matches!(
                        movement.axis,
                        truelearner_workstation::BodyAxis::PalmHorizontal
                            | truelearner_workstation::BodyAxis::PalmVertical
                    )
                })
                .map(|movement| {
                    format!(
                        "{:?}:{}-{}={}",
                        movement.axis,
                        movement.decrease_effort,
                        movement.increase_effort,
                        movement.net_impulse
                    )
                })
                .collect::<Vec<_>>();
            println!(
                "{step:>3} gaze=({:>4},{:>4})/({:>4},{:>4}) palm=({:>4},{:>4},{:>4}) on_target={} on_decoy={} planar={planar:?} events={:?}",
                gaze.x(),
                gaze.y(),
                right_gaze.x(),
                right_gaze.y(),
                palm.x(),
                palm.y(),
                palm.depth(),
                layout.target.is_some_and(|rect| rect.contains(at)),
                layout.decoy.is_some_and(|rect| rect.contains(at)),
                observation.device_events,
            );
        }
        if layout.target != previous_target {
            println!("    target_moved={:?}", layout.target);
            previous_target = layout.target;
        }
    }
    let app = session.world().target().unwrap();
    println!(
        "taps={} target_taps={} decoy_taps={} hits={}",
        app.taps(),
        app.target_taps(),
        app.decoy_taps(),
        app.hits()
    );
}
