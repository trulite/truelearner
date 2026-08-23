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
const PX3_NEGATIVE_CSV_SHA256: &str =
    "579dd5deebe9054cf385fe8a23b52b7641b516aa51c0a2c0f5bd864718339427";
const PX3_NEGATIVE_MD_SHA256: &str =
    "6c41f4433ea3d3a58f1befa52b966926f4c2f2df5b5d1eec48e889c2c86748d3";
const PX3_NEGATIVE_AUDIT_SHA256: &str =
    "001f24f480ff9267616433171179b787f4affeb8b3aed01bbf251b7dd51cb382";
const D1_PROTOCOL_SHA256: &str =
    "8da3c6ca5b5b548233662eacd20d6263f25c35411a48e392f4bd30403c08785f";
const EXECUTION_PROTOCOL_SHA256: &str =
    "2258cd30e8fcf04fb7ac9942f1beffeeb4110243623eeb88cb26cda02c5a78bf";

const SEEDS: [u64; 2] = [2901, 2909];
const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
const CSV_PATH: &str = "results/px3_d1_participation_gated_pair_learning_v1.csv";
const MD_PATH: &str = "results/px3_d1_participation_gated_pair_learning_v1.md";
const CSV_STAGE: &str = "results/.px3_d1_participation_gated_pair_learning_v1.csv.staging";
const MD_STAGE: &str = "results/.px3_d1_participation_gated_pair_learning_v1.md.staging";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    DormantBaseline,
    ReturnOnly,
    AAlone,
    A4Alone,
    ARepeated,
    ALateB,
    AbOneReturn,
    AbRecurrent11,
    AbRecurrent21,
    AbRecurrent44,
    AbNoReturn,
    AbRecurrentHeldout,
    D1rLateA,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scenario {
    name: &'static str,
    kind: Kind,
    couplings: [i32; 4],
}

impl Scenario {
    const ALL: [Self; 13] = [
        Self::new("dormant-baseline", Kind::DormantBaseline, [1; 4]),
        Self::new("return-only", Kind::ReturnOnly, [1; 4]),
        Self::new("a-alone", Kind::AAlone, [1; 4]),
        Self::new("a4-alone", Kind::A4Alone, [4, 1, 1, 1]),
        Self::new("a-repeated", Kind::ARepeated, [1; 4]),
        Self::new("a-then-b-late", Kind::ALateB, [1; 4]),
        Self::new("ab-one-return", Kind::AbOneReturn, [1; 4]),
        Self::new("ab-recurrent-1-1", Kind::AbRecurrent11, [1; 4]),
        Self::new("ab-recurrent-2-1", Kind::AbRecurrent21, [2, 1, 1, 1]),
        Self::new("ab-recurrent-4-4", Kind::AbRecurrent44, [4, 4, 1, 1]),
        Self::new("ab-no-return", Kind::AbNoReturn, [1; 4]),
        Self::new(
            "ab-recurrent-heldout-matrix",
            Kind::AbRecurrentHeldout,
            [1; 4],
        ),
        Self::new("d1r-ab-no-return-late-a", Kind::D1rLateA, [1; 4]),
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
                | Kind::AbRecurrentHeldout
        )
    }

    fn core_applicable(self) -> bool {
        self.kind != Kind::D1rLateA
    }
}

#[derive(Clone)]
struct World {
    namespace: u64,
    substrate: PlasticSubstrate,
    sources: [CellId; 4],
    context: CellId,
    px3_hub: CellId,
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
        self.quiescent &= execution.naturally_quiescent;
    }

    fn work(&mut self, work: WorkLedger) {
        add_work(&mut self.work, &work);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    scenario: &'static str,
    core_applicable: bool,
    core_pass: bool,
    d1r_applicable: bool,
    d1r_positive: bool,
    couplings: [i32; 4],
    scheduled: [usize; 4],
    source_firings: [usize; 4],
    raw_traversals: [usize; 4],
    raw_impulse: [i32; 4],
    outlet_firings: [usize; 4],
    trace_firings: [usize; 4],
    trace_ticks: [String; 4],
    opportunity_firings: [usize; 6],
    candidate_traversals: [usize; 6],
    candidate_impulse: [i32; 6],
    consequence_firings: [usize; 6],
    return_arrivals: [usize; 6],
    resistance_initial: [u32; 6],
    resistance_after_first: [u32; 6],
    resistance_before_second: [u32; 6],
    resistance_after_train: [u32; 6],
    live_after_train: [bool; 6],
    resistance_after_gap: [u32; 6],
    live_after_gap: [bool; 6],
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
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    source_audit();
    frozen_surface_audit();
    require_absent(&[CSV_PATH, MD_PATH, CSV_STAGE, MD_STAGE]);
    match args.as_slice() {
        [flag] if flag == "--preflight" => {
            println!("PX3_D1_PARTICIPATION_GATED_PAIR_LEARNING_PREFLIGHT_OK");
        }
        [flag] if flag == "--d1" => run_evidence(),
        _ => {
            eprintln!("PX3-D1 permits only --preflight or its frozen write-once --d1");
            std::process::exit(2);
        }
    }
}

fn run_evidence() {
    eprintln!("PX3_D1_PARTICIPATION_GATED_PAIR_LEARNING_EVIDENCE");
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
            "results/px3_participation_trace_organization_probe_v1.csv",
            PX3_NEGATIVE_CSV_SHA256,
        ),
        (
            "results/px3_participation_trace_organization_probe_v1.md",
            PX3_NEGATIVE_MD_SHA256,
        ),
        (
            "experiments/px3_participation_trace_organization_probe_result_audit_v1.md",
            PX3_NEGATIVE_AUDIT_SHA256,
        ),
        (
            "experiments/px3_d1_participation_gated_pair_learning_protocol_v1.md",
            D1_PROTOCOL_SHA256,
        ),
        (
            "experiments/px3_d1_participation_gated_pair_learning_execution_protocol_v1.md",
            EXECUTION_PROTOCOL_SHA256,
        ),
    ];
    for (path, expected) in frozen {
        assert_eq!(sha256(path), expected, "frozen input hash drift: {path}");
    }
}

fn frozen_surface_audit() {
    assert_eq!(SEEDS, [2901, 2909]);
    assert_eq!(Scenario::ALL.len(), 13);
    assert_eq!(PAIRS, [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    assert_eq!(
        Scenario::ALL
            .iter()
            .filter(|scenario| scenario.core_applicable())
            .count(),
        12
    );
    assert_eq!(
        Scenario::ALL
            .iter()
            .filter(|scenario| !scenario.core_applicable())
            .count(),
        1
    );
    let names = Scenario::ALL
        .iter()
        .map(|scenario| scenario.name)
        .collect::<BTreeSet<_>>();
    assert_eq!(names.len(), Scenario::ALL.len());
    for forbidden in [
        "arms/px3-d1-participation-gated-pair-learning/src/d2.rs",
        "arms/px3-d1-participation-gated-pair-learning/src/micro.rs",
        "arms/px3-d1-participation-gated-pair-learning/src/gate.rs",
        "results/px3_d2_recursive_normalization_v1.csv",
        "results/px3_micro_v1.csv",
        "results/px3_gate_v1.csv",
    ] {
        assert!(
            !Path::new(forbidden).exists(),
            "unauthorized later-stage surface exists: {forbidden}"
        );
    }
}

fn run_replay(seed: u64, scenario: Scenario) -> Row {
    let first = run(seed, scenario);
    let second = run(seed, scenario);
    let replay_equal = first == second;
    let mut row = first;
    row.replay_equal = replay_equal;
    if row.core_applicable {
        row.core_pass &= replay_equal;
    }
    if row.d1r_applicable {
        row.d1r_positive &= replay_equal;
    }
    row
}

fn run(seed: u64, scenario: Scenario) -> Row {
    let namespace = (seed << 32) | ((scenario_index(scenario) as u64 + 1) << 16);
    let return_enabled = !matches!(scenario.kind, Kind::AbNoReturn | Kind::D1rLateA);
    let mut world = build_world(namespace, scenario.couplings, return_enabled);
    let resistance_initial = candidate_resistance(&world);
    let mut log = Log {
        quiescent: true,
        ..Log::default()
    };
    let mut scheduled = [0; 4];
    let mut resistance_after_first = resistance_initial;
    let mut resistance_before_second = resistance_initial;

    match scenario.kind {
        Kind::DormantBaseline => {
            log.work(world.substrate.advance_time(6));
        }
        Kind::ReturnOnly => {
            enter_return_hub(&mut world, 3);
            log.execution(world.substrate.propagate());
        }
        Kind::AAlone | Kind::A4Alone => {
            expose(&mut world, 2, [true, false, false, false], true, false, &mut log);
            scheduled[0] = 1;
        }
        Kind::ARepeated => {
            expose(&mut world, 2, [true, false, false, false], true, true, &mut log);
            scheduled[0] = 2;
        }
        Kind::ALateB => {
            enter_source(&mut world, 0, 2, 0);
            enter_context(&mut world, 3);
            enter_source(&mut world, 1, 4, 1);
            log.execution(world.substrate.propagate());
            scheduled = [1, 1, 0, 0];
        }
        Kind::AbOneReturn | Kind::AbNoReturn => {
            expose(&mut world, 2, [true, true, false, false], true, false, &mut log);
            scheduled = [1, 1, 0, 0];
        }
        Kind::AbRecurrent11
        | Kind::AbRecurrent21
        | Kind::AbRecurrent44
        | Kind::AbRecurrentHeldout => {
            expose(&mut world, 2, [true, true, false, false], true, false, &mut log);
            resistance_after_first = candidate_resistance(&world);
            log.work(world.substrate.advance_time(11));
            resistance_before_second = candidate_resistance(&world);
            expose(&mut world, 11, [true, true, false, false], true, false, &mut log);
            scheduled = [2, 2, 0, 0];
        }
        Kind::D1rLateA => {
            enter_source(&mut world, 0, 2, 0);
            enter_source(&mut world, 1, 2, 1);
            enter_context(&mut world, 3);
            enter_source(&mut world, 0, 4, 2);
            log.execution(world.substrate.propagate());
            scheduled = [2, 1, 0, 0];
        }
    }
    if !scenario.recurrent() {
        resistance_after_first = candidate_resistance(&world);
        resistance_before_second = resistance_after_first;
    }

    let resistance_after_train = candidate_resistance(&world);
    let live_after_train = candidate_live(&world);
    let persistent_bytes = world.substrate.persistent_bytes();
    let complete_fingerprint = world.substrate.complete_fingerprint();
    let permanent_fingerprint = world.substrate.permanent_fingerprint();

    let mut gap_world = world.clone();
    log.work(gap_world.substrate.advance_time(50));
    let resistance_after_gap = candidate_resistance(&gap_world);
    let live_after_gap = candidate_live(&gap_world);

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
    let trace_firings = four(|side| firings(&log.trace, trace_physical(namespace, side)));
    let trace_ticks = four(|side| firing_ticks(&log.trace, trace_physical(namespace, side)));
    let opportunity_firings =
        six(|pair| firings(&log.trace, opportunity_physical(namespace, pair)));
    let candidate_traversals = six(|pair| {
        crossings(
            &log.crossings,
            opportunity_physical(namespace, pair),
            effect_physical(namespace, pair),
        )
    });
    let candidate_impulse = six(|pair| {
        crossing_impulse(
            &log.crossings,
            opportunity_physical(namespace, pair),
            effect_physical(namespace, pair),
        )
    });
    let consequence_firings = six(|pair| firings(&log.trace, effect_physical(namespace, pair)));
    let return_arrivals = six(|pair| {
        crossings(
            &log.crossings,
            px3_hub_physical(namespace),
            opportunity_physical(namespace, pair),
        )
    });

    let mut row = Row {
        seed,
        scenario: scenario.name,
        core_applicable: scenario.core_applicable(),
        core_pass: false,
        d1r_applicable: !scenario.core_applicable(),
        d1r_positive: false,
        couplings: scenario.couplings,
        scheduled,
        source_firings,
        raw_traversals,
        raw_impulse,
        outlet_firings,
        trace_firings,
        trace_ticks,
        opportunity_firings,
        candidate_traversals,
        candidate_impulse,
        consequence_firings,
        return_arrivals,
        resistance_initial,
        resistance_after_first,
        resistance_before_second,
        resistance_after_train,
        live_after_train,
        resistance_after_gap,
        live_after_gap,
        heldout_trained,
        heldout_crossed,
        heldout_gapped,
        heldout_singleton,
        work: log.work,
        persistent_bytes,
        complete_fingerprint,
        permanent_fingerprint,
        quiescent: log.quiescent,
        replay_equal: false,
    };
    row.core_pass = core_passes(&row, scenario);
    row.d1r_positive = d1r_positive(&row, scenario);
    row
}

fn core_passes(row: &Row, scenario: Scenario) -> bool {
    if !scenario.core_applicable() {
        return false;
    }
    let (scheduled, actual, opportunities, candidates, candidate_impulse, consequences, returns) =
        expectations(scenario.kind);
    let raw_impulse = four(|side| {
        i32::try_from(actual[side]).expect("small count") * scenario.couplings[side]
    });
    let initial = [1; 6];
    let dormant = [1; 6];
    let dead = [0; 6];
    let (after_first, before_second, after_train, live_train, after_gap, live_gap) =
        if scenario.recurrent() {
            (
                [4, 1, 1, 1, 1, 1],
                [3, 0, 0, 0, 0, 0],
                [6, 0, 0, 0, 0, 0],
                [true, false, false, false, false, false],
                [2, 0, 0, 0, 0, 0],
                [true, false, false, false, false, false],
            )
        } else if scenario.kind == Kind::AbOneReturn {
            (
                [4, 1, 1, 1, 1, 1],
                [4, 1, 1, 1, 1, 1],
                [4, 1, 1, 1, 1, 1],
                [true; 6],
                dead,
                [false; 6],
            )
        } else {
            (dormant, dormant, dormant, [true; 6], dead, [false; 6])
        };
    let expected_heldout = usize::from(scenario.recurrent());

    row.resistance_initial == initial
        && row.scheduled == scheduled
        && row.source_firings == actual
        && row.raw_traversals == actual
        && row.raw_impulse == raw_impulse
        && row.outlet_firings == actual
        && row.trace_firings == actual
        && row.opportunity_firings == opportunities
        && row.candidate_traversals == candidates
        && row.candidate_impulse == candidate_impulse
        && row.consequence_firings == consequences
        && row.return_arrivals == returns
        && row.resistance_after_first == after_first
        && row.resistance_before_second == before_second
        && row.resistance_after_train == after_train
        && row.live_after_train == live_train
        && row.resistance_after_gap == after_gap
        && row.live_after_gap == live_gap
        && row.heldout_trained == expected_heldout
        && row.heldout_crossed == 0
        && row.heldout_gapped == 0
        && row.heldout_singleton == 0
        && row.quiescent
}

#[allow(clippy::type_complexity)]
fn expectations(
    kind: Kind,
) -> (
    [usize; 4],
    [usize; 4],
    [usize; 6],
    [usize; 6],
    [i32; 6],
    [usize; 6],
    [usize; 6],
) {
    match kind {
        Kind::DormantBaseline => ([0; 4], [0; 4], [0; 6], [0; 6], [0; 6], [0; 6], [0; 6]),
        Kind::ReturnOnly => ([0; 4], [0; 4], [0; 6], [0; 6], [0; 6], [0; 6], [1; 6]),
        Kind::AAlone | Kind::A4Alone => (
            [1, 0, 0, 0], [1, 0, 0, 0], [0; 6], [0; 6], [0; 6], [0; 6], [0; 6],
        ),
        Kind::ARepeated => (
            [2, 0, 0, 0], [1, 0, 0, 0], [0; 6], [0; 6], [0; 6], [0; 6], [0; 6],
        ),
        Kind::ALateB => (
            [1, 1, 0, 0], [1, 1, 0, 0], [0; 6], [0; 6], [0; 6], [0; 6], [0; 6],
        ),
        Kind::AbOneReturn => (
            [1, 1, 0, 0],
            [1, 1, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1; 6],
        ),
        Kind::AbRecurrent11
        | Kind::AbRecurrent21
        | Kind::AbRecurrent44
        | Kind::AbRecurrentHeldout => (
            [2, 2, 0, 0],
            [2, 2, 0, 0],
            [2, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0],
            [3, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0],
            [2; 6],
        ),
        Kind::AbNoReturn => (
            [1, 1, 0, 0],
            [1, 1, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [0; 6],
        ),
        Kind::D1rLateA => unreachable!("D1-R has an independent verdict"),
    }
}

fn d1r_positive(row: &Row, scenario: Scenario) -> bool {
    if scenario.kind != Kind::D1rLateA {
        return false;
    }
    row.scheduled == [2, 1, 0, 0]
        && row.source_firings == [2, 1, 0, 0]
        && row.raw_traversals == [2, 1, 0, 0]
        && row.trace_firings == [2, 1, 0, 0]
        && row.opportunity_firings == [1, 0, 0, 0, 0, 0]
        && row.candidate_traversals == [1, 0, 0, 0, 0, 0]
        && row.candidate_impulse == [1, 0, 0, 0, 0, 0]
        && row.consequence_firings == [1, 0, 0, 0, 0, 0]
        && row.return_arrivals == [0; 6]
        && row.resistance_initial == [1; 6]
        && row.resistance_after_train == [1; 6]
        && row.resistance_after_gap == [0; 6]
        && row.heldout_trained == 0
        && row.quiescent
}

fn build_world(namespace: u64, couplings: [i32; 4], return_enabled: bool) -> World {
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
            20_000 + pair as i32 * 100,
            60 + pair as i16,
            2,
        ))
    });
    let context = substrate.add_cell(cell(context_physical(namespace), 30_000, 70, 1));
    let px3_hub = substrate.add_cell(cell(px3_hub_physical(namespace), 40_000, 80, 1));

    for side in 0..4 {
        substrate.add_arrow(fixed_arrow(sources[side], outlets[side], 0, couplings[side]));
        substrate.add_arrow(fixed_arrow(outlets[side], traces[side], 1, 1));
        substrate.add_arrow(fixed_arrow(outlets[side], px1_hub, 1, 1));
        substrate.add_arrow(fixed_arrow(px1_hub, traces[side], 0, 1));
    }
    let candidates = six(|pair| {
        let (left, right) = PAIRS[pair];
        substrate.add_arrow(fixed_arrow(traces[left], opportunities[pair], 0, 1));
        substrate.add_arrow(fixed_arrow(traces[right], opportunities[pair], 0, 1));
        substrate.add_arrow(fixed_arrow(context, effects[pair], 2, 1));
        if return_enabled {
            substrate.add_arrow(fixed_arrow(effects[pair], px3_hub, 0, 1));
        }
        substrate.add_arrow(fixed_arrow(px3_hub, opportunities[pair], 1, 1));
        substrate.add_arrow(weak_candidate(opportunities[pair], effects[pair]))
    });
    assert_eq!(substrate.arrow_count(), fixed_arrow_count(return_enabled));
    for candidate in candidates {
        assert!(substrate.arrow_is_live(candidate));
        assert_eq!(substrate.arrow_resistance(candidate), 1);
    }

    World {
        namespace,
        substrate,
        sources,
        context,
        px3_hub,
        candidates,
    }
}

fn fixed_arrow_count(return_enabled: bool) -> usize {
    4 * 4 + 6 * (2 + 1 + usize::from(return_enabled) + 1 + 1)
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

fn enter_return_hub(world: &mut World, tick: i64) {
    world.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: 700,
        origin_physical: 3_000_000,
        target: world.px3_hub,
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

fn candidate_resistance(world: &World) -> [u32; 6] {
    six(|pair| world.substrate.arrow_resistance(world.candidates[pair]))
}

fn candidate_live(world: &World) -> [bool; 6] {
    six(|pair| world.substrate.arrow_is_live(world.candidates[pair]))
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

fn fixed_arrow(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 100,
    }
}

fn weak_candidate(from: CellId, to: CellId) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay: 2,
        phase: 0,
        coupling: 1,
        resistance: 1,
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
    assert_eq!(keys.len(), rows.len());
    for (seed_index, seed) in SEEDS.into_iter().enumerate() {
        for (scenario_index, scenario) in Scenario::ALL.into_iter().enumerate() {
            let row = &rows[seed_index * Scenario::ALL.len() + scenario_index];
            assert_eq!((row.seed, row.scenario), (seed, scenario.name));
            assert_ne!(row.core_applicable, row.d1r_applicable);
        }
    }
    assert_eq!(rows.iter().filter(|row| row.core_applicable).count(), 24);
    assert_eq!(rows.iter().filter(|row| row.d1r_applicable).count(), 2);
}

fn csv(rows: &[Row]) -> String {
    let mut text = String::from(
        "seed,scenario,core_applicable,core_pass,d1r_applicable,d1r_positive,couplings,scheduled,source_firings,raw_traversals,raw_impulse,outlet_firings,trace_firings,trace_ticks,opportunity_firings,candidate_traversals,candidate_impulse,consequence_firings,return_arrivals,resistance_initial,resistance_after_first,resistance_before_second,resistance_after_train,live_after_train,resistance_after_gap,live_after_gap,heldout_trained,heldout_crossed,heldout_gapped,heldout_singleton,work_total,queue_comparisons,spikes_delivered,state_updates,threshold_checks,firings,arrow_checks,spikes_emitted,eligibility_writes,return_updates,pressure_updates,structural_proposals,deallocations,persistent_bytes,complete_fingerprint,permanent_fingerprint,quiescent,replay_equal\n",
    );
    for row in rows {
        let fields = vec![
            row.seed.to_string(),
            row.scenario.to_string(),
            row.core_applicable.to_string(),
            row.core_pass.to_string(),
            row.d1r_applicable.to_string(),
            row.d1r_positive.to_string(),
            join_i32(&row.couplings),
            join_usize(&row.scheduled),
            join_usize(&row.source_firings),
            join_usize(&row.raw_traversals),
            join_i32(&row.raw_impulse),
            join_usize(&row.outlet_firings),
            join_usize(&row.trace_firings),
            row.trace_ticks.join(";"),
            join_usize(&row.opportunity_firings),
            join_usize(&row.candidate_traversals),
            join_i32(&row.candidate_impulse),
            join_usize(&row.consequence_firings),
            join_usize(&row.return_arrivals),
            join_u32(&row.resistance_initial),
            join_u32(&row.resistance_after_first),
            join_u32(&row.resistance_before_second),
            join_u32(&row.resistance_after_train),
            join_bool(&row.live_after_train),
            join_u32(&row.resistance_after_gap),
            join_bool(&row.live_after_gap),
            row.heldout_trained.to_string(),
            row.heldout_crossed.to_string(),
            row.heldout_gapped.to_string(),
            row.heldout_singleton.to_string(),
            row.work.total().to_string(),
            row.work.queue_comparisons.to_string(),
            row.work.spikes_delivered.to_string(),
            row.work.state_updates.to_string(),
            row.work.threshold_checks.to_string(),
            row.work.firings.to_string(),
            row.work.arrow_checks.to_string(),
            row.work.spikes_emitted.to_string(),
            row.work.local_eligibility_writes.to_string(),
            row.work.local_return_updates.to_string(),
            row.work.ordinary_pressure_updates.to_string(),
            row.work.local_structural_proposals.to_string(),
            row.work.physical_deallocations.to_string(),
            row.persistent_bytes.to_string(),
            row.complete_fingerprint.to_string(),
            row.permanent_fingerprint.to_string(),
            row.quiescent.to_string(),
            row.replay_equal.to_string(),
        ];
        text.push_str(&fields.join(","));
        text.push('\n');
    }
    text
}

fn report(rows: &[Row]) -> String {
    let core = rows
        .iter()
        .filter(|row| row.core_applicable)
        .collect::<Vec<_>>();
    let provenance = rows
        .iter()
        .filter(|row| row.d1r_applicable)
        .collect::<Vec<_>>();
    let core_positive = core.iter().all(|row| row.core_pass);
    let d1r_positive = provenance.iter().all(|row| row.d1r_positive);
    let one = rows
        .iter()
        .filter(|row| row.scenario == "ab-one-return")
        .collect::<Vec<_>>();
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
    format!(
        "# PX3-D1 participation-gated pair learning v1\n\n- D1 core: **{}** (`{}/{}` applicable rows passed);\n- D1-R provenance: **{}** (`{}/{}` provenance rows positive);\n- exact replay: `{}`;\n- naturally quiescent: `{}`;\n- one-exposure AB post-return resistance: `{}`;\n- one-exposure AB post-gap resistance: `{}`;\n- recurrent AB first/before-second/after-second/post-gap resistance: `{}`;\n- recurrent trained held-out consequences: `{}`;\n- recurrent crossed/gapped/singleton consequences: `{}/{}/{}`;\n- structural proposals: `{}`;\n- native work: `{}` operations;\n- candidate formation/reproposal/D2/MICRO/GATE executed: `false`.\n\n{}\n",
        if core_positive { "D1-A POSITIVE" } else { "NEGATIVE" },
        core.iter().filter(|row| row.core_pass).count(),
        core.len(),
        if d1r_positive { "D1-R+" } else { "D1-R-" },
        provenance.iter().filter(|row| row.d1r_positive).count(),
        provenance.len(),
        rows.iter().all(|row| row.replay_equal),
        rows.iter().all(|row| row.quiescent),
        one.iter().map(|row| row.resistance_after_train[0].to_string()).collect::<Vec<_>>().join("|"),
        one.iter().map(|row| row.resistance_after_gap[0].to_string()).collect::<Vec<_>>().join("|"),
        recurrent.iter().map(|row| format!("{}>{}>{}>{}", row.resistance_after_first[0], row.resistance_before_second[0], row.resistance_after_train[0], row.resistance_after_gap[0])).collect::<Vec<_>>().join("|"),
        recurrent.iter().map(|row| row.heldout_trained).sum::<usize>(),
        recurrent.iter().map(|row| row.heldout_crossed).sum::<usize>(),
        recurrent.iter().map(|row| row.heldout_gapped).sum::<usize>(),
        recurrent.iter().map(|row| row.heldout_singleton).sum::<usize>(),
        rows.iter().map(|row| row.work.local_structural_proposals).sum::<u64>(),
        rows.iter().map(|row| row.work.total()).sum::<u64>(),
        match (core_positive, d1r_positive) {
            (true, true) => "Traversal gates learning and the frozen late-input case does not masquerade as return.",
            (true, false) => "Traversal gates learning, but eligible structure accepts the frozen late upstream input as evidence: participation attribution is solved while return provenance remains unsolved.",
            (false, _) => "The narrow participation-gating hypothesis failed under at least one frozen core control.",
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
    fn frozen_matrix_and_verdict_partition_are_exact() {
        frozen_surface_audit();
        assert_eq!(SEEDS.len() * Scenario::ALL.len(), 26);
    }

    #[test]
    fn reservoir_contains_each_unordered_pair_once() {
        let pairs = PAIRS.into_iter().collect::<BTreeSet<_>>();
        assert_eq!(pairs.len(), 6);
        assert!(pairs.iter().all(|(left, right)| left < right));
    }

    #[test]
    fn amplitude_and_recurrence_partition_is_frozen() {
        assert_eq!(
            Scenario::ALL
                .iter()
                .filter(|scenario| scenario.recurrent())
                .count(),
            4
        );
        assert_eq!(Scenario::ALL[8].couplings, [2, 1, 1, 1]);
        assert_eq!(Scenario::ALL[9].couplings, [4, 4, 1, 1]);
    }
}
