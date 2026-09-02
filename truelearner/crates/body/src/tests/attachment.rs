use super::*;
use crate::{ArrowState, Junction, Path};

#[test]
fn attachment_preserves_live_returns_when_rebuilding_the_index() {
    let mut host = Body::default();
    let host_junction = host.add_junction(Junction::integrating(1)).unwrap();
    let host_link = host
        .add_link(Link::new(host_junction, host_junction, 0, 0))
        .unwrap();
    let host_path = Path {
        surface: host_junction,
        middle: host_junction,
        output: host_junction,
        first: host_link,
        second: host_link,
    };
    host.replace_arrow_state(host_link, ArrowState::open_return(host_path, 8))
        .unwrap();

    let mut part = Body::default();
    let part_junction = part.add_junction(Junction::integrating(1)).unwrap();
    let part_link = part
        .add_link(Link::new(part_junction, part_junction, 0, 0))
        .unwrap();
    let part_path = Path {
        surface: part_junction,
        middle: part_junction,
        output: part_junction,
        first: part_link,
        second: part_link,
    };
    part.replace_arrow_state(part_link, ArrowState::open_return(part_path, 3))
        .unwrap();
    let part = OpenBody::new(part, vec![part_junction]).unwrap();

    attach(&mut host, part, &[]).unwrap();

    assert_eq!(host.returns.live_count, 2);
    assert!(host.returns.by_source.iter().all(Vec::is_empty));
}
