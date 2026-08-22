#![allow(dead_code)]

use std::env;

#[path = "../ds1_after_d2_cumulative_composition_retry.rs"]
mod ds1_after_d2_cumulative_composition_retry;
#[path = "../ds_c0_anonymous_credit_coupling.rs"]
mod ds_c0_anonymous_credit_coupling;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds1_after_d2_cumulative_composition_retry::run;
use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("unchanged DS1 after D2: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds1_after_d2_cumulative_composition_retry [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = run(mode);
    println!(
        "{} mode={} audit={} claim_eligible={} enabling_only={} M0_authoritative={} M1_exists={} first_collapse_stage={:?} first_collapse={}",
        report.label,
        report.mode,
        if report.audit_passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.enabling_only,
        report.m0_authoritative,
        report.m1_exists,
        report.first_collapse_stage,
        report.first_collapse
    );
    for (index, stage) in report.stages.iter().enumerate() {
        println!("stage[{index}]={stage}");
    }
    for seed in &report.seeds {
        println!(
            "seed={} acquisition={} directions={}/{} updates={} patterns={} divergent={} correctly_mature={} heldout={}/{} abstentions={} D2_work={} learner_work={} learner_bytes={} fingerprint={:016x} controls={:?}",
            seed.seed,
            seed.acquisition_episodes,
            seed.direction_deliveries,
            seed.physical_directions,
            seed.update_calls,
            seed.patterns,
            seed.divergent_patterns,
            seed.correctly_mature_patterns,
            seed.held_out_successes,
            seed.held_out_episodes,
            seed.held_out_abstentions,
            seed.d2_work,
            seed.learner_work,
            seed.persistent_learner_bytes,
            seed.learner_fingerprint,
            seed.negative_controls
        );
    }
    println!(
        "source={:?} parent={} protocol_commit={} protocol_amendment={} M0={} D2={} C0={} E0={} DS1={} results={}",
        report.source,
        ds1_after_d2_cumulative_composition_retry::EXACT_PARENT,
        ds1_after_d2_cumulative_composition_retry::PROTOCOL_COMMIT,
        ds1_after_d2_cumulative_composition_retry::PROTOCOL_AMENDMENT_COMMIT,
        ds1_after_d2_cumulative_composition_retry::AUTHORITATIVE_M0,
        ds1_after_d2_cumulative_composition_retry::FROZEN_D2_SHA256,
        ds1_after_d2_cumulative_composition_retry::FROZEN_C0_SHA256,
        ds1_after_d2_cumulative_composition_retry::FROZEN_E0_SHA256,
        ds1_after_d2_cumulative_composition_retry::FROZEN_DS1_SHA256,
        ds1_after_d2_cumulative_composition_retry::FROZEN_RESULTS_DIGEST
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
