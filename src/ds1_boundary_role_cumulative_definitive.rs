//! Write-once cumulative DS1 definitive matrix over the exact frozen M1 development ancestor.

pub const PROTOCOL: &str = "ds1-boundary-role-cumulative-definitive-v1";
pub const EXACT_PARENT: &str = "302168072f78161f520d531f8b5f3ab0150df8d0";
pub const PROTOCOL_COMMIT: &str = "4f22b8a4dba4c42942d36543f23691c7e8103dc2";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_PARENT_SHA256: &str =
    "2b35d8b181b1b477390a2f84a4ad01993d7ca2b2aec6291d16ffd4fc0faf50b0";
pub const FROZEN_PARENT_HANDOFF_SHA256: &str =
    "bd61304825259bb1951bbd355d6f8db1574b63b3d248dc93df2a917ef319e3d2";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "10b31883051af427494d073a2093f416e856ed1696198ec9e35a6118b30d741d";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

pub const DEFINITIVE_SEEDS: usize = 8;
pub const DEFINITIVE_ACQUISITION: usize = 64;
pub const DEFINITIVE_HELD_OUT: usize = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Probe {
    acquisition: usize,
    events: usize,
    two_roots: usize,
    directions: usize,
    physical_directions: usize,
    deliveries: usize,
    updates: u64,
    patterns: usize,
    divergent_patterns: usize,
    consequence_mature: usize,
    evaluator_mature: usize,
    held_out_attempts: usize,
    held_out_successes: usize,
    held_out_abstentions: usize,
    d3_work: u64,
    learner_work: u64,
    learner_bytes: usize,
    fingerprint: u64,
    evaluator_used_in_acquisition: bool,
}

macro_rules! definitive_parent_access {
    () => {
        pub(super) fn definitive_probe(
            seed: u64,
            acquisition: usize,
            held_out: usize,
            reverse_world: bool,
        ) -> Option<super::Probe> {
            let probe = frozen_e0::composition_probe(seed, acquisition, held_out, reverse_world)?;
            Some(super::Probe {
                acquisition: probe.acquisition_episodes,
                events: probe.event_formations,
                two_roots: probe.two_root_episodes,
                directions: probe.d3_directions,
                physical_directions: probe.physical_directions,
                deliveries: probe.direction_deliveries,
                updates: probe.update_calls,
                patterns: probe.patterns,
                divergent_patterns: probe.divergent_patterns,
                consequence_mature: probe.consequence_mature_patterns,
                evaluator_mature: probe.evaluator_mature_patterns,
                held_out_attempts: probe.held_out_attempts,
                held_out_successes: probe.held_out_successes,
                held_out_abstentions: probe.held_out_abstentions,
                d3_work: probe.d3_work,
                learner_work: probe.learner_work,
                learner_bytes: probe.persistent_learner_bytes,
                fingerprint: probe.learner_fingerprint,
                evaluator_used_in_acquisition: probe.acquisition_used_evaluator_role,
            })
        }

        pub(super) fn definitive_parent_audit() -> bool {
            source_audit().passed()
                && EXACT_PARENT == "ee576d95b88d04629f715d06c639a8f400ff2819"
                && AUTHORITATIVE_M0 == super::AUTHORITATIVE_M0
                && FROZEN_RESULTS_DIGEST == super::FROZEN_RESULTS_DIGEST
        }
    };
}

#[allow(dead_code)]
mod frozen_parent {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds1_after_d3_cumulative_composition_retry.rs"
    ));
    definitive_parent_access!();
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub parent_hash: bool,
    pub parent_handoff_hash: bool,
    pub protocol_hash: bool,
    pub parent_audit: bool,
    pub exact_parent: bool,
    pub exact_matrix: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.parent_hash
            && self.parent_handoff_hash
            && self.protocol_hash
            && self.parent_audit
            && self.exact_parent
            && self.exact_matrix
    }
}

fn source_audit() -> SourceAudit {
    SourceAudit {
        parent_hash: env!("DS1_DEFINITIVE_PARENT_SHA256") == FROZEN_PARENT_SHA256,
        parent_handoff_hash: env!("DS1_DEFINITIVE_PARENT_HANDOFF_SHA256")
            == FROZEN_PARENT_HANDOFF_SHA256,
        protocol_hash: env!("DS1_DEFINITIVE_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        parent_audit: frozen_parent::definitive_parent_audit(),
        exact_parent: frozen_parent::PROTOCOL == "ds1-after-d3-cumulative-composition-retry-v1"
            && frozen_parent::AUTHORITATIVE_M0 == AUTHORITATIVE_M0,
        exact_matrix: DEFINITIVE_SEEDS == 8
            && DEFINITIVE_ACQUISITION == 64
            && DEFINITIVE_HELD_OUT == 32,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RawCell {
    seed: u64,
    main: Probe,
    reversed: Probe,
    stage_ready: [bool; 8],
    first_collapse: String,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub seed: u64,
    pub acquisition: usize,
    pub events: usize,
    pub two_roots: usize,
    pub d3_directions: usize,
    pub physical_directions: usize,
    pub deliveries: usize,
    pub updates: u64,
    pub patterns: usize,
    pub divergent_patterns: usize,
    pub consequence_mature: usize,
    pub evaluator_mature: usize,
    pub held_out_attempts: usize,
    pub held_out_successes: usize,
    pub held_out_abstentions: usize,
    pub reversed_consequence_mature: usize,
    pub reversed_evaluator_mature: usize,
    pub d3_work: u64,
    pub learner_work: u64,
    pub learner_bytes: usize,
    pub fingerprint: u64,
    pub duplicate_deterministic: bool,
    pub first_collapse: String,
    pub passed: bool,
}

const STAGES: [&str; 8] = [
    "source and exact frozen-parent audit",
    "all acquisition episodes form E0 events and two executable roots",
    "D3 delivers one physically executed direction per acquisition episode",
    "acquisition is evaluator-blind",
    "frozen DS1 updates once per acquisition episode",
    "all four patterns diverge and mature to the consequence-supported route",
    "all held-out boundary roles reconstruct without abstention",
    "reversed-world learning follows consequences rather than evaluator roles",
];

fn raw_cell(seed: u64, acquisition: usize, held_out: usize, source: &SourceAudit) -> RawCell {
    let main = frozen_parent::definitive_probe(seed, acquisition, held_out, false)
        .expect("frozen cumulative parent produces the main cell");
    let reversed = frozen_parent::definitive_probe(seed, acquisition, held_out, true)
        .expect("frozen cumulative parent produces the reversed-world cell");
    let stage_ready = [
        source.passed(),
        main.acquisition == acquisition
            && main.events == acquisition
            && main.two_roots == acquisition,
        main.directions == acquisition
            && main.physical_directions == acquisition
            && main.deliveries == acquisition,
        !main.evaluator_used_in_acquisition && !reversed.evaluator_used_in_acquisition,
        main.updates == acquisition as u64,
        main.patterns == 4
            && main.divergent_patterns == 4
            && main.consequence_mature == 4
            && main.evaluator_mature == 4,
        main.held_out_attempts == held_out
            && main.held_out_successes == held_out
            && main.held_out_abstentions == 0,
        reversed.consequence_mature == 4 && reversed.evaluator_mature == 0,
    ];
    let first_collapse = stage_ready
        .iter()
        .position(|ready| !ready)
        .map(|stage| STAGES[stage].to_string())
        .unwrap_or_else(|| "NONE".to_string());
    RawCell {
        seed,
        main,
        reversed,
        stage_ready,
        first_collapse,
        passed: stage_ready.iter().all(|ready| *ready),
    }
}

fn evaluate_cell(seed: u64, acquisition: usize, held_out: usize, source: &SourceAudit) -> Cell {
    let first = raw_cell(seed, acquisition, held_out, source);
    let second = raw_cell(seed, acquisition, held_out, source);
    let duplicate_deterministic = first == second;
    Cell {
        seed,
        acquisition: first.main.acquisition,
        events: first.main.events,
        two_roots: first.main.two_roots,
        d3_directions: first.main.directions,
        physical_directions: first.main.physical_directions,
        deliveries: first.main.deliveries,
        updates: first.main.updates,
        patterns: first.main.patterns,
        divergent_patterns: first.main.divergent_patterns,
        consequence_mature: first.main.consequence_mature,
        evaluator_mature: first.main.evaluator_mature,
        held_out_attempts: first.main.held_out_attempts,
        held_out_successes: first.main.held_out_successes,
        held_out_abstentions: first.main.held_out_abstentions,
        reversed_consequence_mature: first.reversed.consequence_mature,
        reversed_evaluator_mature: first.reversed.evaluator_mature,
        d3_work: first.main.d3_work,
        learner_work: first.main.learner_work,
        learner_bytes: first.main.learner_bytes,
        fingerprint: first.main.fingerprint,
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
    pub m0_authoritative: bool,
    pub m1_exists: bool,
    pub m1_authoritative: bool,
}

fn run_matrix(
    mode: &str,
    claim_eligible: bool,
    seeds: &[u64],
    acquisition: usize,
    held_out: usize,
) -> Report {
    let source = source_audit();
    let cells = seeds
        .iter()
        .map(|seed| evaluate_cell(*seed, acquisition, held_out, &source))
        .collect::<Vec<_>>();
    let passed = source.passed() && !cells.is_empty() && cells.iter().all(|cell| cell.passed);
    Report {
        mode: mode.to_string(),
        claim_eligible,
        source,
        cells,
        passed,
        m0_authoritative: !claim_eligible || !passed,
        m1_exists: passed,
        m1_authoritative: claim_eligible && passed,
    }
}

pub fn run_audit() -> Report {
    run_matrix("AUDIT", false, &[100], 16, 8)
}

pub fn run_definitive() -> Report {
    let seeds = (0..DEFINITIVE_SEEDS as u64).collect::<Vec<_>>();
    run_matrix(
        "DEFINITIVE",
        true,
        &seeds,
        DEFINITIVE_ACQUISITION,
        DEFINITIVE_HELD_OUT,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_uses_the_exact_parent_without_claim_authority() {
        let report = run_audit();
        assert!(report.passed, "{report:#?}");
        assert!(!report.claim_eligible && !report.m1_authoritative);
        assert_eq!(report.cells.len(), 1);
        assert!(report.cells[0].duplicate_deterministic);
    }

    #[test]
    fn source_and_matrix_are_frozen() {
        assert!(source_audit().passed());
        assert_eq!(
            (
                DEFINITIVE_SEEDS,
                DEFINITIVE_ACQUISITION,
                DEFINITIVE_HELD_OUT
            ),
            (8, 64, 32)
        );
    }
}
