#![allow(dead_code)]

use std::env;

#[path = "../ds_d1_stage8b_functional_sufficiency.rs"]
mod ds_d1_stage8b_functional_sufficiency;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds_d1_stage8b_functional_sufficiency::run;
use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-D1: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_d1_stage8b_functional_sufficiency [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = run(mode);
    println!(
        "{} mode={} audit={} claim_eligible={} diagnostic_only={} M0_authoritative={} M1_exists={} parallel_cells={} encoding_equivalent={} sufficient_arms={:?} protocol={}",
        report.label,
        report.mode,
        if report.audit_passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.diagnostic_only,
        report.m0_authoritative,
        report.m1_exists,
        report.parallel_cells,
        report.encoding_equivalent,
        report.functionally_sufficient_arms,
        report.protocol
    );
    println!("source={:?}", report.source);
    for cell in &report.cells {
        println!(
            "seed={} arm={} fields={} acquisition={} update_fires={} updates={} patterns={} divergence={} divergent={} mature_correct={} heldout={} heldout_success={} abstentions={} bool_trace={:016x} episode={:016x} learner={:016x} learner_work={} learner_bytes={} property_bytes={} observable_established={} pass={}",
            cell.seed,
            cell.arm_label,
            cell.property_fields,
            cell.acquisition_episodes,
            cell.update_fires,
            cell.update_calls,
            cell.patterns,
            cell.strength_divergence,
            cell.divergent_patterns,
            cell.correctly_mature_patterns,
            cell.held_out_episodes,
            cell.held_out_successes,
            cell.held_out_abstentions,
            cell.boolean_trace_fingerprint,
            cell.episode_fingerprint,
            cell.learner_fingerprint,
            cell.learner_work,
            cell.persistent_learner_bytes,
            cell.persistent_property_bytes,
            cell.current_substrate_observability_established,
            cell.passed
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M0={} D0={} D0_handoff={} E0={} DS1={} results={}",
        ds_d1_stage8b_functional_sufficiency::EXACT_PARENT,
        ds_d1_stage8b_functional_sufficiency::PROTOCOL_COMMIT,
        ds_d1_stage8b_functional_sufficiency::AUTHORITATIVE_M0,
        ds_d1_stage8b_functional_sufficiency::FROZEN_D0_SOURCE_SHA256,
        ds_d1_stage8b_functional_sufficiency::FROZEN_D0_HANDOFF_SHA256,
        ds_d1_stage8b_functional_sufficiency::FROZEN_E0_SHA256,
        ds_d1_stage8b_functional_sufficiency::FROZEN_DS1_SHA256,
        ds_d1_stage8b_functional_sufficiency::FROZEN_RESULTS_DIGEST
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
