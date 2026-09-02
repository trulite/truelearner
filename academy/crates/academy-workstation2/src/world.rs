use crate::application::Application;
use crate::display::{display_from_screen, DisplayPoint, DisplayRect, Viewport, DISPLAY_SIDE};
use crate::screen::{Touchscreen, CONTACT_DEPTH};
use crate::target::TargetApp;
use crate::{DeviceEvent, ScreenPoint};
use truelearner_workstation::{
    Eye, LightField, VisualField, WorkstationError, WorkstationState, WorldSample,
    FOVEAL_VISION_SIDE, GLOBAL_CHANGE_SUBREGIONS, GLOBAL_VISION_FIELDS, GLOBAL_VISION_SIDE,
};

const GLOBAL_FIELD_SIDE: u16 = DISPLAY_SIDE / GLOBAL_VISION_SIDE as u16;
const GLOBAL_SUBREGION_SIDE: u16 = GLOBAL_FIELD_SIDE / 2;
const FOVEAL_PITCH: i32 = 8;
/// One Workstation2 observation window. This keeps a physical change present
/// while ordinary eye competition carries gaze to its 16x16 subregion.
const TRANSIENT_FRAMES: u8 = 32;
/// The hand's visual size in world units: an occluder about a quarter of a
/// receptor pitch across.
const HAND_RADIUS: i32 = 34;

/// The one application currently drawn on the screen. The organism sees
/// pixels; it never sees which application produced them.
#[derive(Clone, Debug, PartialEq, Eq)]
enum App {
    Keyboard(Application),
    Target(TargetApp),
    Pixels(LightField),
}

impl App {
    fn frame(&self) -> LightField {
        match self {
            Self::Keyboard(app) => app.frame(),
            Self::Target(app) => app.frame(),
            Self::Pixels(frame) => frame.clone(),
        }
    }

    fn apply(&mut self, events: &[DeviceEvent]) {
        match self {
            Self::Keyboard(app) => app.apply(events),
            Self::Target(app) => app.apply(events),
            Self::Pixels(_) => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workstation2 {
    screen: Touchscreen,
    application: App,
    viewport: Viewport,
    previous_frame: Option<LightField>,
    transient_lifetimes: Vec<u8>,
    transient_kinds: Vec<u8>,
}

impl Workstation2 {
    pub fn new(keyboard_shift: i16) -> Self {
        let application = App::Keyboard(Application::new(keyboard_shift));
        let frame = application.frame();
        Self {
            screen: Touchscreen::new(CONTACT_DEPTH),
            application,
            viewport: Viewport::full(frame.width(), frame.height()).expect("non-empty frame"),
            previous_frame: None,
            transient_lifetimes: vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
            transient_kinds: vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
        }
    }

    pub fn with_target(app: TargetApp) -> Self {
        let application = App::Target(app);
        let frame = application.frame();
        Self {
            screen: Touchscreen::new(CONTACT_DEPTH),
            application,
            viewport: Viewport::full(frame.width(), frame.height()).expect("non-empty frame"),
            previous_frame: None,
            transient_lifetimes: vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
            transient_kinds: vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
        }
    }

    /// Attach an ordinary pixels-only application to the touchscreen.
    pub fn with_pixels(frame: LightField) -> Self {
        let viewport = Viewport::full(frame.width(), frame.height()).expect("non-empty frame");
        Self {
            screen: Touchscreen::new(CONTACT_DEPTH),
            application: App::Pixels(frame),
            viewport,
            previous_frame: None,
            transient_lifetimes: vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
            transient_kinds: vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
        }
    }

    pub fn with_pixels_in_viewport(
        frame: LightField,
        viewport: Viewport,
    ) -> Result<Self, WorkstationError> {
        if frame.width() != viewport.source_width() || frame.height() != viewport.source_height() {
            return Err(WorkstationError::InvalidState);
        }
        Ok(Self {
            screen: Touchscreen::new(CONTACT_DEPTH),
            application: App::Pixels(frame),
            viewport,
            previous_frame: None,
            transient_lifetimes: vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
            transient_kinds: vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
        })
    }

    /// Replace the pixels drawn by an attached pixels-only application.
    pub fn replace_pixels(&mut self, frame: LightField) -> Result<(), WorkstationError> {
        match &mut self.application {
            App::Pixels(current) => {
                if frame.width() != self.viewport.source_width()
                    || frame.height() != self.viewport.source_height()
                {
                    return Err(WorkstationError::InvalidState);
                }
                *current = frame;
                Ok(())
            }
            App::Keyboard(_) | App::Target(_) => Err(WorkstationError::InvalidState),
        }
    }

    /// The same target app with the screen placed at `contact_depth`, the
    /// big-toy exposure knob: a close screen presents the contact
    /// consequence within the palm's easy reach, exactly like a toy placed
    /// within a baby's reach. Nothing else changes.
    pub fn with_target_at_depth(app: TargetApp, contact_depth: i16) -> Self {
        let application = App::Target(app);
        let frame = application.frame();
        Self {
            screen: Touchscreen::new(contact_depth),
            application,
            viewport: Viewport::full(frame.width(), frame.height()).expect("non-empty frame"),
            previous_frame: None,
            transient_lifetimes: vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
            transient_kinds: vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS],
        }
    }

    pub fn target(&self) -> Option<&TargetApp> {
        match &self.application {
            App::Target(app) => Some(app),
            App::Keyboard(_) | App::Pixels(_) => None,
        }
    }

    pub fn sense(&mut self, body: &WorkstationState) -> Result<WorldSample, WorkstationError> {
        let display = self.application.frame();
        let fresh_changes = render_changed(self.previous_frame.as_ref(), &display, self.viewport);
        let changed = retain_transients(
            &mut self.transient_lifetimes,
            &mut self.transient_kinds,
            &fresh_changes,
        );
        let eyes = [
            VisualField::new(
                render_global(&display, self.viewport, body, Eye::Left)?,
                changed.clone(),
                render_fovea(&display, self.viewport, body, Eye::Left)?,
            )?,
            VisualField::new(
                render_global(&display, self.viewport, body, Eye::Right)?,
                changed,
                render_fovea(&display, self.viewport, body, Eye::Right)?,
            )?,
        ];
        self.previous_frame = Some(display);
        WorldSample::new_visual(eyes, self.screen.contacts(body))
    }

    pub fn advance(&mut self, body: &WorkstationState) -> Vec<DeviceEvent> {
        let events = self.screen.advance(body);
        self.application.apply(&events);
        events
    }

    pub fn apply_device_events(&mut self, events: &[DeviceEvent]) {
        self.application.apply(events);
    }

    pub fn text(&self) -> &str {
        match &self.application {
            App::Keyboard(app) => app.text(),
            App::Target(_) | App::Pixels(_) => "",
        }
    }

    pub const fn scale(&self) -> i16 {
        match &self.application {
            App::Keyboard(app) => app.scale(),
            App::Target(_) | App::Pixels(_) => 0,
        }
    }
}

fn retain_transients(lifetimes: &mut [u8], kinds: &mut [u8], fresh: &[u8]) -> Vec<u8> {
    for ((lifetime, kind), changed) in lifetimes.iter_mut().zip(kinds.iter_mut()).zip(fresh) {
        *lifetime = if *changed != 0 {
            *kind = *changed;
            TRANSIENT_FRAMES
        } else {
            let remaining = lifetime.saturating_sub(1);
            if remaining == 0 {
                *kind = 0;
            }
            remaining
        };
    }
    kinds
        .iter()
        .zip(fresh)
        .map(|(kind, changed)| *kind | (u8::from(*changed != 0) * 4))
        .collect()
}

fn render_fovea(
    display: &LightField,
    viewport: Viewport,
    body: &WorkstationState,
    eye: Eye,
) -> Result<LightField, WorkstationError> {
    let gaze = body.eye(eye).gaze();
    let gaze = display_from_screen(ScreenPoint {
        x: gaze.x(),
        y: gaze.y(),
    });
    render_fovea_at(display, viewport, body, eye, gaze)
}

fn render_fovea_at(
    display: &LightField,
    viewport: Viewport,
    body: &WorkstationState,
    eye: Eye,
    gaze: DisplayPoint,
) -> Result<LightField, WorkstationError> {
    let center = (FOVEAL_VISION_SIDE / 2) as i32;
    let mut pixels = Vec::with_capacity(FOVEAL_VISION_SIDE * FOVEAL_VISION_SIDE);
    for row in 0..FOVEAL_VISION_SIDE {
        for column in 0..FOVEAL_VISION_SIDE {
            let x = i32::from(gaze.x) + (column as i32 - center) * FOVEAL_PITCH;
            let y = i32::from(gaze.y) + (row as i32 - center) * FOVEAL_PITCH;
            if !(0..i32::from(DISPLAY_SIDE)).contains(&x)
                || !(0..i32::from(DISPLAY_SIDE)).contains(&y)
            {
                pixels.push(0);
                continue;
            }
            let point = DisplayPoint {
                x: x as u16,
                y: y as u16,
            };
            let mut value = display_value(display, viewport, point);
            let body_point = ScreenPoint {
                x: (point.x / 2) as i16,
                y: (point.y / 2) as i16,
            };
            if hand_visible_at(body, eye, body_point) {
                value = 8;
            }
            pixels.push(value);
        }
    }
    LightField::new(FOVEAL_VISION_SIDE as u16, FOVEAL_VISION_SIDE as u16, pixels)
}

fn render_global(
    display: &LightField,
    viewport: Viewport,
    body: &WorkstationState,
    eye: Eye,
) -> Result<LightField, WorkstationError> {
    let mut sums = [0_i64; GLOBAL_VISION_FIELDS];
    for source_y in 0..display.height() {
        for source_x in 0..display.width() {
            let rect = viewport
                .display_rect_for_source(source_x, source_y)
                .expect("source coordinate belongs to its viewport");
            let value = i64::from(source_value(display, source_x, source_y));
            add_rect_to_global_sums(&mut sums, rect, value);
        }
    }
    overlay_hand_on_global_sums(&mut sums, display, viewport, body, eye);
    let area = i64::from(GLOBAL_FIELD_SIDE) * i64::from(GLOBAL_FIELD_SIDE);
    let pixels = sums
        .into_iter()
        .map(|sum| u8::try_from((sum / area).clamp(0, 255)).expect("mean luminance is bounded"))
        .collect();
    LightField::new(GLOBAL_VISION_SIDE as u16, GLOBAL_VISION_SIDE as u16, pixels)
}

fn add_rect_to_global_sums(sums: &mut [i64; GLOBAL_VISION_FIELDS], rect: DisplayRect, value: i64) {
    let first_column = rect.left / GLOBAL_FIELD_SIDE;
    let last_column = (rect.right - 1) / GLOBAL_FIELD_SIDE;
    let first_row = rect.top / GLOBAL_FIELD_SIDE;
    let last_row = (rect.bottom - 1) / GLOBAL_FIELD_SIDE;
    for row in first_row..=last_row {
        for column in first_column..=last_column {
            let cell_left = column * GLOBAL_FIELD_SIDE;
            let cell_top = row * GLOBAL_FIELD_SIDE;
            let overlap_width =
                rect.right.min(cell_left + GLOBAL_FIELD_SIDE) - rect.left.max(cell_left);
            let overlap_height =
                rect.bottom.min(cell_top + GLOBAL_FIELD_SIDE) - rect.top.max(cell_top);
            let index = usize::from(row) * GLOBAL_VISION_SIDE + usize::from(column);
            sums[index] += value * i64::from(overlap_width) * i64::from(overlap_height);
        }
    }
}

fn overlay_hand_on_global_sums(
    sums: &mut [i64; GLOBAL_VISION_FIELDS],
    display: &LightField,
    viewport: Viewport,
    body: &WorkstationState,
    eye: Eye,
) {
    let palm = body.hand().palm();
    let disparity = palm.depth() / 18;
    let projected_x = match eye {
        Eye::Left => palm.x().saturating_sub(disparity),
        Eye::Right => palm.x().saturating_add(disparity),
    };
    let left = i32::from(projected_x).saturating_sub(HAND_RADIUS).max(0) * 2;
    let right = (i32::from(projected_x) + HAND_RADIUS + 1).clamp(0, 1024) * 2;
    let top = i32::from(palm.y()).saturating_sub(HAND_RADIUS).max(0) * 2;
    let bottom = (i32::from(palm.y()) + HAND_RADIUS + 1).clamp(0, 1024) * 2;
    for y in top..bottom {
        for x in left..right {
            let point = DisplayPoint {
                x: x as u16,
                y: y as u16,
            };
            let screen = ScreenPoint {
                x: (point.x / 2) as i16,
                y: (point.y / 2) as i16,
            };
            if !hand_visible_at(body, eye, screen) {
                continue;
            }
            let field = usize::from(point.y / GLOBAL_FIELD_SIDE) * GLOBAL_VISION_SIDE
                + usize::from(point.x / GLOBAL_FIELD_SIDE);
            sums[field] += 8 - i64::from(display_value(display, viewport, point));
        }
    }
}

fn render_changed(
    previous: Option<&LightField>,
    current: &LightField,
    viewport: Viewport,
) -> Vec<u8> {
    let mut changed = vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS];
    let Some(previous) = previous else {
        return changed;
    };
    if previous.width() != current.width() || previous.height() != current.height() {
        changed.fill(3);
        return changed;
    }
    for y in 0..current.height() {
        for x in 0..current.width() {
            if source_value(previous, x, y) == source_value(current, x, y) {
                continue;
            }
            let rect = viewport
                .display_rect_for_source(x, y)
                .expect("source coordinate belongs to its viewport");
            let kind = if source_value(current, x, y) > source_value(previous, x, y) {
                2
            } else {
                1
            };
            mark_changed_rect(&mut changed, rect, kind);
        }
    }
    changed
}

fn mark_changed_rect(changed: &mut [u8], rect: DisplayRect, kind: u8) {
    let first_column = rect.left / GLOBAL_SUBREGION_SIDE;
    let last_column = (rect.right - 1) / GLOBAL_SUBREGION_SIDE;
    let first_row = rect.top / GLOBAL_SUBREGION_SIDE;
    let last_row = (rect.bottom - 1) / GLOBAL_SUBREGION_SIDE;
    for row in first_row..=last_row {
        for column in first_column..=last_column {
            let field_row = row / 2;
            let field_column = column / 2;
            let subregion = usize::from(row % 2) * 2 + usize::from(column % 2);
            let field = usize::from(field_row) * GLOBAL_VISION_SIDE + usize::from(field_column);
            changed[field * GLOBAL_CHANGE_SUBREGIONS + subregion] |= kind;
        }
    }
}

fn display_value(display: &LightField, viewport: Viewport, point: DisplayPoint) -> u8 {
    viewport
        .source_at(point)
        .map_or(0, |(x, y)| source_value(display, x, y))
}

fn source_value(display: &LightField, x: u16, y: u16) -> u8 {
    display.pixels()[usize::from(y) * usize::from(display.width()) + usize::from(x)]
}

/// The hand occludes the screen: held between the eyes and the display, it
/// blocks light and renders as a dark silhouette (8) below every background
/// pixel (18..64) and below the salience floor (129). The learner sees it as
/// contrast, but the body's reflexes never chase it.
fn hand_visible_at(body: &WorkstationState, eye: Eye, sample: ScreenPoint) -> bool {
    let palm = body.hand().palm();
    projected_near(palm.x(), palm.y(), palm.depth(), eye, sample)
}

fn projected_near(x: i16, y: i16, depth: i16, eye: Eye, sample: ScreenPoint) -> bool {
    let disparity = depth / 18;
    let projected_x = match eye {
        Eye::Left => x.saturating_sub(disparity),
        Eye::Right => x.saturating_add(disparity),
    };
    i32::from(projected_x).abs_diff(i32::from(sample.x)) <= HAND_RADIUS as u32
        && i32::from(y).abs_diff(i32::from(sample.y)) <= HAND_RADIUS as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pixels(side: u16, value: u8) -> LightField {
        LightField::filled(side, side, value).unwrap()
    }

    #[test]
    fn every_display_pixel_has_one_global_field_and_subregion() {
        let mut visits = vec![0_u8; usize::from(DISPLAY_SIDE) * usize::from(DISPLAY_SIDE)];
        for field_row in 0..GLOBAL_VISION_SIDE as u16 {
            for field_column in 0..GLOBAL_VISION_SIDE as u16 {
                for sub_row in 0..2_u16 {
                    for sub_column in 0..2_u16 {
                        let left = (field_column * 2 + sub_column) * GLOBAL_SUBREGION_SIDE;
                        let top = (field_row * 2 + sub_row) * GLOBAL_SUBREGION_SIDE;
                        for y in top..top + GLOBAL_SUBREGION_SIDE {
                            for x in left..left + GLOBAL_SUBREGION_SIDE {
                                visits[usize::from(y) * usize::from(DISPLAY_SIDE)
                                    + usize::from(x)] += 1;
                            }
                        }
                    }
                }
            }
        }
        assert!(visits.into_iter().all(|count| count == 1));
    }

    #[test]
    fn equal_statistic_rearrangement_still_has_a_spatial_transient() {
        let viewport = Viewport::full(64, 64).unwrap();
        let mut before = vec![100_u8; 64 * 64];
        let mut after = before.clone();
        before[0] = 40;
        before[1] = 200;
        after[0] = 200;
        after[1] = 40;
        assert_eq!(before.iter().min(), after.iter().min());
        assert_eq!(before.iter().max(), after.iter().max());
        assert_eq!(
            before.iter().map(|value| u64::from(*value)).sum::<u64>(),
            after.iter().map(|value| u64::from(*value)).sum::<u64>()
        );
        let before = LightField::new(64, 64, before).unwrap();
        let after = LightField::new(64, 64, after).unwrap();

        let changed = render_changed(Some(&before), &after, viewport);

        assert!(changed.iter().any(|value| *value != 0));
        assert_eq!(changed.iter().filter(|value| **value != 0).count(), 1);
        assert_eq!(*changed.iter().find(|value| **value != 0).unwrap(), 3);
    }

    #[test]
    fn fovea_reads_nothing_beyond_the_screen() {
        let display = pixels(64, 230);
        let viewport = Viewport::full(64, 64).unwrap();
        let body = WorkstationState::default();
        let fovea = render_fovea_at(
            &display,
            viewport,
            &body,
            Eye::Left,
            DisplayPoint { x: 0, y: 0 },
        )
        .unwrap();
        assert_eq!(fovea.width(), FOVEAL_VISION_SIDE as u16);
        assert_eq!(fovea.pixels()[0], 0);
        assert_eq!(
            fovea.pixels()[FOVEAL_VISION_SIDE * FOVEAL_VISION_SIDE / 2],
            230
        );
        assert!(fovea.pixels().iter().all(|value| matches!(value, 0 | 230)));
    }

    #[test]
    fn spatial_transients_persist_for_the_geometric_orienting_bound() {
        let mut lifetimes = vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS];
        let mut kinds = vec![0; lifetimes.len()];
        let mut fresh = vec![0; lifetimes.len()];
        fresh[7] = 1;
        for remaining in (1..=TRANSIENT_FRAMES).rev() {
            let visible = retain_transients(&mut lifetimes, &mut kinds, &fresh);
            assert_eq!(lifetimes[7], remaining);
            assert_eq!(
                visible[7],
                if remaining == TRANSIENT_FRAMES { 5 } else { 1 }
            );
            fresh[7] = 0;
        }
        assert_eq!(retain_transients(&mut lifetimes, &mut kinds, &fresh)[7], 0);
    }

    #[test]
    fn repeated_change_in_one_retained_region_gets_a_new_fresh_frame() {
        let mut lifetimes = vec![0; GLOBAL_VISION_FIELDS * GLOBAL_CHANGE_SUBREGIONS];
        let mut kinds = vec![0; lifetimes.len()];
        let mut fresh = vec![0; lifetimes.len()];
        fresh[9] = 2;
        assert_eq!(retain_transients(&mut lifetimes, &mut kinds, &fresh)[9], 6);
        fresh[9] = 0;
        assert_eq!(retain_transients(&mut lifetimes, &mut kinds, &fresh)[9], 2);
        fresh[9] = 2;
        assert_eq!(retain_transients(&mut lifetimes, &mut kinds, &fresh)[9], 6);
    }

    #[test]
    fn global_means_and_fovea_sample_the_same_physical_raster() {
        let display = pixels(64, 230);
        let viewport = Viewport::full(64, 64).unwrap();
        let body = WorkstationState::default();
        let global = render_global(&display, viewport, &body, Eye::Left).unwrap();
        let mut sums = [0_u64; GLOBAL_VISION_FIELDS];
        for y in 0..DISPLAY_SIDE {
            for x in 0..DISPLAY_SIDE {
                let point = DisplayPoint { x, y };
                let screen = ScreenPoint {
                    x: (x / 2) as i16,
                    y: (y / 2) as i16,
                };
                let value = if hand_visible_at(&body, Eye::Left, screen) {
                    8
                } else {
                    display_value(&display, viewport, point)
                };
                let field = usize::from(y / GLOBAL_FIELD_SIDE) * GLOBAL_VISION_SIDE
                    + usize::from(x / GLOBAL_FIELD_SIDE);
                sums[field] += u64::from(value);
            }
        }
        let area = u64::from(GLOBAL_FIELD_SIDE) * u64::from(GLOBAL_FIELD_SIDE);
        assert_eq!(
            global.pixels(),
            sums.iter()
                .map(|sum| (sum / area) as u8)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn all_sixteen_palette_luminances_survive_arc_scaling_at_the_fovea() {
        let mut source = vec![0_u8; 64 * 64];
        for value in 0..16_u8 {
            source[10 * 64 + usize::from(value)] = value * 17;
        }
        let display = LightField::new(64, 64, source).unwrap();
        let viewport = Viewport::arc();
        let body = WorkstationState::default();
        for value in 0..16_u16 {
            let rect = viewport.display_rect_for_source(value, 10).unwrap();
            let gaze = DisplayPoint {
                x: rect.left + rect.width() / 2,
                y: rect.top + rect.height() / 2,
            };
            let fovea = render_fovea_at(&display, viewport, &body, Eye::Left, gaze).unwrap();
            assert_eq!(
                fovea.pixels()[FOVEAL_VISION_SIDE * FOVEAL_VISION_SIDE / 2],
                value as u8 * 17
            );
        }
    }

    #[test]
    fn dense_page_glyph_strokes_are_separable_at_thirty_two_display_pixels() {
        let mut source = vec![240_u8; 1024 * 1024];
        for y in 500..524 {
            for x in 480..516 {
                let local_x = if x < 496 {
                    Some(x - 480)
                } else if x >= 500 {
                    Some(x - 500)
                } else {
                    None
                };
                if local_x.is_some_and(|local_x| !(4..12).contains(&local_x)) {
                    source[y * 1024 + x] = 0;
                }
            }
        }
        let display = LightField::new(1024, 1024, source).unwrap();
        let viewport = Viewport::full(1024, 1024).unwrap();
        let body = WorkstationState::default();
        let fovea = render_fovea_at(
            &display,
            viewport,
            &body,
            Eye::Left,
            DisplayPoint { x: 1024, y: 1024 },
        )
        .unwrap();
        let row = &fovea.pixels()[8 * FOVEAL_VISION_SIDE..9 * FOVEAL_VISION_SIDE];
        assert!(row.windows(5).any(|window| window == [0, 240, 240, 0, 240]));
        assert!(row.windows(4).any(|window| window == [0, 240, 240, 0]));
    }
}
