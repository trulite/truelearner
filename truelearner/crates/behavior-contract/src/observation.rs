use crate::{LawTrace, MotorId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Effect {
    pub at: u64,
    pub motor: MotorId,
    pub impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Observation {
    pub effects: Vec<Effect>,
    pub quiet: bool,
    pub trace: LawTrace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expected {
    pub effects: Vec<Effect>,
    pub quiet: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BehaviorMismatch {
    Effects {
        observed: Vec<Effect>,
        expected: Vec<Effect>,
    },
    Quiet {
        observed: bool,
        expected: bool,
    },
}

impl Expected {
    pub fn quiet(effects: Vec<Effect>) -> Self {
        Self {
            effects,
            quiet: true,
        }
    }
}

impl Observation {
    pub fn compare(&self, expected: &Expected) -> Result<(), BehaviorMismatch> {
        if self.effects != expected.effects {
            return Err(BehaviorMismatch::Effects {
                observed: self.effects.clone(),
                expected: expected.effects.clone(),
            });
        }
        if self.quiet != expected.quiet {
            return Err(BehaviorMismatch::Quiet {
                observed: self.quiet,
                expected: expected.quiet,
            });
        }
        Ok(())
    }
}
