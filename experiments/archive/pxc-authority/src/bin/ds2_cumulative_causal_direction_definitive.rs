#![allow(dead_code)]

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[path = "../ds_ac0_selected_affordance_actuation_closure.rs"]
mod ac0;
#[path = "../ds_ap0_aftermath_plasticity_activation.rs"]
mod ap0;
#[path = "../ds_cp0_consequence_probation_coupling.rs"]
mod cp0;
#[path = "../ds2_cumulative_causal_direction_definitive.rs"]
mod definitive;
#[path = "../ds2_after_ir0_mechanistic_retry.rs"]
mod development;
#[path = "../ds_ir0_dependency_invalidation_reopening.rs"]
mod ir0;
#[path = "../ds1_boundary_role_cumulative_definitive.rs"]
mod m1_definitive;
#[path = "../ds2_after_ap0_mechanistic_retry.rs"]
mod post_ap0_retry;
#[path = "../ds2_after_cp0_mechanistic_retry.rs"]
mod post_cp0_retry;
#[path = "../ds2_after_rt0_mechanistic_retry.rs"]
mod post_rt0_retry;
#[path = "../ds2_cumulative_m1_mechanistic_probe.rs"]
mod prior_probe;
#[path = "../ds2_after_ac0_mechanistic_retry.rs"]
mod prior_retry;
#[path = "../research_runtime.rs"]
mod research_runtime;
#[path = "../ds_rt0_retained_direction_execution.rs"]
mod rt0;

use definitive::{Cell, Report};

const DEFAULT_CSV: &str = "results/ds2_cumulative_causal_direction_definitive.csv";
const DEFAULT_MD: &str = "results/ds2_cumulative_causal_direction_definitive.md";

fn csv_header() -> &'static str {
    "protocol,mode,claim_eligible,passed,seed,contexts,changed_lifecycles,compatible_uses,compatible_preservations,invalidations,reopenings,reopened_executions,historical_returns,ambiguous_preservations,layout_transfers,persistent_bytes,duplicate_deterministic,first_collapse\n"
}

fn csv_row(report: &Report, cell: &Cell) -> String {
    let fields = [
        definitive::PROTOCOL.to_string(),
        report.mode.clone(),
        report.claim_eligible.to_string(),
        cell.passed.to_string(),
        cell.seed.to_string(),
        cell.contexts.to_string(),
        cell.changed_lifecycles.to_string(),
        cell.compatible_uses.to_string(),
        cell.compatible_preservations.to_string(),
        cell.invalidations.to_string(),
        cell.reopenings.to_string(),
        cell.reopened_executions.to_string(),
        cell.historical_returns.to_string(),
        cell.ambiguous_preservations.to_string(),
        cell.layout_transfers.to_string(),
        cell.persistent_bytes.to_string(),
        cell.duplicate_deterministic.to_string(),
        cell.first_collapse.clone(),
    ];
    format!("{}\n", fields.join(","))
}

fn markdown(report: &Report) -> String {
    let verdict = if report.passed { "PASS" } else { "FAIL" };
    let authority = if report.m2_authoritative { "M2" } else { "M1" };
    let mut output = format!(
        "# DS2 cumulative causal-direction de-supply definitive result\n\n\
         - Verdict: **{verdict}**\n\
         - Claim eligible: `{}`\n\
         - Authoritative cumulative ancestor: **{authority}**\n\
         - Protocol: `{}`\n\
         - Exact frozen parent: `{}`\n\
         - Matrix: 16 seeds × 4 contexts, changed/compatible/ambiguous/layout conditions, historical return, duplicate replay\n\n\
         | Seed | Changed | Invalidated | Reopened | Executed | Historical | Compatible | Ambiguous | Layout | Bytes | Replay | Result |\n\
         |---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|:---:|:---:|\n",
        report.claim_eligible,
        definitive::PROTOCOL,
        definitive::EXACT_PARENT,
    );
    for cell in &report.cells {
        let result = if cell.passed { "PASS" } else { "FAIL" };
        output.push_str(&format!(
            "| {} | {}/4 | {}/4 | {}/4 | {}/4 | {}/4 | {}/4 | {}/4 | {}/4 | {} | {} | {} |\n",
            cell.seed,
            cell.changed_lifecycles,
            cell.invalidations,
            cell.reopenings,
            cell.reopened_executions,
            cell.historical_returns,
            cell.compatible_preservations,
            cell.ambiguous_preservations,
            cell.layout_transfers,
            cell.persistent_bytes,
            cell.duplicate_deterministic,
            result,
        ));
    }
    output.push_str("\n## Frozen interpretation\n\n");
    if report.passed {
        output.push_str(
            "Learner-visible causal-direction metadata was cumulatively de-supplied. The M1 interaction substrate acquired a role-relative direction through ordinary local proposal/probation and downstream consequence, retained and physically executed it on fresh occurrences, invalidated it when its physical dependency changed, reopened generic inference, and reused preserved historical structure on return. M2 is authoritative.\n",
        );
    } else {
        output.push_str(
            "The cumulative causal-direction matrix failed. M1 remains authoritative; no later scaffold repairs this result.\n",
        );
        for cell in report.cells.iter().filter(|cell| !cell.passed) {
            output.push_str(&format!(
                "\n- Seed {} first collapse: {}\n",
                cell.seed, cell.first_collapse
            ));
        }
    }
    output
}

fn create_new(path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()
}

fn write_definitive(report: &Report, csv_path: &Path, md_path: &Path) -> io::Result<()> {
    if csv_path.exists() || md_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "definitive output already exists",
        ));
    }
    let mut csv = csv_header().to_string();
    for cell in &report.cells {
        csv.push_str(&csv_row(report, cell));
    }
    create_new(csv_path, &csv)?;
    create_new(md_path, &markdown(report))
}

fn print_report(report: &Report) {
    println!(
        "protocol={} mode={} claim_eligible={} pass={} M1_authoritative={} M2_exists={} M2_authoritative={} cells={} source={:?}",
        definitive::PROTOCOL,
        report.mode,
        report.claim_eligible,
        report.passed,
        report.m1_authoritative,
        report.m2_exists,
        report.m2_authoritative,
        report.cells.len(),
        report.source,
    );
    for cell in &report.cells {
        println!(
            "seed={} contexts={} changed={} retained={} compatible={} invalidated={} reopened={} executed={} historical={} ambiguous={} layout={} bytes={} replay={} pass={} collapse={}",
            cell.seed,
            cell.contexts,
            cell.changed_lifecycles,
            cell.compatible_uses,
            cell.compatible_preservations,
            cell.invalidations,
            cell.reopenings,
            cell.reopened_executions,
            cell.historical_returns,
            cell.ambiguous_preservations,
            cell.layout_transfers,
            cell.persistent_bytes,
            cell.duplicate_deterministic,
            cell.passed,
            cell.first_collapse,
        );
    }
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        None | Some("--audit") if args.len() <= 1 => {
            let report = definitive::run_audit();
            print_report(&report);
            if !report.passed {
                std::process::exit(1);
            }
        }
        Some("--definitive") if args.len() <= 3 => {
            let csv = PathBuf::from(args.get(1).map_or(DEFAULT_CSV, String::as_str));
            let md = PathBuf::from(args.get(2).map_or(DEFAULT_MD, String::as_str));
            if csv.exists() || md.exists() {
                eprintln!("refusing definitive run: an output path already exists");
                std::process::exit(2);
            }
            let report = definitive::run_definitive();
            print_report(&report);
            if let Err(error) = write_definitive(&report, &csv, &md) {
                eprintln!("failed to create write-once definitive artifacts: {error}");
                std::process::exit(2);
            }
            if !report.passed {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!(
                "usage: ds2_cumulative_causal_direction_definitive [--audit|--definitive [CSV [MD]]]"
            );
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_schema_is_frozen() {
        assert_eq!(csv_header().trim_end().split(',').count(), 18);
    }

    #[test]
    fn audit_cannot_claim_authority() {
        let report = definitive::run_audit();
        assert!(report.passed, "{report:#?}");
        assert!(!report.claim_eligible && !report.m2_authoritative);
    }
}
