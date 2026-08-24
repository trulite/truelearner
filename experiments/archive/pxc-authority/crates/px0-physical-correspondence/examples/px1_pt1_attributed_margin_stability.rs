use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{rename, OpenOptions};
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
const MICRO_V1_SHA256: &str = "d32e174f77c2440c52baf8978370ff684ddfffb45a72938cbc8cdbb7099f93c2";
const MICRO_V1_AUDIT_SHA256: &str =
    "05a847240a6186a3705b591487673905d6b329090145cbebcdc1094e560a9e36";
const MICRO_V2_PROTOCOL_SHA256: &str =
    "afe799afbd04c81a157065e5e55ead73eca3bb75ca87e4abd61511cfecbfa326";
const MICRO_V2_SHA256: &str = "4e315c3f30b62c4cd6168a86e29564647b37d047ae686e0fbd2f0626b4f90025";
const MICRO_V2_AUDIT_SHA256: &str =
    "4603cf59c0e28fc3cd2d238107840285a234155ea750127bcf58db20a10497d3";
const GATE_PROTOCOL_SHA256: &str =
    "75c0e60a64255d18f350d2912cb3c48cb9652225edce15b17f400297d0710b67";
const GATE_SHA256: &str = "1b75fb15972e226d4cb047c69925f9d5452601b3b15be17434aedb5b37935ebf";
const GATE_AUDIT_SHA256: &str = "32d01b3c3bb5b101d51865e0367509556bb34ca80bc5054589fdbffce0e2a84a";
const READINESS_SHA256: &str = "8d08350ecbd03ba336447f5cb24c28120f57f08448d057e8d2542d5d4690da75";
const DEFINITIVE_PROTOCOL_SHA256: &str =
    "166cabd14f3c1d53830fc673530cb6d7c0f32125468c4120ace04025e7586bef";
const RESULT_CSV: &str = "results/px1_physical_boundary_roles_definitive.csv";
const RESULT_MD: &str = "results/px1_physical_boundary_roles_definitive.md";
const STAGING_CSV: &str = "results/.px1_physical_boundary_roles_definitive.csv.staging";
const STAGING_MD: &str = "results/.px1_physical_boundary_roles_definitive.md.staging";
const DEFINITIVE_SEEDS: usize = 16;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Stratum {
    name: &'static str,
    side_spacing: i32,
    exposure_spacing: i64,
    heldout_gap: i64,
    postgap_gap: i64,
    mirror: bool,
    reverse: bool,
}

const STRATA: [Stratum; 6] = [
    Stratum {
        name: "S0",
        side_spacing: 40,
        exposure_spacing: 12,
        heldout_gap: 20,
        postgap_gap: 60,
        mirror: false,
        reverse: false,
    },
    Stratum {
        name: "S1",
        side_spacing: 40,
        exposure_spacing: 12,
        heldout_gap: 24,
        postgap_gap: 64,
        mirror: true,
        reverse: true,
    },
    Stratum {
        name: "S2",
        side_spacing: 32,
        exposure_spacing: 11,
        heldout_gap: 18,
        postgap_gap: 52,
        mirror: false,
        reverse: true,
    },
    Stratum {
        name: "S3",
        side_spacing: 48,
        exposure_spacing: 11,
        heldout_gap: 22,
        postgap_gap: 56,
        mirror: true,
        reverse: false,
    },
    Stratum {
        name: "S4",
        side_spacing: 56,
        exposure_spacing: 13,
        heldout_gap: 24,
        postgap_gap: 68,
        mirror: false,
        reverse: false,
    },
    Stratum {
        name: "S5",
        side_spacing: 64,
        exposure_spacing: 13,
        heldout_gap: 28,
        postgap_gap: 72,
        mirror: true,
        reverse: true,
    },
];

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
    seed: usize,
    stratum: &'static str,
    scenario: Scenario,
    metrics: Metrics,
    duplicate_exact: bool,
    claims: Claims,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Claims {
    p0: bool,
    p1: bool,
    p2: bool,
    p3: bool,
    p4: bool,
    p5: bool,
    p6: bool,
    p7: bool,
    p8: bool,
    p9: bool,
    p10: bool,
    p11: bool,
    p12: bool,
}

impl Claims {
    fn all(&self) -> bool {
        self.count() == 13
    }

    fn count(&self) -> usize {
        [
            self.p0, self.p1, self.p2, self.p3, self.p4, self.p5, self.p6, self.p7, self.p8,
            self.p9, self.p10, self.p11, self.p12,
        ]
        .into_iter()
        .filter(|value| *value)
        .count()
    }
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let preflight = args == ["--preflight"];
    let definitive = args == ["--definitive"];
    if !preflight && !definitive {
        eprintln!("PX1 authority requires --preflight or --definitive");
        std::process::exit(2);
    }
    assert!(
        source_audit(),
        "frozen PX0/PT0/PT1 inputs must remain exact"
    );
    for path in [RESULT_CSV, RESULT_MD, STAGING_CSV, STAGING_MD] {
        assert!(
            !Path::new(path).exists(),
            "authority artifact exists: {path}"
        );
    }
    if preflight {
        println!("PX1_PHYSICAL_BOUNDARY_ROLES_DEFINITIVE_PREFLIGHT_OK");
        return;
    }
    eprintln!("PX1_PHYSICAL_BOUNDARY_ROLES_DEFINITIVE_EVIDENCE_SPENT");

    let mut rows = Vec::new();
    for seed in 0..DEFINITIVE_SEEDS {
        let stratum = STRATA[seed % STRATA.len()];
        for (scenario_ordinal, scenario) in Scenario::ALL.into_iter().enumerate() {
            let namespace = 0xd100_0000_0000
                + seed as u64 * 0x0100_0000
                + scenario_ordinal as u64 * 0x0010_0000;
            let first = run_world(namespace, scenario, stratum);
            let second = run_world(namespace, scenario, stratum);
            let duplicate_exact = first == second;
            let claims = definitive_claims(scenario, &first, duplicate_exact);
            let passed = claims.all();
            rows.push(ResultRow {
                seed,
                stratum: stratum.name,
                scenario,
                metrics: first,
                duplicate_exact,
                claims,
                passed,
            });
        }
    }
    publish_results(&csv(&rows), &markdown(&rows));
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
        && sha256("results/px1_pt1_attributed_margin_stability_micro_v1.csv") == MICRO_V1_SHA256
        && sha256("experiments/px1_pt1_attributed_margin_stability_micro_v1_negative_audit.md")
            == MICRO_V1_AUDIT_SHA256
        && sha256("experiments/px1_pt1_attributed_margin_stability_micro_v2_protocol.md")
            == MICRO_V2_PROTOCOL_SHA256
        && sha256("results/px1_pt1_attributed_margin_stability_micro_v2.csv") == MICRO_V2_SHA256
        && sha256("experiments/px1_pt1_attributed_margin_stability_micro_v2_result_audit.md")
            == MICRO_V2_AUDIT_SHA256
        && sha256("experiments/px1_pt1_attributed_margin_stability_gate_protocol.md")
            == GATE_PROTOCOL_SHA256
        && sha256("results/px1_pt1_attributed_margin_stability_gate_v1.csv") == GATE_SHA256
        && sha256("experiments/px1_pt1_attributed_margin_stability_gate_result_audit.md")
            == GATE_AUDIT_SHA256
        && sha256("experiments/px1_physical_boundary_roles_development_readiness.md")
            == READINESS_SHA256
        && sha256("experiments/px1_physical_boundary_roles_definitive_protocol.md")
            == DEFINITIVE_PROTOCOL_SHA256
}

fn run_world(namespace: u64, scenario: Scenario, stratum: Stratum) -> Metrics {
    let mut world = build_world(
        namespace,
        stratum.mirror,
        stratum.reverse,
        stratum.side_spacing,
        scenario.return_enabled(),
    );
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
        let tick = 66 + exposure as i64 * stratum.exposure_spacing;
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

    let last_exposure_tick = 66 + (EXPOSURES as i64 - 1) * stratum.exposure_spacing;
    let heldout = measure_execution(&world, last_exposure_tick + stratum.heldout_gap);
    let postgap = measure_execution(&world, last_exposure_tick + stratum.postgap_gap);
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

fn build_world(
    namespace: u64,
    mirror: bool,
    reverse: bool,
    side_spacing: i32,
    return_enabled: bool,
) -> World {
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
        let base = slot as i32 * side_spacing;
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
    let shared_return = usize::from(expected_mature.into_iter().any(|value| value));
    expected_mature.map(|mature| shared_return + usize::from(mature))
}

fn branch_outlet_passed(scenario: Scenario, metrics: &Metrics) -> bool {
    if scenario == Scenario::BlockedReturn {
        metrics.branch_firings == [EXPOSURES, 0]
            && metrics.outlet_firings[0] > 0
            && metrics.outlet_firings[1] == 0
    } else {
        let expected = expected_training(scenario);
        metrics.branch_firings == expected.branches && metrics.outlet_firings == expected.outlets
    }
}

fn trace_arrivals_passed(scenario: Scenario, metrics: &Metrics) -> bool {
    if scenario == Scenario::BlockedReturn {
        metrics.trace_arrivals == metrics.outlet_firings
    } else {
        metrics.trace_arrivals == expected_training(scenario).trace_arrivals
    }
}

fn trace_firings_passed(scenario: Scenario, metrics: &Metrics) -> bool {
    if scenario == Scenario::BlockedReturn {
        metrics.trace_firings == [0, 0]
    } else {
        metrics.trace_firings == expected_training(scenario).trace_firings
    }
}

fn local_returns_passed(scenario: Scenario, metrics: &Metrics) -> bool {
    if scenario == Scenario::BlockedReturn {
        metrics.local_returns == [0, 0]
    } else {
        metrics.local_returns == expected_training(scenario).local_returns
    }
}

fn definitive_claims(scenario: Scenario, metrics: &Metrics, duplicate_exact: bool) -> Claims {
    let expected_mature = scenario.expected_mature();
    let expected_effects = expected_mature.map(usize::from);
    let expected_trace_arrivals = expected_heldout_trace_arrivals(expected_mature);
    Claims {
        p0: true,
        p1: metrics.correspondence_acquired,
        p2: branch_outlet_passed(scenario, metrics),
        p3: trace_arrivals_passed(scenario, metrics),
        p4: trace_firings_passed(scenario, metrics),
        p5: local_returns_passed(scenario, metrics),
        p6: metrics.maturation_exact,
        p7: metrics.heldout_branch_firings == [1, 1]
            && metrics.heldout_outlet_firings == expected_effects,
        p8: metrics.heldout_trace_arrivals == expected_trace_arrivals
            && metrics.heldout_trace_firings == expected_effects,
        p9: metrics.heldout_local_returns == expected_effects
            && metrics.heldout_effects == expected_effects,
        p10: metrics.postgap_effects == expected_effects,
        p11: metrics.extra_source_firings == 0
            && metrics.heldout_extra_source_firings == 0
            && metrics.postgap_extra_source_firings == 0,
        p12: metrics.naturally_quiescent
            && metrics.heldout_quiescent
            && metrics.postgap_quiescent
            && duplicate_exact,
    }
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
        "seed,stratum,scenario,correspondence_resistance,continuation_resistance,training_branch_firings,training_outlet_firings,training_trace_arrivals,training_trace_firings,training_local_returns,training_extra_source_firings,heldout_branch_firings,heldout_outlet_firings,heldout_trace_arrivals,heldout_trace_firings,heldout_local_returns,heldout_effects,postgap_effects,heldout_extra_source_firings,postgap_extra_source_firings,heldout_quiescent,postgap_quiescent,correspondence_acquired,maturation_exact,postgap_exact,training_quiescent,duplicate_exact,work,fingerprint,p0,p1,p2,p3,p4,p5,p6,p7,p8,p9,p10,p11,p12,passed\n",
    );
    for row in rows {
        let value = &row.metrics;
        let fields = vec![
            row.seed.to_string(),
            row.stratum.to_string(),
            row.scenario.name().to_string(),
            pair_u32(value.correspondence_resistance),
            pair_u32(value.continuation_resistance),
            pair_usize(value.branch_firings),
            pair_usize(value.outlet_firings),
            pair_usize(value.trace_arrivals),
            pair_usize(value.trace_firings),
            pair_usize(value.local_returns),
            value.extra_source_firings.to_string(),
            pair_usize(value.heldout_branch_firings),
            pair_usize(value.heldout_outlet_firings),
            pair_usize(value.heldout_trace_arrivals),
            pair_usize(value.heldout_trace_firings),
            pair_usize(value.heldout_local_returns),
            pair_usize(value.heldout_effects),
            pair_usize(value.postgap_effects),
            value.heldout_extra_source_firings.to_string(),
            value.postgap_extra_source_firings.to_string(),
            value.heldout_quiescent.to_string(),
            value.postgap_quiescent.to_string(),
            value.correspondence_acquired.to_string(),
            value.maturation_exact.to_string(),
            value.postgap_exact.to_string(),
            value.naturally_quiescent.to_string(),
            row.duplicate_exact.to_string(),
            value.work.total().to_string(),
            value.fingerprint.to_string(),
            row.claims.p0.to_string(),
            row.claims.p1.to_string(),
            row.claims.p2.to_string(),
            row.claims.p3.to_string(),
            row.claims.p4.to_string(),
            row.claims.p5.to_string(),
            row.claims.p6.to_string(),
            row.claims.p7.to_string(),
            row.claims.p8.to_string(),
            row.claims.p9.to_string(),
            row.claims.p10.to_string(),
            row.claims.p11.to_string(),
            row.claims.p12.to_string(),
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
        "# PX1 physical boundary roles definitive result\n\nOutcome: **{}** (`{}/{}` cells; `{}/{}` claims).\n\n| seed | stratum | scenario | claims | train branch/outlet | train trace arrival/fire | train local return | resistance | held-out branch/outlet | held-out trace arrival/fire | held-out local return/effect | post-gap effect | source refire train/held/post | quiescent train/held/post | replay | pass |\n|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",
        if passed { "POSITIVE" } else { "NEGATIVE" },
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
        rows.iter().map(|row| row.claims.count()).sum::<usize>(),
        rows.len() * 13,
    );
    for row in rows {
        let value = &row.metrics;
        output.push_str(&format!(
            "| {} | {} | {} | {}/13 | `{}/{}` | `{}/{}` | `{}` | `{}` | `{}/{}` | `{}/{}` | `{}/{}` | `{}` | `{}/{}/{}` | `{}/{}/{}` | {} | {} |\n",
            row.seed,
            row.stratum,
            row.scenario.name(),
            row.claims.count(),
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
        "\nEvery physical stage and P0–P12 are serialized separately. PX0 changed: `false`. Authority consequence remains subject to the frozen result audit and handoff.\n",
    );
    output
}

fn write_staging(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("create PX1 authority staging artifact");
    file.write_all(contents.as_bytes())
        .expect("write PX1 authority staging artifact");
    file.sync_all()
        .expect("sync PX1 authority staging artifact");
}

fn publish_results(csv_contents: &str, md_contents: &str) {
    write_staging(STAGING_CSV, csv_contents);
    write_staging(STAGING_MD, md_contents);
    rename(STAGING_CSV, RESULT_CSV).expect("publish PX1 authority CSV");
    rename(STAGING_MD, RESULT_MD).expect("publish PX1 authority report");
}
