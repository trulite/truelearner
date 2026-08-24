#![forbid(unsafe_code)]

use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Crossing, Execution, PlasticSubstrate, SpikeInput,
    TraceEntry,
};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX0: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const DEFINITIVE_SOURCE: &str = "288ce23199f66b65e022afac4314629ac133edaea93072486326357f8c58b328";
const DEFINITIVE_CSV: &str = "3fa85616ae97faef0db2200941a41a6ea7d51f9e254267f84fbed8684a0e0d06";
const DEFINITIVE_REPORT: &str = "3a8f96587b751f3e19303582861d8320c0a78ae1aa25da24b8959efb809c3317";
const DEFINITIVE_AUDIT: &str = "37a2bd47474020510d076e131c65acfa60f0ac34ee1b6f583aa42c3d6f6fd3d5";
const DEFINITIVE_HANDOFF: &str = "f6cd3e4436071758697f8cb7805189aca747914ec6a4999f5143f142f02ca336";
const R3_CSV: &str = "62b34a64396728c28b617bab75cf1141ee2b2db53897ee655809b6180cb2a67b";
const R3_AUDIT: &str = "6f565cf8397afb55e28293360f1ade5aa51b89ba5fa8c19ce0eacaa23086e299";
const PROTOCOL: &str = "6be667083059847cbe8a7e1085b021e5b0e6909ecaf43c1d9fa293e4425894ac";
const EXECUTION_PROTOCOL: &str = "ad8a1af9983f3f9d589c9bf552c086a3edd3de9ed1a1f0ee1b2177748bb249d7";

const SEEDS: [u64; 2] = [3501, 3509];
const CSV: &str = "results/px3_r4_return_window_separability_v1.csv";
const MD: &str = "results/px3_r4_return_window_separability_v1.md";
const CSV_STAGE: &str = "results/.px3_r4_return_window_separability_v1.csv.staging";
const MD_STAGE: &str = "results/.px3_r4_return_window_separability_v1.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Sweep {
    Return,
    Recurrence,
    Collision,
}

impl Sweep {
    fn name(self) -> &'static str {
        match self {
            Self::Return => "lawful-return",
            Self::Recurrence => "renewed-input",
            Self::Collision => "same-tick-collision",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scenario {
    sweep: Sweep,
    parameter: i64,
}

impl Scenario {
    const ALL: [Self; 14] = [
        Self::new(Sweep::Return, 0),
        Self::new(Sweep::Return, 1),
        Self::new(Sweep::Return, 2),
        Self::new(Sweep::Return, 3),
        Self::new(Sweep::Return, 4),
        Self::new(Sweep::Return, 5),
        Self::new(Sweep::Return, 6),
        Self::new(Sweep::Recurrence, 1),
        Self::new(Sweep::Recurrence, 2),
        Self::new(Sweep::Recurrence, 3),
        Self::new(Sweep::Recurrence, 4),
        Self::new(Sweep::Recurrence, 5),
        Self::new(Sweep::Recurrence, 6),
        Self::new(Sweep::Collision, 3),
    ];

    const fn new(sweep: Sweep, parameter: i64) -> Self {
        Self { sweep, parameter }
    }

    fn name(self) -> String {
        match self.sweep {
            Sweep::Return => format!("return-tick-{}", self.parameter),
            Sweep::Recurrence => format!("recurrence-offset-{}", self.parameter),
            Sweep::Collision => "collision-return-3-recurrence-3".to_owned(),
        }
    }
}

#[derive(Clone, Copy)]
struct Stage {
    source: CellId,
    output: CellId,
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    primitive_sources: [CellId; 4],
    stages: [Stage; 3],
    context: CellId,
    global_return: CellId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    primitive_fires: [Vec<i64>; 2],
    primitive_trace_fires: [Vec<i64>; 2],
    opportunity_fires: Vec<i64>,
    opportunity_to_source: Vec<String>,
    source_events: Vec<String>,
    source_fires: Vec<i64>,
    candidate_crossings: Vec<String>,
    output_fires: Vec<i64>,
    source_trace_fires: Vec<i64>,
    output_trace_fires: Vec<i64>,
    global_to_attribution: Vec<String>,
    attribution_fires: Vec<i64>,
    echo_crossings: Vec<String>,
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    stratum: &'static str,
    scenario: String,
    sweep: Sweep,
    parameter: i64,
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
            println!("PX3_R4_RETURN_WINDOW_SEPARABILITY_PREFLIGHT_OK");
        }
        [argument] if argument == "--r4" => {
            audit();
            surface();
            absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
            evidence();
        }
        _ => std::process::exit(2),
    }
}

fn evidence() {
    eprintln!("PX3_R4_RETURN_WINDOW_SEPARABILITY_EVIDENCE_SPENT");
    let mut rows = Vec::new();
    for seed in SEEDS {
        for scenario in Scenario::ALL {
            rows.push(replay(seed, scenario));
        }
    }
    assert_eq!(rows.len(), 28);
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(MD_STAGE, MD, &report(&rows));
}

fn audit() {
    for (path, expected) in [
        ("crates/px0-physical-correspondence/src/lib.rs", PX0),
        (
            "arms/px3-physical-event-organization-definitive/src/main.rs",
            DEFINITIVE_SOURCE,
        ),
        (
            "results/px3_physical_event_organization_definitive.csv",
            DEFINITIVE_CSV,
        ),
        (
            "results/px3_physical_event_organization_definitive.md",
            DEFINITIVE_REPORT,
        ),
        (
            "experiments/px3_physical_event_organization_definitive_result_audit_v1.md",
            DEFINITIVE_AUDIT,
        ),
        (
            "experiments/px3_physical_event_organization_definitive_negative_handoff_v1.md",
            DEFINITIVE_HANDOFF,
        ),
        (
            "results/px3_d1_r3_downstream_participation_attribution_v1.csv",
            R3_CSV,
        ),
        (
            "experiments/px3_d1_r3_downstream_participation_attribution_result_audit_v1.md",
            R3_AUDIT,
        ),
        (
            "experiments/px3_r4_return_window_separability_protocol_v1.md",
            PROTOCOL,
        ),
        (
            "experiments/px3_r4_return_window_separability_execution_protocol_v1.md",
            EXECUTION_PROTOCOL,
        ),
    ] {
        assert_eq!(sha(path), expected, "frozen input changed: {path}");
    }
}

fn surface() {
    assert_eq!(Scenario::ALL.len(), 14);
    assert_eq!(SEEDS.len() * Scenario::ALL.len(), 28);
    assert_eq!(
        Scenario::ALL
            .iter()
            .map(|scenario| (scenario.sweep, scenario.parameter))
            .collect::<BTreeSet<_>>()
            .len(),
        14
    );
    assert_eq!(
        Scenario::ALL
            .iter()
            .filter(|scenario| scenario.sweep == Sweep::Return)
            .count(),
        7
    );
    assert_eq!(
        Scenario::ALL
            .iter()
            .filter(|scenario| scenario.sweep == Sweep::Recurrence)
            .count(),
        6
    );
    assert_eq!(
        Scenario::ALL
            .iter()
            .filter(|scenario| scenario.sweep == Sweep::Collision)
            .count(),
        1
    );
    for forbidden in [
        "arms/px3-r4-return-window-separability/src/px3_fix.rs",
        "results/px3_physical_event_organization_definitive_positive.csv",
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
    row.observation.validity[4] &= exact;
    row.passed = row.observation.validity.into_iter().all(|valid| valid);
    row
}

fn run(seed: u64, scenario: Scenario) -> Row {
    let reverse = seed == 3509;
    let reflect = reverse;
    let namespace = namespace(seed, scenario);
    let mut world = build(namespace, reverse, reflect);
    schedule(&mut world, scenario);
    let execution = world.substrate.propagate();
    let candidates = candidate_arrows(&world, 0);

    let primitive_fires =
        two(|side| firing_ticks(&execution.trace, physical(namespace, 10 + side as u64)));
    let primitive_trace_fires =
        two(|side| firing_ticks(&execution.trace, physical(namespace, 30 + side as u64)));
    let opportunity_fires = firing_ticks(&execution.trace, physical(namespace, 100));
    let opportunity_to_source = crossing_signatures(
        &execution.crossings,
        physical(namespace, 100),
        physical(namespace, 200),
    );
    let source_events = event_signatures(&execution.trace, physical(namespace, 200));
    let source_fires = firing_ticks(&execution.trace, physical(namespace, 200));
    let candidate_crossings = crossing_signatures(
        &execution.crossings,
        physical(namespace, 200),
        physical(namespace, 300),
    );
    let output_fires = firing_ticks(&execution.trace, physical(namespace, 300));
    let source_trace_fires = firing_ticks(&execution.trace, physical(namespace, 400));
    let output_trace_fires = firing_ticks(&execution.trace, physical(namespace, 600));
    let global_to_attribution = crossing_signatures(
        &execution.crossings,
        physical(namespace, 901),
        physical(namespace, 800),
    );
    let attribution_fires = firing_ticks(&execution.trace, physical(namespace, 800));
    let echo_crossings = crossing_signatures(
        &execution.crossings,
        physical(namespace, 800),
        physical(namespace, 200),
    );
    let candidate_resistance = candidates
        .iter()
        .map(|arrow| world.substrate.arrow_resistance(*arrow))
        .collect::<Vec<_>>();
    let candidate_liveness = candidates
        .iter()
        .map(|arrow| world.substrate.arrow_is_live(*arrow))
        .collect::<Vec<_>>();
    let live_candidates = candidate_liveness.iter().filter(|live| **live).count();

    let v0 = [3501, 3509].contains(&seed)
        && namespace == crate::namespace(seed, scenario)
        && ((seed == 3501 && !reverse && !reflect) || (seed == 3509 && reverse && reflect));
    let v1 = participation_valid(scenario, &execution, namespace);
    let v2 = timing_valid(scenario, &execution, namespace);
    let v3 = mechanism_valid(&world, &execution, scenario);
    let v4 = execution.naturally_quiescent;
    let observation = Observation {
        primitive_fires,
        primitive_trace_fires,
        opportunity_fires,
        opportunity_to_source,
        source_events,
        source_fires,
        candidate_crossings,
        output_fires,
        source_trace_fires,
        output_trace_fires,
        global_to_attribution,
        attribution_fires,
        echo_crossings,
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
    };
    Row {
        seed,
        stratum: if reverse { "mirrored" } else { "normal" },
        scenario: scenario.name(),
        sweep: scenario.sweep,
        parameter: scenario.parameter,
        namespace,
        observation,
        replay: false,
        passed: false,
    }
}

fn schedule(world: &mut World, scenario: Scenario) {
    episode(world, 0, true);
    match scenario.sweep {
        Sweep::Return => global_return(world, scenario.parameter),
        Sweep::Recurrence => episode(world, scenario.parameter, true),
        Sweep::Collision => {
            global_return(world, 3);
            episode(world, 3, true);
        }
    }
}

fn participation_valid(scenario: Scenario, execution: &Execution, namespace: u64) -> bool {
    let episodes = if scenario.sweep == Sweep::Return {
        1
    } else {
        2
    };
    let expected_ticks = if episodes == 1 {
        vec![0]
    } else {
        vec![0, scenario.parameter]
    };
    let source = firing_ticks(&execution.trace, physical(namespace, 10));
    let source_b = firing_ticks(&execution.trace, physical(namespace, 11));
    let primitive_a = firing_ticks(&execution.trace, physical(namespace, 30));
    let primitive_b = firing_ticks(&execution.trace, physical(namespace, 31));
    let opportunity = firing_ticks(&execution.trace, physical(namespace, 100));
    let p = firing_ticks(&execution.trace, physical(namespace, 200));
    let candidate = crossing_signatures(
        &execution.crossings,
        physical(namespace, 200),
        physical(namespace, 300),
    );
    let output = firing_ticks(&execution.trace, physical(namespace, 300));
    let attribution = firing_ticks(&execution.trace, physical(namespace, 800));

    source == expected_ticks
        && source_b == expected_ticks
        && primitive_a
            == expected_ticks
                .iter()
                .map(|tick| tick + 1)
                .collect::<Vec<_>>()
        && primitive_b
            == expected_ticks
                .iter()
                .map(|tick| tick + 1)
                .collect::<Vec<_>>()
        && opportunity
            == expected_ticks
                .iter()
                .map(|tick| tick + 1)
                .collect::<Vec<_>>()
        && p.len() == episodes
        && candidate.len() == episodes
        && output.len() == episodes
        && match scenario.sweep {
            Sweep::Return => {
                attribution
                    == if scenario.parameter == 3 {
                        vec![3]
                    } else {
                        vec![]
                    }
            }
            Sweep::Recurrence => attribution.is_empty(),
            Sweep::Collision => attribution == vec![3],
        }
}

fn timing_valid(scenario: Scenario, execution: &Execution, namespace: u64) -> bool {
    let o_to_p = crossing_ticks(
        &execution.crossings,
        physical(namespace, 100),
        physical(namespace, 200),
    );
    let echo = crossing_ticks(
        &execution.crossings,
        physical(namespace, 800),
        physical(namespace, 200),
    );
    let p = firing_ticks(&execution.trace, physical(namespace, 200));
    match scenario.sweep {
        Sweep::Return => {
            o_to_p == vec![1]
                && p == vec![1]
                && echo
                    == if scenario.parameter == 3 {
                        vec![3]
                    } else {
                        vec![]
                    }
                && firing_ticks(&execution.trace, physical(namespace, 400)) == vec![2]
                && firing_ticks(&execution.trace, physical(namespace, 600)) == vec![3]
        }
        Sweep::Recurrence => {
            o_to_p == vec![1, scenario.parameter + 1]
                && p == vec![1, scenario.parameter + 1]
                && echo.is_empty()
        }
        Sweep::Collision => {
            o_to_p == vec![1, 4]
                && echo == vec![3]
                && p == vec![1, 4]
                && event_signatures(&execution.trace, physical(namespace, 200))
                    == vec![
                        "1:1:false",
                        "1:1:true",
                        "4:1:false",
                        "4:1:true",
                        "4:1:false",
                    ]
        }
    }
}

fn mechanism_valid(world: &World, execution: &Execution, scenario: Scenario) -> bool {
    let candidates = candidate_arrows(world, 0);
    let expected_history = if scenario.sweep == Sweep::Recurrence && scenario.parameter >= 5 {
        2
    } else {
        1
    };
    candidates.len() == expected_history
        && world.substrate.arrow_count() >= 42
        && execution.work.local_structural_proposals == expected_history as u64
        && crossing_impulse(
            &execution.crossings,
            physical(world.namespace, 800),
            physical(world.namespace, 200),
        ) <= 1
}

fn build(namespace: u64, reverse: bool, reflect: bool) -> World {
    let mut substrate = PlasticSubstrate::new();
    let primitive_order = if reverse { [3, 2, 1, 0] } else { [0, 1, 2, 3] };
    let stage_order = if reverse { [2, 1, 0] } else { [0, 1, 2] };

    let mut primitive_sources = [None; 4];
    let mut primitive_outlets = [None; 4];
    let mut primitive_traces = [None; 4];
    let mut primitive_hubs = [None; 4];
    for side in primitive_order {
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

    let mut opportunities = [None; 3];
    let mut sources = [None; 3];
    let mut outputs = [None; 3];
    let mut source_traces = [None; 3];
    let mut source_hubs = [None; 3];
    let mut output_traces = [None; 3];
    let mut output_hubs = [None; 3];
    let mut attributions = [None; 3];
    for stage in stage_order {
        opportunities[stage] = Some(substrate.add_cell(cell(
            physical(namespace, 100 + stage as u64),
            -10_000 - stage as i32 * 1_000,
            50 + stage as i16,
            2,
        )));
        let p_position = 10_000 + stage as i32 * 1_000;
        sources[stage] = Some(substrate.add_cell(cell(
            physical(namespace, 200 + stage as u64),
            p_position,
            60 + stage as i16,
            2,
        )));
        outputs[stage] = Some(substrate.add_cell(cell(
            physical(namespace, 300 + stage as u64),
            p_position + if reflect { -1 } else { 1 },
            70 + stage as i16,
            2,
        )));
        source_traces[stage] = Some(substrate.add_cell(cell(
            physical(namespace, 400 + stage as u64),
            30_000 + stage as i32 * 1_000,
            80 + stage as i16,
            2,
        )));
        source_hubs[stage] = Some(substrate.add_cell(cell(
            physical(namespace, 500 + stage as u64),
            40_000 + stage as i32 * 1_000,
            90 + stage as i16,
            1,
        )));
        output_traces[stage] = Some(substrate.add_cell(cell(
            physical(namespace, 600 + stage as u64),
            50_000 + stage as i32 * 1_000,
            100 + stage as i16,
            2,
        )));
        output_hubs[stage] = Some(substrate.add_cell(cell(
            physical(namespace, 700 + stage as u64),
            60_000 + stage as i32 * 1_000,
            110 + stage as i16,
            1,
        )));
        attributions[stage] = Some(substrate.add_cell(cell(
            physical(namespace, 800 + stage as u64),
            70_000 + stage as i32 * 1_000,
            120 + stage as i16,
            3,
        )));
    }
    let opportunities = opportunities.map(|value| value.expect("opportunity"));
    let sources = sources.map(|value| value.expect("source"));
    let outputs = outputs.map(|value| value.expect("output"));
    let source_traces = source_traces.map(|value| value.expect("source trace"));
    let source_hubs = source_hubs.map(|value| value.expect("source hub"));
    let output_traces = output_traces.map(|value| value.expect("output trace"));
    let output_hubs = output_hubs.map(|value| value.expect("output hub"));
    let attributions = attributions.map(|value| value.expect("attribution"));
    let context = substrate.add_cell(cell(physical(namespace, 900), 90_000, 130, 1));
    let global_return = substrate.add_cell(cell(physical(namespace, 901), 100_000, 131, 1));

    for side in primitive_order {
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
    for stage in stage_order {
        normalize(
            &mut substrate,
            outputs[stage],
            output_traces[stage],
            output_hubs[stage],
        );
        normalize(
            &mut substrate,
            sources[stage],
            source_traces[stage],
            source_hubs[stage],
        );
    }
    let left_inputs = [primitive_traces[0], output_traces[0], output_traces[1]];
    let right_inputs = [
        primitive_traces[1],
        primitive_traces[2],
        primitive_traces[3],
    ];
    for stage in stage_order {
        substrate.add_arrow(fixed(left_inputs[stage], opportunities[stage], 0, 1));
        substrate.add_arrow(fixed(right_inputs[stage], opportunities[stage], 0, 1));
        substrate.add_arrow(fixed(opportunities[stage], sources[stage], 0, 1));
        substrate.add_arrow(fixed(context, outputs[stage], 1, 1));
        substrate.add_arrow(fixed(source_traces[stage], attributions[stage], 1, 1));
        substrate.add_arrow(fixed(output_traces[stage], attributions[stage], 0, 1));
        substrate.add_arrow(fixed(global_return, attributions[stage], 0, 1));
        substrate.add_arrow(fixed(attributions[stage], sources[stage], 1, 1));
    }
    let stages = three(|stage| Stage {
        source: sources[stage],
        output: outputs[stage],
    });
    World {
        substrate,
        namespace,
        primitive_sources,
        stages,
        context,
        global_return,
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
    background(world, start + 1);
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

fn background(world: &mut World, tick: i64) {
    for stage in 0..3 {
        pulse(
            &mut world.substrate,
            world.stages[stage].source,
            tick,
            1,
            100 + stage as i32,
        );
    }
}

fn global_return(world: &mut World, tick: i64) {
    pulse(&mut world.substrate, world.global_return, tick, 1, 600);
}

fn candidate_arrows(world: &World, stage: usize) -> Vec<ArrowId> {
    world
        .substrate
        .arrows_between(world.stages[stage].source, world.stages[stage].output)
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

fn namespace(seed: u64, scenario: Scenario) -> u64 {
    (seed << 32) | ((scenario_index(scenario) as u64 + 1) << 16)
}

fn scenario_index(scenario: Scenario) -> usize {
    Scenario::ALL
        .iter()
        .position(|value| *value == scenario)
        .expect("scenario")
}

fn physical(namespace: u64, suffix: u64) -> u64 {
    namespace + suffix
}

fn firing_ticks(trace: &[TraceEntry], physical_id: u64) -> Vec<i64> {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id && entry.fired)
        .map(|entry| entry.tick)
        .collect()
}

fn event_signatures(trace: &[TraceEntry], physical_id: u64) -> Vec<String> {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id)
        .map(|entry| format!("{}:{}:{}", entry.tick, entry.impulse, entry.fired))
        .collect()
}

fn crossing_ticks(crossings: &[Crossing], from: u64, to: u64) -> Vec<i64> {
    crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .map(|crossing| crossing.tick)
        .collect()
}

fn crossing_signatures(crossings: &[Crossing], from: u64, to: u64) -> Vec<String> {
    crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .map(|crossing| format!("{}:{}", crossing.tick, crossing.impulse))
        .collect()
}

fn crossing_impulse(crossings: &[Crossing], from: u64, to: u64) -> i32 {
    crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .map(|crossing| crossing.impulse)
        .sum()
}

fn two<T>(mut function: impl FnMut(usize) -> T) -> [T; 2] {
    [function(0), function(1)]
}

fn three<T>(mut function: impl FnMut(usize) -> T) -> [T; 3] {
    [function(0), function(1), function(2)]
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "seed,stratum,scenario,sweep,parameter,namespace,primitive_fires,primitive_trace_fires,opportunity_fires,opportunity_to_source,source_events,source_fires,candidate_crossings,output_fires,source_trace_fires,output_trace_fires,global_to_attribution,attribution_fires,echo_crossings,historical_candidates,live_candidates,candidate_resistance,candidate_liveness,return_updates,proposals,v0,v1,v2,v3,v4,validity,work,bytes,fingerprint,permanent,quiescent,replay,passed\n",
    );
    for row in rows {
        let observation = &row.observation;
        let fields = vec![
            row.seed.to_string(),
            row.stratum.into(),
            row.scenario.clone(),
            row.sweep.name().into(),
            row.parameter.to_string(),
            row.namespace.to_string(),
            join_nested_i64(&observation.primitive_fires),
            join_nested_i64(&observation.primitive_trace_fires),
            join_i64(&observation.opportunity_fires),
            join_string(&observation.opportunity_to_source),
            join_string(&observation.source_events),
            join_i64(&observation.source_fires),
            join_string(&observation.candidate_crossings),
            join_i64(&observation.output_fires),
            join_i64(&observation.source_trace_fires),
            join_i64(&observation.output_trace_fires),
            join_string(&observation.global_to_attribution),
            join_i64(&observation.attribution_fires),
            join_string(&observation.echo_crossings),
            observation.historical_candidates.to_string(),
            observation.live_candidates.to_string(),
            observation
                .candidate_resistance
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join("|"),
            observation
                .candidate_liveness
                .iter()
                .map(bool::to_string)
                .collect::<Vec<_>>()
                .join("|"),
            observation.return_updates.to_string(),
            observation.proposals.to_string(),
            observation.validity[0].to_string(),
            observation.validity[1].to_string(),
            observation.validity[2].to_string(),
            observation.validity[3].to_string(),
            observation.validity[4].to_string(),
            observation
                .validity
                .into_iter()
                .filter(|valid| *valid)
                .count()
                .to_string(),
            observation.work.to_string(),
            observation.bytes.to_string(),
            observation.fingerprint.to_string(),
            observation.permanent.to_string(),
            observation.quiescent.to_string(),
            row.replay.to_string(),
            row.passed.to_string(),
        ];
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

fn report(rows: &[Row]) -> String {
    let valid = rows.iter().all(|row| row.passed);
    let return_arrivals = rows
        .iter()
        .filter(|row| row.sweep == Sweep::Return && !row.observation.echo_crossings.is_empty())
        .flat_map(|row| {
            row.observation
                .echo_crossings
                .iter()
                .filter_map(|value| value.split(':').next()?.parse::<i64>().ok())
                .map(|tick| tick + 1)
        })
        .collect::<Vec<_>>();
    let recurrence_arrivals = rows
        .iter()
        .filter(|row| row.sweep == Sweep::Recurrence)
        .filter_map(|row| row.observation.opportunity_to_source.get(1))
        .filter_map(|value| value.split(':').next()?.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let latest_return = return_arrivals.iter().copied().max();
    let earliest_recurrence = recurrence_arrivals.iter().copied().min();
    let collision = rows
        .iter()
        .filter(|row| row.sweep == Sweep::Collision)
        .all(|row| {
            row.observation
                .opportunity_to_source
                .get(1)
                .is_some_and(|value| value.starts_with("4:"))
                && row
                    .observation
                    .echo_crossings
                    .first()
                    .is_some_and(|value| value.starts_with("3:"))
        });
    let overlap = latest_return
        .is_some_and(|return_tick| recurrence_arrivals.contains(&return_tick))
        && collision;
    let classification = if !valid {
        "R4-C UNINTERPRETABLE"
    } else if overlap {
        "R4-B TEMPORAL OVERLAP"
    } else if latest_return
        .zip(earliest_recurrence)
        .is_some_and(|(return_tick, recurrence_tick)| return_tick < recurrence_tick)
    {
        "R4-A TEMPORALLY SEPARABLE"
    } else {
        "R4-C UNINTERPRETABLE"
    };
    format!(
        "# PX3-R4 return-window separability v1\n\nOutcome: **{classification}**.\n\n- valid rows: `{}/{}`;\n- validity clauses: `{}/140`;\n- exact replay: `{}`;\n- naturally quiescent: `{}`;\n- lawful echo arrival ticks at P: `{}`;\n- latest lawful echo arrival: `{}`;\n- renewed upstream arrival ticks at P: `{}`;\n- earliest renewed upstream arrival: `{}`;\n- exact same-tick collision observed: `{collision}`;\n- PX0 law changed: `false`;\n- PX3 authority retried: `false`;\n- PX3 authority after R4: `negative`.\n",
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
        rows.iter().map(|row| row.observation.validity.into_iter().filter(|valid| *valid).count()).sum::<usize>(),
        rows.iter().all(|row| row.replay),
        rows.iter().all(|row| row.observation.quiescent),
        join_i64(&return_arrivals),
        latest_return.map_or_else(|| "none".to_owned(), |tick| tick.to_string()),
        join_i64(&recurrence_arrivals),
        earliest_recurrence.map_or_else(|| "none".to_owned(), |tick| tick.to_string()),
    )
}

fn join_nested_i64<const N: usize>(values: &[Vec<i64>; N]) -> String {
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
        assert_eq!(SEEDS.len() * Scenario::ALL.len(), 28);
    }

    #[test]
    fn collision_is_explicit() {
        let collision = Scenario::ALL
            .iter()
            .filter(|scenario| scenario.sweep == Sweep::Collision)
            .collect::<Vec<_>>();
        assert_eq!(collision.len(), 1);
        assert_eq!(collision[0].parameter, 3);
    }
}
