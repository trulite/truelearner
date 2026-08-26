use crate::prelude::*;
use crate::timing_wheel::TimingWheel;

pub(crate) const LOCAL_DECAY_PERIOD: i64 = 10;

pub(crate) fn pressure_epoch(tick: i64) -> i64 {
    tick.div_euclid(LOCAL_DECAY_PERIOD)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhysicalClock {
    pub tick: i64,
}

impl PhysicalClock {
    pub fn pressure_phase(self) -> i64 {
        self.tick.rem_euclid(LOCAL_DECAY_PERIOD)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Firing {
    pub(crate) arrival_tick: i64,
    pub(crate) phase: i32,
    pub(crate) causal_wave: u64,
    pub(crate) origin_physical: u64,
    pub(crate) target_physical: u64,
    pub(crate) target: JunctionId,
    pub(crate) target_generation: Generation,
    pub(crate) impulse: i32,
    pub(crate) strength: i64,
    pub(crate) serial: u64,
    pub(crate) link: Option<(LinkId, Generation)>,
}

const RING_WIDTH: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Schedule(TimingWheel);

impl Schedule {
    pub(super) fn new(head_tick: i64) -> Self {
        Self(TimingWheel::new(head_tick, RING_WIDTH))
    }

    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(super) fn push(&mut self, firing: Firing, cost: &mut ExecutionCost) {
        cost.queue_ops = cost.queue_ops.saturating_add(1);
        cost.touch::<Firing>(1);
        self.0.push(firing, cost);
    }

    pub(super) fn next_wave(&mut self, cost: &mut ExecutionCost) -> Option<Vec<Firing>> {
        let first = self.0.pop_next(cost)?;
        let prefix = (first.arrival_tick, first.phase, first.causal_wave);
        let mut batch = vec![first];
        while self
            .0
            .minimum_key(cost)
            .is_some_and(|key| (key.0, key.1, key.2) == prefix)
        {
            batch.push(
                self.0
                    .pop_next(cost)
                    .expect("peeked causal moment must remain available"),
            );
        }
        Some(batch)
    }

    pub(super) fn canonical(&self) -> Vec<Firing> {
        let mut firings: Vec<_> = self.0.firings().cloned().collect();
        firings.sort_by_key(canonical_storage_key);
        firings
    }

    pub(super) fn from_canonical(head_tick: i64, firings: Vec<Firing>) -> Self {
        let mut schedule = Self::new(head_tick);
        let mut ignored = ExecutionCost::default();
        for firing in firings {
            schedule.push(firing, &mut ignored);
        }
        schedule
    }

    pub(super) fn memory_bytes(&self) -> usize {
        self.0.memory_bytes()
    }
}

fn canonical_storage_key(firing: &Firing) -> (i64, i32, u64, u64, u64, u64) {
    // Checkpoints use a stable order; execution continues to use causal order.
    (
        firing.arrival_tick,
        firing.phase,
        firing.causal_wave,
        firing.origin_physical,
        firing.target.0,
        firing.serial,
    )
}
