//! Fixture: does gaze converge on the only lit thing? Observer-only.
use academy_workstation2::{Rect, TargetApp, TargetLayout, Workstation2, Workstation2Session};
use truelearner_workstation::{Eye, WorkstationCheckpoint, WorkstationHarness};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let steps: usize = args[0].parse().unwrap();
    let checkpoint = if args[1] == "fresh" {
        WorkstationHarness::new(11).unwrap().save().unwrap()
    } else {
        WorkstationCheckpoint::decode(&std::fs::read(&args[1]).unwrap()).unwrap()
    };
    let left: i16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(192);
    let top: i16 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(192);
    let visible = args.get(4).is_none_or(|s| s != "blind");
    let rect = Rect {
        left,
        top,
        right: left + 192,
        bottom: top + 192,
    };
    let app = TargetApp::new(
        TargetLayout {
            target: Some(rect),
            decoy: None,
            goal: None,
            target_band: 230,
            decoy_band: 140,
            goal_band: 176,
            reactive: false,
            visible,
        },
        1,
    );
    let mut session =
        Workstation2Session::with_world(checkpoint, Workstation2::with_target(app)).unwrap();
    let (cx, cy) = (i32::from(left) + 96, i32::from(top) + 96);
    let mut foveal = 0;
    let mut sum_distance = 0_i64;
    for step in 0..steps {
        let o = session.step().unwrap();
        let gaze = o.body.state_after.eye(Eye::Left).gaze();
        let dx = i32::from(gaze.x()) - cx;
        let dy = i32::from(gaze.y()) - cy;
        let distance = dx.abs().max(dy.abs());
        sum_distance += i64::from(distance);
        let pixels = o.sample.eye(Eye::Left).foveal().pixels();
        let on_fovea = pixels[pixels.len() / 2] == 230;
        foveal += usize::from(on_fovea);
        if step % 8 == 0 || step + 1 == steps {
            println!(
                "{step:>4} gaze=({},{}) target_centre=({cx},{cy}) dist={distance:>4} fovea={} hand_in_view={}",
                gaze.x(),
                gaze.y(),
                on_fovea,
                pixels.contains(&8)
            );
        }
    }
    println!(
        "foveal_steps={foveal}/{steps} mean_distance={}",
        sum_distance / steps as i64
    );
}
