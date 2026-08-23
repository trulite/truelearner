#![forbid(unsafe_code)]
//! Isolated CELL/ARROW/SPIKE physics for the CJ-B development arm.

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
pub struct ArrowSpec {
    pub from: CellId,
    pub to: CellId,
    pub delay: i64,
    pub phase: i32,
    pub coupling: i32,
    pub resistance: u32,
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkLedger {
    pub queue_comparisons: u64,
    pub spikes_delivered: u64,
    pub generation_checks: u64,
    pub state_updates: u64,
    pub threshold_checks: u64,
    pub firings: u64,
    pub arrow_checks: u64,
    pub local_state_inspections: u64,
    pub suppressed_transmissions: u64,
    pub local_state_consumptions: u64,
    pub consumed_impulse: u64,
    pub spikes_emitted: u64,
    pub local_eligibility_writes: u64,
    pub local_return_updates: u64,
    pub ordinary_pressure_updates: u64,
    pub local_structural_proposals: u64,
    pub physical_deallocations: u64,
}

impl WorkLedger {
    pub fn total(&self) -> u64 {
        self.queue_comparisons
            + self.spikes_delivered
            + self.generation_checks
            + self.state_updates
            + self.threshold_checks
            + self.firings
            + self.arrow_checks
            + self.local_state_inspections
            + self.suppressed_transmissions
            + self.local_state_consumptions
            + self.consumed_impulse
            + self.spikes_emitted
            + self.local_eligibility_writes
            + self.local_return_updates
            + self.ordinary_pressure_updates
            + self.local_structural_proposals
            + self.physical_deallocations
    }

    pub fn absorb(&mut self, other: &Self) {
        self.queue_comparisons += other.queue_comparisons;
        self.spikes_delivered += other.spikes_delivered;
        self.generation_checks += other.generation_checks;
        self.state_updates += other.state_updates;
        self.threshold_checks += other.threshold_checks;
        self.firings += other.firings;
        self.arrow_checks += other.arrow_checks;
        self.local_state_inspections += other.local_state_inspections;
        self.suppressed_transmissions += other.suppressed_transmissions;
        self.local_state_consumptions += other.local_state_consumptions;
        self.consumed_impulse += other.consumed_impulse;
        self.spikes_emitted += other.spikes_emitted;
        self.local_eligibility_writes += other.local_eligibility_writes;
        self.local_return_updates += other.local_return_updates;
        self.ordinary_pressure_updates += other.ordinary_pressure_updates;
        self.local_structural_proposals += other.local_structural_proposals;
        self.physical_deallocations += other.physical_deallocations;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceEntry {
    pub tick: i64,
    pub target_physical: u64,
    pub impulse: i32,
    pub fired: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transmission {
    pub tick: i64,
    pub from_physical: u64,
    pub to_physical: u64,
    pub destination_state: i32,
    pub coupling: i32,
    pub available: i32,
    pub emitted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Crossing {
    pub tick: i64,
    pub from_physical: u64,
    pub to_physical: u64,
    pub impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Execution {
    pub start_fingerprint: u64,
    pub end_fingerprint: u64,
    pub permanent_fingerprint: u64,
    pub trace: Vec<TraceEntry>,
    pub transmissions: Vec<Transmission>,
    pub crossings: Vec<Crossing>,
    pub work: WorkLedger,
    pub naturally_quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrowSummary {
    pub records: usize,
    pub live: usize,
    pub resistance_max: u32,
    pub coupling_max: i32,
    pub generation_max: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Substrate {
    cells: Vec<Cell>,
    arrows: Vec<Arrow>,
    pending: Vec<Spike>,
    tick: i64,
    next_serial: u64,
    pressure_tick: i64,
}

impl Substrate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_cell(&mut self, spec: CellSpec) -> CellId {
        assert!(spec.threshold > 0, "threshold must be physically positive");
        assert!(
            self.cells
                .iter()
                .all(|cell| cell.physical_id != spec.physical_id),
            "physical identity must be unique"
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
        self.arrows.push(Arrow {
            from: spec.from,
            to: spec.to,
            delay: spec.delay,
            phase: spec.phase,
            coupling: spec.coupling,
            source_generation: self.cells[spec.from.0].generation,
            generation: 1,
            resistance: spec.resistance,
            live: spec.resistance > 0,
            eligible_until: None,
        });
        id
    }

    pub fn enter(&mut self, input: SpikeInput) {
        self.require_cell(input.target);
        assert!(
            input.arrival_tick >= self.tick,
            "arrival precedes local time"
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

    pub fn advance_time(&mut self, tick: i64) -> WorkLedger {
        assert!(tick >= self.tick, "time cannot run backward");
        assert!(self.pending.is_empty(), "activity must propagate first");
        let mut work = WorkLedger::default();
        self.elapse_to(tick, &mut work);
        self.tick = tick;
        work
    }

    pub fn propagate(&mut self) -> Execution {
        let start_fingerprint = self.complete_fingerprint();
        let mut trace = Vec::new();
        let mut transmissions = Vec::new();
        let mut crossings = Vec::new();
        let mut work = WorkLedger::default();

        while !self.pending.is_empty() {
            let mut first = 0;
            for candidate in 1..self.pending.len() {
                work.queue_comparisons += 1;
                if self.spike_order(candidate, first) == Ordering::Less {
                    first = candidate;
                }
            }
            let spike = self.pending.remove(first);
            let external_arrival = spike.arrow.is_none();
            self.elapse_to(spike.arrival_tick, &mut work);
            self.tick = spike.arrival_tick;
            work.spikes_delivered += 1;
            work.generation_checks += 1;

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

            self.apply_local_return(spike.target, self.tick, &mut work);
            self.decay_cell(spike.target, self.tick);
            let target = &mut self.cells[spike.target.0];
            target.state = target.state.saturating_add(spike.impulse);
            work.state_updates += 1;
            work.threshold_checks += 1;
            let fires = self.tick >= target.refractory_until && target.state >= target.threshold;
            trace.push(TraceEntry {
                tick: self.tick,
                target_physical: target.physical_id,
                impulse: spike.impulse,
                fired: fires,
            });
            if !fires {
                continue;
            }

            target.state = 0;
            target.refractory_until = self.tick.saturating_add(1);
            work.firings += 1;
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
                work.arrow_checks += 1;
                if !arrow.live
                    || arrow.from != source
                    || arrow.source_generation != source_generation
                {
                    continue;
                }

                self.decay_cell(arrow.to, self.tick);
                work.local_state_inspections += 1;
                let destination_state = self.cells[arrow.to.0].state;
                let available = destination_state.saturating_add(arrow.coupling);
                let emitted = available >= self.cells[arrow.to.0].threshold;
                transmissions.push(Transmission {
                    tick: self.tick,
                    from_physical: self.cells[arrow.from.0].physical_id,
                    to_physical: self.cells[arrow.to.0].physical_id,
                    destination_state,
                    coupling: arrow.coupling,
                    available,
                    emitted,
                });
                if !emitted {
                    work.suppressed_transmissions += 1;
                    continue;
                }

                self.cells[arrow.to.0].state = 0;
                work.local_state_consumptions += 1;
                work.consumed_impulse += u64::try_from(destination_state.max(0)).unwrap_or(0);
                let from = &self.cells[arrow.from.0];
                let to = &self.cells[arrow.to.0];
                if from.region != to.region {
                    crossings.push(Crossing {
                        tick: self.tick,
                        from_physical: from.physical_id,
                        to_physical: to.physical_id,
                        impulse: available,
                    });
                }
                let live_arrow = &mut self.arrows[arrow_id.0];
                live_arrow.eligible_until = Some(self.tick.saturating_add(LOCAL_WINDOW));
                work.local_eligibility_writes += 1;
                self.pending.push(Spike {
                    arrival_tick: self.tick.saturating_add(arrow.delay),
                    phase: arrow.phase,
                    origin_physical,
                    target: arrow.to,
                    target_generation: to.generation,
                    impulse: available,
                    serial: self.next_serial,
                    arrow: Some((arrow_id, arrow.generation)),
                });
                self.next_serial = self.next_serial.wrapping_add(1);
                work.spikes_emitted += 1;
            }
        }

        Execution {
            start_fingerprint,
            end_fingerprint: self.complete_fingerprint(),
            permanent_fingerprint: self.permanent_fingerprint(),
            trace,
            transmissions,
            crossings,
            work,
            naturally_quiescent: self.pending.is_empty(),
        }
    }

    pub fn current_tick(&self) -> i64 {
        self.tick
    }

    pub fn cell_physical_id(&self, cell: CellId) -> u64 {
        self.require_cell(cell);
        self.cells[cell.0].physical_id
    }

    pub fn cell_state(&mut self, cell: CellId) -> i32 {
        self.require_cell(cell);
        self.decay_cell(cell, self.tick);
        self.cells[cell.0].state
    }

    pub fn complete_fingerprint(&self) -> u64 {
        fingerprint(&self.state_bytes(false))
    }

    pub fn permanent_fingerprint(&self) -> u64 {
        fingerprint(&self.state_bytes(true))
    }

    pub fn persistent_bytes(&self) -> usize {
        self.cells.len() * std::mem::size_of::<Cell>()
            + self.arrows.len() * std::mem::size_of::<Arrow>()
    }

    pub fn arrow_count(&self) -> usize {
        self.arrows.len()
    }

    pub fn arrows_between(&self, from: CellId, to: CellId) -> ArrowSummary {
        self.require_cell(from);
        self.require_cell(to);
        let matches = self
            .arrows
            .iter()
            .filter(|arrow| arrow.from == from && arrow.to == to)
            .collect::<Vec<_>>();
        ArrowSummary {
            records: matches.len(),
            live: matches.iter().filter(|arrow| arrow.live).count(),
            resistance_max: matches
                .iter()
                .filter(|arrow| arrow.live)
                .map(|arrow| arrow.resistance)
                .max()
                .unwrap_or(0),
            coupling_max: matches
                .iter()
                .filter(|arrow| arrow.live)
                .map(|arrow| arrow.coupling)
                .max()
                .unwrap_or(0),
            generation_max: matches
                .iter()
                .map(|arrow| arrow.generation)
                .max()
                .unwrap_or(0),
        }
    }

    fn apply_local_return(&mut self, cell: CellId, tick: i64, work: &mut WorkLedger) {
        for arrow in &mut self.arrows {
            if arrow.live
                && arrow.from == cell
                && arrow.eligible_until.is_some_and(|end| tick <= end)
            {
                let prior_resistance = arrow.resistance;
                arrow.resistance = arrow.resistance.saturating_add(LOCAL_RETURN_STRENGTH);
                if prior_resistance <= COUPLING_PLASTICITY_CEILING && arrow.coupling > 0 {
                    arrow.coupling = arrow.coupling.saturating_add(1).min(2);
                }
                arrow.eligible_until = None;
                work.local_return_updates += 1;
            }
        }
    }

    fn elapse_to(&mut self, tick: i64, work: &mut WorkLedger) {
        let pressure_steps = tick.saturating_sub(self.pressure_tick) / ORDINARY_PRESSURE_PERIOD;
        if pressure_steps > 0 {
            let amount = u32::try_from(pressure_steps).unwrap_or(u32::MAX);
            for arrow in &mut self.arrows {
                if arrow.live {
                    let was_live = arrow.live;
                    pressure_arrow(arrow, amount);
                    work.ordinary_pressure_updates += 1;
                    work.physical_deallocations += u64::from(was_live && !arrow.live);
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
                work.ordinary_pressure_updates += 1;
                work.physical_deallocations += u64::from(was_live && !arrow.live);
            }
        }
        for index in 0..self.cells.len() {
            self.decay_cell(CellId(index), tick);
        }
    }

    fn propose_local_arrows(&mut self, source: CellId, work: &mut WorkLedger) {
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
            });
            work.local_structural_proposals += 1;
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
        assert!(id.0 < self.cells.len(), "cell must belong to substrate");
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

    fn state_bytes(&self, permanent_only: bool) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut cells = self.cells.iter().collect::<Vec<_>>();
        cells.sort_by_key(|cell| cell.physical_id);
        for cell in cells {
            push_u64(&mut bytes, cell.physical_id);
            push_i32(&mut bytes, cell.position);
            bytes.extend_from_slice(&cell.region.to_le_bytes());
            push_i32(&mut bytes, cell.threshold);
            push_u32(&mut bytes, cell.generation);
            push_u32(&mut bytes, cell.resistance);
            bytes.push(u8::from(cell.live));
            if !permanent_only {
                push_i32(&mut bytes, cell.state);
                push_i64(&mut bytes, cell.last_update_tick);
                push_i64(&mut bytes, cell.refractory_until);
            }
        }
        let mut arrows = self.arrows.iter().collect::<Vec<_>>();
        arrows.sort_by_key(|arrow| {
            (
                self.cells[arrow.from.0].physical_id,
                self.cells[arrow.to.0].physical_id,
                arrow.delay,
                arrow.phase,
                arrow.coupling,
                arrow.generation,
            )
        });
        for arrow in arrows {
            push_u64(&mut bytes, self.cells[arrow.from.0].physical_id);
            push_u64(&mut bytes, self.cells[arrow.to.0].physical_id);
            push_i64(&mut bytes, arrow.delay);
            push_i32(&mut bytes, arrow.phase);
            push_i32(&mut bytes, arrow.coupling);
            push_u32(&mut bytes, arrow.source_generation);
            push_u32(&mut bytes, arrow.generation);
            push_u32(&mut bytes, arrow.resistance);
            bytes.push(u8::from(arrow.live));
            if !permanent_only {
                push_i64(&mut bytes, arrow.eligible_until.unwrap_or(i64::MIN));
            }
        }
        if !permanent_only {
            let mut spikes = self.pending.iter().collect::<Vec<_>>();
            spikes.sort_by_key(|spike| {
                (
                    spike.arrival_tick,
                    spike.phase,
                    spike.origin_physical,
                    self.cells[spike.target.0].physical_id,
                    spike.serial,
                )
            });
            for spike in spikes {
                push_i64(&mut bytes, spike.arrival_tick);
                push_i32(&mut bytes, spike.phase);
                push_u64(&mut bytes, spike.origin_physical);
                push_u64(&mut bytes, self.cells[spike.target.0].physical_id);
                push_u32(&mut bytes, spike.target_generation);
                push_i32(&mut bytes, spike.impulse);
                push_u64(&mut bytes, spike.serial);
            }
            push_i64(&mut bytes, self.tick);
            push_u64(&mut bytes, self.next_serial);
            push_i64(&mut bytes, self.pressure_tick);
        }
        bytes
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

fn fingerprint(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn push_i32(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_i64(bytes: &mut Vec<u8>, value: i64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(substrate: &mut Substrate, physical_id: u64, position: i32, threshold: i32) -> CellId {
        substrate.add_cell(CellSpec {
            physical_id,
            position,
            region: 0,
            threshold,
            resistance: 100,
        })
    }

    #[test]
    fn weak_transmission_consumes_current_local_state() {
        let mut substrate = Substrate::new();
        let source = cell(&mut substrate, 10, 0, 3);
        let destination = cell(&mut substrate, 20, 1, 3);
        substrate.add_arrow(ArrowSpec {
            from: source,
            to: destination,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: 1,
        });
        substrate.enter(SpikeInput {
            arrival_tick: 0,
            phase: -1,
            origin_physical: 1,
            target: destination,
            impulse: 2,
        });
        substrate.enter(SpikeInput {
            arrival_tick: 0,
            phase: 0,
            origin_physical: 2,
            target: source,
            impulse: 3,
        });
        let run = substrate.propagate();
        assert!(run.naturally_quiescent);
        assert_eq!(run.work.local_state_consumptions, 1);
        assert!(run
            .trace
            .iter()
            .any(|entry| { entry.target_physical == 20 && entry.impulse == 3 && entry.fired }));
    }

    #[test]
    fn mature_weak_transmission_still_suppresses_source_alone() {
        let mut substrate = Substrate::new();
        let source = cell(&mut substrate, 10, 0, 3);
        let destination = cell(&mut substrate, 20, 1, 3);
        substrate.add_arrow(ArrowSpec {
            from: source,
            to: destination,
            delay: 0,
            phase: 0,
            coupling: 2,
            resistance: 30,
        });
        substrate.enter(SpikeInput {
            arrival_tick: 0,
            phase: 0,
            origin_physical: 2,
            target: source,
            impulse: 3,
        });
        let run = substrate.propagate();
        assert_eq!(run.work.local_state_consumptions, 0);
        assert_eq!(run.work.suppressed_transmissions, 1);
        assert!(!run.trace.iter().any(|entry| entry.target_physical == 20));
    }
}
