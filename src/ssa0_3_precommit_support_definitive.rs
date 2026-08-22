//! Write-once SSA0.3 definitive pre-closure support authority.
//!
//! The organism path retains the byte-exact frozen SSA0 CELL/ARROW/SPIKE
//! propagation loop. Fresh fixtures and all observations remain external.

use std::cmp::Ordering;
use std::collections::BTreeSet;

pub const PROTOCOL: &str = "ssa0-3-precommit-support-definitive-v1";
pub const FROZEN_PARENT: &str = "34277893201c1a72765b143de4b3da1912b6e3b6";
pub const AUTHORITATIVE_M6: &str = "aa4e22efd8a65b7694956a53cfaa970582695215";
pub const DEVELOPMENT_CLASSIFICATION_A: &str = "eeb14186a000a7eefba17e6f9e288e7335c44043";
pub const PROTOCOL_COMMIT: &str = "1ccbc3c588821269eda4fb8552f728a3638808b6";
pub const PARENT_PROTOCOL_SHA256: &str =
    "92ff3f758977a575e2f8ca651f7a45756e15241a6d4bf829012a266bae9489fc";
pub const PARENT_IMPLEMENTATION_SHA256: &str =
    "180b24f6b682ec5d274e44b0c680062d10b1f68b6fddeb4d857ec599b32f6299";
pub const PARENT_RUNNER_SHA256: &str =
    "fb693aa098e45617deefd5ae9b9de1003528d4b7fbfa87078545fbda5e90fa7f";
pub const DEFINITIVE_PROTOCOL_SHA256: &str =
    "2185e4d10dca9919184c12df14f95a7100ea963ba0abdc7b0162cd834558d220";

// SSA0_DEFINITIVE_RUNTIME_BEGIN

const FIRING_THRESHOLD: i16 = 4;
const INHIBITORY_IMPULSE: i16 = -64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CellId(u16);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cell {
    physical_id: u32,
    x: i16,
    threshold: i16,
    activation: i16,
    generation: u16,
    live: bool,
    fired: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Arrow {
    from: CellId,
    to: CellId,
    base_delay: i16,
    transient_delay: i16,
    phase: i16,
    impulse: i16,
    generation: u16,
    live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Spike {
    arrival_tick: i16,
    phase: i16,
    origin_physical: u32,
    target: CellId,
    target_generation: u16,
    impulse: i16,
    serial: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhysicalState {
    cells: Vec<Cell>,
    arrows: Vec<Arrow>,
    pending: Vec<Spike>,
    tick: i16,
    next_serial: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkLedger {
    pub queue_comparisons: u64,
    pub spikes_delivered: u64,
    pub generation_checks: u64,
    pub activation_updates: u64,
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
            + self.activation_updates
            + self.threshold_checks
            + self.firings
            + self.arrow_checks
            + self.spikes_emitted
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceEntry {
    pub tick: i16,
    pub target_physical: u32,
    pub impulse: i16,
    pub fired: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhysicsRun {
    start_bytes: Vec<u8>,
    start_fingerprint: u64,
    permanent_fingerprint: u64,
    trace: Vec<TraceEntry>,
    trace_fingerprint: u64,
    fired: Vec<u32>,
    end_fingerprint: u64,
    work: WorkLedger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Delivery {
    tick: i16,
    phase: i16,
    impulse: i16,
}

impl Delivery {
    const fn new(tick: i16, phase: i16, impulse: i16) -> Self {
        Self {
            tick,
            phase,
            impulse,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum AllocationOrder {
    RotateLeft(usize),
    RotateRight(usize),
    Reverse,
}

#[derive(Clone, Copy, Debug)]
struct TransferAllocation {
    name: &'static str,
    cell_order: AllocationOrder,
    arrow_order: AllocationOrder,
    layout_padding: usize,
    layout_origin: i16,
}

const ALLOCATIONS: [TransferAllocation; 4] = [
    TransferAllocation {
        name: "spiral_29",
        cell_order: AllocationOrder::RotateLeft(3),
        arrow_order: AllocationOrder::RotateLeft(5),
        layout_padding: 29,
        layout_origin: -6000,
    },
    TransferAllocation {
        name: "reverse_31",
        cell_order: AllocationOrder::Reverse,
        arrow_order: AllocationOrder::Reverse,
        layout_padding: 31,
        layout_origin: 6000,
    },
    TransferAllocation {
        name: "woven_37",
        cell_order: AllocationOrder::RotateLeft(7),
        arrow_order: AllocationOrder::RotateRight(11),
        layout_padding: 37,
        layout_origin: -12000,
    },
    TransferAllocation {
        name: "mirror_43",
        cell_order: AllocationOrder::Reverse,
        arrow_order: AllocationOrder::RotateLeft(13),
        layout_padding: 43,
        layout_origin: 12000,
    },
];

#[derive(Clone, Debug)]
struct Configuration {
    deliveries: [Vec<Delivery>; 2],
    blocked: [bool; 2],
    stale: [bool; 2],
    allocation: TransferAllocation,
    world_ordinal: u16,
}

#[derive(Clone, Debug)]
struct Fixture {
    state: PhysicalState,
    contenders: [u32; 2],
    effects: [u32; 2],
    permanent_fingerprint: u64,
}

fn push_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn fingerprint(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn state_bytes(state: &PhysicalState, permanent_only: bool) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut cells = state.cells.iter().collect::<Vec<_>>();
    cells.sort_by_key(|cell| cell.physical_id);
    for cell in cells {
        push_u32(&mut bytes, cell.physical_id);
        push_i16(&mut bytes, cell.x);
        push_i16(&mut bytes, cell.threshold);
        push_u16(&mut bytes, cell.generation);
        bytes.push(u8::from(cell.live));
        if !permanent_only {
            push_i16(&mut bytes, cell.activation);
            bytes.push(u8::from(cell.fired));
        }
    }
    let physical = |id: CellId| state.cells[usize::from(id.0)].physical_id;
    let mut arrows = state.arrows.iter().collect::<Vec<_>>();
    arrows.sort_by_key(|arrow| {
        (
            physical(arrow.from),
            physical(arrow.to),
            arrow.base_delay,
            arrow.phase,
            arrow.impulse,
        )
    });
    for arrow in arrows {
        push_u32(&mut bytes, physical(arrow.from));
        push_u32(&mut bytes, physical(arrow.to));
        push_i16(&mut bytes, arrow.base_delay);
        push_i16(&mut bytes, arrow.phase);
        push_i16(&mut bytes, arrow.impulse);
        push_u16(&mut bytes, arrow.generation);
        bytes.push(u8::from(arrow.live));
        if !permanent_only {
            push_i16(&mut bytes, arrow.transient_delay);
        }
    }
    if !permanent_only {
        let mut spikes = state.pending.iter().collect::<Vec<_>>();
        spikes.sort_by_key(|spike| {
            (
                spike.arrival_tick,
                spike.phase,
                spike.origin_physical,
                physical(spike.target),
                spike.serial,
            )
        });
        for spike in spikes {
            push_i16(&mut bytes, spike.arrival_tick);
            push_i16(&mut bytes, spike.phase);
            push_u32(&mut bytes, spike.origin_physical);
            push_u32(&mut bytes, physical(spike.target));
            push_u16(&mut bytes, spike.target_generation);
            push_i16(&mut bytes, spike.impulse);
            push_u32(&mut bytes, spike.serial);
        }
        push_i16(&mut bytes, state.tick);
        push_u32(&mut bytes, state.next_serial);
    }
    bytes
}

fn trace_fingerprint(trace: &[TraceEntry]) -> u64 {
    let mut bytes = Vec::new();
    for entry in trace {
        push_i16(&mut bytes, entry.tick);
        push_u32(&mut bytes, entry.target_physical);
        push_i16(&mut bytes, entry.impulse);
        bytes.push(u8::from(entry.fired));
    }
    fingerprint(&bytes)
}

fn physical_id(config: &Configuration, base: u32) -> u32 {
    0x9000_0000 + u32::from(config.world_ordinal) * 0x0001_0000 + base
}

fn apply_order<T>(items: &mut [T], order: AllocationOrder) {
    if items.is_empty() {
        return;
    }
    match order {
        AllocationOrder::RotateLeft(amount) => items.rotate_left(amount % items.len()),
        AllocationOrder::RotateRight(amount) => items.rotate_right(amount % items.len()),
        AllocationOrder::Reverse => items.reverse(),
    }
}

fn build_fixture(config: &Configuration) -> Fixture {
    #[derive(Clone)]
    struct CellSpec {
        physical_id: u32,
        x: i16,
        threshold: i16,
        activation: i16,
        generation: u16,
        live: bool,
    }
    #[derive(Clone)]
    struct ArrowSpec {
        from: u32,
        to: u32,
        base_delay: i16,
        transient_delay: i16,
        phase: i16,
        impulse: i16,
        generation: u16,
        live: bool,
    }

    let layout_shift = config.allocation.layout_origin + config.world_ordinal as i16;
    let source = physical_id(config, 10);
    let contenders = [physical_id(config, 20), physical_id(config, 30)];
    let effects = [physical_id(config, 40), physical_id(config, 50)];
    let mut cells = vec![CellSpec {
        physical_id: source,
        x: layout_shift,
        threshold: 1,
        activation: 0,
        generation: 1,
        live: true,
    }];
    for (route, contender) in contenders.iter().enumerate() {
        cells.push(CellSpec {
            physical_id: *contender,
            x: layout_shift + if route == 0 { -47 } else { 47 },
            threshold: FIRING_THRESHOLD,
            activation: 0,
            generation: 1,
            live: true,
        });
    }
    for (route, effect) in effects.iter().enumerate() {
        cells.push(CellSpec {
            physical_id: *effect,
            x: layout_shift + if route == 0 { -83 } else { 83 },
            threshold: 1,
            activation: 0,
            generation: 1,
            live: true,
        });
    }

    let mut arrows = Vec::new();
    for (route, contender) in contenders.iter().enumerate() {
        for (ordinal, delivery) in config.deliveries[route].iter().enumerate() {
            let relay = physical_id(config, 100 + route as u32 * 100 + ordinal as u32);
            cells.push(CellSpec {
                physical_id: relay,
                x: layout_shift
                    + if route == 0 {
                        -101 - ordinal as i16
                    } else {
                        101 + ordinal as i16
                    },
                threshold: 1,
                activation: 0,
                generation: 1,
                live: true,
            });
            let generation = if config.stale[route] { 2 } else { 1 };
            let live = !config.blocked[route];
            arrows.push(ArrowSpec {
                from: source,
                to: relay,
                base_delay: delivery.tick,
                transient_delay: 0,
                phase: -1701,
                impulse: 1,
                generation,
                live,
            });
            arrows.push(ArrowSpec {
                from: relay,
                to: *contender,
                base_delay: 0,
                transient_delay: 0,
                phase: delivery.phase,
                impulse: delivery.impulse,
                generation,
                live,
            });
        }
    }
    for (route, contender) in contenders.iter().enumerate() {
        arrows.push(ArrowSpec {
            from: *contender,
            to: effects[route],
            base_delay: 0,
            transient_delay: 0,
            phase: 0,
            impulse: 1,
            generation: 1,
            live: true,
        });
        arrows.push(ArrowSpec {
            from: *contender,
            to: contenders[1 - route],
            base_delay: 0,
            transient_delay: 0,
            phase: -100,
            impulse: INHIBITORY_IMPULSE,
            generation: 1,
            live: true,
        });
    }
    for ordinal in 0..config.allocation.layout_padding {
        cells.push(CellSpec {
            physical_id: physical_id(config, 10_000 + ordinal as u32),
            x: layout_shift + 401 + ordinal as i16,
            threshold: 32,
            activation: 0,
            generation: 1,
            live: true,
        });
    }
    apply_order(&mut cells, config.allocation.cell_order);
    apply_order(&mut arrows, config.allocation.arrow_order);
    let materialized_cells = cells
        .into_iter()
        .map(|spec| Cell {
            physical_id: spec.physical_id,
            x: spec.x,
            threshold: spec.threshold,
            activation: spec.activation,
            generation: spec.generation,
            live: spec.live,
            fired: false,
        })
        .collect::<Vec<_>>();
    let id_for = |physical: u32| {
        CellId(
            materialized_cells
                .iter()
                .position(|cell| cell.physical_id == physical)
                .expect("fixture arrow endpoint exists") as u16,
        )
    };
    let materialized_arrows = arrows
        .into_iter()
        .map(|spec| Arrow {
            from: id_for(spec.from),
            to: id_for(spec.to),
            base_delay: spec.base_delay,
            transient_delay: spec.transient_delay,
            phase: spec.phase,
            impulse: spec.impulse,
            generation: spec.generation,
            live: spec.live,
        })
        .collect::<Vec<_>>();
    let source_id = id_for(source);
    let state = PhysicalState {
        cells: materialized_cells,
        arrows: materialized_arrows,
        pending: vec![Spike {
            arrival_tick: 0,
            phase: -1901,
            origin_physical: 0xe300_0000 + u32::from(config.world_ordinal),
            target: source_id,
            target_generation: 1,
            impulse: 1,
            serial: 0,
        }],
        tick: 0,
        next_serial: 1,
    };
    let permanent_fingerprint = fingerprint(&state_bytes(&state, true));
    Fixture {
        state,
        contenders,
        effects,
        permanent_fingerprint,
    }
}

// SSA0_PHYSICS_BEGIN
fn spike_order(left: &Spike, right: &Spike, state: &PhysicalState) -> Ordering {
    (
        left.arrival_tick,
        left.phase,
        left.origin_physical,
        state.cells[usize::from(left.target.0)].physical_id,
        left.serial,
    )
        .cmp(&(
            right.arrival_tick,
            right.phase,
            right.origin_physical,
            state.cells[usize::from(right.target.0)].physical_id,
            right.serial,
        ))
}

fn propagate(mut state: PhysicalState, permanent_fingerprint: u64) -> PhysicsRun {
    let start_bytes = state_bytes(&state, false);
    let start_fingerprint = fingerprint(&start_bytes);
    let mut trace = Vec::new();
    let mut fired = Vec::new();
    let mut work = WorkLedger::default();
    while !state.pending.is_empty() {
        let mut first = 0;
        for candidate in 1..state.pending.len() {
            work.queue_comparisons += 1;
            if spike_order(&state.pending[candidate], &state.pending[first], &state)
                == Ordering::Less
            {
                first = candidate;
            }
        }
        let spike = state.pending.remove(first);
        state.tick = spike.arrival_tick;
        work.spikes_delivered += 1;
        work.generation_checks += 1;
        let target_index = usize::from(spike.target.0);
        if !state.cells[target_index].live
            || state.cells[target_index].generation != spike.target_generation
        {
            continue;
        }
        state.cells[target_index].activation = state.cells[target_index]
            .activation
            .saturating_add(spike.impulse);
        work.activation_updates += 1;
        work.threshold_checks += 1;
        let fires = !state.cells[target_index].fired
            && state.cells[target_index].activation >= state.cells[target_index].threshold;
        trace.push(TraceEntry {
            tick: state.tick,
            target_physical: state.cells[target_index].physical_id,
            impulse: spike.impulse,
            fired: fires,
        });
        if !fires {
            continue;
        }
        state.cells[target_index].fired = true;
        fired.push(state.cells[target_index].physical_id);
        work.firings += 1;
        let origin_physical = state.cells[target_index].physical_id;
        for arrow in &state.arrows {
            work.arrow_checks += 1;
            if !arrow.live
                || arrow.from != spike.target
                || arrow.generation != state.cells[target_index].generation
            {
                continue;
            }
            let target_generation = state.cells[usize::from(arrow.to.0)].generation;
            state.pending.push(Spike {
                arrival_tick: state.tick + arrow.base_delay + arrow.transient_delay,
                phase: arrow.phase,
                origin_physical,
                target: arrow.to,
                target_generation,
                impulse: arrow.impulse,
                serial: state.next_serial,
            });
            state.next_serial += 1;
            work.spikes_emitted += 1;
        }
    }
    let trace_fingerprint = trace_fingerprint(&trace);
    let end_fingerprint = fingerprint(&state_bytes(&state, false));
    PhysicsRun {
        start_bytes,
        start_fingerprint,
        permanent_fingerprint,
        trace,
        trace_fingerprint,
        fired,
        end_fingerprint,
        work,
    }
}
// SSA0_PHYSICS_END

#[derive(Clone, Copy, Debug)]
struct Schedule {
    name: &'static str,
    target_ticks: [i16; 4],
    competitor_ticks: [i16; 4],
    phase: i16,
}

impl Schedule {
    const fn closure(self) -> i16 {
        self.competitor_ticks[3]
    }

    const fn slow_tick(self) -> i16 {
        self.target_ticks[3]
    }
}

const SCHEDULES: [Schedule; 3] = [
    Schedule {
        name: "quartz_29",
        target_ticks: [17, 21, 25, 37],
        competitor_ticks: [17, 21, 25, 29],
        phase: 101,
    },
    Schedule {
        name: "rill_53",
        target_ticks: [41, 45, 49, 61],
        competitor_ticks: [41, 45, 49, 53],
        phase: 307,
    },
    Schedule {
        name: "spire_89",
        target_ticks: [73, 79, 85, 101],
        competitor_ticks: [73, 79, 85, 89],
        phase: 709,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AddedSide {
    None,
    PreClosure,
    Late,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PathChange {
    None,
    TargetBlocked,
    TargetStale,
    TargetAbsent,
    CompetitorBlocked,
    CompetitorStale,
    CompetitorAbsent,
}

#[derive(Clone, Debug)]
struct CaseTemplate {
    family: &'static str,
    condition: &'static str,
    extras: Vec<Delivery>,
    added_side: AddedSide,
    path_change: PathChange,
    expected_target_role: bool,
    expected_tick: i16,
}

#[derive(Clone, Debug)]
struct CaseSpec {
    ordinal: u16,
    schedule: Schedule,
    allocation: TransferAllocation,
    target: usize,
    template: CaseTemplate,
}

fn unit(tick: i16, phase: i16) -> Delivery {
    Delivery::new(tick, phase, 1)
}

fn case_templates(schedule: Schedule) -> Vec<CaseTemplate> {
    let c = schedule.closure();
    let f = schedule.target_ticks[0];
    let t = schedule.target_ticks[2];
    let off = schedule.phase + 17;
    let before = schedule.phase - 1000;
    let after = schedule.phase + 1000;
    let mut cases = Vec::with_capacity(31);
    let mut push = |family,
                    condition,
                    extras,
                    added_side,
                    path_change,
                    expected_target_role,
                    expected_tick| {
        cases.push(CaseTemplate {
            family,
            condition,
            extras,
            added_side,
            path_change,
            expected_target_role,
            expected_tick,
        });
    };

    push(
        "timing",
        "baseline",
        vec![],
        AddedSide::None,
        PathChange::None,
        false,
        c,
    );
    push(
        "timing",
        "well_before",
        vec![unit(c - 7, off)],
        AddedSide::PreClosure,
        PathChange::None,
        true,
        t,
    );
    push(
        "timing",
        "just_before",
        vec![unit(c - 1, off)],
        AddedSide::PreClosure,
        PathChange::None,
        true,
        c - 1,
    );
    push(
        "timing",
        "closure_before",
        vec![unit(c, before)],
        AddedSide::PreClosure,
        PathChange::None,
        true,
        c,
    );
    push(
        "timing",
        "closure_after",
        vec![unit(c, after)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );
    push(
        "timing",
        "just_after",
        vec![unit(c + 1, off)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );
    push(
        "timing",
        "well_after",
        vec![unit(c + 7, off)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );

    push(
        "number",
        "one_early",
        vec![unit(c - 2, off)],
        AddedSide::PreClosure,
        PathChange::None,
        true,
        c - 2,
    );
    push(
        "number",
        "one_late",
        vec![unit(c + 2, off)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );
    push(
        "number",
        "two_early",
        vec![unit(t - 2, off), unit(t - 1, off)],
        AddedSide::PreClosure,
        PathChange::None,
        true,
        t - 1,
    );
    push(
        "number",
        "two_late",
        vec![unit(c + 2, off), unit(c + 5, off)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );

    push(
        "impulse",
        "unit_early",
        vec![Delivery::new(c - 3, off, 1)],
        AddedSide::PreClosure,
        PathChange::None,
        true,
        c - 3,
    );
    push(
        "impulse",
        "unit_late",
        vec![Delivery::new(c + 3, off, 1)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );
    push(
        "impulse",
        "double_early",
        vec![Delivery::new(c - 3, off, 2)],
        AddedSide::PreClosure,
        PathChange::None,
        true,
        c - 3,
    );
    push(
        "impulse",
        "double_late",
        vec![Delivery::new(c + 3, off, 2)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );

    push(
        "spacing",
        "wide_early",
        vec![unit(f + 1, off), unit(t - 1, off)],
        AddedSide::PreClosure,
        PathChange::None,
        true,
        t - 1,
    );
    push(
        "spacing",
        "wide_late",
        vec![unit(c + 2, off), unit(c + 11, off)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );
    push(
        "spacing",
        "near_early",
        vec![unit(t - 2, off), unit(t - 1, off)],
        AddedSide::PreClosure,
        PathChange::None,
        true,
        t - 1,
    );
    push(
        "spacing",
        "near_late",
        vec![unit(c + 2, off), unit(c + 3, off)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );
    push(
        "spacing",
        "coincident_early",
        vec![
            unit(t - 1, schedule.phase - 503),
            unit(t - 1, schedule.phase - 401),
        ],
        AddedSide::PreClosure,
        PathChange::None,
        true,
        t - 1,
    );
    push(
        "spacing",
        "coincident_late",
        vec![
            unit(c + 3, schedule.phase + 1401),
            unit(c + 3, schedule.phase + 1503),
        ],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );

    push(
        "count",
        "total_4_pre_3",
        vec![],
        AddedSide::None,
        PathChange::None,
        false,
        c,
    );
    push(
        "count",
        "total_5_pre_3",
        vec![unit(c + 3, off)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );
    push(
        "count",
        "total_6_pre_3",
        vec![unit(c + 3, off), unit(c + 6, off)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );
    push(
        "count",
        "total_7_pre_3",
        vec![unit(c + 3, off), unit(c + 6, off), unit(c + 9, off)],
        AddedSide::Late,
        PathChange::None,
        false,
        c,
    );

    for (condition, change, expected_target_role, expected_tick) in [
        ("target_blocked", PathChange::TargetBlocked, false, c),
        ("target_stale", PathChange::TargetStale, false, c),
        ("target_absent", PathChange::TargetAbsent, false, c),
        (
            "competitor_blocked",
            PathChange::CompetitorBlocked,
            true,
            schedule.slow_tick(),
        ),
        (
            "competitor_stale",
            PathChange::CompetitorStale,
            true,
            schedule.slow_tick(),
        ),
        (
            "competitor_absent",
            PathChange::CompetitorAbsent,
            true,
            schedule.slow_tick(),
        ),
    ] {
        push(
            "route",
            condition,
            vec![],
            AddedSide::None,
            change,
            expected_target_role,
            expected_tick,
        );
    }
    cases
}

fn matrix_specs() -> Vec<CaseSpec> {
    let mut specs = Vec::with_capacity(744);
    let mut ordinal = 0u16;
    for schedule in SCHEDULES {
        for allocation in ALLOCATIONS {
            for target in 0..=1 {
                for template in case_templates(schedule) {
                    specs.push(CaseSpec {
                        ordinal,
                        schedule,
                        allocation,
                        target,
                        template,
                    });
                    ordinal += 1;
                }
            }
        }
    }
    specs
}

fn base_deliveries(target: usize, schedule: Schedule) -> [Vec<Delivery>; 2] {
    let slow = schedule
        .target_ticks
        .into_iter()
        .map(|tick| unit(tick, schedule.phase))
        .collect();
    let fast = schedule
        .competitor_ticks
        .into_iter()
        .map(|tick| unit(tick, schedule.phase))
        .collect();
    if target == 0 {
        [slow, fast]
    } else {
        [fast, slow]
    }
}

fn configuration(spec: &CaseSpec) -> Configuration {
    let target = spec.target;
    let competitor = 1 - target;
    let mut deliveries = base_deliveries(target, spec.schedule);
    deliveries[target].extend_from_slice(&spec.template.extras);
    let mut blocked = [false, false];
    let mut stale = [false, false];
    match spec.template.path_change {
        PathChange::None => {}
        PathChange::TargetBlocked => blocked[target] = true,
        PathChange::TargetStale => stale[target] = true,
        PathChange::TargetAbsent => deliveries[target].clear(),
        PathChange::CompetitorBlocked => blocked[competitor] = true,
        PathChange::CompetitorStale => stale[competitor] = true,
        PathChange::CompetitorAbsent => deliveries[competitor].clear(),
    }
    Configuration {
        deliveries,
        blocked,
        stale,
        allocation: spec.allocation,
        world_ordinal: spec.ordinal,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ObservedRun {
    first_route: Option<usize>,
    closure_tick: Option<i16>,
    closure_trace_index: Option<usize>,
    effects: BTreeSet<u32>,
    physics: PhysicsRun,
}

fn execute(config: &Configuration) -> (ObservedRun, [u32; 2], [u32; 2]) {
    let fixture = build_fixture(config);
    let contenders = fixture.contenders;
    let effects = fixture.effects;
    let physics = propagate(fixture.state, fixture.permanent_fingerprint);
    let closure_trace_index = physics
        .trace
        .iter()
        .position(|entry| entry.fired && contenders.contains(&entry.target_physical));
    let first_route = closure_trace_index.and_then(|index| {
        contenders
            .iter()
            .position(|physical| *physical == physics.trace[index].target_physical)
    });
    let closure_tick = closure_trace_index.map(|index| physics.trace[index].tick);
    let observed_effects = physics
        .fired
        .iter()
        .filter(|physical| effects.contains(physical))
        .copied()
        .collect();
    (
        ObservedRun {
            first_route,
            closure_tick,
            closure_trace_index,
            effects: observed_effects,
            physics,
        },
        contenders,
        effects,
    )
}

fn added_positions(
    trace: &[TraceEntry],
    target_physical: u32,
    extras: &[Delivery],
) -> Option<Vec<usize>> {
    let mut used = BTreeSet::new();
    let mut positions = Vec::with_capacity(extras.len());
    for extra in extras {
        let position = trace.iter().enumerate().find_map(|(index, entry)| {
            (!used.contains(&index)
                && entry.tick == extra.tick
                && entry.target_physical == target_physical
                && entry.impulse == extra.impulse)
                .then_some(index)
        })?;
        used.insert(position);
        positions.push(position);
    }
    Some(positions)
}

#[derive(Clone, Debug)]
pub struct CaseRow {
    pub ordinal: u16,
    pub schedule: &'static str,
    pub allocation: &'static str,
    pub target: usize,
    pub family: &'static str,
    pub condition: &'static str,
    pub target_total: usize,
    pub competitor_total: usize,
    pub expected_first: usize,
    pub observed_first: Option<usize>,
    pub expected_tick: i16,
    pub observed_tick: Option<i16>,
    pub replay_exact: bool,
    pub immediate_inhibition: bool,
    pub added_order_exact: bool,
    pub late_visible: bool,
    pub single_effect_exact: bool,
    pub disabled_path_absent: bool,
    pub permanent_fingerprint: u64,
    pub start_fingerprint: u64,
    pub trace_fingerprint: u64,
    pub end_fingerprint: u64,
    pub work: u64,
    pub passed: bool,
}

fn evaluate_case(spec: &CaseSpec) -> CaseRow {
    let config = configuration(spec);
    let target_total = config.deliveries[spec.target].len();
    let competitor_total = config.deliveries[1 - spec.target].len();
    let (first, contenders, effects) = execute(&config);
    let (second, second_contenders, second_effects) = execute(&config);
    let replay_exact = first == second
        && contenders == second_contenders
        && effects == second_effects
        && first.physics.start_fingerprint == fingerprint(&first.physics.start_bytes);
    let expected_first = if spec.template.expected_target_role {
        spec.target
    } else {
        1 - spec.target
    };
    let closure_index = first.closure_trace_index;
    let immediate_inhibition = closure_index.is_some_and(|index| {
        first.physics.trace.get(index + 1).is_some_and(|entry| {
            entry.tick == first.physics.trace[index].tick
                && entry.target_physical == contenders[1 - expected_first]
                && entry.impulse == INHIBITORY_IMPULSE
                && !entry.fired
        })
    });
    let positions = added_positions(
        &first.physics.trace,
        contenders[spec.target],
        &spec.template.extras,
    );
    let added_order_exact = match (spec.template.added_side, closure_index, &positions) {
        (AddedSide::None, _, Some(items)) => items.is_empty(),
        (AddedSide::PreClosure, Some(index), Some(items)) => {
            items.len() == spec.template.extras.len() && items.iter().all(|item| *item <= index)
        }
        (AddedSide::Late, Some(index), Some(items)) => {
            items.len() == spec.template.extras.len() && items.iter().all(|item| *item > index)
        }
        _ => false,
    };
    let late_visible = spec.template.added_side != AddedSide::Late || added_order_exact;
    let single_effect_exact = first.effects.len() == 1
        && first.effects.contains(&effects[expected_first])
        && !first.effects.contains(&effects[1 - expected_first]);
    let disabled_route = match spec.template.path_change {
        PathChange::TargetBlocked | PathChange::TargetStale | PathChange::TargetAbsent => {
            Some(spec.target)
        }
        PathChange::CompetitorBlocked
        | PathChange::CompetitorStale
        | PathChange::CompetitorAbsent => Some(1 - spec.target),
        PathChange::None => None,
    };
    let disabled_path_absent = disabled_route.is_none_or(|route| {
        !first.physics.fired.contains(&contenders[route])
            && !first.physics.fired.contains(&effects[route])
    });
    let other_never_fired = !first
        .physics
        .fired
        .contains(&contenders[1 - expected_first]);
    let passed = replay_exact
        && first.first_route == Some(expected_first)
        && first.closure_tick == Some(spec.template.expected_tick)
        && immediate_inhibition
        && added_order_exact
        && late_visible
        && single_effect_exact
        && disabled_path_absent
        && other_never_fired;
    CaseRow {
        ordinal: spec.ordinal,
        schedule: spec.schedule.name,
        allocation: spec.allocation.name,
        target: spec.target,
        family: spec.template.family,
        condition: spec.template.condition,
        target_total,
        competitor_total,
        expected_first,
        observed_first: first.first_route,
        expected_tick: spec.template.expected_tick,
        observed_tick: first.closure_tick,
        replay_exact,
        immediate_inhibition,
        added_order_exact,
        late_visible,
        single_effect_exact,
        disabled_path_absent,
        permanent_fingerprint: first.physics.permanent_fingerprint,
        start_fingerprint: first.physics.start_fingerprint,
        trace_fingerprint: first.physics.trace_fingerprint,
        end_fingerprint: first.physics.end_fingerprint,
        work: first.physics.work.total(),
        passed,
    }
}

#[derive(Clone, Debug)]
pub struct DefinitiveReport {
    pub rows: Vec<CaseRow>,
    pub source_preflight: SourcePreflight,
    pub rows_pass: bool,
    pub replay_exact: bool,
    pub closure_exact: bool,
    pub temporal_exact: bool,
    pub count_exact: bool,
    pub number_exact: bool,
    pub impulse_exact: bool,
    pub spacing_exact: bool,
    pub transfer_exact: bool,
    pub route_exact: bool,
    pub passed: bool,
}

pub fn run_definitive(preflight: SourcePreflight) -> DefinitiveReport {
    assert!(
        preflight.passed,
        "zero-cell preflight must pass before row zero"
    );
    let rows = matrix_specs().iter().map(evaluate_case).collect::<Vec<_>>();
    let rows_pass = rows.len() == 744 && rows.iter().all(|row| row.passed);
    let replay_exact = rows.iter().all(|row| row.replay_exact);
    let closure_exact = rows.iter().all(|row| row.immediate_inhibition);
    let temporal_exact = rows
        .iter()
        .filter(|row| row.family == "timing")
        .all(|row| row.passed);
    let count_exact = rows
        .iter()
        .filter(|row| row.family == "count")
        .all(|row| row.passed && row.observed_first == Some(1 - row.target));
    let number_exact = rows
        .iter()
        .filter(|row| row.family == "number")
        .all(|row| row.passed);
    let impulse_exact = rows
        .iter()
        .filter(|row| row.family == "impulse")
        .all(|row| row.passed);
    let spacing_exact = rows
        .iter()
        .filter(|row| row.family == "spacing")
        .all(|row| row.passed);
    let route_exact = rows
        .iter()
        .filter(|row| row.family == "route")
        .all(|row| row.passed && row.disabled_path_absent);
    let transfer_exact = SCHEDULES.iter().all(|schedule| {
        ALLOCATIONS.iter().all(|allocation| {
            (0..=1).all(|target| {
                rows.iter()
                    .filter(|row| {
                        row.schedule == schedule.name
                            && row.allocation == allocation.name
                            && row.target == target
                    })
                    .count()
                    == 31
            })
        })
    });
    let passed = preflight.passed
        && rows_pass
        && replay_exact
        && closure_exact
        && temporal_exact
        && count_exact
        && number_exact
        && impulse_exact
        && spacing_exact
        && transfer_exact
        && route_exact;
    DefinitiveReport {
        rows,
        source_preflight: preflight,
        rows_pass,
        replay_exact,
        closure_exact,
        temporal_exact,
        count_exact,
        number_exact,
        impulse_exact,
        spacing_exact,
        transfer_exact,
        route_exact,
        passed,
    }
}

// SSA0_DEFINITIVE_RUNTIME_END

fn physics_region(source: &str) -> Option<&str> {
    source
        .split("// SSA0_PHYSICS_BEGIN")
        .nth(1)
        .and_then(|tail| tail.split("// SSA0_PHYSICS_END").next())
}

fn runtime_region(source: &str) -> Option<&str> {
    source
        .split("// SSA0_DEFINITIVE_RUNTIME_BEGIN")
        .nth(1)
        .and_then(|tail| tail.split("// SSA0_DEFINITIVE_RUNTIME_END").next())
}

fn sha256(bytes: &[u8]) -> String {
    const INITIAL: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    const ROUND: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let bit_len = (bytes.len() as u64) * 8;
    let mut padded = bytes.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    let mut state = INITIAL;
    for chunk in padded.chunks_exact(64) {
        let mut words = [0u32; 64];
        for (index, word) in words.iter_mut().take(16).enumerate() {
            *word = u32::from_be_bytes(chunk[index * 4..index * 4 + 4].try_into().unwrap());
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(sum1)
                .wrapping_add(choice)
                .wrapping_add(ROUND[index])
                .wrapping_add(words[index]);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = sum0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }
    state.iter().map(|word| format!("{word:08x}")).collect()
}

#[derive(Clone, Debug)]
pub struct SourcePreflight {
    pub frozen_hashes_exact: bool,
    pub physics_byte_exact: bool,
    pub forbidden_runtime_absent: bool,
    pub matrix_shape_exact: bool,
    pub namespaces_fresh: bool,
    pub outputs_absent: bool,
    pub staging_absent: bool,
    pub passed: bool,
}

pub fn source_preflight(outputs_absent: bool, staging_absent: bool) -> SourcePreflight {
    let frozen = [
        (
            include_bytes!("../experiments/ssa0_spatiotemporal_stochastic_affordance_protocol.md")
                .as_slice(),
            "92ff3f758977a575e2f8ca651f7a45756e15241a6d4bf829012a266bae9489fc",
        ),
        (
            include_bytes!("ssa0_spatiotemporal_affordance.rs").as_slice(),
            "180b24f6b682ec5d274e44b0c680062d10b1f68b6fddeb4d857ec599b32f6299",
        ),
        (
            include_bytes!("bin/ssa0_spatiotemporal_affordance.rs").as_slice(),
            "fb693aa098e45617deefd5ae9b9de1003528d4b7fbfa87078545fbda5e90fa7f",
        ),
        (
            include_bytes!("../results/ssa0_spatiotemporal_affordance_micro_v1_negative.md")
                .as_slice(),
            "905a29fa34c4af08815e039ab195f8b834a5631d4110190f87ace3f45985a0e4",
        ),
        (
            include_bytes!("../results/ssa0_spatiotemporal_affordance_micro_v1_negative.csv")
                .as_slice(),
            "ae61b54a63c8dab35d51fcdb642f91bbc02c696bcd20efc8e877d02a3728fe75",
        ),
        (
            include_bytes!("../experiments/ssa0_3_precommit_support_protocol.md").as_slice(),
            "f9121b0b08867b4189892ab9f46658ebd4a95874c73c5a9182bce17f4f49cef1",
        ),
        (
            include_bytes!("ssa0_3_precommit_support.rs").as_slice(),
            "4a4e727f4f8ca6ee03faaae76de1a1091472de20ed9d91388e7a36056326edd7",
        ),
        (
            include_bytes!("bin/ssa0_3_precommit_support.rs").as_slice(),
            "3711f123be3a4efc0494cc01f85fd8bc176ffc765eb1a2183b1e14c76baea435",
        ),
        (
            include_bytes!("../results/ssa0_3_precommit_support_gate_v1_positive.md").as_slice(),
            "ded620267a4a88e63588c1807149ece25d50876c9b53717827bd9070ead99ce2",
        ),
        (
            include_bytes!("../results/ssa0_3_precommit_support_gate_v1.csv").as_slice(),
            "bae174af3e326d0c3446a023e415914188711f09a61426b95217891d063da4e7",
        ),
        (
            include_bytes!("ds_a1_affordance_multiplicity.rs").as_slice(),
            "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1",
        ),
        (
            include_bytes!("ds8_cumulative_semantic_credit_definitive.rs").as_slice(),
            "c2a95199139828e360713320ad57c77a100fc0135ba06b9219624d4f16e1d18d",
        ),
        (
            include_bytes!("research_runtime.rs").as_slice(),
            "e570b3cd0fcff759a02a38a685f22f33bc28de65e25b7beb34f77d138f3fd711",
        ),
        (
            include_bytes!("../experiments/ssa0_3_precommit_support_definitive_protocol.md")
                .as_slice(),
            DEFINITIVE_PROTOCOL_SHA256,
        ),
    ];
    let frozen_hashes_exact = frozen
        .iter()
        .all(|(bytes, expected)| sha256(bytes) == *expected);
    let parent = include_str!("ssa0_spatiotemporal_affordance.rs");
    let development = include_str!("ssa0_3_precommit_support.rs");
    let definitive = include_str!("ssa0_3_precommit_support_definitive.rs");
    let parent_physics = physics_region(parent).unwrap_or("");
    let development_physics = physics_region(development).unwrap_or("");
    let definitive_physics = physics_region(definitive).unwrap_or("");
    let physics_byte_exact = !parent_physics.is_empty()
        && parent_physics == development_physics
        && parent_physics == definitive_physics;
    let runtime = runtime_region(definitive)
        .unwrap_or("")
        .to_ascii_lowercase();
    let forbidden_runtime_absent = [
        "rand::",
        "thread_rng",
        "softmax",
        "noisy_argmax",
        "temperature",
        "probability",
        "supporter_count",
        "precommit_score",
        "choose(",
        "commitment_cell",
        "semantic_effect",
    ]
    .iter()
    .all(|term| !runtime.contains(term));
    let specs = matrix_specs();
    let matrix_shape_exact = specs.len() == 744
        && specs
            .iter()
            .enumerate()
            .all(|(index, spec)| usize::from(spec.ordinal) == index)
        && SCHEDULES
            .iter()
            .all(|schedule| case_templates(*schedule).len() == 31);
    let namespaces_fresh = specs.iter().all(|spec| {
        let base = 0x9000_0000 + u32::from(spec.ordinal) * 0x0001_0000;
        (0x9000_0000..0x9300_0000).contains(&base)
            && (0xe300_0000..0xe300_02e8).contains(&(0xe300_0000 + u32::from(spec.ordinal)))
            && ![2, 4, 5, 6, 7, 8, 9, 10, 11].contains(&spec.schedule.closure())
            && ![-200, -10, 0, 200].contains(&spec.schedule.phase)
    });
    let passed = frozen_hashes_exact
        && physics_byte_exact
        && forbidden_runtime_absent
        && matrix_shape_exact
        && namespaces_fresh
        && outputs_absent
        && staging_absent;
    SourcePreflight {
        frozen_hashes_exact,
        physics_byte_exact,
        forbidden_runtime_absent,
        matrix_shape_exact,
        namespaces_fresh,
        outputs_absent,
        staging_absent,
        passed,
    }
}

fn optional_usize(value: Option<usize>) -> String {
    value.map_or_else(|| "none".to_string(), |item| item.to_string())
}

fn optional_i16(value: Option<i16>) -> String {
    value.map_or_else(|| "none".to_string(), |item| item.to_string())
}

pub fn csv(report: &DefinitiveReport) -> String {
    let mut text = String::from(
        "row_type,protocol,ordinal,schedule,allocation,target,family,condition,target_total,competitor_total,expected_first,observed_first,expected_tick,observed_tick,replay_exact,immediate_inhibition,added_order_exact,late_visible,single_effect_exact,disabled_path_absent,permanent_fingerprint,start_fingerprint,trace_fingerprint,end_fingerprint,work,status\n",
    );
    for row in &report.rows {
        text.push_str(&format!(
            "case,{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:016x},{:016x},{:016x},{:016x},{},{}\n",
            PROTOCOL,
            row.ordinal,
            row.schedule,
            row.allocation,
            row.target,
            row.family,
            row.condition,
            row.target_total,
            row.competitor_total,
            row.expected_first,
            optional_usize(row.observed_first),
            row.expected_tick,
            optional_i16(row.observed_tick),
            row.replay_exact,
            row.immediate_inhibition,
            row.added_order_exact,
            row.late_visible,
            row.single_effect_exact,
            row.disabled_path_absent,
            row.permanent_fingerprint,
            row.start_fingerprint,
            row.trace_fingerprint,
            row.end_fingerprint,
            row.work,
            if row.passed { "PASS" } else { "FAIL" },
        ));
    }
    text
}

pub fn markdown(report: &DefinitiveReport) -> String {
    let passed_rows = report.rows.iter().filter(|row| row.passed).count();
    let work: u64 = report.rows.iter().map(|row| row.work).sum();
    let verdict = if report.passed {
        "DEFINITIVE POSITIVE"
    } else {
        "DEFINITIVE NEGATIVE"
    };
    format!(
        "# SSA0.3 definitive pre-closure support result\n\nVerdict: **{verdict}**.\n\nProtocol: `{PROTOCOL}`. Definitive rows: `{passed_rows}/744`; exact physical propagations: `1488`; descriptive work ledger: `{work}`.\n\n## Conjunctive predicates\n\n| predicate | pass |\n|---|:---:|\n| frozen hashes and no-cell source preflight | {} |\n| every row | {} |\n| complete-state duplicate replay | {} |\n| first firing plus immediate inhibition closure | {} |\n| early/late and within-tick temporal distinction | {} |\n| equal pre-closure integration rejects static total count | {} |\n| number intervention | {} |\n| impulse/activity intervention | {} |\n| spacing intervention | {} |\n| schedules, mirrors, identities, allocations, and layouts | {} |\n| blocked, stale, absent, and independently executable routes | {} |\n\nAll counts are evaluator descriptions only. The organism stores no probability, count score, choice, semantic label, or commitment boundary. Immutable Classification C and developmental Classification A remain unchanged. M6/M7 authority is unchanged; Lane A is isolated; SSA1 and SSA2 remain blocked.\n",
        report.source_preflight.passed,
        report.rows_pass,
        report.replay_exact,
        report.closure_exact,
        report.temporal_exact,
        report.count_exact,
        report.number_exact,
        report.impulse_exact,
        report.spacing_exact,
        report.transfer_exact,
        report.route_exact,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_known_vector_and_frozen_source_audit() {
        assert_eq!(
            sha256(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert!(source_preflight(true, true).passed);
    }

    #[test]
    fn matrix_shape_is_static_and_exact_without_executing_rows() {
        let specs = matrix_specs();
        assert_eq!(specs.len(), 744);
        assert_eq!(specs.first().unwrap().ordinal, 0);
        assert_eq!(specs.last().unwrap().ordinal, 743);
        assert!(SCHEDULES
            .iter()
            .all(|schedule| case_templates(*schedule).len() == 31));
    }
}
