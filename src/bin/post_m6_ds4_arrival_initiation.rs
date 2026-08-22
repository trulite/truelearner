use organism_v0::post_m6_ds4_arrival_initiation::{
    definitive_rejected, run_development, run_probe_retry, run_probe_v1, DevelopmentReport,
};
use organism_v0::research_runtime::HarnessMode;

fn print_development(report: DevelopmentReport) {
    println!("# Post-M6 DS4 arrival-initiation {} result", report.mode);
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
    println!("- M6 authoritative: `{}`", report.m6_authoritative);
    println!("- M7 exists: `{}`", report.m7_exists);
    println!("- DS5 eligible: `{}`", report.ds5_eligible);
    println!("- First collapse: `{}`", report.first_collapse);
    println!(
        "- Learners ready/single-role/total: `{}/{}/{}`",
        report.ready_learners, report.single_role_learners, report.learners
    );
    println!(
        "- Mean competence episode: `{:.3}`",
        report.average_competence_millis as f64 / 1_000.0
    );
    println!("- Learned event activity: `{}`", report.learned_event_activity);
    println!("- Selection/recurrence: `{}/{}`", report.selections, report.recurrences);
    println!("- Consequence/update: `{}/{}`", report.consequences, report.updates);
    println!("- M6 observations: `{}`", report.credit_observations);
    println!(
        "- Held-out correct/explicit/quiescent/total: `{}/{}/{}/{}`",
        report.held_out_correct, report.explicit, report.quiescent, report.held_out_total
    );
    println!("- Serialization positions: `{}/6`", report.positions);
    println!("- Physical work: `{}`", report.physical_work);
    println!(
        "- Held-out non-plastic M3/P4/M6: `{}/{}/{}`",
        report.m3_nonplastic, report.p4_nonplastic, report.m6_nonplastic
    );
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
        eprintln!("post-M6 DS4 arrival-initiation: definitive execution is forbidden");
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
            "usage: post_m6_ds4_arrival_initiation [--probe-v1|--probe-retry|--micro|--gate|--definitive]"
        );
        std::process::exit(2);
    }
    let report = run_probe_v1();
    println!("# Post-M6 DS4 arrival-initiation PROBE v1 result");
    println!();
    println!("Outcome: **EXPECTED DEVELOPMENT NEGATIVE**.");
    println!();
    println!("- Protocol: `{}`", report.protocol);
    println!("- Seed: `{}`", report.seed);
    println!("- Claim eligible: `{}`", report.claim_eligible);
    println!("- Expected negative frozen: `{}`", report.expected_negative);
    println!("- Exact authoritative M6: `{}`", report.exact_m6);
    println!(
        "- Immutable old DS4 negative: `{}`",
        report.immutable_old_negative
    );
    println!("- Frozen protocol: `{}`", report.protocol_frozen);
    println!(
        "- Physical arrival path: `{}`",
        report.physical_arrival_path
    );
    println!(
        "- Learned event activity: `{}`",
        report.learned_event_activity
    );
    println!(
        "- Occurrence selections: `{}`",
        report.occurrence_selections
    );
    println!(
        "- Semantic feedback calls: `{}`",
        report.semantic_feedback_calls
    );
    println!(
        "- M6 differential links: `{}`",
        report.m6_differential_links
    );
    println!("- Lawful updates: `{}`", report.lawful_updates);
    println!("- First collapse: `{}`", report.first_collapse);
    if !report.expected_negative {
        std::process::exit(1);
    }
}
