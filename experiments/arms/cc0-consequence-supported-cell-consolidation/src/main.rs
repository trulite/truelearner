#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowSpec, CellId, CellSpec, ContentHash, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [7_300_000, 7_400_000];
const PHASES: std::ops::Range<i64> = 0..10;
const EXPECTED_CASES: usize = 120;
const EXPECTED_ROWS: usize = 240;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Supported,
    UseOnly,
    Unqualified,
    Relaxed,
    TwoLocal,
    RepeatedSupport,
}

impl Family {
    const ALL: [Self; 6] = [
        Self::Supported,
        Self::UseOnly,
        Self::Unqualified,
        Self::Relaxed,
        Self::TwoLocal,
        Self::RepeatedSupport,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Supported => "participation_plus_local_modulation",
            Self::UseOnly => "repeated_use_without_modulation",
            Self::Unqualified => "unqualified_and_wrong_cell_modulation",
            Self::Relaxed => "late_modulation_after_relaxation",
            Self::TwoLocal => "two_participating_cells_local_consequence",
            Self::RepeatedSupport => "repeated_support_and_eventual_decay",
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
        self.arrow_updates = self.arrow_updates.saturating_add(work.local_return_updates);
        self.cell_updates = self.cell_updates.saturating_add(work.cell_return_updates);
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
    fn new(root: u64, phase: i64, mechanics: MechanicalConfig) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 32, 64, mechanics);
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

    fn arrow(&mut self, from: CellId, to: CellId, mode: TransmissionMode) {
        self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 500,
            mode,
        });
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

    fn cell_update_count(&self, id: CellId) -> usize {
        self.trace
            .iter()
            .filter(|transition| {
                matches!(
                    transition.event,
                    PhysicalEvent::CellResistance { cell, .. } if cell == id
                )
            })
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

struct Fixture {
    world: World,
    driver: CellId,
    modulator: CellId,
    subject: CellId,
}

fn fixture(root: u64, phase: i64, mechanics: MechanicalConfig, resistance: u32) -> Fixture {
    let mut world = World::new(root, phase, mechanics);
    let driver = world.cell(root + 1, -100, 1, 500);
    let subject = world.cell(root + 2, 0, 1, resistance);
    let modulator = world.cell(root + 3, 100, 1, 500);
    world.arrow(driver, subject, TransmissionMode::Drive);
    world.arrow(modulator, subject, TransmissionMode::Modulatory);
    Fixture {
        world,
        driver,
        modulator,
        subject,
    }
}

fn observe(family: Family, root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    match family {
        Family::Supported => observe_supported(root, phase, mechanics),
        Family::UseOnly => observe_use_only(root, phase, mechanics),
        Family::Unqualified => observe_unqualified(root, phase, mechanics),
        Family::Relaxed => observe_relaxed(root, phase, mechanics),
        Family::TwoLocal => observe_two_local(root, phase, mechanics),
        Family::RepeatedSupport => observe_repeated(root, phase, mechanics),
    }
}

fn observe_supported(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let Fixture {
        mut world,
        driver,
        modulator,
        subject,
    } = fixture(root, phase, mechanics, 1);
    world.pulse(driver, 0, root + 10);
    let participation_after_fire = world.body.cell_participation(subject);
    world.pulse(modulator, 1, root + 11);
    let after = world.body.cell_resistance(subject);
    let updates = world.cell_update_count(subject);
    world.advance_age(42);
    let death = world.death_age(subject);
    let markers = vec![format!(
        "participation={participation_after_fire:?};after={after:?};death={death:?};updates={updates}"
    )];
    let checks = vec![
        (
            "actual_fire_left_cell_participation".into(),
            participation_after_fire.is_some_and(|value| value > 0),
        ),
        ("one_subject_fire".into(), world.fire_count(subject) == 1),
        ("supported_resistance_1_to_4".into(), after == Some(4)),
        ("one_cell_update".into(), updates == 1),
        ("lifetime_rebased_and_extended".into(), death == Some(42)),
        ("one_modulatory_delivery".into(), world.work.modulation == 1),
        (
            "no_arrow_plastic_update".into(),
            world.work.arrow_updates == 0,
        ),
        ("natural_quiescence".into(), world.naturally_quiescent),
    ];
    world.finish(markers, checks)
}

fn observe_use_only(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let Fixture {
        mut world,
        driver,
        subject,
        ..
    } = fixture(root, phase, mechanics, 1);
    for age in [0, 2, 4, 6, 8] {
        world.pulse(driver, age, root + 20 + u64::try_from(age).unwrap());
    }
    let before_death = world.body.cell_resistance(subject);
    let participation = world.body.cell_participation(subject);
    world.advance_age(10);
    let death = world.death_age(subject);
    let markers = vec![format!(
        "fires={};resistance={before_death:?};participation={participation:?};death={death:?}",
        world.fire_count(subject)
    )];
    let checks = vec![
        ("five_real_firings".into(), world.fire_count(subject) == 5),
        (
            "use_left_transient_participation".into(),
            participation.is_some_and(|value| value > 0),
        ),
        (
            "use_did_not_raise_resistance".into(),
            before_death == Some(1),
        ),
        ("ordinary_lifetime_unchanged".into(), death == Some(10)),
        ("zero_cell_updates".into(), world.work.cell_updates == 0),
    ];
    world.finish(markers, checks)
}

fn observe_unqualified(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let driver = world.cell(root + 31, -100, 1, 500);
    let participating = world.cell(root + 32, 0, 1, 2);
    let untouched = world.cell(root + 33, 10, 1, 2);
    let modulator = world.cell(root + 34, 100, 1, 500);
    world.arrow(driver, participating, TransmissionMode::Drive);
    world.arrow(modulator, untouched, TransmissionMode::Modulatory);
    world.pulse(driver, 0, root + 35);
    world.pulse(modulator, 1, root + 36);
    let participating_after = world.body.cell_resistance(participating);
    let untouched_after = world.body.cell_resistance(untouched);
    let markers = vec![format!(
        "participating={participating_after:?};untouched={untouched_after:?};updates={}",
        world.work.cell_updates
    )];
    let checks = vec![
        (
            "participating_cell_fired".into(),
            world.fire_count(participating) == 1,
        ),
        (
            "wrong_cell_modulation_did_not_credit_participant".into(),
            participating_after == Some(2),
        ),
        (
            "modulation_without_participation_did_not_credit_target".into(),
            untouched_after == Some(2),
        ),
        ("zero_cell_updates".into(), world.work.cell_updates == 0),
    ];
    world.finish(markers, checks)
}

fn observe_relaxed(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let Fixture {
        mut world,
        driver,
        modulator,
        subject,
    } = fixture(root, phase, mechanics, 200);
    world.pulse(driver, 0, root + 40);
    world.advance_age(1_000);
    let trace_before = world.body.cell_participation(subject);
    let resistance_before = world.body.cell_resistance(subject);
    world.pulse(modulator, 1_000, root + 41);
    let resistance_after = world.body.cell_resistance(subject);
    let markers = vec![format!(
        "trace_before={trace_before:?};resistance={resistance_before:?}->{resistance_after:?}"
    )];
    let checks = vec![
        (
            "participation_relaxed_to_zero".into(),
            trace_before == Some(0),
        ),
        (
            "subject_still_live".into(),
            world.body.cell_is_live(subject) == Some(true),
        ),
        (
            "late_modulation_no_resistance_change".into(),
            resistance_after == resistance_before,
        ),
        ("zero_cell_updates".into(), world.work.cell_updates == 0),
    ];
    world.finish(markers, checks)
}

fn observe_two_local(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let driver = world.cell(root + 51, -100, 1, 500);
    let first = world.cell(root + 52, 0, 1, 1);
    let second = world.cell(root + 53, 1, 1, 1);
    let modulator = world.cell(root + 54, 100, 1, 500);
    world.arrow(driver, first, TransmissionMode::Drive);
    world.arrow(driver, second, TransmissionMode::Drive);
    world.arrow(modulator, first, TransmissionMode::Modulatory);
    world.pulse(driver, 0, root + 55);
    world.pulse(modulator, 1, root + 56);
    let first_after = world.body.cell_resistance(first);
    let second_after = world.body.cell_resistance(second);
    let markers = vec![format!(
        "first={first_after:?};second={second_after:?};updates={}",
        world.work.cell_updates
    )];
    let checks = vec![
        (
            "both_cells_participated".into(),
            world.fire_count(first) == 1 && world.fire_count(second) == 1,
        ),
        ("local_cell_consolidated".into(), first_after == Some(4)),
        (
            "neighbor_did_not_consolidate".into(),
            second_after == Some(1),
        ),
        ("one_cell_update".into(), world.work.cell_updates == 1),
    ];
    world.finish(markers, checks)
}

fn observe_repeated(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let Fixture {
        mut world,
        driver,
        modulator,
        subject,
    } = fixture(root, phase, mechanics, 1);
    world.pulse(driver, 0, root + 60);
    world.pulse(modulator, 1, root + 61);
    let after_first = world.body.cell_resistance(subject);
    world.pulse(driver, 3, root + 62);
    world.pulse(modulator, 4, root + 63);
    let after_second = world.body.cell_resistance(subject);
    world.advance_age(75);
    let death = world.death_age(subject);
    let markers = vec![format!(
        "resistance={after_first:?}->{after_second:?};death={death:?};updates={}",
        world.work.cell_updates
    )];
    let checks = vec![
        ("first_support_1_to_4".into(), after_first == Some(4)),
        ("second_support_4_to_7".into(), after_second == Some(7)),
        ("two_cell_updates".into(), world.work.cell_updates == 2),
        ("no_arrow_updates".into(), world.work.arrow_updates == 0),
        ("unsupported_r7_eventually_died".into(), death == Some(75)),
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
        .unwrap_or_else(|| PathBuf::from("experiments/results/cc0_consequence_supported_cell_v1"));
    fs::create_dir_all(&output_dir).unwrap();

    let mut csv = String::from(
        "case,family,root,phase,mechanics,replay_equal,cross_mechanics_equal,checks_pass,failed,cell_updates,cell_deallocations,physical_work,trace_len,crossings,final_tick,naturally_quiescent,body_hash,markers\n",
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
                        observation.work.cell_updates,
                        observation.work.cell_deallocations,
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
        "# CC0 consequence-supported CELL consolidation\n\n- cases: {cases}/{EXPECTED_CASES}\n- rows: {rows}/{EXPECTED_ROWS}\n- clauses: {passed_clauses}/{clauses}\n- Reference/Production exact: {all_pass}\n- replay exact: {all_pass}\n- natural quiescence: {all_pass}\n- maximum PhysicalWork: {maximum_work}\n"
    );
    fs::write(output_dir.join("matrix.csv"), csv).unwrap();
    fs::write(output_dir.join("report.md"), report).unwrap();
    assert!(all_pass, "CC0 frozen matrix failed");
    println!("CC0_CONSEQUENCE_SUPPORTED_CELL_CONSOLIDATION_POSITIVE_V1");
}
