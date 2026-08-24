use organism_v0::role_discovery::{print_report, run_experiment};
use std::fmt::Write;

fn main() {
    let report = run_experiment();
    print_report(&report);
    if let Some(path) = std::env::args().nth(1) {
        std::fs::write(path, csv(&report)).expect("write P1 role discovery CSV");
    }
    if !report.passed {
        std::process::exit(1);
    }
}

fn csv(report: &organism_v0::role_discovery::P1Report) -> String {
    let mut output = String::from(
        "gate,condition,successful_seeds,total_seeds,correct,total,role_cells,possible_proposals,actual_proposals,used_proposals,surviving_arrows,first_success_episode,competence_episode,fingerprint_unchanged\n",
    );
    writeln!(
        output,
        "P1a,encoding-transfer,{},{},{},{},{},0,0,0,0,,,{}",
        report.roles.successful_seeds,
        report.roles.total_seeds,
        report.roles.transferred_encodings,
        report.roles.transferred_total,
        report.roles.learned_role_cells,
        report.roles.fingerprints_unchanged
    )
    .unwrap();
    writeln!(
        output,
        "P1b,forward-and-reverse,{},{},{},{},{},0,{},{},{},,,{}",
        report.lookup.forward_seeds + report.lookup.reverse_seeds,
        report.lookup.total_seeds * 2,
        report.lookup.transferred_correct,
        report.lookup.transferred_total,
        report.roles.learned_role_cells,
        report.lookup.proposed_arrows,
        report.lookup.ever_used_arrows,
        report.lookup.surviving_arrows,
        report.lookup.role_fingerprints_unchanged
    )
    .unwrap();
    for (condition, result) in [
        ("real", &report.integrated),
        ("shuffled", &report.shuffled),
        ("random", &report.random),
    ] {
        writeln!(
            output,
            "P1c,{},{},{},{},{},{:.3},{},{},{},{},{:?},{:?},{}",
            condition,
            result.competent_seeds,
            result.total_seeds,
            result.held_out_correct,
            result.held_out_total,
            result.average_roles,
            result.possible_proposals,
            result.actual_proposals,
            result.used_proposals,
            result.surviving_arrows,
            result.average_first_success_episode,
            result.average_competence_episode,
            result.fingerprints_unchanged
        )
        .unwrap();
    }
    output
}
