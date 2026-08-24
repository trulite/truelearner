use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const N: usize = 3;
const DEVICES: usize = 4;
const SCAFFOLD: u32 = 1_000;
const STRIDE: i32 = 16;
const CONTEXT_SPACING: i64 = 12;
const CYCLES: usize = 8;

#[derive(Clone)]
struct Fixture {
    substrate: PlasticSubstrate,
    namespace: u64,
    sources: [CellId; N],
    probes: [CellId; N],
    contenders: [CellId; N],
    backgrounds: [CellId; N],
    supports: [[CellId; DEVICES]; N],
    support_delays: [i64; DEVICES],
    incidental_drivers: [CellId; N],
}

#[derive(Clone, Copy)]
struct Arm {
    name: &'static str,
    namespace: u64,
    stable: usize,
    incidental: usize,
    mirror: bool,
    stride: i32,
    context_spacing: i64,
    cycles: usize,
    incidental_form: usize,
    device_order: [usize; DEVICES],
    distractor_load: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Probe,
    Micro,
    Gate,
    D2,
}

#[derive(Clone, Copy, Debug)]
struct D2Config {
    index: usize,
    namespace: u64,
    initial: usize,
    current: usize,
    spacing: i64,
    stride: i32,
    distractor_load: usize,
    incidental_form: usize,
    reverse_allocation: bool,
    mirrored_layout: bool,
    device_order: [usize; DEVICES],
}

#[derive(Debug)]
struct D2Trajectory {
    cell: usize,
    context: usize,
    form: usize,
    stable_opportunity: usize,
    stable_completed: usize,
    stable_completed_cumulative: usize,
    sparse_opportunity: usize,
    sparse_completed: usize,
    sparse_completed_cumulative: usize,
    stable_resistance: u32,
    sparse_resistance: u32,
    stable_live_arrows: usize,
    sparse_live_arrows: usize,
    b_probe_effect: usize,
    a_probe_effect: usize,
    b_executable_contexts: usize,
    deallocations: u64,
    queue_comparisons: u64,
    work: u64,
}

#[derive(Debug)]
struct D2Cell {
    config: D2Config,
    prefix_ok: bool,
    stable_opportunities: usize,
    stable_completed: usize,
    sparse_opportunities: usize,
    sparse_completed: usize,
    stable_final_resistance: u32,
    sparse_final_resistance: u32,
    first_b_execution_context: isize,
    b_executable_contexts: usize,
    final_b_effects: usize,
    final_a_effects: usize,
    first_deallocation_context: isize,
    stable_deallocation_delay: i64,
    sparse_deallocation_delay: i64,
    proposals: u64,
    deallocations: u64,
    queue_comparisons: u64,
    work: u64,
    diagnostic_probe_work: u64,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    duplicate_exact: bool,
    naturally_quiescent: bool,
}

#[derive(Debug)]
struct ResultRow {
    arm: &'static str,
    stable: usize,
    incidental: usize,
    stable_returns: usize,
    incidental_returns: usize,
    stable_devices_used: usize,
    stable_max_resistance: u32,
    incidental_final_max_resistance: u32,
    stable_effects: usize,
    incidental_effects: usize,
    incidental_eventually_dead: bool,
    duplicate_exact: bool,
    naturally_quiescent: bool,
    contexts: usize,
    work: u64,
    persistent_bytes: usize,
    passed: bool,
}

#[derive(Debug)]
struct ControlRow {
    kind: &'static str,
    arm: &'static str,
    route_0_returns: usize,
    route_1_returns: usize,
    route_0_effects: usize,
    route_1_effects: usize,
    simultaneous_effects: usize,
    duplicate_exact: bool,
    naturally_quiescent: bool,
    work: u64,
    passed: bool,
}

#[derive(Debug)]
struct ContextRow {
    arm: &'static str,
    ordinal: usize,
    form: usize,
    stable_device: usize,
    stable_returns: usize,
    incidental_returns: usize,
    crossings: usize,
    work: u64,
}

fn main() {
    let mut args = env::args().skip(1);
    let mut output_prefix = None;
    let mut mode = Mode::Probe;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--output-prefix" => {
                output_prefix = Some(PathBuf::from(
                    args.next().expect("--output-prefix requires a path"),
                ));
            }
            "--micro" => mode = Mode::Micro,
            "--gate" => mode = Mode::Gate,
            "--d2" => mode = Mode::D2,
            "--definitive" | "--definitive-v2" | "--definitive-v3" => {
                eprintln!("PX0-S is development-only; definitive execution is forbidden");
                std::process::exit(2);
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    let output_prefix = output_prefix.expect("development output prefix is required");
    if mode == Mode::D2 {
        let completed = run_d2(&output_prefix);
        std::process::exit(if completed { 0 } else { 1 });
    }

    let probe_arms = vec![
        Arm {
            name: "stable-route-1-direct",
            namespace: 0xb00_0000,
            stable: 1,
            incidental: 0,
            mirror: false,
            stride: STRIDE,
            context_spacing: CONTEXT_SPACING,
            cycles: CYCLES,
            incidental_form: 0,
            device_order: [0, 1, 2, 3],
            distractor_load: 0,
        },
        Arm {
            name: "stable-route-0-mirrored",
            namespace: 0xb40_0000,
            stable: 0,
            incidental: 1,
            mirror: true,
            stride: STRIDE,
            context_spacing: CONTEXT_SPACING,
            cycles: CYCLES,
            incidental_form: 0,
            device_order: [0, 1, 2, 3],
            distractor_load: 0,
        },
    ];
    let micro_arms = vec![
        arm(
            "micro-phase-2-direct",
            0xc00_0000,
            2,
            0,
            false,
            12,
            10,
            6,
            2,
            [2, 0, 3, 1],
            2,
        ),
        arm(
            "micro-phase-3-mirror",
            0xc40_0000,
            0,
            2,
            true,
            20,
            14,
            10,
            3,
            [3, 1, 0, 2],
            8,
        ),
        arm(
            "micro-route-1-direct",
            0xc80_0000,
            1,
            2,
            false,
            14,
            11,
            7,
            1,
            [1, 3, 2, 0],
            4,
        ),
        arm(
            "micro-route-2-mirror",
            0xcc0_0000,
            2,
            1,
            true,
            18,
            16,
            9,
            0,
            [0, 2, 1, 3],
            16,
        ),
    ];
    let gate_arms = vec![
        arm(
            "gate-00",
            0xd00_0000,
            0,
            1,
            false,
            12,
            10,
            6,
            0,
            [0, 1, 2, 3],
            0,
        ),
        arm(
            "gate-01",
            0xd20_0000,
            1,
            0,
            true,
            14,
            11,
            7,
            1,
            [1, 2, 3, 0],
            2,
        ),
        arm(
            "gate-02",
            0xd40_0000,
            2,
            0,
            false,
            16,
            12,
            8,
            2,
            [2, 3, 0, 1],
            4,
        ),
        arm(
            "gate-03",
            0xd60_0000,
            0,
            2,
            true,
            18,
            13,
            9,
            3,
            [3, 0, 1, 2],
            8,
        ),
        arm(
            "gate-04",
            0xd80_0000,
            1,
            2,
            false,
            20,
            14,
            10,
            0,
            [3, 1, 0, 2],
            16,
        ),
        arm(
            "gate-05",
            0xda0_0000,
            2,
            1,
            true,
            22,
            15,
            6,
            1,
            [2, 0, 3, 1],
            32,
        ),
        arm(
            "gate-06",
            0xdc0_0000,
            0,
            1,
            true,
            24,
            16,
            7,
            2,
            [1, 3, 2, 0],
            1,
        ),
        arm(
            "gate-07",
            0xde0_0000,
            1,
            0,
            false,
            26,
            17,
            8,
            3,
            [0, 2, 1, 3],
            3,
        ),
        arm(
            "gate-08",
            0xe00_0000,
            2,
            0,
            true,
            28,
            18,
            9,
            0,
            [2, 1, 3, 0],
            6,
        ),
        arm(
            "gate-09",
            0xe20_0000,
            0,
            2,
            false,
            30,
            19,
            10,
            1,
            [1, 0, 2, 3],
            12,
        ),
        arm(
            "gate-10",
            0xe40_0000,
            1,
            2,
            true,
            32,
            20,
            6,
            2,
            [3, 2, 0, 1],
            24,
        ),
        arm(
            "gate-11",
            0xe60_0000,
            2,
            1,
            false,
            34,
            21,
            8,
            3,
            [0, 3, 1, 2],
            40,
        ),
    ];
    let arms = match mode {
        Mode::Probe => probe_arms,
        Mode::Micro => micro_arms,
        Mode::Gate => gate_arms,
        Mode::D2 => unreachable!("D2 exits before legacy arm selection"),
    };
    let mut results = Vec::new();
    let mut contexts = Vec::new();
    for arm in arms {
        let (result, rows) = run_specificity_arm(arm);
        results.push(result);
        contexts.extend(rows);
    }
    let control_base = match mode {
        Mode::Probe => 0xb80_0000,
        Mode::Micro => 0xcf0_0000,
        Mode::Gate => 0xe80_0000,
        Mode::D2 => unreachable!("D2 exits before legacy controls"),
    };
    let mut controls = vec![
        run_recurring_control("recurring-route-0-direct", control_base, 0, 1, false),
        run_recurring_control(
            "recurring-route-1-mirrored",
            control_base + 0x10_000,
            1,
            0,
            true,
        ),
        run_absent_control("absent-return-direct", control_base + 0x20_000, false),
        run_absent_control("absent-return-mirror", control_base + 0x30_000, true),
    ];
    if mode != Mode::Probe {
        controls.push(run_switch_control(
            "stability-switch-continuous",
            control_base + 0x40_000,
            mode == Mode::Gate,
        ));
    }
    let passed = results.iter().all(|row| row.passed) && controls.iter().all(|row| row.passed);
    write_results(&output_prefix, &results, &controls, &contexts, passed, mode);
    if !passed {
        std::process::exit(1);
    }
}

#[allow(clippy::too_many_arguments)]
fn arm(
    name: &'static str,
    namespace: u64,
    stable: usize,
    incidental: usize,
    mirror: bool,
    stride: i32,
    context_spacing: i64,
    cycles: usize,
    incidental_form: usize,
    device_order: [usize; DEVICES],
    distractor_load: usize,
) -> Arm {
    Arm {
        name,
        namespace,
        stable,
        incidental,
        mirror,
        stride,
        context_spacing,
        cycles,
        incidental_form,
        device_order,
        distractor_load,
    }
}

fn cell(id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id: id,
        position,
        region,
        threshold,
        resistance: SCAFFOLD,
    }
}

fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: SCAFFOLD,
    }
}

fn build(namespace: u64, mirror: bool, stride: i32, distractor_load: usize) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; N];
    let mut probes = [None; N];
    let mut contenders = [None; N];
    let mut gates = [None; N];
    let mut backgrounds = [None; N];
    let mut supports = [[None; DEVICES]; N];
    let mut incidental_drivers = [None; N];
    let support_delays = [1, 2, 3, 4];

    for route in 0..N {
        let slot = if mirror { N - 1 - route } else { route };
        let position = slot as i32 * stride;
        sources[route] =
            Some(substrate.add_cell(cell(namespace + 10 + route as u64, position, 0, 2)));
        probes[route] =
            Some(substrate.add_cell(cell(namespace + 20 + route as u64, position + 1, -1, 2)));
        contenders[route] =
            Some(substrate.add_cell(cell(namespace + 30 + route as u64, position + 2, 0, 2)));
        gates[route] =
            Some(substrate.add_cell(cell(namespace + 40 + route as u64, position + 4, -1, 2)));
        backgrounds[route] = Some(substrate.add_cell(cell(
            namespace + 50 + route as u64,
            2_000 + route as i32 * 10,
            -2,
            1,
        )));
        incidental_drivers[route] =
            Some(substrate.add_cell(cell(namespace + 60 + route as u64, position + 6, -2, 2)));
        for (device, support) in supports[route].iter_mut().enumerate() {
            *support = Some(substrate.add_cell(cell(
                namespace + 100 + route as u64 * 10 + device as u64,
                3_000 + route as i32 * 100 + device as i32 * 11,
                -2,
                1,
            )));
        }
        for distractor in 0..distractor_load {
            substrate.add_cell(cell(
                namespace + 1_000 + route as u64 * 100 + distractor as u64,
                position + 5 + (distractor % 3) as i32,
                -3,
                2,
            ));
        }
    }

    let sources = sources.map(|value| value.expect("source allocated"));
    let probes = probes.map(|value| value.expect("probe allocated"));
    let contenders = contenders.map(|value| value.expect("contender allocated"));
    let gates = gates.map(|value| value.expect("gate allocated"));
    let backgrounds = backgrounds.map(|value| value.expect("background allocated"));
    let supports = supports.map(|route| route.map(|value| value.expect("support allocated")));
    let incidental_drivers =
        incidental_drivers.map(|value| value.expect("incidental driver allocated"));
    let veto = substrate.add_cell(cell(namespace + 900, 5_000, 0, 2));
    let accumulator = substrate.add_cell(cell(namespace + 901, 5_001, 0, 1));
    let outside = substrate.add_cell(cell(namespace + 902, 5_002, 1, 1));

    for route in 0..N {
        substrate.add_arrow(arrow(probes[route], gates[route], 1, 1));
        substrate.add_arrow(arrow(gates[route], sources[route], 1, 1));
        substrate.add_arrow(arrow(backgrounds[route], probes[route], 1, 1));
        for device in 0..DEVICES {
            substrate.add_arrow(arrow(
                supports[route][device],
                gates[route],
                support_delays[device],
                1,
            ));
        }
        substrate.add_arrow(arrow(contenders[route], veto, 0, 1));
        substrate.add_arrow(arrow(contenders[route], accumulator, 2, 1));
    }
    substrate.add_arrow(arrow(veto, accumulator, 1, -4));
    substrate.add_arrow(arrow(accumulator, outside, 0, 1));

    Fixture {
        substrate,
        namespace,
        sources,
        probes,
        contenders,
        backgrounds,
        supports,
        support_delays,
        incidental_drivers,
    }
}

fn build_d2(config: D2Config) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; N];
    let mut probes = [None; N];
    let mut contenders = [None; N];
    let mut gates = [None; N];
    let mut backgrounds = [None; N];
    let mut supports = [[None; DEVICES]; N];
    let mut incidental_drivers = [None; N];
    let support_delays = [1, 2, 3, 4];
    let order = if config.reverse_allocation {
        [2, 1, 0]
    } else {
        [0, 1, 2]
    };

    for route in order {
        let slot = if config.mirrored_layout {
            N - 1 - route
        } else {
            route
        };
        let position = slot as i32 * config.stride;
        sources[route] =
            Some(substrate.add_cell(cell(config.namespace + 10 + route as u64, position, 0, 2)));
        probes[route] = Some(substrate.add_cell(cell(
            config.namespace + 20 + route as u64,
            position + 1,
            -1,
            2,
        )));
        contenders[route] = Some(substrate.add_cell(cell(
            config.namespace + 30 + route as u64,
            position + 2,
            0,
            2,
        )));
        gates[route] = Some(substrate.add_cell(cell(
            config.namespace + 40 + route as u64,
            position + 4,
            -1,
            2,
        )));
        backgrounds[route] = Some(substrate.add_cell(cell(
            config.namespace + 50 + route as u64,
            2_000 + route as i32 * 10,
            -2,
            1,
        )));
        incidental_drivers[route] = Some(substrate.add_cell(cell(
            config.namespace + 60 + route as u64,
            position + 6,
            -2,
            2,
        )));
        for (device, support) in supports[route].iter_mut().enumerate() {
            *support = Some(substrate.add_cell(cell(
                config.namespace + 100 + route as u64 * 10 + device as u64,
                3_000 + route as i32 * 100 + device as i32 * 11,
                -2,
                1,
            )));
        }
        for distractor in 0..config.distractor_load {
            substrate.add_cell(cell(
                config.namespace + 1_000 + route as u64 * 100 + distractor as u64,
                position + 5 + (distractor % 3) as i32,
                -3,
                2,
            ));
        }
    }

    let sources = sources.map(|value| value.expect("source allocated"));
    let probes = probes.map(|value| value.expect("probe allocated"));
    let contenders = contenders.map(|value| value.expect("contender allocated"));
    let gates = gates.map(|value| value.expect("gate allocated"));
    let backgrounds = backgrounds.map(|value| value.expect("background allocated"));
    let supports = supports.map(|route| route.map(|value| value.expect("support allocated")));
    let incidental_drivers =
        incidental_drivers.map(|value| value.expect("incidental driver allocated"));
    let veto = substrate.add_cell(cell(config.namespace + 900, 5_000, 0, 2));
    let accumulator = substrate.add_cell(cell(config.namespace + 901, 5_001, 0, 1));
    let outside = substrate.add_cell(cell(config.namespace + 902, 5_002, 1, 1));

    for route in order {
        substrate.add_arrow(arrow(probes[route], gates[route], 1, 1));
        substrate.add_arrow(arrow(gates[route], sources[route], 1, 1));
        substrate.add_arrow(arrow(backgrounds[route], probes[route], 1, 1));
        for device in 0..DEVICES {
            substrate.add_arrow(arrow(
                supports[route][device],
                gates[route],
                support_delays[device],
                1,
            ));
        }
        substrate.add_arrow(arrow(contenders[route], veto, 0, 1));
        substrate.add_arrow(arrow(contenders[route], accumulator, 2, 1));
    }
    substrate.add_arrow(arrow(veto, accumulator, 1, -4));
    substrate.add_arrow(arrow(accumulator, outside, 0, 1));

    Fixture {
        substrate,
        namespace: config.namespace,
        sources,
        probes,
        contenders,
        backgrounds,
        supports,
        support_delays,
        incidental_drivers,
    }
}

fn enter_twice(fixture: &mut Fixture, target: CellId, tick: i64, origin: u64) {
    for ordinal in 0..2 {
        fixture.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: ordinal,
            origin_physical: origin + ordinal as u64,
            target,
            impulse: 1,
        });
    }
}

fn activate_candidates(fixture: &mut Fixture, routes: &[usize], tick: i64, origin: u64) {
    for route in routes {
        enter_twice(
            fixture,
            fixture.sources[*route],
            tick,
            origin + *route as u64 * 16,
        );
        fixture.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: origin + 0x100 + *route as u64,
            target: fixture.backgrounds[*route],
            impulse: 1,
        });
    }
}

fn activate_support(
    fixture: &mut Fixture,
    route: usize,
    device: usize,
    gate_arrival: i64,
    origin: u64,
) {
    fixture.substrate.enter(SpikeInput {
        arrival_tick: gate_arrival - fixture.support_delays[device],
        phase: 0,
        origin_physical: origin,
        target: fixture.supports[route][device],
        impulse: 1,
    });
}

fn activate_driver(fixture: &mut Fixture, route: usize, tick: i64, origin: u64) {
    enter_twice(fixture, fixture.incidental_drivers[route], tick, origin);
}

#[allow(clippy::too_many_arguments)]
fn run_context(
    fixture: &mut Fixture,
    stable: usize,
    incidental: usize,
    stable_device: usize,
    incidental_active: bool,
    elsewhere_active: bool,
    base: i64,
    origin: u64,
) -> Execution {
    let spare = 3 - stable - incidental;
    let mut active = vec![stable, incidental];
    if elsewhere_active {
        active.push(spare);
    }
    activate_candidates(fixture, &active, base + 2, origin);
    activate_support(fixture, stable, stable_device, base + 4, origin + 0x200);
    if incidental_active {
        activate_driver(fixture, incidental, base + 2, origin + 0x300);
    }
    if elsewhere_active {
        activate_driver(fixture, spare, base + 2, origin + 0x400);
    }
    fixture.substrate.propagate()
}

fn held_out(fixture: &mut Fixture, route: usize, tick: i64, origin: u64) -> Execution {
    activate_candidates(fixture, &[route], tick, origin);
    fixture.substrate.propagate()
}

fn effects(execution: &Execution) -> usize {
    execution
        .crossings
        .iter()
        .filter(|crossing| crossing.from_region == 0 && crossing.to_region == 1)
        .count()
}

fn delayed_returns(fixture: &Fixture, route: usize, base: i64, execution: &Execution) -> usize {
    let source_physical = fixture.namespace + 10 + route as u64;
    execution
        .trace
        .iter()
        .filter(|entry| entry.target_physical == source_physical && entry.tick > base + 2)
        .count()
}

fn variable_ids(fixture: &Fixture, route: usize) -> Vec<ArrowId> {
    let mut ids = fixture
        .substrate
        .arrows_between(fixture.sources[route], fixture.probes[route]);
    ids.extend(
        fixture
            .substrate
            .arrows_between(fixture.sources[route], fixture.contenders[route]),
    );
    ids
}

fn max_live_resistance(fixture: &Fixture, route: usize) -> u32 {
    variable_ids(fixture, route)
        .into_iter()
        .filter(|arrow| fixture.substrate.arrow_is_live(*arrow))
        .map(|arrow| fixture.substrate.arrow_resistance(arrow))
        .max()
        .unwrap_or(0)
}

fn live_variable_count(fixture: &Fixture, route: usize) -> usize {
    variable_ids(fixture, route)
        .into_iter()
        .filter(|arrow| fixture.substrate.arrow_is_live(*arrow))
        .count()
}

fn d2_enter_twice(fixture: &mut Fixture, target: CellId, tick: i64, origin: u64, reverse: bool) {
    let phases = if reverse { [1, 0] } else { [0, 1] };
    for (ordinal, phase) in phases.into_iter().enumerate() {
        fixture.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase,
            origin_physical: origin + ordinal as u64,
            target,
            impulse: 1,
        });
    }
}

fn d2_activate_candidates(
    fixture: &mut Fixture,
    routes: &[usize],
    tick: i64,
    origin: u64,
    reverse: bool,
) {
    let ordered = if reverse {
        routes.iter().rev().copied().collect::<Vec<_>>()
    } else {
        routes.to_vec()
    };
    for route in ordered {
        d2_enter_twice(
            fixture,
            fixture.sources[route],
            tick,
            origin + route as u64 * 16,
            reverse,
        );
        fixture.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: origin + 0x100 + route as u64,
            target: fixture.backgrounds[route],
            impulse: 1,
        });
    }
}

fn d2_supported_experience(
    fixture: &mut Fixture,
    route: usize,
    tick: i64,
    origin: u64,
    reverse: bool,
) -> Execution {
    d2_activate_candidates(fixture, &[0, 1, 2], tick, origin, reverse);
    activate_support(fixture, route, 1, tick + 2, origin + 0x200);
    fixture.substrate.propagate()
}

#[allow(clippy::too_many_arguments)]
fn d2_context(
    fixture: &mut Fixture,
    stable: usize,
    sparse: usize,
    spare: usize,
    device: usize,
    sparse_active: bool,
    elsewhere_active: bool,
    base: i64,
    origin: u64,
    reverse: bool,
) -> Execution {
    d2_activate_candidates(fixture, &[stable, sparse, spare], base + 2, origin, reverse);
    activate_support(fixture, stable, device, base + 4, origin + 0x200);
    if sparse_active {
        d2_enter_twice(
            fixture,
            fixture.incidental_drivers[sparse],
            base + 2,
            origin + 0x300,
            reverse,
        );
    }
    if elsewhere_active {
        d2_enter_twice(
            fixture,
            fixture.incidental_drivers[spare],
            base + 2,
            origin + 0x400,
            !reverse,
        );
    }
    fixture.substrate.propagate()
}

fn d2_held_out(
    fixture: &mut Fixture,
    route: usize,
    tick: i64,
    origin: u64,
    reverse: bool,
) -> Execution {
    d2_activate_candidates(fixture, &[route], tick, origin, reverse);
    fixture.substrate.propagate()
}

fn route_deallocation_delay(mut fixture: Fixture, route: usize, start_tick: i64) -> i64 {
    for step in 1..=200 {
        let delay = step * 10;
        fixture.substrate.advance_time(start_tick + delay);
        if live_variable_count(&fixture, route) == 0 {
            return delay;
        }
    }
    -1
}

fn d2_configs() -> Vec<D2Config> {
    let spacings = [13, 15, 17, 19];
    let strides = [24, 25, 27, 28];
    let loads = [32, 36, 44, 48];
    let mut configs = Vec::new();
    for (spacing_index, spacing) in spacings.into_iter().enumerate() {
        for (stride_index, stride) in strides.into_iter().enumerate() {
            for (load_index, distractor_load) in loads.into_iter().enumerate() {
                for orientation in 0..4 {
                    let index = configs.len();
                    let initial = index % N;
                    let current = (initial + 1) % N;
                    let incidental_form =
                        (spacing_index + 2 * stride_index + load_index + orientation) % DEVICES;
                    let rotation = (index + incidental_form) % DEVICES;
                    let mut device_order = [0, 1, 2, 3];
                    device_order.rotate_left(rotation);
                    if orientation.is_multiple_of(2) {
                        device_order.reverse();
                    }
                    configs.push(D2Config {
                        index,
                        namespace: 0x1_4000_0000 + index as u64 * 0x8_0000,
                        initial,
                        current,
                        spacing,
                        stride,
                        distractor_load,
                        incidental_form,
                        reverse_allocation: orientation >= 2,
                        mirrored_layout: orientation % 2 == 1,
                        device_order,
                    });
                }
            }
        }
    }
    configs
}

fn run_d2_cell(config: D2Config) -> (D2Cell, Vec<D2Trajectory>) {
    let spare = 3 - config.initial - config.current;
    let mut fixture = build_d2(config);
    let mut naturally_quiescent = true;
    let mut proposals = 0u64;
    let mut deallocations = 0u64;
    let mut queue_comparisons = 0u64;
    let mut work = 0u64;
    for presentation in 0usize..4 {
        let run = d2_supported_experience(
            &mut fixture,
            config.initial,
            presentation as i64 * config.spacing,
            config.namespace + 0x1_000 + presentation as u64 * 0x100,
            config.reverse_allocation ^ presentation.is_multiple_of(2),
        );
        proposals += run.work.local_structural_proposals;
        deallocations += run.work.physical_deallocations;
        queue_comparisons += run.work.queue_comparisons;
        work += run.work.total();
        naturally_quiescent &= run.naturally_quiescent;
    }
    let old_ids = variable_ids(&fixture, config.initial);
    let acquired = old_ids.len() == 2
        && old_ids
            .iter()
            .all(|arrow| fixture.substrate.arrow_is_live(*arrow));
    let initial_run = d2_supported_experience(
        &mut fixture,
        config.initial,
        4 * config.spacing,
        config.namespace + 0x2_000,
        !config.reverse_allocation,
    );
    proposals += initial_run.work.local_structural_proposals;
    deallocations += initial_run.work.physical_deallocations;
    queue_comparisons += initial_run.work.queue_comparisons;
    work += initial_run.work.total();
    naturally_quiescent &= initial_run.naturally_quiescent;
    let survival_tick = 4 * config.spacing + 20;
    let pressure = fixture.substrate.advance_time(survival_tick);
    deallocations += pressure.physical_deallocations;
    work += pressure.total();
    let same_live_before = old_ids
        .iter()
        .all(|arrow| fixture.substrate.arrow_is_live(*arrow));
    let survival_run = d2_supported_experience(
        &mut fixture,
        config.initial,
        survival_tick,
        config.namespace + 0x2_800,
        config.reverse_allocation,
    );
    proposals += survival_run.work.local_structural_proposals;
    deallocations += survival_run.work.physical_deallocations;
    queue_comparisons += survival_run.work.queue_comparisons;
    work += survival_run.work.total();
    naturally_quiescent &= survival_run.naturally_quiescent;
    let same_live_after = old_ids
        .iter()
        .all(|arrow| fixture.substrate.arrow_is_live(*arrow));
    let forgetting = fixture.substrate.advance_time(300);
    deallocations += forgetting.physical_deallocations;
    work += forgetting.total();
    let old_dead = old_ids.iter().all(|arrow| {
        !fixture.substrate.arrow_is_live(*arrow) && fixture.substrate.arrow_resistance(*arrow) == 0
    });
    let stale_run = d2_held_out(
        &mut fixture,
        config.initial,
        300,
        config.namespace + 0x3_000,
        !config.reverse_allocation,
    );
    proposals += stale_run.work.local_structural_proposals;
    deallocations += stale_run.work.physical_deallocations;
    queue_comparisons += stale_run.work.queue_comparisons;
    work += stale_run.work.total();
    naturally_quiescent &= stale_run.naturally_quiescent;
    let prefix_ok = acquired
        && effects(&initial_run) == 1
        && same_live_before
        && same_live_after
        && effects(&survival_run) == 1
        && old_dead
        && effects(&stale_run) == 0;

    let dense_start = 300 + config.spacing;
    let mut stable_completed = 0usize;
    let mut sparse_completed = 0usize;
    let mut sparse_opportunities = 0usize;
    let mut b_executable_contexts = 0usize;
    let mut first_b_execution_context = -1isize;
    let mut first_deallocation_context = -1isize;
    let mut diagnostic_probe_work = 0u64;
    let mut trajectory = Vec::new();
    let mut context = 0usize;
    let mut forms = Vec::new();
    for _ in 0..8 {
        forms.extend(0..DEVICES);
    }
    forms.extend((0..DEVICES).filter(|form| *form != config.incidental_form));
    for form in forms {
        let sparse_opportunity =
            usize::from(context < 8 * DEVICES && form == config.incidental_form);
        sparse_opportunities += sparse_opportunity;
        let base = dense_start + context as i64 * config.spacing;
        let run = d2_context(
            &mut fixture,
            config.current,
            config.initial,
            spare,
            config.device_order[form],
            sparse_opportunity == 1,
            form == (config.incidental_form + 3) % DEVICES,
            base,
            config.namespace + 0x4_000 + context as u64 * 0x100,
            config.reverse_allocation ^ context.is_multiple_of(2),
        );
        let stable_here = delayed_returns(&fixture, config.current, base, &run);
        let sparse_here = delayed_returns(&fixture, config.initial, base, &run);
        stable_completed += stable_here;
        sparse_completed += sparse_here;
        proposals += run.work.local_structural_proposals;
        deallocations += run.work.physical_deallocations;
        queue_comparisons += run.work.queue_comparisons;
        work += run.work.total();
        naturally_quiescent &= run.naturally_quiescent;
        if first_deallocation_context < 0 && run.work.physical_deallocations > 0 {
            first_deallocation_context = context as isize;
        }
        let probe_tick = base + 7;
        let mut b_probe_fixture = fixture.clone();
        let mut a_probe_fixture = fixture.clone();
        let b_probe = d2_held_out(
            &mut b_probe_fixture,
            config.current,
            probe_tick,
            config.namespace + 0x10_000 + context as u64 * 0x100,
            config.reverse_allocation,
        );
        let a_probe = d2_held_out(
            &mut a_probe_fixture,
            config.initial,
            probe_tick,
            config.namespace + 0x20_000 + context as u64 * 0x100,
            !config.reverse_allocation,
        );
        diagnostic_probe_work += b_probe.work.total() + a_probe.work.total();
        let b_probe_effect = effects(&b_probe);
        let a_probe_effect = effects(&a_probe);
        if b_probe_effect == 1 {
            if first_b_execution_context < 0 {
                first_b_execution_context = context as isize;
            }
            b_executable_contexts += 1;
        }
        naturally_quiescent &= b_probe.naturally_quiescent && a_probe.naturally_quiescent;
        trajectory.push(D2Trajectory {
            cell: config.index,
            context,
            form,
            stable_opportunity: 1,
            stable_completed: stable_here,
            stable_completed_cumulative: stable_completed,
            sparse_opportunity,
            sparse_completed: sparse_here,
            sparse_completed_cumulative: sparse_completed,
            stable_resistance: max_live_resistance(&fixture, config.current),
            sparse_resistance: max_live_resistance(&fixture, config.initial),
            stable_live_arrows: live_variable_count(&fixture, config.current),
            sparse_live_arrows: live_variable_count(&fixture, config.initial),
            b_probe_effect,
            a_probe_effect,
            b_executable_contexts,
            deallocations: run.work.physical_deallocations,
            queue_comparisons: run.work.queue_comparisons,
            work: run.work.total(),
        });
        context += 1;
    }

    let stable_final_resistance = max_live_resistance(&fixture, config.current);
    let sparse_final_resistance = max_live_resistance(&fixture, config.initial);
    let final_tick = dense_start + context as i64 * config.spacing;
    let mut b_first_fixture = fixture.clone();
    let mut b_second_fixture = fixture.clone();
    let mut a_fixture = fixture.clone();
    let b_first = d2_held_out(
        &mut b_first_fixture,
        config.current,
        final_tick,
        config.namespace + 0x30_000,
        config.reverse_allocation,
    );
    let b_second = d2_held_out(
        &mut b_second_fixture,
        config.current,
        final_tick,
        config.namespace + 0x30_000,
        config.reverse_allocation,
    );
    let a_final = d2_held_out(
        &mut a_fixture,
        config.initial,
        final_tick,
        config.namespace + 0x40_000,
        !config.reverse_allocation,
    );
    diagnostic_probe_work += b_first.work.total() + b_second.work.total() + a_final.work.total();
    naturally_quiescent &=
        b_first.naturally_quiescent && b_second.naturally_quiescent && a_final.naturally_quiescent;
    let duplicate_exact = b_first == b_second
        && b_first_fixture.substrate.complete_fingerprint()
            == b_second_fixture.substrate.complete_fingerprint();
    let stable_deallocation_delay =
        route_deallocation_delay(fixture.clone(), config.current, final_tick);
    let sparse_deallocation_delay =
        route_deallocation_delay(fixture.clone(), config.initial, final_tick);

    (
        D2Cell {
            config,
            prefix_ok,
            stable_opportunities: context,
            stable_completed,
            sparse_opportunities,
            sparse_completed,
            stable_final_resistance,
            sparse_final_resistance,
            first_b_execution_context,
            b_executable_contexts,
            final_b_effects: effects(&b_first),
            final_a_effects: effects(&a_final),
            first_deallocation_context,
            stable_deallocation_delay,
            sparse_deallocation_delay,
            proposals,
            deallocations,
            queue_comparisons,
            work,
            diagnostic_probe_work,
            complete_fingerprint: fixture.substrate.complete_fingerprint(),
            permanent_fingerprint: fixture.substrate.permanent_fingerprint(),
            duplicate_exact,
            naturally_quiescent,
        },
        trajectory,
    )
}

fn run_d2(prefix: &Path) -> bool {
    let cell_path = prefix.with_extension("cells.csv");
    let trajectory_path = prefix.with_extension("trajectory.csv");
    let report_path = prefix.with_extension("md");
    if cell_path.exists() || trajectory_path.exists() || report_path.exists() {
        eprintln!("PX0-D2 write-once outputs already exist");
        return false;
    }
    let mut cells = Vec::new();
    let mut trajectories = Vec::new();
    for config in d2_configs() {
        let (cell, trajectory) = run_d2_cell(config);
        cells.push(cell);
        trajectories.extend(trajectory);
    }
    let integrity = cells.len() == 256
        && cells
            .iter()
            .all(|cell| cell.prefix_ok && cell.duplicate_exact && cell.naturally_quiescent)
        && trajectories.len() == 256 * 35;
    let accounting_gaps = cells
        .iter()
        .filter(|cell| cell.stable_completed < cell.stable_opportunities)
        .count();
    let behavior_breakdowns = cells
        .iter()
        .filter(|cell| cell.final_b_effects != 1 || cell.final_a_effects != 0)
        .count();
    let resistance_boundaries = cells
        .iter()
        .filter(|cell| {
            cell.final_b_effects == 1
                && (cell.stable_final_resistance <= cell.sparse_final_resistance
                    || cell.stable_deallocation_delay <= cell.sparse_deallocation_delay)
        })
        .count();
    let classification = if !integrity {
        "D2-E — SCIENTIFIC AMBIGUITY"
    } else if behavior_breakdowns > 0 {
        "D2-C — SPECIFICITY BREAKDOWN"
    } else if resistance_boundaries > 0 {
        "D2-B — RESISTANCE-SEPARATION BOUNDARY"
    } else if accounting_gaps > 0 {
        "D2-A — ACCOUNTING-ONLY BOUNDARY"
    } else {
        "D2-D — NO NEARBY BOUNDARY"
    };
    write_d2_results(
        &cell_path,
        &trajectory_path,
        &report_path,
        &cells,
        &trajectories,
        classification,
        accounting_gaps,
        resistance_boundaries,
        behavior_breakdowns,
        integrity,
    );
    integrity
}

#[allow(clippy::too_many_arguments)]
fn write_d2_results(
    cell_path: &Path,
    trajectory_path: &Path,
    report_path: &Path,
    cells: &[D2Cell],
    trajectories: &[D2Trajectory],
    classification: &str,
    accounting_gaps: usize,
    resistance_boundaries: usize,
    behavior_breakdowns: usize,
    integrity: bool,
) {
    let mut cell_csv = String::from(
        "index,namespace,initial,current,spacing,stride,distractor_load,incidental_form,reverse_allocation,mirrored_layout,device_order,stable_opportunities,stable_completed,sparse_opportunities,sparse_completed,stable_final_resistance,sparse_final_resistance,first_b_execution_context,b_executable_contexts,final_b_effects,final_a_effects,first_deallocation_context,stable_deallocation_delay,sparse_deallocation_delay,proposals,deallocations,queue_comparisons,work,diagnostic_probe_work,complete_fingerprint,permanent_fingerprint,prefix_ok,duplicate_exact,naturally_quiescent\n",
    );
    for cell in cells {
        let config = cell.config;
        let fields = vec![
            config.index.to_string(),
            format!("0x{:x}", config.namespace),
            config.initial.to_string(),
            config.current.to_string(),
            config.spacing.to_string(),
            config.stride.to_string(),
            config.distractor_load.to_string(),
            config.incidental_form.to_string(),
            config.reverse_allocation.to_string(),
            config.mirrored_layout.to_string(),
            format!(
                "{}-{}-{}-{}",
                config.device_order[0],
                config.device_order[1],
                config.device_order[2],
                config.device_order[3]
            ),
            cell.stable_opportunities.to_string(),
            cell.stable_completed.to_string(),
            cell.sparse_opportunities.to_string(),
            cell.sparse_completed.to_string(),
            cell.stable_final_resistance.to_string(),
            cell.sparse_final_resistance.to_string(),
            cell.first_b_execution_context.to_string(),
            cell.b_executable_contexts.to_string(),
            cell.final_b_effects.to_string(),
            cell.final_a_effects.to_string(),
            cell.first_deallocation_context.to_string(),
            cell.stable_deallocation_delay.to_string(),
            cell.sparse_deallocation_delay.to_string(),
            cell.proposals.to_string(),
            cell.deallocations.to_string(),
            cell.queue_comparisons.to_string(),
            cell.work.to_string(),
            cell.diagnostic_probe_work.to_string(),
            cell.complete_fingerprint.to_string(),
            cell.permanent_fingerprint.to_string(),
            cell.prefix_ok.to_string(),
            cell.duplicate_exact.to_string(),
            cell.naturally_quiescent.to_string(),
        ];
        cell_csv.push_str(&fields.join(","));
        cell_csv.push('\n');
    }
    let mut trajectory_csv = String::from(
        "cell,context,form,stable_opportunity,stable_completed,stable_completed_cumulative,sparse_opportunity,sparse_completed,sparse_completed_cumulative,stable_resistance,sparse_resistance,stable_live_arrows,sparse_live_arrows,b_probe_effect,a_probe_effect,b_executable_contexts,deallocations,queue_comparisons,work\n",
    );
    for row in trajectories {
        trajectory_csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.cell,
            row.context,
            row.form,
            row.stable_opportunity,
            row.stable_completed,
            row.stable_completed_cumulative,
            row.sparse_opportunity,
            row.sparse_completed,
            row.sparse_completed_cumulative,
            row.stable_resistance,
            row.sparse_resistance,
            row.stable_live_arrows,
            row.sparse_live_arrows,
            row.b_probe_effect,
            row.a_probe_effect,
            row.b_executable_contexts,
            row.deallocations,
            row.queue_comparisons,
            row.work,
        ));
    }
    let total_stable_opportunities = cells
        .iter()
        .map(|cell| cell.stable_opportunities)
        .sum::<usize>();
    let total_stable_completed = cells
        .iter()
        .map(|cell| cell.stable_completed)
        .sum::<usize>();
    let total_sparse_opportunities = cells
        .iter()
        .map(|cell| cell.sparse_opportunities)
        .sum::<usize>();
    let total_sparse_completed = cells
        .iter()
        .map(|cell| cell.sparse_completed)
        .sum::<usize>();
    let total_work = cells.iter().map(|cell| cell.work).sum::<u64>();
    let report = format!(
        "# PX0-D2 dense-corner diagnostic result\n\nClassification: **{classification}**.\n\n- cells: `{}`\n- trajectory rows: `{}`\n- stable opportunities/completed: `{}/{}`\n- sparse opportunities/completed: `{}/{}`\n- accounting-gap cells: `{}`\n- resistance-boundary cells: `{}`\n- behavioral-breakdown cells: `{}`\n- integrity: `{}`\n- main-path work: `{}`\n\nPX0 remains non-authoritative. No v3 authority execution is authorized.\n",
        cells.len(),
        trajectories.len(),
        total_stable_completed,
        total_stable_opportunities,
        total_sparse_completed,
        total_sparse_opportunities,
        accounting_gaps,
        resistance_boundaries,
        behavior_breakdowns,
        integrity,
        total_work,
    );
    atomic_write_new(cell_path, &cell_csv);
    atomic_write_new(trajectory_path, &trajectory_csv);
    atomic_write_new(report_path, &report);
}

fn atomic_write_new(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create D2 result directory");
    }
    let staging = path.with_extension(format!(
        "{}.staging",
        path.extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("result")
    ));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staging)
        .expect("create-new D2 staging artifact");
    file.write_all(contents.as_bytes())
        .expect("write D2 staging artifact");
    file.sync_all().expect("sync D2 staging artifact");
    fs::hard_link(&staging, path).expect("publish D2 artifact without replacement");
    fs::remove_file(&staging).expect("remove D2 staging link");
    File::open(path.parent().unwrap_or_else(|| Path::new(".")))
        .and_then(|directory| directory.sync_all())
        .expect("sync D2 result directory");
}

fn run_specificity_arm(arm: Arm) -> (ResultRow, Vec<ContextRow>) {
    let mut fixture = build(arm.namespace, arm.mirror, arm.stride, arm.distractor_load);
    let mut contexts = Vec::new();
    let mut stable_returns = 0;
    let mut incidental_returns = 0;
    let mut work = 0;
    let mut ordinal = 0;

    for _ in 0..arm.cycles {
        for form in 0..DEVICES {
            let base = ordinal as i64 * arm.context_spacing;
            let device = arm.device_order[form];
            let execution = run_context(
                &mut fixture,
                arm.stable,
                arm.incidental,
                device,
                form == arm.incidental_form,
                form == (arm.incidental_form + 3) % DEVICES,
                base,
                arm.namespace + 0x10_000 + ordinal as u64 * 0x100,
            );
            let stable_here = delayed_returns(&fixture, arm.stable, base, &execution);
            let incidental_here = delayed_returns(&fixture, arm.incidental, base, &execution);
            stable_returns += stable_here;
            incidental_returns += incidental_here;
            work += execution.work.total();
            contexts.push(ContextRow {
                arm: arm.name,
                ordinal,
                form,
                stable_device: device,
                stable_returns: stable_here,
                incidental_returns: incidental_here,
                crossings: effects(&execution),
                work: execution.work.total(),
            });
            assert!(execution.naturally_quiescent);
            ordinal += 1;
        }
    }
    for form in (0..DEVICES).filter(|form| *form != arm.incidental_form) {
        let base = ordinal as i64 * arm.context_spacing;
        let device = arm.device_order[form];
        let execution = run_context(
            &mut fixture,
            arm.stable,
            arm.incidental,
            device,
            false,
            form == (arm.incidental_form + 3) % DEVICES,
            base,
            arm.namespace + 0x20_000 + ordinal as u64 * 0x100,
        );
        let stable_here = delayed_returns(&fixture, arm.stable, base, &execution);
        let incidental_here = delayed_returns(&fixture, arm.incidental, base, &execution);
        stable_returns += stable_here;
        incidental_returns += incidental_here;
        work += execution.work.total();
        contexts.push(ContextRow {
            arm: arm.name,
            ordinal,
            form,
            stable_device: device,
            stable_returns: stable_here,
            incidental_returns: incidental_here,
            crossings: effects(&execution),
            work: execution.work.total(),
        });
        assert!(execution.naturally_quiescent);
        ordinal += 1;
    }

    let stable_max_resistance = max_live_resistance(&fixture, arm.stable);
    let incidental_final_max_resistance = max_live_resistance(&fixture, arm.incidental);
    let test_tick = ordinal as i64 * arm.context_spacing;
    let mut stable_first = fixture.clone();
    let mut stable_second = fixture.clone();
    let stable_execution = held_out(
        &mut stable_first,
        arm.stable,
        test_tick,
        arm.namespace + 0x30_000,
    );
    let stable_duplicate = held_out(
        &mut stable_second,
        arm.stable,
        test_tick,
        arm.namespace + 0x30_000,
    );
    let stable_duplicate_exact = stable_execution == stable_duplicate
        && stable_first.substrate.complete_fingerprint()
            == stable_second.substrate.complete_fingerprint();
    let mut incidental_first = fixture.clone();
    let mut incidental_second = fixture.clone();
    let incidental_execution = held_out(
        &mut incidental_first,
        arm.incidental,
        test_tick,
        arm.namespace + 0x40_000,
    );
    let incidental_duplicate = held_out(
        &mut incidental_second,
        arm.incidental,
        test_tick,
        arm.namespace + 0x40_000,
    );
    let incidental_duplicate_exact = incidental_execution == incidental_duplicate
        && incidental_first.substrate.complete_fingerprint()
            == incidental_second.substrate.complete_fingerprint();
    incidental_first.substrate.advance_time(test_tick + 200);
    let incidental_eventually_dead = variable_ids(&incidental_first, arm.incidental)
        .iter()
        .all(|arrow| !incidental_first.substrate.arrow_is_live(*arrow));
    let stable_devices_used = contexts
        .iter()
        .filter(|row| row.stable_returns > 0)
        .map(|row| row.stable_device)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let stable_effects = effects(&stable_execution);
    let incidental_effects = effects(&incidental_execution);
    let duplicate_exact = stable_duplicate_exact && incidental_duplicate_exact;
    let naturally_quiescent =
        stable_execution.naturally_quiescent && incidental_execution.naturally_quiescent;
    let passed = stable_returns == ordinal
        && incidental_returns > 0
        && incidental_returns < stable_returns
        && stable_devices_used == DEVICES
        && stable_max_resistance > incidental_final_max_resistance
        && stable_effects == 1
        && incidental_effects == 0
        && incidental_eventually_dead
        && duplicate_exact
        && naturally_quiescent;
    (
        ResultRow {
            arm: arm.name,
            stable: arm.stable,
            incidental: arm.incidental,
            stable_returns,
            incidental_returns,
            stable_devices_used,
            stable_max_resistance,
            incidental_final_max_resistance,
            stable_effects,
            incidental_effects,
            incidental_eventually_dead,
            duplicate_exact,
            naturally_quiescent,
            contexts: ordinal,
            work,
            persistent_bytes: fixture.substrate.persistent_bytes(),
            passed,
        },
        contexts,
    )
}

fn run_recurring_control(
    name: &'static str,
    namespace: u64,
    explicit: usize,
    dense: usize,
    mirror: bool,
) -> ControlRow {
    let mut fixture = build(namespace, mirror, STRIDE, 8);
    let mut route_0_returns = 0;
    let mut route_1_returns = 0;
    let mut work = 0;
    for ordinal in 0..24 {
        let base = ordinal as i64 * CONTEXT_SPACING;
        activate_candidates(
            &mut fixture,
            &[0, 1],
            base + 2,
            namespace + 0x10_000 + ordinal as u64 * 0x100,
        );
        let device = ordinal % DEVICES;
        activate_support(
            &mut fixture,
            explicit,
            device,
            base + 4,
            namespace + 0x20_000 + ordinal as u64,
        );
        activate_driver(
            &mut fixture,
            dense,
            base + 2,
            namespace + 0x30_000 + ordinal as u64 * 4,
        );
        let execution = fixture.substrate.propagate();
        route_0_returns += delayed_returns(&fixture, 0, base, &execution);
        route_1_returns += delayed_returns(&fixture, 1, base, &execution);
        work += execution.work.total();
        assert!(execution.naturally_quiescent);
    }
    let tick = 24 * CONTEXT_SPACING;
    let mut route_0_fixture = fixture.clone();
    let route_0 = held_out(&mut route_0_fixture, 0, tick, namespace + 0x40_000);
    let mut route_1_fixture = fixture.clone();
    let route_1 = held_out(&mut route_1_fixture, 1, tick, namespace + 0x50_000);
    let mut simultaneous_first = fixture.clone();
    let mut simultaneous_second = fixture.clone();
    activate_candidates(&mut simultaneous_first, &[0, 1], tick, namespace + 0x60_000);
    let simultaneous = simultaneous_first.substrate.propagate();
    activate_candidates(
        &mut simultaneous_second,
        &[0, 1],
        tick,
        namespace + 0x60_000,
    );
    let simultaneous_duplicate = simultaneous_second.substrate.propagate();
    let duplicate_exact = simultaneous == simultaneous_duplicate
        && simultaneous_first.substrate.complete_fingerprint()
            == simultaneous_second.substrate.complete_fingerprint();
    let route_0_effects = effects(&route_0);
    let route_1_effects = effects(&route_1);
    let simultaneous_effects = effects(&simultaneous);
    let naturally_quiescent = route_0.naturally_quiescent
        && route_1.naturally_quiescent
        && simultaneous.naturally_quiescent;
    let passed = route_0_returns > 0
        && route_1_returns > 0
        && route_0_effects == 1
        && route_1_effects == 1
        && simultaneous_effects == 0
        && duplicate_exact
        && naturally_quiescent;
    ControlRow {
        kind: "recurring",
        arm: name,
        route_0_returns,
        route_1_returns,
        route_0_effects,
        route_1_effects,
        simultaneous_effects,
        duplicate_exact,
        naturally_quiescent,
        work,
        passed,
    }
}

fn run_absent_control(name: &'static str, namespace: u64, mirror: bool) -> ControlRow {
    let mut fixture = build(namespace, mirror, STRIDE, 8);
    let mut work = 0;
    for ordinal in 0..24 {
        let tick = ordinal as i64 * CONTEXT_SPACING + 2;
        let execution = held_out(
            &mut fixture,
            ordinal % 2,
            tick,
            namespace + 0x10_000 + ordinal as u64 * 0x100,
        );
        work += execution.work.total();
        assert!(execution.naturally_quiescent);
    }
    let tick = 24 * CONTEXT_SPACING;
    let mut route_0_first = fixture.clone();
    let mut route_0_second = fixture.clone();
    let route_0 = held_out(&mut route_0_first, 0, tick, namespace + 0x20_000);
    let route_0_duplicate = held_out(&mut route_0_second, 0, tick, namespace + 0x20_000);
    let mut route_1_fixture = fixture.clone();
    let route_1 = held_out(&mut route_1_fixture, 1, tick, namespace + 0x30_000);
    let duplicate_exact = route_0 == route_0_duplicate
        && route_0_first.substrate.complete_fingerprint()
            == route_0_second.substrate.complete_fingerprint();
    let route_0_effects = effects(&route_0);
    let route_1_effects = effects(&route_1);
    let naturally_quiescent = route_0.naturally_quiescent && route_1.naturally_quiescent;
    let passed = route_0_effects == 0
        && route_1_effects == 0
        && max_live_resistance(&fixture, 0) <= 1
        && max_live_resistance(&fixture, 1) <= 1
        && duplicate_exact
        && naturally_quiescent;
    ControlRow {
        kind: "absent",
        arm: name,
        route_0_returns: 0,
        route_1_returns: 0,
        route_0_effects,
        route_1_effects,
        simultaneous_effects: 0,
        duplicate_exact,
        naturally_quiescent,
        work,
        passed,
    }
}

fn run_switch_control(name: &'static str, namespace: u64, mirror: bool) -> ControlRow {
    let mut fixture = build(namespace, mirror, 18, 12);
    let mut route_0_returns = 0;
    let mut route_1_returns = 0;
    let mut work = 0;
    let mut ordinal = 0;
    for _ in 0..8 {
        for form in 0..DEVICES {
            let base = ordinal * CONTEXT_SPACING;
            let execution = run_context(
                &mut fixture,
                0,
                1,
                form,
                form == 0,
                form == 3,
                base,
                namespace + 0x10_000 + ordinal as u64 * 0x100,
            );
            route_0_returns += delayed_returns(&fixture, 0, base, &execution);
            route_1_returns += delayed_returns(&fixture, 1, base, &execution);
            work += execution.work.total();
            ordinal += 1;
        }
    }
    let switched_while_both_live =
        max_live_resistance(&fixture, 0) > 1 && max_live_resistance(&fixture, 1) > 0;
    for _ in 0..80 {
        for form in 0..DEVICES {
            let base = ordinal * CONTEXT_SPACING;
            let execution = run_context(
                &mut fixture,
                1,
                0,
                form,
                form == 0,
                form == 3,
                base,
                namespace + 0x20_000 + ordinal as u64 * 0x100,
            );
            route_0_returns += delayed_returns(&fixture, 0, base, &execution);
            route_1_returns += delayed_returns(&fixture, 1, base, &execution);
            work += execution.work.total();
            ordinal += 1;
        }
    }
    for form in 1..DEVICES {
        let base = ordinal * CONTEXT_SPACING;
        let execution = run_context(
            &mut fixture,
            1,
            0,
            form,
            false,
            form == 3,
            base,
            namespace + 0x30_000 + ordinal as u64 * 0x100,
        );
        route_0_returns += delayed_returns(&fixture, 0, base, &execution);
        route_1_returns += delayed_returns(&fixture, 1, base, &execution);
        work += execution.work.total();
        ordinal += 1;
    }
    let tick = ordinal * CONTEXT_SPACING;
    let mut route_0_fixture = fixture.clone();
    let mut route_1_first = fixture.clone();
    let mut route_1_second = fixture.clone();
    let route_0 = held_out(&mut route_0_fixture, 0, tick, namespace + 0x40_000);
    let route_1 = held_out(&mut route_1_first, 1, tick, namespace + 0x50_000);
    let route_1_duplicate = held_out(&mut route_1_second, 1, tick, namespace + 0x50_000);
    let route_0_effects = effects(&route_0);
    let route_1_effects = effects(&route_1);
    let duplicate_exact = route_1 == route_1_duplicate
        && route_1_first.substrate.complete_fingerprint()
            == route_1_second.substrate.complete_fingerprint();
    let naturally_quiescent = route_0.naturally_quiescent && route_1.naturally_quiescent;
    let passed = switched_while_both_live
        && route_0_effects == 0
        && route_1_effects == 1
        && max_live_resistance(&fixture, 1) > max_live_resistance(&fixture, 0)
        && duplicate_exact
        && naturally_quiescent;
    ControlRow {
        kind: "switch",
        arm: name,
        route_0_returns,
        route_1_returns,
        route_0_effects,
        route_1_effects,
        simultaneous_effects: 0,
        duplicate_exact,
        naturally_quiescent,
        work,
        passed,
    }
}

fn write_results(
    prefix: &Path,
    results: &[ResultRow],
    controls: &[ControlRow],
    contexts: &[ContextRow],
    passed: bool,
    mode: Mode,
) {
    let summary_path = prefix.with_extension("csv");
    let context_path = prefix.with_extension("contexts.csv");
    let report_path = prefix.with_extension("md");
    if let Some(parent) = summary_path.parent() {
        fs::create_dir_all(parent).expect("create result directory");
    }
    let mut summary = String::from(
        "kind,arm,stable,incidental,stable_returns,incidental_returns,stable_devices_used,stable_max_resistance,incidental_final_max_resistance,stable_effects,incidental_effects,incidental_eventually_dead,duplicate_exact,naturally_quiescent,contexts,work,persistent_bytes,passed\n",
    );
    for row in results {
        summary.push_str(&format!(
            "specificity,{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.arm,
            row.stable,
            row.incidental,
            row.stable_returns,
            row.incidental_returns,
            row.stable_devices_used,
            row.stable_max_resistance,
            row.incidental_final_max_resistance,
            row.stable_effects,
            row.incidental_effects,
            row.incidental_eventually_dead,
            row.duplicate_exact,
            row.naturally_quiescent,
            row.contexts,
            row.work,
            row.persistent_bytes,
            row.passed,
        ));
    }
    for row in controls {
        summary.push_str(&format!(
            "{},{},{},{},{},{},0,0,0,{},{},false,{},{},24,{},0,{}\n",
            row.kind,
            row.arm,
            0,
            1,
            row.route_0_returns,
            row.route_1_returns,
            row.route_0_effects,
            row.route_1_effects,
            row.duplicate_exact,
            row.naturally_quiescent,
            row.work,
            row.passed,
        ));
    }
    let mut context_csv = String::from(
        "arm,ordinal,form,stable_device,stable_returns,incidental_returns,crossings,work\n",
    );
    for row in contexts {
        context_csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            row.arm,
            row.ordinal,
            row.form,
            row.stable_device,
            row.stable_returns,
            row.incidental_returns,
            row.crossings,
            row.work,
        ));
    }
    let mut report = format!(
        "# PX0-S stable return specificity {}\n\nOutcome: **{}**.\n\n",
        match mode {
            Mode::Probe => "PROBE retry v2",
            Mode::Micro => "MICRO v1",
            Mode::Gate => "GATE v1",
            Mode::D2 => unreachable!("D2 uses its own report"),
        },
        if passed {
            "PX0-S-A — STABLE RETURN SPECIFICITY POSITIVE"
        } else {
            "TARGET NOT MET"
        }
    );
    report.push_str("| arm | stable route | sparse route | stable returns | sparse returns | devices | stable R | sparse final R | stable effect | sparse effect | sparse dies | replay | pass |\n");
    report.push_str("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in results {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.arm,
            row.stable,
            row.incidental,
            row.stable_returns,
            row.incidental_returns,
            row.stable_devices_used,
            row.stable_max_resistance,
            row.incidental_final_max_resistance,
            row.stable_effects,
            row.incidental_effects,
            row.incidental_eventually_dead,
            row.duplicate_exact,
            row.passed,
        ));
    }
    report.push_str("\n## Dense-path and absent-return controls\n\n");
    report.push_str("| arm | route 0 returns | route 1 returns | route 0 effect | route 1 effect | simultaneous | replay | pass |\n");
    report.push_str("|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in controls {
        report.push_str(&format!(
            "| {}:{} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.kind,
            row.arm,
            row.route_0_returns,
            row.route_1_returns,
            row.route_0_effects,
            row.route_1_effects,
            row.simultaneous_effects,
            row.duplicate_exact,
            row.passed,
        ));
    }
    atomic_write(&summary_path, &summary);
    atomic_write(&context_path, &context_csv);
    atomic_write(&report_path, &report);
}

fn atomic_write(path: &Path, contents: &str) {
    let temporary = path.with_extension(format!(
        "{}.tmp",
        path.extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("result")
    ));
    fs::write(&temporary, contents).expect("write temporary result");
    fs::rename(&temporary, path).expect("atomically install result");
}
