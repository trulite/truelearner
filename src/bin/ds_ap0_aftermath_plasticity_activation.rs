#![allow(dead_code)]

use std::env;

#[path = "../ds_ac0_selected_affordance_actuation_closure.rs"]
mod ac0;
#[path = "../ds_ap0_aftermath_plasticity_activation.rs"]
mod ap0;
#[path = "../research_runtime.rs"]
mod research_runtime;

use research_runtime::HarnessMode;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-AP0: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_ap0_aftermath_plasticity_activation [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let report = ap0::run(mode);
    println!(
        "protocol={} mode={} claim_eligible={} enabling_only={} pass={} M1_authoritative={} M2_exists={} source={:?}",
        ap0::PROTOCOL,
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
            "seed={} contexts={} selected_proposals={} support_updates={} templates={} alternate={} alternate_support_updates={} blocked={} permuted={} stale={} skipped={} layout={} unrelated={} probationary_bytes={} pass={}",
            seed.seed,
            seed.contexts,
            seed.selected_proposals,
            seed.selected_support_updates,
            seed.selected_probationary_templates,
            seed.alternate_proposals,
            seed.alternate_support_updates,
            seed.blocked_abstentions,
            seed.permuted_transfers,
            seed.stale_abstentions,
            seed.skipped_abstentions,
            seed.layout_transfers,
            seed.unrelated_abstentions,
            seed.probationary_bytes,
            seed.passed,
        );
    }
    println!(
        "lineage parent={} protocol_commit={} M1={} A1={} AC0={} parent_hash={} protocol_hash={}",
        ap0::EXACT_PARENT,
        ap0::PROTOCOL_COMMIT,
        ap0::AUTHORITATIVE_M1,
        ap0::FROZEN_A1_SHA256,
        ap0::FROZEN_AC0_SHA256,
        ap0::FROZEN_PARENT_SHA256,
        ap0::FROZEN_PROTOCOL_SHA256,
    );
    if !report.passed {
        std::process::exit(1);
    }
}
