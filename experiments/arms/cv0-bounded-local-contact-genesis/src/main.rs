#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowRef, ArrowSpec, CellId, CellRef, CellSlot, ContentHash, MechanicalConfig,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [7_500_000, 7_600_001];
const PHASES: std::ops::Range<i64> = 0..10;
const EXPECTED_CASES: usize = 100;
const EXPECTED_ROWS: usize = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Symmetry,
    NoConsequence,
    BoundedCreation,
    OrphanReuse,
    PositiveConsequence,
}

impl Family {
    const ALL: [Self; 5] = [
        Self::Symmetry,
        Self::NoConsequence,
        Self::BoundedCreation,
        Self::OrphanReuse,
        Self::PositiveConsequence,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Symmetry => "gate_a_sign_symmetry",
            Self::NoConsequence => "gate_b_no_consequence",
            Self::BoundedCreation => "gate_c_bounded_creation",
            Self::OrphanReuse => "gate_d_orphan_cleanup_reuse",
            Self::PositiveConsequence => "gate_e_positive_consequence",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    arrow_updates: u64,
    cell_updates: u64,
    arrow_proposals: u64,
    cell_proposals: u64,
    arrow_deallocations: u64,
    cell_deallocations: u64,
}

impl WorkTotals {
    fn add(&mut self, work: Work) {
        self.physical = self.physical.saturating_add(work.physical_total());
        self.drive = self.drive.saturating_add(work.drive_deliveries);
        self.modulation = self.modulation.saturating_add(work.modulatory_deliveries);
        self.arrow_updates = self.arrow_updates.saturating_add(work.local_return_updates);
        self.cell_updates = self.cell_updates.saturating_add(work.cell_return_updates);
        self.arrow_proposals = self
            .arrow_proposals
            .saturating_add(work.local_structural_proposals);
        self.cell_proposals = self
            .cell_proposals
            .saturating_add(work.local_cell_proposals);
        self.arrow_deallocations = self
            .arrow_deallocations
            .saturating_add(work.physical_deallocations);
        self.cell_deallocations = self
            .cell_deallocations
            .saturating_add(work.cell_deallocations);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    markers: Vec<String>,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
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

#[derive(Clone, Copy, Debug)]
struct Candidate {
    contact: CellId,
    contact_ref: CellRef,
    contact_slot: CellSlot,
    sign: i32,
    stem: ArrowRef,
    outgoing: ArrowRef,
}

struct World {
    body: PlasticSubstrate,
    origin: i64,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    naturally_quiescent: bool,
    source: CellId,
    target: CellId,
}

impl World {
    fn new(root: u64, phase: i64, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 16, 32, mechanics);
        body.set_physical_tracing(true);
        body.advance_time(phase);
        let source = body.add_cell(truelearner_core::CellSpec {
            physical_id: root + 1,
            position: 0,
            region: 0,
            threshold: 1,
            resistance: 500,
        });
        let target = body.add_cell(truelearner_core::CellSpec {
            physical_id: root + 2,
            position: 1,
            region: 0,
            threshold: 100,
            resistance: 500,
        });
        Self {
            body,
            origin: phase,
            trace: Vec::new(),
            work: WorkTotals::default(),
            naturally_quiescent: true,
            source,
            target,
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
        self.trace.extend(result.physical_trace);
        self.work.add(result.work);
        self.naturally_quiescent &= result.naturally_quiescent;
    }

    fn advance_age(&mut self, age: i64) {
        let target = self.origin.saturating_add(age);
        while self.body.clock().tick < target {
            let result = self
                .body
                .advance_time_traced(self.body.clock().tick.saturating_add(1));
            self.trace.extend(result.physical_trace);
            self.work.add(result.work);
            self.naturally_quiescent &= result.naturally_quiescent;
        }
    }

    fn create_candidates(&mut self, root: u64) -> [Candidate; 2] {
        self.pulse(self.source, 0, root + 10);
        self.candidates()
    }

    fn candidates(&self) -> [Candidate; 2] {
        let body = self.body.arena_body(1);
        let mut candidates = body
            .cells
            .iter()
            .filter(|cell| cell.id != self.source && cell.id != self.target)
            .filter_map(|cell| {
                let stem = body
                    .arrows
                    .iter()
                    .find(|arrow| arrow.from.id == self.source && arrow.to.id == cell.id)?;
                let outgoing = body
                    .arrows
                    .iter()
                    .find(|arrow| arrow.from.id == cell.id && arrow.to.id == self.target)?;
                Some(Candidate {
                    contact: cell.id,
                    contact_ref: self.body.cell_reference(cell.id),
                    contact_slot: self.body.cell_resident_slot(cell.id)?,
                    sign: outgoing.coupling,
                    stem: self.body.arrow_reference(stem.id),
                    outgoing: self.body.arrow_reference(outgoing.id),
                })
            })
            .collect::<Vec<_>>();
        candidates.sort_by_key(|candidate| candidate.sign);
        candidates.try_into().expect("exactly two signed contacts")
    }

    fn add_modulation_to(&mut self, root: u64, target: CellId) -> CellId {
        let modulator = self.body.add_cell(truelearner_core::CellSpec {
            physical_id: root + 100,
            position: 100,
            region: 0,
            threshold: 1,
            resistance: 500,
        });
        self.body.add_arrow(ArrowSpec {
            from: modulator,
            to: target,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 500,
            mode: TransmissionMode::Modulatory,
        });
        modulator
    }

    fn relation_live(&self, candidate: Candidate) -> (bool, bool, bool) {
        (
            self.body.resolve_cell(candidate.contact_ref).is_some(),
            self.body.resolve_arrow(candidate.stem).is_some(),
            self.body.resolve_arrow(candidate.outgoing).is_some(),
        )
    }

    fn finish(self, markers: Vec<String>, checks: Vec<(String, bool)>) -> Observation {
        Observation {
            markers,
            trace: self.trace,
            work: self.work,
            final_tick: self.body.clock().tick,
            body_hash: ContentHash::of(&self.body.canonical_body_bytes(1).unwrap()).to_string(),
            naturally_quiescent: self.naturally_quiescent,
            checks,
        }
    }
}

fn observe(family: Family, root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    match family {
        Family::Symmetry => observe_symmetry(root, phase, mechanics),
        Family::NoConsequence => observe_no_consequence(root, phase, mechanics),
        Family::BoundedCreation => observe_bounded(root, phase, mechanics),
        Family::OrphanReuse => observe_orphan_reuse(root, phase, mechanics),
        Family::PositiveConsequence => observe_positive(root, phase, mechanics),
    }
}

fn symmetry_checks(world: &World, negative: Candidate, positive: Candidate) -> Vec<(String, bool)> {
    let body = world.body.arena_body(1);
    let neg_cell = body
        .cells
        .iter()
        .find(|cell| cell.id == negative.contact)
        .unwrap();
    let pos_cell = body
        .cells
        .iter()
        .find(|cell| cell.id == positive.contact)
        .unwrap();
    let neg_stem = body
        .arrows
        .iter()
        .find(|arrow| arrow.id == negative.stem.id)
        .unwrap();
    let pos_stem = body
        .arrows
        .iter()
        .find(|arrow| arrow.id == positive.stem.id)
        .unwrap();
    let neg_out = body
        .arrows
        .iter()
        .find(|arrow| arrow.id == negative.outgoing.id)
        .unwrap();
    let pos_out = body
        .arrows
        .iter()
        .find(|arrow| arrow.id == positive.outgoing.id)
        .unwrap();
    vec![
        (
            "exactly_two_cell_proposals".into(),
            world.work.cell_proposals == 2,
        ),
        (
            "exactly_four_arrow_proposals".into(),
            world.work.arrow_proposals == 4,
        ),
        (
            "signed_pair".into(),
            negative.sign == -1 && positive.sign == 1,
        ),
        (
            "contact_cell_symmetry".into(),
            neg_cell.position == pos_cell.position
                && neg_cell.region == pos_cell.region
                && neg_cell.threshold == pos_cell.threshold
                && neg_cell.resistance == pos_cell.resistance
                && neg_cell.generation == pos_cell.generation,
        ),
        (
            "stem_symmetry".into(),
            neg_stem.coupling == pos_stem.coupling
                && neg_stem.delay == pos_stem.delay
                && neg_stem.phase == pos_stem.phase
                && neg_stem.resistance == pos_stem.resistance,
        ),
        (
            "outgoing_differs_only_by_sign".into(),
            neg_out.delay == pos_out.delay
                && neg_out.phase == pos_out.phase
                && neg_out.resistance == pos_out.resistance
                && neg_out.transmission_mode == pos_out.transmission_mode
                && neg_out.coupling == -pos_out.coupling,
        ),
        ("both_contacts_participated".into(), {
            world
                .body
                .cell_participation(negative.contact)
                .is_some_and(|v| v > 0)
                && world
                    .body
                    .cell_participation(positive.contact)
                    .is_some_and(|v| v > 0)
        }),
        (
            "no_cell_update_from_use".into(),
            world.work.cell_updates == 0,
        ),
    ]
}

fn observe_symmetry(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let [negative, positive] = world.create_candidates(root);
    let checks = symmetry_checks(&world, negative, positive);
    let markers = vec![format!(
        "negative={:?}/{:?};positive={:?}/{:?}",
        negative.contact, negative.contact_slot, positive.contact, positive.contact_slot
    )];
    world.finish(markers, checks)
}

fn observe_no_consequence(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let [negative, positive] = world.create_candidates(root);
    world.advance_age(10);
    let negative_live = world.relation_live(negative);
    let positive_live = world.relation_live(positive);
    let markers = vec![format!(
        "negative_live={negative_live:?};positive_live={positive_live:?}"
    )];
    let checks = vec![
        (
            "negative_relation_reclaimed".into(),
            negative_live == (false, false, false),
        ),
        (
            "positive_relation_reclaimed".into(),
            positive_live == (false, false, false),
        ),
        (
            "two_contact_deallocations".into(),
            world.work.cell_deallocations == 2,
        ),
        (
            "four_arrow_deallocations".into(),
            world.work.arrow_deallocations == 4,
        ),
        ("no_cell_updates".into(), world.work.cell_updates == 0),
        ("no_arrow_updates".into(), world.work.arrow_updates == 0),
    ];
    world.finish(markers, checks)
}

fn observe_bounded(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let [negative, positive] = world.create_candidates(root);
    for age in [2, 4, 6, 8] {
        world.pulse(world.source, age, root + 20 + u64::try_from(age).unwrap());
    }
    let before = (world.work.cell_proposals, world.work.arrow_proposals);
    world.advance_age(10);
    let markers = vec![format!("proposals={before:?}")];
    let checks = vec![
        ("one_pair_only".into(), before == (2, 4)),
        (
            "negative_eventually_reclaimed".into(),
            world.relation_live(negative) == (false, false, false),
        ),
        (
            "positive_eventually_reclaimed".into(),
            world.relation_live(positive) == (false, false, false),
        ),
        (
            "bounded_cell_deallocation".into(),
            world.work.cell_deallocations == 2,
        ),
        (
            "bounded_arrow_deallocation".into(),
            world.work.arrow_deallocations == 4,
        ),
    ];
    world.finish(markers, checks)
}

fn observe_orphan_reuse(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let [negative, positive] = world.create_candidates(root);
    let mut old_cell_slots = [negative.contact_slot, positive.contact_slot];
    let mut old_arrow_slots = [
        world.body.resolve_arrow(negative.stem),
        world.body.resolve_arrow(negative.outgoing),
        world.body.resolve_arrow(positive.stem),
        world.body.resolve_arrow(positive.outgoing),
    ];
    old_cell_slots.sort_by_key(|slot| slot.0);
    old_arrow_slots.sort_by_key(|slot| slot.map_or(usize::MAX, |slot| slot.0));
    world.advance_age(10);
    let replacement_a = world.body.add_cell(truelearner_core::CellSpec {
        physical_id: root + 200,
        position: 0,
        region: 0,
        threshold: 1,
        resistance: 10,
    });
    let replacement_b = world.body.add_cell(truelearner_core::CellSpec {
        physical_id: root + 201,
        position: 0,
        region: 0,
        threshold: 1,
        resistance: 10,
    });
    let mut new_cell_slots = [
        world.body.cell_resident_slot(replacement_a).unwrap(),
        world.body.cell_resident_slot(replacement_b).unwrap(),
    ];
    new_cell_slots.sort_by_key(|slot| slot.0);
    let new_arrows = [
        (world.source, replacement_a),
        (replacement_a, world.target),
        (world.source, replacement_b),
        (replacement_b, world.target),
    ]
    .map(|(from, to)| {
        let id = world.body.add_arrow(ArrowSpec {
            from,
            to,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 10,
            mode: TransmissionMode::Drive,
        });
        world.body.resolve_arrow(world.body.arrow_reference(id))
    });
    let mut new_arrow_slots = new_arrows;
    new_arrow_slots.sort_by_key(|slot| slot.map_or(usize::MAX, |slot| slot.0));
    let markers = vec![format!(
        "cell_slots={old_cell_slots:?}->{new_cell_slots:?};arrow_slots={old_arrow_slots:?}->{new_arrow_slots:?}"
    )];
    let checks = vec![
        (
            "contact_slots_reused".into(),
            old_cell_slots == new_cell_slots,
        ),
        (
            "arrow_slots_reused".into(),
            old_arrow_slots == new_arrow_slots,
        ),
        (
            "old_negative_ref_stale".into(),
            world.body.resolve_cell(negative.contact_ref).is_none(),
        ),
        (
            "old_positive_ref_stale".into(),
            world.body.resolve_cell(positive.contact_ref).is_none(),
        ),
        (
            "replacement_a_resolves".into(),
            world.body.cell_is_live(replacement_a) == Some(true),
        ),
        (
            "replacement_b_resolves".into(),
            world.body.cell_is_live(replacement_b) == Some(true),
        ),
    ];
    world.finish(markers, checks)
}

fn observe_positive(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let [negative, positive] = world.create_candidates(root);
    let modulator = world.add_modulation_to(root, positive.contact);
    world.pulse(modulator, 2, root + 101);
    let body_after = world.body.arena_body(1);
    let positive_contact_r = world.body.cell_resistance(positive.contact);
    let negative_contact_r = world.body.cell_resistance(negative.contact);
    let positive_stem_r = body_after
        .arrows
        .iter()
        .find(|arrow| arrow.id == positive.stem.id)
        .map(|arrow| arrow.resistance);
    let positive_out_r = body_after
        .arrows
        .iter()
        .find(|arrow| arrow.id == positive.outgoing.id)
        .map(|arrow| arrow.resistance);
    let negative_out_r = body_after
        .arrows
        .iter()
        .find(|arrow| arrow.id == negative.outgoing.id)
        .map(|arrow| arrow.resistance);
    world.advance_age(10);
    let positive_live = world.relation_live(positive);
    let negative_live = world.relation_live(negative);
    let markers = vec![format!(
        "positive_r=cell{positive_contact_r:?}/stem{positive_stem_r:?}/out{positive_out_r:?};negative_r=cell{negative_contact_r:?}/out{negative_out_r:?};live10=positive{positive_live:?}/negative{negative_live:?}"
    )];
    let checks = vec![
        (
            "positive_contact_consolidated".into(),
            positive_contact_r == Some(4),
        ),
        (
            "positive_outgoing_consolidated".into(),
            positive_out_r == Some(4),
        ),
        (
            "negative_contact_not_consolidated".into(),
            negative_contact_r == Some(1),
        ),
        (
            "negative_outgoing_not_consolidated".into(),
            negative_out_r == Some(1),
        ),
        (
            "positive_stem_consolidated".into(),
            positive_stem_r.is_some_and(|r| r > 1),
        ),
        (
            "negative_relation_reclaimed".into(),
            negative_live == (false, false, false),
        ),
        (
            "positive_relation_remains_executable".into(),
            positive_live == (true, true, true),
        ),
        ("one_cell_update".into(), world.work.cell_updates == 1),
        ("one_arrow_update".into(), world.work.arrow_updates == 1),
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
        .unwrap_or_else(|| PathBuf::from("experiments/results/cv0_bounded_contact_resume_v1"));
    fs::create_dir_all(&output_dir).unwrap();
    let mut csv = String::from(
        "case,family,root,phase,mechanics,replay_equal,cross_mechanics_equal,checks_pass,failed,cell_proposals,arrow_proposals,cell_updates,arrow_updates,cell_deallocations,arrow_deallocations,physical_work,trace_len,final_tick,naturally_quiescent,body_hash,markers\n",
    );
    let mut cases = 0usize;
    let mut rows = 0usize;
    let mut clauses = 0usize;
    let mut passed_clauses = 0usize;
    let mut all_pass = true;
    let mut gates_a_d_pass = true;
    let mut gate_e_pass = true;
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
                let family_pass = reference.passed() && production.passed() && cross_equal;
                if family == Family::PositiveConsequence {
                    gate_e_pass &= family_pass;
                } else {
                    gates_a_d_pass &= family_pass;
                }
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
                    writeln!(
                        csv,
                        "{cases},{},{root},{phase},{},{replay_equal},{cross_equal},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        family.name(),
                        mechanics_name(config),
                        observation.passed(),
                        observation.failed_names(),
                        observation.work.cell_proposals,
                        observation.work.arrow_proposals,
                        observation.work.cell_updates,
                        observation.work.arrow_updates,
                        observation.work.cell_deallocations,
                        observation.work.arrow_deallocations,
                        observation.work.physical,
                        observation.trace.len(),
                        observation.final_tick,
                        observation.naturally_quiescent,
                        observation.body_hash,
                        observation.markers.join("|").replace(',', ";"),
                    )
                    .unwrap();
                }
            }
        }
    }

    assert_eq!(cases, EXPECTED_CASES);
    assert_eq!(rows, EXPECTED_ROWS);
    let report = format!(
        "# CV0 bounded local contact genesis resumed matrix\n\n- cases: {cases}/{EXPECTED_CASES}\n- rows: {rows}/{EXPECTED_ROWS}\n- clauses: {passed_clauses}/{clauses}\n- Gates A-D: {}\n- Gate E: {}\n- Reference/Production exact: {}\n- replay exact: {}\n- natural quiescence: {}\n- maximum PhysicalWork: {maximum_work}\n",
        if gates_a_d_pass { "PASS" } else { "FAIL" },
        if gate_e_pass { "PASS" } else { "FAIL" },
        reference_production_exact(&csv),
        replay_exact(&csv),
        quiescence_exact(&csv),
    );
    fs::write(output_dir.join("matrix.csv"), csv).unwrap();
    fs::write(output_dir.join("report.md"), report).unwrap();
    assert!(gates_a_d_pass, "CV0 Gates A-D failed");
    assert!(gate_e_pass, "CV0 Gate E failed");
    assert!(all_pass, "CV0 cumulative resumed matrix failed");
    println!("CV0_BOUNDED_LOCAL_CONTACT_GENESIS_POSITIVE_V1");
}

fn reference_production_exact(csv: &str) -> bool {
    !csv.lines()
        .skip(1)
        .any(|line| line.split(',').nth(6) != Some("true"))
}

fn replay_exact(csv: &str) -> bool {
    !csv.lines()
        .skip(1)
        .any(|line| line.split(',').nth(5) != Some("true"))
}

fn quiescence_exact(csv: &str) -> bool {
    !csv.lines()
        .skip(1)
        .any(|line| line.split(',').nth(18) != Some("true"))
}
