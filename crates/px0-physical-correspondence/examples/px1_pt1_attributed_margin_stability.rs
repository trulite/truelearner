use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const SIDES: usize = 2;
const SOURCE_THRESHOLD: usize = 4;
const ACQUISITION: usize = 4;
const EXPOSURES: usize = 8;
const PX0_SOURCE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PARENT_NEGATIVE_SHA256: &str =
    "7ddf75567e4b61fd735a042ddafb949fd85be57021285465a08ca17285c61e80";
const PT0_SOURCE_SHA256: &str = "f0b754ed6f7b0603668319a0735da91b4c168f909d4024fd5ce5e2aea4197410";
const PT0_MICRO_SHA256: &str = "f67185cd9443e5501bc19e1e967a5f5b8c1a403850cb3c2aa206ab94e94f9311";
const PT0_READINESS_SHA256: &str =
    "f184d168331d9a7af413621415c76a78db5ba7f00f297f6cdd690525df5bd2ad";
const PROTOCOL_V2_SHA256: &str = "da8396b1955e393a56be2c770f96b8262bf7fad7e9c71e98a81f5abe9fa38725";
const V1_NEGATIVE_AUDIT_SHA256: &str =
    "53ebee871ff068e222fd5a9049203b59e94a67acf8935c96b5800d9f467d417b";
const RETRY_PROTOCOL_SHA256: &str =
    "e7365d73b086815c6531c1f1d35594a902687000018393e1225d1f1f0d0a364c";
const RESULT_CSV: &str = "results/px1_pt1_attributed_margin_stability_probe_retry_v1.csv";
const RESULT_MD: &str = "results/px1_pt1_attributed_margin_stability_probe_retry_v1.md";

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    sources: [CellId; SIDES],
    endpoints: [CellId; SIDES],
    branches: [CellId; SIDES],
    outlets: [CellId; SIDES],
    acquisition_drivers: [CellId; SIDES],
    support_drivers: [CellId; SIDES],
    context: CellId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Metrics {
    correspondence_resistance: [u32; SIDES],
    continuation_resistance: [u32; SIDES],
    branch_firings: [usize; SIDES],
    outlet_firings: [usize; SIDES],
    trace_arrivals: [usize; SIDES],
    trace_firings: [usize; SIDES],
    local_returns: [usize; SIDES],
    extra_source_firings: usize,
    heldout_effects: [usize; SIDES],
    postgap_effects: [usize; SIDES],
    heldout_extra_source_firings: usize,
    postgap_extra_source_firings: usize,
    heldout_quiescent: bool,
    postgap_quiescent: bool,
    correspondence_acquired: bool,
    role_formed: bool,
    productive_recurrence: bool,
    naturally_quiescent: bool,
    work: WorkLedger,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ResultRow {
    metrics: Metrics,
    duplicate_exact: bool,
    passed: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args != ["--probe"] {
        eprintln!("PX1-PT1 requires --probe; MICRO/GATE/definitive execution is forbidden");
        std::process::exit(2);
    }
    assert!(
        source_audit(),
        "frozen PX0/PT0/PT1 inputs must remain exact"
    );
    assert!(!Path::new(RESULT_CSV).exists(), "PT1 PROBE CSV exists");
    assert!(!Path::new(RESULT_MD).exists(), "PT1 PROBE report exists");
    eprintln!("PX1_PT1_ATTRIBUTED_MARGIN_STABILITY_PROBE_RETRY_EVIDENCE");
    let first = run_world(0x7100_0000, 0, false, false);
    let second = run_world(0x7100_0000, 0, false, false);
    let duplicate_exact = first == second;
    let passed = probe_passed(&first) && duplicate_exact;
    let row = ResultRow {
        metrics: first,
        duplicate_exact,
        passed,
    };
    write_new(RESULT_CSV, &csv(&row));
    write_new(RESULT_MD, &markdown(&row));
}

fn source_audit() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == PX0_SOURCE_SHA256
        && sha256("results/px1_recurrent_role_stability_diagnostic_v2.csv")
            == PARENT_NEGATIVE_SHA256
        && sha256(
            "crates/px0-physical-correspondence/examples/px1_pt0_physical_participation_trace.rs",
        ) == PT0_SOURCE_SHA256
        && sha256("results/px1_pt0_physical_participation_trace_micro_v1.csv") == PT0_MICRO_SHA256
        && sha256("experiments/px1_pt0_physical_participation_trace_development_readiness.md")
            == PT0_READINESS_SHA256
        && sha256("experiments/px1_pt1_attributed_margin_stability_protocol_v2.md")
            == PROTOCOL_V2_SHA256
        && sha256("experiments/px1_pt1_attributed_margin_stability_probe_v1_negative_audit.md")
            == V1_NEGATIVE_AUDIT_SHA256
        && sha256("experiments/px1_pt1_attributed_margin_stability_probe_retry_protocol.md")
            == RETRY_PROTOCOL_SHA256
}

fn run_world(namespace: u64, supported: usize, mirror: bool, reverse: bool) -> Metrics {
    let mut world = build_world(namespace, mirror, reverse);
    let mut work = WorkLedger::default();
    let mut naturally_quiescent = true;

    for exposure in 0..ACQUISITION {
        let tick = exposure as i64 * 16;
        for side in 0..SIDES {
            enter_many(
                &mut world.substrate,
                world.sources[side],
                tick,
                SOURCE_THRESHOLD,
                namespace + 0x1_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
            );
            enter_many(
                &mut world.substrate,
                world.acquisition_drivers[side],
                tick,
                1,
                namespace + 0x2_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
            );
        }
        let run = world.substrate.propagate();
        add_work(&mut work, &run.work);
        naturally_quiescent &= run.naturally_quiescent;
    }

    let correspondence: [Vec<ArrowId>; SIDES] = std::array::from_fn(|side| {
        world
            .substrate
            .arrows_between(world.sources[side], world.endpoints[side])
    });
    assert!(correspondence.iter().all(|arrows| arrows.len() == 1));
    let continuations: [ArrowId; SIDES] = std::array::from_fn(|side| {
        world.substrate.add_arrow(ArrowSpec {
            from: world.branches[side],
            to: world.outlets[side],
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 3,
        })
    });

    let mut branch_firings = [0usize; SIDES];
    let mut outlet_firings = [0usize; SIDES];
    let mut trace_arrivals = [0usize; SIDES];
    let mut trace_firings = [0usize; SIDES];
    let mut local_returns = [0usize; SIDES];
    let mut source_firings = 0usize;
    for exposure in 0..EXPOSURES {
        let tick = 66 + exposure as i64 * 12;
        for side in 0..SIDES {
            enter_many(
                &mut world.substrate,
                world.sources[side],
                tick,
                SOURCE_THRESHOLD,
                namespace + 0x3_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
            );
        }
        enter_many(
            &mut world.substrate,
            world.support_drivers[supported],
            tick,
            1,
            namespace + 0x4_000 + exposure as u64 * 0x100,
        );
        let run = world.substrate.propagate();
        for side in 0..SIDES {
            branch_firings[side] += trace_firings_at(&run, branch_physical(namespace, side));
            outlet_firings[side] += trace_firings_at(&run, outlet_physical(namespace, side));
            trace_arrivals[side] +=
                trace_arrivals_at_tick(&run, trace_physical(namespace, side), tick + 5);
            trace_firings[side] += trace_firings_at(&run, trace_physical(namespace, side));
            local_returns[side] +=
                trace_arrivals_at_tick(&run, branch_physical(namespace, side), tick + 6);
            source_firings += trace_firings_at(&run, source_physical(namespace, side));
        }
        add_work(&mut work, &run.work);
        naturally_quiescent &= run.naturally_quiescent;
    }

    let extra_source_firings = source_firings.saturating_sub(EXPOSURES * SIDES);
    let correspondence_resistance =
        std::array::from_fn(|side| max_resistance(&world.substrate, &correspondence[side]));
    let continuation_resistance = std::array::from_fn(|side| {
        if world.substrate.arrow_is_live(continuations[side]) {
            world.substrate.arrow_resistance(continuations[side])
        } else {
            0
        }
    });
    let correspondence_acquired = correspondence_resistance.iter().all(|value| *value > 1);
    let role_formed =
        continuation_resistance[supported] > 2 && continuation_resistance[1 - supported] == 0;

    let (heldout_effects, heldout_extra_source_firings, heldout_quiescent) =
        measure_execution(&world, 170);
    let (postgap_effects, postgap_extra_source_firings, postgap_quiescent) =
        measure_execution(&world, 210);
    let productive_recurrence = postgap_effects[supported] == 1
        && postgap_effects[1 - supported] == 0
        && postgap_extra_source_firings == 0
        && postgap_quiescent;
    let fingerprint = world.substrate.complete_fingerprint();

    Metrics {
        correspondence_resistance,
        continuation_resistance,
        branch_firings,
        outlet_firings,
        trace_arrivals,
        trace_firings,
        local_returns,
        extra_source_firings,
        heldout_effects,
        postgap_effects,
        heldout_extra_source_firings,
        postgap_extra_source_firings,
        heldout_quiescent,
        postgap_quiescent,
        correspondence_acquired,
        role_formed,
        productive_recurrence,
        naturally_quiescent,
        work,
        fingerprint,
    }
}

fn build_world(namespace: u64, mirror: bool, reverse: bool) -> World {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; SIDES];
    let mut endpoints = [None; SIDES];
    let mut branches = [None; SIDES];
    let mut outlets = [None; SIDES];
    let mut traces = [None; SIDES];
    let mut outside = [None; SIDES];
    let mut acquisition_drivers = [None; SIDES];
    let mut support_drivers = [None; SIDES];
    let mut correspondence_gates = [None; SIDES];
    let order = if reverse { [1, 0] } else { [0, 1] };
    for side in order {
        let slot = if mirror { SIDES - 1 - side } else { side };
        let base = slot as i32 * 40;
        sources[side] = Some(substrate.add_cell(cell(
            source_physical(namespace, side),
            base,
            0,
            SOURCE_THRESHOLD as i32,
        )));
        endpoints[side] =
            Some(substrate.add_cell(cell(endpoint_physical(namespace, side), base + 2, 0, 2)));
        branches[side] =
            Some(substrate.add_cell(cell(branch_physical(namespace, side), base + 8, 0, 2)));
        outlets[side] =
            Some(substrate.add_cell(cell(outlet_physical(namespace, side), base + 10, 0, 2)));
        traces[side] =
            Some(substrate.add_cell(cell(trace_physical(namespace, side), base + 16, 0, 2)));
        outside[side] =
            Some(substrate.add_cell(cell(namespace + 60 + side as u64, 1_000 + base, 1, 1)));
        acquisition_drivers[side] =
            Some(substrate.add_cell(cell(namespace + 70 + side as u64, 1_100 + base, 0, 1)));
        support_drivers[side] =
            Some(substrate.add_cell(cell(namespace + 80 + side as u64, 1_200 + base, 0, 1)));
        correspondence_gates[side] =
            Some(substrate.add_cell(cell(namespace + 90 + side as u64, 1_300 + base, 0, 1)));
    }
    let sources = sources.map(|value| value.expect("source"));
    let endpoints = endpoints.map(|value| value.expect("endpoint"));
    let branches = branches.map(|value| value.expect("branch"));
    let outlets = outlets.map(|value| value.expect("outlet"));
    let traces = traces.map(|value| value.expect("trace"));
    let outside = outside.map(|value| value.expect("outside"));
    let acquisition_drivers = acquisition_drivers.map(|value| value.expect("acquisition driver"));
    let support_drivers = support_drivers.map(|value| value.expect("support driver"));
    let correspondence_gates =
        correspondence_gates.map(|value| value.expect("correspondence gate"));
    let context = substrate.add_cell(cell(namespace + 100, 1_500, 0, 1));
    let hub = substrate.add_cell(cell(namespace + 101, 1_600, 0, 1));

    for side in order {
        substrate.add_arrow(arrow(acquisition_drivers[side], endpoints[side], 2, 1));
        substrate.add_arrow(arrow(endpoints[side], correspondence_gates[side], 1, 1));
        substrate.add_arrow(arrow(correspondence_gates[side], sources[side], 1, 1));
        substrate.add_arrow(arrow(endpoints[side], branches[side], 1, 1));
        substrate.add_arrow(arrow(support_drivers[side], branches[side], 3, 1));
        substrate.add_arrow(arrow(support_drivers[side], outlets[side], 4, 1));
        substrate.add_arrow(arrow(context, branches[side], 3, 1));
        substrate.add_arrow(arrow(outlets[side], traces[side], 1, 1));
        substrate.add_arrow(arrow(outlets[side], hub, 1, 1));
        substrate.add_arrow(arrow(outlets[side], outside[side], 0, 1));
        substrate.add_arrow(arrow(traces[side], branches[side], 1, 1));
        substrate.add_arrow(arrow(hub, traces[side], 0, 1));
    }

    World {
        substrate,
        namespace,
        sources,
        endpoints,
        branches,
        outlets,
        acquisition_drivers,
        support_drivers,
        context,
    }
}

fn measure_execution(world: &World, tick: i64) -> ([usize; SIDES], usize, bool) {
    let mut clone = world.clone();
    clone.substrate.advance_time(tick);
    for side in 0..SIDES {
        enter_many(
            &mut clone.substrate,
            clone.sources[side],
            tick,
            SOURCE_THRESHOLD,
            clone.namespace + 0x5_000 + side as u64 * 0x10,
        );
    }
    enter_many(
        &mut clone.substrate,
        clone.context,
        tick,
        1,
        clone.namespace + 0x6_000,
    );
    let run = clone.substrate.propagate();
    let effects = std::array::from_fn(|side| outward_effects(&run, clone.namespace, side));
    let source_firings = (0..SIDES)
        .map(|side| trace_firings_at(&run, source_physical(clone.namespace, side)))
        .sum::<usize>();
    (
        effects,
        source_firings.saturating_sub(SIDES),
        run.naturally_quiescent,
    )
}

fn probe_passed(metrics: &Metrics) -> bool {
    metrics.correspondence_acquired
        && metrics.role_formed
        && metrics.branch_firings == [EXPOSURES, 0]
        && metrics.outlet_firings == [EXPOSURES, 0]
        && metrics.trace_arrivals == [EXPOSURES * 2, EXPOSURES]
        && metrics.trace_firings == [EXPOSURES, 0]
        && metrics.local_returns == [EXPOSURES, 0]
        && metrics.extra_source_firings == 0
        && metrics.heldout_effects == [1, 0]
        && metrics.postgap_effects == [1, 0]
        && metrics.heldout_extra_source_firings == 0
        && metrics.postgap_extra_source_firings == 0
        && metrics.heldout_quiescent
        && metrics.postgap_quiescent
        && metrics.productive_recurrence
        && metrics.naturally_quiescent
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

fn source_physical(namespace: u64, side: usize) -> u64 {
    namespace + 10 + side as u64
}

fn endpoint_physical(namespace: u64, side: usize) -> u64 {
    namespace + 20 + side as u64
}

fn branch_physical(namespace: u64, side: usize) -> u64 {
    namespace + 30 + side as u64
}

fn outlet_physical(namespace: u64, side: usize) -> u64 {
    namespace + 40 + side as u64
}

fn trace_physical(namespace: u64, side: usize) -> u64 {
    namespace + 50 + side as u64
}

fn trace_firings_at(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn trace_arrivals_at_tick(run: &Execution, physical: u64, tick: i64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.tick == tick)
        .count()
}

fn outward_effects(run: &Execution, namespace: u64, side: usize) -> usize {
    run.crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == outlet_physical(namespace, side)
                && crossing.to_physical == namespace + 60 + side as u64
        })
        .count()
}

fn max_resistance(substrate: &PlasticSubstrate, arrows: &[ArrowId]) -> u32 {
    arrows
        .iter()
        .filter(|arrow| substrate.arrow_is_live(**arrow))
        .map(|arrow| substrate.arrow_resistance(*arrow))
        .max()
        .unwrap_or(0)
}

fn add_work(total: &mut WorkLedger, next: &WorkLedger) {
    total.queue_comparisons += next.queue_comparisons;
    total.spikes_delivered += next.spikes_delivered;
    total.generation_checks += next.generation_checks;
    total.state_updates += next.state_updates;
    total.threshold_checks += next.threshold_checks;
    total.firings += next.firings;
    total.arrow_checks += next.arrow_checks;
    total.spikes_emitted += next.spikes_emitted;
    total.local_eligibility_writes += next.local_eligibility_writes;
    total.local_return_updates += next.local_return_updates;
    total.ordinary_pressure_updates += next.ordinary_pressure_updates;
    total.local_structural_proposals += next.local_structural_proposals;
    total.physical_deallocations += next.physical_deallocations;
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

fn pair_u32(values: [u32; SIDES]) -> String {
    format!("{}|{}", values[0], values[1])
}

fn pair_usize(values: [usize; SIDES]) -> String {
    format!("{}|{}", values[0], values[1])
}

fn csv(row: &ResultRow) -> String {
    let value = &row.metrics;
    format!(
        "correspondence_resistance,continuation_resistance,branch_firings,outlet_firings,trace_arrivals,trace_firings,local_returns,extra_source_firings,heldout_effects,postgap_effects,heldout_extra_source_firings,postgap_extra_source_firings,heldout_quiescent,postgap_quiescent,correspondence_acquired,role_formed,productive_recurrence,training_quiescent,duplicate_exact,work,fingerprint,passed\n{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
        pair_u32(value.correspondence_resistance),
        pair_u32(value.continuation_resistance),
        pair_usize(value.branch_firings),
        pair_usize(value.outlet_firings),
        pair_usize(value.trace_arrivals),
        pair_usize(value.trace_firings),
        pair_usize(value.local_returns),
        value.extra_source_firings,
        pair_usize(value.heldout_effects),
        pair_usize(value.postgap_effects),
        value.heldout_extra_source_firings,
        value.postgap_extra_source_firings,
        value.heldout_quiescent,
        value.postgap_quiescent,
        value.correspondence_acquired,
        value.role_formed,
        value.productive_recurrence,
        value.naturally_quiescent,
        row.duplicate_exact,
        value.work.total(),
        value.fingerprint,
        row.passed,
    )
}

fn markdown(row: &ResultRow) -> String {
    let value = &row.metrics;
    format!(
        "# PX1-PT1 attributed-margin stability PROBE retry v1\n\nOutcome: **{}**.\n\n| clause | value |\n|---|---:|\n| PX0 correspondence resistance | `{}` |\n| continuation resistance | `{}` |\n| branch firings | `{}` |\n| outlet firings | `{}` |\n| trace arrivals | `{}` |\n| trace firings | `{}` |\n| local branch returns | `{}` |\n| training extra source firings | `{}` |\n| held-out effects | `{}` |\n| post-gap effects | `{}` |\n| held-out extra source firings | `{}` |\n| post-gap extra source firings | `{}` |\n| training/held-out/post-gap quiescence | `{}/{}/{}` |\n| productive recurrence | `{}` |\n| duplicate exact | `{}` |\n\nPX0 changed: `false`. PX1 authoritative: `false`. Definitive evidence executed: `false`.\n",
        if row.passed { "POSITIVE" } else { "NEGATIVE" },
        pair_u32(value.correspondence_resistance),
        pair_u32(value.continuation_resistance),
        pair_usize(value.branch_firings),
        pair_usize(value.outlet_firings),
        pair_usize(value.trace_arrivals),
        pair_usize(value.trace_firings),
        pair_usize(value.local_returns),
        value.extra_source_firings,
        pair_usize(value.heldout_effects),
        pair_usize(value.postgap_effects),
        value.heldout_extra_source_firings,
        value.postgap_extra_source_firings,
        value.naturally_quiescent,
        value.heldout_quiescent,
        value.postgap_quiescent,
        value.productive_recurrence,
        row.duplicate_exact,
    )
}

fn write_new(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create PT1 artifact");
    file.write_all(contents.as_bytes())
        .expect("write PT1 artifact");
    file.sync_all().expect("sync PT1 artifact");
}
