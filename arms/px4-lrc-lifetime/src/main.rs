#![forbid(unsafe_code)]

use lr1_modulatory_physical_return::{
    ArrowId, Crossing, Execution, PlasticSubstrate, TraceEntry, TransmissionMode, WorkLedger,
};
use px4_lrc_lifetime::{arrive, field, fork, Field, Fork};
use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::Path;
use std::process::Command;

const AUTHORITY: &str = "f9057fe78a86db9111b0b69310d03accef3bc970";
const LAW_HASH: &str = "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10";
const HANDOFF_HASH: &str = "98067812bc357949af5653a115b353519bede12499804818cfaf4783c0666cbd";
const PROTOCOL_HASH: &str = "dc1bb5efe1a5cfe2f2be0b6c21d1df675213d3334ca784c0844b1b61bc1577dc";
const COUNTS: [usize; 4] = [1, 2, 4, 8];
const RESISTANCES: [u32; 4] = [4, 7, 12, 22];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Flow {
    crossings: Vec<Crossing>,
    trace: Vec<TraceEntry>,
    work: WorkLedger,
    quiescent: bool,
}

impl Flow {
    fn one(execution: Execution) -> Self {
        Self {
            crossings: execution.crossings,
            trace: execution.trace,
            work: execution.work,
            quiescent: execution.naturally_quiescent,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct OneExposure {
    unsupported_dead: bool,
    qualified_resistance: u32,
    qualified_coupling: i32,
    return_alone_empty: bool,
    late_return_rejected: bool,
    drive_return_rejected: bool,
    quiescent: bool,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Curve {
    resistance: [u32; 4],
    deallocation_steps: [u32; 4],
    penultimate_live: [bool; 4],
    final_dead: [bool; 4],
    strict: bool,
    quiescent: bool,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Reuse {
    no_proposal: bool,
    crossing_impulse: i32,
    resistance_before: u32,
    resistance_after: u32,
    disuse_dead: bool,
    reacquired: bool,
    old_generation: u32,
    new_generation: u32,
    quiescent: bool,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Shift {
    old_dead: bool,
    new_live: bool,
    new_resistance: u32,
    new_reused: bool,
    quiescent: bool,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Stale {
    blocked: bool,
    old_generation: u32,
    new_generation: u32,
    effect_firings: usize,
    deallocations: u64,
    quiescent: bool,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Core {
    one: OneExposure,
    curve: Curve,
    reuse: Reuse,
    shift: Shift,
    stale: Stale,
    fresh_layout: bool,
    conformance: [bool; 4],
}

impl Core {
    fn passed(&self) -> bool {
        self.one.unsupported_dead
            && self.one.qualified_resistance == 4
            && self.one.qualified_coupling == 2
            && self.one.return_alone_empty
            && self.one.late_return_rejected
            && self.one.drive_return_rejected
            && self.one.quiescent
            && self.curve.resistance == RESISTANCES
            && self.curve.deallocation_steps == RESISTANCES
            && self.curve.penultimate_live.into_iter().all(|value| value)
            && self.curve.final_dead.into_iter().all(|value| value)
            && self.curve.strict
            && self.curve.quiescent
            && self.reuse.no_proposal
            && self.reuse.crossing_impulse == 2
            && self.reuse.resistance_after == self.reuse.resistance_before + 3
            && self.reuse.disuse_dead
            && self.reuse.reacquired
            && self.reuse.quiescent
            && self.shift.old_dead
            && self.shift.new_live
            && self.shift.new_resistance > 0
            && self.shift.new_reused
            && self.shift.quiescent
            && self.stale.blocked
            && self.stale.effect_firings == 1
            && self.stale.deallocations == 1
            && self.stale.quiescent
            && self.fresh_layout
            && self.conformance.into_iter().all(|value| value)
    }

    fn quiescent(&self) -> bool {
        self.one.quiescent
            && self.curve.quiescent
            && self.reuse.quiescent
            && self.shift.quiescent
            && self.stale.quiescent
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    index: usize,
    mark: u64,
    flip: bool,
    mirror: bool,
    core: Core,
    replay: bool,
    passed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Probe,
    Micro,
    Gate,
}

impl Stage {
    fn name(self) -> &'static str {
        match self {
            Self::Probe => "probe",
            Self::Micro => "micro",
            Self::Gate => "gate",
        }
    }

    fn protocol(self) -> &'static str {
        match self {
            Self::Probe => "px4-lrc-physical-lifetime-probe-v1",
            Self::Micro => "px4-lrc-physical-lifetime-micro-v1",
            Self::Gate => "px4-lrc-physical-lifetime-gate-v1",
        }
    }

    fn marks(self) -> Vec<(u64, bool, bool)> {
        match self {
            Self::Probe => vec![(151_001, false, false)],
            Self::Micro => vec![
                (152_001, false, false),
                (152_002, true, false),
                (152_101, false, true),
                (152_102, true, true),
            ],
            Self::Gate => (0..8)
                .map(|index| (153_001 + index as u64, index % 2 == 1, (index / 2) % 2 == 1))
                .collect(),
        }
    }
}

fn main() {
    let stage = match std::env::args().nth(1).as_deref() {
        Some("--probe") => Stage::Probe,
        Some("--micro") => Stage::Micro,
        Some("--gate") => Stage::Gate,
        _ => panic!("use exactly one of --probe, --micro, or --gate"),
    };
    verify_frozen_inputs();
    let rows = stage
        .marks()
        .into_iter()
        .enumerate()
        .map(|(index, (mark, flip, mirror))| replay_row(index, mark, flip, mirror))
        .collect::<Vec<_>>();
    let all_layouts = rows
        .iter()
        .all(|row| row.core.curve.resistance == rows[0].core.curve.resistance)
        && rows
            .iter()
            .all(|row| row.core.curve.deallocation_steps == rows[0].core.curve.deallocation_steps);
    let passed = all_layouts && rows.iter().all(|row| row.passed);
    let csv = csv(stage, &rows, all_layouts, passed);
    let markdown = markdown(stage, &rows, all_layouts, passed);
    let base = format!("results/px4_lrc_lifetime_{}_v1", stage.name());
    publish_pair(&base, &csv, &markdown);
    println!(
        "PX4 LR-C physical lifetime {}: rows={}/{} replay={} quiescent={} layouts={} verdict={}",
        stage.name().to_uppercase(),
        rows.iter().filter(|row| row.passed).count(),
        rows.len(),
        rows.iter().all(|row| row.replay),
        rows.iter().all(|row| row.core.quiescent()),
        all_layouts,
        if passed { "PASS" } else { "FAIL" }
    );
    if !passed {
        std::process::exit(1);
    }
}

fn replay_row(index: usize, mark: u64, flip: bool, mirror: bool) -> Row {
    let first = run_core(mark, flip, mirror);
    let second = run_core(mark, flip, mirror);
    let replay = first == second;
    let passed = replay && first.passed();
    Row {
        index,
        mark,
        flip,
        mirror,
        core: first,
        replay,
        passed,
    }
}

fn run_core(mark: u64, flip: bool, mirror: bool) -> Core {
    let one = one_exposure(mark + 10_000, flip, mirror);
    let base_curve = curve(mark + 20_000, flip, mirror);
    let reflected = curve(mark + 30_000, !flip, !mirror);
    let fresh_layout = base_curve.resistance == reflected.resistance
        && base_curve.deallocation_steps == reflected.deallocation_steps
        && base_curve.penultimate_live == reflected.penultimate_live
        && base_curve.final_dead == reflected.final_dead;
    let reuse = reuse(mark + 40_000, flip, mirror);
    let shift = shift(mark + 50_000, flip, mirror);
    let stale = stale(mark + 60_000, flip, mirror);
    let forward_only = direction_check(mark + 70_000, flip, mirror);
    let conformance = [
        one.unsupported_dead && reuse.disuse_dead && reuse.reacquired && stale.blocked,
        one.return_alone_empty && one.late_return_rejected,
        forward_only,
        base_curve.strict && reuse.no_proposal && shift.old_dead && shift.new_live,
    ];
    Core {
        one,
        curve: base_curve,
        reuse,
        shift,
        stale,
        fresh_layout,
        conformance,
    }
}

fn one_exposure(mark: u64, flip: bool, mirror: bool) -> OneExposure {
    let mut unsupported = field(mark, flip, mirror, TransmissionMode::Modulatory);
    let unsupported_flow = expose(&mut unsupported, 0, false);
    let unsupported_arrow = only_candidate(&unsupported);
    let unsupported_pressure = unsupported.space.advance_time(5);
    let unsupported_dead = !unsupported.space.arrow_is_live(unsupported_arrow)
        && unsupported_pressure.physical_deallocations == 1;

    let mut qualified = field(mark + 100, flip, mirror, TransmissionMode::Modulatory);
    let qualified_flow = expose(&mut qualified, 0, true);
    let qualified_arrow = only_candidate(&qualified);
    let qualified_resistance = qualified.space.arrow_resistance(qualified_arrow);
    let qualified_coupling = qualified.space.arrow_coupling(qualified_arrow);

    let mut return_alone = field(mark + 200, flip, mirror, TransmissionMode::Modulatory);
    let return_flow = return_only(&mut return_alone, 0);
    let return_alone_empty = return_alone
        .space
        .arrows_between(return_alone.source, return_alone.effect)
        .is_empty()
        && return_flow.work.local_return_updates == 0;

    let mut late = field(mark + 300, flip, mirror, TransmissionMode::Modulatory);
    let late_first = expose(&mut late, 0, false);
    let late_arrow = only_candidate(&late);
    let late_second = return_only(&mut late, 6);
    let late_return_rejected =
        !late.space.arrow_is_live(late_arrow) && late_second.work.local_return_updates == 0;

    let mut drive = field(mark + 400, flip, mirror, TransmissionMode::Drive);
    let drive_flow = expose(&mut drive, 0, true);
    let drive_arrow = only_candidate(&drive);
    let drive_resistance = drive.space.arrow_resistance(drive_arrow);
    let drive_pressure = drive.space.advance_time(8);
    let drive_return_rejected = drive_resistance == 1
        && drive_flow.work.local_return_updates == 0
        && !drive.space.arrow_is_live(drive_arrow)
        && drive_pressure.physical_deallocations == 1;

    OneExposure {
        unsupported_dead,
        qualified_resistance,
        qualified_coupling,
        return_alone_empty,
        late_return_rejected,
        drive_return_rejected,
        quiescent: unsupported_flow.quiescent
            && qualified_flow.quiescent
            && return_flow.quiescent
            && late_first.quiescent
            && late_second.quiescent
            && drive_flow.quiescent,
        fingerprint: qualified.space.complete_fingerprint(),
    }
}

fn curve(mark: u64, flip: bool, mirror: bool) -> Curve {
    let mut resistance = [0; 4];
    let mut deallocation_steps = [0; 4];
    let mut penultimate_live = [false; 4];
    let mut final_dead = [false; 4];
    let mut quiescent = true;
    let mut fingerprint = 0;
    for (index, count) in COUNTS.into_iter().enumerate() {
        let (world, candidate, flows, pressure_tick) =
            trained(mark + index as u64 * 100, flip, mirror, count);
        quiescent &= flows.iter().all(|flow| flow.quiescent);
        resistance[index] = world.space.arrow_resistance(candidate);
        let mut walker = world.clone();
        while walker.space.arrow_is_live(candidate) && deallocation_steps[index] < 64 {
            deallocation_steps[index] += 1;
            walker.space.advance_time(
                pressure_tick + i64::from(deallocation_steps[index]).saturating_mul(10),
            );
        }
        let mut before = world.clone();
        before.space.advance_time(
            pressure_tick + i64::from(resistance[index].saturating_sub(1)).saturating_mul(10),
        );
        penultimate_live[index] = before.space.arrow_is_live(candidate);
        let mut after = world.clone();
        after
            .space
            .advance_time(pressure_tick + i64::from(resistance[index]).saturating_mul(10));
        final_dead[index] = !after.space.arrow_is_live(candidate);
        fingerprint ^= after
            .space
            .complete_fingerprint()
            .rotate_left(index as u32 * 7);
    }
    let strict = resistance.windows(2).all(|pair| pair[0] < pair[1])
        && deallocation_steps.windows(2).all(|pair| pair[0] < pair[1]);
    Curve {
        resistance,
        deallocation_steps,
        penultimate_live,
        final_dead,
        strict,
        quiescent,
        fingerprint,
    }
}

fn reuse(mark: u64, flip: bool, mirror: bool) -> Reuse {
    let (mut world, candidate, flows, _) = trained(mark, flip, mirror, 3);
    world.space.advance_time(50);
    let resistance_before = world.space.arrow_resistance(candidate);
    let flow = expose(&mut world, 51, true);
    let resistance_after = world.space.arrow_resistance(candidate);
    let crossing_impulse = crossing_impulse(
        &flow.crossings,
        world.source_physical,
        world.effect_physical,
    );
    let no_proposal = flow.work.local_structural_proposals == 0;

    let mut spent = field(mark + 100, flip, mirror, TransmissionMode::Modulatory);
    let first = expose(&mut spent, 0, true);
    let old = only_candidate(&spent);
    spent.space.advance_time(40);
    let disuse_dead = !spent.space.arrow_is_live(old);
    let old_generation = spent.space.arrow_generation(old);
    let reacquire_flow = expose(&mut spent, 41, true);
    let arrows = spent.space.arrows_between(spent.source, spent.effect);
    let new = *arrows.last().unwrap();
    let new_generation = spent.space.arrow_generation(new);
    let reacquired = arrows.len() == 2
        && new != old
        && !spent.space.arrow_is_live(old)
        && spent.space.arrow_is_live(new)
        && spent.space.arrow_resistance(new) == 4
        && fires(&reacquire_flow.trace, spent.effect_physical) == 1;
    Reuse {
        no_proposal,
        crossing_impulse,
        resistance_before,
        resistance_after,
        disuse_dead,
        reacquired,
        old_generation,
        new_generation,
        quiescent: flows.iter().all(|item| item.quiescent)
            && flow.quiescent
            && first.quiescent
            && reacquire_flow.quiescent,
        fingerprint: world.space.complete_fingerprint() ^ spent.space.complete_fingerprint(),
    }
}

fn shift(mark: u64, flip: bool, mirror: bool) -> Shift {
    let mut world = fork(mark, flip, mirror);
    let mut quiescent = true;
    for start in [0, 5, 10] {
        quiescent &= expose_fork(&mut world, 0, start).quiescent;
    }
    let old = only_fork_candidate(&world, 0);
    for start in [30, 40, 50, 60] {
        quiescent &= expose_fork(&mut world, 1, start).quiescent;
    }
    let new = only_fork_candidate(&world, 1);
    world.space.advance_time(100);
    let old_dead = !world.space.arrow_is_live(old);
    let new_live = world.space.arrow_is_live(new);
    let new_resistance = world.space.arrow_resistance(new);
    let reuse = expose_fork(&mut world, 1, 101);
    quiescent &= reuse.quiescent;
    let new_reused = reuse.work.local_structural_proposals == 0
        && crossing_impulse(
            &reuse.crossings,
            world.source_physical[1],
            world.effect_physical[1],
        ) == 2;
    Shift {
        old_dead,
        new_live,
        new_resistance,
        new_reused,
        quiescent,
        fingerprint: world.space.complete_fingerprint(),
    }
}

fn stale(mark: u64, flip: bool, mirror: bool) -> Stale {
    let mut world = field(mark, flip, mirror, TransmissionMode::Modulatory);
    arrive(&mut world.space, world.source, 9, 1, world.mark + 1_000);
    arrive(&mut world.space, world.source, 10, 2, world.mark + 2_000);
    let flow = Flow::one(world.space.propagate());
    let arrows = world.space.arrows_between(world.source, world.effect);
    let old = arrows[0];
    let new = arrows[1];
    let old_generation = world.space.arrow_generation(old);
    let new_generation = world.space.arrow_generation(new);
    let effect_firings = fires(&flow.trace, world.effect_physical);
    let blocked = arrows.len() == 2
        && !world.space.arrow_is_live(old)
        && world.space.arrow_is_live(new)
        && flow.work.local_structural_proposals == 2
        && effect_firings == 1;
    Stale {
        blocked,
        old_generation,
        new_generation,
        effect_firings,
        deallocations: flow.work.physical_deallocations,
        quiescent: flow.quiescent,
        fingerprint: world.space.complete_fingerprint(),
    }
}

fn direction_check(mark: u64, flip: bool, mirror: bool) -> bool {
    let mut world = field(mark, flip, mirror, TransmissionMode::Modulatory);
    let flow = expose(&mut world, 0, true);
    world
        .space
        .arrows_between(world.effect, world.source)
        .is_empty()
        && world.space.arrows_between(world.source, world.effect).len() == 1
        && crossing_impulse(
            &flow.crossings,
            world.source_physical,
            world.effect_physical,
        ) == 1
}

fn trained(mark: u64, flip: bool, mirror: bool, count: usize) -> (Field, ArrowId, Vec<Flow>, i64) {
    let mut world = field(mark, flip, mirror, TransmissionMode::Modulatory);
    let mut flows = Vec::new();
    for index in 0..count {
        flows.push(expose(&mut world, index as i64 * 5, true));
    }
    let candidate = only_candidate(&world);
    let last_tick = (count as i64 - 1) * 5 + 3;
    let pressure_tick = last_tick / 10 * 10;
    (world, candidate, flows, pressure_tick)
}

fn expose(world: &mut Field, start: i64, supported: bool) -> Flow {
    arrive(
        &mut world.space,
        world.source,
        start,
        1,
        world.mark + 1_000 + start as u64,
    );
    if supported {
        arrive(
            &mut world.space,
            world.returner,
            start + 2,
            2,
            world.mark + 2_000 + start as u64,
        );
    }
    Flow::one(world.space.propagate())
}

fn return_only(world: &mut Field, start: i64) -> Flow {
    arrive(
        &mut world.space,
        world.returner,
        start,
        2,
        world.mark + 3_000 + start as u64,
    );
    Flow::one(world.space.propagate())
}

fn expose_fork(world: &mut Fork, path: usize, start: i64) -> Flow {
    arrive(
        &mut world.space,
        world.sources[path],
        start,
        10 + path as i32,
        world.mark + 1_000 + path as u64 * 100 + start as u64,
    );
    arrive(
        &mut world.space,
        world.returner,
        start + 2,
        20 + path as i32,
        world.mark + 2_000 + path as u64 * 100 + start as u64,
    );
    Flow::one(world.space.propagate())
}

fn only_candidate(world: &Field) -> ArrowId {
    let arrows = world.space.arrows_between(world.source, world.effect);
    assert_eq!(arrows.len(), 1);
    arrows[0]
}

fn only_fork_candidate(world: &Fork, path: usize) -> ArrowId {
    let arrows = world
        .space
        .arrows_between(world.sources[path], world.effects[path]);
    assert_eq!(arrows.len(), 1);
    arrows[0]
}

fn fires(trace: &[TraceEntry], physical: u64) -> usize {
    trace
        .iter()
        .filter(|entry| entry.target_physical == physical && entry.fired)
        .count()
}

fn crossing_impulse(crossings: &[Crossing], from: u64, to: u64) -> i32 {
    crossings
        .iter()
        .filter(|crossing| crossing.from_physical == from && crossing.to_physical == to)
        .map(|crossing| crossing.impulse)
        .sum()
}

fn csv(stage: Stage, rows: &[Row], all_layouts: bool, passed: bool) -> String {
    let mut output = String::from(
        "stage,protocol,authority,index,mark,flip,mirror,resistance,deallocation_steps,one_dead,qualified_resistance,return_alone,late_return,drive_return,reuse_no_proposal,reuse_impulse,disuse_dead,reacquired,old_generation,new_generation,shift_old_dead,shift_new_live,shift_new_resistance,shift_new_reused,stale_blocked,stale_generations,stale_effect_firings,stale_deallocations,fresh_layout,px0,px1,px2,px3,quiescent,replay,row_passed,all_layouts,verdict\n",
    );
    for row in rows {
        writeln!(
            output,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}|{},{},{},{},{},{},{},{},{},{},{},{},{}",
            stage.name(),
            stage.protocol(),
            AUTHORITY,
            row.index,
            row.mark,
            row.flip,
            row.mirror,
            join_u32(&row.core.curve.resistance),
            join_u32(&row.core.curve.deallocation_steps),
            row.core.one.unsupported_dead,
            row.core.one.qualified_resistance,
            row.core.one.return_alone_empty,
            row.core.one.late_return_rejected,
            row.core.one.drive_return_rejected,
            row.core.reuse.no_proposal,
            row.core.reuse.crossing_impulse,
            row.core.reuse.disuse_dead,
            row.core.reuse.reacquired,
            row.core.reuse.old_generation,
            row.core.reuse.new_generation,
            row.core.shift.old_dead,
            row.core.shift.new_live,
            row.core.shift.new_resistance,
            row.core.shift.new_reused,
            row.core.stale.blocked,
            row.core.stale.old_generation,
            row.core.stale.new_generation,
            row.core.stale.effect_firings,
            row.core.stale.deallocations,
            row.core.fresh_layout,
            row.core.conformance[0],
            row.core.conformance[1],
            row.core.conformance[2],
            row.core.conformance[3],
            row.core.quiescent(),
            row.replay,
            row.passed,
            all_layouts,
            if passed { "PASS" } else { "FAIL" },
        )
        .unwrap();
    }
    output
}

fn markdown(stage: Stage, rows: &[Row], all_layouts: bool, passed: bool) -> String {
    let clauses = rows.iter().filter(|row| row.passed).count();
    let mut output = format!(
        "# PX4 LR-C physical lifetime {} v1\n\nStatus: **DEVELOPMENT {}**; authority absent.\n\nProtocol: `{}`.\n\nAuthority ancestor: `{}`.\n\n- rows: `{}/{}`;\n- resistance sequence: `{}`;\n- deallocation-pressure sequence: `{}`;\n- exact replay: `{}`;\n- natural quiescence: `{}`;\n- fresh identity/layout invariance: `{}`;\n- PX0--PX3 conformance: `{}`.\n\n| row | identity | flip | mirror | one exposure | recurrence/pressure | reuse/reacquisition | changed experience | stale generation | controls | replay | result |\n|---:|---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|\n",
        stage.name().to_uppercase(),
        if passed { "POSITIVE" } else { "NEGATIVE" },
        stage.protocol(),
        AUTHORITY,
        clauses,
        rows.len(),
        join_u32(&rows[0].core.curve.resistance),
        join_u32(&rows[0].core.curve.deallocation_steps),
        rows.iter().all(|row| row.replay),
        rows.iter().all(|row| row.core.quiescent()),
        all_layouts && rows.iter().all(|row| row.core.fresh_layout),
        rows.iter().all(|row| row.core.conformance.into_iter().all(|value| value)),
    );
    for row in rows {
        let controls = row.core.one.return_alone_empty
            && row.core.one.late_return_rejected
            && row.core.one.drive_return_rejected;
        writeln!(
            output,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            row.index,
            row.mark,
            row.flip,
            row.mirror,
            row.core.one.unsupported_dead && row.core.one.qualified_resistance == 4,
            row.core.curve.strict,
            row.core.reuse.no_proposal && row.core.reuse.reacquired,
            row.core.shift.old_dead && row.core.shift.new_live,
            row.core.stale.blocked,
            controls,
            row.replay,
            if row.passed { "PASS" } else { "FAIL" },
        )
        .unwrap();
    }
    output.push_str(
        "\nThe measured quantity is ordinary ARROW resistance under ordinary pressure. No organism-visible lifetime representation, episode boundary, cleanup call or delete operation was added. This artifact is development evidence only and does not advance authority.\n",
    );
    output
}

fn join_u32(values: &[u32]) -> String {
    values
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn publish_pair(base: &str, csv: &str, markdown: &str) {
    let csv_path = format!("{base}.csv");
    let markdown_path = format!("{base}.md");
    assert!(!Path::new(&csv_path).exists(), "CSV already exists");
    assert!(
        !Path::new(&markdown_path).exists(),
        "Markdown already exists"
    );
    let csv_stage = format!("{base}.csv.staging");
    let markdown_stage = format!("{base}.md.staging");
    write_new(&csv_stage, csv);
    write_new(&markdown_stage, markdown);
    fs::rename(csv_stage, csv_path).unwrap();
    fs::rename(markdown_stage, markdown_path).unwrap();
}

fn write_new(path: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.sync_all().unwrap();
}

fn verify_frozen_inputs() {
    require_hash("crates/lr1-modulatory-physical-return/src/lib.rs", LAW_HASH);
    require_hash(
        "experiments/px3_lrc_physical_event_organization_authority_handoff_v2.md",
        HANDOFF_HASH,
    );
    require_hash(
        "experiments/px4_lrc_physical_lifetime_development_protocol_v1.md",
        PROTOCOL_HASH,
    );
}

fn require_hash(path: &str, required: &str) {
    let output = Command::new("sha256sum").arg(path).output().unwrap();
    assert!(output.status.success(), "sha256sum failed for {path}");
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        actual.split_whitespace().next().unwrap(),
        required,
        "{path}"
    );
}

#[allow(dead_code)]
fn _active_type_check(_: &PlasticSubstrate) {}
