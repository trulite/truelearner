//! Semantic-free CELL/ARROW/SPIKE execution physics.
//!
//! This module intentionally knows nothing about requests, answers, actions,
//! correctness, event classes, or lifetime classes. It represents only local
//! state, directional coupling, transient occurrences, deterministic temporal
//! ordering, threshold firing, physical boundaries, and structural strength.

use std::cmp::Ordering;

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
    pub state: i32,
    pub generation: u32,
    pub resistance: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArrowSpec {
    pub from: CellId,
    pub to: CellId,
    pub delay: i32,
    pub transient_delay: i32,
    pub phase: i32,
    pub coupling: i32,
    pub generation: u32,
    pub resistance: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpikeInput {
    pub arrival_tick: i32,
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
    generation: u32,
    resistance: u32,
    live: bool,
    fired: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Arrow {
    from: CellId,
    to: CellId,
    delay: i32,
    transient_delay: i32,
    phase: i32,
    coupling: i32,
    generation: u32,
    resistance: u32,
    live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Spike {
    arrival_tick: i32,
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
    pub spikes_emitted: u64,
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
            + self.spikes_emitted
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceEntry {
    pub tick: i32,
    pub target_physical: u64,
    pub impulse: i32,
    pub fired: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Crossing {
    pub tick: i32,
    pub from_physical: u64,
    pub to_physical: u64,
    pub from_region: i16,
    pub to_region: i16,
    pub impulse: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Execution {
    pub start_fingerprint: u64,
    pub permanent_fingerprint: u64,
    pub trace_fingerprint: u64,
    pub end_fingerprint: u64,
    pub trace: Vec<TraceEntry>,
    pub fired: Vec<u64>,
    pub crossings: Vec<Crossing>,
    pub work: WorkLedger,
    pub naturally_quiescent: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Substrate {
    cells: Vec<Cell>,
    arrows: Vec<Arrow>,
    pending: Vec<Spike>,
    tick: i32,
    next_serial: u64,
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
            "physical cell identity must be unique"
        );
        let id = CellId(self.cells.len());
        self.cells.push(Cell {
            physical_id: spec.physical_id,
            position: spec.position,
            region: spec.region,
            threshold: spec.threshold,
            state: spec.state,
            generation: spec.generation,
            resistance: spec.resistance,
            live: spec.resistance > 0,
            fired: false,
        });
        id
    }

    pub fn add_arrow(&mut self, spec: ArrowSpec) -> ArrowId {
        self.require_cell(spec.from);
        self.require_cell(spec.to);
        let id = ArrowId(self.arrows.len());
        self.arrows.push(Arrow {
            from: spec.from,
            to: spec.to,
            delay: spec.delay,
            transient_delay: spec.transient_delay,
            phase: spec.phase,
            coupling: spec.coupling,
            generation: spec.generation,
            resistance: spec.resistance,
            live: spec.resistance > 0,
        });
        id
    }

    pub fn enter(&mut self, input: SpikeInput) {
        self.require_cell(input.target);
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

    pub fn set_arrow_coupling(&mut self, arrow: ArrowId, coupling: i32) {
        self.require_arrow(arrow);
        self.arrows[arrow.0].coupling = coupling;
    }

    pub fn set_arrow_transient_delay(&mut self, arrow: ArrowId, delay: i32) {
        self.require_arrow(arrow);
        self.arrows[arrow.0].transient_delay = delay;
    }

    pub fn strengthen_arrow(&mut self, arrow: ArrowId, amount: u32) {
        self.require_arrow(arrow);
        let structure = &mut self.arrows[arrow.0];
        structure.resistance = structure.resistance.saturating_add(amount);
        structure.live = structure.resistance > 0;
    }

    pub fn pressure_arrow(&mut self, arrow: ArrowId, amount: u32) {
        self.require_arrow(arrow);
        let structure = &mut self.arrows[arrow.0];
        structure.resistance = structure.resistance.saturating_sub(amount);
        if structure.resistance == 0 && structure.live {
            structure.live = false;
            structure.generation = structure.generation.wrapping_add(1);
        }
    }

    pub fn strengthen_cell(&mut self, cell: CellId, amount: u32) {
        self.require_cell(cell);
        let structure = &mut self.cells[cell.0];
        structure.resistance = structure.resistance.saturating_add(amount);
        structure.live = structure.resistance > 0;
    }

    pub fn pressure_cell(&mut self, cell: CellId, amount: u32) {
        self.require_cell(cell);
        let structure = &mut self.cells[cell.0];
        structure.resistance = structure.resistance.saturating_sub(amount);
        if structure.resistance == 0 && structure.live {
            structure.live = false;
            structure.generation = structure.generation.wrapping_add(1);
        }
    }

    pub fn begin_episode(&mut self) {
        self.pending.clear();
        self.tick = 0;
        self.next_serial = 0;
        for cell in &mut self.cells {
            cell.state = 0;
            cell.fired = false;
        }
        for arrow in &mut self.arrows {
            arrow.transient_delay = 0;
        }
    }

    pub fn propagate(&mut self) -> Execution {
        let start_fingerprint = self.complete_fingerprint();
        let permanent_fingerprint = self.permanent_fingerprint();
        let mut trace = Vec::new();
        let mut fired = Vec::new();
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

            let target = &mut self.cells[spike.target.0];
            target.state = target.state.saturating_add(spike.impulse);
            work.state_updates += 1;
            work.threshold_checks += 1;
            let fires = !target.fired && target.state >= target.threshold;
            trace.push(TraceEntry {
                tick: self.tick,
                target_physical: target.physical_id,
                impulse: spike.impulse,
                fired: fires,
            });
            if !fires {
                continue;
            }

            target.fired = true;
            fired.push(target.physical_id);
            work.firings += 1;
            let source = spike.target;
            let origin_physical = target.physical_id;
            let source_generation = target.generation;
            let outgoing = self
                .arrows
                .iter()
                .enumerate()
                .map(|(index, arrow)| (ArrowId(index), arrow.clone()))
                .collect::<Vec<_>>();
            for (arrow_id, arrow) in outgoing {
                work.arrow_checks += 1;
                if !arrow.live || arrow.from != source || arrow.generation != source_generation {
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
                self.pending.push(Spike {
                    arrival_tick: self.tick + arrow.delay + arrow.transient_delay,
                    phase: arrow.phase,
                    origin_physical,
                    target: arrow.to,
                    target_generation: to.generation,
                    impulse: arrow.coupling,
                    serial: self.next_serial,
                    arrow: Some((arrow_id, arrow.generation)),
                });
                self.next_serial = self.next_serial.wrapping_add(1);
                work.spikes_emitted += 1;
            }
        }

        let trace_fingerprint = trace_fingerprint(&trace);
        Execution {
            start_fingerprint,
            permanent_fingerprint,
            trace_fingerprint,
            end_fingerprint: self.complete_fingerprint(),
            trace,
            fired,
            crossings,
            work,
            naturally_quiescent: self.pending.is_empty(),
        }
    }

    pub fn complete_fingerprint(&self) -> u64 {
        fingerprint(&self.state_bytes(false))
    }

    pub fn permanent_fingerprint(&self) -> u64 {
        fingerprint(&self.state_bytes(true))
    }

    pub fn cell_physical_id(&self, cell: CellId) -> u64 {
        self.require_cell(cell);
        self.cells[cell.0].physical_id
    }

    pub fn arrow_is_live(&self, arrow: ArrowId) -> bool {
        self.require_arrow(arrow);
        self.arrows[arrow.0].live
    }

    fn require_cell(&self, id: CellId) {
        assert!(
            id.0 < self.cells.len(),
            "cell must belong to this substrate"
        );
    }

    fn require_arrow(&self, id: ArrowId) {
        assert!(
            id.0 < self.arrows.len(),
            "arrow must belong to this substrate"
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
                bytes.push(u8::from(cell.fired));
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
            )
        });
        for arrow in arrows {
            push_u64(&mut bytes, self.cells[arrow.from.0].physical_id);
            push_u64(&mut bytes, self.cells[arrow.to.0].physical_id);
            push_i32(&mut bytes, arrow.delay);
            push_i32(&mut bytes, arrow.phase);
            push_i32(&mut bytes, arrow.coupling);
            push_u32(&mut bytes, arrow.generation);
            push_u32(&mut bytes, arrow.resistance);
            bytes.push(u8::from(arrow.live));
            if !permanent_only {
                push_i32(&mut bytes, arrow.transient_delay);
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
                push_i32(&mut bytes, spike.arrival_tick);
                push_i32(&mut bytes, spike.phase);
                push_u64(&mut bytes, spike.origin_physical);
                push_u64(&mut bytes, self.cells[spike.target.0].physical_id);
                push_u32(&mut bytes, spike.target_generation);
                push_i32(&mut bytes, spike.impulse);
                push_u64(&mut bytes, spike.serial);
            }
            push_i32(&mut bytes, self.tick);
            push_u64(&mut bytes, self.next_serial);
        }
        bytes
    }
}

fn trace_fingerprint(trace: &[TraceEntry]) -> u64 {
    let mut bytes = Vec::new();
    for entry in trace {
        push_i32(&mut bytes, entry.tick);
        push_u64(&mut bytes, entry.target_physical);
        push_i32(&mut bytes, entry.impulse);
        bytes.push(u8::from(entry.fired));
    }
    fingerprint(&bytes)
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

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(id: u64, region: i16, threshold: i32) -> CellSpec {
        CellSpec {
            physical_id: id,
            position: id as i32,
            region,
            threshold,
            state: 0,
            generation: 1,
            resistance: 8,
        }
    }

    #[test]
    fn complete_replay_is_exact_and_allocation_independent() {
        let build = |reverse: bool| {
            let mut substrate = Substrate::new();
            let (source, target) = if reverse {
                let target = substrate.add_cell(cell(20, 0, 1));
                let source = substrate.add_cell(cell(10, 0, 1));
                (source, target)
            } else {
                let source = substrate.add_cell(cell(10, 0, 1));
                let target = substrate.add_cell(cell(20, 0, 1));
                (source, target)
            };
            substrate.add_arrow(ArrowSpec {
                from: source,
                to: target,
                delay: 1,
                transient_delay: 0,
                phase: 0,
                coupling: 1,
                generation: 1,
                resistance: 8,
            });
            substrate.enter(SpikeInput {
                arrival_tick: 0,
                phase: 0,
                origin_physical: 1,
                target: source,
                impulse: 1,
            });
            substrate.propagate()
        };
        let first = build(false);
        assert_eq!(first, build(false));
        let permuted = build(true);
        assert_eq!(first.permanent_fingerprint, permuted.permanent_fingerprint);
        assert_eq!(first.trace, permuted.trace);
    }

    #[test]
    fn pressure_physically_removes_and_blocks_stale_structure() {
        let mut substrate = Substrate::new();
        let source = substrate.add_cell(cell(10, 0, 1));
        let target = substrate.add_cell(cell(20, 0, 1));
        let arrow = substrate.add_arrow(ArrowSpec {
            from: source,
            to: target,
            delay: 1,
            transient_delay: 0,
            phase: 0,
            coupling: 1,
            generation: 1,
            resistance: 2,
        });
        substrate.pressure_arrow(arrow, 2);
        assert!(!substrate.arrow_is_live(arrow));
        substrate.enter(SpikeInput {
            arrival_tick: 0,
            phase: 0,
            origin_physical: 1,
            target: source,
            impulse: 1,
        });
        let run = substrate.propagate();
        assert_eq!(run.fired, vec![10]);
        assert!(run.naturally_quiescent);
    }

    #[test]
    fn physical_region_change_is_observed_as_crossing() {
        let mut substrate = Substrate::new();
        let inside = substrate.add_cell(cell(10, 0, 1));
        let outside = substrate.add_cell(cell(20, 1, 1));
        substrate.add_arrow(ArrowSpec {
            from: inside,
            to: outside,
            delay: 0,
            transient_delay: 0,
            phase: 0,
            coupling: 1,
            generation: 1,
            resistance: 1,
        });
        substrate.enter(SpikeInput {
            arrival_tick: 0,
            phase: 0,
            origin_physical: 1,
            target: inside,
            impulse: 1,
        });
        let run = substrate.propagate();
        assert_eq!(run.crossings.len(), 1);
        assert_eq!(
            (run.crossings[0].from_region, run.crossings[0].to_region),
            (0, 1)
        );
    }
}
