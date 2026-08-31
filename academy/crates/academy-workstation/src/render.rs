use crate::{DeviceState, Rect, WorkstationPresentation, WorldError, WorldGeometry};
use font8x8::{UnicodeFonts, BASIC_FONTS};
use image::imageops::FilterType;
use sha2::{Digest, Sha256};
use std::sync::{Arc, OnceLock};
use truelearner_workstation::{Digit, Eye, HandPoint, LightField, WorkstationState, BODY_MAX};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const ASSET: &[u8] = include_bytes!("../assets/coastal-monitor.png");

#[derive(Clone, Debug)]
pub(crate) struct SceneRenderer {
    photo: Arc<[u8]>,
    photo_width: usize,
    photo_height: usize,
    asset_digest: [u8; 32],
}

impl SceneRenderer {
    pub(crate) fn new() -> Result<Self, WorldError> {
        static RENDERER: OnceLock<Result<SceneRenderer, WorldError>> = OnceLock::new();
        RENDERER.get_or_init(Self::decode).clone()
    }

    fn decode() -> Result<Self, WorldError> {
        let decoded = image::load_from_memory(ASSET).map_err(|_| WorldError::AssetDecode)?;
        let resized = decoded
            .resize_exact(760, 465, FilterType::Triangle)
            .to_luma8();
        Ok(Self {
            photo: Arc::from(resized.as_raw().clone()),
            photo_width: usize::try_from(resized.width()).map_err(|_| WorldError::AssetDecode)?,
            photo_height: usize::try_from(resized.height()).map_err(|_| WorldError::AssetDecode)?,
            asset_digest: Sha256::digest(ASSET).into(),
        })
    }

    pub(crate) const fn asset_digest(&self) -> [u8; 32] {
        self.asset_digest
    }

    pub(crate) fn render(
        &self,
        geometry: &WorldGeometry,
        device: &DeviceState,
        presentation: &WorkstationPresentation,
        body: &WorkstationState,
        eye: Eye,
    ) -> Result<LightField, WorldError> {
        let mut raster = Raster::new(48);
        raster.fill_rect(scale_rect(geometry.monitor), 18);
        if let Some(frame) = presentation.monitor_frame() {
            draw_monitor_frame(&mut raster, geometry.screen, frame);
        } else {
            self.draw_photo(&mut raster, geometry.screen);
            if device.selected() {
                raster.stroke_rect(scale_rect(geometry.screen), 250);
            }
            if device.long_pressed_keys().next().is_some() {
                let screen = scale_rect(geometry.screen);
                raster.fill_rect(
                    PixelRect {
                        x: screen.x + screen.width - 30,
                        y: screen.y + 8,
                        width: 18,
                        height: 18,
                    },
                    250,
                );
            }
            draw_text(
                &mut raster,
                scale_x(geometry.screen.x + 18),
                scale_y(geometry.screen.y + 18),
                device.text(),
                245,
                40,
            );
        }
        if let Some(glyph) = presentation.monitor_glyph() {
            let cue_x = scale_x(geometry.screen.right() - 118);
            let cue_y = scale_y(geometry.screen.y + 34);
            raster.fill_rect(
                PixelRect {
                    x: cue_x - 5,
                    y: cue_y - 5,
                    width: 42,
                    height: 42,
                },
                16,
            );
            draw_glyph_scaled(&mut raster, cue_x, cue_y, glyph, 4, 250);
        }
        let cursor_x = geometry.screen.x
            + i16::try_from(
                i32::from(device.cursor().x) * i32::from(geometry.screen.width - 1)
                    / i32::from(BODY_MAX),
            )
            .unwrap_or(0);
        let cursor_y = geometry.screen.y
            + i16::try_from(
                i32::from(device.cursor().y) * i32::from(geometry.screen.height - 1)
                    / i32::from(BODY_MAX),
            )
            .unwrap_or(0);
        raster.cross(scale_x(cursor_x), scale_y(cursor_y), 5, 255);

        raster.fill_rect(scale_rect(geometry.keyboard), 32);
        for key in geometry.keys() {
            let rect = scale_rect(key.rect);
            let pressed = device.keys_down().any(|id| id == key.id);
            let illuminated = presentation.illuminated_key() == Some(key.id);
            raster.fill_rect(
                rect,
                match (illuminated, pressed) {
                    (true, true) => 180,
                    (true, false) => 245,
                    (false, true) => 82,
                    (false, false) => 112,
                },
            );
            raster.stroke_rect(
                rect,
                if illuminated {
                    255
                } else if pressed {
                    150
                } else {
                    190
                },
            );
            let available = usize::try_from(rect.width.max(0)).unwrap_or(0) / 8;
            let label = key.label.chars().take(available.max(1)).collect::<String>();
            draw_text(
                &mut raster,
                rect.x + 2,
                rect.y + 2,
                &label,
                if illuminated { 16 } else { 235 },
                1,
            );
        }
        let pad = scale_rect(geometry.touchpad);
        raster.fill_rect(pad, if device.touching() { 105 } else { 78 });
        raster.stroke_rect(pad, 150);
        self.draw_hand(&mut raster, body, eye);

        Ok(LightField::new(
            u16::try_from(WIDTH).unwrap_or(640),
            u16::try_from(HEIGHT).unwrap_or(360),
            raster.pixels,
        )?)
    }

    fn draw_photo(&self, raster: &mut Raster, destination: Rect) {
        let rect = scale_rect(destination);
        let width = usize::try_from(rect.width.max(1)).unwrap_or(1);
        let height = usize::try_from(rect.height.max(1)).unwrap_or(1);
        for dy in 0..height {
            let source_y = dy.saturating_mul(self.photo_height) / height;
            for dx in 0..width {
                let source_x = dx.saturating_mul(self.photo_width) / width;
                let value = self.photo[source_y.saturating_mul(self.photo_width) + source_x];
                raster.set(
                    rect.x + i32::try_from(dx).unwrap_or(0),
                    rect.y + i32::try_from(dy).unwrap_or(0),
                    value,
                );
            }
        }
    }

    fn draw_hand(&self, raster: &mut Raster, body: &WorkstationState, eye: Eye) {
        let hand = body.hand();
        let palm = project(hand.palm(), eye);
        raster.circle(scale_x(palm.0), scale_y(palm.1), 7, 210);
        for digit in Digit::ALL {
            let tip = project(hand.fingertip(digit), eye);
            raster.line(
                scale_x(palm.0),
                scale_y(palm.1),
                scale_x(tip.0),
                scale_y(tip.1),
                175,
            );
            raster.circle(scale_x(tip.0), scale_y(tip.1), 4, 245);
        }
    }
}

fn draw_monitor_frame(raster: &mut Raster, destination: Rect, frame: &crate::MonitorFrame) {
    let rect = scale_rect(destination);
    let width = usize::try_from(rect.width.max(1)).unwrap_or(1);
    let height = usize::try_from(rect.height.max(1)).unwrap_or(1);
    let source_width = usize::from(frame.width());
    let source_height = usize::from(frame.height());
    for dy in 0..height {
        let source_y = dy.saturating_mul(source_height) / height;
        for dx in 0..width {
            let source_x = dx.saturating_mul(source_width) / width;
            let value = frame.pixels()[source_y.saturating_mul(source_width) + source_x];
            raster.set(
                rect.x + i32::try_from(dx).unwrap_or(0),
                rect.y + i32::try_from(dy).unwrap_or(0),
                value,
            );
        }
    }
}

fn project(point: HandPoint, eye: Eye) -> (i16, i16) {
    let disparity = point.depth() / 18;
    let signed = match eye {
        Eye::Left => -disparity,
        Eye::Right => disparity,
    };
    (
        point.x().saturating_add(signed).clamp(0, BODY_MAX),
        point.y(),
    )
}

#[derive(Clone, Copy)]
struct PixelRect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn scale_rect(rect: Rect) -> PixelRect {
    let left = scale_x(rect.x);
    let top = scale_y(rect.y);
    let right = scale_x(rect.right());
    let bottom = scale_y(rect.bottom());
    PixelRect {
        x: left,
        y: top,
        width: (right - left).max(1),
        height: (bottom - top).max(1),
    }
}

fn scale_x(value: i16) -> i32 {
    i32::from(value) * i32::try_from(WIDTH - 1).unwrap_or(639) / i32::from(BODY_MAX)
}

fn scale_y(value: i16) -> i32 {
    i32::from(value) * i32::try_from(HEIGHT - 1).unwrap_or(359) / i32::from(BODY_MAX)
}

struct Raster {
    pixels: Vec<u8>,
}

impl Raster {
    fn new(value: u8) -> Self {
        Self {
            pixels: vec![value; WIDTH * HEIGHT],
        }
    }

    fn set(&mut self, x: i32, y: i32, value: u8) {
        let Ok(x) = usize::try_from(x) else { return };
        let Ok(y) = usize::try_from(y) else { return };
        if x < WIDTH && y < HEIGHT {
            self.pixels[y * WIDTH + x] = value;
        }
    }

    fn fill_rect(&mut self, rect: PixelRect, value: u8) {
        for y in rect.y..rect.y + rect.height {
            for x in rect.x..rect.x + rect.width {
                self.set(x, y, value);
            }
        }
    }

    fn stroke_rect(&mut self, rect: PixelRect, value: u8) {
        for x in rect.x..rect.x + rect.width {
            self.set(x, rect.y, value);
            self.set(x, rect.y + rect.height - 1, value);
        }
        for y in rect.y..rect.y + rect.height {
            self.set(rect.x, y, value);
            self.set(rect.x + rect.width - 1, y, value);
        }
    }

    fn cross(&mut self, x: i32, y: i32, radius: i32, value: u8) {
        for offset in -radius..=radius {
            self.set(x + offset, y, value);
            self.set(x, y + offset, value);
        }
    }

    fn circle(&mut self, center_x: i32, center_y: i32, radius: i32, value: u8) {
        for y in -radius..=radius {
            for x in -radius..=radius {
                if x * x + y * y <= radius * radius {
                    self.set(center_x + x, center_y + y, value);
                }
            }
        }
    }

    fn line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, value: u8) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;
        loop {
            self.set(x0, y0, value);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let doubled = error * 2;
            if doubled >= dy {
                error += dy;
                x0 += sx;
            }
            if doubled <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }
}

fn draw_text(raster: &mut Raster, x: i32, y: i32, text: &str, value: u8, max_lines: usize) {
    let mut cursor_x = x;
    let mut cursor_y = y;
    let mut lines = 1;
    for character in text.chars() {
        if character == '\n' {
            if lines >= max_lines {
                break;
            }
            lines += 1;
            cursor_x = x;
            cursor_y += 9;
            continue;
        }
        let Some(bitmap) = BASIC_FONTS.get(character) else {
            continue;
        };
        for (row, bits) in bitmap.into_iter().enumerate() {
            for column in 0..8 {
                if bits & (1 << column) != 0 {
                    raster.set(
                        cursor_x + column,
                        cursor_y + i32::try_from(row).unwrap_or(0),
                        value,
                    );
                }
            }
        }
        cursor_x += 8;
    }
}

fn draw_glyph_scaled(raster: &mut Raster, x: i32, y: i32, glyph: char, scale: i32, value: u8) {
    let Some(bitmap) = BASIC_FONTS.get(glyph) else {
        return;
    };
    for (row, bits) in bitmap.into_iter().enumerate() {
        for column in 0..8 {
            if bits & (1 << column) == 0 {
                continue;
            }
            raster.fill_rect(
                PixelRect {
                    x: x + column * scale,
                    y: y + i32::try_from(row).unwrap_or(0) * scale,
                    width: scale,
                    height: scale,
                },
                value,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceState, WorldGeometry};

    #[test]
    fn checked_in_photo_decodes_and_has_stable_digest() {
        let renderer = SceneRenderer::new().unwrap();
        assert_eq!(
            renderer
                .asset_digest()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>(),
            "a25049d016cd28d70b464040c57997fe9aa69db1502a1ff25e071b95b6768a47"
        );
        assert!(
            renderer.photo.iter().copied().min().unwrap_or(0)
                < renderer.photo.iter().copied().max().unwrap_or(0)
        );
    }

    #[test]
    fn separate_eyes_receive_different_hand_projection() {
        let renderer = SceneRenderer::new().unwrap();
        let geometry = WorldGeometry::standard_ansi_104().unwrap();
        let body = WorkstationState::default();
        let left = renderer
            .render(
                &geometry,
                &DeviceState::default(),
                &WorkstationPresentation::default(),
                &body,
                Eye::Left,
            )
            .unwrap();
        let right = renderer
            .render(
                &geometry,
                &DeviceState::default(),
                &WorkstationPresentation::default(),
                &body,
                Eye::Right,
            )
            .unwrap();
        assert_ne!(left, right);
    }
}
