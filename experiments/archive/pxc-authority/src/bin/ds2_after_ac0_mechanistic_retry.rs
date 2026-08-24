#![allow(dead_code)]

use std::env;

#[path = "../ds_ac0_selected_affordance_actuation_closure.rs"]
mod ac0;
#[path = "../ds1_boundary_role_cumulative_definitive.rs"]
mod m1_definitive;
#[path = "../ds2_cumulative_m1_mechanistic_probe.rs"]
mod prior_probe;
#[path = "../research_runtime.rs"]
mod research_runtime;
#[path = "../ds2_after_ac0_mechanistic_retry.rs"]
mod retry;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--probe".to_string());
    if arg == "--definitive" {
        eprintln!("DS2 after AC0: definitive execution is forbidden");
        std::process::exit(2)
    }
    if arg != "--probe" {
        eprintln!("usage: ds2_after_ac0_mechanistic_retry [--probe]");
        std::process::exit(2)
    }
    let report = retry::run();
    println!(
        "protocol={} claim_eligible={} audit={} M1_authoritative={} M2_exists={} AC0_seeds={} first_collapse_stage={:?} first_collapse={} source={:?}",
        report.protocol,
        report.claim_eligible,
        report.audit_passed,
        report.m1_authoritative,
        report.m2_exists,
        report.ac0_seed_count,
        report.first_collapse_stage,
        report.first_collapse,
        report.source,
    );
    for (stage, status) in report.stages.iter().enumerate() {
        println!("stage={} status={}", stage, status);
    }
    println!(
        "lineage parent={} protocol_commit={} M1={} prior_probe={} AC0={} AC0_readiness={} protocol_hash={}",
        retry::EXACT_PARENT,
        retry::PROTOCOL_COMMIT,
        retry::AUTHORITATIVE_M1,
        retry::FROZEN_PRIOR_PROBE_SHA256,
        retry::FROZEN_AC0_SHA256,
        retry::FROZEN_AC0_READINESS_SHA256,
        retry::FROZEN_PROTOCOL_SHA256,
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
