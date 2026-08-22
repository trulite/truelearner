use organism_v0::ds6_cumulative_lifetime_probe::{print_report, run_probe};

fn main() {
    let report = run_probe();
    print_report(&report);
    if !report.diagnostic_complete {
        std::process::exit(1);
    }
}
