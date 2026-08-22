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
    if mode != "--probe" {
        eprintln!("usage: ds4_cumulative_request_start_port --probe");
        std::process::exit(2);
    }
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
}
