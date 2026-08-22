use organism_v0::ds7_cumulative_plasticity_allocation_gate::run;

fn main() {
    let report = run();
    println!(
        "protocol={} passed={} duplicate_exact={} cells={}",
        report.protocol,
        report.passed,
        report.duplicate_exact,
        report.cells.len()
    );
    for cell in report.cells {
        println!(
            "seed={} load={} passed={} acquisition={} heldout={}/{} admitted={}/{} baseline={} reduction={:.6} shuffled={} entry_resistance={}/{} final_resistance={}/{} removed={}/{} stale={} repaired={}/{} shuffled_repair_blocked={} retained_economy={} controls={} audit={} M4={}",
            cell.seed,
            cell.load,
            cell.passed,
            cell.blank_acquisition,
            cell.heldout_correct,
            cell.heldout_total,
            cell.productive_admissions,
            cell.distractor_admissions,
            cell.always_open_admissions,
            cell.admission_reduction,
            cell.shuffled_reversal,
            cell.entry_resistance_correct,
            cell.entry_resistance_shuffled,
            cell.final_resistance_correct,
            cell.final_resistance_shuffled,
            cell.removed_correct,
            cell.removed_shuffled,
            cell.stale_blocked,
            cell.repaired_correct,
            cell.repaired_total,
            cell.shuffled_repair_blocked,
            cell.retained_economy,
            cell.controls,
            cell.source_audit,
            cell.cumulative_m4,
        );
    }
    if !report.passed {
        std::process::exit(1);
    }
}
