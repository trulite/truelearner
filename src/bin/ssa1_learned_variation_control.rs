use organism_v0::ssa1_learned_variation_control::{run_micro, run_probe};

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--micro") {
        let report = run_micro();
        println!("# SSA1 learned variation-control MICRO");
        println!();
        println!("- Classification: **{}**", report.classification);
        println!("- First collapse: `{}`", report.first_collapse);
        println!("- Duplicate exact: `{}`", report.passed);
        println!();
        for cell in report.cells {
            println!("## Seed {}", cell.seed);
            println!();
            println!("- World A: `{}`", cell.world_a);
            println!("- World B: `{}`", cell.world_b);
            for world in [&cell.world_c, &cell.world_d, &cell.world_e] {
                println!(
                    "- {}: `{}`; live `{:?} -> {:?}`; admissions `{:?} -> {:?}`; values `{:?} -> {:?}`; realized `{:?} -> {:?}`; observations `{} -> {}`; physical control `{}`",
                    world.name,
                    world.passed,
                    world.before.live_supporters,
                    world.after.live_supporters,
                    world.before.admissions,
                    world.after.admissions,
                    world.before.value_score,
                    world.after.value_score,
                    world.realized_before,
                    world.realized_after,
                    world.before.observations,
                    world.after.observations,
                    world.physical_control
                );
            }
            println!();
        }
        if !report.passed {
            std::process::exit(1);
        }
        return;
    }

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
