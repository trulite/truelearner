use crate::*;

pub const SURFACE: SensorId = SensorId(0);
pub const CONSEQUENCE: SensorId = SensorId(1);
pub const ACTION: MotorId = MotorId(0);

pub fn quiet() -> Scenario {
    Scenario {
        name: "quiet-is-identity",
        morphology: Morphology {
            sensors: vec![integrating_sensor(SURFACE, 1)],
            ..Morphology::default()
        },
        steps: vec![Step::Run(Episode {
            inputs: Vec::new(),
            moment_limit: 32,
            expected: Expected::quiet(Vec::new()),
        })],
    }
}

pub fn local_action(distance: u32, cause: u64) -> Scenario {
    let action_at = 1 + u64::from(distance);
    Scenario {
        name: "local-input-forms-and-chooses-one-action",
        morphology: action_morphology(distance),
        steps: vec![Step::Run(Episode {
            inputs: vec![
                BoundaryInput {
                    at: 0,
                    target: InputTarget::Sensor(SURFACE),
                    impulse: 1,
                    cause,
                },
                BoundaryInput {
                    at: 1,
                    target: InputTarget::Motor(ACTION),
                    impulse: 1,
                    cause,
                },
            ],
            moment_limit: 64,
            expected: Expected::quiet(vec![Effect {
                at: action_at,
                motor: ACTION,
                impulse: 1,
                cause,
            }]),
        })],
    }
}

pub fn no_local_action(distance: u32, cause: u64) -> Scenario {
    let mut scenario = local_action(distance, cause);
    scenario.name = "distant-input-does-not-form-an-action";
    let Step::Run(episode) = &mut scenario.steps[0] else {
        unreachable!()
    };
    episode.expected.effects.clear();
    scenario
}

pub fn learns_and_reuses(distance: u32, cause: u64) -> Scenario {
    let mut scenario = local_action(distance, cause);
    scenario.name = "consequence-is-retained-and-used-later";
    scenario.steps.extend([
        consequence(10, cause + 1),
        later_surface(20, distance, cause + 2),
    ]);
    scenario
}

pub fn checkpoint_replay(distance: u32, cause: u64) -> Scenario {
    let mut scenario = learns_and_reuses(distance, cause);
    scenario.name = "checkpoint-replays-the-same-next-episode";
    let replay = scenario.steps.pop().expect("reuse episode exists");
    scenario.steps.extend([
        Step::Save {
            checkpoint: "learned",
        },
        replay.clone(),
        Step::Restore {
            checkpoint: "learned",
        },
        replay,
    ]);
    scenario
}

fn consequence(at: u64, cause: u64) -> Step {
    Step::Run(Episode {
        inputs: vec![BoundaryInput {
            at,
            target: InputTarget::Sensor(CONSEQUENCE),
            impulse: 1,
            cause,
        }],
        moment_limit: 64,
        expected: Expected::quiet(Vec::new()),
    })
}

fn later_surface(at: u64, distance: u32, cause: u64) -> Step {
    Step::Run(Episode {
        inputs: vec![BoundaryInput {
            at,
            target: InputTarget::Sensor(SURFACE),
            impulse: 1,
            cause,
        }],
        moment_limit: 64,
        expected: Expected::quiet(vec![Effect {
            at: at + 1 + u64::from(distance),
            motor: ACTION,
            impulse: 1,
            cause,
        }]),
    })
}

fn action_morphology(distance: u32) -> Morphology {
    Morphology {
        sensors: vec![
            integrating_sensor(SURFACE, 1),
            integrating_sensor(CONSEQUENCE, 1),
        ],
        motors: vec![Motor { id: ACTION }],
        nearby: vec![Nearby {
            sensor: SURFACE,
            motor: ACTION,
            distance,
        }],
        outcome_components: vec![OutcomeComponent {
            source: CONSEQUENCE,
            motors: vec![ACTION],
        }],
    }
}

const fn integrating_sensor(id: SensorId, threshold: i32) -> Sensor {
    Sensor {
        id,
        retention: Retention::Integrating { threshold },
    }
}
