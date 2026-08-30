use std::collections::BTreeMap;
use truelearner_behavior_contract::{
    Adapter, Effect, Episode, InputTarget, Morphology, MotorId, Nearby, Observation, Retention,
    Sensor, SensorId,
};
use truelearner_body::{
    harness::{attach_sensor as attach_body_sensor, motor as attach_motor},
    Arrival, Body, Junction, JunctionId, RunError,
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
        for motor in &morphology.motors {
            organism.attach_motor(motor.id)?;
        }
        for sensor in &morphology.sensors {
            organism.attach_sensor(*sensor, &morphology.nearby)?;
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
            waves.entry(input.at).or_default().push(Arrival::caused(
                target,
                input.impulse,
                input.cause,
            ));
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
        organism
            .body
            .run(limit, |event| {
                if let Some(motor) = outward.get(&event.junction) {
                    raw_effects.push((event.at, *motor, event.impulse, event.cause));
                }
            })
            .map_err(NewAdapterError::Run)?;
        let effects = raw_effects
            .into_iter()
            .map(|(at, motor, impulse, cause)| {
                Ok(Effect {
                    at,
                    motor,
                    impulse: i32::try_from(impulse).map_err(|_| NewAdapterError::EffectOverflow)?,
                    cause,
                })
            })
            .collect::<Result<Vec<_>, NewAdapterError>>()?;
        Ok(Observation {
            effects,
            quiet: organism.body.is_quiet(),
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
}

const fn junction(retention: Retention) -> Junction {
    match retention {
        Retention::Integrating { threshold } => Junction::integrating(threshold),
        Retention::Sampled { lifetime } => Junction::sampled(lifetime),
    }
}
