//! Development-only unchanged cumulative DS2 retry after frozen CP0.

use crate::research_runtime::HarnessMode;
use crate::{cp0, post_ap0_retry};

pub const PROTOCOL: &str = "ds2-after-cp0-mechanistic-retry-v1";
pub const EXACT_PARENT: &str = "774a517dff832dc0a7794ea578f58854e2c68bec";
pub const PROTOCOL_COMMIT: &str = "466faf3e8ec63e559ec156c4c362fd8922b5f074";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_PRIOR_RETRY_SHA256: &str =
    "ce0e253ae43136ce7396bbbc237baf6490fa904067a007504fa26bfdbc87044a";
pub const FROZEN_CP0_SHA256: &str =
    "c9fcc53d03296b169060499e2304de557f3f7a93744dbc1f935053f99d41c498";
pub const FROZEN_CP0_READINESS_SHA256: &str =
    "9cdcba7d1bbb8b8c763ee7c8ffab5f1326ff54222b65494f53ccd863391ea013";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "d985aef7fb4b60221b3845bc2be70db14cd1f28d23ed32c0bf07a213910355dd";

const STAGES: [&str; 9] = [
    "0. exact authoritative M1, frozen prior retry, and CP0 fingerprints",
    "1. learner-visible causal/source/target annotation remains absent",
    "2. exact M1 interaction and learned-boundary-role path remains intact",
    "3. frozen DS1 selection physically actuates the existing A1 route",
    "4. returned aftermath is naturally contingent on physical route state",
    "5. ordinary aftermath activates existing A1 proposal/probation physics",
    "6. downstream contrast differentiates support between probationary candidates",
    "7. surviving direction becomes one retained executable structure and transfers",
    "8. dependency change invalidates and reopens ordinary inference",
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
    pub cp0_hash: bool,
    pub cp0_readiness_hash: bool,
    pub protocol_hash: bool,
    pub prior_retry_unchanged: bool,
    pub cp0_source_passed: bool,
    pub existing_observe_sites: usize,
    pub direct_support_mutations: usize,
    pub post_contrast_install_edges: usize,
    pub retained_asset_transfer_edges: usize,
    pub semantic_or_causal_adapters: usize,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.prior_retry_hash
            && self.cp0_hash
            && self.cp0_readiness_hash
            && self.protocol_hash
            && self.prior_retry_unchanged
            && self.cp0_source_passed
            && self.existing_observe_sites == 2
            && self.direct_support_mutations == 0
            && self.post_contrast_install_edges == 0
            && self.retained_asset_transfer_edges == 0
            && self.semantic_or_causal_adapters == 0
    }
}

fn source_audit(prior: &post_ap0_retry::Report, cp0_report: &cp0::Report) -> SourceAudit {
    let source = include_str!("ds_cp0_consequence_probation_coupling.rs");
    let probation = function_body(
        source,
        "pub(super) fn cp0_apply_contrast_to_existing_probation(",
    )
    .unwrap_or_default();
    let forbidden = [
        ["Causal", "Adapter"].concat(),
        ["Direction", "Candidate"].concat(),
        ["correct", "ness"].concat(),
        ["reward", "_value"].concat(),
        ["source", "_role"].concat(),
        ["target", "_role"].concat(),
    ];
    SourceAudit {
        prior_retry_hash: env!("DS2_CP0_RETRY_PRIOR_SHA256") == FROZEN_PRIOR_RETRY_SHA256,
        cp0_hash: env!("DS2_CP0_RETRY_CP0_SHA256") == FROZEN_CP0_SHA256,
        cp0_readiness_hash: env!("DS2_CP0_RETRY_READINESS_SHA256") == FROZEN_CP0_READINESS_SHA256,
        protocol_hash: env!("DS2_CP0_RETRY_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        prior_retry_unchanged: prior.first_collapse_stage == Some(6)
            && prior.selected_support_updates == prior.alternate_support_updates,
        cp0_source_passed: cp0_report.source.passed(),
        existing_observe_sites: probation.matches("probation.observe(&substrate").count(),
        direct_support_mutations: [".count +=", ".count -=", "templates.insert("]
            .iter()
            .map(|token| probation.matches(token).count())
            .sum(),
        post_contrast_install_edges: probation.matches("probation.install(").count(),
        retained_asset_transfer_edges: probation.matches("retained_asset").count()
            + probation.matches("reuse_without_acquisition").count(),
        semantic_or_causal_adapters: forbidden
            .iter()
            .map(|token| probation.matches(token).count())
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
    pub cp0_seed_count: usize,
    pub differentiated_contexts: usize,
    pub reversed_contexts: usize,
    pub fresh_diagnostic_contexts: usize,
    pub stages: [String; 9],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub audit_passed: bool,
}

pub fn run() -> Report {
    let prior = post_ap0_retry::run();
    let cp0_report = cp0::run(HarnessMode::Gate);
    let source = source_audit(&prior, &cp0_report);
    let differentiated_contexts = cp0_report
        .seeds
        .iter()
        .map(|seed| seed.differentiated)
        .sum();
    let reversed_contexts = cp0_report
        .seeds
        .iter()
        .map(|seed| seed.reverse_differentiated)
        .sum();
    let fresh_diagnostic_contexts = cp0_report
        .seeds
        .iter()
        .map(|seed| seed.fresh_transfers)
        .sum();
    let differential_support = prior.stages[5] == "READY"
        && cp0_report.passed
        && differentiated_contexts > 0
        && reversed_contexts == differentiated_contexts;
    let retained_transfer = differential_support
        && source.post_contrast_install_edges > 0
        && source.retained_asset_transfer_edges > 0;
    let invalidation_reopens = retained_transfer;
    let ready = [
        source.passed(),
        prior.stages[1] == "READY",
        prior.stages[2] == "READY",
        prior.stages[3] == "READY",
        prior.stages[4] == "READY",
        prior.stages[5] == "READY",
        differential_support,
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
        cp0_seed_count: cp0_report.seeds.len(),
        differentiated_contexts,
        reversed_contexts,
        fresh_diagnostic_contexts,
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
    fn cp0_advances_unchanged_ds2_to_retained_execution_boundary() {
        let report = run();
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.first_collapse_stage, Some(7));
        assert_eq!(report.differentiated_contexts, 20);
        assert_eq!(report.reversed_contexts, 20);
        assert_eq!(report.source.post_contrast_install_edges, 0);
        assert_eq!(report.source.retained_asset_transfer_edges, 0);
    }

    #[test]
    fn invalidation_remains_blocked() {
        let report = run();
        assert!(report.stages[7].starts_with("COLLAPSE:"));
        assert_eq!(report.stages[8], "BLOCKED");
        assert!(report.m1_authoritative && !report.m2_exists);
    }
}
