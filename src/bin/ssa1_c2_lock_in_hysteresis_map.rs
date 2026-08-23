use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use organism_v0::ssa1_c2_lock_in_hysteresis_map::{run_gate, run_micro, run_probe, Report};

fn write_atomic(path: &Path, contents: &str) {
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, contents).expect("temporary result is writable");
    fs::rename(&temporary, path).expect("result is atomically replaceable");
}

fn markdown(report: &Report) -> String {
    let mut output = String::new();
    writeln!(
        output,
        "# SSA1-C2 lock-in / hysteresis map {}",
        report.stage
    )
    .unwrap();
    writeln!(output).unwrap();
    writeln!(output, "- Classification: **{}**", report.classification).unwrap();
    writeln!(
        output,
        "- B-only subclassification: **{}**",
        report.b_only_subclassification
    )
    .unwrap();
    writeln!(
        output,
        "- First non-responsive edge: `{}`",
        report.first_nonresponsive_edge
    )
    .unwrap();
    writeln!(output, "- Source invariant: `{}`", report.source_invariant).unwrap();
    writeln!(
        output,
        "- Frozen count capacity respected: `{}`",
        report.count_capacity_safe
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
            "- Early productive route: `{}`",
            cell.productive_route
        )
        .unwrap();
        writeln!(output, "- Finite barrier: `{:?}`", cell.finite_barrier).unwrap();
        writeln!(
            output,
            "- B-only absorbing invariant: `{}`",
            cell.b_only_absorbing_invariant
        )
        .unwrap();
        writeln!(
            output,
            "- Forgetting-only reopening: `{}`",
            cell.forgetting_only
        )
        .unwrap();
        writeln!(output, "- Duplicate exact: `{}`", cell.duplicate_exact).unwrap();
        writeln!(output, "- Controls passed: `{}`", cell.controls_passed).unwrap();
        writeln!(output).unwrap();
        writeln!(output, "### Maturation/evidence map").unwrap();
        writeln!(output).unwrap();
        for trajectory in &cell.trajectories {
            let a = cell.productive_route;
            let b = 1 - a;
            let e0 = trajectory.b_only.last().unwrap();
            let e1 = trajectory.paired.last().unwrap();
            let first_e1_reversal = trajectory
                .paired
                .iter()
                .find(|point| point.budget > 0 && point.reversed)
                .map(|point| point.budget);
            writeln!(
                output,
                "- H={}: boundary live `{:?}`, M6 eligible `{:?}`, margins `[{},{}]`; E0@{} live `{:?}`, B obs `{}`, B M5 `+{}/-{}`, abstentions `{}`, edge `{}`, reversed `{}`; E1 first reversal `{:?}`, final live `{:?}`, A/B evidence `[{},{}]`",
                trajectory.maturation,
                trajectory.boundary_landscape.live_supporters,
                trajectory.boundary_audit.routes.each_ref().map(|route| route.evidence_eligible),
                trajectory.boundary_audit.routes[a].evidence_margin,
                trajectory.boundary_audit.routes[b].evidence_margin,
                e0.budget,
                e0.landscape.live_supporters,
                e0.audit.routes[b].evidence_observations,
                e0.audit.routes[b].m5_support,
                e0.audit.routes[b].m5_rejection,
                e0.audit.abstentions,
                e0.first_nonresponsive_edge,
                e0.reversed,
                first_e1_reversal,
                e1.landscape.live_supporters,
                e1.audit.routes[a].evidence_observations,
                e1.audit.routes[b].evidence_observations
            )
            .unwrap();
        }
        writeln!(output).unwrap();
        writeln!(output, "### Physical-support map at H=192").unwrap();
        writeln!(output).unwrap();
        for point in &cell.support_map {
            let b = 1 - cell.productive_route;
            writeln!(
                output,
                "- S={}: B executions `{}`, B observations `{}`, live `{:?}`, edge `{}`",
                point.extra_early_support,
                point.checkpoint.realizations[b],
                point.checkpoint.audit.routes[b].evidence_observations,
                point.checkpoint.landscape.live_supporters,
                point.checkpoint.first_nonresponsive_edge
            )
            .unwrap();
        }
        writeln!(output).unwrap();
        writeln!(output, "### Disuse map at H=192").unwrap();
        writeln!(output).unwrap();
        for point in &cell.disuse_map {
            writeln!(
                output,
                "- T={}: after-pressure live `{:?}`, M6 evidence `[{},{}]`, final live `{:?}`, forgetting-only `{}`",
                point.pressure_events,
                point.landscape_after_pressure.live_supporters,
                point.after_pressure.routes[0].evidence_observations,
                point.after_pressure.routes[1].evidence_observations,
                point.checkpoint.landscape.live_supporters,
                point.forgetting_only_reopening
            )
            .unwrap();
        }
    }
    output
}

fn csv(report: &Report) -> String {
    let mut output = String::from(
        "stage,seed,productive_route,maturation,schedule,budget,realized_0,realized_1,returned_0,returned_1,live_0,live_1,m6_obs_0,m6_obs_1,m6_support_0,m6_support_1,m6_margin_0,m6_margin_1,m6_eligible_0,m6_eligible_1,m5_support_0,m5_support_1,m5_rejection_0,m5_rejection_1,m5_score_0,m5_score_1,abstentions,applications,independent_0,independent_1,edge,reversed\n",
    );
    for cell in &report.cells {
        for trajectory in &cell.trajectories {
            for point in trajectory.b_only.iter().chain(trajectory.paired.iter()) {
                writeln!(
                    output,
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                    report.stage,
                    cell.seed,
                    cell.productive_route,
                    trajectory.maturation,
                    point.schedule.name(),
                    point.budget,
                    point.realizations[0],
                    point.realizations[1],
                    point.consequences_returned[0],
                    point.consequences_returned[1],
                    point.landscape.live_supporters[0],
                    point.landscape.live_supporters[1],
                    point.audit.routes[0].evidence_observations,
                    point.audit.routes[1].evidence_observations,
                    point.audit.routes[0].evidence_support,
                    point.audit.routes[1].evidence_support,
                    point.audit.routes[0].evidence_margin,
                    point.audit.routes[1].evidence_margin,
                    point.audit.routes[0].evidence_eligible,
                    point.audit.routes[1].evidence_eligible,
                    point.audit.routes[0].m5_support,
                    point.audit.routes[1].m5_support,
                    point.audit.routes[0].m5_rejection,
                    point.audit.routes[1].m5_rejection,
                    point.audit.routes[0].m5_score,
                    point.audit.routes[1].m5_score,
                    point.audit.abstentions,
                    point.audit.applications,
                    point.independent_realizations[0],
                    point.independent_realizations[1],
                    point.first_nonresponsive_edge,
                    point.reversed
                )
                .unwrap();
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
            eprintln!("definitive execution is not authorized for SSA1-C2 development");
            std::process::exit(2);
        }
        _ => {
            eprintln!("expected --probe, --micro, --gate, or --definitive");
            std::process::exit(2);
        }
    };
    let stem = format!(
        "results/ssa1_c2_lock_in_hysteresis_map_{}",
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
