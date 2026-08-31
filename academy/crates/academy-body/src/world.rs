use crate::course::{BodyCapability, BodyCourseError};
use truelearner_workstation::{
    BodyAxis, ContactSample, Digit, Eye, HandPoint, HandState, LightField, MotorEffect, Point,
    WorkstationRead, WorkstationStepObservation, WorldSample, BODY_MAX, TOUCH_SITES,
};

const SIDE: u16 = 9;
const CONTACT_DEPTH: i16 = 600;
const EYE_CENTER: i16 = (BODY_MAX + 1) / 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScenePoint {
    x: i32,
    y: i32,
}

impl ScenePoint {
    const fn from_point(point: Point) -> Self {
        Self {
            x: point.x() as i32,
            y: point.y() as i32,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StereoDepth {
    Near,
    Middle,
    Far,
}

impl StereoDepth {
    const fn generated(seed: u64) -> Self {
        match seed % 3 {
            0 => Self::Near,
            1 => Self::Middle,
            _ => Self::Far,
        }
    }

    const fn half_disparity(self) -> i16 {
        match self {
            Self::Near => 384,
            Self::Middle => 256,
            Self::Far => 128,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FlatWorld {
    seed: u64,
    capability: BodyCapability,
    step: u32,
    target: Point,
    stereo_targets: Option<[ScenePoint; 2]>,
    passive_motion: bool,
}

impl FlatWorld {
    pub(crate) fn generated(seed: u64, capability: BodyCapability) -> Self {
        let target = if matches!(
            capability,
            BodyCapability::GazeContingency
                | BodyCapability::GazeControl
                | BodyCapability::BinocularDepth
        ) {
            Point::new(512, 512).expect("fixed target is bounded")
        } else {
            Point::new(
                192 + i16::try_from(seed % 640).unwrap_or(0),
                192 + i16::try_from(seed.rotate_left(17) % 640).unwrap_or(0),
            )
            .expect("generated target is bounded")
        };
        Self {
            seed,
            capability,
            step: 0,
            target,
            stereo_targets: None,
            passive_motion: false,
        }
    }

    #[cfg(test)]
    pub(crate) fn passive(seed: u64) -> Self {
        let mut world = Self::generated(seed, BodyCapability::SelfWorld);
        world.passive_motion = true;
        world
    }

    pub(crate) fn sample(
        &mut self,
        body: &WorkstationRead,
    ) -> Result<WorldSample, BodyCourseError> {
        let target = if self.passive_motion {
            self.target
                .offset_public(i16::try_from(self.step % 7).unwrap_or(0) * 64, 0)
        } else {
            self.target
        };
        let targets = if self.capability == BodyCapability::BinocularDepth {
            let gazes = Eye::ALL.map(|eye| body.state.eye(eye).gaze());
            self.fixed_stereo_targets(gazes)
        } else {
            [ScenePoint::from_point(target); 2]
        };
        let [left_target, right_target] = targets;
        let left = render_eye(self.seed, left_target, body, Eye::Left)?;
        let right = render_eye(self.seed.rotate_left(11), right_target, body, Eye::Right)?;
        let contacts = contacts(self.capability, self.seed, self.step, body)?;
        self.step = self.step.saturating_add(1);
        Ok(WorldSample::new([left, right], contacts)?)
    }

    pub(crate) fn progress_parents(
        &self,
        observation: &WorkstationStepObservation,
    ) -> Vec<MotorEffect> {
        if self.capability < BodyCapability::Contact {
            return Vec::new();
        }
        let before = pose_contact_pressures(observation.state_before.hand());
        let after = pose_contact_pressures(observation.state_after.hand());
        let changed_axes = observation
            .movements
            .iter()
            .filter(|movement| movement.changed)
            .filter_map(|movement| match movement.axis {
                BodyAxis::PalmDepth if before != after => Some(movement.axis),
                BodyAxis::FingerFlexion { digit }
                    if before[digit_index(digit) + 1] != after[digit_index(digit) + 1] =>
                {
                    Some(movement.axis)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        observation
            .crossings
            .iter()
            .copied()
            .filter(|crossing| changed_axes.contains(&crossing.control.axis()))
            .collect()
    }

    fn fixed_stereo_targets(&mut self, gazes: [Point; 2]) -> [ScenePoint; 2] {
        *self
            .stereo_targets
            .get_or_insert_with(|| eye_targets(self.capability, self.seed, gazes))
    }
}

fn eye_targets(capability: BodyCapability, seed: u64, centers: [Point; 2]) -> [ScenePoint; 2] {
    if capability != BodyCapability::BinocularDepth {
        return centers.map(ScenePoint::from_point);
    }
    let disparity = i32::from(StereoDepth::generated(seed).half_disparity());
    [
        ScenePoint {
            x: i32::from(centers[0].x()) + disparity,
            y: i32::from(centers[0].y()),
        },
        ScenePoint {
            x: i32::from(centers[1].x()) - disparity,
            y: i32::from(centers[1].y()),
        },
    ]
}

fn render_eye(
    seed: u64,
    target: ScenePoint,
    body: &WorkstationRead,
    eye: Eye,
) -> Result<LightField, BodyCourseError> {
    render_eye_at(
        seed,
        target,
        body.state.eye(eye).gaze(),
        body.state.hand(),
        eye,
    )
}

fn render_eye_at(
    seed: u64,
    target: ScenePoint,
    gaze: Point,
    hand: &HandState,
    eye: Eye,
) -> Result<LightField, BodyCourseError> {
    let mut pixels = (0..SIDE)
        .flat_map(|y| (0..SIDE).map(move |x| retinal_background(seed, x, y, gaze)))
        .collect::<Vec<_>>();
    set_world_pixel(&mut pixels, target, gaze, 255);
    for index in 0..6_u64 {
        let x = i16::try_from((seed.rotate_left(index as u32) + index * 97) % 1_024).unwrap_or(0);
        let y = i16::try_from((seed.rotate_right(index as u32) + index * 53) % 1_024).unwrap_or(0);
        set_world_pixel(
            &mut pixels,
            ScenePoint::from_point(Point::new(x, y)?),
            gaze,
            48,
        );
    }
    set_world_pixel(
        &mut pixels,
        ScenePoint::from_point(project_hand_point(hand.palm(), eye)),
        gaze,
        96,
    );
    for digit in Digit::ALL {
        set_world_pixel(
            &mut pixels,
            ScenePoint::from_point(project_hand_point(hand.fingertip(digit), eye)),
            gaze,
            128,
        );
    }
    Ok(LightField::new(SIDE, SIDE, pixels)?)
}

fn project_hand_point(point: HandPoint, eye: Eye) -> Point {
    let half_disparity = point.depth() / 16;
    let signed = match eye {
        Eye::Left => -half_disparity,
        Eye::Right => half_disparity,
    };
    Point::new(
        point.x().saturating_add(signed).clamp(0, BODY_MAX),
        point.y(),
    )
    .expect("projected hand point is bounded")
}

fn background_light(seed: u64, x: u16, y: u16) -> u8 {
    let offset = u64::from(x) * 5 + u64::from(y) * 11;
    let value = seed.wrapping_add(offset) % 32;
    u8::try_from(value).unwrap_or(0)
}

fn retinal_background(seed: u64, x: u16, y: u16, gaze: Point) -> u8 {
    let world_index = |retinal: u16, gaze: i16| {
        let retinal = i32::from(retinal) * i32::from(BODY_MAX) / i32::from(SIDE - 1);
        let world = retinal + i32::from(gaze) - i32::from(EYE_CENTER);
        (0..=i32::from(BODY_MAX))
            .contains(&world)
            .then(|| u16::try_from(world * i32::from(SIDE - 1) / i32::from(BODY_MAX)).unwrap_or(0))
    };
    match (world_index(x, gaze.x()), world_index(y, gaze.y())) {
        (Some(world_x), Some(world_y)) => background_light(seed, world_x, world_y),
        _ => 0,
    }
}

fn set_world_pixel(pixels: &mut [u8], point: ScenePoint, gaze: Point, value: u8) {
    if let Some(retinal) = retinal_point(point, gaze) {
        set_body_pixel(pixels, retinal, value);
    }
}

fn retinal_point(point: ScenePoint, gaze: Point) -> Option<Point> {
    let retinal = |world: i32, gaze: i16| {
        let coordinate = world - i32::from(gaze) + i32::from(EYE_CENTER);
        i16::try_from(coordinate)
            .ok()
            .filter(|coordinate| (0..=BODY_MAX).contains(coordinate))
    };
    Point::new(retinal(point.x, gaze.x())?, retinal(point.y, gaze.y())?).ok()
}

fn set_body_pixel(pixels: &mut [u8], point: Point, value: u8) {
    let x = usize::try_from(i32::from(point.x()) * i32::from(SIDE - 1) / i32::from(BODY_MAX))
        .unwrap_or(0);
    let y = usize::try_from(i32::from(point.y()) * i32::from(SIDE - 1) / i32::from(BODY_MAX))
        .unwrap_or(0);
    let index = y.saturating_mul(usize::from(SIDE)).saturating_add(x);
    if let Some(pixel) = pixels.get_mut(index) {
        *pixel = (*pixel).max(value);
    }
}

fn contacts(
    capability: BodyCapability,
    seed: u64,
    step: u32,
    body: &WorkstationRead,
) -> Result<[ContactSample; TOUCH_SITES], BodyCourseError> {
    let mut result = [ContactSample::default(); TOUCH_SITES];
    if capability == BodyCapability::DigitSeparation {
        let pressure = 64 + u16::try_from(seed.wrapping_add(u64::from(step)) % 4).unwrap_or(0) * 64;
        for contact in &mut result[1..] {
            *contact = ContactSample::new(pressure, 0)?;
        }
        return Ok(result);
    }
    if capability < BodyCapability::Contact {
        return Ok(result);
    }
    let hand = body.state.hand();
    if hand.palm().depth() >= CONTACT_DEPTH {
        result[0] = ContactSample::new(contact_pressure(hand.palm().depth()), 0)?;
    }
    for digit in Digit::ALL {
        let tip = hand.fingertip(digit);
        if tip.depth() >= CONTACT_DEPTH {
            result[digit_index(digit) + 1] = ContactSample::new(contact_pressure(tip.depth()), 0)?;
        }
    }
    Ok(result)
}

fn contact_pressure(depth: i16) -> u16 {
    let excess = depth.saturating_sub(CONTACT_DEPTH).unsigned_abs();
    96_u16.saturating_add(excess / 4).min(BODY_MAX as u16)
}

fn pose_contact_pressures(hand: &HandState) -> [u16; TOUCH_SITES] {
    let mut pressures = [0; TOUCH_SITES];
    if hand.palm().depth() >= CONTACT_DEPTH {
        pressures[0] = contact_pressure(hand.palm().depth());
    }
    for digit in Digit::ALL {
        let depth = hand.fingertip(digit).depth();
        if depth >= CONTACT_DEPTH {
            pressures[digit_index(digit) + 1] = contact_pressure(depth);
        }
    }
    pressures
}

const fn digit_index(digit: Digit) -> usize {
    match digit {
        Digit::Thumb => 0,
        Digit::Index => 1,
        Digit::Middle => 2,
        Digit::Ring => 3,
        Digit::Little => 4,
    }
}

trait PointOffset {
    fn offset_public(self, dx: i16, dy: i16) -> Point;
}

impl PointOffset for Point {
    fn offset_public(self, dx: i16, dy: i16) -> Point {
        Point::new(
            self.x().saturating_add(dx).clamp(0, BODY_MAX),
            self.y().saturating_add(dy).clamp(0, BODY_MAX),
        )
        .expect("clamped point is bounded")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use truelearner_workstation::WorkstationHarness;

    #[test]
    fn background_distinguishes_every_adjacent_cell_without_high_contrast() {
        for seed in [0, 71, u64::MAX] {
            for y in 0..SIDE {
                for x in 0..SIDE {
                    let value = background_light(seed, x, y);
                    assert!(value < 48);
                    if x + 1 < SIDE {
                        assert_ne!(value, background_light(seed, x + 1, y));
                    }
                    if y + 1 < SIDE {
                        assert_ne!(value, background_light(seed, x, y + 1));
                    }
                    if x + 1 < SIDE && y + 1 < SIDE {
                        assert_ne!(value, background_light(seed, x + 1, y + 1));
                    }
                }
            }
        }
    }

    #[test]
    fn generated_world_is_static_for_an_unchanged_body() {
        let body = WorkstationHarness::new(72).unwrap().read().unwrap();
        let mut world = FlatWorld::generated(73, BodyCapability::GazeContingency);
        assert_eq!(world.sample(&body).unwrap(), world.sample(&body).unwrap());
    }

    #[test]
    fn digit_world_varies_touch_without_preferring_a_finger() {
        let body = WorkstationHarness::new(72).unwrap().read().unwrap();
        let mut world = FlatWorld::generated(73, BodyCapability::DigitSeparation);
        let first = world.sample(&body).unwrap();
        let second = world.sample(&body).unwrap();

        assert_eq!(first.contacts()[0], ContactSample::default());
        assert!(first.contacts()[1..].iter().all(|contact| {
            contact.pressure() == first.contacts()[1].pressure() && contact.slip() == 0
        }));
        assert_ne!(
            first.contacts()[1].pressure(),
            second.contacts()[1].pressure()
        );
    }

    #[test]
    fn stereo_targets_are_crossed_symmetric_and_near_means_more_disparity() {
        let center = Point::new(512, 512).unwrap();
        let near = eye_targets(BodyCapability::BinocularDepth, 0, [center; 2]);
        let middle = eye_targets(BodyCapability::BinocularDepth, 1, [center; 2]);
        let far = eye_targets(BodyCapability::BinocularDepth, 2, [center; 2]);

        for targets in [near, middle, far] {
            assert_eq!(targets[0].y, i32::from(center.y()));
            assert_eq!(targets[1].y, i32::from(center.y()));
            assert_eq!(targets[0].x + targets[1].x, i32::from(center.x()) * 2);
            assert!(targets[0].x > i32::from(center.x()));
            assert!(targets[1].x < i32::from(center.x()));
        }
        let disparity = |targets: [ScenePoint; 2]| targets[0].x - targets[1].x;
        assert!(disparity(near) > disparity(middle));
        assert!(disparity(middle) > disparity(far));
        assert_eq!(
            eye_targets(BodyCapability::GazeControl, 0, [center; 2]),
            [ScenePoint::from_point(center); 2]
        );
    }

    #[test]
    fn binocular_world_exposes_displaced_targets_without_depth_metadata() {
        let body = WorkstationHarness::new(74).unwrap().read().unwrap();
        let sample = FlatWorld::generated(75, BodyCapability::BinocularDepth)
            .sample(&body)
            .unwrap();
        let target_column = |eye| {
            let field = sample.eye(eye);
            field
                .pixels()
                .iter()
                .position(|pixel| *pixel == 255)
                .unwrap()
                % usize::from(field.width())
        };
        assert_ne!(target_column(Eye::Left), target_column(Eye::Right));

        let wire = serde_json::to_string(&sample).unwrap();
        for forbidden in ["depth", "disparity", "target", "course", "capability"] {
            assert!(!wire.contains(forbidden), "leaked {forbidden}: {wire}");
        }
    }

    #[test]
    fn binocular_targets_are_visible_at_receptor_resolution() {
        let body = WorkstationHarness::new(76).unwrap().read().unwrap();
        for seed in 0..3 {
            let sample = FlatWorld::generated(seed, BodyCapability::BinocularDepth)
                .sample(&body)
                .unwrap();
            for eye in Eye::ALL {
                let field = sample.eye(eye);
                assert_eq!((field.width(), field.height()), (9, 9));
                assert!(field.pixels().contains(&255));
            }
        }
    }

    #[test]
    fn one_converging_eye_step_moves_both_targets_toward_center() {
        let body = WorkstationHarness::new(77).unwrap().read().unwrap();
        let center = Point::new(512, 512).unwrap();
        let targets = eye_targets(BodyCapability::BinocularDepth, 2, [center; 2]);
        let target_column = |field: &LightField| {
            field
                .pixels()
                .iter()
                .position(|pixel| *pixel == 255)
                .unwrap()
                % usize::from(field.width())
        };
        let distance_from_center = |field: &LightField| target_column(field).abs_diff(4);

        let left_before =
            render_eye_at(78, targets[0], center, body.state.hand(), Eye::Left).unwrap();
        let right_before =
            render_eye_at(78, targets[1], center, body.state.hand(), Eye::Right).unwrap();
        let left_after = render_eye_at(
            78,
            targets[0],
            Point::new(640, 512).unwrap(),
            body.state.hand(),
            Eye::Left,
        )
        .unwrap();
        let right_after = render_eye_at(
            78,
            targets[1],
            Point::new(384, 512).unwrap(),
            body.state.hand(),
            Eye::Right,
        )
        .unwrap();

        assert!(distance_from_center(&left_after) < distance_from_center(&left_before));
        assert!(distance_from_center(&right_after) < distance_from_center(&right_before));
    }

    #[test]
    fn crossed_targets_are_visible_from_a_boundary_start() {
        let body = WorkstationHarness::new(79).unwrap().read().unwrap();
        let boundary = Point::new(0, 512).unwrap();
        let targets = eye_targets(BodyCapability::BinocularDepth, 2, [boundary; 2]);
        let target_column = |field: &LightField| {
            field
                .pixels()
                .iter()
                .position(|pixel| *pixel == 255)
                .unwrap()
                % usize::from(field.width())
        };
        let left = render_eye_at(80, targets[0], boundary, body.state.hand(), Eye::Left).unwrap();
        let right = render_eye_at(80, targets[1], boundary, body.state.hand(), Eye::Right).unwrap();

        assert!(target_column(&left) > 4);
        assert!(target_column(&right) < 4);
    }

    #[test]
    fn stereo_targets_do_not_follow_later_gaze() {
        let center = Point::new(512, 512).unwrap();
        let moved = Point::new(640, 384).unwrap();
        let mut world = FlatWorld::generated(81, BodyCapability::BinocularDepth);

        let placed = world.fixed_stereo_targets([center; 2]);

        assert_eq!(world.fixed_stereo_targets([moved; 2]), placed);
    }

    #[test]
    fn moving_one_eye_changes_only_its_world_raster() {
        let body = WorkstationHarness::new(77).unwrap().read().unwrap();
        let target = ScenePoint::from_point(Point::new(256, 512).unwrap());
        let center = Point::new(512, 512).unwrap();
        let moved = Point::new(640, 512).unwrap();
        let before = [
            render_eye_at(78, target, center, body.state.hand(), Eye::Left).unwrap(),
            render_eye_at(78, target, center, body.state.hand(), Eye::Right).unwrap(),
        ];
        let after = [
            render_eye_at(78, target, moved, body.state.hand(), Eye::Left).unwrap(),
            render_eye_at(78, target, center, body.state.hand(), Eye::Right).unwrap(),
        ];

        assert_ne!(before[0], after[0]);
        assert_eq!(before[1], after[1]);
    }
}
