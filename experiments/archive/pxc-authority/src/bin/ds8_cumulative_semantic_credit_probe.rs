use organism_v0::ds8_cumulative_semantic_credit_probe::{audit, run};

fn main() {
    if std::env::args().any(|argument| argument == "--audit") {
        let audit = audit();
        println!(
            "passed={} activation={} dependency={} protocol_v1={} protocol_v2={} allocator={} gate={} csv={} md={} d3={} d2={} c0={} cp0={} forbidden={} linkers={} normalizers={} occurrence_identity={}",
            audit.passed,
            audit.activation,
            audit.dependency,
            audit.protocol_v1,
            audit.protocol_v2,
            audit.allocator,
            audit.gate,
            audit.csv,
            audit.md,
            audit.d3,
            audit.d2,
            audit.c0,
            audit.cp0,
            audit.forbidden,
            audit.linkers,
            audit.normalizers,
            audit.occurrence_identity,
        );
        if !audit.passed {
            std::process::exit(1);
        }
        return;
    }
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
