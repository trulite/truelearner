use crate::{DeviceEvent, ScreenPoint};
use truelearner_workstation::{LightField, BODY_MAX};

const FRAME_SIDE: usize = 64;
const TEXT_LIMIT: usize = 64;
const TAP_TRAVEL: i16 = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Application {
    active: [Option<ScreenPoint>; 5],
    started: [Option<ScreenPoint>; 5],
    keyboard_shift: i16,
    text: String,
    scale: i16,
}

impl Application {
    pub(crate) fn new(keyboard_shift: i16) -> Self {
        Self {
            active: [None; 5],
            started: [None; 5],
            keyboard_shift: keyboard_shift.clamp(-256, 256),
            text: String::new(),
            scale: 128,
        }
    }

    pub(crate) fn apply(&mut self, events: &[DeviceEvent]) {
        for event in events {
            match *event {
                DeviceEvent::TouchStarted { touch, at } => {
                    self.active[touch.index()] = Some(at);
                    self.started[touch.index()] = Some(at);
                }
                DeviceEvent::TouchMoved { touch, from: _, to } => {
                    let before = pair_distance(&self.active);
                    self.active[touch.index()] = Some(to);
                    let after = pair_distance(&self.active);
                    if let (Some(before), Some(after)) = (before, after) {
                        let delta = after.saturating_sub(before) / 4;
                        self.scale = self.scale.saturating_add(delta).clamp(64, 256);
                    }
                }
                DeviceEvent::TouchEnded { touch, at } => {
                    let start = self.started[touch.index()].take();
                    self.active[touch.index()] = None;
                    if start.is_some_and(|start| distance(start, at) <= TAP_TRAVEL) {
                        if let Some(character) = self.key_at(at) {
                            if self.text.chars().count() < TEXT_LIMIT {
                                self.text.push(character);
                            }
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn frame(&self) -> LightField {
        let mut pixels = vec![0; FRAME_SIDE * FRAME_SIDE];
        for y in 0..FRAME_SIDE {
            for x in 0..FRAME_SIDE {
                pixels[y * FRAME_SIDE + x] = 18_u8
                    .saturating_add((x / 2) as u8)
                    .saturating_add((y / 4) as u8);
            }
        }
        let half = i32::from(self.scale) / 4;
        fill_world_rect(
            &mut pixels,
            512 - half,
            352 - half,
            512 + half,
            352 + half,
            116,
        );
        let (a, b) = key_rects(self.keyboard_shift);
        fill_rect(&mut pixels, a, 176);
        fill_rect(&mut pixels, b, 224);
        let marks = self.text.chars().count().min(16);
        for mark in 0..marks {
            let x = 8 + mark * 3;
            for y in 4..8 {
                pixels[y * FRAME_SIDE + x] = 250;
            }
        }
        LightField::new(FRAME_SIDE as u16, FRAME_SIDE as u16, pixels).expect("fixed screen frame")
    }

    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    pub(crate) const fn scale(&self) -> i16 {
        self.scale
    }

    fn key_at(&self, point: ScreenPoint) -> Option<char> {
        let (a, b) = key_rects(self.keyboard_shift);
        if a.contains(point) {
            Some('A')
        } else if b.contains(point) {
            Some('B')
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
struct Rect {
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
}

impl Rect {
    fn contains(self, point: ScreenPoint) -> bool {
        point.x >= self.left && point.x < self.right && point.y >= self.top && point.y < self.bottom
    }
}

fn key_rects(shift: i16) -> (Rect, Rect) {
    (
        Rect {
            left: 384 + shift,
            top: 672,
            right: 528 + shift,
            bottom: 848,
        },
        Rect {
            left: 544 + shift,
            top: 672,
            right: 688 + shift,
            bottom: 848,
        },
    )
}

fn distance(left: ScreenPoint, right: ScreenPoint) -> i16 {
    left.x
        .abs_diff(right.x)
        .saturating_add(left.y.abs_diff(right.y))
        .min(i16::MAX as u16) as i16
}

fn pair_distance(active: &[Option<ScreenPoint>; 5]) -> Option<i16> {
    let points = active.iter().flatten().copied().collect::<Vec<_>>();
    matches!(points.as_slice(), [_, _]).then(|| distance(points[0], points[1]))
}

fn fill_rect(pixels: &mut [u8], rect: Rect, value: u8) {
    fill_world_rect(
        pixels,
        i32::from(rect.left),
        i32::from(rect.top),
        i32::from(rect.right),
        i32::from(rect.bottom),
        value,
    );
}

fn fill_world_rect(pixels: &mut [u8], left: i32, top: i32, right: i32, bottom: i32, value: u8) {
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
