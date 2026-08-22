//! Development-only two-cell DS7 history/economy MICRO.

pub const PROTOCOL: &str = "ds7-cumulative-plasticity-allocation-micro-v3";
pub const PROTOCOL_COMMIT: &str = "6384f27";
pub const AUTHORITATIVE_M4: &str = "8db47281a7c9c97cbb52ced6fc3dcff0e7efa9b2";
pub const SEEDS: [u64; 2] = [21_000_000, 21_500_000];
pub const FROZEN_PROBE_SHA256: &str =
    "7b04bdaca1aec34d563f728bfbcd2c28be07cd28e5bfc9778c69fce197c465ea";
pub const FROZEN_PROBE_RESULT_SHA256: &str =
    "93c11d30b9f3ebfbf6de68a315b3410275acc4031186eaa7e747a81f78a8ded4";
pub const FROZEN_PROBE_AUDIT_SHA256: &str =
    "b6d9f11fcb26d9702d95db989991558f3f86ca99dfe360b597b09a2b84d474f2";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "1648b4a0e19a13c918a3c92ad221c93b7ae145a34c6e960e57fd9d0f91506eb9";

#[derive(Clone, Debug, PartialEq)]
pub struct MicroCell {
    pub seed: u64,
    pub history: &'static str,
    pub acquisition: bool,
    pub productive_admissions: usize,
    pub unproductive_admissions: usize,
    pub always_open_admissions: usize,
    pub reduction: f64,
    pub shuffled_reversal: bool,
    pub reversal_opportunities: usize,
    pub reversal_explorations: usize,
    pub retargeted: bool,
    pub lifecycle: bool,
    pub fresh_identity_layout: bool,
    pub source_audit: bool,
    pub cumulative_m4: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MicroReport {
    pub protocol: &'static str,
    pub cells: Vec<MicroCell>,
    pub duplicate_exact: bool,
    pub passed: bool,
}

#[allow(dead_code)]
mod frozen_probe {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds7_cumulative_plasticity_targeting_probe_frozen.rs"
    ));

    fn score(path: &PlasticityPath, snapshot: EncounterSnapshot) -> i32 {
        path.encoder
            .recognized(snapshot)
            .and_then(|id| path.values.get(&id))
            .map_or(0, |record| record.score())
    }

    fn present(
        path: &mut PlasticityPath,
        encounter: PhysicalEncounter,
        supported: bool,
    ) -> bool {
        let Some(edge) = path.encounter(encounter) else {
            return false;
        };
        path.execute_and_observe(edge, supported)
    }

    fn seeded_p(seed: u64, ordinal: u64, reversed_layout: bool) -> PhysicalEncounter {
        let base = seed.wrapping_add(10_000 + 2 * ordinal);
        if reversed_layout {
            pattern_p(base, base + 1, 91, 90)
        } else {
            pattern_p(base, base + 1, 40, 41)
        }
    }

    fn seeded_n(seed: u64, ordinal: u64, reversed_layout: bool) -> PhysicalEncounter {
        let base = seed.wrapping_add(20_000 + 2 * ordinal);
        if reversed_layout {
            pattern_n(base, base + 1, 101, 100)
        } else {
            pattern_n(base, base + 1, 50, 51)
        }
    }

    pub(super) fn run_micro_cell(seed: u64, interleaved: bool) -> super::MicroCell {
        let mut path = PlasticityPath::default();
        let p_snapshot = seeded_p(seed, 0, false).snapshot();
        let n_snapshot = seeded_n(seed, 0, false).snapshot();
        let u = pattern_u(seed + 30_000, seed + 30_001);
        let u_snapshot = u.snapshot();
        let u_edge = u.edge();

        let u_pre_outcome = path.encounter(u);
        let u_learned = u_pre_outcome.is_some_and(|edge| path.execute_and_observe(edge, false));

        let history: Vec<_> = if interleaved {
            (0..32).map(|index| index % 2 == 1).collect()
        } else {
            vec![false, false, false, false, true, true, true, true]
        };
        let mut first_p_pre_outcome = false;
        let mut first_n_pre_outcome = false;
        let mut p_updates = 0;
        let mut n_updates = 0;
        let mut p_ordinal = 0;
        let mut n_ordinal = 0;
        for productive in history {
            if productive {
                let encounter = seeded_p(seed, p_ordinal, false);
                p_ordinal += 1;
                let proposal = path.encounter(encounter);
                first_p_pre_outcome |= proposal.is_some();
                if let Some(edge) = proposal {
                    p_updates += usize::from(path.execute_and_observe(edge, true));
                }
            } else {
                let encounter = seeded_n(seed, n_ordinal, false);
                n_ordinal += 1;
                let proposal = path.encounter(encounter);
                first_n_pre_outcome |= proposal.is_some();
                if let Some(edge) = proposal {
                    n_updates += usize::from(path.execute_and_observe(edge, false));
                }
            }
        }

        let acquisition = u_learned
            && first_p_pre_outcome
            && first_n_pre_outcome
            && p_updates >= 2
            && n_updates >= 2
            && score(&path, p_snapshot) >= VALUE_THRESHOLD
            && score(&path, n_snapshot) <= -VALUE_THRESHOLD;

        let mut economy = path.clone();
        let mut productive_admissions = 0;
        let mut unproductive_admissions = 0;
        for ordinal in 0..32u64 {
            productive_admissions += usize::from(
                economy
                    .encounter(seeded_p(seed, 100 + ordinal, ordinal % 2 == 0))
                    .is_some(),
            );
            unproductive_admissions += usize::from(
                economy
                    .encounter(seeded_n(seed, 100 + ordinal, ordinal % 2 == 1))
                    .is_some(),
            );
        }
        let always_open_admissions = 64;
        let learned_admissions = productive_admissions + unproductive_admissions;
        let reduction = 1.0 - learned_admissions as f64 / always_open_admissions as f64;
        let economy_pass = productive_admissions == 32
            && unproductive_admissions <= 4
            && reduction >= 0.40;

        let mut shuffled = path.clone();
        let swap = shuffled.swap_values(p_snapshot, n_snapshot);
        let shuffled_p = shuffled
            .encounter(seeded_p(seed, 1_000, true))
            .is_some();
        let shuffled_n = shuffled
            .encounter(seeded_n(seed, 1_000, true))
            .is_some();
        let shuffled_reversal = swap && !shuffled_p && shuffled_n;

        let p_before = path.prototype_resistance(p_snapshot);
        let n_before = path.prototype_resistance(n_snapshot);
        let exploration_before = path.exploration_admissions;
        let mut reversal_opportunities = 0;
        while score(&path, n_snapshot) < VALUE_THRESHOLD && reversal_opportunities < 96 {
            let encounter = seeded_n(seed, 2_000 + reversal_opportunities as u64, true);
            reversal_opportunities += 1;
            if let Some(edge) = path.encounter(encounter) {
                let _ = path.execute_and_observe(edge, true);
            }
            if reversal_opportunities % 4 == 0 {
                if let Some(edge) = path.encounter(seeded_p(
                    seed,
                    3_000 + reversal_opportunities as u64,
                    true,
                )) {
                    let _ = path.execute_and_observe(edge, true);
                }
            }
        }
        let reversal_explorations = path.exploration_admissions - exploration_before;
        let exploration_clock_before = path.exploration_clock;
        let direct_n = path
            .encounter(seeded_n(seed, 4_000, true))
            .is_some();
        let direct_without_exploration = path.exploration_clock == exploration_clock_before;
        let direct_p = path
            .encounter(seeded_p(seed, 4_001, true))
            .is_some();
        let retargeted = reversal_opportunities <= 96
            && reversal_explorations > 0
            && score(&path, n_snapshot) >= VALUE_THRESHOLD
            && direct_n
            && direct_without_exploration
            && direct_p;

        while path.completed < 96 {
            let ordinal = 5_000 + path.completed as u64;
            let encounter = if path.completed % 2 == 0 {
                seeded_p(seed, ordinal, false)
            } else {
                seeded_n(seed, ordinal, false)
            };
            if let Some(edge) = path.encounter(encounter) {
                let _ = path.execute_and_observe(edge, true);
            }
        }
        let lifecycle = path.proposal_resistance(u_edge) == 0
            && path.prototype_resistance(u_snapshot) == 0
            && path.value_resistance(u_snapshot) == 0
            && !path.execute(u_edge)
            && path.prototype_resistance(p_snapshot) >= p_before
            && path.prototype_resistance(n_snapshot) >= n_before
            && path.value_resistance(p_snapshot) > 0
            && path.value_resistance(n_snapshot) > 0;

        let fresh_identity_layout = economy_pass && shuffled_reversal && retargeted;
        let audit = source_audit().passed()
            && env!("DS7_MICRO_PROBE_RESULT_SHA256") == super::FROZEN_PROBE_RESULT_SHA256
            && env!("DS7_MICRO_PROBE_AUDIT_SHA256") == super::FROZEN_PROBE_AUDIT_SHA256
            && env!("DS7_MICRO_PROTOCOL_SHA256") == super::FROZEN_PROTOCOL_SHA256;
        let cumulative_m4 = frozen_m4::authority_exact(seed);
        let passed = acquisition
            && economy_pass
            && shuffled_reversal
            && retargeted
            && lifecycle
            && fresh_identity_layout
            && audit
            && cumulative_m4;
        super::MicroCell {
            seed,
            history: if interleaved { "interleaved" } else { "blocked" },
            acquisition,
            productive_admissions,
            unproductive_admissions,
            always_open_admissions,
            reduction,
            shuffled_reversal,
            reversal_opportunities,
            reversal_explorations,
            retargeted,
            lifecycle,
            fresh_identity_layout,
            source_audit: audit,
            cumulative_m4,
            passed,
        }
    }
}

fn run_cells() -> Vec<MicroCell> {
    vec![
        frozen_probe::run_micro_cell(SEEDS[0], false),
        frozen_probe::run_micro_cell(SEEDS[1], true),
    ]
}

pub fn run() -> MicroReport {
    let cells = run_cells();
    let duplicate_exact = cells == run_cells();
    let passed = duplicate_exact && cells.len() == 2 && cells.iter().all(|cell| cell.passed);
    MicroReport {
        protocol: PROTOCOL,
        cells,
        duplicate_exact,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_is_duplicate_exact_and_conjunctive() {
        let report = run();
        assert!(report.duplicate_exact);
        assert!(report.cells.iter().all(|cell| cell.passed), "{report:#?}");
        assert!(report.passed);
    }
}
