use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const SIDES: usize = 2;
const ACQUISITION: usize = 4;
const EXPOSURES: usize = 8;
const CHILD_TIMEOUT: Duration = Duration::from_secs(5);
const PX0_SOURCE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PROBE_SOURCE_SHA256: &str =
    "1fb0168729e4181a8e778a93f92ebfae7f10576e66d6ef0aa99bc3050a3021a8";
const PROBE_NEGATIVE_SHA256: &str =
    "f45958a07021d0f116a7a77cfdb543d1b08c40ca7b57f675b3f028bbf6f6efaf";
const PROBE_HANDOFF_SHA256: &str =
    "7a32288f4f8e6f3c6cde26cb73af1ba4bfdb5256a04a90b587f0278bf7b3a985";
const PROTOCOL_SHA256: &str = "934dcd65a34d5bccb29915c814e8a7873db745b1136ac388c33d19db497860eb";
const RESULT_CSV: &str = "results/px1_recurrent_role_stability_diagnostic_v1.csv";
const RESULT_MD: &str = "results/px1_recurrent_role_stability_diagnostic_v1.md";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Margin,
    Inhibition,
    Distance,
    Timing,
}

impl Arm {
    const ALL: [Self; 4] = [Self::Margin, Self::Inhibition, Self::Distance, Self::Timing];

    fn name(self) -> &'static str {
        match self {
            Self::Margin => "margin",
            Self::Inhibition => "inhibition",
            Self::Distance => "distance",
            Self::Timing => "timing",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|arm| arm.name() == value)
    }

    fn index(self) -> u64 {
        match self {
            Self::Margin => 0,
            Self::Inhibition => 1,
            Self::Distance => 2,
            Self::Timing => 3,
        }
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    source_threshold: usize,
    sources: [CellId; SIDES],
    endpoints: [CellId; SIDES],
    sites: [CellId; SIDES],
    outlets: [CellId; SIDES],
    acquisition_drivers: [CellId; SIDES],
    role_drivers: [CellId; SIDES],
    brakes: [Option<CellId>; SIDES],
}

#[derive(Clone, Debug)]
struct WorldMetrics {
    role_resistance: [u32; SIDES],
    effects: [usize; SIDES],
    source_returns: usize,
    site_returns: usize,
    extra_source_firings: usize,
    brake_resistance: [u32; SIDES],
    brake_firings: usize,
    role_formed: bool,
    productive_recurrence: bool,
    duplicate_exact: bool,
    naturally_quiescent: bool,
    work: WorkLedger,
    fingerprint: u64,
}

#[derive(Clone, Debug)]
struct ArmResult {
    arm: Arm,
    completed: bool,
    timed_out: bool,
    primary: Option<WorldMetrics>,
    transfer: Option<WorldMetrics>,
    passed: bool,
}

impl ArmResult {
    fn timeout(arm: Arm) -> Self {
        Self {
            arm,
            completed: false,
            timed_out: true,
            primary: None,
            transfer: None,
            passed: false,
        }
    }

    fn encode(&self) -> String {
        let primary = self.primary.as_ref();
        let transfer = self.transfer.as_ref();
        let value = |metrics: Option<&WorldMetrics>, f: fn(&WorldMetrics) -> u64| {
            metrics.map(f).unwrap_or(0)
        };
        [
            "PX1_ARM".to_string(),
            self.arm.name().to_string(),
            self.completed.to_string(),
            self.timed_out.to_string(),
            value(primary, |m| u64::from(m.role_formed)).to_string(),
            value(primary, |m| m.role_resistance[0] as u64).to_string(),
            value(primary, |m| m.role_resistance[1] as u64).to_string(),
            value(primary, |m| m.effects[0] as u64).to_string(),
            value(primary, |m| m.effects[1] as u64).to_string(),
            value(primary, |m| m.source_returns as u64).to_string(),
            value(primary, |m| m.site_returns as u64).to_string(),
            value(primary, |m| m.extra_source_firings as u64).to_string(),
            value(primary, |m| m.brake_resistance[0] as u64).to_string(),
            value(primary, |m| m.brake_firings as u64).to_string(),
            value(primary, |m| u64::from(m.productive_recurrence)).to_string(),
            value(primary, |m| u64::from(m.duplicate_exact)).to_string(),
            value(primary, |m| u64::from(m.naturally_quiescent)).to_string(),
            value(primary, |m| m.work.total()).to_string(),
            value(primary, |m| m.fingerprint).to_string(),
            value(transfer, |m| u64::from(m.role_formed)).to_string(),
            value(transfer, |m| m.role_resistance[0] as u64).to_string(),
            value(transfer, |m| m.role_resistance[1] as u64).to_string(),
            value(transfer, |m| m.effects[0] as u64).to_string(),
            value(transfer, |m| m.effects[1] as u64).to_string(),
            value(transfer, |m| m.source_returns as u64).to_string(),
            value(transfer, |m| m.site_returns as u64).to_string(),
            value(transfer, |m| m.extra_source_firings as u64).to_string(),
            value(transfer, |m| m.brake_resistance[1] as u64).to_string(),
            value(transfer, |m| m.brake_firings as u64).to_string(),
            value(transfer, |m| u64::from(m.productive_recurrence)).to_string(),
            value(transfer, |m| u64::from(m.duplicate_exact)).to_string(),
            value(transfer, |m| u64::from(m.naturally_quiescent)).to_string(),
            value(transfer, |m| m.work.total()).to_string(),
            value(transfer, |m| m.fingerprint).to_string(),
            self.passed.to_string(),
        ]
        .join("|")
    }

    fn decode(line: &str) -> Option<Self> {
        let fields = line.trim().split('|').collect::<Vec<_>>();
        if fields.len() != 35 || fields[0] != "PX1_ARM" {
            return None;
        }
        let arm = Arm::parse(fields[1])?;
        let completed = parse_bool(fields[2])?;
        let timed_out = parse_bool(fields[3])?;
        let primary = completed.then(|| WorldMetrics {
            role_formed: parse_bool(fields[4]).expect("primary role"),
            role_resistance: [parse_u32(fields[5]), parse_u32(fields[6])],
            effects: [parse_usize(fields[7]), parse_usize(fields[8])],
            source_returns: parse_usize(fields[9]),
            site_returns: parse_usize(fields[10]),
            extra_source_firings: parse_usize(fields[11]),
            brake_resistance: [parse_u32(fields[12]), 0],
            brake_firings: parse_usize(fields[13]),
            productive_recurrence: parse_bool(fields[14]).expect("primary recurrence"),
            duplicate_exact: parse_bool(fields[15]).expect("primary replay"),
            naturally_quiescent: parse_bool(fields[16]).expect("primary quiescence"),
            work: work_from_total(parse_u64(fields[17])),
            fingerprint: parse_u64(fields[18]),
        });
        let transfer = completed.then(|| WorldMetrics {
            role_formed: parse_bool(fields[19]).expect("transfer role"),
            role_resistance: [parse_u32(fields[20]), parse_u32(fields[21])],
            effects: [parse_usize(fields[22]), parse_usize(fields[23])],
            source_returns: parse_usize(fields[24]),
            site_returns: parse_usize(fields[25]),
            extra_source_firings: parse_usize(fields[26]),
            brake_resistance: [0, parse_u32(fields[27])],
            brake_firings: parse_usize(fields[28]),
            productive_recurrence: parse_bool(fields[29]).expect("transfer recurrence"),
            duplicate_exact: parse_bool(fields[30]).expect("transfer replay"),
            naturally_quiescent: parse_bool(fields[31]).expect("transfer quiescence"),
            work: work_from_total(parse_u64(fields[32])),
            fingerprint: parse_u64(fields[33]),
        });
        Some(Self {
            arm,
            completed,
            timed_out,
            primary,
            transfer,
            passed: parse_bool(fields[34])?,
        })
    }
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() == 2 && args[0] == "--arm" {
        let arm = Arm::parse(&args[1]).expect("known arm");
        println!("{}", run_arm(arm).encode());
        return;
    }
    if args != ["--diagnostic"] {
        eprintln!("PX1 diagnostic requires --diagnostic; definitive execution is forbidden");
        std::process::exit(2);
    }
    assert!(source_audit(), "frozen PX0/PX1 inputs must remain exact");
    assert!(!Path::new(RESULT_CSV).exists(), "diagnostic CSV exists");
    assert!(!Path::new(RESULT_MD).exists(), "diagnostic report exists");
    eprintln!("PX1_RECURRENT_ROLE_STABILITY_DEVELOPMENT_EVIDENCE");
    let results = run_parallel();
    write_new(RESULT_CSV, &csv(&results));
    write_new(RESULT_MD, &markdown(&results));
}

fn source_audit() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == PX0_SOURCE_SHA256
        && sha256("crates/px0-physical-correspondence/examples/px1_boundary_role_probe.rs")
            == PROBE_SOURCE_SHA256
        && sha256("results/px1_physical_boundary_roles_probe_v1_negative.md")
            == PROBE_NEGATIVE_SHA256
        && sha256("experiments/px1_physical_boundary_roles_probe_v1_collapse_handoff.md")
            == PROBE_HANDOFF_SHA256
        && sha256("experiments/px1_recurrent_role_stability_diagnostic_protocol.md")
            == PROTOCOL_SHA256
}

fn run_parallel() -> Vec<ArmResult> {
    let executable = env::current_exe().expect("current diagnostic executable");
    let mut children = Arm::ALL
        .into_iter()
        .map(|arm| {
            let child = Command::new(&executable)
                .args(["--arm", arm.name()])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("spawn diagnostic arm");
            (arm, child, Instant::now())
        })
        .collect::<Vec<_>>();
    let mut results = Vec::new();
    while !children.is_empty() {
        let mut index = 0;
        while index < children.len() {
            let finished = children[index].1.try_wait().expect("poll arm");
            if finished.is_some() {
                let (arm, child, _) = children.swap_remove(index);
                results.push(read_child(arm, child));
            } else if children[index].2.elapsed() >= CHILD_TIMEOUT {
                let (arm, mut child, _) = children.swap_remove(index);
                child.kill().expect("terminate non-quiescent arm");
                child.wait().expect("reap non-quiescent arm");
                results.push(ArmResult::timeout(arm));
            } else {
                index += 1;
            }
        }
        if !children.is_empty() {
            thread::sleep(Duration::from_millis(10));
        }
    }
    results.sort_by_key(|result| result.arm.index());
    results
}

fn read_child(arm: Arm, mut child: Child) -> ArmResult {
    let mut stdout = String::new();
    child
        .stdout
        .take()
        .expect("arm stdout")
        .read_to_string(&mut stdout)
        .expect("read arm stdout");
    ArmResult::decode(stdout.trim()).unwrap_or(ArmResult {
        arm,
        completed: false,
        timed_out: false,
        primary: None,
        transfer: None,
        passed: false,
    })
}

fn run_arm(arm: Arm) -> ArmResult {
    let primary_namespace = 0x3100_0000 + arm.index() * 0x0200_0000;
    let transfer_namespace = primary_namespace + 0x0100_0000;
    let primary = run_world(arm, primary_namespace, 0, false, false);
    let transfer = run_world(arm, transfer_namespace, 1, true, true);
    let inhibition_specific = if arm == Arm::Inhibition {
        primary.brake_resistance[0] > 1
            && transfer.brake_resistance[1] > 1
            && primary.brake_firings > 0
            && transfer.brake_firings > 0
    } else {
        true
    };
    let passed = world_passed(&primary, 0) && world_passed(&transfer, 1) && inhibition_specific;
    ArmResult {
        arm,
        completed: true,
        timed_out: false,
        primary: Some(primary),
        transfer: Some(transfer),
        passed,
    }
}

fn world_passed(metrics: &WorldMetrics, supported: usize) -> bool {
    let other = 1 - supported;
    metrics.role_formed
        && metrics.role_resistance[supported] > metrics.role_resistance[other]
        && metrics.effects[supported] == 1
        && metrics.effects[other] == 0
        && metrics.source_returns >= EXPOSURES
        && metrics.site_returns >= EXPOSURES
        && metrics.extra_source_firings == 0
        && metrics.productive_recurrence
        && metrics.duplicate_exact
        && metrics.naturally_quiescent
}

fn run_world(
    arm: Arm,
    namespace: u64,
    supported: usize,
    mirror: bool,
    reverse: bool,
) -> WorldMetrics {
    let mut world = build_world(arm, namespace, mirror, reverse);
    let mut work = WorkLedger::default();
    let mut naturally_quiescent = true;
    for exposure in 0..ACQUISITION {
        let tick = exposure as i64 * 16;
        for side in 0..SIDES {
            enter_many(
                &mut world.substrate,
                world.sources[side],
                tick,
                world.source_threshold,
                namespace + 0x1_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
            );
            enter_many(
                &mut world.substrate,
                world.acquisition_drivers[side],
                tick,
                1,
                namespace + 0x2_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
            );
        }
        let run = world.substrate.propagate();
        add_work(&mut work, &run.work);
        naturally_quiescent &= run.naturally_quiescent;
    }
    let correspondence: [Vec<ArrowId>; SIDES] = std::array::from_fn(|side| {
        world
            .substrate
            .arrows_between(world.sources[side], world.endpoints[side])
    });
    assert!(correspondence.iter().all(|arrows| arrows.len() == 1));

    for side in 0..SIDES {
        enter_many(
            &mut world.substrate,
            world.sites[side],
            64,
            2,
            namespace + 0x3_000 + side as u64 * 0x10,
        );
    }
    let prime = world.substrate.propagate();
    add_work(&mut work, &prime.work);
    naturally_quiescent &= prime.naturally_quiescent;
    let role_arrows: [Vec<ArrowId>; SIDES] = std::array::from_fn(|side| {
        world
            .substrate
            .arrows_between(world.sites[side], world.outlets[side])
    });
    assert!(role_arrows.iter().all(|arrows| arrows.len() == 1));
    let brake_arrows: [Vec<ArrowId>; SIDES] = std::array::from_fn(|side| {
        world.brakes[side].map_or_else(Vec::new, |brake| {
            world.substrate.arrows_between(world.sites[side], brake)
        })
    });

    let mut source_returns = 0usize;
    let mut site_returns = 0usize;
    let mut source_firings = 0usize;
    let mut brake_firings = 0usize;
    for exposure in 0..EXPOSURES {
        let tick = 66 + exposure as i64 * 12;
        for side in 0..SIDES {
            enter_many(
                &mut world.substrate,
                world.sources[side],
                tick,
                world.source_threshold,
                namespace + 0x4_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
            );
        }
        enter_many(
            &mut world.substrate,
            world.role_drivers[supported],
            tick,
            1,
            namespace + 0x5_000 + exposure as u64 * 0x100,
        );
        let run = world.substrate.propagate();
        for side in 0..SIDES {
            source_returns += positive_trace_arrivals(&run, source_physical(namespace, side))
                .saturating_sub(world.source_threshold);
            source_firings += trace_firings(&run, source_physical(namespace, side));
            if arm == Arm::Inhibition {
                brake_firings += trace_firings(&run, brake_physical(namespace, side));
            }
        }
        site_returns += positive_trace_arrivals(&run, site_physical(arm, namespace, supported))
            .saturating_sub(1);
        add_work(&mut work, &run.work);
        naturally_quiescent &= run.naturally_quiescent;
    }
    let extra_source_firings = source_firings.saturating_sub(EXPOSURES * SIDES);
    let role_resistance =
        std::array::from_fn(|side| max_resistance(&world.substrate, &role_arrows[side]));
    let brake_resistance =
        std::array::from_fn(|side| max_resistance(&world.substrate, &brake_arrows[side]));
    let role_formed = role_resistance[supported] > 1 && role_resistance[1 - supported] == 0;
    let test_tick = 170;
    let effects = std::array::from_fn(|side| {
        let mut clone = world.clone();
        let run = activate_source(
            &mut clone,
            side,
            test_tick,
            namespace + 0x6_000 + side as u64 * 0x100,
        );
        naturally_quiescent &= run.naturally_quiescent;
        outward_effects(&run, namespace, side)
    });
    let mut first_clone = world.clone();
    let mut second_clone = world.clone();
    let first = activate_source(&mut first_clone, supported, test_tick, namespace + 0x7_000);
    let second = activate_source(&mut second_clone, supported, test_tick, namespace + 0x7_000);
    let duplicate_exact = first == second
        && first_clone.substrate.complete_fingerprint()
            == second_clone.substrate.complete_fingerprint();
    naturally_quiescent &= first.naturally_quiescent && second.naturally_quiescent;

    let mut recurrence = world.clone();
    add_work(&mut work, &recurrence.substrate.advance_time(180));
    let recurrence_run = activate_source(&mut recurrence, supported, 180, namespace + 0x8_000);
    let productive_recurrence = outward_effects(&recurrence_run, namespace, supported) == 1
        && positive_trace_arrivals(&recurrence_run, source_physical(namespace, supported))
            .saturating_sub(world.source_threshold)
            > 0
        && positive_trace_arrivals(&recurrence_run, site_physical(arm, namespace, supported))
            .saturating_sub(1)
            > 0
        && recurrence_run.naturally_quiescent;
    naturally_quiescent &= recurrence_run.naturally_quiescent;
    add_work(&mut work, &recurrence_run.work);
    let fingerprint = world.substrate.complete_fingerprint();

    WorldMetrics {
        role_resistance,
        effects,
        source_returns,
        site_returns,
        extra_source_firings,
        brake_resistance,
        brake_firings,
        role_formed,
        productive_recurrence,
        duplicate_exact,
        naturally_quiescent,
        work,
        fingerprint,
    }
}

fn build_world(arm: Arm, namespace: u64, mirror: bool, reverse: bool) -> World {
    let mut substrate = PlasticSubstrate::new();
    let source_threshold = if arm == Arm::Margin { 4 } else { 3 };
    let mut sources = [None; SIDES];
    let mut endpoints = [None; SIDES];
    let mut sites = [None; SIDES];
    let mut outlets = [None; SIDES];
    let mut acquisition_drivers = [None; SIDES];
    let mut role_drivers = [None; SIDES];
    let mut correspondence_gates = [None; SIDES];
    let mut role_gates = [None; SIDES];
    let mut outside = [None; SIDES];
    let mut brakes = [None; SIDES];
    let order = if reverse { [1, 0] } else { [0, 1] };
    for side in order {
        let slot = if mirror { SIDES - 1 - side } else { side };
        let base = slot as i32 * 32;
        let source = substrate.add_cell(cell(
            namespace + 10 + side as u64,
            base,
            0,
            source_threshold,
        ));
        let endpoint = substrate.add_cell(cell(namespace + 20 + side as u64, base + 2, 0, 2));
        let site = if arm == Arm::Distance {
            substrate.add_cell(cell(namespace + 25 + side as u64, base + 8, 0, 2))
        } else {
            endpoint
        };
        let site_position = if arm == Arm::Distance {
            base + 8
        } else {
            base + 2
        };
        let outlet =
            substrate.add_cell(cell(namespace + 30 + side as u64, site_position + 2, 0, 2));
        sources[side] = Some(source);
        endpoints[side] = Some(endpoint);
        sites[side] = Some(site);
        outlets[side] = Some(outlet);
        acquisition_drivers[side] = Some(substrate.add_cell(cell(
            namespace + 40 + side as u64,
            1_000 + side as i32 * 20,
            0,
            1,
        )));
        role_drivers[side] = Some(substrate.add_cell(cell(
            namespace + 50 + side as u64,
            1_100 + side as i32 * 20,
            0,
            1,
        )));
        correspondence_gates[side] = Some(substrate.add_cell(cell(
            namespace + 60 + side as u64,
            1_200 + side as i32 * 20,
            0,
            1,
        )));
        role_gates[side] = Some(substrate.add_cell(cell(
            namespace + 70 + side as u64,
            1_300 + side as i32 * 20,
            0,
            1,
        )));
        outside[side] = Some(substrate.add_cell(cell(
            namespace + 80 + side as u64,
            1_400 + side as i32 * 20,
            1,
            1,
        )));
        if arm == Arm::Inhibition {
            brakes[side] =
                Some(substrate.add_cell(cell(namespace + 90 + side as u64, base + 3, 0, 2)));
        }
    }
    let sources = sources.map(|value| value.expect("source"));
    let endpoints = endpoints.map(|value| value.expect("endpoint"));
    let sites = sites.map(|value| value.expect("site"));
    let outlets = outlets.map(|value| value.expect("outlet"));
    let acquisition_drivers = acquisition_drivers.map(|value| value.expect("acquisition driver"));
    let role_drivers = role_drivers.map(|value| value.expect("role driver"));
    let correspondence_gates =
        correspondence_gates.map(|value| value.expect("correspondence gate"));
    let role_gates = role_gates.map(|value| value.expect("role gate"));
    let outside = outside.map(|value| value.expect("outside"));
    for side in order {
        substrate.add_arrow(arrow(acquisition_drivers[side], endpoints[side], 2, 1));
        substrate.add_arrow(arrow(endpoints[side], correspondence_gates[side], 1, 1));
        substrate.add_arrow(arrow(correspondence_gates[side], sources[side], 1, 1));
        if arm == Arm::Distance {
            substrate.add_arrow(arrow(endpoints[side], sites[side], 2, 2));
        }
        let driver_delay = if arm == Arm::Distance { 6 } else { 4 };
        substrate.add_arrow(arrow(role_drivers[side], outlets[side], driver_delay, 1));
        let return_leg = if arm == Arm::Timing { 3 } else { 1 };
        substrate.add_arrow(arrow(outlets[side], role_gates[side], return_leg, 1));
        substrate.add_arrow(arrow(role_gates[side], sites[side], return_leg, 1));
        substrate.add_arrow(arrow(outlets[side], outside[side], 0, 1));
        if let Some(brake) = brakes[side] {
            substrate.add_arrow(arrow(brake, sources[side], 0, -4));
        }
    }
    World {
        substrate,
        source_threshold: source_threshold as usize,
        sources,
        endpoints,
        sites,
        outlets,
        acquisition_drivers,
        role_drivers,
        brakes,
    }
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 1_000,
    }
}

fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 1_000,
    }
}

fn enter_many(
    substrate: &mut PlasticSubstrate,
    target: CellId,
    tick: i64,
    count: usize,
    origin: u64,
) {
    for ordinal in 0..count {
        substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: ordinal as i32,
            origin_physical: origin + ordinal as u64,
            target,
            impulse: 1,
        });
    }
}

fn activate_source(world: &mut World, side: usize, tick: i64, origin: u64) -> Execution {
    enter_many(
        &mut world.substrate,
        world.sources[side],
        tick,
        world.source_threshold,
        origin,
    );
    world.substrate.propagate()
}

fn source_physical(namespace: u64, side: usize) -> u64 {
    namespace + 10 + side as u64
}

fn site_physical(arm: Arm, namespace: u64, side: usize) -> u64 {
    if arm == Arm::Distance {
        namespace + 25 + side as u64
    } else {
        namespace + 20 + side as u64
    }
}

fn brake_physical(namespace: u64, side: usize) -> u64 {
    namespace + 90 + side as u64
}

fn outward_effects(run: &Execution, namespace: u64, side: usize) -> usize {
    run.crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == namespace + 30 + side as u64
                && crossing.to_physical == namespace + 80 + side as u64
        })
        .count()
}

fn positive_trace_arrivals(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.impulse > 0)
        .count()
}

fn trace_firings(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn max_resistance(substrate: &PlasticSubstrate, arrows: &[ArrowId]) -> u32 {
    arrows
        .iter()
        .filter(|arrow| substrate.arrow_is_live(**arrow))
        .map(|arrow| substrate.arrow_resistance(*arrow))
        .max()
        .unwrap_or(0)
}

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
    total.ordinary_pressure_updates += next.ordinary_pressure_updates;
    total.local_structural_proposals += next.local_structural_proposals;
    total.physical_deallocations += next.physical_deallocations;
}

fn work_from_total(total: u64) -> WorkLedger {
    WorkLedger {
        state_updates: total,
        ..WorkLedger::default()
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_u64(value: &str) -> u64 {
    value.parse().expect("u64 field")
}

fn parse_u32(value: &str) -> u32 {
    value.parse().expect("u32 field")
}

fn parse_usize(value: &str) -> usize {
    value.parse().expect("usize field")
}

fn sha256(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("run sha256sum");
    assert!(output.status.success(), "hash {path}");
    String::from_utf8(output.stdout)
        .expect("utf8 hash")
        .split_whitespace()
        .next()
        .expect("hash")
        .to_string()
}

fn csv(results: &[ArmResult]) -> String {
    let mut text = String::from(
        "arm,completed,timed_out,primary_role,primary_role_r0,primary_role_r1,primary_effect0,primary_effect1,primary_source_returns,primary_site_returns,primary_extra_source_firings,primary_brake_r,primary_brake_firings,primary_productive_recurrence,primary_replay,primary_quiescent,primary_work,primary_fingerprint,transfer_role,transfer_role_r0,transfer_role_r1,transfer_effect0,transfer_effect1,transfer_source_returns,transfer_site_returns,transfer_extra_source_firings,transfer_brake_r,transfer_brake_firings,transfer_productive_recurrence,transfer_replay,transfer_quiescent,transfer_work,transfer_fingerprint,passed\n",
    );
    for result in results {
        let fields = result.encode().replace("PX1_ARM|", "").replace('|', ",");
        text.push_str(&fields);
        text.push('\n');
    }
    text
}

fn markdown(results: &[ArmResult]) -> String {
    let passed = results.iter().filter(|result| result.passed).count();
    let classification = match passed {
        0 => "NO PHYSICAL ARM PRESERVED LEARNING + RECURRENCE + QUIESCENCE",
        1 => "ONE UNIQUE PHYSICAL ARM SUPPORTED",
        _ => "MULTIPLE PHYSICAL ARMS SUPPORTED; SCIENTIFIC AMBIGUITY REMAINS",
    };
    let mut text = format!(
        "# PX1 recurrent role-stability diagnostic v1\n\nClassification: **{classification}**.\n\n- completed arms: `{}/4`\n- timed-out arms: `{}`\n- passing arms: `{passed}/4`\n- PX1 authoritative: `false`\n- definitive evidence executed: `false`\n\n",
        results.iter().filter(|result| result.completed).count(),
        results.iter().filter(|result| result.timed_out).count(),
    );
    text.push_str("| arm | role primary/transfer | effects primary | effects transfer | source refires primary/transfer | productive recurrence | quiescent | pass |\n");
    text.push_str("|---|---:|---:|---:|---:|---:|---:|:---:|\n");
    for result in results {
        let primary = result.primary.as_ref();
        let transfer = result.transfer.as_ref();
        text.push_str(&format!(
            "| {} | {}/{} | {}/{} | {}/{} | {}/{} | {}/{} | {}/{} | {} |\n",
            result.arm.name(),
            primary.is_some_and(|value| value.role_formed),
            transfer.is_some_and(|value| value.role_formed),
            primary.map_or(0, |value| value.effects[0]),
            primary.map_or(0, |value| value.effects[1]),
            transfer.map_or(0, |value| value.effects[0]),
            transfer.map_or(0, |value| value.effects[1]),
            primary.map_or(0, |value| value.extra_source_firings),
            transfer.map_or(0, |value| value.extra_source_firings),
            primary.is_some_and(|value| value.productive_recurrence),
            transfer.is_some_and(|value| value.productive_recurrence),
            primary.is_some_and(|value| value.naturally_quiescent),
            transfer.is_some_and(|value| value.naturally_quiescent),
            result.passed,
        ));
    }
    text.push_str("\nA quiet arm without learned role structure is negative. Multiple passing arms do not authorize choosing one by convenience; they freeze a remaining scientific ambiguity.\n");
    text
}

fn write_new(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create diagnostic artifact");
    file.write_all(contents.as_bytes())
        .expect("write diagnostic artifact");
    file.sync_all().expect("sync diagnostic artifact");
}
