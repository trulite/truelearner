#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct C2RouteAudit {
    pub(super) evidence_shapes: usize,
    pub(super) evidence_observations: u64,
    pub(super) evidence_support: u16,
    pub(super) evidence_margin: u16,
    pub(super) evidence_eligible: bool,
    pub(super) m5_support: usize,
    pub(super) m5_rejection: usize,
    pub(super) m5_score: i32,
    pub(super) m5_value_resistance: i32,
    pub(super) prototype_resistance: i32,
    pub(super) live_proposals: usize,
    pub(super) proposal_resistance: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct C2InternalAudit {
    pub(super) routes: [C2RouteAudit; 2],
    pub(super) observations: u64,
    pub(super) abstentions: u64,
    pub(super) applications: usize,
    pub(super) exploration_admissions: usize,
    pub(super) completed_events: usize,
}

pub(super) fn c2_audit(session: &Session) -> C2InternalAudit {
    let routes = [0, 1].map(|route| {
        let snapshot = session.routes[route][0].0.snapshot();
        let evidence = session.stack.learner.evidence.get(&snapshot);
        let (evidence_support, evidence_margin) =
            evidence.map(ConsequenceEvidence::margin).unwrap_or_default();
        let representation = session.stack.path.encoder.recognized(snapshot);
        let value = representation.and_then(|id| session.stack.path.values.get(&id));
        let live_proposals = session.routes[route]
            .iter()
            .filter(|encounter| {
                session
                    .stack
                    .path
                    .proposals
                    .contains_key(&encounter.0.edge())
            })
            .count();
        let proposal_resistance = session.routes[route]
            .iter()
            .map(|encounter| {
                session
                    .stack
                    .path
                    .proposal_resistance(encounter.0.edge())
            })
            .max()
            .unwrap_or(0);
        C2RouteAudit {
            evidence_shapes: evidence.map_or(0, |record| record.shapes.len()),
            evidence_observations: evidence.map_or(0, |record| {
                record.shapes.values().map(|count| u64::from(*count)).sum()
            }),
            evidence_support,
            evidence_margin,
            evidence_eligible: evidence_support >= RECURRENT_SUPPORT
                && evidence_margin >= MINIMUM_MARGIN,
            m5_support: value.map_or(0, |record| record.support),
            m5_rejection: value.map_or(0, |record| record.rejection),
            m5_score: value.map_or(0, |record| record.score()),
            m5_value_resistance: value.map_or(0, |record| record.life.resistance),
            prototype_resistance: session.stack.path.prototype_resistance(snapshot),
            live_proposals,
            proposal_resistance,
        }
    });
    C2InternalAudit {
        routes,
        observations: session.stack.learner.work.observations,
        abstentions: session.stack.learner.work.abstentions,
        applications: session.stack.applications,
        exploration_admissions: session.stack.path.exploration_admissions,
        completed_events: session.stack.path.completed,
    }
}

pub(super) fn c2_pressure_only(session: &mut Session, events: usize) {
    for _ in 0..events {
        session.stack.path.begin_event();
        session.episode += 1;
    }
}
