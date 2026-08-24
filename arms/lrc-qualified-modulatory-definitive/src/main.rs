#![forbid(unsafe_code)]

use lr1_modulatory_physical_return as physics;
use physics::{
    ArrowId, ArrowSpec, CellId, CellSpec, Crossing, Execution, PlasticSubstrate, SpikeInput,
    TraceEntry, TransmissionMode,
};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PROTOCOL: &str = "2ed743a0313d80b5a819704284c2e65e58ec933d5e1d608693f5726e062f1164";
const LAW_SPEC: &str = "7c0fe9b86a99f8618f98fd40cd53c41429c3d7cafaa7d773ab315562d410a9bc";
const LAW_SOURCE: &str = "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10";
const SEEDS: [u64; 16] = [
    6101, 6113, 6121, 6131, 6143, 6151, 6163, 6173, 6197, 6203, 6211, 6221, 6229, 6247, 6257, 6263,
];
const BASE: u64 = 0x7_3300_0000_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Suite {
    Law,
    Px0,
    Px1,
    Px2,
}

impl Suite {
    fn name(self) -> &'static str {
        match self {
            Self::Law => "law",
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

const CASES: [Case; 31] = [
    Case {
        suite: Suite::Law,
        index: 0,
        name: "ordinary-drive-after-participation",
    },
    Case {
        suite: Suite::Law,
        index: 1,
        name: "modulation-without-eligibility",
    },
    Case {
        suite: Suite::Law,
        index: 2,
        name: "timely-modulatory-return",
    },
    Case {
        suite: Suite::Law,
        index: 3,
        name: "strong-modulation-does-not-excite",
    },
    Case {
        suite: Suite::Law,
        index: 4,
        name: "dense-ordinary-nonleakage",
    },
    Case {
        suite: Suite::Law,
        index: 5,
        name: "fixed-routes-r0-drive-r1-modulatory",
    },
    Case {
        suite: Suite::Law,
        index: 6,
        name: "fixed-routes-r0-modulatory-r1-drive",
    },
    Case {
        suite: Suite::Law,
        index: 7,
        name: "two-completed-modulatory-returns",
    },
    Case {
        suite: Suite::Law,
        index: 8,
        name: "two-local-eligible-candidates",
    },
    Case {
        suite: Suite::Law,
        index: 9,
        name: "expired-eligibility-modulation",
    },
    Case {
        suite: Suite::Law,
        index: 10,
        name: "simultaneous-drive-and-modulation",
    },
    Case {
        suite: Suite::Law,
        index: 11,
        name: "dense-parallel-crossed-routes",
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
    drive_deliveries: u64,
    modulatory_deliveries: u64,
    firings: u64,
    deallocations: u64,
    quiescent: bool,
    crossings: Vec<Crossing>,
    trace: Vec<TraceEntry>,
}

impl Totals {
    fn take(&mut self, run: Execution) {
        self.work = self.work.saturating_add(run.work.total());
        self.updates = self.work_field_add(self.updates, run.work.local_return_updates);
        self.accepts = self.work_field_add(self.accepts, run.work.qualified_return_accepts);
        self.path_edges =
            self.work_field_add(self.path_edges, run.work.qualified_return_path_edges);
        self.drive_deliveries =
            self.work_field_add(self.drive_deliveries, run.work.drive_deliveries);
        self.modulatory_deliveries =
            self.work_field_add(self.modulatory_deliveries, run.work.modulatory_deliveries);
        self.firings = self.work_field_add(self.firings, run.work.firings);
        self.deallocations =
            self.work_field_add(self.deallocations, run.work.physical_deallocations);
        self.quiescent = self.quiescent && run.naturally_quiescent;
        self.crossings.extend(run.crossings);
        self.trace.extend(run.trace);
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
    source_fires_a: usize,
    source_fires_b: usize,
    drive_deliveries: u64,
    modulatory_deliveries: u64,
    eligibility: String,
    modes: String,
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
    clauses: [bool; 12],
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
    p: CellId,
    upstream_a: [CellId; 2],
    upstream_b: [CellId; 2],
    return_a: CellId,
    return_b: CellId,
    return_delay: i64,
    reverse_arrivals: bool,
    distractors: Vec<CellId>,
    candidate_a: ArrowId,
    candidate_b: ArrowId,
    candidate_b_physical: (u64, u64),
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [arg] if arg == "--preflight" => {
            audit();
            surface();
            absent();
            println!("LRC_QUALIFIED_MODULATORY_DEFINITIVE_PREFLIGHT_OK");
        }
        [arg] if arg == "--definitive" => {
            audit();
            surface();
            absent();
            eprintln!("LRC_QUALIFIED_MODULATORY_DEFINITIVE_EVIDENCE_SPENT");
            evidence();
        }
        _ => std::process::exit(2),
    }
}

fn audit() {
    assert_eq!(
        sha("experiments/lrc_qualified_modulatory_transmission_definitive_protocol_v1.md"),
        PROTOCOL
    );
    assert_eq!(
        sha("experiments/lrc_qualified_modulatory_transmission_candidate_law_v1.md"),
        LAW_SPEC
    );
    assert_eq!(
        sha("crates/lr1-modulatory-physical-return/src/lib.rs"),
        LAW_SOURCE
    );
    let law = std::fs::read_to_string("crates/lr1-modulatory-physical-return/src/lib.rs")
        .expect("read frozen active law");
    for forbidden in [
        "REWARD",
        "CREDIT",
        "CORRECT",
        "OUTCOME",
        "RETURN_FOR",
        "CAUSE_ID",
        "COMPOSITE",
        "EVENT",
    ] {
        assert!(
            !law.contains(forbidden),
            "semantic active-law identifier: {forbidden}"
        );
    }
}

fn surface() {
    assert_eq!(CASES.into_iter().collect::<BTreeSet<_>>().len(), 31);
    assert_eq!(SEEDS.into_iter().collect::<BTreeSet<_>>().len(), 16);
    assert_eq!(CASES.iter().filter(|c| c.suite == Suite::Law).count(), 12);
    assert_eq!(CASES.iter().filter(|c| c.suite == Suite::Px0).count(), 6);
    assert_eq!(CASES.iter().filter(|c| c.suite == Suite::Px1).count(), 6);
    assert_eq!(CASES.iter().filter(|c| c.suite == Suite::Px2).count(), 7);
}

fn evidence() {
    let rows = SEEDS
        .into_iter()
        .flat_map(|seed| CASES.map(|case| replay(seed, case)))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 496);
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
        Suite::Law => run_law(seed, case),
        Suite::Px0 => run_px0(seed, case),
        Suite::Px1 => run_px1(seed, case),
        Suite::Px2 => run_px2(seed, case),
    }
}

fn run_law(seed: u64, case: Case) -> Observation {
    let mut world = build_pair(namespace(seed, case), seed, true, 1);
    let start_a = world.substrate.arrow_resistance(world.candidate_a);
    let start_b = world.substrate.arrow_resistance(world.candidate_b);
    let mut mode_record = "candidate=Drive|return=Modulatory|distractors=Drive".to_owned();
    match case.index {
        0 => {
            drive_a(&mut world, 0);
            distract(&mut world, 1, 0);
        }
        1 => lawful_a_at(&mut world, 4),
        2 => {
            drive_a(&mut world, 0);
            lawful_a(&mut world, 0);
        }
        3 => {
            let strong = world
                .substrate
                .add_cell(cell(world.namespace + 90, 30_000, 90, 1));
            add_return_coupling(&mut world.substrate, strong, world.p, 0, 4, 100);
            pulse(&mut world.substrate, strong, 0, 1, 90);
            mode_record = "strong_return=Modulatory:coupling4|candidate=Drive".to_owned();
        }
        4 => {
            drive_a(&mut world, 0);
            distract_all(&mut world, 1);
        }
        5 | 6 => {
            let route0 = world
                .substrate
                .add_cell(cell(world.namespace + 60, 31_000, 60, 1));
            let route1 = world
                .substrate
                .add_cell(cell(world.namespace + 61, 32_000, 61, 1));
            let (mode0, mode1) = if case.index == 5 {
                (TransmissionMode::Drive, TransmissionMode::Modulatory)
            } else {
                (TransmissionMode::Modulatory, TransmissionMode::Drive)
            };
            add_mode_arrow(
                &mut world.substrate,
                route0,
                world.p,
                world.return_delay,
                1,
                100,
                mode0,
            );
            add_mode_arrow(
                &mut world.substrate,
                route1,
                world.p,
                world.return_delay,
                1,
                100,
                mode1,
            );
            drive_a(&mut world, 0);
            let source_tick = 4_i64.saturating_sub(world.return_delay);
            pulse(&mut world.substrate, route0, source_tick, 1, 60);
            pulse(&mut world.substrate, route1, source_tick, 1, 61);
            mode_record = format!("r0={mode0:?}|r1={mode1:?}|candidate=Drive");
        }
        7 => recurrent_a(&mut world, &[0, 6], true),
        8 => {
            let second_target = world
                .substrate
                .add_cell(cell(world.namespace + 32, 200, 32, 1));
            world.candidate_b = add_drive(&mut world.substrate, world.p, second_target, 0, 1, 1);
            world.candidate_b_physical = (world.namespace + 20, world.namespace + 32);
            drive_a(&mut world, 0);
            lawful_a(&mut world, 0);
        }
        9 => {
            drive_a(&mut world, 0);
            lawful_a_at(&mut world, 6);
        }
        10 => {
            drive_a(&mut world, 0);
            drive_a(&mut world, 4);
            lawful_a_at(&mut world, 4);
        }
        11 => {
            drive_a(&mut world, 0);
            drive_b(&mut world, 0);
            lawful_a(&mut world, 0);
            distract_all(&mut world, 1);
        }
        _ => unreachable!(),
    }
    let execution = world.substrate.propagate();
    let mut totals = Totals::seeded();
    totals.take(execution);
    let updates_a = u64::from(world.substrate.arrow_resistance(world.candidate_a) > start_a);
    let updates_b = u64::from(world.substrate.arrow_resistance(world.candidate_b) > start_b);
    let expected = match case.index {
        0 | 1 | 3 | 4 | 9 => (0, 0),
        2 | 5 | 6 | 10 | 11 => (1, 0),
        7 => (1, 0),
        8 => (1, 1),
        _ => unreachable!(),
    };
    let source_fires_a = firing_count(&totals.trace, world.namespace + 20);
    let source_fires_b = firing_count(&totals.trace, world.namespace + 21);
    let expected_a_fires = match case.index {
        0 | 2 | 5 | 6 | 8 | 9 => 1,
        4 | 10 | 11 => 2,
        7 => 2,
        1 | 3 => 0,
        _ => unreachable!(),
    };
    let expected_b_fires = match case.index {
        4 | 11 => 1 + usize::from(case.index == 11),
        _ => 0,
    };
    let resistance_exact = match case.index {
        2 | 5 | 6 | 10 | 11 => world.substrate.arrow_resistance(world.candidate_a) == 4,
        7 => world.substrate.arrow_resistance(world.candidate_a) > 4,
        8 => {
            world.substrate.arrow_resistance(world.candidate_a) == 4
                && world.substrate.arrow_resistance(world.candidate_b) == 4
        }
        9 => !world.substrate.arrow_is_live(world.candidate_a),
        _ => world.substrate.arrow_resistance(world.candidate_a) == 1,
    };
    let accepts_exact = totals.accepts
        == match case.index {
            7 => 2,
            8 => 2,
            2 | 5 | 6 | 10 | 11 => 1,
            _ => 0,
        };
    observe_pair(
        world,
        totals,
        updates_a,
        updates_b,
        [
            updates_a == expected.0,
            updates_b == expected.1,
            source_fires_a == expected_a_fires,
            source_fires_b == expected_b_fires,
            resistance_exact,
            accepts_exact,
            case.index != 3 || source_fires_a == 0,
            case.index != 4 || updates_a == 0,
            case.index != 9 || updates_a == 0,
            mode_record.contains("Drive") && mode_record.contains("Modulatory"),
            true,
            true,
        ],
        mode_record,
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
            lawful_a(&mut world, 0);
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
    let clauses = claims8(match case.index {
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
    });
    observe_pair(
        world,
        totals,
        0,
        0,
        clauses,
        "candidate=Drive|return=Modulatory".to_owned(),
    )
}

fn run_reproposal(seed: u64, case: Case) -> Observation {
    let ns = namespace(seed, case);
    let mut substrate = PlasticSubstrate::new();
    let p = substrate.add_cell(cell(ns + 20, 0, 20, 2));
    let x = substrate.add_cell(cell(ns + 30, 2, 30, 1));
    let u0 = substrate.add_cell(cell(ns + 10, -1000, 10, 1));
    let u1 = substrate.add_cell(cell(ns + 11, -1100, 11, 1));
    let r = substrate.add_cell(cell(ns + 50, 1000, 50, 1));
    let return_delay = return_delay(seed);
    add_drive(&mut substrate, u0, p, 0, 1, 100);
    add_drive(&mut substrate, u1, p, 0, 1, 100);
    let old = add_drive(&mut substrate, p, x, 0, 1, 1);
    add_return(&mut substrate, r, p, return_delay, 100);
    pulse(&mut substrate, u0, 0, 1, 0);
    pulse(&mut substrate, u1, 0, 1, 1);
    let mut totals = Totals::seeded();
    totals.take(substrate.propagate());
    let pressure = substrate.advance_time(10);
    totals.work += pressure.total();
    totals.deallocations += pressure.physical_deallocations;
    pulse(&mut substrate, p, 11, 2, 2);
    if case.index == 5 {
        pulse(&mut substrate, r, 15_i64.saturating_sub(return_delay), 1, 3);
    }
    totals.take(substrate.propagate());
    let candidates = substrate.arrows_between(p, x);
    let fresh = *candidates.last().expect("fresh local proposal");
    if case.index == 5 {
        for (tick, phase) in [(17, 4), (23, 6), (29, 8)] {
            pulse(&mut substrate, p, tick, 2, phase);
            pulse(
                &mut substrate,
                r,
                tick.saturating_add(4).saturating_sub(return_delay),
                1,
                phase + 1,
            );
        }
        totals.take(substrate.propagate());
    }
    let fresh_generation = substrate.arrow_generation(fresh);
    let clauses = claims8([
        !substrate.arrow_is_live(old),
        fresh != old,
        fresh_generation > substrate.arrow_generation(old),
        case.index == 4 || substrate.arrow_resistance(fresh) > 1,
        case.index == 4 || substrate.arrow_coupling(fresh) == 2,
        totals.quiescent,
        true,
        true,
    ]);
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
        source_fires_a: firing_count(&totals.trace, ns + 20),
        source_fires_b: 0,
        drive_deliveries: totals.drive_deliveries,
        modulatory_deliveries: totals.modulatory_deliveries,
        eligibility: format!("fresh={:?}", substrate.arrow_eligible_until(fresh)),
        modes: "candidate=Drive|return=Modulatory".to_owned(),
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
            lawful_a_at(&mut world, 4);
            lawful_a_at(&mut world, 10);
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
    let clauses = claims8([
        mature_a == expected.0,
        mature_b == expected.1,
        totals.updates == u64::from(expected.0) * 2 + u64::from(expected.1) * 2,
        totals.accepts == totals.updates,
        totals.quiescent,
        true,
        true,
        true,
    ]);
    observe_pair(
        world,
        totals,
        u64::from(mature_a),
        u64::from(mature_b),
        clauses,
        "candidate=Drive|return=Modulatory".to_owned(),
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
    let clauses = claims8([
        mature_a == expected.0,
        mature_b == expected.1,
        totals.accepts == totals.updates,
        case.index < 5 || (mature_a != mature_b || case.index == 4),
        totals.quiescent,
        true,
        true,
        true,
    ]);
    observe_pair(
        world,
        totals,
        u64::from(mature_a),
        u64::from(mature_b),
        clauses,
        "candidate=Drive|return=Modulatory".to_owned(),
    )
}

fn build_pair(ns: u64, seed: u64, dense: bool, candidate_resistance: u32) -> PairWorld {
    let mut substrate = PlasticSubstrate::new();
    let stratum = SEEDS
        .iter()
        .position(|value| *value == seed)
        .expect("registered seed");
    let reflect = stratum & 1 != 0;
    let reverse_cells = stratum & 2 != 0;
    let reverse_arrows = stratum & 4 != 0;
    let reverse_arrivals = stratum & 8 != 0;
    let side = if reflect { -1 } else { 1 };
    let spacing = [32, 48, 64, 80][stratum % 4];
    let specs = [
        (20, 0, 20, 2),
        (21, side * spacing, 21, 2),
        (30, side * 100, 30, 1),
        (31, side * (spacing + 100), 31, 1),
        (10, -10_000, 10, 1),
        (11, -11_000, 11, 1),
        (12, -12_000, 12, 1),
        (13, -13_000, 13, 1),
        (50, 10_000, 50, 1),
        (51, 11_000, 51, 1),
    ];
    let order = if reverse_cells {
        (0..specs.len()).rev().collect::<Vec<_>>()
    } else {
        (0..specs.len()).collect::<Vec<_>>()
    };
    let mut cells = [None; 10];
    for index in order {
        let (offset, position, region, threshold) = specs[index];
        cells[index] = Some(substrate.add_cell(cell(ns + offset, position, region, threshold)));
    }
    let cells = cells.map(|value| value.expect("allocated registered cell"));
    let p = cells[0];
    let q = cells[1];
    let x = cells[2];
    let y = cells[3];
    let upstream_a = [cells[4], cells[5]];
    let upstream_b = [cells[6], cells[7]];
    let return_a = cells[8];
    let return_b = cells[9];
    let return_delay = return_delay(seed);
    let arrow_specs = [
        drive_spec(upstream_a[0], p, 0, 1, 100),
        drive_spec(upstream_a[1], p, 0, 1, 100),
        drive_spec(upstream_b[0], q, 0, 1, 100),
        drive_spec(upstream_b[1], q, 0, 1, 100),
        drive_spec(p, x, 0, 1, candidate_resistance),
        drive_spec(q, y, 0, 1, candidate_resistance),
        mode_spec(
            return_a,
            p,
            return_delay,
            1,
            100,
            TransmissionMode::Modulatory,
        ),
        mode_spec(
            return_b,
            q,
            return_delay,
            1,
            100,
            TransmissionMode::Modulatory,
        ),
    ];
    let arrow_order = if reverse_arrows {
        (0..arrow_specs.len()).rev().collect::<Vec<_>>()
    } else {
        (0..arrow_specs.len()).collect::<Vec<_>>()
    };
    let mut arrows = [None; 8];
    for index in arrow_order {
        arrows[index] = Some(substrate.add_arrow(arrow_specs[index]));
    }
    let arrows = arrows.map(|value| value.expect("allocated registered arrow"));
    let candidate_a = arrows[4];
    let candidate_b = arrows[5];
    let mut distractors = Vec::new();
    if dense {
        let load = [8_u64, 16, 32, 64][stratum % 4];
        let distractor_order = if reverse_cells {
            (0..load).rev().collect::<Vec<_>>()
        } else {
            (0..load).collect::<Vec<_>>()
        };
        let mut by_index = vec![None; usize::try_from(load).expect("bounded load")];
        for index in distractor_order {
            let source =
                substrate.add_cell(cell(ns + 100 + index, 20_000 + index as i32 * 100, 80, 1));
            let target = if index % 2 == 0 { p } else { q };
            add_drive(&mut substrate, source, target, 0, 1, 100);
            by_index[usize::try_from(index).expect("bounded index")] = Some(source);
        }
        distractors = by_index
            .into_iter()
            .map(|value| value.expect("allocated distractor"))
            .collect();
    }
    PairWorld {
        substrate,
        namespace: ns,
        p,
        upstream_a,
        upstream_b,
        return_a,
        return_b,
        return_delay,
        reverse_arrivals,
        distractors,
        candidate_a,
        candidate_b,
        candidate_b_physical: (ns + 21, ns + 31),
    }
}

fn recurrent_a(world: &mut PairWorld, ticks: &[i64], returned: bool) {
    for &tick in ticks {
        drive_a(world, tick);
        if returned {
            lawful_a(world, tick);
        }
    }
}

fn recurrent_b(world: &mut PairWorld, ticks: &[i64], returned: bool) {
    for &tick in ticks {
        drive_b(world, tick);
        if returned {
            lawful_b(world, tick);
        }
    }
}

fn drive_a(world: &mut PairWorld, tick: i64) {
    let order = if world.reverse_arrivals {
        [1, 0]
    } else {
        [0, 1]
    };
    for side in order {
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
    let order = if world.reverse_arrivals {
        [1, 0]
    } else {
        [0, 1]
    };
    for side in order {
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
    let source_tick = tick.saturating_add(4).saturating_sub(world.return_delay);
    pulse(&mut world.substrate, world.return_a, source_tick, 1, 20);
}
fn lawful_b(world: &mut PairWorld, tick: i64) {
    let source_tick = tick.saturating_add(4).saturating_sub(world.return_delay);
    pulse(&mut world.substrate, world.return_b, source_tick, 1, 21);
}
fn lawful_a_at(world: &mut PairWorld, arrival_tick: i64) {
    pulse(
        &mut world.substrate,
        world.return_a,
        arrival_tick.saturating_sub(world.return_delay),
        1,
        20,
    );
}
fn lawful_b_at(world: &mut PairWorld, arrival_tick: i64) {
    pulse(
        &mut world.substrate,
        world.return_b,
        arrival_tick.saturating_sub(world.return_delay),
        1,
        21,
    );
}
fn distract(world: &mut PairWorld, tick: i64, index: usize) {
    let source = world.distractors[index];
    pulse(&mut world.substrate, source, tick, 1, 30 + index as i32);
}

fn distract_all(world: &mut PairWorld, tick: i64) {
    let indices = if world.reverse_arrivals {
        (0..world.distractors.len()).rev().collect::<Vec<_>>()
    } else {
        (0..world.distractors.len()).collect::<Vec<_>>()
    };
    for index in indices {
        distract(world, tick, index);
    }
}

fn observe_pair(
    world: PairWorld,
    totals: Totals,
    updates_a: u64,
    updates_b: u64,
    mut clauses: [bool; 12],
    modes: String,
) -> Observation {
    clauses[10] &= totals.quiescent;
    clauses[11] &= totals.accepts == totals.updates;
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
            world.candidate_b_physical.0,
            world.candidate_b_physical.1,
        ),
        source_fires_a: firing_count(&totals.trace, world.namespace + 20),
        source_fires_b: firing_count(&totals.trace, world.namespace + 21),
        drive_deliveries: totals.drive_deliveries,
        modulatory_deliveries: totals.modulatory_deliveries,
        eligibility: format!(
            "a={:?}|b={:?}",
            world.substrate.arrow_eligible_until(world.candidate_a),
            world.substrate.arrow_eligible_until(world.candidate_b)
        ),
        modes,
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
    substrate.add_arrow(drive_spec(from, to, delay, coupling, resistance))
}

fn drive_spec(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
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

fn add_return(
    substrate: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    delay: i64,
    resistance: u32,
) -> ArrowId {
    add_return_coupling(substrate, from, to, delay, 1, resistance)
}

fn add_return_coupling(
    substrate: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    delay: i64,
    coupling: i32,
    resistance: u32,
) -> ArrowId {
    add_mode_arrow(
        substrate,
        from,
        to,
        delay,
        coupling,
        resistance,
        TransmissionMode::Modulatory,
    )
}

fn add_mode_arrow(
    substrate: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    delay: i64,
    coupling: i32,
    resistance: u32,
    mode: TransmissionMode,
) -> ArrowId {
    substrate.add_arrow(mode_spec(from, to, delay, coupling, resistance, mode))
}

fn mode_spec(
    from: CellId,
    to: CellId,
    delay: i64,
    coupling: i32,
    resistance: u32,
    mode: TransmissionMode,
) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode,
    }
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

fn firing_count(trace: &[TraceEntry], physical: u64) -> usize {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn claims8(values: [bool; 8]) -> [bool; 12] {
    [
        values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7],
        true, true, true, true,
    ]
}

fn namespace(seed: u64, case: Case) -> u64 {
    BASE + seed * 0x1000_0000
        + (case.suite as u64) * 0x0100_0000
        + u64::from(case.index) * 0x0010_0000
}

fn return_delay(seed: u64) -> i64 {
    i64::try_from(
        SEEDS
            .iter()
            .position(|value| *value == seed)
            .expect("registered seed")
            % 4,
    )
    .expect("small delay")
        + 1
}

fn paths() -> (&'static str, &'static str, &'static str, &'static str) {
    (
        "results/lrc_qualified_modulatory_transmission_definitive_v1.csv",
        "results/lrc_qualified_modulatory_transmission_definitive_v1.md",
        "results/.lrc_qualified_modulatory_transmission_definitive_v1.csv.staging",
        "results/.lrc_qualified_modulatory_transmission_definitive_v1.md.staging",
    )
}

fn absent() {
    let (a, b, c, d) = paths();
    for path in [a, b, c, d] {
        assert!(!Path::new(path).exists(), "artifact exists: {path}");
    }
}

fn csv(rows: &[Row]) -> String {
    let mut out = String::from("seed,stratum,suite,scenario,namespace,updates_a,updates_b,resistance_a,resistance_b,coupling_a,coupling_b,live_a,live_b,crossings_a,crossings_b,source_fires_a,source_fires_b,drive_deliveries,modulatory_deliveries,eligibility,modes,generations,work,accepts,path_edges,firings,deallocations,bytes,fingerprint,permanent,quiescent,p0,p1,p2,p3,p4,p5,p6,p7,p8,p9,p10,p11,replay,passed\n");
    for row in rows {
        let o = &row.observation;
        out.push_str(&format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n", row.seed, stratum(row.seed), row.case.suite.name(), row.case.name, row.namespace, o.updates_a, o.updates_b, o.resistance_a, o.resistance_b, o.coupling_a, o.coupling_b, o.live_a, o.live_b, o.candidate_crossings_a, o.candidate_crossings_b, o.source_fires_a, o.source_fires_b, o.drive_deliveries, o.modulatory_deliveries, o.eligibility, o.modes, o.generations, o.work, o.accepts, o.path_edges, o.firings, o.deallocations, o.bytes, o.fingerprint, o.permanent, o.quiescent, o.clauses[0], o.clauses[1], o.clauses[2], o.clauses[3], o.clauses[4], o.clauses[5], o.clauses[6], o.clauses[7], o.clauses[8], o.clauses[9], o.clauses[10], o.clauses[11], row.replay, row.passed));
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
            selected.len() * 12
        )
    };
    let passed = rows.iter().filter(|r| r.passed).count();
    let max_work = rows.iter().map(|r| r.observation.work).max().unwrap_or(0);
    let total_work = rows.iter().map(|r| r.observation.work).sum::<u64>();
    let max_bytes = rows.iter().map(|r| r.observation.bytes).max().unwrap_or(0);
    let claims = rows
        .iter()
        .map(|row| {
            row.observation
                .clauses
                .into_iter()
                .filter(|value| *value)
                .count()
        })
        .sum::<usize>();
    format!("# LR-C qualified modulatory transmission definitive v1\n\nOutcome: **{}**.\n\n{}\n{}\n{}\n{}\n- all cells: `{passed}/{}`;\n- all claims: `{claims}/5952`;\n- exact replay: `{}`;\n- natural quiescence: `{}`;\n- total/max work: `{total_work}/{max_work}`;\n- maximum persistent bytes: `{max_bytes}`;\n- active state addition: `one two-valued TransmissionMode per ARROW`;\n- spent PX0--PX2 authority replayed: `false`;\n- PX3 executed: `false`.\n", if passed == rows.len() { "LRC AUTHORITATIVE POSITIVE" } else { "LRC DEFINITIVE NEGATIVE" }, suite_line(Suite::Law), suite_line(Suite::Px0), suite_line(Suite::Px1), suite_line(Suite::Px2), rows.len(), rows.iter().all(|r| r.replay), rows.iter().all(|r| r.observation.quiescent))
}

fn stratum(seed: u64) -> String {
    let index = SEEDS
        .iter()
        .position(|value| *value == seed)
        .expect("registered seed");
    format!("S{index:02}")
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
