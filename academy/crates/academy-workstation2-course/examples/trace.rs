//! Causally inert per-step trace of a Workstation2 session.
use academy_workstation2::Workstation2Session;
use truelearner_workstation::{BodyAxis, Digit, WorkstationCheckpoint, WorkstationHarness};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let steps: usize = args.first().and_then(|s| s.parse().ok()).unwrap_or(96);
    let shift: i16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let checkpoint = match args.get(1) {
        Some(path) if path != "fresh" => {
            WorkstationCheckpoint::decode(&std::fs::read(path).expect("checkpoint file")).unwrap()
        }
        _ => WorkstationHarness::new(11).unwrap().save().unwrap(),
    };
    let mut session = Workstation2Session::from_checkpoint(checkpoint, shift).unwrap();
    for _ in 0..steps {
        let o = session.step().unwrap();
        let hand = o.body.state_after.hand();
        let palm = hand.palm();
        let tips = Digit::ALL
            .into_iter()
            .map(|d| hand.fingertip(d).depth())
            .collect::<Vec<_>>();
        let crossings = o
            .body
            .crossings
            .iter()
            .map(|c| format!("{:?}{:?}", c.control.axis(), c.control.direction()))
            .collect::<Vec<_>>();
        let moved = o
            .body
            .movements
            .iter()
            .filter(|m| m.changed)
            .map(|m| format!("{:?}:{}", m.axis, m.net_impulse))
            .collect::<Vec<_>>();
        let contacts = o
            .sample
            .contacts()
            .iter()
            .filter(|c| c.pressure() > 0)
            .count();
        let depth_moves = o
            .body
            .movements
            .iter()
            .filter(|m| m.axis == BodyAxis::PalmDepth)
            .map(|m| {
                format!(
                    "d+{} d-{} net{} ch{}",
                    m.increase_effort, m.decrease_effort, m.net_impulse, m.changed
                )
            })
            .collect::<Vec<_>>();
        println!(
            "{:>3} t{:>5} palm=({},{},{}) tips={:?} contacts={} X={:?} M={:?} PD={:?} bp={} pp={} ret={:?} pend={:?} ev={:?} text={:?} scale={}",
            o.sequence,
            o.body.physical_tick,
            palm.x(),
            palm.y(),
            palm.depth(),
            tips,
            contacts,
            crossings,
            moved,
            depth_moves,
            o.body.boundary_parents.len(),
            o.body.progress_parents.len(),
            o.body.returned_transitions,
            o.body.pending_transitions,
            o.device_events,
            o.text,
            o.scale
        );
    }
}
