#![forbid(unsafe_code)]

use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX0: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const R3_CSV: &str = "62b34a64396728c28b617bab75cf1141ee2b2db53897ee655809b6180cb2a67b";
const R3_AUDIT: &str = "6f565cf8397afb55e28293360f1ade5aa51b89ba5fa8c19ce0eacaa23086e299";
const R4_SOURCE: &str = "88425c48a141c3dea5177c2581f35506a215c23166c5b90224cd4cf506fa6986";
const R4_CSV: &str = "81d3296ddda223486c3e3d00b01e590cc18889e5fffe85e59e1da825f143b82e";
const R4_REPORT: &str = "bbf02161d4de8f5f64fec16af545e4758f8c8115a2b03c01e3a28cc430f8e25e";
const R4_AUDIT: &str = "75bb603ba21aa9af0d6fab644264b10f6db0d1bf8a1de4a34049e169550b3785";
const PROTOCOL: &str = "6d6635962d08c23d4e546af9f1a74d07e30896ac1df05b6c124fb558bf1ba8d7";

const SEEDS: [u64; 2] = [3601, 3609];
const CSV: &str = "results/px3_r5_three_factor_return_attribution_v1.csv";
const MD: &str = "results/px3_r5_three_factor_return_attribution_v1.md";
const CSV_STAGE: &str = "results/.px3_r5_three_factor_return_attribution_v1.csv.staging";
const MD_STAGE: &str = "results/.px3_r5_three_factor_return_attribution_v1.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Complete,
    PxNoReturn,
    PrXBlocked,
    XrPAbsent,
    PxLateA,
    AdjacentNoReturn,
    Collision,
    TwoCompleted,
}

impl Kind {
    const ALL: [Self; 8] = [
        Self::Complete,
        Self::PxNoReturn,
        Self::PrXBlocked,
        Self::XrPAbsent,
        Self::PxLateA,
        Self::AdjacentNoReturn,
        Self::Collision,
        Self::TwoCompleted,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Complete => "complete-pxr",
            Self::PxNoReturn => "px-no-return",
            Self::PrXBlocked => "pr-x-blocked",
            Self::XrPAbsent => "xr-p-absent",
            Self::PxLateA => "px-late-a-no-return",
            Self::AdjacentNoReturn => "adjacent-ab-no-return",
            Self::Collision => "collision-real-return",
            Self::TwoCompleted => "two-completed-pxr",
        }
    }

    fn index(self) -> u64 {
        Self::ALL
            .iter()
            .position(|value| *value == self)
            .expect("kind") as u64
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    primitive_sources: [CellId; 2],
    p: CellId,
    x: CellId,
    context: CellId,
    effect_driver: CellId,
    return_source: CellId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    primitive: [Vec<i64>; 2],
    primitive_trace: [Vec<i64>; 2],
    opportunity: Vec<i64>,
    p_events: Vec<String>,
    p_fires: Vec<i64>,
    candidate: Vec<String>,
    x_fires: Vec<i64>,
    p_trace: Vec<i64>,
    x_trace: Vec<i64>,
    return_source: Vec<i64>,
    return_outlet: Vec<i64>,
    return_trace: Vec<i64>,
    attribution: Vec<i64>,
    echo: Vec<String>,
    historical_candidates: usize,
    live_candidates: usize,
    candidate_resistance: Vec<u32>,
    candidate_liveness: Vec<bool>,
    return_updates: u64,
    proposals: u64,
    work: u64,
    bytes: usize,
    fingerprint: u64,
    permanent: u64,
    quiescent: bool,
    validity: [bool; 5],
    claim_pass: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    stratum: &'static str,
    scenario: Kind,
    namespace: u64,
    observation: Observation,
    replay: bool,
    valid: bool,
}

fn main() {
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [argument] if argument == "--preflight" => {
            audit();
            surface();
            absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
            println!("PX3_R5_THREE_FACTOR_RETURN_ATTRIBUTION_PREFLIGHT_OK");
        }
        [argument] if argument == "--r5" => {
            audit();
            surface();
            absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
            evidence();
        }
        _ => std::process::exit(2),
    }
}

fn evidence() {
    eprintln!("PX3_R5_THREE_FACTOR_RETURN_ATTRIBUTION_EVIDENCE_SPENT");
    let mut rows = Vec::new();
    for seed in SEEDS {
        for kind in Kind::ALL {
            rows.push(replay(seed, kind));
        }
    }
    assert_eq!(rows.len(), 16);
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(MD_STAGE, MD, &report(&rows));
}

fn audit() {
    for (path, expected) in [
        ("crates/px0-physical-correspondence/src/lib.rs", PX0),
        (
            "results/px3_d1_r3_downstream_participation_attribution_v1.csv",
            R3_CSV,
        ),
        (
            "experiments/px3_d1_r3_downstream_participation_attribution_result_audit_v1.md",
            R3_AUDIT,
        ),
        (
            "arms/px3-r4-return-window-separability/src/main.rs",
            R4_SOURCE,
        ),
        ("results/px3_r4_return_window_separability_v1.csv", R4_CSV),
        ("results/px3_r4_return_window_separability_v1.md", R4_REPORT),
        (
            "experiments/px3_r4_return_window_separability_result_audit_v1.md",
            R4_AUDIT,
        ),
        (
            "experiments/px3_r5_three_factor_return_attribution_protocol_v1.md",
            PROTOCOL,
        ),
    ] {
        assert_eq!(sha(path), expected, "frozen input changed: {path}");
    }
}

fn surface() {
    assert_eq!(Kind::ALL.len(), 8);
    assert_eq!(Kind::ALL.into_iter().collect::<BTreeSet<_>>().len(), 8);
    assert_eq!(SEEDS.len() * Kind::ALL.len(), 16);
    for forbidden in [
        "arms/px3-r5-three-factor-return-attribution/src/provenance.rs",
        "results/px3_physical_event_organization_definitive_positive.csv",
    ] {
        assert!(!Path::new(forbidden).exists());
    }
}

fn replay(seed: u64, kind: Kind) -> Row {
    let first = run(seed, kind);
    let second = run(seed, kind);
    let exact = first == second;
    let mut row = first;
    row.replay = exact;
    row.observation.validity[4] &= exact;
    row.valid = row.observation.validity.into_iter().all(|value| value);
    row
}

fn run(seed: u64, kind: Kind) -> Row {
    let mirror = seed == 3609;
    let namespace = (seed << 32) | ((kind.index() + 1) << 16);
    let mut world = build(namespace, mirror);
    schedule(&mut world, kind);
    let execution = world.substrate.propagate();
    let candidates = candidate_arrows(&world);
    let candidate_resistance = candidates
        .iter()
        .map(|arrow| world.substrate.arrow_resistance(*arrow))
        .collect::<Vec<_>>();
    let candidate_liveness = candidates
        .iter()
        .map(|arrow| world.substrate.arrow_is_live(*arrow))
        .collect::<Vec<_>>();
    let live_candidates = candidate_liveness.iter().filter(|value| **value).count();

    let primitive = two(|side| firing_ticks(&execution, namespace, 10 + side as u64));
    let primitive_trace = two(|side| firing_ticks(&execution, namespace, 30 + side as u64));
    let opportunity = firing_ticks(&execution, namespace, 100);
    let p_events = events(&execution, namespace, 200);
    let p_fires = firing_ticks(&execution, namespace, 200);
    let candidate = crossings(&execution, namespace, 200, 300);
    let x_fires = firing_ticks(&execution, namespace, 300);
    let p_trace = firing_ticks(&execution, namespace, 400);
    let x_trace = firing_ticks(&execution, namespace, 600);
    let return_source = firing_ticks(&execution, namespace, 910);
    let return_outlet = firing_ticks(&execution, namespace, 920);
    let return_trace = firing_ticks(&execution, namespace, 930);
    let attribution = firing_ticks(&execution, namespace, 800);
    let echo = crossings(&execution, namespace, 800, 200);

    let v0 = SEEDS.contains(&seed)
        && namespace == (seed << 32) | ((kind.index() + 1) << 16)
        && ((seed == 3601 && !mirror) || (seed == 3609 && mirror));
    let v1 = scheduled_participation_valid(
        kind,
        &primitive,
        &primitive_trace,
        &p_fires,
        &x_fires,
        &return_source,
        &return_outlet,
        &return_trace,
    );
    let v2 = attribution
        .iter()
        .all(|tick| echo.iter().any(|value| value == &format!("{tick}:1")))
        && echo.len() == attribution.len()
        && candidate.len() == p_fires.len();
    let expected_history = usize::from(kind != Kind::XrPAbsent);
    let v3 = candidates.len() == expected_history
        && execution.work.local_structural_proposals == expected_history as u64
        && world.substrate.arrow_count() == 27 + expected_history
        && crossing_impulse(&execution, namespace, 800, 200) == attribution.len() as i32;
    let v4 = execution.naturally_quiescent;

    let mut observation = Observation {
        primitive,
        primitive_trace,
        opportunity,
        p_events,
        p_fires,
        candidate,
        x_fires,
        p_trace,
        x_trace,
        return_source,
        return_outlet,
        return_trace,
        attribution,
        echo,
        historical_candidates: candidates.len(),
        live_candidates,
        candidate_resistance,
        candidate_liveness,
        return_updates: execution.work.local_return_updates,
        proposals: execution.work.local_structural_proposals,
        work: execution.work.total(),
        bytes: world.substrate.persistent_bytes(),
        fingerprint: execution.end_fingerprint,
        permanent: execution.permanent_fingerprint,
        quiescent: execution.naturally_quiescent,
        validity: [v0, v1, v2, v3, v4],
        claim_pass: false,
    };
    observation.claim_pass = claim_pass(kind, &observation);
    Row {
        seed,
        stratum: if mirror { "mirrored" } else { "normal" },
        scenario: kind,
        namespace,
        observation,
        replay: false,
        valid: false,
    }
}

fn schedule(world: &mut World, kind: Kind) {
    match kind {
        Kind::Complete => {
            episode(world, 0, true);
            returned(world, 2);
        }
        Kind::PxNoReturn => episode(world, 0, true),
        Kind::PrXBlocked => {
            episode(world, 0, false);
            returned(world, 2);
        }
        Kind::XrPAbsent => {
            pulse(&mut world.substrate, world.effect_driver, 2, 1, 700);
            returned(world, 2);
        }
        Kind::PxLateA => {
            episode(world, 0, true);
            primitive(world, 0, 3, 703);
        }
        Kind::AdjacentNoReturn => {
            episode(world, 0, true);
            episode(world, 1, true);
        }
        Kind::Collision => {
            episode(world, 0, true);
            returned(world, 2);
            episode(world, 3, true);
        }
        Kind::TwoCompleted => {
            episode(world, 0, true);
            returned(world, 2);
            episode(world, 11, true);
            returned(world, 13);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn scheduled_participation_valid(
    kind: Kind,
    primitive: &[Vec<i64>; 2],
    primitive_trace: &[Vec<i64>; 2],
    p: &[i64],
    x: &[i64],
    r_source: &[i64],
    r_outlet: &[i64],
    r_trace: &[i64],
) -> bool {
    match kind {
        Kind::Complete => factor_ticks(
            primitive,
            primitive_trace,
            p,
            x,
            r_source,
            r_outlet,
            r_trace,
            &[0],
            &[1],
            &[1],
            &[2],
            &[2],
            &[2],
            &[3],
        ),
        Kind::PxNoReturn => factor_ticks(
            primitive,
            primitive_trace,
            p,
            x,
            r_source,
            r_outlet,
            r_trace,
            &[0],
            &[1],
            &[1],
            &[2],
            &[],
            &[],
            &[],
        ),
        Kind::PrXBlocked => factor_ticks(
            primitive,
            primitive_trace,
            p,
            x,
            r_source,
            r_outlet,
            r_trace,
            &[0],
            &[1],
            &[1],
            &[],
            &[2],
            &[2],
            &[3],
        ),
        Kind::XrPAbsent => factor_ticks(
            primitive,
            primitive_trace,
            p,
            x,
            r_source,
            r_outlet,
            r_trace,
            &[],
            &[],
            &[],
            &[2],
            &[2],
            &[2],
            &[3],
        ),
        Kind::PxLateA => {
            primitive[0] == [0, 3]
                && primitive[1] == [0]
                && primitive_trace[0] == [1, 4]
                && primitive_trace[1] == [1]
                && p == [1]
                && x == [2]
                && r_source.is_empty()
                && r_outlet.is_empty()
                && r_trace.is_empty()
        }
        Kind::AdjacentNoReturn => factor_ticks(
            primitive,
            primitive_trace,
            p,
            x,
            r_source,
            r_outlet,
            r_trace,
            &[0, 1],
            &[1, 2],
            &[1, 2],
            &[2, 3],
            &[],
            &[],
            &[],
        ),
        Kind::Collision => factor_ticks(
            primitive,
            primitive_trace,
            p,
            x,
            r_source,
            r_outlet,
            r_trace,
            &[0, 3],
            &[1, 4],
            &[1, 4],
            &[2, 5],
            &[2],
            &[2],
            &[3],
        ),
        Kind::TwoCompleted => factor_ticks(
            primitive,
            primitive_trace,
            p,
            x,
            r_source,
            r_outlet,
            r_trace,
            &[0, 11],
            &[1, 12],
            &[1, 12],
            &[2, 13],
            &[2, 13],
            &[2, 13],
            &[3, 14],
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn factor_ticks(
    primitive: &[Vec<i64>; 2],
    primitive_trace: &[Vec<i64>; 2],
    p: &[i64],
    x: &[i64],
    r_source: &[i64],
    r_outlet: &[i64],
    r_trace: &[i64],
    expected_primitive: &[i64],
    expected_primitive_trace: &[i64],
    expected_p: &[i64],
    expected_x: &[i64],
    expected_r_source: &[i64],
    expected_r_outlet: &[i64],
    expected_r_trace: &[i64],
) -> bool {
    primitive[0] == expected_primitive
        && primitive[1] == expected_primitive
        && primitive_trace[0] == expected_primitive_trace
        && primitive_trace[1] == expected_primitive_trace
        && p == expected_p
        && x == expected_x
        && r_source == expected_r_source
        && r_outlet == expected_r_outlet
        && r_trace == expected_r_trace
}

fn claim_pass(kind: Kind, observation: &Observation) -> bool {
    match kind {
        Kind::Complete => {
            observation.attribution == [3]
                && observation.echo == ["3:1"]
                && observation.candidate_resistance == [4]
                && observation.p_fires == [1]
        }
        Kind::PxNoReturn | Kind::PrXBlocked | Kind::XrPAbsent | Kind::PxLateA => {
            observation.attribution.is_empty() && observation.echo.is_empty()
        }
        Kind::AdjacentNoReturn => {
            observation.return_trace.is_empty()
                && observation.attribution.is_empty()
                && observation.echo.is_empty()
        }
        Kind::Collision => {
            observation.return_trace == [3]
                && observation.attribution == [3]
                && observation.echo == ["3:1"]
                && observation.p_fires == [1, 4]
        }
        Kind::TwoCompleted => {
            observation.return_trace == [3, 14]
                && observation.attribution == [3, 14]
                && observation.echo == ["3:1", "14:1"]
                && observation.p_fires == [1, 12]
        }
    }
}

fn build(namespace: u64, mirror: bool) -> World {
    let mut substrate = PlasticSubstrate::new();
    let order = if mirror { [1, 0] } else { [0, 1] };
    let mut primitive_sources = [None; 2];
    let mut primitive_outlets = [None; 2];
    let mut primitive_traces = [None; 2];
    let mut primitive_hubs = [None; 2];
    for side in order {
        primitive_sources[side] = Some(substrate.add_cell(cell(
            physical(namespace, 10 + side as u64),
            -100_000 - side as i32 * 1_000,
            10 + side as i16,
            1,
        )));
        primitive_outlets[side] = Some(substrate.add_cell(cell(
            physical(namespace, 20 + side as u64),
            -90_000 - side as i32 * 1_000,
            20 + side as i16,
            1,
        )));
        primitive_traces[side] = Some(substrate.add_cell(cell(
            physical(namespace, 30 + side as u64),
            -80_000 - side as i32 * 1_000,
            30 + side as i16,
            2,
        )));
        primitive_hubs[side] = Some(substrate.add_cell(cell(
            physical(namespace, 40 + side as u64),
            -70_000 - side as i32 * 1_000,
            40 + side as i16,
            1,
        )));
    }
    let primitive_sources = primitive_sources.map(|value| value.expect("primitive source"));
    let primitive_outlets = primitive_outlets.map(|value| value.expect("primitive outlet"));
    let primitive_traces = primitive_traces.map(|value| value.expect("primitive trace"));
    let primitive_hubs = primitive_hubs.map(|value| value.expect("primitive hub"));

    let opportunity = substrate.add_cell(cell(physical(namespace, 100), -10_000, 50, 2));
    let p_position = 10_000;
    let p = substrate.add_cell(cell(physical(namespace, 200), p_position, 60, 2));
    let x = substrate.add_cell(cell(
        physical(namespace, 300),
        p_position + if mirror { -1 } else { 1 },
        70,
        2,
    ));
    let p_trace = substrate.add_cell(cell(physical(namespace, 400), 30_000, 80, 2));
    let p_hub = substrate.add_cell(cell(physical(namespace, 500), 40_000, 90, 1));
    let x_trace = substrate.add_cell(cell(physical(namespace, 600), 50_000, 100, 2));
    let x_hub = substrate.add_cell(cell(physical(namespace, 700), 60_000, 110, 1));
    let attribution = substrate.add_cell(cell(physical(namespace, 800), 70_000, 120, 3));
    let context = substrate.add_cell(cell(physical(namespace, 900), 80_000, 130, 1));
    let effect_driver = substrate.add_cell(cell(physical(namespace, 901), 90_000, 131, 1));
    let return_source = substrate.add_cell(cell(physical(namespace, 910), 100_000, 140, 1));
    let return_outlet = substrate.add_cell(cell(physical(namespace, 920), 110_000, 141, 1));
    let return_trace = substrate.add_cell(cell(physical(namespace, 930), 120_000, 142, 2));
    let return_hub = substrate.add_cell(cell(physical(namespace, 940), 130_000, 143, 1));

    for side in order {
        substrate.add_arrow(fixed(
            primitive_sources[side],
            primitive_outlets[side],
            0,
            1,
        ));
        normalize(
            &mut substrate,
            primitive_outlets[side],
            primitive_traces[side],
            primitive_hubs[side],
        );
    }
    normalize(&mut substrate, p, p_trace, p_hub);
    normalize(&mut substrate, x, x_trace, x_hub);
    substrate.add_arrow(fixed(return_source, return_outlet, 0, 1));
    normalize(&mut substrate, return_outlet, return_trace, return_hub);
    substrate.add_arrow(fixed(primitive_traces[0], opportunity, 0, 1));
    substrate.add_arrow(fixed(primitive_traces[1], opportunity, 0, 1));
    substrate.add_arrow(fixed(opportunity, p, 0, 1));
    substrate.add_arrow(fixed(context, x, 1, 1));
    substrate.add_arrow(fixed(effect_driver, x, 0, 2));
    substrate.add_arrow(fixed(p_trace, attribution, 1, 1));
    substrate.add_arrow(fixed(x_trace, attribution, 0, 1));
    substrate.add_arrow(fixed(return_trace, attribution, 0, 1));
    substrate.add_arrow(fixed(attribution, p, 1, 1));

    World {
        substrate,
        primitive_sources,
        p,
        x,
        context,
        effect_driver,
        return_source,
    }
}

fn normalize(substrate: &mut PlasticSubstrate, outlet: CellId, trace: CellId, hub: CellId) {
    substrate.add_arrow(fixed(outlet, trace, 1, 1));
    substrate.add_arrow(fixed(outlet, hub, 1, 1));
    substrate.add_arrow(fixed(hub, trace, 0, 1));
}

fn episode(world: &mut World, start: i64, with_context: bool) {
    primitive(world, 0, start, 0);
    primitive(world, 1, start, 1);
    pulse(&mut world.substrate, world.p, start + 1, 1, 100);
    if with_context {
        pulse(&mut world.substrate, world.context, start + 1, 1, 500);
    }
}

fn primitive(world: &mut World, side: usize, tick: i64, phase: i32) {
    pulse(
        &mut world.substrate,
        world.primitive_sources[side],
        tick,
        1,
        phase,
    );
}

fn returned(world: &mut World, tick: i64) {
    pulse(&mut world.substrate, world.return_source, tick, 1, 600);
}

fn candidate_arrows(world: &World) -> Vec<ArrowId> {
    world.substrate.arrows_between(world.p, world.x)
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

fn fixed(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 100,
    }
}

fn pulse(substrate: &mut PlasticSubstrate, target: CellId, tick: i64, impulse: i32, phase: i32) {
    substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: 900_000 + phase as u64,
        target,
        impulse,
    });
}

fn physical(namespace: u64, suffix: u64) -> u64 {
    namespace + suffix
}

fn firing_ticks(execution: &Execution, namespace: u64, suffix: u64) -> Vec<i64> {
    let target = physical(namespace, suffix);
    execution
        .trace
        .iter()
        .filter(|entry| entry.target_physical == target && entry.fired)
        .map(|entry| entry.tick)
        .collect()
}

fn events(execution: &Execution, namespace: u64, suffix: u64) -> Vec<String> {
    let target = physical(namespace, suffix);
    execution
        .trace
        .iter()
        .filter(|entry| entry.target_physical == target)
        .map(|entry| format!("{}:{}:{}", entry.tick, entry.impulse, entry.fired))
        .collect()
}

fn crossings(execution: &Execution, namespace: u64, from: u64, to: u64) -> Vec<String> {
    let from = physical(namespace, from);
    let to = physical(namespace, to);
    execution
        .crossings
        .iter()
        .filter(|entry| entry.from_physical == from && entry.to_physical == to)
        .map(|entry| format!("{}:{}", entry.tick, entry.impulse))
        .collect()
}

fn crossing_impulse(execution: &Execution, namespace: u64, from: u64, to: u64) -> i32 {
    let from = physical(namespace, from);
    let to = physical(namespace, to);
    execution
        .crossings
        .iter()
        .filter(|entry| entry.from_physical == from && entry.to_physical == to)
        .map(|entry| entry.impulse)
        .sum()
}

fn two<T>(mut function: impl FnMut(usize) -> T) -> [T; 2] {
    [function(0), function(1)]
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from("seed,stratum,scenario,namespace,primitive,primitive_trace,opportunity,p_events,p_fires,candidate,x_fires,p_trace,x_trace,return_source,return_outlet,return_trace,attribution,echo,historical_candidates,live_candidates,candidate_resistance,candidate_liveness,return_updates,proposals,v0,v1,v2,v3,v4,validity,claim_pass,work,bytes,fingerprint,permanent,quiescent,replay,valid\n");
    for row in rows {
        let value = &row.observation;
        let fields = vec![
            row.seed.to_string(),
            row.stratum.into(),
            row.scenario.name().into(),
            row.namespace.to_string(),
            join_nested(&value.primitive),
            join_nested(&value.primitive_trace),
            join_i64(&value.opportunity),
            join_string(&value.p_events),
            join_i64(&value.p_fires),
            join_string(&value.candidate),
            join_i64(&value.x_fires),
            join_i64(&value.p_trace),
            join_i64(&value.x_trace),
            join_i64(&value.return_source),
            join_i64(&value.return_outlet),
            join_i64(&value.return_trace),
            join_i64(&value.attribution),
            join_string(&value.echo),
            value.historical_candidates.to_string(),
            value.live_candidates.to_string(),
            value
                .candidate_resistance
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join("|"),
            value
                .candidate_liveness
                .iter()
                .map(bool::to_string)
                .collect::<Vec<_>>()
                .join("|"),
            value.return_updates.to_string(),
            value.proposals.to_string(),
            value.validity[0].to_string(),
            value.validity[1].to_string(),
            value.validity[2].to_string(),
            value.validity[3].to_string(),
            value.validity[4].to_string(),
            value
                .validity
                .into_iter()
                .filter(|item| *item)
                .count()
                .to_string(),
            value.claim_pass.to_string(),
            value.work.to_string(),
            value.bytes.to_string(),
            value.fingerprint.to_string(),
            value.permanent.to_string(),
            value.quiescent.to_string(),
            row.replay.to_string(),
            row.valid.to_string(),
        ];
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

fn report(rows: &[Row]) -> String {
    let all_valid = rows.iter().all(|row| row.valid);
    let core_pass = rows
        .iter()
        .filter(|row| row.scenario != Kind::AdjacentNoReturn)
        .all(|row| row.observation.claim_pass);
    let adjacent_clean = rows
        .iter()
        .filter(|row| row.scenario == Kind::AdjacentNoReturn)
        .all(|row| row.observation.claim_pass);
    let classification = if all_valid && core_pass && adjacent_clean {
        "R5-A THREE-FACTOR POSITIVE"
    } else if all_valid && core_pass && !adjacent_clean {
        "R5-B CARRYOVER NEGATIVE"
    } else {
        "R5-C CORE NEGATIVE"
    };
    let adjacent_m = rows
        .iter()
        .filter(|row| row.scenario == Kind::AdjacentNoReturn)
        .map(|row| row.observation.attribution.len())
        .sum::<usize>();
    let adjacent_r = rows
        .iter()
        .filter(|row| row.scenario == Kind::AdjacentNoReturn)
        .map(|row| row.observation.return_trace.len())
        .sum::<usize>();
    format!("# PX3-R5 three-factor physical return attribution v1\n\nOutcome: **{classification}**.\n\n- valid rows: `{}/{}`;\n- validity clauses: `{}/80`;\n- non-adjacent core controls passed: `{core_pass}`;\n- adjacent recurrence/no-return clean: `{adjacent_clean}`;\n- adjacent recurrence R traces: `{adjacent_r}`;\n- adjacent recurrence M firings: `{adjacent_m}`;\n- exact replay: `{}`;\n- naturally quiescent: `{}`;\n- PX0 law changed: `false`;\n- semantic return/provenance representation added: `false`;\n- PX3 authority after R5: `negative`.\n", rows.iter().filter(|row| row.valid).count(), rows.len(), rows.iter().map(|row| row.observation.validity.into_iter().filter(|item| *item).count()).sum::<usize>(), rows.iter().all(|row| row.replay), rows.iter().all(|row| row.observation.quiescent))
}

fn join_nested<const N: usize>(values: &[Vec<i64>; N]) -> String {
    values
        .iter()
        .map(|value| join_i64(value))
        .collect::<Vec<_>>()
        .join("~")
}

fn join_i64(values: &[i64]) -> String {
    values
        .iter()
        .map(i64::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_string(values: &[String]) -> String {
    values.join("|")
}

fn absent(paths: &[&str]) {
    for path in paths {
        assert!(!Path::new(path).exists(), "artifact exists: {path}");
    }
}

fn publish(stage: &str, destination: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(stage)
        .expect("create staging artifact");
    file.write_all(content.as_bytes()).expect("write artifact");
    file.sync_all().expect("sync artifact");
    rename(stage, destination).expect("publish artifact");
}

fn sha(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum");
    assert!(output.status.success(), "sha256sum failed: {path}");
    String::from_utf8(output.stdout)
        .expect("utf8")
        .split_whitespace()
        .next()
        .expect("digest")
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_is_frozen() {
        surface();
        assert_eq!(SEEDS.len() * Kind::ALL.len(), 16);
    }

    #[test]
    fn adjacent_control_is_decisive() {
        assert!(Kind::ALL.contains(&Kind::AdjacentNoReturn));
        assert_ne!(Kind::AdjacentNoReturn, Kind::Complete);
    }
}
