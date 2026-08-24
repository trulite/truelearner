#![forbid(unsafe_code)]

use lr1_modulatory_physical_return::{
    ArrowId, ArrowSpec, CellId, CellSpec, Crossing, Execution, PlasticSubstrate, SpikeInput,
    TraceEntry, TransmissionMode, WorkLedger,
};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const LAW: &str = "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10";
const AUTHORITY: &str = "3ad1df774690a71ee7e6884f56a9399a098890e14d83c7a2f03231ed9aafeb3c";
const DEVELOPMENT_PROTOCOL: &str =
    "c249be1d74ad219896f2fe3505942166efb369e9bea4e50dfa84585f2a1d7107";
const DEVELOPMENT_AUDIT: &str = "44ce9f414f785a434752ccaab6a7445e430a43be5cac576d7cdec1441d85fd79";
const LIFECYCLE_DEVELOPMENT: &str =
    "0125f52fc9e5427c558caa39b8f720fe1fefea13e16f8c4e88bd0ae46904afef";
const RECURSION_DEVELOPMENT: &str =
    "b59fa4b299d2ec22429255d78269ecb7fa56c22aeb4c122137fc21a299369724";
const PROTOCOL: &str = "06a9ea4515b5ea42bf576a5bd49969c966cc63bba94ef3f4b499fb89da8345cc";
const SEEDS: [u64; 16] = [
    91001, 91002, 91003, 91004, 91005, 91006, 91007, 91008, 91009, 91010, 91011, 91012, 91013,
    91014, 91015, 91016,
];
const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
const INITIAL: [usize; 2] = [0, 5];
const REVERSED: [usize; 2] = [2, 3];
const CSV: &str = "results/px3_lrc_lifecycle_definitive_v1.csv";
const MD: &str = "results/px3_lrc_lifecycle_definitive_v1.md";
const CSV_STAGE: &str = "results/.px3_lrc_lifecycle_definitive_v1.csv.staging";
const MD_STAGE: &str = "results/.px3_lrc_lifecycle_definitive_v1.md.staging";

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    namespace: u64,
    sources: [CellId; 4],
    candidate_sources: [CellId; 6],
    effects: [CellId; 6],
    context: CellId,
    global_return: CellId,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Log {
    trace: Vec<TraceEntry>,
    crossings: Vec<Crossing>,
    work: WorkLedger,
    quiescent: bool,
}

impl Log {
    fn new() -> Self {
        Self {
            quiescent: true,
            ..Self::default()
        }
    }

    fn execution(&mut self, execution: Execution) {
        self.trace.extend(execution.trace);
        self.crossings.extend(execution.crossings);
        add_work(&mut self.work, &execution.work);
        self.quiescent &= execution.naturally_quiescent;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Heldout {
    effects: [usize; 6],
    proposals: [u64; 6],
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ControlMetrics {
    source_firings: [usize; 4],
    raw_crossings: [usize; 4],
    raw_impulse: [i32; 4],
    participant_traces: [usize; 4],
    opportunities: usize,
    candidate_sources: usize,
    candidate_crossings: usize,
    proposals: u64,
    updates: u64,
    live_after_gap: usize,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ModulationControl {
    effect_path: usize,
    return_relay: usize,
    transmitter: usize,
    modulatory_crossings: usize,
    candidate_source_firings: usize,
    proposals: u64,
    updates: u64,
    candidate_count: usize,
    quiescent: bool,
}

#[derive(Clone, Copy)]
enum ControlKind {
    StrongA,
    RepeatedA,
    GappedAb,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    seed: u64,
    namespace: u64,
    reverse: bool,
    reflect: bool,
    strong_a: ControlMetrics,
    repeated_a: ControlMetrics,
    gapped_ab: ControlMetrics,
    modulation_without_participation: ModulationControl,
    unsupported_updates: u64,
    unsupported_candidate_crossings: usize,
    unsupported_after_gap: u32,
    initial_candidate_count: usize,
    initial_proposals: u64,
    initial_primitive_trace: [usize; 4],
    initial_opportunity: [usize; 6],
    initial_p: [usize; 6],
    initial_candidate_crossings: [usize; 6],
    initial_candidate_impulse: [i32; 6],
    initial_effect: [usize; 6],
    initial_p_trace: [usize; 6],
    initial_effect_trace: [usize; 6],
    initial_attribution: [usize; 6],
    initial_credit: [usize; 6],
    initial_after_first: [u32; 6],
    initial_one_exposure_gap: [u32; 6],
    initial_after_train: [u32; 6],
    initial_after_gap: [u32; 6],
    pre_reversal_heldout: [usize; 6],
    pre_reversal_heldout_proposals: [u64; 6],
    old_ids: [String; 2],
    old_generations: [u32; 2],
    old_after_forgetting: [u32; 2],
    old_live_after_forgetting: [bool; 2],
    proposals_during_forgetting: u64,
    reversed_proposals: u64,
    reversed_primitive_trace: [usize; 4],
    reversed_opportunity: [usize; 6],
    reversed_p: [usize; 6],
    reversed_candidate_crossings: [usize; 6],
    reversed_candidate_impulse: [i32; 6],
    reversed_effect: [usize; 6],
    reversed_p_trace: [usize; 6],
    reversed_effect_trace: [usize; 6],
    reversed_attribution: [usize; 6],
    reversed_credit: [usize; 6],
    reversed_after_first: [u32; 6],
    reversed_one_exposure_gap: [u32; 6],
    reversed_after_train: [u32; 6],
    reversed_after_gap: [u32; 6],
    new_ids: [String; 2],
    new_generations: [u32; 2],
    fresh_identity: bool,
    old_still_dead: [bool; 2],
    historical_counts: [usize; 6],
    final_live_counts: [usize; 6],
    post_reversal_heldout: [usize; 6],
    post_reversal_heldout_proposals: [u64; 6],
    total_proposals: u64,
    deallocations: u64,
    return_updates: u64,
    work: u64,
    bytes: usize,
    fingerprint: u64,
    permanent: u64,
    quiescent: bool,
    claims: [bool; 12],
    replay: bool,
    passed: bool,
}

fn main() {
    audit();
    surface();
    absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [argument] if argument == "--preflight" => {
            println!("PX3_LRC_LIFECYCLE_PREFLIGHT_OK");
        }
        [argument] if argument == "--definitive" => evidence(),
        _ => std::process::exit(2),
    }
}

fn evidence() {
    eprintln!("PX3_LRC_LIFECYCLE_DEFINITIVE_EVIDENCE_SPENT");
    let rows = SEEDS.into_iter().map(replay).collect::<Vec<_>>();
    assert_eq!(rows.len(), 16);
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(MD_STAGE, MD, &report(&rows));
}

fn audit() {
    for (path, expected) in [
        ("crates/lr1-modulatory-physical-return/src/lib.rs", LAW),
        (
            "results/lrc_qualified_modulatory_transmission_definitive_v2.md",
            AUTHORITY,
        ),
        (
            "experiments/px3_lrc_fresh_integrated_parallel_gates_protocol_v2.md",
            DEVELOPMENT_PROTOCOL,
        ),
        (
            "experiments/px3_lrc_fresh_integrated_parallel_gates_v2_result_audit.md",
            DEVELOPMENT_AUDIT,
        ),
        ("results/px3_lrc_lifecycle_v2.csv", LIFECYCLE_DEVELOPMENT),
        ("results/px3_lrc_recursion_v2.csv", RECURSION_DEVELOPMENT),
        (
            "experiments/px3_lrc_physical_event_organization_definitive_protocol_v1.md",
            PROTOCOL,
        ),
    ] {
        assert_eq!(sha(path), expected, "frozen input changed: {path}");
    }
}

fn surface() {
    assert_eq!(SEEDS.into_iter().collect::<BTreeSet<_>>().len(), 16);
    assert_eq!(SEEDS.iter().filter(|seed| **seed % 4 == 0).count(), 4);
    assert_eq!(SEEDS.iter().filter(|seed| **seed % 4 == 1).count(), 4);
    assert_eq!(SEEDS.iter().filter(|seed| **seed % 4 == 2).count(), 4);
    assert_eq!(SEEDS.iter().filter(|seed| **seed % 4 == 3).count(), 4);
    assert_eq!(PAIRS.into_iter().collect::<BTreeSet<_>>().len(), 6);
    assert_eq!(INITIAL, [0, 5]);
    assert_eq!(REVERSED, [2, 3]);
    for forbidden in [
        "arms/px3-integrated-micro-reversal/src/gate.rs",
        "results/px3_recursive_gate_v1.csv",
    ] {
        assert!(!Path::new(forbidden).exists());
    }
}

fn replay(seed: u64) -> Row {
    let first = run(seed);
    let second = run(seed);
    let exact = first == second;
    let mut row = first;
    row.replay = exact;
    row.claims[11] &= exact;
    row.passed = row.claims.into_iter().all(|claim| claim);
    row
}

fn run(seed: u64) -> Row {
    let stratum = seed % 4;
    let reverse = stratum == 1 || stratum == 3;
    let reflect = stratum >= 2;
    let namespace = seed << 32;
    let strong_a = control(
        namespace + 0x0100_0000,
        reverse,
        reflect,
        ControlKind::StrongA,
    );
    let repeated_a = control(
        namespace + 0x0200_0000,
        reverse,
        reflect,
        ControlKind::RepeatedA,
    );
    let gapped_ab = control(
        namespace + 0x0300_0000,
        reverse,
        reflect,
        ControlKind::GappedAb,
    );
    let modulation_without_participation =
        modulation_without_participation(namespace + 0x0400_0000, reverse, reflect);
    let mut unsupported = build(namespace + 0x1000_0000, reverse, reflect, [1; 4]);
    let mut unsupported_log = Log::new();
    expose_unsupported(&mut unsupported, 0, 0, &mut unsupported_log);
    expose_unsupported(&mut unsupported, 0, 2, &mut unsupported_log);
    let unsupported_updates = unsupported_log.work.local_return_updates;
    let unsupported_candidate_crossings =
        candidate_crossings(&unsupported_log, unsupported.namespace, 0);
    unsupported.substrate.advance_time(50);
    let unsupported_after_gap = current_resistance(&unsupported)[0];
    let mut world = build(namespace, reverse, reflect, [1; 4]);
    let initial_candidate_count = variable_count(&world);
    let mut full_work = WorkLedger::default();
    let mut all_quiescent = true;

    let mut initial_log = Log::new();
    expose(&mut world, 0, 0, &mut initial_log);
    expose(&mut world, 5, 4, &mut initial_log);
    let initial_after_first = current_resistance(&world);
    let mut initial_once = world.clone();
    add_work(&mut full_work, &initial_once.substrate.advance_time(50));
    let initial_one_exposure_gap = current_resistance(&initial_once);
    expose(&mut world, 0, 11, &mut initial_log);
    expose(&mut world, 5, 15, &mut initial_log);
    let initial_after_train = current_resistance(&world);
    let initial_candidates = [only_historical(&world, 0), only_historical(&world, 5)];
    let old_ids = initial_candidates.map(|arrow| format!("{arrow:?}"));
    let old_generations = initial_candidates.map(|arrow| world.substrate.arrow_generation(arrow));
    let initial_proposals = initial_log.work.local_structural_proposals;
    add_work(&mut full_work, &initial_log.work);
    all_quiescent &= initial_log.quiescent;

    let gap_work = world.substrate.advance_time(50);
    add_work(&mut full_work, &gap_work);
    let initial_after_gap = current_resistance(&world);
    let pre_reversal = heldout(&world, 50);
    all_quiescent &= pre_reversal.quiescent;

    let arrow_count_before_forgetting = world.substrate.arrow_count();
    let forgetting_work = world.substrate.advance_time(70);
    let proposals_during_forgetting = forgetting_work.local_structural_proposals;
    add_work(&mut full_work, &forgetting_work);
    let old_after_forgetting =
        initial_candidates.map(|arrow| world.substrate.arrow_resistance(arrow));
    let old_live_after_forgetting =
        initial_candidates.map(|arrow| world.substrate.arrow_is_live(arrow));
    assert_eq!(world.substrate.arrow_count(), arrow_count_before_forgetting);

    let mut reversed_log = Log::new();
    expose(&mut world, 2, 70, &mut reversed_log);
    expose(&mut world, 3, 74, &mut reversed_log);
    let reversed_after_first = current_resistance(&world);
    let mut reversed_once = world.clone();
    add_work(&mut full_work, &reversed_once.substrate.advance_time(120));
    let reversed_one_exposure_gap = current_resistance(&reversed_once);
    expose(&mut world, 2, 81, &mut reversed_log);
    expose(&mut world, 3, 85, &mut reversed_log);
    let reversed_after_train = current_resistance(&world);
    let new_candidates = [only_live(&world, 2), only_live(&world, 3)];
    let new_ids = new_candidates.map(|arrow| format!("{arrow:?}"));
    let new_generations = new_candidates.map(|arrow| world.substrate.arrow_generation(arrow));
    let fresh_identity = new_candidates.iter().all(|new_arrow| {
        initial_candidates
            .iter()
            .all(|old_arrow| old_arrow != new_arrow)
    }) && new_candidates[0] != new_candidates[1];
    let reversed_proposals = reversed_log.work.local_structural_proposals;
    add_work(&mut full_work, &reversed_log.work);
    all_quiescent &= reversed_log.quiescent;

    let final_gap_work = world.substrate.advance_time(120);
    add_work(&mut full_work, &final_gap_work);
    let reversed_after_gap = current_resistance(&world);
    let old_still_dead = initial_candidates.map(|arrow| !world.substrate.arrow_is_live(arrow));
    let historical_counts = six(|pair| variable_arrows(&world, pair).len());
    let final_live_counts = six(|pair| live_arrows(&world, pair).len());
    let post_reversal = heldout(&world, 120);
    all_quiescent &= post_reversal.quiescent;

    let initial_primitive_trace =
        four(|side| fires(&initial_log.trace, physical(namespace, 30 + side as u64)));
    let initial_opportunity =
        six(|pair| fires(&initial_log.trace, physical(namespace, 100 + pair as u64)));
    let initial_p = six(|pair| fires(&initial_log.trace, physical(namespace, 200 + pair as u64)));
    let initial_candidate_crossings =
        six(|pair| candidate_crossings(&initial_log, namespace, pair));
    let initial_candidate_impulse = six(|pair| candidate_impulse(&initial_log, namespace, pair));
    let initial_effect =
        six(|pair| fires(&initial_log.trace, physical(namespace, 300 + pair as u64)));
    let initial_p_trace =
        six(|pair| fires(&initial_log.trace, physical(namespace, 400 + pair as u64)));
    let initial_effect_trace =
        six(|pair| fires(&initial_log.trace, physical(namespace, 600 + pair as u64)));
    let initial_attribution =
        six(|pair| fires(&initial_log.trace, physical(namespace, 800 + pair as u64)));
    let initial_credit = six(|pair| credit_crossings(&initial_log, namespace, pair));

    let reversed_primitive_trace =
        four(|side| fires(&reversed_log.trace, physical(namespace, 30 + side as u64)));
    let reversed_opportunity =
        six(|pair| fires(&reversed_log.trace, physical(namespace, 100 + pair as u64)));
    let reversed_p = six(|pair| fires(&reversed_log.trace, physical(namespace, 200 + pair as u64)));
    let reversed_candidate_crossings =
        six(|pair| candidate_crossings(&reversed_log, namespace, pair));
    let reversed_candidate_impulse = six(|pair| candidate_impulse(&reversed_log, namespace, pair));
    let reversed_effect =
        six(|pair| fires(&reversed_log.trace, physical(namespace, 300 + pair as u64)));
    let reversed_p_trace =
        six(|pair| fires(&reversed_log.trace, physical(namespace, 400 + pair as u64)));
    let reversed_effect_trace =
        six(|pair| fires(&reversed_log.trace, physical(namespace, 600 + pair as u64)));
    let reversed_attribution =
        six(|pair| fires(&reversed_log.trace, physical(namespace, 800 + pair as u64)));
    let reversed_credit = six(|pair| credit_crossings(&reversed_log, namespace, pair));

    let total_proposals = initial_proposals + reversed_proposals + proposals_during_forgetting;
    let mut row = Row {
        seed,
        namespace,
        reverse,
        reflect,
        strong_a,
        repeated_a,
        gapped_ab,
        modulation_without_participation,
        unsupported_updates,
        unsupported_candidate_crossings,
        unsupported_after_gap,
        initial_candidate_count,
        initial_proposals,
        initial_primitive_trace,
        initial_opportunity,
        initial_p,
        initial_candidate_crossings,
        initial_candidate_impulse,
        initial_effect,
        initial_p_trace,
        initial_effect_trace,
        initial_attribution,
        initial_credit,
        initial_after_first,
        initial_one_exposure_gap,
        initial_after_train,
        initial_after_gap,
        pre_reversal_heldout: pre_reversal.effects,
        pre_reversal_heldout_proposals: pre_reversal.proposals,
        old_ids,
        old_generations,
        old_after_forgetting,
        old_live_after_forgetting,
        proposals_during_forgetting,
        reversed_proposals,
        reversed_primitive_trace,
        reversed_opportunity,
        reversed_p,
        reversed_candidate_crossings,
        reversed_candidate_impulse,
        reversed_effect,
        reversed_p_trace,
        reversed_effect_trace,
        reversed_attribution,
        reversed_credit,
        reversed_after_first,
        reversed_one_exposure_gap,
        reversed_after_train,
        reversed_after_gap,
        new_ids,
        new_generations,
        fresh_identity,
        old_still_dead,
        historical_counts,
        final_live_counts,
        post_reversal_heldout: post_reversal.effects,
        post_reversal_heldout_proposals: post_reversal.proposals,
        total_proposals,
        deallocations: full_work.physical_deallocations,
        return_updates: full_work.local_return_updates,
        work: full_work.total(),
        bytes: world.substrate.persistent_bytes(),
        fingerprint: world.substrate.complete_fingerprint(),
        permanent: world.substrate.permanent_fingerprint(),
        quiescent: all_quiescent,
        claims: [false; 12],
        replay: false,
        passed: false,
    };
    row.claims = claims(&row);
    row.passed = row.claims.into_iter().all(|claim| claim);
    row
}

fn claims(row: &Row) -> [bool; 12] {
    let initial = [2, 0, 0, 0, 0, 2];
    let initial_impulse = [3, 0, 0, 0, 0, 3];
    let initial_four = [4, 0, 0, 0, 0, 4];
    let initial_six = [6, 0, 0, 0, 0, 6];
    let initial_two = [2, 0, 0, 0, 0, 2];
    let reversed = [0, 0, 2, 2, 0, 0];
    let reversed_impulse = [0, 0, 3, 3, 0, 0];
    let reversed_four = [0, 0, 4, 4, 0, 0];
    let reversed_six = [0, 0, 6, 6, 0, 0];
    let reversed_two = [0, 0, 2, 2, 0, 0];
    let z6 = [0; 6];

    let l0 = SEEDS.contains(&row.seed)
        && row.namespace == row.seed << 32
        && row.reverse == matches!(row.seed % 4, 1 | 3)
        && row.reflect == (row.seed % 4 >= 2)
        && row.initial_candidate_count == 0;
    let l1 = control_passes(
        &row.strong_a,
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [4, 0, 0, 0],
        [1, 0, 0, 0],
    );
    let l2 = control_passes(
        &row.repeated_a,
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
    );
    let l3 = control_passes(
        &row.gapped_ab,
        [1, 1, 0, 0],
        [1, 1, 0, 0],
        [1, 1, 0, 0],
        [1, 1, 0, 0],
    );
    let l4 = row.unsupported_updates == 0
        && row.unsupported_candidate_crossings == 2
        && row.unsupported_after_gap == 0;
    let modulation = &row.modulation_without_participation;
    let l5 = modulation.effect_path == 1
        && modulation.return_relay == 1
        && modulation.transmitter == 1
        && modulation.modulatory_crossings == 1
        && modulation.candidate_source_firings == 0
        && modulation.proposals == 0
        && modulation.updates == 0
        && modulation.candidate_count == 0
        && modulation.quiescent;
    let l6 = row.initial_after_first == initial_four
        && row.initial_one_exposure_gap == z6
        && row.reversed_after_first == reversed_four
        && row.reversed_one_exposure_gap == z6;
    let l7 = row.initial_proposals == 2
        && row.initial_primitive_trace == [2; 4]
        && row.initial_opportunity == initial
        && row.initial_p == initial
        && row.initial_candidate_crossings == initial
        && row.initial_candidate_impulse == initial_impulse
        && row.initial_effect == initial
        && row.initial_p_trace == initial
        && row.initial_effect_trace == initial
        && row.initial_attribution == initial
        && row.initial_credit == initial
        && row.initial_after_train == initial_six
        && row.initial_after_gap == initial_two
        && row.reversed_proposals == 2
        && row.reversed_primitive_trace == [2; 4]
        && row.reversed_opportunity == reversed
        && row.reversed_p == reversed
        && row.reversed_candidate_crossings == reversed
        && row.reversed_candidate_impulse == reversed_impulse
        && row.reversed_effect == reversed
        && row.reversed_p_trace == reversed
        && row.reversed_effect_trace == reversed
        && row.reversed_attribution == reversed
        && row.reversed_credit == reversed
        && row.reversed_after_train == reversed_six
        && row.reversed_after_gap == reversed_two
        && row.return_updates == 8
        && row.total_proposals == 4;
    let l8 = row.old_after_forgetting == [0, 0]
        && row.old_live_after_forgetting == [false, false]
        && row.proposals_during_forgetting == 0
        && row.old_still_dead == [true, true]
        && row.historical_counts == [1, 0, 1, 1, 0, 1]
        && row.deallocations == 6;
    let l9 = row.fresh_identity && row.final_live_counts == [0, 0, 1, 1, 0, 0];
    let l10 = row.pre_reversal_heldout == [1, 0, 0, 0, 0, 1]
        && row.pre_reversal_heldout_proposals == [0, 1, 1, 1, 1, 0]
        && row.post_reversal_heldout == [0, 0, 1, 1, 0, 0]
        && row.post_reversal_heldout_proposals == [1, 1, 0, 0, 1, 1]
        && row.quiescent;
    let l11 = row.quiescent;
    [l0, l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11]
}

fn control_passes(
    metric: &ControlMetrics,
    source_firings: [usize; 4],
    raw_crossings: [usize; 4],
    raw_impulse: [i32; 4],
    participant_traces: [usize; 4],
) -> bool {
    metric.source_firings == source_firings
        && metric.raw_crossings == raw_crossings
        && metric.raw_impulse == raw_impulse
        && metric.participant_traces == participant_traces
        && metric.opportunities == 0
        && metric.candidate_sources == 0
        && metric.candidate_crossings == 0
        && metric.proposals == 0
        && metric.updates == 0
        && metric.live_after_gap == 0
        && metric.quiescent
}

fn control(namespace: u64, reverse: bool, reflect: bool, kind: ControlKind) -> ControlMetrics {
    let raw_couplings = match kind {
        ControlKind::StrongA => [4, 1, 1, 1],
        ControlKind::RepeatedA | ControlKind::GappedAb => [1; 4],
    };
    let mut world = build(namespace, reverse, reflect, raw_couplings);
    match kind {
        ControlKind::StrongA => {
            pulse(&mut world.substrate, world.sources[0], 0, 1, 0);
            background_candidates(&mut world, 1);
        }
        ControlKind::RepeatedA => {
            pulse(&mut world.substrate, world.sources[0], 0, 1, 0);
            pulse(&mut world.substrate, world.sources[0], 0, 1, 1000);
            background_candidates(&mut world, 1);
        }
        ControlKind::GappedAb => {
            pulse(&mut world.substrate, world.sources[0], 0, 1, 0);
            background_candidates(&mut world, 1);
            pulse(&mut world.substrate, world.sources[1], 2, 1, 1);
            background_candidates(&mut world, 3);
        }
    }
    let execution = world.substrate.propagate();
    let log = Log {
        trace: execution.trace,
        crossings: execution.crossings,
        work: execution.work,
        quiescent: execution.naturally_quiescent,
    };
    let source_firings = four(|side| fires(&log.trace, physical(namespace, 10 + side as u64)));
    let raw_crossings = four(|side| {
        crossings(
            &log.crossings,
            physical(namespace, 10 + side as u64),
            physical(namespace, 20 + side as u64),
        )
    });
    let raw_impulse = four(|side| {
        log.crossings
            .iter()
            .filter(|crossing| {
                crossing.from_physical == physical(namespace, 10 + side as u64)
                    && crossing.to_physical == physical(namespace, 20 + side as u64)
            })
            .map(|crossing| crossing.impulse)
            .sum()
    });
    let participant_traces = four(|side| fires(&log.trace, physical(namespace, 30 + side as u64)));
    let opportunities = (0..6)
        .map(|pair| fires(&log.trace, physical(namespace, 100 + pair as u64)))
        .sum();
    let candidate_sources = (0..6)
        .map(|pair| fires(&log.trace, physical(namespace, 200 + pair as u64)))
        .sum();
    let candidate_crossings = (0..6)
        .map(|pair| candidate_crossings(&log, namespace, pair))
        .sum();
    let proposals = log.work.local_structural_proposals;
    let updates = log.work.local_return_updates;
    world.substrate.advance_time(50);
    let live_after_gap = (0..6).map(|pair| live_arrows(&world, pair).len()).sum();
    ControlMetrics {
        source_firings,
        raw_crossings,
        raw_impulse,
        participant_traces,
        opportunities,
        candidate_sources,
        candidate_crossings,
        proposals,
        updates,
        live_after_gap,
        quiescent: log.quiescent,
    }
}

fn modulation_without_participation(
    namespace: u64,
    reverse: bool,
    reflect: bool,
) -> ModulationControl {
    let mut world = build(namespace, reverse, reflect, [1; 4]);
    pulse(&mut world.substrate, world.effects[0], 0, 2, 2000);
    pulse(&mut world.substrate, world.global_return, 1, 1, 2001);
    let execution = world.substrate.propagate();
    ModulationControl {
        effect_path: fires(&execution.trace, physical(namespace, 600)),
        return_relay: fires(&execution.trace, physical(namespace, 400)),
        transmitter: fires(&execution.trace, physical(namespace, 800)),
        modulatory_crossings: crossings(
            &execution.crossings,
            physical(namespace, 800),
            physical(namespace, 200),
        ),
        candidate_source_firings: fires(&execution.trace, physical(namespace, 200)),
        proposals: execution.work.local_structural_proposals,
        updates: execution.work.local_return_updates,
        candidate_count: variable_count(&world),
        quiescent: execution.naturally_quiescent,
    }
}

fn build(namespace: u64, reverse: bool, reflect: bool, raw_couplings: [i32; 4]) -> World {
    let mut substrate = PlasticSubstrate::new();
    let participant_order = if reverse { [3, 2, 1, 0] } else { [0, 1, 2, 3] };
    let pair_order = if reverse {
        [5, 4, 3, 2, 1, 0]
    } else {
        [0, 1, 2, 3, 4, 5]
    };

    let mut sources = [None; 4];
    let mut outlets = [None; 4];
    let mut traces = [None; 4];
    let mut hubs = [None; 4];
    for side in participant_order {
        sources[side] = Some(substrate.add_cell(cell(
            physical(namespace, 10 + side as u64),
            -40_000 - side as i32 * 100,
            10 + side as i16,
            1,
        )));
        outlets[side] = Some(substrate.add_cell(cell(
            physical(namespace, 20 + side as u64),
            -30_000 - side as i32 * 100,
            20 + side as i16,
            1,
        )));
        traces[side] = Some(substrate.add_cell(cell(
            physical(namespace, 30 + side as u64),
            -20_000 - side as i32 * 100,
            30 + side as i16,
            2,
        )));
        hubs[side] = Some(substrate.add_cell(cell(
            physical(namespace, 40 + side as u64),
            -10_000 - side as i32 * 100,
            40 + side as i16,
            1,
        )));
    }
    let sources = sources.map(|cell| cell.expect("source"));
    let outlets = outlets.map(|cell| cell.expect("outlet"));
    let traces = traces.map(|cell| cell.expect("trace"));
    let hubs = hubs.map(|cell| cell.expect("hub"));

    let mut opportunities = [None; 6];
    let mut candidate_sources = [None; 6];
    let mut effects = [None; 6];
    let mut p_traces = [None; 6];
    let mut effect_traces = [None; 6];
    let mut attributions = [None; 6];
    for pair in pair_order {
        opportunities[pair] = Some(substrate.add_cell(cell(
            physical(namespace, 100 + pair as u64),
            -1_000 - pair as i32 * 100,
            50 + pair as i16,
            2,
        )));
        let p_position = 10_000 + pair as i32 * 100;
        candidate_sources[pair] = Some(substrate.add_cell(cell(
            physical(namespace, 200 + pair as u64),
            p_position,
            60 + pair as i16,
            2,
        )));
        effects[pair] = Some(substrate.add_cell(cell(
            physical(namespace, 300 + pair as u64),
            p_position + if reflect { -1 } else { 1 },
            70 + pair as i16,
            2,
        )));
        p_traces[pair] = Some(substrate.add_cell(cell(
            physical(namespace, 400 + pair as u64),
            30_000 + pair as i32 * 100,
            80 + pair as i16,
            2,
        )));
        effect_traces[pair] = Some(substrate.add_cell(cell(
            physical(namespace, 600 + pair as u64),
            50_000 + pair as i32 * 100,
            100 + pair as i16,
            1,
        )));
        attributions[pair] = Some(substrate.add_cell(cell(
            physical(namespace, 800 + pair as u64),
            70_000 + pair as i32 * 100,
            120 + pair as i16,
            1,
        )));
    }
    let opportunities = opportunities.map(|cell| cell.expect("opportunity"));
    let candidate_sources = candidate_sources.map(|cell| cell.expect("candidate source"));
    let effects = effects.map(|cell| cell.expect("effect"));
    let p_traces = p_traces.map(|cell| cell.expect("P trace"));
    let effect_traces = effect_traces.map(|cell| cell.expect("effect trace"));
    let attributions = attributions.map(|cell| cell.expect("attribution"));
    let context = substrate.add_cell(cell(physical(namespace, 900), 80_000, 130, 1));
    let global_return = substrate.add_cell(cell(physical(namespace, 901), 90_000, 131, 1));

    for side in participant_order {
        substrate.add_arrow(fixed(sources[side], outlets[side], 0, raw_couplings[side]));
        substrate.add_arrow(fixed(outlets[side], traces[side], 1, 1));
        substrate.add_arrow(fixed(outlets[side], hubs[side], 1, 1));
        substrate.add_arrow(fixed(hubs[side], traces[side], 0, 1));
    }
    for pair in pair_order {
        let (left, right) = PAIRS[pair];
        substrate.add_arrow(fixed(traces[left], opportunities[pair], 0, 1));
        substrate.add_arrow(fixed(traces[right], opportunities[pair], 0, 1));
        substrate.add_arrow(fixed(opportunities[pair], candidate_sources[pair], 0, 1));
        substrate.add_arrow(fixed(context, effects[pair], 1, 1));

        substrate.add_arrow(fixed(effects[pair], effect_traces[pair], 0, 1));
        substrate.add_arrow(fixed(effect_traces[pair], p_traces[pair], 1, 1));
        substrate.add_arrow(fixed(global_return, p_traces[pair], 0, 1));
        substrate.add_arrow(fixed(p_traces[pair], attributions[pair], 0, 1));
        substrate.add_arrow(modulatory(
            attributions[pair],
            candidate_sources[pair],
            1,
            1,
        ));
    }

    World {
        substrate,
        namespace,
        sources,
        candidate_sources,
        effects,
        context,
        global_return,
    }
}

fn expose(world: &mut World, pair: usize, start: i64, log: &mut Log) {
    let (left, right) = PAIRS[pair];
    pulse(
        &mut world.substrate,
        world.sources[left],
        start,
        1,
        left as i32,
    );
    pulse(
        &mut world.substrate,
        world.sources[right],
        start,
        1,
        right as i32,
    );
    for index in 0..6 {
        pulse(
            &mut world.substrate,
            world.candidate_sources[index],
            start + 1,
            1,
            100 + index as i32,
        );
    }
    pulse(&mut world.substrate, world.context, start + 1, 1, 500);
    pulse(&mut world.substrate, world.global_return, start + 3, 1, 600);
    log.execution(world.substrate.propagate());
}

fn expose_unsupported(world: &mut World, pair: usize, start: i64, log: &mut Log) {
    let (left, right) = PAIRS[pair];
    pulse(
        &mut world.substrate,
        world.sources[left],
        start,
        1,
        left as i32,
    );
    pulse(
        &mut world.substrate,
        world.sources[right],
        start,
        1,
        right as i32,
    );
    for index in 0..6 {
        pulse(
            &mut world.substrate,
            world.candidate_sources[index],
            start + 1,
            1,
            100 + index as i32,
        );
    }
    log.execution(world.substrate.propagate());
}

fn background_candidates(world: &mut World, tick: i64) {
    for index in 0..6 {
        pulse(
            &mut world.substrate,
            world.candidate_sources[index],
            tick,
            1,
            700 + index as i32,
        );
    }
}

fn heldout(world: &World, start: i64) -> Heldout {
    let mut effects = [0; 6];
    let mut proposals = [0; 6];
    let mut quiescent = true;
    for pair in 0..6 {
        let mut clone = world.clone();
        let (left, right) = PAIRS[pair];
        pulse(
            &mut clone.substrate,
            clone.sources[left],
            start,
            1,
            left as i32,
        );
        pulse(
            &mut clone.substrate,
            clone.sources[right],
            start,
            1,
            right as i32,
        );
        for index in 0..6 {
            pulse(
                &mut clone.substrate,
                clone.candidate_sources[index],
                start + 1,
                1,
                100 + index as i32,
            );
        }
        let execution = clone.substrate.propagate();
        effects[pair] = fires(
            &execution.trace,
            physical(clone.namespace, 300 + pair as u64),
        );
        proposals[pair] = execution.work.local_structural_proposals;
        quiescent &= execution.naturally_quiescent;
    }
    Heldout {
        effects,
        proposals,
        quiescent,
    }
}

fn variable_arrows(world: &World, pair: usize) -> Vec<ArrowId> {
    world
        .substrate
        .arrows_between(world.candidate_sources[pair], world.effects[pair])
}

fn live_arrows(world: &World, pair: usize) -> Vec<ArrowId> {
    variable_arrows(world, pair)
        .into_iter()
        .filter(|arrow| world.substrate.arrow_is_live(*arrow))
        .collect()
}

fn variable_count(world: &World) -> usize {
    (0..6).map(|pair| variable_arrows(world, pair).len()).sum()
}

fn only_historical(world: &World, pair: usize) -> ArrowId {
    let arrows = variable_arrows(world, pair);
    assert_eq!(arrows.len(), 1, "one historical candidate");
    arrows[0]
}

fn only_live(world: &World, pair: usize) -> ArrowId {
    let arrows = live_arrows(world, pair);
    assert_eq!(arrows.len(), 1, "one live candidate");
    arrows[0]
}

fn current_resistance(world: &World) -> [u32; 6] {
    six(|pair| {
        let arrows = live_arrows(world, pair);
        assert!(arrows.len() <= 1, "at most one live candidate");
        arrows
            .first()
            .map_or(0, |arrow| world.substrate.arrow_resistance(*arrow))
    })
}

fn candidate_crossings(log: &Log, namespace: u64, pair: usize) -> usize {
    crossings(
        &log.crossings,
        physical(namespace, 200 + pair as u64),
        physical(namespace, 300 + pair as u64),
    )
}

fn candidate_impulse(log: &Log, namespace: u64, pair: usize) -> i32 {
    log.crossings
        .iter()
        .filter(|crossing| {
            crossing.from_physical == physical(namespace, 200 + pair as u64)
                && crossing.to_physical == physical(namespace, 300 + pair as u64)
        })
        .map(|crossing| crossing.impulse)
        .sum()
}

fn credit_crossings(log: &Log, namespace: u64, pair: usize) -> usize {
    crossings(
        &log.crossings,
        physical(namespace, 800 + pair as u64),
        physical(namespace, 200 + pair as u64),
    )
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

fn fixed(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 100,
        mode: TransmissionMode::Drive,
    }
}

fn modulatory(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 100,
        mode: TransmissionMode::Modulatory,
    }
}

fn pulse(substrate: &mut PlasticSubstrate, target: CellId, tick: i64, impulse: i32, phase: i32) {
    substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: 900_000 + phase as u64,
        target,
        impulse,
    });
}

fn physical(namespace: u64, suffix: u64) -> u64 {
    namespace + suffix
}

fn fires(trace: &[TraceEntry], physical_id: u64) -> usize {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical_id && entry.fired)
        .count()
}

fn crossings(crossings: &[Crossing], from: u64, to: u64) -> usize {
    crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .count()
}

fn six<T>(mut function: impl FnMut(usize) -> T) -> [T; 6] {
    [
        function(0),
        function(1),
        function(2),
        function(3),
        function(4),
        function(5),
    ]
}

fn four<T>(mut function: impl FnMut(usize) -> T) -> [T; 4] {
    [function(0), function(1), function(2), function(3)]
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
    total.qualified_return_checks += next.qualified_return_checks;
    total.qualified_return_accepts += next.qualified_return_accepts;
    total.qualified_return_path_edges += next.qualified_return_path_edges;
    total.drive_deliveries += next.drive_deliveries;
    total.modulatory_deliveries += next.modulatory_deliveries;
    total.ordinary_pressure_updates += next.ordinary_pressure_updates;
    total.local_structural_proposals += next.local_structural_proposals;
    total.physical_deallocations += next.physical_deallocations;
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "seed,namespace,reverse,reflect,strong_a,repeated_a,gapped_ab,modulation_without_participation,unsupported_updates,unsupported_candidate_crossings,unsupported_after_gap,initial_candidate_count,initial_proposals,initial_primitive_trace,initial_opportunity,initial_p,initial_candidate_crossings,initial_candidate_impulse,initial_effect,return_relay,downstream_path,modulatory_transmitter,modulatory_crossing,initial_after_first,initial_one_exposure_gap,initial_after_train,initial_after_gap,pre_reversal_heldout,pre_reversal_heldout_proposals,old_ids,old_generations,old_after_forgetting,old_live_after_forgetting,proposals_during_forgetting,reversed_proposals,reversed_primitive_trace,reversed_opportunity,reversed_p,reversed_candidate_crossings,reversed_candidate_impulse,reversed_effect,reversed_return_relay,reversed_downstream_path,reversed_modulatory_transmitter,reversed_modulatory_crossing,reversed_after_first,reversed_one_exposure_gap,reversed_after_train,reversed_after_gap,new_ids,new_generations,fresh_identity,old_still_dead,historical_counts,final_live_counts,post_reversal_heldout,post_reversal_heldout_proposals,total_proposals,deallocations,return_updates,work,bytes,fingerprint,permanent,quiescent,claims,replay,passed\n",
    );
    for row in rows {
        let fields = vec![
            row.seed.to_string(),
            row.namespace.to_string(),
            row.reverse.to_string(),
            row.reflect.to_string(),
            control_signature(&row.strong_a),
            control_signature(&row.repeated_a),
            control_signature(&row.gapped_ab),
            modulation_signature(&row.modulation_without_participation),
            row.unsupported_updates.to_string(),
            row.unsupported_candidate_crossings.to_string(),
            row.unsupported_after_gap.to_string(),
            row.initial_candidate_count.to_string(),
            row.initial_proposals.to_string(),
            join_usize(&row.initial_primitive_trace),
            join_usize(&row.initial_opportunity),
            join_usize(&row.initial_p),
            join_usize(&row.initial_candidate_crossings),
            join_i32(&row.initial_candidate_impulse),
            join_usize(&row.initial_effect),
            join_usize(&row.initial_p_trace),
            join_usize(&row.initial_effect_trace),
            join_usize(&row.initial_attribution),
            join_usize(&row.initial_credit),
            join_u32(&row.initial_after_first),
            join_u32(&row.initial_one_exposure_gap),
            join_u32(&row.initial_after_train),
            join_u32(&row.initial_after_gap),
            join_usize(&row.pre_reversal_heldout),
            join_u64(&row.pre_reversal_heldout_proposals),
            row.old_ids.join("|"),
            join_u32(&row.old_generations),
            join_u32(&row.old_after_forgetting),
            join_bool(&row.old_live_after_forgetting),
            row.proposals_during_forgetting.to_string(),
            row.reversed_proposals.to_string(),
            join_usize(&row.reversed_primitive_trace),
            join_usize(&row.reversed_opportunity),
            join_usize(&row.reversed_p),
            join_usize(&row.reversed_candidate_crossings),
            join_i32(&row.reversed_candidate_impulse),
            join_usize(&row.reversed_effect),
            join_usize(&row.reversed_p_trace),
            join_usize(&row.reversed_effect_trace),
            join_usize(&row.reversed_attribution),
            join_usize(&row.reversed_credit),
            join_u32(&row.reversed_after_first),
            join_u32(&row.reversed_one_exposure_gap),
            join_u32(&row.reversed_after_train),
            join_u32(&row.reversed_after_gap),
            row.new_ids.join("|"),
            join_u32(&row.new_generations),
            row.fresh_identity.to_string(),
            join_bool(&row.old_still_dead),
            join_usize(&row.historical_counts),
            join_usize(&row.final_live_counts),
            join_usize(&row.post_reversal_heldout),
            join_u64(&row.post_reversal_heldout_proposals),
            row.total_proposals.to_string(),
            row.deallocations.to_string(),
            row.return_updates.to_string(),
            row.work.to_string(),
            row.bytes.to_string(),
            row.fingerprint.to_string(),
            row.permanent.to_string(),
            row.quiescent.to_string(),
            join_bool(&row.claims),
            row.replay.to_string(),
            row.passed.to_string(),
        ];
        output.push_str(&fields.join(","));
        output.push('\n');
    }
    output
}

fn report(rows: &[Row]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let clauses = rows
        .iter()
        .map(|row| row.claims.into_iter().filter(|claim| *claim).count())
        .sum::<usize>();
    format!(
        "# PX3 LR-C lifecycle and reversal definitive v1\n\nOutcome: **{}**.\n\n- rows: `{passed}/{}` passed;\n- independent clauses: `{clauses}/192`;\n- exact replay: `{}`;\n- naturally quiescent: `{}`;\n- strong-A / repeated-A / gapped-AB controls: `{}`;\n- adjacent unsupported updates/gap: `{}`;\n- modulation without participation rejected: `{}`;\n- initial/reversed native proposals: `{}` / `{}`;\n- old candidates dead before reversal: `{}`;\n- fresh reversed identities: `{}`;\n- total structural proposals: `{}`;\n- active R3/R4/R5/R6 geometry: `false`;\n- recursion half executed here: `false`.\n",
        if passed == rows.len() {
            "PX3-LRC-LIFECYCLE DEFINITIVE POSITIVE"
        } else {
            "NEGATIVE"
        },
        rows.len(),
        rows.iter().all(|row| row.replay),
        rows.iter().all(|row| row.quiescent),
        rows.iter().all(|row| {
            row.strong_a.opportunities == 0
                && row.repeated_a.opportunities == 0
                && row.gapped_ab.opportunities == 0
        }),
        rows
            .iter()
            .map(|row| format!("{}|{}", row.unsupported_updates, row.unsupported_after_gap))
            .collect::<Vec<_>>()
            .join(";"),
        rows.iter().all(|row| row.claims[5]),
        rows.iter().map(|row| row.initial_proposals).sum::<u64>(),
        rows.iter().map(|row| row.reversed_proposals).sum::<u64>(),
        rows.iter().all(|row| row.old_live_after_forgetting == [false, false]),
        rows.iter().all(|row| row.fresh_identity),
        rows.iter().map(|row| row.total_proposals).sum::<u64>(),
    )
}

fn control_signature(metric: &ControlMetrics) -> String {
    format!(
        "src={}~raw={}~imp={}~trace={}~o={}~p={}~cand={}~prop={}~updates={}~live={}~q={}",
        join_usize(&metric.source_firings),
        join_usize(&metric.raw_crossings),
        join_i32(&metric.raw_impulse),
        join_usize(&metric.participant_traces),
        metric.opportunities,
        metric.candidate_sources,
        metric.candidate_crossings,
        metric.proposals,
        metric.updates,
        metric.live_after_gap,
        metric.quiescent,
    )
}

fn modulation_signature(metric: &ModulationControl) -> String {
    format!(
        "effect={}~relay={}~tx={}~mod={}~p={}~prop={}~updates={}~cand={}~q={}",
        metric.effect_path,
        metric.return_relay,
        metric.transmitter,
        metric.modulatory_crossings,
        metric.candidate_source_firings,
        metric.proposals,
        metric.updates,
        metric.candidate_count,
        metric.quiescent,
    )
}

fn join_usize(values: &[usize]) -> String {
    values
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_i32(values: &[i32]) -> String {
    values
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_u32(values: &[u32]) -> String {
    values
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_u64(values: &[u64]) -> String {
    values
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_bool(values: &[bool]) -> String {
    values
        .iter()
        .map(bool::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn absent(paths: &[&str]) {
    for path in paths {
        assert!(!Path::new(path).exists(), "artifact exists: {path}");
    }
}

fn publish(stage: &str, destination: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(stage)
        .expect("create staging artifact");
    file.write_all(content.as_bytes()).expect("write artifact");
    file.sync_all().expect("sync artifact");
    rename(stage, destination).expect("publish artifact");
}

fn sha(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum");
    assert!(output.status.success(), "sha256sum failed: {path}");
    String::from_utf8(output.stdout)
        .expect("utf8")
        .split_whitespace()
        .next()
        .expect("digest")
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_is_frozen() {
        surface();
        assert_eq!(SEEDS.len(), 16);
        assert_eq!(SEEDS[0], 91001);
        assert_eq!(SEEDS[15], 91016);
    }

    #[test]
    fn pair_reversal_is_exact() {
        assert_eq!(PAIRS[INITIAL[0]], (0, 1));
        assert_eq!(PAIRS[INITIAL[1]], (2, 3));
        assert_eq!(PAIRS[REVERSED[0]], (0, 3));
        assert_eq!(PAIRS[REVERSED[1]], (1, 2));
    }
}
