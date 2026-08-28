#![forbid(unsafe_code)]
//! Observer-only rendering of frozen Academy workstation evidence.

use academy_workstation::{RecordedStep, WorkstationRecording};
use font8x8::{UnicodeFonts, BASIC_FONTS};
use image::{ImageFormat, Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::process::Command;
use truelearner_workstation::{BodyAxis, Digit, Eye};

pub const VIDEO_WIDTH: u32 = 1_280;
pub const VIDEO_HEIGHT: u32 = 720;
pub const FRAME_DURATION_MS: u32 = 250;

const DARK: Rgba<u8> = Rgba([12, 18, 27, 255]);
const PANEL: Rgba<u8> = Rgba([24, 34, 46, 255]);
const WHITE: Rgba<u8> = Rgba([242, 246, 244, 255]);
const MUTED: Rgba<u8> = Rgba([158, 176, 188, 255]);
const MINT: Rgba<u8> = Rgba([128, 226, 185, 255]);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewFrame {
    pub file: String,
    pub sha256: String,
    pub sequence: u64,
    pub duration_ms: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationReviewManifest {
    pub schema_version: u16,
    pub seed: u64,
    pub step_count: usize,
    pub replay_exact: bool,
    pub recording_file: String,
    pub recording_sha256: String,
    pub video_file: String,
    pub width: u32,
    pub height: u32,
    pub frames: Vec<ReviewFrame>,
}

pub fn record_and_render(
    destination: &Path,
    seed: u64,
    step_count: usize,
) -> Result<WorkstationReviewManifest, ReviewError> {
    fs::create_dir_all(destination).map_err(ReviewError::io)?;
    let recording = WorkstationRecording::capture(seed, step_count).map_err(ReviewError::world)?;
    let bytes = recording.canonical_bytes().map_err(ReviewError::world)?;
    let recording_path = destination.join("recording.tlwr");
    fs::write(&recording_path, &bytes).map_err(ReviewError::io)?;
    drop(recording);

    render_recording_file(&recording_path, destination)
}

pub fn render_recording_file(
    recording_path: &Path,
    destination: &Path,
) -> Result<WorkstationReviewManifest, ReviewError> {
    let bytes = fs::read(recording_path).map_err(ReviewError::io)?;
    let recording = WorkstationRecording::decode(&bytes).map_err(ReviewError::world)?;
    recording
        .verify_exact_replay()
        .map_err(ReviewError::world)?;
    fs::create_dir_all(destination).map_err(ReviewError::io)?;

    let canonical_recording = destination.join("recording.tlwr");
    if canonical_recording != recording_path {
        fs::write(&canonical_recording, &bytes).map_err(ReviewError::io)?;
    }

    let mut frames = Vec::with_capacity(recording.steps().len());
    for recorded in recording.steps() {
        let name = format!("frame-{:04}.png", recorded.observation.sequence);
        let png = render_frame_png(recorded)?;
        fs::write(destination.join(&name), &png).map_err(ReviewError::io)?;
        frames.push(ReviewFrame {
            file: name,
            sha256: sha256(&png),
            sequence: recorded.observation.sequence,
            duration_ms: FRAME_DURATION_MS,
        });
    }
    encode_video(destination, &frames, "episode.mp4")?;

    let manifest = WorkstationReviewManifest {
        schema_version: 1,
        seed: recording.seed(),
        step_count: frames.len(),
        replay_exact: true,
        recording_file: "recording.tlwr".to_string(),
        recording_sha256: sha256(&bytes),
        video_file: "episode.mp4".to_string(),
        width: VIDEO_WIDTH,
        height: VIDEO_HEIGHT,
        frames,
    };
    let json = serde_json::to_vec_pretty(&manifest).map_err(ReviewError::json)?;
    fs::write(destination.join("manifest.json"), json).map_err(ReviewError::io)?;
    Ok(manifest)
}

pub fn render_frame_png(recorded: &RecordedStep) -> Result<Vec<u8>, ReviewError> {
    let image = render_frame(recorded);
    let mut bytes = Cursor::new(Vec::new());
    image
        .write_to(&mut bytes, ImageFormat::Png)
        .map_err(|error| ReviewError(error.to_string()))?;
    Ok(bytes.into_inner())
}

fn render_frame(recorded: &RecordedStep) -> RgbaImage {
    let mut frame = RgbaImage::from_pixel(VIDEO_WIDTH, VIDEO_HEIGHT, DARK);
    fill_rect(&mut frame, 24, 20, VIDEO_WIDTH - 48, 52, PANEL);
    draw_text(&mut frame, 42, 37, "TRUELEARNER WORKSTATION", 2, MINT);
    draw_text(
        &mut frame,
        964,
        41,
        &format!("STEP {:04}", recorded.observation.sequence),
        1,
        WHITE,
    );

    draw_text(&mut frame, 32, 83, "LEFT EYE | ORGANISM INPUT", 1, MUTED);
    draw_text(&mut frame, 648, 83, "RIGHT EYE | ORGANISM INPUT", 1, MUTED);
    blit_light(
        &mut frame,
        recorded.observation.sample.eye(Eye::Left),
        32,
        104,
        600,
        338,
    );
    blit_light(
        &mut frame,
        recorded.observation.sample.eye(Eye::Right),
        648,
        104,
        600,
        338,
    );
    stroke_rect(&mut frame, 32, 104, 600, 338, MUTED);
    stroke_rect(&mut frame, 648, 104, 600, 338, MUTED);

    fill_rect(&mut frame, 24, 462, VIDEO_WIDTH - 48, 234, PANEL);
    let lines = annotation_lines(recorded);
    for (index, line) in lines.iter().enumerate() {
        let column = index % 2;
        let row = index / 2;
        draw_text(
            &mut frame,
            42 + u32::try_from(column).unwrap_or(0) * 616,
            482 + u32::try_from(row).unwrap_or(0) * 34,
            &clipped(line, 72),
            1,
            if index < 2 { WHITE } else { MUTED },
        );
    }
    frame
}

fn annotation_lines(recorded: &RecordedStep) -> Vec<String> {
    let body = &recorded.observation.body;
    let mut contacts = Vec::new();
    let palm_pressure = recorded.observation.sample.contacts()[0].pressure();
    if palm_pressure > 0 {
        contacts.push(format!("PALM:{palm_pressure}"));
    }
    contacts.extend(Digit::ALL.iter().enumerate().filter_map(|(index, digit)| {
        let pressure = recorded.observation.sample.contacts()[index + 1].pressure();
        (pressure > 0).then(|| format!("{}:{pressure}", digit_short(*digit)))
    }));
    let movements = body
        .movements
        .iter()
        .filter(|movement| movement.changed)
        .take(4)
        .map(|movement| format!("{}:{:+}", axis_short(movement.axis), movement.velocity))
        .collect::<Vec<_>>();
    let events = recorded
        .observation
        .device_events
        .iter()
        .take(3)
        .map(|event| format!("{event:?}"))
        .collect::<Vec<_>>();
    let device = &recorded.before.device;
    let body_fingerprint = body
        .body_fingerprint
        .get(..12)
        .unwrap_or(&body.body_fingerprint);
    let world_fingerprint = recorded
        .before
        .world_fingerprint
        .get(..12)
        .unwrap_or(&recorded.before.world_fingerprint);
    vec![
        format!(
            "PHYSICAL TICK {}   QUIESCENT {}   POSE CHANGED {}",
            body.physical_tick,
            yes_no(body.naturally_quiescent),
            yes_no(body.pose_changed)
        ),
        format!(
            "WORK {}   CROSSINGS {}   UPDATES {}",
            body.metrics.physical_work,
            body.crossings.len(),
            body.metrics.plasticity_updates
        ),
        format!(
            "MOVEMENT {}",
            if movements.is_empty() {
                "NONE".to_string()
            } else {
                movements.join("  ")
            }
        ),
        format!(
            "CONTACT {}",
            if contacts.is_empty() {
                "NONE".to_string()
            } else {
                contacts.join("  ")
            }
        ),
        format!(
            "DEVICE CURSOR {},{}   KEYS {}   SELECTED {}",
            device.cursor().x,
            device.cursor().y,
            device.keys_down().count(),
            yes_no(device.selected())
        ),
        format!(
            "EVENT {}",
            if events.is_empty() {
                "NONE".to_string()
            } else {
                events.join("  ")
            }
        ),
        format!("VISIBLE TEXT {}", clipped(device.text(), 58)),
        format!("BODY {body_fingerprint}   WORLD {world_fingerprint}"),
    ]
}

fn blit_light(
    destination: &mut RgbaImage,
    source: &truelearner_workstation::LightField,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    for dy in 0..height {
        let source_y = dy.saturating_mul(u32::from(source.height())) / height;
        for dx in 0..width {
            let source_x = dx.saturating_mul(u32::from(source.width())) / width;
            let index = usize::try_from(
                source_y
                    .saturating_mul(u32::from(source.width()))
                    .saturating_add(source_x),
            )
            .unwrap_or(usize::MAX);
            if let Some(value) = source.pixels().get(index) {
                destination.put_pixel(x + dx, y + dy, Rgba([*value, *value, *value, 255]));
            }
        }
    }
}

fn fill_rect(image: &mut RgbaImage, x: u32, y: u32, width: u32, height: u32, color: Rgba<u8>) {
    for py in y..y.saturating_add(height).min(image.height()) {
        for px in x..x.saturating_add(width).min(image.width()) {
            image.put_pixel(px, py, color);
        }
    }
}

fn stroke_rect(image: &mut RgbaImage, x: u32, y: u32, width: u32, height: u32, color: Rgba<u8>) {
    for px in x..x.saturating_add(width) {
        put(image, px, y, color);
        put(image, px, y.saturating_add(height).saturating_sub(1), color);
    }
    for py in y..y.saturating_add(height) {
        put(image, x, py, color);
        put(image, x.saturating_add(width).saturating_sub(1), py, color);
    }
}

fn draw_text(image: &mut RgbaImage, x: u32, y: u32, text: &str, scale: u32, color: Rgba<u8>) {
    let mut cursor = x;
    for character in text.chars() {
        let glyph = BASIC_FONTS
            .get(character)
            .or_else(|| BASIC_FONTS.get('?'))
            .unwrap_or([0; 8]);
        for (row, bits) in glyph.into_iter().enumerate() {
            for column in 0..8_u32 {
                if bits & (1 << column) != 0 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            put(
                                image,
                                cursor + column * scale + sx,
                                y + u32::try_from(row).unwrap_or(0) * scale + sy,
                                color,
                            );
                        }
                    }
                }
            }
        }
        cursor = cursor.saturating_add(8 * scale);
        if cursor >= image.width().saturating_sub(8 * scale) {
            break;
        }
    }
}

fn put(image: &mut RgbaImage, x: u32, y: u32, color: Rgba<u8>) {
    if x < image.width() && y < image.height() {
        image.put_pixel(x, y, color);
    }
}

fn encode_video(
    destination: &Path,
    frames: &[ReviewFrame],
    video_name: &str,
) -> Result<(), ReviewError> {
    let mut timeline = String::from("ffconcat version 1.0\n");
    for frame in frames {
        timeline.push_str(&format!("file '{}'\n", frame.file));
        timeline.push_str(&format!(
            "duration {:.3}\n",
            f64::from(frame.duration_ms) / 1_000.0
        ));
    }
    let final_frame = frames
        .last()
        .ok_or_else(|| ReviewError("recording contains no review frames".to_string()))?;
    timeline.push_str(&format!("file '{}'\n", final_frame.file));
    fs::write(destination.join("timeline.ffconcat"), timeline).map_err(ReviewError::io)?;

    let status = Command::new("ffmpeg")
        .current_dir(destination)
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            "timeline.ffconcat",
            "-vf",
            "format=yuv420p",
            "-r",
            "30",
            "-movflags",
            "+faststart",
            video_name,
        ])
        .status()
        .map_err(|error| ReviewError(format!("ffmpeg unavailable: {error}")))?;
    if !status.success() {
        return Err(ReviewError(format!("ffmpeg exited with status {status}")));
    }
    Ok(())
}

fn clipped(value: &str, limit: usize) -> String {
    let clipped = value.chars().take(limit).collect::<String>();
    if value.chars().count() > limit {
        format!("{clipped}...")
    } else if clipped.is_empty() {
        "<EMPTY>".to_string()
    } else {
        clipped
    }
}

fn axis_short(axis: BodyAxis) -> &'static str {
    match axis {
        BodyAxis::EyeHorizontal { eye: Eye::Left } => "LEFT-EYE-X",
        BodyAxis::EyeVertical { eye: Eye::Left } => "LEFT-EYE-Y",
        BodyAxis::EyeHorizontal { eye: Eye::Right } => "RIGHT-EYE-X",
        BodyAxis::EyeVertical { eye: Eye::Right } => "RIGHT-EYE-Y",
        BodyAxis::PalmHorizontal => "PALM-X",
        BodyAxis::PalmVertical => "PALM-Y",
        BodyAxis::PalmDepth => "PALM-DEPTH",
        BodyAxis::Wrist => "WRIST",
        BodyAxis::Spread => "SPREAD",
        BodyAxis::ThumbOpposition => "THUMB-OPPOSE",
        BodyAxis::FingerFlexion { digit } => digit_short(digit),
    }
}

fn digit_short(digit: Digit) -> &'static str {
    match digit {
        Digit::Thumb => "THUMB",
        Digit::Index => "INDEX",
        Digit::Middle => "MIDDLE",
        Digit::Ring => "RING",
        Digit::Little => "LITTLE",
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "YES"
    } else {
        "NO"
    }
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[derive(Debug)]
pub struct ReviewError(String);

impl ReviewError {
    fn io(error: std::io::Error) -> Self {
        Self(error.to_string())
    }

    fn json(error: serde_json::Error) -> Self {
        Self(error.to_string())
    }

    fn world(error: impl fmt::Display) -> Self {
        Self(error.to_string())
    }
}

impl fmt::Display for ReviewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ReviewError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_step_renders_deterministically_with_both_eyes() {
        let recording = WorkstationRecording::capture(82_101, 1).unwrap();
        let recorded = &recording.steps()[0];
        let first = render_frame_png(recorded).unwrap();
        let second = render_frame_png(recorded).unwrap();
        let decoded = image::load_from_memory(&first).unwrap().to_rgba8();

        assert_eq!(first, second);
        assert_eq!(decoded.dimensions(), (VIDEO_WIDTH, VIDEO_HEIGHT));
        assert_ne!(
            recorded.observation.sample.eye(Eye::Left),
            recorded.observation.sample.eye(Eye::Right)
        );
    }

    #[test]
    fn annotations_describe_observer_evidence_not_instructions() {
        let recording = WorkstationRecording::capture(82_102, 1).unwrap();
        let annotation = annotation_lines(&recording.steps()[0]).join(" ");

        for present in ["PHYSICAL TICK", "QUIESCENT", "WORK", "MOVEMENT", "CONTACT"] {
            assert!(annotation.contains(present));
        }
        for absent in ["EXPECTED", "CORRECT ACTION", "TARGET KEY", "REWARD"] {
            assert!(!annotation.contains(absent));
        }
    }
}
