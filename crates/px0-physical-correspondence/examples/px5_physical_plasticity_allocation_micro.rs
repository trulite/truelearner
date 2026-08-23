use px0_physical_correspondence::{
    ArrowId, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

const PROTOCOL: &str = "px5-physical-plasticity-allocation-micro-v1";
const PROTOCOL_COMMIT: &str = "08a707c";
const FROZEN_PARENT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const POSITIVE_PROBE: &str = "f5e1b80677ef89a28801eadb58a613497f139fd8";
const FROZEN_LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const BASE_NAMESPACE: u64 = 0x5_6000_0000;
const CELL_STRIDE: u64 = 0x0100_0000;
const CELLS: usize = 8;
const USEFUL: usize = 4;
const RESULT_CSV: &str = "results/px5_physical_plasticity_allocation_micro_v1.csv";
const RESULT_MD: &str = "results/px5_physical_plasticity_allocation_micro_v1.md";

#[derive(Clone)]
struct Fixture {
    substrate: PlasticSubstrate,
    pairs: Vec<(CellId, CellId)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CellResult {
    index: usize,
    namespace: u64,
    load: usize,
    mirrored: bool,
    reversed_allocation: bool,
    reversed_insertion: bool,
    initial_proposals: u64,
    primary_return_updates: u64,
    useful_live: usize,
    distractor_live: usize,
    held_crossings: usize,
    useful_min_resistance: u32,
    distractor_max_resistance: u32,
    return_free_proposals: u64,
    return_free_updates: u64,
    return_free_live: usize,
    withheld_dead: bool,
    recurrent_live: usize,
    stale_dead: bool,
    replacement_live: bool,
    replacement_crossings: usize,
    old_generation: u32,
    new_generation: u32,
    evaluator_shuffle_inert: bool,
    naturally_quiescent: bool,
    work: WorkLedger,
    persistent_bytes: usize,
    fingerprint: u64,
    claims: [bool; 10],
    duplicate_exact: bool,
    passed: bool,
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("--preflight") => {
            println!(
                "protocol={PROTOCOL} protocol_commit={PROTOCOL_COMMIT} parent={FROZEN_PARENT} probe={POSITIVE_PROBE} law={FROZEN_LAW_SHA256} cells={CELLS} artifacts_absent={}",
                artifacts_absent()
            );
            if !artifacts_absent() {
                std::process::exit(1);
            }
        }
        Some("--micro") if std::env::args().count() == 2 => {
            if !artifacts_absent() {
                eprintln!("refusing to overwrite or rescue frozen MICRO artifacts");
                std::process::exit(2);
            }
            let results = run_matrix();
            let passed = results.len() == CELLS && results.iter().all(|cell| cell.passed);
            if let Err(error) = publish(&results, passed) {
                eprintln!("failed to publish immutable MICRO artifacts: {error}");
                std::process::exit(1);
            }
            println!(
                "PX5_PHYSICAL_PLASTICITY_ALLOCATION_MICRO_EVIDENCE_SPENT outcome={} cells={}/{} claims={}/{}",
                if passed { "PASS" } else { "FAIL" },
                results.iter().filter(|cell| cell.passed).count(),
                CELLS,
                results
                    .iter()
                    .map(|cell| cell.claims.iter().filter(|claim| **claim).count())
                    .sum::<usize>(),
                CELLS * 10
            );
            if !passed {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("refusing execution: pass exactly --preflight or --micro");
            std::process::exit(2);
        }
    }
}

fn run_matrix() -> Vec<CellResult> {
    (0..CELLS)
        .map(|index| {
            let first = run_cell(index);
            let duplicate = run_cell(index);
            let duplicate_exact = first == duplicate;
            CellResult {
                duplicate_exact,
                passed: duplicate_exact && first.claims.iter().all(|claim| *claim),
                ..first
            }
        })
        .collect()
}

fn run_cell(index: usize) -> CellResult {
    let namespace = BASE_NAMESPACE + index as u64 * CELL_STRIDE;
    let load = if index & 1 == 0 { 8 } else { 24 };
    let mirrored = index & 2 != 0;
    let reversed_allocation = index & 4 != 0;
    let reversed_insertion = (index.count_ones() & 1) != 0;
    let mut fixture = build_fixture(namespace, USEFUL + load, mirrored, reversed_allocation);
    let mut work = WorkLedger::default();
    let mut naturally_quiescent = true;

    let all = (0..fixture.pairs.len()).collect::<Vec<_>>();
    let useful = (0..USEFUL).collect::<Vec<_>>();
    enter_batch(
        &mut fixture,
        &all,
        0,
        namespace + 0x10000,
        reversed_insertion,
    );
    let initial = fixture.substrate.propagate();
    naturally_quiescent &= initial.naturally_quiescent;
    let initial_proposals = initial.work.local_structural_proposals;
    add_execution(&mut work, &initial);

    enter_batch(
        &mut fixture,
        &useful,
        2,
        namespace + 0x20000,
        reversed_insertion,
    );
    let first_return = fixture.substrate.propagate();
    naturally_quiescent &= first_return.naturally_quiescent;
    let mut primary_return_updates = first_return.work.local_return_updates;
    add_execution(&mut work, &first_return);

    for (cycle, tick) in [10_i64, 20, 30, 40, 50].into_iter().enumerate() {
        enter_batch(
            &mut fixture,
            &useful,
            tick,
            namespace + 0x30000 + cycle as u64 * 0x100,
            reversed_insertion,
        );
        let start = fixture.substrate.propagate();
        naturally_quiescent &= start.naturally_quiescent;
        add_execution(&mut work, &start);
        enter_batch(
            &mut fixture,
            &useful,
            tick + 2,
            namespace + 0x40000 + cycle as u64 * 0x100,
            reversed_insertion,
        );
        let returned = fixture.substrate.propagate();
        naturally_quiescent &= returned.naturally_quiescent;
        primary_return_updates += returned.work.local_return_updates;
        add_execution(&mut work, &returned);
    }

    add_work(&mut work, &fixture.substrate.advance_time(60));
    let useful_live = useful
        .iter()
        .map(|pair| live_between(&fixture, *pair))
        .sum();
    let distractor_live = (USEFUL..fixture.pairs.len())
        .map(|pair| live_between(&fixture, pair))
        .sum();
    let useful_min_resistance = useful
        .iter()
        .map(|pair| max_resistance(&fixture, *pair))
        .min()
        .unwrap_or(0);
    let distractor_max_resistance = (USEFUL..fixture.pairs.len())
        .map(|pair| max_resistance(&fixture, pair))
        .max()
        .unwrap_or(0);

    enter_batch(
        &mut fixture,
        &useful,
        60,
        namespace + 0x50000,
        reversed_insertion,
    );
    let held = fixture.substrate.propagate();
    naturally_quiescent &= held.naturally_quiescent;
    let held_crossings = held.crossings.len();
    add_execution(&mut work, &held);
    enter_batch(
        &mut fixture,
        &useful,
        62,
        namespace + 0x60000,
        reversed_insertion,
    );
    let held_return = fixture.substrate.propagate();
    naturally_quiescent &= held_return.naturally_quiescent;
    add_execution(&mut work, &held_return);

    let old = sole_live_between(&fixture, 0);
    let recurrent = (1..USEFUL).collect::<Vec<_>>();
    for (cycle, tick) in [70_i64, 80, 90, 100, 110, 120, 130, 140]
        .into_iter()
        .enumerate()
    {
        enter_batch(
            &mut fixture,
            &recurrent,
            tick,
            namespace + 0x70000 + cycle as u64 * 0x100,
            reversed_insertion,
        );
        let start = fixture.substrate.propagate();
        naturally_quiescent &= start.naturally_quiescent;
        add_execution(&mut work, &start);
        enter_batch(
            &mut fixture,
            &recurrent,
            tick + 2,
            namespace + 0x80000 + cycle as u64 * 0x100,
            reversed_insertion,
        );
        let returned = fixture.substrate.propagate();
        naturally_quiescent &= returned.naturally_quiescent;
        add_execution(&mut work, &returned);
    }
    add_work(&mut work, &fixture.substrate.advance_time(150));
    let withheld_dead = !fixture.substrate.arrow_is_live(old) && live_between(&fixture, 0) == 0;
    let stale_dead = !fixture.substrate.arrow_is_live(old);
    let old_generation = fixture.substrate.arrow_generation(old);
    let recurrent_live = recurrent
        .iter()
        .map(|pair| live_between(&fixture, *pair))
        .sum();

    enter_batch(
        &mut fixture,
        &[0],
        150,
        namespace + 0x90000,
        reversed_insertion,
    );
    let replacement = fixture.substrate.propagate();
    naturally_quiescent &= replacement.naturally_quiescent;
    let replacement_crossings = replacement.crossings.len();
    add_execution(&mut work, &replacement);
    enter_batch(
        &mut fixture,
        &[0],
        152,
        namespace + 0xa0000,
        reversed_insertion,
    );
    let replacement_return = fixture.substrate.propagate();
    naturally_quiescent &= replacement_return.naturally_quiescent;
    add_execution(&mut work, &replacement_return);
    let new = sole_live_between(&fixture, 0);
    let new_generation = fixture.substrate.arrow_generation(new);
    let replacement_live = new != old && fixture.substrate.arrow_is_live(new);

    let before_shuffle = fixture.substrate.complete_fingerprint();
    let mut evaluator_allocation = (0..fixture.pairs.len())
        .map(|pair| live_between(&fixture, pair))
        .collect::<Vec<_>>();
    evaluator_allocation.reverse();
    let evaluator_shuffle_inert = fixture.substrate.complete_fingerprint() == before_shuffle
        && evaluator_allocation.iter().sum::<usize>() == USEFUL
        && (USEFUL..fixture.pairs.len()).all(|pair| live_between(&fixture, pair) == 0);

    let return_free = run_return_free(
        namespace + 0x0080_0000,
        mirrored,
        reversed_allocation,
        reversed_insertion,
    );
    naturally_quiescent &= return_free.3;
    add_work(&mut work, &return_free.4);
    let persistent_bytes = fixture.substrate.persistent_bytes() + return_free.5;
    let fingerprint = fixture.substrate.complete_fingerprint();
    let claims = [
        namespace == BASE_NAMESPACE + index as u64 * CELL_STRIDE,
        initial_proposals == (USEFUL + load) as u64,
        primary_return_updates == 24,
        useful_live == USEFUL && held_crossings == USEFUL,
        distractor_live == 0,
        useful_min_resistance > distractor_max_resistance && useful_min_resistance > 0,
        return_free.0 == 6 && return_free.1 == 0 && return_free.2 == 0,
        withheld_dead
            && recurrent_live == USEFUL - 1
            && stale_dead
            && replacement_live
            && replacement_crossings == 1,
        evaluator_shuffle_inert,
        naturally_quiescent && work.total() > 0 && persistent_bytes > 0,
    ];

    CellResult {
        index,
        namespace,
        load,
        mirrored,
        reversed_allocation,
        reversed_insertion,
        initial_proposals,
        primary_return_updates,
        useful_live,
        distractor_live,
        held_crossings,
        useful_min_resistance,
        distractor_max_resistance,
        return_free_proposals: return_free.0,
        return_free_updates: return_free.1,
        return_free_live: return_free.2,
        withheld_dead,
        recurrent_live,
        stale_dead,
        replacement_live,
        replacement_crossings,
        old_generation,
        new_generation,
        evaluator_shuffle_inert,
        naturally_quiescent,
        work,
        persistent_bytes,
        fingerprint,
        claims,
        duplicate_exact: false,
        passed: false,
    }
}

fn run_return_free(
    namespace: u64,
    mirrored: bool,
    reversed_allocation: bool,
    reversed_insertion: bool,
) -> (u64, u64, usize, bool, WorkLedger, usize) {
    let mut fixture = build_fixture(namespace, 1, mirrored, reversed_allocation);
    let mut work = WorkLedger::default();
    let mut quiescent = true;
    for (ordinal, tick) in [0_i64, 6, 12, 18, 24, 30].into_iter().enumerate() {
        enter_batch(
            &mut fixture,
            &[0],
            tick,
            namespace + 0xb0000 + ordinal as u64,
            reversed_insertion,
        );
        let run = fixture.substrate.propagate();
        quiescent &= run.naturally_quiescent;
        add_execution(&mut work, &run);
    }
    add_work(&mut work, &fixture.substrate.advance_time(36));
    (
        work.local_structural_proposals,
        work.local_return_updates,
        live_between(&fixture, 0),
        quiescent,
        work,
        fixture.substrate.persistent_bytes(),
    )
}

fn build_fixture(
    namespace: u64,
    pair_count: usize,
    mirrored: bool,
    reversed_allocation: bool,
) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let mut specifications = Vec::with_capacity(pair_count * 2);
    for pair in 0..pair_count {
        let anchor = pair as i32 * 10;
        let positions = if mirrored {
            (-anchor, -anchor - 1)
        } else {
            (anchor, anchor + 1)
        };
        specifications.push((
            pair,
            false,
            namespace + pair as u64 * 0x100 + 1,
            positions.0,
            0,
        ));
        specifications.push((
            pair,
            true,
            namespace + pair as u64 * 0x100 + 2,
            positions.1,
            1,
        ));
    }
    if reversed_allocation {
        specifications.reverse();
    }
    let mut handles = vec![[None; 2]; pair_count];
    for (pair, second, physical_id, position, region) in specifications {
        let handle = substrate.add_cell(CellSpec {
            physical_id,
            position,
            region,
            threshold: 1,
            resistance: 1_000,
        });
        handles[pair][usize::from(second)] = Some(handle);
    }
    let pairs = handles
        .into_iter()
        .map(|pair| {
            (
                pair[0].expect("first CELL allocated"),
                pair[1].expect("second CELL allocated"),
            )
        })
        .collect();
    Fixture { substrate, pairs }
}

fn enter_batch(
    fixture: &mut Fixture,
    pair_indices: &[usize],
    tick: i64,
    origin_base: u64,
    reversed: bool,
) {
    let mut order = pair_indices.to_vec();
    if reversed {
        order.reverse();
    }
    for (ordinal, pair) in order.into_iter().enumerate() {
        fixture.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: ordinal as i32,
            origin_physical: origin_base + ordinal as u64,
            target: fixture.pairs[pair].0,
            impulse: 1,
        });
    }
}

fn live_between(fixture: &Fixture, pair: usize) -> usize {
    fixture
        .substrate
        .arrows_between(fixture.pairs[pair].0, fixture.pairs[pair].1)
        .into_iter()
        .filter(|arrow| fixture.substrate.arrow_is_live(*arrow))
        .count()
}

fn max_resistance(fixture: &Fixture, pair: usize) -> u32 {
    fixture
        .substrate
        .arrows_between(fixture.pairs[pair].0, fixture.pairs[pair].1)
        .into_iter()
        .map(|arrow| fixture.substrate.arrow_resistance(arrow))
        .max()
        .unwrap_or(0)
}

fn sole_live_between(fixture: &Fixture, pair: usize) -> ArrowId {
    let live = fixture
        .substrate
        .arrows_between(fixture.pairs[pair].0, fixture.pairs[pair].1)
        .into_iter()
        .filter(|arrow| fixture.substrate.arrow_is_live(*arrow))
        .collect::<Vec<_>>();
    assert_eq!(live.len(), 1, "expected one live local variation");
    live[0]
}

fn add_execution(total: &mut WorkLedger, execution: &Execution) {
    add_work(total, &execution.work);
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
    total.ordinary_pressure_updates += next.ordinary_pressure_updates;
    total.local_structural_proposals += next.local_structural_proposals;
    total.physical_deallocations += next.physical_deallocations;
}

fn artifacts_absent() -> bool {
    [RESULT_CSV, RESULT_MD]
        .into_iter()
        .flat_map(|path| [path.to_string(), format!("{path}.staging")])
        .all(|path| !Path::new(&path).exists())
}

fn publish(results: &[CellResult], passed: bool) -> io::Result<()> {
    let mut csv = String::from(
        "cell,namespace,load,mirrored,reversed_allocation,reversed_insertion,initial_proposals,primary_return_updates,useful_live,distractor_live,held_crossings,useful_min_resistance,distractor_max_resistance,return_free_proposals,return_free_updates,return_free_live,withheld_dead,recurrent_live,stale_dead,replacement_live,replacement_crossings,old_generation,new_generation,evaluator_shuffle_inert,quiescent,persistent_bytes,work,fingerprint,p0,p1,p2,p3,p4,p5,p6,p7,p8,p9,duplicate_exact,passed\n",
    );
    for cell in results {
        let mut fields = vec![
            cell.index.to_string(),
            format!("{:x}", cell.namespace),
            cell.load.to_string(),
            cell.mirrored.to_string(),
            cell.reversed_allocation.to_string(),
            cell.reversed_insertion.to_string(),
            cell.initial_proposals.to_string(),
            cell.primary_return_updates.to_string(),
            cell.useful_live.to_string(),
            cell.distractor_live.to_string(),
            cell.held_crossings.to_string(),
            cell.useful_min_resistance.to_string(),
            cell.distractor_max_resistance.to_string(),
            cell.return_free_proposals.to_string(),
            cell.return_free_updates.to_string(),
            cell.return_free_live.to_string(),
            cell.withheld_dead.to_string(),
            cell.recurrent_live.to_string(),
            cell.stale_dead.to_string(),
            cell.replacement_live.to_string(),
            cell.replacement_crossings.to_string(),
            cell.old_generation.to_string(),
            cell.new_generation.to_string(),
            cell.evaluator_shuffle_inert.to_string(),
            cell.naturally_quiescent.to_string(),
            cell.persistent_bytes.to_string(),
            cell.work.total().to_string(),
            format!("{:016x}", cell.fingerprint),
        ];
        fields.extend(cell.claims.iter().map(bool::to_string));
        fields.push(cell.duplicate_exact.to_string());
        fields.push(cell.passed.to_string());
        csv.push_str(&fields.join(","));
        csv.push('\n');
    }

    let claims_passed = results
        .iter()
        .map(|cell| cell.claims.iter().filter(|claim| **claim).count())
        .sum::<usize>();
    let total_work = results.iter().map(|cell| cell.work.total()).sum::<u64>();
    let total_storage = results
        .iter()
        .map(|cell| cell.persistent_bytes)
        .sum::<usize>();
    let mut markdown = format!(
        "# PX5 no-new-mechanism physical plasticity-allocation MICRO v1\n\nOutcome: **{}**. Protocol: `{PROTOCOL}` at `{PROTOCOL_COMMIT}`. Frozen parent: `{FROZEN_PARENT}`. Positive PROBE: `{POSITIVE_PROBE}`.\n\nCells: `{}/{CELLS}`. Claims: `{claims_passed}/{}`. Accounted work: `{total_work}`. Accounted persistent storage: `{total_storage}` bytes.\n\n| cell | load | layout/allocation/insertion | initial | return work | useful/distractor live | held | resistance U/D | return-free P/U/live | withheld/recurrent | stale/replacement/cross | evaluator shuffle | work/storage | P0..P9 | duplicate | pass |\n|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|:---:|---:|---|:---:|:---:|\n",
        if passed { "PASS" } else { "FAIL" },
        results.iter().filter(|cell| cell.passed).count(),
        CELLS * 10,
    );
    for cell in results {
        markdown.push_str(&format!(
            "| {} | {} | {}/{}/{} | {} | {} | {}/{} | {} | {}/{} | {}/{}/{} | {}/{} | {}/{}/{} | {} | {}/{} | {} | {} | {} |\n",
            cell.index,
            cell.load,
            if cell.mirrored { "M" } else { "N" },
            if cell.reversed_allocation { "R" } else { "N" },
            if cell.reversed_insertion { "R" } else { "N" },
            cell.initial_proposals,
            cell.primary_return_updates,
            cell.useful_live,
            cell.distractor_live,
            cell.held_crossings,
            cell.useful_min_resistance,
            cell.distractor_max_resistance,
            cell.return_free_proposals,
            cell.return_free_updates,
            cell.return_free_live,
            cell.withheld_dead,
            cell.recurrent_live,
            cell.stale_dead,
            cell.replacement_live,
            cell.replacement_crossings,
            cell.evaluator_shuffle_inert,
            cell.work.total(),
            cell.persistent_bytes,
            cell.claims.iter().map(|claim| if *claim { '1' } else { '0' }).collect::<String>(),
            cell.duplicate_exact,
            cell.passed,
        ));
    }
    markdown.push_str(
        "\nOnly byte-identical retained CELL/ARROW/SPIKE physics executed. Allocation is the measured distribution of local physical work, not an organism-visible policy or representation. This is development evidence only.\n",
    );
    publish_one(RESULT_CSV, csv.as_bytes())?;
    publish_one(RESULT_MD, markdown.as_bytes())?;
    Ok(())
}

fn publish_one(path: &str, bytes: &[u8]) -> io::Result<()> {
    let staging = format!("{path}.staging");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staging)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    drop(file);
    fs::rename(staging, path)
}
