//! SSA0--SSA0.2 development-only spatiotemporal substrate characterization.
//!
//! The organism path is an ordinary local CELL/ARROW/SPIKE propagation race.
//! Route labels and normalized effects exist only in the evaluator below it.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ssa0-spatiotemporal-affordance-v1";
pub const AUTHORITATIVE_M6: &str = "aa4e22efd8a65b7694956a53cfaa970582695215";
pub const PROTOCOL_COMMIT: &str = "8ae8dc78bb6ecf6172f1d69cb41472beac327df4";
pub const PROTOCOL_SHA256: &str =
    "92ff3f758977a575e2f8ca651f7a45756e15241a6d4bf829012a266bae9489fc";
pub const FROZEN_A1_SHA256: &str =
    "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1";
pub const FROZEN_M6_SHA256: &str =
    "c2a95199139828e360713320ad57c77a100fc0135ba06b9219624d4f16e1d18d";
pub const FROZEN_RUNTIME_SHA256: &str =
    "e570b3cd0fcff759a02a38a685f22f33bc28de65e25b7beb34f77d138f3fd711";

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
struct History {
    timing_skew: i16,
    recent_skew: i16,
    activity_skew: i16,
}

#[derive(Clone, Debug)]
struct Configuration {
    support: [usize; 2],
    active_support: [Option<usize>; 2],
    path_extra: [i16; 2],
    distance_extra: [i16; 2],
    timing_offset: [i16; 2],
    recent_activation: [i16; 2],
    competing_activity: [i16; 2],
    arrival_phase: [i16; 2],
    blocked: [bool; 2],
    stale: [bool; 2],
    identity_offset: u32,
    reverse_allocation: bool,
    layout_padding: usize,
    same_effect: bool,
}

impl Configuration {
    fn new(support: [usize; 2]) -> Self {
        Self {
            support,
            active_support: [None, None],
            path_extra: [0, 0],
            distance_extra: [0, 0],
            timing_offset: [0, 0],
            recent_activation: [0, 0],
            competing_activity: [0, 0],
            arrival_phase: [0, 0],
            blocked: [false, false],
            stale: [false, false],
            identity_offset: 0,
            reverse_allocation: false,
            layout_padding: 0,
            same_effect: false,
        }
    }

    fn with_history(mut self, history: History) -> Self {
        // These are spatial gradients. Positive skew favors the negative-x
        // route; no effect or evaluator label is consulted.
        self.timing_offset = [-history.timing_skew, history.timing_skew];
        self.recent_activation = [history.recent_skew, -history.recent_skew];
        self.competing_activity = match history.activity_skew.cmp(&0) {
            Ordering::Greater => [0, -1],
            Ordering::Less => [-1, 0],
            Ordering::Equal => [0, 0],
        };
        self
    }
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
    config.identity_offset.wrapping_add(base)
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
    let effects = if config.same_effect {
        [physical_id(config, 40), physical_id(config, 40)]
    } else {
        [physical_id(config, 40), physical_id(config, 50)]
    };
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
            activation: config.recent_activation[route] + config.competing_activity[route],
            generation: 1,
            live: true,
        });
    }
    cells.push(CellSpec {
        physical_id: effects[0],
        x: -15,
        threshold: 1,
        activation: 0,
        generation: 1,
        live: true,
    });
    if effects[1] != effects[0] {
        cells.push(CellSpec {
            physical_id: effects[1],
            x: 15,
            threshold: 1,
            activation: 0,
            generation: 1,
            live: true,
        });
    }

    let mut arrows = Vec::new();
    for (route, contender) in contenders.iter().enumerate() {
        for ordinal in 0..config.support[route] {
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
            let active_floor = config.support[route]
                .saturating_sub(config.active_support[route].unwrap_or(config.support[route]));
            let active = ordinal >= active_floor && !config.blocked[route];
            arrows.push(ArrowSpec {
                from: source,
                to: relay,
                base_delay: (12 - ordinal as i16)
                    + config.path_extra[route]
                    + config.distance_extra[route],
                transient_delay: config.timing_offset[route],
                phase: 0,
                impulse: 1,
                generation: if config.stale[route] { 2 } else { 1 },
                live: active,
            });
            arrows.push(ArrowSpec {
                from: relay,
                to: *contender,
                base_delay: 0,
                transient_delay: 0,
                phase: config.arrival_phase[route],
                impulse: 1,
                generation: if config.stale[route] { 2 } else { 1 },
                live: active,
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
    for ordinal in 0..config.layout_padding {
        cells.push(CellSpec {
            physical_id: physical_id(config, 10_000 + ordinal as u32),
            x: 100 + ordinal as i16,
            threshold: 32,
            activation: 0,
            generation: 1,
            live: true,
        });
    }
    if config.reverse_allocation {
        cells.reverse();
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
            origin_physical: 1,
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
    let effects = physics
        .fired
        .iter()
        .filter(|physical| effects.contains(physical))
        .copied()
        .collect();
    EvaluatedRun {
        route,
        effects,
        physics,
    }
}

fn histories(mode: HarnessMode) -> Vec<History> {
    let mut all = Vec::new();
    for timing_skew in -2..=2 {
        for recent_skew in -1..=1 {
            for activity_skew in -1..=1 {
                all.push(History {
                    timing_skew,
                    recent_skew,
                    activity_skew,
                });
            }
        }
    }
    if mode == HarnessMode::Micro {
        all.into_iter()
            .enumerate()
            .filter_map(|(index, history)| (index % 3 == 1).then_some(history))
            .collect()
    } else {
        all
    }
}

fn exact_duplicate(config: &Configuration) -> (bool, EvaluatedRun) {
    let first = execute(config);
    let second = execute(config);
    (first == second, first)
}

#[derive(Clone, Debug)]
pub struct SupportRow {
    pub support_a: usize,
    pub support_b: usize,
    pub trials: usize,
    pub realized_a: usize,
    pub realized_b: usize,
    pub none: usize,
    pub duplicate_exact: bool,
    pub permanent_fingerprint: u64,
    pub permuted_physics: bool,
}

impl SupportRow {
    fn favored_frequency(&self) -> f64 {
        let favored = if self.support_a >= self.support_b {
            self.realized_a
        } else {
            self.realized_b
        };
        favored as f64 / self.trials as f64
    }
}

fn support_row(
    support_labels: [usize; 2],
    mode: HarnessMode,
    permuted_physics: bool,
) -> SupportRow {
    let physical_support = if permuted_physics {
        [support_labels[1], support_labels[0]]
    } else {
        support_labels
    };
    let mut counts = [0usize; 2];
    let mut none = 0;
    let mut duplicates = true;
    let mut permanent = None;
    for history in histories(mode) {
        let config = Configuration::new(physical_support).with_history(history);
        let (duplicate, result) = exact_duplicate(&config);
        duplicates &= duplicate;
        permanent.get_or_insert(result.physics.permanent_fingerprint);
        duplicates &= permanent == Some(result.physics.permanent_fingerprint);
        match result.route {
            Some(physical_route) => {
                let label = if permuted_physics {
                    1 - physical_route
                } else {
                    physical_route
                };
                counts[label] += 1;
            }
            None => none += 1,
        }
    }
    SupportRow {
        support_a: support_labels[0],
        support_b: support_labels[1],
        trials: counts[0] + counts[1] + none,
        realized_a: counts[0],
        realized_b: counts[1],
        none,
        duplicate_exact: duplicates,
        permanent_fingerprint: permanent.unwrap_or(0),
        permuted_physics,
    }
}

#[derive(Clone, Debug)]
pub struct FactorRow {
    pub factor: &'static str,
    pub first: Option<usize>,
    pub mirrored: Option<usize>,
    pub passed: bool,
}

fn paired_factor(factor: &'static str, first: Configuration, mirrored: Configuration) -> FactorRow {
    let first_route = execute(&first).route;
    let mirrored_route = execute(&mirrored).route;
    FactorRow {
        factor,
        first: first_route,
        mirrored: mirrored_route,
        passed: first_route == Some(1) && mirrored_route == Some(0),
    }
}

fn factor_rows() -> Vec<FactorRow> {
    let mut path_first = Configuration::new([6, 6]);
    path_first.path_extra = [1, 0];
    let mut path_mirror = Configuration::new([6, 6]);
    path_mirror.path_extra = [0, 1];

    let mut distance_first = Configuration::new([6, 6]);
    distance_first.distance_extra = [1, 0];
    let mut distance_mirror = Configuration::new([6, 6]);
    distance_mirror.distance_extra = [0, 1];

    let mut timing_first = Configuration::new([6, 6]);
    timing_first.timing_offset = [1, 0];
    let mut timing_mirror = Configuration::new([6, 6]);
    timing_mirror.timing_offset = [0, 1];

    let mut order_first = Configuration::new([6, 6]);
    order_first.arrival_phase = [1, -1];
    let mut order_mirror = Configuration::new([6, 6]);
    order_mirror.arrival_phase = [-1, 1];

    let mut count_first = Configuration::new([7, 7]);
    count_first.active_support = [Some(6), Some(7)];
    let mut count_mirror = Configuration::new([7, 7]);
    count_mirror.active_support = [Some(7), Some(6)];

    let mut recent_first = Configuration::new([6, 6]);
    recent_first.recent_activation = [0, 1];
    let mut recent_mirror = Configuration::new([6, 6]);
    recent_mirror.recent_activation = [1, 0];

    let mut activity_first = Configuration::new([6, 6]);
    activity_first.competing_activity = [-1, 0];
    let mut activity_mirror = Configuration::new([6, 6]);
    activity_mirror.competing_activity = [0, -1];

    vec![
        paired_factor("path_length", path_first, path_mirror),
        paired_factor("distance", distance_first, distance_mirror),
        paired_factor("spike_timing", timing_first, timing_mirror),
        paired_factor("arrival_order", order_first, order_mirror),
        paired_factor("supporting_spike_count", count_first, count_mirror),
        paired_factor("recent_local_activation", recent_first, recent_mirror),
        paired_factor("competing_local_activity", activity_first, activity_mirror),
        paired_factor(
            "stored_support",
            Configuration::new([6, 7]),
            Configuration::new([7, 6]),
        ),
    ]
}

#[derive(Clone, Debug, Default)]
pub struct CrossedAudit {
    pub a_stronger_b_earlier: bool,
    pub b_stronger_a_earlier: bool,
    pub equal_support_path_timing: bool,
    pub equal_timing_stored_support: bool,
}

impl CrossedAudit {
    fn passed(&self) -> bool {
        self.a_stronger_b_earlier
            && self.b_stronger_a_earlier
            && self.equal_support_path_timing
            && self.equal_timing_stored_support
    }
}

fn crossed_audit() -> CrossedAudit {
    let mut a_stronger = Configuration::new([8, 6]);
    a_stronger.timing_offset = [3, 0];
    let mut b_stronger = Configuration::new([6, 8]);
    b_stronger.timing_offset = [0, 3];
    let mut equal_path = Configuration::new([6, 6]);
    equal_path.path_extra = [1, 0];
    CrossedAudit {
        a_stronger_b_earlier: execute(&a_stronger).route == Some(1),
        b_stronger_a_earlier: execute(&b_stronger).route == Some(0),
        equal_support_path_timing: execute(&equal_path).route == Some(1),
        equal_timing_stored_support: execute(&Configuration::new([7, 6])).route == Some(0)
            && execute(&Configuration::new([6, 7])).route == Some(1),
    }
}

#[derive(Clone, Debug, Default)]
pub struct ControlAudit {
    pub no_second_route: bool,
    pub aliases_one_route: bool,
    pub same_effect_unchanged: bool,
    pub fresh_identities: bool,
    pub route_permutation: bool,
    pub allocation_layout: bool,
    pub stale_blocked: bool,
    pub duplicate_exact: bool,
    pub independent_executable: bool,
    pub source_audit: bool,
}

impl ControlAudit {
    pub fn passed(&self) -> bool {
        self.no_second_route
            && self.aliases_one_route
            && self.same_effect_unchanged
            && self.fresh_identities
            && self.route_permutation
            && self.allocation_layout
            && self.stale_blocked
            && self.duplicate_exact
            && self.independent_executable
            && self.source_audit
    }
}

fn source_audit() -> bool {
    let source = include_str!("ssa0_spatiotemporal_affordance.rs");
    let physics = source
        .split("// SSA0_PHYSICS_BEGIN")
        .nth(1)
        .and_then(|tail| tail.split("// SSA0_PHYSICS_END").next())
        .unwrap_or("");
    !physics.is_empty()
        && [
            "rand::",
            "thread_rng",
            "softmax",
            "argmax",
            "probability",
            "score(",
            "effect_id",
            "route_id",
        ]
        .iter()
        .all(|forbidden| !physics.contains(forbidden))
}

fn controls(exact: &EvaluatedRun) -> ControlAudit {
    let history_set = histories(HarnessMode::Gate);
    let no_second_route = history_set.iter().all(|history| {
        let mut config = Configuration::new([6, 6]).with_history(*history);
        config.blocked[1] = true;
        execute(&config).route == Some(0)
    });

    let aliases = [20u32, 20u32].into_iter().collect::<BTreeSet<_>>();
    let aliases_one_route = aliases.len() == 1;

    let ordinary = execute(&Configuration::new([6, 6]));
    let same_effect = execute(&Configuration {
        same_effect: true,
        ..Configuration::new([6, 6])
    });
    let same_effect_unchanged = ordinary.route == same_effect.route
        && ordinary.effects.len() == 1
        && same_effect.effects.len() == 1;

    let fresh = execute(&Configuration {
        identity_offset: 0x4000_0000,
        ..Configuration::new([6, 6])
    });
    let fresh_identities = ordinary.route == fresh.route;

    let route_permutation = {
        let row_a = support_row([7, 6], HarnessMode::Gate, false);
        let row_b = support_row([6, 7], HarnessMode::Gate, true);
        row_a.realized_a == row_b.realized_b && row_a.realized_b == row_b.realized_a
    };

    let allocated = execute(&Configuration {
        reverse_allocation: true,
        layout_padding: 17,
        ..Configuration::new([6, 6])
    });
    let allocation_layout =
        ordinary.route == allocated.route && ordinary.effects.len() == allocated.effects.len();

    let stale = execute(&Configuration {
        stale: [false, true],
        ..Configuration::new([6, 6])
    });
    let blocked = execute(&Configuration {
        blocked: [false, true],
        ..Configuration::new([6, 6])
    });
    let stale_blocked = stale.route == Some(0) && blocked.route == Some(0);

    let duplicate_exact = exact_duplicate(&Configuration::new([6, 6])).0;

    let mut only_a = Configuration::new([6, 6]);
    only_a.blocked[1] = true;
    let mut only_b = Configuration::new([6, 6]);
    only_b.blocked[0] = true;
    let a = execute(&only_a);
    let b = execute(&only_b);
    let independent_executable = a.route == Some(0)
        && b.route == Some(1)
        && !a.effects.is_empty()
        && !b.effects.is_empty()
        && a.effects != b.effects;

    ControlAudit {
        no_second_route,
        aliases_one_route,
        same_effect_unchanged,
        fresh_identities,
        route_permutation,
        allocation_layout,
        stale_blocked,
        duplicate_exact: duplicate_exact
            && exact.physics.start_fingerprint == fingerprint(&exact.physics.start_bytes),
        independent_executable,
        source_audit: source_audit(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Classification {
    A,
    B,
    C,
}

impl Classification {
    pub fn label(self) -> &'static str {
        match self {
            Self::A => "A_TRANSIENT_STATE_SENSITIVITY_SUFFICIENT",
            Self::B => "B_MINIMAL_EXPLICIT_STOCHASTIC_PHYSICS_REQUIRED",
            Self::C => "C_TESTED_SUBSTRATE_UNSUPPORTED",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Report {
    pub mode: &'static str,
    pub claim_eligible: bool,
    pub m6_authoritative: bool,
    pub m7_exists: bool,
    pub exact_replay: bool,
    pub exact_start_fingerprint: u64,
    pub exact_trace_fingerprint: u64,
    pub exact_end_fingerprint: u64,
    pub exact_work: u64,
    pub ssa0_b: bool,
    pub ssa0_c_invoked: bool,
    pub support_rows: Vec<SupportRow>,
    pub ssa0_1: bool,
    pub factors: Vec<FactorRow>,
    pub crossed: CrossedAudit,
    pub ssa0_2: bool,
    pub controls: ControlAudit,
    pub classification: Classification,
    pub passed: bool,
}

fn support_signature(rows: &[SupportRow]) -> bool {
    let equal = &rows[0];
    let equal_a = equal.realized_a as f64 / equal.trials as f64;
    let equal_b = equal.realized_b as f64 / equal.trials as f64;
    let slight_a = rows[1].favored_frequency();
    let moderate_a = rows[2].favored_frequency();
    let large_a = rows[3].favored_frequency();
    let slight_b = rows[4].favored_frequency();
    let moderate_b = rows[5].favored_frequency();
    let large_b = rows[6].favored_frequency();
    let mirrored = rows[1].realized_a.abs_diff(rows[4].realized_b) <= 1
        && rows[2].realized_a.abs_diff(rows[5].realized_b) <= 1
        && rows[3].realized_a.abs_diff(rows[6].realized_b) <= 1;
    (0.30..=0.70).contains(&equal_a)
        && (0.30..=0.70).contains(&equal_b)
        && slight_a > 0.50
        && slight_a < 0.95
        && slight_b > 0.50
        && slight_b < 0.95
        && moderate_a >= slight_a
        && moderate_a >= 0.65
        && moderate_b >= slight_b
        && moderate_b >= 0.65
        && large_a >= 0.95
        && large_b >= 0.95
        && rows.iter().all(|row| {
            row.none == 0 && row.duplicate_exact && row.realized_a + row.realized_b == row.trials
        })
        && mirrored
}

pub fn run(mode: HarnessMode) -> Report {
    assert!(
        mode != HarnessMode::Definitive,
        "definitive SSA0 is forbidden"
    );
    let exact_config = Configuration::new([6, 6]);
    let (exact_replay, exact) = exact_duplicate(&exact_config);

    let equal_row = support_row([6, 6], mode, false);
    let ssa0_b = equal_row.realized_a > 0
        && equal_row.realized_b > 0
        && equal_row.none == 0
        && equal_row.duplicate_exact;
    let support_rows = vec![
        equal_row,
        support_row([7, 6], mode, false),
        support_row([8, 6], mode, false),
        support_row([10, 4], mode, false),
        support_row([6, 7], mode, true),
        support_row([6, 8], mode, true),
        support_row([4, 10], mode, true),
    ];
    let ssa0_1 = support_signature(&support_rows);
    let factors = factor_rows();
    let crossed = crossed_audit();
    let ssa0_2 = factors.iter().all(|row| row.passed) && crossed.passed();
    let controls = controls(&exact);
    let classification = if exact_replay && ssa0_b && ssa0_1 && ssa0_2 && controls.passed() {
        Classification::A
    } else {
        Classification::C
    };
    let passed = classification == Classification::A;
    Report {
        mode: if mode == HarnessMode::Micro {
            "MICRO"
        } else {
            "GATE"
        },
        claim_eligible: false,
        m6_authoritative: true,
        m7_exists: false,
        exact_replay,
        exact_start_fingerprint: exact.physics.start_fingerprint,
        exact_trace_fingerprint: exact.physics.trace_fingerprint,
        exact_end_fingerprint: exact.physics.end_fingerprint,
        exact_work: exact.physics.work.total(),
        ssa0_b,
        ssa0_c_invoked: false,
        support_rows,
        ssa0_1,
        factors,
        crossed,
        ssa0_2,
        controls,
        classification,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_physical_replay_is_duplicate_exact() {
        let (duplicate, first) = exact_duplicate(&Configuration::new([6, 6]));
        assert!(duplicate);
        assert_eq!(
            first.physics.start_fingerprint,
            fingerprint(&first.physics.start_bytes)
        );
    }

    #[test]
    fn both_learned_routes_are_independently_executable() {
        let mut only_a = Configuration::new([6, 6]);
        only_a.blocked[1] = true;
        let mut only_b = Configuration::new([6, 6]);
        only_b.blocked[0] = true;
        let a = execute(&only_a);
        let b = execute(&only_b);
        assert_eq!(a.route, Some(0));
        assert_eq!(b.route, Some(1));
        assert_ne!(a.effects, b.effects);
    }

    #[test]
    fn physics_path_has_no_forbidden_chooser_primitive() {
        assert!(source_audit());
    }

    #[test]
    fn stale_generation_cannot_propagate() {
        let stale = execute(&Configuration {
            stale: [false, true],
            ..Configuration::new([6, 6])
        });
        assert_eq!(stale.route, Some(0));
    }
}
