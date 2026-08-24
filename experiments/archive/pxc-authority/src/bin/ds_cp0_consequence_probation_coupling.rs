#![allow(dead_code)]

use std::env;

#[path = "../ds_cp0_consequence_probation_coupling.rs"]
mod cp0;
#[path = "../research_runtime.rs"]
mod research_runtime;

use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-CP0: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_cp0_consequence_probation_coupling [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = cp0::run(mode);
    println!(
        "protocol={} mode={} claim_eligible={} enabling_only={} pass={} M1_authoritative={} M2_exists={} source={:?}",
        cp0::PROTOCOL,
        report.mode,
        report.claim_eligible,
        report.enabling_only,
        report.passed,
        report.m1_authoritative,
        report.m2_exists,
        report.source,
    );
    for seed in &report.seeds {
        println!(
            "seed={} contexts={} differentiated={} reversed={} ambiguous={} variable={} shuffled={} suppressed={} removed={} fresh={} layout={} permuted={} persistent_bytes={} pass={}",
            seed.seed,
            seed.contexts,
            seed.differentiated,
            seed.reverse_differentiated,
            seed.ambiguous_abstentions,
            seed.variable_abstentions,
            seed.shuffled_abstentions,
            seed.suppressed_abstentions,
            seed.removed_abstentions,
            seed.fresh_transfers,
            seed.layout_transfers,
            seed.permuted_transfers,
            seed.persistent_bytes,
            seed.passed,
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M1={} D3={} A1={} parent_hash={} protocol_hash={}",
        cp0::EXACT_PARENT,
        cp0::PROTOCOL_COMMIT,
        cp0::AUTHORITATIVE_M1,
        cp0::FROZEN_D3_SHA256,
        cp0::FROZEN_A1_SHA256,
        cp0::FROZEN_PARENT_SHA256,
        cp0::FROZEN_PROTOCOL_SHA256,
    );
    if !report.passed {
        std::process::exit(1);
    }
}
