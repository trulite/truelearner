use cj0_d_local_subunit::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const FROZEN_PARENT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const AUTHORITY_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PROTOCOL_SHA256: &str = "940dc88e8a3f70c9dd7a9bc0eb42b1367273d4caea5bed50112bdc4ffce5d195";
const RETRY_PROTOCOL_SHA256: &str =
    "a4c4d3c7e0f1e3d5998b108e0f7225ebabe434d9246a16ebb4c0df28f83e9aa3";
const PROTOCOL: &str = "experiments/cj0_d_local_subunit_development_protocol_v1.md";
const RETRY_PROTOCOL: &str =
    "experiments/cj0_d_local_subunit_probe_v2_timing_correction_protocol.md";
const AUTHORITY: &str = "crates/px0-physical-correspondence/src/lib.rs";

const ROUTES: usize = 4;
const SITES: usize = 6;
const PORTS: usize = 12;
const SITE_ROUTES: [[usize; 2]; SITES] = [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]];
const OLD_SITES: [usize; 2] = [0, 5];
const NEW_SITES: [usize; 2] = [2, 3];
const OLD_PARTITION: [[usize; 2]; 2] = [[0, 1], [2, 3]];
const NEW_PARTITION: [[usize; 2]; 2] = [[0, 3], [2, 1]];

type TimedRoutes = Vec<(i64, Vec<usize>)>;

#[derive(Clone, Copy)]
struct Fixture {
    namespace: u64,
    mirror: bool,
    reverse_allocation: bool,
    reverse_identity: bool,
    reverse_insertion: bool,
    returns: bool,
    weak: bool,
    spacing: i32,
}

#[derive(Clone)]
struct Matter {
    substrate: PlasticSubstrate,
    namespace: u64,
    sources: [CellId; PORTS],
    source_physical: [u64; PORTS],
    locals: [CellId; SITES],
    initial_inputs: [ArrowId; PORTS],
    noise: CellId,
    reverse_insertion: bool,
    schedule_floors: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Counts {
    source_firings: [usize; PORTS],
    traversals: [usize; SITES],
    local_events: [usize; SITES],
    local_firings: [usize; SITES],
    returned_arrivals: [usize; PORTS],
    outward: [usize; SITES],
    noise_firings: usize,
    quiescent: bool,
    work: WorkLedger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    resistance: [u32; PORTS],
    coupling: [i32; PORTS],
    live: [bool; PORTS],
    local_state: [i32; SITES],
    arrow_count: usize,
    persistent_bytes: usize,
    permanent_fingerprint: u64,
    complete_fingerprint: u64,
    schedule_floors: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StageRow {
    case: String,
    namespace: u64,
    matched: bool,
    trained_out: [usize; 2],
    crossed_out: [usize; 2],
    singleton_out: [usize; ROUTES],
    counts: Counts,
    state: State,
    duplicate_exact: bool,
    pass: bool,
    note: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PartitionOutcome {
    training: Counts,
    state: State,
    trained_out: [usize; 2],
    crossed_out: [usize; 2],
    singleton_out: [usize; ROUTES],
    trained_events: [usize; 2],
    crossed_events: [usize; 2],
    source_refiring: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let mode = match args.as_slice() {
        [arg] if arg == "--preflight" => "preflight",
        [arg] if arg == "--probe" => "probe",
        [arg] if arg == "--micro" => "micro",
        [arg] if arg == "--gate" => "gate",
        _ => {
            eprintln!("CJ0-D experiment requires --preflight, --probe, --micro, or --gate");
            std::process::exit(2);
        }
    };
    assert!(
        source_audit(),
        "frozen source/protocol/ancestry audit failed"
    );
    if mode == "preflight" {
        for stage in ["probe", "micro", "gate"] {
            assert_artifacts_absent(stage);
        }
        println!("CJ0_D_LOCAL_SUBUNIT_PREFLIGHT_OK_NO_CELL_ENTERED");
        return;
    }
    assert_artifacts_absent(mode);
    eprintln!("CJ0_D_LOCAL_SUBUNIT_{}_EVIDENCE_SPENT", mode.to_uppercase());
    let (rows, pass, classification) = match mode {
        "probe" => run_probe(),
        "micro" => run_micro(),
        "gate" => run_gate(),
        _ => unreachable!(),
    };
    write_atomic(mode, &rows, pass, classification);
    println!(
        "CJ0_D_LOCAL_SUBUNIT_{}_{}",
        mode.to_uppercase(),
        classification
    );
    if !pass {
        std::process::exit(1);
    }
}

fn fresh(fixture: Fixture) -> Matter {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; PORTS];
    let mut source_physical = [0; PORTS];
    let mut locals = [None; SITES];
    let mut outsides = [None; SITES];
    let order = if fixture.reverse_allocation {
        (0..SITES).rev().collect::<Vec<_>>()
    } else {
        (0..SITES).collect::<Vec<_>>()
    };
    for site in order {
        let center = if fixture.mirror {
            -(site as i32) * 100
        } else {
            site as i32 * 100
        };
        let sides = if fixture.reverse_allocation {
            [1, 0]
        } else {
            [0, 1]
        };
        for side in sides {
            let port = site * 2 + side;
            let id_side = if fixture.reverse_identity {
                1 - side
            } else {
                side
            };
            let direction = if fixture.mirror { -1 } else { 1 };
            let signed = if side == 0 {
                -fixture.spacing
            } else {
                fixture.spacing
            };
            source_physical[port] = fixture.namespace + 100 + (site * 2 + id_side) as u64;
            sources[port] = Some(substrate.add_cell(cell(
                source_physical[port],
                center + direction * signed,
                0,
                2,
                1_000,
            )));
        }
        locals[site] = Some(substrate.add_cell(cell(
            fixture.namespace + 200 + site as u64,
            center,
            0,
            4,
            1_000,
        )));
        outsides[site] = Some(substrate.add_cell(cell(
            fixture.namespace + 300 + site as u64,
            center + 40_000,
            1,
            1,
            1_000,
        )));
    }
    let noise = substrate.add_cell(cell(fixture.namespace + 900, 90_000, 0, 1, 1_000));
    let sources = sources.map(|value| value.expect("source"));
    let locals = locals.map(|value| value.expect("local"));
    let outsides = outsides.map(|value| value.expect("outside"));
    let mut initial_inputs = [None; PORTS];
    for site in 0..SITES {
        for side in 0..2 {
            let port = site * 2 + side;
            initial_inputs[port] = Some(substrate.add_arrow(arrow(
                sources[port],
                locals[site],
                1,
                1,
                u32::from(fixture.weak),
            )));
            substrate.add_arrow(arrow(
                locals[site],
                sources[port],
                1,
                1,
                if fixture.returns { 1_000 } else { 0 },
            ));
        }
        substrate.add_arrow(arrow(locals[site], outsides[site], 1, 1, 1_000));
    }
    Matter {
        substrate,
        namespace: fixture.namespace,
        sources,
        source_physical,
        locals,
        initial_inputs: initial_inputs.map(|value| value.expect("input")),
        noise,
        reverse_insertion: fixture.reverse_insertion,
        schedule_floors: 0,
    }
}

fn enter_routes(matter: &mut Matter, requested_tick: i64, routes: &[usize], origin: u64) {
    let tick = floor_tick(matter, requested_tick, "external-route-entry");
    let mut ports = (0..PORTS)
        .filter(|port| routes.contains(&SITE_ROUTES[*port / 2][*port % 2]))
        .collect::<Vec<_>>();
    if matter.reverse_insertion {
        ports.reverse();
    }
    for port in ports {
        let phases = if matter.reverse_insertion {
            [1, 0]
        } else {
            [0, 1]
        };
        for (serial, phase) in phases.into_iter().enumerate() {
            matter.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase,
                origin_physical: origin + port as u64 * 8 + serial as u64,
                target: matter.sources[port],
                impulse: 1,
            });
        }
    }
}

fn enter_noise(matter: &mut Matter, requested_tick: i64, origin: u64) {
    let tick = floor_tick(matter, requested_tick, "external-noise-entry");
    matter.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target: matter.noise,
        impulse: 1,
    });
}

fn propagate(matter: &mut Matter, active_routes: &[usize]) -> Counts {
    let run = matter.substrate.propagate();
    counts(matter, active_routes, &run)
}

fn counts(matter: &Matter, active_routes: &[usize], run: &Execution) -> Counts {
    let active_port = |port: usize| active_routes.contains(&SITE_ROUTES[port / 2][port % 2]);
    Counts {
        source_firings: std::array::from_fn(|port| firings_at(run, matter.source_physical[port])),
        traversals: std::array::from_fn(|site| {
            run.trace
                .iter()
                .filter(|entry| entry.target_physical == matter.namespace + 200 + site as u64)
                .count()
        }),
        local_events: std::array::from_fn(|site| {
            run.trace
                .iter()
                .filter(|entry| {
                    entry.target_physical == matter.namespace + 200 + site as u64
                        && entry.local_subunit_integration
                })
                .count()
        }),
        local_firings: std::array::from_fn(|site| {
            firings_at(run, matter.namespace + 200 + site as u64)
        }),
        returned_arrivals: std::array::from_fn(|port| {
            let delivered = run
                .trace
                .iter()
                .filter(|entry| entry.target_physical == matter.source_physical[port])
                .count();
            delivered.saturating_sub(if active_port(port) { 2 } else { 0 })
        }),
        outward: std::array::from_fn(|site| {
            run.crossings
                .iter()
                .filter(|crossing| crossing.from_physical == matter.namespace + 200 + site as u64)
                .count()
        }),
        noise_firings: firings_at(run, matter.namespace + 900),
        quiescent: run.naturally_quiescent,
        work: run.work.clone(),
    }
}

fn state(matter: &Matter) -> State {
    let resistance = std::array::from_fn(|port| live_input_values(matter, port).0);
    let coupling = std::array::from_fn(|port| live_input_values(matter, port).1);
    State {
        resistance,
        coupling,
        live: std::array::from_fn(|port| resistance[port] > 0),
        local_state: std::array::from_fn(|site| matter.substrate.cell_state(matter.locals[site])),
        arrow_count: matter.substrate.arrow_count(),
        persistent_bytes: matter.substrate.persistent_bytes(),
        permanent_fingerprint: matter.substrate.permanent_fingerprint(),
        complete_fingerprint: matter.substrate.complete_fingerprint(),
        schedule_floors: matter.schedule_floors,
    }
}

fn live_input_values(matter: &Matter, port: usize) -> (u32, i32) {
    let site = port / 2;
    matter
        .substrate
        .arrows_between(matter.sources[port], matter.locals[site])
        .into_iter()
        .filter(|arrow_id| matter.substrate.arrow_is_live(*arrow_id))
        .map(|arrow_id| {
            (
                matter.substrate.arrow_resistance(arrow_id),
                matter.substrate.arrow_coupling(arrow_id),
            )
        })
        .max_by_key(|value| value.0)
        .unwrap_or((0, 0))
}

fn advance(matter: &mut Matter, requested_tick: i64, total: &mut Counts) {
    let tick = floor_tick(matter, requested_tick, "advance");
    let work = matter.substrate.advance_time(tick);
    merge_work(&mut total.work, &work);
}

fn floor_tick(matter: &mut Matter, requested: i64, surface: &str) -> i64 {
    let current = matter.substrate.current_tick();
    if requested < current {
        matter.schedule_floors += 1;
        eprintln!(
            "CJ0_D_TIMING_FLOOR surface={surface} requested={requested} current={current} namespace={:#x}",
            matter.namespace
        );
        current
    } else {
        requested
    }
}

fn train_partition(
    matter: &mut Matter,
    partition: [[usize; 2]; 2],
    rounds: usize,
    start: i64,
    spacing: i64,
    gap: i64,
) -> (Counts, i64) {
    let mut total = Counts {
        quiescent: true,
        ..Counts::default()
    };
    let mut last = start;
    for round in 0..rounds {
        let order = if round % 2 == 0 { [0, 1] } else { [1, 0] };
        for (cluster, which) in order.into_iter().enumerate() {
            let tick = start + round as i64 * spacing + cluster as i64 * gap;
            advance(matter, tick, &mut total);
            enter_routes(
                matter,
                tick,
                &partition[which],
                matter.namespace + 10_000 + round as u64 * 1_000 + which as u64 * 100,
            );
            let run = propagate(matter, &partition[which]);
            merge_counts(&mut total, &run);
            last = tick;
        }
    }
    (total, last)
}

fn observe(matter: &Matter, routes: &[usize], tick: i64) -> (Counts, State) {
    let mut copy = matter.clone();
    let origin = copy.namespace + 800_000 + tick as u64 * 100;
    let mut total = Counts {
        quiescent: true,
        ..Counts::default()
    };
    advance(&mut copy, tick, &mut total);
    enter_routes(&mut copy, tick, routes, origin);
    let run = propagate(&mut copy, routes);
    merge_counts(&mut total, &run);
    (total, state(&copy))
}

fn partition_outcome(fixture: Fixture, partition: [[usize; 2]; 2]) -> PartitionOutcome {
    let mut matter = fresh(fixture);
    let (training, last) = train_partition(&mut matter, partition, 6, 0, 10, 4);
    let observation_tick = last + 10;
    let trained = partition.map(|routes| observe(&matter, &routes, observation_tick).0);
    let crossed_partition = if partition == OLD_PARTITION {
        NEW_PARTITION
    } else {
        OLD_PARTITION
    };
    let crossed = crossed_partition.map(|routes| observe(&matter, &routes, observation_tick).0);
    let singletons = std::array::from_fn(|route| observe(&matter, &[route], observation_tick).0);
    PartitionOutcome {
        training: training.clone(),
        state: state(&matter),
        trained_out: trained.clone().map(|counts| sum(&counts.outward)),
        crossed_out: crossed.clone().map(|counts| sum(&counts.outward)),
        singleton_out: singletons.map(|counts| sum(&counts.outward)),
        trained_events: trained.map(|counts| sum(&counts.local_events)),
        crossed_events: crossed.map(|counts| sum(&counts.local_events)),
        source_refiring: training.source_firings.iter().any(|count| *count != 6),
    }
}

fn run_probe() -> (Vec<StageRow>, bool, &'static str) {
    let fixtures = [
        Fixture {
            namespace: 0xC_D100_0000,
            mirror: false,
            reverse_allocation: false,
            reverse_identity: false,
            reverse_insertion: false,
            returns: true,
            weak: true,
            spacing: 2,
        },
        Fixture {
            namespace: 0xC_D200_0000,
            mirror: true,
            reverse_allocation: true,
            reverse_identity: true,
            reverse_insertion: true,
            returns: true,
            weak: true,
            spacing: 2,
        },
    ];
    let mut rows = Vec::new();
    for (index, fixture) in fixtures.into_iter().enumerate() {
        let outcome = partition_outcome(fixture, OLD_PARTITION);
        let duplicate = partition_outcome(fixture, OLD_PARTITION);
        let matched = matched_training(&outcome.training);
        let pass = outcome == duplicate
            && matched
            && outcome.trained_out == [1, 1]
            && outcome.crossed_out == [0, 0]
            && outcome.singleton_out == [0; ROUTES]
            && outcome.trained_events == [0, 0]
            && outcome.crossed_events == [1, 1]
            && !outcome.source_refiring
            && outcome.state.schedule_floors == 0
            && outcome.training.quiescent;
        rows.push(row_from_outcome(
            format!("matched-primary-{index}"),
            fixture.namespace,
            outcome,
            matched,
            true,
            pass,
            "six-round A+B/C+D acquisition and held-out discrimination",
        ));
    }

    let alternative_fixture = Fixture {
        namespace: 0xC_D300_0000,
        mirror: false,
        reverse_allocation: true,
        reverse_identity: false,
        reverse_insertion: true,
        returns: true,
        weak: true,
        spacing: 2,
    };
    let alternative = partition_outcome(alternative_fixture, NEW_PARTITION);
    let alternative_pass = alternative.trained_out == [1, 1]
        && alternative.crossed_out == [0, 0]
        && alternative.singleton_out == [0; ROUTES]
        && alternative.state.schedule_floors == 0
        && matched_training(&alternative.training);
    rows.push(row_from_outcome(
        "stable-alternative".into(),
        alternative_fixture.namespace,
        alternative,
        true,
        true,
        alternative_pass,
        "fresh A+D/C+B organization",
    ));

    rows.push(run_silence_control(
        "singleton-only",
        0xC_D400_0000,
        true,
        true,
        2,
        Control::Singleton,
    ));
    rows.push(run_silence_control(
        "too-late-second",
        0xC_D500_0000,
        true,
        true,
        2,
        Control::Late,
    ));
    rows.push(run_silence_control(
        "correlation-without-traversal",
        0xC_D600_0000,
        true,
        true,
        2,
        Control::Noise,
    ));
    rows.push(run_silence_control(
        "traversal-return-blocked",
        0xC_D700_0000,
        false,
        true,
        2,
        Control::Joint,
    ));
    rows.push(run_silence_control(
        "absent-opportunity",
        0xC_D800_0000,
        true,
        false,
        20,
        Control::Joint,
    ));
    rows.push(run_silence_control(
        "stale-opportunity",
        0xC_D900_0000,
        true,
        true,
        2,
        Control::Stale,
    ));
    rows.push(run_multi_control(
        "ambiguity-three",
        0xC_DA00_0000,
        &[0, 1, 2],
        3,
    ));
    rows.push(run_multi_control(
        "genuine-four",
        0xC_DB00_0000,
        &[0, 1, 2, 3],
        6,
    ));
    let pass = rows.iter().all(|row| row.pass);
    (
        rows,
        pass,
        if pass { "POSITIVE" } else { "FROZEN_NEGATIVE" },
    )
}

#[derive(Clone, Copy)]
enum Control {
    Singleton,
    Late,
    Noise,
    Joint,
    Stale,
}

fn run_silence_control(
    name: &str,
    namespace: u64,
    returns: bool,
    weak: bool,
    spacing: i32,
    control: Control,
) -> StageRow {
    let fixture = Fixture {
        namespace,
        mirror: false,
        reverse_allocation: false,
        reverse_identity: false,
        reverse_insertion: false,
        returns,
        weak,
        spacing,
    };
    let mut matter = fresh(fixture);
    let mut total = Counts {
        quiescent: true,
        ..Counts::default()
    };
    if matches!(control, Control::Stale) {
        advance(&mut matter, 30, &mut total);
    }
    for round in 0..4 {
        let base = if matches!(control, Control::Stale) {
            30
        } else {
            0
        } + round * 10;
        advance(&mut matter, base, &mut total);
        match control {
            Control::Singleton => enter_routes(&mut matter, base, &[0], namespace + base as u64),
            Control::Late => {
                enter_routes(&mut matter, base, &[0], namespace + base as u64);
                let run = propagate(&mut matter, &[0]);
                merge_counts(&mut total, &run);
                advance(&mut matter, base + 1, &mut total);
                enter_routes(&mut matter, base + 1, &[1], namespace + 1_000 + base as u64);
            }
            Control::Noise => enter_noise(&mut matter, base, namespace + base as u64),
            Control::Joint | Control::Stale => {
                enter_routes(&mut matter, base, &[0, 1], namespace + base as u64)
            }
        }
        let active: &[usize] = match control {
            Control::Singleton => &[0],
            Control::Late => &[1],
            Control::Noise => &[],
            Control::Joint | Control::Stale => &[0, 1],
        };
        let run = propagate(&mut matter, active);
        merge_counts(&mut total, &run);
    }
    let silence = sum(&total.outward) == 0 && total.local_firings == [0; SITES];
    let expected_events = match control {
        Control::Joint if returns => 1,
        Control::Stale => 1,
        Control::Joint => 4,
        _ => 0,
    };
    let event_ok = sum(&total.local_events) == expected_events;
    let pass = silence && event_ok && total.quiescent;
    StageRow {
        case: name.into(),
        namespace,
        matched: true,
        trained_out: [0, 0],
        crossed_out: [0, 0],
        singleton_out: [0; ROUTES],
        counts: total,
        state: state(&matter),
        duplicate_exact: true,
        pass,
        note: format!("silence={silence}; expected_local_events={expected_events}"),
    }
}

fn run_multi_control(name: &str, namespace: u64, routes: &[usize], expected: usize) -> StageRow {
    let fixture = Fixture {
        namespace,
        mirror: false,
        reverse_allocation: false,
        reverse_identity: false,
        reverse_insertion: false,
        returns: true,
        weak: true,
        spacing: 2,
    };
    let mut matter = fresh(fixture);
    let mut total = Counts {
        quiescent: true,
        ..Counts::default()
    };
    for round in 0..2 {
        let tick = round * 10;
        advance(&mut matter, tick, &mut total);
        enter_routes(&mut matter, tick, routes, namespace + tick as u64);
        let run = propagate(&mut matter, routes);
        merge_counts(&mut total, &run);
    }
    let pass = sum(&total.local_events) == expected
        && sum(&total.local_firings) == expected
        && sum(&total.outward) == expected
        && total.quiescent;
    StageRow {
        case: name.into(),
        namespace,
        matched: true,
        trained_out: [sum(&total.outward), 0],
        crossed_out: [0, 0],
        singleton_out: [0; ROUTES],
        counts: total,
        state: state(&matter),
        duplicate_exact: true,
        pass,
        note: format!("expected_supported_sites={expected}"),
    }
}

fn run_micro() -> (Vec<StageRow>, bool, &'static str) {
    let fixture = Fixture {
        namespace: 0xC_DC00_0000,
        mirror: false,
        reverse_allocation: false,
        reverse_identity: false,
        reverse_insertion: false,
        returns: true,
        weak: true,
        spacing: 2,
    };
    let (rows, exact) = reversal_case(fixture);
    let mut all_rows = rows;
    let mirrored_fixture = Fixture {
        namespace: 0xC_DD00_0000,
        mirror: true,
        reverse_allocation: true,
        reverse_identity: true,
        reverse_insertion: true,
        ..fixture
    };
    let (mirror_rows, mirror_exact) = reversal_case(mirrored_fixture);
    all_rows.extend(mirror_rows);
    all_rows.push(bootstrap_case(0xC_DE00_0000));
    let pass = exact && mirror_exact && all_rows.iter().all(|row| row.pass);
    (
        all_rows,
        pass,
        if pass { "POSITIVE" } else { "FROZEN_NEGATIVE" },
    )
}

fn reversal_case(fixture: Fixture) -> (Vec<StageRow>, bool) {
    let mut matter = fresh(fixture);
    let (initial_counts, initial_last) = train_partition(&mut matter, OLD_PARTITION, 6, 0, 10, 4);
    let initial_state = state(&matter);
    let initial_trained =
        OLD_PARTITION.map(|routes| observe(&matter, &routes, initial_last + 10).0);
    let initial_cross = NEW_PARTITION.map(|routes| observe(&matter, &routes, initial_last + 10).0);
    let initial_pass = initial_trained
        .iter()
        .all(|counts| sum(&counts.outward) == 1)
        && initial_cross.iter().all(|counts| sum(&counts.outward) == 0);
    let initial_row = StageRow {
        case: "before-reversal".into(),
        namespace: fixture.namespace,
        matched: matched_training(&initial_counts),
        trained_out: initial_trained.map(|counts| sum(&counts.outward)),
        crossed_out: initial_cross.map(|counts| sum(&counts.outward)),
        singleton_out: [0; ROUTES],
        counts: initial_counts,
        state: initial_state.clone(),
        duplicate_exact: true,
        pass: initial_pass,
        note: "old organization acquired".into(),
    };

    let (self_counts, self_state) = observe(&matter, &[0], initial_last + 10);
    let self_pass = sum(&self_counts.local_events) == 0
        && sum(&self_counts.local_firings) == 0
        && sum(&self_counts.returned_arrivals) == 0
        && self_counts.work.local_return_updates == 0
        && self_state.resistance[0] <= initial_state.resistance[0];
    let self_row = StageRow {
        case: "self-evidence-old-A-alone".into(),
        namespace: fixture.namespace,
        matched: true,
        trained_out: [0, 0],
        crossed_out: [0, 0],
        singleton_out: [sum(&self_counts.outward), 0, 0, 0],
        counts: self_counts,
        state: self_state,
        duplicate_exact: true,
        pass: self_pass,
        note: "old support is not contemporary two-source evidence".into(),
    };

    let swap_start = initial_last + 20;
    let (swap_counts, swap_last) =
        train_partition(&mut matter, NEW_PARTITION, 18, swap_start, 10, 4);
    let mut pressure_counts = Counts::default();
    advance(&mut matter, swap_last + 10, &mut pressure_counts);
    let final_state = state(&matter);
    let old_observed = OLD_PARTITION.map(|routes| observe(&matter, &routes, swap_last + 20).0);
    let new_observed = NEW_PARTITION.map(|routes| observe(&matter, &routes, swap_last + 20).0);
    let old_support_zero = OLD_SITES.iter().all(|site| {
        final_state.resistance[site * 2] == 0 && final_state.resistance[site * 2 + 1] == 0
    });
    let new_support_live = NEW_SITES
        .iter()
        .all(|site| final_state.coupling[site * 2] == 2 && final_state.coupling[site * 2 + 1] == 2);
    let original_dead = OLD_SITES.iter().all(|site| {
        !matter
            .substrate
            .arrow_is_live(matter.initial_inputs[site * 2])
            && !matter
                .substrate
                .arrow_is_live(matter.initial_inputs[site * 2 + 1])
            && matter
                .substrate
                .arrow_generation(matter.initial_inputs[site * 2])
                > 1
            && matter
                .substrate
                .arrow_generation(matter.initial_inputs[site * 2 + 1])
                > 1
    });
    let swap_pass = old_support_zero
        && new_support_live
        && original_dead
        && old_observed.iter().all(|counts| sum(&counts.outward) == 0)
        && new_observed.iter().all(|counts| sum(&counts.outward) == 1)
        && matched_training(&swap_counts)
        && swap_counts.quiescent;
    let swap_row = StageRow {
        case: "after-contemporary-reversal".into(),
        namespace: fixture.namespace,
        matched: matched_training(&swap_counts),
        trained_out: new_observed.map(|counts| sum(&counts.outward)),
        crossed_out: old_observed.map(|counts| sum(&counts.outward)),
        singleton_out: [0; ROUTES],
        counts: swap_counts,
        state: final_state,
        duplicate_exact: true,
        pass: swap_pass,
        note: format!(
            "old_support_zero={old_support_zero}; new_support_live={new_support_live}; original_generations_dead={original_dead}"
        ),
    };

    let mut duplicate = fresh(fixture);
    let (_, duplicate_initial_last) = train_partition(&mut duplicate, OLD_PARTITION, 6, 0, 10, 4);
    let (_, duplicate_swap_last) = train_partition(
        &mut duplicate,
        NEW_PARTITION,
        18,
        duplicate_initial_last + 20,
        10,
        4,
    );
    let mut duplicate_pressure = Counts::default();
    advance(
        &mut duplicate,
        duplicate_swap_last + 10,
        &mut duplicate_pressure,
    );
    let exact = state(&matter) == state(&duplicate);
    ([initial_row, self_row, swap_row].into(), exact)
}

fn bootstrap_case(namespace: u64) -> StageRow {
    let fixture = Fixture {
        namespace,
        mirror: false,
        reverse_allocation: false,
        reverse_identity: true,
        reverse_insertion: true,
        returns: true,
        weak: true,
        spacing: 2,
    };
    let mut matter = fresh(fixture);
    let mut total = Counts {
        quiescent: true,
        ..Counts::default()
    };
    advance(&mut matter, 20, &mut total);
    let all_initial_dead = matter
        .initial_inputs
        .iter()
        .all(|arrow_id| !matter.substrate.arrow_is_live(*arrow_id));
    let (training, last) = train_partition(&mut matter, NEW_PARTITION, 3, 20, 10, 4);
    merge_counts(&mut total, &training);
    let first_events_before_firing = sum(&training.local_events) == 2;
    let new_observed = NEW_PARTITION.map(|routes| observe(&matter, &routes, last + 10).0);
    let current = state(&matter);
    let pass = all_initial_dead
        && first_events_before_firing
        && training.work.local_structural_proposals > 0
        && new_observed.iter().all(|counts| sum(&counts.outward) == 1)
        && NEW_SITES
            .iter()
            .all(|site| current.coupling[site * 2] == 2 && current.coupling[site * 2 + 1] == 2);
    StageRow {
        case: "bootstrap-after-full-input-deallocation".into(),
        namespace,
        matched: matched_training(&training),
        trained_out: new_observed.map(|counts| sum(&counts.outward)),
        crossed_out: [0, 0],
        singleton_out: [0; ROUTES],
        counts: total,
        state: current,
        duplicate_exact: true,
        pass,
        note: format!(
            "all_initial_dead={all_initial_dead}; subthreshold_events_before_firing={first_events_before_firing}"
        ),
    }
}

fn run_gate() -> (Vec<StageRow>, bool, &'static str) {
    let mut rows = Vec::new();
    for seed in 0..12u64 {
        let fixture = Fixture {
            namespace: 0xD_0000_0000 + seed * 0x0100_0000,
            mirror: seed % 2 == 1,
            reverse_allocation: seed % 3 == 1,
            reverse_identity: seed % 4 >= 2,
            reverse_insertion: seed % 3 == 2,
            returns: true,
            weak: true,
            spacing: 2,
        };
        let spacing = [10, 12, 14][seed as usize % 3];
        let gap = [3, 4, 5][seed as usize % 3];
        let mut matter = fresh(fixture);
        let (initial, last) = train_partition(&mut matter, OLD_PARTITION, 6, 0, spacing, gap);
        let (reversal, swap_last) = train_partition(
            &mut matter,
            NEW_PARTITION,
            18,
            last + spacing * 2,
            spacing,
            gap,
        );
        let mut pressure = Counts::default();
        advance(&mut matter, swap_last + 20, &mut pressure);
        let current = state(&matter);
        let new_out = NEW_PARTITION.map(|routes| observe(&matter, &routes, swap_last + 30).0);
        let old_out = OLD_PARTITION.map(|routes| observe(&matter, &routes, swap_last + 30).0);
        let singleton =
            std::array::from_fn(|route| sum(&observe(&matter, &[route], swap_last + 30).0.outward));
        let old_zero = OLD_SITES
            .iter()
            .all(|site| current.resistance[site * 2] == 0 && current.resistance[site * 2 + 1] == 0);
        let new_live = NEW_SITES
            .iter()
            .all(|site| current.coupling[site * 2] == 2 && current.coupling[site * 2 + 1] == 2);
        let mut duplicate = fresh(fixture);
        let (_, duplicate_last) =
            train_partition(&mut duplicate, OLD_PARTITION, 6, 0, spacing, gap);
        let (_, duplicate_swap_last) = train_partition(
            &mut duplicate,
            NEW_PARTITION,
            18,
            duplicate_last + spacing * 2,
            spacing,
            gap,
        );
        let mut duplicate_pressure = Counts::default();
        advance(
            &mut duplicate,
            duplicate_swap_last + 20,
            &mut duplicate_pressure,
        );
        let exact = state(&matter) == state(&duplicate);
        let pass = matched_training(&initial)
            && matched_training(&reversal)
            && old_zero
            && new_live
            && new_out.iter().all(|counts| sum(&counts.outward) == 1)
            && old_out.iter().all(|counts| sum(&counts.outward) == 0)
            && singleton == [0; ROUTES]
            && exact
            && initial.quiescent
            && reversal.quiescent;
        let mut total = initial;
        merge_counts(&mut total, &reversal);
        merge_counts(&mut total, &pressure);
        rows.push(StageRow {
            case: format!("fresh-gate-{seed:02}"),
            namespace: fixture.namespace,
            matched: true,
            trained_out: new_out.map(|counts| sum(&counts.outward)),
            crossed_out: old_out.map(|counts| sum(&counts.outward)),
            singleton_out: singleton,
            counts: total,
            state: current,
            duplicate_exact: exact,
            pass,
            note: format!(
                "spacing={spacing}; cluster_gap={gap}; old_zero={old_zero}; new_live={new_live}"
            ),
        });
    }
    rows.push(recursion_case(0xD_D000_0000));
    rows.push(or_case(0xD_E000_0000));
    rows.extend(temporal_cases(0xD_F000_0000));
    let pass = rows.iter().all(|row| row.pass);
    (
        rows,
        pass,
        if pass { "POSITIVE" } else { "FROZEN_NEGATIVE" },
    )
}

fn recursion_case(namespace: u64) -> StageRow {
    let mut substrate = PlasticSubstrate::new();
    let primitive: [CellId; 4] = std::array::from_fn(|index| {
        substrate.add_cell(cell(
            namespace + 10 + index as u64,
            10_000 + index as i32 * 100,
            0,
            2,
            1_000,
        ))
    });
    let x_ports = [
        substrate.add_cell(cell(namespace + 100, -2, 0, 2, 1_000)),
        substrate.add_cell(cell(namespace + 101, 2, 0, 2, 1_000)),
    ];
    let x = substrate.add_cell(cell(namespace + 110, 0, 0, 4, 1_000));
    let y_ports = [
        substrate.add_cell(cell(namespace + 200, 98, 0, 2, 1_000)),
        substrate.add_cell(cell(namespace + 201, 102, 0, 2, 1_000)),
    ];
    let y = substrate.add_cell(cell(namespace + 210, 100, 0, 4, 1_000));
    let z_ports = [
        substrate.add_cell(cell(namespace + 300, 198, 0, 2, 1_000)),
        substrate.add_cell(cell(namespace + 301, 202, 0, 2, 1_000)),
    ];
    let z = substrate.add_cell(cell(namespace + 310, 200, 0, 4, 1_000));
    let outside = substrate.add_cell(cell(namespace + 400, 50_000, 1, 1, 1_000));
    for (driver, port) in primitive
        .into_iter()
        .zip([x_ports[0], x_ports[1], y_ports[1], z_ports[1]])
    {
        substrate.add_arrow(arrow(driver, port, 0, 2, 1_000));
    }
    let x_inputs = add_node(&mut substrate, x_ports, x, Some((y_ports[0], 0)));
    let y_inputs = add_node(&mut substrate, y_ports, y, Some((z_ports[0], 0)));
    let z_inputs = add_node(&mut substrate, z_ports, z, None);
    substrate.add_arrow(arrow(z, outside, 1, 1, 1_000));

    let mut total_work = WorkLedger::default();
    let mut all_quiescent = true;
    for level in 0..3 {
        for round in 0..2 {
            let tick = (level * 30 + round * 10) as i64;
            merge_work(&mut total_work, &substrate.advance_time(tick));
            enter_primitive_driver(
                &mut substrate,
                primitive[0],
                tick,
                namespace + 1_000 + tick as u64,
            );
            enter_primitive_driver(
                &mut substrate,
                primitive[1],
                tick,
                namespace + 2_000 + tick as u64,
            );
            if level >= 1 {
                enter_primitive_driver(
                    &mut substrate,
                    primitive[2],
                    tick + 1,
                    namespace + 3_000 + tick as u64,
                );
            }
            if level >= 2 {
                enter_primitive_driver(
                    &mut substrate,
                    primitive[3],
                    tick + 2,
                    namespace + 4_000 + tick as u64,
                );
            }
            let run = substrate.propagate();
            merge_work(&mut total_work, &run.work);
            all_quiescent &= run.naturally_quiescent;
        }
    }
    let held_tick = 100;
    merge_work(&mut total_work, &substrate.advance_time(held_tick));
    enter_primitive_driver(&mut substrate, primitive[0], held_tick, namespace + 10_000);
    enter_primitive_driver(&mut substrate, primitive[1], held_tick, namespace + 11_000);
    enter_primitive_driver(
        &mut substrate,
        primitive[2],
        held_tick + 1,
        namespace + 12_000,
    );
    enter_primitive_driver(
        &mut substrate,
        primitive[3],
        held_tick + 2,
        namespace + 13_000,
    );
    let held = substrate.propagate();
    merge_work(&mut total_work, &held.work);
    let reached_z = held
        .crossings
        .iter()
        .any(|crossing| crossing.from_physical == namespace + 310);
    let x_equal = substrate.arrow_coupling(x_inputs[0]) == substrate.arrow_coupling(x_inputs[1]);
    let y_equal = substrate.arrow_coupling(y_inputs[0]) == substrate.arrow_coupling(y_inputs[1]);
    let z_equal = substrate.arrow_coupling(z_inputs[0]) == substrate.arrow_coupling(z_inputs[1]);

    let mut missing = substrate.clone();
    missing.advance_time(120);
    enter_primitive_driver(&mut missing, primitive[0], 120, namespace + 20_000);
    enter_primitive_driver(&mut missing, primitive[1], 120, namespace + 21_000);
    enter_primitive_driver(&mut missing, primitive[2], 121, namespace + 22_000);
    let missing_run = missing.propagate();
    let missing_z = missing_run
        .crossings
        .iter()
        .any(|crossing| crossing.from_physical == namespace + 310);
    let pass = reached_z && !missing_z && x_equal && y_equal && z_equal && all_quiescent;
    StageRow {
        case: "recursive-A+B-X-X+C-Y-Y+D-Z".into(),
        namespace,
        matched: x_equal && y_equal && z_equal,
        trained_out: [usize::from(reached_z), 0],
        crossed_out: [usize::from(missing_z), 0],
        singleton_out: [0; ROUTES],
        counts: Counts {
            quiescent: all_quiescent && held.naturally_quiescent && missing_run.naturally_quiescent,
            work: total_work,
            ..Counts::default()
        },
        state: State {
            resistance: [0; PORTS],
            coupling: [0; PORTS],
            live: [false; PORTS],
            local_state: [0; SITES],
            arrow_count: substrate.arrow_count(),
            persistent_bytes: substrate.persistent_bytes(),
            permanent_fingerprint: substrate.permanent_fingerprint(),
            complete_fingerprint: substrate.complete_fingerprint(),
            schedule_floors: 0,
        },
        duplicate_exact: true,
        pass,
        note: format!(
            "X_equal={x_equal}; Y_equal={y_equal}; Z_equal={z_equal}; missing_D_silent={}",
            !missing_z
        ),
    }
}

fn add_node(
    substrate: &mut PlasticSubstrate,
    ports: [CellId; 2],
    local: CellId,
    output_port: Option<(CellId, i64)>,
) -> [ArrowId; 2] {
    let inputs = [
        substrate.add_arrow(arrow(ports[0], local, 1, 1, 1)),
        substrate.add_arrow(arrow(ports[1], local, 1, 1, 1)),
    ];
    substrate.add_arrow(arrow(local, ports[0], 1, 1, 1_000));
    substrate.add_arrow(arrow(local, ports[1], 1, 1, 1_000));
    if let Some((port, delay)) = output_port {
        substrate.add_arrow(arrow(local, port, delay, 2, 1_000));
    }
    inputs
}

fn enter_primitive_driver(
    substrate: &mut PlasticSubstrate,
    driver: CellId,
    tick: i64,
    origin: u64,
) {
    substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target: driver,
        impulse: 2,
    });
}

fn or_case(namespace: u64) -> StageRow {
    let mut substrate = PlasticSubstrate::new();
    let a = substrate.add_cell(cell(namespace + 1, 0, 0, 1, 1_000));
    let b = substrate.add_cell(cell(namespace + 2, 10, 0, 1, 1_000));
    let c = substrate.add_cell(cell(namespace + 3, 20, 0, 1, 1_000));
    substrate.add_arrow(arrow(a, c, 1, 1, 1_000));
    substrate.add_arrow(arrow(b, c, 1, 1, 1_000));
    let fire = |base: &PlasticSubstrate, routes: &[CellId], tick: i64| {
        let mut copy = base.clone();
        for (index, route) in routes.iter().enumerate() {
            copy.enter(SpikeInput {
                arrival_tick: tick,
                phase: index as i32,
                origin_physical: namespace + 100 + index as u64,
                target: *route,
                impulse: 1,
            });
        }
        let run = copy.propagate();
        firings_at(&run, namespace + 3)
    };
    let outputs = [
        fire(&substrate, &[a], 0),
        fire(&substrate, &[b], 0),
        fire(&substrate, &[a, b], 0),
    ];
    let pass = outputs == [1, 1, 1];
    StageRow {
        case: "ordinary-convergent-ARROW-inclusive-propagation".into(),
        namespace,
        matched: true,
        trained_out: [outputs[0], outputs[1]],
        crossed_out: [outputs[2], 0],
        singleton_out: [0; ROUTES],
        counts: Counts {
            quiescent: true,
            ..Counts::default()
        },
        state: empty_state(&substrate),
        duplicate_exact: true,
        pass,
        note: format!(
            "A_to_C={}; B_to_C={}; A_plus_B_to_C={}",
            outputs[0], outputs[1], outputs[2]
        ),
    }
}

fn temporal_cases(namespace: u64) -> Vec<StageRow> {
    let fixture = Fixture {
        namespace,
        mirror: false,
        reverse_allocation: false,
        reverse_identity: false,
        reverse_insertion: false,
        returns: true,
        weak: true,
        spacing: 2,
    };
    let mut matter = fresh(fixture);
    let (_, last) = train_partition(&mut matter, OLD_PARTITION, 3, 0, 10, 4);
    let base = last + 20;
    let schedules: [(&str, TimedRoutes); 5] = [
        ("time-together", vec![(base, vec![0, 1])]),
        ("time-A-then-B", vec![(base, vec![0]), (base + 1, vec![1])]),
        (
            "time-overlap-while",
            vec![(base, vec![0]), (base + 1, vec![0, 1])],
        ),
        (
            "time-within-residual-outside-local",
            vec![(base, vec![0]), (base + 1, vec![1])],
        ),
        ("time-B-absent-before-closure", vec![(base, vec![0])]),
    ];
    schedules
        .into_iter()
        .enumerate()
        .map(|(index, (name, schedule))| {
            let mut copy = matter.clone();
            let mut total = Counts {
                quiescent: true,
                ..Counts::default()
            };
            for (tick, routes) in schedule {
                advance(&mut copy, tick, &mut total);
                enter_routes(
                    &mut copy,
                    tick,
                    &routes,
                    namespace + index as u64 * 10_000 + tick as u64,
                );
                let run = propagate(&mut copy, &routes);
                merge_counts(&mut total, &run);
            }
            if name == "time-B-absent-before-closure" {
                advance(&mut copy, base + 3, &mut total);
            }
            let current = state(&copy);
            let site = 0;
            let outcome = (
                sum(&total.outward),
                sum(&total.local_events),
                current.local_state[site],
            );
            let expected = match name {
                "time-together" => (1, 0, 0),
                "time-A-then-B" | "time-within-residual-outside-local" => (0, 0, 3),
                "time-overlap-while" => (1, 0, 0),
                "time-B-absent-before-closure" => (0, 0, 0),
                _ => unreachable!(),
            };
            StageRow {
                case: name.into(),
                namespace: namespace + index as u64,
                matched: true,
                trained_out: [outcome.0, 0],
                crossed_out: [outcome.1, 0],
                singleton_out: [outcome.2.max(0) as usize, 0, 0, 0],
                counts: total,
                state: current,
                duplicate_exact: true,
                pass: outcome == expected,
                note: format!(
                    "outward={}; local_event={}; residual_state={}; serialized_schedule={index}",
                    outcome.0, outcome.1, outcome.2
                ),
            }
        })
        .collect()
}

fn empty_state(substrate: &PlasticSubstrate) -> State {
    State {
        resistance: [0; PORTS],
        coupling: [0; PORTS],
        live: [false; PORTS],
        local_state: [0; SITES],
        arrow_count: substrate.arrow_count(),
        persistent_bytes: substrate.persistent_bytes(),
        permanent_fingerprint: substrate.permanent_fingerprint(),
        complete_fingerprint: substrate.complete_fingerprint(),
        schedule_floors: 0,
    }
}

fn row_from_outcome(
    case: String,
    namespace: u64,
    outcome: PartitionOutcome,
    matched: bool,
    duplicate_exact: bool,
    pass: bool,
    note: &str,
) -> StageRow {
    StageRow {
        case,
        namespace,
        matched,
        trained_out: outcome.trained_out,
        crossed_out: outcome.crossed_out,
        singleton_out: outcome.singleton_out,
        counts: outcome.training,
        state: outcome.state,
        duplicate_exact,
        pass,
        note: note.into(),
    }
}

fn matched_training(counts: &Counts) -> bool {
    counts
        .source_firings
        .iter()
        .all(|count| *count == counts.source_firings[0])
        && counts.source_firings[0] > 0
        && counts.local_firings[OLD_SITES[0]] == counts.local_firings[OLD_SITES[1]]
}

fn firings_at(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn sum<const N: usize>(values: &[usize; N]) -> usize {
    values.iter().sum()
}

fn merge_counts(total: &mut Counts, next: &Counts) {
    for index in 0..PORTS {
        total.source_firings[index] += next.source_firings[index];
        total.returned_arrivals[index] += next.returned_arrivals[index];
    }
    for index in 0..SITES {
        total.traversals[index] += next.traversals[index];
        total.local_events[index] += next.local_events[index];
        total.local_firings[index] += next.local_firings[index];
        total.outward[index] += next.outward[index];
    }
    total.noise_firings += next.noise_firings;
    total.quiescent &= next.quiescent;
    merge_work(&mut total.work, &next.work);
}

fn merge_work(total: &mut WorkLedger, next: &WorkLedger) {
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
    total.ordinary_pressure_updates += next.ordinary_pressure_updates;
    total.local_structural_proposals += next.local_structural_proposals;
    total.physical_deallocations += next.physical_deallocations;
    total.local_subunit_integrations += next.local_subunit_integrations;
    total.local_subunit_spikes_emitted += next.local_subunit_spikes_emitted;
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32, resistance: u32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance,
    }
}

fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
    }
}

fn source_audit() -> bool {
    command_output(
        "git",
        &[
            "rev-parse",
            "px2-physical-causal-direction-authoritative^{commit}",
        ],
    ) == FROZEN_PARENT
        && sha256(AUTHORITY) == AUTHORITY_SHA256
        && sha256(PROTOCOL) == PROTOCOL_SHA256
        && sha256(RETRY_PROTOCOL) == RETRY_PROTOCOL_SHA256
        && Path::new("arms/cj0-d-local-subunit/build.rs").exists()
}

fn sha256(path: &str) -> String {
    command_output("sha256sum", &[path])
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()
}

fn command_output(program: &str, args: &[&str]) -> String {
    let output = Command::new(program)
        .args(args)
        .output()
        .expect("audit command");
    assert!(output.status.success(), "audit command failed: {program}");
    String::from_utf8(output.stdout)
        .expect("utf8 audit output")
        .trim()
        .to_string()
}

fn artifact_paths(stage: &str) -> (String, String, String, String) {
    let version = if stage == "probe" { "v2" } else { "v1" };
    (
        format!("results/cj0_d_local_subunit_{stage}_{version}.csv"),
        format!("results/cj0_d_local_subunit_{stage}_{version}.md"),
        format!("results/.cj0_d_local_subunit_{stage}_{version}.csv.staging"),
        format!("results/.cj0_d_local_subunit_{stage}_{version}.md.staging"),
    )
}

fn assert_artifacts_absent(stage: &str) {
    let paths = artifact_paths(stage);
    for path in [&paths.0, &paths.1, &paths.2, &paths.3] {
        assert!(
            !Path::new(path).exists(),
            "stage artifact already exists: {path}"
        );
    }
}

fn write_atomic(stage: &str, rows: &[StageRow], pass: bool, classification: &str) {
    let (csv, report, staging_csv, staging_report) = artifact_paths(stage);
    let mut csv_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staging_csv)
        .expect("create CSV staging");
    writeln!(csv_file, "stage,case,namespace,matched,trained_out,crossed_out,singleton_out,source_firings,traversals,local_events,local_firings,returned_arrivals,outward,local_return_updates,structural_proposals,deallocations,work,resistance,coupling,live,local_state,arrow_count,persistent_bytes,permanent_fingerprint,complete_fingerprint,schedule_floors,quiescent,duplicate_exact,pass,note").expect("CSV header");
    for row in rows {
        writeln!(
            csv_file,
            "{stage},{},{:#x},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:#018x},{:#018x},{},{},{},{},{}",
            row.case,
            row.namespace,
            row.matched,
            array(&row.trained_out),
            array(&row.crossed_out),
            array(&row.singleton_out),
            array(&row.counts.source_firings),
            array(&row.counts.traversals),
            array(&row.counts.local_events),
            array(&row.counts.local_firings),
            array(&row.counts.returned_arrivals),
            array(&row.counts.outward),
            row.counts.work.local_return_updates,
            row.counts.work.local_structural_proposals,
            row.counts.work.physical_deallocations,
            row.counts.work.total(),
            array(&row.state.resistance),
            array(&row.state.coupling),
            array(&row.state.live),
            array(&row.state.local_state),
            row.state.arrow_count,
            row.state.persistent_bytes,
            row.state.permanent_fingerprint,
            row.state.complete_fingerprint,
            row.state.schedule_floors,
            row.counts.quiescent,
            row.duplicate_exact,
            row.pass,
            row.note.replace(',', ";"),
        )
        .expect("CSV row");
    }
    csv_file.sync_all().expect("sync CSV");

    let total_work: u64 = rows.iter().map(|row| row.counts.work.total()).sum();
    let total_storage: usize = rows.iter().map(|row| row.state.persistent_bytes).sum();
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staging_report)
        .expect("create report staging");
    writeln!(report_file, "# CJ0-D local-subunit {stage} v1\n").expect("report");
    writeln!(report_file, "Status: **{classification}**.\n").expect("report");
    writeln!(report_file, "- conjunctive pass: `{pass}`").expect("report");
    writeln!(report_file, "- serialized rows: `{}`", rows.len()).expect("report");
    writeln!(
        report_file,
        "- passed rows: `{}`",
        rows.iter().filter(|row| row.pass).count()
    )
    .expect("report");
    writeln!(report_file, "- ledgered work: `{total_work}`").expect("report");
    writeln!(
        report_file,
        "- summed row persistent bytes: `{total_storage}`"
    )
    .expect("report");
    writeln!(
        report_file,
        "- authoritative source SHA-256: `{AUTHORITY_SHA256}`"
    )
    .expect("report");
    writeln!(report_file, "- protocol SHA-256: `{PROTOCOL_SHA256}`\n").expect("report");
    writeln!(report_file, "No definitive or authority execution occurred. See the CSV for every physical stage and clause.").expect("report");
    report_file.sync_all().expect("sync report");
    rename(staging_csv, csv).expect("publish CSV");
    rename(staging_report, report).expect("publish report");
}

fn array<T: std::fmt::Display, const N: usize>(values: &[T; N]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}
