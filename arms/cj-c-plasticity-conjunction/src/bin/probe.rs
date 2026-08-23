use cj_c_plasticity_conjunction::{
    ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PROTOCOL: &str = "adc58816e9f78634879c16140de2bb508c5f11261a13d871063e1aa0d68d019f";
const AUTHORITY: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const CSV: &str = "results/cj0_c_plasticity_conjunction_probe_v1.csv";
const REPORT: &str = "results/cj0_c_plasticity_conjunction_probe_v1.md";
const CSV_TMP: &str = "results/.cj0_c_plasticity_conjunction_probe_v1.csv.staging";
const REPORT_TMP: &str = "results/.cj0_c_plasticity_conjunction_probe_v1.md.staging";
const ROUNDS: usize = 16;
const REVERSAL_ROUNDS: usize = 40;

#[derive(Clone, Copy)]
struct Config {
    label: &'static str,
    namespace: u64,
    position: i32,
    reverse_cells: bool,
    reverse_matter: bool,
    reverse_origins: bool,
}

const CONFIGS: [Config; 4] = [
    Config {
        label: "primary",
        namespace: 0xc100_0000,
        position: 0,
        reverse_cells: false,
        reverse_matter: false,
        reverse_origins: false,
    },
    Config {
        label: "mirror",
        namespace: 0xc200_0000,
        position: 17,
        reverse_cells: true,
        reverse_matter: false,
        reverse_origins: true,
    },
    Config {
        label: "allocation-reverse",
        namespace: 0xc300_0000,
        position: -31,
        reverse_cells: false,
        reverse_matter: true,
        reverse_origins: true,
    },
    Config {
        label: "permuted",
        namespace: 0xc400_0000,
        position: 103,
        reverse_cells: true,
        reverse_matter: true,
        reverse_origins: false,
    },
];

#[derive(Clone)]
struct World {
    matter: PlasticSubstrate,
    drivers: [CellId; 4],
    contributors: [u64; 4],
    reservoir: BTreeSet<u64>,
    origins: [u64; 4],
}

#[derive(Default)]
struct Totals {
    work: u64,
    firings: [usize; 4],
    quiescent: bool,
}

#[derive(Clone)]
struct Row {
    label: &'static str,
    passed: bool,
    blank_fp: u64,
    acquired_fp: u64,
    reversed_fp: u64,
    bootstrap_fp: u64,
    learned_targets: String,
    changed_targets: String,
    acquired_resistance: u32,
    old_final_resistance: u32,
    trained_outputs: usize,
    crossed_outputs: usize,
    singleton_outputs: usize,
    changed_outputs: usize,
    old_outputs: usize,
    full_deallocation: bool,
    bootstrap: bool,
    self_evidence: bool,
    controls: bool,
    ambiguity: bool,
    replay: bool,
    marginals: bool,
    quiescent: bool,
    work: u64,
    persistent_bytes: usize,
    arrow_count: usize,
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 || args[1] != "--protocol" || args[2] != PROTOCOL {
        eprintln!("usage: probe --protocol {PROTOCOL}");
        std::process::exit(2);
    }
    refuse_existing();
    preflight();

    let rows = CONFIGS.map(run_cell);
    let passed = rows.iter().all(|row| row.passed);
    let csv = render_csv(&rows);
    let report = render_report(&rows, passed);
    atomic_write(CSV_TMP, CSV, &csv);
    atomic_write(REPORT_TMP, REPORT, &report);
    println!(
        "CJ0-C PROBE v1 {} cells={}/{} protocol={} authority={}",
        if passed {
            "PASS"
        } else {
            "FIRST_CLAUSE_FAILURE"
        },
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
        PROTOCOL,
        AUTHORITY
    );
    if !passed {
        std::process::exit(1);
    }
}

fn build(config: Config, reservoir_count: usize) -> World {
    let mut matter = PlasticSubstrate::new();
    let mut drivers = [None; 4];
    let mut contributors = [0; 4];
    let mut route_order = [0usize, 1, 2, 3];
    if config.reverse_cells {
        route_order.reverse();
    }
    for route in route_order {
        let driver = matter.add_cell(CellSpec {
            physical_id: config.namespace + 0x100 + route as u64,
            position: config.position + 100 + route as i32 * 10,
            region: 0,
            threshold: 1,
            resistance: 10_000,
        });
        let contributor_physical = config.namespace + 0x200 + route as u64;
        let contributor = matter.add_cell(CellSpec {
            physical_id: contributor_physical,
            position: config.position,
            region: 0,
            threshold: 1,
            resistance: 10_000,
        });
        matter.add_arrow(ArrowSpec {
            from: driver,
            to: contributor,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: 10_000,
        });
        drivers[route] = Some(driver);
        contributors[route] = contributor_physical;
    }
    let mut matter_order = (0..reservoir_count).collect::<Vec<_>>();
    if config.reverse_matter {
        matter_order.reverse();
    }
    let mut reservoir = BTreeSet::new();
    for slot in matter_order {
        let physical = config.namespace + 0x1000 + slot as u64;
        matter.add_cell(CellSpec {
            physical_id: physical,
            position: config.position,
            region: 0,
            threshold: 4,
            resistance: 10_000,
        });
        reservoir.insert(physical);
    }
    let mut origins = [
        config.namespace + 0x5001,
        config.namespace + 0x5002,
        config.namespace + 0x5003,
        config.namespace + 0x5004,
    ];
    if config.reverse_origins {
        origins.reverse();
    }
    World {
        matter,
        drivers: drivers.map(Option::unwrap),
        contributors,
        reservoir,
        origins,
    }
}

fn enter(world: &mut World, routes: &[usize], tick: i64) -> Execution {
    for route in routes {
        world.matter.enter(SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: world.origins[*route],
            target: world.drivers[*route],
            impulse: 1,
        });
    }
    world.matter.propagate_cj_c()
}

fn schedule(world: &mut World, first: [[usize; 2]; 2], rounds: usize, start: i64) -> Totals {
    let mut totals = Totals {
        quiescent: true,
        ..Totals::default()
    };
    for round in 0..rounds {
        let base = start + round as i64 * 8;
        let order = if round.is_multiple_of(2) {
            [0, 1]
        } else {
            [1, 0]
        };
        for (offset, item) in order.into_iter().enumerate() {
            let run = enter(world, &first[item], base + offset as i64 * 4);
            totals.work += run.work.total();
            totals.quiescent &= run.naturally_quiescent;
            for (route, physical) in world.contributors.iter().enumerate() {
                totals.firings[route] += run
                    .trace
                    .iter()
                    .filter(|entry| entry.fired && entry.target_physical == *physical)
                    .count();
            }
        }
    }
    totals
}

fn next_start(world: &World) -> i64 {
    ((world.matter.tick() + 7) / 8) * 8
}

fn output_count(world: &World, run: &Execution) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.fired && world.reservoir.contains(&entry.target_physical))
        .count()
}

fn observe(world: &World, clusters: [[usize; 2]; 2]) -> (usize, bool, u64) {
    let mut clone = world.clone();
    let start = next_start(&clone);
    let first = enter(&mut clone, &clusters[0], start);
    let second = enter(&mut clone, &clusters[1], start + 4);
    (
        output_count(&clone, &first) + output_count(&clone, &second),
        first.naturally_quiescent && second.naturally_quiescent,
        clone.matter.permanent_fingerprint(),
    )
}

fn singleton_outputs(world: &World) -> (usize, bool) {
    let mut total = 0;
    let mut quiet = true;
    for route in 0..4 {
        let mut clone = world.clone();
        let tick = next_start(&clone);
        let run = enter(&mut clone, &[route], tick);
        total += output_count(&clone, &run);
        quiet &= run.naturally_quiescent;
    }
    (total, quiet)
}

fn candidate_edges(world: &World) -> Vec<cj_c_plasticity_conjunction::matter::ArrowState> {
    world
        .matter
        .arrow_states()
        .into_iter()
        .filter(|arrow| arrow.coupling == 2)
        .collect()
}

fn target_for(world: &World, left: usize, right: usize) -> Option<u64> {
    let edges = candidate_edges(world);
    let left_targets = edges
        .iter()
        .filter(|arrow| arrow.live && arrow.from_physical == world.contributors[left])
        .map(|arrow| arrow.to_physical)
        .collect::<BTreeSet<_>>();
    edges
        .iter()
        .find(|arrow| {
            arrow.live
                && arrow.from_physical == world.contributors[right]
                && left_targets.contains(&arrow.to_physical)
        })
        .map(|arrow| arrow.to_physical)
}

fn target_text(world: &World, organizations: [[usize; 2]; 2]) -> String {
    organizations
        .iter()
        .map(|item| {
            target_for(world, item[0], item[1])
                .map(|target| format!("{target:x}"))
                .unwrap_or_else(|| "none".to_string())
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn resistance_sum(world: &World, organizations: [[usize; 2]; 2]) -> u32 {
    let targets = organizations
        .iter()
        .filter_map(|item| target_for(world, item[0], item[1]))
        .collect::<BTreeSet<_>>();
    candidate_edges(world)
        .iter()
        .filter(|arrow| arrow.live && targets.contains(&arrow.to_physical))
        .map(|arrow| arrow.resistance)
        .sum()
}

fn self_evidence_control(world: &World, old_target: u64) -> (bool, u64) {
    let mut clone = world.clone();
    let before = candidate_edges(&clone)
        .into_iter()
        .filter(|arrow| arrow.live && arrow.to_physical == old_target)
        .map(|arrow| arrow.resistance)
        .sum::<u32>();
    let tick = next_start(&clone);
    let run = enter(&mut clone, &[0, 3], tick);
    let target_fired = run
        .trace
        .iter()
        .any(|entry| entry.fired && entry.target_physical == old_target);
    let target_was_subthreshold = run
        .trace
        .iter()
        .any(|entry| !entry.fired && entry.target_physical == old_target);
    let after = candidate_edges(&clone)
        .into_iter()
        .filter(|arrow| arrow.live && arrow.to_physical == old_target)
        .map(|arrow| arrow.resistance)
        .sum::<u32>();
    let old_became_source = candidate_edges(&clone)
        .iter()
        .any(|arrow| arrow.from_physical == old_target);
    (
        target_was_subthreshold && !target_fired && !old_became_source && after <= before,
        run.work.total(),
    )
}

fn common_controls(config: Config, trained: &World) -> (bool, bool, u64) {
    let mut work = 0;
    let mut late = trained.clone();
    let tick = next_start(&late);
    let first = enter(&mut late, &[0], tick);
    let second = enter(&mut late, &[1], tick + 4);
    work += first.work.total() + second.work.total();
    let late_silent = output_count(&late, &first) + output_count(&late, &second) == 0;

    let mut absent = build(config, 0);
    let absent_run = enter(&mut absent, &[0, 1], 0);
    work += absent_run.work.total();
    let absent_silent = candidate_edges(&absent).is_empty();

    let mut stale = trained.clone();
    work += stale
        .matter
        .advance_time(stale.matter.tick() + 1000)
        .total();
    let stale_dead = candidate_edges(&stale).iter().all(|arrow| !arrow.live);

    let mut correlation = PlasticSubstrate::new();
    let x = correlation.add_cell(CellSpec {
        physical_id: config.namespace + 0x9000,
        position: config.position + 400,
        region: 0,
        threshold: 1,
        resistance: 100,
    });
    let y = correlation.add_cell(CellSpec {
        physical_id: config.namespace + 0x9001,
        position: config.position + 400,
        region: 0,
        threshold: 1,
        resistance: 100,
    });
    correlation.enter(SpikeInput {
        arrival_tick: 0,
        phase: 0,
        origin_physical: 1,
        target: x,
        impulse: 1,
    });
    correlation.enter(SpikeInput {
        arrival_tick: 0,
        phase: 0,
        origin_physical: 2,
        target: y,
        impulse: 1,
    });
    let correlated = correlation.propagate_cj_c();
    work += correlated.work.total();
    let correlation_silent = correlation.arrow_count() == 0;

    let ambiguity_three = ambiguity_count(config, 3);
    let ambiguity_four = ambiguity_count(config, 4);
    work += ambiguity_three.1 + ambiguity_four.1;
    let ambiguity = ambiguity_three.0 == (3, 6) && ambiguity_four.0 == (6, 12);
    (
        late_silent
            && absent_silent
            && stale_dead
            && correlation_silent
            && correlated.naturally_quiescent,
        ambiguity,
        work,
    )
}

fn ambiguity_count(config: Config, count: usize) -> ((usize, usize), u64) {
    let mut world = build(config, 8);
    let routes = (0..count).collect::<Vec<_>>();
    let first = enter(&mut world, &routes, 0);
    let second = enter(&mut world, &routes, 4);
    let edges = candidate_edges(&world);
    let targets = edges
        .iter()
        .filter(|arrow| arrow.live)
        .map(|arrow| arrow.to_physical)
        .collect::<BTreeSet<_>>();
    (
        (
            targets.len(),
            edges.iter().filter(|arrow| arrow.live).count(),
        ),
        first.work.total() + second.work.total(),
    )
}

fn run_cell(config: Config) -> Row {
    let initial = [[0, 1], [2, 3]];
    let changed = [[0, 3], [2, 1]];
    let mut world = build(config, 16);
    let blank_fp = world.matter.permanent_fingerprint();
    let acquisition = schedule(&mut world, initial, ROUNDS, 0);
    let acquired_fp = world.matter.permanent_fingerprint();
    let learned_targets = target_text(&world, initial);
    let acquired_resistance = resistance_sum(&world, initial);
    let trained = observe(&world, initial);
    let crossed = observe(&world, changed);
    let singletons = singleton_outputs(&world);
    let old_target = target_for(&world, 0, 1).unwrap_or(0);
    let self_evidence = self_evidence_control(&world, old_target);
    let controls = common_controls(config, &world);

    let replay_world = {
        let mut replay = build(config, 16);
        let replay_totals = schedule(&mut replay, initial, ROUNDS, 0);
        (replay, replay_totals)
    };
    let replay = replay_world.0.matter.permanent_fingerprint() == acquired_fp
        && replay_world.1.work == acquisition.work;

    let old_edges = candidate_edges(&world);
    let reversal_start = next_start(&world);
    let reversal = schedule(&mut world, changed, REVERSAL_ROUNDS, reversal_start);
    let reversed_fp = world.matter.permanent_fingerprint();
    let changed_targets = target_text(&world, changed);
    let old_final_resistance = resistance_sum(&world, initial);
    let changed_observation = observe(&world, changed);
    let old_observation = observe(&world, initial);
    let no_resurrection = old_edges.iter().all(|old| {
        world.matter.arrow_states().iter().any(|now| {
            now.from_physical == old.from_physical
                && now.to_physical == old.to_physical
                && now.generation == old.generation.wrapping_add(1)
                && !now.live
        })
    });

    let mut bootstrap = replay_world.0.clone();
    let deallocation_work = bootstrap
        .matter
        .advance_time(bootstrap.matter.tick() + 1000)
        .total();
    let full_deallocation = candidate_edges(&bootstrap).iter().all(|arrow| !arrow.live);
    let dead_before = candidate_edges(&bootstrap);
    let bootstrap_start = next_start(&bootstrap);
    let bootstrap_training = schedule(&mut bootstrap, changed, ROUNDS, bootstrap_start);
    let bootstrap_fp = bootstrap.matter.permanent_fingerprint();
    let bootstrap_observation = observe(&bootstrap, changed);
    let bootstrap_formed = full_deallocation
        && target_for(&bootstrap, 0, 3).is_some()
        && target_for(&bootstrap, 2, 1).is_some()
        && bootstrap_observation.0 == 2
        && dead_before.iter().all(|old| {
            bootstrap.matter.arrow_states().iter().any(|now| {
                now.from_physical == old.from_physical
                    && now.to_physical == old.to_physical
                    && now.generation == old.generation
                    && !now.live
            })
        });

    let marginals = acquisition.firings == [ROUNDS; 4]
        && reversal.firings == [REVERSAL_ROUNDS; 4]
        && bootstrap_training.firings == [ROUNDS; 4];
    let quiescent = acquisition.quiescent
        && reversal.quiescent
        && bootstrap_training.quiescent
        && trained.1
        && crossed.1
        && singletons.1
        && changed_observation.1
        && old_observation.1;
    let passed = target_for(&world, 0, 3).is_some()
        && target_for(&world, 2, 1).is_some()
        && learned_targets != "none|none"
        && acquired_resistance > 0
        && trained.0 == 2
        && crossed.0 == 0
        && singletons.0 == 0
        && old_final_resistance == 0
        && changed_observation.0 == 2
        && old_observation.0 == 0
        && no_resurrection
        && bootstrap_formed
        && self_evidence.0
        && controls.0
        && controls.1
        && replay
        && marginals
        && quiescent;

    Row {
        label: config.label,
        passed,
        blank_fp,
        acquired_fp,
        reversed_fp,
        bootstrap_fp,
        learned_targets,
        changed_targets,
        acquired_resistance,
        old_final_resistance,
        trained_outputs: trained.0,
        crossed_outputs: crossed.0,
        singleton_outputs: singletons.0,
        changed_outputs: changed_observation.0,
        old_outputs: old_observation.0,
        full_deallocation,
        bootstrap: bootstrap_formed,
        self_evidence: self_evidence.0,
        controls: controls.0,
        ambiguity: controls.1,
        replay,
        marginals,
        quiescent,
        work: acquisition.work
            + reversal.work
            + bootstrap_training.work
            + replay_world.1.work
            + deallocation_work
            + self_evidence.1
            + controls.2,
        persistent_bytes: world.matter.persistent_bytes(),
        arrow_count: world.matter.arrow_count(),
    }
}

fn preflight() {
    assert_eq!(
        sha("crates/px0-physical-correspondence/src/lib.rs"),
        AUTHORITY
    );
    assert_eq!(
        sha("experiments/cj0_c_plasticity_conjunction_probe_v2_protocol.md"),
        PROTOCOL
    );
    let empty = PlasticSubstrate::new();
    assert_eq!(empty.arrow_count(), 0);
    assert_eq!(empty.persistent_bytes(), 0);
}

fn sha(path: &str) -> String {
    let output = Command::new("shasum")
        .args(["-a", "256", path])
        .output()
        .expect("run shasum");
    assert!(output.status.success(), "shasum failed for {path}");
    String::from_utf8(output.stdout)
        .expect("utf8 shasum")
        .split_whitespace()
        .next()
        .expect("sha field")
        .to_string()
}

fn refuse_existing() {
    for path in [CSV, REPORT, CSV_TMP, REPORT_TMP] {
        if Path::new(path).exists() {
            eprintln!("refusing existing artifact: {path}");
            std::process::exit(2);
        }
    }
}

fn atomic_write(staging: &str, final_path: &str, body: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(staging)
        .unwrap_or_else(|error| panic!("create {staging}: {error}"));
    file.write_all(body.as_bytes())
        .unwrap_or_else(|error| panic!("write {staging}: {error}"));
    file.sync_all()
        .unwrap_or_else(|error| panic!("sync {staging}: {error}"));
    rename(staging, final_path)
        .unwrap_or_else(|error| panic!("rename {staging} -> {final_path}: {error}"));
}

fn render_csv(rows: &[Row]) -> String {
    let mut out = String::from("protocol,mode,claim_eligible,cell,passed,blank_fp,acquired_fp,reversed_fp,bootstrap_fp,learned_targets,changed_targets,acquired_resistance,old_final_resistance,trained_outputs,crossed_outputs,singleton_outputs,changed_outputs,old_outputs,full_deallocation,bootstrap,self_evidence,controls,ambiguity,replay,marginals,quiescent,work,persistent_bytes,arrow_count\n");
    for row in rows {
        out.push_str(&format!(
            "{PROTOCOL},PROBE,false,{},{},{:016x},{:016x},{:016x},{:016x},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.label,
            row.passed,
            row.blank_fp,
            row.acquired_fp,
            row.reversed_fp,
            row.bootstrap_fp,
            row.learned_targets,
            row.changed_targets,
            row.acquired_resistance,
            row.old_final_resistance,
            row.trained_outputs,
            row.crossed_outputs,
            row.singleton_outputs,
            row.changed_outputs,
            row.old_outputs,
            row.full_deallocation,
            row.bootstrap,
            row.self_evidence,
            row.controls,
            row.ambiguity,
            row.replay,
            row.marginals,
            row.quiescent,
            row.work,
            row.persistent_bytes,
            row.arrow_count,
        ));
    }
    out
}

fn render_report(rows: &[Row], passed: bool) -> String {
    let mut totals = BTreeMap::new();
    totals.insert("cells", rows.len() as u64);
    totals.insert(
        "passed",
        rows.iter().filter(|row| row.passed).count() as u64,
    );
    totals.insert("work", rows.iter().map(|row| row.work).sum());
    totals.insert(
        "storage",
        rows.iter().map(|row| row.persistent_bytes as u64).sum(),
    );
    let mut out = format!(
        "# CJ0-C plasticity-only conjunction PROBE v1 result\n\nVerdict: **{}**. Development PROBE only; claim eligible: `false`.\n\nProtocol: `{PROTOCOL}`. Authority source: `{AUTHORITY}`. Cells: `{}/{}`. Ledgered work: `{}`. Final persistent bytes summed across cells: `{}`.\n\n| cell | learned targets | changed targets | trained/crossed/single | changed/old | old support | deallocated/bootstrap | self-evidence | controls/ambiguity | replay/marginals/quiescence | result |\n|---|---|---|---:|---:|---:|:---:|:---:|:---:|:---:|:---:|\n",
        if passed { "POSITIVE" } else { "FIRST_CLAUSE_FAILURE" },
        totals["passed"],
        totals["cells"],
        totals["work"],
        totals["storage"],
    );
    for row in rows {
        out.push_str(&format!(
            "| {} | `{}` | `{}` | {}/{}/{} | {}/{} | {} -> {} | {}/{} | {} | {}/{} | {}/{}/{} | {} |\n",
            row.label,
            row.learned_targets,
            row.changed_targets,
            row.trained_outputs,
            row.crossed_outputs,
            row.singleton_outputs,
            row.changed_outputs,
            row.old_outputs,
            row.acquired_resistance,
            row.old_final_resistance,
            row.full_deallocation,
            row.bootstrap,
            row.self_evidence,
            row.controls,
            row.ambiguity,
            row.replay,
            row.marginals,
            row.quiescent,
            if row.passed { "PASS" } else { "FAIL" },
        ));
    }
    out.push_str("\n## Classification\n\nThe exact preregistered law is retained only if every cell passes. A positive PROBE permits a separately preregistered MICRO; it creates no authority and spends no definitive evidence. The organism-visible retained state contains only ordinary CELL/ARROW matter and existing numeric substrate fields.\n");
    out
}
