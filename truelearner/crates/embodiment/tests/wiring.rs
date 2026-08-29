use truelearner_core::{HarnessBuilder, Junction, Link, Protocol, TransmissionMode};
use truelearner_embodiment::{DriveSpec, JunctionSpec, Wiring};

fn direct_junction(physical_id: u64, position: i32, region: i16, threshold: i32) -> Junction {
    Junction {
        physical_id,
        position,
        region,
        threshold,
        resistance: u32::MAX,
    }
}

#[test]
fn wiring_preserves_direct_physical_construction() {
    let mut direct = HarnessBuilder::with_capacity(16, 16, 1);
    direct.set_protocol(Protocol::RecursiveLearnerCausalTopologyProductComposition);
    direct.set_physical_tracing(true);
    let direct_output = direct.add_junction(direct_junction(20, 4, 0, 2));
    let direct_sink = direct.add_junction(direct_junction(30, 4, 1, 1));
    direct.add_link(Link {
        from: direct_output,
        to: direct_sink,
        delay: 0,
        phase: 0,
        coupling: 1,
        resistance: u32::MAX,
        mode: TransmissionMode::Drive,
    });
    let direct_outcome = direct.add_junction(direct_junction(40, 12, 0, 1));
    direct.set_outcome_source_for_output(direct_output, direct_outcome);

    let mut composed = HarnessBuilder::with_capacity(16, 16, 1);
    composed.set_protocol(Protocol::RecursiveLearnerCausalTopologyProductComposition);
    composed.set_physical_tracing(true);
    {
        let mut wiring = Wiring::new(&mut composed);
        let output = wiring.junction(JunctionSpec::ordinary(20, 4, 0, 2));
        let sink = wiring.junction(JunctionSpec::ordinary(30, 4, 1, 1));
        wiring.drive(output, sink, DriveSpec::ordinary(1));
        let outcome = wiring.junction(JunctionSpec::ordinary(40, 12, 0, 1));
        wiring.bind_output(output, outcome);
    }

    assert_eq!(direct.build().read(), composed.build().read());
}

#[test]
fn wiring_one_source_fans_out_without_duplication() {
    let mut builder = HarnessBuilder::with_capacity(3, 2, 0);
    let (source, left, right) = {
        let mut wiring = Wiring::new(&mut builder);
        let source = wiring.junction(JunctionSpec::ordinary(10, 0, 0, 1));
        let left = wiring.junction(JunctionSpec::ordinary(11, -1, 1, 1));
        let right = wiring.junction(JunctionSpec::ordinary(12, 1, 1, 1));
        wiring.drive(source, left, DriveSpec::ordinary(1));
        wiring.drive(source, right, DriveSpec::ordinary(1));
        (source, left, right)
    };
    let read = builder.build().read();

    assert_eq!(read.junctions.len(), 3);
    assert_eq!(read.links.len(), 2);
    assert_eq!((read.links[0].from, read.links[0].to), (source, left));
    assert_eq!((read.links[1].from, read.links[1].to), (source, right));
}

#[test]
fn wiring_receptor_and_actuator_banks_preserve_declared_order() {
    let mut builder = HarnessBuilder::with_capacity(32, 32, 1);
    let (actuators, receptors) = {
        let mut wiring = Wiring::new(&mut builder);
        let actuators = wiring.actuator_bank(
            2,
            100,
            200,
            |index, physical| JunctionSpec::ordinary(physical, index as i32, 0, 2),
            |index, physical| JunctionSpec::ordinary(physical, index as i32, 1, 1),
            DriveSpec::ordinary(1),
        );
        let receptors = wiring.receptor_bank::<4>(2, 300, |feature, _, physical| {
            JunctionSpec::ordinary(physical, 10 + feature as i32, 0, 1)
        });
        (actuators, receptors)
    };
    let read = builder.build().read();

    assert_eq!(
        actuators.iter().map(|id| id.0).collect::<Vec<_>>(),
        vec![0, 2]
    );
    assert_eq!(receptors[0].map(|id| id.0), [4, 5, 6, 7]);
    assert_eq!(receptors[1].map(|id| id.0), [8, 9, 10, 11]);
    assert_eq!(
        read.junctions
            .iter()
            .map(|junction| junction.physical_id)
            .collect::<Vec<_>>(),
        vec![100, 200, 101, 201, 300, 301, 302, 303, 304, 305, 306, 307]
    );
}

#[test]
fn wiring_empty_banks_are_identity() {
    let mut builder = HarnessBuilder::with_capacity(0, 0, 0);
    {
        let mut wiring = Wiring::new(&mut builder);
        let actuators = wiring.actuator_bank(
            0,
            100,
            200,
            |index, physical| JunctionSpec::ordinary(physical, index as i32, 0, 2),
            |index, physical| JunctionSpec::ordinary(physical, index as i32, 1, 1),
            DriveSpec::ordinary(1),
        );
        let receptors = wiring.receptor_bank::<4>(0, 300, |feature, _, physical| {
            JunctionSpec::ordinary(physical, 10 + feature as i32, 0, 1)
        });
        assert!(actuators.is_empty());
        assert!(receptors.is_empty());
    }
    let read = builder.build().read();
    assert!(read.junctions.is_empty());
    assert!(read.links.is_empty());
}
