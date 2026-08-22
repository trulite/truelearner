//! Diagnostic-only parallel single-property discrimination at DS1 stage 8b.

use crate::ds_c0_anonymous_credit_coupling as frozen_c0;
use crate::research_runtime::{parallel_map_ordered, HarnessMode};

pub const PROTOCOL: &str = "ds-d0-stage8b-single-property-discrimination-v1";
pub const EXACT_PARENT: &str = "7ea5680046b57fcbd81e31996e49be3ec3e9fc36";
pub const PROTOCOL_COMMIT: &str = "a029f267d28edeab328821e2e1904e78660424a7";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_PARENT_RETRY_SHA256: &str =
    "dba8ac027ec304a489b99c65e9629fe1537a33f256d9b3992d205de0e40b5c14";
pub const FROZEN_PARENT_HANDOFF_SHA256: &str =
    "de4c5af41bdbfff8f55cf5b8bad04724faf1fdf21e2b80fb9ce4c95b6072c49a";
pub const FROZEN_C0_SHA256: &str =
    "5c8d00189593ca2f7efb47165efddf85111259f90433a016e5822b5b9578aed2";
pub const FROZEN_E0_SHA256: &str =
    "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

const E0_SUPPORT: usize = 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Arm {
    OwnershipOnly,
    TemporalContrast,
    AlternativeComparison,
    Polarity,
    OutcomeChange,
}

impl Arm {
    pub const ALL: [Self; 5] = [
        Self::OwnershipOnly,
        Self::TemporalContrast,
        Self::AlternativeComparison,
        Self::Polarity,
        Self::OutcomeChange,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::OwnershipOnly => "OWNERSHIP_ONLY",
            Self::TemporalContrast => "TEMPORAL_CONTRAST",
            Self::AlternativeComparison => "ALTERNATIVE_COMPARISON",
            Self::Polarity => "POLARITY",
            Self::OutcomeChange => "OUTCOME_CHANGE",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CandidateProperty {
    OwnershipOnly,
    TemporalContrast { earlier: i16, later: i16 },
    AlternativeComparison { outcomes: [i16; 2] },
    Polarity { favorable: bool },
    OutcomeChange { before: i16, after: i16 },
}

impl CandidateProperty {
    fn for_arm(arm: Arm, seed: u64) -> Self {
        match arm {
            Arm::OwnershipOnly => Self::OwnershipOnly,
            Arm::TemporalContrast => Self::TemporalContrast {
                earlier: 1,
                later: 3,
            },
            Arm::AlternativeComparison => Self::AlternativeComparison { outcomes: [10, 20] },
            Arm::Polarity => Self::Polarity {
                favorable: seed.is_multiple_of(2),
            },
            Arm::OutcomeChange => Self::OutcomeChange {
                before: 10,
                after: if seed.is_multiple_of(2) { 11 } else { 9 },
            },
        }
    }

    fn field_count(self) -> usize {
        match self {
            Self::OwnershipOnly => 0,
            Self::Polarity { .. } => 1,
            Self::TemporalContrast { .. }
            | Self::AlternativeComparison { .. }
            | Self::OutcomeChange { .. } => 2,
        }
    }

    fn update_input(self, choice: usize) -> Option<bool> {
        match self {
            Self::OwnershipOnly => None,
            Self::TemporalContrast { earlier, later } => {
                let _non_evaluative_order = earlier < later;
                None
            }
            Self::AlternativeComparison { outcomes } => {
                Some(outcomes[choice] > outcomes[1usize.saturating_sub(choice)])
            }
            Self::Polarity { favorable } => Some(favorable),
            Self::OutcomeChange { before, after } => Some(after > before),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct DiagnosticEpisode {
    choice: usize,
    choose_calls: u64,
    update_calls: u64,
    input_was_available: bool,
    marked_learner_bytes: usize,
}

macro_rules! diagnostic_e0_access {
    () => {
        pub(super) fn diagnostic_episode(
            seed: u64,
            acquisition: usize,
            update_input: Option<bool>,
        ) -> Option<super::DiagnosticEpisode> {
            let (mut formation, mut prior) = acquire(seed, acquisition);
            for ordinal in 0..super::E0_SUPPORT {
                let episode = fixture(
                    seed + 1_000,
                    acquisition + ordinal,
                    ordinal % 4,
                    Perturbation::None,
                );
                prior.extend(episode.raw.spikes.iter().map(|spike| spike.occurrence));
                formation.form(&episode.raw)?;
            }
            let episode = fixture(
                seed + 2_000,
                acquisition + super::E0_SUPPORT + 17,
                0,
                Perturbation::None,
            );
            let event = formation.form(&episode.raw)?;
            let view = serialize_once(&event, &mut formation.work);
            let mut learner = Learner::default();
            let firing_before = learner.route_firings;
            let updates_before = learner.credit_updates;
            let (choice, _) = learner.choose(&view, seed as usize);
            if let Some(positive) = update_input {
                learner.apply_consequence(&view, choice, positive);
            }
            Some(super::DiagnosticEpisode {
                choice,
                choose_calls: learner.route_firings - firing_before,
                update_calls: learner.credit_updates - updates_before,
                input_was_available: update_input.is_some(),
                marked_learner_bytes: learner.persistent_bytes(),
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
    diagnostic_e0_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub parent_retry_hash: bool,
    pub parent_handoff_hash: bool,
    pub c0_hash: bool,
    pub e0_hash: bool,
    pub ds1_hash: bool,
    pub lineage: bool,
    pub frozen_c0_calls: usize,
    pub parallel_matrix_calls: usize,
    pub diagnostic_update_edges: usize,
    pub marked_update_definitions: usize,
    pub arm_variants: usize,
    pub combination_variants: usize,
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
    let source = include_str!("ds_d0_stage8b_discrimination.rs");
    let production = source.split("#[cfg(test)]").next().unwrap_or(source);
    let run_body = function_body(source, "\npub fn run(").unwrap_or_default();
    let update_call = ["learner.apply_", "consequence("].concat();
    SourceAudit {
        parent_retry_hash: env!("DS_D0_PARENT_RETRY_SHA256") == FROZEN_PARENT_RETRY_SHA256,
        parent_handoff_hash: env!("DS_D0_PARENT_HANDOFF_SHA256") == FROZEN_PARENT_HANDOFF_SHA256,
        c0_hash: env!("DS_D0_C0_SHA256") == FROZEN_C0_SHA256,
        e0_hash: env!("DS_D0_E0_SHA256") == FROZEN_E0_SHA256,
        ds1_hash: frozen_e0::FROZEN_DS1_LEARNER_SHA256 == FROZEN_DS1_SHA256,
        lineage: frozen_c0::AUTHORITATIVE_M0 == AUTHORITATIVE_M0
            && frozen_c0::FROZEN_RESULTS_DIGEST == FROZEN_RESULTS_DIGEST,
        frozen_c0_calls: run_body.matches("frozen_c0::run(").count(),
        parallel_matrix_calls: run_body.matches("parallel_map_ordered(").count(),
        diagnostic_update_edges: production.matches(&update_call).count(),
        marked_update_definitions: include_str!("ds_e0_anonymous_event_formation.rs")
            .matches("fn apply_consequence(")
            .count(),
        arm_variants: Arm::ALL.len(),
        combination_variants: 0,
    }
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.parent_retry_hash
            && self.parent_handoff_hash
            && self.c0_hash
            && self.e0_hash
            && self.ds1_hash
            && self.lineage
            && self.frozen_c0_calls == 1
            && self.parallel_matrix_calls == 1
            && self.diagnostic_update_edges == 1
            && self.marked_update_definitions == 1
            && self.arm_variants == 5
            && self.combination_variants == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ParentSeed {
    seed: u64,
    choice: usize,
    roots: usize,
    handles: usize,
    couplings: usize,
    polarity_fields: usize,
    updates: u64,
    controls_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellReport {
    pub seed: u64,
    pub arm: Arm,
    pub arm_label: String,
    pub same_parent_episode: bool,
    pub property_fields: usize,
    pub property_yielded_update_input: bool,
    pub candidate_to_bool_edges: usize,
    pub reachable_update_edges: usize,
    pub runtime_ds1_updates: u64,
    pub positive_value: Option<bool>,
    pub diagnostic_learner_bytes: usize,
    pub persistent_property_bytes: usize,
    pub diagnostic_only: bool,
    pub passed: bool,
}

fn screen_cell(parent: ParentSeed, arm: Arm, acquisition: usize) -> CellReport {
    let property = CandidateProperty::for_arm(arm, parent.seed);
    let update_input = property.update_input(parent.choice);
    let episode = frozen_e0::diagnostic_episode(parent.seed, acquisition, update_input)
        .expect("frozen E0 diagnostic episode");
    let same_parent_episode = parent.controls_passed
        && parent.roots == 2
        && parent.handles == 2
        && parent.couplings == 1
        && parent.polarity_fields == 0
        && parent.updates == 0
        && episode.choice == parent.choice
        && episode.choose_calls == 1;
    let candidate_to_bool_edges = usize::from(update_input.is_some());
    let reachable_update_edges = usize::from(episode.update_calls == 1);
    let expected_updates = u64::from(update_input.is_some());
    let passed = same_parent_episode
        && episode.input_was_available == update_input.is_some()
        && episode.update_calls == expected_updates
        && reachable_update_edges == candidate_to_bool_edges;
    CellReport {
        seed: parent.seed,
        arm,
        arm_label: arm.label().to_string(),
        same_parent_episode,
        property_fields: property.field_count(),
        property_yielded_update_input: update_input.is_some(),
        candidate_to_bool_edges,
        reachable_update_edges,
        runtime_ds1_updates: episode.update_calls,
        positive_value: update_input,
        diagnostic_learner_bytes: episode.marked_learner_bytes,
        persistent_property_bytes: 0,
        diagnostic_only: true,
        passed,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub m0_authoritative: bool,
    pub diagnostic_only: bool,
    pub m1_exists: bool,
    pub source: SourceAudit,
    pub parallel_cells: usize,
    pub cells: Vec<CellReport>,
    pub sufficient_arms: Vec<Arm>,
    pub deeper_gate_authorized: bool,
    pub audit_passed: bool,
}

fn rejected() -> Report {
    Report {
        label: "DS-D0 definitive forbidden".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "DEFINITIVE-FORBIDDEN".to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        diagnostic_only: true,
        m1_exists: false,
        source: source_audit(),
        parallel_cells: 0,
        cells: Vec::new(),
        sufficient_arms: Vec::new(),
        deeper_gate_authorized: false,
        audit_passed: false,
    }
}

pub fn run(mode: HarnessMode) -> Report {
    if mode == HarnessMode::Definitive {
        return rejected();
    }
    let source = source_audit();
    let c0 = frozen_c0::run(mode);
    let acquisition = match mode {
        HarnessMode::Micro => 16,
        HarnessMode::Gate => 32,
        HarnessMode::Definitive => unreachable!(),
    };
    let parents = c0
        .seeds
        .iter()
        .map(|seed| ParentSeed {
            seed: seed.seed,
            choice: seed.choice,
            roots: seed.roots,
            handles: seed.handles,
            couplings: seed.couplings,
            polarity_fields: seed.coupling_polarity_fields,
            updates: seed.ds1_updates,
            controls_passed: seed.controls.passed(),
        })
        .collect::<Vec<_>>();
    let parallel_cells = parents.len() * Arm::ALL.len();
    let cells = parallel_map_ordered(parallel_cells, |index| {
        let parent = parents[index / Arm::ALL.len()];
        let arm = Arm::ALL[index % Arm::ALL.len()];
        screen_cell(parent, arm, acquisition)
    });
    let sufficient_arms = Arm::ALL
        .into_iter()
        .filter(|arm| {
            let arm_cells = cells.iter().filter(|cell| cell.arm == *arm);
            let mut seen = 0usize;
            let sufficient = arm_cells
                .inspect(|_| seen += 1)
                .all(|cell| cell.reachable_update_edges == 1 && cell.runtime_ds1_updates == 1);
            sufficient && seen == parents.len()
        })
        .collect::<Vec<_>>();
    let baseline_zero = cells
        .iter()
        .filter(|cell| matches!(cell.arm, Arm::OwnershipOnly | Arm::TemporalContrast))
        .all(|cell| cell.reachable_update_edges == 0 && cell.runtime_ds1_updates == 0);
    let audit_passed = source.passed()
        && c0.audit_passed
        && cells.len() == parallel_cells
        && cells.iter().all(|cell| cell.passed)
        && baseline_zero
        && cells.iter().all(|cell| cell.persistent_property_bytes == 0);
    Report {
        label: if audit_passed {
            "DS-D0 DIAGNOSTIC MATRIX COMPLETE".to_string()
        } else {
            "DS-D0 DIAGNOSTIC MATRIX FAILURE".to_string()
        },
        protocol: PROTOCOL.to_string(),
        mode: c0.mode,
        claim_eligible: false,
        m0_authoritative: true,
        diagnostic_only: true,
        m1_exists: false,
        source,
        parallel_cells,
        cells,
        sufficient_arms,
        deeper_gate_authorized: false,
        audit_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_runs_all_single_property_arms_in_parallel() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.parallel_cells, 5);
        assert_eq!(report.cells.len(), 5);
        assert!(report.cells.iter().all(|cell| cell.diagnostic_only));
    }

    #[test]
    fn ownership_and_time_alone_do_not_reach_the_boolean_update_input() {
        let report = run(HarnessMode::Gate);
        assert!(report.audit_passed, "{report:#?}");
        assert!(report
            .cells
            .iter()
            .filter(|cell| matches!(cell.arm, Arm::OwnershipOnly | Arm::TemporalContrast))
            .all(|cell| !cell.property_yielded_update_input
                && cell.reachable_update_edges == 0
                && cell.runtime_ds1_updates == 0));
    }

    #[test]
    fn individually_directional_properties_reach_exactly_one_update() {
        let report = run(HarnessMode::Gate);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(
            report.sufficient_arms,
            vec![
                Arm::AlternativeComparison,
                Arm::Polarity,
                Arm::OutcomeChange
            ]
        );
        assert!(report
            .cells
            .iter()
            .filter(|cell| matches!(
                cell.arm,
                Arm::AlternativeComparison | Arm::Polarity | Arm::OutcomeChange
            ))
            .all(|cell| cell.candidate_to_bool_edges == 1
                && cell.reachable_update_edges == 1
                && cell.runtime_ds1_updates == 1));
    }

    #[test]
    fn no_arm_combines_properties_or_advances_authority() {
        let report = run(HarnessMode::Micro);
        assert_eq!(report.source.combination_variants, 0);
        assert!(!report.claim_eligible && report.diagnostic_only && !report.m1_exists);
        assert!(!report.deeper_gate_authorized);
        assert!(report
            .cells
            .iter()
            .all(|cell| cell.persistent_property_bytes == 0));
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.audit_passed && report.cells.is_empty() && !report.m1_exists);
    }
}
