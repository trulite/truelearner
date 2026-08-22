use organism_v0::post_m7_ds5_closure_emission::{
    definitive_rejected, run_development, run_probe_retry, run_probe_v1, DevelopmentReport,
};
use organism_v0::research_runtime::HarnessMode;

fn print_development(report: DevelopmentReport) {
    println!("# Post-M7 DS5 closure-emission {} result", report.mode);
    println!();
    println!(
        "Outcome: **{}**.",
        if report.development_ready {
            "DEVELOPMENT POSITIVE"
        } else {
            "DEVELOPMENT NEGATIVE"
        }
    );
    println!();
    println!("- Protocol: `{}`", report.protocol);
    println!("- Claim eligible: `{}`", report.claim_eligible);
    println!("- Development ready: `{}`", report.development_ready);
    println!("- M7 authoritative: `{}`", report.m7_authoritative);
    println!("- M8 exists: `{}`", report.m8_exists);
    println!("- First collapse: `{}`", report.first_collapse);
    println!(
        "- Learners ready/single-role/total: `{}/{}/{}`",
        report.ready_learners, report.single_role_learners, report.learners
    );
    println!(
        "- Mean closure-role competence episode: `{:.3}`",
        report.average_competence_millis as f64 / 1_000.0
    );
    println!("- Learned M7 activations: `{}`", report.m7_activations);
    println!("- Anonymous selections: `{}`", report.selections);
    println!(
        "- Physical consequence/update: `{}/{}`",
        report.consequences, report.updates
    );
    println!("- M6 observations: `{}`", report.m6_observations);
    println!(
        "- Total physical boundary crossings: `{}`",
        report.crossings
    );
    println!(
        "- Held-out correct/quiescent/total: `{}/{}/{}`",
        report.held_out_correct, report.natural_quiescence, report.held_out_total
    );
    println!("- Serialization positions: `{}/6`", report.positions);
    println!("- Held-out depth classes: `{}`", report.depths);
    println!("- Physical work: `{}`", report.physical_work);
    println!(
        "- Held-out non-plastic M7/closure: `{}/{}`",
        report.m7_nonplastic, report.closure_nonplastic
    );
    println!("- Temporary state erased: `{}`", report.temporary_erased);
    println!("- Duplicate exact: `{}`", report.duplicate_exact);
    println!("- Source audit: `{:#?}`", report.source);
    println!();
    println!("| # | Control | Pass |");
    println!("|---:|---|:---:|");
    for control in report.controls {
        println!(
            "| {} | {} | {} |",
            control.number, control.name, control.passed
        );
    }
    if !report.development_ready {
        std::process::exit(1);
    }
}

fn main() {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "--probe-v1".to_string());
    if mode == "--definitive" {
        assert!(definitive_rejected());
        eprintln!("post-M7 DS5 closure-emission: definitive execution is forbidden");
        return;
    }
    if mode == "--probe-retry" {
        print_development(run_probe_retry());
        return;
    }
    if mode == "--micro" {
        print_development(run_development(HarnessMode::Micro));
        return;
    }
    if mode == "--gate" {
        print_development(run_development(HarnessMode::Gate));
        return;
    }
    if mode != "--probe-v1" {
        eprintln!(
            "usage: post_m7_ds5_closure_emission [--probe-v1|--probe-retry|--micro|--gate|--definitive]"
        );
        std::process::exit(2);
    }
    let report = run_probe_v1();
    println!("# Post-M7 DS5 closure-emission PROBE v1 result");
    println!();
    println!("Outcome: **EXPECTED DEVELOPMENT NEGATIVE**.");
    println!();
    println!("- Protocol: `{}`", report.protocol);
    println!("- Seed: `{}`", report.seed);
    println!("- Claim eligible: `{}`", report.claim_eligible);
    println!("- Expected negative frozen: `{}`", report.expected_negative);
    println!("- Exact authoritative M7: `{}`", report.exact_m7);
    println!("- Frozen protocol: `{}`", report.protocol_frozen);
    println!("- Frozen parts: `{}`", report.frozen_parts);
    println!(
        "- Physical closure path: `{}`",
        report.physical_closure_path
    );
    println!(
        "- Terminal-supervision sites: `{}`",
        report.terminal_supervision_sites
    );
    println!(
        "- Semantic-population sites: `{}`",
        report.semantic_population_sites
    );
    println!("- Lawful M6 links: `{}`", report.lawful_m6_links);
    println!("- Lawful updates: `{}`", report.lawful_updates);
    println!("- First collapse: `{}`", report.first_collapse);
    if !report.expected_negative {
        std::process::exit(1);
    }
}
