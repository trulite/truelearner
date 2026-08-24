#![allow(dead_code)]

use std::env;

#[path = "../ds1_after_c0_composition_retry.rs"]
mod ds1_after_c0_composition_retry;
#[path = "../ds_c0_anonymous_credit_coupling.rs"]
mod ds_c0_anonymous_credit_coupling;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds1_after_c0_composition_retry::run;
use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS1-after-C0: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds1_after_c0_composition_retry [--micro|--gate]");
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
        "stages={:?} first_collapse_stage={:?} first_collapse={} source={:?}",
        report.stages, report.first_collapse_stage, report.first_collapse, report.source
    );
    for seed in &report.seeds {
        println!(
            "seed={} roots={} handles={} choice={} choose_calls={} route_executions={} evidence_fields={} eligibility={} couplings={} polarity_fields={} DS1_updates={} E0_work={} A1_work={} R0_primary_work={} R0_parent_audit_work={} C0_primary_work={} C0_control_work={} C0_persistent_bytes={} temporary_peak={} paths={:?}",
            seed.seed,
            seed.roots,
            seed.handles,
            seed.choice,
            seed.choose_calls,
            seed.route_executions,
            seed.evidence_fields,
            seed.eligibility_cells,
            seed.couplings,
            seed.polarity_fields,
            seed.paths.runtime_ds1_updates,
            seed.e0_work,
            seed.a1_work,
            seed.r0_primary_work,
            seed.r0_parent_audit_work,
            seed.c0_primary_work,
            seed.c0_control_work,
            seed.c0_persistent_bytes,
            seed.temporary_peak,
            seed.paths
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M0={} C0={} C0_readiness={} DS1={} results={}",
        ds1_after_c0_composition_retry::EXACT_PARENT,
        ds1_after_c0_composition_retry::PROTOCOL_COMMIT,
        ds1_after_c0_composition_retry::AUTHORITATIVE_M0,
        ds1_after_c0_composition_retry::FROZEN_C0_SHA256,
        ds1_after_c0_composition_retry::FROZEN_C0_READINESS_SHA256,
        ds1_after_c0_composition_retry::FROZEN_DS1_SHA256,
        ds1_after_c0_composition_retry::FROZEN_RESULTS_DIGEST
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
