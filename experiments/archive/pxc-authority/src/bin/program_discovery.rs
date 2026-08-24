use organism_v0::program_discovery::{
    print_program_discovery_report, run_program_discovery_experiment,
};
use std::fmt::Write;

fn main() {
    let report = run_program_discovery_experiment();
    print_program_discovery_report(&report);
    if let Some(path) = std::env::args().nth(1) {
        std::fs::write(path, csv(&report)).expect("write P0 discovery CSV");
    }
    if !report.experimental_gate_valid {
        std::process::exit(1);
    }
}

fn csv(report: &organism_v0::program_discovery::ProgramDiscoveryReport) -> String {
    let mut output = String::from(
        "section,name,episode,successful_seeds,total_seeds,first_success_episode,proposals,spikes,candidates,stable_arrows,lookup_correct,feedback_correct,continuation_correct,finish_correct,heldout_correct,heldout_total\n",
    );
    for gate in &report.isolated {
        writeln!(
            output,
            "isolated,{},{},{},{},{:.3},{},{},{},{},false,false,false,false,{},{}",
            gate.name,
            0,
            gate.successful_seeds,
            gate.total_seeds,
            gate.average_episodes,
            gate.proposed_arrows,
            0,
            gate.surviving_arrows,
            gate.surviving_arrows,
            0,
            0
        )
        .unwrap();
    }
    for condition in [
        &report.real,
        &report.shuffled,
        &report.random,
        &report.activity_only,
    ] {
        writeln!(
            output,
            "condition,{},{},{},{},{:?},{:?},{:?},{:.3},{},false,false,false,false,{},{}",
            condition.name,
            0,
            condition.competent_seeds,
            condition.total_seeds,
            condition.average_first_success_episode,
            condition.average_proposals_before_first_success,
            condition.average_spikes_before_first_success,
            condition.average_final_arrows,
            0,
            condition.held_out_correct,
            condition.held_out_total
        )
        .unwrap();
    }
    for point in &report.real.representative_trajectory {
        writeln!(
            output,
            "trajectory,real terminal feedback,{},{},{},,{},{},{},{},{},{},{},{},{},{}",
            point.episode,
            0,
            0,
            0,
            0,
            point.candidates,
            point.stable_arrows,
            point.lookup_correct,
            point.feedback_correct,
            point.continuation_correct,
            point.finish_correct,
            0,
            0
        )
        .unwrap();
    }
    output
}
