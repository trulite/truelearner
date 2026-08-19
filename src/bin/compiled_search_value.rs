use std::{env, fs, path::PathBuf};

use organism_v0::search_value;

fn main() {
    let output = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/s1_1_compiled_search_value.csv"));
    let report = search_value::run_compiled_experiment();
    let mut csv = String::from(
        "record,depth_or_scope,cases,exhaustive_expanded,guided_expanded,exhaustive_work,current_work,compiled_work,local_work,zero_cost_work\n",
    );
    for depth in &report.depth_reports {
        csv.push_str(&format!(
            "depth,{},{},{},{},{},{},{},{},{}\n",
            depth.required_depth,
            depth.cases,
            depth.exhaustive_expanded,
            depth.compiled_expanded,
            depth.exhaustive_total_work,
            depth.current_total_work,
            depth.compiled_total_work,
            depth.local_total_work,
            depth.zero_cost_total_work
        ));
    }
    csv.push_str(&format!(
        "reachable,all,{},{},{},{},{},{},{},{}\n",
        report.reachable_cases,
        report
            .depth_reports
            .iter()
            .map(|depth| depth.exhaustive_expanded)
            .sum::<usize>(),
        report
            .depth_reports
            .iter()
            .map(|depth| depth.compiled_expanded)
            .sum::<usize>(),
        report.reachable_exhaustive_total_work,
        report.reachable_current_total_work,
        report.reachable_compiled_total_work,
        report.reachable_local_total_work,
        report.reachable_zero_cost_total_work
    ));
    csv.push_str(&format!(
        "unreachable,all,{},,,{},{},{},{},\n",
        report.unreachable_cases,
        report.unreachable_exhaustive_total_work,
        report.unreachable_current_total_work,
        report.unreachable_compiled_total_work,
        report.unreachable_local_total_work
    ));
    fs::write(&output, csv).expect("write S1.1 compiled-value CSV");
    search_value::print_compiled_report(&report);
    println!("wrote compiled-value measurements to {}", output.display());
    println!("passed={}", report.passed);
}
