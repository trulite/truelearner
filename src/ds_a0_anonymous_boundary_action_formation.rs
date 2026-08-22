//! DS-A0 development-only anonymous executable-route formation gate.
//!
//! Raw spikes and propagation observations are not executable routes. Supported
//! coactivity causes local plasticity to install fresh episode CELL/ARROW
//! structures. A bridge copies their opaque roots; execution injects one SPIKE
//! at a root and follows the live adjacency stored in the substrate.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::mem::size_of;

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-a0-anonymous-boundary-action-formation-v1";
pub const EXACT_PARENT: &str = "85b01a50d0f85995632bbd7e604d6d2ff554f0b7";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_DS_E0_SHA256: &str =
    "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615";
pub const FROZEN_DS1_COMPOSITION_SHA256: &str =
    "a4deadedfde7b9896d64d0cacd41560441ea85cf3bda119a5d09aa3aaddcd7a0";
pub const FROZEN_DS1_LEARNER_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";

const SUPPORT_EPISODES: u16 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Occurrence(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CellId(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell {
    occurrence: Occurrence,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ObservedSpike {
    cell: CellId,
    tick: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ObservedPropagation {
    endpoints: [CellId; 2],
}

/// Ordinary activity history. It cannot be traversed by the route executor.
#[derive(Clone, Debug, PartialEq, Eq)]
struct RawActivity {
    spikes: Vec<ObservedSpike>,
    propagation: Vec<ObservedPropagation>,
}

/// Already-learned DS-E0 episode membership and relative support.
#[derive(Clone, Debug, PartialEq, Eq)]
struct EventFrame {
    members: [CellId; 3],
    relative_temporal: [i8; 9],
    relative_propagation: [i8; 9],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TemporarySubstrate {
    cells: Vec<Cell>,
    arrows: Vec<Arrow>,
    event: EventFrame,
    raw: RawActivity,
    padding: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RouteTemplate {
    temporal_deltas: [i8; 2],
    local_incidence: [u8; 3],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SupportEvidence {
    episodes: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ObservedPath {
    cells: [CellId; 3],
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
struct ActionBridge {
    entries: Vec<BridgeEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhysicalExecution {
    trace: Vec<CellId>,
    state: Vec<u16>,
    arrow_steps: u64,
    spike_propagations: u64,
    state_mutations: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkLedger {
    pub raw_propagation_observations: u64,
    pub coactivity_paths: u64,
    pub support_updates: u64,
    pub template_comparisons: u64,
    pub route_cells_installed: u64,
    pub route_arrows_installed: u64,
    pub arrow_traversals: u64,
    pub spike_propagations: u64,
    pub state_mutations: u64,
    pub bridge_reference_copies: u64,
    pub generation_validations: u64,
    pub cleanup_items: u64,
    pub persistent_bytes: usize,
    pub temporary_peak_bytes: usize,
    pub maintenance_work: u64,
    pub carrying_work: u64,
}

impl WorkLedger {
    pub fn physical_work(&self) -> u64 {
        self.raw_propagation_observations
            + self.coactivity_paths
            + self.support_updates
            + self.template_comparisons
            + self.route_cells_installed
            + self.route_arrows_installed
            + self.arrow_traversals
            + self.spike_propagations
            + self.state_mutations
            + self.bridge_reference_copies
            + self.generation_validations
            + self.cleanup_items
            + self.maintenance_work
            + self.carrying_work
    }

    fn absorb(&mut self, other: &Self) {
        self.raw_propagation_observations += other.raw_propagation_observations;
        self.coactivity_paths += other.coactivity_paths;
        self.support_updates += other.support_updates;
        self.template_comparisons += other.template_comparisons;
        self.route_cells_installed += other.route_cells_installed;
        self.route_arrows_installed += other.route_arrows_installed;
        self.arrow_traversals += other.arrow_traversals;
        self.spike_propagations += other.spike_propagations;
        self.state_mutations += other.state_mutations;
        self.bridge_reference_copies += other.bridge_reference_copies;
        self.generation_validations += other.generation_validations;
        self.cleanup_items += other.cleanup_items;
        self.persistent_bytes = self.persistent_bytes.max(other.persistent_bytes);
        self.temporary_peak_bytes = self.temporary_peak_bytes.max(other.temporary_peak_bytes);
        self.maintenance_work += other.maintenance_work;
        self.carrying_work += other.carrying_work;
    }
}

#[derive(Clone, Debug, Default)]
struct RouteLearner {
    templates: BTreeMap<RouteTemplate, SupportEvidence>,
    work: WorkLedger,
}

fn member(event: &EventFrame, cell: CellId) -> bool {
    event.members.contains(&cell)
}

/// Discover recurrent coactivity in raw observations, never executable ARROWs.
fn observed_paths(
    event: &EventFrame,
    raw: &RawActivity,
    work: &mut WorkLedger,
) -> Vec<ObservedPath> {
    let mut paths = Vec::new();
    for (first_index, first) in raw.propagation.iter().enumerate() {
        work.raw_propagation_observations += 1;
        if !member(event, first.endpoints[0]) || !member(event, first.endpoints[1]) {
            continue;
        }
        for (second_index, second) in raw.propagation.iter().enumerate() {
            work.raw_propagation_observations += 1;
            if first_index == second_index
                || first.endpoints[1] != second.endpoints[0]
                || first.endpoints[0] == second.endpoints[1]
                || !member(event, second.endpoints[1])
            {
                continue;
            }
            work.coactivity_paths += 1;
            paths.push(ObservedPath {
                cells: [first.endpoints[0], first.endpoints[1], second.endpoints[1]],
            });
        }
    }
    paths
}

fn route_template(raw: &RawActivity, path: &ObservedPath) -> RouteTemplate {
    let tick = |cell: CellId| {
        raw.spikes
            .iter()
            .find(|spike| spike.cell == cell)
            .expect("observed path cells fired")
            .tick
    };
    let ticks = path.cells.map(tick);
    RouteTemplate {
        temporal_deltas: [
            (ticks[1] - ticks[0]).clamp(i16::from(i8::MIN), i16::from(i8::MAX)) as i8,
            (ticks[2] - ticks[1]).clamp(i16::from(i8::MIN), i16::from(i8::MAX)) as i8,
        ],
        local_incidence: [1, 2, 1],
    }
}

impl RouteLearner {
    fn observe_episode(&mut self, substrate: &TemporarySubstrate, plasticity: bool, reverse: bool) {
        if !plasticity {
            return;
        }
        let mut paths = observed_paths(&substrate.event, &substrate.raw, &mut self.work);
        if reverse {
            paths.reverse();
        }
        let episode_templates = paths
            .iter()
            .map(|path| route_template(&substrate.raw, path))
            .collect::<BTreeSet<_>>();
        for template in episode_templates {
            self.work.template_comparisons += self.templates.len() as u64;
            self.work.support_updates += 1;
            self.templates.entry(template).or_default().episodes += 1;
        }
        self.work.persistent_bytes =
            self.templates.len() * (size_of::<RouteTemplate>() + size_of::<SupportEvidence>());
    }

    /// Supported coactivity installs actual episode-local CELL/ARROW chains.
    fn form_routes(
        &mut self,
        substrate: &mut TemporarySubstrate,
        plasticity: bool,
    ) -> Vec<RouteRoot> {
        if !plasticity {
            return Vec::new();
        }
        let paths = observed_paths(&substrate.event, &substrate.raw, &mut self.work);
        let mut roots = Vec::new();
        for path in paths {
            let template = route_template(&substrate.raw, &path);
            self.work.template_comparisons += self.templates.len() as u64;
            if !self
                .templates
                .get(&template)
                .is_some_and(|support| support.episodes >= SUPPORT_EPISODES)
            {
                continue;
            }
            let first_cell = substrate.cells.len();
            for bound in path.cells {
                let bound_cell = substrate.cells[usize::from(bound.0)];
                substrate.cells.push(Cell {
                    occurrence: bound_cell.occurrence,
                    binding: Some(bound),
                    tick: bound_cell.tick,
                    generation: 1,
                    activation: 0,
                });
                self.work.route_cells_installed += 1;
            }
            for offset in 0..2 {
                substrate.arrows.push(Arrow {
                    endpoints: [
                        CellId((first_cell + offset) as u16),
                        CellId((first_cell + offset + 1) as u16),
                    ],
                    generation: 1,
                    live: true,
                });
                self.work.route_arrows_installed += 1;
            }
            roots.push(RouteRoot {
                cell: CellId(first_cell as u16),
                generation: 1,
            });
        }
        roots
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for (template, evidence) in &self.templates {
            for value in template
                .temporal_deltas
                .iter()
                .map(|value| *value as i64 as u64)
                .chain(template.local_incidence.map(u64::from))
                .chain([u64::from(evidence.episodes)])
            {
                hash ^= value;
                hash = hash.wrapping_mul(0x100_0000_01b3);
            }
        }
        hash
    }
}

fn root_valid(substrate: &TemporarySubstrate, root: RouteRoot, work: &mut WorkLedger) -> bool {
    work.generation_validations += 1;
    let Some(cell) = substrate.cells.get(usize::from(root.cell.0)) else {
        return false;
    };
    if cell.generation != root.generation || cell.binding.is_none() {
        return false;
    }
    work.generation_validations += substrate.arrows.len() as u64;
    substrate
        .arrows
        .iter()
        .any(|arrow| arrow.live && arrow.endpoints[0] == root.cell && arrow.generation > 0)
}

fn remove_stale(substrate: &TemporarySubstrate, roots: &mut Vec<RouteRoot>, work: &mut WorkLedger) {
    roots.retain(|root| root_valid(substrate, *root, work));
}

/// Mechanical reference copy over already-installed roots.
fn expose_routes(roots: &[RouteRoot], permuted: bool, work: &mut WorkLedger) -> ActionBridge {
    let mut copied = roots.to_vec();
    if permuted {
        copied.reverse();
    }
    let entries = copied
        .into_iter()
        .enumerate()
        .map(|(slot, root)| {
            work.bridge_reference_copies += 1;
            BridgeEntry {
                handle: OpaqueHandle((slot as u32).wrapping_mul(2_654_435_761)),
                root,
            }
        })
        .collect();
    ActionBridge { entries }
}

/// Resolve handle to root, inject a SPIKE, then follow live substrate ARROWs.
fn execute_handle(
    frozen_start: &TemporarySubstrate,
    bridge: &ActionBridge,
    handle: OpaqueHandle,
    work: &mut WorkLedger,
) -> Option<PhysicalExecution> {
    let root = bridge
        .entries
        .iter()
        .find(|entry| entry.handle == handle)?
        .root;
    if !root_valid(frozen_start, root, work) {
        return None;
    }
    let mut branch = frozen_start.clone();
    let mut queue = VecDeque::from([root.cell]);
    let mut visited = BTreeSet::new();
    let mut trace = Vec::new();
    let mut arrow_steps = 0u64;
    while let Some(cell_id) = queue.pop_front() {
        if !visited.insert(cell_id) {
            continue;
        }
        let cell = branch.cells.get_mut(usize::from(cell_id.0))?;
        cell.activation += 1;
        trace.push(cell_id);
        work.spike_propagations += 1;
        work.state_mutations += 1;
        for arrow in &branch.arrows {
            if arrow.live && arrow.generation > 0 && arrow.endpoints[0] == cell_id {
                arrow_steps += 1;
                work.arrow_traversals += 1;
                queue.push_back(arrow.endpoints[1]);
            }
        }
    }
    if arrow_steps == 0 {
        return None;
    }
    Some(PhysicalExecution {
        trace,
        state: branch.cells.iter().map(|cell| cell.activation).collect(),
        arrow_steps,
        spike_propagations: visited.len() as u64,
        state_mutations: visited.len() as u64,
    })
}

#[derive(Clone, Copy, Debug, Default)]
struct FixtureOptions {
    relabel: bool,
    allocation: bool,
    layout_padding: bool,
    shifted_timing: bool,
    shuffled_observations: bool,
    removed_observation: bool,
    symmetric: bool,
}

fn fixture(seed: u64, episode: usize, options: FixtureOptions) -> TemporarySubstrate {
    let base = seed
        .wrapping_mul(1_000_003)
        .wrapping_add((episode as u64).wrapping_mul(17))
        .wrapping_add(10_000);
    let mut occurrences = (0..6)
        .map(|offset| Occurrence(base.wrapping_add(offset) as u32))
        .collect::<Vec<_>>();
    if options.relabel {
        occurrences.reverse();
    }
    let shift = i16::from(options.shifted_timing) * 31;
    let final_tick = if options.symmetric { 11 } else { 12 };
    let ticks = [10 + shift, 11 + shift, final_tick + shift, 4, 19, 27];
    let order = if options.allocation {
        [4, 2, 5, 0, 3, 1]
    } else {
        [0, 1, 2, 3, 4, 5]
    };
    let cells = order
        .map(|logical| Cell {
            occurrence: occurrences[logical],
            binding: None,
            tick: ticks[logical],
            generation: 1,
            activation: 0,
        })
        .to_vec();
    let cell_for = |logical: usize| {
        CellId(
            cells
                .iter()
                .position(|cell| cell.occurrence == occurrences[logical])
                .expect("fixture occurrence exists") as u16,
        )
    };
    let members = [cell_for(0), cell_for(1), cell_for(2)];
    let member_ticks = members.map(|cell| cells[usize::from(cell.0)].tick);
    let learned_event_edges = [[0, 1], [1, 2], [0, 2], [2, 1]];
    let mut relative_temporal = [0i8; 9];
    let mut relative_propagation = [0i8; 9];
    for first in 0..3 {
        for second in 0..3 {
            relative_temporal[first * 3 + second] = (member_ticks[second] - member_ticks[first])
                .clamp(i16::from(i8::MIN), i16::from(i8::MAX))
                as i8;
            if learned_event_edges.contains(&[first, second]) {
                relative_propagation[first * 3 + second] = 1;
            }
        }
    }
    let mut observed_edges = vec![[0, 1], [1, 2], [0, 2], [2, 1], [3, 4], [4, 5]];
    if options.removed_observation {
        observed_edges.remove(2);
    }
    let mut propagation = observed_edges
        .into_iter()
        .map(|edge| ObservedPropagation {
            endpoints: [cell_for(edge[0]), cell_for(edge[1])],
        })
        .collect::<Vec<_>>();
    if options.shuffled_observations {
        propagation.reverse();
    }
    let spikes = (0..6)
        .map(|logical| ObservedSpike {
            cell: cell_for(logical),
            tick: ticks[logical],
        })
        .collect();
    let basal_arrows = [[cell_for(3), cell_for(4)], [cell_for(4), cell_for(5)]]
        .into_iter()
        .map(|endpoints| Arrow {
            endpoints,
            generation: 1,
            live: true,
        })
        .collect();
    TemporarySubstrate {
        cells,
        arrows: basal_arrows,
        event: EventFrame {
            members,
            relative_temporal,
            relative_propagation,
        },
        raw: RawActivity {
            spikes,
            propagation,
        },
        padding: vec![0; usize::from(options.layout_padding) * 257],
    }
}

fn event_support_fingerprint(event: &EventFrame) -> u64 {
    event
        .relative_temporal
        .iter()
        .chain(event.relative_propagation.iter())
        .fold(0xcbf2_9ce4_8422_2325u64, |mut hash, value| {
            hash ^= *value as i64 as u64;
            hash.wrapping_mul(0x100_0000_01b3)
        })
}

fn event_executable_roots(substrate: &TemporarySubstrate) -> usize {
    substrate
        .event
        .members
        .iter()
        .filter(|member_cell| {
            substrate
                .arrows
                .iter()
                .any(|arrow| arrow.live && arrow.endpoints[0] == **member_cell)
        })
        .count()
}

fn acquire(
    seed: u64,
    episodes: usize,
    plasticity: bool,
    reverse: bool,
) -> (RouteLearner, BTreeSet<Occurrence>) {
    let mut learner = RouteLearner::default();
    let mut occurrences = BTreeSet::new();
    for episode in 0..episodes {
        let substrate = fixture(seed, episode, FixtureOptions::default());
        occurrences.extend(substrate.cells.iter().map(|cell| cell.occurrence));
        learner.observe_episode(&substrate, plasticity, reverse && episode % 2 == 0);
    }
    (learner, occurrences)
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ControlAudit {
    pub fresh_disjoint: bool,
    pub relabel: bool,
    pub allocation_layout: bool,
    pub handle_permutation: bool,
    pub distractor_interleaving: bool,
    pub shuffled_coactivity: bool,
    pub shuffled_propagation: bool,
    pub removed_route_lawful: bool,
    pub timing_transfer: bool,
    pub symmetric_unranked: bool,
    pub no_plasticity_no_routes: bool,
    pub unsupported_no_routes: bool,
    pub stale_invalidation_removal: bool,
    pub cleanup_zero_retained: bool,
    pub independent_distinct_effects: bool,
    pub baseline_has_no_event_routes: bool,
    pub learner_installs_before_bridge: bool,
}

impl ControlAudit {
    pub fn passed(&self) -> bool {
        self.fresh_disjoint
            && self.relabel
            && self.allocation_layout
            && self.handle_permutation
            && self.distractor_interleaving
            && self.shuffled_coactivity
            && self.shuffled_propagation
            && self.removed_route_lawful
            && self.timing_transfer
            && self.symmetric_unranked
            && self.no_plasticity_no_routes
            && self.unsupported_no_routes
            && self.stale_invalidation_removal
            && self.cleanup_zero_retained
            && self.independent_distinct_effects
            && self.baseline_has_no_event_routes
            && self.learner_installs_before_bridge
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub semantic_opcode_sites: usize,
    pub evaluator_selection_sites: usize,
    pub hidden_executor_sites: usize,
    pub ds1_choose_calls: usize,
    pub ds1_apply_calls: usize,
    pub post_action_consequence_paths: usize,
    pub executor_definitions: usize,
    pub bridge_constructor_definitions: usize,
    pub route_installer_definitions: usize,
    pub preassembled_route_cell_fields: usize,
    pub frozen_sources_untouched: bool,
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.semantic_opcode_sites == 0
            && self.evaluator_selection_sites == 0
            && self.hidden_executor_sites == 0
            && self.ds1_choose_calls == 0
            && self.ds1_apply_calls == 0
            && self.post_action_consequence_paths == 0
            && self.executor_definitions == 1
            && self.bridge_constructor_definitions == 1
            && self.route_installer_definitions == 1
            && self.preassembled_route_cell_fields == 0
            && self.frozen_sources_untouched
    }
}

fn count_fragments(source: &str, fragments: &[String]) -> usize {
    fragments
        .iter()
        .map(|fragment| source.matches(fragment).count())
        .sum()
}

fn function_body<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let marker = ["fn ", name].concat();
    let start = source.find(&marker)?;
    let tail = &source[start..];
    let open = tail.find('{')?;
    let mut depth = 0usize;
    for (offset, byte) in tail[open..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&tail[open..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn source_audit() -> SourceAudit {
    let implementation = include_str!("ds_a0_anonymous_boundary_action_formation.rs");
    let runner = include_str!("bin/ds_a0_anonymous_boundary_action_formation.rs");
    let joined = [implementation, runner].concat();
    let semantic_fragments = [
        ["sw", "ap"].concat(),
        ["ke", "ep"].concat(),
        ["le", "ft"].concat(),
        ["ri", "ght"].concat(),
        ["alpha", "_meaning"].concat(),
        ["beta", "_meaning"].concat(),
        ["action", "_opcode"].concat(),
    ];
    let evaluator_fragments = [
        ["expected", "_action"].concat(),
        ["correct", "_route"].concat(),
        ["evaluator", "_rank"].concat(),
    ];
    let hidden_fragments = [
        ["match ", "handle"].concat(),
        ["match ", "root.cell"].concat(),
    ];
    let ds1_choose_calls = joined.matches(&[".cho", "ose("].concat()).count();
    let ds1_apply_calls = joined
        .matches(&[".apply_", "consequence("].concat())
        .count();
    let executor = function_body(implementation, "execute_handle")
        .expect("owned root executor remains auditable");
    let executor_sink_fragments = [
        ["conse", "quence"].concat(),
        ["cred", "it"].concat(),
        ["rew", "ard"].concat(),
        ["term", "inal"].concat(),
    ];
    let post_action_consequence_paths =
        count_fragments(executor, &executor_sink_fragments) + ds1_apply_calls;
    SourceAudit {
        semantic_opcode_sites: count_fragments(&joined, &semantic_fragments),
        evaluator_selection_sites: count_fragments(&joined, &evaluator_fragments),
        hidden_executor_sites: count_fragments(&joined, &hidden_fragments),
        ds1_choose_calls,
        ds1_apply_calls,
        post_action_consequence_paths,
        executor_definitions: implementation
            .matches(&["fn execute_", "handle("].concat())
            .count(),
        bridge_constructor_definitions: implementation
            .matches(&["fn expose_", "routes("].concat())
            .count(),
        route_installer_definitions: implementation
            .matches(&["fn form_", "routes("].concat())
            .count(),
        preassembled_route_cell_fields: implementation
            .matches(&["struct Route", "Instance"].concat())
            .count(),
        frozen_sources_untouched: crate::ds_e0_anonymous_event_formation::FROZEN_DS1_LEARNER_SHA256
            == FROZEN_DS1_LEARNER_SHA256
            && FROZEN_DS_E0_SHA256
                == "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615"
            && FROZEN_DS1_COMPOSITION_SHA256
                == "a4deadedfde7b9896d64d0cacd41560441ea85cf3bda119a5d09aa3aaddcd7a0",
    }
}

#[derive(Clone, Debug)]
pub struct SeedAudit {
    pub seed: u64,
    pub acquisition_episodes: usize,
    pub evaluation_episodes: usize,
    pub templates: usize,
    pub learner_fingerprint: u64,
    pub preformation_event_roots: usize,
    pub formed_routes: usize,
    pub installed_route_cells: usize,
    pub installed_route_arrows: usize,
    pub exposed_handles: usize,
    pub one_to_one_roots: usize,
    pub physical_execution_paths: usize,
    pub arrow_path_steps: u64,
    pub distinct_effect_pairs: usize,
    pub ds1_choose_calls: usize,
    pub ds1_apply_calls: usize,
    pub post_action_consequence_paths: usize,
    pub controls: ControlAudit,
    pub work: WorkLedger,
    pub passed: bool,
}

fn audit_seed(
    seed: u64,
    acquisition_episodes: usize,
    evaluation_episodes: usize,
    source_audit: &SourceAudit,
) -> SeedAudit {
    let (mut learner, acquisition_occurrences) = acquire(seed, acquisition_episodes, true, false);
    let (coactivity_learner, _) = acquire(seed, acquisition_episodes, true, true);
    let mut symmetric_learner = RouteLearner::default();
    for episode in 0..acquisition_episodes {
        symmetric_learner.observe_episode(
            &fixture(
                seed,
                episode + 2_000,
                FixtureOptions {
                    symmetric: true,
                    ..FixtureOptions::default()
                },
            ),
            true,
            episode % 2 == 0,
        );
    }
    let (mut no_plasticity, _) = acquire(seed, acquisition_episodes, false, false);
    let mut unsupported = RouteLearner::default();
    let mut evaluation_occurrences = BTreeSet::new();
    let mut preformation_event_roots = 0usize;
    let mut formed_routes = 0usize;
    let mut installed_route_cells = 0usize;
    let mut installed_route_arrows = 0usize;
    let mut exposed_handles = 0usize;
    let mut one_to_one_roots = 0usize;
    let mut physical_execution_paths = 0usize;
    let mut arrow_path_steps = 0u64;
    let mut distinct_effect_pairs = 0usize;
    let mut all_distinct = true;
    let mut all_permuted_equal = true;
    let mut relabel_ok = true;
    let mut allocation_layout_ok = true;
    let mut shuffled_propagation_ok = true;
    let mut removed_ok = true;
    let mut timing_ok = true;
    let mut symmetric_ok = true;
    let mut distractor_ok = true;
    let mut stale_ok = true;
    let mut baseline_empty = true;
    let mut installed_before_bridge = true;
    let mut work = WorkLedger::default();

    for episode in 0..evaluation_episodes {
        let ordinal = acquisition_episodes + 100 + episode;
        let mut baseline = fixture(seed, ordinal, FixtureOptions::default());
        evaluation_occurrences.extend(baseline.cells.iter().map(|cell| cell.occurrence));
        let basal_cells = baseline.cells.len();
        let basal_arrows = baseline.arrows.len();
        let existing = event_executable_roots(&baseline);
        preformation_event_roots += existing;
        baseline_empty &= existing == 0;
        let roots = learner.form_routes(&mut baseline, true);
        formed_routes += roots.len();
        installed_route_cells += baseline.cells.len() - basal_cells;
        installed_route_arrows += baseline.arrows.len() - basal_arrows;
        installed_before_bridge &= roots.len() == 2
            && baseline.cells.len() == basal_cells + roots.len() * 3
            && baseline.arrows.len() == basal_arrows + roots.len() * 2;
        distractor_ok &= roots.iter().all(|root| {
            baseline.cells[usize::from(root.cell.0)]
                .binding
                .is_some_and(|bound| baseline.event.members.contains(&bound))
        });
        let bridge = expose_routes(&roots, false, &mut learner.work);
        let permuted_bridge = expose_routes(&roots, true, &mut learner.work);
        exposed_handles += bridge.entries.len();
        let unique_roots = bridge
            .entries
            .iter()
            .map(|entry| entry.root)
            .collect::<BTreeSet<_>>();
        if unique_roots.len() == bridge.entries.len() && bridge.entries.len() == roots.len() {
            one_to_one_roots += bridge.entries.len();
        }
        let effects = bridge
            .entries
            .iter()
            .filter_map(|entry| execute_handle(&baseline, &bridge, entry.handle, &mut learner.work))
            .collect::<Vec<_>>();
        physical_execution_paths += effects.len();
        arrow_path_steps += effects.iter().map(|effect| effect.arrow_steps).sum::<u64>();
        let distinct = effects
            .iter()
            .enumerate()
            .all(|(index, effect)| effects.iter().skip(index + 1).all(|other| effect != other));
        all_distinct &= effects.len() > 1 && distinct;
        distinct_effect_pairs += effects.len() * effects.len().saturating_sub(1) / 2;
        let mut ordinary_states = effects
            .iter()
            .map(|effect| effect.state.clone())
            .collect::<Vec<_>>();
        let mut permuted_states = permuted_bridge
            .entries
            .iter()
            .filter_map(|entry| {
                execute_handle(&baseline, &permuted_bridge, entry.handle, &mut learner.work)
                    .map(|effect| effect.state)
            })
            .collect::<Vec<_>>();
        ordinary_states.sort();
        permuted_states.sort();
        all_permuted_equal &= ordinary_states == permuted_states;

        let mut relabeled = fixture(
            seed,
            ordinal,
            FixtureOptions {
                relabel: true,
                ..FixtureOptions::default()
            },
        );
        relabel_ok &= learner.form_routes(&mut relabeled, true).len() == roots.len();
        let mut allocated = fixture(
            seed,
            ordinal,
            FixtureOptions {
                allocation: true,
                layout_padding: true,
                ..FixtureOptions::default()
            },
        );
        allocation_layout_ok &= learner.form_routes(&mut allocated, true).len() == roots.len();
        let mut shuffled = fixture(
            seed,
            ordinal,
            FixtureOptions {
                shuffled_observations: true,
                ..FixtureOptions::default()
            },
        );
        shuffled_propagation_ok &= learner.form_routes(&mut shuffled, true).len() == roots.len();
        let mut removed = fixture(
            seed,
            ordinal,
            FixtureOptions {
                removed_observation: true,
                ..FixtureOptions::default()
            },
        );
        let removed_roots = learner.form_routes(&mut removed, true);
        removed_ok &= event_support_fingerprint(&baseline.event)
            == event_support_fingerprint(&removed.event)
            && removed_roots.len() + 1 == roots.len();
        let mut shifted = fixture(
            seed,
            ordinal,
            FixtureOptions {
                shifted_timing: true,
                ..FixtureOptions::default()
            },
        );
        timing_ok &= learner.form_routes(&mut shifted, true).len() == roots.len();
        let mut symmetric = fixture(
            seed,
            ordinal,
            FixtureOptions {
                symmetric: true,
                ..FixtureOptions::default()
            },
        );
        symmetric_ok &= symmetric_learner.form_routes(&mut symmetric, true).len() == roots.len();

        let mut stale_substrate = baseline.clone();
        let mut stale_roots = roots.clone();
        if let Some(root) = stale_roots.first().copied() {
            stale_substrate.cells[usize::from(root.cell.0)].generation += 1;
            remove_stale(&stale_substrate, &mut stale_roots, &mut learner.work);
            stale_ok &= stale_roots.len() + 1 == roots.len();
        } else {
            stale_ok = false;
        }
    }

    let raw_only = fixture(seed, acquisition_episodes + 900, FixtureOptions::default());
    let mut no_plasticity_substrate = raw_only.clone();
    let no_plasticity_no_routes = no_plasticity
        .form_routes(&mut no_plasticity_substrate, false)
        .is_empty()
        && event_executable_roots(&no_plasticity_substrate) == 0;
    let mut unsupported_substrate = raw_only;
    let unsupported_no_routes = unsupported
        .form_routes(&mut unsupported_substrate, true)
        .is_empty()
        && event_executable_roots(&unsupported_substrate) == 0;

    let mut cleanup_substrate =
        fixture(seed, acquisition_episodes + 901, FixtureOptions::default());
    let mut cleanup_roots = learner.form_routes(&mut cleanup_substrate, true);
    let mut cleanup_bridge = expose_routes(&cleanup_roots, false, &mut learner.work);
    learner.work.cleanup_items += (cleanup_substrate.cells.len()
        + cleanup_substrate.arrows.len()
        + cleanup_roots.len()
        + cleanup_bridge.entries.len()) as u64;
    cleanup_bridge.entries.clear();
    cleanup_roots.clear();
    cleanup_substrate.cells.clear();
    cleanup_substrate.arrows.clear();
    cleanup_substrate.raw.spikes.clear();
    cleanup_substrate.raw.propagation.clear();
    cleanup_substrate.event.members = [CellId(0); 3];
    cleanup_substrate.padding.clear();
    let cleanup_zero = cleanup_bridge.entries.is_empty()
        && cleanup_roots.is_empty()
        && cleanup_substrate.cells.is_empty()
        && cleanup_substrate.arrows.is_empty()
        && cleanup_substrate.raw.spikes.is_empty()
        && cleanup_substrate.raw.propagation.is_empty();

    work.absorb(&learner.work);
    work.absorb(&coactivity_learner.work);
    work.absorb(&symmetric_learner.work);
    work.absorb(&no_plasticity.work);
    work.absorb(&unsupported.work);
    work.temporary_peak_bytes = work.temporary_peak_bytes.max(
        size_of::<TemporarySubstrate>()
            + 12 * size_of::<Cell>()
            + 6 * size_of::<Arrow>()
            + 2 * size_of::<RouteRoot>()
            + 2 * size_of::<BridgeEntry>()
            + 257,
    );

    let controls = ControlAudit {
        fresh_disjoint: acquisition_occurrences.is_disjoint(&evaluation_occurrences),
        relabel: relabel_ok,
        allocation_layout: allocation_layout_ok,
        handle_permutation: all_permuted_equal,
        distractor_interleaving: distractor_ok,
        shuffled_coactivity: learner.fingerprint() == coactivity_learner.fingerprint(),
        shuffled_propagation: shuffled_propagation_ok,
        removed_route_lawful: removed_ok,
        timing_transfer: timing_ok,
        symmetric_unranked: symmetric_ok,
        no_plasticity_no_routes,
        unsupported_no_routes,
        stale_invalidation_removal: stale_ok,
        cleanup_zero_retained: cleanup_zero,
        independent_distinct_effects: all_distinct,
        baseline_has_no_event_routes: baseline_empty && preformation_event_roots == 0,
        learner_installs_before_bridge: installed_before_bridge,
    };
    let templates = learner.templates.len();
    let fingerprint = learner.fingerprint();
    let passed = templates >= 2
        && preformation_event_roots == 0
        && formed_routes == evaluation_episodes * 2
        && installed_route_cells == formed_routes * 3
        && installed_route_arrows == formed_routes * 2
        && exposed_handles == formed_routes
        && one_to_one_roots == exposed_handles
        && physical_execution_paths == exposed_handles
        && arrow_path_steps == (physical_execution_paths * 2) as u64
        && distinct_effect_pairs == evaluation_episodes
        && controls.passed();

    SeedAudit {
        seed,
        acquisition_episodes,
        evaluation_episodes,
        templates,
        learner_fingerprint: fingerprint,
        preformation_event_roots,
        formed_routes,
        installed_route_cells,
        installed_route_arrows,
        exposed_handles,
        one_to_one_roots,
        physical_execution_paths,
        arrow_path_steps,
        distinct_effect_pairs,
        ds1_choose_calls: source_audit.ds1_choose_calls,
        ds1_apply_calls: source_audit.ds1_apply_calls,
        post_action_consequence_paths: source_audit.post_action_consequence_paths,
        controls,
        work,
        passed,
    }
}

#[derive(Clone, Debug)]
pub struct GateReport {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub m0_authoritative: bool,
    pub enabling_ancestor_only: bool,
    pub prior_ds1_collapse_stage: usize,
    pub m1_exists: bool,
    pub a1: String,
    pub a2: String,
    pub a3: String,
    pub a4: String,
    pub b1: String,
    pub b2: String,
    pub b3: String,
    pub first_collapse: Option<String>,
    pub source_audit: SourceAudit,
    pub seeds: Vec<SeedAudit>,
    pub passed: bool,
}

fn blocked_report() -> GateReport {
    GateReport {
        label: "DS-A0 DEVELOPMENT".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "definitive-forbidden".to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        enabling_ancestor_only: true,
        prior_ds1_collapse_stage: 4,
        m1_exists: false,
        a1: "BLOCKED".to_string(),
        a2: "BLOCKED".to_string(),
        a3: "BLOCKED".to_string(),
        a4: "BLOCKED".to_string(),
        b1: "BLOCKED".to_string(),
        b2: "BLOCKED".to_string(),
        b3: "BLOCKED".to_string(),
        first_collapse: Some("NOT RUN: definitive execution forbidden".to_string()),
        source_audit: source_audit(),
        seeds: Vec::new(),
        passed: false,
    }
}

pub fn run(mode: HarnessMode) -> GateReport {
    if mode == HarnessMode::Definitive {
        return blocked_report();
    }
    let (acquisition_episodes, evaluation_episodes, seeds): (usize, usize, &[u64]) = match mode {
        HarnessMode::Micro => (16, 8, &[100]),
        HarnessMode::Gate => (32, 16, &[100, 101, 102, 103, 104]),
        HarnessMode::Definitive => unreachable!("rejected before harness"),
    };
    let source_audit = source_audit();
    let seed_audits = seeds
        .iter()
        .map(|seed| {
            audit_seed(
                *seed,
                acquisition_episodes,
                evaluation_episodes,
                &source_audit,
            )
        })
        .collect::<Vec<_>>();
    let a1 = seed_audits.iter().all(|seed| {
        seed.templates >= 2
            && seed.work.support_updates > 0
            && seed.preformation_event_roots == 0
            && seed.controls.learner_installs_before_bridge
    });
    let a2 = a1
        && seed_audits.iter().all(|seed| {
            seed.formed_routes == seed.evaluation_episodes * 2
                && seed.installed_route_arrows == seed.formed_routes * 2
        });
    let a3 = a2
        && seed_audits.iter().all(|seed| {
            seed.physical_execution_paths == seed.exposed_handles
                && seed.distinct_effect_pairs == seed.evaluation_episodes
        });
    let a4 = a3 && seed_audits.iter().all(|seed| seed.controls.passed());
    let b1 = a4
        && seed_audits.iter().all(|seed| {
            seed.one_to_one_roots == seed.exposed_handles
                && seed.exposed_handles == seed.formed_routes
        });
    let b2 = b1
        && seed_audits.iter().all(|seed| {
            seed.exposed_handles > seed.evaluation_episodes
                && seed.physical_execution_paths > 0
                && seed.arrow_path_steps > 0
        });
    let b3 = b2
        && source_audit.passed()
        && seed_audits.iter().all(|seed| {
            seed.ds1_choose_calls == 0
                && seed.ds1_apply_calls == 0
                && seed.post_action_consequence_paths == 0
        });
    let ordered = [
        ("A1 legal local candidate route formation", a1),
        (
            "A2 at least two distinct pre-existing executable routes",
            a2,
        ),
        (
            "A3 independent physical execution with distinct temporary effects",
            a3,
        ),
        ("A4 fresh/relabel/layout/control/lifetime validity", a4),
        ("B1 one-to-one opaque-handle bridge over existing roots", b1),
        (
            "B2 exposed alternatives >1 and physical execution paths >0",
            b2,
        ),
        (
            "B3 zero DS1 choice/credit calls and zero consequence paths",
            b3,
        ),
    ];
    let first_collapse = ordered
        .iter()
        .find(|(_, ready)| !ready)
        .map(|(stage, _)| (*stage).to_string());
    let passed = first_collapse.is_none() && seed_audits.iter().all(|seed| seed.passed);
    let stage = |ready: bool, prerequisite: bool| {
        if ready {
            "READY".to_string()
        } else if prerequisite {
            "COLLAPSE".to_string()
        } else {
            "BLOCKED".to_string()
        }
    };
    GateReport {
        label: if passed {
            "DS-A0 DEVELOPMENT IMPLEMENTATION READY".to_string()
        } else {
            format!(
                "DS-A0 DEVELOPMENT COLLAPSE AT {}",
                first_collapse.as_deref().unwrap_or("internal seed audit")
            )
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
        enabling_ancestor_only: true,
        prior_ds1_collapse_stage: 4,
        m1_exists: false,
        a1: stage(a1, true),
        a2: stage(a2, a1),
        a3: stage(a3, a2),
        a4: stage(a4, a3),
        b1: stage(b1, a4),
        b2: stage(b2, b1),
        b3: stage(b3, b2),
        first_collapse,
        source_audit,
        seeds: seed_audits,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_passes_all_ordered_development_stages() {
        let report = run(HarnessMode::Micro);
        assert!(report.passed, "{report:#?}");
        assert_eq!(report.a1, "READY");
        assert_eq!(report.b3, "READY");
        assert!(report.first_collapse.is_none());
    }

    #[test]
    fn plasticity_installs_routes_before_bridge_and_root_spikes_traverse_them() {
        let (mut learner, _) = acquire(100, 16, true, false);
        let mut substrate = fixture(100, 999, FixtureOptions::default());
        assert_eq!(event_executable_roots(&substrate), 0);
        let basal_cells = substrate.cells.len();
        let basal_arrows = substrate.arrows.len();
        let roots = learner.form_routes(&mut substrate, true);
        assert_eq!(roots.len(), 2);
        assert_eq!(substrate.cells.len(), basal_cells + 6);
        assert_eq!(substrate.arrows.len(), basal_arrows + 4);
        let bridge = expose_routes(&roots, true, &mut learner.work);
        let effects = bridge
            .entries
            .iter()
            .map(|entry| {
                execute_handle(&substrate, &bridge, entry.handle, &mut learner.work)
                    .expect("installed root propagates")
            })
            .collect::<Vec<_>>();
        assert_ne!(effects[0], effects[1]);
        assert!(effects.iter().all(|effect| effect.arrow_steps == 2));
    }

    #[test]
    fn identical_raw_activity_without_plastic_formation_has_no_routes() {
        let (mut learner, _) = acquire(100, 16, true, false);
        let mut substrate = fixture(100, 999, FixtureOptions::default());
        let roots = learner.form_routes(&mut substrate, false);
        assert!(roots.is_empty());
        assert_eq!(event_executable_roots(&substrate), 0);
        assert!(expose_routes(&roots, false, &mut learner.work)
            .entries
            .is_empty());
    }

    #[test]
    fn removal_and_staleness_change_installed_root_inventory() {
        let (mut learner, _) = acquire(100, 16, true, false);
        let mut complete = fixture(100, 999, FixtureOptions::default());
        let mut reduced = fixture(
            100,
            999,
            FixtureOptions {
                removed_observation: true,
                ..FixtureOptions::default()
            },
        );
        let roots = learner.form_routes(&mut complete, true);
        assert_eq!(roots.len(), 2);
        assert_eq!(learner.form_routes(&mut reduced, true).len(), 1);
        let mut stale_roots = roots;
        complete.cells[usize::from(stale_roots[0].cell.0)].generation += 1;
        remove_stale(&complete, &mut stale_roots, &mut learner.work);
        assert_eq!(stale_roots.len(), 1);
    }

    #[test]
    fn definitive_mode_is_non_claim_eligible_and_runs_no_seed() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.claim_eligible);
        assert!(report.seeds.is_empty());
        assert!(!report.passed);
    }

    #[test]
    fn forbidden_calls_and_sinks_are_derived_from_owned_source() {
        let audit = source_audit();
        assert_eq!(audit.ds1_choose_calls, 0);
        assert_eq!(audit.ds1_apply_calls, 0);
        assert_eq!(audit.post_action_consequence_paths, 0);
        assert!(audit.passed());
        let report = run(HarnessMode::Micro);
        assert!(report.seeds.iter().all(|seed| {
            seed.ds1_choose_calls == audit.ds1_choose_calls
                && seed.ds1_apply_calls == audit.ds1_apply_calls
                && seed.post_action_consequence_paths == audit.post_action_consequence_paths
        }));
    }
}
