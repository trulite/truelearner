#![allow(dead_code)]

use std::env;

#[path = "../ds_ir0_dependency_invalidation_reopening.rs"]
mod ir0;
#[path = "../research_runtime.rs"]
mod research_runtime;

use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-IR0: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_ir0_dependency_invalidation_reopening [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = ir0::run(mode);
    println!(
        "protocol={} mode={} claim_eligible={} enabling_only={} pass={} M1_authoritative={} M2_exists={} source={:?}",
        ir0::PROTOCOL,
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
            "seed={} contexts={} changed={} compatible={} compatible_preserved={} invalidations={} reopenings={} reopened_exec={} historical={} ambiguous={} layout={} persistent_bytes={} pass={}",
            seed.seed,
            seed.contexts,
            seed.changed_lifecycles,
            seed.compatible_uses,
            seed.compatible_preservations,
            seed.invalidations,
            seed.reopenings,
            seed.reopened_executions,
            seed.historical_returns,
            seed.ambiguous_preservations,
            seed.layout_transfers,
            seed.persistent_bytes,
            seed.passed,
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M1={} RT0={} CP0={} A1={} parent_hash={} protocol_hash={}",
        ir0::EXACT_PARENT,
        ir0::PROTOCOL_COMMIT,
        ir0::AUTHORITATIVE_M1,
        ir0::FROZEN_RT0_SHA256,
        ir0::FROZEN_CP0_SHA256,
        ir0::FROZEN_A1_SHA256,
        ir0::FROZEN_PARENT_SHA256,
        ir0::FROZEN_PROTOCOL_SHA256,
    );
    if !report.passed {
        std::process::exit(1);
    }
}
