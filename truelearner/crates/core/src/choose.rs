use crate::prelude::*;

impl Body {
    pub(crate) fn choose_at(&mut self, moment: &mut Moment, run: &mut RunState) {
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
