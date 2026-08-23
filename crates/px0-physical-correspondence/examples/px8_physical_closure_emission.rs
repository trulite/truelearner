use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{read_to_string, rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::{Command, ExitCode};

const PARENT_COMMIT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const SUBSTRATE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX0_CSV_SHA256: &str = "b750792123de1c0aa7d3104d2d1bcd3fdc6e26a70e54b10f5eedf320fe7d95c9";
const PX1_CSV_SHA256: &str = "6613ff0a96bb3a60fbe7afeb92cd64edced3c6df5dcc04fe47518db158dd88f6";
const PX2_CSV_SHA256: &str = "921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18";
const PROTOCOL_SHA256: &str = "881f4e3d46ce55aa1d637bfe2cc3cc99fb7e7ca2348fc1256e949fd1e3d36c2b";
const SOURCE_PATH: &str = "crates/px0-physical-correspondence/src/lib.rs";
const PROTOCOL_PATH: &str = "experiments/px8_physical_closure_emission_development_protocol.md";

const A: usize = 0;
const B: usize = 1;
const N: usize = 2;
const K: usize = 3;
const O: usize = 4;
const D0: usize = 5;
const D1: usize = 6;
const D2: usize = 7;
const D3: usize = 8;
const CELL_COUNT: usize = 9;

#[derive(Clone, Copy)]
struct Stage {
    argument: &'static str,
    label: &'static str,
    seeds: usize,
    namespace: u64,
    csv: &'static str,
    report: &'static str,
    staging_csv: &'static str,
    staging_report: &'static str,
    prerequisite: Option<&'static str>,
}

const PROBE: Stage = Stage {
    argument: "--probe",
    label: "PROBE",
    seeds: 2,
    namespace: 0x8_8000_0000,
    csv: "results/px8_physical_closure_emission_probe.csv",
    report: "results/px8_physical_closure_emission_probe.md",
    staging_csv: "results/.px8_physical_closure_emission_probe.csv.staging",
    staging_report: "results/.px8_physical_closure_emission_probe.md.staging",
    prerequisite: None,
};

const MICRO: Stage = Stage {
    argument: "--micro",
    label: "MICRO",
    seeds: 8,
    namespace: 0x8_8200_0000,
    csv: "results/px8_physical_closure_emission_micro.csv",
    report: "results/px8_physical_closure_emission_micro.md",
    staging_csv: "results/.px8_physical_closure_emission_micro.csv.staging",
    staging_report: "results/.px8_physical_closure_emission_micro.md.staging",
    prerequisite: Some("results/px8_physical_closure_emission_probe.md"),
};

const GATE: Stage = Stage {
    argument: "--gate",
    label: "GATE",
    seeds: 16,
    namespace: 0x8_8600_0000,
    csv: "results/px8_physical_closure_emission_gate.csv",
    report: "results/px8_physical_closure_emission_gate.md",
    staging_csv: "results/.px8_physical_closure_emission_gate.csv.staging",
    staging_report: "results/.px8_physical_closure_emission_gate.md.staging",
    prerequisite: Some("results/px8_physical_closure_emission_micro.md"),
};

#[derive(Clone, Copy)]
struct Condition {
    name: &'static str,
    positive_a: bool,
    positive_b: bool,
    negative: bool,
    skew_b: i64,
    pressure_outward: bool,
    unrelated_only: bool,
    expected_convergence: bool,
    expected_crossing: bool,
}

const CONDITIONS: [Condition; 8] = [
    Condition {
        name: "coincident-positive",
        positive_a: true,
        positive_b: true,
        negative: false,
        skew_b: 0,
        pressure_outward: false,
        unrelated_only: false,
        expected_convergence: true,
        expected_crossing: true,
    },
    Condition {
        name: "skewed-positive",
        positive_a: true,
        positive_b: true,
        negative: false,
        skew_b: 2,
        pressure_outward: false,
        unrelated_only: false,
        expected_convergence: true,
        expected_crossing: true,
    },
    Condition {
        name: "first-positive-only",
        positive_a: true,
        positive_b: false,
        negative: false,
        skew_b: 0,
        pressure_outward: false,
        unrelated_only: false,
        expected_convergence: false,
        expected_crossing: false,
    },
    Condition {
        name: "second-positive-only",
        positive_a: false,
        positive_b: true,
        negative: false,
        skew_b: 0,
        pressure_outward: false,
        unrelated_only: false,
        expected_convergence: false,
        expected_crossing: false,
    },
    Condition {
        name: "ordinary-negative-block",
        positive_a: true,
        positive_b: true,
        negative: true,
        skew_b: 0,
        pressure_outward: false,
        unrelated_only: false,
        expected_convergence: false,
        expected_crossing: false,
    },
    Condition {
        name: "outward-pressure-block",
        positive_a: true,
        positive_b: true,
        negative: false,
        skew_b: 0,
        pressure_outward: true,
        unrelated_only: false,
        expected_convergence: true,
        expected_crossing: false,
    },
    Condition {
        name: "unrelated-inner-activity",
        positive_a: false,
        positive_b: false,
        negative: false,
        skew_b: 0,
        pressure_outward: false,
        unrelated_only: true,
        expected_convergence: false,
        expected_crossing: false,
    },
    Condition {
        name: "no-arrival",
        positive_a: false,
        positive_b: false,
        negative: false,
        skew_b: 0,
        pressure_outward: false,
        unrelated_only: false,
        expected_convergence: false,
        expected_crossing: false,
    },
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    execution: Execution,
    advance_work: WorkLedger,
    persistent_bytes: usize,
    outward_live: bool,
    positive_a_firings: usize,
    positive_b_firings: usize,
    negative_firings: usize,
    convergence_inputs: Vec<(i64, i32, bool)>,
    convergence_tick: Option<i64>,
    crossing_ticks: Vec<i64>,
    outer_ticks: Vec<i64>,
}

struct Row {
    seed: usize,
    condition: &'static str,
    namespace: u64,
    mirrored: bool,
    reverse_allocation: bool,
    reverse_insertion: bool,
    participant_delay: i64,
    outward_delay: i64,
    unrelated_load: usize,
    observation: Observation,
    duplicate_exact: bool,
    clauses: [bool; 10],
}

fn main() -> ExitCode {
    match run() {
        Ok(passed) => {
            if passed {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<bool, String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    if arguments
        .iter()
        .any(|argument| argument.contains("definitive"))
    {
        return Err("PX8 definitive execution is forbidden in this development lane".into());
    }
    if arguments == ["--preflight"] {
        let audit = audit_basis()?;
        println!("PX8_PHYSICAL_CLOSURE_EMISSION_PREFLIGHT_OK {audit}");
        return Ok(true);
    }
    let stage = [PROBE, MICRO, GATE]
        .into_iter()
        .find(|candidate| arguments == [candidate.argument])
        .ok_or_else(|| {
            "expected exactly one of --preflight, --probe, --micro, or --gate".to_string()
        })?;

    let audit = audit_basis()?;
    refuse_existing(stage)?;
    if let Some(prerequisite) = stage.prerequisite {
        let prior = read_to_string(prerequisite)
            .map_err(|error| format!("missing frozen prerequisite {prerequisite}: {error}"))?;
        if !prior.contains("Outcome: **DEVELOPMENT POSITIVE**") {
            return Err(format!(
                "prerequisite is not a frozen positive: {prerequisite}"
            ));
        }
    }

    println!(
        "PX8_PHYSICAL_CLOSURE_EMISSION_{}_EVIDENCE_SPENT",
        stage.label
    );
    let mut rows = Vec::with_capacity(stage.seeds * CONDITIONS.len());
    for seed in 0..stage.seeds {
        for (condition_index, condition) in CONDITIONS.iter().enumerate() {
            rows.push(run_row(stage, seed, condition_index, *condition));
        }
    }

    let passed_rows = rows
        .iter()
        .filter(|row| row.clauses.iter().all(|value| *value))
        .count();
    let passed_claims = rows
        .iter()
        .flat_map(|row| row.clauses)
        .filter(|value| *value)
        .count();
    let total_claims = rows.len() * 10;
    let passed = passed_rows == rows.len() && passed_claims == total_claims;
    let csv = render_csv(stage, &rows);
    let report = render_report(stage, &rows, &audit, passed_rows, passed_claims, passed);
    publish(stage.staging_csv, stage.csv, &csv)?;
    publish(stage.staging_report, stage.report, &report)?;
    println!(
        "{} {}/{} rows; {}/{} claims",
        if passed { "PASS" } else { "SCIENTIFIC_FAIL" },
        passed_rows,
        rows.len(),
        passed_claims,
        total_claims
    );
    Ok(passed)
}

fn run_row(stage: Stage, seed: usize, condition_index: usize, condition: Condition) -> Row {
    let namespace =
        stage.namespace + (seed as u64) * 0x0010_0000 + (condition_index as u64) * 0x0001_0000;
    let mirrored = seed % 2 == 1;
    let reverse_allocation = seed % 2 == 1;
    let reverse_insertion = (seed + condition_index) % 2 == 1;
    let participant_delay = 3 + (seed % 4) as i64;
    let outward_delay = 2 + ((seed * 3) % 4) as i64;
    let unrelated_load = [0, 4, 12, 24][seed % 4];
    let first = develop(
        namespace,
        mirrored,
        reverse_allocation,
        reverse_insertion,
        participant_delay,
        outward_delay,
        unrelated_load,
        condition,
    );
    let second = develop(
        namespace,
        mirrored,
        reverse_allocation,
        reverse_insertion,
        participant_delay,
        outward_delay,
        unrelated_load,
        condition,
    );
    let duplicate_exact = first == second;
    let base_tick = if condition.pressure_outward { 10 } else { 0 };
    let a_tick = base_tick + participant_delay;
    let b_tick = base_tick + condition.skew_b + participant_delay;
    let expected_convergence_tick = condition.expected_convergence.then_some(a_tick.max(b_tick));
    let expected_crossing_ticks = expected_convergence_tick
        .filter(|_| condition.expected_crossing)
        .into_iter()
        .collect::<Vec<_>>();
    let expected_outer_ticks = expected_convergence_tick
        .filter(|_| condition.expected_crossing)
        .map(|tick| tick + outward_delay)
        .into_iter()
        .collect::<Vec<_>>();
    let mut expected_inputs = Vec::new();
    if condition.negative {
        expected_inputs.push((a_tick, -3, false));
    }
    if condition.positive_a {
        expected_inputs.push((a_tick, 3, false));
    }
    if condition.positive_b {
        expected_inputs.push((b_tick, 3, condition.expected_convergence));
    }
    let participant_match = first.positive_a_firings == usize::from(condition.positive_a)
        && first.positive_b_firings == usize::from(condition.positive_b)
        && first.negative_firings == usize::from(condition.negative);
    let no_premature = first.crossing_ticks.iter().all(|tick| {
        first
            .convergence_tick
            .is_some_and(|convergence_tick| *tick >= convergence_tick)
    });
    let expected_live = !condition.pressure_outward;
    let clauses = [
        namespace >= stage.namespace && namespace < stage.namespace + 0x0200_0000,
        participant_match,
        first.convergence_inputs == expected_inputs,
        no_premature,
        first.convergence_tick == expected_convergence_tick,
        first.crossing_ticks == expected_crossing_ticks && first.outward_live == expected_live,
        first.outer_ticks == expected_outer_ticks,
        first.execution.crossings.len() == first.crossing_ticks.len(),
        first.execution.naturally_quiescent,
        duplicate_exact,
    ];
    Row {
        seed,
        condition: condition.name,
        namespace,
        mirrored,
        reverse_allocation,
        reverse_insertion,
        participant_delay,
        outward_delay,
        unrelated_load,
        observation: first,
        duplicate_exact,
        clauses,
    }
}

#[allow(clippy::too_many_arguments)]
fn develop(
    namespace: u64,
    mirrored: bool,
    reverse_allocation: bool,
    reverse_insertion: bool,
    participant_delay: i64,
    outward_delay: i64,
    unrelated_load: usize,
    condition: Condition,
) -> Observation {
    let mut substrate = PlasticSubstrate::new();
    let sign = if mirrored { -1 } else { 1 };
    let id_offsets = physical_id_offsets(namespace);
    let thresholds = [1, 1, 1, 4, 1, 1, 1, 1, 1];
    let regions = [0, 0, 0, 0, 1, 0, 0, 0, 0];
    let positions = [0, 20, 40, 80, 120, 180, 200, 220, 240];
    let mut cells: Vec<Option<CellId>> = vec![None; CELL_COUNT];
    let mut allocation = (0..CELL_COUNT).collect::<Vec<_>>();
    if reverse_allocation {
        allocation.reverse();
    }
    for role in allocation {
        cells[role] = Some(substrate.add_cell(CellSpec {
            physical_id: id_offsets[role],
            position: sign * positions[role],
            region: regions[role],
            threshold: thresholds[role],
            resistance: 100,
        }));
    }
    let cells = cells
        .into_iter()
        .map(|cell| cell.expect("every physical cell is allocated"))
        .collect::<Vec<_>>();
    add_arrow(
        &mut substrate,
        cells[A],
        cells[K],
        participant_delay,
        0,
        3,
        100,
    );
    add_arrow(
        &mut substrate,
        cells[B],
        cells[K],
        participant_delay,
        1,
        3,
        100,
    );
    add_arrow(
        &mut substrate,
        cells[N],
        cells[K],
        participant_delay,
        -1,
        -3,
        100,
    );
    let outward = add_arrow(
        &mut substrate,
        cells[K],
        cells[O],
        outward_delay,
        0,
        1,
        if condition.pressure_outward { 1 } else { 100 },
    );
    add_arrow(&mut substrate, cells[D0], cells[D1], 1, 0, 1, 100);
    add_arrow(&mut substrate, cells[D1], cells[D2], 1, 0, 1, 100);
    add_arrow(&mut substrate, cells[D2], cells[D3], 1, 0, 1, 100);

    let advance_work = if condition.pressure_outward {
        substrate.advance_time(10)
    } else {
        WorkLedger::default()
    };
    let base_tick = if condition.pressure_outward { 10 } else { 0 };
    let mut arrivals = Vec::new();
    if condition.positive_a {
        arrivals.push((base_tick, 0, namespace + 0xff01, cells[A]));
    }
    if condition.positive_b {
        arrivals.push((
            base_tick + condition.skew_b,
            0,
            namespace + 0xff02,
            cells[B],
        ));
    }
    if condition.negative {
        arrivals.push((base_tick, 0, namespace + 0xff03, cells[N]));
    }
    if condition.unrelated_only {
        let count = unrelated_load.max(4);
        for occurrence in 0..count {
            arrivals.push((
                base_tick + (occurrence as i64) * 2,
                0,
                namespace + 0xfe00 + occurrence as u64,
                cells[D0],
            ));
        }
    }
    if reverse_insertion {
        arrivals.reverse();
    }
    for (arrival_tick, phase, origin_physical, target) in arrivals {
        substrate.enter(SpikeInput {
            arrival_tick,
            phase,
            origin_physical,
            target,
            impulse: 1,
        });
    }
    let persistent_bytes = substrate.persistent_bytes();
    let execution = substrate.propagate();
    let positive_a_firings = firing_count(&execution, id_offsets[A]);
    let positive_b_firings = firing_count(&execution, id_offsets[B]);
    let negative_firings = firing_count(&execution, id_offsets[N]);
    let convergence_inputs = execution
        .trace
        .iter()
        .filter(|entry| entry.target_physical == id_offsets[K])
        .map(|entry| (entry.tick, entry.impulse, entry.fired))
        .collect::<Vec<_>>();
    let convergence_tick = execution
        .trace
        .iter()
        .find(|entry| entry.target_physical == id_offsets[K] && entry.fired)
        .map(|entry| entry.tick);
    let crossing_ticks = execution
        .crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == id_offsets[K] && crossing.to_physical == id_offsets[O]
        })
        .map(|crossing| crossing.tick)
        .collect::<Vec<_>>();
    let outer_ticks = execution
        .trace
        .iter()
        .filter(|entry| entry.target_physical == id_offsets[O] && entry.fired)
        .map(|entry| entry.tick)
        .collect::<Vec<_>>();
    Observation {
        execution,
        advance_work,
        persistent_bytes,
        outward_live: substrate.arrow_is_live(outward),
        positive_a_firings,
        positive_b_firings,
        negative_firings,
        convergence_inputs,
        convergence_tick,
        crossing_ticks,
        outer_ticks,
    }
}

fn physical_id_offsets(namespace: u64) -> [u64; CELL_COUNT] {
    const OFFSETS: [u64; CELL_COUNT] = [0x31, 0x07, 0x55, 0x19, 0x71, 0x93, 0x42, 0xa8, 0x64];
    let rotation = ((namespace >> 16) as usize) % CELL_COUNT;
    std::array::from_fn(|role| namespace + OFFSETS[(role + rotation) % CELL_COUNT])
}

fn add_arrow(
    substrate: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    delay: i64,
    phase: i32,
    coupling: i32,
    resistance: u32,
) -> ArrowId {
    substrate.add_arrow(ArrowSpec {
        from,
        to,
        delay,
        phase,
        coupling,
        resistance,
    })
}

fn firing_count(execution: &Execution, physical_id: u64) -> usize {
    execution
        .trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id && entry.fired)
        .count()
}

fn render_csv(stage: Stage, rows: &[Row]) -> String {
    let mut csv = String::from(
        "stage,seed,condition,namespace,mirrored,reverse_allocation,reverse_insertion,participant_delay,outward_delay,unrelated_load,a_firings,b_firings,negative_firings,convergence_inputs,convergence_tick,crossing_ticks,outer_ticks,outward_live,naturally_quiescent,duplicate_exact,work,persistent_bytes,start_fingerprint,end_fingerprint,permanent_fingerprint,p0,p1,p2,p3,p4,p5,p6,p7,p8,p9,row_pass\n",
    );
    for row in rows {
        let observation = &row.observation;
        let inputs = observation
            .convergence_inputs
            .iter()
            .map(|(tick, impulse, fired)| format!("{tick}:{impulse}:{fired}"))
            .collect::<Vec<_>>()
            .join("|");
        let crossings = join_ticks(&observation.crossing_ticks);
        let outer = join_ticks(&observation.outer_ticks);
        let clauses = row
            .clauses
            .iter()
            .map(bool::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let row_pass = row.clauses.iter().all(|value| *value);
        csv.push_str(&format!(
            "{},{},{},{:#x},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            stage.label,
            row.seed,
            row.condition,
            row.namespace,
            row.mirrored,
            row.reverse_allocation,
            row.reverse_insertion,
            row.participant_delay,
            row.outward_delay,
            row.unrelated_load,
            observation.positive_a_firings,
            observation.positive_b_firings,
            observation.negative_firings,
            inputs,
            observation
                .convergence_tick
                .map_or_else(String::new, |tick| tick.to_string()),
            crossings,
            outer,
            observation.outward_live,
            observation.execution.naturally_quiescent,
            row.duplicate_exact,
            2 * (observation.advance_work.total() + observation.execution.work.total()),
            2 * observation.persistent_bytes,
            observation.execution.start_fingerprint,
            observation.execution.end_fingerprint,
            observation.execution.permanent_fingerprint,
            clauses,
            row_pass,
        ));
    }
    csv
}

fn render_report(
    stage: Stage,
    rows: &[Row],
    audit: &str,
    passed_rows: usize,
    passed_claims: usize,
    passed: bool,
) -> String {
    let total_work = rows
        .iter()
        .map(|row| {
            2 * (row.observation.advance_work.total() + row.observation.execution.work.total())
        })
        .sum::<u64>();
    let total_storage = rows
        .iter()
        .map(|row| 2 * row.observation.persistent_bytes)
        .sum::<usize>();
    let closure_rows = rows
        .iter()
        .filter(|row| row.observation.convergence_tick.is_some())
        .count();
    let crossing_rows = rows
        .iter()
        .filter(|row| !row.observation.crossing_ticks.is_empty())
        .count();
    let quiescent = rows
        .iter()
        .filter(|row| row.observation.execution.naturally_quiescent)
        .count();
    let duplicates = rows.iter().filter(|row| row.duplicate_exact).count();
    format!(
        "# PX8 physical closure-emission {label} result\n\nOutcome: **DEVELOPMENT {outcome}**.\n\n- Frozen parent: `{parent}`\n- Protocol SHA-256: `{protocol}`\n- Retained substrate SHA-256: `{substrate}`\n- Rows: `{passed_rows}/{rows}`\n- Independently serialized claims: `{passed_claims}/{claims}`\n- Exact duplicate developments: `{duplicates}/{rows}`\n- Physical convergence rows: `{closure_rows}`\n- Ordinary outward-crossing rows: `{crossing_rows}`\n- Naturally quiescent rows: `{quiescent}/{rows}`\n- Ledgered work including duplicates: `{total_work}`\n- Persistent-byte accounting including duplicates: `{total_storage}`\n- New substrate law or representation: `false`\n- Definitive/authority execution: `false`\n- Source and ancestry audit: `{audit}`\n\nThe result is immutable development evidence. It does not advance PX3--PX8 authority.\n",
        label = stage.label,
        outcome = if passed { "POSITIVE" } else { "NEGATIVE" },
        parent = PARENT_COMMIT,
        protocol = PROTOCOL_SHA256,
        substrate = SUBSTRATE_SHA256,
        rows = rows.len(),
        claims = rows.len() * 10,
    )
}

fn join_ticks(ticks: &[i64]) -> String {
    ticks
        .iter()
        .map(i64::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn audit_basis() -> Result<String, String> {
    require_sha256(SOURCE_PATH, SUBSTRATE_SHA256)?;
    require_sha256(
        "results/px0_physical_correspondence_definitive_v3.csv",
        PX0_CSV_SHA256,
    )?;
    require_sha256(
        "results/px1_physical_boundary_roles_definitive.csv",
        PX1_CSV_SHA256,
    )?;
    require_sha256(
        "results/px2_physical_causal_direction_definitive.csv",
        PX2_CSV_SHA256,
    )?;
    require_sha256(PROTOCOL_PATH, PROTOCOL_SHA256)?;
    let ancestry = Command::new("git")
        .args(["merge-base", "--is-ancestor", PARENT_COMMIT, "HEAD"])
        .status()
        .map_err(|error| format!("cannot inspect frozen ancestry: {error}"))?;
    if !ancestry.success() {
        return Err("HEAD does not descend from the frozen PX2 parent".into());
    }
    let manifest = read_to_string("crates/px0-physical-correspondence/Cargo.toml")
        .map_err(|error| format!("cannot read PX0 manifest: {error}"))?;
    let dependency_body = manifest
        .split_once("[dependencies]")
        .map_or("", |(_, body)| body)
        .trim();
    if !dependency_body.is_empty() {
        return Err("PX0 physical crate gained a dependency".into());
    }
    let law = read_to_string(SOURCE_PATH)
        .map_err(|error| format!("cannot read retained substrate law: {error}"))?;
    let lower = law.to_ascii_lowercase();
    for forbidden in [
        "finish",
        "answer",
        "terminal supervision",
        "semantic stop",
        "episode ending",
        "serializer",
        "adapter",
    ] {
        if lower.contains(forbidden) {
            return Err(format!(
                "forbidden organism control term in retained law: {forbidden}"
            ));
        }
    }
    Ok("exact-parent-descendant; exact-parent-artifacts; dependency-free; forbidden-control-absent".into())
}

fn require_sha256(path: &str, expected: &str) -> Result<(), String> {
    let output = Command::new("shasum")
        .args(["-a", "256", path])
        .output()
        .map_err(|error| format!("cannot hash {path}: {error}"))?;
    if !output.status.success() {
        return Err(format!("hash command failed for {path}"));
    }
    let actual = String::from_utf8(output.stdout)
        .map_err(|error| format!("invalid hash output for {path}: {error}"))?
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_string();
    if actual != expected {
        return Err(format!("frozen hash mismatch for {path}: {actual}"));
    }
    Ok(())
}

fn refuse_existing(stage: Stage) -> Result<(), String> {
    for path in [
        stage.csv,
        stage.report,
        stage.staging_csv,
        stage.staging_report,
    ] {
        if Path::new(path).exists() {
            return Err(format!(
                "refusing to overwrite or rerun existing evidence path: {path}"
            ));
        }
    }
    Ok(())
}

fn publish(staging: &str, final_path: &str, contents: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(staging)
        .map_err(|error| format!("cannot create atomic staging artifact {staging}: {error}"))?;
    file.write_all(contents.as_bytes())
        .and_then(|()| file.sync_all())
        .map_err(|error| format!("cannot write atomic staging artifact {staging}: {error}"))?;
    rename(staging, final_path)
        .map_err(|error| format!("cannot publish {staging} as {final_path}: {error}"))
}
