#![allow(dead_code)]

use std::env;

#[path = "../ds1_after_d3_cumulative_composition_retry.rs"]
mod ds1_after_d3_cumulative_composition_retry;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds1_after_d3_cumulative_composition_retry::run;
use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("unchanged DS1 after D3: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds1_after_d3_cumulative_composition_retry [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = run(mode);
    println!(
        "{} mode={} audit={} claim_eligible={} enabling_only={} M0_authoritative={} M1_exists={} M1_authoritative={} protocol={} collapse_stage={:?} collapse={} source={:?}",
        report.label,
        report.mode,
        if report.audit_passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.enabling_only,
        report.m0_authoritative,
        report.m1_exists,
        report.m1_authoritative,
        report.protocol,
        report.first_collapse_stage,
        report.first_collapse,
        report.source
    );
    for (stage, status) in report.stages.iter().enumerate() {
        println!("stage={} status={}", stage, status);
    }
    for seed in &report.seeds {
        println!(
            "seed={} acquisition={} events={} two_roots={} D3_directions={} physical={} delivered={} updates={} patterns={} divergent={} consequence_mature={} evaluator_mature={} heldout={}/{} abstain={} reversed_world_mature={} reversed_evaluator={} D3_work={} learner_work={} learner_bytes={} fingerprint={:016x} stages={:?}",
            seed.seed,
            seed.acquisition_episodes,
            seed.event_formations,
            seed.two_root_episodes,
            seed.d3_directions,
            seed.physical_directions,
            seed.direction_deliveries,
            seed.update_calls,
            seed.patterns,
            seed.divergent_patterns,
            seed.consequence_mature_patterns,
            seed.evaluator_mature_patterns,
            seed.held_out_successes,
            seed.held_out_episodes,
            seed.held_out_abstentions,
            seed.reversed_world_mature_patterns,
            seed.reversed_world_evaluator_patterns,
            seed.d3_work,
            seed.learner_work,
            seed.persistent_learner_bytes,
            seed.learner_fingerprint,
            seed.stage_ready
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M0={} D3={} D3_readiness={} E0={} DS1={} results={}",
        ds1_after_d3_cumulative_composition_retry::EXACT_PARENT,
        ds1_after_d3_cumulative_composition_retry::PROTOCOL_COMMIT,
        ds1_after_d3_cumulative_composition_retry::AUTHORITATIVE_M0,
        ds1_after_d3_cumulative_composition_retry::FROZEN_D3_SHA256,
        ds1_after_d3_cumulative_composition_retry::FROZEN_D3_READINESS_SHA256,
        ds1_after_d3_cumulative_composition_retry::FROZEN_E0_SHA256,
        ds1_after_d3_cumulative_composition_retry::FROZEN_DS1_SHA256,
        ds1_after_d3_cumulative_composition_retry::FROZEN_RESULTS_DIGEST
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
