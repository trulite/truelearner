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

    pub(super) fn pop_next_wave(&mut self, cost: &mut ExecutionCost) -> Option<Vec<Firing>> {
        let first = self.pop_next(cost)?;
        let prefix = (first.arrival_tick, first.phase, first.causal_wave);
        let index = self.bucket_index(first.arrival_tick);
        let bucket = &mut self.near[index];
        let mut remainder = bucket
            .extract_if(.., |firing| {
                cost.scans = cost.scans.saturating_add(1);
                cost.touch::<Firing>(1);
                (firing.arrival_tick, firing.phase, firing.causal_wave) == prefix
            })
            .collect::<Vec<_>>();
        self.len = self.len.saturating_sub(remainder.len());
        radix_sort_by_serial(&mut remainder, cost);
        let mut wave = Vec::with_capacity(remainder.len().saturating_add(1));
        wave.push(first);
        wave.extend(remainder);
        Some(wave)
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

fn radix_sort_by_serial(firings: &mut Vec<Firing>, cost: &mut ExecutionCost) {
    if firings.len() < 2 {
        return;
    }
    let already_ordered = firings.windows(2).all(|pair| {
        cost.comparisons = cost.comparisons.saturating_add(1);
        cost.timing_wheel_bucket_selection_comparisons = cost
            .timing_wheel_bucket_selection_comparisons
            .saturating_add(1);
        cost.touch::<Firing>(2);
        pair[0].serial < pair[1].serial
    });
    if already_ordered {
        return;
    }
    let len = firings.len();
    let mut source = firings.drain(..).map(Some).collect::<Vec<_>>();
    let mut target = std::iter::repeat_with(|| None)
        .take(len)
        .collect::<Vec<Option<Firing>>>();
    cost.allocations = cost.allocations.saturating_add(2);
    for shift in (0..u64::BITS).step_by(u8::BITS as usize) {
        let mut counts = [0_usize; 256];
        for firing in source.iter().flatten() {
            cost.scans = cost.scans.saturating_add(1);
            cost.touch::<Firing>(1);
            counts[((firing.serial >> shift) & 0xff) as usize] += 1;
        }
        let mut positions = [0_usize; 256];
        let mut offset = 0;
        for (position, count) in positions.iter_mut().zip(counts) {
            *position = offset;
            offset += count;
        }
        for slot in &mut source {
            let firing = slot.take().expect("radix source is populated");
            cost.scans = cost.scans.saturating_add(1);
            cost.touch::<Firing>(1);
            let bucket = ((firing.serial >> shift) & 0xff) as usize;
            let index = positions[bucket];
            positions[bucket] += 1;
            target[index] = Some(firing);
        }
        std::mem::swap(&mut source, &mut target);
    }
    firings.extend(
        source
            .into_iter()
            .map(|firing| firing.expect("radix result is populated")),
    );
}

fn minimum_index(firings: &[Firing], cost: &mut ExecutionCost) -> Option<usize> {
    let mut first = 0;
    if firings.is_empty() {
        return None;
    }
    for candidate in 1..firings.len() {
        cost.comparisons = cost.comparisons.saturating_add(1);
        cost.timing_wheel_bucket_selection_comparisons = cost
            .timing_wheel_bucket_selection_comparisons
            .saturating_add(1);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn firing(arrival_tick: i64, serial: u64) -> Firing {
        firing_in_wave(arrival_tick, 0, 0, serial)
    }

    fn firing_in_wave(arrival_tick: i64, phase: i32, causal_wave: u64, serial: u64) -> Firing {
        Firing {
            arrival_tick,
            phase,
            causal_wave,
            origin_physical: serial,
            causal_lineage: None,
            physical_incidence: PhysicalIncidence::Sample,
            target_physical: 1,
            target: JunctionId(0),
            target_generation: Generation(0),
            impulse: 1,
            strength: UNIT,
            serial,
            link: None,
        }
    }

    #[test]
    fn timing_wheel_comparison_attribution_partitions_the_existing_total() {
        let mut wheel = TimingWheel::new(0, 8);
        let mut cost = ExecutionCost::default();
        wheel.push(firing(1, 2), &mut cost);
        wheel.push(firing(1, 1), &mut cost);
        wheel.push(firing(2, 3), &mut cost);

        assert!(wheel.pop_next_wave(&mut cost).is_some());
        assert!(cost.timing_wheel_bucket_selection_comparisons > 0);
        assert_eq!(cost.attributed_comparisons(), cost.comparisons);
    }

    #[test]
    fn timing_wheel_wave_preserves_causal_order_and_residual() {
        let mut wheel = TimingWheel::new(0, 8);
        let mut cost = ExecutionCost::default();
        for firing in [
            firing_in_wave(1, 1, 0, 6),
            firing_in_wave(1, 0, 1, 5),
            firing_in_wave(2, 0, 0, 7),
            firing_in_wave(1, 0, 0, 3),
            firing_in_wave(1, 0, 0, 1),
            firing_in_wave(1, 0, 0, 2),
        ] {
            wheel.push(firing, &mut cost);
        }

        let wave = wheel.pop_next_wave(&mut cost).expect("earliest wave");
        assert_eq!(
            wave.iter().map(|firing| firing.serial).collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert_eq!(
            wheel
                .firings()
                .map(causal_order_key)
                .collect::<BTreeSet<_>>(),
            [
                causal_order_key(&firing_in_wave(1, 0, 1, 5)),
                causal_order_key(&firing_in_wave(1, 1, 0, 6)),
                causal_order_key(&firing_in_wave(2, 0, 0, 7)),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn timing_wheel_wave_comparisons_scale_linearly() {
        for size in [4_u64, 8, 16, 32, 64] {
            let mut wheel = TimingWheel::new(0, 8);
            let mut cost = ExecutionCost::default();
            for serial in (0..size).rev() {
                wheel.push(firing_in_wave(1, 0, 0, serial), &mut cost);
            }

            let wave = wheel.pop_next_wave(&mut cost).expect("wave");
            assert_eq!(wave.len(), size as usize);
            assert!(
                cost.comparisons <= size.saturating_mul(2),
                "{size} items used {} comparisons",
                cost.comparisons
            );
        }
    }
}
