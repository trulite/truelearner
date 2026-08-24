use organism_v0::local_plasticity::{print_p2_1_report, run_p2_1_experiment};
use std::fmt::Write;

fn main() {
    let report = run_p2_1_experiment();
    print_p2_1_report(&report);
    if let Some(path) = std::env::args().nth(1) {
        std::fs::write(path, csv(&report)).expect("write P2.1 selective plasticity CSV");
    }
    println!(
        "RESULT: {}",
        if report.passed {
            "PASS - learned plasticity value reduces local coupling churn without losing discovery"
        } else {
            "FAIL - plasticity value did not transfer or gated discovery lost competence"
        }
    );
}

fn csv(report: &organism_v0::local_plasticity::P21Report) -> String {
    let mut output = String::from(
        "section,condition,entries,predicted_useful,competent_seeds,total_seeds,correct,total,created,released,gate_evaluations,gate_admissions,active_touches,local_encounters,eligibility_updates,competence_episode,work\n",
    );
    writeln!(
        output,
        "P2.1a,predictor,{},{},0,0,{},{},0,0,0,0,0,0,0,,0",
        report.predictor.entries,
        report.predictor.predicted_useful,
        report.predictor.useful_recalled,
        report.predictor.useful_total
    )
    .unwrap();
    for (condition, result) in [
        ("always", &report.always),
        ("learned", &report.learned),
        ("random", &report.random),
        ("shuffled", &report.shuffled),
        ("oracle", &report.oracle),
    ] {
        writeln!(
            output,
            "P2.1b,{condition},0,0,{},{},{},{},{},{},{},{},{},{},{},{:?},{}",
            result.competent_seeds,
            result.total_seeds,
            result.held_out_correct,
            result.held_out_total,
            result.average_created,
            result.average_released,
            result.average_gate_evaluations,
            result.average_gate_admissions,
            result.average_active_touches,
            result.average_local_encounters,
            result.average_eligibility_updates,
            result.average_competence_episode,
            result.average_work
        )
        .unwrap();
    }
    output
}
