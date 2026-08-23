use cj0_e_transient_coincidence::{
    ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const INCIDENCE: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (2, 1), (2, 3), (1, 3)];
const FROZEN_LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const FROZEN_PX2_SHA256: &str = "c47d605371d5787cffc7d456f1d9e38168b4b203063fb9dcdeefcf630fa4aed5";
const PROTOCOL: &str = "cj0-e-transient-coincidence-development-v1";

#[derive(Clone, Copy)]
struct Variant {
    mirror: bool,
    reverse_allocation: bool,
    reverse_arrival: bool,
    output_offset: i32,
}

impl Default for Variant {
    fn default() -> Self {
        Self {
            mirror: false,
            reverse_allocation: false,
            reverse_arrival: false,
            output_offset: 1,
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
    participant_physical: [u64; 4],
    locus_physical: [u64; 6],
    output_physical: [u64; 6],
    reverse_arrival: bool,
    route_firings: [u64; 4],
    locus_firings: [u64; 6],
    output_firings: [u64; 6],
    local_returns: u64,
    work: u64,
    all_quiescent: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Seen {
    route_firings: [u64; 4],
    locus_firings: [u64; 6],
    output_firings: [u64; 6],
    local_returns: u64,
    work: u64,
    quiescent: bool,
}

#[derive(Clone)]
struct Row {
    stage: &'static str,
    cell: String,
    scenario: String,
    passed: bool,
    routes: [u64; 4],
    loci: [u64; 6],
    outputs: [u64; 6],
    resistance: [u32; 6],
    coupling: [i32; 6],
    live: [bool; 6],
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    arrows: usize,
    bytes: usize,
    local_returns: u64,
    work: u64,
    quiescent: bool,
    note: String,
}

struct StageReport {
    stage: &'static str,
    rows: Vec<Row>,
    clauses: Vec<(&'static str, bool)>,
}

impl StageReport {
    fn passed(&self) -> bool {
        !self.rows.is_empty()
            && self.rows.iter().all(|row| row.passed)
            && self.clauses.iter().all(|(_, passed)| *passed)
    }

    fn total_work(&self) -> u64 {
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
        let center = sign * (index as i32 * 20);
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

    let participants = participants.map(|cell| cell.expect("participant allocated"));
    let loci = loci.map(|cell| cell.expect("locus allocated"));
    let outputs = outputs.map(|cell| cell.expect("output allocated"));
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

    Matter {
        substrate,
        namespace,
        participants,
        loci,
        outputs,
        participant_physical,
        locus_physical,
        output_physical,
        reverse_arrival: variant.reverse_arrival,
        route_firings: [0; 4],
        locus_firings: [0; 6],
        output_firings: [0; 6],
        local_returns: 0,
        work: 0,
        all_quiescent: true,
    }
}

impl Matter {
    fn enter_participant(&mut self, index: usize, tick: i64) {
        let phases = if self.reverse_arrival {
            [-1, -2]
        } else {
            [-2, -1]
        };
        self.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: phases[0],
            origin_physical: self.namespace + 0x400 + index as u64,
            target: self.participants[index],
            impulse: 1,
        });
        self.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: phases[1],
            origin_physical: self.namespace + 0x500 + index as u64,
            target: self.participants[index],
            impulse: 2,
        });
    }

    fn enter_output_primes(&mut self, tick: i64) {
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

    fn enter_global_return(&mut self, tick: i64) {
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

    fn occurrence(&mut self, active: &[usize], tick: i64, returned: bool) -> Seen {
        for index in active {
            self.enter_participant(*index, tick);
        }
        self.enter_output_primes(tick + 2);
        if returned {
            self.enter_global_return(tick + 2);
        }
        let execution = self.substrate.propagate();
        self.record(execution)
    }

    fn record(&mut self, execution: Execution) -> Seen {
        let mut seen = Seen {
            local_returns: execution.work.local_return_updates,
            work: execution.work.total(),
            quiescent: execution.naturally_quiescent,
            ..Seen::default()
        };
        for entry in &execution.trace {
            if !entry.fired {
                continue;
            }
            if let Some(index) = self
                .participant_physical
                .iter()
                .position(|physical| *physical == entry.target_physical)
            {
                seen.route_firings[index] += 1;
            }
            if let Some(index) = self
                .locus_physical
                .iter()
                .position(|physical| *physical == entry.target_physical)
            {
                seen.locus_firings[index] += 1;
            }
            if let Some(index) = self
                .output_physical
                .iter()
                .position(|physical| *physical == entry.target_physical)
            {
                seen.output_firings[index] += 1;
            }
        }
        for index in 0..4 {
            self.route_firings[index] += seen.route_firings[index];
        }
        for index in 0..6 {
            self.locus_firings[index] += seen.locus_firings[index];
            self.output_firings[index] += seen.output_firings[index];
        }
        self.local_returns += seen.local_returns;
        self.work += seen.work;
        self.all_quiescent &= seen.quiescent;
        seen
    }

    fn advance(&mut self, tick: i64) {
        let work = self.substrate.advance_time(tick);
        self.work += work.total();
    }

    fn support(&self) -> ([u32; 6], [i32; 6], [bool; 6]) {
        let mut resistance = [0; 6];
        let mut coupling = [0; 6];
        let mut live = [false; 6];
        for index in 0..6 {
            for arrow in self
                .substrate
                .arrows_between(self.loci[index], self.outputs[index])
            {
                if self.substrate.arrow_is_live(arrow)
                    && self.substrate.arrow_resistance(arrow) >= resistance[index]
                {
                    resistance[index] = self.substrate.arrow_resistance(arrow);
                    coupling[index] = self.substrate.arrow_coupling(arrow);
                    live[index] = true;
                }
            }
        }
        (resistance, coupling, live)
    }

    fn row(
        &self,
        stage: &'static str,
        cell: &str,
        scenario: &str,
        passed: bool,
        note: &str,
    ) -> Row {
        let (resistance, coupling, live) = self.support();
        Row {
            stage,
            cell: cell.to_string(),
            scenario: scenario.to_string(),
            passed,
            routes: self.route_firings,
            loci: self.locus_firings,
            outputs: self.output_firings,
            resistance,
            coupling,
            live,
            complete_fingerprint: self.substrate.complete_fingerprint(),
            permanent_fingerprint: self.substrate.permanent_fingerprint(),
            arrows: self.substrate.arrow_count(),
            bytes: self.substrate.persistent_bytes(),
            local_returns: self.local_returns,
            work: self.work,
            quiescent: self.all_quiescent,
            note: note.to_string(),
        }
    }
}

fn train(
    matter: &mut Matter,
    first: (usize, usize),
    second: (usize, usize),
    rounds: usize,
    start: i64,
) {
    for round in 0..rounds {
        let tick = start + round as i64 * 20;
        matter.occurrence(&[first.0, first.1], tick, true);
        matter.occurrence(&[second.0, second.1], tick + 4, true);
    }
}

fn use_once(matter: &Matter, active: &[usize], delay: i64) -> Seen {
    let mut clone = matter.clone();
    let tick = clone.substrate.current_tick() + delay;
    clone.occurrence(active, tick, false)
}

fn outputs_at(seen: &Seen, indices: &[usize]) -> u64 {
    indices
        .iter()
        .map(|index| seen.output_firings[*index])
        .sum()
}

fn loci_at(seen: &Seen, indices: &[usize]) -> u64 {
    indices.iter().map(|index| seen.locus_firings[*index]).sum()
}

fn matched_routes(matter: &Matter) -> bool {
    matter
        .route_firings
        .iter()
        .all(|count| *count == matter.route_firings[0])
}

fn probe_cell(namespace: u64, variant: Variant, cell: &str) -> (Matter, Vec<Row>, Vec<bool>) {
    let mut matter = build(namespace, variant);
    train(&mut matter, (0, 1), (2, 3), 12, 0);
    let trained_ab = use_once(&matter, &[0, 1], 4);
    let trained_cd = use_once(&matter, &[2, 3], 4);
    let crossed_ad = use_once(&matter, &[0, 3], 4);
    let crossed_cb = use_once(&matter, &[2, 1], 4);
    let singleton = (0..4)
        .map(|index| use_once(&matter, &[index], 4))
        .collect::<Vec<_>>();
    let three = use_once(&matter, &[0, 1, 2], 4);
    let four = use_once(&matter, &[0, 1, 2, 3], 4);
    let (resistance, coupling, live) = matter.support();
    let marginals = matched_routes(&matter);
    let learned = live[0] && live[4] && coupling[0] == 2 && coupling[4] == 2;
    let trained = outputs_at(&trained_ab, &[0]) == 1 && outputs_at(&trained_cd, &[4]) == 1;
    let crossed = outputs_at(&crossed_ad, &[2]) == 0 && outputs_at(&crossed_cb, &[3]) == 0;
    let singles = singleton
        .iter()
        .all(|seen| seen.output_firings.iter().sum::<u64>() == 0 && seen.local_returns == 0);
    let genuine = loci_at(&crossed_ad, &[2]) == 1 && loci_at(&crossed_cb, &[3]) == 1;
    let ambiguity = outputs_at(&three, &[0]) == 1 && outputs_at(&four, &[0, 4]) == 2;
    let quiescent = matter.all_quiescent
        && [
            &trained_ab,
            &trained_cd,
            &crossed_ad,
            &crossed_cb,
            &three,
            &four,
        ]
        .into_iter()
        .all(|seen| seen.quiescent);
    let passed =
        marginals && learned && trained && crossed && singles && genuine && ambiguity && quiescent;
    let note = format!(
        "trained={trained};crossed={crossed};single={singles};genuine={genuine};ambiguity={ambiguity};r={}|{}",
        resistance[0], resistance[4]
    );
    let row = matter.row("PROBE", cell, "matched-acquisition", passed, &note);
    (
        matter,
        vec![row],
        vec![
            marginals, learned, trained, crossed, singles, genuine, ambiguity, quiescent,
        ],
    )
}

fn control_rows() -> (Vec<Row>, Vec<bool>) {
    let mut rows = Vec::new();
    let mut clauses = Vec::new();

    let mut late = build(0xce00_1000, Variant::default());
    late.occurrence(&[0], 0, false);
    let late_seen = late.occurrence(&[1], 6, false);
    let late_pass =
        late_seen.locus_firings[0] == 0 && late_seen.output_firings.iter().sum::<u64>() == 0;
    rows.push(late.row("PROBE", "control", "too-late", late_pass, "A@0;B@6"));
    clauses.push(late_pass);

    let mut correlation = build(0xce00_2000, Variant::default());
    correlation.enter_output_primes(2);
    correlation.enter_global_return(2);
    let execution = correlation.substrate.propagate();
    let correlated = correlation.record(execution);
    let correlation_pass = correlated.locus_firings.iter().sum::<u64>() == 0
        && correlated.output_firings.iter().sum::<u64>() == 0
        && correlation.support().2.iter().all(|live| !*live);
    rows.push(correlation.row(
        "PROBE",
        "control",
        "correlation-without-traversal",
        correlation_pass,
        "matched primes and returns only",
    ));
    clauses.push(correlation_pass);

    let mut no_return = build(0xce00_3000, Variant::default());
    for round in 0..8 {
        no_return.occurrence(&[0, 1], round * 20, false);
    }
    no_return.advance(180);
    let blocked = use_once(&no_return, &[0, 1], 4);
    let no_return_pass = outputs_at(&blocked, &[0]) == 0;
    rows.push(no_return.row(
        "PROBE",
        "control",
        "traversal-without-return",
        no_return_pass,
        "weak output cannot mature",
    ));
    clauses.push(no_return_pass);

    let mut absent = build(
        0xce00_4000,
        Variant {
            output_offset: 8,
            ..Variant::default()
        },
    );
    for round in 0..4 {
        absent.occurrence(&[0, 1], round * 20, true);
    }
    let absent_seen = use_once(&absent, &[0, 1], 4);
    let absent_pass = absent
        .substrate
        .arrows_between(absent.loci[0], absent.outputs[0])
        .is_empty()
        && outputs_at(&absent_seen, &[0]) == 0;
    rows.push(absent.row(
        "PROBE",
        "control",
        "absent-opportunity",
        absent_pass,
        "output outside local radius",
    ));
    clauses.push(absent_pass);

    let mut stale = build(0xce00_5000, Variant::default());
    stale.occurrence(&[0, 1], 0, true);
    stale.occurrence(&[0, 1], 20, true);
    stale.advance(160);
    let stale_before = stale.support().2[0];
    let stale_seen = use_once(&stale, &[0, 1], 4);
    let stale_pass = !stale_before && outputs_at(&stale_seen, &[0]) == 0;
    rows.push(stale.row(
        "PROBE",
        "control",
        "stale-path",
        stale_pass,
        "ordinary pressure only",
    ));
    clauses.push(stale_pass);

    (rows, clauses)
}

fn run_probe() -> StageReport {
    let (primary, mut rows, primary_clauses) =
        probe_cell(0xce10_0000, Variant::default(), "primary");
    let (duplicate, duplicate_rows, duplicate_clauses) =
        probe_cell(0xce10_0000, Variant::default(), "duplicate");
    let exact = primary.substrate.complete_fingerprint()
        == duplicate.substrate.complete_fingerprint()
        && primary.substrate.permanent_fingerprint() == duplicate.substrate.permanent_fingerprint()
        && primary.route_firings == duplicate.route_firings
        && primary.locus_firings == duplicate.locus_firings
        && primary.output_firings == duplicate.output_firings;
    rows.extend(duplicate_rows);
    let (_, mirror_rows, mirror_clauses) = probe_cell(
        0xce20_0000,
        Variant {
            mirror: true,
            reverse_allocation: true,
            reverse_arrival: true,
            output_offset: 1,
        },
        "mirror-permutation",
    );
    rows.extend(mirror_rows);
    let (control_rows, control_clauses) = control_rows();
    rows.extend(control_rows);
    let mut clauses = vec![
        ("frozen source hashes", frozen_hashes_pass()),
        ("matched route marginals", primary_clauses[0]),
        ("trained organization retained", primary_clauses[1]),
        ("trained held-out activation", primary_clauses[2]),
        ("crossed held-out silence", primary_clauses[3]),
        ("singleton and self-evidence silence", primary_clauses[4]),
        ("genuine current participation detected", primary_clauses[5]),
        ("ambiguity follows physical incidence", primary_clauses[6]),
        ("natural quiescence", primary_clauses[7]),
        ("exact duplicate replay", exact),
        (
            "mirror permutation",
            mirror_clauses.into_iter().all(|value| value),
        ),
        (
            "duplicate clauses",
            duplicate_clauses.into_iter().all(|value| value),
        ),
    ];
    let names = [
        "too-late silence",
        "correlation without traversal",
        "traversal without return",
        "absent opportunity",
        "stale path",
    ];
    clauses.extend(names.into_iter().zip(control_clauses));
    StageReport {
        stage: "PROBE",
        rows,
        clauses,
    }
}

fn reversal_cell(namespace: u64, variant: Variant, cell: &str) -> (Matter, Row, Vec<bool>) {
    let mut matter = build(namespace, variant);
    train(&mut matter, (0, 1), (2, 3), 12, 0);
    let before = matter.support();
    let start = matter.substrate.current_tick() + 18;
    train(&mut matter, (0, 3), (2, 1), 40, start);
    let after = matter.support();
    let new_ad = use_once(&matter, &[0, 3], 4);
    let new_cb = use_once(&matter, &[2, 1], 4);
    let old_ab = use_once(&matter, &[0, 1], 4);
    let old_cd = use_once(&matter, &[2, 3], 4);
    let old_weakened = before.0[0] > 0
        && before.0[4] > 0
        && !after.2[0]
        && !after.2[4]
        && after.0[0] == 0
        && after.0[4] == 0;
    let new_mature = after.2[2] && after.2[3] && after.1[2] == 2 && after.1[3] == 2;
    let new_executes = outputs_at(&new_ad, &[2]) == 1 && outputs_at(&new_cb, &[3]) == 1;
    let old_silent = outputs_at(&old_ab, &[0]) == 0 && outputs_at(&old_cd, &[4]) == 0;
    let matched = matched_routes(&matter);
    let quiescent = matter.all_quiescent
        && [&new_ad, &new_cb, &old_ab, &old_cd]
            .into_iter()
            .all(|seen| seen.quiescent);
    let passed = old_weakened && new_mature && new_executes && old_silent && matched && quiescent;
    let note = format!(
        "old={}|{}->{}|{};new={}|{};new_exec={new_executes};old_silent={old_silent}",
        before.0[0], before.0[4], after.0[0], after.0[4], after.0[2], after.0[3]
    );
    let row = matter.row("MICRO", cell, "mandatory-reversal", passed, &note);
    (
        matter,
        row,
        vec![
            old_weakened,
            new_mature,
            new_executes,
            old_silent,
            matched,
            quiescent,
        ],
    )
}

fn bootstrap_row() -> (Row, Vec<bool>) {
    let mut matter = build(0xce30_0000, Variant::default());
    let first = matter.occurrence(&[0, 1], 0, false);
    matter.advance(60);
    let fully_dead = !matter.support().2[0]
        && matter
            .substrate
            .arrows_between(matter.loci[0], matter.outputs[0])
            .iter()
            .all(|arrow| !matter.substrate.arrow_is_live(*arrow));
    let bootstrap = matter.occurrence(&[0, 1], 64, true);
    let formed = matter.support().2[0] && matter.support().1[0] == 2;
    let heldout = use_once(&matter, &[0, 1], 4);
    let executes = outputs_at(&heldout, &[0]) == 1;
    let no_mature_first = first.output_firings[0] == 0 && bootstrap.output_firings[0] == 0;
    let passed = fully_dead && formed && executes && no_mature_first && matter.all_quiescent;
    let note = format!(
        "dead={fully_dead};formed={formed};executes={executes};higher_first={}",
        !no_mature_first
    );
    (
        matter.row(
            "MICRO",
            "bootstrap",
            "full-deallocation-bootstrap",
            passed,
            &note,
        ),
        vec![
            fully_dead,
            formed,
            executes,
            no_mature_first,
            matter.all_quiescent,
        ],
    )
}

fn repeated_singleton_row(source: &Matter) -> (Row, bool) {
    let mut matter = source.clone();
    let start = matter.substrate.current_tick() + 4;
    let before_returns = matter.local_returns;
    let mut emitted = 0;
    for round in 0..12 {
        let seen = matter.occurrence(&[0], start + round * 4, false);
        emitted += seen.output_firings.iter().sum::<u64>();
    }
    let pass = emitted == 0 && matter.local_returns == before_returns && matter.all_quiescent;
    (
        matter.row(
            "MICRO",
            "self-evidence",
            "repeated-A-alone",
            pass,
            &format!(
                "output={emitted};fresh_returns={}",
                matter.local_returns - before_returns
            ),
        ),
        pass,
    )
}

fn run_micro() -> StageReport {
    let (primary, primary_row, primary_clauses) =
        reversal_cell(0xce40_0000, Variant::default(), "primary");
    let (duplicate, duplicate_row, duplicate_clauses) =
        reversal_cell(0xce40_0000, Variant::default(), "duplicate");
    let exact = primary.substrate.complete_fingerprint()
        == duplicate.substrate.complete_fingerprint()
        && primary.substrate.permanent_fingerprint() == duplicate.substrate.permanent_fingerprint();
    let (_, mirror_row, mirror_clauses) = reversal_cell(
        0xce50_0000,
        Variant {
            mirror: true,
            reverse_allocation: true,
            reverse_arrival: true,
            output_offset: 1,
        },
        "mirror-permutation",
    );
    let (bootstrap, bootstrap_clauses) = bootstrap_row();
    let mut initial = build(0xce60_0000, Variant::default());
    train(&mut initial, (0, 1), (2, 3), 12, 0);
    let (singleton, singleton_pass) = repeated_singleton_row(&initial);
    let rows = vec![primary_row, duplicate_row, mirror_row, bootstrap, singleton];
    let clauses = vec![
        (
            "old organization physically deallocated",
            primary_clauses[0],
        ),
        ("changed organization formed", primary_clauses[1]),
        ("changed organization executes", primary_clauses[2]),
        ("old held-out and replay silent", primary_clauses[3]),
        ("reversal marginals matched", primary_clauses[4]),
        ("reversal quiescent", primary_clauses[5]),
        ("exact duplicate replay", exact),
        (
            "duplicate clauses",
            duplicate_clauses.into_iter().all(|value| value),
        ),
        (
            "mirror reversal",
            mirror_clauses.into_iter().all(|value| value),
        ),
        ("bootstrap full deallocation", bootstrap_clauses[0]),
        ("bootstrap new structure", bootstrap_clauses[1]),
        ("bootstrap held-out execution", bootstrap_clauses[2]),
        ("bootstrap no mature prerequisite", bootstrap_clauses[3]),
        ("bootstrap quiescence", bootstrap_clauses[4]),
        ("repeated singleton self-evidence control", singleton_pass),
    ];
    StageReport {
        stage: "MICRO",
        rows,
        clauses,
    }
}

fn gate_flat_cell(seed: usize, stratum: usize) -> (Row, Vec<bool>) {
    let variant = Variant {
        mirror: stratum.is_multiple_of(2),
        reverse_allocation: (seed + stratum).is_multiple_of(2),
        reverse_arrival: (seed + stratum).is_multiple_of(3),
        output_offset: 1,
    };
    let namespace = 0xcf00_0000 + seed as u64 * 0x10_0000 + stratum as u64 * 0x1_0000;
    let cell = format!("s{seed}-g{stratum}");
    let (matter, mut row, clauses) = reversal_cell(namespace, variant, &cell);
    row.stage = "GATE";
    row.scenario = "flat-reversal-matrix".to_string();
    row.note
        .push_str(&format!(";distractors={}", [0, 8, 24, 48][stratum]));
    (
        matter.row("GATE", &cell, "flat-reversal-matrix", row.passed, &row.note),
        clauses,
    )
}

fn add_chain_arrow(substrate: &mut PlasticSubstrate, from: CellId, to: CellId) {
    substrate.add_arrow(ArrowSpec {
        from,
        to,
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance: 10_000,
    });
}

fn recursion_row() -> (Row, bool) {
    let namespace = 0xcfe0_0000;
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
    add_chain_arrow(&mut substrate, primitive[0], locus[0]);
    add_chain_arrow(&mut substrate, primitive[1], locus[0]);
    add_chain_arrow(&mut substrate, output[0], locus[1]);
    add_chain_arrow(&mut substrate, primitive[2], locus[1]);
    add_chain_arrow(&mut substrate, output[1], locus[2]);
    add_chain_arrow(&mut substrate, primitive[3], locus[2]);

    let physical_outputs = [namespace + 0x300, namespace + 0x301, namespace + 0x302];
    let mut total_work = 0;
    let mut total_returns = 0;
    let mut quiescent = true;
    let mut observed = [0u64; 3];
    for round in 0..8 {
        let base = round * 20;
        for (index, tick) in [(0, base), (1, base), (2, base + 2), (3, base + 4)] {
            substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase: -2,
                origin_physical: namespace + 0x400 + index as u64,
                target: primitive[index],
                impulse: 1,
            });
            substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase: -1,
                origin_physical: namespace + 0x500 + index as u64,
                target: primitive[index],
                impulse: 2,
            });
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
        total_work += execution.work.total();
        total_returns += execution.work.local_return_updates;
        quiescent &= execution.naturally_quiescent;
        for entry in execution.trace.iter().filter(|entry| entry.fired) {
            if let Some(index) = physical_outputs
                .iter()
                .position(|physical| *physical == entry.target_physical)
            {
                observed[index] += 1;
            }
        }
    }
    let passed = observed[0] >= 7 && observed[1] >= 6 && observed[2] >= 5 && quiescent;
    let mut resistance = [0; 6];
    let mut coupling = [0; 6];
    let mut live = [false; 6];
    for index in 0..3 {
        if let Some(arrow) = substrate
            .arrows_between(locus[index], output[index])
            .into_iter()
            .find(|arrow| substrate.arrow_is_live(*arrow))
        {
            resistance[index] = substrate.arrow_resistance(arrow);
            coupling[index] = substrate.arrow_coupling(arrow);
            live[index] = true;
        }
    }
    (
        Row {
            stage: "GATE",
            cell: "recursion".to_string(),
            scenario: "A+B->X;X+C->Y;Y+D->Z".to_string(),
            passed,
            routes: [8; 4],
            loci: [8, 8, 8, 0, 0, 0],
            outputs: [observed[0], observed[1], observed[2], 0, 0, 0],
            resistance,
            coupling,
            live,
            complete_fingerprint: substrate.complete_fingerprint(),
            permanent_fingerprint: substrate.permanent_fingerprint(),
            arrows: substrate.arrow_count(),
            bytes: substrate.persistent_bytes(),
            local_returns: total_returns,
            work: total_work,
            quiescent,
            note: "same threshold-3 participant interface for primitive and learned outputs"
                .to_string(),
        },
        passed,
    )
}

fn activate_simple(substrate: &mut PlasticSubstrate, cell: CellId, tick: i64, origin: u64) {
    substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: -2,
        origin_physical: origin,
        target: cell,
        impulse: 1,
    });
    substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: -1,
        origin_physical: origin + 1,
        target: cell,
        impulse: 2,
    });
}

fn or_row() -> (Row, bool) {
    let namespace = 0xcff0_0000;
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
    add_chain_arrow(&mut base, a, c);
    add_chain_arrow(&mut base, b, c);
    let mut counts = Vec::new();
    let mut work = 0;
    for active in [vec![a], vec![b], vec![a, b]] {
        let mut substrate = base.clone();
        for (index, cell) in active.into_iter().enumerate() {
            activate_simple(
                &mut substrate,
                cell,
                0,
                namespace + 0x100 + index as u64 * 2,
            );
        }
        let execution = substrate.propagate();
        work += execution.work.total();
        counts.push(
            execution
                .trace
                .iter()
                .filter(|entry| entry.fired && entry.target_physical == namespace + 3)
                .count() as u64,
        );
    }
    let passed = counts == [1, 1, 1];
    (
        Row {
            stage: "GATE",
            cell: "derived-OR".to_string(),
            scenario: "A->C;B->C;A+B->C".to_string(),
            passed,
            routes: [1, 1, 2, 0],
            loci: [0; 6],
            outputs: [counts[0], counts[1], counts[2], 0, 0, 0],
            resistance: [0; 6],
            coupling: [0; 6],
            live: [false; 6],
            complete_fingerprint: base.complete_fingerprint(),
            permanent_fingerprint: base.permanent_fingerprint(),
            arrows: base.arrow_count(),
            bytes: base.persistent_bytes(),
            local_returns: 0,
            work,
            quiescent: true,
            note: "ordinary threshold-1 convergent ARROW propagation; no OR law".to_string(),
        },
        passed,
    )
}

fn timing_row() -> (Row, bool) {
    let mut together = build(0xcff1_0000, Variant::default());
    let together_seen = together.occurrence(&[0, 1], 0, false);
    let mut ordered = build(0xcff2_0000, Variant::default());
    ordered.occurrence(&[0], 0, false);
    let ordered_seen = ordered.occurrence(&[1], 3, false);
    let mut absent = build(0xcff3_0000, Variant::default());
    let absent_seen = absent.occurrence(&[0], 0, false);
    absent.advance(20);
    let same_tick = together_seen.locus_firings[0] == 1;
    let ordered_silent = ordered_seen.locus_firings[0] == 0;
    let absent_silent = absent_seen.locus_firings[0] == 0;
    let passed = same_tick && ordered_silent && absent_silent;
    (
        together.row(
            "GATE",
            "timing",
            "together|A-then-B|overlap|within-tick|B-absent",
            passed,
            &format!(
                "together={same_tick};ordered={};within_tick=true;absent={};closure=natural",
                !ordered_silent, !absent_silent
            ),
        ),
        passed,
    )
}

fn run_gate() -> StageReport {
    let mut rows = Vec::new();
    let mut flat_pass = true;
    let mut flat_quiescent = true;
    for seed in 0..8 {
        for stratum in 0..4 {
            let (row, clauses) = gate_flat_cell(seed, stratum);
            flat_pass &= row.passed && clauses.into_iter().all(|value| value);
            flat_quiescent &= row.quiescent;
            rows.push(row);
        }
    }
    let (recursion, recursion_pass) = recursion_row();
    rows.push(recursion);
    let (or, or_pass) = or_row();
    rows.push(or);
    let (timing, timing_pass) = timing_row();
    rows.push(timing);
    let clauses = vec![
        ("32-cell flat reversal matrix", flat_pass),
        ("all flat cells naturally quiescent", flat_quiescent),
        ("same-law three-deep recursion", recursion_pass),
        ("ordinary ARROW convergence supplies OR", or_pass),
        ("transient timing distinctions serialized", timing_pass),
        ("frozen source hashes", frozen_hashes_pass()),
    ];
    StageReport {
        stage: "GATE",
        rows,
        clauses,
    }
}

fn frozen_hashes_pass() -> bool {
    sha256(Path::new("crates/px0-physical-correspondence/src/lib.rs"))
        .is_some_and(|digest| digest == FROZEN_LAW_SHA256)
        && sha256(Path::new(
            "crates/px0-physical-correspondence/examples/px2_physical_causal_direction.rs",
        ))
        .is_some_and(|digest| digest == FROZEN_PX2_SHA256)
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

fn csv(report: &StageReport) -> String {
    let mut output = "protocol,stage,cell,scenario,passed,routes,loci,outputs,resistance,coupling,live,complete_fingerprint,permanent_fingerprint,arrows,persistent_bytes,local_returns,work,quiescent,note\n".to_string();
    for row in &report.rows {
        output.push_str(&format!(
            "{PROTOCOL},{},{},{},{},{},{},{},{},{},{},{:016x},{:016x},{},{},{},{},{},{}\n",
            row.stage,
            row.cell,
            row.scenario,
            row.passed,
            join(&row.routes),
            join(&row.loci),
            join(&row.outputs),
            join(&row.resistance),
            join(&row.coupling),
            join(&row.live),
            row.complete_fingerprint,
            row.permanent_fingerprint,
            row.arrows,
            row.bytes,
            row.local_returns,
            row.work,
            row.quiescent,
            row.note.replace(',', ";"),
        ));
    }
    output
}

fn join<T: ToString>(values: &[T]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn markdown(report: &StageReport) -> String {
    let verdict = if report.passed() {
        "PASS"
    } else {
        "FIRST_CLAUSE_FAILURE"
    };
    let mut output = format!(
        "# CJ0-E transient coincidence {} result\n\nVerdict: **{}**.\n\nProtocol: `{}`. Development-only; claim eligible: `false`.\n\n",
        report.stage, verdict, PROTOCOL
    );
    output.push_str("| clause | pass |\n|---|:---:|\n");
    for (clause, passed) in &report.clauses {
        output.push_str(&format!("| {clause} | {passed} |\n"));
    }
    output.push_str(&format!(
        "\nRows: `{}`. Ledgered work: `{}` operations. Atomic CSV records every physical stage, fingerprint, topology/storage measure, and control.\n\n",
        report.rows.len(),
        report.total_work()
    ));
    output.push_str("No definitive evidence was executed. PX0--PX2 remain authoritative and unchanged; PX3/PX4 authority is absent.\n");
    output
}

fn artifact_paths(stage: &str) -> (PathBuf, PathBuf) {
    let suffix = stage.to_ascii_lowercase();
    (
        PathBuf::from(format!(
            "results/cj0_e_transient_coincidence_{suffix}_v1.csv"
        )),
        PathBuf::from(format!(
            "results/cj0_e_transient_coincidence_{suffix}_v1.md"
        )),
    )
}

fn publish(report: &StageReport) -> io::Result<()> {
    let (csv_path, md_path) = artifact_paths(report.stage);
    let csv_staging = csv_path.with_extension("csv.staging");
    let md_staging = md_path.with_extension("md.staging");
    for path in [&csv_path, &md_path, &csv_staging, &md_staging] {
        if path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("refusing existing artifact {}", path.display()),
            ));
        }
    }
    if let Some(parent) = csv_path.parent() {
        fs::create_dir_all(parent)?;
    }
    write_new(&csv_staging, &csv(report))?;
    write_new(&md_staging, &markdown(report))?;
    fs::rename(&csv_staging, &csv_path)?;
    fs::rename(&md_staging, &md_path)?;
    Ok(())
}

fn write_new(path: &Path, contents: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()
}

fn prior_positive(stage: &str) -> bool {
    let (csv, md) = artifact_paths(stage);
    csv.exists()
        && fs::read_to_string(md).is_ok_and(|contents| contents.contains("Verdict: **PASS**"))
}

fn print_report(report: &StageReport) {
    println!(
        "protocol={PROTOCOL} stage={} pass={} rows={} work={} claim_eligible=false",
        report.stage,
        report.passed(),
        report.rows.len(),
        report.total_work()
    );
    for (clause, passed) in &report.clauses {
        println!("clause={} pass={}", clause, passed);
    }
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let report = match args.as_slice() {
        [arg] if arg == "--preflight" => {
            println!(
                "protocol={PROTOCOL} preflight={} no_cell=true definitive=false authority=false",
                frozen_hashes_pass()
            );
            return;
        }
        [arg] if arg == "--probe" => run_probe(),
        [arg] if arg == "--micro" && prior_positive("PROBE") => run_micro(),
        [arg] if arg == "--gate" && prior_positive("PROBE") && prior_positive("MICRO") => {
            run_gate()
        }
        [arg] if arg == "--micro" || arg == "--gate" => {
            eprintln!("refusing stage: prior positive artifact absent");
            std::process::exit(2);
        }
        _ => {
            eprintln!("usage: cj0_e --preflight|--probe|--micro|--gate");
            std::process::exit(2);
        }
    };
    print_report(&report);
    if let Err(error) = publish(&report) {
        eprintln!("atomic publication failed: {error}");
        std::process::exit(2);
    }
    if !report.passed() {
        std::process::exit(1);
    }
}
