use crate::prelude::*;

#[derive(Clone, Copy)]
struct AcceptedReturn {
    owner: Option<LearnerId>,
}

#[derive(Clone, Copy)]
struct ReturnOriginTiming {
    birth_tick: i64,
    transition_tick: Option<i64>,
}

impl Arena {
    pub(crate) fn return_links(&self, outcomes: &[JunctionId]) -> Vec<LinkId> {
        outcomes
            .iter()
            .flat_map(|outcome| {
                self.outgoing_index
                    .get(outcome.0 as usize)
                    .into_iter()
                    .flatten()
                    .copied()
            })
            .filter(|id| {
                self.link_by_id(*id).is_some_and(|link| {
                    link.live
                        && link.mode == TransmissionMode::Modulatory
                        && self.is_path_junction(link.to)
                })
            })
            .collect()
    }
}

impl Body {
    pub(crate) fn close_natural_physical_cycle(
        &mut self,
        fired: &Fired,
        moment: &Moment,
        run: &mut RunState,
    ) -> bool {
        if !self.protocol.closes_natural_physical_cycles() {
            return false;
        }

        let Some(lineage) = fired.causal_lineage.as_ref() else {
            self.trace_natural_cycle_closure(
                fired.junction,
                0,
                NaturalCycleClosureDecision::NoTransition,
                moment,
                run,
            );
            return false;
        };
        let transition_origins = lineage
            .origins()
            .iter()
            .copied()
            .filter(|origin| lineage.transition_tick(*origin).is_some())
            .collect::<BTreeSet<_>>();
        if transition_origins.is_empty() {
            self.trace_natural_cycle_closure(
                fired.junction,
                0,
                NaturalCycleClosureDecision::NoTransition,
                moment,
                run,
            );
            return false;
        }

        let matches = self
            .arena
            .paths_from(fired.junction)
            .into_iter()
            .filter_map(|path| {
                let first = self.arena.link_by_id(path.first)?;
                let second = self.arena.link_by_id(path.second)?;
                let output = self.arena.junction_by_id(second.to)?;
                let transition_tick = lineage.transition_tick(output.physical_id)?;
                (transition_origins.contains(&output.physical_id)
                    && first.participation_level > 0
                    && second.participation_level > 0
                    && transition_tick > first.opened_tick.max(second.opened_tick))
                .then_some((path, output.id))
            })
            .collect::<Vec<_>>();

        let decision = match matches.len() {
            0 => NaturalCycleClosureDecision::NoMatchingPath,
            1 => NaturalCycleClosureDecision::Closed,
            _ => NaturalCycleClosureDecision::Ambiguous,
        };
        self.trace_natural_cycle_closure(fired.junction, matches.len(), decision, moment, run);
        let [(path, output)] = matches.as_slice() else {
            return false;
        };
        let path = *path;
        let output = *output;
        let owners = self
            .learner_owner_for_origin(
                self.arena
                    .junction_by_id(output)
                    .expect("matched output remains live")
                    .physical_id,
            )
            .into_iter()
            .collect::<Vec<_>>();
        self.apply_outcome(path.junction, &owners, Some(lineage), moment, run);
        self.observe_causal_closure(
            fired.junction,
            output,
            &[path.first, path.second],
            moment,
            run,
        );
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::NaturalCycleClosed {
                    surface: fired.junction,
                    output,
                    first: path.first,
                    second: path.second,
                },
            });
        }
        true
    }

    fn trace_natural_cycle_closure(
        &self,
        surface: JunctionId,
        matching_paths: usize,
        decision: NaturalCycleClosureDecision,
        moment: &Moment,
        run: &mut RunState,
    ) {
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::NaturalCycleClosureEvaluated {
                    surface,
                    matching_paths: u32::try_from(matching_paths).unwrap_or(u32::MAX),
                    decision,
                },
            });
        }
    }

    /// Remove the temporary path after its outcome has physically returned.
    pub(crate) fn return_outcome(&mut self, id: LinkId) {
        let Some(slot) = self.arena.link_slot(id) else {
            return;
        };
        let link = self.arena.link_snapshot(slot.0);
        if !self
            .arena
            .return_links(&self.outcome_sources())
            .contains(&id)
        {
            return;
        }
        if link.delay == 0 {
            self.arena.zero_delay_live_links = self.arena.zero_delay_live_links.saturating_sub(1);
        }
        let index = id.0 as usize;
        self.arena.life[index] = 0;
        self.arena.decay_remainder[index] = 0;
        self.arena.edit_link(slot.0, LinkState::retire);
        self.arena.aging_links.remove(&id);
    }

    pub(crate) fn outcomes_return(&mut self, moment: &Moment, run: &mut RunState) {
        for incidence in &moment.incidences {
            if incidence.outcomes.is_empty() {
                continue;
            }
            let arrivals = u32::try_from(incidence.outcomes.len()).unwrap_or(u32::MAX);
            let impulse = incidence
                .outcomes
                .iter()
                .fold(0_i32, |sum, firing| sum.saturating_add(firing.impulse));
            let count = u64::try_from(incidence.outcomes.len()).unwrap_or(u64::MAX);
            run.work.total = run.work.total.saturating_add(count.saturating_mul(2));
            run.work.modulatory_deliveries = run.work.modulatory_deliveries.saturating_add(count);
            if self.trace_physics {
                for firing in &incidence.outcomes {
                    if let Some(lineage) = &firing.causal_lineage {
                        for origin_physical in lineage.origins() {
                            run.trace.push(PhysicalTransition {
                                tick: self.tick,
                                phase: moment.phase,
                                event: PhysicalEvent::CausalLineageMemberObserved {
                                    target: incidence.junction,
                                    origin_physical: *origin_physical,
                                    mode: TransmissionMode::Modulatory,
                                    link: firing.link.map(|(link, _)| link),
                                    generation: firing.link.map(|(_, generation)| generation.0),
                                    causal_wave: moment.causal,
                                },
                            });
                        }
                    }
                    run.trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase: moment.phase,
                        event: PhysicalEvent::ModulatoryOriginObserved {
                            target: incidence.junction,
                            origin_physical: firing.origin_physical,
                            link: firing.link.map(|(link, _)| link),
                            generation: firing.link.map(|(_, generation)| generation.0),
                            causal_wave: moment.causal,
                        },
                    });
                }
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::ModulatoryIncidence {
                        target: incidence.junction,
                        arrivals,
                        impulse,
                        causal_wave: moment.causal,
                    },
                });
            }
        }
    }

    pub(crate) fn strengthen_outcomes(&mut self, moment: &Moment, run: &mut RunState) {
        for incidence in &moment.incidences {
            for outcome in &incidence.outcomes {
                self.apply_outcome(incidence.junction, &[], None, moment, run);
                if let Some((link, _)) = outcome.link {
                    self.return_outcome(link);
                }
            }
        }
    }

    pub(crate) fn strengthen_candidate_outcomes(&mut self, moment: &Moment, run: &mut RunState) {
        let mut admitted_origins = BTreeSet::new();
        let mut answered_cohorts = BTreeSet::new();
        let mut outcomes = moment
            .incidences
            .iter()
            .flat_map(|incidence| {
                incidence
                    .outcomes
                    .iter()
                    .map(move |outcome| (incidence.junction, outcome))
            })
            .collect::<Vec<_>>();
        if self.protocol.prioritizes_eligible_returns() {
            outcomes.sort_by_key(|(_, outcome)| {
                (
                    !self.outcome_has_consequence_born_origin(outcome),
                    outcome.serial,
                )
            });
        }
        for (junction, outcome) in outcomes {
            let Some((link, generation)) = outcome.link else {
                continue;
            };
            let origins = if self.protocol.preserves_causal_lineage() {
                outcome
                    .causal_lineage
                    .as_ref()
                    .map(|lineage| {
                        lineage
                            .origins()
                            .iter()
                            .map(|origin| {
                                (
                                    *origin,
                                    lineage.birth_tick(*origin).unwrap_or(outcome.arrival_tick),
                                    lineage.transition_tick(*origin),
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_else(|| vec![(outcome.origin_physical, outcome.arrival_tick, None)])
            } else {
                vec![(outcome.origin_physical, outcome.arrival_tick, None)]
            };
            let mut accepted_origins = Vec::new();
            for (origin_physical, birth_tick, transition_tick) in origins {
                if self.protocol.constructs_learners()
                    && admitted_origins.contains(&origin_physical)
                {
                    self.trace_already_admitted_return_origin(
                        link,
                        generation,
                        origin_physical,
                        moment,
                        run,
                    );
                    continue;
                }
                if self.protocol.requires_consequence_born_return()
                    && self.reject_origin_born_before_return(
                        link,
                        generation,
                        origin_physical,
                        birth_tick,
                        moment,
                        run,
                    )
                {
                    continue;
                }
                if self.protocol.requires_physical_transition_return()
                    && self.reject_origin_without_later_transition(
                        link,
                        generation,
                        origin_physical,
                        transition_tick,
                        moment,
                        run,
                    )
                {
                    continue;
                }
                if let Some(accepted) =
                    self.accept_return_origin(link, generation, origin_physical, moment, run)
                {
                    admitted_origins.insert(origin_physical);
                    accepted_origins.push((
                        origin_physical,
                        accepted.owner,
                        birth_tick,
                        transition_tick,
                    ));
                }
            }
            if accepted_origins.is_empty() {
                continue;
            }
            if self.protocol.closes_return_cohort() {
                if let Some(state) = self.arena.link_by_id(link) {
                    answered_cohorts.insert((state.from, state.opened_tick));
                }
            }
            let owners = accepted_origins
                .iter()
                .filter_map(|(_, owner, _, _)| *owner)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let admitted_origins = accepted_origins
                .iter()
                .map(|(origin, _, _, _)| *origin)
                .collect::<BTreeSet<_>>();
            let admitted_lineage = self.protocol.preserves_causal_lineage().then(|| {
                outcome
                    .causal_lineage
                    .as_ref()
                    .and_then(|lineage| lineage.selected(&admitted_origins))
                    .unwrap_or_else(|| {
                        CausalLineage::singleton(outcome.origin_physical, outcome.arrival_tick)
                    })
            });
            self.apply_outcome(junction, &owners, admitted_lineage.as_ref(), moment, run);
            if self.protocol.consolidates_reverse_paths() {
                for (origin_physical, owner, birth_tick, transition_tick) in accepted_origins {
                    self.consolidate_reverse_path(
                        link,
                        origin_physical,
                        ReturnOriginTiming {
                            birth_tick,
                            transition_tick,
                        },
                        owner,
                        moment,
                        run,
                    );
                }
            }
        }
        for (source, opened_tick) in answered_cohorts {
            let cohort = self
                .arena
                .return_links(&[source])
                .into_iter()
                .filter(|link| {
                    self.arena
                        .link_by_id(*link)
                        .is_some_and(|state| state.opened_tick == opened_tick)
                })
                .collect::<Vec<_>>();
            for link in &cohort {
                self.return_outcome(*link);
            }
            if self.trace_physics {
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::ReturnCohortClosed {
                        source,
                        opened_tick,
                        link_count: u32::try_from(cohort.len()).unwrap_or(u32::MAX),
                    },
                });
            }
        }
    }

    fn reject_origin_born_before_return(
        &self,
        link: LinkId,
        expected_generation: Generation,
        origin_physical: u64,
        birth_tick: i64,
        moment: &Moment,
        run: &mut RunState,
    ) -> bool {
        let Some(state) = self.arena.link_by_id(link).filter(|state| {
            state.live
                && state.generation == expected_generation
                && state.mode == TransmissionMode::Modulatory
        }) else {
            return false;
        };
        if birth_tick > state.opened_tick {
            return false;
        }
        let owner = self
            .learner_owner_for_origin(origin_physical)
            .or_else(|| self.return_memory_owner(link));
        let origin = self
            .arena
            .junctions
            .iter()
            .find(|junction| junction.live && junction.physical_id == origin_physical)
            .map(|junction| junction.id);
        let distance = self
            .arena
            .junction_by_id(state.to)
            .zip(origin.and_then(|origin| self.arena.junction_by_id(origin)))
            .map(|(target, origin)| origin.position.saturating_sub(target.position).abs());
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::ClosureEligibilityEvaluated {
                    return_link: link,
                    origin_physical,
                    origin_birth_tick: birth_tick,
                    return_opened_tick: state.opened_tick,
                    eligible: false,
                },
            });
        }
        self.trace_return_origin_evaluation(
            owner,
            link,
            state.generation.0,
            origin_physical,
            Some(state.from),
            Some(state.to),
            origin,
            distance,
            ReturnOriginDecision::RejectedBeforeReturnOpened,
            moment,
            run,
        );
        self.trace_return_origin_admission(owner, state, origin_physical, false, moment, run);
        true
    }

    fn reject_origin_without_later_transition(
        &self,
        link: LinkId,
        expected_generation: Generation,
        origin_physical: u64,
        transition_tick: Option<i64>,
        moment: &Moment,
        run: &mut RunState,
    ) -> bool {
        let Some(state) = self.arena.link_by_id(link).filter(|state| {
            state.live
                && state.generation == expected_generation
                && state.mode == TransmissionMode::Modulatory
        }) else {
            return false;
        };
        let eligible = transition_tick.is_some_and(|tick| tick > state.opened_tick);
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::PhysicalTransitionEligibilityEvaluated {
                    return_link: link,
                    origin_physical,
                    transition_tick,
                    return_opened_tick: state.opened_tick,
                    eligible,
                },
            });
        }
        if eligible {
            return false;
        }
        let owner = self
            .learner_owner_for_origin(origin_physical)
            .or_else(|| self.return_memory_owner(link));
        let origin = self
            .arena
            .junctions
            .iter()
            .find(|junction| junction.live && junction.physical_id == origin_physical)
            .map(|junction| junction.id);
        let distance = self
            .arena
            .junction_by_id(state.to)
            .zip(origin.and_then(|origin| self.arena.junction_by_id(origin)))
            .map(|(target, origin)| origin.position.saturating_sub(target.position).abs());
        self.trace_return_origin_evaluation(
            owner,
            link,
            state.generation.0,
            origin_physical,
            Some(state.from),
            Some(state.to),
            origin,
            distance,
            ReturnOriginDecision::RejectedUnchangedSample,
            moment,
            run,
        );
        self.trace_return_origin_admission(owner, state, origin_physical, false, moment, run);
        true
    }

    fn outcome_has_consequence_born_origin(&self, outcome: &Firing) -> bool {
        let Some((link, generation)) = outcome.link else {
            return false;
        };
        let Some(state) = self
            .arena
            .link_by_id(link)
            .filter(|state| state.live && state.generation == generation)
        else {
            return false;
        };
        outcome.causal_lineage.as_ref().is_some_and(|lineage| {
            lineage.origins().iter().any(|origin| {
                if self.protocol.requires_physical_transition_return() {
                    lineage
                        .transition_tick(*origin)
                        .is_some_and(|tick| tick > state.opened_tick)
                } else {
                    lineage
                        .birth_tick(*origin)
                        .is_some_and(|tick| tick > state.opened_tick)
                }
            })
        }) || (!self.protocol.requires_physical_transition_return()
            && outcome.causal_lineage.is_none()
            && outcome.arrival_tick > state.opened_tick)
    }

    fn trace_already_admitted_return_origin(
        &self,
        link: LinkId,
        generation: Generation,
        origin_physical: u64,
        moment: &Moment,
        run: &mut RunState,
    ) {
        let state = self.arena.link_by_id(link);
        let source = state.map(|state| state.from);
        let target = state.map(|state| state.to);
        let origin = self
            .arena
            .junctions
            .iter()
            .find(|junction| junction.live && junction.physical_id == origin_physical)
            .map(|junction| junction.id);
        let distance = target
            .and_then(|target| self.arena.junction_by_id(target))
            .zip(origin.and_then(|origin| self.arena.junction_by_id(origin)))
            .map(|(target, origin)| origin.position.saturating_sub(target.position).abs());
        self.trace_return_origin_evaluation(
            self.learner_owner_for_origin(origin_physical)
                .or_else(|| self.return_memory_owner(link)),
            link,
            generation.0,
            origin_physical,
            source,
            target,
            origin,
            distance,
            ReturnOriginDecision::RejectedAlreadyAdmittedThisMoment,
            moment,
            run,
        );
    }

    fn consolidate_reverse_path(
        &mut self,
        return_link: LinkId,
        origin: u64,
        timing: ReturnOriginTiming,
        consequence_owner: Option<LearnerId>,
        moment: &Moment,
        run: &mut RunState,
    ) {
        let Some(return_state) = self.arena.link_by_id(return_link).cloned() else {
            self.trace_reverse_path_evaluation(
                return_link,
                origin,
                None,
                None,
                None,
                ReversePathDecision::MissingReturnLink,
                moment,
                run,
            );
            return;
        };
        let eligibility_tick = if self.protocol.requires_physical_transition_return() {
            timing.transition_tick
        } else {
            Some(timing.birth_tick)
        };
        let closure_eligible = !self.protocol.requires_consequence_born_closure()
            || eligibility_tick.is_some_and(|tick| tick > return_state.opened_tick);
        if self.trace_physics && self.protocol.requires_consequence_born_closure() {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::ClosureEligibilityEvaluated {
                    return_link,
                    origin_physical: origin,
                    origin_birth_tick: timing.birth_tick,
                    return_opened_tick: return_state.opened_tick,
                    eligible: closure_eligible,
                },
            });
        }
        let Some(source) = self
            .arena
            .junctions
            .iter()
            .find(|junction| junction.live && junction.physical_id == origin)
            .map(|junction| junction.id)
        else {
            self.trace_reverse_path_evaluation(
                return_link,
                origin,
                None,
                None,
                None,
                ReversePathDecision::OriginNotFound,
                moment,
                run,
            );
            return;
        };
        if source == return_state.from {
            self.trace_reverse_path_evaluation(
                return_link,
                origin,
                Some(source),
                None,
                None,
                ReversePathDecision::OriginIsReturnSource,
                moment,
                run,
            );
            return;
        }
        let action = self
            .arena
            .paths_through(return_state.to)
            .into_iter()
            .filter_map(|path| {
                let second = self.arena.link_by_id(path.second)?;
                (second.participation_level > 0).then_some((
                    path,
                    second.participation_level,
                    self.arena.strength[path.second.0 as usize],
                    self.arena.life[path.second.0 as usize],
                ))
            })
            .max_by_key(|(_, participation, strength, _)| {
                (*participation, strength.unsigned_abs())
            });
        let Some((action, _, action_strength, action_life)) = action else {
            self.trace_reverse_path_evaluation(
                return_link,
                origin,
                Some(source),
                None,
                None,
                ReversePathDecision::NoParticipatingActionPath,
                moment,
                run,
            );
            return;
        };
        let Some(action_second) = self.arena.link_by_id(action.second) else {
            self.trace_reverse_path_evaluation(
                return_link,
                origin,
                Some(source),
                None,
                None,
                ReversePathDecision::MissingActionLink,
                moment,
                run,
            );
            return;
        };
        let output = action_second.to;
        let sign = action_strength.signum();
        if sign == 0 {
            self.trace_reverse_path_evaluation(
                return_link,
                origin,
                Some(source),
                Some(output),
                None,
                ReversePathDecision::ZeroActionStrength,
                moment,
                run,
            );
            return;
        }
        let Some(reverse) = self.arena.paths_from(source).into_iter().find(|path| {
            self.arena.link_by_id(path.second).is_some_and(|second| {
                second.to == output && self.arena.strength[path.second.0 as usize].signum() == sign
            })
        }) else {
            self.trace_reverse_path_evaluation(
                return_link,
                origin,
                Some(source),
                Some(output),
                None,
                ReversePathDecision::NoCompatibleReversePath,
                moment,
                run,
            );
            return;
        };
        let reverse_index = reverse.second.0 as usize;
        let consolidated = action_strength
            .unsigned_abs()
            .max(UNIT_U64.saturating_mul(2));
        self.arena.strength[reverse_index] = if sign < 0 {
            -i64::try_from(consolidated).unwrap_or(i64::MAX)
        } else {
            i64::try_from(consolidated).unwrap_or(i64::MAX)
        };
        self.arena.life[reverse_index] = self.arena.life[reverse_index].max(action_life);
        let observed = self.arena.strength[reverse_index] / UNIT;
        let resistance =
            self.arena.life[reverse_index].saturating_add(UNIT_U64.saturating_sub(1)) / UNIT_U64;
        let tick = self.tick;
        let slot = self
            .arena
            .link_slot(reverse.second)
            .expect("reverse path link resolves");
        let generation = self.arena.link_snapshot(slot.0).generation;
        let hold_consequence = self.protocol.holds_organism_outcome_for_first_choice()
            && timing.transition_tick.is_some();
        self.arena.edit_link(slot.0, |link| {
            link.coupling = i32::try_from(observed).unwrap_or_else(|_| {
                if observed.is_negative() {
                    i32::MIN
                } else {
                    i32::MAX
                }
            });
            link.resistance = u32::try_from(resistance).unwrap_or(u32::MAX);
            link.last_consequence_tick = Some(tick);
            if hold_consequence {
                link.held_consequence_tick = Some(tick);
            }
        });
        if let Some(owner) = consequence_owner {
            self.record_learner_consequence(
                owner,
                reverse.second,
                generation,
                self.tick,
                moment.phase,
                run,
            );
        }
        run.work.total = run.work.total.saturating_add(1);
        self.trace_reverse_path_evaluation(
            return_link,
            origin,
            Some(source),
            Some(output),
            Some(reverse.second),
            ReversePathDecision::Consolidated,
            moment,
            run,
        );
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::ReversePathConsolidated {
                    source,
                    output,
                    link: reverse.second,
                },
            });
        }
        if closure_eligible {
            self.observe_causal_closure(
                source,
                output,
                &[
                    return_link,
                    action.first,
                    action.second,
                    reverse.first,
                    reverse.second,
                ],
                moment,
                run,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn trace_reverse_path_evaluation(
        &self,
        return_link: LinkId,
        origin_physical: u64,
        source: Option<JunctionId>,
        output: Option<JunctionId>,
        reverse_link: Option<LinkId>,
        decision: ReversePathDecision,
        moment: &Moment,
        run: &mut RunState,
    ) {
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::ReversePathEvaluated {
                    return_link,
                    origin_physical,
                    source,
                    output,
                    reverse_link,
                    decision,
                },
            });
        }
    }

    fn accept_return_origin(
        &mut self,
        id: LinkId,
        expected_generation: Generation,
        origin: u64,
        moment: &Moment,
        run: &mut RunState,
    ) -> Option<AcceptedReturn> {
        let Some(slot) = self.arena.link_slot(id) else {
            self.trace_return_origin_evaluation(
                self.learner_owner_for_origin(origin),
                id,
                expected_generation.0,
                origin,
                None,
                None,
                None,
                None,
                ReturnOriginDecision::RejectedMissingLink,
                moment,
                run,
            );
            return None;
        };
        let state = self.arena.link_snapshot(slot.0);
        let owner = self
            .learner_owner_for_origin(origin)
            .or_else(|| self.return_memory_owner(id));
        if !state.live {
            self.trace_return_origin_evaluation(
                owner,
                id,
                state.generation.0,
                origin,
                Some(state.from),
                Some(state.to),
                None,
                None,
                ReturnOriginDecision::RejectedInactiveLink,
                moment,
                run,
            );
            return None;
        }
        if state.mode != TransmissionMode::Modulatory {
            self.trace_return_origin_evaluation(
                owner,
                id,
                state.generation.0,
                origin,
                Some(state.from),
                Some(state.to),
                None,
                None,
                ReturnOriginDecision::RejectedWrongMode,
                moment,
                run,
            );
            return None;
        }
        if !self.return_origin_is_available(owner, id, state.generation, origin) {
            self.trace_return_origin_evaluation(
                owner,
                id,
                state.generation.0,
                origin,
                Some(state.from),
                Some(state.to),
                None,
                None,
                ReturnOriginDecision::RejectedAlreadyRemembered,
                moment,
                run,
            );
            self.trace_return_origin_admission(owner, &state, origin, false, moment, run);
            return None;
        }
        let Some(source) = self.arena.junction_by_id(state.from) else {
            self.trace_return_origin_evaluation(
                owner,
                id,
                state.generation.0,
                origin,
                None,
                Some(state.to),
                None,
                None,
                ReturnOriginDecision::RejectedMissingSource,
                moment,
                run,
            );
            return None;
        };
        let Some(target) = self.arena.junction_by_id(state.to) else {
            self.trace_return_origin_evaluation(
                owner,
                id,
                state.generation.0,
                origin,
                Some(state.from),
                None,
                None,
                None,
                ReturnOriginDecision::RejectedMissingTarget,
                moment,
                run,
            );
            return None;
        };
        let direct = source.physical_id == origin;
        let origin_junction = self
            .arena
            .junctions
            .iter()
            .find(|junction| junction.live && junction.physical_id == origin)
            .map(|junction| (junction.id, junction.position));
        let distance =
            origin_junction.map(|(_, position)| position.saturating_sub(target.position).abs());
        let local = distance.is_some_and(|distance| distance <= LOCAL_VARIATION_RADIUS);
        if !direct && !local {
            self.trace_return_origin_evaluation(
                owner,
                id,
                state.generation.0,
                origin,
                Some(state.from),
                Some(state.to),
                origin_junction.map(|(junction, _)| junction),
                distance,
                if origin_junction.is_some() {
                    ReturnOriginDecision::RejectedNonLocal
                } else {
                    ReturnOriginDecision::RejectedOriginNotFound
                },
                moment,
                run,
            );
            self.trace_return_origin_admission(owner, &state, origin, false, moment, run);
            return None;
        }
        self.remember_return_origin(owner, id, state.generation, origin);
        self.trace_return_origin_evaluation(
            owner,
            id,
            state.generation.0,
            origin,
            Some(state.from),
            Some(state.to),
            origin_junction.map(|(junction, _)| junction),
            distance,
            if direct {
                ReturnOriginDecision::AdmittedDirect
            } else {
                ReturnOriginDecision::AdmittedLocal
            },
            moment,
            run,
        );
        self.trace_return_origin_admission(owner, &state, origin, true, moment, run);
        Some(AcceptedReturn { owner })
    }

    #[allow(clippy::too_many_arguments)]
    fn trace_return_origin_evaluation(
        &self,
        owner: Option<LearnerId>,
        link: LinkId,
        generation: u32,
        origin_physical: u64,
        source: Option<JunctionId>,
        target: Option<JunctionId>,
        origin: Option<JunctionId>,
        distance: Option<i32>,
        decision: ReturnOriginDecision,
        moment: &Moment,
        run: &mut RunState,
    ) {
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::ReturnOriginEvaluated {
                    owner,
                    link,
                    generation,
                    origin_physical,
                    source,
                    target,
                    origin,
                    distance,
                    decision,
                },
            });
        }
    }

    fn trace_return_origin_admission(
        &self,
        owner: Option<LearnerId>,
        state: &LinkState,
        origin_physical: u64,
        admitted: bool,
        moment: &Moment,
        run: &mut RunState,
    ) {
        if self.trace_physics {
            run.trace.push(PhysicalTransition {
                tick: self.tick,
                phase: moment.phase,
                event: PhysicalEvent::ReturnOriginAdmission {
                    owner,
                    link: state.id,
                    generation: state.generation.0,
                    origin_physical,
                    admitted,
                },
            });
        }
    }

    pub(crate) fn apply_outcome(
        &mut self,
        junction: JunctionId,
        consequence_owners: &[LearnerId],
        causal_lineage: Option<&CausalLineage>,
        moment: &Moment,
        run: &mut RunState,
    ) {
        let return_updates_before = run.work.local_return_updates;
        let candidates = if self.protocol.is_sensorimotor() {
            let mut local = self.arena.incoming_index[junction.0 as usize].clone();
            local.extend_from_slice(&self.arena.outgoing_index[junction.0 as usize]);
            local.sort_unstable();
            local.dedup();
            run.cost.allocations = run.cost.allocations.saturating_add(1);
            run.cost.adjacency_accesses = run
                .cost
                .adjacency_accesses
                .saturating_add(u64::try_from(local.len()).unwrap_or(u64::MAX));
            local
        } else {
            run.cost.allocations = run.cost.allocations.saturating_add(1);
            run.cost.scans = run
                .cost
                .scans
                .saturating_add(u64::try_from(self.arena.links.len()).unwrap_or(u64::MAX));
            run.cost.touch::<LinkState>(self.arena.links.len());
            self.arena
                .links
                .iter()
                .map(|link| link.id)
                .collect::<Vec<_>>()
        };
        let qualified_local = candidates.iter().any(|id| {
            let slot = self
                .arena
                .link_slot(*id)
                .expect("indexed LINK must resolve");
            let link = self.arena.link_snapshot(slot.0);
            link.live
                && link.from == junction
                && link.mode == TransmissionMode::Drive
                && link.participation_level > 0
        });
        let mut learner_updates = Vec::new();
        for id in candidates {
            run.cost.scans = run.cost.scans.saturating_add(1);
            let slot = self.arena.link_slot(id).expect("indexed LINK must resolve");
            let link = self.arena.link_snapshot(slot.0);
            let local_participating_structure = link.live
                && link.mode == TransmissionMode::Drive
                && (link.from == junction || link.to == junction);
            if local_participating_structure && link.participation_level > 0 {
                let index = id.0 as usize;
                let participation = link.participation_level;
                let coupling_before = self.arena.strength[index];
                let resistance_before = self.arena.life[index];
                let sign = coupling_before.signum();
                self.arena.strength[index] = coupling_before.saturating_add(
                    sign.saturating_mul(i64::try_from(participation).unwrap_or(i64::MAX)),
                );
                self.arena.life[index] = resistance_before
                    .saturating_add(participation.saturating_mul(u64::from(LOCAL_RETURN_STRENGTH)));
                let coupling_observer = self.arena.strength[index] / UNIT;
                let resistance_observer =
                    self.arena.life[index].saturating_add(UNIT_U64.saturating_sub(1)) / UNIT_U64;
                let hold_consequence = self.protocol.holds_organism_outcome_for_first_choice()
                    && causal_lineage.is_some_and(CausalLineage::contains_transition);
                self.arena.edit_link(slot.0, |live_link| {
                    live_link.coupling = i32::try_from(coupling_observer).unwrap_or_else(|_| {
                        if coupling_observer.is_negative() {
                            i32::MIN
                        } else {
                            i32::MAX
                        }
                    });
                    live_link.resistance = u32::try_from(resistance_observer).unwrap_or(u32::MAX);
                    live_link.decay_load = 0;
                    if self.protocol.is_sensorimotor() {
                        live_link.last_consequence_tick = Some(self.tick);
                        if hold_consequence {
                            live_link.held_consequence_tick = Some(self.tick);
                        }
                    }
                });
                if self.trace_physics && self.protocol.is_sensorimotor() {
                    run.trace.push(PhysicalTransition {
                        tick: self.tick,
                        phase: moment.phase,
                        event: PhysicalEvent::ConsequenceRecorded { link: id, junction },
                    });
                }
                if !consequence_owners.is_empty() {
                    learner_updates.push((id, link.generation));
                }
                run.work.total = run.work.total.saturating_add(4);
                run.work.local_return_updates = run.work.local_return_updates.saturating_add(1);
            }
            run.cost.touch::<LinkState>(1);
        }
        for owner in consequence_owners {
            for (link, generation) in &learner_updates {
                self.record_learner_consequence(
                    *owner,
                    *link,
                    *generation,
                    self.tick,
                    moment.phase,
                    run,
                );
            }
        }
        if run.work.local_return_updates > return_updates_before {
            self.strengthen(junction, &mut run.work, moment.phase, &mut run.trace);
        }
        if qualified_local {
            self.propagate_qualified_local(junction, causal_lineage, moment, run);
        }
    }

    fn propagate_qualified_local(
        &mut self,
        junction: JunctionId,
        causal_lineage: Option<&CausalLineage>,
        moment: &Moment,
        run: &mut RunState,
    ) {
        let outgoing = self.arena.outgoing_index[junction.0 as usize].clone();
        for id in outgoing {
            let slot = self.arena.link_slot(id).expect("indexed LINK must resolve");
            let link = self.arena.link_snapshot(slot.0);
            run.cost.scans = run.cost.scans.saturating_add(1);
            run.cost.touch::<LinkState>(1);
            if !link.live
                || link.from != junction
                || link.trigger != TransmissionTrigger::QualifiedLocalParticipation
            {
                continue;
            }
            assert_eq!(link.mode, TransmissionMode::Modulatory);
            let Some(source_slot) = self.arena.junction_slot(link.from) else {
                continue;
            };
            let Some(target_slot) = self.arena.junction_slot(link.to) else {
                continue;
            };
            let source = self.arena.junction_snapshot(source_slot.0);
            let target = self.arena.junction_snapshot(target_slot.0);
            if source.id != link.from
                || !source.live
                || source.generation != link.source_generation
                || target.id != link.to
                || !target.live
                || target.generation != link.target_generation
            {
                continue;
            }
            let arrival_tick = self.tick.saturating_add(link.delay);
            let arrival_phase = link.phase;
            let generation = link.generation;
            let coupling = link.coupling;
            let target_generation = target.generation;
            let target_id = link.to;
            let origin_physical = source.physical_id;
            self.arena.edit_link(slot.0, |live_link| {
                live_link.participation_level = live_link
                    .participation_level
                    .saturating_add(PARTICIPATION_IMPULSE);
            });
            run.work.total = run.work.total.saturating_add(1);
            run.work.qualified_local_traversals =
                run.work.qualified_local_traversals.saturating_add(1);
            if self.trace_physics {
                run.trace.push(PhysicalTransition {
                    tick: self.tick,
                    phase: moment.phase,
                    event: PhysicalEvent::QualifiedLocalTraversal { link: id },
                });
            }
            self.pending.push(
                Firing {
                    arrival_tick,
                    phase: arrival_phase,
                    causal_wave: if link.delay == 0 && arrival_phase == moment.phase {
                        moment.causal.saturating_add(1)
                    } else {
                        0
                    },
                    origin_physical,
                    causal_lineage: self
                        .protocol
                        .preserves_causal_lineage()
                        .then(|| causal_lineage.cloned())
                        .flatten(),
                    physical_incidence: PhysicalIncidence::Sample,
                    target_physical: target.physical_id,
                    target: target_id,
                    target_generation,
                    impulse: coupling,
                    strength: self.arena.strength[id.0 as usize],
                    serial: self.next_serial,
                    link: Some((id, generation)),
                },
                &mut run.cost,
            );
            self.next_serial = self.next_serial.wrapping_add(1);
        }
    }
}
