use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SensorId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MotorId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retention {
    Integrating { threshold: i32 },
    Sampled { lifetime: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sensor {
    pub id: SensorId,
    pub retention: Retention,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Motor {
    pub id: MotorId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Nearby {
    pub sensor: SensorId,
    pub motor: MotorId,
    pub distance: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Morphology {
    pub sensors: Vec<Sensor>,
    pub motors: Vec<Motor>,
    pub nearby: Vec<Nearby>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputTarget {
    Sensor(SensorId),
    Motor(MotorId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundaryInput {
    pub at: u64,
    pub target: InputTarget,
    pub impulse: i32,
    pub cause: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Episode {
    pub inputs: Vec<BoundaryInput>,
    pub moment_limit: u64,
    pub expected: super::Expected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step {
    Run(Episode),
    Save { checkpoint: &'static str },
    Restore { checkpoint: &'static str },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scenario {
    pub name: &'static str,
    pub morphology: Morphology,
    pub steps: Vec<Step>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationError {
    EmptyName,
    DuplicateSensor(SensorId),
    DuplicateMotor(MotorId),
    InvalidThreshold(SensorId),
    InvalidLifetime(SensorId),
    UnknownSensor(SensorId),
    UnknownMotor(MotorId),
    ZeroDistance,
    TimeWentBackward { previous: u64, requested: u64 },
    ZeroMomentLimit,
    DuplicateCheckpoint(&'static str),
    UnknownCheckpoint(&'static str),
}

impl Scenario {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.is_empty() {
            return Err(ValidationError::EmptyName);
        }
        let mut sensors = BTreeSet::new();
        for sensor in &self.morphology.sensors {
            if !sensors.insert(sensor.id) {
                return Err(ValidationError::DuplicateSensor(sensor.id));
            }
            match sensor.retention {
                Retention::Integrating { threshold } if threshold <= 0 => {
                    return Err(ValidationError::InvalidThreshold(sensor.id));
                }
                Retention::Sampled { lifetime: 0 } => {
                    return Err(ValidationError::InvalidLifetime(sensor.id));
                }
                _ => {}
            }
        }
        let mut motors = BTreeSet::new();
        for motor in &self.morphology.motors {
            if !motors.insert(motor.id) {
                return Err(ValidationError::DuplicateMotor(motor.id));
            }
        }
        for nearby in &self.morphology.nearby {
            if !sensors.contains(&nearby.sensor) {
                return Err(ValidationError::UnknownSensor(nearby.sensor));
            }
            if !motors.contains(&nearby.motor) {
                return Err(ValidationError::UnknownMotor(nearby.motor));
            }
            if nearby.distance == 0 {
                return Err(ValidationError::ZeroDistance);
            }
        }

        let mut now = 0;
        let mut checkpoints = BTreeMap::new();
        for step in &self.steps {
            match step {
                Step::Run(episode) => {
                    if episode.moment_limit == 0 {
                        return Err(ValidationError::ZeroMomentLimit);
                    }
                    for input in &episode.inputs {
                        match input.target {
                            InputTarget::Sensor(sensor) if !sensors.contains(&sensor) => {
                                return Err(ValidationError::UnknownSensor(sensor));
                            }
                            InputTarget::Motor(motor) if !motors.contains(&motor) => {
                                return Err(ValidationError::UnknownMotor(motor));
                            }
                            _ => {}
                        }
                        if input.at < now {
                            return Err(ValidationError::TimeWentBackward {
                                previous: now,
                                requested: input.at,
                            });
                        }
                        now = input.at;
                    }
                }
                Step::Save { checkpoint } => {
                    if checkpoints.insert(*checkpoint, now).is_some() {
                        return Err(ValidationError::DuplicateCheckpoint(checkpoint));
                    }
                }
                Step::Restore { checkpoint } => {
                    now = *checkpoints
                        .get(checkpoint)
                        .ok_or(ValidationError::UnknownCheckpoint(checkpoint))?;
                }
            }
        }
        Ok(())
    }
}
