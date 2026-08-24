//! Development-only physical D3 consequence coupling into existing A1 probation.

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-cp0-consequence-probation-coupling-v1";
pub const EXACT_PARENT: &str = "3d45292e684ab910ea7be0e337e27d22c513a375";
pub const PROTOCOL_COMMIT: &str = "743f37ef542f14a2413fd771b9d8e98f936223b2";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_D3_SHA256: &str =
    "a13f39c86b2c67d225530e7b17cdacd71f452a45be3b2c9942814c0748267f6d";
pub const FROZEN_A1_SHA256: &str =
    "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1";
pub const FROZEN_PARENT_SHA256: &str =
    "9c3ec5bfb643ddb3a8acc09eff9c5dd36043dd0b986075e43300677f4af2aa24";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "c0b8964e743a58c4a7bc566fe015fd96be4981d108c8231cfd0babb35672aed0";

const ACQUISITION: usize = 16;
const CONTEXTS: usize = 4;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct RouteShape {
    trace: Vec<u8>,
    activation: [u16; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ContrastSchedule {
    OneRecurrent { recurrent: usize },
    BothSame,
    BothVariable,
    Shuffled,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ContrastOptions {
    relabel: bool,
    reverse_allocation: bool,
    layout_padding: bool,
    suppress_execution: bool,
    remove_source: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ContrastSignal {
    source_shape: Option<RouteShape>,
    physical_direction_executed: bool,
    direction_arrows: usize,
    semantic_fields: usize,
}

macro_rules! cp0_d3_access {
    () => {
        pub(super) fn cp0_physical_contrast(
            seed: u64,
            acquisition: usize,
            schedule: super::ContrastSchedule,
            options: super::ContrastOptions,
        ) -> Option<super::ContrastSignal> {
            let routes = frozen_d2::d3_routes_with_layout(
                seed,
                acquisition,
                options.reverse_allocation,
                options.layout_padding,
            )?;
            let schedule = match schedule {
                super::ContrastSchedule::OneRecurrent { recurrent } => {
                    Schedule::OneRecurrent { recurrent }
                }
                super::ContrastSchedule::BothSame => Schedule::BothSame,
                super::ContrastSchedule::BothVariable => Schedule::BothVariable,
                super::ContrastSchedule::Shuffled => Schedule::Shuffled,
            };
            let mut learner = acquire_contrasts(
                seed + 40_000,
                &routes.affordances,
                schedule,
                options.relabel,
            );
            let mut alternatives = [Some(&routes.affordances[0]), Some(&routes.affordances[1])];
            let direction = learner.form_direction(alternatives);
            if options.remove_source {
                if let Some(index) = direction {
                    alternatives[index] = None;
                }
                let _ = learner.form_direction(alternatives);
            }
            let physical_direction_executed = !options.suppress_execution
                && !options.remove_source
                && learner.execute_direction();
            let source_shape = physical_direction_executed
                .then(|| {
                    let source = usize::from(learner.direction?.endpoints[0]);
                    let shape = &routes.affordances[source];
                    Some(super::RouteShape {
                        trace: shape.trace.clone(),
                        activation: shape.activation,
                    })
                })
                .flatten();
            Some(super::ContrastSignal {
                source_shape,
                physical_direction_executed,
                direction_arrows: usize::from(learner.direction.is_some()),
                semantic_fields: 0,
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_d3 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_d3_anonymous_consequence_contrast.rs"
    ));
    cp0_d3_access!();
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ProbationOptions {
    reverse_allocation: bool,
    layout_padding: bool,
    permute_handles: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ProbationResult {
    roots: usize,
    initial_counts: [u16; 2],
    final_counts: [u16; 2],
    matched_routes: usize,
    consolidated: usize,
    existing_observe_calls: usize,
    persistent_bytes: usize,
    retained_occurrences: usize,
    retained_handles: usize,
}

macro_rules! cp0_a1_access {
    () => {
        pub(super) fn cp0_apply_contrast_to_existing_probation(
            seed: u64,
            acquisition: usize,
            signal: Option<&super::RouteShape>,
            options: super::ProbationOptions,
        ) -> Option<super::ProbationResult> {
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
            let mut bridge = expose_roots(&roots, false, &mut installer.work);
            if roots.len() != 2 || bridge.entries.len() != 2 {
                return None;
            }
            if options.permute_handles {
                let first = bridge.entries[0].handle;
                bridge.entries[0].handle = bridge.entries[1].handle;
                bridge.entries[1].handle = first;
            }
            let mut route_shapes = Vec::new();
            let mut route_observations = Vec::new();
            for entry in &bridge.entries {
                let effect =
                    execute_handle(&substrate, &bridge, entry.handle, &mut installer.work)?;
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
                                substrate.members[usize::from(traversal[0])],
                                substrate.members[usize::from(traversal[1])],
                            ]
                        })
                        .collect::<Vec<_>>(),
                );
            }
            let mut probation = Learner::default();
            let mut templates = Vec::new();
            let mut existing_observe_calls = 0usize;
            for observations in &route_observations {
                substrate.observations.clone_from(observations);
                let proposals = local_proposals(&substrate, &mut probation.work);
                if proposals.len() != 1 {
                    return None;
                }
                templates.push(proposals[0].template);
                let _ = probation.observe(&substrate, true);
                existing_observe_calls += 1;
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
                substrate
                    .observations
                    .clone_from(&route_observations[index]);
                for _ in 1..SUPPORT_EPISODES {
                    let _ = probation.observe(&substrate, true);
                    existing_observe_calls += 1;
                }
            }
            let final_counts = [
                probation.templates.get(&templates[0])?.count,
                probation.templates.get(&templates[1])?.count,
            ];
            Some(super::ProbationResult {
                roots: roots.len(),
                initial_counts,
                final_counts,
                matched_routes: usize::from(matching.is_some()),
                consolidated: probation.consolidated(),
                existing_observe_calls,
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
    cp0_a1_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub d3_hash: bool,
    pub a1_hash: bool,
    pub parent_hash: bool,
    pub protocol_hash: bool,
    pub existing_d3_form_calls: usize,
    pub existing_d3_execute_calls: usize,
    pub existing_a1_observe_sites: usize,
    pub structural_match_sites: usize,
    pub direct_support_mutations: usize,
    pub new_learner_types: usize,
    pub semantic_or_causal_fields: usize,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.d3_hash
            && self.a1_hash
            && self.parent_hash
            && self.protocol_hash
            && self.existing_d3_form_calls == 2
            && self.existing_d3_execute_calls == 1
            && self.existing_a1_observe_sites == 2
            && self.structural_match_sites == 1
            && self.direct_support_mutations == 0
            && self.new_learner_types == 0
            && self.semantic_or_causal_fields == 0
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
    let source = include_str!("ds_cp0_consequence_probation_coupling.rs");
    let production = source
        .split(&["#[cfg(", "test)]"].concat())
        .next()
        .unwrap_or(source);
    let contrast =
        function_body(production, "pub(super) fn cp0_physical_contrast(").unwrap_or_default();
    let probation = function_body(
        production,
        "pub(super) fn cp0_apply_contrast_to_existing_probation(",
    )
    .unwrap_or_default();
    let direct_mutations = [".count +=", ".count -=", "templates.insert("];
    let forbidden = [
        ["correct", "ness"].concat(),
        ["reward", "_value"].concat(),
        ["semantic", "_polarity"].concat(),
        ["evaluator", "_direction"].concat(),
        ["cause", "_label"].concat(),
        ["source", "_role"].concat(),
        ["target", "_role"].concat(),
    ];
    SourceAudit {
        d3_hash: env!("DS_CP0_D3_SHA256") == FROZEN_D3_SHA256,
        a1_hash: env!("DS_CP0_A1_SHA256") == FROZEN_A1_SHA256,
        parent_hash: env!("DS_CP0_PARENT_SHA256") == FROZEN_PARENT_SHA256,
        protocol_hash: env!("DS_CP0_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        existing_d3_form_calls: contrast.matches("learner.form_direction(").count(),
        existing_d3_execute_calls: contrast.matches("learner.execute_direction()").count(),
        existing_a1_observe_sites: probation.matches("probation.observe(&substrate").count(),
        structural_match_sites: probation
            .matches(".filter(|(_, route)| *route == shape)")
            .count(),
        direct_support_mutations: direct_mutations
            .iter()
            .map(|token| probation.matches(token).count())
            .sum(),
        new_learner_types: production.matches(&["struct ", "Learner"].concat()).count(),
        semantic_or_causal_fields: forbidden
            .iter()
            .map(|token| contrast.matches(token).count() + probation.matches(token).count())
            .sum(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub contexts: usize,
    pub differentiated: usize,
    pub reverse_differentiated: usize,
    pub ambiguous_abstentions: usize,
    pub variable_abstentions: usize,
    pub shuffled_abstentions: usize,
    pub suppressed_abstentions: usize,
    pub removed_abstentions: usize,
    pub fresh_transfers: usize,
    pub layout_transfers: usize,
    pub permuted_transfers: usize,
    pub persistent_bytes: usize,
    pub passed: bool,
}

fn differentiated(result: &ProbationResult) -> bool {
    result.roots == 2
        && result.initial_counts == [1, 1]
        && result.matched_routes == 1
        && result.consolidated == 1
        && ((result.final_counts[0] > result.initial_counts[0]
            && result.final_counts[1] == result.initial_counts[1])
            || (result.final_counts[1] > result.initial_counts[1]
                && result.final_counts[0] == result.initial_counts[0]))
        && result.retained_occurrences == 0
        && result.retained_handles == 0
}

fn abstained(result: &ProbationResult) -> bool {
    result.initial_counts == [1, 1]
        && result.final_counts == [1, 1]
        && result.matched_routes == 0
        && result.consolidated == 0
}

fn audit_seed(seed: u64, source: &SourceAudit) -> SeedAudit {
    let mut differentiated_count = 0usize;
    let mut reverse_differentiated = 0usize;
    let mut ambiguous_abstentions = 0usize;
    let mut variable_abstentions = 0usize;
    let mut shuffled_abstentions = 0usize;
    let mut suppressed_abstentions = 0usize;
    let mut removed_abstentions = 0usize;
    let mut fresh_transfers = 0usize;
    let mut layout_transfers = 0usize;
    let mut permuted_transfers = 0usize;
    let mut persistent_bytes = 0usize;
    for context in 0..CONTEXTS {
        let route_seed = seed + 1_000 + context as u64 * 17;
        let recurrent = (seed as usize + context) % 2;
        let signal = frozen_d3::cp0_physical_contrast(
            route_seed,
            ACQUISITION,
            ContrastSchedule::OneRecurrent { recurrent },
            ContrastOptions::default(),
        )
        .expect("physical D3 contrast");
        let primary = frozen_a1::cp0_apply_contrast_to_existing_probation(
            route_seed,
            ACQUISITION,
            signal.source_shape.as_ref(),
            ProbationOptions::default(),
        )
        .expect("existing A1 probation");
        differentiated_count += usize::from(
            signal.physical_direction_executed
                && signal.direction_arrows == 1
                && signal.semantic_fields == 0
                && differentiated(&primary),
        );
        persistent_bytes = persistent_bytes.max(primary.persistent_bytes);

        let reverse = frozen_d3::cp0_physical_contrast(
            route_seed,
            ACQUISITION,
            ContrastSchedule::OneRecurrent {
                recurrent: 1 - recurrent,
            },
            ContrastOptions::default(),
        )
        .expect("reversed physical D3 contrast");
        let reversed = frozen_a1::cp0_apply_contrast_to_existing_probation(
            route_seed,
            ACQUISITION,
            reverse.source_shape.as_ref(),
            ProbationOptions::default(),
        )
        .expect("reversed existing probation");
        reverse_differentiated +=
            usize::from(differentiated(&reversed) && primary.final_counts != reversed.final_counts);

        for (schedule, counter) in [
            (ContrastSchedule::BothSame, &mut ambiguous_abstentions),
            (ContrastSchedule::BothVariable, &mut variable_abstentions),
            (ContrastSchedule::Shuffled, &mut shuffled_abstentions),
        ] {
            let no_signal = frozen_d3::cp0_physical_contrast(
                route_seed,
                ACQUISITION,
                schedule,
                ContrastOptions::default(),
            )
            .expect("ambiguous D3 history");
            let result = frozen_a1::cp0_apply_contrast_to_existing_probation(
                route_seed,
                ACQUISITION,
                no_signal.source_shape.as_ref(),
                ProbationOptions::default(),
            )
            .expect("abstaining probation");
            *counter += usize::from(no_signal.source_shape.is_none() && abstained(&result));
        }

        let suppressed = frozen_d3::cp0_physical_contrast(
            route_seed,
            ACQUISITION,
            ContrastSchedule::OneRecurrent { recurrent },
            ContrastOptions {
                suppress_execution: true,
                ..ContrastOptions::default()
            },
        )
        .expect("suppressed physical direction");
        let suppressed_result = frozen_a1::cp0_apply_contrast_to_existing_probation(
            route_seed,
            ACQUISITION,
            suppressed.source_shape.as_ref(),
            ProbationOptions::default(),
        )
        .expect("suppressed probation");
        suppressed_abstentions += usize::from(abstained(&suppressed_result));

        let removed = frozen_d3::cp0_physical_contrast(
            route_seed,
            ACQUISITION,
            ContrastSchedule::OneRecurrent { recurrent },
            ContrastOptions {
                remove_source: true,
                ..ContrastOptions::default()
            },
        )
        .expect("removed physical direction");
        let removed_result = frozen_a1::cp0_apply_contrast_to_existing_probation(
            route_seed,
            ACQUISITION,
            removed.source_shape.as_ref(),
            ProbationOptions::default(),
        )
        .expect("removed probation");
        removed_abstentions += usize::from(abstained(&removed_result));

        let fresh = frozen_d3::cp0_physical_contrast(
            route_seed + 100_000,
            ACQUISITION,
            ContrastSchedule::OneRecurrent { recurrent },
            ContrastOptions {
                relabel: true,
                ..ContrastOptions::default()
            },
        )
        .expect("fresh contrast");
        let fresh_result = frozen_a1::cp0_apply_contrast_to_existing_probation(
            route_seed + 100_000,
            ACQUISITION,
            fresh.source_shape.as_ref(),
            ProbationOptions::default(),
        )
        .expect("fresh probation");
        fresh_transfers += usize::from(differentiated(&fresh_result));

        let layout = frozen_d3::cp0_physical_contrast(
            route_seed,
            ACQUISITION,
            ContrastSchedule::OneRecurrent { recurrent },
            ContrastOptions {
                reverse_allocation: true,
                layout_padding: true,
                ..ContrastOptions::default()
            },
        )
        .expect("layout contrast");
        let layout_result = frozen_a1::cp0_apply_contrast_to_existing_probation(
            route_seed,
            ACQUISITION,
            layout.source_shape.as_ref(),
            ProbationOptions {
                reverse_allocation: true,
                layout_padding: true,
                ..ProbationOptions::default()
            },
        )
        .expect("layout probation");
        layout_transfers += usize::from(differentiated(&layout_result));

        let permuted_result = frozen_a1::cp0_apply_contrast_to_existing_probation(
            route_seed,
            ACQUISITION,
            signal.source_shape.as_ref(),
            ProbationOptions {
                permute_handles: true,
                ..ProbationOptions::default()
            },
        )
        .expect("permuted probation");
        permuted_transfers += usize::from(differentiated(&permuted_result));
    }
    let passed = source.passed()
        && differentiated_count == CONTEXTS
        && reverse_differentiated == CONTEXTS
        && ambiguous_abstentions == CONTEXTS
        && variable_abstentions == CONTEXTS
        && shuffled_abstentions == CONTEXTS
        && suppressed_abstentions == CONTEXTS
        && removed_abstentions == CONTEXTS
        && fresh_transfers == CONTEXTS
        && layout_transfers == CONTEXTS
        && permuted_transfers == CONTEXTS
        && persistent_bytes > 0;
    SeedAudit {
        seed,
        contexts: CONTEXTS,
        differentiated: differentiated_count,
        reverse_differentiated,
        ambiguous_abstentions,
        variable_abstentions,
        shuffled_abstentions,
        suppressed_abstentions,
        removed_abstentions,
        fresh_transfers,
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
    fn micro_differentiates_existing_probation_without_semantics() {
        let report = run(HarnessMode::Micro);
        assert!(report.passed, "{report:#?}");
        assert_eq!(report.seeds[0].differentiated, CONTEXTS);
        assert_eq!(report.source.direct_support_mutations, 0);
        assert_eq!(report.source.semantic_or_causal_fields, 0);
    }

    #[test]
    fn gate_passes_all_consequence_probation_controls() {
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
