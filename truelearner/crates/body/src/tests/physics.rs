use super::*;

#[test]
fn native_changes_are_mapped_to_the_common_drive_scale() {
    assert_eq!(normalized_drive(0, 128, 255), 514);
    assert_eq!(normalized_drive(0, 512, 1_023), 512);
    assert_eq!(normalized_drive(-1_023, 1_023, 2_046), 1_023);
}
