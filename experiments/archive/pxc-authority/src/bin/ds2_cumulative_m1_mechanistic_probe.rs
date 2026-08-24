#![allow(dead_code)]

use std::env;

#[path = "../ds1_boundary_role_cumulative_definitive.rs"]
mod m1_definitive;
#[path = "../ds2_cumulative_m1_mechanistic_probe.rs"]
mod probe;
#[path = "../research_runtime.rs"]
mod research_runtime;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--probe".to_string());
    if arg == "--definitive" {
        eprintln!("cumulative DS2 mechanistic probe: definitive execution is forbidden");
        std::process::exit(2);
    }
    if arg != "--probe" {
        eprintln!("usage: ds2_cumulative_m1_mechanistic_probe [--probe]");
        std::process::exit(2);
    }
    let report = probe::run();
    println!(
        "protocol={} claim_eligible={} audit={} M1_authoritative={} M2_exists={} first_collapse_stage={:?} first_collapse={} source={:?}",
        report.protocol,
        report.claim_eligible,
        report.audit_passed,
        report.m1_authoritative,
        report.m2_exists,
        report.first_collapse_stage,
        report.first_collapse,
        report.source,
    );
    for (stage, status) in report.stages.iter().enumerate() {
        println!("stage={} status={}", stage, status);
    }
    println!(
        "lineage M1={} protocol_commit={} M1_core={} M1_parent={} result_csv={} result_md={} protocol_hash={}",
        probe::AUTHORITATIVE_M1,
        probe::PROTOCOL_COMMIT,
        probe::FROZEN_M1_CORE_SHA256,
        probe::FROZEN_M1_PARENT_SHA256,
        probe::FROZEN_M1_CSV_SHA256,
        probe::FROZEN_M1_MD_SHA256,
        probe::FROZEN_PROTOCOL_SHA256,
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
