//! Development-only DS7 path-existence probe over authoritative M4.

use std::collections::BTreeMap;

pub const PROTOCOL: &str = "ds7-cumulative-plasticity-allocation-probe-v1";
pub const PROTOCOL_COMMIT: &str = "15dc6fb3affee96b9470717ca9c7b7f97452f643";
pub const AUTHORITATIVE_M4: &str = "8db47281a7c9c97cbb52ced6fc3dcff0e7efa9b2";
pub const PROBE_SEED: u64 = 20_000_000;
pub const FROZEN_ACTIVATION_SHA256: &str =
    "f7290c939c54b78986596e937d4932335f995766f292c613eb522f552bb3e892";
pub const FROZEN_AUDIT_SHA256: &str =
    "40305dd998b5fe80db9d4fcceee154288ea523f90bc57f26079025a7492b2509";
pub const FROZEN_MANIFEST_SHA256: &str =
    "138827ef5e9d761cd7cb58a672ed4a4776618d5eb56f5f4e88852ce18cb67504";
pub const FROZEN_P2_SHA256: &str =
    "704f757888d9b3bc89a5a3f5387f3422efb9dd4c746e3784506411b1da763b15";
pub const FROZEN_M4_SHA256: &str =
    "3d5659fb26ae804dee6122408f9d703ea1f226349772883075a42686ac3fd110";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "78109beadb9b96164d7a259f88cb73710a10fd6cdf773b053922743c5d8c7044";

const LOCAL_RADIUS: u8 = 2;
const REPRESENTATION_THRESHOLD: usize = 4;
const VALUE_THRESHOLD: i32 = 2;
const EXPLORATION_PERIOD: usize = 8;
const PRESSURE_PERIOD: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    pub name: &'static str,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeReport {
    pub protocol: &'static str,
    pub seed: u64,
    pub checks: Vec<Check>,
    pub prototypes: usize,
    pub values: usize,
    pub proposals: usize,
    pub productive_admissions: usize,
    pub unproductive_admissions: usize,
    pub exploration_admissions: usize,
    pub first_collapse: &'static str,
    pub duplicate_exact: bool,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SourceAudit {
    frozen_inputs: bool,
    no_forbidden_fields: bool,
    delayed_boundary: bool,
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.frozen_inputs && self.no_forbidden_fields && self.delayed_boundary
    }
}

// DS7_ORGANISM_PATH_BEGIN

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EndpointTrace {
    arrival_external: u8,
    arrival_queued: u8,
    occupied_local: u8,
    live_incoming: u8,
    live_outgoing: u8,
    resistance: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EncounterSnapshot {
    first: EndpointTrace,
    second: EndpointTrace,
}

impl EncounterSnapshot {
    fn between(first: EndpointTrace, second: EndpointTrace) -> Self {
        let mut endpoints = [first, second];
        endpoints.sort_unstable();
        Self {
            first: endpoints[0],
            second: endpoints[1],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    first: u64,
    second: u64,
}

impl Edge {
    fn between(first: u64, second: u64) -> Self {
        let mut endpoints = [first, second];
        endpoints.sort_unstable();
        Self {
            first: endpoints[0],
            second: endpoints[1],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PhysicalEncounter {
    first_id: u64,
    second_id: u64,
    first: EndpointTrace,
    second: EndpointTrace,
    separation: u8,
    coactive: bool,
}

impl PhysicalEncounter {
    fn snapshot(self) -> EncounterSnapshot {
        EncounterSnapshot::between(self.first, self.second)
    }

    fn edge(self) -> Edge {
        Edge::between(self.first_id, self.second_id)
    }

    fn locally_active(self) -> bool {
        self.coactive && self.separation <= LOCAL_RADIUS
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScalarAllocation {
    resistance: i32,
}

impl ScalarAllocation {
    fn new() -> Self {
        Self { resistance: 1 }
    }

    fn reused(&mut self) {
        self.resistance += 2;
    }

    fn pressured(&mut self) {
        self.resistance -= 1;
    }

    fn live(self) -> bool {
        self.resistance > 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Prototype {
    id: usize,
    observations: usize,
    life: ScalarAllocation,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct EncounterEncoder {
    records: BTreeMap<EncounterSnapshot, Prototype>,
    next_id: usize,
}

impl EncounterEncoder {
    fn observe(&mut self, snapshot: EncounterSnapshot) -> usize {
        if let Some(record) = self.records.get_mut(&snapshot) {
            record.observations += 1;
            record.life.reused();
            return record.id;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.records.insert(
            snapshot,
            Prototype {
                id,
                observations: 1,
                life: ScalarAllocation::new(),
            },
        );
        id
    }

    fn recognized(&self, snapshot: EncounterSnapshot) -> Option<usize> {
        self.records
            .get(&snapshot)
            .filter(|record| record.observations >= REPRESENTATION_THRESHOLD)
            .map(|record| record.id)
    }

    fn pressure(&mut self) -> Vec<usize> {
        for record in self.records.values_mut() {
            record.life.pressured();
        }
        let removed: Vec<_> = self
            .records
            .values()
            .filter(|record| !record.life.live())
            .map(|record| record.id)
            .collect();
        self.records.retain(|_, record| record.life.live());
        removed
    }

    fn resistance(&self, snapshot: EncounterSnapshot) -> i32 {
        self.records
            .get(&snapshot)
            .map_or(0, |record| record.life.resistance)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ValueRecord {
    support: usize,
    rejection: usize,
    life: ScalarAllocation,
}

impl ValueRecord {
    fn new() -> Self {
        Self {
            support: 0,
            rejection: 0,
            life: ScalarAllocation::new(),
        }
    }

    fn score(self) -> i32 {
        self.support as i32 - self.rejection as i32
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProposalRecord {
    representation: usize,
    life: ScalarAllocation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PlasticityPath {
    encoder: EncounterEncoder,
    values: BTreeMap<usize, ValueRecord>,
    proposals: BTreeMap<Edge, ProposalRecord>,
    eligibility: Option<Edge>,
    completed: usize,
    exploration_clock: usize,
    exploration_admissions: usize,
}

impl Default for PlasticityPath {
    fn default() -> Self {
        Self {
            encoder: EncounterEncoder::default(),
            values: BTreeMap::new(),
            proposals: BTreeMap::new(),
            eligibility: None,
            completed: 0,
            exploration_clock: 0,
            exploration_admissions: 0,
        }
    }
}

impl PlasticityPath {
    fn pressure_if_due(&mut self) {
        if self.completed == 0 || !self.completed.is_multiple_of(PRESSURE_PERIOD) {
            return;
        }
        for record in self.proposals.values_mut() {
            record.life.pressured();
        }
        self.proposals.retain(|_, record| record.life.live());
        if self
            .eligibility
            .is_some_and(|edge| !self.proposals.contains_key(&edge))
        {
            self.eligibility = None;
        }

        let removed = self.encoder.pressure();
        for id in removed {
            self.values.remove(&id);
            self.proposals
                .retain(|_, record| record.representation != id);
        }
        for record in self.values.values_mut() {
            record.life.pressured();
        }
        self.values.retain(|_, record| record.life.live());
    }

    fn has_positive_value(&self) -> bool {
        self.values
            .values()
            .any(|record| record.score() >= VALUE_THRESHOLD)
    }

    fn admit(&mut self, representation: usize) -> bool {
        if !self.has_positive_value() {
            return true;
        }
        if let Some(record) = self.values.get_mut(&representation) {
            record.life.reused();
            if record.score() >= VALUE_THRESHOLD {
                return true;
            }
        }
        self.exploration_clock += 1;
        if self.exploration_clock.is_multiple_of(EXPLORATION_PERIOD) {
            self.exploration_admissions += 1;
            return true;
        }
        false
    }

    fn encounter(&mut self, encounter: PhysicalEncounter) -> Option<Edge> {
        self.pressure_if_due();
        self.completed += 1;
        if !encounter.locally_active() {
            return None;
        }

        let snapshot = encounter.snapshot();
        let observed = self.encoder.observe(snapshot);
        let representation = self.encoder.recognized(snapshot).unwrap_or(observed);
        if !self.admit(representation) {
            return None;
        }

        let edge = encounter.edge();
        self.proposals
            .entry(edge)
            .and_modify(|record| record.life.reused())
            .or_insert(ProposalRecord {
                representation,
                life: ScalarAllocation::new(),
            });
        Some(edge)
    }

    fn execute(&mut self, edge: Edge) -> bool {
        let Some(record) = self.proposals.get_mut(&edge) else {
            return false;
        };
        record.life.reused();
        self.eligibility = Some(edge);
        true
    }

    fn delayed_experience(&mut self, supported: bool) -> bool {
        let Some(edge) = self.eligibility.take() else {
            return false;
        };
        let Some(proposal) = self.proposals.get(&edge).copied() else {
            return false;
        };
        let value = self
            .values
            .entry(proposal.representation)
            .or_insert_with(ValueRecord::new);
        value.life.reused();
        if supported {
            value.support += 1;
        } else {
            value.rejection += 1;
        }
        true
    }

    fn execute_and_observe(&mut self, edge: Edge, supported: bool) -> bool {
        self.execute(edge) && self.delayed_experience(supported)
    }

    fn proposal_resistance(&self, edge: Edge) -> i32 {
        self.proposals
            .get(&edge)
            .map_or(0, |record| record.life.resistance)
    }

    fn prototype_resistance(&self, snapshot: EncounterSnapshot) -> i32 {
        self.encoder.resistance(snapshot)
    }

    fn value_resistance(&self, snapshot: EncounterSnapshot) -> i32 {
        self.encoder
            .recognized(snapshot)
            .and_then(|id| self.values.get(&id))
            .map_or(0, |record| record.life.resistance)
    }

    fn swap_values(&mut self, first: EncounterSnapshot, second: EncounterSnapshot) -> bool {
        let Some(first_id) = self.encoder.recognized(first) else {
            return false;
        };
        let Some(second_id) = self.encoder.recognized(second) else {
            return false;
        };
        let Some(first_value) = self.values.remove(&first_id) else {
            return false;
        };
        let Some(second_value) = self.values.remove(&second_id) else {
            self.values.insert(first_id, first_value);
            return false;
        };
        self.values.insert(first_id, second_value);
        self.values.insert(second_id, first_value);
        true
    }
}

// DS7_ORGANISM_PATH_END

#[allow(dead_code)]
mod frozen_m4 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds6_cumulative_lifetime_frozen.rs"
    ));

    pub(super) fn authority_exact(seed: u64) -> bool {
        let cell = frozen_m3::run_gate_cell(seed);
        cell.passed && cell.lifetimes == [1, 3, 6, 13, 27]
    }
}

fn endpoint(
    arrival_external: u8,
    arrival_queued: u8,
    occupied_local: u8,
    live_incoming: u8,
    live_outgoing: u8,
    resistance: i32,
) -> EndpointTrace {
    EndpointTrace {
        arrival_external,
        arrival_queued,
        occupied_local,
        live_incoming,
        live_outgoing,
        resistance,
    }
}

fn pattern_p(
    first_id: u64,
    second_id: u64,
    first_place: u8,
    second_place: u8,
) -> PhysicalEncounter {
    PhysicalEncounter {
        first_id,
        second_id,
        first: endpoint(1, 0, 0, 0, 1, 3),
        second: endpoint(0, 1, 1, 1, 0, 3),
        separation: first_place.abs_diff(second_place),
        coactive: true,
    }
}

fn pattern_n(
    first_id: u64,
    second_id: u64,
    first_place: u8,
    second_place: u8,
) -> PhysicalEncounter {
    PhysicalEncounter {
        first_id,
        second_id,
        first: endpoint(0, 1, 1, 1, 1, 2),
        second: endpoint(0, 1, 0, 1, 0, 2),
        separation: first_place.abs_diff(second_place),
        coactive: true,
    }
}

fn pattern_u(first_id: u64, second_id: u64) -> PhysicalEncounter {
    PhysicalEncounter {
        first_id,
        second_id,
        first: endpoint(1, 0, 2, 0, 0, 1),
        second: endpoint(1, 0, 2, 0, 0, 1),
        separation: 1,
        coactive: true,
    }
}

fn source_audit() -> SourceAudit {
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/ds7_cumulative_plasticity_targeting_probe.rs"
    ));
    let path = source
        .split("// DS7_ORGANISM_PATH_BEGIN")
        .nth(1)
        .and_then(|text| text.split("// DS7_ORGANISM_PATH_END").next())
        .unwrap_or_default();
    let forbidden = [
        "PlasticUnit",
        "EndpointKind",
        "probation_left",
        "consolidated",
        "TEMPORARY",
        "PERMANENT",
        "LEARN_HERE",
        "candidate_list",
        "target_site",
    ];
    SourceAudit {
        frozen_inputs: env!("DS7_ACTIVATION_SHA256") == FROZEN_ACTIVATION_SHA256
            && env!("DS7_AUDIT_SHA256") == FROZEN_AUDIT_SHA256
            && env!("DS7_MANIFEST_SHA256") == FROZEN_MANIFEST_SHA256
            && env!("DS7_P2_SHA256") == FROZEN_P2_SHA256
            && env!("DS7_M4_SHA256") == FROZEN_M4_SHA256
            && env!("DS7_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        no_forbidden_fields: forbidden.iter().all(|word| !path.contains(word)),
        delayed_boundary: path
            .contains("fn encounter(&mut self, encounter: PhysicalEncounter) -> Option<Edge>")
            && path.contains("fn delayed_experience(&mut self, supported: bool) -> bool"),
    }
}

fn check(name: &'static str, passed: bool) -> Check {
    Check { name, passed }
}

fn run_once() -> ProbeReport {
    let mut path = PlasticityPath::default();
    let p_snapshot = pattern_p(1, 2, 10, 11).snapshot();
    let n_snapshot = pattern_n(3, 4, 20, 21).snapshot();
    let u = pattern_u(5, 6);
    let u_snapshot = u.snapshot();
    let u_edge = u.edge();

    let initial_u = path.encounter(u);
    let u_updated = initial_u.is_some_and(|edge| path.execute_and_observe(edge, false));

    let mut n_training = 0;
    for index in 0..4u64 {
        let encounter = pattern_n(100 + 2 * index, 101 + 2 * index, 30, 31);
        if let Some(edge) = path.encounter(encounter) {
            n_training += usize::from(path.execute_and_observe(edge, false));
        }
    }

    let mut p_training = 0;
    for index in 0..4u64 {
        let encounter = pattern_p(200 + 2 * index, 201 + 2 * index, 40, 41);
        if let Some(edge) = path.encounter(encounter) {
            p_training += usize::from(path.execute_and_observe(edge, true));
        }
    }
    let blank_recruited = p_training == 4
        && n_training == 4
        && path.encoder.recognized(p_snapshot).is_some()
        && path.encoder.recognized(n_snapshot).is_some();

    let mut inactive = pattern_p(700, 701, 50, 51);
    inactive.coactive = false;
    let no_coactivity = path.encounter(inactive).is_none();
    let outside_radius = path.encounter(pattern_p(702, 703, 50, 55)).is_none();
    let mut filler = pattern_p(704, 705, 50, 51);
    filler.coactive = false;
    let _ = path.encounter(filler);
    filler.first_id = 706;
    filler.second_id = 707;
    let _ = path.encounter(filler);

    let unused_removed = path.proposal_resistance(u_edge) == 0
        && path.prototype_resistance(u_snapshot) == 0
        && path.value_resistance(u_snapshot) == 0;
    let stale_blocked = !path.execute(u_edge);

    let mut shuffled = path.clone();
    let swap_complete = shuffled.swap_values(p_snapshot, n_snapshot);
    let shuffled_p = shuffled.encounter(pattern_p(800, 801, 70, 71)).is_some();
    let shuffled_n = shuffled.encounter(pattern_n(802, 803, 80, 81)).is_some();

    let fresh_p = path.encounter(pattern_p(900, 901, 91, 90));
    let fresh_n = path.encounter(pattern_n(902, 903, 101, 100));
    let fresh_layout_exact = fresh_p.is_some() && fresh_n.is_none();
    let inactive_feedback_blocked = !path.delayed_experience(true);

    let recurrent_edge = fresh_p.expect("productive learned encounter is admitted");
    let active_only = path.execute(recurrent_edge) && path.delayed_experience(true);
    let recurrent_before = path.proposal_resistance(recurrent_edge);

    let mut reacquired = false;
    for index in 0..16u64 {
        let encounter = pattern_u(5, 6);
        if let Some(edge) = path.encounter(encounter) {
            if edge == u_edge && path.execute(edge) {
                reacquired = true;
                break;
            }
        }
        if index % 2 == 0 {
            if let Some(edge) = path.encounter(pattern_p(900, 901, 91, 90)) {
                let _ = path.execute(edge);
            }
        }
    }
    let recurrent_after = path.proposal_resistance(recurrent_edge);
    let recurrent_survived = recurrent_after > 0 && recurrent_after >= recurrent_before;

    let source = source_audit();
    let m4_exact = frozen_m4::authority_exact(PROBE_SEED);
    let pre_outcome_proposals = initial_u.is_some() && u_updated;
    let selective = fresh_layout_exact;
    let shuffled_control = swap_complete && !shuffled_p && shuffled_n;
    let lifecycle = unused_removed && recurrent_survived;
    let stale_and_reacquired = stale_blocked && reacquired;
    let checks = vec![
        check("blank recurring prototypes", blank_recruited),
        check("proposal before delayed outcome", pre_outcome_proposals),
        check("local coactivity controls", no_coactivity && outside_radius),
        check(
            "active eligibility only",
            inactive_feedback_blocked && active_only,
        ),
        check("learned selective allocation", selective),
        check("shuffled value control", shuffled_control),
        check("fresh identity and layout", fresh_layout_exact),
        check("M4 lifecycle on DS7 allocations", lifecycle),
        check("stale block and reacquisition", stale_and_reacquired),
        check("source and information audit", source.passed()),
        check("unchanged authoritative M4", m4_exact),
    ];
    let first_collapse = checks
        .iter()
        .find(|item| !item.passed)
        .map_or("none", |item| item.name);
    let passed = checks.iter().all(|item| item.passed);
    ProbeReport {
        protocol: PROTOCOL,
        seed: PROBE_SEED,
        checks,
        prototypes: path.encoder.records.len(),
        values: path.values.len(),
        proposals: path.proposals.len(),
        productive_admissions: p_training + usize::from(fresh_p.is_some()),
        unproductive_admissions: n_training + usize::from(fresh_n.is_some()),
        exploration_admissions: path.exploration_admissions,
        first_collapse,
        duplicate_exact: false,
        passed,
    }
}

pub fn run() -> ProbeReport {
    let mut first = run_once();
    let second = run_once();
    first.duplicate_exact = {
        let mut expected = first.clone();
        expected.duplicate_exact = false;
        expected == second
    };
    first
        .checks
        .push(check("duplicate exact", first.duplicate_exact));
    if !first.duplicate_exact {
        first.first_collapse = "duplicate exact";
        first.passed = false;
    }
    first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_is_duplicate_exact_and_conjunctive() {
        let report = run();
        assert!(report.duplicate_exact);
        assert!(report.checks.iter().all(|item| item.passed));
        assert!(report.passed, "first collapse: {}", report.first_collapse);
    }
}
