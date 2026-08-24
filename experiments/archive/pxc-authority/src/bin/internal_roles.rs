use organism_v0::internal_roles::{print_report, run_experiment};
use std::fmt::Write;

fn main() {
    let report = run_experiment();
    print_report(&report);
    if let Some(path) = std::env::args().nth(1) {
        std::fs::write(path, csv(&report)).expect("write P3 internal-role CSV");
    }
    println!(
        "RESULT: {}",
        if report.passed {
            "PASS - recurring activity discovered sensory, working, and control-event roles used by a recurrent program"
        } else {
            "FAIL - discovered internal roles did not support the complete recurrent program"
        }
    );
}

fn csv(report: &organism_v0::internal_roles::P3Report) -> String {
    let mut output = String::from(
        "gate,condition,successful,total,correct,correct_total,sensory_roles,working_roles,event_roles,program_arrows,encounter_representations,created,released,competence_episode,passed\n",
    );
    writeln!(
        output,
        "P3a,working,{},{},{},{},0,{},0,0,0,0,0,,{}",
        report.working.successful_seeds,
        report.working.total_seeds,
        report.working.transfer_correct,
        report.working.transfer_total,
        report.working.learned_roles,
        report.working.passed
    )
    .unwrap();
    writeln!(
        output,
        "P3b,feedback,{},{},{},{},0,2,0,{},0,0,0,,{}",
        report.feedback.successful_seeds,
        report.feedback.total_seeds,
        report.feedback.depth_correct,
        report.feedback.depth_total,
        report.feedback.permanent_arrows,
        report.feedback.passed
    )
    .unwrap();
    writeln!(
        output,
        "P3c,control,{},{},{},{},0,0,{},2,0,0,0,,{}",
        report.controls.successful_seeds,
        report.controls.total_seeds,
        report.controls.depth_correct,
        report.controls.depth_total,
        report.controls.learned_roles,
        report.controls.passed
    )
    .unwrap();
    for (condition, result) in [
        ("real", &report.integrated),
        ("shuffled", &report.shuffled),
        ("random", &report.random),
    ] {
        writeln!(
            output,
            "P3d,{condition},{},{},{},{},{:.1},{:.1},{:.1},{:.1},{:.1},{},{},{:?},{}",
            result.competent_seeds,
            result.total_seeds,
            result.held_out_correct,
            result.held_out_total,
            result.average_sensory_roles,
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
