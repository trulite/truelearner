use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const SIDES: usize = 2;
const ACQUISITION: usize = 4;
const ROLE_EXPOSURES: usize = 8;
const PX0_AUTHORITY_COMMIT: &str = "e884ae133a562d475565a36700d929b51dd2b2d2";
const PX0_AUTHORITY_TAG: &str = "px0-substrate-native-physical-correspondence-authoritative^{}";
const PX0_SOURCE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX0_CSV_SHA256: &str = "b750792123de1c0aa7d3104d2d1bcd3fdc6e26a70e54b10f5eedf320fe7d95c9";
const PX0_MD_SHA256: &str = "6bf27bb98cf3f2ca821918daa966722c3be9a31c1de6b589565d25539b3c702d";
const PX0_AUDIT_SHA256: &str = "ed2f2d95f4876d58e419ef220daa5de6b28bbf9d343b84c188099cd9fbd0c5d8";
const PX0_HANDOFF_SHA256: &str = "3975ca373ae4ede05230b732ae0651b1851ab84bc71cb1afd6033a7e76a9e160";
const PROTOCOL_SHA256: &str = "39777626848a61d0e3f9d13a9f778fdb22fed588c5f2a89a657636ee2428e3e9";
const RESULT_CSV: &str = "results/px1_physical_boundary_roles_probe_v1.csv";
const RESULT_MD: &str = "results/px1_physical_boundary_roles_probe_v1.md";

#[derive(Clone, Copy)]
enum ReturnSchedule {
    Neither,
    First,
    Second,
    Both,
}

impl ReturnSchedule {
    fn supplies(self, side: usize) -> bool {
        match self {
            Self::Neither => false,
            Self::First => side == 0,
            Self::Second => side == 1,
            Self::Both => true,
        }
    }
}

#[derive(Clone)]
struct PhysicalWorld {
    substrate: PlasticSubstrate,
    namespace: u64,
    sources: [CellId; SIDES],
    endpoints: [CellId; SIDES],
    outlets: [CellId; SIDES],
    correspondence_drivers: [CellId; SIDES],
    return_drivers: [CellId; SIDES],
}

#[derive(Clone, Debug)]
struct WorldObservation {
    correspondence_arrows: [Vec<ArrowId>; SIDES],
    opportunity_arrows: [Vec<ArrowId>; SIDES],
    correspondence_resistance: [u32; SIDES],
    opportunity_resistance: [u32; SIDES],
    endpoint_firings: [usize; SIDES],
    held_out_effects: [usize; SIDES],
    paired_effects: [usize; SIDES],
    no_direct_bypass: bool,
    correspondence_ids_unchanged: bool,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    work: WorkLedger,
    naturally_quiescent: bool,
    replay_exact: bool,
}

#[derive(Clone, Debug)]
struct ProbeResult {
    source_exact: bool,
    main: WorldObservation,
    reversed: WorldObservation,
    absent: WorldObservation,
    equal: WorldObservation,
    broken: WorldObservation,
    stages: [bool; 11],
    first_collapse: String,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args != ["--probe"] {
        eprintln!("PX1 is development-only; use --probe. Definitive execution is forbidden.");
        std::process::exit(2);
    }
    assert!(!Path::new(RESULT_CSV).exists(), "PROBE CSV already exists");
    assert!(
        !Path::new(RESULT_MD).exists(),
        "PROBE report already exists"
    );

    eprintln!("PX1_PHYSICAL_BOUNDARY_ROLE_PROBE_DEVELOPMENT_EVIDENCE");
    let result = run_probe();
    write_new(RESULT_CSV, &csv(&result));
    write_new(RESULT_MD, &markdown(&result));
    std::process::exit(if result.stages.iter().all(|stage| *stage) {
        0
    } else {
        1
    });
}

fn run_probe() -> ProbeResult {
    let source_exact = source_audit();
    let main = observe_world(0x2100_0000, ReturnSchedule::First, false, false, false);
    let reversed = observe_world(0x2200_0000, ReturnSchedule::Second, true, true, false);
    let absent = observe_world(0x2300_0000, ReturnSchedule::Neither, true, false, false);
    let equal = observe_world(0x2400_0000, ReturnSchedule::Both, false, true, false);
    let broken = observe_world(0x2500_0000, ReturnSchedule::First, true, true, true);

    let main_correspondence = main
        .correspondence_arrows
        .iter()
        .all(|arrows| arrows.len() == 1)
        && main
            .correspondence_resistance
            .iter()
            .all(|value| *value > 1);
    let opportunities = main
        .opportunity_arrows
        .iter()
        .all(|arrows| arrows.len() == 1)
        && main.no_direct_bypass;
    let direct_traversal = main.endpoint_firings == [ROLE_EXPOSURES, ROLE_EXPOSURES]
        && main.correspondence_ids_unchanged;
    let differential = main.opportunity_resistance[0] > main.opportunity_resistance[1]
        && main.opportunity_resistance[0] > 1
        && main.opportunity_resistance[1] == 0;
    let main_behavior = main.held_out_effects == [1, 0] && main.paired_effects == [1, 0];
    let reversed_behavior = reversed.held_out_effects == [0, 1]
        && reversed.paired_effects == [0, 1]
        && reversed.opportunity_resistance[1] > reversed.opportunity_resistance[0];
    let absent_control = absent.held_out_effects == [0, 0]
        && absent.paired_effects == [0, 0]
        && absent.opportunity_resistance == [0, 0];
    let equal_control = equal.held_out_effects == [1, 1]
        && equal.opportunity_resistance[0] > 1
        && equal.opportunity_resistance[1] > 1;
    let broken_control = broken.correspondence_arrows.iter().all(Vec::is_empty)
        && broken.endpoint_firings == [0, 0]
        && broken.held_out_effects == [0, 0]
        && broken.paired_effects == [0, 0]
        && broken.no_direct_bypass;
    let replay_and_quiescence = [&main, &reversed, &absent, &equal, &broken]
        .into_iter()
        .all(|world| world.replay_exact && world.naturally_quiescent && world.work.total() > 0);
    let stages = [
        source_exact,
        main_correspondence,
        opportunities,
        direct_traversal,
        differential,
        main_behavior,
        reversed_behavior,
        absent_control,
        equal_control,
        broken_control,
        replay_and_quiescence,
    ];
    let names = [
        "frozen authoritative PX0 source and artifacts",
        "both retained PX0 correspondences form",
        "identical broad endpoint opportunities form without direct bypass",
        "source activity traverses pre-existing PX0 arrows into both endpoints",
        "returned activity differentially stabilizes endpoint-local structure",
        "held-out paired source activity uses only the supported side",
        "fresh mirrored reverse world reverses retained behavior",
        "return-absent control remains non-executable",
        "equal-return control retains both alternatives",
        "broken PX0 path cannot produce source-driven role behavior",
        "natural quiescence and exact replay",
    ];
    let first_collapse = stages
        .iter()
        .position(|stage| !stage)
        .map(|index| names[index].to_string())
        .unwrap_or_else(|| "NONE: PX1 PROBE development positive".to_string());
    ProbeResult {
        source_exact,
        main,
        reversed,
        absent,
        equal,
        broken,
        stages,
        first_collapse,
    }
}

fn source_audit() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == PX0_SOURCE_SHA256
        && sha256("results/px0_physical_correspondence_definitive_v3.csv") == PX0_CSV_SHA256
        && sha256("results/px0_physical_correspondence_definitive_v3.md") == PX0_MD_SHA256
        && sha256("experiments/px0_physical_correspondence_definitive_v3_result_audit.md")
            == PX0_AUDIT_SHA256
        && sha256("experiments/px0_physical_correspondence_definitive_v3_authority_handoff.md")
            == PX0_HANDOFF_SHA256
        && sha256("experiments/px1_physical_boundary_roles_probe_protocol.md") == PROTOCOL_SHA256
        && git_output(&["rev-parse", PX0_AUTHORITY_TAG])
            .is_some_and(|commit| commit == PX0_AUTHORITY_COMMIT)
        && active_source_is_substrate_only()
}

fn active_source_is_substrate_only() -> bool {
    let source =
        fs::read_to_string("crates/px0-physical-correspondence/src/lib.rs").unwrap_or_default();
    let forbidden = [
        "RelationMotif",
        "Neighborhood",
        "Episode",
        "History",
        "Query",
        "ProductiveHistory",
        "ContrastHistory",
        "BoundaryRole",
        "Filler",
        "CORRECT",
        "WRONG",
        "START",
        "FINISH",
    ];
    forbidden.iter().all(|token| !source.contains(token))
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
        .expect("hash field")
        .to_string()
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output.status.success().then(|| {
        String::from_utf8(output.stdout)
            .expect("git utf8")
            .trim()
            .to_string()
    })
}

fn observe_world(
    namespace: u64,
    schedule: ReturnSchedule,
    mirror: bool,
    reverse_allocation: bool,
    broken: bool,
) -> WorldObservation {
    let mut world = build_world(namespace, mirror, reverse_allocation, broken);
    let mut work = WorkLedger::default();
    let mut naturally_quiescent = true;

    for exposure in 0..ACQUISITION {
        let tick = exposure as i64 * 16;
        for side in 0..SIDES {
            enter_many(
                &mut world.substrate,
                world.sources[side],
                tick,
                3,
                namespace + 0x1_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
            );
            enter_many(
                &mut world.substrate,
                world.correspondence_drivers[side],
                tick,
                1,
                namespace + 0x2_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
            );
        }
        let run = world.substrate.propagate();
        add_work(&mut work, &run.work);
        naturally_quiescent &= run.naturally_quiescent;
    }
    let correspondence_arrows = std::array::from_fn(|side| {
        world
            .substrate
            .arrows_between(world.sources[side], world.endpoints[side])
    });

    let prime_tick = 64;
    for side in 0..SIDES {
        enter_many(
            &mut world.substrate,
            world.endpoints[side],
            prime_tick,
            2,
            namespace + 0x3_000 + side as u64 * 0x10,
        );
    }
    let prime = world.substrate.propagate();
    add_work(&mut work, &prime.work);
    naturally_quiescent &= prime.naturally_quiescent;
    let opportunity_arrows = std::array::from_fn(|side| {
        world
            .substrate
            .arrows_between(world.endpoints[side], world.outlets[side])
    });
    let no_direct_bypass = (0..SIDES).all(|side| {
        world
            .substrate
            .arrows_between(world.sources[side], world.outlets[side])
            .is_empty()
    });

    let mut endpoint_firings = [0usize; SIDES];
    for exposure in 0..ROLE_EXPOSURES {
        let tick = 66 + exposure as i64 * 12;
        for side in 0..SIDES {
            enter_many(
                &mut world.substrate,
                world.sources[side],
                tick,
                3,
                namespace + 0x4_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
            );
            if schedule.supplies(side) {
                enter_many(
                    &mut world.substrate,
                    world.return_drivers[side],
                    tick,
                    1,
                    namespace + 0x5_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
                );
            }
        }
        let run = world.substrate.propagate();
        for (side, firings) in endpoint_firings.iter_mut().enumerate() {
            *firings += run
                .trace
                .iter()
                .filter(|entry| {
                    entry.target_physical == endpoint_physical(namespace, side) && entry.fired
                })
                .count();
        }
        add_work(&mut work, &run.work);
        naturally_quiescent &= run.naturally_quiescent;
    }

    if matches!(schedule, ReturnSchedule::Neither) {
        let pressure = world.substrate.advance_time(260);
        add_work(&mut work, &pressure);
    }
    let correspondence_ids_unchanged = (0..SIDES).all(|side| {
        world
            .substrate
            .arrows_between(world.sources[side], world.endpoints[side])
            == correspondence_arrows[side]
    });
    let correspondence_resistance =
        std::array::from_fn(|side| max_resistance(&world.substrate, &correspondence_arrows[side]));
    let opportunity_resistance =
        std::array::from_fn(|side| max_resistance(&world.substrate, &opportunity_arrows[side]));
    let complete_fingerprint = world.substrate.complete_fingerprint();
    let permanent_fingerprint = world.substrate.permanent_fingerprint();

    let test_tick = if matches!(schedule, ReturnSchedule::Neither) {
        260
    } else {
        162
    };
    let held_out_effects = std::array::from_fn(|side| {
        let mut clone = world.clone();
        let run = activate_sources(
            &mut clone,
            &[side],
            test_tick,
            namespace + 0x6_000 + side as u64 * 0x100,
        );
        naturally_quiescent &= run.naturally_quiescent;
        role_effects(&run, &clone, side)
    });
    let mut paired_first = world.clone();
    let mut paired_second = world.clone();
    let first = activate_sources(&mut paired_first, &[0, 1], test_tick, namespace + 0x7_000);
    let second = activate_sources(&mut paired_second, &[0, 1], test_tick, namespace + 0x7_000);
    naturally_quiescent &= first.naturally_quiescent && second.naturally_quiescent;
    let paired_effects = std::array::from_fn(|side| role_effects(&first, &paired_first, side));
    let replay_exact = first == second
        && paired_first.substrate.complete_fingerprint()
            == paired_second.substrate.complete_fingerprint();

    WorldObservation {
        correspondence_arrows,
        opportunity_arrows,
        correspondence_resistance,
        opportunity_resistance,
        endpoint_firings,
        held_out_effects,
        paired_effects,
        no_direct_bypass,
        correspondence_ids_unchanged,
        complete_fingerprint,
        permanent_fingerprint,
        work,
        naturally_quiescent,
        replay_exact,
    }
}

fn build_world(namespace: u64, mirror: bool, reverse: bool, broken: bool) -> PhysicalWorld {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; SIDES];
    let mut endpoints = [None; SIDES];
    let mut outlets = [None; SIDES];
    let mut correspondence_drivers = [None; SIDES];
    let mut return_drivers = [None; SIDES];
    let mut outside = [None; SIDES];
    let mut correspondence_gates = [None; SIDES];
    let mut return_gates = [None; SIDES];
    let order = if reverse { [1, 0] } else { [0, 1] };
    for side in order {
        let slot = if mirror { SIDES - 1 - side } else { side };
        let base = slot as i32 * 24;
        let endpoint_offset = if broken { 4 } else { 2 };
        sources[side] = Some(substrate.add_cell(cell(namespace + 10 + side as u64, base, 0, 3)));
        endpoints[side] = Some(substrate.add_cell(cell(
            namespace + 20 + side as u64,
            base + endpoint_offset,
            0,
            2,
        )));
        outlets[side] = Some(substrate.add_cell(cell(
            namespace + 30 + side as u64,
            base + endpoint_offset + 2,
            0,
            2,
        )));
        correspondence_drivers[side] = Some(substrate.add_cell(cell(
            namespace + 40 + side as u64,
            1_000 + side as i32 * 20,
            0,
            1,
        )));
        return_drivers[side] = Some(substrate.add_cell(cell(
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
        return_gates[side] = Some(substrate.add_cell(cell(
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
    }
    let sources = sources.map(|cell| cell.expect("source"));
    let endpoints = endpoints.map(|cell| cell.expect("endpoint"));
    let outlets = outlets.map(|cell| cell.expect("outlet"));
    let correspondence_drivers = correspondence_drivers.map(|cell| cell.expect("driver"));
    let return_drivers = return_drivers.map(|cell| cell.expect("return driver"));
    let correspondence_gates = correspondence_gates.map(|cell| cell.expect("gate"));
    let return_gates = return_gates.map(|cell| cell.expect("return gate"));
    let outside = outside.map(|cell| cell.expect("outside"));
    for side in order {
        substrate.add_arrow(arrow(correspondence_drivers[side], endpoints[side], 2, 1));
        substrate.add_arrow(arrow(endpoints[side], correspondence_gates[side], 1, 1));
        substrate.add_arrow(arrow(correspondence_gates[side], sources[side], 1, 1));
        substrate.add_arrow(arrow(return_drivers[side], outlets[side], 4, 1));
        substrate.add_arrow(arrow(outlets[side], return_gates[side], 1, 1));
        substrate.add_arrow(arrow(return_gates[side], endpoints[side], 1, 1));
        substrate.add_arrow(arrow(outlets[side], outside[side], 0, 1));
    }
    PhysicalWorld {
        substrate,
        namespace,
        sources,
        endpoints,
        outlets,
        correspondence_drivers,
        return_drivers,
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

fn activate_sources(
    world: &mut PhysicalWorld,
    sides: &[usize],
    tick: i64,
    origin: u64,
) -> Execution {
    for side in sides {
        enter_many(
            &mut world.substrate,
            world.sources[*side],
            tick,
            3,
            origin + *side as u64 * 0x10,
        );
    }
    world.substrate.propagate()
}

fn endpoint_physical(namespace: u64, side: usize) -> u64 {
    namespace + 20 + side as u64
}

fn role_effects(execution: &Execution, world: &PhysicalWorld, side: usize) -> usize {
    let from = world.namespace + 30 + side as u64;
    let to = world.namespace + 80 + side as u64;
    execution
        .crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
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

fn csv(result: &ProbeResult) -> String {
    let mut text = String::from(
        "world,corr0,corr1,opp0,opp1,corr_r0,corr_r1,opp_r0,opp_r1,endpoint_fires0,endpoint_fires1,held0,held1,paired0,paired1,no_bypass,corr_ids_unchanged,replay,quiescent,work,complete_fingerprint,permanent_fingerprint\n",
    );
    for (name, world) in [
        ("main", &result.main),
        ("reversed", &result.reversed),
        ("absent", &result.absent),
        ("equal", &result.equal),
        ("broken", &result.broken),
    ] {
        text.push_str(&format!(
            "{name},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            world.correspondence_arrows[0].len(),
            world.correspondence_arrows[1].len(),
            world.opportunity_arrows[0].len(),
            world.opportunity_arrows[1].len(),
            world.correspondence_resistance[0],
            world.correspondence_resistance[1],
            world.opportunity_resistance[0],
            world.opportunity_resistance[1],
            world.endpoint_firings[0],
            world.endpoint_firings[1],
            world.held_out_effects[0],
            world.held_out_effects[1],
            world.paired_effects[0],
            world.paired_effects[1],
            world.no_direct_bypass,
            world.correspondence_ids_unchanged,
            world.replay_exact,
            world.naturally_quiescent,
            world.work.total(),
            world.complete_fingerprint,
            world.permanent_fingerprint,
        ));
    }
    text
}

fn markdown(result: &ProbeResult) -> String {
    let passed = result.stages.iter().all(|stage| *stage);
    let mut text = format!(
        "# PX1 substrate-native physical boundary-role PROBE v1\n\nOutcome: **{}**.\n\n- frozen PX0 exact: `{}`\n- stages: `{}/11`\n- first collapse: `{}`\n- PX1 authoritative: `false`\n- definitive evidence executed: `false`\n\n",
        if passed { "DEVELOPMENT POSITIVE" } else { "FROZEN DEVELOPMENT NEGATIVE" },
        result.source_exact,
        result.stages.iter().filter(|stage| **stage).count(),
        result.first_collapse,
    );
    text.push_str("| stage | pass |\n|---:|:---:|\n");
    for (index, stage) in result.stages.iter().enumerate() {
        text.push_str(&format!("| P{index} | {stage} |\n"));
    }
    text.push_str("\n| world | correspondence R | opportunity R | endpoint firings | held-out | paired | replay | quiescent |\n");
    text.push_str("|---|---:|---:|---:|---:|---:|:---:|:---:|\n");
    for (name, world) in [
        ("main", &result.main),
        ("reversed", &result.reversed),
        ("absent", &result.absent),
        ("equal", &result.equal),
        ("broken", &result.broken),
    ] {
        text.push_str(&format!(
            "| {name} | {}/{} | {}/{} | {}/{} | {}/{} | {}/{} | {} | {} |\n",
            world.correspondence_resistance[0],
            world.correspondence_resistance[1],
            world.opportunity_resistance[0],
            world.opportunity_resistance[1],
            world.endpoint_firings[0],
            world.endpoint_firings[1],
            world.held_out_effects[0],
            world.held_out_effects[1],
            world.paired_effects[0],
            world.paired_effects[1],
            world.replay_exact,
            world.naturally_quiescent,
        ));
    }
    text.push_str("\nThis PROBE uses no old M0/M1 learner, typed relation object, topology serializer, or evaluator-to-learner path. PX0 remains authoritative; PX1 remains development-only.\n");
    text
}

fn write_new(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create new PROBE artifact");
    file.write_all(contents.as_bytes())
        .expect("write PROBE artifact");
    file.sync_all().expect("sync PROBE artifact");
}
