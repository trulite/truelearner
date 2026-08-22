//! Write-once cumulative DS2 definitive matrix over the frozen development ancestor.

use crate::development;

pub const PROTOCOL: &str = "ds2-cumulative-causal-direction-definitive-v1";
pub const EXACT_PARENT: &str = "4cb6d65e615c674e61f2cd5340c37fa12b2e1ed8";
pub const PROTOCOL_COMMIT: &str = "19c036542785f5f2ff90c34488fb2160e4f5fe8d";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_PARENT_SHA256: &str =
    "d426e0553b9e106ab03c4e02c0f332df40392fcd491c2f4de036699d812a5559";
pub const FROZEN_PARENT_HANDOFF_SHA256: &str =
    "399b44dbd928f17880440e7d3110af599419edada2230f163d15277bdffaca18";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "e584d5118017ef51511c4523f30805400f853781796f78d1ea34cdb85b3ef60b";

pub const DEFINITIVE_SEEDS: usize = 16;
pub const CONTEXTS_PER_SEED: usize = 4;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Probe {
    seed: u64,
    contexts: usize,
    changed_lifecycles: usize,
    compatible_uses: usize,
    compatible_preservations: usize,
    invalidations: usize,
    reopenings: usize,
    reopened_executions: usize,
    historical_returns: usize,
    ambiguous_preservations: usize,
    layout_transfers: usize,
    persistent_bytes: usize,
    passed: bool,
}

macro_rules! definitive_ir0_access {
    () => {
        pub(super) fn definitive_probe(seed: u64) -> super::Probe {
            let source = source_audit();
            let audit = audit_seed(seed, &source);
            super::Probe {
                seed: audit.seed,
                contexts: audit.contexts,
                changed_lifecycles: audit.changed_lifecycles,
                compatible_uses: audit.compatible_uses,
                compatible_preservations: audit.compatible_preservations,
                invalidations: audit.invalidations,
                reopenings: audit.reopenings,
                reopened_executions: audit.reopened_executions,
                historical_returns: audit.historical_returns,
                ambiguous_preservations: audit.ambiguous_preservations,
                layout_transfers: audit.layout_transfers,
                persistent_bytes: audit.persistent_bytes,
                passed: audit.passed,
            }
        }

        pub(super) fn definitive_parent_audit() -> bool {
            source_audit().passed()
                && EXACT_PARENT == "d97f5038e6133a0abe4b24ea3b8eb5b4ba7cd4f4"
                && AUTHORITATIVE_M1 == super::AUTHORITATIVE_M1
        }
    };
}

#[allow(dead_code)]
mod frozen_ir0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_ir0_dependency_invalidation_reopening.rs"
    ));
    definitive_ir0_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub parent_hash: bool,
    pub parent_handoff_hash: bool,
    pub protocol_hash: bool,
    pub development_complete: bool,
    pub ir0_parent_audit: bool,
    pub exact_parent: bool,
    pub exact_matrix: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.parent_hash
            && self.parent_handoff_hash
            && self.protocol_hash
            && self.development_complete
            && self.ir0_parent_audit
            && self.exact_parent
            && self.exact_matrix
    }
}

fn source_audit() -> SourceAudit {
    let development = development::run();
    SourceAudit {
        parent_hash: env!("DS2_DEFINITIVE_PARENT_SHA256") == FROZEN_PARENT_SHA256,
        parent_handoff_hash: env!("DS2_DEFINITIVE_PARENT_HANDOFF_SHA256")
            == FROZEN_PARENT_HANDOFF_SHA256,
        protocol_hash: env!("DS2_DEFINITIVE_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        development_complete: development.audit_passed
            && development.development_complete
            && development.first_collapse_stage.is_none()
            && development.stages.iter().all(|stage| stage == "READY")
            && development.m1_authoritative
            && !development.m2_exists
            && !development.claim_eligible,
        ir0_parent_audit: frozen_ir0::definitive_parent_audit(),
        exact_parent: development::PROTOCOL == "ds2-after-ir0-mechanistic-retry-v1"
            && development::AUTHORITATIVE_M1 == AUTHORITATIVE_M1,
        exact_matrix: DEFINITIVE_SEEDS == 16 && CONTEXTS_PER_SEED == 4,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RawCell {
    probe: Probe,
    stage_ready: [bool; 10],
    first_collapse: String,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub seed: u64,
    pub contexts: usize,
    pub changed_lifecycles: usize,
    pub compatible_uses: usize,
    pub compatible_preservations: usize,
    pub invalidations: usize,
    pub reopenings: usize,
    pub reopened_executions: usize,
    pub historical_returns: usize,
    pub ambiguous_preservations: usize,
    pub layout_transfers: usize,
    pub persistent_bytes: usize,
    pub duplicate_deterministic: bool,
    pub first_collapse: String,
    pub passed: bool,
}

const STAGES: [&str; 10] = [
    "source and exact frozen-development audit",
    "retained direction executes on fresh occurrences",
    "changed physical dependency produces structural mismatch",
    "stale temporary route invalidates and abstains",
    "ordinary generic A1 inference reopens",
    "changed retained route installs and executes",
    "historical direction returns without reacquisition",
    "compatible and ambiguous contexts preserve retained history",
    "allocation layout and opaque-handle permutation transfer",
    "persistent state retains no episode identity or concrete destination",
];

fn raw_cell(seed: u64, source: &SourceAudit) -> RawCell {
    let probe = frozen_ir0::definitive_probe(seed);
    let stage_ready = [
        source.passed() && probe.passed,
        probe.compatible_uses == CONTEXTS_PER_SEED,
        probe.changed_lifecycles == CONTEXTS_PER_SEED,
        probe.invalidations == CONTEXTS_PER_SEED,
        probe.reopenings == CONTEXTS_PER_SEED,
        probe.reopened_executions == CONTEXTS_PER_SEED,
        probe.historical_returns == CONTEXTS_PER_SEED,
        probe.compatible_preservations == CONTEXTS_PER_SEED
            && probe.ambiguous_preservations == CONTEXTS_PER_SEED,
        probe.layout_transfers == CONTEXTS_PER_SEED,
        probe.persistent_bytes == 8,
    ];
    let first_collapse = stage_ready
        .iter()
        .position(|ready| !ready)
        .map(|stage| STAGES[stage].to_string())
        .unwrap_or_else(|| "NONE".to_string());
    let passed = stage_ready.iter().all(|ready| *ready);
    RawCell {
        probe,
        stage_ready,
        first_collapse,
        passed,
    }
}

fn evaluate_cell(seed: u64, source: &SourceAudit) -> Cell {
    let first = raw_cell(seed, source);
    let second = raw_cell(seed, source);
    let duplicate_deterministic = first == second;
    Cell {
        seed,
        contexts: first.probe.contexts,
        changed_lifecycles: first.probe.changed_lifecycles,
        compatible_uses: first.probe.compatible_uses,
        compatible_preservations: first.probe.compatible_preservations,
        invalidations: first.probe.invalidations,
        reopenings: first.probe.reopenings,
        reopened_executions: first.probe.reopened_executions,
        historical_returns: first.probe.historical_returns,
        ambiguous_preservations: first.probe.ambiguous_preservations,
        layout_transfers: first.probe.layout_transfers,
        persistent_bytes: first.probe.persistent_bytes,
        duplicate_deterministic,
        first_collapse: if duplicate_deterministic {
            first.first_collapse
        } else {
            "duplicate deterministic replay".to_string()
        },
        passed: first.passed && duplicate_deterministic,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub mode: String,
    pub claim_eligible: bool,
    pub source: SourceAudit,
    pub cells: Vec<Cell>,
    pub passed: bool,
    pub m1_authoritative: bool,
    pub m2_exists: bool,
    pub m2_authoritative: bool,
}

fn run_matrix(mode: &str, claim_eligible: bool, seeds: &[u64]) -> Report {
    let source = source_audit();
    let cells = seeds
        .iter()
        .map(|seed| evaluate_cell(*seed, &source))
        .collect::<Vec<_>>();
    let passed = source.passed() && !cells.is_empty() && cells.iter().all(|cell| cell.passed);
    Report {
        mode: mode.to_string(),
        claim_eligible,
        source,
        cells,
        passed,
        m1_authoritative: !claim_eligible || !passed,
        m2_exists: claim_eligible && passed,
        m2_authoritative: claim_eligible && passed,
    }
}

pub fn run_audit() -> Report {
    run_matrix("AUDIT", false, &[100])
}

pub fn run_definitive() -> Report {
    let seeds = (0..DEFINITIVE_SEEDS as u64).collect::<Vec<_>>();
    run_matrix("DEFINITIVE", true, &seeds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_uses_exact_parent_without_claim_authority() {
        let report = run_audit();
        assert!(report.passed, "{report:#?}");
        assert!(!report.claim_eligible);
        assert!(report.m1_authoritative);
        assert!(!report.m2_exists && !report.m2_authoritative);
        assert_eq!(report.cells.len(), 1);
        assert!(report.cells[0].duplicate_deterministic);
    }

    #[test]
    fn source_and_matrix_are_frozen() {
        assert!(source_audit().passed());
        assert_eq!((DEFINITIVE_SEEDS, CONTEXTS_PER_SEED), (16, 4));
    }
}
