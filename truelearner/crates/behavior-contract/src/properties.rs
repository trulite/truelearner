use crate::{Scenario, Sensor, SensorId, Step};

pub fn with_cause(mut scenario: Scenario, cause: u64) -> Scenario {
    for step in &mut scenario.steps {
        if let Step::Run(episode) = step {
            for input in &mut episode.inputs {
                input.cause = cause;
            }
            for effect in &mut episode.expected.effects {
                effect.cause = cause;
            }
        }
    }
    scenario
}

pub fn reversed_construction(mut scenario: Scenario) -> Scenario {
    scenario.morphology.sensors.reverse();
    scenario.morphology.motors.reverse();
    scenario.morphology.nearby.reverse();
    scenario.morphology.outcome_components.reverse();
    for component in &mut scenario.morphology.outcome_components {
        component.motors.reverse();
    }
    scenario
}

pub fn with_dormant_sensors(mut scenario: Scenario, count: u16) -> Scenario {
    let start = scenario
        .morphology
        .sensors
        .iter()
        .map(|sensor| sensor.id.0)
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    scenario
        .morphology
        .sensors
        .extend((0..count).map(|offset| Sensor {
            id: SensorId(start.saturating_add(offset)),
            retention: crate::Retention::Integrating { threshold: 1 },
        }));
    scenario
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenarios;

    #[test]
    fn deterministic_variants_preserve_named_physical_references() {
        let original = scenarios::local_action(1, 7);
        let reversed = reversed_construction(original.clone());
        assert_eq!(original.name, reversed.name);
        assert_eq!(original.steps, reversed.steps);
        assert_eq!(
            original.morphology.sensors.len(),
            reversed.morphology.sensors.len()
        );

        let dormant = with_dormant_sensors(original.clone(), 16);
        assert_eq!(
            dormant.morphology.sensors.len(),
            original.morphology.sensors.len() + 16
        );
        dormant.validate().unwrap();

        let changed = with_cause(original, 91);
        let Step::Run(episode) = &changed.steps[0] else {
            unreachable!()
        };
        assert!(episode.inputs.iter().all(|input| input.cause == 91));
        assert!(episode
            .expected
            .effects
            .iter()
            .all(|effect| effect.cause == 91));
    }
}
