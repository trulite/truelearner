use crate::WorkstationError;
use serde::{Deserialize, Serialize};
use truelearner_embodiment::OpposedEffort;

pub const BODY_MAX: i16 = 1_023;
pub const DIGIT_COUNT: usize = 5;
pub const TOUCH_SITES: usize = DIGIT_COUNT + 1;
pub const AXIS_COUNT: usize = 15;
const MAX_PIXELS: usize = 1_048_576;
const MID: i16 = (BODY_MAX + 1) / 2;

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
pub enum Digit {
    Thumb,
    Index,
    Middle,
    Ring,
    Little,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Decrease,
    Increase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "control", rename_all = "snake_case")]
pub enum BodyControl {
    EyeHorizontal { eye: Eye, direction: Direction },
    EyeVertical { eye: Eye, direction: Direction },
    PalmHorizontal { direction: Direction },
    PalmVertical { direction: Direction },
    PalmDepth { direction: Direction },
    Wrist { direction: Direction },
    Spread { direction: Direction },
    ThumbOpposition { direction: Direction },
    FingerFlexion { digit: Digit, direction: Direction },
}

impl BodyControl {
    pub const fn axis(self) -> BodyAxis {
        match self {
            Self::EyeHorizontal { eye, .. } => BodyAxis::EyeHorizontal { eye },
            Self::EyeVertical { eye, .. } => BodyAxis::EyeVertical { eye },
            Self::PalmHorizontal { .. } => BodyAxis::PalmHorizontal,
            Self::PalmVertical { .. } => BodyAxis::PalmVertical,
            Self::PalmDepth { .. } => BodyAxis::PalmDepth,
            Self::Wrist { .. } => BodyAxis::Wrist,
            Self::Spread { .. } => BodyAxis::Spread,
            Self::ThumbOpposition { .. } => BodyAxis::ThumbOpposition,
            Self::FingerFlexion { digit, .. } => BodyAxis::FingerFlexion { digit },
        }
    }

    pub const fn direction(self) -> Direction {
        match self {
            Self::EyeHorizontal { direction, .. }
            | Self::EyeVertical { direction, .. }
            | Self::PalmHorizontal { direction }
            | Self::PalmVertical { direction }
            | Self::PalmDepth { direction }
            | Self::Wrist { direction }
            | Self::Spread { direction }
            | Self::ThumbOpposition { direction }
            | Self::FingerFlexion { direction, .. } => direction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "axis", rename_all = "snake_case")]
pub enum BodyAxis {
    EyeHorizontal { eye: Eye },
    EyeVertical { eye: Eye },
    PalmHorizontal,
    PalmVertical,
    PalmDepth,
    Wrist,
    Spread,
    ThumbOpposition,
    FingerFlexion { digit: Digit },
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
        Self::Wrist,
        Self::Spread,
        Self::ThumbOpposition,
        Self::FingerFlexion {
            digit: Digit::Thumb,
        },
        Self::FingerFlexion {
            digit: Digit::Index,
        },
        Self::FingerFlexion {
            digit: Digit::Middle,
        },
        Self::FingerFlexion { digit: Digit::Ring },
        Self::FingerFlexion {
            digit: Digit::Little,
        },
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::EyeHorizontal { eye } => eye.index() * 2,
            Self::EyeVertical { eye } => eye.index() * 2 + 1,
            Self::PalmHorizontal => 4,
            Self::PalmVertical => 5,
            Self::PalmDepth => 6,
            Self::Wrist => 7,
            Self::Spread => 8,
            Self::ThumbOpposition => 9,
            Self::FingerFlexion { digit } => 10 + digit.index(),
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
    #[cfg(test)]
    pub(crate) fn activate(&mut self, axis: BodyAxis, direction: Direction, impulse: u16) {
        let effort = &mut self.axes[axis.index()];
        let command = match direction {
            Direction::Decrease => OpposedEffort::new(impulse, 0),
            Direction::Increase => OpposedEffort::new(0, impulse),
        };
        *effort = effort.combine_bounded(command, BODY_MAX as u16);
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

    pub(crate) fn offset(self, dx: i16, dy: i16) -> Self {
        Self {
            x: self.x.saturating_add(dx).clamp(0, BODY_MAX),
            y: self.y.saturating_add(dy).clamp(0, BODY_MAX),
        }
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

    fn validate(self) -> Result<(), WorkstationError> {
        if (0..=BODY_MAX).contains(&self.x)
            && (0..=BODY_MAX).contains(&self.y)
            && (0..=BODY_MAX).contains(&self.depth)
        {
            Ok(())
        } else {
            Err(WorkstationError::InvalidState)
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandState {
    palm: HandPoint,
    wrist: i16,
    spread: i16,
    thumb_opposition: i16,
    digits: [DigitState; DIGIT_COUNT],
}

impl HandState {
    pub const fn palm(&self) -> HandPoint {
        self.palm
    }

    pub const fn wrist(&self) -> i16 {
        self.wrist
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

    pub fn fingertip(&self, digit: Digit) -> HandPoint {
        let index = i32::try_from(digit.index()).unwrap_or(0) - 2;
        let spread = i32::from(self.spread) / 24;
        let opposition = if digit == Digit::Thumb {
            i32::from(self.thumb_opposition) / 16
        } else {
            0
        };
        let flexion = i32::from(self.digits[digit.index()].flexion) - i32::from(MID);
        let reach = 56_i32.saturating_sub(flexion / 16);
        HandPoint {
            x: clamp_body(i32::from(self.palm.x) + index * (30 + spread) + opposition),
            y: clamp_body(i32::from(self.palm.y) - reach),
            depth: clamp_body(i32::from(self.palm.depth) + flexion / 4 + opposition.abs() / 4),
        }
    }

    fn validate(&self) -> Result<(), WorkstationError> {
        self.palm.validate()?;
        if !(-BODY_MAX..=BODY_MAX).contains(&self.wrist)
            || !(-BODY_MAX..=BODY_MAX).contains(&self.spread)
            || !(-BODY_MAX..=BODY_MAX).contains(&self.thumb_opposition)
            || self
                .digits
                .iter()
                .any(|digit| digit.flexion > BODY_MAX as u16)
        {
            return Err(WorkstationError::InvalidState);
        }
        Ok(())
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
            wrist: 0,
            spread: 0,
            thumb_opposition: 0,
            digits: [DigitState::default(); DIGIT_COUNT],
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

    pub(crate) fn validate(&self) -> Result<(), WorkstationError> {
        for eye in &self.eyes {
            eye.gaze.validate()?;
        }
        self.hand.validate()?;
        for (index, dynamics) in self.dynamics.iter().enumerate() {
            let axis = BodyAxis::ALL[index];
            if dynamics.decrease_effort > BODY_MAX as u16
                || dynamics.increase_effort > BODY_MAX as u16
                || dynamics.velocity.unsigned_abs() > axis_velocity_bound(axis)
            {
                return Err(WorkstationError::InvalidState);
            }
        }
        Ok(())
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
            BodyAxis::Wrist => self.hand.wrist,
            BodyAxis::Spread => self.hand.spread,
            BodyAxis::ThumbOpposition => self.hand.thumb_opposition,
            BodyAxis::FingerFlexion { digit } => {
                i16::try_from(self.hand.digit(digit).flexion).unwrap_or(BODY_MAX) - MID
            }
        }
    }

    fn limits(&self, axis: BodyAxis) -> (bool, bool) {
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
            BodyAxis::Wrist => (self.hand.wrist == -BODY_MAX, self.hand.wrist == BODY_MAX),
            BodyAxis::Spread => (self.hand.spread == -BODY_MAX, self.hand.spread == BODY_MAX),
            BodyAxis::ThumbOpposition => (
                self.hand.thumb_opposition == -BODY_MAX,
                self.hand.thumb_opposition == BODY_MAX,
            ),
            BodyAxis::FingerFlexion { digit } => {
                let flexion = self.hand.digit(digit).flexion;
                (flexion == 0, flexion == BODY_MAX as u16)
            }
        }
    }

    fn apply_net(&mut self, axis: BodyAxis, net_impulse: i32) {
        let amount = net_impulse.saturating_mul(axis_step(axis));
        match axis {
            BodyAxis::EyeHorizontal { eye } => {
                self.eyes[eye.index()].gaze.x =
                    add_bounded(self.eyes[eye.index()].gaze.x, amount, 0, BODY_MAX);
            }
            BodyAxis::EyeVertical { eye } => {
                self.eyes[eye.index()].gaze.y =
                    add_bounded(self.eyes[eye.index()].gaze.y, amount, 0, BODY_MAX);
            }
            BodyAxis::PalmHorizontal => {
                self.hand.palm.x = add_bounded(self.hand.palm.x, amount, 0, BODY_MAX);
            }
            BodyAxis::PalmVertical => {
                self.hand.palm.y = add_bounded(self.hand.palm.y, amount, 0, BODY_MAX);
            }
            BodyAxis::PalmDepth => {
                self.hand.palm.depth = add_bounded(self.hand.palm.depth, amount, 0, BODY_MAX);
            }
            BodyAxis::Wrist => {
                self.hand.wrist = add_bounded(self.hand.wrist, amount, -BODY_MAX, BODY_MAX);
            }
            BodyAxis::Spread => {
                self.hand.spread = add_bounded(self.hand.spread, amount, -BODY_MAX, BODY_MAX);
            }
            BodyAxis::ThumbOpposition => {
                self.hand.thumb_opposition =
                    add_bounded(self.hand.thumb_opposition, amount, -BODY_MAX, BODY_MAX);
            }
            BodyAxis::FingerFlexion { digit } => {
                self.hand.digits[digit.index()].flexion =
                    add_bounded_u16(self.hand.digits[digit.index()].flexion, amount);
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
    eyes: [LightField; 2],
    contacts: [ContactSample; TOUCH_SITES],
}

impl WorldSample {
    pub fn new(
        eyes: [LightField; 2],
        contacts: [ContactSample; TOUCH_SITES],
    ) -> Result<Self, WorkstationError> {
        let sample = Self { eyes, contacts };
        sample.validate()?;
        Ok(sample)
    }

    pub fn eye(&self, eye: Eye) -> &LightField {
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

const fn axis_step(axis: BodyAxis) -> i32 {
    match axis {
        BodyAxis::EyeHorizontal { .. }
        | BodyAxis::EyeVertical { .. }
        | BodyAxis::PalmHorizontal
        | BodyAxis::PalmVertical
        | BodyAxis::PalmDepth => 16,
        BodyAxis::Wrist | BodyAxis::Spread | BodyAxis::ThumbOpposition => 32,
        BodyAxis::FingerFlexion { .. } => 64,
    }
}

const fn axis_velocity_bound(axis: BodyAxis) -> u16 {
    match axis {
        BodyAxis::Wrist | BodyAxis::Spread | BodyAxis::ThumbOpposition => BODY_MAX as u16 * 2,
        _ => BODY_MAX as u16,
    }
}

fn add_bounded(value: i16, amount: i32, min: i16, max: i16) -> i16 {
    let next = i32::from(value).saturating_add(amount);
    i16::try_from(next.clamp(i32::from(min), i32::from(max))).unwrap_or(value)
}

fn add_bounded_u16(value: u16, amount: i32) -> u16 {
    let next = i32::from(value).saturating_add(amount);
    u16::try_from(next.clamp(0, i32::from(BODY_MAX))).unwrap_or(value)
}

fn clamp_body(value: i32) -> i16 {
    i16::try_from(value.clamp(0, i32::from(BODY_MAX))).unwrap_or(BODY_MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_opposing_effort_is_visible_without_movement() {
        let mut state = WorkstationState::default();
        let before = state.clone();
        let axis = BodyAxis::FingerFlexion {
            digit: Digit::Index,
        };
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
    fn independent_eye_and_hand_axes_commute() {
        let eye = BodyAxis::EyeHorizontal { eye: Eye::Left };
        let finger = BodyAxis::FingerFlexion {
            digit: Digit::Little,
        };
        let mut forward = ActuatorFrame::default();
        forward.activate(eye, Direction::Increase, 2);
        forward.activate(finger, Direction::Decrease, 1);
        let mut reverse = ActuatorFrame::default();
        reverse.activate(finger, Direction::Decrease, 1);
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
    fn five_fingertips_have_bounded_three_dimensional_positions() {
        let state = WorkstationState::default();
        let points = Digit::ALL.map(|digit| state.hand().fingertip(digit));
        assert_eq!(points.len(), DIGIT_COUNT);
        assert!(points.iter().all(|point| {
            (0..=BODY_MAX).contains(&point.x())
                && (0..=BODY_MAX).contains(&point.y())
                && (0..=BODY_MAX).contains(&point.depth())
        }));
    }

    #[test]
    fn bounded_motion_reports_actual_delta_and_limit() {
        let axis = BodyAxis::PalmDepth;
        let mut state = WorkstationState::default();
        let mut frame = ActuatorFrame::default();
        frame.activate(axis, Direction::Increase, BODY_MAX as u16);
        let movement = state.integrate(frame);
        assert_eq!(movement[0].velocity, BODY_MAX - 256);
        assert!(state.proprioception()[axis.index()].at_upper_limit);
    }
}
