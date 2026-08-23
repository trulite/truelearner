use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use organism_v0::ssa2_p_preserved_affordance_generativity::{
    run_ssa2_p_gate, run_ssa2_p_micro, run_ssa2_p_probe, Ssa2pReport,
};

fn write_atomic(path: &Path, contents: &str) {
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, contents).expect("temporary SSA2-P result is writable");
    fs::rename(&temporary, path).expect("SSA2-P result is atomically replaceable");
}

fn side_word(sides: &[usize]) -> String {
    sides
        .iter()
        .map(|side| char::from(b'0' + *side as u8))
        .collect()
}

fn state_word(states: &[usize]) -> String {
    states
        .iter()
        .map(|state| char::from(b'0' + *state as u8))
        .collect()
}

fn markdown(report: &Ssa2pReport) -> String {
    let mut output = String::new();
    writeln!(
        output,
        "# SSA2-P preserved-affordance generativity {}",
        report.stage
    )
    .unwrap();
    writeln!(output).unwrap();
    writeln!(output, "- Classification: **{}**", report.classification).unwrap();
    writeln!(output, "- Depth: `{}`", report.depth).unwrap();
    writeln!(output, "- Histories per cell: `{}`", report.histories).unwrap();
    writeln!(
        output,
        "- Valid trajectories: `{}/{}`",
        report.valid_trajectories, report.total_trajectories
    )
    .unwrap();
    writeln!(
        output,
        "- Minimum distinct trajectories: `{}`",
        report.minimum_distinct_trajectories
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
        writeln!(output, "## Cell {}", cell.seed).unwrap();
        writeln!(output).unwrap();
        writeln!(
            output,
            "- Learned live supporters: `{:?}`",
            cell.learned_live
        )
        .unwrap();
        writeln!(
            output,
            "- Distinct trajectory / trace fingerprints: `{}/{}`",
            cell.distinct_trajectories, cell.distinct_trace_fingerprints
        )
        .unwrap();
        writeln!(
            output,
            "- Both sides at every layer: `{}`",
            cell.both_sides_every_layer
        )
        .unwrap();
        writeln!(output, "- Permanent exact: `{}`", cell.permanent_exact).unwrap();
        writeln!(
            output,
            "- Collapsed control: `{:?}`, distinct `{}`, passed `{}`",
            cell.collapsed_live, cell.collapsed_distinct_trajectories, cell.collapsed_control
        )
        .unwrap();
        writeln!(output, "- Blocked route: `{}`", cell.blocked_control).unwrap();
        writeln!(
            output,
            "- Broken transition: `{}`",
            cell.broken_transition_control
        )
        .unwrap();
        writeln!(
            output,
            "- Handle permutation / duplicate handle: `{}/{}`",
            cell.handle_permutation_control, cell.duplicate_handle_control
        )
        .unwrap();
        writeln!(
            output,
            "- Exact no-transient replay: `{}`",
            cell.no_transient_control
        )
        .unwrap();
        writeln!(output, "- Controls passed: `{}`", cell.controls_passed).unwrap();
        writeln!(output, "- Cell passed: `{}`", cell.passed).unwrap();
        writeln!(output).unwrap();
        writeln!(output, "### First four trajectories").unwrap();
        writeln!(output).unwrap();
        for trajectory in cell.trajectories.iter().take(4) {
            writeln!(
                output,
                "- h=`{}` states=`{}` sides=`{}` trace=`{}` work=`{}`",
                trajectory.history,
                state_word(&trajectory.states),
                side_word(&trajectory.physical_sides),
                trajectory.trace_fingerprint,
                trajectory.work
            )
            .unwrap();
        }
    }
    output
}

fn csv(report: &Ssa2pReport) -> String {
    let mut output = String::from(
        "stage,seed,depth,history,states,physical_sides,handles,complete,structurally_valid,naturally_quiescent,duplicate_exact,one_propagation,permanent_fingerprint,start_fingerprint,trace_fingerprint,end_fingerprint,work,cell_distinct,cell_trace_distinct,cell_passed\n",
    );
    for cell in &report.cells {
        for trajectory in &cell.trajectories {
            writeln!(
                output,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                report.stage,
                cell.seed,
                trajectory.depth,
                trajectory.history,
                state_word(&trajectory.states),
                side_word(&trajectory.physical_sides),
                side_word(&trajectory.handles),
                trajectory.complete,
                trajectory.structurally_valid,
                trajectory.naturally_quiescent,
                trajectory.duplicate_exact,
                trajectory.one_propagation,
                trajectory.permanent_fingerprint,
                trajectory.start_fingerprint,
                trajectory.trace_fingerprint,
                trajectory.end_fingerprint,
                trajectory.work,
                cell.distinct_trajectories,
                cell.distinct_trace_fingerprints,
                cell.passed,
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
        "--probe" => run_ssa2_p_probe(),
        "--micro" => run_ssa2_p_micro(),
        "--gate" => run_ssa2_p_gate(),
        "--definitive" => {
            eprintln!("definitive execution is not authorized for SSA2-P development");
            std::process::exit(2);
        }
        _ => {
            eprintln!("expected --probe, --micro, --gate, or --definitive");
            std::process::exit(2);
        }
    };
    fs::create_dir_all("results").expect("results directory is writable");
    let stem = format!(
        "ssa2_p_preserved_affordance_generativity_{}_v2",
        report.stage.to_ascii_lowercase()
    );
    write_atomic(Path::new(&format!("results/{stem}.md")), &markdown(&report));
    write_atomic(Path::new(&format!("results/{stem}.csv")), &csv(&report));
    println!("{}", markdown(&report));
    if !report.passed {
        std::process::exit(1);
    }
}
