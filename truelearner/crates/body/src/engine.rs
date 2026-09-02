use crate::{
    arena::Arena,
    core::{
        ArrowState, Consolidation, ReactionScratch, ReactionView, ReentryCache, ReturnIndex,
        UsedPaths,
    },
    physics::*,
    trace::{NoTrace, ObserveTrace, TraceArrival, TraceEvent, TraceSink},
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Firing {
    at: Time,
    target: JunctionId,
    impulse: Impulse,
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
    single: Option<Firing>,
    buckets: [Vec<Firing>; 65],
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            now: 0,
            len: 0,
            single: None,
            buckets: std::array::from_fn(|_| Vec::new()),
        }
    }
}

impl Schedule {
    #[inline(always)]
    fn push(&mut self, firing: Firing) -> Result<(), RunError> {
        if firing.at < self.now {
            return Err(RunError::TimeWentBackward {
                now: self.now,
                requested: firing.at,
            });
        }
        if self.len == 0 {
            debug_assert!(self.single.is_none());
            debug_assert!(self.buckets.iter().all(Vec::is_empty));
            self.single = Some(firing);
            self.len = 1;
            return Ok(());
        }
        if let Some(single) = self.single.take() {
            self.buckets[bucket(self.now, single.at)].push(single);
        }
        self.buckets[bucket(self.now, firing.at)].push(firing);
        self.len += 1;
        Ok(())
    }

    #[inline(always)]
    fn pop_into(&mut self, output: &mut Vec<Firing>) -> Option<Time> {
        if self.len == 0 {
            return None;
        }
        if let Some(firing) = self.single.take() {
            self.now = firing.at;
            self.len = 0;
            output.clear();
            output.push(firing);
            return Some(self.now);
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

    fn reaches_by(&self, target: JunctionId, not_before: Time, not_after: Time) -> bool {
        let matches = |firing: &Firing| {
            firing.target == target && (not_before..=not_after).contains(&firing.at)
        };
        self.single.as_ref().is_some_and(matches) || self.buckets.iter().flatten().any(matches)
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
    arrivals: u32,
    first: u32,
    last: u32,
}

impl Default for MeetingState {
    fn default() -> Self {
        Self {
            impulse: 0,
            arrivals: 0,
            first: NO_PARTICIPANT,
            last: NO_PARTICIPANT,
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
    predecessor: Option<LinkId>,
    #[cfg(test)]
    first_participant: u32,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PhysicalMoment {
    participants: Vec<Firing>,
    pub(crate) changes: Vec<MomentChange>,
}

impl PhysicalMoment {
    pub(crate) fn boundary_arrivals(&self) -> impl Iterator<Item = (Time, JunctionId)> + '_ {
        self.participants
            .iter()
            .filter(|participant| participant.via.is_none())
            .map(|participant| (participant.at, participant.target))
    }

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
    pub(crate) arrows: Vec<ArrowState>,
    pub(crate) returns: ReturnIndex,
    pub(crate) consolidation: Option<Box<Consolidation>>,
    pub(crate) reentry: Option<Box<ReentryCache>>,
    pub(crate) has_composites: bool,
    pub(crate) has_local_plasticity: bool,
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
        if let Retention::Sampled { range, .. } = law.retention {
            if range == 0 || range > i32::MAX as u32 {
                return Err(BuildError::InvalidRange);
            }
        }
        let id = self.arena.add_junction(law)?;
        self.returns.by_source.push(Vec::new());
        self.activity.meetings.add_junction();
        Ok(id)
    }

    pub fn add_link(&mut self, law: Link) -> Result<LinkId, BuildError> {
        let endpoints = [law.from, law.to];
        let id = self.add_link_untracked(law)?;
        self.touch_reentry_junctions(endpoints);
        Ok(id)
    }

    pub(crate) fn add_link_untracked(&mut self, law: Link) -> Result<LinkId, BuildError> {
        let id = self.arena.add_link(law)?;
        self.arrows.push(ArrowState::drive());
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
            self.enqueue(at, arrival.target, arrival.impulse, None)?;
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
        if self.activity.moment.participants.len() == 1 {
            return self.step_single(at, work, observe, trace);
        }
        self.step_multiple(at, work, observe, trace)
    }

    #[inline(never)]
    fn step_multiple<T: TraceSink>(
        &mut self,
        at: Time,
        work: &mut Work,
        observe: &mut impl FnMut(Event),
        trace: &mut T,
    ) -> Result<Option<Time>, RunError> {
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
                    via: firing.via,
                }));
            }
            let index = firing.target.slot();
            let meeting = &mut self.activity.meetings.states[index];
            if meeting.arrivals == 0 {
                self.activity.meetings.touched.push(firing.target);
                meeting.first = participant;
            } else {
                self.activity.moment.participants[meeting.last as usize].next_at_junction =
                    participant;
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
            let slot = self.arena.junction_mut(junction).expect("live junction");
            let Some((before, after)) = slot.change(at, meeting.impulse) else {
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
            };
            let mut boundary = false;
            let mut used = UsedPaths::None;
            let mut participant_count = 0_u32;
            let mut all_links_exist = true;
            let body = ReactionView::new(&self.arena, &self.arrows, &self.returns);
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
                predecessor: (meeting.arrivals == 1)
                    .then(|| self.activity.moment.participants[meeting.first as usize].via)
                    .flatten(),
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
            self.transmit(recorded.event, recorded.predecessor, work)?;
        }
        self.react_current_moment(at, trace)?;
        self.activity.meetings.touched.clear();
        Ok(Some(at))
    }

    fn step_single<T: TraceSink>(
        &mut self,
        at: Time,
        work: &mut Work,
        observe: &mut impl FnMut(Event),
        trace: &mut T,
    ) -> Result<Option<Time>, RunError> {
        let firing = self.activity.moment.participants[0];
        work.arrivals += 1;
        if T::ENABLED {
            trace.record(TraceEvent::Arrival(TraceArrival {
                at: firing.at,
                target: firing.target,
                impulse: firing.impulse,
                via: firing.via,
            }));
        }
        work.meetings += 1;
        let Some((before, after)) = self
            .arena
            .junction_mut(firing.target)
            .expect("live junction")
            .change(at, i64::from(firing.impulse))
        else {
            self.activity.moment.changes.clear();
            self.react_current_moment(at, trace)?;
            return Ok(Some(at));
        };
        work.changes += 1;
        let event = Event {
            at,
            junction: firing.target,
            arrivals: 1,
            impulse: i64::from(firing.impulse),
            before,
            after,
        };
        observe(event);
        if T::ENABLED {
            trace.record(TraceEvent::Transition(event));
        }
        let completed = if firing.via.is_none() && self.has_local_plasticity {
            let mut local_junctions = std::mem::take(&mut self.activity.reaction.local_junctions);
            let mut local_eligible = std::mem::take(&mut self.activity.reaction.local_eligible);
            let completed = crate::core::try_complete_single_return(
                self,
                event,
                Some((&mut local_junctions, &mut local_eligible)),
                trace,
            );
            self.activity.reaction.local_junctions = local_junctions;
            self.activity.reaction.local_eligible = local_eligible;
            completed
        } else {
            firing.via.is_none()
                && crate::core::try_complete_single_return(self, event, None, trace)
        };
        if completed {
            self.activity.moment.changes.clear();
            return Ok(Some(at));
        }
        self.transmit(event, firing.via, work)?;
        let body = ReactionView::new(&self.arena, &self.arrows, &self.returns);
        let used = firing
            .via
            .and_then(|link| crate::core::path_from_drive(body, link))
            .map_or(UsedPaths::None, UsedPaths::One);
        self.activity.moment.changes.clear();
        self.activity.moment.changes.push(MomentChange {
            event,
            boundary: firing.via.is_none(),
            used,
            predecessor: firing.via,
            #[cfg(test)]
            first_participant: 0,
        });
        self.react_current_moment(at, trace)?;
        Ok(Some(at))
    }

    fn react_current_moment<T: TraceSink>(
        &mut self,
        at: Time,
        trace: &mut T,
    ) -> Result<(), RunError> {
        if !crate::core::reaction_needed(
            ReactionView::new(&self.arena, &self.arrows, &self.returns),
            &self.activity.moment,
        ) {
            return Ok(());
        }
        crate::core::react_into(
            ReactionView::with_reentry(
                &self.arena,
                &self.arrows,
                &self.returns,
                self.reentry.as_deref(),
            ),
            &self.activity.moment,
            &mut self.activity.reaction,
            trace,
        );
        if self.activity.reaction.change.is_empty() {
            self.activity.reaction.clear();
            return Ok(());
        }
        let mut change = std::mem::take(&mut self.activity.reaction.change);
        let mut applied = std::mem::take(&mut self.activity.reaction.applied);
        let result = self.apply_reusing(&mut change, &mut applied, at, trace);
        self.activity.reaction.change = change;
        self.activity.reaction.applied = applied;
        self.activity.reaction.clear();
        result.map_err(|error| match error {
            crate::core::ApplyError::Build(BuildError::CapacityExhausted) => {
                RunError::CapacityExhausted
            }
            crate::core::ApplyError::Run(error) => error,
            _ => RunError::InvalidReaction,
        })
    }

    #[inline(always)]
    fn transmit(
        &mut self,
        change: Event,
        predecessor: Option<LinkId>,
        work: &mut Work,
    ) -> Result<(), RunError> {
        let mut next = self
            .arena
            .junction(change.junction)
            .and_then(|slot| slot.outgoing_head);
        while let Some(link_id) = next {
            let link = *self.arena.link(link_id).expect("live link");
            let is_drive = self.arrows[link_id.slot()].is_drive()
                && self.arrows[link_id.slot()].factors().is_none();
            if is_drive {
                work.link_visits += 1;
                if opens(link.trigger, change.before, change.after) {
                    let through = if self.has_composites
                        && !self.arrows[link_id.slot()].boundary_crossing()
                    {
                        self.preferred_automatic_drive(link_id)
                    } else {
                        link_id
                    };
                    let through = if through != link_id
                        && self.automatic_drive_is_interrupted(through, change.at, change.at, 0)
                    {
                        link_id
                    } else {
                        through
                    };
                    let selected = *self.arena.link(through).expect("validated automatic drive");
                    let impulse =
                        effective_impulse(selected.impulse, self.arrows[through.slot()].strength());
                    self.activity.pending.push(Firing {
                        at: change.at.saturating_add(selected.delay),
                        target: selected.to,
                        impulse,
                        via: Some(through),
                        next_at_junction: NO_PARTICIPANT,
                    })?;
                    self.arrows[through.slot()]
                        .record_transmission(crate::core::Occurrence { at: change.at });
                    if let Some(first) = predecessor {
                        self.observe_automatic_pair(first, through, change.at);
                    }
                    work.emissions += 1;
                }
            }
            next = link.next;
        }
        Ok(())
    }

    fn automatic_drive_is_interrupted(
        &self,
        link: LinkId,
        started_at: Time,
        segment_started_at: Time,
        depth: usize,
    ) -> bool {
        if depth >= 32 {
            return true;
        }
        let Some([first, second]) = self.arrows[link.slot()].factors() else {
            return false;
        };
        let Some(first_physical) = self.arena.link(first) else {
            return true;
        };
        let middle_at = segment_started_at.saturating_add(first_physical.delay);
        self.automatic_drive_is_interrupted(first, started_at, segment_started_at, depth + 1)
            || self
                .activity
                .pending
                .reaches_by(first_physical.to, started_at, middle_at)
            || self.automatic_drive_is_interrupted(second, started_at, middle_at, depth + 1)
    }

    fn enqueue(
        &mut self,
        at: Time,
        target: JunctionId,
        impulse: Impulse,
        via: Option<LinkId>,
    ) -> Result<(), RunError> {
        self.activity.pending.push(Firing {
            at,
            target,
            impulse,
            via,
            next_at_junction: NO_PARTICIPANT,
        })
    }

    pub(crate) fn send_through(&mut self, at: Time, link_id: LinkId) -> Result<(), RunError> {
        let link = *self.arena.link(link_id).expect("validated live link");
        let impulse = effective_impulse(link.impulse, self.arrows[link_id.slot()].strength());
        self.enqueue(
            at.saturating_add(link.delay),
            link.to,
            impulse,
            Some(link_id),
        )?;
        self.arrows[link_id.slot()].record_transmission(crate::core::Occurrence { at });
        Ok(())
    }
}

fn effective_impulse(impulse: Impulse, strength: i64) -> Impulse {
    i64::from(impulse)
        .saturating_mul(strength)
        .clamp(i64::from(Impulse::MIN), i64::from(Impulse::MAX)) as Impulse
}

#[cfg(test)]
#[path = "tests/engine.rs"]
mod tests;
