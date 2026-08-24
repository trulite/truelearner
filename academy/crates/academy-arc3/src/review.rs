#![forbid(unsafe_code)]

use academy_arc3::{frame_surface, Arc3Observation, Arc3Recording};
use academy_core::VisualSurface;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const GAME_SIDE: u32 = 576;
const DARK: [u8; 4] = [11, 16, 24, 255];
const PANEL: [u8; 4] = [22, 31, 43, 255];
const WHITE: [u8; 4] = [244, 247, 244, 255];
const MUTED: [u8; 4] = [166, 179, 190, 255];
const VIOLET: [u8; 4] = [212, 201, 255, 255];

#[derive(Serialize)]
struct Arc3ReviewManifest {
    schema_version: u16,
    game_id: String,
    toolkit_revision: String,
    observations: usize,
    actions: usize,
    final_state: String,
    levels_completed: u16,
    win_levels: u16,
    recording_file: String,
    video_file: String,
    poster_file: String,
    organism_admission: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: academy-arc3-review RECORDING.jsonl DESTINATION")?;
    let destination = std::env::args_os()
        .nth(2)
        .map(PathBuf::from)
        .ok_or("usage: academy-arc3-review RECORDING.jsonl DESTINATION")?;
    let recording = Arc3Recording::read_jsonl(&source)?;
    fs::create_dir_all(&destination)?;

    let mut timeline = String::from("ffconcat version 1.0\n");
    for (index, observation) in recording.observations.iter().enumerate() {
        let frame = render_observation(&recording, observation)?;
        let frame_name = format!("frame-{index:04}.png");
        let png = frame
            .png_bytes()
            .map_err(|error| format!("unable to encode ARC review frame: {error:?}"))?;
        fs::write(destination.join(&frame_name), png)?;
        timeline.push_str(&format!("file '{frame_name}'\nduration 0.600\n"));
    }
    let final_frame = format!(
        "frame-{:04}.png",
        recording.observations.len().saturating_sub(1)
    );
    timeline.push_str(&format!("file '{final_frame}'\n"));
    fs::write(destination.join("timeline.ffconcat"), timeline)?;

    let status = Command::new("ffmpeg")
        .current_dir(&destination)
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
            "episode.mp4",
        ])
        .status()?;
    if !status.success() {
        return Err(format!("ffmpeg exited with status {status}").into());
    }
    fs::copy(
        destination.join(&final_frame),
        destination.join("poster.png"),
    )?;
    fs::copy(&source, destination.join("recording.jsonl"))?;

    let final_observation = recording
        .observations
        .last()
        .ok_or("ARC recording contains no observations")?;
    let manifest = Arc3ReviewManifest {
        schema_version: 1,
        game_id: recording.metadata.game_id.clone(),
        toolkit_revision: recording.metadata.toolkit_revision.clone(),
        observations: recording.observations.len(),
        actions: recording.observations.len().saturating_sub(1),
        final_state: final_observation.state.clone(),
        levels_completed: final_observation.levels_completed,
        win_levels: final_observation.win_levels,
        recording_file: "recording.jsonl".to_string(),
        video_file: "episode.mp4".to_string(),
        poster_file: "poster.png".to_string(),
        organism_admission: false,
    };
    fs::write(
        destination.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;
    println!(
        "ARC3_REVIEW_READY game={} observations={} destination={}",
        recording.metadata.game_id,
        recording.observations.len(),
        destination.display()
    );
    Ok(())
}

fn render_observation(
    recording: &Arc3Recording,
    observation: &Arc3Observation,
) -> Result<VisualSurface, Box<dyn std::error::Error>> {
    let pixels = observation
        .frames
        .last()
        .ok_or("ARC observation contains no frames")?;
    let game = frame_surface(pixels, GAME_SIDE / 64)?;
    let mut review = VisualSurface::new(WIDTH, HEIGHT, DARK);
    fill_rect(&mut review, 24, 24, WIDTH - 48, 64, PANEL);
    review.draw_text("ARC-AGI-3", (46, 43), 2, VIOLET);
    review.draw_text(
        &recording.metadata.game_id.to_uppercase(),
        (1_090, 46),
        1,
        MUTED,
    );
    blit(&mut review, &game, 40, 112);

    review.draw_text(&format!("TURN {}", observation.turn), (664, 138), 3, WHITE);
    review.draw_text(
        &format!("ACTION {}", action_label(observation)),
        (664, 220),
        2,
        VIOLET,
    );
    review.draw_text(
        &format!("STATE {}", observation.state),
        (664, 286),
        1,
        MUTED,
    );
    review.draw_text(
        &format!(
            "LEVELS {} / {}",
            observation.levels_completed, observation.win_levels
        ),
        (664, 330),
        1,
        MUTED,
    );
    review.draw_text(
        &format!("AVAILABLE {:?}", observation.available_actions),
        (664, 374),
        1,
        MUTED,
    );
    review.draw_text("EXTERNAL WORLD CAPTURE", (664, 508), 2, WHITE);
    review.draw_text("NOT YET ADMITTED TO THE ORGANISM", (664, 560), 1, MUTED);
    review.draw_text("64 x 64   16 COLORS   TURN BASED", (664, 612), 1, MUTED);
    Ok(review)
}

fn action_label(observation: &Arc3Observation) -> String {
    match observation.action {
        None => "RESET".to_string(),
        Some(action) if action.id == 6 => action
            .data
            .map(|point| format!("6 @ {},{}", point.x, point.y))
            .unwrap_or_else(|| "6".to_string()),
        Some(action) => action.id.to_string(),
    }
}

fn fill_rect(surface: &mut VisualSurface, x: u32, y: u32, width: u32, height: u32, color: [u8; 4]) {
    for py in y..y.saturating_add(height) {
        for px in x..x.saturating_add(width) {
            surface.set_pixel(px, py, color);
        }
    }
}

fn blit(destination: &mut VisualSurface, source: &VisualSurface, x: u32, y: u32) {
    for (index, pixel) in source.rgba_pixels().chunks_exact(4).enumerate() {
        let source_x =
            u32::try_from(index % usize::try_from(source.width()).unwrap_or(1)).unwrap_or(0);
        let source_y =
            u32::try_from(index / usize::try_from(source.width()).unwrap_or(1)).unwrap_or(0);
        destination.set_pixel(
            x.saturating_add(source_x),
            y.saturating_add(source_y),
            pixel.try_into().unwrap_or(DARK),
        );
    }
}
