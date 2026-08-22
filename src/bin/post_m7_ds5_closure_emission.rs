use organism_v0::post_m7_ds5_closure_emission::run_probe_v1;

fn main() {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "--probe-v1".to_string());
    if mode == "--definitive" {
        eprintln!("post-M7 DS5 closure-emission: definitive execution is forbidden");
        return;
    }
    if mode != "--probe-v1" {
        eprintln!("usage: post_m7_ds5_closure_emission [--probe-v1|--definitive]");
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
