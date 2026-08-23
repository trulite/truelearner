use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const N: usize = 3;
const CELLS: usize = 16;
const DEVICES: usize = 4;
const SCAFFOLD: u32 = 1_000;
const BASE_NAMESPACE: u64 = 0x1_2000_0000;
const NAMESPACE_STRIDE: u64 = 0x10_0000;
const ACTIVE_LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const RETAINED_PHYSICS_SHA256: &str =
    "6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263";
const V1_CSV_SHA256: &str = "da356bc46a9d83d0cd749bcaa697cba66393b7d694de500e2208565806d680d1";
const V1_MD_SHA256: &str = "7e2c06d63332a680d46031c49d5dc245c6a4f381d7c646a2b0474580469a09b7";
const V1_AUDIT_SHA256: &str = "575da84eb9ced6c48309f1f15ef75dd19fd99e8c112c6bb37f3d5be7bce68a14";
const V1_HANDOFF_SHA256: &str = "aa38d12edb1d83a1249f76c24c6bd6e5ac7af6c4569c4f2a6ff97cca7f2b29d8";
const P1_AUDIT_SHA256: &str = "e9c92d461fb53898fe2c01c3ef3a06633ca3b98d245c2095bb758e6dcbe13c78";
const P1_HANDOFF_SHA256: &str = "a9b2272b968755ff0503efc65e898c0a211a224cd8684422f35818b5f58fc906";
const S_PROTOCOL_SHA256: &str = "3f14e74de3c331eb6657e09077be147ed83cebb5325ea9e11d263230c623e30f";
const S_PROBE_AUDIT_SHA256: &str =
    "51174002f79c281ba98f49c921f788d14c7389ab558c332caa12a8dcb0eeada2";
const S_READINESS_SHA256: &str = "a3ba35f6755b0363db809575182296d9cead69a30bb0842a527c4ae4389fcad7";
const DEFINITIVE_PROTOCOL_SHA256: &str =
    "899819b4605f811211916da447c211604dfd7757b9f7b94aa87084c2ed0e534d";
const READINESS_COMMIT: &str = "a77a564f741e280bb674ff6e1cb6a28df4d80790";
const READINESS_TAG: &str = "px0-s-stable-return-specificity-development-classification-a^{}";
const FINAL_CSV: &str = "results/px0_physical_correspondence_definitive_v2.csv";
const FINAL_MD: &str = "results/px0_physical_correspondence_definitive_v2.md";
const STAGING_CSV: &str = "results/.px0_physical_correspondence_definitive_v2.csv.staging";
const STAGING_MD: &str = "results/.px0_physical_correspondence_definitive_v2.md.staging";

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
    dense_drivers: [CellId; N],
}

#[derive(Clone, Debug)]
struct Preflight {
    active_law_exact: bool,
    retained_physics_exact: bool,
    v1_negative_exact: bool,
    p1_exact: bool,
    specificity_exact: bool,
    protocol_exact: bool,
    readiness_tag_exact: bool,
    dependency_surface_empty: bool,
    source_isolated: bool,
    outputs_absent: bool,
    staging_absent: bool,
}

impl Preflight {
    fn passed(&self) -> bool {
        self.active_law_exact
            && self.retained_physics_exact
            && self.v1_negative_exact
            && self.p1_exact
            && self.specificity_exact
            && self.protocol_exact
            && self.readiness_tag_exact
            && self.dependency_surface_empty
            && self.source_isolated
            && self.outputs_absent
            && self.staging_absent
    }
}

#[derive(Clone, Debug)]
struct CellResult {
    index: usize,
    namespace: u64,
    initial_route: usize,
    reacquired_route: usize,
    spacing: i64,
    active_opportunities: usize,
    reverse_allocation: bool,
    mirrored_layout: bool,
    initial_effects: usize,
    survival_effects: usize,
    reacquired_effects: usize,
    historical_effects: usize,
    sparse_effects: usize,
    stable_dense_effects: usize,
    swapped_stable_effects: usize,
    swapped_sparse_effects: usize,
    return_free_effects: usize,
    absent_effects: usize,
    ambiguous_effects: usize,
    proposals: u64,
    deallocations: u64,
    work: u64,
    persistent_bytes: usize,
    old_arrow_count: usize,
    fresh_arrow_count: usize,
    p: [bool; 13],
}

impl CellResult {
    fn passed(&self) -> bool {
        self.p.iter().all(|value| *value)
    }
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args == ["--preflight-v2"] {
        let preflight = source_preflight();
        println!(
            "PX0 definitive v2 no-cell preflight: {}",
            preflight.passed()
        );
        if !preflight.passed() {
            std::process::exit(2);
        }
        return;
    }
    if args != ["--definitive-v2"] {
        eprintln!("PX0 definitive v2 requires the sole --definitive-v2 authority command");
        std::process::exit(2);
    }

    let preflight = source_preflight();
    if !preflight.passed() {
        eprintln!("PX0 definitive v2 preflight refused before cell zero: {preflight:?}");
        std::process::exit(2);
    }

    eprintln!("PX0_V2_DEFINITIVE_EVIDENCE_SPENT");
    let cells = (0..CELLS).map(run_cell).collect::<Vec<_>>();
    let passed = cells.len() == CELLS
        && cells.iter().all(CellResult::passed)
        && cells
            .iter()
            .map(|cell| cell.p.iter().filter(|p| **p).count())
            .sum::<usize>()
            == CELLS * 13;
    let csv = csv(&cells);
    let markdown = markdown(&preflight, &cells, passed);
    publish_write_once(STAGING_CSV, FINAL_CSV, &csv);
    publish_write_once(STAGING_MD, FINAL_MD, &markdown);
    File::open("results")
        .and_then(|directory| directory.sync_all())
        .expect("sync definitive results directory");
    std::process::exit(if passed { 0 } else { 1 });
}

fn source_preflight() -> Preflight {
    let active_source =
        fs::read_to_string("crates/px0-physical-correspondence/src/lib.rs").unwrap_or_default();
    let manifest =
        fs::read_to_string("crates/px0-physical-correspondence/Cargo.toml").unwrap_or_default();
    let forbidden = [
        "RelationMotif",
        "RelationAtom",
        "AnonymousView",
        "CandidateLink",
        "Neighborhood",
        "Episode",
        "History",
        "Query",
        "ProductiveHistory",
        "ContrastHistory",
        "START",
        "FINISH",
        "CORRECT",
        "WRONG",
        "begin_episode",
        "end_episode",
        "reset_task",
        "supporter_count_score",
        "precommit_score",
        "softmax",
        "temperature",
        "random(",
        "PROBATION",
        "REUSABLE",
        "STABLE_RETURN",
        "INCIDENTAL_RETURN",
    ];
    let dependency_surface_empty = manifest
        .split_once("[dependencies]")
        .is_some_and(|(_, suffix)| suffix.trim().is_empty());
    let readiness_tag_exact =
        git_output(&["rev-parse", READINESS_TAG]).is_none_or(|commit| commit == READINESS_COMMIT);
    Preflight {
        active_law_exact: sha256("crates/px0-physical-correspondence/src/lib.rs")
            == Some(ACTIVE_LAW_SHA256.to_string()),
        retained_physics_exact: sha256("crates/frozen-organism-v1-physics/src/substrate.rs")
            == Some(RETAINED_PHYSICS_SHA256.to_string()),
        v1_negative_exact: sha256("results/px0_physical_correspondence_definitive.csv")
            == Some(V1_CSV_SHA256.to_string())
            && sha256("results/px0_physical_correspondence_definitive.md")
                == Some(V1_MD_SHA256.to_string())
            && sha256("experiments/px0_physical_correspondence_definitive_result_audit.md")
                == Some(V1_AUDIT_SHA256.to_string())
            && sha256("experiments/px0_physical_correspondence_definitive_authority_handoff.md")
                == Some(V1_HANDOFF_SHA256.to_string()),
        p1_exact: sha256("experiments/px0_p1_return_free_proposal_control_result_audit.md")
            == Some(P1_AUDIT_SHA256.to_string())
            && sha256("experiments/px0_p1_return_free_proposal_control_handoff.md")
                == Some(P1_HANDOFF_SHA256.to_string()),
        specificity_exact: sha256("experiments/px0_s_stable_return_specificity_protocol.md")
            == Some(S_PROTOCOL_SHA256.to_string())
            && sha256("experiments/px0_s_stable_return_specificity_probe_v2_result_audit.md")
                == Some(S_PROBE_AUDIT_SHA256.to_string())
            && sha256("experiments/px0_s_stable_return_specificity_development_readiness.md")
                == Some(S_READINESS_SHA256.to_string()),
        protocol_exact: sha256("experiments/px0_physical_correspondence_definitive_v2_protocol.md")
            == Some(DEFINITIVE_PROTOCOL_SHA256.to_string()),
        readiness_tag_exact: readiness_tag_exact
            && READINESS_COMMIT == "a77a564f741e280bb674ff6e1cb6a28df4d80790",
        dependency_surface_empty,
        source_isolated: forbidden.iter().all(|token| !active_source.contains(token)),
        outputs_absent: !Path::new(FINAL_CSV).exists() && !Path::new(FINAL_MD).exists(),
        staging_absent: !Path::new(STAGING_CSV).exists() && !Path::new(STAGING_MD).exists(),
    }
}

fn sha256(path: &str) -> Option<String> {
    let output = Command::new("sha256sum").arg(path).output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()?
        .split_whitespace()
        .next()
        .map(str::to_string)
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
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

fn build(
    namespace: u64,
    reverse: bool,
    mirror: bool,
    stride: i32,
    distractor_load: usize,
) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; N];
    let mut probes = [None; N];
    let mut contenders = [None; N];
    let mut gates = [None; N];
    let mut backgrounds = [None; N];
    let mut supports = [[None; DEVICES]; N];
    let mut dense_drivers = [None; N];
    let support_delays = [1, 2, 3, 4];
    let order = if reverse { [2, 1, 0] } else { [0, 1, 2] };
    for index in order {
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
        dense_drivers[index] =
            Some(substrate.add_cell(cell(namespace + 60 + index as u64, position + 6, -2, 2)));
        for (device, support) in supports[index].iter_mut().enumerate() {
            *support = Some(substrate.add_cell(cell(
                namespace + 100 + index as u64 * 10 + device as u64,
                3_000 + index as i32 * 100 + device as i32 * 11,
                -2,
                1,
            )));
        }
        for distractor in 0..distractor_load {
            substrate.add_cell(cell(
                namespace + 1_000 + index as u64 * 100 + distractor as u64,
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
    let dense_drivers = dense_drivers.map(|value| value.expect("driver allocated"));
    let veto = substrate.add_cell(cell(namespace + 900, 5_000, 0, 2));
    let accumulator = substrate.add_cell(cell(namespace + 901, 5_001, 0, 1));
    let outside = substrate.add_cell(cell(namespace + 902, 5_002, 1, 1));
    for index in order {
        substrate.add_arrow(arrow(probes[index], gates[index], 1, 1));
        substrate.add_arrow(arrow(gates[index], sources[index], 1, 1));
        substrate.add_arrow(arrow(backgrounds[index], probes[index], 1, 1));
        for device in 0..DEVICES {
            substrate.add_arrow(arrow(
                supports[index][device],
                gates[index],
                support_delays[device],
                1,
            ));
        }
        substrate.add_arrow(arrow(contenders[index], veto, 0, 1));
        substrate.add_arrow(arrow(contenders[index], accumulator, 2, 1));
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
        dense_drivers,
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
            target: fixture.supports[*index][1],
            impulse: 1,
        });
    }
    fixture.substrate.propagate()
}

fn enter_twice(fixture: &mut Fixture, target: CellId, tick: i64, origin: u64, reverse: bool) {
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

fn enter_candidates(
    fixture: &mut Fixture,
    routes: &[usize],
    tick: i64,
    origin: u64,
    reverse: bool,
) {
    for route in routes {
        enter_twice(
            fixture,
            fixture.sources[*route],
            tick,
            origin + *route as u64 * 16,
            reverse,
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

fn enter_support(fixture: &mut Fixture, route: usize, device: usize, gate_tick: i64, origin: u64) {
    fixture.substrate.enter(SpikeInput {
        arrival_tick: gate_tick - fixture.support_delays[device],
        phase: 0,
        origin_physical: origin,
        target: fixture.supports[route][device],
        impulse: 1,
    });
}

fn enter_driver(fixture: &mut Fixture, route: usize, tick: i64, origin: u64, reverse: bool) {
    enter_twice(fixture, fixture.dense_drivers[route], tick, origin, reverse);
}

#[allow(clippy::too_many_arguments)]
fn dense_context(
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
    enter_candidates(fixture, &[stable, sparse, spare], base + 2, origin, reverse);
    enter_support(fixture, stable, device, base + 4, origin + 0x200);
    if sparse_active {
        enter_driver(fixture, sparse, base + 2, origin + 0x300, reverse);
    }
    if elsewhere_active {
        enter_driver(fixture, spare, base + 2, origin + 0x400, !reverse);
    }
    fixture.substrate.propagate()
}

fn held_out(
    fixture: &mut Fixture,
    route: usize,
    tick: i64,
    origin: u64,
    reverse: bool,
) -> Execution {
    enter_candidates(fixture, &[route], tick, origin, reverse);
    fixture.substrate.propagate()
}

fn delayed_returns(fixture: &Fixture, route: usize, source_tick: i64, run: &Execution) -> usize {
    let source_physical = fixture.namespace + 10 + route as u64;
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == source_physical && entry.tick > source_tick)
        .count()
}

fn max_live_resistance(fixture: &Fixture, route: usize) -> u32 {
    variable_ids(fixture, route)
        .into_iter()
        .filter(|arrow| fixture.substrate.arrow_is_live(*arrow))
        .map(|arrow| fixture.substrate.arrow_resistance(arrow))
        .max()
        .unwrap_or(0)
}

fn device_for(index: usize, form: usize) -> usize {
    if index.is_multiple_of(2) {
        (form + index) % DEVICES
    } else {
        (DEVICES + index - form) % DEVICES
    }
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

fn live_variable_ids(fixture: &Fixture, index: usize) -> Vec<ArrowId> {
    variable_ids(fixture, index)
        .into_iter()
        .filter(|arrow| fixture.substrate.arrow_is_live(*arrow))
        .collect()
}

fn all_variable_ids(fixture: &Fixture) -> Vec<ArrowId> {
    (0..N)
        .flat_map(|index| variable_ids(fixture, index))
        .collect()
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

#[allow(clippy::too_many_arguments)]
fn run_sequence(
    fixture: &mut Fixture,
    active: &[usize],
    supported: &[usize],
    start: i64,
    spacing: i64,
    namespace: u64,
    count: usize,
    reverse: bool,
    total: &mut WorkLedger,
) -> Vec<Execution> {
    (0..count)
        .map(|ordinal| {
            let execution = experience(
                fixture,
                active,
                supported,
                start + ordinal as i64 * spacing,
                namespace + ordinal as u64 * 0x20,
                reverse ^ (ordinal % 2 == 1),
            );
            add_work(total, &execution.work);
            execution
        })
        .collect()
}

#[cfg(any())]
fn run_cell_v1(index: usize) -> CellResult {
    let namespace = BASE_NAMESPACE + index as u64 * NAMESPACE_STRIDE;
    let initial = index % N;
    let current = (initial + 1) % N;
    let spare = (initial + 2) % N;
    let reverse = index % 2 == 1;
    let mirror = index % 4 >= 2;
    let spacing = 8 + (index % 8) as i64;
    let stride = 6 + (index % 5) as i32;
    let active = if index < 8 {
        vec![initial, current]
    } else {
        vec![initial, current, spare]
    };
    let mut total = WorkLedger::default();
    let mut quiescent = true;
    let mut fixture = build(namespace, reverse, mirror, stride);

    let acquisition = run_sequence(
        &mut fixture,
        &active,
        &[initial],
        0,
        spacing,
        namespace + 0x1_000,
        4,
        reverse,
        &mut total,
    );
    quiescent &= acquisition
        .iter()
        .all(|execution| execution.naturally_quiescent);
    let old_ids = variable_ids(&fixture, initial);
    let acquired = old_ids.len() == 2
        && old_ids
            .iter()
            .all(|arrow| fixture.substrate.arrow_is_live(*arrow));
    let initial_run = experience(
        &mut fixture,
        &[initial],
        &[initial],
        4 * spacing,
        namespace + 0x2_000,
        !reverse,
    );
    add_work(&mut total, &initial_run.work);
    quiescent &= initial_run.naturally_quiescent;
    let initial_effects = effects(&initial_run);

    let survival_tick = 4 * spacing + 20;
    let survival_pressure = fixture.substrate.advance_time(survival_tick);
    add_work(&mut total, &survival_pressure);
    let same_live_before_reuse = old_ids
        .iter()
        .all(|arrow| fixture.substrate.arrow_is_live(*arrow));
    let survival_run = experience(
        &mut fixture,
        &[initial],
        &[initial],
        survival_tick,
        namespace + 0x2_800,
        reverse,
    );
    add_work(&mut total, &survival_run.work);
    quiescent &= survival_run.naturally_quiescent;
    let survival_effects = effects(&survival_run);
    let same_live_after_reuse = old_ids
        .iter()
        .all(|arrow| fixture.substrate.arrow_is_live(*arrow));

    let arrows_before_forgetting = fixture.substrate.arrow_count();
    let forgetting = fixture.substrate.advance_time(300);
    add_work(&mut total, &forgetting);
    let old_dead = old_ids.iter().all(|arrow| {
        !fixture.substrate.arrow_is_live(*arrow) && fixture.substrate.arrow_resistance(*arrow) == 0
    });
    let no_time_proposal = forgetting.local_structural_proposals == 0
        && fixture.substrate.arrow_count() == arrows_before_forgetting;

    let first_renewal = experience(
        &mut fixture,
        &active,
        &[current],
        300,
        namespace + 0x3_000,
        !reverse,
    );
    add_work(&mut total, &first_renewal.work);
    quiescent &= first_renewal.naturally_quiescent;
    let fresh_ids = live_variable_ids(&fixture, current);
    let fresh_identity = fresh_ids.len() == 2
        && fresh_ids
            .iter()
            .all(|fresh| old_ids.iter().all(|old| old != fresh));
    let renewal = run_sequence(
        &mut fixture,
        &active,
        &[current],
        300 + spacing,
        spacing,
        namespace + 0x4_000,
        4,
        reverse,
        &mut total,
    );
    quiescent &= renewal
        .iter()
        .all(|execution| execution.naturally_quiescent);
    let reacquired = experience(
        &mut fixture,
        &[current],
        &[current],
        300 + 5 * spacing,
        namespace + 0x5_000,
        reverse,
    );
    add_work(&mut total, &reacquired.work);
    quiescent &= reacquired.naturally_quiescent;
    let reacquired_effects = effects(&reacquired);
    let historical = experience(
        &mut fixture,
        &[initial],
        &[],
        300 + 6 * spacing,
        namespace + 0x6_000,
        !reverse,
    );
    add_work(&mut total, &historical.work);
    quiescent &= historical.naturally_quiescent;
    let historical_effects = effects(&historical);
    let old_still_dead = old_ids
        .iter()
        .all(|arrow| !fixture.substrate.arrow_is_live(*arrow));
    let current_live = live_variable_ids(&fixture, current).len() == 2;

    let mut absent = build(namespace + 0x10_000, !reverse, !mirror, stride + 1);
    let absent_training = run_sequence(
        &mut absent,
        &active,
        &[],
        0,
        spacing,
        namespace + 0x11_000,
        4,
        !reverse,
        &mut total,
    );
    quiescent &= absent_training
        .iter()
        .all(|execution| execution.naturally_quiescent);
    let absent_before = all_variable_ids(&absent);
    let absent_pressure = absent.substrate.advance_time(200);
    add_work(&mut total, &absent_pressure);
    let absent_dead_before_test = absent_before
        .iter()
        .all(|arrow| !absent.substrate.arrow_is_live(*arrow));
    let absent_run = experience(
        &mut absent,
        &[initial],
        &[],
        200,
        namespace + 0x12_000,
        reverse,
    );
    add_work(&mut total, &absent_run.work);
    quiescent &= absent_run.naturally_quiescent;
    let absent_effects = effects(&absent_run);
    let absent_final_pressure = absent.substrate.advance_time(300);
    add_work(&mut total, &absent_final_pressure);
    let absent_all_dead = all_variable_ids(&absent)
        .iter()
        .all(|arrow| !absent.substrate.arrow_is_live(*arrow));

    let mut ambiguous = build(namespace + 0x20_000, reverse, mirror, stride);
    let ambiguous_training = run_sequence(
        &mut ambiguous,
        &active,
        &[initial, current],
        0,
        spacing,
        namespace + 0x21_000,
        4,
        reverse,
        &mut total,
    );
    quiescent &= ambiguous_training
        .iter()
        .all(|execution| execution.naturally_quiescent);
    let ambiguous_run = experience(
        &mut ambiguous,
        &[initial, current],
        &[initial, current],
        4 * spacing,
        namespace + 0x22_000,
        !reverse,
    );
    add_work(&mut total, &ambiguous_run.work);
    quiescent &= ambiguous_run.naturally_quiescent;
    let ambiguous_effects = effects(&ambiguous_run);
    let ambiguous_live = live_variable_ids(&ambiguous, initial).len() == 2
        && live_variable_ids(&ambiguous, current).len() == 2;

    let mut replay_first = fixture.clone();
    let mut replay_second = fixture.clone();
    let replay_tick = 300 + 7 * spacing;
    let first = experience(
        &mut replay_first,
        &[current],
        &[current],
        replay_tick,
        namespace + 0x30_000,
        true,
    );
    let second = experience(
        &mut replay_second,
        &[current],
        &[current],
        replay_tick,
        namespace + 0x30_000,
        true,
    );
    add_work(&mut total, &first.work);
    quiescent &= first.naturally_quiescent && second.naturally_quiescent;
    let replay_exact = first == second
        && replay_first.substrate.complete_fingerprint()
            == replay_second.substrate.complete_fingerprint();

    let p = [
        namespace == BASE_NAMESPACE + index as u64 * NAMESPACE_STRIDE
            && initial == index % N
            && current == (initial + 1) % N,
        acquired,
        initial_effects == 1 && initial_run.naturally_quiescent,
        same_live_before_reuse && same_live_after_reuse && survival_effects == 1,
        old_dead && forgetting.physical_deallocations > 0,
        no_time_proposal && old_dead,
        first_renewal.work.local_structural_proposals >= (active.len() * 2) as u64
            && fresh_identity,
        reacquired_effects == 1 && historical_effects == 0 && current_live && old_still_dead,
        absent_dead_before_test && absent_effects == 0 && absent_all_dead,
        ambiguous_effects == 0 && ambiguous_live,
        replay_exact,
        quiescent
            && total.total() > 0
            && total.local_structural_proposals > 0
            && total.physical_deallocations > 0
            && fixture.substrate.persistent_bytes() > 0
            && old_still_dead,
    ];
    CellResult {
        index,
        namespace,
        initial_route: initial,
        reacquired_route: current,
        spacing,
        active_opportunities: active.len(),
        reverse_allocation: reverse,
        mirrored_layout: mirror,
        initial_effects,
        survival_effects,
        reacquired_effects,
        historical_effects,
        absent_effects,
        ambiguous_effects,
        proposals: total.local_structural_proposals,
        deallocations: total.physical_deallocations,
        work: total.total(),
        persistent_bytes: fixture.substrate.persistent_bytes(),
        old_arrow_count: old_ids.len(),
        fresh_arrow_count: fresh_ids.len(),
        p,
    }
}

fn run_cell(index: usize) -> CellResult {
    let namespace = BASE_NAMESPACE + index as u64 * NAMESPACE_STRIDE;
    let initial = index % N;
    let current = (initial + 1) % N;
    let spare = (initial + 2) % N;
    let reverse = index % 2 == 1;
    let mirror = index % 4 >= 2;
    let spacing = 10 + (index % 8) as i64;
    let stride = 12 + 2 * (index % 8) as i32;
    let distractor_loads = [0, 2, 4, 8, 16, 24, 32, 40];
    let distractor_load = distractor_loads[index % distractor_loads.len()];
    let incidental_form = index % DEVICES;
    let active = vec![initial, current, spare];
    let mut total = WorkLedger::default();
    let mut quiescent = true;
    let mut fixture = build(namespace, reverse, mirror, stride, distractor_load);

    let acquisition = run_sequence(
        &mut fixture,
        &active,
        &[initial],
        0,
        spacing,
        namespace + 0x1_000,
        4,
        reverse,
        &mut total,
    );
    quiescent &= acquisition.iter().all(|run| run.naturally_quiescent);
    let old_ids = variable_ids(&fixture, initial);
    let acquired = old_ids.len() == 2
        && old_ids
            .iter()
            .all(|arrow| fixture.substrate.arrow_is_live(*arrow));
    let initial_run = experience(
        &mut fixture,
        &[initial],
        &[initial],
        4 * spacing,
        namespace + 0x2_000,
        !reverse,
    );
    add_work(&mut total, &initial_run.work);
    quiescent &= initial_run.naturally_quiescent;
    let initial_effects = effects(&initial_run);

    let survival_tick = 4 * spacing + 20;
    add_work(&mut total, &fixture.substrate.advance_time(survival_tick));
    let same_live_before_reuse = old_ids
        .iter()
        .all(|arrow| fixture.substrate.arrow_is_live(*arrow));
    let survival_run = experience(
        &mut fixture,
        &[initial],
        &[initial],
        survival_tick,
        namespace + 0x2_800,
        reverse,
    );
    add_work(&mut total, &survival_run.work);
    quiescent &= survival_run.naturally_quiescent;
    let survival_effects = effects(&survival_run);
    let same_live_after_reuse = old_ids
        .iter()
        .all(|arrow| fixture.substrate.arrow_is_live(*arrow));

    let arrows_before_forgetting = fixture.substrate.arrow_count();
    let forgetting = fixture.substrate.advance_time(300);
    add_work(&mut total, &forgetting);
    let old_dead = old_ids.iter().all(|arrow| {
        !fixture.substrate.arrow_is_live(*arrow) && fixture.substrate.arrow_resistance(*arrow) == 0
    });
    let no_time_proposal = forgetting.local_structural_proposals == 0
        && fixture.substrate.arrow_count() == arrows_before_forgetting;
    let stale_run = held_out(&mut fixture, initial, 300, namespace + 0x3_000, !reverse);
    add_work(&mut total, &stale_run.work);
    quiescent &= stale_run.naturally_quiescent;
    let stale_effects = effects(&stale_run);
    let post_forgetting_ids = variable_ids(&fixture, initial);
    let fresh_initial_proposal = post_forgetting_ids
        .iter()
        .any(|fresh| old_ids.iter().all(|old| old != fresh));

    let dense_start = 300 + spacing;
    let mut ordinal = 0usize;
    let mut stable_returns = 0usize;
    let mut sparse_returns = 0usize;
    let mut first_dense_proposals = 0u64;
    for _ in 0..8 {
        for form in 0..DEVICES {
            let base = dense_start + ordinal as i64 * spacing;
            let run = dense_context(
                &mut fixture,
                current,
                initial,
                spare,
                device_for(index, form),
                form == incidental_form,
                form == (incidental_form + 3) % DEVICES,
                base,
                namespace + 0x4_000 + ordinal as u64 * 0x100,
                reverse ^ (ordinal % 2 == 1),
            );
            if ordinal == 0 {
                first_dense_proposals = run.work.local_structural_proposals;
            }
            stable_returns += delayed_returns(&fixture, current, base + 2, &run);
            sparse_returns += delayed_returns(&fixture, initial, base + 2, &run);
            add_work(&mut total, &run.work);
            quiescent &= run.naturally_quiescent;
            ordinal += 1;
        }
    }
    for form in (0..DEVICES).filter(|form| *form != incidental_form) {
        let base = dense_start + ordinal as i64 * spacing;
        let run = dense_context(
            &mut fixture,
            current,
            initial,
            spare,
            device_for(index, form),
            false,
            form == (incidental_form + 3) % DEVICES,
            base,
            namespace + 0x8_000 + ordinal as u64 * 0x100,
            reverse ^ (ordinal % 2 == 1),
        );
        stable_returns += delayed_returns(&fixture, current, base + 2, &run);
        sparse_returns += delayed_returns(&fixture, initial, base + 2, &run);
        add_work(&mut total, &run.work);
        quiescent &= run.naturally_quiescent;
        ordinal += 1;
    }
    let stable_resistance = max_live_resistance(&fixture, current);
    let sparse_resistance = max_live_resistance(&fixture, initial);
    let current_ids = live_variable_ids(&fixture, current);
    let fresh_current = current_ids.len() == 2
        && current_ids
            .iter()
            .all(|fresh| old_ids.iter().all(|old| old != fresh));
    let test_tick = dense_start + ordinal as i64 * spacing;
    let mut current_first = fixture.clone();
    let mut current_second = fixture.clone();
    let reacquired = held_out(
        &mut current_first,
        current,
        test_tick,
        namespace + 0x9_000,
        reverse,
    );
    let replay = held_out(
        &mut current_second,
        current,
        test_tick,
        namespace + 0x9_000,
        reverse,
    );
    add_work(&mut total, &reacquired.work);
    quiescent &= reacquired.naturally_quiescent && replay.naturally_quiescent;
    let reacquired_effects = effects(&reacquired);
    let replay_exact = reacquired == replay
        && current_first.substrate.complete_fingerprint()
            == current_second.substrate.complete_fingerprint();
    let mut sparse_fixture = fixture.clone();
    let sparse_run = held_out(
        &mut sparse_fixture,
        initial,
        test_tick,
        namespace + 0xa_000,
        !reverse,
    );
    add_work(&mut total, &sparse_run.work);
    quiescent &= sparse_run.naturally_quiescent;
    let sparse_effects = effects(&sparse_run);
    add_work(
        &mut total,
        &sparse_fixture.substrate.advance_time(test_tick + 200),
    );
    let sparse_eventually_dead = variable_ids(&sparse_fixture, initial)
        .iter()
        .all(|arrow| !sparse_fixture.substrate.arrow_is_live(*arrow));
    let old_still_dead = old_ids
        .iter()
        .all(|arrow| !fixture.substrate.arrow_is_live(*arrow));

    let mut return_free = build(
        namespace + 0x1_0000,
        !reverse,
        !mirror,
        stride.max(16),
        distractor_load,
    );
    let mut return_free_returns = 0usize;
    let mut return_free_effects = 0usize;
    let mut return_free_proposals = 0u64;
    for presentation in 0..16 {
        let tick = presentation as i64 * spacing;
        let route = if presentation % 2 == 0 {
            initial
        } else {
            current
        };
        let run = held_out(
            &mut return_free,
            route,
            tick,
            namespace + 0x1_1000 + presentation as u64 * 0x100,
            reverse ^ (presentation % 2 == 1),
        );
        return_free_returns += delayed_returns(&return_free, route, tick, &run);
        return_free_effects += effects(&run);
        return_free_proposals += run.work.local_structural_proposals;
        add_work(&mut total, &run.work);
        quiescent &= run.naturally_quiescent;
    }
    let return_free_end = 16 * spacing;
    add_work(
        &mut total,
        &return_free.substrate.advance_time(return_free_end + 200),
    );
    let return_free_dead = all_variable_ids(&return_free)
        .iter()
        .all(|arrow| !return_free.substrate.arrow_is_live(*arrow));

    let mut dense = build(
        namespace + 0x2_0000,
        reverse,
        !mirror,
        stride,
        distractor_load,
    );
    let mut dense_initial_returns = 0usize;
    let mut dense_current_returns = 0usize;
    for presentation in 0..24 {
        let base = presentation as i64 * spacing;
        let run = dense_context(
            &mut dense,
            current,
            initial,
            spare,
            device_for(index, presentation % DEVICES),
            true,
            false,
            base,
            namespace + 0x2_1000 + presentation as u64 * 0x100,
            reverse ^ (presentation % 2 == 1),
        );
        dense_initial_returns += delayed_returns(&dense, initial, base + 2, &run);
        dense_current_returns += delayed_returns(&dense, current, base + 2, &run);
        add_work(&mut total, &run.work);
        quiescent &= run.naturally_quiescent;
    }
    let dense_tick = 24 * spacing;
    let mut dense_initial_fixture = dense.clone();
    let mut dense_current_fixture = dense.clone();
    let dense_initial_run = held_out(
        &mut dense_initial_fixture,
        initial,
        dense_tick,
        namespace + 0x2_9000,
        reverse,
    );
    let dense_current_run = held_out(
        &mut dense_current_fixture,
        current,
        dense_tick,
        namespace + 0x2_a000,
        !reverse,
    );
    let stable_dense_effects = effects(&dense_initial_run);
    let explicit_dense_effects = effects(&dense_current_run);
    let mut dense_ambiguous = dense.clone();
    enter_candidates(
        &mut dense_ambiguous,
        &[initial, current],
        dense_tick,
        namespace + 0x2_b000,
        reverse,
    );
    let dense_simultaneous = dense_ambiguous.substrate.propagate();
    add_work(&mut total, &dense_simultaneous.work);
    quiescent &= dense_initial_run.naturally_quiescent
        && dense_current_run.naturally_quiescent
        && dense_simultaneous.naturally_quiescent;

    let mut swapped = build(
        namespace + 0x3_0000,
        !reverse,
        mirror,
        stride,
        distractor_load,
    );
    let mut swap_ordinal = 0usize;
    let mut swapped_stable_returns = 0usize;
    let mut swapped_sparse_returns = 0usize;
    for _ in 0..8 {
        for form in 0..DEVICES {
            let base = swap_ordinal as i64 * spacing;
            let run = dense_context(
                &mut swapped,
                initial,
                current,
                spare,
                device_for(index + 1, form),
                form == incidental_form,
                false,
                base,
                namespace + 0x3_1000 + swap_ordinal as u64 * 0x100,
                !reverse ^ (swap_ordinal % 2 == 1),
            );
            swapped_stable_returns += delayed_returns(&swapped, initial, base + 2, &run);
            swapped_sparse_returns += delayed_returns(&swapped, current, base + 2, &run);
            add_work(&mut total, &run.work);
            quiescent &= run.naturally_quiescent;
            swap_ordinal += 1;
        }
    }
    for form in (0..DEVICES).filter(|form| *form != incidental_form) {
        let base = swap_ordinal as i64 * spacing;
        let run = dense_context(
            &mut swapped,
            initial,
            current,
            spare,
            device_for(index + 1, form),
            false,
            false,
            base,
            namespace + 0x3_9000 + swap_ordinal as u64 * 0x100,
            !reverse,
        );
        swapped_stable_returns += delayed_returns(&swapped, initial, base + 2, &run);
        swapped_sparse_returns += delayed_returns(&swapped, current, base + 2, &run);
        add_work(&mut total, &run.work);
        quiescent &= run.naturally_quiescent;
        swap_ordinal += 1;
    }
    let swap_tick = swap_ordinal as i64 * spacing;
    let mut swapped_stable_fixture = swapped.clone();
    let mut swapped_sparse_fixture = swapped.clone();
    let swapped_stable_run = held_out(
        &mut swapped_stable_fixture,
        initial,
        swap_tick,
        namespace + 0x3_a000,
        reverse,
    );
    let swapped_sparse_run = held_out(
        &mut swapped_sparse_fixture,
        current,
        swap_tick,
        namespace + 0x3_b000,
        !reverse,
    );
    let swapped_stable_effects = effects(&swapped_stable_run);
    let swapped_sparse_effects = effects(&swapped_sparse_run);
    quiescent &= swapped_stable_run.naturally_quiescent && swapped_sparse_run.naturally_quiescent;

    let mut absent = build(
        namespace + 0x4_0000,
        reverse,
        !mirror,
        stride,
        distractor_load,
    );
    let absent_training = run_sequence(
        &mut absent,
        &active,
        &[],
        0,
        spacing,
        namespace + 0x4_1000,
        8,
        reverse,
        &mut total,
    );
    quiescent &= absent_training.iter().all(|run| run.naturally_quiescent);
    add_work(&mut total, &absent.substrate.advance_time(200));
    let absent_run = held_out(&mut absent, initial, 200, namespace + 0x4_9000, reverse);
    let absent_effects = effects(&absent_run);
    quiescent &= absent_run.naturally_quiescent;

    let mut ambiguous = build(
        namespace + 0x5_0000,
        !reverse,
        mirror,
        stride,
        distractor_load,
    );
    let ambiguous_training = run_sequence(
        &mut ambiguous,
        &[initial, current],
        &[initial, current],
        0,
        spacing,
        namespace + 0x5_1000,
        24,
        !reverse,
        &mut total,
    );
    quiescent &= ambiguous_training.iter().all(|run| run.naturally_quiescent);
    let ambiguous_run = experience(
        &mut ambiguous,
        &[initial, current],
        &[initial, current],
        24 * spacing,
        namespace + 0x5_9000,
        reverse,
    );
    add_work(&mut total, &ambiguous_run.work);
    quiescent &= ambiguous_run.naturally_quiescent;
    let ambiguous_effects = effects(&ambiguous_run);
    let ambiguous_live = live_variable_ids(&ambiguous, initial).len() == 2
        && live_variable_ids(&ambiguous, current).len() == 2;

    let p = [
        namespace == BASE_NAMESPACE + index as u64 * NAMESPACE_STRIDE
            && spacing == 10 + (index % 8) as i64
            && stride == 12 + 2 * (index % 8) as i32
            && initial == index % N
            && current == (initial + 1) % N,
        acquired && initial_effects == 1,
        same_live_before_reuse && same_live_after_reuse && survival_effects == 1,
        old_dead && forgetting.physical_deallocations > 0,
        no_time_proposal && stale_effects == 0 && old_still_dead,
        fresh_initial_proposal && first_dense_proposals > 0 && fresh_current,
        stable_returns == ordinal
            && stable_resistance > sparse_resistance
            && reacquired_effects == 1,
        return_free_returns == 0
            && return_free_effects == 0
            && return_free_proposals > 0
            && return_free_dead,
        sparse_returns > 0
            && sparse_returns < stable_returns
            && sparse_resistance <= 1
            && sparse_effects == 0
            && sparse_eventually_dead,
        dense_initial_returns > 0
            && dense_current_returns > 0
            && stable_dense_effects == 1
            && explicit_dense_effects == 1
            && effects(&dense_simultaneous) == 0,
        swapped_stable_returns == swap_ordinal
            && swapped_sparse_returns > 0
            && swapped_sparse_returns < swapped_stable_returns
            && swapped_stable_effects == 1
            && swapped_sparse_effects == 0,
        absent_effects == 0 && ambiguous_effects == 0 && ambiguous_live,
        replay_exact
            && quiescent
            && old_still_dead
            && total.total() > 0
            && total.local_structural_proposals > 0
            && total.physical_deallocations > 0
            && fixture.substrate.persistent_bytes() > 0,
    ];

    CellResult {
        index,
        namespace,
        initial_route: initial,
        reacquired_route: current,
        spacing,
        active_opportunities: active.len(),
        reverse_allocation: reverse,
        mirrored_layout: mirror,
        initial_effects,
        survival_effects,
        reacquired_effects,
        historical_effects: stale_effects,
        sparse_effects,
        stable_dense_effects,
        swapped_stable_effects,
        swapped_sparse_effects,
        return_free_effects,
        absent_effects,
        ambiguous_effects,
        proposals: total.local_structural_proposals,
        deallocations: total.physical_deallocations,
        work: total.total(),
        persistent_bytes: fixture.substrate.persistent_bytes(),
        old_arrow_count: old_ids.len(),
        fresh_arrow_count: current_ids.len(),
        p,
    }
}

fn csv(cells: &[CellResult]) -> String {
    let mut text = String::from(
        "index,namespace,initial_route,reacquired_route,spacing,active_opportunities,reverse_allocation,mirrored_layout,initial_effects,survival_effects,reacquired_effects,historical_effects,sparse_effects,stable_dense_effects,swapped_stable_effects,swapped_sparse_effects,return_free_effects,absent_effects,ambiguous_effects,proposals,deallocations,work,persistent_bytes,old_arrow_count,fresh_arrow_count,p0,p1,p2,p3,p4,p5,p6,p7,p8,p9,p10,p11,p12,passed\n",
    );
    for cell in cells {
        let mut fields = vec![
            cell.index.to_string(),
            format!("0x{:x}", cell.namespace),
            cell.initial_route.to_string(),
            cell.reacquired_route.to_string(),
            cell.spacing.to_string(),
            cell.active_opportunities.to_string(),
            cell.reverse_allocation.to_string(),
            cell.mirrored_layout.to_string(),
            cell.initial_effects.to_string(),
            cell.survival_effects.to_string(),
            cell.reacquired_effects.to_string(),
            cell.historical_effects.to_string(),
            cell.sparse_effects.to_string(),
            cell.stable_dense_effects.to_string(),
            cell.swapped_stable_effects.to_string(),
            cell.swapped_sparse_effects.to_string(),
            cell.return_free_effects.to_string(),
            cell.absent_effects.to_string(),
            cell.ambiguous_effects.to_string(),
            cell.proposals.to_string(),
            cell.deallocations.to_string(),
            cell.work.to_string(),
            cell.persistent_bytes.to_string(),
            cell.old_arrow_count.to_string(),
            cell.fresh_arrow_count.to_string(),
        ];
        fields.extend(cell.p.iter().map(bool::to_string));
        fields.push(cell.passed().to_string());
        text.push_str(&fields.join(","));
        text.push('\n');
    }
    text
}

fn markdown(preflight: &Preflight, cells: &[CellResult], passed: bool) -> String {
    let controls_passed = cells
        .iter()
        .map(|cell| cell.p.iter().filter(|value| **value).count())
        .sum::<usize>();
    let proposals = cells.iter().map(|cell| cell.proposals).sum::<u64>();
    let deallocations = cells.iter().map(|cell| cell.deallocations).sum::<u64>();
    let work = cells.iter().map(|cell| cell.work).sum::<u64>();
    let mut text = format!(
        "# PX0 substrate-native physical correspondence definitive v2 result\n\nOutcome: **{}**.\n\n- cells: `{}/{}`\n- conjunctive claims: `{}/{}`\n- generic local proposals: `{}`\n- physical deallocations: `{}`\n- ledgered work: `{}`\n- PX0 authoritative: `{}`\n- PX1 development eligible: `{}`\n\n",
        if passed { "PASS" } else { "FAIL" },
        cells.iter().filter(|cell| cell.passed()).count(),
        CELLS,
        controls_passed,
        CELLS * 13,
        proposals,
        deallocations,
        work,
        passed,
        passed,
    );
    text.push_str(&format!(
        "Preflight: active law `{}`, retained physics `{}`, v1 negative `{}`, P1 `{}`, specificity `{}`, protocol `{}`, tag `{}`, dependency isolation `{}`, source isolation `{}`, outputs absent `{}`, staging absent `{}`.\n\n",
        preflight.active_law_exact,
        preflight.retained_physics_exact,
        preflight.v1_negative_exact,
        preflight.p1_exact,
        preflight.specificity_exact,
        preflight.protocol_exact,
        preflight.readiness_tag_exact,
        preflight.dependency_surface_empty,
        preflight.source_isolated,
        preflight.outputs_absent,
        preflight.staging_absent,
    ));
    text.push_str("| cell | A→B | spacing | opportunities | allocation | layout | effects initial/reuse/B/old/sparse/dense/swap | claims | work | pass |\n");
    text.push_str("|---:|---|---:|---:|---|---|---|---:|---:|---:|\n");
    for cell in cells {
        text.push_str(&format!(
            "| {} | {}→{} | {} | {} | {} | {} | {}/{}/{}/{}/{}/{}/{} | {}/13 | {} | {} |\n",
            cell.index,
            cell.initial_route,
            cell.reacquired_route,
            cell.spacing,
            cell.active_opportunities,
            if cell.reverse_allocation {
                "reverse"
            } else {
                "forward"
            },
            if cell.mirrored_layout {
                "mirror"
            } else {
                "direct"
            },
            cell.initial_effects,
            cell.survival_effects,
            cell.reacquired_effects,
            cell.historical_effects,
            cell.sparse_effects,
            cell.stable_dense_effects,
            cell.swapped_stable_effects,
            cell.p.iter().filter(|value| **value).count(),
            cell.work,
            cell.passed(),
        ));
    }
    text.push_str("\nAuthoritative claim: anonymous physical activity and ordinary returned activity implement a lifelong correspondence cycle; physically reliable return across changing experience becomes reusable structure, while return-free and sparse incidental candidates remain subthreshold or deallocate.\n");
    text
}

fn publish_write_once(staging: &str, final_path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(staging)
        .expect("create-new definitive staging artifact");
    file.write_all(contents.as_bytes())
        .expect("write definitive staging artifact");
    file.sync_all().expect("sync definitive staging artifact");
    fs::hard_link(staging, final_path).expect("publish definitive artifact without replacement");
    fs::remove_file(staging).expect("remove published staging link");
}
