#![forbid(unsafe_code)]
//! ARC-AGI-3's external frame/action boundary, normalized for Academy.
//!
//! Game identifiers, action identifiers, scores, and terminal states remain
//! Academy evidence. Only rasterized frames may later cross TrueLearner's
//! established physical input boundary.

use academy_core::VisualSurface;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

mod sensorimotor;
pub use sensorimotor::{
    dominant_palette, Arc3AgentCommand, Arc3AgentResponse, Arc3Sensorimotor,
    Arc3SensorimotorError, Arc3SensorimotorObservation, Arc3SensorimotorSnapshot,
};

pub const ARC3_FRAME_SIDE: usize = 64;
pub const ARC3_FRAME_PIXELS: usize = ARC3_FRAME_SIDE * ARC3_FRAME_SIDE;

pub const ARC3_PALETTE: [[u8; 4]; 16] = [
    [255, 255, 255, 255],
    [204, 204, 204, 255],
    [153, 153, 153, 255],
    [102, 102, 102, 255],
    [51, 51, 51, 255],
    [0, 0, 0, 255],
    [229, 58, 163, 255],
    [255, 123, 204, 255],
    [249, 60, 49, 255],
    [30, 147, 255, 255],
    [136, 216, 241, 255],
    [255, 220, 0, 255],
    [255, 133, 27, 255],
    [146, 18, 49, 255],
    [79, 204, 48, 255],
    [163, 86, 214, 255],
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc3Metadata {
    pub schema_version: u16,
    pub kind: String,
    pub game_id: String,
    pub toolkit_revision: String,
    pub seed: u64,
    pub available_actions: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc3Point {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc3Action {
    pub id: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Arc3Point>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc3Observation {
    pub schema_version: u16,
    pub kind: String,
    pub game_id: String,
    pub turn: u32,
    pub action: Option<Arc3Action>,
    pub state: String,
    pub levels_completed: u16,
    pub win_levels: u16,
    pub full_reset: bool,
    pub available_actions: Vec<u8>,
    pub frames: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arc3Recording {
    pub metadata: Arc3Metadata,
    pub observations: Vec<Arc3Observation>,
}

impl Arc3Recording {
    pub fn read_jsonl(path: &Path) -> Result<Self, Arc3Error> {
        let text = fs::read_to_string(path).map_err(Arc3Error::io)?;
        Self::parse_jsonl(&text)
    }

    pub fn parse_jsonl(text: &str) -> Result<Self, Arc3Error> {
        let mut metadata = None;
        let mut observations = Vec::new();
        for (line_index, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let value: serde_json::Value = serde_json::from_str(line)
                .map_err(|error| Arc3Error(format!("line {}: {error}", line_index + 1)))?;
            match value.get("kind").and_then(serde_json::Value::as_str) {
                Some("metadata") => {
                    if metadata.is_some() {
                        return Err(Arc3Error("duplicate metadata record".to_string()));
                    }
                    metadata = Some(
                        serde_json::from_value(value)
                            .map_err(|error| Arc3Error(error.to_string()))?,
                    );
                }
                Some("observation") => observations.push(
                    serde_json::from_value(value).map_err(|error| Arc3Error(error.to_string()))?,
                ),
                other => return Err(Arc3Error(format!("unsupported record kind {other:?}"))),
            }
        }
        let recording = Self {
            metadata: metadata.ok_or_else(|| Arc3Error("metadata record missing".to_string()))?,
            observations,
        };
        recording.validate()?;
        Ok(recording)
    }

    pub fn validate(&self) -> Result<(), Arc3Error> {
        if self.metadata.schema_version != 1 || self.metadata.kind != "metadata" {
            return Err(Arc3Error("unsupported metadata schema".to_string()));
        }
        if self.observations.is_empty() {
            return Err(Arc3Error("recording contains no observations".to_string()));
        }
        for (index, observation) in self.observations.iter().enumerate() {
            if observation.schema_version != 1 || observation.kind != "observation" {
                return Err(Arc3Error(format!(
                    "observation {index} has unsupported schema"
                )));
            }
            if observation.game_id != self.metadata.game_id {
                return Err(Arc3Error(format!(
                    "observation {index} changes game identity"
                )));
            }
            if observation.turn != u32::try_from(index).unwrap_or(u32::MAX) {
                return Err(Arc3Error(format!(
                    "observation {index} has a noncanonical turn"
                )));
            }
            for action in &observation.available_actions {
                if !(1..=7).contains(action) {
                    return Err(Arc3Error(format!(
                        "observation {index} exposes action {action}"
                    )));
                }
            }
            if let Some(action) = observation.action {
                validate_action(action)?;
            }
            if observation.frames.is_empty() {
                return Err(Arc3Error(format!("observation {index} has no frame")));
            }
            for frame in &observation.frames {
                validate_frame(frame)?;
            }
        }
        Ok(())
    }

    pub fn final_surface(&self, scale: u32) -> Result<VisualSurface, Arc3Error> {
        let observation = self
            .observations
            .last()
            .ok_or_else(|| Arc3Error("recording contains no observations".to_string()))?;
        let frame = observation
            .frames
            .last()
            .ok_or_else(|| Arc3Error("observation contains no frame".to_string()))?;
        frame_surface(frame, scale)
    }
}

pub fn frame_surface(frame: &[u8], scale: u32) -> Result<VisualSurface, Arc3Error> {
    validate_frame(frame)?;
    let scale = scale.max(1);
    let side = u32::try_from(ARC3_FRAME_SIDE)
        .map_err(|_| Arc3Error("frame dimensions overflow".to_string()))?
        .saturating_mul(scale);
    let mut surface = VisualSurface::new(side, side, ARC3_PALETTE[0]);
    for (index, color) in frame.iter().copied().enumerate() {
        let x = u32::try_from(index % ARC3_FRAME_SIDE).unwrap_or(0) * scale;
        let y = u32::try_from(index / ARC3_FRAME_SIDE).unwrap_or(0) * scale;
        for offset_y in 0..scale {
            for offset_x in 0..scale {
                surface.set_pixel(x + offset_x, y + offset_y, ARC3_PALETTE[usize::from(color)]);
            }
        }
    }
    Ok(surface)
}

fn validate_frame(frame: &[u8]) -> Result<(), Arc3Error> {
    if frame.len() != ARC3_FRAME_PIXELS {
        return Err(Arc3Error(format!(
            "ARC-AGI-3 frame has {} cells; expected {ARC3_FRAME_PIXELS}",
            frame.len()
        )));
    }
    if let Some(color) = frame.iter().copied().find(|color| *color > 15) {
        return Err(Arc3Error(format!(
            "ARC-AGI-3 color {color} is outside 0..15"
        )));
    }
    Ok(())
}

fn validate_action(action: Arc3Action) -> Result<(), Arc3Error> {
    if action.id > 7 {
        return Err(Arc3Error(format!(
            "ARC-AGI-3 action {} is outside 0..7",
            action.id
        )));
    }
    if action.id == 6 {
        let point = action
            .data
            .ok_or_else(|| Arc3Error("ACTION6 requires coordinates".to_string()))?;
        if point.x > 63 || point.y > 63 {
            return Err(Arc3Error(
                "ACTION6 coordinates are outside 0..63".to_string(),
            ));
        }
    } else if action.data.is_some() {
        return Err(Arc3Error("only ACTION6 accepts coordinates".to_string()));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arc3Error(String);

impl Arc3Error {
    fn io(error: std::io::Error) -> Self {
        Self(error.to_string())
    }
}

impl fmt::Display for Arc3Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Arc3Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_renders_a_normalized_recording() {
        let frame = vec![9_u8; ARC3_FRAME_PIXELS];
        let input = format!(
            "{{\"schema_version\":1,\"kind\":\"metadata\",\"game_id\":\"ls20\",\"toolkit_revision\":\"test\",\"seed\":205,\"available_actions\":[1]}}\n{{\"schema_version\":1,\"kind\":\"observation\",\"game_id\":\"ls20\",\"turn\":0,\"action\":null,\"state\":\"NOT_FINISHED\",\"levels_completed\":0,\"win_levels\":1,\"full_reset\":false,\"available_actions\":[1],\"frames\":{}}}\n",
            serde_json::to_string(&vec![frame]).expect("frame JSON")
        );
        let recording = Arc3Recording::parse_jsonl(&input).expect("valid recording");
        let surface = recording.final_surface(2).expect("rendered frame");
        assert_eq!((surface.width(), surface.height()), (128, 128));
        assert_eq!(&surface.rgba_pixels()[0..4], &ARC3_PALETTE[9]);
    }

    #[test]
    fn rejects_nonphysical_color_values() {
        let mut frame = vec![0_u8; ARC3_FRAME_PIXELS];
        frame[3] = 16;
        assert!(frame_surface(&frame, 1).is_err());
    }
}
