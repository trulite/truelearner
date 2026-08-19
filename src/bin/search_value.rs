use std::{env, fs, path::PathBuf};

use organism_v0::search_value;

fn main() {
    let output = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/s0_s1_search_value.csv"));
    let report = search_value::run_experiment();
    let mut csv = String::from(
        "record,experience_or_depth,entries_or_cases,generated,scored,expanded,complete_candidates,model_applications,heuristic_evaluations,model_work,heuristic_work,total_work\n",
    );
    for checkpoint in &report.checkpoints {
        csv.push_str(&format!(
            "training,{},{},,,{},,{},,,,{}\n",
            checkpoint.examples,
            checkpoint.heuristic_entries,
            checkpoint.held_out_expanded,
            checkpoint.held_out_model_applications,
            checkpoint.held_out_total_work
        ));
    }
    for depth in &report.depth_reports {
        csv.push_str(&format!(
            "learned_depth,{},{},{},{},{},{},{},{},{},{},{}\n",
            depth.required_depth,
            depth.cases,
            depth.learned_generated,
            depth.learned_scored,
            depth.learned_expanded,
            depth.learned_complete,
            depth.learned_model_applications,
            depth.learned_heuristic_evaluations,
            depth.learned_model_work,
            depth.learned_heuristic_work,
            depth.learned_total_work
        ));
        csv.push_str(&format!(
            "exhaustive_depth,{},{},{},,{},{},{},,{},,{}\n",
            depth.required_depth,
            depth.cases,
            depth.exhaustive_generated,
            depth.exhaustive_expanded,
            depth.exhaustive_complete,
            depth.exhaustive_model_applications,
            depth.exhaustive_model_applications * 13,
            depth.exhaustive_total_work
        ));
    }
    fs::write(&output, csv).expect("write S0/S1 search-value CSV");
    search_value::print_report(&report);
    println!("wrote search-value measurements to {}", output.display());
    println!("passed={}", report.passed);
}
