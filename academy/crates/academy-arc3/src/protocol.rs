use crate::{
    Arc3ActionWitness, Arc3Error, Arc3Sensorimotor, Arc3SensorimotorObservation,
    Arc3SensorimotorSnapshot,
};
use academy_workstation2::BezelControl;
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

    fn surface_affordances(&self) -> (bool, Vec<BezelControl>) {
        let controls = [
            (1, BezelControl::North),
            (2, BezelControl::South),
            (3, BezelControl::West),
            (4, BezelControl::East),
            (5, BezelControl::Primary),
            (7, BezelControl::Back),
        ]
        .into_iter()
        .filter_map(|(id, control)| self.contains(id).then_some(control))
        .collect();
        (self.contains(6), controls)
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
    pub action_witnesses: Vec<Arc3ActionWitness>,
    pub call: Option<Arc3ActionCall>,
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
    pub fn restore(body_checkpoint: &[u8]) -> Result<Self, Arc3Error> {
        Ok(Self {
            organism: Arc3Sensorimotor::restore(body_checkpoint)?,
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
        actions.validate()?;
        let (point_enabled, controls) = actions.surface_affordances();
        let organism = self.organism.observe(frame, point_enabled, &controls)?;
        let (action_witnesses, call) =
            filter_application_input(organism.application_input.as_ref(), &actions);
        Ok(Arc3CapstoneObservation {
            organism,
            action_witnesses,
            call,
        })
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

fn filter_application_input(
    input: Option<&crate::Arc3DeviceInput>,
    actions: &Arc3ActionCatalog,
) -> (Vec<Arc3ActionWitness>, Option<Arc3ActionCall>) {
    let action_witnesses = input
        .iter()
        .map(|input| Arc3ActionWitness {
            event: input.event,
            call: input.call,
            offered: actions.contains(input.call.id),
        })
        .collect::<Vec<_>>();
    let call = action_witnesses
        .first()
        .filter(|witness| witness.offered)
        .map(|witness| witness.call);
    (action_witnesses, call)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Arc3DeviceInput, ARC3_FRAME_PIXELS};
    use academy_workstation2::{DeviceEvent, TouchId};

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

    fn cold_body_negative_checkpoint() -> Vec<u8> {
        truelearner_workstation::WorkstationHarness::new(205)
            .unwrap()
            .save()
            .unwrap()
            .canonical_bytes()
            .unwrap()
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
        let mut agent = Arc3CapstoneAgent::restore(&cold_body_negative_checkpoint()).unwrap();
        let before = agent.snapshot().unwrap();
        let error = agent
            .observe(vec![0; ARC3_FRAME_PIXELS], catalog(&[1, 99]))
            .unwrap_err();
        assert!(error.to_string().contains("unsupported ARC action 99"));
        assert_eq!(agent.snapshot().unwrap(), before);
    }

    #[test]
    fn catalog_projects_to_generic_surface_affordances() {
        let (point, controls) = catalog(&[7, 6, 5, 1]).surface_affordances();
        assert!(point);
        assert_eq!(
            controls,
            vec![
                BezelControl::North,
                BezelControl::Primary,
                BezelControl::Back
            ]
        );
    }

    #[test]
    fn catalog_order_cannot_change_physical_workstation_steps() {
        let checkpoint = cold_body_negative_checkpoint();
        let mut left = Arc3CapstoneAgent::restore(&checkpoint).unwrap();
        let mut right = Arc3CapstoneAgent::restore(&checkpoint).unwrap();
        let left = left
            .observe(vec![0; ARC3_FRAME_PIXELS], catalog(&[1, 7]))
            .unwrap();
        let right = right
            .observe(vec![0; ARC3_FRAME_PIXELS], catalog(&[7, 1]))
            .unwrap();
        assert_eq!(left.organism.steps, right.organism.steps);
        assert_eq!(left.organism.device_events, right.organism.device_events);
    }

    #[test]
    fn offered_actions_filter_only_after_a_complete_device_gesture() {
        let input = Arc3DeviceInput {
            event: DeviceEvent::ContentActivated {
                touch: TouchId::new(0).unwrap(),
                column: 31,
                row: 31,
            },
            call: Arc3ActionCall {
                id: 6,
                arguments: Arc3ActionArguments::Point { x: 31, y: 31 },
            },
        };
        let (rejected, call) = filter_application_input(Some(&input), &catalog(&[1]));
        assert_eq!(rejected.len(), 1);
        assert!(!rejected[0].offered);
        assert_eq!(call, None);

        let (accepted, call) = filter_application_input(Some(&input), &catalog(&[6]));
        assert!(accepted[0].offered);
        assert_eq!(call, Some(input.call));
    }
}
