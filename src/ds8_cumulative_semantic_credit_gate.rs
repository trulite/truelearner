//! Development-only cumulative DS8 non-semantic-credit GATE.

pub const PROTOCOL: &str = "ds8-cumulative-semantic-credit-gate-v1";
pub const PROTOCOL_COMMIT: &str = "a45fc51fed2e4a589e25ed0ef41b3cc80ec04fb7";
pub const AUTHORITATIVE_M5: &str = "9c5ba68a6a4ae37b51575ebaae414ab51a248575";
pub const SEEDS: [u64; 6] = [
    42_000_000, 42_500_000, 43_000_000, 43_500_000, 44_000_000, 44_500_000,
];
pub const LOADS: [usize; 3] = [8, 32, 128];
pub const FROZEN_LINKER_SHA256: &str =
    "1f68f7e943f37c42d29f16fe26f0d851a59361ed4c1f4273a82d0537f935d343";
pub const FROZEN_MICRO_SOURCE_SHA256: &str =
    "36dd93a581bb7d15cb82de3089ff6786fc8b5e7a0edf2181383a52422d301b78";
pub const FROZEN_MICRO_RESULT_SHA256: &str =
    "d225ea316bd492df5025088615b8650c7cb5476b9b90cbe5b3e826b22129a894";
pub const FROZEN_MICRO_AUDIT_SHA256: &str =
    "a316da1a1d327ed417e5b07ee3c0099dcc00d72af00fb65479e0e4d52e3b064b";
pub const FROZEN_HANDOFF_SHA256: &str =
    "91feb6e878fae6f8155dd3f9ea5107ec5e8d1fceab6cea460b23808d537e29da";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "c65c08c59056cda39d3f93615da9be28ab12210d8c19b76de18dd8a0ef245b78";

#[derive(Clone, Debug, PartialEq)]
pub struct GateCell {
    pub seed: u64,
    pub load: usize,
    pub blank_acquisition: bool,
    pub physical_exact: bool,
    pub heldout: usize,
    pub heldout_total: usize,
    pub route_admissions: usize,
    pub distractor_admissions: usize,
    pub always_open_admissions: usize,
    pub work_reduction: f64,
    pub raw_history_reversal: bool,
    pub value_shuffle_reversal: bool,
    pub entry_resistance: i32,
    pub final_resistance: i32,
    pub removed: bool,
    pub stale_blocked: bool,
    pub repaired: usize,
    pub repaired_total: usize,
    pub retained_economy: bool,
    pub topology_identity_layout: bool,
    pub controls: bool,
    pub source_audit: bool,
    pub cumulative_m5: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GateReport {
    pub protocol: &'static str,
    pub cells: Vec<GateCell>,
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

    fn raw_consequence(seed: u64, episode: usize, variant: usize) -> RawConsequence {
        let base = seed
            .wrapping_mul(1_000_003)
            .wrapping_add(episode as u64 * 53)
            .wrapping_add(1 << 33);
        let mut occurrences = [base, base + 1, base + 2];
        if episode % 2 == 1 {
            occurrences.rotate_left(1);
            occurrences.reverse();
        }
        let (root, arrows) = match variant % 4 {
            0 => (0, [[0, 1], [1, 2]]),
            1 => (0, [[0, 2], [2, 1]]),
            2 => (1, [[1, 0], [0, 2]]),
            _ => (2, [[2, 1], [1, 0]]),
        };
        RawConsequence {
            occurrences,
            ticks: [MINIMUM_DELAY, MINIMUM_DELAY + 1, MINIMUM_DELAY + 2],
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
        route: Vec<PhysicalEncounter>,
        distractors: Vec<PhysicalEncounter>,
        blank_acquisition: bool,
        physical_exact: bool,
    }

    fn route(seed: u64, reverse_layout: bool) -> Vec<PhysicalEncounter> {
        (0..4u64)
            .map(|index| {
                let base = seed + 1_000 + 2 * index;
                if reverse_layout {
                    pattern_p(base, base + 1, 91, 90)
                } else {
                    pattern_p(base, base + 1, 40, 41)
                }
            })
            .collect()
    }

    fn distractors(seed: u64, load: usize, reverse_layout: bool) -> Vec<PhysicalEncounter> {
        (0..load as u64)
            .map(|index| {
                let base = seed + 10_000 + 2 * index;
                if reverse_layout {
                    pattern_n(base, base + 1, 101, 100)
                } else {
                    pattern_n(base, base + 1, 50, 51)
                }
            })
            .collect()
    }

    fn route_edges(route: &[PhysicalEncounter]) -> Vec<Edge> {
        route.iter().map(|encounter| encounter.edge()).collect()
    }

    fn execute_update(
        path: &mut PlasticityPath,
        learner: &mut ConsequenceLearner,
        encounter: PhysicalEncounter,
        other: EncounterSnapshot,
        raw: RawConsequence,
    ) -> (bool, bool) {
        let active = encounter.snapshot();
        let Some(edge) = path.local_encounter(encounter) else {
            return (false, false);
        };
        if !path.execute(edge) {
            return (false, false);
        }
        (true, learner.apply(path, active, other, raw))
    }

    fn train(seed: u64, load: usize, swapped: bool) -> Trained {
        let reverse_layout = seed % 1_000_000 != 0;
        let route = route(seed, reverse_layout);
        let distractors = distractors(seed, load, !reverse_layout);
        let route_snapshot = route[0].snapshot();
        let distractor_snapshot = distractors[0].snapshot();
        let stable_variant = (seed as usize / 500_000) % 4;
        let mut path = PlasticityPath::default();
        let mut learner = ConsequenceLearner::default();
        let mut route_executions = 0;
        let mut distractor_executions = 0;
        let mut total_executions = 0;
        for sweep in 0..8usize {
            path.begin_event();
            for (index, encounter) in distractors.iter().enumerate() {
                let variant = if swapped {
                    stable_variant
                } else {
                    (sweep * load + index) % 4
                };
                let (executed, _) = execute_update(
                    &mut path,
                    &mut learner,
                    *encounter,
                    route_snapshot,
                    raw_consequence(seed + 100_000, sweep * (load + 4) + index, variant),
                );
                distractor_executions += usize::from(executed);
                total_executions += usize::from(executed);
            }
            for (index, encounter) in route.iter().enumerate() {
                let variant = if swapped {
                    (sweep * 4 + index) % 4
                } else {
                    stable_variant
                };
                let (executed, _) = execute_update(
                    &mut path,
                    &mut learner,
                    *encounter,
                    distractor_snapshot,
                    raw_consequence(
                        seed + 200_000,
                        sweep * (load + 4) + load + index,
                        variant,
                    ),
                );
                route_executions += usize::from(executed);
                total_executions += usize::from(executed);
            }
        }
        let physical_exact = learner.work.spikes == total_executions as u64 * 3
            && learner.work.routes == total_executions as u64 * 2
            && learner.work.observations == total_executions as u64;
        let blank_acquisition = route_executions >= 4
            && distractor_executions > 0
            && route_edges(&route)
                .iter()
                .all(|edge| path.proposals.contains_key(edge));

        if !swapped {
            for sweep in 0..20usize {
                path.begin_event();
                let distractor = distractors[sweep % distractors.len()];
                let _ = execute_update(
                    &mut path,
                    &mut learner,
                    distractor,
                    route_snapshot,
                    raw_consequence(seed + 300_000, sweep * 5, sweep % 4),
                );
                for (index, encounter) in route.iter().enumerate() {
                    let _ = execute_update(
                        &mut path,
                        &mut learner,
                        *encounter,
                        distractor_snapshot,
                        raw_consequence(
                            seed + 300_000,
                            sweep * 5 + index + 1,
                            stable_variant,
                        ),
                    );
                }
            }
        }
        Trained {
            path,
            learner,
            route,
            distractors,
            blank_acquisition,
            physical_exact,
        }
    }

    fn traverse(path: &mut PlasticityPath, edges: &[Edge], occurrence: u64) -> bool {
        if occurrence == 0 || !edges.iter().all(|edge| path.proposals.contains_key(edge)) {
            return false;
        }
        for edge in edges {
            path.proposals
                .get_mut(edge)
                .expect("live route was checked")
                .life
                .reused();
        }
        true
    }

    fn heldout(path: &mut PlasticityPath, edges: &[Edge], seed: u64) -> usize {
        let mut passed = 0;
        for index in 0..32u64 {
            path.begin_event();
            passed += usize::from(traverse(path, edges, seed + 500_000 + index));
        }
        passed
    }

    fn admission_sweep(
        path: &mut PlasticityPath,
        route: &[PhysicalEncounter],
        distractors: &[PhysicalEncounter],
    ) -> (usize, usize) {
        path.begin_event();
        let route_count = route
            .iter()
            .filter(|encounter| path.local_encounter(**encounter).is_some())
            .count();
        let distractor_count = distractors
            .iter()
            .filter(|encounter| path.local_encounter(**encounter).is_some())
            .count();
        (route_count, distractor_count)
    }

    fn shuffle_values(
        path: &mut PlasticityPath,
        first: EncounterSnapshot,
        second: EncounterSnapshot,
    ) -> bool {
        if path.swap_values(first, second) {
            return true;
        }
        let Some(first_id) = path.encoder.recognized(first) else {
            return false;
        };
        let Some(second_id) = path.encoder.recognized(second) else {
            return false;
        };
        match (
            path.values.remove(&first_id),
            path.values.remove(&second_id),
        ) {
            (Some(value), None) => {
                path.values.insert(second_id, value);
                true
            }
            (None, Some(value)) => {
                path.values.insert(first_id, value);
                true
            }
            (first_value, second_value) => {
                if let Some(value) = first_value {
                    path.values.insert(first_id, value);
                }
                if let Some(value) = second_value {
                    path.values.insert(second_id, value);
                }
                false
            }
        }
    }

    fn age_without_one(
        path: &mut PlasticityPath,
        route: &[PhysicalEncounter],
        distractor: PhysicalEncounter,
        withheld: Edge,
    ) -> bool {
        let edges = route_edges(route);
        for index in 0..424u64 {
            if !path.proposals.contains_key(&withheld) {
                return true;
            }
            path.begin_event();
            let keep = route[index as usize % route.len()];
            if keep.edge() != withheld {
                let _ = path.local_encounter(keep);
            }
            let _ = path.local_encounter(distractor);
            for edge in edges.iter().copied().filter(|edge| *edge != withheld) {
                if let Some(record) = path.proposals.get_mut(&edge) {
                    record.life.reused();
                }
            }
        }
        !path.proposals.contains_key(&withheld)
    }

    pub(super) fn run_cell(seed: u64, load: usize, controls: bool) -> super::GateCell {
        let trained = train(seed, load, false);
        let edges = route_edges(&trained.route);
        let mut path = trained.path.clone();
        let heldout_count = heldout(&mut path, &edges, seed);
        let mut economy = path.clone();
        let (route_admissions, distractor_admissions) =
            admission_sweep(&mut economy, &trained.route, &trained.distractors);
        let always_open_admissions = 4 + load;
        let work_reduction = 1.0
            - (route_admissions + distractor_admissions) as f64
                / always_open_admissions as f64;

        let raw_swapped = train(seed + 1_000_000, load, true);
        let mut raw_swapped_path = raw_swapped.path.clone();
        let raw_admissions = admission_sweep(
            &mut raw_swapped_path,
            &raw_swapped.route,
            &raw_swapped.distractors,
        );
        let raw_history_reversal = raw_admissions.0 <= 1 && raw_admissions.1 == load;

        let mut value_shuffled = trained.path.clone();
        let shuffled = shuffle_values(
            &mut value_shuffled,
            trained.route[0].snapshot(),
            trained.distractors[0].snapshot(),
        );
        let shuffled_admissions = admission_sweep(
            &mut value_shuffled,
            &trained.route,
            &trained.distractors,
        );
        let value_shuffle_reversal =
            shuffled && shuffled_admissions.0 <= 1 && shuffled_admissions.1 == load;

        let withheld = edges[0];
        let mut repair = trained.path.clone();
        let mut repair_learner = trained.learner.clone();
        let entry_resistance = repair.proposal_resistance(withheld);
        let removed = age_without_one(
            &mut repair,
            &trained.route,
            trained.distractors[0],
            withheld,
        );
        let final_resistance = repair.proposal_resistance(withheld);
        let stale_blocked = !repair.execute(withheld);
        repair.begin_event();
        let repaired_edge = repair.local_encounter(trained.route[0]);
        let repaired_once = repaired_edge.is_some_and(|edge| {
            repair.execute(edge)
                && repair_learner.apply(
                    &mut repair,
                    trained.route[0].snapshot(),
                    trained.distractors[0].snapshot(),
                    raw_consequence(seed + 900_000, 0, (seed as usize / 500_000) % 4),
                )
        });
        let repaired = if repaired_once {
            heldout(&mut repair, &edges, seed + 2_000_000)
        } else {
            0
        };

        let economy_pass = route_admissions == 4
            && distractor_admissions <= load.div_ceil(8)
            && work_reduction >= 0.50;
        let retained_economy = trained.path.proposals.len() < always_open_admissions
            && edges
                .iter()
                .all(|edge| trained.path.proposals.contains_key(edge));
        let topology_identity_layout = trained.physical_exact
            && raw_swapped.physical_exact
            && trained.route[0].snapshot() == route(seed + 3_000_000, true)[0].snapshot();
        let source_audit = env!("DS8_MICRO_LINKER_FRAGMENT_SHA256")
            == super::FROZEN_LINKER_SHA256
            && env!("DS8_GATE_MICRO_SOURCE_SHA256") == super::FROZEN_MICRO_SOURCE_SHA256
            && env!("DS8_GATE_MICRO_RESULT_SHA256") == super::FROZEN_MICRO_RESULT_SHA256
            && env!("DS8_GATE_MICRO_AUDIT_SHA256") == super::FROZEN_MICRO_AUDIT_SHA256
            && env!("DS8_GATE_HANDOFF_SHA256") == super::FROZEN_HANDOFF_SHA256
            && env!("DS8_GATE_PROTOCOL_SHA256") == super::FROZEN_PROTOCOL_SHA256;
        let cumulative_m5 = controls;
        let passed = trained.blank_acquisition
            && trained.physical_exact
            && heldout_count == 32
            && economy_pass
            && raw_history_reversal
            && value_shuffle_reversal
            && entry_resistance > 0
            && final_resistance == 0
            && removed
            && stale_blocked
            && repaired == 32
            && retained_economy
            && topology_identity_layout
            && controls
            && source_audit
            && cumulative_m5;
        super::GateCell {
            seed,
            load,
            blank_acquisition: trained.blank_acquisition,
            physical_exact: trained.physical_exact,
            heldout: heldout_count,
            heldout_total: 32,
            route_admissions,
            distractor_admissions,
            always_open_admissions,
            work_reduction,
            raw_history_reversal,
            value_shuffle_reversal,
            entry_resistance,
            final_resistance,
            removed,
            stale_blocked,
            repaired,
            repaired_total: 32,
            retained_economy,
            topology_identity_layout,
            controls,
            source_audit,
            cumulative_m5,
            passed,
        }
    }
}

fn run_cells(controls: bool) -> Vec<GateCell> {
    SEEDS
        .iter()
        .copied()
        .flat_map(|seed| {
            LOADS
                .iter()
                .copied()
                .map(move |load| frozen_linker::run_cell(seed, load, controls))
        })
        .collect()
}

pub fn run() -> GateReport {
    let micro = crate::ds8_cumulative_semantic_credit_micro::run();
    let controls = micro.passed;
    let cells = run_cells(controls);
    let duplicate_exact = cells == run_cells(controls);
    let passed = controls
        && duplicate_exact
        && cells.len() == 18
        && cells.iter().all(|cell| cell.passed);
    GateReport {
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
    fn gate_is_duplicate_exact_and_conjunctive() {
        let report = run();
        assert!(report.duplicate_exact);
        assert!(report.cells.iter().all(|cell| cell.passed), "{report:#?}");
        assert!(report.passed);
    }
}
