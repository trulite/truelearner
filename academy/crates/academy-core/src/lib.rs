#![forbid(unsafe_code)]
//! Headless Academy state and the explicit physical boundary to TrueLearner.
//!
//! Semantic curriculum and evaluator information lives here. Only canonical
//! raster samples and `SpikeInput` batches cross into `truelearner-core`.

use font8x8::{UnicodeFonts, BASIC_FONTS};
use image::{codecs::png::PngEncoder, DynamicImage, ExtendedColorType, ImageEncoder};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::sync::mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError};
use std::thread::{self, JoinHandle};
use truelearner_arena_format::FormatError;
use truelearner_core::{
    ArenaId, ArrowSpec, BoundaryError, BoundaryLiveCheckpoint, BoundaryRuntime, CellId, CellSpec,
    ContentHash, Crossing, MechanicalConfig, ResidentArenaId, SpikeInput, TransmissionMode,
};

mod a1;
pub use a1::{
    A1Experience, A1ExperienceKind, A1ProbeFamily, A1ReplayOutcome, A1WorldObservation,
    GenuineTeachingLab, TeachingCase,
};

pub const SURFACE_WIDTH: u32 = 640;
pub const SURFACE_HEIGHT: u32 = 360;
pub const COMMAND_CAPACITY: usize = 16;
pub const EVENT_CAPACITY: usize = 32;
const GLYPH_FIRST: u8 = 32;
const GLYPH_LAST: u8 = 126;
const GLYPH_COUNT: usize = (GLYPH_LAST - GLYPH_FIRST + 1) as usize;
const SENSOR_PHYSICAL_BASE: u64 = 10_000;
const MOTOR_PHYSICAL_BASE: u64 = 20_000;
const OUTPUT_PHYSICAL_BASE: u64 = 30_000;
const OUTWARD_REGION: i16 = 1;
const INPUT_CAPACITY: usize = 4096;
const OUTPUT_CAPACITY: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualSurface {
    width: u32,
    height: u32,
    rgba_pixels: Vec<u8>,
}

impl VisualSurface {
    pub fn new(width: u32, height: u32, rgba: [u8; 4]) -> Self {
        let pixel_count = usize::try_from(width)
            .expect("surface width must fit memory")
            .saturating_mul(usize::try_from(height).expect("surface height must fit memory"));
        let mut rgba_pixels = Vec::with_capacity(pixel_count.saturating_mul(4));
        for _ in 0..pixel_count {
            rgba_pixels.extend_from_slice(&rgba);
        }
        Self {
            width,
            height,
            rgba_pixels,
        }
    }

    pub fn blank() -> Self {
        Self::new(SURFACE_WIDTH, SURFACE_HEIGHT, [246, 244, 238, 255])
    }

    pub fn from_rgba(width: u32, height: u32, rgba_pixels: Vec<u8>) -> Result<Self, SurfaceError> {
        let expected = usize::try_from(width)
            .map_err(|_| SurfaceError::Dimensions)?
            .checked_mul(usize::try_from(height).map_err(|_| SurfaceError::Dimensions)?)
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or(SurfaceError::Dimensions)?;
        if expected != rgba_pixels.len() || width == 0 || height == 0 {
            return Err(SurfaceError::Dimensions);
        }
        Ok(Self {
            width,
            height,
            rgba_pixels,
        })
    }

    pub fn from_encoded_image(bytes: &[u8]) -> Result<Self, SurfaceError> {
        let decoded = image::load_from_memory(bytes)
            .map_err(|error| SurfaceError::Decode(error.to_string()))?;
        Ok(Self::from_dynamic(decoded))
    }

    pub fn from_dynamic(image: DynamicImage) -> Self {
        let rgba = image.to_rgba8();
        Self {
            width: rgba.width(),
            height: rgba.height(),
            rgba_pixels: rgba.into_raw(),
        }
    }

    pub fn render_text(text: &str) -> Self {
        let mut surface = Self::blank();
        let mut x = 20_u32;
        let mut y = 24_u32;
        for character in text.chars().take(2_000) {
            if character == '\n' || x.saturating_add(10) >= surface.width {
                x = 20;
                y = y.saturating_add(12);
                if character == '\n' {
                    continue;
                }
            }
            if y.saturating_add(10) >= surface.height {
                break;
            }
            if let Some(bitmap) = BASIC_FONTS.get(character) {
                for (row, bits) in bitmap.iter().copied().enumerate() {
                    for column in 0..8_u32 {
                        if bits & (1 << column) != 0 {
                            surface.set_pixel(
                                x.saturating_add(column),
                                y.saturating_add(u32::try_from(row).unwrap_or(0)),
                                [28, 36, 43, 255],
                            );
                        }
                    }
                }
            }
            x = x.saturating_add(9);
        }
        surface
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn rgba_pixels(&self) -> &[u8] {
        &self.rgba_pixels
    }

    pub fn clear(&mut self, rgba: [u8; 4]) {
        for pixel in self.rgba_pixels.chunks_exact_mut(4) {
            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, rgba: [u8; 4]) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (usize::try_from(y)
            .unwrap_or(usize::MAX)
            .saturating_mul(usize::try_from(self.width).unwrap_or(usize::MAX))
            .saturating_add(usize::try_from(x).unwrap_or(usize::MAX)))
        .saturating_mul(4);
        if let Some(pixel) = self.rgba_pixels.get_mut(index..index.saturating_add(4)) {
            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn draw_line(&mut self, from: (u32, u32), to: (u32, u32), rgba: [u8; 4], radius: u32) {
        let (mut x0, mut y0) = (i64::from(from.0), i64::from(from.1));
        let (x1, y1) = (i64::from(to.0), i64::from(to.1));
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;
        loop {
            for oy in 0..=radius.saturating_mul(2) {
                for ox in 0..=radius.saturating_mul(2) {
                    let px = x0 + i64::from(ox) - i64::from(radius);
                    let py = y0 + i64::from(oy) - i64::from(radius);
                    if px >= 0 && py >= 0 {
                        self.set_pixel(px as u32, py as u32, rgba);
                    }
                }
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let twice = error.saturating_mul(2);
            if twice >= dy {
                error += dy;
                x0 += sx;
            }
            if twice <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }

    pub fn png_bytes(&self) -> Result<Vec<u8>, SurfaceError> {
        let mut bytes = Vec::new();
        PngEncoder::new(&mut bytes)
            .write_image(
                &self.rgba_pixels,
                self.width,
                self.height,
                ExtendedColorType::Rgba8,
            )
            .map_err(|error| SurfaceError::Encode(error.to_string()))?;
        Ok(bytes)
    }

    pub fn fingerprint(&self) -> String {
        short_hash(ContentHash::of(&self.rgba_pixels).as_bytes())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SurfaceError {
    Dimensions,
    Decode(String),
    Encode(String),
}

impl fmt::Display for SurfaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dimensions => write!(formatter, "invalid raster dimensions"),
            Self::Decode(message) => write!(formatter, "image decode failed: {message}"),
            Self::Encode(message) => write!(formatter, "image encode failed: {message}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExperienceMode {
    Teach,
    Probe,
    Transfer,
    Retention,
}

impl ExperienceMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Teach => "Teach",
            Self::Probe => "Probe",
            Self::Transfer => "Transfer",
            Self::Retention => "Retention",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityStatus {
    Unknown,
    Emerging,
    Acquired,
    General,
    Stable,
    Automatic,
}

impl CapabilityStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Unknown => "UNKNOWN",
            Self::Emerging => "EMERGING",
            Self::Acquired => "ACQUIRED",
            Self::General => "GENERAL",
            Self::Stable => "STABLE",
            Self::Automatic => "AUTOMATIC",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceTotals {
    pub teach_experiences: u32,
    pub fresh_passes: u32,
    pub fresh_attempts: u32,
    pub transfer_passes: u32,
    pub transfer_attempts: u32,
    pub retention_passes: u32,
    pub retention_attempts: u32,
    pub successful_work: Vec<u64>,
}

impl EvidenceTotals {
    pub fn fresh_ratio(&self) -> f64 {
        ratio(self.fresh_passes, self.fresh_attempts)
    }

    pub fn transfer_ratio(&self) -> f64 {
        ratio(self.transfer_passes, self.transfer_attempts)
    }

    pub fn retention_ratio(&self) -> f64 {
        ratio(self.retention_passes, self.retention_attempts)
    }

    pub fn median_work(&self) -> Option<u64> {
        let mut values = self.successful_work.clone();
        values.sort_unstable();
        values.get(values.len() / 2).copied()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capability {
    pub id: String,
    pub title: String,
    pub description: String,
    pub prerequisites: Vec<String>,
    pub status: CapabilityStatus,
    pub evidence: EvidenceTotals,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityGraph {
    capabilities: BTreeMap<String, Capability>,
}

impl Default for CapabilityGraph {
    fn default() -> Self {
        let definitions = [
            (
                "interaction-response",
                "Interaction / response",
                "Produces a physical outward response to admitted activity.",
                &[][..],
            ),
            (
                "copy-symbol",
                "Copy simple symbol",
                "Returns a freshly presented simple symbol.",
                &["interaction-response"][..],
            ),
            (
                "distinguish-symbols",
                "Distinguish two symbols",
                "Responds differently to physically different symbols.",
                &["copy-symbol"][..],
            ),
            (
                "repeat-sequence",
                "Repeat short sequence",
                "Preserves order across a short physical sequence.",
                &["copy-symbol"][..],
            ),
            (
                "novel-binding",
                "Novel symbol binding",
                "Acquires a new physical symbol-to-symbol regularity.",
                &["distinguish-symbols"][..],
            ),
            (
                "retrieve-binding",
                "Retrieve binding",
                "Reuses a previously acquired binding.",
                &["novel-binding"][..],
            ),
            (
                "reverse-binding",
                "Reverse binding",
                "Uses a learned relation in the opposite direction.",
                &["retrieve-binding"][..],
            ),
            (
                "delayed-binding",
                "Delayed binding",
                "Retains a binding across intervening experience.",
                &["retrieve-binding"][..],
            ),
            (
                "replace-binding",
                "Replace binding",
                "Lets changed experience replace an old relation.",
                &["delayed-binding"][..],
            ),
            (
                "sequence-continuation",
                "Sequence continuation",
                "Continues a short repeated physical pattern.",
                &["repeat-sequence"][..],
            ),
            (
                "visual-difference",
                "Visual difference",
                "Responds to a changed raster surface.",
                &["interaction-response"][..],
            ),
            (
                "visual-symbol",
                "Visual ↔ symbol binding",
                "Binds a raster regularity to a physical symbol.",
                &["visual-difference", "novel-binding"][..],
            ),
            (
                "short-recall",
                "Short conversational recall",
                "Reuses relevant recent physical interaction.",
                &["delayed-binding"][..],
            ),
            (
                "composition",
                "Recursive composition",
                "Treats a matured organization as an ordinary participant.",
                &["sequence-continuation", "visual-symbol"][..],
            ),
        ];
        let capabilities = definitions
            .into_iter()
            .map(|(id, title, description, prerequisites)| {
                (
                    id.to_string(),
                    Capability {
                        id: id.to_string(),
                        title: title.to_string(),
                        description: description.to_string(),
                        prerequisites: prerequisites
                            .iter()
                            .map(|item| (*item).to_string())
                            .collect(),
                        status: CapabilityStatus::Unknown,
                        evidence: EvidenceTotals::default(),
                    },
                )
            })
            .collect();
        Self { capabilities }
    }
}

impl CapabilityGraph {
    pub fn capabilities(&self) -> impl Iterator<Item = &Capability> {
        self.capabilities.values()
    }

    pub fn capability(&self, id: &str) -> Option<&Capability> {
        self.capabilities.get(id)
    }

    pub fn record(&mut self, ids: &[String], mode: ExperienceMode, passed: bool, work: u64) {
        for id in ids {
            let Some(capability) = self.capabilities.get_mut(id) else {
                continue;
            };
            match mode {
                ExperienceMode::Teach => {
                    capability.evidence.teach_experiences =
                        capability.evidence.teach_experiences.saturating_add(1);
                }
                ExperienceMode::Probe => {
                    capability.evidence.fresh_attempts =
                        capability.evidence.fresh_attempts.saturating_add(1);
                    if passed {
                        capability.evidence.fresh_passes =
                            capability.evidence.fresh_passes.saturating_add(1);
                    }
                }
                ExperienceMode::Transfer => {
                    capability.evidence.transfer_attempts =
                        capability.evidence.transfer_attempts.saturating_add(1);
                    if passed {
                        capability.evidence.transfer_passes =
                            capability.evidence.transfer_passes.saturating_add(1);
                    }
                }
                ExperienceMode::Retention => {
                    capability.evidence.retention_attempts =
                        capability.evidence.retention_attempts.saturating_add(1);
                    if passed {
                        capability.evidence.retention_passes =
                            capability.evidence.retention_passes.saturating_add(1);
                    }
                }
            }
            if passed {
                capability.evidence.successful_work.push(work);
            }
            capability.status = derive_status(&capability.evidence);
        }
    }

    pub fn stable_count(&self) -> usize {
        self.capabilities
            .values()
            .filter(|capability| {
                matches!(
                    capability.status,
                    CapabilityStatus::Stable | CapabilityStatus::Automatic
                )
            })
            .count()
    }

    pub fn frontier_count(&self) -> usize {
        self.capabilities
            .values()
            .filter(|capability| {
                matches!(
                    capability.status,
                    CapabilityStatus::Emerging
                        | CapabilityStatus::Acquired
                        | CapabilityStatus::General
                )
            })
            .count()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicalInput {
    Text(String),
    Raster(VisualSurface),
}

impl PhysicalInput {
    pub fn summary(&self) -> String {
        match self {
            Self::Text(text) => format!("Text · {} byte(s)", text.len()),
            Self::Raster(surface) => format!(
                "Raster · {}×{} · {}",
                surface.width(),
                surface.height(),
                surface.fingerprint()
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionRequest {
    pub mode: ExperienceMode,
    pub input: PhysicalInput,
    pub capability_ids: Vec<String>,
    pub expected_text: Option<String>,
    pub academy_note: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpikeRecord {
    pub arrival_tick: i64,
    pub phase: i32,
    pub origin_physical: u64,
    pub target: u64,
    pub impulse: i32,
}

impl From<SpikeInput> for SpikeRecord {
    fn from(input: SpikeInput) -> Self {
        Self {
            arrival_tick: input.arrival_tick,
            phase: input.phase,
            origin_physical: input.origin_physical,
            target: input.target.0,
            impulse: input.impulse,
        }
    }
}

impl From<SpikeRecord> for SpikeInput {
    fn from(record: SpikeRecord) -> Self {
        Self {
            arrival_tick: record.arrival_tick,
            phase: record.phase,
            origin_physical: record.origin_physical,
            target: CellId(record.target),
            impulse: record.impulse,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossingRecord {
    pub tick: i64,
    pub from_physical: u64,
    pub to_physical: u64,
    pub from_region: i16,
    pub to_region: i16,
    pub impulse: i32,
}

impl From<Crossing> for CrossingRecord {
    fn from(crossing: Crossing) -> Self {
        Self {
            tick: crossing.tick,
            from_physical: crossing.from_physical,
            to_physical: crossing.to_physical,
            from_region: crossing.from_region,
            to_region: crossing.to_region,
            impulse: crossing.impulse,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmissionRecord {
    pub sequence: u64,
    pub mode: ExperienceMode,
    pub admitted_at_tick: i64,
    pub input_summary: String,
    pub spikes: Vec<SpikeRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceRecord {
    pub id: u64,
    pub mode: ExperienceMode,
    pub admission: AdmissionRecord,
    pub academy_note: String,
    pub capability_ids: Vec<String>,
    pub probe_passed: Option<bool>,
    pub organism_text: String,
    pub crossings: Vec<CrossingRecord>,
    pub body_before: String,
    pub body_after: String,
    pub clock_start: i64,
    pub clock_end: i64,
    pub physical_work: u64,
    pub drive_deliveries: u64,
    pub modulatory_deliveries: u64,
    pub plasticity_updates: u64,
    pub proposals: u64,
    pub deallocations: u64,
    pub resident_bytes: usize,
    pub naturally_quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorSnapshot {
    pub body_version: u64,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub pressure_phase: i64,
    pub pending_inputs: usize,
    pub pending_outputs: usize,
    pub resident_arenas: usize,
    pub active_arena_max: u64,
    pub crossing_total: u64,
    pub physical_work_total: u64,
    pub durable_bytes: usize,
    pub last_run_bytes: usize,
    pub last_run_work: u64,
    pub queue_backpressure: u64,
    pub experience_count: u64,
    pub replay_exact: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionSnapshot {
    pub inspector: InspectorSnapshot,
    pub capabilities: CapabilityGraph,
    pub timeline: Vec<ExperienceRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayOutcome {
    pub exact: bool,
    pub expected_body: String,
    pub observed_body: String,
    pub expected_clock: i64,
    pub observed_clock: i64,
    pub expected_work: u64,
    pub observed_work: u64,
}

struct ReplayCase {
    checkpoint: Vec<u8>,
    spikes: Vec<SpikeInput>,
    crossings: Vec<CrossingRecord>,
    body_after: String,
    clock_end: i64,
    work: u64,
}

pub struct AcademySession {
    boundary: BoundaryRuntime,
    sensors: Vec<CellId>,
    placements: Vec<ResidentArenaId>,
    capabilities: CapabilityGraph,
    timeline: VecDeque<ExperienceRecord>,
    body_version: u64,
    experience_sequence: u64,
    crossing_total: u64,
    work_total: u64,
    queue_backpressure: u64,
    last_run_bytes: usize,
    last_run_work: u64,
    last_active_arena_max: u64,
    saved_checkpoint: Option<Vec<u8>>,
    last_replay: Option<ReplayCase>,
    replay_exact: Option<bool>,
}

impl AcademySession {
    pub fn starter() -> Result<Self, AcademyError> {
        let (boundary, sensors, placements) = starter_body()?;
        Ok(Self {
            boundary,
            sensors,
            placements,
            capabilities: CapabilityGraph::default(),
            timeline: VecDeque::with_capacity(256),
            body_version: 0,
            experience_sequence: 0,
            crossing_total: 0,
            work_total: 0,
            queue_backpressure: 0,
            last_run_bytes: 0,
            last_run_work: 0,
            last_active_arena_max: 0,
            saved_checkpoint: None,
            last_replay: None,
            replay_exact: None,
        })
    }

    pub fn interact(
        &mut self,
        request: InteractionRequest,
    ) -> Result<(ExperienceRecord, VisualSurface), AcademyError> {
        let clock_start = self.boundary.substrate().clock().tick;
        let body_before = self.body_fingerprint()?;
        let checkpoint = self
            .boundary
            .live_checkpoint(self.body_version)?
            .canonical_bytes()?;
        let spikes = self.physical_inputs(&request.input, clock_start.saturating_add(1));
        if spikes.is_empty() {
            return Err(AcademyError::EmptyPhysicalInput);
        }
        let admission = AdmissionRecord {
            sequence: self.experience_sequence,
            mode: request.mode,
            admitted_at_tick: spikes[0].arrival_tick,
            input_summary: request.input.summary(),
            spikes: spikes.iter().copied().map(SpikeRecord::from).collect(),
        };
        let result = self.boundary.arrive(&spikes, OUTWARD_REGION)?;
        let crossings = result
            .crossings
            .iter()
            .copied()
            .map(CrossingRecord::from)
            .collect::<Vec<_>>();
        let organism_text = decode_crossings(&result.crossings);
        let body_after = self.body_fingerprint()?;
        if body_after != body_before {
            self.body_version = self.body_version.saturating_add(1);
        }
        let probe_passed = request
            .expected_text
            .as_ref()
            .map(|expected| normalize_text(expected) == normalize_text(&organism_text));
        let record = ExperienceRecord {
            id: self.experience_sequence,
            mode: request.mode,
            admission,
            academy_note: request.academy_note,
            capability_ids: request.capability_ids.clone(),
            probe_passed,
            organism_text,
            crossings: crossings.clone(),
            body_before,
            body_after: body_after.clone(),
            clock_start,
            clock_end: self.boundary.substrate().clock().tick,
            physical_work: result.work.total(),
            drive_deliveries: result.work.drive_deliveries,
            modulatory_deliveries: result.work.modulatory_deliveries,
            plasticity_updates: result.work.local_return_updates,
            proposals: result.work.local_structural_proposals,
            deallocations: result.work.physical_deallocations,
            resident_bytes: result.resident_bytes,
            naturally_quiescent: result.naturally_quiescent,
        };
        self.capabilities.record(
            &request.capability_ids,
            request.mode,
            probe_passed.unwrap_or(false),
            result.work.total(),
        );
        self.crossing_total = self
            .crossing_total
            .saturating_add(result.crossings.len() as u64);
        self.work_total = self.work_total.saturating_add(result.work.total());
        self.last_run_bytes = result.resident_bytes;
        self.last_run_work = result.work.total();
        self.last_active_arena_max = result.execution_cost.active_arena_max;
        self.experience_sequence = self.experience_sequence.saturating_add(1);
        if self.timeline.len() == 256 {
            self.timeline.pop_front();
        }
        self.timeline.push_back(record.clone());
        self.last_replay = Some(ReplayCase {
            checkpoint,
            spikes,
            crossings,
            body_after,
            clock_end: record.clock_end,
            work: record.physical_work,
        });
        self.replay_exact = None;
        let raster = rasterize_crossings(&result.crossings);
        Ok((record, raster))
    }

    pub fn save_checkpoint(&mut self) -> Result<u64, AcademyError> {
        self.saved_checkpoint = Some(
            self.boundary
                .live_checkpoint(self.body_version)?
                .canonical_bytes()?,
        );
        Ok(self.body_version)
    }

    pub fn restore_checkpoint(&mut self) -> Result<u64, AcademyError> {
        let bytes = self
            .saved_checkpoint
            .clone()
            .ok_or(AcademyError::NoCheckpoint)?;
        self.boundary = restore_boundary(&bytes, &self.placements)?;
        self.body_version = self.body_version.saturating_add(1);
        self.replay_exact = None;
        Ok(self.body_version)
    }

    pub fn replay_last(&mut self) -> Result<ReplayOutcome, AcademyError> {
        let replay = self.last_replay.as_ref().ok_or(AcademyError::NoReplay)?;
        let mut boundary = restore_boundary(&replay.checkpoint, &self.placements)?;
        let result = boundary.arrive(&replay.spikes, OUTWARD_REGION)?;
        let observed_body = fingerprint_body(&boundary, self.body_version)?;
        let observed_crossings = result
            .crossings
            .iter()
            .copied()
            .map(CrossingRecord::from)
            .collect::<Vec<_>>();
        let outcome = ReplayOutcome {
            exact: observed_crossings == replay.crossings
                && observed_body == replay.body_after
                && boundary.substrate().clock().tick == replay.clock_end
                && result.work.total() == replay.work,
            expected_body: replay.body_after.clone(),
            observed_body,
            expected_clock: replay.clock_end,
            observed_clock: boundary.substrate().clock().tick,
            expected_work: replay.work,
            observed_work: result.work.total(),
        };
        self.replay_exact = Some(outcome.exact);
        Ok(outcome)
    }

    pub fn snapshot(&self) -> Result<SessionSnapshot, AcademyError> {
        let body_bytes = self.boundary.substrate().canonical_body_bytes(0)?;
        Ok(SessionSnapshot {
            inspector: InspectorSnapshot {
                body_version: self.body_version,
                body_fingerprint: short_hash(ContentHash::of(&body_bytes).as_bytes()),
                physical_tick: self.boundary.substrate().clock().tick,
                pressure_phase: self.boundary.substrate().clock().pressure_phase(),
                pending_inputs: self.boundary.input_len(),
                pending_outputs: self.boundary.output_len(),
                resident_arenas: self.boundary.substrate().resident_arena_count(),
                active_arena_max: self.last_active_arena_max,
                crossing_total: self.crossing_total,
                physical_work_total: self.work_total,
                durable_bytes: body_bytes.len(),
                last_run_bytes: self.last_run_bytes,
                last_run_work: self.last_run_work,
                queue_backpressure: self.queue_backpressure,
                experience_count: self.experience_sequence,
                replay_exact: self.replay_exact,
            },
            capabilities: self.capabilities.clone(),
            timeline: self.timeline.iter().cloned().collect(),
        })
    }

    fn body_fingerprint(&self) -> Result<String, AcademyError> {
        fingerprint_body(&self.boundary, self.body_version)
    }

    fn physical_inputs(&self, input: &PhysicalInput, start_tick: i64) -> Vec<SpikeInput> {
        match input {
            PhysicalInput::Text(text) => normalize_text(text)
                .bytes()
                .take(256)
                .enumerate()
                .map(|(index, byte)| self.spike_for_byte(byte, start_tick, index))
                .collect(),
            PhysicalInput::Raster(surface) => sample_surface(surface)
                .into_iter()
                .enumerate()
                .map(|(index, byte)| self.spike_for_byte(byte, start_tick, index))
                .collect(),
        }
    }

    fn spike_for_byte(&self, byte: u8, start_tick: i64, index: usize) -> SpikeInput {
        let normalized = byte.clamp(GLYPH_FIRST, GLYPH_LAST);
        let sensor_index = usize::from(normalized - GLYPH_FIRST);
        SpikeInput {
            arrival_tick: start_tick.saturating_add(i64::try_from(index).unwrap_or(i64::MAX)),
            phase: i32::try_from(index % 4).unwrap_or(0),
            origin_physical: 900_000_u64.saturating_add(self.experience_sequence),
            target: self.sensors[sensor_index],
            impulse: 1,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AcademyCommand {
    Interact(InteractionRequest),
    SaveCheckpoint,
    RestoreCheckpoint,
    ReplayLast,
    Shutdown,
}

#[derive(Clone, Debug)]
pub enum AcademyEvent {
    Ready(Box<SessionSnapshot>),
    Completed {
        record: Box<ExperienceRecord>,
        organism_surface: Box<VisualSurface>,
        snapshot: Box<SessionSnapshot>,
    },
    CheckpointSaved {
        body_version: u64,
        snapshot: Box<SessionSnapshot>,
    },
    CheckpointRestored {
        body_version: u64,
        snapshot: Box<SessionSnapshot>,
    },
    ReplayVerified {
        outcome: Box<ReplayOutcome>,
        snapshot: Box<SessionSnapshot>,
    },
    Error(String),
}

pub struct AcademyWorker {
    commands: SyncSender<AcademyCommand>,
    events: Receiver<AcademyEvent>,
    join: Option<JoinHandle<()>>,
}

impl AcademyWorker {
    pub fn spawn() -> Result<Self, AcademyError> {
        let (commands, command_receiver) = mpsc::sync_channel(COMMAND_CAPACITY);
        let (event_sender, events) = mpsc::sync_channel(EVENT_CAPACITY);
        let mut session = AcademySession::starter()?;
        let join = thread::Builder::new()
            .name("truelearner-academy-body".to_string())
            .spawn(move || {
                send_event(
                    &event_sender,
                    session
                        .snapshot()
                        .map(|snapshot| AcademyEvent::Ready(Box::new(snapshot))),
                );
                while let Ok(command) = command_receiver.recv() {
                    match command {
                        AcademyCommand::Interact(request) => {
                            let event =
                                session
                                    .interact(request)
                                    .and_then(|(record, organism_surface)| {
                                        Ok(AcademyEvent::Completed {
                                            record: Box::new(record),
                                            organism_surface: Box::new(organism_surface),
                                            snapshot: Box::new(session.snapshot()?),
                                        })
                                    });
                            send_event(&event_sender, event);
                        }
                        AcademyCommand::SaveCheckpoint => {
                            let event = session.save_checkpoint().and_then(|body_version| {
                                Ok(AcademyEvent::CheckpointSaved {
                                    body_version,
                                    snapshot: Box::new(session.snapshot()?),
                                })
                            });
                            send_event(&event_sender, event);
                        }
                        AcademyCommand::RestoreCheckpoint => {
                            let event = session.restore_checkpoint().and_then(|body_version| {
                                Ok(AcademyEvent::CheckpointRestored {
                                    body_version,
                                    snapshot: Box::new(session.snapshot()?),
                                })
                            });
                            send_event(&event_sender, event);
                        }
                        AcademyCommand::ReplayLast => {
                            let event = session.replay_last().and_then(|outcome| {
                                Ok(AcademyEvent::ReplayVerified {
                                    outcome: Box::new(outcome),
                                    snapshot: Box::new(session.snapshot()?),
                                })
                            });
                            send_event(&event_sender, event);
                        }
                        AcademyCommand::Shutdown => break,
                    }
                }
            })
            .map_err(|error| AcademyError::Worker(error.to_string()))?;
        Ok(Self {
            commands,
            events,
            join: Some(join),
        })
    }

    pub fn try_command(&self, command: AcademyCommand) -> Result<(), WorkerBackpressure> {
        self.commands
            .try_send(command)
            .map_err(|error| match error {
                TrySendError::Full(_) => WorkerBackpressure::Full,
                TrySendError::Disconnected(_) => WorkerBackpressure::Disconnected,
            })
    }

    pub fn try_event(&self) -> Result<Option<AcademyEvent>, WorkerBackpressure> {
        match self.events.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(WorkerBackpressure::Disconnected),
        }
    }
}

impl Drop for AcademyWorker {
    fn drop(&mut self) {
        let _ = self.commands.try_send(AcademyCommand::Shutdown);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerBackpressure {
    Full,
    Disconnected,
}

impl fmt::Display for WorkerBackpressure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full => write!(
                formatter,
                "Academy worker queue is full; wait for the current physical run"
            ),
            Self::Disconnected => write!(formatter, "Academy worker is unavailable"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AcademyError {
    Boundary(BoundaryError),
    Checkpoint(truelearner_core::CheckpointError),
    Format(FormatError),
    EmptyPhysicalInput,
    NoCheckpoint,
    NoReplay,
    Worker(String),
}

impl fmt::Display for AcademyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boundary(error) => write!(formatter, "physical boundary error: {error:?}"),
            Self::Checkpoint(error) => write!(formatter, "checkpoint error: {error:?}"),
            Self::Format(error) => write!(formatter, "body format error: {error:?}"),
            Self::EmptyPhysicalInput => write!(formatter, "the admitted physical input was empty"),
            Self::NoCheckpoint => write!(formatter, "no saved body checkpoint is available"),
            Self::NoReplay => write!(
                formatter,
                "no completed physical interaction is available to replay"
            ),
            Self::Worker(message) => write!(formatter, "worker error: {message}"),
        }
    }
}

impl From<BoundaryError> for AcademyError {
    fn from(error: BoundaryError) -> Self {
        Self::Boundary(error)
    }
}

impl From<truelearner_core::CheckpointError> for AcademyError {
    fn from(error: truelearner_core::CheckpointError) -> Self {
        Self::Checkpoint(error)
    }
}

impl From<FormatError> for AcademyError {
    fn from(error: FormatError) -> Self {
        Self::Format(error)
    }
}

fn starter_body() -> Result<(BoundaryRuntime, Vec<CellId>, Vec<ResidentArenaId>), AcademyError> {
    let mut substrate = truelearner_core::PlasticSubstrate::with_mechanics(
        ArenaId(0),
        1024,
        4096,
        MechanicalConfig::PRODUCTION,
    );
    let mut sensors = Vec::with_capacity(GLYPH_COUNT);
    for index in 0..GLYPH_COUNT {
        let position = i32::try_from(index).unwrap_or(i32::MAX).saturating_mul(6);
        let sensor = substrate.add_cell(CellSpec {
            physical_id: SENSOR_PHYSICAL_BASE.saturating_add(index as u64),
            position,
            region: 0,
            threshold: 1,
            resistance: 64,
        });
        let motor = substrate.add_cell(CellSpec {
            physical_id: MOTOR_PHYSICAL_BASE.saturating_add(index as u64),
            position: position.saturating_add(1),
            region: 0,
            threshold: 1,
            resistance: 64,
        });
        let output = substrate.add_cell(CellSpec {
            physical_id: OUTPUT_PHYSICAL_BASE.saturating_add(index as u64),
            position: position.saturating_add(2),
            region: OUTWARD_REGION,
            threshold: 1,
            resistance: 64,
        });
        substrate.add_arrow(ArrowSpec {
            from: motor,
            to: output,
            delay: 1,
            phase: 2,
            coupling: 1,
            resistance: 64,
            mode: TransmissionMode::Drive,
        });
        substrate.add_arrow(ArrowSpec {
            from: output,
            to: sensor,
            delay: 1,
            phase: 3,
            coupling: 1,
            resistance: 64,
            mode: TransmissionMode::Modulatory,
        });
        sensors.push(sensor);
    }
    let placements = (0..GLYPH_COUNT.saturating_mul(3))
        .map(|index| ResidentArenaId(u32::try_from(index % 8).unwrap_or(0)))
        .collect::<Vec<_>>();
    substrate.repartition_resident(&placements);
    let boundary =
        BoundaryRuntime::new(substrate, OUTWARD_REGION, INPUT_CAPACITY, OUTPUT_CAPACITY)?;
    Ok((boundary, sensors, placements))
}

fn restore_boundary(
    bytes: &[u8],
    placements: &[ResidentArenaId],
) -> Result<BoundaryRuntime, AcademyError> {
    let checkpoint = BoundaryLiveCheckpoint::decode(bytes)?;
    let mut boundary = BoundaryRuntime::from_live_checkpoint(checkpoint)?;
    boundary.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
    boundary.repartition_resident(placements);
    Ok(boundary)
}

fn fingerprint_body(boundary: &BoundaryRuntime, _version: u64) -> Result<String, AcademyError> {
    // Academy lineage labels are not physical state. Differential replay uses
    // a neutral durable version so only the organism body is compared.
    let bytes = boundary.substrate().canonical_body_bytes(0)?;
    Ok(short_hash(ContentHash::of(&bytes).as_bytes()))
}

fn short_hash(hash: &[u8; 32]) -> String {
    hash.iter()
        .take(8)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn normalize_text(text: &str) -> String {
    text.chars()
        .filter_map(|character| {
            if character == '\n' || character == '\t' {
                Some(' ')
            } else if character.is_ascii() && !character.is_ascii_control() {
                Some(character)
            } else {
                None
            }
        })
        .take(256)
        .collect::<String>()
        .trim()
        .to_string()
}

fn sample_surface(surface: &VisualSurface) -> Vec<u8> {
    let x_step = (surface.width / 16).max(1);
    let y_step = (surface.height / 9).max(1);
    let mut samples = Vec::with_capacity(144);
    let mut y = y_step / 2;
    while y < surface.height && samples.len() < 144 {
        let mut x = x_step / 2;
        while x < surface.width && samples.len() < 144 {
            let offset = (usize::try_from(y)
                .unwrap_or(0)
                .saturating_mul(usize::try_from(surface.width).unwrap_or(0))
                .saturating_add(usize::try_from(x).unwrap_or(0)))
            .saturating_mul(4);
            if let Some(pixel) = surface.rgba_pixels.get(offset..offset.saturating_add(4)) {
                let luminance = (u16::from(pixel[0]) * 54
                    + u16::from(pixel[1]) * 183
                    + u16::from(pixel[2]) * 19)
                    / 256;
                let range = u16::from(GLYPH_LAST - GLYPH_FIRST);
                samples.push(GLYPH_FIRST.saturating_add(((luminance * range) / 255) as u8));
            }
            x = x.saturating_add(x_step);
        }
        y = y.saturating_add(y_step);
    }
    samples
}

fn decode_crossings(crossings: &[Crossing]) -> String {
    crossings
        .iter()
        .filter_map(|crossing| {
            let index = crossing.to_physical.checked_sub(OUTPUT_PHYSICAL_BASE)?;
            if index < GLYPH_COUNT as u64 {
                Some(char::from(GLYPH_FIRST.saturating_add(index as u8)))
            } else {
                None
            }
        })
        .collect()
}

fn rasterize_crossings(crossings: &[Crossing]) -> VisualSurface {
    let mut surface = VisualSurface::new(SURFACE_WIDTH, SURFACE_HEIGHT, [24, 31, 37, 255]);
    if crossings.is_empty() {
        return surface;
    }
    let mut previous = (20_u32, SURFACE_HEIGHT / 2);
    for (index, crossing) in crossings.iter().enumerate() {
        let x = 20_u32.saturating_add(
            u32::try_from(index).unwrap_or(u32::MAX).saturating_mul(17) % (SURFACE_WIDTH - 40),
        );
        let y = 30_u32.saturating_add(
            u32::try_from(crossing.to_physical % u64::from(SURFACE_HEIGHT - 60)).unwrap_or(0),
        );
        surface.draw_line(previous, (x, y), [91, 211, 165, 255], 2);
        previous = (x, y);
    }
    surface
}

fn derive_status(evidence: &EvidenceTotals) -> CapabilityStatus {
    let automatic = evidence.retention_passes >= 2
        && evidence.successful_work.len() >= 6
        && evidence.successful_work.last() < evidence.successful_work.first();
    if automatic {
        CapabilityStatus::Automatic
    } else if evidence.retention_passes >= 1 && evidence.fresh_passes >= 2 {
        CapabilityStatus::Stable
    } else if evidence.transfer_passes >= 2 && evidence.transfer_ratio() >= 0.75 {
        CapabilityStatus::General
    } else if evidence.fresh_passes >= 2 && evidence.fresh_ratio() >= 0.75 {
        CapabilityStatus::Acquired
    } else if evidence.teach_experiences > 0 || evidence.fresh_attempts > 0 {
        CapabilityStatus::Emerging
    } else {
        CapabilityStatus::Unknown
    }
}

fn ratio(passes: u32, attempts: u32) -> f64 {
    if attempts == 0 {
        0.0
    } else {
        f64::from(passes) / f64::from(attempts)
    }
}

fn send_event(sender: &SyncSender<AcademyEvent>, event: Result<AcademyEvent, AcademyError>) {
    let event = event.unwrap_or_else(|error| AcademyEvent::Error(error.to_string()));
    let _ = sender.send(event);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raster_is_canonical_and_drawing_is_deterministic() {
        let mut first = VisualSurface::blank();
        first.draw_line((4, 7), (311, 203), [12, 34, 56, 255], 2);
        let mut second = VisualSurface::blank();
        second.draw_line((4, 7), (311, 203), [12, 34, 56, 255], 2);
        assert_eq!(first, second);
        assert_eq!(
            first.rgba_pixels().len(),
            (SURFACE_WIDTH * SURFACE_HEIGHT * 4) as usize
        );
        assert!(!first.png_bytes().unwrap().is_empty());
    }

    #[test]
    fn capability_state_comes_from_evidence() {
        let mut graph = CapabilityGraph::default();
        let ids = vec!["copy-symbol".to_string()];
        graph.record(&ids, ExperienceMode::Teach, false, 100);
        assert_eq!(
            graph.capability("copy-symbol").unwrap().status,
            CapabilityStatus::Emerging
        );
        graph.record(&ids, ExperienceMode::Probe, true, 80);
        graph.record(&ids, ExperienceMode::Probe, true, 60);
        assert_eq!(
            graph.capability("copy-symbol").unwrap().status,
            CapabilityStatus::Acquired
        );
        graph.record(&ids, ExperienceMode::Retention, true, 40);
        assert_eq!(
            graph.capability("copy-symbol").unwrap().status,
            CapabilityStatus::Stable
        );
    }

    #[test]
    fn interaction_records_physical_inputs_and_replays_exactly() {
        let mut session = AcademySession::starter().unwrap();
        let request = InteractionRequest {
            mode: ExperienceMode::Probe,
            input: PhysicalInput::Text("dax".to_string()),
            capability_ids: vec![
                "interaction-response".to_string(),
                "copy-symbol".to_string(),
            ],
            expected_text: Some("dax".to_string()),
            academy_note: "fresh probe".to_string(),
        };
        let (record, _) = session.interact(request).unwrap();
        assert!(!record.admission.spikes.is_empty());
        assert!(record.naturally_quiescent);
        let replay = session.replay_last().unwrap();
        assert!(replay.exact, "{replay:?}");
        assert!(session.snapshot().unwrap().inspector.replay_exact.unwrap());
    }

    #[test]
    fn checkpoint_restores_and_keeps_production_mechanics() {
        let mut session = AcademySession::starter().unwrap();
        let saved = session.save_checkpoint().unwrap();
        let restored = session.restore_checkpoint().unwrap();
        assert!(restored > saved);
        assert_eq!(
            session.boundary.substrate().mechanical_config(),
            MechanicalConfig::PRODUCTION
        );
        assert_eq!(session.boundary.substrate().resident_arena_count(), 8);
    }

    #[test]
    fn worker_channels_are_bounded_and_keep_body_off_caller_thread() {
        let worker = AcademyWorker::spawn().unwrap();
        let mut ready = None;
        for _ in 0..100_000 {
            if let Some(event) = worker.try_event().unwrap() {
                ready = Some(event);
                break;
            }
            std::thread::yield_now();
        }
        assert!(matches!(ready, Some(AcademyEvent::Ready(_))));
    }
}
