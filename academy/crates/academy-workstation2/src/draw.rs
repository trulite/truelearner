//! Shared screen drawing for external applications. Pixels only.
use crate::ScreenPoint;
use truelearner_workstation::{LightField, BODY_MAX};

pub(crate) const FRAME_SIDE: usize = 64;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

impl Rect {
    pub fn contains(self, point: ScreenPoint) -> bool {
        point.x >= self.left && point.x < self.right && point.y >= self.top && point.y < self.bottom
    }

    pub fn area(self) -> u32 {
        u32::from(self.right.abs_diff(self.left)) * u32::from(self.bottom.abs_diff(self.top))
    }

    pub fn overlaps(self, other: Self) -> bool {
        self.left < other.right
            && other.left < self.right
            && self.top < other.bottom
            && other.top < self.bottom
    }
}

pub(crate) fn background() -> Vec<u8> {
    let mut pixels = vec![0; FRAME_SIDE * FRAME_SIDE];
    for y in 0..FRAME_SIDE {
        for x in 0..FRAME_SIDE {
            pixels[y * FRAME_SIDE + x] = 18_u8
                .saturating_add((x / 2) as u8)
                .saturating_add((y / 4) as u8);
        }
    }
    pixels
}

pub(crate) fn frame(pixels: Vec<u8>) -> LightField {
    LightField::new(FRAME_SIDE as u16, FRAME_SIDE as u16, pixels).expect("fixed screen frame")
}

pub(crate) fn distance(left: ScreenPoint, right: ScreenPoint) -> i16 {
    left.x
        .abs_diff(right.x)
        .saturating_add(left.y.abs_diff(right.y))
        .min(i16::MAX as u16) as i16
}

pub(crate) fn fill_rect(pixels: &mut [u8], rect: Rect, value: u8) {
    fill_world_rect(
        pixels,
        i32::from(rect.left),
        i32::from(rect.top),
        i32::from(rect.right),
        i32::from(rect.bottom),
        value,
    );
}

pub(crate) fn fill_world_rect(
    pixels: &mut [u8],
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    value: u8,
) {
    let scale = |coordinate: i32| {
        (coordinate.clamp(0, i32::from(BODY_MAX)) * FRAME_SIDE as i32 / (i32::from(BODY_MAX) + 1))
            as usize
    };
    let start_y = scale(top);
    let end_y = scale(bottom).max(start_y + 1).min(FRAME_SIDE);
    let start_x = scale(left);
    let end_x = scale(right).max(start_x + 1).min(FRAME_SIDE);
    for y in start_y..end_y {
        for x in start_x..end_x {
            pixels[y * FRAME_SIDE + x] = value;
        }
    }
}
