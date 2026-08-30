#![deny(warnings)]

#[path = "behavior_contract/catalog.rs"]
mod catalog;
#[path = "behavior_contract/legacy.rs"]
mod legacy;

use legacy::{LegacyAdapter, LegacyProfile};
use truelearner_behavior_contract::{properties, run_scenario, scenarios};

#[test]
fn fixed_black_box_scenarios_run_on_the_old_harness() {
    let adapter = LegacyAdapter::new(catalog::CORE_STORY, true);
    for scenario in [
        scenarios::quiet(),
        scenarios::local_action(1, 7),
        scenarios::no_local_action(3, 8),
        scenarios::learns_and_reuses(1, 9),
        scenarios::checkpoint_replay(1, 10),
    ] {
        run_scenario(&adapter, &scenario).unwrap_or_else(|error| {
            panic!("{} failed through legacy adapter: {error:?}", scenario.name)
        });
    }
}

#[test]
fn deterministic_property_variants_run_on_the_old_harness() {
    let adapter = LegacyAdapter::new(catalog::CORE_STORY, true);
    let base = scenarios::local_action(1, 7);
    let cases = [
        properties::with_cause(base.clone(), 11),
        properties::with_cause(base.clone(), 99),
        properties::reversed_construction(base.clone()),
        properties::with_dormant_sensors(base.clone(), 32),
        properties::with_dormant_sensors(base, 1_024),
        scenarios::no_local_action(4, 13),
    ];
    for (case, scenario) in cases.iter().enumerate() {
        run_scenario(&adapter, scenario).unwrap_or_else(|error| {
            panic!(
                "property case {case} ({}) failed through legacy adapter: {error:?}",
                scenario.name
            )
        });
    }
}

#[test]
fn old_harness_observation_is_behaviorally_inert() {
    let scenario = scenarios::local_action(1, 23);
    let traced = run_scenario(
        &LegacyAdapter::new(LegacyProfile::Physical, true),
        &scenario,
    )
    .unwrap();
    let silent = run_scenario(
        &LegacyAdapter::new(LegacyProfile::Physical, false),
        &scenario,
    )
    .unwrap();
    assert_eq!(traced, silent);
}
