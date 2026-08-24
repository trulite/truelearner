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
const PX8_SOURCE: &str = "8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f";
const V2_EVALUATOR: &str = "e1a830e15c898b113f295d74e22f6dee1d144bd43ee1aa177d4a7c0ef075043c";
const V2_PROTOCOL: &str = "a47866460ecc4504ee713e0b425d049e7816f48e4aa18bceeb0a1705dcbc5328";
const V2_NEGATIVE: &str = "2a527fecb9906e4bdf4bce703a760e646439f205ec2eee6a8288a38de6cc1620";
const PROTOCOL: &str = "40bafda8f6caa2cf3bce08fbdd34dfe9802aa4c392b6ebb85ab646fab752fa2a";
const WORK_CEILING: u64 = 20_000;
const BYTE_CEILING: usize = 8_192;
const CSV: &str = "results/px8_lrc_closure_authority_v2_negative_diagnostic.csv";
const REPORT: &str = "results/px8_lrc_closure_authority_v2_negative_diagnostic.md";
const CSV_STAGE: &str = "results/px8_lrc_closure_authority_v2_negative_diagnostic.csv.staging";
const REPORT_STAGE: &str = "results/px8_lrc_closure_authority_v2_negative_diagnostic.md.staging";
const SCHEDULE: &str = "formation=learn_twice;complete=all@61;incomplete=omit4@70;blocked=resistance0@61;stale=once_age@111;compact=direct_open_fork_ring@0+aged@10;px7=train@0+10+heldout@20";

const CASES: [Case; 16] = [
    Case::new(864_001, false, false, 0),
    Case::new(864_002, false, false, 137),
    Case::new(864_003, false, false, 274),
    Case::new(864_004, false, false, 411),
    Case::new(864_005, true, false, 0),
    Case::new(864_006, true, false, 137),
    Case::new(864_007, true, false, 274),
    Case::new(864_008, true, false, 411),
    Case::new(864_009, false, true, 0),
    Case::new(864_010, false, true, 137),
    Case::new(864_011, false, true, 274),
    Case::new(864_012, false, true, 411),
    Case::new(864_013, true, true, 0),
    Case::new(864_014, true, true, 137),
    Case::new(864_015, true, true, 274),
    Case::new(864_016, true, true, 411),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Case {
    root: u64,
    reverse: bool,
    reflect: bool,
    twist: u64,
}

impl Case {
    const fn new(root: u64, reverse: bool, reflect: bool, twist: u64) -> Self {
        Self {
            root,
            reverse,
            reflect,
            twist,
        }
    }

    fn layout(self) -> Layout {
        Layout {
            namespace: self.root << 32,
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
    links: [usize; 2],
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
    before: usize,
    after: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MemoryPairs {
    primary: [usize; 2],
    uninterrupted: [usize; 2],
    incomplete: [usize; 2],
    duplicate: [usize; 2],
    blocked: [usize; 2],
    stale: [usize; 2],
    cumulative: [usize; 2],
}

impl MemoryPairs {
    fn values(self) -> [[usize; 2]; 7] {
        [
            self.primary,
            self.uninterrupted,
            self.incomplete,
            self.duplicate,
            self.blocked,
            self.stale,
            self.cumulative,
        ]
    }

    fn stable(self) -> bool {
        self.values().into_iter().all(|pair| pair[0] == pair[1])
    }

    fn maximum(self) -> usize {
        self.values().into_iter().flatten().max().unwrap_or(0)
    }
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
    memory: MemoryPairs,
    maximum_work: u64,
    maximum_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DiagnosticRow {
    trial: Trial,
    replay: bool,
    first_divergence: String,
}

struct Evaluation {
    name: &'static str,
    expected: &'static str,
    actual: String,
    passed: bool,
}

fn main() {
    audit();
    surface();
    absent(&[CSV, REPORT, CSV_STAGE, REPORT_STAGE]);
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [mode] if mode == "--v2-diagnostic-v1" => diagnostic(),
        _ => std::process::exit(2),
    }
}

fn audit() {
    for (path, digest) in [
        ("crates/lr1-modulatory-physical-return/src/lib.rs", LAW),
        ("arms/px4-lrc-lifetime/src/lib.rs", PX4_SOURCE),
        ("crates/px7-lrc-arrival/src/lib.rs", PX7_SOURCE),
        ("arms/px8-lrc-physical-closure/src/lib.rs", PX8_SOURCE),
        (
            "arms/px8-lrc-closure-authority-v2/src/main.rs",
            V2_EVALUATOR,
        ),
        (
            "experiments/px8_lrc_cumulative_closure_authority_v2_protocol_v1.md",
            V2_PROTOCOL,
        ),
        (
            "experiments/px8_lrc_cumulative_closure_authority_v2_negative_v1.md",
            V2_NEGATIVE,
        ),
        (
            "experiments/px8_lrc_closure_authority_v2_negative_diagnostic_protocol_v1.md",
            PROTOCOL,
        ),
    ] {
        assert_eq!(sha(path), digest, "frozen input changed: {path}");
    }
}

fn surface() {
    assert_eq!(CASES.len(), 16);
    assert_eq!(CASES.into_iter().collect::<BTreeSet<_>>().len(), 16);
    assert_eq!(CASES.first().map(|case| case.root), Some(864_001));
    assert_eq!(CASES.last().map(|case| case.root), Some(864_016));
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
    assert_eq!(
        CASES
            .into_iter()
            .flat_map(identity_namespaces)
            .collect::<BTreeSet<_>>()
            .len(),
        128
    );
}

fn diagnostic() {
    eprintln!("PX8_LRC_CLOSURE_AUTHORITY_V2_NEGATIVE_DIAGNOSTIC_SPENT");
    let rows = CASES.into_iter().map(reconstruct).collect::<Vec<_>>();
    let records = rows.iter().map(|row| evaluations(row).len()).sum::<usize>();
    let csv_text = csv(&rows);
    let report_text = report(&rows);
    assert_eq!(rows.len(), 16, "diagnostic roots incomplete");
    assert_eq!(records, 224, "diagnostic clauses incomplete");
    let failures = rows
        .iter()
        .flat_map(evaluations)
        .filter(|item| !item.passed)
        .count();
    publish(CSV_STAGE, CSV, &csv_text);
    publish(REPORT_STAGE, REPORT, &report_text);
    println!(
        "PX8_LRC_CLOSURE_AUTHORITY_V2_DIAGNOSTIC_COMPLETE roots=16 clauses=224 failures={failures} authority=false"
    );
}

fn reconstruct(case: Case) -> DiagnosticRow {
    let first = run(case);
    let second = run(case);
    DiagnosticRow {
        replay: first == second,
        first_divergence: first_divergence(&first, &second),
        trial: first,
    }
}

fn run(case: Case) -> Trial {
    let layout = case.layout();
    let namespace = layout.namespace;
    let mut learned = RecursiveBody::new(layout);
    let formation = learned.learn_twice();
    let primary_before = learned.persistent_bytes();
    let pause_a = learned.fingerprints();
    let pause_b = learned.fingerprints();

    let mut uninterrupted = learned.clone();
    let uninterrupted_before = uninterrupted.persistent_bytes();
    let completed = learned.reuse([true; 4], 61, false);
    let primary_after = learned.persistent_bytes();
    let uninterrupted_completed = uninterrupted.reuse([true; 4], 61, false);
    let uninterrupted_after = uninterrupted.persistent_bytes();

    let mut incomplete_body = uninterrupted.clone();
    let incomplete_before = incomplete_body.persistent_bytes();
    let incomplete = incomplete_body.reuse([true, true, true, false], 70, false);
    let incomplete_after = incomplete_body.persistent_bytes();

    let mut duplicate_body = RecursiveBody::new(layout);
    duplicate_body.learn_twice();
    let duplicate_before = duplicate_body.persistent_bytes();
    let duplicate_recursive = duplicate_body.reuse([true; 4], 61, true);
    let duplicate_after = duplicate_body.persistent_bytes();

    let mut blocked_body = RecursiveBody::new(Layout {
        outward_resistance: 0,
        ..layout
    });
    blocked_body.learn_twice();
    let blocked_before = blocked_body.persistent_bytes();
    let blocked = blocked_body.reuse([true; 4], 61, false);
    let blocked_after = blocked_body.persistent_bytes();

    let mut stale_body = RecursiveBody::new(layout);
    stale_body.learn_once_then_age();
    let stale_before = stale_body.persistent_bytes();
    let stale = stale_body.reuse([true; 4], 111, false);
    let stale_after = stale_body.persistent_bytes();

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
    let memory = MemoryPairs {
        primary: [primary_before, primary_after],
        uninterrupted: [uninterrupted_before, uninterrupted_after],
        incomplete: [incomplete_before, incomplete_after],
        duplicate: [duplicate_before, duplicate_after],
        blocked: [blocked_before, blocked_after],
        stale: [stale_before, stale_after],
        cumulative: [cumulative.before, cumulative.after],
    };
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
    .map(|value| value.work)
    .chain([
        cumulative.first.work,
        cumulative.second.work,
        cumulative.heldout.work,
    ])
    .max()
    .unwrap_or(0);

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
        memory,
        maximum_work,
        maximum_bytes: memory.maximum(),
    }
}

fn cumulative_conformance(case: Case) -> Cumulative {
    let form = Form {
        namespace: (case.root + 1_000_000) << 32,
        reverse_construction: case.reverse,
        reflected_positions: case.reflect,
        altered_pairing: case.altered(),
    };
    let mut body = Body::new(form);
    let arrivals = maturation(&body, 0, case.altered());
    let activity = body.participate(arrivals);
    let first = observe(&body, activity);
    let arrivals = maturation(&body, 10, case.altered());
    let activity = body.participate(arrivals);
    let second = observe(&body, activity);
    let couplings = body.link_couplings();
    let resistance = body.link_resistances();
    let before = body.persistent_bytes();
    let arrivals = boundary(&body, 20, case.altered());
    let activity = body.participate(arrivals);
    let heldout = observe(&body, activity);
    let after = body.persistent_bytes();
    let passed = first.modulation == 2
        && first.updates == 2
        && second.modulation == 2
        && second.updates == 2
        && couplings == [2, 2]
        && resistance == [6, 6]
        && heldout.inner == [1, 1]
        && heldout.downstream == [1, 1]
        && heldout.links == [1, 1]
        && heldout.outward == 1
        && heldout.modulation == 0
        && heldout.updates == 0
        && first.quiet
        && second.quiet
        && heldout.quiet
        && before == after;
    Cumulative {
        passed,
        first,
        second,
        heldout,
        couplings,
        resistance,
        before,
        after,
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
        links: [
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
        origin: body.physical(site) + 20_000_000 + phase as u64,
        position: body.coordinate(site),
        impulse: 1,
    }
}

fn evaluations(row: &DiagnosticRow) -> Vec<Evaluation> {
    let trial = &row.trial;
    vec![
        evaluation(
            "formation_modulation_and_updates",
            "formation.updates>0 && formation.inward_modulation>0",
            reading_signature(trial.formation),
            trial.formation.updates > 0 && trial.formation.inward_modulation > 0,
        ),
        evaluation(
            "completed_exactly_once",
            "completed.outward==1",
            reading_signature(trial.completed),
            trial.completed.outward == 1,
        ),
        evaluation(
            "completed_return_and_update",
            "completed.return==1 && completed.updates==1",
            reading_signature(trial.completed),
            trial.completed.inward_modulation == 1 && trial.completed.updates == 1,
        ),
        evaluation(
            "incomplete_blocked_stale_silence",
            "incomplete.outward==0 && blocked.outward==0 && stale.outward==0",
            format!(
                "incomplete={}~blocked={}~stale={}",
                reading_signature(trial.incomplete),
                reading_signature(trial.blocked),
                reading_signature(trial.stale)
            ),
            trial.incomplete.outward == 0 && trial.blocked.outward == 0 && trial.stale.outward == 0,
        ),
        evaluation(
            "open_aged_silence",
            "open.outward==0 && aged.outward==0",
            format!(
                "open={}~aged={}",
                reading_signature(trial.open),
                reading_signature(trial.aged)
            ),
            trial.open.outward == 0 && trial.aged.outward == 0,
        ),
        evaluation(
            "zero_length_exactly_once",
            "zero_length.outward==1",
            reading_signature(trial.zero_length),
            trial.zero_length.outward == 1,
        ),
        evaluation(
            "duplicates_exactly_once",
            "duplicate_physical.outward==1 && duplicate_recursive.outward==1",
            format!(
                "physical={}~recursive={}",
                reading_signature(trial.duplicate_physical),
                reading_signature(trial.duplicate_recursive)
            ),
            trial.duplicate_physical.outward == 1 && trial.duplicate_recursive.outward == 1,
        ),
        evaluation(
            "branch_cycle_silence",
            "branch.outward==0 && cycle.outward==0",
            format!(
                "branch={}~cycle={}",
                reading_signature(trial.branch),
                reading_signature(trial.cycle)
            ),
            trial.branch.outward == 0 && trial.cycle.outward == 0,
        ),
        evaluation(
            "pause_resume",
            "pause_stable && resume_equal",
            format!(
                "pause_stable={}~resume_equal={}",
                trial.pause_stable, trial.resume_equal
            ),
            trial.pause_stable && trial.resume_equal,
        ),
        evaluation(
            "natural_quiescence",
            "all queues empty and naturally quiescent",
            format!(
                "quiet={}~queue={}",
                all_quiet(trial),
                queue_state(all_quiet(trial))
            ),
            all_quiet(trial),
        ),
        evaluation(
            "bounded_work",
            "maximum_work<=20000",
            format!("maximum_work={}", trial.maximum_work),
            trial.maximum_work <= WORK_CEILING,
        ),
        evaluation(
            "bounded_same_body_memory",
            "maximum_bytes<=8192 && all seven before==after",
            format!(
                "maximum_bytes={}~pairs={}~stable={}",
                trial.maximum_bytes,
                memory_signature(trial.memory),
                trial.memory.stable()
            ),
            trial.maximum_bytes <= BYTE_CEILING && trial.memory.stable(),
        ),
        evaluation(
            "cumulative_px0_px7_lrc",
            "PX7 training/heldout/return/quiet/stability pass",
            cumulative_signature(trial.cumulative),
            trial.cumulative.passed,
        ),
        evaluation(
            "exact_replay",
            "independent Trial values exactly equal",
            format!(
                "exact={}~first_divergence={}",
                row.replay, row.first_divergence
            ),
            row.replay,
        ),
    ]
}

fn evaluation(
    name: &'static str,
    expected: &'static str,
    actual: String,
    passed: bool,
) -> Evaluation {
    Evaluation {
        name,
        expected,
        actual,
        passed,
    }
}

fn csv(rows: &[DiagnosticRow]) -> String {
    let mut output = String::from("root,namespace,reverse,reflect,twist,schedule,clause_index,clause_name,expected,actual,passed,observations,memory_pairs,maximum_work,maximum_bytes,pause_stable,resume_equal,cumulative,replay,first_failed_clause,first_divergent_state\n");
    for row in rows {
        let clauses = evaluations(row);
        let first_failed = clauses
            .iter()
            .find(|item| !item.passed)
            .map(|item| item.name)
            .unwrap_or("none");
        for (index, item) in clauses.iter().enumerate() {
            let fields = [
                row.trial.case.root.to_string(),
                (row.trial.case.root << 32).to_string(),
                row.trial.case.reverse.to_string(),
                row.trial.case.reflect.to_string(),
                row.trial.case.twist.to_string(),
                SCHEDULE.to_owned(),
                (index + 1).to_string(),
                item.name.to_owned(),
                item.expected.to_owned(),
                item.actual.clone(),
                item.passed.to_string(),
                observations_signature(&row.trial),
                memory_signature(row.trial.memory),
                row.trial.maximum_work.to_string(),
                row.trial.maximum_bytes.to_string(),
                row.trial.pause_stable.to_string(),
                row.trial.resume_equal.to_string(),
                cumulative_signature(row.trial.cumulative),
                row.replay.to_string(),
                first_failed.to_owned(),
                row.first_divergence.clone(),
            ];
            output.push_str(&fields.join(","));
            output.push('\n');
        }
    }
    output
}

fn report(rows: &[DiagnosticRow]) -> String {
    let failures = rows
        .iter()
        .flat_map(evaluations)
        .filter(|item| !item.passed)
        .count();
    let failing_roots = rows
        .iter()
        .filter(|row| evaluations(row).iter().any(|item| !item.passed))
        .count();
    let mut output = String::new();
    writeln!(output, "# PX8 authority-v2 negative diagnostic v1\n").unwrap();
    writeln!(output, "Outcome: **DIAGNOSTIC COMPLETE; NOT AUTHORITY**.\n").unwrap();
    writeln!(output, "- roots serialized: `16/16`;").unwrap();
    writeln!(output, "- clause records serialized: `224/224`;").unwrap();
    writeln!(output, "- failing roots: `{failing_roots}`;").unwrap();
    writeln!(output, "- failing clauses: `{failures}`;").unwrap();
    writeln!(
        output,
        "- maximum work: `{}`;",
        rows.iter()
            .map(|row| row.trial.maximum_work)
            .max()
            .unwrap_or(0)
    )
    .unwrap();
    writeln!(
        output,
        "- maximum bytes: `{}`;",
        rows.iter()
            .map(|row| row.trial.maximum_bytes)
            .max()
            .unwrap_or(0)
    )
    .unwrap();
    writeln!(
        output,
        "- exact replay roots: `{}/16`;",
        rows.iter().filter(|row| row.replay).count()
    )
    .unwrap();
    writeln!(
        output,
        "- naturally quiescent roots: `{}/16`;\n",
        rows.iter().filter(|row| all_quiet(&row.trial)).count()
    )
    .unwrap();
    writeln!(output, "## Failed clauses\n").unwrap();
    for row in rows {
        for (index, item) in evaluations(row).iter().enumerate() {
            if !item.passed {
                writeln!(output, "- root `{}` layout `reverse={} reflect={} twist={}` clause `{}` `{}`: expected `{}`; actual `{}`; memory `{}`; first divergent state `{}`.", row.trial.case.root, row.trial.case.reverse, row.trial.case.reflect, row.trial.case.twist, index + 1, item.name, item.expected, item.actual, memory_signature(row.trial.memory), row.first_divergence).unwrap();
            }
        }
    }
    writeln!(output, "\n- authority marker emitted: `false`;").unwrap();
    writeln!(output, "- PX8 promotion claimed: `false`.").unwrap();
    output
}

fn observations_signature(trial: &Trial) -> String {
    [
        ("formation", trial.formation),
        ("completed", trial.completed),
        ("incomplete", trial.incomplete),
        ("blocked", trial.blocked),
        ("stale", trial.stale),
        ("duplicate_recursive", trial.duplicate_recursive),
        ("zero_length", trial.zero_length),
        ("duplicate_physical", trial.duplicate_physical),
        ("open", trial.open),
        ("aged", trial.aged),
        ("branch", trial.branch),
        ("cycle", trial.cycle),
    ]
    .into_iter()
    .map(|(name, value)| format!("{name}:{}", reading_signature(value)))
    .collect::<Vec<_>>()
    .join(";")
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

fn all_quiet(trial: &Trial) -> bool {
    readings(trial).into_iter().all(|value| value.quiet)
        && trial.cumulative.first.quiet
        && trial.cumulative.second.quiet
        && trial.cumulative.heldout.quiet
}

fn first_divergence(first: &Trial, second: &Trial) -> String {
    if first == second {
        return "none".to_owned();
    }
    for (name, left, right) in [
        ("formation", first.formation, second.formation),
        ("completed", first.completed, second.completed),
        ("incomplete", first.incomplete, second.incomplete),
        ("blocked", first.blocked, second.blocked),
        ("stale", first.stale, second.stale),
        (
            "duplicate_recursive",
            first.duplicate_recursive,
            second.duplicate_recursive,
        ),
        ("zero_length", first.zero_length, second.zero_length),
        (
            "duplicate_physical",
            first.duplicate_physical,
            second.duplicate_physical,
        ),
        ("open", first.open, second.open),
        ("aged", first.aged, second.aged),
        ("branch", first.branch, second.branch),
        ("cycle", first.cycle, second.cycle),
    ] {
        if left != right {
            return format!(
                "{name}:left={}~right={}",
                reading_signature(left),
                reading_signature(right)
            );
        }
    }
    if first.memory != second.memory {
        return format!(
            "memory:left={}~right={}",
            memory_signature(first.memory),
            memory_signature(second.memory)
        );
    }
    if first.cumulative != second.cumulative {
        return "cumulative".to_owned();
    }
    if first.maximum_work != second.maximum_work {
        return "maximum_work".to_owned();
    }
    if first.maximum_bytes != second.maximum_bytes {
        return "maximum_bytes".to_owned();
    }
    if first.pause_stable != second.pause_stable {
        return "pause_stable".to_owned();
    }
    if first.resume_equal != second.resume_equal {
        return "resume_equal".to_owned();
    }
    "case".to_owned()
}

fn reading_signature(value: Reading) -> String {
    format!(
        "outward={}~return={}~updates={}~work={}~queue={}~quiet={}~permanent={}~complete={}",
        value.outward,
        value.inward_modulation,
        value.updates,
        value.work,
        queue_state(value.quiet),
        value.quiet,
        value.permanent,
        value.complete
    )
}

fn memory_signature(value: MemoryPairs) -> String {
    format!("primary={}|{};uninterrupted={}|{};incomplete={}|{};duplicate={}|{};blocked={}|{};stale={}|{};cumulative={}|{}", value.primary[0], value.primary[1], value.uninterrupted[0], value.uninterrupted[1], value.incomplete[0], value.incomplete[1], value.duplicate[0], value.duplicate[1], value.blocked[0], value.blocked[1], value.stale[0], value.stale[1], value.cumulative[0], value.cumulative[1])
}

fn cumulative_signature(value: Cumulative) -> String {
    format!("pass={}~first_m={}~first_u={}~first_w={}~first_q={}~second_m={}~second_u={}~second_w={}~second_q={}~heldout_o={}~heldout_m={}~heldout_u={}~heldout_w={}~heldout_q={}~coupling={}~resistance={}~before={}~after={}", value.passed, value.first.modulation, value.first.updates, value.first.work, value.first.quiet, value.second.modulation, value.second.updates, value.second.work, value.second.quiet, value.heldout.outward, value.heldout.modulation, value.heldout.updates, value.heldout.work, value.heldout.quiet, join_i32(&value.couplings), join_u32(&value.resistance), value.before, value.after)
}

fn queue_state(quiet: bool) -> &'static str {
    if quiet {
        "empty"
    } else {
        "not_empty"
    }
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

fn identity_namespaces(case: Case) -> [u64; 8] {
    let base = case.root << 32;
    [
        base,
        base + 10_000,
        base + 20_000,
        base + 30_000,
        base + 40_000,
        base + 50_000,
        base + 60_000,
        (case.root + 1_000_000) << 32,
    ]
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
