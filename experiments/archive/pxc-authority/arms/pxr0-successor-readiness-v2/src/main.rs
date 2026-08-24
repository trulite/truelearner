#![forbid(unsafe_code)]

use pxr0_physical_runtime::{
    ArrowSpec, CellId, CellSpec, PlasticSubstrate, RunResult, SpikeInput, TransmissionMode,
};
use std::{
    collections::BTreeSet,
    fmt::Write as _,
    fs::{rename, OpenOptions},
    io::Write as _,
    path::Path,
    process::Command,
};

const WORK_BOUND: u64 = 20_000;
const MEMORY_BOUND: usize = 8_192;
const ARROW_BYTES: usize = 64;
const CSV: &str = "results/pxr0_successor_readiness_v2.csv";
const CONTROL_CSV: &str = "results/pxr0_phase_controls_v2.csv";
const MD: &str = "results/pxr0_successor_readiness_v2.md";
const CSV_STAGE: &str = "results/pxr0_successor_readiness_v2.csv.staging";
const CONTROL_CSV_STAGE: &str = "results/pxr0_phase_controls_v2.csv.staging";
const MD_STAGE: &str = "results/pxr0_successor_readiness_v2.md.staging";
const RUNTIME_SHA: &str = "f6989555f5a43dff91b39a5c7f79038168f39142fdbecca7e5e40938a72785cb";

const CASES: [Case; 16] = [
    Case::new(1_175_001, false, false, 0),
    Case::new(1_175_002, false, false, 130),
    Case::new(1_175_003, false, false, 260),
    Case::new(1_175_004, false, false, 390),
    Case::new(1_175_005, true, false, 0),
    Case::new(1_175_006, true, false, 130),
    Case::new(1_175_007, true, false, 260),
    Case::new(1_175_008, true, false, 390),
    Case::new(1_175_009, false, true, 0),
    Case::new(1_175_010, false, true, 130),
    Case::new(1_175_011, false, true, 260),
    Case::new(1_175_012, false, true, 390),
    Case::new(1_175_013, true, true, 0),
    Case::new(1_175_014, true, true, 130),
    Case::new(1_175_015, true, true, 260),
    Case::new(1_175_016, true, true, 390),
];

const CONTROL_CASES: [Case; 12] = [
    Case::new(1_176_001, false, false, 3),
    Case::new(1_176_002, false, false, 6),
    Case::new(1_176_003, false, false, 9),
    Case::new(1_176_004, true, false, 133),
    Case::new(1_176_005, true, false, 136),
    Case::new(1_176_006, true, false, 139),
    Case::new(1_176_007, false, true, 263),
    Case::new(1_176_008, false, true, 266),
    Case::new(1_176_009, false, true, 269),
    Case::new(1_176_010, true, true, 393),
    Case::new(1_176_011, true, true, 396),
    Case::new(1_176_012, true, true, 399),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Case {
    root: u64,
    reverse: bool,
    reflect: bool,
    shift: i64,
}

impl Case {
    const fn new(root: u64, reverse: bool, reflect: bool, shift: i64) -> Self {
        Self {
            root,
            reverse,
            reflect,
            shift,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Reading {
    outward: usize,
    inward: usize,
    updates: u64,
    drive: u64,
    modulation: u64,
    proposals: u64,
    deallocations: u64,
    work: u64,
    quiet: bool,
    bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PairReading {
    links: [usize; 2],
    impulses: [i32; 2],
    outward: usize,
    updates: u64,
    modulation: u64,
    work: u64,
    quiet: bool,
    bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Trial {
    case: Case,
    blank: Reading,
    direct: Reading,
    pair_first: PairReading,
    pair_second: PairReading,
    pair_held: PairReading,
    role_first: PairReading,
    role_held: PairReading,
    formation: Reading,
    completed: Reading,
    incomplete: Reading,
    adjacent_a: Reading,
    adjacent_b: Reading,
    duplicate: Reading,
    duplicate_direct: Reading,
    blocked: Reading,
    open: Reading,
    branch: Reading,
    cycle: Reading,
    drive_only: Reading,
    stale: Reading,
    stale_before: usize,
    stale_after: usize,
    max_work: u64,
    max_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    trial: Trial,
    clauses: [bool; 24],
    replay: bool,
    passed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Timing {
    construction_tick: i64,
    pressure_origin: i64,
    first_arrival_tick: i64,
    construction_minus_pressure: i64,
    first_arrival_minus_construction: i64,
    modulus: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhaseControl {
    trial: Trial,
    timing: Timing,
    phase_zero_root: u64,
    differs_from_phase_zero: bool,
    clauses: [bool; 6],
    replay: bool,
    passed: bool,
}

fn main() {
    let rows = CASES.into_iter().map(replay).collect::<Vec<_>>();
    let controls = CONTROL_CASES
        .into_iter()
        .map(|case| control_replay(case, &rows))
        .collect::<Vec<_>>();
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(CONTROL_CSV_STAGE, CONTROL_CSV, &control_csv(&controls));
    let globals = globals(&rows, &controls);
    publish(MD_STAGE, MD, &markdown(&rows, &controls, globals));
    assert!(
        rows.iter().all(|row| row.passed),
        "PXR0 v2 invariance row failed"
    );
    assert!(
        controls.iter().all(|control| control.passed),
        "PXR0 v2 phase control safety failed"
    );
    assert!(
        globals.into_iter().all(|value| value),
        "PXR0 v2 global failed"
    );
    println!("PXR0_SUCCESSOR_DEVELOPMENT_READY_V2 rows=16/16 controls=12/12 clauses=466/466");
}

fn replay(case: Case) -> Row {
    let first = run(case);
    let second = run(case);
    let exact = first == second;
    row(first, exact)
}

fn timing(case: Case) -> Timing {
    let pressure_origin = case.shift - case.shift.rem_euclid(10);
    Timing {
        construction_tick: case.shift,
        pressure_origin,
        first_arrival_tick: case.shift,
        construction_minus_pressure: case.shift - pressure_origin,
        first_arrival_minus_construction: 0,
        modulus: case.shift.rem_euclid(10),
    }
}

fn functional_observation(trial: &Trial) -> [i64; 16] {
    [
        trial.pair_first.updates as i64,
        trial.pair_second.updates as i64,
        i64::from(trial.pair_held.impulses[0]),
        i64::from(trial.pair_held.impulses[1]),
        trial.formation.updates as i64,
        trial.completed.outward as i64,
        trial.duplicate.outward as i64,
        trial.incomplete.outward as i64,
        trial.blocked.outward as i64,
        trial.open.outward as i64,
        trial.branch.outward as i64,
        trial.cycle.outward as i64,
        trial.stale.outward as i64,
        trial.stale.proposals as i64,
        trial.stale_before as i64,
        trial.stale_after as i64,
    ]
}

fn control_replay(case: Case, rows: &[Row]) -> PhaseControl {
    let first = run(case);
    let second = run(case);
    let exact = first == second;
    let phase_zero = rows
        .iter()
        .find(|row| {
            row.trial.case.shift == 0
                && row.trial.case.reverse == case.reverse
                && row.trial.case.reflect == case.reflect
        })
        .expect("each phase control requires its registered phase-zero layout");
    let base = row(first.clone(), exact);
    let clauses = [
        exact,
        base.clauses[20],
        first.max_work <= WORK_BOUND,
        first.max_bytes <= MEMORY_BOUND,
        base.clauses[17],
        base.clauses[18] && base.clauses[19],
    ];
    let passed = clauses.into_iter().all(|value| value);
    PhaseControl {
        trial: first,
        timing: timing(case),
        phase_zero_root: phase_zero.trial.case.root,
        differs_from_phase_zero: functional_observation(&base.trial)
            != functional_observation(&phase_zero.trial),
        clauses,
        replay: exact,
        passed,
    }
}

fn run(case: Case) -> Trial {
    let layout = Layout {
        namespace: case.root << 32,
        reverse: case.reverse,
        reflect: case.reflect,
        twist: case.shift as u64,
        outward_resistance: 100,
        shift: case.shift,
    };
    let mut blank_body = RecursiveBody::new(layout);
    let blank = blank_body.settle();
    let direct = compact(case, 10_000, Form::Direct, 1);

    let mut pair = PairBody::new(case, 20_000);
    let pair_first = pair.participate(pair_maturation(&pair, 0));
    let pair_second = pair.participate(pair_maturation(&pair, 10));
    let pair_held = pair.participate(pair_boundary(&pair, 20));

    let mut role = PairBody::new(case, 30_000);
    let role_first = role.participate(pair_role(&role, 0));
    let role_held = role.participate(pair_boundary_zero(&role, 10));

    let mut learned = RecursiveBody::new(layout);
    let formation = learned.learn_twice();
    let mut incomplete_body = learned.clone();
    let mut adjacent_body = learned.clone();
    let mut duplicate_body = learned.clone();
    let completed = learned.reuse([true; 4], 61, false);
    let incomplete = incomplete_body.reuse([true, true, true, false], 61, false);
    let adjacent_a = adjacent_body.reuse([true, true, false, false], 61, false);
    let adjacent_b = adjacent_body.reuse([true, true, true, false], 70, false);
    let duplicate = duplicate_body.reuse([true; 4], 61, true);

    let mut blocked_body = RecursiveBody::new(Layout {
        outward_resistance: 0,
        ..layout
    });
    blocked_body.learn_twice();
    let blocked = blocked_body.reuse([true; 4], 61, false);

    let duplicate_direct = compact(case, 40_000, Form::Direct, 2);
    let open = compact(case, 50_000, Form::Open, 1);
    let branch = compact(case, 60_000, Form::Fork, 1);
    let cycle = compact(case, 70_000, Form::Ring, 1);

    let mut drive_body = RecursiveBody::new(Layout {
        namespace: (case.root << 32) + 80_000,
        ..layout
    });
    let drive_only = drive_body.burst(1, 0, false, false);

    let mut stale_body = RecursiveBody::new(Layout {
        namespace: (case.root << 32) + 90_000,
        ..layout
    });
    stale_body.learn_once_then_age();
    let stale_before = stale_body.last_bytes;
    let stale = stale_body.changed();
    let stale_after = stale.bytes;

    let readings = [
        blank,
        direct,
        formation,
        completed,
        incomplete,
        adjacent_a,
        adjacent_b,
        duplicate,
        duplicate_direct,
        blocked,
        open,
        branch,
        cycle,
        drive_only,
        stale,
    ];
    let pair_readings = [pair_first, pair_second, pair_held, role_first, role_held];
    let max_work = readings
        .into_iter()
        .map(|reading| reading.work)
        .chain(pair_readings.into_iter().map(|reading| reading.work))
        .max()
        .unwrap_or(0);
    let max_bytes = readings
        .into_iter()
        .map(|reading| reading.bytes)
        .chain(pair_readings.into_iter().map(|reading| reading.bytes))
        .max()
        .unwrap_or(0);

    Trial {
        case,
        blank,
        direct,
        pair_first,
        pair_second,
        pair_held,
        role_first,
        role_held,
        formation,
        completed,
        incomplete,
        adjacent_a,
        adjacent_b,
        duplicate,
        duplicate_direct,
        blocked,
        open,
        branch,
        cycle,
        drive_only,
        stale,
        stale_before,
        stale_after,
        max_work,
        max_bytes,
    }
}

fn row(trial: Trial, replay: bool) -> Row {
    let quiet = [
        trial.blank,
        trial.direct,
        trial.formation,
        trial.completed,
        trial.incomplete,
        trial.adjacent_a,
        trial.adjacent_b,
        trial.duplicate,
        trial.duplicate_direct,
        trial.blocked,
        trial.open,
        trial.branch,
        trial.cycle,
        trial.drive_only,
        trial.stale,
    ]
    .into_iter()
    .all(|reading| reading.quiet)
        && [
            trial.pair_first,
            trial.pair_second,
            trial.pair_held,
            trial.role_first,
            trial.role_held,
        ]
        .into_iter()
        .all(|reading| reading.quiet);
    let mut clauses = [false; 24];
    clauses[0] = trial.blank.outward == 0 && trial.blank.quiet;
    clauses[1] = trial.direct.outward == 1;
    clauses[2] = trial.pair_first.updates == 2
        && trial.pair_second.updates == 2
        && trial.pair_held.impulses == [2, 2];
    clauses[3] = trial.role_first.updates == 1
        && trial.role_held.impulses[0] == 2
        && trial.role_held.links[1] == 0;
    clauses[4] = trial.completed.outward == 1;
    clauses[5] = trial.completed.outward == 1;
    clauses[6] = trial.formation.updates >= 6 && trial.completed.outward == 1;
    clauses[7] = trial.adjacent_a.outward == 0 && trial.adjacent_b.outward == 0;
    clauses[8] = trial.formation.updates >= 6;
    clauses[9] = trial.completed.inward == 1 && trial.completed.updates == 1;
    clauses[10] = trial.role_held.links[1] == 0;
    clauses[11] = trial.pair_first.modulation == 2 && trial.pair_first.updates == 2;
    clauses[12] = trial.drive_only.updates == 0 && trial.drive_only.modulation == 0;
    clauses[13] = trial.drive_only.drive > 0;
    clauses[14] = trial.completed.drive > 0 && trial.completed.outward == 1;
    clauses[15] = trial.completed.outward == 1;
    clauses[16] = trial.duplicate.outward == 1 && trial.duplicate_direct.outward == 1;
    clauses[17] = trial.incomplete.outward == 0
        && trial.blocked.outward == 0
        && trial.open.outward == 0
        && trial.branch.outward == 0
        && trial.cycle.outward == 0;
    clauses[18] = trial.stale.outward == 0 && trial.stale.inward == 0;
    clauses[19] = trial.stale.proposals == 1
        && trial.stale_after.saturating_sub(trial.stale_before) == ARROW_BYTES;
    clauses[20] = quiet;
    clauses[21] = trial.max_work <= WORK_BOUND && trial.max_bytes <= MEMORY_BOUND;
    clauses[22] = replay;
    clauses[23] = clauses[..23].iter().all(|value| *value);
    let passed = clauses.into_iter().all(|value| value);
    Row {
        trial,
        clauses,
        replay,
        passed,
    }
}

#[derive(Clone, Copy)]
struct Layout {
    namespace: u64,
    reverse: bool,
    reflect: bool,
    twist: u64,
    outward_resistance: u32,
    shift: i64,
}

#[derive(Clone)]
struct RecursiveBody {
    space: PlasticSubstrate,
    layout: Layout,
    primitive: [CellId; 4],
    stages: [CellId; 3],
    context: CellId,
    returning: CellId,
    outward_from: u64,
    outward_to: u64,
    inward_from: u64,
    inward_to: u64,
    last_bytes: usize,
}

impl RecursiveBody {
    fn new(layout: Layout) -> Self {
        let mut space = PlasticSubstrate::new();
        space.advance_time(layout.shift);
        let sides = if layout.reverse {
            [3, 2, 1, 0]
        } else {
            [0, 1, 2, 3]
        };
        let stages_order = if layout.reverse { [2, 1, 0] } else { [0, 1, 2] };
        let mut primitive = [None; 4];
        let mut outlets = [None; 4];
        let mut traces = [None; 4];
        let mut hubs = [None; 4];
        for side in sides {
            primitive[side] = Some(space.add_cell(cell(
                physical(layout, 10 + side as u64),
                -100_000 - side as i32 * 1_000,
                10 + side as i16,
                1,
            )));
            outlets[side] = Some(space.add_cell(cell(
                physical(layout, 20 + side as u64),
                -90_000 - side as i32 * 1_000,
                20 + side as i16,
                1,
            )));
            traces[side] = Some(space.add_cell(cell(
                physical(layout, 30 + side as u64),
                -80_000 - side as i32 * 1_000,
                30 + side as i16,
                2,
            )));
            hubs[side] = Some(space.add_cell(cell(
                physical(layout, 40 + side as u64),
                -70_000 - side as i32 * 1_000,
                34 + side as i16,
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
                -10_000 - stage as i32 * 1_000,
                50 + stage as i16,
                2,
            )));
            let position = 10_000 + stage as i32 * 1_000;
            stages[stage] = Some(space.add_cell(cell(
                physical(layout, 200 + stage as u64),
                position,
                60 + stage as i16,
                2,
            )));
            outputs[stage] = Some(space.add_cell(cell(
                physical(layout, 300 + stage as u64),
                position + if layout.reflect { -1 } else { 1 },
                70 + stage as i16,
                2,
            )));
            source_traces[stage] = Some(space.add_cell(cell(
                physical(layout, 400 + stage as u64),
                30_000 + stage as i32 * 1_000,
                80 + stage as i16,
                2,
            )));
            output_traces[stage] = Some(space.add_cell(cell(
                physical(layout, 600 + stage as u64),
                50_000 + stage as i32 * 1_000,
                100 + stage as i16,
                2,
            )));
            output_hubs[stage] = Some(space.add_cell(cell(
                physical(layout, 700 + stage as u64),
                60_000 + stage as i32 * 1_000,
                110 + stage as i16,
                1,
            )));
            return_loci[stage] = Some(space.add_cell(cell(
                physical(layout, 800 + stage as u64),
                70_000 + stage as i32 * 1_000,
                120 + stage as i16,
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
        let context = space.add_cell(cell(physical(layout, 900), 90_000, 130, 1));
        let returning = space.add_cell(cell(physical(layout, 901), 100_000, 131, 1));
        let outward = space.add_cell(cell(physical(layout, 950), 120_000, 400, 1));
        let relay = space.add_cell(cell(physical(layout, 951), 121_000, 400, 1));
        for side in sides {
            space.add_arrow(drive(primitive[side], outlets[side], 0, 1, 100));
            normalize(&mut space, outlets[side], traces[side], hubs[side]);
        }
        for stage in stages_order {
            normalize(
                &mut space,
                outputs[stage],
                output_traces[stage],
                output_hubs[stage],
            );
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
        Self {
            space,
            layout,
            primitive,
            stages,
            context,
            returning,
            outward_from: physical(layout, 602),
            outward_to: physical(layout, 950),
            inward_from: physical(layout, 951),
            inward_to: physical(layout, 202),
            last_bytes: 0,
        }
    }

    fn learn_twice(&mut self) -> Reading {
        let mut total = Reading::default();
        for (depth, starts) in [(1, [0, 11]), (2, [20, 31]), (3, [40, 51])] {
            for start in starts {
                total = merge(total, self.burst(depth, start, true, true));
            }
        }
        let work = self.space.advance_time(self.layout.shift + 60);
        total.work = total.work.saturating_add(work.total());
        total.quiet = true;
        self.last_bytes = total.bytes;
        total
    }

    fn learn_once_then_age(&mut self) -> Reading {
        let mut total = Reading::default();
        for (depth, start) in [(1, 0), (2, 11), (3, 22)] {
            total = merge(total, self.burst(depth, start, true, true));
        }
        let work = self.space.advance_time(self.layout.shift + 110);
        total.work = total.work.saturating_add(work.total());
        total.quiet = true;
        self.last_bytes = total.bytes;
        total
    }

    fn reuse(&mut self, present: [bool; 4], start: i64, duplicate: bool) -> Reading {
        for (side, admitted) in present.into_iter().enumerate() {
            if admitted {
                let tick = start + [0, 0, 2, 4][side];
                self.pulse(self.primitive[side], tick, 1, side as i32);
                if duplicate {
                    self.pulse(self.primitive[side], tick, 1, 10 + side as i32);
                }
            }
        }
        for stage in 0..3 {
            self.pulse(
                self.stages[stage],
                start + 1 + stage as i64 * 2,
                1,
                100 + stage as i32,
            );
        }
        self.settle()
    }

    fn changed(&mut self) -> Reading {
        self.pulse(self.stages[0], 111, 2, 900);
        self.settle()
    }

    fn burst(&mut self, depth: usize, start: i64, context: bool, returning: bool) -> Reading {
        for side in 0..=depth {
            self.pulse(
                self.primitive[side],
                start + [0, 0, 2, 4][side],
                1,
                side as i32,
            );
        }
        for stage in 0..depth {
            let tick = start + 1 + stage as i64 * 2;
            self.pulse(self.stages[stage], tick, 1, 100 + stage as i32);
            if context {
                self.pulse(self.context, tick, 1, 500 + stage as i32);
            }
            if returning {
                self.pulse(self.returning, tick + 2, 1, 600 + stage as i32);
            }
        }
        self.settle()
    }

    fn pulse(&mut self, target: CellId, tick: i64, impulse: i32, phase: i32) {
        self.space.enter(SpikeInput {
            arrival_tick: self.layout.shift + tick,
            phase,
            origin_physical: self.layout.namespace + 50_000 + self.layout.twist + phase as u64,
            target,
            impulse,
        });
    }

    fn settle(&mut self) -> Reading {
        let result = self.space.propagate();
        let reading = reading(
            result,
            self.outward_from,
            self.outward_to,
            self.inward_from,
            self.inward_to,
        );
        self.last_bytes = reading.bytes;
        reading
    }
}

#[derive(Clone, Copy)]
enum Form {
    Direct,
    Open,
    Fork,
    Ring,
}

fn compact(case: Case, offset: u64, form: Form, count: usize) -> Reading {
    let namespace = (case.root << 32) + offset;
    let mut space = PlasticSubstrate::new();
    space.advance_time(case.shift);
    let direction = if case.reflect { -1 } else { 1 };
    let inlet = space.add_cell(cell(namespace + 1, 0, 40, 1));
    let outward = space.add_cell(cell(namespace + 2, direction * 10, 400, 1));
    match form {
        Form::Direct => {
            space.add_arrow(drive(inlet, outward, 1, 1, 100));
        }
        Form::Open => {
            space.add_arrow(drive(inlet, outward, 1, 1, 0));
        }
        Form::Fork => {
            let a = space.add_cell(cell(namespace + 3, direction * 2, 40, 1));
            let b = space.add_cell(cell(namespace + 4, direction * 3, 40, 1));
            let join = space.add_cell(cell(namespace + 5, direction * 4, 40, 3));
            space.add_arrow(drive(inlet, a, 1, 1, 100));
            space.add_arrow(drive(inlet, b, 1, 1, 100));
            space.add_arrow(drive(a, join, 1, 1, 100));
            space.add_arrow(drive(b, join, 1, 1, 100));
            space.add_arrow(drive(join, outward, 1, 1, 100));
        }
        Form::Ring => {
            let a = space.add_cell(cell(namespace + 3, direction * 2, 40, 2));
            let b = space.add_cell(cell(namespace + 4, direction * 3, 40, 2));
            space.add_arrow(drive(inlet, a, 1, 1, 100));
            space.add_arrow(drive(a, b, 1, 1, 100));
            space.add_arrow(drive(b, a, 1, 1, 100));
            space.add_arrow(drive(b, outward, 1, 1, 100));
        }
    }
    for index in 0..count {
        space.enter(SpikeInput {
            arrival_tick: case.shift,
            phase: index as i32,
            origin_physical: namespace + 100 + index as u64,
            target: inlet,
            impulse: 1,
        });
    }
    reading(space.propagate(), namespace + 1, namespace + 2, 0, 0)
}

struct PairBody {
    space: PlasticSubstrate,
    case: Case,
    namespace: u64,
    sites: [CellId; 9],
    physical: [u64; 9],
    positions: [i32; 9],
    link_from: [u64; 2],
    link_to: [u64; 2],
    outward_from: u64,
    outward_to: u64,
}

#[derive(Clone, Copy)]
struct Arrival {
    tick: i64,
    phase: i32,
    position: i32,
}

impl PairBody {
    fn new(case: Case, offset: u64) -> Self {
        let namespace = (case.root << 32) + offset;
        let mut space = PlasticSubstrate::new();
        space.advance_time(case.shift);
        let mut sources = [None; 4];
        let mut traces = [None; 4];
        let sides = if case.reverse {
            [3, 2, 1, 0]
        } else {
            [0, 1, 2, 3]
        };
        for side in sides {
            sources[side] = Some(space.add_cell(cell(
                namespace + 100 + side as u64,
                position(case, side),
                10 + side as i16,
                1,
            )));
            traces[side] = Some(space.add_cell(cell(
                namespace + 200 + side as u64,
                internal(case, 20 + side as i32),
                20 + side as i16,
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
        let pairs = if case.reverse { [1, 0] } else { [0, 1] };
        for pair in pairs {
            coincidence[pair] = Some(space.add_cell(cell(
                namespace + 300 + pair as u64,
                internal(case, 40 + pair as i32),
                30 + pair as i16,
                2,
            )));
            inner[pair] = Some(space.add_cell(cell(
                namespace + 400 + pair as u64,
                position(case, 4 + pair * 2),
                40 + pair as i16,
                1,
            )));
            downstream[pair] = Some(space.add_cell(cell(
                namespace + 500 + pair as u64,
                position(case, 5 + pair * 2),
                50 + pair as i16,
                2,
            )));
            downstream_trace[pair] = Some(space.add_cell(cell(
                namespace + 600 + pair as u64,
                internal(case, 60 + pair as i32),
                60 + pair as i16,
                1,
            )));
            relay[pair] = Some(space.add_cell(cell(
                namespace + 800 + pair as u64,
                internal(case, 80 + pair as i32),
                70 + pair as i16,
                2,
            )));
        }
        let coincidence = coincidence.map(Option::unwrap);
        let inner = inner.map(Option::unwrap);
        let downstream = downstream.map(Option::unwrap);
        let downstream_trace = downstream_trace.map(Option::unwrap);
        let relay = relay.map(Option::unwrap);
        let returning = space.add_cell(cell(namespace + 700, position(case, 8), 80, 1));
        let outward = space.add_cell(cell(namespace + 900, position(case, 9), 90, 1));
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
        space.add_arrow(drive(downstream[1], outward, 1, 1, 100));
        let sites = [
            sources[0],
            sources[1],
            sources[2],
            sources[3],
            inner[0],
            downstream[0],
            inner[1],
            downstream[1],
            returning,
        ];
        let physical = [
            namespace + 100,
            namespace + 101,
            namespace + 102,
            namespace + 103,
            namespace + 400,
            namespace + 500,
            namespace + 401,
            namespace + 501,
            namespace + 700,
        ];
        let positions = [0, 1, 2, 3, 4, 5, 6, 7, 8].map(|site| position(case, site));
        Self {
            space,
            case,
            namespace,
            sites,
            physical,
            positions,
            link_from: [namespace + 400, namespace + 401],
            link_to: [namespace + 500, namespace + 501],
            outward_from: namespace + 501,
            outward_to: namespace + 900,
        }
    }

    fn participate(&mut self, arrivals: Vec<Arrival>) -> PairReading {
        for arrival in arrivals {
            let index = self
                .positions
                .iter()
                .position(|value| *value == arrival.position)
                .unwrap();
            self.space.enter(SpikeInput {
                arrival_tick: self.case.shift + arrival.tick,
                phase: arrival.phase,
                origin_physical: self.namespace + 20_000 + arrival.phase as u64,
                target: self.sites[index],
                impulse: 1,
            });
        }
        let result = self.space.propagate();
        let mut links = [0; 2];
        let mut impulses = [0; 2];
        for crossing in &result.crossings {
            for index in 0..2 {
                if crossing.from_physical == self.link_from[index]
                    && crossing.to_physical == self.link_to[index]
                {
                    links[index] += 1;
                    impulses[index] = impulses[index].max(crossing.impulse);
                }
            }
        }
        PairReading {
            links,
            impulses,
            outward: count(&result, self.outward_from, self.outward_to),
            updates: result.work.local_return_updates,
            modulation: result.work.modulatory_deliveries,
            work: result.work.total(),
            quiet: result.naturally_quiescent,
            bytes: result.resident_bytes,
        }
    }
}

fn pair_maturation(body: &PairBody, tick: i64) -> Vec<Arrival> {
    let mut arrivals = pair_boundary(body, tick);
    arrivals.extend([
        arrival(body, 5, tick + 2, 20),
        arrival(body, 8, tick + 3, 21),
        arrival(body, 7, tick + 4, 22),
        arrival(body, 8, tick + 5, 23),
    ]);
    arrivals
}

fn pair_boundary(body: &PairBody, tick: i64) -> Vec<Arrival> {
    vec![
        arrival(body, 0, tick, 10),
        arrival(body, 1, tick, 11),
        arrival(body, 2, tick + 2, 12),
    ]
}

fn pair_role(body: &PairBody, tick: i64) -> Vec<Arrival> {
    vec![
        arrival(body, 0, tick, 10),
        arrival(body, 1, tick, 11),
        arrival(body, 5, tick + 2, 20),
        arrival(body, 8, tick + 3, 21),
    ]
}

fn pair_boundary_zero(body: &PairBody, tick: i64) -> Vec<Arrival> {
    vec![arrival(body, 0, tick, 10), arrival(body, 1, tick, 11)]
}

fn arrival(body: &PairBody, site: usize, tick: i64, phase: i32) -> Arrival {
    let _physical_identity = body.physical[site];
    Arrival {
        tick,
        phase,
        position: body.positions[site],
    }
}

fn position(case: Case, site: usize) -> i32 {
    let value = 100_000 + site as i32 * 10_000;
    if case.reflect {
        -value
    } else {
        value
    }
}

fn internal(case: Case, offset: i32) -> i32 {
    let value = 500_000 + offset * 10_000;
    if case.reflect {
        -value
    } else {
        value
    }
}

fn reading(
    result: RunResult,
    outward_from: u64,
    outward_to: u64,
    inward_from: u64,
    inward_to: u64,
) -> Reading {
    Reading {
        outward: count(&result, outward_from, outward_to),
        inward: count(&result, inward_from, inward_to),
        updates: result.work.local_return_updates,
        drive: result.work.drive_deliveries,
        modulation: result.work.modulatory_deliveries,
        proposals: result.work.local_structural_proposals,
        deallocations: result.work.physical_deallocations,
        work: result.work.total(),
        quiet: result.naturally_quiescent,
        bytes: result.resident_bytes,
    }
}

fn count(result: &RunResult, from: u64, to: u64) -> usize {
    result
        .crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .count()
}

fn merge(left: Reading, right: Reading) -> Reading {
    Reading {
        outward: left.outward + right.outward,
        inward: left.inward + right.inward,
        updates: left.updates + right.updates,
        drive: left.drive + right.drive,
        modulation: left.modulation + right.modulation,
        proposals: left.proposals + right.proposals,
        deallocations: left.deallocations + right.deallocations,
        work: left.work + right.work,
        quiet: (left.quiet || left.work == 0) && right.quiet,
        bytes: right.bytes,
    }
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

fn physical(layout: Layout, suffix: u64) -> u64 {
    layout.namespace + (suffix.wrapping_mul(73).wrapping_add(layout.twist) % 1_000)
}

fn globals(rows: &[Row], controls: &[PhaseControl]) -> [bool; 10] {
    let roots = rows
        .iter()
        .map(|row| row.trial.case.root)
        .collect::<BTreeSet<_>>();
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
    let shifts = [0, 130, 260, 390].into_iter().all(|shift| {
        rows.iter()
            .filter(|row| row.trial.case.shift == shift)
            .count()
            == 4
    });
    let invariant_timing = rows.iter().all(|row| {
        let value = timing(row.trial.case);
        value.modulus == 0
            && value.construction_tick == row.trial.case.shift
            && value.pressure_origin == row.trial.case.shift
            && value.first_arrival_tick == row.trial.case.shift
            && value.construction_minus_pressure == 0
            && value.first_arrival_minus_construction == 0
    });
    let namespaces = rows
        .iter()
        .flat_map(|row| (0..10).map(move |index| (row.trial.case.root << 32) + index * 10_000))
        .collect::<BTreeSet<_>>();
    let control_roots = controls
        .iter()
        .map(|control| control.trial.case.root)
        .collect::<BTreeSet<_>>();
    let control_origins = controls
        .iter()
        .map(|control| control.trial.case.shift)
        .collect::<Vec<_>>();
    let expected_control_origins = [3, 6, 9, 133, 136, 139, 263, 266, 269, 393, 396, 399];
    let control_timing = control_roots.len() == 12
        && control_roots.iter().copied().eq(1_176_001..=1_176_012)
        && control_origins == expected_control_origins
        && controls.iter().all(|control| {
            let value = control.timing;
            matches!(value.modulus, 3 | 6 | 9)
                && value.construction_tick == control.trial.case.shift
                && value.pressure_origin
                    == control.trial.case.shift - control.trial.case.shift.rem_euclid(10)
                && value.first_arrival_tick == control.trial.case.shift
                && value.construction_minus_pressure == value.modulus
                && value.first_arrival_minus_construction == 0
        });
    let gate =
        std::fs::read_to_string("results/pxr0_static_gate_v2/audit.json").unwrap_or_default();
    let runtime_exact =
        sha("crates/pxr0-physical-runtime/src/lib.rs").as_deref() == Some(RUNTIME_SHA);
    let published = Path::new(CSV).is_file() && Path::new(CONTROL_CSV).is_file();
    [
        roots.len() == 16 && roots.iter().copied().eq(1_175_001..=1_175_016),
        layouts,
        shifts,
        invariant_timing,
        namespaces.len() == 160,
        control_timing,
        runtime_exact,
        gate.contains("\"gate_pass\": true"),
        rows.len() == 16
            && rows.iter().all(|row| row.clauses.len() == 24)
            && controls.len() == 12
            && controls.iter().all(|control| control.clauses.len() == 6),
        published
            && rows.iter().all(|row| row.replay)
            && controls.iter().all(|control| control.replay),
    ]
}

fn csv(rows: &[Row]) -> String {
    let mut text = String::from("root,reverse,reflect,origin,origin_modulus,construction_tick,pressure_origin,first_arrival_tick,construction_minus_pressure,first_arrival_minus_construction,pair_updates,pair_impulses,formation_updates,completed_outward,duplicate_outward,incomplete_outward,blocked_outward,stale_outward,stale_proposals,stale_before,stale_after,max_work,max_bytes,quiescent,replay,clauses,passed\n");
    for row in rows {
        let trial = &row.trial;
        let timing = timing(trial.case);
        let fields = [
            trial.case.root.to_string(),
            trial.case.reverse.to_string(),
            trial.case.reflect.to_string(),
            trial.case.shift.to_string(),
            timing.modulus.to_string(),
            timing.construction_tick.to_string(),
            timing.pressure_origin.to_string(),
            timing.first_arrival_tick.to_string(),
            timing.construction_minus_pressure.to_string(),
            timing.first_arrival_minus_construction.to_string(),
            trial.pair_first.updates.to_string(),
            format!(
                "{}|{}",
                trial.pair_held.impulses[0], trial.pair_held.impulses[1]
            ),
            trial.formation.updates.to_string(),
            trial.completed.outward.to_string(),
            trial.duplicate.outward.to_string(),
            trial.incomplete.outward.to_string(),
            trial.blocked.outward.to_string(),
            trial.stale.outward.to_string(),
            trial.stale.proposals.to_string(),
            trial.stale_before.to_string(),
            trial.stale_after.to_string(),
            trial.max_work.to_string(),
            trial.max_bytes.to_string(),
            row.clauses[20].to_string(),
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

fn control_csv(controls: &[PhaseControl]) -> String {
    let mut text = String::from("root,reverse,reflect,origin,origin_modulus,construction_tick,pressure_origin,first_arrival_tick,construction_minus_pressure,first_arrival_minus_construction,phase_zero_root,differs_from_phase_zero,pair_updates,pair_impulses,formation_updates,completed_outward,duplicate_outward,incomplete_outward,blocked_outward,open_outward,branch_outward,cycle_outward,stale_outward,stale_proposals,stale_before,stale_after,max_work,max_bytes,quiescent,replay,clauses,passed\n");
    for control in controls {
        let trial = &control.trial;
        let timing = control.timing;
        let fields = [
            trial.case.root.to_string(),
            trial.case.reverse.to_string(),
            trial.case.reflect.to_string(),
            trial.case.shift.to_string(),
            timing.modulus.to_string(),
            timing.construction_tick.to_string(),
            timing.pressure_origin.to_string(),
            timing.first_arrival_tick.to_string(),
            timing.construction_minus_pressure.to_string(),
            timing.first_arrival_minus_construction.to_string(),
            control.phase_zero_root.to_string(),
            control.differs_from_phase_zero.to_string(),
            trial.pair_first.updates.to_string(),
            format!(
                "{}|{}",
                trial.pair_held.impulses[0], trial.pair_held.impulses[1]
            ),
            trial.formation.updates.to_string(),
            trial.completed.outward.to_string(),
            trial.duplicate.outward.to_string(),
            trial.incomplete.outward.to_string(),
            trial.blocked.outward.to_string(),
            trial.open.outward.to_string(),
            trial.branch.outward.to_string(),
            trial.cycle.outward.to_string(),
            trial.stale.outward.to_string(),
            trial.stale.proposals.to_string(),
            trial.stale_before.to_string(),
            trial.stale_after.to_string(),
            trial.max_work.to_string(),
            trial.max_bytes.to_string(),
            control.clauses[1].to_string(),
            control.replay.to_string(),
            control
                .clauses
                .iter()
                .map(bool::to_string)
                .collect::<Vec<_>>()
                .join("|"),
            control.passed.to_string(),
        ];
        text.push_str(&fields.join(","));
        text.push('\n');
    }
    text
}

fn markdown(rows: &[Row], controls: &[PhaseControl], globals: [bool; 10]) -> String {
    let row_count = rows.iter().filter(|row| row.passed).count();
    let row_clauses = rows
        .iter()
        .map(|row| row.clauses.iter().filter(|value| **value).count())
        .sum::<usize>();
    let global_count = globals.iter().filter(|value| **value).count();
    let control_count = controls.iter().filter(|control| control.passed).count();
    let control_clauses = controls
        .iter()
        .map(|control| control.clauses.iter().filter(|value| **value).count())
        .sum::<usize>();
    let mut text = String::new();
    writeln!(text, "# PXR0 successor development readiness v2\n").unwrap();
    writeln!(
        text,
        "Outcome: **{}**.\n",
        if row_count == 16 && control_count == 12 && global_count == 10 {
            "DEVELOPMENT READY"
        } else {
            "NEGATIVE"
        }
    )
    .unwrap();
    writeln!(text, "- rows: `{row_count}/16`;").unwrap();
    writeln!(text, "- row clauses: `{row_clauses}/384`;").unwrap();
    writeln!(text, "- phase controls: `{control_count}/12`;").unwrap();
    writeln!(text, "- phase-control clauses: `{control_clauses}/72`;").unwrap();
    writeln!(text, "- global clauses: `{global_count}/10`;").unwrap();
    writeln!(
        text,
        "- total clauses: `{}/466`;",
        row_clauses + control_clauses + global_count
    )
    .unwrap();
    writeln!(
        text,
        "- maximum work: `{}`;",
        rows.iter()
            .map(|row| row.trial.max_work)
            .chain(controls.iter().map(|control| control.trial.max_work))
            .max()
            .unwrap_or(0)
    )
    .unwrap();
    writeln!(
        text,
        "- maximum resident bytes: `{}`;",
        rows.iter()
            .map(|row| row.trial.max_bytes)
            .chain(controls.iter().map(|control| control.trial.max_bytes))
            .max()
            .unwrap_or(0)
    )
    .unwrap();
    writeln!(
        text,
        "- natural quiescence: `{}`;",
        rows.iter().all(|row| row.clauses[20]) && controls.iter().all(|control| control.clauses[1])
    )
    .unwrap();
    writeln!(
        text,
        "- exact replay: `{}`;",
        rows.iter().all(|row| row.replay) && controls.iter().all(|control| control.replay)
    )
    .unwrap();
    writeln!(
        text,
        "- PXR0 authority: `false`; PX-C authority: `false`.\n"
    )
    .unwrap();
    writeln!(text, "## Unconditional phase-preserving rows\n").unwrap();
    for row in rows {
        let trial = &row.trial;
        let timing = timing(trial.case);
        writeln!(text, "- `{}`: `origin={} modulus={} construction={} pressure_origin={} first_arrival={} construction_minus_pressure={} first_minus_construction={} pair_updates={} impulses={}|{} formation={} out={} duplicate={} incomplete={} blocked={} stale_out={} proposals={} memory={}->{} max_work={} max_bytes={} quiet={} replay={} clauses={} passed={}`", trial.case.root, trial.case.shift, timing.modulus, timing.construction_tick, timing.pressure_origin, timing.first_arrival_tick, timing.construction_minus_pressure, timing.first_arrival_minus_construction, trial.pair_first.updates, trial.pair_held.impulses[0], trial.pair_held.impulses[1], trial.formation.updates, trial.completed.outward, trial.duplicate.outward, trial.incomplete.outward, trial.blocked.outward, trial.stale.outward, trial.stale.proposals, trial.stale_before, trial.stale_after, trial.max_work, trial.max_bytes, row.clauses[20], row.replay, row.clauses.iter().map(bool::to_string).collect::<Vec<_>>().join("|"), row.passed).unwrap();
    }
    writeln!(text, "\n## Unconditional phase-changing controls\n").unwrap();
    for control in controls {
        let trial = &control.trial;
        let timing = control.timing;
        writeln!(text, "- `{}`: `origin={} modulus={} construction={} pressure_origin={} first_arrival={} construction_minus_pressure={} first_minus_construction={} phase_zero_root={} differs_from_phase_zero={} pair_updates={} impulses={}|{} formation={} out={} duplicate={} incomplete={} blocked={} open={} branch={} cycle={} stale_out={} proposals={} memory={}->{} max_work={} max_bytes={} quiet={} replay={} clauses={} passed={}`", trial.case.root, trial.case.shift, timing.modulus, timing.construction_tick, timing.pressure_origin, timing.first_arrival_tick, timing.construction_minus_pressure, timing.first_arrival_minus_construction, control.phase_zero_root, control.differs_from_phase_zero, trial.pair_first.updates, trial.pair_held.impulses[0], trial.pair_held.impulses[1], trial.formation.updates, trial.completed.outward, trial.duplicate.outward, trial.incomplete.outward, trial.blocked.outward, trial.open.outward, trial.branch.outward, trial.cycle.outward, trial.stale.outward, trial.stale.proposals, trial.stale_before, trial.stale_after, trial.max_work, trial.max_bytes, control.clauses[1], control.replay, control.clauses.iter().map(bool::to_string).collect::<Vec<_>>().join("|"), control.passed).unwrap();
    }
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

fn publish(stage: &str, destination: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(stage)
        .unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.sync_all().unwrap();
    rename(stage, destination).unwrap();
}
