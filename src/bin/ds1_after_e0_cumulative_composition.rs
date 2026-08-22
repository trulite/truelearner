use std::env;

use organism_v0::ds1_after_e0_cumulative_composition::run;
use organism_v0::research_runtime::HarnessMode;

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
            eprintln!("usage: ds1_after_e0_cumulative_composition [--micro|--gate]");
            std::process::exit(2);
        }
    };
    let report = run(mode);
    println!(
        "{} {} audit={} claim_eligible={} M0_authoritative={} enabling_parent={} M1_exists={} protocol={}",
        report.label,
        report.mode,
        if report.audit_passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.m0_authoritative,
        report.enabling_parent,
        report.m1_exists,
        report.protocol,
    );
    println!(
        "stages 0={} 1={} 2={} 3={} 4={} 5={} 6={} 7={} 8={} 9={} 10={}",
        report.exact_lineage_and_fingerprints,
        report.ds_e0_event_formation,
        report.e0_b_serialization,
        report.frozen_ds1_consumption,
        report.anonymous_action_alternatives,
        report.selected_action_execution,
        report.ordinary_consequence,
        report.ds1_acquisition,
        report.transfer,
        report.invalidation_reopening,
        report.functional_recovery,
    );
    println!(
        "action_surface choose_defs={} choose_calls={} apply_defs={} apply_calls={} frozen_choice_defs={} frozen_choice_calls={} e0_proposals={} propagation_surfaces={} e0_bool_callbacks={} owned_reports={} exposed_action_pairs={} m0_execution_surfaces={} m0_ds1_compatible={} choice_to_execution={} post_action_consequence={}",
        report.action_surface.ds1_choose_definitions,
        report.action_surface.ds1_choose_reachable_calls,
        report.action_surface.ds1_apply_consequence_definitions,
        report.action_surface.ds1_apply_consequence_reachable_calls,
        report.action_surface.ds1_frozen_choice_definitions,
        report.action_surface.ds1_frozen_choice_read_only_calls,
        report.action_surface.e0_candidate_proposal_sites,
        report.action_surface.e0_propagation_observation_surfaces,
        report.action_surface.e0_formation_bool_consequence_callbacks,
        report.action_surface.composition_owned_report_surfaces,
        report.action_surface.composition_exposed_action_pair_values,
        report.action_surface.m0_correspondence_execution_surfaces,
        report.action_surface.m0_ds1_compatible_execution_surfaces,
        report.action_surface.ds1_choice_to_physical_execution_paths,
        report.action_surface.natural_post_action_consequence_paths,
    );
    for seed in &report.seeds {
        println!(
            "CUMULATIVE DS1 DEVELOPMENT seed={} E0-A={} E0-B={} DS1_consumed={} actual_actions={} action_execution=BLANK consequence=BLANK acquisition=BLANK transfer=BLANK invalidation=BLANK recovery=BLANK E0_persistent_bytes={} E0_temporary_peak_bytes={} DS1_read_only_invocations={}",
            seed.seed,
            seed.e0_a_ready,
            seed.e0_b_ready,
            seed.ds1_neighborhood_consumed,
            seed.actual_anonymous_actions_available,
            seed.ledger.e0_persistent_bytes,
            seed.ledger.e0_temporary_peak_bytes,
            seed.ledger.ds1_read_only_invocations,
        );
    }
    println!(
        "frozen_ds1_sha256={} ds_e0_source_sha256={} first_collapse={} outcome=CUMULATIVE DS1 DEVELOPMENT COLLAPSE AT {}",
        report.frozen_ds1_sha256,
        report.ds_e0_source_sha256,
        report.first_collapse,
        report.first_collapse,
    );
    if !report.audit_passed {
        std::process::exit(1);
    }
}
