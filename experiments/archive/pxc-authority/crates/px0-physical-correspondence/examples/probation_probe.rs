use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const N: usize = 3;
const SCAFFOLD: u32 = 1_000;

#[derive(Clone)]
struct Fixture {
    substrate: PlasticSubstrate,
    sources: [CellId; N],
    probes: [CellId; N],
    contenders: [CellId; N],
    backgrounds: [CellId; N],
    supports: [CellId; N],
}

#[derive(Clone, Debug)]
struct Arm {
    name: &'static str,
    namespace: u64,
    initial: usize,
    current: usize,
    spacing: i64,
    active_count: usize,
    mirror: bool,
    stride: i32,
}

#[derive(Clone, Debug)]
struct ResultRow {
    arm: &'static str,
    old_dead: bool,
    fresh_initial_created: bool,
    fresh_current: usize,
    current_effects: usize,
    unsupported_effects: usize,
    unsupported_first_crossing_tick: i64,
    unsupported_trace_entries: usize,
    unsupported_firings: usize,
    initial_gate_firings: usize,
    initial_delayed_returns: usize,
    initial_max_candidate_impulse: i32,
    initial_max_resistance: u32,
    initial_eventually_dead: bool,
    current_max_candidate_impulse: i32,
    duplicate_exact: bool,
    naturally_quiescent: bool,
    passed_target: bool,
}

#[derive(Clone, Debug)]
struct TraceRow {
    arm: &'static str,
    stage: String,
    ordinal: usize,
    tick: i64,
    target_physical: u64,
    impulse: i32,
    fired: bool,
}

fn main() {
    let mut args = env::args().skip(1);
    let mut output_prefix = None;
    let mut return_free = false;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--output-prefix" => {
                output_prefix = Some(PathBuf::from(
                    args.next().expect("--output-prefix requires a path"),
                ));
            }
            "--return-free" => return_free = true,
            "--definitive" => {
                eprintln!("PX0-P is development-only; definitive execution is forbidden");
                std::process::exit(2);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    let base_namespace = if return_free { 0x940_0000 } else { 0x920_0000 };
    let route_stride = if return_free { 8 } else { 6 };
    let arms = [
        Arm {
            name: "fresh-short-two-direct",
            namespace: base_namespace,
            initial: 0,
            current: 1,
            spacing: 8,
            active_count: 2,
            mirror: false,
            stride: route_stride,
        },
        Arm {
            name: "fresh-threshold-three-mirror",
            namespace: base_namespace + 0x4_0000,
            initial: 1,
            current: 2,
            spacing: 10,
            active_count: 3,
            mirror: true,
            stride: route_stride,
        },
    ];
    let mut results = Vec::new();
    let mut traces = Vec::new();
    for arm in &arms {
        let (result, trace) = run_arm(arm, return_free);
        results.push(result);
        traces.extend(trace);
    }
    let passed_target = results.iter().all(|row| row.passed_target);
    write_results(
        &output_prefix.expect("development output prefix is required"),
        &results,
        &traces,
        passed_target,
        return_free,
    );
    if return_free {
        if !passed_target {
            std::process::exit(1);
        }
    } else if passed_target {
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

fn build(namespace: u64, mirror: bool, stride: i32) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; N];
    let mut probes = [None; N];
    let mut contenders = [None; N];
    let mut gates = [None; N];
    let mut backgrounds = [None; N];
    let mut supports = [None; N];
    for index in 0..N {
        let slot = if mirror { N - 1 - index } else { index };
        let position = slot as i32 * stride;
        sources[index] =
            Some(substrate.add_cell(cell(namespace + 10 + index as u64, position, 0, 2)));
        probes[index] =
            Some(substrate.add_cell(cell(namespace + 20 + index as u64, position + 1, -1, 2)));
        contenders[index] =
            Some(substrate.add_cell(cell(namespace + 30 + index as u64, position + 2, 0, 2)));
        gates[index] =
            Some(substrate.add_cell(cell(namespace + 40 + index as u64, position + 4, -1, 2)));
        backgrounds[index] = Some(substrate.add_cell(cell(
            namespace + 50 + index as u64,
            2_000 + index as i32 * 10,
            -2,
            1,
        )));
        supports[index] = Some(substrate.add_cell(cell(
            namespace + 60 + index as u64,
            3_000 + index as i32 * 10,
            -2,
            1,
        )));
    }
    let sources = sources.map(|value| value.expect("source allocated"));
    let probes = probes.map(|value| value.expect("probe allocated"));
    let contenders = contenders.map(|value| value.expect("contender allocated"));
    let gates = gates.map(|value| value.expect("gate allocated"));
    let backgrounds = backgrounds.map(|value| value.expect("background allocated"));
    let supports = supports.map(|value| value.expect("support allocated"));
    let veto = substrate.add_cell(cell(namespace + 100, 1_000, 0, 2));
    let accumulator = substrate.add_cell(cell(namespace + 101, 1_001, 0, 1));
    let outside = substrate.add_cell(cell(namespace + 102, 1_002, 1, 1));
    for index in 0..N {
        substrate.add_arrow(arrow(probes[index], gates[index], 1, 1));
        substrate.add_arrow(arrow(gates[index], sources[index], 1, 1));
        substrate.add_arrow(arrow(backgrounds[index], probes[index], 1, 1));
        substrate.add_arrow(arrow(supports[index], gates[index], 2, 1));
        substrate.add_arrow(arrow(contenders[index], veto, 0, 1));
        substrate.add_arrow(arrow(contenders[index], accumulator, 2, 1));
    }
    substrate.add_arrow(arrow(veto, accumulator, 1, -4));
    substrate.add_arrow(arrow(accumulator, outside, 0, 1));
    Fixture {
        substrate,
        sources,
        probes,
        contenders,
        backgrounds,
        supports,
    }
}

fn experience(
    fixture: &mut Fixture,
    active: &[usize],
    supported: &[usize],
    tick: i64,
    namespace: u64,
    reverse_arrival: bool,
) -> Execution {
    for index in active {
        let phases = if reverse_arrival { [1, 0] } else { [0, 1] };
        for (ordinal, phase) in phases.into_iter().enumerate() {
            fixture.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase,
                origin_physical: namespace + (*index as u64) * 8 + ordinal as u64,
                target: fixture.sources[*index],
                impulse: 1,
            });
        }
        fixture.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: namespace + 0x100 + *index as u64,
            target: fixture.backgrounds[*index],
            impulse: 1,
        });
    }
    for index in supported {
        fixture.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: namespace + 0x200 + *index as u64,
            target: fixture.supports[*index],
            impulse: 1,
        });
    }
    fixture.substrate.propagate()
}

fn effects(execution: &Execution) -> usize {
    execution
        .crossings
        .iter()
        .filter(|crossing| crossing.from_region == 0 && crossing.to_region == 1)
        .count()
}

fn variable_ids(fixture: &Fixture, index: usize) -> Vec<ArrowId> {
    let mut ids = fixture
        .substrate
        .arrows_between(fixture.sources[index], fixture.probes[index]);
    ids.extend(
        fixture
            .substrate
            .arrows_between(fixture.sources[index], fixture.contenders[index]),
    );
    ids
}

fn run_arm(arm: &Arm, return_free: bool) -> (ResultRow, Vec<TraceRow>) {
    let spare = (arm.current + 1) % N;
    let active = if arm.active_count == 2 {
        vec![arm.initial, arm.current]
    } else {
        vec![arm.initial, arm.current, spare]
    };
    let mut fixture = build(arm.namespace, arm.mirror, arm.stride);
    let mut trace = Vec::new();
    for ordinal in 0..4 {
        experience(
            &mut fixture,
            &active,
            &[arm.initial],
            ordinal as i64 * arm.spacing,
            arm.namespace + 0x1_000 + ordinal as u64 * 0x20,
            ordinal % 2 == 1,
        );
    }
    let old_ids = variable_ids(&fixture, arm.initial);
    experience(
        &mut fixture,
        &[arm.initial],
        &[arm.initial],
        4 * arm.spacing,
        arm.namespace + 0x2_000,
        true,
    );
    let survival_tick = 4 * arm.spacing + 20;
    fixture.substrate.advance_time(survival_tick);
    experience(
        &mut fixture,
        &[arm.initial],
        &[arm.initial],
        survival_tick,
        arm.namespace + 0x2_800,
        false,
    );
    fixture.substrate.advance_time(300);
    let old_dead = old_ids.iter().all(|arrow| {
        !fixture.substrate.arrow_is_live(*arrow) && fixture.substrate.arrow_resistance(*arrow) == 0
    });
    let first_renewal = experience(
        &mut fixture,
        &active,
        &[arm.current],
        300,
        arm.namespace + 0x3_000,
        true,
    );
    let fresh_initial_created = variable_ids(&fixture, arm.initial)
        .iter()
        .any(|arrow| old_ids.iter().all(|old| old != arrow));
    append_trace(arm.name, "renewal-0", &first_renewal, &mut trace);
    let mut renewal_executions = vec![(300, first_renewal.clone())];
    for ordinal in 0..4 {
        let renewal = experience(
            &mut fixture,
            &active,
            &[arm.current],
            300 + (ordinal as i64 + 1) * arm.spacing,
            arm.namespace + 0x4_000 + ordinal as u64 * 0x20,
            ordinal % 2 == 1,
        );
        append_trace(
            arm.name,
            &format!("renewal-{}", ordinal + 1),
            &renewal,
            &mut trace,
        );
        renewal_executions.push((300 + (ordinal as i64 + 1) * arm.spacing, renewal));
    }
    let initial_source = arm.namespace + 10 + arm.initial as u64;
    let initial_probe = arm.namespace + 20 + arm.initial as u64;
    let initial_contender = arm.namespace + 30 + arm.initial as u64;
    let initial_gate = arm.namespace + 40 + arm.initial as u64;
    let initial_gate_firings = renewal_executions
        .iter()
        .flat_map(|(_, execution)| &execution.trace)
        .filter(|entry| entry.target_physical == initial_gate && entry.fired)
        .count();
    let initial_delayed_returns = renewal_executions
        .iter()
        .map(|(presentation_tick, execution)| {
            execution
                .trace
                .iter()
                .filter(|entry| {
                    entry.target_physical == initial_source && entry.tick > *presentation_tick
                })
                .count()
        })
        .sum::<usize>();
    let initial_max_candidate_impulse = renewal_executions
        .iter()
        .flat_map(|(_, execution)| &execution.trace)
        .filter(|entry| {
            entry.target_physical == initial_probe || entry.target_physical == initial_contender
        })
        .map(|entry| entry.impulse)
        .max()
        .unwrap_or(0);
    let fresh_initial_ids = variable_ids(&fixture, arm.initial)
        .into_iter()
        .filter(|arrow| old_ids.iter().all(|old| old != arrow))
        .collect::<Vec<_>>();
    let initial_max_resistance = fresh_initial_ids
        .iter()
        .map(|arrow| fixture.substrate.arrow_resistance(*arrow))
        .max()
        .unwrap_or(0);
    let current = experience(
        &mut fixture,
        &[arm.current],
        &[arm.current],
        300 + 5 * arm.spacing,
        arm.namespace + 0x5_000,
        false,
    );
    let current_probe = arm.namespace + 20 + arm.current as u64;
    let current_contender = arm.namespace + 30 + arm.current as u64;
    let current_max_candidate_impulse = current
        .trace
        .iter()
        .filter(|entry| {
            entry.target_physical == current_probe || entry.target_physical == current_contender
        })
        .map(|entry| entry.impulse)
        .max()
        .unwrap_or(0);
    append_trace(arm.name, "current-held-out", &current, &mut trace);
    let mut unsupported_first = fixture.clone();
    let mut unsupported_second = fixture.clone();
    let unsupported = experience(
        &mut unsupported_first,
        &[arm.initial],
        &[],
        300 + 6 * arm.spacing,
        arm.namespace + 0x6_000,
        true,
    );
    let unsupported_duplicate = experience(
        &mut unsupported_second,
        &[arm.initial],
        &[],
        300 + 6 * arm.spacing,
        arm.namespace + 0x6_000,
        true,
    );
    let duplicate_exact = unsupported == unsupported_duplicate
        && unsupported_first.substrate.complete_fingerprint()
            == unsupported_second.substrate.complete_fingerprint();
    let unsupported_trace_entries = unsupported.trace.len();
    let unsupported_firings = unsupported.trace.iter().filter(|entry| entry.fired).count();
    append_trace(arm.name, "unsupported-held-out", &unsupported, &mut trace);
    let unsupported_effects = effects(&unsupported);
    let first_crossing_tick = unsupported
        .crossings
        .iter()
        .find(|crossing| crossing.from_region == 0 && crossing.to_region == 1)
        .map_or(-1, |crossing| crossing.tick);
    let fresh_current = variable_ids(&unsupported_first, arm.current)
        .into_iter()
        .filter(|arrow| fixture.substrate.arrow_is_live(*arrow))
        .count();
    unsupported_first.substrate.advance_time(500);
    let initial_eventually_dead = variable_ids(&unsupported_first, arm.initial)
        .iter()
        .all(|arrow| !unsupported_first.substrate.arrow_is_live(*arrow));
    let passed_target = if return_free {
        old_dead
            && fresh_initial_created
            && fresh_current == 2
            && effects(&current) == 1
            && unsupported_effects == 0
            && initial_gate_firings == 0
            && initial_delayed_returns == 0
            && initial_max_candidate_impulse <= 1
            && initial_max_resistance <= 1
            && initial_eventually_dead
            && current_max_candidate_impulse == 2
            && duplicate_exact
            && current.naturally_quiescent
            && unsupported.naturally_quiescent
    } else {
        old_dead
            && effects(&current) == 1
            && unsupported_effects == 1
            && current.naturally_quiescent
            && unsupported.naturally_quiescent
    };
    (
        ResultRow {
            arm: arm.name,
            old_dead,
            fresh_initial_created,
            fresh_current,
            current_effects: effects(&current),
            unsupported_effects,
            unsupported_first_crossing_tick: first_crossing_tick,
            unsupported_trace_entries,
            unsupported_firings,
            initial_gate_firings,
            initial_delayed_returns,
            initial_max_candidate_impulse,
            initial_max_resistance,
            initial_eventually_dead,
            current_max_candidate_impulse,
            duplicate_exact,
            naturally_quiescent: current.naturally_quiescent && unsupported.naturally_quiescent,
            passed_target,
        },
        trace,
    )
}

fn append_trace(arm: &'static str, stage: &str, execution: &Execution, rows: &mut Vec<TraceRow>) {
    rows.extend(
        execution
            .trace
            .iter()
            .enumerate()
            .map(|(ordinal, entry)| TraceRow {
                arm,
                stage: stage.to_string(),
                ordinal,
                tick: entry.tick,
                target_physical: entry.target_physical,
                impulse: entry.impulse,
                fired: entry.fired,
            }),
    );
}

fn write_results(
    prefix: &Path,
    results: &[ResultRow],
    traces: &[TraceRow],
    passed_target: bool,
    return_free: bool,
) {
    let summary_csv = prefix.with_extension("csv");
    let trace_csv = prefix.with_extension("trace.csv");
    let markdown = prefix.with_extension("md");
    if let Some(parent) = summary_csv.parent() {
        fs::create_dir_all(parent).expect("create result directory");
    }
    let mut summary = String::from(
        "arm,old_dead,fresh_initial_created,fresh_current,current_effects,unsupported_effects,unsupported_first_crossing_tick,unsupported_trace_entries,unsupported_firings,initial_gate_firings,initial_delayed_returns,initial_max_candidate_impulse,initial_max_resistance,initial_eventually_dead,current_max_candidate_impulse,duplicate_exact,naturally_quiescent,passed_target\n",
    );
    for row in results {
        summary.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.arm,
            row.old_dead,
            row.fresh_initial_created,
            row.fresh_current,
            row.current_effects,
            row.unsupported_effects,
            row.unsupported_first_crossing_tick,
            row.unsupported_trace_entries,
            row.unsupported_firings,
            row.initial_gate_firings,
            row.initial_delayed_returns,
            row.initial_max_candidate_impulse,
            row.initial_max_resistance,
            row.initial_eventually_dead,
            row.current_max_candidate_impulse,
            row.duplicate_exact,
            row.naturally_quiescent,
            row.passed_target,
        ));
    }
    let mut trace = String::from("arm,stage,ordinal,tick,target_physical,impulse,fired\n");
    for row in traces {
        trace.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            row.arm, row.stage, row.ordinal, row.tick, row.target_physical, row.impulse, row.fired
        ));
    }
    let mut report = format!(
        "# {}\n\nOutcome: **{}**.\n\n",
        if return_free {
            "PX0-P1 return-free proposal control PROBE v1"
        } else {
            "PX0-P physical proposal probation baseline PROBE v1"
        },
        if passed_target {
            if return_free {
                "P1-A — EXISTING PHYSICAL PROBATION POSITIVE"
            } else {
                "FROZEN NEGATIVE REPRODUCED"
            }
        } else {
            "TARGET NOT MET"
        }
    );
    report.push_str("| arm | old dead | fresh A | A gate fires | A returns | A max impulse | A max resistance | A effect | A dies | B max impulse | B effect | duplicate | quiescent | pass |\n");
    report.push_str("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in results {
        report.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.arm,
            row.old_dead,
            row.fresh_initial_created,
            row.initial_gate_firings,
            row.initial_delayed_returns,
            row.initial_max_candidate_impulse,
            row.initial_max_resistance,
            row.unsupported_effects,
            row.initial_eventually_dead,
            row.current_max_candidate_impulse,
            row.current_effects,
            row.duplicate_exact,
            row.naturally_quiescent,
            row.passed_target,
        ));
    }
    atomic_write(&summary_csv, &summary);
    atomic_write(&trace_csv, &trace);
    atomic_write(&markdown, &report);
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
