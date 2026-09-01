use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

pub type Time = u64;
pub type Impulse = i32;
pub const DRIVE_MAX: u16 = 1_023;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct JunctionId(NonZeroU32);

impl JunctionId {
    pub(crate) fn new(slot: usize) -> Option<Self> {
        u32::try_from(slot)
            .ok()?
            .checked_add(1)
            .and_then(NonZeroU32::new)
            .map(Self)
    }

    pub(crate) const fn slot(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LinkId(NonZeroU32);

impl LinkId {
    pub(crate) fn new(slot: usize) -> Option<Self> {
        u32::try_from(slot)
            .ok()?
            .checked_add(1)
            .and_then(NonZeroU32::new)
            .map(Self)
    }

    pub(crate) const fn slot(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Retention {
    Integrating,
    Sampled { lifetime: Time, range: u32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Junction {
    pub threshold: Impulse,
    pub retention: Retention,
}

impl Junction {
    pub const fn integrating(threshold: Impulse) -> Self {
        Self {
            threshold,
            retention: Retention::Integrating,
        }
    }

    pub const fn sampled(lifetime: Time) -> Self {
        Self::sampled_in(lifetime, DRIVE_MAX as u32)
    }

    pub const fn sampled_in(lifetime: Time, range: u32) -> Self {
        Self {
            threshold: 1,
            retention: Retention::Sampled { lifetime, range },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trigger {
    SourceFires,
    RisesThrough(Impulse),
    FallsThrough(Impulse),
    Rises,
    Falls,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Link {
    pub from: JunctionId,
    pub to: JunctionId,
    pub delay: Time,
    pub impulse: Impulse,
    pub trigger: Trigger,
}

impl Link {
    pub const fn new(from: JunctionId, to: JunctionId, delay: Time, impulse: Impulse) -> Self {
        Self {
            from,
            to,
            delay,
            impulse,
            trigger: Trigger::SourceFires,
        }
    }

    pub const fn when(mut self, trigger: Trigger) -> Self {
        self.trigger = trigger;
        self
    }
}

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
    pub cause: u64,
}

impl Arrival {
    pub const fn new(target: JunctionId, impulse: Impulse) -> Self {
        Self {
            target,
            impulse,
            cause: 0,
        }
    }

    pub const fn caused(target: JunctionId, impulse: Impulse, cause: u64) -> Self {
        Self {
            target,
            impulse,
            cause,
        }
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
    pub cause: u64,
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

    pub(crate) fn change(
        &mut self,
        at: Time,
        impulse: i64,
        cause: u64,
    ) -> Option<(Impulse, Impulse)> {
        match self.retention {
            RetentionTag::Integrating => {
                if self.value != 0 && self.stamp != cause {
                    self.value = 0;
                }
                let before = self.value;
                let after = clamp_signal(i64::from(before) + impulse);
                self.value = after;
                self.stamp = cause;
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

pub(crate) fn opens(trigger: Trigger, before: Impulse, after: Impulse) -> bool {
    match trigger {
        Trigger::SourceFires => true,
        Trigger::Rises => after > before,
        Trigger::Falls => after < before,
        Trigger::RisesThrough(level) => before < level && after >= level,
        Trigger::FallsThrough(level) => before > level && after <= level,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_changes_are_mapped_to_the_common_drive_scale() {
        assert_eq!(normalized_drive(0, 128, 255), 514);
        assert_eq!(normalized_drive(0, 512, 1_023), 512);
        assert_eq!(normalized_drive(-1_023, 1_023, 2_046), 1_023);
    }
}
