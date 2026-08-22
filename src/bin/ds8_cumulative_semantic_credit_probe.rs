use organism_v0::ds8_cumulative_semantic_credit_probe::run;

fn main() {
    let report = run();
    println!(
        "protocol={} seed={} passed={} duplicate_exact={} first_collapse={} updates={}/{} admissions={}/{} swapped={}/{} consequence_spikes={} consequence_routes={}",
        report.protocol,
        report.seed,
        report.passed,
        report.duplicate_exact,
        report.first_collapse,
        report.first_updates,
        report.second_updates,
        report.first_admissions,
        report.second_admissions,
        report.swapped_first_admissions,
        report.swapped_second_admissions,
        report.consequence_spikes,
        report.consequence_routes,
    );
    for check in report.checks {
        println!("check={} passed={}", check.name, check.passed);
    }
    if !report.passed {
        std::process::exit(1);
    }
}
