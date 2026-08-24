#![forbid(unsafe_code)]

use lr1_modulatory_physical_return::{
    ArrowId, ArrowSpec, CellId, CellSpec, PlasticSubstrate, TraceEntry, TransmissionMode,
};
use px4_lrc_lifetime::{arrive, field};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const LAW_HASH: &str = "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10";
const PX4_SOURCE_HASH: &str =
    "a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71";
const PX4_HANDOFF_HASH: &str =
    "848c3b030824d6bc404dddad9498046b55d1f71c4d7e4ff10fda05cffb29e995";
const PX4_CSV_HASH: &str =
    "050a2b489e41d13e8d8a3d55dd7d69c6e06894b85b2c172f7dc24614af09aeaa";
const PX4_REPORT_HASH: &str =
    "445c465ba61cc12c0ece84a8ebb9a83bea1e67c1a4d640964cc7d93c0dbe4390";
const PROTOCOL_HASH: &str =
    "497c559f9477252195e870d2b4be8dfd38f09b163438ecce7047e2f63077c443";

const ROOTS: [u64; 8] = [561001, 561002, 561003, 561004, 561005, 561006, 561007, 561008];
const LOADS: [usize; 3] = [8, 32, 128];
const WORK_LIMIT: u64 = 100_000;
const BYTE_LIMIT: usize = 24_000;
const CSV: &str = "results/px5_lrc_allocation_authority_v1.csv";
const REPORT: &str = "results/px5_lrc_allocation_authority_v1.md";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Site {
    source: CellId,
    effect: CellId,
    source_physical: u64,
    effect_physical: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct World {
    space: PlasticSubstrate,
    returned: Site,
    plain: Vec<Site>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OrderControl {
    ordered_updates: u64,
    ordered_resistance: u32,
    shuffled_updates: u64,
    shuffled_live: bool,
    late_updates: u64,
    late_live: bool,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StaleControl {
    proposals: u64,
    stale_firings: usize,
    old_dead: bool,
    replacement_distinct: bool,
    repair_firings: usize,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Px4Control {
    first_resistance: u32,
    recurrent_resistance: u32,
    recurrent_proposals: u64,
    old_dead: bool,
    replacement_distinct: bool,
    replacement_resistance: u32,
    work: u64,
    quiescent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    root: u64,
    load: usize,
    returned_last: bool,
    mirror: bool,
    blank: usize,
    proposals: u64,
    traversals: usize,
    updates: u64,
    first_returned: u32,
    first_plain: u32,
    live_at_30: usize,
    plain_dead: usize,
    old_at_30: u32,
    old_dead_at_40: bool,
    replacement_distinct: bool,
    replacement_after_return: u32,
    live_at_70: usize,
    reuse_proposals: u64,
    bytes_before_reuse: usize,
    bytes_after_reuse: usize,
    order: OrderControl,
    stale: StaleControl,
    px4: Px4Control,
    work: u64,
    fingerprint: u64,
    quiescent: bool,
    replay: bool,
    claims: [bool; 18],
    passed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Signature {
    proposals: u64,
    traversals: usize,
    updates: u64,
    first_returned: u32,
    first_plain: u32,
    live_at_30: usize,
    plain_dead: usize,
    replacement_after_return: u32,
    live_at_70: usize,
    reuse_proposals: u64,
    bytes_before_reuse: usize,
    bytes_after_reuse: usize,
    order_passed: bool,
    stale_passed: bool,
    px4_passed: bool,
}

fn main() {
    frozen_inputs();
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [argument] if argument == "--authority-preflight" => preflight(),
        [argument] if argument == "--authority-v1" => authority(),
        _ => std::process::exit(2),
    }
}

fn preflight() {
    assert_eq!(ROOTS.len(), 8);
    assert_eq!(ROOTS.into_iter().collect::<BTreeSet<_>>().len(), 8);
    assert_eq!(LOADS, [8, 32, 128]);
    assert_eq!(ROOTS.len() * LOADS.len(), 24);
    absent(&[CSV, REPORT]);
    println!("PX5_LRC_ALLOCATION_AUTHORITY_PREFLIGHT_OK");
}

fn authority() {
    preflight();
    eprintln!("PX5_LRC_ALLOCATION_AUTHORITY_V1_EVIDENCE_SPENT");
    let rows = ROOTS
        .into_iter()
        .flat_map(|root| LOADS.into_iter().map(move |load| replay_row(root, load)))
        .collect::<Vec<_>>();
    let global = global_claims(&rows);
    assert!(rows.iter().all(|row| row.passed));
    assert!(global.into_iter().all(|claim| claim));
    publish(CSV, &csv(&rows));
    publish(REPORT, &report(&rows, global));
    println!("PX5_LRC_ALLOCATION_AUTHORITY_V1_COMPLETE rows=24 clauses=436");
}

fn replay_row(root: u64, load: usize) -> Row {
    let first = run_row(root, load);
    let second = run_row(root, load);
    let replay = first == second;
    let mut row = first;
    row.replay = replay;
    row.claims[17] = replay;
    row.passed = row.claims.into_iter().all(|claim| claim);
    row
}

fn run_row(root: u64, load: usize) -> Row {
    let index = ROOTS
        .iter()
        .position(|candidate| *candidate == root)
        .expect("registered root");
    let returned_last = index % 2 == 1;
    let mirror = index % 4 >= 2;
    let mut world = build_world(root, load, returned_last, mirror);
    let blank = variable_arrows(&world).len();
    for site in sites(&world) {
        arrive(&mut world.space, site.source, 0, 0, site.source_physical + 8);
    }
    let first = world.space.propagate();
    let returned_arrow = only_live(&world.space, world.returned);
    let plain_arrows = world
        .plain
        .iter()
        .map(|site| only_live(&world.space, *site))
        .collect::<Vec<_>>();
    let traversals = sites(&world)
        .into_iter()
        .map(|site| fires(&first.trace, site.effect_physical))
        .sum();
    let first_returned = world.space.arrow_resistance(returned_arrow);
    let first_plain = world.space.arrow_resistance(plain_arrows[0]);
    let pressure_30 = world.space.advance_time(30);
    let live_at_30 = variable_arrows(&world)
        .into_iter()
        .filter(|arrow| world.space.arrow_is_live(*arrow))
        .count();
    let plain_dead = plain_arrows
        .iter()
        .filter(|arrow| !world.space.arrow_is_live(**arrow))
        .count();
    let old_at_30 = world.space.arrow_resistance(returned_arrow);
    let pressure_40 = world.space.advance_time(40);
    let old_dead_at_40 = !world.space.arrow_is_live(returned_arrow);

    arrive(
        &mut world.space,
        world.returned.source,
        40,
        0,
        world.returned.source_physical + 9,
    );
    let reacquisition = world.space.propagate();
    let replacement = only_live(&world.space, world.returned);
    let replacement_distinct = replacement != returned_arrow;
    let replacement_after_return = world.space.arrow_resistance(replacement);
    let pressure_70 = world.space.advance_time(70);
    let live_at_70 = variable_arrows(&world)
        .into_iter()
        .filter(|arrow| world.space.arrow_is_live(*arrow))
        .count();
    let bytes_before_reuse = world.space.persistent_bytes();
    let mut reuse_proposals = 0;
    let mut reuse_work = 0;
    let mut reuse_quiescent = true;
    for tick in [70, 75, 80, 85] {
        arrive(
            &mut world.space,
            world.returned.source,
            tick,
            0,
            world.returned.source_physical + 10 + tick as u64,
        );
        let execution = world.space.propagate();
        reuse_proposals += execution.work.local_structural_proposals;
        reuse_work += execution.work.total();
        reuse_quiescent &= execution.naturally_quiescent;
    }
    let bytes_after_reuse = world.space.persistent_bytes();
    let order = order_control((root << 32) + 0x0200_0000, mirror);
    let stale = stale_control((root << 32) + 0x0300_0000, mirror);
    let px4 = px4_control((root << 32) + 0x0400_0000, returned_last, mirror);
    let work = first.work.total()
        + pressure_30.total()
        + pressure_40.total()
        + reacquisition.work.total()
        + pressure_70.total()
        + reuse_work
        + order.work
        + stale.work
        + px4.work;
    let quiescent = first.naturally_quiescent
        && reacquisition.naturally_quiescent
        && reuse_quiescent
        && order.quiescent
        && stale.quiescent
        && px4.quiescent;
    let order_passed = order.ordered_updates == 1
        && order.ordered_resistance == 4
        && order.shuffled_updates == 0
        && !order.shuffled_live
        && order.late_updates == 0
        && !order.late_live;
    let stale_passed = stale.proposals == 2
        && stale.stale_firings == 0
        && stale.old_dead
        && stale.replacement_distinct
        && stale.repair_firings == 1;
    let px4_passed = px4.first_resistance == 4
        && px4.recurrent_resistance == 7
        && px4.recurrent_proposals == 0
        && px4.old_dead
        && px4.replacement_distinct
        && px4.replacement_resistance == 4;
    let claims = [
        ROOTS.contains(&root) && LOADS.contains(&load),
        blank == 0 && first.work.local_structural_proposals == (load + 1) as u64,
        traversals == load + 1 && first.work.local_return_updates == 1,
        first_returned == 4 && first_plain == 1,
        live_at_30 == 1 && plain_dead == load && old_at_30 == 1,
        live_at_30 == 1,
        old_dead_at_40,
        !world.space.arrow_is_live(returned_arrow),
        replacement_distinct && replacement_after_return == 4,
        live_at_70 == 1,
        reuse_proposals == 0,
        bytes_before_reuse == bytes_after_reuse && bytes_after_reuse <= BYTE_LIMIT,
        work <= WORK_LIMIT,
        order_passed,
        stale_passed,
        px4_passed,
        quiescent,
        true,
    ];
    Row {
        root,
        load,
        returned_last,
        mirror,
        blank,
        proposals: first.work.local_structural_proposals,
        traversals,
        updates: first.work.local_return_updates,
        first_returned,
        first_plain,
        live_at_30,
        plain_dead,
        old_at_30,
        old_dead_at_40,
        replacement_distinct,
        replacement_after_return,
        live_at_70,
        reuse_proposals,
        bytes_before_reuse,
        bytes_after_reuse,
        order,
        stale,
        px4,
        work,
        fingerprint: world.space.complete_fingerprint(),
        quiescent,
        replay: false,
        claims,
        passed: claims.into_iter().all(|claim| claim),
    }
}

fn build_world(root: u64, load: usize, returned_last: bool, mirror: bool) -> World {
    let namespace = root << 32;
    let sign = if mirror { -1 } else { 1 };
    let returned_ordinal = if returned_last { load } else { 0 };
    let mut space = PlasticSubstrate::new();
    let returned = add_returned(&mut space, namespace, returned_ordinal, sign);
    let plain = (0..=load)
        .filter(|ordinal| *ordinal != returned_ordinal)
        .map(|ordinal| add_plain(&mut space, namespace, ordinal, sign))
        .collect();
    World {
        space,
        returned,
        plain,
    }
}

fn add_returned(space: &mut PlasticSubstrate, namespace: u64, ordinal: usize, sign: i32) -> Site {
    let physical = namespace + ordinal as u64 * 16;
    let position = sign * ordinal as i32 * 10;
    let source = space.add_cell(cell(physical, position, 0));
    let effect = space.add_cell(cell(physical + 1, position + sign, 1));
    let returner = space.add_cell(cell(physical + 2, position + 4 * sign, 2));
    space.add_arrow(arrow(effect, returner, TransmissionMode::Drive));
    space.add_arrow(arrow(returner, source, TransmissionMode::Modulatory));
    Site {
        source,
        effect,
        source_physical: physical,
        effect_physical: physical + 1,
    }
}

fn add_plain(space: &mut PlasticSubstrate, namespace: u64, ordinal: usize, sign: i32) -> Site {
    let physical = namespace + ordinal as u64 * 16;
    let position = sign * ordinal as i32 * 10;
    Site {
        source: space.add_cell(cell(physical, position, 0)),
        effect: space.add_cell(cell(physical + 1, position + sign, 1)),
        source_physical: physical,
        effect_physical: physical + 1,
    }
}

fn order_control(mark: u64, mirror: bool) -> OrderControl {
    let mut ordered = ordering_world(mark, mirror);
    arrive(&mut ordered.0, ordered.1, 0, 0, mark + 100);
    let ordered_first = ordered.0.propagate();
    let ordered_arrow = only_between(&ordered.0, ordered.1, ordered.2);
    arrive(&mut ordered.0, ordered.3, 2, 0, mark + 101);
    let ordered_return = ordered.0.propagate();

    let mut shuffled = ordering_world(mark + 10, mirror);
    arrive(&mut shuffled.0, shuffled.3, 0, 0, mark + 102);
    let shuffled_return = shuffled.0.propagate();
    arrive(&mut shuffled.0, shuffled.1, 2, 0, mark + 103);
    let shuffled_proposal = shuffled.0.propagate();
    let shuffled_arrow = only_between(&shuffled.0, shuffled.1, shuffled.2);
    let shuffled_pressure = shuffled.0.advance_time(30);

    let mut late = ordering_world(mark + 20, mirror);
    arrive(&mut late.0, late.1, 0, 0, mark + 104);
    let late_first = late.0.propagate();
    let late_arrow = only_between(&late.0, late.1, late.2);
    arrive(&mut late.0, late.3, 9, 0, mark + 105);
    let late_return = late.0.propagate();
    OrderControl {
        ordered_updates: ordered_return.work.local_return_updates,
        ordered_resistance: ordered.0.arrow_resistance(ordered_arrow),
        shuffled_updates: shuffled_return.work.local_return_updates
            + shuffled_proposal.work.local_return_updates,
        shuffled_live: shuffled.0.arrow_is_live(shuffled_arrow),
        late_updates: late_return.work.local_return_updates,
        late_live: late.0.arrow_is_live(late_arrow),
        work: ordered_first.work.total()
            + ordered_return.work.total()
            + shuffled_return.work.total()
            + shuffled_proposal.work.total()
            + shuffled_pressure.total()
            + late_first.work.total()
            + late_return.work.total(),
        quiescent: ordered_first.naturally_quiescent
            && ordered_return.naturally_quiescent
            && shuffled_return.naturally_quiescent
            && shuffled_proposal.naturally_quiescent
            && late_first.naturally_quiescent
            && late_return.naturally_quiescent,
    }
}

fn ordering_world(mark: u64, mirror: bool) -> (PlasticSubstrate, CellId, CellId, CellId) {
    let sign = if mirror { -1 } else { 1 };
    let mut space = PlasticSubstrate::new();
    let source = space.add_cell(cell(mark, 0, 0));
    let effect = space.add_cell(cell(mark + 1, sign, 1));
    let returner = space.add_cell(cell(mark + 2, 10 * sign, 2));
    space.add_arrow(arrow(returner, source, TransmissionMode::Modulatory));
    (space, source, effect, returner)
}

fn stale_control(mark: u64, mirror: bool) -> StaleControl {
    let sign = if mirror { -1 } else { 1 };
    let mut space = PlasticSubstrate::new();
    let source = space.add_cell(cell(mark, 0, 0));
    let effect = space.add_cell(cell(mark + 1, 2 * sign, 1));
    arrive(&mut space, source, 9, 0, mark + 2);
    let stale = space.propagate();
    let old = only_between(&space, source, effect);
    arrive(&mut space, source, 12, 0, mark + 3);
    let repair = space.propagate();
    let replacement = only_live_between(&space, source, effect);
    StaleControl {
        proposals: stale.work.local_structural_proposals + repair.work.local_structural_proposals,
        stale_firings: fires(&stale.trace, mark + 1),
        old_dead: !space.arrow_is_live(old),
        replacement_distinct: replacement != old,
        repair_firings: fires(&repair.trace, mark + 1),
        work: stale.work.total() + repair.work.total(),
        quiescent: stale.naturally_quiescent && repair.naturally_quiescent,
    }
}

fn px4_control(mark: u64, flip: bool, mirror: bool) -> Px4Control {
    let mut world = field(mark, flip, mirror, TransmissionMode::Modulatory);
    arrive(&mut world.space, world.source, 0, 0, mark + 100);
    arrive(&mut world.space, world.returner, 2, 0, mark + 101);
    let first = world.space.propagate();
    let old = only_between(&world.space, world.source, world.effect);
    let first_resistance = world.space.arrow_resistance(old);
    arrive(&mut world.space, world.source, 5, 0, mark + 102);
    arrive(&mut world.space, world.returner, 7, 0, mark + 103);
    let recurrent = world.space.propagate();
    let recurrent_resistance = world.space.arrow_resistance(old);
    let pressure = world.space.advance_time(80);
    let old_dead = !world.space.arrow_is_live(old);
    arrive(&mut world.space, world.source, 80, 0, mark + 104);
    arrive(&mut world.space, world.returner, 82, 0, mark + 105);
    let reacquisition = world.space.propagate();
    let replacement = only_live_between(&world.space, world.source, world.effect);
    Px4Control {
        first_resistance,
        recurrent_resistance,
        recurrent_proposals: recurrent.work.local_structural_proposals,
        old_dead,
        replacement_distinct: replacement != old,
        replacement_resistance: world.space.arrow_resistance(replacement),
        work: first.work.total()
            + recurrent.work.total()
            + pressure.total()
            + reacquisition.work.total(),
        quiescent: first.naturally_quiescent
            && recurrent.naturally_quiescent
            && reacquisition.naturally_quiescent,
    }
}

fn global_claims(rows: &[Row]) -> [bool; 4] {
    let complete = rows.len() == 24
        && ROOTS
            .iter()
            .all(|root| rows.iter().filter(|row| row.root == *root).count() == 3)
        && LOADS
            .iter()
            .all(|load| rows.iter().filter(|row| row.load == *load).count() == 8);
    let strata = [false, true].into_iter().all(|returned_last| {
        [false, true].into_iter().all(|mirror| {
            rows.iter()
                .filter(|row| row.returned_last == returned_last && row.mirror == mirror)
                .count()
                == 6
        })
    });
    let invariant = LOADS.iter().all(|load| {
        let signatures = rows
            .iter()
            .filter(|row| row.load == *load)
            .map(signature)
            .collect::<BTreeSet<_>>();
        signatures.len() == 1
    });
    let cumulative = rows.iter().all(|row| row.claims[13] && row.claims[14] && row.claims[15]);
    [complete, strata, invariant, cumulative]
}

fn signature(row: &Row) -> Signature {
    Signature {
        proposals: row.proposals,
        traversals: row.traversals,
        updates: row.updates,
        first_returned: row.first_returned,
        first_plain: row.first_plain,
        live_at_30: row.live_at_30,
        plain_dead: row.plain_dead,
        replacement_after_return: row.replacement_after_return,
        live_at_70: row.live_at_70,
        reuse_proposals: row.reuse_proposals,
        bytes_before_reuse: row.bytes_before_reuse,
        bytes_after_reuse: row.bytes_after_reuse,
        order_passed: row.claims[13],
        stale_passed: row.claims[14],
        px4_passed: row.claims[15],
    }
}

fn sites(world: &World) -> Vec<Site> {
    std::iter::once(world.returned)
        .chain(world.plain.iter().copied())
        .collect()
}

fn variable_arrows(world: &World) -> Vec<ArrowId> {
    sites(world)
        .into_iter()
        .flat_map(|site| world.space.arrows_between(site.source, site.effect))
        .collect()
}

fn only_live(space: &PlasticSubstrate, site: Site) -> ArrowId {
    only_live_between(space, site.source, site.effect)
}

fn only_between(space: &PlasticSubstrate, from: CellId, to: CellId) -> ArrowId {
    let arrows = space.arrows_between(from, to);
    assert_eq!(arrows.len(), 1);
    arrows[0]
}

fn only_live_between(space: &PlasticSubstrate, from: CellId, to: CellId) -> ArrowId {
    space
        .arrows_between(from, to)
        .into_iter()
        .find(|arrow| space.arrow_is_live(*arrow))
        .expect("one live physical arrow")
}

fn fires(trace: &[TraceEntry], physical: u64) -> usize {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn cell(physical_id: u64, position: i32, region: i16) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold: 1,
        resistance: 100,
    }
}

fn arrow(from: CellId, to: CellId, mode: TransmissionMode) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance: 100,
        mode,
    }
}

fn frozen_inputs() {
    for (path, expected) in [
        ("crates/lr1-modulatory-physical-return/src/lib.rs", LAW_HASH),
        ("arms/px4-lrc-lifetime/src/lib.rs", PX4_SOURCE_HASH),
        (
            "experiments/px4_lrc_physical_lifetime_authority_handoff_v1.md",
            PX4_HANDOFF_HASH,
        ),
        ("results/px4_lrc_lifetime_authority_v1.csv", PX4_CSV_HASH),
        ("results/px4_lrc_lifetime_authority_v1.md", PX4_REPORT_HASH),
        (
            "experiments/px5_lrc_cumulative_allocation_authority_protocol_v1.md",
            PROTOCOL_HASH,
        ),
    ] {
        assert_eq!(sha(path), expected, "frozen input changed: {path}");
    }
}

fn csv(rows: &[Row]) -> String {
    let mut output = String::from(
        "root,load,returned_last,mirror,blank,proposals,traversals,updates,first_returned,first_plain,live_at_30,plain_dead,old_at_30,old_dead_at_40,replacement_distinct,replacement_after_return,live_at_70,reuse_proposals,bytes_before_reuse,bytes_after_reuse,ordered_updates,shuffled_updates,late_updates,stale_firings,repair_firings,px4_first,px4_recurrent,px4_replacement,work,fingerprint,quiescent,replay,claims,passed\n",
    );
    for row in rows {
        let claims = row
            .claims
            .iter()
            .map(bool::to_string)
            .collect::<Vec<_>>()
            .join("|");
        output.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.root,
            row.load,
            row.returned_last,
            row.mirror,
            row.blank,
            row.proposals,
            row.traversals,
            row.updates,
            row.first_returned,
            row.first_plain,
            row.live_at_30,
            row.plain_dead,
            row.old_at_30,
            row.old_dead_at_40,
            row.replacement_distinct,
            row.replacement_after_return,
            row.live_at_70,
            row.reuse_proposals,
            row.bytes_before_reuse,
            row.bytes_after_reuse,
            row.order.ordered_updates,
            row.order.shuffled_updates,
            row.order.late_updates,
            row.stale.stale_firings,
            row.stale.repair_firings,
            row.px4.first_resistance,
            row.px4.recurrent_resistance,
            row.px4.replacement_resistance,
            row.work,
            row.fingerprint,
            row.quiescent,
            row.replay,
            claims,
            row.passed,
        ));
    }
    output
}

fn report(rows: &[Row], global: [bool; 4]) -> String {
    let passed = rows.iter().filter(|row| row.passed).count();
    let row_clauses = rows
        .iter()
        .map(|row| row.claims.into_iter().filter(|claim| *claim).count())
        .sum::<usize>();
    let global_clauses = global.into_iter().filter(|claim| *claim).count();
    let max_work = rows.iter().map(|row| row.work).max().unwrap_or(0);
    let max_bytes = rows
        .iter()
        .map(|row| row.bytes_after_reuse)
        .max()
        .unwrap_or(0);
    format!(
        "# PX5 LR-C cumulative physical plasticity-allocation authority v1\n\nOutcome: **{}**.\n\n- rows: `{passed}/24`;\n- row clauses: `{row_clauses}/432`;\n- global clauses: `{global_clauses}/4`;\n- total clauses: `{}/436`;\n- exact replay: `{}`;\n- natural quiescence: `{}`;\n- maximum row work: `{max_work}` / `{WORK_LIMIT}`;\n- maximum persistent bytes: `{max_bytes}` / `{BYTE_LIMIT}`;\n- repeated-reuse memory stable: `{}`;\n- live allocation at loads 8/32/128: `1/1/1`;\n- PX4 recurrence/deallocation/reacquisition conformance: `{}`;\n- PX0--PX4+LR-C cumulative conformance: `{}`;\n- new organism source or law: `false`;\n- PX6 executed or advanced: `false`.\n",
        if passed == 24 && row_clauses == 432 && global_clauses == 4 {
            "DEFINITIVE POSITIVE"
        } else {
            "DEFINITIVE NEGATIVE"
        },
        row_clauses + global_clauses,
        rows.iter().all(|row| row.replay),
        rows.iter().all(|row| row.quiescent),
        rows
            .iter()
            .all(|row| row.bytes_before_reuse == row.bytes_after_reuse),
        rows.iter().all(|row| row.claims[15]),
        global[3],
    )
}

fn absent(paths: &[&str]) {
    for path in paths {
        assert!(!Path::new(path).exists(), "result already exists: {path}");
    }
}

fn publish(path: &str, contents: &str) {
    let staging = format!("{path}.staging");
    assert!(!Path::new(&staging).exists(), "staging path exists: {staging}");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staging)
        .expect("create staging artifact");
    file.write_all(contents.as_bytes()).expect("write artifact");
    file.sync_all().expect("sync artifact");
    rename(staging, path).expect("publish artifact");
}

fn sha(path: &str) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .or_else(|_| Command::new("shasum").args(["-a", "256", path]).output())
        .expect("hash command");
    assert!(output.status.success(), "hash failed: {path}");
    String::from_utf8(output.stdout)
        .expect("hash output")
        .split_whitespace()
        .next()
        .expect("hash digest")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_matrix_is_fresh_and_bounded() {
        assert_eq!(ROOTS.len(), 8);
        assert_eq!(ROOTS.into_iter().collect::<BTreeSet<_>>().len(), 8);
        assert_eq!(LOADS, [8, 32, 128]);
        assert_eq!(ROOTS.len() * LOADS.len(), 24);
        assert!(ROOTS.iter().all(|root| *root > 561000));
        assert_eq!(WORK_LIMIT, 100_000);
        assert_eq!(BYTE_LIMIT, 24_000);
    }
}
