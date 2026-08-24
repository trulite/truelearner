//! Development-only structural invalidation and generic reopening for retained DS2 direction.

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-ir0-dependency-invalidation-reopening-v1";
pub const EXACT_PARENT: &str = "d97f5038e6133a0abe4b24ea3b8eb5b4ba7cd4f4";
pub const PROTOCOL_COMMIT: &str = "89bc6ea0cce18cf00d09e192339772502578907d";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_RT0_SHA256: &str =
    "16ef4e2a691e22251d109860ac055c5a1ee78f586ad9335a375589336ad78ed0";
pub const FROZEN_CP0_SHA256: &str =
    "c9fcc53d03296b169060499e2304de557f3f7a93744dbc1f935053f99d41c498";
pub const FROZEN_A1_SHA256: &str =
    "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1";
pub const FROZEN_PARENT_SHA256: &str =
    "59d4e5f7efd29454ee3c9bb3e0a761b0ed85e369ba6081f01a01909b2d5c0a0d";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "348b4d74418493305ab083e7828ab89198d975fb281e28a1519234b1cf55437b";

const ACQUISITION: usize = 16;
const CONTEXTS: usize = 4;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct RouteShape {
    trace: Vec<u8>,
    activation: [u16; 3],
}

macro_rules! ir0_rt0_access {
    () => {
        pub(super) fn ir0_signal(
            seed: u64,
            acquisition: usize,
            recurrent: usize,
            relabel: bool,
            reverse_allocation: bool,
            layout_padding: bool,
        ) -> Option<super::RouteShape> {
            frozen_cp0::rt0_frozen_signal(
                seed,
                acquisition,
                SignalMode::Recurrent(recurrent),
                relabel,
                reverse_allocation,
                layout_padding,
            )
            .map(|shape| super::RouteShape {
                trace: shape.trace,
                activation: shape.activation,
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_rt0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_rt0_retained_direction_execution.rs"
    ));
    ir0_rt0_access!();
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct LifecycleOptions {
    reverse_allocation: bool,
    layout_padding: bool,
    permute_handles: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct LifecycleResult {
    initial_counts: [u16; 2],
    historical_counts_before: [u16; 2],
    historical_counts_after: [u16; 2],
    compatible_retained_uses: usize,
    structural_mismatches: usize,
    stale_routes_invalidated: usize,
    invalidated_route_abstentions: usize,
    generic_reopenings: usize,
    reopened_executions: usize,
    historical_return_uses: usize,
    historical_reacquisition_calls: usize,
    false_invalidations: usize,
    persistent_bytes: usize,
    retained_occurrences: usize,
    retained_handles: usize,
}

macro_rules! ir0_a1_access {
    () => {
        fn ir0_route_views(
            seed: u64,
            acquisition: usize,
            options: super::LifecycleOptions,
        ) -> Option<(Vec<super::RouteShape>, Vec<Vec<[CellId; 2]>>, Vec<u32>)> {
            let bundle = frozen_e0::a1_bundle(seed, acquisition)?;
            let mut installer = train(&bundle.support, false)?;
            let mut substrate = substrate_from_export(
                &bundle.target,
                MappingOptions {
                    reverse_allocation: options.reverse_allocation,
                    layout_padding: options.layout_padding,
                    ..MappingOptions::default()
                },
            )?;
            let (_, installed) = installer.install(&mut substrate, true, false);
            let roots = structural_dedup(&mut substrate, &installed, &mut installer.work);
            let bridge = expose_roots(&roots, options.permute_handles, &mut installer.work);
            if roots.len() != 2 || bridge.entries.len() != 2 {
                return None;
            }
            let mut shapes = Vec::new();
            let mut observations = Vec::new();
            for entry in &bridge.entries {
                let effect =
                    execute_handle(&substrate, &bridge, entry.handle, &mut installer.work)?;
                shapes.push(super::RouteShape {
                    trace: effect.trace.clone(),
                    activation: effect.activation,
                });
                observations.push(
                    effect
                        .trace
                        .windows(2)
                        .map(|trace| {
                            [
                                substrate.members[usize::from(trace[0])],
                                substrate.members[usize::from(trace[1])],
                            ]
                        })
                        .collect(),
                );
            }
            let occurrences = substrate.cells.iter().map(|cell| cell.occurrence).collect();
            Some((shapes, observations, occurrences))
        }

        fn ir0_ground(
            asset: &mut Learner,
            seed: u64,
            acquisition: usize,
            options: super::LifecycleOptions,
        ) -> Option<(Substrate, Vec<RouteRoot>, Vec<NormalizedEffect>)> {
            let bundle = frozen_e0::a1_bundle(seed, acquisition)?;
            let mut substrate = substrate_from_export(
                &bundle.target,
                MappingOptions {
                    reverse_allocation: options.reverse_allocation,
                    layout_padding: options.layout_padding,
                    ..MappingOptions::default()
                },
            )?;
            let (_, installed) = asset.install(&mut substrate, true, false);
            let roots = structural_dedup(&mut substrate, &installed, &mut asset.work);
            let effects = roots
                .iter()
                .map(|root| execute_root(&substrate, *root, &mut asset.work))
                .collect::<Option<Vec<_>>>()?;
            Some((substrate, roots, effects))
        }

        pub(super) fn ir0_lifecycle(
            seed: u64,
            acquisition: usize,
            original_signal: &super::RouteShape,
            current_signal: Option<&super::RouteShape>,
            return_signal: &super::RouteShape,
            options: super::LifecycleOptions,
        ) -> Option<super::LifecycleResult> {
            let (route_shapes, route_observations, _) =
                ir0_route_views(seed, acquisition, options)?;
            let templates_for = |learner: &mut Learner| -> Option<Vec<LocalTemplate>> {
                let bundle = frozen_e0::a1_bundle(seed, acquisition)?;
                let mut substrate = substrate_from_export(
                    &bundle.target,
                    MappingOptions {
                        reverse_allocation: options.reverse_allocation,
                        layout_padding: options.layout_padding,
                        ..MappingOptions::default()
                    },
                )?;
                let mut templates = Vec::new();
                for observations in &route_observations {
                    substrate.observations.clone_from(observations);
                    let proposals = local_proposals(&substrate, &mut learner.work);
                    if proposals.len() != 1 {
                        return None;
                    }
                    templates.push(proposals[0].template);
                    let _ = learner.observe(&substrate, true);
                }
                Some(templates)
            };

            let mut historical = Learner::default();
            let templates = templates_for(&mut historical)?;
            let original_index = route_shapes
                .iter()
                .position(|shape| shape == original_signal)?;
            let bundle = frozen_e0::a1_bundle(seed, acquisition)?;
            let mut support_substrate = substrate_from_export(
                &bundle.target,
                MappingOptions {
                    reverse_allocation: options.reverse_allocation,
                    layout_padding: options.layout_padding,
                    ..MappingOptions::default()
                },
            )?;
            support_substrate
                .observations
                .clone_from(&route_observations[original_index]);
            for _ in 1..SUPPORT_EPISODES {
                let _ = historical.observe(&support_substrate, true);
            }
            let initial_counts = [1, 1];
            let historical_counts_before = [
                historical.templates.get(&templates[0])?.count,
                historical.templates.get(&templates[1])?.count,
            ];

            // FREEZE_HISTORICAL_ASSET: historical.observe is forbidden below.
            let mut compatible_asset = historical.clone();
            let (_, compatible_roots, compatible_effects) =
                ir0_ground(&mut compatible_asset, seed + 500_000, acquisition, options)?;
            let compatible_retained_uses = usize::from(
                compatible_roots.len() == 1
                    && compatible_effects.len() == 1
                    && super::RouteShape {
                        trace: compatible_effects[0].trace.clone(),
                        activation: compatible_effects[0].activation,
                    } == *original_signal,
            );

            let mut structural_mismatches = 0usize;
            let mut stale_routes_invalidated = 0usize;
            let mut invalidated_route_abstentions = 0usize;
            let mut generic_reopenings = 0usize;
            let mut reopened_executions = 0usize;
            let false_invalidations = 0usize;
            if let Some(current) = current_signal {
                let mut stale_asset = historical.clone();
                let (mut stale_substrate, stale_roots, stale_effects) =
                    ir0_ground(&mut stale_asset, seed + 600_000, acquisition, options)?;
                let stale_shape = (stale_effects.len() == 1).then(|| super::RouteShape {
                    trace: stale_effects[0].trace.clone(),
                    activation: stale_effects[0].activation,
                });
                if stale_shape.as_ref().is_some_and(|shape| shape != current) {
                    structural_mismatches += 1;
                    for root in &stale_roots {
                        for arrow in &mut stale_substrate.arrows {
                            if arrow.endpoints[0] == root.cell && arrow.live {
                                arrow.live = false;
                                stale_routes_invalidated += 1;
                            }
                        }
                    }
                    invalidated_route_abstentions += usize::from(stale_roots.iter().all(|root| {
                        execute_root(&stale_substrate, *root, &mut stale_asset.work).is_none()
                    }));

                    let mut reopened = Learner::default();
                    let reopened_templates = templates_for(&mut reopened)?;
                    let current_index = route_shapes.iter().position(|shape| shape == current)?;
                    let mut reopen_substrate = substrate_from_export(
                        &bundle.target,
                        MappingOptions {
                            reverse_allocation: options.reverse_allocation,
                            layout_padding: options.layout_padding,
                            ..MappingOptions::default()
                        },
                    )?;
                    reopen_substrate
                        .observations
                        .clone_from(&route_observations[current_index]);
                    for _ in 1..SUPPORT_EPISODES {
                        let _ = reopened.observe(&reopen_substrate, true);
                    }
                    let reopened_counts = [
                        reopened.templates.get(&reopened_templates[0])?.count,
                        reopened.templates.get(&reopened_templates[1])?.count,
                    ];
                    generic_reopenings += usize::from(reopened_counts != [1, 1]);
                    let (_, reopened_roots, reopened_effects) =
                        ir0_ground(&mut reopened, seed + 700_000, acquisition, options)?;
                    reopened_executions += usize::from(
                        reopened_roots.len() == 1
                            && reopened_effects.len() == 1
                            && super::RouteShape {
                                trace: reopened_effects[0].trace.clone(),
                                activation: reopened_effects[0].activation,
                            } == *current,
                    );
                }
            }

            let mut return_asset = historical.clone();
            let (_, return_roots, return_effects) =
                ir0_ground(&mut return_asset, seed + 800_000, acquisition, options)?;
            let historical_return_uses = usize::from(
                return_roots.len() == 1
                    && return_effects.len() == 1
                    && super::RouteShape {
                        trace: return_effects[0].trace.clone(),
                        activation: return_effects[0].activation,
                    } == *return_signal,
            );
            let historical_counts_after = [
                historical.templates.get(&templates[0])?.count,
                historical.templates.get(&templates[1])?.count,
            ];
            Some(super::LifecycleResult {
                initial_counts,
                historical_counts_before,
                historical_counts_after,
                compatible_retained_uses,
                structural_mismatches,
                stale_routes_invalidated,
                invalidated_route_abstentions,
                generic_reopenings,
                reopened_executions,
                historical_return_uses,
                historical_reacquisition_calls: 0,
                false_invalidations,
                persistent_bytes: historical.work.persistent_bytes,
                retained_occurrences: 0,
                retained_handles: 0,
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
    ir0_a1_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub rt0_hash: bool,
    pub cp0_hash: bool,
    pub a1_hash: bool,
    pub parent_hash: bool,
    pub protocol_hash: bool,
    pub frozen_signal_calls: usize,
    pub existing_observe_sites: usize,
    pub existing_install_sites: usize,
    pub existing_execute_sites: usize,
    pub structural_mismatch_sites: usize,
    pub invalidation_sites: usize,
    pub generic_reopening_sites: usize,
    pub historical_observe_after_freeze: usize,
    pub direct_support_mutations: usize,
    pub new_learner_or_candidate_types: usize,
    pub semantic_fields: usize,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.rt0_hash
            && self.cp0_hash
            && self.a1_hash
            && self.parent_hash
            && self.protocol_hash
            && self.frozen_signal_calls == 1
            && self.existing_observe_sites == 3
            && self.existing_install_sites == 1
            && self.existing_execute_sites == 2
            && self.structural_mismatch_sites == 1
            && self.invalidation_sites == 1
            && self.generic_reopening_sites == 1
            && self.historical_observe_after_freeze == 0
            && self.direct_support_mutations == 0
            && self.new_learner_or_candidate_types == 0
            && self.semantic_fields == 0
    }
}

fn function_body<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
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

fn source_audit() -> SourceAudit {
    let source = include_str!("ds_ir0_dependency_invalidation_reopening.rs");
    let production = source
        .split(&["#[cfg(", "test)]"].concat())
        .next()
        .unwrap_or(source);
    let signal = function_body(production, "pub(super) fn ir0_signal(").unwrap_or_default();
    let lifecycle = function_body(production, "pub(super) fn ir0_lifecycle(").unwrap_or_default();
    let ground = function_body(production, "fn ir0_ground(").unwrap_or_default();
    let after_freeze = lifecycle
        .split("FREEZE_HISTORICAL_ASSET")
        .nth(1)
        .unwrap_or_default();
    let forbidden = [
        ["correct", "ness"].concat(),
        ["reward", "_value"].concat(),
        ["semantic", "_polarity"].concat(),
        ["causal", "_label"].concat(),
        ["evaluator", "_direction"].concat(),
    ];
    SourceAudit {
        rt0_hash: env!("DS_IR0_RT0_SHA256") == FROZEN_RT0_SHA256,
        cp0_hash: env!("DS_IR0_CP0_SHA256") == FROZEN_CP0_SHA256,
        a1_hash: env!("DS_IR0_A1_SHA256") == FROZEN_A1_SHA256,
        parent_hash: env!("DS_IR0_PARENT_SHA256") == FROZEN_PARENT_SHA256,
        protocol_hash: env!("DS_IR0_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        frozen_signal_calls: signal.matches("rt0_frozen_signal(").count(),
        existing_observe_sites: lifecycle.matches(".observe(").count(),
        existing_install_sites: ground.matches("asset.install(").count(),
        existing_execute_sites: ground.matches("execute_root(").count()
            + lifecycle.matches("execute_root(&stale_substrate").count(),
        structural_mismatch_sites: lifecycle.matches("shape != current").count(),
        invalidation_sites: lifecycle.matches("arrow.live = false").count(),
        generic_reopening_sites: lifecycle
            .matches("let mut reopened = Learner::default()")
            .count(),
        historical_observe_after_freeze: after_freeze.matches("historical.observe(").count(),
        direct_support_mutations: [".count +=", ".count -=", "templates.insert("]
            .iter()
            .map(|token| lifecycle.matches(token).count())
            .sum(),
        new_learner_or_candidate_types: production
            .matches(&["struct ", "Learner"].concat())
            .count()
            + production
                .matches(&["struct ", "Candidate"].concat())
                .count(),
        semantic_fields: forbidden
            .iter()
            .map(|token| lifecycle.matches(token).count())
            .sum(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub contexts: usize,
    pub changed_lifecycles: usize,
    pub compatible_uses: usize,
    pub compatible_preservations: usize,
    pub invalidations: usize,
    pub reopenings: usize,
    pub reopened_executions: usize,
    pub historical_returns: usize,
    pub ambiguous_preservations: usize,
    pub layout_transfers: usize,
    pub persistent_bytes: usize,
    pub passed: bool,
}

fn changed_passes(result: &LifecycleResult) -> bool {
    result.initial_counts == [1, 1]
        && result.historical_counts_before == result.historical_counts_after
        && result.compatible_retained_uses == 1
        && result.structural_mismatches == 1
        && result.stale_routes_invalidated == 1
        && result.invalidated_route_abstentions == 1
        && result.generic_reopenings == 1
        && result.reopened_executions == 1
        && result.historical_return_uses == 1
        && result.historical_reacquisition_calls == 0
        && result.false_invalidations == 0
        && result.retained_occurrences == 0
        && result.retained_handles == 0
}

fn audit_seed(seed: u64, source: &SourceAudit) -> SeedAudit {
    let mut changed_lifecycles = 0usize;
    let mut compatible_uses = 0usize;
    let mut compatible_preservations = 0usize;
    let mut invalidations = 0usize;
    let mut reopenings = 0usize;
    let mut reopened_executions = 0usize;
    let mut historical_returns = 0usize;
    let mut ambiguous_preservations = 0usize;
    let mut layout_transfers = 0usize;
    let mut persistent_bytes = 0usize;
    for context in 0..CONTEXTS {
        let route_seed = seed + 1_000 + context as u64 * 17;
        let original = (seed as usize + context) % 2;
        let original_signal =
            frozen_rt0::ir0_signal(route_seed, ACQUISITION, original, false, false, false)
                .expect("original physical signal");
        let changed_signal =
            frozen_rt0::ir0_signal(route_seed, ACQUISITION, 1 - original, false, false, false)
                .expect("changed physical signal");
        let changed = frozen_a1::ir0_lifecycle(
            route_seed,
            ACQUISITION,
            &original_signal,
            Some(&changed_signal),
            &original_signal,
            LifecycleOptions::default(),
        )
        .expect("changed lifecycle");
        changed_lifecycles += usize::from(changed_passes(&changed));
        compatible_uses += changed.compatible_retained_uses;
        invalidations += changed.stale_routes_invalidated;
        reopenings += changed.generic_reopenings;
        reopened_executions += changed.reopened_executions;
        historical_returns += changed.historical_return_uses;
        persistent_bytes = persistent_bytes.max(changed.persistent_bytes);

        let compatible = frozen_a1::ir0_lifecycle(
            route_seed,
            ACQUISITION,
            &original_signal,
            Some(&original_signal),
            &original_signal,
            LifecycleOptions::default(),
        )
        .expect("compatible lifecycle");
        compatible_preservations += usize::from(
            compatible.compatible_retained_uses == 1
                && compatible.structural_mismatches == 0
                && compatible.stale_routes_invalidated == 0
                && compatible.generic_reopenings == 0
                && compatible.false_invalidations == 0
                && compatible.historical_return_uses == 1
                && compatible.historical_counts_before == compatible.historical_counts_after,
        );

        let ambiguous = frozen_a1::ir0_lifecycle(
            route_seed,
            ACQUISITION,
            &original_signal,
            None,
            &original_signal,
            LifecycleOptions::default(),
        )
        .expect("ambiguous lifecycle");
        ambiguous_preservations += usize::from(
            ambiguous.stale_routes_invalidated == 0
                && ambiguous.generic_reopenings == 0
                && ambiguous.historical_return_uses == 1
                && ambiguous.historical_counts_before == ambiguous.historical_counts_after,
        );

        let layout_original =
            frozen_rt0::ir0_signal(route_seed, ACQUISITION, original, true, true, true)
                .expect("layout original signal");
        let layout_changed =
            frozen_rt0::ir0_signal(route_seed, ACQUISITION, 1 - original, true, true, true)
                .expect("layout changed signal");
        let layout = frozen_a1::ir0_lifecycle(
            route_seed,
            ACQUISITION,
            &layout_original,
            Some(&layout_changed),
            &layout_original,
            LifecycleOptions {
                reverse_allocation: true,
                layout_padding: true,
                permute_handles: true,
            },
        )
        .expect("layout lifecycle");
        layout_transfers += usize::from(changed_passes(&layout));
    }
    let passed = source.passed()
        && changed_lifecycles == CONTEXTS
        && compatible_uses == CONTEXTS
        && compatible_preservations == CONTEXTS
        && invalidations == CONTEXTS
        && reopenings == CONTEXTS
        && reopened_executions == CONTEXTS
        && historical_returns == CONTEXTS
        && ambiguous_preservations == CONTEXTS
        && layout_transfers == CONTEXTS
        && persistent_bytes > 0;
    SeedAudit {
        seed,
        contexts: CONTEXTS,
        changed_lifecycles,
        compatible_uses,
        compatible_preservations,
        invalidations,
        reopenings,
        reopened_executions,
        historical_returns,
        ambiguous_preservations,
        layout_transfers,
        persistent_bytes,
        passed,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub mode: String,
    pub claim_eligible: bool,
    pub enabling_only: bool,
    pub m1_authoritative: bool,
    pub m2_exists: bool,
    pub source: SourceAudit,
    pub seeds: Vec<SeedAudit>,
    pub passed: bool,
}

pub fn run(mode: HarnessMode) -> Report {
    if mode == HarnessMode::Definitive {
        return Report {
            mode: "DEFINITIVE-FORBIDDEN".to_string(),
            claim_eligible: false,
            enabling_only: true,
            m1_authoritative: true,
            m2_exists: false,
            source: SourceAudit::default(),
            seeds: Vec::new(),
            passed: false,
        };
    }
    let source = source_audit();
    let seeds = match mode {
        HarnessMode::Micro => vec![audit_seed(100, &source)],
        HarnessMode::Gate => (100..105).map(|seed| audit_seed(seed, &source)).collect(),
        HarnessMode::Definitive => unreachable!(),
    };
    let passed = seeds.iter().all(|seed| seed.passed);
    Report {
        mode: match mode {
            HarnessMode::Micro => "MICRO",
            HarnessMode::Gate => "GATE",
            HarnessMode::Definitive => unreachable!(),
        }
        .to_string(),
        claim_eligible: false,
        enabling_only: true,
        m1_authoritative: true,
        m2_exists: false,
        source,
        seeds,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_invalidates_reopens_and_reuses_history() {
        let report = run(HarnessMode::Micro);
        assert!(report.passed, "{report:#?}");
        assert_eq!(report.seeds[0].changed_lifecycles, CONTEXTS);
        assert_eq!(report.seeds[0].historical_returns, CONTEXTS);
    }

    #[test]
    fn gate_passes_invalidation_reopening_controls() {
        let report = run(HarnessMode::Gate);
        assert!(report.passed, "{report:#?}");
        assert_eq!(report.seeds.len(), 5);
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.passed);
        assert!(report.seeds.is_empty());
        assert!(report.m1_authoritative && !report.m2_exists);
    }
}
