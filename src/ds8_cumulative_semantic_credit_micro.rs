//! Development-only two-cell cumulative DS8 non-semantic-credit MICRO.

pub const PROTOCOL: &str = "ds8-cumulative-semantic-credit-micro-v1";
pub const PROTOCOL_COMMIT: &str = "e26f968ed75973ebc219c1d4597d79751331d92f";
pub const AUTHORITATIVE_M5: &str = "9c5ba68a6a4ae37b51575ebaae414ab51a248575";
pub const SEEDS: [u64; 2] = [41_000_000, 41_500_000];
pub const FROZEN_PROBE_SHA256: &str =
    "11b4229122b3e0788ca30c55579b91ffe07461de9a138860690134565fcf2ed6";
pub const FROZEN_LINKER_SHA256: &str =
    "7d9bfa61ccfcb7e99d5863306920a5a1700aa395e33901a70b76d5ccad27aa17";
pub const FROZEN_RESULT_SHA256: &str =
    "a1acd9c35cb5da4ab1c9aa341926dae6d8d8cb5e10e216be2089879328d8404d";
pub const FROZEN_AUDIT_SHA256: &str =
    "00f788e2b3a6930fed7ec69626062f5d6631167002a179cc96e7499d5280502a";
pub const FROZEN_HANDOFF_SHA256: &str =
    "2653b6db073e3241a4d98e223cbc545826b57ecf272f2bc073f58dac98060087";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "4c66a654607659cccd5e1692dcf3c491452cc394c6cdb9583f52de6ae71b433a";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MicroCell {
    pub seed: u64,
    pub history: &'static str,
    pub blank_acquisition: bool,
    pub executions: usize,
    pub first_updates: usize,
    pub second_updates: usize,
    pub first_admissions: usize,
    pub second_admissions: usize,
    pub swapped_first_admissions: usize,
    pub swapped_second_admissions: usize,
    pub shuffled_first_admissions: usize,
    pub shuffled_second_admissions: usize,
    pub physical_exact: bool,
    pub contrast: bool,
    pub fresh_transfer: bool,
    pub controls: bool,
    pub source_audit: bool,
    pub cumulative_m5: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MicroReport {
    pub protocol: &'static str,
    pub cells: Vec<MicroCell>,
    pub duplicate_exact: bool,
    pub passed: bool,
}

#[allow(dead_code)]
mod frozen_linker {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds7_cumulative_plasticity_targeting_probe_frozen.rs"
    ));

    const MINIMUM_DELAY: u8 = 2;
    const RECURRENT_SUPPORT: u16 = 4;
    const MINIMUM_MARGIN: u16 = 2;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RawConsequence {
        occurrences: [u64; 3],
        ticks: [u8; 3],
        arrows: [[u8; 2]; 2],
        root: u8,
    }

    fn raw_consequence(
        seed: u64,
        episode: usize,
        variant: usize,
        immediate: bool,
        relabel: bool,
    ) -> RawConsequence {
        let base = seed
            .wrapping_mul(1_000_003)
            .wrapping_add(episode as u64 * 53)
            .wrapping_add(1 << 33);
        let mut occurrences = [base, base + 1, base + 2];
        if relabel {
            occurrences.rotate_left(1);
            occurrences.reverse();
        }
        let (root, arrows) = match variant % 4 {
            0 => (0, [[0, 1], [1, 2]]),
            1 => (0, [[0, 2], [2, 1]]),
            2 => (1, [[1, 0], [0, 2]]),
            _ => (2, [[2, 1], [1, 0]]),
        };
        let first_tick = if immediate { 1 } else { MINIMUM_DELAY };
        RawConsequence {
            occurrences,
            ticks: [first_tick, first_tick + 1, first_tick + 2],
            arrows,
            root,
        }
    }

    include!(concat!(
        env!("OUT_DIR"),
        "/ds8_cumulative_semantic_credit_linker_frozen.rs"
    ));

    #[derive(Clone)]
    struct Trained {
        path: PlasticityPath,
        learner: ConsequenceLearner,
        first: EncounterSnapshot,
        second: EncounterSnapshot,
        first_seen: bool,
        second_seen: bool,
        executions: usize,
        first_updates: usize,
        second_updates: usize,
        equal_magnitude: bool,
    }

    fn first(seed: u64, ordinal: u64, reverse_layout: bool) -> PhysicalEncounter {
        let base = seed + 10_000 + ordinal * 2;
        if reverse_layout {
            pattern_p(base, base + 1, 91, 90)
        } else {
            pattern_p(base, base + 1, 40, 41)
        }
    }

    fn second(seed: u64, ordinal: u64, reverse_layout: bool) -> PhysicalEncounter {
        let base = seed + 20_000 + ordinal * 2;
        if reverse_layout {
            pattern_n(base, base + 1, 101, 100)
        } else {
            pattern_n(base, base + 1, 50, 51)
        }
    }

    fn episode(
        path: &mut PlasticityPath,
        learner: &mut ConsequenceLearner,
        encounter: PhysicalEncounter,
        other: EncounterSnapshot,
        raw: RawConsequence,
    ) -> (bool, bool) {
        let active = encounter.snapshot();
        let Some(edge) = path.encounter(encounter) else {
            return (false, false);
        };
        if !path.execute(edge) {
            return (false, false);
        }
        (true, learner.apply(path, active, other, raw))
    }

    fn schedule(interleaved: bool, swapped: bool) -> Vec<bool> {
        if interleaved {
            (0..24).map(|index| index % 2 == 0).collect()
        } else if swapped {
            [vec![true; 12], vec![false; 12]].concat()
        } else {
            [vec![false; 12], vec![true; 12]].concat()
        }
    }

    fn train(seed: u64, interleaved: bool, swapped: bool, relabel: bool) -> Trained {
        let mut path = PlasticityPath::default();
        let mut learner = ConsequenceLearner::default();
        let first_snapshot = first(seed, 0, false).snapshot();
        let second_snapshot = second(seed, 0, false).snapshot();
        let mut first_ordinal = 0u64;
        let mut second_ordinal = 0u64;
        let mut first_seen = false;
        let mut second_seen = false;
        let mut executions = 0;
        let mut first_updates = 0;
        let mut second_updates = 0;
        let mut equal_magnitude = true;
        for (episode_index, use_first) in schedule(interleaved, swapped).into_iter().enumerate() {
            let ordinal = if use_first {
                let current = first_ordinal;
                first_ordinal += 1;
                current
            } else {
                let current = second_ordinal;
                second_ordinal += 1;
                current
            };
            let recurrent = use_first != swapped;
            let variant = if recurrent { 0 } else { ordinal as usize % 4 };
            let raw = raw_consequence(
                seed + 100_000,
                episode_index,
                variant,
                false,
                relabel && episode_index % 2 == 1,
            );
            let mut check_work = ConsequenceWork::default();
            equal_magnitude &= execute_and_normalize(raw, &mut check_work)
                .as_ref()
                .map(ConsequenceShape::magnitude)
                == Some(3);
            let (executed, updated) = if use_first {
                episode(
                    &mut path,
                    &mut learner,
                    first(seed, ordinal, false),
                    second_snapshot,
                    raw,
                )
            } else {
                episode(
                    &mut path,
                    &mut learner,
                    second(seed, ordinal, false),
                    first_snapshot,
                    raw,
                )
            };
            if use_first {
                first_seen |= executed;
                first_updates += usize::from(updated);
            } else {
                second_seen |= executed;
                second_updates += usize::from(updated);
            }
            executions += usize::from(executed);
        }
        Trained {
            path,
            learner,
            first: first_snapshot,
            second: second_snapshot,
            first_seen,
            second_seen,
            executions,
            first_updates,
            second_updates,
            equal_magnitude,
        }
    }

    fn admissions(path: &PlasticityPath, seed: u64, reverse_layout: bool) -> (usize, usize) {
        let mut path = path.clone();
        let mut first_count = 0;
        let mut second_count = 0;
        for ordinal in 100..108u64 {
            first_count += usize::from(
                path.encounter(first(seed, ordinal, reverse_layout))
                    .is_some(),
            );
            second_count += usize::from(
                path.encounter(second(seed, ordinal, reverse_layout))
                    .is_some(),
            );
        }
        (first_count, second_count)
    }

    pub(super) fn run_cell(seed: u64, interleaved: bool) -> super::MicroCell {
        let trained = train(seed, interleaved, false, true);
        let direction = trained
            .learner
            .clone()
            .direction([trained.first, trained.second]);
        let (first_admissions, second_admissions) = admissions(&trained.path, seed, false);
        let fresh = admissions(&trained.path, seed + 1_000_000, true);
        let relabel_control = train(seed, interleaved, false, false);
        let relabel_admissions = admissions(&relabel_control.path, seed + 2_000_000, true);

        let swapped = train(seed + 3_000_000, interleaved, true, true);
        let swapped_direction = swapped
            .learner
            .clone()
            .direction([swapped.first, swapped.second]);
        let (swapped_first_admissions, swapped_second_admissions) =
            admissions(&swapped.path, seed + 3_000_000, true);

        let mut shuffled = trained.path.clone();
        let swap_complete = shuffled.swap_values(trained.first, trained.second);
        let (shuffled_first_admissions, shuffled_second_admissions) =
            admissions(&shuffled, seed + 4_000_000, true);

        let blank_acquisition = trained.first_seen && trained.second_seen;
        let physical_exact = trained.executions > 0
            && trained.learner.work.spikes == trained.executions as u64 * 3
            && trained.learner.work.routes == trained.executions as u64 * 2
            && trained.learner.work.observations == trained.executions as u64;
        let contrast = direction == Some(trained.first)
            && trained.first_updates > 0
            && trained.equal_magnitude;
        let learned_allocation = first_admissions >= 7 && second_admissions <= 1;
        let swapped_control = swapped_direction == Some(swapped.second)
            && swapped_first_admissions <= 1
            && swapped_second_admissions >= 7;
        let shuffled_control = swap_complete
            && shuffled_first_admissions <= 1
            && shuffled_second_admissions >= 7;
        let fresh_transfer = fresh.0 >= 7
            && fresh.1 <= 1
            && relabel_admissions.0 >= 7
            && relabel_admissions.1 <= 1;
        let controls = swapped_control && shuffled_control;
        let source_audit = env!("DS8_MICRO_LINKER_FRAGMENT_SHA256")
            == super::FROZEN_LINKER_SHA256
            && env!("DS8_MICRO_PROBE_SHA256") == super::FROZEN_PROBE_SHA256
            && env!("DS8_MICRO_RESULT_SHA256") == super::FROZEN_RESULT_SHA256
            && env!("DS8_MICRO_AUDIT_SHA256") == super::FROZEN_AUDIT_SHA256
            && env!("DS8_MICRO_HANDOFF_SHA256") == super::FROZEN_HANDOFF_SHA256
            && env!("DS8_MICRO_PROTOCOL_SHA256") == super::FROZEN_PROTOCOL_SHA256;
        let cumulative_probe = crate::ds8_cumulative_semantic_credit_probe::run();
        let cumulative_m5 = cumulative_probe.passed
            && cumulative_probe
                .checks
                .iter()
                .any(|check| check.name == "unchanged authoritative M5" && check.passed);
        let passed = blank_acquisition
            && physical_exact
            && contrast
            && learned_allocation
            && controls
            && fresh_transfer
            && cumulative_probe.passed
            && source_audit
            && cumulative_m5;
        super::MicroCell {
            seed,
            history: if interleaved { "interleaved" } else { "blocked" },
            blank_acquisition,
            executions: trained.executions,
            first_updates: trained.first_updates,
            second_updates: trained.second_updates,
            first_admissions,
            second_admissions,
            swapped_first_admissions,
            swapped_second_admissions,
            shuffled_first_admissions,
            shuffled_second_admissions,
            physical_exact,
            contrast,
            fresh_transfer,
            controls,
            source_audit,
            cumulative_m5,
            passed,
        }
    }
}

fn run_cells() -> Vec<MicroCell> {
    vec![
        frozen_linker::run_cell(SEEDS[0], true),
        frozen_linker::run_cell(SEEDS[1], false),
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
