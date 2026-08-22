#![allow(dead_code)]

use std::env;

#[path = "../ds_c0_anonymous_credit_coupling.rs"]
mod ds_c0_anonymous_credit_coupling;
#[path = "../ds_d0_stage8b_discrimination.rs"]
mod ds_d0_stage8b_discrimination;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds_d0_stage8b_discrimination::run;
use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-D0: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_d0_stage8b_discrimination [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = run(mode);
    println!(
        "{} mode={} audit={} claim_eligible={} diagnostic_only={} M0_authoritative={} M1_exists={} parallel_cells={} protocol={}",
        report.label,
        report.mode,
        if report.audit_passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.diagnostic_only,
        report.m0_authoritative,
        report.m1_exists,
        report.parallel_cells,
        report.protocol
    );
    println!(
        "sufficient_arms={:?} deeper_gate_authorized={} source={:?}",
        report.sufficient_arms, report.deeper_gate_authorized, report.source
    );
    for cell in &report.cells {
        println!(
            "seed={} arm={} same_episode={} fields={} yielded_input={} candidate_to_bool_edges={} reachable_update_edges={} DS1_updates={} input_value={:?} learner_bytes={} persistent_property_bytes={} diagnostic_only={} pass={}",
            cell.seed,
            cell.arm_label,
            cell.same_parent_episode,
            cell.property_fields,
            cell.property_yielded_update_input,
            cell.candidate_to_bool_edges,
            cell.reachable_update_edges,
            cell.runtime_ds1_updates,
            cell.positive_value,
            cell.diagnostic_learner_bytes,
            cell.persistent_property_bytes,
            cell.diagnostic_only,
            cell.passed
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M0={} retry={} handoff={} C0={} E0={} DS1={} results={}",
        ds_d0_stage8b_discrimination::EXACT_PARENT,
        ds_d0_stage8b_discrimination::PROTOCOL_COMMIT,
        ds_d0_stage8b_discrimination::AUTHORITATIVE_M0,
        ds_d0_stage8b_discrimination::FROZEN_PARENT_RETRY_SHA256,
        ds_d0_stage8b_discrimination::FROZEN_PARENT_HANDOFF_SHA256,
        ds_d0_stage8b_discrimination::FROZEN_C0_SHA256,
        ds_d0_stage8b_discrimination::FROZEN_E0_SHA256,
        ds_d0_stage8b_discrimination::FROZEN_DS1_SHA256,
        ds_d0_stage8b_discrimination::FROZEN_RESULTS_DIGEST
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
