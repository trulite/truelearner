#![allow(dead_code)]

#[path = "../binding.rs"]
mod binding;
#[path = "../ds4_cumulative_request_start_port.rs"]
mod port;
#[path = "../research_runtime.rs"]
mod research_runtime;

fn main() {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "--probe".to_string());
    if mode == "--probe" {
        let report = port::run_probe();
        println!(
            "label={} protocol={} claim_eligible={} path_exists={} learned_m3_uses={} completion_activity={} selection={} execution={} update={} selected_from_occurrence={} pre_answer_trace={} m3_work={} no_event_selection={} no_event_execution={} no_event_update={} request_fingerprint={} source={:?}",
            report.label,
            report.protocol,
            report.claim_eligible,
            report.path_exists,
            report.learned_m3_uses,
            report.completion_activity,
            report.request_selection_activations,
            report.request_execution_activations,
            report.request_update_activations,
            report.selected_from_occurrence,
            report.pre_answer_trace,
            report.m3_physical_work,
            report.no_event_selection,
            report.no_event_execution,
            report.no_event_update,
            report.request_fingerprint_after,
            report.source,
        );
        if !report.path_exists {
            std::process::exit(1);
        }
        return;
    }
    let harness = match mode.as_str() {
        "--micro" => research_runtime::HarnessMode::Micro,
        "--gate" => research_runtime::HarnessMode::Gate,
        "--definitive" => research_runtime::HarnessMode::Definitive,
        _ => {
            eprintln!(
                "usage: ds4_cumulative_request_start_port [--probe|--micro|--gate|--definitive]"
            );
            std::process::exit(2)
        }
    };
    let report = port::run(harness);
    println!(
        "label={} protocol={} mode={} claim_eligible={} development_ready={} M3_authoritative={} M4_exists={} replay={} collapse_stage={:?} collapse={} learners={}/{} single_roles={} competence_millis={} held_out={}/{} explicit={} queues_empty={} positions={} m3_learned_uses={} completion_activity={} selection={} execution={} update={} m3_work={} p4_nonplastic={} m3_nonplastic={} source={:?}",
        report.label,
        report.protocol,
        report.mode,
        report.claim_eligible,
        report.development_ready,
        report.m3_authoritative,
        report.m4_exists,
        report.duplicate_deterministic,
        report.first_collapse_stage,
        report.first_collapse,
        report.ready_learners,
        report.learner_count,
        report.single_role_learners,
        report.average_competence_episode_millis,
        report.held_out_correct,
        report.held_out_total,
        report.explicit_answers,
        report.queues_empty,
        report.request_positions,
        report.m3_learned_uses,
        report.completion_activity,
        report.selection_activations,
        report.execution_activations,
        report.update_activations,
        report.m3_physical_work,
        report.p4_nonplastic,
        report.m3_nonplastic,
        report.source,
    );
    for (index, stage) in report.stages.iter().enumerate() {
        println!("stage={} status={}", index, stage);
    }
    for control in &report.controls {
        println!(
            "control={} name={} pass={} diagnostic={}",
            control.number, control.name, control.passed, control.diagnostic
        );
    }
    if harness == research_runtime::HarnessMode::Definitive {
        eprintln!("DS4 definitive is locked pending separate matrix preregistration");
        std::process::exit(2);
    }
    if !report.development_ready {
        std::process::exit(1);
    }
}
