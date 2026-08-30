use crate::course::{BodyCapability, BodyCourseError};
use truelearner_workstation::{
    ContactSample, Digit, Eye, HandPoint, LightField, Point, WorkstationRead, WorldSample,
    BODY_MAX, TOUCH_SITES,
};

const SIDE: u16 = 33;
const CONTACT_DEPTH: i16 = 600;

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
            Self::Near => 96,
            Self::Middle => 64,
            Self::Far => 32,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FlatWorld {
    seed: u64,
    capability: BodyCapability,
    step: u32,
    target: Point,
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
        let [left_target, right_target] = eye_targets(self.capability, self.seed, target);
        let left = render_eye(self.seed, left_target, body, Eye::Left)?;
        let right = render_eye(self.seed.rotate_left(11), right_target, body, Eye::Right)?;
        let contacts = contacts(self.capability, self.seed, self.step, body)?;
        self.step = self.step.saturating_add(1);
        Ok(WorldSample::new([left, right], contacts)?)
    }
}

fn eye_targets(capability: BodyCapability, seed: u64, center: Point) -> [Point; 2] {
    if capability != BodyCapability::BinocularDepth {
        return [center, center];
    }
    let disparity = StereoDepth::generated(seed).half_disparity();
    [
        center.offset_public(-disparity, 0),
        center.offset_public(disparity, 0),
    ]
}

fn render_eye(
    seed: u64,
    target: Point,
    body: &WorkstationRead,
    eye: Eye,
) -> Result<LightField, BodyCourseError> {
    let mut pixels = (0..SIDE)
        .flat_map(|y| (0..SIDE).map(move |x| background_light(seed, x, y)))
        .collect::<Vec<_>>();
    set_body_pixel(&mut pixels, target, 255);
    for index in 0..6_u64 {
        let x = i16::try_from((seed.rotate_left(index as u32) + index * 97) % 1_024).unwrap_or(0);
        let y = i16::try_from((seed.rotate_right(index as u32) + index * 53) % 1_024).unwrap_or(0);
        set_body_pixel(&mut pixels, Point::new(x, y)?, 48);
    }
    let hand = body.state.hand();
    set_body_pixel(&mut pixels, project_hand_point(hand.palm(), eye), 96);
    for digit in Digit::ALL {
        set_body_pixel(
            &mut pixels,
            project_hand_point(hand.fingertip(digit), eye),
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
    fn stereo_targets_are_symmetric_and_near_means_more_disparity() {
        let center = Point::new(512, 512).unwrap();
        let near = eye_targets(BodyCapability::BinocularDepth, 0, center);
        let middle = eye_targets(BodyCapability::BinocularDepth, 1, center);
        let far = eye_targets(BodyCapability::BinocularDepth, 2, center);

        for targets in [near, middle, far] {
            assert_eq!(targets[0].y(), center.y());
            assert_eq!(targets[1].y(), center.y());
            assert_eq!(targets[0].x() + targets[1].x(), center.x() * 2);
            assert!(targets[0].x() < targets[1].x());
        }
        let disparity = |targets: [Point; 2]| targets[1].x() - targets[0].x();
        assert!(disparity(near) > disparity(middle));
        assert!(disparity(middle) > disparity(far));
        assert_eq!(
            eye_targets(BodyCapability::GazeControl, 0, center),
            [center, center]
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
}
