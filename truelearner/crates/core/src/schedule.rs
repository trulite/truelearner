use crate::prelude::*;
#[cfg(not(feature = "compact-body"))]
use crate::timing_wheel::TimingWheel;
#[cfg(feature = "compact-body")]
use truelearner_body::{MomentKey, QueueWork, Timeline, TimelineItem};

pub(crate) const LOCAL_DECAY_PERIOD: i64 = 10;

pub(crate) fn pressure_epoch(tick: i64) -> i64 {
    tick.div_euclid(LOCAL_DECAY_PERIOD)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CausalLineage {
    origins: Vec<u64>,
    birth_ticks: Vec<i64>,
    transition_ticks: Vec<Option<i64>>,
}

impl CausalLineage {
    pub(crate) fn singleton(origin_physical: u64, birth_tick: i64) -> Self {
        Self {
            origins: vec![origin_physical],
            birth_ticks: vec![birth_tick],
            transition_ticks: vec![None],
        }
    }

    pub(crate) fn transitioned(origin_physical: u64, birth_tick: i64) -> Self {
        Self {
            origins: vec![origin_physical],
            birth_ticks: vec![birth_tick],
            transition_ticks: vec![Some(birth_tick)],
        }
    }

    pub(crate) fn from_firings(firings: &[Firing]) -> Self {
        let mut members = BTreeMap::new();
        for firing in firings {
            if let Some(lineage) = &firing.causal_lineage {
                for ((&origin, &birth_tick), &transition_tick) in lineage
                    .origins
                    .iter()
                    .zip(lineage.birth_ticks.iter())
                    .zip(lineage.transition_ticks.iter())
                {
                    members
                        .entry(origin)
                        .and_modify(|known: &mut (i64, Option<i64>)| {
                            known.0 = known.0.max(birth_tick);
                            known.1 = known.1.max(transition_tick);
                        })
                        .or_insert((birth_tick, transition_tick));
                }
            } else {
                members
                    .entry(firing.origin_physical)
                    .and_modify(|known: &mut (i64, Option<i64>)| {
                        known.0 = known.0.max(firing.arrival_tick);
                    })
                    .or_insert((firing.arrival_tick, None));
            }
        }
        debug_assert!(!members.is_empty());
        let (origins, timing): (Vec<_>, Vec<_>) = members.into_iter().unzip();
        let (birth_ticks, transition_ticks) = timing.into_iter().unzip();
        Self {
            origins,
            birth_ticks,
            transition_ticks,
        }
    }

    pub(crate) fn selected(&self, origins: &BTreeSet<u64>) -> Option<Self> {
        let members = self
            .origins
            .iter()
            .copied()
            .zip(self.birth_ticks.iter().copied())
            .zip(self.transition_ticks.iter().copied())
            .filter(|((origin, _), _)| origins.contains(origin))
            .collect::<Vec<_>>();
        if members.is_empty() {
            return None;
        }
        let (identity_and_birth, transition_ticks): (Vec<_>, Vec<_>) = members.into_iter().unzip();
        let (origins, birth_ticks) = identity_and_birth.into_iter().unzip();
        Some(Self {
            origins,
            birth_ticks,
            transition_ticks,
        })
    }

    pub(crate) fn origins(&self) -> &[u64] {
        &self.origins
    }

    pub(crate) fn birth_tick(&self, origin_physical: u64) -> Option<i64> {
        self.origins
            .binary_search(&origin_physical)
            .ok()
            .map(|index| self.birth_ticks[index])
    }

    pub(crate) fn transition_tick(&self, origin_physical: u64) -> Option<i64> {
        self.origins
            .binary_search(&origin_physical)
            .ok()
            .and_then(|index| self.transition_ticks[index])
    }

    pub(crate) fn contains_transition(&self) -> bool {
        self.transition_ticks.iter().any(Option::is_some)
    }

    pub(crate) fn memory_bytes(&self) -> usize {
        self.origins
            .capacity()
            .saturating_mul(std::mem::size_of::<u64>())
            .saturating_add(
                self.birth_ticks
                    .capacity()
                    .saturating_mul(std::mem::size_of::<i64>()),
            )
            .saturating_add(
                self.transition_ticks
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Option<i64>>()),
            )
    }
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
    pub(crate) causal_lineage: Option<CausalLineage>,
    pub(crate) physical_incidence: PhysicalIncidence,
    pub(crate) target_physical: u64,
    pub(crate) target: JunctionId,
    pub(crate) target_generation: Generation,
    pub(crate) impulse: i32,
    pub(crate) strength: i64,
    pub(crate) serial: u64,
    pub(crate) link: Option<(LinkId, Generation)>,
}

#[cfg(feature = "compact-body")]
impl TimelineItem for Firing {
    fn serial(&self) -> u64 {
        self.serial
    }

    fn retained_bytes(&self) -> usize {
        self.causal_lineage
            .as_ref()
            .map_or(0, CausalLineage::memory_bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg(not(feature = "compact-body"))]
pub(super) struct Schedule(TimingWheel);

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg(feature = "compact-body")]
pub(super) struct Schedule(Timeline<Firing>);

impl Schedule {
    pub(super) fn new(head_tick: i64) -> Self {
        #[cfg(not(feature = "compact-body"))]
        {
            const RING_WIDTH: usize = 64;
            return Self(TimingWheel::new(head_tick, RING_WIDTH));
        }
        #[cfg(feature = "compact-body")]
        {
            Self(Timeline::new(head_tick))
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(super) fn push(&mut self, firing: Firing, cost: &mut ExecutionCost) {
        cost.queue_ops = cost.queue_ops.saturating_add(1);
        cost.touch::<Firing>(1);
        #[cfg(not(feature = "compact-body"))]
        self.0.push(firing, cost);
        #[cfg(feature = "compact-body")]
        {
            let moment = MomentKey::new(firing.arrival_tick, firing.phase, firing.causal_wave);
            let mut work = QueueWork::default();
            self.0.push(moment, firing, &mut work);
            account_queue_work(cost, work);
        }
    }

    pub(super) fn next_wave(&mut self, cost: &mut ExecutionCost) -> Option<Vec<Firing>> {
        #[cfg(not(feature = "compact-body"))]
        {
            return self.0.pop_next_wave(cost);
        }
        #[cfg(feature = "compact-body")]
        {
            let mut work = QueueWork::default();
            let wave = self.0.pop(&mut work).map(|(_, wave)| wave);
            account_queue_work(cost, work);
            wave
        }
    }

    pub(super) fn canonical(&self) -> Vec<Firing> {
        #[cfg(not(feature = "compact-body"))]
        let mut firings: Vec<_> = self.0.firings().cloned().collect();
        #[cfg(feature = "compact-body")]
        let mut firings: Vec<_> = self.0.values().cloned().collect();
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
        #[cfg(not(feature = "compact-body"))]
        let firings = self.0.firings();
        #[cfg(feature = "compact-body")]
        let firings = self.0.values();
        self.0.memory_bytes().saturating_add(
            firings
                .filter_map(|firing| firing.causal_lineage.as_ref())
                .map(CausalLineage::memory_bytes)
                .sum::<usize>(),
        )
    }
}

#[cfg(feature = "compact-body")]
fn account_queue_work(cost: &mut ExecutionCost, work: QueueWork) {
    cost.allocations = cost.allocations.saturating_add(work.allocations);
    cost.scans = cost.scans.saturating_add(work.scans);
    cost.comparisons = cost.comparisons.saturating_add(work.comparisons);
    cost.timing_wheel_bucket_selection_comparisons = cost
        .timing_wheel_bucket_selection_comparisons
        .saturating_add(work.comparisons);
    let touched = usize::try_from(work.touched_items).unwrap_or(usize::MAX);
    cost.touch::<Firing>(touched);
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

#[cfg(all(test, feature = "compact-body"))]
mod compact_backend_tests {
    use super::*;

    #[test]
    fn harness_frontiers_are_owned_by_truelearner_body() {
        fn require_body_timeline(_: &Timeline<Firing>) {}

        let schedule = Schedule::new(0);
        require_body_timeline(&schedule.0);
    }
}
