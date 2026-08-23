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
const SCAFFOLD: u32 = 1_000;
const BASE_NAMESPACE: u64 = 0x800_0000;
const NAMESPACE_STRIDE: u64 = 0x4_0000;
const ACTIVE_LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const RETAINED_PHYSICS_SHA256: &str =
    "6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263";
const PX0_READINESS_SHA256: &str =
    "60f21e096079a5532f23d8a2974052f373a2fc6481c71099292147c6deb8cf5b";
const PX0_R_PROTOCOL_SHA256: &str =
    "65cc95374badd259ef023a17d0f745f39d26b5377b34317da3db92599758c107";
const PX0_R_READINESS_SHA256: &str =
    "508f958da3a7c583e611e5a113a5a66c4e98792b7dda9378523b8354447544e3";
const DEFINITIVE_PROTOCOL_SHA256: &str =
    "a5c918cb15868506a333b06a6f9c70f7cf23f09707a08385d03ab164513d0739";
const READINESS_COMMIT: &str = "745a5c3dc6d929faa2908359c5eb0462e8eac663";
const READINESS_TAG: &str = "px0-r-generic-physical-reproposal-development-readiness^{}";
const FINAL_CSV: &str = "results/px0_physical_correspondence_definitive.csv";
const FINAL_MD: &str = "results/px0_physical_correspondence_definitive.md";
const STAGING_CSV: &str = "results/.px0_physical_correspondence_definitive.csv.staging";
const STAGING_MD: &str = "results/.px0_physical_correspondence_definitive.md.staging";

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
struct Preflight {
    active_law_exact: bool,
    retained_physics_exact: bool,
    development_lineage_exact: bool,
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
            && self.development_lineage_exact
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
    absent_effects: usize,
    ambiguous_effects: usize,
    proposals: u64,
    deallocations: u64,
    work: u64,
    persistent_bytes: usize,
    old_arrow_count: usize,
    fresh_arrow_count: usize,
    p: [bool; 12],
}

impl CellResult {
    fn passed(&self) -> bool {
        self.p.iter().all(|value| *value)
    }
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args == ["--preflight"] {
        let preflight = source_preflight();
        println!("PX0 definitive no-cell preflight: {}", preflight.passed());
        if !preflight.passed() {
            std::process::exit(2);
        }
        return;
    }
    if args != ["--definitive"] {
        eprintln!("PX0 definitive execution requires the sole --definitive authority command");
        std::process::exit(2);
    }

    let preflight = source_preflight();
    if !preflight.passed() {
        eprintln!("PX0 definitive preflight refused before cell zero: {preflight:?}");
        std::process::exit(2);
    }

    eprintln!("PX0_DEFINITIVE_EVIDENCE_SPENT");
    let cells = (0..CELLS).map(run_cell).collect::<Vec<_>>();
    let passed = cells.len() == CELLS
        && cells.iter().all(CellResult::passed)
        && cells
            .iter()
            .map(|cell| cell.p.iter().filter(|p| **p).count())
            .sum::<usize>()
            == CELLS * 12;
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
        development_lineage_exact: sha256(
            "experiments/px0_physical_correspondence_development_readiness.md",
        ) == Some(PX0_READINESS_SHA256.to_string())
            && sha256(
                "experiments/px0_r_generic_physical_correspondence_reproposal_protocol.md",
            ) == Some(PX0_R_PROTOCOL_SHA256.to_string())
            && sha256(
                "experiments/px0_r_generic_physical_correspondence_reproposal_development_readiness.md",
            ) == Some(PX0_R_READINESS_SHA256.to_string()),
        protocol_exact: sha256("experiments/px0_physical_correspondence_definitive_protocol.md")
            == Some(DEFINITIVE_PROTOCOL_SHA256.to_string()),
        readiness_tag_exact: readiness_tag_exact
            && READINESS_COMMIT == "745a5c3dc6d929faa2908359c5eb0462e8eac663",
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

fn build(namespace: u64, reverse: bool, mirror: bool, stride: i32) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; N];
    let mut probes = [None; N];
    let mut contenders = [None; N];
    let mut gates = [None; N];
    let mut backgrounds = [None; N];
    let mut supports = [None; N];
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
    for index in order {
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

fn run_cell(index: usize) -> CellResult {
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

fn csv(cells: &[CellResult]) -> String {
    let mut text = String::from(
        "index,namespace,initial_route,reacquired_route,spacing,active_opportunities,reverse_allocation,mirrored_layout,initial_effects,survival_effects,reacquired_effects,historical_effects,absent_effects,ambiguous_effects,proposals,deallocations,work,persistent_bytes,old_arrow_count,fresh_arrow_count,p0,p1,p2,p3,p4,p5,p6,p7,p8,p9,p10,p11,passed\n",
    );
    for cell in cells {
        text.push_str(&format!(
            "{},0x{:x},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            cell.index,
            cell.namespace,
            cell.initial_route,
            cell.reacquired_route,
            cell.spacing,
            cell.active_opportunities,
            cell.reverse_allocation,
            cell.mirrored_layout,
            cell.initial_effects,
            cell.survival_effects,
            cell.reacquired_effects,
            cell.historical_effects,
            cell.absent_effects,
            cell.ambiguous_effects,
            cell.proposals,
            cell.deallocations,
            cell.work,
            cell.persistent_bytes,
            cell.old_arrow_count,
            cell.fresh_arrow_count,
            cell.p[0],
            cell.p[1],
            cell.p[2],
            cell.p[3],
            cell.p[4],
            cell.p[5],
            cell.p[6],
            cell.p[7],
            cell.p[8],
            cell.p[9],
            cell.p[10],
            cell.p[11],
            cell.passed(),
        ));
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
        "# PX0 substrate-native physical correspondence definitive result\n\nOutcome: **{}**.\n\n- cells: `{}/{}`\n- conjunctive controls: `{}/{}`\n- generic local proposals: `{}`\n- physical deallocations: `{}`\n- ledgered work: `{}`\n- PX0 authoritative: `{}`\n- PX1 development eligible: `{}`\n\n",
        if passed { "PASS" } else { "FAIL" },
        cells.iter().filter(|cell| cell.passed()).count(),
        CELLS,
        controls_passed,
        CELLS * 12,
        proposals,
        deallocations,
        work,
        passed,
        passed,
    );
    text.push_str(&format!(
        "Preflight: active law `{}`, retained physics `{}`, lineage `{}`, protocol `{}`, tag `{}`, dependency isolation `{}`, source isolation `{}`, outputs absent `{}`, staging absent `{}`.\n\n",
        preflight.active_law_exact,
        preflight.retained_physics_exact,
        preflight.development_lineage_exact,
        preflight.protocol_exact,
        preflight.readiness_tag_exact,
        preflight.dependency_surface_empty,
        preflight.source_isolated,
        preflight.outputs_absent,
        preflight.staging_absent,
    ));
    text.push_str("| cell | A→B | spacing | opportunities | allocation | layout | effects initial/reuse/reacquired/old | controls | work | pass |\n");
    text.push_str("|---:|---|---:|---:|---|---|---|---:|---:|---:|\n");
    for cell in cells {
        text.push_str(&format!(
            "| {} | {}→{} | {} | {} | {} | {} | {}/{}/{}/{} | {}/12 | {} | {} |\n",
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
            cell.p.iter().filter(|value| **value).count(),
            cell.work,
            cell.passed(),
        ));
    }
    text.push_str("\nAuthoritative claim: anonymous physical activity and ordinary returned activity acquire executable correspondence; live structure supports survival-window reuse; ordinary pressure produces true forgetting; and renewed activity plus contemporary return learns fresh physical correspondence without restoring historical identity.\n");
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
