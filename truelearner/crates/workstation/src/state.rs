use crate::WorkstationError;
use serde::{Deserialize, Serialize};

pub const BODY_MAX: i16 = 1_023;
/// One fingertip contact surface.
pub const TOUCH_SITES: usize = 1;
pub const AXIS_COUNT: usize = 8;
const MAX_PIXELS: usize = 1_048_576;
const MID: i16 = (BODY_MAX + 1) / 2;

/// Three generic physical light intensities at one screen pixel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub const fn gray(value: u8) -> Self {
        Self::new(value, value, value)
    }

    pub const fn red(self) -> u8 {
        self.red
    }

    pub const fn green(self) -> u8 {
        self.green
    }

    pub const fn blue(self) -> u8 {
        self.blue
    }

    /// Integer Rec. 709 luminance. The coefficients sum to 256, so gray is
    /// identity and existing monochrome applications retain exact behavior.
    pub const fn luminance(self) -> u8 {
        ((self.red as u32 * 54 + self.green as u32 * 183 + self.blue as u32 * 19 + 128) / 256) as u8
    }

    pub const fn opponents(self) -> ChromaticSignal {
        ChromaticSignal {
            red_green: self.red as i16 - self.green as i16,
            blue_yellow: self.blue as i16 - ((self.red as i16 + self.green as i16) / 2),
        }
    }
}

/// Two local opponent responses. Neutral light is zero on both axes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChromaticSignal {
    red_green: i16,
    blue_yellow: i16,
}

impl ChromaticSignal {
    pub const fn new(red_green: i16, blue_yellow: i16) -> Self {
        Self {
            red_green,
            blue_yellow,
        }
    }

    pub const fn red_green(self) -> i16 {
        self.red_green
    }

    pub const fn blue_yellow(self) -> i16 {
        self.blue_yellow
    }
}

/// A spatial field of the two retinal opponent responses.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChromaticField {
    width: u16,
    height: u16,
    pixels: Vec<ChromaticSignal>,
}

impl ChromaticField {
    pub fn new(
        width: u16,
        height: u16,
        pixels: Vec<ChromaticSignal>,
    ) -> Result<Self, WorkstationError> {
        let field = Self {
            width,
            height,
            pixels,
        };
        field.validate()?;
        Ok(field)
    }

    pub fn neutral(width: u16, height: u16) -> Result<Self, WorkstationError> {
        let count = usize::from(width)
            .checked_mul(usize::from(height))
            .ok_or(WorkstationError::LightFieldTooLarge)?;
        Self::new(width, height, vec![ChromaticSignal::default(); count])
    }

    pub const fn width(&self) -> u16 {
        self.width
    }

    pub const fn height(&self) -> u16 {
        self.height
    }

    pub fn pixels(&self) -> &[ChromaticSignal] {
        &self.pixels
    }

    fn validate(&self) -> Result<(), WorkstationError> {
        if self.width == 0 || self.height == 0 {
            return Err(WorkstationError::EmptyLightField);
        }
        let expected = usize::from(self.width)
            .checked_mul(usize::from(self.height))
            .ok_or(WorkstationError::LightFieldTooLarge)?;
        if expected > MAX_PIXELS {
            return Err(WorkstationError::LightFieldTooLarge);
        }
        if self.pixels.len() != expected {
            return Err(WorkstationError::LightLength);
        }
        if self.pixels.iter().any(|signal| {
            !(-255..=255).contains(&signal.red_green) || !(-255..=255).contains(&signal.blue_yellow)
        }) {
            return Err(WorkstationError::InvalidState);
        }
        Ok(())
    }
}

/// A bounded RGB screen raster. Retinal projections derive luminance and
/// chromatic opponents from this one physical surface.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorField {
    width: u16,
    height: u16,
    pixels: Vec<Rgb>,
}

impl ColorField {
    pub fn new(width: u16, height: u16, pixels: Vec<Rgb>) -> Result<Self, WorkstationError> {
        let field = Self {
            width,
            height,
            pixels,
        };
        field.validate()?;
        Ok(field)
    }

    pub fn filled(width: u16, height: u16, value: Rgb) -> Result<Self, WorkstationError> {
        let count = usize::from(width)
            .checked_mul(usize::from(height))
            .ok_or(WorkstationError::LightFieldTooLarge)?;
        Self::new(width, height, vec![value; count])
    }

    pub fn from_luminance(field: &LightField) -> Self {
        Self {
            width: field.width,
            height: field.height,
            pixels: field.pixels.iter().copied().map(Rgb::gray).collect(),
        }
    }

    pub const fn width(&self) -> u16 {
        self.width
    }

    pub const fn height(&self) -> u16 {
        self.height
    }

    pub fn pixels(&self) -> &[Rgb] {
        &self.pixels
    }

    fn validate(&self) -> Result<(), WorkstationError> {
        if self.width == 0 || self.height == 0 {
            return Err(WorkstationError::EmptyLightField);
        }
        let expected = usize::from(self.width)
            .checked_mul(usize::from(self.height))
            .ok_or(WorkstationError::LightFieldTooLarge)?;
        if expected > MAX_PIXELS {
            return Err(WorkstationError::LightFieldTooLarge);
        }
        if self.pixels.len() != expected {
            return Err(WorkstationError::LightLength);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct OpposedEffort {
    decrease: u16,
    increase: u16,
}

impl OpposedEffort {
    const fn new(decrease: u16, increase: u16) -> Self {
        Self { decrease, increase }
    }

    fn combine_bounded(self, other: Self, maximum: u16) -> Self {
        Self {
            decrease: self.decrease.saturating_add(other.decrease).min(maximum),
            increase: self.increase.saturating_add(other.increase).min(maximum),
        }
    }

    const fn net(self) -> i32 {
        self.increase as i32 - self.decrease as i32
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Eye {
    Left,
    Right,
}

impl Eye {
    pub const ALL: [Self; 2] = [Self::Left, Self::Right];

    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Left => 0,
            Self::Right => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Decrease,
    Increase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyControl {
    axis: BodyAxis,
    direction: Direction,
}

impl BodyControl {
    pub const fn new(axis: BodyAxis, direction: Direction) -> Self {
        Self { axis, direction }
    }

    pub const fn axis(self) -> BodyAxis {
        self.axis
    }

    pub const fn direction(self) -> Direction {
        self.direction
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyAxis {
    EyeHorizontal { eye: Eye },
    EyeVertical { eye: Eye },
    PalmHorizontal,
    PalmVertical,
    PalmDepth,
    FingerFlexion,
}

impl BodyAxis {
    pub const ALL: [Self; AXIS_COUNT] = [
        Self::EyeHorizontal { eye: Eye::Left },
        Self::EyeVertical { eye: Eye::Left },
        Self::EyeHorizontal { eye: Eye::Right },
        Self::EyeVertical { eye: Eye::Right },
        Self::PalmHorizontal,
        Self::PalmVertical,
        Self::PalmDepth,
        Self::FingerFlexion,
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::EyeHorizontal { eye } => eye.index() * 2,
            Self::EyeVertical { eye } => eye.index() * 2 + 1,
            Self::PalmHorizontal => 4,
            Self::PalmVertical => 5,
            Self::PalmDepth => 6,
            Self::FingerFlexion => 7,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ActuatorFrame {
    pub(crate) axes: [OpposedEffort; AXIS_COUNT],
}

impl Default for ActuatorFrame {
    fn default() -> Self {
        Self {
            axes: [OpposedEffort::default(); AXIS_COUNT],
        }
    }
}

impl ActuatorFrame {
    pub(crate) fn activate(&mut self, axis: BodyAxis, direction: Direction, impulse: u16) {
        let effort = &mut self.axes[axis.index()];
        let command = match direction {
            Direction::Decrease => OpposedEffort::new(impulse, 0),
            Direction::Increase => OpposedEffort::new(0, impulse),
        };
        *effort = effort.combine_bounded(command, BODY_MAX as u16);
    }

    /// Adds only enough external reaction to cancel inward effort. A supporting
    /// surface cannot pull the body outward, and it does not oppose another
    /// axis such as a sideways slip.
    pub(crate) fn resist_increase(&mut self, axis: BodyAxis, resistance: u16) {
        let effort = &mut self.axes[axis.index()];
        let unmatched = effort.increase.saturating_sub(effort.decrease);
        effort.decrease = effort
            .decrease
            .saturating_add(unmatched.min(resistance))
            .min(BODY_MAX as u16);
    }

    pub(crate) fn net(&self, axis: BodyAxis) -> i32 {
        self.axes[axis.index()].net()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyMovement {
    pub axis: BodyAxis,
    pub decrease_effort: u16,
    pub increase_effort: u16,
    pub net_impulse: i16,
    pub velocity: i16,
    pub changed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AxisProprioception {
    pub axis: BodyAxis,
    pub position: i16,
    pub velocity: i16,
    pub decrease_effort: u16,
    pub increase_effort: u16,
    pub at_lower_limit: bool,
    pub at_upper_limit: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct AxisDynamics {
    velocity: i16,
    decrease_effort: u16,
    increase_effort: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    x: i16,
    y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Result<Self, WorkstationError> {
        let point = Self { x, y };
        point.validate()?;
        Ok(point)
    }

    pub const fn x(self) -> i16 {
        self.x
    }

    pub const fn y(self) -> i16 {
        self.y
    }

    fn validate(self) -> Result<(), WorkstationError> {
        if (0..=BODY_MAX).contains(&self.x) && (0..=BODY_MAX).contains(&self.y) {
            Ok(())
        } else {
            Err(WorkstationError::InvalidState)
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandPoint {
    x: i16,
    y: i16,
    depth: i16,
}

impl HandPoint {
    pub const fn x(self) -> i16 {
        self.x
    }

    pub const fn y(self) -> i16 {
        self.y
    }

    pub const fn depth(self) -> i16 {
        self.depth
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightField {
    width: u16,
    height: u16,
    pixels: Vec<u8>,
}

impl LightField {
    pub fn new(width: u16, height: u16, pixels: Vec<u8>) -> Result<Self, WorkstationError> {
        let field = Self {
            width,
            height,
            pixels,
        };
        field.validate()?;
        Ok(field)
    }

    pub fn filled(width: u16, height: u16, value: u8) -> Result<Self, WorkstationError> {
        let count = usize::from(width)
            .checked_mul(usize::from(height))
            .ok_or(WorkstationError::LightFieldTooLarge)?;
        Self::new(width, height, vec![value; count])
    }

    pub const fn width(&self) -> u16 {
        self.width
    }

    pub const fn height(&self) -> u16 {
        self.height
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn sample(&self, position: Point) -> u8 {
        let x = usize::try_from(
            i32::from(position.x) * i32::from(self.width.saturating_sub(1)) / i32::from(BODY_MAX),
        )
        .unwrap_or(0);
        let y = usize::try_from(
            i32::from(position.y) * i32::from(self.height.saturating_sub(1)) / i32::from(BODY_MAX),
        )
        .unwrap_or(0);
        self.pixels[y.saturating_mul(usize::from(self.width)).saturating_add(x)]
    }

    fn validate(&self) -> Result<(), WorkstationError> {
        if self.width == 0 || self.height == 0 {
            return Err(WorkstationError::EmptyLightField);
        }
        let expected = usize::from(self.width)
            .checked_mul(usize::from(self.height))
            .ok_or(WorkstationError::LightFieldTooLarge)?;
        if expected > MAX_PIXELS {
            return Err(WorkstationError::LightFieldTooLarge);
        }
        if self.pixels.len() != expected {
            return Err(WorkstationError::LightLength);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactSample {
    pressure: u16,
    slip: i16,
}

impl ContactSample {
    pub fn new(pressure: u16, slip: i16) -> Result<Self, WorkstationError> {
        let sample = Self { pressure, slip };
        sample.validate()?;
        Ok(sample)
    }

    pub const fn pressure(self) -> u16 {
        self.pressure
    }

    pub const fn slip(self) -> i16 {
        self.slip
    }

    fn validate(self) -> Result<(), WorkstationError> {
        if self.pressure <= BODY_MAX as u16 && (-BODY_MAX..=BODY_MAX).contains(&self.slip) {
            Ok(())
        } else {
            Err(WorkstationError::ContactOutsideRange)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EyeState {
    gaze: Point,
}

impl EyeState {
    pub const fn gaze(&self) -> Point {
        self.gaze
    }
}

impl Default for EyeState {
    fn default() -> Self {
        Self {
            gaze: Point { x: MID, y: MID },
        }
    }
}

/// One arm carrying one independently flexing pointer finger.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandState {
    palm: HandPoint,
    finger_flexion: i16,
}

impl HandState {
    pub const fn palm(&self) -> HandPoint {
        self.palm
    }

    pub const fn finger_flexion(&self) -> i16 {
        self.finger_flexion
    }

    pub fn fingertip(&self) -> HandPoint {
        HandPoint {
            x: self.palm.x,
            y: self.palm.y,
            depth: add_bounded(
                self.palm.depth,
                i32::from(self.finger_flexion - MID) / 4,
                0,
                BODY_MAX,
            ),
        }
    }
}

impl Default for HandState {
    fn default() -> Self {
        Self {
            palm: HandPoint {
                x: MID,
                y: 768,
                depth: 256,
            },
            finger_flexion: MID,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationState {
    eyes: [EyeState; 2],
    hand: HandState,
    dynamics: [AxisDynamics; AXIS_COUNT],
}

impl WorkstationState {
    pub fn eye(&self, eye: Eye) -> &EyeState {
        &self.eyes[eye.index()]
    }

    pub const fn hand(&self) -> &HandState {
        &self.hand
    }

    pub fn proprioception(&self) -> [AxisProprioception; AXIS_COUNT] {
        std::array::from_fn(|index| {
            let axis = BodyAxis::ALL[index];
            let dynamics = self.dynamics[index];
            let (at_lower_limit, at_upper_limit) = self.limits(axis);
            AxisProprioception {
                axis,
                position: self.axis_position(axis),
                velocity: dynamics.velocity,
                decrease_effort: dynamics.decrease_effort,
                increase_effort: dynamics.increase_effort,
                at_lower_limit,
                at_upper_limit,
            }
        })
    }

    pub(crate) fn integrate(&mut self, frame: ActuatorFrame) -> Vec<BodyMovement> {
        self.dynamics.fill(AxisDynamics::default());
        let mut movements = Vec::new();
        for (index, effort) in frame.axes.into_iter().enumerate() {
            if effort == OpposedEffort::default() {
                continue;
            }
            let axis = BodyAxis::ALL[index];
            let before = self.axis_position(axis);
            let net_impulse = effort.net();
            self.apply_net(axis, net_impulse);
            let velocity = self.axis_position(axis).saturating_sub(before);
            self.dynamics[index] = AxisDynamics {
                velocity,
                decrease_effort: effort.decrease,
                increase_effort: effort.increase,
            };
            movements.push(BodyMovement {
                axis,
                decrease_effort: effort.decrease,
                increase_effort: effort.increase,
                net_impulse: i16::try_from(net_impulse).unwrap_or_else(|_| {
                    if net_impulse.is_negative() {
                        i16::MIN
                    } else {
                        i16::MAX
                    }
                }),
                velocity,
                changed: velocity != 0,
            });
        }
        movements
    }

    pub(crate) fn same_pose(&self, other: &Self) -> bool {
        self.eyes == other.eyes && self.hand == other.hand
    }

    fn axis_position(&self, axis: BodyAxis) -> i16 {
        match axis {
            BodyAxis::EyeHorizontal { eye } => self.eye(eye).gaze.x - MID,
            BodyAxis::EyeVertical { eye } => self.eye(eye).gaze.y - MID,
            BodyAxis::PalmHorizontal => self.hand.palm.x - MID,
            BodyAxis::PalmVertical => self.hand.palm.y - 768,
            BodyAxis::PalmDepth => self.hand.palm.depth - 256,
            BodyAxis::FingerFlexion => self.hand.finger_flexion - MID,
        }
    }

    pub(crate) fn limits(&self, axis: BodyAxis) -> (bool, bool) {
        match axis {
            BodyAxis::EyeHorizontal { eye } => {
                (self.eye(eye).gaze.x == 0, self.eye(eye).gaze.x == BODY_MAX)
            }
            BodyAxis::EyeVertical { eye } => {
                (self.eye(eye).gaze.y == 0, self.eye(eye).gaze.y == BODY_MAX)
            }
            BodyAxis::PalmHorizontal => (self.hand.palm.x == 0, self.hand.palm.x == BODY_MAX),
            BodyAxis::PalmVertical => (self.hand.palm.y == 0, self.hand.palm.y == BODY_MAX),
            BodyAxis::PalmDepth => (self.hand.palm.depth == 0, self.hand.palm.depth == BODY_MAX),
            BodyAxis::FingerFlexion => (
                self.hand.finger_flexion == 0,
                self.hand.finger_flexion == BODY_MAX,
            ),
        }
    }

    fn apply_net(&mut self, axis: BodyAxis, net_impulse: i32) {
        let amount = net_impulse.saturating_mul(axis_step(axis));
        match axis {
            BodyAxis::EyeHorizontal { eye } => {
                // The eye is rate-limited: whatever efforts sum, it moves
                // at most one axis step per step. A reflex can then never
                // be outrun by a habit, and summed effort cannot overshoot
                // a target by more than the reflex can correct.
                self.eyes[eye.index()].gaze.x = add_bounded(
                    self.eyes[eye.index()].gaze.x,
                    amount.clamp(-EYE_VELOCITY, EYE_VELOCITY),
                    0,
                    BODY_MAX,
                );
            }
            BodyAxis::EyeVertical { eye } => {
                self.eyes[eye.index()].gaze.y = add_bounded(
                    self.eyes[eye.index()].gaze.y,
                    amount.clamp(-EYE_VELOCITY, EYE_VELOCITY),
                    0,
                    BODY_MAX,
                );
            }
            BodyAxis::PalmHorizontal => {
                // Planar transport is rate-limited: however efforts sum,
                // the palm moves at most four palm steps per step, so an
                // insistent reach approaches what it sees instead of
                // teleporting across it.
                self.hand.palm.x = add_bounded(
                    self.hand.palm.x,
                    amount.clamp(-PALM_VELOCITY, PALM_VELOCITY),
                    0,
                    BODY_MAX,
                );
            }
            BodyAxis::PalmVertical => {
                self.hand.palm.y = add_bounded(
                    self.hand.palm.y,
                    amount.clamp(-PALM_VELOCITY, PALM_VELOCITY),
                    0,
                    BODY_MAX,
                );
            }
            BodyAxis::PalmDepth => {
                self.hand.palm.depth = add_bounded(
                    self.hand.palm.depth,
                    amount.clamp(-PALM_DEPTH_VELOCITY, PALM_DEPTH_VELOCITY),
                    0,
                    BODY_MAX,
                );
            }
            BodyAxis::FingerFlexion => {
                self.hand.finger_flexion =
                    add_bounded(self.hand.finger_flexion, amount, 0, BODY_MAX);
            }
        }
    }
}

impl Default for WorkstationState {
    fn default() -> Self {
        Self {
            eyes: [EyeState::default(), EyeState::default()],
            hand: HandState::default(),
            dynamics: [AxisDynamics::default(); AXIS_COUNT],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldSample {
    eyes: [VisualField; 2],
    contacts: [ContactSample; TOUCH_SITES],
}

pub const GLOBAL_VISION_SIDE: usize = 8;
pub const GLOBAL_VISION_FIELDS: usize = GLOBAL_VISION_SIDE * GLOBAL_VISION_SIDE;
pub const GLOBAL_CHANGE_SUBREGIONS: usize = 4;
pub const FOVEAL_VISION_SIDE: usize = 17;
pub const FOVEAL_VISION_FIELDS: usize = FOVEAL_VISION_SIDE * FOVEAL_VISION_SIDE;
pub const CHROMATIC_VISION_SIDE: usize = 9;
pub const CHROMATIC_VISION_FIELDS: usize = CHROMATIC_VISION_SIDE * CHROMATIC_VISION_SIDE;

/// One eye's generic multiresolution visual surface. The fixed global field
/// carries coarse screen context and spatial transients. The gaze-centred
/// fovea carries local detail. Both are ordinary physical readings.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualField {
    global: LightField,
    /// Bit 0 is darkening, bit 1 is brightening, and bit 2 marks a change
    /// first observed in this frame. Both directions may occur within one
    /// pooled subregion during a spatial rearrangement.
    changed: Vec<u8>,
    foveal: LightField,
    foveal_chromatic: ChromaticField,
    world_aligned_global: bool,
}

impl VisualField {
    pub fn new(
        global: LightField,
        changed: Vec<u8>,
        foveal: LightField,
    ) -> Result<Self, WorkstationError> {
        let foveal_chromatic =
            ChromaticField::neutral(CHROMATIC_VISION_SIDE as u16, CHROMATIC_VISION_SIDE as u16)?;
        Self::new_chromatic(global, changed, foveal, foveal_chromatic)
    }

    pub fn new_chromatic(
        global: LightField,
        changed: Vec<u8>,
        foveal: LightField,
        foveal_chromatic: ChromaticField,
    ) -> Result<Self, WorkstationError> {
        let field = Self {
            global,
            changed,
            foveal,
            foveal_chromatic,
            world_aligned_global: true,
        };
        field.validate()?;
        Ok(field)
    }

    pub const fn global(&self) -> &LightField {
        &self.global
    }

    pub const fn foveal(&self) -> &LightField {
        &self.foveal
    }

    pub const fn foveal_chromatic(&self) -> &ChromaticField {
        &self.foveal_chromatic
    }

    pub(crate) const fn has_world_aligned_global(&self) -> bool {
        self.world_aligned_global
    }

    /// Compatibility view for the original coarse retinal morphology.
    pub fn sample(&self, position: Point) -> u8 {
        self.global.sample(position)
    }

    pub fn changed(&self, field: usize, subregion: usize) -> bool {
        field < GLOBAL_VISION_FIELDS
            && subregion < GLOBAL_CHANGE_SUBREGIONS
            && self.changed[field * GLOBAL_CHANGE_SUBREGIONS + subregion] != 0
    }

    pub(crate) fn brightened(&self, field: usize, subregion: usize) -> bool {
        field < GLOBAL_VISION_FIELDS
            && subregion < GLOBAL_CHANGE_SUBREGIONS
            && self.changed[field * GLOBAL_CHANGE_SUBREGIONS + subregion] & 2 != 0
    }

    pub(crate) fn freshly_brightened(&self, field: usize, subregion: usize) -> bool {
        field < GLOBAL_VISION_FIELDS
            && subregion < GLOBAL_CHANGE_SUBREGIONS
            && self.changed[field * GLOBAL_CHANGE_SUBREGIONS + subregion] & 6 == 6
    }

    pub(crate) fn freshly_changed(&self, field: usize, subregion: usize) -> bool {
        field < GLOBAL_VISION_FIELDS
            && subregion < GLOBAL_CHANGE_SUBREGIONS
            && self.changed[field * GLOBAL_CHANGE_SUBREGIONS + subregion] & 4 != 0
    }

    pub(crate) fn change_impulse(&self, field: usize, subregion: usize) -> i32 {
        if self.brightened(field, subregion) {
            1
        } else if self.changed(field, subregion) {
            -1
        } else {
            0
        }
    }

    pub fn changed_values(&self) -> &[u8] {
        &self.changed
    }

    fn validate(&self) -> Result<(), WorkstationError> {
        self.global.validate()?;
        self.foveal.validate()?;
        self.foveal_chromatic.validate()?;
        if usize::from(self.global.width()) != GLOBAL_VISION_SIDE
            || usize::from(self.global.height()) != GLOBAL_VISION_SIDE
            || usize::from(self.foveal.width()) != FOVEAL_VISION_SIDE
            || usize::from(self.foveal.height()) != FOVEAL_VISION_SIDE
            || usize::from(self.foveal_chromatic.width()) != CHROMATIC_VISION_SIDE
            || usize::from(self.foveal_chromatic.height()) != CHROMATIC_VISION_SIDE
            || self.changed.len() != GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS
            || self
                .changed
                .iter()
                .any(|value| *value > 7 || (*value & 4 != 0 && *value & 3 == 0))
        {
            return Err(WorkstationError::InvalidState);
        }
        Ok(())
    }
}

impl WorldSample {
    pub fn new(
        eyes: [LightField; 2],
        contacts: [ContactSample; TOUCH_SITES],
    ) -> Result<Self, WorkstationError> {
        let [left, right] = eyes;
        let eyes = [legacy_visual_field(left)?, legacy_visual_field(right)?];
        Self::new_visual(eyes, contacts)
    }

    pub fn new_visual(
        eyes: [VisualField; 2],
        contacts: [ContactSample; TOUCH_SITES],
    ) -> Result<Self, WorkstationError> {
        let sample = Self { eyes, contacts };
        sample.validate()?;
        Ok(sample)
    }

    pub fn eye(&self, eye: Eye) -> &VisualField {
        &self.eyes[eye.index()]
    }

    pub const fn contacts(&self) -> &[ContactSample; TOUCH_SITES] {
        &self.contacts
    }

    pub(crate) fn validate(&self) -> Result<(), WorkstationError> {
        for eye in &self.eyes {
            eye.validate()?;
        }
        for contact in &self.contacts {
            contact.validate()?;
        }
        Ok(())
    }
}

fn legacy_visual_field(field: LightField) -> Result<VisualField, WorkstationError> {
    let resample = |side: usize| {
        let mut pixels = Vec::with_capacity(side * side);
        for row in 0..side {
            for column in 0..side {
                let coordinate = |index: usize| {
                    i16::try_from(index * BODY_MAX as usize / (side - 1))
                        .expect("visual sample coordinate is bounded")
                };
                let point = Point::new(coordinate(column), coordinate(row))
                    .expect("visual sample coordinate is bounded");
                pixels.push(field.sample(point));
            }
        }
        LightField::new(side as u16, side as u16, pixels)
    };
    let mut visual = VisualField::new(
        resample(GLOBAL_VISION_SIDE)?,
        vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
        resample(FOVEAL_VISION_SIDE)?,
    )?;
    visual.world_aligned_global = false;
    Ok(visual)
}

const fn axis_step(axis: BodyAxis) -> i32 {
    match axis {
        // Half-pitch granularity: a vergence correction can land within
        // half a receptor pitch, so fusion settles inside the foveal
        // tolerance instead of hunting across it a full pitch at a time.
        BodyAxis::EyeHorizontal { .. } | BodyAxis::EyeVertical { .. } => 32,
        BodyAxis::PalmHorizontal | BodyAxis::PalmVertical => 8,
        BodyAxis::PalmDepth => 16,
        BodyAxis::FingerFlexion => 64,
    }
}

/// The palm's planar speed bound in world units per step.
const PALM_VELOCITY: i32 = 64;

/// Surface-normal arm speed is one depth quantum per body step. This bounds
/// discrete penetration so the finger can always withdraw from a new surface.
const PALM_DEPTH_VELOCITY: i32 = 16;

/// The eye's speed bound in world units per step: one receptor pitch.
/// Summed effort cannot exceed it, so no combination of reflex and habit
/// can overshoot a target by more than one pitch, and the eye can never
/// move fast enough to leave a seen target's grasp.
const EYE_VELOCITY: i32 = 128;

fn add_bounded(value: i16, amount: i32, min: i16, max: i16) -> i16 {
    let next = i32::from(value).saturating_add(amount);
    i16::try_from(next.clamp(i32::from(min), i32::from(max))).unwrap_or(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_is_luminance_identity_and_chromatically_neutral() {
        for value in [0, 1, 127, 255] {
            let gray = Rgb::gray(value);
            assert_eq!(gray.luminance(), value);
            assert_eq!(gray.opponents(), ChromaticSignal::default());
        }
    }

    #[test]
    fn equal_luminance_colours_have_distinct_opponent_responses() {
        let red = Rgb::new(255, 0, 0);
        let green = Rgb::new(0, 75, 0);
        assert_eq!(red.luminance(), green.luminance());
        assert_ne!(red.opponents(), green.opponents());
    }

    #[test]
    fn body_control_is_one_axis_direction_product() {
        let controls = BodyAxis::ALL.map(|axis| BodyControl::new(axis, Direction::Increase));
        for control in controls {
            let encoded = serde_json::to_string(&control).unwrap();
            assert_eq!(
                serde_json::from_str::<BodyControl>(&encoded).unwrap(),
                control
            );
            assert_eq!(control.direction(), Direction::Increase);
        }
        assert_eq!(
            serde_json::to_string(&BodyControl::new(
                BodyAxis::EyeHorizontal { eye: Eye::Left },
                Direction::Decrease,
            ))
            .unwrap(),
            r#"{"axis":{"eye_horizontal":{"eye":"left"}},"direction":"decrease"}"#
        );
    }

    #[test]
    fn equal_opposing_effort_is_visible_without_movement() {
        let mut state = WorkstationState::default();
        let before = state.clone();
        let axis = BodyAxis::PalmHorizontal;
        let mut frame = ActuatorFrame::default();
        frame.activate(axis, Direction::Decrease, 3);
        frame.activate(axis, Direction::Increase, 3);

        let movement = state.integrate(frame);
        let movement = movement.iter().find(|value| value.axis == axis).unwrap();
        let sense = state.proprioception()[axis.index()];

        assert!(!movement.changed);
        assert_eq!(movement.velocity, 0);
        assert!(state.same_pose(&before));
        assert_eq!(sense.decrease_effort, 3);
        assert_eq!(sense.increase_effort, 3);
    }

    #[test]
    fn surface_resistance_cancels_inward_effort_without_causing_withdrawal() {
        let axis = BodyAxis::PalmDepth;
        let mut state = WorkstationState::default();
        let before = state.clone();
        let mut frame = ActuatorFrame::default();
        frame.activate(axis, Direction::Increase, 7);
        frame.resist_increase(axis, BODY_MAX as u16);

        let movement = state.integrate(frame);

        assert_eq!(movement.len(), 1);
        assert_eq!(movement[0].axis, axis);
        assert_eq!(movement[0].decrease_effort, 7);
        assert_eq!(movement[0].increase_effort, 7);
        assert_eq!(movement[0].net_impulse, 0);
        assert!(!movement[0].changed);
        assert!(state.same_pose(&before));
    }

    #[test]
    fn independent_eye_and_hand_axes_commute() {
        let eye = BodyAxis::EyeHorizontal { eye: Eye::Left };
        let planar = BodyAxis::PalmVertical;
        let mut forward = ActuatorFrame::default();
        forward.activate(eye, Direction::Increase, 2);
        forward.activate(planar, Direction::Decrease, 1);
        let mut reverse = ActuatorFrame::default();
        reverse.activate(planar, Direction::Decrease, 1);
        reverse.activate(eye, Direction::Increase, 2);

        let mut first = WorkstationState::default();
        let mut second = WorkstationState::default();
        assert_eq!(first.integrate(forward), second.integrate(reverse));
        assert_eq!(first, second);
        assert_ne!(first.eye(Eye::Left), second.eye(Eye::Right));
    }

    #[test]
    fn one_axis_changes_only_its_owned_state() {
        let initial = WorkstationState::default();
        let mut state = initial.clone();
        let mut frame = ActuatorFrame::default();
        frame.activate(
            BodyAxis::EyeVertical { eye: Eye::Right },
            Direction::Increase,
            1,
        );
        state.integrate(frame);
        assert_eq!(state.eye(Eye::Left), initial.eye(Eye::Left));
        assert_ne!(state.eye(Eye::Right), initial.eye(Eye::Right));
        assert_eq!(state.hand(), initial.hand());
    }

    #[test]
    fn finger_contact_and_arm_pose_are_independent() {
        let initial = WorkstationState::default();
        let palm = initial.hand().palm();
        let tip = initial.hand().fingertip();
        let mut finger = initial.clone();
        let mut frame = ActuatorFrame::default();
        frame.activate(BodyAxis::FingerFlexion, Direction::Increase, 1);
        finger.integrate(frame);

        assert_eq!(finger.hand().palm(), palm);
        assert_eq!(finger.hand().fingertip().x(), tip.x());
        assert_eq!(finger.hand().fingertip().y(), tip.y());
        assert!(finger.hand().fingertip().depth() > tip.depth());

        let mut moved = finger.clone();
        let held_depth = moved.hand().fingertip().depth();
        let mut frame = ActuatorFrame::default();
        frame.activate(BodyAxis::PalmHorizontal, Direction::Increase, 1);
        moved.integrate(frame);

        assert_ne!(moved.hand().fingertip().x(), finger.hand().fingertip().x());
        assert_eq!(moved.hand().fingertip().depth(), held_depth);
        assert_eq!(
            moved.hand().finger_flexion(),
            finger.hand().finger_flexion()
        );
    }

    #[test]
    fn one_eye_impulse_uses_half_pitch_and_is_velocity_capped() {
        // One impulse moves half an original receptor pitch. Summed effort
        // retains the existing two-pitch velocity cap.
        let mut state = WorkstationState::default();
        let before = state.eye(Eye::Left).gaze().x();
        let mut frame = ActuatorFrame::default();
        frame.activate(
            BodyAxis::EyeHorizontal { eye: Eye::Left },
            Direction::Increase,
            1,
        );

        let movement = state.integrate(frame);

        assert_eq!(movement[0].velocity, 32);
        assert_eq!(state.eye(Eye::Left).gaze().x() - before, 32);
        // A larger push remains capped at the existing velocity.
        let mut state = WorkstationState::default();
        let mut frame = ActuatorFrame::default();
        frame.activate(
            BodyAxis::EyeHorizontal { eye: Eye::Left },
            Direction::Increase,
            4,
        );
        let movement = state.integrate(frame);
        assert_eq!(movement[0].velocity, 128);
    }

    #[test]
    fn the_pointer_has_one_bounded_three_dimensional_position() {
        let state = WorkstationState::default();
        let palm = state.hand().palm();
        assert!((0..=BODY_MAX).contains(&palm.x()));
        assert!((0..=BODY_MAX).contains(&palm.y()));
        assert!((0..=BODY_MAX).contains(&palm.depth()));
    }

    #[test]
    fn bounded_motion_reports_actual_delta_and_limit() {
        let axis = BodyAxis::PalmDepth;
        let mut state = WorkstationState::default();
        let mut frame = ActuatorFrame::default();
        frame.activate(axis, Direction::Increase, BODY_MAX as u16);
        let movement = state.integrate(frame);

        assert_eq!(movement[0].velocity, PALM_DEPTH_VELOCITY as i16);
        assert!(!state.proprioception()[axis.index()].at_upper_limit);

        for _ in 0..BODY_MAX / PALM_DEPTH_VELOCITY as i16 {
            let mut frame = ActuatorFrame::default();
            frame.activate(axis, Direction::Increase, BODY_MAX as u16);
            state.integrate(frame);
        }
        assert!(state.proprioception()[axis.index()].at_upper_limit);
    }
}
