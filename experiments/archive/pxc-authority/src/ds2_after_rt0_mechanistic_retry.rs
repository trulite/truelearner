//! Development-only unchanged cumulative DS2 retry after frozen RT0.

use crate::research_runtime::HarnessMode;
use crate::{post_cp0_retry, rt0};

pub const PROTOCOL: &str = "ds2-after-rt0-mechanistic-retry-v1";
pub const EXACT_PARENT: &str = "65f0347ca1d4c3c03ffc4889f4459ab00b508c10";
pub const PROTOCOL_COMMIT: &str = "b9cff3bc65bb7913303541b617eb51e0103361f7";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_PRIOR_RETRY_SHA256: &str =
    "daa1769eb4aff27eb34e25bac13289f3675fa0248dfb5d885f2b2b64bbb70eb0";
pub const FROZEN_RT0_SHA256: &str =
    "16ef4e2a691e22251d109860ac055c5a1ee78f586ad9335a375589336ad78ed0";
pub const FROZEN_RT0_READINESS_SHA256: &str =
    "52d58517c03546c181ec76b38bd4587127c7e9c903471510509dfac4108ce788";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "939400da86ce75622bc2cff0870de98a93eddb8d0d4d4aeb95b0971d3e64e666";

const STAGES: [&str; 9] = [
    "0. exact authoritative M1, frozen prior retry, and RT0 fingerprints",
    "1. learner-visible causal/source/target annotation remains absent",
    "2. exact M1 interaction and learned-boundary-role path remains intact",
    "3. frozen DS1 selection physically actuates the existing A1 route",
    "4. returned aftermath is naturally contingent on physical route state",
    "5. ordinary aftermath activates existing A1 proposal/probation physics",
    "6. downstream contrast differentiates support between probationary candidates",
    "7. surviving direction becomes one retained executable structure and transfers",
    "8. dependency change invalidates stale direction and reopens ordinary inference",
];

fn function_body<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    let start = source.find(marker)?;
    let tail = &source[start..];
    let open = tail.find('{')?;
    let mut depth = 0usize;
    for (offset, byte) in tail[open..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&tail[..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub prior_retry_hash: bool,
    pub rt0_hash: bool,
    pub rt0_readiness_hash: bool,
    pub protocol_hash: bool,
    pub prior_retry_unchanged: bool,
    pub rt0_source_passed: bool,
    pub post_freeze_observe_sites: usize,
    pub retained_install_sites: usize,
    pub retained_execute_sites: usize,
    pub contradiction_observer_edges: usize,
    pub invalidation_edges: usize,
    pub generic_reopening_edges: usize,
    pub semantic_adapters: usize,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.prior_retry_hash
            && self.rt0_hash
            && self.rt0_readiness_hash
            && self.protocol_hash
            && self.prior_retry_unchanged
            && self.rt0_source_passed
            && self.post_freeze_observe_sites == 0
            && self.retained_install_sites == 1
            && self.retained_execute_sites == 1
            && self.contradiction_observer_edges == 0
            && self.invalidation_edges == 0
            && self.generic_reopening_edges == 0
            && self.semantic_adapters == 0
    }
}

fn source_audit(prior: &post_cp0_retry::Report, rt0_report: &rt0::Report) -> SourceAudit {
    let source = include_str!("ds_rt0_retained_direction_execution.rs");
    let transfer =
        function_body(source, "pub(super) fn rt0_retain_install_execute(").unwrap_or_default();
    let post_freeze = transfer
        .split("FREEZE_PROBATION_ASSET")
        .nth(1)
        .unwrap_or_default();
    let forbidden = [
        ["correct", "ness"].concat(),
        ["reward", "_value"].concat(),
        ["causal", "_label"].concat(),
        ["evaluator", "_direction"].concat(),
    ];
    SourceAudit {
        prior_retry_hash: env!("DS2_RT0_RETRY_PRIOR_SHA256") == FROZEN_PRIOR_RETRY_SHA256,
        rt0_hash: env!("DS2_RT0_RETRY_RT0_SHA256") == FROZEN_RT0_SHA256,
        rt0_readiness_hash: env!("DS2_RT0_RETRY_READINESS_SHA256") == FROZEN_RT0_READINESS_SHA256,
        protocol_hash: env!("DS2_RT0_RETRY_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        prior_retry_unchanged: prior.first_collapse_stage == Some(7)
            && prior.source.post_contrast_install_edges == 0,
        rt0_source_passed: rt0_report.source.passed(),
        post_freeze_observe_sites: post_freeze.matches("probation.observe(").count(),
        retained_install_sites: post_freeze.matches("probation.install(").count(),
        retained_execute_sites: post_freeze.matches("execute_root(&fresh_substrate").count(),
        contradiction_observer_edges: post_freeze.matches("observe_contradiction(").count(),
        invalidation_edges: post_freeze.matches("invalidate(").count()
            + post_freeze.matches(".live = false").count(),
        generic_reopening_edges: post_freeze.matches("generic_reopening").count()
            + post_freeze.matches("reopen_inference(").count(),
        semantic_adapters: forbidden
            .iter()
            .map(|token| post_freeze.matches(token).count())
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
    pub m1_authoritative: bool,
    pub m2_exists: bool,
    pub source: SourceAudit,
    pub rt0_seed_count: usize,
    pub retained_contexts: usize,
    pub reversed_contexts: usize,
    pub stages: [String; 9],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub audit_passed: bool,
}

pub fn run() -> Report {
    let prior = post_cp0_retry::run();
    let rt0_report = rt0::run(HarnessMode::Gate);
    let source = source_audit(&prior, &rt0_report);
    let retained_contexts = rt0_report
        .seeds
        .iter()
        .map(|seed| seed.retained_executions)
        .sum();
    let reversed_contexts = rt0_report
        .seeds
        .iter()
        .map(|seed| seed.reversed_executions)
        .sum();
    let retained_transfer = prior.stages[6] == "READY"
        && rt0_report.passed
        && retained_contexts > 0
        && reversed_contexts == retained_contexts;
    let invalidation_reopens = retained_transfer
        && source.contradiction_observer_edges > 0
        && source.invalidation_edges > 0
        && source.generic_reopening_edges > 0;
    let ready = [
        source.passed(),
        prior.stages[1] == "READY",
        prior.stages[2] == "READY",
        prior.stages[3] == "READY",
        prior.stages[4] == "READY",
        prior.stages[5] == "READY",
        prior.stages[6] == "READY",
        retained_transfer,
        invalidation_reopens,
    ];
    let (stages, first_collapse_stage) = ordered_freeze(ready);
    let first_collapse = first_collapse_stage
        .map(|stage| STAGES[stage].to_string())
        .unwrap_or_else(|| "NONE".to_string());
    Report {
        protocol: PROTOCOL.to_string(),
        claim_eligible: false,
        m1_authoritative: true,
        m2_exists: false,
        source,
        rt0_seed_count: rt0_report.seeds.len(),
        retained_contexts,
        reversed_contexts,
        stages,
        first_collapse_stage,
        first_collapse,
        audit_passed: ready
            .iter()
            .take(first_collapse_stage.unwrap_or(ready.len()))
            .all(|stage| *stage),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rt0_advances_unchanged_ds2_to_invalidation_boundary() {
        let report = run();
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.first_collapse_stage, Some(8));
        assert_eq!(report.retained_contexts, 20);
        assert_eq!(report.reversed_contexts, 20);
        assert_eq!(report.source.invalidation_edges, 0);
        assert_eq!(report.source.generic_reopening_edges, 0);
    }
}
