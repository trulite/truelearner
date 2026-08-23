
pub(super) fn s3_offer_masked(session: &mut Session, present: [bool; 2]) -> [usize; 2] {
    session.stack.path.begin_event();
    for (route, encounters) in session.routes.iter().enumerate() {
        if present[route] {
            for encounter in encounters {
                let _ = session.stack.path.local_encounter(encounter.0);
            }
        }
    }
    live_supporters(&session.stack, &session.routes)
}

pub(super) fn s3_recur_live_before_event(session: &mut Session, route: usize) -> usize {
    let live_edges: Vec<_> = session.routes[route]
        .iter()
        .filter(|encounter| session.stack.path.proposals.contains_key(&encounter.0.edge()))
        .copied()
        .collect();
    for encounter in &live_edges {
        let _ = session.stack.path.local_encounter(encounter.0);
    }
    live_edges.len()
}

pub(super) fn s3_session_exact(first: &Session, second: &Session) -> bool {
    first == second
}
