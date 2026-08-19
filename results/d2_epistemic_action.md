# d2 Learned Epistemic Action

## Question

Can unresolved topology learn which opaque action is worth paying for because
its consequences separate competing routes?

D2 does not predict the outcome of an untried action. It learns from actions
whose consequences it has already experienced.

## What Is Supplied

- the unchanged d0 proposal, route-trace, scalar-feedback, and strength-update
  rules,
- an internal unresolved state when multiple routes remain plausible,
- three opaque action identities plus no action,
- a small fixed cost for each real action,
- an action trace that snapshots plausible routes and their strengths,
- delayed cleanup while that trace remains active,
- three internally generated classifications: informative, disruptive, or
  uninformative,
- action meanings supplied by the environment and permuted between runs.

The action trace does not receive a correct route or evaluator information
score.

## Evidence Window

When an action is chosen, the trace records:

- the opaque action identity,
- every currently plausible route,
- each route's strength before acting,
- which routes have since been exercised.

Ordinary d0 route selection and strength updates continue. Snapshotted routes
cannot be pruned until every one has received evidence.

The trace then classifies its own result:

- one route weakens while another remains supported: informative,
- every route weakens: disruptive,
- no useful separation: uninformative.

An unresolved tie is not allowed to consolidate through fixed arrow ordering.
Cleanup resumes only when one route is uniquely strongest.

## Environment

The three opaque action meanings are randomly permuted for every run:

- informative: move the cue while preserving the real slot relationship,
- disruptive: corrupt the outcome so both plausible routes fail,
- inert: leave the ambiguity unchanged.

No action leaves the ambiguity unchanged but has zero cost.

## Results

Across 32 independent runs and all six action permutations:

```text
correct final topology:             32/32
informative action preferred:       32/32
disruptive action preferred:         0/32
paid actions after resolution:       0
```

The learned machines used:

```text
average action decisions:  3.6
average paid actions:      3.0
```

The representative run:

```text
decision 1:
    disruptive action
    real route       1 -> 0
    shortcut route   1 -> 0
    action value         -3

decision 2:
    informative action
    real route       1 -> 2
    shortcut route   1 -> 0
    action value         +2

decision 3:
    same informative action reused
    real route       3 -> 4
    shortcut route   1 -> 0
    action value         +4

topology resolves
further paid actions: 0
```

Across all learned runs, action windows were classified as:

```text
informative:    57
disruptive:     21
uninformative:  38
```

Random labels produced no positive action preference and no stable topology.

## Baselines

```text
learned policy: 32/32 correct
random actions: 32/32 correct
fixed action:   11/32 correct
no action:       0/32 correct
```

Random exploration required an average of 2.6 paid actions, slightly fewer
than the learned policy's 3.0. The action space contains only three real
actions, so random search remains a strong and cheaper discovery strategy in
this experiment.

This negative comparison matters:

> D2 demonstrates consequence-dependent action preference and conditional
> stopping, not superior experimental efficiency.

## Conclusion

D2 supports the narrow claim:

> While topology remained unresolved, the learner acquired a
> context-dependent preference for opaque actions whose consequences
> selectively weakened one plausible route while preserving another, and
> stopped paying for those actions after resolution.

It does not predict information value before experience, understand causal
interventions, or outperform random search in this tiny action space.
