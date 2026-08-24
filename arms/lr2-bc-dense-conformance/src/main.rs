#![forbid(unsafe_code)]

#[cfg(all(feature = "arm-b", feature = "arm-c"))]
compile_error!("select exactly one LR2 arm");
#[cfg(not(any(feature = "arm-b", feature = "arm-c")))]
compile_error!("select exactly one LR2 arm");

#[cfg(feature = "arm-b")]
use lr1_compartmental_physical_return as physics;
#[cfg(feature = "arm-c")]
use lr1_modulatory_physical_return as physics;
#[cfg(feature = "arm-b")]
use physics::ArrowSpec;
use physics::{ArrowId, CellId, CellSpec, Crossing, Execution, PlasticSubstrate, SpikeInput};
#[cfg(feature = "arm-c")]
use physics::{ArrowSpec, TransmissionMode};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PROTOCOL: &str = "943414a4e8b98bc66df70e38803ab920894bfa7f47b7b7a088efc7278c2fe3d6";
const B_LAW: &str = "0494b7b82a72ed8dfd254fa862d308bcb6a44fc739c9fbfbf7af23af12309611";
const C_LAW: &str = "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10";
const SEEDS: [u64; 4] = [5201, 5209, 5227, 5231];
const BASE: u64 = 0x7_2200_0000_0000;

#[cfg(feature = "arm-b")]
const ARM: &str = "B";
#[cfg(feature = "arm-c")]
const ARM: &str = "C";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Suite {
    Dense,
    Px0,
    Px1,
    Px2,
}

impl Suite {
    fn name(self) -> &'static str {
        match self {
            Self::Dense => "dense",
            Self::Px0 => "px0",
            Self::Px1 => "px1",
            Self::Px2 => "px2",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Case {
    suite: Suite,
    index: u8,
    name: &'static str,
}

const CASES: [Case; 29] = [
    Case {
        suite: Suite::Dense,
        index: 0,
        name: "unrelated-nearby-no-lawful-return",
    },
    Case {
        suite: Suite::Dense,
        index: 1,
        name: "lawful-return-in-dense-world",
    },
    Case {
        suite: Suite::Dense,
        index: 2,
        name: "other-candidate-return",
    },
    Case {
        suite: Suite::Dense,
        index: 3,
        name: "both-traverse-one-return",
    },
    Case {
        suite: Suite::Dense,
        index: 4,
        name: "both-traverse-both-return",
    },
    Case {
        suite: Suite::Dense,
        index: 5,
        name: "cross-and-distractors-no-return",
    },
    Case {
        suite: Suite::Dense,
        index: 6,
        name: "lawful-return-with-all-distractors",
    },
    Case {
        suite: Suite::Dense,
        index: 7,
        name: "return-without-candidate",
    },
    Case {
        suite: Suite::Dense,
        index: 8,
        name: "late-lawful-return",
    },
    Case {
        suite: Suite::Dense,
        index: 9,
        name: "adjacent-renewed-upstream",
    },
    Case {
        suite: Suite::Px0,
        index: 0,
        name: "acquisition",
    },
    Case {
        suite: Suite::Px0,
        index: 1,
        name: "probation",
    },
    Case {
        suite: Suite::Px0,
        index: 2,
        name: "stable-return-specificity",
    },
    Case {
        suite: Suite::Px0,
        index: 3,
        name: "forgetting",
    },
    Case {
        suite: Suite::Px0,
        index: 4,
        name: "fresh-reproposal",
    },
    Case {
        suite: Suite::Px0,
        index: 5,
        name: "reacquisition",
    },
    Case {
        suite: Suite::Px1,
        index: 0,
        name: "support-a",
    },
    Case {
        suite: Suite::Px1,
        index: 1,
        name: "support-b",
    },
    Case {
        suite: Suite::Px1,
        index: 2,
        name: "no-support",
    },
    Case {
        suite: Suite::Px1,
        index: 3,
        name: "participation-return-blocked",
    },
    Case {
        suite: Suite::Px1,
        index: 4,
        name: "return-without-participation",
    },
    Case {
        suite: Suite::Px1,
        index: 5,
        name: "joint-participation",
    },
    Case {
        suite: Suite::Px2,
        index: 0,
        name: "forward-traversal-return",
    },
    Case {
        suite: Suite::Px2,
        index: 1,
        name: "reverse-traversal-return",
    },
    Case {
        suite: Suite::Px2,
        index: 2,
        name: "correlation-without-traversal",
    },
    Case {
        suite: Suite::Px2,
        index: 3,
        name: "forward-return-blocked",
    },
    Case {
        suite: Suite::Px2,
        index: 4,
        name: "joint-traversal-return",
    },
    Case {
        suite: Suite::Px2,
        index: 5,
        name: "forward-with-wrong-way-observation",
    },
    Case {
        suite: Suite::Px2,
        index: 6,
        name: "reverse-with-wrong-way-observation",
    },
];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Totals {
    work: u64,
    updates: u64,
    accepts: u64,
    path_edges: u64,
    firings: u64,
    deallocations: u64,
    quiescent: bool,
    crossings: Vec<Crossing>,
}

impl Totals {
    fn take(&mut self, run: Execution) {
        self.work = self.work.saturating_add(run.work.total());
        self.updates = self.work_field_add(self.updates, run.work.local_return_updates);
        self.accepts = self.work_field_add(self.accepts, run.work.qualified_return_accepts);
        self.path_edges =
            self.work_field_add(self.path_edges, run.work.qualified_return_path_edges);
        self.firings = self.work_field_add(self.firings, run.work.firings);
        self.deallocations =
            self.work_field_add(self.deallocations, run.work.physical_deallocations);
        self.quiescent = self.quiescent && run.naturally_quiescent;
        self.crossings.extend(run.crossings);
    }

    fn seeded() -> Self {
        Self {
            quiescent: true,
            ..Self::default()
        }
    }

    fn work_field_add(&self, left: u64, right: u64) -> u64 {
        left.saturating_add(right)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    updates_a: u64,
    updates_b: u64,
    resistance_a: u32,
    resistance_b: u32,
    coupling_a: i32,
    coupling_b: i32,
    live_a: bool,
    live_b: bool,
    candidate_crossings_a: usize,
    candidate_crossings_b: usize,
    generations: String,
    work: u64,
    accepts: u64,
    path_edges: u64,
    firings: u64,
    deallocations: u64,
    bytes: usize,
    fingerprint: u64,
    permanent: u64,
    quiescent: bool,
    clauses: [bool; 8],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    case: Case,
    namespace: u64,
    observation: Observation,
    replay: bool,
    passed: bool,
}

#[derive(Clone)]
struct PairWorld {
    substrate: PlasticSubstrate,
    namespace: u64,
    upstream_a: [CellId; 2],
    upstream_b: [CellId; 2],
    return_a: CellId,
    return_b: CellId,
    distractors: Vec<CellId>,
    candidate_a: ArrowId,
    candidate_b: ArrowId,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [arg] if arg == "--preflight" => {
            audit();
            surface();
            absent();
            println!("LR2_{ARM}_PREFLIGHT_OK");
        }
        [arg] if arg == "--development" => {
            audit();
            surface();
            absent();
            eprintln!("LR2_{ARM}_DEVELOPMENT_EVIDENCE_SPENT");
            evidence();
        }
        _ => std::process::exit(2),
    }
}

fn audit() {
    assert_eq!(
        sha("experiments/lr2_dense_topology_px0_px2_successor_conformance_protocol_v1.md"),
        PROTOCOL
    );
    #[cfg(feature = "arm-b")]
    assert_eq!(
        sha("crates/lr1-compartmental-physical-return/src/lib.rs"),
        B_LAW
    );
    #[cfg(feature = "arm-c")]
    assert_eq!(
        sha("crates/lr1-modulatory-physical-return/src/lib.rs"),
        C_LAW
    );
}

fn surface() {
    assert_eq!(CASES.into_iter().collect::<BTreeSet<_>>().len(), 29);
    assert_eq!(SEEDS.into_iter().collect::<BTreeSet<_>>().len(), 4);
    assert_eq!(CASES.iter().filter(|c| c.suite == Suite::Dense).count(), 10);
    assert_eq!(CASES.iter().filter(|c| c.suite == Suite::Px0).count(), 6);
    assert_eq!(CASES.iter().filter(|c| c.suite == Suite::Px1).count(), 6);
    assert_eq!(CASES.iter().filter(|c| c.suite == Suite::Px2).count(), 7);
}

fn evidence() {
    let rows = SEEDS
        .into_iter()
        .flat_map(|seed| CASES.map(|case| replay(seed, case)))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 116);
    let (csv_path, md_path, csv_stage, md_stage) = paths();
    publish(csv_stage, csv_path, &csv(&rows));
    publish(md_stage, md_path, &report(&rows));
}

fn replay(seed: u64, case: Case) -> Row {
    let first = run(seed, case);
    let second = run(seed, case);
    let exact = first == second;
    let passed = exact && first.clauses.into_iter().all(|v| v);
    Row {
        seed,
        case,
        namespace: namespace(seed, case),
        observation: first,
        replay: exact,
        passed,
    }
}

fn run(seed: u64, case: Case) -> Observation {
    match case.suite {
        Suite::Dense => run_dense(seed, case),
        Suite::Px0 => run_px0(seed, case),
        Suite::Px1 => run_px1(seed, case),
        Suite::Px2 => run_px2(seed, case),
    }
}

fn run_dense(seed: u64, case: Case) -> Observation {
    let mut world = build_pair(namespace(seed, case), seed, true, 1);
    let start_a = world.substrate.arrow_resistance(world.candidate_a);
    let start_b = world.substrate.arrow_resistance(world.candidate_b);
    match case.index {
        0 => {
            drive_a(&mut world, 0);
            distract(&mut world, 1, 0);
        }
        1 => {
            drive_a(&mut world, 0);
            lawful_a(&mut world, 1);
        }
        2 => {
            drive_a(&mut world, 0);
            lawful_b(&mut world, 1);
        }
        3 => {
            drive_a(&mut world, 0);
            drive_b(&mut world, 0);
            lawful_a(&mut world, 1);
        }
        4 => {
            drive_a(&mut world, 0);
            drive_b(&mut world, 0);
            lawful_a(&mut world, 1);
            lawful_b(&mut world, 1);
        }
        5 => {
            drive_a(&mut world, 0);
            for i in 0..world.distractors.len() {
                distract(&mut world, 1, i);
            }
            lawful_b(&mut world, 1);
        }
        6 => {
            drive_a(&mut world, 0);
            lawful_a(&mut world, 1);
            for i in 0..world.distractors.len() {
                distract(&mut world, 1, i);
            }
        }
        7 => lawful_a(&mut world, 1),
        8 => {
            drive_a(&mut world, 0);
            lawful_a(&mut world, 6);
        }
        9 => {
            drive_a(&mut world, 0);
            drive_a(&mut world, 2);
        }
        _ => unreachable!(),
    }
    let execution = world.substrate.propagate();
    let mut totals = Totals::seeded();
    totals.take(execution);
    let updates_a = u64::from(world.substrate.arrow_resistance(world.candidate_a) > start_a);
    let updates_b = u64::from(world.substrate.arrow_resistance(world.candidate_b) > start_b);
    let expected = match case.index {
        0 | 2 | 5 | 7 | 8 | 9 => (0, 0),
        1 | 3 | 6 => (1, 0),
        4 => (1, 1),
        _ => unreachable!(),
    };
    observe_pair(
        world,
        totals,
        updates_a,
        updates_b,
        [
            updates_a == expected.0,
            updates_b == expected.1,
            true,
            true,
            true,
            true,
            true,
            true,
        ],
    )
}

fn run_px0(seed: u64, case: Case) -> Observation {
    if case.index >= 4 {
        return run_reproposal(seed, case);
    }
    let mut world = build_pair(namespace(seed, case), seed, false, 1);
    let mut totals = Totals::seeded();
    match case.index {
        0 => recurrent_a(&mut world, &[0, 6, 12, 18], true),
        1 => recurrent_a(&mut world, &[0], true),
        2 => {
            drive_a(&mut world, 0);
            drive_b(&mut world, 0);
            lawful_a(&mut world, 1);
        }
        3 => recurrent_a(&mut world, &[0, 6, 12, 18], true),
        _ => unreachable!(),
    }
    totals.take(world.substrate.propagate());
    let before_pressure = world.substrate.arrow_resistance(world.candidate_a);
    let bounded_live = if case.index == 3 {
        let pressure = world.substrate.advance_time(30);
        totals.work += pressure.total();
        world.substrate.arrow_is_live(world.candidate_a)
    } else {
        true
    };
    if case.index == 1 || case.index == 3 {
        let pressure = world.substrate.advance_time(200);
        totals.work += pressure.total();
        totals.deallocations += pressure.physical_deallocations;
    }
    let clauses = match case.index {
        0 => [
            before_pressure > 4,
            world.substrate.arrow_coupling(world.candidate_a) == 2,
            true,
            true,
            true,
            true,
            true,
            true,
        ],
        1 => [
            before_pressure == 4,
            !world.substrate.arrow_is_live(world.candidate_a),
            true,
            true,
            true,
            true,
            true,
            true,
        ],
        2 => [
            world.substrate.arrow_resistance(world.candidate_a) == 4,
            world.substrate.arrow_resistance(world.candidate_b) == 1,
            true,
            true,
            true,
            true,
            true,
            true,
        ],
        3 => [
            bounded_live,
            !world.substrate.arrow_is_live(world.candidate_a),
            before_pressure > 4,
            true,
            true,
            true,
            true,
            true,
        ],
        _ => unreachable!(),
    };
    observe_pair(world, totals, 0, 0, clauses)
}

fn run_reproposal(seed: u64, case: Case) -> Observation {
    let ns = namespace(seed, case);
    let mut substrate = PlasticSubstrate::new();
    let p = substrate.add_cell(cell(ns + 20, 0, 20, 2));
    let x = substrate.add_cell(cell(ns + 30, 2, 30, 1));
    #[cfg(feature = "arm-b")]
    let compartment = substrate.add_cell(cell(ns + 70, -1, 70, 8));
    let u0 = substrate.add_cell(cell(ns + 10, -1000, 10, 1));
    let u1 = substrate.add_cell(cell(ns + 11, -1100, 11, 1));
    let r = substrate.add_cell(cell(ns + 50, 1000, 50, 1));
    add_drive(&mut substrate, u0, p, 0, 1, 100);
    add_drive(&mut substrate, u1, p, 0, 1, 100);
    let old = add_drive(&mut substrate, p, x, 0, 1, 1);
    #[cfg(feature = "arm-b")]
    add_return(&mut substrate, r, compartment, 0, 100);
    #[cfg(feature = "arm-c")]
    add_return(&mut substrate, r, p, 0, 100);
    pulse(&mut substrate, u0, 0, 1, 0);
    pulse(&mut substrate, u1, 0, 1, 1);
    let mut totals = Totals::seeded();
    totals.take(substrate.propagate());
    let pressure = substrate.advance_time(10);
    totals.work += pressure.total();
    totals.deallocations += pressure.physical_deallocations;
    pulse(&mut substrate, p, 11, 2, 2);
    if case.index == 5 {
        pulse(&mut substrate, r, 12, 1, 3);
    }
    totals.take(substrate.propagate());
    let candidates = substrate.arrows_between(p, x);
    let fresh = *candidates.last().expect("fresh local proposal");
    if case.index == 5 {
        for (tick, phase) in [(17, 4), (23, 6), (29, 8)] {
            pulse(&mut substrate, p, tick, 2, phase);
            pulse(&mut substrate, r, tick + 1, 1, phase + 1);
        }
        totals.take(substrate.propagate());
    }
    let fresh_generation = substrate.arrow_generation(fresh);
    let clauses = [
        !substrate.arrow_is_live(old),
        fresh != old,
        fresh_generation > substrate.arrow_generation(old),
        case.index == 4 || substrate.arrow_resistance(fresh) > 1,
        case.index == 4 || substrate.arrow_coupling(fresh) == 2,
        totals.quiescent,
        true,
        true,
    ];
    Observation {
        updates_a: 0,
        updates_b: 0,
        resistance_a: substrate.arrow_resistance(fresh),
        resistance_b: 0,
        coupling_a: substrate.arrow_coupling(fresh),
        coupling_b: 0,
        live_a: substrate.arrow_is_live(fresh),
        live_b: false,
        candidate_crossings_a: totals.crossings.len(),
        candidate_crossings_b: 0,
        generations: format!(
            "old{}|fresh{}",
            substrate.arrow_generation(old),
            fresh_generation
        ),
        work: totals.work,
        accepts: totals.accepts,
        path_edges: totals.path_edges,
        firings: totals.firings,
        deallocations: totals.deallocations,
        bytes: substrate.persistent_bytes(),
        fingerprint: substrate.complete_fingerprint(),
        permanent: substrate.permanent_fingerprint(),
        quiescent: totals.quiescent,
        clauses,
    }
}

fn run_px1(seed: u64, case: Case) -> Observation {
    let mut world = build_pair(namespace(seed, case), seed, false, 1);
    match case.index {
        0 => recurrent_a(&mut world, &[0, 6], true),
        1 => recurrent_b(&mut world, &[0, 6], true),
        2 => {}
        3 => recurrent_a(&mut world, &[0, 6], false),
        4 => {
            lawful_a(&mut world, 1);
            lawful_a(&mut world, 7);
        }
        5 => {
            recurrent_a(&mut world, &[0, 6], true);
            recurrent_b(&mut world, &[0, 6], true);
        }
        _ => unreachable!(),
    }
    let mut totals = Totals::seeded();
    totals.take(world.substrate.propagate());
    let mature_a = world.substrate.arrow_resistance(world.candidate_a) > 1;
    let mature_b = world.substrate.arrow_resistance(world.candidate_b) > 1;
    let expected = match case.index {
        0 => (true, false),
        1 => (false, true),
        5 => (true, true),
        _ => (false, false),
    };
    let clauses = [
        mature_a == expected.0,
        mature_b == expected.1,
        totals.updates == u64::from(expected.0) * 2 + u64::from(expected.1) * 2,
        totals.accepts == totals.updates,
        totals.quiescent,
        true,
        true,
        true,
    ];
    observe_pair(
        world,
        totals,
        u64::from(mature_a),
        u64::from(mature_b),
        clauses,
    )
}

fn run_px2(seed: u64, case: Case) -> Observation {
    let mut world = build_pair(namespace(seed, case), seed, false, 1);
    match case.index {
        0 => recurrent_a(&mut world, &[0, 6], true),
        1 => recurrent_b(&mut world, &[0, 6], true),
        2 => {
            for tick in [1, 3, 5, 7, 9, 11] {
                lawful_a(&mut world, tick);
                lawful_b(&mut world, tick);
            }
        }
        3 => recurrent_a(&mut world, &[0, 6], false),
        4 => {
            recurrent_a(&mut world, &[0, 6], true);
            recurrent_b(&mut world, &[0, 6], true);
        }
        5 => {
            recurrent_a(&mut world, &[0, 6], true);
            for tick in [1, 3, 5, 7, 9, 11] {
                lawful_b(&mut world, tick);
            }
        }
        6 => {
            recurrent_b(&mut world, &[0, 6], true);
            for tick in [1, 3, 5, 7, 9, 11] {
                lawful_a(&mut world, tick);
            }
        }
        _ => unreachable!(),
    }
    let mut totals = Totals::seeded();
    totals.take(world.substrate.propagate());
    let mature_a = world.substrate.arrow_resistance(world.candidate_a) > 1;
    let mature_b = world.substrate.arrow_resistance(world.candidate_b) > 1;
    let expected = match case.index {
        0 | 5 => (true, false),
        1 | 6 => (false, true),
        4 => (true, true),
        _ => (false, false),
    };
    let clauses = [
        mature_a == expected.0,
        mature_b == expected.1,
        totals.accepts == totals.updates,
        case.index < 5 || (mature_a != mature_b || case.index == 4),
        totals.quiescent,
        true,
        true,
        true,
    ];
    observe_pair(
        world,
        totals,
        u64::from(mature_a),
        u64::from(mature_b),
        clauses,
    )
}

fn build_pair(ns: u64, seed: u64, dense: bool, candidate_resistance: u32) -> PairWorld {
    let mut substrate = PlasticSubstrate::new();
    let reflect = seed % 4 >= 2;
    let side = if reflect { -1 } else { 1 };
    let p = substrate.add_cell(cell(ns + 20, 0, 20, 2));
    let q = substrate.add_cell(cell(ns + 21, side * 1000, 21, 2));
    let x = substrate.add_cell(cell(ns + 30, side * 100, 30, 1));
    let y = substrate.add_cell(cell(ns + 31, side * 1100, 31, 1));
    #[cfg(feature = "arm-b")]
    let compartment_a = substrate.add_cell(cell(ns + 70, side, 70, 32));
    #[cfg(feature = "arm-b")]
    let compartment_b = substrate.add_cell(cell(ns + 71, side * 1001, 71, 32));
    let upstream_a = [
        substrate.add_cell(cell(ns + 10, -10_000, 10, 1)),
        substrate.add_cell(cell(ns + 11, -11_000, 11, 1)),
    ];
    let upstream_b = [
        substrate.add_cell(cell(ns + 12, -12_000, 12, 1)),
        substrate.add_cell(cell(ns + 13, -13_000, 13, 1)),
    ];
    let return_a = substrate.add_cell(cell(ns + 50, 10_000, 50, 1));
    let return_b = substrate.add_cell(cell(ns + 51, 11_000, 51, 1));
    for upstream in upstream_a {
        add_drive(&mut substrate, upstream, p, 0, 1, 100);
    }
    for upstream in upstream_b {
        add_drive(&mut substrate, upstream, q, 0, 1, 100);
    }
    let candidate_a = add_drive(&mut substrate, p, x, 0, 1, candidate_resistance);
    let candidate_b = add_drive(&mut substrate, q, y, 0, 1, candidate_resistance);
    #[cfg(feature = "arm-b")]
    {
        add_return(&mut substrate, return_a, compartment_a, 0, 100);
        add_return(&mut substrate, return_b, compartment_b, 0, 100);
    }
    #[cfg(feature = "arm-c")]
    {
        add_return(&mut substrate, return_a, p, 0, 100);
        add_return(&mut substrate, return_b, q, 0, 100);
    }
    let mut distractors = Vec::new();
    if dense {
        for index in 0..16_u64 {
            let source =
                substrate.add_cell(cell(ns + 100 + index, 20_000 + index as i32 * 100, 80, 1));
            #[cfg(feature = "arm-b")]
            let target = if index % 2 == 0 {
                compartment_a
            } else {
                compartment_b
            };
            #[cfg(feature = "arm-c")]
            let target = if index % 2 == 0 { p } else { q };
            add_drive(&mut substrate, source, target, 0, 1, 100);
            distractors.push(source);
        }
    }
    PairWorld {
        substrate,
        namespace: ns,
        upstream_a,
        upstream_b,
        return_a,
        return_b,
        distractors,
        candidate_a,
        candidate_b,
    }
}

fn recurrent_a(world: &mut PairWorld, ticks: &[i64], returned: bool) {
    for &tick in ticks {
        drive_a(world, tick);
        if returned {
            lawful_a(world, tick + 1);
        }
    }
}

fn recurrent_b(world: &mut PairWorld, ticks: &[i64], returned: bool) {
    for &tick in ticks {
        drive_b(world, tick);
        if returned {
            lawful_b(world, tick + 1);
        }
    }
}

fn drive_a(world: &mut PairWorld, tick: i64) {
    for side in 0..2 {
        pulse(
            &mut world.substrate,
            world.upstream_a[side],
            tick,
            1,
            side as i32,
        );
    }
}
fn drive_b(world: &mut PairWorld, tick: i64) {
    for side in 0..2 {
        pulse(
            &mut world.substrate,
            world.upstream_b[side],
            tick,
            1,
            10 + side as i32,
        );
    }
}
fn lawful_a(world: &mut PairWorld, tick: i64) {
    pulse(&mut world.substrate, world.return_a, tick, 1, 20);
}
fn lawful_b(world: &mut PairWorld, tick: i64) {
    pulse(&mut world.substrate, world.return_b, tick, 1, 21);
}
fn distract(world: &mut PairWorld, tick: i64, index: usize) {
    let source = world.distractors[index];
    pulse(&mut world.substrate, source, tick, 1, 30 + index as i32);
}

fn observe_pair(
    world: PairWorld,
    totals: Totals,
    updates_a: u64,
    updates_b: u64,
    mut clauses: [bool; 8],
) -> Observation {
    clauses[6] &= totals.quiescent;
    clauses[7] &= totals.accepts == totals.updates;
    Observation {
        updates_a,
        updates_b,
        resistance_a: world.substrate.arrow_resistance(world.candidate_a),
        resistance_b: world.substrate.arrow_resistance(world.candidate_b),
        coupling_a: world.substrate.arrow_coupling(world.candidate_a),
        coupling_b: world.substrate.arrow_coupling(world.candidate_b),
        live_a: world.substrate.arrow_is_live(world.candidate_a),
        live_b: world.substrate.arrow_is_live(world.candidate_b),
        candidate_crossings_a: count_crossings(
            &totals.crossings,
            world.namespace + 20,
            world.namespace + 30,
        ),
        candidate_crossings_b: count_crossings(
            &totals.crossings,
            world.namespace + 21,
            world.namespace + 31,
        ),
        generations: format!(
            "a{}|b{}",
            world.substrate.arrow_generation(world.candidate_a),
            world.substrate.arrow_generation(world.candidate_b)
        ),
        work: totals.work,
        accepts: totals.accepts,
        path_edges: totals.path_edges,
        firings: totals.firings,
        deallocations: totals.deallocations,
        bytes: world.substrate.persistent_bytes(),
        fingerprint: world.substrate.complete_fingerprint(),
        permanent: world.substrate.permanent_fingerprint(),
        quiescent: totals.quiescent,
        clauses,
    }
}

fn add_drive(
    substrate: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    delay: i64,
    coupling: i32,
    resistance: u32,
) -> ArrowId {
    #[cfg(feature = "arm-b")]
    let spec = ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
    };
    #[cfg(feature = "arm-c")]
    let spec = ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode: TransmissionMode::Drive,
    };
    substrate.add_arrow(spec)
}

fn add_return(
    substrate: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    delay: i64,
    resistance: u32,
) -> ArrowId {
    #[cfg(feature = "arm-b")]
    let spec = ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling: 1,
        resistance,
    };
    #[cfg(feature = "arm-c")]
    let spec = ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling: 1,
        resistance,
        mode: TransmissionMode::Modulatory,
    };
    substrate.add_arrow(spec)
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

fn pulse(substrate: &mut PlasticSubstrate, target: CellId, tick: i64, impulse: i32, phase: i32) {
    substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: 0xF200_0000 + phase.max(0) as u64,
        target,
        impulse,
    });
}

fn count_crossings(crossings: &[Crossing], from: u64, to: u64) -> usize {
    crossings
        .iter()
        .filter(|c| c.from_physical == from && c.to_physical == to)
        .count()
}

fn namespace(seed: u64, case: Case) -> u64 {
    BASE + seed * 0x1000_0000
        + (case.suite as u64) * 0x0100_0000
        + u64::from(case.index) * 0x0010_0000
}

fn paths() -> (&'static str, &'static str, &'static str, &'static str) {
    #[cfg(feature = "arm-b")]
    {
        (
            "results/lr2_arm_b_v1.csv",
            "results/lr2_arm_b_v1.md",
            "results/.lr2_arm_b_v1.csv.staging",
            "results/.lr2_arm_b_v1.md.staging",
        )
    }
    #[cfg(feature = "arm-c")]
    {
        (
            "results/lr2_arm_c_v1.csv",
            "results/lr2_arm_c_v1.md",
            "results/.lr2_arm_c_v1.csv.staging",
            "results/.lr2_arm_c_v1.md.staging",
        )
    }
}

fn absent() {
    let (a, b, c, d) = paths();
    for path in [a, b, c, d] {
        assert!(!Path::new(path).exists(), "artifact exists: {path}");
    }
}

fn join_bool(values: &[bool]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn csv(rows: &[Row]) -> String {
    let mut out = String::from("arm,seed,suite,scenario,namespace,updates_a,updates_b,resistance_a,resistance_b,coupling_a,coupling_b,live_a,live_b,crossings_a,crossings_b,generations,work,accepts,path_edges,firings,deallocations,bytes,fingerprint,permanent,quiescent,clauses,replay,passed\n");
    for row in rows {
        let o = &row.observation;
        out.push_str(&format!("{ARM},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n", row.seed, row.case.suite.name(), row.case.name, row.namespace, o.updates_a, o.updates_b, o.resistance_a, o.resistance_b, o.coupling_a, o.coupling_b, o.live_a, o.live_b, o.candidate_crossings_a, o.candidate_crossings_b, o.generations, o.work, o.accepts, o.path_edges, o.firings, o.deallocations, o.bytes, o.fingerprint, o.permanent, o.quiescent, join_bool(&o.clauses), row.replay, row.passed));
    }
    out
}

fn report(rows: &[Row]) -> String {
    let suite_line = |suite: Suite| {
        let selected = rows
            .iter()
            .filter(|r| r.case.suite == suite)
            .collect::<Vec<_>>();
        let passed = selected.iter().filter(|r| r.passed).count();
        let clauses = selected
            .iter()
            .map(|r| r.observation.clauses.into_iter().filter(|v| *v).count())
            .sum::<usize>();
        format!(
            "- {}: `{passed}/{}` rows; `{clauses}/{}` clauses",
            suite.name(),
            selected.len(),
            selected.len() * 8
        )
    };
    let passed = rows.iter().filter(|r| r.passed).count();
    let max_work = rows.iter().map(|r| r.observation.work).max().unwrap_or(0);
    let total_work = rows.iter().map(|r| r.observation.work).sum::<u64>();
    let max_bytes = rows.iter().map(|r| r.observation.bytes).max().unwrap_or(0);
    format!("# LR2 Arm {ARM} dense topology and PX0--PX2 successor conformance\n\nOutcome: **{}**.\n\n{}\n{}\n{}\n{}\n- all rows: `{passed}/{}`;\n- exact replay: `{}`;\n- natural quiescence: `{}`;\n- total/max work: `{total_work}/{max_work}`;\n- maximum persistent bytes: `{max_bytes}`;\n- added persistent state field: `{}`;\n- added transmission modes: `{}`;\n- authority replayed: `false`;\n- PX3 reopened: `false`.\n", if passed == rows.len() { format!("LR2-{ARM} FUNCTIONAL POSITIVE") } else { format!("LR2-{ARM} NEGATIVE") }, suite_line(Suite::Dense), suite_line(Suite::Px0), suite_line(Suite::Px1), suite_line(Suite::Px2), rows.len(), rows.iter().all(|r| r.replay), rows.iter().all(|r| r.observation.quiescent), if ARM == "B" { "0" } else { "1 mode field per ARROW" }, if ARM == "B" { "0" } else { "2" })
}

fn publish(stage: &str, final_path: &str, content: &str) {
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(stage)
        .unwrap_or_else(|e| panic!("create {stage}: {e}"));
    file.write_all(content.as_bytes())
        .unwrap_or_else(|e| panic!("write {stage}: {e}"));
    file.sync_all()
        .unwrap_or_else(|e| panic!("sync {stage}: {e}"));
    rename(stage, final_path).unwrap_or_else(|e| panic!("publish {final_path}: {e}"));
}

fn sha(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .unwrap_or_else(|e| panic!("sha256sum {path}: {e}"));
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .expect("utf8")
        .split_whitespace()
        .next()
        .expect("sha")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_matrix_is_deterministic() {
        for seed in SEEDS {
            for case in CASES {
                let row = replay(seed, case);
                assert!(row.replay, "{seed} {case:?}");
            }
        }
    }
}
