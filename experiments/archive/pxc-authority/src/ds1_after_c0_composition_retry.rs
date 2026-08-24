//! Development-only byte-identical DS1 composition retry after frozen DS-C0.

use crate::ds_c0_anonymous_credit_coupling as frozen_c0;
use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds1-after-c0-composition-retry-v1";
pub const EXACT_PARENT: &str = "5d4a791065fe14e3194ca73d84f141467a7ef903";
pub const PROTOCOL_COMMIT: &str = "1387aa69febd31bed5a3b8163c1d6e30760478e2";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_C0_SHA256: &str =
    "5c8d00189593ca2f7efb47165efddf85111259f90433a016e5822b5b9578aed2";
pub const FROZEN_C0_READINESS_SHA256: &str =
    "a69e440639bc37eefc0e9f30402cde6c3b5dec945d95a77060513d7a96491572";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

const STAGES: [&str; 12] = [
    "0. M0 lineage and frozen C0 controls",
    "1. E0 event formation",
    "2. A1 executable-affordance multiplicity",
    "3. two opaque handles visible to frozen DS1",
    "4. frozen DS1 chooses one handle",
    "5. selected physical route execution",
    "6. R0 anonymous temporary return relation",
    "7. exact anonymous evidence surface",
    "8a. anonymous C0 evidence-to-choice coupling without polarity",
    "8b. frozen DS1 update through an existing path",
    "9. frozen DS1 boundary-role strength divergence",
    "10. held-out boundary-role reconstruction",
];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PathInventory {
    pub update_definitions: usize,
    pub c0_coupling_definitions: usize,
    pub coupling_to_update_call_edges: usize,
    pub strength_observation_edges: usize,
    pub held_out_reconstruction_edges: usize,
    pub semantic_update_edges: usize,
    pub runtime_couplings: u64,
    pub runtime_polarity_fields: u64,
    pub runtime_ds1_updates: u64,
    pub runtime_strength_observations: u64,
    pub runtime_held_out_reconstructions: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub c0_hash: bool,
    pub readiness_hash: bool,
    pub c0_lineage: bool,
    pub ds1_hash: bool,
    pub frozen_c0_read_only: bool,
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

fn derive_paths(composition: &str, c0: &str, e0: &str) -> PathInventory {
    let composition = production(composition);
    let c0 = production(c0);
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
        c0_coupling_definitions: c0.matches("struct CouplingArrow").count(),
        coupling_to_update_call_edges: composition.matches(&update_call).count()
            + c0.matches(&update_call).count(),
        strength_observation_edges: composition.matches(&strength_call).count(),
        held_out_reconstruction_edges: composition.matches(&held_out_call).count(),
        semantic_update_edges: semantic_calls
            .iter()
            .map(|call| composition.matches(call).count() + c0.matches(call).count())
            .sum(),
        ..PathInventory::default()
    }
}

fn source_audit() -> SourceAudit {
    let composition = include_str!("ds1_after_c0_composition_retry.rs");
    let c0 = include_str!("ds_c0_anonymous_credit_coupling.rs");
    let e0 = include_str!("ds_e0_anonymous_event_formation.rs");
    let run_body = function_body(composition, "\npub fn run(").unwrap_or_default();
    let paths = derive_paths(composition, c0, e0);
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
        c0_hash: env!("DS1_C0_C0_SHA256") == FROZEN_C0_SHA256,
        readiness_hash: env!("DS1_C0_READINESS_SHA256") == FROZEN_C0_READINESS_SHA256,
        c0_lineage: frozen_c0::AUTHORITATIVE_M0 == AUTHORITATIVE_M0
            && frozen_c0::FROZEN_DS1_SHA256 == FROZEN_DS1_SHA256
            && frozen_c0::FROZEN_RESULTS_DIGEST == FROZEN_RESULTS_DIGEST,
        ds1_hash: frozen_c0::FROZEN_DS1_SHA256 == FROZEN_DS1_SHA256,
        frozen_c0_read_only: run_body.matches("frozen_c0::run(").count() == 1,
        update_mutation_sensitive: derive_paths(&update_mutation, c0, e0)
            .coupling_to_update_call_edges
            > paths.coupling_to_update_call_edges,
        strength_mutation_sensitive: derive_paths(&strength_mutation, c0, e0)
            .strength_observation_edges
            > paths.strength_observation_edges,
        held_out_mutation_sensitive: derive_paths(&held_out_mutation, c0, e0)
            .held_out_reconstruction_edges
            > paths.held_out_reconstruction_edges,
        paths,
    }
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.c0_hash
            && self.readiness_hash
            && self.c0_lineage
            && self.ds1_hash
            && self.frozen_c0_read_only
            && self.paths.update_definitions == 1
            && self.paths.c0_coupling_definitions == 1
            && self.paths.coupling_to_update_call_edges == 0
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
    pub roots: usize,
    pub handles: usize,
    pub choice: usize,
    pub choose_calls: u64,
    pub route_executions: u64,
    pub evidence_fields: usize,
    pub eligibility_cells: usize,
    pub couplings: usize,
    pub polarity_fields: usize,
    pub paths: PathInventory,
    pub e0_work: u64,
    pub a1_work: u64,
    pub r0_primary_work: u64,
    pub r0_parent_audit_work: u64,
    pub c0_primary_work: u64,
    pub c0_control_work: u64,
    pub c0_persistent_bytes: usize,
    pub temporary_peak: usize,
    pub stage_ready: [bool; 12],
}

fn audit_seed(seed: &frozen_c0::SeedAudit, source: &SourceAudit) -> SeedAudit {
    let stage_one = seed.stage_ready[1] && seed.roots == 2;
    let stage_two = stage_one && seed.roots == 2 && seed.handles == 2;
    let stage_three = stage_two && seed.handles == 2;
    let stage_four = stage_three && seed.choice < seed.handles && seed.choose_calls == 1;
    let stage_five = stage_four && seed.stage_ready[1];
    let stage_six = stage_five && seed.evidence_fields == 4;
    let stage_seven = stage_six && seed.evidence_fields == 4;
    let stage_eight_a = stage_seven
        && seed.eligibility_cells == 1
        && seed.couplings == 1
        && seed.coupling_polarity_fields == 0
        && seed.stage_ready[8];

    let mut paths = source.paths.clone();
    paths.runtime_couplings = seed.couplings as u64;
    paths.runtime_polarity_fields = seed.coupling_polarity_fields as u64;
    paths.runtime_ds1_updates = seed.ds1_updates;
    paths.runtime_strength_observations = 0;
    paths.runtime_held_out_reconstructions = 0;

    let stage_eight_b =
        stage_eight_a && paths.coupling_to_update_call_edges > 0 && paths.runtime_ds1_updates > 0;
    let stage_nine = stage_eight_b
        && paths.strength_observation_edges > 0
        && paths.runtime_strength_observations > 0;
    let stage_ten = stage_nine
        && paths.held_out_reconstruction_edges > 0
        && paths.runtime_held_out_reconstructions > 0;

    SeedAudit {
        seed: seed.seed,
        roots: seed.roots,
        handles: seed.handles,
        choice: seed.choice,
        choose_calls: seed.choose_calls,
        route_executions: u64::from(stage_five),
        evidence_fields: seed.evidence_fields,
        eligibility_cells: seed.eligibility_cells,
        couplings: seed.couplings,
        polarity_fields: seed.coupling_polarity_fields,
        paths,
        e0_work: seed.e0_work,
        a1_work: seed.a1_work,
        r0_primary_work: seed.r0_primary_work,
        r0_parent_audit_work: seed.r0_parent_audit_work,
        c0_primary_work: seed.primary_work.organism_work(),
        c0_control_work: seed.total_c0_work.organism_work(),
        c0_persistent_bytes: seed.c0_persistent_bytes,
        temporary_peak: seed.temporary_peak_bytes,
        stage_ready: [
            source.passed()
                && seed.controls.passed()
                && seed.stage_ready.iter().all(|stage| *stage),
            stage_one,
            stage_two,
            stage_three,
            stage_four,
            stage_five,
            stage_six,
            stage_seven,
            stage_eight_a,
            stage_eight_b,
            stage_nine,
            stage_ten,
        ],
    }
}

fn ordered_freeze(ready: [bool; 12]) -> ([String; 12], Option<usize>) {
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
pub struct CompositionReport {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub m0_authoritative: bool,
    pub enabling_only: bool,
    pub m1_exists: bool,
    pub source: SourceAudit,
    pub stages: [String; 12],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub seeds: Vec<SeedAudit>,
    pub audit_passed: bool,
}

fn rejected() -> CompositionReport {
    CompositionReport {
        label: "UNCHANGED DS1 AFTER C0: definitive forbidden".to_string(),
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
    let c0 = frozen_c0::run(mode);
    let seeds = c0
        .seeds
        .iter()
        .map(|seed| audit_seed(seed, &source))
        .collect::<Vec<_>>();
    let mut ready = [false; 12];
    for (stage, value) in ready.iter_mut().enumerate() {
        *value = seeds.iter().all(|seed| seed.stage_ready[stage]);
    }
    let (stages, first_collapse_stage) = ordered_freeze(ready);
    let first_collapse = first_collapse_stage
        .map(|stage| STAGES[stage].to_string())
        .unwrap_or_else(|| "NONE: M1 requires separate authorization".to_string());
    let audit_passed = c0.audit_passed
        && first_collapse_stage == Some(9)
        && seeds.iter().all(|seed| {
            seed.stage_ready[..9].iter().all(|stage| *stage)
                && !seed.stage_ready[9]
                && seed.couplings == 1
                && seed.polarity_fields == 0
                && seed.paths.runtime_ds1_updates == 0
                && seed.paths.coupling_to_update_call_edges == 0
                && seed.paths.runtime_strength_observations == 0
                && seed.paths.runtime_held_out_reconstructions == 0
        });
    CompositionReport {
        label: first_collapse_stage.map_or_else(
            || "UNCHANGED DS1 AFTER C0: M1-ELIGIBLE DEVELOPMENT".to_string(),
            |stage| format!("UNCHANGED DS1 AFTER C0 COLLAPSE AT {}", STAGES[stage]),
        ),
        protocol: PROTOCOL.to_string(),
        mode: c0.mode,
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
    fn micro_separates_coupling_from_update_and_freezes_at_8b() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.first_collapse_stage, Some(9));
        assert!(report.stages[..9].iter().all(|stage| stage == "READY"));
        assert!(report.stages[9].contains("8b."));
        assert!(report.stages[10..].iter().all(|stage| stage == "BLOCKED"));
    }

    #[test]
    fn gate_preserves_c0_and_has_no_update_or_downstream_observation() {
        let report = run(HarnessMode::Gate);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.seeds.len(), 5);
        assert!(report.seeds.iter().all(|seed| seed.roots == 2
            && seed.handles == 2
            && seed.choose_calls == 1
            && seed.route_executions == 1
            && seed.evidence_fields == 4
            && seed.eligibility_cells == 1
            && seed.couplings == 1
            && seed.polarity_fields == 0
            && seed.paths.runtime_ds1_updates == 0
            && seed.paths.runtime_strength_observations == 0
            && seed.paths.runtime_held_out_reconstructions == 0));
    }

    #[test]
    fn every_absent_path_is_mutation_sensitive() {
        let audit = source_audit();
        assert!(audit.passed(), "{audit:#?}");
        assert!(audit.update_mutation_sensitive);
        assert!(audit.strength_mutation_sensitive);
        assert!(audit.held_out_mutation_sensitive);
    }

    #[test]
    fn ordered_freeze_blocks_every_later_stage() {
        for collapse in 0..12 {
            let mut ready = [true; 12];
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
        assert!(!report.audit_passed && report.seeds.is_empty() && !report.m1_exists);
    }
}
