#![forbid(unsafe_code)]
//! Single-file physical runtime: state, local transitions, and crossings.

use std::cmp::Ordering;
use truelearner_arena_format::{
    ArenaBody, ArenaVersion, BodyVersion, DurableArrow, DurableCell, FormatError,
};
pub use truelearner_arena_format::{
    ArenaId, ArrowId, ArrowRef, CellId, CellRef, ContentHash, Generation,
};

const LOCAL_WINDOW: i64 = 4;
const LOCAL_RETURN_STRENGTH: u32 = 3;
const UNSUPPORTED_USE_PRESSURE: u32 = 1;
const ORDINARY_PRESSURE_PERIOD: i64 = 10;
const LOCAL_VARIATION_RADIUS: i32 = 2;
const COUPLING_PLASTICITY_CEILING: u32 = 16;
const DEFAULT_CELL_CAPACITY: u32 = 65_536;
const DEFAULT_ARROW_CAPACITY: u32 = 262_144;
const CHECKPOINT_VERSION: u16 = 1;
const QUIESCENT_MAGIC: &[u8; 8] = b"TLQUIE01";
const LIVE_MAGIC: &[u8; 8] = b"TLLIVE01";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CellSlot(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArrowSlot(pub usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhysicalClock {
    pub tick: i64,
}

impl PhysicalClock {
    pub fn pressure_phase(self) -> i64 {
        self.tick.rem_euclid(ORDINARY_PRESSURE_PERIOD)
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
    eligible_until: Option<i64>,
    mode: TransmissionMode,
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    total: u64,
    pub drive_deliveries: u64,
    pub modulatory_deliveries: u64,
    pub local_return_updates: u64,
    pub local_structural_proposals: u64,
    pub physical_deallocations: u64,
}

impl Work {
    pub fn total(&self) -> u64 {
        self.total
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunResult {
    pub crossings: Vec<Crossing>,
    pub work: Work,
    pub naturally_quiescent: bool,
    pub resident_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlasticSubstrate {
    arena: ArenaId,
    cells: Vec<Cell>,
    cell_slots: Vec<Option<CellSlot>>,
    arrows: Vec<Arrow>,
    arrow_slots: Vec<Option<ArrowSlot>>,
    cell_capacity: u32,
    arrow_capacity: u32,
    pending: Vec<Spike>,
    tick: i64,
    next_serial: u64,
    pressure_tick: i64,
    pending_loads: Vec<PendingLoad>,
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
    eligible_until: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckpointError {
    NotQuiescent,
    Format(FormatError),
    UnsupportedTransmissionMode(u8),
    MissingCell(CellId),
    MissingArrow(ArrowId),
    ManifestMismatch,
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
            checkpoint_put_optional_tick(&mut payload, arrow.eligible_until);
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
                eligible_until: transient.optional_tick()?,
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
        Self {
            arena,
            cells: Vec::new(),
            cell_slots: Vec::new(),
            arrows: Vec::new(),
            arrow_slots: Vec::new(),
            cell_capacity,
            arrow_capacity,
            pending: Vec::new(),
            tick: 0,
            next_serial: 0,
            pressure_tick: 0,
            pending_loads: Vec::new(),
        }
    }

    pub fn clock(&self) -> PhysicalClock {
        PhysicalClock { tick: self.tick }
    }

    pub fn resolve_cell(&self, reference: CellRef) -> Option<CellSlot> {
        if reference.arena != self.arena {
            return None;
        }
        let slot = self.cell_slot(reference.id)?;
        let cell = &self.cells[slot.0];
        (cell.live && cell.generation == reference.generation).then_some(slot)
    }

    pub fn resolve_arrow(&self, reference: ArrowRef) -> Option<ArrowSlot> {
        if reference.arena != self.arena {
            return None;
        }
        let slot = self.arrow_slot(reference.id)?;
        let arrow = &self.arrows[slot.0];
        (arrow.live && arrow.generation == reference.generation).then_some(slot)
    }

    pub fn add_cell(&mut self, spec: CellSpec) -> CellId {
        assert!(spec.threshold > 0, "threshold must be physically positive");
        assert!(
            self.cells
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
        id
    }

    pub fn add_arrow(&mut self, spec: ArrowSpec) -> ArrowId {
        self.require_cell(spec.from);
        self.require_cell(spec.to);
        assert!(spec.delay >= 0, "delay must not run backward in time");
        let source_slot = self
            .cell_slot(spec.from)
            .expect("required CELL must resolve");
        let source_generation = self.cells[source_slot.0].generation;
        let reusable = self.arrows.iter().position(|arrow| !arrow.live);
        if reusable.is_none() {
            assert!(
                self.arrow_slots.len() < self.arrow_capacity as usize,
                "resident arena has no free ARROW identity"
            );
        }
        let (id, slot, generation) = if let Some(index) = reusable {
            let prior = &self.arrows[index];
            (prior.id, ArrowSlot(index), prior.generation)
        } else {
            let id = ArrowId(self.arrow_slots.len() as u64);
            (id, ArrowSlot(self.arrows.len()), Generation(1))
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
            eligible_until: None,
            mode: spec.mode,
        };
        if slot.0 < self.arrows.len() {
            self.arrows[slot.0] = arrow;
            self.arrow_slots[id.0 as usize] = Some(slot);
        } else {
            self.arrows.push(arrow);
            self.arrow_slots.push(Some(slot));
        }
        id
    }

    pub fn enter(&mut self, input: SpikeInput) {
        self.require_cell(input.target);
        assert!(
            input.arrival_tick >= self.tick,
            "physical arrivals cannot precede current substrate time"
        );
        self.pending.push(Spike {
            arrival_tick: input.arrival_tick,
            phase: input.phase,
            origin_physical: input.origin_physical,
            target: input.target,
            target_generation: self.cells[self.cell_slot(input.target).unwrap().0].generation,
            impulse: input.impulse,
            serial: self.next_serial,
            arrow: None,
        });
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
        self.elapse_to(tick, &mut work);
        self.tick = tick;
        work
    }

    pub fn arena_body(&self, version: u64) -> ArenaBody {
        let minimum_position = self
            .cells
            .iter()
            .map(|cell| cell.position)
            .min()
            .unwrap_or(0);
        let maximum_position = self
            .cells
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
                eligible_until: None,
                mode,
            });
        }
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
                .iter()
                .all(|cell| cell.state == 0 && cell.refractory_until <= self.tick)
            && self
                .arrows
                .iter()
                .all(|arrow| arrow.eligible_until.is_none());
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
        validate_manifest(&checkpoint.body_version, &checkpoint.body)?;
        let mut substrate = Self::from_arena_body(checkpoint.body)?;
        substrate.tick = checkpoint.clock.tick;
        substrate.pressure_tick = pressure_epoch(checkpoint.clock.tick);
        for cell in &mut substrate.cells {
            cell.last_update_tick = checkpoint.clock.tick;
            cell.refractory_until = checkpoint.clock.tick;
        }
        Ok(substrate)
    }

    pub fn live_checkpoint(&self, body_version: u64) -> Result<LiveCheckpoint, CheckpointError> {
        Ok(LiveCheckpoint {
            body_version: self.body_version(body_version)?,
            body: self.arena_body(body_version),
            clock: self.clock(),
            cells: self
                .cells
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
                .iter()
                .map(|arrow| ArrowRuntime {
                    id: arrow.id,
                    eligible_until: arrow.eligible_until,
                })
                .collect(),
            pending: self.pending.clone(),
            next_serial: self.next_serial,
            pending_loads: self.pending_loads.clone(),
        })
    }

    pub fn from_live_checkpoint(checkpoint: LiveCheckpoint) -> Result<Self, CheckpointError> {
        validate_manifest(&checkpoint.body_version, &checkpoint.body)?;
        let mut substrate = Self::from_arena_body(checkpoint.body)?;
        substrate.tick = checkpoint.clock.tick;
        substrate.pressure_tick = pressure_epoch(checkpoint.clock.tick);
        for runtime in checkpoint.cells {
            let slot = substrate
                .cell_slot(runtime.id)
                .ok_or(CheckpointError::MissingCell(runtime.id))?;
            let cell = &mut substrate.cells[slot.0];
            cell.state = runtime.state;
            cell.last_update_tick = runtime.last_update_tick;
            cell.refractory_until = runtime.refractory_until;
        }
        for runtime in checkpoint.arrows {
            let slot = substrate
                .arrow_slot(runtime.id)
                .ok_or(CheckpointError::MissingArrow(runtime.id))?;
            substrate.arrows[slot.0].eligible_until = runtime.eligible_until;
        }
        substrate.pending = checkpoint.pending;
        substrate.next_serial = checkpoint.next_serial;
        substrate.pending_loads = checkpoint.pending_loads;
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
        self.cells.sort_by_key(|cell| std::cmp::Reverse(cell.id));
        self.arrows
            .sort_by_key(|arrow| (!arrow.live, std::cmp::Reverse(arrow.id)));
        self.rebuild_slot_maps();
    }

    pub fn propagate(&mut self) -> RunResult {
        let mut crossings = Vec::new();
        let mut work = Work::default();
        while !self.pending.is_empty() {
            let mut first = 0;
            for candidate in 1..self.pending.len() {
                work.total = work.total.saturating_add(1);
                if self.spike_order(candidate, first) == Ordering::Less {
                    first = candidate;
                }
            }
            let spike = self.pending.remove(first);
            let external_arrival = spike.arrow.is_none();
            self.elapse_to(spike.arrival_tick, &mut work);
            self.tick = spike.arrival_tick;
            work.total = work.total.saturating_add(2);

            if let Some((arrow_id, generation)) = spike.arrow {
                let Some(arrow_slot) = self.arrow_slot(arrow_id) else {
                    continue;
                };
                let arrow = &self.arrows[arrow_slot.0];
                if !arrow.live || arrow.generation != generation {
                    continue;
                }
            }
            let Some(target_slot) = self.cell_slot(spike.target) else {
                continue;
            };
            let target = &self.cells[target_slot.0];
            if !target.live || target.generation != spike.target_generation {
                continue;
            }

            let mode = spike.arrow.map_or(TransmissionMode::Drive, |(arrow, _)| {
                self.arrows[self.arrow_slot(arrow).unwrap().0].mode
            });
            if mode == TransmissionMode::Modulatory {
                work.total = work.total.saturating_add(1);
                work.modulatory_deliveries = work.modulatory_deliveries.saturating_add(1);
                self.apply_modulatory_return(spike.target, self.tick, &mut work);
                continue;
            }
            work.total = work.total.saturating_add(3);
            work.drive_deliveries = work.drive_deliveries.saturating_add(1);
            self.decay_cell(spike.target, self.tick);
            let target_slot = self.cell_slot(spike.target).unwrap();
            let target = &mut self.cells[target_slot.0];
            target.state = target.state.saturating_add(spike.impulse);
            let fires = self.tick >= target.refractory_until && target.state >= target.threshold;
            if !fires {
                continue;
            }

            target.state = 0;
            target.refractory_until = self.tick.saturating_add(1);
            work.total = work.total.saturating_add(1);
            let source = spike.target;
            let origin_physical = target.physical_id;
            let source_generation = target.generation;
            if external_arrival {
                self.propose_local_arrows(source, &mut work);
            }
            let mut outgoing = self
                .arrows
                .iter()
                .map(|arrow| (arrow.id, arrow.clone()))
                .collect::<Vec<_>>();
            outgoing.sort_by_key(|(id, _)| *id);
            for (arrow_id, arrow) in outgoing {
                work.total = work.total.saturating_add(1);
                if !arrow.live
                    || arrow.from != source
                    || arrow.source_generation != source_generation
                {
                    continue;
                }
                let from_slot = self.cell_slot(arrow.from).unwrap();
                let to_slot = self.cell_slot(arrow.to).unwrap();
                let from = &self.cells[from_slot.0];
                let to = &self.cells[to_slot.0];
                if from.region != to.region {
                    crossings.push(Crossing {
                        tick: self.tick,
                        from_physical: from.physical_id,
                        to_physical: to.physical_id,
                        from_region: from.region,
                        to_region: to.region,
                        impulse: arrow.coupling,
                    });
                }
                let arrow_slot = self.arrow_slot(arrow_id).unwrap();
                let live_arrow = &mut self.arrows[arrow_slot.0];
                live_arrow.eligible_until = Some(self.tick.saturating_add(LOCAL_WINDOW));
                work.total = work.total.saturating_add(2);
                self.pending.push(Spike {
                    arrival_tick: self.tick.saturating_add(arrow.delay),
                    phase: arrow.phase,
                    origin_physical,
                    target: arrow.to,
                    target_generation: to.generation,
                    impulse: arrow.coupling,
                    serial: self.next_serial,
                    arrow: Some((arrow_id, arrow.generation)),
                });
                self.next_serial = self.next_serial.wrapping_add(1);
            }
        }
        RunResult {
            crossings,
            work,
            naturally_quiescent: self.pending.is_empty(),
            resident_bytes: self.resident_bytes(),
        }
    }

    fn apply_modulatory_return(&mut self, cell: CellId, tick: i64, work: &mut Work) {
        for arrow in &mut self.arrows {
            if arrow.live
                && arrow.from == cell
                && arrow.eligible_until.is_some_and(|end| tick <= end)
            {
                work.total = work.total.saturating_add(3);
                work.local_return_updates = work.local_return_updates.saturating_add(1);
                let prior_resistance = arrow.resistance;
                arrow.resistance = arrow.resistance.saturating_add(LOCAL_RETURN_STRENGTH);
                if prior_resistance <= COUPLING_PLASTICITY_CEILING && arrow.coupling > 0 {
                    arrow.coupling = arrow.coupling.saturating_add(1).min(2);
                }
                arrow.eligible_until = None;
            }
        }
    }

    fn elapse_to(&mut self, tick: i64, work: &mut Work) {
        let pressure_steps = tick.saturating_sub(self.pressure_tick) / ORDINARY_PRESSURE_PERIOD;
        if pressure_steps > 0 {
            let amount = u32::try_from(pressure_steps).unwrap_or(u32::MAX);
            for arrow in &mut self.arrows {
                if arrow.live {
                    let was_live = arrow.live;
                    pressure_arrow(arrow, amount);
                    work.total = work.total.saturating_add(1);
                    if was_live && !arrow.live {
                        work.total = work.total.saturating_add(1);
                        work.physical_deallocations = work.physical_deallocations.saturating_add(1);
                    }
                }
            }
            self.pressure_tick = self
                .pressure_tick
                .saturating_add(pressure_steps.saturating_mul(ORDINARY_PRESSURE_PERIOD));
        }
        for arrow in &mut self.arrows {
            if arrow.live && arrow.eligible_until.is_some_and(|end| end < tick) {
                let was_live = arrow.live;
                pressure_arrow(arrow, UNSUPPORTED_USE_PRESSURE);
                arrow.eligible_until = None;
                work.total = work.total.saturating_add(1);
                if was_live && !arrow.live {
                    work.total = work.total.saturating_add(1);
                    work.physical_deallocations = work.physical_deallocations.saturating_add(1);
                }
            }
        }
        let cell_ids = self.cells.iter().map(|cell| cell.id).collect::<Vec<_>>();
        for id in cell_ids {
            self.decay_cell(id, tick);
        }
    }

    fn propose_local_arrows(&mut self, source: CellId, work: &mut Work) {
        let source_slot = self.cell_slot(source).unwrap();
        let source_position = self.cells[source_slot.0].position;
        let mut targets = self
            .cells
            .iter()
            .filter_map(|cell| {
                let distance = cell.position.saturating_sub(source_position).abs();
                (cell.id != source
                    && cell.live
                    && (1..=LOCAL_VARIATION_RADIUS).contains(&distance)
                    && !self
                        .arrows
                        .iter()
                        .any(|arrow| arrow.live && arrow.from == source && arrow.to == cell.id))
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
            if self.arrows[slot.0].generation == Generation(1) {
                self.arrows[slot.0].generation =
                    Generation(u32::try_from(id.0).unwrap_or(u32::MAX).saturating_add(2));
            }
            work.total = work.total.saturating_add(1);
            work.local_structural_proposals = work.local_structural_proposals.saturating_add(1);
        }
    }

    fn decay_cell(&mut self, cell: CellId, tick: i64) {
        let slot = self.cell_slot(cell).unwrap();
        let target = &mut self.cells[slot.0];
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
            generation: self.cells[slot.0].generation,
        }
    }

    pub fn arrow_reference(&self, id: ArrowId) -> ArrowRef {
        let slot = self
            .arrow_slot(id)
            .expect("stored ARROW identity must resolve");
        let arrow = &self.arrows[slot.0];
        ArrowRef {
            arena: self.arena,
            id,
            generation: arrow.generation,
        }
    }

    fn rebuild_slot_maps(&mut self) {
        self.cell_slots.fill(None);
        for (index, cell) in self.cells.iter().enumerate() {
            self.cell_slots[cell.id.0 as usize] = Some(CellSlot(index));
        }
        self.arrow_slots.fill(None);
        for (index, arrow) in self.arrows.iter().enumerate() {
            self.arrow_slots[arrow.id.0 as usize] = Some(ArrowSlot(index));
        }
    }

    fn spike_order(&self, left: usize, right: usize) -> Ordering {
        let left = &self.pending[left];
        let right = &self.pending[right];
        (
            left.arrival_tick,
            left.phase,
            left.origin_physical,
            self.cells[self.cell_slot(left.target).unwrap().0].physical_id,
            left.serial,
        )
            .cmp(&(
                right.arrival_tick,
                right.phase,
                right.origin_physical,
                self.cells[self.cell_slot(right.target).unwrap().0].physical_id,
                right.serial,
            ))
    }

    fn resident_bytes(&self) -> usize {
        self.cells.len() * std::mem::size_of::<Cell>()
            + self.arrows.len() * std::mem::size_of::<Arrow>()
    }
}

fn pressure_arrow(arrow: &mut Arrow, amount: u32) {
    arrow.resistance = arrow.resistance.saturating_sub(amount);
    if arrow.resistance == 0 && arrow.live {
        arrow.live = false;
        arrow.generation = Generation(arrow.generation.0.wrapping_add(1));
        arrow.eligible_until = None;
    }
}

fn pressure_epoch(tick: i64) -> i64 {
    tick.div_euclid(ORDINARY_PRESSURE_PERIOD)
        .saturating_mul(ORDINARY_PRESSURE_PERIOD)
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
        assert_eq!(
            ordinary.arrive(&[input(source, 0)], 1),
            compacted.arrive(&[input(source, 0)], 1)
        );
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
        assert_eq!(
            substrate.arrive(&[input(source, 24)], 1),
            restored.arrive(&[input(source, 24)], 1)
        );
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
        let decoded = LiveCheckpoint::decode(&bytes).unwrap();
        assert_eq!(decoded.canonical_bytes().unwrap(), bytes);
        let mut restored = PlasticSubstrate::from_live_checkpoint(decoded).unwrap();
        assert_eq!(restored, substrate);
        assert_eq!(restored.propagate(), substrate.propagate());
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
}
