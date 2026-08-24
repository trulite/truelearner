//! Development-only byte-identical DS1 composition retry after DS-E0 + DS-A1.
//!
//! Frozen E0 produces one event and the unchanged DS1 Neighborhood. Frozen A1
//! independently installs and bridges multiple executable continuations from
//! that same event. This harness freezes the first unavailable downstream path.

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds1-after-e0-a0-a1-composition-retry-v1";
pub const EXACT_PARENT: &str = "3f12055bf6434044095c3e5ca00e23b35806b630";
pub const PROTOCOL_COMMIT: &str = "711a19955c401007ddee446bf8ff3670c896a83c";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_DS_E0_SHA256: &str =
    "fc5d426cc8a5116dbd2749b914e6c30db88529d3070a844a20fc76ac88782615";
pub const FROZEN_DS_A0_SHA256: &str =
    "3eb802f394a225a4ad7f0938b4a672723da2c1303ff95e805423de8161057527";
pub const FROZEN_DS_A1_SHA256: &str =
    "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_M0_SHA256: &str =
    "50cf169bb293177a35270adde656f28f98e68c83a4d39d2876399261b7ee697c";
pub const FROZEN_COMPILED_M0_SHA256: &str =
    "430cd2206c8baa7106c4de7f203d4d0c48b544290e6266596ebcdb91d02655c9";
pub const FROZEN_A1_READINESS_SHA256: &str =
    "5798387bd30558ed86fa092453dd5aafee29983486be3d75af56d2cd18e54676";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

const SUPPORT_PRESENTATIONS: usize = 12;
const STAGE_NAMES: [&str; 10] = [
    "0. M0 lineage and frozen correspondence controls",
    "1. frozen E0 forms the actual learned local event",
    "2. exact one-to-one E0-to-A1 transfer without serializer inference",
    "3. at least two independently formed executable A1 roots before bridge",
    "4. at least two opaque one-to-one alternatives visible to frozen DS1",
    "5. byte-identical frozen DS1 chooses one available alternative",
    "6. selected root physically executes through live CELL/ARROW/SPIKE propagation",
    "7. frozen DS1 receives naturally existing post-choice evidence",
    "8. frozen DS1 updates through its unchanged existing update path",
    "9. boundary-role reconstruction and frozen functional controls",
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
pub struct E0Evidence {
    pub support_exports: usize,
    pub actual_target_events: u64,
    pub exact_export_copy: bool,
    pub exact_neighborhood_copy: bool,
    pub same_target_for_ds1_and_a1: bool,
    pub fresh_occurrences: bool,
    pub learned_shapes: usize,
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

macro_rules! e0_retry_access {
    () => {
        pub(super) struct RetryBundle {
            pub(super) support: Vec<super::E0EpisodeExport>,
            pub(super) target: super::E0EpisodeExport,
            neighborhood: Neighborhood,
            learner: Learner,
            pub(super) evidence: super::E0Evidence,
        }

        fn retry_export(raw: &RawActivity, event: &EventRelations) -> super::E0EpisodeExport {
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

        fn retry_export_matches(
            export: &super::E0EpisodeExport,
            raw: &RawActivity,
            event: &EventRelations,
        ) -> bool {
            export.pulses.len() == raw.spikes.len()
                && export.pulses.iter().zip(&raw.spikes).all(|(copy, source)| {
                    copy.occurrence == source.occurrence.0 && copy.tick == source.local_tick
                })
                && export.observed_propagation.len() == raw.propagation.len()
                && export
                    .observed_propagation
                    .iter()
                    .zip(&raw.propagation)
                    .all(|(copy, source)| *copy == [source.from.0, source.to.0])
                && export.members == event.members.map(|member| member.0)
                && export.relative_temporal == event.temporal
                && export.relative_propagation == event.propagation
        }

        impl RetryBundle {
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

        pub(super) fn choice_arity() -> usize {
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

        pub(super) fn retry_bundle(seed: u64, acquisition: usize) -> Option<RetryBundle> {
            let (mut formation, mut prior_occurrences) = acquire(seed, acquisition);
            let mut support = Vec::new();
            let mut exact_export_copy = true;
            for ordinal in 0..super::SUPPORT_PRESENTATIONS {
                let episode = fixture(
                    seed + 1_000,
                    acquisition + ordinal,
                    ordinal % 4,
                    Perturbation::None,
                );
                prior_occurrences.extend(episode.raw.spikes.iter().map(|spike| spike.occurrence));
                let event = formation.form(&episode.raw)?;
                let export = retry_export(&episode.raw, &event);
                exact_export_copy &= retry_export_matches(&export, &episode.raw, &event);
                support.push(export);
            }
            let episode = fixture(
                seed + 2_000,
                acquisition + super::SUPPORT_PRESENTATIONS + 17,
                0,
                Perturbation::None,
            );
            let target_occurrences = episode
                .raw
                .spikes
                .iter()
                .map(|spike| spike.occurrence)
                .collect::<BTreeSet<_>>();
            let event = formation.form(&episode.raw)?;
            let target = retry_export(&episode.raw, &event);
            exact_export_copy &= retry_export_matches(&target, &episode.raw, &event);
            let neighborhood = serialize_once(&event, &mut formation.work);
            let exact_neighborhood_copy = [
                neighborhood.pair[0].identity.0 as u32,
                neighborhood.pair[1].identity.0 as u32,
                neighborhood.witness.identity.0 as u32,
            ] == target.members
                && [
                    neighborhood.pair[0].position,
                    neighborhood.pair[1].position,
                    neighborhood.witness.position,
                ] == event.propagation_rank
                && [
                    neighborhood.pair[0].tick,
                    neighborhood.pair[1].tick,
                    neighborhood.witness.tick,
                ] == event.temporal_rank
                && [
                    neighborhood.pair[0].window,
                    neighborhood.pair[1].window,
                    neighborhood.witness.window,
                ] == event.attachment_equivalence;
            let actual_target_events = u64::from(members_set(&event.members) == episode.selected);
            Some(RetryBundle {
                evidence: super::E0Evidence {
                    support_exports: support.len(),
                    actual_target_events,
                    exact_export_copy,
                    exact_neighborhood_copy,
                    same_target_for_ds1_and_a1: target.members
                        == event.members.map(|member| member.0),
                    fresh_occurrences: prior_occurrences.is_disjoint(&target_occurrences),
                    learned_shapes: formation.shapes.len(),
                    physical_work: formation.work.organism_work(),
                    persistent_bytes: formation.persistent_bytes(),
                    temporary_bytes: size_of::<EventRelations>()
                        + size_of::<Neighborhood>()
                        + size_of::<RawActivity>(),
                },
                support,
                target,
                neighborhood,
                learner: Learner::default(),
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
    e0_retry_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct A1Evidence {
    pub exact_input_copy: bool,
    pub candidates: usize,
    pub templates: usize,
    pub installed_roots: usize,
    pub structural_roots: usize,
    pub handles: usize,
    pub unique_roots: usize,
    pub nonempty_effects: usize,
    pub unique_effects: usize,
    pub installed_cells: usize,
    pub installed_arrows: usize,
    pub installed_before_bridge: bool,
    pub bridge_one_to_one: bool,
    pub selected_executions: u64,
    pub selected_spike_propagations: u64,
    pub selected_arrow_traversals: u64,
    pub selected_state_mutations: u64,
    pub selected_effect: Option<u64>,
    pub organism_work: u64,
    pub evaluator_comparisons: u64,
    pub persistent_bytes: usize,
    pub temporary_peak_bytes: usize,
    pub cleanup_zero: bool,
}

fn effect_fingerprint(trace: &[u8], activation: &[u16; 3]) -> u64 {
    trace
        .iter()
        .map(|value| u64::from(*value))
        .chain(activation.iter().map(|value| u64::from(*value)))
        .fold(0xcbf2_9ce4_8422_2325u64, |mut hash, value| {
            hash ^= value;
            hash.wrapping_mul(0x100_0000_01b3)
        })
}

macro_rules! a1_retry_access {
    () => {
        pub(super) struct RetryActions {
            substrate: Substrate,
            bridge: OpaqueBridge,
            work: WorkLedger,
            prechoice_effects: BTreeSet<u64>,
            pub(super) evidence: super::A1Evidence,
        }

        fn import_export(source: &super::E0EpisodeExport) -> E0EpisodeExport {
            E0EpisodeExport {
                pulses: source
                    .pulses
                    .iter()
                    .map(|pulse| ExportPulse {
                        occurrence: pulse.occurrence,
                        tick: pulse.tick,
                    })
                    .collect(),
                observed_propagation: source.observed_propagation.clone(),
                members: source.members,
                relative_temporal: source.relative_temporal,
                relative_propagation: source.relative_propagation,
            }
        }

        fn import_exact(source: &super::E0EpisodeExport, target: &E0EpisodeExport) -> bool {
            source.pulses.len() == target.pulses.len()
                && source
                    .pulses
                    .iter()
                    .zip(&target.pulses)
                    .all(|(a, b)| a.occurrence == b.occurrence && a.tick == b.tick)
                && source.observed_propagation == target.observed_propagation
                && source.members == target.members
                && source.relative_temporal == target.relative_temporal
                && source.relative_propagation == target.relative_propagation
        }

        impl RetryActions {
            pub(super) fn execute_choice(&mut self, index: usize) -> bool {
                let Some(entry) = self.bridge.entries.get(index) else {
                    return false;
                };
                let before_spikes = self.work.spike_propagations;
                let before_arrows = self.work.arrow_traversals;
                let before_mutations = self.work.state_mutations;
                let Some(effect) =
                    execute_handle(&self.substrate, &self.bridge, entry.handle, &mut self.work)
                else {
                    return false;
                };
                let fingerprint = super::effect_fingerprint(&effect.trace, &effect.activation);
                self.evidence.selected_executions += 1;
                self.evidence.selected_spike_propagations +=
                    self.work.spike_propagations - before_spikes;
                self.evidence.selected_arrow_traversals +=
                    self.work.arrow_traversals - before_arrows;
                self.evidence.selected_state_mutations +=
                    self.work.state_mutations - before_mutations;
                self.evidence.selected_effect = Some(fingerprint);
                self.evidence.organism_work = self.work.organism_work();
                self.prechoice_effects.contains(&fingerprint)
            }

            pub(super) fn prechoice_effects(&self) -> &BTreeSet<u64> {
                &self.prechoice_effects
            }

            pub(super) fn cleanup(&mut self) -> bool {
                let bindings = self
                    .substrate
                    .cells
                    .iter()
                    .filter(|cell| cell.binding.is_some())
                    .count();
                self.work.cleanup_items += (self.substrate.cells.len()
                    + self.substrate.arrows.len()
                    + self.substrate.observations.len()
                    + self.bridge.entries.len()
                    + bindings) as u64;
                self.bridge.entries.clear();
                self.substrate.cells.clear();
                self.substrate.arrows.clear();
                self.substrate.observations.clear();
                self.substrate.padding.clear();
                self.evidence.cleanup_zero = self.bridge.entries.is_empty()
                    && self.substrate.cells.is_empty()
                    && self.substrate.arrows.is_empty()
                    && self.substrate.observations.is_empty();
                self.evidence.organism_work = self.work.organism_work();
                self.evidence.cleanup_zero
            }
        }

        pub(super) fn retry_actions(
            support_source: &[super::E0EpisodeExport],
            target_source: &super::E0EpisodeExport,
            permuted: bool,
        ) -> Option<RetryActions> {
            let support = support_source.iter().map(import_export).collect::<Vec<_>>();
            let target = import_export(target_source);
            let exact_input_copy = support_source
                .iter()
                .zip(&support)
                .all(|(source, imported)| import_exact(source, imported))
                && import_exact(target_source, &target);
            let mut learner = train(&support, false)?;
            let templates = learner.consolidated();
            let mut substrate = substrate_from_export(&target, MappingOptions::default())?;
            let basal_cells = substrate.cells.len();
            let basal_arrows = substrate.arrows.len();
            let (candidates, installed) = learner.install(&mut substrate, true, false);
            let installed_roots = installed.len();
            let installed_cells = substrate.cells.len() - basal_cells;
            let installed_arrows = substrate.arrows.len() - basal_arrows;
            let structural = structural_dedup(&mut substrate, &installed, &mut learner.work);
            let structural_roots = structural.len();
            let bridge = expose_roots(&structural, permuted, &mut learner.work);
            let unique_roots = bridge
                .entries
                .iter()
                .map(|entry| entry.root)
                .collect::<BTreeSet<_>>()
                .len();
            let (nonempty_effects, effects) =
                bridge_effects(&substrate, &bridge, &mut learner.work);
            let prechoice_effects = effects
                .iter()
                .map(|effect| super::effect_fingerprint(&effect.trace, &effect.activation))
                .collect::<BTreeSet<_>>();
            let handles = bridge.entries.len();
            let temporary_peak_bytes = size_of::<Substrate>()
                + substrate.cells.len() * size_of::<Cell>()
                + substrate.arrows.len() * size_of::<Arrow>()
                + handles * size_of::<BridgeEntry>();
            Some(RetryActions {
                evidence: super::A1Evidence {
                    exact_input_copy,
                    candidates,
                    templates,
                    installed_roots,
                    structural_roots,
                    handles,
                    unique_roots,
                    nonempty_effects,
                    unique_effects: prechoice_effects.len(),
                    installed_cells,
                    installed_arrows,
                    installed_before_bridge: installed_roots > 1
                        && installed_cells == installed_roots * 2
                        && installed_arrows == installed_roots,
                    bridge_one_to_one: handles == structural_roots && unique_roots == handles,
                    selected_executions: 0,
                    selected_spike_propagations: 0,
                    selected_arrow_traversals: 0,
                    selected_state_mutations: 0,
                    selected_effect: None,
                    organism_work: learner.work.organism_work(),
                    evaluator_comparisons: learner.work.evaluator_comparisons,
                    persistent_bytes: learner.work.persistent_bytes,
                    temporary_peak_bytes,
                    cleanup_zero: false,
                },
                substrate,
                bridge,
                work: learner.work,
                prechoice_effects,
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_a1 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_a1_affordance_multiplicity.rs"
    ));
    a1_retry_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PathInventory {
    pub frozen_choose_definitions: usize,
    pub frozen_choose_call_edges: usize,
    pub choice_to_execution_edges: usize,
    pub post_choice_observer_edges: usize,
    pub apply_definitions: usize,
    pub apply_call_edges: usize,
    pub natural_post_choice_evidence_paths: usize,
    pub effect_to_choice_edges: usize,
    pub runtime_choose_calls: u64,
    pub runtime_selected_executions: u64,
    pub runtime_post_choice_evidence_events: u64,
    pub runtime_apply_updates: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub e0_hash: bool,
    pub a0_hash: bool,
    pub a1_hash: bool,
    pub m0_hash: bool,
    pub compiled_m0_hash: bool,
    pub readiness_hash: bool,
    pub marked_ds1_hash: bool,
    pub learner_markers: bool,
    pub frozen_mechanisms_read_only: bool,
    pub paths: PathInventory,
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
                    return Some(&tail[..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn derive_paths(composition: &str, e0: &str, a1: &str) -> PathInventory {
    let choose = function_body(composition, "choose_opaque").unwrap_or_default();
    let execution = function_body(composition, "execute_choice").unwrap_or_default();
    let a1_execution = function_body(a1, "execute_root").unwrap_or_default();
    let production = composition
        .split("#[cfg(test)]")
        .next()
        .unwrap_or(composition);
    let observer = ["observe_", "post_choice("].concat();
    let apply = [".apply_", "consequence("].concat();
    let natural_fragments = ["consequence", "credit", "reward", "accepted", "rejected"];
    PathInventory {
        frozen_choose_definitions: e0.matches("fn choose(").count(),
        frozen_choose_call_edges: choose.matches(".choose(").count(),
        choice_to_execution_edges: execution.matches("execute_handle(").count(),
        post_choice_observer_edges: execution.matches(&observer).count(),
        apply_definitions: e0.matches("fn apply_consequence(").count(),
        apply_call_edges: production.matches(&apply).count(),
        natural_post_choice_evidence_paths: natural_fragments
            .iter()
            .map(|fragment| a1_execution.matches(fragment).count())
            .sum(),
        effect_to_choice_edges: choose.matches("NormalizedEffect").count()
            + choose.matches("execute_").count(),
        ..PathInventory::default()
    }
}

fn source_audit() -> SourceAudit {
    let e0 = include_str!("ds_e0_anonymous_event_formation.rs");
    let a1 = include_str!("ds_a1_affordance_multiplicity.rs");
    let composition = include_str!("ds1_after_e0_a0_a1_composition_retry.rs");
    SourceAudit {
        e0_hash: env!("DS1_A1_E0_SHA256") == FROZEN_DS_E0_SHA256,
        a0_hash: env!("DS1_A1_A0_SHA256") == FROZEN_DS_A0_SHA256,
        a1_hash: env!("DS1_A1_A1_SHA256") == FROZEN_DS_A1_SHA256,
        m0_hash: env!("DS1_A1_M0_SHA256") == FROZEN_M0_SHA256,
        compiled_m0_hash: env!("DS1_A1_COMPILED_M0_SHA256") == FROZEN_COMPILED_M0_SHA256,
        readiness_hash: env!("DS1_A1_READINESS_SHA256") == FROZEN_A1_READINESS_SHA256,
        marked_ds1_hash: frozen_e0::FROZEN_DS1_LEARNER_SHA256 == FROZEN_DS1_SHA256,
        learner_markers: e0.matches("// DS1_LEARNER_BEGIN").count() == 2
            && e0.matches("// DS1_LEARNER_END").count() == 2,
        frozen_mechanisms_read_only: composition
            .lines()
            .filter(|line| line.starts_with("mod frozen_"))
            .count()
            == 2,
        paths: derive_paths(composition, e0, a1),
    }
}

impl SourceAudit {
    fn stage_zero_ready(&self) -> bool {
        self.e0_hash
            && self.a0_hash
            && self.a1_hash
            && self.m0_hash
            && self.compiled_m0_hash
            && self.readiness_hash
            && self.marked_ds1_hash
            && self.learner_markers
            && self.frozen_mechanisms_read_only
            && self.paths.frozen_choose_definitions == 1
            && self.paths.frozen_choose_call_edges == 1
            && self.paths.choice_to_execution_edges == 1
            && self.paths.apply_definitions == 1
            && self.paths.effect_to_choice_edges == 0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ControlAudit {
    pub exact_frozen_signature: bool,
    pub same_event_both_interfaces: bool,
    pub one_to_one_bridge: bool,
    pub permuted_inventory_equal: bool,
    pub permutation_changes_selected_route: bool,
    pub no_route_after_bridge: bool,
    pub zero_paths_mutation_sensitive: bool,
    pub no_semantic_wiring: bool,
    pub cleanup_zero: bool,
}

impl ControlAudit {
    pub fn passed_through_stage_six(&self) -> bool {
        self.exact_frozen_signature
            && self.same_event_both_interfaces
            && self.one_to_one_bridge
            && self.permuted_inventory_equal
            && self.permutation_changes_selected_route
            && self.no_route_after_bridge
            && self.zero_paths_mutation_sensitive
            && self.no_semantic_wiring
            && self.cleanup_zero
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub e0: E0Evidence,
    pub a1: A1Evidence,
    pub permuted_a1: A1Evidence,
    pub choice: ChoiceEvidence,
    pub choice_arity: usize,
    pub controls: ControlAudit,
    pub paths: PathInventory,
    pub stage_ready: [bool; 10],
}

fn audit_seed(seed: u64, acquisition: usize, source: &SourceAudit) -> SeedAudit {
    let mut bundle = frozen_e0::retry_bundle(seed, acquisition)
        .expect("frozen E0 forms the actual target event");
    let e0 = bundle.evidence.clone();
    let mut ordinary = frozen_a1::retry_actions(&bundle.support, &bundle.target, false)
        .expect("frozen A1 consumes the exact E0 export");
    let mut permuted = frozen_a1::retry_actions(&bundle.support, &bundle.target, true)
        .expect("frozen A1 consumes the same event with permuted handles");
    let choice_arity = frozen_e0::choice_arity();

    let stage_one = e0.actual_target_events == 1 && e0.learned_shapes > 0;
    let stage_two = stage_one
        && e0.exact_export_copy
        && e0.exact_neighborhood_copy
        && e0.same_target_for_ds1_and_a1
        && e0.fresh_occurrences
        && ordinary.evidence.exact_input_copy;
    let stage_three = stage_two
        && ordinary.evidence.candidates >= 2
        && ordinary.evidence.templates >= 2
        && ordinary.evidence.installed_roots >= 2
        && ordinary.evidence.structural_roots >= 2
        && ordinary.evidence.installed_before_bridge
        && ordinary.evidence.nonempty_effects == ordinary.evidence.handles
        && ordinary.evidence.unique_effects == ordinary.evidence.handles;
    let stage_four = stage_three
        && ordinary.evidence.handles == choice_arity
        && ordinary.evidence.handles > 1
        && ordinary.evidence.bridge_one_to_one
        && ordinary.prechoice_effects() == permuted.prechoice_effects();

    let choice = if stage_four {
        bundle.choose_opaque(seed as usize)
    } else {
        ChoiceEvidence::default()
    };
    let stage_five = stage_four
        && choice.runtime_choose_calls == 1
        && choice.runtime_credit_updates == 0
        && choice
            .choice
            .is_some_and(|index| index < ordinary.evidence.handles);
    let (ordinary_executed, permuted_executed) = if stage_five {
        let selected = choice.choice.expect("stage five established a choice");
        (
            ordinary.execute_choice(selected),
            permuted.execute_choice(selected),
        )
    } else {
        (false, false)
    };
    let stage_six = stage_five
        && ordinary_executed
        && permuted_executed
        && ordinary.evidence.selected_executions == 1
        && ordinary.evidence.selected_spike_propagations > 0
        && ordinary.evidence.selected_arrow_traversals > 0
        && ordinary.evidence.selected_state_mutations > 0
        && ordinary.evidence.selected_effect.is_some()
        && permuted.evidence.selected_effect.is_some()
        && ordinary.evidence.selected_effect != permuted.evidence.selected_effect;

    let runtime_post_choice_evidence_events = (source
        .paths
        .post_choice_observer_edges
        .min(source.paths.natural_post_choice_evidence_paths)
        as u64)
        .saturating_mul(ordinary.evidence.selected_executions);
    let stage_seven = stage_six
        && source.paths.post_choice_observer_edges > 0
        && source.paths.natural_post_choice_evidence_paths > 0
        && runtime_post_choice_evidence_events > 0;
    let runtime_apply_updates = if stage_seven {
        choice.runtime_credit_updates
    } else {
        0
    };
    let stage_eight = stage_seven && source.paths.apply_call_edges > 0 && runtime_apply_updates > 0;
    let stage_nine = false;

    let composition = include_str!("ds1_after_e0_a0_a1_composition_retry.rs");
    let e0_source = include_str!("ds_e0_anonymous_event_formation.rs");
    let a1_source = include_str!("ds_a1_affordance_multiplicity.rs");
    let mutated_observer = composition.replacen(
        "self.evidence.selected_executions += 1;",
        "self.observe_post_choice(); self.evidence.selected_executions += 1;",
        1,
    );
    let mutated_apply = composition.replacen(
        "#[cfg(test)]",
        "fn mutation_apply() { learner.apply_consequence(view, 0, true); }\n#[cfg(test)]",
        1,
    );
    let mutated_a1 = a1_source.replacen(
        "(!trace.is_empty()).then_some(NormalizedEffect { trace, activation })",
        "let consequence = true; let _ = consequence; (!trace.is_empty()).then_some(NormalizedEffect { trace, activation })",
        1,
    );
    let zero_paths_mutation_sensitive = derive_paths(&mutated_observer, e0_source, a1_source)
        .post_choice_observer_edges
        > 0
        && derive_paths(&mutated_apply, e0_source, a1_source).apply_call_edges > 0
        && derive_paths(composition, e0_source, &mutated_a1).natural_post_choice_evidence_paths > 0;
    let no_semantic_wiring = source.paths.effect_to_choice_edges == 0
        && source.paths.apply_call_edges == 0
        && source.paths.post_choice_observer_edges == 0
        && source.paths.natural_post_choice_evidence_paths == 0;
    let exact_frozen_signature = ordinary.evidence.candidates == 2
        && ordinary.evidence.templates == 3
        && ordinary.evidence.installed_roots == 2
        && ordinary.evidence.structural_roots == 2
        && ordinary.evidence.unique_effects == 2
        && ordinary.evidence.handles == 2;
    let permuted_inventory_equal = ordinary.evidence.candidates == permuted.evidence.candidates
        && ordinary.evidence.templates == permuted.evidence.templates
        && ordinary.evidence.installed_roots == permuted.evidence.installed_roots
        && ordinary.evidence.structural_roots == permuted.evidence.structural_roots
        && ordinary.prechoice_effects() == permuted.prechoice_effects();
    let permutation_changes_selected_route =
        stage_six && ordinary.evidence.selected_effect != permuted.evidence.selected_effect;
    let no_route_after_bridge = ordinary.evidence.installed_roots
        == ordinary.evidence.structural_roots
        && ordinary.evidence.structural_roots == ordinary.evidence.handles;
    let ordinary_cleanup = ordinary.cleanup();
    let permuted_cleanup = permuted.cleanup();
    let controls = ControlAudit {
        exact_frozen_signature,
        same_event_both_interfaces: e0.same_target_for_ds1_and_a1
            && ordinary.evidence.exact_input_copy,
        one_to_one_bridge: ordinary.evidence.bridge_one_to_one,
        permuted_inventory_equal,
        permutation_changes_selected_route,
        no_route_after_bridge,
        zero_paths_mutation_sensitive,
        no_semantic_wiring,
        cleanup_zero: ordinary_cleanup && permuted_cleanup,
    };
    let paths = PathInventory {
        runtime_choose_calls: choice.runtime_choose_calls,
        runtime_selected_executions: ordinary.evidence.selected_executions
            + permuted.evidence.selected_executions,
        runtime_post_choice_evidence_events,
        runtime_apply_updates,
        ..source.paths.clone()
    };
    let stage_ready = [
        source.stage_zero_ready(),
        stage_one,
        stage_two,
        stage_three,
        stage_four,
        stage_five,
        stage_six && controls.passed_through_stage_six(),
        stage_seven,
        stage_eight,
        stage_nine,
    ];
    SeedAudit {
        seed,
        e0,
        a1: ordinary.evidence,
        permuted_a1: permuted.evidence,
        choice,
        choice_arity,
        controls,
        paths,
        stage_ready,
    }
}

fn ordered_freeze(ready: [bool; 10]) -> ([String; 10], Option<usize>) {
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
pub struct CompositionReport {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub m0_authoritative: bool,
    pub enabling_only: bool,
    pub m1_exists: bool,
    pub source: SourceAudit,
    pub stages: [String; 10],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub seeds: Vec<SeedAudit>,
    pub audit_passed: bool,
}

fn definitive_rejection() -> CompositionReport {
    CompositionReport {
        label: "UNCHANGED DS1 RETRY: definitive forbidden".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "DEFINITIVE-FORBIDDEN".to_string(),
        claim_eligible: false,
        m0_authoritative: true,
        enabling_only: true,
        m1_exists: false,
        source: source_audit(),
        stages: std::array::from_fn(|_| "BLOCKED: definitive rejected".to_string()),
        first_collapse_stage: None,
        first_collapse: "NOT RUN: definitive rejected before harness".to_string(),
        seeds: Vec::new(),
        audit_passed: false,
    }
}

pub fn run(mode: HarnessMode) -> CompositionReport {
    if mode == HarnessMode::Definitive {
        return definitive_rejection();
    }
    let source = source_audit();
    let (acquisition, seed_values): (usize, &[u64]) = match mode {
        HarnessMode::Micro => (16, &[100]),
        HarnessMode::Gate => (32, &[100, 101, 102, 103, 104]),
        HarnessMode::Definitive => unreachable!("rejected before harness"),
    };
    let seeds = seed_values
        .iter()
        .map(|seed| audit_seed(*seed, acquisition, &source))
        .collect::<Vec<_>>();
    let mut ready = [false; 10];
    for (stage, value) in ready.iter_mut().enumerate() {
        *value = seeds.iter().all(|seed| seed.stage_ready[stage]);
    }
    let (stages, first_collapse_stage) = ordered_freeze(ready);
    let first_collapse = first_collapse_stage
        .map(|stage| STAGE_NAMES[stage].to_string())
        .unwrap_or_else(|| "NONE: M1 eligibility requires separate authorization".to_string());
    let audit_passed = first_collapse_stage == Some(7)
        && seeds.iter().all(|seed| {
            seed.paths.runtime_choose_calls == 1
                && seed.paths.runtime_selected_executions == 2
                && seed.paths.runtime_post_choice_evidence_events == 0
                && seed.paths.runtime_apply_updates == 0
                && seed.controls.passed_through_stage_six()
        });
    CompositionReport {
        label: first_collapse_stage.map_or_else(
            || "UNCHANGED DS1 RETRY: M1-ELIGIBLE DEVELOPMENT".to_string(),
            |stage| format!("UNCHANGED DS1 RETRY COLLAPSE AT {}", STAGE_NAMES[stage]),
        ),
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
        source,
        stages,
        first_collapse_stage,
        first_collapse,
        seeds,
        audit_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_reaches_physical_execution_then_freezes_exactly() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.first_collapse_stage, Some(7));
        assert!(report.stages[..7].iter().all(|stage| stage == "READY"));
        assert!(report.stages[7].starts_with("COLLAPSE"));
        assert!(report.stages[8..].iter().all(|stage| stage == "BLOCKED"));
    }

    #[test]
    fn actual_event_produces_two_choices_and_selected_route_executes() {
        let report = run(HarnessMode::Micro);
        assert!(report.seeds.iter().all(|seed| {
            seed.e0.same_target_for_ds1_and_a1
                && seed.a1.candidates == 2
                && seed.a1.installed_roots == 2
                && seed.a1.structural_roots == 2
                && seed.a1.handles == 2
                && seed.choice_arity == 2
                && seed.choice.runtime_choose_calls == 1
                && seed.a1.selected_executions == 1
                && seed.a1.selected_arrow_traversals > 0
                && seed.controls.permutation_changes_selected_route
        }));
    }

    #[test]
    fn post_choice_evidence_and_update_absence_are_independent_and_mechanical() {
        let report = run(HarnessMode::Micro);
        assert!(report.seeds.iter().all(|seed| {
            seed.paths.post_choice_observer_edges == 0
                && seed.paths.natural_post_choice_evidence_paths == 0
                && seed.paths.apply_call_edges == 0
                && seed.paths.runtime_post_choice_evidence_events == 0
                && seed.paths.runtime_apply_updates == 0
                && seed.controls.zero_paths_mutation_sensitive
        }));
    }

    #[test]
    fn frozen_hashes_and_source_boundary_hold() {
        let audit = source_audit();
        assert!(audit.stage_zero_ready(), "{audit:#?}");
    }

    #[test]
    fn ordered_freeze_blocks_every_later_stage() {
        for collapse in 0..10 {
            let mut ready = [true; 10];
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
    fn definitive_mode_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.audit_passed);
        assert!(!report.claim_eligible);
        assert!(report.seeds.is_empty());
        assert!(!report.m1_exists);
    }
}
