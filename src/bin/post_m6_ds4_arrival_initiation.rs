use organism_v0::post_m6_ds4_arrival_initiation::{definitive_rejected, run_probe_v1};

fn main() {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "--probe-v1".to_string());
    if mode == "--definitive" {
        assert!(definitive_rejected());
        eprintln!("post-M6 DS4 arrival-initiation: definitive execution is forbidden");
        return;
    }
    if mode != "--probe-v1" {
        eprintln!("usage: post_m6_ds4_arrival_initiation [--probe-v1|--definitive]");
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
