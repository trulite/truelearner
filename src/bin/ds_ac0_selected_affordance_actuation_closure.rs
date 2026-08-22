#![allow(dead_code)]

use std::env;

#[path = "../ds_ac0_selected_affordance_actuation_closure.rs"]
mod ac0;
#[path = "../research_runtime.rs"]
mod research_runtime;

use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-AC0: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_ac0_selected_affordance_actuation_closure [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = ac0::run(mode);
    println!(
        "protocol={} mode={} claim_eligible={} enabling_only={} pass={} M1_authoritative={} M2_exists={} source={:?}",
        ac0::PROTOCOL,
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
            "seed={} contexts={} choices={:?} roots={} handles={} executions={} arrow_traversals={} distinct={} blocked={} permuted={} changed_binding={} skipped={} stale={} layout={} persistent_bytes={} pass={}",
            seed.seed,
            seed.contexts,
            seed.choices,
            seed.roots_before_choice,
            seed.handles_before_choice,
            seed.executions,
            seed.arrow_traversals,
            seed.distinct_aftermaths,
            seed.blocked_route_abstentions,
            seed.permuted_handle_transfers,
            seed.changed_binding_changes,
            seed.skipped_execution_abstentions,
            seed.stale_handle_abstentions,
            seed.layout_transfers,
            seed.incremental_persistent_bytes,
            seed.passed,
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M1={} A1={} M1_parent={} collapse={} protocol_hash={}",
        ac0::EXACT_PARENT,
        ac0::PROTOCOL_COMMIT,
        ac0::AUTHORITATIVE_M1,
        ac0::FROZEN_A1_SHA256,
        ac0::FROZEN_M1_PARENT_SHA256,
        ac0::FROZEN_COLLAPSE_SHA256,
        ac0::FROZEN_PROTOCOL_SHA256,
    );
    if !report.passed {
        std::process::exit(1);
    }
}
