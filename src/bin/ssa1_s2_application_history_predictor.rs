use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use organism_v0::ssa1_s2_application_history_predictor::{
    run_gate, run_micro, run_probe, PredictorId, Report,
};

fn write_atomic(path: &Path, contents: &str) {
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, contents).expect("temporary result is writable");
    fs::rename(&temporary, path).expect("result is atomically replaceable");
}

fn markdown(report: &Report) -> String {
    let mut output = String::new();
    writeln!(
        output,
        "# SSA1-S2 application-history predictor {}",
        report.stage
    )
    .unwrap();
    writeln!(output).unwrap();
    writeln!(output, "- Classification: **{}**", report.classification).unwrap();
    writeln!(
        output,
        "- Selected predictor: `{}`",
        report.selected_predictor.map_or("NONE", PredictorId::name)
    )
    .unwrap();
    writeln!(output, "- Basin diversity: `{}`", report.basin_diversity).unwrap();
    writeln!(
        output,
        "- Trace attribution exact: `{}`",
        report.trace_attribution_exact
    )
    .unwrap();
    writeln!(
        output,
        "- Frozen parent exact: `{}`",
        report.frozen_parent_exact
    )
    .unwrap();
    writeln!(output, "- Development-valid: `{}`", report.passed).unwrap();
    writeln!(
        output,
        "- Definitive claim eligible: `{}`",
        report.claim_eligible
    )
    .unwrap();
    writeln!(output).unwrap();
    writeln!(output, "## Predictor library").unwrap();
    writeln!(output).unwrap();
    writeln!(
        output,
        "| predictor | discovery accuracy | held-out accuracy | held-out coverage | minimum cell | latest episode | qualifies |"
    )
    .unwrap();
    writeln!(output, "|---|---:|---:|---:|---:|---:|---|").unwrap();
    for metrics in &report.predictors {
        writeln!(
            output,
            "| {} | {:.2}% | {:.2}% | {:.2}% | {:.2}% | {} | {} |",
            metrics.id.name(),
            metrics.discovery.accuracy_basis_points as f64 / 100.0,
            metrics.held_out.accuracy_basis_points as f64 / 100.0,
            metrics.held_out.coverage_basis_points as f64 / 100.0,
            metrics.minimum_cell_accuracy_basis_points as f64 / 100.0,
            metrics.held_out.latest_available_episode,
            metrics.qualifies,
        )
        .unwrap();
    }
    for cell in &report.cells {
        writeln!(output).unwrap();
        writeln!(output, "## Seed {}", cell.seed).unwrap();
        writeln!(output).unwrap();
        writeln!(output, "- Incumbent side: `{}`", cell.incumbent_side).unwrap();
        writeln!(output, "- Side -> route: `{:?}`", cell.route_at_side).unwrap();
        writeln!(output, "- Stale blocked: `{}`", cell.stale_blocked).unwrap();
        writeln!(output, "- Post-closure inert: `{}`", cell.postclosure_inert).unwrap();
        writeln!(output, "- Observation inert: `{}`", cell.observation_inert).unwrap();
        writeln!(output, "- Controls passed: `{}`", cell.controls_passed).unwrap();
        let mut by_ratio: BTreeMap<_, [usize; 4]> = BTreeMap::new();
        for trajectory in &cell.trajectories {
            let index = match trajectory.final_class.name() {
                "INCUMBENT_LOCK" => 0,
                "MIXED" => 1,
                "ALTERNATIVE" => 2,
                _ => 3,
            };
            by_ratio.entry(trajectory.descriptor.ratio).or_default()[index] += 1;
        }
        writeln!(output).unwrap();
        writeln!(output, "### Basin counts by equal-multiset ratio").unwrap();
        writeln!(output).unwrap();
        for (ratio, counts) in by_ratio {
            writeln!(
                output,
                "- B:A `{}` -> incumbent `{}`, mixed `{}`, alternative `{}`, subthreshold `{}`",
                ratio.name(),
                counts[0],
                counts[1],
                counts[2],
                counts[3]
            )
            .unwrap();
        }
    }
    output
}

fn trajectory_csv(report: &Report) -> String {
    let mut output = String::from(
        "stage,seed,stride,offset,ratio,discovery,incumbent_side,incumbent_route,alternative_route,scheduled_incumbent,scheduled_alternative,executed_incumbent,executed_alternative,returned_incumbent,returned_alternative,effective_applications,neutral_applications,attribution_failures,first_direction,first_direction_episode,first_4_balance,first_4_episode,first_8_balance,first_8_episode,first_16_balance,first_16_episode,first_opposing_gap,first_opposing_episode,longest_90_direction,ninetieth_application_episode,gap_after_episode_90,alternative_threshold_episode,incumbent_deallocation_episode,first_8_code,final_m5_incumbent,final_m5_alternative,final_live_incumbent,final_live_alternative,final_class,schedule_exact,duplicate_exact,trace_attributed\n",
    );
    for cell in &report.cells {
        for trajectory in &cell.trajectories {
            let incumbent_route = trajectory.route_at_side[trajectory.incumbent_side];
            let alternative_route = trajectory.route_at_side[1 - trajectory.incumbent_side];
            let summary = &trajectory.summary;
            writeln!(
                output,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{},{:?},{:?},{:?},{},{},{},{},{},{},{},{}",
                report.stage,
                cell.seed,
                trajectory.descriptor.stride,
                trajectory.descriptor.offset,
                trajectory.descriptor.ratio.name(),
                trajectory.descriptor.discovery,
                trajectory.incumbent_side,
                incumbent_route,
                alternative_route,
                trajectory.scheduled[0],
                trajectory.scheduled[1],
                trajectory.executions[0],
                trajectory.executions[1],
                trajectory.consequences[0],
                trajectory.consequences[1],
                summary.effective_applications,
                summary.neutral_applications,
                summary.attribution_failures,
                summary.first_direction,
                summary.first_direction_episode,
                summary.first_4_balance,
                summary.first_4_episode,
                summary.first_8_balance,
                summary.first_8_episode,
                summary.first_16_balance,
                summary.first_16_episode,
                summary.first_opposing_gap,
                summary.first_opposing_episode,
                summary.longest_90_direction,
                summary.ninetieth_application_episode,
                summary.gap_after_episode_90,
                summary.alternative_threshold_episode,
                summary.incumbent_deallocation_episode,
                summary.first_8_code.unwrap_or(0),
                trajectory.final_audit.routes[incumbent_route].m5_score,
                trajectory.final_audit.routes[alternative_route].m5_score,
                trajectory.final_landscape.live_supporters[incumbent_route],
                trajectory.final_landscape.live_supporters[alternative_route],
                trajectory.final_class.name(),
                trajectory.schedule_exact,
                trajectory.duplicate_exact,
                trajectory.trace_attributed,
            )
            .unwrap();
        }
    }
    output
}

fn predictor_csv(report: &Report) -> String {
    let mut output = String::from(
        "stage,predictor,discovery_total,discovery_predicted,discovery_correct,discovery_accuracy_bp,discovery_coverage_bp,heldout_total,heldout_predicted,heldout_correct,heldout_accuracy_bp,heldout_coverage_bp,minimum_cell_accuracy_bp,latest_available_episode,qualifies\n",
    );
    for metrics in &report.predictors {
        writeln!(
            output,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            report.stage,
            metrics.id.name(),
            metrics.discovery.total,
            metrics.discovery.predicted,
            metrics.discovery.correct,
            metrics.discovery.accuracy_basis_points,
            metrics.discovery.coverage_basis_points,
            metrics.held_out.total,
            metrics.held_out.predicted,
            metrics.held_out.correct,
            metrics.held_out.accuracy_basis_points,
            metrics.held_out.coverage_basis_points,
            metrics.minimum_cell_accuracy_basis_points,
            metrics.held_out.latest_available_episode,
            metrics.qualifies,
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
            eprintln!("definitive execution is not authorized for SSA1-S2 development");
            std::process::exit(2);
        }
        _ => {
            eprintln!("expected --probe, --micro, --gate, or --definitive");
            std::process::exit(2);
        }
    };
    let stem = format!(
        "results/ssa1_s2_application_history_predictor_{}",
        report.stage.to_ascii_lowercase()
    );
    let markdown = markdown(&report);
    write_atomic(Path::new(&format!("{stem}.md")), &markdown);
    write_atomic(Path::new(&format!("{stem}.csv")), &trajectory_csv(&report));
    write_atomic(
        Path::new(&format!("{stem}_predictors.csv")),
        &predictor_csv(&report),
    );
    print!("{markdown}");
    if !report.passed {
        std::process::exit(1);
    }
}
