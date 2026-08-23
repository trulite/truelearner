#![forbid(unsafe_code)]

use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Crossing, Execution, PlasticSubstrate, SpikeInput,
    TraceEntry, WorkLedger,
};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX0: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX2: &str = "921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18";
const PX2_AUDIT: &str = "7076aca03014d19040020b6bfb126e92f7d25dcac3df9cdab92de7dd7849c6fe";
const PX2_AUTHORITY: &str =
    "98647ab1563593e18e345cd7e5a71c4991d18b397dfe2dec71a4756106d96509";
const GATE_SOURCE: &str =
    "969042a740f92237c577d82c67399447040cb96d2c003c28100034566e30d5aa";
const GATE_CSV: &str = "1c350355843ca0020a275813f1e7aa9fb36b63317375327b5bd437182d144469";
const GATE_AUDIT: &str =
    "4e0f38d31df19fa62449340faf65c31a80b191ec0b90a0293b7fc4fe90f7f321";
const READINESS: &str =
    "92b3a557dd29e362cbefda535dc2bb0355837202d61c96afe25ac85b23203c5a";
const PROTOCOL: &str =
    "fb58387fc8d6f214683fe3d65b5b1c4261eb910e37135fab60db7a8a357d0151";
const EXECUTION_PROTOCOL: &str =
    "b697904ee90c9b7e120e5a20a8c9bd84ceb95554257295081bcabd8d3066cc1a";

const NAMESPACE_BASE: u64 = 0x6_5300_0000_0000;
const SEEDS: usize = 16;
const CSV: &str = "results/px3_physical_event_organization_definitive.csv";
const MD: &str = "results/px3_physical_event_organization_definitive.md";
const CSV_STAGE: &str = "results/.px3_physical_event_organization_definitive.csv.staging";
const MD_STAGE: &str = "results/.px3_physical_event_organization_definitive.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    FullRecursive,
    RecurrenceNoReturn,
    ReturnNoJoint,
    SamePathAmplitudeRepeat,
}

impl Kind {
    const ALL: [Self; 4] = [
        Self::FullRecursive,
        Self::RecurrenceNoReturn,
        Self::ReturnNoJoint,
        Self::SamePathAmplitudeRepeat,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::FullRecursive => "full-recursive",
            Self::RecurrenceNoReturn => "recurrence-no-return",
            Self::ReturnNoJoint => "return-no-joint",
            Self::SamePathAmplitudeRepeat => "same-path-amplitude-repeat",
        }
    }

    fn index(self) -> u64 {
        match self {
            Self::FullRecursive => 0,
            Self::RecurrenceNoReturn => 1,
            Self::ReturnNoJoint => 2,
            Self::SamePathAmplitudeRepeat => 3,
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
    proposals: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    primary: String,
    control_a: String,
    control_b: String,
    resistance: String,
    initial_candidates: usize,
    historical_candidates: usize,
    work: u64,
    bytes: usize,
    fingerprint: u64,
    permanent: u64,
    quiescent: bool,
    claims: [bool; 10],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: usize,
    stratum: &'static str,
    kind: Kind,
    namespace: u64,
    reverse: bool,
    reflect: bool,
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
            println!("PX3_PHYSICAL_EVENT_ORGANIZATION_DEFINITIVE_PREFLIGHT_OK");
        }
        [argument] if argument == "--definitive" => {
            audit();
            surface();
            absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
            evidence();
        }
        _ => std::process::exit(2),
    }
}

fn evidence() {
    eprintln!("PX3_PHYSICAL_EVENT_ORGANIZATION_DEFINITIVE_EVIDENCE_SPENT");
    let mut rows = Vec::new();
    for seed in 0..SEEDS {
        for kind in Kind::ALL {
            rows.push(replay(seed, kind));
        }
    }
    assert_eq!(rows.len(), 64);
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(MD_STAGE, MD, &report(&rows));
}

fn audit() {
    for (path, expected) in [
        ("crates/px0-physical-correspondence/src/lib.rs", PX0),
        ("results/px2_physical_causal_direction_definitive.csv", PX2),
        (
            "experiments/px2_physical_causal_direction_definitive_result_audit.md",
            PX2_AUDIT,
        ),
        (
            "experiments/px2_physical_causal_direction_authority_handoff.md",
            PX2_AUTHORITY,
        ),
        ("arms/px3-recursive-compression-gate/src/main.rs", GATE_SOURCE),
        ("results/px3_recursive_compression_gate_v1.csv", GATE_CSV),
        (
            "experiments/px3_recursive_compression_gate_result_audit_v1.md",
            GATE_AUDIT,
        ),
        (
            "experiments/px3_physical_event_organization_development_readiness_v1.md",
            READINESS,
        ),
        (
            "experiments/px3_physical_event_organization_definitive_protocol_v1.md",
            PROTOCOL,
        ),
        (
            "experiments/px3_physical_event_organization_definitive_execution_protocol_v1.md",
            EXECUTION_PROTOCOL,
        ),
    ] {
        assert_eq!(sha(path), expected, "frozen input changed: {path}");
    }
}

fn surface() {
    assert_eq!(SEEDS, 16);
    assert_eq!(Kind::ALL.into_iter().collect::<BTreeSet<_>>().len(), 4);
    let namespaces = (0..SEEDS)
        .flat_map(|seed| Kind::ALL.map(|kind| namespace(seed, kind)))
        .collect::<BTreeSet<_>>();
    assert_eq!(namespaces.len(), 64);
    assert!(namespaces.iter().all(|value| *value >= NAMESPACE_BASE));
    for forbidden in [
        "arms/px3-physical-event-organization-definitive/src/px4.rs",
        "results/px4_definitive.csv",
    ] {
        assert!(!Path::new(forbidden).exists());
    }
}

fn replay(seed: usize, kind: Kind) -> Row {
    let first = run_cell(seed, kind);
    let second = run_cell(seed, kind);
    let exact = first == second;
    let mut row = first;
    row.replay = exact;
    row.observation.claims[9] &= exact;
    row.passed = row.observation.claims.into_iter().all(|claim| claim);
    row
}

fn run_cell(seed: usize, kind: Kind) -> Row {
    let stratum_index = seed % 4;
    let reverse = stratum_index == 1 || stratum_index == 3;
    let reflect = stratum_index >= 2;
    let namespace = namespace(seed, kind);
    let observation = match kind {
        Kind::FullRecursive => full_recursive(namespace, reverse, reflect),
        Kind::RecurrenceNoReturn => recurrence_no_return(namespace, reverse, reflect),
        Kind::ReturnNoJoint => return_no_joint(namespace, reverse, reflect),
        Kind::SamePathAmplitudeRepeat => same_path(namespace, reverse, reflect),
    };
    Row {
        seed,
        stratum: ["G0", "G1", "G2", "G3"][stratum_index],
        kind,
        namespace,
        reverse,
        reflect,
        passed: observation.claims.into_iter().all(|claim| claim),
        observation,
        replay: false,
    }
}

fn namespace(seed: usize, kind: Kind) -> u64 {
    NAMESPACE_BASE + seed as u64 * 0x1000_0000 + kind.index() * 0x0100_0000
}

fn full_recursive(namespace: u64, reverse: bool, reflect: bool) -> Observation {
    let mut world = build(namespace, reverse, reflect);
    let initial_candidates = candidate_count(&world);
    let mut full_work = WorkLedger::default();

    let mut ab = Log::new();
    expose(&mut world, 1, 0, true, true, &mut ab);
    let after_first_ab = resistances(&world);
    let mut once_ab = world.clone();
    once_ab.substrate.advance_time(50);
    let dead_ab = resistance(&once_ab, 0);
    expose(&mut world, 1, 11, true, true, &mut ab);
    let after_train_ab = resistances(&world);
    add_work(&mut full_work, &ab.work);

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

    let mut xc = Log::new();
    expose(&mut world, 2, 20, true, true, &mut xc);
    let after_first_xc = resistances(&world);
    let mut once_xc = world.clone();
    once_xc.substrate.advance_time(70);
    let dead_xc = resistance(&once_xc, 1);
    expose(&mut world, 2, 31, true, true, &mut xc);
    let after_train_xc = resistances(&world);
    add_work(&mut full_work, &xc.work);

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

    let mut yd = Log::new();
    expose(&mut world, 3, 40, true, true, &mut yd);
    let after_first_yd = resistances(&world);
    let mut once_yd = world.clone();
    once_yd.substrate.advance_time(90);
    let dead_yd = resistance(&once_yd, 2);
    expose(&mut world, 3, 51, true, true, &mut yd);
    let after_train_yd = resistances(&world);
    add_work(&mut full_work, &yd.work);
    add_work(&mut full_work, &world.substrate.advance_time(60));
    let final_resistance = resistances(&world);
    let full_reuse = on_clone(&world, |clone, log| {
        expose(clone, 3, 61, false, false, log);
    });

    let phases = [metrics(&ab, namespace), metrics(&xc, namespace), metrics(&yd, namespace)];
    let phase_active = [[2, 0, 0], [2, 2, 0], [2, 2, 2]];
    let phase_impulse = [[3, 0, 0], [4, 3, 0], [4, 4, 3]];
    let phase_primitive = [[2, 2, 0, 0], [2, 2, 2, 0], [2, 2, 2, 2]];
    let one_death = [dead_ab, dead_xc, dead_yd];
    let after_first = [after_first_ab, after_first_xc, after_first_yd];
    let after_train = [after_train_ab, after_train_xc, after_train_yd];

    let p1 = (0..3).all(|index| phases[index].primitive_trace == phase_primitive[index]);
    let p2 = (0..3).all(|index| {
        phases[index].opportunity == phase_active[index]
            && phases[index].source == phase_active[index]
            && phases[index].candidate == phase_active[index]
            && phases[index].candidate_impulse == phase_impulse[index]
            && phases[index].proposals == 1
    });
    let p3 = (0..3).all(|index| {
        phases[index].output == phase_active[index]
            && phases[index].output_trace_arrivals == phase_active[index].map(|x| x * 2)
            && phases[index].output_trace_impulse
                == phase_active[index].map(|x| i32::try_from(x * 2).expect("small"))
            && phases[index].output_trace == phase_active[index]
    }) && context_free(&full_reuse, [1; 4], [1; 3]);
    let p4 = (0..3).all(|index| {
        phases[index].source_trace == phase_active[index]
            && phases[index].attribution == phase_active[index]
            && phases[index].credit == phase_active[index]
    });
    let p5 = one_death == [0; 3]
        && after_first == [[4, 0, 0], [8, 4, 0], [12, 8, 4]]
        && after_train == [[6, 0, 0], [10, 6, 0], [14, 10, 6]]
        && final_resistance == [13, 9, 5];
    let p7 = context_free(&ab_reuse, [1, 1, 0, 0], [1, 0, 0])
        && empty_control(&c_alone, [0, 0, 1, 0])
        && context_free(&xc_gapped, [1, 1, 1, 0], [1, 0, 0])
        && context_free(&xc_reuse, [1, 1, 1, 0], [1, 1, 0])
        && empty_control(&d_alone, [0, 0, 0, 1])
        && context_free(&yd_gapped, [1; 4], [1, 1, 0]);
    let p8 = (0..3).all(|index| phases[index].source == phases[index].opportunity);
    let quiescent = phases.iter().all(|value| value.quiescent)
        && [
            &ab_reuse,
            &c_alone,
            &xc_gapped,
            &xc_reuse,
            &d_alone,
            &yd_gapped,
            &full_reuse,
        ]
        .into_iter()
        .all(|value| value.quiescent);
    let claims = [
        initial_candidates == 0,
        p1,
        p2,
        p3,
        p4,
        p5,
        one_death == [0; 3],
        p7,
        p8,
        quiescent,
    ];
    Observation {
        primary: format!(
            "ab={}~xc={}~yd={}~full={}",
            signature(&phases[0]),
            signature(&phases[1]),
            signature(&phases[2]),
            signature(&full_reuse)
        ),
        control_a: format!(
            "ab={}~c={}~xcgap={}",
            signature(&ab_reuse),
            signature(&c_alone),
            signature(&xc_gapped)
        ),
        control_b: format!(
            "xc={}~d={}~ydgap={}",
            signature(&xc_reuse),
            signature(&d_alone),
            signature(&yd_gapped)
        ),
        resistance: format!(
            "first={}~train={}~death={}~final={}",
            join_nested_u32(&after_first),
            join_nested_u32(&after_train),
            join_u32(&one_death),
            join_u32(&final_resistance)
        ),
        initial_candidates,
        historical_candidates: candidate_count(&world),
        work: full_work.total(),
        bytes: world.substrate.persistent_bytes(),
        fingerprint: world.substrate.complete_fingerprint(),
        permanent: world.substrate.permanent_fingerprint(),
        quiescent,
        claims,
    }
}

fn recurrence_no_return(namespace: u64, reverse: bool, reflect: bool) -> Observation {
    let mut world = build(namespace, reverse, reflect);
    let initial_candidates = candidate_count(&world);
    let mut training = Log::new();
    expose(&mut world, 1, 0, true, false, &mut training);
    expose(&mut world, 1, 4, true, false, &mut training);
    let trained_resistance = resistances(&world);
    let heldout = on_clone(&world, |clone, log| {
        expose(clone, 1, 8, false, false, log);
    });
    let pressure_work = world.substrate.advance_time(50);
    let final_resistance = resistances(&world);
    let metric = metrics(&training, namespace);
    let p1 = metric.primitive_trace == [2, 2, 0, 0];
    let p2 = metric.opportunity == [2, 0, 0]
        && metric.source == [2, 0, 0]
        && metric.candidate == [2, 0, 0]
        && metric.candidate_impulse == [2, 0, 0]
        && metric.proposals == 1;
    let p3 = metric.output == [2, 0, 0]
        && metric.output_trace_arrivals == [4, 0, 0]
        && metric.output_trace_impulse == [4, 0, 0]
        && metric.output_trace == [2, 0, 0];
    let p4 = metric.source_trace == [2, 0, 0]
        && metric.attribution == [0; 3]
        && metric.credit == [0; 3];
    let p6 = trained_resistance == [1, 0, 0]
        && final_resistance == [0; 3]
        && heldout.output == [0; 3]
        && heldout.output_trace == [0; 3];
    let p8 = metric.source == metric.opportunity && heldout.source == heldout.opportunity;
    let quiescent = metric.quiescent && heldout.quiescent;
    let claims = [
        initial_candidates == 0,
        p1,
        p2,
        p3,
        p4,
        trained_resistance == [1, 0, 0],
        p6,
        heldout.credit == [0; 3],
        p8,
        quiescent,
    ];
    Observation {
        primary: signature(&metric),
        control_a: signature(&heldout),
        control_b: "none".to_owned(),
        resistance: format!(
            "trained={}~final={}",
            join_u32(&trained_resistance),
            join_u32(&final_resistance)
        ),
        initial_candidates,
        historical_candidates: candidate_count(&world),
        work: training.work.total() + pressure_work.total(),
        bytes: world.substrate.persistent_bytes(),
        fingerprint: world.substrate.complete_fingerprint(),
        permanent: world.substrate.permanent_fingerprint(),
        quiescent,
        claims,
    }
}

fn return_no_joint(namespace: u64, reverse: bool, reflect: bool) -> Observation {
    let mut world = build(namespace, reverse, reflect);
    let initial_candidates = candidate_count(&world);
    let mut log = Log::new();
    for start in [0, 11] {
        background(&mut world, start + 1);
        pulse(&mut world.substrate, world.context, start + 1, 1, 500);
        pulse(
            &mut world.substrate,
            world.global_return,
            start + 3,
            1,
            600,
        );
        log.execution(world.substrate.propagate());
    }
    let metric = metrics(&log, namespace);
    let empty = empty_control(&metric, [0; 4]);
    let claims = [
        initial_candidates == 0,
        metric.primitive_trace == [0; 4],
        metric.opportunity == [0; 3] && metric.source == [0; 3],
        metric.output == [0; 3] && metric.output_trace == [0; 3],
        metric.attribution == [0; 3] && metric.credit == [0; 3],
        candidate_count(&world) == 0,
        empty,
        metric.proposals == 0,
        metric.source == metric.opportunity,
        metric.quiescent,
    ];
    Observation {
        primary: signature(&metric),
        control_a: "none".to_owned(),
        control_b: "none".to_owned(),
        resistance: "0|0|0".to_owned(),
        initial_candidates,
        historical_candidates: candidate_count(&world),
        work: log.work.total(),
        bytes: world.substrate.persistent_bytes(),
        fingerprint: world.substrate.complete_fingerprint(),
        permanent: world.substrate.permanent_fingerprint(),
        quiescent: metric.quiescent,
        claims,
    }
}

fn same_path(namespace: u64, reverse: bool, reflect: bool) -> Observation {
    let mut world = build(namespace, reverse, reflect);
    let initial_candidates = candidate_count(&world);
    let mut log = Log::new();
    pulse(
        &mut world.substrate,
        world.primitive_sources[0],
        0,
        4,
        0,
    );
    background(&mut world, 1);
    pulse(
        &mut world.substrate,
        world.primitive_sources[0],
        4,
        1,
        0,
    );
    background(&mut world, 5);
    log.execution(world.substrate.propagate());
    let metric = metrics(&log, namespace);
    let claims = [
        initial_candidates == 0,
        metric.primitive_trace == [2, 0, 0, 0],
        metric.opportunity == [0; 3] && metric.source == [0; 3],
        metric.output == [0; 3] && metric.output_trace == [0; 3],
        metric.attribution == [0; 3] && metric.credit == [0; 3],
        candidate_count(&world) == 0,
        metric.proposals == 0,
        empty_control(&metric, [2, 0, 0, 0]),
        metric.source == metric.opportunity,
        metric.quiescent,
    ];
    Observation {
        primary: signature(&metric),
        control_a: "strong=4~repeat=1".to_owned(),
        control_b: "none".to_owned(),
        resistance: "0|0|0".to_owned(),
        initial_candidates,
        historical_candidates: candidate_count(&world),
        work: log.work.total(),
        bytes: world.substrate.persistent_bytes(),
        fingerprint: world.substrate.complete_fingerprint(),
        permanent: world.substrate.permanent_fingerprint(),
        quiescent: metric.quiescent,
        claims,
    }
}

fn context_free(metric: &Metrics, primitive: [usize; 4], active: [usize; 3]) -> bool {
    metric.primitive_trace == primitive
        && metric.opportunity == active
        && metric.source == active
        && metric.candidate == active
        && metric.candidate_impulse == active.map(|x| i32::try_from(x * 2).expect("small"))
        && metric.output == active
        && metric.source_trace == active
        && metric.output_trace_arrivals == active.map(|x| x * 2)
        && metric.output_trace_impulse
            == active.map(|x| i32::try_from(x * 2).expect("small"))
        && metric.output_trace == active
        && metric.attribution == [0; 3]
        && metric.credit == [0; 3]
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
    let opportunities = opportunities.map(|cell| cell.expect("opportunity"));
    let sources = sources.map(|cell| cell.expect("stage source"));
    let outputs = outputs.map(|cell| cell.expect("stage output"));
    let source_traces = source_traces.map(|cell| cell.expect("source trace"));
    let source_hubs = source_hubs.map(|cell| cell.expect("source hub"));
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
    total.ordinary_pressure_updates += next.ordinary_pressure_updates;
    total.local_structural_proposals += next.local_structural_proposals;
    total.physical_deallocations += next.physical_deallocations;
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "seed,stratum,world,namespace,reverse,reflect,initial_candidates,historical_candidates,primary,control_a,control_b,resistance,p0,p1,p2,p3,p4,p5,p6,p7,p8,p9,claims,work,bytes,fingerprint,permanent,quiescent,replay,passed\n",
    );
    for row in rows {
        let claims = row.observation.claims;
        let mut fields = vec![
            row.seed.to_string(),
            row.stratum.to_owned(),
            row.kind.name().to_owned(),
            format!("0x{:x}", row.namespace),
            row.reverse.to_string(),
            row.reflect.to_string(),
            row.observation.initial_candidates.to_string(),
            row.observation.historical_candidates.to_string(),
            row.observation.primary.clone(),
            row.observation.control_a.clone(),
            row.observation.control_b.clone(),
            row.observation.resistance.clone(),
        ];
        fields.extend(claims.map(|claim| claim.to_string()));
        fields.extend([
            claims.into_iter().filter(|claim| *claim).count().to_string(),
            row.observation.work.to_string(),
            row.observation.bytes.to_string(),
            row.observation.fingerprint.to_string(),
            row.observation.permanent.to_string(),
            row.observation.quiescent.to_string(),
            row.replay.to_string(),
            row.passed.to_string(),
        ]);
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

fn report(rows: &[Row]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let claims = rows
        .iter()
        .flat_map(|row| row.observation.claims)
        .filter(|claim| *claim)
        .count();
    let work = rows.iter().map(|row| row.observation.work).sum::<u64>();
    let worlds = Kind::ALL
        .map(|kind| {
            format!(
                "{}={}/16",
                kind.name(),
                rows.iter()
                    .filter(|row| row.kind == kind && row.passed)
                    .count()
            )
        })
        .join("; ");
    format!(
        "# PX3 physical event organization definitive result\n\nOutcome: **{}** (`{passed}/64` cells; `{claims}/640` claims).\n\n- worlds: `{worlds}`;\n- exact complete-state replay: `{}`;\n- naturally quiescent: `{}`;\n- total ledgered work: `{work}`;\n- PX0--PX3 physical mechanism changed: `false`;\n- Event/member/contributor/composite/level representation added: `false`;\n- PX4 executed: `false`.\n",
        if passed == 64 && claims == 640 {
            "PASS"
        } else {
            "FAIL"
        },
        rows.iter().all(|row| row.replay),
        rows.iter().all(|row| row.observation.quiescent),
    )
}

fn signature(metric: &Metrics) -> String {
    format!(
        "prim={}~o={}~p={}~cand={}~imp={}~out={}~pt={}~arr={}~timp={}~trace={}~m={}~credit={}~prop={}~q={}",
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
        metric.proposals,
        metric.quiescent,
    )
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
    fn matrix_is_exact_and_fresh() {
        let namespaces = (0..SEEDS)
            .flat_map(|seed| Kind::ALL.map(|kind| namespace(seed, kind)))
            .collect::<BTreeSet<_>>();
        assert_eq!(SEEDS * Kind::ALL.len(), 64);
        assert_eq!(namespaces.len(), 64);
        assert!(namespaces.iter().all(|value| *value >= NAMESPACE_BASE));
    }

    #[test]
    fn worlds_and_claim_count_are_frozen() {
        assert_eq!(Kind::ALL.len(), 4);
        assert_eq!(Kind::ALL.into_iter().collect::<BTreeSet<_>>().len(), 4);
        assert_eq!(64 * 10, 640);
    }
}
