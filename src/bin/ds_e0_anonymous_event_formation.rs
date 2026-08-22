use std::env;

use organism_v0::ds_e0_anonymous_event_formation::run;
use organism_v0::research_runtime::HarnessMode;

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--micro".to_string());
    let mode = match argument.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-E0: definitive execution is forbidden by preregistration");
            std::process::exit(2);
        }
        _ => {
            eprintln!("usage: ds_e0_anonymous_event_formation [--micro|--gate]");
            std::process::exit(2);
        }
    };
    let report = run(mode);
    println!(
        "{} {}: {} claim_eligible={} M0_authoritative={} M1_exists={} protocol={} frozen_ds1_sha256={}",
        report.label,
        report.mode,
        if report.passed { "PASS" } else { "FAIL" },
        report.claim_eligible,
        report.m0_authoritative,
        report.m1_exists,
        report.protocol,
        report.frozen_ds1_sha256,
    );
    for seed in &report.seeds {
        println!(
            "DS-E0 DEVELOPMENT seed={} pass={} E0-A={}/{} E0-B={}/{} same_timing={} same_propagation={} relabel={} allocation={} layout={} ambiguous_abstain={} timing_abstain={} propagation_abstain={} no_structure_abstain={} random_resisted={} misleading_not_competent={} proximity_baseline={}/{} invalidations={} reopenings={} reconsolidated={} shapes={} persistent_bytes={} temporary_peak_bytes={} retained_occurrences={} retained_memberships={} fingerprint={:016x} organism_work={} serialized_once_count={} ds1_probe={}",
            seed.seed,
            seed.passed,
            seed.e0_a_formed,
            seed.e0_a_presentations,
            seed.e0_b_exact_copies,
            seed.e0_a_presentations * 6,
            seed.same_timing_formed,
            seed.same_propagation_formed,
            seed.relabel_formed,
            seed.allocation_formed,
            seed.layout_formed,
            seed.ambiguous_abstentions,
            seed.shuffled_timing_abstentions,
            seed.shuffled_propagation_abstentions,
            seed.no_structure_abstentions,
            seed.random_consequence_resisted,
            seed.misleading_evidence_not_competent,
            seed.proximity_baseline_successes,
            seed.e0_a_presentations,
            seed.invalidations,
            seed.reopenings,
            seed.exact_two_and_reconsolidated,
            seed.persistent_shapes,
            seed.persistent_bytes,
            seed.temporary_peak_bytes,
            seed.retained_occurrences,
            seed.retained_memberships,
            seed.fingerprint,
            seed.work.raw_relation_comparisons
                + seed.work.triples_enumerated
                + seed.work.canonical_permutations
                + seed.work.persistent_shape_comparisons
                + seed.work.proposals
                + seed.work.physical_propagations
                + seed.work.consequence_updates
                + seed.work.temporary_formations
                + seed.work.serializations,
            seed.work.serializations,
            seed.frozen_ds1_consumption_probe,
        );
    }
    println!(
        "E0-A={} E0-B={} first_collapse={} outcome={}",
        report.e0_a_outcome,
        report.e0_b_outcome,
        report.first_collapse,
        if report.passed {
            "DS-E0 DEVELOPMENT IMPLEMENTATION READY"
        } else {
            "DS-E0 DEVELOPMENT COLLAPSE"
        }
    );
    if !report.passed {
        std::process::exit(1);
    }
}
