//! Development-only byte-identical DS1 composition retry after frozen DS-R0.

use crate::ds_r0_anonymous_post_action_evidence_return as frozen_r0;
use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds1-after-r0-composition-retry-v1";
pub const EXACT_PARENT: &str = "e0db45c2eb6837236aba9b2859b390db14795c7f";
pub const PROTOCOL_COMMIT: &str = "57a6d48e5c53ca485b14b8fd3f77dd943d7a5234";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_R0_SHA256: &str =
    "f17afa482bf345eb680463f7418b6b6c2553cd78eab9b4dbfce74f7ca1483d51";
pub const FROZEN_R0_READINESS_SHA256: &str =
    "0888ddb8187f606ec7fac72369d4e8b397b624226ddd709540c882925dae82e5";
pub const FROZEN_R0_E2B_SHA256: &str =
    "729948af6d49b9428f04091529d087127b78102f063c87d3593a412713c6e7b3";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

const STAGES: [&str; 11] = [
    "0. M0 lineage and frozen correspondence/R0 controls",
    "1. E0 event formation",
    "2. A1 executable-affordance multiplicity",
    "3. two opaque handles visible to frozen DS1",
    "4. frozen DS1 chooses one handle",
    "5. selected physical route execution",
    "6. R0 anonymous temporary return relation",
    "7. exact anonymous evidence surface",
    "8. existing returned-evidence to frozen-DS1 update coupling",
    "9. frozen DS1 boundary-role strength divergence",
    "10. held-out boundary-role reconstruction",
];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PathInventory {
    pub update_definitions: usize,
    pub r0_bridge_definitions: usize,
    pub evidence_to_update_call_edges: usize,
    pub strength_observation_edges: usize,
    pub held_out_reconstruction_edges: usize,
    pub semantic_update_edges: usize,
    pub runtime_evidence_surfaces: u64,
    pub runtime_ds1_updates: u64,
    pub runtime_strength_observations: u64,
    pub runtime_held_out_reconstructions: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub r0_hash: bool,
    pub readiness_hash: bool,
    pub e2b_hash: bool,
    pub r0_lineage: bool,
    pub ds1_hash: bool,
    pub frozen_r0_read_only: bool,
    pub paths: PathInventory,
    pub update_mutation_sensitive: bool,
    pub strength_mutation_sensitive: bool,
    pub held_out_mutation_sensitive: bool,
}

fn production(source: &str) -> &str {
    source.split("#[cfg(test)]").next().unwrap_or(source)
}

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

fn derive_paths(composition: &str, r0: &str, e0: &str) -> PathInventory {
    let composition = production(composition);
    let r0 = production(r0);
    let update_call = [".apply_", "consequence("].concat();
    let strength_call = ["observe_", "boundary_strengths("].concat();
    let held_out_call = ["held_out_", "reconstruction("].concat();
    let semantic_calls = [
        ["semantic_", "credit("].concat(),
        ["correct_", "choice("].concat(),
        ["reward_", "update("].concat(),
        ["accepted_", "output("].concat(),
        ["rejected_", "output("].concat(),
    ];
    PathInventory {
        update_definitions: e0.matches("fn apply_consequence(").count(),
        r0_bridge_definitions: r0.matches("fn bridge(").count(),
        evidence_to_update_call_edges: composition.matches(&update_call).count()
            + r0.matches(&update_call).count(),
        strength_observation_edges: composition.matches(&strength_call).count(),
        held_out_reconstruction_edges: composition.matches(&held_out_call).count(),
        semantic_update_edges: semantic_calls
            .iter()
            .map(|call| composition.matches(call).count() + r0.matches(call).count())
            .sum(),
        ..PathInventory::default()
    }
}

fn source_audit() -> SourceAudit {
    let composition = include_str!("ds1_after_r0_composition_retry.rs");
    let r0 = include_str!("ds_r0_anonymous_post_action_evidence_return.rs");
    let e0 = include_str!("ds_e0_anonymous_event_formation.rs");
    let run_body = function_body(composition, "\npub fn run(").unwrap_or_default();
    let paths = derive_paths(composition, r0, e0);
    let update_mutation = composition.replacen(
        "#[cfg(test)]",
        "fn mutation_update(){learner.apply_consequence(view,choice,true);}\n#[cfg(test)]",
        1,
    );
    let strength_mutation = composition.replacen(
        "#[cfg(test)]",
        "fn mutation_strength(){observe_boundary_strengths();}\n#[cfg(test)]",
        1,
    );
    let held_out_mutation = composition.replacen(
        "#[cfg(test)]",
        "fn mutation_heldout(){held_out_reconstruction();}\n#[cfg(test)]",
        1,
    );
    SourceAudit {
        r0_hash: env!("DS1_R0_R0_SHA256") == FROZEN_R0_SHA256,
        readiness_hash: env!("DS1_R0_READINESS_SHA256") == FROZEN_R0_READINESS_SHA256,
        e2b_hash: env!("DS1_R0_E2B_SHA256") == FROZEN_R0_E2B_SHA256,
        r0_lineage: frozen_r0::AUTHORITATIVE_M0 == AUTHORITATIVE_M0
            && frozen_r0::FROZEN_DS1_SHA256 == FROZEN_DS1_SHA256
            && frozen_r0::FROZEN_RESULTS_DIGEST == FROZEN_RESULTS_DIGEST,
        ds1_hash: frozen_r0::FROZEN_DS1_SHA256 == FROZEN_DS1_SHA256,
        frozen_r0_read_only: run_body.matches("frozen_r0::run(").count() == 1
            && composition
                .lines()
                .all(|line| !line.trim_start().starts_with("mod frozen_r0")),
        update_mutation_sensitive: derive_paths(&update_mutation, r0, e0)
            .evidence_to_update_call_edges
            > paths.evidence_to_update_call_edges,
        strength_mutation_sensitive: derive_paths(&strength_mutation, r0, e0)
            .strength_observation_edges
            > paths.strength_observation_edges,
        held_out_mutation_sensitive: derive_paths(&held_out_mutation, r0, e0)
            .held_out_reconstruction_edges
            > paths.held_out_reconstruction_edges,
        paths,
    }
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.r0_hash
            && self.readiness_hash
            && self.e2b_hash
            && self.r0_lineage
            && self.ds1_hash
            && self.frozen_r0_read_only
            && self.paths.update_definitions == 1
            && self.paths.r0_bridge_definitions == 1
            && self.paths.evidence_to_update_call_edges == 0
            && self.paths.strength_observation_edges == 0
            && self.paths.held_out_reconstruction_edges == 0
            && self.paths.semantic_update_edges == 0
            && self.update_mutation_sensitive
            && self.strength_mutation_sensitive
            && self.held_out_mutation_sensitive
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub r0_controls: usize,
    pub roots: usize,
    pub handles: usize,
    pub choice: usize,
    pub choose_calls: u64,
    pub route_executions: u64,
    pub activity_pulses: usize,
    pub activity_relations: usize,
    pub temporary_relations: usize,
    pub evidence_fields: usize,
    pub paths: PathInventory,
    pub e0_work: u64,
    pub a1_work: u64,
    pub r0_work: u64,
    pub e0_bytes: usize,
    pub a1_bytes: usize,
    pub ds1_bytes: usize,
    pub r0_bytes: usize,
    pub temporary_peak: usize,
    pub stage_ready: [bool; 11],
}

fn audit_seed(seed: &frozen_r0::SeedAudit, source: &SourceAudit) -> SeedAudit {
    let stage_one = seed.actual && seed.exact && seed.fresh_target;
    let stage_two = stage_one
        && seed.candidates >= 2
        && seed.templates >= 2
        && seed.roots == 2
        && seed.structural == 2;
    let stage_three = stage_two && seed.handles == 2;
    let stage_four = stage_three
        && seed.choice < seed.handles
        && seed.choose_calls == 1
        && seed.ds1_updates == 0;
    let stage_five = stage_four
        && seed.effect_known
        && seed.activity_pulses == 3
        && seed.activity_relations == 2
        && seed.spikes == 2
        && seed.arrows == 1
        && seed.mutations == 2;
    let stage_six = stage_five && seed.mature_shapes == 1 && seed.temporary_relations == 1;
    let stage_seven = stage_six && seed.bridge_fields == 4 && seed.controls.bridge_copy;

    let mut paths = source.paths.clone();
    paths.runtime_evidence_surfaces = u64::from(stage_seven);
    paths.runtime_ds1_updates = seed.ds1_updates;
    paths.runtime_strength_observations = 0;
    paths.runtime_held_out_reconstructions = 0;

    let stage_eight =
        stage_seven && paths.evidence_to_update_call_edges > 0 && paths.runtime_ds1_updates > 0;
    let stage_nine = stage_eight
        && paths.strength_observation_edges > 0
        && paths.runtime_strength_observations > 0;
    let stage_ten = stage_nine
        && paths.held_out_reconstruction_edges > 0
        && paths.runtime_held_out_reconstructions > 0;

    SeedAudit {
        seed: seed.seed,
        r0_controls: 22,
        roots: seed.roots,
        handles: seed.handles,
        choice: seed.choice,
        choose_calls: seed.choose_calls,
        route_executions: u64::from(stage_five),
        activity_pulses: seed.activity_pulses,
        activity_relations: seed.activity_relations,
        temporary_relations: seed.temporary_relations,
        evidence_fields: seed.bridge_fields,
        paths,
        e0_work: seed.e0_work,
        a1_work: seed.a1_work,
        r0_work: seed.return_work.organism_work(),
        e0_bytes: seed.e0_bytes,
        a1_bytes: seed.a1_bytes,
        ds1_bytes: seed.ds1_bytes,
        r0_bytes: seed.return_bytes,
        temporary_peak: seed.temporary_peak,
        stage_ready: [
            source.passed() && seed.controls.passed(),
            stage_one,
            stage_two,
            stage_three,
            stage_four,
            stage_five,
            stage_six,
            stage_seven,
            stage_eight,
            stage_nine,
            stage_ten,
        ],
    }
}

fn ordered_freeze(ready: [bool; 11]) -> ([String; 11], Option<usize>) {
    let first = ready.iter().position(|stage| !stage);
    let stages = std::array::from_fn(|stage| match first {
        None => "READY".to_string(),
        Some(collapse) if stage < collapse => "READY".to_string(),
        Some(collapse) if stage == collapse => format!("COLLAPSE: {}", STAGES[stage]),
        Some(_) => "BLOCKED".to_string(),
    });
    (stages, first)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompositionReport {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub m0_authoritative: bool,
    pub enabling_only: bool,
    pub m1_exists: bool,
    pub source: SourceAudit,
    pub stages: [String; 11],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub seeds: Vec<SeedAudit>,
    pub audit_passed: bool,
}

fn rejected() -> CompositionReport {
    CompositionReport {
        label: "UNCHANGED DS1 AFTER R0: definitive forbidden".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "DEFINITIVE-FORBIDDEN".to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        enabling_only: true,
        m1_exists: false,
        source: source_audit(),
        stages: std::array::from_fn(|_| "BLOCKED: definitive rejected".to_string()),
        first_collapse_stage: None,
        first_collapse: "NOT RUN: definitive rejected before harness".to_string(),
        seeds: Vec::new(),
        audit_passed: false,
    }
}

pub fn run(mode: HarnessMode) -> CompositionReport {
    if mode == HarnessMode::Definitive {
        return rejected();
    }
    let source = source_audit();
    let r0 = frozen_r0::run(mode);
    let seeds = r0
        .seeds
        .iter()
        .map(|seed| audit_seed(seed, &source))
        .collect::<Vec<_>>();
    let mut ready = [false; 11];
    for (stage, value) in ready.iter_mut().enumerate() {
        *value = seeds.iter().all(|seed| seed.stage_ready[stage]);
    }
    let (stages, first_collapse_stage) = ordered_freeze(ready);
    let first_collapse = first_collapse_stage
        .map(|stage| STAGES[stage].to_string())
        .unwrap_or_else(|| "NONE: M1 requires separate authorization".to_string());
    let audit_passed = r0.audit_passed
        && first_collapse_stage == Some(8)
        && seeds.iter().all(|seed| {
            seed.stage_ready[..8].iter().all(|stage| *stage)
                && !seed.stage_ready[8]
                && seed.paths.runtime_evidence_surfaces == 1
                && seed.paths.runtime_ds1_updates == 0
                && seed.paths.evidence_to_update_call_edges == 0
                && seed.paths.runtime_strength_observations == 0
                && seed.paths.runtime_held_out_reconstructions == 0
        });
    CompositionReport {
        label: first_collapse_stage.map_or_else(
            || "UNCHANGED DS1 AFTER R0: M1-ELIGIBLE DEVELOPMENT".to_string(),
            |stage| format!("UNCHANGED DS1 AFTER R0 COLLAPSE AT {}", STAGES[stage]),
        ),
        protocol: PROTOCOL.to_string(),
        mode: r0.mode,
        claim_eligible: false,
        m0_authoritative: true,
        enabling_only: true,
        m1_exists: false,
        source,
        stages,
        first_collapse_stage,
        first_collapse,
        seeds,
        audit_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_freezes_first_collapse_at_existing_update_coupling() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.first_collapse_stage, Some(8));
        assert!(report.stages[..8].iter().all(|stage| stage == "READY"));
        assert!(report.stages[8].starts_with("COLLAPSE"));
        assert!(report.stages[9..].iter().all(|stage| stage == "BLOCKED"));
    }

    #[test]
    fn gate_preserves_r0_and_has_no_update_or_downstream_work() {
        let report = run(HarnessMode::Gate);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.seeds.len(), 5);
        assert!(report.seeds.iter().all(|seed| seed.roots == 2
            && seed.handles == 2
            && seed.choose_calls == 1
            && seed.route_executions == 1
            && seed.temporary_relations == 1
            && seed.evidence_fields == 4
            && seed.paths.runtime_evidence_surfaces == 1
            && seed.paths.runtime_ds1_updates == 0
            && seed.paths.runtime_strength_observations == 0
            && seed.paths.runtime_held_out_reconstructions == 0));
    }

    #[test]
    fn all_zero_paths_are_source_mutation_sensitive() {
        let audit = source_audit();
        assert!(audit.passed(), "{audit:#?}");
        assert!(audit.update_mutation_sensitive);
        assert!(audit.strength_mutation_sensitive);
        assert!(audit.held_out_mutation_sensitive);
    }

    #[test]
    fn every_later_stage_is_blocked_after_first_collapse() {
        for collapse in 0..11 {
            let mut ready = [true; 11];
            ready[collapse] = false;
            let (stages, first) = ordered_freeze(ready);
            assert_eq!(first, Some(collapse));
            assert!(stages[..collapse].iter().all(|stage| stage == "READY"));
            assert!(stages[collapse].starts_with("COLLAPSE"));
            assert!(stages[collapse + 1..]
                .iter()
                .all(|stage| stage == "BLOCKED"));
        }
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.audit_passed);
        assert!(report.seeds.is_empty());
        assert!(!report.m1_exists);
    }
}
