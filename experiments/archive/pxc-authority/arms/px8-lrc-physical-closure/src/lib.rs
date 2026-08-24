#![forbid(unsafe_code)]

#[allow(dead_code)]
#[path = "../../../crates/lr1-modulatory-physical-return/src/lib.rs"]
mod physics;

use physics::{
    ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, TransmissionMode,
    WorkLedger,
};

const INNER: i16 = 40;
const OUTER: i16 = 400;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub namespace: u64,
    pub reverse: bool,
    pub reflect: bool,
    pub twist: u64,
    pub outward_resistance: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Reading {
    pub outward: usize,
    pub inward_modulation: usize,
    pub updates: u64,
    pub work: u64,
    pub quiet: bool,
    pub permanent: u64,
    pub complete: u64,
}

#[derive(Clone)]
pub struct RecursiveBody {
    substrate: PlasticSubstrate,
    namespace: u64,
    twist: u64,
    primitive_sources: [CellId; 4],
    stage_sources: [CellId; 3],
    context: CellId,
    returning: CellId,
    outward_from: u64,
    outward_to: u64,
    inward_from: u64,
    inward_to: u64,
}

impl RecursiveBody {
    pub fn new(layout: Layout) -> Self {
        let mut substrate = PlasticSubstrate::new();
        let primitive_order = if layout.reverse {
            [3, 2, 1, 0]
        } else {
            [0, 1, 2, 3]
        };
        let stage_order = if layout.reverse { [2, 1, 0] } else { [0, 1, 2] };

        let mut primitive_sources = [None; 4];
        let mut primitive_outlets = [None; 4];
        let mut primitive_traces = [None; 4];
        let mut primitive_hubs = [None; 4];
        for side in primitive_order {
            primitive_sources[side] = Some(substrate.add_cell(cell(
                physical(layout, 10 + side as u64),
                -100_000 - side as i32 * 1_000,
                10 + side as i16,
                1,
            )));
            primitive_outlets[side] = Some(substrate.add_cell(cell(
                physical(layout, 20 + side as u64),
                -90_000 - side as i32 * 1_000,
                20 + side as i16,
                1,
            )));
            primitive_traces[side] = Some(substrate.add_cell(cell(
                physical(layout, 30 + side as u64),
                -80_000 - side as i32 * 1_000,
                30 + side as i16,
                2,
            )));
            primitive_hubs[side] = Some(substrate.add_cell(cell(
                physical(layout, 40 + side as u64),
                -70_000 - side as i32 * 1_000,
                34 + side as i16,
                1,
            )));
        }
        let primitive_sources = primitive_sources.map(Option::unwrap);
        let primitive_outlets = primitive_outlets.map(Option::unwrap);
        let primitive_traces = primitive_traces.map(Option::unwrap);
        let primitive_hubs = primitive_hubs.map(Option::unwrap);

        let mut opportunities = [None; 3];
        let mut stage_sources = [None; 3];
        let mut outputs = [None; 3];
        let mut source_traces = [None; 3];
        let mut output_traces = [None; 3];
        let mut output_hubs = [None; 3];
        let mut return_loci = [None; 3];
        for stage in stage_order {
            opportunities[stage] = Some(substrate.add_cell(cell(
                physical(layout, 100 + stage as u64),
                -10_000 - stage as i32 * 1_000,
                50 + stage as i16,
                2,
            )));
            let source_position = 10_000 + stage as i32 * 1_000;
            stage_sources[stage] = Some(substrate.add_cell(cell(
                physical(layout, 200 + stage as u64),
                source_position,
                60 + stage as i16,
                2,
            )));
            outputs[stage] = Some(substrate.add_cell(cell(
                physical(layout, 300 + stage as u64),
                source_position + if layout.reflect { -1 } else { 1 },
                70 + stage as i16,
                2,
            )));
            source_traces[stage] = Some(substrate.add_cell(cell(
                physical(layout, 400 + stage as u64),
                30_000 + stage as i32 * 1_000,
                80 + stage as i16,
                2,
            )));
            output_traces[stage] = Some(substrate.add_cell(cell(
                physical(layout, 600 + stage as u64),
                50_000 + stage as i32 * 1_000,
                100 + stage as i16,
                2,
            )));
            output_hubs[stage] = Some(substrate.add_cell(cell(
                physical(layout, 700 + stage as u64),
                60_000 + stage as i32 * 1_000,
                110 + stage as i16,
                1,
            )));
            return_loci[stage] = Some(substrate.add_cell(cell(
                physical(layout, 800 + stage as u64),
                70_000 + stage as i32 * 1_000,
                120 + stage as i16,
                1,
            )));
        }
        let opportunities = opportunities.map(Option::unwrap);
        let stage_sources = stage_sources.map(Option::unwrap);
        let outputs = outputs.map(Option::unwrap);
        let source_traces = source_traces.map(Option::unwrap);
        let output_traces = output_traces.map(Option::unwrap);
        let output_hubs = output_hubs.map(Option::unwrap);
        let return_loci = return_loci.map(Option::unwrap);
        let context = substrate.add_cell(cell(physical(layout, 900), 90_000, 130, 1));
        let returning = substrate.add_cell(cell(physical(layout, 901), 100_000, 131, 1));
        let outward = substrate.add_cell(cell(physical(layout, 950), 120_000, OUTER, 1));
        let relay = substrate.add_cell(cell(physical(layout, 951), 121_000, OUTER, 1));

        for side in primitive_order {
            substrate.add_arrow(drive(
                primitive_sources[side],
                primitive_outlets[side],
                0,
                1,
                100,
            ));
            normalize(
                &mut substrate,
                primitive_outlets[side],
                primitive_traces[side],
                primitive_hubs[side],
            );
        }
        for stage in stage_order {
            normalize(
                &mut substrate,
                outputs[stage],
                output_traces[stage],
                output_hubs[stage],
            );
        }

        let left = [primitive_traces[0], output_traces[0], output_traces[1]];
        let right = [
            primitive_traces[1],
            primitive_traces[2],
            primitive_traces[3],
        ];
        for stage in stage_order {
            substrate.add_arrow(drive(left[stage], opportunities[stage], 0, 1, 100));
            substrate.add_arrow(drive(right[stage], opportunities[stage], 0, 1, 100));
            substrate.add_arrow(drive(opportunities[stage], stage_sources[stage], 0, 1, 100));
            substrate.add_arrow(drive(context, outputs[stage], 1, 1, 100));
            substrate.add_arrow(drive(output_traces[stage], source_traces[stage], 0, 1, 100));
            substrate.add_arrow(drive(returning, source_traces[stage], 0, 1, 100));
            substrate.add_arrow(drive(source_traces[stage], return_loci[stage], 0, 1, 100));
            substrate.add_arrow(modulatory(
                return_loci[stage],
                stage_sources[stage],
                1,
                1,
                100,
            ));
        }

        substrate.add_arrow(drive(
            output_traces[2],
            outward,
            0,
            1,
            layout.outward_resistance,
        ));
        substrate.add_arrow(drive(outward, relay, 1, 1, 100));
        substrate.add_arrow(modulatory(relay, stage_sources[2], 1, 1, 100));

        Self {
            substrate,
            namespace: layout.namespace,
            twist: layout.twist,
            primitive_sources,
            stage_sources,
            context,
            returning,
            outward_from: physical(layout, 602),
            outward_to: physical(layout, 950),
            inward_from: physical(layout, 951),
            inward_to: physical(layout, 202),
        }
    }

    pub fn learn_twice(&mut self) -> Reading {
        let mut total = Reading::default();
        for (depth, starts) in [(1, [0, 11]), (2, [20, 31]), (3, [40, 51])] {
            for start in starts {
                total = merge(total, self.burst(depth, start, true, true));
            }
        }
        let pressure = self.substrate.advance_time(60);
        total.work = total.work.saturating_add(pressure.total());
        total.quiet &= true;
        total.permanent = self.substrate.permanent_fingerprint();
        total.complete = self.substrate.complete_fingerprint();
        total
    }

    pub fn learn_once_then_age(&mut self) -> Reading {
        let mut total = Reading::default();
        for (depth, start) in [(1, 0), (2, 11), (3, 22)] {
            total = merge(total, self.burst(depth, start, true, true));
        }
        let pressure = self.substrate.advance_time(110);
        total.work = total.work.saturating_add(pressure.total());
        total.permanent = self.substrate.permanent_fingerprint();
        total.complete = self.substrate.complete_fingerprint();
        total
    }

    pub fn reuse(&mut self, present: [bool; 4], start: i64, duplicate: bool) -> Reading {
        for (side, admitted) in present.into_iter().enumerate() {
            if admitted {
                let tick = start + primitive_offset(side);
                self.pulse(self.primitive_sources[side], tick, 1, side as i32);
                if duplicate {
                    self.pulse(self.primitive_sources[side], tick, 1, 10 + side as i32);
                }
            }
        }
        for stage in 0..3 {
            self.pulse(
                self.stage_sources[stage],
                start + 1 + stage as i64 * 2,
                1,
                100 + stage as i32,
            );
        }
        self.settle()
    }

    pub fn fingerprints(&self) -> (u64, u64) {
        (
            self.substrate.complete_fingerprint(),
            self.substrate.permanent_fingerprint(),
        )
    }

    pub fn persistent_bytes(&self) -> usize {
        self.substrate.persistent_bytes()
    }

    fn burst(&mut self, depth: usize, start: i64, context: bool, returning: bool) -> Reading {
        for side in 0..=depth {
            self.pulse(
                self.primitive_sources[side],
                start + primitive_offset(side),
                1,
                side as i32,
            );
        }
        for stage in 0..depth {
            let tick = start + 1 + stage as i64 * 2;
            self.pulse(self.stage_sources[stage], tick, 1, 100 + stage as i32);
            if context {
                self.pulse(self.context, tick, 1, 500 + stage as i32);
            }
            if returning {
                self.pulse(self.returning, tick + 2, 1, 600 + stage as i32);
            }
        }
        self.settle()
    }

    fn pulse(&mut self, target: CellId, tick: i64, impulse: i32, phase: i32) {
        self.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase,
            origin_physical: self.namespace + 50_000 + self.twist + phase as u64,
            target,
            impulse,
        });
    }

    fn settle(&mut self) -> Reading {
        reading(
            self.substrate.propagate(),
            self.outward_from,
            self.outward_to,
            self.inward_from,
            self.inward_to,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompactForm {
    Direct,
    Open,
    Aged,
    Fork,
    Ring,
}

#[derive(Clone)]
pub struct CompactBody {
    substrate: PlasticSubstrate,
    namespace: u64,
    inlet: CellId,
    outward_from: u64,
    outward_to: u64,
}

impl CompactBody {
    pub fn new(namespace: u64, form: CompactForm, reflect: bool) -> Self {
        let mut substrate = PlasticSubstrate::new();
        let direction = if reflect { -1 } else { 1 };
        let inlet = substrate.add_cell(cell(namespace + 1, 0, INNER, 1));
        let outward = substrate.add_cell(cell(namespace + 2, direction * 10, OUTER, 1));
        match form {
            CompactForm::Direct => {
                substrate.add_arrow(drive(inlet, outward, 1, 1, 100));
            }
            CompactForm::Open => {
                substrate.add_arrow(drive(inlet, outward, 1, 1, 0));
            }
            CompactForm::Aged => {
                substrate.add_arrow(drive(inlet, outward, 1, 1, 1));
                substrate.advance_time(10);
            }
            CompactForm::Fork => {
                let a = substrate.add_cell(cell(namespace + 3, direction * 2, INNER, 1));
                let b = substrate.add_cell(cell(namespace + 4, direction * 3, INNER, 1));
                let join = substrate.add_cell(cell(namespace + 5, direction * 4, INNER, 3));
                substrate.add_arrow(drive(inlet, a, 1, 1, 100));
                substrate.add_arrow(drive(inlet, b, 1, 1, 100));
                substrate.add_arrow(drive(a, join, 1, 1, 100));
                substrate.add_arrow(drive(b, join, 1, 1, 100));
                substrate.add_arrow(drive(join, outward, 1, 1, 100));
            }
            CompactForm::Ring => {
                let a = substrate.add_cell(cell(namespace + 3, direction * 2, INNER, 2));
                let b = substrate.add_cell(cell(namespace + 4, direction * 3, INNER, 2));
                substrate.add_arrow(drive(inlet, a, 1, 1, 100));
                substrate.add_arrow(drive(a, b, 1, 1, 100));
                substrate.add_arrow(drive(b, a, 1, 1, 100));
                substrate.add_arrow(drive(b, outward, 1, 1, 100));
            }
        }
        Self {
            substrate,
            namespace,
            inlet,
            outward_from: namespace + 1,
            outward_to: namespace + 2,
        }
    }

    pub fn flow(&mut self, count: usize, tick: i64) -> Reading {
        for index in 0..count {
            self.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase: index as i32,
                origin_physical: self.namespace + 100 + index as u64,
                target: self.inlet,
                impulse: 1,
            });
        }
        reading(
            self.substrate.propagate(),
            self.outward_from,
            self.outward_to,
            0,
            0,
        )
    }
}

fn normalize(substrate: &mut PlasticSubstrate, outlet: CellId, trace: CellId, hub: CellId) {
    substrate.add_arrow(drive(outlet, trace, 1, 1, 100));
    substrate.add_arrow(drive(outlet, hub, 1, 1, 100));
    substrate.add_arrow(drive(hub, trace, 0, 1, 100));
}

fn reading(
    execution: Execution,
    outward_from: u64,
    outward_to: u64,
    inward_from: u64,
    inward_to: u64,
) -> Reading {
    Reading {
        outward: execution
            .crossings
            .iter()
            .filter(|item| item.from_physical == outward_from && item.to_physical == outward_to)
            .count(),
        inward_modulation: execution
            .crossings
            .iter()
            .filter(|item| item.from_physical == inward_from && item.to_physical == inward_to)
            .count(),
        updates: execution.work.local_return_updates,
        work: execution.work.total(),
        quiet: execution.naturally_quiescent,
        permanent: execution.permanent_fingerprint,
        complete: execution.end_fingerprint,
    }
}

fn merge(left: Reading, right: Reading) -> Reading {
    Reading {
        outward: left.outward.saturating_add(right.outward),
        inward_modulation: left
            .inward_modulation
            .saturating_add(right.inward_modulation),
        updates: left.updates.saturating_add(right.updates),
        work: left.work.saturating_add(right.work),
        quiet: (left.quiet || left.work == 0) && right.quiet,
        permanent: right.permanent,
        complete: right.complete,
    }
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 100,
    }
}

fn drive(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode: TransmissionMode::Drive,
    }
}

fn modulatory(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode: TransmissionMode::Modulatory,
    }
}

fn physical(layout: Layout, suffix: u64) -> u64 {
    layout.namespace + (suffix.wrapping_mul(73).wrapping_add(layout.twist) % 1_000)
}

fn primitive_offset(side: usize) -> i64 {
    [0, 0, 2, 4][side]
}

#[allow(dead_code)]
fn add_work(total: &mut WorkLedger, next: &WorkLedger) {
    total.queue_comparisons += next.queue_comparisons;
    total.spikes_delivered += next.spikes_delivered;
    total.generation_checks += next.generation_checks;
    total.state_updates += next.state_updates;
    total.threshold_checks += next.threshold_checks;
    total.firings += next.firings;
    total.arrow_checks += next.arrow_checks;
    total.spikes_emitted += next.spikes_emitted;
    total.local_eligibility_writes += next.local_eligibility_writes;
    total.local_return_updates += next.local_return_updates;
    total.qualified_return_checks += next.qualified_return_checks;
    total.qualified_return_accepts += next.qualified_return_accepts;
    total.qualified_return_path_edges += next.qualified_return_path_edges;
    total.drive_deliveries += next.drive_deliveries;
    total.modulatory_deliveries += next.modulatory_deliveries;
    total.ordinary_pressure_updates += next.ordinary_pressure_updates;
    total.local_structural_proposals += next.local_structural_proposals;
    total.physical_deallocations += next.physical_deallocations;
}
