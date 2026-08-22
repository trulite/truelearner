#![allow(dead_code)]

use std::env;

#[path = "../ds1_after_e0_a0_a1_composition_retry.rs"]
mod ds1_after_e0_a0_a1_composition_retry;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds1_after_e0_a0_a1_composition_retry::run;
use research_runtime::HarnessMode;

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match argument.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("UNCHANGED DS1 RETRY: definitive execution is forbidden");
            std::process::exit(2);
        }
        _ => {
            eprintln!("usage: ds1_after_e0_a0_a1_composition_retry [--micro|--gate]");
            std::process::exit(2);
        }
    };
    let report = run(mode);
    println!(
        "{} mode={} audit={} claim_eligible={} M0_authoritative={} enabling_only={} M1_exists={} protocol={}",
        report.label,
        report.mode,
        if report.audit_passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.m0_authoritative,
        report.enabling_only,
        report.m1_exists,
        report.protocol,
    );
    println!(
        "stages={:?} first_collapse={}",
        report.stages, report.first_collapse
    );
    println!("source={:?}", report.source);
    for seed in &report.seeds {
        println!(
            "seed={} E0_target={} E0_exact={} same_event={} A1_candidates={} A1_templates={} A1_roots={} A1_structural={} A1_effects={} A1_handles={} choice_arity={} choice={:?} choose_calls={} selected_executions={} selected_spikes={} selected_arrows={} selected_mutations={} post_choice_events={} apply_updates={} controls={} E0_work={} A1_work={} DS1_work={} E0_bytes={} A1_bytes={} DS1_bytes={} temporary_peak={}",
            seed.seed,
            seed.e0.actual_target_events,
            seed.e0.exact_export_copy && seed.e0.exact_neighborhood_copy,
            seed.e0.same_target_for_ds1_and_a1,
            seed.a1.candidates,
            seed.a1.templates,
            seed.a1.installed_roots,
            seed.a1.structural_roots,
            seed.a1.unique_effects,
            seed.a1.handles,
            seed.choice_arity,
            seed.choice.choice,
            seed.choice.runtime_choose_calls,
            seed.paths.runtime_selected_executions,
            seed.a1.selected_spike_propagations,
            seed.a1.selected_arrow_traversals,
            seed.a1.selected_state_mutations,
            seed.paths.runtime_post_choice_evidence_events,
            seed.paths.runtime_apply_updates,
            seed.controls.passed_through_stage_six(),
            seed.e0.physical_work,
            seed.a1.organism_work + seed.permuted_a1.organism_work,
            seed.choice.comparisons
                + seed.choice.candidate_evaluations
                + seed.choice.proposals
                + seed.choice.runtime_credit_updates,
            seed.e0.persistent_bytes,
            seed.a1.persistent_bytes,
            seed.choice.persistent_bytes,
            seed.e0
                .temporary_bytes
                .max(seed.a1.temporary_peak_bytes)
                .max(seed.permuted_a1.temporary_peak_bytes),
        );
        println!("seed={} paths={:?}", seed.seed, seed.paths);
        println!("seed={} controls={:?}", seed.seed, seed.controls);
    }
    println!(
        "hashes parent={} protocol={} M0={} E0={} A0={} A1={} DS1={} M0_source={} compiled_M0={} A1_readiness={} results_digest={}",
        ds1_after_e0_a0_a1_composition_retry::EXACT_PARENT,
        ds1_after_e0_a0_a1_composition_retry::PROTOCOL_COMMIT,
        ds1_after_e0_a0_a1_composition_retry::AUTHORITATIVE_M0,
        ds1_after_e0_a0_a1_composition_retry::FROZEN_DS_E0_SHA256,
        ds1_after_e0_a0_a1_composition_retry::FROZEN_DS_A0_SHA256,
        ds1_after_e0_a0_a1_composition_retry::FROZEN_DS_A1_SHA256,
        ds1_after_e0_a0_a1_composition_retry::FROZEN_DS1_SHA256,
        ds1_after_e0_a0_a1_composition_retry::FROZEN_M0_SHA256,
        ds1_after_e0_a0_a1_composition_retry::FROZEN_COMPILED_M0_SHA256,
        ds1_after_e0_a0_a1_composition_retry::FROZEN_A1_READINESS_SHA256,
        ds1_after_e0_a0_a1_composition_retry::FROZEN_RESULTS_DIGEST,
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
