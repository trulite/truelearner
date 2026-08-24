//! Development-only byte-identical frozen DS1 after DS-E0 + DS-A0 retry.
//!
//! The primary path exports one actual E0 episode into the A0 composition
//! scope by one-to-one field copies. No independent A0 fixture, semantic
//! translation, post-action consequence, or DS1 update is present.

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds1-after-e0-a0-composition-retry-v2";
pub const EXACT_PARENT: &str = "ad503db3855884b8cab85903edfacfc753fa746e";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_DS_E0_SHA256: &str =
    "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615";
pub const FROZEN_DS_A0_SHA256: &str =
    "3eb802f394a225a4ad7f0938b4a672723da2c1303ff95e805423de8161057527";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_M0_SHA256: &str =
    "50cf169bb293177a35270adde656f28f98e68c83a4d39d2876399261b7ee697c";
pub const FROZEN_COMPILED_M0_SHA256: &str =
    "430cd2206c8baa7106c4de7f203d4d0c48b544290e6266596ebcdb91d02655c9";
pub const PRIOR_STAGE_FOUR_SOURCE_SHA256: &str =
    "a4deadedfde7b9896d64d0cacd41560441ea85cf3bda119a5d09aa3aaddcd7a0";

const STAGE_NAMES: [&str; 12] = [
    "0. exact lineage and immutable hashes",
    "1. actual DS-E0 learned temporary relational structure exists",
    "2. format-only E0 serialization is sufficient and frozen DS1 consumes it",
    "3. actual DS-A0 plastic route formation occurs before bridge",
    "4. opaque alternatives >1 are one-to-one real roots at the DS1 choice surface",
    "5. frozen DS1 chooses an opaque alternative",
    "6. chosen root executes by ordinary live CELL/ARROW/SPIKE propagation",
    "7. natural post-action consequence is visible through existing parent substrate",
    "8. frozen DS1 updates through unchanged apply_consequence",
    "9. held-out boundary-role reconstruction and functional controls",
    "10. invalidation, reopening, and reconsolidation",
    "11. naturally available recursive compatibility/economics",
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
pub struct E0CompositionEvidence {
    pub learned_shapes: usize,
    pub learned_event_formed: bool,
    pub serializer_exact_copy: bool,
    pub export_exact_copy: bool,
    pub frozen_read_only_consumptions: u64,
    pub fresh_occurrences: bool,
    pub support_exports: usize,
    pub temporary_formations: u64,
    pub serializations: u64,
    pub physical_work: u64,
    pub persistent_bytes: usize,
    pub temporary_bytes: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChoiceEvidence {
    pub choice: Option<usize>,
    pub mature: bool,
    pub runtime_choose_calls: u64,
    pub runtime_credit_updates: u64,
    pub comparisons: u64,
    pub candidate_evaluations: u64,
    pub proposals: u64,
    pub route_firings: u64,
    pub persistent_bytes: usize,
}

macro_rules! e0_composition_access {
    () => {
        pub(super) struct CompositionBundle {
            support: Vec<super::E0EpisodeExport>,
            target: super::E0EpisodeExport,
            neighborhood: Neighborhood,
            learner: Learner,
            evidence: super::E0CompositionEvidence,
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
                && export
                    .pulses
                    .iter()
                    .zip(&raw.spikes)
                    .all(|(copied, source)| {
                        copied.occurrence == source.occurrence.0 && copied.tick == source.local_tick
                    })
                && export.observed_propagation.len() == raw.propagation.len()
                && export
                    .observed_propagation
                    .iter()
                    .zip(&raw.propagation)
                    .all(|(copied, source)| *copied == [source.from.0, source.to.0])
                && export.members == event.members.map(|member| member.0)
                && export.relative_temporal == event.temporal
                && export.relative_propagation == event.propagation
        }

        impl CompositionBundle {
            pub(super) fn evidence(&self) -> &super::E0CompositionEvidence {
                &self.evidence
            }

            pub(super) fn support(&self) -> &[super::E0EpisodeExport] {
                &self.support
            }

            pub(super) fn target(&self) -> &super::E0EpisodeExport {
                &self.target
            }

            pub(super) fn choose_opaque(&mut self, tie_breaker: usize) -> super::ChoiceEvidence {
                let before_updates = self.learner.credit_updates;
                let before_firings = self.learner.route_firings;
                let (choice, mature) = self.learner.choose(&self.neighborhood, tie_breaker);
                super::ChoiceEvidence {
                    choice: Some(choice),
                    mature,
                    runtime_choose_calls: self.learner.route_firings - before_firings,
                    runtime_credit_updates: self.learner.credit_updates - before_updates,
                    comparisons: self.learner.comparisons,
                    candidate_evaluations: self.learner.candidate_evaluations,
                    proposals: self.learner.proposals,
                    route_firings: self.learner.route_firings,
                    persistent_bytes: self.learner.persistent_bytes(),
                }
            }
        }

        pub(super) fn composition_bundle(
            seed: u64,
            acquisition: usize,
            support_presentations: usize,
        ) -> Option<CompositionBundle> {
            let (mut formation, acquisition_occurrences) = acquire(seed, acquisition);
            let before_formations = formation.work.temporary_formations;
            let mut support = Vec::new();
            let mut export_exact_copy = true;
            for ordinal in 0..support_presentations {
                let episode = fixture(seed + 1_000, acquisition + ordinal, 0, Perturbation::None);
                let event = formation.form(&episode.raw)?;
                let export = export_episode(&episode.raw, &event);
                export_exact_copy &= export_matches(&export, &episode.raw, &event);
                support.push(export);
            }
            let target_episode = fixture(
                seed + 2_000,
                acquisition + support_presentations + 17,
                0,
                Perturbation::None,
            );
            let target_occurrences = target_episode
                .raw
                .spikes
                .iter()
                .map(|spike| spike.occurrence)
                .collect::<BTreeSet<_>>();
            let target_event = formation.form(&target_episode.raw)?;
            let learned_event_formed =
                members_set(&target_event.members) == target_episode.selected;
            let target = export_episode(&target_episode.raw, &target_event);
            export_exact_copy &= export_matches(&target, &target_episode.raw, &target_event);
            let before_serializations = formation.work.serializations;
            let neighborhood = serialize_once(&target_event, &mut formation.work);
            let serializer_exact_copy = [
                neighborhood.pair[0].identity.0 as u32,
                neighborhood.pair[1].identity.0 as u32,
                neighborhood.witness.identity.0 as u32,
            ] == target_event.members.map(|member| member.0)
                && [
                    neighborhood.pair[0].position,
                    neighborhood.pair[1].position,
                    neighborhood.witness.position,
                ] == target_event.propagation_rank
                && [
                    neighborhood.pair[0].tick,
                    neighborhood.pair[1].tick,
                    neighborhood.witness.tick,
                ] == target_event.temporal_rank
                && [
                    neighborhood.pair[0].window,
                    neighborhood.pair[1].window,
                    neighborhood.witness.window,
                ] == target_event.attachment_equivalence;
            let learner = Learner::default();
            let frozen_read_only_consumptions =
                u64::from(learner.frozen_choice(&neighborhood).is_none());
            Some(CompositionBundle {
                support,
                target,
                neighborhood,
                learner,
                evidence: super::E0CompositionEvidence {
                    learned_shapes: formation.shapes.len(),
                    learned_event_formed,
                    serializer_exact_copy,
                    export_exact_copy,
                    frozen_read_only_consumptions,
                    fresh_occurrences: acquisition_occurrences.is_disjoint(&target_occurrences),
                    support_exports: support_presentations,
                    temporary_formations: formation.work.temporary_formations - before_formations,
                    serializations: formation.work.serializations - before_serializations,
                    physical_work: formation.work.organism_work(),
                    persistent_bytes: formation.persistent_bytes(),
                    temporary_bytes: size_of::<EventRelations>()
                        + size_of::<Neighborhood>()
                        + size_of::<RawActivity>(),
                },
            })
        }

        pub(super) fn frozen_choice_arity() -> usize {
            Pattern {
                signature: Signature {
                    gap: 0,
                    witness_attachment: 0,
                },
                evidence: [Evidence::default(); 2],
                mature: None,
                contradictions: 0,
            }
            .evidence
            .len()
        }
    };
}

#[allow(dead_code)]
mod frozen_e0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_e0_anonymous_event_formation.rs"
    ));
    e0_composition_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct A0CompositionEvidence {
    pub input_pulses: usize,
    pub mapped_pulses: usize,
    pub input_propagations: usize,
    pub mapped_propagations: usize,
    pub exact_e0_episode_copy: bool,
    pub templates: usize,
    pub preformation_event_roots: usize,
    pub prebridge_roots: usize,
    pub installed_cells: usize,
    pub installed_arrows: usize,
    pub handles: usize,
    pub unique_roots: usize,
    pub prechoice_physical_effects: usize,
    pub distinct_prechoice_effects: usize,
    pub installed_before_bridge: bool,
    pub bridge_one_to_one: bool,
    pub physical_effects_distinct: bool,
    pub selected_executions: u64,
    pub selected_arrow_steps: u64,
    pub selected_spike_propagations: u64,
    pub selected_state_mutations: u64,
    pub selected_effect_fingerprint: Option<u64>,
    pub persistent_bytes: usize,
    pub temporary_peak_bytes: usize,
    pub physical_work: u64,
}

fn effect_fingerprint(state: &[u16], trace: &[u16]) -> u64 {
    state
        .iter()
        .map(|value| u64::from(*value))
        .chain(trace.iter().map(|value| u64::from(*value)))
        .fold(0xcbf2_9ce4_8422_2325u64, |mut hash, value| {
            hash ^= value;
            hash.wrapping_mul(0x100_0000_01b3)
        })
}

macro_rules! a0_composition_access {
    () => {
        pub(super) struct CompositionActions {
            substrate: TemporarySubstrate,
            bridge: ActionBridge,
            work: WorkLedger,
            evidence: super::A0CompositionEvidence,
            prechoice_fingerprints: BTreeSet<u64>,
        }

        fn substrate_from_e0(export: &super::E0EpisodeExport) -> Option<TemporarySubstrate> {
            let cells = export
                .pulses
                .iter()
                .map(|pulse| Cell {
                    occurrence: Occurrence(pulse.occurrence),
                    binding: None,
                    tick: i16::from(pulse.tick),
                    generation: 1,
                    activation: 0,
                })
                .collect::<Vec<_>>();
            let cell_for = |occurrence: u32| {
                cells
                    .iter()
                    .position(|cell| cell.occurrence == Occurrence(occurrence))
                    .map(|index| CellId(index as u16))
            };
            let members = [
                cell_for(export.members[0])?,
                cell_for(export.members[1])?,
                cell_for(export.members[2])?,
            ];
            let spikes = export
                .pulses
                .iter()
                .map(|pulse| {
                    Some(ObservedSpike {
                        cell: cell_for(pulse.occurrence)?,
                        tick: i16::from(pulse.tick),
                    })
                })
                .collect::<Option<Vec<_>>>()?;
            let propagation = export
                .observed_propagation
                .iter()
                .map(|edge| {
                    Some(ObservedPropagation {
                        endpoints: [cell_for(edge[0])?, cell_for(edge[1])?],
                    })
                })
                .collect::<Option<Vec<_>>>()?;
            Some(TemporarySubstrate {
                cells,
                arrows: Vec::new(),
                event: EventFrame {
                    members,
                    relative_temporal: export.relative_temporal,
                    relative_propagation: export.relative_propagation,
                },
                raw: RawActivity {
                    spikes,
                    propagation,
                },
                padding: Vec::new(),
            })
        }

        fn substrate_matches_e0(
            substrate: &TemporarySubstrate,
            export: &super::E0EpisodeExport,
        ) -> bool {
            substrate.cells.len() == export.pulses.len()
                && substrate
                    .cells
                    .iter()
                    .zip(&export.pulses)
                    .all(|(mapped, source)| {
                        mapped.occurrence.0 == source.occurrence
                            && mapped.tick == i16::from(source.tick)
                    })
                && substrate.raw.propagation.len() == export.observed_propagation.len()
                && substrate
                    .raw
                    .propagation
                    .iter()
                    .zip(&export.observed_propagation)
                    .all(|(mapped, source)| {
                        [
                            substrate.cells[usize::from(mapped.endpoints[0].0)]
                                .occurrence
                                .0,
                            substrate.cells[usize::from(mapped.endpoints[1].0)]
                                .occurrence
                                .0,
                        ] == *source
                    })
                && substrate
                    .event
                    .members
                    .map(|cell| substrate.cells[usize::from(cell.0)].occurrence.0)
                    == export.members
                && substrate.event.relative_temporal == export.relative_temporal
                && substrate.event.relative_propagation == export.relative_propagation
        }

        impl CompositionActions {
            pub(super) fn evidence(&self) -> &super::A0CompositionEvidence {
                &self.evidence
            }

            pub(super) fn prechoice_fingerprints(&self) -> &BTreeSet<u64> {
                &self.prechoice_fingerprints
            }

            pub(super) fn execute_choice(&mut self, choice: usize) -> bool {
                let Some(entry) = self.bridge.entries.get(choice) else {
                    return false;
                };
                let Some(effect) =
                    execute_handle(&self.substrate, &self.bridge, entry.handle, &mut self.work)
                else {
                    return false;
                };
                let trace = effect.trace.iter().map(|cell| cell.0).collect::<Vec<_>>();
                let fingerprint = super::effect_fingerprint(&effect.state, &trace);
                self.evidence.selected_executions += 1;
                self.evidence.selected_arrow_steps += effect.arrow_steps;
                self.evidence.selected_spike_propagations += effect.spike_propagations;
                self.evidence.selected_state_mutations += effect.state_mutations;
                self.evidence.selected_effect_fingerprint = Some(fingerprint);
                self.evidence.physical_work = self.work.physical_work();
                self.prechoice_fingerprints.contains(&fingerprint)
            }
        }

        pub(super) fn support_requirement() -> usize {
            usize::from(SUPPORT_EPISODES)
        }

        pub(super) fn composition_actions(
            support: &[super::E0EpisodeExport],
            target: &super::E0EpisodeExport,
            permuted: bool,
        ) -> Option<CompositionActions> {
            let mut learner = RouteLearner::default();
            for export in support {
                let substrate = substrate_from_e0(export)?;
                learner.observe_episode(&substrate, true, false);
            }
            let mut substrate = substrate_from_e0(target)?;
            let exact_e0_episode_copy = substrate_matches_e0(&substrate, target);
            let input_pulses = target.pulses.len();
            let input_propagations = target.observed_propagation.len();
            let mapped_pulses = substrate.cells.len();
            let mapped_propagations = substrate.raw.propagation.len();
            let preformation_event_roots = event_executable_roots(&substrate);
            let basal_cells = substrate.cells.len();
            let basal_arrows = substrate.arrows.len();
            let roots = learner.form_routes(&mut substrate, true);
            let prebridge_roots = roots.len();
            let installed_cells = substrate.cells.len() - basal_cells;
            let installed_arrows = substrate.arrows.len() - basal_arrows;
            let bridge = expose_routes(&roots, permuted, &mut learner.work);
            let unique_roots = bridge
                .entries
                .iter()
                .map(|entry| entry.root)
                .collect::<BTreeSet<_>>()
                .len();
            let effects = bridge
                .entries
                .iter()
                .filter_map(|entry| {
                    execute_handle(&substrate, &bridge, entry.handle, &mut learner.work)
                })
                .collect::<Vec<_>>();
            let prechoice_fingerprints = effects
                .iter()
                .map(|effect| {
                    let trace = effect.trace.iter().map(|cell| cell.0).collect::<Vec<_>>();
                    super::effect_fingerprint(&effect.state, &trace)
                })
                .collect::<BTreeSet<_>>();
            let handles = bridge.entries.len();
            let temporary_peak_bytes = size_of::<TemporarySubstrate>()
                + substrate.cells.len() * size_of::<Cell>()
                + substrate.arrows.len() * size_of::<Arrow>();
            Some(CompositionActions {
                substrate,
                bridge,
                work: learner.work.clone(),
                evidence: super::A0CompositionEvidence {
                    input_pulses,
                    mapped_pulses,
                    input_propagations,
                    mapped_propagations,
                    exact_e0_episode_copy,
                    templates: learner.templates.len(),
                    preformation_event_roots,
                    prebridge_roots,
                    installed_cells,
                    installed_arrows,
                    handles,
                    unique_roots,
                    prechoice_physical_effects: effects.len(),
                    distinct_prechoice_effects: prechoice_fingerprints.len(),
                    installed_before_bridge: preformation_event_roots == 0
                        && prebridge_roots > 0
                        && installed_cells == prebridge_roots * 3
                        && installed_arrows == prebridge_roots * 2,
                    bridge_one_to_one: handles == prebridge_roots && unique_roots == handles,
                    physical_effects_distinct: effects.len() > 1
                        && prechoice_fingerprints.len() == effects.len(),
                    selected_executions: 0,
                    selected_arrow_steps: 0,
                    selected_spike_propagations: 0,
                    selected_state_mutations: 0,
                    selected_effect_fingerprint: None,
                    persistent_bytes: learner.work.persistent_bytes,
                    temporary_peak_bytes,
                    physical_work: learner.work.physical_work(),
                },
                prechoice_fingerprints,
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_a0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_a0_anonymous_boundary_action_formation.rs"
    ));
    a0_composition_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PathInventory {
    pub frozen_choose_definitions: usize,
    pub frozen_choose_call_edges: usize,
    pub choice_to_execution_edges: usize,
    pub post_execution_observation_edges: usize,
    pub apply_definitions: usize,
    pub apply_call_edges: usize,
    pub parent_consequence_paths: usize,
    pub runtime_choose_calls: u64,
    pub runtime_selected_executions: u64,
    pub runtime_consequence_visibility_events: u64,
    pub runtime_apply_updates: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LeakAudit {
    pub e0_persistent_occurrence_fields: usize,
    pub a0_persistent_occurrence_fields: usize,
    pub concrete_destination_fields: usize,
    pub filler_or_evaluator_fields: usize,
    pub integrating_semantic_translation_sites: usize,
    pub passed: bool,
}

fn function_body<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("fn {name}");
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

fn struct_body<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("struct {name}");
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
                    return Some(&tail[..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn derive_source_paths_from(composition: &str, e0: &str, a0: &str) -> PathInventory {
    let choose_wrapper = function_body(composition, "choose_opaque").unwrap_or_default();
    let execution_wrapper = function_body(composition, "execute_choice").unwrap_or_default();
    let production = composition
        .split("#[cfg(test)]")
        .next()
        .unwrap_or(composition);
    let apply_pattern = [".apply_", "consequence("].concat();
    let consequence_observer_pattern = ["observe_", "post_action("].concat();
    let a0_executor = function_body(a0, "execute_handle").unwrap_or_default();
    let consequence_fragments = ["consequence", "credit", "reward", "terminal"];
    PathInventory {
        frozen_choose_definitions: e0.matches("fn choose(").count(),
        frozen_choose_call_edges: choose_wrapper.matches(".choose(").count(),
        choice_to_execution_edges: execution_wrapper.matches("execute_handle(").count(),
        post_execution_observation_edges: execution_wrapper
            .matches(&consequence_observer_pattern)
            .count(),
        apply_definitions: e0.matches("fn apply_consequence(").count(),
        apply_call_edges: production.matches(&apply_pattern).count(),
        parent_consequence_paths: consequence_fragments
            .iter()
            .map(|fragment| a0_executor.matches(fragment).count())
            .sum(),
        ..PathInventory::default()
    }
}

fn derive_source_paths(composition: &str) -> PathInventory {
    derive_source_paths_from(
        composition,
        include_str!("ds_e0_anonymous_event_formation.rs"),
        include_str!("ds_a0_anonymous_boundary_action_formation.rs"),
    )
}

fn leak_audit() -> LeakAudit {
    let e0 = include_str!("ds_e0_anonymous_event_formation.rs");
    let a0 = include_str!("ds_a0_anonymous_boundary_action_formation.rs");
    let composition = include_str!("ds1_after_e0_a0_composition_retry.rs");
    let e0_persistent = ["Learner", "Pattern", "FormationLearner", "ShapeEvidence"]
        .iter()
        .filter_map(|name| struct_body(e0, name))
        .collect::<String>();
    let a0_persistent = ["RouteLearner", "RouteTemplate", "SupportEvidence"]
        .iter()
        .filter_map(|name| struct_body(a0, name))
        .collect::<String>();
    let wiring = [
        function_body(composition, "export_episode").unwrap_or_default(),
        function_body(composition, "substrate_from_e0").unwrap_or_default(),
        function_body(composition, "choose_opaque").unwrap_or_default(),
        function_body(composition, "execute_choice").unwrap_or_default(),
    ]
    .concat();
    let semantic_fragments = [
        "expected_action",
        "correct_route",
        "action_opcode",
        "match handle",
    ];
    let e0_occurrences = e0_persistent.matches("Occurrence").count();
    let a0_occurrences = a0_persistent.matches("Occurrence").count();
    let destinations =
        e0_persistent.matches("destination").count() + a0_persistent.matches("destination").count();
    let filler_evaluator = ["filler", "evaluator"]
        .iter()
        .map(|fragment| {
            e0_persistent.matches(fragment).count() + a0_persistent.matches(fragment).count()
        })
        .sum();
    let translation = semantic_fragments
        .iter()
        .map(|fragment| wiring.matches(fragment).count())
        .sum();
    LeakAudit {
        e0_persistent_occurrence_fields: e0_occurrences,
        a0_persistent_occurrence_fields: a0_occurrences,
        concrete_destination_fields: destinations,
        filler_or_evaluator_fields: filler_evaluator,
        integrating_semantic_translation_sites: translation,
        passed: e0_occurrences + a0_occurrences + destinations + filler_evaluator + translation
            == 0,
    }
}

#[derive(Clone, Debug)]
pub struct SeedAudit {
    pub seed: u64,
    pub e0: E0CompositionEvidence,
    pub choice: ChoiceEvidence,
    pub a0: A0CompositionEvidence,
    pub permuted_a0: A0CompositionEvidence,
    pub choice_arity: usize,
    pub permutation_only_changes_actual_route: Option<bool>,
    pub stage_ready: [bool; 12],
    pub paths: PathInventory,
}

#[derive(Clone, Debug)]
pub struct CompositionReport {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub m0_authoritative: bool,
    pub m1_exists: bool,
    pub stages: [String; 12],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub paths: PathInventory,
    pub leaks: LeakAudit,
    pub seeds: Vec<SeedAudit>,
    pub audit_passed: bool,
}

fn source_boundary_ready() -> bool {
    let e0 = include_str!("ds_e0_anonymous_event_formation.rs");
    let a0 = include_str!("ds_a0_anonymous_boundary_action_formation.rs");
    crate::ds_e0_anonymous_event_formation::FROZEN_DS1_LEARNER_SHA256 == FROZEN_DS1_SHA256
        && e0.matches("// DS1_LEARNER_BEGIN").count() == 2
        && e0.matches("// DS1_LEARNER_END").count() == 2
        && e0.matches("fn serialize_once(").count() == 1
        && a0.matches("fn expose_routes(").count() == 1
        && a0.matches("fn execute_handle(").count() == 1
}

fn ordered_freeze(ready: [bool; 12]) -> ([String; 12], Option<usize>) {
    let first = ready.iter().position(|stage| !stage);
    let statuses = std::array::from_fn(|stage| match first {
        None => "READY".to_string(),
        Some(collapse) if stage < collapse => "READY".to_string(),
        Some(collapse) if stage == collapse => format!("COLLAPSE: {}", STAGE_NAMES[stage]),
        Some(_) => "BLOCKED".to_string(),
    });
    (statuses, first)
}

fn audit_seed(
    seed: u64,
    acquisition: usize,
    source_ready: bool,
    source_paths: &PathInventory,
) -> SeedAudit {
    let support_requirement = frozen_a0::support_requirement();
    let mut bundle = frozen_e0::composition_bundle(seed, acquisition, support_requirement)
        .expect("frozen E0 learned relation remains available");
    let e0 = bundle.evidence().clone();
    let choice_arity = frozen_e0::frozen_choice_arity();
    let mut actions = frozen_a0::composition_actions(bundle.support(), bundle.target(), false)
        .expect("exact E0 episode maps into frozen A0 temporary substrate");
    let mut permuted = frozen_a0::composition_actions(bundle.support(), bundle.target(), true)
        .expect("same exact E0 episode maps into permuted bridge control");

    let stage_one = e0.learned_event_formed && e0.temporary_formations > 0;
    let stage_two = stage_one
        && e0.serializer_exact_copy
        && e0.export_exact_copy
        && e0.frozen_read_only_consumptions == 1
        && e0.serializations == 1;
    let stage_three = stage_two
        && actions.evidence().exact_e0_episode_copy
        && actions.evidence().input_pulses == actions.evidence().mapped_pulses
        && actions.evidence().input_propagations == actions.evidence().mapped_propagations
        && actions.evidence().installed_before_bridge;
    let stage_four = stage_three
        && actions.evidence().bridge_one_to_one
        && actions.evidence().handles == choice_arity
        && actions.evidence().handles > 1
        && actions.evidence().physical_effects_distinct
        && actions.prechoice_fingerprints() == permuted.prechoice_fingerprints();

    let choice = if stage_four {
        bundle.choose_opaque(seed as usize)
    } else {
        ChoiceEvidence::default()
    };
    let stage_five = stage_four
        && choice.runtime_choose_calls == 1
        && choice
            .choice
            .is_some_and(|index| index < actions.evidence().handles);
    let (ordinary_selected, permuted_selected) = if stage_five {
        let index = choice.choice.expect("stage five established a choice");
        (
            actions.execute_choice(index),
            permuted.execute_choice(index),
        )
    } else {
        (false, false)
    };
    let a0 = actions.evidence().clone();
    let permuted_a0 = permuted.evidence().clone();
    let permutation_only_changes_actual_route = stage_five.then(|| {
        ordinary_selected
            && permuted_selected
            && a0.selected_effect_fingerprint.is_some()
            && permuted_a0.selected_effect_fingerprint.is_some()
            && a0.selected_effect_fingerprint != permuted_a0.selected_effect_fingerprint
            && a0
                .selected_effect_fingerprint
                .is_some_and(|fingerprint| actions.prechoice_fingerprints().contains(&fingerprint))
            && permuted_a0
                .selected_effect_fingerprint
                .is_some_and(|fingerprint| permuted.prechoice_fingerprints().contains(&fingerprint))
    });
    let stage_six = stage_five
        && a0.selected_executions == 1
        && a0.selected_arrow_steps > 0
        && a0.selected_spike_propagations > 0
        && permutation_only_changes_actual_route == Some(true);
    let runtime_consequence_visibility_events = (source_paths
        .post_execution_observation_edges
        .min(source_paths.parent_consequence_paths)
        as u64)
        .saturating_mul(
            actions.evidence().selected_executions + permuted.evidence().selected_executions,
        );
    let stage_seven = stage_six
        && source_paths.post_execution_observation_edges > 0
        && source_paths.parent_consequence_paths > 0
        && runtime_consequence_visibility_events > 0;
    let stage_eight =
        stage_seven && source_paths.apply_call_edges > 0 && choice.runtime_credit_updates > 0;
    let stage_ready = [
        source_ready,
        stage_one,
        stage_two,
        stage_three,
        stage_four,
        stage_five,
        stage_six,
        stage_seven,
        stage_eight,
        false,
        false,
        false,
    ];
    let paths = PathInventory {
        runtime_choose_calls: choice.runtime_choose_calls,
        runtime_selected_executions: a0.selected_executions + permuted_a0.selected_executions,
        runtime_consequence_visibility_events,
        runtime_apply_updates: choice.runtime_credit_updates,
        ..source_paths.clone()
    };
    SeedAudit {
        seed,
        e0,
        choice,
        a0,
        permuted_a0,
        choice_arity,
        permutation_only_changes_actual_route,
        stage_ready,
        paths,
    }
}

fn definitive_rejection() -> CompositionReport {
    CompositionReport {
        label: "CUMULATIVE DS1 DEVELOPMENT: definitive forbidden".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "DEFINITIVE-FORBIDDEN".to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        m1_exists: false,
        stages: std::array::from_fn(|_| "BLOCKED: definitive rejected".to_string()),
        first_collapse_stage: None,
        first_collapse: "NOT RUN: definitive rejected before harness".to_string(),
        paths: derive_source_paths(include_str!("ds1_after_e0_a0_composition_retry.rs")),
        leaks: leak_audit(),
        seeds: Vec::new(),
        audit_passed: false,
    }
}

pub fn run(mode: HarnessMode) -> CompositionReport {
    if mode == HarnessMode::Definitive {
        return definitive_rejection();
    }
    let source_ready = source_boundary_ready();
    let source_paths = derive_source_paths(include_str!("ds1_after_e0_a0_composition_retry.rs"));
    let leaks = leak_audit();
    let (acquisition, seed_values): (usize, &[u64]) = match mode {
        HarnessMode::Micro => (16, &[100]),
        HarnessMode::Gate => (32, &[100, 101, 102, 103, 104]),
        HarnessMode::Definitive => unreachable!("rejected before harness"),
    };
    let e0_control = frozen_e0::run(mode);
    let a0_control = frozen_a0::run(mode);
    let seeds = seed_values
        .iter()
        .map(|seed| audit_seed(*seed, acquisition, source_ready, &source_paths))
        .collect::<Vec<_>>();
    let mut aggregate_ready = [false; 12];
    for (stage, ready) in aggregate_ready.iter_mut().enumerate() {
        *ready = seeds.iter().all(|seed| seed.stage_ready[stage]);
    }
    aggregate_ready[0] &= source_ready && leaks.passed && e0_control.passed && a0_control.passed;
    let (stages, first_collapse_stage) = ordered_freeze(aggregate_ready);
    let first_collapse = first_collapse_stage
        .map(|stage| STAGE_NAMES[stage].to_string())
        .unwrap_or_else(|| "NONE: M1_ELIGIBLE / cumulative definitive pending".to_string());
    let label = first_collapse_stage.map_or_else(
        || "M1_ELIGIBLE / cumulative definitive pending".to_string(),
        |stage| {
            format!(
                "CUMULATIVE DS1 DEVELOPMENT COLLAPSE AT {}",
                STAGE_NAMES[stage]
            )
        },
    );
    let audit_passed = first_collapse_stage.is_some()
        && source_paths.frozen_choose_definitions == 1
        && source_paths.frozen_choose_call_edges == 1
        && source_paths.choice_to_execution_edges == 1
        && source_paths.post_execution_observation_edges == 0
        && source_paths.apply_definitions == 1
        && source_paths.apply_call_edges == 0
        && source_paths.parent_consequence_paths == 0
        && seeds.iter().all(|seed| {
            seed.paths.runtime_consequence_visibility_events == 0
                && seed.paths.runtime_apply_updates == 0
        });
    CompositionReport {
        label,
        protocol: PROTOCOL.to_string(),
        mode: match mode {
            HarnessMode::Micro => "MICRO",
            HarnessMode::Gate => "GATE",
            HarnessMode::Definitive => unreachable!("rejected"),
        }
        .to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        m1_exists: false,
        stages,
        first_collapse_stage,
        first_collapse,
        paths: source_paths,
        leaks,
        seeds,
        audit_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_uses_exact_e0_episode_and_freezes_at_first_real_shortage() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.first_collapse_stage, Some(4), "{report:#?}");
        assert!(report.stages[..4].iter().all(|stage| stage == "READY"));
        assert!(report.stages[4].starts_with("COLLAPSE"));
        assert!(report.stages[5..].iter().all(|stage| stage == "BLOCKED"));
        assert!(report.seeds.iter().all(|seed| {
            seed.e0.export_exact_copy
                && seed.a0.exact_e0_episode_copy
                && seed.a0.input_pulses == seed.a0.mapped_pulses
                && seed.a0.input_propagations == seed.a0.mapped_propagations
                && seed.a0.prebridge_roots > 0
                && seed.a0.handles == seed.a0.prebridge_roots
                && seed.a0.handles < seed.choice_arity
        }));
    }

    #[test]
    fn ordered_freeze_reports_each_mutated_earlier_failure_first() {
        for collapse in 0..12 {
            let mut ready = [true; 12];
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
    fn no_path_zeros_are_independent_and_mutation_sensitive() {
        let composition = include_str!("ds1_after_e0_a0_composition_retry.rs");
        let e0 = include_str!("ds_e0_anonymous_event_formation.rs");
        let a0 = include_str!("ds_a0_anonymous_boundary_action_formation.rs");
        let actual = derive_source_paths_from(composition, e0, a0);
        assert_eq!(actual.post_execution_observation_edges, 0);
        assert_eq!(actual.parent_consequence_paths, 0);
        assert_eq!(actual.apply_call_edges, 0);

        let mutated_composition = composition.replacen(
            "self.prechoice_fingerprints.contains(&fingerprint)",
            "self.observe_post_action(); self.prechoice_fingerprints.contains(&fingerprint)",
            1,
        );
        let visible = derive_source_paths_from(&mutated_composition, e0, a0);
        assert!(visible.post_execution_observation_edges > 0);

        let mutated_a0 = a0.replacen(
            "Some(PhysicalExecution {",
            "let consequence = true; let _ = consequence; Some(PhysicalExecution {",
            1,
        );
        let consequential = derive_source_paths_from(composition, e0, &mutated_a0);
        assert!(consequential.parent_consequence_paths > 0);

        let updated = composition.replacen(
            "#[cfg(test)]",
            "fn mutation_probe() { learner.apply_consequence(view, 0, true); }\n#[cfg(test)]",
            1,
        );
        let update_inventory = derive_source_paths_from(&updated, e0, a0);
        assert!(update_inventory.apply_call_edges > 0);
    }

    #[test]
    fn blocked_choice_execution_consequence_and_update_have_distinct_runtime_counts() {
        let report = run(HarnessMode::Micro);
        assert!(report.seeds.iter().all(|seed| {
            seed.paths.runtime_choose_calls == 0
                && seed.paths.runtime_selected_executions == 0
                && seed.paths.runtime_consequence_visibility_events == 0
                && seed.paths.runtime_apply_updates == 0
        }));
    }

    #[test]
    fn permutation_rule_requires_different_existing_route_fingerprints() {
        let report = run(HarnessMode::Micro);
        assert!(report
            .seeds
            .iter()
            .all(|seed| seed.permutation_only_changes_actual_route.is_none()));
        let validate = |ordinary: u64,
                        permuted: u64,
                        ordinary_inventory: &[u64],
                        permuted_inventory: &[u64]| {
            ordinary != permuted
                && ordinary_inventory.contains(&ordinary)
                && permuted_inventory.contains(&permuted)
        };
        assert!(validate(11, 22, &[11, 22], &[11, 22]));
        assert!(!validate(11, 11, &[11, 22], &[11, 22]));
    }

    #[test]
    fn frozen_persistent_state_has_no_identity_destination_or_evaluator_fields() {
        let audit = leak_audit();
        assert!(audit.passed, "{audit:#?}");
    }

    #[test]
    fn definitive_mode_runs_no_seed_and_is_not_claim_eligible() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.claim_eligible);
        assert!(report.seeds.is_empty());
        assert!(!report.audit_passed);
    }
}
