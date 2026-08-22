#![allow(dead_code)]

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

#[path = "../binding.rs"]
mod binding;
#[path = "../ds4_cumulative_request_start_definitive.rs"]
mod definitive;
#[path = "../research_runtime.rs"]
mod research_runtime;

const DEFAULT_CSV: &str = "results/ds4_cumulative_request_start_definitive.csv";
const DEFAULT_MD: &str = "results/ds4_cumulative_request_start_definitive.md";

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
        "first_collapse,competence_episode,request_roles,held_out_correct,held_out_total,",
        "explicit_answers,queues_empty,request_positions,m3_learned_uses,",
        "completion_activity,selection_activations,execution_activations,",
        "update_activations,m3_physical_work,p4_nonplastic,m3_nonplastic,",
        "duplicate_deterministic,control_1,control_2,control_3,control_4,control_5,",
        "control_6,control_7,control_8,control_9,control_10,control_11,control_12\n"
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
            csv_escape(&cell.first_collapse),
            cell.competence_episode.to_string(),
            cell.request_roles.to_string(),
            cell.held_out_correct.to_string(),
            cell.held_out_total.to_string(),
            cell.explicit_answers.to_string(),
            cell.queues_empty.to_string(),
            cell.request_positions.to_string(),
            cell.m3_learned_uses.to_string(),
            cell.completion_activity.to_string(),
            cell.selection_activations.to_string(),
            cell.execution_activations.to_string(),
            cell.update_activations.to_string(),
            cell.m3_physical_work.to_string(),
            cell.p4_nonplastic.to_string(),
            cell.m3_nonplastic.to_string(),
            cell.duplicate_deterministic.to_string(),
        ];
        fields.extend(cell.controls.iter().map(|control| control.passed.to_string()));
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

fn markdown(report: &definitive::Report) -> String {
    let verdict = if report.passed { "PASS" } else { "FAIL" };
    let authority = if report.m4_authoritative { "M4" } else { "M3" };
    let correct = report
        .cells
        .iter()
        .map(|cell| cell.held_out_correct)
        .sum::<usize>();
    let controls = report
        .cells
        .iter()
        .flat_map(|cell| &cell.controls)
        .filter(|control| control.passed)
        .count();
    let mut output = format!(
        "# DS4 cumulative request/start definitive result\n\n\
         - Verdict: **{verdict}**\n\
         - Claim eligible: `{}`\n\
         - Authoritative cumulative ancestor: **{authority}**\n\
         - M4 exists: `{}`\n\
         - DS5 cumulative eligible: `{}`\n\
         - Protocol: `{}`\n\
         - Exact development parent: `{}`\n\
         - Matrix: 16 blank-start cells; 32 held-out executions per cell\n\
         - Held-out executions: `{correct}/512`\n\
         - Passed numbered controls: `{controls}/192`\n\n\
         | Cell | Seed | Competence | Held-out | Positions | Learned uses | Completion | Select/update | M3 work | Replay | Result | Collapse |\n\
         |---:|---:|---:|---:|---:|---:|---:|---:|---:|:---:|:---:|---|\n",
        report.claim_eligible,
        report.m4_exists,
        report.ds5_cumulative_eligible,
        definitive::PROTOCOL,
        definitive::EXACT_DEVELOPMENT_PARENT,
    );
    for cell in &report.cells {
        let result = if cell.passed { "PASS" } else { "FAIL" };
        output.push_str(&format!(
            "| {} | {} | {} | {}/32 | {}/6 | {} | {} | {}/{} | {} | {} | {} | {} |\n",
            cell.cell_id,
            cell.base_seed,
            cell.competence_episode,
            cell.held_out_correct,
            cell.request_positions,
            cell.m3_learned_uses,
            cell.completion_activity,
            cell.selection_activations,
            cell.update_activations,
            cell.m3_physical_work,
            cell.duplicate_deterministic,
            result,
            cell.first_collapse,
        ));
    }
    output.push_str("\n## Frozen interpretation\n\n");
    if report.passed {
        output.push_str(
            "The byte-frozen cumulative DS4 mechanism learned the request role and initiated frozen recurrence only from learned M3 event-completion activity, without a request/query marker or supplied START meaning. M4 is authoritative and cumulative DS5 is eligible. Program-priority decisions are outside this matrix.\n",
        );
    } else {
        output.push_str(
            "The cumulative DS4 matrix failed conjunctively. M3 remains authoritative, M4 is absent, and cumulative DS5 remains blocked. No rescue or rerun is permitted.\n",
        );
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
        "protocol={} mode={} claim_eligible={} pass={} M3_authoritative={} M4_exists={} M4_authoritative={} DS5_eligible={} cells={} source={:?}",
        definitive::PROTOCOL,
        report.mode,
        report.claim_eligible,
        report.passed,
        report.m3_authoritative,
        report.m4_exists,
        report.m4_authoritative,
        report.ds5_cumulative_eligible,
        report.cells.len(),
        report.source,
    );
    for cell in &report.cells {
        println!(
            "cell={} seed={} pass={} competence={} roles={} held_out={}/{} explicit={} queues={} positions={} learned_uses={} completion={} selection={} execution={} update={} m3_work={} p4_nonplastic={} m3_nonplastic={} replay={} collapse={}",
            cell.cell_id,
            cell.base_seed,
            cell.passed,
            cell.competence_episode,
            cell.request_roles,
            cell.held_out_correct,
            cell.held_out_total,
            cell.explicit_answers,
            cell.queues_empty,
            cell.request_positions,
            cell.m3_learned_uses,
            cell.completion_activity,
            cell.selection_activations,
            cell.execution_activations,
            cell.update_activations,
            cell.m3_physical_work,
            cell.p4_nonplastic,
            cell.m3_nonplastic,
            cell.duplicate_deterministic,
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
        env::var_os("DS4_DEFINITIVE_CSV")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CSV)),
        env::var_os("DS4_DEFINITIVE_MD")
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
            let source = definitive::source_preflight(true);
            if !source.passed() {
                eprintln!("P0 source preflight failed before outcome spend: {source:?}");
                std::process::exit(2);
            }
            eprintln!("DEFINITIVE OUTCOME SPENT: cell 0 begins; no rescue or rerun");
            let report = definitive::run_definitive(true);
            print_report(&report);
            if let Err(error) = write_definitive(&report, &csv_path, &md_path) {
                eprintln!("write-once definitive serialization failed: {error}");
                std::process::exit(2);
            }
            if !report.passed {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("usage: ds4_cumulative_request_start_definitive [--audit|--definitive]");
            std::process::exit(2)
        }
    }
}
