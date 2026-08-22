//! Development-only unchanged DS1-on-DS-E0 composition audit.
//!
//! DS-E0 performs the only `Neighborhood` construction. Its existing probe
//! invokes the frozen learner read-only. This harness freezes before choice
//! when no two actual anonymous substrate actions are available.

use crate::ds_e0_anonymous_event_formation::{self as ds_e0, GateReport, WorkLedger};
use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds1-after-e0-cumulative-composition-attempt-v1";
pub const ENABLING_PARENT: &str = "d154fde5632c0ba9d76fc2d1d1a700276045adc8";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_DS_E0_SOURCE_SHA256: &str =
    "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615";
pub const FROZEN_M0_SOURCE_SHA256: &str =
    "50cf169bb293177a35270adde656f28f98e68c83a4d39d2876399261b7ee697c";
pub const FROZEN_M0_COMPILED_SOURCE_SHA256: &str =
    "430cd2206c8baa7106c4de7f203d4d0c48b544290e6266596ebcdb91d02655c9";

pub const COLLAPSE_STAGE: &str =
    "4. actual anonymous boundary-ordering/action alternatives available from current substrate";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompositionLedger {
    pub e0_work: WorkLedger,
    pub e0_persistent_bytes: usize,
    pub e0_temporary_peak_bytes: usize,
    pub ds1_read_only_invocations: u64,
    pub ds1_observation_comparisons: Option<u64>,
    pub ds1_candidate_evaluations: Option<u64>,
    pub ds1_proposals: Option<u64>,
    pub ds1_route_firings: Option<u64>,
    pub ds1_credit_updates: Option<u64>,
    pub selected_action_execution_work: Option<u64>,
    pub ordinary_consequence_work: Option<u64>,
    pub ds1_persistent_bytes: Option<usize>,
    pub maintenance_work: Option<u64>,
    pub carrying_work: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct SeedCompositionAudit {
    pub seed: u64,
    pub e0_a_ready: bool,
    pub e0_b_ready: bool,
    pub ds1_neighborhood_consumed: bool,
    pub actual_anonymous_actions_available: bool,
    pub selected_action_physically_executed: Option<bool>,
    pub route_contingent_consequence_returned: Option<bool>,
    pub learner_acquired: Option<bool>,
    pub transfer_passed: Option<bool>,
    pub invalidation_reopening_passed: Option<bool>,
    pub functional_recovery_passed: Option<bool>,
    pub ledger: CompositionLedger,
}

#[derive(Clone, Debug)]
pub struct CompositionReport {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub m0_authoritative: bool,
    pub enabling_parent: String,
    pub m1_exists: bool,
    pub exact_lineage_and_fingerprints: String,
    pub ds_e0_event_formation: String,
    pub e0_b_serialization: String,
    pub frozen_ds1_consumption: String,
    pub anonymous_action_alternatives: String,
    pub selected_action_execution: String,
    pub ordinary_consequence: String,
    pub ds1_acquisition: String,
    pub transfer: String,
    pub invalidation_reopening: String,
    pub functional_recovery: String,
    pub first_collapse: String,
    pub frozen_ds1_sha256: String,
    pub ds_e0_source_sha256: String,
    pub seeds: Vec<SeedCompositionAudit>,
    pub audit_passed: bool,
}

fn source_boundary_audit() -> bool {
    let e0_source = include_str!("ds_e0_anonymous_event_formation.rs");
    let wiring_source = include_str!("ds1_after_e0_cumulative_composition.rs");
    let learner_markers_once = e0_source.matches("// DS1_LEARNER_BEGIN").count() == 2
        && e0_source.matches("// DS1_LEARNER_END").count() == 2;
    let serializer_once = e0_source.matches("fn serialize_once(").count() == 1;
    let existing_probe = e0_source.contains("frozen.frozen_choice(&neighborhood).is_none()");
    let forbidden_wiring = [
        concat!("fn isolated_", "fixture"),
        concat!("synthetic_two_", "route"),
        concat!("expected_", "action"),
    ];
    let no_isolated_fixture = forbidden_wiring
        .iter()
        .all(|forbidden| !wiring_source.contains(forbidden));
    ds_e0::FROZEN_DS1_LEARNER_SHA256 == FROZEN_DS1_SHA256
        && learner_markers_once
        && serializer_once
        && existing_probe
        && no_isolated_fixture
}

fn audit_seed(seed: &ds_e0::SeedReport) -> SeedCompositionAudit {
    SeedCompositionAudit {
        seed: seed.seed,
        e0_a_ready: seed.passed && seed.e0_a_formed == seed.e0_a_presentations,
        e0_b_ready: seed.passed && seed.e0_b_exact_copies == seed.e0_a_presentations * 6,
        ds1_neighborhood_consumed: seed.frozen_ds1_consumption_probe,
        actual_anonymous_actions_available: false,
        selected_action_physically_executed: None,
        route_contingent_consequence_returned: None,
        learner_acquired: None,
        transfer_passed: None,
        invalidation_reopening_passed: None,
        functional_recovery_passed: None,
        ledger: CompositionLedger {
            e0_work: seed.work.clone(),
            e0_persistent_bytes: seed.persistent_bytes,
            e0_temporary_peak_bytes: seed.temporary_peak_bytes,
            ds1_read_only_invocations: u64::from(seed.frozen_ds1_consumption_probe),
            ds1_observation_comparisons: None,
            ds1_candidate_evaluations: None,
            ds1_proposals: None,
            ds1_route_firings: None,
            ds1_credit_updates: None,
            selected_action_execution_work: None,
            ordinary_consequence_work: None,
            ds1_persistent_bytes: None,
            maintenance_work: None,
            carrying_work: None,
        },
    }
}

fn definitive_rejection() -> CompositionReport {
    CompositionReport {
        label: "CUMULATIVE DS1 DEVELOPMENT".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "definitive-forbidden".to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        enabling_parent: ENABLING_PARENT.to_string(),
        m1_exists: false,
        exact_lineage_and_fingerprints: "NOT RUN: definitive rejected".to_string(),
        ds_e0_event_formation: "BLOCKED".to_string(),
        e0_b_serialization: "BLOCKED".to_string(),
        frozen_ds1_consumption: "BLOCKED".to_string(),
        anonymous_action_alternatives: "BLOCKED".to_string(),
        selected_action_execution: "BLOCKED".to_string(),
        ordinary_consequence: "BLOCKED".to_string(),
        ds1_acquisition: "BLOCKED".to_string(),
        transfer: "BLOCKED".to_string(),
        invalidation_reopening: "BLOCKED".to_string(),
        functional_recovery: "BLOCKED".to_string(),
        first_collapse: "NOT RUN: definitive execution forbidden".to_string(),
        frozen_ds1_sha256: FROZEN_DS1_SHA256.to_string(),
        ds_e0_source_sha256: FROZEN_DS_E0_SOURCE_SHA256.to_string(),
        seeds: Vec::new(),
        audit_passed: false,
    }
}

pub fn run(mode: HarnessMode) -> CompositionReport {
    if mode == HarnessMode::Definitive {
        return definitive_rejection();
    }
    let source_ok = source_boundary_audit();
    let e0_report: GateReport = ds_e0::run(mode);
    let seeds = e0_report.seeds.iter().map(audit_seed).collect::<Vec<_>>();
    let e0_a_ready = e0_report.passed && seeds.iter().all(|seed| seed.e0_a_ready);
    let e0_b_ready = e0_a_ready && seeds.iter().all(|seed| seed.e0_b_ready);
    let ds1_consumed = e0_b_ready && seeds.iter().all(|seed| seed.ds1_neighborhood_consumed);
    let audit_passed = source_ok
        && ds1_consumed
        && seeds
            .iter()
            .all(|seed| !seed.actual_anonymous_actions_available);
    CompositionReport {
        label: "CUMULATIVE DS1 DEVELOPMENT".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: e0_report.mode,
        claim_eligible: false,
        m0_authoritative: true,
        enabling_parent: ENABLING_PARENT.to_string(),
        m1_exists: false,
        exact_lineage_and_fingerprints: if source_ok { "READY" } else { "COLLAPSE" }
            .to_string(),
        ds_e0_event_formation: if e0_a_ready { "READY" } else { "BLOCKED" }.to_string(),
        e0_b_serialization: if e0_b_ready { "READY" } else { "BLOCKED" }.to_string(),
        frozen_ds1_consumption: if ds1_consumed { "READY" } else { "BLOCKED" }.to_string(),
        anonymous_action_alternatives:
            "COLLAPSE: no actual anonymous boundary-ordering/action alternatives in current substrate"
                .to_string(),
        selected_action_execution: "BLOCKED".to_string(),
        ordinary_consequence: "BLOCKED".to_string(),
        ds1_acquisition: "BLOCKED".to_string(),
        transfer: "BLOCKED".to_string(),
        invalidation_reopening: "BLOCKED".to_string(),
        functional_recovery: "BLOCKED".to_string(),
        first_collapse: COLLAPSE_STAGE.to_string(),
        frozen_ds1_sha256: FROZEN_DS1_SHA256.to_string(),
        ds_e0_source_sha256: FROZEN_DS_E0_SOURCE_SHA256.to_string(),
        seeds,
        audit_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_freezes_at_first_missing_actual_action_alternatives() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.first_collapse, COLLAPSE_STAGE);
        assert!(report.seeds.iter().all(|seed| {
            seed.e0_a_ready
                && seed.e0_b_ready
                && seed.ds1_neighborhood_consumed
                && !seed.actual_anonymous_actions_available
                && seed.selected_action_physically_executed.is_none()
        }));
    }

    #[test]
    fn later_causal_stages_and_costs_are_blocked_and_blank() {
        let report = run(HarnessMode::Micro);
        assert_eq!(report.selected_action_execution, "BLOCKED");
        assert_eq!(report.ordinary_consequence, "BLOCKED");
        assert_eq!(report.ds1_acquisition, "BLOCKED");
        assert!(report.seeds.iter().all(|seed| {
            seed.ledger.ds1_route_firings.is_none()
                && seed.ledger.selected_action_execution_work.is_none()
                && seed.ledger.ordinary_consequence_work.is_none()
                && seed.ledger.ds1_persistent_bytes.is_none()
        }));
    }

    #[test]
    fn definitive_mode_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.audit_passed);
        assert!(!report.claim_eligible);
        assert!(report.seeds.is_empty());
    }
}
