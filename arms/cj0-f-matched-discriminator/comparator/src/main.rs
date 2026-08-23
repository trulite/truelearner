#![forbid(unsafe_code)]

use cj0_f_candidate_b as b;
use cj0_f_candidate_e as e;
use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const B_SHA256: &str = "ef0de37a9ac54b632b991f0d4647a5ee78c23810084d61497c88d6f757ec2188";
const E_SHA256: &str = "e64c8c915c2fbc4679d1e34ee69ecfe36e2c5ff05bdff5d7feeb5a55578bf1c1";
const PROTOCOL_SHA256: &str = "18babd7614aadf0e2a0aa4ea9bc622a4bbff9c3d7ebc0b43c1ba301c088c959a";
const SPIKE_LOWER_BOUND_BYTES: usize = 40;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Probe,
    Micro,
    Gate,
}

impl Stage {
    fn name(self) -> &'static str {
        match self {
            Self::Probe => "probe",
            Self::Micro => "micro",
            Self::Gate => "gate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    SameSource,
    Amplitude,
    DenseReturn,
    Timing,
    Control,
}

impl Family {
    fn name(self) -> &'static str {
        match self {
            Self::SameSource => "same_source_bursts",
            Self::Amplitude => "amplitude_vs_multiplicity",
            Self::DenseReturn => "dense_return_topology",
            Self::Timing => "timing_transfer",
            Self::Control => "shared_control",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    DirectStrong,
    DirectTwoDistinct,
    DirectTwoSame,
    DirectAsymmetric,
    DirectAaSpacing,
    OnePathBurst,
    BurstFour,
    GatedDistinct,
    GatedSingleton,
    GatedReuse,
    DenseDirectReturn,
    DenseCrossedReturn,
    DenseNoReturn,
    DenseLateReturn,
    StaleQueued,
    DeallocateBootstrap,
    TimingTransfer,
    ContemporaryReversal,
    Recursion,
    FourWayAmbiguity,
    ExactReplay,
}

#[derive(Clone, Debug)]
struct RowSpec {
    row_id: String,
    stage: Stage,
    family: Family,
    kind: Kind,
    seed: u64,
    mirror: bool,
    reverse_insertion: bool,
    threshold: i32,
    coupling: i32,
    load: usize,
    spacing: i64,
    train_spacing: i64,
    allocation: usize,
    expected_effect: bool,
    genuine: bool,
}

impl RowSpec {
    fn physical_serialization(&self) -> String {
        format!(
            "{}|{}|{}|{:?}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.row_id,
            self.stage.name(),
            self.family.name(),
            self.kind,
            self.seed,
            self.mirror,
            self.reverse_insertion,
            self.threshold,
            self.coupling,
            self.load,
            self.spacing,
            self.train_spacing,
            self.allocation,
            self.expected_effect,
            self.genuine
        )
    }

    fn spec_fingerprint(&self) -> u64 {
        fingerprint(self.physical_serialization().as_bytes())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ArrowState {
    records: usize,
    live: usize,
    resistance_max: u32,
    coupling_max: i32,
    generation_max: u32,
}

#[derive(Clone, Debug, Default)]
struct RunMetrics {
    learning_work: u64,
    execution_work: u64,
    queue_work: u64,
    delivered: u64,
    generation_checks: u64,
    proposals: u64,
    deallocations: u64,
    return_updates: u64,
    eligibility_writes: u64,
    pressure_updates: u64,
    locus_firings: usize,
    effect_firings: usize,
    transmissions: Option<usize>,
    crossings: usize,
    persistent_bytes: usize,
    arrow_records: usize,
    live_structures: usize,
    temporary_peak_count: usize,
    temporary_peak_bytes_lower_bound: usize,
    resistance_start: u32,
    resistance_end: u32,
    coupling_start: i32,
    coupling_end: i32,
    reversal_cost: u64,
    deallocation_cost: u64,
    reacquisition_cost: u64,
    timing_tolerance: i64,
    dense_attribution_ok: bool,
    recursion_depth: usize,
    replay_equal: bool,
    naturally_quiescent: bool,
    runaway: bool,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    false_conjunction: bool,
    heldout_reuse: bool,
    row_pass: bool,
}

#[derive(Clone, Debug, Default)]
struct NativeRun {
    work_total: u64,
    queue_work: u64,
    delivered: u64,
    generation_checks: u64,
    proposals: u64,
    deallocations: u64,
    return_updates: u64,
    eligibility_writes: u64,
    pressure_updates: u64,
    fired_physical: Vec<u64>,
    transmissions: Option<usize>,
    crossings: usize,
    naturally_quiescent: bool,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
}

trait Law: Default {
    fn add_cell(
        &mut self,
        physical_id: u64,
        position: i32,
        region: i16,
        threshold: i32,
        resistance: u32,
    ) -> usize;
    fn add_arrow(
        &mut self,
        from: usize,
        to: usize,
        delay: i64,
        phase: i32,
        coupling: i32,
        resistance: u32,
    );
    fn enter(&mut self, tick: i64, phase: i32, origin_physical: u64, target: usize, impulse: i32);
    fn propagate(&mut self) -> NativeRun;
    fn advance_time(&mut self, tick: i64) -> NativeRun;
    fn arrow_state(&self, from: usize, to: usize) -> ArrowState;
    fn persistent_bytes(&self) -> usize;
    fn arrow_count(&self) -> usize;
    fn complete_fingerprint(&self) -> u64;
    fn permanent_fingerprint(&self) -> u64;
}

#[derive(Default)]
struct LawB {
    substrate: b::Substrate,
    cells: Vec<b::CellId>,
}

impl Law for LawB {
    fn add_cell(
        &mut self,
        physical_id: u64,
        position: i32,
        region: i16,
        threshold: i32,
        resistance: u32,
    ) -> usize {
        let id = self.substrate.add_cell(b::CellSpec {
            physical_id,
            position,
            region,
            threshold,
            resistance,
        });
        self.cells.push(id);
        self.cells.len() - 1
    }

    fn add_arrow(
        &mut self,
        from: usize,
        to: usize,
        delay: i64,
        phase: i32,
        coupling: i32,
        resistance: u32,
    ) {
        self.substrate.add_arrow(b::ArrowSpec {
            from: self.cells[from],
            to: self.cells[to],
            delay,
            phase,
            coupling,
            resistance,
        });
    }

    fn enter(&mut self, tick: i64, phase: i32, origin_physical: u64, target: usize, impulse: i32) {
        self.substrate.enter(b::SpikeInput {
            arrival_tick: tick,
            phase,
            origin_physical,
            target: self.cells[target],
            impulse,
        });
    }

    fn propagate(&mut self) -> NativeRun {
        let run = self.substrate.propagate();
        NativeRun {
            work_total: run.work.total(),
            queue_work: run.work.queue_comparisons,
            delivered: run.work.spikes_delivered,
            generation_checks: run.work.generation_checks,
            proposals: run.work.local_structural_proposals,
            deallocations: run.work.physical_deallocations,
            return_updates: run.work.local_return_updates,
            eligibility_writes: run.work.local_eligibility_writes,
            pressure_updates: run.work.ordinary_pressure_updates,
            fired_physical: run
                .trace
                .iter()
                .filter_map(|entry| entry.fired.then_some(entry.target_physical))
                .collect(),
            transmissions: Some(run.transmissions.iter().filter(|item| item.emitted).count()),
            crossings: run.crossings.len(),
            naturally_quiescent: run.naturally_quiescent,
            complete_fingerprint: run.end_fingerprint,
            permanent_fingerprint: run.permanent_fingerprint,
        }
    }

    fn advance_time(&mut self, tick: i64) -> NativeRun {
        let work = self.substrate.advance_time(tick);
        NativeRun {
            work_total: work.total(),
            queue_work: work.queue_comparisons,
            delivered: work.spikes_delivered,
            generation_checks: work.generation_checks,
            proposals: work.local_structural_proposals,
            deallocations: work.physical_deallocations,
            return_updates: work.local_return_updates,
            eligibility_writes: work.local_eligibility_writes,
            pressure_updates: work.ordinary_pressure_updates,
            naturally_quiescent: true,
            complete_fingerprint: self.substrate.complete_fingerprint(),
            permanent_fingerprint: self.substrate.permanent_fingerprint(),
            transmissions: Some(0),
            ..NativeRun::default()
        }
    }

    fn arrow_state(&self, from: usize, to: usize) -> ArrowState {
        let state = self
            .substrate
            .arrows_between(self.cells[from], self.cells[to]);
        ArrowState {
            records: state.records,
            live: state.live,
            resistance_max: state.resistance_max,
            coupling_max: state.coupling_max,
            generation_max: state.generation_max,
        }
    }

    fn persistent_bytes(&self) -> usize {
        self.substrate.persistent_bytes()
    }

    fn arrow_count(&self) -> usize {
        self.substrate.arrow_count()
    }

    fn complete_fingerprint(&self) -> u64 {
        self.substrate.complete_fingerprint()
    }

    fn permanent_fingerprint(&self) -> u64 {
        self.substrate.permanent_fingerprint()
    }
}

#[derive(Default)]
struct LawE {
    substrate: e::PlasticSubstrate,
    cells: Vec<e::CellId>,
}

impl Law for LawE {
    fn add_cell(
        &mut self,
        physical_id: u64,
        position: i32,
        region: i16,
        threshold: i32,
        resistance: u32,
    ) -> usize {
        let id = self.substrate.add_cell(e::CellSpec {
            physical_id,
            position,
            region,
            threshold,
            resistance,
        });
        self.cells.push(id);
        self.cells.len() - 1
    }

    fn add_arrow(
        &mut self,
        from: usize,
        to: usize,
        delay: i64,
        phase: i32,
        coupling: i32,
        resistance: u32,
    ) {
        self.substrate.add_arrow(e::ArrowSpec {
            from: self.cells[from],
            to: self.cells[to],
            delay,
            phase,
            coupling,
            resistance,
        });
    }

    fn enter(&mut self, tick: i64, phase: i32, origin_physical: u64, target: usize, impulse: i32) {
        self.substrate.enter(e::SpikeInput {
            arrival_tick: tick,
            phase,
            origin_physical,
            target: self.cells[target],
            impulse,
        });
    }

    fn propagate(&mut self) -> NativeRun {
        let run = self.substrate.propagate();
        NativeRun {
            work_total: run.work.total(),
            queue_work: run.work.queue_comparisons,
            delivered: run.work.spikes_delivered,
            generation_checks: run.work.generation_checks,
            proposals: run.work.local_structural_proposals,
            deallocations: run.work.physical_deallocations,
            return_updates: run.work.local_return_updates,
            eligibility_writes: run.work.local_eligibility_writes,
            pressure_updates: run.work.ordinary_pressure_updates,
            fired_physical: run
                .trace
                .iter()
                .filter_map(|entry| entry.fired.then_some(entry.target_physical))
                .collect(),
            transmissions: None,
            crossings: run.crossings.len(),
            naturally_quiescent: run.naturally_quiescent,
            complete_fingerprint: run.end_fingerprint,
            permanent_fingerprint: run.permanent_fingerprint,
        }
    }

    fn advance_time(&mut self, tick: i64) -> NativeRun {
        let work = self.substrate.advance_time(tick);
        NativeRun {
            work_total: work.total(),
            queue_work: work.queue_comparisons,
            delivered: work.spikes_delivered,
            generation_checks: work.generation_checks,
            proposals: work.local_structural_proposals,
            deallocations: work.physical_deallocations,
            return_updates: work.local_return_updates,
            eligibility_writes: work.local_eligibility_writes,
            pressure_updates: work.ordinary_pressure_updates,
            naturally_quiescent: true,
            complete_fingerprint: self.substrate.complete_fingerprint(),
            permanent_fingerprint: self.substrate.permanent_fingerprint(),
            transmissions: None,
            ..NativeRun::default()
        }
    }

    fn arrow_state(&self, from: usize, to: usize) -> ArrowState {
        let arrows = self
            .substrate
            .arrows_between(self.cells[from], self.cells[to]);
        ArrowState {
            records: arrows.len(),
            live: arrows
                .iter()
                .filter(|arrow| self.substrate.arrow_is_live(**arrow))
                .count(),
            resistance_max: arrows
                .iter()
                .filter(|arrow| self.substrate.arrow_is_live(**arrow))
                .map(|arrow| self.substrate.arrow_resistance(*arrow))
                .max()
                .unwrap_or(0),
            coupling_max: arrows
                .iter()
                .filter(|arrow| self.substrate.arrow_is_live(**arrow))
                .map(|arrow| self.substrate.arrow_coupling(*arrow))
                .max()
                .unwrap_or(0),
            generation_max: arrows
                .iter()
                .map(|arrow| self.substrate.arrow_generation(*arrow))
                .max()
                .unwrap_or(0),
        }
    }

    fn persistent_bytes(&self) -> usize {
        self.substrate.persistent_bytes()
    }

    fn arrow_count(&self) -> usize {
        self.substrate.arrow_count()
    }

    fn complete_fingerprint(&self) -> u64 {
        self.substrate.complete_fingerprint()
    }

    fn permanent_fingerprint(&self) -> u64 {
        self.substrate.permanent_fingerprint()
    }
}

fn absorb(metrics: &mut RunMetrics, run: &NativeRun, learning: bool) {
    if learning {
        metrics.learning_work = metrics.learning_work.saturating_add(run.work_total);
    } else {
        metrics.execution_work = metrics.execution_work.saturating_add(run.work_total);
    }
    metrics.queue_work = metrics.queue_work.saturating_add(run.queue_work);
    metrics.delivered = metrics.delivered.saturating_add(run.delivered);
    metrics.generation_checks = metrics
        .generation_checks
        .saturating_add(run.generation_checks);
    metrics.proposals = metrics.proposals.saturating_add(run.proposals);
    metrics.deallocations = metrics.deallocations.saturating_add(run.deallocations);
    metrics.return_updates = metrics.return_updates.saturating_add(run.return_updates);
    metrics.eligibility_writes = metrics
        .eligibility_writes
        .saturating_add(run.eligibility_writes);
    metrics.pressure_updates = metrics
        .pressure_updates
        .saturating_add(run.pressure_updates);
    metrics.crossings = metrics.crossings.saturating_add(run.crossings);
    metrics.transmissions = match (metrics.transmissions, run.transmissions) {
        (Some(left), Some(right)) => Some(left.saturating_add(right)),
        (None, Some(right)) if metrics.learning_work + metrics.execution_work == run.work_total => {
            Some(right)
        }
        _ => None,
    };
    metrics.naturally_quiescent &= run.naturally_quiescent;
    metrics.complete_fingerprint = run.complete_fingerprint;
    metrics.permanent_fingerprint = run.permanent_fingerprint;
}

fn add_cell<L: Law>(
    law: &mut L,
    base: u64,
    offset: u64,
    position: i32,
    mirror: bool,
    region: i16,
    threshold: i32,
) -> usize {
    law.add_cell(
        base + offset,
        if mirror { -position } else { position },
        region,
        threshold,
        1_000,
    )
}

fn finish_metrics<L: Law>(law: &L, metrics: &mut RunMetrics) {
    metrics.persistent_bytes = law.persistent_bytes();
    metrics.arrow_records = law.arrow_count();
    metrics.temporary_peak_bytes_lower_bound = metrics
        .temporary_peak_count
        .saturating_mul(SPIKE_LOWER_BOUND_BYTES);
    metrics.complete_fingerprint = law.complete_fingerprint();
    metrics.permanent_fingerprint = law.permanent_fingerprint();
    metrics.runaway = !metrics.naturally_quiescent || metrics.delivered > 10_000;
}

fn run_direct<L: Law>(spec: &RowSpec) -> RunMetrics {
    let mut law = L::default();
    let base = spec.seed.saturating_mul(10_000);
    let target = add_cell(&mut law, base, 1, 100, spec.mirror, 0, spec.threshold);
    let effect = add_cell(&mut law, base, 2, 200, spec.mirror, 1, 1);
    law.add_arrow(target, effect, 0, 9, 1, 30);
    for index in 0..spec.load {
        let _ = add_cell(
            &mut law,
            base,
            100 + index as u64,
            500 + index as i32 * 10,
            spec.mirror,
            0,
            1,
        );
    }

    let (first, second) = (spec.threshold / 2, spec.threshold - spec.threshold / 2);
    let mut entries = Vec::new();
    match spec.kind {
        Kind::DirectStrong => entries.push((0, 0, base + 500, spec.threshold)),
        Kind::DirectTwoDistinct => {
            entries.push((0, 0, base + 501, first));
            entries.push((0, 1, base + 502, second));
        }
        Kind::DirectTwoSame => {
            entries.push((0, 0, base + 501, first));
            entries.push((0, 1, base + 501, second));
        }
        Kind::DirectAsymmetric => {
            entries.push((0, 0, base + 501, 1));
            entries.push((0, 1, base + 502, spec.threshold - 1));
        }
        Kind::DirectAaSpacing => {
            let spacing = spec.spacing.unsigned_abs() as i64;
            entries.push((0, 0, base + 501, first));
            entries.push((spacing, 1, base + 501, second));
        }
        _ => unreachable!("direct runner received non-direct row"),
    }
    for (tick, phase, origin, impulse) in &entries {
        law.enter(*tick, *phase, *origin, target, *impulse);
    }
    let run = law.propagate();
    let mut metrics = RunMetrics {
        naturally_quiescent: true,
        temporary_peak_count: entries.len(),
        ..RunMetrics::default()
    };
    metrics.locus_firings = run
        .fired_physical
        .iter()
        .filter(|physical| **physical == base + 1)
        .count();
    metrics.effect_firings = run
        .fired_physical
        .iter()
        .filter(|physical| **physical == base + 2)
        .count();
    absorb(&mut metrics, &run, false);
    metrics.false_conjunction = metrics.effect_firings > 0 && !spec.genuine;
    metrics.row_pass = (metrics.effect_firings > 0) == spec.expected_effect;
    finish_metrics(&law, &mut metrics);
    metrics
}

fn run_burst_four<L: Law>(spec: &RowSpec) -> RunMetrics {
    let (mut law, trigger, _target, _effect, base) = build_gated::<L>(spec);
    for tick in 0..4 {
        law.enter(tick, 0, base + 602, trigger, 1);
    }
    let run = law.propagate();
    let mut metrics = RunMetrics {
        naturally_quiescent: true,
        temporary_peak_count: 4,
        locus_firings: run
            .fired_physical
            .iter()
            .filter(|physical| **physical == base + 2)
            .count(),
        effect_firings: run
            .fired_physical
            .iter()
            .filter(|physical| **physical == base + 3)
            .count(),
        ..RunMetrics::default()
    };
    absorb(&mut metrics, &run, false);
    metrics.false_conjunction = metrics.effect_firings > 0;
    metrics.row_pass = metrics.effect_firings == 0;
    finish_metrics(&law, &mut metrics);
    metrics
}

fn build_gated<L: Law>(spec: &RowSpec) -> (L, usize, usize, usize, u64) {
    let mut law = L::default();
    let base = spec.seed.saturating_mul(10_000);
    let trigger = add_cell(&mut law, base, 1, 0, spec.mirror, 0, 1);
    let target = add_cell(&mut law, base, 2, 50, spec.mirror, 0, spec.threshold);
    let effect = add_cell(&mut law, base, 3, 100, spec.mirror, 1, 1);
    law.add_arrow(trigger, target, 0, 4, spec.coupling, 30);
    law.add_arrow(target, effect, 0, 8, 1, 30);
    (law, trigger, target, effect, base)
}

fn enter_genuine<L: Law>(
    law: &mut L,
    trigger: usize,
    target: usize,
    base: u64,
    timing_and_matter: (i64, i32, i32, i64),
) -> usize {
    let (base_tick, threshold, coupling, spacing) = timing_and_matter;
    let prime = (threshold - coupling).max(1);
    let trigger_tick = base_tick.saturating_add(spacing.max(0));
    let prime_tick = base_tick.saturating_add((-spacing).max(0));
    law.enter(prime_tick, 0, base + 601, target, prime);
    law.enter(trigger_tick, 1, base + 602, trigger, 1);
    2
}

fn run_gated<L: Law>(spec: &RowSpec) -> RunMetrics {
    let (mut law, trigger, target, _effect, base) = build_gated::<L>(spec);
    let route_start = law.arrow_state(trigger, target);
    let peak = match spec.kind {
        Kind::GatedSingleton => {
            law.enter(0, 1, base + 602, trigger, 1);
            1
        }
        Kind::OnePathBurst => {
            law.enter(0, 0, base + 602, trigger, 1);
            law.enter(
                spec.spacing.unsigned_abs() as i64,
                1,
                base + 602,
                trigger,
                1,
            );
            2
        }
        Kind::GatedDistinct | Kind::GatedReuse => enter_genuine(
            &mut law,
            trigger,
            target,
            base,
            (0, spec.threshold, spec.coupling, spec.spacing),
        ),
        _ => unreachable!("gated runner received wrong row"),
    };
    let first = law.propagate();
    let first_effects = first
        .fired_physical
        .iter()
        .filter(|physical| **physical == base + 3)
        .count();
    let mut metrics = RunMetrics {
        naturally_quiescent: true,
        temporary_peak_count: peak,
        resistance_start: route_start.resistance_max,
        coupling_start: route_start.coupling_max,
        ..RunMetrics::default()
    };
    metrics.locus_firings = first
        .fired_physical
        .iter()
        .filter(|physical| **physical == base + 2)
        .count();
    metrics.effect_firings = first_effects;
    absorb(&mut metrics, &first, spec.kind == Kind::GatedReuse);

    if spec.kind == Kind::GatedReuse {
        law.enter(1, 0, base + 603, trigger, 1);
        let returned = law.propagate();
        absorb(&mut metrics, &returned, true);
        let _ = law.advance_time(2);
        let entered = enter_genuine(
            &mut law,
            trigger,
            target,
            base,
            (2, spec.threshold, spec.coupling, 0),
        );
        metrics.temporary_peak_count = metrics.temporary_peak_count.max(entered);
        let heldout = law.propagate();
        let heldout_effect = heldout
            .fired_physical
            .iter()
            .any(|physical| *physical == base + 3);
        metrics.heldout_reuse = heldout_effect;
        metrics.effect_firings += usize::from(heldout_effect);
        absorb(&mut metrics, &heldout, false);
    }
    let route_end = law.arrow_state(trigger, target);
    metrics.resistance_end = route_end.resistance_max;
    metrics.coupling_end = route_end.coupling_max;
    metrics.live_structures = route_end.live;
    metrics.false_conjunction = first_effects > 0 && !spec.genuine;
    metrics.timing_tolerance = i64::from(spec.spacing == 0 && first_effects > 0);
    metrics.row_pass = (first_effects > 0) == spec.expected_effect
        && (spec.kind != Kind::GatedReuse || metrics.heldout_reuse);
    finish_metrics(&law, &mut metrics);
    metrics
}

fn run_dense<L: Law>(spec: &RowSpec) -> RunMetrics {
    let mut law = L::default();
    let base = spec.seed.saturating_mul(10_000);
    let target = add_cell(&mut law, base, 1, 100, spec.mirror, 0, spec.threshold);
    let effect = add_cell(&mut law, base, 2, 200, spec.mirror, 1, 1);
    law.add_arrow(target, effect, 0, 20, 1, 30);
    let mut sources = Vec::new();
    for index in 0..spec.allocation {
        sources.push(add_cell(
            &mut law,
            base,
            10 + index as u64,
            index as i32 * 20,
            spec.mirror,
            0,
            1,
        ));
    }
    let order: Vec<usize> = if spec.reverse_insertion {
        (0..sources.len()).rev().collect()
    } else {
        (0..sources.len()).collect()
    };
    for index in order {
        law.add_arrow(sources[index], target, 0, 4, spec.coupling, 3);
    }
    for index in 0..spec.load {
        let distractor = add_cell(
            &mut law,
            base,
            100 + index as u64,
            500 + index as i32 * 10,
            spec.mirror,
            0,
            1,
        );
        law.enter(
            0,
            30 + index as i32,
            base + 900 + index as u64,
            distractor,
            1,
        );
    }
    let selected = sources[0];
    let other = sources[1.min(sources.len() - 1)];
    let selected_start = law.arrow_state(selected, target);
    let other_start = law.arrow_state(other, target);
    law.enter(
        0,
        0,
        base + 601,
        target,
        (spec.threshold - spec.coupling).max(1),
    );
    law.enter(0, 1, base + 602, selected, 1);
    let first = law.propagate();
    let effect_count = first
        .fired_physical
        .iter()
        .filter(|physical| **physical == base + 2)
        .count();
    let mut metrics = RunMetrics {
        naturally_quiescent: true,
        temporary_peak_count: spec.load + 2,
        resistance_start: selected_start.resistance_max,
        coupling_start: selected_start.coupling_max,
        effect_firings: effect_count,
        locus_firings: first
            .fired_physical
            .iter()
            .filter(|physical| **physical == base + 1)
            .count(),
        ..RunMetrics::default()
    };
    absorb(&mut metrics, &first, true);
    match spec.kind {
        Kind::DenseDirectReturn => law.enter(1, 0, base + 701, selected, 1),
        Kind::DenseCrossedReturn => law.enter(1, 0, base + 702, other, 1),
        Kind::DenseNoReturn | Kind::DenseLateReturn => {}
        _ => unreachable!("dense runner received wrong row"),
    }
    if matches!(
        spec.kind,
        Kind::DenseDirectReturn | Kind::DenseCrossedReturn
    ) {
        let returned = law.propagate();
        absorb(&mut metrics, &returned, false);
    }
    let close = law.advance_time(5);
    absorb(&mut metrics, &close, false);
    if spec.kind == Kind::DenseLateReturn {
        law.enter(6, 0, base + 703, selected, 1);
        let late = law.propagate();
        absorb(&mut metrics, &late, false);
    }
    let selected_end = law.arrow_state(selected, target);
    let other_end = law.arrow_state(other, target);
    metrics.resistance_end = selected_end.resistance_max;
    metrics.coupling_end = selected_end.coupling_max;
    metrics.live_structures = selected_end.live + other_end.live;
    let selected_strengthened = selected_end.resistance_max > selected_start.resistance_max;
    let other_strengthened = other_end.resistance_max > other_start.resistance_max;
    metrics.dense_attribution_ok = match spec.kind {
        Kind::DenseDirectReturn => selected_strengthened && !other_strengthened,
        Kind::DenseCrossedReturn | Kind::DenseNoReturn | Kind::DenseLateReturn => {
            !selected_strengthened && !other_strengthened
        }
        _ => false,
    };
    metrics.row_pass =
        (metrics.effect_firings > 0) == spec.expected_effect && metrics.dense_attribution_ok;
    finish_metrics(&law, &mut metrics);
    metrics
}

fn run_stale<L: Law>(spec: &RowSpec) -> RunMetrics {
    let mut law = L::default();
    let base = spec.seed.saturating_mul(10_000);
    let source = add_cell(&mut law, base, 1, 0, spec.mirror, 0, 1);
    let target = add_cell(&mut law, base, 2, 100, spec.mirror, 1, spec.threshold);
    law.add_arrow(source, target, 10, 2, spec.coupling, 1);
    let start = law.arrow_state(source, target);
    law.enter(
        0,
        0,
        base + 601,
        target,
        (spec.threshold - spec.coupling).max(1),
    );
    law.enter(0, 1, base + 602, source, 1);
    let run = law.propagate();
    let end = law.arrow_state(source, target);
    let mut metrics = RunMetrics {
        naturally_quiescent: true,
        temporary_peak_count: 2,
        resistance_start: start.resistance_max,
        resistance_end: end.resistance_max,
        coupling_start: start.coupling_max,
        coupling_end: end.coupling_max,
        locus_firings: run
            .fired_physical
            .iter()
            .filter(|physical| **physical == base + 2)
            .count(),
        effect_firings: 0,
        live_structures: end.live,
        ..RunMetrics::default()
    };
    absorb(&mut metrics, &run, false);
    metrics.row_pass = metrics.locus_firings == 0 && end.live == 0 && run.generation_checks >= 3;
    finish_metrics(&law, &mut metrics);
    metrics
}

fn run_deallocate<L: Law>(spec: &RowSpec) -> RunMetrics {
    let mut law = L::default();
    let base = spec.seed.saturating_mul(10_000);
    let source = add_cell(&mut law, base, 1, 0, spec.mirror, 0, 1);
    let target = add_cell(&mut law, base, 2, 1, spec.mirror, 0, spec.threshold);
    law.add_arrow(source, target, 0, 2, 1, 1);
    let start = law.arrow_state(source, target);
    let mut metrics = RunMetrics {
        naturally_quiescent: true,
        resistance_start: start.resistance_max,
        coupling_start: start.coupling_max,
        temporary_peak_count: 1,
        ..RunMetrics::default()
    };
    let mut cost = 0;
    for tick in [8, 10, 11, 13, 14, 16] {
        let run = law.advance_time(tick);
        cost += run.work_total;
        absorb(&mut metrics, &run, true);
    }
    let dead = law.arrow_state(source, target);
    metrics.deallocation_cost = cost;
    law.enter(16, 0, base + 601, source, 1);
    let bootstrap = law.propagate();
    metrics.reacquisition_cost = bootstrap.work_total;
    absorb(&mut metrics, &bootstrap, false);
    let end = law.arrow_state(source, target);
    metrics.resistance_end = end.resistance_max;
    metrics.coupling_end = end.coupling_max;
    metrics.live_structures = end.live;
    metrics.row_pass = dead.live == 0 && end.live == 1 && bootstrap.proposals == 1;
    finish_metrics(&law, &mut metrics);
    metrics
}

fn run_timing_transfer<L: Law>(spec: &RowSpec) -> RunMetrics {
    let (mut law, trigger, target, _effect, base) = build_gated::<L>(spec);
    enter_genuine(
        &mut law,
        trigger,
        target,
        base,
        (0, spec.threshold, spec.coupling, spec.train_spacing),
    );
    let training = law.propagate();
    let training_effect = training
        .fired_physical
        .iter()
        .filter(|physical| **physical == base + 3)
        .count();
    let mut metrics = RunMetrics {
        naturally_quiescent: true,
        temporary_peak_count: 2,
        locus_firings: training_effect,
        ..RunMetrics::default()
    };
    absorb(&mut metrics, &training, true);
    let return_tick = spec.train_spacing.max(0).saturating_add(1);
    law.enter(return_tick, 0, base + 701, trigger, 1);
    let returned = law.propagate();
    absorb(&mut metrics, &returned, true);
    let closed = law.advance_time(10);
    absorb(&mut metrics, &closed, true);
    enter_genuine(
        &mut law,
        trigger,
        target,
        base,
        (10, spec.threshold, spec.coupling, spec.spacing),
    );
    let heldout = law.propagate();
    metrics.effect_firings = heldout
        .fired_physical
        .iter()
        .filter(|physical| **physical == base + 3)
        .count();
    metrics.heldout_reuse = metrics.effect_firings > 0;
    metrics.timing_tolerance = i64::from(metrics.heldout_reuse);
    absorb(&mut metrics, &heldout, false);
    metrics.row_pass = metrics.naturally_quiescent;
    finish_metrics(&law, &mut metrics);
    metrics
}

fn run_reversal<L: Law>(spec: &RowSpec) -> RunMetrics {
    let mut law = L::default();
    let base = spec.seed.saturating_mul(10_000);
    let a = add_cell(&mut law, base, 1, 0, spec.mirror, 0, 1);
    let d = add_cell(&mut law, base, 2, 1, spec.mirror, 0, spec.threshold);
    let c = add_cell(&mut law, base, 3, 10, spec.mirror, 0, 1);
    let b_cell = add_cell(&mut law, base, 4, 11, spec.mirror, 0, spec.threshold);
    let b_effect = add_cell(&mut law, base, 5, 100, spec.mirror, 1, 1);
    let d_effect = add_cell(&mut law, base, 6, 200, spec.mirror, 1, 1);
    let clock = add_cell(&mut law, base, 7, 400, spec.mirror, 0, 1);
    law.add_arrow(a, b_cell, 0, 4, spec.coupling, 1);
    law.add_arrow(c, d, 0, 4, spec.coupling, 1);
    law.add_arrow(d, a, 0, 9, 0, 100);
    law.add_arrow(b_cell, c, 0, 9, 0, 100);
    law.add_arrow(b_cell, b_effect, 0, 8, 1, 30);
    law.add_arrow(d, d_effect, 0, 8, 1, 30);
    let old_ab_start = law.arrow_state(a, b_cell);
    let old_cd_start = law.arrow_state(c, d);
    let prime = (spec.threshold - spec.coupling).max(1);
    law.enter(0, 0, base + 601, b_cell, prime);
    law.enter(0, 0, base + 602, d, prime);
    law.enter(0, 1, base + 603, a, 1);
    law.enter(0, 1, base + 604, c, 1);
    let initial = law.propagate();
    let initial_effects = initial
        .fired_physical
        .iter()
        .filter(|physical| **physical == base + 5 || **physical == base + 6)
        .count();
    let mut metrics = RunMetrics {
        naturally_quiescent: true,
        temporary_peak_count: 4,
        resistance_start: old_ab_start.resistance_max.max(old_cd_start.resistance_max),
        coupling_start: old_ab_start.coupling_max.max(old_cd_start.coupling_max),
        locus_firings: initial_effects,
        ..RunMetrics::default()
    };
    absorb(&mut metrics, &initial, true);
    let deallocated = law.advance_time(5);
    metrics.deallocation_cost = deallocated.work_total;
    absorb(&mut metrics, &deallocated, true);
    let old_ab_dead = law.arrow_state(a, b_cell);
    let old_cd_dead = law.arrow_state(c, d);

    law.enter(5, 0, base + 605, a, 1);
    law.enter(5, 0, base + 606, c, 1);
    law.enter(6, 20, base + 699, clock, 0);
    let bootstrap = law.propagate();
    metrics.reacquisition_cost = bootstrap.work_total;
    absorb(&mut metrics, &bootstrap, true);
    let new_ad_bootstrap = law.arrow_state(a, d);
    let new_cb_bootstrap = law.arrow_state(c, b_cell);

    law.enter(7, 0, base + 607, d, prime);
    law.enter(7, 0, base + 608, b_cell, prime);
    law.enter(7, 1, base + 609, a, 1);
    law.enter(7, 1, base + 610, c, 1);
    let changed = law.propagate();
    absorb(&mut metrics, &changed, true);
    law.enter(9, 0, base + 611, a, 1);
    law.enter(9, 0, base + 612, c, 1);
    let returned = law.propagate();
    absorb(&mut metrics, &returned, true);

    law.enter(11, 0, base + 613, d, prime);
    law.enter(11, 0, base + 614, b_cell, prime);
    law.enter(11, 1, base + 615, a, 1);
    law.enter(11, 1, base + 616, c, 1);
    let heldout = law.propagate();
    let heldout_effects = heldout
        .fired_physical
        .iter()
        .filter(|physical| **physical == base + 5 || **physical == base + 6)
        .count();
    metrics.effect_firings = heldout_effects;
    metrics.heldout_reuse = heldout_effects == 2;
    absorb(&mut metrics, &heldout, false);
    let reversal_before_pressure = metrics.learning_work + metrics.execution_work;
    // Reversal activity itself observes ticks 8, 10, and 11; the remaining
    // fixed pressure-boundary observations follow held-out completion.
    for tick in [13, 14, 16] {
        let observed = law.advance_time(tick);
        absorb(&mut metrics, &observed, false);
    }
    metrics.reversal_cost = reversal_before_pressure;
    let new_ad_end = law.arrow_state(a, d);
    let new_cb_end = law.arrow_state(c, b_cell);
    metrics.resistance_end = new_ad_end.resistance_max.max(new_cb_end.resistance_max);
    metrics.coupling_end = new_ad_end.coupling_max.max(new_cb_end.coupling_max);
    metrics.live_structures = new_ad_end.live + new_cb_end.live;
    metrics.row_pass = initial_effects == 2
        && old_ab_dead.live == 0
        && old_cd_dead.live == 0
        && new_ad_bootstrap.live == 1
        && new_cb_bootstrap.live == 1
        && metrics.heldout_reuse
        && metrics.naturally_quiescent;
    finish_metrics(&law, &mut metrics);
    metrics
}

fn run_recursion<L: Law>(spec: &RowSpec) -> RunMetrics {
    let mut law = L::default();
    let base = spec.seed.saturating_mul(10_000);
    let trigger = add_cell(&mut law, base, 1, 0, spec.mirror, 0, 1);
    let x = add_cell(&mut law, base, 2, 50, spec.mirror, 0, 2);
    let y = add_cell(&mut law, base, 3, 100, spec.mirror, 0, 2);
    let z = add_cell(&mut law, base, 4, 150, spec.mirror, 0, 2);
    let effect = add_cell(&mut law, base, 5, 200, spec.mirror, 1, 1);
    law.add_arrow(trigger, x, 0, 2, 1, 30);
    law.add_arrow(x, y, 0, 3, 1, 30);
    law.add_arrow(y, z, 0, 4, 1, 30);
    law.add_arrow(z, effect, 0, 5, 1, 30);
    law.enter(0, 0, base + 601, x, 1);
    law.enter(0, 0, base + 602, y, 1);
    law.enter(0, 0, base + 603, z, 1);
    law.enter(0, 1, base + 604, trigger, 1);
    let run = law.propagate();
    let mut metrics = RunMetrics {
        naturally_quiescent: true,
        temporary_peak_count: 4,
        recursion_depth: [base + 2, base + 3, base + 4]
            .iter()
            .filter(|physical| run.fired_physical.contains(physical))
            .count(),
        effect_firings: run
            .fired_physical
            .iter()
            .filter(|physical| **physical == base + 5)
            .count(),
        ..RunMetrics::default()
    };
    metrics.locus_firings = metrics.recursion_depth;
    absorb(&mut metrics, &run, false);
    metrics.row_pass =
        metrics.recursion_depth == 3 && metrics.effect_firings == 1 && run.naturally_quiescent;
    finish_metrics(&law, &mut metrics);
    metrics
}

fn run_four_way<L: Law>(spec: &RowSpec) -> RunMetrics {
    let mut law = L::default();
    let base = spec.seed.saturating_mul(10_000);
    let mut expected = Vec::new();
    for pair in 0..2 {
        let offset = pair * 10;
        let trigger = add_cell(
            &mut law,
            base,
            1 + offset,
            offset as i32 * 20,
            spec.mirror,
            0,
            1,
        );
        let target = add_cell(
            &mut law,
            base,
            2 + offset,
            50 + offset as i32 * 20,
            spec.mirror,
            0,
            spec.threshold,
        );
        let effect = add_cell(
            &mut law,
            base,
            3 + offset,
            100 + offset as i32 * 20,
            spec.mirror,
            1,
            1,
        );
        law.add_arrow(trigger, target, 0, 4, spec.coupling, 30);
        law.add_arrow(target, effect, 0, 8, 1, 30);
        law.enter(
            0,
            pair as i32,
            base + 601 + pair,
            target,
            (spec.threshold - spec.coupling).max(1),
        );
        law.enter(0, 2 + pair as i32, base + 603 + pair, trigger, 1);
        expected.push(base + 3 + offset);
    }
    let run = law.propagate();
    let effects = expected
        .iter()
        .filter(|physical| run.fired_physical.contains(physical))
        .count();
    let mut metrics = RunMetrics {
        naturally_quiescent: true,
        temporary_peak_count: 4,
        effect_firings: effects,
        ..RunMetrics::default()
    };
    absorb(&mut metrics, &run, false);
    metrics.row_pass = effects == 2 && run.naturally_quiescent;
    finish_metrics(&law, &mut metrics);
    metrics
}

fn run_replay<L: Law>(spec: &RowSpec) -> RunMetrics {
    let mut first_spec = spec.clone();
    first_spec.kind = Kind::GatedDistinct;
    let first = run_gated::<L>(&first_spec);
    let second = run_gated::<L>(&first_spec);
    let mut metrics = first;
    metrics.replay_equal = metrics.complete_fingerprint == second.complete_fingerprint
        && metrics.permanent_fingerprint == second.permanent_fingerprint
        && metrics.effect_firings == second.effect_firings
        && metrics.execution_work == second.execution_work;
    metrics.row_pass = metrics.replay_equal;
    metrics
}

fn execute<L: Law>(spec: &RowSpec) -> RunMetrics {
    match spec.kind {
        Kind::DirectStrong
        | Kind::DirectTwoDistinct
        | Kind::DirectTwoSame
        | Kind::DirectAsymmetric
        | Kind::DirectAaSpacing => run_direct::<L>(spec),
        Kind::OnePathBurst | Kind::GatedDistinct | Kind::GatedSingleton | Kind::GatedReuse => {
            run_gated::<L>(spec)
        }
        Kind::BurstFour => run_burst_four::<L>(spec),
        Kind::DenseDirectReturn
        | Kind::DenseCrossedReturn
        | Kind::DenseNoReturn
        | Kind::DenseLateReturn => run_dense::<L>(spec),
        Kind::StaleQueued => run_stale::<L>(spec),
        Kind::DeallocateBootstrap => run_deallocate::<L>(spec),
        Kind::TimingTransfer => run_timing_transfer::<L>(spec),
        Kind::ContemporaryReversal => run_reversal::<L>(spec),
        Kind::Recursion => run_recursion::<L>(spec),
        Kind::FourWayAmbiguity => run_four_way::<L>(spec),
        Kind::ExactReplay => run_replay::<L>(spec),
    }
}

fn push_row(
    rows: &mut Vec<RowSpec>,
    prefix: &str,
    family: Family,
    kind: Kind,
    stratum: &(Stage, u64, bool, bool, i32, i32, usize),
    case: (i64, i64, usize, bool, bool),
) {
    let (spacing, train_spacing, allocation, expected_effect, genuine) = case;
    let (stage, seed, mirror, reverse, threshold, coupling, load) = *stratum;
    let row_id = format!(
        "{}-s{}-m{}-r{}-t{}-c{}-l{}-d{}-tr{}-a{}",
        prefix,
        seed,
        u8::from(mirror),
        u8::from(reverse),
        threshold,
        coupling,
        load,
        spacing,
        train_spacing,
        allocation
    );
    rows.push(RowSpec {
        row_id,
        stage,
        family,
        kind,
        seed,
        mirror,
        reverse_insertion: reverse,
        threshold,
        coupling,
        load,
        spacing,
        train_spacing,
        allocation,
        expected_effect,
        genuine,
    });
}

fn strata(stage: Stage) -> Vec<(Stage, u64, bool, bool, i32, i32, usize)> {
    let (seeds, mirrors, thresholds, loads): (&[u64], &[bool], &[i32], &[usize]) = match stage {
        Stage::Probe => (&[101], &[false], &[2], &[0]),
        Stage::Micro => (&[211, 223], &[false, true], &[2, 3], &[0, 4]),
        Stage::Gate => (
            &[307, 311, 313, 317],
            &[false, true],
            &[2, 3, 4],
            &[0, 4, 12],
        ),
    };
    let mut result = Vec::new();
    for seed in seeds {
        for mirror in mirrors {
            for threshold in thresholds {
                for coupling in [1, 2] {
                    for load in loads {
                        result.push((stage, *seed, *mirror, *mirror, *threshold, coupling, *load));
                    }
                }
            }
        }
    }
    result
}

fn matrix(stage: Stage) -> Vec<RowSpec> {
    let mut rows = Vec::new();
    for stratum in strata(stage) {
        push_row(
            &mut rows,
            "f1-aa-same-tick",
            Family::SameSource,
            Kind::DirectTwoSame,
            &stratum,
            (0, 0, 2, false, false),
        );
        for spacing in [-1, 0, 1, 2, 3, 4, 5] {
            push_row(
                &mut rows,
                "f1-aa-spacing",
                Family::SameSource,
                Kind::DirectAaSpacing,
                &stratum,
                (spacing, 0, 2, false, false),
            );
            push_row(
                &mut rows,
                "f1-ab-distinct",
                Family::SameSource,
                Kind::GatedDistinct,
                &stratum,
                (spacing, 0, 2, spacing == 0, true),
            );
        }
        push_row(
            &mut rows,
            "f1-one-path",
            Family::SameSource,
            Kind::OnePathBurst,
            &stratum,
            (1, 0, 2, false, false),
        );
        push_row(
            &mut rows,
            "f1-a-burst-4",
            Family::SameSource,
            Kind::OnePathBurst,
            &stratum,
            (4, 0, 2, false, false),
        );
        for (prefix, kind, expected, genuine) in [
            ("f2-one-strong", Kind::DirectStrong, false, false),
            ("f2-two-weak-distinct", Kind::DirectTwoDistinct, true, true),
            ("f2-two-weak-same", Kind::DirectTwoSame, false, false),
            ("f2-asymmetric-distinct", Kind::DirectAsymmetric, true, true),
        ] {
            push_row(
                &mut rows,
                prefix,
                Family::Amplitude,
                kind,
                &stratum,
                (0, 0, 2, expected, genuine),
            );
        }
        for allocation in [2, 6] {
            for (prefix, kind) in [
                ("f3-direct-return", Kind::DenseDirectReturn),
                ("f3-crossed-return", Kind::DenseCrossedReturn),
                ("f3-no-return", Kind::DenseNoReturn),
            ] {
                push_row(
                    &mut rows,
                    prefix,
                    Family::DenseReturn,
                    kind,
                    &stratum,
                    (0, 0, allocation, true, true),
                );
            }
        }
        for train_spacing in [0, 2, 4] {
            for spacing in [-1, 0, 1, 2, 3, 4, 5] {
                push_row(
                    &mut rows,
                    "f4-transfer",
                    Family::Timing,
                    Kind::GatedDistinct,
                    &stratum,
                    (spacing, train_spacing, 2, spacing == 0, true),
                );
            }
        }
        for (prefix, kind, expected, genuine) in [
            ("ctl-initial-ab", Kind::GatedReuse, true, true),
            ("ctl-initial-cd", Kind::GatedReuse, true, true),
            ("ctl-crossed-ad", Kind::GatedSingleton, false, false),
            ("ctl-crossed-cb", Kind::GatedSingleton, false, false),
            ("ctl-singleton", Kind::GatedSingleton, false, false),
            ("ctl-old-self-evidence", Kind::DenseNoReturn, true, true),
            ("ctl-blocked-return", Kind::DenseNoReturn, true, true),
            ("ctl-stale", Kind::StaleQueued, false, false),
            (
                "ctl-deallocation-bootstrap",
                Kind::DeallocateBootstrap,
                false,
                false,
            ),
            ("ctl-ambiguity", Kind::FourWayAmbiguity, true, true),
            ("ctl-recursion", Kind::Recursion, true, true),
            ("ctl-replay", Kind::ExactReplay, true, true),
        ] {
            push_row(
                &mut rows,
                prefix,
                Family::Control,
                kind,
                &stratum,
                (
                    0,
                    0,
                    if kind == Kind::DenseNoReturn { 6 } else { 2 },
                    expected,
                    genuine,
                ),
            );
        }
    }
    rows
}

fn correction_strata(stage: Stage) -> Vec<(Stage, u64, bool, bool, i32, i32, usize)> {
    let (seeds, mirrors, thresholds, loads): (&[u64], &[bool], &[i32], &[usize]) = match stage {
        Stage::Probe => (&[1101], &[false], &[2], &[0]),
        Stage::Micro => (&[1211, 1223], &[false, true], &[2, 3], &[0, 4]),
        Stage::Gate => (
            &[1307, 1311, 1313, 1317],
            &[false, true],
            &[2, 3, 4],
            &[0, 4, 12],
        ),
    };
    let mut result = Vec::new();
    for seed in seeds {
        for mirror in mirrors {
            for threshold in thresholds {
                for coupling in [1, 2] {
                    for load in loads {
                        result.push((stage, *seed, *mirror, *mirror, *threshold, coupling, *load));
                    }
                }
            }
        }
    }
    result
}

fn correction_matrix(stage: Stage) -> Vec<RowSpec> {
    let mut rows = Vec::new();
    for stratum in correction_strata(stage) {
        push_row(
            &mut rows,
            "coverage-correction-v2-burst-4",
            Family::SameSource,
            Kind::BurstFour,
            &stratum,
            (1, 0, 2, false, false),
        );
        push_row(
            &mut rows,
            "coverage-correction-v2-blocked-late-return",
            Family::DenseReturn,
            Kind::DenseLateReturn,
            &stratum,
            (0, 0, 6, true, true),
        );
        for train_spacing in [0, 2, 4] {
            for spacing in [-1, 0, 1, 2, 3, 4, 5] {
                push_row(
                    &mut rows,
                    "coverage-correction-v2-timing-transfer",
                    Family::Timing,
                    Kind::TimingTransfer,
                    &stratum,
                    (spacing, train_spacing, 2, false, true),
                );
            }
        }
        push_row(
            &mut rows,
            "coverage-correction-v2-contemporary-reversal",
            Family::Control,
            Kind::ContemporaryReversal,
            &stratum,
            (0, 0, 2, true, true),
        );
    }
    rows
}

fn candidate_csv(
    candidate: &str,
    rows: &[(RowSpec, RunMetrics)],
) -> Result<String, std::fmt::Error> {
    let mut csv = String::from(
        "row_id,candidate,stage,family,spec_fingerprint,seed,mirror,reverse_insertion,threshold,coupling,load,spacing,train_spacing,allocation,genuine,expected_effect,learning_work,execution_work,queue_work,delivered,generation_checks,temporary_peak_count,temporary_peak_bytes_lower_bound,temporary_accounting,persistent_bytes,arrow_records,live_structures,proposals,deallocations,return_updates,eligibility_writes,pressure_updates,locus_firings,effect_firings,transmissions,crossings,false_conjunction,resistance_start,resistance_end,coupling_start,coupling_end,reversal_cost,deallocation_cost,reacquisition_cost,timing_tolerance,dense_attribution_ok,recursion_depth,heldout_reuse,replay_equal,naturally_quiescent,runaway,complete_fingerprint,permanent_fingerprint,row_pass\n",
    );
    for (spec, metrics) in rows {
        let transmissions = metrics
            .transmissions
            .map_or_else(|| "NA".to_string(), |value| value.to_string());
        let fields = vec![
            spec.row_id.clone(),
            candidate.to_string(),
            spec.stage.name().to_string(),
            spec.family.name().to_string(),
            spec.spec_fingerprint().to_string(),
            spec.seed.to_string(),
            spec.mirror.to_string(),
            spec.reverse_insertion.to_string(),
            spec.threshold.to_string(),
            spec.coupling.to_string(),
            spec.load.to_string(),
            spec.spacing.to_string(),
            spec.train_spacing.to_string(),
            spec.allocation.to_string(),
            spec.genuine.to_string(),
            spec.expected_effect.to_string(),
            metrics.learning_work.to_string(),
            metrics.execution_work.to_string(),
            metrics.queue_work.to_string(),
            metrics.delivered.to_string(),
            metrics.generation_checks.to_string(),
            metrics.temporary_peak_count.to_string(),
            metrics.temporary_peak_bytes_lower_bound.to_string(),
            "LOWER_BOUND".to_string(),
            metrics.persistent_bytes.to_string(),
            metrics.arrow_records.to_string(),
            metrics.live_structures.to_string(),
            metrics.proposals.to_string(),
            metrics.deallocations.to_string(),
            metrics.return_updates.to_string(),
            metrics.eligibility_writes.to_string(),
            metrics.pressure_updates.to_string(),
            metrics.locus_firings.to_string(),
            metrics.effect_firings.to_string(),
            transmissions,
            metrics.crossings.to_string(),
            metrics.false_conjunction.to_string(),
            metrics.resistance_start.to_string(),
            metrics.resistance_end.to_string(),
            metrics.coupling_start.to_string(),
            metrics.coupling_end.to_string(),
            metrics.reversal_cost.to_string(),
            metrics.deallocation_cost.to_string(),
            metrics.reacquisition_cost.to_string(),
            metrics.timing_tolerance.to_string(),
            metrics.dense_attribution_ok.to_string(),
            metrics.recursion_depth.to_string(),
            metrics.heldout_reuse.to_string(),
            metrics.replay_equal.to_string(),
            metrics.naturally_quiescent.to_string(),
            metrics.runaway.to_string(),
            metrics.complete_fingerprint.to_string(),
            metrics.permanent_fingerprint.to_string(),
            metrics.row_pass.to_string(),
        ];
        writeln!(csv, "{}", fields.join(","))?;
    }
    Ok(csv)
}

fn paired_csv(
    b_rows: &[(RowSpec, RunMetrics)],
    e_rows: &[(RowSpec, RunMetrics)],
) -> Result<String, std::fmt::Error> {
    let mut csv = String::from(
        "row_id,stage,family,spec_fingerprint,b_effect,e_effect,prediction_diff,b_false_conjunction,e_false_conjunction,b_row_pass,e_row_pass,b_work,e_work,b_persistent_bytes,e_persistent_bytes,b_quiescent,e_quiescent\n",
    );
    for ((b_spec, b_metrics), (e_spec, e_metrics)) in b_rows.iter().zip(e_rows) {
        assert_eq!(b_spec.row_id, e_spec.row_id, "paired row mismatch");
        assert_eq!(
            b_spec.physical_serialization(),
            e_spec.physical_serialization(),
            "physical spec mismatch"
        );
        writeln!(
            csv,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            b_spec.row_id,
            b_spec.stage.name(),
            b_spec.family.name(),
            b_spec.spec_fingerprint(),
            b_metrics.effect_firings,
            e_metrics.effect_firings,
            b_metrics.effect_firings != e_metrics.effect_firings,
            b_metrics.false_conjunction,
            e_metrics.false_conjunction,
            b_metrics.row_pass,
            e_metrics.row_pass,
            b_metrics.learning_work + b_metrics.execution_work,
            e_metrics.learning_work + e_metrics.execution_work,
            b_metrics.persistent_bytes,
            e_metrics.persistent_bytes,
            b_metrics.naturally_quiescent,
            e_metrics.naturally_quiescent
        )?;
    }
    Ok(csv)
}

fn report(
    stage: Stage,
    b_rows: &[(RowSpec, RunMetrics)],
    e_rows: &[(RowSpec, RunMetrics)],
) -> Result<String, std::fmt::Error> {
    let mut text = String::new();
    let b_pass = b_rows
        .iter()
        .filter(|(_, metrics)| metrics.row_pass)
        .count();
    let e_pass = e_rows
        .iter()
        .filter(|(_, metrics)| metrics.row_pass)
        .count();
    let b_false = b_rows
        .iter()
        .filter(|(_, metrics)| metrics.false_conjunction)
        .count();
    let e_false = e_rows
        .iter()
        .filter(|(_, metrics)| metrics.false_conjunction)
        .count();
    let prediction_diffs = b_rows
        .iter()
        .zip(e_rows)
        .filter(|((_, left), (_, right))| left.effect_firings != right.effect_firings)
        .count();
    let b_work: u64 = b_rows
        .iter()
        .map(|(_, metrics)| metrics.learning_work + metrics.execution_work)
        .sum();
    let e_work: u64 = e_rows
        .iter()
        .map(|(_, metrics)| metrics.learning_work + metrics.execution_work)
        .sum();
    let b_scientific = b_pass == b_rows.len();
    let e_scientific = e_pass == e_rows.len();
    let classification = match (b_scientific, e_scientific) {
        (true, false) => "CJ-B SCIENTIFICALLY SUFFICIENT; CJ-E FAILS",
        (false, true) => "CJ-E SCIENTIFICALLY SUFFICIENT; CJ-B FAILS",
        (true, true) => "MULTIPLE VALID MINIMAL SUBSTRATES / IMPLEMENTATION-HARDWARE FORK",
        (false, false) => "BOTH FAIL; SHARED BOUNDARY FROZEN",
    };
    writeln!(
        text,
        "# CJ0-F {} development result",
        stage.name().to_uppercase()
    )?;
    writeln!(text)?;
    writeln!(text, "Status: **{}**.", classification)?;
    writeln!(text)?;
    writeln!(text, "- protocol SHA-256: `{PROTOCOL_SHA256}`;")?;
    writeln!(text, "- CJ-B law SHA-256: `{B_SHA256}`;")?;
    writeln!(text, "- CJ-E law SHA-256: `{E_SHA256}`;")?;
    writeln!(text, "- paired rows: `{}`;", b_rows.len())?;
    writeln!(text, "- differing predictions: `{prediction_diffs}`;")?;
    writeln!(text, "- CJ-B row pass: `{b_pass}/{}`;", b_rows.len())?;
    writeln!(text, "- CJ-E row pass: `{e_pass}/{}`;", e_rows.len())?;
    writeln!(text, "- CJ-B false conjunction rows: `{b_false}`;")?;
    writeln!(text, "- CJ-E false conjunction rows: `{e_false}`;")?;
    writeln!(text, "- CJ-B total work: `{b_work}`;")?;
    writeln!(text, "- CJ-E total work: `{e_work}`;")?;
    writeln!(text)?;
    writeln!(
        text,
        "Classification follows scientific sufficiency before economics. A row-level failure is not repaired or tuned. Candidate-native counters unavailable on one law are serialized as `NA`; temporary bytes remain the preregistered `LOWER_BOUND`."
    )?;
    writeln!(text)?;
    writeln!(
        text,
        "This is development-only evidence and ends no later than GATE. It does not restart PX3 or create definitive/authority evidence."
    )?;
    Ok(text)
}

fn artifact_paths(stage: Stage) -> [(PathBuf, PathBuf); 4] {
    let stem = format!("results/cj0_f_matched_discriminator_{}", stage.name());
    [
        (
            PathBuf::from(format!("{stem}_b.csv")),
            PathBuf::from(format!("results/.cj0_f_{}_b.csv.staging", stage.name())),
        ),
        (
            PathBuf::from(format!("{stem}_e.csv")),
            PathBuf::from(format!("results/.cj0_f_{}_e.csv.staging", stage.name())),
        ),
        (
            PathBuf::from(format!("{stem}_paired.csv")),
            PathBuf::from(format!(
                "results/.cj0_f_{}_paired.csv.staging",
                stage.name()
            )),
        ),
        (
            PathBuf::from(format!("{stem}.md")),
            PathBuf::from(format!("results/.cj0_f_{}.md.staging", stage.name())),
        ),
    ]
}

fn correction_artifact_paths(stage: Stage) -> [(PathBuf, PathBuf); 4] {
    let stem = format!(
        "results/cj0_f_matched_discriminator_coverage_correction_v2_{}",
        stage.name()
    );
    [
        (
            PathBuf::from(format!("{stem}_b.csv")),
            PathBuf::from(format!(
                "results/.cj0_f_coverage_correction_v2_{}_b.csv.staging",
                stage.name()
            )),
        ),
        (
            PathBuf::from(format!("{stem}_e.csv")),
            PathBuf::from(format!(
                "results/.cj0_f_coverage_correction_v2_{}_e.csv.staging",
                stage.name()
            )),
        ),
        (
            PathBuf::from(format!("{stem}_paired.csv")),
            PathBuf::from(format!(
                "results/.cj0_f_coverage_correction_v2_{}_paired.csv.staging",
                stage.name()
            )),
        ),
        (
            PathBuf::from(format!("{stem}.md")),
            PathBuf::from(format!(
                "results/.cj0_f_coverage_correction_v2_{}.md.staging",
                stage.name()
            )),
        ),
    ]
}

fn publish_atomic(final_path: &Path, stage_path: &Path, bytes: &[u8]) -> Result<(), String> {
    if final_path.exists() || stage_path.exists() {
        return Err(format!(
            "refusing existing artifact: {} or {}",
            final_path.display(),
            stage_path.display()
        ));
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(stage_path)
        .map_err(|error| format!("create {}: {error}", stage_path.display()))?;
    file.write_all(bytes)
        .map_err(|error| format!("write {}: {error}", stage_path.display()))?;
    file.sync_all()
        .map_err(|error| format!("sync {}: {error}", stage_path.display()))?;
    drop(file);
    fs::rename(stage_path, final_path).map_err(|error| {
        format!(
            "rename {} to {}: {error}",
            stage_path.display(),
            final_path.display()
        )
    })?;
    Ok(())
}

fn prerequisite(stage: Stage) -> Result<(), String> {
    let prior = match stage {
        Stage::Probe => return Ok(()),
        Stage::Micro => Stage::Probe,
        Stage::Gate => Stage::Micro,
    };
    let report_path = format!("results/cj0_f_matched_discriminator_{}.md", prior.name());
    let report = fs::read_to_string(&report_path)
        .map_err(|error| format!("missing prerequisite {report_path}: {error}"))?;
    if !report.contains("development result") || report.contains("ambiguous") {
        return Err(format!("uninterpretable prerequisite {report_path}"));
    }
    Ok(())
}

fn run_stage(stage: Stage) -> Result<(), String> {
    prerequisite(stage)?;
    for (final_path, stage_path) in artifact_paths(stage) {
        if final_path.exists() || stage_path.exists() {
            return Err(format!(
                "artifact preflight failed: {} or {} exists",
                final_path.display(),
                stage_path.display()
            ));
        }
    }
    let specs = matrix(stage);
    let b_rows = specs
        .iter()
        .cloned()
        .map(|spec| {
            let metrics = execute::<LawB>(&spec);
            (spec, metrics)
        })
        .collect::<Vec<_>>();
    let e_rows = specs
        .iter()
        .cloned()
        .map(|spec| {
            let metrics = execute::<LawE>(&spec);
            (spec, metrics)
        })
        .collect::<Vec<_>>();
    let b_csv = candidate_csv("CJ-B", &b_rows).map_err(|error| error.to_string())?;
    let e_csv = candidate_csv("CJ-E", &e_rows).map_err(|error| error.to_string())?;
    let pairs = paired_csv(&b_rows, &e_rows).map_err(|error| error.to_string())?;
    let summary = report(stage, &b_rows, &e_rows).map_err(|error| error.to_string())?;
    let paths = artifact_paths(stage);
    for ((final_path, stage_path), content) in paths.iter().zip([b_csv, e_csv, pairs, summary]) {
        publish_atomic(final_path, stage_path, content.as_bytes())?;
    }
    Ok(())
}

fn correction_prerequisite(stage: Stage) -> Result<(), String> {
    let prior = match stage {
        Stage::Probe => return Ok(()),
        Stage::Micro => Stage::Probe,
        Stage::Gate => Stage::Micro,
    };
    let report_path = format!(
        "results/cj0_f_matched_discriminator_coverage_correction_v2_{}.md",
        prior.name()
    );
    let report = fs::read_to_string(&report_path)
        .map_err(|error| format!("missing correction prerequisite {report_path}: {error}"))?;
    if !report.contains("development result") {
        return Err(format!(
            "uninterpretable correction prerequisite {report_path}"
        ));
    }
    Ok(())
}

fn run_correction_stage(stage: Stage) -> Result<(), String> {
    correction_prerequisite(stage)?;
    for (final_path, stage_path) in correction_artifact_paths(stage) {
        if final_path.exists() || stage_path.exists() {
            return Err(format!(
                "correction artifact preflight failed: {} or {} exists",
                final_path.display(),
                stage_path.display()
            ));
        }
    }
    let specs = correction_matrix(stage);
    let b_rows = specs
        .iter()
        .cloned()
        .map(|spec| {
            let metrics = execute::<LawB>(&spec);
            (spec, metrics)
        })
        .collect::<Vec<_>>();
    let e_rows = specs
        .iter()
        .cloned()
        .map(|spec| {
            let metrics = execute::<LawE>(&spec);
            (spec, metrics)
        })
        .collect::<Vec<_>>();
    let b_csv = candidate_csv("CJ-B", &b_rows).map_err(|error| error.to_string())?;
    let e_csv = candidate_csv("CJ-E", &e_rows).map_err(|error| error.to_string())?;
    let pairs = paired_csv(&b_rows, &e_rows).map_err(|error| error.to_string())?;
    let summary = report(stage, &b_rows, &e_rows)
        .map_err(|error| error.to_string())?
        .replacen("# CJ0-F", "# CJ0-F COVERAGE CORRECTION V2", 1);
    let paths = correction_artifact_paths(stage);
    for ((final_path, stage_path), content) in paths.iter().zip([b_csv, e_csv, pairs, summary]) {
        publish_atomic(final_path, stage_path, content.as_bytes())?;
    }
    Ok(())
}

fn preflight() -> Result<(), String> {
    if B_SHA256.len() != 64 || E_SHA256.len() != 64 || PROTOCOL_SHA256.len() != 64 {
        return Err("embedded hash width mismatch".to_string());
    }
    for stage in [Stage::Probe, Stage::Micro, Stage::Gate] {
        let rows = matrix(stage);
        if rows.is_empty() {
            return Err(format!("empty {} matrix", stage.name()));
        }
        let mut ids = rows
            .iter()
            .map(|row| row.row_id.as_str())
            .collect::<Vec<_>>();
        ids.sort_unstable();
        if ids.windows(2).any(|window| window[0] == window[1]) {
            return Err(format!("duplicate {} row id", stage.name()));
        }
        let correction_rows = correction_matrix(stage);
        let mut correction_ids = correction_rows
            .iter()
            .map(|row| row.row_id.as_str())
            .collect::<Vec<_>>();
        correction_ids.sort_unstable();
        if correction_ids
            .windows(2)
            .any(|window| window[0] == window[1])
        {
            return Err(format!("duplicate correction {} row id", stage.name()));
        }
    }
    println!(
        "preflight=PASS definitive=false authority=false b_sha256={B_SHA256} e_sha256={E_SHA256} protocol_sha256={PROTOCOL_SHA256} probe_rows={} micro_rows={} gate_rows={} correction_probe_rows={} correction_micro_rows={} correction_gate_rows={}",
        matrix(Stage::Probe).len(),
        matrix(Stage::Micro).len(),
        matrix(Stage::Gate).len(),
        correction_matrix(Stage::Probe).len(),
        correction_matrix(Stage::Micro).len(),
        correction_matrix(Stage::Gate).len()
    );
    Ok(())
}

fn real_main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let command = args.next().ok_or_else(|| {
        "usage: cj0-f-matched-discriminator --preflight|probe|micro|gate".to_string()
    })?;
    if args.next().is_some() {
        return Err("unexpected extra argument".to_string());
    }
    match command.as_str() {
        "--preflight" => preflight(),
        "probe" => run_stage(Stage::Probe),
        "micro" => run_stage(Stage::Micro),
        "gate" => run_stage(Stage::Gate),
        "correction-probe" => run_correction_stage(Stage::Probe),
        "correction-micro" => run_correction_stage(Stage::Micro),
        "correction-gate" => run_correction_stage(Stage::Gate),
        _ => Err("usage: cj0-f-matched-discriminator --preflight|probe|micro|gate|correction-probe|correction-micro|correction-gate".to_string()),
    }
}

fn fingerprint(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrices_have_unique_paired_ids() {
        preflight().expect("preflight");
    }

    #[test]
    fn strong_singleton_discriminates_frozen_laws() {
        let spec = RowSpec {
            row_id: "unit-strong".to_string(),
            stage: Stage::Probe,
            family: Family::Amplitude,
            kind: Kind::DirectStrong,
            seed: 991,
            mirror: false,
            reverse_insertion: false,
            threshold: 2,
            coupling: 1,
            load: 0,
            spacing: 0,
            train_spacing: 0,
            allocation: 2,
            expected_effect: false,
            genuine: false,
        };
        let b = execute::<LawB>(&spec);
        let e = execute::<LawE>(&spec);
        assert_eq!(b.effect_firings, 1);
        assert_eq!(e.effect_firings, 0);
    }

    #[test]
    fn genuine_same_tick_coincidence_reaches_effect() {
        let spec = RowSpec {
            row_id: "unit-genuine".to_string(),
            stage: Stage::Probe,
            family: Family::SameSource,
            kind: Kind::GatedDistinct,
            seed: 992,
            mirror: false,
            reverse_insertion: false,
            threshold: 2,
            coupling: 1,
            load: 0,
            spacing: 0,
            train_spacing: 0,
            allocation: 2,
            expected_effect: true,
            genuine: true,
        };
        assert!(execute::<LawB>(&spec).row_pass);
        assert!(execute::<LawE>(&spec).row_pass);
    }

    #[test]
    fn heldout_reuse_uses_current_absolute_tick_for_both_laws() {
        let spec = RowSpec {
            row_id: "unit-reuse".to_string(),
            stage: Stage::Probe,
            family: Family::Control,
            kind: Kind::GatedReuse,
            seed: 993,
            mirror: false,
            reverse_insertion: false,
            threshold: 2,
            coupling: 1,
            load: 0,
            spacing: 0,
            train_spacing: 0,
            allocation: 2,
            expected_effect: true,
            genuine: true,
        };
        assert!(execute::<LawB>(&spec).heldout_reuse);
        assert!(execute::<LawE>(&spec).heldout_reuse);
    }

    #[test]
    fn corrected_mandatory_fixtures_execute_symmetrically() {
        let base = RowSpec {
            row_id: "unit-correction".to_string(),
            stage: Stage::Probe,
            family: Family::Control,
            kind: Kind::BurstFour,
            seed: 994,
            mirror: false,
            reverse_insertion: false,
            threshold: 2,
            coupling: 1,
            load: 0,
            spacing: 0,
            train_spacing: 0,
            allocation: 2,
            expected_effect: false,
            genuine: false,
        };
        for kind in [
            Kind::BurstFour,
            Kind::DenseLateReturn,
            Kind::TimingTransfer,
            Kind::ContemporaryReversal,
        ] {
            let mut spec = base.clone();
            spec.kind = kind;
            spec.allocation = if kind == Kind::DenseLateReturn { 6 } else { 2 };
            let b = execute::<LawB>(&spec);
            let e = execute::<LawE>(&spec);
            assert!(b.naturally_quiescent, "CJ-B {kind:?}");
            assert!(e.naturally_quiescent, "CJ-E {kind:?}");
        }
        assert_eq!(execute::<LawB>(&base).temporary_peak_count, 4);
        assert_eq!(execute::<LawE>(&base).temporary_peak_count, 4);
    }
}
