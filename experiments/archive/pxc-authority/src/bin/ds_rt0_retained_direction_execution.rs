#![allow(dead_code)]

use std::env;

#[path = "../research_runtime.rs"]
mod research_runtime;
#[path = "../ds_rt0_retained_direction_execution.rs"]
mod rt0;

use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-RT0: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_rt0_retained_direction_execution [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = rt0::run(mode);
    println!(
        "protocol={} mode={} claim_eligible={} enabling_only={} pass={} M1_authoritative={} M2_exists={} source={:?}",
        rt0::PROTOCOL,
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
            "seed={} contexts={} retained={} reversed={} ambiguous={} variable={} shuffled={} suppressed={} removed={} layout={} permuted={} persistent_bytes={} pass={}",
            seed.seed,
            seed.contexts,
            seed.retained_executions,
            seed.reversed_executions,
            seed.ambiguous_abstentions,
            seed.variable_abstentions,
            seed.shuffled_abstentions,
            seed.suppressed_abstentions,
            seed.removed_abstentions,
            seed.layout_transfers,
            seed.permuted_transfers,
            seed.persistent_bytes,
            seed.passed,
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M1={} CP0={} A1={} parent_hash={} protocol_hash={}",
        rt0::EXACT_PARENT,
        rt0::PROTOCOL_COMMIT,
        rt0::AUTHORITATIVE_M1,
        rt0::FROZEN_CP0_SHA256,
        rt0::FROZEN_A1_SHA256,
        rt0::FROZEN_PARENT_SHA256,
        rt0::FROZEN_PROTOCOL_SHA256,
    );
    if !report.passed {
        std::process::exit(1);
    }
}
