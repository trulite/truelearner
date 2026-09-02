use std::collections::BTreeMap;
use truelearner_behavior_contract::{
    Adapter, Effect, Episode, InputTarget, LawTrace, Morphology, MotorId, Nearby, Observation,
    Retention, Sensor, SensorId, TraceArrow,
};
use truelearner_body::{
    harness::{
        attach_outcome_component, attach_sensor as attach_body_sensor, motor as attach_motor,
    },
    Arrival, Body, Junction, JunctionId, ReturnDecision, RunError, TraceEvent,
};

#[derive(Clone, Copy, Debug)]
pub struct NewBodyAdapter;

#[derive(Clone, Default)]
pub struct NewOrganism {
    body: Body,
    handles: Handles,
}

#[derive(Clone, Default)]
struct Handles {
    sensors: BTreeMap<SensorId, JunctionId>,
    motors: BTreeMap<MotorId, JunctionId>,
    outward: BTreeMap<JunctionId, MotorId>,
    nearby: Vec<Nearby>,
}

#[derive(Clone)]
pub struct NewCheckpoint(NewOrganism);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewAdapterError {
    UnknownSensor(SensorId),
    UnknownMotor(MotorId),
    DuplicateSensor(SensorId),
    DuplicateMotor(MotorId),
    ActiveCheckpoint,
    MomentLimitOverflow,
    EffectOverflow,
    Run(RunError),
}

impl Adapter for NewBodyAdapter {
    type Organism = NewOrganism;
    type Checkpoint = NewCheckpoint;
    type Error = NewAdapterError;

    fn build(&self, morphology: &Morphology) -> Result<Self::Organism, Self::Error> {
        let mut organism = NewOrganism::default();
        organism.handles.nearby.clone_from(&morphology.nearby);
        for motor in &morphology.motors {
            organism.attach_motor(motor.id)?;
        }
        for sensor in &morphology.sensors {
            organism.attach_sensor(*sensor, &morphology.nearby)?;
        }
        for component in &morphology.outcome_components {
            organism.attach_outcome_component(component)?;
        }
        Ok(organism)
    }

    fn run(
        &self,
        organism: &mut Self::Organism,
        episode: &Episode,
    ) -> Result<Observation, Self::Error> {
        let mut waves = BTreeMap::<u64, Vec<Arrival>>::new();
        for input in &episode.inputs {
            let target = match input.target {
                InputTarget::Sensor(sensor) => *organism
                    .handles
                    .sensors
                    .get(&sensor)
                    .ok_or(NewAdapterError::UnknownSensor(sensor))?,
                InputTarget::Motor(motor) => *organism
                    .handles
                    .motors
                    .get(&motor)
                    .ok_or(NewAdapterError::UnknownMotor(motor))?,
            };
            waves
                .entry(input.at)
                .or_default()
                .push(Arrival::new(target, input.impulse));
        }
        for (at, arrivals) in waves {
            organism
                .body
                .inputs(at, &arrivals)
                .map_err(NewAdapterError::Run)?;
        }

        let limit = usize::try_from(episode.moment_limit)
            .map_err(|_| NewAdapterError::MomentLimitOverflow)?;
        let outward = &organism.handles.outward;
        let mut raw_effects = Vec::new();
        let mut raw_trace = Vec::new();
        organism
            .body
            .run_traced(
                limit,
                |event| {
                    if let Some(motor) = outward.get(&event.junction) {
                        raw_effects.push((event.at, *motor, event.impulse));
                    }
                },
                |event| raw_trace.push(event),
            )
            .map_err(NewAdapterError::Run)?;
        let effects = raw_effects
            .into_iter()
            .map(|(at, motor, impulse)| {
                Ok(Effect {
                    at,
                    motor,
                    impulse: i32::try_from(impulse).map_err(|_| NewAdapterError::EffectOverflow)?,
                })
            })
            .collect::<Result<Vec<_>, NewAdapterError>>()?;
        let mut trace = LawTrace {
            arrows: episode
                .inputs
                .iter()
                .map(|input| TraceArrow::Input {
                    at: input.at,
                    target: input.target,
                })
                .collect(),
        };
        for event in &raw_trace {
            match event {
                TraceEvent::Candidate(candidate) => {
                    if let (Some(sensor), Some(motor)) = (
                        organism.sensor_for(candidate.path.surface),
                        organism.motor_for(candidate.path.output),
                    ) {
                        trace.arrows.push(TraceArrow::Candidate {
                            at: candidate.at,
                            sensor,
                            motor,
                            new_path: candidate.new_path,
                            participation: candidate.participation,
                        });
                    }
                }
                TraceEvent::Transition(transition) => {
                    let Some(sensor) = organism.sensor_for(transition.junction) else {
                        continue;
                    };
                    trace.arrows.extend(
                        organism
                            .handles
                            .nearby
                            .iter()
                            .filter(|nearby| nearby.sensor == sensor && nearby.distance <= 2)
                            .map(|nearby| TraceArrow::Eligible {
                                at: transition.at,
                                sensor,
                                motor: nearby.motor,
                            }),
                    );
                }
                TraceEvent::Choice(choice) => {
                    let Some(winner) = choice.winner else {
                        continue;
                    };
                    let Some(motor) = organism.motor_for(winner.output) else {
                        continue;
                    };
                    trace.arrows.push(TraceArrow::Choice {
                        at: choice.at,
                        motor,
                    });
                }
                TraceEvent::Return(returned) if returned.decision == ReturnDecision::Accepted => {
                    let Some(path) = returned.path else {
                        continue;
                    };
                    if let Some(motor) = organism.motor_for(path.output) {
                        trace.arrows.push(TraceArrow::Return {
                            at: returned.at,
                            motor,
                        });
                    }
                }
                TraceEvent::Strengthened(strengthened) => {
                    trace.arrows.push(TraceArrow::Strengthen {
                        at: strengthened.at,
                    });
                }
                _ => {}
            }
        }
        trace
            .arrows
            .extend(effects.iter().map(|effect| TraceArrow::Effect {
                at: effect.at,
                motor: effect.motor,
            }));
        Ok(Observation {
            effects,
            quiet: organism.body.is_quiet(),
            trace,
        })
    }

    fn save(&self, organism: &Self::Organism) -> Result<Self::Checkpoint, Self::Error> {
        if !organism.body.is_quiet() {
            return Err(NewAdapterError::ActiveCheckpoint);
        }
        Ok(NewCheckpoint(organism.clone()))
    }

    fn restore(&self, checkpoint: &Self::Checkpoint) -> Result<Self::Organism, Self::Error> {
        Ok(checkpoint.0.clone())
    }
}

impl NewOrganism {
    fn sensor_for(&self, junction: JunctionId) -> Option<SensorId> {
        self.handles
            .sensors
            .iter()
            .find_map(|(sensor, handle)| (*handle == junction).then_some(*sensor))
    }

    fn motor_for(&self, opportunity: JunctionId) -> Option<MotorId> {
        self.handles
            .motors
            .iter()
            .find_map(|(motor, junction)| (*junction == opportunity).then_some(*motor))
    }

    fn attach_motor(&mut self, id: MotorId) -> Result<(), NewAdapterError> {
        let motor = attach_motor(&mut self.body);
        if self.handles.motors.insert(id, motor.opportunity).is_some() {
            return Err(NewAdapterError::DuplicateMotor(id));
        }
        self.handles.outward.insert(motor.effect, id);
        Ok(())
    }

    fn attach_sensor(
        &mut self,
        sensor: Sensor,
        nearness: &[Nearby],
    ) -> Result<(), NewAdapterError> {
        let nearby_motors = nearness
            .iter()
            .filter(|nearby| nearby.sensor == sensor.id)
            .map(|nearby| {
                Ok((
                    *self
                        .handles
                        .motors
                        .get(&nearby.motor)
                        .ok_or(NewAdapterError::UnknownMotor(nearby.motor))?,
                    u64::from(nearby.distance),
                ))
            })
            .collect::<Result<Vec<_>, NewAdapterError>>()?;
        let handle = attach_body_sensor(&mut self.body, junction(sensor.retention), &nearby_motors);
        if self.handles.sensors.insert(sensor.id, handle).is_some() {
            return Err(NewAdapterError::DuplicateSensor(sensor.id));
        }
        Ok(())
    }

    fn attach_outcome_component(
        &mut self,
        component: &truelearner_behavior_contract::OutcomeComponent,
    ) -> Result<(), NewAdapterError> {
        let source = *self
            .handles
            .sensors
            .get(&component.source)
            .ok_or(NewAdapterError::UnknownSensor(component.source))?;
        for motor in &component.motors {
            if !self.handles.motors.contains_key(motor) {
                return Err(NewAdapterError::UnknownMotor(*motor));
            }
        }
        attach_outcome_component(
            &mut self.body,
            source,
            component
                .motors
                .iter()
                .map(|motor| self.handles.motors[motor]),
        );
        Ok(())
    }
}

const fn junction(retention: Retention) -> Junction {
    match retention {
        Retention::Integrating { threshold } => Junction::integrating(threshold),
        Retention::Sampled { lifetime, range } => Junction::sampled_in(lifetime, range),
    }
}
