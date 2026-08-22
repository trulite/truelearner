use std::env;
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use organism_v0::ffs_same0::cs0a::{run_cs0a, Cs0aReport, CS0A_PROTOCOL};
use organism_v0::research_runtime::HarnessMode;

const SAME0_CSV_PATH: &str = "results/ffs_same0_learned_correspondence.csv";
const SAME0_MD_PATH: &str = "results/ffs_same0_learned_correspondence.md";
const SAME0_AUDIT_PATH: &str = "experiments/ffs_same0_learned_correspondence_outcome_audit.md";
const LADDER_PATH: &str = "experiments/identity_desupply_ladder_protocol.md";
const SAME0_CSV_SHA256: &str = "d136492e5ddaa70194f155657e7e86eee97da57e8dedcd9d1f52bcf81395a812";
const SAME0_MD_SHA256: &str = "2bf80d0ed37e4d57aa3fe9ba2fb9ec2efc3663c2eb7ee0b701796731520c3c1f";
const SAME0_AUDIT_SHA256: &str = "79d98b7571d72556610a9a7dc5a33cb6c28df138eef237bed5fdbe5300e4f06b";
const LADDER_SHA256: &str = "493cc50ac67e7c3985b61e7fbe249f19388fed45b026d29dc81ad70f20c9ccbe";

const CSV_HEADER: &str = "row_type,protocol,mode,claim_eligible,passed,seed,arm,correct,total,average_work,average_correspondence_work,parent_acquisition_work,compilation_work,proposed,compiled_routes,persistent_bytes,persistent_fingerprint,subthreshold_routes,shuffled_routes,anonymous_observations,temporal_relations,causal_relations,generic_lookups,generic_comparisons,compiled_activations,support_validations,dependency_comparisons,binding_writes,binding_reads,temp_installations,effect_deliveries,invalidations,generic_reopenings,status,name,diagnostic";

fn file_sha256(path: &str) -> Option<String> {
    let output = Command::new("sha256sum").arg(path).output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()?
        .split_whitespace()
        .next()
        .map(str::to_string)
}

fn ancestry_valid() -> bool {
    file_sha256(SAME0_CSV_PATH).as_deref() == Some(SAME0_CSV_SHA256)
        && file_sha256(SAME0_MD_PATH).as_deref() == Some(SAME0_MD_SHA256)
        && file_sha256(SAME0_AUDIT_PATH).as_deref() == Some(SAME0_AUDIT_SHA256)
        && file_sha256(LADDER_PATH).as_deref() == Some(LADDER_SHA256)
}

fn print_report(report: &Cs0aReport, ancestry: bool) {
    println!(
        "CS0a {}: {}{}",
        report.mode,
        if report.passed && ancestry {
            "PASS"
        } else {
            "FAIL"
        },
        if report.claim_eligible {
            " (claim eligible)"
        } else {
            " (development only; no claim)"
        }
    );
    println!(
        "mature correspondence work generic={} compiled={} reduction={}",
        report.generic_correspondence_work,
        report.compiled_correspondence_work,
        report.mature_reduction
    );
    for acquisition in &report.acquisitions {
        println!(
            "seed {} acquisition parent={} compile={} proposed={} routes={} bytes={} subthreshold={} shuffled={} fingerprint={}",
            acquisition.seed,
            acquisition.parent_correspondence_work,
            acquisition.compilation_work,
            acquisition.proposed,
            acquisition.compiled_routes,
            acquisition.persistent_bytes,
            acquisition.subthreshold_routes,
            acquisition.shuffled_routes,
            acquisition.persistent_fingerprint,
        );
    }
    for row in &report.rows {
        println!(
            "seed {} {} correct={}/{} work={} correspondence={} compiled={} reopen={} invalidations={}",
            row.seed,
            row.arm.name(),
            row.correct,
            row.total,
            row.average_work,
            row.average_correspondence_work,
            row.compiled_uses,
            row.generic_reopenings,
            row.work.compiled_invalidations,
        );
    }
    for control in &report.controls {
        println!(
            "control seed {} {}: {} ({})",
            control.seed,
            control.name,
            if control.passed { "PASS" } else { "FAIL" },
            control.diagnostic
        );
    }
    println!(
        "frozen-ancestry: {}; parent-fixture: {}; duplicate-determinism: {}; source-audit: {}",
        ancestry,
        report.parent_fixture_positive,
        report.duplicate_deterministic,
        report.source_audit_passed,
    );
}

fn empty_fields(report: &Cs0aReport, passed: bool) -> Vec<String> {
    let mut fields = vec![String::new(); CSV_HEADER.split(',').count()];
    fields[1] = CS0A_PROTOCOL.to_string();
    fields[2] = report.mode.clone();
    fields[3] = report.claim_eligible.to_string();
    fields[4] = passed.to_string();
    fields
}

fn csv(report: &Cs0aReport, ancestry: bool) -> String {
    let passed = report.passed && ancestry;
    let mut out = String::new();
    writeln!(out, "{CSV_HEADER}").unwrap();
    for acquisition in &report.acquisitions {
        let mut fields = empty_fields(report, passed);
        fields[0] = "acquisition".into();
        fields[5] = acquisition.seed.to_string();
        fields[11] = acquisition.parent_correspondence_work.to_string();
        fields[12] = acquisition.compilation_work.to_string();
        fields[13] = acquisition.proposed.to_string();
        fields[14] = acquisition.compiled_routes.to_string();
        fields[15] = acquisition.persistent_bytes.to_string();
        fields[16] = acquisition.persistent_fingerprint.to_string();
        fields[17] = acquisition.subthreshold_routes.to_string();
        fields[18] = acquisition.shuffled_routes.to_string();
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    for row in &report.rows {
        let work = row.work;
        let mut fields = empty_fields(report, passed);
        fields[0] = "arm".into();
        fields[5] = row.seed.to_string();
        fields[6] = row.arm.name().into();
        fields[7] = row.correct.to_string();
        fields[8] = row.total.to_string();
        fields[9] = row.average_work.to_string();
        fields[10] = row.average_correspondence_work.to_string();
        fields[16] = row.persistent_fingerprint.to_string();
        fields[19] = work.same0.anonymous_observations.to_string();
        fields[20] = work.same0.temporal_relations.to_string();
        fields[21] = work.same0.causal_relations.to_string();
        fields[22] = work.same0.correspondence_lookups.to_string();
        fields[23] = work.same0.correspondence_comparisons.to_string();
        fields[24] = work.compiled_local_activations.to_string();
        fields[25] = work.context_support_validations.to_string();
        fields[26] = work.compiled_dependency_comparisons.to_string();
        fields[27] = work.same0.binding_writes.to_string();
        fields[28] = work.same0.binding_reads.to_string();
        fields[29] = work.temporary_path_installations.to_string();
        fields[30] = work.same0.effect_deliveries.to_string();
        fields[31] = work.compiled_invalidations.to_string();
        fields[32] = row.generic_reopenings.to_string();
        fields[35] = format!("compiled_uses:{}", row.compiled_uses);
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    for control in &report.controls {
        let mut fields = empty_fields(report, passed);
        fields[0] = "control".into();
        fields[5] = control.seed.to_string();
        fields[33] = if control.passed { "PASS" } else { "FAIL" }.into();
        fields[34] = control.name.clone();
        fields[35] = control.diagnostic.to_string();
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    let mut fields = empty_fields(report, passed);
    fields[0] = "summary".into();
    fields[10] = report.compiled_correspondence_work.to_string();
    fields[33] = if report.compiled_correspondence_work < report.generic_correspondence_work {
        "PASS"
    } else {
        "FAIL"
    }
    .into();
    fields[34] = "compiled-correspondence-reduces-mature-tax".into();
    fields[35] = format!(
        "generic:{};compiled:{};reduction:{}",
        report.generic_correspondence_work,
        report.compiled_correspondence_work,
        report.mature_reduction
    );
    writeln!(out, "{}", fields.join(",")).unwrap();
    for (name, status) in [
        ("frozen-ancestry", ancestry),
        ("parent-fixture-positive", report.parent_fixture_positive),
        ("duplicate-determinism", report.duplicate_deterministic),
        ("persistent-source-audit", report.source_audit_passed),
    ] {
        let mut fields = empty_fields(report, passed);
        fields[0] = "audit".into();
        fields[33] = if status { "PASS" } else { "FAIL" }.into();
        fields[34] = name.into();
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    out
}

fn markdown(report: &Cs0aReport, ancestry: bool) -> String {
    let mut out = String::new();
    writeln!(out, "# CS0a compiled correspondence\n").unwrap();
    writeln!(out, "- protocol: `{CS0A_PROTOCOL}`").unwrap();
    writeln!(out, "- mode: `{}`", report.mode).unwrap();
    writeln!(out, "- claim eligible: `{}`", report.claim_eligible).unwrap();
    writeln!(out, "- passed: `{}`", report.passed && ancestry).unwrap();
    writeln!(out, "- frozen ancestry: `{ancestry}`").unwrap();
    writeln!(out, "\n## Mature correspondence work\n").unwrap();
    writeln!(out, "| Generic | Compiled | Reduction |\n|---:|---:|---:|").unwrap();
    writeln!(
        out,
        "| {} | {} | {} |",
        report.generic_correspondence_work,
        report.compiled_correspondence_work,
        report.mature_reduction
    )
    .unwrap();
    writeln!(out, "\n## Acquisition\n").unwrap();
    writeln!(out, "| Seed | Parent acquisition | Compilation | Routes | Bytes | Subthreshold | Shuffled |\n|---:|---:|---:|---:|---:|---:|---:|").unwrap();
    for row in &report.acquisitions {
        writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} |",
            row.seed,
            row.parent_correspondence_work,
            row.compilation_work,
            row.compiled_routes,
            row.persistent_bytes,
            row.subthreshold_routes,
            row.shuffled_routes,
        )
        .unwrap();
    }
    writeln!(out, "\n## Arms\n").unwrap();
    writeln!(out, "| Seed | Arm | Correct | Work | Correspondence | Compiled uses | Generic reopenings | Invalidations |\n|---:|---|---:|---:|---:|---:|---:|---:|").unwrap();
    for row in &report.rows {
        writeln!(
            out,
            "| {} | {} | {}/{} | {} | {} | {} | {} | {} |",
            row.seed,
            row.arm.name(),
            row.correct,
            row.total,
            row.average_work,
            row.average_correspondence_work,
            row.compiled_uses,
            row.generic_reopenings,
            row.work.compiled_invalidations,
        )
        .unwrap();
    }
    writeln!(out, "\n## Controls\n").unwrap();
    for row in &report.controls {
        writeln!(
            out,
            "- seed {} {}: **{}** ({})",
            row.seed,
            row.name,
            if row.passed { "PASS" } else { "FAIL" },
            row.diagnostic
        )
        .unwrap();
    }
    out
}

fn write_new(path: &Path, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap_or_else(|error| panic!("refuse to overwrite {}: {error}", path.display()));
    file.write_all(contents.as_bytes())
        .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

fn main() {
    let mut arguments = env::args().skip(1).collect::<Vec<_>>();
    let mode = match arguments.first().map(String::as_str) {
        Some("--micro") => {
            arguments.remove(0);
            HarnessMode::Micro
        }
        Some("--gate") => {
            arguments.remove(0);
            HarnessMode::Gate
        }
        Some("--definitive") => {
            arguments.remove(0);
            HarnessMode::Definitive
        }
        _ => panic!("expected --micro, --gate, or --definitive"),
    };
    let ancestry = ancestry_valid();
    if mode != HarnessMode::Definitive {
        assert!(arguments.is_empty(), "development modes write no artifacts");
        let report = run_cs0a(mode);
        print_report(&report, ancestry);
        assert!(report.passed && ancestry, "CS0a development gate failed");
        return;
    }
    assert!(arguments.len() <= 2, "expected [csv] [markdown]");
    let csv_path = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/cs0a_compiled_correspondence.csv"));
    let markdown_path = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/cs0a_compiled_correspondence.md"));
    assert!(
        !csv_path.exists(),
        "refuse to overwrite {}",
        csv_path.display()
    );
    assert!(
        !markdown_path.exists(),
        "refuse to overwrite {}",
        markdown_path.display()
    );
    let report = run_cs0a(mode);
    print_report(&report, ancestry);
    assert!(report.passed && ancestry, "CS0a definitive gate failed");
    write_new(&csv_path, &csv(&report, ancestry));
    write_new(&markdown_path, &markdown(&report, ancestry));
    println!(
        "wrote {} and {}",
        csv_path.display(),
        markdown_path.display()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_has_fixed_schema() {
        let report = run_cs0a(HarnessMode::Micro);
        let data = csv(&report, true);
        let columns = CSV_HEADER.split(',').count();
        assert!(data.lines().all(|line| line.split(',').count() == columns));
    }

    #[test]
    fn development_modes_are_not_claim_eligible() {
        for mode in [HarnessMode::Micro, HarnessMode::Gate] {
            let report = run_cs0a(mode);
            assert!(report.passed);
            assert!(!report.claim_eligible);
        }
    }
}
