#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use truelearner_core::{
    ArenaId, ArrowId, ArrowRef, ArrowSpec, CellId, CellRef, CellSpec, ContentHash, LiveCheckpoint,
    MechanicalConfig, PhysicalEvent, PhysicalTransition, PlasticSubstrate, SpikeInput,
    TransmissionMode, Work,
};

const ROOTS: [u64; 2] = [8_100_000, 8_200_001];
const PHASES: std::ops::Range<i64> = 0..10;
const CEILING: u64 = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    LearnedNegative,
    NoModulation,
    IdentityPermutation,
    LocationPermutation,
    IrrelevantNegative,
    UsefulPositive,
    DisconnectedNegative,
    UntraversedNegative,
    FreshRecurrencePacking,
}

impl Family {
    const ALL: [Self; 9] = [
        Self::LearnedNegative,
        Self::NoModulation,
        Self::IdentityPermutation,
        Self::LocationPermutation,
        Self::IrrelevantNegative,
        Self::UsefulPositive,
        Self::DisconnectedNegative,
        Self::UntraversedNegative,
        Self::FreshRecurrencePacking,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::LearnedNegative => "learned_negative_stabilizes",
            Self::NoModulation => "no_modulation_no_selection",
            Self::IdentityPermutation => "candidate_identity_sign_order_permutation",
            Self::LocationPermutation => "inhibitory_location_permutation",
            Self::IrrelevantNegative => "irrelevant_negative_not_retained",
            Self::UsefulPositive => "useful_positive_selection_control",
            Self::DisconnectedNegative => "learned_negative_disconnected",
            Self::UntraversedNegative => "learned_negative_not_traversed",
            Self::FreshRecurrencePacking => "fresh_recurrence_packing",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WorkTotals {
    physical: u64,
    drive: u64,
    modulation: u64,
    updates: u64,
    proposals: u64,
    cell_proposals: u64,
    arrow_deallocations: u64,
    cell_deallocations: u64,
}

impl WorkTotals {
    fn add(&mut self, work: Work) {
        self.physical = self.physical.saturating_add(work.physical_total());
        self.drive = self.drive.saturating_add(work.drive_deliveries);
        self.modulation = self.modulation.saturating_add(work.modulatory_deliveries);
        self.updates = self.updates.saturating_add(work.local_return_updates);
        self.proposals = self
            .proposals
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

#[derive(Clone, Copy, Debug)]
struct Candidate {
    contact: CellId,
    contact_ref: CellRef,
    sign: i32,
    stem: ArrowRef,
    outgoing: ArrowRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ContinuationObservation {
    trace: Vec<String>,
    work: WorkTotals,
    final_tick: i64,
    body_hash: String,
    naturally_quiescent: bool,
    ceiling_reached: bool,
}

#[derive(Clone, Debug)]
struct Observation {
    markers: Vec<String>,
    trace: Vec<String>,
    raw_trace_hash: String,
    work: WorkTotals,
    final_tick: i64,
    body_hash: String,
    raw_body_hash: String,
    live_hash: String,
    pending_continuation: ContinuationObservation,
    admitted_continuation: ContinuationObservation,
    naturally_quiescent: bool,
    ceiling_reached: bool,
    checks: Vec<(String, bool)>,
}

impl PartialEq for Observation {
    fn eq(&self, other: &Self) -> bool {
        self.markers == other.markers
            && self.trace == other.trace
            && self.work == other.work
            && self.final_tick == other.final_tick
            && self.body_hash == other.body_hash
            && self.pending_continuation == other.pending_continuation
            && self.admitted_continuation == other.admitted_continuation
            && self.naturally_quiescent == other.naturally_quiescent
            && self.ceiling_reached == other.ceiling_reached
            && self.checks == other.checks
    }
}

impl Eq for Observation {}

impl Observation {
    fn passed(&self) -> bool {
        self.checks.iter().all(|(_, pass)| *pass)
    }

    fn failures(&self) -> String {
        self.checks
            .iter()
            .filter(|(_, pass)| !pass)
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join("|")
    }
}

struct World {
    body: PlasticSubstrate,
    mechanics: MechanicalConfig,
    origin: i64,
    trace: Vec<PhysicalTransition>,
    work: WorkTotals,
    source_b: CellId,
    target_a: CellId,
    anchor: CellId,
    anchor_links: [ArrowRef; 2],
    next_aux_physical: u64,
    next_modulator_physical: u64,
    next_probe_physical: u64,
    next_origin_physical: u64,
    next_aux_name: usize,
    next_modulator_name: usize,
    next_probe_name: usize,
    next_arrow_name: usize,
    next_origin_name: usize,
    cell_names: HashMap<CellId, String>,
    arrow_names: HashMap<ArrowId, String>,
    physical_names: HashMap<u64, String>,
}

impl World {
    fn new(
        root: u64,
        phase: i64,
        mechanics: MechanicalConfig,
        target_physical_offset: u64,
        position_offset: i32,
        anchor_first: bool,
    ) -> Self {
        let mut body = PlasticSubstrate::with_mechanics(ArenaId(root), 32, 64, mechanics);
        body.set_physical_tracing(true);
        body.advance_time(phase);
        let add_anchor = |body: &mut PlasticSubstrate, physical_offset: u64| {
            body.add_cell(CellSpec {
                physical_id: root + physical_offset,
                position: position_offset.saturating_add(1_000),
                region: 0,
                threshold: 100,
                resistance: 500,
            })
        };
        let early_anchor = anchor_first.then(|| add_anchor(&mut body, 901));
        let source_b = body.add_cell(CellSpec {
            physical_id: root + 1,
            position: position_offset,
            region: 0,
            threshold: 2,
            resistance: 500,
        });
        let target_a = body.add_cell(CellSpec {
            physical_id: root + target_physical_offset,
            position: position_offset.saturating_add(1),
            region: 0,
            threshold: 2,
            resistance: 500,
        });
        let anchor = early_anchor.unwrap_or_else(|| add_anchor(&mut body, 900));
        let anchor_to_b = body.add_arrow(ArrowSpec {
            from: anchor,
            to: source_b,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 500,
            mode: TransmissionMode::Drive,
        });
        let a_to_anchor = body.add_arrow(ArrowSpec {
            from: target_a,
            to: anchor,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 500,
            mode: TransmissionMode::Drive,
        });
        let anchor_links = [
            body.arrow_reference(anchor_to_b),
            body.arrow_reference(a_to_anchor),
        ];
        let mut cell_names = HashMap::new();
        cell_names.insert(source_b, "source_b".into());
        cell_names.insert(target_a, "target_a".into());
        cell_names.insert(anchor, "anchor".into());
        let mut arrow_names = HashMap::new();
        arrow_names.insert(anchor_to_b, "anchor_to_source_b".into());
        arrow_names.insert(a_to_anchor, "target_a_to_anchor".into());
        let mut physical_names = HashMap::new();
        physical_names.insert(root + 1, "source_b".into());
        physical_names.insert(root + target_physical_offset, "target_a".into());
        physical_names.insert(
            if anchor_first { root + 901 } else { root + 900 },
            "anchor".into(),
        );
        Self {
            body,
            mechanics,
            origin: phase,
            trace: Vec::new(),
            work: WorkTotals::default(),
            source_b,
            target_a,
            anchor,
            anchor_links,
            next_aux_physical: root + 10_000,
            next_modulator_physical: root + 30_000,
            next_probe_physical: root + 50_000,
            next_origin_physical: root + 70_000,
            next_aux_name: 0,
            next_modulator_name: 0,
            next_probe_name: 0,
            next_arrow_name: 0,
            next_origin_name: 0,
            cell_names,
            arrow_names,
            physical_names,
        }
    }

    fn register_cell(&mut self, cell: CellId, physical_id: u64, name: String) -> CellId {
        self.cell_names.insert(cell, name.clone());
        self.physical_names.insert(physical_id, name);
        cell
    }

    fn add_cell_with_physical(
        &mut self,
        physical_id: u64,
        position: i32,
        threshold: i32,
    ) -> CellId {
        self.body.add_cell(CellSpec {
            physical_id,
            position,
            region: 0,
            threshold,
            resistance: 500,
        })
    }

    fn add_aux_cell(&mut self, position: i32, threshold: i32) -> CellId {
        let physical_id = self.next_aux_physical;
        self.next_aux_physical = self.next_aux_physical.saturating_add(1);
        let name = format!("aux_{}", self.next_aux_name);
        self.next_aux_name = self.next_aux_name.saturating_add(1);
        let cell = self.add_cell_with_physical(physical_id, position, threshold);
        self.register_cell(cell, physical_id, name)
    }

    fn add_modulator_cell(&mut self, position: i32, threshold: i32) -> CellId {
        let physical_id = self.next_modulator_physical;
        self.next_modulator_physical = self.next_modulator_physical.saturating_add(1);
        let name = format!("modulator_{}", self.next_modulator_name);
        self.next_modulator_name = self.next_modulator_name.saturating_add(1);
        let cell = self.add_cell_with_physical(physical_id, position, threshold);
        self.register_cell(cell, physical_id, name)
    }

    fn add_probe_cell(&mut self, position: i32, threshold: i32) -> CellId {
        let physical_id = self.next_probe_physical;
        self.next_probe_physical = self.next_probe_physical.saturating_add(1);
        let name = format!("probe_{}", self.next_probe_name);
        self.next_probe_name = self.next_probe_name.saturating_add(1);
        let cell = self.add_cell_with_physical(physical_id, position, threshold);
        self.register_cell(cell, physical_id, name)
    }

    fn add_drive(
        &mut self,
        from: CellId,
        to: CellId,
        coupling: i32,
        delay: i64,
        phase: i32,
        resistance: u32,
    ) -> ArrowRef {
        let id = self.body.add_arrow(ArrowSpec {
            from,
            to,
            delay,
            phase,
            coupling,
            resistance,
            mode: TransmissionMode::Drive,
        });
        self.arrow_names
            .insert(id, format!("fixture_link_{}", self.next_arrow_name));
        self.next_arrow_name = self.next_arrow_name.saturating_add(1);
        self.body.arrow_reference(id)
    }

    fn add_anchor_for(&mut self, cell: CellId) -> ArrowRef {
        let anchor = self.anchor;
        self.add_drive(cell, anchor, 1, 1, 0, 500)
    }

    fn pulse_full(&mut self, target: CellId, age: i64, impulse: i32) {
        let origin_physical = self.next_origin_physical;
        self.next_origin_physical = self.next_origin_physical.saturating_add(1);
        self.physical_names.insert(
            origin_physical,
            format!("external_{}", self.next_origin_name),
        );
        self.next_origin_name = self.next_origin_name.saturating_add(1);
        let result = self.body.arrive(
            &[SpikeInput {
                arrival_tick: self.origin.saturating_add(age),
                phase: 0,
                origin_physical,
                target,
                impulse,
            }],
            i16::MAX,
        );
        self.trace.extend(result.physical_trace);
        self.work.add(result.work);
        assert!(result.naturally_quiescent);
    }

    fn advance_age(&mut self, age: i64) {
        let target = self.origin.saturating_add(age);
        while self.body.clock().tick < target {
            let result = self
                .body
                .advance_time_traced(self.body.clock().tick.saturating_add(1));
            self.trace.extend(result.physical_trace);
            self.work.add(result.work);
            assert!(result.naturally_quiescent);
        }
    }

    fn generate(&mut self) {
        self.pulse_full(self.source_b, 0, 2);
    }

    fn candidates_to(&mut self, target: CellId) -> [Candidate; 2] {
        let body = self.body.arena_body(1);
        let mut candidates = body
            .cells
            .iter()
            .filter_map(|cell| {
                let stem = body
                    .arrows
                    .iter()
                    .find(|arrow| arrow.from.id == self.source_b && arrow.to.id == cell.id)?;
                let outgoing = body
                    .arrows
                    .iter()
                    .find(|arrow| arrow.from.id == cell.id && arrow.to.id == target)?;
                (outgoing.coupling.abs() == 1).then_some(Candidate {
                    contact: cell.id,
                    contact_ref: self.body.cell_reference(cell.id),
                    sign: outgoing.coupling,
                    stem: self.body.arrow_reference(stem.id),
                    outgoing: self.body.arrow_reference(outgoing.id),
                })
            })
            .collect::<Vec<_>>();
        candidates.sort_by_key(|candidate| candidate.sign);
        let target_name = self
            .cell_names
            .get(&target)
            .cloned()
            .expect("candidate target must have a logical name");
        for candidate in &candidates {
            let sign_name = if candidate.sign < 0 {
                "negative"
            } else {
                "positive"
            };
            self.cell_names.insert(
                candidate.contact,
                format!("contact_{target_name}_{sign_name}"),
            );
            self.arrow_names
                .insert(candidate.stem.id, format!("stem_{target_name}_{sign_name}"));
            self.arrow_names.insert(
                candidate.outgoing.id,
                format!("outgoing_{target_name}_{sign_name}"),
            );
            if let Some(cell) = body.cells.iter().find(|cell| cell.id == candidate.contact) {
                self.physical_names.insert(
                    cell.physical_id,
                    format!("contact_{target_name}_{sign_name}"),
                );
            }
        }
        candidates
            .try_into()
            .expect("exactly two signed candidates")
    }

    fn modulate(&mut self, contact: CellId, age: i64) {
        let modulator = self.add_modulator_cell(10_000, 1);
        let id = self.body.add_arrow(ArrowSpec {
            from: modulator,
            to: contact,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 500,
            mode: TransmissionMode::Modulatory,
        });
        self.arrow_names.insert(
            id,
            format!(
                "modulatory_link_{}",
                self.next_modulator_name.saturating_sub(1)
            ),
        );
        let _ = self.body.arrow_reference(id);
        self.pulse_full(modulator, age, 1);
    }

    fn add_recurrence(&mut self, a: CellId, b: CellId, position_base: i32) {
        let u = self.add_probe_cell(position_base.saturating_add(100), 1);
        let w = self.add_probe_cell(position_base.saturating_add(200), 1);
        self.add_drive(a, u, 1, 1, 0, 500);
        self.add_drive(u, b, 2, 1, 0, 500);
        self.add_drive(b, w, 1, 1, 0, 500);
        self.add_drive(w, a, 2, 1, 1, 500);
    }

    fn probe_observed(
        &mut self,
        target: CellId,
        age: i64,
    ) -> (Vec<PhysicalTransition>, bool, bool) {
        let origin_physical = self.next_origin_physical;
        self.next_origin_physical = self.next_origin_physical.saturating_add(1);
        self.body.enter(SpikeInput {
            arrival_tick: self.origin.saturating_add(age),
            phase: 0,
            origin_physical,
            target,
            impulse: 2,
        });
        let observed = self.body.propagate_with_observation_ceiling(CEILING);
        let trace = observed.run.physical_trace.clone();
        let quiescent = observed.run.naturally_quiescent;
        let ceiling = observed.observation_ceiling_reached;
        self.trace.extend(observed.run.physical_trace);
        self.work.add(observed.run.work);
        (trace, quiescent, ceiling)
    }

    fn arrow_resistance(&self, reference: ArrowRef) -> Option<u32> {
        self.body
            .arena_body(1)
            .arrows
            .iter()
            .find(|arrow| arrow.id == reference.id)
            .map(|arrow| arrow.resistance)
    }

    fn relation_live(&self, candidate: Candidate) -> bool {
        self.body.resolve_cell(candidate.contact_ref).is_some()
            && self.body.resolve_arrow(candidate.stem).is_some()
            && self.body.resolve_arrow(candidate.outgoing).is_some()
    }

    fn finish(
        self,
        markers: Vec<String>,
        naturally_quiescent: bool,
        ceiling_reached: bool,
        checks: Vec<(String, bool)>,
    ) -> Observation {
        let raw_body_hash =
            ContentHash::of(&self.body.canonical_body_bytes(1).unwrap()).to_string();
        let body_hash = normalized_body_hash(&self.body, &self.cell_names, &self.arrow_names);
        let checkpoint = self.body.live_checkpoint(1).unwrap();
        let live_hash = ContentHash::of(&checkpoint.canonical_bytes().unwrap()).to_string();
        let pending_continuation = continuation_observation(
            checkpoint.clone(),
            self.mechanics,
            false,
            self.anchor,
            self.next_origin_physical.saturating_add(1_000),
            &self.cell_names,
            &self.arrow_names,
            &self.physical_names,
        );
        let admitted_continuation = continuation_observation(
            checkpoint,
            self.mechanics,
            true,
            self.anchor,
            self.next_origin_physical.saturating_add(2_000),
            &self.cell_names,
            &self.arrow_names,
            &self.physical_names,
        );
        Observation {
            markers,
            trace: normalize_trace(
                &self.trace,
                &self.cell_names,
                &self.arrow_names,
                &self.physical_names,
            ),
            raw_trace_hash: ContentHash::of(format!("{:?}", self.trace).as_bytes()).to_string(),
            work: self.work,
            final_tick: self.body.clock().tick,
            body_hash,
            raw_body_hash,
            live_hash,
            pending_continuation,
            admitted_continuation,
            naturally_quiescent,
            ceiling_reached,
            checks,
        }
    }
}

fn cell_name(names: &HashMap<CellId, String>, id: CellId) -> &str {
    names
        .get(&id)
        .map(String::as_str)
        .unwrap_or("unmapped_cell")
}

fn arrow_name(names: &HashMap<ArrowId, String>, id: ArrowId) -> &str {
    names
        .get(&id)
        .map(String::as_str)
        .unwrap_or("unmapped_arrow")
}

fn physical_name(names: &HashMap<u64, String>, id: u64) -> &str {
    names
        .get(&id)
        .map(String::as_str)
        .unwrap_or("unmapped_physical")
}

fn logical_event(
    event: &PhysicalEvent,
    cell_names: &HashMap<CellId, String>,
    arrow_names: &HashMap<ArrowId, String>,
    physical_names: &HashMap<u64, String>,
) -> String {
    match event {
        PhysicalEvent::DriveIncidence {
            target,
            arrivals,
            impulse,
            causal_wave,
        } => format!(
            "INCIDENCE:{}:arrivals={arrivals}:impulse={impulse}:wave={causal_wave}",
            cell_name(cell_names, *target)
        ),
        PhysicalEvent::ModulatoryIncidence {
            target,
            arrivals,
            impulse,
            causal_wave,
        } => format!(
            "MODULATORY_INCIDENCE:{}:arrivals={arrivals}:impulse={impulse}:wave={causal_wave}",
            cell_name(cell_names, *target)
        ),
        PhysicalEvent::Deliver {
            mode,
            target,
            impulse,
        } => format!(
            "DELIVER:{mode:?}:{}:{impulse}",
            cell_name(cell_names, *target)
        ),
        PhysicalEvent::Fire { cell } => format!("FIRE:{}", cell_name(cell_names, *cell)),
        PhysicalEvent::Resistance {
            arrow,
            before,
            after,
        } => format!(
            "RESISTANCE:{}:{before}:{after}",
            arrow_name(arrow_names, *arrow)
        ),
        PhysicalEvent::Deallocate { arrow } => {
            format!("DEALLOCATE:{}", arrow_name(arrow_names, *arrow))
        }
        PhysicalEvent::CellDeallocate {
            cell,
            before_generation,
            after_generation,
        } => format!(
            "CELL_DEALLOCATE:{}:{}:{}",
            cell_name(cell_names, *cell),
            before_generation.0,
            after_generation.0
        ),
        PhysicalEvent::CellProposal {
            cell,
            source,
            target,
        } => format!(
            "CELL_PROPOSAL:{}:{}:{}",
            cell_name(cell_names, *cell),
            cell_name(cell_names, *source),
            cell_name(cell_names, *target)
        ),
        PhysicalEvent::Proposal { arrow, from, to } => format!(
            "PROPOSAL:{}:{}:{}",
            arrow_name(arrow_names, *arrow),
            cell_name(cell_names, *from),
            cell_name(cell_names, *to)
        ),
        PhysicalEvent::Crossing(crossing) => format!(
            "CROSSING:{}:{}:{}:{}:{}",
            physical_name(physical_names, crossing.from_physical),
            physical_name(physical_names, crossing.to_physical),
            crossing.from_region,
            crossing.to_region,
            crossing.impulse
        ),
        PhysicalEvent::QualifiedLocalTraversal { arrow } => {
            format!("QLP:{}", arrow_name(arrow_names, *arrow))
        }
    }
}

fn normalize_trace(
    trace: &[PhysicalTransition],
    cell_names: &HashMap<CellId, String>,
    arrow_names: &HashMap<ArrowId, String>,
    physical_names: &HashMap<u64, String>,
) -> Vec<String> {
    let mut moments: BTreeMap<(i64, i32), Vec<String>> = BTreeMap::new();
    for transition in trace {
        moments
            .entry((transition.tick, transition.phase))
            .or_default()
            .push(logical_event(
                &transition.event,
                cell_names,
                arrow_names,
                physical_names,
            ));
    }
    moments
        .into_iter()
        .map(|((tick, phase), mut events)| {
            events.sort();
            format!("{tick}:{phase}:[{}]", events.join("|"))
        })
        .collect()
}

fn normalized_body_hash(
    body: &PlasticSubstrate,
    cell_names: &HashMap<CellId, String>,
    arrow_names: &HashMap<ArrowId, String>,
) -> String {
    let arena = body.arena_body(1);
    let mut cells = arena
        .cells
        .iter()
        .map(|cell| {
            format!(
                "CELL:{}:position={}:region={}:threshold={}:resistance={}:generation={}:live={}",
                cell_name(cell_names, cell.id),
                cell.position,
                cell.region,
                cell.threshold,
                cell.resistance,
                cell.generation.0,
                cell.live
            )
        })
        .collect::<Vec<_>>();
    let mut arrows = arena
        .arrows
        .iter()
        .map(|arrow| {
            format!(
                "ARROW:{}:{}:{}:delay={}:phase={}:coupling={}:resistance={}:mode={}:generation={}:live={}",
                arrow_name(arrow_names, arrow.id),
                cell_name(cell_names, arrow.from.id),
                cell_name(cell_names, arrow.to.id),
                arrow.delay,
                arrow.phase,
                arrow.coupling,
                arrow.resistance,
                arrow.transmission_mode,
                arrow.generation.0,
                arrow.live
            )
        })
        .collect::<Vec<_>>();
    cells.sort();
    arrows.sort();
    cells.extend(arrows);
    ContentHash::of(cells.join("\n").as_bytes()).to_string()
}

#[allow(clippy::too_many_arguments)]
fn continuation_observation(
    checkpoint: LiveCheckpoint,
    mechanics: MechanicalConfig,
    admit_future_input: bool,
    anchor: CellId,
    origin_physical: u64,
    cell_names: &HashMap<CellId, String>,
    arrow_names: &HashMap<ArrowId, String>,
    physical_names: &HashMap<u64, String>,
) -> ContinuationObservation {
    let mut body =
        PlasticSubstrate::from_live_checkpoint_with_mechanics(checkpoint, mechanics).unwrap();
    body.set_physical_tracing(true);
    let mut continuation_physical_names = physical_names.clone();
    if admit_future_input {
        continuation_physical_names.insert(origin_physical, "checkpoint_probe".into());
        body.enter(SpikeInput {
            arrival_tick: body.clock().tick.saturating_add(1),
            phase: 0,
            origin_physical,
            target: anchor,
            impulse: 1,
        });
    }
    let observed = body.propagate_with_observation_ceiling(CEILING);
    let mut work = WorkTotals::default();
    work.add(observed.run.work);
    ContinuationObservation {
        trace: normalize_trace(
            &observed.run.physical_trace,
            cell_names,
            arrow_names,
            &continuation_physical_names,
        ),
        work,
        final_tick: body.clock().tick,
        body_hash: normalized_body_hash(&body, cell_names, arrow_names),
        naturally_quiescent: observed.run.naturally_quiescent,
        ceiling_reached: observed.observation_ceiling_reached,
    }
}

fn fire_count(trace: &[PhysicalTransition], cell: CellId) -> usize {
    trace
        .iter()
        .filter(|transition| {
            matches!(transition.event, PhysicalEvent::Fire { cell: fired } if fired == cell)
        })
        .count()
}

fn positive_drive_incidence_count(trace: &[PhysicalTransition], cell: CellId) -> usize {
    trace
        .iter()
        .filter(|transition| {
            matches!(
                transition.event,
                PhysicalEvent::DriveIncidence {
                    target,
                    impulse,
                    ..
                } if target == cell && impulse > 0
            )
        })
        .count()
}

fn recurrent_activity_continues(
    trace: &[PhysicalTransition],
    first: CellId,
    second: CellId,
    quiescent: bool,
) -> bool {
    !quiescent && fire_count(trace, first) > 1 && fire_count(trace, second) > 1
}

fn base_world(
    root: u64,
    phase: i64,
    mechanics: MechanicalConfig,
    identity_permuted: bool,
    position_offset: i32,
) -> World {
    World::new(
        root,
        phase,
        mechanics,
        if identity_permuted { 3 } else { 2 },
        position_offset,
        identity_permuted,
    )
}

fn train_selected(
    world: &mut World,
    target: CellId,
    sign: i32,
    consequence: bool,
) -> (Candidate, Candidate, [Option<u32>; 2], bool) {
    world.generate();
    let training_target_silent = fire_count(&world.trace, target) == 0;
    let [negative, positive] = world.candidates_to(target);
    let (selected, unsupported) = if sign < 0 {
        (negative, positive)
    } else {
        (positive, negative)
    };
    if consequence {
        world.modulate(selected.contact, 2);
    }
    let anchor_resistance = [
        world.arrow_resistance(world.anchor_links[0]),
        world.arrow_resistance(world.anchor_links[1]),
    ];
    world.advance_age(10);
    (
        selected,
        unsupported,
        anchor_resistance,
        training_target_silent,
    )
}

fn observe(family: Family, root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    match family {
        Family::LearnedNegative => {
            observe_learned_negative(root, phase, mechanics, false, 0, false)
        }
        Family::NoModulation => observe_no_modulation(root, phase, mechanics),
        Family::IdentityPermutation => {
            observe_learned_negative(root, phase, mechanics, true, 0, false)
        }
        Family::LocationPermutation => {
            observe_learned_negative(root, phase, mechanics, false, 200, false)
        }
        Family::IrrelevantNegative => observe_irrelevant(root, phase, mechanics),
        Family::UsefulPositive => observe_useful_positive(root, phase, mechanics),
        Family::DisconnectedNegative => observe_disconnected(root, phase, mechanics),
        Family::UntraversedNegative => observe_untraversed(root, phase, mechanics),
        Family::FreshRecurrencePacking => {
            observe_learned_negative(root, phase, mechanics, false, 0, true)
        }
    }
}

fn observe_learned_negative(
    root: u64,
    phase: i64,
    mechanics: MechanicalConfig,
    identity_permuted: bool,
    position_offset: i32,
    fresh_packing: bool,
) -> Observation {
    let mut world = base_world(root, phase, mechanics, identity_permuted, position_offset);
    let target = world.target_a;
    let (selected, unsupported, anchors_after, training_target_silent) =
        train_selected(&mut world, target, -1, true);
    let selected_r = [
        world.arrow_resistance(selected.stem),
        world.arrow_resistance(selected.outgoing),
    ];
    let unsupported_live = world.relation_live(unsupported);
    if fresh_packing {
        let d1 = world.add_probe_cell(position_offset.saturating_add(700), 100);
        let d2 = world.add_probe_cell(position_offset.saturating_add(701), 100);
        let anchor = world.anchor;
        world.add_drive(anchor, d1, 1, 1, 0, 500);
        world.add_drive(d2, anchor, 1, 1, 0, 500);
    }
    let a = world.target_a;
    let b = world.source_b;
    world.add_recurrence(a, b, position_offset);
    let (probe, quiescent, ceiling) = world.probe_observed(a, 10);
    let a_fires = fire_count(&probe, world.target_a);
    let b_fires = fire_count(&probe, world.source_b);
    let selected_contact_fires = fire_count(&probe, selected.contact);
    let markers = vec![format!(
        "selected_sign={};r={selected_r:?};unsupported_live={unsupported_live};anchors={anchors_after:?};fires=a{a_fires}/b{b_fires}/contact{selected_contact_fires};fresh={fresh_packing}",
        selected.sign
    )];
    let checks = vec![
        ("negative_selected".into(), selected.sign == -1),
        (
            "training_target_remained_subthreshold".into(),
            training_target_silent,
        ),
        (
            "selected_relation_consolidated".into(),
            selected_r == [Some(4), Some(4)],
        ),
        ("unsupported_relation_removed".into(), !unsupported_live),
        (
            "anchor_credit_absent".into(),
            anchors_after == [Some(500), Some(500)],
        ),
        (
            "intended_cycle_executes_once".into(),
            a_fires == 1 && b_fires == 1,
        ),
        (
            "learned_negative_traverses".into(),
            selected_contact_fires == 1,
        ),
        ("recurrence_settles".into(), quiescent && !ceiling),
        ("only_two_training_updates".into(), world.work.updates == 2),
    ];
    world.finish(markers, quiescent, ceiling, checks)
}

fn observe_no_modulation(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = base_world(root, phase, mechanics, false, 0);
    let target = world.target_a;
    let (selected, unsupported, anchors_after, training_target_silent) =
        train_selected(&mut world, target, -1, false);
    let candidates_gone = !world.relation_live(selected) && !world.relation_live(unsupported);
    let a = world.target_a;
    let b = world.source_b;
    world.add_recurrence(a, b, 0);
    let (probe, quiescent, ceiling) = world.probe_observed(a, 10);
    let markers = vec![format!(
        "candidates_gone={candidates_gone};anchors={anchors_after:?};fires={}/{};ceiling={ceiling}",
        fire_count(&probe, world.target_a),
        fire_count(&probe, world.source_b)
    )];
    let checks = vec![
        ("no_modulation_no_updates".into(), world.work.updates == 0),
        (
            "training_target_remained_subthreshold".into(),
            training_target_silent,
        ),
        ("unsupported_candidates_removed".into(), candidates_gone),
        (
            "anchor_credit_absent".into(),
            anchors_after == [Some(500), Some(500)],
        ),
        (
            "uninhibited_recurrence_persists".into(),
            recurrent_activity_continues(&probe, a, b, quiescent),
        ),
    ];
    world.finish(markers, quiescent, ceiling, checks)
}

fn observe_irrelevant(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = base_world(root, phase, mechanics, false, 0);
    let irrelevant_target = world.add_aux_cell(-1, 100);
    world.add_anchor_for(irrelevant_target);
    world.generate();
    let training_target_silent = fire_count(&world.trace, world.target_a) == 0
        && fire_count(&world.trace, irrelevant_target) == 0;
    let [useful_negative, useful_positive] = world.candidates_to(world.target_a);
    let [irrelevant_negative, irrelevant_positive] = world.candidates_to(irrelevant_target);
    world.modulate(useful_negative.contact, 2);
    world.advance_age(10);
    let useful_live = world.relation_live(useful_negative);
    let others_gone = !world.relation_live(useful_positive)
        && !world.relation_live(irrelevant_negative)
        && !world.relation_live(irrelevant_positive);
    let a = world.target_a;
    let b = world.source_b;
    world.add_recurrence(a, b, 0);
    let (probe, quiescent, ceiling) = world.probe_observed(a, 10);
    let markers = vec![format!(
        "useful_live={useful_live};others_gone={others_gone};fires={}/{}",
        fire_count(&probe, world.target_a),
        fire_count(&probe, world.source_b)
    )];
    let checks = vec![
        (
            "training_targets_remained_subthreshold".into(),
            training_target_silent,
        ),
        ("useful_negative_retained".into(), useful_live),
        ("irrelevant_candidates_removed".into(), others_gone),
        ("only_useful_links_updated".into(), world.work.updates == 2),
        ("local_recurrence_settles".into(), quiescent && !ceiling),
    ];
    world.finish(markers, quiescent, ceiling, checks)
}

fn observe_useful_positive(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = base_world(root, phase, mechanics, false, 0);
    let target = world.target_a;
    let (selected, unsupported, anchors_after, training_target_silent) =
        train_selected(&mut world, target, 1, true);
    let selected_live = world.relation_live(selected);
    let unsupported_live = world.relation_live(unsupported);
    let contact_before = fire_count(&world.trace, selected.contact);
    let positive_incidences_before =
        positive_drive_incidence_count(&world.trace, world.target_a);
    let source = world.source_b;
    world.pulse_full(source, 10, 2);
    let contact_after = fire_count(&world.trace, selected.contact);
    let positive_incidences_after =
        positive_drive_incidence_count(&world.trace, world.target_a);
    let markers = vec![format!(
        "selected_sign={};live={selected_live}/{unsupported_live};contact_fires={contact_before}->{contact_after};positive_incidences={positive_incidences_before}->{positive_incidences_after};anchors={anchors_after:?}",
        selected.sign
    )];
    let checks = vec![
        ("positive_selected".into(), selected.sign == 1),
        (
            "training_target_remained_subthreshold".into(),
            training_target_silent,
        ),
        ("positive_retained".into(), selected_live),
        ("negative_removed".into(), !unsupported_live),
        (
            "positive_relation_reexecutes".into(),
            contact_after == contact_before.saturating_add(1)
                && positive_incidences_after == positive_incidences_before.saturating_add(1),
        ),
        (
            "anchor_credit_absent".into(),
            anchors_after == [Some(500), Some(500)],
        ),
    ];
    world.finish(markers, true, false, checks)
}

fn observe_disconnected(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = base_world(root, phase, mechanics, false, 0);
    let disconnected = world.add_aux_cell(-1, 2);
    world.add_anchor_for(disconnected);
    world.generate();
    let training_target_silent = fire_count(&world.trace, world.target_a) == 0
        && fire_count(&world.trace, disconnected) == 0;
    let [negative_to_a, positive_to_a] = world.candidates_to(world.target_a);
    let [negative_elsewhere, positive_elsewhere] = world.candidates_to(disconnected);
    world.modulate(negative_elsewhere.contact, 2);
    world.advance_age(10);
    let disconnected_live = world.relation_live(negative_elsewhere);
    let a_candidates_gone =
        !world.relation_live(negative_to_a) && !world.relation_live(positive_to_a);
    let positive_elsewhere_gone = !world.relation_live(positive_elsewhere);
    let a = world.target_a;
    let b = world.source_b;
    world.add_recurrence(a, b, 0);
    let (probe, quiescent, ceiling) = world.probe_observed(a, 10);
    let checks = vec![
        (
            "training_targets_remained_subthreshold".into(),
            training_target_silent,
        ),
        ("disconnected_negative_learned".into(), disconnected_live),
        ("target_candidates_absent".into(), a_candidates_gone),
        ("other_sign_absent".into(), positive_elsewhere_gone),
        (
            "disconnected_inhibition_cannot_settle".into(),
            recurrent_activity_continues(&probe, a, b, quiescent),
        ),
    ];
    world.finish(
        vec![format!(
            "disconnected={disconnected_live};ceiling={ceiling}"
        )],
        quiescent,
        ceiling,
        checks,
    )
}

fn observe_untraversed(root: u64, phase: i64, mechanics: MechanicalConfig) -> Observation {
    let mut world = base_world(root, phase, mechanics, false, 0);
    let target = world.target_a;
    let (selected, _, _, training_target_silent) = train_selected(&mut world, target, -1, true);
    let q = world.add_probe_cell(2, 2);
    let a = world.target_a;
    world.add_recurrence(a, q, 300);
    let (probe, quiescent, ceiling) = world.probe_observed(a, 10);
    let checks = vec![
        (
            "learned_negative_present".into(),
            world.relation_live(selected),
        ),
        (
            "training_target_remained_subthreshold".into(),
            training_target_silent,
        ),
        (
            "learned_contact_not_traversed".into(),
            fire_count(&probe, selected.contact) == 0,
        ),
        (
            "untraversed_inhibition_cannot_settle".into(),
            recurrent_activity_continues(&probe, a, q, quiescent),
        ),
    ];
    world.finish(
        vec![format!(
            "contact_fires={}",
            fire_count(&probe, selected.contact)
        )],
        quiescent,
        ceiling,
        checks,
    )
}

fn mechanics_name(config: MechanicalConfig) -> &'static str {
    if config == MechanicalConfig::REFERENCE {
        "reference"
    } else {
        "production"
    }
}

fn main() {
    let output_dir = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("experiments/results/rs2_learned_inhibitory_topology_v6")
    });
    fs::create_dir_all(&output_dir).unwrap();
    let mut csv = String::from(
        "case,family,root,phase,mechanics,replay_equal,cross_equal,renaming_equal,checks_pass,failed,quiescent,ceiling,physical_work,updates,proposals,cell_proposals,arrow_deallocations,cell_deallocations,normalized_trace_len,final_tick,normalized_trace_hash,raw_trace_hash,normalized_body_hash,raw_body_hash,live_hash,pending_continuation_hash,admitted_continuation_hash,markers\n",
    );
    let mut cases = 0usize;
    let mut rows = 0usize;
    let mut clauses = 0usize;
    let mut passed = 0usize;
    let mut all_pass = true;
    let mut maximum_work = 0u64;
    for root in ROOTS {
        for phase in PHASES {
            let mut reference_identity_baseline = None;
            let mut production_identity_baseline = None;
            for family in Family::ALL {
                cases += 1;
                let reference = observe(family, root, phase, MechanicalConfig::REFERENCE);
                let reference_replay = observe(family, root, phase, MechanicalConfig::REFERENCE);
                let production = observe(family, root, phase, MechanicalConfig::PRODUCTION);
                let production_replay = observe(family, root, phase, MechanicalConfig::PRODUCTION);
                let cross_equal = reference == production;
                let renaming_equal = if family == Family::IdentityPermutation {
                    reference_identity_baseline
                        .as_ref()
                        .is_some_and(|baseline| baseline == &reference)
                        && production_identity_baseline
                            .as_ref()
                            .is_some_and(|baseline| baseline == &production)
                } else {
                    true
                };
                if family == Family::LearnedNegative {
                    reference_identity_baseline = Some(reference.clone());
                    production_identity_baseline = Some(production.clone());
                }
                for (mechanics, observation, replay) in [
                    (MechanicalConfig::REFERENCE, &reference, &reference_replay),
                    (
                        MechanicalConfig::PRODUCTION,
                        &production,
                        &production_replay,
                    ),
                ] {
                    rows += 1;
                    let replay_equal = observation == replay;
                    let row_pass =
                        observation.passed() && replay_equal && cross_equal && renaming_equal;
                    let row_clauses = observation.checks.len().saturating_add(3);
                    clauses = clauses.saturating_add(row_clauses);
                    passed = passed.saturating_add(
                        observation.checks.iter().filter(|(_, pass)| *pass).count()
                            + usize::from(replay_equal)
                            + usize::from(cross_equal)
                            + usize::from(renaming_equal),
                    );
                    all_pass &= row_pass;
                    maximum_work = maximum_work.max(observation.work.physical);
                    let normalized_trace_hash =
                        ContentHash::of(observation.trace.join("\n").as_bytes()).to_string();
                    let pending_continuation_hash = ContentHash::of(
                        format!("{:?}", observation.pending_continuation).as_bytes(),
                    )
                    .to_string();
                    let admitted_continuation_hash = ContentHash::of(
                        format!("{:?}", observation.admitted_continuation).as_bytes(),
                    )
                    .to_string();
                    writeln!(
                        csv,
                        "{cases},{},{root},{phase},{},{replay_equal},{cross_equal},{renaming_equal},{},{},{},{},{},{},{},{},{},{},{},{},{normalized_trace_hash},{},{},{},{},{pending_continuation_hash},{admitted_continuation_hash},{}",
                        family.name(),
                        mechanics_name(mechanics),
                        observation.passed(),
                        observation.failures(),
                        observation.naturally_quiescent,
                        observation.ceiling_reached,
                        observation.work.physical,
                        observation.work.updates,
                        observation.work.proposals,
                        observation.work.cell_proposals,
                        observation.work.arrow_deallocations,
                        observation.work.cell_deallocations,
                        observation.trace.len(),
                        observation.final_tick,
                        observation.raw_trace_hash,
                        observation.body_hash,
                        observation.raw_body_hash,
                        observation.live_hash,
                        observation.markers.join("|").replace(',', ";"),
                    )
                    .unwrap();
                }
            }
        }
    }
    let expected_cases =
        ROOTS.len() * usize::try_from(PHASES.end - PHASES.start).unwrap() * Family::ALL.len();
    let expected_rows = expected_cases.saturating_mul(2);
    assert_eq!(cases, expected_cases);
    assert_eq!(rows, expected_rows);
    let report = format!(
        "# RS2 learned inhibitory topology v6\n\n- cases: {cases}/{expected_cases}\n- rows: {rows}/{expected_rows}\n- clauses: {passed}/{clauses}\n- Reference/Production wave-normalized exact: {}\n- identity renaming exact: {}\n- replay exact: {}\n- meaningful checkpoint continuation exact: {}\n- maximum PhysicalWork: {maximum_work}\n",
        csv.lines().skip(1).all(|line| line.split(',').nth(6) == Some("true")),
        csv.lines().skip(1).all(|line| line.split(',').nth(7) == Some("true")),
        csv.lines().skip(1).all(|line| line.split(',').nth(5) == Some("true")),
        csv.lines().skip(1).all(|line| {
            let fields = line.split(',').collect::<Vec<_>>();
            fields.get(25).is_some() && fields.get(26).is_some()
        }),
    );
    fs::write(output_dir.join("matrix.csv"), csv).unwrap();
    fs::write(output_dir.join("report.md"), report).unwrap();
    assert!(all_pass, "RS2 v6 matrix failed");
    println!("RS2_LEARNED_INHIBITORY_TOPOLOGY_POSITIVE_V6");
}
