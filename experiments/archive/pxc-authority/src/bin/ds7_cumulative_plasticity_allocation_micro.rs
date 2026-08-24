use organism_v0::ds7_cumulative_plasticity_allocation_micro::run;

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
            "seed={} history={} passed={} acquisition={} productive_admissions={} unproductive_admissions={} always_open={} reduction={:.6} shuffled={} reversal_opportunities={} reversal_explorations={} retargeted={} lifecycle={} fresh={} audit={} M4={}",
            cell.seed,
            cell.history,
            cell.passed,
            cell.acquisition,
            cell.productive_admissions,
            cell.unproductive_admissions,
            cell.always_open_admissions,
            cell.reduction,
            cell.shuffled_reversal,
            cell.reversal_opportunities,
            cell.reversal_explorations,
            cell.retargeted,
            cell.lifecycle,
            cell.fresh_identity_layout,
            cell.source_audit,
            cell.cumulative_m4,
        );
    }
    if !report.passed {
        std::process::exit(1);
    }
}
