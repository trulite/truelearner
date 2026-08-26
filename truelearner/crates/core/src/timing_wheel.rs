use crate::prelude::*;

type CausalOrderKey = (i64, i32, u64, u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TimingWheel {
    head_tick: i64,
    near: Vec<Vec<Firing>>,
    overflow: Vec<Firing>,
    len: usize,
}

impl TimingWheel {
    pub(super) fn new(head_tick: i64, width: usize) -> Self {
        assert!(width > 0, "timing wheel must have a positive width");
        Self {
            head_tick,
            near: vec![Vec::new(); width],
            overflow: Vec::new(),
            len: 0,
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub(super) fn push(&mut self, firing: Firing, cost: &mut ExecutionCost) {
        assert!(
            firing.arrival_tick >= self.head_tick,
            "scheduled activity cannot precede timing-wheel head"
        );
        if self.in_near_window(firing.arrival_tick) {
            let index = self.bucket_index(firing.arrival_tick);
            if self.near[index].len() == self.near[index].capacity() {
                cost.allocations = cost.allocations.saturating_add(1);
            }
            self.near[index].push(firing);
        } else {
            if self.overflow.len() == self.overflow.capacity() {
                cost.allocations = cost.allocations.saturating_add(1);
            }
            self.overflow.push(firing);
        }
        self.len += 1;
    }

    pub(super) fn pop_next(&mut self, cost: &mut ExecutionCost) -> Option<Firing> {
        if self.is_empty() {
            return None;
        }
        self.promote_overflow(cost);
        let next_tick = self
            .near
            .iter()
            .flat_map(|bucket| bucket.iter().map(|firing| firing.arrival_tick))
            .min()
            .or_else(|| self.overflow.iter().map(|firing| firing.arrival_tick).min())?;
        if next_tick >= self.head_tick.saturating_add(self.near.len() as i64) {
            self.head_tick = next_tick;
            self.promote_overflow(cost);
        } else {
            self.head_tick = next_tick;
        }
        let index = self.bucket_index(next_tick);
        let bucket = &mut self.near[index];
        let selected =
            minimum_index(bucket, cost).expect("next timing-wheel bucket must contain an arrival");
        let firing = bucket.remove(selected);
        self.len -= 1;
        Some(firing)
    }

    pub(super) fn minimum_key(&self, cost: &mut ExecutionCost) -> Option<CausalOrderKey> {
        let mut selected = None;
        for firing in self.firings() {
            cost.touch::<Firing>(1);
            let key = causal_order_key(firing);
            if selected.is_some() {
                cost.comparisons = cost.comparisons.saturating_add(1);
            }
            if selected.is_none_or(|current| key < current) {
                selected = Some(key);
            }
        }
        selected
    }

    pub(super) fn firings(&self) -> impl Iterator<Item = &Firing> {
        self.near
            .iter()
            .flat_map(|bucket| bucket.iter())
            .chain(self.overflow.iter())
    }

    pub(super) fn memory_bytes(&self) -> usize {
        self.near.capacity() * std::mem::size_of::<Vec<Firing>>()
            + self
                .near
                .iter()
                .map(|bucket| bucket.capacity() * std::mem::size_of::<Firing>())
                .sum::<usize>()
            + self.overflow.capacity() * std::mem::size_of::<Firing>()
    }

    fn promote_overflow(&mut self, cost: &mut ExecutionCost) {
        let end = self.head_tick.saturating_add(self.near.len() as i64);
        let mut retained = Vec::with_capacity(self.overflow.len());
        if !self.overflow.is_empty() {
            cost.allocations = cost.allocations.saturating_add(1);
        }
        for firing in self.overflow.drain(..) {
            cost.scans = cost.scans.saturating_add(1);
            cost.touch::<Firing>(1);
            if firing.arrival_tick < end {
                let offset = firing.arrival_tick.rem_euclid(self.near.len() as i64) as usize;
                if self.near[offset].len() == self.near[offset].capacity() {
                    cost.allocations = cost.allocations.saturating_add(1);
                }
                self.near[offset].push(firing);
            } else {
                retained.push(firing);
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

fn minimum_index(firings: &[Firing], cost: &mut ExecutionCost) -> Option<usize> {
    let mut first = 0;
    if firings.is_empty() {
        return None;
    }
    for candidate in 1..firings.len() {
        cost.comparisons = cost.comparisons.saturating_add(1);
        cost.touch::<Firing>(2);
        if causal_order_key(&firings[candidate]) < causal_order_key(&firings[first]) {
            first = candidate;
        }
    }
    Some(first)
}

fn causal_order_key(firing: &Firing) -> CausalOrderKey {
    (
        firing.arrival_tick,
        firing.phase,
        firing.causal_wave,
        firing.serial,
    )
}
