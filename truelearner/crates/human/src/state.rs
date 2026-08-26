use crate::HumanError;
use serde::{Deserialize, Serialize};

pub const BODY_MAX: i16 = 1_023;
pub const DIGIT_COUNT: usize = 5;
pub const TOUCH_SITES: usize = DIGIT_COUNT + 1;
pub const AXIS_COUNT: usize = 25;
const MAX_PIXELS: usize = 1_048_576;
const MID: i16 = (BODY_MAX + 1) / 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Left => 0,
            Self::Right => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Digit {
    Thumb,
    Index,
    Middle,
    Ring,
    Little,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Decrease,
    Increase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "control", rename_all = "snake_case")]
pub enum BodyControl {
    GazeHorizontal {
        direction: Direction,
    },
    GazeVertical {
        direction: Direction,
    },
    Vergence {
        direction: Direction,
    },
    PalmHorizontal {
        side: Side,
        direction: Direction,
    },
    PalmVertical {
        side: Side,
        direction: Direction,
    },
    Wrist {
        side: Side,
        direction: Direction,
    },
    ContactForce {
        side: Side,
        direction: Direction,
    },
    Spread {
        side: Side,
        direction: Direction,
    },
    ThumbOpposition {
        side: Side,
        direction: Direction,
    },
    FingerFlexion {
        side: Side,
        digit: Digit,
        direction: Direction,
    },
}

impl BodyControl {
    pub const fn axis(self) -> BodyAxis {
        match self {
            Self::GazeHorizontal { .. } => BodyAxis::GazeHorizontal,
            Self::GazeVertical { .. } => BodyAxis::GazeVertical,
            Self::Vergence { .. } => BodyAxis::Vergence,
            Self::PalmHorizontal { side, .. } => BodyAxis::PalmHorizontal { side },
            Self::PalmVertical { side, .. } => BodyAxis::PalmVertical { side },
            Self::Wrist { side, .. } => BodyAxis::Wrist { side },
            Self::ContactForce { side, .. } => BodyAxis::ContactForce { side },
            Self::Spread { side, .. } => BodyAxis::Spread { side },
            Self::ThumbOpposition { side, .. } => BodyAxis::ThumbOpposition { side },
            Self::FingerFlexion { side, digit, .. } => BodyAxis::FingerFlexion { side, digit },
        }
    }

    pub const fn direction(self) -> Direction {
        match self {
            Self::GazeHorizontal { direction }
            | Self::GazeVertical { direction }
            | Self::Vergence { direction }
            | Self::PalmHorizontal { direction, .. }
            | Self::PalmVertical { direction, .. }
            | Self::Wrist { direction, .. }
            | Self::ContactForce { direction, .. }
            | Self::Spread { direction, .. }
            | Self::ThumbOpposition { direction, .. }
            | Self::FingerFlexion { direction, .. } => direction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "axis", rename_all = "snake_case")]
pub enum BodyAxis {
    GazeHorizontal,
    GazeVertical,
    Vergence,
    PalmHorizontal { side: Side },
    PalmVertical { side: Side },
    Wrist { side: Side },
    ContactForce { side: Side },
    Spread { side: Side },
    ThumbOpposition { side: Side },
    FingerFlexion { side: Side, digit: Digit },
}

impl BodyAxis {
    pub const ALL: [Self; AXIS_COUNT] = [
        Self::GazeHorizontal,
        Self::GazeVertical,
        Self::Vergence,
        Self::PalmHorizontal { side: Side::Left },
        Self::PalmVertical { side: Side::Left },
        Self::Wrist { side: Side::Left },
        Self::ContactForce { side: Side::Left },
        Self::Spread { side: Side::Left },
        Self::ThumbOpposition { side: Side::Left },
        Self::FingerFlexion {
            side: Side::Left,
            digit: Digit::Thumb,
        },
        Self::FingerFlexion {
            side: Side::Left,
            digit: Digit::Index,
        },
        Self::FingerFlexion {
            side: Side::Left,
            digit: Digit::Middle,
        },
        Self::FingerFlexion {
            side: Side::Left,
            digit: Digit::Ring,
        },
        Self::FingerFlexion {
            side: Side::Left,
            digit: Digit::Little,
        },
        Self::PalmHorizontal { side: Side::Right },
        Self::PalmVertical { side: Side::Right },
        Self::Wrist { side: Side::Right },
        Self::ContactForce { side: Side::Right },
        Self::Spread { side: Side::Right },
        Self::ThumbOpposition { side: Side::Right },
        Self::FingerFlexion {
            side: Side::Right,
            digit: Digit::Thumb,
        },
        Self::FingerFlexion {
            side: Side::Right,
            digit: Digit::Index,
        },
        Self::FingerFlexion {
            side: Side::Right,
            digit: Digit::Middle,
        },
        Self::FingerFlexion {
            side: Side::Right,
            digit: Digit::Ring,
        },
        Self::FingerFlexion {
            side: Side::Right,
            digit: Digit::Little,
        },
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::GazeHorizontal => 0,
            Self::GazeVertical => 1,
            Self::Vergence => 2,
            Self::PalmHorizontal { side } => 3 + side.index() * 11,
            Self::PalmVertical { side } => 4 + side.index() * 11,
            Self::Wrist { side } => 5 + side.index() * 11,
            Self::ContactForce { side } => 6 + side.index() * 11,
            Self::Spread { side } => 7 + side.index() * 11,
            Self::ThumbOpposition { side } => 8 + side.index() * 11,
            Self::FingerFlexion { side, digit } => 9 + side.index() * 11 + digit.index(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AxisEffort {
    decrease: u16,
    increase: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ActuatorFrame {
    axes: [AxisEffort; AXIS_COUNT],
}

impl Default for ActuatorFrame {
    fn default() -> Self {
        Self {
            axes: [AxisEffort::default(); AXIS_COUNT],
        }
    }
}

impl ActuatorFrame {
    pub(crate) fn activate(&mut self, axis: BodyAxis, direction: Direction, impulse: u16) {
        let effort = &mut self.axes[axis.index()];
        let target = match direction {
            Direction::Decrease => &mut effort.decrease,
            Direction::Increase => &mut effort.increase,
        };
        *target = target.saturating_add(impulse).min(BODY_MAX as u16);
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

impl Digit {
    pub const ALL: [Self; DIGIT_COUNT] = [
        Self::Thumb,
        Self::Index,
        Self::Middle,
        Self::Ring,
        Self::Little,
    ];

    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Thumb => 0,
            Self::Index => 1,
            Self::Middle => 2,
            Self::Ring => 3,
            Self::Little => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    x: i16,
    y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Result<Self, HumanError> {
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

    pub(crate) fn offset(self, dx: i16, dy: i16) -> Self {
        Self {
            x: self.x.saturating_add(dx).clamp(0, BODY_MAX),
            y: self.y.saturating_add(dy).clamp(0, BODY_MAX),
        }
    }

    pub(crate) fn validate(self) -> Result<(), HumanError> {
        if (0..=BODY_MAX).contains(&self.x) && (0..=BODY_MAX).contains(&self.y) {
            Ok(())
        } else {
            Err(HumanError::InvalidState)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightField {
    width: u16,
    height: u16,
    pixels: Vec<u8>,
}

impl LightField {
    pub fn new(width: u16, height: u16, pixels: Vec<u8>) -> Result<Self, HumanError> {
        let field = Self {
            width,
            height,
            pixels,
        };
        field.validate()?;
        Ok(field)
    }

    pub fn filled(width: u16, height: u16, value: u8) -> Result<Self, HumanError> {
        let count = usize::from(width)
            .checked_mul(usize::from(height))
            .ok_or(HumanError::LightFieldTooLarge)?;
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

    pub(crate) fn validate(&self) -> Result<(), HumanError> {
        if self.width == 0 || self.height == 0 {
            return Err(HumanError::EmptyLightField);
        }
        let expected = usize::from(self.width)
            .checked_mul(usize::from(self.height))
            .ok_or(HumanError::LightFieldTooLarge)?;
        if expected > MAX_PIXELS {
            return Err(HumanError::LightFieldTooLarge);
        }
        if self.pixels.len() != expected {
            return Err(HumanError::LightLength);
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
    pub fn new(pressure: u16, slip: i16) -> Result<Self, HumanError> {
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

    pub(crate) fn validate(self) -> Result<(), HumanError> {
        if pressure_in_range(self.pressure) && (-BODY_MAX..=BODY_MAX).contains(&self.slip) {
            Ok(())
        } else {
            Err(HumanError::ContactOutsideRange)
        }
    }
}

const fn pressure_in_range(value: u16) -> bool {
    value <= BODY_MAX as u16
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigitState {
    flexion: u16,
}

impl DigitState {
    pub const fn flexion(self) -> u16 {
        self.flexion
    }
}

impl Default for DigitState {
    fn default() -> Self {
        Self { flexion: 512 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EyeState {
    gaze: Point,
    vergence: i16,
}

impl EyeState {
    pub const fn gaze(&self) -> Point {
        self.gaze
    }

    pub const fn vergence(&self) -> i16 {
        self.vergence
    }

    pub fn focus(&self, side: Side) -> Point {
        let direction = if side == Side::Left { -1 } else { 1 };
        self.gaze.offset(self.vergence.saturating_mul(direction), 0)
    }
}

impl Default for EyeState {
    fn default() -> Self {
        Self {
            gaze: Point { x: MID, y: MID },
            vergence: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandState {
    palm: Point,
    wrist: i16,
    force: u16,
    spread: i16,
    thumb_opposition: i16,
    digits: [DigitState; DIGIT_COUNT],
}

impl HandState {
    pub const fn palm(&self) -> Point {
        self.palm
    }

    pub const fn wrist(&self) -> i16 {
        self.wrist
    }

    pub const fn force(&self) -> u16 {
        self.force
    }

    pub const fn spread(&self) -> i16 {
        self.spread
    }

    pub const fn thumb_opposition(&self) -> i16 {
        self.thumb_opposition
    }

    pub fn digit(&self, digit: Digit) -> DigitState {
        self.digits[digit.index()]
    }

    pub fn fingertip(&self, digit: Digit) -> Point {
        let index = i32::try_from(digit.index()).unwrap_or(0) - 2;
        let spread = i32::from(self.spread) / 24;
        let thumb = if digit == Digit::Thumb {
            i32::from(self.thumb_opposition) / 16
        } else {
            0
        };
        let flexion = i32::from(self.digits[digit.index()].flexion);
        let reach = 120_i32.saturating_sub(flexion / 8);
        let x = i32::from(self.palm.x)
            .saturating_add(index.saturating_mul(30 + spread))
            .saturating_add(thumb);
        let y = i32::from(self.palm.y).saturating_sub(reach);
        Point {
            x: i16::try_from(x.clamp(0, i32::from(BODY_MAX))).unwrap_or(BODY_MAX),
            y: i16::try_from(y.clamp(0, i32::from(BODY_MAX))).unwrap_or(BODY_MAX),
        }
    }

    pub(crate) fn default_for(side: Side) -> Self {
        Self {
            palm: Point {
                x: if side == Side::Left { 360 } else { 664 },
                y: 768,
            },
            wrist: 0,
            force: 0,
            spread: 0,
            thumb_opposition: 0,
            digits: [DigitState::default(); DIGIT_COUNT],
        }
    }

    pub(crate) fn validate(&self) -> Result<(), HumanError> {
        self.palm.validate()?;
        if !(-BODY_MAX..=BODY_MAX).contains(&self.wrist)
            || self.force > BODY_MAX as u16
            || !(-BODY_MAX..=BODY_MAX).contains(&self.spread)
            || !(-BODY_MAX..=BODY_MAX).contains(&self.thumb_opposition)
            || self
                .digits
                .iter()
                .any(|digit| digit.flexion > BODY_MAX as u16)
        {
            return Err(HumanError::InvalidState);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HumanState {
    eyes: EyeState,
    hands: [HandState; 2],
    dynamics: [AxisDynamics; AXIS_COUNT],
}

impl HumanState {
    pub const fn eyes(&self) -> &EyeState {
        &self.eyes
    }

    pub fn hand(&self, side: Side) -> &HandState {
        &self.hands[side.index()]
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

    pub(crate) fn hand_mut(&mut self, side: Side) -> &mut HandState {
        &mut self.hands[side.index()]
    }

    pub(crate) fn validate(&self) -> Result<(), HumanError> {
        self.eyes.gaze.validate()?;
        if !(-BODY_MAX..=BODY_MAX).contains(&self.eyes.vergence) {
            return Err(HumanError::InvalidState);
        }
        for hand in &self.hands {
            hand.validate()?;
        }
        for (index, dynamics) in self.dynamics.iter().enumerate() {
            let axis = BodyAxis::ALL[index];
            if dynamics.decrease_effort > BODY_MAX as u16
                || dynamics.increase_effort > BODY_MAX as u16
                || dynamics.velocity.unsigned_abs() > axis_velocity_bound(axis)
            {
                return Err(HumanError::InvalidState);
            }
        }
        Ok(())
    }

    pub(crate) fn integrate(&mut self, frame: ActuatorFrame) -> Vec<BodyMovement> {
        self.dynamics.fill(AxisDynamics::default());
        let mut movements = Vec::new();
        for (index, effort) in frame.axes.into_iter().enumerate() {
            if effort == AxisEffort::default() {
                continue;
            }
            let axis = BodyAxis::ALL[index];
            let before = self.axis_position(axis);
            let net_impulse = i32::from(effort.increase) - i32::from(effort.decrease);
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
        self.eyes == other.eyes && self.hands == other.hands
    }

    #[cfg(test)]
    fn pose(&self) -> (&EyeState, &[HandState; 2]) {
        (&self.eyes, &self.hands)
    }

    fn axis_position(&self, axis: BodyAxis) -> i16 {
        match axis {
            BodyAxis::GazeHorizontal => self.eyes.gaze.x - MID,
            BodyAxis::GazeVertical => self.eyes.gaze.y - MID,
            BodyAxis::Vergence => self.eyes.vergence,
            BodyAxis::PalmHorizontal { side } => {
                self.hand(side).palm.x - HandState::default_for(side).palm.x
            }
            BodyAxis::PalmVertical { side } => {
                self.hand(side).palm.y - HandState::default_for(side).palm.y
            }
            BodyAxis::Wrist { side } => self.hand(side).wrist,
            BodyAxis::ContactForce { side } => {
                i16::try_from(self.hand(side).force).unwrap_or(BODY_MAX)
            }
            BodyAxis::Spread { side } => self.hand(side).spread,
            BodyAxis::ThumbOpposition { side } => self.hand(side).thumb_opposition,
            BodyAxis::FingerFlexion { side, digit } => {
                i16::try_from(self.hand(side).digit(digit).flexion).unwrap_or(BODY_MAX) - MID
            }
        }
    }

    fn limits(&self, axis: BodyAxis) -> (bool, bool) {
        match axis {
            BodyAxis::GazeHorizontal => (self.eyes.gaze.x == 0, self.eyes.gaze.x == BODY_MAX),
            BodyAxis::GazeVertical => (self.eyes.gaze.y == 0, self.eyes.gaze.y == BODY_MAX),
            BodyAxis::Vergence => (self.eyes.vergence == -128, self.eyes.vergence == 128),
            BodyAxis::PalmHorizontal { side } => (
                self.hand(side).palm.x == 0,
                self.hand(side).palm.x == BODY_MAX,
            ),
            BodyAxis::PalmVertical { side } => (
                self.hand(side).palm.y == 0,
                self.hand(side).palm.y == BODY_MAX,
            ),
            BodyAxis::Wrist { side } => (
                self.hand(side).wrist == -BODY_MAX,
                self.hand(side).wrist == BODY_MAX,
            ),
            BodyAxis::ContactForce { side } => (
                self.hand(side).force == 0,
                self.hand(side).force == BODY_MAX as u16,
            ),
            BodyAxis::Spread { side } => (
                self.hand(side).spread == -BODY_MAX,
                self.hand(side).spread == BODY_MAX,
            ),
            BodyAxis::ThumbOpposition { side } => (
                self.hand(side).thumb_opposition == -BODY_MAX,
                self.hand(side).thumb_opposition == BODY_MAX,
            ),
            BodyAxis::FingerFlexion { side, digit } => {
                let flexion = self.hand(side).digit(digit).flexion;
                (flexion == 0, flexion == BODY_MAX as u16)
            }
        }
    }

    fn apply_net(&mut self, axis: BodyAxis, net_impulse: i32) {
        let signed_amount = net_impulse.saturating_mul(axis_step(axis));
        match axis {
            BodyAxis::GazeHorizontal => {
                self.eyes.gaze.x = add_bounded(self.eyes.gaze.x, signed_amount, 0, BODY_MAX);
            }
            BodyAxis::GazeVertical => {
                self.eyes.gaze.y = add_bounded(self.eyes.gaze.y, signed_amount, 0, BODY_MAX);
            }
            BodyAxis::Vergence => {
                self.eyes.vergence = add_bounded(self.eyes.vergence, signed_amount, -128, 128);
            }
            BodyAxis::PalmHorizontal { side } => {
                let hand = self.hand_mut(side);
                hand.palm.x = add_bounded(hand.palm.x, signed_amount, 0, BODY_MAX);
            }
            BodyAxis::PalmVertical { side } => {
                let hand = self.hand_mut(side);
                hand.palm.y = add_bounded(hand.palm.y, signed_amount, 0, BODY_MAX);
            }
            BodyAxis::Wrist { side } => {
                let hand = self.hand_mut(side);
                hand.wrist = add_bounded(hand.wrist, signed_amount, -BODY_MAX, BODY_MAX);
            }
            BodyAxis::ContactForce { side } => {
                let hand = self.hand_mut(side);
                hand.force = add_bounded_u16(hand.force, signed_amount);
            }
            BodyAxis::Spread { side } => {
                let hand = self.hand_mut(side);
                hand.spread = add_bounded(hand.spread, signed_amount, -BODY_MAX, BODY_MAX);
            }
            BodyAxis::ThumbOpposition { side } => {
                let hand = self.hand_mut(side);
                hand.thumb_opposition =
                    add_bounded(hand.thumb_opposition, signed_amount, -BODY_MAX, BODY_MAX);
            }
            BodyAxis::FingerFlexion { side, digit } => {
                let hand = self.hand_mut(side);
                let state = &mut hand.digits[digit.index()];
                state.flexion = add_bounded_u16(state.flexion, signed_amount);
            }
        }
    }
}

const fn axis_step(axis: BodyAxis) -> i32 {
    match axis {
        BodyAxis::GazeHorizontal
        | BodyAxis::GazeVertical
        | BodyAxis::PalmHorizontal { .. }
        | BodyAxis::PalmVertical { .. } => 16,
        BodyAxis::Vergence => 8,
        BodyAxis::Wrist { .. } | BodyAxis::Spread { .. } | BodyAxis::ThumbOpposition { .. } => 32,
        BodyAxis::ContactForce { .. } | BodyAxis::FingerFlexion { .. } => 64,
    }
}

const fn axis_velocity_bound(axis: BodyAxis) -> u16 {
    match axis {
        BodyAxis::Vergence => 256,
        BodyAxis::Wrist { .. } | BodyAxis::Spread { .. } | BodyAxis::ThumbOpposition { .. } => {
            BODY_MAX as u16 * 2
        }
        BodyAxis::GazeHorizontal
        | BodyAxis::GazeVertical
        | BodyAxis::PalmHorizontal { .. }
        | BodyAxis::PalmVertical { .. }
        | BodyAxis::ContactForce { .. }
        | BodyAxis::FingerFlexion { .. } => BODY_MAX as u16,
    }
}

fn add_bounded(value: i16, signed_amount: i32, min: i16, max: i16) -> i16 {
    let next = i32::from(value).saturating_add(signed_amount);
    i16::try_from(next.clamp(i32::from(min), i32::from(max))).unwrap_or(value)
}

fn add_bounded_u16(value: u16, signed_amount: i32) -> u16 {
    let next = i32::from(value).saturating_add(signed_amount);
    u16::try_from(next.clamp(0, i32::from(BODY_MAX))).unwrap_or(value)
}

impl Default for HumanState {
    fn default() -> Self {
        Self {
            eyes: EyeState::default(),
            hands: [
                HandState::default_for(Side::Left),
                HandState::default_for(Side::Right),
            ],
            dynamics: [AxisDynamics::default(); AXIS_COUNT],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldSample {
    eyes: [LightField; 2],
    contacts: [[ContactSample; TOUCH_SITES]; 2],
}

impl WorldSample {
    pub fn new(
        eyes: [LightField; 2],
        contacts: [[ContactSample; TOUCH_SITES]; 2],
    ) -> Result<Self, HumanError> {
        let sample = Self { eyes, contacts };
        sample.validate()?;
        Ok(sample)
    }

    pub fn eye(&self, side: Side) -> &LightField {
        &self.eyes[side.index()]
    }

    pub fn contacts(&self, side: Side) -> &[ContactSample; TOUCH_SITES] {
        &self.contacts[side.index()]
    }

    pub(crate) fn validate(&self) -> Result<(), HumanError> {
        for eye in &self.eyes {
            eye.validate()?;
        }
        for hand in &self.contacts {
            for contact in hand {
                contact.validate()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod proprioception_tests {
    use super::*;

    #[test]
    fn equal_opposing_effort_is_visible_without_movement() {
        let mut state = HumanState::default();
        let before = state.clone();
        let axis = BodyAxis::FingerFlexion {
            side: Side::Left,
            digit: Digit::Index,
        };
        let mut frame = ActuatorFrame::default();
        frame.activate(axis, Direction::Decrease, 3);
        frame.activate(axis, Direction::Increase, 3);

        let movements = state.integrate(frame);
        let movement = movements.iter().find(|value| value.axis == axis).unwrap();
        let sense = state.proprioception()[axis.index()];

        assert!(!movement.changed);
        assert_eq!(movement.velocity, 0);
        assert_eq!(state.pose(), before.pose());
        assert_eq!(sense.velocity, 0);
        assert_eq!(sense.decrease_effort, 3);
        assert_eq!(sense.increase_effort, 3);
    }

    #[test]
    fn actuator_folding_is_order_invariant_and_reports_actual_velocity() {
        let axis = BodyAxis::GazeHorizontal;
        let mut forward = ActuatorFrame::default();
        forward.activate(axis, Direction::Increase, 3);
        forward.activate(axis, Direction::Decrease, 1);
        let mut reverse = ActuatorFrame::default();
        reverse.activate(axis, Direction::Decrease, 1);
        reverse.activate(axis, Direction::Increase, 3);

        let mut first = HumanState::default();
        let mut second = HumanState::default();
        let first_movement = first.integrate(forward);
        let second_movement = second.integrate(reverse);

        assert_eq!(first, second);
        assert_eq!(first_movement, second_movement);
        assert_eq!(first_movement[0].velocity, 32);
        assert_eq!(first.proprioception()[axis.index()].position, 32);
    }

    #[test]
    fn bounded_motion_reports_actual_delta_and_limit_contact() {
        let axis = BodyAxis::GazeHorizontal;
        let mut state = HumanState::default();
        let mut frame = ActuatorFrame::default();
        frame.activate(axis, Direction::Increase, BODY_MAX as u16);
        let movement = state.integrate(frame);
        let sense = state.proprioception()[axis.index()];
        assert_eq!(movement[0].velocity, BODY_MAX - MID);
        assert_eq!(sense.position, BODY_MAX - MID);
        assert!(sense.at_upper_limit);

        let mut blocked = ActuatorFrame::default();
        blocked.activate(axis, Direction::Increase, BODY_MAX as u16);
        let movement = state.integrate(blocked);
        assert_eq!(movement[0].velocity, 0);
        assert!(!movement[0].changed);
        assert!(state.proprioception()[axis.index()].at_upper_limit);
    }

    #[test]
    fn invalid_dynamics_fail_state_validation() {
        let mut state = HumanState::default();
        state.dynamics[0].increase_effort = BODY_MAX as u16 + 1;
        assert_eq!(state.validate(), Err(HumanError::InvalidState));
    }
}
