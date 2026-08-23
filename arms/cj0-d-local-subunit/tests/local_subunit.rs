use cj0_d_local_subunit::{ArrowId, ArrowSpec, CellId, CellSpec, PlasticSubstrate, SpikeInput};

struct Matter {
    substrate: PlasticSubstrate,
    left: CellId,
    right: CellId,
    local: CellId,
    left_in: ArrowId,
    right_in: ArrowId,
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 1_000,
    }
}

fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
    }
}

fn matter(return_live: bool) -> Matter {
    let mut substrate = PlasticSubstrate::new();
    let left = substrate.add_cell(cell(10, -2, 0, 2));
    let right = substrate.add_cell(cell(20, 2, 0, 2));
    let local = substrate.add_cell(cell(30, 0, 0, 4));
    let outside = substrate.add_cell(cell(40, 100, 1, 1));
    let left_in = substrate.add_arrow(arrow(left, local, 1, 1, 1));
    let right_in = substrate.add_arrow(arrow(right, local, 1, 1, 1));
    let return_resistance = if return_live { 1_000 } else { 0 };
    substrate.add_arrow(arrow(local, left, 1, 1, return_resistance));
    substrate.add_arrow(arrow(local, right, 1, 1, return_resistance));
    substrate.add_arrow(arrow(local, outside, 1, 1, 1_000));
    Matter {
        substrate,
        left,
        right,
        local,
        left_in,
        right_in,
    }
}

fn enter_source(substrate: &mut PlasticSubstrate, target: CellId, tick: i64, origin: u64) {
    for serial in 0..2 {
        substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase: serial,
            origin_physical: origin + serial as u64,
            target,
            impulse: 1,
        });
    }
}

#[test]
fn weak_conjunction_returns_locally_before_cell_firing_then_reuses() {
    let mut matter = matter(true);
    enter_source(&mut matter.substrate, matter.left, 0, 100);
    enter_source(&mut matter.substrate, matter.right, 0, 200);
    let first = matter.substrate.propagate();
    assert_eq!(first.work.local_subunit_integrations, 1);
    assert_eq!(first.work.local_subunit_spikes_emitted, 2);
    assert!(!first
        .trace
        .iter()
        .any(|entry| entry.target_physical == 30 && entry.fired));
    assert!(first.crossings.is_empty());
    assert_eq!(matter.substrate.arrow_resistance(matter.left_in), 4);
    assert_eq!(matter.substrate.arrow_resistance(matter.right_in), 4);
    assert_eq!(matter.substrate.arrow_coupling(matter.left_in), 2);
    assert_eq!(matter.substrate.arrow_coupling(matter.right_in), 2);

    matter.substrate.advance_time(10);
    enter_source(&mut matter.substrate, matter.left, 10, 300);
    enter_source(&mut matter.substrate, matter.right, 10, 400);
    let second = matter.substrate.propagate();
    assert!(second
        .trace
        .iter()
        .any(|entry| entry.target_physical == 30 && entry.fired));
    assert_eq!(second.crossings.len(), 1);
    assert!(second.naturally_quiescent);
}

#[test]
fn singleton_late_same_source_and_blocked_return_do_not_form_output() {
    let mut singleton = matter(true);
    for round in 0..4 {
        let tick = round * 10;
        singleton.substrate.advance_time(tick);
        enter_source(
            &mut singleton.substrate,
            singleton.left,
            tick,
            1_000 + tick as u64,
        );
        let run = singleton.substrate.propagate();
        assert_eq!(run.work.local_subunit_integrations, 0);
        assert!(run.crossings.is_empty());
    }

    let mut late = matter(true);
    enter_source(&mut late.substrate, late.left, 0, 2_000);
    enter_source(&mut late.substrate, late.right, 1, 3_000);
    let late_run = late.substrate.propagate();
    assert_eq!(late_run.work.local_subunit_integrations, 0);
    assert!(late_run.crossings.is_empty());

    let mut same_source = matter(true);
    same_source
        .substrate
        .add_arrow(arrow(same_source.left, same_source.local, 1, 1, 1));
    enter_source(&mut same_source.substrate, same_source.left, 0, 4_000);
    let same_run = same_source.substrate.propagate();
    assert_eq!(same_run.work.local_subunit_integrations, 0);
    assert!(same_run.crossings.is_empty());

    let mut blocked = matter(false);
    for round in 0..4 {
        let tick = round * 10;
        blocked.substrate.advance_time(tick);
        enter_source(
            &mut blocked.substrate,
            blocked.left,
            tick,
            5_000 + tick as u64,
        );
        enter_source(
            &mut blocked.substrate,
            blocked.right,
            tick,
            6_000 + tick as u64,
        );
        let run = blocked.substrate.propagate();
        assert!(run.crossings.is_empty());
    }
}

#[test]
fn fully_deallocated_weak_inputs_reappear_from_ordinary_activity() {
    let mut matter = matter(true);
    matter.substrate.advance_time(20);
    assert!(!matter.substrate.arrow_is_live(matter.left_in));
    assert!(!matter.substrate.arrow_is_live(matter.right_in));

    enter_source(&mut matter.substrate, matter.left, 20, 7_000);
    enter_source(&mut matter.substrate, matter.right, 20, 8_000);
    let bootstrap = matter.substrate.propagate();
    assert_eq!(bootstrap.work.local_structural_proposals, 2);
    assert_eq!(bootstrap.work.local_subunit_integrations, 1);
    assert!(!bootstrap
        .trace
        .iter()
        .any(|entry| entry.target_physical == 30 && entry.fired));

    matter.substrate.advance_time(30);
    enter_source(&mut matter.substrate, matter.left, 30, 9_000);
    enter_source(&mut matter.substrate, matter.right, 30, 10_000);
    let learned = matter.substrate.propagate();
    assert!(learned
        .trace
        .iter()
        .any(|entry| entry.target_physical == 30 && entry.fired));
    assert_eq!(learned.crossings.len(), 1);
}
