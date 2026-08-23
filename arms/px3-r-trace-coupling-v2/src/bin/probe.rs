use px3_r_trace_coupling_v2::arm::{
    acquisition_work, add_work, advance, drive, drive_nearby, fresh, state, use_at, Counts, State,
    LANES,
};
use px3_r_trace_coupling_v2::{LocalActivityOpportunity, WorkLedger};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const FROZEN_START: &str = "873094497ff6eb74363191dc5edc479c7d66de72";
const FROZEN_PARENT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const ARM_LAW_SHA256: &str = "0027d8356170a673c3045980fed8a5a3f1509277072753bba03cb3b43143f6c9";
const ARM_WORLD_SHA256: &str = "abec17d32b110538133044f2366b27f4104b632101ed2d0f34e13f10d484a409";
const PROTOCOL_SHA256: &str = "f671332544b3e7bf0e291d22fcf10ff4416b3f1bfa0b710e07a554933cb19a6c";
const V1_RESULT_SHA256: &str = "f6e001d187f7d1fb3e306d04ad79fb63c4e92d6dff7943ef01708d920e1927bc";
const PROTOCOL: &str = "experiments/px3_r_direct_trace_coupling_probe_v2_retry_protocol.md";
const RESULT_CSV: &str = "results/px3_r_direct_trace_coupling_probe_v2.csv";
const RESULT_MD: &str = "results/px3_r_direct_trace_coupling_probe_v2.md";
const STAGING_CSV: &str = "results/.px3_r_direct_trace_coupling_probe_v2.csv.staging";
const STAGING_MD: &str = "results/.px3_r_direct_trace_coupling_probe_v2.md.staging";
const RECURRENCES: usize = 12;
const FIRST_TICK: i64 = 84;
const NORMAL: LocalActivityOpportunity = LocalActivityOpportunity {
    radius: 8,
    overlap: 1,
    delay: 1,
};
const LATE: LocalActivityOpportunity = LocalActivityOpportunity {
    radius: 8,
    overlap: 1,
    delay: 6,
};
const POSITIONS: [i32; LANES] = [0, 1, 2, 3];

#[derive(Clone, Debug, PartialEq, Eq)]
struct CellResult {
    name: &'static str,
    namespace: u64,
    acquisition: WorkLedger,
    training: Counts,
    before: State,
    trained: Counts,
    trained_after: State,
    crossed: Counts,
    crossed_after: State,
    duplicate_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ControlResult {
    name: &'static str,
    training: Counts,
    before: State,
    after: State,
    additional_work: WorkLedger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Report {
    positive: CellResult,
    absent: ControlResult,
    late: ControlResult,
    spaced: ControlResult,
    correlation: ControlResult,
    stale: ControlResult,
    source_exact: bool,
    marginals_matched: bool,
    organization_exact: bool,
    heldout_different: bool,
    controls_passed: bool,
    quiet: bool,
    positive_candidate: bool,
    classification: &'static str,
    total_work: u64,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let preflight = args == ["--preflight"];
    let probe = args == ["--probe"];
    if !preflight && !probe {
        eprintln!("PX3-R direct trace-coupling PROBE requires --preflight or --probe");
        std::process::exit(2);
    }
    assert!(
        source_audit(),
        "frozen source, lineage, and vocabulary audit"
    );
    for path in [RESULT_CSV, RESULT_MD, STAGING_CSV, STAGING_MD] {
        assert!(!Path::new(path).exists(), "PROBE artifact exists: {path}");
    }
    if preflight {
        println!("PX3_R_DIRECT_TRACE_COUPLING_PROBE_V2_PREFLIGHT_OK");
        return;
    }
    eprintln!("PX3_R_DIRECT_TRACE_COUPLING_PROBE_V2_EVIDENCE_SPENT");
    let report = run_probe();
    write_atomic(&report);
    println!(
        "PX3_R_DIRECT_TRACE_COUPLING_PROBE_V2_{}",
        report.classification
    );
    if !report.positive_candidate {
        std::process::exit(1);
    }
}

fn run_probe() -> Report {
    let mut positive = run_positive("positive", 0xb_3100_0000);
    let duplicate = run_positive("positive", 0xb_3100_0000);
    positive.duplicate_exact = same_cell(&positive, &duplicate);
    let absent = run_control(
        "opportunity-absent",
        0xb_3200_0000,
        None,
        training_entries(RECURRENCES, false),
        false,
        false,
    );
    let late = run_control(
        "late-arrival",
        0xb_3300_0000,
        Some(LATE),
        training_entries(RECURRENCES, false),
        false,
        false,
    );
    let spaced = run_control(
        "temporal-spacing",
        0xb_3400_0000,
        Some(NORMAL),
        training_entries(RECURRENCES, true),
        false,
        false,
    );
    let correlation = run_control(
        "correlation-only",
        0xb_3500_0000,
        Some(NORMAL),
        training_entries(RECURRENCES, false),
        true,
        false,
    );
    let stale = run_control(
        "single-occurrence-stale",
        0xb_3600_0000,
        Some(NORMAL),
        training_entries(1, false),
        false,
        true,
    );

    let marginals_matched = positive.training.continuation_firings == [RECURRENCES; LANES]
        && positive.training.consequence_firings == [RECURRENCES; LANES]
        && positive.training.trace_firings == [RECURRENCES; LANES]
        && positive.training.route_returns == [RECURRENCES; LANES]
        && positive.training.effects == [RECURRENCES; LANES]
        && positive.training.source_firings == [RECURRENCES; LANES]
        && positive
            .before
            .correspondence_resistance
            .windows(2)
            .all(|values| values[0] == values[1])
        && positive
            .before
            .directional_resistance
            .windows(2)
            .all(|values| values[0] == values[1])
        && positive.before.directional_live == [true; LANES];
    let organization_exact = trained_only(&positive.before)
        && strong(&positive.before, 0, 1)
        && strong(&positive.before, 1, 0)
        && strong(&positive.before, 2, 3)
        && strong(&positive.before, 3, 2);
    let heldout_different = positive.trained.effects == [1; LANES]
        && positive.crossed.effects == [1; LANES]
        && positive.trained.local_arrivals == [1; LANES]
        && positive.crossed.local_arrivals == [1; LANES]
        && positive
            .trained
            .local_impulse
            .iter()
            .all(|value| *value >= 2)
        && positive.crossed.local_impulse == [1; LANES]
        && min_trained(&positive.trained_after) > max_crossed(&positive.crossed_after);
    let controls_passed = no_local(&absent.before)
        && no_local(&late.after)
        && no_local(&spaced.before)
        && no_local(&correlation.before)
        && correlation.training.trace_firings == [0; LANES]
        && correlation.training.effects == [0; LANES]
        && no_local(&stale.after)
        && absent.training.trace_firings == [RECURRENCES; LANES]
        && late.training.trace_firings == [RECURRENCES; LANES]
        && spaced.training.trace_firings == [RECURRENCES; LANES];
    let quiet = positive.training.quiescent
        && positive.trained.quiescent
        && positive.crossed.quiescent
        && positive.training.extra_source_firings == 0
        && positive.trained.extra_source_firings == 0
        && positive.crossed.extra_source_firings == 0
        && positive.duplicate_exact
        && [&absent, &late, &spaced, &correlation, &stale]
            .into_iter()
            .all(|cell| cell.training.quiescent && cell.training.extra_source_firings == 0);
    let source_exact = source_audit();
    let positive_candidate = source_exact
        && marginals_matched
        && organization_exact
        && heldout_different
        && controls_passed
        && quiet;
    let classification = if positive_candidate {
        "DIRECT_TRACE_COUPLING_CANDIDATE"
    } else if source_exact && marginals_matched && controls_passed && quiet {
        "FROZEN_NEGATIVE"
    } else {
        "FIRST_CLAUSE_FAILURE"
    };
    let total_work = cell_work(&positive)
        + [&absent, &late, &spaced, &correlation, &stale]
            .into_iter()
            .map(control_work)
            .sum::<u64>();
    Report {
        positive,
        absent,
        late,
        spaced,
        correlation,
        stale,
        source_exact,
        marginals_matched,
        organization_exact,
        heldout_different,
        controls_passed,
        quiet,
        positive_candidate,
        classification,
        total_work,
    }
}

fn run_positive(name: &'static str, namespace: u64) -> CellResult {
    let mut matter = fresh(namespace, false, Some(NORMAL), POSITIONS, false);
    let acquisition = acquisition_work(&matter);
    let entries = training_entries(RECURRENCES, false);
    let training = drive(&mut matter, &entries);
    let before = state(&matter);
    let held_tick = last_tick(&entries) + 24;
    let trained_entries = [(0, 0), (0, 1), (8, 2), (8, 3)];
    let crossed_entries = [(0, 0), (0, 3), (8, 2), (8, 1)];
    let (trained, trained_after) = use_at(&matter, held_tick, &trained_entries);
    let (crossed, crossed_after) = use_at(&matter, held_tick, &crossed_entries);
    CellResult {
        name,
        namespace,
        acquisition,
        training,
        before,
        trained,
        trained_after,
        crossed,
        crossed_after,
        duplicate_exact: false,
    }
}

fn run_control(
    name: &'static str,
    namespace: u64,
    opportunity: Option<LocalActivityOpportunity>,
    entries: Vec<(i64, usize)>,
    nearby: bool,
    pressure_gap: bool,
) -> ControlResult {
    let mut matter = fresh(namespace, false, opportunity, POSITIONS, nearby);
    let mut training = if nearby {
        drive_nearby(&mut matter, &entries)
    } else {
        drive(&mut matter, &entries)
    };
    let before = state(&matter);
    let mut additional_work = WorkLedger::default();
    if pressure_gap {
        additional_work = advance(&mut matter, last_tick(&entries) + 80);
        add_work(&mut training.work, &additional_work);
    }
    let after = state(&matter);
    ControlResult {
        name,
        training,
        before,
        after,
        additional_work,
    }
}

fn training_entries(rounds: usize, spaced: bool) -> Vec<(i64, usize)> {
    let mut entries = Vec::new();
    for round in 0..rounds {
        let base = FIRST_TICK + round as i64 * if spaced { 20 } else { 18 };
        let order = if round.is_multiple_of(2) {
            [[0usize, 1usize], [2usize, 3usize]]
        } else {
            [[2usize, 3usize], [0usize, 1usize]]
        };
        for (slot, lanes) in order.into_iter().enumerate() {
            let cluster = base + slot as i64 * if spaced { 10 } else { 8 };
            entries.push((cluster, lanes[0]));
            entries.push((cluster + if spaced { 4 } else { 0 }, lanes[1]));
        }
    }
    entries
}

fn last_tick(entries: &[(i64, usize)]) -> i64 {
    entries.iter().map(|(tick, _)| *tick).max().unwrap_or(0)
}

fn trained_only(state: &State) -> bool {
    (0..LANES).all(|from| {
        (0..LANES).all(|to| {
            let expected = matches!((from, to), (0, 1) | (1, 0) | (2, 3) | (3, 2));
            state.local_live[from][to] == expected
        })
    })
}

fn strong(state: &State, from: usize, to: usize) -> bool {
    state.local_live[from][to] && state.local_resistance[from][to] > 3
}

fn no_local(state: &State) -> bool {
    state.local_live.iter().flatten().all(|value| !*value)
}

fn min_trained(state: &State) -> u32 {
    [(0, 1), (1, 0), (2, 3), (3, 2)]
        .into_iter()
        .map(|(from, to)| state.local_resistance[from][to])
        .min()
        .unwrap_or(0)
}

fn max_crossed(state: &State) -> u32 {
    [(0, 3), (3, 0), (2, 1), (1, 2)]
        .into_iter()
        .map(|(from, to)| state.local_resistance[from][to])
        .max()
        .unwrap_or(0)
}

fn same_cell(left: &CellResult, right: &CellResult) -> bool {
    left.name == right.name
        && left.namespace == right.namespace
        && left.acquisition == right.acquisition
        && left.training == right.training
        && left.before == right.before
        && left.trained == right.trained
        && left.trained_after == right.trained_after
        && left.crossed == right.crossed
        && left.crossed_after == right.crossed_after
}

fn cell_work(cell: &CellResult) -> u64 {
    cell.acquisition.total()
        + cell.training.work.total()
        + cell.trained.work.total()
        + cell.crossed.work.total()
}

fn control_work(cell: &ControlResult) -> u64 {
    cell.training.work.total()
}

fn source_audit() -> bool {
    let hashes = [
        ("crates/px0-physical-correspondence/src/lib.rs", LAW_SHA256),
        ("arms/px3-r-trace-coupling-v2/src/lib.rs", ARM_LAW_SHA256),
        ("arms/px3-r-trace-coupling-v2/src/arm.rs", ARM_WORLD_SHA256),
        (PROTOCOL, PROTOCOL_SHA256),
        (
            "results/px3_r_direct_trace_coupling_probe_v1.csv",
            V1_RESULT_SHA256,
        ),
    ];
    let hashes_exact = hashes
        .into_iter()
        .all(|(path, expected)| sha256(path).as_deref() == Some(expected));
    let lineage = command_output(&[
        "rev-parse",
        "px3-physical-event-boundaries-frozen-negative-handoff-v1^{commit}",
    ])
    .as_deref()
        == Some(FROZEN_START)
        && command_output(&[
            "rev-parse",
            "px2-physical-causal-direction-authoritative^{commit}",
        ])
        .as_deref()
            == Some(FROZEN_PARENT)
        && Command::new("git")
            .args(["merge-base", "--is-ancestor", FROZEN_START, "HEAD"])
            .status()
            .is_ok_and(|status| status.success());
    let physical = [include_str!("../lib.rs"), include_str!("../arm.rs")]
        .into_iter()
        .flat_map(str::lines)
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
        .to_ascii_lowercase();
    let forbidden = [
        "event",
        "episode",
        "history",
        "pair",
        "group",
        "member",
        "boundary",
        "semantic",
        "evaluator",
        "serializer",
        "old_m",
        "ds3",
    ];
    hashes_exact
        && lineage
        && forbidden.iter().all(|word| {
            !physical
                .split(|ch: char| !ch.is_alphanumeric() && ch != '_')
                .any(|token| token == *word)
        })
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
        .map(str::to_owned)
}

fn command_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn write_atomic(report: &Report) {
    let mut csv = String::from(
        "name,route_strength,direction_strength,continuations,consequences,traces,route_returns,local_arrivals,local_impulse,effects,local_resistance,local_live,trained_local_impulse,crossed_local_impulse,trained_after,crossed_after,quiescent,extra_source_firings,duplicate_exact,arrow_count,persistent_bytes,permanent_fingerprint,complete_fingerprint,work\n",
    );
    csv.push_str(&format!(
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
        report.positive.name,
        join_u32(&report.positive.before.correspondence_resistance),
        join_u32(&report.positive.before.directional_resistance),
        join_usize(&report.positive.training.continuation_firings),
        join_usize(&report.positive.training.consequence_firings),
        join_usize(&report.positive.training.trace_firings),
        join_usize(&report.positive.training.route_returns),
        join_usize(&report.positive.training.local_arrivals),
        join_i32(&report.positive.training.local_impulse),
        join_usize(&report.positive.training.effects),
        matrix_u32(&report.positive.before.local_resistance),
        matrix_bool(&report.positive.before.local_live),
        join_i32(&report.positive.trained.local_impulse),
        join_i32(&report.positive.crossed.local_impulse),
        matrix_u32(&report.positive.trained_after.local_resistance),
        matrix_u32(&report.positive.crossed_after.local_resistance),
        report.positive.training.quiescent,
        report.positive.training.extra_source_firings,
        report.positive.duplicate_exact,
        report.positive.before.arrow_count,
        report.positive.before.persistent_bytes,
        report.positive.before.permanent_fingerprint,
        report.positive.before.complete_fingerprint,
        cell_work(&report.positive),
    ));
    for control in [
        &report.absent,
        &report.late,
        &report.spaced,
        &report.correlation,
        &report.stale,
    ] {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},,,,,{},{},,{},{},{},{},{}\n",
            control.name,
            join_u32(&control.before.correspondence_resistance),
            join_u32(&control.before.directional_resistance),
            join_usize(&control.training.continuation_firings),
            join_usize(&control.training.consequence_firings),
            join_usize(&control.training.trace_firings),
            join_usize(&control.training.route_returns),
            join_usize(&control.training.local_arrivals),
            join_i32(&control.training.local_impulse),
            join_usize(&control.training.effects),
            matrix_u32(&control.after.local_resistance),
            matrix_bool(&control.after.local_live),
            control.training.quiescent,
            control.training.extra_source_firings,
            control.after.arrow_count,
            control.after.persistent_bytes,
            control.after.permanent_fingerprint,
            control.after.complete_fingerprint,
            control_work(control),
        ));
    }
    let markdown = format!(
        "# PX3-R direct physical trace coupling PROBE v2\n\nVerdict: **{}**. This is DEVELOPMENT discrimination; PX3 remains absent.\n\n| clause | pass |\n|---|:---:|\n| frozen sources, lineage, and forbidden-information audit | {} |\n| matched individual-route marginals and strength | {} |\n| recurrent `0<->1` / `2<->3` structure only | {} |\n| trained versus crossed held-out physics differs | {} |\n| absent, delayed-return, spaced, correlation-only, stale controls | {} |\n| natural quiescence, zero autonomous source refiring, exact replay | {} |\n\nIndividual route correspondence resistance: `{}`. Direction resistance: `{}`. Candidate resistance matrix: `{}`. Held-out trained/crossed local impulse: `{}` / `{}`. Total ledgered work: `{}` operations. Persistent bytes in the positive physical cell: `{}`.\n",
        report.classification,
        report.source_exact,
        report.marginals_matched,
        report.organization_exact,
        report.heldout_different,
        report.controls_passed,
        report.quiet,
        join_u32(&report.positive.before.correspondence_resistance),
        join_u32(&report.positive.before.directional_resistance),
        matrix_u32(&report.positive.before.local_resistance),
        join_i32(&report.positive.trained.local_impulse),
        join_i32(&report.positive.crossed.local_impulse),
        report.total_work,
        report.positive.before.persistent_bytes,
    );
    create_new(STAGING_CSV, csv.as_bytes());
    create_new(STAGING_MD, markdown.as_bytes());
    rename(STAGING_CSV, RESULT_CSV).expect("publish PROBE CSV");
    rename(STAGING_MD, RESULT_MD).expect("publish PROBE report");
}

fn create_new(path: &str, bytes: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create staging artifact");
    file.write_all(bytes).expect("write staging artifact");
    file.sync_all().expect("sync staging artifact");
}

fn join_usize(values: &[usize]) -> String {
    values
        .iter()
        .map(usize::to_string)
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

fn join_i32(values: &[i32]) -> String {
    values
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn matrix_u32(values: &[[u32; LANES]; LANES]) -> String {
    values
        .iter()
        .map(|row| join_u32(row))
        .collect::<Vec<_>>()
        .join(";")
}

fn matrix_bool(values: &[[bool; LANES]; LANES]) -> String {
    values
        .iter()
        .map(|row| {
            row.iter()
                .map(bool::to_string)
                .collect::<Vec<_>>()
                .join("|")
        })
        .collect::<Vec<_>>()
        .join(";")
}
