use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MomentKey {
    pub tick: i64,
    pub phase: i32,
    pub causal: u64,
}

impl MomentKey {
    pub const fn new(tick: i64, phase: i32, causal: u64) -> Self {
        Self {
            tick,
            phase,
            causal,
        }
    }
}

pub trait TimelineItem {
    fn serial(&self) -> u64;

    fn retained_bytes(&self) -> usize {
        0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QueueWork {
    pub allocations: u64,
    pub scans: u64,
    pub comparisons: u64,
    pub touched_items: u64,
}

/// Monotone physical moments with stable causal order inside each frontier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Timeline<T: TimelineItem> {
    head_tick: i64,
    moments: BTreeMap<MomentKey, Vec<T>>,
    len: usize,
    memory_bytes: usize,
}

impl<T: TimelineItem> Timeline<T> {
    pub fn new(head_tick: i64) -> Self {
        Self {
            head_tick,
            moments: BTreeMap::new(),
            len: 0,
            memory_bytes: 0,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, moment: MomentKey, value: T, work: &mut QueueWork) {
        assert!(
            moment.tick >= self.head_tick,
            "scheduled activity cannot precede timeline head"
        );
        let (batch, new_moment) = match self.moments.entry(moment) {
            std::collections::btree_map::Entry::Vacant(entry) => (entry.insert(Vec::new()), true),
            std::collections::btree_map::Entry::Occupied(entry) => (entry.into_mut(), false),
        };
        if new_moment {
            self.memory_bytes = self
                .memory_bytes
                .saturating_add(std::mem::size_of::<MomentKey>() + std::mem::size_of::<Vec<T>>());
        }
        let capacity_before = batch.capacity();
        if batch.len() == capacity_before {
            work.allocations = work.allocations.saturating_add(1);
        }
        let retained_bytes = value.retained_bytes();
        batch.push(value);
        self.memory_bytes = self
            .memory_bytes
            .saturating_add(
                batch
                    .capacity()
                    .saturating_sub(capacity_before)
                    .saturating_mul(std::mem::size_of::<T>()),
            )
            .saturating_add(retained_bytes);
        self.len += 1;
    }

    pub fn pop(&mut self, work: &mut QueueWork) -> Option<(MomentKey, Vec<T>)> {
        let (moment, mut entries) = self.moments.pop_first()?;
        self.head_tick = moment.tick;
        self.len = self.len.saturating_sub(entries.len());
        let released = std::mem::size_of::<MomentKey>()
            .saturating_add(std::mem::size_of::<Vec<T>>())
            .saturating_add(entries.capacity().saturating_mul(std::mem::size_of::<T>()))
            .saturating_add(
                entries
                    .iter()
                    .map(TimelineItem::retained_bytes)
                    .sum::<usize>(),
            );
        self.memory_bytes = self.memory_bytes.saturating_sub(released);
        radix_sort_by_serial(&mut entries, work);
        Some((moment, entries))
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.moments.values().flat_map(|batch| batch.iter())
    }

    pub const fn memory_bytes(&self) -> usize {
        self.memory_bytes
    }
}

fn radix_sort_by_serial<T: TimelineItem>(entries: &mut Vec<T>, work: &mut QueueWork) {
    if entries.len() < 2 {
        return;
    }
    let already_ordered = entries.windows(2).all(|pair| {
        work.comparisons = work.comparisons.saturating_add(1);
        work.touched_items = work.touched_items.saturating_add(2);
        pair[0].serial() < pair[1].serial()
    });
    if already_ordered {
        return;
    }

    let len = entries.len();
    let mut source = entries.drain(..).map(Some).collect::<Vec<_>>();
    let mut target = std::iter::repeat_with(|| None)
        .take(len)
        .collect::<Vec<Option<T>>>();
    work.allocations = work.allocations.saturating_add(2);
    for shift in (0..u64::BITS).step_by(u8::BITS as usize) {
        let mut counts = [0_usize; 256];
        for entry in source.iter().flatten() {
            work.scans = work.scans.saturating_add(1);
            work.touched_items = work.touched_items.saturating_add(1);
            counts[((entry.serial() >> shift) & 0xff) as usize] += 1;
        }
        let mut positions = [0_usize; 256];
        let mut offset = 0;
        for (position, count) in positions.iter_mut().zip(counts) {
            *position = offset;
            offset += count;
        }
        for slot in &mut source {
            let entry = slot.take().expect("radix source is populated");
            work.scans = work.scans.saturating_add(1);
            work.touched_items = work.touched_items.saturating_add(1);
            let bucket = ((entry.serial() >> shift) & 0xff) as usize;
            let index = positions[bucket];
            positions[bucket] += 1;
            target[index] = Some(entry);
        }
        std::mem::swap(&mut source, &mut target);
    }
    entries.extend(
        source
            .into_iter()
            .map(|entry| entry.expect("radix result is populated")),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Item(u64);

    impl TimelineItem for Item {
        fn serial(&self) -> u64 {
            self.0
        }
    }

    #[test]
    fn timeline_preserves_moment_and_causal_order() {
        let mut timeline = Timeline::new(0);
        let mut work = QueueWork::default();
        for (moment, serial) in [
            (MomentKey::new(1, 1, 0), 6),
            (MomentKey::new(1, 0, 1), 5),
            (MomentKey::new(2, 0, 0), 7),
            (MomentKey::new(1, 0, 0), 3),
            (MomentKey::new(1, 0, 0), 1),
            (MomentKey::new(1, 0, 0), 2),
        ] {
            timeline.push(moment, Item(serial), &mut work);
        }

        let (moment, wave) = timeline.pop(&mut work).expect("earliest frontier");
        assert_eq!(moment, MomentKey::new(1, 0, 0));
        assert_eq!(wave, [Item(1), Item(2), Item(3)]);
        assert_eq!(
            timeline.values().map(|item| item.0).collect::<Vec<_>>(),
            [5, 6, 7]
        );
    }

    #[test]
    fn quiet_timeline_is_identity() {
        let mut timeline = Timeline::<Item>::new(0);
        assert!(timeline.pop(&mut QueueWork::default()).is_none());
        assert!(timeline.is_empty());
        assert_eq!(timeline.memory_bytes(), 0);
    }
}
