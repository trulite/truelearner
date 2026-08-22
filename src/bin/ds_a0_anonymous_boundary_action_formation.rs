#![allow(dead_code)] // Path-included frozen support modules expose unused helpers in this binary.

use std::env;

#[path = "../ds_a0_anonymous_boundary_action_formation.rs"]
mod ds_a0_anonymous_boundary_action_formation;
#[path = "../ds_e0_anonymous_event_formation.rs"]
mod ds_e0_anonymous_event_formation;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds_a0_anonymous_boundary_action_formation::run;
use research_runtime::HarnessMode;

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match argument.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-A0 DEVELOPMENT: definitive execution is forbidden");
            std::process::exit(2);
        }
        _ => {
            eprintln!("usage: ds_a0_anonymous_boundary_action_formation [--micro|--gate]");
            std::process::exit(2);
        }
    };
    let report = run(mode);
    println!(
        "{} mode={} audit={} claim_eligible={} M0_authoritative={} enabling_ancestor_only={} prior_DS1_collapse_stage={} M1_exists={} protocol={}",
        report.label,
        report.mode,
        if report.passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.m0_authoritative,
        report.enabling_ancestor_only,
        report.prior_ds1_collapse_stage,
        report.m1_exists,
        report.protocol,
    );
    println!(
        "stages A1={} A2={} A3={} A4={} B1={} B2={} B3={} first_collapse={}",
        report.a1,
        report.a2,
        report.a3,
        report.a4,
        report.b1,
        report.b2,
        report.b3,
        report.first_collapse.as_deref().unwrap_or("NONE"),
    );
    println!(
        "source semantic_opcodes={} evaluator_selection={} hidden_executor={} DS1_choose_calls={} DS1_apply_calls={} post_action_consequence_paths={} executors={} bridge_constructors={} frozen_sources_untouched={}",
        report.source_audit.semantic_opcode_sites,
        report.source_audit.evaluator_selection_sites,
        report.source_audit.hidden_executor_sites,
        report.source_audit.ds1_choose_calls,
        report.source_audit.ds1_apply_calls,
        report.source_audit.post_action_consequence_paths,
        report.source_audit.executor_definitions,
        report.source_audit.bridge_constructor_definitions,
        report.source_audit.frozen_sources_untouched,
    );
    for seed in &report.seeds {
        println!(
            "seed={} acquisition={} evaluation={} templates={} fingerprint={:016x} formed_routes={} handles={} one_to_one_roots={} physical_paths={} arrow_steps={} distinct_effect_pairs={} DS1_choose={} DS1_apply={} consequence_paths={} controls={} persistent_bytes={} temporary_peak_bytes={} physical_work={}",
            seed.seed,
            seed.acquisition_episodes,
            seed.evaluation_episodes,
            seed.templates,
            seed.learner_fingerprint,
            seed.formed_routes,
            seed.exposed_handles,
            seed.one_to_one_roots,
            seed.physical_execution_paths,
            seed.arrow_path_steps,
            seed.distinct_effect_pairs,
            seed.ds1_choose_calls,
            seed.ds1_apply_calls,
            seed.post_action_consequence_paths,
            seed.controls.passed(),
            seed.work.persistent_bytes,
            seed.work.temporary_peak_bytes,
            seed.work.physical_work(),
        );
    }
    println!(
        "frozen DS-E0={} frozen_composition={} frozen_DS1={} exact_parent={} authoritative_M0={} outcome={}",
        ds_a0_anonymous_boundary_action_formation::FROZEN_DS_E0_SHA256,
        ds_a0_anonymous_boundary_action_formation::FROZEN_DS1_COMPOSITION_SHA256,
        ds_a0_anonymous_boundary_action_formation::FROZEN_DS1_LEARNER_SHA256,
        ds_a0_anonymous_boundary_action_formation::EXACT_PARENT,
        ds_a0_anonymous_boundary_action_formation::AUTHORITATIVE_M0,
        report.label,
    );
    if !report.passed {
        std::process::exit(1);
    }
}
