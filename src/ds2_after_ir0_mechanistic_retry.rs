//! Development-only unchanged cumulative DS2 retry after frozen IR0.

use crate::research_runtime::HarnessMode;
use crate::{ir0, post_rt0_retry};

pub const PROTOCOL: &str = "ds2-after-ir0-mechanistic-retry-v1";
pub const EXACT_PARENT: &str = "2e2c5cb718f2e4c92a68e3cfd8cb5dca0b047dce";
pub const PROTOCOL_COMMIT: &str = "92bb0c2cdca1a4e00361ed8134d231f985bf63bf";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_PRIOR_RETRY_SHA256: &str =
    "50490f57a77426bb8b8460d8279988a4bcfb9a3f3d450b19e7e89796a4f6f10b";
pub const FROZEN_IR0_SHA256: &str =
    "f81cc694f2d6d9e43cb04e8d1a1db301687e6644899665ae470abed1f9e4a7dc";
pub const FROZEN_IR0_READINESS_SHA256: &str =
    "b3b7557d70462fad01a775b4e8c790ac02a0d231ec3e757f975ae02160af65b6";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "34bde1c9d351446721b75531489b03991285a9d977549776184cf2cab2e35b7f";

const STAGES: [&str; 9] = [
    "0. exact authoritative M1 and frozen mechanism fingerprints",
    "1. learner-visible causal/source/target annotation remains absent",
    "2. exact M1 interaction and learned-boundary-role path remains intact",
    "3. frozen DS1 selection physically actuates the existing A1 route",
    "4. returned aftermath is naturally contingent on physical route state",
    "5. ordinary aftermath activates existing A1 proposal/probation physics",
    "6. downstream contrast differentiates support between probationary candidates",
    "7. surviving direction becomes one retained executable structure and transfers",
    "8. dependency change invalidates stale direction and reopens ordinary inference",
];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub prior_retry_hash: bool,
    pub ir0_hash: bool,
    pub ir0_readiness_hash: bool,
    pub protocol_hash: bool,
    pub prior_retry_unchanged: bool,
    pub ir0_source_passed: bool,
    pub semantic_adapters: usize,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.prior_retry_hash
            && self.ir0_hash
            && self.ir0_readiness_hash
            && self.protocol_hash
            && self.prior_retry_unchanged
            && self.ir0_source_passed
            && self.semantic_adapters == 0
    }
}

fn source_audit(prior: &post_rt0_retry::Report, ir0_report: &ir0::Report) -> SourceAudit {
    let source = include_str!("ds2_after_ir0_mechanistic_retry.rs");
    let forbidden = [
        ["correct", "ness"].concat(),
        ["reward", "_value"].concat(),
        ["causal", "_label"].concat(),
        ["evaluator", "_direction"].concat(),
    ];
    SourceAudit {
        prior_retry_hash: env!("DS2_IR0_RETRY_PRIOR_SHA256") == FROZEN_PRIOR_RETRY_SHA256,
        ir0_hash: env!("DS2_IR0_RETRY_IR0_SHA256") == FROZEN_IR0_SHA256,
        ir0_readiness_hash: env!("DS2_IR0_RETRY_READINESS_SHA256") == FROZEN_IR0_READINESS_SHA256,
        protocol_hash: env!("DS2_IR0_RETRY_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        prior_retry_unchanged: prior.first_collapse_stage == Some(8)
            && prior.source.invalidation_edges == 0
            && prior.source.generic_reopening_edges == 0,
        ir0_source_passed: ir0_report.source.passed(),
        semantic_adapters: forbidden
            .iter()
            .map(|token| source.matches(token).count())
            .sum(),
    }
}

fn ordered_freeze(ready: [bool; 9]) -> ([String; 9], Option<usize>) {
    let first = ready.iter().position(|stage| !stage);
    (
        std::array::from_fn(|stage| match first {
            None => "READY".to_string(),
            Some(collapse) if stage < collapse => "READY".to_string(),
            Some(collapse) if stage == collapse => format!("COLLAPSE: {}", STAGES[stage]),
            Some(_) => "BLOCKED".to_string(),
        }),
        first,
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub protocol: String,
    pub claim_eligible: bool,
    pub development_complete: bool,
    pub m1_authoritative: bool,
    pub m2_exists: bool,
    pub source: SourceAudit,
    pub ir0_seed_count: usize,
    pub changed_lifecycles: usize,
    pub invalidations: usize,
    pub reopenings: usize,
    pub reopened_executions: usize,
    pub historical_returns: usize,
    pub compatible_preservations: usize,
    pub ambiguous_preservations: usize,
    pub stages: [String; 9],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub audit_passed: bool,
}

pub fn run() -> Report {
    let prior = post_rt0_retry::run();
    let ir0_report = ir0::run(HarnessMode::Gate);
    let source = source_audit(&prior, &ir0_report);
    let sum = |field: fn(&ir0::SeedAudit) -> usize| ir0_report.seeds.iter().map(field).sum();
    let changed_lifecycles = sum(|seed| seed.changed_lifecycles);
    let invalidations = sum(|seed| seed.invalidations);
    let reopenings = sum(|seed| seed.reopenings);
    let reopened_executions = sum(|seed| seed.reopened_executions);
    let historical_returns = sum(|seed| seed.historical_returns);
    let compatible_preservations = sum(|seed| seed.compatible_preservations);
    let ambiguous_preservations = sum(|seed| seed.ambiguous_preservations);
    let expected = ir0_report.seeds.len() * 4;
    let invalidation_reopens = ir0_report.passed
        && expected > 0
        && changed_lifecycles == expected
        && invalidations == expected
        && reopenings == expected
        && reopened_executions == expected
        && historical_returns == expected
        && compatible_preservations == expected
        && ambiguous_preservations == expected;
    let ready = [
        source.passed(),
        prior.stages[1] == "READY",
        prior.stages[2] == "READY",
        prior.stages[3] == "READY",
        prior.stages[4] == "READY",
        prior.stages[5] == "READY",
        prior.stages[6] == "READY",
        prior.stages[7] == "READY",
        invalidation_reopens,
    ];
    let (stages, first_collapse_stage) = ordered_freeze(ready);
    let first_collapse = first_collapse_stage
        .map(|stage| STAGES[stage].to_string())
        .unwrap_or_else(|| "NONE".to_string());
    let audit_passed = ready.iter().all(|stage| *stage);
    Report {
        protocol: PROTOCOL.to_string(),
        claim_eligible: false,
        development_complete: audit_passed,
        m1_authoritative: true,
        m2_exists: false,
        source,
        ir0_seed_count: ir0_report.seeds.len(),
        changed_lifecycles,
        invalidations,
        reopenings,
        reopened_executions,
        historical_returns,
        compatible_preservations,
        ambiguous_preservations,
        stages,
        first_collapse_stage,
        first_collapse,
        audit_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ir0_completes_all_unchanged_ds2_development_stages() {
        let report = run();
        assert!(report.audit_passed, "{report:#?}");
        assert!(report.development_complete);
        assert_eq!(report.first_collapse_stage, None);
        assert_eq!(report.changed_lifecycles, 20);
        assert_eq!(report.invalidations, 20);
        assert_eq!(report.reopenings, 20);
        assert_eq!(report.reopened_executions, 20);
        assert_eq!(report.historical_returns, 20);
        assert_eq!(report.compatible_preservations, 20);
        assert_eq!(report.ambiguous_preservations, 20);
        assert!(report.stages.iter().all(|stage| stage == "READY"));
        assert!(report.m1_authoritative);
        assert!(!report.m2_exists);
        assert!(!report.claim_eligible);
    }
}
