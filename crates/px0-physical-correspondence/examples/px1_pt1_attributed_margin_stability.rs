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
const POSITIVE_PROBE_SHA256: &str =
    "cda4bf6750abb40f7b3798e84c0b6f39527704a02c69b133896ce51c3925420b";
const MICRO_PROTOCOL_SHA256: &str =
    "3c3e0d968e247988cb34f5ecac602c2c1af634758060afdf577b6aad927df829";
const RESULT_CSV: &str = "results/px1_pt1_attributed_margin_stability_micro_v1.csv";
const RESULT_MD: &str = "results/px1_pt1_attributed_margin_stability_micro_v1.md";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    SupportA,
    SupportB,
    NoSupport,
    BlockedReturn,
    ReturnWithoutEffect,
    Joint,
}

impl Scenario {
    const ALL: [Self; 6] = [
        Self::SupportA,
        Self::SupportB,
        Self::NoSupport,
        Self::BlockedReturn,
        Self::ReturnWithoutEffect,
        Self::Joint,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::SupportA => "support-a",
            Self::SupportB => "support-b",
            Self::NoSupport => "no-support",
            Self::BlockedReturn => "blocked-return",
            Self::ReturnWithoutEffect => "return-without-effect",
            Self::Joint => "joint",
        }
    }

    fn supported(self) -> [bool; SIDES] {
        match self {
            Self::SupportA | Self::BlockedReturn => [true, false],
            Self::SupportB => [false, true],
            Self::Joint => [true, true],
            Self::NoSupport | Self::ReturnWithoutEffect => [false, false],
        }
    }

    fn expected_mature(self) -> [bool; SIDES] {
        match self {
            Self::SupportA => [true, false],
            Self::SupportB => [false, true],
            Self::Joint => [true, true],
            Self::NoSupport | Self::BlockedReturn | Self::ReturnWithoutEffect => [false, false],
        }
    }

    fn return_enabled(self) -> bool {
        self != Self::BlockedReturn
    }

    fn external_return(self) -> bool {
        self == Self::ReturnWithoutEffect
    }
}

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
    hub: CellId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExecutionMetrics {
    branch_firings: [usize; SIDES],
    outlet_firings: [usize; SIDES],
    trace_arrivals: [usize; SIDES],
    trace_firings: [usize; SIDES],
    local_returns: [usize; SIDES],
    effects: [usize; SIDES],
    extra_source_firings: usize,
    quiescent: bool,
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
    heldout_branch_firings: [usize; SIDES],
    heldout_outlet_firings: [usize; SIDES],
    heldout_trace_arrivals: [usize; SIDES],
    heldout_trace_firings: [usize; SIDES],
    heldout_local_returns: [usize; SIDES],
    heldout_effects: [usize; SIDES],
    postgap_effects: [usize; SIDES],
    heldout_extra_source_firings: usize,
    postgap_extra_source_firings: usize,
    heldout_quiescent: bool,
    postgap_quiescent: bool,
    correspondence_acquired: bool,
    maturation_exact: bool,
    postgap_exact: bool,
    naturally_quiescent: bool,
    work: WorkLedger,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ResultRow {
    scenario: Scenario,
    transfer: bool,
    metrics: Metrics,
    duplicate_exact: bool,
    passed: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args != ["--micro"] {
        eprintln!("PX1-PT1 requires --micro; GATE/definitive execution is forbidden");
        std::process::exit(2);
    }
    assert!(
        source_audit(),
        "frozen PX0/PT0/PT1 inputs must remain exact"
    );
    assert!(!Path::new(RESULT_CSV).exists(), "PT1 MICRO CSV exists");
    assert!(!Path::new(RESULT_MD).exists(), "PT1 MICRO report exists");
    eprintln!("PX1_PT1_ATTRIBUTED_MARGIN_STABILITY_MICRO_EVIDENCE");

    let mut rows = Vec::new();
    for (ordinal, scenario) in Scenario::ALL.into_iter().enumerate() {
        for transfer in [false, true] {
            let namespace =
                0x8100_0000 + ordinal as u64 * 0x0100_0000 + u64::from(transfer) * 0x0080_0000;
            let first = run_world(namespace, scenario, transfer, transfer);
            let second = run_world(namespace, scenario, transfer, transfer);
            let duplicate_exact = first == second;
            let passed = micro_passed(scenario, &first) && duplicate_exact;
            rows.push(ResultRow {
                scenario,
                transfer,
                metrics: first,
                duplicate_exact,
                passed,
            });
        }
    }
    write_new(RESULT_CSV, &csv(&rows));
    write_new(RESULT_MD, &markdown(&rows));
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
        && sha256("results/px1_pt1_attributed_margin_stability_probe_retry_v1.csv")
            == POSITIVE_PROBE_SHA256
        && sha256("experiments/px1_pt1_attributed_margin_stability_micro_protocol.md")
            == MICRO_PROTOCOL_SHA256
}

fn run_world(namespace: u64, scenario: Scenario, mirror: bool, reverse: bool) -> Metrics {
    let mut world = build_world(namespace, mirror, reverse, scenario.return_enabled());
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
        for (side, supported) in scenario.supported().into_iter().enumerate() {
            if supported {
                enter_many(
                    &mut world.substrate,
                    world.support_drivers[side],
                    tick,
                    1,
                    namespace + 0x4_000 + exposure as u64 * 0x100 + side as u64 * 0x10,
                );
            }
        }
        if scenario.external_return() {
            enter_many(
                &mut world.substrate,
                world.hub,
                tick + 5,
                1,
                namespace + 0x4_800 + exposure as u64 * 0x100,
            );
        }
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
    let expected_mature = scenario.expected_mature();
    let maturation_exact = (0..SIDES).all(|side| {
        if expected_mature[side] {
            continuation_resistance[side] > 3
        } else {
            continuation_resistance[side] == 0
        }
    });

    let heldout = measure_execution(&world, 170);
    let postgap = measure_execution(&world, 210);
    let expected_effects = expected_mature.map(usize::from);
    let postgap_exact = postgap.effects == expected_effects
        && postgap.extra_source_firings == 0
        && postgap.quiescent;
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
        heldout_branch_firings: heldout.branch_firings,
        heldout_outlet_firings: heldout.outlet_firings,
        heldout_trace_arrivals: heldout.trace_arrivals,
        heldout_trace_firings: heldout.trace_firings,
        heldout_local_returns: heldout.local_returns,
        heldout_effects: heldout.effects,
        postgap_effects: postgap.effects,
        heldout_extra_source_firings: heldout.extra_source_firings,
        postgap_extra_source_firings: postgap.extra_source_firings,
        heldout_quiescent: heldout.quiescent,
        postgap_quiescent: postgap.quiescent,
        correspondence_acquired,
        maturation_exact,
        postgap_exact,
        naturally_quiescent,
        work,
        fingerprint,
    }
}

fn build_world(namespace: u64, mirror: bool, reverse: bool, return_enabled: bool) -> World {
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
        if return_enabled {
            substrate.add_arrow(arrow(outlets[side], hub, 1, 1));
        }
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
        hub,
    }
}

fn measure_execution(world: &World, tick: i64) -> ExecutionMetrics {
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
    let branch_firings =
        std::array::from_fn(|side| trace_firings_at(&run, branch_physical(clone.namespace, side)));
    let outlet_firings =
        std::array::from_fn(|side| trace_firings_at(&run, outlet_physical(clone.namespace, side)));
    let trace_arrivals = std::array::from_fn(|side| {
        trace_arrivals_at_tick(&run, trace_physical(clone.namespace, side), tick + 5)
    });
    let trace_firings =
        std::array::from_fn(|side| trace_firings_at(&run, trace_physical(clone.namespace, side)));
    let local_returns = std::array::from_fn(|side| {
        trace_arrivals_at_tick(&run, branch_physical(clone.namespace, side), tick + 6)
    });
    let effects = std::array::from_fn(|side| outward_effects(&run, clone.namespace, side));
    let source_firings = (0..SIDES)
        .map(|side| trace_firings_at(&run, source_physical(clone.namespace, side)))
        .sum::<usize>();
    ExecutionMetrics {
        branch_firings,
        outlet_firings,
        trace_arrivals,
        trace_firings,
        local_returns,
        effects,
        extra_source_firings: source_firings.saturating_sub(SIDES),
        quiescent: run.naturally_quiescent,
    }
}

fn expected_training(scenario: Scenario) -> TrainingExpectation {
    match scenario {
        Scenario::SupportA => TrainingExpectation {
            branches: [EXPOSURES, 0],
            outlets: [EXPOSURES, 0],
            trace_arrivals: [EXPOSURES * 2, EXPOSURES],
            trace_firings: [EXPOSURES, 0],
            local_returns: [EXPOSURES, 0],
        },
        Scenario::SupportB => TrainingExpectation {
            branches: [0, EXPOSURES],
            outlets: [0, EXPOSURES],
            trace_arrivals: [EXPOSURES, EXPOSURES * 2],
            trace_firings: [0, EXPOSURES],
            local_returns: [0, EXPOSURES],
        },
        Scenario::NoSupport => TrainingExpectation::default(),
        Scenario::BlockedReturn => TrainingExpectation {
            branches: [EXPOSURES, 0],
            outlets: [EXPOSURES, 0],
            trace_arrivals: [EXPOSURES, 0],
            trace_firings: [0, 0],
            local_returns: [0, 0],
        },
        Scenario::ReturnWithoutEffect => TrainingExpectation {
            branches: [0, 0],
            outlets: [0, 0],
            trace_arrivals: [EXPOSURES, EXPOSURES],
            trace_firings: [0, 0],
            local_returns: [0, 0],
        },
        Scenario::Joint => TrainingExpectation {
            branches: [EXPOSURES, EXPOSURES],
            outlets: [EXPOSURES, EXPOSURES],
            trace_arrivals: [EXPOSURES * 2, EXPOSURES * 2],
            trace_firings: [EXPOSURES, EXPOSURES],
            local_returns: [EXPOSURES, EXPOSURES],
        },
    }
}

#[derive(Default)]
struct TrainingExpectation {
    branches: [usize; SIDES],
    outlets: [usize; SIDES],
    trace_arrivals: [usize; SIDES],
    trace_firings: [usize; SIDES],
    local_returns: [usize; SIDES],
}

fn expected_heldout_trace_arrivals(expected_mature: [bool; SIDES]) -> [usize; SIDES] {
    let mature_count = expected_mature.into_iter().filter(|value| *value).count();
    expected_mature.map(|mature| mature_count + usize::from(mature))
}

fn micro_passed(scenario: Scenario, metrics: &Metrics) -> bool {
    let training = expected_training(scenario);
    let expected_mature = scenario.expected_mature();
    let expected_effects = expected_mature.map(usize::from);
    let expected_trace_arrivals = expected_heldout_trace_arrivals(expected_mature);
    metrics.correspondence_acquired
        && metrics.maturation_exact
        && metrics.branch_firings == training.branches
        && metrics.outlet_firings == training.outlets
        && metrics.trace_arrivals == training.trace_arrivals
        && metrics.trace_firings == training.trace_firings
        && metrics.local_returns == training.local_returns
        && metrics.extra_source_firings == 0
        && metrics.heldout_branch_firings == [1, 1]
        && metrics.heldout_outlet_firings == expected_effects
        && metrics.heldout_trace_arrivals == expected_trace_arrivals
        && metrics.heldout_trace_firings == expected_effects
        && metrics.heldout_local_returns == expected_effects
        && metrics.heldout_effects == expected_effects
        && metrics.postgap_effects == expected_effects
        && metrics.heldout_extra_source_firings == 0
        && metrics.postgap_extra_source_firings == 0
        && metrics.heldout_quiescent
        && metrics.postgap_quiescent
        && metrics.postgap_exact
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

fn csv(rows: &[ResultRow]) -> String {
    let mut output = String::from(
        "scenario,transfer,correspondence_resistance,continuation_resistance,training_branch_firings,training_outlet_firings,training_trace_arrivals,training_trace_firings,training_local_returns,training_extra_source_firings,heldout_branch_firings,heldout_outlet_firings,heldout_trace_arrivals,heldout_trace_firings,heldout_local_returns,heldout_effects,postgap_effects,heldout_extra_source_firings,postgap_extra_source_firings,heldout_quiescent,postgap_quiescent,correspondence_acquired,maturation_exact,postgap_exact,training_quiescent,duplicate_exact,work,fingerprint,passed\n",
    );
    for row in rows {
        let value = &row.metrics;
        output.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.scenario.name(),
            row.transfer,
            pair_u32(value.correspondence_resistance),
            pair_u32(value.continuation_resistance),
            pair_usize(value.branch_firings),
            pair_usize(value.outlet_firings),
            pair_usize(value.trace_arrivals),
            pair_usize(value.trace_firings),
            pair_usize(value.local_returns),
            value.extra_source_firings,
            pair_usize(value.heldout_branch_firings),
            pair_usize(value.heldout_outlet_firings),
            pair_usize(value.heldout_trace_arrivals),
            pair_usize(value.heldout_trace_firings),
            pair_usize(value.heldout_local_returns),
            pair_usize(value.heldout_effects),
            pair_usize(value.postgap_effects),
            value.heldout_extra_source_firings,
            value.postgap_extra_source_firings,
            value.heldout_quiescent,
            value.postgap_quiescent,
            value.correspondence_acquired,
            value.maturation_exact,
            value.postgap_exact,
            value.naturally_quiescent,
            row.duplicate_exact,
            value.work.total(),
            value.fingerprint,
            row.passed,
        ));
    }
    output
}

fn markdown(rows: &[ResultRow]) -> String {
    let passed = rows.iter().all(|row| row.passed);
    let mut output = format!(
        "# PX1-PT1 attributed-margin stability MICRO v1\n\nOutcome: **{}** (`{}/{}` cells).\n\n| scenario | transfer | train branch | train outlet | train trace arrival/fire | train local return | resistance | held-out branch/outlet | held-out trace arrival/fire | held-out local return/effect | post-gap effect | source refire train/held/post | quiescent train/held/post | replay | pass |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
        if passed { "POSITIVE" } else { "NEGATIVE" },
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
    );
    for row in rows {
        let value = &row.metrics;
        output.push_str(&format!(
            "| {} | {} | `{}` | `{}` | `{}/{}` | `{}` | `{}` | `{}/{}` | `{}/{}` | `{}/{}` | `{}` | `{}/{}/{}` | `{}/{}/{}` | {} | {} |\n",
            row.scenario.name(),
            row.transfer,
            pair_usize(value.branch_firings),
            pair_usize(value.outlet_firings),
            pair_usize(value.trace_arrivals),
            pair_usize(value.trace_firings),
            pair_usize(value.local_returns),
            pair_u32(value.continuation_resistance),
            pair_usize(value.heldout_branch_firings),
            pair_usize(value.heldout_outlet_firings),
            pair_usize(value.heldout_trace_arrivals),
            pair_usize(value.heldout_trace_firings),
            pair_usize(value.heldout_local_returns),
            pair_usize(value.heldout_effects),
            pair_usize(value.postgap_effects),
            value.extra_source_firings,
            value.heldout_extra_source_firings,
            value.postgap_extra_source_firings,
            value.naturally_quiescent,
            value.heldout_quiescent,
            value.postgap_quiescent,
            row.duplicate_exact,
            row.passed,
        ));
    }
    output.push_str(
        "\nEvery physical stage is serialized separately. PX0 changed: `false`. PX1 authoritative: `false`. Definitive evidence executed: `false`.\n",
    );
    output
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
