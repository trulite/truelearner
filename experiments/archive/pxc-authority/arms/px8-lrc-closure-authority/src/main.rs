#![forbid(unsafe_code)]

use px7_lrc_arrival::{
    Activity, Arrival, Body, Form, BOUNDARY_A, BOUNDARY_B, BOUNDARY_C, BOUNDARY_D, DOWNSTREAM_ONE,
    DOWNSTREAM_ZERO, INNER_ONE, INNER_ZERO, OUTWARD_SITE, RETURN_SITE,
};
use px8_lrc_physical_closure::{CompactBody, CompactForm, Layout, Reading, RecursiveBody};
use std::collections::BTreeSet;
use std::env;
use std::fmt::Write as _;
use std::fs::{rename, OpenOptions};
use std::io::Write as _;
use std::path::Path;
use std::process::Command;

const LAW: &str = "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10";
const PX4_SOURCE: &str = "a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71";
const PX7_SOURCE: &str = "d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e";
const PX7_CSV: &str = "96baddb76ef1c58dc0173f14a8fabded8c6237e73b729030e4698cf2fe300137";
const PX7_REPORT: &str = "9b23ad3bd34050e1f13b8638b05baa5b9e29adc6cef5208db60dde36b7b058dc";
const PX7_MANIFEST: &str = "db4758baa5aeba36a87251f7d2ccb85cd2215f9489a1189eae4fd9d6408001c2";
const PX8_SOURCE: &str = "8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f";
const PROTOCOL: &str = "510915f264be35318f0f84a62b2277335984458912b431f51da90c7aa1086f7c";
const WORK_CEILING: u64 = 20_000;
const BYTE_CEILING: usize = 8_192;
const CSV: &str = "results/px8_lrc_closure_authority_v1.csv";
const REPORT: &str = "results/px8_lrc_closure_authority_v1.md";
const CSV_STAGE: &str = "results/px8_lrc_closure_authority_v1.csv.staging";
const REPORT_STAGE: &str = "results/px8_lrc_closure_authority_v1.md.staging";

const CASES: [Case; 16] = [
    Case::new(861_001, false, false, 0),
    Case::new(861_002, false, false, 137),
    Case::new(861_003, false, false, 274),
    Case::new(861_004, false, false, 411),
    Case::new(861_005, true, false, 0),
    Case::new(861_006, true, false, 137),
    Case::new(861_007, true, false, 274),
    Case::new(861_008, true, false, 411),
    Case::new(861_009, false, true, 0),
    Case::new(861_010, false, true, 137),
    Case::new(861_011, false, true, 274),
    Case::new(861_012, false, true, 411),
    Case::new(861_013, true, true, 0),
    Case::new(861_014, true, true, 137),
    Case::new(861_015, true, true, 274),
    Case::new(861_016, true, true, 411),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Case {
    number: u64,
    reverse: bool,
    reflect: bool,
    twist: u64,
}

impl Case {
    const fn new(number: u64, reverse: bool, reflect: bool, twist: u64) -> Self {
        Self {
            number,
            reverse,
            reflect,
            twist,
        }
    }

    fn layout(self) -> Layout {
        Layout {
            namespace: self.number << 32,
            reverse: self.reverse,
            reflect: self.reflect,
            twist: self.twist,
            outward_resistance: 100,
        }
    }

    fn altered(self) -> bool {
        self.twist == 137 || self.twist == 411
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ArrivalObservation {
    inner: [usize; 2],
    downstream: [usize; 2],
    link_crossings: [usize; 2],
    outward: usize,
    modulation: u64,
    updates: u64,
    work: u64,
    quiet: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cumulative {
    passed: bool,
    first: ArrivalObservation,
    second: ArrivalObservation,
    heldout: ArrivalObservation,
    couplings: [i32; 2],
    resistance: [u32; 2],
    bytes: usize,
    memory_stable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Trial {
    case: Case,
    formation: Reading,
    completed: Reading,
    incomplete: Reading,
    blocked: Reading,
    stale: Reading,
    duplicate_recursive: Reading,
    zero_length: Reading,
    duplicate_physical: Reading,
    open: Reading,
    aged: Reading,
    branch: Reading,
    cycle: Reading,
    pause_stable: bool,
    resume_equal: bool,
    cumulative: Cumulative,
    maximum_work: u64,
    maximum_bytes: usize,
    memory_stable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    trial: Trial,
    claims: [bool; 14],
    exact_replay: bool,
    passed: bool,
}

#[derive(Clone, Copy)]
enum AuthorityPermission {
    Granted,
}

fn main() {
    audit();
    surface();
    preflight();
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [mode] if mode == "--authority-preflight" => {
            println!("PX8_LRC_CLOSURE_AUTHORITY_PREFLIGHT_OK");
        }
        [mode] if mode == "--authority-v1" => authority(AuthorityPermission::Granted),
        _ => std::process::exit(2),
    }
}

fn audit() {
    for (path, digest) in [
        ("crates/lr1-modulatory-physical-return/src/lib.rs", LAW),
        ("arms/px4-lrc-lifetime/src/lib.rs", PX4_SOURCE),
        ("crates/px7-lrc-arrival/src/lib.rs", PX7_SOURCE),
        ("results/px7_lrc_arrival_authority_v1.csv", PX7_CSV),
        ("results/px7_lrc_arrival_authority_v1.md", PX7_REPORT),
        (
            "experiments/pxc_active_surface_manifest_v5.csv",
            PX7_MANIFEST,
        ),
        (
            "experiments/px8_lrc_cumulative_closure_authority_protocol_v1.md",
            PROTOCOL,
        ),
        ("arms/px8-lrc-physical-closure/src/lib.rs", PX8_SOURCE),
    ] {
        assert_eq!(sha(path), digest, "frozen input changed: {path}");
    }
}

fn surface() {
    assert_eq!(CASES.len(), 16);
    assert_eq!(
        CASES.into_iter().collect::<BTreeSet<_>>().len(),
        CASES.len()
    );
    assert_eq!(CASES.first().map(|case| case.number), Some(861_001));
    assert_eq!(CASES.last().map(|case| case.number), Some(861_016));
    assert_eq!(CASES.iter().filter(|case| case.reverse).count(), 8);
    assert_eq!(CASES.iter().filter(|case| case.reflect).count(), 8);
    for reverse in [false, true] {
        for reflect in [false, true] {
            assert_eq!(
                CASES
                    .iter()
                    .filter(|case| case.reverse == reverse && case.reflect == reflect)
                    .count(),
                4
            );
        }
    }
    for twist in [0, 137, 274, 411] {
        assert_eq!(CASES.iter().filter(|case| case.twist == twist).count(), 4);
    }
}

fn preflight() {
    absent(&[CSV, REPORT, CSV_STAGE, REPORT_STAGE]);
}

fn authority(_permission: AuthorityPermission) {
    eprintln!("PX8_LRC_CLOSURE_AUTHORITY_V1_EVIDENCE_SPENT");
    let rows = CASES.into_iter().map(replay).collect::<Vec<_>>();
    let globals = global_claims(&rows);
    assert!(rows.iter().all(|row| row.passed), "authority row failed");
    assert!(
        globals.into_iter().all(|claim| claim),
        "global clause failed"
    );
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(REPORT_STAGE, REPORT, &report(&rows, globals));
    println!("PX8_LRC_CLOSURE_AUTHORITY_PASS rows=16/16 clauses=230/230");
}

fn replay(case: Case) -> Row {
    let first = run(case);
    let second = run(case);
    let exact = first == second;
    let mut row = row(first);
    row.exact_replay = exact;
    row.claims[13] = exact;
    row.passed = row.claims.into_iter().all(|claim| claim);
    row
}

fn run(case: Case) -> Trial {
    let layout = case.layout();
    let namespace = layout.namespace;
    let mut learned = RecursiveBody::new(layout);
    let formation = learned.learn_twice();
    let trained_bytes = learned.persistent_bytes();
    let pause_a = learned.fingerprints();
    let pause_b = learned.fingerprints();

    let mut uninterrupted = learned.clone();
    let completed = learned.reuse([true; 4], 61, false);
    let uninterrupted_completed = uninterrupted.reuse([true; 4], 61, false);

    let mut incomplete_body = uninterrupted.clone();
    let incomplete = incomplete_body.reuse([true, true, true, false], 70, false);

    let mut duplicate_body = RecursiveBody::new(layout);
    duplicate_body.learn_twice();
    let duplicate_recursive = duplicate_body.reuse([true; 4], 61, true);

    let mut blocked_body = RecursiveBody::new(Layout {
        outward_resistance: 0,
        ..layout
    });
    blocked_body.learn_twice();
    let blocked = blocked_body.reuse([true; 4], 61, false);

    let mut stale_body = RecursiveBody::new(layout);
    stale_body.learn_once_then_age();
    let stale = stale_body.reuse([true; 4], 111, false);

    let mut direct = CompactBody::new(namespace + 10_000, CompactForm::Direct, case.reflect);
    let zero_length = direct.flow(1, 0);
    let mut twin = CompactBody::new(namespace + 20_000, CompactForm::Direct, case.reflect);
    let duplicate_physical = twin.flow(2, 0);
    let mut open_body = CompactBody::new(namespace + 30_000, CompactForm::Open, case.reflect);
    let open = open_body.flow(1, 0);
    let mut aged_body = CompactBody::new(namespace + 40_000, CompactForm::Aged, case.reflect);
    let aged = aged_body.flow(1, 10);
    let mut branch_body = CompactBody::new(namespace + 50_000, CompactForm::Fork, case.reflect);
    let branch = branch_body.flow(1, 0);
    let mut cycle_body = CompactBody::new(namespace + 60_000, CompactForm::Ring, case.reflect);
    let cycle = cycle_body.flow(1, 0);

    let cumulative = cumulative_conformance(case);
    let memory_stable = [
        learned.persistent_bytes(),
        uninterrupted.persistent_bytes(),
        incomplete_body.persistent_bytes(),
        duplicate_body.persistent_bytes(),
        blocked_body.persistent_bytes(),
        stale_body.persistent_bytes(),
    ]
    .into_iter()
    .all(|bytes| bytes == trained_bytes)
        && cumulative.memory_stable;
    let maximum_work = readings_values(
        formation,
        completed,
        incomplete,
        blocked,
        stale,
        duplicate_recursive,
        zero_length,
        duplicate_physical,
        open,
        aged,
        branch,
        cycle,
    )
    .into_iter()
    .map(|reading| reading.work)
    .chain([
        cumulative.first.work,
        cumulative.second.work,
        cumulative.heldout.work,
    ])
    .max()
    .unwrap_or(0);
    let maximum_bytes = trained_bytes.max(cumulative.bytes);

    Trial {
        case,
        formation,
        completed,
        incomplete,
        blocked,
        stale,
        duplicate_recursive,
        zero_length,
        duplicate_physical,
        open,
        aged,
        branch,
        cycle,
        pause_stable: pause_a == pause_b,
        resume_equal: completed == uninterrupted_completed,
        cumulative,
        maximum_work,
        maximum_bytes,
        memory_stable,
    }
}

fn row(trial: Trial) -> Row {
    let quiet = readings(&trial).into_iter().all(|reading| reading.quiet)
        && trial.cumulative.first.quiet
        && trial.cumulative.second.quiet
        && trial.cumulative.heldout.quiet;
    let claims = [
        trial.formation.updates > 0 && trial.formation.inward_modulation > 0,
        trial.completed.outward == 1,
        trial.completed.inward_modulation == 1 && trial.completed.updates == 1,
        trial.incomplete.outward == 0 && trial.blocked.outward == 0 && trial.stale.outward == 0,
        trial.open.outward == 0 && trial.aged.outward == 0,
        trial.zero_length.outward == 1,
        trial.duplicate_physical.outward == 1 && trial.duplicate_recursive.outward == 1,
        trial.branch.outward == 0 && trial.cycle.outward == 0,
        trial.pause_stable && trial.resume_equal,
        quiet,
        trial.maximum_work <= WORK_CEILING,
        trial.maximum_bytes <= BYTE_CEILING && trial.memory_stable,
        trial.cumulative.passed,
        false,
    ];
    Row {
        trial,
        claims,
        exact_replay: false,
        passed: false,
    }
}

fn cumulative_conformance(case: Case) -> Cumulative {
    let form = Form {
        namespace: (case.number + 1_000_000) << 32,
        reverse_construction: case.reverse,
        reflected_positions: case.reflect,
        altered_pairing: case.altered(),
    };
    let mut body = Body::new(form);
    let first_arrivals = maturation(&body, 0, case.altered());
    let first_activity = body.participate(first_arrivals);
    let first = observe(&body, first_activity);
    let second_arrivals = maturation(&body, 10, case.altered());
    let second_activity = body.participate(second_arrivals);
    let second = observe(&body, second_activity);
    let couplings = body.link_couplings();
    let resistance = body.link_resistances();
    let bytes = body.persistent_bytes();
    let heldout_arrivals = boundary(&body, 20, case.altered());
    let heldout_activity = body.participate(heldout_arrivals);
    let heldout = observe(&body, heldout_activity);
    let memory_stable = body.persistent_bytes() == bytes;
    let passed = first.modulation == 2
        && first.updates == 2
        && second.modulation == 2
        && second.updates == 2
        && couplings == [2, 2]
        && resistance == [6, 6]
        && heldout.inner == [1, 1]
        && heldout.downstream == [1, 1]
        && heldout.link_crossings == [1, 1]
        && heldout.outward == 1
        && heldout.modulation == 0
        && heldout.updates == 0
        && first.quiet
        && second.quiet
        && heldout.quiet
        && memory_stable;
    Cumulative {
        passed,
        first,
        second,
        heldout,
        couplings,
        resistance,
        bytes,
        memory_stable,
    }
}

fn observe(body: &Body, activity: Activity) -> ArrivalObservation {
    let execution = &activity.execution;
    let fires = |site| {
        let physical = body.physical(site);
        execution
            .trace
            .iter()
            .filter(|entry| entry.target_physical == physical && entry.fired)
            .count()
    };
    let crossing = |from, to| {
        let from = body.physical(from);
        let to = body.physical(to);
        execution
            .crossings
            .iter()
            .filter(|entry| entry.from_physical == from && entry.to_physical == to)
            .count()
    };
    ArrivalObservation {
        inner: [INNER_ZERO, INNER_ONE].map(fires),
        downstream: [DOWNSTREAM_ZERO, DOWNSTREAM_ONE].map(fires),
        link_crossings: [
            crossing(INNER_ZERO, DOWNSTREAM_ZERO),
            crossing(INNER_ONE, DOWNSTREAM_ONE),
        ],
        outward: crossing(DOWNSTREAM_ONE, OUTWARD_SITE),
        modulation: execution.work.modulatory_deliveries,
        updates: execution.work.local_return_updates,
        work: activity.work(),
        quiet: activity.naturally_quiescent(),
    }
}

fn boundary(body: &Body, begin: i64, altered: bool) -> [Arrival; 3] {
    let paired = if altered { BOUNDARY_D } else { BOUNDARY_B };
    [
        at(body, BOUNDARY_A, begin, 10),
        at(body, paired, begin, 11),
        at(body, BOUNDARY_C, begin + 2, 12),
    ]
}

fn maturation(body: &Body, begin: i64, altered: bool) -> [Arrival; 7] {
    let [a, paired, c] = boundary(body, begin, altered);
    [
        a,
        paired,
        c,
        at(body, DOWNSTREAM_ZERO, begin + 2, 20),
        at(body, RETURN_SITE, begin + 3, 21),
        at(body, DOWNSTREAM_ONE, begin + 4, 22),
        at(body, RETURN_SITE, begin + 5, 23),
    ]
}

fn at(body: &Body, site: usize, tick: i64, phase: i32) -> Arrival {
    Arrival {
        tick,
        phase,
        origin: body.physical(site) + 18_000_000 + phase as u64,
        position: body.coordinate(site),
        impulse: 1,
    }
}

fn readings(trial: &Trial) -> [Reading; 12] {
    readings_values(
        trial.formation,
        trial.completed,
        trial.incomplete,
        trial.blocked,
        trial.stale,
        trial.duplicate_recursive,
        trial.zero_length,
        trial.duplicate_physical,
        trial.open,
        trial.aged,
        trial.branch,
        trial.cycle,
    )
}

#[allow(clippy::too_many_arguments)]
fn readings_values(
    formation: Reading,
    completed: Reading,
    incomplete: Reading,
    blocked: Reading,
    stale: Reading,
    duplicate_recursive: Reading,
    zero_length: Reading,
    duplicate_physical: Reading,
    open: Reading,
    aged: Reading,
    branch: Reading,
    cycle: Reading,
) -> [Reading; 12] {
    [
        formation,
        completed,
        incomplete,
        blocked,
        stale,
        duplicate_recursive,
        zero_length,
        duplicate_physical,
        open,
        aged,
        branch,
        cycle,
    ]
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "root,namespace,reverse,reflect,twist,formation_updates,formation_modulation,completed_outward,completed_return,completed_updates,incomplete_outward,blocked_outward,stale_outward,open_outward,aged_outward,zero_length_outward,duplicate_physical_outward,duplicate_recursive_outward,branch_outward,cycle_outward,pause_stable,resume_equal,cumulative,max_work,max_bytes,memory_stable,quiet,exact_replay,claims,passed\n",
    );
    for row in rows {
        let trial = &row.trial;
        let fields = [
            trial.case.number.to_string(),
            (trial.case.number << 32).to_string(),
            trial.case.reverse.to_string(),
            trial.case.reflect.to_string(),
            trial.case.twist.to_string(),
            trial.formation.updates.to_string(),
            trial.formation.inward_modulation.to_string(),
            trial.completed.outward.to_string(),
            trial.completed.inward_modulation.to_string(),
            trial.completed.updates.to_string(),
            trial.incomplete.outward.to_string(),
            trial.blocked.outward.to_string(),
            trial.stale.outward.to_string(),
            trial.open.outward.to_string(),
            trial.aged.outward.to_string(),
            trial.zero_length.outward.to_string(),
            trial.duplicate_physical.outward.to_string(),
            trial.duplicate_recursive.outward.to_string(),
            trial.branch.outward.to_string(),
            trial.cycle.outward.to_string(),
            trial.pause_stable.to_string(),
            trial.resume_equal.to_string(),
            cumulative_signature(trial.cumulative),
            trial.maximum_work.to_string(),
            trial.maximum_bytes.to_string(),
            trial.memory_stable.to_string(),
            all_quiet(trial).to_string(),
            row.exact_replay.to_string(),
            join_bool(&row.claims),
            row.passed.to_string(),
        ];
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

fn global_claims(rows: &[Row]) -> [bool; 6] {
    let roots = rows
        .iter()
        .map(|row| row.trial.case.number)
        .collect::<BTreeSet<_>>();
    let balanced = [false, true].into_iter().all(|reverse| {
        [false, true].into_iter().all(|reflect| {
            rows.iter()
                .filter(|row| {
                    row.trial.case.reverse == reverse && row.trial.case.reflect == reflect
                })
                .count()
                == 4
        })
    }) && [0, 137, 274, 411].into_iter().all(|twist| {
        rows.iter()
            .filter(|row| row.trial.case.twist == twist)
            .count()
            == 4
    });
    let identities = rows
        .iter()
        .flat_map(|row| identity_namespaces(row.trial.case))
        .collect::<BTreeSet<_>>();
    [
        roots.len() == 16 && roots.iter().copied().eq(861_001..=861_016),
        balanced,
        identities.len() == 128,
        rows.iter().all(|row| row.claims[12]),
        rows.iter()
            .all(|row| row.claims[9] && row.claims[10] && row.claims[11] && row.claims[13]),
        rows.len() == 16 && rows.iter().all(|row| row.claims.len() == 14),
    ]
}

fn identity_namespaces(case: Case) -> [u64; 8] {
    let base = case.number << 32;
    [
        base,
        base + 10_000,
        base + 20_000,
        base + 30_000,
        base + 40_000,
        base + 50_000,
        base + 60_000,
        (case.number + 1_000_000) << 32,
    ]
}

fn report(rows: &[Row], globals: [bool; 6]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let row_clauses = rows
        .iter()
        .map(|row| row.claims.into_iter().filter(|claim| *claim).count())
        .sum::<usize>();
    let global_clauses = globals.into_iter().filter(|claim| *claim).count();
    let mut output = String::new();
    writeln!(
        output,
        "# PX8 LR-C cumulative physical-closure/emission authority v1\n"
    )
    .unwrap();
    writeln!(
        output,
        "Outcome: **{}**.\n",
        if passed == 16 && global_clauses == 6 {
            "DEFINITIVE POSITIVE"
        } else {
            "NEGATIVE"
        }
    )
    .unwrap();
    writeln!(output, "- rows: `{passed}/{}`;", rows.len()).unwrap();
    writeln!(output, "- row clauses: `{row_clauses}/224`;").unwrap();
    writeln!(output, "- global clauses: `{global_clauses}/6`;").unwrap();
    writeln!(
        output,
        "- total clauses: `{}/230`;",
        row_clauses + global_clauses
    )
    .unwrap();
    writeln!(
        output,
        "- completed exactly-once outward crossing: `{}`;",
        rows.iter().all(|row| row.trial.completed.outward == 1)
    )
    .unwrap();
    writeln!(
        output,
        "- completed downstream return/modulation: `{}`;",
        rows.iter().all(|row| row.claims[2])
    )
    .unwrap();
    writeln!(
        output,
        "- incomplete/blocked/stale/open/aged silence: `{}`;",
        rows.iter().all(|row| row.claims[3] && row.claims[4])
    )
    .unwrap();
    writeln!(
        output,
        "- zero-length/duplicate exactly once: `{}`;",
        rows.iter().all(|row| row.claims[5] && row.claims[6])
    )
    .unwrap();
    writeln!(
        output,
        "- branch/cycle silence: `{}`;",
        rows.iter().all(|row| row.claims[7])
    )
    .unwrap();
    writeln!(
        output,
        "- pause/resume exact: `{}`;",
        rows.iter().all(|row| row.claims[8])
    )
    .unwrap();
    writeln!(
        output,
        "- cumulative PX0--PX7+LR-C conformance: `{}`;",
        rows.iter().all(|row| row.claims[12])
    )
    .unwrap();
    writeln!(
        output,
        "- natural quiescence: `{}`;",
        rows.iter().all(|row| row.claims[9])
    )
    .unwrap();
    writeln!(
        output,
        "- maximum field work: `{}` (ceiling `{WORK_CEILING}`);",
        rows.iter()
            .map(|row| row.trial.maximum_work)
            .max()
            .unwrap_or(0)
    )
    .unwrap();
    writeln!(
        output,
        "- maximum persistent bytes: `{}` (ceiling `{BYTE_CEILING}`);",
        rows.iter()
            .map(|row| row.trial.maximum_bytes)
            .max()
            .unwrap_or(0)
    )
    .unwrap();
    writeln!(
        output,
        "- persistent memory stable: `{}`;",
        rows.iter().all(|row| row.trial.memory_stable)
    )
    .unwrap();
    writeln!(
        output,
        "- exact duplicate-state replay: `{}`;",
        rows.iter().all(|row| row.exact_replay)
    )
    .unwrap();
    writeln!(output, "- explicit cleanup calls: `0`;").unwrap();
    writeln!(
        output,
        "- terminal/Episode/Query/begin/reset/finish surface: `false`;"
    )
    .unwrap();
    writeln!(output, "- new organism law: `false`;").unwrap();
    writeln!(
        output,
        "- final PX-C continuous-organism authority: `false`."
    )
    .unwrap();
    output
}

fn all_quiet(trial: &Trial) -> bool {
    readings(trial).into_iter().all(|reading| reading.quiet)
        && trial.cumulative.first.quiet
        && trial.cumulative.second.quiet
        && trial.cumulative.heldout.quiet
}

fn cumulative_signature(value: Cumulative) -> String {
    format!(
        "pass={}~first_m={}~first_u={}~second_m={}~second_u={}~heldout_o={}~coupling={}~resistance={}~bytes={}~stable={}",
        value.passed,
        value.first.modulation,
        value.first.updates,
        value.second.modulation,
        value.second.updates,
        value.heldout.outward,
        join_i32(&value.couplings),
        join_u32(&value.resistance),
        value.bytes,
        value.memory_stable
    )
}

fn join_bool(values: &[bool]) -> String {
    values
        .iter()
        .map(bool::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_i32(values: &[i32]) -> String {
    values
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_u32(values: &[u32]) -> String {
    values
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn sha(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum");
    assert!(output.status.success(), "sha256sum failed: {path}");
    String::from_utf8(output.stdout)
        .expect("utf8")
        .split_whitespace()
        .next()
        .expect("digest")
        .to_owned()
}

fn absent(paths: &[&str]) {
    for path in paths {
        assert!(!Path::new(path).exists(), "artifact exists: {path}");
    }
}

fn publish(stage: &str, destination: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(stage)
        .expect("create staging artifact");
    file.write_all(content.as_bytes()).expect("write artifact");
    file.sync_all().expect("sync artifact");
    rename(stage, destination).expect("publish artifact");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_definition_is_frozen() {
        surface();
        assert_eq!(CASES.len(), 16);
        assert_eq!(WORK_CEILING, 20_000);
        assert_eq!(BYTE_CEILING, 8_192);
        assert_eq!(identity_namespaces(CASES[0]).len(), 8);
    }
}
