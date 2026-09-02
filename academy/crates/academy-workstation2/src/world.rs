use crate::application::Application;
use crate::screen::{Touchscreen, CONTACT_DEPTH};
use crate::target::TargetApp;
use crate::{DeviceEvent, ScreenPoint};
use truelearner_workstation::{
    Eye, LightField, Point, WorkstationError, WorkstationState, WorldSample, BODY_MAX,
};

const RETINA_SIDE: usize = 9;
/// One receptor per 128 world units: a full-field retina. From the primary
/// position the eyes see the whole screen, matching the body course's world
/// and the harness's receptor-position arithmetic. A narrow view would hide
/// most of the screen from a centered gaze.
const VIEW_STEP: i32 = 128;
/// The hand's visual size in world units: an occluder about a quarter of a
/// receptor pitch across.
const HAND_RADIUS: i32 = 34;

/// The one application currently drawn on the screen. The organism sees
/// pixels; it never sees which application produced them.
#[derive(Clone, Debug, PartialEq, Eq)]
enum App {
    Keyboard(Application),
    Target(TargetApp),
}

impl App {
    fn frame(&self) -> LightField {
        match self {
            Self::Keyboard(app) => app.frame(),
            Self::Target(app) => app.frame(),
        }
    }

    fn apply(&mut self, events: &[DeviceEvent]) {
        match self {
            Self::Keyboard(app) => app.apply(events),
            Self::Target(app) => app.apply(events),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workstation2 {
    screen: Touchscreen,
    application: App,
}

impl Workstation2 {
    pub fn new(keyboard_shift: i16) -> Self {
        Self {
            screen: Touchscreen::new(CONTACT_DEPTH),
            application: App::Keyboard(Application::new(keyboard_shift)),
        }
    }

    pub fn with_target(app: TargetApp) -> Self {
        Self {
            screen: Touchscreen::new(CONTACT_DEPTH),
            application: App::Target(app),
        }
    }

    /// The same target app with the screen placed at `contact_depth`, the
    /// big-toy exposure knob: a close screen presents the contact
    /// consequence within the palm's easy reach, exactly like a toy placed
    /// within a baby's reach. Nothing else changes.
    pub fn with_target_at_depth(app: TargetApp, contact_depth: i16) -> Self {
        Self {
            screen: Touchscreen::new(contact_depth),
            application: App::Target(app),
        }
    }

    pub fn target(&self) -> Option<&TargetApp> {
        match &self.application {
            App::Target(app) => Some(app),
            App::Keyboard(_) => None,
        }
    }

    pub fn sense(&self, body: &WorkstationState) -> Result<WorldSample, WorkstationError> {
        let display = self.application.frame();
        let eyes = [
            render_eye(&display, body, Eye::Left)?,
            render_eye(&display, body, Eye::Right)?,
        ];
        WorldSample::new(eyes, self.screen.contacts(body))
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
            App::Target(_) => "",
        }
    }

    pub const fn scale(&self) -> i16 {
        match &self.application {
            App::Keyboard(app) => app.scale(),
            App::Target(_) => 0,
        }
    }
}

fn render_eye(
    display: &LightField,
    body: &WorkstationState,
    eye: Eye,
) -> Result<LightField, WorkstationError> {
    let gaze = body.eye(eye).gaze();
    let center = (RETINA_SIDE / 2) as i32;
    let mut pixels = Vec::with_capacity(RETINA_SIDE * RETINA_SIDE);
    for row in 0..RETINA_SIDE {
        for column in 0..RETINA_SIDE {
            let x = i32::from(gaze.x()) + (column as i32 - center) * VIEW_STEP;
            let y = i32::from(gaze.y()) + (row as i32 - center) * VIEW_STEP;
            if !(0..=i32::from(BODY_MAX)).contains(&x) || !(0..=i32::from(BODY_MAX)).contains(&y) {
                pixels.push(0);
                continue;
            }
            let point = Point::new(x as i16, y as i16)?;
            let mut value = display.sample(point);
            if hand_visible_at(
                body,
                eye,
                ScreenPoint {
                    x: x as i16,
                    y: y as i16,
                },
            ) {
                value = 8;
            }
            pixels.push(value);
        }
    }
    LightField::new(RETINA_SIDE as u16, RETINA_SIDE as u16, pixels)
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
