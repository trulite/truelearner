use crate::{
    arena::Arena,
    core::{LinkMemory, ReactionScratch, ReactionView, ReturnIndex, UsedPaths},
    physics::*,
    trace::{NoTrace, ObserveTrace, TraceArrival, TraceEvent, TraceSink},
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Firing {
    at: Time,
    target: JunctionId,
    impulse: Impulse,
    cause: u64,
    /// `None` is a boundary arrival; `Some` is the exact transmitting link.
    pub(crate) via: Option<LinkId>,
    next_at_junction: u32,
}

const NO_PARTICIPANT: u32 = u32::MAX;

/// Monotone integer time queue. Each firing moves through at most 64 buckets.
#[derive(Clone, Debug)]
struct Schedule {
    now: Time,
    len: usize,
    buckets: [Vec<Firing>; 65],
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            now: 0,
            len: 0,
            buckets: std::array::from_fn(|_| Vec::new()),
        }
    }
}

impl Schedule {
    fn push(&mut self, firing: Firing) -> Result<(), RunError> {
        if firing.at < self.now {
            return Err(RunError::TimeWentBackward {
                now: self.now,
                requested: firing.at,
            });
        }
        self.buckets[bucket(self.now, firing.at)].push(firing);
        self.len += 1;
        Ok(())
    }

    fn pop_into(&mut self, output: &mut Vec<Firing>) -> Option<Time> {
        if self.len == 0 {
            return None;
        }
        if self.buckets[0].is_empty() {
            let source = (1..self.buckets.len()).find(|index| !self.buckets[*index].is_empty())?;
            self.now = self.buckets[source].iter().map(|firing| firing.at).min()?;
            while let Some(firing) = self.buckets[source].pop() {
                self.buckets[bucket(self.now, firing.at)].push(firing);
            }
        }
        output.clear();
        output.append(&mut self.buckets[0]);
        self.len -= output.len();
        Some(self.now)
    }

    const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

fn bucket(now: Time, at: Time) -> usize {
    let distance = now ^ at;
    if distance == 0 {
        0
    } else {
        (Time::BITS - distance.leading_zeros()) as usize
    }
}

#[derive(Clone, Copy, Debug)]
struct MeetingState {
    impulse: i64,
    cause: u64,
    arrivals: u32,
    first: u32,
    last: u32,
    mixed_cause: bool,
}

impl Default for MeetingState {
    fn default() -> Self {
        Self {
            impulse: 0,
            cause: 0,
            arrivals: 0,
            first: NO_PARTICIPANT,
            last: NO_PARTICIPANT,
            mixed_cause: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Meetings {
    states: Vec<MeetingState>,
    touched: Vec<JunctionId>,
}

impl Meetings {
    fn reserve(&mut self, additional: usize) {
        self.states.reserve(additional);
        self.touched.reserve(additional);
    }

    fn add_junction(&mut self) {
        self.states.push(MeetingState::default());
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct MomentChange {
    pub(crate) event: Event,
    pub(crate) boundary: bool,
    pub(crate) used: UsedPaths,
    #[cfg(test)]
    first_participant: u32,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PhysicalMoment {
    participants: Vec<Firing>,
    pub(crate) changes: Vec<MomentChange>,
}

impl PhysicalMoment {
    #[cfg(test)]
    pub(crate) fn participants(&self, change: &MomentChange) -> Participants<'_> {
        self.participants_from(change.first_participant)
    }

    fn participants_from(&self, first: u32) -> Participants<'_> {
        Participants {
            all: &self.participants,
            next: first,
        }
    }
}

pub(crate) struct Participants<'a> {
    all: &'a [Firing],
    next: u32,
}

impl<'a> Iterator for Participants<'a> {
    type Item = &'a Firing;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == NO_PARTICIPANT {
            return None;
        }
        let participant = &self.all[self.next as usize];
        self.next = participant.next_at_junction;
        Some(participant)
    }
}

#[derive(Clone, Debug, Default)]
struct Activity {
    pending: Schedule,
    moment: PhysicalMoment,
    meetings: Meetings,
    reaction: ReactionScratch,
}

#[derive(Clone, Debug, Default)]
pub struct Body {
    pub(crate) arena: Arena,
    pub(crate) link_memory: Vec<LinkMemory>,
    pub(crate) returns: ReturnIndex,
    activity: Activity,
}

impl Body {
    pub fn reserve(&mut self, junctions: usize, links: usize) {
        self.arena.reserve(junctions, links);
        self.returns.by_source.reserve(junctions);
        self.activity.meetings.reserve(junctions);
    }

    pub fn add_junction(&mut self, law: Junction) -> Result<JunctionId, BuildError> {
        if law.threshold <= 0 {
            return Err(BuildError::NonPositiveThreshold);
        }
        let id = self.arena.add_junction(law)?;
        self.returns.by_source.push(Vec::new());
        self.activity.meetings.add_junction();
        Ok(id)
    }

    pub fn add_link(&mut self, law: Link) -> Result<LinkId, BuildError> {
        let id = self.arena.add_link(law)?;
        self.link_memory.push(LinkMemory::default());
        if self.returns.live_count != 0 {
            self.rebuild_live_returns();
        }
        Ok(id)
    }

    pub fn input(
        &mut self,
        at: Time,
        target: JunctionId,
        impulse: Impulse,
    ) -> Result<(), RunError> {
        self.inputs(at, &[Arrival::new(target, impulse)])
    }

    pub fn inputs(&mut self, at: Time, arrivals: &[Arrival]) -> Result<(), RunError> {
        if arrivals.len() > u32::MAX as usize {
            return Err(RunError::WaveTooLarge);
        }
        for arrival in arrivals {
            self.arena
                .require(arrival.target)
                .map_err(RunError::UnknownJunction)?;
        }
        for arrival in arrivals {
            self.enqueue(at, arrival.target, arrival.impulse, arrival.cause, None)?;
        }
        Ok(())
    }

    /// Executes exactly one scheduled frontier. Quiet is the identity step.
    pub fn step(&mut self, mut observe: impl FnMut(Event)) -> Result<Option<Step>, RunError> {
        let mut work = Work::default();
        let at = self.step_kernel(&mut work, &mut observe, &mut NoTrace)?;
        Ok(at.map(|at| Step { at, work }))
    }

    /// Convenience closure: repeat the same frontier kernel until quiet.
    pub fn run(
        &mut self,
        moment_limit: usize,
        mut observe: impl FnMut(Event),
    ) -> Result<Run, RunError> {
        self.run_kernel(moment_limit, &mut observe, &mut NoTrace)
    }

    /// Runs the ordinary kernel while reporting its complete causal chain.
    pub fn run_traced(
        &mut self,
        moment_limit: usize,
        mut observe: impl FnMut(Event),
        trace: impl FnMut(TraceEvent),
    ) -> Result<Run, RunError> {
        self.run_kernel(moment_limit, &mut observe, &mut ObserveTrace::new(trace))
    }

    fn run_kernel<T: TraceSink>(
        &mut self,
        moment_limit: usize,
        observe: &mut impl FnMut(Event),
        trace: &mut T,
    ) -> Result<Run, RunError> {
        let mut run = Run::default();
        while !self.is_quiet() {
            if run.moments == moment_limit as u64 {
                return Err(RunError::MomentLimitReached);
            }
            self.step_kernel(&mut run.work, observe, trace)?
                .expect("body is not quiet");
            run.moments += 1;
        }
        if T::ENABLED {
            trace.record(TraceEvent::Quiet(run));
        }
        Ok(run)
    }

    pub fn held(&self, junction: JunctionId) -> Option<Impulse> {
        self.arena.junction(junction).map(|slot| slot.held())
    }

    pub fn clear(&mut self, junction: JunctionId) -> Result<(), RunError> {
        self.arena
            .junction_mut(junction)
            .ok_or(RunError::UnknownJunction(junction))?
            .clear();
        Ok(())
    }

    pub fn is_quiet(&self) -> bool {
        self.activity.pending.is_empty()
    }

    /// Last physical moment reached by the body.
    pub const fn now(&self) -> Time {
        self.activity.pending.now
    }

    pub(crate) fn attachment_time(&self) -> Time {
        self.activity.pending.now
    }

    pub(crate) fn restore_checkpoint_time(&mut self, now: Time) {
        debug_assert!(self.is_quiet());
        self.activity.pending.now = now;
    }

    pub(crate) fn prepare_attachment(&mut self, junctions: usize, at: Time) {
        debug_assert!(self.is_quiet());
        self.activity.pending.now = at;
        for _ in 0..junctions {
            self.returns.by_source.push(Vec::new());
            self.activity.meetings.add_junction();
        }
    }

    fn step_kernel<T: TraceSink>(
        &mut self,
        work: &mut Work,
        observe: &mut impl FnMut(Event),
        trace: &mut T,
    ) -> Result<Option<Time>, RunError> {
        let Some(at) = self
            .activity
            .pending
            .pop_into(&mut self.activity.moment.participants)
        else {
            return Ok(None);
        };
        self.activity.moment.changes.clear();
        let wave_arrivals = u32::try_from(self.activity.moment.participants.len())
            .map_err(|_| RunError::WaveTooLarge)?;
        work.arrivals += u64::from(wave_arrivals);

        for participant_index in 0..self.activity.moment.participants.len() {
            let participant =
                u32::try_from(participant_index).map_err(|_| RunError::WaveTooLarge)?;
            let firing = self.activity.moment.participants[participant_index];
            if T::ENABLED {
                trace.record(TraceEvent::Arrival(TraceArrival {
                    at: firing.at,
                    target: firing.target,
                    impulse: firing.impulse,
                    cause: firing.cause,
                    via: firing.via,
                }));
            }
            let index = firing.target.slot();
            let meeting = &mut self.activity.meetings.states[index];
            if meeting.arrivals == 0 {
                self.activity.meetings.touched.push(firing.target);
                meeting.cause = firing.cause;
                meeting.first = participant;
            } else {
                self.activity.moment.participants[meeting.last as usize].next_at_junction =
                    participant;
                meeting.mixed_cause |= meeting.cause != firing.cause;
            }
            meeting.last = participant;
            meeting.impulse += i64::from(firing.impulse);
            meeting.arrivals = meeting
                .arrivals
                .checked_add(1)
                .ok_or(RunError::WaveTooLarge)?;
        }

        for touched_index in 0..self.activity.meetings.touched.len() {
            let junction = self.activity.meetings.touched[touched_index];
            let index = junction.slot();
            let meeting = std::mem::take(&mut self.activity.meetings.states[index]);
            work.meetings += 1;
            let cause = if meeting.mixed_cause {
                0
            } else {
                meeting.cause
            };

            let slot = self.arena.junction_mut(junction).expect("live junction");
            let Some((before, after)) = slot.change(at, meeting.impulse, cause) else {
                continue;
            };
            work.changes += 1;
            let change = Event {
                at,
                junction,
                arrivals: meeting.arrivals,
                impulse: meeting.impulse,
                before,
                after,
                cause,
            };
            let mut boundary = false;
            let mut used = UsedPaths::None;
            let mut participant_count = 0_u32;
            let mut all_links_exist = true;
            let body = ReactionView::new(&self.arena, &self.link_memory, &self.returns);
            for participant in self.activity.moment.participants_from(meeting.first) {
                participant_count += 1;
                boundary |= participant.via.is_none();
                all_links_exist &= participant
                    .via
                    .is_none_or(|link| self.arena.link(link).is_some());
                used.include(
                    participant
                        .via
                        .and_then(|link| crate::core::path_from_drive(body, link)),
                );
            }
            debug_assert_eq!(participant_count, change.arrivals);
            debug_assert!(all_links_exist);
            self.activity.moment.changes.push(MomentChange {
                event: change,
                boundary,
                used,
                #[cfg(test)]
                first_participant: meeting.first,
            });
        }
        for change_index in 0..self.activity.moment.changes.len() {
            let recorded = self.activity.moment.changes[change_index];
            observe(recorded.event);
            if T::ENABLED {
                trace.record(TraceEvent::Transition(recorded.event));
            }
            self.transmit(recorded.event, work)?;
        }
        let reaction_needed = crate::core::reaction_needed(
            ReactionView::new(&self.arena, &self.link_memory, &self.returns),
            &self.activity.moment,
        );
        let reaction_result = if !reaction_needed {
            Ok(())
        } else {
            crate::core::react_into(
                ReactionView::new(&self.arena, &self.link_memory, &self.returns),
                &self.activity.moment,
                &mut self.activity.reaction,
                trace,
            );
            if self.activity.reaction.change.is_empty() {
                self.activity.reaction.clear();
                Ok(())
            } else {
                let mut change = std::mem::take(&mut self.activity.reaction.change);
                let mut applied = std::mem::take(&mut self.activity.reaction.applied);
                let result = self.apply_reusing(&mut change, &mut applied, at, trace);
                self.activity.reaction.change = change;
                self.activity.reaction.applied = applied;
                self.activity.reaction.clear();
                result
            }
        };
        reaction_result.map_err(|error| match error {
            crate::core::ApplyError::Build(BuildError::CapacityExhausted) => {
                RunError::CapacityExhausted
            }
            crate::core::ApplyError::Run(error) => error,
            _ => RunError::InvalidReaction,
        })?;
        self.activity.meetings.touched.clear();
        Ok(Some(at))
    }

    fn transmit(&mut self, change: Event, work: &mut Work) -> Result<(), RunError> {
        let mut next = self
            .arena
            .junction(change.junction)
            .and_then(|slot| slot.outgoing_head);
        while let Some(link_id) = next {
            let link = *self.arena.link(link_id).expect("live link");
            if self.link_memory[link_id.slot()].live
                && self.link_memory[link_id.slot()].role == crate::core::LinkRole::Drive
            {
                work.link_visits += 1;
            }
            if self.link_memory[link_id.slot()].live
                && self.link_memory[link_id.slot()].role == crate::core::LinkRole::Drive
                && opens(link.trigger, change.before, change.after)
            {
                let impulse =
                    effective_impulse(link.impulse, self.link_memory[link_id.slot()].strength);
                self.activity.pending.push(Firing {
                    at: change.at.saturating_add(link.delay),
                    target: link.to,
                    impulse,
                    cause: change.cause,
                    via: Some(link_id),
                    next_at_junction: NO_PARTICIPANT,
                })?;
                self.link_memory[link_id.slot()].record_transmission(change.cause, change.at);
                work.emissions += 1;
            }
            next = link.next;
        }
        Ok(())
    }

    fn enqueue(
        &mut self,
        at: Time,
        target: JunctionId,
        impulse: Impulse,
        cause: u64,
        via: Option<LinkId>,
    ) -> Result<(), RunError> {
        self.activity.pending.push(Firing {
            at,
            target,
            impulse,
            cause,
            via,
            next_at_junction: NO_PARTICIPANT,
        })
    }

    pub(crate) fn send_through(
        &mut self,
        at: Time,
        link_id: LinkId,
        cause: u64,
    ) -> Result<(), RunError> {
        let link = *self.arena.link(link_id).expect("validated live link");
        let impulse = effective_impulse(link.impulse, self.link_memory[link_id.slot()].strength);
        self.enqueue(
            at.saturating_add(link.delay),
            link.to,
            impulse,
            cause,
            Some(link_id),
        )?;
        self.link_memory[link_id.slot()].record_transmission(cause, at);
        Ok(())
    }
}

fn effective_impulse(impulse: Impulse, strength: i64) -> Impulse {
    i64::from(impulse)
        .saturating_mul(strength)
        .clamp(i64::from(Impulse::MIN), i64::from(Impulse::MAX)) as Impulse
}

#[cfg(test)]
mod tests {
    use super::*;

    fn participant_links(body: &Body, change: usize) -> Vec<Option<LinkId>> {
        let change = &body.activity.moment.changes[change];
        body.activity
            .moment
            .participants(change)
            .map(|participant| participant.via)
            .collect()
    }

    #[test]
    fn frontier_preserves_boundary_and_link_participants() {
        let mut body = Body::default();
        let source = body.add_junction(Junction::integrating(2)).unwrap();
        let middle = body.add_junction(Junction::integrating(1)).unwrap();
        let target = body.add_junction(Junction::integrating(1)).unwrap();
        let first = body.add_link(Link::new(source, middle, 1, 1)).unwrap();
        let second = body.add_link(Link::new(middle, target, 1, 1)).unwrap();
        body.inputs(
            3,
            &[Arrival::caused(source, 1, 7), Arrival::caused(source, 1, 7)],
        )
        .unwrap();

        body.step(|_| {}).unwrap();
        assert_eq!(body.activity.moment.changes[0].event.cause, 7);
        assert_eq!(participant_links(&body, 0), [None, None]);
        assert!(body.link_memory[first.slot()].transmitted);
        assert_eq!(body.link_memory[first.slot()].cause, 7);
        assert_eq!(body.link_memory[first.slot()].participated_at, 3);

        body.step(|_| {}).unwrap();
        assert_eq!(body.activity.moment.changes[0].event.junction, middle);
        assert_eq!(participant_links(&body, 0), [Some(first)]);
        assert!(body.link_memory[second.slot()].transmitted);
        assert_eq!(body.link_memory[second.slot()].cause, 7);
        assert_eq!(body.link_memory[second.slot()].participated_at, 4);

        body.step(|_| {}).unwrap();
        assert_eq!(body.activity.moment.changes[0].event.junction, target);
        assert_eq!(participant_links(&body, 0), [Some(second)]);
    }

    #[test]
    fn meeting_cause_is_order_independent_and_accumulated_once() {
        fn episode(causes: [u64; 3]) -> u64 {
            let mut body = Body::default();
            let junction = body.add_junction(Junction::integrating(3)).unwrap();
            body.inputs(0, &causes.map(|cause| Arrival::caused(junction, 1, cause)))
                .unwrap();
            body.step(|_| {}).unwrap();
            body.activity.moment.changes[0].event.cause
        }

        assert_eq!(episode([8, 8, 8]), 8);
        assert_eq!(episode([8, 9, 8]), 0);
        assert_eq!(episode([9, 8, 8]), 0);
    }

    #[test]
    fn explicit_send_preserves_its_link_identity() {
        use crate::core::{react_event, Candidate, Context, Event as CoreEvent, Owner, Path};

        let mut body = Body::default();
        let source = body.add_junction(Junction::integrating(1)).unwrap();
        let middle = body.add_junction(Junction::integrating(1)).unwrap();
        let output = body.add_junction(Junction::integrating(1)).unwrap();
        let first = body.add_link(Link::new(source, middle, 2, 1)).unwrap();
        let second = body.add_link(Link::new(middle, output, 0, 1)).unwrap();
        let path = Path {
            surface: source,
            middle,
            output,
            first,
            second,
        };
        let candidates = [Candidate {
            path,
            connected_outcomes: &[],
            owner: Owner::Organism,
            opportunity_from: Some(Owner::Organism),
            executable: true,
            return_cause: None,
            unanswered: true,
            outcome: None,
            participation: 0,
            strength: 1,
            stable_order: 0,
        }];
        let reaction = react_event(Context {
            at: 4,
            event: CoreEvent::PathsReady {
                candidates: &candidates,
                current_transition: Some(12),
            },
        });

        body.apply(reaction.change).unwrap();
        assert!(body.link_memory[first.slot()].transmitted);
        assert_eq!(body.link_memory[first.slot()].cause, 12);
        assert_eq!(body.link_memory[first.slot()].participated_at, 4);
        body.step(|_| {}).unwrap();

        assert_eq!(body.activity.moment.changes[0].event.at, 6);
        assert_eq!(body.activity.moment.changes[0].event.cause, 12);
        assert_eq!(participant_links(&body, 0), [Some(first)]);
    }

    #[test]
    fn boundary_input_and_failed_enqueue_record_no_link_transmission() {
        let mut body = Body::default();
        let source = body.add_junction(Junction::integrating(1)).unwrap();
        let target = body.add_junction(Junction::integrating(1)).unwrap();
        let clock = body.add_junction(Junction::integrating(1)).unwrap();
        let link = body.add_link(Link::new(source, target, 0, 1)).unwrap();
        body.set_link_role(link, crate::core::LinkRole::PathEntry)
            .unwrap();

        body.input(10, clock, 1).unwrap();
        body.step(|_| {}).unwrap();
        assert!(!body.link_memory[link.slot()].transmitted);

        assert_eq!(
            body.send_through(9, link, 3),
            Err(RunError::TimeWentBackward {
                now: 10,
                requested: 9,
            })
        );
        assert!(!body.link_memory[link.slot()].transmitted);
    }

    #[test]
    fn irrelevant_waves_do_not_touch_reaction_workspace() {
        let mut body = Body::default();
        let source = body.add_junction(Junction::integrating(1)).unwrap();
        let target = body.add_junction(Junction::integrating(1)).unwrap();
        body.add_link(Link::new(source, target, 0, 1)).unwrap();

        body.input(1, source, 1).unwrap();
        body.run(4, |_| {}).unwrap();
        assert!(body.activity.reaction.is_clear());
        assert_eq!(body.activity.reaction.fact_capacity(), 0);
    }

    #[test]
    fn relevant_wave_clears_and_reuses_reaction_workspace() {
        let mut body = Body::default();
        let source = body.add_junction(Junction::integrating(1)).unwrap();
        let output = body.add_junction(Junction::integrating(1)).unwrap();
        body.add_link(Link::new(source, output, 1, 0)).unwrap();

        body.input(1, source, 1).unwrap();
        body.step(|_| {}).unwrap();
        assert!(body.activity.reaction.is_clear());
        assert!(body.activity.reaction.fact_capacity() > 0);
    }
}
