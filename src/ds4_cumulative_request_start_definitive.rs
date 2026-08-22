//! Write-once DS4 cumulative request/start definitive matrix.

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds4-cumulative-request-start-definitive-v1";
pub const EXACT_DEVELOPMENT_PARENT: &str = "3a82adc23fd179058f01d5004e894833f1cad0f4";
pub const AUTHORITATIVE_M3: &str = "ffcdfe8b36fc62348b7ebcb09aaf4797f6146ba8";
pub const DEFINITIVE_PROTOCOL_COMMIT: &str = "6af6374406de96a6d3132d2a9289cfa4ab15be6d";
pub const FROZEN_PARENT_SHA256: &str =
    "b65b28256d58c184b41bf2ff8d383c99593e6d812480751684209dce1d82f99a";
pub const FROZEN_RUNNER_SHA256: &str =
    "4f287d66486514dea70cca9fb701e730a8c9e603731fd8159af6ffa7ddfa6846";
pub const FROZEN_HANDOFF_SHA256: &str =
    "9cbc989c8544d1e94ec359197d085dbd1679ba28ed43b9d158b9655c11093235";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "158bfdfcf79ec3b8961ff908bbab1fdaf5f31c1c71f558990936d71f29ad4b38";
pub const FROZEN_RESULTS_TREE_SHA256: &str =
    "97b85f9056a8404fb2caf81e0fa8e3a1cb06398533874a474a9fe2c9696797a4";

const CELLS: usize = 16;
const FIRST_SEED: u64 = 4_000_000;
const CELL_STRIDE: u64 = 100_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlResult {
    pub number: usize,
    pub name: &'static str,
    pub passed: bool,
    pub diagnostic: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellResult {
    pub cell_id: usize,
    pub base_seed: u64,
    pub passed: bool,
    pub stages: [String; 6],
    pub first_collapse: String,
    pub competence_episode: usize,
    pub request_roles: usize,
    pub held_out_correct: usize,
    pub held_out_total: usize,
    pub explicit_answers: usize,
    pub queues_empty: usize,
    pub request_positions: usize,
    pub m3_learned_uses: usize,
    pub completion_activity: usize,
    pub selection_activations: usize,
    pub execution_activations: usize,
    pub update_activations: usize,
    pub m3_physical_work: u64,
    pub p4_nonplastic: bool,
    pub m3_nonplastic: bool,
    pub duplicate_deterministic: bool,
    pub controls: Vec<ControlResult>,
}

macro_rules! ds4_definitive_access {
    () => {
        pub(super) fn definitive_cell(cell_id: usize, base_seed: u64) -> super::CellResult {
            let mut first = development_snapshot(base_seed, 1, 32);
            let second = development_snapshot(base_seed, 1, 32);
            let duplicate_deterministic = first == second;
            if let Some(control) = first.controls.iter_mut().find(|row| row.number == 12) {
                control.passed &= duplicate_deterministic;
                control.diagnostic =
                    format!("{} duplicate={duplicate_deterministic}", control.diagnostic);
            }
            let source_ready = first.source.passed();
            let physical_path = first.m3_learned_uses > 0
                && first.completion_activity > 0
                && first.selection_activations > 0
                && first.execution_activations > 0
                && first.update_activations > 0;
            let request_ready = first.ready_learners == 1
                && first.single_role_learners == 1
                && first.competence_episodes.len() == 1
                && first.competence_episodes[0] <= 4_000;
            let functional_transfer = first.held_out_correct == 32
                && first.held_out_total == 32
                && first.explicit_answers == 32
                && first.queues_empty == 32
                && first.request_positions.len() == 6;
            let controls_ready =
                first.controls.len() == 12 && first.controls.iter().all(|control| control.passed);
            let lifecycle_ready = duplicate_deterministic
                && first.m3_physical_work > 0
                && first.p4_nonplastic
                && first.m3_nonplastic
                && first.acquisition_seeds.is_disjoint(&first.held_out_seeds);
            let ready = [
                source_ready,
                physical_path,
                request_ready,
                functional_transfer,
                controls_ready,
                lifecycle_ready,
            ];
            let first_collapse_stage = ready.iter().position(|value| !value);
            let stages = std::array::from_fn(|stage| match first_collapse_stage {
                None => "READY".to_string(),
                Some(collapse) if stage < collapse => "READY".to_string(),
                Some(collapse) if stage == collapse => format!("COLLAPSE: {}", STAGES[stage]),
                Some(_) => "BLOCKED".to_string(),
            });
            let first_collapse = first_collapse_stage
                .map(|stage| {
                    if stage == 4 {
                        first
                            .controls
                            .iter()
                            .find(|control| !control.passed)
                            .map(|control| {
                                format!("P4/control {} {}", control.number, control.name)
                            })
                            .unwrap_or_else(|| STAGES[stage].to_string())
                    } else {
                        STAGES[stage].to_string()
                    }
                })
                .unwrap_or_else(|| "NONE".to_string());
            super::CellResult {
                cell_id,
                base_seed,
                passed: first_collapse_stage.is_none(),
                stages,
                first_collapse,
                competence_episode: first
                    .competence_episodes
                    .first()
                    .copied()
                    .unwrap_or_default(),
                request_roles: first.single_role_learners,
                held_out_correct: first.held_out_correct,
                held_out_total: first.held_out_total,
                explicit_answers: first.explicit_answers,
                queues_empty: first.queues_empty,
                request_positions: first.request_positions.len(),
                m3_learned_uses: first.m3_learned_uses,
                completion_activity: first.completion_activity,
                selection_activations: first.selection_activations,
                execution_activations: first.execution_activations,
                update_activations: first.update_activations,
                m3_physical_work: first.m3_physical_work,
                p4_nonplastic: first.p4_nonplastic,
                m3_nonplastic: first.m3_nonplastic,
                duplicate_deterministic,
                controls: first
                    .controls
                    .into_iter()
                    .map(|control| super::ControlResult {
                        number: control.number,
                        name: control.name,
                        passed: control.passed,
                        diagnostic: control.diagnostic,
                    })
                    .collect(),
            }
        }

        pub(super) fn definitive_development_audit() -> bool {
            let report = run(HarnessMode::Micro);
            report.development_ready
                && !report.claim_eligible
                && report.m3_authoritative
                && !report.m4_exists
                && report.controls.len() == 12
                && report.controls.iter().all(|control| control.passed)
        }

        pub(super) fn definitive_frozen_source_ok() -> bool {
            source_audit().passed()
        }
    };
}

#[allow(dead_code)]
mod frozen_development {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds4_cumulative_request_start_port.rs"
    ));
    ds4_definitive_access!();
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub exact_development_parent: bool,
    pub authoritative_m3: bool,
    pub parent_hash: bool,
    pub runner_hash: bool,
    pub handoff_hash: bool,
    pub protocol_hash: bool,
    pub frozen_development_source: bool,
    pub results_tree_frozen: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.exact_development_parent
            && self.authoritative_m3
            && self.parent_hash
            && self.runner_hash
            && self.handoff_hash
            && self.protocol_hash
            && self.frozen_development_source
            && self.results_tree_frozen
    }
}

pub fn source_preflight(results_tree_frozen: bool) -> SourceAudit {
    SourceAudit {
        exact_development_parent: EXACT_DEVELOPMENT_PARENT
            == "3a82adc23fd179058f01d5004e894833f1cad0f4",
        authoritative_m3: AUTHORITATIVE_M3 == "ffcdfe8b36fc62348b7ebcb09aaf4797f6146ba8",
        parent_hash: env!("DS4_DEFINITIVE_PARENT_SHA256") == FROZEN_PARENT_SHA256,
        runner_hash: env!("DS4_DEFINITIVE_RUNNER_SHA256") == FROZEN_RUNNER_SHA256,
        handoff_hash: env!("DS4_DEFINITIVE_HANDOFF_SHA256") == FROZEN_HANDOFF_SHA256,
        protocol_hash: env!("DS4_DEFINITIVE_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        frozen_development_source: frozen_development::definitive_frozen_source_ok(),
        results_tree_frozen,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub mode: String,
    pub claim_eligible: bool,
    pub passed: bool,
    pub m3_authoritative: bool,
    pub m4_exists: bool,
    pub m4_authoritative: bool,
    pub ds5_cumulative_eligible: bool,
    pub source: SourceAudit,
    pub cells: Vec<CellResult>,
}

pub fn run_audit(results_tree_frozen: bool) -> Report {
    let source = source_preflight(results_tree_frozen);
    let development_ready = frozen_development::definitive_development_audit();
    Report {
        mode: "AUDIT-NON-CLAIM".to_string(),
        claim_eligible: false,
        passed: source.passed() && development_ready,
        m3_authoritative: true,
        m4_exists: false,
        m4_authoritative: false,
        ds5_cumulative_eligible: false,
        source,
        cells: Vec::new(),
    }
}

pub fn run_definitive(results_tree_frozen: bool) -> Report {
    let source = source_preflight(results_tree_frozen);
    let mut cells = Vec::with_capacity(CELLS);
    if source.passed() {
        for cell_id in 0..CELLS {
            let base_seed = FIRST_SEED + cell_id as u64 * CELL_STRIDE;
            cells.push(frozen_development::definitive_cell(cell_id, base_seed));
        }
    }
    let passed = source.passed()
        && cells.len() == CELLS
        && cells.iter().all(|cell| cell.passed)
        && cells
            .iter()
            .map(|cell| cell.held_out_correct)
            .sum::<usize>()
            == 512
        && cells
            .iter()
            .flat_map(|cell| &cell.controls)
            .filter(|control| control.passed)
            .count()
            == 192;
    Report {
        mode: "DEFINITIVE".to_string(),
        claim_eligible: true,
        passed,
        m3_authoritative: !passed,
        m4_exists: passed,
        m4_authoritative: passed,
        ds5_cumulative_eligible: passed,
        source,
        cells,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_uses_only_development_namespace() {
        let report = run_audit(true);
        assert!(report.passed, "{report:#?}");
        assert!(!report.claim_eligible);
        assert!(report.cells.is_empty());
        assert!(!report.m4_exists);
    }
}
