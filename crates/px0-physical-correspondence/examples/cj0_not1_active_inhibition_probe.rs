use px0_physical_correspondence::{
    ArrowSpec, CellId, CellSpec, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX2_COMMIT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const PX0_SOURCE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX2_RESULT_SHA256: &str = "921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18";
const PROTOCOL_SHA256: &str = "486c761e83afe9f72e3d18516e623812697398c5b2bc1ac2a3cf789dadd12e44";
const RESULT_CSV: &str = "results/cj0_not1_active_inhibition_probe_v1.csv";
const RESULT_MD: &str = "results/cj0_not1_active_inhibition_probe_v1.md";
const STAGING_CSV: &str = "results/.cj0_not1_active_inhibition_probe_v1.csv.staging";
const STAGING_MD: &str = "results/.cj0_not1_active_inhibition_probe_v1.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    Absent,
    Timely,
    Late,
    Blocked,
    Stale,
}

impl Scenario {
    const ALL: [Self; 5] = [
        Self::Absent,
        Self::Timely,
        Self::Late,
        Self::Blocked,
        Self::Stale,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Absent => "a-absent",
            Self::Timely => "a-timely",
            Self::Late => "a-too-late",
            Self::Blocked => "a-blocked",
            Self::Stale => "a-stale",
        }
    }
}

#[derive(Clone)]
struct Fixture {
    substrate: PlasticSubstrate,
    a: CellId,
    b: CellId,
    a_physical: u64,
    b_physical: u64,
    integration_physical: u64,
    output_physical: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    a_firings: usize,
    b_firings: usize,
    integration_firings: usize,
    output_firings: usize,
    negative_arrivals: usize,
    negative_arrival_tick: i64,
    output_tick: i64,
    crossings: usize,
    naturally_quiescent: bool,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    work: WorkLedger,
    persistent_bytes: usize,
}

#[derive(Clone, Debug)]
struct Row {
    scenario: Scenario,
    mirror: bool,
    namespace: u64,
    observation: Observation,
    duplicate_exact: bool,
    passed: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args == ["--preflight"] {
        assert!(frozen_inputs_exact(), "frozen PX2 inputs must remain exact");
        assert_outputs_absent();
        println!("CJ0_NOT1_ACTIVE_INHIBITION_PROBE_V1_PREFLIGHT_OK");
        return;
    }
    if args != ["--probe"] {
        eprintln!("CJ0 NOT-1 PROBE requires --preflight or --probe");
        std::process::exit(2);
    }
    assert!(frozen_inputs_exact(), "frozen PX2 inputs must remain exact");
    assert_outputs_absent();
    eprintln!("CJ0_NOT1_ACTIVE_INHIBITION_PROBE_V1_EVIDENCE_SPENT");

    let mut rows = Vec::new();
    for mirror in [false, true] {
        for (index, scenario) in Scenario::ALL.into_iter().enumerate() {
            let namespace = 0x6_1100_0000
                + u64::from(mirror) * 0x0100_0000
                + u64::try_from(index).expect("bounded index") * 0x0010_0000;
            let fixture = build_fixture(namespace, mirror, scenario);
            let first = execute(fixture.clone(), scenario);
            let duplicate = execute(fixture, scenario);
            let duplicate_exact = first == duplicate;
            let passed = classify(scenario, &first) && duplicate_exact;
            rows.push(Row {
                scenario,
                mirror,
                namespace,
                observation: first,
                duplicate_exact,
                passed,
            });
        }
    }

    let passed = rows.len() == 10 && rows.iter().all(|row| row.passed);
    publish(&rows, passed);
    if !passed {
        std::process::exit(1);
    }
}

fn build_fixture(namespace: u64, mirror: bool, scenario: Scenario) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let sign = if mirror { -1 } else { 1 };
    let spec = |offset: u64, position: i32, region: i16, threshold: i32| CellSpec {
        physical_id: namespace + offset,
        position: position * sign,
        region,
        threshold,
        resistance: 100,
    };

    let (a, b, integration, output) = if mirror {
        let output = substrate.add_cell(spec(40, 30, 2, 1));
        let integration = substrate.add_cell(spec(30, 20, 1, 2));
        let b = substrate.add_cell(spec(20, 10, 0, 1));
        let a = substrate.add_cell(spec(10, 0, 0, 1));
        (a, b, integration, output)
    } else {
        let a = substrate.add_cell(spec(10, 0, 0, 1));
        let b = substrate.add_cell(spec(20, 10, 0, 1));
        let integration = substrate.add_cell(spec(30, 20, 1, 2));
        let output = substrate.add_cell(spec(40, 30, 2, 1));
        (a, b, integration, output)
    };

    let (a_delay, a_phase, a_resistance) = match scenario {
        Scenario::Late => (3, 0, 100),
        Scenario::Blocked => (1, 0, 0),
        Scenario::Stale => (11, 0, 1),
        Scenario::Absent | Scenario::Timely => (1, if mirror { 0 } else { -1 }, 100),
    };
    substrate.add_arrow(ArrowSpec {
        from: a,
        to: integration,
        delay: a_delay,
        phase: a_phase,
        coupling: -2,
        resistance: a_resistance,
    });
    substrate.add_arrow(ArrowSpec {
        from: b,
        to: integration,
        delay: 1,
        phase: 1,
        coupling: 2,
        resistance: 100,
    });
    substrate.add_arrow(ArrowSpec {
        from: integration,
        to: output,
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance: 100,
    });

    Fixture {
        substrate,
        a,
        b,
        a_physical: namespace + 10,
        b_physical: namespace + 20,
        integration_physical: namespace + 30,
        output_physical: namespace + 40,
    }
}

fn execute(mut fixture: Fixture, scenario: Scenario) -> Observation {
    if scenario != Scenario::Absent {
        fixture.substrate.enter(SpikeInput {
            arrival_tick: 0,
            phase: 0,
            origin_physical: fixture.a_physical + 1_000,
            target: fixture.a,
            impulse: 1,
        });
    }
    fixture.substrate.enter(SpikeInput {
        arrival_tick: 0,
        phase: 0,
        origin_physical: fixture.b_physical + 1_000,
        target: fixture.b,
        impulse: 1,
    });
    let run = fixture.substrate.propagate();
    let firing_count = |physical| {
        run.trace
            .iter()
            .filter(|entry| entry.target_physical == physical && entry.fired)
            .count()
    };
    let negative = run
        .trace
        .iter()
        .filter(|entry| entry.target_physical == fixture.integration_physical && entry.impulse < 0)
        .collect::<Vec<_>>();
    let output_tick = run
        .trace
        .iter()
        .find(|entry| entry.target_physical == fixture.output_physical && entry.fired)
        .map_or(-1, |entry| entry.tick);
    Observation {
        a_firings: firing_count(fixture.a_physical),
        b_firings: firing_count(fixture.b_physical),
        integration_firings: firing_count(fixture.integration_physical),
        output_firings: firing_count(fixture.output_physical),
        negative_arrivals: negative.len(),
        negative_arrival_tick: negative.first().map_or(-1, |entry| entry.tick),
        output_tick,
        crossings: run.crossings.len(),
        naturally_quiescent: run.naturally_quiescent,
        complete_fingerprint: fixture.substrate.complete_fingerprint(),
        permanent_fingerprint: fixture.substrate.permanent_fingerprint(),
        work: run.work,
        persistent_bytes: fixture.substrate.persistent_bytes(),
    }
}

fn classify(scenario: Scenario, observation: &Observation) -> bool {
    let common = observation.b_firings == 1
        && observation.naturally_quiescent
        && observation.work.total() > 0
        && observation.persistent_bytes > 0;
    common
        && match scenario {
            Scenario::Absent => {
                observation.a_firings == 0
                    && observation.negative_arrivals == 0
                    && observation.integration_firings == 1
                    && observation.output_firings == 1
                    && observation.output_tick == 2
            }
            Scenario::Timely => {
                observation.a_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.negative_arrival_tick == 1
                    && observation.integration_firings == 0
                    && observation.output_firings == 0
            }
            Scenario::Late => {
                observation.a_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.output_firings == 1
                    && observation.output_tick < observation.negative_arrival_tick
            }
            Scenario::Blocked => {
                observation.a_firings == 1
                    && observation.negative_arrivals == 0
                    && observation.integration_firings == 1
                    && observation.output_firings == 1
            }
            Scenario::Stale => {
                observation.a_firings == 1
                    && observation.negative_arrivals == 0
                    && observation.integration_firings == 1
                    && observation.output_firings == 1
                    && observation.work.physical_deallocations >= 1
            }
        }
}

fn frozen_inputs_exact() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == PX0_SOURCE_SHA256
        && sha256("results/px2_physical_causal_direction_definitive.csv") == PX2_RESULT_SHA256
        && sha256("experiments/cj0_not1_active_inhibition_probe_protocol.md")
            == PROTOCOL_SHA256
        && git_output(&["rev-parse", "2fbee861^{commit}"]) == PX2_COMMIT
        && git_output(&[
            "diff",
            "--name-only",
            PX2_COMMIT,
            "--",
            ":(exclude)experiments/cj0_not1_active_inhibition_probe_protocol.md",
            ":(exclude)experiments/cj0_not2_temporal_absence_probe_protocol.md",
            ":(exclude)experiments/cj0_not1_active_inhibition_probe_implementation_audit.md",
            ":(exclude)crates/px0-physical-correspondence/examples/cj0_not1_active_inhibition_probe.rs",
        ])
        .is_empty()
}

fn assert_outputs_absent() {
    for path in [RESULT_CSV, RESULT_MD, STAGING_CSV, STAGING_MD] {
        assert!(
            !Path::new(path).exists(),
            "evidence path already exists: {path}"
        );
    }
}

fn sha256(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("run sha256sum");
    assert!(output.status.success(), "sha256sum failed for {path}");
    String::from_utf8(output.stdout)
        .expect("sha256 output")
        .split_whitespace()
        .next()
        .expect("sha256 digest")
        .to_string()
}

fn git_output(args: &[&str]) -> String {
    let output = Command::new("git").args(args).output().expect("run git");
    assert!(output.status.success(), "git command failed: {args:?}");
    String::from_utf8(output.stdout)
        .expect("git output")
        .trim()
        .to_string()
}

fn publish(rows: &[Row], passed: bool) {
    let mut csv = String::from(
        "scenario,mirror,namespace,a_firings,b_firings,integration_firings,output_firings,negative_arrivals,negative_arrival_tick,output_tick,crossings,quiescent,complete_fingerprint,permanent_fingerprint,work,persistent_bytes,duplicate_exact,passed\n",
    );
    for row in rows {
        let observation = &row.observation;
        csv.push_str(&format!(
            "{},{},{:#x},{},{},{},{},{},{},{},{},{},{:#x},{:#x},{},{},{},{}\n",
            row.scenario.name(),
            row.mirror,
            row.namespace,
            observation.a_firings,
            observation.b_firings,
            observation.integration_firings,
            observation.output_firings,
            observation.negative_arrivals,
            observation.negative_arrival_tick,
            observation.output_tick,
            observation.crossings,
            observation.naturally_quiescent,
            observation.complete_fingerprint,
            observation.permanent_fingerprint,
            observation.work.total(),
            observation.persistent_bytes,
            row.duplicate_exact,
            row.passed,
        ));
    }
    let report = format!(
        "# CJ0-NOT-1 active-inhibition PROBE v1\n\nClassification: **{}**.\n\nRows passing: `{}/{}`. Normal/mirror, absent/timely/late/blocked/stale, exact replay, natural quiescence, work, storage, and fingerprints are serialized in the CSV.\n\nThis result tests existing signed coupling only. It adds no logical NOT primitive, changes no PX0-PX2 byte, reinterprets no PX3 result, and advances no authority.\n",
        if passed { "POSITIVE" } else { "NEGATIVE" },
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
    );
    write_new(STAGING_CSV, csv.as_bytes());
    write_new(STAGING_MD, report.as_bytes());
    rename(STAGING_CSV, RESULT_CSV).expect("publish CSV");
    rename(STAGING_MD, RESULT_MD).expect("publish report");
}

fn write_new(path: &str, bytes: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap_or_else(|error| panic!("create {path}: {error}"));
    file.write_all(bytes)
        .unwrap_or_else(|error| panic!("write {path}: {error}"));
    file.sync_all()
        .unwrap_or_else(|error| panic!("sync {path}: {error}"));
}
