#![allow(dead_code)]

use std::env;

#[path = "../ds1_after_e0_a0_composition_retry.rs"]
mod ds1_after_e0_a0_composition_retry;
#[path = "../ds_e0_anonymous_event_formation.rs"]
mod ds_e0_anonymous_event_formation;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds1_after_e0_a0_composition_retry::run;
use research_runtime::HarnessMode;

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match argument.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("CUMULATIVE DS1 DEVELOPMENT: definitive execution is forbidden");
            std::process::exit(2);
        }
        _ => {
            eprintln!("usage: ds1_after_e0_a0_composition_retry [--micro|--gate]");
            std::process::exit(2);
        }
    };
    let report = run(mode);
    println!(
        "{} mode={} audit={} claim_eligible={} M0_authoritative={} M1_exists={} protocol={}",
        report.label,
        report.mode,
        if report.audit_passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.m0_authoritative,
        report.m1_exists,
        report.protocol,
    );
    println!(
        "stages={:?} first_collapse={}",
        report.stages, report.first_collapse
    );
    println!(
        "paths choose_defs={} choose_edges={} choice_to_execution={} apply_defs={} apply_edges={} parent_consequence_paths={} leaks={}",
        report.paths.frozen_choose_definitions,
        report.paths.frozen_choose_call_edges,
        report.paths.choice_to_execution_edges,
        report.paths.apply_definitions,
        report.paths.apply_call_edges,
        report.paths.parent_consequence_paths,
        report.leaks.passed,
    );
    for seed in &report.seeds {
        println!(
            "seed={} E0_shapes={} E0_formed={} serialized={} A0_templates={} roots={} handles={} unique_roots={} prechoice_effects={} distinct_effects={} choice={:?} mature={} runtime_choose={} selected_executions={} arrow_steps={} spikes={} mutations={} apply_updates={} consequence_events={} permutation_control={:?} persistent_bytes={} temporary_peak_bytes={} physical_work={}",
            seed.seed,
            seed.e0.learned_shapes,
            seed.e0.learned_event_formed,
            seed.e0.serializations,
            seed.a0.templates,
            seed.a0.prebridge_roots,
            seed.a0.handles,
            seed.a0.unique_roots,
            seed.a0.prechoice_physical_effects,
            seed.a0.distinct_prechoice_effects,
            seed.choice.choice,
            seed.choice.mature,
            seed.paths.runtime_choose_calls,
            seed.paths.runtime_selected_executions,
            seed.a0.selected_arrow_steps,
            seed.a0.selected_spike_propagations,
            seed.a0.selected_state_mutations,
            seed.paths.runtime_apply_updates,
            seed.paths.runtime_consequence_visibility_events,
            seed.permutation_only_changes_actual_route,
            seed.e0.persistent_bytes + seed.choice.persistent_bytes + seed.a0.persistent_bytes,
            seed.e0.temporary_bytes.max(seed.a0.temporary_peak_bytes),
            seed.a0.physical_work,
        );
    }
    println!(
        "hashes parent={} DS_E0={} DS_A0={} marked_DS1={} M0={} compiled_M0={} prior_stage4={} authoritative_M0={} outcome={}",
        ds1_after_e0_a0_composition_retry::EXACT_PARENT,
        ds1_after_e0_a0_composition_retry::FROZEN_DS_E0_SHA256,
        ds1_after_e0_a0_composition_retry::FROZEN_DS_A0_SHA256,
        ds1_after_e0_a0_composition_retry::FROZEN_DS1_SHA256,
        ds1_after_e0_a0_composition_retry::FROZEN_M0_SHA256,
        ds1_after_e0_a0_composition_retry::FROZEN_COMPILED_M0_SHA256,
        ds1_after_e0_a0_composition_retry::PRIOR_STAGE_FOUR_SOURCE_SHA256,
        ds1_after_e0_a0_composition_retry::AUTHORITATIVE_M0,
        report.label,
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
