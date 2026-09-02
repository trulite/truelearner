use serde::{Deserialize, Serialize};
use truelearner_workstation::{ContactSample, WorkstationState, BODY_MAX, TOUCH_SITES};

pub const CONTACT_DEPTH: i16 = 600;
const TOUCH_COUNT: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenPoint {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TouchId(u8);

impl TouchId {
    pub fn new(value: u8) -> Option<Self> {
        (usize::from(value) < TOUCH_COUNT).then_some(Self(value))
    }

    pub(crate) const fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceEvent {
    TouchStarted {
        touch: TouchId,
        at: ScreenPoint,
    },
    TouchMoved {
        touch: TouchId,
        from: ScreenPoint,
        to: ScreenPoint,
    },
    TouchEnded {
        touch: TouchId,
        at: ScreenPoint,
    },
}

/// One contact surface: the whole palm of the undifferentiated hand. The
/// screen sits at a configurable depth so a course can place it within
/// easy reach during development, like a toy placed close to a baby,
/// and at the ordinary distance for its probes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Touchscreen {
    active: Option<ScreenPoint>,
    contact_depth: i16,
}

impl Touchscreen {
    pub(crate) const fn new(contact_depth: i16) -> Self {
        Self {
            active: None,
            contact_depth,
        }
    }

    pub(crate) fn contacts(&self, body: &WorkstationState) -> [ContactSample; TOUCH_SITES] {
        let mut contacts = [ContactSample::default(); TOUCH_SITES];
        if in_contact(body, self.contact_depth) {
            contacts[0] = ContactSample::new(BODY_MAX as u16, 0).expect("bounded screen pressure");
        }
        contacts
    }

    pub(crate) fn advance(&mut self, body: &WorkstationState) -> Vec<DeviceEvent> {
        let touch = TouchId::new(0).expect("palm touch id");
        let next = in_contact(body, self.contact_depth).then(|| {
            let palm = body.hand().palm();
            ScreenPoint {
                x: palm.x(),
                y: palm.y(),
            }
        });
        let mut events = Vec::new();
        match (self.active, next) {
            (None, Some(at)) => events.push(DeviceEvent::TouchStarted { touch, at }),
            (Some(from), Some(to)) if from != to => {
                events.push(DeviceEvent::TouchMoved { touch, from, to });
            }
            (Some(at), None) => events.push(DeviceEvent::TouchEnded { touch, at }),
            _ => {}
        }
        self.active = next;
        events
    }
}

fn in_contact(body: &WorkstationState, contact_depth: i16) -> bool {
    body.hand().palm().depth() >= contact_depth
}
