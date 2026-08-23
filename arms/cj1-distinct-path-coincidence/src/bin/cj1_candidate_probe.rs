#![forbid(unsafe_code)]

use cj1_distinct_path_coincidence::{
    ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX0_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PARENT_PROTOCOL_SHA256: &str =
    "8c31387f3337c3ad38d83e030dd6a43d4fce8f2e146d93596c9c7231e8b8a6ad";
const PROBE_PROTOCOL_SHA256: &str =
    "9481bdf46793475ac6ca28d35910ddc51dd7cafbbd94ea9a7ae7ffc2f68b9984";
const SEEDS: [u64; 2] = [2101, 2111];
const CSV_PATH: &str = "../../results/cj1_candidate_probe_v1.csv";
const MD_PATH: &str = "../../results/cj1_candidate_probe_v1.md";
const CSV_STAGE: &str = "../../results/.cj1_candidate_probe_v1.csv.staging";
const MD_STAGE: &str = "../../results/.cj1_candidate_probe_v1.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    RepeatedOnePath,
    StrongEntryOnePath,
    DifferentPaths,
    DifferentPathsLate,
    DifferentPathsInside,
    FourOnePath,
    RepeatedAWithB,
    AWithRepeatedB,
    ThreePaths,
    MatureOnePath,
    FreshDifferentPaths,
    OneOriginTwoPaths,
    TwoOriginsSharedPath,
}

impl Scenario {
    const ALL: [Self; 13] = [
        Self::RepeatedOnePath,
        Self::StrongEntryOnePath,
        Self::DifferentPaths,
        Self::DifferentPathsLate,
        Self::DifferentPathsInside,
        Self::FourOnePath,
        Self::RepeatedAWithB,
        Self::AWithRepeatedB,
        Self::ThreePaths,
        Self::MatureOnePath,
        Self::FreshDifferentPaths,
        Self::OneOriginTwoPaths,
        Self::TwoOriginsSharedPath,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::RepeatedOnePath => "two-unit-firings-one-path",
            Self::StrongEntryOnePath => "one-impulse-two-one-path",
            Self::DifferentPaths => "two-unit-different-paths",
            Self::DifferentPathsLate => "different-paths-outside-window",
            Self::DifferentPathsInside => "different-paths-inside-window",
            Self::FourOnePath => "four-firings-one-path",
            Self::RepeatedAWithB => "repeated-a-plus-b",
            Self::AWithRepeatedB => "a-plus-repeated-b",
            Self::ThreePaths => "three-different-paths",
            Self::MatureOnePath => "mature-one-path",
            Self::FreshDifferentPaths => "fresh-different-paths",
            Self::OneOriginTwoPaths => "one-origin-two-paths",
            Self::TwoOriginsSharedPath => "two-origins-shared-path",
        }
    }

    fn expected_effect(self) -> usize {
        match self {
            Self::DifferentPaths
            | Self::DifferentPathsInside
            | Self::RepeatedAWithB
            | Self::AWithRepeatedB
            | Self::ThreePaths
            | Self::FreshDifferentPaths
            | Self::OneOriginTwoPaths => 1,
            Self::RepeatedOnePath
            | Self::StrongEntryOnePath
            | Self::DifferentPathsLate
            | Self::FourOnePath
            | Self::MatureOnePath
            | Self::TwoOriginsSharedPath => 0,
        }
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    sources: [CellId; 3],
    shared: CellId,
    locus: CellId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    scenario: &'static str,
    entered_impulse: i32,
    source_firings: usize,
    traversals: usize,
    carried_impulse: i32,
    trace_writes: u64,
    trace_closes: u64,
    locus_arrivals: usize,
    local_contribution: i32,
    locus_firings: usize,
    effects: usize,
    expected_effect: usize,
    heldout_effects: usize,
    work: u64,
    persistent_bytes: usize,
    temporary_peak_lower_bound: usize,
    deallocations: u64,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    quiescent: bool,
    replay_equal: bool,
    passed: bool,
}

fn main() {
    assert_eq!(
        sha256("../../crates/px0-physical-correspondence/src/lib.rs"),
        PX0_SHA256
    );
    assert_eq!(
        sha256("../../experiments/cj1_distinct_path_coincidence_development_protocol_v1.md"),
        PARENT_PROTOCOL_SHA256
    );
    assert_eq!(
        sha256("../../experiments/cj1_candidate_probe_protocol_v1.md"),
        PROBE_PROTOCOL_SHA256
    );
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.as_slice() != ["--candidate-probe"] {
        eprintln!("CJ1 candidate permits only its frozen development PROBE");
        std::process::exit(2);
    }

    require_absent(&[CSV_PATH, MD_PATH, CSV_STAGE, MD_STAGE]);
    eprintln!("CJ1_PATH_LOCAL_SATURATION_CANDIDATE_PROBE_EVIDENCE");
    let rows = SEEDS
        .iter()
        .copied()
        .flat_map(|seed| {
            Scenario::ALL
                .iter()
                .copied()
                .enumerate()
                .map(move |(index, scenario)| run_replay(seed, index, scenario))
        })
        .collect::<Vec<_>>();
    publish(CSV_STAGE, CSV_PATH, &csv(&rows));
    publish(MD_STAGE, MD_PATH, &report(&rows));
}

fn run_replay(seed: u64, index: usize, scenario: Scenario) -> Row {
    let namespace = (seed << 32) | ((index as u64 + 1) << 16);
    let first = run(seed, scenario, namespace);
    let second = run(seed, scenario, namespace);
    let replay_equal = first == second;
    let mut row = first;
    row.replay_equal = replay_equal;
    row.passed &= replay_equal;
    row
}

fn run(seed: u64, scenario: Scenario, namespace: u64) -> Row {
    let mut world = build_world(namespace, scenario);
    let entered_impulse = enter_schedule(&mut world, scenario, 0);
    let execution = world.substrate.propagate();
    let source_firings = world
        .sources
        .iter()
        .map(|cell| firings(&execution, physical(&world, *cell)))
        .sum::<usize>()
        + firings(&execution, namespace + 40);
    let crossings = execution
        .crossings
        .iter()
        .filter(|crossing| crossing.to_physical == namespace + 50)
        .collect::<Vec<_>>();
    let traversals = crossings.len();
    let carried_impulse = crossings.iter().map(|crossing| crossing.impulse).sum();
    let locus_entries = execution
        .trace
        .iter()
        .filter(|entry| entry.target_physical == namespace + 50)
        .collect::<Vec<_>>();
    let locus_arrivals = locus_entries.len();
    let local_contribution = locus_entries.iter().map(|entry| entry.impulse).sum();
    let locus_firings = firings(&execution, namespace + 50);
    let effects = effects(&execution, namespace);
    let heldout_effects = heldout(&world, scenario);
    let expected_effect = scenario.expected_effect();
    let passed = effects == expected_effect
        && locus_firings == expected_effect
        && heldout_effects == expected_effect
        && execution.naturally_quiescent;
    Row {
        seed,
        scenario: scenario.name(),
        entered_impulse,
        source_firings,
        traversals,
        carried_impulse,
        trace_writes: execution.work.local_eligibility_writes,
        trace_closes: execution.work.local_return_updates,
        locus_arrivals,
        local_contribution,
        locus_firings,
        effects,
        expected_effect,
        heldout_effects,
        work: execution.work.total(),
        persistent_bytes: world.substrate.persistent_bytes(),
        temporary_peak_lower_bound: execution.trace.len()
            * std::mem::size_of_val(&execution.trace[0])
            + execution.crossings.len()
                * execution.crossings.first().map_or(0, std::mem::size_of_val),
        deallocations: execution.work.physical_deallocations,
        complete_fingerprint: execution.end_fingerprint,
        permanent_fingerprint: execution.permanent_fingerprint,
        quiescent: execution.naturally_quiescent,
        replay_equal: false,
        passed,
    }
}

fn build_world(namespace: u64, scenario: Scenario) -> World {
    let mut substrate = PlasticSubstrate::new();
    let sources = [0, 1, 2]
        .map(|side| substrate.add_cell(cell(namespace + 10 + side, side as i32 * 20, -1, 2)));
    let shared = substrate.add_cell(cell(namespace + 40, 70, -1, 2));
    let locus = substrate.add_cell(cell(namespace + 50, 100, 0, 2));
    let effect = substrate.add_cell(cell(namespace + 60, 200, 1, 1));
    let coupling_a = if scenario == Scenario::MatureOnePath {
        2
    } else {
        1
    };
    if scenario == Scenario::TwoOriginsSharedPath {
        substrate.add_arrow(arrow(sources[0], shared, 1));
        substrate.add_arrow(arrow(sources[1], shared, 1));
        substrate.add_arrow(arrow(shared, locus, 1));
    } else {
        substrate.add_arrow(arrow(sources[0], locus, coupling_a));
    }
    if scenario == Scenario::OneOriginTwoPaths {
        substrate.add_arrow(arrow(sources[0], locus, 1));
    } else if scenario != Scenario::TwoOriginsSharedPath {
        substrate.add_arrow(arrow(sources[1], locus, 1));
        substrate.add_arrow(arrow(sources[2], locus, 1));
    }
    substrate.add_arrow(arrow(locus, effect, 1));
    World {
        substrate,
        namespace,
        sources,
        shared,
        locus,
    }
}

fn enter_schedule(world: &mut World, scenario: Scenario, base: i64) -> i32 {
    let mut total = 0;
    let mut enter = |target: CellId, tick: i64, impulse: i32, phase: i32| {
        world.substrate.enter(SpikeInput {
            arrival_tick: base + tick,
            phase,
            origin_physical: world.namespace + 1_000 + phase as u64,
            target,
            impulse,
        });
        total += impulse;
    };
    match scenario {
        Scenario::RepeatedOnePath => {
            enter(world.sources[0], 0, 2, 0);
            enter(world.sources[0], 0, 2, 1);
        }
        Scenario::StrongEntryOnePath => enter(world.sources[0], 0, 2, 0),
        Scenario::DifferentPaths
        | Scenario::DifferentPathsInside
        | Scenario::FreshDifferentPaths
        | Scenario::TwoOriginsSharedPath => {
            enter(world.sources[0], 0, 2, 0);
            enter(world.sources[1], 0, 2, 1);
        }
        Scenario::DifferentPathsLate => {
            enter(world.sources[0], 0, 2, 0);
            enter(world.sources[1], 2, 2, 1);
        }
        Scenario::FourOnePath => {
            for phase in 0..4 {
                enter(world.sources[0], 0, 2, phase);
            }
        }
        Scenario::RepeatedAWithB => {
            enter(world.sources[0], 0, 2, 0);
            enter(world.sources[0], 0, 2, 1);
            enter(world.sources[1], 0, 2, 2);
        }
        Scenario::AWithRepeatedB => {
            enter(world.sources[0], 0, 2, 0);
            enter(world.sources[1], 0, 2, 1);
            enter(world.sources[1], 0, 2, 2);
        }
        Scenario::ThreePaths => {
            for side in 0..3 {
                enter(world.sources[side], 0, 2, side as i32);
            }
        }
        Scenario::MatureOnePath => enter(world.sources[0], 0, 2, 0),
        Scenario::OneOriginTwoPaths => enter(world.sources[0], 0, 2, 0),
    }
    total
}

fn heldout(world: &World, scenario: Scenario) -> usize {
    let mut clone = world.clone();
    clone.substrate.advance_time(20);
    enter_schedule(&mut clone, scenario, 20);
    effects(&clone.substrate.propagate(), clone.namespace)
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

fn arrow(from: CellId, to: CellId, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay: 0,
        phase: 0,
        coupling,
        resistance: 100,
    }
}

fn physical(world: &World, cell: CellId) -> u64 {
    if cell == world.shared {
        world.namespace + 40
    } else if cell == world.locus {
        world.namespace + 50
    } else {
        let side = world
            .sources
            .iter()
            .position(|candidate| *candidate == cell)
            .expect("known source");
        world.namespace + 10 + side as u64
    }
}

fn firings(run: &Execution, physical_id: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id && entry.fired)
        .count()
}

fn effects(run: &Execution, namespace: u64) -> usize {
    run.crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == namespace + 50 && crossing.to_physical == namespace + 60
        })
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
        "seed,scenario,entered_impulse,source_firings,traversals,carried_impulse,trace_writes,trace_closes,locus_arrivals,local_contribution,locus_firings,effects,expected_effect,heldout_effects,work,persistent_bytes,temporary_peak_bytes_lower_bound,deallocations,complete_fingerprint,permanent_fingerprint,quiescent,replay_equal,passed\n",
    );
    for row in rows {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.seed,
            row.scenario,
            row.entered_impulse,
            row.source_firings,
            row.traversals,
            row.carried_impulse,
            row.trace_writes,
            row.trace_closes,
            row.locus_arrivals,
            row.local_contribution,
            row.locus_firings,
            row.effects,
            row.expected_effect,
            row.heldout_effects,
            row.work,
            row.persistent_bytes,
            row.temporary_peak_lower_bound,
            row.deallocations,
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
    let passed = rows.iter().filter(|row| row.passed).count();
    let first = rows.iter().find(|row| !row.passed);
    let first_label = first
        .map(|row| format!("{}:{}", row.seed, row.scenario))
        .unwrap_or_else(|| "none".to_string());
    let first_detail = first
        .map(|row| {
            format!(
                "traversals `{}`, carried impulse `{}`, local contribution `{}`, trace closes `{}`, effects `{}` versus expected `{}`",
                row.traversals, row.carried_impulse, row.local_contribution, row.trace_closes,
                row.effects, row.expected_effect
            )
        })
        .unwrap_or_else(|| "none".to_string());
    format!(
        "# CJ1 path-local saturation candidate PROBE v1\n\nOutcome: **{}**.\n\n- rows: `{passed}/{}` passed;\n- earliest ordered failure: `{first_label}`;\n- earliest failure accounting: {first_detail};\n- native work: `{}` operations;\n- summed independently reconstructed persistent bytes: `{}`;\n- all naturally quiescent: `{}`;\n- exact replay: `{}`;\n- authoritative PX0 law changed: `false`;\n- MICRO/GATE/later evidence executed: `false`.\n\n{}\n",
        if passed == rows.len() { "POSITIVE" } else { "NEGATIVE" },
        rows.len(),
        rows.iter().map(|row| row.work).sum::<u64>(),
        rows.iter().map(|row| row.persistent_bytes).sum::<usize>(),
        rows.iter().all(|row| row.quiescent),
        rows.iter().all(|row| row.replay_equal),
        if passed == rows.len() {
            "The sole bounded path-local contribution rule passes PROBE."
        } else {
            "The sole candidate stops at PROBE; no closure, identity or additional state rule is added."
        },
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
    use std::collections::BTreeSet;

    #[test]
    fn probe_dimensions_and_row_keys_are_unique() {
        assert_eq!(SEEDS.len() * Scenario::ALL.len(), 26);
        let keys = SEEDS
            .iter()
            .flat_map(|seed| {
                Scenario::ALL
                    .iter()
                    .map(move |scenario| (*seed, scenario.name()))
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(keys.len(), 26);
    }

    #[test]
    fn expected_partition_is_frozen() {
        assert_eq!(
            Scenario::ALL
                .iter()
                .filter(|scenario| scenario.expected_effect() == 1)
                .count(),
            7
        );
        assert_eq!(
            Scenario::ALL
                .iter()
                .filter(|scenario| scenario.expected_effect() == 0)
                .count(),
            6
        );
    }
}
