#![forbid(unsafe_code)]

use lr1_modulatory_physical_return::{
    ArrowId, ArrowSpec, CellId, CellSpec, Crossing, Execution, PlasticSubstrate, SpikeInput,
    TraceEntry, TransmissionMode, WorkLedger,
};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const LAW: &str = "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10";
const AUTHORITY: &str = "3ad1df774690a71ee7e6884f56a9399a098890e14d83c7a2f03231ed9aafeb3c";
const DEVELOPMENT_PROTOCOL: &str =
    "c249be1d74ad219896f2fe3505942166efb369e9bea4e50dfa84585f2a1d7107";
const DEVELOPMENT_AUDIT: &str = "44ce9f414f785a434752ccaab6a7445e430a43be5cac576d7cdec1441d85fd79";
const LIFECYCLE_DEVELOPMENT: &str =
    "0125f52fc9e5427c558caa39b8f720fe1fefea13e16f8c4e88bd0ae46904afef";
const RECURSION_DEVELOPMENT: &str =
    "b59fa4b299d2ec22429255d78269ecb7fa56c22aeb4c122137fc21a299369724";
const V1_PROTOCOL: &str = "06a9ea4515b5ea42bf576a5bd49969c966cc63bba94ef3f4b499fb89da8345cc";
const V1_RESULT_AUDIT: &str = "668aeb5802194a98002edd95bb55d09100825637c1f3c4a5ca1a711b9e0565a2";
const PROTOCOL: &str = "ef16155950bf84a361ba9804f4455dbd067f81d7aa94e8cc3d917edfcf3807b9";

const CSV: &str = "results/px3_lrc_recursion_definitive_v2.csv";
const MD: &str = "results/px3_lrc_recursion_definitive_v2.md";
const CSV_STAGE: &str = "results/.px3_lrc_recursion_definitive_v2.csv.staging";
const MD_STAGE: &str = "results/.px3_lrc_recursion_definitive_v2.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Config {
    seed: u64,
    reverse: bool,
    reflect: bool,
}

const CONFIGS: [Config; 16] = [
    Config {
        seed: 94003,
        reverse: false,
        reflect: false,
    },
    Config {
        seed: 94007,
        reverse: true,
        reflect: false,
    },
    Config {
        seed: 94011,
        reverse: false,
        reflect: true,
    },
    Config {
        seed: 94015,
        reverse: true,
        reflect: true,
    },
    Config {
        seed: 94021,
        reverse: false,
        reflect: false,
    },
    Config {
        seed: 94025,
        reverse: true,
        reflect: false,
    },
    Config {
        seed: 94029,
        reverse: false,
        reflect: true,
    },
    Config {
        seed: 94033,
        reverse: true,
        reflect: true,
    },
    Config {
        seed: 94039,
        reverse: false,
        reflect: false,
    },
    Config {
        seed: 94043,
        reverse: true,
        reflect: false,
    },
    Config {
        seed: 94047,
        reverse: false,
        reflect: true,
    },
    Config {
        seed: 94051,
        reverse: true,
        reflect: true,
    },
    Config {
        seed: 94059,
        reverse: false,
        reflect: false,
    },
    Config {
        seed: 94063,
        reverse: true,
        reflect: false,
    },
    Config {
        seed: 94067,
        reverse: false,
        reflect: true,
    },
    Config {
        seed: 94071,
        reverse: true,
        reflect: true,
    },
];

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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Log {
    trace: Vec<TraceEntry>,
    crossings: Vec<Crossing>,
    work: WorkLedger,
    quiescent: bool,
}

impl Log {
    fn new() -> Self {
        Self {
            quiescent: true,
            ..Self::default()
        }
    }

    fn execution(&mut self, execution: Execution) {
        self.trace.extend(execution.trace);
        self.crossings.extend(execution.crossings);
        add_work(&mut self.work, &execution.work);
        self.quiescent &= execution.naturally_quiescent;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Metrics {
    primitive_trace: [usize; 4],
    opportunity: [usize; 3],
    source: [usize; 3],
    candidate: [usize; 3],
    candidate_impulse: [i32; 3],
    output: [usize; 3],
    source_trace: [usize; 3],
    output_trace_arrivals: [usize; 3],
    output_trace_impulse: [i32; 3],
    output_trace: [usize; 3],
    attribution: [usize; 3],
    credit: [usize; 3],
    updates: u64,
    proposals: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    namespace: u64,
    reverse: bool,
    reflect: bool,
    initial_candidates: usize,
    phases: [Metrics; 3],
    after_first: [[u32; 3]; 3],
    after_train: [[u32; 3]; 3],
    one_exposure_death: [u32; 3],
    final_resistance: [u32; 3],
    candidate_ids: [String; 3],
    candidate_generations: [u32; 3],
    ab_reuse: Metrics,
    c_alone: Metrics,
    xc_gapped: Metrics,
    xc_reuse: Metrics,
    d_alone: Metrics,
    yd_gapped: Metrics,
    full_reuse: Metrics,
    work: u64,
    bytes: usize,
    fingerprint: u64,
    permanent: u64,
    quiescent: bool,
    claims: [bool; 12],
    replay: bool,
    passed: bool,
}

fn main() {
    audit();
    surface();
    absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [argument] if argument == "--preflight" => {
            println!("PX3_LRC_RECURSION_PREFLIGHT_OK");
        }
        [argument] if argument == "--definitive" => evidence(),
        _ => std::process::exit(2),
    }
}

fn evidence() {
    eprintln!("PX3_LRC_RECURSION_DEFINITIVE_V2_EVIDENCE_SPENT");
    let rows = CONFIGS.into_iter().map(replay).collect::<Vec<_>>();
    assert_eq!(rows.len(), 16);
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(MD_STAGE, MD, &report(&rows));
}

fn audit() {
    for (path, expected) in [
        ("crates/lr1-modulatory-physical-return/src/lib.rs", LAW),
        (
            "results/lrc_qualified_modulatory_transmission_definitive_v2.md",
            AUTHORITY,
        ),
        (
            "experiments/px3_lrc_fresh_integrated_parallel_gates_protocol_v2.md",
            DEVELOPMENT_PROTOCOL,
        ),
        (
            "experiments/px3_lrc_fresh_integrated_parallel_gates_v2_result_audit.md",
            DEVELOPMENT_AUDIT,
        ),
        ("results/px3_lrc_lifecycle_v2.csv", LIFECYCLE_DEVELOPMENT),
        ("results/px3_lrc_recursion_v2.csv", RECURSION_DEVELOPMENT),
        (
            "experiments/px3_lrc_physical_event_organization_definitive_protocol_v1.md",
            V1_PROTOCOL,
        ),
        (
            "experiments/px3_lrc_physical_event_organization_definitive_result_audit_v1.md",
            V1_RESULT_AUDIT,
        ),
        (
            "experiments/px3_lrc_physical_event_organization_definitive_protocol_v2.md",
            PROTOCOL,
        ),
    ] {
        assert_eq!(sha(path), expected, "frozen input changed: {path}");
    }
}

fn surface() {
    assert_eq!(CONFIGS.into_iter().collect::<BTreeSet<_>>().len(), 16);
    assert_eq!(CONFIGS.iter().filter(|config| config.reverse).count(), 8);
    assert_eq!(CONFIGS.iter().filter(|config| config.reflect).count(), 8);
    assert_eq!(
        CONFIGS
            .iter()
            .filter(|config| !config.reverse && !config.reflect)
            .count(),
        4
    );
    assert_eq!(
        CONFIGS
            .iter()
            .filter(|config| config.reverse && !config.reflect)
            .count(),
        4
    );
    assert_eq!(
        CONFIGS
            .iter()
            .filter(|config| !config.reverse && config.reflect)
            .count(),
        4
    );
    assert_eq!(
        CONFIGS
            .iter()
            .filter(|config| config.reverse && config.reflect)
            .count(),
        4
    );
    for forbidden in [
        "arms/px3-recursive-compression-gate/src/definitive.rs",
        "arms/px3-recursive-compression-gate/src/px4.rs",
        "results/px3_recursive_compression_definitive_v1.csv",
    ] {
        assert!(!Path::new(forbidden).exists());
    }
}

fn replay(config: Config) -> Row {
    let first = run(config);
    let second = run(config);
    let exact = first == second;
    let mut row = first;
    row.replay = exact;
    row.claims[11] &= exact;
    row.passed = row.claims.into_iter().all(|claim| claim);
    row
}

fn run(config: Config) -> Row {
    let namespace = config.seed << 32;
    let mut world = build(namespace, config.reverse, config.reflect);
    let initial_candidates = candidate_count(&world);
    let mut continuing_work = WorkLedger::default();

    let mut phase_ab = Log::new();
    expose(&mut world, 1, 0, true, true, &mut phase_ab);
    let after_first_ab = resistances(&world);
    let mut once_ab = world.clone();
    add_work(&mut continuing_work, &once_ab.substrate.advance_time(50));
    let dead_ab = resistance(&once_ab, 0);
    expose(&mut world, 1, 11, true, true, &mut phase_ab);
    let after_train_ab = resistances(&world);
    let phase_ab_metrics = metrics(&phase_ab, namespace);
    add_work(&mut continuing_work, &phase_ab.work);

    let ab_reuse = on_clone(&world, |clone, log| {
        expose(clone, 1, 16, false, false, log);
    });
    let c_alone = on_clone(&world, |clone, log| {
        primitive(clone, 2, 16, 2);
        background(clone, 17);
        log.execution(clone.substrate.propagate());
    });
    let xc_gapped = on_clone(&world, |clone, log| {
        primitive(clone, 0, 16, 0);
        primitive(clone, 1, 16, 1);
        background(clone, 17);
        primitive(clone, 2, 20, 2);
        background(clone, 21);
        log.execution(clone.substrate.propagate());
    });

    let mut phase_xc = Log::new();
    expose(&mut world, 2, 20, true, true, &mut phase_xc);
    let after_first_xc = resistances(&world);
    let mut once_xc = world.clone();
    add_work(&mut continuing_work, &once_xc.substrate.advance_time(70));
    let dead_xc = resistance(&once_xc, 1);
    expose(&mut world, 2, 31, true, true, &mut phase_xc);
    let after_train_xc = resistances(&world);
    let phase_xc_metrics = metrics(&phase_xc, namespace);
    add_work(&mut continuing_work, &phase_xc.work);

    let xc_reuse = on_clone(&world, |clone, log| {
        expose(clone, 2, 38, false, false, log);
    });
    let d_alone = on_clone(&world, |clone, log| {
        primitive(clone, 3, 38, 3);
        background(clone, 39);
        log.execution(clone.substrate.propagate());
    });
    let yd_gapped = on_clone(&world, |clone, log| {
        expose(clone, 2, 38, false, false, log);
        primitive(clone, 3, 46, 3);
        background(clone, 47);
        log.execution(clone.substrate.propagate());
    });

    let mut phase_yd = Log::new();
    expose(&mut world, 3, 40, true, true, &mut phase_yd);
    let after_first_yd = resistances(&world);
    let mut once_yd = world.clone();
    add_work(&mut continuing_work, &once_yd.substrate.advance_time(90));
    let dead_yd = resistance(&once_yd, 2);
    expose(&mut world, 3, 51, true, true, &mut phase_yd);
    let after_train_yd = resistances(&world);
    let phase_yd_metrics = metrics(&phase_yd, namespace);
    add_work(&mut continuing_work, &phase_yd.work);

    let final_pressure = world.substrate.advance_time(60);
    add_work(&mut continuing_work, &final_pressure);
    let final_resistance = resistances(&world);
    let candidates = three(|stage| only_historical(&world, stage));
    let candidate_ids = candidates.map(|arrow| format!("{arrow:?}"));
    let candidate_generations = candidates.map(|arrow| world.substrate.arrow_generation(arrow));
    let full_reuse = on_clone(&world, |clone, log| {
        expose(clone, 3, 61, false, false, log);
    });

    let phases = [phase_ab_metrics, phase_xc_metrics, phase_yd_metrics];
    let controls = [
        &ab_reuse,
        &c_alone,
        &xc_gapped,
        &xc_reuse,
        &d_alone,
        &yd_gapped,
        &full_reuse,
    ];
    let quiescent = phases.iter().all(|metric| metric.quiescent)
        && controls.iter().all(|metric| metric.quiescent);
    let mut row = Row {
        seed: config.seed,
        namespace,
        reverse: config.reverse,
        reflect: config.reflect,
        initial_candidates,
        phases,
        after_first: [after_first_ab, after_first_xc, after_first_yd],
        after_train: [after_train_ab, after_train_xc, after_train_yd],
        one_exposure_death: [dead_ab, dead_xc, dead_yd],
        final_resistance,
        candidate_ids,
        candidate_generations,
        ab_reuse,
        c_alone,
        xc_gapped,
        xc_reuse,
        d_alone,
        yd_gapped,
        full_reuse,
        work: continuing_work.total(),
        bytes: world.substrate.persistent_bytes(),
        fingerprint: world.substrate.complete_fingerprint(),
        permanent: world.substrate.permanent_fingerprint(),
        quiescent,
        claims: [false; 12],
        replay: false,
        passed: false,
    };
    row.claims = claims(&row);
    row.passed = row.claims.into_iter().all(|claim| claim);
    row
}

fn claims(row: &Row) -> [bool; 12] {
    let z3 = [0; 3];
    let phase_active = [[2, 0, 0], [2, 2, 0], [2, 2, 2]];
    let phase_impulse = [[3, 0, 0], [4, 3, 0], [4, 4, 3]];
    let phase_primitive = [[2, 2, 0, 0], [2, 2, 2, 0], [2, 2, 2, 2]];
    let phase_proposals = [1, 1, 1];
    let phase_after_first = [[4, 0, 0], [8, 4, 0], [12, 8, 4]];
    let phase_after_train = [[6, 0, 0], [10, 6, 0], [14, 10, 6]];

    let r0 = CONFIGS.iter().any(|config| {
        config.seed == row.seed && config.reverse == row.reverse && config.reflect == row.reflect
    }) && row.namespace == row.seed << 32
        && row.initial_candidates == 0;
    let r1 = (0..3).all(|phase| {
        let metric = &row.phases[phase];
        metric.primitive_trace == phase_primitive[phase]
            && metric.opportunity == phase_active[phase]
            && metric.source == phase_active[phase]
            && metric.candidate == phase_active[phase]
            && metric.proposals == phase_proposals[phase]
    });
    let r2 = row.one_exposure_death == z3;
    let r3 = (0..3).all(|phase| {
        let metric = &row.phases[phase];
        metric.source_trace == phase_active[phase]
            && metric.attribution == phase_active[phase]
            && metric.credit == phase_active[phase]
            && metric.updates == phase_active[phase].into_iter().sum::<usize>() as u64
    });
    let r4 = row.after_first == phase_after_first
        && row.after_train == phase_after_train
        && row.final_resistance == [13, 9, 5];
    let r5 = (0..3).all(|phase| {
        let metric = &row.phases[phase];
        metric.candidate_impulse == phase_impulse[phase]
            && metric.output == phase_active[phase]
            && metric.output_trace_arrivals == phase_active[phase].map(|x| x * 2)
            && metric.output_trace_impulse
                == phase_active[phase].map(|x| i32::try_from(x * 2).expect("small"))
            && metric.output_trace == phase_active[phase]
    });
    let r6 = empty_control(&row.c_alone, [0, 0, 1, 0]) && empty_control(&row.d_alone, [0, 0, 0, 1]);
    let r7 = context_free(&row.xc_gapped, [1, 1, 1, 0], [1, 0, 0])
        && context_free(&row.yd_gapped, [1, 1, 1, 1], [1, 1, 0]);
    let r8 = context_free(&row.ab_reuse, [1, 1, 0, 0], [1, 0, 0])
        && context_free(&row.xc_reuse, [1, 1, 1, 0], [1, 1, 0])
        && context_free(&row.full_reuse, [1, 1, 1, 1], [1, 1, 1]);
    let execution_only = [
        &row.ab_reuse,
        &row.xc_gapped,
        &row.xc_reuse,
        &row.yd_gapped,
        &row.full_reuse,
    ];
    let r9 = execution_only.into_iter().all(|metric| {
        metric.source_trace == [0; 3]
            && metric.attribution == [0; 3]
            && metric.credit == [0; 3]
            && metric.updates == 0
    });
    let r10 = row.candidate_ids.iter().collect::<BTreeSet<_>>().len() == 3
        && row
            .candidate_generations
            .into_iter()
            .all(|generation| generation > 0)
        && row.phases.iter().all(|metric| metric.quiescent);
    let r11 = row.quiescent;
    [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11]
}

fn context_free(metric: &Metrics, primitive: [usize; 4], active: [usize; 3]) -> bool {
    metric.primitive_trace == primitive
        && metric.opportunity == active
        && metric.source == active
        && metric.candidate == active
        && metric.candidate_impulse == active.map(|x| i32::try_from(x * 2).expect("small"))
        && metric.output == active
        && metric.source_trace == [0; 3]
        && metric.output_trace_arrivals == active.map(|x| x * 2)
        && metric.output_trace_impulse == active.map(|x| i32::try_from(x * 2).expect("small"))
        && metric.output_trace == active
        && metric.attribution == [0; 3]
        && metric.credit == [0; 3]
        && metric.updates == 0
        && metric.proposals == 0
        && metric.quiescent
}

fn empty_control(metric: &Metrics, primitive: [usize; 4]) -> bool {
    metric.primitive_trace == primitive
        && metric.opportunity == [0; 3]
        && metric.source == [0; 3]
        && metric.candidate == [0; 3]
        && metric.candidate_impulse == [0; 3]
        && metric.output == [0; 3]
        && metric.source_trace == [0; 3]
        && metric.output_trace_arrivals == [0; 3]
        && metric.output_trace_impulse == [0; 3]
        && metric.output_trace == [0; 3]
        && metric.attribution == [0; 3]
        && metric.credit == [0; 3]
        && metric.updates == 0
        && metric.proposals == 0
        && metric.quiescent
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
    let primitive_sources = primitive_sources.map(|cell| cell.expect("primitive source"));
    let primitive_outlets = primitive_outlets.map(|cell| cell.expect("primitive outlet"));
    let primitive_traces = primitive_traces.map(|cell| cell.expect("primitive trace"));
    let primitive_hubs = primitive_hubs.map(|cell| cell.expect("primitive hub"));

    let mut opportunities = [None; 3];
    let mut sources = [None; 3];
    let mut outputs = [None; 3];
    let mut source_traces = [None; 3];
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
            1,
        )));
    }
    let opportunities = opportunities.map(|cell| cell.expect("opportunity"));
    let sources = sources.map(|cell| cell.expect("stage source"));
    let outputs = outputs.map(|cell| cell.expect("stage output"));
    let source_traces = source_traces.map(|cell| cell.expect("source trace"));
    let output_traces = output_traces.map(|cell| cell.expect("output trace"));
    let output_hubs = output_hubs.map(|cell| cell.expect("output hub"));
    let attributions = attributions.map(|cell| cell.expect("attribution"));
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
        substrate.add_arrow(fixed(output_traces[stage], source_traces[stage], 0, 1));
        substrate.add_arrow(fixed(global_return, source_traces[stage], 0, 1));
        substrate.add_arrow(fixed(source_traces[stage], attributions[stage], 0, 1));
        substrate.add_arrow(modulatory(attributions[stage], sources[stage], 1, 1));
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

fn expose(
    world: &mut World,
    depth: usize,
    start: i64,
    with_context: bool,
    with_return: bool,
    log: &mut Log,
) {
    assert!((1..=3).contains(&depth));
    primitive(world, 0, start, 0);
    primitive(world, 1, start, 1);
    if depth >= 2 {
        primitive(world, 2, start + 2, 2);
    }
    if depth >= 3 {
        primitive(world, 3, start + 4, 3);
    }
    for stage in 0..depth {
        let opportunity_tick = start + 1 + stage as i64 * 2;
        background(world, opportunity_tick);
        if with_context {
            pulse(
                &mut world.substrate,
                world.context,
                opportunity_tick,
                1,
                500 + stage as i32,
            );
        }
        if with_return {
            pulse(
                &mut world.substrate,
                world.global_return,
                opportunity_tick + 2,
                1,
                600 + stage as i32,
            );
        }
    }
    log.execution(world.substrate.propagate());
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

fn on_clone(world: &World, action: impl FnOnce(&mut World, &mut Log)) -> Metrics {
    let mut clone = world.clone();
    let mut log = Log::new();
    action(&mut clone, &mut log);
    metrics(&log, clone.namespace)
}

fn metrics(log: &Log, namespace: u64) -> Metrics {
    Metrics {
        primitive_trace: four(|side| fires(&log.trace, physical(namespace, 30 + side as u64))),
        opportunity: three(|stage| fires(&log.trace, physical(namespace, 100 + stage as u64))),
        source: three(|stage| fires(&log.trace, physical(namespace, 200 + stage as u64))),
        candidate: three(|stage| {
            crossings(
                &log.crossings,
                physical(namespace, 200 + stage as u64),
                physical(namespace, 300 + stage as u64),
            )
        }),
        candidate_impulse: three(|stage| {
            crossing_impulse(
                &log.crossings,
                physical(namespace, 200 + stage as u64),
                physical(namespace, 300 + stage as u64),
            )
        }),
        output: three(|stage| fires(&log.trace, physical(namespace, 300 + stage as u64))),
        source_trace: three(|stage| fires(&log.trace, physical(namespace, 400 + stage as u64))),
        output_trace_arrivals: three(|stage| {
            arrivals(&log.trace, physical(namespace, 600 + stage as u64))
        }),
        output_trace_impulse: three(|stage| {
            arrival_impulse(&log.trace, physical(namespace, 600 + stage as u64))
        }),
        output_trace: three(|stage| fires(&log.trace, physical(namespace, 600 + stage as u64))),
        attribution: three(|stage| fires(&log.trace, physical(namespace, 800 + stage as u64))),
        credit: three(|stage| {
            crossings(
                &log.crossings,
                physical(namespace, 800 + stage as u64),
                physical(namespace, 200 + stage as u64),
            )
        }),
        updates: log.work.local_return_updates,
        proposals: log.work.local_structural_proposals,
        quiescent: log.quiescent,
    }
}

fn candidate_arrows(world: &World, stage: usize) -> Vec<ArrowId> {
    world
        .substrate
        .arrows_between(world.stages[stage].source, world.stages[stage].output)
}

fn candidate_count(world: &World) -> usize {
    (0..3)
        .map(|stage| candidate_arrows(world, stage).len())
        .sum()
}

fn only_historical(world: &World, stage: usize) -> ArrowId {
    let arrows = candidate_arrows(world, stage);
    assert_eq!(arrows.len(), 1, "one historical candidate per stage");
    arrows[0]
}

fn resistance(world: &World, stage: usize) -> u32 {
    let arrows = candidate_arrows(world, stage);
    assert!(arrows.len() <= 1, "at most one candidate per stage");
    arrows
        .first()
        .map_or(0, |arrow| world.substrate.arrow_resistance(*arrow))
}

fn resistances(world: &World) -> [u32; 3] {
    three(|stage| resistance(world, stage))
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
        mode: TransmissionMode::Drive,
    }
}

fn modulatory(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 100,
        mode: TransmissionMode::Modulatory,
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

fn fires(trace: &[TraceEntry], physical_id: u64) -> usize {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id && entry.fired)
        .count()
}

fn arrivals(trace: &[TraceEntry], physical_id: u64) -> usize {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id)
        .count()
}

fn arrival_impulse(trace: &[TraceEntry], physical_id: u64) -> i32 {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id)
        .map(|entry| entry.impulse)
        .sum()
}

fn crossings(crossings: &[Crossing], from: u64, to: u64) -> usize {
    crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .count()
}

fn crossing_impulse(crossings: &[Crossing], from: u64, to: u64) -> i32 {
    crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .map(|crossing| crossing.impulse)
        .sum()
}

fn three<T>(mut function: impl FnMut(usize) -> T) -> [T; 3] {
    [function(0), function(1), function(2)]
}

fn four<T>(mut function: impl FnMut(usize) -> T) -> [T; 4] {
    [function(0), function(1), function(2), function(3)]
}

fn add_work(total: &mut WorkLedger, next: &WorkLedger) {
    total.queue_comparisons += next.queue_comparisons;
    total.spikes_delivered += next.spikes_delivered;
    total.generation_checks += next.generation_checks;
    total.state_updates += next.state_updates;
    total.threshold_checks += next.threshold_checks;
    total.firings += next.firings;
    total.arrow_checks += next.arrow_checks;
    total.spikes_emitted += next.spikes_emitted;
    total.local_eligibility_writes += next.local_eligibility_writes;
    total.local_return_updates += next.local_return_updates;
    total.qualified_return_checks += next.qualified_return_checks;
    total.qualified_return_accepts += next.qualified_return_accepts;
    total.qualified_return_path_edges += next.qualified_return_path_edges;
    total.drive_deliveries += next.drive_deliveries;
    total.modulatory_deliveries += next.modulatory_deliveries;
    total.ordinary_pressure_updates += next.ordinary_pressure_updates;
    total.local_structural_proposals += next.local_structural_proposals;
    total.physical_deallocations += next.physical_deallocations;
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "seed,namespace,reverse,reflect,initial_candidates,phase_primitive_trace,phase_opportunity,phase_p,phase_candidate,phase_candidate_impulse,phase_output,phase_return_relay,phase_output_trace_arrivals,phase_output_trace_impulse,phase_output_trace,phase_modulatory_transmitter,phase_modulatory_crossing,phase_updates,phase_proposals,after_first,after_train,one_exposure_death,final_resistance,candidate_ids,candidate_generations,ab_reuse,c_alone,xc_gapped,xc_reuse,d_alone,yd_gapped,full_reuse,work,bytes,fingerprint,permanent,quiescent,claims,replay,passed\n",
    );
    for row in rows {
        let fields = vec![
            row.seed.to_string(),
            row.namespace.to_string(),
            row.reverse.to_string(),
            row.reflect.to_string(),
            row.initial_candidates.to_string(),
            join_metric_usize(&row.phases, |metric| metric.primitive_trace.to_vec()),
            join_metric_usize(&row.phases, |metric| metric.opportunity.to_vec()),
            join_metric_usize(&row.phases, |metric| metric.source.to_vec()),
            join_metric_usize(&row.phases, |metric| metric.candidate.to_vec()),
            join_metric_i32(&row.phases, |metric| metric.candidate_impulse.to_vec()),
            join_metric_usize(&row.phases, |metric| metric.output.to_vec()),
            join_metric_usize(&row.phases, |metric| metric.source_trace.to_vec()),
            join_metric_usize(&row.phases, |metric| metric.output_trace_arrivals.to_vec()),
            join_metric_i32(&row.phases, |metric| metric.output_trace_impulse.to_vec()),
            join_metric_usize(&row.phases, |metric| metric.output_trace.to_vec()),
            join_metric_usize(&row.phases, |metric| metric.attribution.to_vec()),
            join_metric_usize(&row.phases, |metric| metric.credit.to_vec()),
            row.phases
                .iter()
                .map(|metric| metric.updates.to_string())
                .collect::<Vec<_>>()
                .join("|"),
            row.phases
                .iter()
                .map(|metric| metric.proposals.to_string())
                .collect::<Vec<_>>()
                .join("|"),
            join_nested_u32(&row.after_first),
            join_nested_u32(&row.after_train),
            join_u32(&row.one_exposure_death),
            join_u32(&row.final_resistance),
            row.candidate_ids.join("|"),
            join_u32(&row.candidate_generations),
            signature(&row.ab_reuse),
            signature(&row.c_alone),
            signature(&row.xc_gapped),
            signature(&row.xc_reuse),
            signature(&row.d_alone),
            signature(&row.yd_gapped),
            signature(&row.full_reuse),
            row.work.to_string(),
            row.bytes.to_string(),
            row.fingerprint.to_string(),
            row.permanent.to_string(),
            row.quiescent.to_string(),
            join_bool(&row.claims),
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
    let clauses = rows
        .iter()
        .map(|row| row.claims.into_iter().filter(|claim| *claim).count())
        .sum::<usize>();
    format!(
        "# PX3 LR-C recursive compression definitive v2\n\nOutcome: **{}**.\n\n- rows: `{passed}/{}` passed;\n- independent clauses: `{clauses}/192`;\n- exact replay: `{}`;\n- naturally quiescent: `{}`;\n- native proposals AB/XC/YD: `{}`;\n- one-exposure candidates dead: `{}`;\n- final AB/XC/YD resistance: `{}`;\n- final context-free X/Y/Z traces: `{}`;\n- context-free return-relay/modulatory traffic absent: `{}`;\n- lawful modulatory return is the only learning feedback: `true`;\n- active R3/R4/R5/R6 geometry: `false`;\n- level-specific participant or conjunction API: `false`;\n- PX4 executed: `false`.\n",
        if passed == rows.len() {
            "PX3-LRC-RECURSION DEFINITIVE POSITIVE"
        } else {
            "NEGATIVE"
        },
        rows.len(),
        rows.iter().all(|row| row.replay),
        rows.iter().all(|row| row.quiescent),
        rows.iter()
            .map(|row| row.phases.iter().map(|m| m.proposals).sum::<u64>())
            .sum::<u64>(),
        rows.iter().all(|row| row.one_exposure_death == [0; 3]),
        rows.iter()
            .map(|row| join_u32(&row.final_resistance))
            .collect::<Vec<_>>()
            .join(";"),
        rows.iter()
            .map(|row| join_usize(&row.full_reuse.output_trace))
            .collect::<Vec<_>>()
            .join(";"),
        rows.iter().all(|row| {
            row.ab_reuse.source_trace == [0; 3]
                && row.xc_reuse.source_trace == [0; 3]
                && row.full_reuse.source_trace == [0; 3]
                && row.ab_reuse.credit == [0; 3]
                && row.xc_reuse.credit == [0; 3]
                && row.full_reuse.credit == [0; 3]
        }),
    )
}

fn signature(metric: &Metrics) -> String {
    format!(
        "prim={}~o={}~p={}~cand={}~imp={}~out={}~relay={}~arr={}~timp={}~trace={}~tx={}~mod={}~updates={}~prop={}~q={}",
        join_usize(&metric.primitive_trace),
        join_usize(&metric.opportunity),
        join_usize(&metric.source),
        join_usize(&metric.candidate),
        join_i32(&metric.candidate_impulse),
        join_usize(&metric.output),
        join_usize(&metric.source_trace),
        join_usize(&metric.output_trace_arrivals),
        join_i32(&metric.output_trace_impulse),
        join_usize(&metric.output_trace),
        join_usize(&metric.attribution),
        join_usize(&metric.credit),
        metric.updates,
        metric.proposals,
        metric.quiescent,
    )
}

fn join_metric_usize(metrics: &[Metrics; 3], field: impl Fn(&Metrics) -> Vec<usize>) -> String {
    metrics
        .iter()
        .map(|metric| join_usize(&field(metric)))
        .collect::<Vec<_>>()
        .join(";")
}

fn join_metric_i32(metrics: &[Metrics; 3], field: impl Fn(&Metrics) -> Vec<i32>) -> String {
    metrics
        .iter()
        .map(|metric| join_i32(&field(metric)))
        .collect::<Vec<_>>()
        .join(";")
}

fn join_nested_u32(values: &[[u32; 3]; 3]) -> String {
    values
        .iter()
        .map(|value| join_u32(value))
        .collect::<Vec<_>>()
        .join(";")
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

fn join_bool(values: &[bool]) -> String {
    values
        .iter()
        .map(bool::to_string)
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
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_is_frozen() {
        surface();
        assert_eq!(CONFIGS.len(), 16);
        assert_eq!(CONFIGS[0].seed, 94003);
        assert_eq!(CONFIGS[15].seed, 94071);
    }

    #[test]
    fn recursive_schedule_is_frozen() {
        assert_eq!([[2, 0, 0], [2, 2, 0], [2, 2, 2]][2], [2, 2, 2]);
        assert_eq!([[3, 0, 0], [4, 3, 0], [4, 4, 3]][2], [4, 4, 3]);
        assert_eq!([[4, 0, 0], [8, 4, 0], [12, 8, 4]][2], [12, 8, 4]);
        assert_eq!([[6, 0, 0], [10, 6, 0], [14, 10, 6]][2], [14, 10, 6]);
        assert_eq!([13, 9, 5], [13, 9, 5]);
    }
}
