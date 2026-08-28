#![forbid(unsafe_code)]

use academy_workstation::{SessionObservation, WorkstationSession, WorldError};
use serde::Serialize;
use sha2::{Digest, Sha256};
use truelearner_workstation::{
    BodyAxis, Protocol, ResearchHarnessConfig, ResearchOpportunityIncidence,
    ResearchTransitionOpportunity,
};
use workstation_contact_contingency::{EVIDENCE_SEED, EVIDENCE_STEPS, project_observations};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ContinuationSummary {
    returned_components: u64,
    emitted_local_inputs: u64,
    return_steps_with_same_component_movement: u64,
    contact_relevant_returns: u64,
    contact_relevant_same_component_movements: u64,
    contact_relevant_same_direction_movements: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CandidateEvidence {
    schema: &'static str,
    outcome: &'static str,
    seed: u64,
    steps: usize,
    exact_replay: bool,
    trace_sha256: String,
    continuation: ContinuationSummary,
    contact: serde_json::Value,
}

pub fn run() -> Result<CandidateEvidence, WorldError> {
    let config = candidate_config();
    let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config)?;
    let checkpoint = session.save()?;
    let mut observations = Vec::with_capacity(EVIDENCE_STEPS);
    for _ in 0..EVIDENCE_STEPS {
        observations.push(session.step()?);
    }

    let mut replay = WorkstationSession::restore_research_config(checkpoint, config)?;
    let mut exact_replay = true;
    for expected in &observations {
        exact_replay &= replay.step()? == *expected;
    }
    exact_replay &= replay.save()? == session.save()?;

    let trace = serde_json::to_vec(&observations).map_err(|_| WorldError::InvalidRecording)?;
    let trace_sha256 = hex_digest(&trace);
    let contact = project_observations(
        EVIDENCE_SEED,
        trace_sha256.clone(),
        &observations,
        exact_replay,
    )?;
    let contact = serde_json::to_value(contact).map_err(|_| WorldError::InvalidRecording)?;
    let continuation = summarize_continuation(&observations);
    let contact_passed = contact["outcome"] == "contact-established";
    let separated = contact["five_finger_steps"] == 0;
    let work_bounded = contact["max_step_work"]
        .as_u64()
        .is_some_and(|work| work <= 2_000);
    let continued = continuation.contact_relevant_same_component_movements > 0;
    let outcome = if exact_replay && contact_passed && separated && work_bounded && continued {
        "passed"
    } else {
        "falsified"
    };

    Ok(CandidateEvidence {
        schema: "workstation-transition-local-opportunity/v1",
        outcome,
        seed: EVIDENCE_SEED,
        steps: EVIDENCE_STEPS,
        exact_replay,
        trace_sha256,
        continuation,
        contact,
    })
}

fn candidate_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::LocalAfterTransition,
    }
}

fn summarize_continuation(observations: &[SessionObservation]) -> ContinuationSummary {
    let mut summary = ContinuationSummary {
        returned_components: 0,
        emitted_local_inputs: 0,
        return_steps_with_same_component_movement: 0,
        contact_relevant_returns: 0,
        contact_relevant_same_component_movements: 0,
        contact_relevant_same_direction_movements: 0,
    };
    for (index, observation) in observations.iter().enumerate() {
        let returned = &observation.body.returned_transitions;
        summary.returned_components = summary
            .returned_components
            .saturating_add(as_u64(returned.len()));
        summary.emitted_local_inputs = summary
            .emitted_local_inputs
            .saturating_add(as_u64(returned.len()).saturating_mul(2));
        let same_component = returned.iter().any(|axis| {
            observation
                .body
                .movements
                .iter()
                .any(|movement| movement.changed && movement.axis == *axis)
        });
        summary.return_steps_with_same_component_movement = summary
            .return_steps_with_same_component_movement
            .saturating_add(u64::from(same_component));

        for axis in returned.iter().filter(|axis| contact_relevant(**axis)) {
            summary.contact_relevant_returns = summary.contact_relevant_returns.saturating_add(1);
            let current = observation
                .body
                .movements
                .iter()
                .find(|movement| movement.changed && movement.axis == *axis);
            if current.is_some() {
                summary.contact_relevant_same_component_movements = summary
                    .contact_relevant_same_component_movements
                    .saturating_add(1);
            }
            let prior = index.checked_sub(1).and_then(|prior_index| {
                observations[prior_index]
                    .body
                    .movements
                    .iter()
                    .find(|movement| movement.changed && movement.axis == *axis)
            });
            if current.zip(prior).is_some_and(|(current, prior)| {
                current.net_impulse.signum() == prior.net_impulse.signum()
            }) {
                summary.contact_relevant_same_direction_movements = summary
                    .contact_relevant_same_direction_movements
                    .saturating_add(1);
            }
        }
    }
    summary
}

fn contact_relevant(axis: BodyAxis) -> bool {
    matches!(axis, BodyAxis::PalmDepth | BodyAxis::FingerFlexion { .. })
}

fn as_u64(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unchanged_samples_open_no_local_opportunity() {
        let mut parent = WorkstationSession::new(EVIDENCE_SEED).unwrap();
        let mut candidate =
            WorkstationSession::new_research(EVIDENCE_SEED, candidate_config()).unwrap();

        let parent_first = parent.step().unwrap();
        let candidate_first = candidate.step().unwrap();
        assert_eq!(candidate_first, parent_first);
        assert!(!candidate_first.body.pending_transitions.is_empty());

        let parent_second = parent.step().unwrap();
        let candidate_second = candidate.step().unwrap();
        assert_eq!(
            candidate_second.body.admitted_inputs,
            parent_second.body.admitted_inputs
                + candidate_second.body.returned_transitions.len() * 2
        );
    }

    #[test]
    fn explicit_research_config_restores_the_exact_local_next_step() {
        let config = candidate_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        session.step().unwrap();
        let checkpoint = session.save().unwrap();
        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        assert_eq!(session.step().unwrap(), replay.step().unwrap());
        assert_eq!(session.save().unwrap(), replay.save().unwrap());
    }
}
