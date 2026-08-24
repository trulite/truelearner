//! Diagnostic-only functional sufficiency screen for stage-8b properties.

use crate::research_runtime::{parallel_map_ordered, HarnessMode};

pub const PROTOCOL: &str = "ds-d1-stage8b-functional-sufficiency-v1";
pub const EXACT_PARENT: &str = "b599e601e9a7257c647cf5ca8f4188d77d024f02";
pub const PROTOCOL_COMMIT: &str = "a916a540f31b2a3b7cf6c863f3611c339ccc6268";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_D0_SOURCE_SHA256: &str =
    "00d30cd3d71d002e4e37fc3a47f94cb6d9bacb8ec97be54a04318b92195b4902";
pub const FROZEN_D0_HANDOFF_SHA256: &str =
    "97eb2fe66000f087bdee591a3471d4c15cfe0f0273d688ff299e53e54612f650";
pub const FROZEN_E0_SHA256: &str =
    "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Arm {
    AlternativeComparison,
    Polarity,
    OutcomeChange,
}

impl Arm {
    pub const ALL: [Self; 3] = [
        Self::AlternativeComparison,
        Self::Polarity,
        Self::OutcomeChange,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::AlternativeComparison => "ALTERNATIVE_COMPARISON",
            Self::Polarity => "POLARITY",
            Self::OutcomeChange => "OUTCOME_CHANGE",
        }
    }

    fn property_fields(self) -> usize {
        match self {
            Self::Polarity => 1,
            Self::AlternativeComparison | Self::OutcomeChange => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Direction {
    positive: bool,
    fields: usize,
}

fn direction_for(arm: Arm, choice: usize, expected: usize) -> Direction {
    match arm {
        Arm::AlternativeComparison => {
            let mut outcomes = [0i16; 2];
            outcomes[expected] = 2;
            Direction {
                positive: outcomes[choice] > outcomes[1usize.saturating_sub(choice)],
                fields: 2,
            }
        }
        Arm::Polarity => Direction {
            positive: choice == expected,
            fields: 1,
        },
        Arm::OutcomeChange => {
            let before = 0i16;
            let after = if choice == expected { 1 } else { -1 };
            Direction {
                positive: after > before,
                fields: 2,
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FunctionalProbe {
    acquisition_episodes: usize,
    update_calls: u64,
    patterns: usize,
    divergent_patterns: usize,
    correctly_mature_patterns: usize,
    held_out_episodes: usize,
    held_out_attempts: usize,
    held_out_successes: usize,
    held_out_abstentions: usize,
    boolean_trace_fingerprint: u64,
    episode_fingerprint: u64,
    learner_fingerprint: u64,
    learner_work: u64,
    persistent_learner_bytes: usize,
    persistent_property_bytes: usize,
    property_fields: usize,
}

fn mix(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(0x100_0000_01b3);
}

macro_rules! functional_e0_access {
    () => {
        fn expected(signature: Signature) -> usize {
            usize::from(signature.gap - 1) ^ usize::from(signature.witness_attachment)
        }

        pub(super) fn functional_probe(
            seed: u64,
            acquisition: usize,
            evaluation: usize,
            arm: super::Arm,
        ) -> Option<super::FunctionalProbe> {
            let (mut formation, _) = acquire(seed, acquisition);
            let mut learner = Learner::default();
            let mut boolean_trace = 0xcbf2_9ce4_8422_2325u64;
            let mut episode_fingerprint = 0xcbf2_9ce4_8422_2325u64;
            let mut observed_fields = 0usize;
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
                let signature = Learner::signature(&view);
                let expected = expected(signature);
                let (choice, _) = learner.choose(&view, ordinal + seed as usize);
                let direction = super::direction_for(arm, choice, expected);
                observed_fields = direction.fields;
                super::mix(&mut boolean_trace, u64::from(direction.positive));
                for value in [
                    signature.gap as u64,
                    signature.witness_attachment as u64,
                    expected as u64,
                    ordinal as u64,
                ] {
                    super::mix(&mut episode_fingerprint, value);
                }
                learner.apply_consequence(&view, choice, direction.positive);
            }
            let divergent_patterns = learner
                .patterns
                .iter()
                .filter(|pattern| pattern.evidence[0].strength != pattern.evidence[1].strength)
                .count();
            let correctly_mature_patterns = learner
                .patterns
                .iter()
                .filter(|pattern| pattern.mature == Some(expected(pattern.signature)))
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
                let expected = expected(Learner::signature(&view));
                if let Some(choice) = learner.frozen_choice(&view) {
                    held_out_attempts += 1;
                    held_out_successes += usize::from(choice == expected);
                } else {
                    held_out_abstentions += 1;
                }
            }
            Some(super::FunctionalProbe {
                acquisition_episodes: acquisition,
                update_calls: learner.credit_updates,
                patterns: learner.patterns.len(),
                divergent_patterns,
                correctly_mature_patterns,
                held_out_episodes: evaluation,
                held_out_attempts,
                held_out_successes,
                held_out_abstentions,
                boolean_trace_fingerprint: boolean_trace,
                episode_fingerprint,
                learner_fingerprint: learner.fingerprint(),
                learner_work: learner.learner_work(),
                persistent_learner_bytes: learner.persistent_bytes(),
                persistent_property_bytes: 0,
                property_fields: observed_fields,
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
    functional_e0_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub d0_source_hash: bool,
    pub d0_handoff_hash: bool,
    pub e0_hash: bool,
    pub ds1_hash: bool,
    pub arm_variants: usize,
    pub combination_variants: usize,
    pub diagnostic_update_edges: usize,
    pub parallel_matrix_calls: usize,
    pub expected_relation_edges: usize,
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
    let source = include_str!("ds_d1_stage8b_functional_sufficiency.rs");
    let update_call = ["learner.apply_", "consequence("].concat();
    let run_body = function_body(source, "\npub fn run(").unwrap_or_default();
    SourceAudit {
        d0_source_hash: env!("DS_D1_D0_SOURCE_SHA256") == FROZEN_D0_SOURCE_SHA256,
        d0_handoff_hash: env!("DS_D1_D0_HANDOFF_SHA256") == FROZEN_D0_HANDOFF_SHA256,
        e0_hash: env!("DS_D1_E0_SHA256") == FROZEN_E0_SHA256,
        ds1_hash: frozen_e0::FROZEN_DS1_LEARNER_SHA256 == FROZEN_DS1_SHA256,
        arm_variants: Arm::ALL.len(),
        combination_variants: 0,
        diagnostic_update_edges: source.matches(&update_call).count(),
        parallel_matrix_calls: run_body.matches("parallel_map_ordered(").count(),
        expected_relation_edges: source
            .matches("\n        fn expected(signature: Signature)")
            .count(),
    }
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.d0_source_hash
            && self.d0_handoff_hash
            && self.e0_hash
            && self.ds1_hash
            && self.arm_variants == 3
            && self.combination_variants == 0
            && self.diagnostic_update_edges == 1
            && self.parallel_matrix_calls == 1
            && self.expected_relation_edges == 1
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellReport {
    pub seed: u64,
    pub arm: Arm,
    pub arm_label: String,
    pub property_fields: usize,
    pub acquisition_episodes: usize,
    pub update_fires: bool,
    pub update_calls: u64,
    pub patterns: usize,
    pub strength_divergence: bool,
    pub divergent_patterns: usize,
    pub correctly_mature_patterns: usize,
    pub held_out_role_recovery: bool,
    pub held_out_episodes: usize,
    pub held_out_successes: usize,
    pub held_out_abstentions: usize,
    pub boolean_trace_fingerprint: u64,
    pub episode_fingerprint: u64,
    pub learner_fingerprint: u64,
    pub learner_work: u64,
    pub persistent_learner_bytes: usize,
    pub persistent_property_bytes: usize,
    pub current_substrate_observability_established: bool,
    pub diagnostic_only: bool,
    pub passed: bool,
}

fn screen(seed: u64, arm: Arm, acquisition: usize, evaluation: usize) -> CellReport {
    let probe = frozen_e0::functional_probe(seed, acquisition, evaluation, arm)
        .expect("frozen E0 functional diagnostic episode schedule");
    let update_fires = probe.update_calls == acquisition as u64;
    let strength_divergence = update_fires
        && probe.patterns == 4
        && probe.divergent_patterns == probe.patterns
        && probe.correctly_mature_patterns == probe.patterns;
    let held_out_role_recovery = strength_divergence
        && probe.held_out_attempts == evaluation
        && probe.held_out_successes == evaluation
        && probe.held_out_abstentions == 0;
    let passed = probe.property_fields == arm.property_fields()
        && update_fires
        && strength_divergence
        && held_out_role_recovery
        && probe.persistent_property_bytes == 0;
    CellReport {
        seed,
        arm,
        arm_label: arm.label().to_string(),
        property_fields: probe.property_fields,
        acquisition_episodes: probe.acquisition_episodes,
        update_fires,
        update_calls: probe.update_calls,
        patterns: probe.patterns,
        strength_divergence,
        divergent_patterns: probe.divergent_patterns,
        correctly_mature_patterns: probe.correctly_mature_patterns,
        held_out_role_recovery,
        held_out_episodes: probe.held_out_episodes,
        held_out_successes: probe.held_out_successes,
        held_out_abstentions: probe.held_out_abstentions,
        boolean_trace_fingerprint: probe.boolean_trace_fingerprint,
        episode_fingerprint: probe.episode_fingerprint,
        learner_fingerprint: probe.learner_fingerprint,
        learner_work: probe.learner_work,
        persistent_learner_bytes: probe.persistent_learner_bytes,
        persistent_property_bytes: probe.persistent_property_bytes,
        current_substrate_observability_established: false,
        diagnostic_only: true,
        passed,
    }
}

fn equivalent_by_seed(cells: &[CellReport]) -> bool {
    let mut seeds = cells.iter().map(|cell| cell.seed).collect::<Vec<_>>();
    seeds.sort_unstable();
    seeds.dedup();
    seeds.into_iter().all(|seed| {
        let group = cells
            .iter()
            .filter(|cell| cell.seed == seed)
            .collect::<Vec<_>>();
        group.len() == Arm::ALL.len()
            && group.windows(2).all(|pair| {
                pair[0].boolean_trace_fingerprint == pair[1].boolean_trace_fingerprint
                    && pair[0].episode_fingerprint == pair[1].episode_fingerprint
                    && pair[0].learner_fingerprint == pair[1].learner_fingerprint
                    && pair[0].update_calls == pair[1].update_calls
                    && pair[0].divergent_patterns == pair[1].divergent_patterns
                    && pair[0].held_out_successes == pair[1].held_out_successes
            })
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub diagnostic_only: bool,
    pub m0_authoritative: bool,
    pub m1_exists: bool,
    pub source: SourceAudit,
    pub parallel_cells: usize,
    pub cells: Vec<CellReport>,
    pub functionally_sufficient_arms: Vec<Arm>,
    pub encoding_equivalent: bool,
    pub learned_prerequisite_authorized: bool,
    pub audit_passed: bool,
}

fn rejected() -> Report {
    Report {
        label: "DS-D1 definitive forbidden".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "DEFINITIVE-FORBIDDEN".to_string(),
        claim_eligible: false,
        diagnostic_only: true,
        m0_authoritative: true,
        m1_exists: false,
        source: source_audit(),
        parallel_cells: 0,
        cells: Vec::new(),
        functionally_sufficient_arms: Vec::new(),
        encoding_equivalent: false,
        learned_prerequisite_authorized: false,
        audit_passed: false,
    }
}

pub fn run(mode: HarnessMode) -> Report {
    if mode == HarnessMode::Definitive {
        return rejected();
    }
    let source = source_audit();
    let (seeds, acquisition, evaluation, mode_label) = match mode {
        HarnessMode::Micro => (vec![100], 16, 8, "MICRO"),
        HarnessMode::Gate => ((100..105).collect(), 32, 16, "GATE"),
        HarnessMode::Definitive => unreachable!(),
    };
    let parallel_cells = seeds.len() * Arm::ALL.len();
    let cells = parallel_map_ordered(parallel_cells, |index| {
        let seed = seeds[index / Arm::ALL.len()];
        let arm = Arm::ALL[index % Arm::ALL.len()];
        screen(seed, arm, acquisition, evaluation)
    });
    let functionally_sufficient_arms = Arm::ALL
        .into_iter()
        .filter(|arm| {
            cells
                .iter()
                .filter(|cell| cell.arm == *arm)
                .all(|cell| cell.held_out_role_recovery)
        })
        .collect::<Vec<_>>();
    let encoding_equivalent = equivalent_by_seed(&cells);
    let audit_passed = source.passed()
        && cells.len() == parallel_cells
        && cells.iter().all(|cell| cell.passed)
        && functionally_sufficient_arms.len() == Arm::ALL.len()
        && encoding_equivalent;
    Report {
        label: if audit_passed {
            "DS-D1 FUNCTIONAL DIAGNOSTIC COMPLETE".to_string()
        } else {
            "DS-D1 FUNCTIONAL DIAGNOSTIC FAILURE".to_string()
        },
        protocol: PROTOCOL.to_string(),
        mode: mode_label.to_string(),
        claim_eligible: false,
        diagnostic_only: true,
        m0_authoritative: true,
        m1_exists: false,
        source,
        parallel_cells,
        cells,
        functionally_sufficient_arms,
        encoding_equivalent,
        learned_prerequisite_authorized: false,
        audit_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_runs_three_independent_functional_arms() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.parallel_cells, 3);
        assert!(report.cells.iter().all(|cell| cell.diagnostic_only));
    }

    #[test]
    fn all_arms_update_diverge_and_recover_held_out_roles() {
        let report = run(HarnessMode::Gate);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.parallel_cells, 15);
        assert!(report.cells.iter().all(|cell| cell.update_fires
            && cell.strength_divergence
            && cell.held_out_role_recovery));
    }

    #[test]
    fn successful_arms_are_identical_at_frozen_ds1_boundary() {
        let report = run(HarnessMode::Gate);
        assert!(report.encoding_equivalent, "{report:#?}");
        assert_eq!(report.functionally_sufficient_arms, Arm::ALL);
    }

    #[test]
    fn no_diagnostic_property_is_persistent_or_claim_eligible() {
        let report = run(HarnessMode::Micro);
        assert!(report
            .cells
            .iter()
            .all(|cell| cell.persistent_property_bytes == 0
                && !cell.current_substrate_observability_established));
        assert!(
            !report.claim_eligible
                && report.diagnostic_only
                && !report.m1_exists
                && !report.learned_prerequisite_authorized
        );
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.audit_passed && report.cells.is_empty() && !report.m1_exists);
    }
}
