#![allow(dead_code)]

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[path = "../ds1_boundary_role_cumulative_definitive.rs"]
mod definitive;
#[path = "../research_runtime.rs"]
mod research_runtime;

use definitive::{Cell, Report};

const DEFAULT_CSV: &str = "results/ds1_boundary_role_cumulative_definitive.csv";
const DEFAULT_MD: &str = "results/ds1_boundary_role_cumulative_definitive.md";

fn csv_header() -> &'static str {
    "protocol,mode,claim_eligible,passed,seed,acquisition,events,two_roots,d3_directions,physical_directions,deliveries,updates,patterns,divergent_patterns,consequence_mature,evaluator_mature,held_out_attempts,held_out_successes,held_out_abstentions,reversed_consequence_mature,reversed_evaluator_mature,d3_work,learner_work,learner_bytes,fingerprint,duplicate_deterministic,first_collapse\n"
}

fn csv_row(report: &Report, cell: &Cell) -> String {
    let fields = [
        definitive::PROTOCOL.to_string(),
        report.mode.clone(),
        report.claim_eligible.to_string(),
        cell.passed.to_string(),
        cell.seed.to_string(),
        cell.acquisition.to_string(),
        cell.events.to_string(),
        cell.two_roots.to_string(),
        cell.d3_directions.to_string(),
        cell.physical_directions.to_string(),
        cell.deliveries.to_string(),
        cell.updates.to_string(),
        cell.patterns.to_string(),
        cell.divergent_patterns.to_string(),
        cell.consequence_mature.to_string(),
        cell.evaluator_mature.to_string(),
        cell.held_out_attempts.to_string(),
        cell.held_out_successes.to_string(),
        cell.held_out_abstentions.to_string(),
        cell.reversed_consequence_mature.to_string(),
        cell.reversed_evaluator_mature.to_string(),
        cell.d3_work.to_string(),
        cell.learner_work.to_string(),
        cell.learner_bytes.to_string(),
        format!("{:016x}", cell.fingerprint),
        cell.duplicate_deterministic.to_string(),
        cell.first_collapse.clone(),
    ];
    format!("{}\n", fields.join(","))
}

fn markdown(report: &Report) -> String {
    let verdict = if report.passed { "PASS" } else { "FAIL" };
    let authority = if report.m1_authoritative { "M1" } else { "M0" };
    let mut output = format!(
        "# DS1 cumulative boundary-role de-supply definitive result\n\n\
         - Verdict: **{verdict}**\n\
         - Claim eligible: `{}`\n\
         - Authoritative cumulative ancestor: **{authority}**\n\
         - Protocol: `{}`\n\
         - Exact frozen parent: `{}`\n\
         - Matrix: 8 seeds × 64 acquisition × 32 held-out, plus reversed world and duplicate replay\n\n\
         | Seed | Updates | Mature | Held-out | Reversed | D3 work | Learner work | Bytes | Replay | Result |\n\
         |---:|---:|---:|---:|---:|---:|---:|---:|:---:|:---:|\n",
        report.claim_eligible,
        definitive::PROTOCOL,
        definitive::EXACT_PARENT,
    );
    for cell in &report.cells {
        let result = if cell.passed { "PASS" } else { "FAIL" };
        output.push_str(&format!(
            "| {} | {}/{} | {}/4 | {}/{} | {}/4 (original {}) | {} | {} | {} | {} | {} |\n",
            cell.seed,
            cell.updates,
            cell.acquisition,
            cell.consequence_mature,
            cell.held_out_successes,
            cell.held_out_attempts,
            cell.reversed_consequence_mature,
            cell.reversed_evaluator_mature,
            cell.d3_work,
            cell.learner_work,
            cell.learner_bytes,
            cell.duplicate_deterministic,
            result,
        ));
    }
    output.push_str("\n## Frozen interpretation\n\n");
    if report.passed {
        output.push_str(
            "Boundary roles were reconstructed cumulatively from anonymous interaction and downstream physical consequences without supplied filler equality or semantic polarity. M1 is the authoritative cumulative ancestor.\n",
        );
    } else {
        output.push_str(
            "The cumulative boundary-role matrix failed. M0 remains authoritative; no later scaffold repairs this result.\n",
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
        "protocol={} mode={} claim_eligible={} pass={} M0_authoritative={} M1_exists={} M1_authoritative={} cells={} source={:?}",
        definitive::PROTOCOL,
        report.mode,
        report.claim_eligible,
        report.passed,
        report.m0_authoritative,
        report.m1_exists,
        report.m1_authoritative,
        report.cells.len(),
        report.source,
    );
    for cell in &report.cells {
        println!(
            "seed={} acquisition={} events={} roots={} directions={} physical={} delivered={} updates={} mature={}/4 evaluator={}/4 heldout={}/{} abstain={} reversed={}/4 reversed_evaluator={} D3_work={} learner_work={} bytes={} fingerprint={:016x} replay={} pass={} collapse={}",
            cell.seed,
            cell.acquisition,
            cell.events,
            cell.two_roots,
            cell.d3_directions,
            cell.physical_directions,
            cell.deliveries,
            cell.updates,
            cell.consequence_mature,
            cell.evaluator_mature,
            cell.held_out_successes,
            cell.held_out_attempts,
            cell.held_out_abstentions,
            cell.reversed_consequence_mature,
            cell.reversed_evaluator_mature,
            cell.d3_work,
            cell.learner_work,
            cell.learner_bytes,
            cell.fingerprint,
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
                "usage: ds1_boundary_role_cumulative_definitive [--audit|--definitive [CSV [MD]]]"
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
        assert_eq!(csv_header().trim_end().split(',').count(), 27);
    }

    #[test]
    fn audit_cannot_claim_authority() {
        let report = definitive::run_audit();
        assert!(report.passed, "{report:#?}");
        assert!(!report.claim_eligible && !report.m1_authoritative);
    }
}
