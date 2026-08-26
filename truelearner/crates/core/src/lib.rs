#![forbid(unsafe_code)]
//! Single-file physical runtime: state, local transitions, and crossings.

use std::collections::{HashSet, VecDeque};
use truelearner_arena_format::{
    ArenaBody, ArenaVersion, BodyVersion, DurableArrow, DurableCell, FormatError,
};

mod mechanics {
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

        pub(super) fn with_mut<R>(
            &mut self,
            index: usize,
            change: impl FnOnce(&mut Cell) -> R,
        ) -> R {
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
        #[cfg(feature = "cl0")]
        decay_loads: Vec<u64>,
        #[cfg(feature = "cc0")]
        participation_levels: Vec<u64>,
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
                #[cfg(feature = "cl0")]
                decay_load: self.decay_loads[index],
                #[cfg(feature = "cc0")]
                participation_level: self.participation_levels[index],
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
            #[cfg(feature = "cl0")]
            {
                self.decay_loads[index] = value.decay_load;
            }
            #[cfg(feature = "cc0")]
            {
                self.participation_levels[index] = value.participation_level;
            }
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
            #[cfg(feature = "cl0")]
            self.decay_loads.push(value.decay_load);
            #[cfg(feature = "cc0")]
            self.participation_levels.push(value.participation_level);
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
                + if cfg!(feature = "cl0") {
                    #[cfg(feature = "cl0")]
                    {
                        self.decay_loads.capacity() * std::mem::size_of::<u64>()
                    }
                    #[cfg(not(feature = "cl0"))]
                    {
                        0
                    }
                } else {
                    0
                }
                + if cfg!(feature = "cc0") {
                    #[cfg(feature = "cc0")]
                    {
                        self.participation_levels.capacity() * std::mem::size_of::<u64>()
                    }
                    #[cfg(not(feature = "cc0"))]
                    {
                        0
                    }
                } else {
                    0
                }
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

        pub(super) fn with_mut<R>(
            &mut self,
            index: usize,
            change: impl FnOnce(&mut Arrow) -> R,
        ) -> R {
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
        #[cfg(feature = "cl0")]
        target_generations: Vec<super::Generation>,
        generations: Vec<super::Generation>,
        resistances: Vec<u32>,
        live: Vec<bool>,
        participation_levels: Vec<u64>,
        plastic_supports: Vec<u64>,
        decay_loads: Vec<u64>,
        modes: Vec<super::TransmissionMode>,
        triggers: Vec<super::TransmissionTrigger>,
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
                #[cfg(feature = "cl0")]
                target_generation: self.target_generations[index],
                generation: self.generations[index],
                resistance: self.resistances[index],
                live: self.live[index],
                participation_level: self.participation_levels[index],
                plastic_support: self.plastic_supports[index],
                decay_load: self.decay_loads[index],
                mode: self.modes[index],
                trigger: self.triggers[index],
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
            #[cfg(feature = "cl0")]
            {
                self.target_generations[index] = value.target_generation;
            }
            self.generations[index] = value.generation;
            self.resistances[index] = value.resistance;
            self.live[index] = value.live;
            self.participation_levels[index] = value.participation_level;
            self.plastic_supports[index] = value.plastic_support;
            self.decay_loads[index] = value.decay_load;
            self.modes[index] = value.mode;
            self.triggers[index] = value.trigger;
        }

        fn push(&mut self, value: Arrow) {
            self.ids.push(value.id);
            self.from.push(value.from);
            self.to.push(value.to);
            self.delays.push(value.delay);
            self.phases.push(value.phase);
            self.couplings.push(value.coupling);
            self.source_generations.push(value.source_generation);
            #[cfg(feature = "cl0")]
            self.target_generations.push(value.target_generation);
            self.generations.push(value.generation);
            self.resistances.push(value.resistance);
            self.live.push(value.live);
            self.participation_levels.push(value.participation_level);
            self.plastic_supports.push(value.plastic_support);
            self.decay_loads.push(value.decay_load);
            self.modes.push(value.mode);
            self.triggers.push(value.trigger);
        }

        fn resident_bytes(&self) -> usize {
            self.ids.capacity() * std::mem::size_of::<ArrowId>()
                + self.from.capacity() * std::mem::size_of::<CellId>()
                + self.to.capacity() * std::mem::size_of::<CellId>()
                + self.delays.capacity() * std::mem::size_of::<i64>()
                + self.phases.capacity() * std::mem::size_of::<i32>()
                + self.couplings.capacity() * std::mem::size_of::<i32>()
                + self.source_generations.capacity() * std::mem::size_of::<super::Generation>()
                + if cfg!(feature = "cl0") {
                    #[cfg(feature = "cl0")]
                    {
                        self.target_generations.capacity()
                            * std::mem::size_of::<super::Generation>()
                    }
                    #[cfg(not(feature = "cl0"))]
                    {
                        0
                    }
                } else {
                    0
                }
                + self.generations.capacity() * std::mem::size_of::<super::Generation>()
                + self.resistances.capacity() * std::mem::size_of::<u32>()
                + self.live.capacity() * std::mem::size_of::<bool>()
                + self.modes.capacity() * std::mem::size_of::<super::TransmissionMode>()
                + self.participation_levels.capacity() * std::mem::size_of::<u64>()
                + self.plastic_supports.capacity() * std::mem::size_of::<u64>()
                + self.decay_loads.capacity() * std::mem::size_of::<u64>()
                + self.triggers.capacity() * std::mem::size_of::<super::TransmissionTrigger>()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SchedulerKind {
        Vec,
        TimingWheel,
    }

    #[cfg(feature = "si0")]
    type CausalOrderKey = (i64, i32, u64, u64);
    #[cfg(not(feature = "si0"))]
    type CausalOrderKey = (i64, i32, u64);

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

        pub(super) fn pop_next(&mut self, cost: &mut ExecutionCost) -> Option<(Spike, u64)> {
            cost.queue_ops = cost.queue_ops.saturating_add(1);
            match self {
                Self::Vec(spikes) => {
                    let index = minimum_index(spikes, cost)?;
                    let comparisons =
                        u64::try_from(spikes.len().saturating_sub(1)).unwrap_or(u64::MAX);
                    Some((spikes.remove(index), comparisons))
                }
                Self::TimingWheel(wheel) => wheel.pop_next(cost),
                Self::PartitionedTimingWheels(wheels) => wheels.pop_next(cost),
            }
        }

        #[cfg(not(feature = "si0"))]
        pub(super) fn pop_same_tick_batch(
            &mut self,
            maximum: usize,
            cost: &mut ExecutionCost,
        ) -> Vec<(Spike, u64)> {
            if maximum == 0 {
                return Vec::new();
            }
            match self {
                Self::Vec(spikes) => {
                    cost.queue_ops = cost.queue_ops.saturating_add(1);
                    let Some(index) = minimum_index(spikes, cost) else {
                        return Vec::new();
                    };
                    let comparisons =
                        u64::try_from(spikes.len().saturating_sub(1)).unwrap_or(u64::MAX);
                    vec![(spikes.remove(index), comparisons)]
                }
                Self::TimingWheel(wheel) => {
                    cost.queue_ops = cost.queue_ops.saturating_add(1);
                    wheel.pop_same_tick_batch(maximum, cost)
                }
                Self::PartitionedTimingWheels(wheels) => {
                    cost.queue_ops = cost.queue_ops.saturating_add(1);
                    wheels.pop_same_tick_batch(maximum, cost)
                }
            }
        }

        #[cfg(feature = "si0")]
        pub(super) fn drain_minimum_wave(&mut self, cost: &mut ExecutionCost) -> Vec<(Spike, u64)> {
            let Some(first) = self.pop_next(cost) else {
                return Vec::new();
            };
            let prefix = (first.0.arrival_tick, first.0.phase, first.0.causal_wave);
            let mut batch = vec![first];
            while self
                .minimum_key(cost)
                .is_some_and(|key| (key.0, key.1, key.2) == prefix)
            {
                batch.push(
                    self.pop_next(cost)
                        .expect("peeked causal wave must remain available"),
                );
            }
            batch
        }

        #[cfg(feature = "si0")]
        fn minimum_key(&self, cost: &mut ExecutionCost) -> Option<CausalOrderKey> {
            match self {
                Self::Vec(spikes) => {
                    let mut selected = None;
                    for spike in spikes {
                        cost.touch::<Spike>(1);
                        let key = causal_order_key(spike);
                        if selected.is_some() {
                            cost.comparisons = cost.comparisons.saturating_add(1);
                        }
                        if selected.is_none_or(|current| key < current) {
                            selected = Some(key);
                        }
                    }
                    selected
                }
                Self::TimingWheel(wheel) => wheel.minimum_key(cost),
                Self::PartitionedTimingWheels(wheels) => {
                    let wheel = wheels.minimum_wheel(cost)?;
                    wheels.wheels[wheel].minimum_key(cost)
                }
            }
        }

        pub(super) fn canonical(&self) -> Vec<Spike> {
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
            spikes.sort_by_key(canonical_storage_key);
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

        fn pop_next(&mut self, cost: &mut ExecutionCost) -> Option<(Spike, u64)> {
            let wheel = self.minimum_wheel(cost)?;
            let result = self.wheels[wheel].pop_next(cost);
            if result.is_some() {
                self.len = self.len.saturating_sub(1);
            }
            result
        }

        #[cfg(not(feature = "si0"))]
        fn pop_same_tick_batch(
            &mut self,
            maximum: usize,
            cost: &mut ExecutionCost,
        ) -> Vec<(Spike, u64)> {
            let mut batch = Vec::with_capacity(maximum.min(self.len));
            if batch.capacity() > 0 {
                cost.allocations = cost.allocations.saturating_add(1);
            }
            let Some(wheel) = self.minimum_wheel(cost) else {
                return batch;
            };
            let Some(first) = self.wheels[wheel].pop_next(cost) else {
                return batch;
            };
            self.len = self.len.saturating_sub(1);
            let tick = first.0.arrival_tick;
            batch.push(first);
            while batch.len() < maximum {
                let Some(wheel) = self.minimum_wheel(cost) else {
                    break;
                };
                if self.wheels[wheel]
                    .minimum_key(cost)
                    .is_none_or(|key| key.0 != tick)
                {
                    break;
                }
                let next = self.wheels[wheel]
                    .pop_next(cost)
                    .expect("selected resident timing wheel must pop");
                self.len = self.len.saturating_sub(1);
                batch.push(next);
            }
            batch
        }

        fn minimum_wheel(&self, cost: &mut ExecutionCost) -> Option<usize> {
            let active = self.wheels.iter().filter(|wheel| wheel.len > 0).count();
            cost.observe_active_arenas(active);
            let mut selected = None;
            for (index, wheel) in self.wheels.iter().enumerate() {
                let Some(key) = wheel.minimum_key(cost) else {
                    continue;
                };
                cost.arena_lookups = cost.arena_lookups.saturating_add(1);
                if selected
                    .as_ref()
                    .is_none_or(|(_, current): &(usize, CausalOrderKey)| key < *current)
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

        fn pop_next(&mut self, cost: &mut ExecutionCost) -> Option<(Spike, u64)> {
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
            let selected = minimum_index(bucket, cost)
                .expect("next timing-wheel bucket must contain an arrival");
            let spike = bucket.remove(selected);
            self.len -= 1;
            let comparisons = u64::try_from(before.saturating_sub(1)).unwrap_or(u64::MAX);
            Some((spike, comparisons))
        }

        #[cfg(not(feature = "si0"))]
        fn pop_same_tick_batch(
            &mut self,
            maximum: usize,
            cost: &mut ExecutionCost,
        ) -> Vec<(Spike, u64)> {
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
            bucket.sort_by_key(causal_order_key);
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

        fn minimum_key(&self, cost: &mut ExecutionCost) -> Option<CausalOrderKey> {
            let mut selected = None;
            for spike in self.spikes() {
                cost.touch::<Spike>(1);
                let key = causal_order_key(spike);
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

    fn minimum_index(spikes: &[Spike], cost: &mut ExecutionCost) -> Option<usize> {
        let mut first = 0;
        if spikes.is_empty() {
            return None;
        }
        for candidate in 1..spikes.len() {
            cost.comparisons = cost.comparisons.saturating_add(1);
            cost.touch::<Spike>(2);
            if causal_order_key(&spikes[candidate]) < causal_order_key(&spikes[first]) {
                first = candidate;
            }
        }
        Some(first)
    }

    fn causal_order_key(spike: &Spike) -> CausalOrderKey {
        #[cfg(feature = "si0")]
        {
            (
                spike.arrival_tick,
                spike.phase,
                spike.causal_wave,
                spike.serial,
            )
        }
        #[cfg(not(feature = "si0"))]
        {
            (spike.arrival_tick, spike.phase, spike.serial)
        }
    }

    fn canonical_storage_key(spike: &Spike) -> (i64, i32, u64, u64, u64, u64) {
        // AH0_STORAGE_ONLY: canonical checkpoint/debug order, never transition order.
        (
            spike.arrival_tick,
            spike.phase,
            #[cfg(feature = "si0")]
            spike.causal_wave,
            #[cfg(not(feature = "si0"))]
            0,
            spike.origin_physical,
            spike.target.0,
            spike.serial,
        )
    }
}
pub use mechanics::SchedulerKind;
use mechanics::{ArrowStore, CellStore, PendingSchedule};
pub use truelearner_arena_format::{
    ArenaId, ArrowId, ArrowRef, CellId, CellRef, ContentHash, Generation,
};

const LOCAL_RETURN_STRENGTH: u32 = 3;
const LOCAL_DECAY_PERIOD: i64 = 10;
const LOCAL_VARIATION_RADIUS: i32 = 2;
const PARTICIPATION_IMPULSE: u64 = 1_u64 << 32;
const PARTICIPATION_RELAX_NUMERATOR: u64 = 15;
const PARTICIPATION_RELAX_DENOMINATOR: u64 = 16;
#[cfg(feature = "core0")]
const MATERIAL_ONE: i64 = 1_i64 << 32;
#[cfg(feature = "core0")]
const MATERIAL_ONE_U64: u64 = 1_u64 << 32;
const DEFAULT_CELL_CAPACITY: u32 = 65_536;
const DEFAULT_ARROW_CAPACITY: u32 = 262_144;
#[cfg(feature = "core1")]
const CHECKPOINT_VERSION: u16 = 6;
#[cfg(all(feature = "si0", not(feature = "core1")))]
const CHECKPOINT_VERSION: u16 = 5;
#[cfg(all(not(feature = "si0"), not(feature = "cl0")))]
const CHECKPOINT_VERSION: u16 = 2;
#[cfg(all(feature = "cl0", not(feature = "cc0"), not(feature = "si0")))]
const CHECKPOINT_VERSION: u16 = 3;
#[cfg(all(feature = "cc0", not(feature = "si0")))]
const CHECKPOINT_VERSION: u16 = 4;
const QUIESCENT_MAGIC: &[u8; 8] = b"TLQUIE01";
const LIVE_MAGIC: &[u8; 8] = b"TLLIVE01";
const BOUNDARY_LIVE_MAGIC: &[u8; 8] = b"TLBNDY01";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CellSlot(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArrowSlot(pub usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResidentArenaId(pub u32);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhysicalClock {
    pub tick: i64,
}

impl PhysicalClock {
    pub fn pressure_phase(self) -> i64 {
        self.tick.rem_euclid(LOCAL_DECAY_PERIOD)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellSpec {
    pub physical_id: u64,
    pub position: i32,
    pub region: i16,
    pub threshold: i32,
    pub resistance: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransmissionMode {
    Drive,
    Modulatory,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TransmissionTrigger {
    #[default]
    SourceFires,
    QualifiedLocalParticipation,
}

#[cfg(feature = "core0")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Core0Profile {
    #[default]
    A,
    B,
    C,
    D,
    #[cfg(feature = "core1")]
    GenericExternal,
    #[cfg(feature = "core1")]
    GenericActivity,
    #[cfg(feature = "core1")]
    GenericDistance,
    #[cfg(feature = "core1")]
    GenericDistanceNoQlp,
}

#[cfg(feature = "core0")]
impl Core0Profile {
    fn continuous(self) -> bool {
        !matches!(self, Self::A)
    }

    fn contact_variation(self) -> bool {
        matches!(self, Self::A | Self::B)
    }

    fn proposal_on_every_firing(self) -> bool {
        if self == Self::D {
            return true;
        }
        #[cfg(feature = "core1")]
        {
            return matches!(
                self,
                Self::GenericActivity | Self::GenericDistance | Self::GenericDistanceNoQlp
            );
        }
        #[cfg(not(feature = "core1"))]
        false
    }

    #[cfg(feature = "core1")]
    fn generic_variation(self) -> bool {
        matches!(
            self,
            Self::GenericExternal
                | Self::GenericActivity
                | Self::GenericDistance
                | Self::GenericDistanceNoQlp
        )
    }

    #[cfg(feature = "core1")]
    fn distance_graded_variation(self) -> bool {
        matches!(self, Self::GenericDistance | Self::GenericDistanceNoQlp)
    }

    #[cfg(feature = "core1")]
    fn qlp_enabled(self) -> bool {
        self != Self::GenericDistanceNoQlp
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraversalKind {
    GlobalScan,
    Adjacency,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityKind {
    FullScan,
    Frontier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutKind {
    AoS,
    SoA,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutorKind {
    Scalar,
    Batched,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MechanicalConfig {
    pub scheduler: SchedulerKind,
    pub traversal: TraversalKind,
    pub activity: ActivityKind,
    pub layout: LayoutKind,
    pub executor: ExecutorKind,
}

impl MechanicalConfig {
    pub const REFERENCE: Self = Self {
        scheduler: SchedulerKind::Vec,
        traversal: TraversalKind::GlobalScan,
        activity: ActivityKind::FullScan,
        layout: LayoutKind::AoS,
        executor: ExecutorKind::Scalar,
    };

    pub const R1: Self = Self {
        scheduler: SchedulerKind::TimingWheel,
        ..Self::REFERENCE
    };

    pub const R2: Self = Self {
        traversal: TraversalKind::Adjacency,
        ..Self::R1
    };

    pub const R3: Self = Self {
        activity: ActivityKind::Frontier,
        ..Self::R2
    };

    pub const R4: Self = Self {
        layout: LayoutKind::SoA,
        ..Self::R3
    };

    pub const R5: Self = Self {
        executor: ExecutorKind::Batched,
        ..Self::R4
    };

    /// PSEL0's measured production selection. The permanent correctness
    /// reference remains `REFERENCE`; batching falls back safely when live
    /// zero-delay topology can add current-tick work.
    pub const PRODUCTION: Self = Self {
        executor: ExecutorKind::Batched,
        ..Self::R3
    };
}

impl Default for MechanicalConfig {
    fn default() -> Self {
        Self::REFERENCE
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArrowSpec {
    pub from: CellId,
    pub to: CellId,
    pub delay: i64,
    pub phase: i32,
    pub coupling: i32,
    pub resistance: u32,
    pub mode: TransmissionMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpikeInput {
    pub arrival_tick: i64,
    pub phase: i32,
    pub origin_physical: u64,
    pub target: CellId,
    pub impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cell {
    id: CellId,
    physical_id: u64,
    position: i32,
    region: i16,
    threshold: i32,
    state: i32,
    last_update_tick: i64,
    refractory_until: i64,
    generation: Generation,
    resistance: u32,
    live: bool,
    #[cfg(feature = "cl0")]
    decay_load: u64,
    #[cfg(feature = "cc0")]
    participation_level: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Arrow {
    id: ArrowId,
    from: CellId,
    to: CellId,
    delay: i64,
    phase: i32,
    coupling: i32,
    source_generation: Generation,
    #[cfg(feature = "cl0")]
    target_generation: Generation,
    generation: Generation,
    resistance: u32,
    live: bool,
    participation_level: u64,
    plastic_support: u64,
    decay_load: u64,
    mode: TransmissionMode,
    trigger: TransmissionTrigger,
}

#[cfg(not(feature = "si0"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PhysicalArrowOrder {
    phase: i32,
    delay: i64,
    from_live: bool,
    from_position: i32,
    from_region: i16,
    from_threshold: i32,
    to_live: bool,
    to_position: i32,
    to_region: i16,
    to_threshold: i32,
    mode: u8,
    trigger: u8,
    coupling: i32,
    resistance: u32,
    participation: u64,
    plastic_support: u64,
    decay_load: u64,
    source_generation: u32,
    target_generation: u32,
    arrow_generation: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Spike {
    arrival_tick: i64,
    phase: i32,
    #[cfg(feature = "si0")]
    causal_wave: u64,
    origin_physical: u64,
    #[cfg(feature = "cl0")]
    target_physical: u64,
    target: CellId,
    target_generation: Generation,
    impulse: i32,
    #[cfg(feature = "core0")]
    material_impulse: i64,
    serial: u64,
    arrow: Option<(ArrowId, Generation)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crossing {
    pub tick: i64,
    pub from_physical: u64,
    pub to_physical: u64,
    pub from_region: i16,
    pub to_region: i16,
    pub impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PhysicalEvent {
    #[cfg(feature = "si0")]
    DriveIncidence {
        target: CellId,
        arrivals: u32,
        impulse: i32,
        causal_wave: u64,
    },
    #[cfg(feature = "si0")]
    ModulatoryIncidence {
        target: CellId,
        arrivals: u32,
        impulse: i32,
        causal_wave: u64,
    },
    #[cfg(feature = "core0")]
    MaterialDriveIncidence {
        target: CellId,
        impulse: i64,
        activation_after: i64,
        causal_wave: u64,
    },
    Deliver {
        mode: TransmissionMode,
        target: CellId,
        impulse: i32,
    },
    Fire {
        cell: CellId,
    },
    Resistance {
        arrow: ArrowId,
        before: u32,
        after: u32,
    },
    #[cfg(feature = "ce0")]
    Coupling {
        arrow: ArrowId,
        before: i32,
        after: i32,
    },
    Deallocate {
        arrow: ArrowId,
    },
    #[cfg(feature = "cl0")]
    CellDeallocate {
        cell: CellId,
        before_generation: Generation,
        after_generation: Generation,
    },
    #[cfg(feature = "cc0")]
    CellResistance {
        cell: CellId,
        before: u32,
        after: u32,
    },
    #[cfg(any(feature = "cv0", feature = "cv0j0"))]
    CellProposal {
        cell: CellId,
        source: CellId,
        target: CellId,
    },
    Proposal {
        arrow: ArrowId,
        from: CellId,
        to: CellId,
    },
    Crossing(Crossing),
    QualifiedLocalTraversal {
        arrow: ArrowId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalTransition {
    pub tick: i64,
    pub phase: i32,
    pub event: PhysicalEvent,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    total: u64,
    pub drive_deliveries: u64,
    pub modulatory_deliveries: u64,
    pub local_return_updates: u64,
    pub local_structural_proposals: u64,
    pub physical_deallocations: u64,
    #[cfg(feature = "cl0")]
    pub cell_deallocations: u64,
    #[cfg(feature = "cc0")]
    pub cell_return_updates: u64,
    #[cfg(any(feature = "cv0", feature = "cv0j0"))]
    pub local_cell_proposals: u64,
    pub qualified_local_traversals: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExecutionCost {
    pub queue_ops: u64,
    pub comparisons: u64,
    pub scans: u64,
    pub allocations: u64,
    pub bytes_touched: u64,
    pub peak_resident_bytes: u64,
    pub adjacency_accesses: u64,
    pub frontier_samples: u64,
    pub active_frontier_total: u64,
    pub active_frontier_max: u64,
    pub batches: u64,
    pub batched_items: u64,
    pub batch_max: u64,
    pub batch_histogram: [u64; 7],
    pub batch_fallback_zero_delay: u64,
    pub arena_lookups: u64,
    pub arena_hops: u64,
    pub active_arena_samples: u64,
    pub active_arena_total: u64,
    pub active_arena_max: u64,
}

impl ExecutionCost {
    pub(crate) fn touch<T>(&mut self, count: usize) {
        self.bytes_touched = self.bytes_touched.saturating_add(
            u64::try_from(std::mem::size_of::<T>().saturating_mul(count)).unwrap_or(u64::MAX),
        );
    }

    fn observe_resident_bytes(&mut self, bytes: usize) {
        self.peak_resident_bytes = self
            .peak_resident_bytes
            .max(u64::try_from(bytes).unwrap_or(u64::MAX));
    }

    fn observe_frontier(&mut self, active: usize) {
        let active = u64::try_from(active).unwrap_or(u64::MAX);
        self.frontier_samples = self.frontier_samples.saturating_add(1);
        self.active_frontier_total = self.active_frontier_total.saturating_add(active);
        self.active_frontier_max = self.active_frontier_max.max(active);
    }

    fn observe_batch(&mut self, size: usize) {
        let size = u64::try_from(size).unwrap_or(u64::MAX);
        self.batches = self.batches.saturating_add(1);
        self.batched_items = self.batched_items.saturating_add(size);
        self.batch_max = self.batch_max.max(size);
        let bucket = match size {
            0 | 1 => 0,
            2 => 1,
            3..=4 => 2,
            5..=8 => 3,
            9..=16 => 4,
            17..=32 => 5,
            _ => 6,
        };
        self.batch_histogram[bucket] = self.batch_histogram[bucket].saturating_add(1);
    }

    fn observe_active_arenas(&mut self, active: usize) {
        let active = u64::try_from(active).unwrap_or(u64::MAX);
        self.active_arena_samples = self.active_arena_samples.saturating_add(1);
        self.active_arena_total = self.active_arena_total.saturating_add(active);
        self.active_arena_max = self.active_arena_max.max(active);
    }
}

impl Work {
    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn physical_total(&self) -> u64 {
        let total = self
            .drive_deliveries
            .saturating_add(self.modulatory_deliveries)
            .saturating_add(self.local_return_updates)
            .saturating_add(self.local_structural_proposals)
            .saturating_add(self.physical_deallocations);
        #[cfg(feature = "cl0")]
        let total = total.saturating_add(self.cell_deallocations);
        #[cfg(feature = "cc0")]
        let total = total.saturating_add(self.cell_return_updates);
        #[cfg(any(feature = "cv0", feature = "cv0j0"))]
        let total = total.saturating_add(self.local_cell_proposals);
        total.saturating_add(self.qualified_local_traversals)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunResult {
    pub crossings: Vec<Crossing>,
    pub work: Work,
    pub naturally_quiescent: bool,
    pub resident_bytes: usize,
    pub execution_cost: ExecutionCost,
    pub physical_trace: Vec<PhysicalTransition>,
}

#[cfg(feature = "rs0")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservedRun {
    pub run: RunResult,
    pub scheduled_deliveries: u64,
    pub observation_ceiling_reached: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundaryError {
    ZeroCapacity,
    InputFull {
        capacity: usize,
        occupied: usize,
        attempted: usize,
    },
    OutputFull {
        capacity: usize,
        occupied: usize,
        required: usize,
    },
    OutputBatchTooLarge {
        capacity: usize,
        required: usize,
    },
    WrongOutwardRegion {
        configured: i16,
        requested: i16,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundaryRun {
    pub consumed_inputs: usize,
    pub produced_outputs: usize,
    pub work: Work,
    pub naturally_quiescent: bool,
    pub resident_bytes: usize,
    pub execution_cost: ExecutionCost,
    pub physical_trace: Vec<PhysicalTransition>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundaryLiveCheckpoint {
    core: LiveCheckpoint,
    outward_region: i16,
    input_capacity: usize,
    output_capacity: usize,
    inputs: Vec<SpikeInput>,
    outputs: Vec<Crossing>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundaryRuntime {
    substrate: PlasticSubstrate,
    outward_region: i16,
    input_capacity: usize,
    output_capacity: usize,
    inputs: VecDeque<SpikeInput>,
    outputs: VecDeque<Crossing>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlasticSubstrate {
    mechanics: MechanicalConfig,
    arena: ArenaId,
    cells: CellStore,
    cell_slots: Vec<Option<CellSlot>>,
    arrows: ArrowStore,
    arrow_slots: Vec<Option<ArrowSlot>>,
    cell_capacity: u32,
    arrow_capacity: u32,
    pending: PendingSchedule,
    tick: i64,
    next_serial: u64,
    pressure_tick: i64,
    pending_loads: Vec<PendingLoad>,
    outgoing_index: Vec<Vec<ArrowId>>,
    resident_arenas: Vec<ResidentArenaId>,
    active_cells: HashSet<CellId>,
    trace_physics: bool,
    zero_delay_live_arrows: usize,
    #[cfg(feature = "core1")]
    in_flight_protection: bool,
    #[cfg(feature = "core1")]
    protect_all_live_arrows: bool,
    #[cfg(feature = "core1")]
    in_flight_arrows: Vec<u32>,
    #[cfg(feature = "core1")]
    capture_used_pending: bool,
    #[cfg(feature = "core1")]
    used_pending_arrows: Vec<bool>,
    #[cfg(feature = "core1")]
    used_pending_protection_enabled: bool,
    #[cfg(feature = "core1")]
    temporary_credit_return_arrows: Vec<bool>,
    #[cfg(feature = "core1")]
    atomic_credit_return_source: Option<CellId>,
    #[cfg(feature = "core1")]
    atomic_credit_return_capture: bool,
    #[cfg(feature = "core0")]
    core0_profile: Core0Profile,
    #[cfg(feature = "core0")]
    core0_activation: Vec<i64>,
    #[cfg(feature = "core0")]
    core0_coupling: Vec<i64>,
    #[cfg(feature = "core0")]
    core0_resistance: Vec<u64>,
    #[cfg(feature = "core0")]
    core0_decay_remainder: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingLoad {
    pub arena: ArenaId,
    pub version: ContentHash,
    pub issue_tick: i64,
    pub availability_tick: Option<i64>,
    pub waiting_arrivals: Vec<SpikeInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuiescentCheckpoint {
    pub body_version: BodyVersion,
    pub body: ArenaBody,
    pub clock: PhysicalClock,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveCheckpoint {
    body_version: BodyVersion,
    body: ArenaBody,
    clock: PhysicalClock,
    cells: Vec<CellRuntime>,
    arrows: Vec<ArrowRuntime>,
    pending: Vec<Spike>,
    next_serial: u64,
    pending_loads: Vec<PendingLoad>,
    #[cfg(feature = "core1")]
    core0_profile: Core0Profile,
    #[cfg(feature = "core1")]
    in_flight_protection: bool,
    #[cfg(feature = "core1")]
    protect_all_live_arrows: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CellRuntime {
    id: CellId,
    state: i32,
    last_update_tick: i64,
    refractory_until: i64,
    #[cfg(feature = "cl0")]
    decay_load: u64,
    #[cfg(feature = "cc0")]
    participation_level: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ArrowRuntime {
    id: ArrowId,
    participation_level: u64,
    plastic_support: u64,
    decay_load: u64,
    trigger: TransmissionTrigger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckpointError {
    NotQuiescent,
    Format(FormatError),
    UnsupportedTransmissionMode(u8),
    MissingCell(CellId),
    MissingArrow(ArrowId),
    ManifestMismatch,
    InvalidPhysicalBody,
    StaleCellReference(CellRef),
    Truncated,
    WrongMagic,
    UnsupportedCheckpointVersion(u16),
    InvalidCheckpoint,
    Checksum,
    TrailingBytes,
}

impl From<FormatError> for CheckpointError {
    fn from(error: FormatError) -> Self {
        Self::Format(error)
    }
}

impl QuiescentCheckpoint {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CheckpointError> {
        validate_manifest(&self.body_version, &self.body)?;
        let manifest = self.body_version.canonical_bytes()?;
        let body = self.body.canonical_bytes()?;
        let mut payload = Vec::with_capacity(manifest.len() + body.len());
        payload.extend_from_slice(&manifest);
        payload.extend_from_slice(&body);
        let mut bytes = Vec::with_capacity(66 + payload.len());
        bytes.extend_from_slice(QUIESCENT_MAGIC);
        checkpoint_put_u16(&mut bytes, CHECKPOINT_VERSION);
        checkpoint_put_i64(&mut bytes, self.clock.tick);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(manifest.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(body.len())?);
        bytes.extend_from_slice(ContentHash::of(&payload).as_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, CheckpointError> {
        if bytes.len() < 66 {
            return Err(CheckpointError::Truncated);
        }
        if &bytes[..8] != QUIESCENT_MAGIC {
            return Err(CheckpointError::WrongMagic);
        }
        let mut cursor = CheckpointCursor::new(bytes, 8);
        let version = cursor.u16()?;
        if version != CHECKPOINT_VERSION {
            return Err(CheckpointError::UnsupportedCheckpointVersion(version));
        }
        let tick = cursor.i64()?;
        let manifest_len = cursor.usize_from_u64()?;
        let body_len = cursor.usize_from_u64()?;
        let checksum = ContentHash(cursor.array_32()?);
        let payload_len = manifest_len
            .checked_add(body_len)
            .ok_or(CheckpointError::InvalidCheckpoint)?;
        let payload = cursor.bytes(payload_len)?;
        cursor.finish()?;
        if ContentHash::of(payload) != checksum {
            return Err(CheckpointError::Checksum);
        }
        let body_version = BodyVersion::decode(&payload[..manifest_len])?;
        let body = ArenaBody::decode(&payload[manifest_len..])?;
        validate_manifest(&body_version, &body)?;
        Ok(Self {
            body_version,
            body,
            clock: PhysicalClock { tick },
        })
    }
}

impl LiveCheckpoint {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CheckpointError> {
        validate_manifest(&self.body_version, &self.body)?;
        let manifest = self.body_version.canonical_bytes()?;
        let body = self.body.canonical_bytes()?;
        let mut cells = self.cells.clone();
        // AH0_STORAGE_ONLY: canonical checkpoint bytes.
        cells.sort_by_key(|cell| cell.id);
        let mut arrows = self.arrows.clone();
        // AH0_STORAGE_ONLY: canonical checkpoint bytes.
        arrows.sort_by_key(|arrow| arrow.id);
        let mut pending = self.pending.clone();
        pending.sort_by_key(|spike| {
            (
                spike.arrival_tick,
                spike.phase,
                #[cfg(feature = "si0")]
                spike.causal_wave,
                spike.origin_physical,
                #[cfg(not(feature = "cl0"))]
                spike.target.0,
                #[cfg(feature = "cl0")]
                spike.target_physical,
                spike.serial,
            )
        });
        let mut loads = self.pending_loads.clone();
        loads.sort_by_key(|load| {
            (
                load.availability_tick.unwrap_or(i64::MAX),
                load.issue_tick,
                load.arena,
            )
        });

        let mut payload = Vec::new();
        payload.extend_from_slice(&manifest);
        payload.extend_from_slice(&body);
        #[cfg(feature = "core1")]
        {
            payload.push(core0_profile_byte(self.core0_profile));
            payload.push(u8::from(self.in_flight_protection));
            payload.push(u8::from(self.protect_all_live_arrows));
        }
        for cell in &cells {
            checkpoint_put_u64(&mut payload, cell.id.0);
            checkpoint_put_i32(&mut payload, cell.state);
            checkpoint_put_i64(&mut payload, cell.last_update_tick);
            checkpoint_put_i64(&mut payload, cell.refractory_until);
            #[cfg(feature = "cl0")]
            checkpoint_put_u64(&mut payload, cell.decay_load);
            #[cfg(feature = "cc0")]
            checkpoint_put_u64(&mut payload, cell.participation_level);
        }
        for arrow in &arrows {
            checkpoint_put_u64(&mut payload, arrow.id.0);
            checkpoint_put_u64(&mut payload, arrow.participation_level);
            checkpoint_put_u64(&mut payload, arrow.plastic_support);
            checkpoint_put_u64(&mut payload, arrow.decay_load);
            payload.push(transmission_trigger_byte(arrow.trigger));
        }
        for spike in &pending {
            encode_spike(&mut payload, spike);
        }
        for load in &loads {
            checkpoint_put_u64(&mut payload, load.arena.0);
            payload.extend_from_slice(load.version.as_bytes());
            checkpoint_put_i64(&mut payload, load.issue_tick);
            checkpoint_put_optional_tick(&mut payload, load.availability_tick);
            checkpoint_put_u32(
                &mut payload,
                checkpoint_len_u32(load.waiting_arrivals.len())?,
            );
            for input in &load.waiting_arrivals {
                encode_input(&mut payload, input);
            }
        }

        let mut bytes = Vec::with_capacity(98 + payload.len());
        bytes.extend_from_slice(LIVE_MAGIC);
        checkpoint_put_u16(&mut bytes, CHECKPOINT_VERSION);
        checkpoint_put_i64(&mut bytes, self.clock.tick);
        checkpoint_put_u64(&mut bytes, self.next_serial);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(manifest.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(body.len())?);
        checkpoint_put_u32(&mut bytes, checkpoint_len_u32(cells.len())?);
        checkpoint_put_u32(&mut bytes, checkpoint_len_u32(arrows.len())?);
        checkpoint_put_u32(&mut bytes, checkpoint_len_u32(pending.len())?);
        checkpoint_put_u32(&mut bytes, checkpoint_len_u32(loads.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(payload.len())?);
        bytes.extend_from_slice(ContentHash::of(&payload).as_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, CheckpointError> {
        if bytes.len() < 98 {
            return Err(CheckpointError::Truncated);
        }
        if &bytes[..8] != LIVE_MAGIC {
            return Err(CheckpointError::WrongMagic);
        }
        let mut cursor = CheckpointCursor::new(bytes, 8);
        let version = cursor.u16()?;
        if version != CHECKPOINT_VERSION {
            return Err(CheckpointError::UnsupportedCheckpointVersion(version));
        }
        let tick = cursor.i64()?;
        let next_serial = cursor.u64()?;
        let manifest_len = cursor.usize_from_u64()?;
        let body_len = cursor.usize_from_u64()?;
        let cell_count = cursor.usize_from_u32()?;
        let arrow_count = cursor.usize_from_u32()?;
        let pending_count = cursor.usize_from_u32()?;
        let load_count = cursor.usize_from_u32()?;
        let payload_len = cursor.usize_from_u64()?;
        let checksum = ContentHash(cursor.array_32()?);
        let payload = cursor.bytes(payload_len)?;
        cursor.finish()?;
        if ContentHash::of(payload) != checksum {
            return Err(CheckpointError::Checksum);
        }
        let structural_len = manifest_len
            .checked_add(body_len)
            .ok_or(CheckpointError::InvalidCheckpoint)?;
        if structural_len > payload.len() {
            return Err(CheckpointError::Truncated);
        }
        let body_version = BodyVersion::decode(&payload[..manifest_len])?;
        let body = ArenaBody::decode(&payload[manifest_len..structural_len])?;
        validate_manifest(&body_version, &body)?;
        let mut transient = CheckpointCursor::new(payload, structural_len);
        #[cfg(feature = "core1")]
        let core0_profile = core0_profile_from_byte(transient.u8()?)?;
        #[cfg(feature = "core1")]
        let in_flight_protection = match transient.u8()? {
            0 => false,
            1 => true,
            _ => return Err(CheckpointError::InvalidCheckpoint),
        };
        #[cfg(feature = "core1")]
        let protect_all_live_arrows = match transient.u8()? {
            0 => false,
            1 => true,
            _ => return Err(CheckpointError::InvalidCheckpoint),
        };
        let mut cells = Vec::with_capacity(cell_count);
        for _ in 0..cell_count {
            cells.push(CellRuntime {
                id: CellId(transient.u64()?),
                state: transient.i32()?,
                last_update_tick: transient.i64()?,
                refractory_until: transient.i64()?,
                #[cfg(feature = "cl0")]
                decay_load: transient.u64()?,
                #[cfg(feature = "cc0")]
                participation_level: transient.u64()?,
            });
        }
        let mut arrows = Vec::with_capacity(arrow_count);
        for _ in 0..arrow_count {
            arrows.push(ArrowRuntime {
                id: ArrowId(transient.u64()?),
                participation_level: transient.u64()?,
                plastic_support: transient.u64()?,
                decay_load: transient.u64()?,
                trigger: transmission_trigger_from_byte(transient.u8()?)?,
            });
        }
        let mut pending = Vec::with_capacity(pending_count);
        for _ in 0..pending_count {
            pending.push(decode_spike(&mut transient)?);
        }
        let mut pending_loads = Vec::with_capacity(load_count);
        for _ in 0..load_count {
            let arena = ArenaId(transient.u64()?);
            let version = ContentHash(transient.array_32()?);
            let issue_tick = transient.i64()?;
            let availability_tick = transient.optional_tick()?;
            let waiting_count = transient.usize_from_u32()?;
            let mut waiting_arrivals = Vec::with_capacity(waiting_count);
            for _ in 0..waiting_count {
                waiting_arrivals.push(decode_input(&mut transient)?);
            }
            pending_loads.push(PendingLoad {
                arena,
                version,
                issue_tick,
                availability_tick,
                waiting_arrivals,
            });
        }
        transient.finish()?;
        Ok(Self {
            body_version,
            body,
            clock: PhysicalClock { tick },
            cells,
            arrows,
            pending,
            next_serial,
            pending_loads,
            #[cfg(feature = "core1")]
            core0_profile,
            #[cfg(feature = "core1")]
            in_flight_protection,
            #[cfg(feature = "core1")]
            protect_all_live_arrows,
        })
    }
}

impl BoundaryLiveCheckpoint {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CheckpointError> {
        if self.input_capacity == 0
            || self.output_capacity == 0
            || self.inputs.len() > self.input_capacity
            || self.outputs.len() > self.output_capacity
        {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        let core = self.core.canonical_bytes()?;
        let mut payload = Vec::with_capacity(
            core.len()
                .saturating_add((self.inputs.len() + self.outputs.len()).saturating_mul(32)),
        );
        payload.extend_from_slice(&core);
        for input in &self.inputs {
            encode_input(&mut payload, input);
        }
        for crossing in &self.outputs {
            encode_crossing(&mut payload, crossing);
        }

        let mut bytes = Vec::with_capacity(92 + payload.len());
        bytes.extend_from_slice(BOUNDARY_LIVE_MAGIC);
        checkpoint_put_u16(&mut bytes, CHECKPOINT_VERSION);
        checkpoint_put_i16(&mut bytes, self.outward_region);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(self.input_capacity)?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(self.output_capacity)?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(self.inputs.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(self.outputs.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(core.len())?);
        checkpoint_put_u64(&mut bytes, checkpoint_len_u64(payload.len())?);
        bytes.extend_from_slice(ContentHash::of(&payload).as_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, CheckpointError> {
        if bytes.len() < 92 {
            return Err(CheckpointError::Truncated);
        }
        if &bytes[..8] != BOUNDARY_LIVE_MAGIC {
            return Err(CheckpointError::WrongMagic);
        }
        let mut cursor = CheckpointCursor::new(bytes, 8);
        let version = cursor.u16()?;
        if version != CHECKPOINT_VERSION {
            return Err(CheckpointError::UnsupportedCheckpointVersion(version));
        }
        let outward_region = cursor.i16()?;
        let input_capacity = cursor.usize_from_u64()?;
        let output_capacity = cursor.usize_from_u64()?;
        let input_count = cursor.usize_from_u64()?;
        let output_count = cursor.usize_from_u64()?;
        let core_len = cursor.usize_from_u64()?;
        let payload_len = cursor.usize_from_u64()?;
        let checksum = ContentHash(cursor.array_32()?);
        let payload = cursor.bytes(payload_len)?;
        cursor.finish()?;
        if ContentHash::of(payload) != checksum {
            return Err(CheckpointError::Checksum);
        }
        if input_capacity == 0
            || output_capacity == 0
            || input_count > input_capacity
            || output_count > output_capacity
        {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        let entries = input_count
            .checked_add(output_count)
            .and_then(|count| count.checked_mul(32))
            .ok_or(CheckpointError::InvalidCheckpoint)?;
        let expected = core_len
            .checked_add(entries)
            .ok_or(CheckpointError::InvalidCheckpoint)?;
        if expected != payload_len || core_len > payload.len() {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        let core = LiveCheckpoint::decode(&payload[..core_len])?;
        let mut transient = CheckpointCursor::new(payload, core_len);
        let mut inputs = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            inputs.push(decode_input(&mut transient)?);
        }
        let mut outputs = Vec::with_capacity(output_count);
        for _ in 0..output_count {
            outputs.push(decode_crossing(&mut transient)?);
        }
        transient.finish()?;
        Ok(Self {
            core,
            outward_region,
            input_capacity,
            output_capacity,
            inputs,
            outputs,
        })
    }
}

impl BoundaryRuntime {
    pub fn new(
        substrate: PlasticSubstrate,
        outward_region: i16,
        input_capacity: usize,
        output_capacity: usize,
    ) -> Result<Self, BoundaryError> {
        if input_capacity == 0 || output_capacity == 0 {
            return Err(BoundaryError::ZeroCapacity);
        }
        Ok(Self {
            substrate,
            outward_region,
            input_capacity,
            output_capacity,
            inputs: VecDeque::with_capacity(input_capacity),
            outputs: VecDeque::with_capacity(output_capacity),
        })
    }

    pub fn substrate(&self) -> &PlasticSubstrate {
        &self.substrate
    }

    #[cfg(feature = "core1")]
    pub fn add_experimental_cell(&mut self, spec: CellSpec) -> CellId {
        self.substrate.add_cell(spec)
    }

    pub fn add_arrow_with_trigger(
        &mut self,
        spec: ArrowSpec,
        trigger: TransmissionTrigger,
    ) -> ArrowId {
        self.substrate.add_arrow_with_trigger(spec, trigger)
    }

    #[cfg(feature = "core0")]
    pub fn set_core0_profile(&mut self, profile: Core0Profile) {
        self.substrate.set_core0_profile(profile);
    }

    #[cfg(feature = "core1")]
    pub fn set_in_flight_protection(&mut self, enabled: bool) {
        self.substrate.set_in_flight_protection(enabled);
    }

    #[cfg(feature = "core1")]
    pub fn set_all_live_arrow_protection(&mut self, enabled: bool) {
        self.substrate.set_all_live_arrow_protection(enabled);
    }

    #[cfg(feature = "core1")]
    pub fn set_used_pending_capture(&mut self, enabled: bool) {
        self.substrate.set_used_pending_capture(enabled);
    }

    #[cfg(feature = "core1")]
    pub fn set_used_pending_protection(&mut self, enabled: bool) {
        self.substrate.set_used_pending_protection(enabled);
    }

    #[cfg(feature = "core1")]
    pub fn clear_used_pending(&mut self) {
        self.substrate.clear_used_pending();
    }

    #[cfg(feature = "core1")]
    pub fn used_pending_count(&self) -> usize {
        self.substrate.used_pending_count()
    }

    #[cfg(feature = "core1")]
    pub fn add_temporary_credit_return(&mut self, spec: ArrowSpec) -> ArrowId {
        self.substrate.add_temporary_credit_return(spec)
    }

    #[cfg(feature = "core1")]
    pub fn clear_temporary_credit_returns(&mut self) {
        self.substrate.clear_temporary_credit_returns();
    }

    #[cfg(feature = "core1")]
    pub fn temporary_credit_return_count(&self) -> usize {
        self.substrate.temporary_credit_return_count()
    }

    #[cfg(feature = "core1")]
    pub fn configure_atomic_credit_return(&mut self, returning: CellId) {
        self.substrate.configure_atomic_credit_return(returning);
    }

    #[cfg(feature = "core1")]
    pub fn set_atomic_credit_return_capture(&mut self, enabled: bool) {
        self.substrate.set_atomic_credit_return_capture(enabled);
    }

    /// Changes only the mechanical execution strategy beneath the boundary.
    /// Physical law and buffered activity are preserved exactly.
    pub fn reconfigure_mechanics(&mut self, mechanics: MechanicalConfig) {
        self.substrate.reconfigure_mechanics(mechanics);
    }

    /// Reassigns resident execution placement without changing durable identity.
    pub fn repartition_resident(&mut self, placements: &[ResidentArenaId]) {
        self.substrate.repartition_resident(placements);
    }

    pub fn input_capacity(&self) -> usize {
        self.input_capacity
    }

    pub fn output_capacity(&self) -> usize {
        self.output_capacity
    }

    pub fn input_len(&self) -> usize {
        self.inputs.len()
    }

    pub fn output_len(&self) -> usize {
        self.outputs.len()
    }

    pub fn enqueue(&mut self, input: SpikeInput) -> Result<(), BoundaryError> {
        self.enqueue_batch(&[input])
    }

    pub fn enqueue_batch(&mut self, inputs: &[SpikeInput]) -> Result<(), BoundaryError> {
        let occupied = self.inputs.len();
        if inputs.len() > self.input_capacity.saturating_sub(occupied) {
            return Err(BoundaryError::InputFull {
                capacity: self.input_capacity,
                occupied,
                attempted: inputs.len(),
            });
        }
        self.inputs.extend(inputs.iter().copied());
        Ok(())
    }

    pub fn run_until_quiescent(&mut self) -> Result<BoundaryRun, BoundaryError> {
        let consumed_inputs = self.inputs.len();
        let mut candidate = self.substrate.clone();
        for input in &self.inputs {
            candidate.enter(*input);
        }
        let mut result = candidate.propagate();
        result
            .crossings
            .retain(|crossing| crossing.to_region == self.outward_region);
        let required = result.crossings.len();
        if required > self.output_capacity {
            return Err(BoundaryError::OutputBatchTooLarge {
                capacity: self.output_capacity,
                required,
            });
        }
        let occupied = self.outputs.len();
        if required > self.output_capacity.saturating_sub(occupied) {
            return Err(BoundaryError::OutputFull {
                capacity: self.output_capacity,
                occupied,
                required,
            });
        }
        self.substrate = candidate;
        self.inputs.clear();
        self.outputs.extend(result.crossings);
        Ok(BoundaryRun {
            consumed_inputs,
            produced_outputs: required,
            work: result.work,
            naturally_quiescent: result.naturally_quiescent,
            resident_bytes: result.resident_bytes,
            execution_cost: result.execution_cost,
            physical_trace: result.physical_trace,
        })
    }

    pub fn drain_output(&mut self, maximum: usize) -> Vec<Crossing> {
        let count = maximum.min(self.outputs.len());
        self.outputs.drain(..count).collect()
    }

    pub fn drain_all_output(&mut self) -> Vec<Crossing> {
        self.drain_output(self.outputs.len())
    }

    pub fn arrive(
        &mut self,
        inputs: &[SpikeInput],
        outward_region: i16,
    ) -> Result<RunResult, BoundaryError> {
        if outward_region != self.outward_region {
            return Err(BoundaryError::WrongOutwardRegion {
                configured: self.outward_region,
                requested: outward_region,
            });
        }
        self.enqueue_batch(inputs)?;
        let run = self.run_until_quiescent()?;
        Ok(RunResult {
            crossings: self.drain_all_output(),
            work: run.work,
            naturally_quiescent: run.naturally_quiescent,
            resident_bytes: run.resident_bytes,
            execution_cost: run.execution_cost,
            physical_trace: run.physical_trace,
        })
    }

    pub fn advance_time(&mut self, tick: i64) -> Work {
        assert!(
            self.inputs.is_empty() && self.outputs.is_empty(),
            "boundary buffers must be drained before advancing time"
        );
        self.substrate.advance_time(tick)
    }

    pub fn live_checkpoint(
        &self,
        body_version: u64,
    ) -> Result<BoundaryLiveCheckpoint, CheckpointError> {
        Ok(BoundaryLiveCheckpoint {
            core: self.substrate.live_checkpoint(body_version)?,
            outward_region: self.outward_region,
            input_capacity: self.input_capacity,
            output_capacity: self.output_capacity,
            inputs: self.inputs.iter().copied().collect(),
            outputs: self.outputs.iter().copied().collect(),
        })
    }

    pub fn from_live_checkpoint(
        checkpoint: BoundaryLiveCheckpoint,
    ) -> Result<Self, CheckpointError> {
        if checkpoint.input_capacity == 0
            || checkpoint.output_capacity == 0
            || checkpoint.inputs.len() > checkpoint.input_capacity
            || checkpoint.outputs.len() > checkpoint.output_capacity
        {
            return Err(CheckpointError::InvalidCheckpoint);
        }
        Ok(Self {
            substrate: PlasticSubstrate::from_live_checkpoint(checkpoint.core)?,
            outward_region: checkpoint.outward_region,
            input_capacity: checkpoint.input_capacity,
            output_capacity: checkpoint.output_capacity,
            inputs: checkpoint.inputs.into(),
            outputs: checkpoint.outputs.into(),
        })
    }
}

impl Default for PlasticSubstrate {
    fn default() -> Self {
        Self::with_capacity(ArenaId(0), DEFAULT_CELL_CAPACITY, DEFAULT_ARROW_CAPACITY)
    }
}

impl PlasticSubstrate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(arena: ArenaId, cell_capacity: u32, arrow_capacity: u32) -> Self {
        Self::with_mechanics(
            arena,
            cell_capacity,
            arrow_capacity,
            MechanicalConfig::REFERENCE,
        )
    }

    pub fn with_mechanics(
        arena: ArenaId,
        cell_capacity: u32,
        arrow_capacity: u32,
        mechanics: MechanicalConfig,
    ) -> Self {
        Self {
            mechanics,
            arena,
            cells: CellStore::new(mechanics.layout),
            cell_slots: Vec::new(),
            arrows: ArrowStore::new(mechanics.layout),
            arrow_slots: Vec::new(),
            cell_capacity,
            arrow_capacity,
            pending: PendingSchedule::new(mechanics.scheduler, 0),
            tick: 0,
            next_serial: 0,
            pressure_tick: 0,
            pending_loads: Vec::new(),
            outgoing_index: Vec::new(),
            resident_arenas: Vec::new(),
            active_cells: HashSet::new(),
            trace_physics: false,
            zero_delay_live_arrows: 0,
            #[cfg(feature = "core1")]
            in_flight_protection: false,
            #[cfg(feature = "core1")]
            protect_all_live_arrows: false,
            #[cfg(feature = "core1")]
            in_flight_arrows: Vec::new(),
            #[cfg(feature = "core1")]
            capture_used_pending: false,
            #[cfg(feature = "core1")]
            used_pending_arrows: Vec::new(),
            #[cfg(feature = "core1")]
            used_pending_protection_enabled: true,
            #[cfg(feature = "core1")]
            temporary_credit_return_arrows: Vec::new(),
            #[cfg(feature = "core1")]
            atomic_credit_return_source: None,
            #[cfg(feature = "core1")]
            atomic_credit_return_capture: false,
            #[cfg(feature = "core0")]
            core0_profile: Core0Profile::A,
            #[cfg(feature = "core0")]
            core0_activation: Vec::new(),
            #[cfg(feature = "core0")]
            core0_coupling: Vec::new(),
            #[cfg(feature = "core0")]
            core0_resistance: Vec::new(),
            #[cfg(feature = "core0")]
            core0_decay_remainder: Vec::new(),
        }
    }

    #[cfg(feature = "core0")]
    pub fn set_core0_profile(&mut self, profile: Core0Profile) {
        assert!(
            self.pending.is_empty(),
            "CORE0 profile changes require quiescence"
        );
        self.core0_profile = profile;
        self.core0_activation.resize(self.cell_slots.len(), 0);
        self.core0_coupling
            .resize(self.arrow_slots.len(), MATERIAL_ONE);
        self.core0_resistance
            .resize(self.arrow_slots.len(), MATERIAL_ONE_U64);
        self.core0_decay_remainder.resize(self.arrow_slots.len(), 0);
        for cell in self.cells.values() {
            let index = cell.id.0 as usize;
            self.core0_activation[index] = i64::from(cell.state).saturating_mul(MATERIAL_ONE);
        }
        for arrow in self.arrows.values() {
            let index = arrow.id.0 as usize;
            self.core0_coupling[index] = i64::from(arrow.coupling).saturating_mul(MATERIAL_ONE);
            self.core0_resistance[index] =
                u64::from(arrow.resistance).saturating_mul(MATERIAL_ONE_U64);
        }
    }

    #[cfg(feature = "core0")]
    pub fn core0_profile(&self) -> Core0Profile {
        self.core0_profile
    }

    #[cfg(feature = "core1")]
    pub fn set_in_flight_protection(&mut self, enabled: bool) {
        self.in_flight_protection = enabled;
    }

    #[cfg(feature = "core1")]
    pub fn set_all_live_arrow_protection(&mut self, enabled: bool) {
        self.protect_all_live_arrows = enabled;
    }

    #[cfg(feature = "core1")]
    pub fn in_flight_count(&self, id: ArrowId) -> u32 {
        self.in_flight_arrows
            .get(id.0 as usize)
            .copied()
            .unwrap_or(0)
    }

    #[cfg(feature = "core1")]
    pub fn set_used_pending_capture(&mut self, enabled: bool) {
        self.capture_used_pending = enabled;
    }

    #[cfg(feature = "core1")]
    pub fn set_used_pending_protection(&mut self, enabled: bool) {
        self.used_pending_protection_enabled = enabled;
    }

    #[cfg(feature = "core1")]
    pub fn clear_used_pending(&mut self) {
        self.used_pending_arrows.fill(false);
    }

    #[cfg(feature = "core1")]
    pub fn used_pending_count(&self) -> usize {
        self.used_pending_arrows
            .iter()
            .filter(|pending| **pending)
            .count()
    }

    #[cfg(feature = "core1")]
    pub fn used_pending(&self, id: ArrowId) -> bool {
        self.used_pending_arrows
            .get(id.0 as usize)
            .copied()
            .unwrap_or(false)
    }

    #[cfg(feature = "core1")]
    pub fn add_temporary_credit_return(&mut self, spec: ArrowSpec) -> ArrowId {
        assert_eq!(spec.mode, TransmissionMode::Modulatory);
        let id = self.add_arrow(spec);
        self.temporary_credit_return_arrows[id.0 as usize] = true;
        id
    }

    #[cfg(feature = "core1")]
    pub fn clear_temporary_credit_returns(&mut self) {
        for index in 0..self.temporary_credit_return_arrows.len() {
            if !self.temporary_credit_return_arrows[index] {
                continue;
            }
            self.temporary_credit_return_arrows[index] = false;
            let id = ArrowId(u64::try_from(index).unwrap_or(u64::MAX));
            let Some(slot) = self.arrow_slot(id) else {
                continue;
            };
            let snapshot = self.arrows.get(slot.0);
            if !snapshot.live {
                continue;
            }
            if snapshot.delay == 0 {
                self.zero_delay_live_arrows = self.zero_delay_live_arrows.saturating_sub(1);
            }
            #[cfg(feature = "core0")]
            {
                self.core0_resistance[index] = 0;
                self.core0_decay_remainder[index] = 0;
            }
            self.arrows
                .with_mut(slot.0, |arrow| decay_arrow(arrow, u32::MAX));
        }
    }

    #[cfg(feature = "core1")]
    pub fn temporary_credit_return_count(&self) -> usize {
        self.temporary_credit_return_arrows
            .iter()
            .filter(|temporary| **temporary)
            .count()
    }

    #[cfg(feature = "core1")]
    pub fn configure_atomic_credit_return(&mut self, returning: CellId) {
        self.require_cell(returning);
        self.atomic_credit_return_source = Some(returning);
    }

    #[cfg(feature = "core1")]
    pub fn set_atomic_credit_return_capture(&mut self, enabled: bool) {
        self.atomic_credit_return_capture = enabled;
    }

    #[cfg(feature = "core1")]
    fn maybe_create_atomic_credit_return(&mut self, participating: ArrowId) {
        if !self.atomic_credit_return_capture {
            return;
        }
        let Some(returning) = self.atomic_credit_return_source else {
            return;
        };
        let Some(slot) = self.arrow_slot(participating) else {
            return;
        };
        let route = self.arrows.get(slot.0);
        if !route.live
            || route.mode != TransmissionMode::Drive
            || route.trigger != TransmissionTrigger::SourceFires
        {
            return;
        }
        let Some(contact_slot) = self.cell_slot(route.from) else {
            return;
        };
        let contact = self.cells.get(contact_slot.0);
        let contact_id = contact.id;
        let contact_position = contact.position;
        let subdivision = self.arrows.values().iter().any(|stem| {
            stem.live
                && stem.mode == TransmissionMode::Drive
                && stem.trigger == TransmissionTrigger::SourceFires
                && stem.to == contact_id
                && stem.from != contact_id
                && self.cell_slot(stem.from).is_some_and(|source_slot| {
                    self.cells.get(source_slot.0).position == contact_position
                })
        });
        if !subdivision {
            return;
        }
        let already_live = self
            .temporary_credit_return_arrows
            .iter()
            .enumerate()
            .filter(|(_, temporary)| **temporary)
            .any(|(index, _)| {
                let id = ArrowId(u64::try_from(index).unwrap_or(u64::MAX));
                self.arrow_slot(id).is_some_and(|connection_slot| {
                    let connection = self.arrows.get(connection_slot.0);
                    connection.live && connection.from == returning && connection.to == contact_id
                })
            });
        if already_live {
            return;
        }
        self.add_temporary_credit_return(ArrowSpec {
            from: returning,
            to: contact_id,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: u32::MAX,
            mode: TransmissionMode::Modulatory,
        });
    }

    #[cfg(feature = "core1")]
    fn protected_by_temporary_credit_return(&self, candidate: &Arrow) -> bool {
        if candidate.mode != TransmissionMode::Drive || candidate.participation_level == 0 {
            return false;
        }
        self.temporary_credit_return_arrows
            .iter()
            .enumerate()
            .filter(|(_, temporary)| **temporary)
            .any(|(index, _)| {
                let id = ArrowId(u64::try_from(index).unwrap_or(u64::MAX));
                self.arrow_slot(id).is_some_and(|slot| {
                    let connection = self.arrows.get(slot.0);
                    connection.live
                        && (candidate.from == connection.to || candidate.to == connection.to)
                })
            })
    }

    pub fn mechanical_config(&self) -> MechanicalConfig {
        self.mechanics
    }

    pub fn set_physical_tracing(&mut self, enabled: bool) {
        self.trace_physics = enabled;
    }

    pub fn reconfigure_mechanics(&mut self, mechanics: MechanicalConfig) {
        let was_partitioned = self.pending.is_partitioned();
        let pending = self.pending.canonical();
        self.cells.convert(mechanics.layout);
        self.arrows.convert(mechanics.layout);
        self.pending = if was_partitioned {
            PendingSchedule::partitioned(self.tick, self.resident_arenas.clone(), pending)
        } else {
            PendingSchedule::from_canonical(mechanics.scheduler, self.tick, pending)
        };
        self.mechanics = mechanics;
        self.rebuild_slot_maps();
        self.rebuild_mechanical_indexes();
    }

    pub fn repartition_resident(&mut self, placements: &[ResidentArenaId]) {
        assert_eq!(
            placements.len(),
            self.cell_slots.len(),
            "resident partition must assign every CELL identity"
        );
        let pending = self.pending.canonical();
        self.resident_arenas = placements.to_vec();
        self.pending =
            PendingSchedule::partitioned(self.tick, self.resident_arenas.clone(), pending);
    }

    pub fn resident_arena(&self, cell: CellId) -> ResidentArenaId {
        self.require_cell(cell);
        self.resident_arenas[cell.0 as usize]
    }

    pub fn resident_arena_count(&self) -> usize {
        self.resident_arenas
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn clock(&self) -> PhysicalClock {
        PhysicalClock { tick: self.tick }
    }

    pub fn resolve_cell(&self, reference: CellRef) -> Option<CellSlot> {
        if reference.arena != self.arena {
            return None;
        }
        let slot = self.cell_slot(reference.id)?;
        let cell = self.cells.get(slot.0);
        (cell.id == reference.id && cell.live && cell.generation == reference.generation)
            .then_some(slot)
    }

    pub fn resolve_arrow(&self, reference: ArrowRef) -> Option<ArrowSlot> {
        if reference.arena != self.arena {
            return None;
        }
        let slot = self.arrow_slot(reference.id)?;
        let arrow = self.arrows.get(slot.0);
        (arrow.live && arrow.generation == reference.generation).then_some(slot)
    }

    pub fn add_cell(&mut self, spec: CellSpec) -> CellId {
        assert!(spec.threshold > 0, "threshold must be physically positive");
        assert!(
            self.cells
                .values()
                .iter()
                .all(|cell| cell.physical_id != spec.physical_id),
            "physical cell identity must be unique"
        );
        #[cfg(feature = "cl0")]
        let reusable = self.cells.values().iter().position(|cell| !cell.live);
        #[cfg(not(feature = "cl0"))]
        let reusable: Option<usize> = None;
        assert!(
            reusable.is_some() || self.cells.len() < self.cell_capacity as usize,
            "resident arena has no free CELL slot"
        );
        let id = CellId(self.cell_slots.len() as u64);
        let (slot, generation, resident_arena) = reusable.map_or_else(
            || {
                (
                    CellSlot(self.cells.len()),
                    Generation(1),
                    ResidentArenaId(0),
                )
            },
            |index| {
                let prior = self.cells.get(index);
                (
                    CellSlot(index),
                    prior.generation,
                    self.resident_arenas[prior.id.0 as usize],
                )
            },
        );
        let cell = Cell {
            id,
            physical_id: spec.physical_id,
            position: spec.position,
            region: spec.region,
            threshold: spec.threshold,
            state: 0,
            last_update_tick: self.tick,
            refractory_until: self.tick,
            generation,
            resistance: spec.resistance,
            live: spec.resistance > 0,
            #[cfg(feature = "cl0")]
            decay_load: 0,
            #[cfg(feature = "cc0")]
            participation_level: 0,
        };
        if slot.0 < self.cells.len() {
            self.cells.with_mut(slot.0, |resident| *resident = cell);
        } else {
            self.cells.push(cell);
        }
        self.cell_slots.push((spec.resistance > 0).then_some(slot));
        self.outgoing_index.push(Vec::new());
        self.resident_arenas.push(resident_arena);
        #[cfg(feature = "core0")]
        {
            self.core0_activation.push(0);
        }
        id
    }

    pub fn add_arrow(&mut self, spec: ArrowSpec) -> ArrowId {
        self.require_cell(spec.from);
        self.require_cell(spec.to);
        assert!(spec.delay >= 0, "delay must not run backward in time");
        let source_slot = self
            .cell_slot(spec.from)
            .expect("required CELL must resolve");
        let source_generation = self.cells.get(source_slot.0).generation;
        #[cfg(feature = "cl0")]
        let target_generation = self
            .cells
            .get(
                self.cell_slot(spec.to)
                    .expect("required CELL must resolve")
                    .0,
            )
            .generation;
        let reusable = self.arrows.values().iter().position(|arrow| !arrow.live);
        if reusable.is_none() {
            assert!(
                self.arrow_slots.len() < self.arrow_capacity as usize,
                "resident arena has no free ARROW identity"
            );
        }
        let (id, slot, generation, prior_source) = if let Some(index) = reusable {
            let prior = self.arrows.get(index);
            (
                prior.id,
                ArrowSlot(index),
                prior.generation,
                Some(prior.from),
            )
        } else {
            let id = ArrowId(self.arrow_slots.len() as u64);
            (id, ArrowSlot(self.arrows.len()), Generation(1), None)
        };
        let arrow = Arrow {
            id,
            from: spec.from,
            to: spec.to,
            delay: spec.delay,
            phase: spec.phase,
            coupling: spec.coupling,
            source_generation,
            #[cfg(feature = "cl0")]
            target_generation,
            generation,
            resistance: spec.resistance,
            live: spec.resistance > 0,
            participation_level: 0,
            plastic_support: 0,
            decay_load: 0,
            mode: spec.mode,
            trigger: TransmissionTrigger::SourceFires,
        };
        if slot.0 < self.arrows.len() {
            if let Some(prior_source) = prior_source {
                self.outgoing_index[prior_source.0 as usize].retain(|candidate| *candidate != id);
            }
            self.arrows.set(slot.0, arrow);
            self.arrow_slots[id.0 as usize] = Some(slot);
        } else {
            self.arrows.push(arrow);
            self.arrow_slots.push(Some(slot));
        }
        #[cfg(feature = "core1")]
        {
            let required = id.0 as usize + 1;
            if self.in_flight_arrows.len() < required {
                self.in_flight_arrows.resize(required, 0);
            }
            self.in_flight_arrows[id.0 as usize] = 0;
            if self.used_pending_arrows.len() < required {
                self.used_pending_arrows.resize(required, false);
            }
            self.used_pending_arrows[id.0 as usize] = false;
            if self.temporary_credit_return_arrows.len() < required {
                self.temporary_credit_return_arrows.resize(required, false);
            }
            self.temporary_credit_return_arrows[id.0 as usize] = false;
        }
        let outgoing = &mut self.outgoing_index[spec.from.0 as usize];
        outgoing.push(id);
        #[cfg(feature = "core0")]
        {
            let index = id.0 as usize;
            if self.core0_coupling.len() <= index {
                self.core0_coupling.resize(index + 1, 0);
                self.core0_resistance.resize(index + 1, 0);
                self.core0_decay_remainder.resize(index + 1, 0);
            }
            self.core0_coupling[index] = i64::from(spec.coupling).saturating_mul(MATERIAL_ONE);
            self.core0_resistance[index] =
                u64::from(spec.resistance).saturating_mul(MATERIAL_ONE_U64);
            self.core0_decay_remainder[index] = 0;
        }
        if spec.resistance > 0 && spec.delay == 0 {
            self.zero_delay_live_arrows = self.zero_delay_live_arrows.saturating_add(1);
        }
        id
    }

    pub fn add_arrow_with_trigger(
        &mut self,
        spec: ArrowSpec,
        trigger: TransmissionTrigger,
    ) -> ArrowId {
        assert!(
            trigger == TransmissionTrigger::SourceFires
                || spec.mode == TransmissionMode::Modulatory,
            "qualified local transmission must have Modulatory effect"
        );
        let id = self.add_arrow(spec);
        let slot = self.arrow_slot(id).expect("new ARROW must resolve");
        self.arrows
            .with_mut(slot.0, |arrow| arrow.trigger = trigger);
        id
    }

    pub fn transmission_trigger(&self, id: ArrowId) -> TransmissionTrigger {
        self.arrows
            .get(self.arrow_slot(id).expect("ARROW identity must resolve").0)
            .trigger
    }

    fn enqueue_physical_spike(&mut self, spike: Spike, cost: &mut ExecutionCost) {
        #[cfg(feature = "core1")]
        if let Some((arrow, generation)) = spike.arrow {
            if self
                .arrow_slot(arrow)
                .is_some_and(|slot| self.arrows.get(slot.0).generation == generation)
            {
                let index = arrow.0 as usize;
                if self.in_flight_arrows.len() <= index {
                    self.in_flight_arrows.resize(index + 1, 0);
                }
                self.in_flight_arrows[index] = self.in_flight_arrows[index].saturating_add(1);
            }
        }
        self.pending.push(spike, cost);
    }

    #[cfg(feature = "core1")]
    fn resolve_physical_spike(&mut self, spike: &Spike) {
        let Some((arrow, generation)) = spike.arrow else {
            return;
        };
        let Some(slot) = self.arrow_slot(arrow) else {
            return;
        };
        if self.arrows.get(slot.0).generation != generation {
            return;
        }
        let count = &mut self.in_flight_arrows[arrow.0 as usize];
        *count = count
            .checked_sub(1)
            .expect("resolved physical arrival must have been emitted");
    }

    pub fn enter(&mut self, input: SpikeInput) {
        self.require_cell(input.target);
        assert!(
            input.arrival_tick >= self.tick,
            "physical arrivals cannot precede current substrate time"
        );
        let mut ignored = ExecutionCost::default();
        self.enqueue_physical_spike(
            Spike {
                arrival_tick: input.arrival_tick,
                phase: input.phase,
                #[cfg(feature = "si0")]
                causal_wave: 0,
                origin_physical: input.origin_physical,
                #[cfg(feature = "cl0")]
                target_physical: self
                    .cells
                    .get(self.cell_slot(input.target).unwrap().0)
                    .physical_id,
                target: input.target,
                target_generation: self
                    .cells
                    .get(self.cell_slot(input.target).unwrap().0)
                    .generation,
                impulse: input.impulse,
                #[cfg(feature = "core0")]
                material_impulse: i64::from(input.impulse).saturating_mul(MATERIAL_ONE),
                serial: self.next_serial,
                arrow: None,
            },
            &mut ignored,
        );
        self.next_serial = self.next_serial.wrapping_add(1);
    }

    pub fn arrive(&mut self, inputs: &[SpikeInput], outward_region: i16) -> RunResult {
        for input in inputs {
            self.enter(*input);
        }
        let mut result = self.propagate();
        result
            .crossings
            .retain(|crossing| crossing.to_region == outward_region);
        result
    }

    pub fn advance_time(&mut self, tick: i64) -> Work {
        assert!(tick >= self.tick, "physical time cannot run backward");
        assert!(
            self.pending.is_empty(),
            "queued activity must propagate first"
        );
        let mut work = Work::default();
        let mut ignored = ExecutionCost::default();
        self.elapse_to(tick, &mut work, &mut ignored);
        self.tick = tick;
        work
    }

    #[cfg(feature = "cl0")]
    pub fn advance_time_traced(&mut self, tick: i64) -> RunResult {
        assert!(tick >= self.tick, "physical time cannot run backward");
        assert!(
            self.pending.is_empty(),
            "queued activity must propagate first"
        );
        let mut work = Work::default();
        let mut execution_cost = ExecutionCost::default();
        let mut physical_trace = Vec::new();
        self.elapse_to_observed(tick, &mut work, &mut execution_cost, 0, &mut physical_trace);
        self.tick = tick;
        execution_cost.observe_resident_bytes(self.mechanical_resident_bytes());
        RunResult {
            crossings: Vec::new(),
            work,
            naturally_quiescent: self.pending.is_empty(),
            resident_bytes: self.resident_bytes(),
            execution_cost,
            physical_trace,
        }
    }

    pub fn arena_body(&self, version: u64) -> ArenaBody {
        let minimum_position = self
            .cells
            .values()
            .iter()
            .map(|cell| cell.position)
            .min()
            .unwrap_or(0);
        let maximum_position = self
            .cells
            .values()
            .iter()
            .map(|cell| cell.position)
            .max()
            .unwrap_or(0);
        ArenaBody {
            arena: self.arena,
            version,
            minimum_position,
            maximum_position,
            cell_capacity: self.cell_capacity,
            arrow_capacity: self.arrow_capacity,
            cells: self
                .cells
                .values()
                .iter()
                .map(|cell| DurableCell {
                    id: cell.id,
                    generation: cell.generation,
                    physical_id: cell.physical_id,
                    position: cell.position,
                    region: cell.region,
                    threshold: cell.threshold,
                    resistance: cell.resistance,
                    live: cell.live,
                })
                .collect(),
            arrows: self
                .arrows
                .values()
                .iter()
                .map(|arrow| DurableArrow {
                    id: arrow.id,
                    generation: arrow.generation,
                    from: CellRef {
                        arena: self.arena,
                        id: arrow.from,
                        generation: arrow.source_generation,
                    },
                    to: CellRef {
                        arena: self.arena,
                        id: arrow.to,
                        #[cfg(not(feature = "cl0"))]
                        generation: self.cell_reference(arrow.to).generation,
                        #[cfg(feature = "cl0")]
                        generation: arrow.target_generation,
                    },
                    delay: arrow.delay,
                    phase: arrow.phase,
                    coupling: arrow.coupling,
                    resistance: arrow.resistance,
                    transmission_mode: transmission_mode_byte(arrow.mode),
                    live: arrow.live,
                })
                .collect(),
        }
    }

    pub fn canonical_body_bytes(&self, version: u64) -> Result<Vec<u8>, FormatError> {
        self.arena_body(version).canonical_bytes()
    }

    pub fn from_body_bytes(bytes: &[u8]) -> Result<Self, CheckpointError> {
        Self::from_arena_body(ArenaBody::decode(bytes)?)
    }

    pub fn from_arena_body(body: ArenaBody) -> Result<Self, CheckpointError> {
        Self::from_arena_body_with_packing(body, false)
    }

    pub fn from_arena_body_with_packing(
        mut body: ArenaBody,
        reverse_slots: bool,
    ) -> Result<Self, CheckpointError> {
        body.validate()?;
        // AH0_STORAGE_ONLY: deliberately selectable resident packing for compaction tests.
        if reverse_slots {
            body.cells.sort_by_key(|cell| std::cmp::Reverse(cell.id));
            body.arrows.sort_by_key(|arrow| std::cmp::Reverse(arrow.id));
        } else {
            body.cells.sort_by_key(|cell| cell.id);
            body.arrows.sort_by_key(|arrow| arrow.id);
        }
        let mut substrate =
            Self::with_capacity(body.arena, body.cell_capacity, body.arrow_capacity);
        let maximum_cell_id = body.cells.iter().map(|cell| cell.id.0).max();
        substrate.cell_slots = maximum_cell_id
            .map(|maximum| vec![None; maximum as usize + 1])
            .unwrap_or_default();
        for durable in body.cells {
            #[cfg(feature = "j0")]
            let invalid_liveness = false;
            #[cfg(not(feature = "j0"))]
            let invalid_liveness = durable.live != (durable.resistance > 0);
            if durable.threshold <= 0 || invalid_liveness {
                return Err(CheckpointError::InvalidPhysicalBody);
            }
            let slot = CellSlot(substrate.cells.len());
            substrate.cell_slots[durable.id.0 as usize] = durable.live.then_some(slot);
            substrate.cells.push(Cell {
                id: durable.id,
                physical_id: durable.physical_id,
                position: durable.position,
                region: durable.region,
                threshold: durable.threshold,
                state: 0,
                last_update_tick: 0,
                refractory_until: 0,
                generation: durable.generation,
                resistance: durable.resistance,
                live: durable.live,
                #[cfg(feature = "cl0")]
                decay_load: 0,
                #[cfg(feature = "cc0")]
                participation_level: 0,
            });
        }
        substrate.outgoing_index = vec![Vec::new(); substrate.cell_slots.len()];
        substrate.resident_arenas = vec![ResidentArenaId(0); substrate.cell_slots.len()];
        let maximum_arrow_id = body.arrows.iter().map(|arrow| arrow.id.0).max();
        substrate.arrow_slots = maximum_arrow_id
            .map(|maximum| vec![None; maximum as usize + 1])
            .unwrap_or_default();
        for durable in body.arrows {
            if durable.from.arena != substrate.arena || durable.to.arena != substrate.arena {
                return Err(CheckpointError::MissingCell(durable.from.id));
            }
            let from_slot = substrate
                .cells
                .values()
                .iter()
                .position(|cell| cell.id == durable.from.id)
                .map(CellSlot)
                .ok_or(CheckpointError::MissingCell(durable.from.id))?;
            let to_slot = substrate
                .cells
                .values()
                .iter()
                .position(|cell| cell.id == durable.to.id)
                .map(CellSlot)
                .ok_or(CheckpointError::MissingCell(durable.to.id))?;
            if durable.live {
                if substrate.cells.get(from_slot.0).generation != durable.from.generation {
                    return Err(CheckpointError::StaleCellReference(durable.from));
                }
                if substrate.cells.get(to_slot.0).generation != durable.to.generation {
                    return Err(CheckpointError::StaleCellReference(durable.to));
                }
                if !substrate.cells.get(from_slot.0).live || !substrate.cells.get(to_slot.0).live {
                    return Err(CheckpointError::InvalidPhysicalBody);
                }
            }
            if durable.delay < 0 || durable.live != (durable.resistance > 0) {
                return Err(CheckpointError::InvalidPhysicalBody);
            }
            let mode = transmission_mode_from_byte(durable.transmission_mode)?;
            let slot = ArrowSlot(substrate.arrows.len());
            substrate.arrow_slots[durable.id.0 as usize] = Some(slot);
            substrate.arrows.push(Arrow {
                id: durable.id,
                from: durable.from.id,
                to: durable.to.id,
                delay: durable.delay,
                phase: durable.phase,
                coupling: durable.coupling,
                source_generation: durable.from.generation,
                #[cfg(feature = "cl0")]
                target_generation: durable.to.generation,
                generation: durable.generation,
                resistance: durable.resistance,
                live: durable.live,
                participation_level: 0,
                plastic_support: 0,
                decay_load: 0,
                mode,
                trigger: TransmissionTrigger::SourceFires,
            });
            substrate.outgoing_index[durable.from.id.0 as usize].push(durable.id);
        }
        substrate.rebuild_mechanical_indexes();
        #[cfg(feature = "core1")]
        substrate
            .in_flight_arrows
            .resize(substrate.arrow_slots.len(), 0);
        #[cfg(feature = "core1")]
        substrate
            .used_pending_arrows
            .resize(substrate.arrow_slots.len(), false);
        #[cfg(feature = "core1")]
        substrate
            .temporary_credit_return_arrows
            .resize(substrate.arrow_slots.len(), false);
        Ok(substrate)
    }

    pub fn quiescent_checkpoint(
        &self,
        body_version: u64,
    ) -> Result<QuiescentCheckpoint, CheckpointError> {
        let transiently_quiet = self.pending.is_empty()
            && self.pending_loads.is_empty()
            && self.cells.values().iter().all(|cell| {
                cell.state == 0 && cell.refractory_until <= self.tick && {
                    #[cfg(feature = "cl0")]
                    {
                        cell.decay_load == 0 && {
                            #[cfg(feature = "cc0")]
                            {
                                cell.participation_level == 0
                            }
                            #[cfg(not(feature = "cc0"))]
                            {
                                true
                            }
                        }
                    }
                    #[cfg(not(feature = "cl0"))]
                    {
                        true
                    }
                }
            })
            && self
                .arrows
                .values()
                .iter()
                .all(|arrow| arrow.participation_level == 0 && arrow.decay_load == 0)
            && {
                #[cfg(feature = "core1")]
                {
                    self.used_pending_count() == 0 && self.temporary_credit_return_count() == 0
                }
                #[cfg(not(feature = "core1"))]
                {
                    true
                }
            };
        if !transiently_quiet {
            return Err(CheckpointError::NotQuiescent);
        }
        Ok(QuiescentCheckpoint {
            body_version: self.body_version(body_version)?,
            body: self.arena_body(body_version),
            clock: self.clock(),
        })
    }

    pub fn from_quiescent_checkpoint(
        checkpoint: QuiescentCheckpoint,
    ) -> Result<Self, CheckpointError> {
        Self::from_quiescent_checkpoint_with_mechanics(checkpoint, MechanicalConfig::REFERENCE)
    }

    pub fn from_quiescent_checkpoint_with_mechanics(
        checkpoint: QuiescentCheckpoint,
        mechanics: MechanicalConfig,
    ) -> Result<Self, CheckpointError> {
        validate_manifest(&checkpoint.body_version, &checkpoint.body)?;
        let mut substrate = Self::from_arena_body(checkpoint.body)?;
        substrate.reconfigure_mechanics(mechanics);
        substrate.tick = checkpoint.clock.tick;
        substrate.pressure_tick = pressure_epoch(checkpoint.clock.tick);
        for index in 0..substrate.cells.len() {
            substrate.cells.with_mut(index, |cell| {
                cell.last_update_tick = checkpoint.clock.tick;
                cell.refractory_until = checkpoint.clock.tick;
            });
        }
        Ok(substrate)
    }

    pub fn live_checkpoint(&self, body_version: u64) -> Result<LiveCheckpoint, CheckpointError> {
        let pending = self.pending.canonical();
        Ok(LiveCheckpoint {
            body_version: self.body_version(body_version)?,
            body: self.arena_body(body_version),
            clock: self.clock(),
            cells: self
                .cells
                .values()
                .iter()
                .map(|cell| CellRuntime {
                    id: cell.id,
                    state: cell.state,
                    last_update_tick: cell.last_update_tick,
                    refractory_until: cell.refractory_until,
                    #[cfg(feature = "cl0")]
                    decay_load: cell.decay_load,
                    #[cfg(feature = "cc0")]
                    participation_level: cell.participation_level,
                })
                .collect(),
            arrows: self
                .arrows
                .values()
                .iter()
                .map(|arrow| ArrowRuntime {
                    id: arrow.id,
                    participation_level: arrow.participation_level,
                    plastic_support: arrow.plastic_support,
                    decay_load: arrow.decay_load,
                    trigger: arrow.trigger,
                })
                .collect(),
            pending,
            next_serial: self.next_serial,
            pending_loads: self.pending_loads.clone(),
            #[cfg(feature = "core1")]
            core0_profile: self.core0_profile,
            #[cfg(feature = "core1")]
            in_flight_protection: self.in_flight_protection,
            #[cfg(feature = "core1")]
            protect_all_live_arrows: self.protect_all_live_arrows,
        })
    }

    pub fn from_live_checkpoint(checkpoint: LiveCheckpoint) -> Result<Self, CheckpointError> {
        Self::from_live_checkpoint_with_mechanics(checkpoint, MechanicalConfig::REFERENCE)
    }

    pub fn from_live_checkpoint_with_mechanics(
        checkpoint: LiveCheckpoint,
        mechanics: MechanicalConfig,
    ) -> Result<Self, CheckpointError> {
        validate_manifest(&checkpoint.body_version, &checkpoint.body)?;
        let mut substrate = Self::from_arena_body(checkpoint.body)?;
        substrate.tick = checkpoint.clock.tick;
        substrate.pressure_tick = pressure_epoch(checkpoint.clock.tick);
        for runtime in checkpoint.cells {
            let slot = substrate
                .cells
                .values()
                .iter()
                .position(|cell| cell.id == runtime.id)
                .map(CellSlot)
                .ok_or(CheckpointError::MissingCell(runtime.id))?;
            substrate.cells.with_mut(slot.0, |cell| {
                cell.state = runtime.state;
                cell.last_update_tick = runtime.last_update_tick;
                cell.refractory_until = runtime.refractory_until;
                #[cfg(feature = "cl0")]
                {
                    cell.decay_load = runtime.decay_load;
                }
                #[cfg(feature = "cc0")]
                {
                    cell.participation_level = runtime.participation_level;
                }
            });
        }
        for runtime in checkpoint.arrows {
            let slot = substrate
                .arrow_slot(runtime.id)
                .ok_or(CheckpointError::MissingArrow(runtime.id))?;
            substrate.arrows.with_mut(slot.0, |arrow| {
                arrow.participation_level = runtime.participation_level;
                arrow.plastic_support = runtime.plastic_support;
                arrow.decay_load = runtime.decay_load;
                arrow.trigger = runtime.trigger;
            });
        }
        #[cfg(feature = "core1")]
        substrate.set_core0_profile(checkpoint.core0_profile);
        substrate.pending = PendingSchedule::from_canonical(
            SchedulerKind::Vec,
            checkpoint.clock.tick,
            checkpoint.pending,
        );
        substrate.next_serial = checkpoint.next_serial;
        substrate.pending_loads = checkpoint.pending_loads;
        #[cfg(feature = "core1")]
        {
            substrate.in_flight_protection = checkpoint.in_flight_protection;
            substrate.protect_all_live_arrows = checkpoint.protect_all_live_arrows;
            substrate.in_flight_arrows.fill(0);
            for spike in substrate.pending.canonical() {
                if let Some((arrow, generation)) = spike.arrow {
                    if substrate
                        .arrow_slot(arrow)
                        .is_some_and(|slot| substrate.arrows.get(slot.0).generation == generation)
                    {
                        let count = &mut substrate.in_flight_arrows[arrow.0 as usize];
                        *count = count.saturating_add(1);
                    }
                }
            }
        }
        substrate.active_cells = substrate
            .cells
            .values()
            .iter()
            .filter(|cell| cell.state != 0)
            .map(|cell| cell.id)
            .collect();
        substrate.reconfigure_mechanics(mechanics);
        Ok(substrate)
    }

    fn body_version(&self, version: u64) -> Result<BodyVersion, CheckpointError> {
        let body = self.arena_body(version);
        Ok(BodyVersion {
            version,
            parent: None,
            arenas: vec![ArenaVersion {
                arena: self.arena,
                block: body.content_hash()?,
            }],
        })
    }

    pub fn register_pending_load(&mut self, load: PendingLoad) {
        assert!(
            load.issue_tick >= self.tick,
            "load issue cannot precede physical time"
        );
        if let Some(availability_tick) = load.availability_tick {
            assert!(
                availability_tick >= load.issue_tick,
                "availability cannot precede load issue"
            );
        }
        self.pending_loads.push(load);
        self.pending_loads.sort_by_key(|load| {
            (
                load.availability_tick.unwrap_or(i64::MAX),
                load.issue_tick,
                load.arena,
            )
        });
    }

    pub fn admit_load_availability(&mut self, arena: ArenaId, availability_tick: i64) {
        let load = self
            .pending_loads
            .iter_mut()
            .find(|load| load.arena == arena && load.availability_tick.is_none())
            .expect("pending load must exist before availability is admitted");
        assert!(
            availability_tick >= load.issue_tick,
            "availability cannot precede load issue"
        );
        load.availability_tick = Some(availability_tick);
    }

    pub fn compact_resident(&mut self) {
        let mut cells = self.cells.values();
        // AH0_STORAGE_ONLY: disposable resident slots; handles and physics stay unchanged.
        cells.sort_by_key(|cell| std::cmp::Reverse(cell.id));
        self.cells.replace_values(cells);
        let mut arrows = self.arrows.values();
        // AH0_STORAGE_ONLY: disposable resident slots; handles and physics stay unchanged.
        arrows.sort_by_key(|arrow| (!arrow.live, std::cmp::Reverse(arrow.id)));
        self.arrows.replace_values(arrows);
        self.rebuild_slot_maps();
    }

    pub fn propagate(&mut self) -> RunResult {
        self.propagate_with_optional_ceiling(None).0
    }

    #[cfg(feature = "rs0")]
    pub fn propagate_with_observation_ceiling(&mut self, ceiling: u64) -> ObservedRun {
        assert!(ceiling > 0, "observation ceiling must be positive");
        let (run, scheduled_deliveries) = self.propagate_with_optional_ceiling(Some(ceiling));
        ObservedRun {
            observation_ceiling_reached: scheduled_deliveries == ceiling
                && !run.naturally_quiescent,
            run,
            scheduled_deliveries,
        }
    }

    fn propagate_with_optional_ceiling(&mut self, ceiling: Option<u64>) -> (RunResult, u64) {
        #[cfg(feature = "si0")]
        {
            self.propagate_si0(ceiling)
        }
        #[cfg(not(feature = "si0"))]
        {
            let mut crossings = Vec::new();
            let mut work = Work::default();
            let mut execution_cost = ExecutionCost::default();
            let mut physical_trace = Vec::new();
            let mut scheduled_deliveries = 0_u64;
            execution_cost.observe_resident_bytes(self.mechanical_resident_bytes());
            while !self.pending.is_empty() {
                if ceiling.is_some_and(|limit| scheduled_deliveries >= limit) {
                    break;
                }
                let maximum_batch = ceiling.map_or(64, |limit| {
                    usize::try_from(limit.saturating_sub(scheduled_deliveries).min(64)).unwrap_or(1)
                });
                let batch = if self.mechanics.executor == ExecutorKind::Batched {
                    if self.zero_delay_live_arrows == 0 {
                        let batch = self.pop_scheduled_batch(maximum_batch, &mut execution_cost);
                        execution_cost.observe_batch(batch.len());
                        batch
                    } else {
                        execution_cost.batch_fallback_zero_delay =
                            execution_cost.batch_fallback_zero_delay.saturating_add(1);
                        execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                        vec![self
                            .pop_scheduled(&mut execution_cost)
                            .expect("nonempty schedule must pop")]
                    }
                } else {
                    execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                    vec![self
                        .pop_scheduled(&mut execution_cost)
                        .expect("nonempty schedule must pop")]
                };
                for (spike, legacy_comparisons) in batch {
                    scheduled_deliveries = scheduled_deliveries.saturating_add(1);
                    execution_cost.touch::<Spike>(1);
                    work.total = work.total.saturating_add(legacy_comparisons);
                    let external_arrival = spike.arrow.is_none();
                    self.elapse_to_observed(
                        spike.arrival_tick,
                        &mut work,
                        &mut execution_cost,
                        spike.phase,
                        &mut physical_trace,
                    );
                    self.tick = spike.arrival_tick;
                    #[cfg(feature = "core1")]
                    self.resolve_physical_spike(&spike);
                    work.total = work.total.saturating_add(2);

                    if let Some((arrow_id, generation)) = spike.arrow {
                        let Some(arrow_slot) = self.arrow_slot(arrow_id) else {
                            continue;
                        };
                        let arrow = self.arrows.get(arrow_slot.0);
                        execution_cost.touch::<Arrow>(1);
                        if !arrow.live || arrow.generation != generation {
                            continue;
                        }
                    }
                    let Some(target_slot) = self.cell_slot(spike.target) else {
                        continue;
                    };
                    let target = self.cells.get(target_slot.0);
                    execution_cost.touch::<Cell>(1);
                    if target.id != spike.target
                        || !target.live
                        || target.generation != spike.target_generation
                    {
                        continue;
                    }

                    let mode = spike.arrow.map_or(TransmissionMode::Drive, |(arrow, _)| {
                        execution_cost.touch::<Arrow>(1);
                        self.arrows.get(self.arrow_slot(arrow).unwrap().0).mode
                    });
                    if self.trace_physics {
                        physical_trace.push(PhysicalTransition {
                            tick: self.tick,
                            phase: spike.phase,
                            event: PhysicalEvent::Deliver {
                                mode,
                                target: spike.target,
                                impulse: spike.impulse,
                            },
                        });
                    }
                    if mode == TransmissionMode::Modulatory {
                        work.total = work.total.saturating_add(1);
                        work.modulatory_deliveries = work.modulatory_deliveries.saturating_add(1);
                        self.apply_modulatory_return(
                            spike.target,
                            self.tick,
                            &mut work,
                            &mut execution_cost,
                            spike.phase,
                            &mut physical_trace,
                            0,
                        );
                        continue;
                    }
                    work.total = work.total.saturating_add(3);
                    work.drive_deliveries = work.drive_deliveries.saturating_add(1);
                    self.decay_cell(spike.target, self.tick);
                    let target_slot = self.cell_slot(spike.target).unwrap();
                    let target = self.cells.with_mut(target_slot.0, |target| {
                        target.state = target.state.saturating_add(spike.impulse);
                        target.clone()
                    });
                    execution_cost.touch::<Cell>(1);
                    if target.state != 0 {
                        self.active_cells.insert(spike.target);
                    }
                    let fires =
                        self.tick >= target.refractory_until && target.state >= target.threshold;
                    if !fires {
                        continue;
                    }

                    self.cells.with_mut(target_slot.0, |target| {
                        target.state = 0;
                        target.refractory_until = self.tick.saturating_add(1);
                        #[cfg(feature = "cc0")]
                        {
                            target.participation_level = target
                                .participation_level
                                .saturating_add(PARTICIPATION_IMPULSE);
                        }
                    });
                    self.active_cells.remove(&spike.target);
                    if self.trace_physics {
                        physical_trace.push(PhysicalTransition {
                            tick: self.tick,
                            phase: spike.phase,
                            event: PhysicalEvent::Fire { cell: spike.target },
                        });
                    }
                    #[cfg(feature = "core1")]
                    if let Some((participating, _)) = spike.arrow {
                        self.maybe_create_atomic_credit_return(participating);
                    }
                    work.total = work.total.saturating_add(1);
                    let source = spike.target;
                    let origin_physical = target.physical_id;
                    let source_generation = target.generation;
                    if external_arrival {
                        self.propose_local_arrows(
                            source,
                            &mut work,
                            &mut execution_cost,
                            spike.phase,
                            &mut physical_trace,
                        );
                    }
                    let mut outgoing = match self.mechanics.traversal {
                        TraversalKind::GlobalScan => {
                            execution_cost.allocations =
                                execution_cost.allocations.saturating_add(1);
                            execution_cost.scans = execution_cost
                                .scans
                                .saturating_add(self.arrows.len() as u64);
                            execution_cost.touch::<Arrow>(self.arrows.len());
                            self.arrows
                                .values()
                                .iter()
                                .map(|arrow| (arrow.id, arrow.clone()))
                                .collect::<Vec<_>>()
                        }
                        TraversalKind::Adjacency => {
                            execution_cost.allocations =
                                execution_cost.allocations.saturating_add(1);
                            execution_cost.adjacency_accesses =
                                execution_cost.adjacency_accesses.saturating_add(
                                    u64::try_from(self.outgoing_index[source.0 as usize].len())
                                        .unwrap_or(u64::MAX),
                                );
                            self.outgoing_index[source.0 as usize]
                                .iter()
                                .filter_map(|id| {
                                    let slot = self.arrow_slot(*id)?;
                                    execution_cost.scans = execution_cost.scans.saturating_add(1);
                                    execution_cost.touch::<Arrow>(1);
                                    Some((*id, self.arrows.get(slot.0)))
                                })
                                .collect()
                        }
                    };
                    outgoing.sort_by_key(|(_, arrow)| self.physical_arrow_order(arrow));
                    for (arrow_id, arrow) in outgoing {
                        execution_cost.touch::<Arrow>(1);
                        work.total = work.total.saturating_add(1);
                        if !arrow.live
                            || arrow.from != source
                            || arrow.source_generation != source_generation
                        {
                            continue;
                        }
                        if arrow.trigger != TransmissionTrigger::SourceFires {
                            continue;
                        }
                        let Some(from_slot) = self.cell_slot(arrow.from) else {
                            continue;
                        };
                        let Some(to_slot) = self.cell_slot(arrow.to) else {
                            continue;
                        };
                        let from = self.cells.get(from_slot.0);
                        let to = self.cells.get(to_slot.0);
                        execution_cost.touch::<Cell>(2);
                        if from.id != arrow.from
                            || !from.live
                            || from.generation != arrow.source_generation
                            || to.id != arrow.to
                            || !to.live
                            || {
                                #[cfg(feature = "cl0")]
                                {
                                    to.generation != arrow.target_generation
                                }
                                #[cfg(not(feature = "cl0"))]
                                {
                                    false
                                }
                            }
                        {
                            continue;
                        }
                        if from.region != to.region {
                            let crossing = Crossing {
                                tick: self.tick,
                                from_physical: from.physical_id,
                                to_physical: to.physical_id,
                                from_region: from.region,
                                to_region: to.region,
                                impulse: arrow.coupling,
                            };
                            if self.trace_physics {
                                physical_trace.push(PhysicalTransition {
                                    tick: self.tick,
                                    phase: spike.phase,
                                    event: PhysicalEvent::Crossing(crossing),
                                });
                            }
                            crossings.push(crossing);
                        }
                        let arrow_slot = self.arrow_slot(arrow_id).unwrap();
                        self.arrows.with_mut(arrow_slot.0, |live_arrow| {
                            live_arrow.participation_level = live_arrow
                                .participation_level
                                .saturating_add(PARTICIPATION_IMPULSE);
                        });
                        #[cfg(feature = "core1")]
                        if self.capture_used_pending {
                            self.used_pending_arrows[arrow_id.0 as usize] = true;
                        }
                        execution_cost.touch::<Arrow>(1);
                        work.total = work.total.saturating_add(1);
                        execution_cost.arena_lookups =
                            execution_cost.arena_lookups.saturating_add(2);
                        if self.resident_arenas[arrow.from.0 as usize]
                            != self.resident_arenas[arrow.to.0 as usize]
                        {
                            execution_cost.arena_hops = execution_cost.arena_hops.saturating_add(1);
                        }
                        self.enqueue_physical_spike(
                            Spike {
                                arrival_tick: self.tick.saturating_add(arrow.delay),
                                phase: arrow.phase,
                                #[cfg(feature = "si0")]
                                causal_wave: if arrow.delay == 0 && arrow.phase == spike.phase {
                                    spike.causal_wave.saturating_add(1)
                                } else {
                                    0
                                },
                                origin_physical,
                                #[cfg(feature = "cl0")]
                                target_physical: to.physical_id,
                                target: arrow.to,
                                target_generation: to.generation,
                                impulse: arrow.coupling,
                                #[cfg(feature = "core0")]
                                material_impulse: self.core0_coupling[arrow_id.0 as usize],
                                serial: self.next_serial,
                                arrow: Some((arrow_id, arrow.generation)),
                            },
                            &mut execution_cost,
                        );
                        self.next_serial = self.next_serial.wrapping_add(1);
                    }
                }
                execution_cost.observe_resident_bytes(self.mechanical_resident_bytes());
            }
            (
                RunResult {
                    crossings,
                    work,
                    naturally_quiescent: self.pending.is_empty(),
                    resident_bytes: self.resident_bytes(),
                    execution_cost,
                    physical_trace,
                },
                scheduled_deliveries,
            )
        }
    }

    #[cfg(feature = "si0")]
    fn propagate_si0(&mut self, ceiling: Option<u64>) -> (RunResult, u64) {
        let mut crossings = Vec::new();
        let mut work = Work::default();
        let mut execution_cost = ExecutionCost::default();
        let mut physical_trace = Vec::new();
        let mut scheduled_deliveries = 0_u64;
        execution_cost.observe_resident_bytes(self.mechanical_resident_bytes());
        while !self.pending.is_empty() {
            if ceiling.is_some_and(|limit| scheduled_deliveries >= limit) {
                break;
            }
            let batch = self.pending.drain_minimum_wave(&mut execution_cost);
            assert!(!batch.is_empty());
            execution_cost.observe_batch(batch.len());
            let arrival_tick = batch[0].0.arrival_tick;
            let phase = batch[0].0.phase;
            let causal_wave = batch[0].0.causal_wave;
            self.elapse_to_observed(
                arrival_tick,
                &mut work,
                &mut execution_cost,
                phase,
                &mut physical_trace,
            );
            self.tick = arrival_tick;
            scheduled_deliveries =
                scheduled_deliveries.saturating_add(u64::try_from(batch.len()).unwrap_or(u64::MAX));

            let mut incidences: Vec<(CellId, Vec<Spike>, Vec<Spike>)> = Vec::new();
            for (spike, _mechanical_comparisons) in batch {
                execution_cost.touch::<Spike>(1);
                #[cfg(feature = "core1")]
                self.resolve_physical_spike(&spike);
                let mode = if let Some((arrow_id, generation)) = spike.arrow {
                    let Some(arrow_slot) = self.arrow_slot(arrow_id) else {
                        continue;
                    };
                    let arrow = self.arrows.get(arrow_slot.0);
                    execution_cost.touch::<Arrow>(1);
                    if !arrow.live || arrow.generation != generation {
                        continue;
                    }
                    arrow.mode
                } else {
                    TransmissionMode::Drive
                };
                let Some(target_slot) = self.cell_slot(spike.target) else {
                    continue;
                };
                let target = self.cells.get(target_slot.0);
                execution_cost.touch::<Cell>(1);
                if target.id != spike.target
                    || !target.live
                    || target.generation != spike.target_generation
                {
                    continue;
                }
                if let Some((_, drive_arrivals, modulatory_arrivals)) = incidences
                    .iter_mut()
                    .find(|(target, _, _)| *target == spike.target)
                {
                    match mode {
                        TransmissionMode::Drive => drive_arrivals.push(spike),
                        TransmissionMode::Modulatory => modulatory_arrivals.push(spike),
                    }
                } else {
                    let target_id = spike.target;
                    let (drive_arrivals, modulatory_arrivals) = match mode {
                        TransmissionMode::Drive => (vec![spike], Vec::new()),
                        TransmissionMode::Modulatory => (Vec::new(), vec![spike]),
                    };
                    incidences.push((target_id, drive_arrivals, modulatory_arrivals));
                }
            }

            // WS0_SYNCHRONOUS_INCIDENCE: Modulatory and Drive incidence update
            // disjoint local state from the same drained wave. Neither packet
            // order nor junction iteration order is a causal fact. All caused
            // transmissions are queued only after this incidence stage.
            for (target_id, _, spikes) in &incidences {
                if spikes.is_empty() {
                    continue;
                }
                let arrivals = u32::try_from(spikes.len()).unwrap_or(u32::MAX);
                let impulse = spikes
                    .iter()
                    .fold(0_i32, |sum, spike| sum.saturating_add(spike.impulse));
                work.total = work.total.saturating_add(
                    u64::try_from(spikes.len())
                        .unwrap_or(u64::MAX)
                        .saturating_mul(2),
                );
                work.modulatory_deliveries = work
                    .modulatory_deliveries
                    .saturating_add(u64::try_from(spikes.len()).unwrap_or(u64::MAX));
                if self.trace_physics {
                    physical_trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase,
                        event: PhysicalEvent::ModulatoryIncidence {
                            target: *target_id,
                            arrivals,
                            impulse,
                            causal_wave,
                        },
                    });
                }
                for _ in spikes {
                    self.apply_modulatory_return(
                        *target_id,
                        self.tick,
                        &mut work,
                        &mut execution_cost,
                        phase,
                        &mut physical_trace,
                        causal_wave,
                    );
                }
            }

            let mut firings = Vec::new();
            for (target_id, spikes, _) in incidences {
                if spikes.is_empty() {
                    continue;
                }
                let arrivals = u32::try_from(spikes.len()).unwrap_or(u32::MAX);
                let impulse = spikes
                    .iter()
                    .fold(0_i32, |sum, spike| sum.saturating_add(spike.impulse));
                #[cfg(feature = "core0")]
                let material_impulse = spikes.iter().fold(0_i64, |sum, spike| {
                    sum.saturating_add(spike.material_impulse)
                });
                let external_arrival = spikes.iter().any(|spike| spike.arrow.is_none());
                work.total = work.total.saturating_add(
                    u64::try_from(spikes.len())
                        .unwrap_or(u64::MAX)
                        .saturating_mul(5),
                );
                work.drive_deliveries = work
                    .drive_deliveries
                    .saturating_add(u64::try_from(spikes.len()).unwrap_or(u64::MAX));
                if self.trace_physics {
                    physical_trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase,
                        event: PhysicalEvent::DriveIncidence {
                            target: target_id,
                            arrivals,
                            impulse,
                            causal_wave,
                        },
                    });
                }

                #[cfg(feature = "core0")]
                let continuous = self.core0_profile.continuous();
                #[cfg(not(feature = "core0"))]
                let continuous = false;
                if continuous {
                    #[cfg(feature = "core0")]
                    self.decay_core0_activation(target_id, self.tick);
                } else {
                    self.decay_cell(target_id, self.tick);
                }
                let target_slot = self.cell_slot(target_id).unwrap();
                #[cfg(feature = "core0")]
                if continuous {
                    let index = target_id.0 as usize;
                    self.core0_activation[index] =
                        self.core0_activation[index].saturating_add(material_impulse);
                    let observer = self.core0_activation[index] / MATERIAL_ONE;
                    self.cells.with_mut(target_slot.0, |target| {
                        target.state = i32::try_from(observer).unwrap_or_else(|_| {
                            if observer.is_negative() {
                                i32::MIN
                            } else {
                                i32::MAX
                            }
                        });
                    });
                }
                let target = self.cells.with_mut(target_slot.0, |target| {
                    if !continuous {
                        target.state = target.state.saturating_add(impulse);
                    }
                    target.clone()
                });
                #[cfg(feature = "core0")]
                if self.trace_physics {
                    let activation_after = if continuous {
                        self.core0_activation[target_id.0 as usize]
                    } else {
                        i64::from(target.state).saturating_mul(MATERIAL_ONE)
                    };
                    physical_trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase,
                        event: PhysicalEvent::MaterialDriveIncidence {
                            target: target_id,
                            impulse: material_impulse,
                            activation_after,
                            causal_wave,
                        },
                    });
                }
                execution_cost.touch::<Cell>(1);
                #[cfg(feature = "core0")]
                let materially_active =
                    continuous && self.core0_activation[target_id.0 as usize] != 0;
                #[cfg(not(feature = "core0"))]
                let materially_active = false;
                if target.state != 0 || materially_active {
                    self.active_cells.insert(target_id);
                }
                #[cfg(feature = "core0")]
                let reaches_threshold = if continuous {
                    self.core0_activation[target_id.0 as usize]
                        >= i64::from(target.threshold).saturating_mul(MATERIAL_ONE)
                } else {
                    target.state >= target.threshold
                };
                #[cfg(not(feature = "core0"))]
                let reaches_threshold = target.state >= target.threshold;
                let fires = self.tick >= target.refractory_until && reaches_threshold;
                if !fires {
                    continue;
                }

                self.cells.with_mut(target_slot.0, |target| {
                    target.state = 0;
                    target.refractory_until = self.tick.saturating_add(1);
                    #[cfg(feature = "cc0")]
                    {
                        target.participation_level = target
                            .participation_level
                            .saturating_add(PARTICIPATION_IMPULSE);
                    }
                });
                #[cfg(feature = "core0")]
                if continuous {
                    self.core0_activation[target_id.0 as usize] = 0;
                }
                self.active_cells.remove(&target_id);
                #[cfg(feature = "core1")]
                for participating in spikes
                    .iter()
                    .filter_map(|spike| spike.arrow.map(|pair| pair.0))
                {
                    self.maybe_create_atomic_credit_return(participating);
                }
                firings.push((target_id, target, external_arrival));
            }

            for (target_id, target, external_arrival) in firings {
                if self.trace_physics {
                    physical_trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase,
                        event: PhysicalEvent::Fire { cell: target_id },
                    });
                }
                work.total = work.total.saturating_add(1);
                let source = target_id;
                let origin_physical = target.physical_id;
                let source_generation = target.generation;
                #[cfg(feature = "core0")]
                let proposal_trigger =
                    external_arrival || self.core0_profile.proposal_on_every_firing();
                #[cfg(not(feature = "core0"))]
                let proposal_trigger = external_arrival;
                if proposal_trigger {
                    self.propose_local_arrows(
                        source,
                        &mut work,
                        &mut execution_cost,
                        phase,
                        &mut physical_trace,
                    );
                }
                let outgoing = match self.mechanics.traversal {
                    TraversalKind::GlobalScan => {
                        execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                        execution_cost.scans = execution_cost
                            .scans
                            .saturating_add(u64::try_from(self.arrows.len()).unwrap_or(u64::MAX));
                        execution_cost.touch::<Arrow>(self.arrows.len());
                        self.arrows
                            .values()
                            .iter()
                            .map(|arrow| (arrow.id, arrow.clone()))
                            .collect::<Vec<_>>()
                    }
                    TraversalKind::Adjacency => {
                        execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                        execution_cost.adjacency_accesses =
                            execution_cost.adjacency_accesses.saturating_add(
                                u64::try_from(self.outgoing_index[source.0 as usize].len())
                                    .unwrap_or(u64::MAX),
                            );
                        self.outgoing_index[source.0 as usize]
                            .iter()
                            .filter_map(|id| {
                                let slot = self.arrow_slot(*id)?;
                                execution_cost.scans = execution_cost.scans.saturating_add(1);
                                execution_cost.touch::<Arrow>(1);
                                Some((*id, self.arrows.get(slot.0)))
                            })
                            .collect()
                    }
                };
                for (arrow_id, arrow) in outgoing {
                    execution_cost.touch::<Arrow>(1);
                    if !arrow.live
                        || arrow.from != source
                        || arrow.source_generation != source_generation
                        || arrow.trigger != TransmissionTrigger::SourceFires
                    {
                        continue;
                    }
                    let Some(from_slot) = self.cell_slot(arrow.from) else {
                        continue;
                    };
                    let Some(to_slot) = self.cell_slot(arrow.to) else {
                        continue;
                    };
                    let from = self.cells.get(from_slot.0);
                    let to = self.cells.get(to_slot.0);
                    execution_cost.touch::<Cell>(2);
                    if from.id != arrow.from
                        || !from.live
                        || from.generation != arrow.source_generation
                        || to.id != arrow.to
                        || !to.live
                        || {
                            #[cfg(feature = "cl0")]
                            {
                                to.generation != arrow.target_generation
                            }
                            #[cfg(not(feature = "cl0"))]
                            {
                                false
                            }
                        }
                    {
                        continue;
                    }
                    work.total = work.total.saturating_add(2);
                    if from.region != to.region {
                        let crossing = Crossing {
                            tick: self.tick,
                            from_physical: from.physical_id,
                            to_physical: to.physical_id,
                            from_region: from.region,
                            to_region: to.region,
                            impulse: arrow.coupling,
                        };
                        if self.trace_physics {
                            physical_trace.push(PhysicalTransition {
                                tick: self.tick,
                                phase,
                                event: PhysicalEvent::Crossing(crossing),
                            });
                        }
                        crossings.push(crossing);
                    }
                    let arrow_slot = self.arrow_slot(arrow_id).unwrap();
                    self.arrows.with_mut(arrow_slot.0, |live_arrow| {
                        live_arrow.participation_level = live_arrow
                            .participation_level
                            .saturating_add(PARTICIPATION_IMPULSE);
                    });
                    #[cfg(feature = "core1")]
                    if self.capture_used_pending {
                        self.used_pending_arrows[arrow_id.0 as usize] = true;
                    }
                    execution_cost.touch::<Arrow>(1);
                    execution_cost.arena_lookups = execution_cost.arena_lookups.saturating_add(2);
                    if self.resident_arenas[arrow.from.0 as usize]
                        != self.resident_arenas[arrow.to.0 as usize]
                    {
                        execution_cost.arena_hops = execution_cost.arena_hops.saturating_add(1);
                    }
                    let next_tick = self.tick.saturating_add(arrow.delay);
                    let next_wave = if arrow.delay == 0 && arrow.phase == phase {
                        causal_wave.saturating_add(1)
                    } else {
                        0
                    };
                    self.enqueue_physical_spike(
                        Spike {
                            arrival_tick: next_tick,
                            phase: arrow.phase,
                            causal_wave: next_wave,
                            origin_physical,
                            #[cfg(feature = "cl0")]
                            target_physical: to.physical_id,
                            target: arrow.to,
                            target_generation: to.generation,
                            impulse: arrow.coupling,
                            #[cfg(feature = "core0")]
                            material_impulse: self.core0_coupling[arrow_id.0 as usize],
                            serial: self.next_serial,
                            arrow: Some((arrow_id, arrow.generation)),
                        },
                        &mut execution_cost,
                    );
                    self.next_serial = self.next_serial.wrapping_add(1);
                }
            }
            execution_cost.observe_resident_bytes(self.mechanical_resident_bytes());
        }
        (
            RunResult {
                crossings,
                work,
                naturally_quiescent: self.pending.is_empty(),
                resident_bytes: self.resident_bytes(),
                execution_cost,
                physical_trace,
            },
            scheduled_deliveries,
        )
    }

    #[cfg(not(feature = "si0"))]
    fn pop_scheduled(&mut self, execution_cost: &mut ExecutionCost) -> Option<(Spike, u64)> {
        self.pending.pop_next(execution_cost)
    }

    #[cfg(not(feature = "si0"))]
    fn pop_scheduled_batch(
        &mut self,
        maximum: usize,
        execution_cost: &mut ExecutionCost,
    ) -> Vec<(Spike, u64)> {
        self.pending.pop_same_tick_batch(maximum, execution_cost)
    }

    fn apply_modulatory_return(
        &mut self,
        cell: CellId,
        tick: i64,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        phase: i32,
        physical_trace: &mut Vec<PhysicalTransition>,
        causal_wave: u64,
    ) {
        #[cfg(feature = "cc0")]
        {
            let slot = self
                .cell_slot(cell)
                .expect("delivered Modulation target must resolve");
            let updated = self.cells.with_mut(slot.0, |cell_state| {
                if !cell_state.live || cell_state.participation_level == 0 {
                    return None;
                }
                let gain = local_consequence_gain(cell_state.participation_level);
                let before = cell_state.resistance;
                cell_state.resistance = cell_state.resistance.saturating_add(gain);
                if cell_state.resistance != before {
                    cell_state.decay_load = 0;
                }
                Some((before, cell_state.resistance))
            });
            execution_cost.touch::<Cell>(1);
            if let Some((before, after)) = updated {
                if before != after {
                    work.total = work.total.saturating_add(3);
                    work.cell_return_updates = work.cell_return_updates.saturating_add(1);
                    if self.trace_physics {
                        physical_trace.push(PhysicalTransition {
                            tick,
                            phase,
                            event: PhysicalEvent::CellResistance {
                                cell,
                                before,
                                after,
                            },
                        });
                    }
                }
            }
        }
        #[cfg(feature = "j0")]
        let candidates = {
            execution_cost.allocations = execution_cost.allocations.saturating_add(1);
            execution_cost.scans = execution_cost
                .scans
                .saturating_add(u64::try_from(self.arrows.len()).unwrap_or(u64::MAX));
            execution_cost.touch::<Arrow>(self.arrows.len());
            self.arrows
                .values()
                .iter()
                .map(|arrow| arrow.id)
                .collect::<Vec<_>>()
        };
        #[cfg(not(feature = "j0"))]
        let candidates = match self.mechanics.traversal {
            TraversalKind::GlobalScan => {
                execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                execution_cost.scans = execution_cost
                    .scans
                    .saturating_add(self.arrows.len() as u64);
                execution_cost.touch::<Arrow>(self.arrows.len());
                self.arrows
                    .values()
                    .iter()
                    .map(|arrow| arrow.id)
                    .collect::<Vec<_>>()
            }
            TraversalKind::Adjacency => {
                execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                execution_cost.adjacency_accesses =
                    execution_cost.adjacency_accesses.saturating_add(
                        u64::try_from(self.outgoing_index[cell.0 as usize].len())
                            .unwrap_or(u64::MAX),
                    );
                self.outgoing_index[cell.0 as usize].clone()
            }
        };
        #[cfg(not(feature = "si0"))]
        let mut candidates = candidates;
        #[cfg(not(feature = "si0"))]
        self.sort_arrow_ids_by_physics(&mut candidates);
        let qualified_local = candidates.iter().any(|id| {
            let slot = self.arrow_slot(*id).expect("indexed ARROW must resolve");
            let arrow = self.arrows.get(slot.0);
            arrow.live
                && arrow.from == cell
                && arrow.mode == TransmissionMode::Drive
                && arrow.participation_level > 0
        });
        for id in candidates {
            execution_cost.scans = execution_cost.scans.saturating_add(1);
            let slot = self.arrow_slot(id).expect("indexed ARROW must resolve");
            #[cfg(feature = "core0")]
            if self.core0_profile.continuous() {
                let arrow = self.arrows.get(slot.0);
                let local_participating_structure = arrow.live
                    && arrow.mode == TransmissionMode::Drive
                    && (arrow.from == cell || arrow.to == cell);
                if local_participating_structure && arrow.participation_level > 0 {
                    let index = id.0 as usize;
                    let participation = arrow.participation_level;
                    let coupling_before = self.core0_coupling[index];
                    let resistance_before = self.core0_resistance[index];
                    let sign = coupling_before.signum();
                    self.core0_coupling[index] = coupling_before.saturating_add(
                        sign.saturating_mul(i64::try_from(participation).unwrap_or(i64::MAX)),
                    );
                    self.core0_resistance[index] = resistance_before.saturating_add(
                        participation.saturating_mul(u64::from(LOCAL_RETURN_STRENGTH)),
                    );
                    let coupling_observer = self.core0_coupling[index] / MATERIAL_ONE;
                    let resistance_observer = self.core0_resistance[index]
                        .saturating_add(MATERIAL_ONE_U64.saturating_sub(1))
                        / MATERIAL_ONE_U64;
                    self.arrows.with_mut(slot.0, |live_arrow| {
                        live_arrow.coupling =
                            i32::try_from(coupling_observer).unwrap_or_else(|_| {
                                if coupling_observer.is_negative() {
                                    i32::MIN
                                } else {
                                    i32::MAX
                                }
                            });
                        live_arrow.resistance =
                            u32::try_from(resistance_observer).unwrap_or(u32::MAX);
                        live_arrow.decay_load = 0;
                    });
                    work.total = work.total.saturating_add(4);
                    work.local_return_updates = work.local_return_updates.saturating_add(1);
                }
                execution_cost.touch::<Arrow>(1);
                continue;
            }
            let updated = self.arrows.with_mut(slot.0, |arrow| {
                #[cfg(feature = "j0")]
                let local_participating_structure = arrow.live
                    && arrow.mode == TransmissionMode::Drive
                    && (arrow.from == cell || arrow.to == cell);
                #[cfg(not(feature = "j0"))]
                let local_participating_structure = arrow.live && arrow.from == cell;
                if !local_participating_structure {
                    return None;
                }
                let participation = arrow.participation_level;
                #[cfg(feature = "ce0")]
                let support_before = arrow.plastic_support;
                arrow.plastic_support = arrow.plastic_support.saturating_add(participation);
                #[cfg(feature = "ce0")]
                let coupling_before = arrow.coupling;
                #[cfg(feature = "ce0")]
                {
                    let completed_before = support_before / PARTICIPATION_IMPULSE;
                    let completed_after = arrow.plastic_support / PARTICIPATION_IMPULSE;
                    let efficacy_gain = completed_after.saturating_sub(completed_before);
                    let efficacy_gain = i32::try_from(efficacy_gain).unwrap_or(i32::MAX);
                    #[cfg(feature = "ce1")]
                    let efficacy_gain = efficacy_gain.saturating_mul(arrow.coupling.signum());
                    arrow.coupling = arrow.coupling.saturating_add(efficacy_gain);
                }
                let gain = local_consequence_gain(participation);
                let before = arrow.resistance;
                arrow.resistance = arrow.resistance.saturating_add(gain);
                if arrow.resistance != before {
                    arrow.decay_load = 0;
                }
                #[cfg(feature = "ce0")]
                return Some((before, arrow.resistance, coupling_before, arrow.coupling));
                #[cfg(not(feature = "ce0"))]
                Some((before, arrow.resistance))
            });
            execution_cost.touch::<Arrow>(1);
            #[cfg(feature = "ce0")]
            if let Some((before, after, coupling_before, coupling_after)) = updated {
                if before != after {
                    work.total = work.total.saturating_add(3);
                    work.local_return_updates = work.local_return_updates.saturating_add(1);
                }
                if coupling_before != coupling_after {
                    work.total = work.total.saturating_add(1);
                }
                if self.trace_physics && before != after {
                    physical_trace.push(PhysicalTransition {
                        tick,
                        phase,
                        event: PhysicalEvent::Resistance {
                            arrow: id,
                            before,
                            after,
                        },
                    });
                }
                if self.trace_physics && coupling_before != coupling_after {
                    physical_trace.push(PhysicalTransition {
                        tick,
                        phase,
                        event: PhysicalEvent::Coupling {
                            arrow: id,
                            before: coupling_before,
                            after: coupling_after,
                        },
                    });
                }
            }
            #[cfg(not(feature = "ce0"))]
            if let Some((before, after)) = updated {
                if before != after {
                    work.total = work.total.saturating_add(3);
                    work.local_return_updates = work.local_return_updates.saturating_add(1);
                }
                if self.trace_physics && before != after {
                    physical_trace.push(PhysicalTransition {
                        tick,
                        phase,
                        event: PhysicalEvent::Resistance {
                            arrow: id,
                            before,
                            after,
                        },
                    });
                }
            }
        }
        #[cfg(feature = "core1")]
        let qlp_enabled = self.core0_profile.qlp_enabled();
        #[cfg(not(feature = "core1"))]
        let qlp_enabled = true;
        if qualified_local && qlp_enabled {
            self.propagate_qualified_local(
                cell,
                tick,
                phase,
                work,
                execution_cost,
                physical_trace,
                causal_wave,
            );
        }
    }

    fn propagate_qualified_local(
        &mut self,
        cell: CellId,
        tick: i64,
        phase: i32,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        physical_trace: &mut Vec<PhysicalTransition>,
        causal_wave: u64,
    ) {
        let outgoing = self.outgoing_index[cell.0 as usize].clone();
        #[cfg(not(feature = "si0"))]
        let mut outgoing = outgoing;
        #[cfg(not(feature = "si0"))]
        self.sort_arrow_ids_by_physics(&mut outgoing);
        for id in outgoing {
            let slot = self.arrow_slot(id).expect("indexed ARROW must resolve");
            let arrow = self.arrows.get(slot.0);
            execution_cost.scans = execution_cost.scans.saturating_add(1);
            execution_cost.touch::<Arrow>(1);
            if !arrow.live
                || arrow.from != cell
                || arrow.trigger != TransmissionTrigger::QualifiedLocalParticipation
            {
                continue;
            }
            assert_eq!(arrow.mode, TransmissionMode::Modulatory);
            let Some(source_slot) = self.cell_slot(arrow.from) else {
                continue;
            };
            let Some(target_slot) = self.cell_slot(arrow.to) else {
                continue;
            };
            let source = self.cells.get(source_slot.0);
            let target = self.cells.get(target_slot.0);
            if source.id != arrow.from
                || !source.live
                || source.generation != arrow.source_generation
                || target.id != arrow.to
                || !target.live
                || {
                    #[cfg(feature = "cl0")]
                    {
                        target.generation != arrow.target_generation
                    }
                    #[cfg(not(feature = "cl0"))]
                    {
                        false
                    }
                }
            {
                continue;
            }
            let arrival_tick = tick.saturating_add(arrow.delay);
            let arrival_phase = arrow.phase;
            let generation = arrow.generation;
            let coupling = arrow.coupling;
            let target_generation = target.generation;
            let target_id = arrow.to;
            let origin_physical = source.physical_id;
            self.arrows.with_mut(slot.0, |live_arrow| {
                live_arrow.participation_level = live_arrow
                    .participation_level
                    .saturating_add(PARTICIPATION_IMPULSE);
            });
            work.total = work.total.saturating_add(1);
            work.qualified_local_traversals = work.qualified_local_traversals.saturating_add(1);
            if self.trace_physics {
                physical_trace.push(PhysicalTransition {
                    tick,
                    phase,
                    event: PhysicalEvent::QualifiedLocalTraversal { arrow: id },
                });
            }
            self.enqueue_physical_spike(
                Spike {
                    arrival_tick,
                    phase: arrival_phase,
                    #[cfg(feature = "si0")]
                    causal_wave: if arrow.delay == 0 && arrival_phase == phase {
                        causal_wave.saturating_add(1)
                    } else {
                        0
                    },
                    origin_physical,
                    #[cfg(feature = "cl0")]
                    target_physical: target.physical_id,
                    target: target_id,
                    target_generation,
                    impulse: coupling,
                    #[cfg(feature = "core0")]
                    material_impulse: self.core0_coupling[id.0 as usize],
                    serial: self.next_serial,
                    arrow: Some((id, generation)),
                },
                execution_cost,
            );
            self.next_serial = self.next_serial.wrapping_add(1);
        }
    }

    fn elapse_to(&mut self, tick: i64, work: &mut Work, execution_cost: &mut ExecutionCost) {
        self.elapse_fd0_decay(tick, work, execution_cost, None, 0);
        self.elapse_cells_to(tick, work, execution_cost, None, 0);
        self.elapse_activation_to(tick, execution_cost);
    }

    fn elapse_to_observed(
        &mut self,
        tick: i64,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        phase: i32,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) {
        self.elapse_fd0_decay(
            tick,
            work,
            execution_cost,
            Some(&mut *physical_trace),
            phase,
        );
        self.elapse_cells_to(
            tick,
            work,
            execution_cost,
            Some(&mut *physical_trace),
            phase,
        );
        self.elapse_activation_to(tick, execution_cost);
    }

    fn elapse_activation_to(&mut self, tick: i64, execution_cost: &mut ExecutionCost) {
        execution_cost.observe_frontier(self.active_cells.len());
        let cell_ids = match self.mechanics.activity {
            ActivityKind::FullScan => {
                execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                execution_cost.touch::<Cell>(self.cells.len());
                self.cells
                    .values()
                    .iter()
                    .filter(|cell| cell.live)
                    .map(|cell| cell.id)
                    .collect::<Vec<_>>()
            }
            ActivityKind::Frontier => {
                execution_cost.allocations = execution_cost.allocations.saturating_add(1);
                self.active_cells.iter().copied().collect()
            }
        };
        for id in cell_ids {
            if self.cell_slot(id).is_none() {
                continue;
            }
            execution_cost.scans = execution_cost.scans.saturating_add(1);
            execution_cost.touch::<Cell>(1);
            #[cfg(feature = "core0")]
            if self.core0_profile.continuous() {
                self.decay_core0_activation(id, tick);
                continue;
            }
            self.decay_cell(id, tick);
        }
    }

    fn elapse_fd0_decay(
        &mut self,
        tick: i64,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        physical_trace: Option<&mut Vec<PhysicalTransition>>,
        phase: i32,
    ) {
        #[cfg(feature = "cl0")]
        let mut physical_trace = physical_trace;
        #[cfg(not(feature = "cl0"))]
        let _ = (physical_trace, phase);
        let elapsed = tick.saturating_sub(self.tick);
        let elapsed_u64 = u64::try_from(elapsed).unwrap_or(u64::MAX);
        for index in 0..self.arrows.len() {
            execution_cost.scans = execution_cost.scans.saturating_add(1);
            #[cfg(feature = "core0")]
            if self.core0_profile.continuous() {
                let snapshot = self.arrows.get(index);
                if !snapshot.live {
                    continue;
                }
                #[cfg(feature = "core1")]
                if self.protect_all_live_arrows
                    || (self.in_flight_protection
                        && self.in_flight_arrows[snapshot.id.0 as usize] > 0)
                    || (self.used_pending_protection_enabled
                        && self.used_pending_arrows[snapshot.id.0 as usize])
                    || self.temporary_credit_return_arrows[snapshot.id.0 as usize]
                    || self.protected_by_temporary_credit_return(&snapshot)
                {
                    self.arrows.with_mut(index, |arrow| {
                        arrow.participation_level =
                            relax_participation(arrow.participation_level, elapsed);
                    });
                    work.total = work.total.saturating_add(elapsed_u64);
                    execution_cost.touch::<Arrow>(1);
                    continue;
                }
                let material_index = snapshot.id.0 as usize;
                let before = self.core0_resistance[material_index];
                let decay_numerator = elapsed_u64
                    .saturating_mul(MATERIAL_ONE_U64)
                    .saturating_add(self.core0_decay_remainder[material_index]);
                let loss = decay_numerator / 10;
                self.core0_decay_remainder[material_index] = decay_numerator % 10;
                let active_ticks = elapsed_u64.min(
                    before
                        .saturating_mul(10)
                        .saturating_add(MATERIAL_ONE_U64 - 1)
                        / MATERIAL_ONE_U64,
                );
                let after = before.saturating_sub(loss);
                self.core0_resistance[material_index] = after;
                let deallocated = before > 0 && after == 0;
                self.arrows.with_mut(index, |arrow| {
                    arrow.participation_level =
                        relax_participation(arrow.participation_level, elapsed);
                    if deallocated {
                        decay_arrow(arrow, u32::MAX);
                    } else {
                        let observer =
                            after.saturating_add(MATERIAL_ONE_U64 - 1) / MATERIAL_ONE_U64;
                        arrow.resistance = u32::try_from(observer).unwrap_or(u32::MAX);
                    }
                });
                work.total = work.total.saturating_add(active_ticks);
                execution_cost.touch::<Arrow>(1);
                if deallocated && snapshot.delay == 0 {
                    self.zero_delay_live_arrows = self.zero_delay_live_arrows.saturating_sub(1);
                }
                if deallocated {
                    work.total = work.total.saturating_add(1);
                    work.physical_deallocations = work.physical_deallocations.saturating_add(1);
                    #[cfg(feature = "cl0")]
                    if let Some(trace) = physical_trace.as_deref_mut() {
                        trace.push(PhysicalTransition {
                            tick,
                            phase,
                            event: PhysicalEvent::Deallocate { arrow: snapshot.id },
                        });
                    }
                }
                continue;
            }
            #[cfg(feature = "core1")]
            if self.in_flight_protection
                || self.protect_all_live_arrows
                || (self.used_pending_protection_enabled && self.used_pending_count() > 0)
                || self.temporary_credit_return_count() > 0
            {
                let snapshot = self.arrows.get(index);
                if snapshot.live
                    && (self.protect_all_live_arrows
                        || self.in_flight_arrows[snapshot.id.0 as usize] > 0
                        || (self.used_pending_protection_enabled
                            && self.used_pending_arrows[snapshot.id.0 as usize])
                        || self.temporary_credit_return_arrows[snapshot.id.0 as usize]
                        || self.protected_by_temporary_credit_return(&snapshot))
                {
                    self.arrows.with_mut(index, |arrow| {
                        arrow.participation_level =
                            relax_participation(arrow.participation_level, elapsed);
                    });
                    work.total = work.total.saturating_add(elapsed_u64);
                    execution_cost.touch::<Arrow>(1);
                    continue;
                }
            }
            let (deallocated, zero_delay, active_ticks) = self.arrows.with_mut(index, |arrow| {
                if !arrow.live {
                    return (false, false, 0);
                }
                arrow.participation_level = relax_participation(arrow.participation_level, elapsed);
                let lifetime_remaining = u64::from(arrow.resistance)
                    .saturating_mul(u64::try_from(LOCAL_DECAY_PERIOD).unwrap_or(u64::MAX))
                    .saturating_sub(arrow.decay_load);
                let active_ticks = elapsed_u64.min(lifetime_remaining);
                let total_decay = arrow.decay_load.saturating_add(elapsed_u64);
                let durable_loss = total_decay / u64::try_from(LOCAL_DECAY_PERIOD).unwrap_or(1);
                arrow.decay_load =
                    total_decay % u64::try_from(LOCAL_DECAY_PERIOD).unwrap_or(u64::MAX);
                let was_live = arrow.live;
                if durable_loss > 0 {
                    decay_arrow(arrow, u32::try_from(durable_loss).unwrap_or(u32::MAX));
                }
                (was_live && !arrow.live, arrow.delay == 0, active_ticks)
            });
            work.total = work.total.saturating_add(active_ticks);
            execution_cost.touch::<Arrow>(1);
            if deallocated && zero_delay {
                self.zero_delay_live_arrows = self.zero_delay_live_arrows.saturating_sub(1);
            }
            if deallocated {
                work.total = work.total.saturating_add(1);
                work.physical_deallocations = work.physical_deallocations.saturating_add(1);
                #[cfg(feature = "cl0")]
                if let Some(trace) = physical_trace.as_deref_mut() {
                    trace.push(PhysicalTransition {
                        tick,
                        phase,
                        event: PhysicalEvent::Deallocate {
                            arrow: self.arrows.get(index).id,
                        },
                    });
                }
            }
        }
    }

    fn elapse_cells_to(
        &mut self,
        tick: i64,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        physical_trace: Option<&mut Vec<PhysicalTransition>>,
        phase: i32,
    ) {
        #[cfg(not(feature = "cl0"))]
        {
            let _ = (tick, work, execution_cost, physical_trace, phase);
        }
        #[cfg(all(feature = "cl0", not(feature = "j0")))]
        {
            let mut physical_trace = physical_trace;
            let elapsed = tick.saturating_sub(self.tick);
            let elapsed_u64 = u64::try_from(elapsed).unwrap_or(u64::MAX);
            for index in 0..self.cells.len() {
                execution_cost.scans = execution_cost.scans.saturating_add(1);
                let (id, deallocated, active_ticks, before_generation, after_generation) =
                    self.cells.with_mut(index, |cell| {
                        if !cell.live {
                            return (cell.id, false, 0, cell.generation, cell.generation);
                        }
                        #[cfg(feature = "cc0")]
                        {
                            cell.participation_level =
                                relax_participation(cell.participation_level, elapsed);
                        }
                        let lifetime_remaining = u64::from(cell.resistance)
                            .saturating_mul(u64::try_from(LOCAL_DECAY_PERIOD).unwrap_or(u64::MAX))
                            .saturating_sub(cell.decay_load);
                        let active_ticks = elapsed_u64.min(lifetime_remaining);
                        let total_decay = cell.decay_load.saturating_add(elapsed_u64);
                        let durable_loss =
                            total_decay / u64::try_from(LOCAL_DECAY_PERIOD).unwrap_or(1);
                        cell.decay_load =
                            total_decay % u64::try_from(LOCAL_DECAY_PERIOD).unwrap_or(u64::MAX);
                        let before_generation = cell.generation;
                        if durable_loss > 0 {
                            decay_cell_structure(
                                cell,
                                u32::try_from(durable_loss).unwrap_or(u32::MAX),
                            );
                        }
                        (
                            cell.id,
                            before_generation != cell.generation,
                            active_ticks,
                            before_generation,
                            cell.generation,
                        )
                    });
                work.total = work.total.saturating_add(active_ticks);
                execution_cost.touch::<Cell>(1);
                if deallocated {
                    if let Some(mapping) = self.cell_slots.get_mut(id.0 as usize) {
                        *mapping = None;
                    }
                    self.active_cells.remove(&id);
                    work.total = work.total.saturating_add(1);
                    work.cell_deallocations = work.cell_deallocations.saturating_add(1);
                    if let Some(trace) = physical_trace.as_deref_mut() {
                        trace.push(PhysicalTransition {
                            tick,
                            phase,
                            event: PhysicalEvent::CellDeallocate {
                                cell: id,
                                before_generation,
                                after_generation,
                            },
                        });
                    }
                }
            }
        }
        #[cfg(feature = "j0")]
        {
            let mut physical_trace = physical_trace;
            let arrows = self.arrows.values();
            let required = self
                .cells
                .values()
                .iter()
                .filter(|cell| {
                    cell.live
                        && arrows.iter().any(|arrow| {
                            arrow.live && (arrow.from == cell.id || arrow.to == cell.id)
                        })
                })
                .map(|cell| cell.id)
                .collect::<HashSet<_>>();
            execution_cost.scans = execution_cost.scans.saturating_add(
                u64::try_from(self.cells.len().saturating_mul(arrows.len())).unwrap_or(u64::MAX),
            );
            execution_cost.touch::<Cell>(self.cells.len());
            execution_cost.touch::<Arrow>(arrows.len());
            for index in 0..self.cells.len() {
                let cell = self.cells.get(index);
                if !cell.live || required.contains(&cell.id) {
                    continue;
                }
                let id = cell.id;
                let before_generation = cell.generation;
                let after_generation = Generation(before_generation.0.wrapping_add(1));
                self.cells.with_mut(index, |cell| {
                    cell.live = false;
                    cell.generation = after_generation;
                    cell.state = 0;
                    cell.refractory_until = 0;
                    cell.decay_load = 0;
                });
                if let Some(mapping) = self.cell_slots.get_mut(id.0 as usize) {
                    *mapping = None;
                }
                self.active_cells.remove(&id);
                work.total = work.total.saturating_add(1);
                work.cell_deallocations = work.cell_deallocations.saturating_add(1);
                if let Some(trace) = physical_trace.as_deref_mut() {
                    trace.push(PhysicalTransition {
                        tick,
                        phase,
                        event: PhysicalEvent::CellDeallocate {
                            cell: id,
                            before_generation,
                            after_generation,
                        },
                    });
                }
            }
        }
    }

    fn propose_local_arrows(
        &mut self,
        source: CellId,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        phase: i32,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) {
        #[cfg(feature = "core0")]
        {
            #[cfg(feature = "core1")]
            if self.core0_profile.generic_variation() {
                self.propose_generic_local_edits(
                    source,
                    work,
                    execution_cost,
                    phase,
                    physical_trace,
                );
                return;
            }
            if self.core0_profile.contact_variation() {
                self.propose_local_contacts(source, work, execution_cost, phase, physical_trace);
            } else {
                self.propose_direct_local_arrows(
                    source,
                    work,
                    execution_cost,
                    phase,
                    physical_trace,
                );
            }
            return;
        }
        #[cfg(not(feature = "core0"))]
        {
            #[cfg(any(feature = "cv0", feature = "cv0j0"))]
            self.propose_local_contacts(source, work, execution_cost, phase, physical_trace);
            #[cfg(not(any(feature = "cv0", feature = "cv0j0")))]
            self.propose_direct_local_arrows(source, work, execution_cost, phase, physical_trace);
        }
    }

    #[cfg(feature = "core1")]
    fn propose_generic_local_edits(
        &mut self,
        source: CellId,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        phase: i32,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) {
        let source_slot = self.cell_slot(source).expect("generic source must resolve");
        let source_cell = self.cells.get(source_slot.0);
        let source_position = source_cell.position;
        let source_region = source_cell.region;
        let distance_graded = self.core0_profile.distance_graded_variation();
        let cells = self.cells.values().clone();
        let arrows = self.arrows.values().clone();
        execution_cost.allocations = execution_cost.allocations.saturating_add(2);
        execution_cost.touch::<Cell>(cells.len());
        execution_cost.touch::<Arrow>(arrows.len());

        let mut targets = cells
            .iter()
            .filter_map(|cell| {
                let distance = cell.position.saturating_sub(source_position).abs();
                (cell.live
                    && cell.id != source
                    && distance > 0
                    && (distance_graded || distance <= LOCAL_VARIATION_RADIUS))
                    .then_some((distance, cell.position, cell.id))
            })
            .collect::<Vec<_>>();
        targets.sort_by_key(|target| (target.0, target.1));

        for (distance, _, target) in &targets {
            for sign in [1_i64, -1_i64] {
                let exists = arrows.iter().any(|arrow| {
                    arrow.live
                        && arrow.from == source
                        && arrow.to == *target
                        && arrow.mode == TransmissionMode::Drive
                        && self.core0_coupling[arrow.id.0 as usize].signum() == sign
                });
                if exists {
                    continue;
                }
                let magnitude = if distance_graded {
                    MATERIAL_ONE / i64::from(distance.saturating_add(1))
                } else {
                    MATERIAL_ONE
                };
                self.add_generic_arrow(
                    source,
                    *target,
                    magnitude.saturating_mul(sign),
                    i64::from((*distance).max(1)),
                    work,
                    phase,
                    physical_trace,
                );
            }
        }

        let source_is_subdivision = arrows.iter().any(|arrow| {
            arrow.live
                && arrow.to == source
                && self
                    .cell_slot(arrow.from)
                    .is_some_and(|slot| self.cells.get(slot.0).position == source_position)
        });
        if source_is_subdivision {
            return;
        }

        let direct = arrows
            .iter()
            .filter(|arrow| {
                if !arrow.live
                    || arrow.from != source
                    || arrow.mode != TransmissionMode::Drive
                    || arrow.trigger != TransmissionTrigger::SourceFires
                {
                    return false;
                }
                self.cell_slot(arrow.to).is_some_and(|slot| {
                    let target = self.cells.get(slot.0);
                    let distance = target.position.saturating_sub(source_position).abs();
                    distance > 0 && (distance_graded || distance <= LOCAL_VARIATION_RADIUS)
                })
            })
            .cloned()
            .collect::<Vec<_>>();

        for candidate in direct {
            let sign = self.core0_coupling[candidate.id.0 as usize].signum();
            let exists = cells.iter().any(|contact| {
                contact.live
                    && contact.id != source
                    && contact.id != candidate.to
                    && contact.position == source_position
                    && arrows
                        .iter()
                        .any(|arrow| arrow.live && arrow.from == source && arrow.to == contact.id)
                    && arrows.iter().any(|arrow| {
                        arrow.live
                            && arrow.from == contact.id
                            && arrow.to == candidate.to
                            && self.core0_coupling[arrow.id.0 as usize].signum() == sign
                    })
            });
            if exists {
                continue;
            }
            let next_physical = self
                .cells
                .values()
                .iter()
                .map(|cell| cell.physical_id)
                .max()
                .unwrap_or(0)
                .checked_add(1)
                .expect("generic CELL physical identity exhausted");
            let contact = self.add_cell(CellSpec {
                physical_id: next_physical,
                position: source_position,
                region: source_region,
                threshold: 1,
                resistance: 1,
            });
            work.total = work.total.saturating_add(1);
            work.local_cell_proposals = work.local_cell_proposals.saturating_add(1);
            if self.trace_physics {
                physical_trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase,
                    event: PhysicalEvent::CellProposal {
                        cell: contact,
                        source,
                        target: candidate.to,
                    },
                });
            }
            let target_position = self
                .cell_slot(candidate.to)
                .map(|slot| self.cells.get(slot.0).position)
                .expect("generic target must resolve");
            let distance = target_position.saturating_sub(source_position).abs();
            self.add_generic_arrow(
                source,
                contact,
                MATERIAL_ONE,
                1,
                work,
                phase,
                physical_trace,
            );
            self.add_generic_arrow(
                contact,
                candidate.to,
                self.core0_coupling[candidate.id.0 as usize],
                i64::from(distance.max(1)),
                work,
                phase,
                physical_trace,
            );
        }
    }

    #[cfg(feature = "core1")]
    fn add_generic_arrow(
        &mut self,
        from: CellId,
        to: CellId,
        material: i64,
        delay: i64,
        work: &mut Work,
        phase: i32,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) -> ArrowId {
        let id = self.add_arrow(ArrowSpec {
            from,
            to,
            delay,
            phase: 0,
            coupling: i32::try_from(material / MATERIAL_ONE).unwrap_or(0),
            resistance: 1,
            mode: TransmissionMode::Drive,
        });
        self.core0_coupling[id.0 as usize] = material;
        let slot = self.arrow_slot(id).expect("generic ARROW must resolve");
        self.arrows.with_mut(slot.0, |arrow| {
            arrow.coupling = material.signum() as i32;
            if arrow.generation == Generation(1) {
                arrow.generation =
                    Generation(u32::try_from(id.0).unwrap_or(u32::MAX).saturating_add(2));
            }
        });
        work.total = work.total.saturating_add(1);
        work.local_structural_proposals = work.local_structural_proposals.saturating_add(1);
        if self.trace_physics {
            physical_trace.push(PhysicalTransition {
                tick: self.tick,
                phase,
                event: PhysicalEvent::Proposal {
                    arrow: id,
                    from,
                    to,
                },
            });
        }
        id
    }

    #[cfg(any(feature = "core0", not(any(feature = "cv0", feature = "cv0j0"))))]
    fn propose_direct_local_arrows(
        &mut self,
        source: CellId,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        phase: i32,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) {
        execution_cost.allocations = execution_cost.allocations.saturating_add(1);
        let source_slot = self.cell_slot(source).unwrap();
        let source_position = self.cells.get(source_slot.0).position;
        execution_cost.touch::<Cell>(1);
        let mut targets = self
            .cells
            .values()
            .iter()
            .filter_map(|cell| {
                execution_cost.touch::<Cell>(1);
                let distance = cell.position.saturating_sub(source_position).abs();
                #[cfg(feature = "core0")]
                let within_opportunity = self.core0_profile == Core0Profile::D
                    || (1..=LOCAL_VARIATION_RADIUS).contains(&distance);
                #[cfg(not(feature = "core0"))]
                let within_opportunity = (1..=LOCAL_VARIATION_RADIUS).contains(&distance);
                (cell.id != source
                    && cell.live
                    && distance > 0
                    && within_opportunity
                    && !self.arrows.values().iter().any(|arrow| {
                        execution_cost.touch::<Arrow>(1);
                        arrow.live && arrow.from == source && arrow.to == cell.id
                    }))
                .then_some((
                    distance,
                    cell.position,
                    cell.region,
                    cell.threshold,
                    cell.resistance,
                    cell.generation.0,
                    cell.id,
                ))
            })
            .collect::<Vec<_>>();
        targets.sort_by_key(|target| (target.0, target.1, target.2, target.3, target.4, target.5));
        #[cfg(any(feature = "sv0", feature = "core0"))]
        let proposal_couplings: &[i32] = &[1, -1];
        #[cfg(not(any(feature = "sv0", feature = "core0")))]
        let proposal_couplings: &[i32] = &[1];
        for (distance, _, _, _, _, _, target) in targets {
            for coupling in proposal_couplings {
                let id = self.add_arrow(ArrowSpec {
                    from: source,
                    to: target,
                    delay: i64::from(distance.max(1)),
                    phase: 0,
                    coupling: *coupling,
                    resistance: 1,
                    mode: TransmissionMode::Drive,
                });
                let slot = self.arrow_slot(id).unwrap();
                #[cfg(feature = "core0")]
                if self.core0_profile == Core0Profile::D {
                    let magnitude = MATERIAL_ONE / i64::from(distance.saturating_add(1));
                    self.core0_coupling[id.0 as usize] =
                        magnitude.saturating_mul(i64::from(coupling.signum()));
                    self.arrows.with_mut(slot.0, |arrow| {
                        arrow.coupling = coupling.signum();
                    });
                }
                self.arrows.with_mut(slot.0, |arrow| {
                    if arrow.generation == Generation(1) {
                        arrow.generation =
                            Generation(u32::try_from(id.0).unwrap_or(u32::MAX).saturating_add(2));
                    }
                });
                work.total = work.total.saturating_add(1);
                work.local_structural_proposals = work.local_structural_proposals.saturating_add(1);
                if self.trace_physics {
                    physical_trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase,
                        event: PhysicalEvent::Proposal {
                            arrow: id,
                            from: source,
                            to: target,
                        },
                    });
                }
            }
        }
    }

    #[cfg(any(feature = "cv0", feature = "cv0j0"))]
    fn propose_local_contacts(
        &mut self,
        source: CellId,
        work: &mut Work,
        execution_cost: &mut ExecutionCost,
        phase: i32,
        physical_trace: &mut Vec<PhysicalTransition>,
    ) {
        execution_cost.allocations = execution_cost.allocations.saturating_add(1);
        let source_slot = self
            .cell_slot(source)
            .expect("proposal source must resolve");
        let source_cell = self.cells.get(source_slot.0);
        let source_position = source_cell.position;
        let source_region = source_cell.region;
        execution_cost.touch::<Cell>(1);

        let cells = self.cells.values();
        let arrows = self.arrows.values();
        execution_cost.touch::<Cell>(cells.len());
        execution_cost.touch::<Arrow>(arrows.len());
        execution_cost.scans = execution_cost
            .scans
            .saturating_add(u64::try_from(cells.len() + arrows.len()).unwrap_or(u64::MAX));
        let mut targets = cells
            .iter()
            .filter_map(|target| {
                let distance = target.position.saturating_sub(source_position).abs();
                if target.id == source
                    || !target.live
                    || !(1..=LOCAL_VARIATION_RADIUS).contains(&distance)
                {
                    return None;
                }
                let existing_direct = arrows
                    .iter()
                    .any(|arrow| arrow.live && arrow.from == source && arrow.to == target.id);
                let existing_contact_relation = cells.iter().any(|contact| {
                    contact.live
                        && contact.id != source
                        && contact.id != target.id
                        && contact.position == source_position
                        && arrows.iter().any(|arrow| {
                            arrow.live && arrow.from == source && arrow.to == contact.id
                        })
                        && arrows.iter().any(|arrow| {
                            arrow.live && arrow.from == contact.id && arrow.to == target.id
                        })
                });
                (!existing_direct && !existing_contact_relation).then_some((
                    distance,
                    target.position,
                    target.region,
                    target.threshold,
                    target.resistance,
                    target.generation.0,
                    target.id,
                ))
            })
            .collect::<Vec<_>>();
        targets.sort_by_key(|target| (target.0, target.1, target.2, target.3, target.4, target.5));

        for (distance, _, _, _, _, _, target) in targets {
            let free_cells = self.cell_capacity.saturating_sub(
                u32::try_from(self.cells.values().iter().filter(|cell| cell.live).count())
                    .unwrap_or(u32::MAX),
            );
            let free_arrows = self.arrow_capacity.saturating_sub(
                u32::try_from(
                    self.arrows
                        .values()
                        .iter()
                        .filter(|arrow| arrow.live)
                        .count(),
                )
                .unwrap_or(u32::MAX),
            );
            if free_cells < 2 || free_arrows < 4 {
                continue;
            }
            let next_physical = self
                .cells
                .values()
                .iter()
                .map(|cell| cell.physical_id)
                .max()
                .unwrap_or(0)
                .checked_add(1)
                .expect("generated CELL physical identity exhausted");
            let signs = [1, -1];
            for (offset, coupling) in signs.into_iter().enumerate() {
                let contact = self.add_cell(CellSpec {
                    physical_id: next_physical
                        .checked_add(u64::try_from(offset).unwrap_or(u64::MAX))
                        .expect("generated CELL physical identity exhausted"),
                    position: source_position,
                    region: source_region,
                    threshold: 1,
                    resistance: 1,
                });
                work.total = work.total.saturating_add(1);
                work.local_cell_proposals = work.local_cell_proposals.saturating_add(1);
                if self.trace_physics {
                    physical_trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase,
                        event: PhysicalEvent::CellProposal {
                            cell: contact,
                            source,
                            target,
                        },
                    });
                }
                for (from, to, arrow_coupling, delay) in [
                    (source, contact, 1, 1),
                    (contact, target, coupling, i64::from(distance.max(1))),
                ] {
                    let id = self.add_arrow(ArrowSpec {
                        from,
                        to,
                        delay,
                        phase: 0,
                        coupling: arrow_coupling,
                        resistance: 1,
                        mode: TransmissionMode::Drive,
                    });
                    let slot = self.arrow_slot(id).expect("proposed ARROW must resolve");
                    self.arrows.with_mut(slot.0, |arrow| {
                        if arrow.generation == Generation(1) {
                            arrow.generation = Generation(
                                u32::try_from(id.0).unwrap_or(u32::MAX).saturating_add(2),
                            );
                        }
                    });
                    work.total = work.total.saturating_add(1);
                    work.local_structural_proposals =
                        work.local_structural_proposals.saturating_add(1);
                    if self.trace_physics {
                        physical_trace.push(PhysicalTransition {
                            tick: self.tick,
                            phase,
                            event: PhysicalEvent::Proposal {
                                arrow: id,
                                from,
                                to,
                            },
                        });
                    }
                }
            }
        }
    }

    fn decay_cell(&mut self, cell: CellId, tick: i64) {
        let slot = self.cell_slot(cell).unwrap();
        let state = self.cells.with_mut(slot.0, |target| {
            let elapsed = tick.saturating_sub(target.last_update_tick);
            if elapsed > 0 {
                let decay = i32::try_from(elapsed).unwrap_or(i32::MAX);
                target.state = if target.state > 0 {
                    target.state.saturating_sub(decay).max(0)
                } else {
                    target.state.saturating_add(decay).min(0)
                };
                target.last_update_tick = tick;
            }
            target.state
        });
        if state == 0 {
            self.active_cells.remove(&cell);
        } else {
            self.active_cells.insert(cell);
        }
    }

    #[cfg(feature = "core0")]
    fn decay_core0_activation(&mut self, cell: CellId, tick: i64) {
        let slot = self.cell_slot(cell).expect("material CELL must resolve");
        let last_update_tick = self.cells.get(slot.0).last_update_tick;
        let elapsed = tick.saturating_sub(last_update_tick);
        if elapsed <= 0 {
            return;
        }
        let decay = i64::try_from(elapsed)
            .unwrap_or(i64::MAX)
            .saturating_mul(MATERIAL_ONE);
        let index = cell.0 as usize;
        let level = self.core0_activation[index];
        self.core0_activation[index] = if level > 0 {
            level.saturating_sub(decay).max(0)
        } else {
            level.saturating_add(decay).min(0)
        };
        let observer = self.core0_activation[index] / MATERIAL_ONE;
        self.cells.with_mut(slot.0, |target| {
            target.state = i32::try_from(observer).unwrap_or_else(|_| {
                if observer.is_negative() {
                    i32::MIN
                } else {
                    i32::MAX
                }
            });
            target.last_update_tick = tick;
        });
        if self.core0_activation[index] == 0 {
            self.active_cells.remove(&cell);
        } else {
            self.active_cells.insert(cell);
        }
    }

    #[cfg(not(feature = "si0"))]
    fn physical_arrow_order(&self, arrow: &Arrow) -> PhysicalArrowOrder {
        let from = self
            .cell_slot(arrow.from)
            .map(|slot| self.cells.get(slot.0));
        let to = self.cell_slot(arrow.to).map(|slot| self.cells.get(slot.0));
        PhysicalArrowOrder {
            phase: arrow.phase,
            delay: arrow.delay,
            from_live: from.as_ref().is_some_and(|cell| cell.live),
            from_position: from.as_ref().map_or(0, |cell| cell.position),
            from_region: from.as_ref().map_or(0, |cell| cell.region),
            from_threshold: from.as_ref().map_or(0, |cell| cell.threshold),
            to_live: to.as_ref().is_some_and(|cell| cell.live),
            to_position: to.as_ref().map_or(0, |cell| cell.position),
            to_region: to.as_ref().map_or(0, |cell| cell.region),
            to_threshold: to.as_ref().map_or(0, |cell| cell.threshold),
            mode: transmission_mode_byte(arrow.mode),
            trigger: transmission_trigger_byte(arrow.trigger),
            coupling: arrow.coupling,
            resistance: arrow.resistance,
            participation: arrow.participation_level,
            plastic_support: arrow.plastic_support,
            decay_load: arrow.decay_load,
            source_generation: arrow.source_generation.0,
            #[cfg(feature = "cl0")]
            target_generation: arrow.target_generation.0,
            #[cfg(not(feature = "cl0"))]
            target_generation: to.as_ref().map_or(0, |cell| cell.generation.0),
            arrow_generation: arrow.generation.0,
        }
    }

    #[cfg(not(feature = "si0"))]
    fn sort_arrow_ids_by_physics(&self, arrows: &mut [ArrowId]) {
        arrows.sort_by_key(|id| {
            let slot = self.arrow_slot(*id).expect("indexed ARROW must resolve");
            self.physical_arrow_order(&self.arrows.get(slot.0))
        });
    }

    fn require_cell(&self, id: CellId) {
        let valid = self.cell_slot(id).is_some_and(|slot| {
            let cell = self.cells.get(slot.0);
            cell.id == id && cell.live
        });
        assert!(valid, "cell must be live in this substrate");
    }

    fn cell_slot(&self, id: CellId) -> Option<CellSlot> {
        usize::try_from(id.0)
            .ok()
            .and_then(|index| self.cell_slots.get(index))
            .copied()
            .flatten()
    }

    fn arrow_slot(&self, id: ArrowId) -> Option<ArrowSlot> {
        usize::try_from(id.0)
            .ok()
            .and_then(|index| self.arrow_slots.get(index))
            .copied()
            .flatten()
    }

    pub fn cell_reference(&self, id: CellId) -> CellRef {
        let slot = self
            .cell_slot(id)
            .expect("stored CELL identity must resolve");
        assert_eq!(
            self.cells.get(slot.0).id,
            id,
            "CELL identity must be current"
        );
        CellRef {
            arena: self.arena,
            id,
            generation: self.cells.get(slot.0).generation,
        }
    }

    pub fn arrow_reference(&self, id: ArrowId) -> ArrowRef {
        let slot = self
            .arrow_slot(id)
            .expect("stored ARROW identity must resolve");
        let arrow = self.arrows.get(slot.0);
        ArrowRef {
            arena: self.arena,
            id,
            generation: arrow.generation,
        }
    }

    pub fn local_participation(&self, id: ArrowId) -> u64 {
        self.arrows
            .get(self.arrow_slot(id).expect("ARROW identity must resolve").0)
            .participation_level
    }

    pub fn local_plastic_support(&self, id: ArrowId) -> u64 {
        self.arrows
            .get(self.arrow_slot(id).expect("ARROW identity must resolve").0)
            .plastic_support
    }

    pub fn local_pressure_load(&self, id: ArrowId) -> u64 {
        self.local_decay_load(id)
    }

    pub fn local_decay_load(&self, id: ArrowId) -> u64 {
        self.arrows
            .get(self.arrow_slot(id).expect("ARROW identity must resolve").0)
            .decay_load
    }

    #[cfg(feature = "core0")]
    pub fn core0_coupling_material(&self, id: ArrowId) -> i64 {
        if self.core0_profile == Core0Profile::A {
            let arrow = self
                .arrows
                .get(self.arrow_slot(id).expect("ARROW identity must resolve").0);
            return i64::from(arrow.coupling).saturating_mul(MATERIAL_ONE);
        }
        self.core0_coupling[id.0 as usize]
    }

    #[cfg(feature = "core0")]
    pub fn core0_resistance_material(&self, id: ArrowId) -> u64 {
        if self.core0_profile == Core0Profile::A {
            let arrow = self
                .arrows
                .get(self.arrow_slot(id).expect("ARROW identity must resolve").0);
            return u64::from(arrow.resistance).saturating_mul(MATERIAL_ONE_U64);
        }
        self.core0_resistance[id.0 as usize]
    }

    #[cfg(feature = "core0")]
    pub fn core0_activation_material(&self, id: CellId) -> i64 {
        if self.core0_profile == Core0Profile::A {
            let cell = self
                .cells
                .get(self.cell_slot(id).expect("CELL identity must resolve").0);
            return i64::from(cell.state).saturating_mul(MATERIAL_ONE);
        }
        self.core0_activation[id.0 as usize]
    }

    #[cfg(feature = "core0")]
    pub const fn core0_material_one() -> u64 {
        MATERIAL_ONE_U64
    }

    /// Fixture-only material initialization for frozen CORE0-family
    /// characterization worlds. This changes no transition law.
    #[cfg(feature = "core0")]
    pub fn set_core0_coupling_material(&mut self, id: ArrowId, value: i64) -> bool {
        if !self.core0_profile.continuous() {
            return false;
        }
        let Some(slot) = self.arrow_slot(id) else {
            return false;
        };
        if !self.arrows.get(slot.0).live {
            return false;
        }
        self.core0_coupling[id.0 as usize] = value;
        let observer = value / MATERIAL_ONE;
        self.arrows.with_mut(slot.0, |arrow| {
            arrow.coupling = i32::try_from(observer).unwrap_or_else(|_| {
                if observer.is_negative() {
                    i32::MIN
                } else {
                    i32::MAX
                }
            });
        });
        true
    }

    #[cfg(feature = "core0")]
    pub fn pending_physical_activity(&self) -> usize {
        self.pending.len()
    }

    #[cfg(feature = "cl0")]
    pub fn cell_resistance(&self, id: CellId) -> Option<u32> {
        self.cells
            .values()
            .iter()
            .find(|cell| cell.id == id)
            .map(|cell| cell.resistance)
    }

    #[cfg(feature = "cl0")]
    pub fn cell_generation(&self, id: CellId) -> Option<Generation> {
        self.cells
            .values()
            .iter()
            .find(|cell| cell.id == id)
            .map(|cell| cell.generation)
    }

    #[cfg(feature = "cl0")]
    pub fn cell_is_live(&self, id: CellId) -> Option<bool> {
        self.cells
            .values()
            .iter()
            .find(|cell| cell.id == id)
            .map(|cell| cell.live)
    }

    #[cfg(feature = "cl0")]
    pub fn cell_decay_load(&self, id: CellId) -> Option<u64> {
        self.cells
            .values()
            .iter()
            .find(|cell| cell.id == id)
            .map(|cell| cell.decay_load)
    }

    #[cfg(feature = "cc0")]
    pub fn cell_participation(&self, id: CellId) -> Option<u64> {
        self.cells
            .values()
            .iter()
            .find(|cell| cell.id == id)
            .map(|cell| cell.participation_level)
    }

    #[cfg(feature = "cl0")]
    pub fn cell_resident_slot(&self, id: CellId) -> Option<CellSlot> {
        self.cells
            .values()
            .iter()
            .position(|cell| cell.id == id)
            .map(CellSlot)
    }

    fn rebuild_slot_maps(&mut self) {
        self.cell_slots.fill(None);
        for (index, cell) in self.cells.values().iter().enumerate() {
            self.cell_slots[cell.id.0 as usize] = cell.live.then_some(CellSlot(index));
        }
        self.arrow_slots.fill(None);
        for (index, arrow) in self.arrows.values().iter().enumerate() {
            self.arrow_slots[arrow.id.0 as usize] = Some(ArrowSlot(index));
        }
    }

    fn rebuild_mechanical_indexes(&mut self) {
        self.outgoing_index = vec![Vec::new(); self.cell_slots.len()];
        for arrow in self.arrows.values() {
            self.outgoing_index[arrow.from.0 as usize].push(arrow.id);
        }
        self.active_cells = self
            .cells
            .values()
            .into_iter()
            .filter(|cell| cell.state != 0)
            .map(|cell| cell.id)
            .collect();
        self.zero_delay_live_arrows = self
            .arrows
            .values()
            .into_iter()
            .filter(|arrow| arrow.live && arrow.delay == 0)
            .count();
    }

    fn resident_bytes(&self) -> usize {
        self.cells.len() * std::mem::size_of::<Cell>()
            + self.arrows.len() * std::mem::size_of::<Arrow>()
    }

    fn mechanical_resident_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            .saturating_add(self.cells.resident_bytes())
            .saturating_add(self.arrows.resident_bytes())
            .saturating_add(
                self.cell_slots
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Option<CellSlot>>()),
            )
            .saturating_add(
                self.arrow_slots
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Option<ArrowSlot>>()),
            )
            .saturating_add(self.pending.resident_bytes())
            .saturating_add(
                self.outgoing_index
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Vec<ArrowId>>()),
            )
            .saturating_add(
                self.outgoing_index
                    .iter()
                    .map(|ids| {
                        ids.capacity()
                            .saturating_mul(std::mem::size_of::<ArrowId>())
                    })
                    .sum::<usize>(),
            )
            .saturating_add(
                self.resident_arenas
                    .capacity()
                    .saturating_mul(std::mem::size_of::<ResidentArenaId>()),
            )
            .saturating_add(
                self.active_cells.len().saturating_mul(
                    std::mem::size_of::<CellId>() + 3 * std::mem::size_of::<usize>(),
                ),
            )
            .saturating_add({
                #[cfg(feature = "core1")]
                {
                    self.in_flight_arrows
                        .capacity()
                        .saturating_mul(std::mem::size_of::<u32>())
                        .saturating_add(
                            self.used_pending_arrows
                                .capacity()
                                .saturating_mul(std::mem::size_of::<bool>()),
                        )
                        .saturating_add(
                            self.temporary_credit_return_arrows
                                .capacity()
                                .saturating_mul(std::mem::size_of::<bool>()),
                        )
                }
                #[cfg(not(feature = "core1"))]
                {
                    0
                }
            })
    }
}

fn decay_arrow(arrow: &mut Arrow, amount: u32) {
    arrow.resistance = arrow.resistance.saturating_sub(amount);
    if arrow.resistance == 0 && arrow.live {
        arrow.live = false;
        arrow.generation = Generation(arrow.generation.0.wrapping_add(1));
        arrow.participation_level = 0;
        arrow.plastic_support = 0;
        arrow.decay_load = 0;
    }
}

#[cfg(all(feature = "cl0", not(feature = "j0")))]
fn decay_cell_structure(cell: &mut Cell, amount: u32) {
    cell.resistance = cell.resistance.saturating_sub(amount);
    if cell.resistance == 0 && cell.live {
        cell.live = false;
        cell.generation = Generation(cell.generation.0.wrapping_add(1));
        cell.state = 0;
        cell.refractory_until = 0;
        cell.decay_load = 0;
        #[cfg(feature = "cc0")]
        {
            cell.participation_level = 0;
        }
    }
}

fn relax_participation(mut level: u64, elapsed: i64) -> u64 {
    for _ in 0..u64::try_from(elapsed).unwrap_or(u64::MAX) {
        level =
            level.saturating_mul(PARTICIPATION_RELAX_NUMERATOR) / PARTICIPATION_RELAX_DENOMINATOR;
    }
    level
}

fn local_consequence_gain(participation: u64) -> u32 {
    let bounded = participation.min(PARTICIPATION_IMPULSE);
    let numerator = u128::from(bounded).saturating_mul(u128::from(LOCAL_RETURN_STRENGTH));
    let gain = numerator.saturating_add(u128::from(PARTICIPATION_IMPULSE).saturating_sub(1))
        / u128::from(PARTICIPATION_IMPULSE);
    u32::try_from(gain).unwrap_or(LOCAL_RETURN_STRENGTH)
}

fn pressure_epoch(tick: i64) -> i64 {
    tick.div_euclid(LOCAL_DECAY_PERIOD)
        .saturating_mul(LOCAL_DECAY_PERIOD)
}

fn transmission_mode_byte(mode: TransmissionMode) -> u8 {
    match mode {
        TransmissionMode::Drive => 0,
        TransmissionMode::Modulatory => 1,
    }
}

fn transmission_mode_from_byte(mode: u8) -> Result<TransmissionMode, CheckpointError> {
    match mode {
        0 => Ok(TransmissionMode::Drive),
        1 => Ok(TransmissionMode::Modulatory),
        other => Err(CheckpointError::UnsupportedTransmissionMode(other)),
    }
}

fn transmission_trigger_byte(trigger: TransmissionTrigger) -> u8 {
    match trigger {
        TransmissionTrigger::SourceFires => 0,
        TransmissionTrigger::QualifiedLocalParticipation => 1,
    }
}

fn transmission_trigger_from_byte(trigger: u8) -> Result<TransmissionTrigger, CheckpointError> {
    match trigger {
        0 => Ok(TransmissionTrigger::SourceFires),
        1 => Ok(TransmissionTrigger::QualifiedLocalParticipation),
        other => Err(CheckpointError::UnsupportedTransmissionMode(other)),
    }
}

#[cfg(feature = "core1")]
fn core0_profile_byte(profile: Core0Profile) -> u8 {
    match profile {
        Core0Profile::A => 0,
        Core0Profile::B => 1,
        Core0Profile::C => 2,
        Core0Profile::D => 3,
        Core0Profile::GenericExternal => 4,
        Core0Profile::GenericActivity => 5,
        Core0Profile::GenericDistance => 6,
        Core0Profile::GenericDistanceNoQlp => 7,
    }
}

#[cfg(feature = "core1")]
fn core0_profile_from_byte(profile: u8) -> Result<Core0Profile, CheckpointError> {
    match profile {
        0 => Ok(Core0Profile::A),
        1 => Ok(Core0Profile::B),
        2 => Ok(Core0Profile::C),
        3 => Ok(Core0Profile::D),
        4 => Ok(Core0Profile::GenericExternal),
        5 => Ok(Core0Profile::GenericActivity),
        6 => Ok(Core0Profile::GenericDistance),
        7 => Ok(Core0Profile::GenericDistanceNoQlp),
        _ => Err(CheckpointError::InvalidCheckpoint),
    }
}

fn validate_manifest(manifest: &BodyVersion, body: &ArenaBody) -> Result<(), CheckpointError> {
    let hash = body.content_hash()?;
    let matches = manifest.arenas.len() == 1
        && manifest.arenas[0].arena == body.arena
        && manifest.arenas[0].block == hash;
    if matches {
        Ok(())
    } else {
        Err(CheckpointError::ManifestMismatch)
    }
}

fn checkpoint_put_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_put_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_len_u32(length: usize) -> Result<u32, CheckpointError> {
    u32::try_from(length).map_err(|_| CheckpointError::InvalidCheckpoint)
}

fn checkpoint_len_u64(length: usize) -> Result<u64, CheckpointError> {
    u64::try_from(length).map_err(|_| CheckpointError::InvalidCheckpoint)
}

fn checkpoint_put_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_put_i32(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_put_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_put_i64(bytes: &mut Vec<u8>, value: i64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn checkpoint_put_optional_tick(bytes: &mut Vec<u8>, tick: Option<i64>) {
    bytes.push(u8::from(tick.is_some()));
    checkpoint_put_i64(bytes, tick.unwrap_or_default());
}

fn encode_input(bytes: &mut Vec<u8>, input: &SpikeInput) {
    checkpoint_put_i64(bytes, input.arrival_tick);
    checkpoint_put_i32(bytes, input.phase);
    checkpoint_put_u64(bytes, input.origin_physical);
    checkpoint_put_u64(bytes, input.target.0);
    checkpoint_put_i32(bytes, input.impulse);
}

fn decode_input(cursor: &mut CheckpointCursor<'_>) -> Result<SpikeInput, CheckpointError> {
    Ok(SpikeInput {
        arrival_tick: cursor.i64()?,
        phase: cursor.i32()?,
        origin_physical: cursor.u64()?,
        target: CellId(cursor.u64()?),
        impulse: cursor.i32()?,
    })
}

fn encode_crossing(bytes: &mut Vec<u8>, crossing: &Crossing) {
    checkpoint_put_i64(bytes, crossing.tick);
    checkpoint_put_u64(bytes, crossing.from_physical);
    checkpoint_put_u64(bytes, crossing.to_physical);
    checkpoint_put_i16(bytes, crossing.from_region);
    checkpoint_put_i16(bytes, crossing.to_region);
    checkpoint_put_i32(bytes, crossing.impulse);
}

fn decode_crossing(cursor: &mut CheckpointCursor<'_>) -> Result<Crossing, CheckpointError> {
    Ok(Crossing {
        tick: cursor.i64()?,
        from_physical: cursor.u64()?,
        to_physical: cursor.u64()?,
        from_region: cursor.i16()?,
        to_region: cursor.i16()?,
        impulse: cursor.i32()?,
    })
}

fn encode_spike(bytes: &mut Vec<u8>, spike: &Spike) {
    checkpoint_put_i64(bytes, spike.arrival_tick);
    checkpoint_put_i32(bytes, spike.phase);
    #[cfg(feature = "si0")]
    checkpoint_put_u64(bytes, spike.causal_wave);
    checkpoint_put_u64(bytes, spike.origin_physical);
    #[cfg(feature = "cl0")]
    checkpoint_put_u64(bytes, spike.target_physical);
    checkpoint_put_u64(bytes, spike.target.0);
    checkpoint_put_u32(bytes, spike.target_generation.0);
    checkpoint_put_i32(bytes, spike.impulse);
    #[cfg(feature = "core0")]
    checkpoint_put_i64(bytes, spike.material_impulse);
    checkpoint_put_u64(bytes, spike.serial);
    bytes.push(u8::from(spike.arrow.is_some()));
    let (arrow, generation) = spike.arrow.unwrap_or((ArrowId(0), Generation(0)));
    checkpoint_put_u64(bytes, arrow.0);
    checkpoint_put_u32(bytes, generation.0);
}

fn decode_spike(cursor: &mut CheckpointCursor<'_>) -> Result<Spike, CheckpointError> {
    let arrival_tick = cursor.i64()?;
    let phase = cursor.i32()?;
    #[cfg(feature = "si0")]
    let causal_wave = cursor.u64()?;
    let origin_physical = cursor.u64()?;
    #[cfg(feature = "cl0")]
    let target_physical = cursor.u64()?;
    let target = CellId(cursor.u64()?);
    let target_generation = Generation(cursor.u32()?);
    let impulse = cursor.i32()?;
    #[cfg(feature = "core0")]
    let material_impulse = cursor.i64()?;
    let serial = cursor.u64()?;
    let arrow_present = cursor.u8()?;
    if arrow_present > 1 {
        return Err(CheckpointError::InvalidCheckpoint);
    }
    let arrow_id = ArrowId(cursor.u64()?);
    let arrow_generation = Generation(cursor.u32()?);
    Ok(Spike {
        arrival_tick,
        phase,
        #[cfg(feature = "si0")]
        causal_wave,
        origin_physical,
        #[cfg(feature = "cl0")]
        target_physical,
        target,
        target_generation,
        impulse,
        #[cfg(feature = "core0")]
        material_impulse,
        serial,
        arrow: (arrow_present == 1).then_some((arrow_id, arrow_generation)),
    })
}

struct CheckpointCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> CheckpointCursor<'a> {
    fn new(bytes: &'a [u8], offset: usize) -> Self {
        Self { bytes, offset }
    }

    fn take<const N: usize>(&mut self) -> Result<[u8; N], CheckpointError> {
        let end = self
            .offset
            .checked_add(N)
            .ok_or(CheckpointError::Truncated)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(CheckpointError::Truncated)?;
        self.offset = end;
        value.try_into().map_err(|_| CheckpointError::Truncated)
    }

    fn bytes(&mut self, length: usize) -> Result<&'a [u8], CheckpointError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(CheckpointError::Truncated)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(CheckpointError::Truncated)?;
        self.offset = end;
        Ok(value)
    }

    fn finish(&self) -> Result<(), CheckpointError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(CheckpointError::TrailingBytes)
        }
    }

    fn u8(&mut self) -> Result<u8, CheckpointError> {
        Ok(self.take::<1>()?[0])
    }

    fn u16(&mut self) -> Result<u16, CheckpointError> {
        Ok(u16::from_le_bytes(self.take()?))
    }

    fn i16(&mut self) -> Result<i16, CheckpointError> {
        Ok(i16::from_le_bytes(self.take()?))
    }

    fn u32(&mut self) -> Result<u32, CheckpointError> {
        Ok(u32::from_le_bytes(self.take()?))
    }

    fn i32(&mut self) -> Result<i32, CheckpointError> {
        Ok(i32::from_le_bytes(self.take()?))
    }

    fn u64(&mut self) -> Result<u64, CheckpointError> {
        Ok(u64::from_le_bytes(self.take()?))
    }

    fn i64(&mut self) -> Result<i64, CheckpointError> {
        Ok(i64::from_le_bytes(self.take()?))
    }

    fn usize_from_u32(&mut self) -> Result<usize, CheckpointError> {
        usize::try_from(self.u32()?).map_err(|_| CheckpointError::InvalidCheckpoint)
    }

    fn usize_from_u64(&mut self) -> Result<usize, CheckpointError> {
        usize::try_from(self.u64()?).map_err(|_| CheckpointError::InvalidCheckpoint)
    }

    fn array_32(&mut self) -> Result<[u8; 32], CheckpointError> {
        self.take()
    }

    fn optional_tick(&mut self) -> Result<Option<i64>, CheckpointError> {
        let present = self.u8()?;
        let tick = self.i64()?;
        match present {
            0 => Ok(None),
            1 => Ok(Some(tick)),
            _ => Err(CheckpointError::InvalidCheckpoint),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn substrate(arrow_resistance: u32) -> (PlasticSubstrate, CellId, CellId, ArrowId) {
        let mut substrate = PlasticSubstrate::with_capacity(ArenaId(42), 8, 8);
        let source = substrate.add_cell(CellSpec {
            physical_id: 900,
            position: 10,
            region: 0,
            threshold: 1,
            resistance: 10,
        });
        let target = substrate.add_cell(CellSpec {
            physical_id: 100,
            position: 20,
            region: 1,
            threshold: 1,
            resistance: 10,
        });
        let arrow = substrate.add_arrow(ArrowSpec {
            from: source,
            to: target,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: arrow_resistance,
            mode: TransmissionMode::Drive,
        });
        (substrate, source, target, arrow)
    }

    fn input(target: CellId, tick: i64) -> SpikeInput {
        SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: 7,
            target,
            impulse: 1,
        }
    }

    fn physical_work(work: Work) -> (u64, u64, u64, u64, u64) {
        (
            work.drive_deliveries,
            work.modulatory_deliveries,
            work.local_return_updates,
            work.local_structural_proposals,
            work.physical_deallocations,
        )
    }

    fn differential_body() -> (PlasticSubstrate, CellId) {
        let mut body = PlasticSubstrate::with_capacity(ArenaId(700), 16, 32);
        let cells = (0..8)
            .map(|index| {
                body.add_cell(CellSpec {
                    physical_id: 10_000 + index,
                    position: (index as i32) * 10,
                    region: if index < 6 { 0 } else { 1 },
                    threshold: if index == 0 { 2 } else { 1 },
                    resistance: 20,
                })
            })
            .collect::<Vec<_>>();
        let arrows = [
            (0, 1, 0, -3, TransmissionMode::Drive),
            (0, 2, 1, 2, TransmissionMode::Drive),
            (1, 3, 65, 0, TransmissionMode::Drive),
            (2, 3, 2, 1, TransmissionMode::Drive),
            (3, 4, 1, 0, TransmissionMode::Drive),
            (4, 0, 1, 4, TransmissionMode::Modulatory),
            (3, 6, 3, 0, TransmissionMode::Drive),
            (4, 7, 4, 0, TransmissionMode::Drive),
        ];
        for (from, to, delay, phase, mode) in arrows {
            body.add_arrow(ArrowSpec {
                from: cells[from],
                to: cells[to],
                delay,
                phase,
                coupling: 1,
                resistance: 40,
                mode,
            });
        }
        (body, cells[0])
    }

    fn assert_physical_equivalence(
        reference: &PlasticSubstrate,
        reference_result: &RunResult,
        candidate: &PlasticSubstrate,
        candidate_result: &RunResult,
    ) {
        assert_eq!(candidate_result.crossings, reference_result.crossings);
        assert_eq!(
            physical_work(candidate_result.work),
            physical_work(reference_result.work)
        );
        assert_eq!(
            candidate_result.work.physical_total(),
            reference_result.work.physical_total()
        );
        assert_eq!(candidate.clock(), reference.clock());
        assert_eq!(
            candidate.clock().pressure_phase(),
            reference.clock().pressure_phase()
        );
        assert_eq!(
            candidate.canonical_body_bytes(991).unwrap(),
            reference.canonical_body_bytes(991).unwrap()
        );
        assert_eq!(
            candidate_result.naturally_quiescent,
            reference_result.naturally_quiescent
        );
        assert_eq!(
            candidate_result.physical_trace,
            reference_result.physical_trace
        );
    }

    #[test]
    fn r1_r5_mechanical_prefixes_preserve_physics() {
        let configs = [
            MechanicalConfig::R1,
            MechanicalConfig::R2,
            MechanicalConfig::R3,
            MechanicalConfig::R4,
            MechanicalConfig::R5,
        ];
        for origin in [0, 130, 260, 390] {
            let (base, source) = differential_body();
            let arrivals = [
                SpikeInput {
                    arrival_tick: origin,
                    phase: 0,
                    origin_physical: 91,
                    target: source,
                    impulse: 1,
                },
                SpikeInput {
                    arrival_tick: origin,
                    phase: 1,
                    origin_physical: 92,
                    target: source,
                    impulse: 1,
                },
                SpikeInput {
                    arrival_tick: origin + 70,
                    phase: -1,
                    origin_physical: 93,
                    target: source,
                    impulse: 2,
                },
            ];
            let mut reference = base.clone();
            for arrival in arrivals {
                reference.enter(arrival);
            }
            let canonical_pending = reference
                .live_checkpoint(990)
                .unwrap()
                .canonical_bytes()
                .unwrap();
            let reference_result = reference.propagate();
            for config in configs {
                let mut candidate = base.clone();
                candidate.reconfigure_mechanics(config);
                for arrival in arrivals {
                    candidate.enter(arrival);
                }
                assert_eq!(
                    candidate
                        .live_checkpoint(990)
                        .unwrap()
                        .canonical_bytes()
                        .unwrap(),
                    canonical_pending,
                    "canonical pending activity differs for {config:?}"
                );
                let candidate_result = candidate.propagate();
                assert_physical_equivalence(
                    &reference,
                    &reference_result,
                    &candidate,
                    &candidate_result,
                );

                let reference_pressure = {
                    let mut value = reference.clone();
                    value.advance_time(reference.clock().tick + 100)
                };
                let candidate_pressure = {
                    let mut value = candidate.clone();
                    value.advance_time(candidate.clock().tick + 100)
                };
                assert_eq!(
                    physical_work(candidate_pressure),
                    physical_work(reference_pressure),
                    "pressure work differs for {config:?}"
                );
            }
        }
    }

    #[test]
    fn resident_partition_preserves_identity_pending_order_and_physics() {
        let (base, source) = differential_body();
        let arrivals = [
            SpikeInput {
                arrival_tick: 0,
                phase: 0,
                origin_physical: 91,
                target: source,
                impulse: 1,
            },
            SpikeInput {
                arrival_tick: 0,
                phase: 1,
                origin_physical: 92,
                target: source,
                impulse: 1,
            },
            SpikeInput {
                arrival_tick: 70,
                phase: -1,
                origin_physical: 93,
                target: source,
                impulse: 2,
            },
        ];
        let mut one_arena = base.clone();
        one_arena.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
        let durable_reference = one_arena.cell_reference(source);
        let mut partitioned = base;
        partitioned.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
        partitioned.repartition_resident(&[
            ResidentArenaId(0),
            ResidentArenaId(1),
            ResidentArenaId(2),
            ResidentArenaId(3),
            ResidentArenaId(0),
            ResidentArenaId(1),
            ResidentArenaId(2),
            ResidentArenaId(3),
        ]);
        assert_eq!(partitioned.resident_arena_count(), 4);
        assert_eq!(partitioned.cell_reference(source), durable_reference);
        assert_eq!(
            partitioned.canonical_body_bytes(992).unwrap(),
            one_arena.canonical_body_bytes(992).unwrap()
        );
        for arrival in arrivals {
            one_arena.enter(arrival);
            partitioned.enter(arrival);
        }
        assert_eq!(
            partitioned
                .live_checkpoint(993)
                .unwrap()
                .canonical_bytes()
                .unwrap(),
            one_arena
                .live_checkpoint(993)
                .unwrap()
                .canonical_bytes()
                .unwrap()
        );
        let one_arena_result = one_arena.propagate();
        let partitioned_result = partitioned.propagate();
        assert!(partitioned_result.execution_cost.arena_hops > 0);
        assert_physical_equivalence(
            &one_arena,
            &one_arena_result,
            &partitioned,
            &partitioned_result,
        );
    }

    #[test]
    fn r5_batches_safe_same_tick_activity_without_changing_physics() {
        let mut base = PlasticSubstrate::with_capacity(ArenaId(701), 2, 2);
        let target = base.add_cell(CellSpec {
            physical_id: 55_000,
            position: 0,
            region: 0,
            threshold: 100,
            resistance: 20,
        });
        let arrivals = (0..100)
            .map(|serial| SpikeInput {
                arrival_tick: 10,
                phase: serial % 3,
                origin_physical: 70_000 + serial as u64,
                target,
                impulse: 1,
            })
            .collect::<Vec<_>>();

        let mut scalar = base.clone();
        scalar.reconfigure_mechanics(MechanicalConfig::R4);
        let scalar_result = scalar.arrive(&arrivals, 1);

        let mut batched = base;
        batched.reconfigure_mechanics(MechanicalConfig::R5);
        let batched_result = batched.arrive(&arrivals, 1);

        assert_physical_equivalence(&scalar, &scalar_result, &batched, &batched_result);
        #[cfg(not(feature = "si0"))]
        assert!(
            batched_result.execution_cost.queue_ops < scalar_result.execution_cost.queue_ops,
            "batched scheduler must consume fewer queue operations"
        );
        #[cfg(feature = "si0")]
        assert_eq!(
            batched_result.execution_cost.queue_ops, scalar_result.execution_cost.queue_ops,
            "causal-wave draining subsumes the former executor-specific batch boundary"
        );
    }

    #[test]
    fn r4_soa_compaction_and_restart_preserve_stable_identity() {
        for config in [MechanicalConfig::R4, MechanicalConfig::R5] {
            let (mut ordinary, source, target, arrow) = substrate(50);
            ordinary.reconfigure_mechanics(config);
            ordinary.add_arrow(ArrowSpec {
                from: target,
                to: source,
                delay: 1,
                phase: 0,
                coupling: 0,
                resistance: 50,
                mode: TransmissionMode::Drive,
            });
            ordinary.enter(input(source, 5));
            let checkpoint = ordinary.live_checkpoint(811).unwrap();
            let mut restored =
                PlasticSubstrate::from_live_checkpoint_with_mechanics(checkpoint, config).unwrap();
            let source_reference = restored.cell_reference(source);
            let arrow_reference = restored.arrow_reference(arrow);
            let source_slot_before = restored.resolve_cell(source_reference).unwrap();
            let arrow_slot_before = restored.resolve_arrow(arrow_reference).unwrap();
            restored.compact_resident();
            assert_ne!(
                restored.resolve_cell(source_reference).unwrap(),
                source_slot_before
            );
            assert_ne!(
                restored.resolve_arrow(arrow_reference).unwrap(),
                arrow_slot_before
            );
            let restored_result = restored.propagate();
            let ordinary_result = ordinary.propagate();
            assert_physical_equivalence(&ordinary, &ordinary_result, &restored, &restored_result);
        }
    }

    #[test]
    fn compaction_changes_slots_not_physics() {
        let (original, source, _, _) = substrate(50);
        let reference = original.cell_reference(source);
        let before = original.resolve_cell(reference).unwrap();
        let mut compacted = original.clone();
        compacted.compact_resident();
        let after = compacted.resolve_cell(reference).unwrap();
        assert_ne!(before, after);

        let mut ordinary = original;
        let ordinary_result = ordinary.arrive(&[input(source, 0)], 1);
        let compacted_result = compacted.arrive(&[input(source, 0)], 1);
        assert_physical_equivalence(&ordinary, &ordinary_result, &compacted, &compacted_result);
    }

    #[test]
    fn canonical_body_round_trip_is_structurally_exact() {
        let (substrate, _, _, _) = substrate(50);
        let bytes = substrate.canonical_body_bytes(3).unwrap();
        let restored = PlasticSubstrate::from_body_bytes(&bytes).unwrap();
        assert_eq!(restored.canonical_body_bytes(3).unwrap(), bytes);
    }

    #[test]
    fn quiescent_checkpoint_preserves_clock_phase_and_future_behavior() {
        let (mut substrate, source, _, _) = substrate(50);
        substrate.advance_time(23);
        let checkpoint = substrate.quiescent_checkpoint(4).unwrap();
        assert_eq!(checkpoint.clock.pressure_phase(), 3);
        let bytes = checkpoint.canonical_bytes().unwrap();
        let decoded = QuiescentCheckpoint::decode(&bytes).unwrap();
        assert_eq!(decoded.canonical_bytes().unwrap(), bytes);
        let mut restored = PlasticSubstrate::from_quiescent_checkpoint(decoded).unwrap();
        assert_eq!(restored.clock(), substrate.clock());
        let substrate_result = substrate.arrive(&[input(source, 24)], 1);
        let restored_result = restored.arrive(&[input(source, 24)], 1);
        assert_physical_equivalence(&substrate, &substrate_result, &restored, &restored_result);
    }

    #[test]
    fn live_checkpoint_preserves_pending_activity_and_load_availability() {
        let (mut substrate, source, _, _) = substrate(50);
        substrate.enter(input(source, 5));
        substrate.register_pending_load(PendingLoad {
            arena: ArenaId(99),
            version: ContentHash([3; 32]),
            issue_tick: 0,
            availability_tick: Some(7),
            waiting_arrivals: vec![input(source, 8)],
        });
        let checkpoint = substrate.live_checkpoint(5).unwrap();
        let bytes = checkpoint.canonical_bytes().unwrap();
        let mut corrupt = bytes.clone();
        *corrupt.last_mut().unwrap() ^= 1;
        assert_eq!(
            LiveCheckpoint::decode(&corrupt),
            Err(CheckpointError::Checksum)
        );
        let decoded = LiveCheckpoint::decode(&bytes).unwrap();
        assert_eq!(decoded.canonical_bytes().unwrap(), bytes);
        let mut restored = PlasticSubstrate::from_live_checkpoint(decoded).unwrap();
        assert_eq!(restored, substrate);
        let restored_result = restored.propagate();
        let substrate_result = substrate.propagate();
        assert_physical_equivalence(&substrate, &substrate_result, &restored, &restored_result);
    }

    #[cfg(feature = "core1")]
    #[test]
    fn e18_in_flight_generation_round_trips_and_protects_only_until_resolution() {
        fn delayed(protected: bool) -> (PlasticSubstrate, CellId, ArrowId) {
            let (mut substrate, source, _target, arrow) = substrate(1);
            let slot = substrate.arrow_slot(arrow).unwrap();
            substrate
                .arrows
                .with_mut(slot.0, |candidate| candidate.delay = 20);
            substrate.set_core0_profile(Core0Profile::GenericExternal);
            substrate.set_in_flight_protection(protected);
            (substrate, source, arrow)
        }

        let (mut protected, source, arrow) = delayed(true);
        protected.enter(input(source, 0));
        let prefix = protected.propagate_with_observation_ceiling(1);
        assert!(!prefix.run.naturally_quiescent);
        assert_eq!(protected.in_flight_count(arrow), 1);

        let bytes = protected
            .live_checkpoint(18)
            .unwrap()
            .canonical_bytes()
            .unwrap();
        let checkpoint = LiveCheckpoint::decode(&bytes).unwrap();
        let mut restored = PlasticSubstrate::from_live_checkpoint(checkpoint).unwrap();
        assert_eq!(restored.in_flight_count(arrow), 1);
        let suffix = restored.propagate();
        assert!(suffix.naturally_quiescent);
        assert_eq!(restored.in_flight_count(arrow), 0);
        assert!(restored
            .arrow_slot(arrow)
            .is_some_and(|slot| restored.arrows.get(slot.0).live));

        let (mut idle_decay, idle_source, idle_arrow) = delayed(false);
        idle_decay.enter(input(idle_source, 0));
        idle_decay.propagate_with_observation_ceiling(1);
        let unprotected = idle_decay.propagate();
        assert!(unprotected.naturally_quiescent);
        assert!(!idle_decay
            .arrow_slot(idle_arrow)
            .is_some_and(|slot| idle_decay.arrows.get(slot.0).live));
    }

    #[test]
    fn reused_identity_rejects_stale_generation() {
        let (mut substrate, source, target, arrow) = substrate(1);
        let stale = substrate.arrow_reference(arrow);
        substrate.advance_time(10);
        assert_eq!(substrate.resolve_arrow(stale), None);
        let reused = substrate.add_arrow(ArrowSpec {
            from: source,
            to: target,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 4,
            mode: TransmissionMode::Drive,
        });
        assert_eq!(reused, arrow);
        let current = substrate.arrow_reference(reused);
        assert_ne!(current.generation, stale.generation);
        assert_eq!(substrate.resolve_arrow(stale), None);
        assert!(substrate.resolve_arrow(current).is_some());
    }

    #[test]
    fn durable_body_rejects_stale_internal_references() {
        let (substrate, _, _, _) = substrate(4);
        let mut body = substrate.arena_body(1);
        body.arrows[0].from.generation = Generation(99);
        let bytes = body.canonical_bytes().unwrap();
        assert!(matches!(
            PlasticSubstrate::from_body_bytes(&bytes),
            Err(CheckpointError::StaleCellReference(_))
        ));
    }

    #[test]
    fn input_capacity_rejects_batches_atomically() {
        let (substrate, source, _, _) = substrate(50);
        let mut runtime = BoundaryRuntime::new(substrate, 1, 2, 4).unwrap();
        runtime.enqueue(input(source, 0)).unwrap();
        let before = runtime.clone();
        assert_eq!(
            runtime.enqueue_batch(&[input(source, 1), input(source, 2)]),
            Err(BoundaryError::InputFull {
                capacity: 2,
                occupied: 1,
                attempted: 2,
            })
        );
        assert_eq!(runtime, before);
    }

    #[test]
    fn output_backpressure_is_bounded_transactional_and_fifo() {
        let (substrate, source, _, _) = substrate(50);
        let mut runtime = BoundaryRuntime::new(substrate, 1, 4, 1).unwrap();
        runtime.enqueue(input(source, 0)).unwrap();
        let first = runtime.run_until_quiescent().unwrap();
        assert_eq!(first.produced_outputs, 1);
        runtime.enqueue(input(source, 2)).unwrap();
        let before = runtime.clone();
        assert_eq!(
            runtime.run_until_quiescent(),
            Err(BoundaryError::OutputFull {
                capacity: 1,
                occupied: 1,
                required: 1,
            })
        );
        assert_eq!(runtime, before);
        let first_output = runtime.drain_output(1);
        assert_eq!(first_output.len(), 1);
        runtime.run_until_quiescent().unwrap();
        let second_output = runtime.drain_output(1);
        assert_eq!(second_output.len(), 1);
        assert!(first_output[0].tick < second_output[0].tick);
    }

    #[test]
    fn output_batch_larger_than_capacity_changes_nothing() {
        let mut substrate = PlasticSubstrate::with_capacity(ArenaId(55), 4, 4);
        let source = substrate.add_cell(CellSpec {
            physical_id: 1,
            position: 0,
            region: 0,
            threshold: 1,
            resistance: 10,
        });
        for physical_id in [2, 3] {
            let target = substrate.add_cell(CellSpec {
                physical_id,
                position: physical_id as i32,
                region: 1,
                threshold: 1,
                resistance: 10,
            });
            substrate.add_arrow(ArrowSpec {
                from: source,
                to: target,
                delay: 1,
                phase: 0,
                coupling: 1,
                resistance: 50,
                mode: TransmissionMode::Drive,
            });
        }
        let mut runtime = BoundaryRuntime::new(substrate, 1, 2, 1).unwrap();
        runtime.enqueue(input(source, 0)).unwrap();
        let before = runtime.clone();
        assert_eq!(
            runtime.run_until_quiescent(),
            Err(BoundaryError::OutputBatchTooLarge {
                capacity: 1,
                required: 2,
            })
        );
        assert_eq!(runtime, before);
    }

    #[test]
    fn buffered_path_and_live_checkpoint_preserve_exact_behavior() {
        let (mut direct, source, _, _) = substrate(50);
        let inputs = [input(source, 0), input(source, 2)];
        let expected = direct.arrive(&inputs, 1);

        let (buffered, buffered_source, _, _) = substrate(50);
        assert_eq!(source, buffered_source);
        let mut runtime = BoundaryRuntime::new(buffered, 1, 4, 4).unwrap();
        runtime.enqueue(inputs[0]).unwrap();
        runtime.run_until_quiescent().unwrap();
        runtime.enqueue(inputs[1]).unwrap();
        let checkpoint = runtime.live_checkpoint(6).unwrap();
        let bytes = checkpoint.canonical_bytes().unwrap();
        let decoded = BoundaryLiveCheckpoint::decode(&bytes).unwrap();
        assert_eq!(decoded.canonical_bytes().unwrap(), bytes);
        let mut restored = BoundaryRuntime::from_live_checkpoint(decoded).unwrap();
        assert_eq!(restored, runtime);

        let first = restored.drain_all_output();
        let second_run = restored.run_until_quiescent().unwrap();
        let mut actual = first;
        actual.extend(restored.drain_all_output());
        assert_eq!(actual, expected.crossings);
        assert!(second_run.naturally_quiescent);
        assert_eq!(restored.substrate(), &direct);
    }

    #[cfg(feature = "rs0")]
    #[test]
    fn rs0_observation_ceiling_pauses_and_resumes_without_changing_quiescent_runs() {
        let (bounded_body, source, _, _) = substrate(100_000);
        let mut bounded = bounded_body.clone();
        let mut ordinary = bounded_body;
        bounded.enter(input(source, 0));
        ordinary.enter(input(source, 0));
        let observed = bounded.propagate_with_observation_ceiling(16);
        let unbounded = ordinary.propagate();
        assert!(!observed.observation_ceiling_reached);
        assert_eq!(observed.scheduled_deliveries, 2);
        assert_physical_equivalence(&bounded, &observed.run, &ordinary, &unbounded);
        assert_eq!(bounded, ordinary);

        let mut recurrent = PlasticSubstrate::with_capacity(ArenaId(99), 4, 4);
        let a = recurrent.add_cell(CellSpec {
            physical_id: 1,
            position: 0,
            region: 0,
            threshold: 1,
            resistance: 100_000,
        });
        let b = recurrent.add_cell(CellSpec {
            physical_id: 2,
            position: 100,
            region: 0,
            threshold: 1,
            resistance: 100_000,
        });
        for (from, to) in [(a, b), (b, a)] {
            recurrent.add_arrow(ArrowSpec {
                from,
                to,
                delay: 1,
                phase: 0,
                coupling: 1,
                resistance: 100_000,
                mode: TransmissionMode::Drive,
            });
        }
        recurrent.enter(input(a, 0));
        let first = recurrent.propagate_with_observation_ceiling(16);
        assert_eq!(first.scheduled_deliveries, 16);
        assert!(first.observation_ceiling_reached);
        assert!(!first.run.naturally_quiescent);
        let second = recurrent.propagate_with_observation_ceiling(8);
        assert_eq!(second.scheduled_deliveries, 8);
        assert!(second.observation_ceiling_reached);
        assert!(!second.run.naturally_quiescent);
        assert!(recurrent
            .arena_body(1)
            .arrows
            .iter()
            .all(|arrow| arrow.live && arrow.resistance > 99_990));
    }

    #[cfg(feature = "sv0")]
    #[test]
    fn sv0_local_variation_proposes_equal_weak_signed_alternatives() {
        let mut substrate = PlasticSubstrate::with_capacity(ArenaId(100), 4, 8);
        let source = substrate.add_cell(CellSpec {
            physical_id: 1,
            position: 0,
            region: 0,
            threshold: 1,
            resistance: 100,
        });
        let target = substrate.add_cell(CellSpec {
            physical_id: 2,
            position: 1,
            region: 1,
            threshold: 100,
            resistance: 100,
        });
        substrate.enter(input(source, 0));
        let result = substrate.propagate();
        let mut alternatives = substrate
            .arena_body(1)
            .arrows
            .into_iter()
            .filter(|arrow| arrow.from.id == source && arrow.to.id == target)
            .collect::<Vec<_>>();
        alternatives.sort_by_key(|arrow| arrow.coupling);
        assert_eq!(result.work.local_structural_proposals, 2);
        assert_eq!(alternatives.len(), 2);
        assert_eq!(alternatives[0].coupling, -1);
        assert_eq!(alternatives[1].coupling, 1);
        for alternative in &alternatives {
            assert_eq!(alternative.delay, 1);
            assert_eq!(alternative.phase, 0);
            assert_eq!(alternative.resistance, 1);
            assert_eq!(alternative.transmission_mode, 0);
            assert!(alternative.live);
        }
    }
}
