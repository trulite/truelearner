use std::num::NonZeroU32;

pub type Time = u64;
pub type Impulse = i32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retention {
    Integrating,
    Sampled { lifetime: Time },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        Self {
            threshold: 1,
            retention: Retention::Sampled { lifetime },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigger {
    SourceFires,
    RisesThrough(Impulse),
    FallsThrough(Impulse),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Event {
    pub at: Time,
    pub junction: JunctionId,
    pub arrivals: u32,
    pub impulse: i64,
    pub before: Impulse,
    pub after: Impulse,
    pub cause: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub arrivals: u64,
    pub meetings: u64,
    pub changes: u64,
    pub link_visits: u64,
    pub emissions: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Step {
    pub at: Time,
    pub work: Work,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
    sampled_at: Time,
    value: Impulse,
    threshold: Impulse,
    pub(crate) outgoing_head: Option<LinkId>,
    retention: RetentionTag,
    sampled_known: bool,
}

impl JunctionSlot {
    pub(crate) const fn new(law: Junction) -> Self {
        let (retention, lifetime) = match law.retention {
            Retention::Integrating => (RetentionTag::Integrating, 0),
            Retention::Sampled { lifetime } => (RetentionTag::Sampled, lifetime),
        };
        Self {
            lifetime,
            sampled_at: 0,
            value: 0,
            threshold: law.threshold,
            outgoing_head: None,
            retention,
            sampled_known: false,
        }
    }

    pub(crate) const fn held(self) -> Impulse {
        self.value
    }

    pub(crate) fn clear(&mut self) {
        self.value = 0;
        self.sampled_known = false;
    }

    pub(crate) fn change(&mut self, at: Time, impulse: i64) -> Option<(Impulse, Impulse)> {
        match self.retention {
            RetentionTag::Integrating => {
                let before = self.value;
                let after = clamp_signal(i64::from(before) + impulse);
                self.value = after;
                (after >= self.threshold).then(|| {
                    self.value = 0;
                    (before, after)
                })
            }
            RetentionTag::Sampled => {
                let impulse = clamp_signal(impulse);
                let before = (self.sampled_known
                    && at.saturating_sub(self.sampled_at) < self.lifetime)
                    .then_some(self.value);
                self.value = impulse;
                self.sampled_at = at;
                self.sampled_known = true;
                before
                    .filter(|before| *before != impulse)
                    .map(|before| (before, impulse))
            }
        }
    }
}

fn clamp_signal(value: i64) -> Impulse {
    value.clamp(i64::from(Impulse::MIN), i64::from(Impulse::MAX)) as Impulse
}

pub(crate) fn opens(trigger: Trigger, before: Impulse, after: Impulse) -> bool {
    match trigger {
        Trigger::SourceFires => true,
        Trigger::RisesThrough(level) => before < level && after >= level,
        Trigger::FallsThrough(level) => before > level && after <= level,
    }
}
