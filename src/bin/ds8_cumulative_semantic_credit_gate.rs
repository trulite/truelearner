use organism_v0::ds8_cumulative_semantic_credit_gate::run;

fn main() {
    let report = run();
    println!(
        "protocol={} passed={} duplicate_exact={} cells={}",
        report.protocol,
        report.passed,
        report.duplicate_exact,
        report.cells.len(),
    );
    for cell in report.cells {
        println!(
            "seed={} load={} passed={} acquired={} physical={} heldout={}/{} admission={}/{} reduction={:.2}% raw_swap={} value_shuffle={} resistance={}->{} removed={} stale={} repaired={}/{} economy={} controls={} source={} m5={}",
            cell.seed,
            cell.load,
            cell.passed,
            cell.blank_acquisition,
            cell.physical_exact,
            cell.heldout,
            cell.heldout_total,
            cell.route_admissions,
            cell.distractor_admissions,
            100.0 * cell.work_reduction,
            cell.raw_history_reversal,
            cell.value_shuffle_reversal,
            cell.entry_resistance,
            cell.final_resistance,
            cell.removed,
            cell.stale_blocked,
            cell.repaired,
            cell.repaired_total,
            cell.retained_economy,
            cell.controls,
            cell.source_audit,
            cell.cumulative_m5,
        );
    }
    if !report.passed {
        std::process::exit(1);
    }
}
