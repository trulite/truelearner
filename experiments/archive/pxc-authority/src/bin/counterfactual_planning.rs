use std::{env, fs, path::PathBuf};

use organism_v0::composable_models;

fn join(values: &[u64]) -> String {
    values
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_roles(values: &[usize]) -> String {
    values
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn main() {
    let output = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/d4b_planning_traces.csv"));
    let report = composable_models::run_planning_experiment();
    let mut csv = String::from(
        "seed,required_depth,reachable,candidate_index,action_ids,predicted_marker_roles,distinguishes,before_real_action\n",
    );
    for trace in &report.traces {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            trace.seed_index,
            trace.required_depth,
            trace.reachable,
            trace.candidate_index,
            join(&trace.action_ids),
            join_roles(&trace.predicted_marker_roles),
            trace.distinguishes,
            trace.before_real_action
        ));
    }
    fs::write(&output, csv).expect("write d4b planning trace CSV");
    composable_models::print_planning_report(&report);
    println!(
        "wrote {} pre-action candidate traces to {}",
        report.traces.len(),
        output.display()
    );
}
