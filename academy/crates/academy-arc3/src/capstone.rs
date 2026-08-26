use crate::{
    Arc3Sensorimotor, Arc3SensorimotorError, Arc3SensorimotorObservation, Arc3SensorimotorSnapshot,
};
use academy_core::{ActionArguments, ActionCall, ActionCatalog, ActionId, ActionSchema};
use serde::{Deserialize, Serialize};

const PHYSICAL_ACTION_MAP: [u8; 6] = [1, 2, 3, 4, 5, 7];

/// The entire organism-visible ARC-AGI-3 process protocol.
///
/// Evaluator state is deliberately unrepresentable. `deny_unknown_fields`
/// makes accidental score, identity, teaching, or action-map leakage fail at
/// deserialization rather than being silently ignored.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case", deny_unknown_fields)]
pub enum Arc3CapstoneCommand {
    Observe {
        frame: Vec<u8>,
        actions: ActionCatalog,
    },
    Shutdown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc3CapstoneObservation {
    #[serde(flatten)]
    pub organism: Arc3SensorimotorObservation,
    pub call: Option<ActionCall>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "response", rename_all = "snake_case")]
pub enum Arc3CapstoneResponse {
    Ready(Arc3SensorimotorSnapshot),
    Observation(Arc3CapstoneObservation),
    Error { message: String },
}

/// A teaching-free capstone agent that owns the Harness-backed sensorimotor.
pub struct Arc3CapstoneAgent {
    organism: Arc3Sensorimotor,
}

impl Arc3CapstoneAgent {
    pub fn new(seed: u64) -> Result<Self, Arc3SensorimotorError> {
        Ok(Self {
            organism: Arc3Sensorimotor::new_spatial(seed)?,
        })
    }

    pub fn snapshot(&self) -> Result<Arc3SensorimotorSnapshot, Arc3SensorimotorError> {
        self.organism.snapshot()
    }

    pub fn observe(
        &mut self,
        frame: Vec<u8>,
        actions: ActionCatalog,
    ) -> Result<Arc3CapstoneObservation, Arc3SensorimotorError> {
        let nullary_actions = actions
            .offers()
            .iter()
            .filter_map(|offer| {
                (offer.schema == ActionSchema::Unit)
                    .then(|| u8::try_from(offer.id.get()).ok())
                    .flatten()
            })
            .collect::<Vec<_>>();
        if nullary_actions.is_empty() {
            return Err(Arc3SensorimotorError::boundary(
                "the offered actions require arguments; foveation is not installed",
            ));
        }
        let organism =
            self.organism
                .observe_autonomous(frame, &nullary_actions, &PHYSICAL_ACTION_MAP)?;
        let call = organism
            .action
            .map(|action| actions.call(ActionId::new(u16::from(action)), ActionArguments::Unit))
            .transpose()
            .map_err(|error| Arc3SensorimotorError::boundary(error.to_string()))?;
        Ok(Arc3CapstoneObservation { organism, call })
    }

    pub fn handle(
        &mut self,
        command: Arc3CapstoneCommand,
    ) -> Result<Option<Arc3CapstoneResponse>, Arc3SensorimotorError> {
        match command {
            Arc3CapstoneCommand::Observe { frame, actions } => self
                .observe(frame, actions)
                .map(Arc3CapstoneResponse::Observation)
                .map(Some),
            Arc3CapstoneCommand::Shutdown => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ARC3_FRAME_PIXELS;
    use academy_core::{ActionOffer, ActionSchema};

    fn frame() -> Vec<u8> {
        vec![0; ARC3_FRAME_PIXELS]
    }

    fn catalog(ids: &[u16]) -> ActionCatalog {
        ActionCatalog::new(
            ids.iter()
                .map(|id| ActionOffer {
                    id: ActionId::new(*id),
                    schema: if *id == 6 {
                        ActionSchema::Point {
                            width: 64,
                            height: 64,
                        }
                    } else {
                        ActionSchema::Unit
                    },
                })
                .collect(),
        )
        .unwrap()
    }

    #[test]
    fn evaluator_and_teaching_fields_are_rejected() {
        for forbidden in [
            r#"{"command":"observe","frame":[],"actions":{"offers":[{"id":1,"schema":{"type":"unit"}}]},"score":1}"#,
            r#"{"command":"observe","frame":[],"actions":{"offers":[{"id":1,"schema":{"type":"unit"}}]},"game_id":"ls20"}"#,
            r#"{"command":"observe","frame":[],"actions":{"offers":[{"id":1,"schema":{"type":"unit"}}]},"babble_action":1}"#,
        ] {
            assert!(serde_json::from_str::<Arc3CapstoneCommand>(forbidden).is_err());
        }
    }

    #[test]
    fn unsupported_actuator_fails_before_body_mutation() {
        let mut agent = Arc3CapstoneAgent::new(205).unwrap();
        let before = agent.snapshot().unwrap();
        let error = agent.observe(frame(), catalog(&[1, 99])).unwrap_err();
        let after = agent.snapshot().unwrap();

        assert!(error.to_string().contains("received 99"));
        assert_eq!(before, after);
    }

    #[test]
    fn point_only_catalog_waits_for_foveation_without_mutation() {
        let mut agent = Arc3CapstoneAgent::new(205).unwrap();
        let before = agent.snapshot().unwrap();
        let error = agent.observe(frame(), catalog(&[6])).unwrap_err();
        let after = agent.snapshot().unwrap();

        assert!(error.to_string().contains("foveation is not installed"));
        assert_eq!(before, after);
    }

    #[test]
    fn fresh_untrained_body_explores_through_the_harness() {
        let mut agent = Arc3CapstoneAgent::new(205).unwrap();
        let response = agent
            .observe(frame(), catalog(&[1, 2, 3, 4, 5, 7]))
            .unwrap();

        assert_eq!(response.organism.sequence, 0);
        assert!(response.call.is_some());
        assert_eq!(
            response.call.map(|call| call.id.get()),
            response.organism.action.map(u16::from)
        );
        assert_eq!(response.organism.babble_action, response.organism.action);
        assert_eq!(response.organism.outward_crossings, 1);
        assert!(response.organism.naturally_quiescent);
    }

    #[test]
    fn every_nullary_arc_action_can_cross_as_a_unit_call() {
        for action in [1_u16, 2, 3, 4, 5, 7] {
            let mut agent = Arc3CapstoneAgent::new(205).unwrap();
            let response = agent.observe(frame(), catalog(&[action])).unwrap();
            assert_eq!(
                response.call,
                Some(ActionCall {
                    id: ActionId::new(action),
                    arguments: ActionArguments::Unit,
                })
            );
        }
    }

    #[test]
    fn catalogue_order_does_not_choose_the_action() {
        let mut left = Arc3CapstoneAgent::new(205).unwrap();
        let mut right = Arc3CapstoneAgent::new(205).unwrap();
        let left_response = left.observe(frame(), catalog(&[1, 2, 3, 4, 5, 7])).unwrap();
        let right_response = right
            .observe(frame(), catalog(&[7, 5, 4, 3, 2, 1]))
            .unwrap();

        assert_eq!(left_response, right_response);
        assert_eq!(left.snapshot().unwrap(), right.snapshot().unwrap());
    }
}
