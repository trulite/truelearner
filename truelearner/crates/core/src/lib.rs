#![forbid(unsafe_code)]
//! Single-file physical runtime: state, local transitions, and crossings.

use std::collections::{BTreeSet, VecDeque};
use truelearner_arena_format::{
    ArenaBody, ArenaVersion, BodyVersion, DurableArrow, DurableCell, FormatError,
};

mod mechanics;
pub use mechanics::SchedulerKind;
use mechanics::{ArrowStore, CellStore, PendingSchedule};
pub use truelearner_arena_format::{
    ArenaId, ArrowId, ArrowRef, CellId, CellRef, ContentHash, Generation,
};

const LOCAL_RETURN_STRENGTH: u32 = 3;
const LOCAL_DECAY_PERIOD: i64 = 10;
const LOCAL_VARIATION_RADIUS: i32 = 2;
const PARTICIPATION_IMPULSE: u64 = 1_u64 << 32;
const PARTICIPATION_RELAX_NUMERATOR: u64 = 15;
const PARTICIPATION_RELAX_DENOMINATOR: u64 = 16;
const DEFAULT_CELL_CAPACITY: u32 = 65_536;
const DEFAULT_ARROW_CAPACITY: u32 = 262_144;
const CHECKPOINT_VERSION: u16 = 2;
const QUIESCENT_MAGIC: &[u8; 8] = b"TLQUIE01";
const LIVE_MAGIC: &[u8; 8] = b"TLLIVE01";
const BOUNDARY_LIVE_MAGIC: &[u8; 8] = b"TLBNDY01";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CellSlot(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArrowSlot(pub usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResidentArenaId(pub u32);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhysicalClock {
    pub tick: i64,
}

impl PhysicalClock {
    pub fn pressure_phase(self) -> i64 {
        self.tick.rem_euclid(LOCAL_DECAY_PERIOD)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellSpec {
    pub physical_id: u64,
    pub position: i32,
    pub region: i16,
    pub threshold: i32,
    pub resistance: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransmissionMode {
    Drive,
    Modulatory,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TransmissionTrigger {
    #[default]
    SourceFires,
    QualifiedLocalParticipation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraversalKind {
    GlobalScan,
    Adjacency,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityKind {
    FullScan,
    Frontier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutKind {
    AoS,
    SoA,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutorKind {
    Scalar,
    Batched,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MechanicalConfig {
    pub scheduler: SchedulerKind,
    pub traversal: TraversalKind,
    pub activity: ActivityKind,
    pub layout: LayoutKind,
    pub executor: ExecutorKind,
}

impl MechanicalConfig {
    pub const REFERENCE: Self = Self {
        scheduler: SchedulerKind::Vec,
        traversal: TraversalKind::GlobalScan,
        activity: ActivityKind::FullScan,
        layout: LayoutKind::AoS,
        executor: ExecutorKind::Scalar,
    };

    pub const R1: Self = Self {
        scheduler: SchedulerKind::TimingWheel,
        ..Self::REFERENCE
    };

    pub const R2: Self = Self {
        traversal: TraversalKind::Adjacency,
        ..Self::R1
    };

    pub const R3: Self = Self {
        activity: ActivityKind::Frontier,
        ..Self::R2
    };

    pub const R4: Self = Self {
        layout: LayoutKind::SoA,
        ..Self::R3
    };

    pub const R5: Self = Self {
        executor: ExecutorKind::Batched,
        ..Self::R4
    };

    /// PSEL0's measured production selection. The permanent correctness
    /// reference remains `REFERENCE`; batching falls back safely when live
    /// zero-delay topology can add current-tick work.
    pub const PRODUCTION: Self = Self {
        executor: ExecutorKind::Batched,
        ..Self::R3
    };
}

impl Default for MechanicalConfig {
    fn default() -> Self {
        Self::REFERENCE
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArrowSpec {
    pub from: CellId,
    pub to: CellId,
    pub delay: i64,
    pub phase: i32,
    pub coupling: i32,
    pub resistance: u32,
    pub mode: TransmissionMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpikeInput {
    pub arrival_tick: i64,
    pub phase: i32,
    pub origin_physical: u64,
    pub target: CellId,
    pub impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cell {
    id: CellId,
    physical_id: u64,
    position: i32,
    region: i16,
    threshold: i32,
    state: i32,
    last_update_tick: i64,
    refractory_until: i64,
    generation: Generation,
    resistance: u32,
    live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Arrow {
    id: ArrowId,
    from: CellId,
    to: CellId,
    delay: i64,
    phase: i32,
    coupling: i32,
    source_generation: Generation,
    generation: Generation,
    resistance: u32,
    live: bool,
    participation_level: u64,
    plastic_support: u64,
    decay_load: u64,
    mode: TransmissionMode,
    trigger: TransmissionTrigger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Spike {
    arrival_tick: i64,
    phase: i32,
    origin_physical: u64,
    target: CellId,
    target_generation: Generation,
    impulse: i32,
    serial: u64,
    arrow: Option<(ArrowId, Generation)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crossing {
    pub tick: i64,
    pub from_physical: u64,
    pub to_physical: u64,
    pub from_region: i16,
    pub to_region: i16,
    pub impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PhysicalEvent {
    Deliver {
        mode: TransmissionMode,
        target: CellId,
        impulse: i32,
    },
    Fire {
        cell: CellId,
    },
    Resistance {
        arrow: ArrowId,
        before: u32,
        after: u32,
    },
    Deallocate {
        arrow: ArrowId,
    },
    Proposal {
        arrow: ArrowId,
        from: CellId,
        to: CellId,
    },
    Crossing(Crossing),
    QualifiedLocalTraversal {
        arrow: ArrowId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalTransition {
    pub tick: i64,
    pub phase: i32,
    pub event: PhysicalEvent,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    total: u64,
    pub drive_deliveries: u64,
    pub modulatory_deliveries: u64,
    pub local_return_updates: u64,
    pub local_structural_proposals: u64,
    pub physical_deallocations: u64,
    pub qualified_local_traversals: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExecutionCost {
    pub queue_ops: u64,
    pub comparisons: u64,
    pub scans: u64,
    pub allocations: u64,
    pub bytes_touched: u64,
    pub peak_resident_bytes: u64,
    pub adjacency_accesses: u64,
    pub frontier_samples: u64,
    pub active_frontier_total: u64,
    pub active_frontier_max: u64,
    pub batches: u64,
    pub batched_items: u64,
    pub batch_max: u64,
    pub batch_histogram: [u64; 7],
    pub batch_fallback_zero_delay: u64,
    pub arena_lookups: u64,
    pub arena_hops: u64,
    pub active_arena_samples: u64,
    pub active_arena_total: u64,
    pub active_arena_max: u64,
}

impl ExecutionCost {
    pub(crate) fn touch<T>(&mut self, count: usize) {
        self.bytes_touched = self.bytes_touched.saturating_add(
            u64::try_from(std::mem::size_of::<T>().saturating_mul(count)).unwrap_or(u64::MAX),
        );
    }

    fn observe_resident_bytes(&mut self, bytes: usize) {
        self.peak_resident_bytes = self
            .peak_resident_bytes
            .max(u64::try_from(bytes).unwrap_or(u64::MAX));
    }

    fn observe_frontier(&mut self, active: usize) {
        let active = u64::try_from(active).unwrap_or(u64::MAX);
        self.frontier_samples = self.frontier_samples.saturating_add(1);
        self.active_frontier_total = self.active_frontier_total.saturating_add(active);
        self.active_frontier_max = self.active_frontier_max.max(active);
    }

    fn observe_batch(&mut self, size: usize) {
        let size = u64::try_from(size).unwrap_or(u64::MAX);
        self.batches = self.batches.saturating_add(1);
        self.batched_items = self.batched_items.saturating_add(size);
        self.batch_max = self.batch_max.max(size);
        let bucket = match size {
            0 | 1 => 0,
            2 => 1,
            3..=4 => 2,
            5..=8 => 3,
            9..=16 => 4,
            17..=32 => 5,
            _ => 6,
        };
        self.batch_histogram[bucket] = self.batch_histogram[bucket].saturating_add(1);
    }

    fn observe_active_arenas(&mut self, active: usize) {
        let active = u64::try_from(active).unwrap_or(u64::MAX);
        self.active_arena_samples = self.active_arena_samples.saturating_add(1);
        self.active_arena_total = self.active_arena_total.saturating_add(active);
        self.active_arena_max = self.active_arena_max.max(active);
    }
}

impl Work {
    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn physical_total(&self) -> u64 {
        let total = self
            .drive_deliveries
            .saturating_add(self.modulatory_deliveries)
            .saturating_add(self.local_return_updates)
            .saturating_add(self.local_structural_proposals)
            .saturating_add(self.physical_deallocations);
        total.saturating_add(self.qualified_local_traversals)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunResult {
    pub crossings: Vec<Crossing>,
    pub work: Work,
    pub naturally_quiescent: bool,
    pub resident_bytes: usize,
    pub execution_cost: ExecutionCost,
    pub physical_trace: Vec<PhysicalTransition>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundaryError {
    ZeroCapacity,
    InputFull {
        capacity: usize,
        occupied: usize,
        attempted: usize,
    },
    OutputFull {
        capacity: usize,
        occupied: usize,
        required: usize,
    },
    OutputBatchTooLarge {
        capacity: usize,
        required: usize,
    },
    WrongOutwardRegion {
        configured: i16,
        requested: i16,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundaryRun {
    pub consumed_inputs: usize,
    pub produced_outputs: usize,
    pub work: Work,
    pub naturally_quiescent: bool,
    pub resident_bytes: usize,
    pub execution_cost: ExecutionCost,
    pub physical_trace: Vec<PhysicalTransition>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundaryLiveCheckpoint {
    core: LiveCheckpoint,
    outward_region: i16,
    input_capacity: usize,
    output_capacity: usize,
    inputs: Vec<SpikeInput>,
    outputs: Vec<Crossing>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundaryRuntime {
    substrate: PlasticSubstrate,
    outward_region: i16,
    input_capacity: usize,
    output_capacity: usize,
    inputs: VecDeque<SpikeInput>,
    outputs: VecDeque<Crossing>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlasticSubstrate {
    mechanics: MechanicalConfig,
    arena: ArenaId,
    cells: CellStore,
    cell_slots: Vec<Option<CellSlot>>,
    arrows: ArrowStore,
    arrow_slots: Vec<Option<ArrowSlot>>,
    cell_capacity: u32,
    arrow_capacity: u32,
    pending: PendingSchedule,
    tick: i64,
    next_serial: u64,
    pressure_tick: i64,
    pending_loads: Vec<PendingLoad>,
    outgoing_index: Vec<Vec<ArrowId>>,
    resident_arenas: Vec<ResidentArenaId>,
    active_cells: BTreeSet<CellId>,
    trace_physics: bool,
    zero_delay_live_arrows: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingLoad {
    pub arena: ArenaId,
    pub version: ContentHash,
    pub issue_tick: i64,
    pub availability_tick: Option<i64>,
    pub waiting_arrivals: Vec<SpikeInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuiescentCheckpoint {
    pub body_version: BodyVersion,
    pub body: ArenaBody,
    pub clock: PhysicalClock,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveCheckpoint {
    body_version: BodyVersion,
    body: ArenaBody,
    clock: PhysicalClock,
    cells: Vec<CellRuntime>,
    arrows: Vec<ArrowRuntime>,
    pending: Vec<Spike>,
    next_serial: u64,
    pending_loads: Vec<PendingLoad>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CellRuntime {
    id: CellId,
    state: i32,
    last_update_tick: i64,
    refractory_until: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ArrowRuntime {
    id: ArrowId,
    participation_level: u64,
    plastic_support: u64,
    decay_load: u64,
    trigger: TransmissionTrigger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckpointError {
    NotQuiescent,
    Format(FormatError),
    UnsupportedTransmissionMode(u8),
    MissingCell(CellId),
    MissingArrow(ArrowId),
    ManifestMismatch,
    InvalidPhysicalBody,
    StaleCellReference(CellRef),
    Truncated,
    WrongMagic,
    UnsupportedCheckpointVersion(u16),
    InvalidCheckpoint,
    Checksum,
    TrailingBytes,
}

impl From<FormatError> for CheckpointError {
    fn from(error: FormatError) -> Self {
        Self::Format(error)
    }
}

impl QuiescentCheckpoint {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CheckpointError> {
        validate_manifest(&self.body_version, &self.body)?;
        let manifest = self.body_version.canonical_bytes()?;
        let body = self.body.canonical_bytes()?;
        let mut payload = Vec::with_capacity(manifest.len() + body.len());
        payload.extend_from_slice(&manifest);
        payload.extend_from_slice(&body);
        let mut bytes = Vec::with_capacity(66 + payload.len());
        bytes.extend_from_slice(QUIESCENT_MAGIC);
        checkpoint_put_u16(&mut bytes, CHECKPOINT_VERSION);
        checkpoint_put_i64(&mut bytes, self.clock.tick);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(manifest.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(body.len())?);
        bytes.extend_from_slice(ContentHash::of(&payload).as_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, CheckpointError> {
        if bytes.len() < 66 {
            return Err(CheckpointError::Truncated);
        }
        if &bytes[..8] != QUIESCENT_MAGIC {
            return Err(CheckpointError::WrongMagic);
        }
        let mut cursor = CheckpointCursor::new(bytes, 8);
        let version = cursor.u16()?;
        if version != CHECKPOINT_VERSION {
            return Err(CheckpointError::UnsupportedCheckpointVersion(version));
        }
        let tick = cursor.i64()?;
        let manifest_len = cursor.usize_from_u64()?;
        let body_len = cursor.usize_from_u64()?;
        let checksum = ContentHash(cursor.array_32()?);
        let payload_len = manifest_len
            .checked_add(body_len)
            .ok_or(CheckpointError::InvalidCheckpoint)?;
        let payload = cursor.bytes(payload_len)?;
        cursor.finish()?;
        if ContentHash::of(payload) != checksum {
            return Err(CheckpointError::Checksum);
        }
        let body_version = BodyVersion::decode(&payload[..manifest_len])?;
        let body = ArenaBody::decode(&payload[manifest_len..])?;
        validate_manifest(&body_version, &body)?;
        Ok(Self {
            body_version,
            body,
            clock: PhysicalClock { tick },
        })
    }
}

impl LiveCheckpoint {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CheckpointError> {
        validate_manifest(&self.body_version, &self.body)?;
        let manifest = self.body_version.canonical_bytes()?;
        let body = self.body.canonical_bytes()?;
        let mut cells = self.cells.clone();
        cells.sort_by_key(|cell| cell.id);
        let mut arrows = self.arrows.clone();
        arrows.sort_by_key(|arrow| arrow.id);
        let mut pending = self.pending.clone();
        pending.sort_by_key(|spike| {
            (
                spike.arrival_tick,
                spike.phase,
                spike.origin_physical,
                spike.target,
                spike.serial,
            )
        });
        let mut loads = self.pending_loads.clone();
        loads.sort_by_key(|load| {
            (
                load.availability_tick.unwrap_or(i64::MAX),
                load.issue_tick,
                load.arena,
            )
        });

        let mut payload = Vec::new();
        payload.extend_from_slice(&manifest);
        payload.extend_from_slice(&body);
        for cell in &cells {
            checkpoint_put_u64(&mut payload, cell.id.0);
            checkpoint_put_i32(&mut payload, cell.state);
            checkpoint_put_i64(&mut payload, cell.last_update_tick);
            checkpoint_put_i64(&mut payload, cell.refractory_until);
        }
        for arrow in &arrows {
            checkpoint_put_u64(&mut payload, arrow.id.0);
            checkpoint_put_u64(&mut payload, arrow.participation_level);
            checkpoint_put_u64(&mut payload, arrow.plastic_support);
            checkpoint_put_u64(&mut payload, arrow.decay_load);
            payload.push(transmission_trigger_byte(arrow.trigger));
        }
        for spike in &pending {
            encode_spike(&mut payload, spike);
        }
        for load in &loads {
            checkpoint_put_u64(&mut payload, load.arena.0);
            payload.extend_from_slice(load.version.as_bytes());
            checkpoint_put_i64(&mut payload, load.issue_tick);
            checkpoint_put_optional_tick(&mut payload, load.availability_tick);
            checkpoint_put_u32(
                &mut payload,
                checkpoint_len_u32(load.waiting_arrivals.len())?,
            );
            for input in &load.waiting_arrivals {
                encode_input(&mut payload, input);
            }
        }

        let mut bytes = Vec::with_capacity(98 + payload.len());
        bytes.extend_from_slice(LIVE_MAGIC);
        checkpoint_put_u16(&mut bytes, CHECKPOINT_VERSION);
        checkpoint_put_i64(&mut bytes, self.clock.tick);
        checkpoint_put_u64(&mut bytes, self.next_serial);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(manifest.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(body.len())?);
        checkpoint_put_u32(&mut bytes, checkpoint_len_u32(cells.len())?);
        checkpoint_put_u32(&mut bytes, checkpoint_len_u32(arrows.len())?);
        checkpoint_put_u32(&mut bytes, checkpoint_len_u32(pending.len())?);
        checkpoint_put_u32(&mut bytes, checkpoint_len_u32(loads.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(payload.len())?);
        bytes.extend_from_slice(ContentHash::of(&payload).as_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, CheckpointError> {
        if bytes.len() < 98 {
            return Err(CheckpointError::Truncated);
        }
        if &bytes[..8] != LIVE_MAGIC {
            return Err(CheckpointError::WrongMagic);
        }
        let mut cursor = CheckpointCursor::new(bytes, 8);
        let version = cursor.u16()?;
        if version != CHECKPOINT_VERSION {
            return Err(CheckpointError::UnsupportedCheckpointVersion(version));
        }
        let tick = cursor.i64()?;
        let next_serial = cursor.u64()?;
        let manifest_len = cursor.usize_from_u64()?;
        let body_len = cursor.usize_from_u64()?;
        let cell_count = cursor.usize_from_u32()?;
        let arrow_count = cursor.usize_from_u32()?;
        let pending_count = cursor.usize_from_u32()?;
        let load_count = cursor.usize_from_u32()?;
        let payload_len = cursor.usize_from_u64()?;
        let checksum = ContentHash(cursor.array_32()?);
        let payload = cursor.bytes(payload_len)?;
        cursor.finish()?;
        if ContentHash::of(payload) != checksum {
            return Err(CheckpointError::Checksum);
        }
        let structural_len = manifest_len
            .checked_add(body_len)
            .ok_or(CheckpointError::InvalidCheckpoint)?;
        if structural_len > payload.len() {
            return Err(CheckpointError::Truncated);
        }
        let body_version = BodyVersion::decode(&payload[..manifest_len])?;
        let body = ArenaBody::decode(&payload[manifest_len..structural_len])?;
        validate_manifest(&body_version, &body)?;
        let mut transient = CheckpointCursor::new(payload, structural_len);
        let mut cells = Vec::with_capacity(cell_count);
        for _ in 0..cell_count {
            cells.push(CellRuntime {
                id: CellId(transient.u64()?),
                state: transient.i32()?,
                last_update_tick: transient.i64()?,
                refractory_until: transient.i64()?,
            });
        }
        let mut arrows = Vec::with_capacity(arrow_count);
        for _ in 0..arrow_count {
            arrows.push(ArrowRuntime {
                id: ArrowId(transient.u64()?),
                participation_level: transient.u64()?,
                plastic_support: transient.u64()?,
                decay_load: transient.u64()?,
                trigger: transmission_trigger_from_byte(transient.u8()?)?,
            });
        }
        let mut pending = Vec::with_capacity(pending_count);
        for _ in 0..pending_count {
            pending.push(decode_spike(&mut transient)?);
        }
        let mut pending_loads = Vec::with_capacity(load_count);
        for _ in 0..load_count {
            let arena = ArenaId(transient.u64()?);
            let version = ContentHash(transient.array_32()?);
            let issue_tick = transient.i64()?;
            let availability_tick = transient.optional_tick()?;
            let waiting_count = transient.usize_from_u32()?;
            let mut waiting_arrivals = Vec::with_capacity(waiting_count);
            for _ in 0..waiting_count {
                waiting_arrivals.push(decode_input(&mut transient)?);
            }
            pending_loads.push(PendingLoad {
                arena,
                version,
                issue_tick,
                availability_tick,
                waiting_arrivals,
            });
        }
        transient.finish()?;
        Ok(Self {
            body_version,
            body,
            clock: PhysicalClock { tick },
            cells,
            arrows,
            pending,
            next_serial,
            pending_loads,
        })
    }
}

impl BoundaryLiveCheckpoint {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CheckpointError> {
        if self.input_capacity == 0
            || self.output_capacity == 0
            || self.inputs.len() > self.input_capacity
            || self.outputs.len() > self.output_capacity
        {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        let core = self.core.canonical_bytes()?;
        let mut payload = Vec::with_capacity(
            core.len()
                .saturating_add((self.inputs.len() + self.outputs.len()).saturating_mul(32)),
        );
        payload.extend_from_slice(&core);
        for input in &self.inputs {
            encode_input(&mut payload, input);
        }
        for crossing in &self.outputs {
            encode_crossing(&mut payload, crossing);
        }

        let mut bytes = Vec::with_capacity(92 + payload.len());
        bytes.extend_from_slice(BOUNDARY_LIVE_MAGIC);
        checkpoint_put_u16(&mut bytes, CHECKPOINT_VERSION);
        checkpoint_put_i16(&mut bytes, self.outward_region);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(self.input_capacity)?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(self.output_capacity)?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(self.inputs.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(self.outputs.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(core.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(payload.len())?);
        bytes.extend_from_slice(ContentHash::of(&payload).as_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, CheckpointError> {
        if bytes.len() < 92 {
            return Err(CheckpointError::Truncated);
        }
        if &bytes[..8] != BOUNDARY_LIVE_MAGIC {
            return Err(CheckpointError::WrongMagic);
        }
        let mut cursor = CheckpointCursor::new(bytes, 8);
        let version = cursor.u16()?;
        if version != CHECKPOINT_VERSION {
            return Err(CheckpointError::UnsupportedCheckpointVersion(version));
        }
        let outward_region = cursor.i16()?;
        let input_capacity = cursor.usize_from_u64()?;
        let output_capacity = cursor.usize_from_u64()?;
        let input_count = cursor.usize_from_u64()?;
        let output_count = cursor.usize_from_u64()?;
        let core_len = cursor.usize_from_u64()?;
        let payload_len = cursor.usize_from_u64()?;
        let checksum = ContentHash(cursor.array_32()?);
        let payload = cursor.bytes(payload_len)?;
        cursor.finish()?;
        if ContentHash::of(payload) != checksum {
            return Err(CheckpointError::Checksum);
        }
        if input_capacity == 0
            || output_capacity == 0
            || input_count > input_capacity
            || output_count > output_capacity
        {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        let entries = input_count
            .checked_add(output_count)
            .and_then(|count| count.checked_mul(32))
            .ok_or(CheckpointError::InvalidCheckpoint)?;
        let expected = core_len
            .checked_add(entries)
            .ok_or(CheckpointError::InvalidCheckpoint)?;
        if expected != payload_len || core_len > payload.len() {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        let core = LiveCheckpoint::decode(&payload[..core_len])?;
        let mut transient = CheckpointCursor::new(payload, core_len);
        let mut inputs = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            inputs.push(decode_input(&mut transient)?);
        }
        let mut outputs = Vec::with_capacity(output_count);
        for _ in 0..output_count {
            outputs.push(decode_crossing(&mut transient)?);
        }
        transient.finish()?;
        Ok(Self {
            core,
            outward_region,
            input_capacity,
            output_capacity,
            inputs,
            outputs,
        })
    }
}

impl BoundaryRuntime {
    pub fn new(
        substrate: PlasticSubstrate,
        outward_region: i16,
        input_capacity: usize,
        output_capacity: usize,
    ) -> Result<Self, BoundaryError> {
        if input_capacity == 0 || output_capacity == 0 {
            return Err(BoundaryError::ZeroCapacity);
        }
        Ok(Self {
            substrate,
            outward_region,
            input_capacity,
            output_capacity,
            inputs: VecDeque::with_capacity(input_capacity),
            outputs: VecDeque::with_capacity(output_capacity),
        })
    }

    pub fn substrate(&self) -> &PlasticSubstrate {
        &self.substrate
    }

    /// Changes only the mechanical execution strategy beneath the boundary.
    /// Physical law and buffered activity are preserved exactly.
    pub fn reconfigure_mechanics(&mut self, mechanics: MechanicalConfig) {
        self.substrate.reconfigure_mechanics(mechanics);
    }

    /// Reassigns resident execution placement without changing durable identity.
    pub fn repartition_resident(&mut self, placements: &[ResidentArenaId]) {
        self.substrate.repartition_resident(placements);
    }

    pub fn input_capacity(&self) -> usize {
        self.input_capacity
    }

    pub fn output_capacity(&self) -> usize {
        self.output_capacity
    }

    pub fn input_len(&self) -> usize {
        self.inputs.len()
    }

    pub fn output_len(&self) -> usize {
        self.outputs.len()
    }

    pub fn enqueue(&mut self, input: SpikeInput) -> Result<(), BoundaryError> {
        self.enqueue_batch(&[input])
    }

    pub fn enqueue_batch(&mut self, inputs: &[SpikeInput]) -> Result<(), BoundaryError> {
        let occupied = self.inputs.len();
        if inputs.len() > self.input_capacity.saturating_sub(occupied) {
            return Err(BoundaryError::InputFull {
                capacity: self.input_capacity,
                occupied,
                attempted: inputs.len(),
            });
        }
        self.inputs.extend(inputs.iter().copied());
        Ok(())
    }

    pub fn run_until_quiescent(&mut self) -> Result<BoundaryRun, BoundaryError> {
        let consumed_inputs = self.inputs.len();
        let mut candidate = self.substrate.clone();
        for input in &self.inputs {
            candidate.enter(*input);
        }
        let mut result = candidate.propagate();
        result
            .crossings
            .retain(|crossing| crossing.to_region == self.outward_region);
        let required = result.crossings.len();
        if required > self.output_capacity {
            return Err(BoundaryError::OutputBatchTooLarge {
                capacity: self.output_capacity,
                required,
            });
        }
        let occupied = self.outputs.len();
        if required > self.output_capacity.saturating_sub(occupied) {
            return Err(BoundaryError::OutputFull {
                capacity: self.output_capacity,
                occupied,
                required,
            });
        }
        self.substrate = candidate;
        self.inputs.clear();
        self.outputs.extend(result.crossings);
        Ok(BoundaryRun {
            consumed_inputs,
            produced_outputs: required,
            work: result.work,
            naturally_quiescent: result.naturally_quiescent,
            resident_bytes: result.resident_bytes,
            execution_cost: result.execution_cost,
            physical_trace: result.physical_trace,
        })
    }

    pub fn drain_output(&mut self, maximum: usize) -> Vec<Crossing> {
        let count = maximum.min(self.outputs.len());
        self.outputs.drain(..count).collect()
    }

    pub fn drain_all_output(&mut self) -> Vec<Crossing> {
        self.drain_output(self.outputs.len())
    }

    pub fn arrive(
        &mut self,
        inputs: &[SpikeInput],
        outward_region: i16,
    ) -> Result<RunResult, BoundaryError> {
        if outward_region != self.outward_region {
            return Err(BoundaryError::WrongOutwardRegion {
                configured: self.outward_region,
                requested: outward_region,
            });
        }
        self.enqueue_batch(inputs)?;
        let run = self.run_until_quiescent()?;
        Ok(RunResult {
            crossings: self.drain_all_output(),
            work: run.work,
            naturally_quiescent: run.naturally_quiescent,
            resident_bytes: run.resident_bytes,
            execution_cost: run.execution_cost,
            physical_trace: run.physical_trace,
        })
    }

    pub fn advance_time(&mut self, tick: i64) -> Work {
        assert!(
            self.inputs.is_empty() && self.outputs.is_empty(),
            "boundary buffers must be drained before advancing time"
        );
        self.substrate.advance_time(tick)
    }

    pub fn live_checkpoint(
        &self,
        body_version: u64,
    ) -> Result<BoundaryLiveCheckpoint, CheckpointError> {
        Ok(BoundaryLiveCheckpoint {
            core: self.substrate.live_checkpoint(body_version)?,
            outward_region: self.outward_region,
            input_capacity: self.input_capacity,
            output_capacity: self.output_capacity,
            inputs: self.inputs.iter().copied().collect(),
            outputs: self.outputs.iter().copied().collect(),
        })
    }

    pub fn from_live_checkpoint(
        checkpoint: BoundaryLiveCheckpoint,
    ) -> Result<Self, CheckpointError> {
        if checkpoint.input_capacity == 0
            || checkpoint.output_capacity == 0
            || checkpoint.inputs.len() > checkpoint.input_capacity
            || checkpoint.outputs.len() > checkpoint.output_capacity
        {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        Ok(Self {
            substrate: PlasticSubstrate::from_live_checkpoint(checkpoint.core)?,
            outward_region: checkpoint.outward_region,
            input_capacity: checkpoint.input_capacity,
            output_capacity: checkpoint.output_capacity,
            inputs: checkpoint.inputs.into(),
            outputs: checkpoint.outputs.into(),
        })
    }
}

impl Default for PlasticSubstrate {
    fn default() -> Self {
        Self::with_capacity(ArenaId(0), DEFAULT_CELL_CAPACITY, DEFAULT_ARROW_CAPACITY)
    }
}

impl PlasticSubstrate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(arena: ArenaId, cell_capacity: u32, arrow_capacity: u32) -> Self {
        Self::with_mechanics(
            arena,
            cell_capacity,
            arrow_capacity,
            MechanicalConfig::REFERENCE,
        )
    }

    pub fn with_mechanics(
        arena: ArenaId,
        cell_capacity: u32,
        arrow_capacity: u32,
        mechanics: MechanicalConfig,
    ) -> Self {
        Self {
            mechanics,
            arena,
            cells: CellStore::new(mechanics.layout),
            cell_slots: Vec::new(),
            arrows: ArrowStore::new(mechanics.layout),
            arrow_slots: Vec::new(),
            cell_capacity,
            arrow_capacity,
            pending: PendingSchedule::new(mechanics.scheduler, 0),
            tick: 0,
            next_serial: 0,
            pressure_tick: 0,
            pending_loads: Vec::new(),
            outgoing_index: Vec::new(),
            resident_arenas: Vec::new(),
            active_cells: BTreeSet::new(),
            trace_physics: false,
            zero_delay_live_arrows: 0,
        }
    }

    pub fn mechanical_config(&self) -> MechanicalConfig {
        self.mechanics
    }

    pub fn set_physical_tracing(&mut self, enabled: bool) {
        self.trace_physics = enabled;
    }

    pub fn reconfigure_mechanics(&mut self, mechanics: MechanicalConfig) {
        let was_partitioned = self.pending.is_partitioned();
        let pending = self.pending.canonical(|id| {
            self.cells
                .get(self.cell_slot(id).expect("scheduled CELL must resolve").0)
                .physical_id
        });
        self.cells.convert(mechanics.layout);
        self.arrows.convert(mechanics.layout);
        self.pending = if was_partitioned {
            PendingSchedule::partitioned(self.tick, self.resident_arenas.clone(), pending)
        } else {
            PendingSchedule::from_canonical(mechanics.scheduler, self.tick, pending)
        };
        self.mechanics = mechanics;
        self.rebuild_slot_maps();
        self.rebuild_mechanical_indexes();
    }

    pub fn repartition_resident(&mut self, placements: &[ResidentArenaId]) {
        assert_eq!(
            placements.len(),
            self.cell_slots.len(),
            "resident partition must assign every CELL identity"
        );
        let pending = self.pending.canonical(|id| {
            self.cells
                .get(self.cell_slot(id).expect("scheduled CELL must resolve").0)
                .physical_id
        });
        self.resident_arenas = placements.to_vec();
        self.pending =
            PendingSchedule::partitioned(self.tick, self.resident_arenas.clone(), pending);
    }

    pub fn resident_arena(&self, cell: CellId) -> ResidentArenaId {
        self.require_cell(cell);
        self.resident_arenas[cell.0 as usize]
    }

    pub fn resident_arena_count(&self) -> usize {
        self.resident_arenas
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
    }

    pub fn clock(&self) -> PhysicalClock {
        PhysicalClock { tick: self.tick }
    }

    pub fn resolve_cell(&self, reference: CellRef) -> Option<CellSlot> {
        if reference.arena != self.arena {
            return None;
        }
        let slot = self.cell_slot(reference.id)?;
        let cell = self.cells.get(slot.0);
        (cell.live && cell.generation == reference.generation).then_some(slot)
    }

    pub fn resolve_arrow(&self, reference: ArrowRef) -> Option<ArrowSlot> {
        if reference.arena != self.arena {
            return None;
        }
        let slot = self.arrow_slot(reference.id)?;
        let arrow = self.arrows.get(slot.0);
        (arrow.live && arrow.generation == reference.generation).then_some(slot)
    }

    pub fn add_cell(&mut self, spec: CellSpec) -> CellId {
        assert!(spec.threshold > 0, "threshold must be physically positive");
        assert!(
            self.cells
                .values()
                .iter()
                .all(|cell| cell.physical_id != spec.physical_id),
            "physical cell identity must be unique"
        );
        assert!(
            self.cell_slots.len() < self.cell_capacity as usize,
            "resident arena has no free CELL identity"
        );
        let id = CellId(self.cell_slots.len() as u64);
        let slot = CellSlot(self.cells.len());
        self.cells.push(Cell {
            id,
            physical_id: spec.physical_id,
            position: spec.position,
            region: spec.region,
            threshold: spec.threshold,
            state: 0,
            last_update_tick: self.tick,
            refractory_until: self.tick,
            generation: Generation(1),
            resistance: spec.resistance,
            live: spec.resistance > 0,
        });
        self.cell_slots.push(Some(slot));
        self.outgoing_index.push(Vec::new());
        self.resident_arenas.push(ResidentArenaId(0));
        id
    }

    pub fn add_arrow(&mut self, spec: ArrowSpec) -> ArrowId {
        self.require_cell(spec.from);
        self.require_cell(spec.to);
        assert!(spec.delay >= 0, "delay must not run backward in time");
        let source_slot = self
            .cell_slot(spec.from)
            .expect("required CELL must resolve");
        let source_generation = self.cells.get(source_slot.0).generation;
        let reusable = self.arrows.values().iter().position(|arrow| !arrow.live);
        if reusable.is_none() {
            assert!(
                self.arrow_slots.len() < self.arrow_capacity as usize,
                "resident arena has no free ARROW identity"
            );
        }
        let (id, slot, generation, prior_source) = if let Some(index) = reusable {
            let prior = self.arrows.get(index);
            (
                prior.id,
                ArrowSlot(index),
                prior.generation,
                Some(prior.from),
            )
        } else {
            let id = ArrowId(self.arrow_slots.len() as u64);
            (id, ArrowSlot(self.arrows.len()), Generation(1), None)
        };
        let arrow = Arrow {
            id,
            from: spec.from,
            to: spec.to,
            delay: spec.delay,
            phase: spec.phase,
            coupling: spec.coupling,
            source_generation,
            generation,
            resistance: spec.resistance,
            live: spec.resistance > 0,
            participation_level: 0,
            plastic_support: 0,
            decay_load: 0,
            mode: spec.mode,
            trigger: TransmissionTrigger::SourceFires,
        };
        if slot.0 < self.arrows.len() {
            if let Some(prior_source) = prior_source {
                self.outgoing_index[prior_source.0 as usize].retain(|candidate| *candidate != id);
            }
            self.arrows.set(slot.0, arrow);
            self.arrow_slots[id.0 as usize] = Some(slot);
        } else {
            self.arrows.push(arrow);
            self.arrow_slots.push(Some(slot));
        }
        let outgoing = &mut self.outgoing_index[spec.from.0 as usize];
        outgoing.push(id);
        outgoing.sort_unstable();
        if spec.resistance > 0 && spec.delay == 0 {
            self.zero_delay_live_arrows = self.zero_delay_live_arrows.saturating_add(1);
        }
        id
    }

    pub fn add_arrow_with_trigger(
        &mut self,
        spec: ArrowSpec,
        trigger: TransmissionTrigger,
    ) -> ArrowId {
        assert!(
            trigger == TransmissionTrigger::SourceFires
                || spec.mode == TransmissionMode::Modulatory,
            "qualified local transmission must have Modulatory effect"
        );
        let id = self.add_arrow(spec);
        let slot = self.arrow_slot(id).expect("new ARROW must resolve");
        self.arrows
            .with_mut(slot.0, |arrow| arrow.trigger = trigger);
        id
    }

    pub fn transmission_trigger(&self, id: ArrowId) -> TransmissionTrigger {
        self.arrows
            .get(self.arrow_slot(id).expect("ARROW identity must resolve").0)
            .trigger
    }

    pub fn enter(&mut self, input: SpikeInput) {
        self.require_cell(input.target);
        assert!(
            input.arrival_tick >= self.tick,
            "physical arrivals cannot precede current substrate time"
        );
        let mut ignored = ExecutionCost::default();
        self.pending.push(
            Spike {
                arrival_tick: input.arrival_tick,
                phase: input.phase,
                origin_physical: input.origin_physical,
                target: input.target,
                target_generation: self
                    .cells
                    .get(self.cell_slot(input.target).unwrap().0)
                    .generation,
                impulse: input.impulse,
                serial: self.next_serial,
                arrow: None,
            },
            &mut ignored,
        );
        self.next_serial = self.next_serial.wrapping_add(1);
    }

    pub fn arrive(&mut self, inputs: &[SpikeInput], outward_region: i16) -> RunResult {
        for input in inputs {
            self.enter(*input);
        }
        let mut result = self.propagate();
        result
            .crossings
            .retain(|crossing| crossing.to_region == outward_region);
        result
    }

    pub fn advance_time(&mut self, tick: i64) -> Work {
        assert!(tick >= self.tick, "physical time cannot run backward");
        assert!(
            self.pending.is_empty(),
            "queued activity must propagate first"
        );
        let mut work = Work::default();
        let mut ignored = ExecutionCost::default();
        self.elapse_to(tick, &mut work, &mut ignored);
        self.tick = tick;
        work
    }

    pub fn arena_body(&self, version: u64) -> ArenaBody {
        let minimum_position = self
            .cells
            .values()
            .iter()
            .map(|cell| cell.position)
            .min()
            .unwrap_or(0);
        let maximum_position = self
            .cells
            .values()
            .iter()
            .map(|cell| cell.position)
            .max()
            .unwrap_or(0);
        ArenaBody {
            arena: self.arena,
            version,
            minimum_position,
            maximum_position,
            cell_capacity: self.cell_capacity,
            arrow_capacity: self.arrow_capacity,
            cells: self
                .cells
                .values()
                .iter()
                .map(|cell| DurableCell {
                    id: cell.id,
                    generation: cell.generation,
                    physical_id: cell.physical_id,
                    position: cell.position,
                    region: cell.region,
                    threshold: cell.threshold,
                    resistance: cell.resistance,
                    live: cell.live,
                })
                .collect(),
            arrows: self
                .arrows
                .values()
                .iter()
                .map(|arrow| DurableArrow {
                    id: arrow.id,
                    generation: arrow.generation,
                    from: self.cell_reference(arrow.from),
                    to: self.cell_reference(arrow.to),
                    delay: arrow.delay,
                    phase: arrow.phase,
                    coupling: arrow.coupling,
                    resistance: arrow.resistance,
                    transmission_mode: transmission_mode_byte(arrow.mode),
                    live: arrow.live,
                })
                .collect(),
        }
    }

    pub fn canonical_body_bytes(&self, version: u64) -> Result<Vec<u8>, FormatError> {
        self.arena_body(version).canonical_bytes()
    }

    pub fn from_body_bytes(bytes: &[u8]) -> Result<Self, CheckpointError> {
        Self::from_arena_body(ArenaBody::decode(bytes)?)
    }

    pub fn from_arena_body(body: ArenaBody) -> Result<Self, CheckpointError> {
        Self::from_arena_body_with_packing(body, false)
    }

    pub fn from_arena_body_with_packing(
        mut body: ArenaBody,
        reverse_slots: bool,
    ) -> Result<Self, CheckpointError> {
        body.validate()?;
        if reverse_slots {
            body.cells.sort_by_key(|cell| std::cmp::Reverse(cell.id));
            body.arrows.sort_by_key(|arrow| std::cmp::Reverse(arrow.id));
        } else {
            body.cells.sort_by_key(|cell| cell.id);
            body.arrows.sort_by_key(|arrow| arrow.id);
        }
        let mut substrate =
            Self::with_capacity(body.arena, body.cell_capacity, body.arrow_capacity);
        let maximum_cell_id = body.cells.iter().map(|cell| cell.id.0).max();
        substrate.cell_slots = maximum_cell_id
            .map(|maximum| vec![None; maximum as usize + 1])
            .unwrap_or_default();
        for durable in body.cells {
            if durable.threshold <= 0 || durable.live != (durable.resistance > 0) {
                return Err(CheckpointError::InvalidPhysicalBody);
            }
            let slot = CellSlot(substrate.cells.len());
            substrate.cell_slots[durable.id.0 as usize] = Some(slot);
            substrate.cells.push(Cell {
                id: durable.id,
                physical_id: durable.physical_id,
                position: durable.position,
                region: durable.region,
                threshold: durable.threshold,
                state: 0,
                last_update_tick: 0,
                refractory_until: 0,
                generation: durable.generation,
                resistance: durable.resistance,
                live: durable.live,
            });
        }
        substrate.outgoing_index = vec![Vec::new(); substrate.cell_slots.len()];
        substrate.resident_arenas = vec![ResidentArenaId(0); substrate.cell_slots.len()];
        let maximum_arrow_id = body.arrows.iter().map(|arrow| arrow.id.0).max();
        substrate.arrow_slots = maximum_arrow_id
            .map(|maximum| vec![None; maximum as usize + 1])
            .unwrap_or_default();
        for durable in body.arrows {
            if durable.from.arena != substrate.arena || durable.to.arena != substrate.arena {
                return Err(CheckpointError::MissingCell(durable.from.id));
            }
            substrate.require_cell_result(durable.from.id)?;
            substrate.require_cell_result(durable.to.id)?;
            let from_slot = substrate.cell_slot(durable.from.id).unwrap();
            let to_slot = substrate.cell_slot(durable.to.id).unwrap();
            if substrate.cells.get(from_slot.0).generation != durable.from.generation {
                return Err(CheckpointError::StaleCellReference(durable.from));
            }
            if substrate.cells.get(to_slot.0).generation != durable.to.generation {
                return Err(CheckpointError::StaleCellReference(durable.to));
            }
            if durable.delay < 0 || durable.live != (durable.resistance > 0) {
                return Err(CheckpointError::InvalidPhysicalBody);
            }
            let mode = transmission_mode_from_byte(durable.transmission_mode)?;
            let slot = ArrowSlot(substrate.arrows.len());
            substrate.arrow_slots[durable.id.0 as usize] = Some(slot);
            substrate.arrows.push(Arrow {
                id: durable.id,
                from: durable.from.id,
                to: durable.to.id,
                delay: durable.delay,
                phase: durable.phase,
                coupling: durable.coupling,
                source_generation: durable.from.generation,
                generation: durable.generation,
                resistance: durable.resistance,
                live: durable.live,
                participation_level: 0,
                plastic_support: 0,
                decay_load: 0,
                mode,
                trigger: TransmissionTrigger::SourceFires,
            });
            substrate.outgoing_index[durable.from.id.0 as usize].push(durable.id);
        }
        for outgoing in &mut substrate.outgoing_index {
            outgoing.sort_unstable();
        }
        substrate.rebuild_mechanical_indexes();
        Ok(substrate)
    }

    pub fn quiescent_checkpoint(
        &self,
        body_version: u64,
    ) -> Result<QuiescentCheckpoint, CheckpointError> {
        let transiently_quiet = self.pending.is_empty()
            && self.pending_loads.is_empty()
            && self
                .cells
                .values()
                .iter()
                .all(|cell| cell.state == 0 && cell.refractory_until <= self.tick)
            && self
                .arrows
                .values()
                .iter()
                .all(|arrow| arrow.participation_level == 0 && arrow.decay_load == 0);
        if !transiently_quiet {
            return Err(CheckpointError::NotQuiescent);
        }
        Ok(QuiescentCheckpoint {
            body_version: self.body_version(body_version)?,
            body: self.arena_body(body_version),
            clock: self.clock(),
        })
    }

    pub fn from_quiescent_checkpoint(
        checkpoint: QuiescentCheckpoint,
    ) -> Result<Self, CheckpointError> {
        Self::from_quiescent_checkpoint_with_mechanics(checkpoint, MechanicalConfig::REFERENCE)
    }

    pub fn from_quiescent_checkpoint_with_mechanics(
        checkpoint: QuiescentCheckpoint,
        mechanics: MechanicalConfig,
    ) -> Result<Self, CheckpointError> {
        validate_manifest(&checkpoint.body_version, &checkpoint.body)?;
        let mut substrate = Self::from_arena_body(checkpoint.body)?;
        substrate.reconfigure_mechanics(mechanics);
        substrate.tick = checkpoint.clock.tick;
        substrate.pressure_tick = pressure_epoch(checkpoint.clock.tick);
        for index in 0..substrate.cells.len() {
            substrate.cells.with_mut(index, |cell| {
                cell.last_update_tick = checkpoint.clock.tick;
                cell.refractory_until = checkpoint.clock.tick;
            });
        }
        Ok(substrate)
    }

    pub fn live_checkpoint(&self, body_version: u64) -> Result<LiveCheckpoint, CheckpointError> {
        let pending = self.pending.canonical(|id| {
            self.cells
                .get(self.cell_slot(id).expect("scheduled CELL must resolve").0)
                .physical_id
        });
        Ok(LiveCheckpoint {
            body_version: self.body_version(body_version)?,
            body: self.arena_body(body_version),
            clock: self.clock(),
            cells: self
                .cells
                .values()
                .iter()
                .map(|cell| CellRuntime {
                    id: cell.id,
                    state: cell.state,
                    last_update_tick: cell.last_update_tick,
                    refractory_until: cell.refractory_until,
                })
                .collect(),
            arrows: self
                .arrows
                .values()
                .iter()
                .map(|arrow| ArrowRuntime {
                    id: arrow.id,
                    participation_level: arrow.participation_level,
                    plastic_support: arrow.plastic_support,
                    decay_load: arrow.decay_load,
                    trigger: arrow.trigger,
                })
                .collect(),
            pending,
            next_serial: self.next_serial,
            pending_loads: self.pending_loads.clone(),
        })
    }

    pub fn from_live_checkpoint(checkpoint: LiveCheckpoint) -> Result<Self, CheckpointError> {
        Self::from_live_checkpoint_with_mechanics(checkpoint, MechanicalConfig::REFERENCE)
    }

    pub fn from_live_checkpoint_with_mechanics(
        checkpoint: LiveCheckpoint,
        mechanics: MechanicalConfig,
    ) -> Result<Self, CheckpointError> {
        validate_manifest(&checkpoint.body_version, &checkpoint.body)?;
        let mut substrate = Self::from_arena_body(checkpoint.body)?;
        substrate.tick = checkpoint.clock.tick;
        substrate.pressure_tick = pressure_epoch(checkpoint.clock.tick);
        for runtime in checkpoint.cells {
            let slot = substrate
                .cell_slot(runtime.id)
                .ok_or(CheckpointError::MissingCell(runtime.id))?;
            substrate.cells.with_mut(slot.0, |cell| {
                cell.state = runtime.state;
                cell.last_update_tick = runtime.last_update_tick;
                cell.refractory_until = runtime.refractory_until;
            });
        }
        for runtime in checkpoint.arrows {
            let slot = substrate
                .arrow_slot(runtime.id)
                .ok_or(CheckpointError::MissingArrow(runtime.id))?;
            substrate.arrows.with_mut(slot.0, |arrow| {
                arrow.participation_level = runtime.participation_level;
                arrow.plastic_support = runtime.plastic_support;
                arrow.decay_load = runtime.decay_load;
                arrow.trigger = runtime.trigger;
            });
        }
        substrate.pending = PendingSchedule::from_canonical(
            SchedulerKind::Vec,
            checkpoint.clock.tick,
            checkpoint.pending,
        );
        substrate.next_serial = checkpoint.next_serial;
        substrate.pending_loads = checkpoint.pending_loads;
        substrate.active_cells = substrate
            .cells
            .values()
            .iter()
            .filter(|cell| cell.state != 0)
            .map(|cell| cell.id)
            .collect();
        substrate.reconfigure_mechanics(mechanics);
        Ok(substrate)
    }

    fn body_version(&self, version: u64) -> Result<BodyVersion, CheckpointError> {
        let body = self.arena_body(version);
        Ok(BodyVersion {
            version,
            parent: None,
            arenas: vec![ArenaVersion {
                arena: self.arena,
                block: body.content_hash()?,
            }],
        })
    }

    pub fn register_pending_load(&mut self, load: PendingLoad) {
        assert!(
            load.issue_tick >= self.tick,
            "load issue cannot precede physical time"
        );
        if let Some(availability_tick) = load.availability_tick {
            assert!(
                availability_tick >= load.issue_tick,
                "availability cannot precede load issue"
            );
        }
        self.pending_loads.push(load);
        self.pending_loads.sort_by_key(|load| {
            (
                load.availability_tick.unwrap_or(i64::MAX),
                load.issue_tick,
                load.arena,
            )
        });
    }

    pub fn admit_load_availability(&mut self, arena: ArenaId, availability_tick: i64) {
        let load = self
            .pending_loads
            .iter_mut()
            .find(|load| load.arena == arena && load.availability_tick.is_none())
            .expect("pending load must exist before availability is admitted");
        assert!(
            availability_tick >= load.issue_tick,
            "availability cannot precede load issue"
        );
        load.availability_tick = Some(availability_tick);
    }

    pub fn compact_resident(&mut self) {
        let mut cells = self.cells.values();
        cells.sort_by_key(|cell| std::cmp::Reverse(cell.id));
        self.cells.replace_values(cells);
        let mut arrows = self.arrows.values();
        arrows.sort_by_key(|arrow| (!arrow.live, std::cmp::Reverse(arrow.id)));
        self.arrows.replace_values(arrows);
        self.rebuild_slot_maps();
    }

    pub fn propagate(&mut self) -> RunResult {
        let mut crossings = Vec::new();
        let mut work = Work::default();
        let mut execution_cost = ExecutionCost::default();
        let mut physical_trace = Vec::new();
        execution_cost.observe_resident_bytes(self.mechanical_resident_bytes());
        while !self.pending.is_empty() {
            let batch = if self.mechanics.executor == ExecutorKind::Batched {
                if self.zero_delay_live_arrows == 0 {
                    let batch = self.pop_scheduled_batch(64, &mut execution_cost);
                    execution_cost.observe_batch(batch.len());
                    batch
                } else {
                    execution_cost.batch_fallback_zero_delay =
                        execution_cost.batch_fallback_zero_delay.saturating_add(1);
                    execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                    vec![self
                        .pop_scheduled(&mut execution_cost)
                        .expect("nonempty schedule must pop")]
                }
            } else {
                execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                vec![self
                    .pop_scheduled(&mut execution_cost)
                    .expect("nonempty schedule must pop")]
            };
            for (spike, legacy_comparisons) in batch {
                execution_cost.touch::<Spike>(1);
                work.total = work.total.saturating_add(legacy_comparisons);
                let external_arrival = spike.arrow.is_none();
                self.elapse_to(spike.arrival_tick, &mut work, &mut execution_cost);
                self.tick = spike.arrival_tick;
                work.total = work.total.saturating_add(2);

                if let Some((arrow_id, generation)) = spike.arrow {
                    let Some(arrow_slot) = self.arrow_slot(arrow_id) else {
                        continue;
                    };
                    let arrow = self.arrows.get(arrow_slot.0);
                    execution_cost.touch::<Arrow>(1);
                    if !arrow.live || arrow.generation != generation {
                        continue;
                    }
                }
                let Some(target_slot) = self.cell_slot(spike.target) else {
                    continue;
                };
                let target = self.cells.get(target_slot.0);
                execution_cost.touch::<Cell>(1);
                if !target.live || target.generation != spike.target_generation {
                    continue;
                }

                let mode = spike.arrow.map_or(TransmissionMode::Drive, |(arrow, _)| {
                    execution_cost.touch::<Arrow>(1);
                    self.arrows.get(self.arrow_slot(arrow).unwrap().0).mode
                });
                if self.trace_physics {
                    physical_trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase: spike.phase,
                        event: PhysicalEvent::Deliver {
                            mode,
                            target: spike.target,
                            impulse: spike.impulse,
                        },
                    });
                }
                if mode == TransmissionMode::Modulatory {
                    work.total = work.total.saturating_add(1);
                    work.modulatory_deliveries = work.modulatory_deliveries.saturating_add(1);
                    self.apply_modulatory_return(
                        spike.target,
                        self.tick,
                        &mut work,
                        &mut execution_cost,
                        spike.phase,
                        &mut physical_trace,
                    );
                    continue;
                }
                work.total = work.total.saturating_add(3);
                work.drive_deliveries = work.drive_deliveries.saturating_add(1);
                self.decay_cell(spike.target, self.tick);
                let target_slot = self.cell_slot(spike.target).unwrap();
                let target = self.cells.with_mut(target_slot.0, |target| {
                    target.state = target.state.saturating_add(spike.impulse);
                    target.clone()
                });
                execution_cost.touch::<Cell>(1);
                if target.state != 0 {
                    self.active_cells.insert(spike.target);
                }
                let fires =
                    self.tick >= target.refractory_until && target.state >= target.threshold;
                if !fires {
                    continue;
                }

                self.cells.with_mut(target_slot.0, |target| {
                    target.state = 0;
                    target.refractory_until = self.tick.saturating_add(1);
                });
                self.active_cells.remove(&spike.target);
                if self.trace_physics {
                    physical_trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase: spike.phase,
                        event: PhysicalEvent::Fire { cell: spike.target },
                    });
                }
                work.total = work.total.saturating_add(1);
                let source = spike.target;
                let origin_physical = target.physical_id;
                let source_generation = target.generation;
                if external_arrival {
                    self.propose_local_arrows(
                        source,
                        &mut work,
                        &mut execution_cost,
                        spike.phase,
                        &mut physical_trace,
                    );
                }
                let mut outgoing = match self.mechanics.traversal {
                    TraversalKind::GlobalScan => {
                        execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                        execution_cost.scans = execution_cost
                            .scans
                            .saturating_add(self.arrows.len() as u64);
                        execution_cost.touch::<Arrow>(self.arrows.len());
                        self.arrows
                            .values()
                            .iter()
                            .map(|arrow| (arrow.id, arrow.clone()))
                            .collect::<Vec<_>>()
                    }
                    TraversalKind::Adjacency => {
                        execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                        execution_cost.adjacency_accesses =
                            execution_cost.adjacency_accesses.saturating_add(
                                u64::try_from(self.outgoing_index[source.0 as usize].len())
                                    .unwrap_or(u64::MAX),
                            );
                        self.outgoing_index[source.0 as usize]
                            .iter()
                            .filter_map(|id| {
                                let slot = self.arrow_slot(*id)?;
                                execution_cost.scans = execution_cost.scans.saturating_add(1);
                                execution_cost.touch::<Arrow>(1);
                                Some((*id, self.arrows.get(slot.0)))
                            })
                            .collect()
                    }
                };
                outgoing.sort_by_key(|(id, _)| *id);
                for (arrow_id, arrow) in outgoing {
                    execution_cost.touch::<Arrow>(1);
                    work.total = work.total.saturating_add(1);
                    if !arrow.live
                        || arrow.from != source
                        || arrow.source_generation != source_generation
                    {
                        continue;
                    }
                    if arrow.trigger != TransmissionTrigger::SourceFires {
                        continue;
                    }
                    let from_slot = self.cell_slot(arrow.from).unwrap();
                    let to_slot = self.cell_slot(arrow.to).unwrap();
                    let from = self.cells.get(from_slot.0);
                    let to = self.cells.get(to_slot.0);
                    execution_cost.touch::<Cell>(2);
                    if from.region != to.region {
                        let crossing = Crossing {
                            tick: self.tick,
                            from_physical: from.physical_id,
                            to_physical: to.physical_id,
                            from_region: from.region,
                            to_region: to.region,
                            impulse: arrow.coupling,
                        };
                        if self.trace_physics {
                            physical_trace.push(PhysicalTransition {
                                tick: self.tick,
                                phase: spike.phase,
                                event: PhysicalEvent::Crossing(crossing),
                            });
                        }
                        crossings.push(crossing);
                    }
                    let arrow_slot = self.arrow_slot(arrow_id).unwrap();
                    self.arrows.with_mut(arrow_slot.0, |live_arrow| {
                        live_arrow.participation_level = live_arrow
                            .participation_level
                            .saturating_add(PARTICIPATION_IMPULSE);
                    });
                    execution_cost.touch::<Arrow>(1);
                    work.total = work.total.saturating_add(1);
                    execution_cost.arena_lookups = execution_cost.arena_lookups.saturating_add(2);
                    if self.resident_arenas[arrow.from.0 as usize]
                        != self.resident_arenas[arrow.to.0 as usize]
                    {
                        execution_cost.arena_hops = execution_cost.arena_hops.saturating_add(1);
                    }
                    self.pending.push(
                        Spike {
                            arrival_tick: self.tick.saturating_add(arrow.delay),
                            phase: arrow.phase,
                            origin_physical,
                            target: arrow.to,
                            target_generation: to.generation,
                            impulse: arrow.coupling,
                            serial: self.next_serial,
                            arrow: Some((arrow_id, arrow.generation)),
                        },
                        &mut execution_cost,
                    );
                    self.next_serial = self.next_serial.wrapping_add(1);
                }
            }
            execution_cost.observe_resident_bytes(self.mechanical_resident_bytes());
        }
        RunResult {
            crossings,
            work,
            naturally_quiescent: self.pending.is_empty(),
            resident_bytes: self.resident_bytes(),
            execution_cost,
            physical_trace,
        }
    }

    fn pop_scheduled(&mut self, execution_cost: &mut ExecutionCost) -> Option<(Spike, u64)> {
        let cells = &self.cells;
        let slots = &self.cell_slots;
        self.pending.pop_next(
            |id| {
                let slot = slots[id.0 as usize].expect("scheduled CELL must resolve");
                cells.get(slot.0).physical_id
            },
            execution_cost,
        )
    }

    fn pop_scheduled_batch(
        &mut self,
        maximum: usize,
        execution_cost: &mut ExecutionCost,
    ) -> Vec<(Spike, u64)> {
        let cells = &self.cells;
        let slots = &self.cell_slots;
        self.pending.pop_same_tick_batch(
            maximum,
            |id| {
                let slot = slots[id.0 as usize].expect("scheduled CELL must resolve");
                cells.get(slot.0).physical_id
            },
            execution_cost,
        )
    }

    fn apply_modulatory_return(
        &mut self,
        cell: CellId,
        tick: i64,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        phase: i32,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) {
        let candidates = match self.mechanics.traversal {
            TraversalKind::GlobalScan => {
                execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                execution_cost.scans = execution_cost
                    .scans
                    .saturating_add(self.arrows.len() as u64);
                execution_cost.touch::<Arrow>(self.arrows.len());
                self.arrows
                    .values()
                    .iter()
                    .map(|arrow| arrow.id)
                    .collect::<Vec<_>>()
            }
            TraversalKind::Adjacency => {
                execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                execution_cost.adjacency_accesses =
                    execution_cost.adjacency_accesses.saturating_add(
                        u64::try_from(self.outgoing_index[cell.0 as usize].len())
                            .unwrap_or(u64::MAX),
                    );
                self.outgoing_index[cell.0 as usize].clone()
            }
        };
        let qualified_local = candidates.iter().any(|id| {
            let slot = self.arrow_slot(*id).expect("indexed ARROW must resolve");
            let arrow = self.arrows.get(slot.0);
            arrow.live
                && arrow.from == cell
                && arrow.mode == TransmissionMode::Drive
                && arrow.participation_level > 0
        });
        for id in candidates {
            execution_cost.scans = execution_cost.scans.saturating_add(1);
            let slot = self.arrow_slot(id).expect("indexed ARROW must resolve");
            let updated = self.arrows.with_mut(slot.0, |arrow| {
                if !(arrow.live && arrow.from == cell) {
                    return None;
                }
                let participation = arrow.participation_level;
                arrow.plastic_support = arrow.plastic_support.saturating_add(participation);
                let bounded = participation.min(PARTICIPATION_IMPULSE);
                let numerator =
                    u128::from(bounded).saturating_mul(u128::from(LOCAL_RETURN_STRENGTH));
                let gain = numerator
                    .saturating_add(u128::from(PARTICIPATION_IMPULSE).saturating_sub(1))
                    / u128::from(PARTICIPATION_IMPULSE);
                let gain = u32::try_from(gain).unwrap_or(LOCAL_RETURN_STRENGTH);
                let before = arrow.resistance;
                arrow.resistance = arrow.resistance.saturating_add(gain);
                Some((before, arrow.resistance))
            });
            execution_cost.touch::<Arrow>(1);
            if let Some((before, after)) = updated {
                if before != after {
                    work.total = work.total.saturating_add(3);
                    work.local_return_updates = work.local_return_updates.saturating_add(1);
                }
                if self.trace_physics && before != after {
                    physical_trace.push(PhysicalTransition {
                        tick,
                        phase,
                        event: PhysicalEvent::Resistance {
                            arrow: id,
                            before,
                            after,
                        },
                    });
                }
            }
        }
        if qualified_local {
            self.propagate_qualified_local(cell, tick, phase, work, execution_cost, physical_trace);
        }
    }

    fn propagate_qualified_local(
        &mut self,
        cell: CellId,
        tick: i64,
        phase: i32,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) {
        let outgoing = self.outgoing_index[cell.0 as usize].clone();
        for id in outgoing {
            let slot = self.arrow_slot(id).expect("indexed ARROW must resolve");
            let arrow = self.arrows.get(slot.0);
            execution_cost.scans = execution_cost.scans.saturating_add(1);
            execution_cost.touch::<Arrow>(1);
            if !arrow.live
                || arrow.from != cell
                || arrow.trigger != TransmissionTrigger::QualifiedLocalParticipation
            {
                continue;
            }
            assert_eq!(arrow.mode, TransmissionMode::Modulatory);
            let source = self.cells.get(self.cell_slot(arrow.from).unwrap().0);
            let target = self.cells.get(self.cell_slot(arrow.to).unwrap().0);
            let arrival_tick = tick.saturating_add(arrow.delay);
            let arrival_phase = arrow.phase;
            let generation = arrow.generation;
            let coupling = arrow.coupling;
            let target_generation = target.generation;
            let target_id = arrow.to;
            let origin_physical = source.physical_id;
            self.arrows.with_mut(slot.0, |live_arrow| {
                live_arrow.participation_level = live_arrow
                    .participation_level
                    .saturating_add(PARTICIPATION_IMPULSE);
            });
            work.total = work.total.saturating_add(1);
            work.qualified_local_traversals = work.qualified_local_traversals.saturating_add(1);
            if self.trace_physics {
                physical_trace.push(PhysicalTransition {
                    tick,
                    phase,
                    event: PhysicalEvent::QualifiedLocalTraversal { arrow: id },
                });
            }
            self.pending.push(
                Spike {
                    arrival_tick,
                    phase: arrival_phase,
                    origin_physical,
                    target: target_id,
                    target_generation,
                    impulse: coupling,
                    serial: self.next_serial,
                    arrow: Some((id, generation)),
                },
                execution_cost,
            );
            self.next_serial = self.next_serial.wrapping_add(1);
        }
    }

    fn elapse_to(&mut self, tick: i64, work: &mut Work, execution_cost: &mut ExecutionCost) {
        self.elapse_fd0_decay(tick, work, execution_cost);
        execution_cost.observe_frontier(self.active_cells.len());
        let cell_ids = match self.mechanics.activity {
            ActivityKind::FullScan => {
                execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                execution_cost.touch::<Cell>(self.cells.len());
                self.cells
                    .values()
                    .iter()
                    .map(|cell| cell.id)
                    .collect::<Vec<_>>()
            }
            ActivityKind::Frontier => {
                execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                self.active_cells.iter().copied().collect()
            }
        };
        for id in cell_ids {
            execution_cost.scans = execution_cost.scans.saturating_add(1);
            execution_cost.touch::<Cell>(1);
            self.decay_cell(id, tick);
        }
    }

    fn elapse_fd0_decay(&mut self, tick: i64, work: &mut Work, execution_cost: &mut ExecutionCost) {
        let elapsed = tick.saturating_sub(self.tick);
        let elapsed_u64 = u64::try_from(elapsed).unwrap_or(u64::MAX);
        for index in 0..self.arrows.len() {
            execution_cost.scans = execution_cost.scans.saturating_add(1);
            let (deallocated, zero_delay, active_ticks) = self.arrows.with_mut(index, |arrow| {
                if !arrow.live {
                    return (false, false, 0);
                }
                arrow.participation_level = relax_participation(arrow.participation_level, elapsed);
                let lifetime_remaining = u64::from(arrow.resistance)
                    .saturating_mul(u64::try_from(LOCAL_DECAY_PERIOD).unwrap_or(u64::MAX))
                    .saturating_sub(arrow.decay_load);
                let active_ticks = elapsed_u64.min(lifetime_remaining);
                let total_decay = arrow.decay_load.saturating_add(elapsed_u64);
                let durable_loss = total_decay / u64::try_from(LOCAL_DECAY_PERIOD).unwrap_or(1);
                arrow.decay_load =
                    total_decay % u64::try_from(LOCAL_DECAY_PERIOD).unwrap_or(u64::MAX);
                let was_live = arrow.live;
                if durable_loss > 0 {
                    decay_arrow(arrow, u32::try_from(durable_loss).unwrap_or(u32::MAX));
                }
                (was_live && !arrow.live, arrow.delay == 0, active_ticks)
            });
            work.total = work.total.saturating_add(active_ticks);
            execution_cost.touch::<Arrow>(1);
            if deallocated && zero_delay {
                self.zero_delay_live_arrows = self.zero_delay_live_arrows.saturating_sub(1);
            }
            if deallocated {
                work.total = work.total.saturating_add(1);
                work.physical_deallocations = work.physical_deallocations.saturating_add(1);
            }
        }
    }

    fn propose_local_arrows(
        &mut self,
        source: CellId,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        phase: i32,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) {
        execution_cost.allocations = execution_cost.allocations.saturating_add(1);
        let source_slot = self.cell_slot(source).unwrap();
        let source_position = self.cells.get(source_slot.0).position;
        execution_cost.touch::<Cell>(1);
        let mut targets = self
            .cells
            .values()
            .iter()
            .filter_map(|cell| {
                execution_cost.touch::<Cell>(1);
                let distance = cell.position.saturating_sub(source_position).abs();
                (cell.id != source
                    && cell.live
                    && (1..=LOCAL_VARIATION_RADIUS).contains(&distance)
                    && !self.arrows.values().iter().any(|arrow| {
                        execution_cost.touch::<Arrow>(1);
                        arrow.live && arrow.from == source && arrow.to == cell.id
                    }))
                .then_some((cell.physical_id, cell.id, distance))
            })
            .collect::<Vec<_>>();
        targets.sort_by_key(|(physical_id, _, _)| *physical_id);
        for (_, target, distance) in targets {
            let id = self.add_arrow(ArrowSpec {
                from: source,
                to: target,
                delay: i64::from(distance.max(1)),
                phase: 0,
                coupling: 1,
                resistance: 1,
                mode: TransmissionMode::Drive,
            });
            let slot = self.arrow_slot(id).unwrap();
            self.arrows.with_mut(slot.0, |arrow| {
                if arrow.generation == Generation(1) {
                    arrow.generation =
                        Generation(u32::try_from(id.0).unwrap_or(u32::MAX).saturating_add(2));
                }
            });
            work.total = work.total.saturating_add(1);
            work.local_structural_proposals = work.local_structural_proposals.saturating_add(1);
            if self.trace_physics {
                physical_trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase,
                    event: PhysicalEvent::Proposal {
                        arrow: id,
                        from: source,
                        to: target,
                    },
                });
            }
        }
    }

    fn decay_cell(&mut self, cell: CellId, tick: i64) {
        let slot = self.cell_slot(cell).unwrap();
        let state = self.cells.with_mut(slot.0, |target| {
            let elapsed = tick.saturating_sub(target.last_update_tick);
            if elapsed > 0 {
                let decay = i32::try_from(elapsed).unwrap_or(i32::MAX);
                target.state = if target.state > 0 {
                    target.state.saturating_sub(decay).max(0)
                } else {
                    target.state.saturating_add(decay).min(0)
                };
                target.last_update_tick = tick;
            }
            target.state
        });
        if state == 0 {
            self.active_cells.remove(&cell);
        } else {
            self.active_cells.insert(cell);
        }
    }

    fn require_cell(&self, id: CellId) {
        assert!(
            self.cell_slot(id).is_some(),
            "cell must belong to this substrate"
        );
    }

    fn require_cell_result(&self, id: CellId) -> Result<(), CheckpointError> {
        self.cell_slot(id)
            .map(|_| ())
            .ok_or(CheckpointError::MissingCell(id))
    }

    fn cell_slot(&self, id: CellId) -> Option<CellSlot> {
        usize::try_from(id.0)
            .ok()
            .and_then(|index| self.cell_slots.get(index))
            .copied()
            .flatten()
    }

    fn arrow_slot(&self, id: ArrowId) -> Option<ArrowSlot> {
        usize::try_from(id.0)
            .ok()
            .and_then(|index| self.arrow_slots.get(index))
            .copied()
            .flatten()
    }

    pub fn cell_reference(&self, id: CellId) -> CellRef {
        let slot = self
            .cell_slot(id)
            .expect("stored CELL identity must resolve");
        CellRef {
            arena: self.arena,
            id,
            generation: self.cells.get(slot.0).generation,
        }
    }

    pub fn arrow_reference(&self, id: ArrowId) -> ArrowRef {
        let slot = self
            .arrow_slot(id)
            .expect("stored ARROW identity must resolve");
        let arrow = self.arrows.get(slot.0);
        ArrowRef {
            arena: self.arena,
            id,
            generation: arrow.generation,
        }
    }

    pub fn local_participation(&self, id: ArrowId) -> u64 {
        self.arrows
            .get(self.arrow_slot(id).expect("ARROW identity must resolve").0)
            .participation_level
    }

    pub fn local_plastic_support(&self, id: ArrowId) -> u64 {
        self.arrows
            .get(self.arrow_slot(id).expect("ARROW identity must resolve").0)
            .plastic_support
    }

    pub fn local_pressure_load(&self, id: ArrowId) -> u64 {
        self.local_decay_load(id)
    }

    pub fn local_decay_load(&self, id: ArrowId) -> u64 {
        self.arrows
            .get(self.arrow_slot(id).expect("ARROW identity must resolve").0)
            .decay_load
    }

    fn rebuild_slot_maps(&mut self) {
        self.cell_slots.fill(None);
        for (index, cell) in self.cells.values().iter().enumerate() {
            self.cell_slots[cell.id.0 as usize] = Some(CellSlot(index));
        }
        self.arrow_slots.fill(None);
        for (index, arrow) in self.arrows.values().iter().enumerate() {
            self.arrow_slots[arrow.id.0 as usize] = Some(ArrowSlot(index));
        }
    }

    fn rebuild_mechanical_indexes(&mut self) {
        self.outgoing_index = vec![Vec::new(); self.cell_slots.len()];
        for arrow in self.arrows.values() {
            self.outgoing_index[arrow.from.0 as usize].push(arrow.id);
        }
        for outgoing in &mut self.outgoing_index {
            outgoing.sort_unstable();
        }
        self.active_cells = self
            .cells
            .values()
            .into_iter()
            .filter(|cell| cell.state != 0)
            .map(|cell| cell.id)
            .collect();
        self.zero_delay_live_arrows = self
            .arrows
            .values()
            .into_iter()
            .filter(|arrow| arrow.live && arrow.delay == 0)
            .count();
    }

    fn resident_bytes(&self) -> usize {
        self.cells.len() * std::mem::size_of::<Cell>()
            + self.arrows.len() * std::mem::size_of::<Arrow>()
    }

    fn mechanical_resident_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            .saturating_add(self.cells.resident_bytes())
            .saturating_add(self.arrows.resident_bytes())
            .saturating_add(
                self.cell_slots
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Option<CellSlot>>()),
            )
            .saturating_add(
                self.arrow_slots
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Option<ArrowSlot>>()),
            )
            .saturating_add(self.pending.resident_bytes())
            .saturating_add(
                self.outgoing_index
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Vec<ArrowId>>()),
            )
            .saturating_add(
                self.outgoing_index
                    .iter()
                    .map(|ids| {
                        ids.capacity()
                            .saturating_mul(std::mem::size_of::<ArrowId>())
                    })
                    .sum::<usize>(),
            )
            .saturating_add(
                self.resident_arenas
                    .capacity()
                    .saturating_mul(std::mem::size_of::<ResidentArenaId>()),
            )
            .saturating_add(
                self.active_cells.len().saturating_mul(
                    std::mem::size_of::<CellId>() + 3 * std::mem::size_of::<usize>(),
                ),
            )
    }
}

fn decay_arrow(arrow: &mut Arrow, amount: u32) {
    arrow.resistance = arrow.resistance.saturating_sub(amount);
    if arrow.resistance == 0 && arrow.live {
        arrow.live = false;
        arrow.generation = Generation(arrow.generation.0.wrapping_add(1));
        arrow.participation_level = 0;
        arrow.plastic_support = 0;
        arrow.decay_load = 0;
    }
}

fn relax_participation(mut level: u64, elapsed: i64) -> u64 {
    for _ in 0..u64::try_from(elapsed).unwrap_or(u64::MAX) {
        level =
            level.saturating_mul(PARTICIPATION_RELAX_NUMERATOR) / PARTICIPATION_RELAX_DENOMINATOR;
    }
    level
}

fn pressure_epoch(tick: i64) -> i64 {
    tick.div_euclid(LOCAL_DECAY_PERIOD)
        .saturating_mul(LOCAL_DECAY_PERIOD)
}

fn transmission_mode_byte(mode: TransmissionMode) -> u8 {
    match mode {
        TransmissionMode::Drive => 0,
        TransmissionMode::Modulatory => 1,
    }
}

fn transmission_mode_from_byte(mode: u8) -> Result<TransmissionMode, CheckpointError> {
    match mode {
        0 => Ok(TransmissionMode::Drive),
        1 => Ok(TransmissionMode::Modulatory),
        other => Err(CheckpointError::UnsupportedTransmissionMode(other)),
    }
}

fn transmission_trigger_byte(trigger: TransmissionTrigger) -> u8 {
    match trigger {
        TransmissionTrigger::SourceFires => 0,
        TransmissionTrigger::QualifiedLocalParticipation => 1,
    }
}

fn transmission_trigger_from_byte(trigger: u8) -> Result<TransmissionTrigger, CheckpointError> {
    match trigger {
        0 => Ok(TransmissionTrigger::SourceFires),
        1 => Ok(TransmissionTrigger::QualifiedLocalParticipation),
        other => Err(CheckpointError::UnsupportedTransmissionMode(other)),
    }
}

fn validate_manifest(manifest: &BodyVersion, body: &ArenaBody) -> Result<(), CheckpointError> {
    let hash = body.content_hash()?;
    let matches = manifest.arenas.len() == 1
        && manifest.arenas[0].arena == body.arena
        && manifest.arenas[0].block == hash;
    if matches {
        Ok(())
    } else {
        Err(CheckpointError::ManifestMismatch)
    }
}

fn checkpoint_put_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_put_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_len_u32(length: usize) -> Result<u32, CheckpointError> {
    u32::try_from(length).map_err(|_| CheckpointError::InvalidCheckpoint)
}

fn checkpoint_len_u64(length: usize) -> Result<u64, CheckpointError> {
    u64::try_from(length).map_err(|_| CheckpointError::InvalidCheckpoint)
}

fn checkpoint_put_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_put_i32(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_put_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_put_i64(bytes: &mut Vec<u8>, value: i64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_put_optional_tick(bytes: &mut Vec<u8>, tick: Option<i64>) {
    bytes.push(u8::from(tick.is_some()));
    checkpoint_put_i64(bytes, tick.unwrap_or_default());
}

fn encode_input(bytes: &mut Vec<u8>, input: &SpikeInput) {
    checkpoint_put_i64(bytes, input.arrival_tick);
    checkpoint_put_i32(bytes, input.phase);
    checkpoint_put_u64(bytes, input.origin_physical);
    checkpoint_put_u64(bytes, input.target.0);
    checkpoint_put_i32(bytes, input.impulse);
}

fn decode_input(cursor: &mut CheckpointCursor<'_>) -> Result<SpikeInput, CheckpointError> {
    Ok(SpikeInput {
        arrival_tick: cursor.i64()?,
        phase: cursor.i32()?,
        origin_physical: cursor.u64()?,
        target: CellId(cursor.u64()?),
        impulse: cursor.i32()?,
    })
}

fn encode_crossing(bytes: &mut Vec<u8>, crossing: &Crossing) {
    checkpoint_put_i64(bytes, crossing.tick);
    checkpoint_put_u64(bytes, crossing.from_physical);
    checkpoint_put_u64(bytes, crossing.to_physical);
    checkpoint_put_i16(bytes, crossing.from_region);
    checkpoint_put_i16(bytes, crossing.to_region);
    checkpoint_put_i32(bytes, crossing.impulse);
}

fn decode_crossing(cursor: &mut CheckpointCursor<'_>) -> Result<Crossing, CheckpointError> {
    Ok(Crossing {
        tick: cursor.i64()?,
        from_physical: cursor.u64()?,
        to_physical: cursor.u64()?,
        from_region: cursor.i16()?,
        to_region: cursor.i16()?,
        impulse: cursor.i32()?,
    })
}

fn encode_spike(bytes: &mut Vec<u8>, spike: &Spike) {
    checkpoint_put_i64(bytes, spike.arrival_tick);
    checkpoint_put_i32(bytes, spike.phase);
    checkpoint_put_u64(bytes, spike.origin_physical);
    checkpoint_put_u64(bytes, spike.target.0);
    checkpoint_put_u32(bytes, spike.target_generation.0);
    checkpoint_put_i32(bytes, spike.impulse);
    checkpoint_put_u64(bytes, spike.serial);
    bytes.push(u8::from(spike.arrow.is_some()));
    let (arrow, generation) = spike.arrow.unwrap_or((ArrowId(0), Generation(0)));
    checkpoint_put_u64(bytes, arrow.0);
    checkpoint_put_u32(bytes, generation.0);
}

fn decode_spike(cursor: &mut CheckpointCursor<'_>) -> Result<Spike, CheckpointError> {
    let arrival_tick = cursor.i64()?;
    let phase = cursor.i32()?;
    let origin_physical = cursor.u64()?;
    let target = CellId(cursor.u64()?);
    let target_generation = Generation(cursor.u32()?);
    let impulse = cursor.i32()?;
    let serial = cursor.u64()?;
    let arrow_present = cursor.u8()?;
    if arrow_present > 1 {
        return Err(CheckpointError::InvalidCheckpoint);
    }
    let arrow_id = ArrowId(cursor.u64()?);
    let arrow_generation = Generation(cursor.u32()?);
    Ok(Spike {
        arrival_tick,
        phase,
        origin_physical,
        target,
        target_generation,
        impulse,
        serial,
        arrow: (arrow_present == 1).then_some((arrow_id, arrow_generation)),
    })
}

struct CheckpointCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> CheckpointCursor<'a> {
    fn new(bytes: &'a [u8], offset: usize) -> Self {
        Self { bytes, offset }
    }

    fn take<const N: usize>(&mut self) -> Result<[u8; N], CheckpointError> {
        let end = self
            .offset
            .checked_add(N)
            .ok_or(CheckpointError::Truncated)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(CheckpointError::Truncated)?;
        self.offset = end;
        value.try_into().map_err(|_| CheckpointError::Truncated)
    }

    fn bytes(&mut self, length: usize) -> Result<&'a [u8], CheckpointError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(CheckpointError::Truncated)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(CheckpointError::Truncated)?;
        self.offset = end;
        Ok(value)
    }

    fn finish(&self) -> Result<(), CheckpointError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(CheckpointError::TrailingBytes)
        }
    }

    fn u8(&mut self) -> Result<u8, CheckpointError> {
        Ok(self.take::<1>()?[0])
    }

    fn u16(&mut self) -> Result<u16, CheckpointError> {
        Ok(u16::from_le_bytes(self.take()?))
    }

    fn i16(&mut self) -> Result<i16, CheckpointError> {
        Ok(i16::from_le_bytes(self.take()?))
    }

    fn u32(&mut self) -> Result<u32, CheckpointError> {
        Ok(u32::from_le_bytes(self.take()?))
    }

    fn i32(&mut self) -> Result<i32, CheckpointError> {
        Ok(i32::from_le_bytes(self.take()?))
    }

    fn u64(&mut self) -> Result<u64, CheckpointError> {
        Ok(u64::from_le_bytes(self.take()?))
    }

    fn i64(&mut self) -> Result<i64, CheckpointError> {
        Ok(i64::from_le_bytes(self.take()?))
    }

    fn usize_from_u32(&mut self) -> Result<usize, CheckpointError> {
        usize::try_from(self.u32()?).map_err(|_| CheckpointError::InvalidCheckpoint)
    }

    fn usize_from_u64(&mut self) -> Result<usize, CheckpointError> {
        usize::try_from(self.u64()?).map_err(|_| CheckpointError::InvalidCheckpoint)
    }

    fn array_32(&mut self) -> Result<[u8; 32], CheckpointError> {
        self.take()
    }

    fn optional_tick(&mut self) -> Result<Option<i64>, CheckpointError> {
        let present = self.u8()?;
        let tick = self.i64()?;
        match present {
            0 => Ok(None),
            1 => Ok(Some(tick)),
            _ => Err(CheckpointError::InvalidCheckpoint),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn substrate(arrow_resistance: u32) -> (PlasticSubstrate, CellId, CellId, ArrowId) {
        let mut substrate = PlasticSubstrate::with_capacity(ArenaId(42), 8, 8);
        let source = substrate.add_cell(CellSpec {
            physical_id: 900,
            position: 10,
            region: 0,
            threshold: 1,
            resistance: 10,
        });
        let target = substrate.add_cell(CellSpec {
            physical_id: 100,
            position: 20,
            region: 1,
            threshold: 1,
            resistance: 10,
        });
        let arrow = substrate.add_arrow(ArrowSpec {
            from: source,
            to: target,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: arrow_resistance,
            mode: TransmissionMode::Drive,
        });
        (substrate, source, target, arrow)
    }

    fn input(target: CellId, tick: i64) -> SpikeInput {
        SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 7,
            target,
            impulse: 1,
        }
    }

    fn physical_work(work: Work) -> (u64, u64, u64, u64, u64) {
        (
            work.drive_deliveries,
            work.modulatory_deliveries,
            work.local_return_updates,
            work.local_structural_proposals,
            work.physical_deallocations,
        )
    }

    fn differential_body() -> (PlasticSubstrate, CellId) {
        let mut body = PlasticSubstrate::with_capacity(ArenaId(700), 16, 32);
        let cells = (0..8)
            .map(|index| {
                body.add_cell(CellSpec {
                    physical_id: 10_000 + index,
                    position: (index as i32) * 10,
                    region: if index < 6 { 0 } else { 1 },
                    threshold: if index == 0 { 2 } else { 1 },
                    resistance: 20,
                })
            })
            .collect::<Vec<_>>();
        let arrows = [
            (0, 1, 0, -3, TransmissionMode::Drive),
            (0, 2, 1, 2, TransmissionMode::Drive),
            (1, 3, 65, 0, TransmissionMode::Drive),
            (2, 3, 2, 1, TransmissionMode::Drive),
            (3, 4, 1, 0, TransmissionMode::Drive),
            (4, 0, 1, 4, TransmissionMode::Modulatory),
            (3, 6, 3, 0, TransmissionMode::Drive),
            (4, 7, 4, 0, TransmissionMode::Drive),
        ];
        for (from, to, delay, phase, mode) in arrows {
            body.add_arrow(ArrowSpec {
                from: cells[from],
                to: cells[to],
                delay,
                phase,
                coupling: 1,
                resistance: 40,
                mode,
            });
        }
        (body, cells[0])
    }

    fn assert_physical_equivalence(
        reference: &PlasticSubstrate,
        reference_result: &RunResult,
        candidate: &PlasticSubstrate,
        candidate_result: &RunResult,
    ) {
        assert_eq!(candidate_result.crossings, reference_result.crossings);
        assert_eq!(
            physical_work(candidate_result.work),
            physical_work(reference_result.work)
        );
        assert_eq!(
            candidate_result.work.physical_total(),
            reference_result.work.physical_total()
        );
        assert_eq!(candidate.clock(), reference.clock());
        assert_eq!(
            candidate.clock().pressure_phase(),
            reference.clock().pressure_phase()
        );
        assert_eq!(
            candidate.canonical_body_bytes(991).unwrap(),
            reference.canonical_body_bytes(991).unwrap()
        );
        assert_eq!(
            candidate_result.naturally_quiescent,
            reference_result.naturally_quiescent
        );
        assert_eq!(
            candidate_result.physical_trace,
            reference_result.physical_trace
        );
    }

    #[test]
    fn r1_r5_mechanical_prefixes_preserve_physics() {
        let configs = [
            MechanicalConfig::R1,
            MechanicalConfig::R2,
            MechanicalConfig::R3,
            MechanicalConfig::R4,
            MechanicalConfig::R5,
        ];
        for origin in [0, 130, 260, 390] {
            let (base, source) = differential_body();
            let arrivals = [
                SpikeInput {
                    arrival_tick: origin,
                    phase: 0,
                    origin_physical: 91,
                    target: source,
                    impulse: 1,
                },
                SpikeInput {
                    arrival_tick: origin,
                    phase: 1,
                    origin_physical: 92,
                    target: source,
                    impulse: 1,
                },
                SpikeInput {
                    arrival_tick: origin + 70,
                    phase: -1,
                    origin_physical: 93,
                    target: source,
                    impulse: 2,
                },
            ];
            let mut reference = base.clone();
            for arrival in arrivals {
                reference.enter(arrival);
            }
            let canonical_pending = reference
                .live_checkpoint(990)
                .unwrap()
                .canonical_bytes()
                .unwrap();
            let reference_result = reference.propagate();
            for config in configs {
                let mut candidate = base.clone();
                candidate.reconfigure_mechanics(config);
                for arrival in arrivals {
                    candidate.enter(arrival);
                }
                assert_eq!(
                    candidate
                        .live_checkpoint(990)
                        .unwrap()
                        .canonical_bytes()
                        .unwrap(),
                    canonical_pending,
                    "canonical pending activity differs for {config:?}"
                );
                let candidate_result = candidate.propagate();
                assert_physical_equivalence(
                    &reference,
                    &reference_result,
                    &candidate,
                    &candidate_result,
                );

                let reference_pressure = {
                    let mut value = reference.clone();
                    value.advance_time(reference.clock().tick + 100)
                };
                let candidate_pressure = {
                    let mut value = candidate.clone();
                    value.advance_time(candidate.clock().tick + 100)
                };
                assert_eq!(
                    physical_work(candidate_pressure),
                    physical_work(reference_pressure),
                    "pressure work differs for {config:?}"
                );
            }
        }
    }

    #[test]
    fn resident_partition_preserves_identity_pending_order_and_physics() {
        let (base, source) = differential_body();
        let arrivals = [
            SpikeInput {
                arrival_tick: 0,
                phase: 0,
                origin_physical: 91,
                target: source,
                impulse: 1,
            },
            SpikeInput {
                arrival_tick: 0,
                phase: 1,
                origin_physical: 92,
                target: source,
                impulse: 1,
            },
            SpikeInput {
                arrival_tick: 70,
                phase: -1,
                origin_physical: 93,
                target: source,
                impulse: 2,
            },
        ];
        let mut one_arena = base.clone();
        one_arena.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
        let durable_reference = one_arena.cell_reference(source);
        let mut partitioned = base;
        partitioned.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
        partitioned.repartition_resident(&[
            ResidentArenaId(0),
            ResidentArenaId(1),
            ResidentArenaId(2),
            ResidentArenaId(3),
            ResidentArenaId(0),
            ResidentArenaId(1),
            ResidentArenaId(2),
            ResidentArenaId(3),
        ]);
        assert_eq!(partitioned.resident_arena_count(), 4);
        assert_eq!(partitioned.cell_reference(source), durable_reference);
        assert_eq!(
            partitioned.canonical_body_bytes(992).unwrap(),
            one_arena.canonical_body_bytes(992).unwrap()
        );
        for arrival in arrivals {
            one_arena.enter(arrival);
            partitioned.enter(arrival);
        }
        assert_eq!(
            partitioned
                .live_checkpoint(993)
                .unwrap()
                .canonical_bytes()
                .unwrap(),
            one_arena
                .live_checkpoint(993)
                .unwrap()
                .canonical_bytes()
                .unwrap()
        );
        let one_arena_result = one_arena.propagate();
        let partitioned_result = partitioned.propagate();
        assert!(partitioned_result.execution_cost.arena_hops > 0);
        assert_physical_equivalence(
            &one_arena,
            &one_arena_result,
            &partitioned,
            &partitioned_result,
        );
    }

    #[test]
    fn r5_batches_safe_same_tick_activity_without_changing_physics() {
        let mut base = PlasticSubstrate::with_capacity(ArenaId(701), 2, 2);
        let target = base.add_cell(CellSpec {
            physical_id: 55_000,
            position: 0,
            region: 0,
            threshold: 100,
            resistance: 20,
        });
        let arrivals = (0..100)
            .map(|serial| SpikeInput {
                arrival_tick: 10,
                phase: serial % 3,
                origin_physical: 70_000 + serial as u64,
                target,
                impulse: 1,
            })
            .collect::<Vec<_>>();

        let mut scalar = base.clone();
        scalar.reconfigure_mechanics(MechanicalConfig::R4);
        let scalar_result = scalar.arrive(&arrivals, 1);

        let mut batched = base;
        batched.reconfigure_mechanics(MechanicalConfig::R5);
        let batched_result = batched.arrive(&arrivals, 1);

        assert_physical_equivalence(&scalar, &scalar_result, &batched, &batched_result);
        assert!(
            batched_result.execution_cost.queue_ops < scalar_result.execution_cost.queue_ops,
            "batched scheduler must consume fewer queue operations"
        );
    }

    #[test]
    fn r4_soa_compaction_and_restart_preserve_stable_identity() {
        for config in [MechanicalConfig::R4, MechanicalConfig::R5] {
            let (mut ordinary, source, target, arrow) = substrate(50);
            ordinary.reconfigure_mechanics(config);
            ordinary.add_arrow(ArrowSpec {
                from: target,
                to: source,
                delay: 1,
                phase: 0,
                coupling: 0,
                resistance: 50,
                mode: TransmissionMode::Drive,
            });
            ordinary.enter(input(source, 5));
            let checkpoint = ordinary.live_checkpoint(811).unwrap();
            let mut restored =
                PlasticSubstrate::from_live_checkpoint_with_mechanics(checkpoint, config).unwrap();
            let source_reference = restored.cell_reference(source);
            let arrow_reference = restored.arrow_reference(arrow);
            let source_slot_before = restored.resolve_cell(source_reference).unwrap();
            let arrow_slot_before = restored.resolve_arrow(arrow_reference).unwrap();
            restored.compact_resident();
            assert_ne!(
                restored.resolve_cell(source_reference).unwrap(),
                source_slot_before
            );
            assert_ne!(
                restored.resolve_arrow(arrow_reference).unwrap(),
                arrow_slot_before
            );
            let restored_result = restored.propagate();
            let ordinary_result = ordinary.propagate();
            assert_physical_equivalence(&ordinary, &ordinary_result, &restored, &restored_result);
        }
    }

    #[test]
    fn compaction_changes_slots_not_physics() {
        let (original, source, _, _) = substrate(50);
        let reference = original.cell_reference(source);
        let before = original.resolve_cell(reference).unwrap();
        let mut compacted = original.clone();
        compacted.compact_resident();
        let after = compacted.resolve_cell(reference).unwrap();
        assert_ne!(before, after);

        let mut ordinary = original;
        let ordinary_result = ordinary.arrive(&[input(source, 0)], 1);
        let compacted_result = compacted.arrive(&[input(source, 0)], 1);
        assert_physical_equivalence(&ordinary, &ordinary_result, &compacted, &compacted_result);
    }

    #[test]
    fn canonical_body_round_trip_is_structurally_exact() {
        let (substrate, _, _, _) = substrate(50);
        let bytes = substrate.canonical_body_bytes(3).unwrap();
        let restored = PlasticSubstrate::from_body_bytes(&bytes).unwrap();
        assert_eq!(restored.canonical_body_bytes(3).unwrap(), bytes);
    }

    #[test]
    fn quiescent_checkpoint_preserves_clock_phase_and_future_behavior() {
        let (mut substrate, source, _, _) = substrate(50);
        substrate.advance_time(23);
        let checkpoint = substrate.quiescent_checkpoint(4).unwrap();
        assert_eq!(checkpoint.clock.pressure_phase(), 3);
        let bytes = checkpoint.canonical_bytes().unwrap();
        let decoded = QuiescentCheckpoint::decode(&bytes).unwrap();
        assert_eq!(decoded.canonical_bytes().unwrap(), bytes);
        let mut restored = PlasticSubstrate::from_quiescent_checkpoint(decoded).unwrap();
        assert_eq!(restored.clock(), substrate.clock());
        let substrate_result = substrate.arrive(&[input(source, 24)], 1);
        let restored_result = restored.arrive(&[input(source, 24)], 1);
        assert_physical_equivalence(&substrate, &substrate_result, &restored, &restored_result);
    }

    #[test]
    fn live_checkpoint_preserves_pending_activity_and_load_availability() {
        let (mut substrate, source, _, _) = substrate(50);
        substrate.enter(input(source, 5));
        substrate.register_pending_load(PendingLoad {
            arena: ArenaId(99),
            version: ContentHash([3; 32]),
            issue_tick: 0,
            availability_tick: Some(7),
            waiting_arrivals: vec![input(source, 8)],
        });
        let checkpoint = substrate.live_checkpoint(5).unwrap();
        let bytes = checkpoint.canonical_bytes().unwrap();
        let mut corrupt = bytes.clone();
        *corrupt.last_mut().unwrap() ^= 1;
        assert_eq!(
            LiveCheckpoint::decode(&corrupt),
            Err(CheckpointError::Checksum)
        );
        let decoded = LiveCheckpoint::decode(&bytes).unwrap();
        assert_eq!(decoded.canonical_bytes().unwrap(), bytes);
        let mut restored = PlasticSubstrate::from_live_checkpoint(decoded).unwrap();
        assert_eq!(restored, substrate);
        let restored_result = restored.propagate();
        let substrate_result = substrate.propagate();
        assert_physical_equivalence(&substrate, &substrate_result, &restored, &restored_result);
    }

    #[test]
    fn reused_identity_rejects_stale_generation() {
        let (mut substrate, source, target, arrow) = substrate(1);
        let stale = substrate.arrow_reference(arrow);
        substrate.advance_time(10);
        assert_eq!(substrate.resolve_arrow(stale), None);
        let reused = substrate.add_arrow(ArrowSpec {
            from: source,
            to: target,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 4,
            mode: TransmissionMode::Drive,
        });
        assert_eq!(reused, arrow);
        let current = substrate.arrow_reference(reused);
        assert_ne!(current.generation, stale.generation);
        assert_eq!(substrate.resolve_arrow(stale), None);
        assert!(substrate.resolve_arrow(current).is_some());
    }

    #[test]
    fn durable_body_rejects_stale_internal_references() {
        let (substrate, _, _, _) = substrate(4);
        let mut body = substrate.arena_body(1);
        body.arrows[0].from.generation = Generation(99);
        let bytes = body.canonical_bytes().unwrap();
        assert!(matches!(
            PlasticSubstrate::from_body_bytes(&bytes),
            Err(CheckpointError::StaleCellReference(_))
        ));
    }

    #[test]
    fn input_capacity_rejects_batches_atomically() {
        let (substrate, source, _, _) = substrate(50);
        let mut runtime = BoundaryRuntime::new(substrate, 1, 2, 4).unwrap();
        runtime.enqueue(input(source, 0)).unwrap();
        let before = runtime.clone();
        assert_eq!(
            runtime.enqueue_batch(&[input(source, 1), input(source, 2)]),
            Err(BoundaryError::InputFull {
                capacity: 2,
                occupied: 1,
                attempted: 2,
            })
        );
        assert_eq!(runtime, before);
    }

    #[test]
    fn output_backpressure_is_bounded_transactional_and_fifo() {
        let (substrate, source, _, _) = substrate(50);
        let mut runtime = BoundaryRuntime::new(substrate, 1, 4, 1).unwrap();
        runtime.enqueue(input(source, 0)).unwrap();
        let first = runtime.run_until_quiescent().unwrap();
        assert_eq!(first.produced_outputs, 1);
        runtime.enqueue(input(source, 2)).unwrap();
        let before = runtime.clone();
        assert_eq!(
            runtime.run_until_quiescent(),
            Err(BoundaryError::OutputFull {
                capacity: 1,
                occupied: 1,
                required: 1,
            })
        );
        assert_eq!(runtime, before);
        let first_output = runtime.drain_output(1);
        assert_eq!(first_output.len(), 1);
        runtime.run_until_quiescent().unwrap();
        let second_output = runtime.drain_output(1);
        assert_eq!(second_output.len(), 1);
        assert!(first_output[0].tick < second_output[0].tick);
    }

    #[test]
    fn output_batch_larger_than_capacity_changes_nothing() {
        let mut substrate = PlasticSubstrate::with_capacity(ArenaId(55), 4, 4);
        let source = substrate.add_cell(CellSpec {
            physical_id: 1,
            position: 0,
            region: 0,
            threshold: 1,
            resistance: 10,
        });
        for physical_id in [2, 3] {
            let target = substrate.add_cell(CellSpec {
                physical_id,
                position: physical_id as i32,
                region: 1,
                threshold: 1,
                resistance: 10,
            });
            substrate.add_arrow(ArrowSpec {
                from: source,
                to: target,
                delay: 1,
                phase: 0,
                coupling: 1,
                resistance: 50,
                mode: TransmissionMode::Drive,
            });
        }
        let mut runtime = BoundaryRuntime::new(substrate, 1, 2, 1).unwrap();
        runtime.enqueue(input(source, 0)).unwrap();
        let before = runtime.clone();
        assert_eq!(
            runtime.run_until_quiescent(),
            Err(BoundaryError::OutputBatchTooLarge {
                capacity: 1,
                required: 2,
            })
        );
        assert_eq!(runtime, before);
    }

    #[test]
    fn buffered_path_and_live_checkpoint_preserve_exact_behavior() {
        let (mut direct, source, _, _) = substrate(50);
        let inputs = [input(source, 0), input(source, 2)];
        let expected = direct.arrive(&inputs, 1);

        let (buffered, buffered_source, _, _) = substrate(50);
        assert_eq!(source, buffered_source);
        let mut runtime = BoundaryRuntime::new(buffered, 1, 4, 4).unwrap();
        runtime.enqueue(inputs[0]).unwrap();
        runtime.run_until_quiescent().unwrap();
        runtime.enqueue(inputs[1]).unwrap();
        let checkpoint = runtime.live_checkpoint(6).unwrap();
        let bytes = checkpoint.canonical_bytes().unwrap();
        let decoded = BoundaryLiveCheckpoint::decode(&bytes).unwrap();
        assert_eq!(decoded.canonical_bytes().unwrap(), bytes);
        let mut restored = BoundaryRuntime::from_live_checkpoint(decoded).unwrap();
        assert_eq!(restored, runtime);

        let first = restored.drain_all_output();
        let second_run = restored.run_until_quiescent().unwrap();
        let mut actual = first;
        actual.extend(restored.drain_all_output());
        assert_eq!(actual, expected.crossings);
        assert!(second_run.naturally_quiescent);
        assert_eq!(restored.substrate(), &direct);
    }
}
