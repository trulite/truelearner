#![forbid(unsafe_code)]

use cj_b_locally_gated_arrow::{
    ArrowSpec, ArrowSummary, CellId, CellSpec, Execution, SpikeInput, Substrate, WorkLedger,
};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

const CSV_PATH: &str = "results/cj0_b_locally_gated_arrow_gate_v1.csv";
const REPORT_PATH: &str = "results/cj0_b_locally_gated_arrow_gate_v1.md";
const CSV_STAGE: &str = "results/.cj0_b_locally_gated_arrow_gate_v1.csv.staging";
const REPORT_STAGE: &str = "results/.cj0_b_locally_gated_arrow_gate_v1.md.staging";

#[derive(Clone, Copy)]
enum Duo {
    Ab,
    Cd,
    Ad,
    Cb,
}

impl Duo {
    fn source(self) -> usize {
        match self {
            Self::Ab | Self::Ad => 0,
            Self::Cd | Self::Cb => 1,
        }
    }

    fn contributor(self) -> usize {
        match self {
            Self::Ab | Self::Cb => 0,
            Self::Cd | Self::Ad => 1,
        }
    }
}

#[derive(Clone, Copy)]
struct Variant {
    name: &'static str,
    namespace: u64,
    mirror: bool,
    reverse_cells: bool,
    reverse_arrows: bool,
    reverse_insertion: bool,
    initial_spacing: i64,
    initial_offset: i64,
    changed_spacing: i64,
}

#[derive(Clone)]
struct FlatField {
    substrate: Substrate,
    sources: [CellId; 2],
    sites: [CellId; 4],
    drivers: [CellId; 2],
    reverse_insertion: bool,
}

#[derive(Default)]
struct Observation {
    crossings: usize,
    candidate_consumptions: usize,
    source_firings: usize,
    output_firings: usize,
    quiescent: bool,
    work: u64,
}

struct FlatResult {
    initial_fp: u64,
    initial_acquired_fp: u64,
    initial_gap_fp: u64,
    changed_acquired_fp: u64,
    changed_gap_fp: u64,
    permanent_fp: u64,
    training_ab: usize,
    training_cd: usize,
    trained_ab: Observation,
    trained_cd: Observation,
    crossed_ad: Observation,
    crossed_cb: Observation,
    changed_ad: usize,
    changed_cb: usize,
    new_ad: Observation,
    new_cb: Observation,
    old_ab: Observation,
    old_cd: Observation,
    late_old_ab: Observation,
    late_old_cd: Observation,
    source_alone: Observation,
    contributor_alone: Observation,
    old_ab_structure: ArrowSummary,
    old_cd_structure: ArrowSummary,
    new_ad_structure: ArrowSummary,
    new_cb_structure: ArrowSummary,
    duplicate_equal: bool,
    proposals: u64,
    deallocations: u64,
    arrow_count: usize,
    persistent_bytes: usize,
    quiescent: bool,
    work: u64,
}

struct RecursiveResult {
    training_crossings: usize,
    candidate: [ArrowSummary; 3],
    level_outputs: [usize; 3],
    full_outputs: [usize; 3],
    full_crossings: usize,
    missing_crossings: [usize; 3],
    duplicate_equal: bool,
    quiescent: bool,
    max_source_firings: usize,
    permanent_fp: u64,
    arrow_count: usize,
    persistent_bytes: usize,
    work: u64,
}

struct ConvergenceResult {
    source_a: usize,
    source_b: usize,
    joint: usize,
    crossings: [usize; 3],
    duplicate_equal: bool,
    quiescent: bool,
    permanent_fp: u64,
    work: u64,
}

#[derive(Default)]
struct TemporalCase {
    crossings: usize,
    output_tick: i64,
    output_impulse: i32,
    contributor_arrivals: usize,
    fingerprint: u64,
    quiescent: bool,
    work: u64,
}

struct TemporalResult {
    together: TemporalCase,
    a_then_b: TemporalCase,
    overlap: TemporalCase,
    within: TemporalCase,
    absent: TemporalCase,
    permanent_fp: u64,
    work: u64,
}

struct Row {
    variant: Variant,
    flat: FlatResult,
    recursive: RecursiveResult,
    convergence: ConvergenceResult,
    temporal: TemporalResult,
    clauses: [bool; 10],
    total_work: u64,
}

fn add_cell(
    substrate: &mut Substrate,
    physical_id: u64,
    position: i32,
    region: i16,
    threshold: i32,
) -> CellId {
    substrate.add_cell(CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 1_000,
    })
}

fn add_arrow(
    substrate: &mut Substrate,
    from: CellId,
    to: CellId,
    coupling: i32,
    resistance: u32,
    phase: i32,
) {
    substrate.add_arrow(ArrowSpec {
        from,
        to,
        delay: 0,
        phase,
        coupling,
        resistance,
    });
}

fn build_flat(variant: Variant) -> FlatField {
    let sign = if variant.mirror { -1 } else { 1 };
    let positions = [0, 100, 1, 2, 101, 102, -20, 80, 30, 31, 130, 131].map(|value| value * sign);
    let regions = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1];
    let thresholds = [3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1];
    let mut substrate = Substrate::new();
    let mut handles: [Option<CellId>; 12] = [None; 12];
    let mut order = (0..12).collect::<Vec<_>>();
    if variant.reverse_cells {
        order.reverse();
    }
    for role in order {
        let offset = if variant.reverse_cells {
            12 - role
        } else {
            role + 1
        };
        handles[role] = Some(add_cell(
            &mut substrate,
            variant.namespace + u64::try_from(offset).expect("small role"),
            positions[role],
            regions[role],
            thresholds[role],
        ));
    }
    let cells = handles.map(|entry| entry.expect("complete flat matter"));
    let sources = [cells[0], cells[1]];
    let sites = [cells[2], cells[3], cells[4], cells[5]];
    let drivers = [cells[6], cells[7]];
    let outward = [cells[8], cells[9], cells[10], cells[11]];
    let mut arrows = Vec::new();
    for (from, to) in [
        (sources[0], sites[0]),
        (sources[0], sites[1]),
        (sources[1], sites[2]),
        (sources[1], sites[3]),
    ] {
        arrows.push((from, to, 1, 1, 0));
    }
    for (from, to) in drivers.into_iter().zip(sources) {
        arrows.push((from, to, 3, 1_000, 0));
    }
    for index in 0..4 {
        arrows.push((sites[index], sources[usize::from(index >= 2)], 3, 1_000, 1));
        arrows.push((sites[index], outward[index], 1, 1_000, 2));
    }
    if variant.reverse_arrows {
        arrows.reverse();
    }
    for (from, to, coupling, resistance, phase) in arrows {
        add_arrow(&mut substrate, from, to, coupling, resistance, phase);
    }
    FlatField {
        substrate,
        sources,
        sites,
        drivers,
        reverse_insertion: variant.reverse_insertion,
    }
}

fn enter_flat_contributor(field: &mut FlatField, contributor: usize, tick: i64) {
    let mut targets = if contributor == 0 {
        [field.sites[0], field.sites[3]]
    } else {
        [field.sites[1], field.sites[2]]
    };
    if field.reverse_insertion {
        targets.reverse();
    }
    for (index, target) in targets.into_iter().enumerate() {
        field.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: -10,
            origin_physical: 0x410 + u64::try_from(contributor * 2 + index).expect("small"),
            target,
            impulse: 2,
        });
    }
}

fn enter_flat_source(field: &mut FlatField, source: usize, tick: i64, direct: bool) {
    field.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 0x420 + u64::try_from(source).expect("small"),
        target: if direct {
            field.sources[source]
        } else {
            field.drivers[source]
        },
        impulse: 3,
    });
}

fn execute_flat(field: &mut FlatField, duo: Duo, tick: i64, direct: bool) -> Execution {
    if field.reverse_insertion {
        enter_flat_source(field, duo.source(), tick, direct);
        enter_flat_contributor(field, duo.contributor(), tick);
    } else {
        enter_flat_contributor(field, duo.contributor(), tick);
        enter_flat_source(field, duo.source(), tick, direct);
    }
    field.substrate.propagate()
}

fn summarize_flat(field: &FlatField, run: &Execution) -> Observation {
    let source_ids = field
        .sources
        .map(|cell| field.substrate.cell_physical_id(cell));
    let site_ids = field
        .sites
        .map(|cell| field.substrate.cell_physical_id(cell));
    Observation {
        crossings: run.crossings.len(),
        candidate_consumptions: run
            .transmissions
            .iter()
            .filter(|entry| {
                entry.emitted
                    && entry.destination_state > 0
                    && source_ids.contains(&entry.from_physical)
                    && site_ids.contains(&entry.to_physical)
            })
            .count(),
        source_firings: run
            .trace
            .iter()
            .filter(|entry| entry.fired && source_ids.contains(&entry.target_physical))
            .count(),
        output_firings: run
            .trace
            .iter()
            .filter(|entry| entry.fired && site_ids.contains(&entry.target_physical))
            .count(),
        quiescent: run.naturally_quiescent,
        work: run.work.total(),
    }
}

fn observe_flat(field: &mut FlatField, duo: Duo) -> (Execution, Observation) {
    let tick = field.substrate.current_tick();
    let run = execute_flat(field, duo, tick, false);
    let summary = summarize_flat(field, &run);
    (run, summary)
}

fn observe_flat_source(field: &mut FlatField, source: usize) -> Observation {
    let tick = field.substrate.current_tick();
    enter_flat_source(field, source, tick, false);
    let run = field.substrate.propagate();
    summarize_flat(field, &run)
}

fn observe_flat_contributor(field: &mut FlatField, contributor: usize) -> Observation {
    let tick = field.substrate.current_tick();
    enter_flat_contributor(field, contributor, tick);
    let run = field.substrate.propagate();
    summarize_flat(field, &run)
}

fn train_flat_initial(
    field: &mut FlatField,
    variant: Variant,
    work: &mut WorkLedger,
) -> (usize, usize) {
    let mut ab = 0;
    let mut cd = 0;
    for round in 0..8 {
        let base = i64::from(round) * variant.initial_spacing;
        let order = if round % 2 == 0 {
            [(Duo::Ab, base), (Duo::Cd, base + variant.initial_offset)]
        } else {
            [(Duo::Cd, base), (Duo::Ab, base + variant.initial_offset)]
        };
        for (duo, tick) in order {
            let run = execute_flat(field, duo, tick, true);
            ab += usize::from(matches!(duo, Duo::Ab) && run.crossings.len() == 1);
            cd += usize::from(matches!(duo, Duo::Cd) && run.crossings.len() == 1);
            work.absorb(&run.work);
        }
    }
    (ab, cd)
}

fn train_flat_changed(
    field: &mut FlatField,
    variant: Variant,
    start: i64,
    work: &mut WorkLedger,
) -> (usize, usize) {
    let mut ad = 0;
    let mut cb = 0;
    for round in 0..20 {
        let base = start + i64::from(round) * variant.changed_spacing;
        let order = if round % 2 == 0 {
            [(Duo::Ad, base), (Duo::Cb, base + 4)]
        } else {
            [(Duo::Cb, base), (Duo::Ad, base + 4)]
        };
        for (duo, tick) in order {
            let run = execute_flat(field, duo, tick, true);
            ad += usize::from(matches!(duo, Duo::Ad) && run.crossings.len() == 1);
            cb += usize::from(matches!(duo, Duo::Cb) && run.crossings.len() == 1);
            work.absorb(&run.work);
        }
    }
    (ad, cb)
}

fn run_flat(variant: Variant) -> FlatResult {
    let mut field = build_flat(variant);
    let initial_fp = field.substrate.permanent_fingerprint();
    let mut ledger = WorkLedger::default();
    let (training_ab, training_cd) = train_flat_initial(&mut field, variant, &mut ledger);
    let initial_acquired_fp = field.substrate.permanent_fingerprint();
    let initial_gap_tick = field.substrate.current_tick() + 25;
    let gap = field.substrate.advance_time(initial_gap_tick);
    ledger.absorb(&gap);
    let initial_gap_fp = field.substrate.complete_fingerprint();

    let mut trained_ab_field = field.clone();
    let (_, trained_ab) = observe_flat(&mut trained_ab_field, Duo::Ab);
    let mut trained_cd_field = field.clone();
    let (_, trained_cd) = observe_flat(&mut trained_cd_field, Duo::Cd);
    let mut crossed_ad_field = field.clone();
    let (_, crossed_ad) = observe_flat(&mut crossed_ad_field, Duo::Ad);
    let mut crossed_cb_field = field.clone();
    let (_, crossed_cb) = observe_flat(&mut crossed_cb_field, Duo::Cb);

    let change_start = field.substrate.current_tick();
    let (changed_ad, changed_cb) =
        train_flat_changed(&mut field, variant, change_start, &mut ledger);
    let changed_acquired_fp = field.substrate.permanent_fingerprint();
    let changed_gap_tick = field.substrate.current_tick() + 40;
    let gap = field.substrate.advance_time(changed_gap_tick);
    ledger.absorb(&gap);
    let changed_gap_fp = field.substrate.complete_fingerprint();
    let permanent_fp = field.substrate.permanent_fingerprint();

    let old_ab_structure = field
        .substrate
        .arrows_between(field.sources[0], field.sites[0]);
    let old_cd_structure = field
        .substrate
        .arrows_between(field.sources[1], field.sites[2]);
    let new_ad_structure = field
        .substrate
        .arrows_between(field.sources[0], field.sites[1]);
    let new_cb_structure = field
        .substrate
        .arrows_between(field.sources[1], field.sites[3]);
    let mut new_ad_field = field.clone();
    let (duplicate_left, new_ad) = observe_flat(&mut new_ad_field, Duo::Ad);
    let mut duplicate_field = field.clone();
    let (duplicate_right, _) = observe_flat(&mut duplicate_field, Duo::Ad);
    let duplicate_equal = duplicate_left == duplicate_right
        && new_ad_field.substrate.complete_fingerprint()
            == duplicate_field.substrate.complete_fingerprint();
    let mut new_cb_field = field.clone();
    let (_, new_cb) = observe_flat(&mut new_cb_field, Duo::Cb);
    let mut old_ab_field = field.clone();
    let (_, old_ab) = observe_flat(&mut old_ab_field, Duo::Ab);
    let mut old_cd_field = field.clone();
    let (_, old_cd) = observe_flat(&mut old_cd_field, Duo::Cd);
    let mut source_field = field.clone();
    let source_alone = observe_flat_source(&mut source_field, 0);
    let mut contributor_field = field.clone();
    let contributor_alone = observe_flat_contributor(&mut contributor_field, 0);
    let mut late = field.clone();
    let late_gap = late
        .substrate
        .advance_time(late.substrate.current_tick() + 50);
    let mut late_ab_field = late.clone();
    let (_, late_old_ab) = observe_flat(&mut late_ab_field, Duo::Ab);
    let mut late_cd_field = late.clone();
    let (_, late_old_cd) = observe_flat(&mut late_cd_field, Duo::Cd);
    let observations = [
        &trained_ab,
        &trained_cd,
        &crossed_ad,
        &crossed_cb,
        &new_ad,
        &new_cb,
        &old_ab,
        &old_cd,
        &late_old_ab,
        &late_old_cd,
        &source_alone,
        &contributor_alone,
    ];
    let observation_work = observations.iter().map(|entry| entry.work).sum::<u64>();
    let quiescent = observations.iter().all(|entry| entry.quiescent);
    FlatResult {
        initial_fp,
        initial_acquired_fp,
        initial_gap_fp,
        changed_acquired_fp,
        changed_gap_fp,
        permanent_fp,
        training_ab,
        training_cd,
        trained_ab,
        trained_cd,
        crossed_ad,
        crossed_cb,
        changed_ad,
        changed_cb,
        new_ad,
        new_cb,
        old_ab,
        old_cd,
        late_old_ab,
        late_old_cd,
        source_alone,
        contributor_alone,
        old_ab_structure,
        old_cd_structure,
        new_ad_structure,
        new_cb_structure,
        duplicate_equal,
        proposals: ledger.local_structural_proposals,
        deallocations: ledger.physical_deallocations,
        arrow_count: field.substrate.arrow_count(),
        persistent_bytes: field.substrate.persistent_bytes(),
        quiescent,
        work: ledger.total() + observation_work + late_gap.total(),
    }
}

#[derive(Clone)]
struct RecursiveField {
    substrate: Substrate,
    sources: [CellId; 3],
    targets: [CellId; 3],
    drivers: [CellId; 3],
    outward: CellId,
}

fn build_recursive(variant: Variant) -> RecursiveField {
    let mut substrate = Substrate::new();
    let sign = if variant.mirror { -1 } else { 1 };
    let base = variant.namespace + 0x10_000;
    let a = add_cell(&mut substrate, base + 1, 0, 0, 3);
    let x = add_cell(&mut substrate, base + 2, 2 * sign, 0, 3);
    let y = add_cell(&mut substrate, base + 3, 4 * sign, 0, 3);
    let z = add_cell(&mut substrate, base + 4, 6 * sign, 0, 3);
    let driver_a = add_cell(&mut substrate, base + 5, -30 * sign, 0, 3);
    let driver_x = add_cell(&mut substrate, base + 6, -40 * sign, 0, 3);
    let driver_y = add_cell(&mut substrate, base + 7, -50 * sign, 0, 3);
    let outward = add_cell(&mut substrate, base + 8, 30 * sign, 1, 1);
    for (from, to) in [(a, x), (x, y), (y, z)] {
        add_arrow(&mut substrate, from, to, 1, 1, 0);
    }
    for (from, to) in [(x, a), (y, x), (z, y)] {
        add_arrow(&mut substrate, from, to, 3, 1_000, 1);
    }
    for (from, to) in [(driver_a, a), (driver_x, x), (driver_y, y)] {
        add_arrow(&mut substrate, from, to, 3, 1_000, 0);
    }
    add_arrow(&mut substrate, z, outward, 1, 1_000, 2);
    RecursiveField {
        substrate,
        sources: [a, x, y],
        targets: [x, y, z],
        drivers: [driver_a, driver_x, driver_y],
        outward,
    }
}

fn prime_recursive(field: &mut RecursiveField, indexes: &[usize], tick: i64) {
    for index in indexes {
        field.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: -10,
            origin_physical: 0x510 + u64::try_from(*index).expect("small"),
            target: field.targets[*index],
            impulse: 2,
        });
    }
}

fn activate_recursive(field: &mut RecursiveField, level: usize, tick: i64, direct: bool) {
    field.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 0x520 + u64::try_from(level).expect("small"),
        target: if direct {
            field.sources[level]
        } else {
            field.drivers[level]
        },
        impulse: 3,
    });
}

fn recursive_output_counts(field: &RecursiveField, run: &Execution) -> [usize; 3] {
    field.targets.map(|target| {
        let physical = field.substrate.cell_physical_id(target);
        run.trace
            .iter()
            .filter(|entry| entry.fired && entry.target_physical == physical)
            .count()
    })
}

fn recursive_observation(
    field: &mut RecursiveField,
    level: usize,
    primes: &[usize],
) -> (Execution, [usize; 3]) {
    let tick = field.substrate.current_tick();
    prime_recursive(field, primes, tick);
    activate_recursive(field, level, tick, false);
    let run = field.substrate.propagate();
    let outputs = recursive_output_counts(field, &run);
    (run, outputs)
}

fn run_recursive(variant: Variant) -> RecursiveResult {
    let mut field = build_recursive(variant);
    let mut ledger = WorkLedger::default();
    let mut training_crossings = 0;
    for round in 0..8 {
        let tick = i64::from(round * 4);
        prime_recursive(&mut field, &[0, 1, 2], tick);
        activate_recursive(&mut field, 0, tick, true);
        let run = field.substrate.propagate();
        training_crossings += run.crossings.len();
        ledger.absorb(&run.work);
    }
    let gap = field.substrate.advance_time(50);
    ledger.absorb(&gap);
    let candidate = [
        field
            .substrate
            .arrows_between(field.sources[0], field.targets[0]),
        field
            .substrate
            .arrows_between(field.sources[1], field.targets[1]),
        field
            .substrate
            .arrows_between(field.sources[2], field.targets[2]),
    ];
    let mut level_outputs = [0; 3];
    let mut observation_work = 0;
    let mut quiescent = true;
    let mut max_source_firings = 0;
    for level in 0..3 {
        let mut clone = field.clone();
        let (run, outputs) = recursive_observation(&mut clone, level, &[level]);
        level_outputs[level] = outputs[level];
        observation_work += run.work.total();
        quiescent &= run.naturally_quiescent;
        let source_ids = field
            .sources
            .map(|cell| field.substrate.cell_physical_id(cell));
        max_source_firings = max_source_firings.max(
            run.trace
                .iter()
                .filter(|entry| entry.fired && source_ids.contains(&entry.target_physical))
                .count(),
        );
    }
    let mut full_field = field.clone();
    let (duplicate_left, full_outputs) = recursive_observation(&mut full_field, 0, &[0, 1, 2]);
    let full_crossings = duplicate_left.crossings.len();
    observation_work += duplicate_left.work.total();
    quiescent &= duplicate_left.naturally_quiescent;
    let mut duplicate_field = field.clone();
    let (duplicate_right, _) = recursive_observation(&mut duplicate_field, 0, &[0, 1, 2]);
    let duplicate_equal = duplicate_left == duplicate_right
        && full_field.substrate.complete_fingerprint()
            == duplicate_field.substrate.complete_fingerprint();
    let mut missing_crossings = [0; 3];
    for (missing, crossing) in missing_crossings.iter_mut().enumerate() {
        let primes = (0..3).filter(|index| *index != missing).collect::<Vec<_>>();
        let mut clone = field.clone();
        let (run, _) = recursive_observation(&mut clone, 0, &primes);
        *crossing = run.crossings.len();
        observation_work += run.work.total();
        quiescent &= run.naturally_quiescent;
    }
    let _outward_physical = field.substrate.cell_physical_id(field.outward);
    RecursiveResult {
        training_crossings,
        candidate,
        level_outputs,
        full_outputs,
        full_crossings,
        missing_crossings,
        duplicate_equal,
        quiescent,
        max_source_firings,
        permanent_fp: field.substrate.permanent_fingerprint(),
        arrow_count: field.substrate.arrow_count(),
        persistent_bytes: field.substrate.persistent_bytes(),
        work: ledger.total() + observation_work + duplicate_right.work.total(),
    }
}

struct ConvergenceField {
    substrate: Substrate,
    sources: [CellId; 2],
    center: CellId,
}

fn build_convergence(variant: Variant) -> ConvergenceField {
    let mut substrate = Substrate::new();
    let base = variant.namespace + 0x20_000;
    let a = add_cell(&mut substrate, base + 1, 0, 0, 3);
    let b = add_cell(&mut substrate, base + 2, 10, 0, 3);
    let center = add_cell(&mut substrate, base + 3, 20, 0, 3);
    let outward = add_cell(&mut substrate, base + 4, 30, 1, 1);
    add_arrow(&mut substrate, a, center, 3, 1_000, 0);
    add_arrow(&mut substrate, b, center, 3, 1_000, 0);
    add_arrow(&mut substrate, center, outward, 1, 1_000, 1);
    ConvergenceField {
        substrate,
        sources: [a, b],
        center,
    }
}

fn execute_convergence(field: &mut ConvergenceField, active: &[usize]) -> Execution {
    for index in active {
        field.substrate.enter(SpikeInput {
            arrival_tick: 0,
            phase: 0,
            origin_physical: 0x610 + u64::try_from(*index).expect("small"),
            target: field.sources[*index],
            impulse: 3,
        });
    }
    field.substrate.propagate()
}

fn center_firings(field: &ConvergenceField, run: &Execution) -> usize {
    let physical = field.substrate.cell_physical_id(field.center);
    run.trace
        .iter()
        .filter(|entry| entry.fired && entry.target_physical == physical)
        .count()
}

fn run_convergence(variant: Variant) -> ConvergenceResult {
    let field = build_convergence(variant);
    let mut a_field = field;
    let run_a = execute_convergence(&mut a_field, &[0]);
    let field = build_convergence(variant);
    let mut b_field = field;
    let run_b = execute_convergence(&mut b_field, &[1]);
    let field = build_convergence(variant);
    let permanent_fp = field.substrate.permanent_fingerprint();
    let mut joint_field = field;
    let run_joint = execute_convergence(&mut joint_field, &[0, 1]);
    let field = build_convergence(variant);
    let mut duplicate_field = field;
    let duplicate_run = execute_convergence(&mut duplicate_field, &[0, 1]);
    ConvergenceResult {
        source_a: center_firings(&a_field, &run_a),
        source_b: center_firings(&b_field, &run_b),
        joint: center_firings(&joint_field, &run_joint),
        crossings: [
            run_a.crossings.len(),
            run_b.crossings.len(),
            run_joint.crossings.len(),
        ],
        duplicate_equal: run_joint == duplicate_run
            && joint_field.substrate.complete_fingerprint()
                == duplicate_field.substrate.complete_fingerprint(),
        quiescent: run_a.naturally_quiescent
            && run_b.naturally_quiescent
            && run_joint.naturally_quiescent,
        permanent_fp,
        work: run_a.work.total()
            + run_b.work.total()
            + run_joint.work.total()
            + duplicate_run.work.total(),
    }
}

struct TemporalField {
    substrate: Substrate,
    source: CellId,
    target: CellId,
}

fn build_temporal(variant: Variant) -> TemporalField {
    let mut substrate = Substrate::new();
    let base = variant.namespace + 0x30_000;
    let source = add_cell(&mut substrate, base + 1, 0, 0, 3);
    let target = add_cell(&mut substrate, base + 2, 1, 0, 3);
    let outward = add_cell(&mut substrate, base + 3, 20, 1, 1);
    add_arrow(&mut substrate, source, target, 2, 100, 0);
    add_arrow(&mut substrate, target, outward, 1, 1_000, 1);
    TemporalField {
        substrate,
        source,
        target,
    }
}

fn temporal_contributor(field: &mut TemporalField, tick: i64, impulse: i32, origin: u64) {
    field.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: -10,
        origin_physical: origin,
        target: field.target,
        impulse,
    });
}

fn temporal_source(field: &mut TemporalField, tick: i64) {
    field.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: 0x720,
        target: field.source,
        impulse: 3,
    });
}

fn temporal_case(
    field: &mut TemporalField,
    runs: &[Execution],
    contributor_arrivals: usize,
    extra_work: u64,
) -> TemporalCase {
    let target_physical = field.substrate.cell_physical_id(field.target);
    let output = runs.iter().find_map(|run| {
        run.trace.iter().find(|entry| {
            entry.fired && entry.target_physical == target_physical && entry.impulse > 0
        })
    });
    TemporalCase {
        crossings: runs.iter().map(|run| run.crossings.len()).sum(),
        output_tick: output.map(|entry| entry.tick).unwrap_or(-1),
        output_impulse: output.map(|entry| entry.impulse).unwrap_or(0),
        contributor_arrivals,
        fingerprint: field.substrate.complete_fingerprint(),
        quiescent: runs.iter().all(|run| run.naturally_quiescent),
        work: runs.iter().map(|run| run.work.total()).sum::<u64>() + extra_work,
    }
}

fn run_temporal(variant: Variant) -> TemporalResult {
    let base = build_temporal(variant);
    let permanent_fp = base.substrate.permanent_fingerprint();

    let mut together_field = build_temporal(variant);
    temporal_contributor(&mut together_field, 0, 2, 0x710);
    temporal_source(&mut together_field, 0);
    let together_run = together_field.substrate.propagate();
    let together = temporal_case(&mut together_field, &[together_run], 1, 0);

    let mut a_then_b_field = build_temporal(variant);
    temporal_source(&mut a_then_b_field, 0);
    let a_then_b_first = a_then_b_field.substrate.propagate();
    temporal_contributor(&mut a_then_b_field, 1, 2, 0x711);
    let a_then_b_second = a_then_b_field.substrate.propagate();
    let a_then_b = temporal_case(
        &mut a_then_b_field,
        &[a_then_b_first, a_then_b_second],
        1,
        0,
    );

    let mut overlap_field = build_temporal(variant);
    temporal_contributor(&mut overlap_field, 0, 1, 0x712);
    let overlap_first = overlap_field.substrate.propagate();
    temporal_contributor(&mut overlap_field, 1, 2, 0x713);
    temporal_source(&mut overlap_field, 1);
    let overlap_second = overlap_field.substrate.propagate();
    let overlap = temporal_case(&mut overlap_field, &[overlap_first, overlap_second], 2, 0);

    let mut within_field = build_temporal(variant);
    temporal_contributor(&mut within_field, 0, 2, 0x714);
    let within_first = within_field.substrate.propagate();
    temporal_source(&mut within_field, 1);
    let within_second = within_field.substrate.propagate();
    let within = temporal_case(&mut within_field, &[within_first, within_second], 1, 0);

    let mut absent_field = build_temporal(variant);
    temporal_source(&mut absent_field, 0);
    let absent_run = absent_field.substrate.propagate();
    let absent_pressure = absent_field.substrate.advance_time(1);
    let absent = temporal_case(&mut absent_field, &[absent_run], 0, absent_pressure.total());
    let work = together.work + a_then_b.work + overlap.work + within.work + absent.work;
    TemporalResult {
        together,
        a_then_b,
        overlap,
        within,
        absent,
        permanent_fp,
        work,
    }
}

fn run_row(variant: Variant) -> Row {
    let flat = run_flat(variant);
    let recursive = run_recursive(variant);
    let convergence = run_convergence(variant);
    let temporal = run_temporal(variant);
    let g0 = true;
    let g1 = flat.training_ab == 8
        && flat.training_cd == 8
        && flat.trained_ab.crossings == 1
        && flat.trained_cd.crossings == 1
        && flat.trained_ab.output_firings == 1
        && flat.trained_cd.output_firings == 1
        && flat.crossed_ad.crossings == 0
        && flat.crossed_cb.crossings == 0;
    let g2 = flat.source_alone.crossings == 0
        && flat.source_alone.candidate_consumptions == 0
        && flat.source_alone.output_firings == 0
        && flat.contributor_alone.crossings == 0
        && flat.contributor_alone.candidate_consumptions == 0
        && flat.contributor_alone.output_firings == 0;
    let g3 = flat.changed_ad == 20
        && flat.changed_cb == 20
        && flat.new_ad.crossings == 1
        && flat.new_cb.crossings == 1
        && flat.new_ad.output_firings == 1
        && flat.new_cb.output_firings == 1
        && flat.new_ad.source_firings <= 2
        && flat.new_cb.source_firings <= 2
        && flat.old_ab.crossings == 0
        && flat.old_cd.crossings == 0
        && flat.late_old_ab.crossings == 0
        && flat.late_old_cd.crossings == 0
        && flat.old_ab_structure.live == 0
        && flat.old_cd_structure.live == 0
        && flat.new_ad_structure.live == 1
        && flat.new_cb_structure.live == 1;
    let g4 = recursive.training_crossings == 8
        && recursive.level_outputs == [1, 1, 1]
        && recursive.full_outputs == [1, 1, 1]
        && recursive.full_crossings == 1
        && recursive.missing_crossings == [0, 0, 0];
    let g5 = recursive
        .candidate
        .iter()
        .all(|arrow| arrow.live == 1 && arrow.coupling_max == 2)
        && recursive.max_source_firings <= 3;
    let g6 = convergence.source_a == 1
        && convergence.source_b == 1
        && convergence.joint == 1
        && convergence.crossings == [1, 1, 1];
    let g7 = temporal.together.crossings == 1
        && temporal.together.output_tick == 0
        && temporal.together.output_impulse == 4
        && temporal.together.contributor_arrivals == 1
        && temporal.a_then_b.crossings == 0
        && temporal.overlap.crossings == 1
        && temporal.overlap.output_tick == 1
        && temporal.overlap.output_impulse == 4
        && temporal.overlap.contributor_arrivals == 2
        && temporal.within.crossings == 1
        && temporal.within.output_tick == 1
        && temporal.within.output_impulse == 3
        && temporal.within.contributor_arrivals == 1
        && temporal.absent.crossings == 0
        && temporal.a_then_b.fingerprint != temporal.absent.fingerprint;
    let g8 = flat.duplicate_equal
        && recursive.duplicate_equal
        && convergence.duplicate_equal
        && flat.quiescent
        && recursive.quiescent
        && convergence.quiescent
        && temporal.together.quiescent
        && temporal.a_then_b.quiescent
        && temporal.overlap.quiescent
        && temporal.within.quiescent
        && temporal.absent.quiescent;
    let g9 = flat.work > 0
        && recursive.work > 0
        && convergence.work > 0
        && temporal.work > 0
        && flat.persistent_bytes > 0
        && recursive.persistent_bytes > 0;
    let clauses = [g0, g1, g2, g3, g4, g5, g6, g7, g8, g9];
    let total_work = flat.work + recursive.work + convergence.work + temporal.work;
    Row {
        variant,
        flat,
        recursive,
        convergence,
        temporal,
        clauses,
        total_work,
    }
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "variant,namespace,initial_spacing,changed_spacing,flat_initial_fp,flat_initial_acquired_fp,flat_initial_gap_fp,flat_changed_acquired_fp,flat_changed_gap_fp,flat_permanent_fp,training_ab,training_cd,trained_ab,trained_cd,crossed_ad,crossed_cb,self_crossings,self_consumptions,changed_training_ad,changed_training_cb,new_ad,new_cb,old_ab,old_cd,late_old_ab,late_old_cd,old_ab_live,old_cd_live,new_ad_live,new_ad_resistance,new_cb_live,new_cb_resistance,flat_proposals,flat_deallocations,flat_arrow_count,flat_bytes,recursive_training,recursive_level_x,recursive_level_y,recursive_level_z,recursive_full_x,recursive_full_y,recursive_full_z,recursive_full_crossings,recursive_missing_b,recursive_missing_c,recursive_missing_d,recursive_r0,recursive_r1,recursive_r2,recursive_c0,recursive_c1,recursive_c2,recursive_fp,recursive_arrows,recursive_bytes,convergent_a,convergent_b,convergent_joint,convergent_cross_a,convergent_cross_b,convergent_cross_joint,convergent_fp,together_cross,together_tick,together_impulse,a_then_b_cross,a_then_b_fp,overlap_cross,overlap_tick,overlap_impulse,overlap_arrivals,within_cross,within_tick,within_impulse,absent_cross,absent_fp,temporal_fp,flat_duplicate,recursive_duplicate,convergent_duplicate,quiescent,total_work,G0,G1,G2,G3,G4,G5,G6,G7,G8,G9,row_pass\n",
    );
    for row in rows {
        let quiescent = row.flat.quiescent
            && row.recursive.quiescent
            && row.convergence.quiescent
            && row.temporal.together.quiescent
            && row.temporal.a_then_b.quiescent
            && row.temporal.overlap.quiescent
            && row.temporal.within.quiescent
            && row.temporal.absent.quiescent;
        let mut values = vec![
            row.variant.name.to_string(),
            format!("{:#x}", row.variant.namespace),
            row.variant.initial_spacing.to_string(),
            row.variant.changed_spacing.to_string(),
            format!("{:#018x}", row.flat.initial_fp),
            format!("{:#018x}", row.flat.initial_acquired_fp),
            format!("{:#018x}", row.flat.initial_gap_fp),
            format!("{:#018x}", row.flat.changed_acquired_fp),
            format!("{:#018x}", row.flat.changed_gap_fp),
            format!("{:#018x}", row.flat.permanent_fp),
            row.flat.training_ab.to_string(),
            row.flat.training_cd.to_string(),
            row.flat.trained_ab.crossings.to_string(),
            row.flat.trained_cd.crossings.to_string(),
            row.flat.crossed_ad.crossings.to_string(),
            row.flat.crossed_cb.crossings.to_string(),
            row.flat.source_alone.crossings.to_string(),
            row.flat.source_alone.candidate_consumptions.to_string(),
            row.flat.changed_ad.to_string(),
            row.flat.changed_cb.to_string(),
            row.flat.new_ad.crossings.to_string(),
            row.flat.new_cb.crossings.to_string(),
            row.flat.old_ab.crossings.to_string(),
            row.flat.old_cd.crossings.to_string(),
            row.flat.late_old_ab.crossings.to_string(),
            row.flat.late_old_cd.crossings.to_string(),
            row.flat.old_ab_structure.live.to_string(),
            row.flat.old_cd_structure.live.to_string(),
            row.flat.new_ad_structure.live.to_string(),
            row.flat.new_ad_structure.resistance_max.to_string(),
            row.flat.new_cb_structure.live.to_string(),
            row.flat.new_cb_structure.resistance_max.to_string(),
            row.flat.proposals.to_string(),
            row.flat.deallocations.to_string(),
            row.flat.arrow_count.to_string(),
            row.flat.persistent_bytes.to_string(),
            row.recursive.training_crossings.to_string(),
            row.recursive.level_outputs[0].to_string(),
            row.recursive.level_outputs[1].to_string(),
            row.recursive.level_outputs[2].to_string(),
            row.recursive.full_outputs[0].to_string(),
            row.recursive.full_outputs[1].to_string(),
            row.recursive.full_outputs[2].to_string(),
            row.recursive.full_crossings.to_string(),
            row.recursive.missing_crossings[0].to_string(),
            row.recursive.missing_crossings[1].to_string(),
            row.recursive.missing_crossings[2].to_string(),
            row.recursive.candidate[0].resistance_max.to_string(),
            row.recursive.candidate[1].resistance_max.to_string(),
            row.recursive.candidate[2].resistance_max.to_string(),
            row.recursive.candidate[0].coupling_max.to_string(),
            row.recursive.candidate[1].coupling_max.to_string(),
            row.recursive.candidate[2].coupling_max.to_string(),
            format!("{:#018x}", row.recursive.permanent_fp),
            row.recursive.arrow_count.to_string(),
            row.recursive.persistent_bytes.to_string(),
            row.convergence.source_a.to_string(),
            row.convergence.source_b.to_string(),
            row.convergence.joint.to_string(),
            row.convergence.crossings[0].to_string(),
            row.convergence.crossings[1].to_string(),
            row.convergence.crossings[2].to_string(),
            format!("{:#018x}", row.convergence.permanent_fp),
            row.temporal.together.crossings.to_string(),
            row.temporal.together.output_tick.to_string(),
            row.temporal.together.output_impulse.to_string(),
            row.temporal.a_then_b.crossings.to_string(),
            format!("{:#018x}", row.temporal.a_then_b.fingerprint),
            row.temporal.overlap.crossings.to_string(),
            row.temporal.overlap.output_tick.to_string(),
            row.temporal.overlap.output_impulse.to_string(),
            row.temporal.overlap.contributor_arrivals.to_string(),
            row.temporal.within.crossings.to_string(),
            row.temporal.within.output_tick.to_string(),
            row.temporal.within.output_impulse.to_string(),
            row.temporal.absent.crossings.to_string(),
            format!("{:#018x}", row.temporal.absent.fingerprint),
            format!("{:#018x}", row.temporal.permanent_fp),
            row.flat.duplicate_equal.to_string(),
            row.recursive.duplicate_equal.to_string(),
            row.convergence.duplicate_equal.to_string(),
            quiescent.to_string(),
            row.total_work.to_string(),
        ];
        values.extend(row.clauses.iter().map(bool::to_string));
        values.push(row.clauses.iter().all(|value| *value).to_string());
        output.push_str(&values.join(","));
        output.push('\n');
    }
    output
}

fn report(rows: &[Row]) -> String {
    let passed = rows
        .iter()
        .filter(|row| row.clauses.iter().all(|value| *value))
        .count();
    let claims = rows
        .iter()
        .map(|row| row.clauses.iter().filter(|value| **value).count())
        .sum::<usize>();
    let work = rows.iter().map(|row| row.total_work).sum::<u64>();
    let status = if passed == rows.len() {
        "POSITIVE TERMINAL DEVELOPMENT GATE"
    } else {
        "FROZEN TERMINAL DEVELOPMENT NEGATIVE"
    };
    format!(
        "# CJ0 ARM CJ-B locally gated ARROW terminal GATE v1 result\n\nStatus: **{status}**.\n\n- conjunctive rows: `{passed}/{}`;\n- claims: `{claims}/{}`;\n- ledgered physical work: `{work}`;\n- flat trained/crossed: `{}`;\n- reversal new/old: `{}`;\n- recursive level/full/missing: `{}`;\n- ordinary convergence A/B/joint: `{}`;\n- temporal together/A-then-B/overlap/within/absent: `{}`;\n- later-stage surfaces: `none`; lane terminates here.\n\nThe physical rule and all earlier frozen sources/results remain unchanged. Exact stage fingerprints, structure, recurrence, work, storage, replay, quiescence, and clause bits are serialized in the companion CSV.\n",
        rows.len(),
        rows.len() * 10,
        rows.iter().map(|row| format!("{}|{} / {}|{}", row.flat.trained_ab.crossings, row.flat.trained_cd.crossings, row.flat.crossed_ad.crossings, row.flat.crossed_cb.crossings)).collect::<Vec<_>>().join(";"),
        rows.iter().map(|row| format!("{}|{} / {}|{}", row.flat.new_ad.crossings, row.flat.new_cb.crossings, row.flat.old_ab.crossings, row.flat.old_cd.crossings)).collect::<Vec<_>>().join(";"),
        rows.iter().map(|row| format!("{}|{}|{} / {}|{}|{} / {}|{}|{}", row.recursive.level_outputs[0], row.recursive.level_outputs[1], row.recursive.level_outputs[2], row.recursive.full_outputs[0], row.recursive.full_outputs[1], row.recursive.full_outputs[2], row.recursive.missing_crossings[0], row.recursive.missing_crossings[1], row.recursive.missing_crossings[2])).collect::<Vec<_>>().join(";"),
        rows.iter().map(|row| format!("{}|{}|{}", row.convergence.source_a, row.convergence.source_b, row.convergence.joint)).collect::<Vec<_>>().join(";"),
        rows.iter().map(|row| format!("{}|{}|{}|{}|{}", row.temporal.together.crossings, row.temporal.a_then_b.crossings, row.temporal.overlap.crossings, row.temporal.within.crossings, row.temporal.absent.crossings)).collect::<Vec<_>>().join(";")
    )
}

fn create_stage(path: &str, contents: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("cannot create {path}: {error}"))?;
    file.write_all(contents.as_bytes())
        .map_err(|error| format!("cannot write {path}: {error}"))?;
    file.sync_all()
        .map_err(|error| format!("cannot sync {path}: {error}"))
}

fn publish(rows: &[Row]) -> Result<(), String> {
    for path in [CSV_PATH, REPORT_PATH, CSV_STAGE, REPORT_STAGE] {
        if Path::new(path).exists() {
            return Err(format!("refusing to overwrite {path}"));
        }
    }
    create_stage(CSV_STAGE, &csv(rows))?;
    if let Err(error) = create_stage(REPORT_STAGE, &report(rows)) {
        let _ = fs::remove_file(CSV_STAGE);
        return Err(error);
    }
    fs::rename(CSV_STAGE, CSV_PATH).map_err(|error| format!("publish CSV failed: {error}"))?;
    fs::rename(REPORT_STAGE, REPORT_PATH)
        .map_err(|error| format!("publish report failed: {error}"))?;
    Ok(())
}

fn variants() -> [Variant; 8] {
    [
        Variant {
            name: "normal_s0",
            namespace: 0x9_b610_0000,
            mirror: false,
            reverse_cells: false,
            reverse_arrows: false,
            reverse_insertion: false,
            initial_spacing: 4,
            initial_offset: 2,
            changed_spacing: 10,
        },
        Variant {
            name: "mirror_s0",
            namespace: 0x9_b620_0000,
            mirror: true,
            reverse_cells: false,
            reverse_arrows: false,
            reverse_insertion: true,
            initial_spacing: 4,
            initial_offset: 2,
            changed_spacing: 10,
        },
        Variant {
            name: "reverse_s0",
            namespace: 0x9_b630_0000,
            mirror: false,
            reverse_cells: true,
            reverse_arrows: true,
            reverse_insertion: false,
            initial_spacing: 4,
            initial_offset: 2,
            changed_spacing: 10,
        },
        Variant {
            name: "permuted_s0",
            namespace: 0x9_b640_0000,
            mirror: true,
            reverse_cells: true,
            reverse_arrows: true,
            reverse_insertion: true,
            initial_spacing: 4,
            initial_offset: 2,
            changed_spacing: 10,
        },
        Variant {
            name: "normal_s1",
            namespace: 0x9_b650_0000,
            mirror: false,
            reverse_cells: false,
            reverse_arrows: true,
            reverse_insertion: true,
            initial_spacing: 6,
            initial_offset: 3,
            changed_spacing: 12,
        },
        Variant {
            name: "mirror_s1",
            namespace: 0x9_b660_0000,
            mirror: true,
            reverse_cells: false,
            reverse_arrows: true,
            reverse_insertion: false,
            initial_spacing: 6,
            initial_offset: 3,
            changed_spacing: 12,
        },
        Variant {
            name: "reverse_s1",
            namespace: 0x9_b670_0000,
            mirror: false,
            reverse_cells: true,
            reverse_arrows: false,
            reverse_insertion: true,
            initial_spacing: 6,
            initial_offset: 3,
            changed_spacing: 12,
        },
        Variant {
            name: "permuted_s1",
            namespace: 0x9_b680_0000,
            mirror: true,
            reverse_cells: true,
            reverse_arrows: false,
            reverse_insertion: false,
            initial_spacing: 6,
            initial_offset: 3,
            changed_spacing: 12,
        },
    ]
}

fn execute_gate() -> Result<bool, String> {
    println!("CJ0_B_LOCALLY_GATED_ARROW_GATE_V1_EVIDENCE_SPENT");
    let rows = variants().into_iter().map(run_row).collect::<Vec<_>>();
    let passed = rows
        .iter()
        .all(|row| row.clauses.iter().all(|value| *value));
    publish(&rows)?;
    Ok(passed)
}

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments == ["--preflight"] {
        println!(
            "{{\"arm\":\"CJ-B\",\"stage\":\"GATE\",\"cells_entered\":0,\"artifacts_written\":0}}"
        );
        return ExitCode::SUCCESS;
    }
    if arguments != ["--gate"] {
        eprintln!("refusing execution: expected exactly --preflight or --gate");
        return ExitCode::from(2);
    }
    match execute_gate() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
