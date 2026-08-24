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
        .unwrap_or_else(|| PathBuf::from("results/d4a_composition_traces.csv"));
    let report = composable_models::run_composable_model_experiment();
    let mut csv = String::from(
        "seed,sequence,exact,initial,expected,predicted,fingerprint_before,fingerprint_after,step,action,source_roles,result\n",
    );
    for trace in &report.traces {
        for step in &trace.steps {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                trace.seed_index,
                trace.sequence_label,
                trace.exact,
                join(&trace.initial_identity_ids),
                join(&trace.expected_identity_ids),
                join(&trace.predicted_identity_ids),
                trace.model_fingerprint_before,
                trace.model_fingerprint_after,
                step.step,
                step.action_id,
                join_roles(&step.source_roles),
                join(&step.resulting_identity_ids)
            ));
        }
    }
    fs::write(&output, csv).expect("write d4a composition trace CSV");
    composable_models::print_report(&report);
    println!(
        "wrote {} sequence traces and {} model applications to {}",
        report.traces.len(),
        report.model_applications,
        output.display()
    );
}
