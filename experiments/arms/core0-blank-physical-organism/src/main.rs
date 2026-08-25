#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use academy_arc3::{spatial_context, Arc3Sensorimotor, ARC3_FRAME_PIXELS};
use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, CellId, CellSpec, Core0Profile, MechanicalConfig, PhysicalEvent,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, TransmissionTrigger,
};

const HIGH_RESISTANCE: u32 = 10_000;
const CEILING: u64 = 20_000;
const CORE1_PROFILES: [Core0Profile; 5] = [
    Core0Profile::B,
    Core0Profile::GenericExternal,
    Core0Profile::GenericActivity,
    Core0Profile::GenericDistance,
    Core0Profile::GenericDistanceNoQlp,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Gate {
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    E13,
    E14,
}

impl Gate {
    const ALL: [Self; 15] = [
        Self::E0,
        Self::E1,
        Self::E2,
        Self::E3,
        Self::E4,
        Self::E5,
        Self::E6,
        Self::E7,
        Self::E8,
        Self::E9,
        Self::E10,
        Self::E11,
        Self::E12,
        Self::E13,
        Self::E14,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::E0 => "propagate_activity",
            Self::E1 => "form_local_relation",
            Self::E2 => "retain_supported_relation",
            Self::E3 => "remove_unsupported_structure",
            Self::E4 => "positive_efficacy",
            Self::E5 => "negative_efficacy",
            Self::E6 => "competing_possibility_selection",
            Self::E7 => "build_contact_compartment",
            Self::E8 => "spatial_credit_specificity",
            Self::E9 => "delayed_consequence",
            Self::E10 => "multi_hop_closure",
            Self::E11 => "stable_recurrence",
            Self::E12 => "learned_recurrence_stabilization",
            Self::E13 => "four_context_relations",
            Self::E14 => "frozen_arc_a2",
        }
    }

    fn experiences(self) -> u32 {
        match self {
            Self::E0 | Self::E3 | Self::E11 => 1,
            Self::E1 | Self::E7 => 3,
            Self::E2 | Self::E4 | Self::E5 | Self::E8 | Self::E9 => 2,
            Self::E6 => 3,
            Self::E10 => 10,
            Self::E12 => 6,
            Self::E13 | Self::E14 => 5,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MaterialArrow {
    id: ArrowId,
    live: bool,
    coupling: i64,
    resistance: u64,
    participation: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    pass: bool,
    reason: String,
    trace: Vec<PhysicalTransition>,
    arrows: Vec<MaterialArrow>,
    tick: i64,
    work: u64,
    proposals: u64,
    deallocations: u64,
    qlp: u64,
    live_cells: usize,
    live_arrows: usize,
    quiescent: bool,
}

#[derive(Clone, Debug)]
struct Row {
    profile: Core0Profile,
    gate: Gate,
    status: &'static str,
    experiences: u32,
    work: u64,
    live_cells: usize,
    live_arrows: usize,
    proposals: u64,
    deallocations: u64,
    qlp: u64,
    replay_exact: bool,
    mechanics_exact: bool,
    reason: String,
}

fn profile_name(profile: Core0Profile) -> &'static str {
    match profile {
        Core0Profile::B => "CORE1-A",
        Core0Profile::GenericExternal => "CORE1-B",
        Core0Profile::GenericActivity => "CORE1-C",
        Core0Profile::GenericDistance => "CORE1-D",
        Core0Profile::GenericDistanceNoQlp => "CORE1-E",
        Core0Profile::A | Core0Profile::C | Core0Profile::D => "UNUSED",
    }
}

fn body(root: u64, profile: Core0Profile, mechanics: MechanicalConfig) -> PlasticSubstrate {
    let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 512, 2048, mechanics);
    body.set_core0_profile(profile);
    body.set_physical_tracing(true);
    body
}

fn cell(body: &mut PlasticSubstrate, physical: u64, position: i32, threshold: i32) -> CellId {
    body.add_cell(CellSpec {
        physical_id: physical,
        position,
        region: 0,
        threshold,
        resistance: HIGH_RESISTANCE,
    })
}

fn arrow(
    body: &mut PlasticSubstrate,
    from: CellId,
    to: CellId,
    coupling: i32,
    delay: i64,
    mode: TransmissionMode,
    resistance: u32,
) -> ArrowId {
    body.add_arrow(ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode,
    })
}

fn anchor(body: &mut PlasticSubstrate, root: u64, cells: &[CellId]) {
    let anchor = cell(body, root + 90_000, 90_000, i32::MAX);
    for target in cells {
        arrow(
            body,
            anchor,
            *target,
            1,
            1,
            TransmissionMode::Drive,
            HIGH_RESISTANCE,
        );
    }
}

fn pulse(
    body: &mut PlasticSubstrate,
    target: CellId,
    tick: i64,
    impulse: i32,
    origin: u64,
) -> truelearner_core::RunResult {
    body.arrive(
        &[SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: origin,
            target,
            impulse,
        }],
        i16::MAX,
    )
}

fn pulse_many(
    body: &mut PlasticSubstrate,
    targets: &[CellId],
    tick: i64,
    impulse: i32,
    origin: u64,
) -> truelearner_core::RunResult {
    let inputs = targets
        .iter()
        .enumerate()
        .map(|(index, target)| SpikeInput {
            arrival_tick: tick,
            phase: 0,
            origin_physical: origin.saturating_add(u64::try_from(index).unwrap_or(u64::MAX)),
            target: *target,
            impulse,
        })
        .collect::<Vec<_>>();
    body.arrive(&inputs, i16::MAX)
}

fn modulate(
    body: &mut PlasticSubstrate,
    root: u64,
    target: CellId,
    tick: i64,
) -> truelearner_core::RunResult {
    let source = cell(
        body,
        root + 80_000 + u64::try_from(tick).unwrap_or(0),
        80_000,
        1,
    );
    arrow(
        body,
        source,
        target,
        1,
        0,
        TransmissionMode::Modulatory,
        HIGH_RESISTANCE,
    );
    pulse(
        body,
        source,
        tick,
        1,
        root + 70_000 + u64::try_from(tick).unwrap_or(0),
    )
}

fn fires(trace: &[PhysicalTransition], cell: CellId) -> usize {
    trace
        .iter()
        .filter(|entry| matches!(entry.event, PhysicalEvent::Fire { cell: id } if id == cell))
        .count()
}

fn material_snapshot(body: &PlasticSubstrate) -> Vec<MaterialArrow> {
    let mut arrows = body
        .arena_body(1)
        .arrows
        .into_iter()
        .map(|arrow| MaterialArrow {
            id: arrow.id,
            live: arrow.live,
            coupling: body.core0_coupling_material(arrow.id),
            resistance: body.core0_resistance_material(arrow.id),
            participation: body.local_participation(arrow.id),
        })
        .collect::<Vec<_>>();
    arrows.sort_by_key(|arrow| arrow.id.0);
    arrows
}

#[allow(clippy::too_many_arguments)]
fn finish(
    body: &PlasticSubstrate,
    pass: bool,
    reason: impl Into<String>,
    trace: Vec<PhysicalTransition>,
    work: u64,
    proposals: u64,
    deallocations: u64,
    qlp: u64,
    quiescent: bool,
) -> Observation {
    let durable = body.arena_body(1);
    Observation {
        pass,
        reason: reason.into(),
        trace,
        arrows: material_snapshot(body),
        tick: body.clock().tick,
        work,
        proposals,
        deallocations,
        qlp,
        live_cells: durable.cells.iter().filter(|cell| cell.live).count(),
        live_arrows: durable.arrows.iter().filter(|arrow| arrow.live).count(),
        quiescent,
    }
}

fn relation(
    body: &PlasticSubstrate,
    source: CellId,
    target: CellId,
) -> (bool, Vec<ArrowId>, Vec<CellId>) {
    let durable = body.arena_body(1);
    let mut links = durable
        .arrows
        .iter()
        .filter(|arrow| arrow.live && arrow.from.id == source && arrow.to.id == target)
        .map(|arrow| arrow.id)
        .collect::<Vec<_>>();
    let mut contacts = Vec::new();
    for candidate in durable.cells.iter().filter(|cell| cell.live) {
        let stem = durable
            .arrows
            .iter()
            .find(|arrow| arrow.live && arrow.from.id == source && arrow.to.id == candidate.id);
        let outgoing = durable
            .arrows
            .iter()
            .find(|arrow| arrow.live && arrow.from.id == candidate.id && arrow.to.id == target);
        if let (Some(stem), Some(outgoing)) = (stem, outgoing) {
            contacts.push(candidate.id);
            links.push(stem.id);
            links.push(outgoing.id);
        }
    }
    (!links.is_empty(), links, contacts)
}

fn sum_runs(
    runs: &[&truelearner_core::RunResult],
) -> (Vec<PhysicalTransition>, u64, u64, u64, u64, bool) {
    let mut trace = Vec::new();
    let mut work = 0_u64;
    let mut proposals = 0_u64;
    let mut deallocations = 0_u64;
    let mut qlp = 0_u64;
    let mut quiescent = true;
    for run in runs {
        trace.extend(run.physical_trace.clone());
        work = work.saturating_add(run.work.physical_total());
        proposals = proposals.saturating_add(run.work.local_structural_proposals);
        deallocations = deallocations.saturating_add(run.work.physical_deallocations);
        qlp = qlp.saturating_add(run.work.qualified_local_traversals);
        quiescent &= run.naturally_quiescent;
    }
    (trace, work, proposals, deallocations, qlp, quiescent)
}

fn gate_e0(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    let mut body = body(root, profile, mechanics);
    let a = cell(&mut body, root + 1, 0, 1);
    let b = cell(&mut body, root + 2, 10, 1);
    anchor(&mut body, root, &[a, b]);
    arrow(&mut body, a, b, 1, 0, TransmissionMode::Drive, 100);
    let run = pulse(&mut body, a, 0, 1, root + 1_000);
    let pass = fires(&run.physical_trace, b) == 1 && run.naturally_quiescent;
    let (trace, work, proposals, deallocations, qlp, quiescent) = sum_runs(&[&run]);
    finish(
        &body,
        pass,
        "target fires exactly once",
        trace,
        work,
        proposals,
        deallocations,
        qlp,
        quiescent,
    )
}

fn gate_e1(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    let mut body = body(root, profile, mechanics);
    let source = cell(&mut body, root + 1, 0, 1);
    let target = cell(&mut body, root + 2, 1, 100);
    anchor(&mut body, root, &[source, target]);
    let run = pulse(&mut body, source, 0, 1, root + 1_000);
    let second_tick = body.clock().tick.saturating_add(1);
    let second = pulse(&mut body, source, second_tick, 1, root + 2_000);
    let third_tick = body.clock().tick.saturating_add(1);
    let third = pulse(&mut body, source, third_tick, 1, root + 3_000);
    let (exists, _, _) = relation(&body, source, target);
    let proposals = run
        .work
        .local_structural_proposals
        .saturating_add(second.work.local_structural_proposals)
        .saturating_add(third.work.local_structural_proposals);
    let pass = exists
        && proposals > 0
        && run.naturally_quiescent
        && second.naturally_quiescent
        && third.naturally_quiescent;
    let (trace, work, proposals, deallocations, qlp, quiescent) =
        sum_runs(&[&run, &second, &third]);
    finish(
        &body,
        pass,
        "new source-target relation exists",
        trace,
        work,
        proposals,
        deallocations,
        qlp,
        quiescent,
    )
}

fn generated_world(
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
) -> (
    PlasticSubstrate,
    CellId,
    CellId,
    truelearner_core::RunResult,
) {
    let mut body = body(root, profile, mechanics);
    let source = cell(&mut body, root + 1, 0, 1);
    let target = cell(&mut body, root + 2, 1, 100);
    anchor(&mut body, root, &[source, target]);
    let run = pulse(&mut body, source, 0, 1, root + 1_000);
    (body, source, target, run)
}

fn selected_contact(
    body: &PlasticSubstrate,
    source: CellId,
    target: CellId,
    positive: bool,
) -> Option<(CellId, Vec<ArrowId>)> {
    let durable = body.arena_body(1);
    for candidate in &durable.cells {
        let Some(stem) = durable
            .arrows
            .iter()
            .find(|arrow| arrow.live && arrow.from.id == source && arrow.to.id == candidate.id)
        else {
            continue;
        };
        let Some(outgoing) = durable
            .arrows
            .iter()
            .find(|arrow| arrow.live && arrow.from.id == candidate.id && arrow.to.id == target)
        else {
            continue;
        };
        if (outgoing.coupling > 0) == positive {
            return Some((candidate.id, vec![stem.id, outgoing.id]));
        }
    }
    None
}

fn direct_signed(
    body: &PlasticSubstrate,
    source: CellId,
    target: CellId,
    positive: bool,
) -> Option<ArrowId> {
    body.arena_body(1)
        .arrows
        .into_iter()
        .find(|arrow| {
            arrow.live
                && arrow.from.id == source
                && arrow.to.id == target
                && (arrow.coupling > 0) == positive
        })
        .map(|arrow| arrow.id)
}

fn gate_e2(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    let (mut body, source, target, first) = generated_world(profile, mechanics, root);
    let selection = selected_contact(&body, source, target, true)
        .or_else(|| direct_signed(&body, source, target, true).map(|id| (source, vec![id])));
    let Some((local, links)) = selection else {
        let (trace, work, proposals, deallocations, qlp, quiescent) = sum_runs(&[&first]);
        return finish(
            &body,
            false,
            "no positive candidate",
            trace,
            work,
            proposals,
            deallocations,
            qlp,
            quiescent,
        );
    };
    let before = links
        .iter()
        .map(|id| body.core0_resistance_material(*id))
        .collect::<Vec<_>>();
    let second_tick = body.clock().tick;
    let second = pulse(&mut body, source, second_tick, 1, root + 1_001);
    let consequence_tick = body.clock().tick;
    let third = modulate(&mut body, root, local, consequence_tick);
    let supported = links
        .iter()
        .zip(before)
        .all(|(id, before)| body.core0_resistance_material(*id) > before);
    let pass = supported
        && first.naturally_quiescent
        && second.naturally_quiescent
        && third.naturally_quiescent;
    let (trace, work, proposals, deallocations, qlp, quiescent) =
        sum_runs(&[&first, &second, &third]);
    finish(
        &body,
        pass,
        "qualified consequence increases candidate durability",
        trace,
        work,
        proposals,
        deallocations,
        qlp,
        quiescent,
    )
}

fn gate_e3(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    let (mut body, source, target, first) = generated_world(profile, mechanics, root);
    let (_, links, contacts) = relation(&body, source, target);
    let decay = body.advance_time_traced(25);
    let durable = body.arena_body(1);
    let links_dead = links.iter().all(|id| {
        durable
            .arrows
            .iter()
            .find(|arrow| arrow.id == *id)
            .is_some_and(|arrow| !arrow.live)
    });
    let contacts_dead = contacts.iter().all(|id| {
        durable
            .cells
            .iter()
            .find(|cell| cell.id == *id)
            .is_some_and(|cell| !cell.live)
    });
    let pass = links_dead && contacts_dead && decay.naturally_quiescent;
    let (trace, work, proposals, deallocations, qlp, quiescent) = sum_runs(&[&first, &decay]);
    finish(
        &body,
        pass,
        "unsupported relation and orphan contacts disappear",
        trace,
        work,
        proposals,
        deallocations,
        qlp,
        quiescent,
    )
}

fn efficacy_world(
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
    coupling: i32,
    delayed: i64,
) -> (Observation, i64, u64) {
    let mut body = body(root, profile, mechanics);
    let source = cell(&mut body, root + 1, 0, 1);
    let contact = cell(&mut body, root + 2, 10, 1);
    let target = cell(&mut body, root + 3, 20, 100);
    anchor(&mut body, root, &[source, contact, target]);
    arrow(
        &mut body,
        source,
        contact,
        1,
        0,
        TransmissionMode::Drive,
        100,
    );
    let learned = arrow(
        &mut body,
        contact,
        target,
        coupling,
        0,
        TransmissionMode::Drive,
        100,
    );
    let before_c = body.core0_coupling_material(learned);
    let before_r = body.core0_resistance_material(learned);
    let first = pulse(&mut body, source, 0, 1, root + 1_000);
    if delayed > 0 {
        body.advance_time(delayed);
    }
    let second = modulate(&mut body, root, contact, delayed.max(1));
    let after_c = body.core0_coupling_material(learned);
    let after_r = body.core0_resistance_material(learned);
    let sign_ok = if coupling > 0 {
        after_c > before_c
    } else {
        after_c < before_c
    };
    let pass =
        sign_ok && after_r > before_r && first.naturally_quiescent && second.naturally_quiescent;
    let (trace, work, proposals, deallocations, qlp, quiescent) = sum_runs(&[&first, &second]);
    (
        finish(
            &body,
            pass,
            "supported efficacy and durability mature",
            trace,
            work,
            proposals,
            deallocations,
            qlp,
            quiescent,
        ),
        after_c,
        after_r,
    )
}

fn gate_e4(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    efficacy_world(profile, mechanics, root, 1, 0).0
}

fn gate_e5(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    efficacy_world(profile, mechanics, root, -1, 0).0
}

fn competing(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    let mut body = body(root, profile, mechanics);
    let source = cell(&mut body, root + 1, 0, 1);
    let cp = cell(&mut body, root + 2, 10, 1);
    let cn = cell(&mut body, root + 3, 20, 1);
    let xp = cell(&mut body, root + 4, 30, 100);
    let xn = cell(&mut body, root + 5, 40, 100);
    anchor(&mut body, root, &[source, cp, cn, xp, xn]);
    arrow(&mut body, source, cp, 1, 0, TransmissionMode::Drive, 100);
    arrow(&mut body, source, cn, 1, 0, TransmissionMode::Drive, 100);
    let ap = arrow(&mut body, cp, xp, 1, 0, TransmissionMode::Drive, 100);
    let an = arrow(&mut body, cn, xn, -1, 0, TransmissionMode::Drive, 100);
    let before_p = body.core0_coupling_material(ap);
    let before_n = body.core0_coupling_material(an);
    let first = pulse(&mut body, source, 0, 1, root + 1_000);
    let second = modulate(&mut body, root, cp, 1);
    let pass =
        body.core0_coupling_material(ap) > before_p && body.core0_coupling_material(an) == before_n;
    let (trace, work, proposals, deallocations, qlp, quiescent) = sum_runs(&[&first, &second]);
    finish(
        &body,
        pass,
        "only the locally supported compartment matures",
        trace,
        work,
        proposals,
        deallocations,
        qlp,
        quiescent,
    )
}

fn gate_e7(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    let (mut body, source, target, run) = generated_world(profile, mechanics, root);
    let second_tick = body.clock().tick.saturating_add(1);
    let second = pulse(&mut body, source, second_tick, 1, root + 2_000);
    let third_tick = body.clock().tick.saturating_add(1);
    let third = pulse(&mut body, source, third_tick, 1, root + 3_000);
    let (_, _, contacts) = relation(&body, source, target);
    let cell_proposals = run
        .work
        .local_cell_proposals
        .saturating_add(second.work.local_cell_proposals)
        .saturating_add(third.work.local_cell_proposals);
    let pass = contacts.len() >= 2 && cell_proposals >= 2;
    let (trace, work, proposals, deallocations, qlp, quiescent) =
        sum_runs(&[&run, &second, &third]);
    finish(
        &body,
        pass,
        "variation creates separate ordinary contact compartments",
        trace,
        work,
        proposals,
        deallocations,
        qlp,
        quiescent,
    )
}

fn gate_e9(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    efficacy_world(profile, mechanics, root, 1, 5).0
}

fn gate_e10(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    let mut all_trace = Vec::new();
    let mut total_work = 0_u64;
    let mut total_qlp = 0_u64;
    let mut representative = body(root, profile, mechanics);
    let mut pass = true;
    for (case, depth) in [1_usize, 2, 4, 8, 16].into_iter().enumerate() {
        let mut local = body(
            root + u64::try_from(case).unwrap() * 1_000,
            profile,
            mechanics,
        );
        let contacts = (0..depth)
            .map(|index| {
                cell(
                    &mut local,
                    root + 10 + u64::try_from(case * 100 + index).unwrap(),
                    i32::try_from(index * 10).unwrap(),
                    1,
                )
            })
            .collect::<Vec<_>>();
        let effect = cell(
            &mut local,
            root + 500 + u64::try_from(case).unwrap(),
            500,
            2,
        );
        let source = cell(
            &mut local,
            root + 600 + u64::try_from(case).unwrap(),
            -10,
            1,
        );
        anchor(
            &mut local,
            root + u64::try_from(case).unwrap() * 1_000,
            &contacts
                .iter()
                .copied()
                .chain([effect, source])
                .collect::<Vec<_>>(),
        );
        arrow(
            &mut local,
            source,
            contacts[0],
            1,
            0,
            TransmissionMode::Drive,
            100,
        );
        let mut learned = Vec::new();
        for index in 0..depth {
            let to = if index + 1 < depth {
                contacts[index + 1]
            } else {
                effect
            };
            learned.push(arrow(
                &mut local,
                contacts[index],
                to,
                1,
                0,
                TransmissionMode::Drive,
                100,
            ));
        }
        arrow(
            &mut local,
            effect,
            contacts[depth - 1],
            1,
            0,
            TransmissionMode::Modulatory,
            100,
        );
        for index in (1..depth).rev() {
            local.add_arrow_with_trigger(
                ArrowSpec {
                    from: contacts[index],
                    to: contacts[index - 1],
                    delay: 0,
                    phase: 0,
                    coupling: 1,
                    resistance: 100,
                    mode: TransmissionMode::Modulatory,
                },
                TransmissionTrigger::QualifiedLocalParticipation,
            );
        }
        let before = learned
            .iter()
            .map(|id| local.core0_resistance_material(*id))
            .collect::<Vec<_>>();
        let forward = pulse(
            &mut local,
            source,
            0,
            1,
            root + 7_000 + u64::try_from(case).unwrap(),
        );
        let consequence = pulse(
            &mut local,
            effect,
            1,
            2,
            root + 8_000 + u64::try_from(case).unwrap(),
        );
        let supported = learned
            .iter()
            .zip(before)
            .all(|(id, before)| local.core0_resistance_material(*id) > before);
        let qlp = forward
            .work
            .qualified_local_traversals
            .saturating_add(consequence.work.qualified_local_traversals);
        pass &= supported
            && qlp == u64::try_from(depth.saturating_sub(1)).unwrap()
            && consequence.naturally_quiescent;
        all_trace.extend(forward.physical_trace);
        all_trace.extend(consequence.physical_trace);
        total_work = total_work
            .saturating_add(forward.work.physical_total())
            .saturating_add(consequence.work.physical_total());
        total_qlp = total_qlp.saturating_add(qlp);
        representative = local;
    }
    finish(
        &representative,
        pass,
        "one local closure rule composes through depths 1/2/4/8/16",
        all_trace,
        total_work,
        0,
        0,
        total_qlp,
        true,
    )
}

fn recurrence_world(
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
    learned: bool,
) -> Observation {
    let mut body = body(root, profile, mechanics);
    let a = cell(&mut body, root + 1, 0, 2);
    let b = cell(&mut body, root + 2, 10, 2);
    let ia = cell(&mut body, root + 3, 20, 1);
    let ib = cell(&mut body, root + 4, 30, 1);
    anchor(&mut body, root, &[a, b, ia, ib]);
    arrow(
        &mut body,
        a,
        b,
        2,
        1,
        TransmissionMode::Drive,
        HIGH_RESISTANCE,
    );
    arrow(
        &mut body,
        b,
        a,
        2,
        1,
        TransmissionMode::Drive,
        HIGH_RESISTANCE,
    );
    let ia_link = arrow(
        &mut body,
        a,
        ia,
        1,
        0,
        TransmissionMode::Drive,
        HIGH_RESISTANCE,
    );
    let ib_link = arrow(
        &mut body,
        b,
        ib,
        1,
        0,
        TransmissionMode::Drive,
        HIGH_RESISTANCE,
    );
    let na = arrow(
        &mut body,
        ia,
        a,
        -16,
        0,
        TransmissionMode::Drive,
        HIGH_RESISTANCE,
    );
    let nb = arrow(
        &mut body,
        ib,
        b,
        -16,
        0,
        TransmissionMode::Drive,
        HIGH_RESISTANCE,
    );
    if learned {
        let train = pulse(&mut body, a, 0, 2, root + 1_000);
        let _ = modulate(&mut body, root, ia, 2);
        let _ = modulate(&mut body, root + 1, ib, 3);
        let _ = train;
    }
    let observed = {
        body.enter(SpikeInput {
            arrival_tick: body.clock().tick.saturating_add(1),
            phase: 0,
            origin_physical: root + 2_000,
            target: a,
            impulse: 2,
        });
        body.propagate_with_observation_ceiling(CEILING)
    };
    let intended = fires(&observed.run.physical_trace, b) >= 1;
    let inhibition = [ia_link, ib_link, na, nb]
        .iter()
        .all(|id| body.resolve_arrow(body.arrow_reference(*id)).is_some());
    let pass = intended
        && inhibition
        && observed.run.naturally_quiescent
        && !observed.observation_ceiling_reached;
    finish(
        &body,
        pass,
        "ordinary inhibitory topology preserves intended traversal and settles recurrence",
        observed.run.physical_trace,
        observed.run.work.physical_total(),
        observed.run.work.local_structural_proposals,
        observed.run.work.physical_deallocations,
        observed.run.work.qualified_local_traversals,
        observed.run.naturally_quiescent,
    )
}

#[derive(Clone, Copy)]
enum E12Consequence {
    Local,
    None,
    Wrong,
}

struct E12Body {
    body: PlasticSubstrate,
    a: CellId,
    b: CellId,
    ia: CellId,
    ib: CellId,
    unrelated: CellId,
    modulator: CellId,
    negative: [ArrowId; 2],
    unrelated_negative: ArrowId,
    positive: [ArrowId; 2],
}

struct E12Training {
    world: E12Body,
    coupling_curve: Vec<i64>,
    stable_probe: Vec<bool>,
    traces: Vec<Vec<PhysicalTransition>>,
    work: u64,
    quiescent: bool,
}

fn e12_body(
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
    permuted: bool,
    consequence: E12Consequence,
) -> E12Body {
    let mut body = body(root, profile, mechanics);
    let a = cell(&mut body, root + 1, 0, 100);
    let b = cell(&mut body, root + 2, 100, 100);
    let (ia, ib, pa, pb, unrelated) = if permuted {
        let pb = cell(&mut body, root + 30, 40_000, 1);
        let unrelated = cell(&mut body, root + 50, 50_000, 1);
        let ib = cell(&mut body, root + 4, 20_000, 1);
        let pa = cell(&mut body, root + 20, 30_000, 1);
        let ia = cell(&mut body, root + 3, 10_000, 1);
        (ia, ib, pa, pb, unrelated)
    } else {
        (
            cell(&mut body, root + 3, 10_000, 1),
            cell(&mut body, root + 4, 20_000, 1),
            cell(&mut body, root + 20, 30_000, 1),
            cell(&mut body, root + 30, 40_000, 1),
            cell(&mut body, root + 50, 50_000, 1),
        )
    };
    let modulator = cell(&mut body, root + 60, 60_000, 1);
    let wrong = cell(&mut body, root + 70, 70_000, 100);
    let negative = [
        arrow(
            &mut body,
            ia,
            a,
            -2,
            0,
            TransmissionMode::Drive,
            HIGH_RESISTANCE,
        ),
        arrow(
            &mut body,
            ib,
            b,
            -2,
            0,
            TransmissionMode::Drive,
            HIGH_RESISTANCE,
        ),
    ];
    let unrelated_negative = arrow(
        &mut body,
        unrelated,
        a,
        -2,
        0,
        TransmissionMode::Drive,
        HIGH_RESISTANCE,
    );
    let positive = [
        arrow(
            &mut body,
            pa,
            a,
            2,
            0,
            TransmissionMode::Drive,
            HIGH_RESISTANCE,
        ),
        arrow(
            &mut body,
            pb,
            b,
            2,
            0,
            TransmissionMode::Drive,
            HIGH_RESISTANCE,
        ),
    ];
    if !matches!(consequence, E12Consequence::None) {
        let targets = if matches!(consequence, E12Consequence::Wrong) {
            [wrong, wrong]
        } else {
            [ia, ib]
        };
        for target in targets {
            arrow(
                &mut body,
                modulator,
                target,
                2,
                0,
                TransmissionMode::Modulatory,
                HIGH_RESISTANCE,
            );
        }
    }
    E12Body {
        body,
        a,
        b,
        ia,
        ib,
        unrelated,
        modulator,
        negative,
        unrelated_negative,
        positive,
    }
}

fn e12_probe(world: &E12Body) -> (bool, usize, usize, u64, Vec<PhysicalTransition>) {
    let mut body = world.body.clone();
    for (from, to, coupling) in [
        (world.a, world.b, 2),
        (world.b, world.a, 2),
        (world.a, world.ia, 1),
        (world.b, world.ib, 1),
    ] {
        arrow(
            &mut body,
            from,
            to,
            coupling,
            if coupling == 2 { 1 } else { 0 },
            TransmissionMode::Drive,
            HIGH_RESISTANCE,
        );
    }
    body.enter(SpikeInput {
        arrival_tick: body.clock().tick.saturating_add(1),
        phase: 0,
        origin_physical: 98_000_000,
        target: world.a,
        impulse: 2,
    });
    let first = body.propagate_with_observation_ceiling(256);
    let mut trace = first.run.physical_trace.clone();
    let mut work = first.run.work.physical_total();
    let mut quiescent = first.run.naturally_quiescent;
    if first.observation_ceiling_reached {
        let continuation = body.propagate_with_observation_ceiling(32);
        trace.extend(continuation.run.physical_trace.clone());
        work = work.saturating_add(continuation.run.work.physical_total());
        quiescent = continuation.run.naturally_quiescent;
    }
    (
        quiescent,
        fires(&trace, world.a),
        fires(&trace, world.b),
        work,
        trace,
    )
}

fn e12_train(
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
    permuted: bool,
    consequence: E12Consequence,
) -> E12Training {
    let mut world = e12_body(profile, mechanics, root, permuted, consequence);
    let mut curve = vec![-world.body.core0_coupling_material(world.negative[0])];
    let initial_probe = e12_probe(&world);
    let mut probe_classes = vec![initial_probe.0 && initial_probe.1 == 1 && initial_probe.2 == 1];
    let mut traces = vec![initial_probe.4];
    let mut work = initial_probe.3;
    let mut quiescent = true;
    for experience in 0..6_u64 {
        let start = world.body.clock().tick.saturating_add(1);
        let traversal = pulse_many(
            &mut world.body,
            &[world.ia, world.ib, world.unrelated],
            start,
            1,
            root + 10_000 + experience * 1_000,
        );
        work = work.saturating_add(traversal.work.physical_total());
        traces.push(traversal.physical_trace);
        quiescent &= traversal.naturally_quiescent;
        if !matches!(consequence, E12Consequence::None) {
            let closure = pulse(
                &mut world.body,
                world.modulator,
                start.saturating_add(12),
                1,
                root + 20_000 + experience * 1_000,
            );
            work = work.saturating_add(closure.work.physical_total());
            traces.push(closure.physical_trace);
            quiescent &= closure.naturally_quiescent;
        } else {
            world.body.advance_time(start.saturating_add(12));
        }
        curve.push(-world.body.core0_coupling_material(world.negative[0]));
        let probe = e12_probe(&world);
        probe_classes.push(probe.0 && probe.1 == 1 && probe.2 == 1);
        work = work.saturating_add(probe.3);
        traces.push(probe.4);
    }
    E12Training {
        world,
        coupling_curve: curve,
        stable_probe: probe_classes,
        traces,
        work,
        quiescent,
    }
}

fn gate_e12(profile: Core0Profile, mechanics: MechanicalConfig, root: u64) -> Observation {
    let primary = e12_train(profile, mechanics, root, false, E12Consequence::Local);
    let permuted = e12_train(
        profile,
        mechanics,
        root + 100_000,
        true,
        E12Consequence::Local,
    );
    let none = e12_train(
        profile,
        mechanics,
        root + 200_000,
        false,
        E12Consequence::None,
    );
    let wrong = e12_train(
        profile,
        mechanics,
        root + 300_000,
        false,
        E12Consequence::Wrong,
    );
    let E12Training {
        world: mut learned,
        coupling_curve: curve,
        stable_probe: stable,
        traces,
        mut work,
        mut quiescent,
    } = primary;
    let unrelated_before = learned
        .body
        .core0_coupling_material(learned.unrelated_negative);
    let positive_before = learned
        .positive
        .map(|id| learned.body.core0_coupling_material(id));
    let learned_before = learned
        .negative
        .map(|id| learned.body.core0_coupling_material(id));
    let tick = learned.body.clock().tick.saturating_add(1);
    let final_use = pulse_many(
        &mut learned.body,
        &[learned.ia, learned.ib, learned.unrelated],
        tick,
        1,
        root + 900_000,
    );
    work = work.saturating_add(final_use.work.physical_total());
    quiescent &= final_use.naturally_quiescent;
    let learned_after = learned
        .negative
        .map(|id| learned.body.core0_coupling_material(id));
    let monotonic = curve.windows(2).all(|pair| pair[1] > pair[0]);
    let zero_active = !stable[0];
    let later_stable = stable.iter().skip(1).any(|value| *value);
    let controls = none
        .coupling_curve
        .iter()
        .all(|value| *value == none.coupling_curve[0])
        && wrong
            .coupling_curve
            .iter()
            .all(|value| *value == wrong.coupling_curve[0])
        && curve == permuted.coupling_curve
        && stable == permuted.stable_probe
        && learned_before == learned_after
        && unrelated_before
            == learned
                .body
                .core0_coupling_material(learned.unrelated_negative)
        && positive_before
            == learned
                .positive
                .map(|id| learned.body.core0_coupling_material(id));
    let pass = monotonic
        && zero_active
        && later_stable
        && controls
        && quiescent
        && permuted.quiescent
        && none.quiescent
        && wrong.quiescent;
    let trace = traces.into_iter().flatten().collect::<Vec<_>>();
    let reason = format!(
        "six fixed experiences curve={} stable={}",
        curve
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join("|"),
        stable
            .iter()
            .map(bool::to_string)
            .collect::<Vec<_>>()
            .join("|")
    );
    let proposals = final_use.work.local_structural_proposals;
    let deallocations = final_use.work.physical_deallocations;
    let qlp = final_use.work.qualified_local_traversals;
    finish(
        &learned.body,
        pass,
        reason,
        trace,
        work,
        proposals,
        deallocations,
        qlp,
        quiescent,
    )
}

fn frames() -> Vec<Vec<u8>> {
    let mut frames = Vec::new();
    let mut contexts = BTreeSet::new();
    for nonce in 0_u16..u16::MAX {
        let mut candidate = vec![4_u8; ARC3_FRAME_PIXELS];
        candidate[0] = (nonce & 0x0f) as u8;
        candidate[1] = (nonce >> 4 & 0x0f) as u8;
        candidate[2] = (nonce >> 8 & 0x0f) as u8;
        candidate[3] = (nonce >> 12 & 0x0f) as u8;
        let context = spatial_context(&candidate).expect("valid frame");
        if contexts.insert(context) {
            frames.push(candidate);
            if frames.len() == 5 {
                break;
            }
        }
    }
    frames
}

fn academy_gate(
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
    probe_all: bool,
) -> Observation {
    let mut organism =
        Arc3Sensorimotor::new_spatial_with_profile(root, mechanics, profile).expect("Academy body");
    let frames = frames();
    let actions = [1_u8, 4, 2, 3];
    let mut observed = Vec::new();
    for (index, action) in actions.into_iter().enumerate() {
        observed.push(
            organism
                .observe(
                    frames[index].clone(),
                    &[1, 2, 3, 4],
                    Some(action),
                    index > 0,
                    false,
                    &[1, 2, 3, 4],
                )
                .expect("training observation"),
        );
    }
    observed.push(
        organism
            .observe(
                frames[4].clone(),
                &[1, 2, 3, 4],
                None,
                true,
                false,
                &[1, 2, 3, 4],
            )
            .expect("closure observation"),
    );
    let pass = if probe_all {
        organism.clear_episode();
        let mut outputs = Vec::new();
        for frame in frames.iter().take(4) {
            outputs.push(
                organism
                    .observe(
                        frame.clone(),
                        &[1, 2, 3, 4],
                        None,
                        false,
                        false,
                        &[1, 2, 3, 4],
                    )
                    .expect("probe")
                    .action,
            );
            organism.clear_episode();
        }
        outputs == [Some(1), Some(4), Some(2), Some(3)]
    } else {
        observed
            .iter()
            .take(4)
            .map(|entry| entry.action)
            .collect::<Vec<_>>()
            == [Some(1), Some(4), Some(2), Some(3)]
            && observed
                .iter()
                .map(|entry| entry.plasticity_updates)
                .collect::<Vec<_>>()
                == [0, 1, 1, 1, 1]
    };
    let work = observed.iter().map(|entry| entry.physical_work).sum();
    let quiescent = observed.iter().all(|entry| entry.naturally_quiescent);
    Observation {
        pass: pass && quiescent,
        reason: if probe_all {
            "four learned context/action relations re-execute"
        } else {
            "frozen ARC A2 regimen remains unchanged"
        }
        .to_string(),
        trace: Vec::new(),
        arrows: Vec::new(),
        tick: observed.last().map_or(0, |entry| entry.physical_tick),
        work,
        proposals: 0,
        deallocations: 0,
        qlp: 0,
        live_cells: 0,
        live_arrows: 0,
        quiescent,
    }
}

fn execute(
    gate: Gate,
    profile: Core0Profile,
    mechanics: MechanicalConfig,
    root: u64,
) -> Observation {
    match gate {
        Gate::E0 => gate_e0(profile, mechanics, root),
        Gate::E1 => gate_e1(profile, mechanics, root),
        Gate::E2 => gate_e2(profile, mechanics, root),
        Gate::E3 => gate_e3(profile, mechanics, root),
        Gate::E4 => gate_e4(profile, mechanics, root),
        Gate::E5 => gate_e5(profile, mechanics, root),
        Gate::E6 => competing(profile, mechanics, root),
        Gate::E7 => gate_e7(profile, mechanics, root),
        Gate::E8 => competing(profile, mechanics, root),
        Gate::E9 => gate_e9(profile, mechanics, root),
        Gate::E10 => gate_e10(profile, mechanics, root),
        Gate::E11 => recurrence_world(profile, mechanics, root, false),
        Gate::E12 => gate_e12(profile, mechanics, root),
        Gate::E13 => academy_gate(profile, mechanics, root, true),
        Gate::E14 => academy_gate(profile, mechanics, root, false),
    }
}

fn equivalent(left: &Observation, right: &Observation) -> bool {
    left.pass == right.pass
        && left.trace == right.trace
        && left.arrows == right.arrows
        && left.tick == right.tick
        && left.work == right.work
        && left.proposals == right.proposals
        && left.deallocations == right.deallocations
        && left.qlp == right.qlp
        && left.live_cells == right.live_cells
        && left.live_arrows == right.live_arrows
        && left.quiescent == right.quiescent
}

fn run_matrix() -> Vec<Row> {
    let mut rows = Vec::new();
    for (profile_index, profile) in CORE1_PROFILES.into_iter().enumerate() {
        let mut reached = true;
        for (gate_index, gate) in Gate::ALL.into_iter().enumerate() {
            if !reached {
                rows.push(Row {
                    profile,
                    gate,
                    status: "NOT_REACHED",
                    experiences: 0,
                    work: 0,
                    live_cells: 0,
                    live_arrows: 0,
                    proposals: 0,
                    deallocations: 0,
                    qlp: 0,
                    replay_exact: false,
                    mechanics_exact: false,
                    reason: "earlier capability failed".to_string(),
                });
                continue;
            }
            let root =
                8_000_000 + u64::try_from(profile_index * 100_000 + gate_index * 1_000).unwrap();
            eprintln!(
                "CORE1_DIAGNOSTIC profile={} gate={:?} mechanics=reference replay=0",
                profile_name(profile),
                gate
            );
            let reference = execute(gate, profile, MechanicalConfig::REFERENCE, root);
            eprintln!(
                "CORE1_DIAGNOSTIC profile={} gate={:?} mechanics=reference replay=1",
                profile_name(profile),
                gate
            );
            let replay = execute(gate, profile, MechanicalConfig::REFERENCE, root);
            eprintln!(
                "CORE1_DIAGNOSTIC profile={} gate={:?} mechanics=production replay=0",
                profile_name(profile),
                gate
            );
            let production = execute(gate, profile, MechanicalConfig::PRODUCTION, root);
            let replay_exact = equivalent(&reference, &replay);
            let mechanics_exact = equivalent(&reference, &production);
            let pass = reference.pass && reference.quiescent && replay_exact && mechanics_exact;
            let reason = if !reference.pass {
                reference.reason.clone()
            } else if !reference.quiescent {
                "not naturally quiescent".to_string()
            } else if !replay_exact {
                "exact replay mismatch".to_string()
            } else if !mechanics_exact {
                "Reference/Production physical mismatch".to_string()
            } else {
                reference.reason.clone()
            };
            rows.push(Row {
                profile,
                gate,
                status: if pass { "PASS" } else { "FAIL" },
                experiences: gate.experiences(),
                work: reference.work,
                live_cells: reference.live_cells,
                live_arrows: reference.live_arrows,
                proposals: reference.proposals,
                deallocations: reference.deallocations,
                qlp: reference.qlp,
                replay_exact,
                mechanics_exact,
                reason,
            });
            reached = pass;
        }
    }
    rows
}

fn write_results(destination: &Path, rows: &[Row]) {
    fs::create_dir_all(destination).expect("create CORE1 result directory");
    let mut csv = String::from("profile,gate,capability,status,experiences,physical_work,live_cells,live_arrows,proposals,deallocations,qlp_traversals,replay_exact,mechanics_exact,new_physics,reason\n");
    for row in rows {
        writeln!(
            csv,
            "{},{:?},{},{},{},{},{},{},{},{},{},{},{},false,{}",
            profile_name(row.profile),
            row.gate,
            row.gate.name(),
            row.status,
            row.experiences,
            row.work,
            row.live_cells,
            row.live_arrows,
            row.proposals,
            row.deallocations,
            row.qlp,
            row.replay_exact,
            row.mechanics_exact,
            row.reason.replace(',', ";")
        )
        .unwrap();
    }
    fs::write(destination.join("matrix.csv"), csv).expect("write CORE1 CSV");

    let mut report = String::from("# CORE1 radical de-supply result\n\n| Profile | First failure | Passed prefix |\n|---|---|---:|\n");
    for profile in CORE1_PROFILES {
        let profile_rows = rows
            .iter()
            .filter(|row| row.profile == profile)
            .collect::<Vec<_>>();
        let first_failure = profile_rows
            .iter()
            .find(|row| row.status == "FAIL")
            .map_or("none".to_string(), |row| {
                format!("{:?} {}", row.gate, row.gate.name())
            });
        let passed = profile_rows
            .iter()
            .filter(|row| row.status == "PASS")
            .count();
        writeln!(
            report,
            "| {} | {} | {} |",
            profile_name(profile),
            first_failure,
            passed
        )
        .unwrap();
    }
    report.push_str("\n## Complete prefix matrix\n\n| Profile | Gate | Capability | Status | Experiences | PhysicalWork | Reason |\n|---|---|---|---|---:|---:|---|\n");
    for row in rows {
        writeln!(
            report,
            "| {} | {:?} | {} | {} | {} | {} | {} |",
            profile_name(row.profile),
            row.gate,
            row.gate.name(),
            row.status,
            row.experiences,
            row.work,
            row.reason
        )
        .unwrap();
    }
    fs::write(destination.join("report.md"), report).expect("write CORE1 report");
}

fn main() {
    eprintln!("CORE1_RADICAL_DESUPPLY_V1_EVIDENCE_SPENT");
    let destination = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/core1_radical_desupply_v1"));
    let rows = run_matrix();
    write_results(&destination, &rows);
    let failures = rows.iter().filter(|row| row.status == "FAIL").count();
    assert_eq!(rows.len(), 75);
    assert!(
        failures > 0,
        "destructive ablation unexpectedly has no frontier"
    );
    println!(
        "CORE1_COMPLETE rows={} first_failures={failures}",
        rows.len()
    );
}
