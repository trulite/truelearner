#![allow(dead_code)]

use std::env;

#[path = "../ds1_after_r0_composition_retry.rs"]
mod ds1_after_r0_composition_retry;
#[path = "../ds_r0_anonymous_post_action_evidence_return.rs"]
mod ds_r0_anonymous_post_action_evidence_return;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds1_after_r0_composition_retry::run;
use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS1-after-R0 retry: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds1_after_r0_composition_retry [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = run(mode);
    println!(
        "{} mode={} audit={} claim_eligible={} M0_authoritative={} enabling_only={} M1_exists={} protocol={}",
        report.label,
        report.mode,
        if report.audit_passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.m0_authoritative,
        report.enabling_only,
        report.m1_exists,
        report.protocol
    );
    println!(
        "first_collapse_stage={:?} first_collapse={} stages={:?}",
        report.first_collapse_stage, report.first_collapse, report.stages
    );
    println!("source={:?}", report.source);
    for seed in &report.seeds {
        println!(
            "seed={} controls={}/22 roots={} handles={} choice={} choose_calls={} executions={} pulses={} relations={} return_relations={} evidence_fields={} evidence_surfaces={} updates={} strengths={} heldout={} E0_work={} A1_work={} R0_work={} E0_bytes={} A1_bytes={} DS1_bytes={} R0_bytes={} temporary_peak={}",
            seed.seed,
            seed.r0_controls,
            seed.roots,
            seed.handles,
            seed.choice,
            seed.choose_calls,
            seed.route_executions,
            seed.activity_pulses,
            seed.activity_relations,
            seed.temporary_relations,
            seed.evidence_fields,
            seed.paths.runtime_evidence_surfaces,
            seed.paths.runtime_ds1_updates,
            seed.paths.runtime_strength_observations,
            seed.paths.runtime_held_out_reconstructions,
            seed.e0_work,
            seed.a1_work,
            seed.r0_work,
            seed.e0_bytes,
            seed.a1_bytes,
            seed.ds1_bytes,
            seed.r0_bytes,
            seed.temporary_peak
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M0={} R0={} R0_readiness={} R0_e2b={} DS1={} results={}",
        ds1_after_r0_composition_retry::EXACT_PARENT,
        ds1_after_r0_composition_retry::PROTOCOL_COMMIT,
        ds1_after_r0_composition_retry::AUTHORITATIVE_M0,
        ds1_after_r0_composition_retry::FROZEN_R0_SHA256,
        ds1_after_r0_composition_retry::FROZEN_R0_READINESS_SHA256,
        ds1_after_r0_composition_retry::FROZEN_R0_E2B_SHA256,
        ds1_after_r0_composition_retry::FROZEN_DS1_SHA256,
        ds1_after_r0_composition_retry::FROZEN_RESULTS_DIGEST
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
