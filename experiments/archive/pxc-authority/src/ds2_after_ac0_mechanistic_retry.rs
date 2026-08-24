//! Development-only unchanged cumulative DS2 retry after frozen AC0.

use crate::research_runtime::HarnessMode;
use crate::{ac0, prior_probe};

pub const PROTOCOL: &str = "ds2-after-ac0-mechanistic-retry-v1";
pub const EXACT_PARENT: &str = "80cf99f9fd4450b3d3b0ffbe612c9d8976e703b9";
pub const PROTOCOL_COMMIT: &str = "25271445dc658637960eea64649652126105983d";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_PRIOR_PROBE_SHA256: &str =
    "37542fdcefc8e1d8cf8012181be78aca366f1ecb320add62fd209d0f2c812683";
pub const FROZEN_AC0_SHA256: &str =
    "860e89304e86f254dd02a5aa35cf63cc240af160039b4166fa0cb5856dacb84a";
pub const FROZEN_AC0_READINESS_SHA256: &str =
    "85efbb950dc323e6e3d4cb9f484cbd01bcd74ad5a3a810036f9c00616266d072";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "744b6e330931856dca1e40baa68e566b706d61df0a9442fc4e1c4717f879649c";

const STAGES: [&str; 9] = [
    "0. exact authoritative M1, frozen DS2 probe, and AC0 fingerprints",
    "1. learner-visible causal/source/target annotation remains absent",
    "2. exact M1 interaction and learned-boundary-role path remains intact",
    "3. frozen DS1 selection physically actuates the existing A1 route",
    "4. returned aftermath is naturally contingent on physical route state",
    "5. ordinary M1+AC0 aftermath reaches existing ordered-candidate proposal physics",
    "6. contrasting interactions update supported and reverse/noncausal candidates",
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
    pub prior_probe_hash: bool,
    pub ac0_hash: bool,
    pub ac0_readiness_hash: bool,
    pub protocol_hash: bool,
    pub prior_probe_unchanged: bool,
    pub ac0_source_passed: bool,
    pub existing_proposal_formers: usize,
    pub aftermath_to_proposal_edges: usize,
    pub causal_adapter_edges: usize,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.prior_probe_hash
            && self.ac0_hash
            && self.ac0_readiness_hash
            && self.protocol_hash
            && self.prior_probe_unchanged
            && self.ac0_source_passed
            && self.causal_adapter_edges == 0
    }
}

fn source_audit(prior: &prior_probe::Report, ac0_report: &ac0::Report) -> SourceAudit {
    let ac0_source = include_str!("ds_ac0_selected_affordance_actuation_closure.rs");
    let acquisition = function_body(ac0_source, "fn audit_seed(").unwrap_or_default();
    let aftermath_to_proposal_edges = [
        "local_proposals(&baseline_aftermath",
        "observe(&baseline_aftermath",
        "form(&baseline_aftermath",
        "propose(&baseline_aftermath",
    ]
    .iter()
    .map(|edge| acquisition.matches(edge).count())
    .sum();
    let forbidden_adapters = [
        ["Causal", "Adapter"].concat(),
        ["Intervention", "Record"].concat(),
        ["Direction", "Candidate"].concat(),
        ["source", "_position"].concat(),
        ["target", "_position"].concat(),
    ];
    SourceAudit {
        prior_probe_hash: env!("DS2_RETRY_PRIOR_PROBE_SHA256") == FROZEN_PRIOR_PROBE_SHA256,
        ac0_hash: env!("DS2_RETRY_AC0_SHA256") == FROZEN_AC0_SHA256,
        ac0_readiness_hash: env!("DS2_RETRY_AC0_READINESS_SHA256") == FROZEN_AC0_READINESS_SHA256,
        protocol_hash: env!("DS2_RETRY_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        prior_probe_unchanged: prior.first_collapse_stage == Some(3)
            && prior.source.selected_route_execution_edges == 0
            && prior.source.choice_to_d3_edges == 0,
        ac0_source_passed: ac0_report.source.passed(),
        existing_proposal_formers: prior.source.a1_proposal_formers,
        aftermath_to_proposal_edges,
        causal_adapter_edges: forbidden_adapters
            .iter()
            .map(|token| acquisition.matches(token).count())
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
    pub ac0_seed_count: usize,
    pub stages: [String; 9],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub audit_passed: bool,
}

pub fn run() -> Report {
    let prior = prior_probe::run();
    let ac0_report = ac0::run(HarnessMode::Gate);
    let source = source_audit(&prior, &ac0_report);
    let actuation_closes = ac0_report.passed
        && ac0_report.seeds.iter().all(|seed| {
            seed.executions == seed.contexts
                && seed.arrow_traversals >= seed.contexts as u64
                && seed.roots_before_choice == seed.contexts * 2
        });
    let route_contingent = actuation_closes
        && ac0_report.seeds.iter().all(|seed| {
            seed.distinct_aftermaths == seed.contexts
                && seed.blocked_route_abstentions == seed.contexts
                && seed.changed_binding_changes == seed.contexts
                && seed.skipped_execution_abstentions == seed.contexts
                && seed.stale_handle_abstentions == seed.contexts
        });
    let proposals_reachable = route_contingent
        && source.existing_proposal_formers > 0
        && source.aftermath_to_proposal_edges > 0;
    let contrast_updates_reachable = proposals_reachable
        && include_str!("ds1_after_d3_cumulative_composition_retry.rs")
            .matches("learner.apply_consequence(")
            .count()
            > 0;
    let reusable_direction = contrast_updates_reachable && prior.source.a1_route_executors > 0;
    let invalidation_reopens = reusable_direction
        && include_str!("ds1_after_d3_cumulative_composition_retry.rs")
            .matches("generic_reopening")
            .count()
            > 0;
    let ready = [
        source.passed(),
        prior.source.annotation_absent(),
        prior.passed_through_interaction_inventory,
        actuation_closes,
        route_contingent,
        proposals_reachable,
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
        ac0_seed_count: ac0_report.seeds.len(),
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
    fn ac0_advances_unchanged_ds2_to_proposal_boundary() {
        let report = run();
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.first_collapse_stage, Some(5));
        assert_eq!(report.source.existing_proposal_formers, 1);
        assert_eq!(report.source.aftermath_to_proposal_edges, 0);
    }

    #[test]
    fn later_direction_stages_remain_blocked() {
        let report = run();
        assert!(report.stages[5].starts_with("COLLAPSE:"));
        assert!(report.stages[6..].iter().all(|status| status == "BLOCKED"));
        assert!(report.m1_authoritative && !report.m2_exists);
    }
}
