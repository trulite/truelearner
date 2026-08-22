use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn file_sha256(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .or_else(|| {
            Command::new("shasum")
                .args(["-a", "256", path])
                .output()
                .ok()
                .filter(|output| output.status.success())
        })
        .expect("a SHA-256 utility is available");
    String::from_utf8(output.stdout)
        .expect("SHA-256 output is UTF-8")
        .split_whitespace()
        .next()
        .expect("SHA-256 output contains a digest")
        .to_string()
}

fn composition_copy(source: &Path, destination: &Path) {
    let text = fs::read_to_string(source).expect("frozen composition source is readable");
    let mut body_started = false;
    let mut output = String::new();
    for line in text.split_inclusive('\n') {
        if !body_started
            && (line.trim().is_empty()
                || line.starts_with("//!")
                || line.starts_with("#![allow(dead_code)]"))
        {
            continue;
        }
        body_started = true;
        output.push_str(line);
    }
    fs::write(destination, output).expect("composition include copy is writable");
    println!("cargo:rerun-if-changed={}", source.display());
}

fn main() {
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo supplies OUT_DIR"));
    let bin_output = output.join("bin");
    fs::create_dir_all(&bin_output).expect("generated bin directory is writable");
    composition_copy(
        Path::new("src/ds_e0_anonymous_event_formation.rs"),
        &output.join("ds_e0_anonymous_event_formation.rs"),
    );
    composition_copy(
        Path::new("src/ds_a0_anonymous_boundary_action_formation.rs"),
        &output.join("ds_a0_anonymous_boundary_action_formation.rs"),
    );
    composition_copy(
        Path::new("src/ds_a1_affordance_multiplicity.rs"),
        &output.join("ds_a1_affordance_multiplicity.rs"),
    );
    composition_copy(
        Path::new("src/ds1_after_e0_a0_composition_retry.rs"),
        &output.join("ds1_after_e0_a0_composition_retry.rs"),
    );
    composition_copy(
        Path::new("src/ds_r0_anonymous_post_action_evidence_return.rs"),
        &output.join("ds_r0_anonymous_post_action_evidence_return.rs"),
    );
    composition_copy(
        Path::new("src/ds_d2_differential_evidence.rs"),
        &output.join("ds_d2_differential_evidence.rs"),
    );
    composition_copy(
        Path::new("src/ds_d3_anonymous_consequence_contrast.rs"),
        &output.join("ds_d3_anonymous_consequence_contrast.rs"),
    );
    composition_copy(
        Path::new("src/ds1_after_d3_cumulative_composition_retry.rs"),
        &output.join("ds1_after_d3_cumulative_composition_retry.rs"),
    );
    fs::copy(
        "src/bin/ds_a0_anonymous_boundary_action_formation.rs",
        bin_output.join("ds_a0_anonymous_boundary_action_formation.rs"),
    )
    .expect("frozen A0 runner audit copy is writable");
    println!("cargo:rerun-if-changed=src/bin/ds_a0_anonymous_boundary_action_formation.rs");
    for (name, path) in [
        ("DS_A1_E0_SHA256", "src/ds_e0_anonymous_event_formation.rs"),
        (
            "DS_A1_A0_SHA256",
            "src/ds_a0_anonymous_boundary_action_formation.rs",
        ),
        (
            "DS_A1_PRIOR_SHA256",
            "src/ds1_after_e0_a0_composition_retry.rs",
        ),
        ("DS1_A1_E0_SHA256", "src/ds_e0_anonymous_event_formation.rs"),
        (
            "DS1_A1_A0_SHA256",
            "src/ds_a0_anonymous_boundary_action_formation.rs",
        ),
        ("DS1_A1_A1_SHA256", "src/ds_a1_affordance_multiplicity.rs"),
        ("DS1_A1_M0_SHA256", "src/ffs_same0.rs"),
        ("DS1_A1_COMPILED_M0_SHA256", "src/ffs_same0/cs0a.rs"),
        (
            "DS1_A1_READINESS_SHA256",
            "experiments/ds_a1_development_readiness_handoff.md",
        ),
        ("DS_R0_E0_SHA256", "src/ds_e0_anonymous_event_formation.rs"),
        ("DS_R0_A1_SHA256", "src/ds_a1_affordance_multiplicity.rs"),
        (
            "DS_R0_PARENT_SHA256",
            "src/ds1_after_e0_a0_a1_composition_retry.rs",
        ),
        (
            "DS_R0_PARENT_HANDOFF_SHA256",
            "experiments/ds1_after_e0_a0_a1_composition_collapse_handoff.md",
        ),
        (
            "DS1_R0_R0_SHA256",
            "src/ds_r0_anonymous_post_action_evidence_return.rs",
        ),
        (
            "DS1_R0_READINESS_SHA256",
            "experiments/ds_r0_development_readiness_handoff.md",
        ),
        (
            "DS1_R0_E2B_SHA256",
            "experiments/ds_r0_e2b_validation_amendment.md",
        ),
        (
            "DS_C0_R0_SHA256",
            "src/ds_r0_anonymous_post_action_evidence_return.rs",
        ),
        (
            "DS_C0_PARENT_RETRY_SHA256",
            "src/ds1_after_r0_composition_retry.rs",
        ),
        (
            "DS_C0_PARENT_HANDOFF_SHA256",
            "experiments/ds1_after_r0_composition_collapse_handoff.md",
        ),
        ("DS1_C0_C0_SHA256", "src/ds_c0_anonymous_credit_coupling.rs"),
        (
            "DS1_C0_READINESS_SHA256",
            "experiments/ds_c0_development_readiness_handoff.md",
        ),
        (
            "DS_D0_PARENT_RETRY_SHA256",
            "src/ds1_after_c0_composition_retry.rs",
        ),
        (
            "DS_D0_PARENT_HANDOFF_SHA256",
            "experiments/ds1_after_c0_composition_collapse_handoff.md",
        ),
        ("DS_D0_C0_SHA256", "src/ds_c0_anonymous_credit_coupling.rs"),
        ("DS_D0_E0_SHA256", "src/ds_e0_anonymous_event_formation.rs"),
        (
            "DS_D1_D0_SOURCE_SHA256",
            "src/ds_d0_stage8b_discrimination.rs",
        ),
        (
            "DS_D1_D0_HANDOFF_SHA256",
            "experiments/ds_d0_stage8b_discrimination_handoff.md",
        ),
        ("DS_D1_E0_SHA256", "src/ds_e0_anonymous_event_formation.rs"),
        (
            "DS_D2_D1_SOURCE_SHA256",
            "src/ds_d1_stage8b_functional_sufficiency.rs",
        ),
        (
            "DS_D2_D1_HANDOFF_SHA256",
            "experiments/ds_d1_stage8b_functional_sufficiency_handoff.md",
        ),
        ("DS_D2_A1_SHA256", "src/ds_a1_affordance_multiplicity.rs"),
        ("DS1_D2_D2_SHA256", "src/ds_d2_differential_evidence.rs"),
        (
            "DS1_D2_D2_HANDOFF_SHA256",
            "experiments/ds_d2_differential_evidence_handoff.md",
        ),
        ("DS1_D2_C0_SHA256", "src/ds_c0_anonymous_credit_coupling.rs"),
        (
            "DS1_D2_C0_READINESS_SHA256",
            "experiments/ds_c0_development_readiness_handoff.md",
        ),
        ("DS1_D2_E0_SHA256", "src/ds_e0_anonymous_event_formation.rs"),
        (
            "DS1_D2_PROTOCOL_AMENDMENT_SHA256",
            "experiments/ds1_after_d2_cumulative_composition_retry_protocol_amendment.md",
        ),
        ("DS_D3_D2_SHA256", "src/ds_d2_differential_evidence.rs"),
        (
            "DS_D3_D2_HANDOFF_SHA256",
            "experiments/ds_d2_differential_evidence_handoff.md",
        ),
        (
            "DS_D3_PARENT_RETRY_SHA256",
            "src/ds1_after_d2_cumulative_composition_retry.rs",
        ),
        (
            "DS_D3_PARENT_HANDOFF_SHA256",
            "experiments/ds1_after_d2_cumulative_composition_collapse_handoff.md",
        ),
        (
            "DS1_D3_D3_SHA256",
            "src/ds_d3_anonymous_consequence_contrast.rs",
        ),
        (
            "DS1_D3_D3_READINESS_SHA256",
            "experiments/ds_d3_development_readiness_handoff.md",
        ),
        ("DS1_D3_E0_SHA256", "src/ds_e0_anonymous_event_formation.rs"),
        (
            "DS1_DEFINITIVE_PARENT_SHA256",
            "src/ds1_after_d3_cumulative_composition_retry.rs",
        ),
        (
            "DS1_DEFINITIVE_PARENT_HANDOFF_SHA256",
            "experiments/ds1_after_d3_cumulative_composition_handoff.md",
        ),
        (
            "DS1_DEFINITIVE_PROTOCOL_SHA256",
            "experiments/ds1_boundary_role_cumulative_definitive_protocol.md",
        ),
        (
            "DS2_M1_DEFINITIVE_CORE_SHA256",
            "src/ds1_boundary_role_cumulative_definitive.rs",
        ),
        (
            "DS2_M1_PARENT_SHA256",
            "src/ds1_after_d3_cumulative_composition_retry.rs",
        ),
        (
            "DS2_M1_RESULT_CSV_SHA256",
            "results/ds1_boundary_role_cumulative_definitive.csv",
        ),
        (
            "DS2_M1_RESULT_MD_SHA256",
            "results/ds1_boundary_role_cumulative_definitive.md",
        ),
        (
            "DS2_M1_PROTOCOL_SHA256",
            "experiments/ds2_cumulative_m1_mechanistic_probe_protocol.md",
        ),
        ("DS_AC0_A1_SHA256", "src/ds_a1_affordance_multiplicity.rs"),
        (
            "DS_AC0_M1_PARENT_SHA256",
            "src/ds1_after_d3_cumulative_composition_retry.rs",
        ),
        (
            "DS_AC0_COLLAPSE_SHA256",
            "experiments/ds2_cumulative_m1_mechanistic_probe_collapse_handoff.md",
        ),
        (
            "DS_AC0_PROTOCOL_SHA256",
            "experiments/ds_ac0_selected_affordance_actuation_closure_protocol.md",
        ),
        (
            "DS2_RETRY_PRIOR_PROBE_SHA256",
            "src/ds2_cumulative_m1_mechanistic_probe.rs",
        ),
        (
            "DS2_RETRY_AC0_SHA256",
            "src/ds_ac0_selected_affordance_actuation_closure.rs",
        ),
        (
            "DS2_RETRY_AC0_READINESS_SHA256",
            "experiments/ds_ac0_development_readiness_handoff.md",
        ),
        (
            "DS2_RETRY_PROTOCOL_SHA256",
            "experiments/ds2_after_ac0_mechanistic_retry_protocol.md",
        ),
        (
            "DS_AP0_AC0_SHA256",
            "src/ds_ac0_selected_affordance_actuation_closure.rs",
        ),
        (
            "DS_AP0_PARENT_SHA256",
            "experiments/ds2_after_ac0_mechanistic_retry_collapse_handoff.md",
        ),
        (
            "DS_AP0_PROTOCOL_SHA256",
            "experiments/ds_ap0_aftermath_plasticity_activation_protocol.md",
        ),
        (
            "DS2_AP0_RETRY_PRIOR_SHA256",
            "src/ds2_after_ac0_mechanistic_retry.rs",
        ),
        (
            "DS2_AP0_RETRY_AP0_SHA256",
            "src/ds_ap0_aftermath_plasticity_activation.rs",
        ),
        (
            "DS2_AP0_RETRY_READINESS_SHA256",
            "experiments/ds_ap0_development_readiness_handoff.md",
        ),
        (
            "DS2_AP0_RETRY_PROTOCOL_SHA256",
            "experiments/ds2_after_ap0_mechanistic_retry_protocol.md",
        ),
        (
            "DS_CP0_D3_SHA256",
            "src/ds_d3_anonymous_consequence_contrast.rs",
        ),
        ("DS_CP0_A1_SHA256", "src/ds_a1_affordance_multiplicity.rs"),
        (
            "DS_CP0_PARENT_SHA256",
            "experiments/ds2_after_ap0_mechanistic_retry_collapse_handoff.md",
        ),
        (
            "DS_CP0_PROTOCOL_SHA256",
            "experiments/ds_cp0_consequence_probation_coupling_protocol.md",
        ),
    ] {
        println!("cargo:rustc-env={name}={}", file_sha256(path));
        println!("cargo:rerun-if-changed={path}");
    }
}
