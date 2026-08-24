#![allow(dead_code)]

use std::env;

#[path = "../ds_ac0_selected_affordance_actuation_closure.rs"]
mod ac0;
#[path = "../ds_ap0_aftermath_plasticity_activation.rs"]
mod ap0;
#[path = "../ds_cp0_consequence_probation_coupling.rs"]
mod cp0;
#[path = "../ds_ir0_dependency_invalidation_reopening.rs"]
mod ir0;
#[path = "../ds1_boundary_role_cumulative_definitive.rs"]
mod m1_definitive;
#[path = "../ds2_after_ap0_mechanistic_retry.rs"]
mod post_ap0_retry;
#[path = "../ds2_after_cp0_mechanistic_retry.rs"]
mod post_cp0_retry;
#[path = "../ds2_after_rt0_mechanistic_retry.rs"]
mod post_rt0_retry;
#[path = "../ds2_cumulative_m1_mechanistic_probe.rs"]
mod prior_probe;
#[path = "../ds2_after_ac0_mechanistic_retry.rs"]
mod prior_retry;
#[path = "../research_runtime.rs"]
mod research_runtime;
#[path = "../ds2_after_ir0_mechanistic_retry.rs"]
mod retry;
#[path = "../ds_rt0_retained_direction_execution.rs"]
mod rt0;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--probe".to_string());
    if arg == "--definitive" {
        eprintln!("DS2 after IR0: definitive execution is forbidden");
        std::process::exit(2)
    }
    if arg != "--probe" {
        eprintln!("usage: ds2_after_ir0_mechanistic_retry [--probe]");
        std::process::exit(2)
    }
    let report = retry::run();
    println!(
        "protocol={} claim_eligible={} complete={} audit={} M1_authoritative={} M2_exists={} IR0_seeds={} changed={} invalidations={} reopenings={} reopened_exec={} historical={} compatible={} ambiguous={} first_collapse_stage={:?} first_collapse={} source={:?}",
        report.protocol,
        report.claim_eligible,
        report.development_complete,
        report.audit_passed,
        report.m1_authoritative,
        report.m2_exists,
        report.ir0_seed_count,
        report.changed_lifecycles,
        report.invalidations,
        report.reopenings,
        report.reopened_executions,
        report.historical_returns,
        report.compatible_preservations,
        report.ambiguous_preservations,
        report.first_collapse_stage,
        report.first_collapse,
        report.source,
    );
    for (stage, status) in report.stages.iter().enumerate() {
        println!("stage={} status={}", stage, status);
    }
    println!(
        "lineage parent={} protocol_commit={} M1={} prior_retry={} IR0={} IR0_readiness={} protocol_hash={}",
        retry::EXACT_PARENT,
        retry::PROTOCOL_COMMIT,
        retry::AUTHORITATIVE_M1,
        retry::FROZEN_PRIOR_RETRY_SHA256,
        retry::FROZEN_IR0_SHA256,
        retry::FROZEN_IR0_READINESS_SHA256,
        retry::FROZEN_PROTOCOL_SHA256,
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
