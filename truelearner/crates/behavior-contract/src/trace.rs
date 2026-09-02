use crate::{InputTarget, MotorId, SensorId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceArrow {
    Input {
        at: u64,
        target: InputTarget,
    },
    Eligible {
        at: u64,
        sensor: SensorId,
        motor: MotorId,
    },
    Candidate {
        at: u64,
        sensor: SensorId,
        motor: MotorId,
        new_path: bool,
        participation: u64,
    },
    Choice {
        at: u64,
        motor: MotorId,
    },
    Effect {
        at: u64,
        motor: MotorId,
    },
    Return {
        at: u64,
        motor: MotorId,
    },
    Strengthen {
        at: u64,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LawTrace {
    pub arrows: Vec<TraceArrow>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceLawViolation {
    EligibleWithoutCandidate {
        at: u64,
        sensor: SensorId,
        motor: MotorId,
    },
    ChoiceWithoutCandidate {
        at: u64,
        motor: MotorId,
    },
    EffectWithoutChoice {
        at: u64,
        motor: MotorId,
    },
    ReturnWithoutEffect {
        at: u64,
        motor: MotorId,
    },
    StrengthenWithoutReturn {
        at: u64,
    },
}

impl LawTrace {
    pub fn then(mut self, next: Self) -> Self {
        self.arrows.extend(next.arrows);
        self
    }

    pub fn verify_composition(&self) -> Result<(), TraceLawViolation> {
        for arrow in &self.arrows {
            match *arrow {
                TraceArrow::Eligible {
                    at,
                    sensor,
                    motor,
                } if !self.arrows.iter().any(|candidate| {
                    matches!(
                        candidate,
                        TraceArrow::Candidate {
                            at: candidate_at,
                            sensor: candidate_sensor,
                            motor: candidate_motor,
                            ..
                        } if *candidate_at == at
                            && *candidate_sensor == sensor
                            && *candidate_motor == motor
                    )
                }) => {
                    return Err(TraceLawViolation::EligibleWithoutCandidate {
                        at,
                        sensor,
                        motor,
                    });
                }
                TraceArrow::Choice { at, motor }
                    if !self.arrows.iter().any(|candidate| {
                        matches!(
                            candidate,
                            TraceArrow::Candidate {
                                at: candidate_at,
                                motor: candidate_motor,
                                ..
                            } if *candidate_at == at
                                && *candidate_motor == motor
                        )
                    }) =>
                {
                    return Err(TraceLawViolation::ChoiceWithoutCandidate { at, motor });
                }
                TraceArrow::Effect { at, motor }
                    if !self.arrows.iter().any(|choice| {
                        matches!(
                            choice,
                            TraceArrow::Choice {
                                at: choice_at,
                                motor: choice_motor,
                            } if *choice_at <= at
                                && *choice_motor == motor
                        )
                    }) =>
                {
                    return Err(TraceLawViolation::EffectWithoutChoice { at, motor });
                }
                TraceArrow::Return { at, motor }
                    if !self.arrows.iter().any(|effect| {
                        matches!(
                            effect,
                            TraceArrow::Effect {
                                at: effect_at,
                                motor: effect_motor,
                            } if *effect_at <= at
                                && *effect_motor == motor
                        )
                    }) =>
                {
                    return Err(TraceLawViolation::ReturnWithoutEffect { at, motor });
                }
                TraceArrow::Strengthen { at }
                    if !self.arrows.iter().any(|returned| {
                        matches!(returned, TraceArrow::Return { at: return_at, .. } if *return_at == at)
                    }) =>
                {
                    return Err(TraceLawViolation::StrengthenWithoutReturn { at });
                }
                _ => {}
            }
        }
        Ok(())
    }
}
