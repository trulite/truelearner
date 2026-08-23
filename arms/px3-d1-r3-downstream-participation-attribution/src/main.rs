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
const D1: &str = "511efc76fc36c9c3c77815f1bd53bd4f8ea38411f8892bbc26005af1e7d0fecb";
const D1_AUDIT: &str = "06135ca0e63cb9dd944f172e6c072db2ae77b6c1eb1653aabba254f9247d13de";
const R2_AUDIT: &str = "dcdcaf31fad49d9d559b4ec93cadf728326343cf1d491a9bfa09ccb3e06232e8";
const PROTOCOL: &str = "fd1d3442a699ef4a1d3c62ff92ad85937b16e6d80f315ae9c94ce44b504b96f7";
const EXECUTION_PROTOCOL: &str =
    "8282b318cf23b34553b7199c6dab136a154b48b42052f93c05704d8cb32dc9af";
const SEEDS: [u64; 2] = [3201, 3209];
const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
const CSV: &str = "results/px3_d1_r3_downstream_participation_attribution_v1.csv";
const MD: &str = "results/px3_d1_r3_downstream_participation_attribution_v1.md";
const CSV_STAGE: &str = "results/.px3_d1_r3_downstream_participation_attribution_v1.csv.staging";
const MD_STAGE: &str = "results/.px3_d1_r3_downstream_participation_attribution_v1.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    ReturnOnly,
    RealSubthreshold,
    BlockedLateA,
    BlockedEffect,
    IndependentEffect,
    NoReturn,
    LateReturn,
    TwoCompleted,
    Suprathreshold,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scenario {
    name: &'static str,
    kind: Kind,
}

impl Scenario {
    const ALL: [Self; 9] = [
        Self::new("return-only", Kind::ReturnOnly),
        Self::new("ab-real-subthreshold", Kind::RealSubthreshold),
        Self::new("ab-blocked-late-a", Kind::BlockedLateA),
        Self::new("ab-blocked-effect", Kind::BlockedEffect),
        Self::new("no-ab-independent-effect", Kind::IndependentEffect),
        Self::new("ab-real-no-return", Kind::NoReturn),
        Self::new("ab-real-late-return", Kind::LateReturn),
        Self::new("ab-then-cd-two-completed", Kind::TwoCompleted),
        Self::new("ab-suprathreshold-return-control", Kind::Suprathreshold),
    ];

    const fn new(name: &'static str, kind: Kind) -> Self {
        Self { name, kind }
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    sources: [CellId; 4],
    context: CellId,
    effect_driver: CellId,
    global_return: CellId,
    candidates: [ArrowId; 6],
    connectors: [ArrowId; 6],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    scenario: &'static str,
    mirror: bool,
    attribution_coupling: i32,
    source: [usize; 4],
    primitive_trace: [usize; 4],
    opportunity: [usize; 6],
    candidate_source: [usize; 6],
    candidate_crossings: [usize; 6],
    candidate_impulse: [i32; 6],
    effect: [usize; 6],
    p_trace: [usize; 6],
    effect_trace: [usize; 6],
    p_trace_to_attribution: [usize; 6],
    effect_trace_to_attribution: [usize; 6],
    global_to_attribution: [usize; 6],
    attribution: [usize; 6],
    credit_crossings: [usize; 6],
    credit_impulse: [i32; 6],
    candidate_resistance: [u32; 6],
    connector_resistance: [u32; 6],
    after_pressure: [u32; 6],
    return_updates: u64,
    proposals: u64,
    work: u64,
    bytes: usize,
    fingerprint: u64,
    permanent: u64,
    quiescent: bool,
    replay: bool,
    passed: bool,
}

fn main() {
    audit();
    surface();
    absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [argument] if argument == "--preflight" => {
            println!("PX3_D1_R3_DOWNSTREAM_PARTICIPATION_ATTRIBUTION_PREFLIGHT_OK");
        }
        [argument] if argument == "--r3" => evidence(),
        _ => std::process::exit(2),
    }
}

fn evidence() {
    eprintln!("PX3_D1_R3_DOWNSTREAM_PARTICIPATION_ATTRIBUTION_EVIDENCE");
    let mut rows = Vec::new();
    for seed in SEEDS {
        for scenario in Scenario::ALL {
            rows.push(replay(seed, scenario));
        }
    }
    assert_eq!(rows.len(), 18);
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(MD_STAGE, MD, &report(&rows));
}

fn audit() {
    for (path, expected) in [
        ("crates/px0-physical-correspondence/src/lib.rs", PX0),
        (
            "results/px3_d1_participation_gated_pair_learning_v1.csv",
            D1,
        ),
        (
            "experiments/px3_d1_participation_gated_pair_learning_result_audit_v1.md",
            D1_AUDIT,
        ),
        (
            "experiments/px3_d1_r2_closed_loop_return_attribution_result_audit_v1.md",
            R2_AUDIT,
        ),
        (
            "experiments/px3_d1_r3_downstream_participation_attribution_protocol_v1.md",
            PROTOCOL,
        ),
        (
            "experiments/px3_d1_r3_downstream_participation_attribution_execution_protocol_v1.md",
            EXECUTION_PROTOCOL,
        ),
    ] {
        assert_eq!(sha(path), expected, "frozen input changed: {path}");
    }
}

fn surface() {
    assert_eq!(
        Scenario::ALL
            .iter()
            .map(|scenario| scenario.name)
            .collect::<BTreeSet<_>>()
            .len(),
        9
    );
    assert_eq!(PAIRS.into_iter().collect::<BTreeSet<_>>().len(), 6);
    for forbidden in [
        "arms/px3-d1-r3-downstream-participation-attribution/src/d2.rs",
        "results/px3_gate_v1.csv",
    ] {
        assert!(!Path::new(forbidden).exists());
    }
}

fn replay(seed: u64, scenario: Scenario) -> Row {
    let first = run(seed, scenario);
    let second = run(seed, scenario);
    let exact = first == second;
    let mut row = first;
    row.replay = exact;
    row.passed &= exact;
    row
}

fn run(seed: u64, scenario: Scenario) -> Row {
    let mirror = seed == 3209;
    let namespace = (seed << 32) | ((scenario_index(scenario) as u64 + 1) << 16);
    let attribution_coupling = if scenario.kind == Kind::Suprathreshold {
        2
    } else {
        1
    };
    let mut world = build(namespace, mirror, attribution_coupling);
    schedule(&mut world, scenario.kind);
    let execution = world.substrate.propagate();

    let source = four(|index| fires(&execution, physical(namespace, 10 + index as u64)));
    let primitive_trace = four(|index| fires(&execution, physical(namespace, 30 + index as u64)));
    let opportunity = six(|index| fires(&execution, physical(namespace, 100 + index as u64)));
    let candidate_source = six(|index| fires(&execution, physical(namespace, 200 + index as u64)));
    let candidate_crossings = six(|index| {
        crosses(
            &execution,
            physical(namespace, 200 + index as u64),
            physical(namespace, 300 + index as u64),
        )
    });
    let candidate_impulse = six(|index| {
        crossing_impulse(
            &execution,
            physical(namespace, 200 + index as u64),
            physical(namespace, 300 + index as u64),
        )
    });
    let effect = six(|index| fires(&execution, physical(namespace, 300 + index as u64)));
    let p_trace = six(|index| fires(&execution, physical(namespace, 400 + index as u64)));
    let effect_trace = six(|index| fires(&execution, physical(namespace, 600 + index as u64)));
    let p_trace_to_attribution = six(|index| {
        crosses(
            &execution,
            physical(namespace, 400 + index as u64),
            physical(namespace, 800 + index as u64),
        )
    });
    let effect_trace_to_attribution = six(|index| {
        crosses(
            &execution,
            physical(namespace, 600 + index as u64),
            physical(namespace, 800 + index as u64),
        )
    });
    let global_to_attribution = six(|index| {
        crosses(
            &execution,
            physical(namespace, 902),
            physical(namespace, 800 + index as u64),
        )
    });
    let attribution = six(|index| fires(&execution, physical(namespace, 800 + index as u64)));
    let credit_crossings = six(|index| {
        crosses(
            &execution,
            physical(namespace, 800 + index as u64),
            physical(namespace, 200 + index as u64),
        )
    });
    let credit_impulse = six(|index| {
        crossing_impulse(
            &execution,
            physical(namespace, 800 + index as u64),
            physical(namespace, 200 + index as u64),
        )
    });
    let candidate_resistance = resistance(&world, world.candidates);
    let connector_resistance = resistance(&world, world.connectors);
    let mut pressured = world.clone();
    pressured.substrate.advance_time(50);
    let after_pressure = resistance(&pressured, pressured.candidates);

    let mut row = Row {
        seed,
        scenario: scenario.name,
        mirror,
        attribution_coupling,
        source,
        primitive_trace,
        opportunity,
        candidate_source,
        candidate_crossings,
        candidate_impulse,
        effect,
        p_trace,
        effect_trace,
        p_trace_to_attribution,
        effect_trace_to_attribution,
        global_to_attribution,
        attribution,
        credit_crossings,
        credit_impulse,
        candidate_resistance,
        connector_resistance,
        after_pressure,
        return_updates: execution.work.local_return_updates,
        proposals: execution.work.local_structural_proposals,
        work: execution.work.total(),
        bytes: world.substrate.persistent_bytes(),
        fingerprint: execution.end_fingerprint,
        permanent: execution.permanent_fingerprint,
        quiescent: execution.naturally_quiescent,
        replay: false,
        passed: false,
    };
    row.passed = passes(&row, scenario.kind);
    row
}

fn schedule(world: &mut World, kind: Kind) {
    match kind {
        Kind::ReturnOnly => pulse_global(world, 6, 90),
        Kind::RealSubthreshold | Kind::Suprathreshold => {
            pulse_pair(world, [0, 1], 2);
            pulse_context(world, 3, 91);
            pulse_global(world, 6, 92);
        }
        Kind::BlockedLateA => {
            pulse_pair(world, [0, 1], 2);
            pulse_source(world, 0, 4, 93);
            pulse_global(world, 6, 92);
        }
        Kind::BlockedEffect => {
            pulse_pair(world, [0, 1], 2);
            pulse_global(world, 6, 92);
        }
        Kind::IndependentEffect => {
            pulse_driver(world, 5, 94);
            pulse_global(world, 6, 92);
        }
        Kind::NoReturn => {
            pulse_pair(world, [0, 1], 2);
            pulse_context(world, 3, 91);
        }
        Kind::LateReturn => {
            pulse_pair(world, [0, 1], 2);
            pulse_context(world, 3, 91);
            pulse_global(world, 9, 92);
        }
        Kind::TwoCompleted => {
            pulse_pair(world, [0, 1], 0);
            pulse_context(world, 1, 91);
            pulse_global(world, 4, 92);
            pulse_pair(world, [2, 3], 4);
            pulse_context(world, 5, 95);
            pulse_global(world, 8, 96);
        }
    }
}

fn passes(row: &Row, kind: Kind) -> bool {
    let z4 = [0; 4];
    let z6 = [0; 6];
    let z6i = [0_i32; 6];
    let one6 = [1; 6];
    let z6r = [0_u32; 6];
    let one6r = [1_u32; 6];
    let hundred6 = [100_u32; 6];
    let ab4 = [4_u32, 1, 1, 1, 1, 1];
    let ab = [1, 0, 0, 0, 0, 0];
    let (
        source,
        primitive,
        opportunity,
        p,
        candidate,
        candidate_impulse,
        effect,
        p_trace,
        effect_trace,
        p_to_m,
        effect_to_m,
        global_to_m,
        attribution,
        credit,
        credit_impulse,
        candidate_r,
        connector_r,
    ) = match kind {
        Kind::ReturnOnly => (
            z4, z4, z6, z6, z6, z6i, z6, z6, z6, z6, z6, one6, z6, z6, z6i, one6r, hundred6,
        ),
        Kind::RealSubthreshold => (
            [1, 1, 0, 0],
            [1, 1, 0, 0],
            ab,
            ab,
            ab,
            [1, 0, 0, 0, 0, 0],
            ab,
            ab,
            ab,
            ab,
            ab,
            one6,
            ab,
            ab,
            [1, 0, 0, 0, 0, 0],
            ab4,
            hundred6,
        ),
        Kind::BlockedLateA => (
            [2, 1, 0, 0],
            [2, 1, 0, 0],
            ab,
            ab,
            ab,
            [1, 0, 0, 0, 0, 0],
            z6,
            ab,
            z6,
            ab,
            z6,
            one6,
            z6,
            z6,
            z6i,
            one6r,
            [103, 100, 100, 100, 100, 100],
        ),
        Kind::BlockedEffect => (
            [1, 1, 0, 0],
            [1, 1, 0, 0],
            ab,
            ab,
            ab,
            [1, 0, 0, 0, 0, 0],
            z6,
            ab,
            z6,
            ab,
            z6,
            one6,
            z6,
            z6,
            z6i,
            one6r,
            hundred6,
        ),
        Kind::IndependentEffect => (
            z4, z4, z6, z6, z6, z6i, one6, z6, one6, z6, one6, one6, z6, z6, z6i, one6r, hundred6,
        ),
        Kind::NoReturn => (
            [1, 1, 0, 0],
            [1, 1, 0, 0],
            ab,
            ab,
            ab,
            [1, 0, 0, 0, 0, 0],
            ab,
            ab,
            ab,
            ab,
            ab,
            z6,
            z6,
            z6,
            z6i,
            one6r,
            hundred6,
        ),
        Kind::LateReturn => (
            [1, 1, 0, 0],
            [1, 1, 0, 0],
            ab,
            ab,
            ab,
            [1, 0, 0, 0, 0, 0],
            ab,
            ab,
            ab,
            ab,
            ab,
            one6,
            z6,
            z6,
            z6i,
            [0_u32, 1, 1, 1, 1, 1],
            hundred6,
        ),
        Kind::TwoCompleted => (
            [1; 4],
            [1; 4],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [2; 6],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [4_u32, 1, 1, 1, 1, 4],
            hundred6,
        ),
        Kind::Suprathreshold => (
            [1, 1, 0, 0],
            [1, 1, 0, 0],
            ab,
            [2, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0],
            [3, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0],
            one6,
            ab,
            ab,
            [2, 0, 0, 0, 0, 0],
            [3_u32, 1, 1, 1, 1, 1],
            hundred6,
        ),
    };

    row.source == source
        && row.primitive_trace == primitive
        && row.opportunity == opportunity
        && row.candidate_source == p
        && row.candidate_crossings == candidate
        && row.candidate_impulse == candidate_impulse
        && row.effect == effect
        && row.p_trace == p_trace
        && row.effect_trace == effect_trace
        && row.p_trace_to_attribution == p_to_m
        && row.effect_trace_to_attribution == effect_to_m
        && row.global_to_attribution == global_to_m
        && row.attribution == attribution
        && row.credit_crossings == credit
        && row.credit_impulse == credit_impulse
        && row.candidate_resistance == candidate_r
        && row.connector_resistance == connector_r
        && row.after_pressure == z6r
        && row.proposals == 0
        && row.quiescent
}

fn build(namespace: u64, mirror: bool, attribution_coupling: i32) -> World {
    let mut substrate = PlasticSubstrate::new();
    let participant_order = if mirror { [3, 2, 1, 0] } else { [0, 1, 2, 3] };
    let pair_order = if mirror {
        [5, 4, 3, 2, 1, 0]
    } else {
        [0, 1, 2, 3, 4, 5]
    };

    let mut sources = [None; 4];
    let mut outlets = [None; 4];
    let mut traces = [None; 4];
    let mut hubs = [None; 4];
    for index in participant_order {
        sources[index] = Some(substrate.add_cell(cell(
            physical(namespace, 10 + index as u64),
            -40_000 - index as i32 * 100,
            10 + index as i16,
            1,
        )));
        outlets[index] = Some(substrate.add_cell(cell(
            physical(namespace, 20 + index as u64),
            -30_000 - index as i32 * 100,
            20 + index as i16,
            1,
        )));
        traces[index] = Some(substrate.add_cell(cell(
            physical(namespace, 30 + index as u64),
            -20_000 - index as i32 * 100,
            30 + index as i16,
            2,
        )));
        hubs[index] = Some(substrate.add_cell(cell(
            physical(namespace, 40 + index as u64),
            -10_000 - index as i32 * 100,
            40 + index as i16,
            1,
        )));
    }
    let sources = sources.map(|cell| cell.expect("source"));
    let outlets = outlets.map(|cell| cell.expect("outlet"));
    let traces = traces.map(|cell| cell.expect("trace"));
    let hubs = hubs.map(|cell| cell.expect("hub"));

    let mut opportunities = [None; 6];
    let mut candidate_sources = [None; 6];
    let mut effects = [None; 6];
    let mut p_traces = [None; 6];
    let mut p_hubs = [None; 6];
    let mut effect_traces = [None; 6];
    let mut effect_hubs = [None; 6];
    let mut attributions = [None; 6];
    for index in pair_order {
        opportunities[index] = Some(substrate.add_cell(cell(
            physical(namespace, 100 + index as u64),
            10_000 + index as i32 * 100,
            50 + index as i16,
            2,
        )));
        candidate_sources[index] = Some(substrate.add_cell(cell(
            physical(namespace, 200 + index as u64),
            20_000 + index as i32 * 100,
            60 + index as i16,
            2,
        )));
        effects[index] = Some(substrate.add_cell(cell(
            physical(namespace, 300 + index as u64),
            30_000 + index as i32 * 100,
            70 + index as i16,
            2,
        )));
        p_traces[index] = Some(substrate.add_cell(cell(
            physical(namespace, 400 + index as u64),
            40_000 + index as i32 * 100,
            80 + index as i16,
            2,
        )));
        p_hubs[index] = Some(substrate.add_cell(cell(
            physical(namespace, 500 + index as u64),
            50_000 + index as i32 * 100,
            90 + index as i16,
            1,
        )));
        effect_traces[index] = Some(substrate.add_cell(cell(
            physical(namespace, 600 + index as u64),
            60_000 + index as i32 * 100,
            100 + index as i16,
            2,
        )));
        effect_hubs[index] = Some(substrate.add_cell(cell(
            physical(namespace, 700 + index as u64),
            70_000 + index as i32 * 100,
            110 + index as i16,
            1,
        )));
        attributions[index] = Some(substrate.add_cell(cell(
            physical(namespace, 800 + index as u64),
            80_000 + index as i32 * 100,
            120 + index as i16,
            3,
        )));
    }
    let opportunities = opportunities.map(|cell| cell.expect("opportunity"));
    let candidate_sources = candidate_sources.map(|cell| cell.expect("candidate source"));
    let effects = effects.map(|cell| cell.expect("effect"));
    let p_traces = p_traces.map(|cell| cell.expect("P trace"));
    let p_hubs = p_hubs.map(|cell| cell.expect("P hub"));
    let effect_traces = effect_traces.map(|cell| cell.expect("effect trace"));
    let effect_hubs = effect_hubs.map(|cell| cell.expect("effect hub"));
    let attributions = attributions.map(|cell| cell.expect("attribution"));

    let context = substrate.add_cell(cell(physical(namespace, 900), 90_000, 130, 1));
    let effect_driver = substrate.add_cell(cell(physical(namespace, 901), 91_000, 131, 1));
    let global_return = substrate.add_cell(cell(physical(namespace, 902), 92_000, 132, 1));

    for index in participant_order {
        substrate.add_arrow(fixed(sources[index], outlets[index], 0, 1));
        substrate.add_arrow(fixed(outlets[index], traces[index], 1, 1));
        substrate.add_arrow(fixed(outlets[index], hubs[index], 1, 1));
        substrate.add_arrow(fixed(hubs[index], traces[index], 0, 1));
    }

    let mut candidates = [None; 6];
    let mut connectors = [None; 6];
    for index in pair_order {
        let (left, right) = PAIRS[index];
        substrate.add_arrow(fixed(traces[left], opportunities[index], 0, 1));
        substrate.add_arrow(fixed(traces[right], opportunities[index], 0, 1));
        connectors[index] =
            Some(substrate.add_arrow(fixed(opportunities[index], candidate_sources[index], 0, 2)));
        candidates[index] =
            Some(substrate.add_arrow(weak(candidate_sources[index], effects[index])));
        substrate.add_arrow(fixed(context, effects[index], 2, 1));
        substrate.add_arrow(fixed(effect_driver, effects[index], 0, 2));

        substrate.add_arrow(fixed(candidate_sources[index], p_traces[index], 1, 1));
        substrate.add_arrow(fixed(candidate_sources[index], p_hubs[index], 1, 1));
        substrate.add_arrow(fixed(p_hubs[index], p_traces[index], 0, 1));
        substrate.add_arrow(fixed(p_traces[index], attributions[index], 2, 1));

        substrate.add_arrow(fixed(effects[index], effect_traces[index], 1, 1));
        substrate.add_arrow(fixed(effects[index], effect_hubs[index], 1, 1));
        substrate.add_arrow(fixed(effect_hubs[index], effect_traces[index], 0, 1));
        substrate.add_arrow(fixed(effect_traces[index], attributions[index], 0, 1));

        substrate.add_arrow(fixed(global_return, attributions[index], 0, 1));
        let coupling = if index == 0 { attribution_coupling } else { 1 };
        substrate.add_arrow(fixed(
            attributions[index],
            candidate_sources[index],
            1,
            coupling,
        ));
    }

    World {
        substrate,
        sources,
        context,
        effect_driver,
        global_return,
        candidates: candidates.map(|arrow| arrow.expect("candidate")),
        connectors: connectors.map(|arrow| arrow.expect("connector")),
    }
}

fn pulse_pair(world: &mut World, sides: [usize; 2], tick: i64) {
    for side in sides {
        pulse_source(world, side, tick, side as i32);
    }
}

fn pulse_source(world: &mut World, side: usize, tick: i64, phase: i32) {
    pulse(&mut world.substrate, world.sources[side], tick, 1, phase);
}

fn pulse_context(world: &mut World, tick: i64, phase: i32) {
    pulse(&mut world.substrate, world.context, tick, 1, phase);
}

fn pulse_driver(world: &mut World, tick: i64, phase: i32) {
    pulse(&mut world.substrate, world.effect_driver, tick, 1, phase);
}

fn pulse_global(world: &mut World, tick: i64, phase: i32) {
    pulse(&mut world.substrate, world.global_return, tick, 1, phase);
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

fn weak(from: CellId, to: CellId) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay: 2,
        phase: 0,
        coupling: 1,
        resistance: 1,
    }
}

fn resistance(world: &World, arrows: [ArrowId; 6]) -> [u32; 6] {
    six(|index| world.substrate.arrow_resistance(arrows[index]))
}

fn physical(namespace: u64, suffix: u64) -> u64 {
    namespace + suffix
}

fn scenario_index(scenario: Scenario) -> usize {
    Scenario::ALL
        .iter()
        .position(|candidate| *candidate == scenario)
        .expect("scenario")
}

fn four<T>(mut function: impl FnMut(usize) -> T) -> [T; 4] {
    [function(0), function(1), function(2), function(3)]
}

fn six<T>(mut function: impl FnMut(usize) -> T) -> [T; 6] {
    [
        function(0),
        function(1),
        function(2),
        function(3),
        function(4),
        function(5),
    ]
}

fn fires(execution: &Execution, target: u64) -> usize {
    execution
        .trace
        .iter()
        .filter(|entry| entry.target_physical == target && entry.fired)
        .count()
}

fn crosses(execution: &Execution, from: u64, to: u64) -> usize {
    execution
        .crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .count()
}

fn crossing_impulse(execution: &Execution, from: u64, to: u64) -> i32 {
    execution
        .crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .map(|crossing| crossing.impulse)
        .sum()
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "seed,scenario,mirror,attribution_coupling,source,primitive_trace,opportunity,candidate_source,candidate_crossings,candidate_impulse,effect,p_trace,effect_trace,p_trace_to_attribution,effect_trace_to_attribution,global_to_attribution,attribution,credit_crossings,credit_impulse,candidate_resistance,connector_resistance,after_pressure,return_updates,proposals,work,bytes,fingerprint,permanent,quiescent,replay,passed\n",
    );
    for row in rows {
        let fields = vec![
            row.seed.to_string(),
            row.scenario.into(),
            row.mirror.to_string(),
            row.attribution_coupling.to_string(),
            join_usize(&row.source),
            join_usize(&row.primitive_trace),
            join_usize(&row.opportunity),
            join_usize(&row.candidate_source),
            join_usize(&row.candidate_crossings),
            join_i32(&row.candidate_impulse),
            join_usize(&row.effect),
            join_usize(&row.p_trace),
            join_usize(&row.effect_trace),
            join_usize(&row.p_trace_to_attribution),
            join_usize(&row.effect_trace_to_attribution),
            join_usize(&row.global_to_attribution),
            join_usize(&row.attribution),
            join_usize(&row.credit_crossings),
            join_i32(&row.credit_impulse),
            join_u32(&row.candidate_resistance),
            join_u32(&row.connector_resistance),
            join_u32(&row.after_pressure),
            row.return_updates.to_string(),
            row.proposals.to_string(),
            row.work.to_string(),
            row.bytes.to_string(),
            row.fingerprint.to_string(),
            row.permanent.to_string(),
            row.quiescent.to_string(),
            row.replay.to_string(),
            row.passed.to_string(),
        ];
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

fn report(rows: &[Row]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let unit_refirings = rows
        .iter()
        .filter(|row| row.scenario == "ab-real-subthreshold")
        .map(|row| row.candidate_source[0].saturating_sub(1))
        .sum::<usize>();
    let high_refirings = rows
        .iter()
        .filter(|row| row.scenario == "ab-suprathreshold-return-control")
        .map(|row| row.candidate_source[0].saturating_sub(1))
        .sum::<usize>();
    let late_a_credits = rows
        .iter()
        .filter(|row| row.scenario == "ab-blocked-late-a")
        .map(|row| row.credit_crossings[0])
        .sum::<usize>();
    format!(
        "# PX3-D1-R3 downstream participation attribution v1\n\nOutcome: **{}**.\n\n- rows: `{passed}/{}` passed;\n- exact replay: `{}`;\n- naturally quiescent: `{}`;\n- unit-attribution P refirings: `{unit_refirings}`;\n- suprathreshold-control P refirings: `{high_refirings}`;\n- blocked-effect late-A credit pulses: `{late_a_credits}`;\n- structural proposals: `{}`;\n- candidate formation/persistence/D2/MICRO/GATE executed: `false`.\n",
        if passed == rows.len() {
            "R3-A POSITIVE"
        } else {
            "NEGATIVE"
        },
        rows.len(),
        rows.iter().all(|row| row.replay),
        rows.iter().all(|row| row.quiescent),
        rows.iter().map(|row| row.proposals).sum::<u64>(),
    )
}

fn join_usize(values: &[usize]) -> String {
    values
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_i32(values: &[i32]) -> String {
    values
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_u32(values: &[u32]) -> String {
    values
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join("|")
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
        assert_eq!(SEEDS.len() * Scenario::ALL.len(), 18);
    }

    #[test]
    fn safety_margin_is_explicit() {
        assert_eq!(Scenario::ALL[1].kind, Kind::RealSubthreshold);
        assert_eq!(Scenario::ALL[8].kind, Kind::Suprathreshold);
    }
}
