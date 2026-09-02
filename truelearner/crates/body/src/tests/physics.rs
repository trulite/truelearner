use super::*;

#[test]
fn native_changes_are_mapped_to_the_common_drive_scale() {
    assert_eq!(normalized_drive(0, 128, 255), 514);
    assert_eq!(normalized_drive(0, 512, 1_023), 512);
    assert_eq!(normalized_drive(-1_023, 1_023, 2_046), 1_023);
}

#[test]
fn nearby_activity_integrates_without_an_episode_label() {
    let mut junction = JunctionSlot::new(Junction::integrating(10));

    assert_eq!(junction.change(10, 5), None);
    assert_eq!(junction.change(12, 7), Some((5, 12)));
}

#[test]
fn old_activity_cannot_be_joined_by_a_distant_event() {
    let mut junction = JunctionSlot::new(Junction::integrating(10));

    assert_eq!(junction.change(10, 5), None);
    assert_eq!(junction.change(20, 7), None);
    assert_eq!(junction.held(), 7);
}
