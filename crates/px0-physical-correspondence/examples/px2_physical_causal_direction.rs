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
const EXPERIENCES: usize = 8;
const PX0_SOURCE_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX1_CSV_SHA256: &str = "6613ff0a96bb3a60fbe7afeb92cd64edced3c6df5dcc04fe47518db158dd88f6";
const PX1_AUDIT_SHA256: &str = "fa4a516fcb6977a45e547ca1bb3b7db3b427c05b381fb60d2700e92fa2ae7c70";
const PX1_HANDOFF_SHA256: &str = "ab4142a24f6ca1095c1c1364f391253752808382ac6ee70ef9d49eac722df28c";
const PROTOCOL_SHA256: &str = "5c43ffda226a125bbbaf7f24dbb1ec8e70861b78a2d730c630535254def85c23";
const PROBE_V1_SHA256: &str = "bfc2963d2afcda7de28fc6c7e2636f34800a2b7e3fa51056319bc71fe8bb0de9";
const PROBE_V1_AUDIT_SHA256: &str =
    "45c6f331fe7d29403314495f9db532ef85ec03b4d6f32469d3d8f3d995982cb8";
const PROBE_V2_PROTOCOL_SHA256: &str =
    "4a8278c6f5cc42f996dfa662176d3c56204b822ae83f8608e84b0186a9ecac0f";
const RESULT_CSV: &str = "results/px2_physical_causal_direction_trace_sufficiency_probe_v2.csv";
const RESULT_MD: &str = "results/px2_physical_causal_direction_trace_sufficiency_probe_v2.md";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    Forward,
    Reverse,
    CorrelationOnly,
    Joint,
    BlockedReturn,
}

impl Scenario {
    const ALL: [Self; 5] = [
        Self::Forward,
        Self::Reverse,
        Self::CorrelationOnly,
        Self::Joint,
        Self::BlockedReturn,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Forward => "forward-participation",
            Self::Reverse => "reverse-participation",
            Self::CorrelationOnly => "correlation-only",
            Self::Joint => "joint-participation",
            Self::BlockedReturn => "participation-without-return",
        }
    }

    fn participants(self) -> [bool; SIDES] {
        match self {
            Self::Forward | Self::BlockedReturn => [true, false],
            Self::Reverse => [false, true],
            Self::Joint => [true, true],
            Self::CorrelationOnly => [false, false],
        }
    }

    fn independent_changes(self) -> [bool; SIDES] {
        match self {
            Self::Forward | Self::BlockedReturn => [false, true],
            Self::Reverse => [true, false],
            Self::CorrelationOnly => [true, true],
            Self::Joint => [false, false],
        }
    }

    fn expected_mature(self) -> [bool; SIDES] {
        match self {
            Self::Forward => [true, false],
            Self::Reverse => [false, true],
            Self::Joint => [true, true],
            Self::CorrelationOnly | Self::BlockedReturn => [false, false],
        }
    }

    fn return_enabled(self) -> bool {
        self != Self::BlockedReturn
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    arrivals: [CellId; SIDES],
    correspondence_ends: [CellId; SIDES],
    continuations: [CellId; SIDES],
    consequences: [CellId; SIDES],
    acquisition_drivers: [CellId; SIDES],
    participation_drivers: [CellId; SIDES],
    independent_drivers: [CellId; SIDES],
    context: CellId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExecutionMetrics {
    continuation_firings: [usize; SIDES],
    consequence_firings: [usize; SIDES],
    trace_arrivals: [usize; SIDES],
    trace_firings: [usize; SIDES],
    local_returns: [usize; SIDES],
    effects: [usize; SIDES],
    extra_arrival_firings: usize,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Metrics {
    correspondence_resistance: [u32; SIDES],
    directional_resistance: [u32; SIDES],
    training_continuation_firings: [usize; SIDES],
    training_consequence_firings: [usize; SIDES],
    training_trace_arrivals: [usize; SIDES],
    training_trace_firings: [usize; SIDES],
    training_local_returns: [usize; SIDES],
    training_effects: [usize; SIDES],
    training_extra_arrival_firings: usize,
    heldout: ExecutionMetrics,
    postgap: ExecutionMetrics,
    training_quiescent: bool,
    work: WorkLedger,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ResultRow {
    scenario: Scenario,
    metrics: Metrics,
    duplicate_exact: bool,
    passed: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args != ["--probe"] {
        eprintln!("PX2 trace-sufficiency development requires --probe");
        std::process::exit(2);
    }
    assert!(
        source_audit(),
        "authoritative PX0/PX1 inputs must remain exact"
    );
    assert!(!Path::new(RESULT_CSV).exists(), "PX2 PROBE CSV exists");
    assert!(!Path::new(RESULT_MD).exists(), "PX2 PROBE report exists");
    eprintln!("PX2_PHYSICAL_CAUSAL_DIRECTION_TRACE_SUFFICIENCY_PROBE_V2_EVIDENCE");

    let mut rows = Vec::new();
    for (ordinal, scenario) in Scenario::ALL.into_iter().enumerate() {
        let namespace = 0xf200_0000 + ordinal as u64 * 0x0100_0000;
        let first = run_world(namespace, scenario);
        let second = run_world(namespace, scenario);
        let duplicate_exact = first == second;
        let passed = probe_passed(scenario, &first) && duplicate_exact;
        rows.push(ResultRow {
            scenario,
            metrics: first,
            duplicate_exact,
            passed,
        });
    }

    write_new(RESULT_CSV, &csv(&rows));
    write_new(RESULT_MD, &markdown(&rows));
}

fn source_audit() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == PX0_SOURCE_SHA256
        && sha256("results/px1_physical_boundary_roles_definitive.csv") == PX1_CSV_SHA256
        && sha256("experiments/px1_physical_boundary_roles_definitive_result_audit.md")
            == PX1_AUDIT_SHA256
        && sha256("experiments/px1_physical_boundary_roles_authority_handoff.md")
            == PX1_HANDOFF_SHA256
        && sha256("experiments/px2_physical_causal_direction_trace_sufficiency_protocol.md")
            == PROTOCOL_SHA256
        && sha256("results/px2_physical_causal_direction_trace_sufficiency_probe_v1.csv")
            == PROBE_V1_SHA256
        && sha256(
            "experiments/px2_physical_causal_direction_trace_sufficiency_probe_v1_negative_audit.md",
        ) == PROBE_V1_AUDIT_SHA256
        && sha256(
            "experiments/px2_physical_causal_direction_trace_sufficiency_probe_v2_protocol.md",
        ) == PROBE_V2_PROTOCOL_SHA256
}

fn run_world(namespace: u64, scenario: Scenario) -> Metrics {
    let mut world = build_world(namespace, scenario.return_enabled());
    let mut work = WorkLedger::default();
    let mut training_quiescent = true;

    for experience in 0..ACQUISITION {
        let tick = experience as i64 * 16;
        for side in 0..SIDES {
            enter_many(
                &mut world.substrate,
                world.arrivals[side],
                tick,
                SOURCE_THRESHOLD,
                namespace + 0x1_000 + experience as u64 * 0x100 + side as u64 * 0x10,
            );
            enter_many(
                &mut world.substrate,
                world.acquisition_drivers[side],
                tick,
                1,
                namespace + 0x2_000 + experience as u64 * 0x100 + side as u64 * 0x10,
            );
        }
        let run = world.substrate.propagate();
        add_work(&mut work, &run.work);
        training_quiescent &= run.naturally_quiescent;
    }

    let correspondence: [Vec<ArrowId>; SIDES] = std::array::from_fn(|side| {
        world
            .substrate
            .arrows_between(world.arrivals[side], world.correspondence_ends[side])
    });
    assert!(correspondence.iter().all(|arrows| arrows.len() == 1));
    let directional: [ArrowId; SIDES] = std::array::from_fn(|side| {
        world.substrate.add_arrow(ArrowSpec {
            from: world.continuations[side],
            to: world.consequences[side],
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 3,
        })
    });

    let mut training_continuation_firings = [0usize; SIDES];
    let mut training_consequence_firings = [0usize; SIDES];
    let mut training_trace_arrivals = [0usize; SIDES];
    let mut training_trace_firings = [0usize; SIDES];
    let mut training_local_returns = [0usize; SIDES];
    let mut training_effects = [0usize; SIDES];
    let mut arrival_firings = 0usize;
    for experience in 0..EXPERIENCES {
        let tick = 66 + experience as i64 * 12;
        for side in 0..SIDES {
            enter_many(
                &mut world.substrate,
                world.arrivals[side],
                tick,
                SOURCE_THRESHOLD,
                namespace + 0x3_000 + experience as u64 * 0x100 + side as u64 * 0x10,
            );
            if scenario.participants()[side] {
                enter_many(
                    &mut world.substrate,
                    world.participation_drivers[side],
                    tick,
                    1,
                    namespace + 0x4_000 + experience as u64 * 0x100 + side as u64 * 0x10,
                );
            }
            if scenario.independent_changes()[side] {
                enter_many(
                    &mut world.substrate,
                    world.independent_drivers[side],
                    tick,
                    1,
                    namespace + 0x5_000 + experience as u64 * 0x100 + side as u64 * 0x10,
                );
            }
        }
        let run = world.substrate.propagate();
        for side in 0..SIDES {
            training_continuation_firings[side] +=
                firings_at(&run, continuation_physical(namespace, side));
            training_consequence_firings[side] +=
                firings_at(&run, consequence_physical(namespace, side));
            training_trace_arrivals[side] +=
                arrivals_at_tick(&run, trace_physical(namespace, side), tick + 5);
            training_trace_firings[side] += firings_at(&run, trace_physical(namespace, side));
            training_local_returns[side] +=
                arrivals_at_tick(&run, continuation_physical(namespace, side), tick + 6);
            training_effects[side] += outward_effects(&run, namespace, side);
            arrival_firings += firings_at(&run, arrival_physical(namespace, side));
        }
        add_work(&mut work, &run.work);
        training_quiescent &= run.naturally_quiescent;
    }

    let correspondence_resistance =
        std::array::from_fn(|side| max_resistance(&world.substrate, &correspondence[side]));
    let directional_resistance = std::array::from_fn(|side| {
        if world.substrate.arrow_is_live(directional[side]) {
            world.substrate.arrow_resistance(directional[side])
        } else {
            0
        }
    });
    let training_extra_arrival_firings = arrival_firings.saturating_sub(EXPERIENCES * SIDES);
    let heldout = measure_execution(&world, 170);
    let postgap = measure_execution(&world, 210);
    let fingerprint = world.substrate.complete_fingerprint();

    Metrics {
        correspondence_resistance,
        directional_resistance,
        training_continuation_firings,
        training_consequence_firings,
        training_trace_arrivals,
        training_trace_firings,
        training_local_returns,
        training_effects,
        training_extra_arrival_firings,
        heldout,
        postgap,
        training_quiescent,
        work,
        fingerprint,
    }
}

fn build_world(namespace: u64, return_enabled: bool) -> World {
    let mut substrate = PlasticSubstrate::new();
    let arrivals = std::array::from_fn(|side| {
        substrate.add_cell(cell(
            arrival_physical(namespace, side),
            side as i32 * 40,
            0,
            4,
        ))
    });
    let correspondence_ends = std::array::from_fn(|side| {
        substrate.add_cell(cell(
            correspondence_physical(namespace, side),
            side as i32 * 40 + 2,
            0,
            2,
        ))
    });
    let continuations = std::array::from_fn(|side| {
        substrate.add_cell(cell(
            continuation_physical(namespace, side),
            side as i32 * 40 + 8,
            0,
            2,
        ))
    });
    let consequences = std::array::from_fn(|side| {
        substrate.add_cell(cell(
            consequence_physical(namespace, side),
            side as i32 * 40 + 10,
            0,
            2,
        ))
    });
    let traces: [CellId; SIDES] = std::array::from_fn(|side| {
        substrate.add_cell(cell(
            trace_physical(namespace, side),
            side as i32 * 40 + 16,
            0,
            2,
        ))
    });
    let outside: [CellId; SIDES] = std::array::from_fn(|side| {
        substrate.add_cell(cell(
            namespace + 60 + side as u64,
            1_000 + side as i32 * 40,
            1,
            1,
        ))
    });
    let acquisition_drivers = std::array::from_fn(|side| {
        substrate.add_cell(cell(
            namespace + 70 + side as u64,
            1_100 + side as i32 * 40,
            0,
            1,
        ))
    });
    let participation_drivers = std::array::from_fn(|side| {
        substrate.add_cell(cell(
            namespace + 80 + side as u64,
            1_200 + side as i32 * 40,
            0,
            1,
        ))
    });
    let independent_drivers = std::array::from_fn(|side| {
        substrate.add_cell(cell(
            namespace + 90 + side as u64,
            1_300 + side as i32 * 40,
            0,
            1,
        ))
    });
    let gates: [CellId; SIDES] = std::array::from_fn(|side| {
        substrate.add_cell(cell(
            namespace + 100 + side as u64,
            1_400 + side as i32 * 40,
            0,
            1,
        ))
    });
    let context = substrate.add_cell(cell(namespace + 110, 1_500, 0, 1));
    let hub = substrate.add_cell(cell(namespace + 111, 1_600, 0, 1));

    for side in 0..SIDES {
        substrate.add_arrow(arrow(
            acquisition_drivers[side],
            correspondence_ends[side],
            2,
            1,
        ));
        substrate.add_arrow(arrow(correspondence_ends[side], gates[side], 1, 1));
        substrate.add_arrow(arrow(gates[side], arrivals[side], 1, 1));
        substrate.add_arrow(arrow(correspondence_ends[side], continuations[side], 1, 1));
        substrate.add_arrow(arrow(
            participation_drivers[side],
            continuations[side],
            3,
            1,
        ));
        substrate.add_arrow(arrow(participation_drivers[side], consequences[side], 4, 1));
        substrate.add_arrow(arrow(independent_drivers[side], consequences[side], 4, 2));
        substrate.add_arrow(arrow(context, continuations[side], 3, 1));
        substrate.add_arrow(arrow(consequences[side], traces[side], 1, 1));
        if return_enabled {
            substrate.add_arrow(arrow(consequences[side], hub, 1, 1));
        }
        substrate.add_arrow(arrow(consequences[side], outside[side], 0, 1));
        substrate.add_arrow(arrow(traces[side], continuations[side], 1, 1));
        substrate.add_arrow(arrow(hub, traces[side], 0, 1));
    }

    World {
        substrate,
        namespace,
        arrivals,
        correspondence_ends,
        continuations,
        consequences,
        acquisition_drivers,
        participation_drivers,
        independent_drivers,
        context,
    }
}

fn measure_execution(world: &World, tick: i64) -> ExecutionMetrics {
    let mut clone = world.clone();
    clone.substrate.advance_time(tick);
    for side in 0..SIDES {
        enter_many(
            &mut clone.substrate,
            clone.arrivals[side],
            tick,
            SOURCE_THRESHOLD,
            clone.namespace + 0x6_000 + side as u64 * 0x10,
        );
    }
    enter_many(
        &mut clone.substrate,
        clone.context,
        tick,
        1,
        clone.namespace + 0x7_000,
    );
    let run = clone.substrate.propagate();
    let continuation_firings =
        std::array::from_fn(|side| firings_at(&run, continuation_physical(clone.namespace, side)));
    let consequence_firings =
        std::array::from_fn(|side| firings_at(&run, consequence_physical(clone.namespace, side)));
    let trace_arrivals = std::array::from_fn(|side| {
        arrivals_at_tick(&run, trace_physical(clone.namespace, side), tick + 5)
    });
    let trace_firings =
        std::array::from_fn(|side| firings_at(&run, trace_physical(clone.namespace, side)));
    let local_returns = std::array::from_fn(|side| {
        arrivals_at_tick(&run, continuation_physical(clone.namespace, side), tick + 6)
    });
    let effects = std::array::from_fn(|side| outward_effects(&run, clone.namespace, side));
    let arrival_firings = (0..SIDES)
        .map(|side| firings_at(&run, arrival_physical(clone.namespace, side)))
        .sum::<usize>();
    ExecutionMetrics {
        continuation_firings,
        consequence_firings,
        trace_arrivals,
        trace_firings,
        local_returns,
        effects,
        extra_arrival_firings: arrival_firings.saturating_sub(SIDES),
        quiescent: run.naturally_quiescent,
    }
}

fn probe_passed(scenario: Scenario, metrics: &Metrics) -> bool {
    let participants = scenario
        .participants()
        .map(|value| usize::from(value) * EXPERIENCES);
    let expected_mature = scenario.expected_mature();
    let expected_effects = expected_mature.map(usize::from);
    let expected_trace_arrivals = if scenario.return_enabled() {
        [EXPERIENCES * 2, EXPERIENCES * 2]
    } else {
        metrics.training_consequence_firings
    };
    let expected_trace_firings = if scenario.return_enabled() {
        [EXPERIENCES, EXPERIENCES]
    } else {
        [0, 0]
    };
    let expected_heldout_trace_arrivals = if expected_mature.into_iter().any(|value| value) {
        expected_mature.map(|mature| 1 + usize::from(mature))
    } else {
        [0, 0]
    };
    let consequence_firings_exact = if scenario == Scenario::BlockedReturn {
        metrics.training_consequence_firings[0] > 0
            && metrics.training_consequence_firings[1] == EXPERIENCES
    } else {
        metrics.training_consequence_firings == [EXPERIENCES, EXPERIENCES]
    };
    let effects_exact = if scenario == Scenario::BlockedReturn {
        metrics.training_effects == metrics.training_consequence_firings
    } else {
        metrics.training_effects == [EXPERIENCES, EXPERIENCES]
    };
    metrics
        .correspondence_resistance
        .iter()
        .all(|value| *value > 1)
        && metrics.training_continuation_firings == participants
        && consequence_firings_exact
        && metrics.training_trace_arrivals == expected_trace_arrivals
        && metrics.training_trace_firings == expected_trace_firings
        && metrics.training_local_returns == expected_trace_firings
        && effects_exact
        && metrics.training_extra_arrival_firings == 0
        && (0..SIDES).all(|side| {
            if expected_mature[side] {
                metrics.directional_resistance[side] > 3
            } else {
                metrics.directional_resistance[side] == 0
            }
        })
        && metrics.heldout.continuation_firings == [1, 1]
        && metrics.heldout.consequence_firings == expected_effects
        && metrics.heldout.trace_arrivals == expected_heldout_trace_arrivals
        && metrics.heldout.trace_firings == expected_effects
        && metrics.heldout.local_returns == expected_effects
        && metrics.heldout.effects == expected_effects
        && metrics.postgap.effects == expected_effects
        && metrics.heldout.extra_arrival_firings == 0
        && metrics.postgap.extra_arrival_firings == 0
        && metrics.training_quiescent
        && metrics.heldout.quiescent
        && metrics.postgap.quiescent
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

fn arrival_physical(namespace: u64, side: usize) -> u64 {
    namespace + 10 + side as u64
}

fn correspondence_physical(namespace: u64, side: usize) -> u64 {
    namespace + 20 + side as u64
}

fn continuation_physical(namespace: u64, side: usize) -> u64 {
    namespace + 30 + side as u64
}

fn consequence_physical(namespace: u64, side: usize) -> u64 {
    namespace + 40 + side as u64
}

fn trace_physical(namespace: u64, side: usize) -> u64 {
    namespace + 50 + side as u64
}

fn firings_at(run: &Execution, physical: u64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn arrivals_at_tick(run: &Execution, physical: u64, tick: i64) -> usize {
    run.trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.tick == tick)
        .count()
}

fn outward_effects(run: &Execution, namespace: u64, side: usize) -> usize {
    run.crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == consequence_physical(namespace, side)
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
        "scenario,correspondence_resistance,directional_resistance,training_continuation_firings,training_consequence_firings,training_trace_arrivals,training_trace_firings,training_local_returns,training_effects,training_extra_arrival_firings,heldout_continuation_firings,heldout_consequence_firings,heldout_trace_arrivals,heldout_trace_firings,heldout_local_returns,heldout_effects,postgap_effects,heldout_extra_arrival_firings,postgap_extra_arrival_firings,training_quiescent,heldout_quiescent,postgap_quiescent,work,fingerprint,duplicate_exact,passed\n",
    );
    for row in rows {
        let value = &row.metrics;
        let fields = vec![
            row.scenario.name().to_string(),
            pair_u32(value.correspondence_resistance),
            pair_u32(value.directional_resistance),
            pair_usize(value.training_continuation_firings),
            pair_usize(value.training_consequence_firings),
            pair_usize(value.training_trace_arrivals),
            pair_usize(value.training_trace_firings),
            pair_usize(value.training_local_returns),
            pair_usize(value.training_effects),
            value.training_extra_arrival_firings.to_string(),
            pair_usize(value.heldout.continuation_firings),
            pair_usize(value.heldout.consequence_firings),
            pair_usize(value.heldout.trace_arrivals),
            pair_usize(value.heldout.trace_firings),
            pair_usize(value.heldout.local_returns),
            pair_usize(value.heldout.effects),
            pair_usize(value.postgap.effects),
            value.heldout.extra_arrival_firings.to_string(),
            value.postgap.extra_arrival_firings.to_string(),
            value.training_quiescent.to_string(),
            value.heldout.quiescent.to_string(),
            value.postgap.quiescent.to_string(),
            value.work.total().to_string(),
            value.fingerprint.to_string(),
            row.duplicate_exact.to_string(),
            row.passed.to_string(),
        ];
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

fn markdown(rows: &[ResultRow]) -> String {
    let passed = rows.iter().all(|row| row.passed);
    let mut output = format!(
        "# PX2 physical causal direction trace-sufficiency PROBE v2\n\nOutcome: **{}** (`{}/{}` worlds).\n\n| world | continuation fire | consequence fire | trace arrival/fire | local return | directional resistance | held-out continuation/consequence | held-out trace arrival/fire | held-out local/effect | post-gap effect | source refire train/held/post | quiescent train/held/post | replay | pass |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
        if passed { "POSITIVE" } else { "NEGATIVE" },
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
    );
    for row in rows {
        let value = &row.metrics;
        output.push_str(&format!(
            "| {} | `{}` | `{}` | `{}/{}` | `{}` | `{}` | `{}/{}` | `{}/{}` | `{}/{}` | `{}` | `{}/{}/{}` | `{}/{}/{}` | {} | {} |\n",
            row.scenario.name(),
            pair_usize(value.training_continuation_firings),
            pair_usize(value.training_consequence_firings),
            pair_usize(value.training_trace_arrivals),
            pair_usize(value.training_trace_firings),
            pair_usize(value.training_local_returns),
            pair_u32(value.directional_resistance),
            pair_usize(value.heldout.continuation_firings),
            pair_usize(value.heldout.consequence_firings),
            pair_usize(value.heldout.trace_arrivals),
            pair_usize(value.heldout.trace_firings),
            pair_usize(value.heldout.local_returns),
            pair_usize(value.heldout.effects),
            pair_usize(value.postgap.effects),
            value.training_extra_arrival_firings,
            value.heldout.extra_arrival_firings,
            value.postgap.extra_arrival_firings,
            value.training_quiescent,
            value.heldout.quiescent,
            value.postgap.quiescent,
            row.duplicate_exact,
            row.passed,
        ));
    }
    output.push_str(
        "\nNo PX0/PX1 physics changed. PX2 remains non-authoritative; no definitive evidence was executed.\n",
    );
    output
}

fn write_new(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create PX2 PROBE artifact");
    file.write_all(contents.as_bytes())
        .expect("write PX2 PROBE artifact");
    file.sync_all().expect("sync PX2 PROBE artifact");
}
