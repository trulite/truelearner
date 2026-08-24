//! Development-only byte-identical DS1 cumulative retry after frozen DS-D2.

use crate::ds_c0_anonymous_credit_coupling as frozen_c0;
use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds1-after-d2-cumulative-composition-retry-v1";
pub const EXACT_PARENT: &str = "9b2c9f27f4e6e9c8d0e4f79f3225106de0344faf";
pub const PROTOCOL_COMMIT: &str = "bd7cdc2f3f6f59b90ae17310799e905baa96e449";
pub const PROTOCOL_AMENDMENT_COMMIT: &str = "c61ceaac2661975b20cbe4bbf2a869f16a820519";
pub const PROTOCOL_AMENDMENT_SHA256: &str =
    "104ef8d4f0da19ca6d819e5aef3e0ca843511580ac2e28a1977f675f8afab991";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_D2_SHA256: &str =
    "ac257b53e28b0bdbcfd4cbcb7ca855086d1de5812a07029f4b2405fda2a6da8f";
pub const FROZEN_D2_HANDOFF_SHA256: &str =
    "03012d13bbffc16760c51ee1b12e9a2afbbf8ef970addcb2db4ba850adba1b03";
pub const FROZEN_C0_SHA256: &str =
    "5c8d00189593ca2f7efb47165efddf85111259f90433a016e5822b5b9578aed2";
pub const FROZEN_C0_READINESS_SHA256: &str =
    "a69e440639bc37eefc0e9f30402cde6c3b5dec945d95a77060513d7a96491572";
pub const FROZEN_E0_SHA256: &str =
    "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

const STAGES: [&str; 12] = [
    "0. exact M0/C0/D2/DS1 lineage and frozen controls",
    "1. E0 anonymous event formation",
    "2. A0/A1 executable-affordance multiplicity",
    "3. frozen DS1 sees two opaque alternatives",
    "4. frozen DS1 chooses one alternative",
    "5. selected route physically executes",
    "6. R0 returned evidence and C0 temporal ownership",
    "7. D2 uniquely compatible temporary directional ARROW",
    "8c. D2 directional relation reaches frozen DS1",
    "9. frozen DS1 update fires",
    "10. boundary-role strengths diverge",
    "11. held-out boundary-role reconstruction",
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct DirectionProbe {
    direction: Option<usize>,
    physical: bool,
    work: u64,
    cleanup: bool,
}

macro_rules! composition_d2_access {
    () => {
        pub(super) fn composition_direction(
            seed: u64,
            acquisition: usize,
            selected: usize,
        ) -> Option<super::DirectionProbe> {
            if selected >= 2 {
                return None;
            }
            let export = frozen_a1::d2_export(seed, acquisition)?;
            let returned = export.predictions[selected].clone();
            let mut workspace = DifferentialWorkspace::default();
            let direction = workspace.form(
                [Some(&export.predictions[0]), Some(&export.predictions[1])],
                &returned,
            );
            let physical = workspace.execute_direction();
            let work = workspace.work.organism_work();
            let cleanup = workspace.cleanup();
            Some(super::DirectionProbe {
                direction,
                physical,
                work,
                cleanup,
            })
        }

        pub(super) fn composition_negative_controls(
            seed: u64,
            acquisition: usize,
        ) -> Option<[bool; 5]> {
            let export = frozen_a1::d2_export(seed, acquisition)?;
            let mut tie = DifferentialWorkspace::default();
            let tie_abstains = tie
                .form(
                    [Some(&export.predictions[0]), Some(&export.predictions[0])],
                    &export.predictions[0],
                )
                .is_none();
            let mut neither_evidence = export.predictions[0].clone();
            neither_evidence.trace.push(255);
            let mut neither = DifferentialWorkspace::default();
            let neither_abstains = neither
                .form(
                    [Some(&export.predictions[0]), Some(&export.predictions[1])],
                    &neither_evidence,
                )
                .is_none();
            let mut removed = DifferentialWorkspace::default();
            let removed_abstains = removed
                .form([None, Some(&export.predictions[1])], &export.predictions[0])
                .is_none();
            let mut swapped = DifferentialWorkspace::default();
            let swap_reverses = swapped.form(
                [Some(&export.predictions[1]), Some(&export.predictions[0])],
                &export.predictions[0],
            ) == Some(1);
            let cleanup =
                tie.cleanup() && neither.cleanup() && removed.cleanup() && swapped.cleanup();
            Some([
                tie_abstains,
                neither_abstains,
                removed_abstains,
                swap_reverses,
                cleanup,
            ])
        }
    };
}

#[allow(dead_code)]
mod frozen_d2 {
    include!(concat!(env!("OUT_DIR"), "/ds_d2_differential_evidence.rs"));
    composition_d2_access!();
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FunctionalProbe {
    acquisition_episodes: usize,
    direction_deliveries: usize,
    physical_directions: usize,
    update_calls: u64,
    patterns: usize,
    divergent_patterns: usize,
    correctly_mature_patterns: usize,
    held_out_episodes: usize,
    held_out_attempts: usize,
    held_out_successes: usize,
    held_out_abstentions: usize,
    d2_work: u64,
    learner_work: u64,
    persistent_learner_bytes: usize,
    learner_fingerprint: u64,
    acquisition_used_expected_relation: bool,
}

macro_rules! composition_e0_access {
    () => {
        fn evaluator_expected(signature: Signature) -> usize {
            usize::from(signature.gap - 1) ^ usize::from(signature.witness_attachment)
        }

        pub(super) fn composition_probe(
            seed: u64,
            acquisition: usize,
            evaluation: usize,
        ) -> Option<super::FunctionalProbe> {
            let (mut formation, _) = acquire(seed, acquisition);
            let mut learner = Learner::default();
            let mut direction_deliveries = 0usize;
            let mut physical_directions = 0usize;
            let mut d2_work = 0u64;
            for ordinal in 0..acquisition {
                let context = ordinal % 4;
                let episode = fixture(
                    seed + 20_000,
                    acquisition + 100 + ordinal,
                    context,
                    Perturbation::None,
                );
                let event = formation.form(&episode.raw)?;
                let view = serialize_once(&event, &mut formation.work);
                let (choice, _) = learner.choose(&view, ordinal + seed as usize);
                let direction = super::frozen_d2::composition_direction(
                    100 + (ordinal % 5) as u64,
                    acquisition.max(16),
                    choice,
                )?;
                if direction.direction == Some(choice) {
                    direction_deliveries += 1;
                }
                physical_directions += usize::from(direction.physical && direction.cleanup);
                d2_work += direction.work;
                learner.apply_consequence(&view, choice, direction.direction == Some(choice));
            }
            let divergent_patterns = learner
                .patterns
                .iter()
                .filter(|pattern| pattern.evidence[0].strength != pattern.evidence[1].strength)
                .count();
            let correctly_mature_patterns = learner
                .patterns
                .iter()
                .filter(|pattern| pattern.mature == Some(evaluator_expected(pattern.signature)))
                .count();
            let mut held_out_attempts = 0usize;
            let mut held_out_successes = 0usize;
            let mut held_out_abstentions = 0usize;
            for ordinal in 0..evaluation {
                let context = ordinal % 4;
                let episode = fixture(
                    seed + 30_000,
                    acquisition + 10_000 + ordinal,
                    context,
                    Perturbation::None,
                );
                let event = formation.form(&episode.raw)?;
                let view = serialize_once(&event, &mut formation.work);
                let expected = evaluator_expected(Learner::signature(&view));
                if let Some(choice) = learner.frozen_choice(&view) {
                    held_out_attempts += 1;
                    held_out_successes += usize::from(choice == expected);
                } else {
                    held_out_abstentions += 1;
                }
            }
            Some(super::FunctionalProbe {
                acquisition_episodes: acquisition,
                direction_deliveries,
                physical_directions,
                update_calls: learner.credit_updates,
                patterns: learner.patterns.len(),
                divergent_patterns,
                correctly_mature_patterns,
                held_out_episodes: evaluation,
                held_out_attempts,
                held_out_successes,
                held_out_abstentions,
                d2_work,
                learner_work: learner.learner_work(),
                persistent_learner_bytes: learner.persistent_bytes(),
                learner_fingerprint: learner.fingerprint(),
                acquisition_used_expected_relation: false,
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_e0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_e0_anonymous_event_formation.rs"
    ));
    composition_e0_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub d2_hash: bool,
    pub d2_handoff_hash: bool,
    pub c0_hash: bool,
    pub c0_readiness_hash: bool,
    pub e0_hash: bool,
    pub protocol_amendment_hash: bool,
    pub ds1_hash: bool,
    pub d2_lineage: bool,
    pub c0_lineage: bool,
    pub update_edges: usize,
    pub semantic_acquisition_edges: usize,
    pub definitive_edges: usize,
}

fn source_audit() -> SourceAudit {
    let source = include_str!("ds1_after_d2_cumulative_composition_retry.rs");
    let test_boundary = ["#[cfg(", "test)]"].concat();
    let production = source.split(&test_boundary).next().unwrap_or(source);
    let update_call = ["learner.apply_", "consequence("].concat();
    let definitive_marker = ["HarnessMode::", "Definitive"].concat();
    let semantic_calls = [
        ["correct_", "choice("].concat(),
        ["reward_", "update("].concat(),
        ["expected_", "during_acquisition("].concat(),
        ["semantic_", "polarity("].concat(),
    ];
    SourceAudit {
        d2_hash: env!("DS1_D2_D2_SHA256") == FROZEN_D2_SHA256,
        d2_handoff_hash: env!("DS1_D2_D2_HANDOFF_SHA256") == FROZEN_D2_HANDOFF_SHA256,
        c0_hash: env!("DS1_D2_C0_SHA256") == FROZEN_C0_SHA256,
        c0_readiness_hash: env!("DS1_D2_C0_READINESS_SHA256") == FROZEN_C0_READINESS_SHA256,
        e0_hash: env!("DS1_D2_E0_SHA256") == FROZEN_E0_SHA256,
        protocol_amendment_hash: env!("DS1_D2_PROTOCOL_AMENDMENT_SHA256")
            == PROTOCOL_AMENDMENT_SHA256,
        ds1_hash: frozen_e0::FROZEN_DS1_LEARNER_SHA256 == FROZEN_DS1_SHA256,
        d2_lineage: frozen_d2::AUTHORITATIVE_M0 == AUTHORITATIVE_M0
            && frozen_d2::FROZEN_DS1_SHA256 == FROZEN_DS1_SHA256
            && frozen_d2::FROZEN_RESULTS_DIGEST == FROZEN_RESULTS_DIGEST,
        c0_lineage: frozen_c0::AUTHORITATIVE_M0 == AUTHORITATIVE_M0
            && frozen_c0::FROZEN_DS1_SHA256 == FROZEN_DS1_SHA256
            && frozen_c0::FROZEN_RESULTS_DIGEST == FROZEN_RESULTS_DIGEST,
        update_edges: production.matches(&update_call).count(),
        semantic_acquisition_edges: semantic_calls
            .iter()
            .map(|call| production.matches(call).count())
            .sum(),
        definitive_edges: production.matches(&definitive_marker).count(),
    }
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.d2_hash
            && self.d2_handoff_hash
            && self.c0_hash
            && self.c0_readiness_hash
            && self.e0_hash
            && self.protocol_amendment_hash
            && self.ds1_hash
            && self.d2_lineage
            && self.c0_lineage
            && self.update_edges == 1
            && self.semantic_acquisition_edges == 0
            && self.definitive_edges == 2
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub acquisition_episodes: usize,
    pub direction_deliveries: usize,
    pub physical_directions: usize,
    pub update_calls: u64,
    pub patterns: usize,
    pub divergent_patterns: usize,
    pub correctly_mature_patterns: usize,
    pub held_out_episodes: usize,
    pub held_out_attempts: usize,
    pub held_out_successes: usize,
    pub held_out_abstentions: usize,
    pub d2_work: u64,
    pub learner_work: u64,
    pub persistent_learner_bytes: usize,
    pub learner_fingerprint: u64,
    pub negative_controls: [bool; 5],
    pub stage_ready: [bool; 12],
}

fn audit_seed(
    seed: u64,
    acquisition: usize,
    evaluation: usize,
    c0_ready: bool,
    d2_ready: bool,
    source: &SourceAudit,
) -> SeedAudit {
    let probe = frozen_e0::composition_probe(seed, acquisition, evaluation)
        .expect("frozen E0/DS1 acquisition schedule");
    let negative_controls = frozen_d2::composition_negative_controls(seed, acquisition.max(16))
        .expect("frozen D2 negative controls");
    let stages = [
        source.passed() && c0_ready && d2_ready,
        c0_ready,
        c0_ready,
        c0_ready,
        c0_ready,
        c0_ready,
        c0_ready,
        d2_ready && negative_controls.iter().all(|control| *control),
        probe.direction_deliveries == acquisition
            && probe.physical_directions == acquisition
            && !probe.acquisition_used_expected_relation,
        probe.update_calls == acquisition as u64,
        probe.patterns == 4 && probe.divergent_patterns == probe.patterns,
        probe.correctly_mature_patterns == probe.patterns
            && probe.held_out_attempts == evaluation
            && probe.held_out_successes == evaluation
            && probe.held_out_abstentions == 0,
    ];
    SeedAudit {
        seed,
        acquisition_episodes: acquisition,
        direction_deliveries: probe.direction_deliveries,
        physical_directions: probe.physical_directions,
        update_calls: probe.update_calls,
        patterns: probe.patterns,
        divergent_patterns: probe.divergent_patterns,
        correctly_mature_patterns: probe.correctly_mature_patterns,
        held_out_episodes: probe.held_out_episodes,
        held_out_attempts: probe.held_out_attempts,
        held_out_successes: probe.held_out_successes,
        held_out_abstentions: probe.held_out_abstentions,
        d2_work: probe.d2_work,
        learner_work: probe.learner_work,
        persistent_learner_bytes: probe.persistent_learner_bytes,
        learner_fingerprint: probe.learner_fingerprint,
        negative_controls,
        stage_ready: stages,
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
pub struct Report {
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

fn rejected() -> Report {
    Report {
        label: "UNCHANGED DS1 AFTER D2: definitive forbidden".to_string(),
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

pub fn run(mode: HarnessMode) -> Report {
    if mode == HarnessMode::Definitive {
        return rejected();
    }
    let source = source_audit();
    let c0 = frozen_c0::run(mode);
    let d2 = frozen_d2::run(mode);
    let (acquisition, evaluation, mode_label) = match mode {
        HarnessMode::Micro => (16, 8, "MICRO"),
        HarnessMode::Gate => (32, 16, "GATE"),
        HarnessMode::Definitive => unreachable!(),
    };
    let seeds = d2
        .seeds
        .iter()
        .map(|d2_seed| {
            let c0_ready = c0
                .seeds
                .iter()
                .find(|c0_seed| c0_seed.seed == d2_seed.seed)
                .is_some_and(|c0_seed| {
                    c0_seed.stage_ready.iter().all(|stage| *stage) && c0_seed.controls.passed()
                });
            audit_seed(
                d2_seed.seed,
                acquisition,
                evaluation,
                c0_ready,
                d2_seed.passed,
                &source,
            )
        })
        .collect::<Vec<_>>();
    let ready = std::array::from_fn(|stage| {
        !seeds.is_empty() && seeds.iter().all(|seed| seed.stage_ready[stage])
    });
    let (stages, first_collapse_stage) = ordered_freeze(ready);
    let first_collapse = first_collapse_stage
        .map(|stage| STAGES[stage].to_string())
        .unwrap_or_else(|| "NONE: all cumulative DS1 stages ready".to_string());
    let expected_collapse = first_collapse_stage == Some(11);
    let full_pass = first_collapse_stage.is_none();
    let audit_passed = source.passed()
        && c0.audit_passed
        && d2.audit_passed
        && (expected_collapse || full_pass)
        && seeds.iter().all(|seed| {
            seed.stage_ready[..11].iter().all(|stage| *stage)
                && seed.negative_controls.iter().all(|control| *control)
        });
    Report {
        label: first_collapse_stage.map_or_else(
            || "UNCHANGED DS1 AFTER D2: M1 CREATED IN DEVELOPMENT".to_string(),
            |stage| format!("UNCHANGED DS1 AFTER D2 COLLAPSE AT {}", STAGES[stage]),
        ),
        protocol: PROTOCOL.to_string(),
        mode: mode_label.to_string(),
        claim_eligible: false,
        m0_authoritative: !full_pass,
        enabling_only: !full_pass,
        m1_exists: full_pass,
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
    fn micro_reaches_the_frozen_update_path() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert!(report
            .seeds
            .iter()
            .all(|seed| seed.stage_ready[..11].iter().all(|stage| *stage)));
    }

    #[test]
    fn gate_freezes_only_at_observed_generalization_boundary_or_passes() {
        let report = run(HarnessMode::Gate);
        assert!(report.audit_passed, "{report:#?}");
        assert!(matches!(report.first_collapse_stage, Some(11) | None));
    }

    #[test]
    fn no_semantic_target_enters_acquisition() {
        let report = run(HarnessMode::Gate);
        assert_eq!(report.source.semantic_acquisition_edges, 0);
        assert!(report.seeds.iter().all(|seed| {
            seed.direction_deliveries == seed.acquisition_episodes
                && seed.update_calls == seed.acquisition_episodes as u64
        }));
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.audit_passed && report.seeds.is_empty() && !report.m1_exists);
    }
}
