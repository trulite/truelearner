use crate::geometry::{KeyEffect, KeyId, Rect, WorldGeometry};
use crate::render::SceneRenderer;
use crate::{WorldError, KEY_COUNT};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use truelearner_workstation::{
    BodyAxis, ContactSample, Digit, Eye, HandPoint, MotorEffect, WorkstationState,
    WorkstationStepObservation, WorldSample, BODY_MAX, TOUCH_SITES,
};

pub const CONTACT_DEPTH: i16 = 600;
pub const KEY_PRESS_DEPTH: i16 = 720;
pub const KEY_RELEASE_DEPTH: i16 = 660;
pub const LONG_PRESS_STEPS: u64 = 2;
const MAX_TEXT: usize = 64;
const MAX_TAP_STEPS: u64 = 5;
const MAX_TAP_TRAVEL: u32 = 28;

const MAX_MONITOR_PIXELS: usize = 640 * 360;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorFrame {
    width: u16,
    height: u16,
    pixels: Vec<u8>,
}

impl MonitorFrame {
    pub fn new(width: u16, height: u16, pixels: Vec<u8>) -> Result<Self, WorldError> {
        let expected = usize::from(width)
            .checked_mul(usize::from(height))
            .ok_or(WorldError::InvalidPresentation)?;
        if width == 0 || height == 0 || expected > MAX_MONITOR_PIXELS || pixels.len() != expected {
            return Err(WorldError::InvalidPresentation);
        }
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub const fn width(&self) -> u16 {
        self.width
    }

    pub const fn height(&self) -> u16 {
        self.height
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    fn valid(&self) -> bool {
        usize::from(self.width)
            .checked_mul(usize::from(self.height))
            .is_some_and(|expected| {
                self.width > 0
                    && self.height > 0
                    && expected <= MAX_MONITOR_PIXELS
                    && self.pixels.len() == expected
            })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkstationPresentation {
    illuminated_key: Option<KeyId>,
    monitor_glyph: Option<char>,
    monitor_frame: Option<MonitorFrame>,
}

impl WorkstationPresentation {
    pub fn with_illuminated_key(key: KeyId) -> Self {
        Self {
            illuminated_key: Some(key),
            monitor_glyph: None,
            monitor_frame: None,
        }
    }

    pub fn with_monitor_glyph(glyph: char) -> Self {
        Self {
            illuminated_key: None,
            monitor_glyph: Some(glyph),
            monitor_frame: None,
        }
    }

    pub fn with_monitor_frame(frame: MonitorFrame) -> Self {
        Self {
            illuminated_key: None,
            monitor_glyph: None,
            monitor_frame: Some(frame),
        }
    }

    pub const fn illuminated_key(&self) -> Option<KeyId> {
        self.illuminated_key
    }

    pub const fn monitor_glyph(&self) -> Option<char> {
        self.monitor_glyph
    }

    pub const fn monitor_frame(&self) -> Option<&MonitorFrame> {
        self.monitor_frame.as_ref()
    }

    pub(crate) fn validate(&self, geometry: &WorldGeometry) -> Result<(), WorldError> {
        let invalid_key = self
            .illuminated_key
            .is_some_and(|key| geometry.key(key).is_none());
        let invalid_glyph = self
            .monitor_glyph
            .is_some_and(|glyph| !glyph.is_ascii_graphic());
        let invalid_frame = self
            .monitor_frame
            .as_ref()
            .is_some_and(|frame| !frame.valid());
        if invalid_key
            || invalid_glyph
            || invalid_frame
            || self.monitor_glyph.is_some() && self.monitor_frame.is_some()
        {
            Err(WorldError::InvalidPresentation)
        } else {
            Ok(())
        }
    }
}

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
    drag_parent: Option<MotorEffect>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceState {
    keys_down: BTreeSet<u16>,
    key_started_at: BTreeMap<u16, u64>,
    long_pressed: BTreeSet<u16>,
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
            key_started_at: BTreeMap::new(),
            long_pressed: BTreeSet::new(),
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

    pub fn long_pressed_keys(&self) -> impl Iterator<Item = KeyId> + '_ {
        self.long_pressed.iter().copied().map(KeyId)
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
            || self
                .key_started_at
                .keys()
                .any(|key| !self.keys_down.contains(key))
            || self
                .keys_down
                .iter()
                .any(|key| !self.key_started_at.contains_key(key))
            || self
                .long_pressed
                .iter()
                .any(|key| !self.key_started_at.contains_key(key))
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
    LongPressActivated { key: u16 },
    KeyReleased { key: u16 },
    TextChanged,
    CursorMoved { from: ScreenPoint, to: ScreenPoint },
    DragEnded,
    ObjectMoved { from: ScreenPoint, to: ScreenPoint },
    ObjectDepthMoved { from: i16, to: i16 },
    PinchReleased,
    Clicked { selected: bool },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTransition {
    pub events: Vec<DeviceEvent>,
    pub boundary_parents: Vec<MotorEffect>,
    pub progress_parents: Vec<MotorEffect>,
}

impl WorldTransition {
    pub fn external(events: Vec<DeviceEvent>) -> Self {
        Self {
            events,
            boundary_parents: Vec::new(),
            progress_parents: Vec::new(),
        }
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PinchObject {
    grip_digit: Digit,
    thumb_zone: Rect,
    finger_zone: Rect,
    position: ScreenPoint,
    depth: i16,
    drag_parent: Option<MotorEffect>,
}

impl PinchObject {
    fn contact_site(self, digit: Digit, point: SurfacePoint) -> bool {
        match digit {
            Digit::Thumb => self.thumb_zone.contains_xy(point.x, point.y),
            candidate if candidate == self.grip_digit => {
                self.finger_zone.contains_xy(point.x, point.y)
            }
            _ => false,
        }
    }

    fn contact_pressure(self, digit: Digit, point: SurfacePoint) -> Option<u16> {
        self.contact_site(digit, point)
            .then_some(if digit == Digit::Thumb {
                1
            } else {
                BODY_MAX as u16
            })
    }

    fn grip_point(self, frame: SurfaceFrame) -> Option<ScreenPoint> {
        let thumb = frame.tips[digit_index(Digit::Thumb)];
        let finger = frame.tips[digit_index(self.grip_digit)];
        (self.contact_site(Digit::Thumb, thumb) && self.contact_site(self.grip_digit, finger))
            .then_some(ScreenPoint {
                x: midpoint(thumb.x, finger.x),
                y: midpoint(thumb.y, finger.y),
            })
    }

    fn advance(&mut self, before: SurfaceFrame, after: SurfaceFrame) -> Vec<DeviceEvent> {
        match (self.grip_point(before), self.grip_point(after)) {
            (Some(_), Some(_)) => {
                let before_thumb = before.tips[digit_index(Digit::Thumb)];
                let after_thumb = after.tips[digit_index(Digit::Thumb)];
                let before_finger = before.tips[digit_index(self.grip_digit)];
                let after_finger = after.tips[digit_index(self.grip_digit)];
                let thumb_delta = (
                    after_thumb.x.saturating_sub(before_thumb.x),
                    after_thumb.y.saturating_sub(before_thumb.y),
                    after_thumb.depth.saturating_sub(before_thumb.depth),
                );
                let finger_delta = (
                    after_finger.x.saturating_sub(before_finger.x),
                    after_finger.y.saturating_sub(before_finger.y),
                    after_finger.depth.saturating_sub(before_finger.depth),
                );
                if thumb_delta != finger_delta || thumb_delta == (0, 0, 0) {
                    return Vec::new();
                }
                let (dx, dy, depth_delta) = thumb_delta;
                let actual_dx = bounded_translation(
                    dx,
                    self.thumb_zone.x.min(self.finger_zone.x),
                    self.thumb_zone.right().max(self.finger_zone.right()),
                );
                let actual_dy = bounded_translation(
                    dy,
                    self.thumb_zone.y.min(self.finger_zone.y),
                    self.thumb_zone.bottom().max(self.finger_zone.bottom()),
                );
                let actual_depth = depth_delta.clamp(
                    self.depth.saturating_neg(),
                    BODY_MAX.saturating_sub(self.depth),
                );
                if actual_dx == 0 && actual_dy == 0 && actual_depth == 0 {
                    return Vec::new();
                }
                let from = self.position;
                let from_depth = self.depth;
                translate_rect(&mut self.thumb_zone, actual_dx, actual_dy);
                translate_rect(&mut self.finger_zone, actual_dx, actual_dy);
                self.position.x = self.position.x.saturating_add(actual_dx);
                self.position.y = self.position.y.saturating_add(actual_dy);
                self.depth = self.depth.saturating_add(actual_depth);
                let mut events = Vec::with_capacity(2);
                if from != self.position {
                    events.push(DeviceEvent::ObjectMoved {
                        from,
                        to: self.position,
                    });
                }
                if from_depth != self.depth {
                    events.push(DeviceEvent::ObjectDepthMoved {
                        from: from_depth,
                        to: self.depth,
                    });
                }
                events
            }
            (Some(_), None) => {
                self.drag_parent = None;
                vec![DeviceEvent::PinchReleased]
            }
            _ => Vec::new(),
        }
    }
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
    touchpad: Rect,
    touchpad_contact_depth: i16,
    touchpad_fixed_pressure: Option<u16>,
    keyboard_enabled: bool,
    pinch_object: Option<PinchObject>,
    device: DeviceState,
    presentation: WorkstationPresentation,
    renderer: SceneRenderer,
    key_press_depth: i16,
    key_release_depth: i16,
}

impl PartialEq for WorkstationWorld {
    fn eq(&self, other: &Self) -> bool {
        self.geometry == other.geometry
            && self.touchpad == other.touchpad
            && self.touchpad_contact_depth == other.touchpad_contact_depth
            && self.touchpad_fixed_pressure == other.touchpad_fixed_pressure
            && self.keyboard_enabled == other.keyboard_enabled
            && self.pinch_object == other.pinch_object
            && self.device == other.device
            && self.presentation == other.presentation
            && self.key_press_depth == other.key_press_depth
            && self.key_release_depth == other.key_release_depth
            && self.renderer.asset_digest() == other.renderer.asset_digest()
    }
}

impl Eq for WorkstationWorld {}

impl WorkstationWorld {
    pub fn new() -> Result<Self, WorldError> {
        Self::new_with_presentation(WorkstationPresentation::default())
    }

    pub fn new_with_key_depths(
        key_press_depth: i16,
        key_release_depth: i16,
    ) -> Result<Self, WorldError> {
        Self::new_with_presentation_and_key_depths(
            WorkstationPresentation::default(),
            key_press_depth,
            key_release_depth,
        )
    }

    /// Builds the same physical device with one uninterrupted contact pad and
    /// no active keys. Academy uses this to present contact motion without a
    /// button consequence repeatedly closing the preceding depth path.
    pub fn new_with_contact_pad(touchpad: Rect) -> Result<Self, WorldError> {
        Self::new_with_contact_patch(touchpad, CONTACT_DEPTH, BODY_MAX as u16)
    }

    /// Builds a light vertical touch patch. Crossing its x/y boundary produces
    /// tactile contact without pretending that it supports palm depth.
    pub fn new_with_side_contact_patch(touchpad: Rect) -> Result<Self, WorldError> {
        Self::new_with_contact_patch(touchpad, 0, 1)
    }

    /// Builds a supported movable object whose two independent contact zones
    /// can be held only by the thumb and one other digit. The thumb begins just
    /// outside its zone, so a real opposition crossing must complete the grip.
    pub fn new_with_pinch_object(
        body: &WorkstationState,
        grip_digit: Digit,
    ) -> Result<Self, WorldError> {
        if grip_digit == Digit::Thumb {
            return Err(WorldError::InvalidGeometry);
        }
        let thumb = body.hand().fingertip(Digit::Thumb);
        let finger = body.hand().fingertip(grip_digit);
        let thumb_zone = Rect {
            x: thumb.x().saturating_sub(48),
            y: thumb.y().saturating_sub(24),
            width: 48,
            height: 48,
        };
        let finger_zone = centered_rect(finger.x(), finger.y(), 32, 32)?;
        if !thumb_zone.valid() || !finger_zone.valid() {
            return Err(WorldError::InvalidGeometry);
        }
        let mut world = Self::new_with_contact_patch(finger_zone, 0, BODY_MAX as u16)?;
        world.pinch_object = Some(PinchObject {
            grip_digit,
            thumb_zone,
            finger_zone,
            position: ScreenPoint {
                x: midpoint(thumb_zone.x + thumb_zone.width / 2, finger.x()),
                y: midpoint(thumb.y(), finger.y()),
            },
            depth: midpoint(thumb.depth(), finger.depth()),
            drag_parent: None,
        });
        Ok(world)
    }

    fn new_with_contact_patch(
        touchpad: Rect,
        contact_depth: i16,
        contact_pressure: u16,
    ) -> Result<Self, WorldError> {
        if !touchpad.valid() {
            return Err(WorldError::InvalidGeometry);
        }
        if !(0..=BODY_MAX).contains(&contact_depth) {
            return Err(WorldError::InvalidGeometry);
        }
        let mut world = Self::new()?;
        world.touchpad = touchpad;
        world.touchpad_contact_depth = contact_depth;
        world.touchpad_fixed_pressure = Some(contact_pressure);
        world.keyboard_enabled = false;
        Ok(world)
    }

    pub fn new_with_presentation(
        presentation: WorkstationPresentation,
    ) -> Result<Self, WorldError> {
        Self::new_with_presentation_and_key_depths(presentation, KEY_PRESS_DEPTH, KEY_RELEASE_DEPTH)
    }

    fn new_with_presentation_and_key_depths(
        presentation: WorkstationPresentation,
        key_press_depth: i16,
        key_release_depth: i16,
    ) -> Result<Self, WorldError> {
        let geometry = WorldGeometry::standard_ansi_104()?;
        presentation.validate(&geometry)?;
        if !(CONTACT_DEPTH..key_press_depth).contains(&key_release_depth)
            || key_press_depth > BODY_MAX
        {
            return Err(WorldError::InvalidKeyDepths);
        }
        Ok(Self {
            touchpad: geometry.touchpad,
            touchpad_contact_depth: CONTACT_DEPTH,
            touchpad_fixed_pressure: None,
            keyboard_enabled: true,
            pinch_object: None,
            geometry,
            device: DeviceState::default(),
            presentation,
            renderer: SceneRenderer::new()?,
            key_press_depth,
            key_release_depth,
        })
    }

    pub fn geometry(&self) -> &WorldGeometry {
        &self.geometry
    }

    pub const fn device(&self) -> &DeviceState {
        &self.device
    }

    pub const fn presentation(&self) -> &WorkstationPresentation {
        &self.presentation
    }

    pub fn set_presentation(
        &mut self,
        presentation: WorkstationPresentation,
    ) -> Result<(), WorldError> {
        presentation.validate(&self.geometry)?;
        self.presentation = presentation;
        Ok(())
    }

    pub fn asset_digest(&self) -> [u8; 32] {
        self.renderer.asset_digest()
    }

    pub const fn key_press_depth(&self) -> i16 {
        self.key_press_depth
    }

    pub const fn key_release_depth(&self) -> i16 {
        self.key_release_depth
    }

    pub const fn pinch_object_position(&self) -> Option<ScreenPoint> {
        match self.pinch_object {
            Some(object) => Some(object.position),
            None => None,
        }
    }

    pub const fn pinch_object_depth(&self) -> Option<i16> {
        match self.pinch_object {
            Some(object) => Some(object.depth),
            None => None,
        }
    }

    pub fn sense(&self, body: &WorkstationState) -> Result<WorldSample, WorldError> {
        let contacts = self.contact_samples(body)?;
        let eyes = [
            self.renderer.render(
                &self.geometry,
                &self.device,
                &self.presentation,
                body,
                Eye::Left,
            )?,
            self.renderer.render(
                &self.geometry,
                &self.device,
                &self.presentation,
                body,
                Eye::Right,
            )?,
        ];
        Ok(WorldSample::new(eyes, contacts)?)
    }

    pub fn contact_samples(
        &self,
        body: &WorkstationState,
    ) -> Result<[ContactSample; TOUCH_SITES], WorldError> {
        self.contacts(SurfaceFrame::from_body(body))
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

    pub fn advance_observation(
        &mut self,
        observation: &WorkstationStepObservation,
    ) -> WorldTransition {
        let contact_change = match (
            self.contact_samples(&observation.state_before),
            self.contact_samples(&observation.state_after),
        ) {
            (Ok(before), Ok(after)) => Some((before, after)),
            _ => None,
        };
        let ending_drag_parent = self.device.touch.and_then(|touch| touch.drag_parent);
        let ending_pinch_parent = self.pinch_object.and_then(|object| object.drag_parent);
        let events = self.advance(&observation.state_before, &observation.state_after);
        let has_direct_boundary_effect = events.iter().any(|event| {
            matches!(
                event,
                DeviceEvent::KeyPressed { .. }
                    | DeviceEvent::KeyReleased { .. }
                    | DeviceEvent::TextChanged
                    | DeviceEvent::PinchReleased
                    | DeviceEvent::Clicked { .. }
            )
        });
        let cursor_moved = events
            .iter()
            .any(|event| matches!(event, DeviceEvent::CursorMoved { .. }));
        let drag_ended = events
            .iter()
            .any(|event| matches!(event, DeviceEvent::DragEnded));
        let object_moved = events.iter().any(|event| {
            matches!(
                event,
                DeviceEvent::ObjectMoved { .. } | DeviceEvent::ObjectDepthMoved { .. }
            )
        });
        let pinch_released = events
            .iter()
            .any(|event| matches!(event, DeviceEvent::PinchReleased));
        let changed_axes = observation
            .movements
            .iter()
            .filter(|movement| movement.changed)
            .map(|movement| movement.axis)
            .filter(|axis| is_hand_axis(*axis))
            .collect::<Vec<_>>();
        let transport_parents = observation
            .crossings
            .iter()
            .copied()
            .filter(|crossing| {
                changed_axes.contains(&crossing.control.axis())
                    && matches!(
                        crossing.control.axis(),
                        BodyAxis::PalmHorizontal | BodyAxis::PalmVertical | BodyAxis::PalmDepth
                    )
            })
            .collect::<Vec<_>>();
        let lateral_parents = transport_parents
            .iter()
            .copied()
            .filter(|parent| {
                matches!(
                    parent.control.axis(),
                    BodyAxis::PalmHorizontal | BodyAxis::PalmVertical
                )
            })
            .collect::<Vec<_>>();
        let depth_sensitive = self.touchpad_contact_depth > 0 || self.keyboard_enabled;
        let mut boundary_parents = if has_direct_boundary_effect {
            observation
                .crossings
                .iter()
                .copied()
                .filter(|crossing| changed_axes.contains(&crossing.control.axis()))
                .collect()
        } else {
            Vec::new()
        };
        if drag_ended {
            boundary_parents = match lateral_parents.as_slice() {
                [parent] => vec![*parent],
                _ => ending_drag_parent.into_iter().collect(),
            };
        } else if pinch_released {
            boundary_parents = match transport_parents.as_slice() {
                [parent] => vec![*parent],
                _ => ending_pinch_parent.into_iter().collect(),
            };
        }
        let mut progress_axes = contact_change
            .map(|(before, after)| {
                changed_axes
                    .iter()
                    .copied()
                    .filter(|axis| {
                        affected_contact_changed(*axis, &before, &after, depth_sensitive)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if cursor_moved || object_moved {
            progress_axes.extend(changed_axes.iter().copied().filter(|axis| {
                matches!(
                    axis,
                    BodyAxis::PalmHorizontal | BodyAxis::PalmVertical | BodyAxis::PalmDepth
                )
            }));
            if cursor_moved {
                if let [parent] = lateral_parents.as_slice() {
                    if let Some(touch) = &mut self.device.touch {
                        touch.drag_parent = Some(*parent);
                    }
                }
            }
            if object_moved {
                if let [parent] = transport_parents.as_slice() {
                    if let Some(object) = &mut self.pinch_object {
                        object.drag_parent = Some(*parent);
                    }
                }
            }
        }
        let progress_parents = observation
            .crossings
            .iter()
            .copied()
            .filter(|crossing| progress_axes.contains(&crossing.control.axis()))
            .collect();
        WorldTransition {
            events,
            boundary_parents,
            progress_parents,
        }
    }

    pub fn advance_external_key(
        &mut self,
        key: KeyId,
        pressed_before: bool,
        pressed_after: bool,
    ) -> Result<Vec<DeviceEvent>, WorldError> {
        let key = self.geometry.key(key).ok_or(WorldError::InvalidGeometry)?;
        let frame = |pressed| {
            let mut frame = SurfaceFrame {
                palm: SurfacePoint::default(),
                tips: [SurfacePoint::default(); 5],
            };
            if pressed {
                frame.tips[0] = SurfacePoint {
                    x: key.rect.x + key.rect.width / 2,
                    y: key.rect.y + key.rect.height / 2,
                    depth: self.key_press_depth,
                };
            }
            frame
        };
        Ok(self.advance_frames(frame(pressed_before), frame(pressed_after)))
    }

    pub fn fingerprint(&self) -> Result<String, WorldError> {
        self.device.validate()?;
        let bytes = bincode::serialize(&self.device).map_err(|_| WorldError::InvalidState)?;
        let mut digest = Sha256::new();
        digest.update(b"truelearner-workstation-world-v4");
        digest.update(self.renderer.asset_digest());
        digest.update(self.touchpad.x.to_le_bytes());
        digest.update(self.touchpad.y.to_le_bytes());
        digest.update(self.touchpad.width.to_le_bytes());
        digest.update(self.touchpad.height.to_le_bytes());
        digest.update(self.touchpad_contact_depth.to_le_bytes());
        digest.update([u8::from(self.touchpad_fixed_pressure.is_some())]);
        digest.update(self.touchpad_fixed_pressure.unwrap_or(0).to_le_bytes());
        digest.update([u8::from(self.keyboard_enabled)]);
        if let Some(object) = self.pinch_object {
            digest.update([
                1,
                u8::try_from(digit_index(object.grip_digit)).unwrap_or(u8::MAX),
            ]);
            for rect in [object.thumb_zone, object.finger_zone] {
                digest.update(rect.x.to_le_bytes());
                digest.update(rect.y.to_le_bytes());
                digest.update(rect.width.to_le_bytes());
                digest.update(rect.height.to_le_bytes());
            }
            digest.update(object.position.x.to_le_bytes());
            digest.update(object.position.y.to_le_bytes());
            digest.update(object.depth.to_le_bytes());
            digest.update(
                bincode::serialize(&object.drag_parent).map_err(|_| WorldError::InvalidState)?,
            );
        } else {
            digest.update([0]);
        }
        digest.update(self.key_press_depth.to_le_bytes());
        digest.update(self.key_release_depth.to_le_bytes());
        digest
            .update(bincode::serialize(&self.presentation).map_err(|_| WorldError::InvalidState)?);
        digest.update(bytes);
        Ok(digest
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }

    pub(crate) fn from_parts(
        device: DeviceState,
        presentation: WorkstationPresentation,
        expected_asset_digest: [u8; 32],
    ) -> Result<Self, WorldError> {
        device.validate()?;
        let world = Self::new_with_presentation(presentation)?;
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
            contacts[0] = self.contact_at(frame.palm, 0)?;
        }
        for (index, tip) in frame.tips.into_iter().enumerate() {
            let digit = Digit::ALL[index];
            if let Some(pressure) = self
                .pinch_object
                .and_then(|object| object.contact_pressure(digit, tip))
            {
                contacts[index + 1] = ContactSample::new(pressure, 0)?;
                continue;
            }
            if self.on_surface(tip) {
                let slip = self
                    .device
                    .touch
                    .map(|touch| tip.x.saturating_sub(touch.point.x))
                    .unwrap_or(0);
                contacts[index + 1] = self.contact_at(tip, slip)?;
            }
        }
        Ok(contacts)
    }

    fn on_surface(&self, point: SurfacePoint) -> bool {
        (point.depth >= self.touchpad_contact_depth && self.touchpad.contains_xy(point.x, point.y))
            || (point.depth >= CONTACT_DEPTH
                && self.keyboard_enabled
                && self
                    .geometry
                    .keys()
                    .iter()
                    .any(|key| key.rect.contains_xy(point.x, point.y)))
    }

    fn contact_at(&self, point: SurfacePoint, slip: i16) -> Result<ContactSample, WorldError> {
        if self.touchpad.contains_xy(point.x, point.y) {
            if let Some(pressure) = self.touchpad_fixed_pressure {
                return ContactSample::new(pressure, slip.clamp(-BODY_MAX, BODY_MAX))
                    .map_err(WorldError::from);
            }
            return contact_sample(point.depth, slip);
        }
        contact_sample(point.depth, slip)
    }

    fn advance_frames(&mut self, before: SurfaceFrame, after: SurfaceFrame) -> Vec<DeviceEvent> {
        let mut events = Vec::new();
        let pressed = self.keys_at_depth(after, self.key_press_depth);
        let pressed_before = self.keys_at_depth(before, self.key_press_depth);
        let held = self.keys_at_depth(after, self.key_release_depth);
        let prior = self.device.keys_down.clone();
        let mut next = prior.intersection(&held).copied().collect::<BTreeSet<_>>();
        next.extend(pressed.difference(&pressed_before).copied());

        for key in prior.difference(&next).copied().collect::<Vec<_>>() {
            events.push(DeviceEvent::KeyReleased { key });
            self.device.key_started_at.remove(&key);
            self.device.long_pressed.remove(&key);
        }
        let newly_pressed = next.difference(&prior).copied().collect::<Vec<_>>();
        for key in &newly_pressed {
            events.push(DeviceEvent::KeyPressed { key: *key });
            self.device.key_started_at.insert(*key, self.device.step);
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
        for key in &self.device.keys_down {
            let held_steps = self
                .device
                .key_started_at
                .get(key)
                .map(|started| self.device.step.saturating_sub(*started))
                .unwrap_or_default();
            if held_steps >= LONG_PRESS_STEPS && self.device.long_pressed.insert(*key) {
                events.push(DeviceEvent::LongPressActivated { key: *key });
            }
        }

        if let Some(object) = &mut self.pinch_object {
            events.extend(object.advance(before, after));
        } else {
            let touch = after
                .tips
                .into_iter()
                .find(|tip| {
                    tip.depth >= self.touchpad_contact_depth
                        && self.touchpad.contains_xy(tip.x, tip.y)
                })
                .map(|tip| ScreenPoint { x: tip.x, y: tip.y });
            match (self.device.touch, touch) {
                (None, Some(point)) => {
                    self.device.touch = Some(TouchTrack {
                        point,
                        started_at: self.device.step,
                        travel: 0,
                        drag_parent: None,
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
                    } else if track.travel > MAX_TAP_TRAVEL {
                        events.push(DeviceEvent::DragEnded);
                    }
                    self.device.touch = None;
                }
                (None, None) => {}
            }
        }
        self.device.step = self.device.step.saturating_add(1);
        events
    }

    fn keys_at_depth(&self, frame: SurfaceFrame, depth: i16) -> BTreeSet<u16> {
        if !self.keyboard_enabled {
            return BTreeSet::new();
        }
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

fn digit_index(digit: Digit) -> usize {
    Digit::ALL
        .iter()
        .position(|candidate| *candidate == digit)
        .expect("digit belongs to the fixed hand")
}

fn centered_rect(x: i16, y: i16, width: i16, height: i16) -> Result<Rect, WorldError> {
    let maximum_x = BODY_MAX + 1 - width;
    let maximum_y = BODY_MAX + 1 - height;
    let rect = Rect {
        x: x.saturating_sub(width / 2).clamp(0, maximum_x),
        y: y.saturating_sub(height / 2).clamp(0, maximum_y),
        width,
        height,
    };
    rect.valid()
        .then_some(rect)
        .ok_or(WorldError::InvalidGeometry)
}

fn midpoint(left: i16, right: i16) -> i16 {
    i16::try_from((i32::from(left) + i32::from(right)) / 2).unwrap_or(left)
}

fn bounded_translation(delta: i16, minimum: i16, maximum: i16) -> i16 {
    delta.clamp(
        minimum.saturating_neg(),
        (BODY_MAX + 1).saturating_sub(maximum),
    )
}

fn translate_rect(rect: &mut Rect, dx: i16, dy: i16) {
    rect.x = rect.x.saturating_add(dx);
    rect.y = rect.y.saturating_add(dy);
}

const fn is_hand_axis(axis: BodyAxis) -> bool {
    !matches!(
        axis,
        BodyAxis::EyeHorizontal { .. } | BodyAxis::EyeVertical { .. }
    )
}

fn affected_contact_changed(
    axis: BodyAxis,
    before: &[ContactSample; TOUCH_SITES],
    after: &[ContactSample; TOUCH_SITES],
    depth_sensitive: bool,
) -> bool {
    let changed = |site: usize| before[site] != after[site];
    match axis {
        BodyAxis::EyeHorizontal { .. } | BodyAxis::EyeVertical { .. } => false,
        BodyAxis::PalmHorizontal | BodyAxis::PalmVertical => before != after,
        BodyAxis::PalmDepth => depth_sensitive && before != after,
        BodyAxis::Wrist | BodyAxis::Spread => (1..TOUCH_SITES).any(changed),
        BodyAxis::ThumbOpposition => changed(1),
        BodyAxis::FingerFlexion { digit } => Digit::ALL
            .iter()
            .position(|candidate| *candidate == digit)
            .is_some_and(|index| changed(index + 1)),
    }
}

fn contact_sample(depth: i16, slip: i16) -> Result<ContactSample, WorldError> {
    let pressure = depth
        .saturating_sub(CONTACT_DEPTH)
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
    fn contact_progress_factorizes_by_the_actuator_component() {
        let before = [ContactSample::default(); TOUCH_SITES];
        let mut palm_after = before;
        palm_after[0] = ContactSample::new(1, 0).unwrap();
        assert!(affected_contact_changed(
            BodyAxis::PalmDepth,
            &before,
            &palm_after,
            true,
        ));
        assert!(!affected_contact_changed(
            BodyAxis::FingerFlexion { digit: Digit::Ring },
            &before,
            &palm_after,
            true,
        ));

        let mut thumb_after = before;
        thumb_after[1] = ContactSample::new(1, 0).unwrap();
        assert!(affected_contact_changed(
            BodyAxis::ThumbOpposition,
            &before,
            &thumb_after,
            false,
        ));
        assert!(!affected_contact_changed(
            BodyAxis::FingerFlexion { digit: Digit::Ring },
            &before,
            &thumb_after,
            false,
        ));
        assert!(!affected_contact_changed(
            BodyAxis::PalmDepth,
            &before,
            &thumb_after,
            false,
        ));
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
            depth: KEY_PRESS_DEPTH - 1,
        };
        assert!(world
            .advance_surface_for_test(empty_frame(), hover)
            .is_empty());
        let mut pressed = hover;
        pressed.tips[1].depth = KEY_PRESS_DEPTH;
        let events = world.advance_surface_for_test(hover, pressed);
        assert!(events
            .iter()
            .any(|event| matches!(event, DeviceEvent::KeyPressed { .. })));
        assert_eq!(world.device().text(), "a");
    }

    #[test]
    fn softer_key_changes_only_the_external_crossing_depth() {
        let key = WorldGeometry::standard_ansi_104()
            .unwrap()
            .keys()
            .iter()
            .find(|key| key.label == "A")
            .unwrap()
            .clone();
        let mut frame = empty_frame();
        frame.tips[0] = SurfacePoint {
            x: key.rect.x + 2,
            y: key.rect.y + 2,
            depth: 640,
        };
        let mut standard = WorkstationWorld::new().unwrap();
        assert!(standard
            .advance_surface_for_test(empty_frame(), frame)
            .iter()
            .all(|event| !matches!(event, DeviceEvent::KeyPressed { .. })));

        let mut practice = WorkstationWorld::new_with_key_depths(640, 608).unwrap();
        assert!(practice
            .advance_surface_for_test(empty_frame(), frame)
            .iter()
            .any(|event| matches!(event, DeviceEvent::KeyPressed { .. })));
        assert_eq!(practice.key_press_depth(), 640);
        assert_eq!(practice.key_release_depth(), 608);
        assert_ne!(
            practice.fingerprint().unwrap(),
            standard.fingerprint().unwrap()
        );
    }

    #[test]
    fn invalid_key_depths_fail_closed() {
        assert_eq!(
            WorkstationWorld::new_with_key_depths(CONTACT_DEPTH, CONTACT_DEPTH).unwrap_err(),
            WorldError::InvalidKeyDepths
        );
        assert_eq!(
            WorkstationWorld::new_with_key_depths(BODY_MAX + 1, KEY_RELEASE_DEPTH).unwrap_err(),
            WorldError::InvalidKeyDepths
        );
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
            depth: KEY_PRESS_DEPTH,
        };
        world.advance_surface_for_test(empty_frame(), pressed);
        let mut held = pressed;
        held.tips[0].depth = KEY_RELEASE_DEPTH;
        assert!(world
            .advance_surface_for_test(pressed, held)
            .iter()
            .all(|event| !matches!(event, DeviceEvent::KeyReleased { .. })));
        let mut released = held;
        released.tips[0].depth = KEY_RELEASE_DEPTH - 1;
        assert!(world
            .advance_surface_for_test(held, released)
            .iter()
            .any(|event| matches!(event, DeviceEvent::KeyReleased { .. })));
    }

    #[test]
    fn external_key_contact_uses_the_same_long_press_law() {
        let mut world = WorkstationWorld::new().unwrap();
        let key = world
            .geometry()
            .keys()
            .iter()
            .find(|key| key.label == "A")
            .unwrap()
            .id;

        let pressed = world.advance_external_key(key, false, true).unwrap();
        assert!(pressed.iter().any(
            |event| matches!(event, DeviceEvent::KeyPressed { key: pressed } if *pressed == key.0)
        ));
        let first_hold = world.advance_external_key(key, true, true).unwrap();
        assert!(first_hold
            .iter()
            .all(|event| !matches!(event, DeviceEvent::LongPressActivated { .. })));
        let activated = world.advance_external_key(key, true, true).unwrap();
        assert!(activated.iter().any(
            |event| matches!(event, DeviceEvent::LongPressActivated { key: activated } if *activated == key.0)
        ));
        assert_eq!(
            world.device().long_pressed_keys().collect::<Vec<_>>(),
            [key]
        );

        let released = world.advance_external_key(key, true, false).unwrap();
        assert!(released
            .iter()
            .any(|event| matches!(event, DeviceEvent::KeyReleased { key: released } if *released == key.0)));
        assert!(world.device().keys_down().next().is_none());
        assert!(world.device().long_pressed_keys().next().is_none());
    }

    #[test]
    fn long_press_activation_changes_visible_world_state() {
        let mut world = WorkstationWorld::new().unwrap();
        let key = world
            .geometry()
            .keys()
            .iter()
            .find(|key| key.label == "A")
            .unwrap()
            .id;
        let body = WorkstationState::default();

        world.advance_external_key(key, false, true).unwrap();
        world.advance_external_key(key, true, true).unwrap();
        let held = world.sense(&body).unwrap();
        world.advance_external_key(key, true, true).unwrap();
        let activated = world.sense(&body).unwrap();

        for eye in Eye::ALL {
            assert_ne!(held.eye(eye), activated.eye(eye));
        }
    }

    #[test]
    fn disjoint_simultaneous_key_presses_commute() {
        let geometry = WorldGeometry::standard_ansi_104().unwrap();
        let a = geometry.keys().iter().find(|key| key.label == "A").unwrap();
        let b = geometry.keys().iter().find(|key| key.label == "B").unwrap();
        let point = |key: &crate::Key| SurfacePoint {
            x: key.rect.x + 2,
            y: key.rect.y + 2,
            depth: KEY_PRESS_DEPTH,
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
            depth: KEY_PRESS_DEPTH,
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
            depth: CONTACT_DEPTH,
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
    fn academy_contact_patches_preserve_their_distinct_physics() {
        let patch = Rect {
            x: 300,
            y: 600,
            width: 80,
            height: 80,
        };
        let mut frame = empty_frame();
        frame.tips[0] = SurfacePoint {
            x: patch.x + 1,
            y: patch.y + 1,
            depth: CONTACT_DEPTH,
        };

        let rigid = WorkstationWorld::new_with_contact_pad(patch).unwrap();
        assert_eq!(
            rigid.contacts(frame).unwrap()[1].pressure(),
            BODY_MAX as u16
        );

        frame.tips[0].depth = 0;
        let side = WorkstationWorld::new_with_side_contact_patch(patch).unwrap();
        assert_eq!(side.contacts(frame).unwrap()[1].pressure(), 1);
        assert_ne!(rigid.fingerprint().unwrap(), side.fingerprint().unwrap());

        let key = rigid
            .geometry()
            .keys()
            .iter()
            .find(|key| key.label == "A")
            .unwrap();
        let mut key_frame = empty_frame();
        key_frame.tips[0] = SurfacePoint {
            x: key.rect.x + 1,
            y: key.rect.y + 1,
            depth: KEY_PRESS_DEPTH,
        };
        let mut isolated = rigid;
        assert!(isolated
            .advance_surface_for_test(empty_frame(), key_frame)
            .iter()
            .all(|event| !matches!(event, DeviceEvent::KeyPressed { .. })));
    }

    #[test]
    fn pinch_object_requires_two_contacts_and_coherent_transport() {
        let body = WorkstationState::default();
        let mut world = WorkstationWorld::new_with_pinch_object(&body, Digit::Ring).unwrap();
        let finger_only = SurfaceFrame::from_body(&body);
        let object = world.pinch_object.unwrap();
        assert!(!object.contact_site(Digit::Thumb, finger_only.tips[0]));
        assert!(object.contact_site(Digit::Ring, finger_only.tips[3]));

        let mut gripped = finger_only;
        gripped.tips[0].x = gripped.tips[0].x.saturating_sub(2);
        let contacts = world.contacts(gripped).unwrap();
        assert_eq!(contacts[1].pressure(), 1);
        assert_eq!(contacts[4].pressure(), BODY_MAX as u16);
        assert!(world
            .advance_frames(finger_only, gripped)
            .iter()
            .all(|event| !matches!(
                event,
                DeviceEvent::ObjectMoved { .. } | DeviceEvent::ObjectDepthMoved { .. }
            )));

        let mut carried = gripped;
        carried.tips[0].depth = carried.tips[0].depth.saturating_sub(16);
        carried.tips[3].depth = carried.tips[3].depth.saturating_sub(16);
        assert!(world.advance_frames(gripped, carried).iter().any(
            |event| matches!(event, DeviceEvent::ObjectDepthMoved { from, to } if from != to)
        ));

        let mut one_contact = carried;
        one_contact.tips[3].x = object.finger_zone.right().saturating_add(1);
        let mut one_contact_moved = one_contact;
        one_contact_moved.tips[0].depth = one_contact_moved.tips[0].depth.saturating_sub(16);
        one_contact_moved.tips[3].depth = one_contact_moved.tips[3].depth.saturating_sub(16);
        assert!(world
            .advance_frames(one_contact, one_contact_moved)
            .iter()
            .all(|event| !matches!(
                event,
                DeviceEvent::ObjectMoved { .. } | DeviceEvent::ObjectDepthMoved { .. }
            )));
    }

    #[test]
    fn thumb_alone_cannot_be_the_second_pinch_contact() {
        assert_eq!(
            WorkstationWorld::new_with_pinch_object(&WorkstationState::default(), Digit::Thumb)
                .unwrap_err(),
            WorldError::InvalidGeometry
        );
    }

    #[test]
    fn invalid_academy_contact_patch_fails_closed() {
        let invalid = Rect {
            x: BODY_MAX,
            y: BODY_MAX,
            width: 2,
            height: 2,
        };
        assert_eq!(
            WorkstationWorld::new_with_contact_pad(invalid).unwrap_err(),
            WorldError::InvalidGeometry
        );
        assert_eq!(
            WorkstationWorld::new_with_side_contact_patch(invalid).unwrap_err(),
            WorldError::InvalidGeometry
        );
    }

    #[test]
    fn short_stationary_release_clicks_but_drag_release_does_not() {
        let pad = WorkstationWorld::new().unwrap().geometry().touchpad;
        let mut contact = empty_frame();
        contact.tips[0] = SurfacePoint {
            x: pad.x + 10,
            y: pad.y + 10,
            depth: CONTACT_DEPTH,
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
        assert!(events
            .iter()
            .any(|event| matches!(event, DeviceEvent::DragEnded)));
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
