#![allow(dead_code)]

use std::env;

#[path = "../ds_d3_anonymous_consequence_contrast.rs"]
mod ds_d3_anonymous_consequence_contrast;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds_d3_anonymous_consequence_contrast::run;
use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-D3: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_d3_anonymous_consequence_contrast [--micro|--gate]");
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
            "seed={} roots={} recurrent_slot={} direction={:?} observations={} consequence_spikes={} consequence_routes={} D3_work={} A1_work={} persistent_bytes={} temporary_peak={} retained_occurrences={} retained_handles={} semantic_fields={} DS1_calls={} DS1_updates={} controls={} pass={} detail={:?}",
            seed.seed,
            seed.roots,
            seed.recurrent_slot,
            seed.direction,
            seed.observations,
            seed.consequence_firings,
            seed.consequence_routes,
            seed.work.organism_work(),
            seed.a1_work,
            seed.persistent_bytes,
            seed.temporary_peak_bytes,
            seed.retained_occurrences,
            seed.retained_handles,
            seed.semantic_fields,
            seed.ds1_calls,
            seed.ds1_updates,
            seed.controls.passed(),
            seed.passed,
            seed.controls
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M0={} D2={} D2_handoff={} parent_retry={} parent_handoff={} DS1={} results={}",
        ds_d3_anonymous_consequence_contrast::EXACT_PARENT,
        ds_d3_anonymous_consequence_contrast::PROTOCOL_COMMIT,
        ds_d3_anonymous_consequence_contrast::AUTHORITATIVE_M0,
        ds_d3_anonymous_consequence_contrast::FROZEN_D2_SHA256,
        ds_d3_anonymous_consequence_contrast::FROZEN_D2_HANDOFF_SHA256,
        ds_d3_anonymous_consequence_contrast::FROZEN_PARENT_RETRY_SHA256,
        ds_d3_anonymous_consequence_contrast::FROZEN_PARENT_HANDOFF_SHA256,
        ds_d3_anonymous_consequence_contrast::FROZEN_DS1_SHA256,
        ds_d3_anonymous_consequence_contrast::FROZEN_RESULTS_DIGEST
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
