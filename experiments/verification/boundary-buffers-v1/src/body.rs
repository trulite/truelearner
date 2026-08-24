use pxr0_physical_runtime::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, CheckpointError, ContentHash, LiveCheckpoint,
    PendingLoad, PlasticSubstrate, QuiescentCheckpoint, RunResult, SpikeInput, TransmissionMode,
    Work,
};
use std::panic::{catch_unwind, AssertUnwindSafe};
use truelearner_arena_format::{ArenaBody, ArenaVersion, BodyVersion, Generation};

pub const CLAUSE_NAMES: [&str; 16] = [
    "canonical_arena_round_trip",
    "equivalent_arena_hash",
    "canonical_manifest_hash",
    "cell_reference_compaction",
    "arrow_reference_compaction",
    "compaction_behavior",
    "quiescent_checkpoint_round_trip",
    "quiescent_clock_phase",
    "quiescent_future_behavior",
    "live_checkpoint_round_trip",
    "live_pending_continuation",
    "bounded_capacity",
    "deterministic_reuse_generation",
    "stale_reference_rejected",
    "corrupt_arena_rejected",
    "stale_durable_reference_rejected",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyEvidence {
    pub clauses: [bool; 16],
    pub detail: String,
}

pub fn evaluate() -> BodyEvidence {
    match catch_unwind(AssertUnwindSafe(evaluate_inner)) {
        Ok(evidence) => evidence,
        Err(_) => BodyEvidence {
            clauses: [false; 16],
            detail: "authority body evaluation panicked".to_owned(),
        },
    }
}

fn evaluate_inner() -> BodyEvidence {
    let (original, source, _, arrow) = substrate(50);
    let body = original.arena_body(91);
    let body_bytes = body.canonical_bytes().expect("frozen body must encode");
    let decoded_body = ArenaBody::decode(&body_bytes).expect("frozen body must decode");
    let encoded_again = decoded_body
        .canonical_bytes()
        .expect("decoded body must encode");
    let canonical_arena_round_trip = body_bytes == encoded_again;
    let equivalent_arena_hash = body.content_hash().ok() == decoded_body.content_hash().ok()
        && body.content_hash().ok() == Some(ContentHash::of(&body_bytes));

    let first = ArenaVersion {
        arena: ArenaId(84_100_009),
        block: ContentHash([9; 32]),
    };
    let second = ArenaVersion {
        arena: ArenaId(84_100_002),
        block: ContentHash([2; 32]),
    };
    let manifest = BodyVersion {
        version: 92,
        parent: Some(ContentHash([1; 32])),
        arenas: vec![first, second],
    };
    let manifest_bytes = manifest
        .canonical_bytes()
        .expect("frozen manifest must encode");
    let decoded_manifest = BodyVersion::decode(&manifest_bytes).expect("manifest must decode");
    let canonical_manifest_hash = decoded_manifest.arenas == vec![second, first]
        && decoded_manifest.canonical_bytes().ok().as_deref() == Some(manifest_bytes.as_slice())
        && decoded_manifest.content_hash().ok() == Some(ContentHash::of(&manifest_bytes));

    let cell_reference = original.cell_reference(source);
    let arrow_reference = original.arrow_reference(arrow);
    let cell_slot_before = original.resolve_cell(cell_reference);
    let arrow_slot_before = original.resolve_arrow(arrow_reference);
    let mut compacted = original.clone();
    compacted.compact_resident();
    let cell_slot_after = compacted.resolve_cell(cell_reference);
    let arrow_slot_after = compacted.resolve_arrow(arrow_reference);
    let cell_reference_compaction = cell_slot_before.is_some()
        && cell_slot_after.is_some()
        && cell_slot_before != cell_slot_after
        && compacted.cell_reference(source) == cell_reference;
    let arrow_reference_compaction = arrow_slot_before.is_some()
        && arrow_slot_after.is_some()
        && compacted.arrow_reference(arrow) == arrow_reference;
    let mut ordinary = original.clone();
    let ordinary_result = ordinary.arrive(&[input(source, 0)], 1);
    let compacted_result = compacted.arrive(&[input(source, 0)], 1);
    let compaction_behavior = same_physics(
        &ordinary,
        &ordinary_result,
        &compacted,
        &compacted_result,
    );

    let (mut quiet_source, quiet_cell, _, _) = substrate(50);
    quiet_source.advance_time(23);
    let quiet_checkpoint = quiet_source
        .quiescent_checkpoint(93)
        .expect("settled body must checkpoint");
    let quiet_bytes = quiet_checkpoint
        .canonical_bytes()
        .expect("quiescent checkpoint must encode");
    let quiet_decoded =
        QuiescentCheckpoint::decode(&quiet_bytes).expect("quiescent checkpoint must decode");
    let quiescent_checkpoint_round_trip =
        quiet_decoded.canonical_bytes().ok().as_deref() == Some(quiet_bytes.as_slice());
    let mut quiet_restored = PlasticSubstrate::from_quiescent_checkpoint(quiet_decoded)
        .expect("quiescent checkpoint must restore");
    let quiescent_clock_phase = quiet_restored.clock() == quiet_source.clock()
        && quiet_restored.clock().pressure_phase() == 3;
    let quiet_source_result = quiet_source.arrive(&[input(quiet_cell, 24)], 1);
    let quiet_restored_result = quiet_restored.arrive(&[input(quiet_cell, 24)], 1);
    let quiescent_future_behavior = same_physics(
        &quiet_source,
        &quiet_source_result,
        &quiet_restored,
        &quiet_restored_result,
    );

    let (mut live_source, live_cell, _, _) = substrate(50);
    live_source.enter(input(live_cell, 5));
    live_source.register_pending_load(PendingLoad {
        arena: ArenaId(84_100_099),
        version: ContentHash([3; 32]),
        issue_tick: 0,
        availability_tick: Some(7),
        waiting_arrivals: vec![input(live_cell, 8)],
    });
    let live_checkpoint = live_source
        .live_checkpoint(94)
        .expect("live body must checkpoint");
    let live_bytes = live_checkpoint
        .canonical_bytes()
        .expect("live checkpoint must encode");
    let live_decoded = LiveCheckpoint::decode(&live_bytes).expect("live checkpoint must decode");
    let live_checkpoint_round_trip =
        live_decoded.canonical_bytes().ok().as_deref() == Some(live_bytes.as_slice());
    let mut live_restored =
        PlasticSubstrate::from_live_checkpoint(live_decoded).expect("live checkpoint must restore");
    let live_checkpoint_state_equal = live_restored == live_source;
    let live_restored_result = live_restored.propagate();
    let live_source_result = live_source.propagate();
    let live_pending_continuation = live_checkpoint_state_equal
        && same_physics(
            &live_source,
            &live_source_result,
            &live_restored,
            &live_restored_result,
        );

    let cell_overflow = catch_unwind(AssertUnwindSafe(|| {
        let mut limited = PlasticSubstrate::with_capacity(ArenaId(84_100_200), 1, 1);
        limited.add_cell(cell_spec(84_100_201, 0, 0));
        limited.add_cell(cell_spec(84_100_202, 10, 0));
    }))
    .is_err();
    let arrow_overflow = catch_unwind(AssertUnwindSafe(|| {
        let mut limited = PlasticSubstrate::with_capacity(ArenaId(84_100_300), 2, 1);
        let from = limited.add_cell(cell_spec(84_100_301, 0, 0));
        let to = limited.add_cell(cell_spec(84_100_302, 10, 0));
        limited.add_arrow(arrow_spec(from, to, 4));
        limited.add_arrow(arrow_spec(from, to, 4));
    }))
    .is_err();
    let bounded_capacity = cell_overflow && arrow_overflow;

    let (mut reusable, reuse_source, reuse_target, reuse_arrow) = substrate(1);
    let stale = reusable.arrow_reference(reuse_arrow);
    reusable.advance_time(10);
    let stale_reference_rejected_before_reuse = reusable.resolve_arrow(stale).is_none();
    let reused = reusable.add_arrow(arrow_spec(reuse_source, reuse_target, 4));
    let current = reusable.arrow_reference(reused);
    let deterministic_reuse_generation =
        reused == reuse_arrow && current.generation != stale.generation;
    let stale_reference_rejected = stale_reference_rejected_before_reuse
        && reusable.resolve_arrow(stale).is_none()
        && reusable.resolve_arrow(current).is_some();

    let mut corrupt = body_bytes.clone();
    if let Some(last) = corrupt.last_mut() {
        *last ^= 1;
    }
    let truncated = &body_bytes[..body_bytes.len().saturating_sub(1)];
    let mut trailing = body_bytes.clone();
    trailing.push(0);
    let mut overlapping = body_bytes.clone();
    overlapping[68..76].copy_from_slice(&156_u64.to_le_bytes());
    let corrupt_arena_rejected = ArenaBody::decode(&corrupt).is_err()
        && ArenaBody::decode(truncated).is_err()
        && ArenaBody::decode(&trailing).is_err()
        && ArenaBody::decode(&overlapping).is_err();

    let mut stale_body = original.arena_body(95);
    stale_body.arrows[0].from.generation = Generation(99);
    let stale_body_bytes = stale_body
        .canonical_bytes()
        .expect("stale relation remains representable as durable bytes");
    let stale_durable_reference_rejected = matches!(
        PlasticSubstrate::from_body_bytes(&stale_body_bytes),
        Err(CheckpointError::StaleCellReference(_))
    );

    let clauses = [
        canonical_arena_round_trip,
        equivalent_arena_hash,
        canonical_manifest_hash,
        cell_reference_compaction,
        arrow_reference_compaction,
        compaction_behavior,
        quiescent_checkpoint_round_trip,
        quiescent_clock_phase,
        quiescent_future_behavior,
        live_checkpoint_round_trip,
        live_pending_continuation,
        bounded_capacity,
        deterministic_reuse_generation,
        stale_reference_rejected,
        corrupt_arena_rejected,
        stale_durable_reference_rejected,
    ];
    BodyEvidence {
        clauses,
        detail: format!(
            "arena_bytes={} manifest_bytes={} quiet_bytes={} live_bytes={} cell_slots={:?}->{:?} arrow_slots={:?}->{:?} stale_generation={} current_generation={}",
            body_bytes.len(),
            manifest_bytes.len(),
            quiet_bytes.len(),
            live_bytes.len(),
            cell_slot_before,
            cell_slot_after,
            arrow_slot_before,
            arrow_slot_after,
            stale.generation.0,
            current.generation.0,
        ),
    }
}

fn same_physics(
    left: &PlasticSubstrate,
    left_result: &RunResult,
    right: &PlasticSubstrate,
    right_result: &RunResult,
) -> bool {
    left_result.crossings == right_result.crossings
        && physical_work(left_result.work) == physical_work(right_result.work)
        && left.clock() == right.clock()
        && left.canonical_body_bytes(999).ok() == right.canonical_body_bytes(999).ok()
        && left_result.naturally_quiescent == right_result.naturally_quiescent
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

fn substrate(arrow_resistance: u32) -> (PlasticSubstrate, CellId, CellId, ArrowId) {
    let mut substrate = PlasticSubstrate::with_capacity(ArenaId(84_100_001), 8, 8);
    let source = substrate.add_cell(cell_spec(84_100_010, 10, 0));
    let target = substrate.add_cell(cell_spec(84_100_020, 20, 1));
    let arrow = substrate.add_arrow(arrow_spec(source, target, arrow_resistance));
    (substrate, source, target, arrow)
}

fn cell_spec(physical_id: u64, position: i32, region: i16) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold: 1,
        resistance: 10,
    }
}

fn arrow_spec(from: CellId, to: CellId, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance,
        mode: TransmissionMode::Drive,
    }
}

fn input(target: CellId, tick: i64) -> SpikeInput {
    SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 84_100_777,
        target,
        impulse: 1,
    }
}
