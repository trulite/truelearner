use std::path::PathBuf;
use std::time::Instant;
use workstation_return_bearing_opportunity_composition::capture_runtime_attached_complete_candidate_with_progress;

fn main() {
    let mut args = std::env::args_os().skip(1);
    let output = PathBuf::from(args.next().expect(
        "usage: runtime_attached_complete_workstation_trace OUTPUT [DEVELOPMENT_STEPS] [PROBE_STEPS]",
    ));
    let development_steps = parse_steps(args.next()).unwrap_or(96);
    let probe_steps = parse_steps(args.next()).unwrap_or(64);
    assert!(args.next().is_none(), "too many arguments");
    let started = Instant::now();
    let trace = capture_runtime_attached_complete_candidate_with_progress(
        development_steps,
        probe_steps,
        |progress| {
            if progress.stage_step == 1
                || progress.stage_step == progress.stage_budget
                || progress.stage_step.is_multiple_of(8)
            {
                eprintln!(
                    "progress {}/{} stage={:?} step={}/{} replay={} work={} resident_bytes={} elapsed_seconds={:.3}",
                    progress.completed_steps,
                    progress.planned_max_steps,
                    progress.stage,
                    progress.stage_step,
                    progress.stage_budget,
                    progress.replay,
                    progress.physical_work,
                    progress.resident_bytes,
                    started.elapsed().as_secs_f64(),
                );
            }
        },
    )
    .expect("complete trace capture succeeds");
    let encoded = serde_json::to_vec_pretty(&trace).expect("complete trace serializes");
    std::fs::write(output, encoded).expect("complete trace writes");
}

fn parse_steps(value: Option<std::ffi::OsString>) -> Option<usize> {
    value.map(|value| value.to_string_lossy().parse().expect("steps are integers"))
}
