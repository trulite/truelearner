use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum UnansweredReturnPolicy {
    Ignore,
    Defer,
    Replace,
}

struct OutputCandidate {
    incidence: usize,
    position: i32,
    drive: u64,
    participation: u64,
    unanswered: Vec<LinkId>,
    origins: Vec<u64>,
    consequence_tick: Option<i64>,
}

const RECENT_ELIGIBILITY_TICKS: i64 = 4;

impl Body {
    pub(crate) fn choose_at(&mut self, moment: &mut Moment, run: &mut RunState) {
        self.choose_with_unanswered_policy(moment, run, UnansweredReturnPolicy::Ignore, false);
    }

    pub(crate) fn choose_after_unanswered_return(
        &mut self,
        moment: &mut Moment,
        run: &mut RunState,
    ) {
        self.choose_with_unanswered_policy(moment, run, UnansweredReturnPolicy::Defer, false);
    }

    pub(crate) fn choose_and_replace_unanswered_return(
        &mut self,
        moment: &mut Moment,
        run: &mut RunState,
    ) {
        self.choose_with_unanswered_policy(moment, run, UnansweredReturnPolicy::Replace, false);
    }

    pub(crate) fn choose_sensorimotor_candidate(
        &mut self,
        moment: &mut Moment,
        run: &mut RunState,
    ) {
        self.choose_with_unanswered_policy(moment, run, UnansweredReturnPolicy::Replace, true);
    }

    fn choose_with_unanswered_policy(
        &mut self,
        moment: &mut Moment,
        run: &mut RunState,
        policy: UnansweredReturnPolicy,
        candidate_protocol: bool,
    ) {
        self.compete_local_outputs(moment, run, policy, candidate_protocol);
        if candidate_protocol {
            self.compete_causal_origins(moment, run);
        }
        for incidence in &mut moment.incidences {
            self.choose(
                incidence.junction,
                &mut incidence.inputs,
                &mut run.work,
                &mut run.trace,
                moment.phase,
            );
        }
    }

    /// Let one locally ready output participate before neighboring outputs can
    /// cancel one another outside the body. Learned drive wins; equal drive
    /// prefers the route that has participated less recently.
    fn compete_local_outputs(
        &mut self,
        moment: &mut Moment,
        run: &mut RunState,
        policy: UnansweredReturnPolicy,
        candidate_protocol: bool,
    ) {
        let mut candidates = moment
            .incidences
            .iter()
            .enumerate()
            .filter_map(|(index, incidence)| {
                if !self.arena.is_output_junction(incidence.junction) {
                    return None;
                }
                let target = self
                    .arena
                    .junction_snapshot(self.arena.junction_slot(incidence.junction)?.0);
                let held = self
                    .arena
                    .activation
                    .get(incidence.junction.0 as usize)
                    .copied()
                    .unwrap_or(0);
                let other = incidence.inputs.iter().fold(0_i64, |sum, firing| {
                    if firing
                        .link
                        .is_some_and(|(link, _)| self.arena.completes_path(link))
                    {
                        sum
                    } else {
                        sum.saturating_add(firing.strength)
                    }
                });
                let opportunity = if self.protocol.integrates_current_opportunity() {
                    held.saturating_add(other)
                } else {
                    held
                };
                let (drive, participation, admitted) =
                    admitted_path_drive(&self.arena, &incidence.inputs, opportunity);
                let projected = held.saturating_add(other).saturating_add(admitted);
                (projected >= i64::from(target.threshold).saturating_mul(UNIT)).then(|| {
                    let unanswered = if policy == UnansweredReturnPolicy::Ignore {
                        Vec::new()
                    } else {
                        admitted_path_returns(
                            self,
                            incidence.junction,
                            &incidence.inputs,
                            admitted.signum(),
                        )
                    };
                    let mut origins = incidence
                        .inputs
                        .iter()
                        .map(|firing| firing.origin_physical)
                        .collect::<Vec<_>>();
                    origins.sort_unstable();
                    origins.dedup();
                    OutputCandidate {
                        incidence: index,
                        position: target.position,
                        drive,
                        participation,
                        unanswered,
                        origins,
                        consequence_tick: latest_consequence_tick(&self.arena, &incidence.inputs),
                    }
                })
            })
            .collect::<Vec<_>>();
        candidates.sort_by_key(|candidate| (candidate.position, candidate.incidence));

        let mut cursor = 0;
        let mut superseded = Vec::new();
        while cursor < candidates.len() {
            let mut end = cursor + 1;
            while end < candidates.len()
                && candidates[end]
                    .position
                    .saturating_sub(candidates[end - 1].position)
                    .abs()
                    <= LOCAL_VARIATION_RADIUS
            {
                end += 1;
            }
            if end - cursor > 1 {
                let group = &candidates[cursor..end];
                let ordinary = rank_candidate(group.iter(), self.tick, candidate_protocol);
                let fresh = group
                    .iter()
                    .filter(|candidate| candidate.unanswered.is_empty())
                    .collect::<Vec<_>>();
                let winner = if policy != UnansweredReturnPolicy::Ignore
                    && !ordinary.unanswered.is_empty()
                    && !fresh.is_empty()
                {
                    if policy == UnansweredReturnPolicy::Replace {
                        superseded.extend(ordinary.unanswered.iter().copied());
                    }
                    rank_candidate(fresh.into_iter(), self.tick, candidate_protocol).incidence
                } else {
                    ordinary.incidence
                };
                let recent_cohort = candidate_protocol
                    .then(|| recent_cohort(group.iter(), self.tick))
                    .flatten();
                for candidate in group {
                    let admitted = recent_cohort
                        .as_ref()
                        .is_some_and(|cohort| cohort.contains(&candidate.incidence))
                        || (recent_cohort.is_none() && candidate.incidence == winner);
                    if self.trace_physics && candidate_protocol {
                        run.trace.push(PhysicalTransition {
                            tick: self.tick,
                            phase: moment.phase,
                            event: PhysicalEvent::CandidateSelection {
                                target: moment.incidences[candidate.incidence].junction,
                                origin_scope: None,
                                consequence_tick: candidate.consequence_tick,
                                admitted,
                            },
                        });
                    }
                    if !admitted {
                        moment.incidences[candidate.incidence].inputs.clear();
                        run.work.total = run.work.total.saturating_add(1);
                    }
                }
            }
            cursor = end;
        }
        for link in superseded {
            self.return_outcome(link);
            run.work.total = run.work.total.saturating_add(1);
            run.work.physical_deallocations = run.work.physical_deallocations.saturating_add(1);
            if self.trace_physics {
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::ReturnSuperseded { link },
                });
            }
        }
    }

    fn compete_causal_origins(&mut self, moment: &mut Moment, run: &mut RunState) {
        let mut candidates = moment
            .incidences
            .iter()
            .enumerate()
            .filter_map(|(incidence, value)| {
                if value.inputs.is_empty() || !self.arena.is_output_junction(value.junction) {
                    return None;
                }
                let mut origins = value
                    .inputs
                    .iter()
                    .map(|firing| firing.origin_physical)
                    .collect::<Vec<_>>();
                origins.sort_unstable();
                origins.dedup();
                let held = self
                    .arena
                    .activation
                    .get(value.junction.0 as usize)
                    .copied()
                    .unwrap_or(0);
                let other = value.inputs.iter().fold(0_i64, |sum, firing| {
                    if firing
                        .link
                        .is_some_and(|(link, _)| self.arena.completes_path(link))
                    {
                        sum
                    } else {
                        sum.saturating_add(firing.strength)
                    }
                });
                let (drive, participation, _) = admitted_path_drive(
                    &self.arena,
                    &value.inputs,
                    if self.protocol.integrates_current_opportunity() {
                        held.saturating_add(other)
                    } else {
                        held
                    },
                );
                Some(OutputCandidate {
                    incidence,
                    position: self
                        .arena
                        .junction_by_id(value.junction)
                        .map(|junction| junction.position)
                        .unwrap_or_default(),
                    drive,
                    participation,
                    unanswered: Vec::new(),
                    origins,
                    consequence_tick: latest_consequence_tick(&self.arena, &value.inputs),
                })
            })
            .collect::<Vec<_>>();
        candidates.sort_by_key(|candidate| {
            (
                candidate.origins.first().copied().unwrap_or(u64::MAX),
                candidate.position,
                candidate.incidence,
            )
        });
        let mut cursor = 0;
        while cursor < candidates.len() {
            let origin = candidates[cursor]
                .origins
                .first()
                .copied()
                .unwrap_or(u64::MAX);
            let mut end = cursor + 1;
            while end < candidates.len()
                && candidates[end].origins.first().copied().unwrap_or(u64::MAX) == origin
            {
                end += 1;
            }
            let group = &candidates[cursor..end];
            if group.len() > 1 {
                let cohort = recent_cohort(group.iter(), self.tick);
                let winner = rank_candidate(group.iter(), self.tick, true).incidence;
                for candidate in group {
                    let admitted = cohort
                        .as_ref()
                        .is_some_and(|members| members.contains(&candidate.incidence))
                        || (cohort.is_none() && candidate.incidence == winner);
                    if self.trace_physics {
                        run.trace.push(PhysicalTransition {
                            tick: self.tick,
                            phase: moment.phase,
                            event: PhysicalEvent::CandidateSelection {
                                target: moment.incidences[candidate.incidence].junction,
                                origin_scope: Some(origin),
                                consequence_tick: candidate.consequence_tick,
                                admitted,
                            },
                        });
                    }
                    if !admitted {
                        moment.incidences[candidate.incidence].inputs.clear();
                        run.work.total = run.work.total.saturating_add(1);
                    }
                }
            }
            cursor = end;
        }
    }

    /// Choose among opposite paths before their strengths can cancel.
    fn choose(
        &mut self,
        target: JunctionId,
        firings: &mut Vec<Firing>,
        work: &mut Work,
        trace: &mut Vec<PhysicalTransition>,
        phase: i32,
    ) {
        let choices = firings
            .iter()
            .filter_map(|firing| {
                let link = firing.link?.0;
                let sign = firing.strength.signum() as i8;
                (self.arena.completes_path(link) && sign != 0).then_some((
                    link,
                    sign,
                    firing.strength.unsigned_abs(),
                ))
            })
            .collect::<Vec<_>>();
        if choices.is_empty() {
            return;
        }
        let positive = choices
            .iter()
            .filter(|(_, sign, _)| *sign > 0)
            .fold(0_u64, |sum, (_, _, strength)| sum.saturating_add(*strength));
        let negative = choices
            .iter()
            .filter(|(_, sign, _)| *sign < 0)
            .fold(0_u64, |sum, (_, _, strength)| sum.saturating_add(*strength));
        let opposing = positive > 0 && negative > 0;
        let held_opportunity = self
            .arena
            .activation
            .get(target.0 as usize)
            .copied()
            .unwrap_or(0)
            > 0;
        let current_opportunity = self.protocol.integrates_current_opportunity()
            && firings.iter().any(|firing| {
                firing
                    .link
                    .is_none_or(|(link, _)| !self.arena.completes_path(link))
                    && firing.strength > 0
            });
        let opportunity = held_opportunity || current_opportunity;
        let admitted = if positive > negative {
            Some(1)
        } else if negative > positive {
            Some(-1)
        } else if opposing && opportunity {
            Some(1)
        } else {
            None
        };
        let suppressed = choices
            .iter()
            .filter_map(|(link, sign, _)| (opposing && Some(*sign) != admitted).then_some(*link))
            .collect::<HashSet<_>>();
        firings.retain(|firing| {
            firing
                .link
                .is_none_or(|(link, _)| !suppressed.contains(&link))
        });
        for (link, sign, _) in choices {
            if !opposing || Some(sign) == admitted {
                self.reuse(link);
            }
        }
        work.total = work
            .total
            .saturating_add(u64::try_from(suppressed.len()).unwrap_or(u64::MAX));
        if opposing && self.trace_physics {
            trace.push(PhysicalTransition {
                tick: self.tick,
                phase,
                event: PhysicalEvent::PathChosen {
                    target,
                    positive_strength: positive,
                    negative_strength: negative,
                    opportunity_active: opportunity,
                    admitted_sign: admitted.unwrap_or(0),
                },
            });
        }
    }
}

fn rank_candidate<'a>(
    candidates: impl Iterator<Item = &'a OutputCandidate>,
    tick: i64,
    candidate_protocol: bool,
) -> &'a OutputCandidate {
    let candidates = candidates.collect::<Vec<_>>();
    if candidate_protocol {
        if let Some(latest) = candidates
            .iter()
            .filter_map(|candidate| candidate.consequence_tick)
            .filter(|consequence| tick.saturating_sub(*consequence) <= RECENT_ELIGIBILITY_TICKS)
            .max()
        {
            let recent = candidates
                .iter()
                .copied()
                .filter(|candidate| candidate.consequence_tick == Some(latest))
                .collect::<Vec<_>>();
            if recent.len() == 1 {
                return recent[0];
            }
        }
    }
    let strongest = candidates
        .iter()
        .map(|candidate| candidate.drive)
        .max()
        .unwrap_or(0);
    let least_used = candidates
        .iter()
        .filter(|candidate| candidate.drive == strongest)
        .map(|candidate| candidate.participation)
        .min()
        .unwrap_or(0);
    let tied = candidates
        .into_iter()
        .filter(|candidate| candidate.drive == strongest && candidate.participation == least_used)
        .collect::<Vec<_>>();
    let rotation = usize::try_from(tick.unsigned_abs()).unwrap_or(usize::MAX);
    tied[rotation % tied.len()]
}

fn recent_cohort<'a>(
    candidates: impl Iterator<Item = &'a OutputCandidate>,
    tick: i64,
) -> Option<HashSet<usize>> {
    let candidates = candidates.collect::<Vec<_>>();
    let latest = candidates
        .iter()
        .filter_map(|candidate| candidate.consequence_tick)
        .filter(|consequence| tick.saturating_sub(*consequence) <= RECENT_ELIGIBILITY_TICKS)
        .max()?;
    let cohort = candidates
        .into_iter()
        .filter(|candidate| candidate.consequence_tick == Some(latest))
        .map(|candidate| candidate.incidence)
        .collect::<HashSet<_>>();
    (cohort.len() > 1).then_some(cohort)
}

fn latest_consequence_tick(arena: &Arena, firings: &[Firing]) -> Option<i64> {
    firings
        .iter()
        .filter_map(|firing| firing.link.map(|(link, _)| link))
        .filter_map(|link| arena.link_by_id(link))
        .filter_map(|link| link.last_consequence_tick)
        .max()
}

fn admitted_path_returns(
    body: &Body,
    output: JunctionId,
    firings: &[Firing],
    admitted_sign: i64,
) -> Vec<LinkId> {
    let Some(source) = body.outcome_source_for_output(output) else {
        return Vec::new();
    };
    let waiting_junctions = firings
        .iter()
        .filter(|firing| firing.strength.signum() == admitted_sign)
        .filter_map(|firing| {
            firing
                .link
                .and_then(|(link, _)| body.arena.path_for_second(link))
        })
        .map(|path| path.junction)
        .collect::<HashSet<_>>();
    body.arena
        .return_links(&[source])
        .into_iter()
        .filter(|link| {
            body.arena.link_by_id(*link).is_some_and(|state| {
                waiting_junctions.contains(&state.to)
                    && (!body.protocol.is_sensorimotor() || state.return_origins.is_empty())
            })
        })
        .collect()
}

fn admitted_path_drive(arena: &Arena, firings: &[Firing], opportunity: i64) -> (u64, u64, i64) {
    let positive = firings
        .iter()
        .filter(|firing| {
            firing
                .link
                .is_some_and(|(link, _)| arena.completes_path(link))
                && firing.strength > 0
        })
        .fold(0_i64, |sum, firing| sum.saturating_add(firing.strength));
    let negative = firings
        .iter()
        .filter(|firing| {
            firing
                .link
                .is_some_and(|(link, _)| arena.completes_path(link))
                && firing.strength < 0
        })
        .fold(0_i64, |sum, firing| sum.saturating_add(firing.strength));
    let positive_magnitude = positive.unsigned_abs();
    let negative_magnitude = negative.unsigned_abs();
    if positive_magnitude > negative_magnitude {
        (
            positive_magnitude,
            path_participation(arena, firings, 1),
            positive,
        )
    } else if negative_magnitude > positive_magnitude {
        (
            negative_magnitude,
            path_participation(arena, firings, -1),
            negative,
        )
    } else if positive_magnitude > 0 && negative_magnitude > 0 && opportunity > 0 {
        (
            positive_magnitude,
            path_participation(arena, firings, 1),
            positive,
        )
    } else {
        (0, 0, 0)
    }
}

fn path_participation(arena: &Arena, firings: &[Firing], sign: i64) -> u64 {
    firings
        .iter()
        .filter(|firing| firing.strength.signum() == sign)
        .filter_map(|firing| firing.link.map(|(link, _)| link))
        .filter(|link| arena.completes_path(*link))
        .filter_map(|link| arena.link_by_id(link))
        .fold(0_u64, |sum, link| {
            sum.saturating_add(link.participation_level)
        })
}
