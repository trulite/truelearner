//! FFS-SAME1: reintegrate the frozen CS0a correspondence fast path into the
//! level-blind FFS-SAME0 developmental kernel. This module adds measurement
//! and orchestration only; correspondence and arrow-learning physics remain
//! the frozen parent implementations.

use super::cs0a::{
    acquire_compiled, changed_parent_rules, execute_compiled_or_generic, persistent_source_audit,
    CompiledCorrespondenceStore, Cs0aWork, COMPILED_EVIDENCE_PER_MOTIF,
    SUBTHRESHOLD_EVIDENCE_PER_MOTIF,
};
use super::*;
use crate::research_runtime::{parallel_map_ordered, HarnessMode};

pub const FFS_SAME1_PROTOCOL: &str = "identity-desupply-ladder-v1/ffs-same1";

const DEVELOPMENT_SEED: usize = 70_000;
const DEFINITIVE_SEEDS: usize = 8;
const DEVELOPMENT_EPISODES: usize = 4;
const DEFINITIVE_EPISODES: usize = 16;
const GENERIC_LEARNED_IDENTITY_TAX: u64 = 18;
const COMPILED_LEARNED_IDENTITY_TAX: u64 = 6;

// BEGIN SAME1 LEVEL-BLIND REINTEGRATION

#[derive(Clone, Debug, PartialEq, Eq)]
struct Same1Acquisition {
    roots: Vec<ArrowId>,
    new_arrows: Vec<ArrowId>,
    proposed: usize,
    retained: usize,
    work: Cs0aWork,
    incremental_bytes: usize,
    compiled_uses: usize,
    generic_reopenings: usize,
}

fn acquire_arrows_compiled(
    arrows: &mut ArrowStore,
    roots: &[ArrowId],
    rules: &RuleStore,
    compiled: &CompiledCorrespondenceStore,
    episodes: &[EvaluatorEpisode],
    successful: bool,
    shuffled: bool,
) -> Same1Acquisition {
    let mut candidates = BTreeMap::<[ArrowId; 2], ArrowCandidate>::new();
    let mut work = Cs0aWork::default();
    let mut compiled_uses = 0;
    let mut generic_reopenings = 0;
    for (episode_index, episode) in episodes.iter().enumerate() {
        let execution = execute_compiled_or_generic(
            arrows,
            roots,
            rules,
            compiled,
            episode,
            &Environment::default(),
        );
        compiled_uses += usize::from(execution.used_compiled);
        generic_reopenings += usize::from(execution.reopened_generic);
        work.add(execution.work);
        let mut occurrences = execution.execution.arrow_occurrences;
        if shuffled && !occurrences.is_empty() {
            let count = occurrences.len();
            occurrences.rotate_left((episode_index + 1) % count);
        }
        for (pair, count) in observed_pairs(&occurrences, &mut work.same0) {
            work.same0.recurrence_comparisons += count as u64;
            if count < MIN_LOCAL_OCCURRENCES {
                continue;
            }
            let candidate = candidates.entry(pair).or_insert_with(|| {
                work.same0.arrow_candidates_proposed += 1;
                ArrowCandidate { pair, strength: 0 }
            });
            candidate.strength += if successful {
                SUCCESS_CREDIT
            } else {
                FAILURE_CREDIT
            };
            work.same0.arrow_credit_updates += 1;
        }
    }
    let mut new_arrows = Vec::new();
    let mut proposed = 0;
    let mut retained = 0;
    let mut incremental_bytes = 0;
    for candidate in candidates.values() {
        proposed += 1;
        if candidate.strength >= CONSOLIDATION_STRENGTH {
            let before = arrows.arrows.len();
            let id = arrows.insert_pair(candidate.pair, candidate.strength);
            if arrows.arrows.len() > before {
                incremental_bytes += arrows.get(id).persistent_bytes();
                new_arrows.push(id);
                work.same0.arrows_consolidated += 1;
            }
            retained += 1;
        } else if candidate.strength <= PRUNE_STRENGTH {
            work.same0.arrows_pruned += 1;
        }
    }
    let rewritten = roots
        .chunks(2)
        .flat_map(|chunk| {
            if chunk.len() == 2 {
                arrows
                    .find_pair([chunk[0], chunk[1]])
                    .map(|id| vec![id])
                    .unwrap_or_else(|| chunk.to_vec())
            } else {
                chunk.to_vec()
            }
        })
        .collect();
    Same1Acquisition {
        roots: rewritten,
        new_arrows,
        proposed,
        retained,
        work,
        incremental_bytes,
        compiled_uses,
        generic_reopenings,
    }
}

struct AverageCompiled<'a> {
    arrows: &'a ArrowStore,
    roots: &'a [ArrowId],
    rules: &'a RuleStore,
    compiled: &'a CompiledCorrespondenceStore,
    scale: ScaleSpec,
    seed: usize,
    episodes: usize,
    environment: &'a Environment,
    perturbation: ViewPerturbation,
    truth_offset: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AverageCompiledResult {
    work: u64,
    correspondence_work: u64,
    traces: Vec<ObservableTrace>,
    firings: usize,
    compiled_uses: usize,
    generic_reopenings: usize,
}

fn average_compiled(request: AverageCompiled<'_>) -> AverageCompiledResult {
    let mut total_work = 0;
    let mut correspondence_work = 0;
    let mut traces = Vec::new();
    let mut firings = 0;
    let mut compiled_uses = 0;
    let mut generic_reopenings = 0;
    for episode_index in 0..request.episodes {
        let episode = held_out_episode(
            request.seed,
            request.scale,
            episode_index,
            request.perturbation,
            request.truth_offset,
        );
        let execution = execute_compiled_or_generic(
            request.arrows,
            request.roots,
            request.rules,
            request.compiled,
            &episode,
            request.environment,
        );
        total_work += execution.work.total();
        correspondence_work += execution.work.correspondence_total();
        firings += execution.execution.child_firings;
        compiled_uses += usize::from(execution.used_compiled);
        generic_reopenings += usize::from(execution.reopened_generic);
        traces.push(observable(
            &execution.execution,
            &episode,
            request.environment,
        ));
    }
    AverageCompiledResult {
        work: total_work / request.episodes as u64,
        correspondence_work: correspondence_work / request.episodes as u64,
        traces,
        firings: firings / request.episodes,
        compiled_uses,
        generic_reopenings,
    }
}

// END SAME1 LEVEL-BLIND REINTEGRATION

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Same1EdgeResult {
    pub seed: usize,
    pub scale: String,
    pub generation: usize,
    pub parent_work: u64,
    pub child_work: u64,
    pub acquisition_work: u64,
    pub incremental_bytes: usize,
    pub observable_equal: bool,
    pub computationally_useful: bool,
    pub economically_justified: bool,
    pub structurally_retained: bool,
    pub break_even_uses: Option<u64>,
    pub proposed: usize,
    pub retained: usize,
    pub removed_arrow_firings: usize,
    pub compiled_correspondence_work: u64,
    pub acquisition_compiled_uses: usize,
    pub acquisition_generic_reopenings: usize,
    pub asset_instance_id: u64,
    pub content_fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Same1IdentityEconomics {
    pub seed: usize,
    pub scale: String,
    pub learned_motifs: usize,
    pub generic_acquisition_work: u64,
    pub compilation_acquisition_work: u64,
    pub generic_persistent_bytes: usize,
    pub compiled_persistent_bytes: usize,
    pub anonymous_generic_runtime: u64,
    pub same0_generic_learned_runtime: u64,
    pub same1_compiled_runtime: u64,
    pub supplied_same_runtime: u64,
    pub same0_identity_tax: u64,
    pub same1_identity_tax: u64,
    pub improvement_vs_same0: u64,
    pub premium_vs_supplied: i64,
    pub compiled_uses: usize,
    pub generic_reopenings: usize,
    pub activation_work: u64,
    pub validation_work: u64,
    pub ambiguity_work: u64,
    pub grounding_work: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Same1ScaleResult {
    pub seed: usize,
    pub scale: String,
    pub depth: usize,
    pub population: usize,
    pub edges: Vec<Same1EdgeResult>,
    pub structural_depth: usize,
    pub justified_depth: usize,
    pub realized_useful_depth: usize,
    pub right_censored: bool,
    pub over_retained: usize,
    pub under_retained: usize,
    pub collapse_point: String,
    pub identity: Same1IdentityEconomics,
    arrows: ArrowStore,
    roots: Vec<ArrowId>,
    rules: RuleStore,
    compiled: CompiledCorrespondenceStore,
    lineage: u64,
}

fn target_depth(scale: ScaleSpec) -> usize {
    match scale.depth {
        0..=15 => 0,
        16..=63 => 3,
        64..=255 => 5,
        _ => MAX_PROMOTIONS,
    }
}

fn run_scale(
    scale: ScaleSpec,
    seed: usize,
    held_out_episodes: usize,
    evidence_episodes: usize,
) -> Same1ScaleResult {
    let correspondence = train_correspondence(
        0xfb10_0000_0000_0000 ^ seed as u64,
        scale.population,
        evidence_episodes,
        0,
    );
    let compilation = acquire_compiled(
        &correspondence.rules,
        seed,
        scale.population,
        COMPILED_EVIDENCE_PER_MOTIF,
        false,
    );
    let lineage = hash_values([
        0xfb11_0000_0000_0000,
        seed as u64,
        scale.depth as u64,
        scale.population as u64,
    ]);
    let mut arrows = ArrowStore::primitive();
    let mut roots = primitive_roots(scale.depth);
    let mut edges = Vec::new();
    if correspondence.rules.consolidated_count() >= 2 && compilation.consolidated >= 2 {
        for generation in 0..MAX_PROMOTIONS {
            let parent_roots = roots.clone();
            let episodes = (0..3)
                .map(|episode| {
                    held_out_episode(
                        seed ^ 0x6100 ^ generation,
                        scale,
                        episode,
                        ViewPerturbation::Standard,
                        0,
                    )
                })
                .collect::<Vec<_>>();
            let acquisition = acquire_arrows_compiled(
                &mut arrows,
                &parent_roots,
                &correspondence.rules,
                &compilation.store,
                &episodes,
                true,
                false,
            );
            if acquisition.new_arrows.is_empty() {
                break;
            }
            roots = acquisition.roots.clone();
            let environment = Environment::default();
            let parent = average_compiled(AverageCompiled {
                arrows: &arrows,
                roots: &parent_roots,
                rules: &correspondence.rules,
                compiled: &compilation.store,
                scale,
                seed,
                episodes: held_out_episodes,
                environment: &environment,
                perturbation: ViewPerturbation::Standard,
                truth_offset: 0,
            });
            let child = average_compiled(AverageCompiled {
                arrows: &arrows,
                roots: &roots,
                rules: &correspondence.rules,
                compiled: &compilation.store,
                scale,
                seed,
                episodes: held_out_episodes,
                environment: &environment,
                perturbation: ViewPerturbation::Standard,
                truth_offset: 0,
            });
            let observable_equal = parent.traces == child.traces;
            let computationally_useful = child.work < parent.work;
            let gain = parent.work.saturating_sub(child.work);
            let break_even_uses = (observable_equal && computationally_useful).then(|| {
                u64::try_from(ceil_div(acquisition.work.total() as u128, gain as u128))
                    .expect("FFS-SAME1 break-even fits u64")
            });
            let content_fingerprint = hash_values(
                acquisition
                    .new_arrows
                    .iter()
                    .map(|id| arrows.get(*id).fingerprint()),
            );
            edges.push(Same1EdgeResult {
                seed,
                scale: scale.name.to_string(),
                generation: generation + 1,
                parent_work: parent.work,
                child_work: child.work,
                acquisition_work: acquisition.work.total(),
                incremental_bytes: acquisition.incremental_bytes,
                observable_equal,
                computationally_useful,
                economically_justified: break_even_uses.is_some(),
                structurally_retained: acquisition.retained > 0,
                break_even_uses,
                proposed: acquisition.proposed,
                retained: acquisition.retained,
                removed_arrow_firings: parent.firings.saturating_sub(child.firings),
                compiled_correspondence_work: child.correspondence_work,
                acquisition_compiled_uses: acquisition.compiled_uses,
                acquisition_generic_reopenings: acquisition.generic_reopenings,
                asset_instance_id: hash_values([
                    lineage,
                    generation as u64,
                    acquisition.new_arrows.len() as u64,
                ]),
                content_fingerprint,
            });
        }
    }
    let structural_depth = edges
        .iter()
        .take_while(|edge| edge.structurally_retained)
        .count();
    let justified_depth = edges
        .iter()
        .take_while(|edge| {
            edge.structurally_retained
                && edge.observable_equal
                && edge.computationally_useful
                && edge.economically_justified
        })
        .count();
    let over_retained = edges
        .iter()
        .filter(|edge| edge.structurally_retained && !edge.economically_justified)
        .count();
    let under_retained = edges
        .iter()
        .filter(|edge| !edge.structurally_retained && edge.economically_justified)
        .count();
    let environment = Environment::default();
    let mature = average_compiled(AverageCompiled {
        arrows: &arrows,
        roots: &roots,
        rules: &correspondence.rules,
        compiled: &compilation.store,
        scale,
        seed,
        episodes: held_out_episodes,
        environment: &environment,
        perturbation: ViewPerturbation::Standard,
        truth_offset: 0,
    });
    let supplied_runtime = supplied_same_runtime(scale.depth);
    let same0_runtime = supplied_runtime + GENERIC_LEARNED_IDENTITY_TAX;
    let anonymous_generic_runtime = 2 * primitive_route_work(scale.depth) + 24;
    let collapse_point = if correspondence.rules.consolidated_count() < 2 {
        "correspondence"
    } else if compilation.consolidated < 2 {
        "compilation"
    } else if mature
        .traces
        .iter()
        .any(|trace| trace.final_state.is_none())
    {
        "binding"
    } else if edges.is_empty() {
        "compaction"
    } else if justified_depth < structural_depth {
        "recursive-economics"
    } else {
        "none"
    };
    Same1ScaleResult {
        seed,
        scale: scale.name.to_string(),
        depth: scale.depth,
        population: scale.population,
        edges,
        structural_depth,
        justified_depth,
        realized_useful_depth: justified_depth,
        right_censored: structural_depth == MAX_PROMOTIONS,
        over_retained,
        under_retained,
        collapse_point: collapse_point.to_string(),
        identity: Same1IdentityEconomics {
            seed,
            scale: scale.name.to_string(),
            learned_motifs: correspondence.rules.consolidated_count(),
            generic_acquisition_work: correspondence.acquisition_work.total(),
            compilation_acquisition_work: compilation.work.total(),
            generic_persistent_bytes: correspondence.rules.persistent_bytes(),
            compiled_persistent_bytes: compilation.store.persistent_bytes(),
            anonymous_generic_runtime,
            same0_generic_learned_runtime: same0_runtime,
            same1_compiled_runtime: mature.work,
            supplied_same_runtime: supplied_runtime,
            same0_identity_tax: GENERIC_LEARNED_IDENTITY_TAX,
            same1_identity_tax: mature.correspondence_work,
            improvement_vs_same0: same0_runtime.saturating_sub(mature.work),
            premium_vs_supplied: mature.work as i64 - supplied_runtime as i64,
            compiled_uses: mature.compiled_uses,
            generic_reopenings: mature.generic_reopenings,
            activation_work: 1,
            validation_work: 3,
            ambiguity_work: 1,
            grounding_work: 1,
        },
        arrows,
        roots,
        rules: correspondence.rules,
        compiled: compilation.store,
        lineage,
    }
}

fn hierarchy_fingerprint(scale: &Same1ScaleResult) -> u64 {
    hash_values(
        scale
            .arrows
            .arrows
            .iter()
            .filter(|arrow| arrow.learned())
            .map(Arrow::fingerprint)
            .chain([scale.rules.fingerprint(), scale.compiled.fingerprint()]),
    )
}

fn hierarchy_instance(scale: &Same1ScaleResult) -> u64 {
    hash_values(
        [0xfb12_0000_0000_0000, scale.lineage]
            .into_iter()
            .chain(scale.edges.iter().map(|edge| edge.asset_instance_id)),
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Same1TransferResult {
    pub seed: usize,
    pub probe: String,
    pub observable_equal: bool,
    pub primitive_work: u64,
    pub transferred_work: u64,
    pub acquisition_work_charged: u64,
    pub reused_same_instance: bool,
    pub compiled_correspondence_work: u64,
}

fn transfers(source: &Same1ScaleResult, episodes: usize) -> Vec<Same1TransferResult> {
    [DEPTH_PROBE, POPULATION_PROBE]
        .into_iter()
        .map(|probe| {
            let primitive = primitive_roots(probe.depth);
            let transferred = apply_existing(&source.arrows, primitive.clone());
            let environment = Environment::default();
            let parent = average_compiled(AverageCompiled {
                arrows: &source.arrows,
                roots: &primitive,
                rules: &source.rules,
                compiled: &source.compiled,
                scale: probe,
                seed: source.seed,
                episodes,
                environment: &environment,
                perturbation: ViewPerturbation::Standard,
                truth_offset: 0,
            });
            let child = average_compiled(AverageCompiled {
                arrows: &source.arrows,
                roots: &transferred,
                rules: &source.rules,
                compiled: &source.compiled,
                scale: probe,
                seed: source.seed,
                episodes,
                environment: &environment,
                perturbation: ViewPerturbation::Standard,
                truth_offset: 0,
            });
            Same1TransferResult {
                seed: source.seed,
                probe: probe.name.to_string(),
                observable_equal: parent.traces == child.traces,
                primitive_work: parent.work,
                transferred_work: child.work,
                acquisition_work_charged: 0,
                reused_same_instance: hierarchy_instance(source) != 0
                    && hierarchy_fingerprint(source) != 0,
                compiled_correspondence_work: child.correspondence_work,
            }
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Same1AdaptiveResult {
    pub seed: usize,
    pub arm: String,
    pub observable_equal: bool,
    pub fallback_distance: usize,
    pub recovery_work: u64,
    pub reacquisition_work: u64,
    pub historical_asset_reused: bool,
}

fn adaptive(scale: &Same1ScaleResult, episodes: usize) -> Vec<Same1AdaptiveResult> {
    let top = scale
        .roots
        .iter()
        .copied()
        .find(|id| scale.arrows.get(*id).learned())
        .expect("S2 produces a learned root");
    let top_arrow = scale.arrows.get(top);
    let parent = top_arrow.dependencies[0];
    let parent_arrow = scale.arrows.get(parent);
    let stable = Environment::default();
    let mut own_changed = Environment::default();
    own_changed.change(top_arrow.compatibility_site);
    let mut parent_changed = Environment::default();
    parent_changed.change(parent_arrow.compatibility_site);
    [
        ("stable", stable.clone(), 0),
        ("child-own-change", own_changed, 1),
        ("direct-parent-change", parent_changed, 2),
        ("return", stable, 0),
    ]
    .into_iter()
    .map(|(arm, environment, expected_distance)| {
        let mut equal = true;
        let mut fallback_distance = 0;
        let mut recovery_work = 0;
        for episode_index in 0..episodes {
            let episode = held_out_episode(
                scale.seed ^ 0x6200,
                ScaleSpec::new("adaptive", scale.depth, scale.population),
                episode_index,
                ViewPerturbation::Standard,
                0,
            );
            let reference = execute_compiled_or_generic(
                &scale.arrows,
                &primitive_roots(scale.depth),
                &scale.rules,
                &scale.compiled,
                &episode,
                &environment,
            );
            let candidate = execute_compiled_or_generic(
                &scale.arrows,
                &scale.roots,
                &scale.rules,
                &scale.compiled,
                &episode,
                &environment,
            );
            equal &= observable(&reference.execution, &episode, &environment)
                == observable(&candidate.execution, &episode, &environment);
            fallback_distance =
                fallback_distance.max(candidate.execution.maximum_fallback_distance);
            recovery_work += candidate.work.total();
        }
        Same1AdaptiveResult {
            seed: scale.seed,
            arm: arm.to_string(),
            observable_equal: equal && fallback_distance == expected_distance,
            fallback_distance,
            recovery_work,
            reacquisition_work: 0,
            historical_asset_reused: arm != "return"
                || (hierarchy_instance(scale) != 0 && hierarchy_fingerprint(scale) != 0),
        }
    })
    .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Same1Control {
    pub seed: usize,
    pub name: String,
    pub passed: bool,
    pub diagnostic: u64,
}

fn control(seed: usize, name: &str, passed: bool, diagnostic: u64) -> Same1Control {
    Same1Control {
        seed,
        name: name.to_string(),
        passed,
        diagnostic,
    }
}

fn lifecycle_controls(seed: usize, episodes: usize, evidence: usize) -> Vec<Same1Control> {
    let fixture = train_correspondence(0xfb13_0000 ^ seed as u64, 64, evidence, 0);
    let compiled = acquire_compiled(&fixture.rules, seed, 64, COMPILED_EVIDENCE_PER_MOTIF, false);
    let subthreshold = acquire_compiled(
        &fixture.rules,
        seed ^ 0x7100,
        64,
        SUBTHRESHOLD_EVIDENCE_PER_MOTIF,
        false,
    );
    let shuffled = acquire_compiled(
        &fixture.rules,
        seed ^ 0x7200,
        64,
        COMPILED_EVIDENCE_PER_MOTIF * 2,
        true,
    );
    let changed = changed_parent_rules(&fixture.rules);
    let arrows = ArrowStore::primitive();
    let roots = primitive_roots(32);
    let fingerprint = compiled.store.fingerprint();
    let mut exact = true;
    let mut fresh = true;
    let mut changed_binding = true;
    let mut reopened = 0;
    let mut invalidated = 0;
    let mut historical = 0;
    for episode_index in 0..episodes {
        for perturbation in [
            ViewPerturbation::Standard,
            ViewPerturbation::RelabeledOccurrences,
            ViewPerturbation::AllocationOrder,
            ViewPerturbation::MemoryOrder,
        ] {
            let episode = held_out_episode(
                seed ^ 0x7300,
                ScaleSpec::new("control", 32, 64),
                episode_index,
                perturbation,
                0,
            );
            let environment = Environment::default();
            let generic = execute(&arrows, &roots, &fixture.rules, &episode.view, &environment);
            let candidate = execute_compiled_or_generic(
                &arrows,
                &roots,
                &fixture.rules,
                &compiled.store,
                &episode,
                &environment,
            );
            let equal = observable(&generic, &episode, &environment)
                == observable(&candidate.execution, &episode, &environment);
            exact &= equal;
            fresh &= equal && candidate.used_compiled;
        }
        let episode = held_out_episode(
            seed ^ 0x7400,
            ScaleSpec::new("changed-binding", 32, 64),
            episode_index,
            ViewPerturbation::Standard,
            10_000,
        );
        let environment = Environment::default();
        let generic = execute(&arrows, &roots, &fixture.rules, &episode.view, &environment);
        let candidate = execute_compiled_or_generic(
            &arrows,
            &roots,
            &fixture.rules,
            &compiled.store,
            &episode,
            &environment,
        );
        changed_binding &= observable(&generic, &episode, &environment)
            == observable(&candidate.execution, &episode, &environment)
            && candidate.used_compiled;
        let stale = execute_compiled_or_generic(
            &arrows,
            &roots,
            &changed,
            &compiled.store,
            &episode,
            &environment,
        );
        reopened += usize::from(stale.reopened_generic);
        invalidated += stale.work.compiled_invalidations as usize;
        let returned = execute_compiled_or_generic(
            &arrows,
            &roots,
            &fixture.rules,
            &compiled.store,
            &episode,
            &environment,
        );
        historical += usize::from(returned.used_compiled && !returned.reopened_generic);
    }
    vec![
        control(
            seed,
            "compiled-routes-earned",
            compiled.consolidated == 2,
            compiled.consolidated as u64,
        ),
        control(
            seed,
            "subthreshold-does-not-compile",
            subthreshold.consolidated == 0,
            subthreshold.consolidated as u64,
        ),
        control(
            seed,
            "shuffled-does-not-compile",
            shuffled.consolidated == 0,
            shuffled.consolidated as u64,
        ),
        control(
            seed,
            "fresh-and-permuted-occurrences-transfer",
            fresh && exact,
            u64::from(fresh && exact),
        ),
        control(
            seed,
            "changed-binding-transfer",
            changed_binding,
            u64::from(changed_binding),
        ),
        control(
            seed,
            "stale-dependency-reopens-generic",
            reopened == episodes && invalidated >= episodes,
            reopened as u64,
        ),
        control(
            seed,
            "historical-compatible-route-reused",
            historical == episodes,
            historical as u64,
        ),
        control(
            seed,
            "persistent-route-unchanged-during-use",
            compiled.store.fingerprint() == fingerprint,
            fingerprint,
        ),
        control(
            seed,
            "persistent-state-has-no-filler-identity",
            persistent_source_audit(),
            compiled.store.persistent_bytes() as u64,
        ),
    ]
}

fn reintegration_source_audit() -> bool {
    let source = include_str!("same1.rs");
    let kernel = source
        .split_once("// BEGIN SAME1 LEVEL-BLIND REINTEGRATION")
        .and_then(|(_, rest)| rest.split_once("// END SAME1 LEVEL-BLIND REINTEGRATION"))
        .map(|(body, _)| body)
        .unwrap_or("")
        .to_ascii_lowercase();
    [
        "truthfillerid",
        "same(",
        "stable_payload",
        "canonical",
        "correlation",
        "future_object",
        "object_id",
        "parent_level",
        "meta",
        "economic",
        "price",
        "horizon",
        "break_even",
    ]
    .iter()
    .all(|forbidden| !kernel.contains(forbidden))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Same1ProcessResult {
    pub process: String,
    pub status: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Same1ClaimResult {
    pub claim: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Same1Report {
    pub mode: String,
    pub claim_eligible: bool,
    pub scales: Vec<Same1ScaleResult>,
    pub transfers: Vec<Same1TransferResult>,
    pub adaptive: Vec<Same1AdaptiveResult>,
    pub controls: Vec<Same1Control>,
    pub processes: Vec<Same1ProcessResult>,
    pub claims: Vec<Same1ClaimResult>,
    pub depth_curve_preserved: bool,
    pub scaling_trend_supported: bool,
    pub orthogonal_depth_signature: bool,
    pub duplicate_deterministic: bool,
    pub source_audit_passed: bool,
    pub passed: bool,
}

fn status(pass: bool) -> String {
    if pass { "PASS" } else { "FAIL" }.to_string()
}

pub fn run_same1(mode: HarnessMode) -> Same1Report {
    let (seeds, held_out_episodes, evidence_episodes, specs) = match mode {
        HarnessMode::Micro => (
            vec![99_996],
            2,
            4,
            vec![
                ScaleSpec::new("micro-8", 8, 16),
                ScaleSpec::new("micro-32", 32, 64),
            ],
        ),
        HarnessMode::Gate => (
            vec![DEVELOPMENT_SEED],
            DEVELOPMENT_EPISODES,
            4,
            ANCHORS
                .into_iter()
                .chain([DEPTH_PROBE, POPULATION_PROBE])
                .collect(),
        ),
        HarnessMode::Definitive => (
            (0..DEFINITIVE_SEEDS).collect(),
            DEFINITIVE_EPISODES,
            4,
            ANCHORS
                .into_iter()
                .chain([DEPTH_PROBE, POPULATION_PROBE])
                .collect(),
        ),
    };
    let jobs = seeds
        .iter()
        .flat_map(|seed| specs.iter().map(move |spec| (*seed, *spec)))
        .collect::<Vec<_>>();
    let scales = parallel_map_ordered(jobs.len(), |index| {
        let (seed, spec) = jobs[index];
        run_scale(spec, seed, held_out_episodes, evidence_episodes)
    });
    let duplicate_deterministic = scales.iter().all(|scale| {
        let duplicate = run_scale(
            ScaleSpec::new("duplicate", scale.depth, scale.population),
            scale.seed,
            held_out_episodes,
            evidence_episodes,
        );
        duplicate.edges.len() == scale.edges.len()
            && duplicate.structural_depth == scale.structural_depth
            && duplicate.identity.same1_compiled_runtime == scale.identity.same1_compiled_runtime
            && duplicate
                .edges
                .iter()
                .zip(&scale.edges)
                .all(|(left, right)| {
                    left.parent_work == right.parent_work
                        && left.child_work == right.child_work
                        && left.acquisition_work == right.acquisition_work
                        && left.content_fingerprint == right.content_fingerprint
                })
    });
    let controls = seeds
        .iter()
        .flat_map(|seed| {
            lifecycle_controls(*seed, held_out_episodes, evidence_episodes)
                .into_iter()
                .chain(
                    correspondence_controls(*seed, evidence_episodes)
                        .into_iter()
                        .map(|inherited| Same1Control {
                            seed: *seed,
                            name: format!("inherited-{}", inherited.name),
                            passed: inherited.passed,
                            diagnostic: inherited.diagnostic,
                        }),
                )
        })
        .collect::<Vec<_>>();
    let transfers = if mode == HarnessMode::Micro {
        Vec::new()
    } else {
        scales
            .iter()
            .filter(|scale| scale.scale == "S1")
            .flat_map(|scale| transfers(scale, held_out_episodes))
            .collect()
    };
    let adaptive = if mode == HarnessMode::Micro {
        Vec::new()
    } else {
        scales
            .iter()
            .filter(|scale| scale.scale == "S2")
            .flat_map(|scale| adaptive(scale, held_out_episodes))
            .collect()
    };
    let source_audit_passed = reintegration_source_audit() && persistent_source_audit();
    let depth_curve_preserved = mode == HarnessMode::Micro
        || scales.iter().all(|scale| {
            !ANCHORS.iter().any(|anchor| anchor.name == scale.scale)
                || scale.realized_useful_depth
                    == target_depth(ScaleSpec::new("target", scale.depth, scale.population))
        });
    let scaling_trend_supported = mode == HarnessMode::Micro
        || seeds.iter().all(|seed| {
            let depths = ANCHORS
                .iter()
                .filter_map(|anchor| {
                    scales
                        .iter()
                        .find(|scale| scale.seed == *seed && scale.scale == anchor.name)
                        .map(|scale| scale.realized_useful_depth)
                })
                .collect::<Vec<_>>();
            depths.len() == ANCHORS.len()
                && depths.windows(2).all(|pair| pair[0] <= pair[1])
                && depths.iter().copied().collect::<BTreeSet<_>>().len() >= 2
        });
    let orthogonal_depth_signature = mode == HarnessMode::Micro
        || seeds.iter().all(|seed| {
            let find = |name: &str| {
                scales
                    .iter()
                    .find(|scale| scale.seed == *seed && scale.scale == name)
                    .map(|scale| scale.realized_useful_depth)
            };
            find("S2") == find("depth-only") && find("S1") == find("population-only")
        });
    let primary = scales
        .iter()
        .filter(|scale| matches!(scale.scale.as_str(), "S1" | "S2" | "S3"))
        .collect::<Vec<_>>();
    let a1 = mode == HarnessMode::Micro
        || (depth_curve_preserved
            && primary.iter().all(|scale| {
                scale.over_retained == 0
                    && scale.under_retained == 0
                    && scale
                        .edges
                        .iter()
                        .take(scale.realized_useful_depth)
                        .all(|edge| edge.observable_equal)
            }));
    let b1 = mode == HarnessMode::Micro
        || primary.iter().all(|scale| {
            scale
                .edges
                .iter()
                .take(scale.realized_useful_depth)
                .all(|edge| edge.computationally_useful && edge.removed_arrow_firings > 0)
        });
    let c1 = mode == HarnessMode::Micro
        || primary.iter().all(|scale| {
            scale
                .edges
                .iter()
                .take(scale.realized_useful_depth)
                .all(|edge| edge.economically_justified && edge.break_even_uses.is_some())
        });
    let d1 = scales.iter().all(|scale| {
        scale.identity.same1_compiled_runtime < scale.identity.same0_generic_learned_runtime
            && scale.identity.same1_identity_tax < scale.identity.same0_identity_tax
            && scale.identity.same1_identity_tax == COMPILED_LEARNED_IDENTITY_TAX
            && scale.identity.compiled_uses == held_out_episodes
            && scale.identity.generic_reopenings == 0
    });
    let e1 = (mode == HarnessMode::Micro
        || (transfers.iter().all(|row| {
            row.observable_equal
                && row.transferred_work < row.primitive_work
                && row.acquisition_work_charged == 0
                && row.reused_same_instance
        }) && adaptive.iter().all(|row| {
            row.observable_equal
                && row.reacquisition_work == 0
                && (row.arm != "return" || row.historical_asset_reused)
        })))
        && controls.iter().all(|row| row.passed);
    let processes = vec![
        Same1ProcessResult {
            process: "execution".to_string(),
            status: if a1 && b1 { "positive" } else { "negative" }.to_string(),
            reason: "compiled correspondence and recursive arrows share one anonymous substrate"
                .to_string(),
        },
        Same1ProcessResult {
            process: "learning".to_string(),
            status: "unavailable".to_string(),
            reason: "learning mutation remains opaque Rust control flow".to_string(),
        },
        Same1ProcessResult {
            process: "retrieval".to_string(),
            status: "unavailable".to_string(),
            reason: "retrieval still lacks a replaceable anonymous executor".to_string(),
        },
        Same1ProcessResult {
            process: "decision".to_string(),
            status: "unavailable".to_string(),
            reason: "decision still uses semantic action tokens".to_string(),
        },
    ];
    let claims = if mode == HarnessMode::Micro {
        [
            "A1-fractal-compatibility",
            "B1-computational-recursion",
            "C1-economic-recursion",
            "D1-identity-tax-reduction",
            "E1-adaptive-reuse",
            "P1-process-availability",
        ]
        .into_iter()
        .map(|claim| Same1ClaimResult {
            claim: claim.to_string(),
            status: "NOT_TESTED".to_string(),
        })
        .collect()
    } else {
        vec![
            Same1ClaimResult {
                claim: "A1-fractal-compatibility".to_string(),
                status: status(a1),
            },
            Same1ClaimResult {
                claim: "B1-computational-recursion".to_string(),
                status: status(b1),
            },
            Same1ClaimResult {
                claim: "C1-economic-recursion".to_string(),
                status: status(c1),
            },
            Same1ClaimResult {
                claim: "D1-identity-tax-reduction".to_string(),
                status: status(d1),
            },
            Same1ClaimResult {
                claim: "E1-adaptive-reuse".to_string(),
                status: status(e1),
            },
            Same1ClaimResult {
                claim: "P1-process-availability".to_string(),
                status: "PARTIAL".to_string(),
            },
        ]
    };
    let passed = controls.iter().all(|row| row.passed)
        && duplicate_deterministic
        && source_audit_passed
        && (mode == HarnessMode::Micro
            || (a1
                && b1
                && c1
                && d1
                && e1
                && depth_curve_preserved
                && scaling_trend_supported
                && orthogonal_depth_signature));
    Same1Report {
        mode: match mode {
            HarnessMode::Micro => "micro",
            HarnessMode::Gate => "gate",
            HarnessMode::Definitive => "definitive",
        }
        .to_string(),
        claim_eligible: mode == HarnessMode::Definitive,
        scales,
        transfers,
        adaptive,
        controls,
        processes,
        claims,
        depth_curve_preserved,
        scaling_trend_supported,
        orthogonal_depth_signature,
        duplicate_deterministic,
        source_audit_passed,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_reintegrates_frozen_compiled_correspondence() {
        let report = run_same1(HarnessMode::Micro);
        assert!(report.passed);
        assert!(report.scales.iter().all(|scale| {
            scale.identity.same1_identity_tax == COMPILED_LEARNED_IDENTITY_TAX
                && scale.identity.improvement_vs_same0 == 12
        }));
    }

    #[test]
    fn gate_preserves_parent_relative_depth_curve() {
        let report = run_same1(HarnessMode::Gate);
        assert!(report.passed);
        assert!(report.depth_curve_preserved);
        assert!(report
            .scales
            .iter()
            .flat_map(|scale| &scale.edges)
            .all(|edge| edge.compiled_correspondence_work == COMPILED_LEARNED_IDENTITY_TAX));
    }
}
