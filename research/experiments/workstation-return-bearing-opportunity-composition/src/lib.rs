#![forbid(unsafe_code)]

use academy_workstation::{SessionObservation, WorkstationSession, WorldError};
use serde::Serialize;
use sha2::{Digest, Sha256};
use truelearner_workstation::{
    BodyAxis, BodyControl, BodyMovement, Direction, Protocol, ResearchChoiceDiagnostic,
    ResearchHarnessConfig, ResearchOpportunityIncidence, ResearchTransitionOpportunity,
};
use workstation_contact_contingency::{EVIDENCE_SEED, EVIDENCE_STEPS, project_observations};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ContinuationSummary {
    returned_components: u64,
    emitted_composed_inputs: u64,
    contact_relevant_returns: u64,
    contact_relevant_same_component_movements: u64,
    contact_relevant_same_direction_movements: u64,
    opposing_effort_steps: u64,
    admitted_current_transition_steps: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ComposedWitness {
    sequence: u64,
    axis: BodyAxis,
    control: BodyControl,
    prior_movement: BodyMovement,
    current_movement: BodyMovement,
    candidate: ResearchChoiceDiagnostic,
    continuation: ResearchChoiceDiagnostic,
    choice: ResearchChoiceDiagnostic,
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
    first_composed_witness: Option<ComposedWitness>,
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
    let continuation = summarize(&observations);
    let first_composed_witness = find_composed_witness(&observations);
    let separated = contact["five_finger_steps"] == 0;
    let work_bounded = contact["max_step_work"]
        .as_u64()
        .is_some_and(|work| work <= 2_000);
    let local_solve = first_composed_witness.is_some()
        && continuation.contact_relevant_same_direction_movements > 63
        && continuation.opposing_effort_steps == 0;
    let outcome = if exact_replay && separated && work_bounded && local_solve {
        "local-solve-passed"
    } else {
        "falsified"
    };

    Ok(CandidateEvidence {
        schema: "workstation-return-bearing-opportunity-composition/v1",
        outcome,
        seed: EVIDENCE_SEED,
        steps: EVIDENCE_STEPS,
        exact_replay,
        trace_sha256,
        continuation,
        first_composed_witness,
        contact,
    })
}

fn candidate_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::ComposedWithReturn,
    }
}

fn summarize(observations: &[SessionObservation]) -> ContinuationSummary {
    let mut summary = ContinuationSummary {
        returned_components: 0,
        emitted_composed_inputs: 0,
        contact_relevant_returns: 0,
        contact_relevant_same_component_movements: 0,
        contact_relevant_same_direction_movements: 0,
        opposing_effort_steps: 0,
        admitted_current_transition_steps: 0,
    };
    for (index, observation) in observations.iter().enumerate() {
        let returned = &observation.body.returned_transitions;
        summary.returned_components += as_u64(returned.len());
        summary.emitted_composed_inputs += as_u64(returned.len()).saturating_mul(2);
        summary.opposing_effort_steps += u64::from(
            observation
                .body
                .movements
                .iter()
                .any(|movement| movement.decrease_effort > 0 && movement.increase_effort > 0),
        );
        summary.admitted_current_transition_steps += u64::from(
            observation
                .body
                .choice_diagnostics
                .iter()
                .any(|diagnostic| {
                    matches!(
                        diagnostic,
                        ResearchChoiceDiagnostic::TransitionContinuation {
                            current_owner_transition: true,
                            admitted: true,
                            ..
                        }
                    )
                }),
        );
        for axis in returned
            .iter()
            .copied()
            .filter(|axis| contact_relevant(*axis))
        {
            summary.contact_relevant_returns += 1;
            let current = changed_movement(observation, axis);
            summary.contact_relevant_same_component_movements += u64::from(current.is_some());
            let prior = index
                .checked_sub(1)
                .and_then(|prior| changed_movement(&observations[prior], axis));
            summary.contact_relevant_same_direction_movements +=
                u64::from(current.zip(prior).is_some_and(|(current, prior)| {
                    current.net_impulse.signum() == prior.net_impulse.signum()
                }));
        }
    }
    summary
}

fn find_composed_witness(observations: &[SessionObservation]) -> Option<ComposedWitness> {
    for (index, current) in observations.iter().enumerate().skip(1) {
        let prior = &observations[index - 1];
        for axis in current
            .body
            .returned_transitions
            .iter()
            .copied()
            .filter(|axis| contact_relevant(*axis))
        {
            let prior_movement = changed_movement(prior, axis)?;
            let current_movement = changed_movement(current, axis)?;
            if current_movement.net_impulse.signum() != prior_movement.net_impulse.signum() {
                continue;
            }
            let control = control_for(
                axis,
                if prior_movement.net_impulse.is_negative() {
                    Direction::Decrease
                } else {
                    Direction::Increase
                },
            );
            let candidate = current.body.choice_diagnostics.iter().find(|diagnostic| {
                matches!(diagnostic, ResearchChoiceDiagnostic::Candidate { control: candidate, path_inputs, executable: true, .. } if *candidate == control && *path_inputs > 0)
            })?;
            let continuation = current.body.choice_diagnostics.iter().find(|diagnostic| {
                matches!(diagnostic, ResearchChoiceDiagnostic::TransitionContinuation { control: candidate, current_owner_transition: true, admitted: true, .. } if *candidate == control)
            })?;
            let choice = current.body.choice_diagnostics.iter().find(|diagnostic| {
                matches!(diagnostic, ResearchChoiceDiagnostic::Choice { current_transition_control: Some(candidate), admitted_controls, .. } if *candidate == control && admitted_controls.as_slice() == [control])
            })?;
            return Some(ComposedWitness {
                sequence: current.sequence,
                axis,
                control,
                prior_movement,
                current_movement,
                candidate: candidate.clone(),
                continuation: continuation.clone(),
                choice: choice.clone(),
            });
        }
    }
    None
}

fn changed_movement(observation: &SessionObservation, axis: BodyAxis) -> Option<BodyMovement> {
    observation
        .body
        .movements
        .iter()
        .find(|movement| movement.changed && movement.axis == axis)
        .copied()
}

fn contact_relevant(axis: BodyAxis) -> bool {
    matches!(axis, BodyAxis::PalmDepth | BodyAxis::FingerFlexion { .. })
}

fn control_for(axis: BodyAxis, direction: Direction) -> BodyControl {
    match axis {
        BodyAxis::EyeHorizontal { eye } => BodyControl::EyeHorizontal { eye, direction },
        BodyAxis::EyeVertical { eye } => BodyControl::EyeVertical { eye, direction },
        BodyAxis::PalmHorizontal => BodyControl::PalmHorizontal { direction },
        BodyAxis::PalmVertical => BodyControl::PalmVertical { direction },
        BodyAxis::PalmDepth => BodyControl::PalmDepth { direction },
        BodyAxis::Wrist => BodyControl::Wrist { direction },
        BodyAxis::Spread => BodyControl::Spread { direction },
        BodyAxis::ThumbOpposition => BodyControl::ThumbOpposition { direction },
        BodyAxis::FingerFlexion { digit } => BodyControl::FingerFlexion { digit, direction },
    }
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
    fn co_scheduled_return_and_opportunity_do_not_form_a_current_arrow() {
        let config = candidate_config();
        let mut session = WorkstationSession::new_research(EVIDENCE_SEED, config).unwrap();
        let checkpoint = session.save().unwrap();
        let mut observations = Vec::new();
        for _ in 0..48 {
            observations.push(session.step().unwrap());
            if find_composed_witness(&observations).is_some() {
                break;
            }
        }
        let summary = summarize(&observations);
        assert!(find_composed_witness(&observations).is_none());
        assert_eq!(summary.contact_relevant_same_component_movements, 0);
        assert_eq!(summary.admitted_current_transition_steps, 0);

        let mut replay = WorkstationSession::restore_research_config(checkpoint, config).unwrap();
        for expected in observations {
            assert_eq!(replay.step().unwrap(), expected);
        }
    }
}
