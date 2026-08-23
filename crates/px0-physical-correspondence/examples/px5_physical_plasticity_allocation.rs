use px0_physical_correspondence::{
    ArrowId, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

const PROTOCOL: &str = "px5-physical-plasticity-allocation-probe-v1";
const PROTOCOL_COMMIT: &str = "5c8391a";
const FROZEN_PARENT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const FROZEN_LAW_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const BASE_NAMESPACE: u64 = 0x5_5000_0000;
const CELL_STRIDE: u64 = 0x0100_0000;
const CELLS: usize = 4;
const PAIRS: usize = 5;
const RESULT_CSV: &str = "results/px5_physical_plasticity_allocation_probe_v1.csv";
const RESULT_MD: &str = "results/px5_physical_plasticity_allocation_probe_v1.md";

#[derive(Clone)]
struct Fixture {
    substrate: PlasticSubstrate,
    pairs: [(CellId, CellId); PAIRS],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CellResult {
    index: usize,
    namespace: u64,
    mirrored: bool,
    reversed_allocation: bool,
    initial_proposals: u64,
    useful_return_updates: u64,
    useful_live_after_pressure: usize,
    distractor_live_after_pressure: usize,
    return_free_proposals: u64,
    return_free_updates: u64,
    return_free_live: usize,
    old_generation: u32,
    new_generation: u32,
    stale_dead: bool,
    reacquired_live: bool,
    reacquired_crossings: usize,
    label_shuffle_inert: bool,
    naturally_quiescent: bool,
    persistent_bytes: usize,
    work: WorkLedger,
    claims: [bool; 8],
    duplicate_exact: bool,
    passed: bool,
    final_fingerprint: u64,
}

fn main() {
    let argument = std::env::args().nth(1);
    match argument.as_deref() {
        Some("--preflight") => {
            println!(
                "protocol={PROTOCOL} protocol_commit={PROTOCOL_COMMIT} parent={FROZEN_PARENT} law={FROZEN_LAW_SHA256} cells={CELLS} artifacts_absent={}",
                artifacts_absent()
            );
            if !artifacts_absent() {
                std::process::exit(1);
            }
        }
        Some("--probe") if std::env::args().count() == 2 => {
            if !artifacts_absent() {
                eprintln!("refusing to overwrite or rescue frozen PROBE artifacts");
                std::process::exit(2);
            }
            let results = run_matrix();
            let passed = results.len() == CELLS && results.iter().all(|cell| cell.passed);
            if let Err(error) = publish(&results, passed) {
                eprintln!("failed to publish immutable PROBE artifacts: {error}");
                std::process::exit(1);
            }
            println!(
                "PX5_PHYSICAL_PLASTICITY_ALLOCATION_PROBE_EVIDENCE_SPENT outcome={} cells={}/{} claims={}/{}",
                if passed { "PASS" } else { "FAIL" },
                results.iter().filter(|cell| cell.passed).count(),
                CELLS,
                results
                    .iter()
                    .map(|cell| cell.claims.iter().filter(|claim| **claim).count())
                    .sum::<usize>(),
                CELLS * 8
            );
            if !passed {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("refusing execution: pass exactly --preflight or --probe");
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
    let mirrored = index & 1 != 0;
    let reversed_allocation = index & 2 != 0;
    let mut fixture = build_fixture(namespace, mirrored, reversed_allocation, PAIRS);
    let mut work = WorkLedger::default();
    let mut naturally_quiescent = true;

    for (ordinal, (first, _)) in fixture.pairs.iter().enumerate() {
        fixture.substrate.enter(SpikeInput {
            arrival_tick: 0,
            phase: if reversed_allocation {
                -(ordinal as i32)
            } else {
                ordinal as i32
            },
            origin_physical: namespace + 0x1000 + ordinal as u64,
            target: *first,
            impulse: 1,
        });
    }
    let initial = fixture.substrate.propagate();
    naturally_quiescent &= initial.naturally_quiescent;
    add_execution(&mut work, &initial);
    let initial_proposals = initial.work.local_structural_proposals;

    enter_one(
        &mut fixture.substrate,
        fixture.pairs[0].0,
        namespace + 0x2000,
        2,
    );
    let returned = fixture.substrate.propagate();
    naturally_quiescent &= returned.naturally_quiescent;
    let useful_return_updates = returned.work.local_return_updates;
    add_execution(&mut work, &returned);

    add_work(&mut work, &fixture.substrate.advance_time(20));
    let useful_live_after_pressure = live_between(&fixture.substrate, fixture.pairs[0]);
    let distractor_live_after_pressure = fixture.pairs[1..]
        .iter()
        .map(|pair| live_between(&fixture.substrate, *pair))
        .sum();

    enter_one(
        &mut fixture.substrate,
        fixture.pairs[0].0,
        namespace + 0x3000,
        20,
    );
    let held = fixture.substrate.propagate();
    naturally_quiescent &= held.naturally_quiescent;
    add_execution(&mut work, &held);
    enter_one(
        &mut fixture.substrate,
        fixture.pairs[0].0,
        namespace + 0x3001,
        22,
    );
    let held_return = fixture.substrate.propagate();
    naturally_quiescent &= held_return.naturally_quiescent;
    add_execution(&mut work, &held_return);

    let old = sole_live_between(&fixture.substrate, fixture.pairs[0]);
    add_work(&mut work, &fixture.substrate.advance_time(72));
    let old_generation = fixture.substrate.arrow_generation(old);
    let stale_dead = !fixture.substrate.arrow_is_live(old);

    enter_one(
        &mut fixture.substrate,
        fixture.pairs[0].0,
        namespace + 0x4000,
        72,
    );
    let reacquired = fixture.substrate.propagate();
    naturally_quiescent &= reacquired.naturally_quiescent;
    let reacquired_crossings = reacquired.crossings.len();
    add_execution(&mut work, &reacquired);
    enter_one(
        &mut fixture.substrate,
        fixture.pairs[0].0,
        namespace + 0x4001,
        74,
    );
    let reacquired_return = fixture.substrate.propagate();
    naturally_quiescent &= reacquired_return.naturally_quiescent;
    add_execution(&mut work, &reacquired_return);
    let new = sole_live_between(&fixture.substrate, fixture.pairs[0]);
    let new_generation = fixture.substrate.arrow_generation(new);
    let reacquired_live = new != old && fixture.substrate.arrow_is_live(new);

    let before_shuffle = fixture.substrate.complete_fingerprint();
    let mut reversed_observations = fixture
        .pairs
        .iter()
        .map(|pair| live_between(&fixture.substrate, *pair))
        .collect::<Vec<_>>();
    reversed_observations.reverse();
    let label_shuffle_inert = fixture.substrate.complete_fingerprint() == before_shuffle
        && reversed_observations.first() == Some(&0)
        && reversed_observations.last() == Some(&1);

    let return_free = run_return_free(namespace + 0x0080_0000, mirrored, reversed_allocation);
    naturally_quiescent &= return_free.3;
    add_work(&mut work, &return_free.4);

    let final_fingerprint = fixture.substrate.complete_fingerprint();
    let persistent_bytes = fixture.substrate.persistent_bytes() + return_free.5;
    let claims = [
        namespace == BASE_NAMESPACE + index as u64 * CELL_STRIDE,
        initial_proposals == PAIRS as u64,
        useful_return_updates == 1 && useful_live_after_pressure == 1,
        distractor_live_after_pressure == 0,
        return_free.0 == 4 && return_free.1 == 0 && return_free.2 == 0,
        stale_dead && reacquired_live && reacquired_crossings == 1,
        label_shuffle_inert,
        naturally_quiescent && work.total() > 0 && persistent_bytes > 0,
    ];

    CellResult {
        index,
        namespace,
        mirrored,
        reversed_allocation,
        initial_proposals,
        useful_return_updates,
        useful_live_after_pressure,
        distractor_live_after_pressure,
        return_free_proposals: return_free.0,
        return_free_updates: return_free.1,
        return_free_live: return_free.2,
        old_generation,
        new_generation,
        stale_dead,
        reacquired_live,
        reacquired_crossings,
        label_shuffle_inert,
        naturally_quiescent,
        persistent_bytes,
        work,
        claims,
        duplicate_exact: false,
        passed: false,
        final_fingerprint,
    }
}

fn run_return_free(
    namespace: u64,
    mirrored: bool,
    reversed_allocation: bool,
) -> (u64, u64, usize, bool, WorkLedger, usize) {
    let mut fixture = build_fixture(namespace, mirrored, reversed_allocation, 1);
    let mut work = WorkLedger::default();
    let mut quiescent = true;
    for (ordinal, tick) in [0_i64, 6, 12, 18].into_iter().enumerate() {
        enter_one(
            &mut fixture.substrate,
            fixture.pairs[0].0,
            namespace + 0x5000 + ordinal as u64,
            tick,
        );
        let run = fixture.substrate.propagate();
        quiescent &= run.naturally_quiescent;
        add_execution(&mut work, &run);
    }
    add_work(&mut work, &fixture.substrate.advance_time(24));
    (
        work.local_structural_proposals,
        work.local_return_updates,
        live_between(&fixture.substrate, fixture.pairs[0]),
        quiescent,
        work,
        fixture.substrate.persistent_bytes(),
    )
}

fn build_fixture(
    namespace: u64,
    mirrored: bool,
    reversed_allocation: bool,
    active_pairs: usize,
) -> Fixture {
    assert!((1..=PAIRS).contains(&active_pairs));
    let mut substrate = PlasticSubstrate::new();
    let mut specifications = Vec::with_capacity(PAIRS * 2);
    for pair in 0..PAIRS {
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
    let mut handles = [[None; 2]; PAIRS];
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
    let all_pairs = std::array::from_fn(|pair| {
        (
            handles[pair][0].expect("first CELL allocated"),
            handles[pair][1].expect("second CELL allocated"),
        )
    });
    let mut pairs = all_pairs;
    for pair in active_pairs..PAIRS {
        pairs[pair] = pairs[0];
    }
    Fixture { substrate, pairs }
}

fn enter_one(substrate: &mut PlasticSubstrate, target: CellId, origin: u64, tick: i64) {
    substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase: 0,
        origin_physical: origin,
        target,
        impulse: 1,
    });
}

fn live_between(substrate: &PlasticSubstrate, pair: (CellId, CellId)) -> usize {
    substrate
        .arrows_between(pair.0, pair.1)
        .into_iter()
        .filter(|arrow| substrate.arrow_is_live(*arrow))
        .count()
}

fn sole_live_between(substrate: &PlasticSubstrate, pair: (CellId, CellId)) -> ArrowId {
    let live = substrate
        .arrows_between(pair.0, pair.1)
        .into_iter()
        .filter(|arrow| substrate.arrow_is_live(*arrow))
        .collect::<Vec<_>>();
    assert_eq!(live.len(), 1, "expected exactly one live local variation");
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
        "cell,namespace,mirrored,reversed_allocation,initial_proposals,useful_return_updates,useful_live,distractor_live,return_free_proposals,return_free_updates,return_free_live,old_generation,new_generation,stale_dead,reacquired_live,reacquired_crossings,label_shuffle_inert,quiescent,persistent_bytes,work,fingerprint,p0,p1,p2,p3,p4,p5,p6,p7,duplicate_exact,passed\n",
    );
    for cell in results {
        let mut fields = vec![
            cell.index.to_string(),
            format!("{:x}", cell.namespace),
            cell.mirrored.to_string(),
            cell.reversed_allocation.to_string(),
            cell.initial_proposals.to_string(),
            cell.useful_return_updates.to_string(),
            cell.useful_live_after_pressure.to_string(),
            cell.distractor_live_after_pressure.to_string(),
            cell.return_free_proposals.to_string(),
            cell.return_free_updates.to_string(),
            cell.return_free_live.to_string(),
            cell.old_generation.to_string(),
            cell.new_generation.to_string(),
            cell.stale_dead.to_string(),
            cell.reacquired_live.to_string(),
            cell.reacquired_crossings.to_string(),
            cell.label_shuffle_inert.to_string(),
            cell.naturally_quiescent.to_string(),
            cell.persistent_bytes.to_string(),
            cell.work.total().to_string(),
            format!("{:016x}", cell.final_fingerprint),
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
        "# PX5 no-new-mechanism physical plasticity-allocation PROBE v1\n\nOutcome: **{}**. Protocol: `{PROTOCOL}` at `{PROTOCOL_COMMIT}`. Frozen parent: `{FROZEN_PARENT}`.\n\nCells: `{}/{CELLS}`. Claims: `{claims_passed}/{}`. Accounted work: `{total_work}`. Accounted persistent storage: `{total_storage}` bytes.\n\n| cell | layout/allocation | initial variation | useful return/live | distractor live | return-free P/U/live | stale/reacquired/crossings | label shuffle inert | work/storage | P0..P7 | duplicate | pass |\n|---:|---|---:|---:|---:|---:|---:|:---:|---:|---|:---:|:---:|\n",
        if passed { "PASS" } else { "FAIL" },
        results.iter().filter(|cell| cell.passed).count(),
        CELLS * 8,
    );
    for cell in results {
        markdown.push_str(&format!(
            "| {} | {}/{} | {} | {}/{} | {} | {}/{}/{} | {}/{}/{} | {} | {}/{} | {} | {} | {} |\n",
            cell.index,
            if cell.mirrored { "mirror" } else { "normal" },
            if cell.reversed_allocation {
                "reverse"
            } else {
                "normal"
            },
            cell.initial_proposals,
            cell.useful_return_updates,
            cell.useful_live_after_pressure,
            cell.distractor_live_after_pressure,
            cell.return_free_proposals,
            cell.return_free_updates,
            cell.return_free_live,
            cell.stale_dead,
            cell.reacquired_live,
            cell.reacquired_crossings,
            cell.label_shuffle_inert,
            cell.work.total(),
            cell.persistent_bytes,
            cell.claims
                .iter()
                .map(|claim| if *claim { '1' } else { '0' })
                .collect::<String>(),
            cell.duplicate_exact,
            cell.passed,
        ));
    }
    markdown.push_str(
        "\nThe organism path was the byte-identical PX0--PX2 substrate law. No allocator, encounter representation, proposal-site label, supplied gate, old M5 schema, serializer, adapter, or evaluator-selected mutation executed. This is development evidence only and advances no authoritative generation.\n",
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
