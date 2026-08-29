use truelearner_embodiment::{
    Availability, ChangeDetector, Driver, Incidence, LocalBlockShape, Origin, Signal, SpatialField,
    SpatialFieldError,
};

fn available<T>(value: T) -> Availability<T> {
    Availability::Available(value)
}

#[test]
fn spatial_field_rejects_invalid_shape_and_cell_count() {
    assert_eq!(
        SpatialField::<u8, 0>::new([], vec![available(1)]),
        Err(SpatialFieldError::NoDimensions)
    );
    assert_eq!(
        LocalBlockShape::<0>::new([]),
        Err(SpatialFieldError::NoDimensions)
    );
    assert_eq!(
        SpatialField::<u8, 2>::new([2, 0], Vec::new()),
        Err(SpatialFieldError::ZeroDimension { axis: 1 })
    );
    assert_eq!(
        SpatialField::<u8, 2>::new([usize::MAX, 2], Vec::new()),
        Err(SpatialFieldError::CellCountOverflow)
    );
    assert_eq!(
        SpatialField::<u8, 2>::new([2, 2], vec![available(1); 3]),
        Err(SpatialFieldError::WrongCellCount {
            expected: 4,
            actual: 3,
        })
    );
    assert_eq!(
        LocalBlockShape::<2>::new([8, 0]),
        Err(SpatialFieldError::ZeroBlockDimension { axis: 1 })
    );
}

#[test]
fn spatial_field_factorization_is_lossless_and_local_at_edges() {
    let field = SpatialField::new([5, 7], (0_u8..35).map(available).collect::<Vec<_>>()).unwrap();
    let blocks = field
        .clone()
        .factor_local(LocalBlockShape::new([3, 4]).unwrap());

    assert_eq!(blocks.original_shape(), [5, 7]);
    assert_eq!(blocks.block_shape(), [3, 4]);
    assert_eq!(blocks.blocks().len(), 4);
    assert_eq!(blocks.blocks()[0].origin(), [0, 0]);
    assert_eq!(blocks.blocks()[0].shape(), [3, 4]);
    assert_eq!(blocks.blocks()[1].origin(), [0, 4]);
    assert_eq!(blocks.blocks()[1].shape(), [3, 3]);
    assert_eq!(blocks.blocks()[2].origin(), [3, 0]);
    assert_eq!(blocks.blocks()[2].shape(), [2, 4]);
    assert_eq!(blocks.blocks()[3].origin(), [3, 4]);
    assert_eq!(blocks.blocks()[3].shape(), [2, 3]);
    assert_eq!(blocks.reassemble(), field);
}

#[test]
fn spatial_field_mapping_commutes_with_local_factorization() {
    let field = SpatialField::new(
        [3, 4],
        vec![
            available(1_u16),
            Availability::Unavailable,
            available(3),
            available(4),
            available(5),
            available(6),
            available(7),
            available(8),
            available(9),
            available(10),
            available(11),
            available(12),
        ],
    )
    .unwrap();
    let block_shape = LocalBlockShape::new([2, 3]).unwrap();

    let mapped_then_factored = field
        .clone()
        .map(|value| value.saturating_mul(2))
        .factor_local(block_shape);
    let factored_then_mapped = field
        .clone()
        .factor_local(block_shape)
        .map(|value| value.saturating_mul(2));

    assert_eq!(mapped_then_factored, factored_then_mapped);
    assert_eq!(
        factored_then_mapped.clone().map(|value| value),
        factored_then_mapped
    );
    assert_eq!(
        field
            .clone()
            .map(|value| value.saturating_mul(2))
            .map(|value| value.saturating_add(3)),
        field.map(|value| value.saturating_mul(2).saturating_add(3))
    );
}

#[test]
fn spatial_field_preserves_arrangement_and_repeated_sample() {
    let left = SpatialField::new(
        [2, 2],
        vec![available(1_u8), available(2), available(1), available(2)],
    )
    .unwrap();
    let right = SpatialField::new(
        [2, 2],
        vec![available(1_u8), available(1), available(2), available(2)],
    )
    .unwrap();
    assert_ne!(left, right);

    let mut detector = ChangeDetector::default();
    let first = detector.step(Signal::new(Origin(90), Incidence::Sample, left.clone()));
    let repeated = detector.step(Signal::new(Origin(90), Incidence::Sample, left));
    let rearranged = detector.step(Signal::new(Origin(90), Incidence::Sample, right));

    assert_eq!(first.incidence, Incidence::Sample);
    assert_eq!(repeated.incidence, Incidence::Sample);
    assert_eq!(rearranged.incidence, Incidence::Transition);
}

#[test]
fn spatial_field_transfers_across_value_and_dimension() {
    let intensity = SpatialField::new(
        [2, 3],
        vec![
            available(12_u8),
            available(40),
            available(90),
            available(12),
            available(40),
            available(90),
        ],
    )
    .unwrap();
    let depth = SpatialField::new(
        [2, 2],
        vec![
            available(800_u16),
            Availability::Unavailable,
            available(1200),
            available(1400),
        ],
    )
    .unwrap();
    let touch = SpatialField::new(
        [2, 2],
        vec![
            available(false),
            available(true),
            available(true),
            available(false),
        ],
    )
    .unwrap();
    let volume =
        SpatialField::new([2, 2, 2], (0_i16..8).map(available).collect::<Vec<_>>()).unwrap();

    assert_eq!(
        intensity
            .clone()
            .factor_local(LocalBlockShape::new([1, 2]).unwrap())
            .reassemble(),
        intensity
    );
    assert_eq!(
        depth
            .clone()
            .factor_local(LocalBlockShape::new([2, 1]).unwrap())
            .reassemble(),
        depth
    );
    assert_eq!(
        touch
            .clone()
            .factor_local(LocalBlockShape::new([1, 1]).unwrap())
            .reassemble(),
        touch
    );
    assert_eq!(
        volume
            .clone()
            .factor_local(LocalBlockShape::new([2, 1, 2]).unwrap())
            .reassemble(),
        volume
    );
}
