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

fn marked_copy(source: &Path, destination: &Path, begin: &str, end: &str) {
    let text = fs::read_to_string(source).expect("marked source is readable");
    let mut copying = false;
    let mut output = String::new();
    for line in text.split_inclusive('\n') {
        if line.contains(begin) {
            copying = true;
            continue;
        }
        if line.contains(end) {
            break;
        }
        if copying {
            output.push_str(line);
        }
    }
    assert!(copying && !output.is_empty(), "marked source body exists");
    fs::write(destination, output).expect("marked include copy is writable");
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
        Path::new("src/ds_ac0_selected_affordance_actuation_closure.rs"),
        &output.join("ds_ac0_selected_affordance_actuation_closure.rs"),
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
    composition_copy(
        Path::new("src/ds_cp0_consequence_probation_coupling.rs"),
        &output.join("ds_cp0_consequence_probation_coupling.rs"),
    );
    composition_copy(
        Path::new("src/ds_rt0_retained_direction_execution.rs"),
        &output.join("ds_rt0_retained_direction_execution.rs"),
    );
    composition_copy(
        Path::new("src/ds_ir0_dependency_invalidation_reopening.rs"),
        &output.join("ds_ir0_dependency_invalidation_reopening.rs"),
    );
    composition_copy(
        Path::new("src/ds3_event_boundary.rs"),
        &output.join("ds3_event_boundary.rs"),
    );
    composition_copy(
        Path::new("src/ds3_event_boundary.rs"),
        &output.join("ds6_ds3_event_boundary.rs"),
    );
    composition_copy(
        Path::new("src/ds6_cumulative_lifetime_probe.rs"),
        &output.join("ds6_cumulative_lifetime_frozen.rs"),
    );
    composition_copy(
        Path::new("src/ds7_cumulative_plasticity_targeting_probe.rs"),
        &output.join("ds7_cumulative_plasticity_targeting_probe_frozen.rs"),
    );
    composition_copy(
        Path::new("src/ds7_cumulative_plasticity_allocation_gate.rs"),
        &output.join("ds7_cumulative_plasticity_allocation_gate_frozen.rs"),
    );
    composition_copy(
        Path::new("src/ds8_cumulative_semantic_credit_probe.rs"),
        &output.join("ds8_cumulative_semantic_credit_probe_frozen.rs"),
    );
    let ds8_linker = output.join("ds8_cumulative_semantic_credit_linker_frozen.rs");
    marked_copy(
        Path::new("src/ds8_cumulative_semantic_credit_probe.rs"),
        &ds8_linker,
        "// DS8_ORGANISM_PATH_BEGIN",
        "// DS8_ORGANISM_PATH_END",
    );
    println!(
        "cargo:rustc-env=DS8_MICRO_LINKER_FRAGMENT_SHA256={}",
        file_sha256(ds8_linker.to_str().expect("generated path is UTF-8"))
    );
    composition_copy(
        Path::new("src/ds3_cumulative_event_boundary_port.rs"),
        &output.join("ds3_cumulative_event_boundary_port.rs"),
    );
    composition_copy(
        Path::new("src/ds3_cumulative_event_boundary_port.rs"),
        &output.join("ds4_m3_cumulative_event_boundary_port.rs"),
    );
    composition_copy(
        Path::new("src/request_roles.rs"),
        &output.join("ds4_request_roles.rs"),
    );
    composition_copy(
        Path::new("src/request_roles.rs"),
        &output.join("request_roles.rs"),
    );
    composition_copy(
        Path::new("src/ds4_cumulative_request_start_port.rs"),
        &output.join("ds4_cumulative_request_start_port.rs"),
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
        (
            "DS2_CP0_RETRY_PRIOR_SHA256",
            "src/ds2_after_ap0_mechanistic_retry.rs",
        ),
        (
            "DS2_CP0_RETRY_CP0_SHA256",
            "src/ds_cp0_consequence_probation_coupling.rs",
        ),
        (
            "DS2_CP0_RETRY_READINESS_SHA256",
            "experiments/ds_cp0_development_readiness_handoff.md",
        ),
        (
            "DS2_CP0_RETRY_PROTOCOL_SHA256",
            "experiments/ds2_after_cp0_mechanistic_retry_protocol.md",
        ),
        (
            "DS_RT0_CP0_SHA256",
            "src/ds_cp0_consequence_probation_coupling.rs",
        ),
        ("DS_RT0_A1_SHA256", "src/ds_a1_affordance_multiplicity.rs"),
        (
            "DS_RT0_PARENT_SHA256",
            "experiments/ds2_after_cp0_mechanistic_retry_collapse_handoff.md",
        ),
        (
            "DS_RT0_PROTOCOL_SHA256",
            "experiments/ds_rt0_retained_direction_execution_protocol.md",
        ),
        (
            "DS2_RT0_RETRY_PRIOR_SHA256",
            "src/ds2_after_cp0_mechanistic_retry.rs",
        ),
        (
            "DS2_RT0_RETRY_RT0_SHA256",
            "src/ds_rt0_retained_direction_execution.rs",
        ),
        (
            "DS2_RT0_RETRY_READINESS_SHA256",
            "experiments/ds_rt0_development_readiness_handoff.md",
        ),
        (
            "DS2_RT0_RETRY_PROTOCOL_SHA256",
            "experiments/ds2_after_rt0_mechanistic_retry_protocol.md",
        ),
        (
            "DS_IR0_RT0_SHA256",
            "src/ds_rt0_retained_direction_execution.rs",
        ),
        (
            "DS_IR0_CP0_SHA256",
            "src/ds_cp0_consequence_probation_coupling.rs",
        ),
        ("DS_IR0_A1_SHA256", "src/ds_a1_affordance_multiplicity.rs"),
        (
            "DS_IR0_PARENT_SHA256",
            "experiments/ds2_after_rt0_mechanistic_retry_collapse_handoff.md",
        ),
        (
            "DS_IR0_PROTOCOL_SHA256",
            "experiments/ds_ir0_dependency_invalidation_reopening_protocol.md",
        ),
        (
            "DS2_IR0_RETRY_PRIOR_SHA256",
            "src/ds2_after_rt0_mechanistic_retry.rs",
        ),
        (
            "DS2_IR0_RETRY_IR0_SHA256",
            "src/ds_ir0_dependency_invalidation_reopening.rs",
        ),
        (
            "DS2_IR0_RETRY_READINESS_SHA256",
            "experiments/ds_ir0_development_readiness_handoff.md",
        ),
        (
            "DS2_IR0_RETRY_PROTOCOL_SHA256",
            "experiments/ds2_after_ir0_mechanistic_retry_protocol.md",
        ),
        (
            "DS2_DEFINITIVE_PARENT_SHA256",
            "src/ds2_after_ir0_mechanistic_retry.rs",
        ),
        (
            "DS2_DEFINITIVE_PARENT_HANDOFF_SHA256",
            "experiments/ds2_after_ir0_mechanistic_retry_development_handoff.md",
        ),
        (
            "DS2_DEFINITIVE_PROTOCOL_SHA256",
            "experiments/ds2_cumulative_causal_direction_definitive_protocol.md",
        ),
        ("DS3_CUM_MECHANISM_SHA256", "src/ds3_event_boundary.rs"),
        ("DS3_CUM_A1_SHA256", "src/ds_a1_affordance_multiplicity.rs"),
        (
            "DS3_CUM_AC0_SHA256",
            "src/ds_ac0_selected_affordance_actuation_closure.rs",
        ),
        (
            "DS3_CUM_IR0_SHA256",
            "src/ds_ir0_dependency_invalidation_reopening.rs",
        ),
        (
            "DS3_CUM_PROTOCOL_SHA256",
            "experiments/ds3_cumulative_event_boundary_protocol.md",
        ),
        (
            "DS3_CUM_EXPECTATION_SHA256",
            "experiments/ds3_cumulative_event_boundary_expectation_freeze.md",
        ),
        (
            "DS3_DEFINITIVE_PARENT_SHA256",
            "src/ds3_cumulative_event_boundary_port.rs",
        ),
        (
            "DS3_DEFINITIVE_PARENT_HANDOFF_SHA256",
            "experiments/ds3_cumulative_event_boundary_development_handoff.md",
        ),
        (
            "DS3_DEFINITIVE_PROTOCOL_SHA256",
            "experiments/ds3_cumulative_event_boundary_definitive_protocol.md",
        ),
        (
            "DS4_M3_PORT_SHA256",
            "src/ds3_cumulative_event_boundary_port.rs",
        ),
        (
            "DS4_M3_DEFINITIVE_SHA256",
            "src/ds3_cumulative_event_boundary_definitive.rs",
        ),
        (
            "DS4_M3_RESULT_CSV_SHA256",
            "results/ds3_cumulative_event_boundary_definitive.csv",
        ),
        (
            "DS4_M3_RESULT_MD_SHA256",
            "results/ds3_cumulative_event_boundary_definitive.md",
        ),
        ("DS4_P4_SHA256", "src/request_roles.rs"),
        ("DS4_P4_RESULT_CSV_SHA256", "results/p4_request_roles.csv"),
        ("DS4_P4_RESULT_MD_SHA256", "results/p4_request_roles.md"),
        (
            "DS4_TARGET_FREEZE_SHA256",
            "experiments/ds4_ds8_target_freeze.md",
        ),
        (
            "DS4_PROTOCOL_SHA256",
            "experiments/ds4_cumulative_request_start_protocol.md",
        ),
        (
            "DS4_DEFINITIVE_PARENT_SHA256",
            "src/ds4_cumulative_request_start_port.rs",
        ),
        (
            "DS4_DEFINITIVE_RUNNER_SHA256",
            "src/bin/ds4_cumulative_request_start_port.rs",
        ),
        (
            "DS4_DEFINITIVE_HANDOFF_SHA256",
            "experiments/ds4_cumulative_request_start_development_handoff.md",
        ),
        (
            "DS4_DEFINITIVE_PROTOCOL_SHA256",
            "experiments/ds4_cumulative_request_start_definitive_protocol.md",
        ),
        ("DS6_M3_SHA256", "src/ds3_event_boundary.rs"),
        ("DS6_TARGET_SHA256", "experiments/ds4_ds8_target_freeze.md"),
        (
            "DS6_ORDER_SHA256",
            "experiments/desupply_sprint_order_amendment_after_ds4_negative.md",
        ),
        (
            "DS6_AUDIT_SHA256",
            "experiments/ds6_cumulative_lifetime_dependency_audit.md",
        ),
        (
            "DS6_PROTOCOL_SHA256",
            "experiments/ds6_cumulative_lifetime_probe_protocol.md",
        ),
        (
            "DS6_DEFINITIVE_DEVELOPMENT_SHA256",
            "src/ds6_cumulative_lifetime_probe.rs",
        ),
        (
            "DS6_DEFINITIVE_HANDOFF_SHA256",
            "experiments/ds6_cumulative_lifetime_development_handoff.md",
        ),
        (
            "DS6_DEFINITIVE_M3_HANDOFF_SHA256",
            "experiments/m3_cumulative_event_boundary_authoritative_handoff.md",
        ),
        (
            "DS6_DEFINITIVE_M3_CSV_SHA256",
            "results/ds3_cumulative_event_boundary_definitive.csv",
        ),
        (
            "DS6_DEFINITIVE_M3_MD_SHA256",
            "results/ds3_cumulative_event_boundary_definitive.md",
        ),
        (
            "DS6_DEFINITIVE_PROTOCOL_SHA256",
            "experiments/ds6_cumulative_lifetime_definitive_protocol.md",
        ),
        (
            "DS7_ACTIVATION_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_activation_handoff.md",
        ),
        (
            "DS7_AUDIT_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_dependency_audit.md",
        ),
        (
            "DS7_MANIFEST_SHA256",
            "experiments/desupply_ds4_ds8_dependency_manifest.csv",
        ),
        ("DS7_P2_SHA256", "src/local_plasticity.rs"),
        ("DS7_M4_SHA256", "src/ds6_cumulative_lifetime_probe.rs"),
        (
            "DS7_PROTOCOL_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_probe_protocol.md",
        ),
        (
            "DS7_MICRO_PROBE_RESULT_SHA256",
            "results/ds7_cumulative_plasticity_allocation_probe.md",
        ),
        (
            "DS7_MICRO_PROBE_AUDIT_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_probe_result_audit.md",
        ),
        (
            "DS7_MICRO_PROTOCOL_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_micro_protocol.md",
        ),
        (
            "DS7_GATE_HANDOFF_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_development_handoff.md",
        ),
        (
            "DS7_GATE_MICRO_RESULT_SHA256",
            "results/ds7_cumulative_plasticity_allocation_micro.md",
        ),
        (
            "DS7_GATE_MICRO_AUDIT_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_micro_result_audit.md",
        ),
        (
            "DS7_GATE_MICRO_SHA256",
            "src/ds7_cumulative_plasticity_allocation_micro.rs",
        ),
        (
            "DS7_GATE_RT0_SHA256",
            "src/ds_rt0_retained_direction_execution.rs",
        ),
        (
            "DS7_GATE_PROTOCOL_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_gate_protocol.md",
        ),
        (
            "DS7_DEFINITIVE_TARGET_SHA256",
            "experiments/ds4_ds8_target_freeze.md",
        ),
        (
            "DS7_DEFINITIVE_ORDER_SHA256",
            "experiments/desupply_sprint_order_amendment_after_ds4_negative.md",
        ),
        (
            "DS7_DEFINITIVE_M4_HANDOFF_SHA256",
            "experiments/m4_cumulative_learned_lifetime_authoritative_handoff.md",
        ),
        (
            "DS7_DEFINITIVE_M4_CSV_SHA256",
            "results/ds6_cumulative_lifetime_definitive.csv",
        ),
        (
            "DS7_DEFINITIVE_M4_MD_SHA256",
            "results/ds6_cumulative_lifetime_definitive.md",
        ),
        (
            "DS7_DEFINITIVE_M4_SOURCE_SHA256",
            "src/ds6_cumulative_lifetime_probe.rs",
        ),
        (
            "DS7_DEFINITIVE_ACTIVATION_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_activation_handoff.md",
        ),
        (
            "DS7_DEFINITIVE_DEPENDENCY_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_dependency_audit.md",
        ),
        (
            "DS7_DEFINITIVE_MANIFEST_SHA256",
            "experiments/desupply_ds4_ds8_dependency_manifest.csv",
        ),
        ("DS7_DEFINITIVE_P2_SHA256", "src/local_plasticity.rs"),
        (
            "DS7_DEFINITIVE_PROBE_PROTOCOL_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_probe_protocol.md",
        ),
        (
            "DS7_DEFINITIVE_ALLOCATOR_SHA256",
            "src/ds7_cumulative_plasticity_targeting_probe.rs",
        ),
        (
            "DS7_DEFINITIVE_PROBE_RUNNER_SHA256",
            "src/bin/ds7_cumulative_plasticity_targeting_probe.rs",
        ),
        (
            "DS7_DEFINITIVE_PROBE_RESULT_SHA256",
            "results/ds7_cumulative_plasticity_allocation_probe.md",
        ),
        (
            "DS7_DEFINITIVE_PROBE_AUDIT_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_probe_result_audit.md",
        ),
        (
            "DS7_DEFINITIVE_MICRO_PROTOCOL_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_micro_protocol.md",
        ),
        (
            "DS7_DEFINITIVE_MICRO_SOURCE_SHA256",
            "src/ds7_cumulative_plasticity_allocation_micro.rs",
        ),
        (
            "DS7_DEFINITIVE_MICRO_RUNNER_SHA256",
            "src/bin/ds7_cumulative_plasticity_allocation_micro.rs",
        ),
        (
            "DS7_DEFINITIVE_MICRO_RESULT_SHA256",
            "results/ds7_cumulative_plasticity_allocation_micro.md",
        ),
        (
            "DS7_DEFINITIVE_MICRO_AUDIT_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_micro_result_audit.md",
        ),
        (
            "DS7_DEFINITIVE_DEVELOPMENT_HANDOFF_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_development_handoff.md",
        ),
        (
            "DS7_DEFINITIVE_V2_RESULT_SHA256",
            "results/ds7_cumulative_plasticity_allocation_gate_v2.md",
        ),
        (
            "DS7_DEFINITIVE_V2_AUDIT_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_gate_v2_negative_audit.md",
        ),
        (
            "DS7_DEFINITIVE_COLLAPSE_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_gate_v2_collapse_handoff.md",
        ),
        (
            "DS7_DEFINITIVE_GATE_PROTOCOL_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_gate_protocol.md",
        ),
        (
            "DS7_DEFINITIVE_GATE_SOURCE_SHA256",
            "src/ds7_cumulative_plasticity_allocation_gate.rs",
        ),
        (
            "DS7_DEFINITIVE_GATE_RUNNER_SHA256",
            "src/bin/ds7_cumulative_plasticity_allocation_gate.rs",
        ),
        (
            "DS7_DEFINITIVE_GATE_RESULT_SHA256",
            "results/ds7_cumulative_plasticity_allocation_gate_v3.md",
        ),
        (
            "DS7_DEFINITIVE_GATE_AUDIT_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_gate_v3_result_audit.md",
        ),
        (
            "DS7_DEFINITIVE_READINESS_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_definitive_readiness_handoff.md",
        ),
        (
            "DS7_DEFINITIVE_PROTOCOL_SHA256",
            "experiments/ds7_cumulative_plasticity_allocation_definitive_protocol.md",
        ),
        (
            "DS8_ACTIVATION_SHA256",
            "experiments/ds8_cumulative_semantic_credit_activation_handoff.md",
        ),
        (
            "DS8_AUDIT_SHA256",
            "experiments/ds8_cumulative_semantic_credit_dependency_audit.md",
        ),
        (
            "DS8_PROTOCOL_SHA256",
            "experiments/ds8_cumulative_semantic_credit_probe_protocol.md",
        ),
        (
            "DS8_PROTOCOL_V2_SHA256",
            "experiments/ds8_cumulative_semantic_credit_probe_protocol_v2.md",
        ),
        (
            "DS8_M5_ALLOCATOR_SHA256",
            "src/ds7_cumulative_plasticity_targeting_probe.rs",
        ),
        (
            "DS8_M5_GATE_SHA256",
            "src/ds7_cumulative_plasticity_allocation_gate.rs",
        ),
        (
            "DS8_M5_CSV_SHA256",
            "results/ds7_cumulative_plasticity_allocation_definitive.csv",
        ),
        (
            "DS8_M5_MD_SHA256",
            "results/ds7_cumulative_plasticity_allocation_definitive.md",
        ),
        (
            "DS8_D3_SHA256",
            "src/ds_d3_anonymous_consequence_contrast.rs",
        ),
        ("DS8_D2_SHA256", "src/ds_d2_differential_evidence.rs"),
        ("DS8_C0_SHA256", "src/ds_c0_anonymous_credit_coupling.rs"),
        (
            "DS8_CP0_SHA256",
            "src/ds_cp0_consequence_probation_coupling.rs",
        ),
        (
            "DS8_MICRO_PROBE_SHA256",
            "src/ds8_cumulative_semantic_credit_probe.rs",
        ),
        (
            "DS8_MICRO_RESULT_SHA256",
            "results/ds8_cumulative_semantic_credit_probe_v2.md",
        ),
        (
            "DS8_MICRO_AUDIT_SHA256",
            "experiments/ds8_cumulative_semantic_credit_probe_v2_result_audit.md",
        ),
        (
            "DS8_MICRO_HANDOFF_SHA256",
            "experiments/ds8_cumulative_semantic_credit_probe_handoff.md",
        ),
        (
            "DS8_MICRO_PROTOCOL_SHA256",
            "experiments/ds8_cumulative_semantic_credit_micro_protocol.md",
        ),
        (
            "DS8_MICRO_PROTOCOL_V2_SHA256",
            "experiments/ds8_cumulative_semantic_credit_micro_protocol_v2.md",
        ),
    ] {
        println!("cargo:rustc-env={name}={}", file_sha256(path));
        println!("cargo:rerun-if-changed={path}");
    }
}
