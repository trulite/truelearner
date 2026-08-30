use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SensorId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MotorId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retention {
    Integrating { threshold: i32 },
    Sampled { lifetime: u64, range: u32 },
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutcomeComponent {
    pub source: SensorId,
    pub motors: Vec<MotorId>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Morphology {
    pub sensors: Vec<Sensor>,
    pub motors: Vec<Motor>,
    pub nearby: Vec<Nearby>,
    pub outcome_components: Vec<OutcomeComponent>,
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
    InvalidRange(SensorId),
    UnknownSensor(SensorId),
    UnknownMotor(MotorId),
    ZeroDistance,
    EmptyOutcomeComponent(SensorId),
    DuplicateOutcomeSource(SensorId),
    DuplicateOutcomeMotor(MotorId),
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
                Retention::Sampled { lifetime: 0, .. } => {
                    return Err(ValidationError::InvalidLifetime(sensor.id));
                }
                Retention::Sampled { range, .. } if range == 0 || range > i32::MAX as u32 => {
                    return Err(ValidationError::InvalidRange(sensor.id));
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
        let mut outcome_sources = BTreeSet::new();
        let mut outcome_motors = BTreeSet::new();
        for component in &self.morphology.outcome_components {
            if !sensors.contains(&component.source) {
                return Err(ValidationError::UnknownSensor(component.source));
            }
            if component.motors.is_empty() {
                return Err(ValidationError::EmptyOutcomeComponent(component.source));
            }
            if !outcome_sources.insert(component.source) {
                return Err(ValidationError::DuplicateOutcomeSource(component.source));
            }
            for motor in &component.motors {
                if !motors.contains(motor) {
                    return Err(ValidationError::UnknownMotor(*motor));
                }
                if !outcome_motors.insert(*motor) {
                    return Err(ValidationError::DuplicateOutcomeMotor(*motor));
                }
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
