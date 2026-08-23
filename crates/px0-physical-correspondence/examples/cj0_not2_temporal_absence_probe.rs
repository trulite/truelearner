use px0_physical_correspondence::{ArrowSpec, CellId, CellSpec, PlasticSubstrate, SpikeInput};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX2_COMMIT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const PX0_SOURCE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX2_RESULT_SHA256: &str = "921e433e3bf358e89e3f8f288b4ab0472e9503a2a3ac25fe037a2b7f6cf6eb18";
const PROTOCOL_SHA256: &str = "b736efa9203740ea0932c7a2e997fc7fd2b583d2471f9856ff6061e8679ca498";
const RESULT_CSV: &str = "results/cj0_not2_temporal_absence_probe_v1.csv";
const RESULT_MD: &str = "results/cj0_not2_temporal_absence_probe_v1.md";
const STAGING_CSV: &str = "results/.cj0_not2_temporal_absence_probe_v1.csv.staging";
const STAGING_MD: &str = "results/.cj0_not2_temporal_absence_probe_v1.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    Absent,
    InWindow,
    AtClosure,
    AfterClosure,
    Blocked,
    Stale,
}

impl Scenario {
    const ALL: [Self; 6] = [
        Self::Absent,
        Self::InWindow,
        Self::AtClosure,
        Self::AfterClosure,
        Self::Blocked,
        Self::Stale,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Absent => "b-absent-through-closure",
            Self::InWindow => "b-in-window",
            Self::AtClosure => "b-at-closure-before-arrival",
            Self::AfterClosure => "b-after-closure",
            Self::Blocked => "b-blocked",
            Self::Stale => "b-stale",
        }
    }
}

#[derive(Clone)]
struct Fixture {
    substrate: PlasticSubstrate,
    trigger: CellId,
    b: CellId,
    closure: CellId,
    trigger_physical: u64,
    b_physical: u64,
    closure_physical: u64,
    transient_physical: u64,
    output_physical: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    initial_fingerprint: u64,
    after_trigger_fingerprint: u64,
    trigger_firings: usize,
    b_firings: usize,
    closure_firings: usize,
    transient_firings: usize,
    output_firings: usize,
    positive_state_arrivals: usize,
    negative_arrivals: usize,
    negative_arrival_tick: i64,
    closure_arrival_tick: i64,
    output_tick: i64,
    trigger_quiescent: bool,
    final_quiescent: bool,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    work_total: u64,
    physical_deallocations: u64,
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
        println!("CJ0_NOT2_TEMPORAL_ABSENCE_PROBE_V1_PREFLIGHT_OK");
        return;
    }
    if args != ["--probe"] {
        eprintln!("CJ0 NOT-2 PROBE requires --preflight or --probe");
        std::process::exit(2);
    }
    assert!(frozen_inputs_exact(), "frozen PX2 inputs must remain exact");
    assert_outputs_absent();
    eprintln!("CJ0_NOT2_TEMPORAL_ABSENCE_PROBE_V1_EVIDENCE_SPENT");

    let mut rows = Vec::new();
    for mirror in [false, true] {
        for (index, scenario) in Scenario::ALL.into_iter().enumerate() {
            let namespace = 0x6_2100_0000
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
    let passed = rows.len() == 12 && rows.iter().all(|row| row.passed);
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
    let (trigger, b, closure, transient, output) = if mirror {
        let output = substrate.add_cell(spec(50, 40, 2, 1));
        let transient = substrate.add_cell(spec(40, 30, 1, 3));
        let closure = substrate.add_cell(spec(30, 20, 0, 1));
        let b = substrate.add_cell(spec(20, 10, 0, 1));
        let trigger = substrate.add_cell(spec(10, 0, 0, 1));
        (trigger, b, closure, transient, output)
    } else {
        let trigger = substrate.add_cell(spec(10, 0, 0, 1));
        let b = substrate.add_cell(spec(20, 10, 0, 1));
        let closure = substrate.add_cell(spec(30, 20, 0, 1));
        let transient = substrate.add_cell(spec(40, 30, 1, 3));
        let output = substrate.add_cell(spec(50, 40, 2, 1));
        (trigger, b, closure, transient, output)
    };

    substrate.add_arrow(ArrowSpec {
        from: trigger,
        to: transient,
        delay: 1,
        phase: if mirror { 0 } else { -2 },
        coupling: 2,
        resistance: 100,
    });
    let (b_delay, b_phase, b_resistance) = match scenario {
        Scenario::AtClosure => (1, -1, 100),
        Scenario::AfterClosure => (2, 0, 100),
        Scenario::Blocked => (0, 1, 0),
        Scenario::Stale => (10, 0, 1),
        Scenario::Absent | Scenario::InWindow => (0, 1, 100),
    };
    substrate.add_arrow(ArrowSpec {
        from: b,
        to: transient,
        delay: b_delay,
        phase: b_phase,
        coupling: -2,
        resistance: b_resistance,
    });
    substrate.add_arrow(ArrowSpec {
        from: closure,
        to: transient,
        delay: 1,
        phase: 1,
        coupling: 2,
        resistance: 100,
    });
    substrate.add_arrow(ArrowSpec {
        from: transient,
        to: output,
        delay: 0,
        phase: 2,
        coupling: 1,
        resistance: 100,
    });

    Fixture {
        substrate,
        trigger,
        b,
        closure,
        trigger_physical: namespace + 10,
        b_physical: namespace + 20,
        closure_physical: namespace + 30,
        transient_physical: namespace + 40,
        output_physical: namespace + 50,
    }
}

fn execute(mut fixture: Fixture, scenario: Scenario) -> Observation {
    let initial_fingerprint = fixture.substrate.complete_fingerprint();
    fixture.substrate.enter(SpikeInput {
        arrival_tick: 0,
        phase: 0,
        origin_physical: fixture.trigger_physical + 1_000,
        target: fixture.trigger,
        impulse: 1,
    });
    let trigger_run = fixture.substrate.propagate();
    let after_trigger_fingerprint = fixture.substrate.complete_fingerprint();

    fixture.substrate.enter(SpikeInput {
        arrival_tick: 1,
        phase: 1,
        origin_physical: fixture.closure_physical + 1_000,
        target: fixture.closure,
        impulse: 1,
    });
    if scenario != Scenario::Absent {
        fixture.substrate.enter(SpikeInput {
            arrival_tick: 1,
            phase: 0,
            origin_physical: fixture.b_physical + 1_000,
            target: fixture.b,
            impulse: 1,
        });
    }
    let final_run = fixture.substrate.propagate();
    let firing_count = |physical| {
        trigger_run
            .trace
            .iter()
            .chain(&final_run.trace)
            .filter(|entry| entry.target_physical == physical && entry.fired)
            .count()
    };
    let transient_arrivals = trigger_run
        .trace
        .iter()
        .chain(&final_run.trace)
        .filter(|entry| entry.target_physical == fixture.transient_physical)
        .collect::<Vec<_>>();
    let negative = transient_arrivals
        .iter()
        .filter(|entry| entry.impulse < 0)
        .collect::<Vec<_>>();
    let closure_arrival_tick = transient_arrivals
        .iter()
        .find(|entry| entry.impulse == 2 && entry.tick == 2)
        .map_or(-1, |entry| entry.tick);
    let output_tick = final_run
        .trace
        .iter()
        .find(|entry| entry.target_physical == fixture.output_physical && entry.fired)
        .map_or(-1, |entry| entry.tick);
    Observation {
        initial_fingerprint,
        after_trigger_fingerprint,
        trigger_firings: firing_count(fixture.trigger_physical),
        b_firings: firing_count(fixture.b_physical),
        closure_firings: firing_count(fixture.closure_physical),
        transient_firings: firing_count(fixture.transient_physical),
        output_firings: firing_count(fixture.output_physical),
        positive_state_arrivals: transient_arrivals
            .iter()
            .filter(|entry| entry.impulse > 0)
            .count(),
        negative_arrivals: negative.len(),
        negative_arrival_tick: negative.first().map_or(-1, |entry| entry.tick),
        closure_arrival_tick,
        output_tick,
        trigger_quiescent: trigger_run.naturally_quiescent,
        final_quiescent: final_run.naturally_quiescent,
        complete_fingerprint: fixture.substrate.complete_fingerprint(),
        permanent_fingerprint: fixture.substrate.permanent_fingerprint(),
        work_total: trigger_run.work.total() + final_run.work.total(),
        physical_deallocations: trigger_run.work.physical_deallocations
            + final_run.work.physical_deallocations,
        persistent_bytes: fixture.substrate.persistent_bytes(),
    }
}

fn classify(scenario: Scenario, observation: &Observation) -> bool {
    let common = observation.initial_fingerprint != observation.after_trigger_fingerprint
        && observation.trigger_firings == 1
        && observation.closure_firings == 1
        && observation.positive_state_arrivals == 2
        && observation.closure_arrival_tick == 2
        && observation.trigger_quiescent
        && observation.final_quiescent
        && observation.work_total > 0
        && observation.persistent_bytes > 0;
    common
        && match scenario {
            Scenario::Absent => {
                observation.b_firings == 0
                    && observation.negative_arrivals == 0
                    && observation.transient_firings == 1
                    && observation.output_firings == 1
                    && observation.output_tick == 2
            }
            Scenario::InWindow => {
                observation.b_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.negative_arrival_tick == 1
                    && observation.transient_firings == 0
                    && observation.output_firings == 0
            }
            Scenario::AtClosure => {
                observation.b_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.negative_arrival_tick == observation.closure_arrival_tick
                    && observation.transient_firings == 0
                    && observation.output_firings == 0
            }
            Scenario::AfterClosure => {
                observation.b_firings == 1
                    && observation.negative_arrivals == 1
                    && observation.output_firings == 1
                    && observation.output_tick < observation.negative_arrival_tick
            }
            Scenario::Blocked => {
                observation.b_firings == 1
                    && observation.negative_arrivals == 0
                    && observation.transient_firings == 1
                    && observation.output_firings == 1
            }
            Scenario::Stale => {
                observation.b_firings == 1
                    && observation.negative_arrivals == 0
                    && observation.transient_firings == 1
                    && observation.output_firings == 1
                    && observation.physical_deallocations >= 1
            }
        }
}

fn frozen_inputs_exact() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == PX0_SOURCE_SHA256
        && sha256("results/px2_physical_causal_direction_definitive.csv") == PX2_RESULT_SHA256
        && sha256("experiments/cj0_not2_temporal_absence_probe_protocol.md") == PROTOCOL_SHA256
        && git_output(&["rev-parse", "2fbee861^{commit}"]) == PX2_COMMIT
        && git_output(&[
            "diff",
            "--name-only",
            PX2_COMMIT,
            "--",
            "crates/px0-physical-correspondence/src/lib.rs",
            "results/px0_physical_correspondence_definitive.csv",
            "results/px1_physical_boundary_roles_definitive.csv",
            "results/px2_physical_causal_direction_definitive.csv",
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
        "scenario,mirror,namespace,initial_fingerprint,after_trigger_fingerprint,trigger_firings,b_firings,closure_firings,transient_firings,output_firings,positive_state_arrivals,negative_arrivals,negative_arrival_tick,closure_arrival_tick,output_tick,trigger_quiescent,final_quiescent,complete_fingerprint,permanent_fingerprint,work,physical_deallocations,persistent_bytes,duplicate_exact,passed\n",
    );
    for row in rows {
        let observation = &row.observation;
        csv.push_str(&format!(
            "{},{},{:#x},{:#x},{:#x},{},{},{},{},{},{},{},{},{},{},{},{},{:#x},{:#x},{},{},{},{},{}\n",
            row.scenario.name(),
            row.mirror,
            row.namespace,
            observation.initial_fingerprint,
            observation.after_trigger_fingerprint,
            observation.trigger_firings,
            observation.b_firings,
            observation.closure_firings,
            observation.transient_firings,
            observation.output_firings,
            observation.positive_state_arrivals,
            observation.negative_arrivals,
            observation.negative_arrival_tick,
            observation.closure_arrival_tick,
            observation.output_tick,
            observation.trigger_quiescent,
            observation.final_quiescent,
            observation.complete_fingerprint,
            observation.permanent_fingerprint,
            observation.work_total,
            observation.physical_deallocations,
            observation.persistent_bytes,
            row.duplicate_exact,
            row.passed,
        ));
    }
    let report = format!(
        "# CJ0-NOT-2 temporal-absence PROBE v1\n\nClassification: **{}**.\n\nRows passing: `{}/{}`. Trigger-state fingerprints, closure timing sweep, normal/mirror layout, absent/blocked/stale paths, exact replay, natural quiescence, work, storage, and final fingerprints are serialized in the CSV.\n\nThis result uses no absence symbol, timeout label, evaluator-selected branch, or new persistent variable. It changes no PX0-PX2 byte, reinterprets no PX3 result, and advances no authority.\n",
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
