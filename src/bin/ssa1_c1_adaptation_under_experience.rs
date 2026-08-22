use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use organism_v0::ssa1_c1_adaptation_under_experience::{run_gate, run_micro, run_probe, Report};

fn write_atomic(path: &Path, contents: &str) {
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, contents).expect("temporary result artifact is writable");
    fs::rename(&temporary, path).expect("result artifact is atomically replaceable");
}

fn markdown(report: &Report) -> String {
    let mut output = String::new();
    writeln!(
        output,
        "# SSA1-C1 adaptation under experience {}",
        report.stage
    )
    .unwrap();
    writeln!(output).unwrap();
    writeln!(output, "- Classification: **{}**", report.classification).unwrap();
    writeln!(output, "- First collapse: `{}`", report.first_collapse).unwrap();
    writeln!(
        output,
        "- Frozen parent exact: `{}`",
        report.frozen_parent_exact
    )
    .unwrap();
    writeln!(output, "- Duplicate exact: `{}`", report.duplicate_exact).unwrap();
    writeln!(output, "- Development-valid: `{}`", report.passed).unwrap();
    writeln!(
        output,
        "- Definitive claim eligible: `{}`",
        report.claim_eligible
    )
    .unwrap();
    for cell in &report.cells {
        writeln!(output).unwrap();
        writeln!(output, "## Seed {}", cell.seed).unwrap();
        writeln!(output).unwrap();
        writeln!(output, "- Productive route: `{}`", cell.productive_route).unwrap();
        writeln!(output, "- P0 baseline passed: `{}`", cell.baseline.passed).unwrap();
        writeln!(
            output,
            "- P0 lock: live `{:?} -> {:?}`; changed realization `{:?}`",
            cell.baseline.before_change.live_supporters,
            cell.baseline.after_change.live_supporters,
            cell.baseline.changed_realizations
        )
        .unwrap();
        writeln!(
            output,
            "- C1 counterexperience: obtained `{}`; learned change `{}`; recovery `{}`; persistence `{}`",
            cell.counterexperience.counterexperience_obtained,
            cell.counterexperience.learning_changed,
            cell.counterexperience.recovered,
            cell.counterexperience.persisted
        )
        .unwrap();
        writeln!(
            output,
            "- C1 landscape: live `{:?} -> {:?} -> {:?}`; values `{:?} -> {:?}`; forced realization `{:?}`",
            cell.counterexperience.before.live_supporters,
            cell.counterexperience.after_counterexperience.live_supporters,
            cell.counterexperience.after_persistence.live_supporters,
            cell.counterexperience.before.value_score,
            cell.counterexperience.after_counterexperience.value_score,
            cell.counterexperience.forced_realizations
        )
        .unwrap();
        writeln!(
            output,
            "- C2 adaptation frontier: `{:?}` initial executions",
            cell.adaptation_frontier
        )
        .unwrap();
        writeln!(output, "- C2 timing cells:").unwrap();
        for point in &cell.curriculum {
            writeln!(
                output,
                "  - `{}`: boundary live `{:?}`, final live `{:?}`, realized `{:?}`, recovered `{}`",
                point.initial_exposures,
                point.at_change.live_supporters,
                point.after.live_supporters,
                point.changed_realizations,
                point.recovered
            )
            .unwrap();
        }
        writeln!(
            output,
            "- C3 timing-only B executions: `{}`; minimum early background: `{:?}`",
            cell.transient_history.timing_only_suppressed_realizations,
            cell.transient_history.minimum_early_background
        )
        .unwrap();
        writeln!(
            output,
            "- C3 richness: obtained `{}`; learned change `{}`; recovery `{}`; persistence `{}`; realization `{:?}`",
            cell.transient_history.counterexperience_obtained,
            cell.transient_history.learning_changed,
            cell.transient_history.recovered,
            cell.transient_history.persisted,
            cell.transient_history.rich_realizations
        )
        .unwrap();
        writeln!(output, "- Cell controls passed: `{}`", cell.passed).unwrap();
    }
    output
}

fn csv(report: &Report) -> String {
    let mut output = String::from(
        "stage,seed,productive_route,arm,initial_exposures,before_live_0,before_live_1,after_live_0,after_live_1,realized_0,realized_1,recovered,persisted,controls\n",
    );
    for cell in &report.cells {
        writeln!(
            output,
            "{},{},{},P0,,{},{},{},{},{},{},false,false,{}",
            report.stage,
            cell.seed,
            cell.productive_route,
            cell.baseline.before_change.live_supporters[0],
            cell.baseline.before_change.live_supporters[1],
            cell.baseline.after_change.live_supporters[0],
            cell.baseline.after_change.live_supporters[1],
            cell.baseline.changed_realizations[0],
            cell.baseline.changed_realizations[1],
            cell.baseline.passed
        )
        .unwrap();
        writeln!(
            output,
            "{},{},{},C1,,{},{},{},{},{},{},{},{},{}",
            report.stage,
            cell.seed,
            cell.productive_route,
            cell.counterexperience.before.live_supporters[0],
            cell.counterexperience.before.live_supporters[1],
            cell.counterexperience
                .after_counterexperience
                .live_supporters[0],
            cell.counterexperience
                .after_counterexperience
                .live_supporters[1],
            cell.counterexperience.forced_realizations[0],
            cell.counterexperience.forced_realizations[1],
            cell.counterexperience.recovered,
            cell.counterexperience.persisted,
            cell.counterexperience.controls_passed
        )
        .unwrap();
        for point in &cell.curriculum {
            writeln!(
                output,
                "{},{},{},C2,{},{},{},{},{},{},{},{},false,{}",
                report.stage,
                cell.seed,
                cell.productive_route,
                point.initial_exposures,
                point.at_change.live_supporters[0],
                point.at_change.live_supporters[1],
                point.after.live_supporters[0],
                point.after.live_supporters[1],
                point.changed_realizations[0],
                point.changed_realizations[1],
                point.recovered,
                point.duplicate_exact
            )
            .unwrap();
        }
        writeln!(
            output,
            "{},{},{},C3,,{},{},{},{},{},{},{},{},{}",
            report.stage,
            cell.seed,
            cell.productive_route,
            cell.transient_history.before.live_supporters[0],
            cell.transient_history.before.live_supporters[1],
            cell.transient_history.after_richness.live_supporters[0],
            cell.transient_history.after_richness.live_supporters[1],
            cell.transient_history.rich_realizations[0],
            cell.transient_history.rich_realizations[1],
            cell.transient_history.recovered,
            cell.transient_history.persisted,
            cell.transient_history.controls_passed
        )
        .unwrap();
    }
    output
}

fn main() {
    let argument = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "--probe".to_string());
    let report = match argument.as_str() {
        "--probe" => run_probe(),
        "--micro" => run_micro(),
        "--gate" => run_gate(),
        "--definitive" => {
            eprintln!("definitive execution is not authorized for SSA1-C1 development");
            std::process::exit(2);
        }
        _ => {
            eprintln!("expected --probe, --micro, --gate, or --definitive");
            std::process::exit(2);
        }
    };
    let stem = format!(
        "results/ssa1_c1_adaptation_under_experience_{}",
        report.stage.to_ascii_lowercase()
    );
    let markdown = markdown(&report);
    write_atomic(Path::new(&format!("{stem}.md")), &markdown);
    write_atomic(Path::new(&format!("{stem}.csv")), &csv(&report));
    print!("{markdown}");
    if !report.passed {
        std::process::exit(1);
    }
}
