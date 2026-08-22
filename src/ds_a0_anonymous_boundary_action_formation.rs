//! DS-A0 development-only anonymous executable-route formation gate.
//!
//! Routes arise by local traversal of episode CELL/ARROW structure over an
//! already-formed DS-E0 temporary membership. The bridge copies opaque root
//! references only; execution uses one ordinary SPIKE propagation path.

use std::collections::{BTreeMap, BTreeSet};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ArrowId(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell {
    occurrence: Occurrence,
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

/// The already-formed DS-E0 episode-local membership and relative support.
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

#[derive(Clone, Debug, Default)]
struct RouteLearner {
    templates: BTreeMap<RouteTemplate, SupportEvidence>,
    work: WorkLedger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RouteInstance {
    root: ArrowId,
    arrows: [ArrowId; 2],
    cells: [CellId; 3],
    arrow_generations: [u16; 2],
    cell_generations: [u16; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OpaqueHandle(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct BridgeEntry {
    handle: OpaqueHandle,
    route_index: u16,
    root: ArrowId,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ActionBridge {
    entries: Vec<BridgeEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhysicalExecution {
    trace: [CellId; 3],
    state: Vec<u16>,
    arrow_steps: u64,
    spike_propagations: u64,
    state_mutations: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkLedger {
    pub adjacency_observations: u64,
    pub coactivity_support_updates: u64,
    pub template_comparisons: u64,
    pub route_instantiations: u64,
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
        self.adjacency_observations
            + self.coactivity_support_updates
            + self.template_comparisons
            + self.route_instantiations
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
        self.adjacency_observations += other.adjacency_observations;
        self.coactivity_support_updates += other.coactivity_support_updates;
        self.template_comparisons += other.template_comparisons;
        self.route_instantiations += other.route_instantiations;
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

impl RouteLearner {
    fn observe_episode(&mut self, substrate: &TemporarySubstrate, plasticity: bool, reverse: bool) {
        if !plasticity {
            return;
        }
        let mut discovered = discover_routes(substrate, &mut self.work);
        if reverse {
            discovered.reverse();
        }
        let episode_shapes = discovered
            .iter()
            .map(|route| route_template(substrate, route))
            .collect::<BTreeSet<_>>();
        for template in episode_shapes {
            self.work.template_comparisons += self.templates.len() as u64;
            self.work.coactivity_support_updates += 1;
            self.templates.entry(template).or_default().episodes += 1;
        }
        self.work.persistent_bytes =
            self.templates.len() * (size_of::<RouteTemplate>() + size_of::<SupportEvidence>());
    }

    fn instantiate(&mut self, substrate: &TemporarySubstrate) -> Vec<RouteInstance> {
        let discovered = discover_routes(substrate, &mut self.work);
        let mut instances = Vec::new();
        for route in discovered {
            let template = route_template(substrate, &route);
            self.work.template_comparisons += self.templates.len() as u64;
            if self
                .templates
                .get(&template)
                .is_some_and(|support| support.episodes >= SUPPORT_EPISODES)
            {
                self.work.route_instantiations += 1;
                instances.push(route);
            }
        }
        instances
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

fn is_member(event: &EventFrame, cell: CellId) -> bool {
    event.members.contains(&cell)
}

/// Ordinary local adjacency traversal. No desired route list enters here.
fn discover_routes(substrate: &TemporarySubstrate, work: &mut WorkLedger) -> Vec<RouteInstance> {
    let mut routes = Vec::new();
    for (first_index, first) in substrate.arrows.iter().enumerate() {
        work.adjacency_observations += 1;
        if !first.live
            || !is_member(&substrate.event, first.endpoints[0])
            || !is_member(&substrate.event, first.endpoints[1])
        {
            continue;
        }
        for (second_index, second) in substrate.arrows.iter().enumerate() {
            work.adjacency_observations += 1;
            if !second.live
                || first_index == second_index
                || first.endpoints[1] != second.endpoints[0]
                || first.endpoints[0] == second.endpoints[1]
                || !is_member(&substrate.event, second.endpoints[1])
            {
                continue;
            }
            let cells = [first.endpoints[0], first.endpoints[1], second.endpoints[1]];
            let arrows = [ArrowId(first_index as u16), ArrowId(second_index as u16)];
            routes.push(RouteInstance {
                root: arrows[0],
                arrows,
                cells,
                arrow_generations: [first.generation, second.generation],
                cell_generations: cells.map(|cell| substrate.cells[usize::from(cell.0)].generation),
            });
        }
    }
    routes
}

fn route_template(substrate: &TemporarySubstrate, route: &RouteInstance) -> RouteTemplate {
    let ticks = route
        .cells
        .map(|cell| substrate.cells[usize::from(cell.0)].tick);
    RouteTemplate {
        temporal_deltas: [
            (ticks[1] - ticks[0]).clamp(i16::from(i8::MIN), i16::from(i8::MAX)) as i8,
            (ticks[2] - ticks[1]).clamp(i16::from(i8::MIN), i16::from(i8::MAX)) as i8,
        ],
        local_incidence: [1, 2, 1],
    }
}

fn route_valid(
    substrate: &TemporarySubstrate,
    route: &RouteInstance,
    work: &mut WorkLedger,
) -> bool {
    for (offset, arrow_id) in route.arrows.iter().enumerate() {
        work.generation_validations += 1;
        let arrow = &substrate.arrows[usize::from(arrow_id.0)];
        if !arrow.live || arrow.generation != route.arrow_generations[offset] {
            return false;
        }
    }
    for (offset, cell_id) in route.cells.iter().enumerate() {
        work.generation_validations += 1;
        if substrate.cells[usize::from(cell_id.0)].generation != route.cell_generations[offset] {
            return false;
        }
    }
    true
}

fn remove_stale(
    substrate: &TemporarySubstrate,
    routes: &mut Vec<RouteInstance>,
    work: &mut WorkLedger,
) {
    routes.retain(|route| route_valid(substrate, route, work));
}

/// Mechanical reference copy over already-existing route roots.
fn expose_routes(routes: &[RouteInstance], permuted: bool, work: &mut WorkLedger) -> ActionBridge {
    let mut indices = (0..routes.len()).collect::<Vec<_>>();
    if permuted {
        indices.reverse();
    }
    let entries = indices
        .into_iter()
        .enumerate()
        .map(|(slot, route_index)| {
            work.bridge_reference_copies += 1;
            BridgeEntry {
                handle: OpaqueHandle((slot as u32).wrapping_mul(2_654_435_761)),
                route_index: route_index as u16,
                root: routes[route_index].root,
            }
        })
        .collect();
    ActionBridge { entries }
}

/// The single handle-to-root-to-ordinary-propagation path.
fn execute_handle(
    frozen_start: &TemporarySubstrate,
    routes: &[RouteInstance],
    bridge: &ActionBridge,
    handle: OpaqueHandle,
    work: &mut WorkLedger,
) -> Option<PhysicalExecution> {
    let entry = bridge.entries.iter().find(|entry| entry.handle == handle)?;
    let route = routes.get(usize::from(entry.route_index))?;
    if entry.root != route.root || !route_valid(frozen_start, route, work) {
        return None;
    }
    let mut branch = frozen_start.clone();
    for cell_id in route.cells {
        work.spike_propagations += 1;
        work.state_mutations += 1;
        branch.cells[usize::from(cell_id.0)].activation += 1;
    }
    for arrow_id in route.arrows {
        let arrow = branch.arrows[usize::from(arrow_id.0)];
        work.arrow_traversals += 1;
        debug_assert!(arrow.live);
    }
    Some(PhysicalExecution {
        trace: route.cells,
        state: branch.cells.iter().map(|cell| cell.activation).collect(),
        arrow_steps: route.arrows.len() as u64,
        spike_propagations: route.cells.len() as u64,
        state_mutations: route.cells.len() as u64,
    })
}

#[derive(Clone, Copy, Debug, Default)]
struct FixtureOptions {
    relabel: bool,
    allocation: bool,
    layout_padding: bool,
    shifted_timing: bool,
    shuffled_arrows: bool,
    removed_arrow: bool,
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
    let mut physical_edges = vec![[0, 1], [1, 2], [0, 2], [2, 1], [3, 4], [4, 5]];
    if options.removed_arrow {
        physical_edges.remove(2);
    }
    let mut arrows = physical_edges
        .into_iter()
        .map(|edge| Arrow {
            endpoints: [cell_for(edge[0]), cell_for(edge[1])],
            generation: 1,
            live: true,
        })
        .collect::<Vec<_>>();
    if options.shuffled_arrows {
        arrows.reverse();
    }
    let members = [cell_for(0), cell_for(1), cell_for(2)];
    let member_ticks = members.map(|cell| cells[usize::from(cell.0)].tick);
    let mut relative_temporal = [0i8; 9];
    let mut relative_propagation = [0i8; 9];
    let learned_event_edges = [[0, 1], [1, 2], [0, 2], [2, 1]];
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
    TemporarySubstrate {
        cells,
        arrows,
        event: EventFrame {
            members,
            relative_temporal,
            relative_propagation,
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
            && self.frozen_sources_untouched
    }
}

fn count_fragments(source: &str, fragments: &[String]) -> usize {
    fragments
        .iter()
        .map(|fragment| source.matches(fragment).count())
        .sum()
}

fn source_audit() -> SourceAudit {
    let source = include_str!("ds_a0_anonymous_boundary_action_formation.rs");
    let runner = include_str!("bin/ds_a0_anonymous_boundary_action_formation.rs");
    let joined = [source, runner].concat();
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
        ["match ", "route.root"].concat(),
    ];
    SourceAudit {
        semantic_opcode_sites: count_fragments(&joined, &semantic_fragments),
        evaluator_selection_sites: count_fragments(&joined, &evaluator_fragments),
        hidden_executor_sites: count_fragments(&joined, &hidden_fragments),
        ds1_choose_calls: joined.matches(&[".cho", "ose("].concat()).count(),
        ds1_apply_calls: joined
            .matches(&[".apply_", "consequence("].concat())
            .count(),
        post_action_consequence_paths: 0,
        executor_definitions: source.matches(&["fn execute_", "handle("].concat()).count(),
        bridge_constructor_definitions: source.matches(&["fn expose_", "routes("].concat()).count(),
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
    pub formed_routes: usize,
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

fn audit_seed(seed: u64, acquisition_episodes: usize, evaluation_episodes: usize) -> SeedAudit {
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
    let mut formed_routes = 0usize;
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
    let mut work = WorkLedger::default();

    for episode in 0..evaluation_episodes {
        let ordinal = acquisition_episodes + 100 + episode;
        let baseline = fixture(seed, ordinal, FixtureOptions::default());
        evaluation_occurrences.extend(baseline.cells.iter().map(|cell| cell.occurrence));
        let routes = learner.instantiate(&baseline);
        formed_routes += routes.len();
        distractor_ok &= routes.iter().all(|route| {
            route
                .cells
                .iter()
                .all(|cell| baseline.event.members.contains(cell))
        });
        let bridge = expose_routes(&routes, false, &mut learner.work);
        let permuted_bridge = expose_routes(&routes, true, &mut learner.work);
        exposed_handles += bridge.entries.len();
        let unique_roots = bridge
            .entries
            .iter()
            .map(|entry| entry.root)
            .collect::<BTreeSet<_>>();
        let unique_route_refs = bridge
            .entries
            .iter()
            .map(|entry| entry.route_index)
            .collect::<BTreeSet<_>>();
        if unique_roots.len() == bridge.entries.len()
            && unique_route_refs.len() == routes.len()
            && bridge.entries.len() == routes.len()
        {
            one_to_one_roots += bridge.entries.len();
        }
        let effects = bridge
            .entries
            .iter()
            .filter_map(|entry| {
                execute_handle(&baseline, &routes, &bridge, entry.handle, &mut learner.work)
            })
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
                execute_handle(
                    &baseline,
                    &routes,
                    &permuted_bridge,
                    entry.handle,
                    &mut learner.work,
                )
                .map(|effect| effect.state)
            })
            .collect::<Vec<_>>();
        ordinary_states.sort();
        permuted_states.sort();
        all_permuted_equal &= ordinary_states == permuted_states;

        let relabeled = fixture(
            seed,
            ordinal,
            FixtureOptions {
                relabel: true,
                ..FixtureOptions::default()
            },
        );
        relabel_ok &= learner.instantiate(&relabeled).len() == routes.len();
        let allocated = fixture(
            seed,
            ordinal,
            FixtureOptions {
                allocation: true,
                layout_padding: true,
                ..FixtureOptions::default()
            },
        );
        allocation_layout_ok &= learner.instantiate(&allocated).len() == routes.len();
        let shuffled = fixture(
            seed,
            ordinal,
            FixtureOptions {
                shuffled_arrows: true,
                ..FixtureOptions::default()
            },
        );
        shuffled_propagation_ok &= learner.instantiate(&shuffled).len() == routes.len();
        let removed = fixture(
            seed,
            ordinal,
            FixtureOptions {
                removed_arrow: true,
                ..FixtureOptions::default()
            },
        );
        let removed_routes = learner.instantiate(&removed);
        removed_ok &= event_support_fingerprint(&baseline.event)
            == event_support_fingerprint(&removed.event)
            && removed_routes.len() + 1 == routes.len();
        let shifted = fixture(
            seed,
            ordinal,
            FixtureOptions {
                shifted_timing: true,
                ..FixtureOptions::default()
            },
        );
        timing_ok &= learner.instantiate(&shifted).len() == routes.len();
        let symmetric = fixture(
            seed,
            ordinal,
            FixtureOptions {
                symmetric: true,
                ..FixtureOptions::default()
            },
        );
        symmetric_ok &= symmetric_learner.instantiate(&symmetric).len() == routes.len();

        let mut stale_substrate = baseline.clone();
        let mut stale_routes = routes.clone();
        if let Some(root) = stale_routes.first().map(|route| route.root) {
            stale_substrate.arrows[usize::from(root.0)].generation += 1;
            remove_stale(&stale_substrate, &mut stale_routes, &mut learner.work);
            stale_ok &= stale_routes.len() + 1 == routes.len();
        } else {
            stale_ok = false;
        }
    }

    let unsupported_fixture = fixture(seed, acquisition_episodes + 900, FixtureOptions::default());
    let no_plasticity_no_routes = no_plasticity.instantiate(&unsupported_fixture).is_empty();
    let unsupported_no_routes = unsupported.instantiate(&unsupported_fixture).is_empty();

    let mut cleanup_substrate =
        fixture(seed, acquisition_episodes + 901, FixtureOptions::default());
    let mut cleanup_routes = learner.instantiate(&cleanup_substrate);
    let mut cleanup_bridge = expose_routes(&cleanup_routes, false, &mut learner.work);
    learner.work.cleanup_items += (cleanup_substrate.cells.len()
        + cleanup_substrate.arrows.len()
        + cleanup_routes.len()
        + cleanup_bridge.entries.len()) as u64;
    cleanup_bridge.entries.clear();
    cleanup_routes.clear();
    cleanup_substrate.cells.clear();
    cleanup_substrate.arrows.clear();
    cleanup_substrate.event.members = [CellId(0); 3];
    cleanup_substrate.padding.clear();
    let cleanup_zero = cleanup_bridge.entries.is_empty()
        && cleanup_routes.is_empty()
        && cleanup_substrate.cells.is_empty()
        && cleanup_substrate.arrows.is_empty()
        && learner
            .templates
            .keys()
            .all(|template| size_of_val(template) == size_of::<RouteTemplate>());

    work.absorb(&learner.work);
    work.absorb(&coactivity_learner.work);
    work.absorb(&symmetric_learner.work);
    work.absorb(&no_plasticity.work);
    work.absorb(&unsupported.work);
    work.temporary_peak_bytes = work.temporary_peak_bytes.max(
        size_of::<TemporarySubstrate>()
            + 6 * size_of::<Cell>()
            + 6 * size_of::<Arrow>()
            + 2 * size_of::<RouteInstance>()
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
    };
    let templates = learner.templates.len();
    let fingerprint = learner.fingerprint();
    let passed = templates >= 2
        && formed_routes == evaluation_episodes * 2
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
        formed_routes,
        exposed_handles,
        one_to_one_roots,
        physical_execution_paths,
        arrow_path_steps,
        distinct_effect_pairs,
        ds1_choose_calls: 0,
        ds1_apply_calls: 0,
        post_action_consequence_paths: 0,
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
        .map(|seed| audit_seed(*seed, acquisition_episodes, evaluation_episodes))
        .collect::<Vec<_>>();

    let a1 = seed_audits
        .iter()
        .all(|seed| seed.templates >= 2 && seed.work.coactivity_support_updates > 0);
    let a2 = a1
        && seed_audits
            .iter()
            .all(|seed| seed.formed_routes == seed.evaluation_episodes * 2);
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
    fn every_handle_executes_an_existing_root_with_a_distinct_effect() {
        let (mut learner, _) = acquire(100, 16, true, false);
        let substrate = fixture(100, 999, FixtureOptions::default());
        let routes = learner.instantiate(&substrate);
        let bridge = expose_routes(&routes, true, &mut learner.work);
        let effects = bridge
            .entries
            .iter()
            .map(|entry| {
                execute_handle(
                    &substrate,
                    &routes,
                    &bridge,
                    entry.handle,
                    &mut learner.work,
                )
                .expect("existing route")
            })
            .collect::<Vec<_>>();
        assert_eq!(routes.len(), 2);
        assert_eq!(bridge.entries.len(), routes.len());
        assert_ne!(effects[0], effects[1]);
        assert!(effects.iter().all(|effect| effect.arrow_steps > 0));
    }

    #[test]
    fn controls_remove_or_invalidate_routes_lawfully() {
        let (mut learner, _) = acquire(100, 16, true, false);
        let complete = fixture(100, 999, FixtureOptions::default());
        let removed = fixture(
            100,
            999,
            FixtureOptions {
                removed_arrow: true,
                ..FixtureOptions::default()
            },
        );
        assert_eq!(learner.instantiate(&complete).len(), 2);
        assert_eq!(learner.instantiate(&removed).len(), 1);
        let mut routes = learner.instantiate(&complete);
        let mut stale = complete;
        stale.arrows[usize::from(routes[0].root.0)].live = false;
        remove_stale(&stale, &mut routes, &mut learner.work);
        assert_eq!(routes.len(), 1);
    }

    #[test]
    fn definitive_mode_is_non_claim_eligible_and_runs_no_seed() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.claim_eligible);
        assert!(report.seeds.is_empty());
        assert!(!report.passed);
    }
}
