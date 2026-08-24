use std::{env, fs, path::PathBuf};

use organism_v0::discovery;

fn main() {
    let output = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/d2_3_plasticity.csv"));
    let report = discovery::run_plasticity_experiment();
    let mut csv = String::from(
        "choices,maturity,seed,phase,problem,old_action,new_action,preferred_action,trusted,violation_streak,reopen_count,selected_actions,paid_actions,correct,historical_evidence\n",
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
            point.seed_index,
            point.phase,
            point.problem,
            point.old_action,
            point.new_action,
            preferred,
            point.trusted,
            point.violation_streak,
            point.reopen_count,
            selected,
            point.paid_actions,
            point.correct,
            point.historical_evidence
        ));
    }
    fs::write(&output, csv).expect("write plasticity trajectory CSV");
    println!(
        "wrote {} trajectories to {}",
        report.trajectories.len(),
        output.display()
    );
}
