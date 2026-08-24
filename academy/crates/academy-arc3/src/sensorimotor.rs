use crate::ARC3_FRAME_PIXELS;
use serde::{Deserialize, Serialize};
use std::fmt;
use truelearner_arena_format::{ArenaId, ArrowId, CellId, ContentHash};
use truelearner_core::{
    ArrowSpec, BoundaryError, BoundaryRuntime, CellSpec, MechanicalConfig, PlasticSubstrate,
    SpikeInput, TransmissionMode,
};

const PALETTE_CONTEXTS: usize = 16;
const SPATIAL_CONTEXTS: usize = 1_024;
const MOTORS: usize = 4;
const OUTWARD_REGION: i16 = 1;
const INPUT_CAPACITY: usize = 256;
const OUTPUT_CAPACITY: usize = 32;
const SCAFFOLD_RESISTANCE: u32 = 64;
const CANDIDATE_RESISTANCE: u32 = 1;
const MOTOR_PHYSICAL_BASE: u64 = 4_000_000;
const OUTPUT_PHYSICAL_BASE: u64 = 5_000_000;
const BABBLER_PHYSICAL_BASE: u64 = 7_000_000;
const EXTERNAL_PHYSICAL_BASE: u64 = 9_000_000;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Arc3AgentCommand {
    Observe {
        frame: Vec<u8>,
        available_actions: Vec<u8>,
        babble_action: Option<u8>,
        support_previous: bool,
        settle_pressure: bool,
        action_map: Vec<u8>,
    },
    ClearEpisode,
    AdvanceGap {
        ticks: i64,
    },
    ResetBody,
    Snapshot,
    Shutdown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "response", rename_all = "snake_case")]
pub enum Arc3AgentResponse {
    Observation(Arc3SensorimotorObservation),
    Snapshot(Arc3SensorimotorSnapshot),
    Ack,
    Error { message: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc3SensorimotorObservation {
    pub sequence: u64,
    pub context: u16,
    pub frame_changed: Option<bool>,
    pub support_admitted: bool,
    pub babble_action: Option<u8>,
    pub motor_crossing: Option<u8>,
    pub action: Option<u8>,
    pub outward_crossings: usize,
    pub plasticity_updates: u64,
    pub modulatory_deliveries: u64,
    pub physical_work: u64,
    pub naturally_quiescent: bool,
    pub candidate_resistance: u32,
    pub candidate_coupling: i32,
    pub candidate_live: bool,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub pressure_phase: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc3SensorimotorSnapshot {
    pub sequence: u64,
    pub body_fingerprint: String,
    pub physical_tick: i64,
    pub pressure_phase: i64,
    pub previous_context: Option<u16>,
    pub previous_motor: Option<u8>,
    pub resident_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arc3A1EpisodeClass {
    Development,
    Test,
    Control,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arc3A1EpisodeOutcome {
    ScaffoldedAction,
    StructureFormed,
    LearnedAction,
    ExpectedSilence,
    MappingFollowed,
    RetainedAction,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc3A1Turn {
    pub turn: u32,
    pub frame: Vec<u8>,
    pub organism: Arc3SensorimotorObservation,
    pub official_state: String,
    pub levels_completed: u16,
    pub win_levels: u16,
    pub caption: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc3A1Episode {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub class: Arc3A1EpisodeClass,
    pub outcome: Arc3A1EpisodeOutcome,
    pub turns: Vec<Arc3A1Turn>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc3A1Suite {
    pub schema_version: u16,
    pub game_id: String,
    pub toolkit_revision: String,
    pub seed: u64,
    pub exact_replay: bool,
    pub episodes: Vec<Arc3A1Episode>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SensorMode {
    DominantPalette,
    SpatialFingerprint,
}

#[derive(Clone)]
struct Sites {
    candidate_sources: Vec<[CellId; MOTORS]>,
    context_traces: Vec<[CellId; MOTORS]>,
    motors: Vec<[CellId; MOTORS]>,
    babblers: Vec<[CellId; MOTORS]>,
    returning: CellId,
    candidates: Vec<[Option<ArrowId>; MOTORS]>,
}

pub struct Arc3Sensorimotor {
    seed: u64,
    sensor_mode: SensorMode,
    boundary: BoundaryRuntime,
    sites: Sites,
    previous_frame: Option<Vec<u8>>,
    previous_context: Option<u16>,
    previous_motor: Option<u8>,
    sequence: u64,
}

impl Arc3Sensorimotor {
    pub fn new(seed: u64) -> Result<Self, Arc3SensorimotorError> {
        Self::with_sensor(seed, SensorMode::DominantPalette)
    }

    pub fn new_spatial(seed: u64) -> Result<Self, Arc3SensorimotorError> {
        Self::with_sensor(seed, SensorMode::SpatialFingerprint)
    }

    fn with_sensor(seed: u64, sensor_mode: SensorMode) -> Result<Self, Arc3SensorimotorError> {
        let context_count = match sensor_mode {
            SensorMode::DominantPalette => PALETTE_CONTEXTS,
            SensorMode::SpatialFingerprint => SPATIAL_CONTEXTS,
        };
        let (boundary, sites) = build_body(seed, sensor_mode, context_count)?;
        Ok(Self {
            seed,
            sensor_mode,
            boundary,
            sites,
            previous_frame: None,
            previous_context: None,
            previous_motor: None,
            sequence: 0,
        })
    }

    pub fn observe(
        &mut self,
        frame: Vec<u8>,
        available_actions: &[u8],
        babble_action: Option<u8>,
        support_previous: bool,
        settle_pressure: bool,
        action_map: &[u8],
    ) -> Result<Arc3SensorimotorObservation, Arc3SensorimotorError> {
        validate_sensor_frame(&frame)?;
        let available_motors = available_motor_indices(available_actions)?;
        validate_action_map(action_map)?;
        if let Some(action) = babble_action {
            if !available_actions.contains(&action) {
                return Err(Arc3SensorimotorError(format!(
                    "babble action {action} is not available"
                )));
            }
        }

        let frame_changed = self
            .previous_frame
            .as_ref()
            .map(|previous| previous != &frame);
        let mut total_work = 0_u64;
        let mut plasticity_updates = 0_u64;
        let mut modulatory_deliveries = 0_u64;
        let mut naturally_quiescent = true;
        let support_admitted = support_previous
            && frame_changed == Some(true)
            && self.previous_context.is_some()
            && self.previous_motor.is_some();

        if support_admitted {
            let tick = self.boundary.substrate().clock().tick;
            let result = self.boundary.arrive(
                &[SpikeInput {
                    arrival_tick: tick,
                    phase: 20,
                    origin_physical: EXTERNAL_PHYSICAL_BASE
                        .saturating_add(self.sequence)
                        .saturating_add(1_000),
                    target: self.sites.returning,
                    impulse: 1,
                }],
                OUTWARD_REGION,
            )?;
            total_work = total_work.saturating_add(result.work.total());
            plasticity_updates =
                plasticity_updates.saturating_add(result.work.local_return_updates);
            modulatory_deliveries =
                modulatory_deliveries.saturating_add(result.work.modulatory_deliveries);
            naturally_quiescent &= result.naturally_quiescent;
        }

        if settle_pressure && self.previous_frame.is_some() {
            let tick = self.boundary.substrate().clock().tick;
            let settled = tick.div_euclid(10).saturating_add(1).saturating_mul(10);
            total_work = total_work.saturating_add(self.boundary.advance_time(settled).total());
        }

        let context = self.sensor_context(&frame)?;
        let context_index = usize::from(context);
        let start = self.boundary.substrate().clock().tick.saturating_add(1);
        let mut inputs = Vec::with_capacity(available_motors.len().saturating_mul(2) + 1);
        for motor in &available_motors {
            inputs.push(SpikeInput {
                arrival_tick: start,
                phase: i32::try_from(*motor).unwrap_or(0),
                origin_physical: EXTERNAL_PHYSICAL_BASE
                    .saturating_add(self.sequence.saturating_mul(100))
                    .saturating_add(u64::from(context)),
                target: self.sites.candidate_sources[context_index][*motor],
                impulse: 1,
            });
            inputs.push(SpikeInput {
                arrival_tick: start,
                phase: i32::try_from(*motor).unwrap_or(0).saturating_add(8),
                origin_physical: EXTERNAL_PHYSICAL_BASE
                    .saturating_add(self.sequence.saturating_mul(100))
                    .saturating_add(50)
                    .saturating_add(u64::from(context)),
                target: self.sites.context_traces[context_index][*motor],
                impulse: 1,
            });
        }
        if let Some(action) = babble_action {
            let motor = action_index(action)?;
            inputs.push(SpikeInput {
                arrival_tick: start.saturating_add(1),
                phase: 30,
                origin_physical: EXTERNAL_PHYSICAL_BASE
                    .saturating_add(self.sequence.saturating_mul(100))
                    .saturating_add(99),
                target: self.sites.babblers[context_index][motor],
                impulse: 1,
            });
        }

        let result = self.boundary.arrive(&inputs, OUTWARD_REGION)?;
        total_work = total_work.saturating_add(result.work.total());
        plasticity_updates = plasticity_updates.saturating_add(result.work.local_return_updates);
        modulatory_deliveries =
            modulatory_deliveries.saturating_add(result.work.modulatory_deliveries);
        naturally_quiescent &= result.naturally_quiescent;
        let motor_crossings = result
            .crossings
            .iter()
            .filter_map(|crossing| {
                crossing
                    .from_physical
                    .checked_sub(MOTOR_PHYSICAL_BASE)
                    .and_then(|index| match self.sensor_mode {
                        SensorMode::DominantPalette if index < MOTORS as u64 => {
                            u8::try_from(index).ok()
                        }
                        SensorMode::SpatialFingerprint => u8::try_from(index % MOTORS as u64).ok(),
                        SensorMode::DominantPalette => None,
                    })
            })
            .collect::<Vec<_>>();
        if motor_crossings.len() > 1 {
            return Err(Arc3SensorimotorError(format!(
                "ambiguous organism output: {} motor crossings",
                motor_crossings.len()
            )));
        }
        let motor_crossing = motor_crossings.first().copied();
        let action = motor_crossing.map(|motor| action_map[usize::from(motor)]);
        let inspected_motor = motor_crossing
            .map(usize::from)
            .or_else(|| babble_action.and_then(|action| action_index(action).ok()))
            .unwrap_or(0);
        let (candidate_resistance, candidate_coupling, candidate_live) =
            self.candidate_state(context, inspected_motor);

        self.previous_frame = Some(frame);
        self.previous_context = Some(context);
        self.previous_motor = motor_crossing;
        let observation = Arc3SensorimotorObservation {
            sequence: self.sequence,
            context,
            frame_changed,
            support_admitted,
            babble_action,
            motor_crossing,
            action,
            outward_crossings: result.crossings.len(),
            plasticity_updates,
            modulatory_deliveries,
            physical_work: total_work,
            naturally_quiescent,
            candidate_resistance,
            candidate_coupling,
            candidate_live,
            body_fingerprint: self.body_fingerprint()?,
            physical_tick: self.boundary.substrate().clock().tick,
            pressure_phase: self.boundary.substrate().clock().pressure_phase(),
        };
        self.sequence = self.sequence.saturating_add(1);
        Ok(observation)
    }

    pub fn clear_episode(&mut self) {
        self.previous_frame = None;
        self.previous_context = None;
        self.previous_motor = None;
    }

    pub fn advance_gap(&mut self, ticks: i64) -> Result<(), Arc3SensorimotorError> {
        if ticks < 0 {
            return Err(Arc3SensorimotorError(
                "retention gap cannot be negative".to_string(),
            ));
        }
        let target = self.boundary.substrate().clock().tick.saturating_add(ticks);
        self.boundary.advance_time(target);
        self.clear_episode();
        Ok(())
    }

    pub fn reset_body(&mut self) -> Result<(), Arc3SensorimotorError> {
        let replacement = Self::with_sensor(self.seed, self.sensor_mode)?;
        *self = replacement;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<Arc3SensorimotorSnapshot, Arc3SensorimotorError> {
        Ok(Arc3SensorimotorSnapshot {
            sequence: self.sequence,
            body_fingerprint: self.body_fingerprint()?,
            physical_tick: self.boundary.substrate().clock().tick,
            pressure_phase: self.boundary.substrate().clock().pressure_phase(),
            previous_context: self.previous_context,
            previous_motor: self.previous_motor,
            resident_bytes: self.boundary.substrate().canonical_body_bytes(0)?.len(),
        })
    }

    pub fn handle(
        &mut self,
        command: Arc3AgentCommand,
    ) -> Result<Option<Arc3AgentResponse>, Arc3SensorimotorError> {
        match command {
            Arc3AgentCommand::Observe {
                frame,
                available_actions,
                babble_action,
                support_previous,
                settle_pressure,
                action_map,
            } => self
                .observe(
                    frame,
                    &available_actions,
                    babble_action,
                    support_previous,
                    settle_pressure,
                    &action_map,
                )
                .map(Arc3AgentResponse::Observation)
                .map(Some),
            Arc3AgentCommand::ClearEpisode => {
                self.clear_episode();
                Ok(Some(Arc3AgentResponse::Ack))
            }
            Arc3AgentCommand::AdvanceGap { ticks } => {
                self.advance_gap(ticks)?;
                Ok(Some(Arc3AgentResponse::Ack))
            }
            Arc3AgentCommand::ResetBody => {
                self.reset_body()?;
                Ok(Some(Arc3AgentResponse::Ack))
            }
            Arc3AgentCommand::Snapshot => {
                self.snapshot().map(Arc3AgentResponse::Snapshot).map(Some)
            }
            Arc3AgentCommand::Shutdown => Ok(None),
        }
    }

    fn candidate_state(&self, context: u16, motor: usize) -> (u32, i32, bool) {
        let context = usize::from(context);
        let body = self.boundary.substrate().arena_body(0);
        if let Some(id) = self.sites.candidates[context][motor] {
            return body
                .arrows
                .into_iter()
                .find(|arrow| arrow.id == id)
                .map_or((0, 0, false), |arrow| {
                    (arrow.resistance, arrow.coupling, arrow.live)
                });
        }
        let source = self.sites.candidate_sources[context][motor];
        let target = self.sites.motors[context][motor];
        body.arrows
            .into_iter()
            .filter(|arrow| arrow.from.id == source && arrow.to.id == target)
            .max_by_key(|arrow| (arrow.live, arrow.id))
            .map_or((0, 0, false), |arrow| {
                (arrow.resistance, arrow.coupling, arrow.live)
            })
    }

    fn body_fingerprint(&self) -> Result<String, Arc3SensorimotorError> {
        let bytes = self.boundary.substrate().canonical_body_bytes(0)?;
        Ok(ContentHash::of(&bytes)
            .as_bytes()
            .iter()
            .take(8)
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }

    fn sensor_context(&self, frame: &[u8]) -> Result<u16, Arc3SensorimotorError> {
        match self.sensor_mode {
            SensorMode::DominantPalette => dominant_palette(frame).map(u16::from),
            SensorMode::SpatialFingerprint => spatial_context(frame),
        }
    }
}

pub fn dominant_palette(frame: &[u8]) -> Result<u8, Arc3SensorimotorError> {
    validate_sensor_frame(frame)?;
    let mut counts = [0_u32; PALETTE_CONTEXTS];
    for color in frame {
        counts[usize::from(*color)] = counts[usize::from(*color)].saturating_add(1);
    }
    counts
        .iter()
        .enumerate()
        .max_by_key(|(color, count)| (**count, std::cmp::Reverse(*color)))
        .and_then(|(color, _)| u8::try_from(color).ok())
        .ok_or_else(|| Arc3SensorimotorError("raster has no dominant palette value".to_string()))
}

pub fn spatial_context(frame: &[u8]) -> Result<u16, Arc3SensorimotorError> {
    validate_sensor_frame(frame)?;
    let digest = ContentHash::of(frame);
    let mut prefix = [0_u8; 8];
    prefix.copy_from_slice(&digest.as_bytes()[..8]);
    let context = u64::from_be_bytes(prefix) % SPATIAL_CONTEXTS as u64;
    u16::try_from(context)
        .map_err(|_| Arc3SensorimotorError("spatial context exceeds u16".to_string()))
}

fn build_body(
    seed: u64,
    sensor_mode: SensorMode,
    context_count: usize,
) -> Result<(BoundaryRuntime, Sites), Arc3SensorimotorError> {
    let spatial = sensor_mode == SensorMode::SpatialFingerprint;
    let cells_per_pair = if spatial { 6 } else { 3 };
    let arrows_per_pair = if spatial { 6 } else { 5 };
    let cell_capacity = u32::try_from(
        (context_count
            .saturating_mul(MOTORS)
            .saturating_mul(cells_per_pair)
            + 16)
            .max(512),
    )
    .map_err(|_| Arc3SensorimotorError("cell capacity exceeds u32".to_string()))?;
    let arrow_capacity = u32::try_from(
        context_count
            .saturating_mul(MOTORS)
            .saturating_mul(arrows_per_pair)
            .saturating_add(context_count.saturating_mul(8))
            .saturating_add(256)
            .max(1_024),
    )
    .map_err(|_| Arc3SensorimotorError("arrow capacity exceeds u32".to_string()))?;
    let mut body = PlasticSubstrate::with_mechanics(
        ArenaId(seed),
        cell_capacity,
        arrow_capacity,
        MechanicalConfig::PRODUCTION,
    );
    body.set_physical_tracing(true);
    let mut candidate_sources = vec![[CellId(0); MOTORS]; context_count];
    let mut context_traces = vec![[CellId(0); MOTORS]; context_count];
    let mut relays = vec![[CellId(0); MOTORS]; context_count];
    let mut motors = vec![[CellId(0); MOTORS]; context_count];
    let mut babblers = vec![[CellId(0); MOTORS]; context_count];
    let mut outputs = vec![[CellId(0); MOTORS]; context_count];
    let band_span = i32::try_from(
        context_count
            .saturating_mul(MOTORS)
            .saturating_mul(20)
            .saturating_add(1_000),
    )
    .map_err(|_| Arc3SensorimotorError("spatial sensor band exceeds i32".to_string()))?;
    let trace_base = if context_count == PALETTE_CONTEXTS {
        20_000
    } else {
        band_span + 100
    };
    let relay_base = if context_count == PALETTE_CONTEXTS {
        40_000
    } else {
        band_span.saturating_mul(2) + 100
    };
    let motor_base = if context_count == PALETTE_CONTEXTS {
        60_000
    } else {
        100
    };
    let babble_base = if context_count == PALETTE_CONTEXTS {
        60_000
    } else {
        band_span.saturating_mul(3) + 100
    };
    let output_base = if context_count == PALETTE_CONTEXTS {
        70_000
    } else {
        band_span.saturating_mul(4) + 100
    };
    let return_position = if context_count == PALETTE_CONTEXTS {
        80_000
    } else {
        band_span.saturating_mul(5) + 100
    };
    for context in 0..context_count {
        for motor in 0..MOTORS {
            let pair = context.saturating_mul(MOTORS).saturating_add(motor);
            let offset = i32::try_from(pair.saturating_mul(20))
                .map_err(|_| Arc3SensorimotorError("context position exceeds i32".to_string()))?;
            candidate_sources[context][motor] =
                body.add_cell(cell(1_000_000 + pair as u64, 100 + offset, 0, 1));
            context_traces[context][motor] =
                body.add_cell(cell(2_000_000 + pair as u64, trace_base + offset, 0, 1));
            relays[context][motor] =
                body.add_cell(cell(3_000_000 + pair as u64, relay_base + offset, 0, 3));
            if spatial {
                motors[context][motor] = body.add_cell(cell(
                    MOTOR_PHYSICAL_BASE + pair as u64,
                    motor_base + offset + 1,
                    0,
                    2,
                ));
                babblers[context][motor] = body.add_cell(cell(
                    BABBLER_PHYSICAL_BASE + pair as u64,
                    babble_base + offset,
                    0,
                    1,
                ));
                outputs[context][motor] = body.add_cell(cell(
                    OUTPUT_PHYSICAL_BASE + pair as u64,
                    output_base + offset,
                    OUTWARD_REGION,
                    1,
                ));
            }
        }
    }
    if !spatial {
        let shared_motors = std::array::from_fn(|motor| {
            body.add_cell(cell(
                MOTOR_PHYSICAL_BASE + motor as u64,
                motor_base + motor as i32 * 20,
                0,
                2,
            ))
        });
        let shared_outputs = std::array::from_fn(|motor| {
            body.add_cell(cell(
                OUTPUT_PHYSICAL_BASE + motor as u64,
                output_base + motor as i32 * 20,
                OUTWARD_REGION,
                1,
            ))
        });
        motors.fill(shared_motors);
        babblers.fill(shared_motors);
        outputs.fill(shared_outputs);
    }
    let returning = body.add_cell(cell(6_000_000, return_position, 0, 1));
    let mut candidates = vec![[None; MOTORS]; context_count];
    for context in 0..context_count {
        for motor in 0..MOTORS {
            if !spatial {
                candidates[context][motor] = Some(body.add_arrow(drive(
                    candidate_sources[context][motor],
                    motors[context][motor],
                    1,
                    1,
                    CANDIDATE_RESISTANCE,
                )));
            }
            body.add_arrow(drive(
                context_traces[context][motor],
                relays[context][motor],
                3,
                1,
                SCAFFOLD_RESISTANCE,
            ));
            body.add_arrow(drive(
                motors[context][motor],
                relays[context][motor],
                2,
                1,
                SCAFFOLD_RESISTANCE,
            ));
            body.add_arrow(drive(
                returning,
                relays[context][motor],
                0,
                1,
                SCAFFOLD_RESISTANCE,
            ));
            body.add_arrow(modulatory(
                relays[context][motor],
                candidate_sources[context][motor],
                0,
                1,
                SCAFFOLD_RESISTANCE,
            ));
            if spatial {
                body.add_arrow(drive(
                    babblers[context][motor],
                    motors[context][motor],
                    1,
                    1,
                    SCAFFOLD_RESISTANCE,
                ));
                body.add_arrow(drive(
                    motors[context][motor],
                    outputs[context][motor],
                    0,
                    1,
                    SCAFFOLD_RESISTANCE,
                ));
            }
        }
    }
    if !spatial {
        for motor in 0..MOTORS {
            body.add_arrow(drive(
                motors[0][motor],
                outputs[0][motor],
                0,
                1,
                SCAFFOLD_RESISTANCE,
            ));
        }
    }
    let sites = Sites {
        candidate_sources,
        context_traces,
        motors,
        babblers,
        returning,
        candidates,
    };
    Ok((
        BoundaryRuntime::new(body, OUTWARD_REGION, INPUT_CAPACITY, OUTPUT_CAPACITY)?,
        sites,
    ))
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: SCAFFOLD_RESISTANCE,
    }
}

fn drive(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode: TransmissionMode::Drive,
    }
}

fn modulatory(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode: TransmissionMode::Modulatory,
    }
}

fn validate_sensor_frame(frame: &[u8]) -> Result<(), Arc3SensorimotorError> {
    if frame.len() != ARC3_FRAME_PIXELS {
        return Err(Arc3SensorimotorError(format!(
            "ARC raster has {} cells; expected {ARC3_FRAME_PIXELS}",
            frame.len()
        )));
    }
    if let Some(color) = frame.iter().copied().find(|color| *color > 15) {
        return Err(Arc3SensorimotorError(format!(
            "ARC raster color {color} is outside 0..15"
        )));
    }
    Ok(())
}

fn available_motor_indices(actions: &[u8]) -> Result<Vec<usize>, Arc3SensorimotorError> {
    let mut motors = Vec::new();
    for action in actions {
        let motor = action_index(*action)?;
        if !motors.contains(&motor) {
            motors.push(motor);
        }
    }
    motors.sort_unstable();
    if motors.is_empty() {
        return Err(Arc3SensorimotorError(
            "no supported simple motor is available".to_string(),
        ));
    }
    Ok(motors)
}

fn action_index(action: u8) -> Result<usize, Arc3SensorimotorError> {
    if !(1..=MOTORS as u8).contains(&action) {
        return Err(Arc3SensorimotorError(format!(
            "ARC3-A1 supports simple actions 1..={MOTORS}; received {action}"
        )));
    }
    Ok(usize::from(action - 1))
}

fn validate_action_map(map: &[u8]) -> Result<(), Arc3SensorimotorError> {
    if map.len() != MOTORS || map.iter().any(|action| !(1..=7).contains(action)) {
        return Err(Arc3SensorimotorError(
            "action map must contain four ARC action identifiers in 1..=7".to_string(),
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arc3SensorimotorError(String);

impl fmt::Display for Arc3SensorimotorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Arc3SensorimotorError {}

impl From<BoundaryError> for Arc3SensorimotorError {
    fn from(error: BoundaryError) -> Self {
        Self(format!("boundary error: {error:?}"))
    }
}

impl From<truelearner_arena_format::FormatError> for Arc3SensorimotorError {
    fn from(error: truelearner_arena_format::FormatError) -> Self {
        Self(format!("format error: {error:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(color: u8) -> Vec<u8> {
        vec![color; ARC3_FRAME_PIXELS]
    }

    #[test]
    fn changed_raster_supports_one_motor_and_probe_needs_no_babble() {
        let mut organism = Arc3Sensorimotor::new(205).unwrap();
        let first = organism
            .observe(
                frame(4),
                &[1, 2, 3, 4],
                Some(1),
                false,
                false,
                &[1, 2, 3, 4],
            )
            .unwrap();
        assert_eq!(first.action, Some(1));
        assert_eq!(first.candidate_coupling, 1);

        let mut changed = frame(4);
        changed[0] = 3;
        let learned = organism
            .observe(changed, &[1, 2, 3, 4], None, true, true, &[1, 2, 3, 4])
            .unwrap();
        assert_eq!(learned.plasticity_updates, 1);
        assert_eq!(learned.action, Some(1));
        assert_eq!(learned.babble_action, None);
        assert_eq!(learned.candidate_coupling, 2);
        assert!(learned.naturally_quiescent);
    }

    #[test]
    fn blocked_return_cannot_mature_adjacent_babbled_episodes() {
        let mut organism = Arc3Sensorimotor::new(206).unwrap();
        let first = organism
            .observe(
                frame(4),
                &[1, 2, 3, 4],
                Some(1),
                false,
                false,
                &[1, 2, 3, 4],
            )
            .unwrap();
        assert_eq!(first.action, Some(1));
        let mut changed = frame(4);
        changed[1] = 3;
        let second = organism
            .observe(
                changed.clone(),
                &[1, 2, 3, 4],
                Some(1),
                false,
                false,
                &[1, 2, 3, 4],
            )
            .unwrap();
        assert_eq!(second.action, Some(1));
        assert_eq!(second.plasticity_updates, 0);
        changed[2] = 3;
        let settled = organism
            .observe(changed, &[1, 2, 3, 4], None, false, true, &[1, 2, 3, 4])
            .unwrap();
        assert_eq!(settled.action, None);
        assert_eq!(settled.plasticity_updates, 0);
        assert!(!settled.candidate_live);
    }

    #[test]
    fn action_meaning_follows_the_external_map() {
        let mut organism = Arc3Sensorimotor::new(207).unwrap();
        let _ = organism
            .observe(
                frame(4),
                &[1, 2, 3, 4],
                Some(1),
                false,
                false,
                &[1, 2, 3, 4],
            )
            .unwrap();
        let mut changed = frame(4);
        changed[0] = 3;
        let _ = organism
            .observe(changed, &[1, 2, 3, 4], None, true, true, &[1, 2, 3, 4])
            .unwrap();
        organism.clear_episode();
        let shuffled = organism
            .observe(frame(4), &[1, 2, 3, 4], None, false, false, &[2, 1, 3, 4])
            .unwrap();
        assert_eq!(shuffled.motor_crossing, Some(0));
        assert_eq!(shuffled.action, Some(2));
    }

    #[test]
    fn spatial_fingerprint_distinguishes_raw_raster_positions() {
        let mut first = frame(4);
        let mut second = frame(4);
        first[64 * 7 + 9] = 14;
        second[64 * 7 + 10] = 14;
        assert_ne!(
            spatial_context(&first).unwrap(),
            spatial_context(&second).unwrap()
        );
    }

    #[test]
    fn spatial_body_learns_distinct_contexts_without_changing_a1_sensor() {
        let mut organism = Arc3Sensorimotor::new_spatial(208).unwrap();
        let first_frame = frame(4);
        let first_context = spatial_context(&first_frame).unwrap();
        assert_eq!(organism.candidate_state(first_context, 0), (0, 0, false));
        let first = organism
            .observe(
                first_frame,
                &[1, 2, 3, 4],
                Some(1),
                false,
                false,
                &[1, 2, 3, 4],
            )
            .unwrap();
        assert_eq!(first.context, first_context);
        assert_eq!(first.action, Some(1));
        assert_eq!(organism.candidate_state(first_context, 0), (1, 1, true));

        let mut changed = frame(4);
        changed[0] = 3;
        let second_context = spatial_context(&changed).unwrap();
        assert_ne!(first_context, second_context);
        let second = organism
            .observe(changed, &[1, 2, 3, 4], Some(2), true, false, &[1, 2, 3, 4])
            .unwrap();
        assert_eq!(second.context, second_context);
        assert_eq!(second.action, Some(2));
        assert_eq!(second.plasticity_updates, 1);
        assert_eq!(organism.candidate_state(first_context, 0), (4, 2, true));
    }
}
