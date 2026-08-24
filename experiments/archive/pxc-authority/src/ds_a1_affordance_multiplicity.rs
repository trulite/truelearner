//! DS-A1 development-only affordance-multiplicity enabling gate.
//!
//! The primary path uses one actual frozen E0 event. Local relation traversal
//! installs live episode adjacency before an opaque bridge. Normalized effects
//! are evaluator-only post-bridge evidence and never alter bridge membership.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::mem::size_of;

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-a1-affordance-multiplicity-v1";
pub const EXACT_PARENT: &str = "f4aeae4ae2f1832bc469621d79f7bb5b3fd6d1d0";
pub const PROTOCOL_COMMIT: &str = "08797f85b67ddfc69e6068e6bc71321ed0927a3b";
pub const PROTOCOL_AMENDMENT: &str = "0cf66a6bf1957fc1d9e6b22d7541623e3405e354";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_DS_E0_SHA256: &str =
    "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615";
pub const FROZEN_DS_A0_SHA256: &str =
    "3eb802f394a225a4ad7f0938b4a672723da2c1303ff95e805423de8161057527";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const PRIOR_COMPOSITION_SHA256: &str =
    "3e3f5227fa570e52043c8eb4d3bdbe8242c74f0fa8fe8394693b76bde420af8b";

const SUPPORT_EPISODES: u16 = 3;
const SUPPORT_PRESENTATIONS: usize = 12;
const STAGE_NAMES: [&str; 9] = [
    "0. exact lineage and frozen hashes",
    "1. actual E0 event/export exists from one target episode",
    "2. local semantics-blind variation candidates arise without supplied menu/count",
    "3. repeated support consolidates >=2 anonymous continuation templates",
    "4. target episode installs >=2 executable roots before bridge",
    "5. structural dedup leaves >=2 unique E0-relative live-adjacency continuations",
    "6. independent root-spike executions yield >=2 distinct normalized effects",
    "7. one-to-one opaque bridge exposes exactly those roots without rank",
    "8. transfer/leak/lifetime/negative controls",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ExportPulse {
    occurrence: u32,
    tick: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct E0EpisodeExport {
    pulses: Vec<ExportPulse>,
    observed_propagation: Vec<[u32; 2]>,
    members: [u32; 3],
    relative_temporal: [i8; 9],
    relative_propagation: [i8; 9],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProvenanceAudit {
    pub support_exports: usize,
    pub actual_target_events: u64,
    pub raw_pulses: usize,
    pub copied_pulses: usize,
    pub raw_relations: usize,
    pub copied_relations: usize,
    pub copied_members: usize,
    pub copied_temporal_fields: usize,
    pub copied_propagation_fields: usize,
    pub exact_field_copy: bool,
    pub fresh_disjoint: bool,
    pub frozen_e0_work: u64,
    pub frozen_e0_persistent_bytes: usize,
    pub frozen_e0_temporary_bytes: usize,
}

macro_rules! e0_a1_access {
    () => {
        pub(super) struct A1Bundle {
            pub(super) support: Vec<super::E0EpisodeExport>,
            pub(super) target: super::E0EpisodeExport,
            pub(super) provenance: super::ProvenanceAudit,
        }

        fn export_episode(raw: &RawActivity, event: &EventRelations) -> super::E0EpisodeExport {
            super::E0EpisodeExport {
                pulses: raw
                    .spikes
                    .iter()
                    .map(|spike| super::ExportPulse {
                        occurrence: spike.occurrence.0,
                        tick: spike.local_tick,
                    })
                    .collect(),
                observed_propagation: raw
                    .propagation
                    .iter()
                    .map(|edge| [edge.from.0, edge.to.0])
                    .collect(),
                members: event.members.map(|member| member.0),
                relative_temporal: event.temporal,
                relative_propagation: event.propagation,
            }
        }

        fn export_matches(
            export: &super::E0EpisodeExport,
            raw: &RawActivity,
            event: &EventRelations,
        ) -> bool {
            export.pulses.len() == raw.spikes.len()
                && export.pulses.iter().zip(&raw.spikes).all(|(copy, origin)| {
                    copy.occurrence == origin.occurrence.0 && copy.tick == origin.local_tick
                })
                && export.observed_propagation.len() == raw.propagation.len()
                && export
                    .observed_propagation
                    .iter()
                    .zip(&raw.propagation)
                    .all(|(copy, origin)| *copy == [origin.from.0, origin.to.0])
                && export.members == event.members.map(|member| member.0)
                && export.relative_temporal == event.temporal
                && export.relative_propagation == event.propagation
        }

        pub(super) fn a1_bundle(seed: u64, acquisition: usize) -> Option<A1Bundle> {
            let (mut formation, mut prior_occurrences) = acquire(seed, acquisition);
            let mut support = Vec::new();
            let mut exact_field_copy = true;
            for ordinal in 0..super::SUPPORT_PRESENTATIONS {
                let episode = fixture(
                    seed + 1_000,
                    acquisition + ordinal,
                    ordinal % 4,
                    Perturbation::None,
                );
                prior_occurrences.extend(episode.raw.spikes.iter().map(|spike| spike.occurrence));
                let event = formation.form(&episode.raw)?;
                let export = export_episode(&episode.raw, &event);
                exact_field_copy &= export_matches(&export, &episode.raw, &event);
                support.push(export);
            }
            let episode = fixture(
                seed + 2_000,
                acquisition + super::SUPPORT_PRESENTATIONS + 17,
                0,
                Perturbation::None,
            );
            let current_occurrences = episode
                .raw
                .spikes
                .iter()
                .map(|spike| spike.occurrence)
                .collect::<BTreeSet<_>>();
            let event = formation.form(&episode.raw)?;
            let actual_target_events = u64::from(members_set(&event.members) == episode.selected);
            let target = export_episode(&episode.raw, &event);
            exact_field_copy &= export_matches(&target, &episode.raw, &event);
            Some(A1Bundle {
                provenance: super::ProvenanceAudit {
                    support_exports: support.len(),
                    actual_target_events,
                    raw_pulses: episode.raw.spikes.len(),
                    copied_pulses: target.pulses.len(),
                    raw_relations: episode.raw.propagation.len(),
                    copied_relations: target.observed_propagation.len(),
                    copied_members: target.members.len(),
                    copied_temporal_fields: target.relative_temporal.len(),
                    copied_propagation_fields: target.relative_propagation.len(),
                    exact_field_copy,
                    fresh_disjoint: prior_occurrences.is_disjoint(&current_occurrences),
                    frozen_e0_work: formation.work.organism_work(),
                    frozen_e0_persistent_bytes: formation.persistent_bytes(),
                    frozen_e0_temporary_bytes: size_of::<EventRelations>()
                        + size_of::<RawActivity>(),
                },
                support,
                target,
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_e0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_e0_anonymous_event_formation.rs"
    ));
    e0_a1_access!();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CellId(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell {
    occurrence: u32,
    binding: Option<CellId>,
    tick: i16,
    generation: u16,
    activation: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Arrow {
    endpoints: [CellId; 2],
    generation: u16,
    live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Substrate {
    cells: Vec<Cell>,
    arrows: Vec<Arrow>,
    members: [CellId; 3],
    relative_temporal: [i8; 9],
    relative_propagation: [i8; 9],
    observations: Vec<[CellId; 2]>,
    padding: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct LocalTemplate {
    temporal_delta: i8,
    directional_incidence: i8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SupportEvidence {
    count: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Proposal {
    endpoints: [CellId; 2],
    template: LocalTemplate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RouteRoot {
    cell: CellId,
    generation: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OpaqueHandle(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct BridgeEntry {
    handle: OpaqueHandle,
    root: RouteRoot,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct OpaqueBridge {
    entries: Vec<BridgeEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct StructuralTrace {
    bindings: Vec<u8>,
    adjacency: Vec<[u8; 2]>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct NormalizedEffect {
    trace: Vec<u8>,
    activation: [u16; 3],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkLedger {
    pub relation_observations: u64,
    pub membership_checks: u64,
    pub learned_relation_checks: u64,
    pub proposals_formed: u64,
    pub episode_dedup_checks: u64,
    pub support_comparisons: u64,
    pub support_updates: u64,
    pub cells_installed: u64,
    pub arrows_installed: u64,
    pub structural_normalizations: u64,
    pub structural_dedup_checks: u64,
    pub generation_validations: u64,
    pub spike_propagations: u64,
    pub arrow_traversals: u64,
    pub state_mutations: u64,
    pub bridge_reference_copies: u64,
    pub duplicate_reclamations: u64,
    pub cleanup_items: u64,
    pub persistent_bytes: usize,
    pub temporary_peak_bytes: usize,
    pub maintenance_work: u64,
    pub carrying_work: u64,
    pub evaluator_comparisons: u64,
}

impl WorkLedger {
    pub fn organism_work(&self) -> u64 {
        self.relation_observations
            + self.membership_checks
            + self.learned_relation_checks
            + self.proposals_formed
            + self.episode_dedup_checks
            + self.support_comparisons
            + self.support_updates
            + self.cells_installed
            + self.arrows_installed
            + self.structural_normalizations
            + self.structural_dedup_checks
            + self.generation_validations
            + self.spike_propagations
            + self.arrow_traversals
            + self.state_mutations
            + self.bridge_reference_copies
            + self.duplicate_reclamations
            + self.cleanup_items
            + self.maintenance_work
            + self.carrying_work
    }

    fn absorb(&mut self, other: &Self) {
        self.relation_observations += other.relation_observations;
        self.membership_checks += other.membership_checks;
        self.learned_relation_checks += other.learned_relation_checks;
        self.proposals_formed += other.proposals_formed;
        self.episode_dedup_checks += other.episode_dedup_checks;
        self.support_comparisons += other.support_comparisons;
        self.support_updates += other.support_updates;
        self.cells_installed += other.cells_installed;
        self.arrows_installed += other.arrows_installed;
        self.structural_normalizations += other.structural_normalizations;
        self.structural_dedup_checks += other.structural_dedup_checks;
        self.generation_validations += other.generation_validations;
        self.spike_propagations += other.spike_propagations;
        self.arrow_traversals += other.arrow_traversals;
        self.state_mutations += other.state_mutations;
        self.bridge_reference_copies += other.bridge_reference_copies;
        self.duplicate_reclamations += other.duplicate_reclamations;
        self.cleanup_items += other.cleanup_items;
        self.persistent_bytes = self.persistent_bytes.max(other.persistent_bytes);
        self.temporary_peak_bytes = self.temporary_peak_bytes.max(other.temporary_peak_bytes);
        self.maintenance_work += other.maintenance_work;
        self.carrying_work += other.carrying_work;
        self.evaluator_comparisons += other.evaluator_comparisons;
    }
}

#[derive(Clone, Debug, Default)]
struct Learner {
    templates: BTreeMap<LocalTemplate, SupportEvidence>,
    work: WorkLedger,
}

#[derive(Clone, Copy, Debug, Default)]
struct MappingOptions {
    reverse_allocation: bool,
    layout_padding: bool,
    reverse_observations: bool,
}

fn substrate_from_export(export: &E0EpisodeExport, options: MappingOptions) -> Option<Substrate> {
    let mut pulses = export.pulses.clone();
    if options.reverse_allocation {
        pulses.reverse();
    }
    let cells = pulses
        .iter()
        .map(|pulse| Cell {
            occurrence: pulse.occurrence,
            binding: None,
            tick: i16::from(pulse.tick),
            generation: 1,
            activation: 0,
        })
        .collect::<Vec<_>>();
    let cell_for = |occurrence: u32| {
        cells
            .iter()
            .position(|cell| cell.occurrence == occurrence)
            .map(|index| CellId(index as u16))
    };
    let members = export
        .members
        .map(cell_for)
        .into_iter()
        .collect::<Option<Vec<_>>>()?;
    let members: [CellId; 3] = members.try_into().ok()?;
    let mut observations = export
        .observed_propagation
        .iter()
        .map(|relation| Some([cell_for(relation[0])?, cell_for(relation[1])?]))
        .collect::<Option<Vec<_>>>()?;
    if options.reverse_observations {
        observations.reverse();
    }
    Some(Substrate {
        cells,
        arrows: Vec::new(),
        members,
        relative_temporal: export.relative_temporal,
        relative_propagation: export.relative_propagation,
        observations,
        padding: vec![0; usize::from(options.layout_padding) * 257],
    })
}

fn substrate_matches_export(substrate: &Substrate, export: &E0EpisodeExport) -> bool {
    substrate.cells.len() == export.pulses.len()
        && export.pulses.iter().all(|pulse| {
            substrate.cells.iter().any(|cell| {
                cell.occurrence == pulse.occurrence && cell.tick == i16::from(pulse.tick)
            })
        })
        && substrate
            .members
            .map(|cell| substrate.cells[usize::from(cell.0)].occurrence)
            == export.members
        && substrate.relative_temporal == export.relative_temporal
        && substrate.relative_propagation == export.relative_propagation
        && export.observed_propagation.iter().all(|relation| {
            substrate.observations.iter().any(|mapped| {
                mapped.map(|cell| substrate.cells[usize::from(cell.0)].occurrence) == *relation
            })
        })
}

fn member_index(substrate: &Substrate, cell: CellId) -> Option<usize> {
    substrate.members.iter().position(|member| *member == cell)
}

fn local_proposals(substrate: &Substrate, work: &mut WorkLedger) -> Vec<Proposal> {
    let mut proposals = BTreeSet::new();
    for endpoints in &substrate.observations {
        work.relation_observations += 1;
        work.membership_checks += 2;
        let (Some(from), Some(to)) = (
            member_index(substrate, endpoints[0]),
            member_index(substrate, endpoints[1]),
        ) else {
            continue;
        };
        work.learned_relation_checks += 1;
        let incidence = substrate.relative_propagation[from * 3 + to];
        if incidence <= 0 {
            continue;
        }
        let proposal = Proposal {
            endpoints: *endpoints,
            template: LocalTemplate {
                temporal_delta: substrate.relative_temporal[from * 3 + to],
                directional_incidence: incidence,
            },
        };
        work.episode_dedup_checks += proposals.len() as u64;
        if proposals.insert(proposal) {
            work.proposals_formed += 1;
        }
    }
    proposals.into_iter().collect()
}

impl Learner {
    fn observe(&mut self, substrate: &Substrate, plasticity: bool) -> usize {
        if !plasticity {
            return 0;
        }
        let proposals = local_proposals(substrate, &mut self.work);
        let episode_shapes = proposals
            .iter()
            .map(|proposal| proposal.template)
            .collect::<BTreeSet<_>>();
        for template in episode_shapes {
            self.work.support_comparisons += self.templates.len() as u64;
            self.work.support_updates += 1;
            self.templates.entry(template).or_default().count += 1;
        }
        self.work.persistent_bytes =
            self.templates.len() * (size_of::<LocalTemplate>() + size_of::<SupportEvidence>());
        proposals.len()
    }

    fn consolidated(&self) -> usize {
        self.templates
            .values()
            .filter(|support| support.count >= SUPPORT_EPISODES)
            .count()
    }

    fn install(
        &mut self,
        substrate: &mut Substrate,
        plasticity: bool,
        reverse_order: bool,
    ) -> (usize, Vec<RouteRoot>) {
        if !plasticity {
            return (0, Vec::new());
        }
        let mut proposals = local_proposals(substrate, &mut self.work);
        if reverse_order {
            proposals.reverse();
        }
        let candidates = proposals.len();
        let mut roots = Vec::new();
        for proposal in proposals {
            self.work.support_comparisons += self.templates.len() as u64;
            if !self
                .templates
                .get(&proposal.template)
                .is_some_and(|support| support.count >= SUPPORT_EPISODES)
            {
                continue;
            }
            let first = substrate.cells.len();
            for binding in proposal.endpoints {
                let member = substrate.cells[usize::from(binding.0)];
                substrate.cells.push(Cell {
                    occurrence: member.occurrence,
                    binding: Some(binding),
                    tick: member.tick,
                    generation: 1,
                    activation: 0,
                });
                self.work.cells_installed += 1;
            }
            substrate.arrows.push(Arrow {
                endpoints: [CellId(first as u16), CellId((first + 1) as u16)],
                generation: 1,
                live: true,
            });
            self.work.arrows_installed += 1;
            roots.push(RouteRoot {
                cell: CellId(first as u16),
                generation: 1,
            });
        }
        (candidates, roots)
    }
}

fn bound_member(substrate: &Substrate, route_cell: CellId) -> Option<u8> {
    let bound = substrate.cells.get(usize::from(route_cell.0))?.binding?;
    member_index(substrate, bound).map(|index| index as u8)
}

fn route_valid(substrate: &Substrate, root: RouteRoot, work: &mut WorkLedger) -> bool {
    work.generation_validations += 1;
    let Some(cell) = substrate.cells.get(usize::from(root.cell.0)) else {
        return false;
    };
    if cell.generation != root.generation || cell.binding.is_none() {
        return false;
    }
    substrate.arrows.iter().any(|arrow| {
        work.generation_validations += 1;
        arrow.live && arrow.generation > 0 && arrow.endpoints[0] == root.cell
    })
}

fn structural_trace(
    substrate: &Substrate,
    root: RouteRoot,
    work: &mut WorkLedger,
) -> Option<StructuralTrace> {
    if !route_valid(substrate, root, work) {
        return None;
    }
    work.structural_normalizations += 1;
    let mut queue = VecDeque::from([root.cell]);
    let mut visited = BTreeSet::new();
    let mut bindings = Vec::new();
    let mut adjacency = Vec::new();
    while let Some(cell) = queue.pop_front() {
        if !visited.insert(cell) {
            continue;
        }
        let from = bound_member(substrate, cell)?;
        bindings.push(from);
        for arrow in &substrate.arrows {
            if arrow.live && arrow.generation > 0 && arrow.endpoints[0] == cell {
                let to = bound_member(substrate, arrow.endpoints[1])?;
                adjacency.push([from, to]);
                queue.push_back(arrow.endpoints[1]);
            }
        }
    }
    adjacency.sort();
    Some(StructuralTrace {
        bindings,
        adjacency,
    })
}

fn structural_dedup(
    substrate: &mut Substrate,
    roots: &[RouteRoot],
    work: &mut WorkLedger,
) -> Vec<RouteRoot> {
    let mut accepted = Vec::new();
    let mut seen = BTreeSet::new();
    for root in roots {
        let Some(trace) = structural_trace(substrate, *root, work) else {
            continue;
        };
        work.structural_dedup_checks += seen.len() as u64;
        if seen.insert(trace) {
            accepted.push(*root);
        } else {
            for arrow in &mut substrate.arrows {
                if arrow.endpoints[0] == root.cell && arrow.live {
                    arrow.live = false;
                    work.duplicate_reclamations += 1;
                }
            }
        }
    }
    accepted
}

fn expose_roots(roots: &[RouteRoot], permuted: bool, work: &mut WorkLedger) -> OpaqueBridge {
    let mut copied = roots.to_vec();
    if permuted {
        copied.reverse();
    }
    OpaqueBridge {
        entries: copied
            .into_iter()
            .enumerate()
            .map(|(slot, root)| {
                work.bridge_reference_copies += 1;
                BridgeEntry {
                    handle: OpaqueHandle((slot as u32).wrapping_mul(2_654_435_761)),
                    root,
                }
            })
            .collect(),
    }
}

fn execute_root(
    frozen_start: &Substrate,
    root: RouteRoot,
    work: &mut WorkLedger,
) -> Option<NormalizedEffect> {
    if !route_valid(frozen_start, root, work) {
        return None;
    }
    let mut branch = frozen_start.clone();
    let mut queue = VecDeque::from([root.cell]);
    let mut visited = BTreeSet::new();
    let mut trace = Vec::new();
    let mut activation = [0u16; 3];
    while let Some(cell_id) = queue.pop_front() {
        if !visited.insert(cell_id) {
            continue;
        }
        let member = usize::from(bound_member(&branch, cell_id)?);
        branch.cells[usize::from(cell_id.0)].activation += 1;
        activation[member] += 1;
        trace.push(member as u8);
        work.spike_propagations += 1;
        work.state_mutations += 1;
        for arrow in &branch.arrows {
            if arrow.live && arrow.generation > 0 && arrow.endpoints[0] == cell_id {
                work.arrow_traversals += 1;
                queue.push_back(arrow.endpoints[1]);
            }
        }
    }
    (!trace.is_empty()).then_some(NormalizedEffect { trace, activation })
}

fn execute_handle(
    frozen_start: &Substrate,
    bridge: &OpaqueBridge,
    handle: OpaqueHandle,
    work: &mut WorkLedger,
) -> Option<NormalizedEffect> {
    let root = bridge
        .entries
        .iter()
        .find(|entry| entry.handle == handle)?
        .root;
    execute_root(frozen_start, root, work)
}

fn bridge_effects(
    substrate: &Substrate,
    bridge: &OpaqueBridge,
    work: &mut WorkLedger,
) -> (usize, BTreeSet<NormalizedEffect>) {
    let mut effects = BTreeSet::new();
    let mut nonempty = 0;
    for entry in &bridge.entries {
        if let Some(effect) = execute_handle(substrate, bridge, entry.handle, work) {
            nonempty += 1;
            work.evaluator_comparisons += effects.len() as u64;
            effects.insert(effect);
        }
    }
    (nonempty, effects)
}

fn train(support: &[E0EpisodeExport], reversed: bool) -> Option<Learner> {
    let mut learner = Learner::default();
    for export in support {
        let substrate = substrate_from_export(
            export,
            MappingOptions {
                reverse_observations: reversed,
                ..MappingOptions::default()
            },
        )?;
        learner.observe(&substrate, true);
    }
    Some(learner)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Inventory {
    candidates: usize,
    installed: usize,
    structural: usize,
    handles: usize,
    nonempty: usize,
    effects: BTreeSet<NormalizedEffect>,
}

fn inventory(
    learner: &mut Learner,
    export: &E0EpisodeExport,
    options: MappingOptions,
    reverse_installation: bool,
    permuted_handles: bool,
) -> Option<Inventory> {
    let mut substrate = substrate_from_export(export, options)?;
    let (candidates, installed) = learner.install(&mut substrate, true, reverse_installation);
    let roots = structural_dedup(&mut substrate, &installed, &mut learner.work);
    let bridge = expose_roots(&roots, permuted_handles, &mut learner.work);
    let (nonempty, effects) = bridge_effects(&substrate, &bridge, &mut learner.work);
    Some(Inventory {
        candidates,
        installed: installed.len(),
        structural: roots.len(),
        handles: bridge.entries.len(),
        nonempty,
        effects,
    })
}

fn relabel_export(export: &E0EpisodeExport) -> E0EpisodeExport {
    let mut changed = export.clone();
    let mapping = export
        .pulses
        .iter()
        .enumerate()
        .map(|(index, pulse)| (pulse.occurrence, 0x8000_0000u32 + index as u32 * 17))
        .collect::<BTreeMap<_, _>>();
    for pulse in &mut changed.pulses {
        pulse.occurrence = mapping[&pulse.occurrence];
    }
    for relation in &mut changed.observed_propagation {
        *relation = [mapping[&relation[0]], mapping[&relation[1]]];
    }
    changed.members = export.members.map(|member| mapping[&member]);
    changed
}

fn remove_member_relation(export: &E0EpisodeExport) -> E0EpisodeExport {
    let mut changed = export.clone();
    if let Some(index) = changed.observed_propagation.iter().position(|relation| {
        changed.members.contains(&relation[0]) && changed.members.contains(&relation[1])
    }) {
        changed.observed_propagation.remove(index);
    }
    changed
}

fn duplicate_member_relation(export: &E0EpisodeExport) -> E0EpisodeExport {
    let mut changed = export.clone();
    if let Some(relation) = changed
        .observed_propagation
        .iter()
        .find(|relation| {
            changed.members.contains(&relation[0]) && changed.members.contains(&relation[1])
        })
        .copied()
    {
        changed.observed_propagation.push(relation);
    }
    changed
}

fn symmetric_export(export: &E0EpisodeExport) -> E0EpisodeExport {
    let mut changed = export.clone();
    let tick = changed
        .pulses
        .iter()
        .find(|pulse| changed.members.contains(&pulse.occurrence))
        .map_or(0, |pulse| pulse.tick);
    for pulse in &mut changed.pulses {
        if changed.members.contains(&pulse.occurrence) {
            pulse.tick = tick;
        }
    }
    changed.relative_temporal = [0; 9];
    changed
}

fn clone_route(
    substrate: &mut Substrate,
    roots: &mut Vec<RouteRoot>,
    parallel_arrow: bool,
) -> bool {
    let Some(origin) = roots.first().copied() else {
        return false;
    };
    let Some(arrow) = substrate
        .arrows
        .iter()
        .find(|arrow| arrow.live && arrow.endpoints[0] == origin.cell)
        .copied()
    else {
        return false;
    };
    let first = substrate.cells.len();
    substrate
        .cells
        .push(substrate.cells[usize::from(origin.cell.0)]);
    substrate
        .cells
        .push(substrate.cells[usize::from(arrow.endpoints[1].0)]);
    let copied = Arrow {
        endpoints: [CellId(first as u16), CellId((first + 1) as u16)],
        generation: 1,
        live: true,
    };
    substrate.arrows.push(copied);
    if parallel_arrow {
        substrate.arrows.push(copied);
    }
    roots.push(RouteRoot {
        cell: CellId(first as u16),
        generation: 1,
    });
    true
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ControlAudit {
    pub relabel: bool,
    pub allocation_layout: bool,
    pub observation_installation_shuffle: bool,
    pub handle_permutation: bool,
    pub plasticity_disabled_none: bool,
    pub insufficient_support_none: bool,
    pub removed_relation_lawful_reduction: bool,
    pub duplicate_evidence_one: bool,
    pub duplicate_root_one: bool,
    pub same_effect_forces_failure_diagnostic: bool,
    pub stale_only_affected: bool,
    pub symmetric_unranked: bool,
    pub distractors_excluded: bool,
    pub cleanup_zero: bool,
    pub export_provenance: bool,
    pub zero_paths_mutation_sensitive: bool,
}

impl ControlAudit {
    pub fn passed(&self) -> bool {
        self.relabel
            && self.allocation_layout
            && self.observation_installation_shuffle
            && self.handle_permutation
            && self.plasticity_disabled_none
            && self.insufficient_support_none
            && self.removed_relation_lawful_reduction
            && self.duplicate_evidence_one
            && self.duplicate_root_one
            && self.same_effect_forces_failure_diagnostic
            && self.stale_only_affected
            && self.symmetric_unranked
            && self.distractors_excluded
            && self.cleanup_zero
            && self.export_provenance
            && self.zero_paths_mutation_sensitive
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub actual_e0_hash_match: bool,
    pub accepted_a0_hash_match: bool,
    pub prior_composition_hash_match: bool,
    pub export_fields_match_prior: bool,
    pub proposal_traversals: usize,
    pub route_installers: usize,
    pub adjacency_executors: usize,
    pub bridge_constructors: usize,
    pub ds1_choose_calls: usize,
    pub ds1_apply_calls: usize,
    pub consequence_paths: usize,
    pub semantic_opcode_sites: usize,
    pub expected_table_sites: usize,
    pub hidden_executor_sites: usize,
    pub persistent_identity_fields: usize,
    pub effect_to_bridge_edges: usize,
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.actual_e0_hash_match
            && self.accepted_a0_hash_match
            && self.prior_composition_hash_match
            && self.export_fields_match_prior
            && self.proposal_traversals == 1
            && self.route_installers == 1
            && self.adjacency_executors == 1
            && self.bridge_constructors == 1
            && self.ds1_choose_calls == 0
            && self.ds1_apply_calls == 0
            && self.consequence_paths == 0
            && self.semantic_opcode_sites == 0
            && self.expected_table_sites == 0
            && self.hidden_executor_sites == 0
            && self.persistent_identity_fields == 0
            && self.effect_to_bridge_edges == 0
    }
}

fn block_body<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    let start = source.find(marker)?;
    let tail = &source[start..];
    let open = tail.find('{')?;
    let mut depth = 0usize;
    for (offset, byte) in tail[open..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&tail[..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn count_fragments(source: &str, fragments: &[String]) -> usize {
    fragments
        .iter()
        .map(|fragment| source.matches(fragment).count())
        .sum()
}

fn derived_source_audit(source: &str) -> SourceAudit {
    let production = source.split("#[cfg(test)]").next().unwrap_or(source);
    let persistent = [
        block_body(source, "struct LocalTemplate").unwrap_or_default(),
        block_body(source, "struct SupportEvidence").unwrap_or_default(),
        block_body(source, "struct Learner").unwrap_or_default(),
    ]
    .concat();
    let prior = include_str!("ds1_after_e0_a0_composition_retry.rs");
    let current_export = block_body(source, "fn export_episode").unwrap_or_default();
    let prior_export = block_body(prior, "fn export_episode").unwrap_or_default();
    let required_export_fields = [
        "spike.occurrence.0",
        "spike.local_tick",
        "edge.from.0",
        "edge.to.0",
        "event.members.map",
        "event.temporal",
        "event.propagation",
    ];
    let semantic_fragments = [
        ["action", "_opcode"].concat(),
        ["alpha", "_meaning"].concat(),
        ["beta", "_meaning"].concat(),
        ["swap", "_meaning"].concat(),
        ["route", "_meaning"].concat(),
    ];
    let expected_fragments = [
        ["expected", "_routes"].concat(),
        ["legal", "_routes"].concat(),
        ["correct", "_route"].concat(),
    ];
    let hidden_fragments = [
        ["match ", "handle"].concat(),
        ["match ", "root.cell"].concat(),
    ];
    let consequence_fragments = [
        ["apply_", "consequence("].concat(),
        ["observe_", "post_action("].concat(),
        ["credit_", "update("].concat(),
    ];
    let bridge_body = block_body(source, "fn expose_roots").unwrap_or_default();
    SourceAudit {
        actual_e0_hash_match: env!("DS_A1_E0_SHA256") == FROZEN_DS_E0_SHA256,
        accepted_a0_hash_match: env!("DS_A1_A0_SHA256") == FROZEN_DS_A0_SHA256,
        prior_composition_hash_match: env!("DS_A1_PRIOR_SHA256") == PRIOR_COMPOSITION_SHA256,
        export_fields_match_prior: required_export_fields
            .iter()
            .all(|field| current_export.contains(field) && prior_export.contains(field)),
        proposal_traversals: production.matches("fn local_proposals(").count(),
        route_installers: production.matches("fn install(").count(),
        adjacency_executors: production.matches("fn execute_root(").count(),
        bridge_constructors: production.matches("fn expose_roots(").count(),
        ds1_choose_calls: production.matches(&[".cho", "ose("].concat()).count(),
        ds1_apply_calls: production
            .matches(&[".apply_", "consequence("].concat())
            .count(),
        consequence_paths: count_fragments(production, &consequence_fragments),
        semantic_opcode_sites: count_fragments(production, &semantic_fragments),
        expected_table_sites: count_fragments(production, &expected_fragments),
        hidden_executor_sites: count_fragments(production, &hidden_fragments),
        persistent_identity_fields: ["Occurrence", "CellId", "destination", "seed", "episode"]
            .iter()
            .map(|fragment| persistent.matches(fragment).count())
            .sum(),
        effect_to_bridge_edges: bridge_body.matches("NormalizedEffect").count()
            + bridge_body.matches("execute_").count(),
    }
}

fn source_audit() -> SourceAudit {
    derived_source_audit(include_str!("ds_a1_affordance_multiplicity.rs"))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub candidate_proposals: usize,
    pub consolidated_templates: usize,
    pub installed_roots: usize,
    pub structural_unique_roots: usize,
    pub unique_effects: usize,
    pub handles: usize,
    pub installed_cells: usize,
    pub installed_arrows: usize,
    pub provenance: ProvenanceAudit,
    pub controls: ControlAudit,
    pub work: WorkLedger,
    pub stage_ready: [bool; 9],
}

fn audit_seed(seed: u64, acquisition: usize, source: &SourceAudit) -> SeedAudit {
    let bundle = frozen_e0::a1_bundle(seed, acquisition).expect("actual frozen E0 event forms");
    let provenance = bundle.provenance.clone();
    let mut learner = train(&bundle.support, false).expect("support exports map");
    let consolidated_templates = learner.consolidated();
    let mut substrate = substrate_from_export(&bundle.target, MappingOptions::default())
        .expect("target export maps");
    let export_provenance = substrate_matches_export(&substrate, &bundle.target);
    let basal_cells = substrate.cells.len();
    let basal_arrows = substrate.arrows.len();
    let (candidate_proposals, roots) = learner.install(&mut substrate, true, false);
    let installed_roots = roots.len();
    let installed_cells = substrate.cells.len() - basal_cells;
    let installed_arrows = substrate.arrows.len() - basal_arrows;
    let structural = structural_dedup(&mut substrate, &roots, &mut learner.work);
    let structural_unique_roots = structural.len();
    let bridge = expose_roots(&structural, false, &mut learner.work);
    let handles = bridge.entries.len();
    let bridge_one_to_one = bridge_unique_roots(&bridge) == bridge.entries.len();
    let (nonempty, effects) = bridge_effects(&substrate, &bridge, &mut learner.work);
    let unique_effects = effects.len();
    let baseline = Inventory {
        candidates: candidate_proposals,
        installed: installed_roots,
        structural: structural_unique_roots,
        handles,
        nonempty,
        effects: effects.clone(),
    };

    let relabel = inventory(
        &mut learner.clone(),
        &relabel_export(&bundle.target),
        MappingOptions::default(),
        false,
        false,
    ) == Some(baseline.clone());
    let allocation_layout = inventory(
        &mut learner.clone(),
        &bundle.target,
        MappingOptions {
            reverse_allocation: true,
            layout_padding: true,
            ..MappingOptions::default()
        },
        false,
        false,
    ) == Some(baseline.clone());
    let mut shuffled_learner = train(&bundle.support, true).expect("shuffled support maps");
    let observation_installation_shuffle = inventory(
        &mut shuffled_learner,
        &bundle.target,
        MappingOptions {
            reverse_observations: true,
            ..MappingOptions::default()
        },
        true,
        false,
    ) == Some(baseline.clone());
    let handle_permutation = inventory(
        &mut learner.clone(),
        &bundle.target,
        MappingOptions::default(),
        false,
        true,
    ) == Some(baseline.clone());

    let mut disabled_substrate = substrate_from_export(&bundle.target, MappingOptions::default())
        .expect("disabled target maps");
    let plasticity_disabled_none = learner
        .install(&mut disabled_substrate, false, false)
        .1
        .is_empty();
    let mut weak = train(&bundle.support[..2], false).expect("weak support maps");
    let mut weak_substrate =
        substrate_from_export(&bundle.target, MappingOptions::default()).expect("weak target maps");
    let insufficient_support_none = weak.install(&mut weak_substrate, true, false).1.is_empty();

    let reduced = inventory(
        &mut learner.clone(),
        &remove_member_relation(&bundle.target),
        MappingOptions::default(),
        false,
        false,
    );
    let removed_relation_lawful_reduction = reduced.is_some_and(|value| {
        value.candidates + 1 == baseline.candidates
            && value.structural + 1 == baseline.structural
            && value.handles + 1 == baseline.handles
            && value.effects.len() + 1 == baseline.effects.len()
    });
    let duplicate_evidence = inventory(
        &mut learner.clone(),
        &duplicate_member_relation(&bundle.target),
        MappingOptions::default(),
        false,
        false,
    );
    let duplicate_evidence_one = duplicate_evidence == Some(baseline.clone());

    let mut duplicate_substrate = substrate_from_export(&bundle.target, MappingOptions::default())
        .expect("duplicate target maps");
    let (_, mut duplicate_roots) = learner.install(&mut duplicate_substrate, true, false);
    let duplicate_added = clone_route(&mut duplicate_substrate, &mut duplicate_roots, false);
    let duplicate_bridge_roots = structural_dedup(
        &mut duplicate_substrate,
        &duplicate_roots,
        &mut learner.work,
    );
    let duplicate_root_one = duplicate_added
        && duplicate_roots.len() == installed_roots + 1
        && duplicate_bridge_roots.len() == structural_unique_roots;

    let mut effect_substrate = substrate_from_export(&bundle.target, MappingOptions::default())
        .expect("effect target maps");
    let (_, mut effect_roots) = learner.install(&mut effect_substrate, true, false);
    let effect_added = clone_route(&mut effect_substrate, &mut effect_roots, true);
    let effect_structural =
        structural_dedup(&mut effect_substrate, &effect_roots, &mut learner.work);
    let effect_bridge = expose_roots(&effect_structural, false, &mut learner.work);
    let (effect_nonempty, effect_inventory) =
        bridge_effects(&effect_substrate, &effect_bridge, &mut learner.work);
    let same_effect_forces_failure_diagnostic = effect_added
        && effect_structural.len() == structural_unique_roots + 1
        && effect_bridge.entries.len() == effect_structural.len()
        && effect_nonempty == effect_bridge.entries.len()
        && effect_inventory.len() == unique_effects
        && effect_inventory.len() < effect_bridge.entries.len();

    let mut stale_substrate = substrate_from_export(&bundle.target, MappingOptions::default())
        .expect("stale target maps");
    let (_, stale_installed) = learner.install(&mut stale_substrate, true, false);
    let stale_before = structural_dedup(&mut stale_substrate, &stale_installed, &mut learner.work);
    if let Some(root) = stale_before.first() {
        stale_substrate.cells[usize::from(root.cell.0)].generation += 1;
    }
    let stale_after = structural_dedup(&mut stale_substrate, &stale_before, &mut learner.work);
    let stale_only_affected = stale_after.len() + 1 == stale_before.len();

    let symmetric_support = bundle
        .support
        .iter()
        .map(symmetric_export)
        .collect::<Vec<_>>();
    let mut symmetric_learner = train(&symmetric_support, false).expect("symmetric support maps");
    let symmetric = inventory(
        &mut symmetric_learner,
        &symmetric_export(&bundle.target),
        MappingOptions::default(),
        true,
        true,
    );
    let symmetric_unranked = symmetric.is_some_and(|value| {
        value.structural >= 2 && value.handles == value.structural && value.effects.len() >= 2
    });

    let member_relations = bundle
        .target
        .observed_propagation
        .iter()
        .filter(|relation| {
            bundle.target.members.contains(&relation[0])
                && bundle.target.members.contains(&relation[1])
        })
        .count();
    let distractors_excluded = candidate_proposals == member_relations
        && member_relations < bundle.target.observed_propagation.len();

    let mut cleanup_substrate = substrate;
    let mut cleanup_roots = structural;
    let mut cleanup_bridge = bridge;
    let bindings = cleanup_substrate
        .cells
        .iter()
        .filter(|cell| cell.binding.is_some())
        .count();
    learner.work.cleanup_items += (cleanup_substrate.cells.len()
        + cleanup_substrate.arrows.len()
        + cleanup_roots.len()
        + cleanup_bridge.entries.len()
        + bindings) as u64;
    cleanup_bridge.entries.clear();
    cleanup_roots.clear();
    cleanup_substrate.cells.clear();
    cleanup_substrate.arrows.clear();
    cleanup_substrate.observations.clear();
    cleanup_substrate.members = [CellId(0); 3];
    cleanup_substrate.padding.clear();
    let cleanup_zero = cleanup_substrate.cells.is_empty()
        && cleanup_substrate.arrows.is_empty()
        && cleanup_substrate.observations.is_empty()
        && cleanup_roots.is_empty()
        && cleanup_bridge.entries.is_empty();

    let implementation = include_str!("ds_a1_affordance_multiplicity.rs");
    let mutate =
        |body: &str| implementation.replacen("#[cfg(test)]", &format!("{body}\n#[cfg(test)]"), 1);
    let zero_paths_mutation_sensitive = derived_source_audit(&mutate(
        &["fn mutation_choose() { learner.cho", "ose(view, 0); }"].concat(),
    ))
    .ds1_choose_calls
        > 0
        && derived_source_audit(&mutate(
            &[
                "fn mutation_apply() { learner.apply_",
                "consequence(view, 0, true); }",
            ]
            .concat(),
        ))
        .ds1_apply_calls
            > 0
        && derived_source_audit(&mutate(
            &["fn mutation_opcode() { let action", "_opcode = 1; }"].concat(),
        ))
        .semantic_opcode_sites
            > 0
        && derived_source_audit(&mutate(
            &[
                "fn mutation_hidden(handle: usize) { match ",
                "handle { _ => {} } }",
            ]
            .concat(),
        ))
        .hidden_executor_sites
            > 0;

    let controls = ControlAudit {
        relabel,
        allocation_layout,
        observation_installation_shuffle,
        handle_permutation,
        plasticity_disabled_none,
        insufficient_support_none,
        removed_relation_lawful_reduction,
        duplicate_evidence_one,
        duplicate_root_one,
        same_effect_forces_failure_diagnostic,
        stale_only_affected,
        symmetric_unranked,
        distractors_excluded,
        cleanup_zero,
        export_provenance,
        zero_paths_mutation_sensitive,
    };

    learner.work.temporary_peak_bytes = size_of::<Substrate>()
        + bundle.target.pulses.len() * size_of::<Cell>()
        + installed_cells * size_of::<Cell>()
        + installed_arrows * size_of::<Arrow>()
        + structural_unique_roots * size_of::<RouteRoot>()
        + handles * size_of::<BridgeEntry>()
        + 257;
    learner.work.absorb(&shuffled_learner.work);
    learner.work.absorb(&symmetric_learner.work);

    let stage_ready = [
        source.passed(),
        provenance.actual_target_events == 1
            && provenance.exact_field_copy
            && provenance.fresh_disjoint
            && export_provenance,
        candidate_proposals > 0,
        consolidated_templates >= 2,
        installed_roots >= 2 && installed_cells > 0 && installed_arrows > 0,
        structural_unique_roots >= 2,
        nonempty == handles && unique_effects >= 2 && unique_effects == handles,
        handles == structural_unique_roots && bridge_one_to_one,
        controls.passed(),
    ];

    SeedAudit {
        seed,
        candidate_proposals,
        consolidated_templates,
        installed_roots,
        structural_unique_roots,
        unique_effects,
        handles,
        installed_cells,
        installed_arrows,
        provenance,
        controls,
        work: learner.work,
        stage_ready,
    }
}

fn bridge_unique_roots(bridge: &OpaqueBridge) -> usize {
    bridge
        .entries
        .iter()
        .map(|entry| entry.root)
        .collect::<BTreeSet<_>>()
        .len()
}

fn ordered_freeze(ready: [bool; 9]) -> ([String; 9], Option<usize>) {
    let first = ready.iter().position(|stage| !stage);
    let stages = std::array::from_fn(|stage| match first {
        None => "READY".to_string(),
        Some(collapse) if stage < collapse => "READY".to_string(),
        Some(collapse) if stage == collapse => format!("COLLAPSE: {}", STAGE_NAMES[stage]),
        Some(_) => "BLOCKED".to_string(),
    });
    (stages, first)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateReport {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub m0_authoritative: bool,
    pub enabling_only: bool,
    pub m1_exists: bool,
    pub ds1_retry: bool,
    pub source_audit: SourceAudit,
    pub stages: [String; 9],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub seeds: Vec<SeedAudit>,
    pub passed: bool,
}

fn definitive_rejection() -> GateReport {
    GateReport {
        label: "DS-A1 DEVELOPMENT: definitive forbidden".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "DEFINITIVE-FORBIDDEN".to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        enabling_only: true,
        m1_exists: false,
        ds1_retry: false,
        source_audit: source_audit(),
        stages: std::array::from_fn(|_| "BLOCKED: definitive rejected".to_string()),
        first_collapse_stage: None,
        first_collapse: "NOT RUN: definitive rejected before harness".to_string(),
        seeds: Vec::new(),
        passed: false,
    }
}

pub fn run(mode: HarnessMode) -> GateReport {
    if mode == HarnessMode::Definitive {
        return definitive_rejection();
    }
    let (acquisition, seeds): (usize, &[u64]) = match mode {
        HarnessMode::Micro => (16, &[100]),
        HarnessMode::Gate => (32, &[100, 101, 102, 103, 104]),
        HarnessMode::Definitive => unreachable!("rejected before harness"),
    };
    let source = source_audit();
    let seed_audits = seeds
        .iter()
        .map(|seed| audit_seed(*seed, acquisition, &source))
        .collect::<Vec<_>>();
    let mut ready = [false; 9];
    for (stage, value) in ready.iter_mut().enumerate() {
        *value = seed_audits.iter().all(|seed| seed.stage_ready[stage]);
    }
    let (stages, first_collapse_stage) = ordered_freeze(ready);
    let first_collapse = first_collapse_stage
        .map(|stage| STAGE_NAMES[stage].to_string())
        .unwrap_or_else(|| "NONE: DS-A1 development implementation ready".to_string());
    let passed = first_collapse_stage.is_none();
    GateReport {
        label: if passed {
            "DS-A1 DEVELOPMENT IMPLEMENTATION READY".to_string()
        } else {
            format!("DS-A1 DEVELOPMENT COLLAPSE AT {first_collapse}")
        },
        protocol: PROTOCOL.to_string(),
        mode: match mode {
            HarnessMode::Micro => "MICRO",
            HarnessMode::Gate => "GATE",
            HarnessMode::Definitive => unreachable!("rejected"),
        }
        .to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        enabling_only: true,
        m1_exists: false,
        ds1_retry: false,
        source_audit: source,
        stages,
        first_collapse_stage,
        first_collapse,
        seeds: seed_audits,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_uses_actual_e0_export_and_passes_all_stages() {
        let report = run(HarnessMode::Micro);
        assert!(report.passed, "{report:#?}");
        assert!(report.stages.iter().all(|stage| stage == "READY"));
        assert!(report.seeds.iter().all(|seed| {
            seed.provenance.actual_target_events == 1
                && seed.provenance.exact_field_copy
                && seed.candidate_proposals >= 2
                && seed.consolidated_templates >= 2
                && seed.installed_roots >= 2
                && seed.structural_unique_roots >= 2
                && seed.unique_effects >= 2
                && seed.handles == seed.structural_unique_roots
        }));
    }

    #[test]
    fn all_required_controls_pass() {
        let report = run(HarnessMode::Micro);
        assert!(
            report.seeds.iter().all(|seed| seed.controls.passed()),
            "{report:#?}"
        );
    }

    #[test]
    fn source_and_mutation_audits_are_mechanical() {
        let report = run(HarnessMode::Micro);
        assert!(report.source_audit.passed(), "{:#?}", report.source_audit);
        assert!(report
            .seeds
            .iter()
            .all(|seed| seed.controls.zero_paths_mutation_sensitive));
    }

    #[test]
    fn ordered_freeze_blocks_later_stages() {
        for collapse in 0..9 {
            let mut ready = [true; 9];
            ready[collapse] = false;
            let (stages, first) = ordered_freeze(ready);
            assert_eq!(first, Some(collapse));
            assert!(stages[..collapse].iter().all(|stage| stage == "READY"));
            assert!(stages[collapse].starts_with("COLLAPSE"));
            assert!(stages[collapse + 1..]
                .iter()
                .all(|stage| stage == "BLOCKED"));
        }
    }

    #[test]
    fn definitive_mode_runs_no_seed() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.passed);
        assert!(!report.claim_eligible);
        assert!(report.seeds.is_empty());
        assert!(!report.ds1_retry);
    }
}
