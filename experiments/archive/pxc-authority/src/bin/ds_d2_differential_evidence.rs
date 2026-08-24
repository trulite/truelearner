#![allow(dead_code)]

use std::env;

#[path = "../ds_d2_differential_evidence.rs"]
mod ds_d2_differential_evidence;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds_d2_differential_evidence::run;
use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-D2: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_d2_differential_evidence [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = run(mode);
    println!(
        "{} mode={} audit={} claim_eligible={} enabling_only={} M0_authoritative={} M1_exists={} protocol={} source={:?}",
        report.label,
        report.mode,
        if report.audit_passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.enabling_only,
        report.m0_authoritative,
        report.m1_exists,
        report.protocol,
        report.source
    );
    for seed in &report.seeds {
        println!(
            "seed={} roots={} selected={} direction={:?} unique={} equal_magnitude={} controls={} D2_work={} A1_work={} persistent_bytes={} temporary_peak={} DS1_calls={} DS1_updates={} pass={} detail={:?}",
            seed.seed,
            seed.roots,
            seed.selected,
            seed.direction,
            seed.unique_compatibility,
            seed.magnitude_equal,
            seed.controls.passed(),
            seed.primary_work.organism_work(),
            seed.a1_work,
            seed.persistent_bytes,
            seed.temporary_peak_bytes,
            seed.ds1_calls,
            seed.ds1_updates,
            seed.passed,
            seed.controls
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M0={} D1={} D1_handoff={} A1={} DS1={} results={}",
        ds_d2_differential_evidence::EXACT_PARENT,
        ds_d2_differential_evidence::PROTOCOL_COMMIT,
        ds_d2_differential_evidence::AUTHORITATIVE_M0,
        ds_d2_differential_evidence::FROZEN_D1_SOURCE_SHA256,
        ds_d2_differential_evidence::FROZEN_D1_HANDOFF_SHA256,
        ds_d2_differential_evidence::FROZEN_A1_SHA256,
        ds_d2_differential_evidence::FROZEN_DS1_SHA256,
        ds_d2_differential_evidence::FROZEN_RESULTS_DIGEST
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
