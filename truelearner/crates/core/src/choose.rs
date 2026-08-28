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
    competition_component: u64,
    drive: u64,
    participation: u64,
    unanswered: Vec<LinkId>,
    origins: Vec<u64>,
    causal_path_origins: Vec<u64>,
    owner: Option<LearnerId>,
    consequence_tick: Option<i64>,
    held_consequence_tick: Option<i64>,
    current_owner_transition: bool,
    latest_unanswered_opened_tick: Option<i64>,
}

struct CompletedCycleResolution<'a> {
    state: CompletedCycleState,
    winner: Option<&'a OutputCandidate>,
    winner_tick: Option<i64>,
}

struct BlockedOutputCandidate {
    incidence: usize,
    position: i32,
    owner: Option<LearnerId>,
    positive_path_strength: u64,
    positive_participation: u64,
    base_drive: i64,
    threshold: i64,
}

struct OwnerCandidateGroup {
    owner: LearnerId,
    inputs: Vec<Firing>,
    path_inputs: u32,
    drive: u64,
    participation: u64,
    consequence_tick: Option<i64>,
    executable: bool,
}

struct OriginCandidateGroup {
    origin_physical: u64,
    ownership: CandidateOwnership,
    inputs: Vec<Firing>,
    path_inputs: u32,
    drive: u64,
    participation: u64,
    consequence_tick: Option<i64>,
    executable: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ReturnSchedulingDecision {
    owner: Option<LearnerId>,
    link: LinkId,
    generation: u32,
    admitted: bool,
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
        if self.protocol.factors_candidate_owners() {
            self.factor_mixed_owner_candidates(moment, run);
        } else if self.protocol.factors_candidate_origins() {
            self.factor_causal_origin_candidates(moment, run);
        }
        self.compete_local_outputs(moment, run, policy, candidate_protocol);
        if candidate_protocol {
            self.compete_causal_origins(moment, run);
        }
        for incidence in &mut moment.incidences {
            if incidence.supplied_opportunity > 0 && !incidence.inputs.is_empty() {
                if let Some(activation) =
                    self.arena.activation.get_mut(incidence.junction.0 as usize)
                {
                    *activation = activation.saturating_add(incidence.supplied_opportunity);
                    run.work.total = run.work.total.saturating_add(1);
                }
            }
            self.choose(
                incidence.junction,
                &mut incidence.inputs,
                incidence.supplied_opportunity,
                &mut run.work,
                &mut run.trace,
                moment.phase,
            );
        }
    }

    fn factor_causal_origin_candidates(&mut self, moment: &mut Moment, run: &mut RunState) {
        for incidence in &mut moment.incidences {
            if !self.arena.is_output_junction(incidence.junction) {
                continue;
            }
            let origins = incidence
                .inputs
                .iter()
                .filter(|firing| {
                    firing
                        .link
                        .is_some_and(|(link, _)| self.arena.completes_path(link))
                        && firing.strength != 0
                })
                .map(|firing| firing.origin_physical)
                .collect::<BTreeSet<_>>();
            if origins.len() < 2 {
                continue;
            }
            let target = self
                .arena
                .junction_snapshot(self.arena.junction_slot(incidence.junction).unwrap().0);
            let threshold = i64::from(target.threshold).saturating_mul(UNIT);
            let held = self
                .arena
                .activation
                .get(incidence.junction.0 as usize)
                .copied()
                .unwrap_or(0);
            let mut groups = origins
                .iter()
                .map(|origin_physical| {
                    let inputs = incidence
                        .inputs
                        .iter()
                        .filter(|firing| firing.origin_physical == *origin_physical)
                        .cloned()
                        .collect::<Vec<_>>();
                    let ownership = candidate_ownership(self, &inputs);
                    let owner = match ownership {
                        CandidateOwnership::Owned(owner) => Some(owner),
                        CandidateOwnership::Organism | CandidateOwnership::Ambiguous => None,
                    };
                    let other = inputs.iter().fold(0_i64, |sum, firing| {
                        let non_path = firing
                            .link
                            .is_none_or(|(link, _)| !self.arena.completes_path(link));
                        if non_path {
                            sum.saturating_add(firing.strength)
                        } else {
                            sum
                        }
                    });
                    let owned_opportunity = owner.map_or(0, |owner| {
                        inputs.iter().fold(0_i64, |sum, firing| {
                            let non_path = firing
                                .link
                                .is_none_or(|(link, _)| !self.arena.completes_path(link));
                            if non_path
                                && firing.strength > 0
                                && self.learner_owner_for_origin(firing.origin_physical)
                                    == Some(owner)
                            {
                                sum.saturating_add(firing.strength)
                            } else {
                                sum
                            }
                        })
                    });
                    let opportunity = match ownership {
                        CandidateOwnership::Owned(_) => owned_opportunity,
                        CandidateOwnership::Organism => held.saturating_add(other),
                        CandidateOwnership::Ambiguous => 0,
                    };
                    let (drive, participation, admitted) =
                        admitted_path_drive(&self.arena, &inputs, opportunity);
                    let projected = match ownership {
                        CandidateOwnership::Owned(_) => owned_opportunity.saturating_add(admitted),
                        CandidateOwnership::Organism => {
                            held.saturating_add(other).saturating_add(admitted)
                        }
                        CandidateOwnership::Ambiguous => 0,
                    };
                    let executable =
                        ownership != CandidateOwnership::Ambiguous && projected >= threshold;
                    let consequence_tick = executable
                        .then(|| latest_candidate_consequence_tick(self, owner, &inputs))
                        .flatten();
                    OriginCandidateGroup {
                        origin_physical: *origin_physical,
                        ownership,
                        path_inputs: u32::try_from(
                            inputs
                                .iter()
                                .filter(|firing| {
                                    firing
                                        .link
                                        .is_some_and(|(link, _)| self.arena.completes_path(link))
                                        && firing.strength != 0
                                })
                                .count(),
                        )
                        .unwrap_or(u32::MAX),
                        inputs,
                        drive,
                        participation,
                        consequence_tick,
                        executable,
                    }
                })
                .collect::<Vec<_>>();
            let executable = groups
                .iter()
                .enumerate()
                .filter_map(|(index, group)| group.executable.then_some(index))
                .collect::<Vec<_>>();
            let selected = executable.iter().copied().max_by(|left, right| {
                compare_origin_groups(&groups[*left], &groups[*right], self.tick)
            });
            let selected = selected.filter(|selected| {
                executable
                    .iter()
                    .filter(|other| {
                        compare_origin_groups(&groups[**other], &groups[*selected], self.tick)
                            .is_eq()
                    })
                    .count()
                    == 1
            });
            let selected_origin = selected.map(|index| groups[index].origin_physical);
            let selected_ownership = selected.map(|index| groups[index].ownership);
            let selected_path_inputs = selected.map_or(0, |index| groups[index].path_inputs);
            if self.trace_physics {
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::CausalOriginCandidateResolved {
                        target: incidence.junction,
                        origin_count: u32::try_from(groups.len()).unwrap_or(u32::MAX),
                        executable_groups: u32::try_from(executable.len()).unwrap_or(u32::MAX),
                        selected_origin,
                        selected_ownership,
                        selected_path_inputs,
                    },
                });
            }
            if let Some(selected) = selected {
                let before = incidence.inputs.len();
                incidence.inputs = std::mem::take(&mut groups[selected].inputs);
                run.work.total = run.work.total.saturating_add(
                    u64::try_from(before.saturating_sub(incidence.inputs.len()))
                        .unwrap_or(u64::MAX),
                );
            }
        }
    }

    fn factor_mixed_owner_candidates(&mut self, moment: &mut Moment, run: &mut RunState) {
        for incidence in &mut moment.incidences {
            if !self.arena.is_output_junction(incidence.junction) {
                continue;
            }
            let path_firings = incidence
                .inputs
                .iter()
                .filter(|firing| {
                    firing
                        .link
                        .is_some_and(|(link, _)| self.arena.completes_path(link))
                        && firing.strength != 0
                })
                .collect::<Vec<_>>();
            let owners = path_firings
                .iter()
                .map(|firing| self.learner_owner_for_origin(firing.origin_physical))
                .collect::<BTreeSet<_>>();
            if owners.len() < 2 || owners.contains(&None) {
                continue;
            }
            let target = self
                .arena
                .junction_snapshot(self.arena.junction_slot(incidence.junction).unwrap().0);
            let threshold = i64::from(target.threshold).saturating_mul(UNIT);
            let mut groups = owners
                .iter()
                .filter_map(|owner| *owner)
                .map(|owner| {
                    let inputs = incidence
                        .inputs
                        .iter()
                        .filter(|firing| {
                            self.learner_owner_for_origin(firing.origin_physical) == Some(owner)
                        })
                        .cloned()
                        .collect::<Vec<_>>();
                    let opportunity = inputs.iter().fold(0_i64, |sum, firing| {
                        let non_path = firing
                            .link
                            .is_none_or(|(link, _)| !self.arena.completes_path(link));
                        if non_path && firing.strength > 0 {
                            sum.saturating_add(firing.strength)
                        } else {
                            sum
                        }
                    });
                    let (drive, participation, admitted) =
                        admitted_path_drive(&self.arena, &inputs, opportunity);
                    let projected = opportunity.saturating_add(admitted);
                    let executable = projected >= threshold;
                    let consequence_tick = executable
                        .then(|| latest_candidate_consequence_tick(self, Some(owner), &inputs))
                        .flatten();
                    OwnerCandidateGroup {
                        owner,
                        path_inputs: u32::try_from(
                            inputs
                                .iter()
                                .filter(|firing| {
                                    firing
                                        .link
                                        .is_some_and(|(link, _)| self.arena.completes_path(link))
                                        && firing.strength != 0
                                })
                                .count(),
                        )
                        .unwrap_or(u32::MAX),
                        inputs,
                        drive,
                        participation,
                        consequence_tick,
                        executable,
                    }
                })
                .collect::<Vec<_>>();
            let executable = groups
                .iter()
                .enumerate()
                .filter_map(|(index, group)| group.executable.then_some(index))
                .collect::<Vec<_>>();
            let selected = executable.iter().copied().max_by(|left, right| {
                compare_owner_groups(&groups[*left], &groups[*right], self.tick)
            });
            let selected = selected.filter(|selected| {
                executable
                    .iter()
                    .filter(|other| {
                        compare_owner_groups(&groups[**other], &groups[*selected], self.tick)
                            .is_eq()
                    })
                    .count()
                    == 1
            });
            let selected_owner = selected.map(|index| groups[index].owner);
            let selected_path_inputs = selected.map_or(0, |index| groups[index].path_inputs);
            if self.trace_physics {
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::MixedOwnerCandidateResolved {
                        target: incidence.junction,
                        owner_count: u32::try_from(groups.len()).unwrap_or(u32::MAX),
                        executable_groups: u32::try_from(executable.len()).unwrap_or(u32::MAX),
                        selected_owner,
                        selected_path_inputs,
                    },
                });
            }
            if let Some(selected) = selected {
                let before = incidence.inputs.len();
                incidence.inputs = std::mem::take(&mut groups[selected].inputs);
                run.work.total = run.work.total.saturating_add(
                    u64::try_from(before.saturating_sub(incidence.inputs.len()))
                        .unwrap_or(u64::MAX),
                );
            }
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
        let mut return_scheduling = Vec::new();
        let mut proprioceptive_opportunities = Vec::new();
        let mut owned_ineligible = Vec::new();
        let mut blocked_candidates = Vec::new();
        let mut promoted = HashSet::new();
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
                let ownership = candidate_ownership(self, &incidence.inputs);
                let owner = match ownership {
                    CandidateOwnership::Owned(owner) => Some(owner),
                    CandidateOwnership::Organism | CandidateOwnership::Ambiguous => None,
                };
                let owned_opportunity = owner
                    .map(|owner| {
                        incidence.inputs.iter().fold(0_i64, |sum, firing| {
                            let non_path = firing
                                .link
                                .is_none_or(|(link, _)| !self.arena.completes_path(link));
                            if !non_path || firing.strength <= 0 {
                                return sum;
                            }
                            let admitted = self.learner_owner_for_origin(firing.origin_physical)
                                == Some(owner);
                            proprioceptive_opportunities.push((
                                owner,
                                incidence.junction,
                                firing.origin_physical,
                                admitted,
                            ));
                            if admitted {
                                sum.saturating_add(firing.strength)
                            } else {
                                sum
                            }
                        })
                    })
                    .unwrap_or(0);
                let mut opportunity = if owner.is_some() {
                    owned_opportunity
                } else if ownership == CandidateOwnership::Ambiguous {
                    0
                } else if self.protocol.integrates_current_opportunity() {
                    held.saturating_add(other)
                } else {
                    held
                };
                let (positive_path_strength, negative_path_strength) =
                    path_strengths(&self.arena, &incidence.inputs);
                let current_owner_transition =
                    current_owner_transition(self, owner, &incidence.inputs, self.tick);
                let (mut drive, mut participation, mut admitted) =
                    admitted_path_drive(&self.arena, &incidence.inputs, opportunity);
                let mut projected = if owner.is_some() {
                    owned_opportunity.saturating_add(admitted)
                } else if ownership == CandidateOwnership::Ambiguous {
                    0
                } else {
                    held.saturating_add(other).saturating_add(admitted)
                };
                let threshold = i64::from(target.threshold).saturating_mul(UNIT);
                let mut executable = projected >= threshold;
                let mut supplied_opportunity = 0;
                if admits_direct_transition_opportunity(
                    self.protocol,
                    executable,
                    current_owner_transition,
                    opportunity,
                    positive_path_strength,
                    negative_path_strength,
                ) {
                    supplied_opportunity = UNIT;
                    opportunity = supplied_opportunity;
                    (drive, participation, admitted) =
                        admitted_path_drive(&self.arena, &incidence.inputs, opportunity);
                    projected = if owner.is_some() {
                        owned_opportunity.saturating_add(admitted)
                    } else if ownership == CandidateOwnership::Ambiguous {
                        0
                    } else {
                        held.saturating_add(other).saturating_add(admitted)
                    };
                    executable = projected >= threshold;
                    if executable {
                        promoted.insert(index);
                    }
                }
                if self.protocol.supplies_fresh_opportunity()
                    && !executable
                    && ownership != CandidateOwnership::Ambiguous
                    && opportunity == 0
                    && positive_path_strength > 0
                    && positive_path_strength == negative_path_strength
                {
                    blocked_candidates.push(BlockedOutputCandidate {
                        incidence: index,
                        position: target.position,
                        owner,
                        positive_path_strength,
                        positive_participation: path_participation(
                            &self.arena,
                            &incidence.inputs,
                            1,
                        ),
                        base_drive: if owner.is_some() {
                            owned_opportunity
                        } else {
                            held.saturating_add(other)
                        },
                        threshold,
                    });
                }
                let consequence_tick = executable
                    .then(|| latest_candidate_consequence_tick(self, owner, &incidence.inputs))
                    .flatten();
                let current_owner_transition = executable && current_owner_transition;
                let unanswered = if !executable || policy == UnansweredReturnPolicy::Ignore {
                    Vec::new()
                } else {
                    let (unanswered, decisions) = admitted_path_returns(
                        self,
                        incidence.junction,
                        &incidence.inputs,
                        admitted.signum(),
                    );
                    return_scheduling.extend(decisions);
                    unanswered
                };
                let latest_unanswered_opened_tick = unanswered
                    .iter()
                    .filter_map(|link| self.arena.link_by_id(*link).map(|state| state.opened_tick))
                    .max();
                if self.trace_physics && candidate_protocol {
                    let path_firings = incidence
                        .inputs
                        .iter()
                        .filter(|firing| {
                            firing
                                .link
                                .is_some_and(|(link, _)| self.arena.completes_path(link))
                                && firing.strength != 0
                        })
                        .collect::<Vec<_>>();
                    let path_inputs = path_firings.len();
                    let distinct_path_origins = path_firings
                        .iter()
                        .map(|firing| firing.origin_physical)
                        .collect::<BTreeSet<_>>()
                        .len();
                    let distinct_path_owners = path_firings
                        .iter()
                        .filter_map(|firing| self.learner_owner_for_origin(firing.origin_physical))
                        .collect::<BTreeSet<_>>()
                        .len();
                    run.trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase: moment.phase,
                        event: PhysicalEvent::OutputCandidateEvaluated {
                            target: incidence.junction,
                            ownership,
                            path_inputs: u32::try_from(path_inputs).unwrap_or(u32::MAX),
                            distinct_path_origins: u32::try_from(distinct_path_origins)
                                .unwrap_or(u32::MAX),
                            distinct_path_owners: u32::try_from(distinct_path_owners)
                                .unwrap_or(u32::MAX),
                            positive_path_strength,
                            negative_path_strength,
                            opportunity,
                            supplied_opportunity,
                            admitted_drive: admitted,
                            projected_drive: projected,
                            threshold,
                            consequence_tick,
                            unanswered_returns: u32::try_from(unanswered.len()).unwrap_or(u32::MAX),
                            executable,
                        },
                    });
                }
                if ownership != CandidateOwnership::Organism && !executable {
                    owned_ineligible.push(index);
                }
                executable.then(|| {
                    let mut origins = incidence
                        .inputs
                        .iter()
                        .map(|firing| firing.origin_physical)
                        .collect::<Vec<_>>();
                    origins.sort_unstable();
                    origins.dedup();
                    let causal_path_origins = if self.protocol.admits_return_bearing_continuation()
                    {
                        causal_path_origins(&self.arena, &incidence.inputs)
                    } else {
                        Vec::new()
                    };
                    OutputCandidate {
                        incidence: index,
                        position: target.position,
                        competition_component: 0,
                        drive,
                        participation,
                        unanswered,
                        origins,
                        causal_path_origins,
                        owner,
                        consequence_tick,
                        held_consequence_tick: self
                            .protocol
                            .holds_construction_outcome_for_first_choice()
                            .then(|| {
                                latest_candidate_held_consequence_tick(
                                    self,
                                    owner,
                                    &incidence.inputs,
                                )
                            })
                            .flatten(),
                        current_owner_transition,
                        latest_unanswered_opened_tick,
                    }
                })
            })
            .collect::<Vec<_>>();
        for incidence in promoted.iter().copied() {
            moment.incidences[incidence].supplied_opportunity = UNIT;
        }
        if self.protocol.supplies_fresh_opportunity()
            && !candidates.is_empty()
            && !blocked_candidates.is_empty()
        {
            enum LocalCandidate {
                Executable(usize),
                Blocked(usize),
            }

            let (topology_components, topology_work) =
                if self.protocol.uses_causal_topology_opportunity_products() {
                    causal_topology_component_keys(self)
                } else {
                    (Vec::new(), 0)
                };
            run.work.total = run.work.total.saturating_add(topology_work);

            let mut local = candidates
                .iter()
                .enumerate()
                .map(|(index, candidate)| {
                    (
                        output_topology_component(
                            self,
                            moment.incidences[candidate.incidence].junction,
                            &topology_components,
                        ),
                        candidate.position,
                        candidate.incidence,
                        LocalCandidate::Executable(index),
                    )
                })
                .chain(
                    blocked_candidates
                        .iter()
                        .enumerate()
                        .map(|(index, candidate)| {
                            (
                                output_topology_component(
                                    self,
                                    moment.incidences[candidate.incidence].junction,
                                    &topology_components,
                                ),
                                candidate.position,
                                candidate.incidence,
                                LocalCandidate::Blocked(index),
                            )
                        }),
                )
                .collect::<Vec<_>>();
            local.sort_by_key(|(component, position, incidence, _)| {
                (*component, *position, *incidence)
            });

            let mut cursor = 0;
            while cursor < local.len() {
                let mut end = cursor + 1;
                while end < local.len()
                    && local[end].0 == local[end - 1].0
                    && local[end].1.saturating_sub(local[end - 1].1).abs() <= LOCAL_VARIATION_RADIUS
                {
                    end += 1;
                }
                let mut compatible = Vec::new();
                for (_, _, _, recipient) in &local[cursor..end] {
                    let LocalCandidate::Blocked(recipient) = recipient else {
                        continue;
                    };
                    let recipient_index = *recipient;
                    let recipient = &blocked_candidates[recipient_index];
                    for (_, _, _, donor) in &local[cursor..end] {
                        let LocalCandidate::Executable(donor) = donor else {
                            continue;
                        };
                        let donor_index = *donor;
                        let donor = &candidates[donor_index];
                        let donor_is_recent = donor.consequence_tick.is_some_and(|consequence| {
                            self.tick.saturating_sub(consequence) <= RECENT_ELIGIBILITY_TICKS
                        });
                        for return_link in donor.unanswered.iter().copied() {
                            let return_owner = self.return_memory_owner(return_link);
                            let ownership_relation = self
                                .fresh_opportunity_owner_relation(return_owner, recipient.owner);
                            let owner_compatible = self
                                .protocol
                                .admits_fresh_opportunity_relation(ownership_relation);
                            let decision = if donor_is_recent {
                                Some(FreshOpportunityDecision::RejectedRecentDonor)
                            } else if !owner_compatible {
                                Some(FreshOpportunityDecision::RejectedOwnerMismatch)
                            } else {
                                compatible.push((donor_index, recipient_index, return_link));
                                None
                            };
                            if let Some(decision) = decision {
                                if self.trace_physics && candidate_protocol {
                                    run.trace.push(PhysicalTransition {
                                        tick: self.tick,
                                        phase: moment.phase,
                                        event: PhysicalEvent::FreshOpportunityEvaluated {
                                            donor: moment.incidences[donor.incidence].junction,
                                            recipient: moment.incidences[recipient.incidence]
                                                .junction,
                                            return_link,
                                            return_owner,
                                            recipient_owner: recipient.owner,
                                            ownership_relation,
                                            decision,
                                        },
                                    });
                                }
                            }
                        }
                    }
                }
                let selected = compatible.into_iter().max_by(|left, right| {
                    let left_recipient = &blocked_candidates[left.1];
                    let right_recipient = &blocked_candidates[right.1];
                    left_recipient
                        .positive_path_strength
                        .cmp(&right_recipient.positive_path_strength)
                        .then_with(|| {
                            right_recipient
                                .positive_participation
                                .cmp(&left_recipient.positive_participation)
                        })
                        .then_with(|| right_recipient.incidence.cmp(&left_recipient.incidence))
                        .then_with(|| candidates[left.0].drive.cmp(&candidates[right.0].drive))
                        .then_with(|| {
                            candidates[right.0]
                                .incidence
                                .cmp(&candidates[left.0].incidence)
                        })
                });
                if let Some((donor_index, recipient_index, return_link)) = selected {
                    let donor_incidence = candidates[donor_index].incidence;
                    let recipient = &blocked_candidates[recipient_index];
                    let incidence = &moment.incidences[recipient.incidence];
                    let return_owner = self.return_memory_owner(return_link);
                    let ownership_relation =
                        self.fresh_opportunity_owner_relation(return_owner, recipient.owner);
                    let supplied_opportunity = UNIT;
                    let (drive, participation, admitted) =
                        admitted_path_drive(&self.arena, &incidence.inputs, supplied_opportunity);
                    let projected = recipient
                        .base_drive
                        .saturating_add(supplied_opportunity)
                        .saturating_add(admitted);
                    if projected >= recipient.threshold {
                        let (unanswered, decisions) = admitted_path_returns(
                            self,
                            incidence.junction,
                            &incidence.inputs,
                            admitted.signum(),
                        );
                        return_scheduling.extend(decisions);
                        let current_owner_transition = current_owner_transition(
                            self,
                            recipient.owner,
                            &incidence.inputs,
                            self.tick,
                        );
                        if admits_promoted_candidate(
                            self.protocol,
                            current_owner_transition,
                            unanswered.len(),
                        ) {
                            let consequence_tick = latest_candidate_consequence_tick(
                                self,
                                recipient.owner,
                                &incidence.inputs,
                            );
                            let mut origins = incidence
                                .inputs
                                .iter()
                                .map(|firing| firing.origin_physical)
                                .collect::<Vec<_>>();
                            origins.sort_unstable();
                            origins.dedup();
                            let causal_path_origins =
                                if self.protocol.admits_return_bearing_continuation() {
                                    causal_path_origins(&self.arena, &incidence.inputs)
                                } else {
                                    Vec::new()
                                };
                            let unanswered_return_count = unanswered.len();
                            let latest_unanswered_opened_tick = unanswered
                                .iter()
                                .filter_map(|link| {
                                    self.arena.link_by_id(*link).map(|state| state.opened_tick)
                                })
                                .max();
                            candidates.push(OutputCandidate {
                                incidence: recipient.incidence,
                                position: recipient.position,
                                competition_component: 0,
                                drive,
                                participation,
                                unanswered,
                                origins,
                                causal_path_origins,
                                owner: recipient.owner,
                                consequence_tick,
                                held_consequence_tick: self
                                    .protocol
                                    .holds_construction_outcome_for_first_choice()
                                    .then(|| {
                                        latest_candidate_held_consequence_tick(
                                            self,
                                            recipient.owner,
                                            &incidence.inputs,
                                        )
                                    })
                                    .flatten(),
                                current_owner_transition,
                                latest_unanswered_opened_tick,
                            });
                            promoted.insert(recipient.incidence);
                            if self.trace_physics && candidate_protocol {
                                if let Some(transition) = run.trace.iter_mut().rev().find(|transition| {
                                    transition.tick == self.tick
                                        && transition.phase == moment.phase
                                        && matches!(
                                            transition.event,
                                            PhysicalEvent::OutputCandidateEvaluated { target, .. }
                                                if target == incidence.junction
                                        )
                                }) {
                                    if let PhysicalEvent::OutputCandidateEvaluated {
                                        opportunity,
                                        supplied_opportunity: observed_supplied,
                                        admitted_drive,
                                        projected_drive,
                                        consequence_tick: observed_consequence,
                                        unanswered_returns,
                                        executable,
                                        ..
                                    } = &mut transition.event
                                    {
                                        *opportunity = opportunity.saturating_add(supplied_opportunity);
                                        *observed_supplied = supplied_opportunity;
                                        *admitted_drive = admitted;
                                        *projected_drive = projected;
                                        *observed_consequence = consequence_tick;
                                        *unanswered_returns = u32::try_from(unanswered_return_count)
                                            .unwrap_or(u32::MAX);
                                        *executable = true;
                                    }
                                }
                                run.trace.push(PhysicalTransition {
                                    tick: self.tick,
                                    phase: moment.phase,
                                    event: PhysicalEvent::FreshOpportunityTransferred {
                                        donor: moment.incidences[donor_incidence].junction,
                                        recipient: incidence.junction,
                                        return_link,
                                        owner: recipient.owner,
                                        opportunity: supplied_opportunity,
                                    },
                                });
                                run.trace.push(PhysicalTransition {
                                    tick: self.tick,
                                    phase: moment.phase,
                                    event: PhysicalEvent::FreshOpportunityEvaluated {
                                        donor: moment.incidences[donor_incidence].junction,
                                        recipient: incidence.junction,
                                        return_link,
                                        return_owner,
                                        recipient_owner: recipient.owner,
                                        ownership_relation,
                                        decision: FreshOpportunityDecision::Admitted,
                                    },
                                });
                            }
                            moment.incidences[recipient.incidence].supplied_opportunity =
                                supplied_opportunity;
                        } else if self.trace_physics && candidate_protocol {
                            run.trace.push(PhysicalTransition {
                                tick: self.tick,
                                phase: moment.phase,
                                event: PhysicalEvent::FreshOpportunityEvaluated {
                                    donor: moment.incidences[donor_incidence].junction,
                                    recipient: incidence.junction,
                                    return_link,
                                    return_owner,
                                    recipient_owner: recipient.owner,
                                    ownership_relation,
                                    decision: FreshOpportunityDecision::RejectedRecipientHasReturn,
                                },
                            });
                        }
                    } else if self.trace_physics && candidate_protocol {
                        run.trace.push(PhysicalTransition {
                            tick: self.tick,
                            phase: moment.phase,
                            event: PhysicalEvent::FreshOpportunityEvaluated {
                                donor: moment.incidences[donor_incidence].junction,
                                recipient: incidence.junction,
                                return_link,
                                return_owner,
                                recipient_owner: recipient.owner,
                                ownership_relation,
                                decision: FreshOpportunityDecision::RejectedBelowThreshold,
                            },
                        });
                    }
                }
                cursor = end;
            }
        }
        for incidence in owned_ineligible {
            if promoted.contains(&incidence) {
                continue;
            }
            moment.incidences[incidence].inputs.clear();
            run.work.total = run.work.total.saturating_add(1);
        }
        if self.trace_physics {
            for (owner, target, origin_physical, admitted) in proprioceptive_opportunities {
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::ProprioceptiveOpportunity {
                        owner,
                        target,
                        origin_physical,
                        admitted,
                    },
                });
            }
        }
        if self.trace_physics && self.protocol.constructs_learners() {
            return_scheduling.sort_unstable();
            return_scheduling.dedup();
            for decision in return_scheduling {
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::ReturnScheduling {
                        owner: decision.owner,
                        link: decision.link,
                        generation: decision.generation,
                        admitted: decision.admitted,
                    },
                });
            }
        }
        candidates.sort_by_key(|candidate| (candidate.position, candidate.incidence));
        if self.protocol.composes_independent_output_products() {
            let component_checks = if self.protocol.uses_causal_topology_output_products() {
                assign_causal_topology_product_components(self, moment, &mut candidates)
            } else {
                assign_causal_origin_product_components(
                    &mut candidates,
                    self.protocol.uses_causal_path_output_products(),
                )
            };
            run.work.total = run.work.total.saturating_add(component_checks);
            if self.trace_physics {
                let basis = if self.protocol.uses_causal_topology_output_products() {
                    OutputCompetitionBasis::CausalTopology
                } else if self.protocol.uses_causal_path_output_products() {
                    OutputCompetitionBasis::CausalPathOrigin
                } else {
                    OutputCompetitionBasis::ImmediateOrigin
                };
                for candidate in &candidates {
                    run.trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase: moment.phase,
                        event: PhysicalEvent::OutputCompetitionComponent {
                            target: moment.incidences[candidate.incidence].junction,
                            outcome_source: self.outcome_source_for_output(
                                moment.incidences[candidate.incidence].junction,
                            ),
                            component: (candidate.competition_component != u64::MAX)
                                .then_some(candidate.competition_component),
                            basis,
                        },
                    });
                }
            }
            candidates.sort_by_key(|candidate| {
                (
                    candidate.competition_component,
                    candidate.position,
                    candidate.incidence,
                )
            });
        }

        let mut cursor = 0;
        let mut superseded = Vec::new();
        while cursor < candidates.len() {
            let mut end = cursor + 1;
            while end < candidates.len()
                && candidates[end].competition_component
                    == candidates[end - 1].competition_component
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
                let continuation =
                    resolve_current_transition(self.protocol, group.iter(), |candidate| {
                        promoted.contains(&candidate.incidence)
                    });
                let coherent = group
                    .iter()
                    .filter(|candidate| {
                        candidate
                            .latest_unanswered_opened_tick
                            .is_some_and(|opened| {
                                self.tick.saturating_sub(opened) <= RECENT_ELIGIBILITY_TICKS
                            })
                    })
                    .collect::<Vec<_>>();
                let coherent = (coherent.len() == 1).then(|| coherent[0]);
                let completed_cycle_resolution = resolve_completed_cycle(
                    self.protocol.composes_completed_physical_cycle(),
                    group.iter(),
                    self.tick,
                );
                let completed_cycle = completed_cycle_resolution.winner;
                let mut held_consequences =
                    if self.protocol.holds_construction_outcome_for_first_choice() {
                        group
                            .iter()
                            .flat_map(|candidate| {
                                candidate_held_consequence_witnesses(
                                    self,
                                    candidate.owner,
                                    &moment.incidences[candidate.incidence].inputs,
                                )
                                .into_iter()
                                .map(
                                    |(owner, link, generation, consequence_tick)| {
                                        (
                                            moment.incidences[candidate.incidence].junction,
                                            owner,
                                            link,
                                            generation,
                                            consequence_tick,
                                        )
                                    },
                                )
                            })
                            .collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    };
                held_consequences.sort_unstable();
                held_consequences.dedup();
                let crosses_ownership_view = group.first().is_some_and(|first| {
                    group.iter().any(|candidate| candidate.owner != first.owner)
                });
                let fresh = group
                    .iter()
                    .filter(|candidate| candidate.unanswered.is_empty())
                    .collect::<Vec<_>>();
                let current_path_origin_count = group
                    .iter()
                    .flat_map(|candidate| candidate.causal_path_origins.iter().copied())
                    .collect::<BTreeSet<_>>()
                    .len();
                let no_change_fresh = admits_sample_release(
                    self.protocol,
                    continuation.is_some(),
                    group
                        .iter()
                        .any(|candidate| candidate.current_owner_transition),
                    current_path_origin_count,
                    policy != UnansweredReturnPolicy::Ignore,
                    !ordinary.unanswered.is_empty(),
                    !fresh.is_empty(),
                )
                .then(|| rank_candidate(fresh.iter().copied(), self.tick, candidate_protocol));
                let continuation_winner = self
                    .protocol
                    .continues_current_physical_transition()
                    .then_some(continuation)
                    .flatten();
                let coherent_winner = self
                    .protocol
                    .coheres_recent_unanswered_effect()
                    .then_some(coherent)
                    .flatten();
                let prioritized_completed_cycle = self
                    .protocol
                    .admits_return_bearing_continuation()
                    .then_some(completed_cycle)
                    .flatten()
                    .filter(|candidate| {
                        candidate.held_consequence_tick == completed_cycle_resolution.winner_tick
                    });
                let deferred_completed_cycle = if prioritized_completed_cycle.is_some() {
                    None
                } else {
                    completed_cycle
                };
                let (winner, computed_winner_basis) = if let Some(candidate) =
                    prioritized_completed_cycle
                {
                    (candidate.incidence, OutputChoiceBasis::CompletedCycle)
                } else if let Some(candidate) = continuation_winner {
                    (candidate.incidence, OutputChoiceBasis::CurrentTransition)
                } else if let Some(candidate) = no_change_fresh {
                    if policy == UnansweredReturnPolicy::Replace {
                        superseded.extend(ordinary.unanswered.iter().copied());
                    }
                    (candidate.incidence, OutputChoiceBasis::FreshAlternative)
                } else if let Some(candidate) = coherent_winner {
                    (candidate.incidence, OutputChoiceBasis::CoherentEffect)
                } else if let Some(candidate) = deferred_completed_cycle {
                    (candidate.incidence, OutputChoiceBasis::CompletedCycle)
                } else if policy != UnansweredReturnPolicy::Ignore
                    && !ordinary.unanswered.is_empty()
                    && !fresh.is_empty()
                {
                    if policy == UnansweredReturnPolicy::Replace {
                        superseded.extend(ordinary.unanswered.iter().copied());
                    }
                    (
                        rank_candidate(fresh.into_iter(), self.tick, candidate_protocol).incidence,
                        OutputChoiceBasis::FreshAlternative,
                    )
                } else {
                    (ordinary.incidence, OutputChoiceBasis::Ordinary)
                };
                let recent_cohort = candidate_protocol
                    .then(|| recent_cohort(group.iter(), self.tick))
                    .flatten();
                let admission_basis = if recent_cohort.is_some() {
                    OutputChoiceBasis::RecentCohort
                } else {
                    computed_winner_basis
                };
                let admitted = group
                    .iter()
                    .filter(|candidate| {
                        recent_cohort
                            .as_ref()
                            .is_some_and(|cohort| cohort.contains(&candidate.incidence))
                            || (recent_cohort.is_none() && candidate.incidence == winner)
                    })
                    .map(|candidate| OutputAdmission {
                        target: moment.incidences[candidate.incidence].junction,
                        owner: candidate.owner,
                    })
                    .collect::<Vec<_>>();
                if self.trace_physics && candidate_protocol {
                    let target = |candidate: &OutputCandidate| {
                        moment.incidences[candidate.incidence].junction
                    };
                    run.trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase: moment.phase,
                        event: PhysicalEvent::OutputChoiceResolved {
                            ordinary_target: target(ordinary),
                            current_transition_target: continuation_winner.map(target),
                            coherent_effect_target: coherent_winner.map(target),
                            completed_cycle_target: completed_cycle.map(target),
                            computed_winner_target: moment.incidences[winner].junction,
                            admitted,
                            computed_winner_basis,
                            admission_basis,
                            completed_cycle_state: completed_cycle_resolution.state,
                            crosses_ownership_view,
                        },
                    });
                }
                for candidate in group {
                    let admitted = recent_cohort
                        .as_ref()
                        .is_some_and(|cohort| cohort.contains(&candidate.incidence))
                        || (recent_cohort.is_none() && candidate.incidence == winner);
                    if self.trace_physics && candidate_protocol {
                        if self.protocol.continues_current_physical_transition() {
                            run.trace.push(PhysicalTransition {
                                tick: self.tick,
                                phase: moment.phase,
                                event: PhysicalEvent::PhysicalTransitionContinuationEvaluated {
                                    target: moment.incidences[candidate.incidence].junction,
                                    owner: candidate.owner,
                                    current_owner_transition: candidate.current_owner_transition,
                                    unanswered_returns: u32::try_from(candidate.unanswered.len())
                                        .unwrap_or(u32::MAX),
                                    admitted: continuation.is_some_and(|continuation| {
                                        continuation.incidence == candidate.incidence
                                    }),
                                },
                            });
                        }
                        if self.protocol.coheres_recent_unanswered_effect() {
                            run.trace.push(PhysicalTransition {
                                tick: self.tick,
                                phase: moment.phase,
                                event: PhysicalEvent::CoherentEffectEvaluated {
                                    target: moment.incidences[candidate.incidence].junction,
                                    owner: candidate.owner,
                                    latest_unanswered_opened_tick: candidate
                                        .latest_unanswered_opened_tick,
                                    unanswered_returns: u32::try_from(candidate.unanswered.len())
                                        .unwrap_or(u32::MAX),
                                    admitted: coherent.is_some_and(|coherent| {
                                        coherent.incidence == candidate.incidence
                                    }),
                                },
                            });
                        }
                        if self.protocol.composes_completed_physical_cycle() {
                            let consequence_witnesses = candidate.consequence_tick.map_or_else(
                                Vec::new,
                                |consequence_tick| {
                                    candidate_consequence_witnesses(
                                        self,
                                        candidate.owner,
                                        &moment.incidences[candidate.incidence].inputs,
                                        consequence_tick,
                                    )
                                },
                            );
                            run.trace.push(PhysicalTransition {
                                tick: self.tick,
                                phase: moment.phase,
                                event: PhysicalEvent::CompletedCycleContinuationEvaluated {
                                    target: moment.incidences[candidate.incidence].junction,
                                    owner: candidate.owner,
                                    consequence_tick: candidate.consequence_tick,
                                    consequence_witnesses: consequence_witnesses
                                        .into_iter()
                                        .map(|(link, generation)| (link, generation.0))
                                        .collect(),
                                    unique_latest_tick: completed_cycle_resolution.winner_tick,
                                    crosses_ownership_view,
                                    admitted: completed_cycle.is_some_and(|completed| {
                                        completed.incidence == candidate.incidence
                                    }),
                                },
                            });
                        }
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
                        if let Some(owner) = candidate.owner {
                            run.trace.push(PhysicalTransition {
                                tick: self.tick,
                                phase: moment.phase,
                                event: PhysicalEvent::LearnerCandidatePreference {
                                    owner,
                                    target: moment.incidences[candidate.incidence].junction,
                                    consequence_tick: candidate.consequence_tick,
                                    admitted,
                                },
                            });
                        }
                    }
                    if !admitted {
                        moment.incidences[candidate.incidence].inputs.clear();
                        run.work.total = run.work.total.saturating_add(1);
                    }
                }
                for (target, owner, link, generation, expected_tick) in held_consequences {
                    let Some(consequence_tick) =
                        self.consume_held_learner_consequence(owner, link, generation)
                    else {
                        continue;
                    };
                    debug_assert_eq!(consequence_tick, expected_tick);
                    run.work.total = run.work.total.saturating_add(1);
                    if self.trace_physics {
                        run.trace.push(PhysicalTransition {
                            tick: self.tick,
                            phase: moment.phase,
                            event: PhysicalEvent::ConstructionContinuationConsumed {
                                target,
                                owner,
                                link,
                                generation: generation.0,
                                consequence_tick,
                            },
                        });
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

    fn fresh_opportunity_owner_relation(
        &self,
        donor: Option<LearnerId>,
        recipient: Option<LearnerId>,
    ) -> LearnerOwnershipRelation {
        classify_learner_ownership_relation_with(donor, recipient, |owner| {
            self.learners
                .iter()
                .find(|learner| learner.id == owner)
                .map(|learner| learner.parent)
        })
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
                let owner = match candidate_ownership(self, &value.inputs) {
                    CandidateOwnership::Owned(owner) => Some(owner),
                    CandidateOwnership::Organism | CandidateOwnership::Ambiguous => None,
                };
                Some(OutputCandidate {
                    incidence,
                    position: self
                        .arena
                        .junction_by_id(value.junction)
                        .map(|junction| junction.position)
                        .unwrap_or_default(),
                    competition_component: 0,
                    drive,
                    participation,
                    unanswered: Vec::new(),
                    origins,
                    causal_path_origins: if self.protocol.admits_return_bearing_continuation() {
                        causal_path_origins(&self.arena, &value.inputs)
                    } else {
                        Vec::new()
                    },
                    owner,
                    consequence_tick: latest_candidate_consequence_tick(self, owner, &value.inputs),
                    held_consequence_tick: self
                        .protocol
                        .holds_construction_outcome_for_first_choice()
                        .then(|| latest_candidate_held_consequence_tick(self, owner, &value.inputs))
                        .flatten(),
                    current_owner_transition: current_owner_transition(
                        self,
                        owner,
                        &value.inputs,
                        self.tick,
                    ),
                    latest_unanswered_opened_tick: None,
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
                        if let Some(owner) = candidate.owner {
                            run.trace.push(PhysicalTransition {
                                tick: self.tick,
                                phase: moment.phase,
                                event: PhysicalEvent::LearnerCandidatePreference {
                                    owner,
                                    target: moment.incidences[candidate.incidence].junction,
                                    consequence_tick: candidate.consequence_tick,
                                    admitted,
                                },
                            });
                        }
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
        supplied_opportunity: i64,
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
        let ownership = candidate_ownership(self, firings);
        let owner = match ownership {
            CandidateOwnership::Owned(owner) => Some(owner),
            CandidateOwnership::Organism | CandidateOwnership::Ambiguous => None,
        };
        let owned_current_opportunity = owner.is_some_and(|owner| {
            firings.iter().any(|firing| {
                firing
                    .link
                    .is_none_or(|(link, _)| !self.arena.completes_path(link))
                    && firing.strength > 0
                    && self.learner_owner_for_origin(firing.origin_physical) == Some(owner)
            })
        });
        let opportunity = if owner.is_some() {
            owned_current_opportunity || supplied_opportunity > 0
        } else if ownership == CandidateOwnership::Ambiguous {
            false
        } else {
            held_opportunity || current_opportunity || supplied_opportunity > 0
        };
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

fn compare_owner_groups(
    left: &OwnerCandidateGroup,
    right: &OwnerCandidateGroup,
    tick: i64,
) -> std::cmp::Ordering {
    let recent = |group: &OwnerCandidateGroup| {
        group
            .consequence_tick
            .filter(|consequence| tick.saturating_sub(*consequence) <= RECENT_ELIGIBILITY_TICKS)
    };
    recent(left)
        .cmp(&recent(right))
        .then_with(|| left.drive.cmp(&right.drive))
        .then_with(|| right.participation.cmp(&left.participation))
}

fn compare_origin_groups(
    left: &OriginCandidateGroup,
    right: &OriginCandidateGroup,
    tick: i64,
) -> std::cmp::Ordering {
    let recent = |group: &OriginCandidateGroup| {
        group
            .consequence_tick
            .filter(|consequence| tick.saturating_sub(*consequence) <= RECENT_ELIGIBILITY_TICKS)
    };
    recent(left)
        .cmp(&recent(right))
        .then_with(|| left.drive.cmp(&right.drive))
        .then_with(|| right.participation.cmp(&left.participation))
}

fn current_owner_transition(
    body: &Body,
    owner: Option<LearnerId>,
    firings: &[Firing],
    tick: i64,
) -> bool {
    let Some(owner) = owner else {
        return false;
    };
    firings.iter().any(|firing| {
        firing
            .link
            .is_some_and(|(link, _)| body.arena.completes_path(link))
            && firing.causal_lineage.as_ref().is_some_and(|lineage| {
                lineage.origins().iter().copied().any(|origin| {
                    body.learner_owner_for_origin(origin) == Some(owner)
                        && lineage.transition_tick(origin).is_some_and(|transition| {
                            tick.saturating_sub(transition) <= RECENT_ELIGIBILITY_TICKS
                        })
                })
            })
    })
}

fn admits_promoted_candidate(
    protocol: Protocol,
    current_owner_transition: bool,
    unanswered_returns: usize,
) -> bool {
    unanswered_returns == 0
        || (protocol.admits_return_bearing_continuation()
            && current_owner_transition
            && unanswered_returns > 0)
}

fn admits_direct_transition_opportunity(
    protocol: Protocol,
    executable: bool,
    current_owner_transition: bool,
    opportunity: i64,
    positive_path_strength: u64,
    negative_path_strength: u64,
) -> bool {
    protocol.admits_return_bearing_continuation()
        && !executable
        && current_owner_transition
        && opportunity == 0
        && positive_path_strength > 0
        && positive_path_strength == negative_path_strength
}

fn admits_sample_release(
    protocol: Protocol,
    continuation_present: bool,
    current_owner_transition_present: bool,
    current_path_origin_count: usize,
    policy_enabled: bool,
    ordinary_has_return: bool,
    fresh_alternative_present: bool,
) -> bool {
    protocol.admits_return_bearing_continuation()
        && !continuation_present
        && !current_owner_transition_present
        && current_path_origin_count > 1
        && policy_enabled
        && ordinary_has_return
        && fresh_alternative_present
}

fn assign_causal_origin_product_components(
    candidates: &mut [OutputCandidate],
    use_causal_path_origins: bool,
) -> u64 {
    let mut parents = (0..candidates.len()).collect::<Vec<_>>();
    let mut checks = 0_u64;
    for left in 0..candidates.len() {
        for right in left + 1..candidates.len() {
            if candidates[right]
                .position
                .saturating_sub(candidates[left].position)
                .abs()
                > LOCAL_VARIATION_RADIUS
            {
                break;
            }
            checks = checks.saturating_add(1);
            if origins_overlap_or_unknown(
                product_origins(&candidates[left], use_causal_path_origins),
                product_origins(&candidates[right], use_causal_path_origins),
            ) {
                union_components(&mut parents, left, right);
            }
        }
    }

    let mut component_keys = BTreeMap::<usize, u64>::new();
    for (index, candidate) in candidates.iter().enumerate() {
        let root = find_component(&mut parents, index);
        let candidate_key = product_origins(candidate, use_causal_path_origins)
            .first()
            .copied()
            .unwrap_or(u64::MAX);
        component_keys
            .entry(root)
            .and_modify(|key| *key = (*key).min(candidate_key))
            .or_insert(candidate_key);
    }
    for (index, candidate) in candidates.iter_mut().enumerate() {
        let root = find_component(&mut parents, index);
        candidate.competition_component = component_keys[&root];
    }
    checks
}

fn assign_causal_topology_product_components(
    body: &Body,
    moment: &Moment,
    candidates: &mut [OutputCandidate],
) -> u64 {
    let (component_keys, mut work) = causal_topology_component_keys(body);
    for candidate in candidates {
        let output = moment.incidences[candidate.incidence].junction;
        candidate.competition_component = output_topology_component(body, output, &component_keys);
        work = work.saturating_add(1);
    }
    work
}

fn causal_topology_component_keys(body: &Body) -> (Vec<u64>, u64) {
    let mut parents = (0..body.arena.junction_slots.len()).collect::<Vec<_>>();
    let mut work = 0_u64;
    for link in body.arena.links.iter().filter(|link| link.live) {
        union_components(&mut parents, link.from.0 as usize, link.to.0 as usize);
        work = work.saturating_add(1);
    }

    let mut root_keys = BTreeMap::<usize, u64>::new();
    for junction in body.arena.junctions.iter().filter(|junction| junction.live) {
        let root = find_component(&mut parents, junction.id.0 as usize);
        root_keys
            .entry(root)
            .and_modify(|key| *key = (*key).min(junction.physical_id))
            .or_insert(junction.physical_id);
        work = work.saturating_add(1);
    }
    let mut component_keys = vec![u64::MAX; parents.len()];
    for junction in body.arena.junctions.iter().filter(|junction| junction.live) {
        let root = find_component(&mut parents, junction.id.0 as usize);
        component_keys[junction.id.0 as usize] = root_keys[&root];
        work = work.saturating_add(1);
    }
    (component_keys, work)
}

fn output_topology_component(body: &Body, output: JunctionId, component_keys: &[u64]) -> u64 {
    if component_keys.is_empty() {
        return 0;
    }
    body.outcome_source_for_output(output)
        .and_then(|source| component_keys.get(source.0 as usize).copied())
        .unwrap_or(u64::MAX)
}

fn product_origins(candidate: &OutputCandidate, use_causal_path_origins: bool) -> &[u64] {
    if use_causal_path_origins {
        &candidate.causal_path_origins
    } else {
        &candidate.origins
    }
}

fn origins_overlap_or_unknown(left: &[u64], right: &[u64]) -> bool {
    if left.is_empty() || right.is_empty() {
        return true;
    }
    let (mut left_index, mut right_index) = (0, 0);
    while left_index < left.len() && right_index < right.len() {
        match left[left_index].cmp(&right[right_index]) {
            std::cmp::Ordering::Less => left_index += 1,
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => right_index += 1,
        }
    }
    false
}

fn find_component(parents: &mut [usize], index: usize) -> usize {
    if parents[index] != index {
        parents[index] = find_component(parents, parents[index]);
    }
    parents[index]
}

fn union_components(parents: &mut [usize], left: usize, right: usize) {
    let left_root = find_component(parents, left);
    let right_root = find_component(parents, right);
    if left_root != right_root {
        let root = left_root.min(right_root);
        parents[left_root] = root;
        parents[right_root] = root;
    }
}

fn rank_candidate<'a>(
    candidates: impl Iterator<Item = &'a OutputCandidate>,
    tick: i64,
    candidate_protocol: bool,
) -> &'a OutputCandidate {
    let candidates = candidates.collect::<Vec<_>>();
    let contains_owned = candidates.iter().any(|candidate| candidate.owner.is_some());
    let common_owner = candidates
        .first()
        .and_then(|candidate| candidate.owner)
        .filter(|owner| {
            candidates
                .iter()
                .all(|candidate| candidate.owner == Some(*owner))
        });
    if candidate_protocol {
        if common_owner.is_some() || !contains_owned {
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
        if contains_owned {
            let rotation = usize::try_from(tick.unsigned_abs()).unwrap_or(usize::MAX);
            return candidates[rotation % candidates.len()];
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

fn resolve_completed_cycle<'a>(
    enabled: bool,
    candidates: impl Iterator<Item = &'a OutputCandidate>,
    tick: i64,
) -> CompletedCycleResolution<'a> {
    if !enabled {
        return CompletedCycleResolution {
            state: CompletedCycleState::NotApplicable,
            winner: None,
            winner_tick: None,
        };
    }
    let candidates = candidates.collect::<Vec<_>>();
    if !candidates.iter().any(|candidate| {
        candidate.consequence_tick.is_some() || candidate.held_consequence_tick.is_some()
    }) {
        return CompletedCycleResolution {
            state: CompletedCycleState::Missing,
            winner: None,
            winner_tick: None,
        };
    }
    let Some(latest) = candidates
        .iter()
        .filter_map(|candidate| eligible_completed_cycle_tick(candidate, tick))
        .max()
    else {
        return CompletedCycleResolution {
            state: CompletedCycleState::Stale,
            winner: None,
            winner_tick: None,
        };
    };
    let latest = candidates
        .into_iter()
        .filter(|candidate| eligible_completed_cycle_tick(candidate, tick) == Some(latest))
        .collect::<Vec<_>>();
    if latest.len() == 1 {
        CompletedCycleResolution {
            state: CompletedCycleState::Unique,
            winner: Some(latest[0]),
            winner_tick: eligible_completed_cycle_tick(latest[0], tick),
        }
    } else {
        CompletedCycleResolution {
            state: CompletedCycleState::AmbiguousLatest,
            winner: None,
            winner_tick: None,
        }
    }
}

fn resolve_current_transition<'a>(
    protocol: Protocol,
    candidates: impl Iterator<Item = &'a OutputCandidate>,
    is_promoted: impl Fn(&OutputCandidate) -> bool,
) -> Option<&'a OutputCandidate> {
    let current = candidates
        .filter(|candidate| candidate.current_owner_transition)
        .collect::<Vec<_>>();
    if protocol.admits_return_bearing_continuation() {
        let promoted = current
            .iter()
            .copied()
            .filter(|candidate| is_promoted(candidate))
            .collect::<Vec<_>>();
        if !promoted.is_empty() {
            return (promoted.len() == 1).then(|| promoted[0]);
        }
        let latest = current
            .iter()
            .filter_map(|candidate| candidate.consequence_tick)
            .max()?;
        let latest = current
            .into_iter()
            .filter(|candidate| candidate.consequence_tick == Some(latest))
            .collect::<Vec<_>>();
        return (latest.len() == 1).then(|| latest[0]);
    }
    let unanswered = current
        .into_iter()
        .filter(|candidate| !candidate.unanswered.is_empty())
        .collect::<Vec<_>>();
    (unanswered.len() == 1).then(|| unanswered[0])
}

fn eligible_completed_cycle_tick(candidate: &OutputCandidate, tick: i64) -> Option<i64> {
    candidate
        .consequence_tick
        .filter(|consequence| tick.saturating_sub(*consequence) <= RECENT_ELIGIBILITY_TICKS)
        .into_iter()
        .chain(candidate.held_consequence_tick)
        .max()
}

fn recent_cohort<'a>(
    candidates: impl Iterator<Item = &'a OutputCandidate>,
    tick: i64,
) -> Option<HashSet<usize>> {
    let candidates = candidates.collect::<Vec<_>>();
    let contains_owned = candidates.iter().any(|candidate| candidate.owner.is_some());
    if contains_owned {
        let owner = candidates.first()?.owner?;
        if candidates
            .iter()
            .any(|candidate| candidate.owner != Some(owner))
        {
            return None;
        }
    }
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

fn candidate_ownership(body: &Body, firings: &[Firing]) -> CandidateOwnership {
    if !body.protocol.constructs_learners() {
        return CandidateOwnership::Organism;
    }
    let owners = firings
        .iter()
        .filter(|firing| {
            firing
                .link
                .is_some_and(|(link, _)| body.arena.completes_path(link))
                && firing.strength != 0
        })
        .map(|firing| body.learner_owner_for_origin(firing.origin_physical))
        .collect::<Vec<_>>();
    let Some(first) = owners.first().copied() else {
        return CandidateOwnership::Organism;
    };
    if owners.iter().all(|owner| *owner == first) {
        first.map_or(CandidateOwnership::Organism, CandidateOwnership::Owned)
    } else {
        CandidateOwnership::Ambiguous
    }
}

fn latest_candidate_consequence_tick(
    body: &Body,
    owner: Option<LearnerId>,
    firings: &[Firing],
) -> Option<i64> {
    if let Some(owner) = owner {
        firings
            .iter()
            .filter_map(|firing| firing.link)
            .filter_map(|(link, generation)| body.learner_consequence_tick(owner, link, generation))
            .max()
    } else {
        latest_consequence_tick(&body.arena, firings)
    }
}

fn latest_candidate_held_consequence_tick(
    body: &Body,
    owner: Option<LearnerId>,
    firings: &[Firing],
) -> Option<i64> {
    let owner = owner?;
    firings
        .iter()
        .filter_map(|firing| firing.link)
        .filter(|(link, _)| body.arena.completes_path(*link))
        .filter_map(|(link, generation)| {
            body.held_learner_consequence_tick(owner, link, generation)
        })
        .max()
}

fn candidate_held_consequence_witnesses(
    body: &Body,
    owner: Option<LearnerId>,
    firings: &[Firing],
) -> Vec<(LearnerId, LinkId, Generation, i64)> {
    let Some(owner) = owner else {
        return Vec::new();
    };
    let mut witnesses = firings
        .iter()
        .filter_map(|firing| firing.link)
        .filter(|(link, _)| body.arena.completes_path(*link))
        .filter_map(|(link, generation)| {
            body.held_learner_consequence_tick(owner, link, generation)
                .map(|tick| (owner, link, generation, tick))
        })
        .collect::<Vec<_>>();
    witnesses.sort_unstable();
    witnesses.dedup();
    witnesses
}

fn candidate_consequence_witnesses(
    body: &Body,
    owner: Option<LearnerId>,
    firings: &[Firing],
    consequence_tick: i64,
) -> Vec<(LinkId, Generation)> {
    let mut witnesses = firings
        .iter()
        .filter_map(|firing| firing.link)
        .filter(|(link, _)| body.arena.completes_path(*link))
        .filter(|(link, generation)| match owner {
            Some(owner) => {
                body.learner_consequence_tick(owner, *link, *generation) == Some(consequence_tick)
            }
            None => body.arena.link_by_id(*link).is_some_and(|state| {
                state.live
                    && state.generation == *generation
                    && state.last_consequence_tick == Some(consequence_tick)
            }),
        })
        .collect::<Vec<_>>();
    witnesses.sort_unstable();
    witnesses.dedup();
    witnesses
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
) -> (Vec<LinkId>, Vec<ReturnSchedulingDecision>) {
    let Some(source) = body.outcome_source_for_output(output) else {
        return (Vec::new(), Vec::new());
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
    let decisions = body
        .arena
        .return_links(&[source])
        .into_iter()
        .filter_map(|link| {
            let state = body.arena.link_by_id(link)?;
            waiting_junctions.contains(&state.to).then(|| {
                let admitted = !body.protocol.is_sensorimotor() || body.return_is_available(link);
                ReturnSchedulingDecision {
                    owner: body.return_memory_owner(link),
                    link,
                    generation: state.generation.0,
                    admitted,
                }
            })
        })
        .collect::<Vec<_>>();
    let admitted = decisions
        .iter()
        .filter_map(|decision| decision.admitted.then_some(decision.link))
        .collect();
    (admitted, decisions)
}

fn admitted_path_drive(arena: &Arena, firings: &[Firing], opportunity: i64) -> (u64, u64, i64) {
    let (positive_magnitude, negative_magnitude) = path_strengths(arena, firings);
    let positive = i64::try_from(positive_magnitude).unwrap_or(i64::MAX);
    let negative = -i64::try_from(negative_magnitude).unwrap_or(i64::MAX);
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

fn path_strengths(arena: &Arena, firings: &[Firing]) -> (u64, u64) {
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
    (positive.unsigned_abs(), negative.unsigned_abs())
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

fn causal_path_origins(arena: &Arena, firings: &[Firing]) -> Vec<u64> {
    let mut origins = firings
        .iter()
        .filter(|firing| {
            firing
                .link
                .is_some_and(|(link, _)| arena.completes_path(link))
        })
        .flat_map(|firing| {
            firing
                .causal_lineage
                .as_ref()
                .map(|lineage| lineage.origins().to_vec())
                .unwrap_or_else(|| vec![firing.origin_physical])
        })
        .collect::<Vec<_>>();
    origins.sort_unstable();
    origins.dedup();
    origins
}

#[cfg(test)]
mod completed_cycle_tests {
    use super::*;

    fn junction(physical_id: u64, position: i32, region: i16) -> Junction {
        Junction {
            physical_id,
            position,
            region,
            threshold: 1,
            resistance: u32::MAX,
        }
    }

    fn link(from: JunctionId, to: JunctionId) -> Link {
        Link {
            from,
            to,
            delay: 0,
            phase: 0,
            coupling: 1,
            resistance: u32::MAX,
            mode: TransmissionMode::Drive,
        }
    }

    fn firing(target: JunctionId, link: LinkId, generation: Generation) -> Firing {
        Firing {
            arrival_tick: 0,
            phase: 0,
            causal_wave: 0,
            origin_physical: 1,
            causal_lineage: None,
            physical_incidence: PhysicalIncidence::Sample,
            target_physical: 3,
            target,
            target_generation: Generation(1),
            impulse: 1,
            strength: UNIT,
            serial: 0,
            link: Some((link, generation)),
        }
    }

    fn candidate(
        incidence: usize,
        owner: Option<LearnerId>,
        consequence_tick: Option<i64>,
    ) -> OutputCandidate {
        OutputCandidate {
            incidence,
            position: i32::try_from(incidence).unwrap_or(i32::MAX),
            competition_component: 0,
            drive: 1,
            participation: 0,
            unanswered: Vec::new(),
            origins: Vec::new(),
            causal_path_origins: Vec::new(),
            owner,
            consequence_tick,
            held_consequence_tick: None,
            current_owner_transition: false,
            latest_unanswered_opened_tick: None,
        }
    }

    fn held_candidate(
        incidence: usize,
        owner: LearnerId,
        consequence_tick: i64,
    ) -> OutputCandidate {
        OutputCandidate {
            held_consequence_tick: Some(consequence_tick),
            ..candidate(incidence, Some(owner), Some(consequence_tick))
        }
    }

    fn origin_candidate(incidence: usize, position: i32, origins: &[u64]) -> OutputCandidate {
        let mut candidate = candidate(incidence, None, None);
        candidate.position = position;
        candidate.origins = origins.to_vec();
        candidate
    }

    #[test]
    fn causal_origin_product_splits_disjoint_nearby_outputs() {
        let mut candidates = [
            origin_candidate(0, 0, &[10, 11]),
            origin_candidate(1, 1, &[20, 21]),
        ];
        let checks = assign_causal_origin_product_components(&mut candidates, false);
        assert_eq!(checks, 1);
        assert_ne!(
            candidates[0].competition_component,
            candidates[1].competition_component
        );
    }

    #[test]
    fn causal_origin_product_preserves_overlap_transitivity_and_unknowns() {
        let mut candidates = [
            origin_candidate(0, 0, &[10]),
            origin_candidate(1, 1, &[10, 20]),
            origin_candidate(2, 2, &[20]),
            origin_candidate(3, 3, &[]),
        ];
        assign_causal_origin_product_components(&mut candidates, false);
        assert!(candidates
            .iter()
            .all(|candidate| candidate.competition_component == 10));
    }

    #[test]
    fn causal_origin_product_is_invariant_to_candidate_order() {
        let mut forward = [origin_candidate(0, 0, &[10]), origin_candidate(1, 1, &[20])];
        let mut reverse = [origin_candidate(1, 0, &[10]), origin_candidate(0, 1, &[20])];
        assign_causal_origin_product_components(&mut forward, false);
        assign_causal_origin_product_components(&mut reverse, false);
        let forward_keys = forward
            .iter()
            .map(|candidate| (candidate.origins.clone(), candidate.competition_component))
            .collect::<BTreeMap<_, _>>();
        let reverse_keys = reverse
            .iter()
            .map(|candidate| (candidate.origins.clone(), candidate.competition_component))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(forward_keys, reverse_keys);
    }

    #[test]
    fn causal_path_product_uses_completed_path_origins_not_immediate_origins() {
        let mut candidates = [origin_candidate(0, 0, &[10]), origin_candidate(1, 1, &[20])];
        candidates[0].causal_path_origins = vec![30];
        candidates[1].causal_path_origins = vec![30];
        assign_causal_origin_product_components(&mut candidates, true);
        assert_eq!(
            candidates[0].competition_component,
            candidates[1].competition_component
        );
    }

    #[test]
    fn causal_topology_product_groups_connected_outcomes_and_splits_disconnected_ones() {
        let mut body = Body::default();
        let output_a = body.add_junction(junction(10, 0, 0));
        let output_b = body.add_junction(junction(11, 1, 0));
        let output_c = body.add_junction(junction(12, 2, 0));
        let sink_a = body.add_junction(junction(20, 0, 1));
        let sink_b = body.add_junction(junction(21, 1, 1));
        let sink_c = body.add_junction(junction(22, 2, 1));
        body.add_link(link(output_a, sink_a));
        body.add_link(link(output_b, sink_b));
        body.add_link(link(output_c, sink_c));

        let shared_anchor = body.add_junction(junction(30, 10, 0));
        let source_a = body.add_junction(junction(31, 11, 0));
        let source_b = body.add_junction(junction(32, 12, 0));
        let isolated_anchor = body.add_junction(junction(40, 20, 0));
        let source_c = body.add_junction(junction(41, 21, 0));
        body.add_link(link(shared_anchor, source_a));
        body.add_link(link(shared_anchor, source_b));
        body.add_link(link(isolated_anchor, source_c));
        body.set_outcome_source_for_output(output_a, source_a);
        body.set_outcome_source_for_output(output_b, source_b);
        body.set_outcome_source_for_output(output_c, source_c);

        let (components, work) = causal_topology_component_keys(&body);
        let component_a = output_topology_component(&body, output_a, &components);
        let component_b = output_topology_component(&body, output_b, &components);
        let component_c = output_topology_component(&body, output_c, &components);
        assert!(work > 0);
        assert_eq!(component_a, component_b);
        assert_ne!(component_a, component_c);
    }

    #[test]
    fn output_choice_resolution_classifies_completed_cycle_evidence() {
        let mixed_ownership = [
            candidate(0, None, Some(20)),
            candidate(1, Some(LearnerId(7)), None),
        ];
        let disabled = resolve_completed_cycle(false, mixed_ownership.iter(), 24);
        assert_eq!(disabled.state, CompletedCycleState::NotApplicable);
        assert!(disabled.winner.is_none());

        let missing = [candidate(0, None, None), candidate(1, None, None)];
        let missing = resolve_completed_cycle(true, missing.iter(), 24);
        assert_eq!(missing.state, CompletedCycleState::Missing);
        assert!(missing.winner.is_none());

        let unique = resolve_completed_cycle(true, mixed_ownership.iter(), 24);
        assert_eq!(unique.state, CompletedCycleState::Unique);
        assert_eq!(unique.winner.map(|winner| winner.incidence), Some(0));

        let ambiguous_latest = [
            candidate(0, None, Some(20)),
            candidate(1, Some(LearnerId(7)), Some(20)),
        ];
        let ambiguous = resolve_completed_cycle(true, ambiguous_latest.iter(), 24);
        assert_eq!(ambiguous.state, CompletedCycleState::AmbiguousLatest);
        assert!(ambiguous.winner.is_none());

        let stale = [
            candidate(0, None, Some(19)),
            candidate(1, Some(LearnerId(7)), None),
        ];
        let stale = resolve_completed_cycle(true, stale.iter(), 24);
        assert_eq!(stale.state, CompletedCycleState::Stale);
        assert!(stale.winner.is_none());
    }

    #[test]
    fn bounded_construction_continuation_is_eligible_only_for_its_first_choice() {
        let held = [
            held_candidate(0, LearnerId(7), 16),
            candidate(1, None, None),
        ];
        let first = resolve_completed_cycle(true, held.iter(), 23);
        assert_eq!(first.state, CompletedCycleState::Unique);
        assert_eq!(first.winner.map(|winner| winner.incidence), Some(0));

        let ordinary = [
            candidate(0, Some(LearnerId(7)), Some(16)),
            candidate(1, None, None),
        ];
        let second = resolve_completed_cycle(true, ordinary.iter(), 23);
        assert_eq!(second.state, CompletedCycleState::Stale);
        assert!(second.winner.is_none());

        let ambiguous = [
            held_candidate(0, LearnerId(7), 16),
            held_candidate(1, LearnerId(8), 16),
        ];
        let ambiguous = resolve_completed_cycle(true, ambiguous.iter(), 23);
        assert_eq!(ambiguous.state, CompletedCycleState::AmbiguousLatest);
        assert!(ambiguous.winner.is_none());
    }

    #[test]
    fn return_bearing_continuation_requires_opt_in_current_transition_and_return() {
        assert!(admits_promoted_candidate(
            Protocol::RecursiveLearnerBoundedConstructionContinuation,
            false,
            0
        ));
        assert!(!admits_promoted_candidate(
            Protocol::RecursiveLearnerBoundedConstructionContinuation,
            true,
            1
        ));
        assert!(!admits_promoted_candidate(
            Protocol::RecursiveLearnerReturnBearingContinuation,
            false,
            1
        ));
        assert!(admits_promoted_candidate(
            Protocol::RecursiveLearnerReturnBearingContinuation,
            true,
            1
        ));
    }

    #[test]
    fn return_bearing_continuation_prefers_promoted_then_unique_latest_current_path() {
        let owner = Some(LearnerId(7));
        let mut promoted = candidate(0, owner, None);
        promoted.current_owner_transition = true;
        promoted.unanswered.push(LinkId(1));
        let mut old = candidate(1, owner, Some(32));
        old.current_owner_transition = true;
        old.unanswered.push(LinkId(2));
        assert_eq!(
            resolve_current_transition(
                Protocol::RecursiveLearnerReturnBearingContinuation,
                [&promoted, &old].into_iter(),
                |candidate| candidate.incidence == 0
            )
            .map(|candidate| candidate.incidence),
            Some(0)
        );

        let mut newer = candidate(0, owner, Some(56));
        newer.current_owner_transition = true;
        assert_eq!(
            resolve_current_transition(
                Protocol::RecursiveLearnerReturnBearingContinuation,
                [&newer, &old].into_iter(),
                |_| false
            )
            .map(|candidate| candidate.incidence),
            Some(0)
        );

        old.consequence_tick = Some(56);
        assert!(resolve_current_transition(
            Protocol::RecursiveLearnerReturnBearingContinuation,
            [&newer, &old].into_iter(),
            |_| false
        )
        .is_none());
    }

    #[test]
    fn return_bearing_continuation_rejects_samples_unbalanced_paths_and_single_origin_release() {
        let protocol = Protocol::RecursiveLearnerReturnBearingContinuation;
        assert!(admits_direct_transition_opportunity(
            protocol, false, true, 0, 1, 1
        ));
        assert!(!admits_direct_transition_opportunity(
            protocol, false, false, 0, 1, 1
        ));
        assert!(!admits_direct_transition_opportunity(
            protocol, false, true, 0, 2, 1
        ));
        assert!(!admits_direct_transition_opportunity(
            Protocol::RecursiveLearnerBoundedConstructionContinuation,
            false,
            true,
            0,
            1,
            1
        ));

        assert!(admits_sample_release(
            protocol, false, false, 2, true, true, true
        ));
        assert!(!admits_sample_release(
            protocol, false, false, 1, true, true, true
        ));
        assert!(!admits_sample_release(
            protocol, false, true, 2, true, true, true
        ));
    }

    #[test]
    fn completed_cycle_consequence_witnesses_select_live_completing_exact_generation() {
        let mut body = Body::with_capacity(8, 8);
        let source = body.add_junction(junction(1, 0, 0));
        let middle = body.add_junction(junction(2, 0, 0));
        let motor = body.add_junction(junction(3, 1, 0));
        let outside = body.add_junction(junction(4, 1, 1));
        let non_completing = body.add_link(link(source, middle));
        let completing = body.add_link(link(middle, motor));
        body.add_link(link(motor, outside));
        let generation = body.arena.link_by_id(completing).unwrap().generation;
        let unrelated_generation = body.arena.link_by_id(non_completing).unwrap().generation;
        let owner = LearnerId(7);
        body.learners.push(LearnerState {
            id: owner,
            parent: None,
            surface: source,
            output: motor,
            junctions: vec![source, middle, motor],
            links: vec![non_completing, completing],
            return_memory: Vec::new(),
            consequence_memory: vec![
                LearnerConsequenceMemory {
                    link: non_completing,
                    generation: unrelated_generation,
                    last_consequence_tick: 44,
                    lifetime: ConsequenceLifetime::Ordinary,
                },
                LearnerConsequenceMemory {
                    link: completing,
                    generation,
                    last_consequence_tick: 44,
                    lifetime: ConsequenceLifetime::HeldForFirstChoice,
                },
            ],
        });
        let inputs = [
            firing(motor, non_completing, unrelated_generation),
            firing(motor, completing, generation),
        ];

        assert_eq!(
            candidate_consequence_witnesses(&body, Some(owner), &inputs, 44),
            vec![(completing, generation)]
        );
        assert_eq!(
            candidate_held_consequence_witnesses(&body, Some(owner), &inputs),
            vec![(owner, completing, generation, 44)]
        );
        assert!(candidate_consequence_witnesses(
            &body,
            Some(owner),
            &[firing(
                motor,
                completing,
                Generation(generation.0.saturating_add(1))
            )],
            44
        )
        .is_empty());
        assert!(candidate_consequence_witnesses(&body, Some(owner), &inputs, 43).is_empty());

        let slot = body.arena.link_slot(completing).unwrap();
        body.arena.edit_link(slot.0, |state| state.live = false);
        assert!(candidate_consequence_witnesses(&body, Some(owner), &inputs, 44).is_empty());
    }
}
