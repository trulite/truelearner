use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const N: usize = 3;
const SCAFFOLD: u32 = 1_000;

#[derive(Clone)]
struct Fixture {
    substrate: PlasticSubstrate,
    sources: [CellId; N],
    probes: [CellId; N],
    contenders: [CellId; N],
    gates: [CellId; N],
}

#[derive(Clone, Debug)]
struct Row {
    name: &'static str,
    passed: bool,
    effects: usize,
    proposals: u64,
    deallocations: u64,
    arrows: usize,
    work: u64,
    diagnostic: String,
}

fn main() {
    let mut args = env::args().skip(1);
    let mut output_prefix = None;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--output-prefix" => {
                output_prefix = Some(PathBuf::from(
                    args.next().expect("--output-prefix requires a path"),
                ));
            }
            "--definitive" => {
                eprintln!("PX0-R definitive execution is not authorized");
                std::process::exit(2);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    let rows = run_probe();
    let passed = rows.iter().all(|row| row.passed);
    write_results(
        &output_prefix.expect("development output prefix is required"),
        &rows,
        passed,
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

fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: SCAFFOLD,
    }
}

fn build(namespace: u64, reverse: bool) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let mut sources = [None; N];
    let mut probes = [None; N];
    let mut contenders = [None; N];
    let mut gates = [None; N];
    let order = if reverse { [2, 1, 0] } else { [0, 1, 2] };
    for index in order {
        let position = index as i32 * 10;
        sources[index] =
            Some(substrate.add_cell(cell(namespace + 10 + index as u64, position, 0, 2)));
        probes[index] =
            Some(substrate.add_cell(cell(namespace + 20 + index as u64, position + 1, -1, 2)));
        contenders[index] =
            Some(substrate.add_cell(cell(namespace + 30 + index as u64, position + 2, 0, 2)));
        gates[index] =
            Some(substrate.add_cell(cell(namespace + 40 + index as u64, position + 4, -1, 2)));
    }
    let sources = sources.map(|value| value.expect("source allocated"));
    let probes = probes.map(|value| value.expect("probe allocated"));
    let contenders = contenders.map(|value| value.expect("contender allocated"));
    let gates = gates.map(|value| value.expect("gate allocated"));
    let veto = substrate.add_cell(cell(namespace + 100, 100, 0, 2));
    let accumulator = substrate.add_cell(cell(namespace + 101, 101, 0, 1));
    let outside = substrate.add_cell(cell(namespace + 102, 102, 1, 1));
    for index in order {
        substrate.add_arrow(arrow(probes[index], gates[index], 1, 1));
        substrate.add_arrow(arrow(gates[index], sources[index], 1, 1));
        substrate.add_arrow(arrow(contenders[index], veto, 0, 1));
        substrate.add_arrow(arrow(contenders[index], accumulator, 2, 1));
    }
    substrate.add_arrow(arrow(veto, accumulator, 1, -4));
    substrate.add_arrow(arrow(accumulator, outside, 0, 1));
    Fixture {
        substrate,
        sources,
        probes,
        contenders,
        gates,
    }
}

fn experience(
    fixture: &mut Fixture,
    active: &[usize],
    supported: &[usize],
    tick: i64,
    namespace: u64,
    reverse_arrival: bool,
) -> Execution {
    for index in active {
        let phases = if reverse_arrival { [1, 0] } else { [0, 1] };
        for (ordinal, phase) in phases.into_iter().enumerate() {
            fixture.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase,
                origin_physical: namespace + (*index as u64) * 8 + ordinal as u64,
                target: fixture.sources[*index],
                impulse: 1,
            });
        }
        fixture.substrate.enter(SpikeInput {
            arrival_tick: tick + 1,
            phase: 0,
            origin_physical: namespace + 0x100 + *index as u64,
            target: fixture.probes[*index],
            impulse: 1,
        });
    }
    for index in supported {
        fixture.substrate.enter(SpikeInput {
            arrival_tick: tick + 2,
            phase: 0,
            origin_physical: namespace + 0x200 + *index as u64,
            target: fixture.gates[*index],
            impulse: 1,
        });
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

fn variable_ids(fixture: &Fixture, index: usize) -> Vec<ArrowId> {
    let mut ids = fixture
        .substrate
        .arrows_between(fixture.sources[index], fixture.probes[index]);
    ids.extend(
        fixture
            .substrate
            .arrows_between(fixture.sources[index], fixture.contenders[index]),
    );
    ids
}

fn live_variable_count(fixture: &Fixture, index: usize) -> usize {
    variable_ids(fixture, index)
        .into_iter()
        .filter(|arrow| fixture.substrate.arrow_is_live(*arrow))
        .count()
}

fn run_experiences(
    fixture: &mut Fixture,
    active: &[usize],
    supported: &[usize],
    start: i64,
    namespace: u64,
) -> (u64, u64, u64) {
    let mut work = 0;
    let mut proposals = 0;
    let mut deallocations = 0;
    for ordinal in 0..4 {
        let execution = experience(
            fixture,
            active,
            supported,
            start + ordinal * 10,
            namespace + ordinal as u64 * 0x10,
            ordinal % 2 == 1,
        );
        work += execution.work.total();
        proposals += execution.work.local_structural_proposals;
        deallocations += execution.work.physical_deallocations;
    }
    (work, proposals, deallocations)
}

#[allow(clippy::too_many_arguments)]
fn row(
    name: &'static str,
    passed: bool,
    effects: usize,
    proposals: u64,
    deallocations: u64,
    fixture: &Fixture,
    work: u64,
    diagnostic: impl Into<String>,
) -> Row {
    Row {
        name,
        passed,
        effects,
        proposals,
        deallocations,
        arrows: fixture.substrate.arrow_count(),
        work,
        diagnostic: diagnostic.into(),
    }
}

fn run_probe() -> Vec<Row> {
    let mut rows = Vec::new();
    let mut fixture = build(0x70_000, false);
    let (initial_work, initial_proposals, initial_deallocations) =
        run_experiences(&mut fixture, &[0, 1], &[0], 0, 0x71_000);
    let old_ids = variable_ids(&fixture, 0);
    let initial = experience(&mut fixture, &[0], &[0], 40, 0x72_000, false);
    let initial_effects = effects(&initial);
    rows.push(row(
        "initial-physical-acquisition",
        old_ids.len() == 2
            && old_ids
                .iter()
                .all(|arrow| fixture.substrate.arrow_is_live(*arrow))
            && initial_effects == 1,
        initial_effects,
        initial_proposals,
        initial_deallocations,
        &fixture,
        initial_work + initial.work.total(),
        "local adjacency formed the first retained correspondence",
    ));

    let arrows_before_absence = fixture.substrate.arrow_count();
    let absence_work = fixture.substrate.advance_time(200);
    let old_dead = old_ids
        .iter()
        .all(|arrow| !fixture.substrate.arrow_is_live(*arrow));
    rows.push(row(
        "full-deallocation-no-resurrection",
        old_dead && fixture.substrate.arrow_count() == arrows_before_absence,
        0,
        0,
        absence_work.physical_deallocations,
        &fixture,
        absence_work.total(),
        "time and pressure alone created no proposal and revived no old arrow",
    ));

    let first_renewal = experience(&mut fixture, &[0, 1], &[1], 200, 0x73_000, true);
    let new_ids = variable_ids(&fixture, 1)
        .into_iter()
        .filter(|arrow| fixture.substrate.arrow_is_live(*arrow))
        .collect::<Vec<_>>();
    let fresh_identity = new_ids.len() == 2
        && new_ids.iter().all(|new_arrow| {
            old_ids.iter().all(|old_arrow| old_arrow != new_arrow)
                && fixture.substrate.arrow_generation(*new_arrow)
                    != fixture.substrate.arrow_generation(old_ids[0])
        });
    rows.push(row(
        "renewed-activity-forms-fresh-identity",
        first_renewal.work.local_structural_proposals >= 4 && fresh_identity && old_dead,
        effects(&first_renewal),
        first_renewal.work.local_structural_proposals,
        first_renewal.work.physical_deallocations,
        &fixture,
        first_renewal.work.total(),
        "fresh arrows arose from current adjacency; dead arrows stayed dead",
    ));

    let (renewal_work, renewal_proposals, renewal_deallocations) =
        run_experiences(&mut fixture, &[0, 1], &[1], 210, 0x74_000);
    let restored = experience(&mut fixture, &[1], &[1], 250, 0x75_000, false);
    let restored_effects = effects(&restored);
    let historical = experience(&mut fixture, &[0], &[], 260, 0x76_000, false);
    let historical_effects = effects(&historical);
    rows.push(row(
        "opposite-return-reacquires-opposite-correspondence",
        restored_effects == 1
            && historical_effects == 0
            && live_variable_count(&fixture, 1) == 2
            && old_ids
                .iter()
                .all(|arrow| !fixture.substrate.arrow_is_live(*arrow)),
        restored_effects,
        renewal_proposals,
        renewal_deallocations,
        &fixture,
        renewal_work + restored.work.total() + historical.work.total(),
        "new physical return, not historical identity, selected the survivor",
    ));

    let mut absent = build(0x80_000, true);
    let (absent_work, absent_proposals, absent_deallocations) =
        run_experiences(&mut absent, &[0, 1], &[], 0, 0x81_000);
    let final_pressure = absent.substrate.advance_time(100);
    let absent_run = experience(&mut absent, &[0], &[], 100, 0x82_000, true);
    rows.push(row(
        "absent-return-retains-nothing",
        absent_proposals > 0 && live_variable_count(&absent, 0) <= 2 && effects(&absent_run) == 0,
        effects(&absent_run),
        absent_proposals,
        absent_deallocations + final_pressure.physical_deallocations,
        &absent,
        absent_work + final_pressure.total() + absent_run.work.total(),
        "fresh opportunities without return never became outward correspondence",
    ));

    let mut ambiguous = build(0x90_000, false);
    let (ambiguous_work, ambiguous_proposals, ambiguous_deallocations) =
        run_experiences(&mut ambiguous, &[0, 1], &[0, 1], 0, 0x91_000);
    let ambiguous_run = experience(&mut ambiguous, &[0, 1], &[0, 1], 40, 0x92_000, false);
    rows.push(row(
        "ambiguous-return-no-privileged-reacquisition",
        effects(&ambiguous_run) == 0
            && live_variable_count(&ambiguous, 0) == 2
            && live_variable_count(&ambiguous, 1) == 2,
        effects(&ambiguous_run),
        ambiguous_proposals,
        ambiguous_deallocations,
        &ambiguous,
        ambiguous_work + ambiguous_run.work.total(),
        "equal physical return retained both alternatives but emitted no arbitrary winner",
    ));

    let mut replay_first = fixture.clone();
    let mut replay_second = fixture.clone();
    replay_first.substrate.advance_time(280);
    replay_second.substrate.advance_time(280);
    let first = experience(&mut replay_first, &[1], &[1], 280, 0xa0_000, true);
    let second = experience(&mut replay_second, &[1], &[1], 280, 0xa0_000, true);
    rows.push(row(
        "complete-state-replay-exact",
        first == second
            && replay_first.substrate.complete_fingerprint()
                == replay_second.substrate.complete_fingerprint(),
        effects(&first),
        first.work.local_structural_proposals,
        first.work.physical_deallocations,
        &replay_first,
        first.work.total(),
        "complete physical duplicates remained exact",
    ));
    rows
}

fn write_results(prefix: &Path, rows: &[Row], passed: bool) {
    let csv_path = prefix.with_extension("csv");
    let md_path = prefix.with_extension("md");
    if let Some(parent) = csv_path.parent() {
        fs::create_dir_all(parent).expect("create result directory");
    }
    let mut csv =
        String::from("name,passed,effects,proposals,deallocations,arrows,work,diagnostic\n");
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},\"{}\"\n",
            row.name,
            row.passed,
            row.effects,
            row.proposals,
            row.deallocations,
            row.arrows,
            row.work,
            row.diagnostic.replace('"', "\"\"")
        ));
    }
    let mut md = format!(
        "# PX0-R generic physical correspondence reproposal PROBE v1\n\nOutcome: **{}**.\n\n",
        if passed { "POSITIVE" } else { "NEGATIVE" }
    );
    md.push_str("| control | pass | effects | proposals | deallocations | arrows | work |\n");
    md.push_str("|---|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            row.name,
            row.passed,
            row.effects,
            row.proposals,
            row.deallocations,
            row.arrows,
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
