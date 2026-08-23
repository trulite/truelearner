use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX2_PARENT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const PX0_SOURCE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const RETAINED_SOURCE_SHA256: &str =
    "6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263";
const PX2_CSV_SHA256: &str = "921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18";
const PX2_AUDIT_SHA256: &str = "7076aca03014d19040020b6bfb126e92f7d25dcac3df9cdab92de7dd7849c6fe";
const PX2_HANDOFF_SHA256: &str = "98647ab1563593e18e345cd7e5a71c4991d18b397dfe2dec71a4756106d96509";
const M_LEDGER_SHA256: &str = "9bd352ee3b7d0b729aa27e411fd18601d3cef7a0ac3280442980eb3cd132b146";
const PROBE_PROTOCOL_SHA256: &str =
    "08d3719c263a5dbd8fd4077fff28356b1d5d37dfc9d372f5edecefacd23b4beb";
const PROBE_CSV_SHA256: &str = "a68e867d77b6b8f6459cf1210166969f78507456dcdaf14887a451ce7273da0a";
const PROBE_MD_SHA256: &str = "18f32ae350fafbba7585e0fa99b00aee12e281b1c9b9a91089c5281e25db4ed8";
const PROBE_AUDIT_SHA256: &str = "c3efcbe6d14929ae470c3a909e596fea55ff70aba1539a494bde9bac684bd81e";
const MICRO_PROTOCOL_SHA256: &str =
    "1f18bb049bd08e8268af2d61358f20c771babeb7ded594d812cc77e05c077d96";

const PROBE_CSV: &str = "results/px7_physical_arrival_initiation_probe_v1.csv";
const PROBE_MD: &str = "results/px7_physical_arrival_initiation_probe_v1.md";
const PROBE_STAGING_CSV: &str = "results/.px7_physical_arrival_initiation_probe_v1.csv.staging";
const PROBE_STAGING_MD: &str = "results/.px7_physical_arrival_initiation_probe_v1.md.staging";
const PROBE_NAMESPACE: u64 = 0x7a10_0000_0000;
const MICRO_CSV: &str = "results/px7_physical_arrival_initiation_micro_v1.csv";
const MICRO_MD: &str = "results/px7_physical_arrival_initiation_micro_v1.md";
const MICRO_STAGING_CSV: &str = "results/.px7_physical_arrival_initiation_micro_v1.csv.staging";
const MICRO_STAGING_MD: &str = "results/.px7_physical_arrival_initiation_micro_v1.md.staging";
const MICRO_NAMESPACE: u64 = 0x7b20_0000_0000;
const TRAINING_OCCURRENCES: usize = 4;
const OCCURRENCE_SPACING: i64 = 10;
const RETURN_OFFSET: i64 = 3;
const HELD_OUT_TICK: i64 = 40;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    LearnedReturn,
    Unreturned,
    Subthreshold,
    Absent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MicroCase {
    M0,
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
}

impl MicroCase {
    const ALL: [Self; 8] = [
        Self::M0,
        Self::M1,
        Self::M2,
        Self::M3,
        Self::M4,
        Self::M5,
        Self::M6,
        Self::M7,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::M0 => "M0-ordinary",
            Self::M1 => "M1-mirrored",
            Self::M2 => "M2-reversed-allocation",
            Self::M3 => "M3-reversed-insertion-load4",
            Self::M4 => "M4-combined-load12",
            Self::M5 => "M5-novel-locus",
            Self::M6 => "M6-late-return",
            Self::M7 => "M7-post-gap",
        }
    }

    fn config(self) -> MicroConfig {
        MicroConfig {
            mirror: matches!(self, Self::M1 | Self::M4 | Self::M7),
            reverse_allocation: matches!(self, Self::M2 | Self::M4 | Self::M6),
            reverse_insertion: matches!(self, Self::M3 | Self::M4 | Self::M7),
            distractors: match self {
                Self::M3 => 4,
                Self::M4 => 12,
                Self::M7 => 8,
                _ => 0,
            },
            return_offset: if self == Self::M6 { 6 } else { RETURN_OFFSET },
            held_out_tick: if self == Self::M7 { 70 } else { HELD_OUT_TICK },
            held_out_at_novel: self == Self::M5,
            followup_learned: self == Self::M5,
        }
    }

    fn expected_initial_execution(self) -> usize {
        usize::from(!matches!(self, Self::M5 | Self::M6))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MicroConfig {
    mirror: bool,
    reverse_allocation: bool,
    reverse_insertion: bool,
    distractors: usize,
    return_offset: i64,
    held_out_tick: i64,
    held_out_at_novel: bool,
    followup_learned: bool,
}

impl Arm {
    const PROBE: [Self; 4] = [
        Self::LearnedReturn,
        Self::Unreturned,
        Self::Subthreshold,
        Self::Absent,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::LearnedReturn => "learned-return",
            Self::Unreturned => "unreturned",
            Self::Subthreshold => "subthreshold",
            Self::Absent => "absent",
        }
    }

    fn receives_training_return(self) -> bool {
        self != Self::Unreturned
    }

    fn held_out_impulse(self) -> Option<i32> {
        match self {
            Self::LearnedReturn | Self::Unreturned => Some(2),
            Self::Subthreshold => Some(1),
            Self::Absent => None,
        }
    }

    fn expected_held_out_firings(self) -> usize {
        usize::from(self == Self::LearnedReturn)
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    arrival: CellId,
    execution: CellId,
    persistent_bytes_before: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    training_arrival_firings: usize,
    training_execution_firings: usize,
    training_boundary_firings: usize,
    return_deliveries: usize,
    candidate_total: usize,
    candidate_live: usize,
    candidate_max_coupling: u32,
    candidate_max_resistance: u32,
    held_out_deliveries: usize,
    held_out_arrival_firings: usize,
    held_out_execution_firings: usize,
    held_out_boundary_firings: usize,
    held_out_crossings: usize,
    naturally_quiescent: bool,
    work: u64,
    persistent_bytes_before: usize,
    persistent_bytes_after: usize,
    permanent_fingerprint: u64,
    complete_fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    arm: &'static str,
    namespace: u64,
    observation: Observation,
    duplicate_exact: bool,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MicroObservation {
    training_arrival_firings: usize,
    training_execution_firings: usize,
    return_deliveries: usize,
    background_firings: usize,
    candidate_live: usize,
    candidate_max_coupling: u32,
    candidate_max_resistance: u32,
    held_out_source_firings: usize,
    held_out_execution_firings: usize,
    held_out_boundary_firings: usize,
    held_out_crossings: usize,
    followup_source_firings: usize,
    followup_execution_firings: usize,
    followup_boundary_firings: usize,
    followup_crossings: usize,
    naturally_quiescent: bool,
    work: u64,
    persistent_bytes_before: usize,
    persistent_bytes_after: usize,
    permanent_fingerprint: u64,
    complete_fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MicroRow {
    case: &'static str,
    namespace: u64,
    config: MicroConfig,
    observation: MicroObservation,
    duplicate_exact: bool,
    passed: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [value] if value == "--preflight" => {
            assert!(
                source_audit(),
                "frozen parent and protocol hashes must be exact"
            );
            assert!(
                micro_result_paths_absent(),
                "MICRO result and staging paths must be absent"
            );
            assert!(
                namespace_is_fresh(),
                "PROBE namespace must remain outside PX0--PX2"
            );
            println!("PX7_PHYSICAL_ARRIVAL_INITIATION_PREFLIGHT_PASS parent={PX2_PARENT}");
        }
        [value] if value == "--probe" => execute_probe(),
        [value] if value == "--micro" => execute_micro(),
        _ => {
            eprintln!("PX7 development harness requires --preflight or --probe");
            std::process::exit(2);
        }
    }
}

fn execute_probe() {
    assert!(
        source_audit(),
        "frozen parent and protocol hashes must be exact"
    );
    assert!(
        probe_result_paths_absent(),
        "PROBE result and staging paths must be absent"
    );
    assert!(
        namespace_is_fresh(),
        "PROBE namespace must remain outside PX0--PX2"
    );
    eprintln!("PX7_PHYSICAL_ARRIVAL_INITIATION_PROBE_EVIDENCE_SPENT");

    let rows = Arm::PROBE
        .iter()
        .copied()
        .enumerate()
        .map(|(ordinal, arm)| {
            let namespace = PROBE_NAMESPACE + ordinal as u64 * 0x0010_0000;
            run_duplicate(arm, namespace)
        })
        .collect::<Vec<_>>();
    let passed = rows.iter().all(|row| row.passed);

    write_staging(PROBE_STAGING_CSV, &csv(&rows));
    write_staging(PROBE_STAGING_MD, &markdown(&rows, passed));
    rename(PROBE_STAGING_CSV, PROBE_CSV).expect("PROBE CSV atomic rename succeeds");
    rename(PROBE_STAGING_MD, PROBE_MD).expect("PROBE report atomic rename succeeds");
    println!(
        "PX7 physical arrival initiation PROBE {} ({}/{})",
        if passed { "PASS" } else { "FAIL" },
        rows.iter().filter(|row| row.passed).count(),
        rows.len()
    );
}

fn execute_micro() {
    assert!(
        source_audit(),
        "frozen parent, PROBE, and MICRO protocol hashes must be exact"
    );
    assert!(
        micro_result_paths_absent(),
        "MICRO result and staging paths must be absent"
    );
    assert!(
        namespace_is_fresh(),
        "namespace ranges must remain disjoint"
    );
    eprintln!("PX7_PHYSICAL_ARRIVAL_INITIATION_MICRO_EVIDENCE_SPENT");

    let rows = MicroCase::ALL
        .iter()
        .copied()
        .enumerate()
        .map(|(ordinal, case)| {
            run_micro_duplicate(case, MICRO_NAMESPACE + ordinal as u64 * 0x0010_0000)
        })
        .collect::<Vec<_>>();
    let passed = rows.iter().all(|row| row.passed);

    write_staging(MICRO_STAGING_CSV, &micro_csv(&rows));
    write_staging(MICRO_STAGING_MD, &micro_markdown(&rows, passed));
    rename(MICRO_STAGING_CSV, MICRO_CSV).expect("MICRO CSV atomic rename succeeds");
    rename(MICRO_STAGING_MD, MICRO_MD).expect("MICRO report atomic rename succeeds");
    println!(
        "PX7 physical arrival initiation MICRO {} ({}/{})",
        if passed { "PASS" } else { "FAIL" },
        rows.iter().filter(|row| row.passed).count(),
        rows.len()
    );
}

fn source_audit() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == PX0_SOURCE_SHA256
        && sha256("crates/frozen-organism-v1-physics/src/substrate.rs") == RETAINED_SOURCE_SHA256
        && sha256("results/px2_physical_causal_direction_definitive.csv") == PX2_CSV_SHA256
        && sha256("experiments/px2_physical_causal_direction_definitive_result_audit.md")
            == PX2_AUDIT_SHA256
        && sha256("experiments/px2_physical_causal_direction_authority_handoff.md")
            == PX2_HANDOFF_SHA256
        && sha256("docs/learned_machinery_ledger.md") == M_LEDGER_SHA256
        && sha256("experiments/px7_physical_arrival_initiation_probe_protocol.md")
            == PROBE_PROTOCOL_SHA256
        && sha256(PROBE_CSV) == PROBE_CSV_SHA256
        && sha256(PROBE_MD) == PROBE_MD_SHA256
        && sha256("experiments/px7_physical_arrival_initiation_probe_v1_result_audit.md")
            == PROBE_AUDIT_SHA256
        && sha256("experiments/px7_physical_arrival_initiation_micro_protocol.md")
            == MICRO_PROTOCOL_SHA256
}

fn probe_result_paths_absent() -> bool {
    [PROBE_CSV, PROBE_MD, PROBE_STAGING_CSV, PROBE_STAGING_MD]
        .iter()
        .all(|path| !Path::new(path).exists())
}

fn micro_result_paths_absent() -> bool {
    [MICRO_CSV, MICRO_MD, MICRO_STAGING_CSV, MICRO_STAGING_MD]
        .iter()
        .all(|path| !Path::new(path).exists())
}

fn namespace_is_fresh() -> bool {
    PROBE_NAMESPACE == 0x7a10_0000_0000
        && Arm::PROBE.len() == 4
        && MICRO_NAMESPACE == 0x7b20_0000_0000
        && MicroCase::ALL.len() == 8
        && MICRO_NAMESPACE > PROBE_NAMESPACE + 4 * 0x0010_0000
        && PROBE_NAMESPACE > 0x000f_ffff_ffff
}

fn run_duplicate(arm: Arm, namespace: u64) -> Row {
    let first = run_arm(arm, namespace);
    let second = run_arm(arm, namespace);
    let duplicate_exact = first == second;
    let expected = arm.expected_held_out_firings();
    let learned = arm.receives_training_return();
    let source_expected = usize::from(arm.held_out_impulse() == Some(2));
    let passed = first.training_arrival_firings == TRAINING_OCCURRENCES
        && first.return_deliveries == if learned { TRAINING_OCCURRENCES } else { 0 }
        && (!learned
            || (first.candidate_live == 1
                && first.candidate_max_coupling == 2
                && first.candidate_max_resistance > 1))
        && first.held_out_arrival_firings == source_expected
        && first.held_out_execution_firings == expected
        && first.held_out_boundary_firings == expected
        && first.held_out_crossings == expected
        && first.naturally_quiescent
        && duplicate_exact;
    Row {
        arm: arm.name(),
        namespace,
        observation: first,
        duplicate_exact,
        passed,
    }
}

// PX7_NO_NEW_MECHANISM_EXECUTION_BEGIN
fn run_arm(arm: Arm, namespace: u64) -> Observation {
    let mut world = build_world(namespace);
    let mut training_arrival_firings = 0;
    let mut training_execution_firings = 0;
    let mut training_boundary_firings = 0;
    let mut return_deliveries = 0;
    let mut observed_candidate_coupling = 0;
    let mut work = 0;
    let mut naturally_quiescent = true;

    for occurrence in 0..TRAINING_OCCURRENCES {
        let tick = occurrence as i64 * OCCURRENCE_SPACING;
        world.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: namespace + 0x80 + occurrence as u64,
            target: world.arrival,
            impulse: 2,
        });
        let traversal = world.substrate.propagate();
        training_arrival_firings += firings(&traversal, namespace + 1);
        training_execution_firings += firings(&traversal, namespace + 2);
        training_boundary_firings += firings(&traversal, namespace + 3);
        observed_candidate_coupling = observed_candidate_coupling.max(
            traversal
                .trace
                .iter()
                .filter(|entry| entry.target_physical == namespace + 2)
                .map(|entry| entry.impulse.max(0) as u32)
                .max()
                .unwrap_or(0),
        );
        work += traversal.work.total();
        naturally_quiescent &= traversal.naturally_quiescent;

        if arm.receives_training_return() {
            world.substrate.enter(SpikeInput {
                arrival_tick: tick + RETURN_OFFSET,
                phase: 0,
                origin_physical: namespace + 0x100 + occurrence as u64,
                target: world.arrival,
                impulse: 1,
            });
            let returned = world.substrate.propagate();
            return_deliveries += returned
                .trace
                .iter()
                .filter(|entry| entry.target_physical == namespace + 1 && entry.impulse == 1)
                .count();
            work += returned.work.total();
            naturally_quiescent &= returned.naturally_quiescent;
        }
    }

    let held_out = if let Some(impulse) = arm.held_out_impulse() {
        world.substrate.enter(SpikeInput {
            arrival_tick: HELD_OUT_TICK,
            phase: 0,
            origin_physical: namespace + 0x200,
            target: world.arrival,
            impulse,
        });
        world.substrate.propagate()
    } else {
        work += world.substrate.advance_time(HELD_OUT_TICK).total();
        world.substrate.propagate()
    };
    work += held_out.work.total();
    naturally_quiescent &= held_out.naturally_quiescent;

    let candidates = world
        .substrate
        .arrows_between(world.arrival, world.execution);
    let live_candidates = candidates
        .iter()
        .copied()
        .filter(|arrow| world.substrate.arrow_is_live(*arrow))
        .collect::<Vec<ArrowId>>();
    let candidate_max_coupling = observed_candidate_coupling.max(
        held_out
            .trace
            .iter()
            .filter(|entry| entry.target_physical == namespace + 2)
            .map(|entry| entry.impulse.max(0) as u32)
            .max()
            .unwrap_or(0),
    );
    let candidate_max_resistance = live_candidates
        .iter()
        .map(|arrow| world.substrate.arrow_resistance(*arrow))
        .max()
        .unwrap_or(0);

    Observation {
        training_arrival_firings,
        training_execution_firings,
        training_boundary_firings,
        return_deliveries,
        candidate_total: candidates.len(),
        candidate_live: live_candidates.len(),
        candidate_max_coupling,
        candidate_max_resistance,
        held_out_deliveries: held_out.trace.len(),
        held_out_arrival_firings: firings(&held_out, namespace + 1),
        held_out_execution_firings: firings(&held_out, namespace + 2),
        held_out_boundary_firings: firings(&held_out, namespace + 3),
        held_out_crossings: held_out.crossings.len(),
        naturally_quiescent,
        work,
        persistent_bytes_before: world.persistent_bytes_before,
        persistent_bytes_after: world.substrate.persistent_bytes(),
        permanent_fingerprint: world.substrate.permanent_fingerprint(),
        complete_fingerprint: world.substrate.complete_fingerprint(),
    }
}

fn build_world(namespace: u64) -> World {
    let mut substrate = PlasticSubstrate::new();
    let arrival = substrate.add_cell(CellSpec {
        physical_id: namespace + 1,
        position: 0,
        region: 0,
        threshold: 2,
        resistance: 100,
    });
    let execution = substrate.add_cell(CellSpec {
        physical_id: namespace + 2,
        position: 1,
        region: 0,
        threshold: 2,
        resistance: 100,
    });
    let boundary = substrate.add_cell(CellSpec {
        physical_id: namespace + 3,
        position: 8,
        region: 1,
        threshold: 1,
        resistance: 100,
    });
    substrate.add_arrow(ArrowSpec {
        from: execution,
        to: boundary,
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance: 100,
    });
    let persistent_bytes_before = substrate.persistent_bytes();
    World {
        substrate,
        arrival,
        execution,
        persistent_bytes_before,
    }
}
// PX7_NO_NEW_MECHANISM_EXECUTION_END

#[derive(Clone)]
struct MicroWorld {
    substrate: PlasticSubstrate,
    arrival: CellId,
    execution: CellId,
    novel: CellId,
    distractors: Vec<CellId>,
    distractor_physical_ids: Vec<u64>,
    persistent_bytes_before: usize,
}

fn run_micro_duplicate(case: MicroCase, namespace: u64) -> MicroRow {
    let config = case.config();
    let first = run_micro_case(namespace, config);
    let second = run_micro_case(namespace, config);
    let duplicate_exact = first == second;
    let expected = case.expected_initial_execution();
    let followup_expected = usize::from(config.followup_learned);
    let expected_coupling = if config.return_offset > 4 { 1 } else { 2 };
    let passed = first.training_arrival_firings == TRAINING_OCCURRENCES
        && first.return_deliveries == TRAINING_OCCURRENCES
        && first.background_firings == 0
        && first.candidate_live == 1
        && first.candidate_max_coupling == expected_coupling
        && first.held_out_source_firings == 1
        && first.held_out_execution_firings == expected
        && first.held_out_boundary_firings == expected
        && first.held_out_crossings == expected
        && first.followup_source_firings == followup_expected
        && first.followup_execution_firings == followup_expected
        && first.followup_boundary_firings == followup_expected
        && first.followup_crossings == followup_expected
        && first.naturally_quiescent
        && duplicate_exact;
    MicroRow {
        case: case.name(),
        namespace,
        config,
        observation: first,
        duplicate_exact,
        passed,
    }
}

// PX7_MICRO_NO_NEW_MECHANISM_EXECUTION_BEGIN
fn run_micro_case(namespace: u64, config: MicroConfig) -> MicroObservation {
    let mut world = build_micro_world(namespace, config);
    let mut training_arrival_firings = 0;
    let mut training_execution_firings = 0;
    let mut return_deliveries = 0;
    let mut background_firings = 0;
    let mut observed_candidate_coupling = 0;
    let mut naturally_quiescent = true;
    let mut work = 0;

    for occurrence in 0..TRAINING_OCCURRENCES {
        let tick = occurrence as i64 * OCCURRENCE_SPACING;
        let arrival = world.arrival;
        enter_micro_occurrence(
            &mut world,
            namespace,
            tick,
            occurrence,
            arrival,
            config.reverse_insertion,
        );
        let traversal = world.substrate.propagate();
        training_arrival_firings += firings(&traversal, namespace + 1);
        training_execution_firings += firings(&traversal, namespace + 2);
        background_firings += firings_in(&traversal, &world.distractor_physical_ids);
        observed_candidate_coupling = observed_candidate_coupling.max(
            traversal
                .trace
                .iter()
                .filter(|entry| entry.target_physical == namespace + 2)
                .map(|entry| entry.impulse.max(0) as u32)
                .max()
                .unwrap_or(0),
        );
        work += traversal.work.total();
        naturally_quiescent &= traversal.naturally_quiescent;

        world.substrate.enter(SpikeInput {
            arrival_tick: tick + config.return_offset,
            phase: 0,
            origin_physical: namespace + 0x200 + occurrence as u64,
            target: world.arrival,
            impulse: 1,
        });
        let returned = world.substrate.propagate();
        return_deliveries += returned
            .trace
            .iter()
            .filter(|entry| entry.target_physical == namespace + 1 && entry.impulse == 1)
            .count();
        background_firings += firings_in(&returned, &world.distractor_physical_ids);
        work += returned.work.total();
        naturally_quiescent &= returned.naturally_quiescent;
    }

    let held_out_target = if config.held_out_at_novel {
        world.novel
    } else {
        world.arrival
    };
    enter_micro_occurrence(
        &mut world,
        namespace,
        config.held_out_tick,
        TRAINING_OCCURRENCES,
        held_out_target,
        config.reverse_insertion,
    );
    let held_out = world.substrate.propagate();
    background_firings += firings_in(&held_out, &world.distractor_physical_ids);
    observed_candidate_coupling = observed_candidate_coupling.max(
        held_out
            .trace
            .iter()
            .filter(|entry| entry.target_physical == namespace + 2)
            .map(|entry| entry.impulse.max(0) as u32)
            .max()
            .unwrap_or(0),
    );
    work += held_out.work.total();
    naturally_quiescent &= held_out.naturally_quiescent;

    let followup = if config.followup_learned {
        let arrival = world.arrival;
        enter_micro_occurrence(
            &mut world,
            namespace,
            config.held_out_tick + OCCURRENCE_SPACING,
            TRAINING_OCCURRENCES + 1,
            arrival,
            config.reverse_insertion,
        );
        world.substrate.propagate()
    } else {
        world.substrate.propagate()
    };
    background_firings += firings_in(&followup, &world.distractor_physical_ids);
    work += followup.work.total();
    naturally_quiescent &= followup.naturally_quiescent;

    let candidates = world
        .substrate
        .arrows_between(world.arrival, world.execution);
    let live_candidates = candidates
        .iter()
        .copied()
        .filter(|arrow| world.substrate.arrow_is_live(*arrow))
        .collect::<Vec<_>>();
    let candidate_max_resistance = live_candidates
        .iter()
        .map(|arrow| world.substrate.arrow_resistance(*arrow))
        .max()
        .unwrap_or(0);
    let held_out_source_id = if config.held_out_at_novel {
        namespace + 4
    } else {
        namespace + 1
    };

    MicroObservation {
        training_arrival_firings,
        training_execution_firings,
        return_deliveries,
        background_firings,
        candidate_live: live_candidates.len(),
        candidate_max_coupling: observed_candidate_coupling,
        candidate_max_resistance,
        held_out_source_firings: firings(&held_out, held_out_source_id),
        held_out_execution_firings: firings(&held_out, namespace + 2),
        held_out_boundary_firings: firings(&held_out, namespace + 3),
        held_out_crossings: held_out.crossings.len(),
        followup_source_firings: firings(&followup, namespace + 1),
        followup_execution_firings: firings(&followup, namespace + 2),
        followup_boundary_firings: firings(&followup, namespace + 3),
        followup_crossings: followup.crossings.len(),
        naturally_quiescent,
        work,
        persistent_bytes_before: world.persistent_bytes_before,
        persistent_bytes_after: world.substrate.persistent_bytes(),
        permanent_fingerprint: world.substrate.permanent_fingerprint(),
        complete_fingerprint: world.substrate.complete_fingerprint(),
    }
}

fn build_micro_world(namespace: u64, config: MicroConfig) -> MicroWorld {
    let direction = if config.mirror { -1 } else { 1 };
    let specs = [
        CellSpec {
            physical_id: namespace + 1,
            position: 0,
            region: 0,
            threshold: 2,
            resistance: 100,
        },
        CellSpec {
            physical_id: namespace + 2,
            position: direction,
            region: 0,
            threshold: 2,
            resistance: 100,
        },
        CellSpec {
            physical_id: namespace + 4,
            position: -direction,
            region: 0,
            threshold: 2,
            resistance: 100,
        },
        CellSpec {
            physical_id: namespace + 3,
            position: direction * 8,
            region: 1,
            threshold: 1,
            resistance: 100,
        },
    ];
    let order = if config.reverse_allocation {
        [3, 2, 1, 0]
    } else {
        [0, 1, 2, 3]
    };
    let mut substrate = PlasticSubstrate::new();
    let mut cells = [None; 4];
    for logical in order {
        cells[logical] = Some(substrate.add_cell(specs[logical]));
    }
    let arrival = cells[0].expect("arrival cell allocated");
    let execution = cells[1].expect("execution cell allocated");
    let novel = cells[2].expect("novel cell allocated");
    let boundary = cells[3].expect("boundary cell allocated");

    let mut distractors = Vec::new();
    let mut distractor_physical_ids = Vec::new();
    for index in 0..config.distractors {
        let physical_id = namespace + 0x1000 + index as u64;
        distractor_physical_ids.push(physical_id);
        distractors.push(substrate.add_cell(CellSpec {
            physical_id,
            position: direction * (100 + index as i32 * 3),
            region: 0,
            threshold: 3,
            resistance: 100,
        }));
    }
    substrate.add_arrow(ArrowSpec {
        from: execution,
        to: boundary,
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance: 100,
    });
    let persistent_bytes_before = substrate.persistent_bytes();
    MicroWorld {
        substrate,
        arrival,
        execution,
        novel,
        distractors,
        distractor_physical_ids,
        persistent_bytes_before,
    }
}

fn enter_micro_occurrence(
    world: &mut MicroWorld,
    namespace: u64,
    tick: i64,
    occurrence: usize,
    focal: CellId,
    reverse_insertion: bool,
) {
    let focal_input = SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: namespace + 0x400 + occurrence as u64,
        target: focal,
        impulse: 2,
    };
    if !reverse_insertion {
        world.substrate.enter(focal_input);
    }
    let indices: Vec<usize> = if reverse_insertion {
        (0..world.distractors.len()).rev().collect()
    } else {
        (0..world.distractors.len()).collect()
    };
    for index in indices {
        world.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: namespace + 0x800 + occurrence as u64 * 0x100 + index as u64,
            target: world.distractors[index],
            impulse: 1,
        });
    }
    if reverse_insertion {
        world.substrate.enter(focal_input);
    }
}
// PX7_MICRO_NO_NEW_MECHANISM_EXECUTION_END

fn firings_in(run: &Execution, physical_ids: &[u64]) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.fired && physical_ids.contains(&entry.target_physical))
        .count()
}

fn firings(run: &Execution, physical_id: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id && entry.fired)
        .count()
}

fn csv(rows: &[Row]) -> String {
    let mut out = String::from(
        "arm,namespace,training_arrival_firings,training_execution_firings,training_boundary_firings,return_deliveries,candidate_total,candidate_live,candidate_max_coupling,candidate_max_resistance,held_out_deliveries,held_out_arrival_firings,held_out_execution_firings,held_out_boundary_firings,held_out_crossings,naturally_quiescent,duplicate_exact,work,persistent_bytes_before,persistent_bytes_after,permanent_fingerprint,complete_fingerprint,passed\n",
    );
    for row in rows {
        let o = &row.observation;
        out.push_str(&format!(
            "{},{:#x},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.arm,
            row.namespace,
            o.training_arrival_firings,
            o.training_execution_firings,
            o.training_boundary_firings,
            o.return_deliveries,
            o.candidate_total,
            o.candidate_live,
            o.candidate_max_coupling,
            o.candidate_max_resistance,
            o.held_out_deliveries,
            o.held_out_arrival_firings,
            o.held_out_execution_firings,
            o.held_out_boundary_firings,
            o.held_out_crossings,
            o.naturally_quiescent,
            row.duplicate_exact,
            o.work,
            o.persistent_bytes_before,
            o.persistent_bytes_after,
            o.permanent_fingerprint,
            o.complete_fingerprint,
            row.passed
        ));
    }
    out
}

fn markdown(rows: &[Row], passed: bool) -> String {
    let mut out = format!(
        "# PX7 physical arrival initiation PROBE result\n\nVerdict: **{}**.\n\nFrozen parent: `{PX2_PARENT}`. No new mechanism or substrate-law change was used.\n\n| arm | learned coupling | held-out source/execution/boundary | crossing | quiescent | duplicate | work | bytes before/after | result |\n|---|---:|---|---:|:---:|:---:|---:|---|:---:|\n",
        if passed { "PASS" } else { "FAIL" }
    );
    for row in rows {
        let o = &row.observation;
        out.push_str(&format!(
            "| {} | {} | {}/{}/{} | {} | {} | {} | {} | {}/{} | {} |\n",
            row.arm,
            o.candidate_max_coupling,
            o.held_out_arrival_firings,
            o.held_out_execution_firings,
            o.held_out_boundary_firings,
            o.held_out_crossings,
            o.naturally_quiescent,
            row.duplicate_exact,
            o.work,
            o.persistent_bytes_before,
            o.persistent_bytes_after,
            if row.passed { "PASS" } else { "FAIL" }
        ));
    }
    out.push_str(
        "\nOrganism-visible execution used only the frozen CELL/ARROW/SPIKE substrate and actual local participation/return state. Scenario names and pass clauses were evaluator-only.\n",
    );
    out
}

fn micro_csv(rows: &[MicroRow]) -> String {
    let mut out = String::from(
        "case,namespace,mirror,reverse_allocation,reverse_insertion,distractors,return_offset,held_out_tick,training_arrival_firings,training_execution_firings,return_deliveries,background_firings,candidate_live,candidate_max_coupling,candidate_max_resistance,held_out_source_firings,held_out_execution_firings,held_out_boundary_firings,held_out_crossings,followup_source_firings,followup_execution_firings,followup_boundary_firings,followup_crossings,naturally_quiescent,duplicate_exact,work,persistent_bytes_before,persistent_bytes_after,permanent_fingerprint,complete_fingerprint,passed\n",
    );
    for row in rows {
        let c = row.config;
        let o = &row.observation;
        out.push_str(&format!(
            "{},{:#x},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.case,
            row.namespace,
            c.mirror,
            c.reverse_allocation,
            c.reverse_insertion,
            c.distractors,
            c.return_offset,
            c.held_out_tick,
            o.training_arrival_firings,
            o.training_execution_firings,
            o.return_deliveries,
            o.background_firings,
            o.candidate_live,
            o.candidate_max_coupling,
            o.candidate_max_resistance,
            o.held_out_source_firings,
            o.held_out_execution_firings,
            o.held_out_boundary_firings,
            o.held_out_crossings,
            o.followup_source_firings,
            o.followup_execution_firings,
            o.followup_boundary_firings,
            o.followup_crossings,
            o.naturally_quiescent,
            row.duplicate_exact,
            o.work,
            o.persistent_bytes_before,
            o.persistent_bytes_after,
            o.permanent_fingerprint,
            o.complete_fingerprint,
            row.passed
        ));
    }
    out
}

fn micro_markdown(rows: &[MicroRow], passed: bool) -> String {
    let mut out = format!(
        "# PX7 physical arrival initiation MICRO result\n\nVerdict: **{}**.\n\nFrozen parent: `{PX2_PARENT}`. The PROBE was not rerun and no new mechanism was used.\n\n| case | coupling/resistance | held-out source/execution/boundary | follow-up source/execution/boundary | crossings initial/follow-up | background | quiescent | duplicate | work | bytes before/after | result |\n|---|---|---|---|---|---:|:---:|:---:|---:|---|:---:|\n",
        if passed { "PASS" } else { "FAIL" }
    );
    for row in rows {
        let o = &row.observation;
        out.push_str(&format!(
            "| {} | {}/{} | {}/{}/{} | {}/{}/{} | {}/{} | {} | {} | {} | {} | {}/{} | {} |\n",
            row.case,
            o.candidate_max_coupling,
            o.candidate_max_resistance,
            o.held_out_source_firings,
            o.held_out_execution_firings,
            o.held_out_boundary_firings,
            o.followup_source_firings,
            o.followup_execution_firings,
            o.followup_boundary_firings,
            o.held_out_crossings,
            o.followup_crossings,
            o.background_firings,
            o.naturally_quiescent,
            row.duplicate_exact,
            o.work,
            o.persistent_bytes_before,
            o.persistent_bytes_after,
            if row.passed { "PASS" } else { "FAIL" }
        ));
    }
    out.push_str(
        "\nAll supplied schedules were fixed anonymous physical arrivals. Organism-visible execution used only frozen CELL/ARROW/SPIKE state and local physical timing.\n",
    );
    out
}

fn write_staging(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("staging artifact must be fresh");
    file.write_all(contents.as_bytes())
        .expect("staging artifact write succeeds");
    file.sync_all().expect("staging artifact sync succeeds");
}

fn sha256(path: &str) -> String {
    let output = Command::new("shasum")
        .args(["-a", "256", path])
        .output()
        .expect("shasum is available");
    assert!(output.status.success(), "hash input exists: {path}");
    String::from_utf8(output.stdout)
        .expect("hash output is UTF-8")
        .split_whitespace()
        .next()
        .expect("hash output contains digest")
        .to_string()
}
