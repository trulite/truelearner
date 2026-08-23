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
const PROTOCOL_SHA256: &str = "8c31387f3337c3ad38d83e030dd6a43d4fce8f2e146d93596c9c7231e8b8a6ad";
const CSV_PATH: &str = "../../results/cj1_existing_physics_probe_v1.csv";
const MD_PATH: &str = "../../results/cj1_existing_physics_probe_v1.md";
const CSV_STAGE: &str = "../../results/.cj1_existing_physics_probe_v1.csv.staging";
const MD_STAGE: &str = "../../results/.cj1_existing_physics_probe_v1.md.staging";

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
    name: &'static str,
    entered_impulse: i32,
    source_firings: usize,
    traversals: usize,
    trace_writes: u64,
    trace_closes: u64,
    locus_arrivals: usize,
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
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.as_slice() != ["--existing-probe"] {
        eprintln!("CJ1 permits only --existing-probe at this frozen implementation");
        std::process::exit(2);
    }
    assert_eq!(
        sha256("../../crates/px0-physical-correspondence/src/lib.rs"),
        PX0_SHA256
    );
    assert_eq!(
        sha256("../../experiments/cj1_distinct_path_coincidence_development_protocol_v1.md"),
        PROTOCOL_SHA256
    );
    for path in [CSV_PATH, MD_PATH, CSV_STAGE, MD_STAGE] {
        assert!(
            !Path::new(path).exists(),
            "artifact path must be absent: {path}"
        );
    }
    eprintln!("CJ1_EXISTING_PHYSICS_PROBE_EVIDENCE");

    let rows = Scenario::ALL
        .iter()
        .copied()
        .enumerate()
        .map(|(index, scenario)| run_replay(scenario, 0xC110_0000 + index as u64 * 0x1_0000))
        .collect::<Vec<_>>();
    publish(CSV_STAGE, CSV_PATH, &csv(&rows));
    publish(MD_STAGE, MD_PATH, &report(&rows));
}

fn run_replay(scenario: Scenario, namespace: u64) -> Row {
    let first = run(scenario, namespace);
    let second = run(scenario, namespace);
    let replay_equal = first == second;
    let mut row = first;
    row.replay_equal = replay_equal;
    row.passed &= replay_equal;
    row
}

fn run(scenario: Scenario, namespace: u64) -> Row {
    let mut world = build_world(namespace, scenario);
    let entered_impulse = enter_schedule(&mut world, scenario, 0);
    let execution = world.substrate.propagate();
    let source_firings = world
        .sources
        .iter()
        .map(|cell| firings(&execution, physical(&world, *cell)))
        .sum::<usize>()
        + firings(&execution, namespace + 40);
    let traversals = execution
        .crossings
        .iter()
        .filter(|crossing| crossing.to_physical == namespace + 50)
        .count();
    let locus_arrivals = arrivals(&execution, namespace + 50);
    let locus_firings = firings(&execution, namespace + 50);
    let effects = execution
        .crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == namespace + 50 && crossing.to_physical == namespace + 60
        })
        .count();
    let heldout_effects = heldout(&world, scenario);
    let expected_effect = scenario.expected_effect();
    let passed = effects == expected_effect
        && locus_firings == expected_effect
        && execution.naturally_quiescent;
    Row {
        name: scenario.name(),
        entered_impulse,
        source_firings,
        traversals,
        trace_writes: execution.work.local_eligibility_writes,
        trace_closes: execution.work.local_return_updates,
        locus_arrivals,
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
    substrate.add_arrow(arrow(sources[0], locus, coupling_a));
    if scenario == Scenario::OneOriginTwoPaths {
        substrate.add_arrow(arrow(sources[0], locus, 1));
    } else if scenario == Scenario::TwoOriginsSharedPath {
        substrate.add_arrow(arrow(sources[0], shared, 1));
        substrate.add_arrow(arrow(sources[1], shared, 1));
        substrate.add_arrow(arrow(shared, locus, 1));
    } else {
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
    let run = clone.substrate.propagate();
    run.crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == clone.namespace + 50
                && crossing.to_physical == clone.namespace + 60
        })
        .count()
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

fn arrivals(run: &Execution, physical_id: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id)
        .count()
}

fn firings(run: &Execution, physical_id: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id && entry.fired)
        .count()
}

fn csv(rows: &[Row]) -> String {
    let mut text = String::from(
        "scenario,entered_impulse,source_firings,traversals,trace_writes,trace_closes,locus_arrivals,locus_firings,effects,expected_effect,heldout_effects,work,persistent_bytes,temporary_peak_bytes_lower_bound,deallocations,complete_fingerprint,permanent_fingerprint,quiescent,replay_equal,passed\n",
    );
    for row in rows {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.name,
            row.entered_impulse,
            row.source_firings,
            row.traversals,
            row.trace_writes,
            row.trace_closes,
            row.locus_arrivals,
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
    let first = rows.iter().find(|row| !row.passed).map(|row| row.name);
    let work = rows.iter().map(|row| row.work).sum::<u64>();
    let persistent = rows.iter().map(|row| row.persistent_bytes).sum::<usize>();
    format!(
        "# CJ1 unchanged-physics PROBE v1\n\nOutcome: **{}**.\n\n- rows: `{passed}/{}` passed;\n- first ordered physical collapse: `{}`;\n- native work: `{work}` operations;\n- summed independently reconstructed persistent bytes: `{persistent}`;\n- all naturally quiescent: `{}`;\n- exact replay: `{}`;\n- authoritative PX0 law changed: `false`;\n- later-stage evidence executed: `false`.\n\nThe unchanged substrate preserves unit-path burst inhibition and genuine different-path accumulation, but the first mature coupling-2 path supplies the entire threshold by amplitude. Existing physics therefore does not satisfy the full matched discriminator.\n",
        if passed == rows.len() { "POSITIVE" } else { "NEGATIVE" },
        rows.len(),
        first.unwrap_or("none"),
        rows.iter().all(|row| row.quiescent),
        rows.iter().all(|row| row.replay_equal),
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
    fn frozen_probe_dimension_and_names_are_unique() {
        assert_eq!(Scenario::ALL.len(), 13);
        let names = Scenario::ALL
            .iter()
            .map(|scenario| scenario.name())
            .collect::<BTreeSet<_>>();
        assert_eq!(names.len(), Scenario::ALL.len());
    }

    #[test]
    fn no_later_stage_surface_exists() {
        assert!(!env!("CARGO_MANIFEST_DIR").contains("definitive"));
        assert!(!env!("CARGO_MANIFEST_DIR").contains("authority"));
    }
}
