#![forbid(unsafe_code)]

use px0_physical_correspondence::{
    ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX0_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PROTOCOL_SHA256: &str =
    "4c904459c7684261d1a5c63b1ff16eb3a6dc47dbf8596ec23386f254834c8762";
const SEED: u64 = 2401;
const OFFSETS: [i64; 6] = [0, 1, 2, 3, 4, 5];
const CSV_PATH: &str = "results/cj1_t_refractory_trace_geometry_v1.csv";
const MD_PATH: &str = "results/cj1_t_refractory_trace_geometry_v1.md";
const CSV_STAGE: &str = "results/.cj1_t_refractory_trace_geometry_v1.csv.staging";
const MD_STAGE: &str = "results/.cj1_t_refractory_trace_geometry_v1.md.staging";

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    sources: [CellId; 2],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    case_name: String,
    offset: i64,
    first_source_firings: usize,
    first_traversals: usize,
    advance_pressure_updates: u64,
    trace_live_before_second_arrival: bool,
    second_return_closes: u64,
    second_source_firings: usize,
    second_traversals: usize,
    second_locus_arrivals: usize,
    second_locus_impulse: i32,
    first_locus_firings: usize,
    second_locus_firings: usize,
    cumulative_locus_firings: usize,
    eligibility_writes: u64,
    distinct_a_probe_closes: u64,
    distinct_b_probe_closes: u64,
    work: u64,
    persistent_bytes: usize,
    first_end_fingerprint: u64,
    second_end_fingerprint: u64,
    quiescent: bool,
    replay_equal: bool,
    passed: bool,
}

fn main() {
    assert_eq!(
        sha256("crates/px0-physical-correspondence/src/lib.rs"),
        PX0_SHA256
    );
    assert_eq!(
        sha256("experiments/cj1_t_refractory_trace_geometry_protocol_v1.md"),
        PROTOCOL_SHA256
    );
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.as_slice() != ["--geometry"] {
        eprintln!("CJ1-T permits only its frozen window-geometry diagnostic");
        std::process::exit(2);
    }
    require_absent(&[CSV_PATH, MD_PATH, CSV_STAGE, MD_STAGE]);

    eprintln!("CJ1_T_REFRACTORY_TRACE_GEOMETRY_EVIDENCE");
    let mut rows = OFFSETS
        .iter()
        .copied()
        .map(run_same_path_replay)
        .collect::<Vec<_>>();
    rows.push(run_distinct_path_replay());
    publish(CSV_STAGE, CSV_PATH, &csv(&rows));
    publish(MD_STAGE, MD_PATH, &report(&rows));
}

fn run_same_path_replay(offset: i64) -> Row {
    let first = run_same_path(offset);
    let second = run_same_path(offset);
    let replay_equal = first == second;
    let mut row = first;
    row.replay_equal = replay_equal;
    row.passed &= replay_equal;
    row
}

fn run_same_path(offset: i64) -> Row {
    let namespace = (SEED << 32) | ((offset as u64 + 1) << 16);
    let mut world = build_world(namespace, false);
    enter(&mut world, 0, 0, 0);
    let first = world.substrate.propagate();
    let advance = world.substrate.advance_time(offset);
    enter(&mut world, 0, offset, 1);
    let second = world.substrate.propagate();

    let first_source_firings = firings(&first, namespace + 10);
    let first_traversals = traversals(&first, namespace + 50);
    let second_source_firings = firings(&second, namespace + 10);
    let second_traversals = traversals(&second, namespace + 50);
    let second_locus_arrivals = arrivals(&second, namespace + 50);
    let second_locus_impulse = impulse(&second, namespace + 50);
    let first_locus_firings = firings(&first, namespace + 50);
    let second_locus_firings = firings(&second, namespace + 50);
    let trace_live_before_second_arrival = second.work.local_return_updates == 1;
    let expected_second_traversal = usize::from(offset >= 1);
    let expected_live = offset <= 4;
    let expected_pressure = u64::from(offset == 5);
    let expected_second_arrival = expected_second_traversal;
    let expected_second_impulse = i32::try_from(expected_second_traversal).expect("unit");
    let passed = first_source_firings == 1
        && first_traversals == 1
        && first_locus_firings == 0
        && advance.ordinary_pressure_updates == expected_pressure
        && trace_live_before_second_arrival == expected_live
        && second.work.local_return_updates == u64::from(expected_live)
        && second_source_firings == expected_second_traversal
        && second_traversals == expected_second_traversal
        && second_locus_arrivals == expected_second_arrival
        && second_locus_impulse == expected_second_impulse
        && second_locus_firings == 0
        && first.naturally_quiescent
        && second.naturally_quiescent;

    Row {
        seed: SEED,
        case_name: "same-path".to_string(),
        offset,
        first_source_firings,
        first_traversals,
        advance_pressure_updates: advance.ordinary_pressure_updates,
        trace_live_before_second_arrival,
        second_return_closes: second.work.local_return_updates,
        second_source_firings,
        second_traversals,
        second_locus_arrivals,
        second_locus_impulse,
        first_locus_firings,
        second_locus_firings,
        cumulative_locus_firings: first_locus_firings + second_locus_firings,
        eligibility_writes: first.work.local_eligibility_writes
            + second.work.local_eligibility_writes,
        distinct_a_probe_closes: 0,
        distinct_b_probe_closes: 0,
        work: first.work.total() + advance.total() + second.work.total(),
        persistent_bytes: world.substrate.persistent_bytes(),
        first_end_fingerprint: first.end_fingerprint,
        second_end_fingerprint: second.end_fingerprint,
        quiescent: first.naturally_quiescent && second.naturally_quiescent,
        replay_equal: false,
        passed,
    }
}

fn run_distinct_path_replay() -> Row {
    let first = run_distinct_path();
    let second = run_distinct_path();
    let replay_equal = first == second;
    let mut row = first;
    row.replay_equal = replay_equal;
    row.passed &= replay_equal;
    row
}

fn run_distinct_path() -> Row {
    let namespace = (SEED << 32) | (100 << 16);
    let mut world = build_world(namespace, true);
    enter(&mut world, 0, 0, 0);
    enter(&mut world, 1, 0, 1);
    let first = world.substrate.propagate();

    let mut probe_a = world.clone();
    enter(&mut probe_a, 0, 0, 10);
    let a = probe_a.substrate.propagate();
    let mut probe_b = world.clone();
    enter(&mut probe_b, 1, 0, 11);
    let b = probe_b.substrate.propagate();

    let first_source_firings = firings(&first, namespace + 10) + firings(&first, namespace + 11);
    let first_traversals = traversals(&first, namespace + 50);
    let first_locus_firings = firings(&first, namespace + 50);
    let passed = first_source_firings == 2
        && first_traversals == 2
        && first_locus_firings == 1
        && first.work.local_eligibility_writes == 2
        && a.work.local_return_updates == 1
        && b.work.local_return_updates == 1
        && traversals(&a, namespace + 50) == 0
        && traversals(&b, namespace + 50) == 0
        && first.naturally_quiescent
        && a.naturally_quiescent
        && b.naturally_quiescent;

    Row {
        seed: SEED,
        case_name: "distinct-paths-same-tick".to_string(),
        offset: 0,
        first_source_firings,
        first_traversals,
        advance_pressure_updates: 0,
        trace_live_before_second_arrival: false,
        second_return_closes: 0,
        second_source_firings: 0,
        second_traversals: 0,
        second_locus_arrivals: 0,
        second_locus_impulse: 0,
        first_locus_firings,
        second_locus_firings: 0,
        cumulative_locus_firings: first_locus_firings,
        eligibility_writes: first.work.local_eligibility_writes,
        distinct_a_probe_closes: a.work.local_return_updates,
        distinct_b_probe_closes: b.work.local_return_updates,
        work: first.work.total() + a.work.total() + b.work.total(),
        persistent_bytes: world.substrate.persistent_bytes(),
        first_end_fingerprint: first.end_fingerprint,
        second_end_fingerprint: a.end_fingerprint ^ b.end_fingerprint.rotate_left(1),
        quiescent: first.naturally_quiescent
            && a.naturally_quiescent
            && b.naturally_quiescent,
        replay_equal: false,
        passed,
    }
}

fn build_world(namespace: u64, distinct: bool) -> World {
    let mut substrate = PlasticSubstrate::new();
    let source_a = substrate.add_cell(cell(namespace + 10, 0, -1, 1));
    let source_b = substrate.add_cell(cell(namespace + 11, 20, -1, 1));
    let locus = substrate.add_cell(cell(namespace + 50, 100, 0, 2));
    substrate.add_arrow(arrow(source_a, locus));
    if distinct {
        substrate.add_arrow(arrow(source_b, locus));
    }
    World {
        substrate,
        namespace,
        sources: [source_a, source_b],
    }
}

fn enter(world: &mut World, source: usize, tick: i64, phase: i32) {
    world.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: world.namespace + 1_000 + phase as u64,
        target: world.sources[source],
        impulse: 1,
    });
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 100,
    }
}

fn arrow(from: CellId, to: CellId) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: 100,
    }
}

fn firings(run: &Execution, physical_id: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id && entry.fired)
        .count()
}

fn arrivals(run: &Execution, physical_id: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id)
        .count()
}

fn impulse(run: &Execution, physical_id: u64) -> i32 {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id)
        .map(|entry| entry.impulse)
        .sum()
}

fn traversals(run: &Execution, to_physical: u64) -> usize {
    run.crossings
        .iter()
        .filter(|crossing| crossing.to_physical == to_physical)
        .count()
}

fn require_absent(paths: &[&str]) {
    for path in paths {
        assert!(
            !Path::new(path).exists(),
            "artifact path must be absent: {path}"
        );
    }
}

fn csv(rows: &[Row]) -> String {
    let mut text = String::from(
        "seed,case,offset,first_source_firings,first_traversals,advance_pressure_updates,trace_live_before_second_arrival,second_return_closes,second_source_firings,second_traversals,second_locus_arrivals,second_locus_impulse,first_locus_firings,second_locus_firings,cumulative_locus_firings,eligibility_writes,distinct_a_probe_closes,distinct_b_probe_closes,work,persistent_bytes,first_end_fingerprint,second_end_fingerprint,quiescent,replay_equal,passed\n",
    );
    for row in rows {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.seed,
            row.case_name,
            row.offset,
            row.first_source_firings,
            row.first_traversals,
            row.advance_pressure_updates,
            row.trace_live_before_second_arrival,
            row.second_return_closes,
            row.second_source_firings,
            row.second_traversals,
            row.second_locus_arrivals,
            row.second_locus_impulse,
            row.first_locus_firings,
            row.second_locus_firings,
            row.cumulative_locus_firings,
            row.eligibility_writes,
            row.distinct_a_probe_closes,
            row.distinct_b_probe_closes,
            row.work,
            row.persistent_bytes,
            row.first_end_fingerprint,
            row.second_end_fingerprint,
            row.quiescent,
            row.replay_equal,
            row.passed,
        ));
    }
    text
}

fn report(rows: &[Row]) -> String {
    let same = rows
        .iter()
        .filter(|row| row.case_name == "same-path")
        .collect::<Vec<_>>();
    let distinct = rows
        .iter()
        .find(|row| row.case_name == "distinct-paths-same-tick")
        .expect("distinct control");
    let first_retraversal = same
        .iter()
        .find(|row| row.second_traversals == 1)
        .map(|row| row.offset);
    let last_live_arrival = same
        .iter()
        .filter(|row| row.trace_live_before_second_arrival)
        .map(|row| row.offset)
        .max();
    let live_retraversal = same.iter().any(|row| {
        row.trace_live_before_second_arrival
            && row.second_return_closes == 1
            && row.second_traversals == 1
    });
    let retained_retraversal = same.iter().any(|row| {
        row.offset <= 4 && row.second_traversals == 1 && row.second_return_closes == 0
    });
    let all_passed = rows.iter().all(|row| row.passed);
    let geometry = if live_retraversal && !retained_retraversal {
        "R < T; ordinary local return closes the old trace before retraversal"
    } else if retained_retraversal {
        "R < T; retraversal occurs without an intervening eligibility close"
    } else {
        "R >= T; no post-refractory retraversal occurs while eligibility is live"
    };
    format!(
        "# CJ1-T refractory/trace window geometry v1\n\nOutcome: **{}**.\n\n- geometry: **{geometry}**;\n- first actual same-path retraversal offset: `{}`;\n- last offset with first trace observed live at second arrival: `{}`;\n- same-path threshold-2 firings across sweep: `{}`;\n- distinct same-tick traversals/local firings: `{}/{}`;\n- independent A/B live-trace probes: `{}/{}`;\n- rows: `{}/{}` passed;\n- exact replay: `{}`;\n- all naturally quiescent: `{}`;\n- native work: `{}` operations;\n- authoritative PX0 law changed: `false`;\n- candidate/CJ1 MICRO/GATE executed: `false`.\n\nThe unchanged unit-coupling physics admits a second same-path traversal at offsets 1 through 4 while the first eligibility is live immediately before the source arrival. That arrival performs ordinary local-return closure before the source fires and writes the new traversal trace. The receiving unit contribution has also decayed before every possible retraversal, so same-path repetition never fires the threshold-2 locus. Two distinct paths traverse at the same tick, retain independently observable live traces, and fire the locus once. This geometry resolves repetition, but it does not repair CJ1's separate mature coupling-2 amplitude substitution.\n",
        if all_passed { "POSITIVE GEOMETRY" } else { "INVALID" },
        first_retraversal.map_or_else(|| "none".to_string(), |value| value.to_string()),
        last_live_arrival.map_or_else(|| "none".to_string(), |value| value.to_string()),
        same.iter().map(|row| row.cumulative_locus_firings).sum::<usize>(),
        distinct.first_traversals,
        distinct.first_locus_firings,
        distinct.distinct_a_probe_closes,
        distinct.distinct_b_probe_closes,
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
        rows.iter().all(|row| row.replay_equal),
        rows.iter().all(|row| row.quiescent),
        rows.iter().map(|row| row.work).sum::<u64>(),
    )
}

fn publish(staging: &str, destination: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(staging)
        .expect("create fresh staging artifact");
    file.write_all(contents.as_bytes()).expect("write artifact");
    file.sync_all().expect("sync artifact");
    rename(staging, destination).expect("publish artifact atomically");
}

fn sha256(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("run sha256sum");
    assert!(output.status.success(), "hash {path}");
    String::from_utf8(output.stdout)
        .expect("utf8 hash")
        .split_whitespace()
        .next()
        .expect("digest")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_dimensions_are_frozen() {
        assert_eq!(OFFSETS, [0, 1, 2, 3, 4, 5]);
        assert_eq!(OFFSETS.len() + 1, 7);
    }

    #[test]
    fn only_expected_artifact_surface_exists() {
        assert!(CSV_PATH.contains("geometry"));
        assert!(MD_PATH.contains("geometry"));
        assert!(!CSV_PATH.contains("definitive"));
        assert!(!MD_PATH.contains("authority"));
    }

    #[test]
    fn work_ledger_remains_native_px0_accounting() {
        assert_eq!(px0_physical_correspondence::WorkLedger::default().total(), 0);
    }
}
