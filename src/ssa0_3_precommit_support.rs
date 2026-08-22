//! SSA0.3 development-only pre-commitment support integration.
//!
//! The organism path is the byte-exact frozen SSA0 CELL/ARROW/SPIKE
//! propagation loop. Labels, commitment observations, and comparisons exist
//! only in the evaluator around it.

use std::cmp::Ordering;
use std::collections::BTreeSet;

pub const PROTOCOL: &str = "ssa0-3-precommit-support-v1";
pub const FROZEN_PARENT: &str = "34277893201c1a72765b143de4b3da1912b6e3b6";
pub const AUTHORITATIVE_M6: &str = "aa4e22efd8a65b7694956a53cfaa970582695215";
pub const PARENT_PROTOCOL_SHA256: &str =
    "92ff3f758977a575e2f8ca651f7a45756e15241a6d4bf829012a266bae9489fc";
pub const PARENT_IMPLEMENTATION_SHA256: &str =
    "180b24f6b682ec5d274e44b0c680062d10b1f68b6fddeb4d857ec599b32f6299";
pub const PARENT_RUNNER_SHA256: &str =
    "fb693aa098e45617deefd5ae9b9de1003528d4b7fbfa87078545fbda5e90fa7f";

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
    const fn unit(tick: i16) -> Self {
        Self {
            tick,
            phase: 0,
            impulse: 1,
        }
    }

    const fn phased(tick: i16, phase: i16) -> Self {
        Self {
            tick,
            phase,
            impulse: 1,
        }
    }

    const fn strong(tick: i16, impulse: i16) -> Self {
        Self {
            tick,
            phase: 0,
            impulse,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct TransferVariant {
    name: &'static str,
    identity_offset: u32,
    occurrence_origin: u32,
    reverse_cells: bool,
    reverse_arrows: bool,
    layout_padding: usize,
}

const CANONICAL: TransferVariant = TransferVariant {
    name: "canonical",
    identity_offset: 0,
    occurrence_origin: 1,
    reverse_cells: false,
    reverse_arrows: false,
    layout_padding: 0,
};

const MICRO_FRESH: [TransferVariant; 2] = [
    TransferVariant {
        name: "fresh",
        identity_offset: 0x0010_0000,
        occurrence_origin: 0x7000_0001,
        reverse_cells: false,
        reverse_arrows: false,
        layout_padding: 0,
    },
    TransferVariant {
        name: "fresh_permuted",
        identity_offset: 0x0020_0000,
        occurrence_origin: 0x7000_0101,
        reverse_cells: true,
        reverse_arrows: true,
        layout_padding: 17,
    },
];

const GATE_VARIANTS: [TransferVariant; 3] = [
    CANONICAL,
    TransferVariant {
        name: "fresh_alloc_layout",
        identity_offset: 0x0030_0000,
        occurrence_origin: 0x7100_0001,
        reverse_cells: true,
        reverse_arrows: true,
        layout_padding: 17,
    },
    TransferVariant {
        name: "fresh_handle_layout",
        identity_offset: 0x0040_0000,
        occurrence_origin: 0x7200_0001,
        reverse_cells: true,
        reverse_arrows: false,
        layout_padding: 3,
    },
];

#[derive(Clone, Debug)]
struct Configuration {
    deliveries: [Vec<Delivery>; 2],
    blocked: [bool; 2],
    stale: [bool; 2],
    variant: TransferVariant,
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
    config.variant.identity_offset.wrapping_add(base)
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

    let source = physical_id(config, 10);
    let contenders = [physical_id(config, 20), physical_id(config, 30)];
    let effects = [physical_id(config, 40), physical_id(config, 50)];
    let mut cells = vec![CellSpec {
        physical_id: source,
        x: 0,
        threshold: 1,
        activation: 0,
        generation: 1,
        live: true,
    }];
    for (route, contender) in contenders.iter().enumerate() {
        cells.push(CellSpec {
            physical_id: *contender,
            x: if route == 0 { -10 } else { 10 },
            threshold: FIRING_THRESHOLD,
            activation: 0,
            generation: 1,
            live: true,
        });
    }
    for (route, effect) in effects.iter().enumerate() {
        cells.push(CellSpec {
            physical_id: *effect,
            x: if route == 0 { -15 } else { 15 },
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
                x: if route == 0 {
                    -2 - ordinal as i16
                } else {
                    2 + ordinal as i16
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
                phase: 0,
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
    for ordinal in 0..config.variant.layout_padding {
        cells.push(CellSpec {
            physical_id: physical_id(config, 10_000 + ordinal as u32),
            x: 100 + ordinal as i16,
            threshold: 32,
            activation: 0,
            generation: 1,
            live: true,
        });
    }
    if config.variant.reverse_cells {
        cells.reverse();
    }
    if config.variant.reverse_arrows {
        arrows.reverse();
    }
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
            phase: 0,
            origin_physical: config.variant.occurrence_origin,
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct EvaluatedRun {
    route: Option<usize>,
    commitment_tick: Option<i16>,
    effects: BTreeSet<u32>,
    physics: PhysicsRun,
}

fn execute(config: &Configuration) -> EvaluatedRun {
    let fixture = build_fixture(config);
    let contenders = fixture.contenders;
    let effects = fixture.effects;
    let physics = propagate(fixture.state, fixture.permanent_fingerprint);
    let route = physics
        .fired
        .iter()
        .find_map(|physical| contenders.iter().position(|cell| cell == physical));
    let commitment_tick = physics.trace.iter().find_map(|entry| {
        (entry.fired && contenders.contains(&entry.target_physical)).then_some(entry.tick)
    });
    let effects = physics
        .fired
        .iter()
        .filter(|physical| effects.contains(physical))
        .copied()
        .collect();
    EvaluatedRun {
        route,
        commitment_tick,
        effects,
        physics,
    }
}

fn exact_duplicate(config: &Configuration) -> (bool, EvaluatedRun) {
    let first = execute(config);
    let second = execute(config);
    (first == second, first)
}

fn base_deliveries(target: usize) -> [Vec<Delivery>; 2] {
    let slow = [4, 5, 6, 8].into_iter().map(Delivery::unit).collect();
    let fast = [4, 5, 6, 7].into_iter().map(Delivery::unit).collect();
    if target == 0 {
        [slow, fast]
    } else {
        [fast, slow]
    }
}

fn world(target: usize, extras: &[Delivery], variant: TransferVariant) -> Configuration {
    let mut deliveries = base_deliveries(target);
    deliveries[target].extend_from_slice(extras);
    Configuration {
        deliveries,
        blocked: [false, false],
        stale: [false, false],
        variant,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Probe,
    Micro,
    Gate,
}

impl Stage {
    pub fn label(self) -> &'static str {
        match self {
            Self::Probe => "PROBE",
            Self::Micro => "MICRO",
            Self::Gate => "GATE",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Classification {
    A,
    B,
    C,
    D,
}

impl Classification {
    pub fn label(self) -> &'static str {
        match self {
            Self::A => "A_PRECOMMITMENT_SUPPORT_LAW_POSITIVE",
            Self::B => "B_STATIC_COUNT_INDEPENDENT_EFFECT",
            Self::C => "C_NEITHER_DISTINCTION_SURVIVES",
            Self::D => "D_SCIENTIFIC_AMBIGUITY",
        }
    }
}

#[derive(Clone, Debug)]
pub struct CaseRow {
    pub family: &'static str,
    pub condition: &'static str,
    pub variant: &'static str,
    pub target: usize,
    pub target_supporters: usize,
    pub competitor_supporters: usize,
    pub winner: Option<usize>,
    pub expected_winner: Option<usize>,
    pub commitment_tick: Option<i16>,
    pub duplicate_exact: bool,
    pub permanent_fingerprint: u64,
    pub start_fingerprint: u64,
    pub trace_fingerprint: u64,
    pub end_fingerprint: u64,
    pub work: u64,
    pub passed: bool,
}

fn case_row(
    family: &'static str,
    condition: &'static str,
    target: usize,
    extras: &[Delivery],
    variant: TransferVariant,
    expected_winner: usize,
    expected_tick: i16,
) -> CaseRow {
    let config = world(target, extras, variant);
    let target_supporters = config.deliveries[target].len();
    let competitor_supporters = config.deliveries[1 - target].len();
    let (duplicate_exact, result) = exact_duplicate(&config);
    let passed = duplicate_exact
        && result.route == Some(expected_winner)
        && result.commitment_tick == Some(expected_tick)
        && result.effects.len() == 1;
    CaseRow {
        family,
        condition,
        variant: variant.name,
        target,
        target_supporters,
        competitor_supporters,
        winner: result.route,
        expected_winner: Some(expected_winner),
        commitment_tick: result.commitment_tick,
        duplicate_exact,
        permanent_fingerprint: result.physics.permanent_fingerprint,
        start_fingerprint: result.physics.start_fingerprint,
        trace_fingerprint: result.physics.trace_fingerprint,
        end_fingerprint: result.physics.end_fingerprint,
        work: result.physics.work.total(),
        passed,
    }
}

#[derive(Clone, Debug, Default)]
pub struct ControlAudit {
    pub physics_byte_exact: bool,
    pub forbidden_primitives_absent: bool,
    pub duplicate_exact: bool,
    pub fresh_route_identities: bool,
    pub fresh_occurrence_identities: bool,
    pub handle_allocation_layout: bool,
    pub full_mirror: bool,
    pub blocked_route_cannot_win: bool,
    pub stale_route_cannot_win: bool,
    pub independently_executable: bool,
}

impl ControlAudit {
    pub fn passed(&self) -> bool {
        self.physics_byte_exact
            && self.forbidden_primitives_absent
            && self.duplicate_exact
            && self.fresh_route_identities
            && self.fresh_occurrence_identities
            && self.handle_allocation_layout
            && self.full_mirror
            && self.blocked_route_cannot_win
            && self.stale_route_cannot_win
            && self.independently_executable
    }
}

fn physics_region(source: &str) -> Option<&str> {
    source
        .split("// SSA0_PHYSICS_BEGIN")
        .nth(1)
        .and_then(|tail| tail.split("// SSA0_PHYSICS_END").next())
}

fn source_audit() -> (bool, bool) {
    let parent = include_str!("ssa0_spatiotemporal_affordance.rs");
    let successor = include_str!("ssa0_3_precommit_support.rs");
    let parent_physics = physics_region(parent).unwrap_or("");
    let successor_physics = physics_region(successor).unwrap_or("");
    let exact = !parent_physics.is_empty() && parent_physics == successor_physics;
    let forbidden_absent = [
        "rand::",
        "thread_rng",
        "softmax",
        "argmax",
        "probability",
        "score(",
        "effect_id",
        "route_id",
        "choose(",
    ]
    .iter()
    .all(|forbidden| !successor_physics.contains(forbidden));
    (exact, forbidden_absent)
}

fn route_controls(variants: &[TransferVariant]) -> ControlAudit {
    let (physics_byte_exact, forbidden_primitives_absent) = source_audit();
    let duplicate_exact = variants
        .iter()
        .all(|variant| (0..=1).all(|target| exact_duplicate(&world(target, &[], *variant)).0));

    let canonical = (0..=1)
        .map(|target| execute(&world(target, &[], CANONICAL)))
        .collect::<Vec<_>>();
    let identity_only = TransferVariant {
        name: "identity_only_control",
        identity_offset: 0x0050_0000,
        ..CANONICAL
    };
    let occurrence_only = TransferVariant {
        name: "occurrence_only_control",
        occurrence_origin: 0x7300_0001,
        ..CANONICAL
    };
    let fresh_route_identities = (0..=1).all(|target| {
        let run = execute(&world(target, &[], identity_only));
        run.route == canonical[target].route
            && run.commitment_tick == canonical[target].commitment_tick
    });
    let fresh_occurrence_identities = (0..=1).all(|target| {
        let run = execute(&world(target, &[], occurrence_only));
        run.route == canonical[target].route
            && run.commitment_tick == canonical[target].commitment_tick
    });
    let handle_only = TransferVariant {
        name: "handle_only_control",
        reverse_cells: true,
        ..CANONICAL
    };
    let allocation_only = TransferVariant {
        name: "allocation_only_control",
        reverse_arrows: true,
        ..CANONICAL
    };
    let layout_only = TransferVariant {
        name: "layout_only_control",
        layout_padding: 19,
        ..CANONICAL
    };
    let permutation_controls = [handle_only, allocation_only, layout_only];
    let handle_allocation_layout = permutation_controls.iter().all(|variant| {
        (0..=1).all(|target| {
            let run = execute(&world(target, &[Delivery::unit(6)], *variant));
            run.route == Some(target) && run.commitment_tick == Some(6)
        })
    }) && variants.iter().all(|variant| {
        (0..=1).all(|target| {
            let run = execute(&world(target, &[Delivery::unit(6)], *variant));
            run.route == Some(target) && run.commitment_tick == Some(6)
        })
    });
    let full_mirror = variants.iter().all(|variant| {
        let left = execute(&world(0, &[Delivery::unit(6)], *variant));
        let right = execute(&world(1, &[Delivery::unit(6)], *variant));
        left.route == Some(0)
            && right.route == Some(1)
            && left.commitment_tick == right.commitment_tick
    });

    let mut blocked_route_cannot_win = true;
    let mut stale_route_cannot_win = true;
    let mut independently_executable = true;
    for variant in variants {
        for target in 0..=1 {
            let competitor = 1 - target;
            let mut target_blocked = world(target, &[], *variant);
            target_blocked.blocked[target] = true;
            let mut competitor_blocked = world(target, &[], *variant);
            competitor_blocked.blocked[competitor] = true;
            let target_blocked_run = execute(&target_blocked);
            let competitor_blocked_run = execute(&competitor_blocked);
            blocked_route_cannot_win &= target_blocked_run.route == Some(competitor)
                && competitor_blocked_run.route == Some(target);
            independently_executable &= !target_blocked_run.effects.is_empty()
                && !competitor_blocked_run.effects.is_empty()
                && target_blocked_run.effects != competitor_blocked_run.effects;

            let mut target_stale = world(target, &[], *variant);
            target_stale.stale[target] = true;
            let mut competitor_stale = world(target, &[], *variant);
            competitor_stale.stale[competitor] = true;
            stale_route_cannot_win &= execute(&target_stale).route == Some(competitor)
                && execute(&competitor_stale).route == Some(target);
        }
    }

    ControlAudit {
        physics_byte_exact,
        forbidden_primitives_absent,
        duplicate_exact,
        fresh_route_identities,
        fresh_occurrence_identities,
        handle_allocation_layout,
        full_mirror,
        blocked_route_cannot_win,
        stale_route_cannot_win,
        independently_executable,
    }
}

#[derive(Clone, Debug)]
pub struct Report {
    pub stage: Stage,
    pub rows: Vec<CaseRow>,
    pub controls: ControlAudit,
    pub temporal_distinction: bool,
    pub static_count_effect: bool,
    pub comparable_delivery_effect: bool,
    pub classification: Option<Classification>,
    pub passed: bool,
}

fn staged_classification(
    controls: &ControlAudit,
    temporal: bool,
    static_count_effect: bool,
    comparable: bool,
) -> Classification {
    if controls.passed() && static_count_effect {
        Classification::B
    } else if controls.passed() && temporal && comparable {
        Classification::A
    } else if controls.passed() && !temporal && !static_count_effect {
        Classification::C
    } else {
        Classification::D
    }
}

fn probe() -> Report {
    let mut rows = Vec::new();
    for target in 0..=1 {
        rows.push(case_row(
            "core",
            "base",
            target,
            &[],
            CANONICAL,
            1 - target,
            7,
        ));
        rows.push(case_row(
            "core",
            "extra_early",
            target,
            &[Delivery::unit(6)],
            CANONICAL,
            target,
            6,
        ));
        rows.push(case_row(
            "core",
            "extra_late",
            target,
            &[Delivery::unit(10)],
            CANONICAL,
            1 - target,
            7,
        ));
    }
    let controls = route_controls(&[CANONICAL]);
    let temporal_distinction = rows.iter().all(|row| row.passed);
    let static_count_effect = rows
        .iter()
        .filter(|row| row.condition == "extra_late")
        .any(|row| row.winner == Some(row.target));
    let passed = controls.passed() && temporal_distinction && !static_count_effect;
    let classification = (!passed).then(|| {
        staged_classification(
            &controls,
            temporal_distinction,
            static_count_effect,
            temporal_distinction,
        )
    });
    Report {
        stage: Stage::Probe,
        rows,
        controls,
        temporal_distinction,
        static_count_effect,
        comparable_delivery_effect: true,
        classification,
        passed,
    }
}

fn micro() -> Report {
    let mut rows = Vec::new();
    for variant in MICRO_FRESH {
        for target in 0..=1 {
            rows.push(case_row(
                "crossed",
                "base_count4_early3",
                target,
                &[],
                variant,
                1 - target,
                7,
            ));
            rows.push(case_row(
                "crossed",
                "count5_extra_early",
                target,
                &[Delivery::unit(6)],
                variant,
                target,
                6,
            ));
            rows.push(case_row(
                "crossed",
                "count5_extra_late",
                target,
                &[Delivery::unit(10)],
                variant,
                1 - target,
                7,
            ));
        }
    }
    let controls = route_controls(&MICRO_FRESH);
    let temporal_distinction = rows.iter().all(|row| row.passed);
    let static_count_effect = rows
        .iter()
        .filter(|row| row.condition == "count5_extra_late")
        .any(|row| row.winner == Some(row.target));
    let passed = controls.passed() && temporal_distinction && !static_count_effect;
    let classification = (!passed).then(|| {
        staged_classification(
            &controls,
            temporal_distinction,
            static_count_effect,
            temporal_distinction,
        )
    });
    Report {
        stage: Stage::Micro,
        rows,
        controls,
        temporal_distinction,
        static_count_effect,
        comparable_delivery_effect: true,
        classification,
        passed,
    }
}

fn gate() -> Report {
    let timing = [
        ("well_before", Delivery::unit(2), true, 6),
        ("just_before", Delivery::phased(6, -10), true, 6),
        ("threshold_before", Delivery::phased(7, -200), true, 7),
        ("threshold_after", Delivery::phased(7, 200), false, 7),
        ("just_after", Delivery::unit(8), false, 7),
        ("well_after", Delivery::unit(11), false, 7),
    ];
    let number = [
        ("zero", Vec::new(), false, 7),
        ("one_early", vec![Delivery::unit(6)], true, 6),
        (
            "two_early",
            vec![Delivery::unit(5), Delivery::unit(6)],
            true,
            6,
        ),
        ("one_late", vec![Delivery::unit(10)], false, 7),
        (
            "two_late",
            vec![Delivery::unit(10), Delivery::unit(11)],
            false,
            7,
        ),
    ];
    let strength = [
        ("unit_early", Delivery::strong(6, 1), true, 6),
        ("double_early", Delivery::strong(6, 2), true, 6),
        ("unit_late", Delivery::strong(10, 1), false, 7),
        ("double_late", Delivery::strong(10, 2), false, 7),
    ];
    let spacing = [
        (
            "spread_early",
            vec![Delivery::unit(2), Delivery::unit(6)],
            true,
            6,
        ),
        (
            "near_early",
            vec![Delivery::unit(5), Delivery::unit(6)],
            true,
            6,
        ),
        (
            "coincident_early",
            vec![Delivery::unit(6), Delivery::unit(6)],
            true,
            6,
        ),
        (
            "spread_late",
            vec![Delivery::unit(9), Delivery::unit(10)],
            false,
            7,
        ),
        (
            "near_late",
            vec![Delivery::unit(10), Delivery::unit(11)],
            false,
            7,
        ),
        (
            "coincident_late",
            vec![Delivery::unit(11), Delivery::unit(11)],
            false,
            7,
        ),
    ];

    let mut rows = Vec::new();
    for variant in GATE_VARIANTS {
        for target in 0..=1 {
            for (condition, delivery, target_wins, tick) in timing {
                rows.push(case_row(
                    "timing",
                    condition,
                    target,
                    &[delivery],
                    variant,
                    if target_wins { target } else { 1 - target },
                    tick,
                ));
            }
            for (condition, extras, target_wins, tick) in &number {
                rows.push(case_row(
                    "number",
                    condition,
                    target,
                    extras,
                    variant,
                    if *target_wins { target } else { 1 - target },
                    *tick,
                ));
            }
            for (condition, delivery, target_wins, tick) in strength {
                rows.push(case_row(
                    "strength",
                    condition,
                    target,
                    &[delivery],
                    variant,
                    if target_wins { target } else { 1 - target },
                    tick,
                ));
            }
            for (condition, extras, target_wins, tick) in &spacing {
                rows.push(case_row(
                    "spacing",
                    condition,
                    target,
                    extras,
                    variant,
                    if *target_wins { target } else { 1 - target },
                    *tick,
                ));
            }
        }
    }

    let controls = route_controls(&GATE_VARIANTS);
    let temporal_distinction = rows
        .iter()
        .filter(|row| row.family == "timing")
        .all(|row| row.passed);
    let comparable_delivery_effect = rows
        .iter()
        .filter(|row| matches!(row.family, "number" | "strength" | "spacing"))
        .all(|row| row.passed);
    let static_count_effect = rows
        .iter()
        .filter(|row| row.family == "number" && matches!(row.condition, "one_late" | "two_late"))
        .any(|row| row.winner == Some(row.target));
    let classification = staged_classification(
        &controls,
        temporal_distinction,
        static_count_effect,
        comparable_delivery_effect,
    );
    let passed = rows.iter().all(|row| row.passed)
        && controls.passed()
        && classification == Classification::A;
    Report {
        stage: Stage::Gate,
        rows,
        controls,
        temporal_distinction,
        static_count_effect,
        comparable_delivery_effect,
        classification: Some(classification),
        passed,
    }
}

pub fn run(stage: Stage) -> Report {
    match stage {
        Stage::Probe => probe(),
        Stage::Micro => micro(),
        Stage::Gate => gate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copied_physics_is_byte_exact_and_forbidden_free() {
        assert_eq!(source_audit(), (true, true));
    }

    #[test]
    fn complete_physical_replay_is_duplicate_exact() {
        let config = world(0, &[], CANONICAL);
        let (duplicate, first) = exact_duplicate(&config);
        assert!(duplicate);
        assert_eq!(
            first.physics.start_fingerprint,
            fingerprint(&first.physics.start_bytes)
        );
    }

    #[test]
    fn each_route_is_physically_executable_alone() {
        let controls = route_controls(&[CANONICAL]);
        assert!(controls.independently_executable);
        assert!(controls.blocked_route_cannot_win);
        assert!(controls.stale_route_cannot_win);
    }
}
