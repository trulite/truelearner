use truelearner_embodiment::{Availability, FocusProfile, FocusedField, SpatialField};

fn available<T>(value: T) -> Availability<T> {
    Availability::Available(value)
}

fn focused_sum<const D: usize>(
    field: SpatialField<u16, D>,
    profile: FocusProfile<D>,
    foci: impl IntoIterator<Item = [usize; D]>,
) -> FocusedField<u64, D> {
    let focus = profile.focuses(field.shape(), foci).unwrap();
    field
        .focus_partition(focus)
        .unwrap()
        .transduce_complete(0_u64, u64::from, u64::saturating_add)
}

#[test]
fn focused_receptor_frame_has_fixed_arity_and_unavailable_padding() {
    let field = SpatialField::new([8, 8], (1_u16..=64).map(available).collect::<Vec<_>>()).unwrap();
    let profile = FocusProfile::new(3, 1).unwrap();
    let focused = focused_sum(field, profile, [[3, 3]]);
    let expected = focused
        .regions()
        .iter()
        .map(|region| *region.value())
        .collect::<Vec<_>>();
    let frame = focused.into_receptor_frame();

    assert_eq!(frame.original_shape(), [8, 8]);
    assert_eq!(frame.profile(), profile);
    assert_eq!(frame.foci(), &[[3, 3]]);
    assert_eq!(frame.active_region_count(), expected.len());
    assert_eq!(frame.slots().len(), profile.region_bound());
    assert_eq!(&frame.slots()[..expected.len()], expected);
    assert!(frame.slots()[expected.len()..]
        .iter()
        .all(|slot| slot == &Availability::Unavailable));
}

#[test]
fn focused_receptor_frame_mapping_commutes_with_framing() {
    let field = SpatialField::new([7, 5], (0_u16..35).map(available).collect::<Vec<_>>()).unwrap();
    let profile = FocusProfile::new(3, 2).unwrap();
    let focused = focused_sum(field, profile, [[1, 1], [5, 3]]);

    let focus_then_map = focused
        .clone()
        .into_receptor_frame()
        .map(|value| value.saturating_mul(3));
    let map_then_frame = focused
        .map(|value| value.saturating_mul(3))
        .into_receptor_frame();

    assert_eq!(focus_then_map, map_then_frame);
}

#[test]
fn focused_receptor_frame_repeats_exactly_and_observes_focus_movement() {
    let field = SpatialField::new([9, 11], (0_u16..99).map(available).collect::<Vec<_>>()).unwrap();
    let profile = FocusProfile::new(4, 1).unwrap();
    let first = focused_sum(field.clone(), profile, [[1, 2]]).into_receptor_frame();
    let repeated = focused_sum(field.clone(), profile, [[1, 2]]).into_receptor_frame();
    let moved = focused_sum(field, profile, [[7, 9]]).into_receptor_frame();

    assert_eq!(first, repeated);
    assert_eq!(first.slots().len(), moved.slots().len());
    assert_ne!(first.foci(), moved.foci());
    assert_ne!(first.slots(), moved.slots());
}

#[test]
fn focused_receptor_frames_compose_independently() {
    let profile = FocusProfile::new(3, 1).unwrap();
    let left = SpatialField::new([5, 7], (0_u16..35).map(available).collect::<Vec<_>>()).unwrap();
    let right =
        SpatialField::new([5, 7], (100_u16..135).map(available).collect::<Vec<_>>()).unwrap();
    let left_frame = focused_sum(left, profile, [[1, 2]]).into_receptor_frame();
    let right_frame = focused_sum(right, profile, [[3, 4]]).into_receptor_frame();

    assert_eq!(left_frame.slots().len(), right_frame.slots().len());
    assert_ne!(left_frame.slots(), right_frame.slots());
    assert_eq!(left_frame.foci(), &[[1, 2]]);
    assert_eq!(right_frame.foci(), &[[3, 4]]);
}

#[test]
fn focused_receptor_frame_transfers_across_dimensions_and_controls() {
    let spectrum = SpatialField::new([9], (0_u16..9).map(available).collect::<Vec<_>>()).unwrap();
    let volume =
        SpatialField::new([3, 5, 7], (0_u16..105).map(available).collect::<Vec<_>>()).unwrap();
    let unavailable = SpatialField::new([3, 3], vec![Availability::<u16>::Unavailable; 9]).unwrap();

    let spectrum_profile = FocusProfile::new(4, 1).unwrap();
    let spectrum_frame = focused_sum(spectrum, spectrum_profile, [[8]]).into_receptor_frame();
    assert_eq!(
        spectrum_frame.slots().len(),
        spectrum_profile.region_bound()
    );

    let volume_profile = FocusProfile::new(4, 5).unwrap();
    let volume_frame = focused_sum(
        volume,
        volume_profile,
        [[0, 0, 0], [0, 4, 6], [1, 2, 3], [2, 0, 6], [2, 4, 0]],
    )
    .into_receptor_frame();
    assert_eq!(volume_frame.slots().len(), volume_profile.region_bound());
    assert!(volume_frame.active_region_count() <= volume_profile.region_bound());

    let zero_depth = FocusProfile::new(0, 1).unwrap();
    let unavailable_frame = focused_sum(unavailable, zero_depth, [[1, 1]]).into_receptor_frame();
    assert_eq!(unavailable_frame.active_region_count(), 1);
    assert_eq!(unavailable_frame.slots(), &[Availability::Unavailable]);
}
