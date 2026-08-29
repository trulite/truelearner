use std::path::PathBuf;
use workstation_return_bearing_opportunity_composition::capture_monitor_cue_action_outcome_reuse;

fn main() {
    let mut args = std::env::args_os().skip(1);
    let output = PathBuf::from(
        args.next()
            .expect("usage: monitor_cue_reuse OUTPUT [DEVELOPMENT_STEPS] [PROBE_STEPS]"),
    );
    let development_steps = parse_steps(args.next()).unwrap_or(120);
    let probe_steps = parse_steps(args.next()).unwrap_or(24);
    assert!(args.next().is_none(), "too many arguments");
    let trace = capture_monitor_cue_action_outcome_reuse(development_steps, probe_steps)
        .expect("trace capture succeeds");
    let encoded = serde_json::to_vec_pretty(&trace).expect("trace serializes");
    std::fs::write(output, encoded).expect("trace writes");
}

fn parse_steps(value: Option<std::ffi::OsString>) -> Option<usize> {
    value.map(|value| value.to_string_lossy().parse().expect("steps are integers"))
}
