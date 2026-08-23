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
const PROTOCOL_SHA256: &str = "91706ac253ccfe0b7a6c9b56d64ec6b97c91ac72f37abc14a874f02749872ffb";
const LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const V1_SOURCE_SHA256: &str = "b0d13b59cf9e89ea2d41cc3807aaea40ced78b615b9ac5dbb64623ed82a378b8";
const V1_FAILURE_SHA256: &str = "2437cf9090ebc37b068bd9ae448b614c8ea2147009c97c34a0cb37258df4166a";
const PROTOCOL: &str = "experiments/px3_r_trace_coupling_generic_opportunity_probe_v2_protocol.md";
const RESULT_CSV: &str = "results/px3_r_trace_coupling_generic_opportunity_probe_v2.csv";
const RESULT_MD: &str = "results/px3_r_trace_coupling_generic_opportunity_probe_v2.md";
const STAGING_CSV: &str = "results/.px3_r_trace_coupling_generic_opportunity_probe_v2.csv.staging";
const STAGING_MD: &str = "results/.px3_r_trace_coupling_generic_opportunity_probe_v2.md.staging";
const ACTUAL_NAMESPACE: u64 = 0x9_3300_0000;
const REFERENCE_NAMESPACE: u64 = 0x9_3400_0000;
const ROUTES: usize = 4;
const RECURRENCES: usize = 6;

// PX3_R_ORGANISM_VISIBLE_BEGIN
mod physics {
    use super::{
        ArrowSpec, CellId, CellSpec, PlasticSubstrate, SpikeInput, WorkLedger, RECURRENCES, ROUTES,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct State {
        pub trace_firings: [usize; ROUTES],
        pub source_firings: [usize; ROUTES],
        pub passive_firings: usize,
        pub proposals: u64,
        pub arrows: usize,
        pub local_arrows: usize,
        pub complete_fingerprint: u64,
        pub permanent_fingerprint: u64,
        pub quiescent: bool,
        pub extra_source_firings: usize,
        pub work: WorkLedger,
    }

    pub(super) fn propagated(namespace: u64) -> State {
        let mut substrate = PlasticSubstrate::new();
        let mut drivers = [None; ROUTES];
        let mut consequences = [None; ROUTES];
        let mut traces = [None; ROUTES];
        for lane in 0..ROUTES {
            drivers[lane] = Some(substrate.add_cell(cell(
                namespace + 10 + lane as u64,
                100 + lane as i32 * 100,
                1,
            )));
            consequences[lane] = Some(substrate.add_cell(cell(
                namespace + 20 + lane as u64,
                1_000 + lane as i32 * 100,
                1,
            )));
            traces[lane] =
                Some(substrate.add_cell(cell(namespace + 30 + lane as u64, lane as i32, 1)));
        }
        let drivers = drivers.map(|value| value.expect("driver CELL"));
        let consequences = consequences.map(|value| value.expect("consequence CELL"));
        let traces = traces.map(|value| value.expect("trace CELL"));
        for lane in 0..ROUTES {
            substrate.add_arrow(arrow(drivers[lane], consequences[lane]));
            substrate.add_arrow(arrow(consequences[lane], traces[lane]));
        }
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
                        &mut substrate,
                        drivers[lane],
                        tick,
                        namespace + 0x10_000 + round as u64 * 0x100 + lane as u64,
                    );
                }
            }
        }
        let run = substrate.propagate();
        let trace_firings =
            std::array::from_fn(|lane| firings_at(&run.trace, namespace + 30 + lane as u64));
        let source_firings =
            std::array::from_fn(|lane| firings_at(&run.trace, namespace + 10 + lane as u64));
        let local_arrows = count_local(&substrate, &traces);
        State {
            trace_firings,
            source_firings,
            passive_firings: 0,
            proposals: run.work.local_structural_proposals,
            arrows: substrate.arrow_count(),
            local_arrows,
            complete_fingerprint: substrate.complete_fingerprint(),
            permanent_fingerprint: substrate.permanent_fingerprint(),
            quiescent: run.naturally_quiescent,
            extra_source_firings: source_firings
                .iter()
                .sum::<usize>()
                .saturating_sub(ROUTES * RECURRENCES),
            work: run.work,
        }
    }

    pub(super) fn external(namespace: u64) -> State {
        let mut substrate = PlasticSubstrate::new();
        let source = substrate.add_cell(cell(namespace + 30, 0, 1));
        let passive = [
            substrate.add_cell(cell(namespace + 31, 1, 100)),
            substrate.add_cell(cell(namespace + 32, 2, 100)),
        ];
        enter(&mut substrate, source, 0, namespace + 0x20_000);
        let run = substrate.propagate();
        let passive_firings = passive
            .iter()
            .map(|cell| {
                let physical = if *cell == passive[0] {
                    namespace + 31
                } else {
                    namespace + 32
                };
                firings_at(&run.trace, physical)
            })
            .sum();
        let local_arrows = passive
            .iter()
            .map(|target| substrate.arrows_between(source, *target).len())
            .sum();
        State {
            trace_firings: [1, 0, 0, 0],
            source_firings: [0; ROUTES],
            passive_firings,
            proposals: run.work.local_structural_proposals,
            arrows: substrate.arrow_count(),
            local_arrows,
            complete_fingerprint: substrate.complete_fingerprint(),
            permanent_fingerprint: substrate.permanent_fingerprint(),
            quiescent: run.naturally_quiescent,
            extra_source_firings: firings_at(&run.trace, namespace + 30).saturating_sub(1),
            work: run.work,
        }
    }

    fn count_local(substrate: &PlasticSubstrate, traces: &[CellId; ROUTES]) -> usize {
        (0..ROUTES)
            .flat_map(|from| (0..ROUTES).filter_map(move |to| (from != to).then_some((from, to))))
            .map(|(from, to)| substrate.arrows_between(traces[from], traces[to]).len())
            .sum()
    }

    fn firings_at(entries: &[px0_physical_correspondence::TraceEntry], physical: u64) -> usize {
        entries
            .iter()
            .filter(|entry| entry.target_physical == physical && entry.fired)
            .count()
    }

    fn cell(physical_id: u64, position: i32, threshold: i32) -> CellSpec {
        CellSpec {
            physical_id,
            position,
            region: 0,
            threshold,
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
        eprintln!("PX3-R generic-opportunity PROBE v2 requires --preflight or --probe");
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
        println!("PX3_R_TRACE_COUPLING_GENERIC_OPPORTUNITY_PROBE_V2_PREFLIGHT_OK");
        return;
    }
    eprintln!("PX3_R_TRACE_COUPLING_GENERIC_OPPORTUNITY_PROBE_V2_EVIDENCE_SPENT");
    let report = run_probe();
    write_atomic(&report);
    println!(
        "PX3_R_TRACE_COUPLING_GENERIC_OPPORTUNITY_PROBE_V2_{}",
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
    let controls = source_audit()
        && actual.trace_firings == [RECURRENCES; ROUTES]
        && actual.source_firings == [RECURRENCES; ROUTES]
        && actual.quiescent
        && reference.quiescent
        && actual.extra_source_firings == 0
        && reference.extra_source_firings == 0
        && reference.passive_firings == 0
        && actual_duplicate_exact
        && reference_duplicate_exact;
    let collapse = controls
        && actual.proposals == 0
        && actual.local_arrows == 0
        && reference.proposals == 2
        && reference.local_arrows == 2;
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
        ("crates/px0-physical-correspondence/src/lib.rs", LAW_SHA256),
        (
            "crates/px0-physical-correspondence/examples/px3_r_trace_coupling_generic_opportunity_probe.rs",
            V1_SOURCE_SHA256,
        ),
        (
            "experiments/px3_r_trace_coupling_generic_opportunity_probe_v1_result_audit.md",
            V1_FAILURE_SHA256,
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
    let source = include_str!("px3_r_trace_coupling_generic_opportunity_probe_v2.rs");
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
        "cell,trace_firings,source_firings,passive_firings,proposals,arrows,local_arrows,complete_fingerprint,permanent_fingerprint,quiescent,extra_source_firings,duplicate_exact,queue_comparisons,spikes_delivered,generation_checks,state_updates,threshold_checks,firings,arrow_checks,spikes_emitted,local_eligibility_writes,local_return_updates,ordinary_pressure_updates,local_structural_proposals,physical_deallocations,work\n",
    );
    for (name, state, duplicate) in [
        (
            "actual-participation",
            &report.actual,
            report.actual_duplicate_exact,
        ),
        (
            "passive-reference",
            &report.reference,
            report.reference_duplicate_exact,
        ),
    ] {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            name,
            join_usize(&state.trace_firings),
            join_usize(&state.source_firings),
            state.passive_firings,
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
        "# PX3-R generic trace-coupling opportunity PROBE v2\n\nVerdict: **{}**.\n\nThis development PROBE freezes the generic-opportunity collapse; PX3 remains absent. Actual participation produced trace firings `{}`, generic proposals `{}`, and local ARROWs `{}`. The passive local reference produced proposals/ARROWs `{}/{}` without passive firing. Natural quiescence actual/reference: `{}/{}`; autonomous source refiring: `{}/{}`; complete duplicate replay: `{}/{}`; frozen source audit: `{}`. Ledgered work: `{}` operations.\n",
        report.classification,
        join_usize(&report.actual.trace_firings),
        report.actual.proposals,
        report.actual.local_arrows,
        report.reference.proposals,
        report.reference.local_arrows,
        report.actual.quiescent,
        report.reference.quiescent,
        report.actual.extra_source_firings,
        report.reference.extra_source_firings,
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
