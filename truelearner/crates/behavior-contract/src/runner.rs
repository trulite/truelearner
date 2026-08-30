use crate::{BehaviorMismatch, Episode, Morphology, Observation, Scenario, Step, ValidationError};
use std::{collections::BTreeMap, fmt::Debug};

pub trait Adapter {
    type Organism;
    type Checkpoint: Clone;
    type Error: Debug;

    fn build(&self, morphology: &Morphology) -> Result<Self::Organism, Self::Error>;
    fn run(
        &self,
        organism: &mut Self::Organism,
        episode: &Episode,
    ) -> Result<Observation, Self::Error>;
    fn save(&self, organism: &Self::Organism) -> Result<Self::Checkpoint, Self::Error>;
    fn restore(&self, checkpoint: &Self::Checkpoint) -> Result<Self::Organism, Self::Error>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContractError<E> {
    Invalid(ValidationError),
    Adapter(E),
    Mismatch(BehaviorMismatch),
    MissingCheckpoint(&'static str),
}

pub fn run_scenario<A: Adapter>(
    adapter: &A,
    scenario: &Scenario,
) -> Result<Vec<Observation>, ContractError<A::Error>> {
    scenario.validate().map_err(ContractError::Invalid)?;
    let mut organism = adapter
        .build(&scenario.morphology)
        .map_err(ContractError::Adapter)?;
    let mut checkpoints = BTreeMap::new();
    let mut observations = Vec::new();
    for step in &scenario.steps {
        match step {
            Step::Run(episode) => {
                let observation = adapter
                    .run(&mut organism, episode)
                    .map_err(ContractError::Adapter)?;
                observation
                    .compare(&episode.expected)
                    .map_err(ContractError::Mismatch)?;
                observations.push(observation);
            }
            Step::Save { checkpoint } => {
                checkpoints.insert(
                    *checkpoint,
                    adapter.save(&organism).map_err(ContractError::Adapter)?,
                );
            }
            Step::Restore { checkpoint } => {
                let saved = checkpoints
                    .get(checkpoint)
                    .ok_or(ContractError::MissingCheckpoint(checkpoint))?;
                organism = adapter.restore(saved).map_err(ContractError::Adapter)?;
            }
        }
    }
    Ok(observations)
}
