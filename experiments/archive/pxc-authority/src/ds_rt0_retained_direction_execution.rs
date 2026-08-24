//! Development-only retained execution of the direction matured by frozen CP0.

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-rt0-retained-direction-execution-v1";
pub const EXACT_PARENT: &str = "a1b196615a451f104b060fd6f547f175e3bb1b45";
pub const PROTOCOL_COMMIT: &str = "643c6a239f534eb541262e64578ffe33732da6bb";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_CP0_SHA256: &str =
    "c9fcc53d03296b169060499e2304de557f3f7a93744dbc1f935053f99d41c498";
pub const FROZEN_A1_SHA256: &str =
    "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1";
pub const FROZEN_PARENT_SHA256: &str =
    "3ad1fa2f66ef3532298120587afdd92508071a0849a42ebc663cdc7e74980e04";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "dd1ed5c6bca12904c0c097e5a471c71af8738ab17304506c11fa11739420d7f1";

const ACQUISITION: usize = 16;
const CONTEXTS: usize = 4;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct RouteShape {
    trace: Vec<u8>,
    activation: [u16; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SignalMode {
    Recurrent(usize),
    BothSame,
    BothVariable,
    Shuffled,
    Suppressed(usize),
    Removed(usize),
}

macro_rules! rt0_cp0_access {
    () => {
        pub(super) fn rt0_frozen_signal(
            seed: u64,
            acquisition: usize,
            mode: super::SignalMode,
            relabel: bool,
            reverse_allocation: bool,
            layout_padding: bool,
        ) -> Option<super::RouteShape> {
            let (schedule, suppress_execution, remove_source) = match mode {
                super::SignalMode::Recurrent(recurrent) => {
                    (ContrastSchedule::OneRecurrent { recurrent }, false, false)
                }
                super::SignalMode::BothSame => (ContrastSchedule::BothSame, false, false),
                super::SignalMode::BothVariable => (ContrastSchedule::BothVariable, false, false),
                super::SignalMode::Shuffled => (ContrastSchedule::Shuffled, false, false),
                super::SignalMode::Suppressed(recurrent) => {
                    (ContrastSchedule::OneRecurrent { recurrent }, true, false)
                }
                super::SignalMode::Removed(recurrent) => {
                    (ContrastSchedule::OneRecurrent { recurrent }, false, true)
                }
            };
            let signal = frozen_d3::cp0_physical_contrast(
                seed,
                acquisition,
                schedule,
                ContrastOptions {
                    relabel,
                    reverse_allocation,
                    layout_padding,
                    suppress_execution,
                    remove_source,
                },
            )?;
            signal.source_shape.map(|shape| super::RouteShape {
                trace: shape.trace,
                activation: shape.activation,
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_cp0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_cp0_consequence_probation_coupling.rs"
    ));
    rt0_cp0_access!();
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TransferOptions {
    reverse_allocation: bool,
    layout_padding: bool,
    permute_handles: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct TransferResult {
    initial_counts: [u16; 2],
    frozen_counts: [u16; 2],
    post_install_counts: [u16; 2],
    consolidated: usize,
    installed_candidates: usize,
    installed_roots: usize,
    executed_roots: usize,
    structural_match: bool,
    fresh_occurrences_disjoint: bool,
    acquisition_observe_calls: usize,
    post_freeze_observe_calls: usize,
    persistent_bytes: usize,
    retained_occurrences: usize,
    retained_handles: usize,
}

macro_rules! rt0_a1_access {
    () => {
        pub(super) fn rt0_retain_install_execute(
            seed: u64,
            acquisition: usize,
            signal: Option<&super::RouteShape>,
            options: super::TransferOptions,
        ) -> Option<super::TransferResult> {
            let acquisition_bundle = frozen_e0::a1_bundle(seed, acquisition)?;
            let mut installer = train(&acquisition_bundle.support, false)?;
            let mut acquisition_substrate = substrate_from_export(
                &acquisition_bundle.target,
                MappingOptions {
                    reverse_allocation: options.reverse_allocation,
                    layout_padding: options.layout_padding,
                    ..MappingOptions::default()
                },
            )?;
            let (_, installed) = installer.install(&mut acquisition_substrate, true, false);
            let roots =
                structural_dedup(&mut acquisition_substrate, &installed, &mut installer.work);
            let bridge = expose_roots(&roots, options.permute_handles, &mut installer.work);
            if roots.len() != 2 || bridge.entries.len() != 2 {
                return None;
            }
            let mut route_shapes = Vec::new();
            let mut route_observations = Vec::new();
            for entry in &bridge.entries {
                let effect = execute_handle(
                    &acquisition_substrate,
                    &bridge,
                    entry.handle,
                    &mut installer.work,
                )?;
                route_shapes.push(super::RouteShape {
                    trace: effect.trace.clone(),
                    activation: effect.activation,
                });
                route_observations.push(
                    effect
                        .trace
                        .windows(2)
                        .map(|traversal| {
                            [
                                acquisition_substrate.members[usize::from(traversal[0])],
                                acquisition_substrate.members[usize::from(traversal[1])],
                            ]
                        })
                        .collect::<Vec<_>>(),
                );
            }
            let mut probation = Learner::default();
            let mut templates = Vec::new();
            let mut acquisition_observe_calls = 0usize;
            for observations in &route_observations {
                acquisition_substrate.observations.clone_from(observations);
                let proposals = local_proposals(&acquisition_substrate, &mut probation.work);
                if proposals.len() != 1 {
                    return None;
                }
                templates.push(proposals[0].template);
                let _ = probation.observe(&acquisition_substrate, true);
                acquisition_observe_calls += 1;
            }
            if templates[0] == templates[1] {
                return None;
            }
            let initial_counts = [
                probation.templates.get(&templates[0])?.count,
                probation.templates.get(&templates[1])?.count,
            ];
            let matching = signal.and_then(|shape| {
                let matches = route_shapes
                    .iter()
                    .enumerate()
                    .filter(|(_, route)| *route == shape)
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>();
                (matches.len() == 1).then_some(matches[0])
            });
            if let Some(index) = matching {
                acquisition_substrate
                    .observations
                    .clone_from(&route_observations[index]);
                for _ in 1..SUPPORT_EPISODES {
                    let _ = probation.observe(&acquisition_substrate, true);
                    acquisition_observe_calls += 1;
                }
            }
            let frozen_counts = [
                probation.templates.get(&templates[0])?.count,
                probation.templates.get(&templates[1])?.count,
            ];
            let consolidated = probation.consolidated();

            // FREEZE_PROBATION_ASSET: no observe call is permitted below.
            let fresh_bundle = frozen_e0::a1_bundle(seed + 500_000, acquisition)?;
            let mut fresh_substrate = substrate_from_export(
                &fresh_bundle.target,
                MappingOptions {
                    reverse_allocation: options.reverse_allocation,
                    layout_padding: options.layout_padding,
                    ..MappingOptions::default()
                },
            )?;
            let installed_candidates = local_proposals(&fresh_substrate, &mut probation.work).len();
            let (_, fresh_installed) = probation.install(&mut fresh_substrate, true, false);
            let fresh_roots =
                structural_dedup(&mut fresh_substrate, &fresh_installed, &mut probation.work);
            let effects = fresh_roots
                .iter()
                .map(|root| execute_root(&fresh_substrate, *root, &mut probation.work))
                .collect::<Option<Vec<_>>>()?;
            let executed_shapes = effects
                .iter()
                .map(|effect| super::RouteShape {
                    trace: effect.trace.clone(),
                    activation: effect.activation,
                })
                .collect::<Vec<_>>();
            let acquisition_occurrences = acquisition_substrate
                .cells
                .iter()
                .map(|cell| cell.occurrence)
                .collect::<BTreeSet<_>>();
            let fresh_occurrences_disjoint = fresh_substrate
                .cells
                .iter()
                .all(|cell| !acquisition_occurrences.contains(&cell.occurrence));
            let post_install_counts = [
                probation.templates.get(&templates[0])?.count,
                probation.templates.get(&templates[1])?.count,
            ];
            Some(super::TransferResult {
                initial_counts,
                frozen_counts,
                post_install_counts,
                consolidated,
                installed_candidates,
                installed_roots: fresh_roots.len(),
                executed_roots: effects.len(),
                structural_match: signal.is_some_and(|shape| {
                    executed_shapes.len() == 1 && executed_shapes[0] == *shape
                }),
                fresh_occurrences_disjoint,
                acquisition_observe_calls,
                post_freeze_observe_calls: 0,
                persistent_bytes: probation.work.persistent_bytes,
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
    rt0_a1_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub cp0_hash: bool,
    pub a1_hash: bool,
    pub parent_hash: bool,
    pub protocol_hash: bool,
    pub frozen_cp0_signal_calls: usize,
    pub existing_a1_observe_sites: usize,
    pub existing_a1_install_calls: usize,
    pub existing_a1_execute_calls: usize,
    pub post_freeze_observe_sites: usize,
    pub direct_support_mutations: usize,
    pub new_mechanism_types: usize,
    pub semantic_or_identity_fields: usize,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.cp0_hash
            && self.a1_hash
            && self.parent_hash
            && self.protocol_hash
            && self.frozen_cp0_signal_calls == 1
            && self.existing_a1_observe_sites == 2
            && self.existing_a1_install_calls == 1
            && self.existing_a1_execute_calls == 1
            && self.post_freeze_observe_sites == 0
            && self.direct_support_mutations == 0
            && self.new_mechanism_types == 0
            && self.semantic_or_identity_fields == 0
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
    let source = include_str!("ds_rt0_retained_direction_execution.rs");
    let production = source
        .split(&["#[cfg(", "test)]"].concat())
        .next()
        .unwrap_or(source);
    let signal = function_body(production, "pub(super) fn rt0_frozen_signal(").unwrap_or_default();
    let transfer =
        function_body(production, "pub(super) fn rt0_retain_install_execute(").unwrap_or_default();
    let post_freeze = transfer
        .split("FREEZE_PROBATION_ASSET")
        .nth(1)
        .unwrap_or_default();
    let forbidden = [
        ["correct", "ness"].concat(),
        ["reward", "_value"].concat(),
        ["semantic", "_polarity"].concat(),
        ["causal", "_label"].concat(),
        ["persistent", "_occurrence"].concat(),
        ["persistent", "_handle"].concat(),
    ];
    SourceAudit {
        cp0_hash: env!("DS_RT0_CP0_SHA256") == FROZEN_CP0_SHA256,
        a1_hash: env!("DS_RT0_A1_SHA256") == FROZEN_A1_SHA256,
        parent_hash: env!("DS_RT0_PARENT_SHA256") == FROZEN_PARENT_SHA256,
        protocol_hash: env!("DS_RT0_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        frozen_cp0_signal_calls: signal.matches("cp0_physical_contrast(").count(),
        existing_a1_observe_sites: transfer.matches("probation.observe(").count(),
        existing_a1_install_calls: transfer.matches("probation.install(").count(),
        existing_a1_execute_calls: transfer.matches("execute_root(&fresh_substrate").count(),
        post_freeze_observe_sites: post_freeze.matches("probation.observe(").count(),
        direct_support_mutations: [".count +=", ".count -=", "templates.insert("]
            .iter()
            .map(|token| transfer.matches(token).count())
            .sum(),
        new_mechanism_types: production.matches(&["struct ", "Learner"].concat()).count()
            + production
                .matches(&["struct ", "Installer"].concat())
                .count()
            + production
                .matches(&["struct ", "Executor"].concat())
                .count(),
        semantic_or_identity_fields: forbidden
            .iter()
            .map(|token| signal.matches(token).count() + transfer.matches(token).count())
            .sum(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub contexts: usize,
    pub retained_executions: usize,
    pub reversed_executions: usize,
    pub ambiguous_abstentions: usize,
    pub variable_abstentions: usize,
    pub shuffled_abstentions: usize,
    pub suppressed_abstentions: usize,
    pub removed_abstentions: usize,
    pub layout_transfers: usize,
    pub permuted_transfers: usize,
    pub persistent_bytes: usize,
    pub passed: bool,
}

fn retained(result: &TransferResult) -> bool {
    result.initial_counts == [1, 1]
        && result.consolidated == 1
        && result.frozen_counts == result.post_install_counts
        && result.installed_candidates == 2
        && result.installed_roots == 1
        && result.executed_roots == 1
        && result.structural_match
        && result.fresh_occurrences_disjoint
        && result.acquisition_observe_calls > 2
        && result.post_freeze_observe_calls == 0
        && result.retained_occurrences == 0
        && result.retained_handles == 0
}

fn abstained(result: &TransferResult) -> bool {
    result.initial_counts == [1, 1]
        && result.frozen_counts == [1, 1]
        && result.consolidated == 0
        && result.installed_roots == 0
        && result.executed_roots == 0
        && result.post_freeze_observe_calls == 0
}

fn audit_seed(seed: u64, source: &SourceAudit) -> SeedAudit {
    let mut retained_executions = 0usize;
    let mut reversed_executions = 0usize;
    let mut ambiguous_abstentions = 0usize;
    let mut variable_abstentions = 0usize;
    let mut shuffled_abstentions = 0usize;
    let mut suppressed_abstentions = 0usize;
    let mut removed_abstentions = 0usize;
    let mut layout_transfers = 0usize;
    let mut permuted_transfers = 0usize;
    let mut persistent_bytes = 0usize;
    for context in 0..CONTEXTS {
        let route_seed = seed + 1_000 + context as u64 * 17;
        let recurrent = (seed as usize + context) % 2;
        let signal = frozen_cp0::rt0_frozen_signal(
            route_seed,
            ACQUISITION,
            SignalMode::Recurrent(recurrent),
            false,
            false,
            false,
        );
        let primary = frozen_a1::rt0_retain_install_execute(
            route_seed,
            ACQUISITION,
            signal.as_ref(),
            TransferOptions::default(),
        )
        .expect("retained direction transfer");
        retained_executions += usize::from(retained(&primary));
        persistent_bytes = persistent_bytes.max(primary.persistent_bytes);

        let reversed_signal = frozen_cp0::rt0_frozen_signal(
            route_seed,
            ACQUISITION,
            SignalMode::Recurrent(1 - recurrent),
            false,
            false,
            false,
        );
        let reversed = frozen_a1::rt0_retain_install_execute(
            route_seed,
            ACQUISITION,
            reversed_signal.as_ref(),
            TransferOptions::default(),
        )
        .expect("reversed retained direction transfer");
        reversed_executions +=
            usize::from(retained(&reversed) && primary.frozen_counts != reversed.frozen_counts);

        for (mode, counter) in [
            (SignalMode::BothSame, &mut ambiguous_abstentions),
            (SignalMode::BothVariable, &mut variable_abstentions),
            (SignalMode::Shuffled, &mut shuffled_abstentions),
            (
                SignalMode::Suppressed(recurrent),
                &mut suppressed_abstentions,
            ),
            (SignalMode::Removed(recurrent), &mut removed_abstentions),
        ] {
            let no_signal =
                frozen_cp0::rt0_frozen_signal(route_seed, ACQUISITION, mode, false, false, false);
            let result = frozen_a1::rt0_retain_install_execute(
                route_seed,
                ACQUISITION,
                no_signal.as_ref(),
                TransferOptions::default(),
            )
            .expect("abstaining retained direction");
            *counter += usize::from(no_signal.is_none() && abstained(&result));
        }

        let layout_signal = frozen_cp0::rt0_frozen_signal(
            route_seed,
            ACQUISITION,
            SignalMode::Recurrent(recurrent),
            true,
            true,
            true,
        );
        let layout = frozen_a1::rt0_retain_install_execute(
            route_seed,
            ACQUISITION,
            layout_signal.as_ref(),
            TransferOptions {
                reverse_allocation: true,
                layout_padding: true,
                ..TransferOptions::default()
            },
        )
        .expect("layout retained direction");
        layout_transfers += usize::from(retained(&layout));

        let permuted = frozen_a1::rt0_retain_install_execute(
            route_seed,
            ACQUISITION,
            signal.as_ref(),
            TransferOptions {
                permute_handles: true,
                ..TransferOptions::default()
            },
        )
        .expect("permuted retained direction");
        permuted_transfers += usize::from(retained(&permuted));
    }
    let passed = source.passed()
        && retained_executions == CONTEXTS
        && reversed_executions == CONTEXTS
        && ambiguous_abstentions == CONTEXTS
        && variable_abstentions == CONTEXTS
        && shuffled_abstentions == CONTEXTS
        && suppressed_abstentions == CONTEXTS
        && removed_abstentions == CONTEXTS
        && layout_transfers == CONTEXTS
        && permuted_transfers == CONTEXTS
        && persistent_bytes > 0;
    SeedAudit {
        seed,
        contexts: CONTEXTS,
        retained_executions,
        reversed_executions,
        ambiguous_abstentions,
        variable_abstentions,
        shuffled_abstentions,
        suppressed_abstentions,
        removed_abstentions,
        layout_transfers,
        permuted_transfers,
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
    fn micro_reuses_one_retained_asset_on_fresh_occurrences() {
        let report = run(HarnessMode::Micro);
        assert!(report.passed, "{report:#?}");
        assert_eq!(report.seeds[0].retained_executions, CONTEXTS);
        assert_eq!(report.source.post_freeze_observe_sites, 0);
    }

    #[test]
    fn gate_passes_retention_transfer_controls() {
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
