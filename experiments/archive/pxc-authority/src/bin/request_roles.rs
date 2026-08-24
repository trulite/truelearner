use organism_v0::request_roles::{print_report, run_experiment, P4Report};
use std::fmt::Write;

fn main() {
    let report = run_experiment();
    print_report(&report);
    if let Some(path) = std::env::args().nth(1) {
        std::fs::write(path, csv(&report)).expect("write P4 request-role CSV");
    }
    if !report.passed {
        std::process::exit(1);
    }
}

fn csv(report: &P4Report) -> String {
    let mut output = String::from(
        "gate,condition,successful,total,correct,correct_total,sensory_roles,request_roles,working_roles,event_roles,program_arrows,encounter_representations,created,released,competence_episode,passed\n",
    );
    writeln!(
        output,
        "P4a,request,{},{},{},{},0,{},0,0,0,0,0,0,,{}",
        report.request.successful_seeds,
        report.request.total_seeds,
        report.request.transfer_correct,
        report.request.transfer_total,
        report.request.learned_roles,
        report.request.passed
    )
    .unwrap();
    writeln!(
        output,
        "P4b,interface,{},{},{},{},0,1,2,2,4,0,0,0,,{}",
        report.use_report.successful_seeds,
        report.use_report.total_seeds,
        report.use_report.depth_correct,
        report.use_report.depth_total,
        report.use_report.passed
    )
    .unwrap();
    for (condition, result) in [
        ("real", &report.integrated),
        ("shuffled", &report.shuffled),
        ("random", &report.random),
    ] {
        writeln!(
            output,
            "P4c,{condition},{},{},{},{},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{},{},{:?},{}",
            result.competent_seeds,
            result.total_seeds,
            result.held_out_correct,
            result.held_out_total,
            result.average_sensory_roles,
            result.average_request_roles,
            result.average_working_roles,
            result.average_event_roles,
            result.average_program_arrows,
            result.average_encounter_representations,
            result.average_created,
            result.average_released,
            result.average_competence_episode,
            condition == "real" && report.passed
        )
        .unwrap();
    }
    output
}
