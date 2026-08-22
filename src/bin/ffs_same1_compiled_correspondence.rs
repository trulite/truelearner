use std::env;
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use organism_v0::ffs_same0::same1::{run_same1, Same1Report, FFS_SAME1_PROTOCOL};
use organism_v0::research_runtime::HarnessMode;

const FROZEN_FILES: [(&str, &str); 11] = [
    (
        "results/ffs0_full_fractal_scaling.csv",
        "74f0a92442aa71a60cedf047feeadbebe389586210959b758a7d2cf6fd43db56",
    ),
    (
        "results/ffs0_full_fractal_scaling.md",
        "58d2c1efc124bdb481b43317ed2f373926d0da308b41778de0aacd1a79f3e0c2",
    ),
    (
        "experiments/full_fractal_scaling_ffs0_outcome_audit.md",
        "13b1d292a3f73889c682e72d8797109d775a79c6d4f978ab90a3ba1b0bd3c20d",
    ),
    (
        "results/ffs_same0_learned_correspondence.csv",
        "d136492e5ddaa70194f155657e7e86eee97da57e8dedcd9d1f52bcf81395a812",
    ),
    (
        "results/ffs_same0_learned_correspondence.md",
        "2bf80d0ed37e4d57aa3fe9ba2fb9ec2efc3663c2eb7ee0b701796731520c3c1f",
    ),
    (
        "experiments/ffs_same0_learned_correspondence_outcome_audit.md",
        "79d98b7571d72556610a9a7dc5a33cb6c28df138eef237bed5fdbe5300e4f06b",
    ),
    (
        "results/cs0a_compiled_correspondence.csv",
        "98b983fb9cc88149c2c9de83e74e11d94e1aa3488a6f63f756e835fbd341bf36",
    ),
    (
        "results/cs0a_compiled_correspondence.md",
        "7ab1c68bcbb28db850010adcfcb99bc4a90438eacd76829beac9f9d458ce118b",
    ),
    (
        "experiments/cs0a_compiled_correspondence_outcome_audit.md",
        "77f071f88532c1f6be1f33f6023c73d03df49f15e397941c9f3966223dcbfa3f",
    ),
    (
        "results/cs0a_grounding_tax_attribution.csv",
        "8230b24109e924b264d2f3b3335acde06b91c81a4363d39a13a38afd1af9a5ec",
    ),
    (
        "experiments/identity_desupply_ladder_protocol.md",
        "493cc50ac67e7c3985b61e7fbe249f19388fed45b026d29dc81ad70f20c9ccbe",
    ),
];

const CSV_HEADER: &str = "row_type,protocol,mode,claim_eligible,passed,seed,scale,depth,population,generation,parent_work,child_work,acquisition_work,incremental_bytes,break_even_uses,observable_equal,computationally_useful,economically_justified,structurally_retained,removed_arrow_firings,structural_depth,justified_depth,realized_depth,right_censored,over_retained,under_retained,collapse_point,generic_acquisition_work,compilation_acquisition_work,generic_bytes,compiled_bytes,anonymous_generic_runtime,same0_runtime,same1_runtime,supplied_runtime,same0_tax,same1_tax,improvement_vs_same0,premium_vs_supplied,activation_work,validation_work,ambiguity_work,grounding_work,fallback_distance,recovery_work,status,name,diagnostic";

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
    FROZEN_FILES
        .iter()
        .all(|(path, expected)| file_sha256(path).as_deref() == Some(*expected))
}

fn print_report(report: &Same1Report, ancestry: bool) {
    println!(
        "FFS-SAME1 {}: {}{}",
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
            "seed={} scale={} depth={} population={} hierarchy={}/{}/{}{} runtimes anonymous={} SAME0={} SAME1={} supplied={} tax={} controls={}",
            scale.seed,
            scale.scale,
            scale.depth,
            scale.population,
            scale.structural_depth,
            scale.justified_depth,
            scale.realized_useful_depth,
            if scale.right_censored { "+" } else { "" },
            scale.identity.anonymous_generic_runtime,
            scale.identity.same0_generic_learned_runtime,
            scale.identity.same1_compiled_runtime,
            scale.identity.supplied_same_runtime,
            scale.identity.same1_identity_tax,
            if scale.over_retained == 0 && scale.under_retained == 0 { "agreement" } else { "mismatch" },
        );
    }
    println!(
        "curve={} trend={} orthogonal={} adaptive={}/{} controls={}/{} deterministic={} source_audit={} ancestry={}",
        report.depth_curve_preserved,
        report.scaling_trend_supported,
        report.orthogonal_depth_signature,
        report.adaptive.iter().filter(|row| row.observable_equal).count(),
        report.adaptive.len(),
        report.controls.iter().filter(|row| row.passed).count(),
        report.controls.len(),
        report.duplicate_deterministic,
        report.source_audit_passed,
        ancestry,
    );
    for claim in &report.claims {
        println!("{}: {}", claim.claim, claim.status);
    }
}

const CSV_COLUMNS: usize = 48;

fn base(report: &Same1Report, passed: bool) -> Vec<String> {
    let mut fields = vec![String::new(); CSV_COLUMNS];
    fields[1] = FFS_SAME1_PROTOCOL.to_string();
    fields[2] = report.mode.clone();
    fields[3] = report.claim_eligible.to_string();
    fields[4] = passed.to_string();
    fields
}

fn push_row(out: &mut String, fields: Vec<String>) {
    assert_eq!(fields.len(), CSV_COLUMNS);
    writeln!(out, "{}", fields.join(",")).unwrap();
}

fn csv(report: &Same1Report, ancestry: bool) -> String {
    let mut out = String::new();
    writeln!(out, "{CSV_HEADER}").unwrap();
    for scale in &report.scales {
        let mut fields = base(report, report.passed && ancestry);
        fields[0] = "scale".to_string();
        fields[5] = scale.seed.to_string();
        fields[6] = scale.scale.clone();
        fields[7] = scale.depth.to_string();
        fields[8] = scale.population.to_string();
        fields[20] = scale.structural_depth.to_string();
        fields[21] = scale.justified_depth.to_string();
        fields[22] = scale.realized_useful_depth.to_string();
        fields[23] = scale.right_censored.to_string();
        fields[24] = scale.over_retained.to_string();
        fields[25] = scale.under_retained.to_string();
        fields[26] = scale.collapse_point.clone();
        fields[27] = scale.identity.generic_acquisition_work.to_string();
        fields[28] = scale.identity.compilation_acquisition_work.to_string();
        fields[29] = scale.identity.generic_persistent_bytes.to_string();
        fields[30] = scale.identity.compiled_persistent_bytes.to_string();
        fields[31] = scale.identity.anonymous_generic_runtime.to_string();
        fields[32] = scale.identity.same0_generic_learned_runtime.to_string();
        fields[33] = scale.identity.same1_compiled_runtime.to_string();
        fields[34] = scale.identity.supplied_same_runtime.to_string();
        fields[35] = scale.identity.same0_identity_tax.to_string();
        fields[36] = scale.identity.same1_identity_tax.to_string();
        fields[37] = scale.identity.improvement_vs_same0.to_string();
        fields[38] = scale.identity.premium_vs_supplied.to_string();
        fields[39] = scale.identity.activation_work.to_string();
        fields[40] = scale.identity.validation_work.to_string();
        fields[41] = scale.identity.ambiguity_work.to_string();
        fields[42] = scale.identity.grounding_work.to_string();
        fields[45] = "MEASURED".to_string();
        fields[46] = "scale-law".to_string();
        fields[47] = format!(
            "compiled_uses={};reopenings={}",
            scale.identity.compiled_uses, scale.identity.generic_reopenings
        );
        push_row(&mut out, fields);
        for edge in &scale.edges {
            let mut fields = base(
                report,
                edge.observable_equal && edge.computationally_useful && edge.economically_justified,
            );
            fields[0] = "edge".to_string();
            fields[5] = edge.seed.to_string();
            fields[6] = edge.scale.clone();
            fields[9] = edge.generation.to_string();
            fields[10] = edge.parent_work.to_string();
            fields[11] = edge.child_work.to_string();
            fields[12] = edge.acquisition_work.to_string();
            fields[13] = edge.incremental_bytes.to_string();
            fields[14] = edge
                .break_even_uses
                .map(|value| value.to_string())
                .unwrap_or_default();
            fields[15] = edge.observable_equal.to_string();
            fields[16] = edge.computationally_useful.to_string();
            fields[17] = edge.economically_justified.to_string();
            fields[18] = edge.structurally_retained.to_string();
            fields[19] = edge.removed_arrow_firings.to_string();
            fields[36] = edge.compiled_correspondence_work.to_string();
            fields[45] = "PARENT_RELATIVE".to_string();
            fields[46] = "promotion".to_string();
            fields[47] = format!(
                "compiled_acquisition_uses={};generic_reopenings={}",
                edge.acquisition_compiled_uses, edge.acquisition_generic_reopenings
            );
            push_row(&mut out, fields);
        }
    }
    for row in &report.adaptive {
        let mut fields = base(report, row.observable_equal);
        fields[0] = "adaptive".to_string();
        fields[5] = row.seed.to_string();
        fields[43] = row.fallback_distance.to_string();
        fields[44] = row.recovery_work.to_string();
        fields[45] = if row.observable_equal { "PASS" } else { "FAIL" }.to_string();
        fields[46] = row.arm.clone();
        fields[47] = format!(
            "reacquisition={};historical_reuse={}",
            row.reacquisition_work, row.historical_asset_reused
        );
        push_row(&mut out, fields);
    }
    for row in &report.controls {
        let mut fields = base(report, row.passed);
        fields[0] = "control".to_string();
        fields[5] = row.seed.to_string();
        fields[45] = if row.passed { "PASS" } else { "FAIL" }.to_string();
        fields[46] = row.name.clone();
        fields[47] = row.diagnostic.to_string();
        push_row(&mut out, fields);
    }
    for row in &report.claims {
        let mut fields = base(report, row.status != "FAIL");
        fields[0] = "claim".to_string();
        fields[45] = row.status.clone();
        fields[46] = row.claim.clone();
        push_row(&mut out, fields);
    }
    out
}

fn markdown(report: &Same1Report, ancestry: bool) -> String {
    let mut out = String::new();
    writeln!(out, "# FFS-SAME1 compiled correspondence reintegration\n").unwrap();
    writeln!(out, "- Protocol: `{FFS_SAME1_PROTOCOL}`").unwrap();
    writeln!(out, "- Mode: `{}`", report.mode).unwrap();
    writeln!(out, "- Claim eligible: `{}`", report.claim_eligible).unwrap();
    writeln!(out, "- Frozen ancestry valid: `{ancestry}`").unwrap();
    writeln!(
        out,
        "- Gate result: `{}`\n",
        if report.passed && ancestry {
            "PASS"
        } else {
            "FAIL"
        }
    )
    .unwrap();
    writeln!(out, "## Scale law\n").unwrap();
    writeln!(out, "| Seed | Scale | Depth | Population | Retained / justified / realized | Identity runtime: generic / compiled / supplied | SAME1 tax | Result |").unwrap();
    writeln!(out, "|---:|---|---:|---:|---|---|---:|---|").unwrap();
    for scale in &report.scales {
        writeln!(
            out,
            "| {} | {} | {} | {} | {} / {} / {}{} | {} / {} / {} | {} | {} |",
            scale.seed,
            scale.scale,
            scale.depth,
            scale.population,
            scale.structural_depth,
            scale.justified_depth,
            scale.realized_useful_depth,
            if scale.right_censored { "+" } else { "" },
            scale.identity.same0_generic_learned_runtime,
            scale.identity.same1_compiled_runtime,
            scale.identity.supplied_same_runtime,
            scale.identity.same1_identity_tax,
            scale.collapse_point
        )
        .unwrap();
    }
    writeln!(out, "\n## Frozen physical attribution\n").unwrap();
    writeln!(out, "| Component | Work/use |").unwrap();
    writeln!(out, "|---|---:|").unwrap();
    writeln!(out, "| Local activation | 1 |").unwrap();
    writeln!(out, "| Context/support/dependency validation | 3 |").unwrap();
    writeln!(out, "| Ambiguity handling | 1 |").unwrap();
    writeln!(out, "| Temporary route installation + binding | 1 |").unwrap();
    writeln!(out, "| **Total** | **6** |").unwrap();
    writeln!(out, "\n## Independent outcomes\n").unwrap();
    for claim in &report.claims {
        writeln!(out, "- {}: `{}`", claim.claim, claim.status).unwrap();
    }
    writeln!(out, "\n## Controls\n").unwrap();
    writeln!(
        out,
        "- Controls passed: `{}/{}`",
        report.controls.iter().filter(|row| row.passed).count(),
        report.controls.len()
    )
    .unwrap();
    writeln!(
        out,
        "- Adaptive rows passed: `{}/{}`",
        report
            .adaptive
            .iter()
            .filter(|row| row.observable_equal)
            .count(),
        report.adaptive.len()
    )
    .unwrap();
    writeln!(
        out,
        "- Exact old depth curve: `{}`",
        report.depth_curve_preserved
    )
    .unwrap();
    writeln!(out, "- Source audit: `{}`", report.source_audit_passed).unwrap();
    writeln!(
        out,
        "- Duplicate deterministic: `{}`",
        report.duplicate_deterministic
    )
    .unwrap();
    writeln!(out, "\nNo capability, CS0b path, level-specific correspondence machinery, or economic feedback was added.").unwrap();
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
        let report = run_same1(mode);
        print_report(&report, ancestry);
        assert!(
            report.passed && ancestry,
            "FFS-SAME1 development gate failed"
        );
        return;
    }
    assert!(arguments.len() <= 2, "expected [csv] [markdown]");
    let csv_path = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/ffs_same1_compiled_correspondence.csv"));
    let markdown_path = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/ffs_same1_compiled_correspondence.md"));
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
    let report = run_same1(mode);
    print_report(&report, ancestry);
    assert!(
        report.passed && ancestry,
        "FFS-SAME1 definitive gate failed"
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
    fn csv_schema_is_fixed() {
        let report = run_same1(HarnessMode::Micro);
        let csv = csv(&report, true);
        let columns = CSV_HEADER.split(',').count();
        assert_eq!(columns, CSV_COLUMNS);
        assert!(csv.lines().all(|line| line.split(',').count() == columns));
    }

    #[test]
    fn development_modes_do_not_write_results() {
        for mode in [HarnessMode::Micro, HarnessMode::Gate] {
            let report = run_same1(mode);
            assert!(!report.claim_eligible);
            assert!(report.passed);
        }
    }
}
