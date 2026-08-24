# d1 Intervention-Robust Discovery

## Question

Can the unchanged d0 learner reject a predictive shortcut when its experience
contains deliberately contrasting cases?

The narrow target is not general causal understanding. It is:

> Retain topology that remains predictive when observable nuisance
> correlations are counterbalanced.

## Unchanged Learner

D1 uses the same d0 mechanisms:

- temporal coactivity proposes arrows,
- one arrow competes per episode,
- the used arrow receives a recent-use trace,
- scalar success strengthens it,
- scalar failure weakens it,
- a repeatedly successful arrow consolidates,
- other provisional arrows are pruned.

No learning rule, strength update, proposal window, competition rule, or
consolidation threshold changes.

## Matched Curricula

Each observation-only learner and contrasting learner receives:

- the same opaque identities,
- the same relations and query,
- the same relation ordering,
- the same changed or unchanged right-side identity,
- the same learner seed,
- the same 792-episode budget.

Only cue placement differs.

The observation-only stream always places the cue on the queried relation.
Both the true slot route and the cue shortcut therefore predict the answer.

The contrasting stream includes matched cases where:

- the cue moves while the answer remains unchanged,
- the true right-side identity changes while the cue stays fixed,
- the cue occupies every relation position equally,
- the cue is absent equally often,
- changed and unchanged answers occur equally often.

No intervention marker or explanation enters the learner.

## Counterbalance

For the representative curriculum:

```text
cue on relative relation positions 0 through 9: 72 episodes each
cue absent:                                  72 episodes
unchanged answers:                          396 episodes
changed answers:                            396 episodes
```

Relation order and target position are identical between the paired
observation and contrasting streams.

## Results

Across 32 identical learner seeds:

| Curriculum | Shortcut failures after cue removal | Correct final route |
| --- | ---: | ---: |
| Observation only | 10/32 | 22/32 |
| Contrasting | 0/32 | 32/32 |

The independent d0 observation curriculum remains recorded at 15 failures in
32 runs. D1's paired comparison uses different, exactly matched curricula, so
its observation result is 10 failures rather than 15.

The representative contrasting learner showed:

```text
true Slot1 -> Slot2 route:
    strength reaches 4 and consolidates

cue -> Slot2 shortcut:
    early peak 1
    later value -1

competence:
    episode 83
```

Twenty thousand further held-out episodes without cues left the permanent
fingerprint unchanged.

The random-label control produced 62 successful guesses in 10,000 training
episodes and retained no stable arrow.

## Conclusion

D1 supports the narrow claim:

> Counterbalanced contrasting experience supplied enough information for the
> unchanged topology learner to reject a predictive shortcut and retain the
> intervention-stable slot route.

The curriculum chooses the contrasts. The learner does not understand an
intervention, choose an action, or seek information. Autonomous experiment
selection remains the next question.
