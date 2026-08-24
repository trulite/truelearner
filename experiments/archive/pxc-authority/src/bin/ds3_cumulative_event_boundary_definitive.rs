#![allow(dead_code)]

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

#[path = "../ds3_cumulative_event_boundary_definitive.rs"]
mod definitive;
#[path = "../research_runtime.rs"]
mod research_runtime;

const DEFAULT_CSV: &str = "results/ds3_cumulative_event_boundary_definitive.csv";
const DEFAULT_MD: &str = "results/ds3_cumulative_event_boundary_definitive.md";

fn results_tree_digest() -> Option<String> {
    let output = Command::new("bash")
        .args([
            "-lc",
            "find results -type f -print0 | sort -z | xargs -0 sha256sum | sha256sum",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()?
        .split_whitespace()
        .next()
        .map(str::to_string)
}

fn results_tree_is_frozen() -> bool {
    results_tree_digest().as_deref() == Some(definitive::FROZEN_RESULTS_TREE_SHA256)
}

fn csv_escape(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn csv(report: &definitive::Report) -> String {
    let mut output = concat!(
        "protocol,mode,claim_eligible,matrix_passed,cell_id,base_seed,cell_passed,",
        "duplicate_deterministic,first_collapse,acquisition_m2_work,",
        "acquisition_observations,candidate_comparisons,held_out_spans,",
        "held_out_used_learned,held_out_acquisition_observations,",
        "generic_mature_work,learned_mature_work,chunks,persistent_bytes,",
        "control_1,control_2,control_3,control_4,control_5,control_6,",
        "control_7,control_8,control_9,control_10,control_11,control_12\n"
    )
    .to_string();
    for cell in &report.cells {
        let mut fields = vec![
            definitive::PROTOCOL.to_string(),
            report.mode.clone(),
            report.claim_eligible.to_string(),
            report.passed.to_string(),
            cell.cell_id.to_string(),
            cell.base_seed.to_string(),
            cell.passed.to_string(),
            cell.duplicate_deterministic.to_string(),
            csv_escape(&cell.first_collapse),
            cell.acquisition_m2_work.to_string(),
            cell.acquisition_observations.to_string(),
            cell.candidate_comparisons.to_string(),
            cell.held_out_spans.to_string(),
            cell.held_out_used_learned.to_string(),
            cell.held_out_acquisition_observations.to_string(),
            cell.generic_mature_work.to_string(),
            cell.learned_mature_work.to_string(),
            cell.chunk_count.to_string(),
            cell.persistent_bytes.to_string(),
        ];
        fields.extend(
            cell.controls
                .iter()
                .map(|control| control.passed.to_string()),
        );
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

fn markdown(report: &definitive::Report) -> String {
    let verdict = if report.passed { "PASS" } else { "FAIL" };
    let authority = if report.m3_authoritative { "M3" } else { "M2" };
    let held_out_spans = report
        .cells
        .iter()
        .map(|cell| cell.held_out_spans)
        .sum::<usize>();
    let controls_passed = report
        .cells
        .iter()
        .flat_map(|cell| &cell.controls)
        .filter(|control| control.passed)
        .count();
    let mut output = format!(
        "# DS3 cumulative event-boundary definitive result\n\n\
         - Verdict: **{verdict}**\n\
         - Claim eligible: `{}`\n\
         - Authoritative cumulative ancestor: **{authority}**\n\
         - M3 exists: `{}`\n\
         - DS4 cumulative eligible: `{}`\n\
         - Protocol: `{}`\n\
         - Exact frozen development parent: `{}`\n\
         - Matrix: 16 blank-start cells, 8 acquisition streams and 16 held-out streams per cell\n\
         - Held-out reconstructed spans: `{held_out_spans}/512`\n\
         - Passed numbered controls: `{controls_passed}/192`\n\n\
         | Cell | Base seed | Spans | Learned uses | M2 work | Generic | Learned | Chunks | Bytes | Replay | Result | First collapse |\n\
         |---:|---:|---:|---:|---:|---:|---:|---:|---:|:---:|:---:|---|\n",
        report.claim_eligible,
        report.m3_exists,
        report.ds4_cumulative_eligible,
        definitive::PROTOCOL,
        definitive::EXACT_DEVELOPMENT_PARENT,
    );
    for cell in &report.cells {
        let result = if cell.passed { "PASS" } else { "FAIL" };
        output.push_str(&format!(
            "| {} | {} | {}/32 | {}/32 | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            cell.cell_id,
            cell.base_seed,
            cell.held_out_spans,
            cell.held_out_used_learned,
            cell.acquisition_m2_work,
            cell.generic_mature_work,
            cell.learned_mature_work,
            cell.chunk_count,
            cell.persistent_bytes,
            cell.duplicate_deterministic,
            result,
            cell.first_collapse,
        ));
    }
    output.push_str("\n## Frozen interpretation\n\n");
    if report.passed {
        output.push_str(
            "The byte-frozen isolated DS3 mechanism reconstructed event boundaries on the cumulative M2 substrate using learned correspondence, interaction-role, causal-direction, physical-execution, and invalidation/reopening activity. M3 is the authoritative cumulative ancestor, and cumulative DS4 is eligible. Program-priority decisions are outside this matrix's evidentiary scope.\n",
        );
    } else {
        output.push_str(
            "The cumulative DS3 matrix failed conjunctively. M2 remains authoritative, M3 is absent, and cumulative DS4 is not eligible. No rescue or rerun is permitted.\n",
        );
        for cell in report.cells.iter().filter(|cell| !cell.passed) {
            output.push_str(&format!(
                "\n- Cell {} first collapse: {}\n",
                cell.cell_id, cell.first_collapse
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

fn write_definitive(
    report: &definitive::Report,
    csv_path: &Path,
    md_path: &Path,
) -> io::Result<()> {
    if csv_path.exists() || md_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "definitive output already exists",
        ));
    }
    create_new(csv_path, &csv(report))?;
    create_new(md_path, &markdown(report))
}

fn print_report(report: &definitive::Report) {
    println!(
        "protocol={} mode={} claim_eligible={} pass={} M2_authoritative={} M3_exists={} M3_authoritative={} DS4_eligible={} cells={} source={:?}",
        definitive::PROTOCOL,
        report.mode,
        report.claim_eligible,
        report.passed,
        report.m2_authoritative,
        report.m3_exists,
        report.m3_authoritative,
        report.ds4_cumulative_eligible,
        report.cells.len(),
        report.source,
    );
    for cell in &report.cells {
        println!(
            "cell={} seed={} pass={} replay={} spans={} learned_uses={} heldout_acquire={} m2_work={} acquisition_observations={} comparisons={} generic={} learned={} chunks={} bytes={} collapse={}",
            cell.cell_id,
            cell.base_seed,
            cell.passed,
            cell.duplicate_deterministic,
            cell.held_out_spans,
            cell.held_out_used_learned,
            cell.held_out_acquisition_observations,
            cell.acquisition_m2_work,
            cell.acquisition_observations,
            cell.candidate_comparisons,
            cell.generic_mature_work,
            cell.learned_mature_work,
            cell.chunk_count,
            cell.persistent_bytes,
            cell.first_collapse,
        );
        for control in &cell.controls {
            println!(
                "cell={} control={} name={} pass={} diagnostic={}",
                cell.cell_id, control.number, control.name, control.passed, control.diagnostic
            );
        }
    }
}

fn artifact_paths() -> (PathBuf, PathBuf) {
    (
        env::var_os("DS3_DEFINITIVE_CSV")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CSV)),
        env::var_os("DS3_DEFINITIVE_MD")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_MD)),
    )
}

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--audit".to_string());
    let digest_ok = results_tree_is_frozen();
    match arg.as_str() {
        "--audit" => {
            let report = definitive::run_audit(digest_ok);
            print_report(&report);
            if !report.passed {
                std::process::exit(1);
            }
        }
        "--definitive" => {
            let (csv_path, md_path) = artifact_paths();
            if csv_path.exists() || md_path.exists() {
                eprintln!("definitive output already exists");
                std::process::exit(2);
            }
            if !digest_ok {
                eprintln!(
                    "pre-existing results tree digest mismatch: observed={:?} expected={}",
                    results_tree_digest(),
                    definitive::FROZEN_RESULTS_TREE_SHA256
                );
                std::process::exit(2);
            }
            eprintln!(
                "DS3 definitive outcome begins with cell 0; the one-shot evidence is now spent"
            );
            let report = definitive::run_definitive(true);
            if let Err(error) = write_definitive(&report, &csv_path, &md_path) {
                eprintln!(
                    "definitive matrix completed but write-once serialization failed: {error}"
                );
                std::process::exit(3);
            }
            print_report(&report);
            if !report.passed {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("usage: ds3_cumulative_event_boundary_definitive [--audit|--definitive]");
            std::process::exit(2)
        }
    }
}
