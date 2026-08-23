use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const SIDES: usize = 2;
const PRESENTATIONS: usize = 8;
const SPACING: i64 = 14;
const FIRST_TICK: i64 = 2;
const NAMESPACE_BASE: u64 = 0x6_1200_0000;
const SUBSTRATE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PROTOCOL_SHA256: &str = "987fdb349775d8666202f0d9609c655fcfa805010168c14029d3b349812cf8e3";
const V1_NEGATIVE_AUDIT_SHA256: &str =
    "96ff5d804df3c1692d68dd6b5a88dd8ccf529eb477571fc5385b2865bb68c703";
const FROZEN_PARENT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const RESULT_CSV: &str = "results/px6_physical_consequence_credit_probe_v2.csv";
const RESULT_MD: &str = "results/px6_physical_consequence_credit_probe_v2.md";
const STAGING_CSV: &str = "results/.px6_physical_consequence_credit_probe_v2.csv.staging";
const STAGING_MD: &str = "results/.px6_physical_consequence_credit_probe_v2.md.staging";

// This enum belongs to the external measurement harness. No value of this
// type, nor any expectation derived from it, enters PlasticSubstrate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldKind {
    Left,
    Right,
    Both,
    Correlation,
    CrossedReturn,
    NoReturn,
}

impl WorldKind {
    const ALL: [Self; 6] = [
        Self::Left,
        Self::Right,
        Self::Both,
        Self::Correlation,
        Self::CrossedReturn,
        Self::NoReturn,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Both => "both",
            Self::Correlation => "correlation",
            Self::CrossedReturn => "crossed-return",
            Self::NoReturn => "no-return",
        }
    }

    fn participants(self) -> [bool; SIDES] {
        match self {
            Self::Left | Self::CrossedReturn | Self::NoReturn => [true, false],
            Self::Right => [false, true],
            Self::Both => [true, true],
            Self::Correlation => [false, false],
        }
    }

    fn expected_live(self) -> [bool; SIDES] {
        match self {
            Self::Left => [true, false],
            Self::Right => [false, true],
            Self::Both => [true, true],
            Self::Correlation | Self::CrossedReturn | Self::NoReturn => [false, false],
        }
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    entries: [CellId; SIDES],
    training_drivers: [CellId; SIDES],
    correlation_drivers: [CellId; SIDES],
    context: CellId,
    candidates: [ArrowId; SIDES],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Metrics {
    traversals: [usize; SIDES],
    consequence_firings: [usize; SIDES],
    trace_firings: [usize; SIDES],
    return_arrivals: [usize; SIDES],
    outward_crossings: [usize; SIDES],
    resistance_before: [u32; SIDES],
    resistance_after: [u32; SIDES],
    live_after: [bool; SIDES],
    heldout_crossings: [usize; SIDES],
    naturally_quiescent: bool,
    persistent_bytes: usize,
    work: WorkLedger,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    world: WorldKind,
    namespace: u64,
    metrics: Metrics,
    duplicate_exact: bool,
    passed: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let preflight = args == ["--preflight"];
    let probe = args == ["--probe-v2"];
    if !preflight && !probe {
        eprintln!("PX6 development PROBE v2 requires --preflight or --probe-v2");
        std::process::exit(2);
    }
    assert!(source_audit(), "frozen PX2 parent or protocol changed");
    for path in [RESULT_CSV, RESULT_MD, STAGING_CSV, STAGING_MD] {
        assert!(
            !Path::new(path).exists(),
            "write-once artifact exists: {path}"
        );
    }
    if preflight {
        println!("PX6_PHYSICAL_CONSEQUENCE_CREDIT_PROBE_V2_PREFLIGHT_OK");
        return;
    }

    eprintln!("PX6_PHYSICAL_CONSEQUENCE_CREDIT_PROBE_V2_EVIDENCE_SPENT");
    let rows = WorldKind::ALL
        .into_iter()
        .enumerate()
        .map(|(ordinal, kind)| {
            let namespace = NAMESPACE_BASE + ordinal as u64 * 0x0010_0000;
            let metrics = run_world(namespace, kind);
            let duplicate = run_world(namespace, kind);
            let duplicate_exact = metrics == duplicate;
            let passed = evaluate(kind, &metrics, duplicate_exact);
            Row {
                world: kind,
                namespace,
                metrics,
                duplicate_exact,
                passed,
            }
        })
        .collect::<Vec<_>>();

    publish(&csv(&rows), &markdown(&rows));
    if rows.iter().any(|row| !row.passed) {
        std::process::exit(1);
    }
}

fn source_audit() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == SUBSTRATE_SHA256
        && sha256("experiments/px6_physical_consequence_credit_probe_v2_protocol.md")
            == PROTOCOL_SHA256
        && sha256("experiments/px6_physical_consequence_credit_probe_v1_negative_audit.md")
            == V1_NEGATIVE_AUDIT_SHA256
        && git_rev("px2-physical-causal-direction-authoritative^{commit}") == FROZEN_PARENT
        && !cargo_manifest().contains("ds8")
}

fn cargo_manifest() -> String {
    std::fs::read_to_string("crates/px0-physical-correspondence/Cargo.toml")
        .expect("read PX substrate manifest")
}

fn build_world(namespace: u64, kind: WorldKind) -> World {
    let mut substrate = PlasticSubstrate::new();
    let mut entries = [None; SIDES];
    let mut participants = [None; SIDES];
    let mut consequences = [None; SIDES];
    let mut traces = [None; SIDES];
    let mut outside = [None; SIDES];
    let mut training_drivers = [None; SIDES];
    let mut correlation_drivers = [None; SIDES];

    for side in 0..SIDES {
        let base = side as i32 * 40;
        entries[side] = Some(substrate.add_cell(cell(namespace + side as u64, base, 0, 1)));
        participants[side] =
            Some(substrate.add_cell(cell(namespace + 10 + side as u64, base + 8, 0, 2)));
        consequences[side] =
            Some(substrate.add_cell(cell(namespace + 20 + side as u64, base + 16, 0, 2)));
        traces[side] =
            Some(substrate.add_cell(cell(namespace + 30 + side as u64, base + 24, 0, 2)));
        outside[side] =
            Some(substrate.add_cell(cell(namespace + 40 + side as u64, 1_000 + base, 1, 1)));
        training_drivers[side] =
            Some(substrate.add_cell(cell(namespace + 50 + side as u64, 1_100 + base, 0, 1)));
        correlation_drivers[side] =
            Some(substrate.add_cell(cell(namespace + 60 + side as u64, 1_200 + base, 0, 1)));
    }
    let entries = entries.map(Option::unwrap);
    let participants = participants.map(Option::unwrap);
    let consequences = consequences.map(Option::unwrap);
    let traces = traces.map(Option::unwrap);
    let outside = outside.map(Option::unwrap);
    let training_drivers = training_drivers.map(Option::unwrap);
    let correlation_drivers = correlation_drivers.map(Option::unwrap);
    let context = substrate.add_cell(cell(namespace + 70, 1_400, 0, 1));

    for side in 0..SIDES {
        substrate.add_arrow(stable(entries[side], participants[side], 1, 1, 0));
        substrate.add_arrow(stable(training_drivers[side], participants[side], 1, 1, 0));
        substrate.add_arrow(stable(training_drivers[side], consequences[side], 2, 1, 0));
        substrate.add_arrow(stable(
            correlation_drivers[side],
            consequences[side],
            2,
            2,
            0,
        ));
        substrate.add_arrow(stable(consequences[side], traces[side], 1, 1, 0));
        substrate.add_arrow(stable(consequences[side], traces[side], 1, 1, 1));
        substrate.add_arrow(stable(consequences[side], outside[side], 0, 1, 0));
        substrate.add_arrow(stable(context, participants[side], 1, 1, side as i32));
    }

    match kind {
        WorldKind::Left | WorldKind::Right | WorldKind::Both | WorldKind::Correlation => {
            for side in 0..SIDES {
                substrate.add_arrow(stable(traces[side], participants[side], 1, 1, 0));
            }
        }
        WorldKind::CrossedReturn => {
            substrate.add_arrow(stable(traces[0], participants[1], 1, 1, 0));
        }
        WorldKind::NoReturn => {}
    }

    let candidates = std::array::from_fn(|side| {
        substrate.add_arrow(ArrowSpec {
            from: participants[side],
            to: consequences[side],
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 3,
        })
    });

    World {
        substrate,
        namespace,
        entries,
        training_drivers,
        correlation_drivers,
        context,
        candidates,
    }
}

fn run_world(namespace: u64, kind: WorldKind) -> Metrics {
    let mut world = build_world(namespace, kind);
    let participants = kind.participants();
    let resistance_before =
        std::array::from_fn(|side| world.substrate.arrow_resistance(world.candidates[side]));
    let mut metrics = Metrics {
        resistance_before,
        naturally_quiescent: true,
        persistent_bytes: world.substrate.persistent_bytes(),
        ..Metrics::default()
    };

    for presentation in 0..PRESENTATIONS {
        let tick = FIRST_TICK + presentation as i64 * SPACING;
        for side in 0..SIDES {
            if participants[side] {
                enter(
                    &mut world.substrate,
                    world.entries[side],
                    tick,
                    namespace + 0x1_000 + presentation as u64 * 0x100 + side as u64,
                );
                enter(
                    &mut world.substrate,
                    world.training_drivers[side],
                    tick,
                    namespace + 0x2_000 + presentation as u64 * 0x100 + side as u64,
                );
            } else {
                enter(
                    &mut world.substrate,
                    world.correlation_drivers[side],
                    tick,
                    namespace + 0x3_000 + presentation as u64 * 0x100 + side as u64,
                );
            }
        }
        let run = world.substrate.propagate();
        for side in 0..SIDES {
            metrics.traversals[side] += firings(&run, physical(world.namespace, 10, side));
            metrics.consequence_firings[side] += firings(&run, physical(world.namespace, 20, side));
            metrics.trace_firings[side] += firings(&run, physical(world.namespace, 30, side));
            metrics.return_arrivals[side] += arrivals(&run, physical(world.namespace, 10, side))
                .saturating_sub(2 * usize::from(participants[side]));
            metrics.outward_crossings[side] += crossings(&run, physical(world.namespace, 20, side));
        }
        add_work(&mut metrics.work, &run.work);
        metrics.naturally_quiescent &= run.naturally_quiescent;
    }

    metrics.resistance_after = std::array::from_fn(|side| {
        if world.substrate.arrow_is_live(world.candidates[side]) {
            world.substrate.arrow_resistance(world.candidates[side])
        } else {
            0
        }
    });
    metrics.live_after =
        std::array::from_fn(|side| world.substrate.arrow_is_live(world.candidates[side]));
    let heldout_tick = FIRST_TICK + PRESENTATIONS as i64 * SPACING + 4;
    metrics.heldout_crossings = heldout(&world, heldout_tick);
    metrics.fingerprint = world.substrate.complete_fingerprint();
    metrics
}

fn heldout(world: &World, tick: i64) -> [usize; SIDES] {
    let mut clone = world.clone();
    let advance = clone.substrate.advance_time(tick);
    for side in 0..SIDES {
        enter(
            &mut clone.substrate,
            clone.entries[side],
            tick,
            clone.namespace + 0x8_000 + side as u64,
        );
    }
    enter(
        &mut clone.substrate,
        clone.context,
        tick,
        clone.namespace + 0x9_000,
    );
    let run = clone.substrate.propagate();
    assert!(advance.total() > 0);
    assert!(run.naturally_quiescent);
    std::array::from_fn(|side| crossings(&run, physical(clone.namespace, 20, side)))
}

fn evaluate(kind: WorldKind, metrics: &Metrics, duplicate_exact: bool) -> bool {
    let participants = kind.participants();
    let expected_live = kind.expected_live();
    let expected_traversals = participants.map(|value| usize::from(value) * PRESENTATIONS);
    let expected_heldout = expected_live.map(usize::from);
    let expected_returns = match kind {
        WorldKind::Left => [PRESENTATIONS, PRESENTATIONS],
        WorldKind::Right => [PRESENTATIONS, PRESENTATIONS],
        WorldKind::Both => [PRESENTATIONS, PRESENTATIONS],
        WorldKind::Correlation => [PRESENTATIONS, PRESENTATIONS],
        WorldKind::CrossedReturn => [0, metrics.consequence_firings[0]],
        WorldKind::NoReturn => [0, 0],
    };

    let downstream_exact = match kind {
        WorldKind::CrossedReturn | WorldKind::NoReturn => {
            (1..PRESENTATIONS).contains(&metrics.consequence_firings[0])
                && metrics.consequence_firings[1] == PRESENTATIONS
                && metrics.trace_firings == metrics.consequence_firings
                && metrics.outward_crossings == metrics.consequence_firings
        }
        _ => {
            metrics.consequence_firings == [PRESENTATIONS, PRESENTATIONS]
                && metrics.trace_firings == [PRESENTATIONS, PRESENTATIONS]
                && metrics.outward_crossings == [PRESENTATIONS, PRESENTATIONS]
        }
    };
    let global_return_work_exact = match kind {
        WorldKind::Left | WorldKind::Right | WorldKind::Both => {
            metrics.work.local_return_updates > 0
        }
        WorldKind::Correlation | WorldKind::CrossedReturn | WorldKind::NoReturn => {
            metrics.work.local_return_updates == 0
        }
    };

    metrics.traversals == expected_traversals
        && downstream_exact
        && metrics.return_arrivals == expected_returns
        && metrics.resistance_before == [3, 3]
        && metrics.live_after == expected_live
        && (0..SIDES).all(|side| {
            if expected_live[side] {
                metrics.resistance_after[side] > metrics.resistance_before[side]
            } else {
                metrics.resistance_after[side] == 0
            }
        })
        && metrics.heldout_crossings == expected_heldout
        && metrics.naturally_quiescent
        && global_return_work_exact
        && metrics.persistent_bytes > 0
        && metrics.work.total() > 0
        && duplicate_exact
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 10_000,
    }
}

fn stable(from: CellId, to: CellId, delay: i64, coupling: i32, phase: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase,
        coupling,
        resistance: 10_000,
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

fn physical(namespace: u64, offset: u64, side: usize) -> u64 {
    namespace + offset + side as u64
}

fn firings(run: &Execution, target: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == target && entry.fired)
        .count()
}

fn arrivals(run: &Execution, target: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == target)
        .count()
}

fn crossings(run: &Execution, from: u64) -> usize {
    run.crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_region == 1)
        .count()
}

fn add_work(total: &mut WorkLedger, value: &WorkLedger) {
    total.queue_comparisons += value.queue_comparisons;
    total.spikes_delivered += value.spikes_delivered;
    total.generation_checks += value.generation_checks;
    total.state_updates += value.state_updates;
    total.threshold_checks += value.threshold_checks;
    total.firings += value.firings;
    total.arrow_checks += value.arrow_checks;
    total.spikes_emitted += value.spikes_emitted;
    total.local_eligibility_writes += value.local_eligibility_writes;
    total.local_return_updates += value.local_return_updates;
    total.ordinary_pressure_updates += value.ordinary_pressure_updates;
    total.local_structural_proposals += value.local_structural_proposals;
    total.physical_deallocations += value.physical_deallocations;
}

fn csv(rows: &[Row]) -> String {
    let mut out = String::from(
        "world,namespace,traversals,consequence_firings,trace_firings,return_arrivals,outward_crossings,resistance_before,resistance_after,live_after,heldout_crossings,quiescent,local_return_updates,pressure_updates,deallocations,work,persistent_bytes,fingerprint,duplicate_exact,passed\n",
    );
    for row in rows {
        let m = &row.metrics;
        out.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.world.name(),
            row.namespace,
            pair_usize(m.traversals),
            pair_usize(m.consequence_firings),
            pair_usize(m.trace_firings),
            pair_usize(m.return_arrivals),
            pair_usize(m.outward_crossings),
            pair_u32(m.resistance_before),
            pair_u32(m.resistance_after),
            pair_bool(m.live_after),
            pair_usize(m.heldout_crossings),
            m.naturally_quiescent,
            m.work.local_return_updates,
            m.work.ordinary_pressure_updates,
            m.work.physical_deallocations,
            m.work.total(),
            m.persistent_bytes,
            m.fingerprint,
            row.duplicate_exact,
            row.passed,
        ));
    }
    out
}

fn markdown(rows: &[Row]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let work = rows.iter().map(|row| row.metrics.work.total()).sum::<u64>();
    let storage = rows
        .iter()
        .map(|row| row.metrics.persistent_bytes)
        .sum::<usize>();
    let mut out = format!(
        "# PX6 physical consequence-credit no-new-mechanism PROBE v2\n\nOutcome: **{passed}/{} worlds passed**.\n\nFrozen parent: `{FROZEN_PARENT}`. Active substrate SHA-256: `{SUBSTRATE_SHA256}`.\n\nTotal measured work: `{work}`. Aggregate per-world persistent storage: `{storage}` bytes.\n\n| world | traversal | downstream | return arrivals | resistance after | live | held-out | work | replay | pass |\n|---|---:|---:|---:|---:|---:|---:|---:|:---:|:---:|\n",
        rows.len()
    );
    for row in rows {
        let m = &row.metrics;
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.world.name(),
            pair_usize(m.traversals),
            pair_usize(m.consequence_firings),
            pair_usize(m.return_arrivals),
            pair_u32(m.resistance_after),
            pair_bool(m.live_after),
            pair_usize(m.heldout_crossings),
            m.work.total(),
            row.duplicate_exact,
            row.passed,
        ));
    }
    out.push_str(
        "\nThe unchanged PX0--PX2 law alone produced differential persistence. Downstream occurrence and return without local participation did not preserve either weak arrow. No additional organism mechanism executed.\n",
    );
    out
}

fn pair_usize(value: [usize; SIDES]) -> String {
    format!("{}|{}", value[0], value[1])
}

fn pair_u32(value: [u32; SIDES]) -> String {
    format!("{}|{}", value[0], value[1])
}

fn pair_bool(value: [bool; SIDES]) -> String {
    format!("{}|{}", value[0], value[1])
}

fn publish(csv: &str, markdown: &str) {
    write_new(STAGING_CSV, csv);
    write_new(STAGING_MD, markdown);
    rename(STAGING_CSV, RESULT_CSV).expect("publish CSV atomically");
    rename(STAGING_MD, RESULT_MD).expect("publish report atomically");
}

fn write_new(path: &str, body: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create write-once staging artifact");
    file.write_all(body.as_bytes()).expect("write artifact");
    file.sync_all().expect("sync artifact");
}

fn sha256(path: &str) -> String {
    command("shasum", &["-a", "256", path])
        .split_whitespace()
        .next()
        .expect("SHA-256 output")
        .to_string()
}

fn git_rev(spec: &str) -> String {
    command("git", &["rev-parse", spec]).trim().to_string()
}

fn command(program: &str, args: &[&str]) -> String {
    let output = Command::new(program)
        .args(args)
        .output()
        .expect("run audit command");
    assert!(output.status.success(), "audit command failed: {program}");
    String::from_utf8(output.stdout).expect("UTF-8 audit output")
}
