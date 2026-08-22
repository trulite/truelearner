#![allow(dead_code)]

use std::env;

#[path = "../ssa0_3_precommit_support.rs"]
mod ssa0_3_precommit_support;

use ssa0_3_precommit_support::{run, Stage};

fn option_usize(value: Option<usize>) -> String {
    value.map_or_else(|| "none".to_string(), |item| item.to_string())
}

fn option_i16(value: Option<i16>) -> String {
    value.map_or_else(|| "none".to_string(), |item| item.to_string())
}

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--probe".to_string());
    let stage = match argument.as_str() {
        "--probe" => Stage::Probe,
        "--micro" => Stage::Micro,
        "--gate" => Stage::Gate,
        "--definitive" => {
            eprintln!(
                "SSA0.3 DEVELOPMENT ONLY: definitive execution is forbidden; M6/M7 unchanged"
            );
            std::process::exit(2);
        }
        _ => {
            eprintln!("usage: ssa0_3_precommit_support [--probe|--micro|--gate]");
            std::process::exit(2);
        }
    };

    let report = run(stage);
    let classification = report
        .classification
        .map_or("CONTINUE", |value| value.label());
    println!(
        "row_type,protocol,stage,claim_eligible,M6_authoritative,M7_exists,family,condition,variant,target,target_supporters,competitor_supporters,winner,expected_winner,commitment_tick,duplicate_exact,permanent_fingerprint,start_fingerprint,trace_fingerprint,end_fingerprint,work,status,classification"
    );
    println!(
        "summary,{},{},false,true,false,,,,,,,,,,,,,,,,{},{},{}",
        ssa0_3_precommit_support::PROTOCOL,
        report.stage.label(),
        if report.passed { "PASS" } else { "FAIL" },
        classification,
        if report.passed {
            "ORDERED_PASS"
        } else {
            "STOP"
        },
    );
    for row in &report.rows {
        println!(
            "case,{},{},false,true,false,{},{},{},{},{},{},{},{},{},{},{:016x},{:016x},{:016x},{:016x},{},{},{}",
            ssa0_3_precommit_support::PROTOCOL,
            report.stage.label(),
            row.family,
            row.condition,
            row.variant,
            row.target,
            row.target_supporters,
            row.competitor_supporters,
            option_usize(row.winner),
            option_usize(row.expected_winner),
            option_i16(row.commitment_tick),
            row.duplicate_exact,
            row.permanent_fingerprint,
            row.start_fingerprint,
            row.trace_fingerprint,
            row.end_fingerprint,
            row.work,
            if row.passed { "PASS" } else { "FAIL" },
            classification,
        );
    }
    println!("controls={:?}", report.controls);
    println!(
        "causal temporal_distinction={} static_count_effect={} comparable_delivery_effect={}",
        report.temporal_distinction, report.static_count_effect, report.comparable_delivery_effect
    );
    println!(
        "lineage frozen_parent={} authoritative_M6={} parent_protocol_sha256={} parent_implementation_sha256={} parent_runner_sha256={}",
        ssa0_3_precommit_support::FROZEN_PARENT,
        ssa0_3_precommit_support::AUTHORITATIVE_M6,
        ssa0_3_precommit_support::PARENT_PROTOCOL_SHA256,
        ssa0_3_precommit_support::PARENT_IMPLEMENTATION_SHA256,
        ssa0_3_precommit_support::PARENT_RUNNER_SHA256,
    );
    if !report.passed {
        std::process::exit(1);
    }
}
