use super::{Arrow, ArrowId, Cell, CellId, ExecutionCost, LayoutKind, ResidentArenaId, Spike};

const DEFAULT_RING_WIDTH: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum CellStore {
    AoS(Vec<Cell>),
    SoA(Box<CellColumns>),
}

impl CellStore {
    pub(super) fn new(layout: LayoutKind) -> Self {
        match layout {
            LayoutKind::AoS => Self::AoS(Vec::new()),
            LayoutKind::SoA => Self::SoA(Box::default()),
        }
    }

    pub(super) fn len(&self) -> usize {
        match self {
            Self::AoS(values) => values.len(),
            Self::SoA(values) => values.ids.len(),
        }
    }

    pub(super) fn get(&self, index: usize) -> Cell {
        match self {
            Self::AoS(values) => values[index].clone(),
            Self::SoA(values) => values.get(index),
        }
    }

    pub(super) fn with_mut<R>(&mut self, index: usize, change: impl FnOnce(&mut Cell) -> R) -> R {
        match self {
            Self::AoS(values) => change(&mut values[index]),
            Self::SoA(values) => {
                let mut value = values.get(index);
                let result = change(&mut value);
                values.set(index, value);
                result
            }
        }
    }

    pub(super) fn push(&mut self, value: Cell) {
        match self {
            Self::AoS(values) => values.push(value),
            Self::SoA(values) => values.push(value),
        }
    }

    pub(super) fn values(&self) -> Vec<Cell> {
        (0..self.len()).map(|index| self.get(index)).collect()
    }

    pub(super) fn replace_values(&mut self, values: Vec<Cell>) {
        *self = match self {
            Self::AoS(_) => Self::AoS(values),
            Self::SoA(_) => Self::SoA(Box::new(CellColumns::from_values(values))),
        };
    }

    pub(super) fn convert(&mut self, layout: LayoutKind) {
        if matches!(
            (&*self, layout),
            (Self::AoS(_), LayoutKind::AoS) | (Self::SoA(_), LayoutKind::SoA)
        ) {
            return;
        }
        let values = self.values();
        *self = match layout {
            LayoutKind::AoS => Self::AoS(values),
            LayoutKind::SoA => Self::SoA(Box::new(CellColumns::from_values(values))),
        };
    }

    pub(super) fn resident_bytes(&self) -> usize {
        match self {
            Self::AoS(values) => values
                .capacity()
                .saturating_mul(std::mem::size_of::<Cell>()),
            Self::SoA(values) => values.resident_bytes(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct CellColumns {
    ids: Vec<CellId>,
    physical_ids: Vec<u64>,
    positions: Vec<i32>,
    regions: Vec<i16>,
    thresholds: Vec<i32>,
    states: Vec<i32>,
    last_update_ticks: Vec<i64>,
    refractory_until: Vec<i64>,
    generations: Vec<super::Generation>,
    resistances: Vec<u32>,
    live: Vec<bool>,
}

impl CellColumns {
    fn from_values(values: Vec<Cell>) -> Self {
        let mut columns = Self::default();
        for value in values {
            columns.push(value);
        }
        columns
    }

    fn get(&self, index: usize) -> Cell {
        Cell {
            id: self.ids[index],
            physical_id: self.physical_ids[index],
            position: self.positions[index],
            region: self.regions[index],
            threshold: self.thresholds[index],
            state: self.states[index],
            last_update_tick: self.last_update_ticks[index],
            refractory_until: self.refractory_until[index],
            generation: self.generations[index],
            resistance: self.resistances[index],
            live: self.live[index],
        }
    }

    fn set(&mut self, index: usize, value: Cell) {
        self.ids[index] = value.id;
        self.physical_ids[index] = value.physical_id;
        self.positions[index] = value.position;
        self.regions[index] = value.region;
        self.thresholds[index] = value.threshold;
        self.states[index] = value.state;
        self.last_update_ticks[index] = value.last_update_tick;
        self.refractory_until[index] = value.refractory_until;
        self.generations[index] = value.generation;
        self.resistances[index] = value.resistance;
        self.live[index] = value.live;
    }

    fn push(&mut self, value: Cell) {
        self.ids.push(value.id);
        self.physical_ids.push(value.physical_id);
        self.positions.push(value.position);
        self.regions.push(value.region);
        self.thresholds.push(value.threshold);
        self.states.push(value.state);
        self.last_update_ticks.push(value.last_update_tick);
        self.refractory_until.push(value.refractory_until);
        self.generations.push(value.generation);
        self.resistances.push(value.resistance);
        self.live.push(value.live);
    }

    fn resident_bytes(&self) -> usize {
        self.ids.capacity() * std::mem::size_of::<CellId>()
            + self.physical_ids.capacity() * std::mem::size_of::<u64>()
            + self.positions.capacity() * std::mem::size_of::<i32>()
            + self.regions.capacity() * std::mem::size_of::<i16>()
            + self.thresholds.capacity() * std::mem::size_of::<i32>()
            + self.states.capacity() * std::mem::size_of::<i32>()
            + self.last_update_ticks.capacity() * std::mem::size_of::<i64>()
            + self.refractory_until.capacity() * std::mem::size_of::<i64>()
            + self.generations.capacity() * std::mem::size_of::<super::Generation>()
            + self.resistances.capacity() * std::mem::size_of::<u32>()
            + self.live.capacity() * std::mem::size_of::<bool>()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ArrowStore {
    AoS(Vec<Arrow>),
    SoA(Box<ArrowColumns>),
}

impl ArrowStore {
    pub(super) fn new(layout: LayoutKind) -> Self {
        match layout {
            LayoutKind::AoS => Self::AoS(Vec::new()),
            LayoutKind::SoA => Self::SoA(Box::default()),
        }
    }

    pub(super) fn len(&self) -> usize {
        match self {
            Self::AoS(values) => values.len(),
            Self::SoA(values) => values.ids.len(),
        }
    }

    pub(super) fn get(&self, index: usize) -> Arrow {
        match self {
            Self::AoS(values) => values[index].clone(),
            Self::SoA(values) => values.get(index),
        }
    }

    pub(super) fn with_mut<R>(&mut self, index: usize, change: impl FnOnce(&mut Arrow) -> R) -> R {
        match self {
            Self::AoS(values) => change(&mut values[index]),
            Self::SoA(values) => {
                let mut value = values.get(index);
                let result = change(&mut value);
                values.set(index, value);
                result
            }
        }
    }

    pub(super) fn push(&mut self, value: Arrow) {
        match self {
            Self::AoS(values) => values.push(value),
            Self::SoA(values) => values.push(value),
        }
    }

    pub(super) fn set(&mut self, index: usize, value: Arrow) {
        match self {
            Self::AoS(values) => values[index] = value,
            Self::SoA(values) => values.set(index, value),
        }
    }

    pub(super) fn values(&self) -> Vec<Arrow> {
        (0..self.len()).map(|index| self.get(index)).collect()
    }

    pub(super) fn replace_values(&mut self, values: Vec<Arrow>) {
        *self = match self {
            Self::AoS(_) => Self::AoS(values),
            Self::SoA(_) => Self::SoA(Box::new(ArrowColumns::from_values(values))),
        };
    }

    pub(super) fn convert(&mut self, layout: LayoutKind) {
        if matches!(
            (&*self, layout),
            (Self::AoS(_), LayoutKind::AoS) | (Self::SoA(_), LayoutKind::SoA)
        ) {
            return;
        }
        let values = self.values();
        *self = match layout {
            LayoutKind::AoS => Self::AoS(values),
            LayoutKind::SoA => Self::SoA(Box::new(ArrowColumns::from_values(values))),
        };
    }

    pub(super) fn resident_bytes(&self) -> usize {
        match self {
            Self::AoS(values) => values
                .capacity()
                .saturating_mul(std::mem::size_of::<Arrow>()),
            Self::SoA(values) => values.resident_bytes(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct ArrowColumns {
    ids: Vec<ArrowId>,
    from: Vec<CellId>,
    to: Vec<CellId>,
    delays: Vec<i64>,
    phases: Vec<i32>,
    couplings: Vec<i32>,
    source_generations: Vec<super::Generation>,
    generations: Vec<super::Generation>,
    resistances: Vec<u32>,
    live: Vec<bool>,
    eligible_until: Vec<Option<i64>>,
    #[cfg(feature = "cpc1")]
    participation_levels: Vec<u64>,
    #[cfg(feature = "cpc1")]
    plastic_supports: Vec<u64>,
    modes: Vec<super::TransmissionMode>,
}

impl ArrowColumns {
    fn from_values(values: Vec<Arrow>) -> Self {
        let mut columns = Self::default();
        for value in values {
            columns.push(value);
        }
        columns
    }

    fn get(&self, index: usize) -> Arrow {
        Arrow {
            id: self.ids[index],
            from: self.from[index],
            to: self.to[index],
            delay: self.delays[index],
            phase: self.phases[index],
            coupling: self.couplings[index],
            source_generation: self.source_generations[index],
            generation: self.generations[index],
            resistance: self.resistances[index],
            live: self.live[index],
            eligible_until: self.eligible_until[index],
            #[cfg(feature = "cpc1")]
            participation_level: self.participation_levels[index],
            #[cfg(feature = "cpc1")]
            plastic_support: self.plastic_supports[index],
            mode: self.modes[index],
        }
    }

    fn set(&mut self, index: usize, value: Arrow) {
        self.ids[index] = value.id;
        self.from[index] = value.from;
        self.to[index] = value.to;
        self.delays[index] = value.delay;
        self.phases[index] = value.phase;
        self.couplings[index] = value.coupling;
        self.source_generations[index] = value.source_generation;
        self.generations[index] = value.generation;
        self.resistances[index] = value.resistance;
        self.live[index] = value.live;
        self.eligible_until[index] = value.eligible_until;
        #[cfg(feature = "cpc1")]
        {
            self.participation_levels[index] = value.participation_level;
            self.plastic_supports[index] = value.plastic_support;
        }
        self.modes[index] = value.mode;
    }

    fn push(&mut self, value: Arrow) {
        self.ids.push(value.id);
        self.from.push(value.from);
        self.to.push(value.to);
        self.delays.push(value.delay);
        self.phases.push(value.phase);
        self.couplings.push(value.coupling);
        self.source_generations.push(value.source_generation);
        self.generations.push(value.generation);
        self.resistances.push(value.resistance);
        self.live.push(value.live);
        self.eligible_until.push(value.eligible_until);
        #[cfg(feature = "cpc1")]
        {
            self.participation_levels.push(value.participation_level);
            self.plastic_supports.push(value.plastic_support);
        }
        self.modes.push(value.mode);
    }

    fn resident_bytes(&self) -> usize {
        let bytes = self.ids.capacity() * std::mem::size_of::<ArrowId>()
            + self.from.capacity() * std::mem::size_of::<CellId>()
            + self.to.capacity() * std::mem::size_of::<CellId>()
            + self.delays.capacity() * std::mem::size_of::<i64>()
            + self.phases.capacity() * std::mem::size_of::<i32>()
            + self.couplings.capacity() * std::mem::size_of::<i32>()
            + self.source_generations.capacity() * std::mem::size_of::<super::Generation>()
            + self.generations.capacity() * std::mem::size_of::<super::Generation>()
            + self.resistances.capacity() * std::mem::size_of::<u32>()
            + self.live.capacity() * std::mem::size_of::<bool>()
            + self.eligible_until.capacity() * std::mem::size_of::<Option<i64>>()
            + self.modes.capacity() * std::mem::size_of::<super::TransmissionMode>();
        #[cfg(feature = "cpc1")]
        {
            bytes
                + self.participation_levels.capacity() * std::mem::size_of::<u64>()
                + self.plastic_supports.capacity() * std::mem::size_of::<u64>()
        }
        #[cfg(not(feature = "cpc1"))]
        {
            bytes
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchedulerKind {
    Vec,
    TimingWheel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum PendingSchedule {
    Vec(Vec<Spike>),
    TimingWheel(TimingWheel),
    PartitionedTimingWheels(PartitionedTimingWheels),
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
            Self::PartitionedTimingWheels(wheels) => wheels.len,
        }
    }

    pub(super) fn push(&mut self, spike: Spike, cost: &mut ExecutionCost) {
        cost.queue_ops = cost.queue_ops.saturating_add(1);
        cost.touch::<Spike>(1);
        match self {
            Self::Vec(spikes) => {
                if spikes.len() == spikes.capacity() {
                    cost.allocations = cost.allocations.saturating_add(1);
                }
                spikes.push(spike);
            }
            Self::TimingWheel(wheel) => wheel.push(spike, cost),
            Self::PartitionedTimingWheels(wheels) => wheels.push(spike, cost),
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
            Self::PartitionedTimingWheels(wheels) => wheels.pop_next(target_physical, cost),
        }
    }

    pub(super) fn pop_same_tick_batch<F>(
        &mut self,
        maximum: usize,
        target_physical: F,
        cost: &mut ExecutionCost,
    ) -> Vec<(Spike, u64)>
    where
        F: Fn(CellId) -> u64,
    {
        if maximum == 0 {
            return Vec::new();
        }
        match self {
            Self::Vec(spikes) => {
                cost.queue_ops = cost.queue_ops.saturating_add(1);
                let Some(index) = minimum_index(spikes, &target_physical, cost) else {
                    return Vec::new();
                };
                let comparisons = u64::try_from(spikes.len().saturating_sub(1)).unwrap_or(u64::MAX);
                vec![(spikes.remove(index), comparisons)]
            }
            Self::TimingWheel(wheel) => {
                cost.queue_ops = cost.queue_ops.saturating_add(1);
                wheel.pop_same_tick_batch(maximum, target_physical, cost)
            }
            Self::PartitionedTimingWheels(wheels) => {
                cost.queue_ops = cost.queue_ops.saturating_add(1);
                wheels.pop_same_tick_batch(maximum, target_physical, cost)
            }
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
            Self::PartitionedTimingWheels(wheels) => wheels
                .wheels
                .iter()
                .flat_map(TimingWheel::spikes)
                .cloned()
                .collect(),
        };
        spikes.sort_by_key(|spike| order_key(spike, &target_physical));
        spikes
    }

    pub(super) fn from_canonical(kind: SchedulerKind, head_tick: i64, spikes: Vec<Spike>) -> Self {
        let mut schedule = Self::new(kind, head_tick);
        let mut ignored = ExecutionCost::default();
        for spike in spikes {
            schedule.push(spike, &mut ignored);
        }
        schedule
    }

    pub(super) fn partitioned(
        head_tick: i64,
        cell_arenas: Vec<ResidentArenaId>,
        spikes: Vec<Spike>,
    ) -> Self {
        let mut schedule =
            Self::PartitionedTimingWheels(PartitionedTimingWheels::new(head_tick, cell_arenas));
        let mut ignored = ExecutionCost::default();
        for spike in spikes {
            schedule.push(spike, &mut ignored);
        }
        schedule
    }

    pub(super) fn is_partitioned(&self) -> bool {
        matches!(self, Self::PartitionedTimingWheels(_))
    }

    pub(super) fn resident_bytes(&self) -> usize {
        match self {
            Self::Vec(spikes) => spikes
                .capacity()
                .saturating_mul(std::mem::size_of::<Spike>()),
            Self::TimingWheel(wheel) => wheel.resident_bytes(),
            Self::PartitionedTimingWheels(wheels) => wheels.resident_bytes(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PartitionedTimingWheels {
    cell_arenas: Vec<ResidentArenaId>,
    arena_ids: Vec<ResidentArenaId>,
    wheels: Vec<TimingWheel>,
    len: usize,
}

impl PartitionedTimingWheels {
    fn new(head_tick: i64, cell_arenas: Vec<ResidentArenaId>) -> Self {
        let mut arena_ids = cell_arenas.clone();
        arena_ids.sort_unstable();
        arena_ids.dedup();
        let wheels = arena_ids
            .iter()
            .map(|_| TimingWheel::new(head_tick, DEFAULT_RING_WIDTH))
            .collect();
        Self {
            cell_arenas,
            arena_ids,
            wheels,
            len: 0,
        }
    }

    fn push(&mut self, spike: Spike, cost: &mut ExecutionCost) {
        let arena = self.cell_arenas[spike.target.0 as usize];
        let wheel = self
            .arena_ids
            .binary_search(&arena)
            .expect("resident arena must own scheduled CELL");
        cost.arena_lookups = cost.arena_lookups.saturating_add(1);
        self.wheels[wheel].push(spike, cost);
        self.len = self.len.saturating_add(1);
    }

    fn pop_next<F>(&mut self, target_physical: F, cost: &mut ExecutionCost) -> Option<(Spike, u64)>
    where
        F: Fn(CellId) -> u64,
    {
        let wheel = self.minimum_wheel(&target_physical, cost)?;
        let result = self.wheels[wheel].pop_next(target_physical, cost);
        if result.is_some() {
            self.len = self.len.saturating_sub(1);
        }
        result
    }

    fn pop_same_tick_batch<F>(
        &mut self,
        maximum: usize,
        target_physical: F,
        cost: &mut ExecutionCost,
    ) -> Vec<(Spike, u64)>
    where
        F: Fn(CellId) -> u64,
    {
        let mut batch = Vec::with_capacity(maximum.min(self.len));
        if batch.capacity() > 0 {
            cost.allocations = cost.allocations.saturating_add(1);
        }
        let Some(wheel) = self.minimum_wheel(&target_physical, cost) else {
            return batch;
        };
        let Some(first) = self.wheels[wheel].pop_next(&target_physical, cost) else {
            return batch;
        };
        self.len = self.len.saturating_sub(1);
        let tick = first.0.arrival_tick;
        batch.push(first);
        while batch.len() < maximum {
            let Some(wheel) = self.minimum_wheel(&target_physical, cost) else {
                break;
            };
            if self.wheels[wheel]
                .minimum_key(&target_physical, cost)
                .is_none_or(|key| key.0 != tick)
            {
                break;
            }
            let next = self.wheels[wheel]
                .pop_next(&target_physical, cost)
                .expect("selected resident timing wheel must pop");
            self.len = self.len.saturating_sub(1);
            batch.push(next);
        }
        batch
    }

    fn minimum_wheel<F>(&self, target_physical: &F, cost: &mut ExecutionCost) -> Option<usize>
    where
        F: Fn(CellId) -> u64,
    {
        let active = self.wheels.iter().filter(|wheel| wheel.len > 0).count();
        cost.observe_active_arenas(active);
        let mut selected = None;
        for (index, wheel) in self.wheels.iter().enumerate() {
            let Some(key) = wheel.minimum_key(target_physical, cost) else {
                continue;
            };
            cost.arena_lookups = cost.arena_lookups.saturating_add(1);
            if selected
                .as_ref()
                .is_none_or(|(_, current): &(usize, (i64, i32, u64, u64, u64))| key < *current)
            {
                selected = Some((index, key));
            }
        }
        selected.map(|(index, _)| index)
    }

    fn resident_bytes(&self) -> usize {
        self.cell_arenas.capacity() * std::mem::size_of::<ResidentArenaId>()
            + self.arena_ids.capacity() * std::mem::size_of::<ResidentArenaId>()
            + self.wheels.capacity() * std::mem::size_of::<TimingWheel>()
            + self
                .wheels
                .iter()
                .map(TimingWheel::resident_bytes)
                .sum::<usize>()
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

    fn push(&mut self, spike: Spike, cost: &mut ExecutionCost) {
        assert!(
            spike.arrival_tick >= self.head_tick,
            "scheduled activity cannot precede timing-wheel head"
        );
        if self.in_near_window(spike.arrival_tick) {
            let index = self.bucket_index(spike.arrival_tick);
            if self.near[index].len() == self.near[index].capacity() {
                cost.allocations = cost.allocations.saturating_add(1);
            }
            self.near[index].push(spike);
        } else {
            if self.overflow.len() == self.overflow.capacity() {
                cost.allocations = cost.allocations.saturating_add(1);
            }
            self.overflow.push(spike);
        }
        self.len += 1;
    }

    fn pop_next<F>(&mut self, target_physical: F, cost: &mut ExecutionCost) -> Option<(Spike, u64)>
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

    fn pop_same_tick_batch<F>(
        &mut self,
        maximum: usize,
        target_physical: F,
        cost: &mut ExecutionCost,
    ) -> Vec<(Spike, u64)>
    where
        F: Fn(CellId) -> u64,
    {
        if self.len == 0 {
            return Vec::new();
        }
        self.promote_overflow(cost);
        let Some(next_tick) = self
            .near
            .iter()
            .flat_map(|bucket| bucket.iter().map(|spike| spike.arrival_tick))
            .min()
            .or_else(|| self.overflow.iter().map(|spike| spike.arrival_tick).min())
        else {
            return Vec::new();
        };
        if next_tick >= self.head_tick.saturating_add(self.near.len() as i64) {
            self.head_tick = next_tick;
            self.promote_overflow(cost);
        } else {
            self.head_tick = next_tick;
        }
        let index = self.bucket_index(next_tick);
        let bucket = &mut self.near[index];
        bucket.sort_by_key(|spike| order_key(spike, &target_physical));
        let count = bucket
            .iter()
            .take_while(|spike| spike.arrival_tick == next_tick)
            .count()
            .min(maximum);
        cost.allocations = cost.allocations.saturating_add(1);
        self.len -= count;
        bucket.drain(..count).map(|spike| (spike, 0)).collect()
    }

    fn promote_overflow(&mut self, cost: &mut ExecutionCost) {
        let end = self.head_tick.saturating_add(self.near.len() as i64);
        let mut retained = Vec::with_capacity(self.overflow.len());
        if !self.overflow.is_empty() {
            cost.allocations = cost.allocations.saturating_add(1);
        }
        for spike in self.overflow.drain(..) {
            cost.scans = cost.scans.saturating_add(1);
            cost.touch::<Spike>(1);
            if spike.arrival_tick < end {
                let offset = spike.arrival_tick.rem_euclid(self.near.len() as i64) as usize;
                if self.near[offset].len() == self.near[offset].capacity() {
                    cost.allocations = cost.allocations.saturating_add(1);
                }
                self.near[offset].push(spike);
            } else {
                retained.push(spike);
            }
        }
        self.overflow = retained;
    }

    fn spikes(&self) -> impl Iterator<Item = &Spike> {
        self.near
            .iter()
            .flat_map(|bucket| bucket.iter())
            .chain(self.overflow.iter())
    }

    fn minimum_key<F>(
        &self,
        target_physical: &F,
        cost: &mut ExecutionCost,
    ) -> Option<(i64, i32, u64, u64, u64)>
    where
        F: Fn(CellId) -> u64,
    {
        let mut selected = None;
        for spike in self.spikes() {
            cost.touch::<Spike>(1);
            let key = order_key(spike, target_physical);
            if selected.is_some() {
                cost.comparisons = cost.comparisons.saturating_add(1);
            }
            if selected.is_none_or(|current| key < current) {
                selected = Some(key);
            }
        }
        selected
    }

    fn resident_bytes(&self) -> usize {
        self.near.capacity() * std::mem::size_of::<Vec<Spike>>()
            + self
                .near
                .iter()
                .map(|bucket| bucket.capacity() * std::mem::size_of::<Spike>())
                .sum::<usize>()
            + self.overflow.capacity() * std::mem::size_of::<Spike>()
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
        cost.touch::<Spike>(2);
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
