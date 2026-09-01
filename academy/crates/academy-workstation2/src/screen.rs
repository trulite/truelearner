use serde::{Deserialize, Serialize};
use truelearner_workstation::{ContactSample, Digit, WorkstationState, BODY_MAX, TOUCH_SITES};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Touchscreen {
    active: [Option<ScreenPoint>; TOUCH_COUNT],
}

impl Touchscreen {
    pub(crate) fn new() -> Self {
        Self {
            active: [None; TOUCH_COUNT],
        }
    }

    pub(crate) fn contacts(&self, body: &WorkstationState) -> [ContactSample; TOUCH_SITES] {
        let mut contacts = [ContactSample::default(); TOUCH_SITES];
        for (index, digit) in Digit::ALL.into_iter().enumerate() {
            if in_contact(body, digit) {
                contacts[index + 1] =
                    ContactSample::new(BODY_MAX as u16, 0).expect("bounded screen pressure");
            }
        }
        contacts
    }

    pub(crate) fn advance(&mut self, body: &WorkstationState) -> Vec<DeviceEvent> {
        let mut events = Vec::new();
        for (index, digit) in Digit::ALL.into_iter().enumerate() {
            let touch = TouchId::new(u8::try_from(index).expect("five digits")).expect("digit id");
            let next = in_contact(body, digit).then(|| {
                let tip = body.hand().fingertip(digit);
                ScreenPoint {
                    x: tip.x(),
                    y: tip.y(),
                }
            });
            match (self.active[index], next) {
                (None, Some(at)) => events.push(DeviceEvent::TouchStarted { touch, at }),
                (Some(from), Some(to)) if from != to => {
                    events.push(DeviceEvent::TouchMoved { touch, from, to });
                }
                (Some(at), None) => events.push(DeviceEvent::TouchEnded { touch, at }),
                _ => {}
            }
            self.active[index] = next;
        }
        events
    }
}

fn in_contact(body: &WorkstationState, digit: Digit) -> bool {
    body.hand().fingertip(digit).depth() >= CONTACT_DEPTH
}
