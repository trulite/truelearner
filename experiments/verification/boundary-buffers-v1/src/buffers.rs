use pxr0_physical_runtime::{
    ArenaId, ArrowSpec, BoundaryError, BoundaryLiveCheckpoint, BoundaryRuntime, CellId, CellSpec,
    PlasticSubstrate, SpikeInput, TransmissionMode,
};

pub const CLAUSE_NAMES: [&str; 8] = [
    "atomic_input_backpressure",
    "transactional_output_backpressure",
    "oversize_output_rejected",
    "partial_drain_fifo",
    "direct_buffered_equivalence",
    "boundary_checkpoint_canonical",
    "boundary_checkpoint_continuation",
    "invalid_boundary_configuration_rejected",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BufferEvidence {
    pub clauses: [bool; 8],
    pub detail: String,
}

pub fn evaluate() -> BufferEvidence {
    let (substrate, source) = one_crossing_body();
    let mut input_limited = BoundaryRuntime::new(substrate.clone(), 1, 2, 4).unwrap();
    input_limited.enqueue(input(source, 0, 3)).unwrap();
    let input_before = input_limited.clone();
    let atomic_input_backpressure = matches!(
        input_limited.enqueue_batch(&[input(source, 1, 4), input(source, 2, 5)]),
        Err(BoundaryError::InputFull { .. })
    ) && input_limited == input_before;

    let mut output_limited = BoundaryRuntime::new(substrate.clone(), 1, 4, 1).unwrap();
    output_limited.enqueue(input(source, 0, 6)).unwrap();
    output_limited.run_until_quiescent().unwrap();
    output_limited.enqueue(input(source, 2, 7)).unwrap();
    let output_before = output_limited.clone();
    let transactional_output_backpressure = matches!(
        output_limited.run_until_quiescent(),
        Err(BoundaryError::OutputFull { .. })
    ) && output_limited == output_before;
    let first = output_limited.drain_output(1);
    output_limited.run_until_quiescent().unwrap();
    let second = output_limited.drain_output(1);
    let partial_drain_fifo = first.len() == 1 && second.len() == 1 && first[0].tick < second[0].tick;

    let (two_crossing, two_source) = two_crossing_body();
    let mut oversize = BoundaryRuntime::new(two_crossing, 1, 2, 1).unwrap();
    oversize.enqueue(input(two_source, 0, 8)).unwrap();
    let oversize_before = oversize.clone();
    let oversize_output_rejected = matches!(
        oversize.run_until_quiescent(),
        Err(BoundaryError::OutputBatchTooLarge {
            capacity: 1,
            required: 2
        })
    ) && oversize == oversize_before;

    let same_tick = [input(source, 0, 30), input(source, 0, 20)];
    let mut direct = substrate.clone();
    let direct_result = direct.arrive(&same_tick, 1);
    let mut buffered = BoundaryRuntime::new(substrate.clone(), 1, 4, 4).unwrap();
    let buffered_result = buffered.arrive(&same_tick, 1).unwrap();
    let direct_buffered_equivalence = direct_result == buffered_result && buffered.substrate() == &direct;

    let mut checkpointed = BoundaryRuntime::new(substrate, 1, 4, 4).unwrap();
    checkpointed.enqueue(input(source, 0, 40)).unwrap();
    checkpointed.run_until_quiescent().unwrap();
    checkpointed.enqueue(input(source, 2, 41)).unwrap();
    let checkpoint = checkpointed.live_checkpoint(100).unwrap();
    let bytes = checkpoint.canonical_bytes().unwrap();
    let decoded = BoundaryLiveCheckpoint::decode(&bytes).unwrap();
    let boundary_checkpoint_canonical = decoded.canonical_bytes().ok().as_deref() == Some(bytes.as_slice());
    let mut restored = BoundaryRuntime::from_live_checkpoint(decoded).unwrap();
    let exact_state = restored == checkpointed;
    let first_original = checkpointed.drain_all_output();
    let first_restored = restored.drain_all_output();
    let run_original = checkpointed.run_until_quiescent().unwrap();
    let run_restored = restored.run_until_quiescent().unwrap();
    let boundary_checkpoint_continuation = exact_state
        && first_original == first_restored
        && run_original == run_restored
        && checkpointed.drain_all_output() == restored.drain_all_output()
        && checkpointed == restored;

    let invalid_boundary_configuration_rejected =
        BoundaryRuntime::new(one_crossing_body().0, 1, 0, 1)
            == Err(BoundaryError::ZeroCapacity)
            && BoundaryRuntime::new(one_crossing_body().0, 1, 1, 0)
                == Err(BoundaryError::ZeroCapacity)
            && matches!(
                BoundaryRuntime::new(one_crossing_body().0, 1, 1, 1)
                    .unwrap()
                    .arrive(&[], 2),
                Err(BoundaryError::WrongOutwardRegion { .. })
            );

    let clauses = [
        atomic_input_backpressure,
        transactional_output_backpressure,
        oversize_output_rejected,
        partial_drain_fifo,
        direct_buffered_equivalence,
        boundary_checkpoint_canonical,
        boundary_checkpoint_continuation,
        invalid_boundary_configuration_rejected,
    ];
    BufferEvidence {
        clauses,
        detail: format!(
            "checkpoint_bytes={} input_capacity=2 output_capacity=1 first_tick={} second_tick={}",
            bytes.len(),
            first.first().map_or(-1, |crossing| crossing.tick),
            second.first().map_or(-1, |crossing| crossing.tick)
        ),
    }
}

fn one_crossing_body() -> (PlasticSubstrate, CellId) {
    let mut substrate = PlasticSubstrate::with_capacity(ArenaId(7_100), 4, 4);
    let source = substrate.add_cell(cell(7_101, 0, 0));
    let target = substrate.add_cell(cell(7_102, 1, 1));
    substrate.add_arrow(arrow(source, target));
    (substrate, source)
}

fn two_crossing_body() -> (PlasticSubstrate, CellId) {
    let mut substrate = PlasticSubstrate::with_capacity(ArenaId(7_200), 4, 4);
    let source = substrate.add_cell(cell(7_201, 0, 0));
    for (physical, position) in [(7_202, 1), (7_203, 2)] {
        let target = substrate.add_cell(cell(physical, position, 1));
        substrate.add_arrow(arrow(source, target));
    }
    (substrate, source)
}

fn cell(physical_id: u64, position: i32, region: i16) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold: 1,
        resistance: 10,
    }
}

fn arrow(from: CellId, to: CellId) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance: 50,
        mode: TransmissionMode::Drive,
    }
}

fn input(target: CellId, arrival_tick: i64, origin_physical: u64) -> SpikeInput {
    SpikeInput {
        arrival_tick,
        phase: 0,
        origin_physical,
        target,
        impulse: 1,
    }
}
