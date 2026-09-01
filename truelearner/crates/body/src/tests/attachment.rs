use super::*;
use crate::{Junction, LinkRole};

#[test]
fn attachment_preserves_live_returns_when_rebuilding_the_index() {
    let mut host = Body::default();
    let host_junction = host.add_junction(Junction::integrating(1)).unwrap();
    let host_link = host
        .add_link(Link::new(host_junction, host_junction, 0, 0))
        .unwrap();
    host.set_link_role(
        host_link,
        LinkRole::Return {
            cause: 8,
            cohort: 8,
        },
    )
    .unwrap();

    let mut part = Body::default();
    let part_junction = part.add_junction(Junction::integrating(1)).unwrap();
    let part_link = part
        .add_link(Link::new(part_junction, part_junction, 0, 0))
        .unwrap();
    part.set_link_role(
        part_link,
        LinkRole::Return {
            cause: 3,
            cohort: 3,
        },
    )
    .unwrap();
    let part = OpenBody::new(part, vec![part_junction]).unwrap();

    attach(&mut host, part, &[]).unwrap();

    assert_eq!(host.returns.live_count, 2);
    assert!(host.returns.by_source.iter().all(Vec::is_empty));
}
