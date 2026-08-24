use std::{env, fs, path::PathBuf};

use organism_v0::discovery;

fn main() {
    let output = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/d2_2_remap.csv"));
    let report = discovery::run_remap_experiment();
    let mut csv = String::from(
        "choices,maturity,case,seed,policy,problem,old_action,new_action,old_value,new_value,preferred_action,selected_actions,paid_actions,correct,cumulative_cost\n",
    );
    for point in &report.trajectories {
        let selected = point
            .selected_actions
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join("|");
        let preferred = point
            .preferred_action
            .map(|action| action.to_string())
            .unwrap_or_default();
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            point.choice_count,
            point.maturity,
            point.remap_case,
            point.seed_index,
            point.policy_kind,
            point.problem,
            point.old_action,
            point.new_action,
            point.old_action_value,
            point.new_action_value,
            preferred,
            selected,
            point.paid_actions,
            point.correct,
            point.cumulative_cost
        ));
    }
    fs::write(&output, csv).expect("write remap trajectory CSV");
    println!(
        "wrote {} trajectories to {}",
        report.trajectories.len(),
        output.display()
    );
}
