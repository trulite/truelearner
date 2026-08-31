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

pub fn independent_outcome_components() -> Scenario {
    let surfaces = [SensorId(10), SensorId(11), SensorId(12), SensorId(13)];
    let outcomes = [SensorId(14), SensorId(15)];
    let motors = [MotorId(10), MotorId(11), MotorId(12), MotorId(13)];
    Scenario {
        name: "outcome-components-return-independently",
        morphology: Morphology {
            sensors: surfaces
                .into_iter()
                .chain(outcomes)
                .map(|id| integrating_sensor(id, 1))
                .collect(),
            motors: motors.map(|id| Motor { id }).to_vec(),
            nearby: surfaces
                .into_iter()
                .zip(motors)
                .map(|(sensor, motor)| Nearby {
                    sensor,
                    motor,
                    distance: 1,
                })
                .collect(),
            outcome_components: vec![
                OutcomeComponent {
                    source: outcomes[0],
                    motors: motors[..2].to_vec(),
                },
                OutcomeComponent {
                    source: outcomes[1],
                    motors: motors[2..].to_vec(),
                },
            ],
        },
        steps: vec![
            actions(0, 1, &[surfaces[1]], &[motors[1]], &[motors[1]]),
            returned_outcome(10, 2, outcomes[0]),
            actions(20, 3, &[surfaces[3]], &[motors[3]], &[motors[3]]),
            returned_outcome(30, 4, outcomes[1]),
            actions(
                40,
                7,
                &[surfaces[0], surfaces[2]],
                &[motors[0], motors[2]],
                &[motors[0], motors[2]],
            ),
            returned_outcome(50, 8, outcomes[0]),
            returned_outcome(51, 8, outcomes[1]),
            actions(60, 9, &surfaces, &motors, &[motors[0], motors[2]]),
        ],
    }
}

pub fn unanswered_output_releases() -> Scenario {
    let surfaces = [SensorId(20), SensorId(21)];
    let outcome = SensorId(22);
    let motors = [MotorId(20), MotorId(21)];
    Scenario {
        name: "unanswered-output-releases-to-an-alternative",
        morphology: Morphology {
            sensors: surfaces
                .into_iter()
                .chain([outcome])
                .map(|id| integrating_sensor(id, 1))
                .collect(),
            motors: motors.map(|id| Motor { id }).to_vec(),
            nearby: surfaces
                .into_iter()
                .zip(motors)
                .map(|(sensor, motor)| Nearby {
                    sensor,
                    motor,
                    distance: 1,
                })
                .collect(),
            outcome_components: vec![OutcomeComponent {
                source: outcome,
                motors: motors.to_vec(),
            }],
        },
        steps: vec![
            actions(0, 1, &[surfaces[0]], &[motors[0]], &[motors[0]]),
            actions(10, 2, &surfaces, &motors, &[motors[1]]),
        ],
    }
}

pub fn changed_contingency_releases_and_relearns() -> Scenario {
    let surfaces = [SensorId(30), SensorId(31)];
    let outcome = SensorId(32);
    let motors = [MotorId(30), MotorId(31)];
    Scenario {
        name: "changed-contingency-releases-and-relearns",
        morphology: Morphology {
            sensors: surfaces
                .into_iter()
                .chain([outcome])
                .map(|id| integrating_sensor(id, 1))
                .collect(),
            motors: motors.map(|id| Motor { id }).to_vec(),
            nearby: surfaces
                .into_iter()
                .zip(motors)
                .map(|(sensor, motor)| Nearby {
                    sensor,
                    motor,
                    distance: 1,
                })
                .collect(),
            outcome_components: vec![OutcomeComponent {
                source: outcome,
                motors: motors.to_vec(),
            }],
        },
        steps: vec![
            actions(0, 1, &[surfaces[0]], &[motors[0]], &[motors[0]]),
            returned_outcome(2, 1, outcome),
            actions(10, 2, &[surfaces[1]], &[motors[1]], &[motors[1]]),
            actions(20, 3, &surfaces, &motors, &[motors[0]]),
            actions(30, 4, &surfaces, &motors, &[motors[1]]),
            returned_outcome(32, 4, outcome),
            Step::Save {
                checkpoint: "switched",
            },
            actions(40, 5, &surfaces, &motors, &[motors[1]]),
            Step::Restore {
                checkpoint: "switched",
            },
            actions(40, 5, &surfaces, &motors, &[motors[1]]),
        ],
    }
}

fn actions(
    at: u64,
    cause: u64,
    surfaces: &[SensorId],
    opportunities: &[MotorId],
    expected: &[MotorId],
) -> Step {
    let inputs = surfaces
        .iter()
        .copied()
        .map(|sensor| BoundaryInput {
            at,
            target: InputTarget::Sensor(sensor),
            impulse: 1,
            cause,
        })
        .chain(opportunities.iter().copied().map(|motor| BoundaryInput {
            at: at + 1,
            target: InputTarget::Motor(motor),
            impulse: 1,
            cause,
        }))
        .collect();
    Step::Run(Episode {
        inputs,
        moment_limit: 64,
        expected: Expected::quiet(
            expected
                .iter()
                .copied()
                .map(|motor| Effect {
                    at: at + 2,
                    motor,
                    impulse: 1,
                    cause,
                })
                .collect(),
        ),
    })
}

fn returned_outcome(at: u64, cause: u64, outcome: SensorId) -> Step {
    Step::Run(Episode {
        inputs: vec![BoundaryInput {
            at,
            target: InputTarget::Sensor(outcome),
            impulse: 1,
            cause,
        }],
        moment_limit: 64,
        expected: Expected::quiet(Vec::new()),
    })
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
