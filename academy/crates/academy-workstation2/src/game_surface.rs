use crate::draw::{distance, Rect};
use crate::{BezelControl, DeviceEvent, ScreenPoint, TAP_TRAVEL};
use truelearner_workstation::{ColorField, Rgb, WorkstationError};

const SURFACE_SIDE: usize = 1024;
const CONTENT_MIN: i16 = 128;
const CONTENT_MAX: i16 = 896;
const BEZEL_DARK: u8 = 18;
const CONTROL_INERT: u8 = 64;
const CONTROL_ENABLED: u8 = 176;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Region {
    Content,
    Control(BezelControl),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TouchStart {
    at: ScreenPoint,
    current: ScreenPoint,
    travel: u32,
    region: Region,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct GameSurface {
    content: ColorField,
    point_enabled: bool,
    enabled: u8,
    rendered: ColorField,
    started: [Option<TouchStart>; 5],
}

impl GameSurface {
    pub(crate) fn new(
        content: ColorField,
        point_enabled: bool,
        enabled: &[BezelControl],
    ) -> Result<Self, WorkstationError> {
        let enabled = enabled_bits(enabled);
        let rendered = render(&content, enabled)?;
        Ok(Self {
            content,
            point_enabled,
            enabled,
            rendered,
            started: [None; 5],
        })
    }

    pub(crate) fn replace(
        &mut self,
        content: ColorField,
        point_enabled: bool,
        enabled: &[BezelControl],
    ) -> Result<(), WorkstationError> {
        let enabled = enabled_bits(enabled);
        let rendered = render(&content, enabled)?;
        self.content = content;
        self.point_enabled = point_enabled;
        self.enabled = enabled;
        self.rendered = rendered;
        Ok(())
    }

    pub(crate) fn frame(&self) -> ColorField {
        self.rendered.clone()
    }

    pub(crate) fn apply(&mut self, events: &[DeviceEvent]) -> Vec<DeviceEvent> {
        let mut completed = Vec::new();
        for event in events {
            match *event {
                DeviceEvent::TouchStarted { touch, at } => {
                    self.started[touch.index()] = self.region_at(at).map(|region| TouchStart {
                        at,
                        current: at,
                        travel: 0,
                        region,
                    });
                }
                DeviceEvent::TouchMoved { touch, from, to } => {
                    let region = self.region_at(to);
                    let Some(start) = &mut self.started[touch.index()] else {
                        continue;
                    };
                    if start.current != from || region != Some(start.region) {
                        self.started[touch.index()] = None;
                        continue;
                    }
                    start.travel = start.travel.saturating_add(path_distance(from, to));
                    start.current = to;
                }
                DeviceEvent::TouchEnded { touch, at } => {
                    let Some(start) = self.started[touch.index()].take() else {
                        continue;
                    };
                    let travel = start
                        .travel
                        .saturating_add(path_distance(start.current, at));
                    if travel > TAP_TRAVEL as u32
                        || distance(start.at, at) > TAP_TRAVEL
                        || self.region_at(at) != Some(start.region)
                    {
                        continue;
                    }
                    match start.region {
                        Region::Content => {
                            let (column, row) = self.content_point(at);
                            completed.push(DeviceEvent::ContentActivated { touch, column, row });
                        }
                        Region::Control(control) => {
                            completed.push(DeviceEvent::ControlActivated { touch, control });
                        }
                    }
                }
                DeviceEvent::ContentActivated { .. } | DeviceEvent::ControlActivated { .. } => {}
            }
        }
        completed
    }

    fn region_at(&self, point: ScreenPoint) -> Option<Region> {
        if self.point_enabled
            && (CONTENT_MIN..CONTENT_MAX).contains(&point.x)
            && (CONTENT_MIN..CONTENT_MAX).contains(&point.y)
        {
            return Some(Region::Content);
        }
        BezelControl::ALL.into_iter().find_map(|control| {
            (self.enabled & control.bit() != 0 && control_rect(control).contains(point))
                .then_some(Region::Control(control))
        })
    }

    fn content_point(&self, point: ScreenPoint) -> (u16, u16) {
        let scale = |value: i16, size: u16| {
            let offset = u32::from(value.saturating_sub(CONTENT_MIN) as u16);
            (offset * u32::from(size) / u32::from((CONTENT_MAX - CONTENT_MIN) as u16))
                .min(u32::from(size - 1)) as u16
        };
        (
            scale(point.x, self.content.width()),
            scale(point.y, self.content.height()),
        )
    }
}

fn enabled_bits(enabled: &[BezelControl]) -> u8 {
    enabled.iter().fold(0, |bits, control| bits | control.bit())
}

fn path_distance(from: ScreenPoint, to: ScreenPoint) -> u32 {
    u32::from(from.x.abs_diff(to.x)) + u32::from(from.y.abs_diff(to.y))
}

fn control_rect(control: BezelControl) -> Rect {
    match control {
        BezelControl::North => Rect {
            left: 464,
            top: 16,
            right: 560,
            bottom: 112,
        },
        BezelControl::South => Rect {
            left: 464,
            top: 912,
            right: 560,
            bottom: 1008,
        },
        BezelControl::West => Rect {
            left: 16,
            top: 464,
            right: 112,
            bottom: 560,
        },
        BezelControl::East => Rect {
            left: 912,
            top: 464,
            right: 1008,
            bottom: 560,
        },
        BezelControl::Primary => Rect {
            left: 160,
            top: 16,
            right: 256,
            bottom: 112,
        },
        BezelControl::Back => Rect {
            left: 160,
            top: 912,
            right: 256,
            bottom: 1008,
        },
        BezelControl::Reset => Rect {
            left: 768,
            top: 16,
            right: 864,
            bottom: 112,
        },
    }
}

fn render(content: &ColorField, enabled: u8) -> Result<ColorField, WorkstationError> {
    let mut pixels = vec![Rgb::gray(BEZEL_DARK); SURFACE_SIDE * SURFACE_SIDE];
    for y in CONTENT_MIN as usize..CONTENT_MAX as usize {
        let source_y = (y - CONTENT_MIN as usize) * usize::from(content.height())
            / (CONTENT_MAX - CONTENT_MIN) as usize;
        for x in CONTENT_MIN as usize..CONTENT_MAX as usize {
            let source_x = (x - CONTENT_MIN as usize) * usize::from(content.width())
                / (CONTENT_MAX - CONTENT_MIN) as usize;
            pixels[y * SURFACE_SIDE + x] =
                content.pixels()[source_y * usize::from(content.width()) + source_x];
        }
    }
    for control in BezelControl::ALL {
        let rect = control_rect(control);
        let value = if enabled & control.bit() != 0 {
            CONTROL_ENABLED
        } else {
            CONTROL_INERT
        };
        for y in rect.top as usize..rect.bottom as usize {
            pixels[y * SURFACE_SIDE + rect.left as usize..y * SURFACE_SIDE + rect.right as usize]
                .fill(Rgb::gray(value));
        }
    }
    ColorField::new(SURFACE_SIDE as u16, SURFACE_SIDE as u16, pixels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TouchId;

    fn content() -> ColorField {
        ColorField::new(
            2,
            2,
            vec![Rgb::gray(1), Rgb::gray(2), Rgb::gray(3), Rgb::gray(4)],
        )
        .unwrap()
    }

    fn touch() -> TouchId {
        TouchId::new(0).unwrap()
    }

    fn tap(surface: &mut GameSurface, at: ScreenPoint) -> Vec<DeviceEvent> {
        surface.apply(&[
            DeviceEvent::TouchStarted { touch: touch(), at },
            DeviceEvent::TouchEnded { touch: touch(), at },
        ])
    }

    #[test]
    fn enabled_order_and_duplicates_do_not_change_the_surface() {
        let left = GameSurface::new(
            content(),
            true,
            &[
                BezelControl::Back,
                BezelControl::Primary,
                BezelControl::Back,
            ],
        )
        .unwrap();
        let right = GameSurface::new(
            content(),
            true,
            &[BezelControl::Primary, BezelControl::Back],
        )
        .unwrap();
        assert_eq!(left.frame(), right.frame());
    }

    #[test]
    fn availability_changes_only_bezel_pixels() {
        let primary = GameSurface::new(content(), true, &[BezelControl::Primary]).unwrap();
        let back = GameSurface::new(content(), true, &[BezelControl::Back]).unwrap();
        let left = primary.frame();
        let right = back.frame();
        for y in 0..SURFACE_SIDE {
            for x in 0..SURFACE_SIDE {
                if (CONTENT_MIN as usize..CONTENT_MAX as usize).contains(&x)
                    && (CONTENT_MIN as usize..CONTENT_MAX as usize).contains(&y)
                {
                    assert_eq!(
                        left.pixels()[y * SURFACE_SIDE + x],
                        right.pixels()[y * SURFACE_SIDE + x]
                    );
                }
            }
        }
        assert_ne!(left, right);
    }

    #[test]
    fn enabled_control_and_content_emit_completed_activations() {
        let mut surface = GameSurface::new(content(), true, &[BezelControl::Primary]).unwrap();
        assert_eq!(
            tap(&mut surface, ScreenPoint { x: 208, y: 64 }),
            vec![DeviceEvent::ControlActivated {
                touch: touch(),
                control: BezelControl::Primary,
            }]
        );
        assert_eq!(
            tap(&mut surface, ScreenPoint { x: 895, y: 895 }),
            vec![DeviceEvent::ContentActivated {
                touch: touch(),
                column: 1,
                row: 1,
            }]
        );
    }

    #[test]
    fn disabled_and_cross_region_gestures_are_inert() {
        let mut surface = GameSurface::new(content(), true, &[]).unwrap();
        assert!(tap(&mut surface, ScreenPoint { x: 208, y: 64 }).is_empty());
        assert!(surface
            .apply(&[
                DeviceEvent::TouchStarted {
                    touch: touch(),
                    at: ScreenPoint { x: 128, y: 128 },
                },
                DeviceEvent::TouchEnded {
                    touch: touch(),
                    at: ScreenPoint { x: 127, y: 128 },
                },
            ])
            .is_empty());
        assert!(surface
            .apply(&[
                DeviceEvent::TouchStarted {
                    touch: touch(),
                    at: ScreenPoint { x: 512, y: 512 },
                },
                DeviceEvent::TouchMoved {
                    touch: touch(),
                    from: ScreenPoint { x: 512, y: 512 },
                    to: ScreenPoint { x: 544, y: 512 },
                },
                DeviceEvent::TouchMoved {
                    touch: touch(),
                    from: ScreenPoint { x: 544, y: 512 },
                    to: ScreenPoint { x: 512, y: 512 },
                },
                DeviceEvent::TouchEnded {
                    touch: touch(),
                    at: ScreenPoint { x: 512, y: 512 },
                },
            ])
            .is_empty());
    }
}
