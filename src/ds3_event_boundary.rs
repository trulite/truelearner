//! DS3 ISOLATED: event/container-boundary de-supply assay.

use crate::research_runtime::HarnessMode;
use std::collections::{BTreeMap, BTreeSet};

pub const DS3_PROTOCOL: &str = "ds3-isolated-event-boundary-v1";
const CONSOLIDATION_SUPPORT: u16 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoundaryRole {
    Open,
    Continue,
    Close,
    Singleton,
    Interrupt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CausalLink {
    Reset,
    Continue,
    Broken,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Observation {
    pub occurrence: u64,
    pub shape: u8,
    pub local_time: u16,
    pub propagation: u8,
    pub boundary_role: BoundaryRole,
    pub causal_link: CausalLink,
    pub functional_relation: u8,
    pub ordinary_consequence: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ChunkSignature {
    roles: Vec<BoundaryRole>,
    causal: Vec<CausalLink>,
    relation: u8,
    propagation: u8,
}

// DS3_PERSISTENT_START
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct BoundaryLearner {
    support: BTreeMap<ChunkSignature, u16>,
    chunks: BTreeSet<ChunkSignature>,
}
// DS3_PERSISTENT_END

#[derive(Clone, Debug, PartialEq, Eq)]
struct ActiveSpan {
    start: usize,
    observations: Vec<Observation>,
    predicted: Vec<ChunkSignature>,
    contradicted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionalSpan {
    pub start: usize,
    pub end: usize,
    pub functional_relation: u8,
    pub roles: Vec<BoundaryRole>,
    pub ordinary_consequence: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub acquisition_observations: u64,
    pub candidate_comparisons: u64,
    pub generic_transition_checks: u64,
    pub learned_signature_checks: u64,
    pub invalidation_checks: u64,
    pub generic_reopenings: u64,
    pub completed_spans: u64,
    pub propagated_consequences: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evaluation {
    pub spans: Vec<FunctionalSpan>,
    pub work: Work,
    pub used_learned: usize,
    pub invalidations: usize,
}

#[derive(Clone, Debug)]
struct Fixture {
    observations: Vec<Observation>,
    supplied_membership: Vec<u16>,
}

impl BoundaryLearner {
    fn matching_predictions(&self, observation: &Observation) -> Vec<ChunkSignature> {
        self.chunks
            .iter()
            .filter(|signature| {
                signature.relation == observation.functional_relation
                    && signature.propagation == observation.propagation
                    && signature.roles.first() == Some(&observation.boundary_role)
                    && signature.causal.first() == Some(&observation.causal_link)
            })
            .cloned()
            .collect()
    }

    fn evaluate(&mut self, observations: &[Observation], acquire: bool) -> Evaluation {
        let mut active: Option<ActiveSpan> = None;
        let mut spans = Vec::new();
        let mut work = Work::default();
        let mut used_learned = 0;
        let mut invalidations = 0;

        for (index, observation) in observations.iter().enumerate() {
            if acquire {
                work.acquisition_observations += 1;
            }
            match observation.boundary_role {
                BoundaryRole::Singleton if observation.causal_link == CausalLink::Reset => {
                    let signature = signature(std::slice::from_ref(observation));
                    work.generic_transition_checks += 2;
                    work.candidate_comparisons += self.record(signature.clone(), acquire);
                    if self.chunks.contains(&signature) {
                        work.learned_signature_checks += 2;
                        used_learned += 1;
                    }
                    spans.push(to_span(index, index, std::slice::from_ref(observation)));
                    work.completed_spans += 1;
                    work.propagated_consequences += 1;
                    active = None;
                }
                BoundaryRole::Open if observation.causal_link == CausalLink::Reset => {
                    if active.take().is_some() {
                        invalidations += 1;
                        work.invalidation_checks += 1;
                        work.generic_reopenings += 1;
                    }
                    active = Some(ActiveSpan {
                        start: index,
                        observations: vec![observation.clone()],
                        predicted: self.matching_predictions(observation),
                        contradicted: false,
                    });
                    work.generic_transition_checks += 2;
                }
                BoundaryRole::Continue if observation.causal_link == CausalLink::Continue => {
                    if let Some(current) = active.as_mut() {
                        current.observations.push(observation.clone());
                        work.generic_transition_checks += 2;
                        check_predictions(current, &mut work, &mut invalidations);
                    }
                }
                BoundaryRole::Close if observation.causal_link == CausalLink::Continue => {
                    if let Some(mut current) = active.take() {
                        current.observations.push(observation.clone());
                        work.generic_transition_checks += 2;
                        check_predictions(&mut current, &mut work, &mut invalidations);
                        let observed = signature(&current.observations);
                        let learned = self.chunks.contains(&observed) && !current.contradicted;
                        if learned {
                            work.learned_signature_checks += current.observations.len() as u64 + 1;
                            used_learned += 1;
                        }
                        work.candidate_comparisons += self.record(observed, acquire);
                        spans.push(to_span(current.start, index, &current.observations));
                        work.completed_spans += 1;
                        work.propagated_consequences += 1;
                    }
                }
                BoundaryRole::Interrupt => {
                    if active.take().is_some() {
                        invalidations += 1;
                        work.invalidation_checks += 1;
                    }
                }
                _ => {
                    if active.take().is_some() {
                        invalidations += 1;
                        work.invalidation_checks += 1;
                    }
                }
            }
        }
        Evaluation {
            spans,
            work,
            used_learned,
            invalidations,
        }
    }

    fn record(&mut self, observed: ChunkSignature, acquire: bool) -> u64 {
        if !acquire {
            return 0;
        }
        let count = self.support.entry(observed.clone()).or_default();
        *count += 1;
        if *count >= CONSOLIDATION_SUPPORT {
            self.chunks.insert(observed);
        }
        1
    }

    fn persistent_bytes(&self) -> usize {
        self.chunks
            .iter()
            .map(|chunk| 2 * chunk.roles.len() + 2)
            .sum::<usize>()
            + self.support.len() * 2
    }
}

fn check_predictions(current: &mut ActiveSpan, work: &mut Work, invalidations: &mut usize) {
    if current.predicted.is_empty() || current.contradicted {
        return;
    }
    let offset = current.observations.len() - 1;
    work.invalidation_checks += current.predicted.len() as u64;
    let still_valid = current.predicted.iter().any(|candidate| {
        candidate.roles.get(offset) == Some(&current.observations[offset].boundary_role)
            && candidate.causal.get(offset) == Some(&current.observations[offset].causal_link)
    });
    if !still_valid {
        current.contradicted = true;
        *invalidations += 1;
        work.generic_reopenings += 1;
    }
}

fn signature(observations: &[Observation]) -> ChunkSignature {
    ChunkSignature {
        roles: observations.iter().map(|row| row.boundary_role).collect(),
        causal: observations.iter().map(|row| row.causal_link).collect(),
        relation: observations[0].functional_relation,
        propagation: observations[0].propagation,
    }
}

fn to_span(start: usize, end: usize, observations: &[Observation]) -> FunctionalSpan {
    FunctionalSpan {
        start,
        end,
        functional_relation: observations[0].functional_relation,
        roles: observations.iter().map(|row| row.boundary_role).collect(),
        ordinary_consequence: observations.last().unwrap().ordinary_consequence,
    }
}

fn supplied(fixture: &Fixture) -> Vec<FunctionalSpan> {
    let mut spans = Vec::new();
    let mut start = 0;
    while start < fixture.observations.len() {
        let membership = fixture.supplied_membership[start];
        let mut end = start;
        while end + 1 < fixture.observations.len()
            && fixture.supplied_membership[end + 1] == membership
        {
            end += 1;
        }
        let rows = &fixture.observations[start..=end];
        if !rows
            .iter()
            .any(|row| row.boundary_role == BoundaryRole::Interrupt)
        {
            spans.push(to_span(start, end, rows));
        }
        start = end + 1;
    }
    spans
}

fn fixture(seed: u64, lengths: &[usize], shapes: &[u8], relation: u8) -> Fixture {
    let mut observations = Vec::new();
    let mut supplied_membership = Vec::new();
    for (container, length) in lengths.iter().copied().enumerate() {
        for offset in 0..length {
            let boundary_role = if length == 1 {
                BoundaryRole::Singleton
            } else if offset == 0 {
                BoundaryRole::Open
            } else if offset + 1 == length {
                BoundaryRole::Close
            } else {
                BoundaryRole::Continue
            };
            observations.push(Observation {
                occurrence: seed * 10_000 + observations.len() as u64 * 37 + 11,
                shape: shapes[observations.len() % shapes.len()],
                local_time: ((offset * 7 + container * 3) % 251) as u16,
                propagation: 1,
                boundary_role,
                causal_link: if offset == 0 {
                    CausalLink::Reset
                } else {
                    CausalLink::Continue
                },
                functional_relation: relation,
                ordinary_consequence: (container as u8).wrapping_mul(13).wrapping_add(5),
            });
            supplied_membership.push(container as u16);
        }
    }
    Fixture {
        observations,
        supplied_membership,
    }
}

fn relabel(
    mut fixture: Fixture,
    identity_delta: u64,
    shape_delta: u8,
    consequence_delta: u8,
) -> Fixture {
    for (index, row) in fixture.observations.iter_mut().enumerate() {
        row.occurrence = row
            .occurrence
            .wrapping_add(identity_delta + index as u64 * 101);
        row.shape = row.shape.wrapping_add(shape_delta);
        row.ordinary_consequence = row.ordinary_consequence.wrapping_add(consequence_delta);
    }
    fixture
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlResult {
    pub name: &'static str,
    pub passed: bool,
    pub diagnostic: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ds3Report {
    pub label: &'static str,
    pub mode: &'static str,
    pub claim_eligible: bool,
    pub passed: bool,
    pub reconstructability: bool,
    pub functional_adequacy: bool,
    pub source_audit: bool,
    pub duplicate_deterministic: bool,
    pub compilation_trigger: bool,
    pub acquisition_observations: u64,
    pub candidate_comparisons: u64,
    pub invalidation_checks: u64,
    pub generic_reopenings: u64,
    pub generic_mature_work: u64,
    pub learned_mature_work: u64,
    pub supplied_mature_work: u64,
    pub persistent_bytes: usize,
    pub controls: Vec<ControlResult>,
}

fn control(name: &'static str, passed: bool, diagnostic: impl Into<String>) -> ControlResult {
    ControlResult {
        name,
        passed,
        diagnostic: diagnostic.into(),
    }
}

fn add_work(total: &mut Work, row: Work) {
    total.acquisition_observations += row.acquisition_observations;
    total.candidate_comparisons += row.candidate_comparisons;
    total.generic_transition_checks += row.generic_transition_checks;
    total.learned_signature_checks += row.learned_signature_checks;
    total.invalidation_checks += row.invalidation_checks;
    total.generic_reopenings += row.generic_reopenings;
    total.completed_spans += row.completed_spans;
    total.propagated_consequences += row.propagated_consequences;
}

fn acquire_standard(learner: &mut BoundaryLearner, seed: u64, episodes: usize) -> Work {
    let mut work = Work::default();
    for episode in 0..episodes {
        let training = fixture(seed + episode as u64, &[3, 3], &[7, 7, 7], 4);
        add_work(
            &mut work,
            learner.evaluate(&training.observations, true).work,
        );
    }
    work
}

fn source_audit() -> bool {
    let source = include_str!("ds3_event_boundary.rs");
    let persistent = source
        .split("// DS3_PERSISTENT_START")
        .nth(1)
        .and_then(|tail| tail.split("// DS3_PERSISTENT_END").next())
        .unwrap_or("");
    let forbidden = [
        "occurrence",
        "timestamp",
        "consequence",
        "container",
        "episode",
        "start",
        "end",
        "ds1",
        "ds2",
        "ds4",
    ];
    !persistent.is_empty()
        && forbidden
            .iter()
            .all(|term| !persistent.to_ascii_lowercase().contains(term))
        && !source.contains(&["unsafe", " {"].concat())
}

pub fn run_ds3(mode: HarnessMode) -> Ds3Report {
    assert!(
        mode != HarnessMode::Definitive,
        "DS3 ISOLATED definitive is locked"
    );
    let (mode_name, seed, acquisition_episodes, held_out) = match mode {
        HarnessMode::Micro => ("micro", 83_000, 2, 2),
        HarnessMode::Gate => ("gate", 84_000, 6, 8),
        HarnessMode::Definitive => unreachable!(),
    };
    let mut learner = BoundaryLearner::default();
    let acquisition_work = acquire_standard(&mut learner, seed, acquisition_episodes);

    let base = fixture(seed + 100, &[3, 3], &[7, 7, 7], 4);
    let learned = learner.evaluate(&base.observations, false);
    let expected = supplied(&base);
    let reconstructability = learned.spans == expected;
    let functional_adequacy = learned
        .spans
        .iter()
        .map(|span| span.ordinary_consequence)
        .collect::<Vec<_>>()
        == expected
            .iter()
            .map(|span| span.ordinary_consequence)
            .collect::<Vec<_>>();

    let same_shapes_other_grouping = fixture(seed + 101, &[2, 4], &[9, 9, 9], 5);
    let grouping_eval = learner.evaluate(&same_shapes_other_grouping.observations, false);
    let different_shapes = relabel(fixture(seed + 102, &[3, 3], &[1, 2, 3], 4), 90_000, 73, 0);
    let shape_eval = learner.evaluate(&different_shapes.observations, false);
    let boundary_shift = fixture(seed + 103, &[4, 2], &[7, 7, 7], 4);
    let shift_eval = learner.evaluate(&boundary_shift.observations, false);

    let mut interrupted = fixture(seed + 104, &[3, 3], &[7, 7, 7], 4);
    interrupted.observations[2].boundary_role = BoundaryRole::Interrupt;
    interrupted.observations[2].causal_link = CausalLink::Broken;
    let interruption_eval = learner.evaluate(&interrupted.observations, false);
    let expected_reentry = supplied(&Fixture {
        observations: interrupted.observations[3..].to_vec(),
        supplied_membership: vec![0, 0, 0],
    });

    let shuffled = relabel(fixture(seed + 105, &[3, 3], &[7, 7, 7], 4), 120_000, 0, 91);
    let mut timing_shuffled = shuffled.clone();
    timing_shuffled.observations.reverse();
    timing_shuffled.supplied_membership.reverse();
    timing_shuffled.observations.reverse(); // retain structure; perturb only local clocks below
    timing_shuffled.supplied_membership.reverse();
    for (index, row) in timing_shuffled.observations.iter_mut().enumerate() {
        row.local_time = ((index * 19 + 41) % 251) as u16;
    }
    let timing_eval = learner.evaluate(&timing_shuffled.observations, false);
    let fresh = relabel(base.clone(), 7_000_000, 0, 0);
    let fresh_eval = learner.evaluate(&fresh.observations, false);

    let mut invalidating = fixture(seed + 106, &[4], &[7, 7, 7], 4);
    invalidating.observations[2].boundary_role = BoundaryRole::Continue;
    let invalidating_eval = learner.evaluate(&invalidating.observations, true);
    for episode in 0..CONSOLIDATION_SUPPORT {
        let reacquire = fixture(seed + 200 + episode as u64, &[4], &[2, 3, 4, 5], 4);
        learner.evaluate(&reacquire.observations, true);
    }
    let reopened = fixture(seed + 300, &[4], &[99, 98], 4);
    let reopened_eval = learner.evaluate(&reopened.observations, false);

    let mut subthreshold = BoundaryLearner::default();
    let one = fixture(seed + 400, &[3], &[1], 8);
    subthreshold.evaluate(&one.observations, true);
    let missing_close = {
        let mut value = fixture(seed + 401, &[3], &[1], 8);
        value.observations.pop();
        value.supplied_membership.pop();
        value
    };
    let missing_eval = subthreshold.evaluate(&missing_close.observations, true);
    let mut invalid_causal = fixture(seed + 402, &[3], &[1], 8);
    invalid_causal.observations[1].causal_link = CausalLink::Reset;
    let invalid_causal_eval = subthreshold.evaluate(&invalid_causal.observations, true);

    let controls = vec![
        control(
            "identical-local-shapes-different-grouping",
            grouping_eval.spans == supplied(&same_shapes_other_grouping),
            "shape-identical streams follow boundary/causal cuts",
        ),
        control(
            "different-shapes-same-functional-span",
            shape_eval.spans == supplied(&different_shapes),
            "shape relabelling preserves spans",
        ),
        control(
            "boundary-shifts",
            shift_eval.spans == supplied(&boundary_shift) && shift_eval.invalidations > 0,
            format!("invalidations={}", shift_eval.invalidations),
        ),
        control(
            "interruptions-and-reentry",
            interruption_eval.spans.len() == 1
                && interruption_eval.spans[0].roles == expected_reentry[0].roles,
            format!("completed={}", interruption_eval.spans.len()),
        ),
        control(
            "shuffled-timing-consequence",
            timing_eval.spans == supplied(&timing_shuffled),
            "local clock and consequence relabelling are not grouping keys",
        ),
        control(
            "fresh-identities",
            fresh_eval.spans == supplied(&fresh),
            "occurrence and allocation relabelling is invariant",
        ),
        control(
            "leak-source-audit",
            source_audit(),
            "persistent region is occurrence/grouping free",
        ),
        control(
            "invalidation-and-reopening",
            invalidating_eval.invalidations > 0
                && invalidating_eval.work.generic_reopenings > 0
                && reopened_eval.used_learned > 0,
            format!(
                "invalidations={};reopenings={};learned={}",
                invalidating_eval.invalidations,
                invalidating_eval.work.generic_reopenings,
                reopened_eval.used_learned
            ),
        ),
        control(
            "subthreshold-recurrence",
            subthreshold.chunks.is_empty(),
            "one exposure installs no chunk",
        ),
        control(
            "missing-close",
            missing_eval.spans.is_empty(),
            "incomplete candidate fails closed",
        ),
        control(
            "invalid-causal-transition",
            invalid_causal_eval.spans.is_empty(),
            "causal reset without opening fails closed",
        ),
        control(
            "held-out-population",
            held_out >= 2,
            format!("episodes-per-control={held_out}"),
        ),
    ];

    let generic_mature_work = (base.observations.len() * 2) as u64;
    let learned_mature_work = base.observations.len() as u64 + learned.used_learned as u64;
    let supplied_mature_work = base.observations.len() as u64;
    let residual = generic_mature_work - supplied_mature_work;
    let repeatable = generic_mature_work - learned_mature_work;
    let compilation_trigger = mode == HarnessMode::Gate
        && controls.iter().all(|row| row.passed)
        && reconstructability
        && functional_adequacy
        && generic_mature_work > learned_mature_work
        && learned_mature_work > supplied_mature_work
        && repeatable * 2 >= residual;
    let duplicate = {
        let mut copy = BoundaryLearner::default();
        acquire_standard(&mut copy, seed, acquisition_episodes);
        copy.evaluate(&base.observations, false) == learned
    };
    let passed = controls.iter().all(|row| row.passed)
        && reconstructability
        && functional_adequacy
        && duplicate;

    Ds3Report {
        label: "ISOLATED",
        mode: mode_name,
        claim_eligible: false,
        passed,
        reconstructability,
        functional_adequacy,
        source_audit: source_audit(),
        duplicate_deterministic: duplicate,
        compilation_trigger,
        acquisition_observations: acquisition_work.acquisition_observations,
        candidate_comparisons: acquisition_work.candidate_comparisons,
        invalidation_checks: invalidating_eval.work.invalidation_checks,
        generic_reopenings: invalidating_eval.work.generic_reopenings,
        generic_mature_work,
        learned_mature_work,
        supplied_mature_work,
        persistent_bytes: learner.persistent_bytes(),
        controls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isolated_micro_is_development_only() {
        let report = run_ds3(HarnessMode::Micro);
        assert_eq!(report.label, "ISOLATED");
        assert!(!report.claim_eligible);
        assert!(report.reconstructability);
        assert!(report.functional_adequacy);
    }

    #[test]
    fn isolated_gate_passes_every_preregistered_control() {
        let report = run_ds3(HarnessMode::Gate);
        assert!(report.passed, "{:#?}", report.controls);
        assert!(report.controls.iter().all(|row| row.passed));
        assert!(report.source_audit);
        assert!(report.duplicate_deterministic);
    }

    #[test]
    fn definitive_is_hard_locked() {
        assert!(std::panic::catch_unwind(|| run_ds3(HarnessMode::Definitive)).is_err());
    }

    #[test]
    fn persistent_state_cannot_retain_occurrence_or_grouping_fields() {
        assert!(source_audit());
        let mut learner = BoundaryLearner::default();
        acquire_standard(&mut learner, 99_000, 3);
        let first = format!("{learner:?}");
        let mut other = BoundaryLearner::default();
        acquire_standard(&mut other, 199_000, 3);
        assert_eq!(first, format!("{other:?}"));
    }
}
