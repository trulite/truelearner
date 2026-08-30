use std::convert::Infallible;
use truelearner_behavior_contract::{
    run_scenario, scenarios, Adapter, BehaviorMismatch, ContractError, Episode, Expected,
    Morphology, MotorId, Observation, OutcomeComponent, Scenario, SensorId, Step, ValidationError,
};

#[derive(Clone)]
struct Fake;

impl Adapter for Fake {
    type Organism = usize;
    type Checkpoint = usize;
    type Error = Infallible;

    fn build(&self, _: &Morphology) -> Result<Self::Organism, Self::Error> {
        Ok(0)
    }

    fn run(
        &self,
        organism: &mut Self::Organism,
        episode: &Episode,
    ) -> Result<Observation, Self::Error> {
        *organism += 1;
        Ok(Observation {
            effects: episode.expected.effects.clone(),
            quiet: episode.expected.quiet,
            trace: Default::default(),
        })
    }

    fn save(&self, organism: &Self::Organism) -> Result<Self::Checkpoint, Self::Error> {
        Ok(*organism)
    }

    fn restore(&self, checkpoint: &Self::Checkpoint) -> Result<Self::Organism, Self::Error> {
        Ok(*checkpoint)
    }
}

#[derive(Clone)]
struct Noisy;

impl Adapter for Noisy {
    type Organism = ();
    type Checkpoint = ();
    type Error = Infallible;

    fn build(&self, _: &Morphology) -> Result<Self::Organism, Self::Error> {
        Ok(())
    }

    fn run(&self, _: &mut Self::Organism, _: &Episode) -> Result<Observation, Self::Error> {
        Ok(Observation {
            effects: Vec::new(),
            quiet: false,
            trace: Default::default(),
        })
    }

    fn save(&self, _: &Self::Organism) -> Result<Self::Checkpoint, Self::Error> {
        Ok(())
    }

    fn restore(&self, _: &Self::Checkpoint) -> Result<Self::Organism, Self::Error> {
        Ok(())
    }
}

#[test]
fn validated_scenario_composes_run_save_restore_and_run() {
    let scenario = scenarios::checkpoint_replay(1, 7);
    let observations = run_scenario(&Fake, &scenario).unwrap();
    assert_eq!(observations.len(), 4);
}

#[test]
fn invalid_scenario_fails_before_adapter_construction() {
    let scenario = Scenario {
        name: "invalid",
        morphology: Morphology::default(),
        steps: vec![Step::Run(Episode {
            inputs: Vec::new(),
            moment_limit: 0,
            expected: Expected::quiet(Vec::new()),
        })],
    };
    assert_eq!(scenario.validate(), Err(ValidationError::ZeroMomentLimit));
}

#[test]
fn behavioral_mismatch_is_data_not_a_runner_panic() {
    assert_eq!(
        run_scenario(&Noisy, &scenarios::quiet()),
        Err(ContractError::Mismatch(BehaviorMismatch::Quiet {
            observed: false,
            expected: true,
        }))
    );
}

#[test]
fn unknown_nearby_motor_fails_before_adapter_construction() {
    let mut scenario = scenarios::local_action(1, 7);
    scenario.morphology.nearby[0].motor = MotorId(99);
    assert_eq!(
        scenario.validate(),
        Err(ValidationError::UnknownMotor(MotorId(99)))
    );
}

#[test]
fn outcome_components_are_part_of_validated_morphology() {
    let scenario = scenarios::local_action(1, 7);
    assert_eq!(
        scenario.morphology.outcome_components,
        [OutcomeComponent {
            source: scenarios::CONSEQUENCE,
            motors: vec![scenarios::ACTION],
        }]
    );

    let mut unknown_source = scenario.clone();
    unknown_source.morphology.outcome_components[0].source = SensorId(99);
    assert_eq!(
        unknown_source.validate(),
        Err(ValidationError::UnknownSensor(SensorId(99)))
    );

    let mut duplicate_motor = scenario;
    duplicate_motor
        .morphology
        .outcome_components
        .push(OutcomeComponent {
            source: scenarios::SURFACE,
            motors: vec![scenarios::ACTION],
        });
    assert_eq!(
        duplicate_motor.validate(),
        Err(ValidationError::DuplicateOutcomeMotor(scenarios::ACTION))
    );
}

#[test]
fn product_and_release_scenarios_are_valid_shared_data() {
    scenarios::independent_outcome_components()
        .validate()
        .unwrap();
    scenarios::unanswered_output_releases().validate().unwrap();
}
