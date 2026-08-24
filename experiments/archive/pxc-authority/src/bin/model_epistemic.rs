use std::{env, fs, path::PathBuf};

use organism_v0::model_epistemic;

fn join(values: &[usize]) -> String {
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
        .unwrap_or_else(|| PathBuf::from("results/d3_pre_action_traces.csv"));
    let report = model_epistemic::run_model_based_action_experiment();
    let mut csv = String::from(
        "seed,template,expected_action,chosen_action,correct,first_route,second_route,shared_roles,first_only_roles,second_only_roles,shared_arrows,first_only_arrows,second_only_arrows,assessed_action,changed_roles,preserved_roles,changes_route_specific,preserves_route_specific,changes_shared,preserves_shared,model_fingerprint_before,model_fingerprint_after_choice\n",
    );
    for decision in &report.decisions {
        for assessment in &decision.trace.assessments {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                decision.seed_index,
                decision.template_index,
                decision.expected_action_id,
                decision.chosen_action_id,
                decision.correct,
                join(&decision.trace.first_route_roles),
                join(&decision.trace.second_route_roles),
                join(&decision.trace.shared_roles),
                join(&decision.trace.first_only_roles),
                join(&decision.trace.second_only_roles),
                decision.trace.shared_arrows,
                decision.trace.first_only_arrows,
                decision.trace.second_only_arrows,
                assessment.action_id,
                join(&assessment.changed_roles),
                join(&assessment.preserved_roles),
                assessment.changes_route_specific,
                assessment.preserves_route_specific,
                assessment.changes_shared,
                assessment.preserves_shared,
                decision.trace.model_fingerprint_before,
                decision.trace.model_fingerprint_after_choice
            ));
        }
    }
    fs::write(&output, csv).expect("write d3 pre-action trace CSV");
    println!(
        "wrote {} decisions and {} action assessments to {}",
        report.decisions.len(),
        report
            .decisions
            .iter()
            .map(|decision| decision.trace.assessments.len())
            .sum::<usize>(),
        output.display()
    );
}
