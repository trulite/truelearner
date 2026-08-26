use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActionId(u16);

impl ActionId {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionSchema {
    Unit,
    Point { width: u16, height: u16 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionOffer {
    pub id: ActionId,
    pub schema: ActionSchema,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionArguments {
    Unit,
    Point { x: u16, y: u16 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionCall {
    pub id: ActionId,
    pub arguments: ActionArguments,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ActionCatalogWire", into = "ActionCatalogWire")]
pub struct ActionCatalog {
    offers: Vec<ActionOffer>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ActionCatalogWire {
    offers: Vec<ActionOffer>,
}

impl ActionCatalog {
    pub fn new(offers: Vec<ActionOffer>) -> Result<Self, ActionError> {
        if offers.is_empty() {
            return Err(ActionError::EmptyCatalog);
        }
        let mut ids = BTreeSet::new();
        for offer in &offers {
            if !ids.insert(offer.id) {
                return Err(ActionError::DuplicateId(offer.id));
            }
            if let ActionSchema::Point { width, height } = offer.schema {
                if width == 0 || height == 0 {
                    return Err(ActionError::EmptyPointDomain(offer.id));
                }
            }
        }
        Ok(Self { offers })
    }

    pub fn offers(&self) -> &[ActionOffer] {
        &self.offers
    }

    pub fn offer(&self, id: ActionId) -> Option<ActionOffer> {
        self.offers.iter().copied().find(|offer| offer.id == id)
    }

    pub fn call(
        &self,
        id: ActionId,
        arguments: ActionArguments,
    ) -> Result<ActionCall, ActionError> {
        let offer = self.offer(id).ok_or(ActionError::NotOffered(id))?;
        match (offer.schema, arguments) {
            (ActionSchema::Unit, ActionArguments::Unit) => {}
            (ActionSchema::Point { width, height }, ActionArguments::Point { x, y })
                if x < width && y < height => {}
            (ActionSchema::Point { width, height }, ActionArguments::Point { x, y }) => {
                return Err(ActionError::PointOutsideDomain {
                    id,
                    x,
                    y,
                    width,
                    height,
                });
            }
            _ => return Err(ActionError::ArgumentMismatch(id)),
        }
        Ok(ActionCall { id, arguments })
    }
}

impl TryFrom<ActionCatalogWire> for ActionCatalog {
    type Error = ActionError;

    fn try_from(value: ActionCatalogWire) -> Result<Self, Self::Error> {
        Self::new(value.offers)
    }
}

impl From<ActionCatalog> for ActionCatalogWire {
    fn from(value: ActionCatalog) -> Self {
        Self {
            offers: value.offers,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionError {
    EmptyCatalog,
    DuplicateId(ActionId),
    EmptyPointDomain(ActionId),
    NotOffered(ActionId),
    ArgumentMismatch(ActionId),
    PointOutsideDomain {
        id: ActionId,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    },
}

impl fmt::Display for ActionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCatalog => formatter.write_str("action catalog is empty"),
            Self::DuplicateId(id) => write!(formatter, "action {} is offered twice", id.get()),
            Self::EmptyPointDomain(id) => {
                write!(formatter, "point action {} has an empty domain", id.get())
            }
            Self::NotOffered(id) => write!(formatter, "action {} is not offered", id.get()),
            Self::ArgumentMismatch(id) => {
                write!(
                    formatter,
                    "action {} received the wrong arguments",
                    id.get()
                )
            }
            Self::PointOutsideDomain {
                id,
                x,
                y,
                width,
                height,
            } => write!(
                formatter,
                "action {} point ({x}, {y}) is outside {width}x{height}",
                id.get()
            ),
        }
    }
}

impl std::error::Error for ActionError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog() -> ActionCatalog {
        ActionCatalog::new(vec![
            ActionOffer {
                id: ActionId::new(1),
                schema: ActionSchema::Unit,
            },
            ActionOffer {
                id: ActionId::new(6),
                schema: ActionSchema::Point {
                    width: 64,
                    height: 64,
                },
            },
        ])
        .unwrap()
    }

    #[test]
    fn only_complete_well_typed_calls_can_be_built() {
        let catalog = catalog();
        assert_eq!(
            catalog.call(ActionId::new(1), ActionArguments::Unit),
            Ok(ActionCall {
                id: ActionId::new(1),
                arguments: ActionArguments::Unit,
            })
        );
        assert_eq!(
            catalog.call(ActionId::new(6), ActionArguments::Point { x: 63, y: 63 }),
            Ok(ActionCall {
                id: ActionId::new(6),
                arguments: ActionArguments::Point { x: 63, y: 63 },
            })
        );
        assert!(matches!(
            catalog.call(ActionId::new(6), ActionArguments::Unit),
            Err(ActionError::ArgumentMismatch(_))
        ));
        assert!(matches!(
            catalog.call(ActionId::new(6), ActionArguments::Point { x: 64, y: 0 }),
            Err(ActionError::PointOutsideDomain { .. })
        ));
    }

    #[test]
    fn invalid_catalogs_fail_at_deserialization() {
        let duplicate =
            r#"{"offers":[{"id":1,"schema":{"type":"unit"}},{"id":1,"schema":{"type":"unit"}}]}"#;
        assert!(serde_json::from_str::<ActionCatalog>(duplicate).is_err());
        let empty_point =
            r#"{"offers":[{"id":6,"schema":{"type":"point","width":0,"height":64}}]}"#;
        assert!(serde_json::from_str::<ActionCatalog>(empty_point).is_err());
    }

    #[test]
    fn catalog_round_trips_canonically() {
        let catalog = catalog();
        let encoded = serde_json::to_string(&catalog).unwrap();
        assert_eq!(
            serde_json::from_str::<ActionCatalog>(&encoded).unwrap(),
            catalog
        );
    }
}
