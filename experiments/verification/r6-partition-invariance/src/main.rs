#![forbid(unsafe_code)]

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use truelearner_core::{
    ArenaId, ArrowSpec, CellId, CellSpec, Crossing, ExecutionCost, MechanicalConfig, PhysicalClock,
    PhysicalTransition, PlasticSubstrate, ResidentArenaId, SpikeInput, TransmissionMode, Work,
};

const OUTWARD_REGION: i16 = 1;

#[derive(Clone)]
struct World {
    name: &'static str,
    body: Vec<u8>,
    cell_count: usize,
    inputs: Vec<SpikeInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Physics {
    pending_before: Vec<u8>,
    crossings: Vec<Crossing>,
    trace: Vec<PhysicalTransition>,
    work: [u64; 5],
    clock: PhysicalClock,
    pressure_phase: i64,
    body: Vec<u8>,
    quiescent: bool,
}

#[derive(Clone, Copy)]
enum Partition {
    TwoContiguous,
    FourContiguous,
    Striped,
    DeterministicRandom,
    Adversarial,
    Aggressive,
}

impl Partition {
    const ALL: [Self; 6] = [
        Self::TwoContiguous,
        Self::FourContiguous,
        Self::Striped,
        Self::DeterministicRandom,
        Self::Adversarial,
        Self::Aggressive,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::TwoContiguous => "P1_two_contiguous",
            Self::FourContiguous => "P2_four_contiguous",
            Self::Striped => "P3_striped",
            Self::DeterministicRandom => "P4_deterministic_random",
            Self::Adversarial => "P5_adversarial",
            Self::Aggressive => "P6_aggressive",
        }
    }
}

fn main() {
    let output = output_directory();
    let worlds = worlds();
    let mut rows = String::from(
        "world,partition,resident_arenas,arena_hops,arena_lookups,active_arena_samples,active_arena_total,active_arena_max,peak_resident_bytes,queue_ops,comparisons,scans,allocations,bytes_touched,passed\n",
    );
    let mut comparisons = 0usize;

    for world in &worlds {
        let (reference, _) = run(world, None);
        let (replay, _) = run(world, None);
        assert_eq!(reference, replay, "one-arena replay: {}", world.name);
        for partition in Partition::ALL {
            let placements = placements(partition, world.cell_count);
            let (candidate, cost) = run(world, Some(&placements));
            let (candidate_replay, replay_cost) = run(world, Some(&placements));
            assert_eq!(candidate, candidate_replay, "partition replay");
            assert_eq!(cost, replay_cost, "mechanical replay");
            assert_eq!(reference, candidate, "{} {}", world.name, partition.name());
            write_row(
                &mut rows,
                world.name,
                partition.name(),
                distinct_count(&placements),
                cost,
            );
            comparisons += 1;
        }
    }

    assert!(quiescent_repartition_control());
    assert!(live_repartition_control());

    fs::create_dir_all(&output).expect("create R6 output directory");
    fs::write(output.join("r6_partition_matrix.csv"), rows).expect("write R6 matrix");
    let clauses = comparisons + 2;
    let report = format!(
        "# R6 Partition Invariance Result\n\n- worlds: `{}`\n- nontrivial partitions per world: `6`\n- partition comparisons: `{comparisons}/{comparisons}`\n- checkpoint controls: `2/2`\n- total clauses: `{clauses}/{clauses}`\n- exact replay: `true`\n- natural quiescence: `true`\n- added inter-arena latency: `0`\n",
        worlds.len(),
    );
    fs::write(output.join("r6_partition_report.md"), report).expect("write R6 report");
    println!(
        "R6_PARTITION_INVARIANCE_PASS worlds={} comparisons={comparisons} checkpoint_controls=2/2 clauses={clauses}/{clauses} output={}",
        worlds.len(),
        output.display()
    );
}

fn output_directory() -> PathBuf {
    let mut args = std::env::args().skip(1);
    match (args.next(), args.next(), args.next()) {
        (Some(flag), Some(path), None) if flag == "--output" => PathBuf::from(path),
        (None, None, None) => PathBuf::from("results/r6_partition_invariance_v1"),
        _ => panic!("usage: r6-partition-invariance [--output DIRECTORY]"),
    }
}

fn run(world: &World, partition: Option<&[ResidentArenaId]>) -> (Physics, ExecutionCost) {
    let mut body = PlasticSubstrate::from_body_bytes(&world.body).expect("decode R6 body");
    body.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
    if let Some(partition) = partition {
        body.repartition_resident(partition);
    }
    body.set_physical_tracing(true);
    for input in &world.inputs {
        body.enter(*input);
    }
    let pending_before = body
        .live_checkpoint(600)
        .expect("live checkpoint before R6 run")
        .canonical_bytes()
        .expect("canonical pending activity");
    let result = body.propagate();
    let physics = Physics {
        pending_before,
        crossings: result.crossings,
        trace: result.physical_trace,
        work: physical_work(result.work),
        clock: body.clock(),
        pressure_phase: body.clock().pressure_phase(),
        body: body
            .canonical_body_bytes(601)
            .expect("canonical final R6 body"),
        quiescent: result.naturally_quiescent,
    };
    assert!(physics.quiescent);
    (physics, result.execution_cost)
}

fn physical_work(work: Work) -> [u64; 5] {
    [
        work.drive_deliveries,
        work.modulatory_deliveries,
        work.local_return_updates,
        work.local_structural_proposals,
        work.physical_deallocations,
    ]
}

fn write_row(
    rows: &mut String,
    world: &str,
    partition: &str,
    resident_arenas: usize,
    cost: ExecutionCost,
) {
    writeln!(
        rows,
        "{world},{partition},{resident_arenas},{},{},{},{},{},{},{},{},{},{},{},true",
        cost.arena_hops,
        cost.arena_lookups,
        cost.active_arena_samples,
        cost.active_arena_total,
        cost.active_arena_max,
        cost.peak_resident_bytes,
        cost.queue_ops,
        cost.comparisons,
        cost.scans,
        cost.allocations,
        cost.bytes_touched,
    )
    .unwrap();
}

fn placements(partition: Partition, count: usize) -> Vec<ResidentArenaId> {
    (0..count)
        .map(|index| {
            let arena = match partition {
                Partition::TwoContiguous => index.saturating_mul(2) / count.max(1),
                Partition::FourContiguous => index.saturating_mul(4) / count.max(1),
                Partition::Striped => index % 4,
                Partition::DeterministicRandom => {
                    index
                        .wrapping_mul(2_654_435_761)
                        .wrapping_add(1_013_904_223)
                        % 7
                }
                Partition::Adversarial => (index / 2) % 2,
                Partition::Aggressive => index % count.min(128).max(1),
            };
            ResidentArenaId(u32::try_from(arena).unwrap())
        })
        .collect()
}

fn distinct_count(values: &[ResidentArenaId]) -> usize {
    let mut values = values.to_vec();
    values.sort_unstable();
    values.dedup();
    values.len()
}

fn worlds() -> Vec<World> {
    vec![
        global_order_world(),
        dense_layers_world(),
        modulation_world(),
        long_delay_world(),
        zero_delay_world(),
        lifecycle_world(),
    ]
}

fn body(arena: u64, cells: usize, arrows: usize) -> PlasticSubstrate {
    PlasticSubstrate::with_capacity(
        ArenaId(arena),
        u32::try_from(cells + 32).unwrap(),
        u32::try_from(arrows + 64).unwrap(),
    )
}

fn add_cells(
    body: &mut PlasticSubstrate,
    count: usize,
    threshold: impl Fn(usize) -> i32,
    region: impl Fn(usize) -> i16,
    position: impl Fn(usize) -> i32,
) {
    for index in 0..count {
        body.add_cell(CellSpec {
            physical_id: 8_000_000 + u64::try_from(count - index).unwrap(),
            position: position(index),
            region: region(index),
            threshold: threshold(index),
            resistance: 1_000,
        });
    }
}

fn arrow(
    body: &mut PlasticSubstrate,
    from: usize,
    to: usize,
    delay: i64,
    coupling: i32,
    mode: TransmissionMode,
) {
    body.add_arrow(ArrowSpec {
        from: CellId(from as u64),
        to: CellId(to as u64),
        delay,
        phase: if mode == TransmissionMode::Drive {
            0
        } else {
            1
        },
        coupling,
        resistance: 1_000,
        mode,
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

fn freeze(name: &'static str, body: PlasticSubstrate, inputs: Vec<SpikeInput>) -> World {
    let cell_count = body.arena_body(1).cells.len();
    World {
        name,
        body: body.canonical_body_bytes(1).expect("freeze R6 body"),
        cell_count,
        inputs,
    }
}

fn global_order_world() -> World {
    let pairs = 64;
    let mut body = body(201, pairs * 2, pairs);
    add_cells(
        &mut body,
        pairs * 2,
        |index| if index < pairs { 1 } else { 1_000 },
        |_| 0,
        |index| i32::try_from(index).unwrap() * 10,
    );
    for index in 0..pairs {
        arrow(
            &mut body,
            index,
            pairs + (pairs - index - 1),
            1,
            1,
            TransmissionMode::Drive,
        );
    }
    freeze(
        "global_order",
        body,
        (0..pairs).map(|index| input(index, 1, index)).collect(),
    )
}

fn dense_layers_world() -> World {
    let width = 32;
    let layers = 4;
    let cells = width * layers;
    let mut body = body(202, cells, width * (layers - 1) * 4);
    add_cells(
        &mut body,
        cells,
        |_| 1,
        |index| if index >= width * 3 { 1 } else { 0 },
        |index| i32::try_from(index).unwrap() * 10,
    );
    for layer in 0..layers - 1 {
        for source in 0..width {
            for offset in 0..4 {
                arrow(
                    &mut body,
                    layer * width + source,
                    (layer + 1) * width + (source + offset) % width,
                    1,
                    1,
                    TransmissionMode::Drive,
                );
            }
        }
    }
    freeze(
        "dense_layers",
        body,
        (0..width).map(|index| input(index, 1, index)).collect(),
    )
}

fn modulation_world() -> World {
    let pairs = 64;
    let mut body = body(203, pairs * 2, pairs * 2);
    add_cells(
        &mut body,
        pairs * 2,
        |_| 1,
        |_| 0,
        |index| i32::try_from(index).unwrap() * 10,
    );
    for index in 0..pairs {
        arrow(
            &mut body,
            index,
            pairs + index,
            1,
            1,
            TransmissionMode::Drive,
        );
        arrow(
            &mut body,
            pairs + index,
            index,
            1,
            1,
            TransmissionMode::Modulatory,
        );
    }
    freeze(
        "modulation",
        body,
        (0..pairs).map(|index| input(index, 1, index)).collect(),
    )
}

fn long_delay_world() -> World {
    let pairs = 64;
    let mut body = body(204, pairs * 2, pairs);
    add_cells(
        &mut body,
        pairs * 2,
        |index| if index < pairs { 1 } else { 1_000 },
        |_| 0,
        |index| i32::try_from(index).unwrap() * 10,
    );
    for index in 0..pairs {
        arrow(
            &mut body,
            index,
            pairs + index,
            65 + (index % 32) as i64,
            1,
            TransmissionMode::Drive,
        );
    }
    freeze(
        "long_delay",
        body,
        (0..pairs).map(|index| input(index, 1, index)).collect(),
    )
}

fn zero_delay_world() -> World {
    let pairs = 32;
    let mut body = body(205, pairs * 2, pairs);
    add_cells(
        &mut body,
        pairs * 2,
        |index| if index < pairs { 1 } else { 1_000 },
        |_| 0,
        |index| i32::try_from(index).unwrap() * 10,
    );
    for index in 0..pairs {
        arrow(
            &mut body,
            index,
            pairs + index,
            0,
            1,
            TransmissionMode::Drive,
        );
    }
    freeze(
        "zero_delay",
        body,
        (0..pairs).map(|index| input(index, 1, index)).collect(),
    )
}

fn lifecycle_world() -> World {
    let cells = 8;
    let mut body = body(206, cells, 64);
    add_cells(
        &mut body,
        cells,
        |_| 1,
        |_| 0,
        |index| i32::try_from(index).unwrap(),
    );
    freeze(
        "proposal_lifecycle",
        body,
        vec![input(0, 1, 0), input(0, 20, 1)],
    )
}

fn quiescent_repartition_control() -> bool {
    let world = global_order_world();
    let mut original = PlasticSubstrate::from_body_bytes(&world.body).unwrap();
    original.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
    let first = input(0, 1, 0);
    original.arrive(&[first], OUTWARD_REGION);
    original.advance_time(10);
    let checkpoint = original.quiescent_checkpoint(700).unwrap();
    let mut baseline = PlasticSubstrate::from_quiescent_checkpoint(checkpoint.clone()).unwrap();
    baseline.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
    let mut partitioned = PlasticSubstrate::from_quiescent_checkpoint(checkpoint).unwrap();
    partitioned.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
    partitioned.repartition_resident(&placements(Partition::Striped, world.cell_count));
    let future = input(1, 11, 1);
    let baseline_result = baseline.arrive(&[future], OUTWARD_REGION);
    let partitioned_result = partitioned.arrive(&[future], OUTWARD_REGION);
    same_physics(
        &baseline,
        &baseline_result,
        &partitioned,
        &partitioned_result,
    )
}

fn live_repartition_control() -> bool {
    let world = global_order_world();
    let mut source = PlasticSubstrate::from_body_bytes(&world.body).unwrap();
    source.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
    for input in &world.inputs {
        source.enter(*input);
    }
    let checkpoint = source.live_checkpoint(701).unwrap();
    let mut baseline = PlasticSubstrate::from_live_checkpoint(checkpoint.clone()).unwrap();
    baseline.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
    let mut partitioned = PlasticSubstrate::from_live_checkpoint(checkpoint).unwrap();
    partitioned.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
    partitioned.repartition_resident(&placements(
        Partition::DeterministicRandom,
        world.cell_count,
    ));
    let baseline_result = baseline.propagate();
    let partitioned_result = partitioned.propagate();
    same_physics(
        &baseline,
        &baseline_result,
        &partitioned,
        &partitioned_result,
    )
}

fn same_physics(
    left: &PlasticSubstrate,
    left_result: &truelearner_core::RunResult,
    right: &PlasticSubstrate,
    right_result: &truelearner_core::RunResult,
) -> bool {
    left_result.crossings == right_result.crossings
        && physical_work(left_result.work) == physical_work(right_result.work)
        && left.clock() == right.clock()
        && left.clock().pressure_phase() == right.clock().pressure_phase()
        && left.canonical_body_bytes(702).ok() == right.canonical_body_bytes(702).ok()
        && left_result.naturally_quiescent == right_result.naturally_quiescent
}
