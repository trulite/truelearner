#![forbid(unsafe_code)]

use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Crossing, Execution, PlasticSubstrate, SpikeInput,
    TraceEntry, WorkLedger,
};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX0_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const PX1_SHA256: &str = "74716c87d146cb697b37ddf802c12e67a5cb93daf82ec20f8b982e54922bd696";
const PX2_SHA256: &str = "c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5";
const PRIOR_PX3_SHA256: &str = "39ec595fc1204a29083d271ebcadcdb7950c07d1c44e4ce07c0107fca54730ba";
const PRIOR_PX3_AUDIT_SHA256: &str = "a029f250ed88f8f2fc164e0d2c9042675bf0a8c9ae51c89cf83ad1aa42e4fa9b";
const CJ1_T_AUDIT_SHA256: &str = "f5dd663eab0717d67fb00ef97792cced46ae79ad8d0915c2f6d76899e2945ae4";
const CJ1_PA_AUDIT_SHA256: &str = "1bcb24c7736e8aeab8dfc3eb3ebf4da3ede2ec9ea324d52ba546aecc78f36aca";
const CJ1_HANDOFF_SHA256: &str = "5157ea555fdf1aa8b7d7b9ad89bf762dc908c05ab8be00f0deb3dbed6ebdc60e";
const DEVELOPMENT_PROTOCOL_SHA256: &str =
    "61ce9535ccab0133db660f8fc4d1e408bc61c2f9d8948877f34a431ec063c9c7";
const PROBE_PROTOCOL_V1_SHA256: &str =
    "ca95a80eef60b13f5e1b975533723de46e591bc96621965beaa1b903501f1de7";
const PROBE_PROTOCOL_V2_SHA256: &str =
    "a7ce66c9ffc1fe20dc85334d8c7622855cddfd1971f5ce7785d50bf2acd410cb";

const SEEDS: [u64; 2] = [2601, 2609];
const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
const CSV_PATH: &str = "results/px3_participation_trace_organization_probe_v1.csv";
const MD_PATH: &str = "results/px3_participation_trace_organization_probe_v1.md";
const CSV_STAGE: &str =
    "results/.px3_participation_trace_organization_probe_v1.csv.staging";
const MD_STAGE: &str =
    "results/.px3_participation_trace_organization_probe_v1.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    AAlone,
    BAlone,
    A4Alone,
    ARepeated,
    ALateB,
    AbOne,
    AbRecurrent11,
    AbRecurrent21,
    AbRecurrent44,
    AbHeldoutMatrix,
    AbBlockedReturn,
    ProposalOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scenario {
    name: &'static str,
    kind: Kind,
    couplings: [i32; 4],
}

impl Scenario {
    const ALL: [Self; 12] = [
        Self::new("a-alone", Kind::AAlone, [1, 1, 1, 1]),
        Self::new("b-alone", Kind::BAlone, [1, 1, 1, 1]),
        Self::new("a4-alone", Kind::A4Alone, [4, 1, 1, 1]),
        Self::new("a-repeated", Kind::ARepeated, [1, 1, 1, 1]),
        Self::new("a-then-b-late", Kind::ALateB, [1, 1, 1, 1]),
        Self::new("ab-one-overlap", Kind::AbOne, [1, 1, 1, 1]),
        Self::new("ab-recurrent-1-1", Kind::AbRecurrent11, [1, 1, 1, 1]),
        Self::new("ab-recurrent-2-1", Kind::AbRecurrent21, [2, 1, 1, 1]),
        Self::new("ab-recurrent-4-4", Kind::AbRecurrent44, [4, 4, 1, 1]),
        Self::new(
            "ab-recurrent-heldout-matrix",
            Kind::AbHeldoutMatrix,
            [1, 1, 1, 1],
        ),
        Self::new("ab-blocked-return", Kind::AbBlockedReturn, [1, 1, 1, 1]),
        Self::new("proposal-only", Kind::ProposalOnly, [1, 1, 1, 1]),
    ];

    const fn new(name: &'static str, kind: Kind, couplings: [i32; 4]) -> Self {
        Self {
            name,
            kind,
            couplings,
        }
    }

    fn recurrent(self) -> bool {
        matches!(
            self.kind,
            Kind::AbRecurrent11
                | Kind::AbRecurrent21
                | Kind::AbRecurrent44
                | Kind::AbHeldoutMatrix
        )
    }
}

#[derive(Clone)]
struct World {
    namespace: u64,
    substrate: PlasticSubstrate,
    sources: [CellId; 4],
    context: CellId,
    candidates: [ArrowId; 6],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Log {
    trace: Vec<TraceEntry>,
    crossings: Vec<Crossing>,
    work: WorkLedger,
    quiescent: bool,
}

impl Log {
    fn execution(&mut self, execution: Execution) {
        self.trace.extend(execution.trace);
        self.crossings.extend(execution.crossings);
        add_work(&mut self.work, &execution.work);
        self.quiescent = execution.naturally_quiescent;
    }

    fn work(&mut self, work: WorkLedger) {
        add_work(&mut self.work, &work);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    scenario: &'static str,
    couplings: [i32; 4],
    scheduled: [usize; 4],
    source_firings: [usize; 4],
    raw_traversals: [usize; 4],
    raw_impulse: [i32; 4],
    outlet_firings: [usize; 4],
    unit_participation: [usize; 4],
    trace_firings: [usize; 4],
    trace_ticks: [String; 4],
    opportunity_firings: [usize; 6],
    candidate_traversals: [usize; 6],
    consequence_firings: [usize; 6],
    return_arrivals: [usize; 6],
    proposals: u64,
    live_after_train: [bool; 6],
    resistance_after_train: [u32; 6],
    coupling_after_train: [i32; 6],
    live_after_gap: [bool; 6],
    resistance_after_gap: [u32; 6],
    coupling_after_gap: [i32; 6],
    heldout_trained: usize,
    heldout_crossed: usize,
    heldout_gapped: usize,
    heldout_singleton: usize,
    work: WorkLedger,
    persistent_bytes: usize,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    quiescent: bool,
    replay_equal: bool,
    passed: bool,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    source_audit();
    frozen_surface_audit();
    require_absent(&[CSV_PATH, MD_PATH, CSV_STAGE, MD_STAGE]);
    match args.as_slice() {
        [flag] if flag == "--preflight" => {
            println!("PX3_PARTICIPATION_TRACE_ORGANIZATION_PROBE_PREFLIGHT_OK");
        }
        [flag] if flag == "--probe" => run_probe(),
        _ => {
            eprintln!("PX3 permits only --preflight or its frozen write-once --probe");
            std::process::exit(2);
        }
    }
}

fn run_probe() {
    eprintln!("PX3_PARTICIPATION_TRACE_ORGANIZATION_PROBE_EVIDENCE");
    let mut rows = Vec::with_capacity(SEEDS.len() * Scenario::ALL.len());
    for seed in SEEDS {
        for scenario in Scenario::ALL {
            rows.push(run_replay(seed, scenario));
        }
    }
    audit_rows(&rows);
    publish(CSV_STAGE, CSV_PATH, &csv(&rows));
    publish(MD_STAGE, MD_PATH, &report(&rows));
}

fn source_audit() {
    let frozen = [
        ("crates/px0-physical-correspondence/src/lib.rs", PX0_SHA256),
        (
            "crates/px0-physical-correspondence/examples/px1_pt1_attributed_margin_stability.rs",
            PX1_SHA256,
        ),
        (
            "crates/px0-physical-correspondence/examples/px2_physical_causal_direction.rs",
            PX2_SHA256,
        ),
        (
            "crates/px0-physical-correspondence/examples/px3_physical_event_boundaries_probe_v3.rs",
            PRIOR_PX3_SHA256,
        ),
        (
            "experiments/px3_physical_event_boundaries_frozen_negative_handoff.md",
            PRIOR_PX3_AUDIT_SHA256,
        ),
        (
            "experiments/cj1_t_refractory_trace_geometry_result_audit_v1.md",
            CJ1_T_AUDIT_SHA256,
        ),
        (
            "experiments/cj1_pa_participation_amplitude_geometry_result_audit_v1.md",
            CJ1_PA_AUDIT_SHA256,
        ),
        (
            "experiments/cj1_physical_participation_conjunction_handoff_v1.md",
            CJ1_HANDOFF_SHA256,
        ),
        (
            "experiments/px3_participation_trace_organization_development_protocol_v1.md",
            DEVELOPMENT_PROTOCOL_SHA256,
        ),
        (
            "experiments/px3_participation_trace_organization_probe_protocol_v1.md",
            PROBE_PROTOCOL_V1_SHA256,
        ),
        (
            "experiments/px3_participation_trace_organization_probe_protocol_v2.md",
            PROBE_PROTOCOL_V2_SHA256,
        ),
    ];
    for (path, expected) in frozen {
        assert_eq!(sha256(path), expected, "frozen input hash drift: {path}");
    }
}

fn frozen_surface_audit() {
    assert_eq!(SEEDS, [2601, 2609]);
    assert_eq!(Scenario::ALL.len(), 12);
    assert_eq!(PAIRS, [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let names = Scenario::ALL
        .iter()
        .map(|scenario| scenario.name)
        .collect::<BTreeSet<_>>();
    assert_eq!(names.len(), Scenario::ALL.len());
    for forbidden in [
        "arms/px3-participation-trace-organization/src/micro.rs",
        "arms/px3-participation-trace-organization/src/gate.rs",
        "results/px3_participation_trace_organization_micro_v1.csv",
        "results/px3_participation_trace_organization_gate_v1.csv",
    ] {
        assert!(!Path::new(forbidden).exists(), "later-stage surface exists: {forbidden}");
    }
}

fn run_replay(seed: u64, scenario: Scenario) -> Row {
    let first = run(seed, scenario);
    let second = run(seed, scenario);
    let replay_equal = first == second;
    let mut row = first;
    row.replay_equal = replay_equal;
    row.passed &= replay_equal;
    row
}

fn run(seed: u64, scenario: Scenario) -> Row {
    let namespace = (seed << 32) | ((scenario_index(scenario) as u64 + 1) << 16);
    let return_enabled = scenario.kind != Kind::AbBlockedReturn;
    let (mut world, prime) = build_world(namespace, scenario.couplings, return_enabled);
    let proposals = prime.work.local_structural_proposals;
    let mut log = Log {
        quiescent: true,
        ..Log::default()
    };
    let mut scheduled = [0; 4];

    match scenario.kind {
        Kind::AAlone | Kind::A4Alone => {
            expose(&mut world, 2, [true, false, false, false], true, false, &mut log);
            scheduled[0] = 1;
        }
        Kind::BAlone => {
            expose(&mut world, 2, [false, true, false, false], true, false, &mut log);
            scheduled[1] = 1;
        }
        Kind::ARepeated => {
            expose(&mut world, 2, [true, false, false, false], true, true, &mut log);
            scheduled[0] = 2;
        }
        Kind::ALateB => {
            enter_source(&mut world, 0, 2, 0);
            enter_source(&mut world, 1, 4, 1);
            enter_context(&mut world, 3);
            log.execution(world.substrate.propagate());
            scheduled = [1, 1, 0, 0];
        }
        Kind::AbOne => {
            expose(&mut world, 2, [true, true, false, false], true, false, &mut log);
            scheduled = [1, 1, 0, 0];
        }
        Kind::AbRecurrent11
        | Kind::AbRecurrent21
        | Kind::AbRecurrent44
        | Kind::AbHeldoutMatrix
        | Kind::AbBlockedReturn => {
            expose(&mut world, 2, [true, true, false, false], true, false, &mut log);
            expose(&mut world, 11, [true, true, false, false], true, false, &mut log);
            scheduled = [2, 2, 0, 0];
        }
        Kind::ProposalOnly => {}
    }

    let live_after_train = candidate_map(&world, |substrate, candidate| {
        substrate.arrow_is_live(candidate)
    });
    let resistance_after_train = candidate_map(&world, |substrate, candidate| {
        substrate.arrow_resistance(candidate)
    });
    let coupling_after_train = candidate_couplings(&world);

    let mut gap_world = world.clone();
    log.work(gap_world.substrate.advance_time(50));
    let live_after_gap = candidate_map(&gap_world, |substrate, candidate| {
        substrate.arrow_is_live(candidate)
    });
    let resistance_after_gap = candidate_map(&gap_world, |substrate, candidate| {
        substrate.arrow_resistance(candidate)
    });
    let coupling_after_gap = coupling_after_train;

    let heldout_trained = heldout_consequences(world.clone(), Heldout::TrainedAb);
    let heldout_crossed = heldout_consequences(world.clone(), Heldout::CrossedAd);
    let heldout_gapped = heldout_consequences(world.clone(), Heldout::GappedAb);
    let heldout_singleton = heldout_consequences(world.clone(), Heldout::SingletonA);

    let source_firings = four(|side| firings(&log.trace, source_physical(namespace, side)));
    let raw_traversals = four(|side| {
        crossings(
            &log.crossings,
            source_physical(namespace, side),
            outlet_physical(namespace, side),
        )
    });
    let raw_impulse = four(|side| {
        crossing_impulse(
            &log.crossings,
            source_physical(namespace, side),
            outlet_physical(namespace, side),
        )
    });
    let outlet_firings = four(|side| firings(&log.trace, outlet_physical(namespace, side)));
    let unit_participation = four(|side| {
        crossings(
            &log.crossings,
            outlet_physical(namespace, side),
            trace_physical(namespace, side),
        )
    });
    let trace_firings = four(|side| firings(&log.trace, trace_physical(namespace, side)));
    let trace_ticks = four(|side| firing_ticks(&log.trace, trace_physical(namespace, side)));
    let opportunity_firings = six(|pair| {
        firings(&log.trace, opportunity_physical(namespace, pair))
    });
    let candidate_traversals = six(|pair| {
        crossings(
            &log.crossings,
            opportunity_physical(namespace, pair),
            effect_physical(namespace, pair),
        )
    });
    let consequence_firings =
        six(|pair| firings(&log.trace, effect_physical(namespace, pair)));
    let return_arrivals =
        six(|pair| arrivals(&log.trace, opportunity_physical(namespace, pair))
            .saturating_sub(opportunity_trace_arrivals(&log, namespace, pair)));

    let persistent_bytes = world.substrate.persistent_bytes();
    let complete_fingerprint = world.substrate.complete_fingerprint();
    let permanent_fingerprint = world.substrate.permanent_fingerprint();
    let quiescent = prime.naturally_quiescent && log.quiescent;

    let mut row = Row {
        seed,
        scenario: scenario.name,
        couplings: scenario.couplings,
        scheduled,
        source_firings,
        raw_traversals,
        raw_impulse,
        outlet_firings,
        unit_participation,
        trace_firings,
        trace_ticks,
        opportunity_firings,
        candidate_traversals,
        consequence_firings,
        return_arrivals,
        proposals,
        live_after_train,
        resistance_after_train,
        coupling_after_train,
        live_after_gap,
        resistance_after_gap,
        coupling_after_gap,
        heldout_trained,
        heldout_crossed,
        heldout_gapped,
        heldout_singleton,
        work: log.work,
        persistent_bytes,
        complete_fingerprint,
        permanent_fingerprint,
        quiescent,
        replay_equal: false,
        passed: false,
    };
    row.passed = row_passes(&row, scenario);
    row
}

fn row_passes(row: &Row, scenario: Scenario) -> bool {
    let (expected_scheduled, expected_actual, expected_opportunities, expected_consequences) =
        expectations(scenario.kind);
    let expected_candidate_traversals = if scenario.kind == Kind::AbBlockedReturn {
        [1, 0, 0, 0, 0, 0]
    } else {
        expected_opportunities
    };
    let expected_raw_impulse = four(|side| {
        i32::try_from(expected_actual[side]).expect("small count") * scenario.couplings[side]
    });
    let expected_live_train = if scenario.recurrent() {
        [true, false, false, false, false, false]
    } else if scenario.kind == Kind::AbOne {
        [true, false, false, false, false, false]
    } else {
        [false; 6]
    };
    let expected_live_gap = if scenario.recurrent() {
        [true, false, false, false, false, false]
    } else {
        [false; 6]
    };
    let expected_train_resistance = if scenario.recurrent() {
        [6, 0, 0, 0, 0, 0]
    } else if scenario.kind == Kind::AbOne {
        [4, 0, 0, 0, 0, 0]
    } else {
        [0; 6]
    };
    let expected_train_coupling = if matches!(scenario.kind, Kind::AbOne) || scenario.recurrent() {
        [2, 1, 1, 1, 1, 1]
    } else {
        [1; 6]
    };
    let expected_gap_resistance = if scenario.recurrent() {
        [2, 0, 0, 0, 0, 0]
    } else {
        [0; 6]
    };
    let expected_heldout = usize::from(scenario.recurrent());

    row.proposals == 6
        && row.scheduled == expected_scheduled
        && row.source_firings == expected_actual
        && row.raw_traversals == expected_actual
        && row.raw_impulse == expected_raw_impulse
        && row.outlet_firings == expected_actual
        && row.unit_participation == expected_actual
        && row.trace_firings == expected_actual
        && row.opportunity_firings == expected_opportunities
        && row.candidate_traversals == expected_candidate_traversals
        && row.consequence_firings == expected_consequences
        && row.live_after_train == expected_live_train
        && row.resistance_after_train == expected_train_resistance
        && row.coupling_after_train == expected_train_coupling
        && row.live_after_gap == expected_live_gap
        && row.resistance_after_gap == expected_gap_resistance
        && row.coupling_after_gap == expected_train_coupling
        && row.heldout_trained == expected_heldout
        && row.heldout_crossed == 0
        && row.heldout_gapped == 0
        && row.heldout_singleton == 0
        && row.quiescent
}

fn expectations(kind: Kind) -> ([usize; 4], [usize; 4], [usize; 6], [usize; 6]) {
    match kind {
        Kind::AAlone | Kind::A4Alone => ([1, 0, 0, 0], [1, 0, 0, 0], [0; 6], [0; 6]),
        Kind::BAlone => ([0, 1, 0, 0], [0, 1, 0, 0], [0; 6], [0; 6]),
        Kind::ARepeated => ([2, 0, 0, 0], [1, 0, 0, 0], [0; 6], [0; 6]),
        Kind::ALateB => ([1, 1, 0, 0], [1, 1, 0, 0], [0; 6], [0; 6]),
        Kind::AbOne => ([1, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0]),
        Kind::AbRecurrent11
        | Kind::AbRecurrent21
        | Kind::AbRecurrent44
        | Kind::AbHeldoutMatrix => ([2, 2, 0, 0], [2, 2, 0, 0], [2, 0, 0, 0, 0, 0], [2, 0, 0, 0, 0, 0]),
        Kind::AbBlockedReturn => ([2, 2, 0, 0], [2, 2, 0, 0], [2, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0]),
        Kind::ProposalOnly => ([0; 4], [0; 4], [0; 6], [0; 6]),
    }
}

fn build_world(
    namespace: u64,
    couplings: [i32; 4],
    return_enabled: bool,
) -> (World, Execution) {
    let mut substrate = PlasticSubstrate::new();
    let sources = four(|side| {
        substrate.add_cell(cell(
            source_physical(namespace, side),
            -20_000 - side as i32 * 100,
            10 + side as i16,
            1,
        ))
    });
    let outlets = four(|side| {
        substrate.add_cell(cell(
            outlet_physical(namespace, side),
            -10_000 - side as i32 * 100,
            20 + side as i16,
            1,
        ))
    });
    let traces = four(|side| {
        substrate.add_cell(cell(
            trace_physical(namespace, side),
            -5_000 - side as i32 * 100,
            30 + side as i16,
            2,
        ))
    });
    let px1_hub = substrate.add_cell(cell(px1_hub_physical(namespace), -2_000, 40, 1));
    let opportunities = six(|pair| {
        substrate.add_cell(cell(
            opportunity_physical(namespace, pair),
            10_000 + pair as i32 * 100,
            50 + pair as i16,
            2,
        ))
    });
    let effects = six(|pair| {
        substrate.add_cell(cell(
            effect_physical(namespace, pair),
            10_002 + pair as i32 * 100,
            60 + pair as i16,
            2,
        ))
    });
    let context = substrate.add_cell(cell(context_physical(namespace), 30_000, 70, 1));
    let px3_hub = substrate.add_cell(cell(px3_hub_physical(namespace), 40_000, 80, 1));

    for side in 0..4 {
        substrate.add_arrow(arrow(sources[side], outlets[side], 0, couplings[side]));
        substrate.add_arrow(arrow(outlets[side], traces[side], 1, 1));
        substrate.add_arrow(arrow(outlets[side], px1_hub, 1, 1));
        substrate.add_arrow(arrow(px1_hub, traces[side], 0, 1));
    }
    for (pair, (left, right)) in PAIRS.into_iter().enumerate() {
        substrate.add_arrow(arrow(traces[left], opportunities[pair], 0, 1));
        substrate.add_arrow(arrow(traces[right], opportunities[pair], 0, 1));
        substrate.add_arrow(arrow(context, effects[pair], 2, 1));
        if return_enabled {
            substrate.add_arrow(arrow(effects[pair], px3_hub, 0, 1));
        }
        substrate.add_arrow(arrow(px3_hub, opportunities[pair], 1, 1));
    }

    for (pair, opportunity) in opportunities.into_iter().enumerate() {
        substrate.enter(SpikeInput {
            arrival_tick: 0,
            phase: pair as i32,
            origin_physical: namespace + 90_000 + pair as u64,
            target: opportunity,
            impulse: 2,
        });
    }
    let prime = substrate.propagate();
    assert!(prime.naturally_quiescent, "prime must quiesce");
    assert_eq!(prime.work.local_structural_proposals, 6);
    assert_eq!(substrate.arrow_count(), fixed_arrow_count(return_enabled) + 6);
    let candidates = six(|pair| {
        let candidates = substrate.arrows_between(opportunities[pair], effects[pair]);
        assert_eq!(candidates.len(), 1, "exactly one anonymous candidate");
        candidates[0]
    });
    for candidate in candidates {
        assert!(substrate.arrow_is_live(candidate));
        assert_eq!(substrate.arrow_resistance(candidate), 1);
    }

    (
        World {
            namespace,
            substrate,
            sources,
            context,
            candidates,
        },
        prime,
    )
}

fn fixed_arrow_count(return_enabled: bool) -> usize {
    4 * 4 + 6 * (2 + 1 + usize::from(return_enabled) + 1)
}

fn expose(
    world: &mut World,
    source_tick: i64,
    active: [bool; 4],
    with_context: bool,
    repeat_a: bool,
    log: &mut Log,
) {
    for (side, is_active) in active.into_iter().enumerate() {
        if is_active {
            enter_source(world, side, source_tick, side as i32);
        }
    }
    if repeat_a {
        enter_source(world, 0, source_tick, 100);
    }
    if with_context {
        enter_context(world, source_tick + 1);
    }
    log.execution(world.substrate.propagate());
}

fn enter_source(world: &mut World, side: usize, tick: i64, phase: i32) {
    world.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: 1_000_000 + side as u64,
        target: world.sources[side],
        impulse: 1,
    });
}

fn enter_context(world: &mut World, tick: i64) {
    world.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: 500,
        origin_physical: 2_000_000,
        target: world.context,
        impulse: 1,
    });
}

#[derive(Clone, Copy)]
enum Heldout {
    TrainedAb,
    CrossedAd,
    GappedAb,
    SingletonA,
}

fn heldout_consequences(mut world: World, heldout: Heldout) -> usize {
    world.substrate.advance_time(50);
    match heldout {
        Heldout::TrainedAb => {
            enter_source(&mut world, 0, 50, 0);
            enter_source(&mut world, 1, 50, 1);
        }
        Heldout::CrossedAd => {
            enter_source(&mut world, 0, 50, 0);
            enter_source(&mut world, 3, 50, 3);
        }
        Heldout::GappedAb => {
            enter_source(&mut world, 0, 50, 0);
            enter_source(&mut world, 1, 52, 1);
        }
        Heldout::SingletonA => enter_source(&mut world, 0, 50, 0),
    }
    let execution = world.substrate.propagate();
    assert!(execution.naturally_quiescent, "held-out use must quiesce");
    (0..6)
        .map(|pair| firings(&execution.trace, effect_physical(world.namespace, pair)))
        .sum()
}

fn candidate_map<T: Copy>(
    world: &World,
    mut inspect: impl FnMut(&PlasticSubstrate, ArrowId) -> T,
) -> [T; 6] {
    six(|pair| inspect(&world.substrate, world.candidates[pair]))
}

fn candidate_couplings(world: &World) -> [i32; 6] {
    // Frozen PX0 raises a positive candidate from coupling one to two exactly
    // when its eligible local return raises resistance by three. PX0 does not
    // expose an arrow-coupling getter, so this serialized value is the exact
    // law-derived post-return state, paired with native crossing impulses.
    candidate_map(world, |substrate, candidate| {
        if substrate.arrow_resistance(candidate) >= 4 {
            2
        } else {
            1
        }
    })
}

fn opportunity_trace_arrivals(log: &Log, namespace: u64, pair: usize) -> usize {
    let (left, right) = PAIRS[pair];
    crossings(
        &log.crossings,
        trace_physical(namespace, left),
        opportunity_physical(namespace, pair),
    ) + crossings(
        &log.crossings,
        trace_physical(namespace, right),
        opportunity_physical(namespace, pair),
    )
}

fn scenario_index(scenario: Scenario) -> usize {
    Scenario::ALL
        .iter()
        .position(|candidate| *candidate == scenario)
        .expect("frozen scenario")
}

fn four<T>(mut make: impl FnMut(usize) -> T) -> [T; 4] {
    [make(0), make(1), make(2), make(3)]
}

fn six<T>(mut make: impl FnMut(usize) -> T) -> [T; 6] {
    [make(0), make(1), make(2), make(3), make(4), make(5)]
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 100,
    }
}

fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 100,
    }
}

fn source_physical(namespace: u64, side: usize) -> u64 {
    namespace + 10 + side as u64
}

fn outlet_physical(namespace: u64, side: usize) -> u64 {
    namespace + 20 + side as u64
}

fn trace_physical(namespace: u64, side: usize) -> u64 {
    namespace + 30 + side as u64
}

fn px1_hub_physical(namespace: u64) -> u64 {
    namespace + 40
}

fn opportunity_physical(namespace: u64, pair: usize) -> u64 {
    namespace + 100 + pair as u64
}

fn effect_physical(namespace: u64, pair: usize) -> u64 {
    namespace + 200 + pair as u64
}

fn context_physical(namespace: u64) -> u64 {
    namespace + 300
}

fn px3_hub_physical(namespace: u64) -> u64 {
    namespace + 400
}

fn firings(trace: &[TraceEntry], physical: u64) -> usize {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn arrivals(trace: &[TraceEntry], physical: u64) -> usize {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical)
        .count()
}

fn firing_ticks(trace: &[TraceEntry], physical: u64) -> String {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .map(|entry| entry.tick.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

fn crossings(crossings: &[Crossing], from: u64, to: u64) -> usize {
    crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .count()
}

fn crossing_impulse(crossings: &[Crossing], from: u64, to: u64) -> i32 {
    crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .map(|crossing| crossing.impulse)
        .sum()
}

fn add_work(total: &mut WorkLedger, work: &WorkLedger) {
    total.queue_comparisons += work.queue_comparisons;
    total.spikes_delivered += work.spikes_delivered;
    total.generation_checks += work.generation_checks;
    total.state_updates += work.state_updates;
    total.threshold_checks += work.threshold_checks;
    total.firings += work.firings;
    total.arrow_checks += work.arrow_checks;
    total.spikes_emitted += work.spikes_emitted;
    total.local_eligibility_writes += work.local_eligibility_writes;
    total.local_return_updates += work.local_return_updates;
    total.ordinary_pressure_updates += work.ordinary_pressure_updates;
    total.local_structural_proposals += work.local_structural_proposals;
    total.physical_deallocations += work.physical_deallocations;
}

fn audit_rows(rows: &[Row]) {
    assert_eq!(rows.len(), SEEDS.len() * Scenario::ALL.len());
    let keys = rows
        .iter()
        .map(|row| (row.seed, row.scenario))
        .collect::<BTreeSet<_>>();
    assert_eq!(keys.len(), rows.len(), "seed/scenario rows must be unique");
    for (seed_index, seed) in SEEDS.into_iter().enumerate() {
        for (scenario_index, scenario) in Scenario::ALL.into_iter().enumerate() {
            let row = &rows[seed_index * Scenario::ALL.len() + scenario_index];
            assert_eq!((row.seed, row.scenario), (seed, scenario.name));
        }
    }
}

fn csv(rows: &[Row]) -> String {
    let mut text = String::from(
        "seed,scenario,couplings,scheduled,source_firings,raw_traversals,raw_impulse,outlet_firings,unit_participation,trace_firings,trace_ticks,opportunity_firings,candidate_traversals,consequence_firings,return_arrivals,proposals,live_after_train,resistance_after_train,coupling_after_train,live_after_gap,resistance_after_gap,coupling_after_gap,heldout_trained,heldout_crossed,heldout_gapped,heldout_singleton,work_total,queue_comparisons,spikes_delivered,state_updates,threshold_checks,firings,arrow_checks,spikes_emitted,eligibility_writes,return_updates,pressure_updates,structural_proposals,deallocations,persistent_bytes,complete_fingerprint,permanent_fingerprint,quiescent,replay_equal,passed\n",
    );
    for row in rows {
        text.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.seed,
            row.scenario,
            join_i32(&row.couplings),
            join_usize(&row.scheduled),
            join_usize(&row.source_firings),
            join_usize(&row.raw_traversals),
            join_i32(&row.raw_impulse),
            join_usize(&row.outlet_firings),
            join_usize(&row.unit_participation),
            join_usize(&row.trace_firings),
            row.trace_ticks.join(";"),
            join_usize(&row.opportunity_firings),
            join_usize(&row.candidate_traversals),
            join_usize(&row.consequence_firings),
            join_usize(&row.return_arrivals),
            row.proposals,
            join_bool(&row.live_after_train),
            join_u32(&row.resistance_after_train),
            join_i32(&row.coupling_after_train),
            join_bool(&row.live_after_gap),
            join_u32(&row.resistance_after_gap),
            join_i32(&row.coupling_after_gap),
            row.heldout_trained,
            row.heldout_crossed,
            row.heldout_gapped,
            row.heldout_singleton,
            row.work.total(),
            row.work.queue_comparisons,
            row.work.spikes_delivered,
            row.work.state_updates,
            row.work.threshold_checks,
            row.work.firings,
            row.work.arrow_checks,
            row.work.spikes_emitted,
            row.work.local_eligibility_writes,
            row.work.local_return_updates,
            row.work.ordinary_pressure_updates,
            row.work.local_structural_proposals,
            row.work.physical_deallocations,
            row.persistent_bytes,
            row.complete_fingerprint,
            row.permanent_fingerprint,
            row.quiescent,
            row.replay_equal,
            row.passed,
        ));
    }
    text
}

fn report(rows: &[Row]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let all_passed = passed == rows.len();
    let recurrent = rows
        .iter()
        .filter(|row| {
            matches!(
                row.scenario,
                "ab-recurrent-1-1"
                    | "ab-recurrent-2-1"
                    | "ab-recurrent-4-4"
                    | "ab-recurrent-heldout-matrix"
            )
        })
        .collect::<Vec<_>>();
    let controls = rows
        .iter()
        .filter(|row| {
            !matches!(
                row.scenario,
                "ab-recurrent-1-1"
                    | "ab-recurrent-2-1"
                    | "ab-recurrent-4-4"
                    | "ab-recurrent-heldout-matrix"
            )
        })
        .collect::<Vec<_>>();
    format!(
        "# PX3 participation-trace organization PROBE v1\n\nOutcome: **{}**.\n\n- rows passed: `{passed}/{}`;\n- exact replay: `{}`;\n- naturally quiescent: `{}`;\n- symmetric candidates proposed per row: `{}`;\n- recurrent AB rows with only AB live after gap: `{}/{}`;\n- recurrent trained held-out consequences: `{}`;\n- recurrent crossed/gapped/singleton consequences: `{}/{}/{}`;\n- non-recurrent/control reusable consequences: `{}`;\n- native work: `{}` operations;\n- authoritative PX0--PX2 changed: `false`;\n- new substrate law, contributor ID or typed Event added: `false`;\n- MICRO/GATE/definitive/authority executed: `false`.\n\n{}\n",
        if all_passed { "POSITIVE PROBE" } else { "NEGATIVE PROBE" },
        rows.len(),
        rows.iter().all(|row| row.replay_equal),
        rows.iter().all(|row| row.quiescent),
        rows.iter().map(|row| row.proposals).collect::<BTreeSet<_>>().into_iter().map(|value| value.to_string()).collect::<Vec<_>>().join("|"),
        recurrent.iter().filter(|row| row.live_after_gap == [true, false, false, false, false, false]).count(),
        recurrent.len(),
        recurrent.iter().map(|row| row.heldout_trained).sum::<usize>(),
        recurrent.iter().map(|row| row.heldout_crossed).sum::<usize>(),
        recurrent.iter().map(|row| row.heldout_gapped).sum::<usize>(),
        recurrent.iter().map(|row| row.heldout_singleton).sum::<usize>(),
        controls.iter().map(|row| row.heldout_trained).sum::<usize>(),
        rows.iter().map(|row| row.work.total()).sum::<u64>(),
        if all_passed {
            "Distinct overlapping PX1 trace firings selected one reusable anonymous physical route; amplitude, repetition, lateness, unsupported proposal and blocked return did not."
        } else {
            "At least one frozen discriminator failed; this PROBE does not support advancement."
        },
    )
}

fn join_i32(values: &[i32]) -> String {
    values.iter().map(i32::to_string).collect::<Vec<_>>().join("|")
}

fn join_u32(values: &[u32]) -> String {
    values.iter().map(u32::to_string).collect::<Vec<_>>().join("|")
}

fn join_usize(values: &[usize]) -> String {
    values.iter().map(usize::to_string).collect::<Vec<_>>().join("|")
}

fn join_bool(values: &[bool]) -> String {
    values.iter().map(bool::to_string).collect::<Vec<_>>().join("|")
}

fn require_absent(paths: &[&str]) {
    for path in paths {
        assert!(!Path::new(path).exists(), "artifact path must be absent: {path}");
    }
}

fn publish(staging: &str, destination: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(staging)
        .expect("create fresh staging artifact");
    file.write_all(contents.as_bytes()).expect("write artifact");
    file.sync_all().expect("sync artifact");
    rename(staging, destination).expect("publish artifact atomically");
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
        .expect("digest")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_matrix_is_exact_and_unique() {
        frozen_surface_audit();
        assert_eq!(SEEDS.len() * Scenario::ALL.len(), 24);
    }

    #[test]
    fn reservoir_contains_every_unordered_primitive_pair_once() {
        let pairs = PAIRS.into_iter().collect::<BTreeSet<_>>();
        assert_eq!(pairs.len(), 6);
        assert!(pairs.iter().all(|(left, right)| left < right));
    }

    #[test]
    fn only_frozen_recurrent_controls_expect_reuse() {
        assert_eq!(
            Scenario::ALL
                .iter()
                .filter(|scenario| scenario.recurrent())
                .count(),
            4
        );
    }
}
