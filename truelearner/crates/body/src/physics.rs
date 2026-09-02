use serde::Serialize;
pub(crate) use truelearner_core::opens;
pub use truelearner_core::{
    Impulse, Junction, JunctionId, Link, LinkId, Retention, Time, Trigger, DRIVE_MAX,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildError {
    NonPositiveThreshold,
    InvalidRange,
    CapacityExhausted,
    UnknownJunction(JunctionId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunError {
    UnknownJunction(JunctionId),
    TimeWentBackward { now: Time, requested: Time },
    WaveTooLarge,
    CapacityExhausted,
    InvalidReaction,
    MomentLimitReached,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arrival {
    pub target: JunctionId,
    pub impulse: Impulse,
}

impl Arrival {
    pub const fn new(target: JunctionId, impulse: Impulse) -> Self {
        Self { target, impulse }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Event {
    pub at: Time,
    pub junction: JunctionId,
    pub arrivals: u32,
    pub impulse: i64,
    pub before: Impulse,
    pub after: Impulse,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Work {
    pub arrivals: u64,
    pub meetings: u64,
    pub changes: u64,
    pub link_visits: u64,
    pub emissions: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Step {
    pub at: Time,
    pub work: Work,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Run {
    pub moments: u64,
    pub work: Work,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetentionTag {
    Integrating,
    Sampled,
}

/// One locally traversable two-link path may still meet membrane potential
/// already held at its destination. Older potential cannot participate.
pub(crate) const INTEGRATION_WINDOW: Time = 4;

/// Stored junction state and its propagation edge, in one 32-byte unit.
#[repr(align(32))]
#[derive(Clone, Copy, Debug)]
pub(crate) struct JunctionSlot {
    lifetime: Time,
    stamp: Time,
    value: Impulse,
    threshold: Impulse,
    pub(crate) outgoing_head: Option<LinkId>,
    retention: RetentionTag,
    sampled_known: bool,
}

impl JunctionSlot {
    pub(crate) const fn new(law: Junction) -> Self {
        let (retention, lifetime, threshold) = match law.retention {
            Retention::Integrating => (RetentionTag::Integrating, 0, law.threshold),
            Retention::Sampled { lifetime, range } => {
                (RetentionTag::Sampled, lifetime, range as Impulse)
            }
        };
        Self {
            lifetime,
            stamp: 0,
            value: 0,
            threshold,
            outgoing_head: None,
            retention,
            sampled_known: false,
        }
    }

    pub(crate) const fn held(self) -> Impulse {
        self.value
    }

    pub(crate) const fn checkpoint_law(self) -> Junction {
        Junction {
            threshold: self.threshold,
            retention: match self.retention {
                RetentionTag::Integrating => Retention::Integrating,
                RetentionTag::Sampled => Retention::Sampled {
                    lifetime: self.lifetime,
                    range: self.threshold as u32,
                },
            },
        }
    }

    pub(crate) const fn checkpoint_state(self) -> (Time, Impulse, bool) {
        (self.stamp, self.value, self.sampled_known)
    }

    pub(crate) fn restore_state(&mut self, stamp: Time, value: Impulse, sampled_known: bool) {
        self.stamp = stamp;
        self.value = value;
        self.sampled_known = sampled_known;
    }

    pub(crate) fn clear(&mut self) {
        self.value = 0;
        self.stamp = 0;
        self.sampled_known = false;
    }

    pub(crate) fn change(&mut self, at: Time, impulse: i64) -> Option<(Impulse, Impulse)> {
        match self.retention {
            RetentionTag::Integrating => {
                if self.value != 0 && at.saturating_sub(self.stamp) > INTEGRATION_WINDOW {
                    self.value = 0;
                }
                let before = self.value;
                let after = clamp_signal(i64::from(before) + impulse);
                self.value = after;
                self.stamp = at;
                (after >= self.threshold).then(|| {
                    self.value = 0;
                    (before, after)
                })
            }
            RetentionTag::Sampled => {
                let impulse = clamp_signal(impulse);
                let before = (self.sampled_known && at.saturating_sub(self.stamp) < self.lifetime)
                    .then_some(self.value);
                self.value = impulse;
                self.stamp = at;
                self.sampled_known = true;
                before
                    .filter(|before| *before != impulse)
                    .map(|before| (before, impulse))
            }
        }
    }

    pub(crate) fn drive(&self, before: Impulse, after: Impulse) -> u16 {
        let drive = match self.retention {
            RetentionTag::Integrating => before.abs_diff(after).min(u32::from(DRIVE_MAX)),
            RetentionTag::Sampled => normalized_drive(before, after, self.threshold as u32),
        };
        drive as u16
    }
}

fn normalized_drive(before: Impulse, after: Impulse, range: u32) -> u32 {
    let change = before.abs_diff(after);
    if range == u32::from(DRIVE_MAX) {
        return change.min(u32::from(DRIVE_MAX));
    }
    let change = u64::from(change);
    let range = u64::from(range);
    let scaled = (change * u64::from(DRIVE_MAX) + range / 2) / range;
    u32::try_from(scaled.min(u64::from(DRIVE_MAX))).unwrap_or(u32::from(DRIVE_MAX))
}

fn clamp_signal(value: i64) -> Impulse {
    value.clamp(i64::from(Impulse::MIN), i64::from(Impulse::MAX)) as Impulse
}

#[cfg(test)]
#[path = "tests/physics.rs"]
mod tests;
