use std::fmt::Write;

fn main() {
    let report = organism_v0::local_plasticity::run_p2_experiment();
    organism_v0::local_plasticity::print_p2_report(&report);
    if let Some(path) = std::env::args().nth(1) {
        std::fs::write(path, csv(&report)).expect("write P2 local plasticity CSV");
    }
    std::process::exit(i32::from(!report.passed));
}

fn csv(report: &organism_v0::local_plasticity::P2Report) -> String {
    let mut output = String::from(
        "section,condition,total_cells,active_irrelevant,successful_seeds,total_seeds,correct,total,role_cells,program_arrows,active_touches,dormant_touches,local_encounters,created,released,used,peak_probationary,first_success_episode,competence_episode\n",
    );
    writeln!(
        output,
        "P2a,forward-and-reverse,0,0,{},{},0,0,0,{},{},0,0,{},{},{},0,,",
        report.lookup.forward_seeds + report.lookup.reverse_seeds,
        report.lookup.total_seeds * 2,
        report.lookup.average_surviving,
        0,
        report.lookup.average_created,
        report
            .lookup
            .average_created
            .saturating_sub(report.lookup.average_surviving),
        report.lookup.average_used
    )
    .unwrap();
    writeln!(
        output,
        "P2b,encoding-transfer,0,0,{},{},{},{},{},0,0,0,0,0,0,0,0,,",
        report.roles.successful_seeds,
        report.roles.total_seeds,
        report.roles.transferred_encodings,
        report.roles.transferred_total,
        report.roles.learned_role_cells
    )
    .unwrap();
    for (condition, result) in [
        ("real", &report.integrated),
        ("shuffled", &report.shuffled),
        ("random", &report.random),
    ] {
        writeln!(
            output,
            "P2c,{},0,{},{},{},{},{},{:.3},{:.3},0,0,0,{},{},{},{},{:?},{:?}",
            condition,
            8,
            result.competent_seeds,
            result.total_seeds,
            result.held_out_correct,
            result.held_out_total,
            result.average_roles,
            result.average_surviving_program,
            result.average_created,
            result.average_released,
            result.average_used,
            result.average_peak_probationary,
            result.average_first_success_episode,
            result.average_competence_episode
        )
        .unwrap();
    }
    for point in &report.dormant_scaling {
        writeln!(
            output,
            "scaling,dormant,{},8,0,0,{},{},0,0,{}, {},{},0,0,0,0,,",
            point.total_cells,
            point.held_out_correct,
            point.held_out_total,
            point.active_touches,
            point.dormant_touches,
            point.local_encounters
        )
        .unwrap();
    }
    for point in &report.active_scaling {
        writeln!(
            output,
            "scaling,active,0,{},0,0,{},{},0,0,{},0,{},{},{},0,0,,",
            point.active_irrelevant,
            point.held_out_correct,
            point.held_out_total,
            point.active_touches,
            point.local_encounters,
            point.created,
            point.released
        )
        .unwrap();
    }
    for point in &report.slot_diagnostic {
        writeln!(
            output,
            "diagnostic,slots-{},0,0,{},{},0,0,0,0,0,0,0,0,0,0,0,,",
            point.slots_per_cell, point.successful_lookup_seeds, report.lookup.total_seeds
        )
        .unwrap();
    }
    output
}
