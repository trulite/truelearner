use crate::WorldError;
use serde::{Deserialize, Serialize};
use truelearner_workstation::{HandPoint, BODY_MAX};

pub const KEY_COUNT: usize = 104;
const KEY_HEIGHT: i16 = 42;
const GAP: i16 = 3;
const UNIT: i16 = 29;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct KeyId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
}

impl Rect {
    pub const fn right(self) -> i16 {
        self.x + self.width
    }

    pub const fn bottom(self) -> i16 {
        self.y + self.height
    }

    pub const fn contains_xy(self, x: i16, y: i16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    pub const fn contains_hand(self, point: HandPoint) -> bool {
        self.contains_xy(point.x(), point.y())
    }

    fn valid(self) -> bool {
        self.x >= 0
            && self.y >= 0
            && self.width > 0
            && self.height > 0
            && self.right() <= BODY_MAX + 1
            && self.bottom() <= BODY_MAX + 1
    }

    fn overlaps(self, other: Self) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum KeyEffect {
    Character(char),
    Backspace,
    Enter,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Key {
    pub id: KeyId,
    pub label: String,
    pub rect: Rect,
    pub(crate) effect: KeyEffect,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldGeometry {
    pub monitor: Rect,
    pub screen: Rect,
    pub keyboard: Rect,
    pub touchpad: Rect,
    keys: Vec<Key>,
}

impl WorldGeometry {
    pub fn standard_ansi_104() -> Result<Self, WorldError> {
        let mut keys = Vec::with_capacity(KEY_COUNT);
        add_function_row(&mut keys);
        add_main_rows(&mut keys);
        add_navigation(&mut keys);
        add_numpad(&mut keys);
        for (index, key) in keys.iter_mut().enumerate() {
            key.id = KeyId(u16::try_from(index).map_err(|_| WorldError::InvalidGeometry)?);
        }
        let geometry = Self {
            monitor: Rect {
                x: 100,
                y: 35,
                width: 800,
                height: 515,
            },
            screen: Rect {
                x: 120,
                y: 55,
                width: 760,
                height: 465,
            },
            keyboard: Rect {
                x: 18,
                y: 580,
                width: 820,
                height: 330,
            },
            touchpad: Rect {
                x: 850,
                y: 620,
                width: 150,
                height: 280,
            },
            keys,
        };
        geometry.validate()?;
        Ok(geometry)
    }

    pub fn keys(&self) -> &[Key] {
        &self.keys
    }

    pub fn key(&self, id: KeyId) -> Option<&Key> {
        self.keys.get(usize::from(id.0))
    }

    pub fn key_at(&self, point: HandPoint) -> Option<&Key> {
        self.keys.iter().find(|key| key.rect.contains_hand(point))
    }

    fn validate(&self) -> Result<(), WorldError> {
        if self.keys.len() != KEY_COUNT
            || !self.monitor.valid()
            || !self.screen.valid()
            || !self.keyboard.valid()
            || !self.touchpad.valid()
            || self.touchpad.overlaps(self.keyboard)
            || self.keys.iter().any(|key| !key.rect.valid())
        {
            return Err(WorldError::InvalidGeometry);
        }
        for (index, key) in self.keys.iter().enumerate() {
            if usize::from(key.id.0) != index
                || self
                    .keys
                    .iter()
                    .skip(index + 1)
                    .any(|other| key.rect.overlaps(other.rect))
            {
                return Err(WorldError::InvalidGeometry);
            }
        }
        Ok(())
    }
}

fn add_key(keys: &mut Vec<Key>, label: &str, rect: Rect) {
    keys.push(Key {
        id: KeyId(0),
        label: label.to_owned(),
        rect,
        effect: effect(label),
    });
}

fn effect(label: &str) -> KeyEffect {
    match label {
        "Back" => KeyEffect::Backspace,
        "Enter" => KeyEffect::Enter,
        "Space" => KeyEffect::Character(' '),
        value if value.chars().count() == 1 => {
            let character = value.chars().next().unwrap_or_default();
            if character.is_ascii_alphabetic() {
                KeyEffect::Character(character.to_ascii_lowercase())
            } else {
                KeyEffect::Character(character)
            }
        }
        _ => KeyEffect::None,
    }
}

fn add_function_row(keys: &mut Vec<Key>) {
    let labels = [
        "Esc", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "Prt",
        "Scr", "Pse",
    ];
    let mut x = 28;
    for (index, label) in labels.into_iter().enumerate() {
        add_key(
            keys,
            label,
            Rect {
                x,
                y: 590,
                width: 27,
                height: KEY_HEIGHT,
            },
        );
        x += 30;
        if matches!(index, 0 | 4 | 8 | 12) {
            x += 11;
        }
    }
}

fn add_main_rows(keys: &mut Vec<Key>) {
    let rows: [(&[(&str, u8)], i16); 5] = [
        (
            &[
                ("`", 4),
                ("1", 4),
                ("2", 4),
                ("3", 4),
                ("4", 4),
                ("5", 4),
                ("6", 4),
                ("7", 4),
                ("8", 4),
                ("9", 4),
                ("0", 4),
                ("-", 4),
                ("=", 4),
                ("Back", 8),
            ],
            650,
        ),
        (
            &[
                ("Tab", 6),
                ("Q", 4),
                ("W", 4),
                ("E", 4),
                ("R", 4),
                ("T", 4),
                ("Y", 4),
                ("U", 4),
                ("I", 4),
                ("O", 4),
                ("P", 4),
                ("[", 4),
                ("]", 4),
                ("\\", 6),
            ],
            700,
        ),
        (
            &[
                ("Caps", 7),
                ("A", 4),
                ("S", 4),
                ("D", 4),
                ("F", 4),
                ("G", 4),
                ("H", 4),
                ("J", 4),
                ("K", 4),
                ("L", 4),
                (";", 4),
                ("'", 4),
                ("Enter", 9),
            ],
            750,
        ),
        (
            &[
                ("Shift", 9),
                ("Z", 4),
                ("X", 4),
                ("C", 4),
                ("V", 4),
                ("B", 4),
                ("N", 4),
                ("M", 4),
                (",", 4),
                (".", 4),
                ("/", 4),
                ("Shift", 11),
            ],
            800,
        ),
        (
            &[
                ("Ctrl", 5),
                ("Win", 5),
                ("Alt", 5),
                ("Space", 25),
                ("Alt", 5),
                ("Win", 5),
                ("Menu", 5),
                ("Ctrl", 5),
            ],
            850,
        ),
    ];
    for (specs, y) in rows {
        let mut x = 28;
        for (label, quarter_units) in specs {
            let width = i16::from(*quarter_units) * UNIT / 4;
            add_key(
                keys,
                label,
                Rect {
                    x,
                    y,
                    width,
                    height: KEY_HEIGHT,
                },
            );
            x += width + GAP;
        }
    }
}

fn add_navigation(keys: &mut Vec<Key>) {
    for (row, labels) in [["Ins", "Home", "PgUp"], ["Del", "End", "PgDn"]]
        .into_iter()
        .enumerate()
    {
        for (column, label) in labels.into_iter().enumerate() {
            add_key(
                keys,
                label,
                Rect {
                    x: 525 + i16::try_from(column).unwrap_or(0) * 41,
                    y: 650 + i16::try_from(row).unwrap_or(0) * 50,
                    width: 38,
                    height: KEY_HEIGHT,
                },
            );
        }
    }
    add_key(
        keys,
        "Up",
        Rect {
            x: 566,
            y: 800,
            width: 38,
            height: KEY_HEIGHT,
        },
    );
    for (column, label) in ["Left", "Down", "Right"].into_iter().enumerate() {
        add_key(
            keys,
            label,
            Rect {
                x: 525 + i16::try_from(column).unwrap_or(0) * 41,
                y: 850,
                width: 38,
                height: KEY_HEIGHT,
            },
        );
    }
}

fn add_numpad(keys: &mut Vec<Key>) {
    let x = [665, 706, 747, 788];
    for (column, label) in ["Num", "/", "*", "-"].into_iter().enumerate() {
        add_key(
            keys,
            label,
            Rect {
                x: x[column],
                y: 650,
                width: 38,
                height: KEY_HEIGHT,
            },
        );
    }
    for (column, label) in ["7", "8", "9"].into_iter().enumerate() {
        add_key(
            keys,
            label,
            Rect {
                x: x[column],
                y: 700,
                width: 38,
                height: KEY_HEIGHT,
            },
        );
    }
    add_key(
        keys,
        "+",
        Rect {
            x: x[3],
            y: 700,
            width: 38,
            height: 92,
        },
    );
    for (column, label) in ["4", "5", "6"].into_iter().enumerate() {
        add_key(
            keys,
            label,
            Rect {
                x: x[column],
                y: 750,
                width: 38,
                height: KEY_HEIGHT,
            },
        );
    }
    for (column, label) in ["1", "2", "3"].into_iter().enumerate() {
        add_key(
            keys,
            label,
            Rect {
                x: x[column],
                y: 800,
                width: 38,
                height: KEY_HEIGHT,
            },
        );
    }
    add_key(
        keys,
        "Enter",
        Rect {
            x: x[3],
            y: 800,
            width: 38,
            height: 92,
        },
    );
    add_key(
        keys,
        "0",
        Rect {
            x: x[0],
            y: 850,
            width: 79,
            height: KEY_HEIGHT,
        },
    );
    add_key(
        keys,
        ".",
        Rect {
            x: x[2],
            y: 850,
            width: 38,
            height: KEY_HEIGHT,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_keyboard_has_all_104_non_overlapping_keys() {
        let geometry = WorldGeometry::standard_ansi_104().unwrap();
        assert_eq!(geometry.keys().len(), 104);
        assert!(geometry.keys().iter().any(|key| key.label == "F12"));
        assert!(geometry
            .keys()
            .iter()
            .any(|key| key.label == "Space" && key.rect.width > UNIT * 5));
        assert!(geometry
            .keys()
            .iter()
            .any(|key| key.label == "Enter" && key.rect.height > KEY_HEIGHT));
    }
}
