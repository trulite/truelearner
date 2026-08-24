#![forbid(unsafe_code)]

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use truelearner_core::{
    ActivityKind, ArenaId, ArrowSpec, CellId, CellSpec, ExecutionCost, ExecutorKind,
    LayoutKind, MechanicalConfig, PlasticSubstrate, RunResult, SchedulerKind, SpikeInput,
    TransmissionMode, TraversalKind, Work,
};

const OUTWARD_REGION: i16 = 1;
const REPETITIONS: usize = 3;

#[derive(Clone)]
struct StressWorld {
    name: &'static str,
    body: Vec<u8>,
    inputs: Vec<SpikeInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    crossings: Vec<truelearner_core::Crossing>,
    work: PhysicalWork,
    clock: truelearner_core::PhysicalClock,
    body: Vec<u8>,
    naturally_quiescent: bool,
    cost: ExecutionCost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PhysicalWork {
    drive_deliveries: u64,
    modulatory_deliveries: u64,
    local_return_updates: u64,
    local_structural_proposals: u64,
    physical_deallocations: u64,
}

impl From<Work> for PhysicalWork {
    fn from(work: Work) -> Self {
        Self {
            drive_deliveries: work.drive_deliveries,
            modulatory_deliveries: work.modulatory_deliveries,
            local_return_updates: work.local_return_updates,
            local_structural_proposals: work.local_structural_proposals,
            physical_deallocations: work.physical_deallocations,
        }
    }
}

#[derive(Clone, Copy)]
struct Candidate {
    name: &'static str,
    config: MechanicalConfig,
}

fn main() {
    let output = output_directory();
    let worlds = stress_worlds();
    let candidates = candidates();
    let mut csv = String::from(
        "world,configuration,elapsed_median_ns,queue_ops,comparisons,scans,allocations,bytes_touched,peak_resident_bytes,adjacency_accesses,frontier_samples,active_frontier_total,active_frontier_max,eligible_frontier_total,eligible_frontier_max,batches,batched_items,batch_max,batch_histogram,batch_fallback_zero_delay\n",
    );
    let mut comparisons = 0usize;

    for world in &worlds {
        let (reference, _) = measured(world, MechanicalConfig::REFERENCE);
        let (reference_replay, _) = measured(world, MechanicalConfig::REFERENCE);
        assert_eq!(reference, reference_replay, "reference replay: {}", world.name);

        for candidate in candidates {
            let mut observations = Vec::with_capacity(REPETITIONS);
            let mut elapsed = Vec::with_capacity(REPETITIONS);
            for _ in 0..REPETITIONS {
                let (observation, duration) = measured(world, candidate.config);
                assert_physics(world.name, &reference, &observation);
                observations.push(observation);
                elapsed.push(duration.as_nanos());
            }
            assert!(
                observations.windows(2).all(|pair| pair[0] == pair[1]),
                "candidate replay: {} {}",
                world.name,
                candidate.name
            );
            elapsed.sort_unstable();
            write_row(
                &mut csv,
                world.name,
                candidate.name,
                elapsed[REPETITIONS / 2],
                observations[0].cost,
            );
            comparisons += 1;
        }
    }

    fs::create_dir_all(&output).expect("create PSEL0 output directory");
    let csv_path = output.join("psel0_costs.csv");
    fs::write(&csv_path, &csv).expect("write PSEL0 costs");
    let report = format!(
        "# PSEL0 Mechanical Stress Result\n\n- worlds: `{}`\n- candidates: `{}`\n- physics comparisons: `{comparisons}/{comparisons}`\n- candidate repetitions: `{REPETITIONS}`\n- exact replay: `true`\n- natural quiescence: `true`\n- cost table: `psel0_costs.csv`\n",
        worlds.len(),
        candidates.len(),
    );
    fs::write(output.join("psel0_report.md"), report).expect("write PSEL0 report");
    println!(
        "PSEL0_MECHANICAL_STRESS_PASS worlds={} candidates={} comparisons={comparisons} repetitions={REPETITIONS} output={}",
        worlds.len(),
        candidates.len(),
        output.display()
    );
}

fn output_directory() -> PathBuf {
    let mut args = std::env::args().skip(1);
    match (args.next().as_deref(), args.next(), args.next()) {
        (Some("--output"), Some(path), None) => PathBuf::from(path),
        (None, None, None) => PathBuf::from("results/psel0_mechanics"),
        _ => panic!("usage: psel0-mechanics [--output DIRECTORY]"),
    }
}

fn candidates() -> [Candidate; 4] {
    let base = MechanicalConfig {
        scheduler: SchedulerKind::TimingWheel,
        traversal: TraversalKind::Adjacency,
        activity: ActivityKind::Frontier,
        layout: LayoutKind::AoS,
        executor: ExecutorKind::Scalar,
    };
    [
        Candidate {
            name: "A_AoS_Scalar",
            config: base,
        },
        Candidate {
            name: "B_AoS_Batched",
            config: MechanicalConfig {
                executor: ExecutorKind::Batched,
                ..base
            },
        },
        Candidate {
            name: "C_SoA_Scalar",
            config: MechanicalConfig {
                layout: LayoutKind::SoA,
                ..base
            },
        },
        Candidate {
            name: "D_SoA_Batched",
            config: MechanicalConfig {
                layout: LayoutKind::SoA,
                executor: ExecutorKind::Batched,
                ..base
            },
        },
    ]
}

fn measured(world: &StressWorld, config: MechanicalConfig) -> (Observation, Duration) {
    let mut substrate =
        PlasticSubstrate::from_body_bytes(&world.body).expect("decode frozen stress body");
    substrate.reconfigure_mechanics(config);
    let started = Instant::now();
    let result = substrate.arrive(&world.inputs, OUTWARD_REGION);
    let elapsed = started.elapsed();
    let observation = observation(&substrate, result);
    assert!(observation.naturally_quiescent, "world must quiesce");
    (observation, elapsed)
}

fn observation(substrate: &PlasticSubstrate, result: RunResult) -> Observation {
    Observation {
        crossings: result.crossings,
        work: result.work.into(),
        clock: substrate.clock(),
        body: substrate
            .canonical_body_bytes(2)
            .expect("encode final stress body"),
        naturally_quiescent: result.naturally_quiescent,
        cost: result.execution_cost,
    }
}

fn assert_physics(name: &str, reference: &Observation, candidate: &Observation) {
    assert_eq!(reference.crossings, candidate.crossings, "crossings: {name}");
    assert_eq!(reference.work, candidate.work, "physical work: {name}");
    assert_eq!(reference.clock, candidate.clock, "clock: {name}");
    assert_eq!(reference.body, candidate.body, "durable body: {name}");
    assert_eq!(
        reference.naturally_quiescent, candidate.naturally_quiescent,
        "quiescence: {name}"
    );
}

fn write_row(
    output: &mut String,
    world: &str,
    configuration: &str,
    elapsed_ns: u128,
    cost: ExecutionCost,
) {
    let histogram = cost
        .batch_histogram
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join("|");
    writeln!(
        output,
        "{world},{configuration},{elapsed_ns},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\"{}\",{}",
        cost.queue_ops,
        cost.comparisons,
        cost.scans,
        cost.allocations,
        cost.bytes_touched,
        cost.peak_resident_bytes,
        cost.adjacency_accesses,
        cost.frontier_samples,
        cost.active_frontier_total,
        cost.active_frontier_max,
        cost.eligible_frontier_total,
        cost.eligible_frontier_max,
        cost.batches,
        cost.batched_items,
        cost.batch_max,
        histogram,
        cost.batch_fallback_zero_delay,
    )
    .unwrap();
}

fn stress_worlds() -> Vec<StressWorld> {
    vec![
        many_cells_sparse(),
        dense_layers(),
        long_delays(),
        many_same_tick(),
        high_fanout(),
        heavy_modulation(),
        mostly_dormant(),
        zero_delay_fallback(),
    ]
}

fn substrate(arena: u64, cells: usize, arrows: usize) -> PlasticSubstrate {
    PlasticSubstrate::with_capacity(
        ArenaId(arena),
        u32::try_from(cells + 32).unwrap(),
        u32::try_from(arrows + 32).unwrap(),
    )
}

fn add_cells(body: &mut PlasticSubstrate, count: usize, threshold: impl Fn(usize) -> i32) {
    for index in 0..count {
        body.add_cell(CellSpec {
            physical_id: 1_000_000 + index as u64,
            position: i32::try_from(index).unwrap().saturating_mul(10),
            region: 0,
            threshold: threshold(index),
            resistance: 1_000,
        });
    }
}

fn drive(body: &mut PlasticSubstrate, from: usize, to: usize, delay: i64, coupling: i32) {
    body.add_arrow(ArrowSpec {
        from: CellId(from as u64),
        to: CellId(to as u64),
        delay,
        phase: 0,
        coupling,
        resistance: 1_000,
        mode: TransmissionMode::Drive,
    });
}

fn modulation(body: &mut PlasticSubstrate, from: usize, to: usize, delay: i64) {
    body.add_arrow(ArrowSpec {
        from: CellId(from as u64),
        to: CellId(to as u64),
        delay,
        phase: 1,
        coupling: 1,
        resistance: 1_000,
        mode: TransmissionMode::Modulatory,
    });
}

fn input(target: usize, tick: i64, serial: usize) -> SpikeInput {
    SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 9_000_000 + serial as u64,
        target: CellId(target as u64),
        impulse: 1,
    }
}

fn freeze(name: &'static str, body: PlasticSubstrate, inputs: Vec<SpikeInput>) -> StressWorld {
    StressWorld {
        name,
        body: body.canonical_body_bytes(1).expect("freeze stress body"),
        inputs,
    }
}

fn many_cells_sparse() -> StressWorld {
    let cells = 4_096;
    let arrows = cells - 1;
    let mut body = substrate(101, cells, arrows);
    add_cells(&mut body, cells, |index| if index < 16 { 1 } else { 1_000 });
    for index in 0..arrows {
        drive(&mut body, index, index + 1, 1, 0);
    }
    freeze(
        "many_cells_sparse",
        body,
        (0..16).map(|index| input(index, 1, index)).collect(),
    )
}

fn dense_layers() -> StressWorld {
    let width = 64;
    let layers = 4;
    let cells = width * layers;
    let arrows = width * (layers - 1) * 4;
    let mut body = substrate(102, cells, arrows);
    add_cells(&mut body, cells, |_| 1);
    for layer in 0..layers - 1 {
        for source in 0..width {
            for offset in 0..4 {
                let from = layer * width + source;
                let to = (layer + 1) * width + (source + offset) % width;
                drive(&mut body, from, to, 1, 1);
            }
        }
    }
    freeze(
        "dense_layers",
        body,
        (0..width).map(|index| input(index, 1, index)).collect(),
    )
}

fn long_delays() -> StressWorld {
    let pairs = 256;
    let mut body = substrate(103, pairs * 2, pairs);
    add_cells(&mut body, pairs * 2, |index| if index < pairs { 1 } else { 1_000 });
    for index in 0..pairs {
        drive(&mut body, index, pairs + index, 128 + (index % 32) as i64, 1);
    }
    freeze(
        "long_delays",
        body,
        (0..pairs).map(|index| input(index, 1, index)).collect(),
    )
}

fn many_same_tick() -> StressWorld {
    let pairs = 1_024;
    let mut body = substrate(104, pairs * 2, pairs);
    add_cells(&mut body, pairs * 2, |index| if index < pairs { 1 } else { 1_000 });
    for index in 0..pairs {
        drive(&mut body, index, pairs + index, 1, 1);
    }
    freeze(
        "many_same_tick",
        body,
        (0..pairs).map(|index| input(index, 1, index)).collect(),
    )
}

fn high_fanout() -> StressWorld {
    let sources = 8;
    let sinks = 512;
    let mut body = substrate(105, sources + sinks, sources * sinks);
    add_cells(&mut body, sources + sinks, |index| if index < sources { 1 } else { 1_000 });
    for source in 0..sources {
        for sink in 0..sinks {
            drive(&mut body, source, sources + sink, 1, 1);
        }
    }
    freeze(
        "high_fanout",
        body,
        (0..sources).map(|index| input(index, 1, index)).collect(),
    )
}

fn heavy_modulation() -> StressWorld {
    let pairs = 512;
    let mut body = substrate(106, pairs * 2, pairs * 2);
    add_cells(&mut body, pairs * 2, |_| 1);
    for index in 0..pairs {
        drive(&mut body, index, pairs + index, 1, 1);
        modulation(&mut body, pairs + index, index, 1);
    }
    freeze(
        "heavy_modulation",
        body,
        (0..pairs).map(|index| input(index, 1, index)).collect(),
    )
}

fn mostly_dormant() -> StressWorld {
    let cells = 2_048;
    let fanout = 8;
    let arrows = cells * fanout;
    let mut body = substrate(107, cells, arrows);
    add_cells(&mut body, cells, |index| if index < 4 { 1 } else { 1_000 });
    for source in 0..cells {
        for offset in 1..=fanout {
            drive(&mut body, source, (source + offset) % cells, 1, 0);
        }
    }
    freeze(
        "mostly_dormant",
        body,
        (0..4).map(|index| input(index, 1, index)).collect(),
    )
}

fn zero_delay_fallback() -> StressWorld {
    let pairs = 512;
    let mut body = substrate(108, pairs * 2, pairs);
    add_cells(&mut body, pairs * 2, |index| if index < pairs { 1 } else { 1_000 });
    for index in 0..pairs {
        drive(&mut body, index, pairs + index, 0, 1);
    }
    freeze(
        "zero_delay_fallback",
        body,
        (0..pairs).map(|index| input(index, 1, index)).collect(),
    )
}
