#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowRef, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [7_100_000, 7_200_000];
const PHASES: std::ops::Range<i64> = 0..10;
const EXPECTED_CASES: usize = 100;
const EXPECTED_ROWS: usize = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Lifetime,
    Reuse,
    IncomingStale,
    OutgoingStale,
    OrphanTopology,
}

impl Family {
    const ALL: [Self; 5] = [
        Self::Lifetime,
        Self::Reuse,
        Self::IncomingStale,
        Self::OutgoingStale,
        Self::OrphanTopology,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Lifetime => "cell_lifetime_phase_invariance",
            Self::Reuse => "cell_slot_generation_reuse",
            Self::IncomingStale => "incoming_stale_arrow",
            Self::OutgoingStale => "outgoing_stale_arrow",
            Self::OrphanTopology => "orphan_topology_reclamation",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    updates: u64,
    proposals: u64,
    arrow_deallocations: u64,
    cell_deallocations: u64,
    qlp: u64,
}

impl WorkTotals {
    fn add(&mut self, work: Work) {
        self.physical = self.physical.saturating_add(work.physical_total());
        self.drive = self.drive.saturating_add(work.drive_deliveries);
        self.modulation = self.modulation.saturating_add(work.modulatory_deliveries);
        self.updates = self.updates.saturating_add(work.local_return_updates);
        self.proposals = self
            .proposals
            .saturating_add(work.local_structural_proposals);
        self.arrow_deallocations = self
            .arrow_deallocations
            .saturating_add(work.physical_deallocations);
        self.cell_deallocations = self
            .cell_deallocations
            .saturating_add(work.cell_deallocations);
        self.qlp = self.qlp.saturating_add(work.qualified_local_traversals);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    markers: Vec<String>,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    crossings: usize,
    final_tick: i64,
    body_hash: String,
    naturally_quiescent: bool,
    checks: Vec<(String, bool)>,
}

impl Observation {
    fn passed(&self) -> bool {
        self.checks.iter().all(|(_, passed)| *passed)
    }

    fn failed_names(&self) -> String {
        self.checks
            .iter()
            .filter(|(_, passed)| !passed)
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join("|")
    }
}

struct World {
    body: PlasticSubstrate,
    origin: i64,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    crossings: usize,
    naturally_quiescent: bool,
}

impl World {
    fn new(root: u64, phase: i64, mechanics: MechanicalConfig, cell_capacity: u32) -> Self {
        let mut body =
            PlasticSubstrate::with_mechanics(ArenaId(root), cell_capacity, 64, mechanics);
        body.set_physical_tracing(true);
        body.advance_time(phase);
        Self {
            body,
            origin: phase,
            trace: Vec::new(),
            work: WorkTotals::default(),
            crossings: 0,
            naturally_quiescent: true,
        }
    }

    fn cell(&mut self, physical_id: u64, position: i32, threshold: i32, resistance: u32) -> CellId {
        self.body.add_cell(CellSpec {
            physical_id,
            position,
            region: 0,
            threshold,
            resistance,
        })
    }

    fn arrow(&mut self, from: CellId, to: CellId, resistance: u32, delay: i64) -> ArrowRef {
        let id = self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay,
            phase: 0,
            coupling: 1,
            resistance,
            mode: TransmissionMode::Drive,
        });
        self.body.arrow_reference(id)
    }

    fn advance_age(&mut self, age: i64) {
        let target = self.origin.saturating_add(age);
        while self.body.clock().tick < target {
            let tick = self.body.clock().tick.saturating_add(1);
            let result = self.body.advance_time_traced(tick);
            self.trace.extend(result.physical_trace);
            self.work.add(result.work);
            self.naturally_quiescent &= result.naturally_quiescent;
        }
    }

    fn pulse(&mut self, target: CellId, age: i64, origin_physical: u64) {
        let result = self.body.arrive(
            &[SpikeInput {
                arrival_tick: self.origin.saturating_add(age),
                phase: 0,
                origin_physical,
                target,
                impulse: 1,
            }],
            i16::MAX,
        );
        self.crossings = self.crossings.saturating_add(result.crossings.len());
        self.trace.extend(result.physical_trace);
        self.work.add(result.work);
        self.naturally_quiescent &= result.naturally_quiescent;
    }

    fn death_age(&self, id: CellId) -> Option<i64> {
        self.trace.iter().find_map(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::CellDeallocate { cell, .. } if cell == id
            )
            .then_some(transition.tick.saturating_sub(self.origin))
        })
    }

    fn fire_count(&self, id: CellId) -> usize {
        self.trace
            .iter()
            .filter(
                |transition| matches!(transition.event, PhysicalEvent::Fire { cell } if cell == id),
            )
            .count()
    }

    fn finish(self, markers: Vec<String>, checks: Vec<(String, bool)>) -> Observation {
        Observation {
            markers,
            trace: self.trace,
            work: self.work,
            crossings: self.crossings,
            final_tick: self.body.clock().tick,
            body_hash: ContentHash::of(&self.body.canonical_body_bytes(1).unwrap()).to_string(),
            naturally_quiescent: self.naturally_quiescent,
            checks,
        }
    }
}

fn observe(family: Family, root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    match family {
        Family::Lifetime => observe_lifetime(root, phase, mechanics),
        Family::Reuse => observe_reuse(root, phase, mechanics),
        Family::IncomingStale => observe_incoming_stale(root, phase, mechanics),
        Family::OutgoingStale => observe_outgoing_stale(root, phase, mechanics),
        Family::OrphanTopology => observe_orphan(root, phase, mechanics),
    }
}

fn observe_lifetime(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, 8);
    let r1 = world.cell(root + 1, 0, 10, 1);
    let r2 = world.cell(root + 2, 10, 10, 2);
    let r4 = world.cell(root + 3, 20, 10, 4);
    world.advance_age(9);
    let before = [r1, r2, r4].map(|id| {
        (
            world.body.cell_is_live(id),
            world.body.cell_resistance(id),
            world.body.cell_decay_load(id),
        )
    });
    world.advance_age(40);
    let deaths = [r1, r2, r4].map(|id| world.death_age(id));
    let markers = vec![format!("before={before:?}"), format!("deaths={deaths:?}")];
    let checks = vec![
        (
            "all_live_at_age_9".into(),
            before.iter().all(|state| state.0 == Some(true)),
        ),
        ("r1_death_age_10".into(), deaths[0] == Some(10)),
        ("r2_death_age_20".into(), deaths[1] == Some(20)),
        ("r4_death_age_40".into(), deaths[2] == Some(40)),
        (
            "three_cell_deallocations".into(),
            world.work.cell_deallocations == 3,
        ),
        ("natural_quiescence".into(), world.naturally_quiescent),
    ];
    world.finish(markers, checks)
}

fn observe_reuse(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, 1);
    let old = world.cell(root + 11, 0, 1, 1);
    let old_ref = world.body.cell_reference(old);
    let old_slot = world.body.cell_resident_slot(old);
    world.advance_age(10);
    let dead_live = world.body.cell_is_live(old);
    let dead_generation = world.body.cell_generation(old);
    let old_resolves_after_death = world.body.resolve_cell(old_ref);
    let new = world.cell(root + 12, 0, 1, 4);
    let new_ref = world.body.cell_reference(new);
    let new_slot = world.body.cell_resident_slot(new);
    let old_resolves_after_reuse = world.body.resolve_cell(old_ref);
    let markers = vec![format!(
        "old={old:?}/{old_ref:?}/{old_slot:?};dead={dead_live:?}/{dead_generation:?};new={new:?}/{new_ref:?}/{new_slot:?}"
    )];
    let checks = vec![
        ("old_died".into(), dead_live == Some(false)),
        (
            "old_reference_stale_at_death".into(),
            old_resolves_after_death.is_none(),
        ),
        ("fresh_cell_id".into(), new != old),
        ("resident_slot_reused".into(), new_slot == old_slot),
        (
            "generation_advanced".into(),
            new_ref.generation.0 == old_ref.generation.0.saturating_add(1),
        ),
        (
            "old_reference_stale_after_reuse".into(),
            old_resolves_after_reuse.is_none(),
        ),
        (
            "new_reference_resolves".into(),
            world.body.resolve_cell(new_ref) == new_slot,
        ),
        (
            "one_cell_deallocation".into(),
            world.work.cell_deallocations == 1,
        ),
    ];
    world.finish(markers, checks)
}

fn observe_incoming_stale(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, 2);
    let source = world.cell(root + 21, -100, 1, 100);
    let old_target = world.cell(root + 22, 100, 1, 1);
    let old_ref = world.body.cell_reference(old_target);
    let old_slot = world.body.cell_resident_slot(old_target);
    let incoming = world.arrow(source, old_target, 4, 1);
    world.advance_age(10);
    let replacement = world.cell(root + 23, 100, 1, 100);
    let replacement_ref = world.body.cell_reference(replacement);
    let replacement_slot = world.body.cell_resident_slot(replacement);
    let arrow_live_before = world.body.resolve_arrow(incoming).is_some();
    world.pulse(source, 10, root + 24);
    let replacement_fires = world.fire_count(replacement);
    let deliveries_to_replacement = world
        .trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::Deliver { target, .. } if target == replacement
            )
        })
        .count();
    world.advance_age(40);
    let markers = vec![format!(
        "old={old_ref:?}/{old_slot:?};replacement={replacement_ref:?}/{replacement_slot:?};incoming_live_at_reuse={arrow_live_before};replacement_fires={replacement_fires}"
    )];
    let checks = vec![
        ("target_slot_reused".into(), replacement_slot == old_slot),
        (
            "old_reference_stale".into(),
            world.body.resolve_cell(old_ref).is_none(),
        ),
        (
            "incoming_arrow_survived_cell_death".into(),
            arrow_live_before,
        ),
        (
            "stale_incoming_no_delivery".into(),
            deliveries_to_replacement == 0,
        ),
        ("replacement_not_fired".into(), replacement_fires == 0),
        ("stale_incoming_no_crossing".into(), world.crossings == 0),
        (
            "incoming_arrow_died_independently".into(),
            world.body.resolve_arrow(incoming).is_none(),
        ),
    ];
    world.finish(markers, checks)
}

fn observe_outgoing_stale(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, 2);
    let old_source = world.cell(root + 31, -100, 1, 1);
    let sink = world.cell(root + 32, 100, 1, 100);
    let old_ref = world.body.cell_reference(old_source);
    let old_slot = world.body.cell_resident_slot(old_source);
    let outgoing = world.arrow(old_source, sink, 4, 1);
    world.advance_age(10);
    let replacement = world.cell(root + 33, -100, 1, 100);
    let replacement_ref = world.body.cell_reference(replacement);
    let replacement_slot = world.body.cell_resident_slot(replacement);
    let arrow_live_before = world.body.resolve_arrow(outgoing).is_some();
    world.pulse(replacement, 10, root + 34);
    let sink_fires = world.fire_count(sink);
    let stale_qlp = world
        .trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::QualifiedLocalTraversal { arrow } if arrow == outgoing.id
            )
        })
        .count();
    world.advance_age(40);
    let markers = vec![format!(
        "old={old_ref:?}/{old_slot:?};replacement={replacement_ref:?}/{replacement_slot:?};outgoing_live_at_reuse={arrow_live_before};sink_fires={sink_fires}"
    )];
    let checks = vec![
        ("source_slot_reused".into(), replacement_slot == old_slot),
        (
            "old_reference_stale".into(),
            world.body.resolve_cell(old_ref).is_none(),
        ),
        (
            "outgoing_arrow_survived_cell_death".into(),
            arrow_live_before,
        ),
        (
            "replacement_fired_once".into(),
            world.fire_count(replacement) == 1,
        ),
        ("stale_outgoing_no_sink_fire".into(), sink_fires == 0),
        ("stale_outgoing_no_crossing".into(), world.crossings == 0),
        ("stale_outgoing_no_qlp".into(), stale_qlp == 0),
        (
            "outgoing_arrow_died_independently".into(),
            world.body.resolve_arrow(outgoing).is_none(),
        ),
    ];
    world.finish(markers, checks)
}

fn observe_orphan(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics, 3);
    let source = world.cell(root + 41, -100, 10, 100);
    let contact = world.cell(root + 42, 0, 10, 1);
    let sink = world.cell(root + 43, 100, 10, 100);
    let contact_slot = world.body.cell_resident_slot(contact);
    let incoming = world.arrow(source, contact, 2, 1);
    let outgoing = world.arrow(contact, sink, 2, 1);
    let incoming_slot = world.body.resolve_arrow(incoming);
    let outgoing_slot = world.body.resolve_arrow(outgoing);
    world.advance_age(10);
    let arrows_live_at_cell_death = world.body.resolve_arrow(incoming).is_some()
        && world.body.resolve_arrow(outgoing).is_some();
    let replacement = world.cell(root + 44, 0, 10, 100);
    let replacement_slot = world.body.cell_resident_slot(replacement);
    world.advance_age(19);
    let arrows_live_at_19 = world.body.resolve_arrow(incoming).is_some()
        && world.body.resolve_arrow(outgoing).is_some();
    world.advance_age(20);
    let arrows_dead_at_20 = world.body.resolve_arrow(incoming).is_none()
        && world.body.resolve_arrow(outgoing).is_none();
    let new_incoming = world.arrow(source, replacement, 4, 1);
    let new_outgoing = world.arrow(replacement, sink, 4, 1);
    let reused_arrow_slots = [
        world.body.resolve_arrow(new_incoming),
        world.body.resolve_arrow(new_outgoing),
    ];
    let mut old_slots = [incoming_slot, outgoing_slot];
    let mut new_slots = reused_arrow_slots;
    old_slots.sort_by_key(|slot| slot.map_or(usize::MAX, |slot| slot.0));
    new_slots.sort_by_key(|slot| slot.map_or(usize::MAX, |slot| slot.0));
    let markers = vec![format!(
        "contact_slot={contact_slot:?};replacement_slot={replacement_slot:?};old_arrow_slots={old_slots:?};new_arrow_slots={new_slots:?}"
    )];
    let checks = vec![
        (
            "contact_died_at_10".into(),
            world.death_age(contact) == Some(10),
        ),
        (
            "cell_slot_reclaimed".into(),
            replacement_slot == contact_slot,
        ),
        ("no_cascade_delete".into(), arrows_live_at_cell_death),
        ("incident_arrows_live_at_19".into(), arrows_live_at_19),
        ("incident_arrows_die_at_20".into(), arrows_dead_at_20),
        ("arrow_slots_reused".into(), old_slots == new_slots),
        (
            "one_cell_deallocation".into(),
            world.work.cell_deallocations == 1,
        ),
        (
            "two_arrow_deallocations".into(),
            world.work.arrow_deallocations == 2,
        ),
        ("natural_quiescence".into(), world.naturally_quiescent),
    ];
    world.finish(markers, checks)
}

fn mechanics_name(config: MechanicalConfig) -> &'static str {
    if config == MechanicalConfig::REFERENCE {
        "reference"
    } else if config == MechanicalConfig::PRODUCTION {
        "production"
    } else {
        "unknown"
    }
}

fn main() {
    let output_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/cl0_ordinary_cell_lifetime_v1"));
    fs::create_dir_all(&output_dir).unwrap();

    let mut csv = String::from(
        "case,family,root,phase,mechanics,replay_equal,cross_mechanics_equal,checks_pass,failed,cell_deallocations,arrow_deallocations,physical_work,trace_len,crossings,final_tick,naturally_quiescent,body_hash,markers\n",
    );
    let mut cases = 0usize;
    let mut rows = 0usize;
    let mut clauses = 0usize;
    let mut passed_clauses = 0usize;
    let mut all_pass = true;
    let mut maximum_work = 0u64;

    for root in ROOTS {
        for phase in PHASES {
            for family in Family::ALL {
                cases += 1;
                let reference = observe(family, root, phase, MechanicalConfig::REFERENCE);
                let reference_replay = observe(family, root, phase, MechanicalConfig::REFERENCE);
                let production = observe(family, root, phase, MechanicalConfig::PRODUCTION);
                let production_replay = observe(family, root, phase, MechanicalConfig::PRODUCTION);
                let cross_equal = reference == production;
                for (config, observation, replay) in [
                    (MechanicalConfig::REFERENCE, &reference, &reference_replay),
                    (
                        MechanicalConfig::PRODUCTION,
                        &production,
                        &production_replay,
                    ),
                ] {
                    rows += 1;
                    let replay_equal = observation == replay;
                    let check_count = observation.checks.len().saturating_add(3);
                    let row_pass = observation.passed()
                        && replay_equal
                        && cross_equal
                        && observation.naturally_quiescent;
                    clauses = clauses.saturating_add(check_count);
                    passed_clauses = passed_clauses.saturating_add(
                        observation.checks.iter().filter(|(_, pass)| *pass).count()
                            + usize::from(replay_equal)
                            + usize::from(cross_equal)
                            + usize::from(observation.naturally_quiescent),
                    );
                    all_pass &= row_pass;
                    maximum_work = maximum_work.max(observation.work.physical);
                    let markers = observation.markers.join("|").replace(',', ";");
                    writeln!(
                        csv,
                        "{cases},{},{root},{phase},{},{replay_equal},{cross_equal},{},{},{},{},{},{},{},{},{},{},{}",
                        family.name(),
                        mechanics_name(config),
                        observation.passed(),
                        observation.failed_names(),
                        observation.work.cell_deallocations,
                        observation.work.arrow_deallocations,
                        observation.work.physical,
                        observation.trace.len(),
                        observation.crossings,
                        observation.final_tick,
                        observation.naturally_quiescent,
                        observation.body_hash,
                        markers,
                    )
                    .unwrap();
                }
            }
        }
    }

    assert_eq!(cases, EXPECTED_CASES);
    assert_eq!(rows, EXPECTED_ROWS);
    let report = format!(
        "# CL0 ordinary CELL lifetime Gates 1-8\n\n- cases: {cases}/{EXPECTED_CASES}\n- rows: {rows}/{EXPECTED_ROWS}\n- clauses: {passed_clauses}/{clauses}\n- Reference/Production exact: {all_pass}\n- replay exact: {all_pass}\n- natural quiescence: {all_pass}\n- maximum total work: {maximum_work}\n- Gate 9: pending independent static audit\n"
    );
    fs::write(output_dir.join("matrix.csv"), csv).unwrap();
    fs::write(output_dir.join("report.md"), report).unwrap();
    assert!(all_pass, "CL0 Gates 1-8 failed");
    println!("CL0_ORDINARY_CELL_LIFETIME_GATES_1_8_POSITIVE_V1");
}
