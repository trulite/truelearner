use std::env;
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use organism_v0::ffs_same0::{run_same0, EdgeResult, Same0Report, FFS_SAME0_PROTOCOL};
use organism_v0::research_runtime::HarnessMode;

const PRICE_MICROS: [u64; 5] = [0, 1, 10, 100, 1_000];
const MICROS_PER_WORK: i128 = 1_000_000;
const FFS0_CSV_PATH: &str = "results/ffs0_full_fractal_scaling.csv";
const FFS0_MD_PATH: &str = "results/ffs0_full_fractal_scaling.md";
const FFS0_AUDIT_PATH: &str = "experiments/full_fractal_scaling_ffs0_outcome_audit.md";
const SAME0_PROTOCOL_PATH: &str = "experiments/ffs_same0_learned_correspondence_protocol.md";
const FFS0_CSV_SHA256: &str = "74f0a92442aa71a60cedf047feeadbebe389586210959b758a7d2cf6fd43db56";
const FFS0_MD_SHA256: &str = "58d2c1efc124bdb481b43317ed2f373926d0da308b41778de0aacd1a79f3e0c2";
const FFS0_AUDIT_SHA256: &str = "13b1d292a3f73889c682e72d8797109d775a79c6d4f978ab90a3ba1b0bd3c20d";
const SAME0_PROTOCOL_SHA256: &str =
    "392f92a0074353c67c2c7bb0f18a22770e47729e7d3ddd9abfe6fef63cec249b";

const CSV_HEADER: &str = "row_type,protocol,mode,claim_eligible,passed,seed,scale,depth,population,generation,process,context,parent_work,child_work,acquisition_work,installation_work,incremental_bytes,maintenance_work,price_micros,gain_micros,break_even_uses,observable_equal,computationally_useful,economically_justified,structurally_retained,proposed,retained,removed_arrow_firings,asset_instance_id,content_fingerprint,structural_depth,justified_depth,realized_useful_depth,right_censored,collapse_point,learned_motifs,identity_acquisition_work,identity_bytes,generic_runtime,learned_identity_runtime,supplied_same_runtime,same_less_runtime,mature_delta_vs_supplied,identity_break_even_uses,fallback_distance,recovery_work,status,name,diagnostic";

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
    file_sha256(FFS0_CSV_PATH).as_deref() == Some(FFS0_CSV_SHA256)
        && file_sha256(FFS0_MD_PATH).as_deref() == Some(FFS0_MD_SHA256)
        && file_sha256(FFS0_AUDIT_PATH).as_deref() == Some(FFS0_AUDIT_SHA256)
        && file_sha256(SAME0_PROTOCOL_PATH).as_deref() == Some(SAME0_PROTOCOL_SHA256)
}

fn priced_break_even(edge: &EdgeResult, price_micros: u64) -> (i128, Option<u64>) {
    let physical_gain =
        edge.parent_work as i128 - edge.child_work as i128 - edge.maintenance_work as i128;
    let gain_micros =
        physical_gain * MICROS_PER_WORK - edge.incremental_bytes as i128 * price_micros as i128;
    let break_even = (edge.observable_equal && gain_micros > 0).then(|| {
        let numerator =
            (edge.acquisition_work + edge.installation_work) as u128 * MICROS_PER_WORK as u128;
        let denominator = gain_micros as u128;
        u64::try_from(numerator / denominator + u128::from(!numerator.is_multiple_of(denominator)))
            .expect("FFS-SAME0 priced break-even fits u64")
    });
    (gain_micros, break_even)
}

fn print_report(report: &Same0Report, ancestry: bool) {
    println!(
        "FFS-SAME0 {}: {}{}",
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
    for scale in &report.scales {
        println!(
            "seed {} {} depth={} population={} motifs={} proposed={} functional={} computational={} economic={} retained={} depths={}/{}/{} collapse={} censored={} identity={} supplied={} same-less={} delta={}",
            scale.seed,
            scale.scale,
            scale.depth,
            scale.population,
            scale.identity.learned_motifs,
            scale.proposed,
            scale.functionally_valid,
            scale.computationally_useful,
            scale.economically_justified,
            scale.endogenously_retained,
            scale.structural_depth,
            scale.justified_depth,
            scale.realized_useful_depth,
            scale.collapse_point,
            scale.right_censored,
            scale.identity.acquisition_work,
            scale.identity.supplied_same_runtime,
            scale.identity.same_less_runtime,
            scale.identity.mature_delta_vs_supplied,
        );
        for edge in &scale.edges {
            println!(
                "  edge {} parent={} child={} acquisition={} bytes={} H*={} removed={} trace={}",
                edge.generation,
                edge.parent_work,
                edge.child_work,
                edge.acquisition_work,
                edge.incremental_bytes,
                edge.break_even_uses
                    .map_or_else(|| "NONE".to_string(), |value| value.to_string()),
                edge.removed_arrow_firings,
                edge.observable_equal,
            );
        }
    }
    for row in &report.transfers {
        println!(
            "transfer seed {} {} work={}->{} trace={} acquisition={} same_instance={}",
            row.seed,
            row.probe,
            row.primitive_work,
            row.transferred_work,
            row.observable_equal,
            row.acquisition_work_charged,
            row.reused_same_instance,
        );
    }
    for row in &report.adaptive {
        println!(
            "adaptive seed {} {} trace={} fallback={} recovery={} reacquisition={} historical={}",
            row.seed,
            row.arm,
            row.observable_equal,
            row.fallback_distance,
            row.recovery_work,
            row.reacquisition_work,
            row.historical_asset_reused,
        );
    }
    for row in &report.controls {
        println!(
            "control {}: {} ({})",
            row.name,
            if row.passed { "PASS" } else { "FAIL" },
            row.diagnostic
        );
    }
    for row in &report.processes {
        println!("process {}: {} ({})", row.process, row.status, row.reason);
    }
    for row in &report.claims {
        println!("claim {}: {}", row.claim, row.status);
    }
    println!(
        "frozen-ancestry: {}; duplicate-determinism: {}; source-audit: {}; identity-leak-audits: {}; scaling-trend: {}; orthogonal-depth-signature: {}",
        ancestry,
        report.duplicate_deterministic,
        report.source_audit_passed,
        report.identity_leak_audits_passed,
        report.scaling_trend_supported,
        report.orthogonal_depth_signature,
    );
}

fn empty_fields(report: &Same0Report, passed: bool) -> Vec<String> {
    let mut fields = vec![String::new(); CSV_HEADER.split(',').count()];
    fields[1] = FFS_SAME0_PROTOCOL.to_string();
    fields[2] = report.mode.clone();
    fields[3] = report.claim_eligible.to_string();
    fields[4] = passed.to_string();
    fields
}

fn csv(report: &Same0Report, ancestry: bool) -> String {
    let passed = report.passed && ancestry;
    let mut out = String::new();
    writeln!(out, "{CSV_HEADER}").unwrap();
    for scale in &report.scales {
        let mut fields = empty_fields(report, passed);
        fields[0] = "scale".into();
        fields[5] = scale.seed.to_string();
        fields[6] = scale.scale.clone();
        fields[7] = scale.depth.to_string();
        fields[8] = scale.population.to_string();
        fields[25] = scale.proposed.to_string();
        fields[26] = scale.endogenously_retained.to_string();
        fields[30] = scale.structural_depth.to_string();
        fields[31] = scale.justified_depth.to_string();
        fields[32] = scale.realized_useful_depth.to_string();
        fields[33] = scale.right_censored.to_string();
        fields[34] = scale.collapse_point.clone();
        fields[35] = scale.identity.learned_motifs.to_string();
        fields[36] = scale.identity.acquisition_work.to_string();
        fields[37] = scale.identity.persistent_bytes.to_string();
        fields[38] = scale.identity.generic_runtime.to_string();
        fields[39] = scale.identity.learned_identity_runtime.to_string();
        fields[40] = scale.identity.supplied_same_runtime.to_string();
        fields[41] = scale.identity.same_less_runtime.to_string();
        fields[42] = scale.identity.mature_delta_vs_supplied.to_string();
        fields[43] = scale
            .identity
            .internal_break_even_uses
            .map_or_else(String::new, |value| value.to_string());
        fields[48] = format!(
            "functional:{};computational:{};economic:{};over:{};under:{}",
            scale.functionally_valid,
            scale.computationally_useful,
            scale.economically_justified,
            scale.over_retained,
            scale.under_retained,
        );
        writeln!(out, "{}", fields.join(",")).unwrap();
        for edge in &scale.edges {
            for price in PRICE_MICROS {
                let (gain, break_even) = priced_break_even(edge, price);
                let mut fields = empty_fields(report, passed);
                fields[0] = "edge".into();
                fields[5] = edge.seed.to_string();
                fields[6] = edge.scale.clone();
                fields[9] = edge.generation.to_string();
                fields[12] = edge.parent_work.to_string();
                fields[13] = edge.child_work.to_string();
                fields[14] = edge.acquisition_work.to_string();
                fields[15] = edge.installation_work.to_string();
                fields[16] = edge.incremental_bytes.to_string();
                fields[17] = edge.maintenance_work.to_string();
                fields[18] = price.to_string();
                fields[19] = gain.to_string();
                fields[20] = break_even.map_or_else(String::new, |value| value.to_string());
                fields[21] = edge.observable_equal.to_string();
                fields[22] = edge.computationally_useful.to_string();
                fields[23] = edge.economically_justified.to_string();
                fields[24] = edge.structurally_retained.to_string();
                fields[25] = edge.proposed.to_string();
                fields[26] = edge.retained.to_string();
                fields[27] = edge.removed_arrow_firings.to_string();
                fields[28] = edge.asset_instance_id.to_string();
                fields[29] = edge.content_fingerprint.to_string();
                writeln!(out, "{}", fields.join(",")).unwrap();
            }
        }
        let mut fields = empty_fields(report, passed);
        fields[0] = "identity".into();
        fields[5] = scale.seed.to_string();
        fields[6] = scale.scale.clone();
        fields[7] = scale.depth.to_string();
        fields[8] = scale.population.to_string();
        fields[35] = scale.identity.learned_motifs.to_string();
        fields[36] = scale.identity.acquisition_work.to_string();
        fields[37] = scale.identity.persistent_bytes.to_string();
        fields[38] = scale.identity.generic_runtime.to_string();
        fields[39] = scale.identity.learned_identity_runtime.to_string();
        fields[40] = scale.identity.supplied_same_runtime.to_string();
        fields[41] = scale.identity.same_less_runtime.to_string();
        fields[42] = scale.identity.mature_delta_vs_supplied.to_string();
        fields[43] = scale
            .identity
            .break_even_vs_supplied
            .map_or_else(String::new, |value| value.to_string());
        fields[46] = if scale.identity.break_even_vs_supplied.is_some() {
            "PARITY_OR_BETTER"
        } else {
            "EXPENSIVE"
        }
        .into();
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    for row in &report.transfers {
        let mut fields = empty_fields(report, passed);
        fields[0] = "transfer".into();
        fields[5] = row.seed.to_string();
        fields[6] = row.probe.clone();
        fields[7] = row.depth.to_string();
        fields[8] = row.population.to_string();
        fields[12] = row.primitive_work.to_string();
        fields[13] = row.transferred_work.to_string();
        fields[14] = row.acquisition_work_charged.to_string();
        fields[21] = row.observable_equal.to_string();
        fields[28] = row.asset_instance_id.to_string();
        fields[29] = row.content_fingerprint.to_string();
        fields[46] = if row.reused_same_instance {
            "PASS"
        } else {
            "FAIL"
        }
        .into();
        fields[47] = row.source_scale.clone();
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    for row in &report.adaptive {
        let mut fields = empty_fields(report, passed);
        fields[0] = "adaptive".into();
        fields[5] = row.seed.to_string();
        fields[11] = row.arm.clone();
        fields[21] = row.observable_equal.to_string();
        fields[28] = row.asset_instance_id.to_string();
        fields[29] = row.content_fingerprint.to_string();
        fields[44] = row.fallback_distance.to_string();
        fields[45] = row.recovery_work.to_string();
        fields[46] = if row.observable_equal { "PASS" } else { "FAIL" }.into();
        fields[48] = format!(
            "reacquisition:{};historical:{}",
            row.reacquisition_work, row.historical_asset_reused
        );
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    for row in &report.controls {
        let mut fields = empty_fields(report, passed);
        fields[0] = "control".into();
        fields[5] = row.seed.to_string();
        fields[46] = if row.passed { "PASS" } else { "FAIL" }.into();
        fields[47] = row.name.clone();
        fields[48] = row.diagnostic.to_string();
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    for row in &report.processes {
        let mut fields = empty_fields(report, passed);
        fields[0] = "process".into();
        fields[10] = row.process.clone();
        fields[46] = row.status.clone();
        fields[48] = row.reason.replace(',', ";");
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    for row in &report.claims {
        let mut fields = empty_fields(report, passed);
        fields[0] = "claim".into();
        fields[46] = row.status.clone();
        fields[47] = row.claim.clone();
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    for (name, status) in [
        ("frozen-ancestry", ancestry),
        ("duplicate-determinism", report.duplicate_deterministic),
        ("source-audit", report.source_audit_passed),
        ("identity-leak-audits", report.identity_leak_audits_passed),
        ("scaling-trend", report.scaling_trend_supported),
        (
            "orthogonal-depth-signature",
            report.orthogonal_depth_signature,
        ),
    ] {
        let mut fields = empty_fields(report, passed);
        fields[0] = "audit".into();
        fields[46] = if status { "PASS" } else { "FAIL" }.into();
        fields[47] = name.into();
        writeln!(out, "{}", fields.join(",")).unwrap();
    }
    out
}

fn markdown(report: &Same0Report, ancestry: bool) -> String {
    let mut out = String::new();
    writeln!(out, "# FFS-SAME0 learned correspondence\n").unwrap();
    writeln!(out, "- protocol: `{FFS_SAME0_PROTOCOL}`").unwrap();
    writeln!(out, "- mode: `{}`", report.mode).unwrap();
    writeln!(out, "- claim eligible: `{}`", report.claim_eligible).unwrap();
    writeln!(out, "- passed: `{}`", report.passed && ancestry).unwrap();
    writeln!(out, "- frozen ancestry: `{ancestry}`").unwrap();
    writeln!(out, "\n## Scale law\n").unwrap();
    writeln!(out, "| seed | scale | depth | population | motifs | retained depth | collapse | supplied | SAME-less | delta | censored |").unwrap();
    writeln!(out, "|---:|---|---:|---:|---:|---:|---|---:|---:|---:|---|").unwrap();
    for scale in &report.scales {
        writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            scale.seed,
            scale.scale,
            scale.depth,
            scale.population,
            scale.identity.learned_motifs,
            scale.realized_useful_depth,
            scale.collapse_point,
            scale.identity.supplied_same_runtime,
            scale.identity.same_less_runtime,
            scale.identity.mature_delta_vs_supplied,
            scale.right_censored,
        )
        .unwrap();
    }
    writeln!(out, "\n## Parent-relative edges\n").unwrap();
    writeln!(
        out,
        "| seed | scale | edge | parent | child | acquisition | bytes | H* | removed | trace |"
    )
    .unwrap();
    writeln!(out, "|---:|---|---:|---:|---:|---:|---:|---:|---:|---|").unwrap();
    for scale in &report.scales {
        for edge in &scale.edges {
            writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                edge.seed,
                edge.scale,
                edge.generation,
                edge.parent_work,
                edge.child_work,
                edge.acquisition_work,
                edge.incremental_bytes,
                edge.break_even_uses
                    .map_or_else(|| "none".to_string(), |value| value.to_string()),
                edge.removed_arrow_firings,
                edge.observable_equal,
            )
            .unwrap();
        }
    }
    writeln!(out, "\n## Independent outcomes\n").unwrap();
    for claim in &report.claims {
        writeln!(out, "- {}: **{}**", claim.claim, claim.status).unwrap();
    }
    writeln!(out, "\n## Process availability\n").unwrap();
    for process in &report.processes {
        writeln!(
            out,
            "- {}: **{}** — {}",
            process.process, process.status, process.reason
        )
        .unwrap();
    }
    writeln!(out, "\n## Controls\n").unwrap();
    for control in &report.controls {
        writeln!(
            out,
            "- seed {} {}: **{}** ({})",
            control.seed,
            control.name,
            if control.passed { "PASS" } else { "FAIL" },
            control.diagnostic
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
        let report = run_same0(mode);
        print_report(&report, ancestry);
        assert!(
            report.passed && ancestry,
            "FFS-SAME0 development gate failed"
        );
        return;
    }
    assert!(arguments.len() <= 2, "expected [csv] [markdown]");
    let csv_path = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/ffs_same0_learned_correspondence.csv"));
    let markdown_path = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/ffs_same0_learned_correspondence.md"));
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
    let report = run_same0(mode);
    print_report(&report, ancestry);
    assert!(
        report.passed && ancestry,
        "FFS-SAME0 definitive gate failed"
    );
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
    fn csv_has_one_fixed_schema() {
        let report = run_same0(HarnessMode::Micro);
        let csv = csv(&report, true);
        let columns = CSV_HEADER.split(',').count();
        assert!(csv.lines().all(|line| line.split(',').count() == columns));
    }

    #[test]
    fn priced_break_even_is_parent_relative() {
        let report = run_same0(HarnessMode::Gate);
        let edge = &report
            .scales
            .iter()
            .find(|scale| !scale.edges.is_empty())
            .unwrap()
            .edges[0];
        let (_, physical) = priced_break_even(edge, 0);
        assert_eq!(physical, edge.break_even_uses);
    }
}
