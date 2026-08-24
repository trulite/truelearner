#![forbid(unsafe_code)]

use academy_arc3::{
    frame_surface, Arc3A1Episode, Arc3A1EpisodeClass, Arc3A1EpisodeOutcome, Arc3A1Suite,
    Arc3A1Turn,
};
use academy_core::VisualSurface;
use academy_episodes::{
    EpisodeCatalog, EpisodeClass, EpisodeFrame, EpisodeOutcome, ReviewEpisode,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use truelearner_arena_format::ContentHash;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const WORLD_SIDE: u32 = 512;
const DARK: [u8; 4] = [10, 14, 22, 255];
const PANEL: [u8; 4] = [20, 29, 41, 255];
const WHITE: [u8; 4] = [244, 247, 244, 255];
const MUTED: [u8; 4] = [161, 176, 188, 255];
const MINT: [u8; 4] = [128, 230, 184, 255];
const VIOLET: [u8; 4] = [207, 194, 255, 255];
const AMBER: [u8; 4] = [247, 190, 111, 255];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: academy-arc3-a1-review SUITE.json DESTINATION")?;
    let destination = std::env::args_os()
        .nth(2)
        .map(PathBuf::from)
        .ok_or("usage: academy-arc3-a1-review SUITE.json DESTINATION")?;
    let suite: Arc3A1Suite = serde_json::from_slice(&fs::read(&source)?)?;
    validate_suite(&suite)?;
    fs::create_dir_all(&destination)?;
    let episodes = suite
        .episodes
        .iter()
        .map(|episode| write_episode(&suite, episode, &destination))
        .collect::<Result<Vec<_>, _>>()?;
    let catalog = EpisodeCatalog {
        schema_version: 1,
        title: "ARC-AGI-3 · First sensorimotor learning".to_string(),
        episodes,
    };
    fs::write(
        destination.join("catalog.json"),
        serde_json::to_vec_pretty(&catalog)?,
    )?;
    fs::copy(&source, destination.join("suite.json"))?;
    println!(
        "ARC3_A1_REVIEW_READY game={} episodes={} destination={}",
        suite.game_id,
        suite.episodes.len(),
        destination.display()
    );
    Ok(())
}

fn validate_suite(suite: &Arc3A1Suite) -> Result<(), Box<dyn std::error::Error>> {
    if suite.schema_version != 1 || suite.episodes.len() != 7 || !suite.exact_replay {
        return Err("ARC3-A1 suite is incomplete or not replay-exact".into());
    }
    for episode in &suite.episodes {
        if episode.turns.is_empty() {
            return Err(format!("episode {} contains no turns", episode.id).into());
        }
        for turn in &episode.turns {
            if turn.frame.len() != 64 * 64 || turn.frame.iter().any(|color| *color > 15) {
                return Err(format!("episode {} contains an invalid ARC frame", episode.id).into());
            }
            if !turn.organism.naturally_quiescent {
                return Err(format!("episode {} did not quiesce", episode.id).into());
            }
        }
    }
    Ok(())
}

fn write_episode(
    suite: &Arc3A1Suite,
    episode: &Arc3A1Episode,
    root: &Path,
) -> Result<ReviewEpisode, Box<dyn std::error::Error>> {
    let relative = PathBuf::from("episodes").join(&episode.id);
    let destination = root.join(&relative);
    fs::create_dir_all(&destination)?;
    let mut timeline = String::from("ffconcat version 1.0\n");
    let mut frames = Vec::new();
    for (index, turn) in episode.turns.iter().enumerate() {
        let frame = render_turn(suite, episode, turn)?;
        let name = format!("frame-{index:03}.png");
        fs::write(destination.join(&name), frame.png_bytes()?)?;
        timeline.push_str(&format!("file '{name}'\nduration 2.200\n"));
        frames.push(EpisodeFrame {
            file: relative.join(&name).to_string_lossy().to_string(),
            duration_ms: 2_200,
            caption: turn.caption.clone(),
            world_fingerprint: fingerprint(&turn.frame),
            output_fingerprint: fingerprint(
                &serde_json::to_vec(&turn.organism).unwrap_or_default(),
            ),
        });
    }
    let final_name = format!("frame-{:03}.png", episode.turns.len().saturating_sub(1));
    timeline.push_str(&format!("file '{final_name}'\n"));
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
        return Err(format!("ffmpeg exited with {status}").into());
    }
    fs::copy(destination.join(&final_name), destination.join("poster.png"))?;
    fs::write(
        destination.join("record.json"),
        serde_json::to_vec_pretty(episode)?,
    )?;

    let first = &episode.turns[0].organism;
    let last = &episode.turns[episode.turns.len() - 1].organism;
    Ok(ReviewEpisode {
        schema_version: 1,
        id: episode.id.clone(),
        title: episode.title.clone(),
        summary: episode.summary.clone(),
        display_name: suite.game_id.clone(),
        class: episode_class(episode.class),
        outcome: episode_outcome(episode.outcome),
        seed: suite.seed,
        physical_work: episode
            .turns
            .iter()
            .map(|turn| turn.organism.physical_work)
            .sum(),
        plasticity_updates: episode
            .turns
            .iter()
            .map(|turn| turn.organism.plasticity_updates)
            .sum(),
        outward_crossings: episode
            .turns
            .iter()
            .map(|turn| turn.organism.outward_crossings)
            .sum(),
        naturally_quiescent: episode
            .turns
            .iter()
            .all(|turn| turn.organism.naturally_quiescent),
        replay_exact: suite.exact_replay,
        body_before: first.body_fingerprint.clone(),
        body_after: last.body_fingerprint.clone(),
        video_file: relative.join("episode.mp4").to_string_lossy().to_string(),
        poster_file: relative.join("poster.png").to_string_lossy().to_string(),
        record_file: relative.join("record.json").to_string_lossy().to_string(),
        frames,
    })
}

fn render_turn(
    suite: &Arc3A1Suite,
    episode: &Arc3A1Episode,
    turn: &Arc3A1Turn,
) -> Result<VisualSurface, Box<dyn std::error::Error>> {
    let world = frame_surface(&turn.frame, WORLD_SIDE / 64)?;
    let mut surface = VisualSurface::new(WIDTH, HEIGHT, DARK);
    fill_rect(&mut surface, 24, 24, WIDTH - 48, 64, PANEL);
    surface.draw_text("ARC-AGI-3", (46, 43), 2, VIOLET);
    surface.draw_text(&suite.game_id.to_uppercase(), (1_090, 46), 1, MUTED);
    blit(&mut surface, &world, 40, 128);
    surface.draw_text(&episode.title.to_uppercase(), (604, 132), 2, WHITE);
    draw_wrapped(&mut surface, &turn.caption, (604, 186), 46, MUTED);

    metric(
        &mut surface,
        604,
        276,
        "RASTER CONTEXT",
        &turn.organism.context.to_string(),
    );
    metric(
        &mut surface,
        900,
        276,
        "MOTOR CROSSING",
        &optional_number(turn.organism.motor_crossing),
    );
    metric(
        &mut surface,
        604,
        356,
        "EXTERNAL ACTION",
        &optional_number(turn.organism.action),
    );
    metric(
        &mut surface,
        900,
        356,
        "MOTOR BABBLING",
        if turn.organism.babble_action.is_some() {
            "YES"
        } else {
            "NO"
        },
    );
    metric(
        &mut surface,
        604,
        436,
        "VISIBLE RETURN",
        if turn.organism.support_admitted {
            "ADMITTED"
        } else {
            "NONE"
        },
    );
    metric(
        &mut surface,
        900,
        436,
        "PLASTICITY",
        &format!("{} UPDATE(S)", turn.organism.plasticity_updates),
    );
    metric(
        &mut surface,
        604,
        516,
        "ROUTE",
        &format!(
            "R{}  C{}  {}",
            turn.organism.candidate_resistance,
            turn.organism.candidate_coupling,
            if turn.organism.candidate_live {
                "LIVE"
            } else {
                "GONE"
            }
        ),
    );
    metric(
        &mut surface,
        900,
        516,
        "PHYSICAL WORK",
        &turn.organism.physical_work.to_string(),
    );
    fill_rect(&mut surface, 604, 612, 620, 56, PANEL);
    surface.draw_text(
        if turn.organism.babble_action.is_none() && turn.organism.action.is_some() {
            "ACTION FROM RETAINED PHYSICAL STRUCTURE"
        } else if turn.organism.babble_action.is_some() {
            "ACTION FROM DEVELOPMENTAL MOTOR BABBLING"
        } else {
            "NO OUTWARD ACTION"
        },
        (624, 630),
        1,
        if turn.organism.action.is_some() {
            MINT
        } else {
            AMBER
        },
    );
    Ok(surface)
}

fn metric(surface: &mut VisualSurface, x: u32, y: u32, label: &str, value: &str) {
    surface.draw_text(label, (x, y), 1, MUTED);
    surface.draw_text(value, (x, y + 28), 2, WHITE);
}

fn draw_wrapped(
    surface: &mut VisualSurface,
    text: &str,
    origin: (u32, u32),
    width: usize,
    color: [u8; 4],
) {
    let mut line = String::new();
    let mut y = origin.1;
    for word in text.split_whitespace() {
        if !line.is_empty() && line.len().saturating_add(word.len()).saturating_add(1) > width {
            surface.draw_text(&line, (origin.0, y), 1, color);
            line.clear();
            y = y.saturating_add(20);
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        surface.draw_text(&line, (origin.0, y), 1, color);
    }
}

fn optional_number(value: Option<u8>) -> String {
    value.map_or_else(|| "NONE".to_string(), |value| value.to_string())
}

fn episode_class(class: Arc3A1EpisodeClass) -> EpisodeClass {
    match class {
        Arc3A1EpisodeClass::Development => EpisodeClass::Development,
        Arc3A1EpisodeClass::Test => EpisodeClass::Test,
        Arc3A1EpisodeClass::Control => EpisodeClass::Control,
    }
}

fn episode_outcome(outcome: Arc3A1EpisodeOutcome) -> EpisodeOutcome {
    match outcome {
        Arc3A1EpisodeOutcome::ScaffoldedAction => EpisodeOutcome::ScaffoldedAction,
        Arc3A1EpisodeOutcome::StructureFormed => EpisodeOutcome::StructureFormed,
        Arc3A1EpisodeOutcome::LearnedAction => EpisodeOutcome::LearnedResponse,
        Arc3A1EpisodeOutcome::ExpectedSilence => EpisodeOutcome::ExpectedSilence,
        Arc3A1EpisodeOutcome::MappingFollowed => EpisodeOutcome::MappingFollowed,
        Arc3A1EpisodeOutcome::RetainedAction => EpisodeOutcome::RetainedResponse,
    }
}

fn fingerprint(bytes: &[u8]) -> String {
    ContentHash::of(bytes)
        .as_bytes()
        .iter()
        .take(8)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn fill_rect(surface: &mut VisualSurface, x: u32, y: u32, width: u32, height: u32, color: [u8; 4]) {
    for py in y..y.saturating_add(height) {
        for px in x..x.saturating_add(width) {
            surface.set_pixel(px, py, color);
        }
    }
}

fn blit(destination: &mut VisualSurface, source: &VisualSurface, x: u32, y: u32) {
    let width = usize::try_from(source.width()).unwrap_or(1);
    for (index, pixel) in source.rgba_pixels().chunks_exact(4).enumerate() {
        destination.set_pixel(
            x.saturating_add(u32::try_from(index % width).unwrap_or(0)),
            y.saturating_add(u32::try_from(index / width).unwrap_or(0)),
            pixel.try_into().unwrap_or(DARK),
        );
    }
}
