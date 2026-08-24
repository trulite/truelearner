#![forbid(unsafe_code)]

use lr1_route_aware_physical_return::{
    ArrowId, ArrowSpec, CellId, CellSpec, Crossing, Execution, PlasticSubstrate, SpikeInput,
    TraceEntry,
};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX0: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const LR0: &str = "96be2143b13eb42bbfe6fda418b312123824e9f57a8904ff89ca6fd64148e6ff";
const COLLAPSE: &str = "52072394fcf9867f23d1ec982f030fac6d5b5601c8f7294f178098743664b033";
const PROTOCOL: &str = "07d85eba99b731f776a6c5dec56dee59019d63667598f20118a857f01346924b";
const PARALLEL_PROTOCOL: &str = "ab7dae326f25cf12cdfb4b8d580f82c13f2afc95ef3000dba9e7d188db339860";

const CSV: &str = "results/lr1_three_factor_local_plasticity_arm_a_v1.csv";
const MD: &str = "results/lr1_three_factor_local_plasticity_arm_a_v1.md";
const CSV_STAGE: &str = "results/.lr1_three_factor_local_plasticity_arm_a_v1.csv.staging";
const MD_STAGE: &str = "results/.lr1_three_factor_local_plasticity_arm_a_v1.md.staging";
const SEEDS: [u64; 4] = [4101, 4109, 4111, 4127];
const NAMESPACE_BASE: u64 = 0x7_1100_0000_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    RenewedUpstreamNoReturn,
    CompletedDownstreamReturn,
    IndependentReturnNoCandidate,
    LateDownstreamReturn,
    SimultaneousUpstreamAndReturn,
    UpstreamExecutionOnly,
}

impl Kind {
    const ALL: [Self; 6] = [
        Self::RenewedUpstreamNoReturn,
        Self::CompletedDownstreamReturn,
        Self::IndependentReturnNoCandidate,
        Self::LateDownstreamReturn,
        Self::SimultaneousUpstreamAndReturn,
        Self::UpstreamExecutionOnly,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::RenewedUpstreamNoReturn => "renewed-upstream-no-return",
            Self::CompletedDownstreamReturn => "completed-downstream-return",
            Self::IndependentReturnNoCandidate => "independent-return-no-candidate",
            Self::LateDownstreamReturn => "late-downstream-return",
            Self::SimultaneousUpstreamAndReturn => "simultaneous-upstream-and-return",
            Self::UpstreamExecutionOnly => "upstream-execution-only",
        }
    }

    fn index(self) -> u64 {
        match self {
            Self::RenewedUpstreamNoReturn => 0,
            Self::CompletedDownstreamReturn => 1,
            Self::IndependentReturnNoCandidate => 2,
            Self::LateDownstreamReturn => 3,
            Self::SimultaneousUpstreamAndReturn => 4,
            Self::UpstreamExecutionOnly => 5,
        }
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    upstream: [CellId; 2],
    world: CellId,
    independent_x: CellId,
    upstream_arrows: [ArrowId; 2],
    candidate: ArrowId,
    return_arrow: ArrowId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    upstream_fires: [Vec<i64>; 2],
    p_fires: Vec<i64>,
    x_fires: Vec<i64>,
    world_fires: Vec<i64>,
    return_source_fires: Vec<i64>,
    upstream_crossings: [usize; 2],
    candidate_crossings: usize,
    effect_crossings: usize,
    return_crossings: usize,
    incoming_routes: String,
    candidate_resistance: u32,
    candidate_coupling: i32,
    candidate_live: bool,
    candidate_eligible_until: Option<i64>,
    return_updates: u64,
    qualification_checks: u64,
    qualification_accepts: u64,
    path_edges: u64,
    work: u64,
    bytes: usize,
    fingerprint: u64,
    permanent: u64,
    quiescent: bool,
    validity: [bool; 8],
    claim: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    stratum: &'static str,
    kind: Kind,
    namespace: u64,
    observation: Observation,
    replay: bool,
    passed: bool,
}

fn main() {
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [argument] if argument == "--preflight" => {
            audit();
            surface();
            absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
            println!("LR1_THREE_FACTOR_LOCAL_PLASTICITY_ARM_A_PREFLIGHT_OK");
        }
        [argument] if argument == "--lr1-a" => {
            audit();
            surface();
            absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
            evidence();
        }
        _ => std::process::exit(2),
    }
}

fn evidence() {
    eprintln!("LR1_THREE_FACTOR_LOCAL_PLASTICITY_ARM_A_EVIDENCE_SPENT");
    let rows = SEEDS
        .into_iter()
        .flat_map(|seed| Kind::ALL.map(|kind| replay(seed, kind)))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 24);
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(MD_STAGE, MD, &report(&rows));
}

fn audit() {
    for (path, expected) in [
        ("crates/px0-physical-correspondence/src/lib.rs", PX0),
        (
            "experiments/lr0_d0_qualified_physical_return_information_sufficiency_audit_v1.md",
            LR0,
        ),
        (
            "experiments/px3_r6_integrated_preflight_collapse_audit_v1.md",
            COLLAPSE,
        ),
        (
            "experiments/lr1_three_factor_local_plasticity_arm_a_protocol_v1.md",
            PROTOCOL,
        ),
        (
            "experiments/lr1_three_factor_local_plasticity_parallel_arms_protocol_v1.md",
            PARALLEL_PROTOCOL,
        ),
    ] {
        assert_eq!(sha(path), expected, "frozen input changed: {path}");
    }
}

fn surface() {
    assert_eq!(Kind::ALL.into_iter().collect::<BTreeSet<_>>().len(), 6);
    assert_eq!(SEEDS.into_iter().collect::<BTreeSet<_>>().len(), 4);
    let namespaces = SEEDS
        .into_iter()
        .flat_map(|seed| Kind::ALL.map(|kind| namespace(seed, kind)))
        .collect::<BTreeSet<_>>();
    assert_eq!(namespaces.len(), 24);
    assert!(namespaces.iter().all(|value| *value >= NAMESPACE_BASE));
}

fn replay(seed: u64, kind: Kind) -> Row {
    let first = run(seed, kind);
    let second = run(seed, kind);
    let exact = first == second;
    let mut row = first;
    row.replay = exact;
    row.passed =
        exact && row.observation.validity.into_iter().all(|value| value) && row.observation.claim;
    row
}

fn run(seed: u64, kind: Kind) -> Row {
    let stratum_index = usize::try_from(seed % 4).expect("small stratum");
    let reverse = stratum_index == 1 || stratum_index == 3;
    let reflect = stratum_index >= 2;
    let namespace = namespace(seed, kind);
    let mut world = build(
        namespace,
        reverse,
        reflect,
        kind == Kind::LateDownstreamReturn,
    );
    schedule(&mut world, kind);
    let execution = world.substrate.propagate();
    observe(world, execution, seed, kind, stratum_index)
}

fn schedule(world: &mut World, kind: Kind) {
    match kind {
        Kind::RenewedUpstreamNoReturn => {
            drive_p(world, 0);
            drive_p(world, 2);
        }
        Kind::CompletedDownstreamReturn => {
            drive_p(world, 0);
            pulse(&mut world.substrate, world.world, 1, 1, 200);
        }
        Kind::IndependentReturnNoCandidate => {
            pulse(&mut world.substrate, world.independent_x, 0, 1, 300);
            pulse(&mut world.substrate, world.world, 1, 1, 200);
        }
        Kind::LateDownstreamReturn => {
            drive_p(world, 0);
            pulse(&mut world.substrate, world.world, 1, 1, 200);
        }
        Kind::SimultaneousUpstreamAndReturn => {
            drive_p(world, 0);
            pulse(&mut world.substrate, world.world, 1, 1, 200);
            drive_p(world, 2);
        }
        Kind::UpstreamExecutionOnly => drive_p(world, 0),
    }
}

fn drive_p(world: &mut World, tick: i64) {
    for side in 0..2 {
        pulse(
            &mut world.substrate,
            world.upstream[side],
            tick,
            1,
            side as i32,
        );
    }
}

fn observe(world: World, execution: Execution, seed: u64, kind: Kind, stratum_index: usize) -> Row {
    let trace = &execution.trace;
    let crossings = &execution.crossings;
    let upstream_fires = [
        firing_ticks(trace, world.namespace, 10),
        firing_ticks(trace, world.namespace, 11),
    ];
    let p_fires = firing_ticks(trace, world.namespace, 20);
    let x_fires = firing_ticks(trace, world.namespace, 30);
    let world_fires = firing_ticks(trace, world.namespace, 40);
    let return_source_fires = firing_ticks(trace, world.namespace, 50);
    let upstream_crossings = [
        count_crossings(crossings, world.namespace, 10, 20),
        count_crossings(crossings, world.namespace, 11, 20),
    ];
    let candidate_crossings = count_crossings(crossings, world.namespace, 20, 30);
    let effect_crossings = count_crossings(crossings, world.namespace, 30, 50);
    let return_crossings = count_crossings(crossings, world.namespace, 50, 20);
    let incoming_routes = format!(
        "u0={:?}:{:?}:c{}:d0:g{}|u1={:?}:{:?}:c{}:d0:g{}|r={:?}:{:?}:c{}:d{}:g{}",
        world.upstream_arrows[0],
        world.substrate.arrow_endpoints(world.upstream_arrows[0]),
        world.substrate.arrow_coupling(world.upstream_arrows[0]),
        world.substrate.arrow_generation(world.upstream_arrows[0]),
        world.upstream_arrows[1],
        world.substrate.arrow_endpoints(world.upstream_arrows[1]),
        world.substrate.arrow_coupling(world.upstream_arrows[1]),
        world.substrate.arrow_generation(world.upstream_arrows[1]),
        world.return_arrow,
        world.substrate.arrow_endpoints(world.return_arrow),
        world.substrate.arrow_coupling(world.return_arrow),
        if kind == Kind::LateDownstreamReturn {
            5
        } else {
            1
        },
        world.substrate.arrow_generation(world.return_arrow),
    );
    let candidate_resistance = world.substrate.arrow_resistance(world.candidate);
    let candidate_coupling = world.substrate.arrow_coupling(world.candidate);
    let candidate_live = world.substrate.arrow_is_live(world.candidate);
    let candidate_eligible_until = world.substrate.arrow_eligible_until(world.candidate);
    let validity = [
        upstream_crossings == [upstream_fires[0].len(), upstream_fires[1].len()],
        candidate_crossings == p_fires.len(),
        effect_crossings == x_fires.len(),
        return_crossings == return_source_fires.len(),
        world.substrate.arrow_coupling(world.upstream_arrows[0]) == 1
            && world.substrate.arrow_coupling(world.upstream_arrows[1]) == 1
            && world.substrate.arrow_coupling(world.return_arrow) == 1,
        world.substrate.arrow_generation(world.candidate)
            == if kind == Kind::LateDownstreamReturn {
                2
            } else {
                1
            },
        execution.naturally_quiescent,
        execution.work.local_structural_proposals == 0,
    ];
    let claim = claim(
        kind,
        &p_fires,
        candidate_crossings,
        return_crossings,
        candidate_resistance,
        candidate_coupling,
        execution.work.local_return_updates,
        execution.work.qualified_return_accepts,
    );
    let observation = Observation {
        upstream_fires,
        p_fires,
        x_fires,
        world_fires,
        return_source_fires,
        upstream_crossings,
        candidate_crossings,
        effect_crossings,
        return_crossings,
        incoming_routes,
        candidate_resistance,
        candidate_coupling,
        candidate_live,
        candidate_eligible_until,
        return_updates: execution.work.local_return_updates,
        qualification_checks: execution.work.qualified_return_checks,
        qualification_accepts: execution.work.qualified_return_accepts,
        path_edges: execution.work.qualified_return_path_edges,
        work: execution.work.total(),
        bytes: world.substrate.persistent_bytes(),
        fingerprint: execution.end_fingerprint,
        permanent: execution.permanent_fingerprint,
        quiescent: execution.naturally_quiescent,
        validity,
        claim,
    };
    Row {
        seed,
        stratum: ["G0", "G1", "G2", "G3"][stratum_index],
        kind,
        namespace: world.namespace,
        passed: observation.validity.into_iter().all(|value| value) && observation.claim,
        observation,
        replay: false,
    }
}

#[allow(clippy::too_many_arguments)]
fn claim(
    kind: Kind,
    p_fires: &[i64],
    candidate_crossings: usize,
    return_crossings: usize,
    resistance: u32,
    coupling: i32,
    updates: u64,
    accepts: u64,
) -> bool {
    match kind {
        Kind::RenewedUpstreamNoReturn => {
            p_fires == [0, 2]
                && candidate_crossings == 2
                && return_crossings == 0
                && resistance == 1
                && coupling == 1
                && updates == 0
                && accepts == 0
        }
        Kind::CompletedDownstreamReturn => {
            p_fires == [0]
                && candidate_crossings == 1
                && return_crossings == 1
                && resistance == 4
                && coupling == 2
                && updates == 1
                && accepts == 1
        }
        Kind::IndependentReturnNoCandidate => {
            p_fires.is_empty()
                && candidate_crossings == 0
                && return_crossings == 1
                && resistance == 1
                && coupling == 1
                && updates == 0
                && accepts == 0
        }
        Kind::LateDownstreamReturn => {
            p_fires == [0]
                && candidate_crossings == 1
                && return_crossings == 1
                && resistance == 0
                && coupling == 1
                && updates == 0
                && accepts == 0
        }
        Kind::SimultaneousUpstreamAndReturn => {
            p_fires == [0, 2]
                && candidate_crossings == 2
                && return_crossings == 1
                && resistance == 4
                && coupling == 2
                && updates == 1
                && accepts == 1
        }
        Kind::UpstreamExecutionOnly => {
            p_fires == [0]
                && candidate_crossings == 1
                && return_crossings == 0
                && resistance == 1
                && coupling == 1
                && updates == 0
                && accepts == 0
        }
    }
}

fn build(namespace: u64, reverse: bool, reflect: bool, late: bool) -> World {
    let mut substrate = PlasticSubstrate::new();
    let side = if reflect { -1 } else { 1 };
    let specs = [
        (10, -60_000, 10, 1),
        (11, -50_000, 11, 1),
        (20, 0, 20, 2),
        (30, side, 30, 1),
        (40, 60_000, 40, 1),
        (50, 70_000, 50, 2),
        (60, -70_000, 60, 1),
    ];
    let order: Vec<usize> = if reverse {
        (0..specs.len()).rev().collect()
    } else {
        (0..specs.len()).collect()
    };
    let mut cells = [None; 7];
    for index in order {
        let (offset, position, region, threshold) = specs[index];
        cells[index] = Some(substrate.add_cell(cell(
            physical(namespace, offset),
            position,
            region,
            threshold,
        )));
    }
    let cells = cells.map(|value| value.expect("cell allocated"));
    let upstream = [cells[0], cells[1]];
    let p = cells[2];
    let x = cells[3];
    let world = cells[4];
    let return_source = cells[5];
    let independent_x = cells[6];
    let arrow_specs = [
        fixed(upstream[0], p, 0, 1),
        fixed(upstream[1], p, 0, 1),
        fixed(p, x, 0, 1),
        fixed(x, return_source, 1, 1),
        fixed(world, return_source, 0, 1),
        return_path(return_source, p, if late { 5 } else { 1 }),
        fixed(independent_x, x, 0, 1),
    ];
    let arrow_order: Vec<usize> = if reverse {
        (0..arrow_specs.len()).rev().collect()
    } else {
        (0..arrow_specs.len()).collect()
    };
    let mut arrows = [None; 7];
    for index in arrow_order {
        arrows[index] = Some(substrate.add_arrow(arrow_specs[index]));
    }
    let arrows = arrows.map(|value| value.expect("arrow allocated"));
    World {
        substrate,
        namespace,
        upstream,
        world,
        independent_x,
        upstream_arrows: [arrows[0], arrows[1]],
        candidate: arrows[2],
        return_arrow: arrows[5],
    }
}

fn namespace(seed: u64, kind: Kind) -> u64 {
    NAMESPACE_BASE + seed * 0x1000_0000 + kind.index() * 0x0100_0000
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 1,
    }
}

fn fixed(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 1,
    }
}

fn return_path(from: CellId, to: CellId, delay: i64) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling: 1,
        resistance: 2,
    }
}

fn pulse(substrate: &mut PlasticSubstrate, target: CellId, tick: i64, impulse: i32, phase: i32) {
    substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: 0xF100_0000 + u64::try_from(phase.max(0)).expect("phase"),
        target,
        impulse,
    });
}

fn physical(namespace: u64, offset: u64) -> u64 {
    namespace + offset
}

fn firing_ticks(trace: &[TraceEntry], namespace: u64, offset: u64) -> Vec<i64> {
    trace
        .iter()
        .filter_map(|entry| {
            (entry.target_physical == physical(namespace, offset) && entry.fired)
                .then_some(entry.tick)
        })
        .collect()
}

fn count_crossings(crossings: &[Crossing], namespace: u64, from: u64, to: u64) -> usize {
    crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == physical(namespace, from)
                && crossing.to_physical == physical(namespace, to)
        })
        .count()
}

fn join_i64(values: &[i64]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_bool(values: &[bool]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn csv(rows: &[Row]) -> String {
    let mut out = String::from("seed,stratum,scenario,namespace,u0_fires,u1_fires,p_fires,x_fires,world_fires,return_source_fires,u0_crossings,u1_crossings,candidate_crossings,effect_crossings,return_crossings,incoming_routes,candidate_resistance,candidate_coupling,candidate_live,candidate_eligible_until,return_updates,qualification_checks,qualification_accepts,path_edges,validity,claim,work,bytes,fingerprint,permanent,quiescent,replay,passed\n");
    for row in rows {
        let value = &row.observation;
        out.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.seed,
            row.stratum,
            row.kind.name(),
            row.namespace,
            join_i64(&value.upstream_fires[0]),
            join_i64(&value.upstream_fires[1]),
            join_i64(&value.p_fires),
            join_i64(&value.x_fires),
            join_i64(&value.world_fires),
            join_i64(&value.return_source_fires),
            value.upstream_crossings[0],
            value.upstream_crossings[1],
            value.candidate_crossings,
            value.effect_crossings,
            value.return_crossings,
            value.incoming_routes,
            value.candidate_resistance,
            value.candidate_coupling,
            value.candidate_live,
            value.candidate_eligible_until.map_or_else(|| "none".to_owned(), |x| x.to_string()),
            value.return_updates,
            value.qualification_checks,
            value.qualification_accepts,
            value.path_edges,
            join_bool(&value.validity),
            value.claim,
            value.work,
            value.bytes,
            value.fingerprint,
            value.permanent,
            value.quiescent,
            row.replay,
            row.passed,
        ));
    }
    out
}

fn report(rows: &[Row]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let clauses = rows
        .iter()
        .map(|row| {
            row.observation
                .validity
                .into_iter()
                .filter(|value| *value)
                .count()
        })
        .sum::<usize>();
    let accepts = rows
        .iter()
        .map(|row| row.observation.qualification_accepts)
        .sum::<u64>();
    let edges = rows
        .iter()
        .map(|row| row.observation.path_edges)
        .sum::<u64>();
    format!(
        "# LR1 three-factor local plasticity Arm A v1\n\nOutcome: **{}**.\n\n- rows: `{passed}/{}`;\n- validity clauses: `{clauses}/192`;\n- qualified accepts: `{accepts}`;\n- route-search edges: `{edges}`;\n- exact replay: `{}`;\n- naturally quiescent: `{}`;\n- authoritative PX0 changed: `false`;\n- new active state or semantic return flag: `false`;\n- Arm B/C or PX3 executed: `false`.\n",
        if passed == rows.len() {
            "LR1-A FUNCTIONAL POSITIVE"
        } else {
            "LR1-A NEGATIVE"
        },
        rows.len(),
        rows.iter().all(|row| row.replay),
        rows.iter().all(|row| row.observation.quiescent),
    )
}

fn sha(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .unwrap_or_else(|error| panic!("sha256sum {path}: {error}"));
    assert!(output.status.success(), "sha256sum failed: {path}");
    String::from_utf8(output.stdout)
        .expect("sha output")
        .split_whitespace()
        .next()
        .expect("sha value")
        .to_owned()
}

fn absent(paths: &[&str]) {
    for path in paths {
        assert!(!Path::new(path).exists(), "artifact already exists: {path}");
    }
}

fn publish(stage: &str, final_path: &str, content: &str) {
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(stage)
        .unwrap_or_else(|error| panic!("create {stage}: {error}"));
    file.write_all(content.as_bytes())
        .unwrap_or_else(|error| panic!("write {stage}: {error}"));
    file.sync_all()
        .unwrap_or_else(|error| panic!("sync {stage}: {error}"));
    rename(stage, final_path).unwrap_or_else(|error| panic!("publish {final_path}: {error}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_matrix_freezes_only_simultaneous_runaway() {
        let mut failures = Vec::new();
        for seed in SEEDS {
            for kind in Kind::ALL {
                let row = replay(seed, kind);
                if row.passed {
                    assert_ne!(kind, Kind::SimultaneousUpstreamAndReturn);
                } else {
                    failures.push((seed, kind, row.observation.quiescent));
                }
            }
        }
        assert_eq!(failures.len(), SEEDS.len());
        assert!(failures.iter().all(|(_, kind, quiescent)| *kind
            == Kind::SimultaneousUpstreamAndReturn
            && !quiescent));
    }

    #[test]
    fn renewed_drive_is_rejected_and_completed_route_is_accepted() {
        let renewed = run(SEEDS[0], Kind::RenewedUpstreamNoReturn);
        let completed = run(SEEDS[0], Kind::CompletedDownstreamReturn);
        assert_eq!(renewed.observation.qualification_accepts, 0);
        assert_eq!(renewed.observation.return_updates, 0);
        assert_eq!(completed.observation.qualification_accepts, 1);
        assert_eq!(completed.observation.return_updates, 1);
    }
}
