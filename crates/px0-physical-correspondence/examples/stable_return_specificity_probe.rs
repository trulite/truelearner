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
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--output-prefix" => {
                output_prefix = Some(PathBuf::from(
                    args.next().expect("--output-prefix requires a path"),
                ));
            }
            "--definitive" => {
                eprintln!("PX0-S is development-only; definitive execution is forbidden");
                std::process::exit(2);
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    let arms = [
        Arm {
            name: "stable-route-1-direct",
            namespace: 0xb00_0000,
            stable: 1,
            incidental: 0,
            mirror: false,
        },
        Arm {
            name: "stable-route-0-mirrored",
            namespace: 0xb40_0000,
            stable: 0,
            incidental: 1,
            mirror: true,
        },
    ];
    let mut results = Vec::new();
    let mut contexts = Vec::new();
    for arm in arms {
        let (result, rows) = run_specificity_arm(arm);
        results.push(result);
        contexts.extend(rows);
    }
    let controls = [
        run_recurring_control("recurring-route-0-direct", 0xb80_0000, 0, 1, false),
        run_recurring_control("recurring-route-1-mirrored", 0xbc0_0000, 1, 0, true),
    ];
    let passed = results.iter().all(|row| row.passed) && controls.iter().all(|row| row.passed);
    write_results(
        &output_prefix.expect("development output prefix is required"),
        &results,
        &controls,
        &contexts,
        passed,
    );
    if !passed {
        std::process::exit(1);
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

fn build(namespace: u64, mirror: bool) -> Fixture {
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
        let position = slot as i32 * STRIDE;
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

fn run_context(
    fixture: &mut Fixture,
    stable: usize,
    incidental: usize,
    form: usize,
    base: i64,
    origin: u64,
) -> Execution {
    let spare = 3 - stable - incidental;
    let mut active = vec![stable, incidental];
    if form == 3 {
        active.push(spare);
    }
    activate_candidates(fixture, &active, base + 2, origin);
    activate_support(fixture, stable, form, base + 4, origin + 0x200);
    if form == 0 {
        activate_driver(fixture, incidental, base + 2, origin + 0x300);
    }
    if form == 3 {
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
    let mut fixture = build(arm.namespace, arm.mirror);
    let mut contexts = Vec::new();
    let mut stable_returns = 0;
    let mut incidental_returns = 0;
    let mut work = 0;
    let mut ordinal = 0;

    for _ in 0..CYCLES {
        for form in 0..DEVICES {
            let base = ordinal as i64 * CONTEXT_SPACING;
            let execution = run_context(
                &mut fixture,
                arm.stable,
                arm.incidental,
                form,
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
                stable_device: form,
                stable_returns: stable_here,
                incidental_returns: incidental_here,
                crossings: effects(&execution),
                work: execution.work.total(),
            });
            assert!(execution.naturally_quiescent);
            ordinal += 1;
        }
    }
    for form in 1..DEVICES {
        let base = ordinal as i64 * CONTEXT_SPACING;
        let execution = run_context(
            &mut fixture,
            arm.stable,
            arm.incidental,
            form,
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
            stable_device: form,
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
    let test_tick = ordinal as i64 * CONTEXT_SPACING;
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
    let mut fixture = build(namespace, mirror);
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

fn write_results(
    prefix: &Path,
    results: &[ResultRow],
    controls: &[ControlRow],
    contexts: &[ContextRow],
    passed: bool,
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
            "recurring,{},{},{},{},{},0,0,0,{},{},false,{},{},24,{},0,{}\n",
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
        "# PX0-S stable return specificity PROBE retry v2\n\nOutcome: **{}**.\n\n",
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
    report.push_str("\n## Recurring dense-path controls\n\n");
    report.push_str("| arm | route 0 returns | route 1 returns | route 0 effect | route 1 effect | simultaneous | replay | pass |\n");
    report.push_str("|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in controls {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
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
