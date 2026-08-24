use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CANDIDATES: usize = 3;
const SCAFFOLD_RESISTANCE: u32 = 1_000;
const CANDIDATE_RESISTANCE: u32 = 3;

#[derive(Clone)]
struct Fixture {
    substrate: PlasticSubstrate,
    sources: [CellId; CANDIDATES],
    variable_arrows: [[ArrowId; 2]; CANDIDATES],
}

#[derive(Clone, Debug)]
struct Row {
    name: &'static str,
    passed: bool,
    effects: usize,
    first_resistance: u32,
    second_resistance: u32,
    third_resistance: u32,
    work: u64,
    diagnostic: String,
}

fn main() {
    let mut args = env::args().skip(1);
    let mut output_prefix = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output-prefix" => {
                output_prefix = Some(PathBuf::from(
                    args.next().expect("--output-prefix requires a path"),
                ));
            }
            "--definitive" => {
                eprintln!("PX0 definitive execution is not authorized");
                std::process::exit(2);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    let output_prefix = output_prefix.expect("development output prefix is required");
    let rows = run_probe();
    let passed = rows.iter().all(|row| row.passed);
    write_results(&output_prefix, &rows, passed);
    if !passed {
        std::process::exit(1);
    }
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: SCAFFOLD_RESISTANCE,
    }
}

fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
    }
}

fn build(namespace: u64, returned: [bool; CANDIDATES], reverse: bool) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; CANDIDATES];
    let mut probes = [None; CANDIDATES];
    let mut contenders = [None; CANDIDATES];
    let order = if reverse { [2, 1, 0] } else { [0, 1, 2] };
    for candidate in order {
        sources[candidate] = Some(substrate.add_cell(cell(
            namespace + 10 + candidate as u64,
            candidate as i32 * 3,
            0,
            2,
        )));
        probes[candidate] = Some(substrate.add_cell(cell(
            namespace + 20 + candidate as u64,
            candidate as i32 * 3 + 1,
            -1,
            1,
        )));
        contenders[candidate] = Some(substrate.add_cell(cell(
            namespace + 30 + candidate as u64,
            candidate as i32 * 3 + 2,
            0,
            1,
        )));
    }
    let sources = sources.map(|cell| cell.expect("source allocated"));
    let probes = probes.map(|cell| cell.expect("probe allocated"));
    let contenders = contenders.map(|cell| cell.expect("contender allocated"));
    let veto = substrate.add_cell(cell(namespace + 40, 20, 0, 2));
    let accumulator = substrate.add_cell(cell(namespace + 41, 21, 0, 1));
    let outside = substrate.add_cell(cell(namespace + 42, 22, 1, 1));

    let mut variable_arrows = [[None; 2]; CANDIDATES];
    for candidate in order {
        variable_arrows[candidate][0] = Some(substrate.add_arrow(arrow(
            sources[candidate],
            probes[candidate],
            1,
            1,
            CANDIDATE_RESISTANCE,
        )));
        variable_arrows[candidate][1] = Some(substrate.add_arrow(arrow(
            sources[candidate],
            contenders[candidate],
            1,
            1,
            CANDIDATE_RESISTANCE,
        )));
        if returned[candidate] {
            substrate.add_arrow(arrow(
                probes[candidate],
                sources[candidate],
                1,
                1,
                SCAFFOLD_RESISTANCE,
            ));
        }
        substrate.add_arrow(arrow(
            contenders[candidate],
            veto,
            0,
            1,
            SCAFFOLD_RESISTANCE,
        ));
        substrate.add_arrow(arrow(
            contenders[candidate],
            accumulator,
            2,
            1,
            SCAFFOLD_RESISTANCE,
        ));
    }
    substrate.add_arrow(arrow(veto, accumulator, 1, -4, SCAFFOLD_RESISTANCE));
    substrate.add_arrow(arrow(accumulator, outside, 0, 1, SCAFFOLD_RESISTANCE));

    Fixture {
        substrate,
        sources,
        variable_arrows: variable_arrows.map(|pair| pair.map(|arrow| arrow.expect("allocated"))),
    }
}

fn activate(fixture: &mut Fixture, candidates: &[usize], tick: i64, namespace: u64) -> Execution {
    for candidate in candidates {
        for ordinal in 0..2 {
            fixture.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase: ordinal,
                origin_physical: namespace + (*candidate as u64).rotate_left(7) + ordinal as u64,
                target: fixture.sources[*candidate],
                impulse: 1,
            });
        }
    }
    fixture.substrate.propagate()
}

fn outward_effects(execution: &Execution) -> usize {
    execution
        .crossings
        .iter()
        .filter(|crossing| crossing.from_region == 0 && crossing.to_region == 1)
        .count()
}

fn train(fixture: &mut Fixture, candidates: &[usize], namespace: u64) -> u64 {
    [0, 10, 20]
        .into_iter()
        .map(|tick| {
            activate(fixture, candidates, tick, namespace + tick as u64)
                .work
                .total()
        })
        .sum()
}

fn resistance(fixture: &Fixture, candidate: usize) -> u32 {
    fixture.variable_arrows[candidate]
        .iter()
        .map(|arrow| fixture.substrate.arrow_resistance(*arrow))
        .min()
        .expect("two variable arrows")
}

fn row(
    name: &'static str,
    passed: bool,
    effects: usize,
    fixture: &Fixture,
    work: u64,
    diagnostic: impl Into<String>,
) -> Row {
    Row {
        name,
        passed,
        effects,
        first_resistance: resistance(fixture, 0),
        second_resistance: resistance(fixture, 1),
        third_resistance: resistance(fixture, 2),
        work,
        diagnostic: diagnostic.into(),
    }
}

fn run_probe() -> Vec<Row> {
    let mut rows = Vec::new();
    let mut learned = build(0x5000, [true, false, false], false);
    let training_work = train(&mut learned, &[0, 1], 0x6000);
    let time_work = learned.substrate.advance_time(100).total();
    let acquisition_passed =
        resistance(&learned, 0) > 0 && resistance(&learned, 1) == 0 && resistance(&learned, 2) == 0;
    rows.push(row(
        "recurring-returned-configuration-persists",
        acquisition_passed,
        0,
        &learned,
        training_work + time_work,
        "ordinary returned activity preserves only locally eligible routes",
    ));

    let good = activate(&mut learned, &[0], 100, 0x7000);
    let good_effects = outward_effects(&good);
    rows.push(row(
        "held-out-physical-execution",
        good_effects == 1 && good.naturally_quiescent,
        good_effects,
        &learned,
        good.work.total(),
        "fresh anonymous origins activate the retained physical route",
    ));
    learned.substrate.advance_time(120);
    let missing = activate(&mut learned, &[2], 120, 0x7100);
    let missing_effects = outward_effects(&missing);
    rows.push(row(
        "missing-configuration-no-effect",
        missing_effects == 0,
        missing_effects,
        &learned,
        missing.work.total(),
        "ordinary pressure removed the unsupported route",
    ));

    let mut ambiguous = build(0x8000, [true, true, false], false);
    let ambiguity_training = train(&mut ambiguous, &[0, 1], 0x9000);
    ambiguous.substrate.advance_time(100);
    let ambiguous_run = activate(&mut ambiguous, &[0, 1], 100, 0xa000);
    let ambiguous_effects = outward_effects(&ambiguous_run);
    rows.push(row(
        "equal-incompatible-support-no-arbitrary-effect",
        ambiguous_effects == 0,
        ambiguous_effects,
        &ambiguous,
        ambiguity_training + ambiguous_run.work.total(),
        "ordinary early inhibition cancels equal simultaneous continuations",
    ));

    let mut replay_base = build(0xb000, [true, false, false], false);
    train(&mut replay_base, &[0, 1], 0xc000);
    replay_base.substrate.advance_time(100);
    let mut first = replay_base.clone();
    let mut second = replay_base.clone();
    let first_run = activate(&mut first, &[0], 100, 0xd000);
    let second_run = activate(&mut second, &[0], 100, 0xd000);
    let duplicate_passed = first_run == second_run
        && first.substrate.complete_fingerprint() == second.substrate.complete_fingerprint();
    rows.push(row(
        "complete-state-replay-exact",
        duplicate_passed,
        outward_effects(&first_run),
        &first,
        first_run.work.total(),
        "complete physical duplicates produce byte-equivalent observables",
    ));

    let mut permuted = build(0xe000, [true, false, false], true);
    train(&mut permuted, &[0, 1], 0xf000);
    permuted.substrate.advance_time(100);
    let permuted_run = activate(&mut permuted, &[0], 100, 0x10_000);
    let permuted_effects = outward_effects(&permuted_run);
    rows.push(row(
        "fresh-identities-allocation-permutation",
        permuted_effects == 1 && resistance(&permuted, 0) > 0 && resistance(&permuted, 1) == 0,
        permuted_effects,
        &permuted,
        permuted_run.work.total(),
        "the physical law transfers across namespace and allocation order",
    ));

    rows.push(row(
        "continuous-no-reset",
        good.naturally_quiescent
            && missing.naturally_quiescent
            && ambiguous_run.naturally_quiescent,
        0,
        &learned,
        0,
        "all presentations share one monotonically advancing physical lifetime",
    ));
    rows
}

fn write_results(prefix: &Path, rows: &[Row], passed: bool) {
    let csv_path = prefix.with_extension("csv");
    let md_path = prefix.with_extension("md");
    if let Some(parent) = csv_path.parent() {
        fs::create_dir_all(parent).expect("create result directory");
    }
    let mut csv = String::from(
        "name,passed,effects,first_resistance,second_resistance,third_resistance,work,diagnostic\n",
    );
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},\"{}\"\n",
            row.name,
            row.passed,
            row.effects,
            row.first_resistance,
            row.second_resistance,
            row.third_resistance,
            row.work,
            row.diagnostic.replace('"', "\"\"")
        ));
    }
    let mut md = format!(
        "# PX0 substrate-native correspondence PROBE v1\n\nOutcome: **{}**.\n\n",
        if passed { "POSITIVE" } else { "NEGATIVE" }
    );
    md.push_str("| control | pass | effects | route 0 | route 1 | route 2 | work |\n");
    md.push_str("|---|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            row.name,
            row.passed,
            row.effects,
            row.first_resistance,
            row.second_resistance,
            row.third_resistance,
            row.work
        ));
    }
    md.push_str(&format!(
        "\nPersistent bytes in the primary cell: `{}`.\n",
        primary_persistent_bytes()
    ));
    atomic_write(&csv_path, &csv);
    atomic_write(&md_path, &md);
}

fn primary_persistent_bytes() -> usize {
    build(0x11_000, [true, false, false], false)
        .substrate
        .persistent_bytes()
}

fn atomic_write(path: &Path, contents: &str) {
    let temporary = path.with_extension(format!(
        "{}.tmp",
        path.extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("result")
    ));
    fs::write(&temporary, contents).expect("write temporary result");
    fs::rename(&temporary, path).expect("atomically install result");
}
