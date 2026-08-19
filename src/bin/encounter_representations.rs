use organism_v0::local_plasticity::{print_p2_2_report, run_p2_2_experiment};
use std::fmt::Write;

fn main() {
    let report = run_p2_2_experiment();
    print_p2_2_report(&report);
    if let Some(path) = std::env::args().nth(1) {
        std::fs::write(path, csv(&report)).expect("write P2.2 encounter representation CSV");
    }
    println!(
        "RESULT: {}",
        if report.passed {
            "PASS - pre-coupling encounter representations learn to control structural plasticity"
        } else {
            "FAIL - learned encounter representations did not preserve sparse program discovery"
        }
    );
}

fn csv(report: &organism_v0::local_plasticity::P22Report) -> String {
    let mut output = String::from(
        "section,condition,representations,valued,mixed,context_sensitive,competent_seeds,total_seeds,correct,total,created,gate_evaluations,gate_admissions,recognition_work,value_updates,first_useful_episode,competence_episode,early_created_per_episode,late_created_per_episode,work\n",
    );
    writeln!(
        output,
        "P2.2a,representation,{},{},{},{},0,0,{},{},0,0,0,0,0,,,0,0,0",
        report.representation.representations,
        report.representation.valued_representations,
        report.representation.mixed_outcome_representations,
        report.representation.context_sensitive_representations,
        report.representation.useful_recalled,
        report.representation.useful_total
    )
    .unwrap();
    for (condition, result) in [
        ("always", &report.always),
        ("frozen", &report.frozen_gate),
        ("random", &report.random),
        ("shuffled", &report.shuffled),
        ("oracle", &report.oracle),
    ] {
        writeln!(
            output,
            "P2.2b,{condition},0,0,0,0,{},{},{},{},{},{},{},0,0,,{:?},0,0,{}",
            result.competent_seeds,
            result.total_seeds,
            result.held_out_correct,
            result.held_out_total,
            result.average_created,
            result.average_gate_evaluations,
            result.average_gate_admissions,
            result.average_competence_episode,
            result.average_work
        )
        .unwrap();
    }
    writeln!(
        output,
        "P2.2c,adaptive,{:.1},{:.1},0,0,{},{},{},{},{},{},{},{},{},{:?},{:?},{:.4},{:.4},{}",
        report.adaptive.average_representations,
        report.adaptive.average_valued_representations,
        report.adaptive.condition.competent_seeds,
        report.adaptive.condition.total_seeds,
        report.adaptive.condition.held_out_correct,
        report.adaptive.condition.held_out_total,
        report.adaptive.condition.average_created,
        report.adaptive.condition.average_gate_evaluations,
        report.adaptive.condition.average_gate_admissions,
        report.adaptive.average_representation_work,
        report.adaptive.average_value_updates,
        report.adaptive.average_first_useful_episode,
        report.adaptive.condition.average_competence_episode,
        report.adaptive.early_created_per_episode,
        report.adaptive.late_created_per_episode,
        report.adaptive.condition.average_work
    )
    .unwrap();
    output
}
