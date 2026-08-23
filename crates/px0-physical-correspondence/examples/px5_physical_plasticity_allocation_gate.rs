use px0_physical_correspondence::{
    ArrowId, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

const PROTOCOL: &str = "px5-physical-plasticity-allocation-gate-v1";
const PROTOCOL_COMMIT: &str = "1a5edf1";
const FROZEN_PARENT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const POSITIVE_MICRO: &str = "1b2c4aaaf0f6fbc858e7289b527e884cc9623d45";
const FROZEN_LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const BASE_NAMESPACE: u64 = 0x5_7000_0000;
const CELL_STRIDE: u64 = 0x0100_0000;
const CELLS: usize = 12;
const RESULT_CSV: &str = "results/px5_physical_plasticity_allocation_gate_v1.csv";
const RESULT_MD: &str = "results/px5_physical_plasticity_allocation_gate_v1.md";

#[derive(Clone)]
struct Fixture {
    substrate: PlasticSubstrate,
    pairs: Vec<(CellId, CellId)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Auxiliary {
    proposals: u64,
    updates: u64,
    live: usize,
    crossings: usize,
    quiescent: bool,
    work: WorkLedger,
    storage: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CellResult {
    index: usize,
    namespace: u64,
    load: usize,
    useful: usize,
    distance: i32,
    mirrored: bool,
    reversed_allocation: bool,
    reversed_insertion: bool,
    initial_proposals: u64,
    primary_return_updates: u64,
    expected_return_updates: u64,
    useful_live: usize,
    distractor_live: usize,
    held_crossings: usize,
    useful_min_resistance: u32,
    distractor_max_resistance: u32,
    recurrent_distractor_proposals: u64,
    return_free: Auxiliary,
    late_return: Auxiliary,
    outside_radius: Auxiliary,
    withheld_dead: bool,
    recurrent_live: usize,
    stale_dead: bool,
    replacement_live: bool,
    replacement_crossings: usize,
    old_generation: u32,
    new_generation: u32,
    evaluator_shuffle_inert: bool,
    permuted_original_live: usize,
    permuted_returned_live: usize,
    naturally_quiescent: bool,
    work: WorkLedger,
    persistent_bytes: usize,
    fingerprint: u64,
    claims: [bool; 12],
    duplicate_exact: bool,
    passed: bool,
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("--preflight") => {
            println!(
                "protocol={PROTOCOL} protocol_commit={PROTOCOL_COMMIT} parent={FROZEN_PARENT} micro={POSITIVE_MICRO} law={FROZEN_LAW_SHA256} cells={CELLS} artifacts_absent={}",
                artifacts_absent()
            );
            if !artifacts_absent() {
                std::process::exit(1);
            }
        }
        Some("--gate") if std::env::args().count() == 2 => {
            if !artifacts_absent() {
                eprintln!("refusing to overwrite or rescue frozen GATE artifacts");
                std::process::exit(2);
            }
            let results = run_matrix();
            let passed = results.len() == CELLS && results.iter().all(|cell| cell.passed);
            if let Err(error) = publish(&results, passed) {
                eprintln!("failed to publish immutable GATE artifacts: {error}");
                std::process::exit(1);
            }
            println!(
                "PX5_PHYSICAL_PLASTICITY_ALLOCATION_GATE_EVIDENCE_SPENT outcome={} cells={}/{} claims={}/{}",
                if passed { "PASS" } else { "FAIL" },
                results.iter().filter(|cell| cell.passed).count(),
                CELLS,
                results
                    .iter()
                    .map(|cell| cell.claims.iter().filter(|claim| **claim).count())
                    .sum::<usize>(),
                CELLS * 12
            );
            if !passed {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("refusing execution: pass exactly --preflight or --gate");
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
    let load = [32, 64, 128][index % 3];
    let useful = if (index / 3) & 1 == 0 { 4 } else { 8 };
    let distance = if index < 6 { 1 } else { 2 };
    let mirrored = index & 1 != 0;
    let reversed_allocation = (index / 2) & 1 != 0;
    let reversed_insertion = (index / 4) & 1 != 0;
    let mut fixture = build_fixture(
        namespace,
        useful + load,
        distance,
        mirrored,
        reversed_allocation,
    );
    let mut work = WorkLedger::default();
    let mut naturally_quiescent = true;
    let all = (0..fixture.pairs.len()).collect::<Vec<_>>();
    let useful_pairs = (0..useful).collect::<Vec<_>>();
    let hot = (0..useful / 2).collect::<Vec<_>>();
    let recurring_distractors = (useful..useful + load.min(8)).collect::<Vec<_>>();

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
        &useful_pairs,
        2,
        namespace + 0x20000,
        reversed_insertion,
    );
    let returned = fixture.substrate.propagate();
    naturally_quiescent &= returned.naturally_quiescent;
    let mut primary_return_updates = returned.work.local_return_updates;
    add_execution(&mut work, &returned);

    run_start_return(
        &mut fixture,
        &hot,
        10,
        namespace + 0x30000,
        reversed_insertion,
        &mut naturally_quiescent,
        &mut work,
        &mut primary_return_updates,
    );
    let mut recurrent_distractor_proposals = 0;
    enter_batch(
        &mut fixture,
        &recurring_distractors,
        15,
        namespace + 0x40000,
        reversed_insertion,
    );
    let distractor_15 = fixture.substrate.propagate();
    naturally_quiescent &= distractor_15.naturally_quiescent;
    recurrent_distractor_proposals += distractor_15.work.local_structural_proposals;
    add_execution(&mut work, &distractor_15);
    run_start_return(
        &mut fixture,
        &useful_pairs,
        20,
        namespace + 0x50000,
        reversed_insertion,
        &mut naturally_quiescent,
        &mut work,
        &mut primary_return_updates,
    );

    let mut tick_30 = hot.clone();
    tick_30.extend(recurring_distractors.iter().copied());
    enter_batch(
        &mut fixture,
        &tick_30,
        30,
        namespace + 0x60000,
        reversed_insertion,
    );
    let start_30 = fixture.substrate.propagate();
    naturally_quiescent &= start_30.naturally_quiescent;
    recurrent_distractor_proposals += start_30.work.local_structural_proposals;
    add_execution(&mut work, &start_30);
    enter_batch(
        &mut fixture,
        &hot,
        32,
        namespace + 0x61000,
        reversed_insertion,
    );
    let return_32 = fixture.substrate.propagate();
    naturally_quiescent &= return_32.naturally_quiescent;
    primary_return_updates += return_32.work.local_return_updates;
    add_execution(&mut work, &return_32);

    run_start_return(
        &mut fixture,
        &useful_pairs,
        40,
        namespace + 0x70000,
        reversed_insertion,
        &mut naturally_quiescent,
        &mut work,
        &mut primary_return_updates,
    );
    enter_batch(
        &mut fixture,
        &recurring_distractors,
        45,
        namespace + 0x80000,
        reversed_insertion,
    );
    let distractor_45 = fixture.substrate.propagate();
    naturally_quiescent &= distractor_45.naturally_quiescent;
    recurrent_distractor_proposals += distractor_45.work.local_structural_proposals;
    add_execution(&mut work, &distractor_45);
    run_start_return(
        &mut fixture,
        &hot,
        50,
        namespace + 0x90000,
        reversed_insertion,
        &mut naturally_quiescent,
        &mut work,
        &mut primary_return_updates,
    );

    add_work(&mut work, &fixture.substrate.advance_time(60));
    let useful_live = (0..useful).map(|pair| live_between(&fixture, pair)).sum();
    let distractor_live = (useful..fixture.pairs.len())
        .map(|pair| live_between(&fixture, pair))
        .sum();
    let useful_min_resistance = (0..useful)
        .map(|pair| max_resistance(&fixture, pair))
        .min()
        .unwrap_or(0);
    let distractor_max_resistance = (useful..fixture.pairs.len())
        .map(|pair| max_resistance(&fixture, pair))
        .max()
        .unwrap_or(0);
    let expected_return_updates = (3 * useful + 3 * (useful / 2)) as u64;

    enter_batch(
        &mut fixture,
        &useful_pairs,
        60,
        namespace + 0xa0000,
        reversed_insertion,
    );
    let held = fixture.substrate.propagate();
    naturally_quiescent &= held.naturally_quiescent;
    let held_crossings = held.crossings.len();
    add_execution(&mut work, &held);
    enter_batch(
        &mut fixture,
        &useful_pairs,
        62,
        namespace + 0xb0000,
        reversed_insertion,
    );
    let held_return = fixture.substrate.propagate();
    naturally_quiescent &= held_return.naturally_quiescent;
    add_execution(&mut work, &held_return);

    let old = sole_live_between(&fixture, 0);
    let recurrent_useful = (1..useful).collect::<Vec<_>>();
    let mut ignored_updates = 0;
    run_start_return(
        &mut fixture,
        &recurrent_useful,
        70,
        namespace + 0xc0000,
        reversed_insertion,
        &mut naturally_quiescent,
        &mut work,
        &mut ignored_updates,
    );
    run_start_return(
        &mut fixture,
        &recurrent_useful,
        80,
        namespace + 0xd0000,
        reversed_insertion,
        &mut naturally_quiescent,
        &mut work,
        &mut ignored_updates,
    );
    add_work(&mut work, &fixture.substrate.advance_time(90));
    let withheld_dead = live_between(&fixture, 0) == 0;
    let stale_dead = !fixture.substrate.arrow_is_live(old);
    let old_generation = fixture.substrate.arrow_generation(old);
    let recurrent_live = (1..useful).map(|pair| live_between(&fixture, pair)).sum();
    enter_batch(
        &mut fixture,
        &[0],
        90,
        namespace + 0xe0000,
        reversed_insertion,
    );
    let replacement = fixture.substrate.propagate();
    naturally_quiescent &= replacement.naturally_quiescent;
    let replacement_crossings = replacement.crossings.len();
    add_execution(&mut work, &replacement);
    enter_batch(
        &mut fixture,
        &[0],
        92,
        namespace + 0xe1000,
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
        && evaluator_allocation.iter().sum::<usize>() == useful
        && (useful..fixture.pairs.len()).all(|pair| live_between(&fixture, pair) == 0);

    let return_free = run_return_free(
        namespace + 0x0020_0000,
        distance,
        mirrored,
        reversed_allocation,
        reversed_insertion,
    );
    let late_return = run_late_return(
        namespace + 0x0040_0000,
        distance,
        mirrored,
        reversed_allocation,
        reversed_insertion,
    );
    let outside_radius = run_outside_radius(
        namespace + 0x0060_0000,
        mirrored,
        reversed_allocation,
        reversed_insertion,
    );
    let permutation = run_permutation(
        namespace + 0x0080_0000,
        useful,
        load,
        distance,
        mirrored,
        reversed_allocation,
        reversed_insertion,
    );
    naturally_quiescent &=
        return_free.quiescent && late_return.quiescent && outside_radius.quiescent && permutation.2;
    add_work(&mut work, &return_free.work);
    add_work(&mut work, &late_return.work);
    add_work(&mut work, &outside_radius.work);
    add_work(&mut work, &permutation.3);
    let permuted_original_live = permutation.0;
    let permuted_returned_live = permutation.1;
    let persistent_bytes = fixture.substrate.persistent_bytes()
        + return_free.storage
        + late_return.storage
        + outside_radius.storage
        + permutation.4;
    let fingerprint = fixture.substrate.complete_fingerprint();
    let claims = [
        namespace == BASE_NAMESPACE + index as u64 * CELL_STRIDE,
        initial_proposals == (useful + load) as u64,
        primary_return_updates == expected_return_updates,
        useful_live == useful && held_crossings == useful,
        distractor_live == 0 && recurrent_distractor_proposals == 3 * load.min(8) as u64,
        useful_min_resistance > 0 && useful_min_resistance > distractor_max_resistance,
        return_free.proposals == 6 && return_free.updates == 0 && return_free.live == 0,
        late_return.updates == 0
            && late_return.live == 0
            && outside_radius.proposals == 0
            && outside_radius.crossings == 0,
        withheld_dead
            && recurrent_live == useful - 1
            && stale_dead
            && replacement_live
            && replacement_crossings == 1,
        evaluator_shuffle_inert,
        permuted_original_live == 0 && permuted_returned_live == useful,
        naturally_quiescent && work.total() > 0 && persistent_bytes > 0,
    ];

    CellResult {
        index,
        namespace,
        load,
        useful,
        distance,
        mirrored,
        reversed_allocation,
        reversed_insertion,
        initial_proposals,
        primary_return_updates,
        expected_return_updates,
        useful_live,
        distractor_live,
        held_crossings,
        useful_min_resistance,
        distractor_max_resistance,
        recurrent_distractor_proposals,
        return_free,
        late_return,
        outside_radius,
        withheld_dead,
        recurrent_live,
        stale_dead,
        replacement_live,
        replacement_crossings,
        old_generation,
        new_generation,
        evaluator_shuffle_inert,
        permuted_original_live,
        permuted_returned_live,
        naturally_quiescent,
        work,
        persistent_bytes,
        fingerprint,
        claims,
        duplicate_exact: false,
        passed: false,
    }
}

#[allow(clippy::too_many_arguments)]
fn run_start_return(
    fixture: &mut Fixture,
    pairs: &[usize],
    tick: i64,
    origin: u64,
    reversed: bool,
    quiescent: &mut bool,
    work: &mut WorkLedger,
    updates: &mut u64,
) {
    enter_batch(fixture, pairs, tick, origin, reversed);
    let start = fixture.substrate.propagate();
    *quiescent &= start.naturally_quiescent;
    add_execution(work, &start);
    enter_batch(fixture, pairs, tick + 2, origin + 0x800, reversed);
    let returned = fixture.substrate.propagate();
    *quiescent &= returned.naturally_quiescent;
    *updates += returned.work.local_return_updates;
    add_execution(work, &returned);
}

fn run_return_free(
    namespace: u64,
    distance: i32,
    mirrored: bool,
    reversed_allocation: bool,
    reversed_insertion: bool,
) -> Auxiliary {
    let mut fixture = build_fixture(namespace, 1, distance, mirrored, reversed_allocation);
    let mut work = WorkLedger::default();
    let mut quiescent = true;
    for (ordinal, tick) in [0_i64, 6, 12, 18, 24, 30].into_iter().enumerate() {
        enter_batch(
            &mut fixture,
            &[0],
            tick,
            namespace + 0x10000 + ordinal as u64,
            reversed_insertion,
        );
        let run = fixture.substrate.propagate();
        quiescent &= run.naturally_quiescent;
        add_execution(&mut work, &run);
    }
    add_work(&mut work, &fixture.substrate.advance_time(36));
    Auxiliary {
        proposals: work.local_structural_proposals,
        updates: work.local_return_updates,
        live: live_between(&fixture, 0),
        crossings: 0,
        quiescent,
        work,
        storage: fixture.substrate.persistent_bytes(),
    }
}

fn run_late_return(
    namespace: u64,
    distance: i32,
    mirrored: bool,
    reversed_allocation: bool,
    reversed_insertion: bool,
) -> Auxiliary {
    let mut fixture = build_fixture(namespace, 1, distance, mirrored, reversed_allocation);
    let mut work = WorkLedger::default();
    enter_batch(
        &mut fixture,
        &[0],
        0,
        namespace + 0x10000,
        reversed_insertion,
    );
    let first = fixture.substrate.propagate();
    let mut quiescent = first.naturally_quiescent;
    add_execution(&mut work, &first);
    enter_batch(
        &mut fixture,
        &[0],
        6,
        namespace + 0x20000,
        reversed_insertion,
    );
    let late = fixture.substrate.propagate();
    quiescent &= late.naturally_quiescent;
    add_execution(&mut work, &late);
    add_work(&mut work, &fixture.substrate.advance_time(12));
    Auxiliary {
        proposals: work.local_structural_proposals,
        updates: work.local_return_updates,
        live: live_between(&fixture, 0),
        crossings: first.crossings.len() + late.crossings.len(),
        quiescent,
        work,
        storage: fixture.substrate.persistent_bytes(),
    }
}

fn run_outside_radius(
    namespace: u64,
    mirrored: bool,
    reversed_allocation: bool,
    reversed_insertion: bool,
) -> Auxiliary {
    let mut fixture = build_fixture(namespace, 1, 3, mirrored, reversed_allocation);
    enter_batch(
        &mut fixture,
        &[0],
        0,
        namespace + 0x10000,
        reversed_insertion,
    );
    let run = fixture.substrate.propagate();
    Auxiliary {
        proposals: run.work.local_structural_proposals,
        updates: run.work.local_return_updates,
        live: live_between(&fixture, 0),
        crossings: run.crossings.len(),
        quiescent: run.naturally_quiescent,
        work: run.work,
        storage: fixture.substrate.persistent_bytes(),
    }
}

#[allow(clippy::too_many_arguments)]
fn run_permutation(
    namespace: u64,
    useful: usize,
    load: usize,
    distance: i32,
    mirrored: bool,
    reversed_allocation: bool,
    reversed_insertion: bool,
) -> (usize, usize, bool, WorkLedger, usize) {
    let mut fixture = build_fixture(
        namespace,
        useful + load,
        distance,
        mirrored,
        reversed_allocation,
    );
    let mut work = WorkLedger::default();
    let all = (0..fixture.pairs.len()).collect::<Vec<_>>();
    enter_batch(
        &mut fixture,
        &all,
        0,
        namespace + 0x10000,
        reversed_insertion,
    );
    let first = fixture.substrate.propagate();
    let mut quiescent = first.naturally_quiescent;
    add_execution(&mut work, &first);
    let shifted = (useful..2 * useful).collect::<Vec<_>>();
    enter_batch(
        &mut fixture,
        &shifted,
        2,
        namespace + 0x20000,
        reversed_insertion,
    );
    let returned = fixture.substrate.propagate();
    quiescent &= returned.naturally_quiescent;
    add_execution(&mut work, &returned);
    add_work(&mut work, &fixture.substrate.advance_time(20));
    let original_live = (0..useful).map(|pair| live_between(&fixture, pair)).sum();
    let returned_live = shifted
        .iter()
        .map(|pair| live_between(&fixture, *pair))
        .sum();
    (
        original_live,
        returned_live,
        quiescent,
        work,
        fixture.substrate.persistent_bytes(),
    )
}

fn build_fixture(
    namespace: u64,
    pair_count: usize,
    distance: i32,
    mirrored: bool,
    reversed_allocation: bool,
) -> Fixture {
    let mut substrate = PlasticSubstrate::new();
    let mut specifications = Vec::with_capacity(pair_count * 2);
    for pair in 0..pair_count {
        let anchor = pair as i32 * 12;
        let positions = if mirrored {
            (-anchor, -anchor - distance)
        } else {
            (anchor, anchor + distance)
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
        "cell,namespace,load,useful,distance,mirrored,reversed_allocation,reversed_insertion,initial_proposals,primary_return_updates,expected_return_updates,useful_live,distractor_live,held_crossings,useful_min_resistance,distractor_max_resistance,recurrent_distractor_proposals,return_free_proposals,return_free_updates,return_free_live,late_return_updates,late_return_live,outside_proposals,outside_crossings,withheld_dead,recurrent_live,stale_dead,replacement_live,replacement_crossings,old_generation,new_generation,evaluator_shuffle_inert,permuted_original_live,permuted_returned_live,quiescent,persistent_bytes,work,fingerprint,p0,p1,p2,p3,p4,p5,p6,p7,p8,p9,p10,p11,duplicate_exact,passed\n",
    );
    for cell in results {
        let mut fields = vec![
            cell.index.to_string(),
            format!("{:x}", cell.namespace),
            cell.load.to_string(),
            cell.useful.to_string(),
            cell.distance.to_string(),
            cell.mirrored.to_string(),
            cell.reversed_allocation.to_string(),
            cell.reversed_insertion.to_string(),
            cell.initial_proposals.to_string(),
            cell.primary_return_updates.to_string(),
            cell.expected_return_updates.to_string(),
            cell.useful_live.to_string(),
            cell.distractor_live.to_string(),
            cell.held_crossings.to_string(),
            cell.useful_min_resistance.to_string(),
            cell.distractor_max_resistance.to_string(),
            cell.recurrent_distractor_proposals.to_string(),
            cell.return_free.proposals.to_string(),
            cell.return_free.updates.to_string(),
            cell.return_free.live.to_string(),
            cell.late_return.updates.to_string(),
            cell.late_return.live.to_string(),
            cell.outside_radius.proposals.to_string(),
            cell.outside_radius.crossings.to_string(),
            cell.withheld_dead.to_string(),
            cell.recurrent_live.to_string(),
            cell.stale_dead.to_string(),
            cell.replacement_live.to_string(),
            cell.replacement_crossings.to_string(),
            cell.old_generation.to_string(),
            cell.new_generation.to_string(),
            cell.evaluator_shuffle_inert.to_string(),
            cell.permuted_original_live.to_string(),
            cell.permuted_returned_live.to_string(),
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
        "# PX5 no-new-mechanism physical plasticity-allocation GATE v1\n\nOutcome: **{}**. Protocol: `{PROTOCOL}` at `{PROTOCOL_COMMIT}`. Frozen parent: `{FROZEN_PARENT}`. Positive MICRO: `{POSITIVE_MICRO}`. Authority: **ABSENT**.\n\nCells: `{}/{CELLS}`. Claims: `{claims_passed}/{}`. Accounted work: `{total_work}`. Accounted persistent storage: `{total_storage}` bytes.\n\n| cell | load/U/d | M/A/I | initial | return work | live U/D | held | resistance U/D | repeated D proposals | RF P/U/live | late U/live | outside P/X | withheld/recurrent | stale/replacement/X | eval shuffle | permuted original/returned | work/storage | P0..P11 | duplicate | pass |\n|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|:---:|---:|---:|---|:---:|:---:|\n",
        if passed { "PASS" } else { "FAIL" },
        results.iter().filter(|cell| cell.passed).count(),
        CELLS * 12,
    );
    for cell in results {
        markdown.push_str(&format!(
            "| {} | {}/{}/{} | {}/{}/{} | {} | {}/{} | {}/{} | {} | {}/{} | {} | {}/{}/{} | {}/{} | {}/{} | {}/{} | {}/{}/{} | {} | {}/{} | {}/{} | {} | {} | {} |\n",
            cell.index,
            cell.load,
            cell.useful,
            cell.distance,
            if cell.mirrored { "M" } else { "N" },
            if cell.reversed_allocation { "R" } else { "N" },
            if cell.reversed_insertion { "R" } else { "N" },
            cell.initial_proposals,
            cell.primary_return_updates,
            cell.expected_return_updates,
            cell.useful_live,
            cell.distractor_live,
            cell.held_crossings,
            cell.useful_min_resistance,
            cell.distractor_max_resistance,
            cell.recurrent_distractor_proposals,
            cell.return_free.proposals,
            cell.return_free.updates,
            cell.return_free.live,
            cell.late_return.updates,
            cell.late_return.live,
            cell.outside_radius.proposals,
            cell.outside_radius.crossings,
            cell.withheld_dead,
            cell.recurrent_live,
            cell.stale_dead,
            cell.replacement_live,
            cell.replacement_crossings,
            cell.evaluator_shuffle_inert,
            cell.permuted_original_live,
            cell.permuted_returned_live,
            cell.work.total(),
            cell.persistent_bytes,
            cell.claims.iter().map(|claim| if *claim { '1' } else { '0' }).collect::<String>(),
            cell.duplicate_exact,
            cell.passed,
        ));
    }
    markdown.push_str(
        "\nClassification: **A — existing PX0--PX2 physics sufficient; no mechanism added**. Only retained CELL/ARROW/SPIKE physics executed. This is non-authoritative development readiness, not a definitive or authority result.\n",
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
