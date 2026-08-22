use organism_v0::ds7_cumulative_plasticity_targeting_probe::run;

fn main() {
    let report = run();
    println!(
        "protocol={} seed={} passed={} duplicate_exact={} first_collapse={} prototypes={} values={} proposals={} productive_admissions={} unproductive_admissions={} exploration_admissions={}",
        report.protocol,
        report.seed,
        report.passed,
        report.duplicate_exact,
        report.first_collapse,
        report.prototypes,
        report.values,
        report.proposals,
        report.productive_admissions,
        report.unproductive_admissions,
        report.exploration_admissions,
    );
    for check in report.checks {
        println!("check={} passed={}", check.name, check.passed);
    }
    if !report.passed {
        std::process::exit(1);
    }
}

