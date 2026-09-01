use crate::application::Application;
use crate::screen::Touchscreen;
use crate::{DeviceEvent, ScreenPoint};
use truelearner_workstation::{
    Digit, Eye, LightField, Point, WorkstationError, WorkstationState, WorldSample, BODY_MAX,
};

const RETINA_SIDE: usize = 9;
const VIEW_STEP: i32 = 64;
const HAND_RADIUS: i32 = 34;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workstation2 {
    screen: Touchscreen,
    application: Application,
}

impl Workstation2 {
    pub fn new(keyboard_shift: i16) -> Self {
        Self {
            screen: Touchscreen::new(),
            application: Application::new(keyboard_shift),
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
        self.application.text()
    }

    pub const fn scale(&self) -> i16 {
        self.application.scale()
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
                value = 250;
            }
            pixels.push(value);
        }
    }
    LightField::new(RETINA_SIDE as u16, RETINA_SIDE as u16, pixels)
}

fn hand_visible_at(body: &WorkstationState, eye: Eye, sample: ScreenPoint) -> bool {
    let palm = body.hand().palm();
    projected_near(palm.x(), palm.y(), palm.depth(), eye, sample)
        || Digit::ALL.into_iter().any(|digit| {
            let tip = body.hand().fingertip(digit);
            projected_near(tip.x(), tip.y(), tip.depth(), eye, sample)
        })
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
