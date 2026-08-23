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
const PX1_PT0_SHA256: &str = "f0b754ed6f7b0603668319a0735da91b4c168f909d4024fd5ce5e2aea4197410";
const PX1_DEFINITIVE_SOURCE_SHA256: &str =
    "74716c87d146cb697b37ddf802c12e67a5cb93daf82ec20f8b982e54922bd696";
const PX1_DEFINITIVE_CSV_SHA256: &str =
    "6613ff0a96bb3a60fbe7afeb92cd64edced3c6df5dcc04fe47518db158dd88f6";
const PX1_RESULT_AUDIT_SHA256: &str =
    "fa4a516fcb6977a45e547ca1bb3b7db3b427c05b381fb60d2700e92fa2ae7c70";
const PX1_HANDOFF_SHA256: &str = "ab4142a24f6ca1095c1c1364f391253752808382ac6ee70ef9d49eac722df28c";
const PROTOCOL_SHA256: &str = "c5827a66f693a2bd6a3558a2fbcece5b38ae1f1c5a66eedcc25bb97d37853abd";
const SEED: u64 = 2501;
const CSV_PATH: &str = "results/cj1_pa_participation_amplitude_geometry_v1.csv";
const MD_PATH: &str = "results/cj1_pa_participation_amplitude_geometry_v1.md";
const CSV_STAGE: &str = "results/.cj1_pa_participation_amplitude_geometry_v1.csv.staging";
const MD_STAGE: &str = "results/.cj1_pa_participation_amplitude_geometry_v1.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scenario {
    name: &'static str,
    couplings: [i32; 2],
}

impl Scenario {
    const ALL: [Self; 6] = [
        Self {
            name: "a1",
            couplings: [1, 0],
        },
        Self {
            name: "a2",
            couplings: [2, 0],
        },
        Self {
            name: "a4",
            couplings: [4, 0],
        },
        Self {
            name: "a1-b1",
            couplings: [1, 1],
        },
        Self {
            name: "a2-b1",
            couplings: [2, 1],
        },
        Self {
            name: "a4-b4",
            couplings: [4, 4],
        },
    ];

    fn active(self) -> [bool; 2] {
        self.couplings.map(|coupling| coupling > 0)
    }

    fn active_count(self) -> usize {
        self.active().into_iter().map(usize::from).sum()
    }

    fn expected_conjunction(self) -> usize {
        usize::from(self.active_count() == 2)
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    sources: [CellId; 2],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    scenario: &'static str,
    couplings: [i32; 2],
    scheduled_sources: [usize; 2],
    source_firings: [usize; 2],
    raw_traversals: [usize; 2],
    raw_impulse: [i32; 2],
    outlet_firings: [usize; 2],
    unit_participation_crossings: [usize; 2],
    hub_input_crossings: usize,
    hub_firings: usize,
    hub_return_crossings: [usize; 2],
    trace_arrivals: [usize; 2],
    trace_arrival_impulse: [i32; 2],
    trace_firings: [usize; 2],
    unit_conjunction_crossings: [usize; 2],
    conjunction_arrivals: usize,
    conjunction_impulse: i32,
    conjunction_firings: usize,
    expected_conjunction: usize,
    work: u64,
    persistent_bytes: usize,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    quiescent: bool,
    replay_equal: bool,
    passed: bool,
}

fn main() {
    source_audit();
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.as_slice() != ["--geometry"] {
        eprintln!("CJ1-PA permits only its frozen participation/amplitude geometry");
        std::process::exit(2);
    }
    require_absent(&[CSV_PATH, MD_PATH, CSV_STAGE, MD_STAGE]);

    eprintln!("CJ1_PA_PARTICIPATION_AMPLITUDE_GEOMETRY_EVIDENCE");
    let rows = Scenario::ALL
        .iter()
        .copied()
        .enumerate()
        .map(|(index, scenario)| run_replay(index, scenario))
        .collect::<Vec<_>>();
    publish(CSV_STAGE, CSV_PATH, &csv(&rows));
    publish(MD_STAGE, MD_PATH, &report(&rows));
}

fn source_audit() {
    let frozen = [
        ("crates/px0-physical-correspondence/src/lib.rs", PX0_SHA256),
        (
            "crates/px0-physical-correspondence/examples/px1_pt0_physical_participation_trace.rs",
            PX1_PT0_SHA256,
        ),
        (
            "crates/px0-physical-correspondence/examples/px1_pt1_attributed_margin_stability.rs",
            PX1_DEFINITIVE_SOURCE_SHA256,
        ),
        (
            "results/px1_physical_boundary_roles_definitive.csv",
            PX1_DEFINITIVE_CSV_SHA256,
        ),
        (
            "experiments/px1_physical_boundary_roles_definitive_result_audit.md",
            PX1_RESULT_AUDIT_SHA256,
        ),
        (
            "experiments/px1_physical_boundary_roles_authority_handoff.md",
            PX1_HANDOFF_SHA256,
        ),
        (
            "experiments/cj1_pa_participation_amplitude_geometry_protocol_v1.md",
            PROTOCOL_SHA256,
        ),
    ];
    for (path, expected) in frozen {
        assert_eq!(sha256(path), expected, "frozen input hash drift: {path}");
    }
}

fn run_replay(index: usize, scenario: Scenario) -> Row {
    let first = run(index, scenario);
    let second = run(index, scenario);
    let replay_equal = first == second;
    let mut row = first;
    row.replay_equal = replay_equal;
    row.passed &= replay_equal;
    row
}

fn run(index: usize, scenario: Scenario) -> Row {
    let namespace = (SEED << 32) | ((index as u64 + 1) << 16);
    let active = scenario.active();
    let mut world = build_world(namespace, scenario.couplings);
    for (side, is_active) in active.into_iter().enumerate() {
        if is_active {
            world.substrate.enter(SpikeInput {
                arrival_tick: 0,
                phase: side as i32,
                origin_physical: namespace + 1_000 + side as u64,
                target: world.sources[side],
                impulse: 1,
            });
        }
    }
    let execution = world.substrate.propagate();

    let scheduled_sources = active.map(usize::from);
    let source_firings = pair(|side| firings(&execution, source_physical(namespace, side)));
    let raw_traversals = pair(|side| {
        crossings(
            &execution,
            source_physical(namespace, side),
            outlet_physical(namespace, side),
        )
    });
    let raw_impulse = pair(|side| {
        crossing_impulse(
            &execution,
            source_physical(namespace, side),
            outlet_physical(namespace, side),
        )
    });
    let outlet_firings = pair(|side| firings(&execution, outlet_physical(namespace, side)));
    let unit_participation_crossings = pair(|side| {
        crossings(
            &execution,
            outlet_physical(namespace, side),
            trace_physical(namespace, side),
        )
    });
    let hub_input_crossings = (0..2)
        .map(|side| {
            crossings(
                &execution,
                outlet_physical(namespace, side),
                hub_physical(namespace),
            )
        })
        .sum();
    let hub_firings = firings(&execution, hub_physical(namespace));
    let hub_return_crossings = pair(|side| {
        crossings(
            &execution,
            hub_physical(namespace),
            trace_physical(namespace, side),
        )
    });
    let trace_arrivals = pair(|side| arrivals(&execution, trace_physical(namespace, side)));
    let trace_arrival_impulse = pair(|side| impulse(&execution, trace_physical(namespace, side)));
    let trace_firings = pair(|side| firings(&execution, trace_physical(namespace, side)));
    let unit_conjunction_crossings = pair(|side| {
        crossings(
            &execution,
            trace_physical(namespace, side),
            conjunction_physical(namespace),
        )
    });
    let conjunction_arrivals = arrivals(&execution, conjunction_physical(namespace));
    let conjunction_impulse = impulse(&execution, conjunction_physical(namespace));
    let conjunction_firings = firings(&execution, conjunction_physical(namespace));
    let active_count = scenario.active_count();
    let expected_trace_arrivals = active.map(|is_active| 1 + usize::from(is_active));
    let expected_trace_impulse = expected_trace_arrivals.map(|value| value as i32);
    let expected_conjunction = scenario.expected_conjunction();
    let passed = scheduled_sources == active.map(usize::from)
        && source_firings == active.map(usize::from)
        && raw_traversals == active.map(usize::from)
        && raw_impulse == scenario.couplings
        && outlet_firings == active.map(usize::from)
        && unit_participation_crossings == active.map(usize::from)
        && hub_input_crossings == active_count
        && hub_firings == 1
        && hub_return_crossings == [1, 1]
        && trace_arrivals == expected_trace_arrivals
        && trace_arrival_impulse == expected_trace_impulse
        && trace_firings == active.map(usize::from)
        && unit_conjunction_crossings == active.map(usize::from)
        && conjunction_arrivals == active_count
        && conjunction_impulse == active_count as i32
        && conjunction_firings == expected_conjunction
        && execution.naturally_quiescent;

    Row {
        scenario: scenario.name,
        couplings: scenario.couplings,
        scheduled_sources,
        source_firings,
        raw_traversals,
        raw_impulse,
        outlet_firings,
        unit_participation_crossings,
        hub_input_crossings,
        hub_firings,
        hub_return_crossings,
        trace_arrivals,
        trace_arrival_impulse,
        trace_firings,
        unit_conjunction_crossings,
        conjunction_arrivals,
        conjunction_impulse,
        conjunction_firings,
        expected_conjunction,
        work: execution.work.total(),
        persistent_bytes: world.substrate.persistent_bytes(),
        complete_fingerprint: execution.end_fingerprint,
        permanent_fingerprint: execution.permanent_fingerprint,
        quiescent: execution.naturally_quiescent,
        replay_equal: false,
        passed,
    }
}

fn build_world(namespace: u64, couplings: [i32; 2]) -> World {
    let mut substrate = PlasticSubstrate::new();
    let sources = pair(|side| {
        substrate.add_cell(cell(
            source_physical(namespace, side),
            side as i32 * 100,
            -1,
            1,
        ))
    });
    let outlets = pair(|side| {
        substrate.add_cell(cell(
            outlet_physical(namespace, side),
            20 + side as i32 * 100,
            0,
            1,
        ))
    });
    let traces = pair(|side| {
        substrate.add_cell(cell(
            trace_physical(namespace, side),
            40 + side as i32 * 100,
            1,
            2,
        ))
    });
    let hub = substrate.add_cell(cell(hub_physical(namespace), 300, 2, 1));
    let conjunction = substrate.add_cell(cell(conjunction_physical(namespace), 500, 3, 2));

    for side in 0..2 {
        if couplings[side] > 0 {
            substrate.add_arrow(arrow(sources[side], outlets[side], 0, couplings[side]));
        }
        substrate.add_arrow(arrow(outlets[side], traces[side], 1, 1));
        substrate.add_arrow(arrow(outlets[side], hub, 1, 1));
        substrate.add_arrow(arrow(hub, traces[side], 0, 1));
        substrate.add_arrow(arrow(traces[side], conjunction, 0, 1));
    }
    World { substrate, sources }
}

fn pair<T>(mut make: impl FnMut(usize) -> T) -> [T; 2] {
    [make(0), make(1)]
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

fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 100,
    }
}

fn source_physical(namespace: u64, side: usize) -> u64 {
    namespace + 10 + side as u64
}

fn outlet_physical(namespace: u64, side: usize) -> u64 {
    namespace + 20 + side as u64
}

fn trace_physical(namespace: u64, side: usize) -> u64 {
    namespace + 30 + side as u64
}

fn hub_physical(namespace: u64) -> u64 {
    namespace + 40
}

fn conjunction_physical(namespace: u64) -> u64 {
    namespace + 50
}

fn firings(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn arrivals(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical)
        .count()
}

fn impulse(run: &Execution, physical: u64) -> i32 {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical)
        .map(|entry| entry.impulse)
        .sum()
}

fn crossings(run: &Execution, from: u64, to: u64) -> usize {
    run.crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .count()
}

fn crossing_impulse(run: &Execution, from: u64, to: u64) -> i32 {
    run.crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .map(|crossing| crossing.impulse)
        .sum()
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
        "scenario,couplings,scheduled_sources,source_firings,raw_traversals,raw_impulse,outlet_firings,unit_participation_crossings,hub_input_crossings,hub_firings,hub_return_crossings,trace_arrivals,trace_arrival_impulse,trace_firings,unit_conjunction_crossings,conjunction_arrivals,conjunction_impulse,conjunction_firings,expected_conjunction,work,persistent_bytes,complete_fingerprint,permanent_fingerprint,quiescent,replay_equal,passed\n",
    );
    for row in rows {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.scenario,
            pair_i32(row.couplings),
            pair_usize(row.scheduled_sources),
            pair_usize(row.source_firings),
            pair_usize(row.raw_traversals),
            pair_i32(row.raw_impulse),
            pair_usize(row.outlet_firings),
            pair_usize(row.unit_participation_crossings),
            row.hub_input_crossings,
            row.hub_firings,
            pair_usize(row.hub_return_crossings),
            pair_usize(row.trace_arrivals),
            pair_i32(row.trace_arrival_impulse),
            pair_usize(row.trace_firings),
            pair_usize(row.unit_conjunction_crossings),
            row.conjunction_arrivals,
            row.conjunction_impulse,
            row.conjunction_firings,
            row.expected_conjunction,
            row.work,
            row.persistent_bytes,
            row.complete_fingerprint,
            row.permanent_fingerprint,
            row.quiescent,
            row.replay_equal,
            row.passed,
        ));
    }
    text
}

fn report(rows: &[Row]) -> String {
    let singles = rows
        .iter()
        .filter(|row| row.couplings[1] == 0)
        .collect::<Vec<_>>();
    let pairs = rows
        .iter()
        .filter(|row| row.couplings[1] > 0)
        .collect::<Vec<_>>();
    let amplitude_invariant = singles.iter().all(|row| {
        row.outlet_firings == [1, 0] && row.trace_firings == [1, 0] && row.conjunction_firings == 0
    });
    let distinct_invariant = pairs.iter().all(|row| {
        row.raw_traversals == [1, 1] && row.trace_firings == [1, 1] && row.conjunction_firings == 1
    });
    let all_passed = rows.iter().all(|row| row.passed);
    format!(
        "# CJ1-PA participation/amplitude geometry v1\n\nOutcome: **{}**.\n\n- raw single-path amplitudes tested: `1, 2, 4`;\n- single-path trace firings: `{}`;\n- single-path conjunction firings: `{}`;\n- two-path raw amplitude sums tested: `2, 3, 8`;\n- two-path trace firings: `{}`;\n- two-path conjunction firings: `{}`;\n- amplitude-invariant one-participant mapping: `{amplitude_invariant}`;\n- distinct two-participant mapping: `{distinct_invariant}`;\n- rows: `{}/{}` passed;\n- exact replay: `{}`;\n- all naturally quiescent: `{}`;\n- native work: `{}` operations;\n- authoritative PX0/PX1 changed: `false`;\n- new mechanism or conjunction law added: `false`;\n- definitive/authority/PX3/PX-C executed: `false`.\n\n{}\n",
        if all_passed { "POSITIVE GEOMETRY" } else { "NEGATIVE" },
        triples(&singles, |row| row.trace_firings.iter().sum()),
        triples(&singles, |row| row.conjunction_firings),
        triples(&pairs, |row| row.trace_firings.iter().sum()),
        triples(&pairs, |row| row.conjunction_firings),
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
        rows.iter().all(|row| row.replay_equal),
        rows.iter().all(|row| row.quiescent),
        rows.iter().map(|row| row.work).sum::<u64>(),
        if all_passed {
            "The existing PX1 trace-cell layer converts each actually participating path to one ordinary unit firing regardless of upstream coupling. A threshold-two downstream CELL therefore responds to two physical participants, not one strong path."
        } else {
            "The existing PX1 trace-cell layer does not remove the mature-amplitude alias under the frozen matrix."
        },
    )
}

fn triples(rows: &[&Row], value: impl Fn(&Row) -> usize) -> String {
    rows.iter()
        .map(|row| value(row).to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn pair_i32(values: [i32; 2]) -> String {
    format!("{}|{}", values[0], values[1])
}

fn pair_usize(values: [usize; 2]) -> String {
    format!("{}|{}", values[0], values[1])
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
    use std::collections::BTreeSet;

    #[test]
    fn six_geometry_rows_are_unique() {
        assert_eq!(Scenario::ALL.len(), 6);
        let names = Scenario::ALL
            .iter()
            .map(|row| row.name)
            .collect::<BTreeSet<_>>();
        assert_eq!(names.len(), 6);
    }

    #[test]
    fn frozen_amplitude_partition_is_exact() {
        assert_eq!(
            Scenario::ALL
                .iter()
                .filter(|row| row.active_count() == 1)
                .count(),
            3
        );
        assert_eq!(
            Scenario::ALL
                .iter()
                .filter(|row| row.active_count() == 2)
                .count(),
            3
        );
        assert_eq!(
            Scenario::ALL.map(|row| row.expected_conjunction()),
            [0, 0, 0, 1, 1, 1]
        );
    }
}
