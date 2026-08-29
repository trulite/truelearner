#![forbid(unsafe_code)]
//! Small, deterministic building blocks for physical sensor and actuator drivers.

use std::fmt;
use truelearner_core::{
    ComponentJunction, ComponentLink, HarnessBuilder, Input, Junction, JunctionId, Link,
    PhysicalAttachment, PhysicalComponentSpec, PhysicalIncidence, PhysicalInput, TransmissionMode,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JunctionSpec {
    pub physical_id: u64,
    pub position: i32,
    pub region: i16,
    pub threshold: i32,
    pub resistance: u32,
}

impl JunctionSpec {
    pub const fn ordinary(physical_id: u64, position: i32, region: i16, threshold: i32) -> Self {
        Self {
            physical_id,
            position,
            region,
            threshold,
            resistance: u32::MAX,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DriveSpec {
    pub delay: i64,
    pub phase: i32,
    pub coupling: i32,
    pub resistance: u32,
}

impl DriveSpec {
    pub const fn ordinary(coupling: i32) -> Self {
        Self {
            delay: 0,
            phase: 0,
            coupling,
            resistance: u32::MAX,
        }
    }
}

pub struct Wiring<'a> {
    builder: &'a mut HarnessBuilder,
}

impl<'a> Wiring<'a> {
    pub const fn new(builder: &'a mut HarnessBuilder) -> Self {
        Self { builder }
    }

    pub fn junction(&mut self, spec: JunctionSpec) -> JunctionId {
        self.builder.add_junction(Junction {
            physical_id: spec.physical_id,
            position: spec.position,
            region: spec.region,
            threshold: spec.threshold,
            resistance: spec.resistance,
        })
    }

    pub fn drive(&mut self, from: JunctionId, to: JunctionId, spec: DriveSpec) {
        self.builder.add_link(Link {
            from,
            to,
            delay: spec.delay,
            phase: spec.phase,
            coupling: spec.coupling,
            resistance: spec.resistance,
            mode: TransmissionMode::Drive,
        });
    }

    pub fn bind_output(&mut self, output: JunctionId, outcome: JunctionId) {
        self.builder.set_outcome_source_for_output(output, outcome);
    }

    pub fn junction_bank(
        &mut self,
        count: usize,
        physical_base: u64,
        mut spec: impl FnMut(usize, u64) -> JunctionSpec,
    ) -> Vec<JunctionId> {
        (0..count)
            .map(|index| {
                let physical_id =
                    physical_base.saturating_add(u64::try_from(index).unwrap_or(u64::MAX));
                self.junction(spec(index, physical_id))
            })
            .collect()
    }

    pub fn receptor_bank<const BINS: usize>(
        &mut self,
        features: usize,
        physical_base: u64,
        mut spec: impl FnMut(usize, usize, u64) -> JunctionSpec,
    ) -> Vec<[JunctionId; BINS]> {
        (0..features)
            .map(|feature| {
                std::array::from_fn(|bin| {
                    let offset = feature.saturating_mul(BINS).saturating_add(bin);
                    let physical_id =
                        physical_base.saturating_add(u64::try_from(offset).unwrap_or(u64::MAX));
                    self.junction(spec(feature, bin, physical_id))
                })
            })
            .collect()
    }

    pub fn actuator_bank(
        &mut self,
        count: usize,
        output_physical_base: u64,
        sink_physical_base: u64,
        mut output_spec: impl FnMut(usize, u64) -> JunctionSpec,
        mut sink_spec: impl FnMut(usize, u64) -> JunctionSpec,
        drive: DriveSpec,
    ) -> Vec<JunctionId> {
        (0..count)
            .map(|index| {
                let offset = u64::try_from(index).unwrap_or(u64::MAX);
                let output = self.junction(output_spec(
                    index,
                    output_physical_base.saturating_add(offset),
                ));
                let sink =
                    self.junction(sink_spec(index, sink_physical_base.saturating_add(offset)));
                self.drive(output, sink, drive);
                output
            })
            .collect()
    }
}

const TRACE_INTERNAL_POSITION: i32 = 1_024;
const TRACE_COMPARE_PHASE: i32 = 3;
const TRACE_CLEAR_PHASE: i32 = 4;
const TRACE_REWRITE_PHASE: i32 = 5;
const TRACE_FACTOR_PHASE: i32 = 6;
const TRACE_DRIVE_PHASE: i32 = 7;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChangeFactor {
    Split,
    PerThreshold,
    Unified,
    Calibration,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalTraceSpec {
    thresholds: Vec<i32>,
    lifetime: u32,
}

impl PhysicalTraceSpec {
    pub fn new(thresholds: Vec<i32>, lifetime: u32) -> Result<Self, PhysicalTraceSpecError> {
        if thresholds.is_empty() {
            return Err(PhysicalTraceSpecError::NoThresholds);
        }
        if lifetime == 0 || lifetime >= i32::MAX as u32 {
            return Err(PhysicalTraceSpecError::InvalidLifetime);
        }
        if thresholds.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(PhysicalTraceSpecError::ThresholdsNotIncreasing);
        }
        Ok(Self {
            thresholds,
            lifetime,
        })
    }

    pub fn build(&self) -> PhysicalTraceComponent {
        self.build_with_change_factor(ChangeFactor::Split)
    }

    pub fn build_factored_change(&self) -> PhysicalTraceComponent {
        self.build_with_change_factor(ChangeFactor::PerThreshold)
    }

    pub fn build_unified_change(&self) -> PhysicalTraceComponent {
        self.build_with_change_factor(ChangeFactor::Unified)
    }

    /// Builds physical memory whose nonzero sample is a local drive and whose
    /// threshold falls return to that same surface as candidate outcomes.
    pub fn build_calibration(&self) -> PhysicalTraceComponent {
        self.build_with_change_factor(ChangeFactor::Calibration)
    }

    fn build_with_change_factor(&self, factor_change: ChangeFactor) -> PhysicalTraceComponent {
        let mut junctions = Vec::new();
        let mut links = Vec::new();
        let mut ports = Vec::new();

        let sample = push_junction(&mut junctions, TRACE_INTERNAL_POSITION, 1);
        ports.push(sample);
        let above = self
            .thresholds
            .iter()
            .map(|_| {
                let port = push_junction(&mut junctions, TRACE_INTERNAL_POSITION, 1);
                ports.push(port);
                port
            })
            .collect::<Vec<_>>();
        let drive = (factor_change == ChangeFactor::Calibration).then(|| {
            let drive = push_junction(&mut junctions, 0, 1);
            ports.push(drive);
            drive
        });

        let lifetime = i32::try_from(self.lifetime).expect("validated trace lifetime fits i32");
        let latch_threshold = lifetime.saturating_add(1);
        let mut rise = Vec::with_capacity(self.thresholds.len());
        let mut fall = Vec::with_capacity(self.thresholds.len());
        let mut change = Vec::with_capacity(self.thresholds.len());
        let unified_change =
            (factor_change == ChangeFactor::Unified).then(|| push_junction(&mut junctions, 0, 1));

        for above in above {
            let current_high = push_junction(&mut junctions, TRACE_INTERNAL_POSITION, 1);
            let current_low = push_junction(&mut junctions, TRACE_INTERNAL_POSITION, 1);
            let high_latch =
                push_junction(&mut junctions, TRACE_INTERNAL_POSITION, latch_threshold);
            let low_latch = push_junction(&mut junctions, TRACE_INTERNAL_POSITION, latch_threshold);
            let known_latch =
                push_junction(&mut junctions, TRACE_INTERNAL_POSITION, latch_threshold);
            let directional_position = if factor_change != ChangeFactor::Split {
                TRACE_INTERNAL_POSITION
            } else {
                0
            };
            let rise_output = push_junction(&mut junctions, directional_position, 3);
            let fall_output = push_junction(&mut junctions, directional_position, 3);
            rise.push(rise_output);
            fall.push(fall_output);
            let change_output = match factor_change {
                ChangeFactor::Split => None,
                ChangeFactor::PerThreshold => Some(push_junction(&mut junctions, 0, 1)),
                ChangeFactor::Unified => unified_change,
                ChangeFactor::Calibration => None,
            };
            change.push(change_output);

            push_link(&mut links, above, current_high, 1, 1);
            push_link(&mut links, sample, current_low, 2, 1);
            push_link(&mut links, current_high, current_low, 2, -1);

            for latch in [high_latch, low_latch, known_latch] {
                push_link(&mut links, sample, latch, 1, lifetime);
                push_link(&mut links, sample, latch, TRACE_CLEAR_PHASE, -lifetime);
                push_link(&mut links, latch, latch, TRACE_CLEAR_PHASE, lifetime);
            }

            push_compare_input(&mut links, current_high, rise_output);
            push_compare_input(&mut links, low_latch, rise_output);
            push_compare_input(&mut links, known_latch, rise_output);
            push_compare_input(&mut links, current_low, fall_output);
            push_compare_input(&mut links, high_latch, fall_output);
            push_compare_input(&mut links, known_latch, fall_output);
            push_link(&mut links, rise_output, rise_output, TRACE_CLEAR_PHASE, 3);
            push_link(&mut links, fall_output, fall_output, TRACE_CLEAR_PHASE, 3);
            if let Some(change_output) = change_output {
                push_link(
                    &mut links,
                    rise_output,
                    change_output,
                    TRACE_FACTOR_PHASE,
                    1,
                );
                push_link(
                    &mut links,
                    fall_output,
                    change_output,
                    TRACE_FACTOR_PHASE,
                    1,
                );
            }
            if let Some(drive) = drive {
                push_link(&mut links, fall_output, drive, TRACE_FACTOR_PHASE, 1);
            }

            push_link(
                &mut links,
                current_high,
                high_latch,
                TRACE_REWRITE_PHASE,
                lifetime,
            );
            push_link(
                &mut links,
                current_low,
                low_latch,
                TRACE_REWRITE_PHASE,
                lifetime,
            );
            push_link(
                &mut links,
                current_high,
                known_latch,
                TRACE_REWRITE_PHASE,
                lifetime,
            );
            push_link(
                &mut links,
                current_low,
                known_latch,
                TRACE_REWRITE_PHASE,
                lifetime,
            );
        }

        PhysicalTraceComponent {
            component: PhysicalComponentSpec::new(junctions, links, ports)
                .expect("physical trace topology is internally valid"),
            thresholds: self.thresholds.clone(),
            rise,
            fall,
            change,
            drive,
        }
    }
}

fn push_junction(
    junctions: &mut Vec<ComponentJunction>,
    relative_position: i32,
    threshold: i32,
) -> usize {
    let index = junctions.len();
    junctions.push(ComponentJunction::ordinary(relative_position, threshold));
    index
}

fn push_link(links: &mut Vec<ComponentLink>, from: usize, to: usize, phase: i32, coupling: i32) {
    links.push(
        ComponentLink::new(
            from,
            to,
            0,
            phase,
            coupling,
            u32::MAX,
            TransmissionMode::Drive,
        )
        .expect("physical trace link is internally valid"),
    );
}

fn push_compare_input(links: &mut Vec<ComponentLink>, from: usize, comparison: usize) {
    push_link(links, from, comparison, TRACE_COMPARE_PHASE, 1);
    push_link(links, from, comparison, TRACE_CLEAR_PHASE, -1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhysicalTraceSpecError {
    NoThresholds,
    InvalidLifetime,
    ThresholdsNotIncreasing,
}

impl fmt::Display for PhysicalTraceSpecError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid physical trace: {self:?}")
    }
}

impl std::error::Error for PhysicalTraceSpecError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalTraceComponent {
    component: PhysicalComponentSpec,
    thresholds: Vec<i32>,
    rise: Vec<usize>,
    fall: Vec<usize>,
    change: Vec<Option<usize>>,
    drive: Option<usize>,
}

impl PhysicalTraceComponent {
    pub const fn component(&self) -> &PhysicalComponentSpec {
        &self.component
    }

    pub fn sample_inputs(
        &self,
        attachment: &PhysicalAttachment,
        tick: i64,
        value: i32,
        incidence: PhysicalIncidence,
    ) -> Result<Vec<PhysicalInput>, PhysicalTraceInputError> {
        let observation_origin = attachment
            .port(0)
            .ok_or(PhysicalTraceInputError::WrongAttachment {
                expected: self
                    .thresholds
                    .len()
                    .saturating_add(1)
                    .saturating_add(usize::from(self.drive.is_some())),
                actual: attachment.len(),
            })?
            .origin_physical();
        self.sample_inputs_from(attachment, tick, value, incidence, observation_origin)
    }

    pub fn sample_inputs_from(
        &self,
        attachment: &PhysicalAttachment,
        tick: i64,
        value: i32,
        incidence: PhysicalIncidence,
        origin_physical: u64,
    ) -> Result<Vec<PhysicalInput>, PhysicalTraceInputError> {
        let expected = self
            .thresholds
            .len()
            .saturating_add(1)
            .saturating_add(usize::from(self.drive.is_some()));
        if attachment.len() != expected {
            return Err(PhysicalTraceInputError::WrongAttachment {
                expected,
                actual: attachment.len(),
            });
        }
        let mut inputs = Vec::with_capacity(expected);
        for (index, threshold) in self.thresholds.iter().enumerate() {
            if value >= *threshold {
                inputs.push(trace_input(
                    attachment
                        .port(index.saturating_add(1))
                        .expect("validated attachment contains threshold port"),
                    tick,
                    origin_physical,
                    incidence,
                ));
            }
        }
        inputs.push(trace_input(
            attachment
                .port(0)
                .expect("validated attachment contains sample port"),
            tick,
            origin_physical,
            incidence,
        ));
        if self.drive.is_some() && value > 0 {
            inputs.push(trace_input_at_phase(
                attachment
                    .port(self.thresholds.len().saturating_add(1))
                    .expect("validated attachment contains calibration drive port"),
                tick,
                TRACE_DRIVE_PHASE,
                origin_physical,
                PhysicalIncidence::Sample,
            ));
        }
        Ok(inputs)
    }

    pub fn rise_origin(&self, attachment: &PhysicalAttachment, threshold: usize) -> Option<u64> {
        component_origin(attachment, *self.rise.get(threshold)?)
    }

    pub fn fall_origin(&self, attachment: &PhysicalAttachment, threshold: usize) -> Option<u64> {
        component_origin(attachment, *self.fall.get(threshold)?)
    }

    pub fn change_origin(&self, attachment: &PhysicalAttachment, threshold: usize) -> Option<u64> {
        component_origin(attachment, self.change.get(threshold)?.as_ref().copied()?)
    }

    pub fn drive_origin(&self, attachment: &PhysicalAttachment) -> Option<u64> {
        component_origin(attachment, self.drive?)
    }
}

fn component_origin(attachment: &PhysicalAttachment, local: usize) -> Option<u64> {
    attachment
        .port(0)?
        .origin_physical()
        .checked_add(u64::try_from(local).ok()?)
}

fn trace_input(
    port: truelearner_core::PhysicalPort,
    tick: i64,
    origin_physical: u64,
    incidence: PhysicalIncidence,
) -> PhysicalInput {
    trace_input_at_phase(port, tick, 0, origin_physical, incidence)
}

fn trace_input_at_phase(
    port: truelearner_core::PhysicalPort,
    tick: i64,
    phase: i32,
    origin_physical: u64,
    incidence: PhysicalIncidence,
) -> PhysicalInput {
    PhysicalInput {
        input: Input {
            target: port.target(),
            arrival_tick: tick,
            phase,
            impulse: 1,
            origin_physical,
        },
        incidence,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhysicalTraceInputError {
    WrongAttachment { expected: usize, actual: usize },
}

impl fmt::Display for PhysicalTraceInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "physical trace input failed: {self:?}")
    }
}

impl std::error::Error for PhysicalTraceInputError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Origin(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Incidence {
    Sample,
    Transition,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signal<T> {
    pub origin: Origin,
    pub incidence: Incidence,
    pub value: T,
}

impl<T> Signal<T> {
    pub const fn new(origin: Origin, incidence: Incidence, value: T) -> Self {
        Self {
            origin,
            incidence,
            value,
        }
    }

    pub fn map<U>(self, transform: impl FnOnce(T) -> U) -> Signal<U> {
        Signal {
            origin: self.origin,
            incidence: self.incidence,
            value: transform(self.value),
        }
    }

    pub fn fan_out<const N: usize>(&self) -> [Self; N]
    where
        T: Clone,
    {
        std::array::from_fn(|_| self.clone())
    }

    pub fn route(self, port: Port) -> RoutedSignal<T> {
        RoutedSignal { port, signal: self }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Availability<T> {
    Available(T),
    Unavailable,
}

impl<T> Availability<T> {
    pub fn map<U>(self, transform: impl FnOnce(T) -> U) -> Availability<U> {
        match self {
            Self::Available(value) => Availability::Available(transform(value)),
            Self::Unavailable => Availability::Unavailable,
        }
    }

    pub fn zip<U>(self, other: Availability<U>) -> Availability<(T, U)> {
        match (self, other) {
            (Self::Available(left), Availability::Available(right)) => {
                Availability::Available((left, right))
            }
            _ => Availability::Unavailable,
        }
    }

    pub const fn is_available(&self) -> bool {
        matches!(self, Self::Available(_))
    }
}

/// Non-negative distance from a component's locally normal relation to its body.
///
/// Zero is the quiet identity. Independent residuals compose by saturating
/// addition without assigning any device-specific meaning to their values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Residual(u32);

impl Residual {
    pub const ZERO: Self = Self(0);

    pub const fn new(amount: u32) -> Self {
        Self(amount)
    }

    pub const fn amount(self) -> u32 {
        self.0
    }

    pub const fn is_quiet(self) -> bool {
        self.0 == 0
    }

    pub const fn has_opportunity(self) -> bool {
        !self.is_quiet()
    }

    pub const fn effect_mode(self) -> EffectMode {
        if self.is_quiet() {
            EffectMode::Identity
        } else {
            EffectMode::Apply
        }
    }

    pub const fn combine(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }
}

/// A body-curried component relation.
///
/// The relation decides what "normal" means from local body context. This
/// driver only applies that relation to available observations; it owns no
/// temporal memory and invents no value when a component is unavailable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Normalizer<B, R> {
    body: B,
    relation: R,
}

pub const fn calibrate<B, R>(body: B, relation: R) -> Normalizer<B, R> {
    Normalizer { body, relation }
}

impl<B, R> Normalizer<B, R> {
    pub const fn body(&self) -> &B {
        &self.body
    }
}

impl<T, B, R> Driver<Signal<Availability<T>>> for Normalizer<B, R>
where
    R: FnMut(&B, &T) -> Residual,
{
    type Output = Signal<Availability<Residual>>;

    fn step(&mut self, input: Signal<Availability<T>>) -> Self::Output {
        let body = &self.body;
        let relation = &mut self.relation;
        input.map(|observation| observation.map(|value| relation(body, &value)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialFieldError {
    NoDimensions,
    ZeroDimension {
        axis: usize,
    },
    CellCountOverflow,
    WrongCellCount {
        expected: usize,
        actual: usize,
    },
    ZeroBlockDimension {
        axis: usize,
    },
    RegionCountOverflow,
    TooManyFoci {
        maximum: usize,
        actual: usize,
    },
    FocusOutsideField {
        axis: usize,
        coordinate: usize,
        extent: usize,
    },
    FocusFieldShapeMismatch,
}

impl fmt::Display for SpatialFieldError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoDimensions => formatter.write_str("spatial field must have a dimension"),
            Self::ZeroDimension { axis } => {
                write!(formatter, "spatial field dimension {axis} is zero")
            }
            Self::CellCountOverflow => formatter.write_str("spatial field cell count overflows"),
            Self::WrongCellCount { expected, actual } => write!(
                formatter,
                "spatial field needs {expected} cells but received {actual}"
            ),
            Self::ZeroBlockDimension { axis } => {
                write!(formatter, "local block dimension {axis} is zero")
            }
            Self::RegionCountOverflow => formatter.write_str("focused region count overflows"),
            Self::TooManyFoci { maximum, actual } => write!(
                formatter,
                "focus profile permits {maximum} foci but received {actual}"
            ),
            Self::FocusOutsideField {
                axis,
                coordinate,
                extent,
            } => write!(
                formatter,
                "focus coordinate {coordinate} is outside axis {axis} with extent {extent}"
            ),
            Self::FocusFieldShapeMismatch => {
                formatter.write_str("focus set belongs to another field shape")
            }
        }
    }
}

impl std::error::Error for SpatialFieldError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalBlockShape<const D: usize> {
    shape: [usize; D],
}

impl<const D: usize> LocalBlockShape<D> {
    pub fn new(shape: [usize; D]) -> Result<Self, SpatialFieldError> {
        if D == 0 {
            return Err(SpatialFieldError::NoDimensions);
        }
        if let Some(axis) = shape.iter().position(|extent| *extent == 0) {
            return Err(SpatialFieldError::ZeroBlockDimension { axis });
        }
        Ok(Self { shape })
    }

    pub const fn shape(self) -> [usize; D] {
        self.shape
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpatialField<T, const D: usize> {
    shape: [usize; D],
    cells: Vec<Availability<T>>,
}

impl<T, const D: usize> SpatialField<T, D> {
    pub fn new(shape: [usize; D], cells: Vec<Availability<T>>) -> Result<Self, SpatialFieldError> {
        let expected = checked_cell_count(shape)?;
        if cells.len() != expected {
            return Err(SpatialFieldError::WrongCellCount {
                expected,
                actual: cells.len(),
            });
        }
        Ok(Self { shape, cells })
    }

    pub const fn shape(&self) -> [usize; D] {
        self.shape
    }

    pub fn cells(&self) -> &[Availability<T>] {
        &self.cells
    }

    pub fn map<U>(self, mut transform: impl FnMut(T) -> U) -> SpatialField<U, D> {
        SpatialField {
            shape: self.shape,
            cells: self
                .cells
                .into_iter()
                .map(|cell| cell.map(&mut transform))
                .collect(),
        }
    }

    pub fn factor_local(self, block_shape: LocalBlockShape<D>) -> LocalFactorization<T, D> {
        let original_shape = self.shape;
        let block_shape = block_shape.shape;
        let mut source = self.cells.into_iter().map(Some).collect::<Vec<_>>();
        let mut blocks = Vec::new();
        let mut origin = [0; D];

        loop {
            let shape = std::array::from_fn(|axis| {
                block_shape[axis].min(original_shape[axis] - origin[axis])
            });
            let cell_count = cell_count_of_valid_shape(shape);
            let mut cells = Vec::with_capacity(cell_count);
            let mut local = [0; D];
            loop {
                let coordinate =
                    std::array::from_fn(|axis| origin[axis].saturating_add(local[axis]));
                let index = row_major_index(coordinate, original_shape);
                cells.push(source[index].take().expect("each field cell is moved once"));
                if !advance_coordinate(&mut local, shape) {
                    break;
                }
            }
            blocks.push(LocalBlock {
                origin,
                shape,
                cells,
            });
            if !advance_block_origin(&mut origin, original_shape, block_shape) {
                break;
            }
        }
        debug_assert!(source.iter().all(Option::is_none));

        LocalFactorization {
            original_shape,
            block_shape,
            blocks,
        }
    }

    pub fn focus_partition(
        self,
        focus: FocusSet<D>,
    ) -> Result<FocusedPartition<T, D>, SpatialFieldError> {
        if self.shape != focus.field_shape {
            return Err(SpatialFieldError::FocusFieldShapeMismatch);
        }
        let original_shape = self.shape;
        let mut regions = vec![Region {
            origin: [0; D],
            shape: original_shape,
        }];
        for _ in 0..focus.profile.refinement_depth {
            let mut next = Vec::new();
            for region in regions {
                if region.is_splittable() && region.contains_any(&focus.points) {
                    next.extend(region.split());
                } else {
                    next.push(region);
                }
            }
            regions = next;
        }
        debug_assert!(regions.len() <= focus.profile.region_bound);
        let blocks = move_cells_into_regions(original_shape, self.cells, &regions);
        Ok(FocusedPartition {
            original_shape,
            profile: focus.profile,
            foci: focus.points,
            regions: blocks,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FocusProfile<const D: usize> {
    refinement_depth: usize,
    maximum_foci: usize,
    region_bound: usize,
}

impl<const D: usize> FocusProfile<D> {
    pub fn new(refinement_depth: usize, maximum_foci: usize) -> Result<Self, SpatialFieldError> {
        if D == 0 {
            return Err(SpatialFieldError::NoDimensions);
        }
        let shift = u32::try_from(D).map_err(|_| SpatialFieldError::RegionCountOverflow)?;
        let children = 3_usize
            .checked_pow(shift)
            .ok_or(SpatialFieldError::RegionCountOverflow)?;
        let growth = children.saturating_sub(1);
        let region_bound = maximum_foci
            .checked_mul(refinement_depth)
            .and_then(|value| value.checked_mul(growth))
            .and_then(|value| value.checked_add(1))
            .ok_or(SpatialFieldError::RegionCountOverflow)?;
        Ok(Self {
            refinement_depth,
            maximum_foci,
            region_bound,
        })
    }

    pub const fn refinement_depth(self) -> usize {
        self.refinement_depth
    }

    pub const fn maximum_foci(self) -> usize {
        self.maximum_foci
    }

    pub const fn region_bound(self) -> usize {
        self.region_bound
    }

    pub fn focuses(
        self,
        field_shape: [usize; D],
        foci: impl IntoIterator<Item = [usize; D]>,
    ) -> Result<FocusSet<D>, SpatialFieldError> {
        checked_cell_count(field_shape)?;
        let mut points = foci.into_iter().collect::<Vec<_>>();
        points.sort_unstable();
        points.dedup();
        if points.len() > self.maximum_foci {
            return Err(SpatialFieldError::TooManyFoci {
                maximum: self.maximum_foci,
                actual: points.len(),
            });
        }
        for point in &points {
            for axis in 0..D {
                if point[axis] >= field_shape[axis] {
                    return Err(SpatialFieldError::FocusOutsideField {
                        axis,
                        coordinate: point[axis],
                        extent: field_shape[axis],
                    });
                }
            }
        }
        Ok(FocusSet {
            profile: self,
            field_shape,
            points,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusSet<const D: usize> {
    profile: FocusProfile<D>,
    field_shape: [usize; D],
    points: Vec<[usize; D]>,
}

impl<const D: usize> FocusSet<D> {
    pub const fn profile(&self) -> FocusProfile<D> {
        self.profile
    }

    pub fn points(&self) -> &[[usize; D]] {
        &self.points
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalBlock<T, const D: usize> {
    origin: [usize; D],
    shape: [usize; D],
    cells: Vec<Availability<T>>,
}

impl<T, const D: usize> LocalBlock<T, D> {
    pub const fn origin(&self) -> [usize; D] {
        self.origin
    }

    pub const fn shape(&self) -> [usize; D] {
        self.shape
    }

    pub fn cells(&self) -> &[Availability<T>] {
        &self.cells
    }

    fn map<U>(self, transform: &mut impl FnMut(T) -> U) -> LocalBlock<U, D> {
        LocalBlock {
            origin: self.origin,
            shape: self.shape,
            cells: self
                .cells
                .into_iter()
                .map(|cell| cell.map(&mut *transform))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalFactorization<T, const D: usize> {
    original_shape: [usize; D],
    block_shape: [usize; D],
    blocks: Vec<LocalBlock<T, D>>,
}

impl<T, const D: usize> LocalFactorization<T, D> {
    pub const fn original_shape(&self) -> [usize; D] {
        self.original_shape
    }

    pub const fn block_shape(&self) -> [usize; D] {
        self.block_shape
    }

    pub fn blocks(&self) -> &[LocalBlock<T, D>] {
        &self.blocks
    }

    pub fn map<U>(self, mut transform: impl FnMut(T) -> U) -> LocalFactorization<U, D> {
        LocalFactorization {
            original_shape: self.original_shape,
            block_shape: self.block_shape,
            blocks: self
                .blocks
                .into_iter()
                .map(|block| block.map(&mut transform))
                .collect(),
        }
    }

    pub fn reassemble(self) -> SpatialField<T, D> {
        let cell_count = cell_count_of_valid_shape(self.original_shape);
        let mut destination = (0..cell_count).map(|_| None).collect::<Vec<_>>();
        for block in self.blocks {
            let mut local = [0; D];
            let mut cells = block.cells.into_iter();
            loop {
                let coordinate =
                    std::array::from_fn(|axis| block.origin[axis].saturating_add(local[axis]));
                let index = row_major_index(coordinate, self.original_shape);
                let value = cells.next().expect("valid local block has every cell");
                debug_assert!(destination[index].is_none());
                destination[index] = Some(value);
                if !advance_coordinate(&mut local, block.shape) {
                    break;
                }
            }
            debug_assert!(cells.next().is_none());
        }
        SpatialField {
            shape: self.original_shape,
            cells: destination
                .into_iter()
                .map(|cell| cell.expect("local blocks cover the original field"))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusedPartition<T, const D: usize> {
    original_shape: [usize; D],
    profile: FocusProfile<D>,
    foci: Vec<[usize; D]>,
    regions: Vec<LocalBlock<T, D>>,
}

impl<T, const D: usize> FocusedPartition<T, D> {
    pub const fn original_shape(&self) -> [usize; D] {
        self.original_shape
    }

    pub const fn profile(&self) -> FocusProfile<D> {
        self.profile
    }

    pub fn foci(&self) -> &[[usize; D]] {
        &self.foci
    }

    pub fn regions(&self) -> &[LocalBlock<T, D>] {
        &self.regions
    }

    pub fn map<U>(self, mut transform: impl FnMut(T) -> U) -> FocusedPartition<U, D> {
        FocusedPartition {
            original_shape: self.original_shape,
            profile: self.profile,
            foci: self.foci,
            regions: self
                .regions
                .into_iter()
                .map(|region| region.map(&mut transform))
                .collect(),
        }
    }

    pub fn reassemble(self) -> SpatialField<T, D> {
        reassemble_regions(self.original_shape, self.regions)
    }

    pub fn transduce_complete<U: Clone>(
        self,
        identity: U,
        mut transform: impl FnMut(T) -> U,
        mut combine: impl FnMut(U, U) -> U,
    ) -> FocusedField<U, D> {
        let regions = self
            .regions
            .into_iter()
            .map(|region| {
                let mut value = identity.clone();
                let mut complete = true;
                for cell in region.cells {
                    match cell {
                        Availability::Available(cell) => {
                            value = combine(value, transform(cell));
                        }
                        Availability::Unavailable => complete = false,
                    }
                }
                FocusedRegion {
                    origin: region.origin,
                    shape: region.shape,
                    value: if complete {
                        Availability::Available(value)
                    } else {
                        Availability::Unavailable
                    },
                }
            })
            .collect();
        FocusedField {
            original_shape: self.original_shape,
            profile: self.profile,
            foci: self.foci,
            regions,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusedRegion<T, const D: usize> {
    origin: [usize; D],
    shape: [usize; D],
    value: Availability<T>,
}

impl<T, const D: usize> FocusedRegion<T, D> {
    pub const fn origin(&self) -> [usize; D] {
        self.origin
    }

    pub const fn shape(&self) -> [usize; D] {
        self.shape
    }

    pub const fn value(&self) -> &Availability<T> {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusedField<T, const D: usize> {
    original_shape: [usize; D],
    profile: FocusProfile<D>,
    foci: Vec<[usize; D]>,
    regions: Vec<FocusedRegion<T, D>>,
}

impl<T, const D: usize> FocusedField<T, D> {
    pub const fn original_shape(&self) -> [usize; D] {
        self.original_shape
    }

    pub const fn profile(&self) -> FocusProfile<D> {
        self.profile
    }

    pub fn foci(&self) -> &[[usize; D]] {
        &self.foci
    }

    pub fn regions(&self) -> &[FocusedRegion<T, D>] {
        &self.regions
    }

    pub fn map<U>(self, mut transform: impl FnMut(T) -> U) -> FocusedField<U, D> {
        FocusedField {
            original_shape: self.original_shape,
            profile: self.profile,
            foci: self.foci,
            regions: self
                .regions
                .into_iter()
                .map(|region| FocusedRegion {
                    origin: region.origin,
                    shape: region.shape,
                    value: region.value.map(&mut transform),
                })
                .collect(),
        }
    }

    pub fn into_receptor_frame(self) -> FocusedReceptorFrame<T, D> {
        let active_region_count = self.regions.len();
        debug_assert!(active_region_count <= self.profile.region_bound);
        let mut slots = self
            .regions
            .into_iter()
            .map(|region| region.value)
            .collect::<Vec<_>>();
        slots.extend(
            std::iter::repeat_with(|| Availability::Unavailable).take(
                self.profile
                    .region_bound
                    .saturating_sub(active_region_count),
            ),
        );
        FocusedReceptorFrame {
            original_shape: self.original_shape,
            profile: self.profile,
            foci: self.foci,
            active_region_count,
            slots,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusedReceptorFrame<T, const D: usize> {
    original_shape: [usize; D],
    profile: FocusProfile<D>,
    foci: Vec<[usize; D]>,
    active_region_count: usize,
    slots: Vec<Availability<T>>,
}

impl<T, const D: usize> FocusedReceptorFrame<T, D> {
    pub const fn original_shape(&self) -> [usize; D] {
        self.original_shape
    }

    pub const fn profile(&self) -> FocusProfile<D> {
        self.profile
    }

    pub fn foci(&self) -> &[[usize; D]] {
        &self.foci
    }

    pub const fn active_region_count(&self) -> usize {
        self.active_region_count
    }

    pub fn slots(&self) -> &[Availability<T>] {
        &self.slots
    }

    pub fn map<U>(self, mut transform: impl FnMut(T) -> U) -> FocusedReceptorFrame<U, D> {
        FocusedReceptorFrame {
            original_shape: self.original_shape,
            profile: self.profile,
            foci: self.foci,
            active_region_count: self.active_region_count,
            slots: self
                .slots
                .into_iter()
                .map(|slot| slot.map(&mut transform))
                .collect(),
        }
    }
}

#[derive(Clone, Copy)]
struct Region<const D: usize> {
    origin: [usize; D],
    shape: [usize; D],
}

impl<const D: usize> Region<D> {
    fn is_splittable(self) -> bool {
        self.shape.into_iter().any(|extent| extent > 1)
    }

    fn contains_any(self, foci: &[[usize; D]]) -> bool {
        foci.iter().any(|focus| {
            (0..D).all(|axis| {
                focus[axis] >= self.origin[axis]
                    && focus[axis] < self.origin[axis].saturating_add(self.shape[axis])
            })
        })
    }

    fn split(self) -> Vec<Self> {
        let parts = std::array::from_fn(|axis| match self.shape[axis] {
            0 | 1 => 1,
            extent if extent % 2 == 0 => 2,
            _ => 3,
        });
        let mut part = [0; D];
        let mut children = Vec::new();
        loop {
            let shape = std::array::from_fn(|axis| {
                let half = self.shape[axis] / 2;
                match (parts[axis], part[axis]) {
                    (1, _) => self.shape[axis],
                    (2, _) => half,
                    (3, 1) => 1,
                    (3, _) => half,
                    _ => unreachable!("validated symmetric split"),
                }
            });
            let origin = std::array::from_fn(|axis| {
                let half = self.shape[axis] / 2;
                match (parts[axis], part[axis]) {
                    (1, _) | (_, 0) => self.origin[axis],
                    (2, 1) | (3, 1) => self.origin[axis].saturating_add(half),
                    (3, 2) => self.origin[axis].saturating_add(half).saturating_add(1),
                    _ => unreachable!("validated symmetric split"),
                }
            });
            children.push(Self { origin, shape });
            if !advance_coordinate(&mut part, parts) {
                break;
            }
        }
        children
    }
}

fn move_cells_into_regions<T, const D: usize>(
    original_shape: [usize; D],
    cells: Vec<Availability<T>>,
    regions: &[Region<D>],
) -> Vec<LocalBlock<T, D>> {
    let mut source = cells.into_iter().map(Some).collect::<Vec<_>>();
    let mut blocks = Vec::with_capacity(regions.len());
    for region in regions {
        let mut cells = Vec::with_capacity(cell_count_of_valid_shape(region.shape));
        let mut local = [0; D];
        loop {
            let coordinate =
                std::array::from_fn(|axis| region.origin[axis].saturating_add(local[axis]));
            let index = row_major_index(coordinate, original_shape);
            cells.push(
                source[index]
                    .take()
                    .expect("focused regions do not overlap"),
            );
            if !advance_coordinate(&mut local, region.shape) {
                break;
            }
        }
        blocks.push(LocalBlock {
            origin: region.origin,
            shape: region.shape,
            cells,
        });
    }
    debug_assert!(source.iter().all(Option::is_none));
    blocks
}

fn reassemble_regions<T, const D: usize>(
    original_shape: [usize; D],
    regions: Vec<LocalBlock<T, D>>,
) -> SpatialField<T, D> {
    let cell_count = cell_count_of_valid_shape(original_shape);
    let mut destination = (0..cell_count).map(|_| None).collect::<Vec<_>>();
    for region in regions {
        let mut local = [0; D];
        let mut cells = region.cells.into_iter();
        loop {
            let coordinate =
                std::array::from_fn(|axis| region.origin[axis].saturating_add(local[axis]));
            let index = row_major_index(coordinate, original_shape);
            let value = cells.next().expect("focused region has every cell");
            debug_assert!(destination[index].is_none());
            destination[index] = Some(value);
            if !advance_coordinate(&mut local, region.shape) {
                break;
            }
        }
        debug_assert!(cells.next().is_none());
    }
    SpatialField {
        shape: original_shape,
        cells: destination
            .into_iter()
            .map(|cell| cell.expect("focused regions cover the original field"))
            .collect(),
    }
}

fn checked_cell_count<const D: usize>(shape: [usize; D]) -> Result<usize, SpatialFieldError> {
    if D == 0 {
        return Err(SpatialFieldError::NoDimensions);
    }
    let mut count = 1_usize;
    for (axis, extent) in shape.into_iter().enumerate() {
        if extent == 0 {
            return Err(SpatialFieldError::ZeroDimension { axis });
        }
        count = count
            .checked_mul(extent)
            .ok_or(SpatialFieldError::CellCountOverflow)?;
    }
    Ok(count)
}

fn cell_count_of_valid_shape<const D: usize>(shape: [usize; D]) -> usize {
    shape
        .into_iter()
        .try_fold(1_usize, usize::checked_mul)
        .expect("validated spatial shape has a bounded cell count")
}

fn row_major_index<const D: usize>(coordinate: [usize; D], shape: [usize; D]) -> usize {
    coordinate
        .into_iter()
        .zip(shape)
        .fold(0_usize, |index, (coordinate, extent)| {
            index
                .checked_mul(extent)
                .and_then(|value| value.checked_add(coordinate))
                .expect("validated coordinate has a bounded row-major index")
        })
}

fn advance_coordinate<const D: usize>(coordinate: &mut [usize; D], shape: [usize; D]) -> bool {
    for axis in (0..D).rev() {
        coordinate[axis] += 1;
        if coordinate[axis] < shape[axis] {
            return true;
        }
        coordinate[axis] = 0;
    }
    false
}

fn advance_block_origin<const D: usize>(
    origin: &mut [usize; D],
    field_shape: [usize; D],
    block_shape: [usize; D],
) -> bool {
    for axis in (0..D).rev() {
        let next = origin[axis].saturating_add(block_shape[axis]);
        if next < field_shape[axis] {
            origin[axis] = next;
            return true;
        }
        origin[axis] = 0;
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Port(pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutedSignal<T> {
    pub port: Port,
    pub signal: Signal<T>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InteractionStep<O, E> {
    pub before: O,
    pub effect: Option<E>,
    pub after: O,
}

pub fn interact<State, Observation, Command, Effect>(
    state: &mut State,
    mut sense: impl FnMut(&State) -> Observation,
    choose: impl FnOnce(&Observation) -> Option<Command>,
    mut act: impl FnMut(&mut State, Command) -> Effect,
) -> InteractionStep<Observation, Effect> {
    let before = sense(state);
    let effect = choose(&before).map(|command| act(state, command));
    let after = sense(state);
    InteractionStep {
        before,
        effect,
        after,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EffectMode {
    Apply,
    Identity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnknownPort {
    pub port: Port,
    pub ports: usize,
}

impl fmt::Display for UnknownPort {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "port {} is outside a frame with {} ports",
            self.port.0, self.ports
        )
    }
}

impl std::error::Error for UnknownPort {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandCollector<C, const N: usize> {
    commands: [Option<C>; N],
}

impl<C, const N: usize> Default for CommandCollector<C, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C, const N: usize> CommandCollector<C, N> {
    pub fn new() -> Self {
        Self {
            commands: std::array::from_fn(|_| None),
        }
    }

    pub fn add(
        &mut self,
        port: Port,
        command: C,
        combine: impl FnOnce(C, C) -> C,
    ) -> Result<(), UnknownPort> {
        let index = port_index::<N>(port)?;
        let slot = &mut self.commands[index];
        *slot = Some(match slot.take() {
            Some(existing) => combine(existing, command),
            None => command,
        });
        Ok(())
    }

    pub fn finish(self) -> CommandFrame<C, N> {
        CommandFrame {
            commands: self.commands,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandFrame<C, const N: usize> {
    commands: [Option<C>; N],
}

impl<C, const N: usize> CommandFrame<C, N> {
    pub fn command(&self, port: Port) -> Result<Option<&C>, UnknownPort> {
        Ok(self.commands[port_index::<N>(port)?].as_ref())
    }

    pub const fn commands(&self) -> &[Option<C>; N] {
        &self.commands
    }

    pub fn constrain(&mut self, port: Port, mode: EffectMode) -> Result<(), UnknownPort> {
        let index = port_index::<N>(port)?;
        if mode == EffectMode::Identity {
            self.commands[index] = None;
        }
        Ok(())
    }

    pub fn into_commands(self) -> [Option<C>; N] {
        self.commands
    }
}

fn port_index<const N: usize>(port: Port) -> Result<usize, UnknownPort> {
    let index = usize::try_from(port.0).map_err(|_| UnknownPort { port, ports: N })?;
    if index < N {
        Ok(index)
    } else {
        Err(UnknownPort { port, ports: N })
    }
}

pub trait Driver<I> {
    type Output;

    fn step(&mut self, input: I) -> Self::Output;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Identity;

impl<T> Driver<T> for Identity {
    type Output = T;

    fn step(&mut self, input: T) -> Self::Output {
        input
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Then<A, B> {
    first: A,
    second: B,
}

pub const fn then<A, B>(first: A, second: B) -> Then<A, B> {
    Then { first, second }
}

impl<I, A, B> Driver<I> for Then<A, B>
where
    A: Driver<I>,
    B: Driver<A::Output>,
{
    type Output = B::Output;

    fn step(&mut self, input: I) -> Self::Output {
        let intermediate = self.first.step(input);
        self.second.step(intermediate)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parallel<A, B> {
    left: A,
    right: B,
}

pub const fn parallel<A, B>(left: A, right: B) -> Parallel<A, B> {
    Parallel { left, right }
}

impl<I, J, A, B> Driver<(I, J)> for Parallel<A, B>
where
    A: Driver<I>,
    B: Driver<J>,
{
    type Output = (A::Output, B::Output);

    fn step(&mut self, (left, right): (I, J)) -> Self::Output {
        (self.left.step(left), self.right.step(right))
    }
}

/// Composes an opportunity with either its event or the identity (no event).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpportunityGate;

impl<T> Driver<(bool, T)> for OpportunityGate {
    type Output = Option<T>;

    fn step(&mut self, (open, event): (bool, T)) -> Self::Output {
        open.then_some(event)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeDetector<T> {
    previous: Option<T>,
}

impl<T> Default for ChangeDetector<T> {
    fn default() -> Self {
        Self { previous: None }
    }
}

impl<T: Clone + PartialEq> Driver<Signal<T>> for ChangeDetector<T> {
    type Output = Signal<T>;

    fn step(&mut self, input: Signal<T>) -> Self::Output {
        let incidence = match self.previous.as_ref() {
            Some(previous) if previous != &input.value => Incidence::Transition,
            _ => Incidence::Sample,
        };
        let output = Signal {
            origin: input.origin,
            incidence,
            value: input.value,
        };
        self.previous = Some(output.value.clone());
        output
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quantizer {
    width: u16,
    bins: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuantizerError {
    ZeroWidth,
    ZeroBins,
}

impl fmt::Display for QuantizerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroWidth => formatter.write_str("quantizer width must be positive"),
            Self::ZeroBins => formatter.write_str("quantizer bin count must be positive"),
        }
    }
}

impl std::error::Error for QuantizerError {}

impl Quantizer {
    pub const fn new(width: u16, bins: u16) -> Result<Self, QuantizerError> {
        if width == 0 {
            Err(QuantizerError::ZeroWidth)
        } else if bins == 0 {
            Err(QuantizerError::ZeroBins)
        } else {
            Ok(Self { width, bins })
        }
    }

    pub const fn bin(self, value: u16) -> u16 {
        let bin = value / self.width;
        if bin >= self.bins {
            self.bins - 1
        } else {
            bin
        }
    }
}

impl Driver<Signal<u16>> for Quantizer {
    type Output = Signal<u16>;

    fn step(&mut self, input: Signal<u16>) -> Self::Output {
        input.map(|value| self.bin(value))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Threshold {
    pub bin: u16,
    pub final_bin: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ThresholdFactorizer {
    previous: Option<u16>,
}

impl Driver<Signal<u16>> for ThresholdFactorizer {
    type Output = Vec<Signal<Threshold>>;

    fn step(&mut self, input: Signal<u16>) -> Self::Output {
        let mut output = Vec::new();
        if input.incidence == Incidence::Transition {
            if let Some(previous) = self.previous {
                if previous < input.value {
                    for bin in previous.saturating_add(1)..input.value {
                        output.push(Signal::new(
                            input.origin,
                            Incidence::Sample,
                            Threshold {
                                bin,
                                final_bin: false,
                            },
                        ));
                    }
                } else {
                    for bin in (input.value.saturating_add(1)..previous).rev() {
                        output.push(Signal::new(
                            input.origin,
                            Incidence::Sample,
                            Threshold {
                                bin,
                                final_bin: false,
                            },
                        ));
                    }
                }
            }
        }
        self.previous = Some(input.value);
        output.push(Signal::new(
            input.origin,
            input.incidence,
            Threshold {
                bin: input.value,
                final_bin: true,
            },
        ));
        output
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpposedEffort {
    pub decrease: u16,
    pub increase: u16,
}

impl OpposedEffort {
    pub const fn new(decrease: u16, increase: u16) -> Self {
        Self { decrease, increase }
    }

    pub const fn net(self) -> i32 {
        opposed_net(self.decrease, self.increase)
    }

    pub const fn combine_bounded(self, other: Self, maximum: u16) -> Self {
        let decrease = self.decrease.saturating_add(other.decrease);
        let increase = self.increase.saturating_add(other.increase);
        Self {
            decrease: if decrease > maximum {
                maximum
            } else {
                decrease
            },
            increase: if increase > maximum {
                maximum
            } else {
                increase
            },
        }
    }
}

pub const fn opposed_net(decrease: u16, increase: u16) -> i32 {
    increase as i32 - decrease as i32
}

pub const fn signed_channels(value: i16) -> [u8; 2] {
    if value < 0 {
        [bounded_magnitude(value.unsigned_abs()), 0]
    } else {
        [0, bounded_magnitude(value.unsigned_abs())]
    }
}

pub const fn bounded_magnitude(value: u16) -> u8 {
    if value > u8::MAX as u16 {
        u8::MAX
    } else {
        value as u8
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AxisReading {
    pub position: i32,
    pub at_lower_limit: bool,
    pub at_upper_limit: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AxisEffect {
    pub requested: i32,
    pub actual: i32,
    pub feedback: Signal<AxisReading>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedAxis {
    position: i32,
    lower: i32,
    upper: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisError {
    ReversedBounds,
    PositionOutsideBounds,
}

impl fmt::Display for AxisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReversedBounds => formatter.write_str("axis lower bound exceeds upper bound"),
            Self::PositionOutsideBounds => {
                formatter.write_str("axis position is outside its bounds")
            }
        }
    }
}

impl std::error::Error for AxisError {}

impl BoundedAxis {
    pub fn new(position: i32, lower: i32, upper: i32) -> Result<Self, AxisError> {
        if lower > upper {
            return Err(AxisError::ReversedBounds);
        }
        if position < lower || position > upper {
            return Err(AxisError::PositionOutsideBounds);
        }
        Ok(Self {
            position,
            lower,
            upper,
        })
    }

    pub const fn position(&self) -> i32 {
        self.position
    }
}

impl Driver<Signal<OpposedEffort>> for BoundedAxis {
    type Output = AxisEffect;

    fn step(&mut self, input: Signal<OpposedEffort>) -> Self::Output {
        let requested = input.value.net();
        let lower = self.lower;
        let upper = self.upper;
        let interaction = interact(
            &mut self.position,
            |position| AxisReading {
                position: *position,
                at_lower_limit: *position == lower,
                at_upper_limit: *position == upper,
            },
            |_| Some(requested),
            |position, movement| {
                let before = *position;
                *position = position.saturating_add(movement).clamp(lower, upper);
                *position - before
            },
        );
        let actual = interaction
            .effect
            .expect("bounded axis always receives an available command");
        AxisEffect {
            requested,
            actual,
            feedback: Signal {
                origin: input.origin,
                incidence: if actual == 0 {
                    Incidence::Sample
                } else {
                    Incidence::Transition
                },
                value: interaction.after,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DriverBank<D> {
    drivers: Vec<D>,
}

impl<D> DriverBank<D> {
    pub fn new(drivers: Vec<D>) -> Self {
        Self { drivers }
    }

    pub fn len(&self) -> usize {
        self.drivers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.drivers.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BankArityError {
    pub drivers: usize,
    pub inputs: usize,
}

impl fmt::Display for BankArityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "driver bank has {} drivers but received {} inputs",
            self.drivers, self.inputs
        )
    }
}

impl std::error::Error for BankArityError {}

impl<D> DriverBank<D> {
    pub fn step<I>(&mut self, inputs: Vec<I>) -> Result<Vec<D::Output>, BankArityError>
    where
        D: Driver<I>,
    {
        if inputs.len() != self.drivers.len() {
            return Err(BankArityError {
                drivers: self.drivers.len(),
                inputs: inputs.len(),
            });
        }
        Ok(self
            .drivers
            .iter_mut()
            .zip(inputs)
            .map(|(driver, input)| driver.step(input))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Add(i32);

    impl Driver<i32> for Add {
        type Output = i32;

        fn step(&mut self, input: i32) -> Self::Output {
            input + self.0
        }
    }

    #[test]
    fn identity_and_association_hold() {
        assert_eq!(then(Identity, Add(2)).step(3), 5);
        assert_eq!(then(Add(2), Identity).step(3), 5);
        let mut left = then(then(Add(1), Add(2)), Add(3));
        let mut right = then(Add(1), then(Add(2), Add(3)));
        assert_eq!(left.step(7), right.step(7));
    }

    #[test]
    fn mapping_routing_and_fanout_preserve_the_cause() {
        let signal = Signal::new(Origin(17), Incidence::Transition, 4_u8);
        let mapped = signal.clone().map(u16::from);
        assert_eq!(
            (mapped.origin, mapped.incidence),
            (signal.origin, signal.incidence)
        );
        let routed = signal.clone().route(Port(3));
        assert_eq!(routed.signal, signal);
        assert!(signal.fan_out::<4>().iter().all(|copy| copy == &signal));
    }

    #[test]
    fn repeated_observation_is_not_a_transition() {
        let mut detector = ChangeDetector::default();
        let input = || Signal::new(Origin(9), Incidence::Sample, 42_u8);
        assert_eq!(detector.step(input()).incidence, Incidence::Sample);
        assert_eq!(detector.step(input()).incidence, Incidence::Sample);
        assert_eq!(
            detector
                .step(Signal::new(Origin(9), Incidence::Sample, 43))
                .incidence,
            Incidence::Transition
        );
    }

    #[test]
    fn quantized_thresholds_preserve_order_and_one_transition() {
        let quantizer = Quantizer::new(64, 4).unwrap();
        let mut sensor = then(
            quantizer,
            then(ChangeDetector::default(), ThresholdFactorizer::default()),
        );
        let first = sensor.step(Signal::new(Origin(5), Incidence::Sample, 1_u16));
        assert_eq!(first.len(), 1);
        assert_eq!(first[0].incidence, Incidence::Sample);

        let rising = sensor.step(Signal::new(Origin(5), Incidence::Sample, 255_u16));
        assert_eq!(
            rising
                .iter()
                .map(|signal| (signal.value.bin, signal.incidence, signal.value.final_bin))
                .collect::<Vec<_>>(),
            vec![
                (1, Incidence::Sample, false),
                (2, Incidence::Sample, false),
                (3, Incidence::Transition, true),
            ]
        );
        assert!(rising.iter().all(|signal| signal.origin == Origin(5)));

        let repeated = sensor.step(Signal::new(Origin(5), Incidence::Sample, 254_u16));
        assert_eq!(repeated.len(), 1);
        assert_eq!(repeated[0].incidence, Incidence::Sample);
    }

    #[test]
    fn cloned_driver_replays_the_exact_next_signal() {
        let mut original = ChangeDetector::default();
        original.step(Signal::new(Origin(4), Incidence::Sample, 8_u8));
        let mut replay = original.clone();
        let next = Signal::new(Origin(4), Incidence::Sample, 9_u8);
        assert_eq!(original.step(next.clone()), replay.step(next));
    }

    #[test]
    fn bounded_axis_reports_only_actual_change() {
        let mut axis = BoundedAxis::new(5, 0, 10).unwrap();
        let hold = axis.step(Signal::new(
            Origin(1),
            Incidence::Sample,
            OpposedEffort::new(3, 3),
        ));
        assert_eq!((hold.requested, hold.actual), (0, 0));
        assert_eq!(hold.feedback.incidence, Incidence::Sample);
        let upper = axis.step(Signal::new(
            Origin(2),
            Incidence::Transition,
            OpposedEffort::new(0, 20),
        ));
        assert_eq!((upper.requested, upper.actual), (20, 5));
        assert_eq!(upper.feedback.incidence, Incidence::Transition);
        assert_eq!(upper.feedback.origin, Origin(2));
        assert!(upper.feedback.value.at_upper_limit);
        let saturated = axis.step(Signal::new(
            Origin(3),
            Incidence::Transition,
            OpposedEffort::new(0, 1),
        ));
        assert_eq!(saturated.actual, 0);
        assert_eq!(saturated.feedback.incidence, Incidence::Sample);
    }
}
