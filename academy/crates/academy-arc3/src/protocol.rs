use crate::{Arc3Error, Arc3Sensorimotor, Arc3SensorimotorObservation, Arc3SensorimotorSnapshot};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Arc3ActionSchema {
    Unit,
    Point { width: u8, height: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arc3ActionOffer {
    pub id: u8,
    pub schema: Arc3ActionSchema,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arc3ActionCatalog {
    pub offers: Vec<Arc3ActionOffer>,
}

impl Arc3ActionCatalog {
    pub fn validate(&self) -> Result<(), Arc3Error> {
        if self.offers.is_empty() {
            return Err(Arc3Error::boundary("the action catalog is empty"));
        }
        let mut ids = BTreeSet::new();
        for offer in &self.offers {
            if !(1..=7).contains(&offer.id) {
                return Err(Arc3Error::boundary(format!(
                    "received unsupported ARC action {}",
                    offer.id
                )));
            }
            if !ids.insert(offer.id) {
                return Err(Arc3Error::boundary(format!(
                    "ARC action {} is offered more than once",
                    offer.id
                )));
            }
            let expected = if offer.id == 6 {
                Arc3ActionSchema::Point {
                    width: 64,
                    height: 64,
                }
            } else {
                Arc3ActionSchema::Unit
            };
            if offer.schema != expected {
                return Err(Arc3Error::boundary(format!(
                    "ARC action {} has the wrong public argument shape",
                    offer.id
                )));
            }
        }
        Ok(())
    }

    pub fn contains(&self, id: u8) -> bool {
        self.offers.iter().any(|offer| offer.id == id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Arc3ActionArguments {
    Unit,
    Point { x: u8, y: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arc3ActionCall {
    pub id: u8,
    pub arguments: Arc3ActionArguments,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case", deny_unknown_fields)]
pub enum Arc3CapstoneCommand {
    Observe {
        frame: Vec<u8>,
        actions: Arc3ActionCatalog,
    },
    Shutdown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Arc3CapstoneObservation {
    #[serde(flatten)]
    pub organism: Arc3SensorimotorObservation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "response", rename_all = "snake_case")]
pub enum Arc3CapstoneResponse {
    Ready(Arc3SensorimotorSnapshot),
    Observation(Box<Arc3CapstoneObservation>),
    Error { message: String },
}

pub struct Arc3CapstoneAgent {
    organism: Arc3Sensorimotor,
}

impl Arc3CapstoneAgent {
    pub fn new(seed: u64) -> Result<Self, Arc3Error> {
        Ok(Self {
            organism: Arc3Sensorimotor::new(seed)?,
        })
    }

    pub fn snapshot(&self) -> Result<Arc3SensorimotorSnapshot, Arc3Error> {
        self.organism.snapshot()
    }

    pub fn observe(
        &mut self,
        frame: Vec<u8>,
        actions: Arc3ActionCatalog,
    ) -> Result<Arc3CapstoneObservation, Arc3Error> {
        let organism = self.organism.observe(frame, &actions)?;
        Ok(Arc3CapstoneObservation { organism })
    }

    pub fn handle(
        &mut self,
        command: Arc3CapstoneCommand,
    ) -> Result<Option<Arc3CapstoneResponse>, Arc3Error> {
        match command {
            Arc3CapstoneCommand::Observe { frame, actions } => self
                .observe(frame, actions)
                .map(Box::new)
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

    fn catalog(ids: &[u8]) -> Arc3ActionCatalog {
        Arc3ActionCatalog {
            offers: ids
                .iter()
                .map(|id| Arc3ActionOffer {
                    id: *id,
                    schema: if *id == 6 {
                        Arc3ActionSchema::Point {
                            width: 64,
                            height: 64,
                        }
                    } else {
                        Arc3ActionSchema::Unit
                    },
                })
                .collect(),
        }
    }

    #[test]
    fn evaluator_and_teaching_fields_are_unrepresentable() {
        for forbidden in [
            r#"{"command":"observe","frame":[],"actions":{"offers":[{"id":1,"schema":{"type":"unit"}}]},"score":1}"#,
            r#"{"command":"observe","frame":[],"actions":{"offers":[{"id":1,"schema":{"type":"unit"}}]},"game_id":"ls20"}"#,
            r#"{"command":"observe","frame":[],"actions":{"offers":[{"id":1,"schema":{"type":"unit"}}]},"expected_action":1}"#,
        ] {
            assert!(serde_json::from_str::<Arc3CapstoneCommand>(forbidden).is_err());
        }
    }

    #[test]
    fn invalid_catalog_fails_before_body_mutation() {
        let mut agent = Arc3CapstoneAgent::new(205).unwrap();
        let before = agent.snapshot().unwrap();
        let error = agent
            .observe(vec![0; ARC3_FRAME_PIXELS], catalog(&[1, 99]))
            .unwrap_err();
        assert!(error.to_string().contains("unsupported ARC action 99"));
        assert_eq!(agent.snapshot().unwrap(), before);
    }

    #[test]
    fn catalog_order_does_not_choose_the_action() {
        let mut left = Arc3CapstoneAgent::new(205).unwrap();
        let mut right = Arc3CapstoneAgent::new(205).unwrap();
        let left = left
            .observe(vec![0; ARC3_FRAME_PIXELS], catalog(&[1, 2, 3, 4, 5, 6, 7]))
            .unwrap();
        let right = right
            .observe(vec![0; ARC3_FRAME_PIXELS], catalog(&[7, 6, 5, 4, 3, 2, 1]))
            .unwrap();
        assert_eq!(left, right);
    }
}
