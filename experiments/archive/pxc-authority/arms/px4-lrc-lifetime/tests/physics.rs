use lr1_modulatory_physical_return::TransmissionMode;
use px4_lrc_lifetime::{arrive, field};

#[test]
fn qualified_flow_changes_only_a_traversed_candidate() {
    let mut world = field(700_001, false, false, TransmissionMode::Modulatory);
    assert_eq!(world.space.arrow_count(), 1);
    arrive(&mut world.space, world.source, 0, 1, world.mark + 1_000);
    arrive(&mut world.space, world.returner, 2, 2, world.mark + 2_000);
    let flow = world.space.propagate();
    let candidates = world.space.arrows_between(world.source, world.effect);
    assert!(flow.naturally_quiescent);
    assert_eq!(candidates.len(), 1);
    assert_eq!(world.space.arrow_resistance(candidates[0]), 4);
    assert_eq!(world.space.arrow_coupling(candidates[0]), 2);
}

#[test]
fn unsupported_use_is_spent_by_the_retained_pressure_path() {
    let mut world = field(700_101, true, true, TransmissionMode::Modulatory);
    arrive(&mut world.space, world.source, 0, 1, world.mark + 1_000);
    assert!(world.space.propagate().naturally_quiescent);
    let candidate = world.space.arrows_between(world.source, world.effect)[0];
    assert!(world.space.arrow_is_live(candidate));
    let work = world.space.advance_time(5);
    assert!(!world.space.arrow_is_live(candidate));
    assert_eq!(work.physical_deallocations, 1);
}
