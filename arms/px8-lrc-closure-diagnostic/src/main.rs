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
const AUTHORITY_EVALUATOR: &str =
    "ccbf3547ae0534ccbbb0c00e8d058f47f9471afb4a30733cc124e981a0f606d0";
const NEGATIVE_DIAGNOSTIC: &str =
    "3c8df23536157cc91d315c96862d408027aabdb28c65bab133696405132b3116";
const PROTOCOL: &str = "0d769dfb4b6c9a0420cfdf8f6c299aa89c8b614720cc15386365ca6c6a2577a5";
const WORK_CEILING: u64 = 20_000;
const BYTE_CEILING: usize = 8_192;
const CSV: &str = "results/px8_lrc_closure_negative_v1_diagnostic.csv";
const REPORT: &str = "results/px8_lrc_closure_negative_v1_diagnostic.md";
const CSV_STAGE: &str = "results/px8_lrc_closure_negative_v1_diagnostic.csv.staging";
const REPORT_STAGE: &str = "results/px8_lrc_closure_negative_v1_diagnostic.md.staging";
const SCHEDULE: &str = "formation=learn_twice;complete=all@61;incomplete=omit4@70;blocked=resistance0@61;stale=once_age@111;compact=direct_open_fork_ring@0+aged@10;px7=train@0+10+heldout@20";

const CASES: [Case; 16] = [
    Case::new(862_001, false, false, 0),
    Case::new(862_002, false, false, 137),
    Case::new(862_003, false, false, 274),
    Case::new(862_004, false, false, 411),
    Case::new(862_005, true, false, 0),
    Case::new(862_006, true, false, 137),
    Case::new(862_007, true, false, 274),
    Case::new(862_008, true, false, 411),
    Case::new(862_009, false, true, 0),
    Case::new(862_010, false, true, 137),
    Case::new(862_011, false, true, 274),
    Case::new(862_012, false, true, 411),
    Case::new(862_013, true, true, 0),
    Case::new(862_014, true, true, 137),
    Case::new(862_015, true, true, 274),
    Case::new(862_016, true, true, 411),
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
struct DiagnosticRow {
    first: Trial,
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
        [mode] if mode == "--diagnostic-v1" => diagnostic(),
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
            "arms/px8-lrc-closure-authority/src/main.rs",
            AUTHORITY_EVALUATOR,
        ),
        (
            "experiments/px8_lrc_cumulative_closure_authority_negative_diagnostic_v1.md",
            NEGATIVE_DIAGNOSTIC,
        ),
        (
            "experiments/px8_lrc_closure_negative_v1_diagnostic_protocol_v1.md",
            PROTOCOL,
        ),
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
    assert_eq!(CASES.first().map(|case| case.root), Some(862_001));
    assert_eq!(CASES.last().map(|case| case.root), Some(862_016));
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
    let namespaces = CASES
        .into_iter()
        .flat_map(identity_namespaces)
        .collect::<BTreeSet<_>>();
    assert_eq!(namespaces.len(), 128);
}

fn diagnostic() {
    eprintln!("PX8_LRC_CLOSURE_NEGATIVE_V1_DIAGNOSTIC_SPENT");
    let rows = CASES.into_iter().map(reconstruct).collect::<Vec<_>>();
    let clause_count = rows.iter().map(|row| evaluations(row).len()).sum::<usize>();
    assert_eq!(rows.len(), 16, "diagnostic root serialization incomplete");
    assert_eq!(
        clause_count, 224,
        "diagnostic clause serialization incomplete"
    );
    let failures = rows
        .iter()
        .flat_map(evaluations)
        .filter(|evaluation| !evaluation.passed)
        .count();
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(REPORT_STAGE, REPORT, &report(&rows));
    println!(
        "PX8_LRC_CLOSURE_DIAGNOSTIC_COMPLETE roots=16 clauses=224 failures={failures} authority=false"
    );
}

fn reconstruct(case: Case) -> DiagnosticRow {
    let first = run(case);
    let second = run(case);
    DiagnosticRow {
        replay: first == second,
        first_divergence: first_divergence(&first, &second),
        first,
    }
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

fn cumulative_conformance(case: Case) -> Cumulative {
    let form = Form {
        namespace: (case.root + 1_000_000) << 32,
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

fn evaluations(row: &DiagnosticRow) -> Vec<Evaluation> {
    let trial = &row.first;
    let quiet = all_quiet(trial);
    vec![
        Evaluation {
            name: "formation_modulation_and_updates",
            expected: "formation.updates>0 && formation.inward_modulation>0",
            actual: format!(
                "updates={}~inward_modulation={}~outward={}~work={}~queue={}~quiet={}",
                trial.formation.updates,
                trial.formation.inward_modulation,
                trial.formation.outward,
                trial.formation.work,
                queue_state(trial.formation.quiet),
                trial.formation.quiet
            ),
            passed: trial.formation.updates > 0 && trial.formation.inward_modulation > 0,
        },
        Evaluation {
            name: "completed_exactly_once",
            expected: "completed.outward==1",
            actual: reading_signature(trial.completed),
            passed: trial.completed.outward == 1,
        },
        Evaluation {
            name: "completed_return_and_update",
            expected: "completed.inward_modulation==1 && completed.updates==1",
            actual: reading_signature(trial.completed),
            passed: trial.completed.inward_modulation == 1 && trial.completed.updates == 1,
        },
        Evaluation {
            name: "incomplete_blocked_stale_silence",
            expected: "incomplete.outward==0 && blocked.outward==0 && stale.outward==0",
            actual: format!(
                "incomplete={}~blocked={}~stale={}",
                reading_signature(trial.incomplete),
                reading_signature(trial.blocked),
                reading_signature(trial.stale)
            ),
            passed: trial.incomplete.outward == 0
                && trial.blocked.outward == 0
                && trial.stale.outward == 0,
        },
        Evaluation {
            name: "open_aged_silence",
            expected: "open.outward==0 && aged.outward==0",
            actual: format!(
                "open={}~aged={}",
                reading_signature(trial.open),
                reading_signature(trial.aged)
            ),
            passed: trial.open.outward == 0 && trial.aged.outward == 0,
        },
        Evaluation {
            name: "zero_length_exactly_once",
            expected: "zero_length.outward==1",
            actual: reading_signature(trial.zero_length),
            passed: trial.zero_length.outward == 1,
        },
        Evaluation {
            name: "duplicates_exactly_once",
            expected: "duplicate_physical.outward==1 && duplicate_recursive.outward==1",
            actual: format!(
                "physical={}~recursive={}",
                reading_signature(trial.duplicate_physical),
                reading_signature(trial.duplicate_recursive)
            ),
            passed: trial.duplicate_physical.outward == 1
                && trial.duplicate_recursive.outward == 1,
        },
        Evaluation {
            name: "branch_cycle_silence",
            expected: "branch.outward==0 && cycle.outward==0",
            actual: format!(
                "branch={}~cycle={}",
                reading_signature(trial.branch),
                reading_signature(trial.cycle)
            ),
            passed: trial.branch.outward == 0 && trial.cycle.outward == 0,
        },
        Evaluation {
            name: "pause_resume",
            expected: "pause_stable==true && resume_equal==true",
            actual: format!(
                "pause_stable={}~resume_equal={}",
                trial.pause_stable, trial.resume_equal
            ),
            passed: trial.pause_stable && trial.resume_equal,
        },
        Evaluation {
            name: "natural_quiescence",
            expected: "all batches naturally_quiescent with queue empty",
            actual: format!("quiet={}~queue={}", quiet, queue_state(quiet)),
            passed: quiet,
        },
        Evaluation {
            name: "bounded_work",
            expected: "maximum_work<=20000",
            actual: format!("maximum_work={}", trial.maximum_work),
            passed: trial.maximum_work <= WORK_CEILING,
        },
        Evaluation {
            name: "bounded_stable_memory",
            expected: "maximum_bytes<=8192 && memory_stable==true",
            actual: format!(
                "maximum_bytes={}~memory_stable={}",
                trial.maximum_bytes, trial.memory_stable
            ),
            passed: trial.maximum_bytes <= BYTE_CEILING && trial.memory_stable,
        },
        Evaluation {
            name: "cumulative_px0_px7_lrc",
            expected: "PX7 train returns 2+2; coupling 2|2; resistance 6|6; heldout outward 1; quiet/stable",
            actual: cumulative_signature(trial.cumulative),
            passed: trial.cumulative.passed,
        },
        Evaluation {
            name: "exact_replay",
            expected: "independent Trial values exactly equal",
            actual: format!(
                "exact={}~first_divergence={}",
                row.replay, row.first_divergence
            ),
            passed: row.replay,
        },
    ]
}

fn csv(rows: &[DiagnosticRow]) -> String {
    let mut output = String::from(
        "root,namespace,reverse,reflect,twist,schedule,clause_index,clause_name,expected,actual,passed,observations,maximum_work,maximum_bytes,memory_stable,pause_stable,resume_equal,cumulative,replay,first_failed_clause,first_divergent_state\n",
    );
    for row in rows {
        let clauses = evaluations(row);
        let first_failed = clauses
            .iter()
            .find(|clause| !clause.passed)
            .map(|clause| clause.name)
            .unwrap_or("none");
        for (index, clause) in clauses.iter().enumerate() {
            let fields = [
                row.first.case.root.to_string(),
                (row.first.case.root << 32).to_string(),
                row.first.case.reverse.to_string(),
                row.first.case.reflect.to_string(),
                row.first.case.twist.to_string(),
                SCHEDULE.to_owned(),
                (index + 1).to_string(),
                clause.name.to_owned(),
                clause.expected.to_owned(),
                clause.actual.clone(),
                clause.passed.to_string(),
                observations_signature(&row.first),
                row.first.maximum_work.to_string(),
                row.first.maximum_bytes.to_string(),
                row.first.memory_stable.to_string(),
                row.first.pause_stable.to_string(),
                row.first.resume_equal.to_string(),
                cumulative_signature(row.first.cumulative),
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
        .filter(|evaluation| !evaluation.passed)
        .count();
    let failing_roots = rows
        .iter()
        .filter(|row| evaluations(row).iter().any(|evaluation| !evaluation.passed))
        .count();
    let mut output = String::new();
    writeln!(output, "# PX8 closure negative-v1 diagnostic v1\n").unwrap();
    writeln!(output, "Outcome: **DIAGNOSTIC COMPLETE; NOT AUTHORITY**.\n").unwrap();
    writeln!(output, "- roots serialized: `16/16`;").unwrap();
    writeln!(output, "- clause records serialized: `224/224`;").unwrap();
    writeln!(output, "- failing roots: `{failing_roots}`;").unwrap();
    writeln!(output, "- failing clauses: `{failures}`;").unwrap();
    writeln!(
        output,
        "- maximum work: `{}`;",
        rows.iter()
            .map(|row| row.first.maximum_work)
            .max()
            .unwrap_or(0)
    )
    .unwrap();
    writeln!(
        output,
        "- maximum persistent bytes: `{}`;",
        rows.iter()
            .map(|row| row.first.maximum_bytes)
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
        rows.iter().filter(|row| all_quiet(&row.first)).count()
    )
    .unwrap();
    writeln!(output, "## Failed clauses\n").unwrap();
    for row in rows {
        for (index, evaluation) in evaluations(row).iter().enumerate() {
            if !evaluation.passed {
                writeln!(
                    output,
                    "- root `{}` layout `reverse={} reflect={} twist={}` clause `{}` `{}`: expected `{}`; actual `{}`; first divergent state `{}`.",
                    row.first.case.root,
                    row.first.case.reverse,
                    row.first.case.reflect,
                    row.first.case.twist,
                    index + 1,
                    evaluation.name,
                    evaluation.expected,
                    evaluation.actual,
                    row.first_divergence
                )
                .unwrap();
            }
        }
    }
    writeln!(output, "\n## Firewall\n").unwrap();
    writeln!(output, "- authority-v1 marker emitted: `false`;").unwrap();
    writeln!(output, "- authority-v1 result path written: `false`;").unwrap();
    writeln!(output, "- PX8 promotion or authority claim: `false`.").unwrap();
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
    .map(|(name, reading)| format!("{name}:{}", reading_signature(reading)))
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
    readings(trial).into_iter().all(|reading| reading.quiet)
        && trial.cumulative.first.quiet
        && trial.cumulative.second.quiet
        && trial.cumulative.heldout.quiet
}

fn first_divergence(first: &Trial, second: &Trial) -> String {
    if first.case != second.case {
        return "case".to_owned();
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
    if first.pause_stable != second.pause_stable {
        return "pause_stable".to_owned();
    }
    if first.resume_equal != second.resume_equal {
        return "resume_equal".to_owned();
    }
    if first.cumulative != second.cumulative {
        return format!(
            "cumulative:left={}~right={}",
            cumulative_signature(first.cumulative),
            cumulative_signature(second.cumulative)
        );
    }
    if first.maximum_work != second.maximum_work {
        return format!(
            "maximum_work:left={}~right={}",
            first.maximum_work, second.maximum_work
        );
    }
    if first.maximum_bytes != second.maximum_bytes {
        return format!(
            "maximum_bytes:left={}~right={}",
            first.maximum_bytes, second.maximum_bytes
        );
    }
    if first.memory_stable != second.memory_stable {
        return "memory_stable".to_owned();
    }
    "none".to_owned()
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

fn cumulative_signature(value: Cumulative) -> String {
    format!(
        "pass={}~first_m={}~first_u={}~first_w={}~first_q={}~second_m={}~second_u={}~second_w={}~second_q={}~heldout_o={}~heldout_m={}~heldout_u={}~heldout_w={}~heldout_q={}~coupling={}~resistance={}~bytes={}~stable={}",
        value.passed,
        value.first.modulation,
        value.first.updates,
        value.first.work,
        value.first.quiet,
        value.second.modulation,
        value.second.updates,
        value.second.work,
        value.second.quiet,
        value.heldout.outward,
        value.heldout.modulation,
        value.heldout.updates,
        value.heldout.work,
        value.heldout.quiet,
        join_i32(&value.couplings),
        join_u32(&value.resistance),
        value.bytes,
        value.memory_stable
    )
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
