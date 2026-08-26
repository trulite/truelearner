use crate::{
    Arc3Sensorimotor, Arc3SensorimotorError, Arc3SensorimotorObservation, Arc3SensorimotorSnapshot,
};
use serde::{Deserialize, Serialize};

const PHYSICAL_ACTION_MAP: [u8; 4] = [1, 2, 3, 4];

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
        available_actions: Vec<u8>,
    },
    Shutdown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "response", rename_all = "snake_case")]
pub enum Arc3CapstoneResponse {
    Ready(Arc3SensorimotorSnapshot),
    Observation(Arc3SensorimotorObservation),
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
        available_actions: Vec<u8>,
    ) -> Result<Arc3SensorimotorObservation, Arc3SensorimotorError> {
        self.organism.observe(
            frame,
            &available_actions,
            None,
            false,
            false,
            &PHYSICAL_ACTION_MAP,
        )
    }

    pub fn handle(
        &mut self,
        command: Arc3CapstoneCommand,
    ) -> Result<Option<Arc3CapstoneResponse>, Arc3SensorimotorError> {
        match command {
            Arc3CapstoneCommand::Observe {
                frame,
                available_actions,
            } => self
                .observe(frame, available_actions)
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

    fn frame() -> Vec<u8> {
        vec![0; ARC3_FRAME_PIXELS]
    }

    #[test]
    fn evaluator_and_teaching_fields_are_rejected() {
        for forbidden in [
            r#"{"command":"observe","frame":[],"available_actions":[1],"score":1}"#,
            r#"{"command":"observe","frame":[],"available_actions":[1],"game_id":"ls20"}"#,
            r#"{"command":"observe","frame":[],"available_actions":[1],"babble_action":1}"#,
        ] {
            assert!(serde_json::from_str::<Arc3CapstoneCommand>(forbidden).is_err());
        }
    }

    #[test]
    fn unsupported_actuator_fails_before_body_mutation() {
        let mut agent = Arc3CapstoneAgent::new(205).unwrap();
        let before = agent.snapshot().unwrap();
        let error = agent.observe(frame(), vec![1, 6]).unwrap_err();
        let after = agent.snapshot().unwrap();

        assert!(error.to_string().contains("received 6"));
        assert_eq!(before, after);
    }

    #[test]
    fn fresh_untrained_body_is_allowed_to_remain_silent() {
        let mut agent = Arc3CapstoneAgent::new(205).unwrap();
        let response = agent.observe(frame(), vec![1, 2, 3, 4]).unwrap();

        assert_eq!(response.sequence, 0);
        assert_eq!(response.action, None);
        assert!(response.naturally_quiescent);
    }
}
