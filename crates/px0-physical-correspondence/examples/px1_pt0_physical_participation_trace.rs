use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const BRANCHES: usize = 3;
const INITIAL_RESISTANCE: u32 = 2;
const PX0_SOURCE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PARENT_CSV_SHA256: &str = "7ddf75567e4b61fd735a042ddafb949fd85be57021285465a08ca17285c61e80";
const PARENT_AUDIT_SHA256: &str =
    "5935c9aea0ff330aefe28a80903a10d515e440c0fd07fa658b4db68644651a28";
const PROTOCOL_SHA256: &str = "9c0e26e9b81cf41c91a5f6044969569ffe43e110e5db835dafd26d18f0eec17a";
const PROBE_CSV: &str = "results/px1_pt0_physical_participation_trace_probe_v1.csv";
const PROBE_MD: &str = "results/px1_pt0_physical_participation_trace_probe_v1.md";
const MICRO_CSV: &str = "results/px1_pt0_physical_participation_trace_micro_v1.csv";
const MICRO_MD: &str = "results/px1_pt0_physical_participation_trace_micro_v1.md";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    InWindowA,
    LateA,
    NoParticipation,
    SwapB,
    Joint,
    NoReturnA,
    BoundaryInsideA,
    BoundaryOutsideA,
}

impl Scenario {
    const PROBE: [Self; 5] = [
        Self::InWindowA,
        Self::LateA,
        Self::NoParticipation,
        Self::SwapB,
        Self::Joint,
    ];
    const MICRO: [Self; 8] = [
        Self::InWindowA,
        Self::LateA,
        Self::NoParticipation,
        Self::SwapB,
        Self::Joint,
        Self::NoReturnA,
        Self::BoundaryInsideA,
        Self::BoundaryOutsideA,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::InWindowA => "in-window-a",
            Self::LateA => "late-a",
            Self::NoParticipation => "no-participation",
            Self::SwapB => "swap-b",
            Self::Joint => "joint",
            Self::NoReturnA => "no-return-a",
            Self::BoundaryInsideA => "boundary-inside-a",
            Self::BoundaryOutsideA => "boundary-outside-a",
        }
    }

    fn active(self) -> [bool; BRANCHES] {
        match self {
            Self::NoParticipation => [false, false, false],
            Self::SwapB => [false, true, false],
            Self::Joint => [true, true, false],
            _ => [true, false, false],
        }
    }

    fn expected_mature(self) -> [bool; BRANCHES] {
        match self {
            Self::InWindowA | Self::BoundaryInsideA => [true, false, false],
            Self::SwapB => [false, true, false],
            Self::Joint => [true, true, false],
            Self::LateA | Self::NoParticipation | Self::NoReturnA | Self::BoundaryOutsideA => {
                [false, false, false]
            }
        }
    }

    fn return_delay(self) -> Option<i64> {
        match self {
            Self::NoReturnA => None,
            Self::LateA | Self::BoundaryOutsideA => Some(2),
            _ => Some(1),
        }
    }

    fn externally_activate_return_hub(self) -> bool {
        self == Self::NoParticipation
    }

    fn expected_return_tick(self) -> Option<i64> {
        match self {
            Self::NoReturnA => None,
            Self::LateA | Self::BoundaryOutsideA => Some(5),
            _ => Some(4),
        }
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    branches: [CellId; BRANCHES],
    effects: [CellId; BRANCHES],
    hub: CellId,
    candidates: [ArrowId; BRANCHES],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    scenario: &'static str,
    transfer: bool,
    active: [bool; BRANCHES],
    expected_mature: [bool; BRANCHES],
    resistance: [u32; BRANCHES],
    branch_firings: [usize; BRANCHES],
    effect_firings: [usize; BRANCHES],
    return_arrivals: [usize; BRANCHES],
    heldout_effects: [usize; BRANCHES],
    quiescent: bool,
    duplicate_exact: bool,
    work: u64,
    fingerprint: u64,
    passed: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let (stage, scenarios, csv_path, md_path, marker) = match args.as_slice() {
        [value] if value == "--probe" => (
            "PROBE",
            Scenario::PROBE.as_slice(),
            PROBE_CSV,
            PROBE_MD,
            "PX1_PT0_PHYSICAL_PARTICIPATION_TRACE_PROBE_EVIDENCE",
        ),
        [value] if value == "--micro" => (
            "MICRO",
            Scenario::MICRO.as_slice(),
            MICRO_CSV,
            MICRO_MD,
            "PX1_PT0_PHYSICAL_PARTICIPATION_TRACE_MICRO_EVIDENCE",
        ),
        _ => {
            eprintln!("PX1-PT0 requires --probe or --micro; definitive execution is forbidden");
            std::process::exit(2);
        }
    };
    assert!(source_audit(), "frozen PX0/PT0 inputs must remain exact");
    assert!(!Path::new(csv_path).exists(), "stage CSV already exists");
    assert!(!Path::new(md_path).exists(), "stage report already exists");
    eprintln!("{marker}");

    let mut rows = Vec::new();
    for (ordinal, scenario) in scenarios.iter().copied().enumerate() {
        let base = if stage == "PROBE" {
            0x5100_0000
        } else {
            0x6100_0000
        } + ordinal as u64 * 0x0010_0000;
        rows.push(run_duplicate(scenario, base, false, false, false));
        if stage == "MICRO" {
            rows.push(run_duplicate(
                scenario,
                base + 0x0008_0000,
                true,
                true,
                true,
            ));
        }
    }
    write_new(csv_path, &csv(&rows));
    write_new(md_path, &markdown(stage, &rows));
}

fn source_audit() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == PX0_SOURCE_SHA256
        && sha256("results/px1_recurrent_role_stability_diagnostic_v2.csv") == PARENT_CSV_SHA256
        && sha256("experiments/px1_recurrent_role_stability_diagnostic_v2_result_audit.md")
            == PARENT_AUDIT_SHA256
        && sha256("experiments/px1_pt0_physical_participation_trace_protocol.md") == PROTOCOL_SHA256
}

fn run_duplicate(
    scenario: Scenario,
    namespace: u64,
    transfer: bool,
    mirror: bool,
    reverse: bool,
) -> Row {
    let first = run_scenario(scenario, namespace, transfer, mirror, reverse);
    let second = run_scenario(scenario, namespace, transfer, mirror, reverse);
    let duplicate_exact = first == second;
    let mut row = first;
    row.duplicate_exact = duplicate_exact;
    row.passed &= duplicate_exact;
    row
}

fn run_scenario(
    scenario: Scenario,
    namespace: u64,
    transfer: bool,
    mirror: bool,
    reverse: bool,
) -> Row {
    let active = scenario.active();
    let expected_mature = scenario.expected_mature();
    let mut world = build_world(namespace, mirror, reverse, scenario.return_delay());

    for (side, is_active) in active.iter().copied().enumerate() {
        if is_active {
            enter_many(
                &mut world.substrate,
                world.branches[side],
                0,
                2,
                namespace + 0x1_000 + side as u64 * 0x100,
            );
            world.substrate.enter(SpikeInput {
                arrival_tick: 2,
                phase: -10,
                origin_physical: namespace + 0x2_000 + side as u64,
                target: world.effects[side],
                impulse: 1,
            });
        }
    }
    if scenario.externally_activate_return_hub() {
        world.substrate.enter(SpikeInput {
            arrival_tick: 3,
            phase: 0,
            origin_physical: namespace + 0x3_000,
            target: world.hub,
            impulse: 1,
        });
    }
    let learning = world.substrate.propagate();
    let return_tick = scenario.expected_return_tick();
    let resistance = std::array::from_fn(|side| {
        if world.substrate.arrow_is_live(world.candidates[side]) {
            world.substrate.arrow_resistance(world.candidates[side])
        } else {
            0
        }
    });
    let branch_firings =
        std::array::from_fn(|side| trace_firings(&learning, branch_physical(namespace, side)));
    let effect_firings =
        std::array::from_fn(|side| trace_firings(&learning, effect_physical(namespace, side)));
    let return_arrivals = std::array::from_fn(|side| {
        return_tick.map_or(0, |tick| {
            trace_arrivals_at(&learning, branch_physical(namespace, side), tick)
        })
    });
    let heldout_effects = std::array::from_fn(|side| heldout_effect(&world, side));
    let matured = resistance.map(|value| value > INITIAL_RESISTANCE);
    let expected_learning_effects = active.map(usize::from);
    let learning_effects = std::array::from_fn(|side| outward_effects(&learning, namespace, side));
    let expected_returns = usize::from(return_tick.is_some());
    let return_exact = return_arrivals
        .iter()
        .all(|arrivals| *arrivals == expected_returns);
    let passed = matured == expected_mature
        && branch_firings == expected_learning_effects
        && effect_firings == expected_learning_effects
        && learning_effects == expected_learning_effects
        && heldout_effects == expected_mature.map(usize::from)
        && return_exact
        && learning.naturally_quiescent;

    Row {
        scenario: scenario.name(),
        transfer,
        active,
        expected_mature,
        resistance,
        branch_firings,
        effect_firings,
        return_arrivals,
        heldout_effects,
        quiescent: learning.naturally_quiescent,
        duplicate_exact: false,
        work: learning.work.total(),
        fingerprint: world.substrate.complete_fingerprint(),
        passed,
    }
}

fn build_world(namespace: u64, mirror: bool, reverse: bool, return_delay: Option<i64>) -> World {
    let mut substrate = PlasticSubstrate::new();
    let mut branches = [None; BRANCHES];
    let mut effects = [None; BRANCHES];
    let mut outside = [None; BRANCHES];
    let order = if reverse { [2, 1, 0] } else { [0, 1, 2] };
    for side in order {
        let slot = if mirror { BRANCHES - 1 - side } else { side };
        let base = slot as i32 * 20;
        branches[side] =
            Some(substrate.add_cell(cell(branch_physical(namespace, side), base, 0, 2)));
        effects[side] =
            Some(substrate.add_cell(cell(effect_physical(namespace, side), base + 2, 0, 2)));
        outside[side] =
            Some(substrate.add_cell(cell(outside_physical(namespace, side), 1_000 + base, 1, 1)));
    }
    let branches = branches.map(|value| value.expect("branch"));
    let effects = effects.map(|value| value.expect("effect"));
    let outside = outside.map(|value| value.expect("outside"));
    let hub = substrate.add_cell(cell(namespace + 40, 500, 0, 1));
    let candidates = std::array::from_fn(|side| {
        substrate.add_arrow(ArrowSpec {
            from: branches[side],
            to: effects[side],
            delay: 2,
            phase: 0,
            coupling: 1,
            resistance: INITIAL_RESISTANCE,
        })
    });
    for side in order {
        substrate.add_arrow(arrow(effects[side], outside[side], 0, 1));
        if let Some(delay) = return_delay {
            substrate.add_arrow(arrow(effects[side], hub, delay, 1));
        }
        substrate.add_arrow(arrow(hub, branches[side], 1, 1));
    }
    World {
        substrate,
        namespace,
        branches,
        effects,
        hub,
        candidates,
    }
}

fn heldout_effect(world: &World, side: usize) -> usize {
    let mut clone = world.clone();
    clone.substrate.advance_time(20);
    enter_many(
        &mut clone.substrate,
        clone.branches[side],
        20,
        2,
        clone.namespace + 0x4_000 + side as u64 * 0x100,
    );
    let run = clone.substrate.propagate();
    outward_effects(&run, clone.namespace, side)
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 1_000,
    }
}

fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 1_000,
    }
}

fn enter_many(
    substrate: &mut PlasticSubstrate,
    target: CellId,
    tick: i64,
    count: usize,
    origin: u64,
) {
    for ordinal in 0..count {
        substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: ordinal as i32,
            origin_physical: origin + ordinal as u64,
            target,
            impulse: 1,
        });
    }
}

fn branch_physical(namespace: u64, side: usize) -> u64 {
    namespace + 10 + side as u64
}

fn effect_physical(namespace: u64, side: usize) -> u64 {
    namespace + 20 + side as u64
}

fn outside_physical(namespace: u64, side: usize) -> u64 {
    namespace + 30 + side as u64
}

fn trace_firings(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn trace_arrivals_at(run: &Execution, physical: u64, tick: i64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.tick == tick)
        .count()
}

fn outward_effects(run: &Execution, namespace: u64, side: usize) -> usize {
    run.crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == effect_physical(namespace, side)
                && crossing.to_physical == outside_physical(namespace, side)
        })
        .count()
}

fn sha256(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("run sha256sum");
    assert!(output.status.success(), "hash {path}");
    String::from_utf8(output.stdout)
        .expect("utf8 hash")
        .split_whitespace()
        .next()
        .expect("hash")
        .to_string()
}

fn csv(rows: &[Row]) -> String {
    let mut text = String::from(
        "scenario,transfer,active,expected_mature,resistance,branch_firings,effect_firings,return_arrivals,heldout_effects,quiescent,duplicate_exact,work,fingerprint,passed\n",
    );
    for row in rows {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.scenario,
            row.transfer,
            triple_bool(row.active),
            triple_bool(row.expected_mature),
            triple_u32(row.resistance),
            triple_usize(row.branch_firings),
            triple_usize(row.effect_firings),
            triple_usize(row.return_arrivals),
            triple_usize(row.heldout_effects),
            row.quiescent,
            row.duplicate_exact,
            row.work,
            row.fingerprint,
            row.passed,
        ));
    }
    text
}

fn markdown(stage: &str, rows: &[Row]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let all_passed = passed == rows.len();
    let mut text = format!(
        "# PX1-PT0 physical participation trace {stage}\n\nOutcome: **{}**.\n\n- rows: `{}/{}` passed\n- frozen PX0 law changed: `false`\n- PX1 authoritative: `false`\n- definitive evidence executed: `false`\n\n",
        if all_passed { "POSITIVE" } else { "NEGATIVE" },
        passed,
        rows.len(),
    );
    text.push_str("| scenario | transfer | active | expected mature | resistance | held-out effects | return | quiescent | replay | pass |\n");
    text.push_str("|---|:---:|---:|---:|---:|---:|---:|:---:|:---:|:---:|\n");
    for row in rows {
        text.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.scenario,
            row.transfer,
            triple_bool(row.active),
            triple_bool(row.expected_mature),
            triple_u32(row.resistance),
            triple_usize(row.heldout_effects),
            triple_usize(row.return_arrivals),
            row.quiescent,
            row.duplicate_exact,
            row.passed,
        ));
    }
    text.push_str("\nThe return is physically identical at every branch. Only recent branch activity may create an eligibility window; no provenance value enters the substrate.\n");
    text
}

fn triple_bool(values: [bool; BRANCHES]) -> String {
    format!("{}|{}|{}", values[0], values[1], values[2])
}

fn triple_u32(values: [u32; BRANCHES]) -> String {
    format!("{}|{}|{}", values[0], values[1], values[2])
}

fn triple_usize(values: [usize; BRANCHES]) -> String {
    format!("{}|{}|{}", values[0], values[1], values[2])
}

fn write_new(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create PT0 artifact");
    file.write_all(contents.as_bytes())
        .expect("write PT0 artifact");
    file.sync_all().expect("sync PT0 artifact");
}
