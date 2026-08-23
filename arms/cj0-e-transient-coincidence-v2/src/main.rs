use cj0_e_transient_coincidence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const INCIDENCE: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (2, 1), (2, 3), (1, 3)];
const PROTOCOL: &str = "cj0-e-transient-coincidence-micro-v2";
const CSV_PATH: &str = "results/cj0_e_transient_coincidence_micro_v2.csv";
const MD_PATH: &str = "results/cj0_e_transient_coincidence_micro_v2.md";

const FROZEN_FILES: [(&str, &str); 10] = [
    (
        "experiments/cj0_e_transient_coincidence_development_protocol_v1.md",
        "b10a7454ae4da2f71e53dcc9662078acc5b7a09c7c506a7d77e5159b33774a8e",
    ),
    (
        "experiments/cj0_e_transient_coincidence_implementation_audit_v1.md",
        "0e67c8140f1e1da6d350aadc2b28a30496b9dcd0eafc809a8500e5863d94e226",
    ),
    (
        "experiments/cj0_e_transient_coincidence_frozen_negative_handoff_v1.md",
        "740a20d85e56f4236c365d613a8db86bcd1b1cea09c7558adecfc38e9024bc0d",
    ),
    (
        "arms/cj0-e-transient-coincidence/build.rs",
        "de69f4913e7c29379bdd13fddf8b70042d92159e59842ed0a9c2f5f9b21201b9",
    ),
    (
        "arms/cj0-e-transient-coincidence/src/lib.rs",
        "2da7f40828878bdd408cda2674d297f2c8eb65c63740baa9caa3d57b4f6c568a",
    ),
    (
        "arms/cj0-e-transient-coincidence/src/bin/cj0_e.rs",
        "df8d5c3cb1bcf5d35b345268deed7f40166b2c7307250723020f5907d699c963",
    ),
    (
        "results/cj0_e_transient_coincidence_probe_v1.csv",
        "d2b1bc6da9f865add6eaf35a8f06adb027c491b0f20557d58c034532ed6b6a6f",
    ),
    (
        "results/cj0_e_transient_coincidence_probe_v1.md",
        "c1b9f6c833093d4f95ad6e1748b2f59da38f7978b278090d162ccedd0a618617",
    ),
    (
        "results/cj0_e_transient_coincidence_micro_v1.csv",
        "340db47ffc46b9f07d79fc7df1069ffe3c53390eb2466b3801bc030c9753dc04",
    ),
    (
        "results/cj0_e_transient_coincidence_micro_v1.md",
        "f5f01ebc069c2fdc4292d101a53931f6af556a9c737b853dea5329f336093143",
    ),
];

#[derive(Clone, Copy, Default)]
struct Variant {
    mirror: bool,
    reverse_allocation: bool,
    reverse_arrival: bool,
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
    aggregate_returns: u64,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, Default)]
struct Seen {
    routes: [u64; 4],
    loci: [u64; 6],
    outputs: [u64; 6],
    learned_traversals: u64,
    aggregate_returns: u64,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct Support {
    resistance: u32,
    coupling: i32,
    generation: u32,
    live: bool,
}

#[derive(Clone)]
struct Row {
    cell: &'static str,
    scenario: &'static str,
    passed: bool,
    routes: [u64; 4],
    loci: [u64; 6],
    outputs: [u64; 6],
    support_before: [u32; 6],
    support_after: [u32; 6],
    coupling_before: [i32; 6],
    coupling_after: [i32; 6],
    generation_before: [u32; 6],
    generation_after: [u32; 6],
    live_after: [bool; 6],
    learned_traversals: u64,
    learned_output_firings: u64,
    learned_support_increases: u64,
    aggregate_returns: u64,
    complete_fingerprint: u64,
    permanent_fingerprint: u64,
    arrows: usize,
    bytes: usize,
    work: u64,
    quiescent: bool,
    note: String,
}

struct Report {
    rows: Vec<Row>,
    clauses: Vec<(&'static str, bool)>,
}

impl Report {
    fn passed(&self) -> bool {
        self.rows.iter().all(|row| row.passed) && self.clauses.iter().all(|(_, passed)| *passed)
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
            position: center + sign,
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
            self.route_firings[index] += seen.routes[index];
        }
        for index in 0..6 {
            self.locus_firings[index] += seen.loci[index];
            self.output_firings[index] += seen.outputs[index];
        }
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
) {
    for round in 0..rounds {
        let tick = start + round as i64 * 20;
        matter.occurrence(&[first.0, first.1], tick, true, None);
        matter.occurrence(&[second.0, second.1], tick + 4, true, None);
    }
}

fn use_once(matter: &Matter, active: &[usize]) -> Seen {
    let mut clone = matter.clone();
    let tick = clone.substrate.current_tick() + 4;
    clone.occurrence(active, tick, false, None)
}

fn blank_row(
    matter: &Matter,
    cell: &'static str,
    scenario: &'static str,
    passed: bool,
    before: ([u32; 6], [i32; 6], [u32; 6], [bool; 6]),
    note: String,
) -> Row {
    let after = matter.supports();
    Row {
        cell,
        scenario,
        passed,
        routes: matter.route_firings,
        loci: matter.locus_firings,
        outputs: matter.output_firings,
        support_before: before.0,
        support_after: after.0,
        coupling_before: before.1,
        coupling_after: after.1,
        generation_before: before.2,
        generation_after: after.2,
        live_after: after.3,
        learned_traversals: 0,
        learned_output_firings: 0,
        learned_support_increases: 0,
        aggregate_returns: matter.aggregate_returns,
        complete_fingerprint: matter.substrate.complete_fingerprint(),
        permanent_fingerprint: matter.substrate.permanent_fingerprint(),
        arrows: matter.substrate.arrow_count(),
        bytes: matter.substrate.persistent_bytes(),
        work: matter.work,
        quiescent: matter.quiescent,
        note,
    }
}

fn reversal(namespace: u64, variant: Variant, cell: &'static str) -> (Matter, Row, [bool; 6]) {
    let mut matter = build(namespace, variant);
    train(&mut matter, (0, 1), (2, 3), 12, 0);
    let before = matter.supports();
    let start = matter.substrate.current_tick() + 18;
    train(&mut matter, (0, 3), (2, 1), 40, start);
    let after = matter.supports();
    let new_ad = use_once(&matter, &[0, 3]);
    let new_cb = use_once(&matter, &[2, 1]);
    let old_ab = use_once(&matter, &[0, 1]);
    let old_cd = use_once(&matter, &[2, 3]);
    let old_dead = before.0[0] > 0 && before.0[4] > 0 && !after.3[0] && !after.3[4];
    let new_live = after.3[2] && after.3[3] && after.1[2] == 2 && after.1[3] == 2;
    let new_executes = new_ad.outputs[2] == 1 && new_cb.outputs[3] == 1;
    let old_silent = old_ab.outputs[0] == 0 && old_cd.outputs[4] == 0;
    let matched = matter
        .route_firings
        .iter()
        .all(|count| *count == matter.route_firings[0]);
    let quiescent = matter.quiescent
        && [&new_ad, &new_cb, &old_ab, &old_cd]
            .into_iter()
            .all(|seen| seen.quiescent);
    let passed = old_dead && new_live && new_executes && old_silent && matched && quiescent;
    let row = blank_row(
        &matter,
        cell,
        "mandatory-reversal",
        passed,
        before,
        format!(
            "old={}|{}->{}|{};new={}|{};new_exec={new_executes};old_silent={old_silent}",
            before.0[0], before.0[4], after.0[0], after.0[4], after.0[2], after.0[3]
        ),
    );
    (
        matter,
        row,
        [
            old_dead,
            new_live,
            new_executes,
            old_silent,
            matched,
            quiescent,
        ],
    )
}

fn bootstrap() -> (Row, [bool; 5]) {
    let mut matter = build(0xd220_0000, Variant::default());
    let empty = matter.supports();
    let first = matter.occurrence(&[0, 1], 0, false, Some(0));
    matter.advance(60);
    let dead = matter.selected_arrow(0).is_none();
    let bootstrap = matter.occurrence(&[0, 1], 64, true, Some(0));
    let formed = matter.support(0).live && matter.support(0).coupling == 2;
    let heldout = use_once(&matter, &[0, 1]);
    let executes = heldout.outputs[0] == 1;
    let no_mature_first = first.outputs[0] == 0 && bootstrap.outputs[0] == 0;
    let passed = dead && formed && executes && no_mature_first && matter.quiescent;
    (
        blank_row(
            &matter,
            "bootstrap",
            "full-deallocation-bootstrap",
            passed,
            empty,
            format!(
                "dead={dead};formed={formed};executes={executes};mature_first={}",
                !no_mature_first
            ),
        ),
        [dead, formed, executes, no_mature_first, matter.quiescent],
    )
}

fn self_evidence() -> (Row, [bool; 7]) {
    let mut matter = build(0xd230_0000, Variant::default());
    train(&mut matter, (0, 1), (2, 3), 12, 0);
    let before_all = matter.supports();
    let before = matter.support(0);
    let start = matter.substrate.current_tick() + 4;
    let aggregate_before = matter.aggregate_returns;
    let mut learned_traversals = 0;
    let mut learned_outputs = 0;
    let mut support_increases = 0;
    for round in 0..12 {
        let resistance_before = matter.support(0).resistance;
        let seen = matter.occurrence(&[0], start + round * 4, false, Some(0));
        learned_traversals += seen.learned_traversals;
        learned_outputs += seen.outputs[0];
        support_increases += u64::from(matter.support(0).resistance > resistance_before);
    }
    let after = matter.support(0);
    let aggregate_fresh = matter.aggregate_returns - aggregate_before;
    let no_traversal = learned_traversals == 0;
    let no_output = learned_outputs == 0;
    let no_reinforcement = support_increases == 0 && after.resistance < before.resistance;
    let same_matter =
        after.live && after.coupling == before.coupling && after.generation == before.generation;
    let contributor_returns_visible = aggregate_fresh > 0;
    let quiescent = matter.quiescent;
    let passed = no_traversal
        && no_output
        && no_reinforcement
        && same_matter
        && contributor_returns_visible
        && quiescent;
    let mut row = blank_row(
        &matter,
        "self-evidence",
        "repeated-A-alone",
        passed,
        before_all,
        format!(
            "learned_r={}->{};aggregate_fresh={aggregate_fresh};learned_traversal={learned_traversals};learned_output={learned_outputs};support_increases={support_increases}",
            before.resistance, after.resistance
        ),
    );
    row.learned_traversals = learned_traversals;
    row.learned_output_firings = learned_outputs;
    row.learned_support_increases = support_increases;
    row.aggregate_returns = aggregate_fresh;
    (
        row,
        [
            no_traversal,
            no_output,
            no_reinforcement,
            same_matter,
            contributor_returns_visible,
            quiescent,
            passed,
        ],
    )
}

fn run_micro() -> Report {
    let (primary, primary_row, primary_checks) =
        reversal(0xd200_0000, Variant::default(), "primary");
    let (duplicate, duplicate_row, duplicate_checks) =
        reversal(0xd200_0000, Variant::default(), "duplicate");
    let exact = primary.substrate.complete_fingerprint()
        == duplicate.substrate.complete_fingerprint()
        && primary.substrate.permanent_fingerprint() == duplicate.substrate.permanent_fingerprint();
    let (_, mirror_row, mirror_checks) = reversal(
        0xd210_0000,
        Variant {
            mirror: true,
            reverse_allocation: true,
            reverse_arrival: true,
        },
        "mirror-permutation",
    );
    let (bootstrap_row, bootstrap_checks) = bootstrap();
    let (self_row, self_checks) = self_evidence();
    Report {
        rows: vec![
            primary_row,
            duplicate_row,
            mirror_row,
            bootstrap_row,
            self_row,
        ],
        clauses: vec![
            ("v1 bytes and physical law preserved", frozen_files_pass()),
            ("old organization deallocated", primary_checks[0]),
            ("changed organization formed", primary_checks[1]),
            ("changed organization executes", primary_checks[2]),
            ("old use and replay silent", primary_checks[3]),
            ("reversal marginals matched", primary_checks[4]),
            ("reversal quiescent", primary_checks[5]),
            ("exact duplicate replay", exact),
            (
                "duplicate matrix",
                duplicate_checks.into_iter().all(|value| value),
            ),
            (
                "mirror permutation",
                mirror_checks.into_iter().all(|value| value),
            ),
            ("bootstrap fully deallocated", bootstrap_checks[0]),
            ("bootstrap formed", bootstrap_checks[1]),
            ("bootstrap executes", bootstrap_checks[2]),
            ("bootstrap no mature prerequisite", bootstrap_checks[3]),
            ("learned traversal absent under A alone", self_checks[0]),
            ("learned output absent under A alone", self_checks[1]),
            ("learned support not reinforced", self_checks[2]),
            ("learned generation and coupling stable", self_checks[3]),
            ("unrelated aggregate returns disclosed", self_checks[4]),
            ("self-evidence quiescent", self_checks[5]),
        ],
    }
}

fn frozen_files_pass() -> bool {
    FROZEN_FILES
        .iter()
        .all(|(path, expected)| sha256(Path::new(path)).is_some_and(|actual| actual == *expected))
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

fn join<T: ToString>(values: &[T]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn csv(report: &Report) -> String {
    let mut out = "protocol,cell,scenario,passed,routes,loci,outputs,support_before,support_after,coupling_before,coupling_after,generation_before,generation_after,live_after,learned_traversals,learned_output_firings,learned_support_increases,aggregate_returns,complete_fingerprint,permanent_fingerprint,arrows,persistent_bytes,work,quiescent,note\n".to_string();
    for row in &report.rows {
        out.push_str(&format!(
            "{PROTOCOL},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:016x},{:016x},{},{},{},{},{}\n",
            row.cell,
            row.scenario,
            row.passed,
            join(&row.routes),
            join(&row.loci),
            join(&row.outputs),
            join(&row.support_before),
            join(&row.support_after),
            join(&row.coupling_before),
            join(&row.coupling_after),
            join(&row.generation_before),
            join(&row.generation_after),
            join(&row.live_after),
            row.learned_traversals,
            row.learned_output_firings,
            row.learned_support_increases,
            row.aggregate_returns,
            row.complete_fingerprint,
            row.permanent_fingerprint,
            row.arrows,
            row.bytes,
            row.work,
            row.quiescent,
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
        "# CJ0-E transient coincidence MICRO v2 result\n\nVerdict: **{verdict}**. Development-only.\n\n| clause | pass |\n|---|:---:|\n"
    );
    for (clause, passed) in &report.clauses {
        out.push_str(&format!("| {clause} | {passed} |\n"));
    }
    out.push_str(&format!(
        "\nRows: `{}`. Ledgered work: `{}`. V1 bytes remain frozen. Aggregate contributor returns remain visible in the CSV.\n",
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
                frozen_files_pass()
            );
        }
        [arg] if arg == "--micro" => {
            let report = run_micro();
            println!(
                "protocol={PROTOCOL} pass={} rows={} work={} development_only=true",
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
        _ => {
            eprintln!("usage: cj0-e-transient-coincidence-v2 --preflight|--micro");
            std::process::exit(2);
        }
    }
}
