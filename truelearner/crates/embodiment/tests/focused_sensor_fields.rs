use truelearner_embodiment::{
    interact, Availability, FocusProfile, FocusedPartition, SpatialField, SpatialFieldError,
};

fn available<T>(value: T) -> Availability<T> {
    Availability::Available(value)
}

fn field_2d(rows: usize, columns: usize) -> SpatialField<u16, 2> {
    SpatialField::new(
        [rows, columns],
        (0..rows.saturating_mul(columns))
            .map(|value| available(u16::try_from(value).unwrap()))
            .collect(),
    )
    .unwrap()
}

#[test]
fn focused_sensor_field_rejects_invalid_profiles_and_coordinates() {
    assert_eq!(
        FocusProfile::<0>::new(3, 1),
        Err(SpatialFieldError::NoDimensions)
    );
    assert_eq!(
        FocusProfile::<128>::new(1, 1),
        Err(SpatialFieldError::RegionCountOverflow)
    );

    let profile = FocusProfile::<2>::new(3, 1).unwrap();
    assert_eq!(profile.region_bound(), 25);
    assert_eq!(
        profile.focuses([8, 8], [[2, 2], [5, 5]]),
        Err(SpatialFieldError::TooManyFoci {
            maximum: 1,
            actual: 2,
        })
    );
    assert_eq!(
        profile.focuses([8, 8], [[2, 8]]),
        Err(SpatialFieldError::FocusOutsideField {
            axis: 1,
            coordinate: 8,
            extent: 8,
        })
    );
    let wrong_shape = profile.focuses([8, 8], [[2, 2]]).unwrap();
    assert_eq!(
        field_2d(4, 4).focus_partition(wrong_shape),
        Err(SpatialFieldError::FocusFieldShapeMismatch)
    );
}

#[test]
fn focused_sensor_field_is_bounded_and_reconstructs_exactly() {
    let field = field_2d(8, 8);
    let profile = FocusProfile::new(3, 1).unwrap();
    let focus = profile.focuses(field.shape(), [[3, 3]]).unwrap();
    let focused = field.clone().focus_partition(focus).unwrap();

    assert_eq!(focused.profile(), profile);
    assert_eq!(focused.foci(), &[[3, 3]]);
    assert_eq!(focused.regions().len(), 10);
    assert!(focused.regions().len() <= profile.region_bound());
    assert_eq!(focused.reassemble(), field);

    let no_focus = profile.focuses(field.shape(), []).unwrap();
    let coarse = field.clone().focus_partition(no_focus).unwrap();
    assert_eq!(coarse.regions().len(), 1);
    assert_eq!(coarse.regions()[0].origin(), [0, 0]);
    assert_eq!(coarse.regions()[0].shape(), [8, 8]);
    assert_eq!(coarse.reassemble(), field);
}

#[test]
fn focused_sensor_field_focus_union_is_commutative_and_idempotent() {
    let field = field_2d(9, 11);
    let profile = FocusProfile::new(4, 5).unwrap();
    let left = profile
        .focuses(field.shape(), [[1, 2], [7, 9], [4, 5]])
        .unwrap();
    let reversed_and_repeated = profile
        .focuses(field.shape(), [[4, 5], [7, 9], [1, 2], [7, 9], [1, 2]])
        .unwrap();

    assert_eq!(left, reversed_and_repeated);
    assert_eq!(
        field.clone().focus_partition(left).unwrap(),
        field.focus_partition(reversed_and_repeated).unwrap()
    );
}

#[test]
fn focused_sensor_field_mapping_commutes_with_focus() {
    let field = field_2d(7, 5);
    let profile = FocusProfile::new(3, 2).unwrap();
    let focus = profile.focuses(field.shape(), [[1, 1], [5, 3]]).unwrap();

    let map_then_focus = field
        .clone()
        .map(|value| value.saturating_mul(3))
        .focus_partition(focus.clone())
        .unwrap();
    let focus_then_map = field
        .focus_partition(focus)
        .unwrap()
        .map(|value| value.saturating_mul(3));

    assert_eq!(map_then_focus, focus_then_map);
}

#[test]
fn focused_sensor_field_commutes_with_a_horizontal_mirror() {
    for (shape, focus) in [([8, 10], [2, 1]), ([9, 11], [4, 3])] {
        let field = field_2d(shape[0], shape[1]);
        let mirrored = mirror_horizontal(&field);
        let mirrored_focus = [focus[0], shape[1] - focus[1] - 1];
        let profile = FocusProfile::new(3, 1).unwrap();
        let left = field
            .clone()
            .focus_partition(profile.focuses(field.shape(), [focus]).unwrap())
            .unwrap();
        let right = mirrored
            .clone()
            .focus_partition(profile.focuses(mirrored.shape(), [mirrored_focus]).unwrap())
            .unwrap();

        assert_eq!(
            mirrored_region_descriptors(&left),
            region_descriptors(&right)
        );
        assert_eq!(left.reassemble(), field);
        assert_eq!(right.reassemble(), mirrored);
    }
}

#[test]
fn focused_sensor_field_handles_identity_and_value_blind_controls() {
    let field = field_2d(7, 9);
    let five_focus_profile = FocusProfile::new(4, 5).unwrap();
    let five_foci = [[0, 0], [0, 8], [3, 4], [6, 0], [6, 8]];
    let focused = field
        .clone()
        .focus_partition(
            five_focus_profile
                .focuses(field.shape(), five_foci)
                .unwrap(),
        )
        .unwrap();
    assert!(focused.regions().len() <= five_focus_profile.region_bound());
    assert_eq!(focused.clone().reassemble(), field);

    let uniform =
        SpatialField::new(field.shape(), vec![available(7_u16); field.cells().len()]).unwrap();
    let uniform_focused = uniform
        .focus_partition(
            five_focus_profile
                .focuses(field.shape(), five_foci)
                .unwrap(),
        )
        .unwrap();
    assert_eq!(
        region_descriptors(&focused),
        region_descriptors(&uniform_focused)
    );

    let zero_depth = FocusProfile::new(0, 1).unwrap();
    let unchanged = field
        .clone()
        .focus_partition(zero_depth.focuses(field.shape(), [[3, 4]]).unwrap())
        .unwrap();
    assert_eq!(zero_depth.region_bound(), 1);
    assert_eq!(unchanged.regions().len(), 1);
    assert_eq!(unchanged.reassemble(), field);

    let unit = SpatialField::new([1, 1], vec![available(42_u16)]).unwrap();
    let deep = FocusProfile::new(64, 1).unwrap();
    let unit_focused = unit
        .clone()
        .focus_partition(deep.focuses(unit.shape(), [[0, 0]]).unwrap())
        .unwrap();
    assert_eq!(unit_focused.regions().len(), 1);
    assert_eq!(unit_focused.reassemble(), unit);
}

#[test]
fn focused_sensor_field_transduction_is_complete_and_refinement_consistent() {
    let field = SpatialField::new([4, 4], (1_u16..=16).map(available).collect::<Vec<_>>()).unwrap();
    let profile = FocusProfile::new(2, 1).unwrap();
    let coarse = field
        .clone()
        .focus_partition(profile.focuses(field.shape(), []).unwrap())
        .unwrap();
    let refined = field
        .clone()
        .focus_partition(profile.focuses(field.shape(), [[1, 1]]).unwrap())
        .unwrap();
    let coarse_values = coarse.transduce_complete(0_u64, u64::from, u64::saturating_add);
    let refined_values = refined.transduce_complete(0_u64, u64::from, u64::saturating_add);

    assert_eq!(coarse_values.regions().len(), 1);
    assert_eq!(coarse_values.regions()[0].value(), &available(136));
    assert_eq!(sum_available(&refined_values), 136);

    let unavailable = SpatialField::new(
        [2, 2],
        vec![
            available(3_u16),
            Availability::Unavailable,
            available(5),
            available(7),
        ],
    )
    .unwrap();
    let focused = unavailable
        .focus_partition(profile.focuses([2, 2], [[0, 0]]).unwrap())
        .unwrap();
    let values = focused.transduce_complete(0_u64, u64::from, u64::saturating_add);
    assert!(values
        .regions()
        .iter()
        .any(|region| region.value() == &Availability::Unavailable));
    assert!(values
        .regions()
        .iter()
        .any(|region| region.value().is_available()));
}

#[test]
fn focused_sensor_field_observes_only_actual_post_effect_focus() {
    #[derive(Clone)]
    struct SensorState {
        field: SpatialField<u8, 1>,
        focus: [usize; 1],
    }

    let profile = FocusProfile::new(3, 1).unwrap();
    let mut state = SensorState {
        field: SpatialField::new(
            [8],
            [3_u8, 5, 8, 13, 21, 34, 55, 89]
                .into_iter()
                .map(available)
                .collect(),
        )
        .unwrap(),
        focus: [1],
    };
    let observation = interact(
        &mut state,
        |state| {
            state
                .field
                .clone()
                .focus_partition(profile.focuses(state.field.shape(), [state.focus]).unwrap())
                .unwrap()
        },
        |_| Some([6]),
        |state, focus| state.focus = focus,
    );

    assert_eq!(observation.effect, Some(()));
    assert_ne!(observation.before, observation.after);
    assert_eq!(state.focus, [6]);
    assert_eq!(observation.before.foci(), &[[1]]);
    assert_eq!(observation.after.foci(), &[[6]]);

    let repeated = interact(
        &mut state,
        |state| {
            state
                .field
                .clone()
                .focus_partition(profile.focuses(state.field.shape(), [state.focus]).unwrap())
                .unwrap()
        },
        |_| None::<[usize; 1]>,
        |state, focus| state.focus = focus,
    );
    assert_eq!(repeated.effect, None);
    assert_eq!(repeated.before, repeated.after);
    assert_eq!(state.focus, [6]);
}

#[test]
fn focused_sensor_field_transfers_across_dimensions_and_modalities() {
    let spectrum = SpatialField::new([9], (0_u16..9).map(available).collect::<Vec<_>>()).unwrap();
    let intensity = field_2d(7, 5);
    let depth = SpatialField::new(
        [3, 3],
        vec![
            available(700_u16),
            available(710),
            Availability::Unavailable,
            available(900),
            available(910),
            available(920),
            available(1100),
            available(1110),
            available(1120),
        ],
    )
    .unwrap();
    let touch = SpatialField::new(
        [3, 2],
        [false, true, false, false, true, true]
            .into_iter()
            .map(available)
            .collect(),
    )
    .unwrap();
    let volume =
        SpatialField::new([3, 5, 7], (0_i16..105).map(available).collect::<Vec<_>>()).unwrap();

    assert_round_trip(spectrum, FocusProfile::new(4, 1).unwrap(), [[4]]);
    assert_round_trip(intensity, FocusProfile::new(3, 1).unwrap(), [[2, 3]]);
    assert_round_trip(depth, FocusProfile::new(3, 1).unwrap(), [[1, 1]]);
    assert_round_trip(touch, FocusProfile::new(2, 1).unwrap(), [[1, 0]]);
    assert_round_trip(
        volume,
        FocusProfile::new(4, 2).unwrap(),
        [[1, 1, 1], [2, 4, 6]],
    );
}

fn assert_round_trip<T: Clone + PartialEq + std::fmt::Debug, const D: usize, const N: usize>(
    field: SpatialField<T, D>,
    profile: FocusProfile<D>,
    foci: [[usize; D]; N],
) {
    let focus = profile.focuses(field.shape(), foci).unwrap();
    let partition = field.clone().focus_partition(focus).unwrap();
    assert!(partition.regions().len() <= profile.region_bound());
    assert_eq!(partition.reassemble(), field);
}

fn sum_available<const D: usize>(field: &truelearner_embodiment::FocusedField<u64, D>) -> u64 {
    field
        .regions()
        .iter()
        .filter_map(|region| match region.value() {
            Availability::Available(value) => Some(*value),
            Availability::Unavailable => None,
        })
        .sum()
}

fn mirror_horizontal(field: &SpatialField<u16, 2>) -> SpatialField<u16, 2> {
    let [rows, columns] = field.shape();
    let mut cells = Vec::with_capacity(field.cells().len());
    for row in 0..rows {
        for column in (0..columns).rev() {
            cells.push(field.cells()[row * columns + column]);
        }
    }
    SpatialField::new([rows, columns], cells).unwrap()
}

fn region_descriptors<T>(partition: &FocusedPartition<T, 2>) -> Vec<([usize; 2], [usize; 2])> {
    let mut descriptors = partition
        .regions()
        .iter()
        .map(|region| (region.origin(), region.shape()))
        .collect::<Vec<_>>();
    descriptors.sort_unstable();
    descriptors
}

fn mirrored_region_descriptors<T>(
    partition: &FocusedPartition<T, 2>,
) -> Vec<([usize; 2], [usize; 2])> {
    let columns = partition.original_shape()[1];
    let mut descriptors = partition
        .regions()
        .iter()
        .map(|region| {
            let origin = region.origin();
            let shape = region.shape();
            ([origin[0], columns - origin[1] - shape[1]], shape)
        })
        .collect::<Vec<_>>();
    descriptors.sort_unstable();
    descriptors
}
