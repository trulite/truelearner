use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const N: usize = 3;
const SCAFFOLD: u32 = 1_000;
const VARIABLE: u32 = 3;

#[derive(Clone)]
struct Fixture {
    substrate: PlasticSubstrate,
    sources: [CellId; N],
    variable: [[ArrowId; 2]; N],
}

#[derive(Clone, Debug)]
struct Row {
    cell: usize,
    name: &'static str,
    passed: bool,
    effects: usize,
    selected: u32,
    rejected: u32,
    unused: u32,
    work: u64,
}

struct Observation {
    cell: usize,
    name: &'static str,
    passed: bool,
    effects: usize,
    selected: usize,
    rejected: usize,
    unused: usize,
    work: u64,
}

fn main() {
    let mut args = env::args().skip(1);
    let mut output_prefix = None;
    let mut cell_count = 2;
    let mut base_namespace = 0x20_000;
    let mut stage = String::from("MICRO");
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--output-prefix" => {
                output_prefix = Some(PathBuf::from(
                    args.next().expect("--output-prefix requires a path"),
                ));
            }
            "--cells" => {
                cell_count = args
                    .next()
                    .expect("--cells requires a count")
                    .parse()
                    .expect("--cells must be an integer");
            }
            "--base-namespace" => {
                base_namespace = u64::from_str_radix(
                    args.next()
                        .expect("--base-namespace requires a hexadecimal value")
                        .trim_start_matches("0x"),
                    16,
                )
                .expect("--base-namespace must be hexadecimal");
            }
            "--stage" => stage = args.next().expect("--stage requires a name"),
            "--definitive" => {
                eprintln!("PX0 definitive execution is not authorized");
                std::process::exit(2);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    let rows = run_matrix(cell_count, base_namespace);
    let passed = rows.iter().all(|row| row.passed);
    write_results(
        &output_prefix.expect("development output prefix is required"),
        &rows,
        passed,
        &stage,
    );
    if !passed {
        std::process::exit(1);
    }
}

fn cell(id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id: id,
        position,
        region,
        threshold,
        resistance: SCAFFOLD,
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

fn build(namespace: u64, returned: [bool; N], reverse: bool) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; N];
    let mut probes = [None; N];
    let mut contenders = [None; N];
    let order = if reverse { [2, 1, 0] } else { [0, 1, 2] };
    for index in order {
        sources[index] =
            Some(substrate.add_cell(cell(namespace + 10 + index as u64, index as i32 * 3, 0, 2)));
        probes[index] = Some(substrate.add_cell(cell(
            namespace + 20 + index as u64,
            index as i32 * 3 + 1,
            -1,
            1,
        )));
        contenders[index] = Some(substrate.add_cell(cell(
            namespace + 30 + index as u64,
            index as i32 * 3 + 2,
            0,
            1,
        )));
    }
    let sources = sources.map(|value| value.expect("allocated source"));
    let probes = probes.map(|value| value.expect("allocated probe"));
    let contenders = contenders.map(|value| value.expect("allocated contender"));
    let veto = substrate.add_cell(cell(namespace + 40, 20, 0, 2));
    let accumulator = substrate.add_cell(cell(namespace + 41, 21, 0, 1));
    let outside = substrate.add_cell(cell(namespace + 42, 22, 1, 1));
    let mut variable = [[None; 2]; N];
    for index in order {
        variable[index][0] =
            Some(substrate.add_arrow(arrow(sources[index], probes[index], 1, 1, VARIABLE)));
        variable[index][1] =
            Some(substrate.add_arrow(arrow(sources[index], contenders[index], 1, 1, VARIABLE)));
        if returned[index] {
            substrate.add_arrow(arrow(probes[index], sources[index], 1, 1, SCAFFOLD));
        }
        substrate.add_arrow(arrow(contenders[index], veto, 0, 1, SCAFFOLD));
        substrate.add_arrow(arrow(contenders[index], accumulator, 2, 1, SCAFFOLD));
    }
    substrate.add_arrow(arrow(veto, accumulator, 1, -4, SCAFFOLD));
    substrate.add_arrow(arrow(accumulator, outside, 0, 1, SCAFFOLD));
    Fixture {
        substrate,
        sources,
        variable: variable.map(|pair| pair.map(|value| value.expect("allocated arrow"))),
    }
}

fn activate(
    fixture: &mut Fixture,
    indices: &[usize],
    tick: i64,
    namespace: u64,
    reverse_arrival: bool,
) -> Execution {
    for index in indices {
        let phases = if reverse_arrival { [1, 0] } else { [0, 1] };
        for (ordinal, phase) in phases.into_iter().enumerate() {
            fixture.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase,
                origin_physical: namespace + (*index as u64).rotate_left(11) + ordinal as u64,
                target: fixture.sources[*index],
                impulse: 1,
            });
        }
    }
    fixture.substrate.propagate()
}

fn effects(execution: &Execution) -> usize {
    execution
        .crossings
        .iter()
        .filter(|crossing| crossing.from_region == 0 && crossing.to_region == 1)
        .count()
}

fn resistance(fixture: &Fixture, index: usize) -> u32 {
    fixture.variable[index]
        .iter()
        .map(|arrow| fixture.substrate.arrow_resistance(*arrow))
        .min()
        .expect("variable pair")
}

fn record(rows: &mut Vec<Row>, fixture: &Fixture, observation: Observation) {
    rows.push(Row {
        cell: observation.cell,
        name: observation.name,
        passed: observation.passed,
        effects: observation.effects,
        selected: resistance(fixture, observation.selected),
        rejected: resistance(fixture, observation.rejected),
        unused: resistance(fixture, observation.unused),
        work: observation.work,
    });
}

fn run_matrix(cell_count: usize, base_namespace: u64) -> Vec<Row> {
    let mut rows = Vec::new();
    for cell in 0..cell_count {
        let namespace = base_namespace + (cell as u64) * 0x10_000;
        let selected = cell % N;
        let rejected = (selected + 1) % N;
        let unused = (selected + 2) % N;
        let mut returned = [false; N];
        returned[selected] = true;
        let mut fixture = build(namespace, returned, cell == 1);
        let ticks = match cell % 3 {
            0 => [0, 10, 20],
            1 => [0, 11, 22],
            _ => [0, 12, 24],
        };
        let test_tick = ticks[2] + 80;
        let mut work = 0;
        for (ordinal, tick) in ticks.into_iter().enumerate() {
            work += activate(
                &mut fixture,
                &[selected, rejected],
                tick,
                namespace + 0x100 + ordinal as u64 * 10,
                cell == 1,
            )
            .work
            .total();
        }
        work += fixture.substrate.advance_time(test_tick).total();
        let acquired = resistance(&fixture, selected) > 0
            && resistance(&fixture, rejected) == 0
            && resistance(&fixture, unused) == 0;
        record(
            &mut rows,
            &fixture,
            Observation {
                cell,
                name: "physical-acquisition",
                passed: acquired,
                effects: 0,
                selected,
                rejected,
                unused,
                work,
            },
        );

        let held_out = activate(
            &mut fixture,
            &[selected],
            test_tick,
            namespace + 0x200,
            cell == 0,
        );
        let held_out_effects = effects(&held_out);
        record(
            &mut rows,
            &fixture,
            Observation {
                cell,
                name: "fresh-origin-held-out",
                passed: held_out_effects == 1 && held_out.naturally_quiescent,
                effects: held_out_effects,
                selected,
                rejected,
                unused,
                work: held_out.work.total(),
            },
        );

        fixture.substrate.advance_time(test_tick + 20);
        let changed = activate(
            &mut fixture,
            &[unused],
            test_tick + 20,
            namespace + 0x300,
            false,
        );
        let changed_effects = effects(&changed);
        record(
            &mut rows,
            &fixture,
            Observation {
                cell,
                name: "changed-physical-neighborhood-invalidates",
                passed: changed_effects == 0,
                effects: changed_effects,
                selected,
                rejected,
                unused,
                work: changed.work.total(),
            },
        );

        fixture.substrate.advance_time(test_tick + 30);
        let historical = activate(
            &mut fixture,
            &[selected],
            test_tick + 30,
            namespace + 0x400,
            true,
        );
        let historical_effects = effects(&historical);
        record(
            &mut rows,
            &fixture,
            Observation {
                cell,
                name: "historical-physical-neighborhood-reuse",
                passed: historical_effects == 1,
                effects: historical_effects,
                selected,
                rejected,
                unused,
                work: historical.work.total(),
            },
        );

        let mut ambiguous_return = [false; N];
        ambiguous_return[selected] = true;
        ambiguous_return[rejected] = true;
        let mut ambiguous = build(namespace + 0x1_000, ambiguous_return, cell == 0);
        for (ordinal, tick) in ticks.into_iter().enumerate() {
            activate(
                &mut ambiguous,
                &[selected, rejected],
                tick,
                namespace + 0x1_100 + ordinal as u64 * 10,
                cell == 0,
            );
        }
        ambiguous.substrate.advance_time(test_tick);
        let ambiguous_run = activate(
            &mut ambiguous,
            &[selected, rejected],
            test_tick,
            namespace + 0x1_200,
            cell == 1,
        );
        let ambiguous_effects = effects(&ambiguous_run);
        record(
            &mut rows,
            &ambiguous,
            Observation {
                cell,
                name: "equal-support-ambiguous",
                passed: ambiguous_effects == 0,
                effects: ambiguous_effects,
                selected,
                rejected,
                unused,
                work: ambiguous_run.work.total(),
            },
        );

        let mut replay_first = fixture.clone();
        let mut replay_second = fixture.clone();
        replay_first.substrate.advance_time(test_tick + 50);
        replay_second.substrate.advance_time(test_tick + 50);
        let first = activate(
            &mut replay_first,
            &[selected],
            test_tick + 50,
            namespace + 0x500,
            false,
        );
        let second = activate(
            &mut replay_second,
            &[selected],
            test_tick + 50,
            namespace + 0x500,
            false,
        );
        let replay_passed = first == second
            && replay_first.substrate.complete_fingerprint()
                == replay_second.substrate.complete_fingerprint();
        record(
            &mut rows,
            &replay_first,
            Observation {
                cell,
                name: "duplicate-exact",
                passed: replay_passed,
                effects: effects(&first),
                selected,
                rejected,
                unused,
                work: first.work.total(),
            },
        );
    }
    rows
}

fn write_results(prefix: &Path, rows: &[Row], passed: bool, stage: &str) {
    let csv_path = prefix.with_extension("csv");
    let md_path = prefix.with_extension("md");
    if let Some(parent) = csv_path.parent() {
        fs::create_dir_all(parent).expect("create result directory");
    }
    let mut csv = String::from(
        "cell,name,passed,effects,selected_resistance,rejected_resistance,unused_resistance,work\n",
    );
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            row.cell,
            row.name,
            row.passed,
            row.effects,
            row.selected,
            row.rejected,
            row.unused,
            row.work
        ));
    }
    let mut md = format!(
        "# PX0 substrate-native correspondence {} v1\n\nOutcome: **{}**.\n\n",
        stage,
        if passed { "POSITIVE" } else { "NEGATIVE" }
    );
    md.push_str("| cell | control | pass | effects | selected | rejected | unused | work |\n");
    md.push_str("|---:|---|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.cell,
            row.name,
            row.passed,
            row.effects,
            row.selected,
            row.rejected,
            row.unused,
            row.work
        ));
    }
    atomic_write(&csv_path, &csv);
    atomic_write(&md_path, &md);
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
