use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use organism_v0::ssa1_r_rich_changing_world::{run_gate, run_micro, run_probe, Report};

fn write_atomic(path: &Path, contents: &str) {
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, contents).expect("temporary result is writable");
    fs::rename(&temporary, path).expect("result is atomically replaceable");
}

fn markdown(report: &Report) -> String {
    let mut output = String::new();
    writeln!(output, "# SSA1-R rich changing world {}", report.stage).unwrap();
    writeln!(output).unwrap();
    writeln!(output, "- Classification: **{}**", report.classification).unwrap();
    writeln!(output, "- Best common dwell: `{:?}`", report.best_dwell).unwrap();
    writeln!(
        output,
        "- Frozen parent exact: `{}`",
        report.frozen_parent_exact
    )
    .unwrap();
    writeln!(
        output,
        "- Anti-pairing audit: `{}`",
        report.anti_pairing_audit
    )
    .unwrap();
    writeln!(
        output,
        "- Count capacity safe: `{}`",
        report.count_capacity_safe
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
            "- Physical-side route map: `{:?}`",
            cell.route_at_side
        )
        .unwrap();
        writeln!(output, "- R0 passed: `{}`", cell.stationary_winner.passed).unwrap();
        writeln!(output, "- R1 passed: `{}`", cell.multi_useful.passed).unwrap();
        writeln!(output, "- Stale route blocked: `{}`", cell.stale_control).unwrap();
        writeln!(output, "- Duplicate exact: `{}`", cell.duplicate_exact).unwrap();
        writeln!(output, "- Cell functional conjunction: `{}`", cell.passed).unwrap();
        for world in [&cell.stationary_winner, &cell.multi_useful] {
            let phase = &world.outcome;
            writeln!(
                output,
                "- {}: executions `{:?}`, consequences `{:?}`, live `{:?}`, independent `{:?}`, M6 eligible `{:?}`, M5 scores `{:?}`",
                world.name,
                phase.executions,
                phase.consequences,
                phase.landscape.live_supporters,
                phase.independent_realizations,
                phase.end_audit.routes.each_ref().map(|route| route.evidence_eligible),
                phase.end_audit.routes.each_ref().map(|route| route.m5_score),
            )
            .unwrap();
        }
        writeln!(output).unwrap();
        writeln!(output, "### R2 dwell worlds").unwrap();
        writeln!(output).unwrap();
        for world in &cell.changing {
            writeln!(
                output,
                "- dwell `{}`: tracked `{}/3`, full `{}`, comparative `{}`",
                world.dwell, world.tracked_phases, world.fully_tracked, world.comparative_evidence
            )
            .unwrap();
            for phase in &world.phases {
                writeln!(
                    output,
                    "  - phase {} stable `{:?}`: executions `{:?}`, consequences `{:?}`, live `{:?}`, independent `{:?}`, M6 eligible `{:?}`, M6 margins `{:?}`, M5 scores `{:?}`, correct `{}`",
                    phase.phase,
                    phase.stable_side,
                    phase.executions,
                    phase.consequences,
                    phase.landscape.live_supporters,
                    phase.independent_realizations,
                    phase.end_audit.routes.each_ref().map(|route| route.evidence_eligible),
                    phase.end_audit.routes.each_ref().map(|route| route.evidence_margin),
                    phase.end_audit.routes.each_ref().map(|route| route.m5_score),
                    phase.dominance_correct,
                )
                .unwrap();
            }
        }
        writeln!(output).unwrap();
        writeln!(output, "### Causal controls").unwrap();
        writeln!(output).unwrap();
        for world in [
            &cell.no_field_control,
            &cell.postclosure_control,
            &cell.clock_phase_control,
        ] {
            writeln!(
                output,
                "- {}: tracked `{}/3`, full `{}`, comparative `{}`, physical controls `{}`",
                world.name,
                world.tracked_phases,
                world.fully_tracked,
                world.comparative_evidence,
                world.passed_controls,
            )
            .unwrap();
        }
    }
    output
}

fn csv(report: &Report) -> String {
    let mut output = String::from(
        "stage,seed,world,dwell,phase,stable_side,exec_side0,exec_side1,consequence_side0,consequence_side1,live_route0,live_route1,independent_side0,independent_side1,m6_obs_route0,m6_obs_route1,m6_margin_route0,m6_margin_route1,m6_eligible_route0,m6_eligible_route1,m5_score_route0,m5_score_route1,abstentions,applications,field_balanced,dominance_correct\n",
    );
    for cell in &report.cells {
        let worlds = cell.changing.iter().chain([
            &cell.no_field_control,
            &cell.postclosure_control,
            &cell.clock_phase_control,
        ]);
        for world in worlds {
            for phase in &world.phases {
                writeln!(
                    output,
                    "{},{},{},{},{},{:?},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                    report.stage,
                    cell.seed,
                    world.name,
                    world.dwell,
                    phase.phase,
                    phase.stable_side,
                    phase.executions[0],
                    phase.executions[1],
                    phase.consequences[0],
                    phase.consequences[1],
                    phase.landscape.live_supporters[0],
                    phase.landscape.live_supporters[1],
                    phase.independent_realizations[0],
                    phase.independent_realizations[1],
                    phase.end_audit.routes[0].evidence_observations,
                    phase.end_audit.routes[1].evidence_observations,
                    phase.end_audit.routes[0].evidence_margin,
                    phase.end_audit.routes[1].evidence_margin,
                    phase.end_audit.routes[0].evidence_eligible,
                    phase.end_audit.routes[1].evidence_eligible,
                    phase.end_audit.routes[0].m5_score,
                    phase.end_audit.routes[1].m5_score,
                    phase.end_audit.abstentions,
                    phase.end_audit.applications,
                    phase.field_balanced,
                    phase.dominance_correct,
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
            eprintln!("definitive execution is not authorized for SSA1-R development");
            std::process::exit(2);
        }
        _ => {
            eprintln!("expected --probe, --micro, --gate, or --definitive");
            std::process::exit(2);
        }
    };
    let stem = format!(
        "results/ssa1_r_rich_changing_world_{}",
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
