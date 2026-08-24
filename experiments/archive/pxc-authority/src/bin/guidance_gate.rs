use std::{env, fs, path::PathBuf};

use organism_v0::search_value;

fn main() {
    let output = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/s1_2_guidance_gate.csv"));
    let report = search_value::run_guidance_gate_experiment();
    let mut csv = String::from(
        "record,name_or_problems,entries_or_cases,training_work,neutral_training_work,exploration,held_out_work,neutral_work,guided_work,oracle_choices,false_guides,missed_guides\n",
    );
    for checkpoint in &report.checkpoints {
        csv.push_str(&format!(
            "checkpoint,{},{},{},{},{},{},0,0,{},{},{}\n",
            checkpoint.training_problems,
            checkpoint.gate_entries,
            checkpoint.cumulative_training_work,
            checkpoint.cumulative_neutral_work,
            checkpoint.cumulative_exploration_choices,
            checkpoint.held_out_total_work,
            checkpoint.oracle_mode_choices,
            checkpoint.false_guides,
            checkpoint.missed_guides
        ));
    }
    for context in &report.contexts {
        csv.push_str(&format!(
            "context,{},{},0,0,0,0,{},{},{},{},0\n",
            context.context,
            context.cases,
            context.neutral_total_work,
            context.guided_total_work,
            context.oracle_guided_cases,
            usize::from(context.learned_guided)
        ));
    }
    csv.push_str(&format!(
        "summary,learned,{},{},{},{},{},{},{},{},{},{}\n",
        report.held_out_cases,
        report.training_total_work,
        report.training_neutral_work,
        report.exploration_choices,
        report.learned_total_work,
        report.neutral_total_work,
        report.guided_total_work,
        report.learned_oracle_choices,
        report.learned_false_guides,
        report.learned_missed_guides
    ));
    fs::write(&output, csv).expect("write S1.2 guidance-gate CSV");
    search_value::print_guidance_gate_report(&report);
    println!("wrote guidance-gate measurements to {}", output.display());
    println!("passed={}", report.passed);
}
