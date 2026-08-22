#![allow(dead_code)]

use std::env;

#[path = "../ds_c0_anonymous_credit_coupling.rs"]
mod ds_c0_anonymous_credit_coupling;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds_c0_anonymous_credit_coupling::run;
use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-C0: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_c0_anonymous_credit_coupling [--micro|--gate]");
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
        "stages={:?} first_collapse={:?} source={:?}",
        report.stages, report.first_collapse, report.source
    );
    for seed in &report.seeds {
        println!(
            "seed={} roots={} handles={} choice={} choose_calls={} DS1_updates={} eligibility={} couplings={} polarity_fields={} evidence_fields={} controls={} C0_primary_work={} C0_total_control_work={} E0_work={} A1_work={} R0_primary_work={} R0_parent_audit_work={} persistent_bytes={} temporary_peak={}",
            seed.seed,
            seed.roots,
            seed.handles,
            seed.choice,
            seed.choose_calls,
            seed.ds1_updates,
            seed.eligibility_cells,
            seed.couplings,
            seed.coupling_polarity_fields,
            seed.evidence_fields,
            seed.controls.passed(),
            seed.primary_work.organism_work(),
            seed.total_c0_work.organism_work(),
            seed.e0_work,
            seed.a1_work,
            seed.r0_primary_work,
            seed.r0_parent_audit_work,
            seed.c0_persistent_bytes,
            seed.temporary_peak_bytes
        );
        println!(
            "seed={} controls={:?} primary_work={:?} total_c0_work={:?}",
            seed.seed, seed.controls, seed.primary_work, seed.total_c0_work
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M0={} R0={} retry={} handoff={} DS1={} results={}",
        ds_c0_anonymous_credit_coupling::EXACT_PARENT,
        ds_c0_anonymous_credit_coupling::PROTOCOL_COMMIT,
        ds_c0_anonymous_credit_coupling::AUTHORITATIVE_M0,
        ds_c0_anonymous_credit_coupling::FROZEN_R0_SHA256,
        ds_c0_anonymous_credit_coupling::FROZEN_PARENT_RETRY_SHA256,
        ds_c0_anonymous_credit_coupling::FROZEN_PARENT_HANDOFF_SHA256,
        ds_c0_anonymous_credit_coupling::FROZEN_DS1_SHA256,
        ds_c0_anonymous_credit_coupling::FROZEN_RESULTS_DIGEST
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
