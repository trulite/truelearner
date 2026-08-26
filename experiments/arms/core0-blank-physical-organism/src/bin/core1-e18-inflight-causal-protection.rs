#![forbid(unsafe_code)]

#[allow(dead_code)]
#[path = "core1-e17-symmetry-breaking-tournament.rs"]
mod e17;

use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use e17::{e18_observe, E18Observation};
use truelearner_core::MechanicalConfig;

const SEEDS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    FrozenR,
    InFlight,
    NoTrigger,
    GlobalLifetime,
}

impl Arm {
    const ALL: [Self; 4] = [
        Self::FrozenR,
        Self::InFlight,
        Self::NoTrigger,
        Self::GlobalLifetime,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::FrozenR => "A_frozen_e17_r",
            Self::InFlight => "B_in_flight",
            Self::NoTrigger => "C_no_self_trigger",
            Self::GlobalLifetime => "D_global_lifetime",
        }
    }

    fn law(self) -> (bool, bool, bool) {
        match self {
            Self::FrozenR => (false, false, true),
            Self::InFlight => (true, false, true),
            Self::NoTrigger => (true, false, false),
            Self::GlobalLifetime => (false, true, true),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    arm: Arm,
    observation: E18Observation,
}

fn execute(seed: usize, mechanics: MechanicalConfig, root: u64) -> Vec<Row> {
    Arm::ALL
        .into_iter()
        .map(|arm| {
            let (in_flight, protect_all, self_trigger) = arm.law();
            Row {
                arm,
                observation: e18_observe(
                    seed,
                    mechanics,
                    root.saturating_add((arm as u64) * 10_000),
                    in_flight,
                    protect_all,
                    self_trigger,
                ),
            }
        })
        .collect()
}

fn option(value: Option<u8>) -> String {
    value.map_or_else(|| "none".to_string(), |value| value.to_string())
}

fn list(values: &[u8]) -> String {
    values
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments
        .get(1)
        .is_some_and(|arg| arg.starts_with("--preflight-"))
    {
        let arm = match arguments[1].as_str() {
            "--preflight-a" => Arm::FrozenR,
            "--preflight-b" => Arm::InFlight,
            "--preflight-c" => Arm::NoTrigger,
            "--preflight-d" => Arm::GlobalLifetime,
            other => panic!("unknown E18 preflight arm {other}"),
        };
        let (in_flight, protect_all, self_trigger) = arm.law();
        let seed = arguments
            .get(2)
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(7);
        let mechanics = if arguments.get(3).is_some_and(|value| value == "production") {
            MechanicalConfig::PRODUCTION
        } else {
            MechanicalConfig::REFERENCE
        };
        println!(
            "{:#?}",
            e18_observe(
                seed,
                mechanics,
                108_000_000 + seed as u64 * 100_000 + arm as u64 * 10_000,
                in_flight,
                protect_all,
                self_trigger,
            )
        );
        return;
    }
    if arguments.get(1).is_some_and(|arg| arg == "--preflight") {
        println!(
            "{:#?}",
            execute(7, MechanicalConfig::REFERENCE, 108_700_000)
        );
        return;
    }

    eprintln!("CORE1_E18_INFLIGHT_CAUSAL_PROTECTION_V1_EVIDENCE_SPENT");
    let destination = arguments.get(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/core1_e18_inflight_causal_protection_v1")
    });
    fs::create_dir_all(&destination).expect("create E18 results");
    let mut csv = BufWriter::new(File::create(destination.join("matrix.csv")).expect("matrix"));
    writeln!(csv, "seed,mechanics,arm,useful,first_action,final_action,attempted_1,attempted_2,participated_1,participated_2,updates_1,updates_2,live_candidate_links,work,quiescent,replay_exact,mechanics_exact").expect("header");

    let mut first = [0_usize; 4];
    let mut learned = [0_usize; 4];
    let mut replay_exact = true;
    let mut mechanics_exact = true;
    let mut quiescent = true;
    let mut all_rows = Vec::new();
    for seed in 0..SEEDS {
        let root = 108_000_000_u64 + seed as u64 * 100_000;
        let reference = execute(seed, MechanicalConfig::REFERENCE, root);
        let replay = execute(seed, MechanicalConfig::REFERENCE, root);
        let production = execute(seed, MechanicalConfig::PRODUCTION, root);
        let row_replay = reference == replay;
        let row_mechanics = reference == production;
        replay_exact &= row_replay;
        mechanics_exact &= row_mechanics;
        for (index, row) in reference.iter().enumerate() {
            first[index] += usize::from(row.observation.first_action.is_some());
            learned[index] +=
                usize::from(row.observation.final_action == Some(row.observation.useful));
            quiescent &= row.observation.quiescent;
        }
        for (mechanics, rows) in [
            ("reference", &reference),
            ("replay", &replay),
            ("production", &production),
        ] {
            for row in rows {
                let o = &row.observation;
                writeln!(
                    csv,
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                    seed,
                    mechanics,
                    row.arm.name(),
                    o.useful,
                    option(o.first_action),
                    option(o.final_action),
                    list(&o.attempted[0]),
                    list(&o.attempted[1]),
                    list(&o.participated[0]),
                    list(&o.participated[1]),
                    o.consequence_updates[0],
                    o.consequence_updates[1],
                    o.live_candidate_links,
                    o.work,
                    o.quiescent,
                    row_replay,
                    row_mechanics,
                )
                .expect("row");
            }
        }
        eprintln!(
            "E18 seed={} A={}/{} B={}/{} C={}/{} D={}/{} replay={} mechanics={}",
            seed,
            option(reference[0].observation.first_action),
            option(reference[0].observation.final_action),
            option(reference[1].observation.first_action),
            option(reference[1].observation.final_action),
            option(reference[2].observation.first_action),
            option(reference[2].observation.final_action),
            option(reference[3].observation.first_action),
            option(reference[3].observation.final_action),
            row_replay,
            row_mechanics,
        );
        all_rows.push(reference);
    }
    csv.flush().expect("flush");

    let a_boundary = learned[0] == 2;
    let b_positive = first[1] == SEEDS && learned[1] == SEEDS;
    let c_silent = first[2] == 0 && learned[2] == 0;
    let d_overbroad = all_rows.iter().all(|rows| {
        rows[3].observation.live_candidate_links > rows[2].observation.live_candidate_links
    });
    let passed = a_boundary
        && b_positive
        && c_silent
        && d_overbroad
        && replay_exact
        && mechanics_exact
        && quiescent;
    let report = format!(
        "# CORE1 E18 in-flight causal protection v1\n\n| Arm | First action | Learned useful |\n|---|---:|---:|\n| A frozen E17-R | {}/8 | {}/8 |\n| B exact in-flight | {}/8 | {}/8 |\n| C no self-trigger | {}/8 | {}/8 |\n| D global lifetime | {}/8 | {}/8 |\n\n- A boundary exact: `{}`\n- B positive: `{}`\n- C silent: `{}`\n- D observably overbroad: `{}`\n- exact replay: `{}`\n- Reference/Production exact: `{}`\n- natural quiescence: `{}`\n- primary matrix: `{}`\n",
        first[0], learned[0], first[1], learned[1], first[2], learned[2], first[3],
        learned[3], a_boundary, b_positive, c_silent, d_overbroad, replay_exact,
        mechanics_exact, quiescent, passed,
    );
    fs::write(destination.join("report.md"), report).expect("report");
    println!(
        "CORE1_E18_INFLIGHT_CAUSAL_PROTECTION_V1_COMPLETE A={}|{} B={}|{} C={}|{} D={}|{} pass={} replay={} mechanics={} quiescent={}",
        first[0], learned[0], first[1], learned[1], first[2], learned[2], first[3],
        learned[3], passed, replay_exact, mechanics_exact, quiescent,
    );
    assert!(passed, "E18 primary matrix failed; inspect streamed rows");
}
