//! Development-only cumulative DS3 event-boundary port over authoritative M2.
//!
//! The isolated DS3 learner is included byte-for-byte. This module only wires
//! observations from the frozen A1/AC0/IR0 lifecycle machinery and supplies
//! evaluator-side fixtures, controls, and reporting. Definitive mode remains
//! locked pending its separately preregistered matrix.

use crate::research_runtime::HarnessMode;
use std::collections::BTreeSet;

pub const PROTOCOL: &str = "ds3-cumulative-event-boundary-v1";
pub const EXACT_PARENT: &str = "162a5b2082a8c1ac9ede45bc5178fecf3509b476";
pub const PROTOCOL_COMMIT: &str = "1878c018e520cae8cac9e1af229f03f87831a9b5";
pub const EXPECTATION_COMMIT: &str = "8ca36f5b44f57f675057307783cae3bc984b641a";
pub const MECHANISM_INSTALL_COMMIT: &str = "6d3fea34e13b1417356f76cbf04e9d9916ec61fb";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_MECHANISM_SHA256: &str =
    "a8d8fe060b497c7a6b5f9a5a88b7ed2292dc8a729a8781f599547b6027efc0a0";
pub const FROZEN_A1_SHA256: &str =
    "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1";
pub const FROZEN_AC0_SHA256: &str =
    "860e89304e86f254dd02a5aa35cf63cc240af160039b4166fa0cb5856dacb84a";
pub const FROZEN_IR0_SHA256: &str =
    "f81cc694f2d6d9e43cb04e8d1a1db301687e6644899665ae470abed1f9e4a7dc";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "72e13ef4029d727712d40dd54a6ac8f28b6e94950dd038c865f576b35b9bcf34";
pub const FROZEN_EXPECTATION_SHA256: &str =
    "772685104558b9e5eb91c28900381b712fed80e2fdbe607093e7046dd05f0ef5";

const M2_ACQUISITION: usize = 16;
const ROUTES: usize = 2;
const STAGES: [&str; 6] = [
    "P0 parent and frozen mechanism hash audit",
    "P1 wiring produces legal role/link streams from learned machinery alone",
    "P2 reconstruction on held-out streams",
    "P3 functional adequacy (consequence parity)",
    "P4 controls 1-12",
    "P5 duplicate determinism and work attribution",
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct GlueEffect {
    trace: Vec<u8>,
    activation: [u16; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GlueRole {
    Open,
    Continue,
    Close,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GlueLifecycleRow {
    role: GlueRole,
    temporal_delta: i8,
    directional_incidence: i8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GlueLifecycle {
    rows: Vec<GlueLifecycleRow>,
    effect: GlueEffect,
    support_threshold: u16,
    installed_roots: usize,
    exposed_handles: usize,
    work: u64,
}

macro_rules! glue_a1_access {
    () => {
        pub(super) fn glue_lifecycle(
            seed: u64,
            acquisition: usize,
            route: usize,
            presentations: usize,
            relabel: bool,
            reverse_allocation: bool,
        ) -> Option<super::GlueLifecycle> {
            let bundle = frozen_e0::a1_bundle(seed, acquisition)?;
            let export = if relabel {
                relabel_export(&bundle.target)
            } else {
                bundle.target.clone()
            };
            let mapping = MappingOptions {
                reverse_allocation,
                ..MappingOptions::default()
            };

            // First obtain the route view from the already-consolidated A1
            // support lineage. This selects an existing physical route; it
            // does not manufacture a boundary signal.
            let mut view_learner = train(&bundle.support, false)?;
            let mut view_substrate = substrate_from_export(&export, mapping)?;
            let (_, installed) = view_learner.install(&mut view_substrate, true, false);
            let roots = structural_dedup(&mut view_substrate, &installed, &mut view_learner.work);
            if roots.len() != super::ROUTES || route >= roots.len() {
                return None;
            }
            let view_effect = execute_root(&view_substrate, roots[route], &mut view_learner.work)?;
            let observations = view_effect
                .trace
                .windows(2)
                .map(|pair| {
                    [
                        view_substrate.members[usize::from(pair[0])],
                        view_substrate.members[usize::from(pair[1])],
                    ]
                })
                .collect::<Vec<_>>();
            if observations.is_empty() {
                return None;
            }

            // Run a fresh learner through ordinary A1 probation. Open and
            // Continue come from the support count transition. Close exists
            // only when the threshold-crossing proposal installs, survives
            // structural dedup, is exposed, and physically executes.
            let mut learner = Learner::default();
            let mut rows = Vec::new();
            let mut installed_roots = 0usize;
            let mut exposed_handles = 0usize;
            let steps = presentations.min(usize::from(SUPPORT_EPISODES));
            for _ in 0..steps {
                let mut substrate = substrate_from_export(&export, mapping)?;
                substrate.observations.clone_from(&observations);
                let proposals = local_proposals(&substrate, &mut learner.work);
                if proposals.len() != 1 {
                    return None;
                }
                let template = proposals[0].template;
                let before = learner
                    .templates
                    .get(&template)
                    .map_or(0, |support| support.count);
                let observed = learner.observe(&substrate, true);
                let after = learner.templates.get(&template)?.count;
                if observed != 1 || after != before + 1 {
                    return None;
                }

                let mut role = if before == 0 {
                    super::GlueRole::Open
                } else {
                    super::GlueRole::Continue
                };
                if before < SUPPORT_EPISODES && after >= SUPPORT_EPISODES {
                    let (_, installed) = learner.install(&mut substrate, true, false);
                    let roots = structural_dedup(&mut substrate, &installed, &mut learner.work);
                    let bridge = expose_roots(&roots, false, &mut learner.work);
                    if roots.len() != 1 || bridge.entries.len() != 1 {
                        return None;
                    }
                    let completed = execute_handle(
                        &substrate,
                        &bridge,
                        bridge.entries[0].handle,
                        &mut learner.work,
                    )?;
                    if completed.trace != view_effect.trace
                        || completed.activation != view_effect.activation
                    {
                        return None;
                    }
                    installed_roots = roots.len();
                    exposed_handles = bridge.entries.len();
                    role = super::GlueRole::Close;
                }
                rows.push(super::GlueLifecycleRow {
                    role,
                    temporal_delta: template.temporal_delta,
                    directional_incidence: template.directional_incidence,
                });
            }
            view_learner.work.absorb(&learner.work);
            Some(super::GlueLifecycle {
                rows,
                effect: super::GlueEffect {
                    trace: view_effect.trace,
                    activation: view_effect.activation,
                },
                support_threshold: SUPPORT_EPISODES,
                installed_roots,
                exposed_handles,
                work: view_learner.work.organism_work(),
            })
        }

        pub(super) fn glue_source_ok() -> bool {
            source_audit().passed()
        }
    };
}

#[allow(dead_code)]
mod frozen_a1 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_a1_affordance_multiplicity.rs"
    ));
    glue_a1_access!();
}

macro_rules! glue_ac0_access {
    () => {
        pub(super) fn glue_abstention(
            seed: u64,
            acquisition: usize,
            selected: usize,
            stale: bool,
        ) -> Option<bool> {
            let actuation = frozen_a1::actuate_existing(
                seed,
                acquisition,
                selected,
                ActuationOptions {
                    block_selected: !stale,
                    stale_handle: stale,
                    ..ActuationOptions::default()
                },
            )?;
            Some(
                source_audit().passed()
                    && actuation.roots_before_choice == super::ROUTES
                    && actuation.handles_before_choice == super::ROUTES
                    && actuation.bridge_one_to_one
                    && actuation.delta.is_none(),
            )
        }

        pub(super) fn glue_source_ok() -> bool {
            source_audit().passed()
        }
    };
}

#[allow(dead_code)]
mod frozen_ac0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_ac0_selected_affordance_actuation_closure.rs"
    ));
    glue_ac0_access!();
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GlueIr0Lifecycle {
    original: GlueEffect,
    current: GlueEffect,
    compatible_uses: usize,
    mismatches: usize,
    invalidations: usize,
    invalidated_abstentions: usize,
    reopenings: usize,
    reopened_executions: usize,
    historical_returns: usize,
    historical_unchanged: bool,
    source_ok: bool,
}

macro_rules! glue_ir0_access {
    () => {
        pub(super) fn glue_lifecycle(
            seed: u64,
            acquisition: usize,
            original: usize,
            changed: bool,
            layout_transfer: bool,
        ) -> Option<super::GlueIr0Lifecycle> {
            if original >= super::ROUTES {
                return None;
            }
            let current = if changed { 1 - original } else { original };
            let original_signal = frozen_rt0::ir0_signal(
                seed,
                acquisition,
                original,
                layout_transfer,
                layout_transfer,
                layout_transfer,
            )?;
            let current_signal = frozen_rt0::ir0_signal(
                seed,
                acquisition,
                current,
                layout_transfer,
                layout_transfer,
                layout_transfer,
            )?;
            let result = frozen_a1::ir0_lifecycle(
                seed,
                acquisition,
                &original_signal,
                Some(&current_signal),
                &original_signal,
                LifecycleOptions {
                    reverse_allocation: layout_transfer,
                    layout_padding: layout_transfer,
                    permute_handles: layout_transfer,
                },
            )?;
            Some(super::GlueIr0Lifecycle {
                original: super::GlueEffect {
                    trace: original_signal.trace,
                    activation: original_signal.activation,
                },
                current: super::GlueEffect {
                    trace: current_signal.trace,
                    activation: current_signal.activation,
                },
                compatible_uses: result.compatible_retained_uses,
                mismatches: result.structural_mismatches,
                invalidations: result.stale_routes_invalidated,
                invalidated_abstentions: result.invalidated_route_abstentions,
                reopenings: result.generic_reopenings,
                reopened_executions: result.reopened_executions,
                historical_returns: result.historical_return_uses,
                historical_unchanged: result.historical_counts_before
                    == result.historical_counts_after,
                source_ok: source_audit().passed(),
            })
        }

        pub(super) fn glue_source_ok() -> bool {
            source_audit().passed()
        }
    };
}

#[allow(dead_code)]
mod frozen_ir0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_ir0_dependency_invalidation_reopening.rs"
    ));
    glue_ir0_access!();
}

macro_rules! glue_ds3_access {
    () => {
        pub(super) fn glue_default_boundary() -> BoundaryLearner {
            BoundaryLearner::default()
        }

        pub(super) fn glue_evaluate(
            learner: &mut BoundaryLearner,
            observations: &[Observation],
            acquire: bool,
        ) -> Evaluation {
            learner.evaluate(observations, acquire)
        }

        pub(super) fn glue_chunk_count(learner: &BoundaryLearner) -> usize {
            learner.chunks.len()
        }

        pub(super) fn glue_persistent_bytes(learner: &BoundaryLearner) -> usize {
            learner.persistent_bytes()
        }

        pub(super) fn glue_mechanism_source_audit() -> bool {
            source_audit()
        }
    };
}

#[allow(dead_code)]
mod frozen_ds3 {
    include!(concat!(env!("OUT_DIR"), "/ds3_event_boundary.rs"));
    glue_ds3_access!();
}

use frozen_ds3::{BoundaryRole, CausalLink, Evaluation, Observation};

#[derive(Clone, Debug, PartialEq, Eq)]
struct ExpectedSpan {
    start: usize,
    end: usize,
    relation: u8,
    roles: Vec<BoundaryRole>,
    consequence: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Stream {
    observations: Vec<Observation>,
    expected: Vec<ExpectedSpan>,
    m2_work: u64,
    organic_rows: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RenderOptions {
    flat_shape: Option<u8>,
    shape_xor: u8,
    consequence_delta: u8,
    reverse_time: bool,
    relabel: bool,
    reverse_allocation: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            flat_shape: None,
            shape_xor: 0,
            consequence_delta: 0,
            reverse_time: false,
            relabel: false,
            reverse_allocation: false,
        }
    }
}

fn relation_class(effect: &GlueEffect) -> u8 {
    effect.trace.iter().fold(17u8, |value, member| {
        value.wrapping_mul(31).wrapping_add(*member)
    })
}

fn propagation_class(effect: &GlueEffect) -> u8 {
    effect.trace.windows(2).count().min(255) as u8
}

fn consequence(effect: &GlueEffect, delta: u8) -> u8 {
    (effect
        .activation
        .iter()
        .fold(0u32, |total, value| total + u32::from(*value))
        .min(255) as u8)
        .wrapping_add(delta)
}

fn next_occurrence(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}

fn append_lifecycle(
    stream: &mut Stream,
    seed: u64,
    route: usize,
    presentations: usize,
    options: RenderOptions,
    occurrences: &mut u64,
) -> bool {
    let Some(lifecycle) = frozen_a1::glue_lifecycle(
        seed,
        M2_ACQUISITION,
        route,
        presentations,
        options.relabel,
        options.reverse_allocation,
    ) else {
        return false;
    };
    if lifecycle.rows.is_empty()
        || lifecycle.support_threshold != 3
        || (presentations >= usize::from(lifecycle.support_threshold)
            && (lifecycle.installed_roots != 1 || lifecycle.exposed_handles != 1))
    {
        return false;
    }
    let relation = relation_class(&lifecycle.effect);
    let propagation = propagation_class(&lifecycle.effect);
    let span_start = stream.observations.len();
    let mut roles = Vec::new();
    for (offset, row) in lifecycle.rows.iter().enumerate() {
        let role = match row.role {
            GlueRole::Open => BoundaryRole::Open,
            GlueRole::Continue => BoundaryRole::Continue,
            GlueRole::Close => BoundaryRole::Close,
        };
        let link = if role == BoundaryRole::Open {
            CausalLink::Reset
        } else {
            CausalLink::Continue
        };
        let shape = options.flat_shape.unwrap_or_else(|| {
            (row.temporal_delta as u8)
                .wrapping_mul(19)
                .wrapping_add(row.directional_incidence as u8)
                ^ options.shape_xor
        });
        let local_time = if options.reverse_time {
            ((lifecycle.rows.len() - offset) * 29) as u16
        } else {
            (offset * 7 + 3) as u16
        };
        let ordinary_consequence = if role == BoundaryRole::Close {
            consequence(&lifecycle.effect, options.consequence_delta)
        } else {
            0
        };
        stream.observations.push(Observation {
            occurrence: next_occurrence(occurrences),
            shape,
            local_time,
            propagation,
            boundary_role: role,
            causal_link: link,
            functional_relation: relation,
            ordinary_consequence,
        });
        roles.push(role);
    }
    stream.m2_work += lifecycle.work;
    stream.organic_rows += lifecycle.rows.len();
    if roles.last() == Some(&BoundaryRole::Close) {
        stream.expected.push(ExpectedSpan {
            start: span_start,
            end: stream.observations.len() - 1,
            relation,
            roles,
            consequence: consequence(&lifecycle.effect, options.consequence_delta),
        });
    }
    true
}

fn append_singleton(
    stream: &mut Stream,
    seed: u64,
    route: usize,
    options: RenderOptions,
    occurrences: &mut u64,
) -> bool {
    let Some(lifecycle) = frozen_ir0::glue_lifecycle(
        seed,
        M2_ACQUISITION,
        route,
        false,
        options.reverse_allocation,
    ) else {
        return false;
    };
    if !lifecycle.source_ok
        || lifecycle.compatible_uses != 1
        || lifecycle.mismatches != 0
        || lifecycle.invalidations != 0
        || lifecycle.historical_returns != 1
        || !lifecycle.historical_unchanged
    {
        return false;
    }
    let index = stream.observations.len();
    let relation = relation_class(&lifecycle.original);
    let row = Observation {
        occurrence: next_occurrence(occurrences),
        shape: options.flat_shape.unwrap_or(41) ^ options.shape_xor,
        local_time: if options.reverse_time { 233 } else { 11 },
        propagation: propagation_class(&lifecycle.original),
        boundary_role: BoundaryRole::Singleton,
        causal_link: CausalLink::Reset,
        functional_relation: relation,
        ordinary_consequence: consequence(&lifecycle.original, options.consequence_delta),
    };
    stream.expected.push(ExpectedSpan {
        start: index,
        end: index,
        relation,
        roles: vec![BoundaryRole::Singleton],
        consequence: row.ordinary_consequence,
    });
    stream.observations.push(row);
    stream.organic_rows += 1;
    true
}

fn append_interruption(
    stream: &mut Stream,
    seed: u64,
    route: usize,
    stale: bool,
    occurrences: &mut u64,
) -> bool {
    if frozen_ac0::glue_abstention(seed, M2_ACQUISITION, route, stale) != Some(true) {
        return false;
    }
    stream.observations.push(Observation {
        occurrence: next_occurrence(occurrences),
        shape: 0,
        local_time: 0,
        propagation: 0,
        boundary_role: BoundaryRole::Interrupt,
        causal_link: CausalLink::Broken,
        functional_relation: 0,
        ordinary_consequence: 0,
    });
    stream.organic_rows += 1;
    true
}

fn standard_stream(seed: u64, options: RenderOptions) -> Option<Stream> {
    let mut stream = Stream::default();
    let mut occurrences = seed ^ 0xD53C_0000_0000_0001;
    for route in 0..ROUTES {
        if !append_lifecycle(
            &mut stream,
            seed + route as u64 * 101,
            route,
            3,
            options,
            &mut occurrences,
        ) {
            return None;
        }
    }
    Some(stream)
}

fn observed_spans(evaluation: &Evaluation) -> Vec<ExpectedSpan> {
    evaluation
        .spans
        .iter()
        .map(|span| ExpectedSpan {
            start: span.start,
            end: span.end,
            relation: span.functional_relation,
            roles: span.roles.clone(),
            consequence: span.ordinary_consequence,
        })
        .collect()
}

fn stream_legal(stream: &Stream) -> bool {
    let mut active = false;
    for row in &stream.observations {
        let pairing = match row.boundary_role {
            BoundaryRole::Open | BoundaryRole::Singleton => row.causal_link == CausalLink::Reset,
            BoundaryRole::Continue | BoundaryRole::Close => row.causal_link == CausalLink::Continue,
            BoundaryRole::Interrupt => row.causal_link == CausalLink::Broken,
        };
        if !pairing {
            return false;
        }
        match row.boundary_role {
            BoundaryRole::Open => active = true,
            BoundaryRole::Continue | BoundaryRole::Close if !active => return false,
            BoundaryRole::Close | BoundaryRole::Interrupt | BoundaryRole::Singleton => {
                active = false
            }
            BoundaryRole::Continue => {}
        }
    }
    true
}

fn exact_reconstruction(evaluation: &Evaluation, stream: &Stream) -> bool {
    observed_spans(evaluation) == stream.expected
}

fn consequence_parity(evaluation: &Evaluation, stream: &Stream) -> bool {
    evaluation
        .spans
        .iter()
        .map(|span| span.ordinary_consequence)
        .collect::<Vec<_>>()
        == stream
            .expected
            .iter()
            .map(|span| span.consequence)
            .collect::<Vec<_>>()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAudit {
    pub exact_parent: bool,
    pub mechanism_hash: bool,
    pub a1_hash: bool,
    pub ac0_hash: bool,
    pub ir0_hash: bool,
    pub protocol_hash: bool,
    pub expectation_hash: bool,
    pub frozen_mechanism_source: bool,
    pub a1_source: bool,
    pub ac0_source: bool,
    pub ir0_source: bool,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.exact_parent
            && self.mechanism_hash
            && self.a1_hash
            && self.ac0_hash
            && self.ir0_hash
            && self.protocol_hash
            && self.expectation_hash
            && self.frozen_mechanism_source
            && self.a1_source
            && self.ac0_source
            && self.ir0_source
    }
}

fn source_audit() -> SourceAudit {
    SourceAudit {
        exact_parent: EXACT_PARENT == "162a5b2082a8c1ac9ede45bc5178fecf3509b476"
            && AUTHORITATIVE_M1 == "16a1002b59bf0dbc23a6b6bf03572efca53b33ce",
        mechanism_hash: env!("DS3_CUM_MECHANISM_SHA256") == FROZEN_MECHANISM_SHA256,
        a1_hash: env!("DS3_CUM_A1_SHA256") == FROZEN_A1_SHA256,
        ac0_hash: env!("DS3_CUM_AC0_SHA256") == FROZEN_AC0_SHA256,
        ir0_hash: env!("DS3_CUM_IR0_SHA256") == FROZEN_IR0_SHA256,
        protocol_hash: env!("DS3_CUM_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        expectation_hash: env!("DS3_CUM_EXPECTATION_SHA256") == FROZEN_EXPECTATION_SHA256,
        frozen_mechanism_source: frozen_ds3::glue_mechanism_source_audit(),
        a1_source: frozen_a1::glue_source_ok(),
        ac0_source: frozen_ac0::glue_source_ok(),
        ir0_source: frozen_ir0::glue_source_ok(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlResult {
    pub number: usize,
    pub name: &'static str,
    pub passed: bool,
    pub diagnostic: String,
}

fn control(
    number: usize,
    name: &'static str,
    passed: bool,
    diagnostic: impl Into<String>,
) -> ControlResult {
    ControlResult {
        number,
        name,
        passed,
        diagnostic: diagnostic.into(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Probe {
    source: SourceAudit,
    wiring_legal: bool,
    reconstructability: bool,
    functional_adequacy: bool,
    controls: Vec<ControlResult>,
    acquisition_m2_work: u64,
    acquisition_observations: u64,
    candidate_comparisons: u64,
    generic_mature_work: u64,
    learned_mature_work: u64,
    held_out_used_learned: usize,
    persistent_bytes: usize,
    chunk_count: usize,
    held_out_seed_count: usize,
}

fn run_probe(seed: u64, acquisition_episodes: usize, held_out: usize) -> Probe {
    let source = source_audit();
    let mut learner = frozen_ds3::glue_default_boundary();
    let mut acquisition_m2_work = 0u64;
    let mut acquisition_observations = 0u64;
    let mut candidate_comparisons = 0u64;
    let mut acquisition_seeds = BTreeSet::new();
    let mut wiring_legal = true;
    for episode in 0..acquisition_episodes {
        let episode_seed = seed + episode as u64;
        acquisition_seeds.insert(episode_seed);
        let Some(stream) = standard_stream(episode_seed, RenderOptions::default()) else {
            wiring_legal = false;
            continue;
        };
        wiring_legal &= stream_legal(&stream) && stream.organic_rows == 6;
        acquisition_m2_work += stream.m2_work;
        let evaluation = frozen_ds3::glue_evaluate(&mut learner, &stream.observations, true);
        acquisition_observations += evaluation.work.acquisition_observations;
        candidate_comparisons += evaluation.work.candidate_comparisons;
    }

    let mut reconstructability = true;
    let mut functional_adequacy = true;
    let mut held_out_used_learned = 0usize;
    let mut generic_mature_work = 0u64;
    let mut learned_mature_work = 0u64;
    let mut held_out_seeds = BTreeSet::new();
    for episode in 0..held_out {
        let episode_seed = seed + 10_000 + episode as u64;
        held_out_seeds.insert(episode_seed);
        let Some(stream) = standard_stream(episode_seed, RenderOptions::default()) else {
            wiring_legal = false;
            reconstructability = false;
            functional_adequacy = false;
            continue;
        };
        wiring_legal &= stream_legal(&stream);
        let evaluation = frozen_ds3::glue_evaluate(&mut learner, &stream.observations, false);
        reconstructability &= exact_reconstruction(&evaluation, &stream);
        functional_adequacy &= consequence_parity(&evaluation, &stream);
        held_out_used_learned += evaluation.used_learned;
        generic_mature_work += evaluation.work.generic_transition_checks
            + evaluation.work.completed_spans
            + evaluation.work.propagated_consequences;
        learned_mature_work += evaluation.work.learned_signature_checks;
    }

    // C1: shape-identical activity follows two different organic lifecycle cuts.
    let flat = RenderOptions {
        flat_shape: Some(9),
        ..RenderOptions::default()
    };
    let flat_standard = standard_stream(seed + 20_001, flat);
    let mut regrouped = Stream::default();
    let mut regrouped_occurrences = seed ^ 0xC001;
    let regrouped_ok = append_singleton(
        &mut regrouped,
        seed + 20_002,
        0,
        flat,
        &mut regrouped_occurrences,
    ) && append_lifecycle(
        &mut regrouped,
        seed + 20_003,
        1,
        3,
        flat,
        &mut regrouped_occurrences,
    );
    let (control1, grouping_diagnostic) = match flat_standard {
        Some(flat_stream) if regrouped_ok => {
            let first = frozen_ds3::glue_evaluate(&mut learner, &flat_stream.observations, false);
            let second = frozen_ds3::glue_evaluate(&mut learner, &regrouped.observations, false);
            (
                exact_reconstruction(&first, &flat_stream)
                    && exact_reconstruction(&second, &regrouped)
                    && observed_spans(&first) != observed_spans(&second),
                format!(
                    "standard_spans={} regrouped_spans={}",
                    first.spans.len(),
                    second.spans.len()
                ),
            )
        }
        _ => (false, "organic regrouping fixture unavailable".to_string()),
    };

    // C2/C5: surface shape, clock, and consequence relabellings are not cuts.
    let relabelled = standard_stream(
        seed + 20_010,
        RenderOptions {
            shape_xor: 0xA7,
            consequence_delta: 31,
            reverse_time: true,
            relabel: true,
            reverse_allocation: true,
            ..RenderOptions::default()
        },
    );
    let (control2, control5, control6) = match relabelled {
        Some(stream) => {
            let evaluation = frozen_ds3::glue_evaluate(&mut learner, &stream.observations, false);
            let exact = exact_reconstruction(&evaluation, &stream);
            (
                exact,
                exact && consequence_parity(&evaluation, &stream),
                exact && evaluation.used_learned == stream.expected.len(),
            )
        }
        None => (false, false, false),
    };

    // C3: the cut moves from two 3-row units to singleton + 3-row lifecycle.
    let control3 = regrouped_ok && stream_legal(&regrouped) && {
        let evaluation = frozen_ds3::glue_evaluate(&mut learner, &regrouped.observations, false);
        exact_reconstruction(&evaluation, &regrouped)
            && regrouped
                .expected
                .iter()
                .map(|span| span.end - span.start + 1)
                .collect::<Vec<_>>()
                == vec![1, 3]
    };

    // C4: blocked/stale AC0 actuation interrupts; only the new opening completes.
    let mut interrupted = Stream::default();
    let mut interruption_occurrences = seed ^ 0xC004;
    let interruption_fixture = append_lifecycle(
        &mut interrupted,
        seed + 20_020,
        0,
        2,
        RenderOptions::default(),
        &mut interruption_occurrences,
    ) && append_interruption(
        &mut interrupted,
        seed + 20_021,
        0,
        true,
        &mut interruption_occurrences,
    ) && append_lifecycle(
        &mut interrupted,
        seed + 20_022,
        1,
        3,
        RenderOptions::default(),
        &mut interruption_occurrences,
    );
    let control4 = interruption_fixture && {
        let evaluation = frozen_ds3::glue_evaluate(&mut learner, &interrupted.observations, false);
        exact_reconstruction(&evaluation, &interrupted)
            && evaluation.spans.len() == 1
            && evaluation.invalidations > 0
    };

    // C7: frozen persistent block and all parent mechanisms pass their audits.
    let control7 = source.passed();

    // C8: actual IR0 mismatch invalidates, abstains, generically reopens, and executes.
    let ir0_changed = frozen_ir0::glue_lifecycle(seed + 20_030, M2_ACQUISITION, 0, true, false);
    let ir0_layout = frozen_ir0::glue_lifecycle(seed + 20_031, M2_ACQUISITION, 1, true, true);
    let control8 = [ir0_changed.as_ref(), ir0_layout.as_ref()]
        .into_iter()
        .flatten()
        .count()
        == 2
        && [ir0_changed.as_ref(), ir0_layout.as_ref()]
            .into_iter()
            .flatten()
            .all(|value| {
                value.source_ok
                    && value.original != value.current
                    && value.compatible_uses == 1
                    && value.mismatches == 1
                    && value.invalidations == 1
                    && value.invalidated_abstentions == 1
                    && value.reopenings == 1
                    && value.reopened_executions == 1
                    && value.historical_returns == 1
                    && value.historical_unchanged
            });

    // C9: one complete observation of a signature remains below DS3 support 2.
    let mut subthreshold_learner = frozen_ds3::glue_default_boundary();
    let mut subthreshold = Stream::default();
    let mut subthreshold_occurrences = seed ^ 0xC009;
    let subthreshold_fixture = append_lifecycle(
        &mut subthreshold,
        seed + 20_040,
        0,
        3,
        RenderOptions::default(),
        &mut subthreshold_occurrences,
    );
    if subthreshold_fixture {
        let _ =
            frozen_ds3::glue_evaluate(&mut subthreshold_learner, &subthreshold.observations, true);
    }
    let control9 = subthreshold_fixture && frozen_ds3::glue_chunk_count(&subthreshold_learner) == 0;

    // C10: probation without threshold-crossing installation has no Close and fails closed.
    let mut missing_close = Stream::default();
    let mut missing_occurrences = seed ^ 0xC010;
    let missing_fixture = append_lifecycle(
        &mut missing_close,
        seed + 20_050,
        0,
        2,
        RenderOptions::default(),
        &mut missing_occurrences,
    );
    let missing_evaluation =
        frozen_ds3::glue_evaluate(&mut subthreshold_learner, &missing_close.observations, true);
    let control10 =
        missing_fixture && missing_close.expected.is_empty() && missing_evaluation.spans.is_empty();

    // C11: evaluator-only negative mutation. This stream is never acquired.
    let mut invalid_transition = standard_stream(seed + 20_060, RenderOptions::default());
    let control11 = invalid_transition.as_mut().is_some_and(|stream| {
        if stream.observations.len() < 2 {
            return false;
        }
        stream.observations[1].causal_link = CausalLink::Reset;
        let evaluation = frozen_ds3::glue_evaluate(&mut learner, &stream.observations, false);
        evaluation.spans.len() < stream.expected.len() && evaluation.invalidations > 0
    });

    // C12: evaluation populations are seed-disjoint and non-plastic.
    let chunks_before_held_out_control = frozen_ds3::glue_chunk_count(&learner);
    let held_out_control = standard_stream(seed + 30_000, RenderOptions::default());
    let control12 = held_out_control.is_some_and(|stream| {
        let evaluation = frozen_ds3::glue_evaluate(&mut learner, &stream.observations, false);
        acquisition_seeds.is_disjoint(&held_out_seeds)
            && !acquisition_seeds.contains(&(seed + 30_000))
            && exact_reconstruction(&evaluation, &stream)
            && evaluation.used_learned == stream.expected.len()
            && frozen_ds3::glue_chunk_count(&learner) == chunks_before_held_out_control
    });

    let controls = vec![
        control(
            1,
            "identical-local-shapes-different-grouping",
            control1,
            grouping_diagnostic,
        ),
        control(
            2,
            "different-shapes-same-functional-span",
            control2,
            "surface shape relabelling preserves learned cuts",
        ),
        control(
            3,
            "boundary-shifts",
            control3,
            "organic retained-use and probation signals move span lengths to [1,3]",
        ),
        control(
            4,
            "interruptions-and-reentry",
            control4,
            "AC0 abstention interrupts; only a new opening completes",
        ),
        control(
            5,
            "shuffled-timing-relabeled-consequences",
            control5,
            "clock and consequence relabellings are not grouping keys",
        ),
        control(
            6,
            "fresh-identities-and-allocation",
            control6,
            "fresh E0 identities and reverse allocation reuse learned signatures",
        ),
        control(
            7,
            "leak-source-audit",
            control7,
            format!("source={source:?}"),
        ),
        control(
            8,
            "invalidation-generic-reopening-reacquisition",
            control8,
            "IR0 mismatch invalidates, abstains, reopens, and executes",
        ),
        control(
            9,
            "subthreshold-recurrence",
            control9,
            format!(
                "chunks={}",
                frozen_ds3::glue_chunk_count(&subthreshold_learner)
            ),
        ),
        control(
            10,
            "missing-close",
            control10,
            format!("completed={}", missing_evaluation.spans.len()),
        ),
        control(
            11,
            "invalid-causal-transition",
            control11,
            "reset without opening is a control-only mutation and fails closed",
        ),
        control(
            12,
            "held-out-population",
            control12,
            format!(
                "acquisition={} held_out={}",
                acquisition_seeds.len(),
                held_out_seeds.len()
            ),
        ),
    ];

    Probe {
        source,
        wiring_legal,
        reconstructability,
        functional_adequacy,
        controls,
        acquisition_m2_work,
        acquisition_observations,
        candidate_comparisons,
        generic_mature_work,
        learned_mature_work,
        held_out_used_learned,
        persistent_bytes: frozen_ds3::glue_persistent_bytes(&learner),
        chunk_count: frozen_ds3::glue_chunk_count(&learner),
        held_out_seed_count: held_out_seeds.len(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub development_ready: bool,
    pub m2_authoritative: bool,
    pub m3_exists: bool,
    pub source: SourceAudit,
    pub stages: [String; 6],
    pub first_collapse_stage: Option<usize>,
    pub first_collapse: String,
    pub reconstructability: bool,
    pub functional_adequacy: bool,
    pub duplicate_deterministic: bool,
    pub work_attributed: bool,
    pub acquisition_m2_work: u64,
    pub acquisition_observations: u64,
    pub candidate_comparisons: u64,
    pub generic_mature_work: u64,
    pub learned_mature_work: u64,
    pub held_out_used_learned: usize,
    pub persistent_bytes: usize,
    pub chunk_count: usize,
    pub held_out_seed_count: usize,
    pub controls: Vec<ControlResult>,
}

fn forbidden_report() -> Report {
    Report {
        label: "DS3-CUMULATIVE DEFINITIVE FORBIDDEN".to_string(),
        protocol: PROTOCOL.to_string(),
        mode: "DEFINITIVE-FORBIDDEN".to_string(),
        claim_eligible: false,
        development_ready: false,
        m2_authoritative: true,
        m3_exists: false,
        source: source_audit(),
        stages: std::array::from_fn(|_| "BLOCKED".to_string()),
        first_collapse_stage: None,
        first_collapse: "separate definitive matrix preregistration required".to_string(),
        reconstructability: false,
        functional_adequacy: false,
        duplicate_deterministic: false,
        work_attributed: false,
        acquisition_m2_work: 0,
        acquisition_observations: 0,
        candidate_comparisons: 0,
        generic_mature_work: 0,
        learned_mature_work: 0,
        held_out_used_learned: 0,
        persistent_bytes: 0,
        chunk_count: 0,
        held_out_seed_count: 0,
        controls: Vec::new(),
    }
}

pub fn run(mode: HarnessMode) -> Report {
    if mode == HarnessMode::Definitive {
        return forbidden_report();
    }
    let (mode_name, seed, acquisition, held_out) = match mode {
        HarnessMode::Micro => ("MICRO", 83_000, 2, 2),
        HarnessMode::Gate => ("GATE", 84_000, 6, 8),
        HarnessMode::Definitive => unreachable!(),
    };
    let first = run_probe(seed, acquisition, held_out);
    let second = run_probe(seed, acquisition, held_out);
    let duplicate_deterministic = first == second;
    let work_attributed = first.acquisition_m2_work > 0
        && first.acquisition_observations > 0
        && first.candidate_comparisons > 0
        && first.generic_mature_work > 0
        && first.learned_mature_work > 0
        && first.held_out_used_learned >= held_out * ROUTES
        && first.persistent_bytes > 0
        && first.chunk_count >= ROUTES;
    let controls_passed =
        first.controls.len() == 12 && first.controls.iter().all(|control| control.passed);
    let ready = [
        first.source.passed(),
        first.wiring_legal,
        first.reconstructability,
        first.functional_adequacy,
        controls_passed,
        duplicate_deterministic && work_attributed,
    ];
    let first_collapse_stage = ready.iter().position(|value| !value);
    let stages = std::array::from_fn(|stage| match first_collapse_stage {
        None => "READY".to_string(),
        Some(collapse) if stage < collapse => "READY".to_string(),
        Some(collapse) if stage == collapse => format!("COLLAPSE: {}", STAGES[stage]),
        Some(_) => "BLOCKED".to_string(),
    });
    let first_collapse = first_collapse_stage
        .map(|stage| {
            if stage == 4 {
                first
                    .controls
                    .iter()
                    .find(|control| !control.passed)
                    .map(|control| format!("P4/control {} {}", control.number, control.name))
                    .unwrap_or_else(|| STAGES[stage].to_string())
            } else {
                STAGES[stage].to_string()
            }
        })
        .unwrap_or_else(|| "NONE".to_string());
    let development_ready = first_collapse_stage.is_none();
    Report {
        label: if development_ready {
            "DS3-CUMULATIVE DEVELOPMENT READY".to_string()
        } else {
            format!("DS3-CUMULATIVE COLLAPSE AT {first_collapse}")
        },
        protocol: PROTOCOL.to_string(),
        mode: mode_name.to_string(),
        claim_eligible: false,
        development_ready,
        m2_authoritative: true,
        m3_exists: false,
        source: first.source,
        stages,
        first_collapse_stage,
        first_collapse,
        reconstructability: first.reconstructability,
        functional_adequacy: first.functional_adequacy,
        duplicate_deterministic,
        work_attributed,
        acquisition_m2_work: first.acquisition_m2_work,
        acquisition_observations: first.acquisition_observations,
        candidate_comparisons: first.candidate_comparisons,
        generic_mature_work: first.generic_mature_work,
        learned_mature_work: first.learned_mature_work,
        held_out_used_learned: first.held_out_used_learned,
        persistent_bytes: first.persistent_bytes,
        chunk_count: first.chunk_count,
        held_out_seed_count: first.held_out_seed_count,
        controls: first.controls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_reaches_an_ordered_development_result() {
        let report = run(HarnessMode::Micro);
        assert_eq!(report.controls.len(), 12, "{report:#?}");
        assert!(!report.claim_eligible && report.m2_authoritative && !report.m3_exists);
        assert!(report.first_collapse_stage.is_none(), "{report:#?}");
    }

    #[test]
    fn definitive_stays_locked() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.development_ready && !report.claim_eligible);
        assert!(report.controls.is_empty());
    }
}
