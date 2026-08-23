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

const PROBE_CSV: &str = "results/px7_physical_arrival_initiation_probe_v1.csv";
const PROBE_MD: &str = "results/px7_physical_arrival_initiation_probe_v1.md";
const PROBE_STAGING_CSV: &str = "results/.px7_physical_arrival_initiation_probe_v1.csv.staging";
const PROBE_STAGING_MD: &str = "results/.px7_physical_arrival_initiation_probe_v1.md.staging";
const PROBE_NAMESPACE: u64 = 0x7a10_0000_0000;
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

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [value] if value == "--preflight" => {
            assert!(
                source_audit(),
                "frozen parent and protocol hashes must be exact"
            );
            assert!(
                result_paths_absent(),
                "PROBE result and staging paths must be absent"
            );
            assert!(
                namespace_is_fresh(),
                "PROBE namespace must remain outside PX0--PX2"
            );
            println!("PX7_PHYSICAL_ARRIVAL_INITIATION_PREFLIGHT_PASS parent={PX2_PARENT}");
        }
        [value] if value == "--probe" => execute_probe(),
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
        result_paths_absent(),
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
}

fn result_paths_absent() -> bool {
    [PROBE_CSV, PROBE_MD, PROBE_STAGING_CSV, PROBE_STAGING_MD]
        .iter()
        .all(|path| !Path::new(path).exists())
}

fn namespace_is_fresh() -> bool {
    PROBE_NAMESPACE == 0x7a10_0000_0000
        && Arm::PROBE.len() == 4
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
