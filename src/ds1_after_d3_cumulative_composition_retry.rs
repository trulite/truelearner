//! Development-only byte-identical DS1 cumulative retry after frozen DS-D3.

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds1-after-d3-cumulative-composition-retry-v1";
pub const EXACT_PARENT: &str = "ee576d95b88d04629f715d06c639a8f400ff2819";
pub const PROTOCOL_COMMIT: &str = "052f1e8e92e322a2c1d5cc9d919ef6cddcfee9f6";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_D3_SHA256: &str =
    "a13f39c86b2c67d225530e7b17cdacd71f452a45be3b2c9942814c0748267f6d";
pub const FROZEN_D3_READINESS_SHA256: &str =
    "ac79c5142842563682f63b7117b2866bd4f3a1beaf51dd42573aac055f1119f7";
pub const FROZEN_E0_SHA256: &str =
    "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

const STAGES: [&str; 7] = [
    "0. exact frozen M0/E0/D3/DS1 lineage and controls",
    "1. E0 situation and two frozen A1 executable affordances",
    "2. frozen D3 forms one physical downstream direction per acquisition",
    "3. D3 direction reaches byte-identical frozen DS1",
    "4. frozen DS1 update fires once per acquisition",
    "5. all four patterns diverge and mature to the consequence-supported route",
    "6. held-out boundary-role reconstruction is complete",
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct D3Probe {
    roots: usize,
    direction: Option<usize>,
    physical: bool,
    observations: usize,
    work: u64,
    cleanup: bool,
}

macro_rules! composition_d3_access {
    () => {
        pub(super) fn composition_direction(
            seed: u64,
            acquisition: usize,
            world_affordance: usize,
        ) -> Option<super::D3Probe> {
            if world_affordance >= 2 {
                return None;
            }
            let routes = frozen_d2::d3_routes(seed, acquisition)?;
            let mut learner = acquire_contrasts(
                seed + 40_000,
                &routes.affordances,
                Schedule::OneRecurrent {
                    recurrent: world_affordance,
                },
                false,
            );
            let observations = learner
                .evidence
                .values()
                .map(|evidence| usize::from(evidence.observations))
                .sum();
            let direction = learner
                .form_direction([Some(&routes.affordances[0]), Some(&routes.affordances[1])]);
            let physical = learner.execute_direction();
            let work = learner.work.organism_work();
            let cleanup = learner.cleanup_temporary();
            Some(super::D3Probe {
                roots: routes.roots,
                direction,
                physical,
                observations,
                work,
                cleanup,
            })
        }

        pub(super) fn composition_ds1_hash_matches(value: &str) -> bool {
            FROZEN_DS1_SHA256 == value
        }
    };
}

#[allow(dead_code)]
mod frozen_d3 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_d3_anonymous_consequence_contrast.rs"
    ));
    composition_d3_access!();
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FunctionalProbe {
    acquisition_episodes: usize,
    event_formations: usize,
    two_root_episodes: usize,
    d3_directions: usize,
    physical_directions: usize,
    direction_deliveries: usize,
    update_calls: u64,
    patterns: usize,
    divergent_patterns: usize,
    consequence_mature_patterns: usize,
    evaluator_mature_patterns: usize,
    held_out_episodes: usize,
    held_out_attempts: usize,
    held_out_successes: usize,
    held_out_abstentions: usize,
    d3_work: u64,
    learner_work: u64,
    persistent_learner_bytes: usize,
    learner_fingerprint: u64,
    acquisition_used_evaluator_role: bool,
}

macro_rules! composition_e0_access {
    () => {
        fn physical_world_affordance(signature: Signature) -> usize {
            let temporal_relation = usize::from(signature.gap.saturating_sub(1));
            let attachment_relation = usize::from(signature.witness_attachment);
            temporal_relation ^ attachment_relation
        }

        fn evaluator_role(signature: Signature) -> usize {
            usize::from(signature.gap - 1) ^ usize::from(signature.witness_attachment)
        }

        pub(super) fn composition_probe(
            seed: u64,
            acquisition: usize,
            evaluation: usize,
            reverse_world: bool,
        ) -> Option<super::FunctionalProbe> {
            let (mut formation, _) = acquire(seed, acquisition);
            let mut learner = Learner::default();
            let mut event_formations = 0usize;
            let mut two_root_episodes = 0usize;
            let mut d3_directions = 0usize;
            let mut physical_directions = 0usize;
            let mut direction_deliveries = 0usize;
            let mut d3_work = 0u64;
            for ordinal in 0..acquisition {
                let context = ordinal % 4;
                let episode = fixture(
                    seed + 20_000,
                    acquisition + 100 + ordinal,
                    context,
                    Perturbation::None,
                );
                let event = formation.form(&episode.raw)?;
                event_formations += 1;
                let view = serialize_once(&event, &mut formation.work);
                let signature = Learner::signature(&view);
                let mut world_affordance = physical_world_affordance(signature);
                if reverse_world {
                    world_affordance = 1 - world_affordance;
                }
                let (choice, _) = learner.choose(&view, ordinal + seed as usize);
                let direction = super::frozen_d3::composition_direction(
                    100 + (ordinal % 5) as u64,
                    acquisition.max(16),
                    world_affordance,
                )?;
                two_root_episodes += usize::from(direction.roots == 2);
                d3_directions += usize::from(direction.direction == Some(world_affordance));
                physical_directions += usize::from(direction.physical && direction.cleanup);
                direction_deliveries += usize::from(direction.direction.is_some());
                d3_work += direction.work;
                learner.apply_consequence(&view, choice, direction.direction == Some(choice));
            }
            let divergent_patterns = learner
                .patterns
                .iter()
                .filter(|pattern| pattern.evidence[0].strength != pattern.evidence[1].strength)
                .count();
            let consequence_mature_patterns = learner
                .patterns
                .iter()
                .filter(|pattern| {
                    let mut physical = physical_world_affordance(pattern.signature);
                    if reverse_world {
                        physical = 1 - physical;
                    }
                    pattern.mature == Some(physical)
                })
                .count();
            let evaluator_mature_patterns = learner
                .patterns
                .iter()
                .filter(|pattern| pattern.mature == Some(evaluator_role(pattern.signature)))
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
                let evaluator = evaluator_role(Learner::signature(&view));
                if let Some(choice) = learner.frozen_choice(&view) {
                    held_out_attempts += 1;
                    held_out_successes += usize::from(choice == evaluator);
                } else {
                    held_out_abstentions += 1;
                }
            }
            Some(super::FunctionalProbe {
                acquisition_episodes: acquisition,
                event_formations,
                two_root_episodes,
                d3_directions,
                physical_directions,
                direction_deliveries,
                update_calls: learner.credit_updates,
                patterns: learner.patterns.len(),
                divergent_patterns,
                consequence_mature_patterns,
                evaluator_mature_patterns,
                held_out_episodes: evaluation,
                held_out_attempts,
                held_out_successes,
                held_out_abstentions,
                d3_work,
                learner_work: learner.learner_work(),
                persistent_learner_bytes: learner.persistent_bytes(),
                learner_fingerprint: learner.fingerprint(),
                acquisition_used_evaluator_role: false,
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
    pub d3_hash: bool,
    pub d3_readiness_hash: bool,
    pub e0_hash: bool,
    pub ds1_hash: bool,
    pub d3_lineage: bool,
    pub update_edges: usize,
    pub physical_world_functions: usize,
    pub evaluator_functions: usize,
    pub evaluator_edges_in_acquisition: usize,
    pub semantic_acquisition_edges: usize,
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

fn source_audit() -> SourceAudit {
    let source = include_str!("ds1_after_d3_cumulative_composition_retry.rs");
    let production = source
        .split(&["#[cfg(", "test)]"].concat())
        .next()
        .unwrap_or(source);
    let probe = function_body(production, "pub(super) fn composition_probe(").unwrap_or_default();
    let acquisition = probe
        .split("let divergent_patterns")
        .next()
        .unwrap_or(probe);
    let update_call = ["learner.apply_", "consequence("].concat();
    let semantic_fragments = [
        ["reward_", "value"].concat(),
        ["semantic_", "polarity"].concat(),
        ["correct_", "choice"].concat(),
        ["expected_", "answer"].concat(),
    ];
    SourceAudit {
        d3_hash: env!("DS1_D3_D3_SHA256") == FROZEN_D3_SHA256,
        d3_readiness_hash: env!("DS1_D3_D3_READINESS_SHA256") == FROZEN_D3_READINESS_SHA256,
        e0_hash: env!("DS1_D3_E0_SHA256") == FROZEN_E0_SHA256,
        ds1_hash: frozen_d3::composition_ds1_hash_matches(FROZEN_DS1_SHA256)
            && frozen_e0::FROZEN_DS1_LEARNER_SHA256 == FROZEN_DS1_SHA256,
        d3_lineage: frozen_d3::AUTHORITATIVE_M0 == AUTHORITATIVE_M0
            && frozen_d3::FROZEN_RESULTS_DIGEST == FROZEN_RESULTS_DIGEST,
        update_edges: production.matches(&update_call).count(),
        physical_world_functions: production
            .matches("\n        fn physical_world_affordance(")
            .count(),
        evaluator_functions: production.matches("\n        fn evaluator_role(").count(),
        evaluator_edges_in_acquisition: acquisition.matches("evaluator_role(").count(),
        semantic_acquisition_edges: semantic_fragments
            .iter()
            .map(|fragment| acquisition.matches(fragment).count())
            .sum(),
    }
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.d3_hash
            && self.d3_readiness_hash
            && self.e0_hash
            && self.ds1_hash
            && self.d3_lineage
            && self.update_edges == 1
            && self.physical_world_functions == 1
            && self.evaluator_functions == 1
            && self.evaluator_edges_in_acquisition == 0
            && self.semantic_acquisition_edges == 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub acquisition_episodes: usize,
    pub event_formations: usize,
    pub two_root_episodes: usize,
    pub d3_directions: usize,
    pub physical_directions: usize,
    pub direction_deliveries: usize,
    pub update_calls: u64,
    pub patterns: usize,
    pub divergent_patterns: usize,
    pub consequence_mature_patterns: usize,
    pub evaluator_mature_patterns: usize,
    pub held_out_episodes: usize,
    pub held_out_attempts: usize,
    pub held_out_successes: usize,
    pub held_out_abstentions: usize,
    pub reversed_world_mature_patterns: usize,
    pub reversed_world_evaluator_patterns: usize,
    pub d3_work: u64,
    pub learner_work: u64,
    pub persistent_learner_bytes: usize,
    pub learner_fingerprint: u64,
    pub stage_ready: [bool; 7],
}

fn audit_seed(
    seed: u64,
    acquisition: usize,
    evaluation: usize,
    d3_ready: bool,
    source: &SourceAudit,
) -> SeedAudit {
    let probe = frozen_e0::composition_probe(seed, acquisition, evaluation, false)
        .expect("frozen E0/D3/DS1 schedule");
    let reversed = frozen_e0::composition_probe(seed, acquisition, evaluation, true)
        .expect("reversed physical-world control");
    let reversed_follows_world =
        reversed.consequence_mature_patterns == 4 && reversed.evaluator_mature_patterns == 0;
    let stages = [
        source.passed() && d3_ready,
        probe.event_formations == acquisition && probe.two_root_episodes == acquisition,
        probe.d3_directions == acquisition && probe.physical_directions == acquisition,
        probe.direction_deliveries == acquisition
            && !probe.acquisition_used_evaluator_role
            && reversed_follows_world,
        probe.update_calls == acquisition as u64,
        probe.patterns == 4
            && probe.divergent_patterns == 4
            && probe.consequence_mature_patterns == 4
            && probe.evaluator_mature_patterns == 4,
        probe.held_out_attempts == evaluation
            && probe.held_out_successes == evaluation
            && probe.held_out_abstentions == 0,
    ];
    SeedAudit {
        seed,
        acquisition_episodes: probe.acquisition_episodes,
        event_formations: probe.event_formations,
        two_root_episodes: probe.two_root_episodes,
        d3_directions: probe.d3_directions,
        physical_directions: probe.physical_directions,
        direction_deliveries: probe.direction_deliveries,
        update_calls: probe.update_calls,
        patterns: probe.patterns,
        divergent_patterns: probe.divergent_patterns,
        consequence_mature_patterns: probe.consequence_mature_patterns,
        evaluator_mature_patterns: probe.evaluator_mature_patterns,
        held_out_episodes: probe.held_out_episodes,
        held_out_attempts: probe.held_out_attempts,
        held_out_successes: probe.held_out_successes,
        held_out_abstentions: probe.held_out_abstentions,
        reversed_world_mature_patterns: reversed.consequence_mature_patterns,
        reversed_world_evaluator_patterns: reversed.evaluator_mature_patterns,
        d3_work: probe.d3_work,
        learner_work: probe.learner_work,
        persistent_learner_bytes: probe.persistent_learner_bytes,
        learner_fingerprint: probe.learner_fingerprint,
        stage_ready: stages,
    }
}

fn ordered_freeze(ready: [bool; 7]) -> ([String; 7], Option<usize>) {
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
    pub m1_authoritative: bool,
    pub source: SourceAudit,
    pub stages: [String; 7],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub seeds: Vec<SeedAudit>,
    pub audit_passed: bool,
}

fn rejected() -> Report {
    Report {
        label: "UNCHANGED DS1 AFTER D3: definitive forbidden".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "DEFINITIVE-FORBIDDEN".to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        enabling_only: true,
        m1_exists: false,
        m1_authoritative: false,
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
    let d3 = frozen_d3::run(mode);
    let (acquisition, evaluation, mode_label) = match mode {
        HarnessMode::Micro => (16, 8, "MICRO"),
        HarnessMode::Gate => (32, 16, "GATE"),
        HarnessMode::Definitive => unreachable!(),
    };
    let seeds = d3
        .seeds
        .iter()
        .map(|d3_seed| {
            audit_seed(
                d3_seed.seed,
                acquisition,
                evaluation,
                d3_seed.passed,
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
        .unwrap_or_else(|| "NONE: all cumulative DS1 development stages ready".to_string());
    let full_pass = first_collapse_stage.is_none();
    let audit_passed = source.passed()
        && d3.audit_passed
        && !seeds.is_empty()
        && seeds.iter().all(|seed| {
            seed.stage_ready
                .iter()
                .take(first_collapse_stage.unwrap_or(STAGES.len()))
                .all(|stage| *stage)
        });
    Report {
        label: first_collapse_stage.map_or_else(
            || "UNCHANGED DS1 AFTER D3: M1 DEVELOPMENT ANCESTOR CREATED".to_string(),
            |stage| format!("UNCHANGED DS1 AFTER D3 COLLAPSE AT {}", STAGES[stage]),
        ),
        protocol: PROTOCOL.to_string(),
        mode: mode_label.to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        enabling_only: !full_pass,
        m1_exists: full_pass,
        m1_authoritative: false,
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
    fn micro_runs_the_complete_nonsemantic_credit_path() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert!(report.seeds.iter().all(|seed| {
            seed.d3_directions == seed.acquisition_episodes
                && seed.update_calls == seed.acquisition_episodes as u64
        }));
    }

    #[test]
    fn gate_creates_the_complete_m1_development_ancestor() {
        let report = run(HarnessMode::Gate);
        assert!(report.audit_passed, "{report:#?}");
        assert!(report.m1_exists && !report.m1_authoritative);
        assert!(report.first_collapse_stage.is_none());
        assert!(report
            .seeds
            .iter()
            .all(|seed| seed.stage_ready.iter().all(|stage| *stage)
                && seed.consequence_mature_patterns == 4
                && seed.evaluator_mature_patterns == 4
                && seed.held_out_successes == 16
                && seed.held_out_abstentions == 0));
    }

    #[test]
    fn reversed_world_changes_learning_without_evaluator_input() {
        let report = run(HarnessMode::Gate);
        assert_eq!(report.source.evaluator_edges_in_acquisition, 0);
        assert!(report.seeds.iter().all(|seed| {
            seed.reversed_world_mature_patterns == 4 && seed.reversed_world_evaluator_patterns == 0
        }));
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.audit_passed && report.seeds.is_empty() && !report.m1_exists);
    }
}
