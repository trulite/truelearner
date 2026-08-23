use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use organism_v0::ssa1_s3_structural_commitment_causality::{
    run_s3_gate, run_s3_micro, run_s3_probe, S3Report,
};

fn write_atomic(path: &Path, contents: &str) {
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, contents).expect("temporary S3 result is writable");
    fs::rename(&temporary, path).expect("S3 result is atomically replaceable");
}

fn markdown(report: &S3Report) -> String {
    let mut output = String::new();
    writeln!(
        output,
        "# SSA1-S3 structural-commitment causality {}",
        report.stage
    )
    .unwrap();
    writeln!(output).unwrap();
    writeln!(output, "- Classification: **{}**", report.classification).unwrap();
    writeln!(
        output,
        "- Threshold-causal cells: `{}/{}`",
        report.threshold_causal_cells,
        report.cells.len()
    )
    .unwrap();
    writeln!(
        output,
        "- Deallocation-causal cells: `{}/{}`",
        report.deallocation_causal_cells,
        report.cells.len()
    )
    .unwrap();
    writeln!(
        output,
        "- Post-commitment inert cells: `{}/{}`",
        report.postcommit_inert_cells,
        report.cells.len()
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
        writeln!(
            output,
            "## Cell {} / {}:{}:{}",
            cell.seed,
            cell.descriptor.ratio.name(),
            cell.descriptor.stride,
            cell.descriptor.offset
        )
        .unwrap();
        writeln!(output).unwrap();
        writeln!(
            output,
            "- Side -> route: `{:?}`; incumbent side `{}`",
            cell.route_at_side, cell.incumbent_side
        )
        .unwrap();
        writeln!(
            output,
            "- Reference transitions: B threshold `{}`, A deallocation `{}`",
            cell.reference_threshold_episode, cell.reference_deallocation_episode
        )
        .unwrap();
        writeln!(output, "- Prefix exact: `{}`", cell.prefix_exact).unwrap();
        writeln!(output, "- T1 threshold causal: `{}`", cell.threshold_causal).unwrap();
        writeln!(
            output,
            "- T2 deallocation causal: `{}`",
            cell.deallocation_causal
        )
        .unwrap();
        writeln!(
            output,
            "- T3 post-commitment inert: `{}`",
            cell.postcommit_inert
        )
        .unwrap();
        writeln!(output, "- Controls passed: `{}`", cell.controls_passed).unwrap();
        writeln!(output).unwrap();
        writeln!(
            output,
            "| arm | episode | before | after | recurrences | transition changed | final | class |"
        )
        .unwrap();
        writeln!(output, "|---|---:|---|---|---:|---|---|---|").unwrap();
        for arm in [
            &cell.reference,
            &cell.threshold_block,
            &cell.deallocation_protection,
            &cell.post_threshold_block,
            &cell.post_deallocation_recurrence,
        ] {
            writeln!(
                output,
                "| {} | {} | `{:?}` | `{:?}` | {} | {} | `{:?}` | {} |",
                arm.name,
                arm.intervention_episode,
                arm.live_before,
                arm.live_after,
                arm.recurrences_delivered,
                arm.target_transition_changed,
                arm.final_live,
                arm.final_class
            )
            .unwrap();
        }
    }
    output
}

fn csv(report: &S3Report) -> String {
    let mut output = String::from("stage,seed,ratio,stride,offset,incumbent_side,route_at_side,threshold_episode,deallocation_episode,arm,intervention_episode,live_before_incumbent,live_before_alternative,live_after_incumbent,live_after_alternative,recurrences,target_transition_changed,final_live_incumbent,final_live_alternative,final_class,duplicate_exact,schedule_exact,threshold_causal,deallocation_causal,postcommit_inert,controls_passed\n");
    for cell in &report.cells {
        for arm in [
            &cell.reference,
            &cell.threshold_block,
            &cell.deallocation_protection,
            &cell.post_threshold_block,
            &cell.post_deallocation_recurrence,
        ] {
            writeln!(
                output,
                "{},{},{},{},{},{},{}:{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                report.stage,
                cell.seed,
                cell.descriptor.ratio.name(),
                cell.descriptor.stride,
                cell.descriptor.offset,
                cell.incumbent_side,
                cell.route_at_side[0],
                cell.route_at_side[1],
                cell.reference_threshold_episode,
                cell.reference_deallocation_episode,
                arm.name,
                arm.intervention_episode,
                arm.live_before[0],
                arm.live_before[1],
                arm.live_after[0],
                arm.live_after[1],
                arm.recurrences_delivered,
                arm.target_transition_changed,
                arm.final_live[0],
                arm.final_live[1],
                arm.final_class,
                arm.duplicate_exact,
                arm.schedule_exact,
                cell.threshold_causal,
                cell.deallocation_causal,
                cell.postcommit_inert,
                cell.controls_passed
            )
            .unwrap();
        }
    }
    output
}

fn main() {
    let argument = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "--probe".to_string());
    let report = match argument.as_str() {
        "--probe" => run_s3_probe(),
        "--micro" => run_s3_micro(),
        "--gate" => run_s3_gate(),
        "--definitive" => {
            eprintln!("definitive execution is not authorized for SSA1-S3 development");
            std::process::exit(2);
        }
        _ => {
            eprintln!("expected --probe, --micro, --gate, or --definitive");
            std::process::exit(2);
        }
    };
    fs::create_dir_all("results").expect("results directory is writable");
    let stem = format!(
        "ssa1_s3_structural_commitment_causality_{}_v2",
        report.stage.to_ascii_lowercase()
    );
    write_atomic(Path::new(&format!("results/{stem}.md")), &markdown(&report));
    write_atomic(Path::new(&format!("results/{stem}.csv")), &csv(&report));
    println!("{}", markdown(&report));
    if !report.passed {
        std::process::exit(1);
    }
}
