use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs;
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
            "--definitive" => {
                eprintln!("PX0-S is development-only; definitive execution is forbidden");
                std::process::exit(2);
            }
            other => panic!("unknown argument: {other}"),
        }
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
    write_results(
        &output_prefix.expect("development output prefix is required"),
        &results,
        &controls,
        &contexts,
        passed,
        mode,
    );
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
