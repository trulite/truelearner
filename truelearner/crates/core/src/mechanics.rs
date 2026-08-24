use super::{CellId, ExecutionCost, Spike};

const DEFAULT_RING_WIDTH: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchedulerKind {
    Vec,
    TimingWheel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum PendingSchedule {
    Vec(Vec<Spike>),
    TimingWheel(TimingWheel),
}

impl PendingSchedule {
    pub(super) fn new(kind: SchedulerKind, head_tick: i64) -> Self {
        match kind {
            SchedulerKind::Vec => Self::Vec(Vec::new()),
            SchedulerKind::TimingWheel => {
                Self::TimingWheel(TimingWheel::new(head_tick, DEFAULT_RING_WIDTH))
            }
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(super) fn len(&self) -> usize {
        match self {
            Self::Vec(spikes) => spikes.len(),
            Self::TimingWheel(wheel) => wheel.len,
        }
    }

    pub(super) fn push(&mut self, spike: Spike, cost: &mut ExecutionCost) {
        cost.queue_ops = cost.queue_ops.saturating_add(1);
        match self {
            Self::Vec(spikes) => spikes.push(spike),
            Self::TimingWheel(wheel) => wheel.push(spike),
        }
    }

    pub(super) fn pop_next<F>(
        &mut self,
        target_physical: F,
        cost: &mut ExecutionCost,
    ) -> Option<(Spike, u64)>
    where
        F: Fn(CellId) -> u64,
    {
        cost.queue_ops = cost.queue_ops.saturating_add(1);
        match self {
            Self::Vec(spikes) => {
                let index = minimum_index(spikes, &target_physical, cost)?;
                let comparisons = u64::try_from(spikes.len().saturating_sub(1)).unwrap_or(u64::MAX);
                Some((spikes.remove(index), comparisons))
            }
            Self::TimingWheel(wheel) => wheel.pop_next(target_physical, cost),
        }
    }

    pub(super) fn canonical<F>(&self, target_physical: F) -> Vec<Spike>
    where
        F: Fn(CellId) -> u64,
    {
        let mut spikes = match self {
            Self::Vec(spikes) => spikes.clone(),
            Self::TimingWheel(wheel) => wheel
                .near
                .iter()
                .flat_map(|bucket| bucket.iter().cloned())
                .chain(wheel.overflow.iter().cloned())
                .collect(),
        };
        spikes.sort_by_key(|spike| order_key(spike, &target_physical));
        spikes
    }

    pub(super) fn from_canonical(
        kind: SchedulerKind,
        head_tick: i64,
        spikes: Vec<Spike>,
    ) -> Self {
        let mut schedule = Self::new(kind, head_tick);
        let mut ignored = ExecutionCost::default();
        for spike in spikes {
            schedule.push(spike, &mut ignored);
        }
        schedule
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TimingWheel {
    head_tick: i64,
    near: Vec<Vec<Spike>>,
    overflow: Vec<Spike>,
    len: usize,
}

impl TimingWheel {
    fn new(head_tick: i64, width: usize) -> Self {
        assert!(width > 0, "timing wheel must have a positive width");
        Self {
            head_tick,
            near: vec![Vec::new(); width],
            overflow: Vec::new(),
            len: 0,
        }
    }

    fn push(&mut self, spike: Spike) {
        assert!(
            spike.arrival_tick >= self.head_tick,
            "scheduled activity cannot precede timing-wheel head"
        );
        if self.in_near_window(spike.arrival_tick) {
            let index = self.bucket_index(spike.arrival_tick);
            self.near[index].push(spike);
        } else {
            self.overflow.push(spike);
        }
        self.len += 1;
    }

    fn pop_next<F>(
        &mut self,
        target_physical: F,
        cost: &mut ExecutionCost,
    ) -> Option<(Spike, u64)>
    where
        F: Fn(CellId) -> u64,
    {
        if self.len == 0 {
            return None;
        }
        self.promote_overflow(cost);
        let next_tick = self
            .near
            .iter()
            .flat_map(|bucket| bucket.iter().map(|spike| spike.arrival_tick))
            .min()
            .or_else(|| self.overflow.iter().map(|spike| spike.arrival_tick).min())?;
        if next_tick >= self.head_tick.saturating_add(self.near.len() as i64) {
            self.head_tick = next_tick;
            self.promote_overflow(cost);
        } else {
            self.head_tick = next_tick;
        }
        let index = self.bucket_index(next_tick);
        let bucket = &mut self.near[index];
        let before = bucket.len();
        let selected = minimum_index(bucket, &target_physical, cost)
            .expect("next timing-wheel bucket must contain an arrival");
        let spike = bucket.remove(selected);
        self.len -= 1;
        let comparisons = u64::try_from(before.saturating_sub(1)).unwrap_or(u64::MAX);
        Some((spike, comparisons))
    }

    fn promote_overflow(&mut self, cost: &mut ExecutionCost) {
        let end = self.head_tick.saturating_add(self.near.len() as i64);
        let mut retained = Vec::with_capacity(self.overflow.len());
        for spike in self.overflow.drain(..) {
            cost.scans = cost.scans.saturating_add(1);
            if spike.arrival_tick < end {
                let offset = spike.arrival_tick.rem_euclid(self.near.len() as i64) as usize;
                self.near[offset].push(spike);
            } else {
                retained.push(spike);
            }
        }
        self.overflow = retained;
    }

    fn in_near_window(&self, tick: i64) -> bool {
        tick < self.head_tick.saturating_add(self.near.len() as i64)
    }

    fn bucket_index(&self, tick: i64) -> usize {
        tick.rem_euclid(self.near.len() as i64) as usize
    }
}

fn minimum_index<F>(
    spikes: &[Spike],
    target_physical: &F,
    cost: &mut ExecutionCost,
) -> Option<usize>
where
    F: Fn(CellId) -> u64,
{
    let mut first = 0;
    if spikes.is_empty() {
        return None;
    }
    for candidate in 1..spikes.len() {
        cost.comparisons = cost.comparisons.saturating_add(1);
        if order_key(&spikes[candidate], target_physical)
            < order_key(&spikes[first], target_physical)
        {
            first = candidate;
        }
    }
    Some(first)
}

fn order_key<F>(spike: &Spike, target_physical: &F) -> (i64, i32, u64, u64, u64)
where
    F: Fn(CellId) -> u64,
{
    (
        spike.arrival_tick,
        spike.phase,
        spike.origin_physical,
        target_physical(spike.target),
        spike.serial,
    )
}
