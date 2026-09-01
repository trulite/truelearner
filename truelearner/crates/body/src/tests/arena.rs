use super::*;
use std::mem::size_of;

#[test]
fn propagation_slots_are_compact() {
    eprintln!(
        "junction_slot={} junction_spec={} retention={} option_time={} option_link={} link_slot={} link_spec={} trigger={}",
        size_of::<JunctionSlot>(),
        size_of::<Junction>(),
        size_of::<Retention>(),
        size_of::<Option<Time>>(),
        size_of::<Option<LinkId>>(),
        size_of::<LinkSlot>(),
        size_of::<Link>(),
        size_of::<Trigger>()
    );
    assert_eq!(size_of::<JunctionSlot>(), 32);
    assert_eq!(size_of::<LinkSlot>(), 32);
    assert_eq!(std::mem::align_of::<JunctionSlot>(), 32);
    assert_eq!(std::mem::align_of::<LinkSlot>(), 32);
    assert_eq!(size_of::<Option<JunctionId>>(), size_of::<JunctionId>());
    assert_eq!(size_of::<Option<LinkId>>(), size_of::<LinkId>());
}

#[test]
fn incoming_incidence_preserves_source_and_construction_order() {
    let mut arena = Arena::default();
    let left = arena.add_junction(Junction::integrating(1)).unwrap();
    let right = arena.add_junction(Junction::integrating(1)).unwrap();
    let target = arena.add_junction(Junction::integrating(1)).unwrap();
    let first = arena.add_link(Link::new(left, target, 1, 2)).unwrap();
    let second = arena.add_link(Link::new(right, target, 3, 4)).unwrap();

    assert_eq!(arena.incoming(target).collect::<Vec<_>>(), [first, second]);
    assert_eq!(arena.link(first).unwrap().from, left);
    assert_eq!(arena.link(second).unwrap().from, right);
    assert!(arena.incoming_is_consistent());
}

#[test]
fn append_remaps_both_ends_and_incoming_incidence() {
    let mut host = Arena::default();
    host.add_junction(Junction::integrating(1)).unwrap();

    let mut part = Arena::default();
    let left = part.add_junction(Junction::integrating(1)).unwrap();
    let right = part.add_junction(Junction::integrating(1)).unwrap();
    let target = part.add_junction(Junction::integrating(1)).unwrap();
    let first = part.add_link(Link::new(left, target, 1, 1)).unwrap();
    let second = part.add_link(Link::new(right, target, 1, 1)).unwrap();

    let (junction_base, link_base) = host.append(part);
    let left = remap_junction(left, junction_base);
    let right = remap_junction(right, junction_base);
    let target = remap_junction(target, junction_base);
    let first = remap_link(first, link_base);
    let second = remap_link(second, link_base);

    assert_eq!(host.incoming(target).collect::<Vec<_>>(), [first, second]);
    assert_eq!(host.link(first).unwrap().from, left);
    assert_eq!(host.link(second).unwrap().from, right);
    assert!(host.incoming_is_consistent());
}
