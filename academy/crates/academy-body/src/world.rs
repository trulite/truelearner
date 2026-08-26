use crate::course::{BodyCapability, BodyCourseError};
use truelearner_human::{
    ContactSample, Digit, HumanRead, LightField, Point, Side, WorldSample, BODY_MAX, TOUCH_SITES,
};

const SIDE: u16 = 33;

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
            BodyCapability::GazeContingency | BodyCapability::GazeControl
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

    pub(crate) fn sample(&mut self, body: &HumanRead) -> Result<WorldSample, BodyCourseError> {
        let target = if self.passive_motion {
            self.target
                .offset_public(i16::try_from(self.step % 7).unwrap_or(0) * 64, 0)
        } else {
            self.target
        };
        let left = render_eye(self.seed, target, body, Side::Left)?;
        let right = render_eye(self.seed.rotate_left(11), target, body, Side::Right)?;
        let contacts = contacts(self.capability, body)?;
        self.step = self.step.saturating_add(1);
        Ok(WorldSample::new([left, right], contacts)?)
    }
}

fn render_eye(
    seed: u64,
    target: Point,
    body: &HumanRead,
    side: Side,
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
    let hand = body.state.hand(side);
    set_body_pixel(&mut pixels, hand.palm(), 96);
    for digit in Digit::ALL {
        set_body_pixel(&mut pixels, hand.fingertip(digit), 128);
    }
    Ok(LightField::new(SIDE, SIDE, pixels)?)
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
    body: &HumanRead,
) -> Result<[[ContactSample; TOUCH_SITES]; 2], BodyCourseError> {
    let mut result = [[ContactSample::default(); TOUCH_SITES]; 2];
    if capability < BodyCapability::Contact {
        return Ok(result);
    }
    for side in [Side::Left, Side::Right] {
        let hand = body.state.hand(side);
        if hand.palm().y() >= 640 {
            result[side_index(side)][0] = ContactSample::new(96 + hand.force() / 4, 0)?;
        }
        for digit in Digit::ALL {
            let tip = hand.fingertip(digit);
            if tip.y() >= 600 {
                result[side_index(side)][digit_index(digit) + 1] =
                    ContactSample::new(128 + hand.force() / 4, 0)?;
            }
        }
    }
    Ok(result)
}

const fn side_index(side: Side) -> usize {
    match side {
        Side::Left => 0,
        Side::Right => 1,
    }
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
    use truelearner_human::HumanHarness;

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
        let body = HumanHarness::new(72).unwrap().read().unwrap();
        let mut world = FlatWorld::generated(73, BodyCapability::GazeContingency);
        assert_eq!(world.sample(&body).unwrap(), world.sample(&body).unwrap());
    }
}
