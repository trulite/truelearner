use organism_v0::ds8_cumulative_semantic_credit_micro::run;

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
            "seed={} history={} passed={} executions={} updates={}/{} admissions={}/{} swapped={}/{} shuffled={}/{} physical={} contrast={} transfer={} controls={} source={} m5={}",
            cell.seed,
            cell.history,
            cell.passed,
            cell.executions,
            cell.first_updates,
            cell.second_updates,
            cell.first_admissions,
            cell.second_admissions,
            cell.swapped_first_admissions,
            cell.swapped_second_admissions,
            cell.shuffled_first_admissions,
            cell.shuffled_second_admissions,
            cell.physical_exact,
            cell.contrast,
            cell.fresh_transfer,
            cell.controls,
            cell.source_audit,
            cell.cumulative_m5,
        );
    }
    if !report.passed {
        std::process::exit(1);
    }
}
