#![allow(dead_code)]

use std::env;

#[path = "../ds3_cumulative_event_boundary_port.rs"]
mod port;
#[path = "../research_runtime.rs"]
mod research_runtime;

use research_runtime::HarnessMode;

fn print_report(report: &port::Report) {
    println!(
        "label={} protocol={} mode={} claim_eligible={} development_ready={} M2_authoritative={} M3_exists={} reconstructability={} functional_adequacy={} replay={} work_attributed={} collapse_stage={:?} collapse={}",
        report.label,
        report.protocol,
        report.mode,
        report.claim_eligible,
        report.development_ready,
        report.m2_authoritative,
        report.m3_exists,
        report.reconstructability,
        report.functional_adequacy,
        report.duplicate_deterministic,
        report.work_attributed,
        report.first_collapse_stage,
        report.first_collapse,
    );
    println!(
        "m2_work={} acquisition_observations={} candidate_comparisons={} generic_mature_work={} learned_mature_work={} held_out_used_learned={} persistent_bytes={} chunks={} held_out_seeds={} source={:?}",
        report.acquisition_m2_work,
        report.acquisition_observations,
        report.candidate_comparisons,
        report.generic_mature_work,
        report.learned_mature_work,
        report.held_out_used_learned,
        report.persistent_bytes,
        report.chunk_count,
        report.held_out_seed_count,
        report.source,
    );
    for (index, stage) in report.stages.iter().enumerate() {
        println!("stage={} status={}", index, stage);
    }
    for control in &report.controls {
        println!(
            "control={} name={} pass={} diagnostic={}",
            control.number, control.name, control.passed, control.diagnostic
        );
    }
    println!(
        "lineage parent={} protocol_commit={} expectation_commit={} mechanism_install={} M1={} mechanism={} A1={} AC0={} IR0={} protocol_hash={} expectation_hash={}",
        port::EXACT_PARENT,
        port::PROTOCOL_COMMIT,
        port::EXPECTATION_COMMIT,
        port::MECHANISM_INSTALL_COMMIT,
        port::AUTHORITATIVE_M1,
        port::FROZEN_MECHANISM_SHA256,
        port::FROZEN_A1_SHA256,
        port::FROZEN_AC0_SHA256,
        port::FROZEN_IR0_SHA256,
        port::FROZEN_PROTOCOL_SHA256,
        port::FROZEN_EXPECTATION_SHA256,
    );
}

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => HarnessMode::Definitive,
        _ => {
            eprintln!("usage: ds3_cumulative_event_boundary_port [--micro|--gate|--definitive]");
            std::process::exit(2)
        }
    };
    let report = port::run(mode);
    print_report(&report);
    if mode == HarnessMode::Definitive {
        eprintln!("DS3 cumulative definitive is locked pending separate matrix preregistration");
        std::process::exit(2)
    }
    if !report.development_ready {
        std::process::exit(1)
    }
}
