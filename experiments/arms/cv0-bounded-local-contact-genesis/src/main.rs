#![forbid(unsafe_code)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowRef, ArrowSpec, CellId, CellRef, CellSlot, ContentHash, MechanicalConfig,
    PhysicalTransition, PlasticSubstrate, SpikeInput, TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [7_500_000, 7_600_001];
const PHASES: std::ops::Range<i64> = 0..10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    GateE,
    Full,
}

impl Stage {
    fn parse(value: &str) -> Self {
        match value {
            "gate-e" => Self::GateE,
            "full" => Self::Full,
            other => panic!("unknown CV0-J0 stage: {other}"),
        }
    }

    fn families(self) -> &'static [Family] {
        match self {
            Self::GateE => &[
                Family::AnchorOnly,
                Family::PositiveConsequence,
                Family::AnchorPermutation,
            ],
            Self::Full => &Family::ALL,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    AnchorOnly,
    Symmetry,
    NoConsequence,
    BoundedCreation,
    OrphanReuse,
    PositiveConsequence,
    NegativeConsequence,
    Permutation,
    NeitherUseful,
    BothUseful,
    SharedContact,
    AnchorPermutation,
}

impl Family {
    const ALL: [Self; 12] = [
        Self::AnchorOnly,
        Self::Symmetry,
        Self::NoConsequence,
        Self::BoundedCreation,
        Self::OrphanReuse,
        Self::PositiveConsequence,
        Self::NegativeConsequence,
        Self::Permutation,
        Self::NeitherUseful,
        Self::BothUseful,
        Self::SharedContact,
        Self::AnchorPermutation,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::AnchorOnly => "v2_anchor_only_control",
            Self::Symmetry => "gate_a_sign_symmetry",
            Self::NoConsequence => "gate_b_no_consequence",
            Self::BoundedCreation => "gate_c_bounded_creation",
            Self::OrphanReuse => "gate_d_orphan_cleanup_reuse",
            Self::PositiveConsequence => "gate_e_positive_consequence",
            Self::NegativeConsequence => "gate_f_negative_consequence",
            Self::Permutation => "gate_g_permutation",
            Self::NeitherUseful => "gate_h_neither_useful",
            Self::BothUseful => "gate_h_both_useful",
            Self::SharedContact => "gate_i_shared_contact_alias",
            Self::AnchorPermutation => "v2_anchor_identity_slot_permutation",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    arrow_updates: u64,
    arrow_proposals: u64,
    cell_proposals: u64,
    arrow_deallocations: u64,
    cell_deallocations: u64,
}

impl WorkTotals {
    fn add(&mut self, work: Work) {
        self.physical = self.physical.saturating_add(work.physical_total());
        self.drive = self.drive.saturating_add(work.drive_deliveries);
        self.modulation = self.modulation.saturating_add(work.modulatory_deliveries);
        self.arrow_updates = self.arrow_updates.saturating_add(work.local_return_updates);
        self.arrow_proposals = self
            .arrow_proposals
            .saturating_add(work.local_structural_proposals);
        self.cell_proposals = self
            .cell_proposals
            .saturating_add(work.local_cell_proposals);
        self.arrow_deallocations = self
            .arrow_deallocations
            .saturating_add(work.physical_deallocations);
        self.cell_deallocations = self
            .cell_deallocations
            .saturating_add(work.cell_deallocations);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Observation {
    markers: Vec<String>,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    final_tick: i64,
    body_hash: String,
    naturally_quiescent: bool,
    checks: Vec<(String, bool)>,
}

impl Observation {
    fn passed(&self) -> bool {
        self.checks.iter().all(|(_, passed)| *passed)
    }

    fn failed_names(&self) -> String {
        self.checks
            .iter()
            .filter(|(_, passed)| !passed)
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join("|")
    }
}

#[derive(Clone, Copy, Debug)]
struct Candidate {
    contact: CellId,
    contact_ref: CellRef,
    contact_slot: CellSlot,
    sign: i32,
    stem: ArrowRef,
    outgoing: ArrowRef,
}

struct World {
    body: PlasticSubstrate,
    origin: i64,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    naturally_quiescent: bool,
    source: CellId,
    target: CellId,
    anchor: CellId,
    anchor_to_source: ArrowRef,
    target_to_anchor: ArrowRef,
}

impl World {
    fn new(root: u64, phase: i64, mechanics: MechanicalConfig) -> Self {
        Self::new_with_geometry(root, phase, mechanics, 2, 0, false)
    }

    fn new_with_geometry(
        root: u64,
        phase: i64,
        mechanics: MechanicalConfig,
        target_physical_offset: u64,
        position_offset: i32,
        anchor_first: bool,
    ) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 16, 32, mechanics);
        body.set_physical_tracing(true);
        body.advance_time(phase);
        let add_anchor = |body: &mut PlasticSubstrate, physical_offset: u64| {
            body.add_cell(truelearner_core::CellSpec {
                physical_id: root + physical_offset,
                position: position_offset.saturating_add(1_000),
                region: 0,
                threshold: 100,
                resistance: 500,
            })
        };
        let early_anchor = anchor_first.then(|| add_anchor(&mut body, 901));
        let source = body.add_cell(truelearner_core::CellSpec {
            physical_id: root + 1,
            position: position_offset,
            region: 0,
            threshold: 1,
            resistance: 500,
        });
        let target = body.add_cell(truelearner_core::CellSpec {
            physical_id: root + target_physical_offset,
            position: position_offset.saturating_add(1),
            region: 0,
            threshold: 100,
            resistance: 500,
        });
        let anchor = early_anchor.unwrap_or_else(|| add_anchor(&mut body, 900));
        let anchor_to_source_id = body.add_arrow(ArrowSpec {
            from: anchor,
            to: source,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 500,
            mode: TransmissionMode::Drive,
        });
        let target_to_anchor_id = body.add_arrow(ArrowSpec {
            from: target,
            to: anchor,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 500,
            mode: TransmissionMode::Drive,
        });
        let anchor_to_source = body.arrow_reference(anchor_to_source_id);
        let target_to_anchor = body.arrow_reference(target_to_anchor_id);
        Self {
            body,
            origin: phase,
            trace: Vec::new(),
            work: WorkTotals::default(),
            naturally_quiescent: true,
            source,
            target,
            anchor,
            anchor_to_source,
            target_to_anchor,
        }
    }

    fn pulse(&mut self, target: CellId, age: i64, origin_physical: u64) {
        self.pulse_many(age, &[(target, origin_physical)]);
    }

    fn pulse_many(&mut self, age: i64, targets: &[(CellId, u64)]) {
        let arrivals = targets
            .iter()
            .map(|(target, origin_physical)| SpikeInput {
                arrival_tick: self.origin.saturating_add(age),
                phase: 0,
                origin_physical: *origin_physical,
                target: *target,
                impulse: 1,
            })
            .collect::<Vec<_>>();
        let result = self.body.arrive(&arrivals, i16::MAX);
        self.trace.extend(result.physical_trace);
        self.work.add(result.work);
        self.naturally_quiescent &= result.naturally_quiescent;
    }

    fn advance_age(&mut self, age: i64) {
        let target = self.origin.saturating_add(age);
        while self.body.clock().tick < target {
            let result = self
                .body
                .advance_time_traced(self.body.clock().tick.saturating_add(1));
            self.trace.extend(result.physical_trace);
            self.work.add(result.work);
            self.naturally_quiescent &= result.naturally_quiescent;
        }
    }

    fn create_candidates(&mut self, root: u64) -> [Candidate; 2] {
        self.pulse(self.source, 0, root + 10);
        self.candidates()
    }

    fn candidates(&self) -> [Candidate; 2] {
        let body = self.body.arena_body(1);
        let mut candidates = body
            .cells
            .iter()
            .filter(|cell| cell.id != self.source && cell.id != self.target)
            .filter_map(|cell| {
                let stem = body
                    .arrows
                    .iter()
                    .find(|arrow| arrow.from.id == self.source && arrow.to.id == cell.id)?;
                let outgoing = body
                    .arrows
                    .iter()
                    .find(|arrow| arrow.from.id == cell.id && arrow.to.id == self.target)?;
                Some(Candidate {
                    contact: cell.id,
                    contact_ref: self.body.cell_reference(cell.id),
                    contact_slot: self.body.cell_resident_slot(cell.id)?,
                    sign: outgoing.coupling,
                    stem: self.body.arrow_reference(stem.id),
                    outgoing: self.body.arrow_reference(outgoing.id),
                })
            })
            .collect::<Vec<_>>();
        candidates.sort_by_key(|candidate| candidate.sign);
        candidates.try_into().expect("exactly two signed contacts")
    }

    fn add_modulation_to(&mut self, root: u64, physical_offset: u64, target: CellId) -> CellId {
        let modulator = self.body.add_cell(truelearner_core::CellSpec {
            physical_id: root + physical_offset,
            position: 100,
            region: 0,
            threshold: 1,
            resistance: 500,
        });
        self.body.add_arrow(ArrowSpec {
            from: modulator,
            to: target,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 500,
            mode: TransmissionMode::Modulatory,
        });
        modulator
    }

    fn relation_live(&self, candidate: Candidate) -> (bool, bool, bool) {
        (
            self.body.resolve_cell(candidate.contact_ref).is_some(),
            self.body.resolve_arrow(candidate.stem).is_some(),
            self.body.resolve_arrow(candidate.outgoing).is_some(),
        )
    }

    fn finish(self, markers: Vec<String>, checks: Vec<(String, bool)>) -> Observation {
        Observation {
            markers,
            trace: self.trace,
            work: self.work,
            final_tick: self.body.clock().tick,
            body_hash: ContentHash::of(&self.body.canonical_body_bytes(1).unwrap()).to_string(),
            naturally_quiescent: self.naturally_quiescent,
            checks,
        }
    }
}

fn observe(family: Family, root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    match family {
        Family::AnchorOnly => observe_anchor_only(root, phase, mechanics),
        Family::Symmetry => observe_symmetry(root, phase, mechanics),
        Family::NoConsequence => observe_no_consequence(root, phase, mechanics),
        Family::BoundedCreation => observe_bounded(root, phase, mechanics),
        Family::OrphanReuse => observe_orphan_reuse(root, phase, mechanics),
        Family::PositiveConsequence => observe_positive(root, phase, mechanics),
        Family::NegativeConsequence => observe_negative(root, phase, mechanics),
        Family::Permutation => observe_permutation(root, phase, mechanics),
        Family::NeitherUseful => observe_neither(root, phase, mechanics),
        Family::BothUseful => observe_both(root, phase, mechanics),
        Family::SharedContact => observe_shared_contact(root, phase, mechanics),
        Family::AnchorPermutation => observe_anchor_permutation(root, phase, mechanics),
    }
}

fn observe_anchor_only(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let source_ref = world.body.cell_reference(world.source);
    let target_ref = world.body.cell_reference(world.target);
    let anchor_ref = world.body.cell_reference(world.anchor);
    world.advance_age(10);
    let anchor_link_resistances = [
        arrow_resistance(&world, world.anchor_to_source),
        arrow_resistance(&world, world.target_to_anchor),
    ];
    let markers = vec![format!(
        "anchor={:?};boundary={:?}/{:?};links={anchor_link_resistances:?}",
        world.anchor, world.source, world.target
    )];
    let checks = vec![
        (
            "boundary_cells_remain_live".into(),
            world.body.resolve_cell(source_ref).is_some()
                && world.body.resolve_cell(target_ref).is_some(),
        ),
        (
            "anchor_remains_live".into(),
            world.body.resolve_cell(anchor_ref).is_some(),
        ),
        (
            "anchor_links_remain_live".into(),
            world.body.resolve_arrow(world.anchor_to_source).is_some()
                && world.body.resolve_arrow(world.target_to_anchor).is_some(),
        ),
        (
            "anchors_do_not_execute".into(),
            world.work.drive == 0
                && world.work.modulation == 0
                && world.work.arrow_updates == 0,
        ),
        (
            "anchors_do_not_generate_candidates".into(),
            world.work.cell_proposals == 0 && world.work.arrow_proposals == 0,
        ),
        (
            "anchors_do_not_deallocate".into(),
            world.work.cell_deallocations == 0 && world.work.arrow_deallocations == 0,
        ),
    ];
    world.finish(markers, checks)
}

fn symmetry_checks(world: &World, negative: Candidate, positive: Candidate) -> Vec<(String, bool)> {
    let body = world.body.arena_body(1);
    let neg_cell = body
        .cells
        .iter()
        .find(|cell| cell.id == negative.contact)
        .unwrap();
    let pos_cell = body
        .cells
        .iter()
        .find(|cell| cell.id == positive.contact)
        .unwrap();
    let neg_stem = body
        .arrows
        .iter()
        .find(|arrow| arrow.id == negative.stem.id)
        .unwrap();
    let pos_stem = body
        .arrows
        .iter()
        .find(|arrow| arrow.id == positive.stem.id)
        .unwrap();
    let neg_out = body
        .arrows
        .iter()
        .find(|arrow| arrow.id == negative.outgoing.id)
        .unwrap();
    let pos_out = body
        .arrows
        .iter()
        .find(|arrow| arrow.id == positive.outgoing.id)
        .unwrap();
    vec![
        (
            "exactly_two_cell_proposals".into(),
            world.work.cell_proposals == 2,
        ),
        (
            "exactly_four_arrow_proposals".into(),
            world.work.arrow_proposals == 4,
        ),
        (
            "signed_pair".into(),
            negative.sign == -1 && positive.sign == 1,
        ),
        (
            "contact_cell_symmetry".into(),
            neg_cell.position == pos_cell.position
                && neg_cell.region == pos_cell.region
                && neg_cell.threshold == pos_cell.threshold
                && neg_cell.resistance == pos_cell.resistance
                && neg_cell.generation == pos_cell.generation,
        ),
        (
            "stem_symmetry".into(),
            neg_stem.coupling == pos_stem.coupling
                && neg_stem.delay == pos_stem.delay
                && neg_stem.phase == pos_stem.phase
                && neg_stem.resistance == pos_stem.resistance,
        ),
        (
            "outgoing_differs_only_by_sign".into(),
            neg_out.delay == pos_out.delay
                && neg_out.phase == pos_out.phase
                && neg_out.resistance == pos_out.resistance
                && neg_out.transmission_mode == pos_out.transmission_mode
                && neg_out.coupling == -pos_out.coupling,
        ),
        (
            "both_relations_participated".into(),
            world.body.local_participation(negative.stem.id) > 0
                && world.body.local_participation(negative.outgoing.id) > 0
                && world.body.local_participation(positive.stem.id) > 0
                && world.body.local_participation(positive.outgoing.id) > 0,
        ),
    ]
}

fn observe_symmetry(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let [negative, positive] = world.create_candidates(root);
    let checks = symmetry_checks(&world, negative, positive);
    let markers = vec![format!(
        "negative={:?}/{:?};positive={:?}/{:?}",
        negative.contact, negative.contact_slot, positive.contact, positive.contact_slot
    )];
    world.finish(markers, checks)
}

fn observe_no_consequence(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let [negative, positive] = world.create_candidates(root);
    world.advance_age(10);
    let negative_live = world.relation_live(negative);
    let positive_live = world.relation_live(positive);
    let markers = vec![format!(
        "negative_live={negative_live:?};positive_live={positive_live:?}"
    )];
    let checks = vec![
        (
            "negative_relation_reclaimed".into(),
            negative_live == (false, false, false),
        ),
        (
            "positive_relation_reclaimed".into(),
            positive_live == (false, false, false),
        ),
        (
            "two_contact_deallocations".into(),
            world.work.cell_deallocations == 2,
        ),
        (
            "four_arrow_deallocations".into(),
            world.work.arrow_deallocations == 4,
        ),
        ("no_arrow_updates".into(), world.work.arrow_updates == 0),
    ];
    world.finish(markers, checks)
}

fn observe_bounded(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let [negative, positive] = world.create_candidates(root);
    for age in [2, 4, 6, 8] {
        world.pulse(world.source, age, root + 20 + u64::try_from(age).unwrap());
    }
    let before = (world.work.cell_proposals, world.work.arrow_proposals);
    world.advance_age(10);
    let markers = vec![format!("proposals={before:?}")];
    let checks = vec![
        ("one_pair_only".into(), before == (2, 4)),
        (
            "negative_eventually_reclaimed".into(),
            world.relation_live(negative) == (false, false, false),
        ),
        (
            "positive_eventually_reclaimed".into(),
            world.relation_live(positive) == (false, false, false),
        ),
        (
            "bounded_cell_deallocation".into(),
            world.work.cell_deallocations == 2,
        ),
        (
            "bounded_arrow_deallocation".into(),
            world.work.arrow_deallocations == 4,
        ),
    ];
    world.finish(markers, checks)
}

fn observe_orphan_reuse(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let [negative, positive] = world.create_candidates(root);
    let mut old_cell_slots = [negative.contact_slot, positive.contact_slot];
    let mut old_arrow_slots = [
        world.body.resolve_arrow(negative.stem),
        world.body.resolve_arrow(negative.outgoing),
        world.body.resolve_arrow(positive.stem),
        world.body.resolve_arrow(positive.outgoing),
    ];
    old_cell_slots.sort_by_key(|slot| slot.0);
    old_arrow_slots.sort_by_key(|slot| slot.map_or(usize::MAX, |slot| slot.0));
    world.advance_age(10);
    let replacement_a = world.body.add_cell(truelearner_core::CellSpec {
        physical_id: root + 200,
        position: 0,
        region: 0,
        threshold: 1,
        resistance: 10,
    });
    let replacement_b = world.body.add_cell(truelearner_core::CellSpec {
        physical_id: root + 201,
        position: 0,
        region: 0,
        threshold: 1,
        resistance: 10,
    });
    let mut new_cell_slots = [
        world.body.cell_resident_slot(replacement_a).unwrap(),
        world.body.cell_resident_slot(replacement_b).unwrap(),
    ];
    new_cell_slots.sort_by_key(|slot| slot.0);
    let new_arrows = [
        (world.source, replacement_a),
        (replacement_a, world.target),
        (world.source, replacement_b),
        (replacement_b, world.target),
    ]
    .map(|(from, to)| {
        let id = world.body.add_arrow(ArrowSpec {
            from,
            to,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 10,
            mode: TransmissionMode::Drive,
        });
        world.body.resolve_arrow(world.body.arrow_reference(id))
    });
    let mut new_arrow_slots = new_arrows;
    new_arrow_slots.sort_by_key(|slot| slot.map_or(usize::MAX, |slot| slot.0));
    let markers = vec![format!(
        "cell_slots={old_cell_slots:?}->{new_cell_slots:?};arrow_slots={old_arrow_slots:?}->{new_arrow_slots:?}"
    )];
    let checks = vec![
        (
            "contact_slots_reused".into(),
            old_cell_slots == new_cell_slots,
        ),
        (
            "arrow_slots_reused".into(),
            old_arrow_slots == new_arrow_slots,
        ),
        (
            "old_negative_ref_stale".into(),
            world.body.resolve_cell(negative.contact_ref).is_none(),
        ),
        (
            "old_positive_ref_stale".into(),
            world.body.resolve_cell(positive.contact_ref).is_none(),
        ),
        (
            "replacement_a_resolves".into(),
            world.body.cell_is_live(replacement_a) == Some(true),
        ),
        (
            "replacement_b_resolves".into(),
            world.body.cell_is_live(replacement_b) == Some(true),
        ),
    ];
    world.finish(markers, checks)
}

fn observe_positive(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    observe_selected(root, phase, mechanics, 1, false, false)
}

fn observe_negative(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    observe_selected(root, phase, mechanics, -1, false, false)
}

fn observe_permutation(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let selected_sign = if root & 1 == 0 { -1 } else { 1 };
    observe_selected(root, phase, mechanics, selected_sign, true, false)
}

fn observe_anchor_permutation(
    root: u64,
    phase: i64,
    mechanics: MechanicalConfig,
) -> Observation {
    observe_selected(root, phase, mechanics, 1, false, true)
}

fn arrow_resistance(world: &World, arrow: ArrowRef) -> Option<u32> {
    world
        .body
        .arena_body(1)
        .arrows
        .iter()
        .find(|candidate| candidate.id == arrow.id)
        .map(|candidate| candidate.resistance)
}

fn fire_count(trace: &[PhysicalTransition], cell: CellId) -> usize {
    trace
        .iter()
        .filter(|transition| {
            matches!(transition.event, truelearner_core::PhysicalEvent::Fire { cell: fired } if fired == cell)
        })
        .count()
}

fn observe_selected(
    root: u64,
    phase: i64,
    mechanics: MechanicalConfig,
    selected_sign: i32,
    permuted_geometry: bool,
    anchor_first: bool,
) -> Observation {
    let mut world = if permuted_geometry {
        World::new_with_geometry(root, phase, mechanics, 3, 100, anchor_first)
    } else {
        World::new_with_geometry(root, phase, mechanics, 2, 0, anchor_first)
    };
    let [negative, positive] = world.create_candidates(root);
    let (selected, unsupported) = if selected_sign < 0 {
        (negative, positive)
    } else {
        (positive, negative)
    };
    let selected_contact_before = world.body.cell_resistance(selected.contact);
    let unsupported_contact_before = world.body.cell_resistance(unsupported.contact);
    let source_ref = world.body.cell_reference(world.source);
    let target_ref = world.body.cell_reference(world.target);
    let anchor_ref = world.body.cell_reference(world.anchor);
    let anchor_slot = world.body.cell_resident_slot(world.anchor);
    let modulator = world.add_modulation_to(root, 100, selected.contact);
    world.pulse(modulator, 2, root + 101);
    let anchor_resistance_after_consequence = [
        arrow_resistance(&world, world.anchor_to_source),
        arrow_resistance(&world, world.target_to_anchor),
    ];
    let selected_stem_r = arrow_resistance(&world, selected.stem);
    let selected_out_r = arrow_resistance(&world, selected.outgoing);
    let unsupported_stem_r = arrow_resistance(&world, unsupported.stem);
    let unsupported_out_r = arrow_resistance(&world, unsupported.outgoing);
    let selected_contact_after = world.body.cell_resistance(selected.contact);
    let unsupported_contact_after = world.body.cell_resistance(unsupported.contact);
    world.advance_age(10);
    let selected_live = world.relation_live(selected);
    let unsupported_live = world.relation_live(unsupported);
    let selected_fires_before_probe = fire_count(&world.trace, selected.contact);
    let unsupported_fires_before_probe = fire_count(&world.trace, unsupported.contact);
    world.pulse(world.source, 10, root + 102);
    let selected_fires_after_probe = fire_count(&world.trace, selected.contact);
    let unsupported_fires_after_probe = fire_count(&world.trace, unsupported.contact);
    let markers = vec![format!(
        "selected_sign={selected_sign};selected_id={:?}/slot{:?};unsupported_id={:?}/slot{:?};anchor={:?}/slot{anchor_slot:?};selected_r=stem{selected_stem_r:?}/out{selected_out_r:?};unsupported_r=stem{unsupported_stem_r:?}/out{unsupported_out_r:?};anchor_r={anchor_resistance_after_consequence:?};live10=selected{selected_live:?}/unsupported{unsupported_live:?};fires={selected_fires_before_probe}->{selected_fires_after_probe}/{unsupported_fires_before_probe}->{unsupported_fires_after_probe}",
        selected.contact, selected.contact_slot, unsupported.contact, unsupported.contact_slot, world.anchor
    )];
    let checks = vec![
        (
            "selected_sign_matches_world".into(),
            selected.sign == selected_sign,
        ),
        (
            "junction_resistance_unchanged".into(),
            selected_contact_before == Some(1)
                && selected_contact_after == Some(1)
                && unsupported_contact_before == Some(1)
                && unsupported_contact_after == Some(1),
        ),
        (
            "selected_stem_consolidated".into(),
            selected_stem_r == Some(4),
        ),
        (
            "selected_outgoing_consolidated".into(),
            selected_out_r == Some(4),
        ),
        (
            "unsupported_stem_not_consolidated".into(),
            unsupported_stem_r == Some(1),
        ),
        (
            "unsupported_outgoing_not_consolidated".into(),
            unsupported_out_r == Some(1),
        ),
        (
            "unsupported_relation_reclaimed".into(),
            unsupported_live == (false, false, false),
        ),
        (
            "selected_relation_retained".into(),
            selected_live == (true, true, true),
        ),
        (
            "selected_relation_reexecutes".into(),
            selected_fires_after_probe == selected_fires_before_probe.saturating_add(1),
        ),
        (
            "unsupported_relation_stays_inert".into(),
            unsupported_fires_after_probe == unsupported_fires_before_probe,
        ),
        (
            "boundary_fixture_remains_live".into(),
            world.body.resolve_cell(source_ref).is_some()
                && world.body.resolve_cell(target_ref).is_some()
                && world.body.resolve_cell(anchor_ref).is_some(),
        ),
        (
            "no_credit_leakage_to_anchor_links".into(),
            anchor_resistance_after_consequence == [Some(500), Some(500)],
        ),
        (
            "two_link_updates_only".into(),
            world.work.arrow_updates == 2,
        ),
        (
            "one_orphan_reclaimed".into(),
            world.work.cell_deallocations == 1,
        ),
    ];
    world.finish(markers, checks)
}

fn observe_neither(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut observation = observe_no_consequence(root, phase, mechanics);
    observation.markers.push("neither_useful=true".into());
    observation
}

fn observe_both(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let [negative, positive] = world.create_candidates(root);
    let negative_modulator = world.add_modulation_to(root, 100, negative.contact);
    let positive_modulator = world.add_modulation_to(root, 101, positive.contact);
    world.pulse_many(
        2,
        &[
            (negative_modulator, root + 102),
            (positive_modulator, root + 103),
        ],
    );
    let resistances = [
        arrow_resistance(&world, negative.stem),
        arrow_resistance(&world, negative.outgoing),
        arrow_resistance(&world, positive.stem),
        arrow_resistance(&world, positive.outgoing),
    ];
    let cell_resistances = [
        world.body.cell_resistance(negative.contact),
        world.body.cell_resistance(positive.contact),
    ];
    world.advance_age(10);
    let negative_live = world.relation_live(negative);
    let positive_live = world.relation_live(positive);
    let negative_before = fire_count(&world.trace, negative.contact);
    let positive_before = fire_count(&world.trace, positive.contact);
    world.pulse(world.source, 10, root + 104);
    let negative_after = fire_count(&world.trace, negative.contact);
    let positive_after = fire_count(&world.trace, positive.contact);
    let markers = vec![format!(
        "resistances={resistances:?};cells={cell_resistances:?};live10={negative_live:?}/{positive_live:?};fires={negative_before}->{negative_after}/{positive_before}->{positive_after}"
    )];
    let checks = vec![
        (
            "both_relations_consolidated".into(),
            resistances == [Some(4), Some(4), Some(4), Some(4)],
        ),
        (
            "both_junctions_unchanged".into(),
            cell_resistances == [Some(1), Some(1)],
        ),
        (
            "both_relations_retained".into(),
            negative_live == (true, true, true) && positive_live == (true, true, true),
        ),
        (
            "both_relations_reexecute".into(),
            negative_after == negative_before.saturating_add(1)
                && positive_after == positive_before.saturating_add(1),
        ),
        ("four_link_updates".into(), world.work.arrow_updates == 4),
        (
            "no_orphan_reclamation".into(),
            world.work.cell_deallocations == 0,
        ),
    ];
    world.finish(markers, checks)
}

fn observe_shared_contact(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = World::new(root, phase, mechanics);
    let contact = world.body.add_cell(truelearner_core::CellSpec {
        physical_id: root + 3,
        position: 0,
        region: 0,
        threshold: 1,
        resistance: 1,
    });
    let stem_id = world.body.add_arrow(ArrowSpec {
        from: world.source,
        to: contact,
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance: 1,
        mode: TransmissionMode::Drive,
    });
    let positive_id = world.body.add_arrow(ArrowSpec {
        from: contact,
        to: world.target,
        delay: 1,
        phase: 0,
        coupling: 1,
        resistance: 1,
        mode: TransmissionMode::Drive,
    });
    let negative_id = world.body.add_arrow(ArrowSpec {
        from: contact,
        to: world.target,
        delay: 1,
        phase: 0,
        coupling: -1,
        resistance: 1,
        mode: TransmissionMode::Drive,
    });
    let stem = world.body.arrow_reference(stem_id);
    let positive = world.body.arrow_reference(positive_id);
    let negative = world.body.arrow_reference(negative_id);
    world.pulse(world.source, 0, root + 10);
    let modulator = world.add_modulation_to(root, 100, contact);
    world.pulse(modulator, 2, root + 101);
    let resistances = [
        arrow_resistance(&world, stem),
        arrow_resistance(&world, positive),
        arrow_resistance(&world, negative),
    ];
    let contact_resistance = world.body.cell_resistance(contact);
    world.advance_age(10);
    let live = (
        world
            .body
            .resolve_cell(world.body.cell_reference(contact))
            .is_some(),
        world.body.resolve_arrow(stem).is_some(),
        world.body.resolve_arrow(positive).is_some(),
        world.body.resolve_arrow(negative).is_some(),
    );
    let markers = vec![format!(
        "shared_resistances={resistances:?};contact={contact_resistance:?};live10={live:?}"
    )];
    let checks = vec![
        (
            "shared_compartment_alias_reproduced".into(),
            resistances == [Some(4), Some(4), Some(4)],
        ),
        (
            "shared_junction_unchanged".into(),
            contact_resistance == Some(1),
        ),
        (
            "shared_topology_retained".into(),
            live == (true, true, true, true),
        ),
        ("three_link_updates".into(), world.work.arrow_updates == 3),
        (
            "no_generated_alternatives".into(),
            world.work.cell_proposals == 0 && world.work.arrow_proposals == 0,
        ),
    ];
    world.finish(markers, checks)
}

fn mechanics_name(config: MechanicalConfig) -> &'static str {
    if config == MechanicalConfig::REFERENCE {
        "reference"
    } else if config == MechanicalConfig::PRODUCTION {
        "production"
    } else {
        "unknown"
    }
}

fn main() {
    let stage = Stage::parse(&env::args().nth(1).unwrap_or_else(|| "gate-e".to_string()));
    let output_dir = env::args()
        .nth(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("experiments/results/cv0e_j0_v1"));
    fs::create_dir_all(&output_dir).unwrap();
    let mut csv = String::from(
        "case,family,root,phase,mechanics,replay_equal,cross_mechanics_equal,checks_pass,failed,cell_proposals,arrow_proposals,arrow_updates,cell_deallocations,arrow_deallocations,physical_work,trace_len,final_tick,naturally_quiescent,body_hash,markers\n",
    );
    let mut cases = 0usize;
    let mut rows = 0usize;
    let mut clauses = 0usize;
    let mut passed_clauses = 0usize;
    let mut all_pass = true;
    let mut gates_a_d_pass = true;
    let mut gate_e_pass = true;
    let mut gates_f_i_pass = true;
    let mut maximum_work = 0u64;

    for root in ROOTS {
        for phase in PHASES {
            for &family in stage.families() {
                cases += 1;
                let reference = observe(family, root, phase, MechanicalConfig::REFERENCE);
                let reference_replay = observe(family, root, phase, MechanicalConfig::REFERENCE);
                let production = observe(family, root, phase, MechanicalConfig::PRODUCTION);
                let production_replay = observe(family, root, phase, MechanicalConfig::PRODUCTION);
                let cross_equal = reference == production;
                let family_pass = reference.passed() && production.passed() && cross_equal;
                match family {
                    Family::Symmetry
                    | Family::NoConsequence
                    | Family::BoundedCreation
                    | Family::OrphanReuse => gates_a_d_pass &= family_pass,
                    Family::AnchorOnly
                    | Family::PositiveConsequence
                    | Family::AnchorPermutation => gate_e_pass &= family_pass,
                    Family::NegativeConsequence
                    | Family::Permutation
                    | Family::NeitherUseful
                    | Family::BothUseful
                    | Family::SharedContact => gates_f_i_pass &= family_pass,
                }
                for (config, observation, replay) in [
                    (MechanicalConfig::REFERENCE, &reference, &reference_replay),
                    (
                        MechanicalConfig::PRODUCTION,
                        &production,
                        &production_replay,
                    ),
                ] {
                    rows += 1;
                    let replay_equal = observation == replay;
                    let check_count = observation.checks.len().saturating_add(3);
                    let row_pass = observation.passed()
                        && replay_equal
                        && cross_equal
                        && observation.naturally_quiescent;
                    clauses = clauses.saturating_add(check_count);
                    passed_clauses = passed_clauses.saturating_add(
                        observation.checks.iter().filter(|(_, pass)| *pass).count()
                            + usize::from(replay_equal)
                            + usize::from(cross_equal)
                            + usize::from(observation.naturally_quiescent),
                    );
                    all_pass &= row_pass;
                    maximum_work = maximum_work.max(observation.work.physical);
                    writeln!(
                        csv,
                        "{cases},{},{root},{phase},{},{replay_equal},{cross_equal},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        family.name(),
                        mechanics_name(config),
                        observation.passed(),
                        observation.failed_names(),
                        observation.work.cell_proposals,
                        observation.work.arrow_proposals,
                        observation.work.arrow_updates,
                        observation.work.cell_deallocations,
                        observation.work.arrow_deallocations,
                        observation.work.physical,
                        observation.trace.len(),
                        observation.final_tick,
                        observation.naturally_quiescent,
                        observation.body_hash,
                        observation.markers.join("|").replace(',', ";"),
                    )
                    .unwrap();
                }
            }
        }
    }

    let expected_cases =
        ROOTS.len() * usize::try_from(PHASES.end - PHASES.start).unwrap() * stage.families().len();
    let expected_rows = expected_cases.saturating_mul(2);
    assert_eq!(cases, expected_cases);
    assert_eq!(rows, expected_rows);
    let report = format!(
        "# CV0-E/J0 junction-lifetime matrix\n\n- stage: {stage:?}\n- cases: {cases}/{expected_cases}\n- rows: {rows}/{expected_rows}\n- clauses: {passed_clauses}/{clauses}\n- Gates A-D: {}\n- Gate E: {}\n- Gates F-I: {}\n- Gate J Reference/Production exact: {}\n- replay exact: {}\n- natural quiescence: {}\n- maximum PhysicalWork: {maximum_work}\n",
        if stage == Stage::GateE { "NOT RUN" } else if gates_a_d_pass { "PASS" } else { "FAIL" },
        if gate_e_pass { "PASS" } else { "FAIL" },
        if stage == Stage::GateE { "NOT RUN" } else if gates_f_i_pass { "PASS" } else { "FAIL" },
        reference_production_exact(&csv),
        replay_exact(&csv),
        quiescence_exact(&csv),
    );
    fs::write(output_dir.join("matrix.csv"), csv).unwrap();
    fs::write(output_dir.join("report.md"), report).unwrap();
    assert!(gate_e_pass, "CV0 Gate E failed");
    if stage == Stage::Full {
        assert!(gates_a_d_pass, "CV0 Gates A-D failed");
        assert!(gates_f_i_pass, "CV0 Gates F-I failed");
    }
    assert!(all_pass, "CV0 cumulative resumed matrix failed");
    match stage {
        Stage::GateE => println!("CV0E_J0_GATE_E_POSITIVE_V1"),
        Stage::Full => println!("CV0_J0_BOUNDED_LOCAL_CONTACT_GENESIS_POSITIVE_V1"),
    }
}

fn reference_production_exact(csv: &str) -> bool {
    !csv.lines()
        .skip(1)
        .any(|line| line.split(',').nth(6) != Some("true"))
}

fn replay_exact(csv: &str) -> bool {
    !csv.lines()
        .skip(1)
        .any(|line| line.split(',').nth(5) != Some("true"))
}

fn quiescence_exact(csv: &str) -> bool {
    !csv.lines()
        .skip(1)
        .any(|line| line.split(',').nth(17) != Some("true"))
}
