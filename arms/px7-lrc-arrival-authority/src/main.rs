#![forbid(unsafe_code)]

use lr1_modulatory_physical_return::TransmissionMode;
use px4_lrc_lifetime::{arrive, field};
use px7_lrc_arrival::{
    Activity, Arrival, Body, Form, BOUNDARY_A, BOUNDARY_B, BOUNDARY_C, BOUNDARY_D, DOWNSTREAM_ONE,
    DOWNSTREAM_ZERO, INNER_ONE, INNER_ZERO, OUTWARD_SITE, RETURN_SITE,
};
use std::collections::BTreeSet;
use std::env;
use std::fmt::Write as _;
use std::fs::{rename, OpenOptions};
use std::io::Write as _;
use std::path::Path;
use std::process::Command;

const LAW: &str = "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10";
const PX4_SOURCE: &str = "a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71";
const PX6_CSV: &str = "9e14b0f065ba37966c2ffc300f6d149b0847d092cb90e666456d3889d889d9c6";
const PX6_REPORT: &str = "94a088fc732c24385a3b581af0e5cea2638645806c8bb8a73c81bfa39c9ec5a2";
const PX6_MANIFEST: &str = "653289cf42577dabb242475fd88abe24405b3e9a7e3cd4f2961489cc5fe6953a";
const PX7_SOURCE: &str = "d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e";
const PROTOCOL: &str = "827a220f12ba2c6713becb4d9f87bd1a21b0d756efbe5c9a8f88cd6dded51c8a";
const WORK_CEILING: u64 = 4096;
const BYTE_CEILING: usize = 4096;
const CSV: &str = "results/px7_lrc_arrival_authority_v1.csv";
const REPORT: &str = "results/px7_lrc_arrival_authority_v1.md";
const CSV_STAGE: &str = "results/px7_lrc_arrival_authority_v1.csv.staging";
const REPORT_STAGE: &str = "results/px7_lrc_arrival_authority_v1.md.staging";

const CASES: [Case; 16] = [
    Case::new(761_001, false, false, false),
    Case::new(761_002, false, false, true),
    Case::new(761_003, false, false, false),
    Case::new(761_004, false, false, true),
    Case::new(761_005, true, false, false),
    Case::new(761_006, true, false, true),
    Case::new(761_007, true, false, false),
    Case::new(761_008, true, false, true),
    Case::new(761_009, false, true, false),
    Case::new(761_010, false, true, true),
    Case::new(761_011, false, true, false),
    Case::new(761_012, false, true, true),
    Case::new(761_013, true, true, false),
    Case::new(761_014, true, true, true),
    Case::new(761_015, true, true, false),
    Case::new(761_016, true, true, true),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Case {
    number: u64,
    reverse: bool,
    reflect: bool,
    altered: bool,
}

impl Case {
    const fn new(number: u64, reverse: bool, reflect: bool, altered: bool) -> Self {
        Self {
            number,
            reverse,
            reflect,
            altered,
        }
    }

    fn form(self) -> Form {
        Form {
            namespace: self.number << 32,
            reverse_construction: self.reverse,
            reflected_positions: self.reflect,
            altered_pairing: self.altered,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Observation {
    boundary: [usize; 4],
    inner: [usize; 2],
    downstream: [usize; 2],
    link_crossings: [usize; 2],
    link_impulses: [i32; 2],
    outward: usize,
    modulation: u64,
    updates: u64,
    proposals: u64,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cumulative {
    passed: bool,
    work: u64,
    bytes: usize,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    case: Case,
    initial_coupling: [i32; 2],
    after_first_coupling: [i32; 2],
    mature_coupling: [i32; 2],
    mature_resistance: [u32; 2],
    no_arrival: Observation,
    subthreshold: Observation,
    unmatched: Observation,
    outward_only: Observation,
    weak: Observation,
    mature: Observation,
    overlap: Observation,
    recurrent: [Observation; 3],
    resume_gap: Observation,
    resumed: Observation,
    stale: Observation,
    stale_links: [bool; 2],
    changed_old: Observation,
    changed_new: Observation,
    cumulative: Cumulative,
    maximum_work: u64,
    maximum_bytes: usize,
    memory_stable: bool,
    permanent: u64,
    bytes: usize,
    claims: [bool; 12],
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
            println!("PX7_LRC_ARRIVAL_AUTHORITY_PREFLIGHT_OK");
        }
        [mode] if mode == "--authority-v1" => authority(AuthorityPermission::Granted),
        _ => std::process::exit(2),
    }
}

fn audit() {
    for (path, digest) in [
        ("crates/lr1-modulatory-physical-return/src/lib.rs", LAW),
        (
            "experiments/px7_lrc_cumulative_arrival_authority_protocol_v1.md",
            PROTOCOL,
        ),
        ("arms/px4-lrc-lifetime/src/lib.rs", PX4_SOURCE),
        ("results/px6_lrc_consequence_authority_v1.csv", PX6_CSV),
        ("results/px6_lrc_consequence_authority_v1.md", PX6_REPORT),
        (
            "experiments/pxc_active_surface_manifest_v4.csv",
            PX6_MANIFEST,
        ),
        ("crates/px7-lrc-arrival/src/lib.rs", PX7_SOURCE),
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
    assert_eq!(CASES.iter().filter(|case| case.reverse).count(), 8);
    assert_eq!(CASES.iter().filter(|case| case.reflect).count(), 8);
    assert_eq!(CASES.iter().filter(|case| case.altered).count(), 8);
    assert_eq!(CASES.first().map(|case| case.number), Some(761_001));
    assert_eq!(CASES.last().map(|case| case.number), Some(761_016));
    for reverse in [false, true] {
        for reflect in [false, true] {
            assert_eq!(
                CASES
                    .iter()
                    .filter(|case| case.reverse == reverse && case.reflect == reflect)
                    .count(),
                4
            );
            assert_eq!(
                CASES
                    .iter()
                    .filter(|case| {
                        case.reverse == reverse && case.reflect == reflect && case.altered
                    })
                    .count(),
                2
            );
        }
    }
}

fn preflight() {
    absent(&[CSV, REPORT, CSV_STAGE, REPORT_STAGE]);
}

fn authority(_permission: AuthorityPermission) {
    eprintln!("PX7_LRC_ARRIVAL_AUTHORITY_V1_EVIDENCE_SPENT");
    let rows = CASES.into_iter().map(replay).collect::<Vec<_>>();
    let globals = global_claims(&rows);
    assert!(rows.iter().all(|row| row.passed), "authority row failed");
    assert!(
        globals.into_iter().all(|claim| claim),
        "global clause failed"
    );
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(REPORT_STAGE, REPORT, &report(&rows, globals));
    println!("PX7_LRC_ARRIVAL_AUTHORITY_PASS rows=16/16 clauses=198/198");
}

fn replay(case: Case) -> Row {
    let first = run(case);
    let second = run(case);
    let exact = first == second;
    let mut row = first;
    row.exact_replay = exact;
    row.claims[11] = exact;
    row.passed = row.claims.into_iter().all(|claim| claim);
    row
}

fn run(case: Case) -> Row {
    let form = case.form();
    let mut empty_body = Body::new(form);
    let no_arrival = perform(&mut empty_body, []);

    let mut low_body = Body::new(form);
    let low_arrivals = [at(&low_body, BOUNDARY_A, 0, 0, 1)];
    let subthreshold = perform(&mut low_body, low_arrivals);

    let mut unmatched_body = Body::new(form);
    let unmatched_arrivals = [
        at(&unmatched_body, BOUNDARY_A, 0, 1, 4),
        at(&unmatched_body, BOUNDARY_C, 2, 1, 5),
    ];
    let unmatched = perform(&mut unmatched_body, unmatched_arrivals);

    let mut outward_body = Body::new(form);
    let outward_arrivals = [at(&outward_body, OUTWARD_SITE, 0, 1, 2)];
    let outward_only = perform(&mut outward_body, outward_arrivals);

    let mut weak_body = Body::new(form);
    let weak_arrivals = heldout(&weak_body, 0, case.altered, false);
    let weak = perform(&mut weak_body, weak_arrivals);

    let mut body = Body::new(form);
    let initial_coupling = body.link_couplings();
    let first_arrivals = maturation(&body, 0, case.altered);
    let first_training = perform(&mut body, first_arrivals);
    let after_first_coupling = body.link_couplings();
    let second_arrivals = maturation(&body, 10, case.altered);
    let second_training = perform(&mut body, second_arrivals);
    let mature_coupling = body.link_couplings();
    let mature_resistance = body.link_resistances();
    let permanent = body.permanent_fingerprint();
    let bytes = body.persistent_bytes();

    let mut mature_clone = body.clone();
    let mature_arrivals = heldout(&mature_clone, 20, case.altered, false);
    let mature = perform(&mut mature_clone, mature_arrivals);

    let mut overlap_body = body.clone();
    let overlap_arrivals = heldout(&overlap_body, 20, case.altered, true);
    let overlap = perform(&mut overlap_body, overlap_arrivals);

    let mut recurrent_body = body.clone();
    let recurrent = [20, 30, 40].map(|tick| {
        let arrivals = heldout(&recurrent_body, tick, case.altered, false);
        perform(&mut recurrent_body, arrivals)
    });

    let mut resume_body = body.clone();
    let gap_arrivals = [at(&resume_body, BOUNDARY_D, 20, 1, 3)];
    let resume_gap = perform(&mut resume_body, gap_arrivals);
    let resumed_arrivals = heldout(&resume_body, 30, case.altered, false);
    let resumed = perform(&mut resume_body, resumed_arrivals);

    let mut stale_body = body.clone();
    let stale_pressure = stale_body.elapse(100);
    let stale_links = stale_body.links_live();
    let stale_arrivals = heldout(&stale_body, 110, case.altered, false);
    let stale = perform(&mut stale_body, stale_arrivals);

    let changed_form = Form {
        namespace: (case.number + 1_000_000) << 32,
        reverse_construction: !case.reverse,
        reflected_positions: !case.reflect,
        altered_pairing: !case.altered,
    };
    let mut changed_body = Body::new(changed_form);
    let changed_pair = !case.altered;
    let changed_first_arrivals = maturation(&changed_body, 0, changed_pair);
    let changed_first = perform(&mut changed_body, changed_first_arrivals);
    let changed_second_arrivals = maturation(&changed_body, 10, changed_pair);
    let changed_second = perform(&mut changed_body, changed_second_arrivals);
    let changed_old_arrivals = heldout(&changed_body, 20, case.altered, false);
    let changed_old = perform(&mut changed_body, changed_old_arrivals);
    let changed_new_arrivals = heldout(&changed_body, 30, changed_pair, false);
    let changed_new = perform(&mut changed_body, changed_new_arrivals);
    let cumulative = cumulative_conformance(case);

    let memory_stable = body.persistent_bytes() == bytes
        && mature_clone.persistent_bytes() == bytes
        && overlap_body.persistent_bytes() == bytes
        && recurrent_body.persistent_bytes() == bytes
        && resume_body.persistent_bytes() == bytes
        && stale_body.persistent_bytes() == bytes
        && changed_body.persistent_bytes() == bytes;

    let observations = [
        &no_arrival,
        &subthreshold,
        &unmatched,
        &outward_only,
        &weak,
        &first_training,
        &second_training,
        &mature,
        &overlap,
        &recurrent[0],
        &recurrent[1],
        &recurrent[2],
        &resume_gap,
        &resumed,
        &stale,
        &changed_first,
        &changed_second,
        &changed_old,
        &changed_new,
    ];
    let maximum_work = observations
        .iter()
        .map(|observation| observation.work)
        .chain([stale_pressure.total(), cumulative.work])
        .max()
        .unwrap_or(0);
    let maximum_bytes = bytes.max(cumulative.bytes);

    let quiet = observations.iter().all(|observation| observation.quiescent);
    let no_activity = |observation: &Observation| {
        observation.inner == [0, 0]
            && observation.downstream == [0, 0]
            && observation.outward == 0
            && observation.updates == 0
            && observation.proposals == 0
    };
    let useful = |observation: &Observation| {
        observation.inner == [1, 1]
            && observation.downstream == [1, 1]
            && observation.link_crossings == [1, 1]
            && observation.link_impulses == [2, 2]
            && observation.outward == 1
            && observation.updates == 0
            && observation.proposals == 0
            && observation.work <= WORK_CEILING
            && observation.quiescent
    };

    let mut row = Row {
        case,
        initial_coupling,
        after_first_coupling,
        mature_coupling,
        mature_resistance,
        no_arrival,
        subthreshold,
        unmatched,
        outward_only,
        weak,
        mature,
        overlap,
        recurrent,
        resume_gap,
        resumed,
        stale,
        stale_links,
        changed_old,
        changed_new,
        cumulative,
        maximum_work,
        maximum_bytes,
        memory_stable,
        permanent,
        bytes,
        claims: [false; 12],
        exact_replay: false,
        passed: false,
    };
    row.claims = [
        row.no_arrival.boundary == [0; 4] && no_activity(&row.no_arrival),
        row.subthreshold.boundary == [0; 4] && no_activity(&row.subthreshold),
        no_activity(&row.unmatched) && no_activity(&row.outward_only),
        row.initial_coupling == [1, 1]
            && row.after_first_coupling == [2, 2]
            && row.mature_coupling == [2, 2]
            && row.mature_resistance == [6, 6]
            && row.weak.outward == 0
            && useful(&row.mature),
        useful(&row.overlap),
        row.recurrent.iter().all(useful),
        row.resume_gap.outward == 0
            && row.resume_gap.quiescent
            && useful(&row.resumed)
            && row.maximum_work <= WORK_CEILING,
        row.stale_links == [false, false] && row.stale.outward == 0,
        row.changed_old.outward == 0 && useful(&row.changed_new),
        first_training.modulation == 2
            && first_training.updates == 2
            && second_training.modulation == 2
            && second_training.updates == 2
            && row.mature.modulation == 0
            && row.cumulative.passed
            && row.cumulative.quiescent
            && row.permanent != 0
            && row.bytes > 0,
        quiet
            && row.maximum_work <= WORK_CEILING
            && row.maximum_bytes <= BYTE_CEILING
            && row.memory_stable,
        false,
    ];
    row.passed = row.claims.into_iter().all(|claim| claim);
    row
}

fn cumulative_conformance(case: Case) -> Cumulative {
    let mark = (case.number << 32) + 0x0700_0000;
    let mut world = field(
        mark,
        case.reverse,
        case.reflect,
        TransmissionMode::Modulatory,
    );
    arrive(&mut world.space, world.source, 0, 31, mark + 901);
    arrive(&mut world.space, world.returner, 2, 32, mark + 902);
    let execution = world.space.propagate();
    let candidates = world.space.arrows_between(world.source, world.effect);
    let learned = candidates.len() == 1
        && world.space.arrow_is_live(candidates[0])
        && world.space.arrow_resistance(candidates[0]) == 4
        && world.space.arrow_coupling(candidates[0]) == 2
        && execution.work.local_return_updates == 1
        && execution.work.modulatory_deliveries == 1;
    let pressure = world.space.advance_time(20);
    let retained = world.space.arrow_is_live(candidates[0])
        && world.space.arrow_resistance(candidates[0]) == 2;
    Cumulative {
        passed: learned && retained,
        work: execution.work.total().max(pressure.total()),
        bytes: world.space.persistent_bytes(),
        quiescent: execution.naturally_quiescent,
    }
}

fn perform<const N: usize>(body: &mut Body, arrivals: [Arrival; N]) -> Observation {
    let activity = body.participate(arrivals);
    observe(body, activity)
}

fn observe(body: &Body, activity: Activity) -> Observation {
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
    let crossing_impulse = |from, to| {
        let from = body.physical(from);
        let to = body.physical(to);
        execution
            .crossings
            .iter()
            .filter(|entry| entry.from_physical == from && entry.to_physical == to)
            .map(|entry| entry.impulse)
            .sum()
    };
    Observation {
        boundary: [BOUNDARY_A, BOUNDARY_B, BOUNDARY_C, BOUNDARY_D].map(fires),
        inner: [INNER_ZERO, INNER_ONE].map(fires),
        downstream: [DOWNSTREAM_ZERO, DOWNSTREAM_ONE].map(fires),
        link_crossings: [
            crossing(INNER_ZERO, DOWNSTREAM_ZERO),
            crossing(INNER_ONE, DOWNSTREAM_ONE),
        ],
        link_impulses: [
            crossing_impulse(INNER_ZERO, DOWNSTREAM_ZERO),
            crossing_impulse(INNER_ONE, DOWNSTREAM_ONE),
        ],
        outward: crossing(DOWNSTREAM_ONE, OUTWARD_SITE),
        modulation: execution.work.modulatory_deliveries,
        updates: execution.work.local_return_updates,
        proposals: execution.work.local_structural_proposals,
        work: activity.work(),
        quiescent: activity.naturally_quiescent(),
    }
}

fn boundary(body: &Body, altered: bool, begin: i64) -> [Arrival; 3] {
    let paired = if altered { BOUNDARY_D } else { BOUNDARY_B };
    [
        at(body, BOUNDARY_A, begin, 1, 10),
        at(body, paired, begin, 1, 11),
        at(body, BOUNDARY_C, begin + 2, 1, 12),
    ]
}

fn heldout(body: &Body, begin: i64, altered: bool, overlap: bool) -> [Arrival; 4] {
    let [a, paired, c] = boundary(body, altered, begin);
    let extra = if overlap {
        if altered {
            BOUNDARY_B
        } else {
            BOUNDARY_D
        }
    } else {
        paired_site(altered)
    };
    let impulse = i32::from(overlap);
    [a, paired, c, at(body, extra, begin, impulse, 13)]
}

fn maturation(body: &Body, begin: i64, altered: bool) -> [Arrival; 7] {
    let [a, paired, c] = boundary(body, altered, begin);
    [
        a,
        paired,
        c,
        at(body, DOWNSTREAM_ZERO, begin + 2, 1, 20),
        at(body, RETURN_SITE, begin + 3, 1, 21),
        at(body, DOWNSTREAM_ONE, begin + 4, 1, 22),
        at(body, RETURN_SITE, begin + 5, 1, 23),
    ]
}

fn paired_site(altered: bool) -> usize {
    if altered {
        BOUNDARY_D
    } else {
        BOUNDARY_B
    }
}

fn at(body: &Body, site: usize, tick: i64, impulse: i32, phase: i32) -> Arrival {
    Arrival {
        tick,
        phase,
        origin: 9_000_000 + site as u64 * 100 + phase as u64,
        position: body.coordinate(site),
        impulse,
    }
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "root,namespace,reverse,reflect,altered,initial_coupling,after_first_coupling,mature_coupling,mature_resistance,no_arrival,subthreshold,unmatched,outward_only,weak,mature,overlap,recurrent,resume_gap,resumed,stale,stale_links,changed_old,changed_new,cumulative,maximum_work,maximum_bytes,memory_stable,permanent,bytes,claims,exact_replay,passed\n",
    );
    for row in rows {
        let fields = [
            row.case.number.to_string(),
            (row.case.number << 32).to_string(),
            row.case.reverse.to_string(),
            row.case.reflect.to_string(),
            row.case.altered.to_string(),
            join_i32(&row.initial_coupling),
            join_i32(&row.after_first_coupling),
            join_i32(&row.mature_coupling),
            join_u32(&row.mature_resistance),
            signature(&row.no_arrival),
            signature(&row.subthreshold),
            signature(&row.unmatched),
            signature(&row.outward_only),
            signature(&row.weak),
            signature(&row.mature),
            signature(&row.overlap),
            row.recurrent
                .iter()
                .map(signature)
                .collect::<Vec<_>>()
                .join(";"),
            signature(&row.resume_gap),
            signature(&row.resumed),
            signature(&row.stale),
            join_bool(&row.stale_links),
            signature(&row.changed_old),
            signature(&row.changed_new),
            format!(
                "pass={}~work={}~bytes={}~q={}",
                row.cumulative.passed,
                row.cumulative.work,
                row.cumulative.bytes,
                row.cumulative.quiescent
            ),
            row.maximum_work.to_string(),
            row.maximum_bytes.to_string(),
            row.memory_stable.to_string(),
            row.permanent.to_string(),
            row.bytes.to_string(),
            join_bool(&row.claims),
            row.exact_replay.to_string(),
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
        .map(|row| row.case.number)
        .collect::<BTreeSet<_>>();
    let changed = rows
        .iter()
        .map(|row| (row.case.number + 1_000_000) << 32)
        .collect::<BTreeSet<_>>();
    let strata = [false, true].into_iter().all(|reverse| {
        [false, true].into_iter().all(|reflect| {
            let matching = rows
                .iter()
                .filter(|row| row.case.reverse == reverse && row.case.reflect == reflect)
                .collect::<Vec<_>>();
            matching.len() == 4 && matching.iter().filter(|row| row.case.altered).count() == 2
        })
    });
    [
        roots.len() == 16 && roots.iter().copied().eq(761_001..=761_016),
        strata,
        changed.len() == 16
            && rows
                .iter()
                .all(|row| (761_001..=761_016).contains(&row.case.number)),
        rows.iter().all(|row| row.claims[9]),
        rows.iter().all(|row| row.claims[10] && row.claims[11]),
        rows.len() == 16,
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
        "# PX7 LR-C cumulative physical-arrival authority v1\n"
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
    writeln!(output, "- row clauses: `{row_clauses}/192`;").unwrap();
    writeln!(output, "- global clauses: `{global_clauses}/6`;").unwrap();
    writeln!(
        output,
        "- total clauses: `{}/198`;",
        row_clauses + global_clauses
    )
    .unwrap();
    writeln!(
        output,
        "- exact replay: `{}`;",
        rows.iter().all(|row| row.exact_replay)
    )
    .unwrap();
    writeln!(
        output,
        "- natural quiescence: `{}`;",
        rows.iter().all(|row| row.claims[10])
    )
    .unwrap();
    writeln!(
        output,
        "- maximum batch work: `{}` (ceiling `{WORK_CEILING}`);",
        rows.iter().map(|row| row.maximum_work).max().unwrap_or(0)
    )
    .unwrap();
    writeln!(
        output,
        "- maximum persistent bytes: `{}` (ceiling `{BYTE_CEILING}`);",
        rows.iter().map(|row| row.maximum_bytes).max().unwrap_or(0)
    )
    .unwrap();
    writeln!(
        output,
        "- retained memory stable: `{}`;",
        rows.iter().all(|row| row.memory_stable)
    )
    .unwrap();
    writeln!(output, "- learned couplings: `2|2` in every row;").unwrap();
    writeln!(
        output,
        "- no-arrival/subthreshold/unmatched/outward-only controls: `{}`;",
        rows.iter()
            .all(|row| row.claims[0] && row.claims[1] && row.claims[2])
    )
    .unwrap();
    writeln!(
        output,
        "- mature/overlap/recurrent/resume controls: `{}`;",
        rows.iter()
            .all(|row| row.claims[3] && row.claims[4] && row.claims[5] && row.claims[6])
    )
    .unwrap();
    writeln!(
        output,
        "- stale/reorganized controls: `{}`;",
        rows.iter().all(|row| row.claims[7] && row.claims[8])
    )
    .unwrap();
    writeln!(
        output,
        "- cumulative PX0--PX6+LR-C conformance: `{}`;",
        rows.iter().all(|row| row.claims[9])
    )
    .unwrap();
    writeln!(
        output,
        "- one position-addressed participation interface: `true`;"
    )
    .unwrap();
    writeln!(
        output,
        "- request/start/invocation/query/session interface: `false`;"
    )
    .unwrap();
    writeln!(output, "- new organism law: `false`;").unwrap();
    writeln!(output, "- PX8 executed or advanced: `false`.").unwrap();
    output
}

fn signature(value: &Observation) -> String {
    format!(
        "b={}~i={}~d={}~x={}~xi={}~o={}~m={}~u={}~p={}~w={}~q={}",
        join_usize(&value.boundary),
        join_usize(&value.inner),
        join_usize(&value.downstream),
        join_usize(&value.link_crossings),
        join_i32(&value.link_impulses),
        value.outward,
        value.modulation,
        value.updates,
        value.proposals,
        value.work,
        value.quiescent
    )
}

fn join_usize(values: &[usize]) -> String {
    values
        .iter()
        .map(usize::to_string)
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

fn join_bool(values: &[bool]) -> String {
    values
        .iter()
        .map(bool::to_string)
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
        assert_eq!(WORK_CEILING, 4096);
        assert_eq!(BYTE_CEILING, 4096);
    }
}
