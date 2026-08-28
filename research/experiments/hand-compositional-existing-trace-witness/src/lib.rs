#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

const EXPECTED_SOURCE_SHA256: &str =
    "2baf097b176ee40ec9004529cb590db6365b5c1d887357e7fca9ac82ca9aebb2";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct TaggedEvent {
    event: String,
    evidence: Value,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct TracePoint {
    hand_step: usize,
    tick: i64,
    phase: i32,
    event: TaggedEvent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Verdict {
    OwnerProjectionGap,
    PhysicalWitnessDeallocated,
    PhysicalWitnessNotParticipating,
    InsufficientExistingTrace,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ComposedLinkEvidence {
    link: u64,
    generations: Vec<u64>,
    downstream_target_events: Vec<TracePoint>,
    deallocations: Vec<TracePoint>,
    failure_participation: Vec<TracePoint>,
    matching_owner_writes: Vec<TracePoint>,
    uninterrupted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct FoldEvidence {
    consequence_tick: i64,
    failure_tick: i64,
    witness_target: u64,
    fresh_owner: u64,
    consequence_links: Vec<u64>,
    missing_owner_preferences: Vec<TracePoint>,
    composed_links: Vec<ComposedLinkEvidence>,
    exact_supporting_slice: Vec<TracePoint>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OfflineResult {
    schema: &'static str,
    pub arm: &'static str,
    outcome: &'static str,
    source_sha256: String,
    source_hash_matches: bool,
    exact_first_failure_matches: bool,
    verdict: Verdict,
    reason: String,
    evidence: FoldEvidence,
}

fn number(value: &Value, field: &str) -> Option<u64> {
    value.get(field)?.as_u64()
}

fn signed(value: &Value, field: &str) -> Option<i64> {
    value.get(field)?.as_i64()
}

fn boolean(value: &Value, field: &str) -> Option<bool> {
    value.get(field)?.as_bool()
}

fn event_link(point: &TracePoint) -> Option<u64> {
    number(&point.event.evidence, "link")
}

fn analyze_points(
    trace: &[TracePoint],
    consequence_tick: i64,
    failure_tick: i64,
    witness_target: u64,
    fresh_owner: u64,
) -> (Verdict, String, FoldEvidence) {
    let consequence_links = trace
        .iter()
        .filter(|point| {
            point.tick == consequence_tick && point.event.event == "ConsequenceRecorded"
        })
        .filter_map(event_link)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let missing_owner_preferences = trace
        .iter()
        .filter(|point| {
            point.tick == failure_tick
                && point.event.event == "LearnerCandidatePreference"
                && number(&point.event.evidence, "owner") == Some(fresh_owner)
                && number(&point.event.evidence, "target") == Some(witness_target)
                && point.event.evidence.get("consequence_tick") == Some(&Value::Null)
        })
        .cloned()
        .collect::<Vec<_>>();
    let composed_links = consequence_links
        .iter()
        .map(|link| {
            let downstream_target_events = trace
                .iter()
                .filter(|point| {
                    point.tick > consequence_tick
                        && point.tick <= failure_tick
                        && point.event.event == "DriveProvenanceObserved"
                        && event_link(point) == Some(*link)
                        && number(&point.event.evidence, "target") == Some(witness_target)
                        && boolean(&point.event.evidence, "completes_path") == Some(true)
                })
                .cloned()
                .collect::<Vec<_>>();
            let deallocations = trace
                .iter()
                .filter(|point| {
                    point.tick >= consequence_tick
                        && point.tick <= failure_tick
                        && point.event.event == "LinkDeallocated"
                        && event_link(point) == Some(*link)
                })
                .cloned()
                .collect::<Vec<_>>();
            let failure_participation = downstream_target_events
                .iter()
                .filter(|point| point.tick == failure_tick)
                .cloned()
                .collect::<Vec<_>>();
            let matching_owner_writes = trace
                .iter()
                .filter(|point| {
                    point.tick >= consequence_tick
                        && point.tick <= failure_tick
                        && point.event.event == "LearnerConsequenceRecorded"
                        && event_link(point) == Some(*link)
                        && number(&point.event.evidence, "owner") == Some(fresh_owner)
                })
                .cloned()
                .collect::<Vec<_>>();
            let generations = trace
                .iter()
                .filter(|point| {
                    point.tick >= consequence_tick
                        && point.tick <= failure_tick
                        && point.event.event == "CausalLineageMemberObserved"
                        && event_link(point) == Some(*link)
                })
                .filter_map(|point| number(&point.event.evidence, "generation"))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            ComposedLinkEvidence {
                link: *link,
                generations,
                uninterrupted: deallocations.is_empty(),
                downstream_target_events,
                deallocations,
                failure_participation,
                matching_owner_writes,
            }
        })
        .collect::<Vec<_>>();
    let candidates = composed_links
        .iter()
        .filter(|link| !link.downstream_target_events.is_empty())
        .collect::<Vec<_>>();
    let live = candidates
        .iter()
        .filter(|link| link.uninterrupted)
        .copied()
        .collect::<Vec<_>>();
    let participating = live
        .iter()
        .filter(|link| !link.failure_participation.is_empty())
        .copied()
        .collect::<Vec<_>>();
    let exact_supporting_slice = trace
        .iter()
        .filter(|point| {
            consequence_links.contains(&event_link(point).unwrap_or(u64::MAX))
                || (point.tick == failure_tick
                    && matches!(
                        point.event.event.as_str(),
                        "LearnerCandidatePreference" | "OutputChoiceResolved"
                    ))
                || point.event.event == "LearnerConstructed"
        })
        .cloned()
        .collect::<Vec<_>>();

    let (verdict, reason) = if consequence_links.is_empty() || candidates.is_empty() {
        (
            Verdict::InsufficientExistingTrace,
            "no tick-eight consequence link composes into the completed target".to_owned(),
        )
    } else if live.is_empty() {
        (
            Verdict::PhysicalWitnessDeallocated,
            "every consequence link that composes into the target is deallocated before failure"
                .to_owned(),
        )
    } else if participating.is_empty() {
        (
            Verdict::PhysicalWitnessNotParticipating,
            "a composed consequence link remains live but does not participate at failure"
                .to_owned(),
        )
    } else if !missing_owner_preferences.is_empty()
        && participating
            .iter()
            .all(|link| link.matching_owner_writes.is_empty())
    {
        (
            Verdict::OwnerProjectionGap,
            "an uninterrupted consequence-bearing link still completes the target path at failure while the fresh owner has no matching private write and reports no consequence"
                .to_owned(),
        )
    } else {
        (
            Verdict::InsufficientExistingTrace,
            "physical participation and owner-local absence are not both established".to_owned(),
        )
    };
    (
        verdict,
        reason,
        FoldEvidence {
            consequence_tick,
            failure_tick,
            witness_target,
            fresh_owner,
            consequence_links,
            missing_owner_preferences,
            composed_links,
            exact_supporting_slice,
        },
    )
}

pub fn analyze_source(path: &Path) -> OfflineResult {
    let bytes = std::fs::read(path).expect("source artifact reads");
    let source_sha256 = format!("{:x}", Sha256::digest(&bytes));
    let source_hash_matches = source_sha256 == EXPECTED_SOURCE_SHA256;
    let source: Value = serde_json::from_slice(&bytes).expect("source artifact is JSON");
    let analysis = &source["observations"]["witness_analysis"];
    let consequence_tick = signed(analysis, "completed_consequence_tick").unwrap_or_default();
    let failure = &analysis["first_changed_choice"];
    let previous = &analysis["previous_choice"];
    let failure_tick = signed(failure, "tick").unwrap_or_default();
    let witness_target = number(analysis, "witness_target").unwrap_or_default();
    let fresh_owner = number(analysis, "fresh_owner").unwrap_or_default();
    let exact_first_failure_matches = consequence_tick == 8
        && failure_tick == 23
        && signed(previous, "tick") == Some(11)
        && previous["admitted"][0]["target"].as_u64() == Some(11)
        && failure["admitted"][0]["target"].as_u64() == Some(10)
        && failure["admitted"][0]["owner"].as_u64() == Some(2)
        && failure["admission_basis"].as_str() == Some("FreshAlternative")
        && failure["completed_cycle_state"].as_str() == Some("Missing");
    let trace: Vec<TracePoint> = serde_json::from_value(analysis["decisive_slice"].clone())
        .expect("decisive slice has the frozen schema");
    let (mut verdict, mut reason, evidence) = analyze_points(
        &trace,
        consequence_tick,
        failure_tick,
        witness_target,
        fresh_owner,
    );
    if !source_hash_matches || !exact_first_failure_matches {
        verdict = Verdict::InsufficientExistingTrace;
        reason =
            "the immutable source hash or exact first-failure control does not match".to_owned();
    }
    OfflineResult {
        schema: "hand-compositional-existing-trace-witness/v1",
        arm: "compositional-existing-trace-witness",
        outcome: if verdict == Verdict::InsufficientExistingTrace {
            "falsified"
        } else {
            "survived"
        },
        source_sha256,
        source_hash_matches,
        exact_first_failure_matches,
        verdict,
        reason,
        evidence,
    }
}

pub fn analyze_source_all(path: &Path) -> Vec<OfflineResult> {
    let diagnostic = analyze_source(path);
    let mut control = diagnostic.clone();
    control.arm = "immutable-source-control";
    control.outcome = if control.source_hash_matches && control.exact_first_failure_matches {
        "survived"
    } else {
        "falsified"
    };
    control.reason = if control.outcome == "survived" {
        "the immutable source hash and exact first-failure identity match".to_owned()
    } else {
        "the immutable source hash or exact first-failure identity differs".to_owned()
    };
    vec![control, diagnostic]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(tick: i64, event: &str, evidence: Value) -> TracePoint {
        TracePoint {
            hand_step: 0,
            tick,
            phase: 0,
            event: TaggedEvent {
                event: event.to_owned(),
                evidence,
            },
        }
    }

    fn base() -> Vec<TracePoint> {
        vec![
            point(
                8,
                "ConsequenceRecorded",
                serde_json::json!({"link": 36, "junction": 18}),
            ),
            point(
                15,
                "CausalLineageMemberObserved",
                serde_json::json!({"link": 36, "generation": 1}),
            ),
            point(16, "LearnerConstructed", serde_json::json!({"learner": 2})),
            point(
                23,
                "DriveProvenanceObserved",
                serde_json::json!({"link": 36, "target": 11, "completes_path": true}),
            ),
            point(
                23,
                "LearnerCandidatePreference",
                serde_json::json!({"owner": 2, "target": 11, "consequence_tick": null}),
            ),
        ]
    }

    #[test]
    fn compositional_verdict_distinguishes_all_registered_outcomes() {
        assert_eq!(
            analyze_points(&base(), 8, 23, 11, 2).0,
            Verdict::OwnerProjectionGap
        );

        let mut deallocated = base();
        deallocated.insert(
            3,
            point(20, "LinkDeallocated", serde_json::json!({"link": 36})),
        );
        assert_eq!(
            analyze_points(&deallocated, 8, 23, 11, 2).0,
            Verdict::PhysicalWitnessDeallocated
        );

        let non_participating = base()
            .into_iter()
            .map(|mut point| {
                if point.event.event == "DriveProvenanceObserved" {
                    point.tick = 22;
                }
                point
            })
            .collect::<Vec<_>>();
        assert_eq!(
            analyze_points(&non_participating, 8, 23, 11, 2).0,
            Verdict::PhysicalWitnessNotParticipating
        );

        let insufficient = base()
            .into_iter()
            .filter(|point| point.event.event != "ConsequenceRecorded")
            .collect::<Vec<_>>();
        assert_eq!(
            analyze_points(&insufficient, 8, 23, 11, 2).0,
            Verdict::InsufficientExistingTrace
        );
    }
}
