#[path = "behavior_contract/new_body.rs"]
mod new_body;

use new_body::NewBodyAdapter;
use proptest::prelude::*;
use std::collections::BTreeSet;
use truelearner_behavior_contract::{
    run_scenario, scenarios, Adapter, BoundaryInput, Episode, Expected, InputTarget, LawTrace,
    Morphology, Motor, MotorId, Nearby, OutcomeComponent, Retention, Sensor, SensorId, TraceArrow,
};

fn symmetric_trace(count: u16, reversed: bool, base: u16) -> (LawTrace, BTreeSet<MotorId>) {
    let motors = (0..count)
        .map(|index| MotorId(base + index))
        .collect::<Vec<_>>();
    let sensors = (0..count)
        .map(|index| SensorId(base + 100 + index))
        .collect::<Vec<_>>();
    let outcome = SensorId(base + 200);
    let mut morphology = Morphology {
        sensors: sensors
            .iter()
            .copied()
            .chain([outcome])
            .map(|id| Sensor {
                id,
                retention: Retention::Integrating { threshold: 1 },
            })
            .collect(),
        motors: motors.iter().copied().map(|id| Motor { id }).collect(),
        nearby: sensors
            .iter()
            .copied()
            .zip(motors.iter().copied())
            .map(|(sensor, motor)| Nearby {
                sensor,
                motor,
                distance: 1,
            })
            .collect(),
        outcome_components: vec![OutcomeComponent {
            source: outcome,
            motors: motors.clone(),
        }],
    };
    if reversed {
        morphology.sensors.reverse();
        morphology.motors.reverse();
        morphology.nearby.reverse();
        morphology.outcome_components[0].motors.reverse();
    }

    let adapter = NewBodyAdapter;
    let mut organism = adapter
        .build(&morphology)
        .expect("valid generated morphology");
    let mut trace = LawTrace::default();
    let mut seen = BTreeSet::new();
    for turn in 0..count {
        let at = u64::from(turn) * 10;
        let cause = u64::from(turn) + 1;
        let (active_sensors, active_motors) = if turn == 0 {
            (&sensors[..1], &motors[..1])
        } else {
            (sensors.as_slice(), motors.as_slice())
        };
        let inputs = active_sensors
            .iter()
            .copied()
            .map(|sensor| BoundaryInput {
                at,
                target: InputTarget::Sensor(sensor),
                impulse: 1,
                cause,
            })
            .chain(active_motors.iter().copied().map(|motor| BoundaryInput {
                at: at + 1,
                target: InputTarget::Motor(motor),
                impulse: 1,
                cause,
            }))
            .collect();
        let observation = adapter
            .run(
                &mut organism,
                &Episode {
                    inputs,
                    moment_limit: 128,
                    expected: Expected::quiet(Vec::new()),
                },
            )
            .expect("generated episode runs");
        assert!(observation.quiet);
        assert_eq!(observation.effects.len(), 1);
        seen.insert(observation.effects[0].motor);
        trace = trace.then(observation.trace);
    }
    (trace, seen)
}

fn egglog_program(trace: &LawTrace) -> String {
    let mut program = String::from(include_str!("trace_laws.egg"));
    for arrow in &trace.arrows {
        match arrow {
            TraceArrow::Eligible {
                cause,
                sensor,
                motor,
                ..
            } => program.push_str(&format!("(eligible {cause} {} {})\n", sensor.0, motor.0)),
            TraceArrow::Candidate {
                cause,
                sensor,
                motor,
                ..
            } => {
                program.push_str(&format!(
                    "(candidate-at {cause} {} {})\n",
                    sensor.0, motor.0
                ));
                program.push_str(&format!("(candidate {cause} {})\n", motor.0));
            }
            TraceArrow::Choice { cause, motor, .. } => {
                program.push_str(&format!("(chosen {cause} {})\n", motor.0));
            }
            TraceArrow::Effect { cause, motor, .. } => {
                program.push_str(&format!("(effect {cause} {})\n", motor.0));
            }
            TraceArrow::Return { cause, motor, .. } => {
                program.push_str(&format!("(returned {cause} {})\n", motor.0));
            }
            _ => {}
        }
    }
    program.push_str("(run 4)\n");
    for arrow in &trace.arrows {
        let check = match arrow {
            TraceArrow::Eligible {
                cause,
                sensor,
                motor,
                ..
            } => Some(format!(
                "(check (present {cause} {} {}))\n",
                sensor.0, motor.0
            )),
            TraceArrow::Choice { cause, motor, .. } => {
                Some(format!("(check (path-chosen {cause} {}))\n", motor.0))
            }
            TraceArrow::Effect { cause, motor, .. } => {
                Some(format!("(check (moved {cause} {}))\n", motor.0))
            }
            TraceArrow::Return { cause, motor, .. } => {
                Some(format!("(check (closed {cause} {}))\n", motor.0))
            }
            _ => None,
        };
        if let Some(check) = check {
            program.push_str(&check);
        }
    }
    program
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn symmetric_outputs_are_explored_without_naming_one(
        count in 2_u16..=6,
        reversed in any::<bool>(),
        base in 0_u16..=100,
    ) {
        let (trace, seen) = symmetric_trace(count, reversed, base);
        prop_assert_eq!(seen.len(), usize::from(count));
        prop_assert_eq!(trace.verify_composition(), Ok(()));
    }

    #[test]
    fn trace_composition_has_identity_and_associativity(
        left_motor in any::<u16>(),
        middle_motor in any::<u16>(),
        right_motor in any::<u16>(),
    ) {
        let trace = |motor| LawTrace {
            arrows: vec![TraceArrow::Candidate {
                at: 1,
                cause: 1,
                sensor: SensorId(0),
                motor: MotorId(motor),
                new_path: true,
                participation: 0,
            }],
        };
        let left = trace(left_motor);
        let middle = trace(middle_motor);
        let right = trace(right_motor);
        let identity = LawTrace::default();

        prop_assert_eq!(identity.clone().then(left.clone()), left.clone());
        prop_assert_eq!(left.clone().then(identity), left.clone());
        prop_assert_eq!(
            left.clone().then(middle.clone()).then(right.clone()),
            left.then(middle.then(right)),
        );
    }
}

#[test]
fn egglog_proves_the_same_composed_trace() {
    let observations = run_scenario(&NewBodyAdapter, &scenarios::learns_and_reuses(1, 9)).unwrap();
    let trace = observations
        .into_iter()
        .fold(LawTrace::default(), |trace, observation| {
            trace.then(observation.trace)
        });
    trace.verify_composition().unwrap();

    let mut egraph = egglog::EGraph::default();
    egraph
        .parse_and_run_program(None, &egglog_program(&trace))
        .unwrap();
}

#[test]
fn egglog_rejects_a_missing_candidate_arrow() {
    let trace = LawTrace {
        arrows: vec![TraceArrow::Eligible {
            at: 1,
            cause: 1,
            sensor: SensorId(6),
            motor: MotorId(7),
        }],
    };
    assert!(trace.verify_composition().is_err());

    let mut egraph = egglog::EGraph::default();
    assert!(egraph
        .parse_and_run_program(None, &egglog_program(&trace))
        .is_err());
}
