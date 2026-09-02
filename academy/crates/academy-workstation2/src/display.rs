use crate::ScreenPoint;

pub const DISPLAY_SIDE: u16 = 2048;
pub const ARC_VIEWPORT_SIDE: u16 = 1536;
pub const ARC_VIEWPORT_MARGIN: u16 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayPoint {
    pub x: u16,
    pub y: u16,
}

impl DisplayPoint {
    pub const fn new(x: u16, y: u16) -> Option<Self> {
        if x < DISPLAY_SIDE && y < DISPLAY_SIDE {
            Some(Self { x, y })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayRect {
    pub left: u16,
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
}

impl DisplayRect {
    pub const fn new(left: u16, top: u16, right: u16, bottom: u16) -> Option<Self> {
        if left < right && top < bottom && right <= DISPLAY_SIDE && bottom <= DISPLAY_SIDE {
            Some(Self {
                left,
                top,
                right,
                bottom,
            })
        } else {
            None
        }
    }

    pub const fn width(self) -> u16 {
        self.right - self.left
    }

    pub const fn height(self) -> u16 {
        self.bottom - self.top
    }

    pub const fn contains(self, point: DisplayPoint) -> bool {
        point.x >= self.left && point.x < self.right && point.y >= self.top && point.y < self.bottom
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Viewport {
    rect: DisplayRect,
    source_width: u16,
    source_height: u16,
}

impl Viewport {
    pub const fn new(rect: DisplayRect, source_width: u16, source_height: u16) -> Option<Self> {
        if source_width > 0
            && source_height > 0
            && source_width <= rect.width()
            && source_height <= rect.height()
        {
            Some(Self {
                rect,
                source_width,
                source_height,
            })
        } else {
            None
        }
    }

    pub const fn full(source_width: u16, source_height: u16) -> Option<Self> {
        let rect = DisplayRect {
            left: 0,
            top: 0,
            right: DISPLAY_SIDE,
            bottom: DISPLAY_SIDE,
        };
        Self::new(rect, source_width, source_height)
    }

    pub const fn arc() -> Self {
        Self {
            rect: DisplayRect {
                left: ARC_VIEWPORT_MARGIN,
                top: ARC_VIEWPORT_MARGIN,
                right: ARC_VIEWPORT_MARGIN + ARC_VIEWPORT_SIDE,
                bottom: ARC_VIEWPORT_MARGIN + ARC_VIEWPORT_SIDE,
            },
            source_width: 64,
            source_height: 64,
        }
    }

    pub const fn rect(self) -> DisplayRect {
        self.rect
    }

    pub const fn source_width(self) -> u16 {
        self.source_width
    }

    pub const fn source_height(self) -> u16 {
        self.source_height
    }

    pub const fn source_at(self, point: DisplayPoint) -> Option<(u16, u16)> {
        if !self.rect.contains(point) {
            return None;
        }
        let x =
            (point.x - self.rect.left) as u32 * self.source_width as u32 / self.rect.width() as u32;
        let y = (point.y - self.rect.top) as u32 * self.source_height as u32
            / self.rect.height() as u32;
        Some((x as u16, y as u16))
    }

    pub const fn display_rect_for_source(self, x: u16, y: u16) -> Option<DisplayRect> {
        if x >= self.source_width || y >= self.source_height {
            return None;
        }
        let left =
            self.rect.left as u32 + x as u32 * self.rect.width() as u32 / self.source_width as u32;
        let right = self.rect.left as u32
            + (x as u32 + 1) * self.rect.width() as u32 / self.source_width as u32;
        let top =
            self.rect.top as u32 + y as u32 * self.rect.height() as u32 / self.source_height as u32;
        let bottom = self.rect.top as u32
            + (y as u32 + 1) * self.rect.height() as u32 / self.source_height as u32;
        DisplayRect::new(left as u16, top as u16, right as u16, bottom as u16)
    }

    pub const fn source_at_screen(self, point: ScreenPoint) -> Option<(u16, u16)> {
        self.source_at(display_from_screen(point))
    }
}

pub const fn display_from_screen(point: ScreenPoint) -> DisplayPoint {
    DisplayPoint {
        x: double_body_coordinate(point.x),
        y: double_body_coordinate(point.y),
    }
}

const fn double_body_coordinate(value: i16) -> u16 {
    let bounded = if value < 0 {
        0
    } else if value > 1024 {
        1024
    } else {
        value
    } as u16;
    if bounded == 1024 {
        DISPLAY_SIDE - 1
    } else {
        bounded * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_cells_scale_to_exact_twenty_four_pixel_squares() {
        let viewport = Viewport::arc();
        for y in 0..64 {
            for x in 0..64 {
                let rect = viewport.display_rect_for_source(x, y).unwrap();
                assert_eq!((rect.width(), rect.height()), (24, 24));
                assert_eq!(
                    viewport.source_at(DisplayPoint {
                        x: rect.left,
                        y: rect.top
                    }),
                    Some((x, y))
                );
                assert_eq!(
                    viewport.source_at(DisplayPoint {
                        x: rect.right - 1,
                        y: rect.bottom - 1
                    }),
                    Some((x, y))
                );
            }
        }
    }

    #[test]
    fn every_arc_column_has_a_reachable_eight_unit_palm_position() {
        let viewport = Viewport::arc();
        for coordinate in 0..64_u16 {
            let lower = 128 + 12 * coordinate as i16;
            let palm = ((lower + 7) / 8) * 8;
            let source = viewport
                .source_at_screen(ScreenPoint { x: palm, y: palm })
                .unwrap();
            assert_eq!(source, (coordinate, coordinate));
        }
    }

    #[test]
    fn arc_point_mapping_round_trips_every_cell_and_corner() {
        let viewport = Viewport::arc();
        for y in 0..64_u16 {
            for x in 0..64_u16 {
                let rect = viewport.display_rect_for_source(x, y).unwrap();
                for point in [
                    DisplayPoint {
                        x: rect.left,
                        y: rect.top,
                    },
                    DisplayPoint {
                        x: rect.right - 1,
                        y: rect.top,
                    },
                    DisplayPoint {
                        x: rect.left,
                        y: rect.bottom - 1,
                    },
                    DisplayPoint {
                        x: rect.right - 1,
                        y: rect.bottom - 1,
                    },
                ] {
                    assert_eq!(viewport.source_at(point), Some((x, y)));
                }
            }
        }
        assert_eq!(viewport.source_at(DisplayPoint { x: 255, y: 256 }), None);
        assert_eq!(viewport.source_at(DisplayPoint { x: 1792, y: 1791 }), None);
    }

    #[test]
    fn a_source_cell_cannot_be_smaller_than_one_display_pixel() {
        let rect = DisplayRect::new(0, 0, 8, 8).unwrap();
        assert!(Viewport::new(rect, 9, 8).is_none());
        assert!(Viewport::new(rect, 8, 9).is_none());
    }
}
