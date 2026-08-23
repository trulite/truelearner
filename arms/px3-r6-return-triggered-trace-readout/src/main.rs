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
const R5_SOURCE: &str = "13ad86ab078e5fb72bdcbd0b5bff87f85f4cca0a493cda33d22f8dac647ac4fe";
const R5_CSV: &str = "947257da74420ca4d8a1dc4f49402ddd63bb7ae6d1fe758362c6779365bc73cf";
const R5_REPORT: &str = "c88dcec19cd18de0723ec927dcaec0829efff1b5c7c4f867b598161b9d61acd1";
const R5_AUDIT: &str = "49870f078da9b28d222b456ebea54bb72babf017dfdbde76be4e590bf105cbfe";
const PROTOCOL: &str = "25a30a95c629d3e67665ee70f7f42024f6afff5996e4cc82b83e1511460ea6ce";
const SEEDS: [u64; 2] = [3701, 3709];
const CSV: &str = "results/px3_r6_return_triggered_trace_readout_v1.csv";
const MD: &str = "results/px3_r6_return_triggered_trace_readout_v1.md";
const CSV_STAGE: &str = "results/.px3_r6_return_triggered_trace_readout_v1.csv.staging";
const MD_STAGE: &str = "results/.px3_r6_return_triggered_trace_readout_v1.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    RepeatedNoReturn,
    Complete,
    POnlyReturn,
    XOnlyReturn,
    LateReturn,
    AdjacentCurrentReturn,
}

impl Kind {
    const ALL: [Self; 6] = [
        Self::RepeatedNoReturn,
        Self::Complete,
        Self::POnlyReturn,
        Self::XOnlyReturn,
        Self::LateReturn,
        Self::AdjacentCurrentReturn,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::RepeatedNoReturn => "px-no-return-100-adjacent",
            Self::Complete => "complete-pxr",
            Self::POnlyReturn => "p-only-r",
            Self::XOnlyReturn => "x-only-r",
            Self::LateReturn => "px-late-r",
            Self::AdjacentCurrentReturn => "adjacent-current-r",
        }
    }

    fn index(self) -> u64 {
        Self::ALL.iter().position(|value| *value == self).expect("kind") as u64
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    primitive_sources: [CellId; 2],
    p: CellId,
    x: CellId,
    context: CellId,
    driver: CellId,
    return_source: CellId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    primitive: [Vec<i64>; 2],
    primitive_trace: [Vec<i64>; 2],
    p_fires: Vec<i64>,
    x_fires: Vec<i64>,
    p_trace: Vec<i64>,
    x_trace: Vec<i64>,
    return_trace: Vec<i64>,
    p_foot_events: Vec<String>,
    x_foot_events: Vec<String>,
    p_foot_fires: Vec<i64>,
    x_foot_fires: Vec<i64>,
    m_events: Vec<String>,
    m_fires: Vec<i64>,
    echo: Vec<String>,
    candidate: Vec<String>,
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
    kind: Kind,
    namespace: u64,
    observation: Observation,
    replay: bool,
    passed: bool,
}

fn main() {
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [argument] if argument == "--preflight" => {
            audit();
            surface();
            absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
            println!("PX3_R6_RETURN_TRIGGERED_TRACE_READOUT_PREFLIGHT_OK");
        }
        [argument] if argument == "--r6" => {
            audit();
            surface();
            absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
            evidence();
        }
        _ => std::process::exit(2),
    }
}

fn evidence() {
    eprintln!("PX3_R6_RETURN_TRIGGERED_TRACE_READOUT_EVIDENCE_SPENT");
    let mut rows = Vec::new();
    for seed in SEEDS {
        for kind in Kind::ALL {
            rows.push(replay(seed, kind));
        }
    }
    assert_eq!(rows.len(), 12);
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(MD_STAGE, MD, &report(&rows));
}

fn audit() {
    for (path, expected) in [
        ("crates/px0-physical-correspondence/src/lib.rs", PX0),
        ("arms/px3-r5-three-factor-return-attribution/src/main.rs", R5_SOURCE),
        ("results/px3_r5_three_factor_return_attribution_v1.csv", R5_CSV),
        ("results/px3_r5_three_factor_return_attribution_v1.md", R5_REPORT),
        ("experiments/px3_r5_three_factor_return_attribution_result_audit_v1.md", R5_AUDIT),
        ("experiments/px3_r6_return_triggered_trace_readout_protocol_v1.md", PROTOCOL),
    ] {
        assert_eq!(sha(path), expected, "frozen input changed: {path}");
    }
}

fn surface() {
    assert_eq!(Kind::ALL.into_iter().collect::<BTreeSet<_>>().len(), 6);
    assert_eq!(SEEDS.len() * Kind::ALL.len(), 12);
    for forbidden in [
        "arms/px3-r6-return-triggered-trace-readout/src/gated_arrow.rs",
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
    row.passed = row.observation.validity.into_iter().all(|value| value)
        && row.observation.claim_pass;
    row
}

fn run(seed: u64, kind: Kind) -> Row {
    let mirror = seed == 3709;
    let namespace = (seed << 32) | ((kind.index() + 1) << 16);
    let mut world = build(namespace, mirror);
    schedule(&mut world, kind);
    let execution = world.substrate.propagate();
    let candidates = world.substrate.arrows_between(world.p, world.x);
    let primitive = two(|side| firing_ticks(&execution, namespace, 10 + side as u64));
    let primitive_trace = two(|side| firing_ticks(&execution, namespace, 30 + side as u64));
    let p_fires = firing_ticks(&execution, namespace, 200);
    let x_fires = firing_ticks(&execution, namespace, 300);
    let p_trace = firing_ticks(&execution, namespace, 400);
    let x_trace = firing_ticks(&execution, namespace, 600);
    let return_trace = firing_ticks(&execution, namespace, 930);
    let p_foot_events = events(&execution, namespace, 1000);
    let x_foot_events = events(&execution, namespace, 1001);
    let p_foot_fires = firing_ticks(&execution, namespace, 1000);
    let x_foot_fires = firing_ticks(&execution, namespace, 1001);
    let m_events = events(&execution, namespace, 800);
    let m_fires = firing_ticks(&execution, namespace, 800);
    let echo = crossings(&execution, namespace, 800, 200);
    let candidate = crossings(&execution, namespace, 200, 300);
    let candidate_resistance = candidates.iter().map(|arrow| world.substrate.arrow_resistance(*arrow)).collect::<Vec<_>>();
    let candidate_liveness = candidates.iter().map(|arrow| world.substrate.arrow_is_live(*arrow)).collect::<Vec<_>>();
    let expected_history = usize::from(kind != Kind::XOnlyReturn);
    let v0 = SEEDS.contains(&seed) && ((seed == 3701 && !mirror) || (seed == 3709 && mirror));
    let v1 = schedule_valid(kind, &primitive, &primitive_trace, &p_fires, &x_fires, &p_trace, &x_trace, &return_trace);
    let v2 = m_fires.len() == echo.len() && candidate.len() == p_fires.len();
    let v3 = candidates.len() == expected_history
        && execution.work.local_structural_proposals == expected_history as u64
        && world.substrate.arrow_count() == 30 + expected_history
        && crossing_impulse(&execution, namespace, 800, 200) == m_fires.len() as i32;
    let v4 = execution.naturally_quiescent;
    let mut observation = Observation {
        primitive,
        primitive_trace,
        p_fires,
        x_fires,
        p_trace,
        x_trace,
        return_trace,
        p_foot_events,
        x_foot_events,
        p_foot_fires,
        x_foot_fires,
        m_events,
        m_fires,
        echo,
        candidate,
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
    observation.claim_pass = claim(kind, &observation);
    Row {
        seed,
        stratum: if mirror { "mirrored" } else { "normal" },
        kind,
        namespace,
        observation,
        replay: false,
        passed: false,
    }
}

fn schedule(world: &mut World, kind: Kind) {
    match kind {
        Kind::RepeatedNoReturn => {
            for start in 0..100 {
                episode(world, start, true);
            }
        }
        Kind::Complete => {
            episode(world, 0, true);
            returned(world, 2);
        }
        Kind::POnlyReturn => {
            episode(world, 0, false);
            returned(world, 2);
        }
        Kind::XOnlyReturn => {
            pulse(&mut world.substrate, world.driver, 2, 1, 700);
            returned(world, 2);
        }
        Kind::LateReturn => {
            episode(world, 0, true);
            returned(world, 6);
        }
        Kind::AdjacentCurrentReturn => {
            episode(world, 0, true);
            episode(world, 1, true);
            returned(world, 3);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn schedule_valid(
    kind: Kind,
    primitive: &[Vec<i64>; 2],
    primitive_trace: &[Vec<i64>; 2],
    p: &[i64],
    x: &[i64],
    p_trace: &[i64],
    x_trace: &[i64],
    r_trace: &[i64],
) -> bool {
    match kind {
        Kind::RepeatedNoReturn => {
            let s = (0..100).collect::<Vec<_>>();
            primitive[0] == s
                && primitive[1] == s
                && primitive_trace[0] == (1..=100).collect::<Vec<_>>()
                && primitive_trace[1] == (1..=100).collect::<Vec<_>>()
                && p == (1..=100).collect::<Vec<_>>()
                && x == (2..=101).collect::<Vec<_>>()
                && p_trace == (2..=101).collect::<Vec<_>>()
                && x_trace == (3..=102).collect::<Vec<_>>()
                && r_trace.is_empty()
        }
        Kind::Complete => factors(primitive, primitive_trace, p, x, p_trace, x_trace, r_trace, &[0], &[1], &[1], &[2], &[2], &[3], &[3]),
        Kind::POnlyReturn => factors(primitive, primitive_trace, p, x, p_trace, x_trace, r_trace, &[0], &[1], &[1], &[], &[2], &[], &[3]),
        Kind::XOnlyReturn => factors(primitive, primitive_trace, p, x, p_trace, x_trace, r_trace, &[], &[], &[], &[2], &[], &[3], &[3]),
        Kind::LateReturn => factors(primitive, primitive_trace, p, x, p_trace, x_trace, r_trace, &[0], &[1], &[1], &[2], &[2], &[3], &[7]),
        Kind::AdjacentCurrentReturn => factors(primitive, primitive_trace, p, x, p_trace, x_trace, r_trace, &[0, 1], &[1, 2], &[1, 2], &[2, 3], &[2, 3], &[3, 4], &[4]),
    }
}

#[allow(clippy::too_many_arguments)]
fn factors(
    primitive: &[Vec<i64>; 2], primitive_trace: &[Vec<i64>; 2], p: &[i64], x: &[i64],
    p_trace: &[i64], x_trace: &[i64], r_trace: &[i64], expected_primitive: &[i64],
    expected_primitive_trace: &[i64], expected_p: &[i64], expected_x: &[i64],
    expected_p_trace: &[i64], expected_x_trace: &[i64], expected_r_trace: &[i64],
) -> bool {
    primitive[0] == expected_primitive && primitive[1] == expected_primitive
        && primitive_trace[0] == expected_primitive_trace && primitive_trace[1] == expected_primitive_trace
        && p == expected_p && x == expected_x && p_trace == expected_p_trace
        && x_trace == expected_x_trace && r_trace == expected_r_trace
}

fn claim(kind: Kind, value: &Observation) -> bool {
    match kind {
        Kind::RepeatedNoReturn => value.p_foot_fires.is_empty()
            && value.x_foot_fires.is_empty()
            && value.m_events.is_empty()
            && value.m_fires.is_empty()
            && value.echo.is_empty(),
        Kind::Complete => value.p_foot_fires == [3]
            && value.x_foot_fires == [3]
            && value.m_events == ["3:1:false", "3:1:true"]
            && value.m_fires == [3]
            && value.echo == ["3:1"]
            && value.p_fires == [1]
            && value.candidate_resistance == [4],
        Kind::POnlyReturn => value.p_foot_fires == [3]
            && value.x_foot_fires.is_empty()
            && value.m_events == ["3:1:false"]
            && value.m_fires.is_empty()
            && value.echo.is_empty(),
        Kind::XOnlyReturn => value.p_foot_fires.is_empty()
            && value.x_foot_fires == [3]
            && value.m_events == ["3:1:false"]
            && value.m_fires.is_empty()
            && value.echo.is_empty(),
        Kind::LateReturn => value.p_foot_fires.is_empty()
            && value.x_foot_fires.is_empty()
            && value.m_events.is_empty()
            && value.m_fires.is_empty()
            && value.echo.is_empty(),
        Kind::AdjacentCurrentReturn => value.p_foot_fires == [4]
            && value.x_foot_fires == [4]
            && value.m_fires == [4]
            && value.echo == ["4:1"]
            && value.p_fires == [1, 2],
    }
}

fn build(namespace: u64, mirror: bool) -> World {
    let mut substrate = PlasticSubstrate::new();
    let order = if mirror { [1, 0] } else { [0, 1] };
    let mut sources = [None; 2];
    let mut outlets = [None; 2];
    let mut traces = [None; 2];
    let mut hubs = [None; 2];
    for side in order {
        sources[side] = Some(substrate.add_cell(cell(physical(namespace, 10 + side as u64), -100_000 - side as i32 * 1_000, 10 + side as i16, 1)));
        outlets[side] = Some(substrate.add_cell(cell(physical(namespace, 20 + side as u64), -90_000 - side as i32 * 1_000, 20 + side as i16, 1)));
        traces[side] = Some(substrate.add_cell(cell(physical(namespace, 30 + side as u64), -80_000 - side as i32 * 1_000, 30 + side as i16, 2)));
        hubs[side] = Some(substrate.add_cell(cell(physical(namespace, 40 + side as u64), -70_000 - side as i32 * 1_000, 40 + side as i16, 1)));
    }
    let sources = sources.map(|value| value.expect("source"));
    let outlets = outlets.map(|value| value.expect("outlet"));
    let traces = traces.map(|value| value.expect("trace"));
    let hubs = hubs.map(|value| value.expect("hub"));
    let opportunity = substrate.add_cell(cell(physical(namespace, 100), -10_000, 50, 2));
    let p = substrate.add_cell(cell(physical(namespace, 200), 10_000, 60, 2));
    let x = substrate.add_cell(cell(physical(namespace, 300), 10_000 + if mirror { -1 } else { 1 }, 70, 2));
    let p_trace = substrate.add_cell(cell(physical(namespace, 400), 30_000, 80, 2));
    let p_hub = substrate.add_cell(cell(physical(namespace, 500), 40_000, 90, 1));
    let x_trace = substrate.add_cell(cell(physical(namespace, 600), 50_000, 100, 2));
    let x_hub = substrate.add_cell(cell(physical(namespace, 700), 60_000, 110, 1));
    let m = substrate.add_cell(cell(physical(namespace, 800), 70_000, 120, 2));
    let context = substrate.add_cell(cell(physical(namespace, 900), 80_000, 130, 1));
    let driver = substrate.add_cell(cell(physical(namespace, 901), 90_000, 131, 1));
    let r_source = substrate.add_cell(cell(physical(namespace, 910), 100_000, 140, 1));
    let r_outlet = substrate.add_cell(cell(physical(namespace, 920), 110_000, 141, 1));
    let r_trace = substrate.add_cell(cell(physical(namespace, 930), 120_000, 142, 2));
    let r_hub = substrate.add_cell(cell(physical(namespace, 940), 130_000, 143, 1));
    let p_foot = substrate.add_cell(cell(physical(namespace, 1000), 140_000, 150, 2));
    let x_foot = substrate.add_cell(cell(physical(namespace, 1001), 150_000, 151, 2));
    for side in order {
        substrate.add_arrow(fixed(sources[side], outlets[side], 0, 1));
        normalize(&mut substrate, outlets[side], traces[side], hubs[side]);
    }
    normalize(&mut substrate, p, p_trace, p_hub);
    normalize(&mut substrate, x, x_trace, x_hub);
    substrate.add_arrow(fixed(r_source, r_outlet, 0, 1));
    normalize(&mut substrate, r_outlet, r_trace, r_hub);
    substrate.add_arrow(fixed(traces[0], opportunity, 0, 1));
    substrate.add_arrow(fixed(traces[1], opportunity, 0, 1));
    substrate.add_arrow(fixed(opportunity, p, 0, 1));
    substrate.add_arrow(fixed(context, x, 1, 1));
    substrate.add_arrow(fixed(driver, x, 0, 2));
    substrate.add_arrow(fixed(p_trace, p_foot, 1, 1));
    substrate.add_arrow(fixed(x_trace, x_foot, 0, 1));
    substrate.add_arrow(fixed(r_trace, p_foot, 0, 1));
    substrate.add_arrow(fixed(r_trace, x_foot, 0, 1));
    substrate.add_arrow(fixed(p_foot, m, 0, 1));
    substrate.add_arrow(fixed(x_foot, m, 0, 1));
    substrate.add_arrow(fixed(m, p, 1, 1));
    World { substrate, primitive_sources: sources, p, x, context, driver, return_source: r_source }
}

fn normalize(substrate: &mut PlasticSubstrate, outlet: CellId, trace: CellId, hub: CellId) {
    substrate.add_arrow(fixed(outlet, trace, 1, 1));
    substrate.add_arrow(fixed(outlet, hub, 1, 1));
    substrate.add_arrow(fixed(hub, trace, 0, 1));
}

fn episode(world: &mut World, start: i64, context: bool) {
    for side in 0..2 {
        pulse(&mut world.substrate, world.primitive_sources[side], start, 1, side as i32);
    }
    pulse(&mut world.substrate, world.p, start + 1, 1, 100);
    if context {
        pulse(&mut world.substrate, world.context, start + 1, 1, 500);
    }
}

fn returned(world: &mut World, tick: i64) {
    pulse(&mut world.substrate, world.return_source, tick, 1, 600);
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec { physical_id, position, region, threshold, resistance: 100 }
}

fn fixed(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec { from, to, delay, phase: 0, coupling, resistance: 100 }
}

fn pulse(substrate: &mut PlasticSubstrate, target: CellId, tick: i64, impulse: i32, phase: i32) {
    substrate.enter(SpikeInput { arrival_tick: tick, phase, origin_physical: 900_000 + phase as u64, target, impulse });
}

fn physical(namespace: u64, suffix: u64) -> u64 { namespace + suffix }

fn firing_ticks(execution: &Execution, namespace: u64, suffix: u64) -> Vec<i64> {
    let target = physical(namespace, suffix);
    execution.trace.iter().filter(|entry| entry.target_physical == target && entry.fired).map(|entry| entry.tick).collect()
}

fn events(execution: &Execution, namespace: u64, suffix: u64) -> Vec<String> {
    let target = physical(namespace, suffix);
    execution.trace.iter().filter(|entry| entry.target_physical == target).map(|entry| format!("{}:{}:{}", entry.tick, entry.impulse, entry.fired)).collect()
}

fn crossings(execution: &Execution, namespace: u64, from: u64, to: u64) -> Vec<String> {
    let from = physical(namespace, from); let to = physical(namespace, to);
    execution.crossings.iter().filter(|entry| entry.from_physical == from && entry.to_physical == to).map(|entry| format!("{}:{}", entry.tick, entry.impulse)).collect()
}

fn crossing_impulse(execution: &Execution, namespace: u64, from: u64, to: u64) -> i32 {
    let from = physical(namespace, from); let to = physical(namespace, to);
    execution.crossings.iter().filter(|entry| entry.from_physical == from && entry.to_physical == to).map(|entry| entry.impulse).sum()
}

fn two<T>(mut function: impl FnMut(usize) -> T) -> [T; 2] { [function(0), function(1)] }

fn csv(rows: &[Row]) -> String {
    let mut out = String::from("seed,stratum,scenario,namespace,primitive,primitive_trace,p_fires,x_fires,p_trace,x_trace,return_trace,p_foot_events,x_foot_events,p_foot_fires,x_foot_fires,m_events,m_fires,echo,candidate,candidate_resistance,candidate_liveness,return_updates,proposals,v0,v1,v2,v3,v4,validity,claim_pass,work,bytes,fingerprint,permanent,quiescent,replay,passed\n");
    for row in rows {
        let v = &row.observation;
        let fields = vec![row.seed.to_string(), row.stratum.into(), row.kind.name().into(), row.namespace.to_string(), join_nested(&v.primitive), join_nested(&v.primitive_trace), join_i64(&v.p_fires), join_i64(&v.x_fires), join_i64(&v.p_trace), join_i64(&v.x_trace), join_i64(&v.return_trace), join_string(&v.p_foot_events), join_string(&v.x_foot_events), join_i64(&v.p_foot_fires), join_i64(&v.x_foot_fires), join_string(&v.m_events), join_i64(&v.m_fires), join_string(&v.echo), join_string(&v.candidate), v.candidate_resistance.iter().map(u32::to_string).collect::<Vec<_>>().join("|"), v.candidate_liveness.iter().map(bool::to_string).collect::<Vec<_>>().join("|"), v.return_updates.to_string(), v.proposals.to_string(), v.validity[0].to_string(), v.validity[1].to_string(), v.validity[2].to_string(), v.validity[3].to_string(), v.validity[4].to_string(), v.validity.into_iter().filter(|item| *item).count().to_string(), v.claim_pass.to_string(), v.work.to_string(), v.bytes.to_string(), v.fingerprint.to_string(), v.permanent.to_string(), v.quiescent.to_string(), row.replay.to_string(), row.passed.to_string()];
        out.push_str(&fields.join(",")); out.push('\n');
    }
    out
}

fn report(rows: &[Row]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let repeated = rows.iter().filter(|row| row.kind == Kind::RepeatedNoReturn);
    let repeated_m_arrivals = repeated.clone().map(|row| row.observation.m_events.len()).sum::<usize>();
    let repeated_m_fires = repeated.map(|row| row.observation.m_fires.len()).sum::<usize>();
    format!("# PX3-R6 return-triggered trace readout v1\n\nOutcome: **{}**.\n\n- rows: `{passed}/{}`;\n- validity clauses: `{}/60`;\n- 100-adjacent/no-return M arrivals: `{repeated_m_arrivals}`;\n- 100-adjacent/no-return M firings: `{repeated_m_fires}`;\n- exact replay: `{}`;\n- naturally quiescent: `{}`;\n- authoritative PX0 changed: `false`;\n- CJ-B gated-ARROW law imported: `false`;\n- PX3 authority after R6: `negative`.\n", if passed == rows.len() { "R6-A POSITIVE" } else { "R6-B NEGATIVE" }, rows.len(), rows.iter().map(|row| row.observation.validity.into_iter().filter(|item| *item).count()).sum::<usize>(), rows.iter().all(|row| row.replay), rows.iter().all(|row| row.observation.quiescent))
}

fn join_nested<const N: usize>(values: &[Vec<i64>; N]) -> String { values.iter().map(|value| join_i64(value)).collect::<Vec<_>>().join("~") }
fn join_i64(values: &[i64]) -> String { values.iter().map(i64::to_string).collect::<Vec<_>>().join("|") }
fn join_string(values: &[String]) -> String { values.join("|") }

fn absent(paths: &[&str]) { for path in paths { assert!(!Path::new(path).exists(), "artifact exists: {path}"); } }

fn publish(stage: &str, destination: &str, content: &str) {
    let mut file = OpenOptions::new().write(true).create_new(true).open(stage).expect("create stage");
    file.write_all(content.as_bytes()).expect("write"); file.sync_all().expect("sync"); rename(stage, destination).expect("publish");
}

fn sha(path: &str) -> String {
    let output = Command::new("sha256sum").arg(path).output().expect("sha256sum");
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("utf8").split_whitespace().next().expect("digest").into()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn matrix() { surface(); assert_eq!(SEEDS.len() * Kind::ALL.len(), 12); }
    #[test] fn repeated_control() { assert!(Kind::ALL.contains(&Kind::RepeatedNoReturn)); }
}
