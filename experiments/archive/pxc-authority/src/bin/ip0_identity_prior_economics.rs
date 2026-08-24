use std::env;
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use organism_v0::identity_prior_economics::{run_ip0, Ip0Report, IP0_PROTOCOL};
use organism_v0::research_runtime::HarnessMode;

const FROZEN_FILES: [(&str, &str); 15] = [
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
        "results/ffs_same1_compiled_correspondence.csv",
        "7883f71918d48c4c622d7cd2d9dd7561f5954f7287f8bc6abb535f5a9f994a55",
    ),
    (
        "results/ffs_same1_compiled_correspondence.md",
        "a788106462498dd7581fcbd324d6fbc71a1ca0a46c3390a4d289ae180731edad",
    ),
    (
        "experiments/ffs_same1_compiled_correspondence_outcome_audit.md",
        "680573606d128378489972f6a751663a91d4ef13905845cdfe7cba59007a94d0",
    ),
    (
        "experiments/identity_desupply_ladder_protocol.md",
        "493cc50ac67e7c3985b61e7fbe249f19388fed45b026d29dc81ad70f20c9ccbe",
    ),
    (
        "experiments/ffs_same1_compiled_correspondence_implementation_audit.md",
        "5a63816434a7c20539ba06895468869665c151f1f26faa2d0a00a1e0bffe4e44",
    ),
];

const CSV_HEADER: &str = "row_type,protocol,mode,claim_eligible,passed,seed,scale,depth,population,ownership,price_micros,generic_acquisition_work,compilation_acquisition_work,grounding_acquisition_work,installation_work,marginal_persistent_bytes,maintenance_work_per_use,supplied_runtime,generic_runtime,compiled_runtime,generic_to_compiled_saving,compiled_premium_vs_supplied,fixed_cost_micros,per_use_delta_micros,break_even_vs_supplied,compilation_break_even_vs_generic,classification,architectural_necessity,reconstructed,compiled,recursive_compatible,developmental_accelerator,mature_execution_accelerator,scaling_enabling_prerequisite,recursion_enabling_prerequisite,supplied_value,status,name,diagnostic";
const CSV_COLUMNS: usize = 39;

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

fn print_report(report: &Ip0Report, ancestry: bool) {
    println!(
        "IP0 {}: {}{}",
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
        "classification=EXECUTION_PRIOR_ADVANTAGE workloads={} generic_to_compiled=-{} compiled_vs_supplied=+{} compilation_break_even={} supplied_break_even=none",
        report.rows.len(),
        report.generic_to_compiled_reduction,
        report.compiled_premium_vs_supplied,
        report.zero_price_compilation_break_even,
    );
    println!(
        "necessity={} reconstruction={} compilation={} recursive={} developmental_accelerator={} mature_accelerator={} scaling_prior={} recursion_prior={}",
        report.ledger.architectural_necessity,
        report.ledger.reconstructed,
        report.ledger.compiled,
        report.ledger.recursive_compatible,
        report.ledger.developmental_accelerator,
        report.ledger.mature_execution_accelerator,
        report.ledger.scaling_enabling_prerequisite,
        report.ledger.recursion_enabling_prerequisite,
    );
    println!(
        "parents={} cs0b_skipped={} all_case1={} no_break_even={} ancestry={}",
        report.parent_artifacts_consistent,
        report.cs0b_skipped,
        report.all_execution_prior_advantage,
        report.all_no_break_even_vs_supplied,
        ancestry,
    );
}

fn base(report: &Ip0Report, passed: bool) -> Vec<String> {
    let mut fields = vec![String::new(); CSV_COLUMNS];
    fields[1] = IP0_PROTOCOL.to_string();
    fields[2] = report.mode.clone();
    fields[3] = report.claim_eligible.to_string();
    fields[4] = passed.to_string();
    fields
}

fn push_row(out: &mut String, fields: Vec<String>) {
    assert_eq!(fields.len(), CSV_COLUMNS);
    writeln!(out, "{}", fields.join(",")).unwrap();
}

fn csv(report: &Ip0Report, ancestry: bool) -> String {
    let mut out = String::new();
    writeln!(out, "{CSV_HEADER}").unwrap();
    for row in &report.rows {
        let mut fields = base(report, report.passed && ancestry);
        fields[0] = "accounting".to_string();
        fields[5] = row.seed.to_string();
        fields[6] = row.scale.clone();
        fields[7] = row.depth.to_string();
        fields[8] = row.population.to_string();
        fields[9] = row.ownership.label().to_string();
        fields[10] = row.price_micros.to_string();
        fields[11] = row.generic_acquisition_work.to_string();
        fields[12] = row.compilation_acquisition_work.to_string();
        fields[13] = row.grounding_acquisition_work.to_string();
        fields[14] = row.installation_work.to_string();
        fields[15] = row.marginal_persistent_bytes.to_string();
        fields[16] = row.maintenance_work_per_use.to_string();
        fields[17] = row.supplied_runtime.to_string();
        fields[18] = row.generic_runtime.to_string();
        fields[19] = row.compiled_runtime.to_string();
        fields[20] = row.generic_to_compiled_saving.to_string();
        fields[21] = row.compiled_premium_vs_supplied.to_string();
        fields[22] = row.fixed_cost_micros.to_string();
        fields[23] = row.per_use_delta_micros.to_string();
        fields[24] = row
            .break_even_vs_supplied
            .map(|value| value.to_string())
            .unwrap_or_default();
        fields[25] = row
            .compilation_break_even_vs_generic
            .map(|value| value.to_string())
            .unwrap_or_default();
        fields[26] = row.classification.label().to_string();
        fields[36] = "CLASSIFIED".to_string();
        fields[37] = "identity-prior-economics".to_string();
        fields[38] = "exact-integer-accounting".to_string();
        push_row(&mut out, fields);
    }
    let mut fields = base(report, report.passed && ancestry);
    fields[0] = "scaffold-ledger".to_string();
    fields[27] = report.ledger.architectural_necessity.clone();
    fields[28] = report.ledger.reconstructed.clone();
    fields[29] = report.ledger.compiled.clone();
    fields[30] = report.ledger.recursive_compatible.clone();
    fields[31] = report.ledger.developmental_accelerator.clone();
    fields[32] = report.ledger.mature_execution_accelerator.clone();
    fields[33] = report.ledger.scaling_enabling_prerequisite.clone();
    fields[34] = report.ledger.recursion_enabling_prerequisite.clone();
    fields[35] = report.ledger.supplied_value.clone();
    fields[36] = if report.passed && ancestry {
        "PASS"
    } else {
        "FAIL"
    }
    .to_string();
    fields[37] = report.ledger.scaffold.clone();
    fields[38] = format!(
        "parents={};cs0b_skipped={};no_break_even={}",
        report.parent_artifacts_consistent,
        report.cs0b_skipped,
        report.all_no_break_even_vs_supplied
    );
    push_row(&mut out, fields);
    out
}

fn markdown(report: &Ip0Report, ancestry: bool) -> String {
    let mut out = String::new();
    writeln!(out, "# IP0 identity-prior economics\n").unwrap();
    writeln!(out, "- Protocol: `{IP0_PROTOCOL}`").unwrap();
    writeln!(out, "- Mode: `{}`", report.mode).unwrap();
    writeln!(out, "- Claim eligible: `{}`", report.claim_eligible).unwrap();
    writeln!(out, "- Frozen ancestry valid: `{ancestry}`").unwrap();
    writeln!(
        out,
        "- Result: `{}`\n",
        if report.passed && ancestry {
            "PASS"
        } else {
            "FAIL"
        }
    )
    .unwrap();
    writeln!(out, "## Classification\n").unwrap();
    writeln!(out, "> Supplied filler equality is a nonessential but economically valuable substrate prior. Its function can be learned, compiled, and recursively reused without changing fractal organization; supplying it eliminates developmental acquisition/storage and a fixed six-work mature-use cost.\n").unwrap();
    writeln!(out, "| Dimension | Classification |").unwrap();
    writeln!(out, "|---|---|").unwrap();
    writeln!(
        out,
        "| Architectural necessity | {} |",
        report.ledger.architectural_necessity
    )
    .unwrap();
    writeln!(out, "| Reconstructed | {} |", report.ledger.reconstructed).unwrap();
    writeln!(out, "| Compiled | {} |", report.ledger.compiled).unwrap();
    writeln!(
        out,
        "| Full-fractal compatible | {} |",
        report.ledger.recursive_compatible
    )
    .unwrap();
    writeln!(
        out,
        "| Developmental accelerator | {} |",
        report.ledger.developmental_accelerator
    )
    .unwrap();
    writeln!(
        out,
        "| Mature execution accelerator | {} |",
        report.ledger.mature_execution_accelerator
    )
    .unwrap();
    writeln!(
        out,
        "| Scaling-enabling prerequisite | {} |",
        report.ledger.scaling_enabling_prerequisite
    )
    .unwrap();
    writeln!(
        out,
        "| Recursion-enabling prerequisite | {} |",
        report.ledger.recursion_enabling_prerequisite
    )
    .unwrap();
    writeln!(out, "\n## Frozen physical ledger\n").unwrap();
    writeln!(out, "| Quantity | Work / bytes |").unwrap();
    writeln!(out, "|---|---:|").unwrap();
    writeln!(out, "| Generic correspondence acquisition A | 860 work |").unwrap();
    writeln!(out, "| Compilation acquisition C | 988 work |").unwrap();
    writeln!(
        out,
        "| Optional CS0b grounding acquisition G | 0 work (SKIPPED) |"
    )
    .unwrap();
    writeln!(out, "| Installation I | 0 work |").unwrap();
    writeln!(out, "| Generic + compiled persistent S | 106 bytes |").unwrap();
    writeln!(out, "| Maintenance M_use / M_time | 0 / 0 work |").unwrap();
    writeln!(out, "| Generic learned mature premium | +18 work/use |").unwrap();
    writeln!(out, "| Compiled learned mature premium | +6 work/use |").unwrap();
    writeln!(out, "| Generic -> compiled saving | 12 work/use |").unwrap();
    writeln!(
        out,
        "| Zero-price compilation break-even vs generic | {} uses |",
        report.zero_price_compilation_break_even
    )
    .unwrap();
    writeln!(out, "\n## Supplied-SAME comparison\n").unwrap();
    writeln!(out, "For blank-start ownership at zero carrying price:\n").unwrap();
    writeln!(out, "```text").unwrap();
    writeln!(out, "Delta(H,d) = 860 + 988 + 6H").unwrap();
    writeln!(out, "           = 1848 + 6H > 0 for every H >= 0").unwrap();
    writeln!(out, "```").unwrap();
    writeln!(
        out,
        "\nFor an exact already-owned asset at zero marginal storage/acquisition:\n"
    )
    .unwrap();
    writeln!(out, "```text").unwrap();
    writeln!(out, "Delta(H,d) = 6H > 0 for every H > 0").unwrap();
    writeln!(out, "```").unwrap();
    writeln!(out, "\nAll prices `0, 1, 10, 100, 1000` millionths of work per byte-use increase or preserve the positive slope. Therefore no workload or ownership view has finite break-even against free supplied SAME.\n").unwrap();
    writeln!(out, "## Workload classifications\n").unwrap();
    writeln!(out, "| Seed | Scale | Depth | Population | W supplied | W generic | W compiled | Classification |").unwrap();
    writeln!(out, "|---:|---|---:|---:|---:|---:|---:|---|").unwrap();
    for row in report
        .rows
        .iter()
        .filter(|row| row.ownership.label() == "blank-start" && row.price_micros == 0)
    {
        writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            row.seed,
            row.scale,
            row.depth,
            row.population,
            row.supplied_runtime,
            row.generic_runtime,
            row.compiled_runtime,
            row.classification.label()
        )
        .unwrap();
    }
    writeln!(out, "\nNo learner, execution, correspondence, compilation, grounding, maintenance, or invalidation behavior was added or rerun.").unwrap();
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
        let report = run_ip0(mode);
        print_report(&report, ancestry);
        assert!(
            report.passed && ancestry,
            "IP0 development accounting failed"
        );
        return;
    }
    assert!(arguments.len() <= 2, "expected [csv] [markdown]");
    let csv_path = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/ip0_identity_prior_economics.csv"));
    let markdown_path = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/ip0_identity_prior_economics.md"));
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
    let report = run_ip0(mode);
    print_report(&report, ancestry);
    assert!(
        report.passed && ancestry,
        "IP0 definitive accounting failed"
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
        let report = run_ip0(HarnessMode::Micro);
        let output = csv(&report, true);
        let columns = CSV_HEADER.split(',').count();
        assert_eq!(columns, CSV_COLUMNS);
        assert!(output
            .lines()
            .all(|line| line.split(',').count() == columns));
    }

    #[test]
    fn development_modes_are_not_claim_eligible() {
        for mode in [HarnessMode::Micro, HarnessMode::Gate] {
            let report = run_ip0(mode);
            assert!(!report.claim_eligible);
            assert!(report.passed);
        }
    }
}
