use px0_physical_correspondence::{
    ArrowSpec, CellId, CellSpec, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const FROZEN_START: &str = "873094497ff6eb74363191dc5edc479c7d66de72";
const FROZEN_PARENT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const PROTOCOL_SHA256: &str = "2918aeb4088f387617708a0bd05a25aa4290c260d9121b3f0aea4853030f0599";
const PX0_PX2_LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const FROZEN_NEGATIVE_SHA256: &str =
    "a029f250ed88f8f2fc164e0d2c9042675bf0a8c9ae51c89cf83ad1aa42e4fa9b";
const FROZEN_NEGATIVE_CSV_SHA256: &str =
    "685dc04db32a5785224c62ba5b589fa8e1e37382a8b613f5f2b5e396aa005f38";

const PROTOCOL: &str = "experiments/px3_r_trace_coupling_generic_opportunity_probe_v1_protocol.md";
const RESULT_CSV: &str = "results/px3_r_trace_coupling_generic_opportunity_probe_v1.csv";
const RESULT_MD: &str = "results/px3_r_trace_coupling_generic_opportunity_probe_v1.md";
const STAGING_CSV: &str = "results/.px3_r_trace_coupling_generic_opportunity_probe_v1.csv.staging";
const STAGING_MD: &str = "results/.px3_r_trace_coupling_generic_opportunity_probe_v1.md.staging";
const ACTUAL_NAMESPACE: u64 = 0x9_3100_0000;
const REFERENCE_NAMESPACE: u64 = 0x9_3200_0000;
const ROUTES: usize = 4;
const RECURRENCES: usize = 6;

// PX3_R_ORGANISM_VISIBLE_BEGIN
mod physics {
    use super::{
        ArrowSpec, CellId, CellSpec, PlasticSubstrate, SpikeInput, WorkLedger, RECURRENCES, ROUTES,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct State {
        pub firings: [usize; ROUTES],
        pub source_firings: [usize; ROUTES],
        pub proposals: u64,
        pub arrows: usize,
        pub local_arrows: usize,
        pub complete_fingerprint: u64,
        pub permanent_fingerprint: u64,
        pub quiescent: bool,
        pub extra_source_firings: usize,
        pub work: WorkLedger,
    }

    struct Matter {
        substrate: PlasticSubstrate,
        namespace: u64,
        drivers: [CellId; ROUTES],
        traces: [CellId; ROUTES],
    }

    pub(super) fn propagated(namespace: u64) -> State {
        let mut matter = fresh(namespace, true);
        for round in 0..RECURRENCES {
            let base = round as i64 * 12;
            let order = if round.is_multiple_of(2) {
                [[0usize, 1usize], [2usize, 3usize]]
            } else {
                [[2usize, 3usize], [0usize, 1usize]]
            };
            for (slot, lanes) in order.into_iter().enumerate() {
                let tick = base + slot as i64 * 4;
                for lane in lanes {
                    enter(
                        &mut matter.substrate,
                        matter.drivers[lane],
                        tick,
                        namespace + 0x10_000 + round as u64 * 0x100 + lane as u64,
                    );
                }
            }
        }
        observe(matter, true)
    }

    pub(super) fn external(namespace: u64) -> State {
        let mut matter = fresh(namespace, false);
        for round in 0..RECURRENCES {
            let base = round as i64 * 12;
            let order = if round.is_multiple_of(2) {
                [[0usize, 1usize], [2usize, 3usize]]
            } else {
                [[2usize, 3usize], [0usize, 1usize]]
            };
            for (slot, lanes) in order.into_iter().enumerate() {
                let tick = base + slot as i64 * 4;
                for lane in lanes {
                    enter(
                        &mut matter.substrate,
                        matter.traces[lane],
                        tick,
                        namespace + 0x20_000 + round as u64 * 0x100 + lane as u64,
                    );
                }
            }
        }
        observe(matter, false)
    }

    fn fresh(namespace: u64, paths: bool) -> Matter {
        let mut substrate = PlasticSubstrate::new();
        let mut drivers = [None; ROUTES];
        let mut consequences = [None; ROUTES];
        let mut traces = [None; ROUTES];
        for lane in 0..ROUTES {
            drivers[lane] = Some(
                substrate.add_cell(cell(namespace + 10 + lane as u64, 100 + lane as i32 * 100)),
            );
            consequences[lane] = Some(substrate.add_cell(cell(
                namespace + 20 + lane as u64,
                1_000 + lane as i32 * 100,
            )));
            traces[lane] =
                Some(substrate.add_cell(cell(namespace + 30 + lane as u64, lane as i32)));
        }
        let drivers = drivers.map(|cell| cell.expect("driver CELL"));
        let consequences = consequences.map(|cell| cell.expect("consequence CELL"));
        let traces = traces.map(|cell| cell.expect("trace CELL"));
        if paths {
            for lane in 0..ROUTES {
                substrate.add_arrow(arrow(drivers[lane], consequences[lane]));
                substrate.add_arrow(arrow(consequences[lane], traces[lane]));
            }
        }
        Matter {
            substrate,
            namespace,
            drivers,
            traces,
        }
    }

    fn observe(mut matter: Matter, paths: bool) -> State {
        let run = matter.substrate.propagate();
        let firings = std::array::from_fn(|lane| {
            run.trace
                .iter()
                .filter(|entry| {
                    entry.target_physical == matter.namespace + 30 + lane as u64 && entry.fired
                })
                .count()
        });
        let source_firings = std::array::from_fn(|lane| {
            run.trace
                .iter()
                .filter(|entry| {
                    entry.target_physical == matter.namespace + 10 + lane as u64 && entry.fired
                })
                .count()
        });
        let local_arrows = (0..ROUTES)
            .flat_map(|from| (0..ROUTES).filter_map(move |to| (from != to).then_some((from, to))))
            .map(|(from, to)| {
                matter
                    .substrate
                    .arrows_between(matter.traces[from], matter.traces[to])
                    .into_iter()
                    .filter(|arrow| matter.substrate.arrow_is_live(*arrow))
                    .count()
            })
            .sum();
        State {
            firings,
            source_firings,
            proposals: run.work.local_structural_proposals,
            arrows: matter.substrate.arrow_count(),
            local_arrows,
            complete_fingerprint: matter.substrate.complete_fingerprint(),
            permanent_fingerprint: matter.substrate.permanent_fingerprint(),
            quiescent: run.naturally_quiescent,
            extra_source_firings: source_firings
                .iter()
                .sum::<usize>()
                .saturating_sub(if paths { ROUTES * RECURRENCES } else { 0 }),
            work: run.work,
        }
    }

    fn cell(physical_id: u64, position: i32) -> CellSpec {
        CellSpec {
            physical_id,
            position,
            region: 0,
            threshold: 1,
            resistance: 1_000,
        }
    }

    fn arrow(from: CellId, to: CellId) -> ArrowSpec {
        ArrowSpec {
            from,
            to,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 1_000,
        }
    }

    fn enter(substrate: &mut PlasticSubstrate, target: CellId, tick: i64, origin: u64) {
        substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: origin,
            target,
            impulse: 1,
        });
    }
}
// PX3_R_ORGANISM_VISIBLE_END

#[derive(Clone, Debug, PartialEq, Eq)]
struct Report {
    actual: physics::State,
    reference: physics::State,
    actual_duplicate_exact: bool,
    reference_duplicate_exact: bool,
    source_exact: bool,
    collapse: bool,
    classification: &'static str,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let preflight = args == ["--preflight"];
    let probe = args == ["--probe"];
    if !preflight && !probe {
        eprintln!("PX3-R generic-opportunity PROBE requires --preflight or --probe");
        std::process::exit(2);
    }
    assert!(
        source_audit(),
        "frozen inputs and organism block must be exact"
    );
    for path in [RESULT_CSV, RESULT_MD, STAGING_CSV, STAGING_MD] {
        assert!(!Path::new(path).exists(), "PROBE artifact exists: {path}");
    }
    if preflight {
        println!("PX3_R_TRACE_COUPLING_GENERIC_OPPORTUNITY_PROBE_V1_PREFLIGHT_OK");
        return;
    }

    eprintln!("PX3_R_TRACE_COUPLING_GENERIC_OPPORTUNITY_PROBE_V1_EVIDENCE_SPENT");
    let report = run_probe();
    write_atomic(&report);
    println!(
        "PX3_R_TRACE_COUPLING_GENERIC_OPPORTUNITY_PROBE_V1_{}",
        report.classification
    );
    if report.collapse {
        std::process::exit(1);
    }
}

fn run_probe() -> Report {
    let actual = physics::propagated(ACTUAL_NAMESPACE);
    let actual_duplicate = physics::propagated(ACTUAL_NAMESPACE);
    let reference = physics::external(REFERENCE_NAMESPACE);
    let reference_duplicate = physics::external(REFERENCE_NAMESPACE);
    let actual_duplicate_exact = actual == actual_duplicate;
    let reference_duplicate_exact = reference == reference_duplicate;
    let exact_activity = actual.firings == [RECURRENCES; ROUTES]
        && actual.source_firings == [RECURRENCES; ROUTES]
        && reference.firings == [RECURRENCES; ROUTES];
    let controls = source_audit()
        && exact_activity
        && actual.quiescent
        && reference.quiescent
        && actual.extra_source_firings == 0
        && actual_duplicate_exact
        && reference_duplicate_exact;
    let collapse = controls
        && actual.proposals == 0
        && actual.local_arrows == 0
        && reference.proposals > 0
        && reference.local_arrows > 0;
    let classification = if collapse {
        "GENERIC_PROPOSAL_REQUIRES_EXTERNAL_FIRING"
    } else if controls && actual.proposals > 0 && actual.local_arrows > 0 {
        "EXISTING_GENERIC_OPPORTUNITY_PRESENT"
    } else {
        "FIRST_CLAUSE_FAILURE"
    };
    Report {
        actual,
        reference,
        actual_duplicate_exact,
        reference_duplicate_exact,
        source_exact: source_audit(),
        collapse,
        classification,
    }
}

fn source_audit() -> bool {
    let hashes = [
        (
            "crates/px0-physical-correspondence/src/lib.rs",
            PX0_PX2_LAW_SHA256,
        ),
        (
            "experiments/px3_physical_event_boundaries_frozen_negative_handoff.md",
            FROZEN_NEGATIVE_SHA256,
        ),
        (
            "results/px3_physical_event_boundaries_no_new_mechanism_probe_v3.csv",
            FROZEN_NEGATIVE_CSV_SHA256,
        ),
        (PROTOCOL, PROTOCOL_SHA256),
    ];
    let hashes_exact = hashes
        .into_iter()
        .all(|(path, expected)| sha256(path).as_deref() == Some(expected));
    let lineage_exact = command_output(&[
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
    let source = include_str!("px3_r_trace_coupling_generic_opportunity_probe.rs");
    let physical = source
        .split("// PX3_R_ORGANISM_VISIBLE_BEGIN")
        .nth(1)
        .and_then(|text| text.split("// PX3_R_ORGANISM_VISIBLE_END").next())
        .unwrap_or("")
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
        && lineage_exact
        && !physical.is_empty()
        && forbidden.iter().all(|word| !physical.contains(word))
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
        "cell,trace_firings,source_firings,proposals,arrows,local_arrows,complete_fingerprint,permanent_fingerprint,quiescent,extra_source_firings,duplicate_exact,queue_comparisons,spikes_delivered,generation_checks,state_updates,threshold_checks,firings,arrow_checks,spikes_emitted,local_eligibility_writes,local_return_updates,ordinary_pressure_updates,local_structural_proposals,physical_deallocations,work\n",
    );
    for (name, state, duplicate) in [
        (
            "actual-participation",
            &report.actual,
            report.actual_duplicate_exact,
        ),
        (
            "direct-external-reference",
            &report.reference,
            report.reference_duplicate_exact,
        ),
    ] {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            name,
            join_usize(&state.firings),
            join_usize(&state.source_firings),
            state.proposals,
            state.arrows,
            state.local_arrows,
            state.complete_fingerprint,
            state.permanent_fingerprint,
            state.quiescent,
            state.extra_source_firings,
            duplicate,
            ledger_csv(&state.work),
            state.work.total(),
        ));
    }
    let markdown = format!(
        "# PX3-R generic trace-coupling opportunity PROBE v1\n\nVerdict: **{}**.\n\nThis development PROBE freezes the first collapse; PX3 remains absent. Actual internally propagated participation fired every trace locus `{}` times but produced `{}` generic structural proposals and `{}` inter-trace ARROWs. The fresh direct-external reference fired the same trace marginals and produced `{}` proposals and `{}` local ARROWs. Both queues drained naturally, autonomous source refiring was `{}`, and complete duplicates were actual/reference `{}/{}`. Frozen source audit: `{}`. Ledgered work: `{}` operations.\n\nThe existing generic law therefore supplies spatial proposal physics only on external-source firing. It does not expose that opportunity when the same local CELL loci fire through actual retained participation. No arm-specific mechanism was present or tested.\n",
        report.classification,
        RECURRENCES,
        report.actual.proposals,
        report.actual.local_arrows,
        report.reference.proposals,
        report.reference.local_arrows,
        report.actual.extra_source_firings,
        report.actual_duplicate_exact,
        report.reference_duplicate_exact,
        report.source_exact,
        report.actual.work.total() + report.reference.work.total(),
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

fn ledger_csv(work: &WorkLedger) -> String {
    [
        work.queue_comparisons,
        work.spikes_delivered,
        work.generation_checks,
        work.state_updates,
        work.threshold_checks,
        work.firings,
        work.arrow_checks,
        work.spikes_emitted,
        work.local_eligibility_writes,
        work.local_return_updates,
        work.ordinary_pressure_updates,
        work.local_structural_proposals,
        work.physical_deallocations,
    ]
    .iter()
    .map(u64::to_string)
    .collect::<Vec<_>>()
    .join(",")
}
