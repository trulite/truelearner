use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use organism_v0::ssa1_s_exposure_bias_map::{run_gate, run_micro, run_probe, Report};

fn write_atomic(path: &Path, contents: &str) {
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, contents).expect("temporary result is writable");
    fs::rename(&temporary, path).expect("result is atomically replaceable");
}

fn markdown(report: &Report) -> String {
    let mut output = String::new();
    writeln!(output, "# SSA1-S exposure bias map {}", report.stage).unwrap();
    writeln!(output).unwrap();
    writeln!(output, "- Classification: **{}**", report.classification).unwrap();
    writeln!(
        output,
        "- Environmental exposure varied: `{}`",
        report.exposure_varied
    )
    .unwrap();
    writeln!(
        output,
        "- Phase map monotonic: `{}`",
        report.phase_map_monotonic
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
    for cell in &report.cells {
        writeln!(output).unwrap();
        writeln!(output, "## Seed {}", cell.seed).unwrap();
        writeln!(output).unwrap();
        writeln!(
            output,
            "- Incumbent physical side: `{}`",
            cell.incumbent_side
        )
        .unwrap();
        writeln!(output, "- Side -> route: `{:?}`", cell.route_at_side).unwrap();
        writeln!(
            output,
            "- Reversal thresholds: `{:?}`",
            cell.reversal_thresholds
        )
        .unwrap();
        writeln!(
            output,
            "- Exposure monotonic: `{}`",
            cell.exposure_monotonic
        )
        .unwrap();
        writeln!(
            output,
            "- Allocation monotonic: `{}`",
            cell.allocation_monotonic
        )
        .unwrap();
        writeln!(
            output,
            "- Maturity monotonic: `{}`",
            cell.maturity_monotonic
        )
        .unwrap();
        writeln!(output, "- Stale blocked: `{}`", cell.stale_blocked).unwrap();
        writeln!(output, "- Post-closure inert: `{}`", cell.postclosure_inert).unwrap();
        writeln!(
            output,
            "- Anti-adaptation: `{}`",
            cell.anti_adaptation_audit
        )
        .unwrap();
        writeln!(output, "- Controls passed: `{}`", cell.controls_passed).unwrap();
        writeln!(output).unwrap();
        writeln!(output, "### Opportunity -> evidence -> allocation").unwrap();
        writeln!(output).unwrap();
        for trajectory in &cell.trajectories {
            let point = trajectory.checkpoints.last().unwrap();
            let incumbent_route = trajectory.route_at_side[trajectory.incumbent_side];
            let alternative_route = trajectory.route_at_side[1 - trajectory.incumbent_side];
            writeln!(
                output,
                "- H={} B:A={}: scheduled `{:?}`, executions `{:?}`, M6 observations `[{},{}]`, M6 margins `[{},{}]`, M5 scores `[{},{}]`, live `{:?}`, class `{}`",
                trajectory.maturity,
                trajectory.ratio.name(),
                point.scheduled,
                point.executions,
                point.audit.routes[incumbent_route].evidence_observations,
                point.audit.routes[alternative_route].evidence_observations,
                point.audit.routes[incumbent_route].evidence_margin,
                point.audit.routes[alternative_route].evidence_margin,
                point.audit.routes[incumbent_route].m5_score,
                point.audit.routes[alternative_route].m5_score,
                point.landscape.live_supporters,
                point.class.name(),
            )
            .unwrap();
        }
        writeln!(output).unwrap();
        writeln!(output, "### Equal-consequence controls").unwrap();
        writeln!(output).unwrap();
        for control in &cell.equal_controls {
            writeln!(
                output,
                "- H={}: executions `{:?}`, live `{:?}`, class `{}`, abstentions `{}`",
                control.maturity,
                control.final_checkpoint.executions,
                control.final_checkpoint.landscape.live_supporters,
                control.final_checkpoint.class.name(),
                control.final_checkpoint.audit.abstentions,
            )
            .unwrap();
        }
    }
    output
}

fn csv(report: &Report) -> String {
    let mut output = String::from(
        "stage,seed,control,maturity,ratio,phase_offset,budget,incumbent_side,incumbent_route,alternative_route,scheduled_incumbent,scheduled_alternative,executed_incumbent,executed_alternative,returned_incumbent,returned_alternative,unresolved,m6_obs_incumbent,m6_obs_alternative,m6_support_incumbent,m6_support_alternative,m6_margin_incumbent,m6_margin_alternative,m6_eligible_incumbent,m6_eligible_alternative,m5_support_incumbent,m5_support_alternative,m5_rejection_incumbent,m5_rejection_alternative,m5_score_incumbent,m5_score_alternative,live_incumbent,live_alternative,abstentions,applications,independent_side0,independent_side1,class,schedule_exact,exposure_transferred,duplicate_exact\n",
    );
    for cell in &report.cells {
        for (control, trajectories) in [
            ("baseline", &cell.trajectories),
            ("phase", &cell.phase_controls),
        ] {
            for trajectory in trajectories {
                let incumbent_route = trajectory.route_at_side[trajectory.incumbent_side];
                let alternative_route = trajectory.route_at_side[1 - trajectory.incumbent_side];
                for point in &trajectory.checkpoints {
                    writeln!(
                        output,
                        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        report.stage,
                        cell.seed,
                        control,
                        trajectory.maturity,
                        trajectory.ratio.name(),
                        trajectory.phase_offset,
                        point.budget,
                        trajectory.incumbent_side,
                        incumbent_route,
                        alternative_route,
                        point.scheduled[0],
                        point.scheduled[1],
                        point.executions[0],
                        point.executions[1],
                        point.consequences[0],
                        point.consequences[1],
                        point.unresolved,
                        point.audit.routes[incumbent_route].evidence_observations,
                        point.audit.routes[alternative_route].evidence_observations,
                        point.audit.routes[incumbent_route].evidence_support,
                        point.audit.routes[alternative_route].evidence_support,
                        point.audit.routes[incumbent_route].evidence_margin,
                        point.audit.routes[alternative_route].evidence_margin,
                        point.audit.routes[incumbent_route].evidence_eligible,
                        point.audit.routes[alternative_route].evidence_eligible,
                        point.audit.routes[incumbent_route].m5_support,
                        point.audit.routes[alternative_route].m5_support,
                        point.audit.routes[incumbent_route].m5_rejection,
                        point.audit.routes[alternative_route].m5_rejection,
                        point.audit.routes[incumbent_route].m5_score,
                        point.audit.routes[alternative_route].m5_score,
                        point.landscape.live_supporters[incumbent_route],
                        point.landscape.live_supporters[alternative_route],
                        point.audit.abstentions,
                        point.audit.applications,
                        point.independent_realizations[0],
                        point.independent_realizations[1],
                        point.class.name(),
                        trajectory.schedule_exact,
                        trajectory.exposure_transferred,
                        trajectory.duplicate_exact,
                    )
                    .unwrap();
                }
            }
        }
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
            eprintln!("definitive execution is not authorized for SSA1-S development");
            std::process::exit(2);
        }
        _ => {
            eprintln!("expected --probe, --micro, --gate, or --definitive");
            std::process::exit(2);
        }
    };
    let stem = format!(
        "results/ssa1_s_exposure_bias_map_{}",
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
