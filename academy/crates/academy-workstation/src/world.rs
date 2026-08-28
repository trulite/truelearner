use crate::geometry::{KeyEffect, KeyId, WorldGeometry};
use crate::render::SceneRenderer;
use crate::{WorldError, KEY_COUNT};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use truelearner_workstation::{
    ContactSample, Digit, Eye, HandPoint, WorkstationState, WorldSample, BODY_MAX, TOUCH_SITES,
};

const SURFACE_DEPTH: i16 = 600;
const PRESS_DEPTH: i16 = 720;
const RELEASE_DEPTH: i16 = 660;
const MAX_TEXT: usize = 64;
const MAX_TAP_STEPS: u64 = 5;
const MAX_TAP_TRAVEL: u32 = 28;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenPoint {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TouchTrack {
    point: ScreenPoint,
    started_at: u64,
    travel: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceState {
    keys_down: BTreeSet<u16>,
    cursor: ScreenPoint,
    touch: Option<TouchTrack>,
    text: String,
    selected: bool,
    step: u64,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {
            keys_down: BTreeSet::new(),
            cursor: ScreenPoint { x: 512, y: 512 },
            touch: None,
            text: String::new(),
            selected: false,
            step: 0,
        }
    }
}

impl DeviceState {
    pub fn keys_down(&self) -> impl Iterator<Item = KeyId> + '_ {
        self.keys_down.iter().copied().map(KeyId)
    }

    pub const fn cursor(&self) -> ScreenPoint {
        self.cursor
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub const fn selected(&self) -> bool {
        self.selected
    }

    pub const fn step(&self) -> u64 {
        self.step
    }

    pub const fn touching(&self) -> bool {
        self.touch.is_some()
    }

    pub(crate) fn validate(&self) -> Result<(), WorldError> {
        if self
            .keys_down
            .iter()
            .any(|key| usize::from(*key) >= KEY_COUNT)
            || !(0..=BODY_MAX).contains(&self.cursor.x)
            || !(0..=BODY_MAX).contains(&self.cursor.y)
            || self.text.chars().count() > MAX_TEXT
        {
            return Err(WorldError::InvalidState);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum DeviceEvent {
    KeyPressed { key: u16 },
    KeyReleased { key: u16 },
    TextChanged,
    CursorMoved { from: ScreenPoint, to: ScreenPoint },
    Clicked { selected: bool },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SurfacePoint {
    x: i16,
    y: i16,
    depth: i16,
}

impl From<HandPoint> for SurfacePoint {
    fn from(point: HandPoint) -> Self {
        Self {
            x: point.x(),
            y: point.y(),
            depth: point.depth(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SurfaceFrame {
    palm: SurfacePoint,
    tips: [SurfacePoint; 5],
}

impl SurfaceFrame {
    fn from_body(body: &WorkstationState) -> Self {
        Self {
            palm: body.hand().palm().into(),
            tips: Digit::ALL.map(|digit| body.hand().fingertip(digit).into()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorkstationWorld {
    geometry: WorldGeometry,
    device: DeviceState,
    renderer: SceneRenderer,
}

impl PartialEq for WorkstationWorld {
    fn eq(&self, other: &Self) -> bool {
        self.geometry == other.geometry
            && self.device == other.device
            && self.renderer.asset_digest() == other.renderer.asset_digest()
    }
}

impl Eq for WorkstationWorld {}

impl WorkstationWorld {
    pub fn new() -> Result<Self, WorldError> {
        Ok(Self {
            geometry: WorldGeometry::standard_ansi_104()?,
            device: DeviceState::default(),
            renderer: SceneRenderer::new()?,
        })
    }

    pub fn geometry(&self) -> &WorldGeometry {
        &self.geometry
    }

    pub const fn device(&self) -> &DeviceState {
        &self.device
    }

    pub fn asset_digest(&self) -> [u8; 32] {
        self.renderer.asset_digest()
    }

    pub fn sense(&self, body: &WorkstationState) -> Result<WorldSample, WorldError> {
        let contacts = self.contacts(SurfaceFrame::from_body(body))?;
        let eyes = [
            self.renderer
                .render(&self.geometry, &self.device, body, Eye::Left)?,
            self.renderer
                .render(&self.geometry, &self.device, body, Eye::Right)?,
        ];
        Ok(WorldSample::new(eyes, contacts)?)
    }

    pub fn advance(
        &mut self,
        before: &WorkstationState,
        after: &WorkstationState,
    ) -> Vec<DeviceEvent> {
        self.advance_frames(
            SurfaceFrame::from_body(before),
            SurfaceFrame::from_body(after),
        )
    }

    pub fn fingerprint(&self) -> Result<String, WorldError> {
        self.device.validate()?;
        let bytes = bincode::serialize(&self.device).map_err(|_| WorldError::InvalidState)?;
        let mut digest = Sha256::new();
        digest.update(b"truelearner-workstation-world-v1");
        digest.update(self.renderer.asset_digest());
        digest.update(bytes);
        Ok(digest
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }

    pub(crate) fn from_parts(
        device: DeviceState,
        expected_asset_digest: [u8; 32],
    ) -> Result<Self, WorldError> {
        device.validate()?;
        let world = Self::new()?;
        if world.asset_digest() != expected_asset_digest {
            return Err(WorldError::AssetDigest);
        }
        Ok(Self { device, ..world })
    }

    pub(crate) fn device_clone(&self) -> DeviceState {
        self.device.clone()
    }

    fn contacts(&self, frame: SurfaceFrame) -> Result<[ContactSample; TOUCH_SITES], WorldError> {
        let mut contacts = [ContactSample::default(); TOUCH_SITES];
        if self.on_surface(frame.palm) {
            contacts[0] = contact_sample(frame.palm.depth, 0)?;
        }
        for (index, tip) in frame.tips.into_iter().enumerate() {
            if self.on_surface(tip) {
                let slip = self
                    .device
                    .touch
                    .map(|touch| tip.x.saturating_sub(touch.point.x))
                    .unwrap_or(0);
                contacts[index + 1] = contact_sample(tip.depth, slip)?;
            }
        }
        Ok(contacts)
    }

    fn on_surface(&self, point: SurfacePoint) -> bool {
        point.depth >= SURFACE_DEPTH
            && (self.geometry.touchpad.contains_xy(point.x, point.y)
                || self
                    .geometry
                    .keys()
                    .iter()
                    .any(|key| key.rect.contains_xy(point.x, point.y)))
    }

    fn advance_frames(&mut self, before: SurfaceFrame, after: SurfaceFrame) -> Vec<DeviceEvent> {
        let mut events = Vec::new();
        let pressed = self.keys_at_depth(after, PRESS_DEPTH);
        let pressed_before = self.keys_at_depth(before, PRESS_DEPTH);
        let held = self.keys_at_depth(after, RELEASE_DEPTH);
        let prior = self.device.keys_down.clone();
        let mut next = prior.intersection(&held).copied().collect::<BTreeSet<_>>();
        next.extend(pressed.difference(&pressed_before).copied());

        for key in prior.difference(&next).copied().collect::<Vec<_>>() {
            events.push(DeviceEvent::KeyReleased { key });
        }
        let newly_pressed = next.difference(&prior).copied().collect::<Vec<_>>();
        for key in &newly_pressed {
            events.push(DeviceEvent::KeyPressed { key: *key });
        }
        self.device.keys_down = next;
        let shifted = self.device.keys_down.iter().any(|id| {
            self.geometry
                .key(KeyId(*id))
                .is_some_and(|key| key.label == "Shift")
        });
        let mut text_changed = false;
        for id in newly_pressed {
            if let Some(key) = self.geometry.key(KeyId(id)) {
                text_changed |= apply_key_effect(&mut self.device.text, key.effect, shifted);
            }
        }
        if text_changed {
            events.push(DeviceEvent::TextChanged);
        }

        let touch = after
            .tips
            .into_iter()
            .find(|tip| {
                tip.depth >= SURFACE_DEPTH && self.geometry.touchpad.contains_xy(tip.x, tip.y)
            })
            .map(|tip| ScreenPoint { x: tip.x, y: tip.y });
        match (self.device.touch, touch) {
            (None, Some(point)) => {
                self.device.touch = Some(TouchTrack {
                    point,
                    started_at: self.device.step,
                    travel: 0,
                });
            }
            (Some(mut track), Some(point)) => {
                let dx = point.x.saturating_sub(track.point.x);
                let dy = point.y.saturating_sub(track.point.y);
                let distance =
                    u32::from(dx.unsigned_abs()).saturating_add(u32::from(dy.unsigned_abs()));
                track.travel = track.travel.saturating_add(distance);
                if dx != 0 || dy != 0 {
                    let from = self.device.cursor;
                    self.device.cursor.x = add_screen(self.device.cursor.x, i32::from(dx) * 2);
                    self.device.cursor.y = add_screen(self.device.cursor.y, i32::from(dy) * 2);
                    events.push(DeviceEvent::CursorMoved {
                        from,
                        to: self.device.cursor,
                    });
                }
                track.point = point;
                self.device.touch = Some(track);
            }
            (Some(track), None) => {
                let duration = self.device.step.saturating_sub(track.started_at);
                if duration <= MAX_TAP_STEPS && track.travel <= MAX_TAP_TRAVEL {
                    self.device.selected = !self.device.selected;
                    events.push(DeviceEvent::Clicked {
                        selected: self.device.selected,
                    });
                }
                self.device.touch = None;
            }
            (None, None) => {}
        }
        self.device.step = self.device.step.saturating_add(1);
        events
    }

    fn keys_at_depth(&self, frame: SurfaceFrame, depth: i16) -> BTreeSet<u16> {
        frame
            .tips
            .into_iter()
            .filter(|tip| tip.depth >= depth)
            .filter_map(|tip| {
                self.geometry
                    .keys()
                    .iter()
                    .find(|key| key.rect.contains_xy(tip.x, tip.y))
                    .map(|key| key.id.0)
            })
            .collect()
    }

    #[cfg(test)]
    fn set_device_for_test(&mut self, state: DeviceState) {
        self.device = state;
    }

    #[cfg(test)]
    fn advance_surface_for_test(
        &mut self,
        before: SurfaceFrame,
        after: SurfaceFrame,
    ) -> Vec<DeviceEvent> {
        self.advance_frames(before, after)
    }
}

fn contact_sample(depth: i16, slip: i16) -> Result<ContactSample, WorldError> {
    let pressure = depth
        .saturating_sub(SURFACE_DEPTH)
        .unsigned_abs()
        .saturating_add(1)
        .min(BODY_MAX as u16);
    Ok(ContactSample::new(
        pressure,
        slip.clamp(-BODY_MAX, BODY_MAX),
    )?)
}

fn apply_key_effect(text: &mut String, effect: KeyEffect, shifted: bool) -> bool {
    match effect {
        KeyEffect::Character(mut character) => {
            if text.chars().count() >= MAX_TEXT {
                return false;
            }
            if shifted && character.is_ascii_alphabetic() {
                character = character.to_ascii_uppercase();
            }
            text.push(character);
            true
        }
        KeyEffect::Backspace => text.pop().is_some(),
        KeyEffect::Enter => {
            if text.chars().count() >= MAX_TEXT {
                false
            } else {
                text.push('\n');
                true
            }
        }
        KeyEffect::None => false,
    }
}

fn add_screen(value: i16, delta: i32) -> i16 {
    i16::try_from((i32::from(value) + delta).clamp(0, i32::from(BODY_MAX))).unwrap_or(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_frame() -> SurfaceFrame {
        SurfaceFrame {
            palm: SurfacePoint::default(),
            tips: [SurfacePoint::default(); 5],
        }
    }

    #[test]
    fn hovering_does_not_press_but_crossing_depth_does() {
        let mut world = WorkstationWorld::new().unwrap();
        let key = world
            .geometry()
            .keys()
            .iter()
            .find(|key| key.label == "A")
            .unwrap();
        let mut hover = empty_frame();
        hover.tips[1] = SurfacePoint {
            x: key.rect.x + 2,
            y: key.rect.y + 2,
            depth: PRESS_DEPTH - 1,
        };
        assert!(world
            .advance_surface_for_test(empty_frame(), hover)
            .is_empty());
        let mut pressed = hover;
        pressed.tips[1].depth = PRESS_DEPTH;
        let events = world.advance_surface_for_test(hover, pressed);
        assert!(events
            .iter()
            .any(|event| matches!(event, DeviceEvent::KeyPressed { .. })));
        assert_eq!(world.device().text(), "a");
    }

    #[test]
    fn key_hysteresis_holds_then_releases() {
        let mut world = WorkstationWorld::new().unwrap();
        let key = world
            .geometry()
            .keys()
            .iter()
            .find(|key| key.label == "A")
            .unwrap();
        let mut pressed = empty_frame();
        pressed.tips[0] = SurfacePoint {
            x: key.rect.x + 2,
            y: key.rect.y + 2,
            depth: PRESS_DEPTH,
        };
        world.advance_surface_for_test(empty_frame(), pressed);
        let mut held = pressed;
        held.tips[0].depth = RELEASE_DEPTH;
        assert!(world
            .advance_surface_for_test(pressed, held)
            .iter()
            .all(|event| !matches!(event, DeviceEvent::KeyReleased { .. })));
        let mut released = held;
        released.tips[0].depth = RELEASE_DEPTH - 1;
        assert!(world
            .advance_surface_for_test(held, released)
            .iter()
            .any(|event| matches!(event, DeviceEvent::KeyReleased { .. })));
    }

    #[test]
    fn disjoint_simultaneous_key_presses_commute() {
        let geometry = WorldGeometry::standard_ansi_104().unwrap();
        let a = geometry.keys().iter().find(|key| key.label == "A").unwrap();
        let b = geometry.keys().iter().find(|key| key.label == "B").unwrap();
        let point = |key: &crate::Key| SurfacePoint {
            x: key.rect.x + 2,
            y: key.rect.y + 2,
            depth: PRESS_DEPTH,
        };
        let mut first_frame = empty_frame();
        first_frame.tips[0] = point(a);
        first_frame.tips[1] = point(b);
        let mut second_frame = empty_frame();
        second_frame.tips[0] = point(b);
        second_frame.tips[1] = point(a);
        let mut first = WorkstationWorld::new().unwrap();
        let mut second = WorkstationWorld::new().unwrap();
        assert_eq!(
            first.advance_surface_for_test(empty_frame(), first_frame),
            second.advance_surface_for_test(empty_frame(), second_frame)
        );
        assert_eq!(first.device(), second.device());
    }

    #[test]
    fn key_device_change_is_visible_in_the_following_render() {
        let mut world = WorkstationWorld::new().unwrap();
        let body = WorkstationState::default();
        let before = world.sense(&body).unwrap();
        let key = world
            .geometry()
            .keys()
            .iter()
            .find(|key| key.label == "A")
            .unwrap();
        let mut pressed = empty_frame();
        pressed.tips[0] = SurfacePoint {
            x: key.rect.x + 2,
            y: key.rect.y + 2,
            depth: PRESS_DEPTH,
        };
        world.advance_surface_for_test(empty_frame(), pressed);
        let after = world.sense(&body).unwrap();
        assert_ne!(before.eye(Eye::Left), after.eye(Eye::Left));
    }

    #[test]
    fn touchpad_requires_contact_and_actual_motion() {
        let mut world = WorkstationWorld::new().unwrap();
        let pad = world.geometry().touchpad;
        let mut contact = empty_frame();
        contact.tips[1] = SurfacePoint {
            x: pad.x + 20,
            y: pad.y + 20,
            depth: SURFACE_DEPTH,
        };
        assert!(world
            .advance_surface_for_test(empty_frame(), contact)
            .is_empty());
        assert!(world.advance_surface_for_test(contact, contact).is_empty());
        let mut moved = contact;
        moved.tips[1].x += 12;
        let events = world.advance_surface_for_test(contact, moved);
        assert!(events
            .iter()
            .any(|event| matches!(event, DeviceEvent::CursorMoved { .. })));
    }

    #[test]
    fn short_stationary_release_clicks_but_drag_release_does_not() {
        let pad = WorkstationWorld::new().unwrap().geometry().touchpad;
        let mut contact = empty_frame();
        contact.tips[0] = SurfacePoint {
            x: pad.x + 10,
            y: pad.y + 10,
            depth: SURFACE_DEPTH,
        };
        let mut tap = WorkstationWorld::new().unwrap();
        tap.advance_surface_for_test(empty_frame(), contact);
        let events = tap.advance_surface_for_test(contact, empty_frame());
        assert!(events
            .iter()
            .any(|event| matches!(event, DeviceEvent::Clicked { selected: true })));

        let mut drag = WorkstationWorld::new().unwrap();
        drag.advance_surface_for_test(empty_frame(), contact);
        let mut moved = contact;
        moved.tips[0].x += 40;
        drag.advance_surface_for_test(contact, moved);
        let events = drag.advance_surface_for_test(moved, empty_frame());
        assert!(!events
            .iter()
            .any(|event| matches!(event, DeviceEvent::Clicked { .. })));
    }

    #[test]
    fn invalid_device_state_is_rejected() {
        let mut world = WorkstationWorld::new().unwrap();
        let mut state = DeviceState::default();
        state.keys_down.insert(u16::MAX);
        world.set_device_for_test(state);
        assert_eq!(world.device().validate(), Err(WorldError::InvalidState));
    }
}
