//! Development-only cumulative DS2 probe over authoritative M1.

use crate::m1_definitive;

pub const PROTOCOL: &str = "ds2-cumulative-m1-mechanistic-probe-v1";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const PROTOCOL_COMMIT: &str = "3625564994bcafb0fe60f8f73366e573ef90444f";
pub const FROZEN_M1_CORE_SHA256: &str =
    "2d220a77a7992771c84cf455d66138ba5d3ffdaa90b2f8bdb452a8630c38e66e";
pub const FROZEN_M1_PARENT_SHA256: &str =
    "2b35d8b181b1b477390a2f84a4ad01993d7ca2b2aec6291d16ffd4fc0faf50b0";
pub const FROZEN_M1_CSV_SHA256: &str =
    "fede145a50bc059ffcd19a26dc65763843a83b1644c89bd44a3b27e8cd7cea27";
pub const FROZEN_M1_MD_SHA256: &str =
    "5971c78eb2688e9fb2d31e59b8d835000ece4923b84aef57d6a1ef48f9295bea";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "b11ca0f4996a74b4d04ec31f63391d40ed78d8888303eccfa48eb01adc89ab0b";

const STAGES: [&str; 9] = [
    "0. exact authoritative M1 lineage and result artifacts",
    "1. no learner-visible causal/source/target annotation in M1 acquisition",
    "2. exact M1 interaction and learned-boundary-role path is present",
    "3. selected DS1 affordance physically executes before downstream evidence",
    "4. returned downstream activity is naturally contingent on the selected route",
    "5. ordinary M1 activity exposes both ordered direction candidates",
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

fn call_expression<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    let start = source.find(marker)?;
    let tail = &source[start..];
    let end = tail.find(")?;")? + 3;
    Some(&tail[..end])
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub m1_core_hash: bool,
    pub m1_parent_hash: bool,
    pub m1_csv_hash: bool,
    pub m1_md_hash: bool,
    pub protocol_hash: bool,
    pub m1_runtime_audit: bool,
    pub causal_metadata_edges: usize,
    pub choice_calls: usize,
    pub d3_calls: usize,
    pub ds1_update_edges: usize,
    pub a1_route_executors: usize,
    pub a1_proposal_formers: usize,
    pub selected_route_execution_edges: usize,
    pub choice_to_d3_edges: usize,
}

impl SourceAudit {
    pub fn lineage_passed(&self) -> bool {
        self.m1_core_hash
            && self.m1_parent_hash
            && self.m1_csv_hash
            && self.m1_md_hash
            && self.protocol_hash
            && self.m1_runtime_audit
    }

    pub fn annotation_absent(&self) -> bool {
        self.causal_metadata_edges == 0
    }

    pub fn interaction_path_present(&self) -> bool {
        self.choice_calls == 1
            && self.d3_calls == 1
            && self.ds1_update_edges == 1
            && self.a1_route_executors >= 2
            && self.a1_proposal_formers >= 1
    }
}

fn source_audit(runtime_ok: bool) -> SourceAudit {
    let parent = include_str!("ds1_after_d3_cumulative_composition_retry.rs");
    let a1 = include_str!("ds_a1_affordance_multiplicity.rs");
    let a1_production = a1
        .split(&["#[cfg(", "test)]"].concat())
        .next()
        .unwrap_or(a1);
    let acquisition = function_body(parent, "pub(super) fn composition_probe(")
        .and_then(|body| body.split("let divergent_patterns").next())
        .unwrap_or_default();
    let d3_call = call_expression(acquisition, "super::frozen_d3::composition_direction(")
        .unwrap_or_default();
    let causal_metadata_edges = [
        "causal_annotation",
        "source_role",
        "target_role",
        "source_position",
        "target_position",
        "evaluator_role(",
        "correct_choice",
        "semantic_polarity",
    ]
    .iter()
    .map(|token| acquisition.matches(token).count())
    .sum();
    SourceAudit {
        m1_core_hash: env!("DS2_M1_DEFINITIVE_CORE_SHA256") == FROZEN_M1_CORE_SHA256,
        m1_parent_hash: env!("DS2_M1_PARENT_SHA256") == FROZEN_M1_PARENT_SHA256,
        m1_csv_hash: env!("DS2_M1_RESULT_CSV_SHA256") == FROZEN_M1_CSV_SHA256,
        m1_md_hash: env!("DS2_M1_RESULT_MD_SHA256") == FROZEN_M1_MD_SHA256,
        protocol_hash: env!("DS2_M1_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        m1_runtime_audit: runtime_ok,
        causal_metadata_edges,
        choice_calls: acquisition.matches("learner.choose(").count(),
        d3_calls: acquisition
            .matches("super::frozen_d3::composition_direction(")
            .count(),
        ds1_update_edges: acquisition.matches("learner.apply_consequence(").count(),
        a1_route_executors: a1_production.matches("fn execute_root(").count()
            + a1_production.matches("fn execute_handle(").count(),
        a1_proposal_formers: a1_production.matches("fn local_proposals(").count(),
        selected_route_execution_edges: acquisition.matches("execute_root(").count()
            + acquisition.matches("execute_handle(").count(),
        choice_to_d3_edges: usize::from(d3_call.contains("choice")),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub protocol: String,
    pub claim_eligible: bool,
    pub m1_authoritative: bool,
    pub m2_exists: bool,
    pub source: SourceAudit,
    pub stages: [String; 9],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub passed_through_interaction_inventory: bool,
    pub audit_passed: bool,
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

pub fn run() -> Report {
    let m1 = m1_definitive::run_audit();
    let m1_runtime_ok = m1.passed
        && !m1.claim_eligible
        && m1.cells.len() == 1
        && m1.cells[0].events == 16
        && m1.cells[0].two_roots == 16
        && m1.cells[0].d3_directions == 16
        && m1.cells[0].updates == 16
        && m1.cells[0].consequence_mature == 4
        && m1.cells[0].held_out_successes == 8;
    let source = source_audit(m1_runtime_ok);
    let selected_route_executes = source.selected_route_execution_edges > 0;
    let route_contingent_evidence = selected_route_executes && source.choice_to_d3_edges > 0;
    let ordered_candidates_available = route_contingent_evidence && source.a1_proposal_formers > 0;
    let contrasting_updates_reachable = ordered_candidates_available && source.ds1_update_edges > 0;
    let reusable_direction_present = contrasting_updates_reachable
        && m1.cells[0].consequence_mature == 4
        && m1.cells[0].held_out_successes == 8;
    let invalidation_reopens = reusable_direction_present
        && include_str!("ds1_after_d3_cumulative_composition_retry.rs")
            .matches("generic_reopening")
            .count()
            > 0;
    let ready = [
        source.lineage_passed(),
        source.annotation_absent(),
        source.interaction_path_present(),
        selected_route_executes,
        route_contingent_evidence,
        ordered_candidates_available,
        contrasting_updates_reachable,
        reusable_direction_present,
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
        passed_through_interaction_inventory: ready[..3].iter().all(|stage| *stage),
        audit_passed: source.lineage_passed()
            && ready
                .iter()
                .take(first_collapse_stage.unwrap_or(ready.len()))
                .all(|stage| *stage),
        source,
        stages,
        first_collapse_stage,
        first_collapse,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_m1_reaches_the_selected_execution_boundary() {
        let report = run();
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.first_collapse_stage, Some(3));
        assert_eq!(report.source.selected_route_execution_edges, 0);
        assert_eq!(report.source.choice_to_d3_edges, 0);
        assert!(report.m1_authoritative && !report.m2_exists);
    }

    #[test]
    fn later_mechanistic_stages_are_blocked() {
        let report = run();
        assert!(report.stages[3].starts_with("COLLAPSE:"));
        assert!(report.stages[4..].iter().all(|status| status == "BLOCKED"));
    }
}
