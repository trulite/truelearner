#![forbid(unsafe_code)]

use pxr0_physical_runtime::{
    ArrowSpec, CellId, CellSpec, Crossing, PlasticSubstrate, RunResult, SpikeInput,
    TransmissionMode, Work,
};
use std::{
    collections::BTreeSet,
    env,
    fmt::Write as _,
    fs::{rename, OpenOptions},
    io::Write as _,
    path::Path,
    process::Command,
};

const OUTWARD_REGION: i16 = 1;
const WORK_BOUND: u64 = 200_000;
const MEMORY_BOUND: usize = 65_536;
const RUNTIME_SHA: &str = "09f388da26e188f14f6d9a15d9c9b16df3137bdeda55419301dbe95943b7653c";
const PXR0_ACCEPTANCE_SHA: &str =
    "fb30e4db84d5e1396b8751be16d83ca2c9ef2315f8aaee4e8a1d419630e846a7";
const PXR0_ROWS_SHA: &str = "d1bf714bdf24bbee10c362727abec02f42066cedd05ee807c88ef2c645a96d5e";
const PXR0_CONTROLS_SHA: &str =
    "6900a8d6a5a504bed95ea729acec522c5cf28e30169779cad9d34f76588fbb7f";
const PXR0_REPORT_SHA: &str =
    "82b234bb9db445922885af29fd1b31097057372dc8417ddaa88e75cea4758848";

const DEVELOPMENT_CASES: [Case; 16] = [
    Case::new(3_100_001, false, false, 0),
    Case::new(3_100_002, false, false, 130),
    Case::new(3_100_003, false, false, 260),
    Case::new(3_100_004, false, false, 390),
    Case::new(3_100_005, true, false, 0),
    Case::new(3_100_006, true, false, 130),
    Case::new(3_100_007, true, false, 260),
    Case::new(3_100_008, true, false, 390),
    Case::new(3_100_009, false, true, 0),
    Case::new(3_100_010, false, true, 130),
    Case::new(3_100_011, false, true, 260),
    Case::new(3_100_012, false, true, 390),
    Case::new(3_100_013, true, true, 0),
    Case::new(3_100_014, true, true, 130),
    Case::new(3_100_015, true, true, 260),
    Case::new(3_100_016, true, true, 390),
];

const AUTHORITY_CASES: [Case; 16] = [
    Case::new(3_200_001, false, false, 520),
    Case::new(3_200_002, false, false, 650),
    Case::new(3_200_003, false, false, 780),
    Case::new(3_200_004, false, false, 910),
    Case::new(3_200_005, true, false, 520),
    Case::new(3_200_006, true, false, 650),
    Case::new(3_200_007, true, false, 780),
    Case::new(3_200_008, true, false, 910),
    Case::new(3_200_009, false, true, 520),
    Case::new(3_200_010, false, true, 650),
    Case::new(3_200_011, false, true, 780),
    Case::new(3_200_012, false, true, 910),
    Case::new(3_200_013, true, true, 520),
    Case::new(3_200_014, true, true, 650),
    Case::new(3_200_015, true, true, 780),
    Case::new(3_200_016, true, true, 910),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Development,
    Authority,
}

impl Mode {
    fn parse() -> Self {
        match env::args().nth(1).as_deref() {
            Some("--development") if env::args().count() == 2 => Self::Development,
            Some("--authority") if env::args().count() == 2 => Self::Authority,
            _ => {
                eprintln!("expected exactly --development or --authority");
                std::process::exit(2);
            }
        }
    }

    fn cases(self) -> [Case; 16] {
        match self {
            Self::Development => DEVELOPMENT_CASES,
            Self::Authority => AUTHORITY_CASES,
        }
    }

    fn csv(self) -> &'static str {
        match self {
            Self::Development => "results/pxc_continuous_development_v1.csv",
            Self::Authority => "results/pxc_continuous_authority_v1.csv",
        }
    }

    fn report(self) -> &'static str {
        match self {
            Self::Development => "results/pxc_continuous_development_v1.md",
            Self::Authority => "results/pxc_continuous_authority_v1.md",
        }
    }

    fn marker(self) -> &'static str {
        match self {
            Self::Development => "PXC_CONTINUOUS_DEVELOPMENT_EVIDENCE_SPENT_V1",
            Self::Authority => "PXC_CONTINUOUS_AUTHORITY_EVIDENCE_SPENT_V1",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Development => "PX-C continuous-organism development v1",
            Self::Authority => "PX-C continuous-organism authority v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Case {
    root: u64,
    reverse: bool,
    reflect: bool,
    origin: i64,
}

impl Case {
    const fn new(root: u64, reverse: bool, reflect: bool, origin: i64) -> Self {
        Self {
            root,
            reverse,
            reflect,
            origin,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Batch {
    crossings: Vec<Crossing>,
    work: Work,
    quiet: bool,
    bytes: usize,
}

impl Batch {
    fn from(result: RunResult) -> Self {
        Self {
            crossings: result.crossings,
            work: result.work,
            quiet: result.naturally_quiescent,
            bytes: result.resident_bytes,
        }
    }

    fn count(&self, from: u64, to: u64) -> usize {
        self.crossings
            .iter()
            .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
            .count()
    }

    fn outward_only(&self) -> bool {
        self.crossings
            .iter()
            .all(|crossing| crossing.to_region == OUTWARD_REGION)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Trial {
    case: Case,
    construction_tick: i64,
    pressure_origin: i64,
    first_arrival_tick: i64,
    blank: Batch,
    batches: Vec<Batch>,
    pause_work: Work,
    age_work: Work,
    paired_held: usize,
    selective_held: usize,
    retained: usize,
    partial: usize,
    adjacent_first: usize,
    adjacent_second: usize,
    duplicated: usize,
    resisted: usize,
    direct: usize,
    duplicate_direct: usize,
    open: usize,
    fork: usize,
    ring: usize,
    aged: usize,
    formation_updates: u64,
    formation_modulation: u64,
    max_work: u64,
    max_bytes: usize,
    last_arrival_tick: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    trial: Trial,
    clauses: [bool; 32],
    replay: bool,
    passed: bool,
}

#[derive(Clone, Copy)]
struct Layout {
    namespace: u64,
    position_base: i32,
    reverse: bool,
    reflect: bool,
    outward_resistance: u32,
}

#[derive(Clone, Copy)]
struct CascadeSites {
    namespace: u64,
    primitive: [CellId; 4],
    stages: [CellId; 3],
    context: CellId,
    returning: CellId,
    outward_from: u64,
    outward_to: u64,
}

#[derive(Clone, Copy)]
struct PairSites {
    namespace: u64,
    sources: [CellId; 4],
    downstream: [CellId; 2],
    returning: CellId,
    outward_from: u64,
    outward_to: u64,
}

#[derive(Clone, Copy)]
struct CompactSites {
    namespace: u64,
    inlet: CellId,
    outward_from: u64,
    outward_to: u64,
}

#[derive(Clone, Copy)]
struct DriveSites {
    namespace: u64,
    inlet: CellId,
}

fn main() {
    let mode = Mode::parse();
    println!("{}", mode.marker());
    let pxr0_exact = pxr0_inputs_exact();
    let rows = mode
        .cases()
        .into_iter()
        .map(|case| replay(case, pxr0_exact))
        .collect::<Vec<_>>();
    publish(mode.csv(), &csv(&rows));
    let globals = globals(mode, &rows, pxr0_exact);
    publish(mode.report(), &markdown(mode, &rows, globals));
    assert!(rows.iter().all(|row| row.passed), "PX-C row failed");
    assert!(globals.into_iter().all(|value| value), "PX-C global failed");
    println!("PXC_CONTINUOUS_{}_V1 rows=16/16 clauses=524/524", match mode {
        Mode::Development => "DEVELOPMENT_READY",
        Mode::Authority => "AUTHORITY_ESTABLISHED",
    });
}

fn replay(case: Case, pxr0_exact: bool) -> Row {
    let first = run(case);
    let second = run(case);
    let exact = first == second;
    row(first, exact, pxr0_exact)
}

fn run(case: Case) -> Trial {
    let mut space = PlasticSubstrate::new();
    space.advance_time(case.origin);
    let root_namespace = case.root << 32;
    let retained = build_cascade(
        &mut space,
        layout(case, root_namespace + 100_000, 1, 100),
    );
    let partial = build_cascade(
        &mut space,
        layout(case, root_namespace + 200_000, 2, 100),
    );
    let adjacent = build_cascade(
        &mut space,
        layout(case, root_namespace + 300_000, 3, 100),
    );
    let duplicated = build_cascade(
        &mut space,
        layout(case, root_namespace + 400_000, 4, 100),
    );
    let resisted = build_cascade(
        &mut space,
        layout(case, root_namespace + 500_000, 5, 0),
    );
    let aged = build_cascade(
        &mut space,
        layout(case, root_namespace + 600_000, 6, 100),
    );
    let cascades = [retained, partial, adjacent, duplicated, resisted];
    let paired = build_pair(
        &mut space,
        layout(case, root_namespace + 700_000, 7, 100),
        1,
    );
    let selective = build_pair(
        &mut space,
        layout(case, root_namespace + 800_000, 8, 100),
        0,
    );
    let direct = build_direct(
        &mut space,
        layout(case, root_namespace + 900_000, 9, 100),
    );
    let duplicate_direct = build_direct(
        &mut space,
        layout(case, root_namespace + 1_000_000, 10, 100),
    );
    let open = build_open(
        &mut space,
        layout(case, root_namespace + 1_100_000, 11, 100),
    );
    let fork = build_fork(
        &mut space,
        layout(case, root_namespace + 1_200_000, 12, 100),
    );
    let ring = build_ring(
        &mut space,
        layout(case, root_namespace + 1_300_000, 13, 100),
    );
    let drive_only = build_drive_only(
        &mut space,
        layout(case, root_namespace + 1_400_000, 14, 100),
    );

    let blank = Batch::from(space.arrive(&[], OUTWARD_REGION));
    let mut batches = Vec::new();

    let mut inputs = Vec::new();
    inputs.extend(pair_maturation(paired, case.origin, 0));
    inputs.extend(pair_selective(selective, case.origin, 0));
    for sites in cascades {
        inputs.extend(cascade_burst(sites, case.origin, 1, 0));
    }
    inputs.extend(cascade_burst(aged, case.origin, 1, 0));
    batches.push(Batch::from(space.arrive(&inputs, OUTWARD_REGION)));

    inputs.clear();
    inputs.extend(pair_maturation(paired, case.origin, 10));
    inputs.extend(pair_boundary(selective, case.origin, 10, false));
    for sites in cascades {
        inputs.extend(cascade_burst(sites, case.origin, 1, 11));
    }
    inputs.extend(cascade_burst(aged, case.origin, 2, 11));
    batches.push(Batch::from(space.arrive(&inputs, OUTWARD_REGION)));

    inputs.clear();
    inputs.extend(pair_boundary(paired, case.origin, 20, true));
    for sites in cascades {
        inputs.extend(cascade_burst(sites, case.origin, 2, 20));
    }
    inputs.extend(cascade_burst(aged, case.origin, 3, 22));
    batches.push(Batch::from(space.arrive(&inputs, OUTWARD_REGION)));

    for (depth, start) in [(2, 31), (3, 40), (3, 51)] {
        inputs.clear();
        for sites in cascades {
            inputs.extend(cascade_burst(sites, case.origin, depth, start));
        }
        batches.push(Batch::from(space.arrive(&inputs, OUTWARD_REGION)));
    }

    let pause_work = space.advance_time(case.origin + 60);

    inputs.clear();
    inputs.extend(cascade_reuse(retained, case.origin, [true; 4], 61, false));
    inputs.extend(cascade_reuse(
        partial,
        case.origin,
        [true, true, true, false],
        61,
        false,
    ));
    inputs.extend(cascade_reuse(
        adjacent,
        case.origin,
        [true, true, false, false],
        61,
        false,
    ));
    inputs.extend(cascade_reuse(
        duplicated,
        case.origin,
        [true; 4],
        61,
        true,
    ));
    inputs.extend(cascade_reuse(resisted, case.origin, [true; 4], 61, false));
    inputs.extend(compact_inputs(direct, case.origin, 61, 1));
    inputs.extend(compact_inputs(duplicate_direct, case.origin, 61, 2));
    inputs.extend(compact_inputs(open, case.origin, 61, 1));
    inputs.extend(compact_inputs(fork, case.origin, 61, 1));
    inputs.extend(compact_inputs(ring, case.origin, 61, 1));
    batches.push(Batch::from(space.arrive(&inputs, OUTWARD_REGION)));

    inputs = cascade_reuse(
        adjacent,
        case.origin,
        [true, true, true, false],
        70,
        false,
    );
    batches.push(Batch::from(space.arrive(&inputs, OUTWARD_REGION)));

    let age_work = space.advance_time(case.origin + 110);
    inputs = cascade_changed(aged, case.origin, 111);
    batches.push(Batch::from(space.arrive(&inputs, OUTWARD_REGION)));

    inputs = vec![physical_input(
        drive_only.namespace,
        drive_only.inlet,
        case.origin + 120,
        1,
        901,
    )];
    batches.push(Batch::from(space.arrive(&inputs, OUTWARD_REGION)));

    let paired_held = batches[2].count(paired.outward_from, paired.outward_to);
    let selective_held = batches[1].count(selective.outward_from, selective.outward_to);
    let retained_count = batches[6].count(retained.outward_from, retained.outward_to);
    let partial_count = batches[6].count(partial.outward_from, partial.outward_to);
    let adjacent_first = batches[6].count(adjacent.outward_from, adjacent.outward_to);
    let adjacent_second = batches[7].count(adjacent.outward_from, adjacent.outward_to);
    let duplicated_count = batches[6].count(duplicated.outward_from, duplicated.outward_to);
    let resisted_count = batches[6].count(resisted.outward_from, resisted.outward_to);
    let direct_count = batches[6].count(direct.outward_from, direct.outward_to);
    let duplicate_direct_count =
        batches[6].count(duplicate_direct.outward_from, duplicate_direct.outward_to);
    let open_count = batches[6].count(open.outward_from, open.outward_to);
    let fork_count = batches[6].count(fork.outward_from, fork.outward_to);
    let ring_count = batches[6].count(ring.outward_from, ring.outward_to);
    let aged_count = batches[8].count(aged.outward_from, aged.outward_to);
    let formation_updates = batches[..6]
        .iter()
        .map(|batch| batch.work.local_return_updates)
        .sum();
    let formation_modulation = batches[..6]
        .iter()
        .map(|batch| batch.work.modulatory_deliveries)
        .sum();
    let max_work = batches
        .iter()
        .map(|batch| batch.work.total())
        .chain([blank.work.total(), pause_work.total(), age_work.total()])
        .max()
        .unwrap_or(0);
    let max_bytes = batches
        .iter()
        .map(|batch| batch.bytes)
        .chain([blank.bytes])
        .max()
        .unwrap_or(0);

    Trial {
        case,
        construction_tick: case.origin,
        pressure_origin: case.origin,
        first_arrival_tick: case.origin,
        blank,
        batches,
        pause_work,
        age_work,
        paired_held,
        selective_held,
        retained: retained_count,
        partial: partial_count,
        adjacent_first,
        adjacent_second,
        duplicated: duplicated_count,
        resisted: resisted_count,
        direct: direct_count,
        duplicate_direct: duplicate_direct_count,
        open: open_count,
        fork: fork_count,
        ring: ring_count,
        aged: aged_count,
        formation_updates,
        formation_modulation,
        max_work,
        max_bytes,
        last_arrival_tick: case.origin + 120,
    }
}

fn row(trial: Trial, replay: bool, pxr0_exact: bool) -> Row {
    let every_quiet = trial.batches.iter().all(|batch| batch.quiet);
    let outward_only = trial.blank.outward_only()
        && trial.batches.iter().all(Batch::outward_only);
    let formation_each_updates = trial.batches[..6]
        .iter()
        .all(|batch| batch.work.local_return_updates > 0);
    let mut clauses = [false; 32];
    clauses[0] = trial.blank.crossings.is_empty() && trial.blank.quiet;
    clauses[1] = every_quiet;
    clauses[2] = outward_only;
    clauses[3] = trial.case.origin.rem_euclid(10) == 0
        && trial.construction_tick == trial.case.origin
        && trial.pressure_origin == trial.case.origin
        && trial.first_arrival_tick == trial.case.origin;
    clauses[4] = trial.batches.len() == 10 && trial.last_arrival_tick == trial.case.origin + 120;
    clauses[5] = trial.batches[0].work.local_return_updates > 0;
    clauses[6] = trial.paired_held == 1;
    clauses[7] = trial.selective_held == 1;
    clauses[8] = formation_each_updates && trial.formation_updates > 0;
    clauses[9] = trial.retained == 1;
    clauses[10] = trial.partial == 0;
    clauses[11] = trial.adjacent_first == 0;
    clauses[12] = trial.adjacent_second == 0;
    clauses[13] = trial.duplicated == 1;
    clauses[14] = trial.resisted == 0;
    clauses[15] = trial.direct == 1;
    clauses[16] = trial.duplicate_direct == 1;
    clauses[17] = trial.open == 0;
    clauses[18] = trial.fork == 0;
    clauses[19] = trial.ring == 0;
    clauses[20] = trial.aged == 0;
    clauses[21] = trial.batches[8].work.local_structural_proposals == 1;
    clauses[22] = trial.age_work.physical_deallocations > 0;
    clauses[23] = trial.batches[9].work.local_return_updates == 0
        && trial.batches[9].work.modulatory_deliveries == 0;
    clauses[24] = trial.batches[9].work.drive_deliveries > 0;
    clauses[25] = trial.formation_modulation > 0 && trial.formation_updates > 0;
    clauses[26] = trial.pause_work.total() <= WORK_BOUND && trial.retained == 1;
    clauses[27] = trial.max_work <= WORK_BOUND;
    clauses[28] = trial.max_bytes <= MEMORY_BOUND;
    clauses[29] = replay;
    clauses[30] = pxr0_exact;
    clauses[31] = clauses[..31].iter().all(|value| *value);
    let passed = clauses.into_iter().all(|value| value);
    Row {
        trial,
        clauses,
        replay,
        passed,
    }
}

fn build_cascade(space: &mut PlasticSubstrate, layout: Layout) -> CascadeSites {
    let sides = if layout.reverse { [3, 2, 1, 0] } else { [0, 1, 2, 3] };
    let stages_order = if layout.reverse { [2, 1, 0] } else { [0, 1, 2] };
    let mut primitive = [None; 4];
    let mut outlets = [None; 4];
    let mut traces = [None; 4];
    let mut hubs = [None; 4];
    for side in sides {
        primitive[side] = Some(space.add_cell(cell(
            physical(layout, 10 + side as u64),
            located(layout, -100_000 - side as i32 * 1_000),
            0,
            1,
        )));
        outlets[side] = Some(space.add_cell(cell(
            physical(layout, 20 + side as u64),
            located(layout, -90_000 - side as i32 * 1_000),
            0,
            1,
        )));
        traces[side] = Some(space.add_cell(cell(
            physical(layout, 30 + side as u64),
            located(layout, -80_000 - side as i32 * 1_000),
            0,
            2,
        )));
        hubs[side] = Some(space.add_cell(cell(
            physical(layout, 40 + side as u64),
            located(layout, -70_000 - side as i32 * 1_000),
            0,
            1,
        )));
    }
    let primitive = primitive.map(Option::unwrap);
    let outlets = outlets.map(Option::unwrap);
    let traces = traces.map(Option::unwrap);
    let hubs = hubs.map(Option::unwrap);
    let mut opportunities = [None; 3];
    let mut stages = [None; 3];
    let mut outputs = [None; 3];
    let mut source_traces = [None; 3];
    let mut output_traces = [None; 3];
    let mut output_hubs = [None; 3];
    let mut return_loci = [None; 3];
    for stage in stages_order {
        opportunities[stage] = Some(space.add_cell(cell(
            physical(layout, 100 + stage as u64),
            located(layout, -10_000 - stage as i32 * 1_000),
            0,
            2,
        )));
        let position = 10_000 + stage as i32 * 1_000;
        stages[stage] = Some(space.add_cell(cell(
            physical(layout, 200 + stage as u64),
            located(layout, position),
            0,
            2,
        )));
        outputs[stage] = Some(space.add_cell(cell(
            physical(layout, 300 + stage as u64),
            located(layout, position + 1),
            0,
            2,
        )));
        source_traces[stage] = Some(space.add_cell(cell(
            physical(layout, 400 + stage as u64),
            located(layout, 30_000 + stage as i32 * 1_000),
            0,
            2,
        )));
        output_traces[stage] = Some(space.add_cell(cell(
            physical(layout, 600 + stage as u64),
            located(layout, 50_000 + stage as i32 * 1_000),
            0,
            2,
        )));
        output_hubs[stage] = Some(space.add_cell(cell(
            physical(layout, 700 + stage as u64),
            located(layout, 60_000 + stage as i32 * 1_000),
            0,
            1,
        )));
        return_loci[stage] = Some(space.add_cell(cell(
            physical(layout, 800 + stage as u64),
            located(layout, 70_000 + stage as i32 * 1_000),
            0,
            1,
        )));
    }
    let opportunities = opportunities.map(Option::unwrap);
    let stages = stages.map(Option::unwrap);
    let outputs = outputs.map(Option::unwrap);
    let source_traces = source_traces.map(Option::unwrap);
    let output_traces = output_traces.map(Option::unwrap);
    let output_hubs = output_hubs.map(Option::unwrap);
    let return_loci = return_loci.map(Option::unwrap);
    let context = space.add_cell(cell(physical(layout, 900), located(layout, 90_000), 0, 1));
    let returning = space.add_cell(cell(physical(layout, 901), located(layout, 100_000), 0, 1));
    let outward = space.add_cell(cell(
        physical(layout, 950),
        located(layout, 120_000),
        OUTWARD_REGION,
        1,
    ));
    let relay = space.add_cell(cell(
        physical(layout, 951),
        located(layout, 121_000),
        OUTWARD_REGION,
        1,
    ));
    for side in sides {
        space.add_arrow(drive(primitive[side], outlets[side], 0, 1, 100));
        normalize(space, outlets[side], traces[side], hubs[side]);
    }
    for stage in stages_order {
        normalize(space, outputs[stage], output_traces[stage], output_hubs[stage]);
    }
    let left = [traces[0], output_traces[0], output_traces[1]];
    let right = [traces[1], traces[2], traces[3]];
    for stage in stages_order {
        space.add_arrow(drive(left[stage], opportunities[stage], 0, 1, 100));
        space.add_arrow(drive(right[stage], opportunities[stage], 0, 1, 100));
        space.add_arrow(drive(opportunities[stage], stages[stage], 0, 1, 100));
        space.add_arrow(drive(context, outputs[stage], 1, 1, 100));
        space.add_arrow(drive(output_traces[stage], source_traces[stage], 0, 1, 100));
        space.add_arrow(drive(returning, source_traces[stage], 0, 1, 100));
        space.add_arrow(drive(source_traces[stage], return_loci[stage], 0, 1, 100));
        space.add_arrow(modulatory(return_loci[stage], stages[stage], 1, 1, 100));
    }
    space.add_arrow(drive(
        output_traces[2],
        outward,
        0,
        1,
        layout.outward_resistance,
    ));
    space.add_arrow(drive(outward, relay, 1, 1, 100));
    space.add_arrow(modulatory(relay, stages[2], 1, 1, 100));
    CascadeSites {
        namespace: layout.namespace,
        primitive,
        stages,
        context,
        returning,
        outward_from: physical(layout, 602),
        outward_to: physical(layout, 950),
    }
}

fn build_pair(space: &mut PlasticSubstrate, layout: Layout, output_pair: usize) -> PairSites {
    let sides = if layout.reverse { [3, 2, 1, 0] } else { [0, 1, 2, 3] };
    let pairs = if layout.reverse { [1, 0] } else { [0, 1] };
    let mut sources = [None; 4];
    let mut traces = [None; 4];
    for side in sides {
        sources[side] = Some(space.add_cell(cell(
            layout.namespace + 100 + side as u64,
            located(layout, 100_000 + side as i32 * 10_000),
            0,
            1,
        )));
        traces[side] = Some(space.add_cell(cell(
            layout.namespace + 200 + side as u64,
            located(layout, 500_000 + (20 + side as i32) * 10_000),
            0,
            1,
        )));
    }
    let sources = sources.map(Option::unwrap);
    let traces = traces.map(Option::unwrap);
    let mut coincidence = [None; 2];
    let mut inner = [None; 2];
    let mut downstream = [None; 2];
    let mut downstream_trace = [None; 2];
    let mut relay = [None; 2];
    for pair in pairs {
        coincidence[pair] = Some(space.add_cell(cell(
            layout.namespace + 300 + pair as u64,
            located(layout, 500_000 + (40 + pair as i32) * 10_000),
            0,
            2,
        )));
        inner[pair] = Some(space.add_cell(cell(
            layout.namespace + 400 + pair as u64,
            located(layout, 100_000 + (4 + pair as i32 * 2) * 10_000),
            0,
            1,
        )));
        downstream[pair] = Some(space.add_cell(cell(
            layout.namespace + 500 + pair as u64,
            located(layout, 100_000 + (5 + pair as i32 * 2) * 10_000),
            0,
            2,
        )));
        downstream_trace[pair] = Some(space.add_cell(cell(
            layout.namespace + 600 + pair as u64,
            located(layout, 500_000 + (60 + pair as i32) * 10_000),
            0,
            1,
        )));
        relay[pair] = Some(space.add_cell(cell(
            layout.namespace + 800 + pair as u64,
            located(layout, 500_000 + (80 + pair as i32) * 10_000),
            0,
            2,
        )));
    }
    let coincidence = coincidence.map(Option::unwrap);
    let inner = inner.map(Option::unwrap);
    let downstream = downstream.map(Option::unwrap);
    let downstream_trace = downstream_trace.map(Option::unwrap);
    let relay = relay.map(Option::unwrap);
    let returning = space.add_cell(cell(
        layout.namespace + 700,
        located(layout, 180_000),
        0,
        1,
    ));
    let outward = space.add_cell(cell(
        layout.namespace + 900,
        located(layout, 190_000),
        OUTWARD_REGION,
        1,
    ));
    for side in sides {
        space.add_arrow(drive(sources[side], traces[side], 1, 1, 100));
    }
    space.add_arrow(drive(traces[0], coincidence[0], 0, 1, 100));
    space.add_arrow(drive(traces[1], coincidence[0], 0, 1, 100));
    space.add_arrow(drive(coincidence[0], inner[0], 0, 1, 100));
    space.add_arrow(drive(downstream_trace[0], coincidence[1], 0, 1, 100));
    space.add_arrow(drive(traces[2], coincidence[1], 0, 1, 100));
    space.add_arrow(drive(coincidence[1], inner[1], 0, 1, 100));
    space.add_arrow(drive(inner[0], downstream[0], 1, 1, 1));
    space.add_arrow(drive(inner[1], downstream[1], 1, 1, 1));
    for pair in pairs {
        space.add_arrow(drive(downstream[pair], downstream_trace[pair], 1, 1, 100));
        space.add_arrow(drive(downstream_trace[pair], relay[pair], 0, 1, 100));
        space.add_arrow(drive(returning, relay[pair], 0, 1, 100));
        space.add_arrow(modulatory(relay[pair], inner[pair], 1, 1, 100));
    }
    space.add_arrow(drive(downstream[output_pair], outward, 1, 1, 100));
    PairSites {
        namespace: layout.namespace,
        sources,
        downstream,
        returning,
        outward_from: layout.namespace + 500 + output_pair as u64,
        outward_to: layout.namespace + 900,
    }
}

fn build_direct(space: &mut PlasticSubstrate, layout: Layout) -> CompactSites {
    let inlet = space.add_cell(cell(layout.namespace + 1, located(layout, 0), 0, 1));
    let outward = space.add_cell(cell(
        layout.namespace + 2,
        located(layout, 10),
        OUTWARD_REGION,
        1,
    ));
    space.add_arrow(drive(inlet, outward, 1, 1, 100));
    CompactSites {
        namespace: layout.namespace,
        inlet,
        outward_from: layout.namespace + 1,
        outward_to: layout.namespace + 2,
    }
}

fn build_open(space: &mut PlasticSubstrate, layout: Layout) -> CompactSites {
    let inlet = space.add_cell(cell(layout.namespace + 1, located(layout, 0), 0, 1));
    let outward = space.add_cell(cell(
        layout.namespace + 2,
        located(layout, 10),
        OUTWARD_REGION,
        1,
    ));
    space.add_arrow(drive(inlet, outward, 1, 1, 0));
    CompactSites {
        namespace: layout.namespace,
        inlet,
        outward_from: layout.namespace + 1,
        outward_to: layout.namespace + 2,
    }
}

fn build_fork(space: &mut PlasticSubstrate, layout: Layout) -> CompactSites {
    let inlet = space.add_cell(cell(layout.namespace + 1, located(layout, 0), 0, 1));
    let a = space.add_cell(cell(layout.namespace + 3, located(layout, 2), 0, 1));
    let b = space.add_cell(cell(layout.namespace + 4, located(layout, 3), 0, 1));
    let join = space.add_cell(cell(layout.namespace + 5, located(layout, 4), 0, 3));
    let outward = space.add_cell(cell(
        layout.namespace + 2,
        located(layout, 10),
        OUTWARD_REGION,
        1,
    ));
    space.add_arrow(drive(inlet, a, 1, 1, 100));
    space.add_arrow(drive(inlet, b, 1, 1, 100));
    space.add_arrow(drive(a, join, 1, 1, 100));
    space.add_arrow(drive(b, join, 1, 1, 100));
    space.add_arrow(drive(join, outward, 1, 1, 100));
    CompactSites {
        namespace: layout.namespace,
        inlet,
        outward_from: layout.namespace + 5,
        outward_to: layout.namespace + 2,
    }
}

fn build_ring(space: &mut PlasticSubstrate, layout: Layout) -> CompactSites {
    let inlet = space.add_cell(cell(layout.namespace + 1, located(layout, 0), 0, 1));
    let a = space.add_cell(cell(layout.namespace + 3, located(layout, 2), 0, 2));
    let b = space.add_cell(cell(layout.namespace + 4, located(layout, 3), 0, 2));
    let outward = space.add_cell(cell(
        layout.namespace + 2,
        located(layout, 10),
        OUTWARD_REGION,
        1,
    ));
    space.add_arrow(drive(inlet, a, 1, 1, 100));
    space.add_arrow(drive(a, b, 1, 1, 100));
    space.add_arrow(drive(b, a, 1, 1, 100));
    space.add_arrow(drive(b, outward, 1, 1, 100));
    CompactSites {
        namespace: layout.namespace,
        inlet,
        outward_from: layout.namespace + 4,
        outward_to: layout.namespace + 2,
    }
}

fn build_drive_only(space: &mut PlasticSubstrate, layout: Layout) -> DriveSites {
    let inlet = space.add_cell(cell(layout.namespace + 1, located(layout, 0), 0, 1));
    let target = space.add_cell(cell(layout.namespace + 2, located(layout, 10), 0, 1));
    space.add_arrow(drive(inlet, target, 1, 1, 100));
    DriveSites {
        namespace: layout.namespace,
        inlet,
    }
}

fn cascade_burst(
    sites: CascadeSites,
    origin: i64,
    depth: usize,
    start: i64,
) -> Vec<SpikeInput> {
    let mut inputs = Vec::new();
    for side in 0..=depth {
        inputs.push(physical_input(
            sites.namespace,
            sites.primitive[side],
            origin + start + [0, 0, 2, 4][side],
            1,
            side as i32,
        ));
    }
    for stage in 0..depth {
        let tick = origin + start + 1 + stage as i64 * 2;
        inputs.push(physical_input(
            sites.namespace,
            sites.stages[stage],
            tick,
            1,
            100 + stage as i32,
        ));
        inputs.push(physical_input(
            sites.namespace,
            sites.context,
            tick,
            1,
            500 + stage as i32,
        ));
        inputs.push(physical_input(
            sites.namespace,
            sites.returning,
            tick + 2,
            1,
            600 + stage as i32,
        ));
    }
    inputs
}

fn cascade_reuse(
    sites: CascadeSites,
    origin: i64,
    present: [bool; 4],
    start: i64,
    duplicate: bool,
) -> Vec<SpikeInput> {
    let mut inputs = Vec::new();
    for (side, admitted) in present.into_iter().enumerate() {
        if admitted {
            let tick = origin + start + [0, 0, 2, 4][side];
            inputs.push(physical_input(
                sites.namespace,
                sites.primitive[side],
                tick,
                1,
                side as i32,
            ));
            if duplicate {
                inputs.push(physical_input(
                    sites.namespace,
                    sites.primitive[side],
                    tick,
                    1,
                    10 + side as i32,
                ));
            }
        }
    }
    for stage in 0..3 {
        inputs.push(physical_input(
            sites.namespace,
            sites.stages[stage],
            origin + start + 1 + stage as i64 * 2,
            1,
            100 + stage as i32,
        ));
    }
    inputs
}

fn cascade_changed(sites: CascadeSites, origin: i64, start: i64) -> Vec<SpikeInput> {
    vec![physical_input(
        sites.namespace,
        sites.stages[0],
        origin + start,
        2,
        900,
    )]
}

fn pair_maturation(sites: PairSites, origin: i64, start: i64) -> Vec<SpikeInput> {
    vec![
        physical_input(sites.namespace, sites.sources[0], origin + start, 1, 10),
        physical_input(sites.namespace, sites.sources[1], origin + start, 1, 11),
        physical_input(sites.namespace, sites.sources[2], origin + start + 2, 1, 12),
        physical_input(sites.namespace, sites.downstream[0], origin + start + 2, 1, 20),
        physical_input(sites.namespace, sites.returning, origin + start + 3, 1, 21),
        physical_input(sites.namespace, sites.downstream[1], origin + start + 4, 1, 22),
        physical_input(sites.namespace, sites.returning, origin + start + 5, 1, 23),
    ]
}

fn pair_selective(sites: PairSites, origin: i64, start: i64) -> Vec<SpikeInput> {
    vec![
        physical_input(sites.namespace, sites.sources[0], origin + start, 1, 10),
        physical_input(sites.namespace, sites.sources[1], origin + start, 1, 11),
        physical_input(sites.namespace, sites.downstream[0], origin + start + 2, 1, 20),
        physical_input(sites.namespace, sites.returning, origin + start + 3, 1, 21),
    ]
}

fn pair_boundary(
    sites: PairSites,
    origin: i64,
    start: i64,
    include_third: bool,
) -> Vec<SpikeInput> {
    let mut inputs = vec![
        physical_input(sites.namespace, sites.sources[0], origin + start, 1, 10),
        physical_input(sites.namespace, sites.sources[1], origin + start, 1, 11),
    ];
    if include_third {
        inputs.push(physical_input(
            sites.namespace,
            sites.sources[2],
            origin + start + 2,
            1,
            12,
        ));
    }
    inputs
}

fn compact_inputs(
    sites: CompactSites,
    origin: i64,
    start: i64,
    count: usize,
) -> Vec<SpikeInput> {
    (0..count)
        .map(|index| {
            physical_input(
                sites.namespace,
                sites.inlet,
                origin + start,
                1,
                index as i32,
            )
        })
        .collect()
}

fn physical_input(
    namespace: u64,
    target: CellId,
    tick: i64,
    impulse: i32,
    phase: i32,
) -> SpikeInput {
    SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: namespace + 50_000 + phase as u64,
        target,
        impulse,
    }
}

fn layout(case: Case, namespace: u64, index: i32, outward_resistance: u32) -> Layout {
    Layout {
        namespace,
        position_base: index * 2_000_000,
        reverse: case.reverse,
        reflect: case.reflect,
        outward_resistance,
    }
}

fn located(layout: Layout, offset: i32) -> i32 {
    layout.position_base + if layout.reflect { -offset } else { offset }
}

fn physical(layout: Layout, suffix: u64) -> u64 {
    layout.namespace + suffix.wrapping_mul(73) % 10_000
}

fn normalize(space: &mut PlasticSubstrate, outlet: CellId, trace: CellId, hub: CellId) {
    space.add_arrow(drive(outlet, trace, 1, 1, 100));
    space.add_arrow(drive(outlet, hub, 1, 1, 100));
    space.add_arrow(drive(hub, trace, 0, 1, 100));
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 100,
    }
}

fn drive(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode: TransmissionMode::Drive,
    }
}

fn modulatory(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode: TransmissionMode::Modulatory,
    }
}

fn pxr0_inputs_exact() -> bool {
    sha("results/pxr0_v2_acceptance_v1/audit.json").as_deref() == Some(PXR0_ACCEPTANCE_SHA)
        && sha("results/pxr0_successor_readiness_v2.csv").as_deref() == Some(PXR0_ROWS_SHA)
        && sha("results/pxr0_phase_controls_v2.csv").as_deref() == Some(PXR0_CONTROLS_SHA)
        && sha("results/pxr0_successor_readiness_v2.md").as_deref() == Some(PXR0_REPORT_SHA)
}

fn globals(mode: Mode, rows: &[Row], pxr0_exact: bool) -> [bool; 12] {
    let roots = rows
        .iter()
        .map(|row| row.trial.case.root)
        .collect::<BTreeSet<_>>();
    let expected_start = match mode {
        Mode::Development => 3_100_001,
        Mode::Authority => 3_200_001,
    };
    let layouts = [false, true].into_iter().all(|reverse| {
        [false, true].into_iter().all(|reflect| {
            rows.iter()
                .filter(|row| {
                    row.trial.case.reverse == reverse && row.trial.case.reflect == reflect
                })
                .count()
                == 4
        })
    });
    let expected_origins = match mode {
        Mode::Development => [0, 130, 260, 390],
        Mode::Authority => [520, 650, 780, 910],
    };
    let origins = expected_origins.into_iter().all(|origin| {
        rows.iter()
            .filter(|row| row.trial.case.origin == origin)
            .count()
            == 4
    });
    let timing = rows.iter().all(|row| {
        row.trial.case.origin.rem_euclid(10) == 0
            && row.trial.construction_tick == row.trial.case.origin
            && row.trial.pressure_origin == row.trial.case.origin
            && row.trial.first_arrival_tick == row.trial.case.origin
    });
    let harness_gate = std::fs::read_to_string("results/pxc_harness_audit_v1/audit.json")
        .unwrap_or_default();
    let active_gate =
        std::fs::read_to_string("results/pxc_active_gate_v1/audit.json").unwrap_or_default();
    let execution_gate = match mode {
        Mode::Development => harness_gate.contains("\"gate_pass\": true"),
        Mode::Authority => std::fs::read_to_string("results/pxc_authority_firewall_v1/audit.json")
            .unwrap_or_default()
            .contains("\"gate_pass\": true"),
    };
    let published = Path::new(mode.csv()).is_file();
    [
        roots.len() == 16 && roots.iter().copied().eq(expected_start..=expected_start + 15),
        layouts,
        origins,
        timing,
        rows.iter().map(|row| row.trial.case.root).collect::<BTreeSet<_>>().len() == 16,
        harness_gate.contains("\"gate_pass\": true"),
        sha("../../../truelearner/crates/core/src/lib.rs").as_deref() == Some(RUNTIME_SHA),
        active_gate.contains("\"gate_pass\": true"),
        pxr0_exact,
        execution_gate,
        rows.len() == 16 && rows.iter().all(|row| row.clauses.len() == 32),
        published && rows.iter().all(|row| row.replay),
    ]
}

fn csv(rows: &[Row]) -> String {
    let mut text = String::from("root,reverse,reflect,origin,construction_tick,pressure_origin,first_arrival_tick,batches,paired_held,selective_held,retained,partial,adjacent_first,adjacent_second,duplicated,resisted,direct,duplicate_direct,open,fork,ring,aged,formation_updates,formation_modulation,age_deallocations,max_work,max_bytes,batch_outputs,batch_work,batch_bytes,all_quiet,outward_only,replay,clauses,passed\n");
    for row in rows {
        let trial = &row.trial;
        let fields = [
            trial.case.root.to_string(),
            trial.case.reverse.to_string(),
            trial.case.reflect.to_string(),
            trial.case.origin.to_string(),
            trial.construction_tick.to_string(),
            trial.pressure_origin.to_string(),
            trial.first_arrival_tick.to_string(),
            trial.batches.len().to_string(),
            trial.paired_held.to_string(),
            trial.selective_held.to_string(),
            trial.retained.to_string(),
            trial.partial.to_string(),
            trial.adjacent_first.to_string(),
            trial.adjacent_second.to_string(),
            trial.duplicated.to_string(),
            trial.resisted.to_string(),
            trial.direct.to_string(),
            trial.duplicate_direct.to_string(),
            trial.open.to_string(),
            trial.fork.to_string(),
            trial.ring.to_string(),
            trial.aged.to_string(),
            trial.formation_updates.to_string(),
            trial.formation_modulation.to_string(),
            trial.age_work.physical_deallocations.to_string(),
            trial.max_work.to_string(),
            trial.max_bytes.to_string(),
            trial
                .batches
                .iter()
                .map(|batch| batch.crossings.len().to_string())
                .collect::<Vec<_>>()
                .join("|"),
            trial
                .batches
                .iter()
                .map(|batch| batch.work.total().to_string())
                .collect::<Vec<_>>()
                .join("|"),
            trial
                .batches
                .iter()
                .map(|batch| batch.bytes.to_string())
                .collect::<Vec<_>>()
                .join("|"),
            trial.batches.iter().all(|batch| batch.quiet).to_string(),
            trial
                .batches
                .iter()
                .all(Batch::outward_only)
                .to_string(),
            row.replay.to_string(),
            row.clauses
                .iter()
                .map(bool::to_string)
                .collect::<Vec<_>>()
                .join("|"),
            row.passed.to_string(),
        ];
        text.push_str(&fields.join(","));
        text.push('\n');
    }
    text
}

fn markdown(mode: Mode, rows: &[Row], globals: [bool; 12]) -> String {
    let passed_rows = rows.iter().filter(|row| row.passed).count();
    let row_clauses = rows
        .iter()
        .map(|row| row.clauses.iter().filter(|value| **value).count())
        .sum::<usize>();
    let global_clauses = globals.iter().filter(|value| **value).count();
    let mut text = String::new();
    writeln!(text, "# {}\n", mode.title()).unwrap();
    writeln!(
        text,
        "Outcome: **{}**.\n",
        if passed_rows == 16 && global_clauses == 12 {
            match mode {
                Mode::Development => "DEVELOPMENT READY",
                Mode::Authority => "AUTHORITY ESTABLISHED",
            }
        } else {
            "NEGATIVE"
        }
    )
    .unwrap();
    writeln!(text, "- rows: `{passed_rows}/16`;").unwrap();
    writeln!(text, "- row clauses: `{row_clauses}/512`;").unwrap();
    writeln!(text, "- global clauses: `{global_clauses}/12`;").unwrap();
    writeln!(text, "- total clauses: `{}/524`;", row_clauses + global_clauses).unwrap();
    writeln!(
        text,
        "- maximum per-batch work: `{}` / `{WORK_BOUND}`;",
        rows.iter().map(|row| row.trial.max_work).max().unwrap_or(0)
    )
    .unwrap();
    writeln!(
        text,
        "- maximum resident bytes: `{}` / `{MEMORY_BOUND}`;",
        rows.iter().map(|row| row.trial.max_bytes).max().unwrap_or(0)
    )
    .unwrap();
    writeln!(
        text,
        "- natural quiescence: `{}`;",
        rows.iter().all(|row| row.trial.batches.iter().all(|batch| batch.quiet))
    )
    .unwrap();
    writeln!(
        text,
        "- outward-only boundary: `{}`;",
        rows.iter()
            .all(|row| row.trial.batches.iter().all(Batch::outward_only))
    )
    .unwrap();
    writeln!(
        text,
        "- exact replay: `{}`.\n",
        rows.iter().all(|row| row.replay)
    )
    .unwrap();
    writeln!(text, "## Unconditional rows\n").unwrap();
    for row in rows {
        let trial = &row.trial;
        writeln!(text, "- `{}`: `origin={} layout={}|{} paired={} selective={} retained={} partial={} adjacent={}|{} duplicate={} resisted={} direct={}|{} open={} fork={} ring={} aged={} updates={} modulation={} deallocations={} max_work={} max_bytes={} quiet={} outward_only={} replay={} clauses={} passed={}`", trial.case.root, trial.case.origin, trial.case.reverse, trial.case.reflect, trial.paired_held, trial.selective_held, trial.retained, trial.partial, trial.adjacent_first, trial.adjacent_second, trial.duplicated, trial.resisted, trial.direct, trial.duplicate_direct, trial.open, trial.fork, trial.ring, trial.aged, trial.formation_updates, trial.formation_modulation, trial.age_work.physical_deallocations, trial.max_work, trial.max_bytes, trial.batches.iter().all(|batch| batch.quiet), trial.batches.iter().all(Batch::outward_only), row.replay, row.clauses.iter().map(bool::to_string).collect::<Vec<_>>().join("|"), row.passed).unwrap();
    }
    writeln!(text, "\n## Global clauses\n").unwrap();
    writeln!(
        text,
        "`{}`",
        globals
            .iter()
            .map(bool::to_string)
            .collect::<Vec<_>>()
            .join("|")
    )
    .unwrap();
    text
}

fn sha(path: &str) -> Option<String> {
    let output = Command::new("sha256sum").arg(path).output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()?
        .split_whitespace()
        .next()
        .map(str::to_owned)
}

fn publish(destination: &str, content: &str) {
    let stage = format!("{destination}.staging");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&stage)
        .unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.sync_all().unwrap();
    rename(stage, destination).unwrap();
}
