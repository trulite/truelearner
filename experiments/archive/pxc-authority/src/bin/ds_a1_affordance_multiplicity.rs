#![allow(dead_code)]

use std::env;

#[path = "../ds_a1_affordance_multiplicity.rs"]
mod ds_a1_affordance_multiplicity;
#[path = "../research_runtime.rs"]
mod research_runtime;

use ds_a1_affordance_multiplicity::run;
use research_runtime::HarnessMode;

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match argument.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-A1 DEVELOPMENT: definitive execution is forbidden");
            std::process::exit(2);
        }
        _ => {
            eprintln!("usage: ds_a1_affordance_multiplicity [--micro|--gate]");
            std::process::exit(2);
        }
    };
    let report = run(mode);
    println!(
        "{} mode={} audit={} claim_eligible={} M0_authoritative={} enabling_only={} M1_exists={} DS1_retry={} protocol={}",
        report.label,
        report.mode,
        if report.passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.m0_authoritative,
        report.enabling_only,
        report.m1_exists,
        report.ds1_retry,
        report.protocol,
    );
    println!(
        "stages={:?} first_collapse={}",
        report.stages, report.first_collapse
    );
    println!("source={:?}", report.source_audit);
    for seed in &report.seeds {
        println!(
            "seed={} candidates={} templates={} roots={} structural_unique={} unique_effects={} handles={} cells={} arrows={} E0_support_exports={} E0_target_events={} E0_fields_exact={} fresh={} controls={} organism_work={} evaluator_comparisons={} persistent_bytes={} temporary_peak_bytes={} maintenance={} carrying={}",
            seed.seed,
            seed.candidate_proposals,
            seed.consolidated_templates,
            seed.installed_roots,
            seed.structural_unique_roots,
            seed.unique_effects,
            seed.handles,
            seed.installed_cells,
            seed.installed_arrows,
            seed.provenance.support_exports,
            seed.provenance.actual_target_events,
            seed.provenance.exact_field_copy,
            seed.provenance.fresh_disjoint,
            seed.controls.passed(),
            seed.work.organism_work() + seed.provenance.frozen_e0_work,
            seed.work.evaluator_comparisons,
            seed.work.persistent_bytes + seed.provenance.frozen_e0_persistent_bytes,
            seed.work.temporary_peak_bytes.max(seed.provenance.frozen_e0_temporary_bytes),
            seed.work.maintenance_work,
            seed.work.carrying_work,
        );
        println!("seed={} controls={:?}", seed.seed, seed.controls);
    }
    println!(
        "hashes exact_parent={} protocol={} amendment={} DS_E0={} DS_A0={} marked_DS1={} prior_composition={} authoritative_M0={}",
        ds_a1_affordance_multiplicity::EXACT_PARENT,
        ds_a1_affordance_multiplicity::PROTOCOL_COMMIT,
        ds_a1_affordance_multiplicity::PROTOCOL_AMENDMENT,
        ds_a1_affordance_multiplicity::FROZEN_DS_E0_SHA256,
        ds_a1_affordance_multiplicity::FROZEN_DS_A0_SHA256,
        ds_a1_affordance_multiplicity::FROZEN_DS1_SHA256,
        ds_a1_affordance_multiplicity::PRIOR_COMPOSITION_SHA256,
        ds_a1_affordance_multiplicity::AUTHORITATIVE_M0,
    );
    if !report.passed {
        std::process::exit(1);
    }
}
