use organism_v0::ssa1_learned_variation_control::run_probe;

fn main() {
    let report = run_probe();
    println!("# SSA1 learned variation-control PROBE");
    println!();
    println!("- Passed: `{}`", report.passed);
    println!("- Claim eligible: `{}`", report.claim_eligible);
    println!(
        "- Frozen learning controls: `{}`",
        report.frozen_learning_controls
    );
    println!("- First collapse: `{}`", report.first_collapse);
    println!();
    for world in [&report.world_a, &report.world_b] {
        println!("## {}", world.name);
        println!();
        println!("- Passed: `{}`", world.passed);
        println!("- Live supporters: `{:?}`", world.landscape.live_supporters);
        println!("- Admissions: `{:?}`", world.landscape.admissions);
        println!("- Value score: `{:?}`", world.landscape.value_score);
        println!("- Realized routes: `{:?}`", world.realized);
        println!("- Unresolved: `{}`", world.unresolved);
        println!("- Duplicate exact: `{}`", world.duplicate_exact);
        println!();
    }
    if !report.passed {
        std::process::exit(1);
    }
}
