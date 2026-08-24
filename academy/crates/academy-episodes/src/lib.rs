#![forbid(unsafe_code)]
//! Deterministic, human-facing renderings of canonical Academy evidence.
//!
//! Episode media is observational infrastructure. It is produced only after
//! the physical run and is never admitted back into TrueLearner.

use academy_core::{
    A1Experience, A1ProbeFamily, GenuineTeachingLab, TeachingCase, VisualSurface,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const VIDEO_WIDTH: u32 = 1280;
const VIDEO_HEIGHT: u32 = 720;
const VIEW_WIDTH: u32 = 584;
const VIEW_HEIGHT: u32 = 329;
const LEFT_X: u32 = 40;
const RIGHT_X: u32 = 656;
const VIEW_Y: u32 = 112;
const DARK: [u8; 4] = [14, 20, 30, 255];
const PANEL: [u8; 4] = [22, 31, 43, 255];
const WHITE: [u8; 4] = [244, 247, 244, 255];
const MUTED: [u8; 4] = [166, 179, 190, 255];
const MINT: [u8; 4] = [139, 226, 190, 255];
const VIOLET: [u8; 4] = [212, 201, 255, 255];
const AMBER: [u8; 4] = [240, 197, 150, 255];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EpisodeClass {
    Development,
    Test,
    Control,
}

impl EpisodeClass {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Development => "Development",
            Self::Test => "Test",
            Self::Control => "Control",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EpisodeOutcome {
    StructureFormed,
    LearnedResponse,
    ExpectedSilence,
}

impl EpisodeOutcome {
    pub const fn label(self) -> &'static str {
        match self {
            Self::StructureFormed => "Structure formed",
            Self::LearnedResponse => "Learned response",
            Self::ExpectedSilence => "Correctly silent",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodeFrame {
    pub file: String,
    pub duration_ms: u32,
    pub caption: String,
    pub world_fingerprint: String,
    pub output_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewEpisode {
    pub schema_version: u16,
    pub id: String,
    pub title: String,
    pub summary: String,
    pub display_name: String,
    pub class: EpisodeClass,
    pub outcome: EpisodeOutcome,
    pub seed: u64,
    pub physical_work: u64,
    pub plasticity_updates: u64,
    pub outward_crossings: usize,
    pub naturally_quiescent: bool,
    pub replay_exact: bool,
    pub body_before: String,
    pub body_after: String,
    pub video_file: String,
    pub poster_file: String,
    pub record_file: String,
    pub frames: Vec<EpisodeFrame>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodeCatalog {
    pub schema_version: u16,
    pub title: String,
    pub episodes: Vec<ReviewEpisode>,
}

impl EpisodeCatalog {
    pub fn load(root: &Path) -> Result<Self, EpisodeError> {
        let bytes = fs::read(root.join("catalog.json")).map_err(EpisodeError::io)?;
        serde_json::from_slice(&bytes).map_err(EpisodeError::json)
    }
}

pub fn generate_a1_gallery(root: &Path) -> Result<EpisodeCatalog, EpisodeError> {
    fs::create_dir_all(root).map_err(EpisodeError::io)?;
    let case = TeachingCase::named_creature(205, "Momo");
    let mut learned_lab = GenuineTeachingLab::new(case.clone()).map_err(EpisodeError::academy)?;

    let teaching = learned_lab
        .teach_supported()
        .map_err(EpisodeError::academy)?;
    let teaching = replayed(&learned_lab, teaching)?;
    let learned = learned_lab
        .probe(A1ProbeFamily::LearnedRelation)
        .map_err(EpisodeError::academy)?;
    let learned = replayed(&learned_lab, learned)?;

    let mut episodes = Vec::new();
    episodes.push(write_episode(
        root,
        teaching,
        EpisodeSpec {
            id: "a1-development-momo",
            title: "Momo meets the chime",
            summary: "Two completed physical loops support one reusable relation.",
            class: EpisodeClass::Development,
            outcome: EpisodeOutcome::StructureFormed,
            arrival_caption: "The relation is experienced twice",
            result_caption: "Supported participation changes the body",
        },
    )?);
    episodes.push(write_episode(
        root,
        learned,
        EpisodeSpec {
            id: "a1-test-learned-relation",
            title: "The chime returns",
            summary: "A fresh probe produces the learned wave with no teaching update.",
            class: EpisodeClass::Test,
            outcome: EpisodeOutcome::LearnedResponse,
            arrival_caption: "Fresh probe · no teaching",
            result_caption: "One outward crossing · zero plasticity updates",
        },
    )?);

    let controls = [
        (
            A1ProbeFamily::Echo,
            "a1-control-echo",
            "One cue alone",
            "A single familiar cue cannot manufacture the learned relation.",
        ),
        (
            A1ProbeFamily::Distractor,
            "a1-control-distractor",
            "A distractor arrives",
            "A familiar cue paired with the wrong participant remains silent.",
        ),
        (
            A1ProbeFamily::WrongContext,
            "a1-control-context",
            "The context is wrong",
            "Nearby activity without the learned relation produces no response.",
        ),
        (
            A1ProbeFamily::UnsupportedReturn,
            "a1-control-return",
            "Return without participation",
            "A later return cannot claim structure that did not participate.",
        ),
    ];
    for (family, id, title, summary) in controls {
        let mut lab = GenuineTeachingLab::new(case.clone()).map_err(EpisodeError::academy)?;
        let _ = lab.teach_supported().map_err(EpisodeError::academy)?;
        let control = lab.probe(family).map_err(EpisodeError::academy)?;
        let control = replayed(&lab, control)?;
        episodes.push(write_episode(
            root,
            control,
            EpisodeSpec {
                id,
                title,
                summary,
                class: EpisodeClass::Control,
                outcome: EpisodeOutcome::ExpectedSilence,
                arrival_caption: "Fresh negative control · no teaching",
                result_caption: "No outward crossing · body remains quiet",
            },
        )?);
    }

    let catalog = EpisodeCatalog {
        schema_version: 1,
        title: "A1 · First learned relation".to_string(),
        episodes,
    };
    write_json(&root.join("catalog.json"), &catalog)?;
    Ok(catalog)
}

#[derive(Clone, Copy)]
struct EpisodeSpec<'a> {
    id: &'a str,
    title: &'a str,
    summary: &'a str,
    class: EpisodeClass,
    outcome: EpisodeOutcome,
    arrival_caption: &'a str,
    result_caption: &'a str,
}

fn replayed(
    lab: &GenuineTeachingLab,
    mut experience: A1Experience,
) -> Result<A1Experience, EpisodeError> {
    experience.replay_exact = Some(
        lab.replay(&experience.id)
            .map_err(EpisodeError::academy)?
            .exact,
    );
    Ok(experience)
}

fn write_episode(
    root: &Path,
    experience: A1Experience,
    spec: EpisodeSpec<'_>,
) -> Result<ReviewEpisode, EpisodeError> {
    let relative_dir = PathBuf::from("episodes").join(spec.id);
    let episode_dir = root.join(&relative_dir);
    fs::create_dir_all(&episode_dir).map_err(EpisodeError::io)?;

    let quiet = VisualSurface::new(
        experience.organism_surface.width(),
        experience.organism_surface.height(),
        PANEL,
    );
    let rendered = [
        (
            render_review_frame(&experience, &quiet, spec, true),
            1_800,
            spec.arrival_caption,
        ),
        (
            render_review_frame(&experience, &experience.organism_surface, spec, false),
            2_800,
            spec.result_caption,
        ),
    ];
    let mut frames = Vec::new();
    for (index, (surface, duration_ms, caption)) in rendered.into_iter().enumerate() {
        let name = format!("frame-{index:03}.png");
        fs::write(
            episode_dir.join(&name),
            surface.png_bytes().map_err(EpisodeError::surface)?,
        )
        .map_err(EpisodeError::io)?;
        frames.push(EpisodeFrame {
            file: relative_dir.join(&name).to_string_lossy().into_owned(),
            duration_ms,
            caption: caption.to_string(),
            world_fingerprint: experience.shared_world_surface.fingerprint(),
            output_fingerprint: if index == 0 {
                quiet.fingerprint()
            } else {
                experience.organism_surface.fingerprint()
            },
        });
    }

    let video_name = "episode.mp4";
    encode_video(&episode_dir, &frames, video_name)?;
    let poster_name = "poster.png";
    fs::copy(
        root.join(&frames.last().expect("episode has frames").file),
        episode_dir.join(poster_name),
    )
    .map_err(EpisodeError::io)?;
    let record_name = "record.json";
    write_json(&episode_dir.join(record_name), &experience)?;

    let episode = ReviewEpisode {
        schema_version: 1,
        id: spec.id.to_string(),
        title: spec.title.to_string(),
        summary: spec.summary.to_string(),
        display_name: experience.display_name.clone(),
        class: spec.class,
        outcome: spec.outcome,
        seed: experience.seed,
        physical_work: experience.observation.physical_work,
        plasticity_updates: experience.observation.plasticity_updates,
        outward_crossings: experience.observation.outward_relation_crossings,
        naturally_quiescent: experience.observation.naturally_quiescent,
        replay_exact: experience.replay_exact == Some(true),
        body_before: experience.observation.body_before.clone(),
        body_after: experience.observation.body_after.clone(),
        video_file: relative_dir.join(video_name).to_string_lossy().into_owned(),
        poster_file: relative_dir.join(poster_name).to_string_lossy().into_owned(),
        record_file: relative_dir.join(record_name).to_string_lossy().into_owned(),
        frames,
    };
    write_json(&episode_dir.join("manifest.json"), &episode)?;
    Ok(episode)
}

fn render_review_frame(
    experience: &A1Experience,
    output: &VisualSurface,
    spec: EpisodeSpec<'_>,
    arrival: bool,
) -> VisualSurface {
    let mut frame = VisualSurface::new(VIDEO_WIDTH, VIDEO_HEIGHT, DARK);
    fill_rect(&mut frame, 24, 24, VIDEO_WIDTH - 48, 64, PANEL);
    frame.draw_text(
        &spec.title.to_uppercase(),
        (46, 43),
        2,
        match spec.class {
            EpisodeClass::Development => MINT,
            EpisodeClass::Test => VIOLET,
            EpisodeClass::Control => AMBER,
        },
    );
    frame.draw_text(spec.class.label(), (1_070, 46), 1, MUTED);
    frame.draw_text("WORLD", (LEFT_X, VIEW_Y - 26), 1, MUTED);
    frame.draw_text("OUTPUT", (RIGHT_X, VIEW_Y - 26), 1, MUTED);
    blit_scaled(
        &mut frame,
        &experience.shared_world_surface,
        LEFT_X,
        VIEW_Y,
        VIEW_WIDTH,
        VIEW_HEIGHT,
    );
    blit_scaled(
        &mut frame,
        output,
        RIGHT_X,
        VIEW_Y,
        VIEW_WIDTH,
        VIEW_HEIGHT,
    );
    let caption = if arrival {
        spec.arrival_caption
    } else {
        spec.result_caption
    };
    frame.draw_text(caption, (42, 486), 2, WHITE);
    frame.draw_text(
        &format!(
            "WORK {}   CROSSINGS {}   UPDATES {}",
            experience.observation.physical_work,
            if arrival {
                0
            } else {
                experience.observation.outward_relation_crossings
            },
            if arrival {
                0
            } else {
                experience.observation.plasticity_updates
            }
        ),
        (42, 548),
        1,
        MUTED,
    );
    frame.draw_text(
        &format!(
            "T{} -> T{}   QUIESCENT {}   REPLAY {}",
            experience.observation.clock_start,
            experience.observation.clock_end,
            yes_no(experience.observation.naturally_quiescent),
            yes_no(experience.replay_exact == Some(true)),
        ),
        (42, 582),
        1,
        MUTED,
    );
    frame.draw_text(spec.outcome.label(), (42, 642), 2, WHITE);
    frame
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "YES"
    } else {
        "NO"
    }
}

fn fill_rect(
    surface: &mut VisualSurface,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: [u8; 4],
) {
    for py in y..y.saturating_add(height) {
        for px in x..x.saturating_add(width) {
            surface.set_pixel(px, py, color);
        }
    }
}

fn blit_scaled(
    destination: &mut VisualSurface,
    source: &VisualSurface,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    let source_width = source.width();
    let source_height = source.height();
    for dy in 0..height {
        let source_y = dy.saturating_mul(source_height) / height;
        for dx in 0..width {
            let source_x = dx.saturating_mul(source_width) / width;
            let index = usize::try_from(
                source_y
                    .saturating_mul(source_width)
                    .saturating_add(source_x)
                    .saturating_mul(4),
            )
            .unwrap_or(usize::MAX);
            if let Some(pixel) = source.rgba_pixels().get(index..index.saturating_add(4)) {
                destination.set_pixel(
                    x.saturating_add(dx),
                    y.saturating_add(dy),
                    pixel.try_into().unwrap_or(DARK),
                );
            }
        }
    }
}

fn encode_video(
    episode_dir: &Path,
    frames: &[EpisodeFrame],
    video_name: &str,
) -> Result<(), EpisodeError> {
    let mut timeline = String::from("ffconcat version 1.0\n");
    for frame in frames {
        let name = Path::new(&frame.file)
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| EpisodeError("invalid frame name".to_string()))?;
        timeline.push_str(&format!("file '{name}'\n"));
        timeline.push_str(&format!("duration {:.3}\n", f64::from(frame.duration_ms) / 1_000.0));
    }
    let final_name = Path::new(&frames.last().expect("episode has frames").file)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| EpisodeError("invalid final frame name".to_string()))?;
    timeline.push_str(&format!("file '{final_name}'\n"));
    fs::write(episode_dir.join("timeline.ffconcat"), timeline).map_err(EpisodeError::io)?;

    let result = Command::new("ffmpeg")
        .current_dir(episode_dir)
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
        .map_err(|error| EpisodeError(format!("ffmpeg unavailable: {error}")))?;
    if !result.success() {
        return Err(EpisodeError(format!(
            "ffmpeg exited with status {result}"
        )));
    }
    Ok(())
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), EpisodeError> {
    let bytes = serde_json::to_vec_pretty(value).map_err(EpisodeError::json)?;
    fs::write(path, bytes).map_err(EpisodeError::io)
}

#[derive(Debug)]
pub struct EpisodeError(String);

impl EpisodeError {
    fn io(error: std::io::Error) -> Self {
        Self(error.to_string())
    }

    fn json(error: serde_json::Error) -> Self {
        Self(error.to_string())
    }

    fn academy(error: impl fmt::Display) -> Self {
        Self(error.to_string())
    }

    fn surface(error: impl fmt::Display) -> Self {
        Self(error.to_string())
    }
}

impl fmt::Display for EpisodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for EpisodeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn episode_classes_have_stable_labels() {
        assert_eq!(EpisodeClass::Development.label(), "Development");
        assert_eq!(EpisodeOutcome::LearnedResponse.label(), "Learned response");
    }
}
