//! Unit-test frontier for retaining identity-free causal form as composed motifs.
//!
//! Existing active laws already cover quiet identity, causal composition,
//! independent products, directional physical triggers, exact-return support,
//! recursive compaction, branch invalidation, attachment renaming, and work.
//! The audit found two missing handoffs. This module isolates retaining the
//! direction of a sampled change on a closed learned path; the existing
//! planning ladder freezes caused cross-instance membership separately.

use crate::{
    harness::{attach_outcome_component, attach_sensor, effect, motor, schedule, Motor},
    verify_choice_contract, Arrival, Body, ChoiceWarrant, Junction, JunctionId, LinkId,
    PhysicalEvent, ReturnDecision, TraceEvent, TracePath, Trigger,
};

#[derive(Clone)]
struct DeltaWorld {
    body: Body,
    surface: JunctionId,
    motors: [Motor; 2],
    outcome: JunctionId,
    outcome_value: i32,
    baseline: i32,
}

impl DeltaWorld {
    const PROXY: usize = 0;
    const CLOSER: usize = 1;

    fn new(baseline: i32) -> Self {
        let mut body = Body::default();
        let motors = std::array::from_fn(|_| motor(&mut body));
        let surface = attach_sensor(
            &mut body,
            Junction::sampled(1_000),
            &[
                (motors[Self::PROXY].opportunity, 1),
                (motors[Self::CLOSER].opportunity, 1),
            ],
        );
        let outcome = attach_sensor(&mut body, Junction::sampled(1_000), &[]);
        attach_outcome_component(&mut body, outcome, [motors[Self::CLOSER].opportunity]);
        schedule(
            &mut body,
            0,
            &[Arrival::new(surface, baseline), Arrival::new(outcome, 0)],
        );
        run(&mut body);
        Self {
            body,
            surface,
            motors,
            outcome,
            outcome_value: 0,
            baseline,
        }
    }

    fn act(&mut self, value: i32, outputs: &[usize], at: u64) -> (Vec<usize>, Vec<TraceEvent>) {
        schedule(&mut self.body, at, &[Arrival::new(self.surface, value)]);
        schedule(
            &mut self.body,
            at + 1,
            &outputs
                .iter()
                .map(|output| Arrival::new(self.motors[*output].opportunity, 1))
                .collect::<Vec<_>>(),
        );
        let (events, trace) = run(&mut self.body);
        (effect(&events, &self.motors), trace)
    }

    fn close(&mut self, at: u64) {
        self.outcome_value += 1;
        schedule(
            &mut self.body,
            at,
            &[Arrival::new(self.outcome, self.outcome_value)],
        );
        let (_, trace) = run(&mut self.body);
        assert!(trace.iter().any(|event| matches!(
            event,
            TraceEvent::Return(returned)
                if returned.source == self.outcome
                    && returned.decision == ReturnDecision::Accepted
        )));
    }

    fn demonstrate_rise(&mut self) {
        assert_eq!(
            self.act(self.baseline + 2, &[Self::PROXY], 10).0,
            [Self::PROXY]
        );
        assert_eq!(
            self.act(self.baseline + 4, &[Self::CLOSER], 20).0,
            [Self::CLOSER]
        );
        self.close(22);
    }

    fn probe(&mut self, value: i32) -> (Vec<usize>, Vec<TraceEvent>) {
        self.act(value, &[Self::PROXY, Self::CLOSER], 30)
    }
}

fn run(body: &mut Body) -> (Vec<PhysicalEvent>, Vec<TraceEvent>) {
    let mut events = Vec::new();
    let mut trace = Vec::new();
    body.run_traced(256, |event| events.push(event), |event| trace.push(event))
        .unwrap();
    assert!(body.is_quiet());
    verify_choice_contract(&trace).unwrap();
    (events, trace)
}

fn selected_candidate(trace: &[TraceEvent]) -> (&TracePath, u16, ChoiceWarrant) {
    let choice = trace.iter().find_map(|event| match event {
        TraceEvent::Choice(choice) if choice.sent => Some(choice),
        _ => None,
    });
    let choice = choice.expect("one physical delta produces one ordinary choice");
    let winner = choice.winner.as_ref().expect("a sent choice has a winner");
    let candidate = trace.iter().find_map(|event| match event {
        TraceEvent::Candidate(candidate) if &candidate.path == winner => Some(candidate),
        _ => None,
    });
    let candidate = candidate.expect("the choice names a traced candidate");
    (
        winner,
        candidate.drive,
        choice
            .warrant
            .expect("a sent choice names its physical warrant"),
    )
}

#[test]
fn shifted_absolute_baselines_preserve_the_same_closed_rise_behavior() {
    let episode = |baseline| {
        let mut world = DeltaWorld::new(baseline);
        world.demonstrate_rise();
        let (events, trace) = world.probe(baseline + 6);
        let (_, drive, warrant) = selected_candidate(&trace);
        (events, drive, warrant)
    };

    let low = episode(10);
    let high = episode(100);
    assert_eq!(low.0, [DeltaWorld::CLOSER]);
    assert_eq!(low.2, ChoiceWarrant::RetainedContinuation);
    assert_eq!(low, high);
}

#[test]
fn a_closed_rise_motif_does_not_reuse_for_an_equal_magnitude_fall() {
    let mut learned = DeltaWorld::new(10);
    learned.demonstrate_rise();
    let mut rising = learned.clone();

    assert_eq!(rising.probe(16).0, [DeltaWorld::CLOSER]);
    let (events, trace) = learned.probe(12);

    assert_eq!(
        events,
        [DeltaWorld::PROXY],
        "choices={:#?}",
        trace
            .iter()
            .filter(|event| matches!(event, TraceEvent::Candidate(_) | TraceEvent::Choice(_)))
            .collect::<Vec<_>>()
    );
}

#[test]
fn a_shortcut_preserves_the_sampled_direction_of_its_closed_path() {
    let mut learned = DeltaWorld::new(10);
    learned.demonstrate_rise();
    for (value, at) in [(16, 30), (18, 40)] {
        assert_eq!(
            learned.act(value, &[DeltaWorld::CLOSER], at).0,
            [DeltaWorld::CLOSER]
        );
        learned.close(at + 2);
    }

    let shortcut_triggers = learned
        .body
        .arrows
        .iter()
        .enumerate()
        .filter(|(_, memory)| memory.factors().is_some())
        .map(|(slot, _)| {
            learned
                .body
                .arena
                .link(LinkId::new(slot).expect("live shortcut slot"))
                .expect("live shortcut")
                .trigger
        })
        .collect::<Vec<_>>();
    assert_eq!(shortcut_triggers, [Trigger::Rises]);

    let mut rising = learned.clone();
    assert_eq!(
        rising
            .act(20, &[DeltaWorld::PROXY, DeltaWorld::CLOSER], 50)
            .0,
        [DeltaWorld::CLOSER]
    );
    assert_eq!(
        learned
            .act(16, &[DeltaWorld::PROXY, DeltaWorld::CLOSER], 50)
            .0,
        [DeltaWorld::PROXY]
    );
}
