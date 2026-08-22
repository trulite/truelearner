//! Development-only unchanged cumulative DS2 retry after frozen AP0.

use crate::research_runtime::HarnessMode;
use crate::{ap0, prior_retry};

pub const PROTOCOL: &str = "ds2-after-ap0-mechanistic-retry-v1";
pub const EXACT_PARENT: &str = "830d80c3c925a3acf1be8026e9dd8cbe520c763e";
pub const PROTOCOL_COMMIT: &str = "f6d8116da9f331e79b2b28a92a0334318af4b2c3";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_PRIOR_RETRY_SHA256: &str =
    "da05e976dc43ceb5f14fdbb56928207d0fdc99fb52a5d8d630ced588c26d4224";
pub const FROZEN_AP0_SHA256: &str =
    "a33019958327b145bdb14f4386f628e2c4fd5fcca94e413736513f8b86cf78f5";
pub const FROZEN_AP0_READINESS_SHA256: &str =
    "718e6b65c037d76fb3cbd75fdb35cf8e90c2ff3bc8a36f1fae346bcd86075cf5";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "d703f12b378a82122d9807610edbc6b140dbd1dc0a8c1dc501b32b44b8083911";

const STAGES: [&str; 9] = [
    "0. exact authoritative M1, frozen prior retry, and AP0 fingerprints",
    "1. learner-visible causal/source/target annotation remains absent",
    "2. exact M1 interaction and learned-boundary-role path remains intact",
    "3. frozen DS1 selection physically actuates the existing A1 route",
    "4. returned aftermath is naturally contingent on physical route state",
    "5. ordinary aftermath activates existing A1 proposal/probation physics",
    "6. contrasting interactions differentiate supported and reverse/noncausal candidates",
    "7. one reusable role-relative direction consolidates and transfers",
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
    pub ap0_hash: bool,
    pub ap0_readiness_hash: bool,
    pub protocol_hash: bool,
    pub prior_retry_unchanged: bool,
    pub ap0_source_passed: bool,
    pub physical_executor_calls: usize,
    pub existing_proposal_calls: usize,
    pub consequence_to_probation_edges: usize,
    pub consolidation_or_pruning_edges: usize,
    pub semantic_or_causal_adapters: usize,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.prior_retry_hash
            && self.ap0_hash
            && self.ap0_readiness_hash
            && self.protocol_hash
            && self.prior_retry_unchanged
            && self.ap0_source_passed
            && self.physical_executor_calls == 1
            && self.existing_proposal_calls == 1
            && self.consequence_to_probation_edges == 0
            && self.consolidation_or_pruning_edges == 0
            && self.semantic_or_causal_adapters == 0
    }
}

fn source_audit(prior: &prior_retry::Report, ap0_report: &ap0::Report) -> SourceAudit {
    let source = include_str!("ds_ap0_aftermath_plasticity_activation.rs");
    let wire = function_body(
        source,
        "pub(super) fn physical_aftermath_to_existing_proposals(",
    )
    .unwrap_or_default();
    let consequence_edges = [
        "apply_consequence(",
        "downstream_contrast",
        "consequence_direction",
        "d3::",
    ];
    let maturation_edges = ["consolidate(", "prune(", "invalidate_direction("];
    let forbidden = [
        ["Causal", "Adapter"].concat(),
        ["Direction", "Candidate"].concat(),
        ["source", "_position"].concat(),
        ["target", "_position"].concat(),
        ["reward", "_value"].concat(),
        ["correct", "ness"].concat(),
    ];
    SourceAudit {
        prior_retry_hash: env!("DS2_AP0_RETRY_PRIOR_SHA256") == FROZEN_PRIOR_RETRY_SHA256,
        ap0_hash: env!("DS2_AP0_RETRY_AP0_SHA256") == FROZEN_AP0_SHA256,
        ap0_readiness_hash: env!("DS2_AP0_RETRY_READINESS_SHA256") == FROZEN_AP0_READINESS_SHA256,
        protocol_hash: env!("DS2_AP0_RETRY_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        prior_retry_unchanged: prior.first_collapse_stage == Some(5)
            && prior.source.existing_proposal_formers == 1
            && prior.source.aftermath_to_proposal_edges == 0,
        ap0_source_passed: ap0_report.source.passed(),
        physical_executor_calls: wire.matches("execute_handle(&substrate").count(),
        existing_proposal_calls: wire.matches("probation.observe(&substrate").count(),
        consequence_to_probation_edges: consequence_edges
            .iter()
            .map(|edge| wire.matches(edge).count())
            .sum(),
        consolidation_or_pruning_edges: maturation_edges
            .iter()
            .map(|edge| wire.matches(edge).count())
            .sum(),
        semantic_or_causal_adapters: forbidden
            .iter()
            .map(|token| wire.matches(token).count())
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
    pub ap0_seed_count: usize,
    pub selected_proposals: usize,
    pub selected_support_updates: u64,
    pub alternate_proposals: usize,
    pub alternate_support_updates: u64,
    pub stages: [String; 9],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub audit_passed: bool,
}

pub fn run() -> Report {
    let prior = prior_retry::run();
    let ap0_report = ap0::run(HarnessMode::Gate);
    let source = source_audit(&prior, &ap0_report);
    let selected_proposals = ap0_report
        .seeds
        .iter()
        .map(|seed| seed.selected_proposals)
        .sum();
    let selected_support_updates = ap0_report
        .seeds
        .iter()
        .map(|seed| seed.selected_support_updates)
        .sum();
    let alternate_proposals = ap0_report
        .seeds
        .iter()
        .map(|seed| seed.alternate_proposals)
        .sum();
    let alternate_support_updates = ap0_report
        .seeds
        .iter()
        .map(|seed| seed.alternate_support_updates)
        .sum();
    let actuation_closes = prior.stages[3] == "READY";
    let route_contingent = prior.stages[4] == "READY";
    let plasticity_reachable = route_contingent
        && ap0_report.passed
        && selected_proposals > 0
        && selected_support_updates > 0
        && alternate_proposals > 0
        && alternate_support_updates > 0;
    let contrast_updates_reachable = plasticity_reachable
        && source.consequence_to_probation_edges > 0
        && selected_support_updates != alternate_support_updates;
    let reusable_direction =
        contrast_updates_reachable && source.consolidation_or_pruning_edges > 0;
    let invalidation_reopens = reusable_direction;
    let ready = [
        source.passed(),
        prior.stages[1] == "READY",
        prior.stages[2] == "READY",
        actuation_closes,
        route_contingent,
        plasticity_reachable,
        contrast_updates_reachable,
        reusable_direction,
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
        ap0_seed_count: ap0_report.seeds.len(),
        selected_proposals,
        selected_support_updates,
        alternate_proposals,
        alternate_support_updates,
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
    fn ap0_advances_unchanged_ds2_to_differential_update_boundary() {
        let report = run();
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.first_collapse_stage, Some(6));
        assert!(report.selected_proposals > 0);
        assert_eq!(report.selected_proposals, report.alternate_proposals);
        assert_eq!(
            report.selected_support_updates,
            report.alternate_support_updates
        );
        assert_eq!(report.source.consequence_to_probation_edges, 0);
    }

    #[test]
    fn consolidation_and_invalidation_remain_blocked() {
        let report = run();
        assert!(report.stages[6].starts_with("COLLAPSE:"));
        assert!(report.stages[7..].iter().all(|status| status == "BLOCKED"));
        assert!(report.m1_authoritative && !report.m2_exists);
    }
}
