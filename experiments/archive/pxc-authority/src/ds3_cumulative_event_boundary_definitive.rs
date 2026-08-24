//! Write-once cumulative DS3 definitive matrix over the frozen development port.

pub const PROTOCOL: &str = "ds3-cumulative-event-boundary-definitive-v2";
pub const EXACT_DEVELOPMENT_PARENT: &str = "8e24a1316327f0af40fa3e7c70ad940d2a3e203f";
pub const AMENDED_PROTOCOL_COMMIT: &str = "0094a8fbebd085a8ea4709f841cb15e295553450";
pub const AUTHORITATIVE_M2: &str = "162a5b2082a8c1ac9ede45bc5178fecf3509b476";
pub const FROZEN_PARENT_SHA256: &str =
    "c4fc7aca11a5925effeb5a84b90184a70da0f66da7c063d0f87ba46ca36addf3";
pub const FROZEN_PARENT_HANDOFF_SHA256: &str =
    "75c8f734cd0b7c958b84e252be15a376541bfab7ef2b4df60029ce29f596b321";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "9abead91291169898024f809d8ab6e13b60545056f0f1980ec3daa80b216dd59";
pub const FROZEN_RESULTS_TREE_SHA256: &str =
    "b6dcf5ae5fd782b47f0121705f8b3406c2e00e60a5ec217772677818343a0848";

pub const DEFINITIVE_CELLS: usize = 16;
pub const ACQUISITION_STREAMS: usize = 8;
pub const HELD_OUT_STREAMS: usize = 16;
pub const CONTROLS_PER_CELL: usize = 12;
pub const BASE_SEED_START: u64 = 1_000_000;
pub const BASE_SEED_STRIDE: u64 = 100_000;

const STAGES: [&str; 6] = [
    "P0 exact parent, hashes, source, and matrix dimensions",
    "P1 legal learned M2 role/link wiring",
    "P2 held-out event-boundary reconstruction",
    "P3 ordinary-consequence parity",
    "P4 controls 1-12",
    "P5 duplicate determinism, cell isolation, and work attribution",
];

const CONTROL_NAMES: [&str; 12] = [
    "identical-local-shapes-different-grouping",
    "different-shapes-same-functional-span",
    "boundary-shifts",
    "interruptions-and-reentry",
    "shuffled-timing-relabeled-consequences",
    "fresh-identities-and-allocation",
    "leak-source-audit",
    "invalidation-generic-reopening-reacquisition",
    "subthreshold-recurrence",
    "missing-close",
    "invalid-causal-transition",
    "held-out-population",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlCell {
    pub number: usize,
    pub name: String,
    pub passed: bool,
    pub diagnostic: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ParentAudit {
    source_passed: bool,
    stages_ready: bool,
    controls_exact: bool,
    definitive_locked: bool,
    development_ready: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RawCell {
    cell_id: usize,
    base_seed: u64,
    stage_ready: [bool; 6],
    first_collapse: String,
    controls: Vec<ControlCell>,
    acquisition_m2_work: u64,
    acquisition_observations: u64,
    candidate_comparisons: u64,
    held_out_spans: usize,
    held_out_used_learned: usize,
    held_out_acquisition_observations: u64,
    generic_mature_work: u64,
    learned_mature_work: u64,
    chunk_count: usize,
    persistent_bytes: usize,
    passed: bool,
}

#[allow(dead_code)]
mod frozen_development {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds3_cumulative_event_boundary_port.rs"
    ));

    pub(super) fn parent_audit(expected_controls: &[&str]) -> super::ParentAudit {
        let gate = run(crate::research_runtime::HarnessMode::Gate);
        let locked = run(crate::research_runtime::HarnessMode::Definitive);
        super::ParentAudit {
            source_passed: gate.source.passed(),
            stages_ready: gate.stages.iter().all(|stage| stage == "READY"),
            controls_exact: gate.controls.len() == expected_controls.len()
                && gate.controls.iter().zip(expected_controls).enumerate().all(
                    |(index, (control, expected))| {
                        control.number == index + 1 && control.name == *expected && control.passed
                    },
                ),
            definitive_locked: locked.mode == "DEFINITIVE-FORBIDDEN"
                && !locked.claim_eligible
                && !locked.development_ready
                && locked.controls.is_empty(),
            development_ready: gate.development_ready
                && !gate.claim_eligible
                && gate.m2_authoritative
                && !gate.m3_exists,
        }
    }

    fn held_out_options(ordinal: usize) -> RenderOptions {
        match ordinal % 4 {
            0 => RenderOptions::default(),
            1 => RenderOptions {
                shape_xor: 0xA7,
                relabel: true,
                ..RenderOptions::default()
            },
            2 => RenderOptions {
                reverse_time: true,
                reverse_allocation: true,
                ..RenderOptions::default()
            },
            3 => RenderOptions {
                shape_xor: 0xA7,
                consequence_delta: 31,
                reverse_time: true,
                relabel: true,
                reverse_allocation: true,
                ..RenderOptions::default()
            },
            _ => unreachable!(),
        }
    }

    fn blocked_interruption_control(
        base_seed: u64,
        learner: &mut frozen_ds3::GlueBoundary,
    ) -> bool {
        let mut stream = Stream::default();
        let mut occurrences = base_seed ^ 0xD3F4_0000;
        let fixture =
            append_lifecycle(
                &mut stream,
                base_seed + 20_070,
                0,
                2,
                RenderOptions::default(),
                &mut occurrences,
            ) && append_interruption(&mut stream, base_seed + 20_071, 0, false, &mut occurrences)
                && append_lifecycle(
                    &mut stream,
                    base_seed + 20_072,
                    1,
                    3,
                    RenderOptions::default(),
                    &mut occurrences,
                );
        if !fixture {
            return false;
        }
        let evaluation = frozen_ds3::glue_evaluate(learner, &stream.observations, false);
        exact_reconstruction(&evaluation, &stream)
            && evaluation.spans.len() == 1
            && evaluation.invalidations > 0
    }

    pub(super) fn definitive_cell(
        cell_id: usize,
        base_seed: u64,
        source_ok: bool,
    ) -> super::RawCell {
        let mut learner = frozen_ds3::glue_default_boundary();
        let mut wiring_legal = true;
        let mut acquisition_m2_work = 0u64;
        let mut acquisition_observations = 0u64;
        let mut candidate_comparisons = 0u64;

        for ordinal in 0..super::ACQUISITION_STREAMS {
            let Some(stream) =
                standard_stream(base_seed + ordinal as u64, RenderOptions::default())
            else {
                wiring_legal = false;
                continue;
            };
            wiring_legal &=
                stream_legal(&stream) && stream.organic_rows == 6 && stream.expected.len() == 2;
            acquisition_m2_work += stream.m2_work;
            let evaluation = frozen_ds3::glue_evaluate(&mut learner, &stream.observations, true);
            acquisition_observations += evaluation.work.acquisition_observations;
            candidate_comparisons += evaluation.work.candidate_comparisons;
        }

        let chunks_after_acquisition = frozen_ds3::glue_chunk_count(&learner);
        let bytes_after_acquisition = frozen_ds3::glue_persistent_bytes(&learner);
        let mut reconstructability = true;
        let mut functional_adequacy = true;
        let mut held_out_spans = 0usize;
        let mut held_out_used_learned = 0usize;
        let mut held_out_acquisition_observations = 0u64;
        let mut generic_mature_work = 0u64;
        let mut learned_mature_work = 0u64;

        for ordinal in 0..super::HELD_OUT_STREAMS {
            let Some(stream) = standard_stream(
                base_seed + 10_000 + ordinal as u64,
                held_out_options(ordinal),
            ) else {
                wiring_legal = false;
                reconstructability = false;
                functional_adequacy = false;
                continue;
            };
            wiring_legal &=
                stream_legal(&stream) && stream.organic_rows == 6 && stream.expected.len() == 2;
            let evaluation = frozen_ds3::glue_evaluate(&mut learner, &stream.observations, false);
            reconstructability &= exact_reconstruction(&evaluation, &stream);
            functional_adequacy &= consequence_parity(&evaluation, &stream);
            held_out_spans += evaluation.spans.len();
            held_out_used_learned += evaluation.used_learned;
            held_out_acquisition_observations += evaluation.work.acquisition_observations;
            generic_mature_work += evaluation.work.generic_transition_checks
                + evaluation.work.completed_spans
                + evaluation.work.propagated_consequences;
            learned_mature_work += evaluation.work.learned_signature_checks;
        }

        let chunks_after_held_out = frozen_ds3::glue_chunk_count(&learner);
        let bytes_after_held_out = frozen_ds3::glue_persistent_bytes(&learner);

        // Reuse the frozen twelve-control development battery on this cell's
        // fresh namespace. Add the preregistered blocked-route arm to control 4.
        let control_probe = run_probe(
            base_seed,
            super::ACQUISITION_STREAMS,
            super::HELD_OUT_STREAMS,
        );
        let blocked = blocked_interruption_control(base_seed, &mut learner);
        let mut controls = control_probe
            .controls
            .into_iter()
            .map(|control| super::ControlCell {
                number: control.number,
                name: control.name.to_string(),
                passed: control.passed,
                diagnostic: control.diagnostic,
            })
            .collect::<Vec<_>>();
        if let Some(interruption) = controls.iter_mut().find(|control| control.number == 4) {
            interruption.passed &= blocked;
            interruption
                .diagnostic
                .push_str(&format!("; blocked_route={blocked}"));
        }

        let controls_exact = controls.len() == super::CONTROLS_PER_CELL
            && controls.iter().zip(super::CONTROL_NAMES).enumerate().all(
                |(index, (control, expected))| {
                    control.number == index + 1 && control.name == expected && control.passed
                },
            );
        let work_exact = acquisition_m2_work == 6_872
            && acquisition_observations == 48
            && candidate_comparisons == 16
            && held_out_spans == 32
            && held_out_used_learned == 32
            && held_out_acquisition_observations == 0
            && generic_mature_work == 256
            && learned_mature_work == 128
            && chunks_after_acquisition == 2
            && chunks_after_held_out == chunks_after_acquisition
            && bytes_after_acquisition == 20
            && bytes_after_held_out == bytes_after_acquisition;
        let stage_ready = [
            source_ok,
            wiring_legal && control_probe.wiring_legal,
            reconstructability,
            functional_adequacy,
            controls_exact,
            work_exact,
        ];
        let first_collapse = stage_ready
            .iter()
            .position(|ready| !ready)
            .map(|stage| {
                if stage == 4 {
                    controls
                        .iter()
                        .find(|control| !control.passed)
                        .map(|control| format!("P4/control {} {}", control.number, control.name))
                        .unwrap_or_else(|| super::STAGES[stage].to_string())
                } else {
                    super::STAGES[stage].to_string()
                }
            })
            .unwrap_or_else(|| "NONE".to_string());
        let passed = stage_ready.iter().all(|ready| *ready);
        super::RawCell {
            cell_id,
            base_seed,
            stage_ready,
            first_collapse,
            controls,
            acquisition_m2_work,
            acquisition_observations,
            candidate_comparisons,
            held_out_spans,
            held_out_used_learned,
            held_out_acquisition_observations,
            generic_mature_work,
            learned_mature_work,
            chunk_count: chunks_after_held_out,
            persistent_bytes: bytes_after_held_out,
            passed,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub parent_hash: bool,
    pub parent_handoff_hash: bool,
    pub amended_protocol_hash: bool,
    pub exact_parent: bool,
    pub exact_protocol_commit: bool,
    pub exact_authoritative_m2: bool,
    pub exact_matrix: bool,
    pub results_tree_digest: bool,
    pub development_source_passed: bool,
    pub development_stages_ready: bool,
    pub development_controls_exact: bool,
    pub development_definitive_locked: bool,
    pub development_ready: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.parent_hash
            && self.parent_handoff_hash
            && self.amended_protocol_hash
            && self.exact_parent
            && self.exact_protocol_commit
            && self.exact_authoritative_m2
            && self.exact_matrix
            && self.results_tree_digest
            && self.development_source_passed
            && self.development_stages_ready
            && self.development_controls_exact
            && self.development_definitive_locked
            && self.development_ready
    }
}

fn source_audit(results_tree_digest: bool) -> SourceAudit {
    let parent = frozen_development::parent_audit(&CONTROL_NAMES);
    SourceAudit {
        parent_hash: env!("DS3_DEFINITIVE_PARENT_SHA256") == FROZEN_PARENT_SHA256,
        parent_handoff_hash: env!("DS3_DEFINITIVE_PARENT_HANDOFF_SHA256")
            == FROZEN_PARENT_HANDOFF_SHA256,
        amended_protocol_hash: env!("DS3_DEFINITIVE_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        exact_parent: EXACT_DEVELOPMENT_PARENT == "8e24a1316327f0af40fa3e7c70ad940d2a3e203f",
        exact_protocol_commit: AMENDED_PROTOCOL_COMMIT
            == "0094a8fbebd085a8ea4709f841cb15e295553450",
        exact_authoritative_m2: AUTHORITATIVE_M2 == "162a5b2082a8c1ac9ede45bc5178fecf3509b476",
        exact_matrix: DEFINITIVE_CELLS == 16
            && ACQUISITION_STREAMS == 8
            && HELD_OUT_STREAMS == 16
            && CONTROLS_PER_CELL == 12
            && BASE_SEED_START == 1_000_000
            && BASE_SEED_STRIDE == 100_000,
        results_tree_digest,
        development_source_passed: parent.source_passed,
        development_stages_ready: parent.stages_ready,
        development_controls_exact: parent.controls_exact,
        development_definitive_locked: parent.definitive_locked,
        development_ready: parent.development_ready,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub cell_id: usize,
    pub base_seed: u64,
    pub stage_ready: [bool; 6],
    pub first_collapse: String,
    pub controls: Vec<ControlCell>,
    pub acquisition_m2_work: u64,
    pub acquisition_observations: u64,
    pub candidate_comparisons: u64,
    pub held_out_spans: usize,
    pub held_out_used_learned: usize,
    pub held_out_acquisition_observations: u64,
    pub generic_mature_work: u64,
    pub learned_mature_work: u64,
    pub chunk_count: usize,
    pub persistent_bytes: usize,
    pub duplicate_deterministic: bool,
    pub passed: bool,
}

fn evaluate_cell(cell_id: usize, base_seed: u64, source: &SourceAudit) -> Cell {
    let first = frozen_development::definitive_cell(cell_id, base_seed, source.passed());
    let second = frozen_development::definitive_cell(cell_id, base_seed, source.passed());
    let duplicate_deterministic = first == second;
    let first_collapse = if duplicate_deterministic {
        first.first_collapse.clone()
    } else {
        STAGES[5].to_string()
    };
    Cell {
        cell_id: first.cell_id,
        base_seed: first.base_seed,
        stage_ready: first.stage_ready,
        first_collapse,
        controls: first.controls,
        acquisition_m2_work: first.acquisition_m2_work,
        acquisition_observations: first.acquisition_observations,
        candidate_comparisons: first.candidate_comparisons,
        held_out_spans: first.held_out_spans,
        held_out_used_learned: first.held_out_used_learned,
        held_out_acquisition_observations: first.held_out_acquisition_observations,
        generic_mature_work: first.generic_mature_work,
        learned_mature_work: first.learned_mature_work,
        chunk_count: first.chunk_count,
        persistent_bytes: first.persistent_bytes,
        duplicate_deterministic,
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
    pub m2_authoritative: bool,
    pub m3_exists: bool,
    pub m3_authoritative: bool,
    pub ds4_cumulative_eligible: bool,
}

fn run_matrix(
    mode: &str,
    claim_eligible: bool,
    seeds: &[(usize, u64)],
    results_tree_digest: bool,
) -> Report {
    let source = source_audit(results_tree_digest);
    let cells = seeds
        .iter()
        .map(|(cell_id, seed)| evaluate_cell(*cell_id, *seed, &source))
        .collect::<Vec<_>>();
    let passed = source.passed()
        && !cells.is_empty()
        && cells.iter().all(|cell| cell.passed)
        && (!claim_eligible || cells.len() == DEFINITIVE_CELLS);
    Report {
        mode: mode.to_string(),
        claim_eligible,
        source,
        cells,
        passed,
        m2_authoritative: !claim_eligible || !passed,
        m3_exists: claim_eligible && passed,
        m3_authoritative: claim_eligible && passed,
        ds4_cumulative_eligible: claim_eligible && passed,
    }
}

pub fn run_audit(results_tree_digest: bool) -> Report {
    run_matrix("AUDIT", false, &[(0, 83_000)], results_tree_digest)
}

pub fn run_definitive(results_tree_digest: bool) -> Report {
    let seeds = (0..DEFINITIVE_CELLS)
        .map(|cell_id| (cell_id, BASE_SEED_START + cell_id as u64 * BASE_SEED_STRIDE))
        .collect::<Vec<_>>();
    run_matrix("DEFINITIVE", true, &seeds, results_tree_digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_dimensions_and_fresh_namespaces_are_exact() {
        let seeds = (0..DEFINITIVE_CELLS)
            .map(|cell_id| BASE_SEED_START + cell_id as u64 * BASE_SEED_STRIDE)
            .collect::<Vec<_>>();
        assert_eq!(seeds.len(), 16);
        assert_eq!(seeds[0], 1_000_000);
        assert_eq!(seeds[15], 2_500_000);
        assert!(seeds.windows(2).all(|pair| pair[1] - pair[0] == 100_000));
        assert!(!seeds.contains(&83_000) && !seeds.contains(&84_000));
    }

    #[test]
    fn audit_is_non_claim_and_uses_only_development_seed() {
        let report = run_audit(true);
        assert!(report.passed, "{report:#?}");
        assert!(!report.claim_eligible);
        assert_eq!(report.cells.len(), 1);
        assert_eq!(report.cells[0].base_seed, 83_000);
        assert!(report.m2_authoritative && !report.m3_exists);
        assert!(!report.ds4_cumulative_eligible);
    }

    #[test]
    fn results_tree_digest_is_frozen_for_external_preflight() {
        assert_eq!(
            FROZEN_RESULTS_TREE_SHA256,
            "b6dcf5ae5fd782b47f0121705f8b3406c2e00e60a5ec217772677818343a0848"
        );
    }
}
