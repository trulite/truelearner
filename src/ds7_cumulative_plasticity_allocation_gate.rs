//! Development-only cumulative DS7 retained-route GATE.

pub const PROTOCOL: &str = "ds7-cumulative-plasticity-allocation-gate-v2";
pub const PROTOCOL_COMMIT: &str = "12ada39";
pub const AUTHORITATIVE_M4: &str = "8db47281a7c9c97cbb52ced6fc3dcff0e7efa9b2";
pub const SEEDS: [u64; 6] = [
    22_000_000, 22_500_000, 23_000_000, 23_500_000, 24_000_000, 24_500_000,
];
pub const LOADS: [usize; 3] = [8, 32, 128];
pub const FROZEN_HANDOFF_SHA256: &str =
    "e2cd6556b31c15457e6e0accf6b1369d0140378138fc063c417372da09fb3a1c";
pub const FROZEN_MICRO_RESULT_SHA256: &str =
    "fa3d0510e0d821d32b2e3dd8c5bc9f357fca653f6e96f7d9d6b8fa19e61a377b";
pub const FROZEN_MICRO_AUDIT_SHA256: &str =
    "bc4e2cd797fb31133b315d8844301ca9734835176713cced0b049508475e84ff";
pub const FROZEN_MICRO_SHA256: &str =
    "3fd46245614c17476b8fa44887e4a710d0c358ecc4f7ebe7da5fbdb5bb9459ec";
pub const FROZEN_RT0_SHA256: &str =
    "16ef4e2a691e22251d109860ac055c5a1ee78f586ad9335a375589336ad78ed0";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "7f03c573fa6d5ad32f0401a1c8c260a844e9f943998a97fcc9a213a7636b61e3";

#[derive(Clone, Debug, PartialEq)]
pub struct GateCell {
    pub seed: u64,
    pub load: usize,
    pub blank_acquisition: bool,
    pub heldout_correct: usize,
    pub heldout_total: usize,
    pub productive_admissions: usize,
    pub distractor_admissions: usize,
    pub always_open_admissions: usize,
    pub admission_reduction: f64,
    pub shuffled_reversal: bool,
    pub withheld_removed: bool,
    pub stale_blocked: bool,
    pub repaired_correct: usize,
    pub repaired_total: usize,
    pub shuffled_repair_blocked: bool,
    pub retained_economy: bool,
    pub controls: bool,
    pub source_audit: bool,
    pub cumulative_m4: bool,
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
mod frozen_allocator {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds7_cumulative_plasticity_targeting_probe_frozen.rs"
    ));

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

    fn training_sweep(
        path: &mut PlasticityPath,
        route: &[PhysicalEncounter],
        distractors: &[PhysicalEncounter],
    ) -> (usize, usize) {
        path.begin_event();
        let mut distractor_updates = 0;
        let mut route_updates = 0;
        for encounter in distractors {
            if let Some(edge) = path.local_encounter(*encounter) {
                distractor_updates += usize::from(path.execute_and_observe(edge, false));
            }
        }
        for encounter in route {
            if let Some(edge) = path.local_encounter(*encounter) {
                route_updates += usize::from(path.execute_and_observe(edge, true));
            }
        }
        (route_updates, distractor_updates)
    }

    fn retain_training_state(
        path: &mut PlasticityPath,
        route: &[PhysicalEncounter],
        distractors: &[PhysicalEncounter],
    ) {
        for _ in 0..20 {
            path.begin_event();
            if let Some(representative) = distractors.first() {
                if let Some(edge) = path.local_encounter(*representative) {
                    let _ = path.execute_and_observe(edge, false);
                }
            }
            for encounter in route {
                if let Some(edge) = path.local_encounter(*encounter) {
                    let _ = path.execute_and_observe(edge, true);
                }
            }
        }
    }

    fn heldout(path: &mut PlasticityPath, edges: &[Edge], seed: u64) -> usize {
        let mut correct = 0;
        for index in 0..32u64 {
            path.begin_event();
            correct += usize::from(traverse(path, edges, seed + 100_000 + index));
        }
        correct
    }

    fn admission_sweep(
        path: &mut PlasticityPath,
        route: &[PhysicalEncounter],
        distractors: &[PhysicalEncounter],
    ) -> (usize, usize) {
        path.begin_event();
        let productive = route
            .iter()
            .filter(|encounter| path.local_encounter(**encounter).is_some())
            .count();
        let rejected = distractors
            .iter()
            .filter(|encounter| path.local_encounter(**encounter).is_some())
            .count();
        (productive, rejected)
    }

    fn shuffled_reversal(
        trained: &PlasticityPath,
        route: &[PhysicalEncounter],
        distractors: &[PhysicalEncounter],
    ) -> bool {
        let mut shuffled = trained.clone();
        let p = route[0].snapshot();
        let n = distractors[0].snapshot();
        if !shuffled.swap_values(p, n) {
            return false;
        }
        let mut productive_rejected = false;
        for encounter in route.iter().take(2) {
            shuffled.begin_event();
            if shuffled.local_encounter(*encounter).is_none() {
                productive_rejected = true;
                break;
            }
        }
        shuffled.begin_event();
        let distractor_admitted = shuffled.local_encounter(distractors[0]).is_some();
        productive_rejected && distractor_admitted
    }

    fn age_without_one(
        path: &mut PlasticityPath,
        route: &[PhysicalEncounter],
        distractor: PhysicalEncounter,
        withheld: Edge,
    ) -> bool {
        let edges = route_edges(route);
        for index in 0..400u64 {
            if !path.proposals.contains_key(&withheld) {
                return true;
            }
            path.begin_event();
            let keep = route[(index as usize) % route.len()];
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

    fn repair_comparison(
        trained: &PlasticityPath,
        route: &[PhysicalEncounter],
        distractors: &[PhysicalEncounter],
        seed: u64,
    ) -> (bool, bool, usize, bool) {
        let edges = route_edges(route);
        let withheld = edges[0];
        let mut correct = trained.clone();
        let mut shuffled = trained.clone();
        let swapped = shuffled.swap_values(route[0].snapshot(), distractors[0].snapshot());
        let removed_correct = age_without_one(&mut correct, route, distractors[0], withheld);
        let removed_shuffled = age_without_one(&mut shuffled, route, distractors[0], withheld);
        let stale_blocked = !correct.execute(withheld) && !shuffled.execute(withheld);

        correct.begin_event();
        let repaired = correct
            .local_encounter(route[0])
            .is_some_and(|edge| correct.execute_and_observe(edge, true));
        let repaired_correct = if repaired {
            heldout(&mut correct, &edges, seed + 200_000)
        } else {
            0
        };

        if (shuffled.exploration_clock + 1).is_multiple_of(EXPLORATION_PERIOD) {
            shuffled.begin_event();
            let _ = shuffled.local_encounter(route[1]);
        }
        shuffled.begin_event();
        let shuffled_missing = shuffled.local_encounter(route[0]).is_none()
            && !shuffled.proposals.contains_key(&withheld);
        (
            removed_correct && removed_shuffled,
            stale_blocked,
            repaired_correct,
            swapped && shuffled_missing,
        )
    }

    pub(super) fn run_gate_cell(seed: u64, load: usize) -> super::GateCell {
        let route = route(seed, seed % 1_000_000 != 0);
        let distractors = distractors(seed, load, seed % 1_000_000 == 0);
        let edges = route_edges(&route);
        let mut path = PlasticityPath::default();
        let mut first_route = false;
        let mut first_distractor = false;
        let mut route_updates = 0;
        let mut distractor_updates = 0;
        for sweep in 0..8 {
            let (route_count, distractor_count) = training_sweep(&mut path, &route, &distractors);
            if sweep == 0 {
                first_route = route_count == 4;
                first_distractor = distractor_count > 0;
            }
            route_updates += route_count;
            distractor_updates += distractor_count;
        }
        retain_training_state(&mut path, &route, &distractors);
        let blank_acquisition = first_route
            && first_distractor
            && route_updates >= 8
            && distractor_updates >= 2
            && edges.iter().all(|edge| path.proposals.contains_key(edge));

        let repair_base = path.clone();
        let heldout_correct = heldout(&mut path, &edges, seed);
        let mut economy = path.clone();
        let (productive_admissions, distractor_admissions) =
            admission_sweep(&mut economy, &route, &distractors);
        let always_open_admissions = 4 + load;
        let admission_reduction = 1.0
            - (productive_admissions + distractor_admissions) as f64
                / always_open_admissions as f64;
        let economy_pass = productive_admissions == 4
            && distractor_admissions <= (load + 7) / 8
            && admission_reduction >= 0.50;
        let shuffled_reversal = shuffled_reversal(&path, &route, &distractors);
        let (withheld_removed, stale_blocked, repaired_correct, shuffled_repair_blocked) =
            repair_comparison(&repair_base, &route, &distractors, seed);
        let retained_economy = path.proposals.len() < always_open_admissions
            && edges.iter().all(|edge| path.proposals.contains_key(edge))
            && path.prototype_resistance(route[0].snapshot()) > 0
            && path.value_resistance(route[0].snapshot()) > 0;

        let mut control = path.clone();
        control.begin_event();
        let mut inactive = pattern_p(seed + 300_000, seed + 300_001, 10, 11);
        inactive.coactive = false;
        let no_coactivity = control.local_encounter(inactive).is_none();
        let outside_radius = control
            .local_encounter(pattern_p(seed + 300_002, seed + 300_003, 10, 15))
            .is_none();
        let inactive_feedback = !control.delayed_experience(true);
        control.begin_event();
        let fresh_layout = control
            .local_encounter(pattern_p(seed + 300_004, seed + 300_005, 91, 90))
            .is_some();
        let controls = no_coactivity && outside_radius && inactive_feedback && fresh_layout;
        let source_audit = source_audit().passed()
            && env!("DS7_GATE_HANDOFF_SHA256") == super::FROZEN_HANDOFF_SHA256
            && env!("DS7_GATE_MICRO_RESULT_SHA256") == super::FROZEN_MICRO_RESULT_SHA256
            && env!("DS7_GATE_MICRO_AUDIT_SHA256") == super::FROZEN_MICRO_AUDIT_SHA256
            && env!("DS7_GATE_MICRO_SHA256") == super::FROZEN_MICRO_SHA256
            && env!("DS7_GATE_RT0_SHA256") == super::FROZEN_RT0_SHA256
            && env!("DS7_GATE_PROTOCOL_SHA256") == super::FROZEN_PROTOCOL_SHA256;
        let cumulative_m4 = frozen_m4::authority_exact(seed);
        let passed = blank_acquisition
            && heldout_correct == 32
            && economy_pass
            && shuffled_reversal
            && withheld_removed
            && stale_blocked
            && repaired_correct == 32
            && shuffled_repair_blocked
            && retained_economy
            && controls
            && source_audit
            && cumulative_m4;
        super::GateCell {
            seed,
            load,
            blank_acquisition,
            heldout_correct,
            heldout_total: 32,
            productive_admissions,
            distractor_admissions,
            always_open_admissions,
            admission_reduction,
            shuffled_reversal,
            withheld_removed,
            stale_blocked,
            repaired_correct,
            repaired_total: 32,
            shuffled_repair_blocked,
            retained_economy,
            controls,
            source_audit,
            cumulative_m4,
            passed,
        }
    }
}

fn run_cells() -> Vec<GateCell> {
    SEEDS
        .iter()
        .copied()
        .flat_map(|seed| {
            LOADS
                .iter()
                .copied()
                .map(move |load| frozen_allocator::run_gate_cell(seed, load))
        })
        .collect()
}

pub fn run() -> GateReport {
    let cells = run_cells();
    let duplicate_exact = cells == run_cells();
    let passed = duplicate_exact && cells.len() == 18 && cells.iter().all(|cell| cell.passed);
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
