#![allow(dead_code)]

use std::env;

#[path = "../research_runtime.rs"]
mod research_runtime;
#[path = "../ssa0_spatiotemporal_affordance.rs"]
mod ssa0_spatiotemporal_affordance;

use research_runtime::HarnessMode;
use ssa0_spatiotemporal_affordance::run;

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match argument.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("SSA0 DEVELOPMENT ONLY: definitive execution is forbidden; M6 unchanged");
            std::process::exit(2);
        }
        _ => {
            eprintln!("usage: ssa0_spatiotemporal_affordance [--micro|--gate]");
            std::process::exit(2);
        }
    };
    let report = run(mode);
    println!(
        "row_type,protocol,mode,claim_eligible,M6_authoritative,M7_exists,SSA0_A,SSA0_B,SSA0_C_invoked,SSA0_1,SSA0_2,controls,classification,start_fingerprint,trace_fingerprint,end_fingerprint,work,support_a,support_b,trials,realized_a,realized_b,none,permuted_physics,factor,first,mirrored,status"
    );
    println!(
        "summary,{},{},{},{},{},{},{},{},{},{},{},{},{:016x},{:016x},{:016x},{},,,,,,,,,,,{}",
        ssa0_spatiotemporal_affordance::PROTOCOL,
        report.mode,
        report.claim_eligible,
        report.m6_authoritative,
        report.m7_exists,
        report.exact_replay,
        report.ssa0_b,
        report.ssa0_c_invoked,
        report.ssa0_1,
        report.ssa0_2,
        report.controls.passed(),
        report.classification.label(),
        report.exact_start_fingerprint,
        report.exact_trace_fingerprint,
        report.exact_end_fingerprint,
        report.exact_work,
        if report.passed { "PASS" } else { "FAIL" },
    );
    for row in &report.support_rows {
        println!(
            "support,{},{},false,true,false,,,,,,,,{:016x},,,,{},{},{},{},{},{},{},,,,,{}",
            ssa0_spatiotemporal_affordance::PROTOCOL,
            report.mode,
            row.permanent_fingerprint,
            row.support_a,
            row.support_b,
            row.trials,
            row.realized_a,
            row.realized_b,
            row.none,
            row.permuted_physics,
            if row.duplicate_exact { "PASS" } else { "FAIL" },
        );
    }
    for row in &report.factors {
        println!(
            "factor,{},{},false,true,false,,,,,,,,,,,,,,,,,,,{}, {:?},{:?},{}",
            ssa0_spatiotemporal_affordance::PROTOCOL,
            report.mode,
            row.factor,
            row.first,
            row.mirrored,
            if row.passed { "PASS" } else { "FAIL" },
        );
    }
    println!("controls={:?}", report.controls);
    println!("crossed={:?}", report.crossed);
    println!(
        "lineage authoritative_M6={} protocol_commit={} protocol_sha256={} frozen_A1_sha256={} frozen_M6_sha256={} frozen_runtime_sha256={}",
        ssa0_spatiotemporal_affordance::AUTHORITATIVE_M6,
        ssa0_spatiotemporal_affordance::PROTOCOL_COMMIT,
        ssa0_spatiotemporal_affordance::PROTOCOL_SHA256,
        ssa0_spatiotemporal_affordance::FROZEN_A1_SHA256,
        ssa0_spatiotemporal_affordance::FROZEN_M6_SHA256,
        ssa0_spatiotemporal_affordance::FROZEN_RUNTIME_SHA256,
    );
    if !report.passed {
        std::process::exit(1);
    }
}
