use cj0_e_transient_coincidence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const INCIDENCE: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (2, 1), (2, 3), (1, 3)];
const PROTOCOL: &str = "cj0-e-transient-coincidence-development-gate-v1";
const CSV_PATH: &str = "results/cj0_e_transient_coincidence_gate_v1.csv";
const MD_PATH: &str = "results/cj0_e_transient_coincidence_gate_v1.md";
const MICRO_V2_MD_SHA256: &str = "361252975b6c588a2e2046b83ce9b85fe72ead3718ec2325c304ec784b03cd07";
const LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const V1_EVALUATOR_SHA256: &str =
    "df8d5c3cb1bcf5d35b345268deed7f40166b2c7307250723020f5907d699c963";
const V2_EVALUATOR_SHA256: &str =
    "f72f481bfaf0462bcadf7a65179a37e2d91b4bfda32c8aa03501f62a93e82695";

#[derive(Clone, Copy)]
struct Variant {
    mirror: bool,
    reverse_allocation: bool,
    reverse_arrival: bool,
    output_offset: i32,
    distractors: usize,
}

impl Default for Variant {
    fn default() -> Self {
        Self {
            mirror: false,
            reverse_allocation: false,
            reverse_arrival: false,
            output_offset: 1,
            distractors: 0,
        }
    }
}

#[derive(Clone)]
struct Matter {
    substrate: PlasticSubstrate,
    namespace: u64,
    participants: [CellId; 4],
    loci: [CellId; 6],
    outputs: [CellId; 6],
    distractors: Vec<CellId>,
    participant_physical: [u64; 4],
    locus_physical: [u64; 6],
    output_physical: [u64; 6],
    distractor_physical: Vec<u64>,
    reverse_arrival: bool,
    routes: [u64; 4],
    locus_firings: [u64; 6],
    output_firings: [u64; 6],
    distractor_firings: u64,
    aggregate_returns: u64,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, Default)]
struct Seen {
    routes: [u64; 4],
    loci: [u64; 6],
    outputs: [u64; 6],
    distractors: u64,
    learned_traversals: u64,
    aggregate_returns: u64,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Support {
    resistance: u32,
    coupling: i32,
    generation: u32,
    live: bool,
}

#[derive(Clone)]
struct Row {
    category: &'static str,
    cell: String,
    scenario: &'static str,
    passed: bool,
    routes: [u64; 4],
    loci: [u64; 6],
    outputs: [u64; 6],
    support_before: [u32; 6],
    support_after: [u32; 6],
    coupling_after: [i32; 6],
    generation_after: [u32; 6],
    live_after: [bool; 6],
    learned_traversals: u64,
    learned_output_firings: u64,
    learned_support_increases: u64,
    aggregate_returns: u64,
    distractor_firings: u64,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    arrows: usize,
    bytes: usize,
    work: u64,
    quiescent: bool,
    duplicate_exact: bool,
    note: String,
}

struct Report {
    rows: Vec<Row>,
    clauses: Vec<(&'static str, bool)>,
}

impl Report {
    fn passed(&self) -> bool {
        !self.rows.is_empty()
            && self.rows.iter().all(|row| row.passed)
            && self.clauses.iter().all(|(_, pass)| *pass)
    }

    fn work(&self) -> u64 {
        self.rows.iter().map(|row| row.work).sum()
    }
}

fn build(namespace: u64, variant: Variant) -> Matter {
    let mut substrate = PlasticSubstrate::new();
    let sign = if variant.mirror { -1 } else { 1 };
    let participant_order: Vec<usize> = if variant.reverse_allocation {
        (0..4).rev().collect()
    } else {
        (0..4).collect()
    };
    let mut participants = [None; 4];
    let mut participant_physical = [0; 4];
    for index in participant_order {
        let physical_id = namespace + 0x100 + index as u64;
        participant_physical[index] = physical_id;
        participants[index] = Some(substrate.add_cell(CellSpec {
            physical_id,
            position: sign * (1_000 + index as i32 * 20),
            region: 0,
            threshold: 3,
            resistance: 10_000,
        }));
    }
    let site_order: Vec<usize> = if variant.reverse_allocation {
        (0..6).rev().collect()
    } else {
        (0..6).collect()
    };
    let mut loci = [None; 6];
    let mut outputs = [None; 6];
    let mut locus_physical = [0; 6];
    let mut output_physical = [0; 6];
    for index in site_order {
        let center = sign * index as i32 * 20;
        let locus_id = namespace + 0x200 + index as u64;
        let output_id = namespace + 0x300 + index as u64;
        locus_physical[index] = locus_id;
        output_physical[index] = output_id;
        loci[index] = Some(substrate.add_cell(CellSpec {
            physical_id: locus_id,
            position: center,
            region: 0,
            threshold: 2,
            resistance: 10_000,
        }));
        outputs[index] = Some(substrate.add_cell(CellSpec {
            physical_id: output_id,
            position: center + sign * variant.output_offset,
            region: 1,
            threshold: 3,
            resistance: 10_000,
        }));
    }
    let participants = participants.map(|cell| cell.expect("participant"));
    let loci = loci.map(|cell| cell.expect("locus"));
    let outputs = outputs.map(|cell| cell.expect("output"));
    for (index, (left, right)) in INCIDENCE.iter().copied().enumerate() {
        for participant in [left, right] {
            substrate.add_arrow(ArrowSpec {
                from: participants[participant],
                to: loci[index],
                delay: 1,
                phase: 0,
                coupling: 1,
                resistance: 10_000,
            });
        }
    }
    let mut distractors = Vec::new();
    let mut distractor_physical = Vec::new();
    for index in 0..variant.distractors {
        let physical_id = namespace + 0x800 + index as u64;
        distractor_physical.push(physical_id);
        distractors.push(substrate.add_cell(CellSpec {
            physical_id,
            position: sign * (10_000 + index as i32 * 10),
            region: 0,
            threshold: 1,
            resistance: 10_000,
        }));
    }
    Matter {
        substrate,
        namespace,
        participants,
        loci,
        outputs,
        distractors,
        participant_physical,
        locus_physical,
        output_physical,
        distractor_physical,
        reverse_arrival: variant.reverse_arrival,
        routes: [0; 4],
        locus_firings: [0; 6],
        output_firings: [0; 6],
        distractor_firings: 0,
        aggregate_returns: 0,
        work: 0,
        quiescent: true,
    }
}

impl Matter {
    fn enter_participant(&mut self, index: usize, tick: i64) {
        let phases = if self.reverse_arrival {
            [-1, -2]
        } else {
            [-2, -1]
        };
        for (phase, impulse, offset) in [(phases[0], 1, 0x400), (phases[1], 2, 0x500)] {
            self.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase,
                origin_physical: self.namespace + offset + index as u64,
                target: self.participants[index],
                impulse,
            });
        }
    }

    fn enter_primes(&mut self, tick: i64) {
        for (index, target) in self.outputs.iter().copied().enumerate() {
            self.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase: -20,
                origin_physical: self.namespace + 0x600 + index as u64,
                target,
                impulse: 1,
            });
        }
    }

    fn enter_returns(&mut self, tick: i64) {
        for (index, target) in self.loci.iter().copied().enumerate() {
            self.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase: 20,
                origin_physical: self.namespace + 0x700 + index as u64,
                target,
                impulse: 1,
            });
        }
    }

    fn enter_distractors(&mut self, tick: i64) {
        for (index, target) in self.distractors.iter().copied().enumerate() {
            self.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase: 30,
                origin_physical: self.namespace + 0x900 + index as u64,
                target,
                impulse: 1,
            });
        }
    }

    fn occurrence(
        &mut self,
        active: &[usize],
        tick: i64,
        returned: bool,
        monitored: Option<usize>,
    ) -> Seen {
        for index in active {
            self.enter_participant(*index, tick);
        }
        self.enter_primes(tick + 2);
        if returned {
            self.enter_returns(tick + 2);
        }
        self.enter_distractors(tick + 2);
        let execution = self.substrate.propagate();
        self.record(execution, monitored)
    }

    fn record(&mut self, execution: Execution, monitored: Option<usize>) -> Seen {
        let mut seen = Seen {
            aggregate_returns: execution.work.local_return_updates,
            work: execution.work.total(),
            quiescent: execution.naturally_quiescent,
            ..Seen::default()
        };
        for entry in execution.trace.iter().filter(|entry| entry.fired) {
            if let Some(index) = self
                .participant_physical
                .iter()
                .position(|physical| *physical == entry.target_physical)
            {
                seen.routes[index] += 1;
            }
            if let Some(index) = self
                .locus_physical
                .iter()
                .position(|physical| *physical == entry.target_physical)
            {
                seen.loci[index] += 1;
            }
            if let Some(index) = self
                .output_physical
                .iter()
                .position(|physical| *physical == entry.target_physical)
            {
                seen.outputs[index] += 1;
            }
            if self.distractor_physical.contains(&entry.target_physical) {
                seen.distractors += 1;
            }
        }
        if let Some(index) = monitored {
            seen.learned_traversals = execution
                .crossings
                .iter()
                .filter(|crossing| {
                    crossing.from_physical == self.locus_physical[index]
                        && crossing.to_physical == self.output_physical[index]
                })
                .count() as u64;
        }
        for index in 0..4 {
            self.routes[index] += seen.routes[index];
        }
        for index in 0..6 {
            self.locus_firings[index] += seen.loci[index];
            self.output_firings[index] += seen.outputs[index];
        }
        self.distractor_firings += seen.distractors;
        self.aggregate_returns += seen.aggregate_returns;
        self.work += seen.work;
        self.quiescent &= seen.quiescent;
        seen
    }

    fn advance(&mut self, tick: i64) {
        self.work += self.substrate.advance_time(tick).total();
    }

    fn selected_arrow(&self, index: usize) -> Option<ArrowId> {
        self.substrate
            .arrows_between(self.loci[index], self.outputs[index])
            .into_iter()
            .filter(|arrow| self.substrate.arrow_is_live(*arrow))
            .max_by_key(|arrow| self.substrate.arrow_resistance(*arrow))
    }

    fn support(&self, index: usize) -> Support {
        self.selected_arrow(index)
            .map_or(Support::default(), |arrow| Support {
                resistance: self.substrate.arrow_resistance(arrow),
                coupling: self.substrate.arrow_coupling(arrow),
                generation: self.substrate.arrow_generation(arrow),
                live: true,
            })
    }

    fn supports(&self) -> ([u32; 6], [i32; 6], [u32; 6], [bool; 6]) {
        let all = std::array::from_fn::<Support, 6, _>(|index| self.support(index));
        (
            all.map(|support| support.resistance),
            all.map(|support| support.coupling),
            all.map(|support| support.generation),
            all.map(|support| support.live),
        )
    }
}

fn train(
    matter: &mut Matter,
    first: (usize, usize),
    second: (usize, usize),
    rounds: usize,
    start: i64,
    spacing: i64,
    gap: i64,
) {
    for round in 0..rounds {
        let tick = start + round as i64 * spacing;
        matter.occurrence(&[first.0, first.1], tick, true, None);
        matter.occurrence(&[second.0, second.1], tick + gap, true, None);
    }
}

fn use_once(matter: &Matter, active: &[usize], monitored: Option<usize>) -> Seen {
    let mut clone = matter.clone();
    let tick = clone.substrate.current_tick() + 4;
    clone.occurrence(active, tick, false, monitored)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SelfCheck {
    traversals: u64,
    outputs: u64,
    increases: u64,
    aggregate_returns: u64,
    before: Support,
    after: Support,
    quiescent: bool,
}

fn repeated_singleton(source: &Matter, site: usize, participant: usize) -> SelfCheck {
    let mut matter = source.clone();
    let before = matter.support(site);
    let aggregate_before = matter.aggregate_returns;
    let start = matter.substrate.current_tick() + 4;
    let mut traversals = 0;
    let mut outputs = 0;
    let mut increases = 0;
    for round in 0..12 {
        let resistance_before = matter.support(site).resistance;
        let seen = matter.occurrence(&[participant], start + round * 4, false, Some(site));
        traversals += seen.learned_traversals;
        outputs += seen.outputs[site];
        increases += u64::from(matter.support(site).resistance > resistance_before);
    }
    SelfCheck {
        traversals,
        outputs,
        increases,
        aggregate_returns: matter.aggregate_returns - aggregate_before,
        before,
        after: matter.support(site),
        quiescent: matter.quiescent,
    }
}

fn row_from_matter(
    matter: &Matter,
    category: &'static str,
    cell: String,
    scenario: &'static str,
    passed: bool,
    before: ([u32; 6], [i32; 6], [u32; 6], [bool; 6]),
    note: String,
) -> Row {
    let after = matter.supports();
    Row {
        category,
        cell,
        scenario,
        passed,
        routes: matter.routes,
        loci: matter.locus_firings,
        outputs: matter.output_firings,
        support_before: before.0,
        support_after: after.0,
        coupling_after: after.1,
        generation_after: after.2,
        live_after: after.3,
        learned_traversals: 0,
        learned_output_firings: 0,
        learned_support_increases: 0,
        aggregate_returns: matter.aggregate_returns,
        distractor_firings: matter.distractor_firings,
        complete_fingerprint: matter.substrate.complete_fingerprint(),
        permanent_fingerprint: matter.substrate.permanent_fingerprint(),
        arrows: matter.substrate.arrow_count(),
        bytes: matter.substrate.persistent_bytes(),
        work: matter.work,
        quiescent: matter.quiescent,
        duplicate_exact: false,
        note,
    }
}

fn flat_run(seed: usize, stratum: usize, duplicate: bool) -> Row {
    let namespace = 0xd300_0000 + seed as u64 * 0x10_0000 + stratum as u64 * 0x1_0000;
    let spacing = [20, 22, 24, 26][stratum];
    let gap = [3, 4, 5, 6][stratum];
    let distractors = [0, 8, 24, 48][stratum];
    let variant = Variant {
        mirror: stratum.is_multiple_of(2),
        reverse_allocation: (seed + stratum).is_multiple_of(2),
        reverse_arrival: (seed + stratum).is_multiple_of(3),
        output_offset: 1,
        distractors,
    };
    let mut matter = build(namespace, variant);
    train(&mut matter, (0, 1), (2, 3), 12, 0, spacing, gap);
    let initial = matter.supports();
    let trained_ab = use_once(&matter, &[0, 1], Some(0));
    let trained_cd = use_once(&matter, &[2, 3], Some(4));
    let crossed_ad = use_once(&matter, &[0, 3], Some(2));
    let crossed_cb = use_once(&matter, &[2, 1], Some(3));
    let initial_live = initial.3[0] && initial.3[4] && initial.1[0] == 2 && initial.1[4] == 2;
    let trained = trained_ab.outputs[0] == 1 && trained_cd.outputs[4] == 1;
    let crossed = crossed_ad.outputs[2] == 0 && crossed_cb.outputs[3] == 0;
    let start = matter.substrate.current_tick() + spacing;
    train(&mut matter, (0, 3), (2, 1), 40, start, spacing, gap);
    let changed = matter.supports();
    let new_ad = use_once(&matter, &[0, 3], Some(2));
    let new_cb = use_once(&matter, &[2, 1], Some(3));
    let old_ab = use_once(&matter, &[0, 1], Some(0));
    let old_cd = use_once(&matter, &[2, 3], Some(4));
    let old_dead = !changed.3[0] && !changed.3[4];
    let new_live = changed.3[2] && changed.3[3] && changed.1[2] == 2 && changed.1[3] == 2;
    let new_executes = new_ad.outputs[2] == 1 && new_cb.outputs[3] == 1;
    let old_silent = old_ab.outputs[0] == 0 && old_cd.outputs[4] == 0;
    let self_check = repeated_singleton(&matter, 2, 0);
    let self_pass = self_check.traversals == 0
        && self_check.outputs == 0
        && self_check.increases == 0
        && self_check.after.resistance < self_check.before.resistance
        && self_check.after.coupling == self_check.before.coupling
        && self_check.after.generation == self_check.before.generation
        && self_check.aggregate_returns > 0
        && self_check.quiescent;
    let matched = matter.routes.iter().all(|count| *count == matter.routes[0]);
    let expected_distractors = (12 * 2 + 40 * 2) as u64 * distractors as u64;
    let distractors_exact = matter.distractor_firings == expected_distractors;
    let quiescent = matter.quiescent
        && [
            &trained_ab,
            &trained_cd,
            &crossed_ad,
            &crossed_cb,
            &new_ad,
            &new_cb,
            &old_ab,
            &old_cd,
        ]
        .into_iter()
        .all(|seen| seen.quiescent);
    let passed = initial_live
        && trained
        && crossed
        && old_dead
        && new_live
        && new_executes
        && old_silent
        && self_pass
        && matched
        && distractors_exact
        && quiescent;
    let mut row = row_from_matter(
        &matter,
        "flat",
        format!("s{seed}-g{stratum}-{}", if duplicate { "dup" } else { "primary" }),
        "matched-reversal",
        passed,
        initial,
        format!(
            "initial={initial_live};trained={trained};crossed={crossed};old_dead={old_dead};new={new_live};new_exec={new_executes};old_silent={old_silent};self={self_pass};spacing={spacing};gap={gap};load={distractors}"
        ),
    );
    row.learned_traversals = self_check.traversals;
    row.learned_output_firings = self_check.outputs;
    row.learned_support_increases = self_check.increases;
    row
}

fn rows_exact(left: &Row, right: &Row) -> bool {
    left.passed == right.passed
        && left.routes == right.routes
        && left.loci == right.loci
        && left.outputs == right.outputs
        && left.support_before == right.support_before
        && left.support_after == right.support_after
        && left.coupling_after == right.coupling_after
        && left.generation_after == right.generation_after
        && left.live_after == right.live_after
        && left.learned_traversals == right.learned_traversals
        && left.learned_output_firings == right.learned_output_firings
        && left.learned_support_increases == right.learned_support_increases
        && left.aggregate_returns == right.aggregate_returns
        && left.distractor_firings == right.distractor_firings
        && left.complete_fingerprint == right.complete_fingerprint
        && left.permanent_fingerprint == right.permanent_fingerprint
        && left.arrows == right.arrows
        && left.bytes == right.bytes
        && left.work == right.work
        && left.quiescent == right.quiescent
}

fn control_row(matter: &Matter, name: &'static str, passed: bool, note: &str) -> Row {
    row_from_matter(
        matter,
        "control",
        name.to_string(),
        name,
        passed,
        ([0; 6], [0; 6], [0; 6], [false; 6]),
        note.to_string(),
    )
}

fn controls() -> Vec<Row> {
    let mut rows = Vec::new();

    let mut mature = build(0xd400_0000, Variant::default());
    train(&mut mature, (0, 1), (2, 3), 12, 0, 20, 4);
    let singleton_pass = (0..4).all(|index| {
        use_once(&mature, &[index], None)
            .outputs
            .iter()
            .sum::<u64>()
            == 0
    });
    rows.push(control_row(
        &mature,
        "singletons",
        singleton_pass,
        "A|B|C|D output zero",
    ));

    let mut late = build(0xd401_0000, Variant::default());
    late.occurrence(&[0], 0, false, None);
    let late_seen = late.occurrence(&[1], 6, false, Some(0));
    rows.push(control_row(
        &late,
        "too-late",
        late_seen.loci[0] == 0 && late_seen.outputs.iter().sum::<u64>() == 0,
        "A@0 B@6",
    ));

    let mut correlation = build(0xd402_0000, Variant::default());
    correlation.enter_primes(2);
    correlation.enter_returns(2);
    let execution = correlation.substrate.propagate();
    let seen = correlation.record(execution, None);
    rows.push(control_row(
        &correlation,
        "correlation-without-traversal",
        seen.loci.iter().sum::<u64>() == 0 && seen.outputs.iter().sum::<u64>() == 0,
        "global prime and return only",
    ));

    let mut no_return = build(0xd403_0000, Variant::default());
    for round in 0..8 {
        no_return.occurrence(&[0, 1], round * 20, false, Some(0));
    }
    no_return.advance(180);
    let blocked = use_once(&no_return, &[0, 1], Some(0));
    rows.push(control_row(
        &no_return,
        "traversal-without-return",
        blocked.outputs[0] == 0 && no_return.selected_arrow(0).is_none(),
        "weak output removed",
    ));

    let mut absent = build(
        0xd404_0000,
        Variant {
            output_offset: 8,
            ..Variant::default()
        },
    );
    for round in 0..4 {
        absent.occurrence(&[0, 1], round * 20, true, Some(0));
    }
    let absent_seen = use_once(&absent, &[0, 1], Some(0));
    rows.push(control_row(
        &absent,
        "absent-opportunity",
        absent
            .substrate
            .arrows_between(absent.loci[0], absent.outputs[0])
            .is_empty()
            && absent_seen.outputs[0] == 0,
        "output beyond local radius",
    ));

    let mut stale = build(0xd405_0000, Variant::default());
    stale.occurrence(&[0, 1], 0, true, Some(0));
    stale.occurrence(&[0, 1], 20, true, Some(0));
    stale.advance(160);
    let stale_seen = use_once(&stale, &[0, 1], Some(0));
    rows.push(control_row(
        &stale,
        "stale-path",
        stale.selected_arrow(0).is_none() && stale_seen.outputs[0] == 0,
        "ordinary pressure deallocation",
    ));

    let genuine = use_once(&mature, &[0, 3], Some(2));
    rows.push(control_row(
        &mature,
        "genuine-current-participation",
        genuine.loci[2] == 1 && genuine.outputs[2] == 0,
        "fresh locus fires without mature output",
    ));

    let three = use_once(&mature, &[0, 1, 2], None);
    let four = use_once(&mature, &[0, 1, 2, 3], None);
    rows.push(control_row(
        &mature,
        "ambiguity-and-four-way",
        three.outputs[0] == 1
            && three.outputs.iter().sum::<u64>() == 1
            && four.outputs[0] == 1
            && four.outputs[4] == 1
            && four.outputs.iter().sum::<u64>() == 2,
        "only mature physical incidence emits",
    ));

    let mut bootstrap = build(0xd406_0000, Variant::default());
    bootstrap.occurrence(&[0, 1], 0, false, Some(0));
    bootstrap.advance(60);
    let dead = bootstrap.selected_arrow(0).is_none();
    let first = bootstrap.occurrence(&[0, 1], 64, true, Some(0));
    let formed = bootstrap.support(0).live && bootstrap.support(0).coupling == 2;
    let heldout = use_once(&bootstrap, &[0, 1], Some(0));
    rows.push(control_row(
        &bootstrap,
        "full-deallocation-bootstrap",
        dead && formed && first.outputs[0] == 0 && heldout.outputs[0] == 1,
        "new generation forms before mature output",
    ));

    let self_check = repeated_singleton(&mature, 0, 0);
    rows.push(control_row(
        &mature,
        "self-evidence",
        self_check.traversals == 0
            && self_check.outputs == 0
            && self_check.increases == 0
            && self_check.after.resistance < self_check.before.resistance
            && self_check.aggregate_returns > 0,
        &format!(
            "learned={}->{} aggregate={}",
            self_check.before.resistance, self_check.after.resistance, self_check.aggregate_returns
        ),
    ));

    let mut recurrence = mature.clone();
    let start = recurrence.substrate.current_tick() + 4;
    let mut outputs = 0;
    let before_routes = recurrence.routes;
    for round in 0..8 {
        let seen = recurrence.occurrence(&[0, 1], start + round * 20, true, Some(0));
        outputs += seen.outputs[0];
    }
    let scheduled = recurrence.routes[0] - before_routes[0] == 8
        && recurrence.routes[1] - before_routes[1] == 8;
    rows.push(control_row(
        &recurrence,
        "useful-recurrence",
        outputs == 8 && scheduled && recurrence.quiescent,
        "eight scheduled outputs and no autonomous source",
    ));
    rows
}

fn add_arrow(substrate: &mut PlasticSubstrate, from: CellId, to: CellId) {
    substrate.add_arrow(ArrowSpec {
        from,
        to,
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance: 10_000,
    });
}

fn activate(substrate: &mut PlasticSubstrate, cell: CellId, tick: i64, origin: u64) {
    for (phase, impulse, offset) in [(-2, 1, 0), (-1, 2, 1)] {
        substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase,
            origin_physical: origin + offset,
            target: cell,
            impulse,
        });
    }
}

fn recursion_row() -> Row {
    let namespace = 0xd500_0000;
    let mut substrate = PlasticSubstrate::new();
    let mut primitive = Vec::new();
    for index in 0..4 {
        primitive.push(substrate.add_cell(CellSpec {
            physical_id: namespace + 0x100 + index,
            position: 1_000 + index as i32 * 20,
            region: 0,
            threshold: 3,
            resistance: 10_000,
        }));
    }
    let mut locus = Vec::new();
    let mut output = Vec::new();
    for index in 0..3 {
        locus.push(substrate.add_cell(CellSpec {
            physical_id: namespace + 0x200 + index,
            position: index as i32 * 20,
            region: 0,
            threshold: 2,
            resistance: 10_000,
        }));
        output.push(substrate.add_cell(CellSpec {
            physical_id: namespace + 0x300 + index,
            position: index as i32 * 20 + 1,
            region: 1,
            threshold: 3,
            resistance: 10_000,
        }));
    }
    add_arrow(&mut substrate, primitive[0], locus[0]);
    add_arrow(&mut substrate, primitive[1], locus[0]);
    add_arrow(&mut substrate, output[0], locus[1]);
    add_arrow(&mut substrate, primitive[2], locus[1]);
    add_arrow(&mut substrate, output[1], locus[2]);
    add_arrow(&mut substrate, primitive[3], locus[2]);
    let output_physical = [namespace + 0x300, namespace + 0x301, namespace + 0x302];
    let mut observed = [0u64; 3];
    let mut work = 0;
    let mut returns = 0;
    let mut quiescent = true;
    for round in 0..8 {
        let base = round * 20;
        for (index, tick) in [(0, base), (1, base), (2, base + 2), (3, base + 4)] {
            activate(
                &mut substrate,
                primitive[index],
                tick,
                namespace + 0x400 + index as u64 * 2,
            );
        }
        for (index, tick) in [(0, base + 2), (1, base + 4), (2, base + 6)] {
            substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase: -20,
                origin_physical: namespace + 0x600 + index as u64,
                target: output[index],
                impulse: 1,
            });
            substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase: 20,
                origin_physical: namespace + 0x700 + index as u64,
                target: locus[index],
                impulse: 1,
            });
        }
        let execution = substrate.propagate();
        work += execution.work.total();
        returns += execution.work.local_return_updates;
        quiescent &= execution.naturally_quiescent;
        for entry in execution.trace.iter().filter(|entry| entry.fired) {
            if let Some(index) = output_physical
                .iter()
                .position(|physical| *physical == entry.target_physical)
            {
                observed[index] += 1;
            }
        }
    }
    let heldout_start = substrate.current_tick() + 14;
    for (index, tick) in [
        (0, heldout_start),
        (1, heldout_start),
        (2, heldout_start + 2),
        (3, heldout_start + 4),
    ] {
        activate(
            &mut substrate,
            primitive[index],
            tick,
            namespace + 0xa00 + index as u64 * 2,
        );
    }
    for (index, tick) in [
        (0, heldout_start + 2),
        (1, heldout_start + 4),
        (2, heldout_start + 6),
    ] {
        substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: -20,
            origin_physical: namespace + 0xb00 + index as u64,
            target: output[index],
            impulse: 1,
        });
    }
    let heldout = substrate.propagate();
    work += heldout.work.total();
    quiescent &= heldout.naturally_quiescent;
    let heldout_z = heldout
        .trace
        .iter()
        .filter(|entry| entry.fired && entry.target_physical == output_physical[2])
        .count() as u64;
    let passed = observed == [7, 6, 5] && heldout_z == 1 && quiescent;
    Row {
        category: "recursion",
        cell: "three-deep".to_string(),
        scenario: "A+B->X;X+C->Y;Y+D->Z",
        passed,
        routes: [8; 4],
        loci: [8, 7, 6, 0, 0, 0],
        outputs: [observed[0], observed[1], observed[2], 0, 0, 0],
        support_before: [0; 6],
        support_after: [0; 6],
        coupling_after: [0; 6],
        generation_after: [0; 6],
        live_after: [false; 6],
        learned_traversals: 0,
        learned_output_firings: heldout_z,
        learned_support_increases: 0,
        aggregate_returns: returns,
        distractor_firings: 0,
        complete_fingerprint: substrate.complete_fingerprint(),
        permanent_fingerprint: substrate.permanent_fingerprint(),
        arrows: substrate.arrow_count(),
        bytes: substrate.persistent_bytes(),
        work,
        quiescent,
        duplicate_exact: true,
        note: format!(
            "training={}|{}|{};heldout_z={heldout_z}",
            observed[0], observed[1], observed[2]
        ),
    }
}

fn or_row() -> Row {
    let namespace = 0xd600_0000;
    let mut base = PlasticSubstrate::new();
    let a = base.add_cell(CellSpec {
        physical_id: namespace + 1,
        position: 0,
        region: 0,
        threshold: 3,
        resistance: 10_000,
    });
    let b = base.add_cell(CellSpec {
        physical_id: namespace + 2,
        position: 10,
        region: 0,
        threshold: 3,
        resistance: 10_000,
    });
    let c = base.add_cell(CellSpec {
        physical_id: namespace + 3,
        position: 20,
        region: 1,
        threshold: 1,
        resistance: 10_000,
    });
    add_arrow(&mut base, a, c);
    add_arrow(&mut base, b, c);
    let mut counts = Vec::new();
    let mut work = 0;
    let mut quiescent = true;
    for active in [vec![a], vec![b], vec![a, b]] {
        let mut substrate = base.clone();
        for (index, cell) in active.into_iter().enumerate() {
            activate(
                &mut substrate,
                cell,
                0,
                namespace + 0x100 + index as u64 * 2,
            );
        }
        let execution = substrate.propagate();
        work += execution.work.total();
        quiescent &= execution.naturally_quiescent;
        counts.push(
            execution
                .trace
                .iter()
                .filter(|entry| entry.fired && entry.target_physical == namespace + 3)
                .count() as u64,
        );
    }
    let passed = counts == [1, 1, 1] && quiescent;
    Row {
        category: "reachability",
        cell: "convergent-arrows".to_string(),
        scenario: "A->C;B->C;A+B->C",
        passed,
        routes: [1, 1, 2, 0],
        loci: [0; 6],
        outputs: [counts[0], counts[1], counts[2], 0, 0, 0],
        support_before: [0; 6],
        support_after: [0; 6],
        coupling_after: [0; 6],
        generation_after: [0; 6],
        live_after: [false; 6],
        learned_traversals: 0,
        learned_output_firings: counts.iter().sum(),
        learned_support_increases: 0,
        aggregate_returns: 0,
        distractor_firings: 0,
        complete_fingerprint: base.complete_fingerprint(),
        permanent_fingerprint: base.permanent_fingerprint(),
        arrows: base.arrow_count(),
        bytes: base.persistent_bytes(),
        work,
        quiescent,
        duplicate_exact: true,
        note: format!("A={};B={};both={}", counts[0], counts[1], counts[2]),
    }
}

fn timing_row(
    namespace: u64,
    cell: &'static str,
    scenario: &'static str,
    a_tick: i64,
    b_tick: Option<i64>,
    reverse_arrival: bool,
    expected: u64,
) -> Row {
    let mut matter = build(
        namespace,
        Variant {
            reverse_arrival,
            ..Variant::default()
        },
    );
    matter.enter_participant(0, a_tick);
    if let Some(tick) = b_tick {
        matter.enter_participant(1, tick);
    }
    let prime_tick = b_tick.unwrap_or(a_tick) + 2;
    matter.enter_primes(prime_tick);
    let execution = matter.substrate.propagate();
    let seen = matter.record(execution, Some(0));
    if b_tick.is_none() {
        matter.advance(prime_tick + 10);
    }
    let actual = seen.loci[0];
    let passed = actual == expected && seen.quiescent && seen.outputs[0] == 0;
    let mut row = row_from_matter(
        &matter,
        "timing",
        cell.to_string(),
        scenario,
        passed,
        ([0; 6], [0; 6], [0; 6], [false; 6]),
        format!("A={a_tick};B={b_tick:?};locus={actual};expected={expected}"),
    );
    row.learned_traversals = seen.learned_traversals;
    row
}

fn temporal_rows() -> Vec<Row> {
    vec![
        timing_row(0xd700_0000, "together", "together", 0, Some(0), false, 1),
        timing_row(
            0xd701_0000,
            "same-tick-order",
            "A-then-B-within-tick",
            0,
            Some(0),
            false,
            1,
        ),
        timing_row(
            0xd702_0000,
            "overlap",
            "overlap-while-live",
            0,
            Some(0),
            true,
            1,
        ),
        timing_row(
            0xd703_0000,
            "one-tick-delay",
            "A-then-B-after-decay",
            0,
            Some(1),
            false,
            0,
        ),
        timing_row(
            0xd704_0000,
            "B-absent",
            "B-absent-before-closure",
            0,
            None,
            false,
            0,
        ),
        timing_row(
            0xd705_0000,
            "post-closure",
            "B-later-than-closure",
            0,
            Some(6),
            false,
            0,
        ),
    ]
}

fn frozen_inputs_pass() -> bool {
    sha256(Path::new("crates/px0-physical-correspondence/src/lib.rs"))
        .is_some_and(|hash| hash == LAW_SHA256)
        && sha256(Path::new(
            "arms/cj0-e-transient-coincidence/src/bin/cj0_e.rs",
        ))
        .is_some_and(|hash| hash == V1_EVALUATOR_SHA256)
        && sha256(Path::new("arms/cj0-e-transient-coincidence-v2/src/main.rs"))
            .is_some_and(|hash| hash == V2_EVALUATOR_SHA256)
        && sha256(Path::new("results/cj0_e_transient_coincidence_micro_v2.md"))
            .is_some_and(|hash| hash == MICRO_V2_MD_SHA256)
        && fs::read_to_string("results/cj0_e_transient_coincidence_micro_v2.md")
            .is_ok_and(|text| text.contains("Verdict: **PASS**"))
}

fn sha256(path: &Path) -> Option<String> {
    let output = Command::new("sha256sum").arg(path).output().ok()?;
    output.status.success().then(|| {
        String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_string()
    })
}

fn run_gate() -> Report {
    let mut rows = Vec::new();
    let mut flat_pass = true;
    let mut duplicate_pass = true;
    for seed in 0..8 {
        for stratum in 0..4 {
            let mut primary = flat_run(seed, stratum, false);
            let mut duplicate = flat_run(seed, stratum, true);
            let exact = rows_exact(&primary, &duplicate);
            primary.duplicate_exact = exact;
            duplicate.duplicate_exact = exact;
            primary.passed &= exact;
            duplicate.passed &= exact;
            flat_pass &= primary.passed && duplicate.passed;
            duplicate_pass &= exact;
            rows.push(primary);
            rows.push(duplicate);
        }
    }
    let controls = controls();
    let controls_pass = controls.iter().all(|row| row.passed);
    rows.extend(controls);
    let recursion = recursion_row();
    let recursion_pass = recursion.passed;
    rows.push(recursion);
    let reachability = or_row();
    let reachability_pass = reachability.passed;
    rows.push(reachability);
    let timing = temporal_rows();
    let timing_pass = timing.iter().all(|row| row.passed);
    rows.extend(timing);
    let all_quiescent = rows.iter().all(|row| row.quiescent);
    Report {
        rows,
        clauses: vec![
            (
                "frozen inputs and MICRO v2 eligibility",
                frozen_inputs_pass(),
            ),
            ("64 flat executions", flat_pass),
            ("32 exact duplicate comparisons", duplicate_pass),
            ("common controls", controls_pass),
            ("same-law three-deep recursion", recursion_pass),
            ("ordinary convergent reachability", reachability_pass),
            ("temporal expressivity", timing_pass),
            ("all rows naturally quiescent", all_quiescent),
        ],
    }
}

fn join<T: ToString>(values: &[T]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn csv(report: &Report) -> String {
    let mut out = "protocol,category,cell,scenario,passed,routes,loci,outputs,support_before,support_after,coupling_after,generation_after,live_after,learned_traversals,learned_output_firings,learned_support_increases,aggregate_returns,distractor_firings,complete_fingerprint,permanent_fingerprint,arrows,persistent_bytes,work,quiescent,duplicate_exact,note\n".to_string();
    for row in &report.rows {
        out.push_str(&format!(
            "{PROTOCOL},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:016x},{:016x},{},{},{},{},{},{}\n",
            row.category,
            row.cell,
            row.scenario,
            row.passed,
            join(&row.routes),
            join(&row.loci),
            join(&row.outputs),
            join(&row.support_before),
            join(&row.support_after),
            join(&row.coupling_after),
            join(&row.generation_after),
            join(&row.live_after),
            row.learned_traversals,
            row.learned_output_firings,
            row.learned_support_increases,
            row.aggregate_returns,
            row.distractor_firings,
            row.complete_fingerprint,
            row.permanent_fingerprint,
            row.arrows,
            row.bytes,
            row.work,
            row.quiescent,
            row.duplicate_exact,
            row.note.replace(',', ";"),
        ));
    }
    out
}

fn markdown(report: &Report) -> String {
    let verdict = if report.passed() {
        "PASS"
    } else {
        "FIRST_CLAUSE_FAILURE"
    };
    let mut out = format!(
        "# CJ0-E transient coincidence development GATE v1 result\n\nVerdict: **{verdict}**. Development ends at this result.\n\n| clause | pass |\n|---|:---:|\n"
    );
    for (clause, passed) in &report.clauses {
        out.push_str(&format!("| {clause} | {passed} |\n"));
    }
    let flat = report
        .rows
        .iter()
        .filter(|row| row.category == "flat")
        .count();
    let controls = report
        .rows
        .iter()
        .filter(|row| row.category == "control")
        .count();
    out.push_str(&format!(
        "\nRows: `{}` (`{flat}` flat, `{controls}` controls). Ledgered serialized work: `{}`. The CSV contains all fingerprints, physical support, learned-specific evidence, aggregate returns, distractor activity, storage, and quiescence.\n",
        report.rows.len(),
        report.work()
    ));
    out
}

fn write_new(path: &Path, contents: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()
}

fn publish(report: &Report) -> io::Result<()> {
    let csv_path = PathBuf::from(CSV_PATH);
    let md_path = PathBuf::from(MD_PATH);
    let csv_staging = csv_path.with_extension("csv.staging");
    let md_staging = md_path.with_extension("md.staging");
    for path in [&csv_path, &md_path, &csv_staging, &md_staging] {
        if path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("existing artifact {}", path.display()),
            ));
        }
    }
    write_new(&csv_staging, &csv(report))?;
    write_new(&md_staging, &markdown(report))?;
    fs::rename(csv_staging, csv_path)?;
    fs::rename(md_staging, md_path)
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [arg] if arg == "--preflight" => {
            println!(
                "protocol={PROTOCOL} preflight={} no_cell=true development_only=true",
                frozen_inputs_pass()
            );
        }
        [arg] if arg == "--gate" && frozen_inputs_pass() => {
            let report = run_gate();
            println!(
                "protocol={PROTOCOL} pass={} rows={} work={} development_complete=true",
                report.passed(),
                report.rows.len(),
                report.work()
            );
            for (clause, passed) in &report.clauses {
                println!("clause={clause} pass={passed}");
            }
            if let Err(error) = publish(&report) {
                eprintln!("publication failed: {error}");
                std::process::exit(2);
            }
            if !report.passed() {
                std::process::exit(1);
            }
        }
        [arg] if arg == "--gate" => {
            eprintln!("refusing GATE: frozen input or MICRO v2 PASS missing");
            std::process::exit(2);
        }
        _ => {
            eprintln!("usage: cj0-e-transient-coincidence-gate --preflight|--gate");
            std::process::exit(2);
        }
    }
}
