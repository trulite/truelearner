#![forbid(unsafe_code)]
//! Single-file physical runtime: state, local transitions, and crossings.

use std::cmp::Ordering;

const LOCAL_WINDOW: i64 = 4;
const LOCAL_RETURN_STRENGTH: u32 = 3;
const UNSUPPORTED_USE_PRESSURE: u32 = 1;
const ORDINARY_PRESSURE_PERIOD: i64 = 10;
const LOCAL_VARIATION_RADIUS: i32 = 2;
const COUPLING_PLASTICITY_CEILING: u32 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArrowId(usize);

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
    physical_id: u64,
    position: i32,
    region: i16,
    threshold: i32,
    state: i32,
    last_update_tick: i64,
    refractory_until: i64,
    generation: u32,
    resistance: u32,
    live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Arrow {
    from: CellId,
    to: CellId,
    delay: i64,
    phase: i32,
    coupling: i32,
    source_generation: u32,
    generation: u32,
    resistance: u32,
    live: bool,
    eligible_until: Option<i64>,
    mode: TransmissionMode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Spike {
    arrival_tick: i64,
    phase: i32,
    origin_physical: u64,
    target: CellId,
    target_generation: u32,
    impulse: i32,
    serial: u64,
    arrow: Option<(ArrowId, u32)>,
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    total: u64,
    pub drive_deliveries: u64,
    pub modulatory_deliveries: u64,
    pub local_return_updates: u64,
    pub local_structural_proposals: u64,
    pub physical_deallocations: u64,
}

impl Work {
    pub fn total(&self) -> u64 {
        self.total
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunResult {
    pub crossings: Vec<Crossing>,
    pub work: Work,
    pub naturally_quiescent: bool,
    pub resident_bytes: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlasticSubstrate {
    cells: Vec<Cell>,
    arrows: Vec<Arrow>,
    pending: Vec<Spike>,
    tick: i64,
    next_serial: u64,
    pressure_tick: i64,
}

impl PlasticSubstrate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_cell(&mut self, spec: CellSpec) -> CellId {
        assert!(spec.threshold > 0, "threshold must be physically positive");
        assert!(
            self.cells
                .iter()
                .all(|cell| cell.physical_id != spec.physical_id),
            "physical cell identity must be unique"
        );
        let id = CellId(self.cells.len());
        self.cells.push(Cell {
            physical_id: spec.physical_id,
            position: spec.position,
            region: spec.region,
            threshold: spec.threshold,
            state: 0,
            last_update_tick: self.tick,
            refractory_until: self.tick,
            generation: 1,
            resistance: spec.resistance,
            live: spec.resistance > 0,
        });
        id
    }

    pub fn add_arrow(&mut self, spec: ArrowSpec) -> ArrowId {
        self.require_cell(spec.from);
        self.require_cell(spec.to);
        assert!(spec.delay >= 0, "delay must not run backward in time");
        let id = ArrowId(self.arrows.len());
        let source_generation = self.cells[spec.from.0].generation;
        self.arrows.push(Arrow {
            from: spec.from,
            to: spec.to,
            delay: spec.delay,
            phase: spec.phase,
            coupling: spec.coupling,
            source_generation,
            generation: 1,
            resistance: spec.resistance,
            live: spec.resistance > 0,
            eligible_until: None,
            mode: spec.mode,
        });
        id
    }

    pub fn enter(&mut self, input: SpikeInput) {
        self.require_cell(input.target);
        assert!(
            input.arrival_tick >= self.tick,
            "physical arrivals cannot precede current substrate time"
        );
        self.pending.push(Spike {
            arrival_tick: input.arrival_tick,
            phase: input.phase,
            origin_physical: input.origin_physical,
            target: input.target,
            target_generation: self.cells[input.target.0].generation,
            impulse: input.impulse,
            serial: self.next_serial,
            arrow: None,
        });
        self.next_serial = self.next_serial.wrapping_add(1);
    }

    pub fn advance_time(&mut self, tick: i64) -> Work {
        assert!(tick >= self.tick, "physical time cannot run backward");
        assert!(
            self.pending.is_empty(),
            "queued activity must propagate first"
        );
        let mut work = Work::default();
        self.elapse_to(tick, &mut work);
        self.tick = tick;
        work
    }

    pub fn propagate(&mut self) -> RunResult {
        let mut crossings = Vec::new();
        let mut work = Work::default();
        while !self.pending.is_empty() {
            let mut first = 0;
            for candidate in 1..self.pending.len() {
                work.total = work.total.saturating_add(1);
                if self.spike_order(candidate, first) == Ordering::Less {
                    first = candidate;
                }
            }
            let spike = self.pending.remove(first);
            let external_arrival = spike.arrow.is_none();
            self.elapse_to(spike.arrival_tick, &mut work);
            self.tick = spike.arrival_tick;
            work.total = work.total.saturating_add(2);

            if let Some((arrow_id, generation)) = spike.arrow {
                let arrow = &self.arrows[arrow_id.0];
                if !arrow.live || arrow.generation != generation {
                    continue;
                }
            }
            let target = &self.cells[spike.target.0];
            if !target.live || target.generation != spike.target_generation {
                continue;
            }

            let mode = spike.arrow.map_or(TransmissionMode::Drive, |(arrow, _)| {
                self.arrows[arrow.0].mode
            });
            if mode == TransmissionMode::Modulatory {
                work.total = work.total.saturating_add(1);
                work.modulatory_deliveries = work.modulatory_deliveries.saturating_add(1);
                self.apply_modulatory_return(spike.target, self.tick, &mut work);
                continue;
            }
            work.total = work.total.saturating_add(3);
            work.drive_deliveries = work.drive_deliveries.saturating_add(1);
            self.decay_cell(spike.target, self.tick);
            let target = &mut self.cells[spike.target.0];
            target.state = target.state.saturating_add(spike.impulse);
            let fires = self.tick >= target.refractory_until && target.state >= target.threshold;
            if !fires {
                continue;
            }

            target.state = 0;
            target.refractory_until = self.tick.saturating_add(1);
            work.total = work.total.saturating_add(1);
            let source = spike.target;
            let origin_physical = target.physical_id;
            let source_generation = target.generation;
            if external_arrival {
                self.propose_local_arrows(source, &mut work);
            }
            let outgoing = self
                .arrows
                .iter()
                .enumerate()
                .map(|(index, arrow)| (ArrowId(index), arrow.clone()))
                .collect::<Vec<_>>();
            for (arrow_id, arrow) in outgoing {
                work.total = work.total.saturating_add(1);
                if !arrow.live
                    || arrow.from != source
                    || arrow.source_generation != source_generation
                {
                    continue;
                }
                let from = &self.cells[arrow.from.0];
                let to = &self.cells[arrow.to.0];
                if from.region != to.region {
                    crossings.push(Crossing {
                        tick: self.tick,
                        from_physical: from.physical_id,
                        to_physical: to.physical_id,
                        from_region: from.region,
                        to_region: to.region,
                        impulse: arrow.coupling,
                    });
                }
                let live_arrow = &mut self.arrows[arrow_id.0];
                live_arrow.eligible_until = Some(self.tick.saturating_add(LOCAL_WINDOW));
                work.total = work.total.saturating_add(2);
                self.pending.push(Spike {
                    arrival_tick: self.tick.saturating_add(arrow.delay),
                    phase: arrow.phase,
                    origin_physical,
                    target: arrow.to,
                    target_generation: to.generation,
                    impulse: arrow.coupling,
                    serial: self.next_serial,
                    arrow: Some((arrow_id, arrow.generation)),
                });
                self.next_serial = self.next_serial.wrapping_add(1);
            }
        }
        RunResult {
            crossings,
            work,
            naturally_quiescent: self.pending.is_empty(),
            resident_bytes: self.resident_bytes(),
        }
    }

    fn apply_modulatory_return(&mut self, cell: CellId, tick: i64, work: &mut Work) {
        for arrow in &mut self.arrows {
            if arrow.live
                && arrow.from == cell
                && arrow.eligible_until.is_some_and(|end| tick <= end)
            {
                work.total = work.total.saturating_add(3);
                work.local_return_updates = work.local_return_updates.saturating_add(1);
                let prior_resistance = arrow.resistance;
                arrow.resistance = arrow.resistance.saturating_add(LOCAL_RETURN_STRENGTH);
                if prior_resistance <= COUPLING_PLASTICITY_CEILING && arrow.coupling > 0 {
                    arrow.coupling = arrow.coupling.saturating_add(1).min(2);
                }
                arrow.eligible_until = None;
            }
        }
    }

    fn elapse_to(&mut self, tick: i64, work: &mut Work) {
        let pressure_steps = tick.saturating_sub(self.pressure_tick) / ORDINARY_PRESSURE_PERIOD;
        if pressure_steps > 0 {
            let amount = u32::try_from(pressure_steps).unwrap_or(u32::MAX);
            for arrow in &mut self.arrows {
                if arrow.live {
                    let was_live = arrow.live;
                    pressure_arrow(arrow, amount);
                    work.total = work.total.saturating_add(1);
                    if was_live && !arrow.live {
                        work.total = work.total.saturating_add(1);
                        work.physical_deallocations = work.physical_deallocations.saturating_add(1);
                    }
                }
            }
            self.pressure_tick = self
                .pressure_tick
                .saturating_add(pressure_steps.saturating_mul(ORDINARY_PRESSURE_PERIOD));
        }
        for arrow in &mut self.arrows {
            if arrow.live && arrow.eligible_until.is_some_and(|end| end < tick) {
                let was_live = arrow.live;
                pressure_arrow(arrow, UNSUPPORTED_USE_PRESSURE);
                arrow.eligible_until = None;
                work.total = work.total.saturating_add(1);
                if was_live && !arrow.live {
                    work.total = work.total.saturating_add(1);
                    work.physical_deallocations = work.physical_deallocations.saturating_add(1);
                }
            }
        }
        for index in 0..self.cells.len() {
            self.decay_cell(CellId(index), tick);
        }
    }

    fn propose_local_arrows(&mut self, source: CellId, work: &mut Work) {
        let source_position = self.cells[source.0].position;
        let mut targets = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(index, cell)| {
                let distance = cell.position.saturating_sub(source_position).abs();
                (CellId(index) != source
                    && cell.live
                    && (1..=LOCAL_VARIATION_RADIUS).contains(&distance)
                    && !self.arrows.iter().any(|arrow| {
                        arrow.live && arrow.from == source && arrow.to == CellId(index)
                    }))
                .then_some((cell.physical_id, CellId(index), distance))
            })
            .collect::<Vec<_>>();
        targets.sort_by_key(|(physical_id, _, _)| *physical_id);
        for (_, target, distance) in targets {
            let generation = u32::try_from(self.arrows.len())
                .unwrap_or(u32::MAX)
                .saturating_add(2);
            self.arrows.push(Arrow {
                from: source,
                to: target,
                delay: i64::from(distance.max(1)),
                phase: 0,
                coupling: 1,
                source_generation: self.cells[source.0].generation,
                generation,
                resistance: 1,
                live: true,
                eligible_until: None,
                mode: TransmissionMode::Drive,
            });
            work.total = work.total.saturating_add(1);
            work.local_structural_proposals = work.local_structural_proposals.saturating_add(1);
        }
    }

    fn decay_cell(&mut self, cell: CellId, tick: i64) {
        let target = &mut self.cells[cell.0];
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
    }

    fn require_cell(&self, id: CellId) {
        assert!(
            id.0 < self.cells.len(),
            "cell must belong to this substrate"
        );
    }

    fn spike_order(&self, left: usize, right: usize) -> Ordering {
        let left = &self.pending[left];
        let right = &self.pending[right];
        (
            left.arrival_tick,
            left.phase,
            left.origin_physical,
            self.cells[left.target.0].physical_id,
            left.serial,
        )
            .cmp(&(
                right.arrival_tick,
                right.phase,
                right.origin_physical,
                self.cells[right.target.0].physical_id,
                right.serial,
            ))
    }

    fn resident_bytes(&self) -> usize {
        self.cells.len() * std::mem::size_of::<Cell>()
            + self.arrows.len() * std::mem::size_of::<Arrow>()
    }
}

fn pressure_arrow(arrow: &mut Arrow, amount: u32) {
    arrow.resistance = arrow.resistance.saturating_sub(amount);
    if arrow.resistance == 0 && arrow.live {
        arrow.live = false;
        arrow.generation = arrow.generation.wrapping_add(1);
        arrow.eligible_until = None;
    }
}
