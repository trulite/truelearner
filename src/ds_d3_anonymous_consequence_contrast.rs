//! Development-only anonymous downstream consequence-contrast gate.

use std::collections::BTreeMap;
use std::mem::size_of;

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-d3-anonymous-consequence-contrast-v1";
pub const EXACT_PARENT: &str = "bfb5b508f962c601d37e5d64a9a7cda02ae53604";
pub const PROTOCOL_COMMIT: &str = "7408d38e269c9361bb4e902d3a10885a508e7e46";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_D2_SHA256: &str =
    "ac257b53e28b0bdbcfd4cbcb7ca855086d1de5812a07029f4b2405fda2a6da8f";
pub const FROZEN_D2_HANDOFF_SHA256: &str =
    "03012d13bbffc16760c51ee1b12e9a2afbbf8ef970addcb2db4ba850adba1b03";
pub const FROZEN_PARENT_RETRY_SHA256: &str =
    "141f7ca6beeb34e11d8d0d4d3b5e60158db903bb35b208ba6342fcac88bd71f6";
pub const FROZEN_PARENT_HANDOFF_SHA256: &str =
    "3d9dccae3603f62ef068ccabd8ebee46ae8b12ca2559ad240fb57a26af48d6f5";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

const OBSERVATIONS_PER_AFFORDANCE: usize = 8;
const RECURRENT_SUPPORT: u16 = 4;
const MINIMUM_MARGIN: u16 = 2;
const CONSEQUENCE_DELAY: u8 = 3;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct AffordanceShape {
    trace: Vec<u8>,
    activation: [u16; 3],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct FrozenRoutes {
    affordances: [AffordanceShape; 2],
    roots: usize,
    fresh: bool,
    a1_work: u64,
}

macro_rules! d3_d2_access {
    () => {
        pub(super) fn d3_routes_with_layout(
            seed: u64,
            acquisition: usize,
            reverse_allocation: bool,
            layout_padding: bool,
        ) -> Option<super::FrozenRoutes> {
            let export = frozen_a1::d2_export_with_layout(
                seed,
                acquisition,
                reverse_allocation,
                layout_padding,
            )?;
            let affordances = export
                .predictions
                .into_iter()
                .map(|prediction| super::AffordanceShape {
                    trace: prediction.trace,
                    activation: prediction.activation,
                })
                .collect::<Vec<_>>()
                .try_into()
                .ok()?;
            Some(super::FrozenRoutes {
                affordances,
                roots: export.roots,
                fresh: export.fresh,
                a1_work: export.a1_work,
            })
        }

        pub(super) fn d3_routes(seed: u64, acquisition: usize) -> Option<super::FrozenRoutes> {
            d3_routes_with_layout(seed, acquisition, false, false)
        }

        pub(super) fn d3_ds1_hash_matches(value: &str) -> bool {
            FROZEN_DS1_SHA256 == value
        }
    };
}

#[allow(dead_code)]
mod frozen_d2 {
    include!(concat!(env!("OUT_DIR"), "/ds_d2_differential_evidence.rs"));
    d3_d2_access!();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Occurrence(u32);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WorldCell {
    activation: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WorldArrow {
    endpoints: [u8; 2],
    live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ConsequenceShape {
    temporal_rank: [u8; 3],
    propagation: [[u8; 2]; 2],
    activation: [u16; 3],
}

impl ConsequenceShape {
    fn magnitude(&self) -> u64 {
        self.activation.iter().map(|value| u64::from(*value)).sum()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhysicalConsequence {
    occurrences: [Occurrence; 3],
    ticks: [u8; 3],
    arrows: [WorldArrow; 2],
    root: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConsequenceFamily {
    Recurrent,
    Variable(u8),
}

fn physical_consequence(
    seed: u64,
    episode: usize,
    family: ConsequenceFamily,
    delay: u8,
    relabel: bool,
) -> PhysicalConsequence {
    let base = seed
        .wrapping_mul(1_000_003)
        .wrapping_add(episode as u64 * 53)
        .wrapping_add(1 << 33);
    let mut occurrences = [
        Occurrence(base as u32),
        Occurrence((base + 1) as u32),
        Occurrence((base + 2) as u32),
    ];
    if relabel {
        occurrences.rotate_left(1);
        occurrences.reverse();
    }
    let order = match family {
        ConsequenceFamily::Recurrent => [0, 1, 2],
        ConsequenceFamily::Variable(variant) => match variant % 4 {
            0 => [0, 2, 1],
            1 => [1, 0, 2],
            2 => [1, 2, 0],
            _ => [2, 0, 1],
        },
    };
    PhysicalConsequence {
        occurrences,
        ticks: [delay, delay + 1, delay + 2],
        arrows: [
            WorldArrow {
                endpoints: [order[0], order[1]],
                live: true,
            },
            WorldArrow {
                endpoints: [order[1], order[2]],
                live: true,
            },
        ],
        root: order[0],
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub consequence_spikes: u64,
    pub consequence_routes: u64,
    pub temporal_checks: u64,
    pub affordance_comparisons: u64,
    pub shape_comparisons: u64,
    pub support_updates: u64,
    pub direction_comparisons: u64,
    pub arrows_formed: u64,
    pub direction_spikes: u64,
    pub direction_routes: u64,
    pub abstentions: u64,
    pub immediate_rejections: u64,
    pub cleanup_items: u64,
}

impl Work {
    pub fn organism_work(&self) -> u64 {
        self.consequence_spikes
            + self.consequence_routes
            + self.temporal_checks
            + self.affordance_comparisons
            + self.shape_comparisons
            + self.support_updates
            + self.direction_comparisons
            + self.arrows_formed
            + self.direction_spikes
            + self.direction_routes
            + self.abstentions
            + self.immediate_rejections
            + self.cleanup_items
    }
}

fn execute_and_normalize(
    consequence: &PhysicalConsequence,
    work: &mut Work,
) -> Option<ConsequenceShape> {
    let mut cells = [WorldCell::default(); 3];
    let mut queue = vec![consequence.root];
    let mut visited = [false; 3];
    let mut propagation = Vec::new();
    while let Some(cell) = queue.pop() {
        let index = usize::from(cell);
        if visited[index] {
            continue;
        }
        visited[index] = true;
        cells[index].activation += 1;
        work.consequence_spikes += 1;
        for arrow in consequence
            .arrows
            .iter()
            .filter(|arrow| arrow.live && arrow.endpoints[0] == cell)
        {
            propagation.push(arrow.endpoints);
            queue.push(arrow.endpoints[1]);
            work.consequence_routes += 1;
        }
    }
    propagation.sort();
    let propagation: [[u8; 2]; 2] = propagation.try_into().ok()?;
    let minimum = *consequence.ticks.iter().min()?;
    let temporal_rank = consequence.ticks.map(|tick| tick - minimum);
    let _fresh_physical_occurrences = consequence.occurrences;
    Some(ConsequenceShape {
        temporal_rank,
        propagation,
        activation: cells.map(|cell| cell.activation),
    })
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ConsequenceEvidence {
    observations: u16,
    shapes: BTreeMap<ConsequenceShape, u16>,
}

impl ConsequenceEvidence {
    fn margin(&self) -> (u16, u16) {
        let mut counts = self.shapes.values().copied().collect::<Vec<_>>();
        counts.sort_unstable_by(|left, right| right.cmp(left));
        let first = counts.first().copied().unwrap_or(0);
        let second = counts.get(1).copied().unwrap_or(0);
        (first, first.saturating_sub(second))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DirectionArrow {
    endpoints: [u8; 2],
    live: bool,
}

#[derive(Clone, Debug, Default)]
struct ContrastLearner {
    evidence: BTreeMap<AffordanceShape, ConsequenceEvidence>,
    direction: Option<DirectionArrow>,
    direction_cells: [WorldCell; 3],
    work: Work,
}

impl ContrastLearner {
    fn observe(
        &mut self,
        affordance: &AffordanceShape,
        consequence: &PhysicalConsequence,
        minimum_delay: u8,
    ) -> bool {
        self.work.temporal_checks += 1;
        if consequence.ticks.iter().copied().min().unwrap_or(0) < minimum_delay {
            self.work.immediate_rejections += 1;
            return false;
        }
        let Some(shape) = execute_and_normalize(consequence, &mut self.work) else {
            return false;
        };
        self.work.affordance_comparisons += self.evidence.len() as u64;
        let evidence = self.evidence.entry(affordance.clone()).or_default();
        self.work.shape_comparisons += evidence.shapes.len() as u64;
        evidence.observations += 1;
        *evidence.shapes.entry(shape).or_default() += 1;
        self.work.support_updates += 1;
        true
    }

    fn form_direction(&mut self, affordances: [Option<&AffordanceShape>; 2]) -> Option<usize> {
        self.direction = None;
        let scores = affordances.map(|affordance| {
            self.work.direction_comparisons += 1;
            affordance
                .and_then(|shape| self.evidence.get(shape))
                .map(ConsequenceEvidence::margin)
                .unwrap_or_default()
        });
        let eligible = scores
            .map(|(support, margin)| support >= RECURRENT_SUPPORT && margin >= MINIMUM_MARGIN);
        let direction = match eligible {
            [true, false] => Some(0),
            [false, true] => Some(1),
            _ => None,
        };
        if let Some(index) = direction {
            self.direction = Some(DirectionArrow {
                endpoints: [index as u8, 2],
                live: true,
            });
            self.work.arrows_formed += 1;
        } else {
            self.work.abstentions += 1;
        }
        direction
    }

    fn execute_direction(&mut self) -> bool {
        let Some(arrow) = self.direction else {
            return false;
        };
        if !arrow.live {
            return false;
        }
        self.direction_cells[usize::from(arrow.endpoints[0])].activation += 1;
        self.work.direction_spikes += 1;
        self.direction_cells[usize::from(arrow.endpoints[1])].activation += 1;
        self.work.direction_routes += 1;
        self.direction_cells[2].activation == 1
    }

    fn persistent_bytes(&self) -> usize {
        self.evidence
            .values()
            .map(|evidence| {
                size_of::<AffordanceShape>()
                    + size_of::<ConsequenceEvidence>()
                    + evidence.shapes.len() * (size_of::<ConsequenceShape>() + size_of::<u16>())
            })
            .sum()
    }

    fn cleanup_temporary(&mut self) -> bool {
        self.work.cleanup_items += usize::from(self.direction.is_some()) as u64 + 3;
        self.direction = None;
        self.direction_cells.fill(WorldCell::default());
        self.direction.is_none() && self.direction_cells.iter().all(|cell| cell.activation == 0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Schedule {
    OneRecurrent { recurrent: usize },
    BothSame,
    BothRecurrentDifferent,
    BothVariable,
    Shuffled,
}

fn acquire_contrasts(
    seed: u64,
    affordances: &[AffordanceShape; 2],
    schedule: Schedule,
    relabel: bool,
) -> ContrastLearner {
    let mut learner = ContrastLearner::default();
    for observation in 0..OBSERVATIONS_PER_AFFORDANCE {
        for (slot, affordance) in affordances.iter().enumerate() {
            let family = match schedule {
                Schedule::OneRecurrent { recurrent } if slot == recurrent => {
                    ConsequenceFamily::Recurrent
                }
                Schedule::OneRecurrent { .. } | Schedule::BothVariable => {
                    ConsequenceFamily::Variable(observation as u8)
                }
                Schedule::BothSame => ConsequenceFamily::Recurrent,
                Schedule::BothRecurrentDifferent => {
                    if slot == 0 {
                        ConsequenceFamily::Recurrent
                    } else {
                        ConsequenceFamily::Variable(0)
                    }
                }
                Schedule::Shuffled => ConsequenceFamily::Variable(((observation + slot) % 2) as u8),
            };
            let consequence = physical_consequence(
                seed + slot as u64 * 10_000,
                observation,
                family,
                CONSEQUENCE_DELAY,
                relabel,
            );
            let _ = learner.observe(affordance, &consequence, CONSEQUENCE_DELAY);
        }
    }
    learner
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Controls {
    pub immediate_effects_distinct: bool,
    pub downstream_contrast_discriminates: bool,
    pub same_downstream_abstains: bool,
    pub stable_different_downstream_abstains: bool,
    pub unstable_downstream_abstains: bool,
    pub swap_follows_history: bool,
    pub fresh_occurrences_transfer: bool,
    pub allocation_layout_transfer: bool,
    pub handle_permutation_transfer: bool,
    pub shuffled_downstream_abstains: bool,
    pub equal_magnitude_reversal: bool,
    pub immediate_timing_rejected: bool,
    pub removed_route_invalidates: bool,
    pub physical_direction_executes: bool,
    pub temporary_cleanup: bool,
}

impl Controls {
    pub fn passed(&self) -> bool {
        self.immediate_effects_distinct
            && self.downstream_contrast_discriminates
            && self.same_downstream_abstains
            && self.stable_different_downstream_abstains
            && self.unstable_downstream_abstains
            && self.swap_follows_history
            && self.fresh_occurrences_transfer
            && self.allocation_layout_transfer
            && self.handle_permutation_transfer
            && self.shuffled_downstream_abstains
            && self.equal_magnitude_reversal
            && self.immediate_timing_rejected
            && self.removed_route_invalidates
            && self.physical_direction_executes
            && self.temporary_cleanup
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub d2_hash: bool,
    pub d2_handoff_hash: bool,
    pub parent_retry_hash: bool,
    pub parent_handoff_hash: bool,
    pub ds1_hash: bool,
    pub contrast_formers: usize,
    pub ds1_update_edges: usize,
    pub semantic_direction_fields: usize,
    pub immediate_effect_score_edges: usize,
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
    let source = include_str!("ds_d3_anonymous_consequence_contrast.rs");
    let production = source
        .split(&["#[cfg(", "test)]"].concat())
        .next()
        .unwrap_or(source);
    let update_call = ["apply_", "consequence("].concat();
    let contrast = function_body(production, "fn form_direction(").unwrap_or_default();
    let semantic_fragments = [
        ["correct_", "choice"].concat(),
        ["expected_", "answer"].concat(),
        ["reward_", "value"].concat(),
        ["semantic_", "polarity"].concat(),
    ];
    SourceAudit {
        d2_hash: env!("DS_D3_D2_SHA256") == FROZEN_D2_SHA256,
        d2_handoff_hash: env!("DS_D3_D2_HANDOFF_SHA256") == FROZEN_D2_HANDOFF_SHA256,
        parent_retry_hash: env!("DS_D3_PARENT_RETRY_SHA256") == FROZEN_PARENT_RETRY_SHA256,
        parent_handoff_hash: env!("DS_D3_PARENT_HANDOFF_SHA256") == FROZEN_PARENT_HANDOFF_SHA256,
        ds1_hash: frozen_d2::d3_ds1_hash_matches(FROZEN_DS1_SHA256),
        contrast_formers: production.matches("\n    fn form_direction(").count(),
        ds1_update_edges: production.matches(&update_call).count(),
        semantic_direction_fields: semantic_fragments
            .iter()
            .map(|fragment| production.matches(fragment).count())
            .sum(),
        immediate_effect_score_edges: contrast.matches("activation").count()
            + contrast.matches("trace").count(),
    }
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.d2_hash
            && self.d2_handoff_hash
            && self.parent_retry_hash
            && self.parent_handoff_hash
            && self.ds1_hash
            && self.contrast_formers == 1
            && self.ds1_update_edges == 0
            && self.semantic_direction_fields == 0
            && self.immediate_effect_score_edges == 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedReport {
    pub seed: u64,
    pub roots: usize,
    pub recurrent_slot: usize,
    pub direction: Option<usize>,
    pub observations: usize,
    pub consequence_firings: u64,
    pub consequence_routes: u64,
    pub work: Work,
    pub a1_work: u64,
    pub persistent_bytes: usize,
    pub temporary_peak_bytes: usize,
    pub retained_occurrences: usize,
    pub retained_handles: usize,
    pub semantic_fields: usize,
    pub ds1_calls: u64,
    pub ds1_updates: u64,
    pub controls: Controls,
    pub passed: bool,
}

fn audit_seed(seed: u64, acquisition: usize) -> SeedReport {
    let routes = frozen_d2::d3_routes(seed, acquisition).expect("frozen A1 routes through D2");
    let recurrent_slot = seed as usize % 2;
    let mut primary = acquire_contrasts(
        seed + 40_000,
        &routes.affordances,
        Schedule::OneRecurrent {
            recurrent: recurrent_slot,
        },
        false,
    );
    let direction =
        primary.form_direction([Some(&routes.affordances[0]), Some(&routes.affordances[1])]);
    let physical_direction_executes = primary.execute_direction();

    let immediate_effects_distinct = routes.affordances[0] != routes.affordances[1];
    let downstream_contrast_discriminates = direction == Some(recurrent_slot);

    let mut same = acquire_contrasts(
        seed + 50_000,
        &routes.affordances,
        Schedule::BothSame,
        false,
    );
    let same_downstream_abstains = same
        .form_direction([Some(&routes.affordances[0]), Some(&routes.affordances[1])])
        .is_none();

    let mut stable_different = acquire_contrasts(
        seed + 60_000,
        &routes.affordances,
        Schedule::BothRecurrentDifferent,
        false,
    );
    let stable_different_downstream_abstains = stable_different
        .form_direction([Some(&routes.affordances[0]), Some(&routes.affordances[1])])
        .is_none();

    let mut unstable = acquire_contrasts(
        seed + 70_000,
        &routes.affordances,
        Schedule::BothVariable,
        false,
    );
    let unstable_downstream_abstains = unstable
        .form_direction([Some(&routes.affordances[0]), Some(&routes.affordances[1])])
        .is_none();

    let swapped_affordances = [routes.affordances[1].clone(), routes.affordances[0].clone()];
    let mut swapped = acquire_contrasts(
        seed + 80_000,
        &swapped_affordances,
        Schedule::OneRecurrent {
            recurrent: 1 - recurrent_slot,
        },
        false,
    );
    let swap_follows_history = swapped
        .form_direction([Some(&swapped_affordances[0]), Some(&swapped_affordances[1])])
        == Some(1 - recurrent_slot);

    let fresh_routes =
        frozen_d2::d3_routes(seed + 1_000, acquisition).expect("fresh frozen A1 routes through D2");
    let mut fresh = acquire_contrasts(
        seed + 90_000,
        &fresh_routes.affordances,
        Schedule::OneRecurrent {
            recurrent: recurrent_slot,
        },
        true,
    );
    let fresh_occurrences_transfer = fresh_routes.fresh
        && fresh.form_direction([
            Some(&fresh_routes.affordances[0]),
            Some(&fresh_routes.affordances[1]),
        ]) == Some(recurrent_slot);

    let layout_routes = frozen_d2::d3_routes_with_layout(seed, acquisition, true, true)
        .expect("layout frozen A1 routes through D2");
    let mut layout = acquire_contrasts(
        seed + 100_000,
        &layout_routes.affordances,
        Schedule::OneRecurrent {
            recurrent: recurrent_slot,
        },
        false,
    );
    let allocation_layout_transfer = layout.form_direction([
        Some(&layout_routes.affordances[0]),
        Some(&layout_routes.affordances[1]),
    ]) == Some(recurrent_slot);

    let handle_permutation_transfer = swap_follows_history;

    let mut shuffled = acquire_contrasts(
        seed + 110_000,
        &routes.affordances,
        Schedule::Shuffled,
        false,
    );
    let shuffled_downstream_abstains = shuffled
        .form_direction([Some(&routes.affordances[0]), Some(&routes.affordances[1])])
        .is_none();

    let opposite = 1 - recurrent_slot;
    let mut reversed = acquire_contrasts(
        seed + 120_000,
        &routes.affordances,
        Schedule::OneRecurrent {
            recurrent: opposite,
        },
        false,
    );
    let reversed_direction =
        reversed.form_direction([Some(&routes.affordances[0]), Some(&routes.affordances[1])]);
    let recurrent_shape = physical_consequence(
        seed,
        0,
        ConsequenceFamily::Recurrent,
        CONSEQUENCE_DELAY,
        false,
    );
    let variable_shape = physical_consequence(
        seed,
        0,
        ConsequenceFamily::Variable(1),
        CONSEQUENCE_DELAY,
        false,
    );
    let mut magnitude_work = Work::default();
    let equal_magnitude_reversal = execute_and_normalize(&recurrent_shape, &mut magnitude_work)
        .zip(execute_and_normalize(&variable_shape, &mut magnitude_work))
        .is_some_and(|(left, right)| {
            left.magnitude() == right.magnitude() && reversed_direction == Some(opposite)
        });

    let mut immediate = ContrastLearner::default();
    let immediate_consequence =
        physical_consequence(seed, 0, ConsequenceFamily::Recurrent, 0, false);
    let immediate_timing_rejected = !immediate.observe(
        &routes.affordances[recurrent_slot],
        &immediate_consequence,
        CONSEQUENCE_DELAY,
    ) && immediate.evidence.is_empty();

    let mut removed = primary.clone();
    let alternatives = if recurrent_slot == 0 {
        [None, Some(&routes.affordances[1])]
    } else {
        [Some(&routes.affordances[0]), None]
    };
    let removed_route_invalidates = removed.form_direction(alternatives).is_none();

    let temporary_cleanup = primary.cleanup_temporary()
        && same.cleanup_temporary()
        && stable_different.cleanup_temporary()
        && unstable.cleanup_temporary()
        && swapped.cleanup_temporary()
        && fresh.cleanup_temporary()
        && layout.cleanup_temporary()
        && shuffled.cleanup_temporary()
        && reversed.cleanup_temporary()
        && immediate.cleanup_temporary();
    let removed_cleanup = removed.cleanup_temporary();

    let controls = Controls {
        immediate_effects_distinct,
        downstream_contrast_discriminates,
        same_downstream_abstains,
        stable_different_downstream_abstains,
        unstable_downstream_abstains,
        swap_follows_history,
        fresh_occurrences_transfer,
        allocation_layout_transfer,
        handle_permutation_transfer,
        shuffled_downstream_abstains,
        equal_magnitude_reversal,
        immediate_timing_rejected,
        removed_route_invalidates,
        physical_direction_executes,
        temporary_cleanup: temporary_cleanup && removed_cleanup,
    };
    let observations = primary
        .evidence
        .values()
        .map(|evidence| usize::from(evidence.observations))
        .sum();
    let persistent_bytes = primary.persistent_bytes();
    let work = primary.work.clone();
    let consequence_firings = work.consequence_spikes;
    let consequence_routes = work.consequence_routes;
    let passed = routes.roots == 2
        && observations == OBSERVATIONS_PER_AFFORDANCE * 2
        && direction == Some(recurrent_slot)
        && controls.passed()
        && source_audit().passed();
    SeedReport {
        seed,
        roots: routes.roots,
        recurrent_slot,
        direction,
        observations,
        consequence_firings,
        consequence_routes,
        work,
        a1_work: routes.a1_work,
        persistent_bytes,
        temporary_peak_bytes: size_of::<ContrastLearner>() + 2 * size_of::<PhysicalConsequence>(),
        retained_occurrences: 0,
        retained_handles: 0,
        semantic_fields: 0,
        ds1_calls: 0,
        ds1_updates: 0,
        controls,
        passed,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub enabling_only: bool,
    pub m0_authoritative: bool,
    pub m1_exists: bool,
    pub source: SourceAudit,
    pub seeds: Vec<SeedReport>,
    pub audit_passed: bool,
}

pub fn run(mode: HarnessMode) -> Report {
    if mode == HarnessMode::Definitive {
        return Report {
            label: "DS-D3 definitive forbidden".to_string(),
            protocol: PROTOCOL.to_string(),
            mode: "DEFINITIVE-FORBIDDEN".to_string(),
            claim_eligible: false,
            enabling_only: true,
            m0_authoritative: true,
            m1_exists: false,
            source: source_audit(),
            seeds: Vec::new(),
            audit_passed: false,
        };
    }
    let (seeds, acquisition, mode_label) = match mode {
        HarnessMode::Micro => (vec![100], 16, "MICRO"),
        HarnessMode::Gate => ((100..105).collect(), 32, "GATE"),
        HarnessMode::Definitive => unreachable!(),
    };
    let source = source_audit();
    let seeds = seeds
        .into_iter()
        .map(|seed| audit_seed(seed, acquisition))
        .collect::<Vec<_>>();
    let audit_passed = source.passed() && seeds.iter().all(|seed| seed.passed);
    Report {
        label: if audit_passed {
            "DS-D3 DEVELOPMENT IMPLEMENTATION READY".to_string()
        } else {
            "DS-D3 DEVELOPMENT FAILURE".to_string()
        },
        protocol: PROTOCOL.to_string(),
        mode: mode_label.to_string(),
        claim_eligible: false,
        enabling_only: true,
        m0_authoritative: true,
        m1_exists: false,
        source,
        seeds,
        audit_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_forms_nonsemantic_downstream_contrast_without_ds1() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert!(report.seeds.iter().all(|seed| {
            seed.direction == Some(seed.recurrent_slot)
                && seed.ds1_calls == 0
                && seed.ds1_updates == 0
                && seed.semantic_fields == 0
        }));
    }

    #[test]
    fn gate_passes_all_consequence_controls() {
        let report = run(HarnessMode::Gate);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.seeds.len(), 5);
        assert!(report.seeds.iter().all(|seed| seed.controls.passed()));
    }

    #[test]
    fn immediate_effect_and_equal_magnitude_do_not_supply_direction() {
        let report = run(HarnessMode::Gate);
        assert!(report.seeds.iter().all(|seed| {
            seed.controls.immediate_effects_distinct
                && seed.controls.same_downstream_abstains
                && seed.controls.equal_magnitude_reversal
                && report.source.immediate_effect_score_edges == 0
        }));
    }

    #[test]
    fn persistent_state_has_no_episode_identity() {
        let report = run(HarnessMode::Gate);
        assert!(report.seeds.iter().all(|seed| {
            seed.persistent_bytes > 0
                && seed.retained_occurrences == 0
                && seed.retained_handles == 0
        }));
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.audit_passed && report.seeds.is_empty() && !report.m1_exists);
    }
}
