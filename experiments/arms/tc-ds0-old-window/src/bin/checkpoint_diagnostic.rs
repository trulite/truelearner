#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowSpec, ContentHash, MechanicalConfig, PlasticSubstrate, SpikeInput,
    TransmissionMode,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct CellRuntime {
    id: u64,
    state: i32,
    last_update_tick: i64,
    refractory_until: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArrowRuntime {
    id: u64,
    eligible_until: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DecodedCheckpoint {
    tick: i64,
    next_serial: u64,
    manifest_len: usize,
    body_len: usize,
    cell_count: usize,
    arrow_count: usize,
    pending_count: usize,
    load_count: usize,
    cells: Vec<CellRuntime>,
    arrows: Vec<ArrowRuntime>,
}

#[derive(Clone)]
struct Run {
    checkpoint: Vec<u8>,
    decoded: DecodedCheckpoint,
    body_hash: String,
    trace_hash: String,
    replay_exact: bool,
    continuation_hash: String,
    continuation_body_hash: String,
    continuation_equal_fields: String,
}

fn input(target: truelearner_core::CellId, tick: i64, origin: u64, impulse: i32) -> SpikeInput {
    SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target,
        impulse,
    }
}

fn add_cell(
    body: &mut PlasticSubstrate,
    physical_id: u64,
    position: i32,
    threshold: i32,
) -> truelearner_core::CellId {
    body.add_cell(truelearner_core::CellSpec {
        physical_id,
        position,
        region: 0,
        threshold,
        resistance: 100,
    })
}

fn build(mechanics: MechanicalConfig) -> (PlasticSubstrate, [truelearner_core::CellId; 7]) {
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(901_000), 12, 16, mechanics);
    body.set_physical_tracing(true);
    let cells = [
        add_cell(&mut body, 10_001, 0, 1),
        add_cell(&mut body, 10_002, 10, 2),
        add_cell(&mut body, 10_003, 20, 2),
        add_cell(&mut body, 10_004, 30, 1),
        add_cell(&mut body, 10_005, 40, 2),
        add_cell(&mut body, 10_006, 50, 1),
        add_cell(&mut body, 10_007, 60, 2),
    ];
    body.add_arrow(ArrowSpec {
        from: cells[0],
        to: cells[1],
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: 1,
        mode: TransmissionMode::Drive,
    });
    body.add_arrow(ArrowSpec {
        from: cells[1],
        to: cells[0],
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: 100,
        mode: TransmissionMode::Modulatory,
    });
    (body, cells)
}

fn execute(mechanics: MechanicalConfig) -> Run {
    let (mut body, cells) = build(mechanics);
    let initial = body.arrive(&[input(cells[0], 0, 80_001, 1)], 99);
    let event = body.arrive(&[input(cells[1], 0, 80_002, 2)], 99);
    let mut trace = initial.physical_trace;
    trace.extend(event.physical_trace);
    for tick in 1..=15 {
        body.advance_time(tick);
    }
    let body_hash = ContentHash::of(&body.canonical_body_bytes(1).unwrap()).to_string();
    let trace_hash = ContentHash::of(format!("{trace:?}").as_bytes()).to_string();
    let checkpoint = body.live_checkpoint(1).unwrap().canonical_bytes().unwrap();
    let decoded = decode(&checkpoint);
    let mut restored = PlasticSubstrate::from_live_checkpoint_with_mechanics(
        truelearner_core::LiveCheckpoint::decode(&checkpoint).unwrap(),
        mechanics,
    )
    .unwrap();
    restored.set_physical_tracing(true);
    let replay_exact = restored
        .live_checkpoint(1)
        .unwrap()
        .canonical_bytes()
        .unwrap()
        == checkpoint;
    let continuation = restored.arrive(
        &[
            input(cells[0], 16, 90_001, 1),
            input(cells[1], 16, 90_002, 2),
            input(cells[2], 16, 90_003, 2),
            input(cells[3], 16, 90_004, 1),
            input(cells[5], 16, 90_005, 1),
        ],
        99,
    );
    let continuation_text = format!(
        "crossings={:?};trace={:?};drive={};mod={};updates={};proposals={};deallocations={};tick={};phase={};quiescent={}",
        continuation.crossings,
        continuation.physical_trace,
        continuation.work.drive_deliveries,
        continuation.work.modulatory_deliveries,
        continuation.work.local_return_updates,
        continuation.work.local_structural_proposals,
        continuation.work.physical_deallocations,
        restored.clock().tick,
        restored.clock().pressure_phase(),
        continuation.naturally_quiescent,
    );
    Run {
        checkpoint,
        decoded,
        body_hash,
        trace_hash,
        replay_exact,
        continuation_hash: ContentHash::of(continuation_text.as_bytes()).to_string(),
        continuation_body_hash: ContentHash::of(&restored.canonical_body_bytes(2).unwrap())
            .to_string(),
        continuation_equal_fields: continuation_text,
    }
}

fn read_u16(bytes: &[u8], offset: &mut usize) -> u16 {
    let value = u16::from_le_bytes(bytes[*offset..*offset + 2].try_into().unwrap());
    *offset += 2;
    value
}

fn read_u32(bytes: &[u8], offset: &mut usize) -> u32 {
    let value = u32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    value
}

fn read_i32(bytes: &[u8], offset: &mut usize) -> i32 {
    let value = i32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    value
}

fn read_u64(bytes: &[u8], offset: &mut usize) -> u64 {
    let value = u64::from_le_bytes(bytes[*offset..*offset + 8].try_into().unwrap());
    *offset += 8;
    value
}

fn read_i64(bytes: &[u8], offset: &mut usize) -> i64 {
    let value = i64::from_le_bytes(bytes[*offset..*offset + 8].try_into().unwrap());
    *offset += 8;
    value
}

fn decode(bytes: &[u8]) -> DecodedCheckpoint {
    assert_eq!(&bytes[..8], b"TLLIVE01");
    let mut offset = 8;
    assert_eq!(read_u16(bytes, &mut offset), 1);
    let tick = read_i64(bytes, &mut offset);
    let next_serial = read_u64(bytes, &mut offset);
    let manifest_len = usize::try_from(read_u64(bytes, &mut offset)).unwrap();
    let body_len = usize::try_from(read_u64(bytes, &mut offset)).unwrap();
    let cell_count = usize::try_from(read_u32(bytes, &mut offset)).unwrap();
    let arrow_count = usize::try_from(read_u32(bytes, &mut offset)).unwrap();
    let pending_count = usize::try_from(read_u32(bytes, &mut offset)).unwrap();
    let load_count = usize::try_from(read_u32(bytes, &mut offset)).unwrap();
    let _payload_len = read_u64(bytes, &mut offset);
    offset += 32;
    offset += manifest_len + body_len;
    let mut cells = Vec::with_capacity(cell_count);
    for _ in 0..cell_count {
        cells.push(CellRuntime {
            id: read_u64(bytes, &mut offset),
            state: read_i32(bytes, &mut offset),
            last_update_tick: read_i64(bytes, &mut offset),
            refractory_until: read_i64(bytes, &mut offset),
        });
    }
    let mut arrows = Vec::with_capacity(arrow_count);
    for _ in 0..arrow_count {
        let id = read_u64(bytes, &mut offset);
        let present = bytes[offset];
        offset += 1;
        let tick = read_i64(bytes, &mut offset);
        arrows.push(ArrowRuntime {
            id,
            eligible_until: (present == 1).then_some(tick),
        });
    }
    DecodedCheckpoint {
        tick,
        next_serial,
        manifest_len,
        body_len,
        cell_count,
        arrow_count,
        pending_count,
        load_count,
        cells,
        arrows,
    }
}

fn first_difference(left: &[u8], right: &[u8], start: usize) -> Option<usize> {
    left.iter()
        .zip(right)
        .enumerate()
        .skip(start)
        .find_map(|(index, (a, b))| (a != b).then_some(index))
        .or_else(|| (left.len() != right.len()).then_some(left.len().min(right.len())))
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results/tc_ds0_checkpoint_diagnostic_v1"));
    fs::create_dir_all(&output).unwrap();
    let reference = execute(MechanicalConfig::REFERENCE);
    let production = execute(MechanicalConfig::PRODUCTION);
    let first_any = first_difference(&reference.checkpoint, &production.checkpoint, 0).unwrap();
    let first_payload = first_difference(&reference.checkpoint, &production.checkpoint, 98).unwrap();

    let mut csv = String::from(
        "cell_id,reference_state,production_state,reference_last_update,production_last_update,reference_refractory,production_refractory,equal\n",
    );
    let mut differences = Vec::new();
    for (left, right) in reference
        .decoded
        .cells
        .iter()
        .zip(&production.decoded.cells)
    {
        assert_eq!(left.id, right.id);
        let equal = left == right;
        if !equal {
            differences.push(format!(
                "CELL {}: state {}|{}, last_update {}|{}, refractory {}|{}",
                left.id,
                left.state,
                right.state,
                left.last_update_tick,
                right.last_update_tick,
                left.refractory_until,
                right.refractory_until
            ));
        }
        writeln!(
            csv,
            "{},{},{},{},{},{},{},{}",
            left.id,
            left.state,
            right.state,
            left.last_update_tick,
            right.last_update_tick,
            left.refractory_until,
            right.refractory_until,
            equal
        )
        .unwrap();
    }
    assert_eq!(reference.decoded.arrows, production.decoded.arrows);
    assert_eq!(reference.body_hash, production.body_hash);
    assert_eq!(reference.trace_hash, production.trace_hash);
    assert_eq!(
        reference.continuation_equal_fields,
        production.continuation_equal_fields
    );
    assert_eq!(
        reference.continuation_body_hash,
        production.continuation_body_hash
    );

    let report = format!(
        "# TC-DS0 checkpoint-negative diagnostic v1\n\n\
         - first differing byte including checksum: `{first_any}`\n\
         - first differing payload byte: `{first_payload}`\n\
         - decoded differing fields: `{}`\n\
         - header reference: `{:?}`\n\
         - header production: `{:?}`\n\
         - ARROW runtime equal: `true`\n\
         - durable-body hash equal: `{}`\n\
         - physical-transition hash equal: `{}`\n\
         - independent replay: `{}/{}`\n\
         - identical future causal continuation: `true`\n\
         - continuation hash: `{}`\n\
         - continuation durable-body hash: `{}`\n\n\
         Differing fields:\n\n{}\n",
        differences.len(),
        reference.decoded,
        production.decoded,
        reference.body_hash == production.body_hash,
        reference.trace_hash == production.trace_hash,
        reference.replay_exact,
        production.replay_exact,
        reference.continuation_hash,
        reference.continuation_body_hash,
        differences
            .iter()
            .map(|value| format!("- {value}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    fs::write(output.join("cells.csv"), csv).unwrap();
    fs::write(output.join("report.md"), report).unwrap();
    println!(
        "TC_DS0_CHECKPOINT_DIAGNOSTIC_COMPLETE differences={} continuation_equal=true",
        differences.len()
    );
}
