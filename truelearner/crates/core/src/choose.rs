use crate::prelude::*;

impl Body {
    pub(crate) fn choose_at(&mut self, moment: &mut Moment, run: &mut RunState) {
        self.compete_local_outputs(moment, &mut run.work);
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
    fn compete_local_outputs(&self, moment: &mut Moment, work: &mut Work) {
        let path_seconds = self
            .arena
            .paths()
            .into_iter()
            .map(|path| path.second)
            .collect::<HashSet<_>>();
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
                let (drive, participation, admitted) =
                    admitted_path_drive(&self.arena, &incidence.inputs, &path_seconds, held);
                let other = incidence.inputs.iter().fold(0_i64, |sum, firing| {
                    if firing
                        .link
                        .is_some_and(|(link, _)| path_seconds.contains(&link))
                    {
                        sum
                    } else {
                        sum.saturating_add(firing.strength)
                    }
                });
                let projected = held.saturating_add(other).saturating_add(admitted);
                (projected >= i64::from(target.threshold).saturating_mul(UNIT)).then_some((
                    index,
                    target.position,
                    incidence.junction,
                    drive,
                    participation,
                ))
            })
            .collect::<Vec<_>>();
        candidates.sort_by_key(|(_, position, junction, _, _)| (*position, junction.0));

        let mut cursor = 0;
        while cursor < candidates.len() {
            let mut end = cursor + 1;
            while end < candidates.len()
                && candidates[end]
                    .1
                    .saturating_sub(candidates[end - 1].1)
                    .abs()
                    <= LOCAL_VARIATION_RADIUS
            {
                end += 1;
            }
            if end - cursor > 1 {
                let group = &candidates[cursor..end];
                let strongest = group.iter().map(|candidate| candidate.3).max().unwrap_or(0);
                let least_used = group
                    .iter()
                    .filter(|candidate| candidate.3 == strongest)
                    .map(|candidate| candidate.4)
                    .min()
                    .unwrap_or(0);
                let tied = group
                    .iter()
                    .filter(|candidate| candidate.3 == strongest && candidate.4 == least_used)
                    .collect::<Vec<_>>();
                let rotation = usize::try_from(self.tick.unsigned_abs()).unwrap_or(usize::MAX);
                let winner = tied[rotation % tied.len()].0;
                for candidate in group {
                    if candidate.0 != winner {
                        moment.incidences[candidate.0].inputs.clear();
                        work.total = work.total.saturating_add(1);
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
        let seconds = self
            .arena
            .paths()
            .into_iter()
            .map(|path| path.second)
            .collect::<HashSet<_>>();
        let choices = firings
            .iter()
            .filter_map(|firing| {
                let link = firing.link?.0;
                let sign = firing.strength.signum() as i8;
                (seconds.contains(&link) && sign != 0).then_some((
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
        let opportunity = self
            .arena
            .activation
            .get(target.0 as usize)
            .copied()
            .unwrap_or(0)
            > 0;
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

fn admitted_path_drive(
    arena: &Arena,
    firings: &[Firing],
    path_seconds: &HashSet<LinkId>,
    held: i64,
) -> (u64, u64, i64) {
    let positive = firings
        .iter()
        .filter(|firing| {
            firing
                .link
                .is_some_and(|(link, _)| path_seconds.contains(&link))
                && firing.strength > 0
        })
        .fold(0_i64, |sum, firing| sum.saturating_add(firing.strength));
    let negative = firings
        .iter()
        .filter(|firing| {
            firing
                .link
                .is_some_and(|(link, _)| path_seconds.contains(&link))
                && firing.strength < 0
        })
        .fold(0_i64, |sum, firing| sum.saturating_add(firing.strength));
    let positive_magnitude = positive.unsigned_abs();
    let negative_magnitude = negative.unsigned_abs();
    if positive_magnitude > negative_magnitude {
        (
            positive_magnitude,
            path_participation(arena, firings, path_seconds, 1),
            positive,
        )
    } else if negative_magnitude > positive_magnitude {
        (
            negative_magnitude,
            path_participation(arena, firings, path_seconds, -1),
            negative,
        )
    } else if positive_magnitude > 0 && negative_magnitude > 0 && held > 0 {
        (
            positive_magnitude,
            path_participation(arena, firings, path_seconds, 1),
            positive,
        )
    } else {
        (0, 0, 0)
    }
}

fn path_participation(
    arena: &Arena,
    firings: &[Firing],
    path_seconds: &HashSet<LinkId>,
    sign: i64,
) -> u64 {
    firings
        .iter()
        .filter(|firing| firing.strength.signum() == sign)
        .filter_map(|firing| firing.link.map(|(link, _)| link))
        .filter(|link| path_seconds.contains(link))
        .filter_map(|link| arena.link_by_id(link))
        .fold(0_u64, |sum, link| {
            sum.saturating_add(link.participation_level)
        })
}
