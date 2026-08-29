use std::path::PathBuf;
use workstation_return_bearing_opportunity_composition::capture_visual_transition_pair;

fn main() {
    let mut args = std::env::args_os().skip(1);
    let output = PathBuf::from(
        args.next()
            .expect("usage: visual_transition_trace OUTPUT [STEPS]"),
    );
    let steps = args
        .next()
        .map(|value| {
            value
                .to_string_lossy()
                .parse()
                .expect("STEPS is an integer")
        })
        .unwrap_or(64);
    assert!(args.next().is_none(), "too many arguments");
    let trace = capture_visual_transition_pair(steps).expect("trace capture succeeds");
    let encoded = serde_json::to_vec_pretty(&trace).expect("trace serializes");
    std::fs::write(output, encoded).expect("trace writes");
}
