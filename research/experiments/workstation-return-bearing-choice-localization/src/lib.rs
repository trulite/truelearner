#![forbid(unsafe_code)]

use academy_workstation::{SessionObservation, WorkstationSession, WorldError};
use serde::Serialize;
use truelearner_workstation::{
    BodyAxis, BodyControl, BodyMovement, Direction, Protocol, ResearchChoiceDiagnostic,
    ResearchHarnessConfig, ResearchOpportunityIncidence, ResearchTransitionOpportunity,
};

const SEED: u64 = 82_001;
const MAX_STEPS: usize = 48;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct BreakEvidence {
    sequence: u64,
    returned_axis: BodyAxis,
    prior_control: BodyControl,
    prior_movement: BodyMovement,
    current_movement: Option<BodyMovement>,
    classification: &'static str,
    prior_candidate: Option<ResearchChoiceDiagnostic>,
    continuation_evaluations: Vec<ResearchChoiceDiagnostic>,
    relevant_choices: Vec<ResearchChoiceDiagnostic>,
    all_step_diagnostic_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LocalizationEvidence {
    schema: &'static str,
    outcome: &'static str,
    seed: u64,
    observed_steps: usize,
    observer_stopped_after_break: bool,
    exact_replay: bool,
    naturally_quiescent: bool,
    first_break: Option<BreakEvidence>,
}

pub fn run() -> Result<LocalizationEvidence, WorldError> {
    let config = candidate_config();
    let mut session = WorkstationSession::new_research(SEED, config)?;
    let checkpoint = session.save()?;
    let mut observations = Vec::new();
    let mut first_break = None;
    for _ in 0..MAX_STEPS {
        observations.push(session.step()?);
        if observations.len() >= 2 {
            first_break = find_break(&observations);
            if first_break.is_some() {
                break;
            }
        }
    }

    let mut replay = WorkstationSession::restore_research_config(checkpoint, config)?;
    let mut exact_replay = true;
    for expected in &observations {
        exact_replay &= replay.step()? == *expected;
    }
    let naturally_quiescent = observations
        .iter()
        .all(|observation| observation.body.naturally_quiescent);
    let outcome = if first_break.is_some() && exact_replay && naturally_quiescent {
        "localized"
    } else {
        "inconclusive"
    };
    Ok(LocalizationEvidence {
        schema: "workstation-return-bearing-choice-localization/v1",
        outcome,
        seed: SEED,
        observed_steps: observations.len(),
        observer_stopped_after_break: first_break.is_some(),
        exact_replay,
        naturally_quiescent,
        first_break,
    })
}

fn candidate_config() -> ResearchHarnessConfig {
    ResearchHarnessConfig {
        protocol: Protocol::RecursiveLearnerCausalTopologyProductComposition,
        opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
        transition_opportunity: ResearchTransitionOpportunity::LocalAfterTransition,
    }
}

fn find_break(observations: &[SessionObservation]) -> Option<BreakEvidence> {
    let current = observations.last()?;
    let prior = observations.get(observations.len().checked_sub(2)?)?;
    for axis in current
        .body
        .returned_transitions
        .iter()
        .copied()
        .filter(|axis| contact_relevant(*axis))
    {
        let prior_movement = prior
            .body
            .movements
            .iter()
            .find(|movement| movement.changed && movement.axis == axis)
            .copied()?;
        let current_movement = current
            .body
            .movements
            .iter()
            .find(|movement| movement.axis == axis)
            .copied();
        let continues = current_movement.is_some_and(|movement| {
            movement.changed && movement.net_impulse.signum() == prior_movement.net_impulse.signum()
        });
        if continues {
            continue;
        }
        let prior_control = control_for(
            axis,
            if prior_movement.net_impulse.is_negative() {
                Direction::Decrease
            } else {
                Direction::Increase
            },
        );
        let prior_candidate = current
            .body
            .choice_diagnostics
            .iter()
            .find(|diagnostic| candidate_control(diagnostic) == Some(prior_control))
            .cloned();
        let continuation_evaluations = current
            .body
            .choice_diagnostics
            .iter()
            .filter(|diagnostic| continuation_axis(diagnostic) == Some(axis))
            .cloned()
            .collect::<Vec<_>>();
        let relevant_choices = current
            .body
            .choice_diagnostics
            .iter()
            .filter(|diagnostic| choice_mentions_axis(diagnostic, axis))
            .cloned()
            .collect::<Vec<_>>();
        let classification = classify(
            prior_control,
            prior_candidate.as_ref(),
            &continuation_evaluations,
            &relevant_choices,
            current_movement,
        );
        return Some(BreakEvidence {
            sequence: current.sequence,
            returned_axis: axis,
            prior_control,
            prior_movement,
            current_movement,
            classification,
            prior_candidate,
            continuation_evaluations,
            relevant_choices,
            all_step_diagnostic_count: current.body.choice_diagnostics.len(),
        });
    }
    None
}

fn classify(
    prior_control: BodyControl,
    prior_candidate: Option<&ResearchChoiceDiagnostic>,
    continuations: &[ResearchChoiceDiagnostic],
    choices: &[ResearchChoiceDiagnostic],
    current_movement: Option<BodyMovement>,
) -> &'static str {
    let Some(ResearchChoiceDiagnostic::Candidate { executable, .. }) = prior_candidate else {
        return "prior-direction-candidate-absent";
    };
    if !executable {
        return "prior-direction-candidate-non-executable";
    }
    if continuations.iter().any(|diagnostic| {
        matches!(diagnostic, ResearchChoiceDiagnostic::TransitionContinuation { control, current_owner_transition: true, admitted: false, .. } if *control == prior_control)
    }) {
        return "current-transition-not-admitted";
    }
    let admitted = choices.iter().any(|diagnostic| {
        matches!(diagnostic, ResearchChoiceDiagnostic::Choice { admitted_controls, .. } if admitted_controls.contains(&prior_control))
    });
    if !admitted {
        return "prior-direction-lost-choice";
    }
    if current_movement
        .is_some_and(|movement| movement.decrease_effort > 0 && movement.increase_effort > 0)
    {
        return "prior-direction-selected-but-canceled";
    }
    "admitted-without-matching-movement"
}

fn candidate_control(diagnostic: &ResearchChoiceDiagnostic) -> Option<BodyControl> {
    match diagnostic {
        ResearchChoiceDiagnostic::Candidate { control, .. } => Some(*control),
        _ => None,
    }
}

fn continuation_axis(diagnostic: &ResearchChoiceDiagnostic) -> Option<BodyAxis> {
    match diagnostic {
        ResearchChoiceDiagnostic::TransitionContinuation { control, .. } => Some(control.axis()),
        _ => None,
    }
}

fn choice_mentions_axis(diagnostic: &ResearchChoiceDiagnostic, axis: BodyAxis) -> bool {
    match diagnostic {
        ResearchChoiceDiagnostic::Choice {
            ordinary_control,
            current_transition_control,
            computed_winner_control,
            admitted_controls,
            ..
        } => {
            ordinary_control.is_some_and(|control| control.axis() == axis)
                || current_transition_control.is_some_and(|control| control.axis() == axis)
                || computed_winner_control.is_some_and(|control| control.axis() == axis)
                || admitted_controls
                    .iter()
                    .any(|control| control.axis() == axis)
        }
        _ => false,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_step_exposes_mapped_candidate_and_choice_diagnostics() {
        let mut session = WorkstationSession::new_research(SEED, candidate_config()).unwrap();
        session.step().unwrap();
        let observation = session.step().unwrap();
        assert!(
            observation
                .body
                .choice_diagnostics
                .iter()
                .any(|diagnostic| matches!(diagnostic, ResearchChoiceDiagnostic::Candidate { .. }))
        );
        assert!(
            observation
                .body
                .choice_diagnostics
                .iter()
                .any(|diagnostic| matches!(diagnostic, ResearchChoiceDiagnostic::Choice { .. }))
        );
    }
}
