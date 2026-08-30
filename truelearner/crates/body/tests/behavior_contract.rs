#![deny(warnings)]

#[path = "behavior_contract/new_body.rs"]
mod new_body;

use new_body::NewBodyAdapter;
use truelearner_behavior_contract::{properties, run_scenario, scenarios, Scenario};

fn assert_scenario(scenario: Scenario) {
    run_scenario(&NewBodyAdapter, &scenario)
        .unwrap_or_else(|error| panic!("{} failed on compact body: {error:?}", scenario.name));
}

#[test]
fn quiet_is_the_same_on_the_compact_body() {
    assert_scenario(scenarios::quiet());
}

#[test]
fn local_action_is_the_same_on_the_compact_body() {
    assert_scenario(scenarios::local_action(1, 7));
}

#[test]
fn distant_action_is_rejected_on_the_compact_body() {
    assert_scenario(scenarios::no_local_action(3, 8));
}

#[test]
fn learned_action_is_reused_on_the_compact_body() {
    assert_scenario(scenarios::learns_and_reuses(1, 9));
}

#[test]
fn learned_action_replays_after_checkpoint_on_the_compact_body() {
    assert_scenario(scenarios::checkpoint_replay(1, 10));
}

#[test]
fn deterministic_variants_are_the_same_on_the_compact_body() {
    let base = scenarios::local_action(1, 7);
    let failures = [
        properties::with_cause(base.clone(), 11),
        properties::with_cause(base.clone(), 99),
        properties::reversed_construction(base.clone()),
        properties::with_dormant_sensors(base.clone(), 32),
        properties::with_dormant_sensors(base, 1_024),
        scenarios::no_local_action(4, 13),
    ]
    .into_iter()
    .enumerate()
    .filter_map(|(case, scenario)| {
        run_scenario(&NewBodyAdapter, &scenario).err().map(|error| {
            format!(
                "property case {case} ({}) failed on compact body: {error:?}",
                scenario.name
            )
        })
    })
    .collect::<Vec<_>>();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
