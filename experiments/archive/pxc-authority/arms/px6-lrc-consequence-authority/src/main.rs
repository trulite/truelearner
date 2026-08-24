#![forbid(unsafe_code)]
//! Serial PX6 physical-consequence authority evaluator.
//!
//! This binary adds no organism mechanism. It constructs fresh physical
//! worlds around the authoritative LR-C substrate and reads public physical
//! observables after natural propagation.

use lr1_modulatory_physical_return::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
    TransmissionMode, WorkLedger,
};
use px4_lrc_lifetime::{arrive, field};
use std::collections::BTreeSet;
use std::env;
use std::fmt::Write as _;
use std::fs::{rename, OpenOptions};
use std::io::Write as _;
use std::path::Path;
use std::process::Command;

const LAW_HASH: &str = "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10";
const PX4_SOURCE_HASH: &str = "a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71";
const PX5_HANDOFF_HASH: &str = "1e6c947660b55d6f21060734c57a54f56541303b65069f23010344ca7d362a97";
const PX5_CSV_HASH: &str = "5ccfa15b6da93ac276b9474c4d501ef9c7769748c52dbf7a8882620758b1259a";
const PX5_REPORT_HASH: &str = "e96622614e4c9569f1f90d60fa0ef822072afae5e09c316b2c37344e31f194ed";
const PX5_MANIFEST_HASH: &str = "32fda2d86f6c836438fdd01fc5433c8731f4ec92e3e052e5d1a64fe751d15388";
const PROTOCOL_HASH: &str = "bec04fbcefa97567ab8e3034c38915517460693acd2d57376c41eae4dd898990";

const ROOTS: [u64; 8] = [
    661001, 661002, 661003, 661004, 661005, 661006, 661007, 661008,
];
const LOADS: [usize; 3] = [8, 32, 128];
const WORK_LIMIT: u64 = 150_000;
const BYTE_LIMIT: usize = 32_000;
const CSV: &str = "results/px6_lrc_consequence_authority_v1.csv";
const REPORT: &str = "results/px6_lrc_consequence_authority_v1.md";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReturnShape {
    Direct,
    Relayed,
}

impl ReturnShape {
    fn label(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Relayed => "relayed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Config {
    namespace: u64,
    load: usize,
    return_tick: i64,
    shape: ReturnShape,
    reflect: bool,
    reverse_arrivals: bool,
    returned_side: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Resources {
    work: u64,
    max_bytes: usize,
    reuse_stable: bool,
    quiescent: bool,
}

impl Resources {
    fn new() -> Self {
        Self {
            work: 0,
            max_bytes: 0,
            reuse_stable: true,
            quiescent: true,
        }
    }

    fn execution(&mut self, substrate: &PlasticSubstrate, execution: &Execution) {
        self.work = self.work.saturating_add(execution.work.total());
        self.max_bytes = self.max_bytes.max(substrate.persistent_bytes());
        self.quiescent &= execution.naturally_quiescent;
    }

    fn pressure(&mut self, substrate: &PlasticSubstrate, work: &WorkLedger) {
        self.work = self.work.saturating_add(work.total());
        self.max_bytes = self.max_bytes.max(substrate.persistent_bytes());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ReturnPathSpec {
    namespace: u64,
    effect: CellId,
    transmitter: CellId,
    source: CellId,
    return_tick: i64,
    shape: ReturnShape,
    resistance: u32,
    reflect: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct PhysicalLog {
    drive_deliveries: u64,
    modulatory_deliveries: u64,
    updates: u64,
    work: u64,
    quiescent: bool,
}

impl PhysicalLog {
    fn from_execution(execution: &Execution) -> Self {
        Self {
            drive_deliveries: execution.work.drive_deliveries,
            modulatory_deliveries: execution.work.modulatory_deliveries,
            updates: execution.work.local_return_updates,
            work: execution.work.total(),
            quiescent: execution.naturally_quiescent,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LoopMeasure {
    resistance: u32,
    coupling: i32,
    distractor_crossings: usize,
    fingerprint: u64,
    permanent: u64,
    log: PhysicalLog,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DirectionMeasure {
    resistance: [u32; 2],
    live: [bool; 2],
    updates: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PathVariationMeasure {
    stable_resistance: u32,
    varying_resistance: u32,
    weak_path_removed: bool,
    replacement_crossed: bool,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LifecycleMeasure {
    initial_generation: u32,
    replacement_generation: u32,
    initial_dead: bool,
    replacement_live: bool,
    replacement_resistance: u32,
    proposals: u64,
    deallocations: u64,
    updates: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ConformanceMeasure {
    px0: bool,
    px1: bool,
    px2: bool,
    px3: bool,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CoreObservation {
    candidate_resistance: u32,
    candidate_coupling: i32,
    updates: u64,
    modulatory_deliveries: u64,
    drive_deliveries: u64,
    distractor_crossings: usize,
    loop_work: u64,
    work: u64,
    max_bytes: usize,
    fingerprint: u64,
    permanent: u64,
    modulation_learning: bool,
    modulation_without_participation: bool,
    ordinary_drive_while_eligible: bool,
    blocked_downstream: bool,
    missing_downstream: bool,
    immediate_modulation: bool,
    late_modulation: bool,
    stable_and_varying_return: bool,
    physical_side_reversal: bool,
    multiple_lawful_loops: bool,
    pressure_persistence: bool,
    deallocation_reacquisition: bool,
    px0: bool,
    px1: bool,
    px2: bool,
    px3: bool,
    px4: bool,
    px5: bool,
    reuse_stable: bool,
    quiescent: bool,
}

impl CoreObservation {
    fn passed(&self) -> bool {
        self.modulation_learning
            && self.modulation_without_participation
            && self.ordinary_drive_while_eligible
            && self.blocked_downstream
            && self.missing_downstream
            && self.immediate_modulation
            && self.late_modulation
            && self.stable_and_varying_return
            && self.physical_side_reversal
            && self.multiple_lawful_loops
            && self.pressure_persistence
            && self.deallocation_reacquisition
            && self.px0
            && self.px1
            && self.px2
            && self.px3
            && self.px4
            && self.px5
            && self.work <= WORK_LIMIT
            && self.max_bytes <= BYTE_LIMIT
            && self.reuse_stable
            && self.quiescent
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    root: u64,
    load: usize,
    config: Config,
    core: CoreObservation,
    replay: bool,
    claims: [bool; 23],
    passed: bool,
}

#[derive(Clone, Copy)]
enum AuthorityPermission {
    Granted,
}

fn physical(namespace: u64, offset: u64) -> u64 {
    namespace.wrapping_add(offset)
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

fn arrow(
    from: CellId,
    to: CellId,
    delay: i64,
    coupling: i32,
    resistance: u32,
    mode: TransmissionMode,
) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode,
    }
}

fn drive(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    arrow(
        from,
        to,
        delay,
        coupling,
        resistance,
        TransmissionMode::Drive,
    )
}

fn modulatory(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    arrow(
        from,
        to,
        delay,
        coupling,
        resistance,
        TransmissionMode::Modulatory,
    )
}

fn pulse(substrate: &mut PlasticSubstrate, target: CellId, tick: i64, phase: i32) {
    substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: 0,
        target,
        impulse: 1,
    });
}

fn add_three_cells(
    substrate: &mut PlasticSubstrate,
    namespace: u64,
    reflect: bool,
    reverse_insertion: bool,
    effect_threshold: i32,
) -> [CellId; 3] {
    let sign = if reflect { -1 } else { 1 };
    let specs = [
        cell(physical(namespace, 10), sign * 100, 10, 1),
        cell(physical(namespace, 20), sign * 1_000, 20, effect_threshold),
        cell(physical(namespace, 30), sign * 2_000, 30, 1),
    ];
    let order = if reverse_insertion {
        [2, 1, 0]
    } else {
        [0, 1, 2]
    };
    let mut ids = [None; 3];
    for index in order {
        ids[index] = Some(substrate.add_cell(specs[index]));
    }
    ids.map(|id| id.expect("logical cell installed"))
}

fn install_return_path(substrate: &mut PlasticSubstrate, spec: ReturnPathSpec) -> Vec<ArrowId> {
    let first_delay = spec.return_tick / 2;
    let final_delay = spec.return_tick - first_delay;
    match spec.shape {
        ReturnShape::Direct => vec![
            substrate.add_arrow(drive(
                spec.effect,
                spec.transmitter,
                first_delay,
                1,
                spec.resistance,
            )),
            substrate.add_arrow(modulatory(
                spec.transmitter,
                spec.source,
                final_delay,
                1,
                spec.resistance,
            )),
        ],
        ReturnShape::Relayed => {
            let sign = if spec.reflect { -1 } else { 1 };
            let relay = substrate.add_cell(cell(physical(spec.namespace, 40), sign * 1_500, 25, 1));
            vec![
                substrate.add_arrow(drive(spec.effect, relay, 0, 1, spec.resistance)),
                substrate.add_arrow(drive(
                    relay,
                    spec.transmitter,
                    first_delay,
                    1,
                    spec.resistance,
                )),
                substrate.add_arrow(modulatory(
                    spec.transmitter,
                    spec.source,
                    final_delay,
                    1,
                    spec.resistance,
                )),
            ]
        }
    }
}

fn loop_trial(config: Config, resources: &mut Resources) -> LoopMeasure {
    let mut substrate = PlasticSubstrate::new();
    let [source, effect, transmitter] = add_three_cells(
        &mut substrate,
        config.namespace,
        config.reflect,
        config.reverse_arrivals,
        1,
    );
    let candidate = substrate.add_arrow(drive(source, effect, 0, 1, 1));
    install_return_path(
        &mut substrate,
        ReturnPathSpec {
            namespace: config.namespace,
            effect,
            transmitter,
            source,
            return_tick: config.return_tick,
            shape: config.shape,
            resistance: 8,
            reflect: config.reflect,
        },
    );

    let sign = if config.reflect { -1 } else { 1 };
    let mut distractor_ids = Vec::with_capacity(config.load);
    for index in 0..config.load {
        let offset = 1_000 + index as u64 * 2;
        let position = 10_000 + index as i32 * 10;
        let from = substrate.add_cell(cell(
            physical(config.namespace, offset),
            sign * position,
            100 + index as i16,
            1,
        ));
        let to = substrate.add_cell(cell(
            physical(config.namespace, offset + 1),
            sign * (position + 5),
            500 + index as i16,
            1,
        ));
        substrate.add_arrow(drive(from, to, (index % 5) as i64, 1, 8));
        distractor_ids.push(physical(config.namespace, offset));
        pulse(&mut substrate, from, 0, index as i32);
    }
    pulse(&mut substrate, source, 0, 10_000);
    let execution = substrate.propagate();
    resources.execution(&substrate, &execution);
    let distractor_crossings = execution
        .crossings
        .iter()
        .filter(|crossing| distractor_ids.contains(&crossing.from_physical))
        .count();
    LoopMeasure {
        resistance: substrate.arrow_resistance(candidate),
        coupling: substrate.arrow_coupling(candidate),
        distractor_crossings,
        fingerprint: substrate.complete_fingerprint(),
        permanent: substrate.permanent_fingerprint(),
        log: PhysicalLog::from_execution(&execution),
    }
}

fn isolated_trial(
    namespace: u64,
    kind: &str,
    resources: &mut Resources,
) -> (u32, bool, PhysicalLog) {
    let mut substrate = PlasticSubstrate::new();
    let [source, effect, transmitter] = add_three_cells(&mut substrate, namespace, false, false, 1);
    let candidate = substrate.add_arrow(drive(source, effect, 0, 1, 1));
    match kind {
        "unparticipated" => {
            substrate.add_arrow(modulatory(transmitter, source, 0, 1, 8));
            pulse(&mut substrate, transmitter, 0, 0);
        }
        "blocked" => unreachable!("blocked uses a threshold-specific world"),
        "missing" => pulse(&mut substrate, source, 0, 0),
        _ => unreachable!("known isolated trial"),
    }
    let execution = substrate.propagate();
    resources.execution(&substrate, &execution);
    (
        substrate.arrow_resistance(candidate),
        substrate.arrow_eligible_until(candidate).is_some(),
        PhysicalLog::from_execution(&execution),
    )
}

fn blocked_trial(namespace: u64, resources: &mut Resources) -> (u32, bool, PhysicalLog) {
    let mut substrate = PlasticSubstrate::new();
    let [source, effect, transmitter] = add_three_cells(&mut substrate, namespace, false, false, 2);
    let candidate = substrate.add_arrow(drive(source, effect, 0, 1, 1));
    install_return_path(
        &mut substrate,
        ReturnPathSpec {
            namespace,
            effect,
            transmitter,
            source,
            return_tick: 0,
            shape: ReturnShape::Direct,
            resistance: 8,
            reflect: false,
        },
    );
    pulse(&mut substrate, source, 0, 0);
    let execution = substrate.propagate();
    resources.execution(&substrate, &execution);
    (
        substrate.arrow_resistance(candidate),
        substrate.arrow_eligible_until(candidate).is_some(),
        PhysicalLog::from_execution(&execution),
    )
}

fn drive_while_eligible(namespace: u64, resources: &mut Resources) -> (u32, bool, PhysicalLog) {
    let mut substrate = PlasticSubstrate::new();
    let [source, effect, _] = add_three_cells(&mut substrate, namespace, false, false, 2);
    let driver = substrate.add_cell(cell(physical(namespace, 50), 4_000, 40, 1));
    let candidate = substrate.add_arrow(drive(source, effect, 0, 1, 1));
    substrate.add_arrow(drive(driver, source, 0, 0, 8));
    pulse(&mut substrate, source, 0, 0);
    pulse(&mut substrate, driver, 1, 1);
    let execution = substrate.propagate();
    resources.execution(&substrate, &execution);
    (
        substrate.arrow_resistance(candidate),
        substrate.arrow_eligible_until(candidate).is_some(),
        PhysicalLog::from_execution(&execution),
    )
}

fn pressure_trial(namespace: u64, resources: &mut Resources) -> (u32, bool, u32, bool, bool) {
    let positive = Config {
        namespace,
        load: 0,
        return_tick: 0,
        shape: ReturnShape::Direct,
        reflect: false,
        reverse_arrivals: false,
        returned_side: 0,
    };
    let mut supported = PlasticSubstrate::new();
    let [source, effect, transmitter] = add_three_cells(&mut supported, namespace, false, false, 1);
    let supported_arrow = supported.add_arrow(drive(source, effect, 0, 1, 1));
    install_return_path(
        &mut supported,
        ReturnPathSpec {
            namespace,
            effect,
            transmitter,
            source,
            return_tick: positive.return_tick,
            shape: positive.shape,
            resistance: 8,
            reflect: false,
        },
    );
    pulse(&mut supported, source, 0, 0);
    let supported_execution = supported.propagate();
    resources.execution(&supported, &supported_execution);
    let supported_pressure = supported.advance_time(30);
    resources.pressure(&supported, &supported_pressure);

    let mut unsupported = PlasticSubstrate::new();
    let [source, effect, _] = add_three_cells(&mut unsupported, namespace + 100, false, false, 2);
    let unsupported_arrow = unsupported.add_arrow(drive(source, effect, 0, 1, 1));
    pulse(&mut unsupported, source, 0, 0);
    let unsupported_execution = unsupported.propagate();
    resources.execution(&unsupported, &unsupported_execution);
    let unsupported_pressure = unsupported.advance_time(5);
    resources.pressure(&unsupported, &unsupported_pressure);
    (
        supported.arrow_resistance(supported_arrow),
        supported.arrow_is_live(supported_arrow),
        unsupported.arrow_resistance(unsupported_arrow),
        unsupported.arrow_is_live(unsupported_arrow),
        supported_execution.naturally_quiescent && unsupported_execution.naturally_quiescent,
    )
}

fn direction_trial(
    namespace: u64,
    returned_side: usize,
    reverse: bool,
    resources: &mut Resources,
) -> DirectionMeasure {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; 2];
    let mut effects = [None; 2];
    let mut transmitters = [None; 2];
    let insertion = if reverse { [1, 0] } else { [0, 1] };
    for side in insertion {
        sources[side] = Some(substrate.add_cell(cell(
            physical(namespace, 10 + side as u64),
            100 + side as i32 * 1_000,
            10 + side as i16,
            1,
        )));
        effects[side] = Some(substrate.add_cell(cell(
            physical(namespace, 20 + side as u64),
            300 + side as i32 * 1_000,
            20 + side as i16,
            1,
        )));
        transmitters[side] = Some(substrate.add_cell(cell(
            physical(namespace, 30 + side as u64),
            500 + side as i32 * 1_000,
            30 + side as i16,
            1,
        )));
    }
    let sources = sources.map(|id| id.expect("source"));
    let effects = effects.map(|id| id.expect("effect"));
    let transmitters = transmitters.map(|id| id.expect("transmitter"));
    let candidates = [
        substrate.add_arrow(drive(sources[0], effects[0], 0, 1, 1)),
        substrate.add_arrow(drive(sources[1], effects[1], 0, 1, 1)),
    ];
    substrate.add_arrow(drive(
        effects[returned_side],
        transmitters[returned_side],
        0,
        1,
        8,
    ));
    substrate.add_arrow(modulatory(
        transmitters[returned_side],
        sources[returned_side],
        0,
        1,
        8,
    ));
    let arrival_order = if reverse { [1, 0] } else { [0, 1] };
    for (phase, side) in arrival_order.into_iter().enumerate() {
        pulse(&mut substrate, sources[side], 0, phase as i32);
    }
    let execution = substrate.propagate();
    resources.execution(&substrate, &execution);
    let pressure = substrate.advance_time(5);
    resources.pressure(&substrate, &pressure);
    DirectionMeasure {
        resistance: candidates.map(|id| substrate.arrow_resistance(id)),
        live: candidates.map(|id| substrate.arrow_is_live(id)),
        updates: execution.work.local_return_updates,
        quiescent: execution.naturally_quiescent,
    }
}

fn repeated_path_trial(
    namespace: u64,
    varying: bool,
    resources: &mut Resources,
) -> (u32, bool, bool) {
    let mut substrate = PlasticSubstrate::new();
    let [source, effect, transmitter] = add_three_cells(&mut substrate, namespace, false, false, 1);
    let candidate = substrate.add_arrow(drive(source, effect, 0, 1, 1));
    let return_arrows = install_return_path(
        &mut substrate,
        ReturnPathSpec {
            namespace,
            effect,
            transmitter,
            source,
            return_tick: 0,
            shape: ReturnShape::Direct,
            resistance: if varying { 1 } else { 8 },
            reflect: false,
        },
    );
    pulse(&mut substrate, source, 0, 0);
    let first = substrate.propagate();
    resources.execution(&substrate, &first);
    let pressure = substrate.advance_time(5);
    resources.pressure(&substrate, &pressure);
    let weak_path_removed = return_arrows.iter().all(|id| !substrate.arrow_is_live(*id));
    if varying {
        let replacement = substrate.add_cell(cell(physical(namespace, 60), 3_000, 60, 1));
        substrate.add_arrow(drive(effect, replacement, 0, 1, 8));
        substrate.add_arrow(modulatory(replacement, source, 0, 1, 8));
    }
    let bytes_before_reuse = substrate.persistent_bytes();
    let arrows_before_reuse = substrate.arrow_count();
    pulse(&mut substrate, source, 6, 0);
    let second = substrate.propagate();
    resources.execution(&substrate, &second);
    if !varying {
        resources.reuse_stable &= bytes_before_reuse == substrate.persistent_bytes()
            && arrows_before_reuse == substrate.arrow_count();
    }
    (
        substrate.arrow_resistance(candidate),
        !varying || weak_path_removed,
        first.naturally_quiescent && second.naturally_quiescent,
    )
}

fn path_variation_trial(namespace: u64, resources: &mut Resources) -> PathVariationMeasure {
    let (stable_resistance, stable_condition, stable_quiescent) =
        repeated_path_trial(namespace, false, resources);
    let (varying_resistance, weak_path_removed, varying_quiescent) =
        repeated_path_trial(namespace + 100, true, resources);
    PathVariationMeasure {
        stable_resistance,
        varying_resistance,
        weak_path_removed,
        replacement_crossed: varying_resistance == 7,
        quiescent: stable_condition && stable_quiescent && varying_quiescent,
    }
}

fn multiple_loops_trial(
    namespace: u64,
    loops: usize,
    resources: &mut Resources,
) -> (Vec<u32>, u64, bool) {
    let mut substrate = PlasticSubstrate::new();
    let mut candidates = Vec::with_capacity(loops);
    let mut sources = Vec::with_capacity(loops);
    for index in 0..loops {
        let offset = index as u64 * 100;
        let source = substrate.add_cell(cell(
            physical(namespace, offset + 10),
            index as i32 * 10_000,
            10 + index as i16,
            1,
        ));
        let effect = substrate.add_cell(cell(
            physical(namespace, offset + 20),
            index as i32 * 10_000 + 1_000,
            20 + index as i16,
            1,
        ));
        let transmitter = substrate.add_cell(cell(
            physical(namespace, offset + 30),
            index as i32 * 10_000 + 2_000,
            30 + index as i16,
            1,
        ));
        candidates.push(substrate.add_arrow(drive(source, effect, 0, 1, 1)));
        substrate.add_arrow(drive(effect, transmitter, (index % 3) as i64, 1, 8));
        substrate.add_arrow(modulatory(transmitter, source, 0, 1, 8));
        sources.push(source);
    }
    for (phase, source) in sources.into_iter().enumerate() {
        pulse(&mut substrate, source, 0, phase as i32);
    }
    let execution = substrate.propagate();
    resources.execution(&substrate, &execution);
    (
        candidates
            .into_iter()
            .map(|id| substrate.arrow_resistance(id))
            .collect(),
        execution.work.local_return_updates,
        execution.naturally_quiescent,
    )
}

fn lifecycle_trial(namespace: u64, resources: &mut Resources) -> LifecycleMeasure {
    let mut substrate = PlasticSubstrate::new();
    let source = substrate.add_cell(cell(physical(namespace, 10), 0, 10, 1));
    let effect = substrate.add_cell(cell(physical(namespace, 20), 1, 20, 1));
    pulse(&mut substrate, source, 0, 0);
    let first_execution = substrate.propagate();
    resources.execution(&substrate, &first_execution);
    let initial = substrate.arrows_between(source, effect)[0];
    let initial_generation = substrate.arrow_generation(initial);
    let expiry = substrate.advance_time(5);
    resources.pressure(&substrate, &expiry);

    let transmitter = substrate.add_cell(cell(physical(namespace, 30), 1_000, 30, 1));
    substrate.add_arrow(drive(effect, transmitter, 0, 1, 8));
    substrate.add_arrow(modulatory(transmitter, source, 0, 1, 8));
    pulse(&mut substrate, source, 6, 0);
    let second_execution = substrate.propagate();
    resources.execution(&substrate, &second_execution);
    let arrows = substrate.arrows_between(source, effect);
    let replacement = *arrows
        .iter()
        .find(|id| substrate.arrow_is_live(**id))
        .expect("ordinary reproposal creates a live replacement");
    LifecycleMeasure {
        initial_generation,
        replacement_generation: substrate.arrow_generation(replacement),
        initial_dead: !substrate.arrow_is_live(initial),
        replacement_live: substrate.arrow_is_live(replacement),
        replacement_resistance: substrate.arrow_resistance(replacement),
        proposals: first_execution.work.local_structural_proposals
            + second_execution.work.local_structural_proposals,
        deallocations: expiry.physical_deallocations,
        updates: second_execution.work.local_return_updates,
        quiescent: first_execution.naturally_quiescent && second_execution.naturally_quiescent,
    }
}

fn pair_trial(namespace: u64, both: bool, resources: &mut Resources) -> (u32, u64, bool) {
    let mut substrate = PlasticSubstrate::new();
    let left = substrate.add_cell(cell(physical(namespace, 10), -2_000, 10, 1));
    let right = substrate.add_cell(cell(physical(namespace, 11), -1_000, 11, 1));
    let coincidence = substrate.add_cell(cell(physical(namespace, 20), 0, 20, 2));
    let source = substrate.add_cell(cell(physical(namespace, 30), 1_000, 30, 1));
    let effect = substrate.add_cell(cell(physical(namespace, 40), 2_000, 40, 1));
    let transmitter = substrate.add_cell(cell(physical(namespace, 50), 3_000, 50, 1));
    substrate.add_arrow(drive(left, coincidence, 0, 1, 8));
    substrate.add_arrow(drive(right, coincidence, 0, 1, 8));
    substrate.add_arrow(drive(coincidence, source, 0, 1, 8));
    let candidate = substrate.add_arrow(drive(source, effect, 0, 1, 1));
    substrate.add_arrow(drive(effect, transmitter, 0, 1, 8));
    substrate.add_arrow(modulatory(transmitter, source, 0, 1, 8));
    let mut updates = 0;
    let mut quiescent = true;
    for tick in [0, 2] {
        pulse(&mut substrate, left, tick, 0);
        if both {
            pulse(&mut substrate, right, tick, 1);
        }
        let execution = substrate.propagate();
        resources.execution(&substrate, &execution);
        updates += execution.work.local_return_updates;
        quiescent &= execution.naturally_quiescent;
    }
    (substrate.arrow_resistance(candidate), updates, quiescent)
}

fn conformance_trial(namespace: u64, resources: &mut Resources) -> ConformanceMeasure {
    let immediate = loop_trial(
        Config {
            namespace,
            load: 0,
            return_tick: 0,
            shape: ReturnShape::Direct,
            reflect: false,
            reverse_arrivals: false,
            returned_side: 0,
        },
        resources,
    );
    let (unparticipated_resistance, _, unparticipated_log) =
        isolated_trial(namespace + 100, "unparticipated", resources);
    let direction_left = direction_trial(namespace + 200, 0, false, resources);
    let direction_right = direction_trial(namespace + 300, 1, true, resources);
    let (single_resistance, single_updates, single_quiescent) =
        pair_trial(namespace + 400, false, resources);
    let (pair_resistance, pair_updates, pair_quiescent) =
        pair_trial(namespace + 500, true, resources);
    ConformanceMeasure {
        px0: immediate.resistance == 4 && immediate.coupling == 2 && immediate.log.updates == 1,
        px1: unparticipated_resistance == 1
            && unparticipated_log.modulatory_deliveries == 1
            && unparticipated_log.updates == 0,
        px2: direction_left.resistance == [4, 0]
            && direction_right.resistance == [0, 4]
            && direction_left.updates == 1
            && direction_right.updates == 1,
        px3: single_resistance == 1
            && single_updates == 0
            && pair_resistance == 7
            && pair_updates == 2,
        quiescent: immediate.log.quiescent
            && unparticipated_log.quiescent
            && direction_left.quiescent
            && direction_right.quiescent
            && single_quiescent
            && pair_quiescent,
    }
}

fn px4_trial(namespace: u64, flip: bool, mirror: bool, resources: &mut Resources) -> bool {
    let mut world = field(namespace, flip, mirror, TransmissionMode::Modulatory);
    arrive(&mut world.space, world.source, 0, 0, namespace + 100);
    arrive(&mut world.space, world.returner, 2, 0, namespace + 101);
    let first = world.space.propagate();
    resources.execution(&world.space, &first);
    let old = world.space.arrows_between(world.source, world.effect)[0];
    let first_resistance = world.space.arrow_resistance(old);
    let bytes_before_reuse = world.space.persistent_bytes();
    let arrows_before_reuse = world.space.arrow_count();

    arrive(&mut world.space, world.source, 5, 0, namespace + 102);
    arrive(&mut world.space, world.returner, 7, 0, namespace + 103);
    let recurrent = world.space.propagate();
    resources.execution(&world.space, &recurrent);
    resources.reuse_stable &= bytes_before_reuse == world.space.persistent_bytes()
        && arrows_before_reuse == world.space.arrow_count();
    let recurrent_resistance = world.space.arrow_resistance(old);

    let pressure = world.space.advance_time(80);
    resources.pressure(&world.space, &pressure);
    let old_dead = !world.space.arrow_is_live(old);
    arrive(&mut world.space, world.source, 80, 0, namespace + 104);
    arrive(&mut world.space, world.returner, 82, 0, namespace + 105);
    let reacquisition = world.space.propagate();
    resources.execution(&world.space, &reacquisition);
    let replacement = world
        .space
        .arrows_between(world.source, world.effect)
        .into_iter()
        .find(|candidate| world.space.arrow_is_live(*candidate))
        .expect("PX4 ordinary reacquisition");

    first_resistance == 4
        && recurrent_resistance == 7
        && recurrent.work.local_structural_proposals == 0
        && old_dead
        && replacement != old
        && world.space.arrow_resistance(replacement) == 4
        && first.naturally_quiescent
        && recurrent.naturally_quiescent
        && reacquisition.naturally_quiescent
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AllocationSite {
    source: CellId,
    effect: CellId,
    returned: bool,
}

fn px5_trial(
    namespace: u64,
    load: usize,
    returned_last: bool,
    reflect: bool,
    resources: &mut Resources,
) -> bool {
    let sign = if reflect { -1 } else { 1 };
    let returned_ordinal = if returned_last { load } else { 0 };
    let mut substrate = PlasticSubstrate::new();
    let mut sites = Vec::with_capacity(load + 1);
    for ordinal in 0..=load {
        let offset = 1_000 + ordinal as u64 * 16;
        let position = sign * ordinal as i32 * 10;
        let source = substrate.add_cell(cell(
            physical(namespace, offset),
            position,
            100 + ordinal as i16,
            1,
        ));
        let effect = substrate.add_cell(cell(
            physical(namespace, offset + 1),
            position + sign,
            500 + ordinal as i16,
            1,
        ));
        let returned = ordinal == returned_ordinal;
        if returned {
            let returner = substrate.add_cell(cell(
                physical(namespace, offset + 2),
                position + 4 * sign,
                900 + ordinal as i16,
                1,
            ));
            substrate.add_arrow(drive(effect, returner, 1, 1, 100));
            substrate.add_arrow(modulatory(returner, source, 1, 1, 100));
        }
        sites.push(AllocationSite {
            source,
            effect,
            returned,
        });
    }

    for (phase, site) in sites.iter().enumerate() {
        arrive(
            &mut substrate,
            site.source,
            0,
            phase as i32,
            physical(namespace, 20_000 + phase as u64),
        );
    }
    let first = substrate.propagate();
    resources.execution(&substrate, &first);
    let candidates = sites
        .iter()
        .map(|site| substrate.arrows_between(site.source, site.effect)[0])
        .collect::<Vec<_>>();
    let returned_index = sites
        .iter()
        .position(|site| site.returned)
        .expect("one returned site");
    let returned_arrow = candidates[returned_index];
    let first_exact = candidates.iter().enumerate().all(|(index, candidate)| {
        substrate.arrow_resistance(*candidate) == if index == returned_index { 4 } else { 1 }
    });

    let pressure = substrate.advance_time(30);
    resources.pressure(&substrate, &pressure);
    let live = candidates
        .iter()
        .filter(|candidate| substrate.arrow_is_live(**candidate))
        .count();
    let bytes_before_reuse = substrate.persistent_bytes();
    let arrows_before_reuse = substrate.arrow_count();
    arrive(
        &mut substrate,
        sites[returned_index].source,
        30,
        0,
        physical(namespace, 30_000),
    );
    let reuse = substrate.propagate();
    resources.execution(&substrate, &reuse);
    resources.reuse_stable &= bytes_before_reuse == substrate.persistent_bytes()
        && arrows_before_reuse == substrate.arrow_count();

    first.work.local_structural_proposals == (load + 1) as u64
        && first.work.local_return_updates == 1
        && first_exact
        && live == 1
        && substrate.arrow_is_live(returned_arrow)
        && substrate.arrow_resistance(returned_arrow) == 4
        && reuse.work.local_structural_proposals == 0
        && reuse.work.local_return_updates == 1
        && first.naturally_quiescent
        && reuse.naturally_quiescent
}

fn core(config: Config) -> CoreObservation {
    let mut resources = Resources::new();
    let positive = loop_trial(config, &mut resources);
    let immediate = loop_trial(
        Config {
            namespace: config.namespace + 10_000,
            load: 0,
            return_tick: 0,
            shape: ReturnShape::Direct,
            ..config
        },
        &mut resources,
    );
    let late = loop_trial(
        Config {
            namespace: config.namespace + 20_000,
            load: 0,
            return_tick: 4,
            shape: ReturnShape::Relayed,
            ..config
        },
        &mut resources,
    );
    let (unparticipated_resistance, unparticipated_eligible, unparticipated_log) =
        isolated_trial(config.namespace + 30_000, "unparticipated", &mut resources);
    let (drive_resistance, drive_eligible, drive_log) =
        drive_while_eligible(config.namespace + 40_000, &mut resources);
    let (blocked_resistance, blocked_eligible, blocked_log) =
        blocked_trial(config.namespace + 50_000, &mut resources);
    let (missing_resistance, missing_eligible, missing_log) =
        isolated_trial(config.namespace + 60_000, "missing", &mut resources);
    let (
        supported_resistance,
        supported_live,
        unsupported_resistance,
        unsupported_live,
        pressure_q,
    ) = pressure_trial(config.namespace + 70_000, &mut resources);
    let paths = path_variation_trial(config.namespace + 80_000, &mut resources);
    let direction = direction_trial(
        config.namespace + 90_000,
        config.returned_side,
        config.reverse_arrivals,
        &mut resources,
    );
    let direction_swapped = direction_trial(
        config.namespace + 100_000,
        1 - config.returned_side,
        !config.reverse_arrivals,
        &mut resources,
    );
    let (multiple_resistance, multiple_updates, multiple_q) =
        multiple_loops_trial(config.namespace + 110_000, 3, &mut resources);
    let lifecycle = lifecycle_trial(config.namespace + 120_000, &mut resources);
    let conformance = conformance_trial(config.namespace + 130_000, &mut resources);
    let px4 = px4_trial(
        config.namespace + 140_000,
        config.reverse_arrivals,
        config.reflect,
        &mut resources,
    );
    let px5 = px5_trial(
        config.namespace + 200_000,
        config.load,
        config.returned_side == 1,
        config.reflect,
        &mut resources,
    );

    let expected_direction = if config.returned_side == 0 {
        [4, 0]
    } else {
        [0, 4]
    };
    let expected_swapped = if config.returned_side == 0 {
        [0, 4]
    } else {
        [4, 0]
    };
    let all_quiescent = positive.log.quiescent
        && immediate.log.quiescent
        && late.log.quiescent
        && unparticipated_log.quiescent
        && drive_log.quiescent
        && blocked_log.quiescent
        && missing_log.quiescent
        && pressure_q
        && paths.quiescent
        && direction.quiescent
        && direction_swapped.quiescent
        && multiple_q
        && lifecycle.quiescent
        && conformance.quiescent
        && resources.quiescent;

    CoreObservation {
        candidate_resistance: positive.resistance,
        candidate_coupling: positive.coupling,
        updates: positive.log.updates,
        modulatory_deliveries: positive.log.modulatory_deliveries,
        drive_deliveries: positive.log.drive_deliveries,
        distractor_crossings: positive.distractor_crossings,
        loop_work: positive.log.work,
        work: resources.work,
        max_bytes: resources.max_bytes,
        fingerprint: positive.fingerprint,
        permanent: positive.permanent,
        modulation_learning: positive.resistance == 4
            && positive.coupling == 2
            && positive.log.updates == 1
            && positive.log.modulatory_deliveries == 1
            && positive.distractor_crossings == config.load,
        modulation_without_participation: unparticipated_resistance == 1
            && !unparticipated_eligible
            && unparticipated_log.modulatory_deliveries == 1
            && unparticipated_log.updates == 0,
        ordinary_drive_while_eligible: drive_resistance == 1
            && drive_eligible
            && drive_log.modulatory_deliveries == 0
            && drive_log.updates == 0,
        blocked_downstream: blocked_resistance == 1
            && blocked_eligible
            && blocked_log.modulatory_deliveries == 0
            && blocked_log.updates == 0,
        missing_downstream: missing_resistance == 1
            && missing_eligible
            && missing_log.modulatory_deliveries == 0
            && missing_log.updates == 0,
        immediate_modulation: immediate.resistance == 4
            && immediate.log.updates == 1
            && immediate.log.modulatory_deliveries == 1,
        late_modulation: late.resistance == 4
            && late.log.updates == 1
            && late.log.modulatory_deliveries == 1,
        stable_and_varying_return: paths.stable_resistance == 7
            && paths.varying_resistance == 7
            && paths.weak_path_removed
            && paths.replacement_crossed,
        physical_side_reversal: direction.resistance == expected_direction
            && direction.live == expected_direction.map(|value| value > 0)
            && direction.updates == 1
            && direction_swapped.resistance == expected_swapped
            && direction_swapped.live == expected_swapped.map(|value| value > 0)
            && direction_swapped.updates == 1,
        multiple_lawful_loops: multiple_resistance == vec![4; 3] && multiple_updates == 3,
        pressure_persistence: supported_resistance == 1
            && supported_live
            && unsupported_resistance == 0
            && !unsupported_live,
        deallocation_reacquisition: lifecycle.initial_dead
            && lifecycle.replacement_live
            && lifecycle.initial_generation != lifecycle.replacement_generation
            && lifecycle.replacement_resistance == 4
            && lifecycle.proposals == 2
            && lifecycle.deallocations == 1
            && lifecycle.updates == 1,
        px0: conformance.px0,
        px1: conformance.px1,
        px2: conformance.px2,
        px3: conformance.px3,
        px4,
        px5,
        reuse_stable: resources.reuse_stable,
        quiescent: all_quiescent,
    }
}

fn registered_config(root: u64, load: usize) -> Config {
    let root_index = ROOTS
        .iter()
        .position(|candidate| *candidate == root)
        .expect("registered authority root");
    let load_index = LOADS
        .iter()
        .position(|candidate| *candidate == load)
        .expect("registered authority load");
    Config {
        namespace: (root << 32) + 0x0600_0000,
        load,
        return_tick: ((root_index + load_index) % 5) as i64,
        shape: if load_index == 1 {
            ReturnShape::Relayed
        } else {
            ReturnShape::Direct
        },
        reflect: root_index >= 4,
        reverse_arrivals: (root_index / 2) % 2 == 1,
        returned_side: root_index % 2,
    }
}

fn matrix_definition() -> Vec<(u64, usize, Config)> {
    ROOTS
        .into_iter()
        .flat_map(|root| {
            LOADS
                .into_iter()
                .map(move |load| (root, load, registered_config(root, load)))
        })
        .collect()
}

fn replay_row(root: u64, load: usize, _permission: AuthorityPermission) -> Row {
    let config = registered_config(root, load);
    let first = core(config);
    let second = core(config);
    let replay = first == second;
    let claims = [
        config == registered_config(root, load)
            && ROOTS.contains(&root)
            && LOADS.contains(&load)
            && config.namespace >= (root << 32)
            && config.namespace < (root << 32) + 0x0700_0000,
        first.modulation_learning,
        first.modulation_without_participation,
        first.ordinary_drive_while_eligible,
        first.blocked_downstream,
        first.missing_downstream,
        first.immediate_modulation,
        first.late_modulation,
        first.stable_and_varying_return,
        first.physical_side_reversal,
        first.multiple_lawful_loops,
        first.pressure_persistence,
        first.deallocation_reacquisition,
        first.px0,
        first.px1,
        first.px2,
        first.px3,
        first.px4,
        first.px5,
        first.work <= WORK_LIMIT,
        first.max_bytes <= BYTE_LIMIT && first.reuse_stable,
        first.quiescent,
        replay,
    ];
    let passed = first.passed() && replay && claims.into_iter().all(|claim| claim);
    Row {
        root,
        load,
        config,
        core: first,
        replay,
        claims,
        passed,
    }
}

fn global_claims(rows: &[Row], inputs_exact: bool) -> [bool; 6] {
    let complete = rows.len() == 24
        && ROOTS
            .iter()
            .all(|root| rows.iter().filter(|row| row.root == *root).count() == 3)
        && LOADS
            .iter()
            .all(|load| rows.iter().filter(|row| row.load == *load).count() == 8);
    let strata = [0, 1].into_iter().all(|side| {
        [false, true].into_iter().all(|reverse| {
            [false, true].into_iter().all(|reflect| {
                rows.iter()
                    .filter(|row| {
                        row.config.returned_side == side
                            && row.config.reverse_arrivals == reverse
                            && row.config.reflect == reflect
                    })
                    .count()
                    == 3
            })
        })
    });
    let shapes_and_ticks = [ReturnShape::Direct, ReturnShape::Relayed]
        .into_iter()
        .all(|shape| rows.iter().any(|row| row.config.shape == shape))
        && (0..=4).all(|tick| rows.iter().any(|row| row.config.return_tick == tick));
    let namespaces = rows.iter().all(|row| {
        row.root >= 661001
            && row.root <= 661008
            && row.config.namespace >= (row.root << 32)
            && row.config.namespace < (row.root << 32) + 0x0700_0000
    });
    let cumulative = rows.iter().all(|row| {
        row.core.px0 && row.core.px1 && row.core.px2 && row.core.px3 && row.core.px4 && row.core.px5
    });
    let bounded = rows.iter().all(|row| {
        row.core.work <= WORK_LIMIT
            && row.core.max_bytes <= BYTE_LIMIT
            && row.core.reuse_stable
            && row.core.quiescent
            && row.replay
    });
    [
        complete,
        strata,
        shapes_and_ticks,
        inputs_exact && namespaces,
        cumulative,
        bounded,
    ]
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "root,load,namespace,return_tick,shape,reflect,reverse_arrivals,returned_side,candidate_resistance,candidate_coupling,updates,modulatory_deliveries,drive_deliveries,distractor_crossings,loop_work,total_work,max_bytes,reuse_stable,fingerprint,permanent,modulation_learning,modulation_without_participation,ordinary_drive_while_eligible,blocked_downstream,missing_downstream,immediate_modulation,late_modulation,stable_and_varying_return,physical_side_reversal,multiple_lawful_loops,pressure_persistence,deallocation_reacquisition,px0,px1,px2,px3,px4,px5,quiescent,replay,claims,passed\n",
    );
    for row in rows {
        let c = &row.core;
        let claims = row
            .claims
            .iter()
            .map(bool::to_string)
            .collect::<Vec<_>>()
            .join("|");
        writeln!(
            output,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            row.root,
            row.config.load,
            row.config.namespace,
            row.config.return_tick,
            row.config.shape.label(),
            row.config.reflect,
            row.config.reverse_arrivals,
            row.config.returned_side,
            c.candidate_resistance,
            c.candidate_coupling,
            c.updates,
            c.modulatory_deliveries,
            c.drive_deliveries,
            c.distractor_crossings,
            c.loop_work,
            c.work,
            c.max_bytes,
            c.reuse_stable,
            c.fingerprint,
            c.permanent,
            c.modulation_learning,
            c.modulation_without_participation,
            c.ordinary_drive_while_eligible,
            c.blocked_downstream,
            c.missing_downstream,
            c.immediate_modulation,
            c.late_modulation,
            c.stable_and_varying_return,
            c.physical_side_reversal,
            c.multiple_lawful_loops,
            c.pressure_persistence,
            c.deallocation_reacquisition,
            c.px0,
            c.px1,
            c.px2,
            c.px3,
            c.px4,
            c.px5,
            c.quiescent,
            row.replay,
            claims,
            row.passed,
        )
        .expect("String writes cannot fail");
    }
    output
}

fn report(rows: &[Row], global: [bool; 6]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let row_clauses = rows
        .iter()
        .map(|row| row.claims.into_iter().filter(|claim| *claim).count())
        .sum::<usize>();
    let global_clauses = global.into_iter().filter(|claim| *claim).count();
    let max_work = rows.iter().map(|row| row.core.work).max().unwrap_or(0);
    let max_bytes = rows.iter().map(|row| row.core.max_bytes).max().unwrap_or(0);
    format!(
        "# PX6 LR-C cumulative physical-consequence authority v1\n\n\
Outcome: **{}**.\n\n\
- rows: `{passed}/24`;\n\
- row clauses: `{row_clauses}/552`;\n\
- global clauses: `{global_clauses}/6`;\n\
- total clauses: `{}/558`;\n\
- exact complete-state replay: `{}`;\n\
- natural quiescence: `{}`;\n\
- maximum complete row work: `{max_work}` / `{WORK_LIMIT}`;\n\
- maximum persistent bytes: `{max_bytes}` / `{BYTE_LIMIT}`;\n\
- repeated-reuse memory stable: `{}`;\n\
- dense loads 8/32/128: `passed`;\n\
- PX0--PX5+LR-C cumulative conformance: `{}`;\n\
- new organism source or law: `false`;\n\
- PX7 executed or advanced: `false`.\n",
        if passed == 24 && row_clauses == 552 && global_clauses == 6 {
            "DEFINITIVE POSITIVE"
        } else {
            "DEFINITIVE NEGATIVE"
        },
        row_clauses + global_clauses,
        rows.iter().all(|row| row.replay),
        rows.iter().all(|row| row.core.quiescent),
        rows.iter().all(|row| row.core.reuse_stable),
        global[4],
    )
}

fn frozen_inputs() -> bool {
    for (path, expected) in [
        ("crates/lr1-modulatory-physical-return/src/lib.rs", LAW_HASH),
        ("arms/px4-lrc-lifetime/src/lib.rs", PX4_SOURCE_HASH),
        (
            "experiments/px5_lrc_cumulative_allocation_authority_handoff_v1.md",
            PX5_HANDOFF_HASH,
        ),
        ("results/px5_lrc_allocation_authority_v1.csv", PX5_CSV_HASH),
        (
            "results/px5_lrc_allocation_authority_v1.md",
            PX5_REPORT_HASH,
        ),
        (
            "experiments/pxc_active_surface_manifest_v3.csv",
            PX5_MANIFEST_HASH,
        ),
        (
            "experiments/px6_lrc_cumulative_consequence_authority_protocol_v1.md",
            PROTOCOL_HASH,
        ),
    ] {
        assert_eq!(sha(path), expected, "frozen input changed: {path}");
    }
    true
}

fn preflight() {
    assert!(frozen_inputs());
    let matrix = matrix_definition();
    assert_eq!(ROOTS.into_iter().collect::<BTreeSet<_>>().len(), 8);
    assert_eq!(LOADS, [8, 32, 128]);
    assert_eq!(matrix.len(), 24);
    assert_eq!(
        matrix
            .iter()
            .map(|(_, _, config)| config.namespace)
            .collect::<BTreeSet<_>>()
            .len(),
        8
    );
    assert!(matrix.iter().all(|(root, load, config)| {
        *root >= 661001
            && *root <= 661008
            && LOADS.contains(load)
            && config.namespace >= (*root << 32)
            && config.namespace < (*root << 32) + 0x0700_0000
    }));
    absent(&[CSV, REPORT]);
    println!("PX6_LRC_CONSEQUENCE_AUTHORITY_PREFLIGHT_OK");
}

fn authority() {
    preflight();
    eprintln!("PX6_LRC_CONSEQUENCE_AUTHORITY_V1_EVIDENCE_SPENT");
    let rows = matrix_definition()
        .into_iter()
        .map(|(root, load, _)| replay_row(root, load, AuthorityPermission::Granted))
        .collect::<Vec<_>>();
    let global = global_claims(&rows, true);
    assert!(rows.iter().all(|row| row.passed));
    assert!(global.into_iter().all(|claim| claim));
    publish(CSV, &csv(&rows));
    publish(REPORT, &report(&rows, global));
    println!("PX6_LRC_CONSEQUENCE_AUTHORITY_V1_COMPLETE rows=24 clauses=558");
}

fn main() {
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [argument] if argument == "--authority-preflight" => preflight(),
        [argument] if argument == "--authority-v1" => authority(),
        _ => std::process::exit(2),
    }
}

fn absent(paths: &[&str]) {
    for path in paths {
        assert!(!Path::new(path).exists(), "result already exists: {path}");
    }
}

fn publish(path: &str, contents: &str) {
    let staging = format!("{path}.staging");
    assert!(
        !Path::new(&staging).exists(),
        "staging path exists: {staging}"
    );
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staging)
        .expect("create staging artifact");
    file.write_all(contents.as_bytes()).expect("write artifact");
    file.sync_all().expect("sync artifact");
    rename(staging, path).expect("publish artifact");
}

fn sha(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .or_else(|_| Command::new("shasum").args(["-a", "256", path]).output())
        .expect("hash command");
    assert!(output.status.success(), "hash failed: {path}");
    String::from_utf8(output.stdout)
        .expect("hash output")
        .split_whitespace()
        .next()
        .expect("hash digest")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authority_matrix_is_fresh_bounded_and_world_free() {
        let matrix = matrix_definition();
        assert_eq!(matrix.len(), 24);
        assert_eq!(ROOTS.into_iter().collect::<BTreeSet<_>>().len(), 8);
        assert_eq!(LOADS, [8, 32, 128]);
        assert!(ROOTS.iter().all(|root| *root >= 661001 && *root <= 661008));
        assert_eq!(WORK_LIMIT, 150_000);
        assert_eq!(BYTE_LIMIT, 32_000);
        assert!(matrix.iter().all(|(root, _, config)| {
            config.namespace >= (*root << 32) && config.namespace < (*root << 32) + 0x0700_0000
        }));
    }
}
