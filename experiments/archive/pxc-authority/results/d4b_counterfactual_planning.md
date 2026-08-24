# D4b: Counterfactual Search Over Learned Transformations

## Question

Can supplied bounded search use frozen learned transformations to choose an
unseen multi-step intervention before acting?

## Boundary

The action models are learned using the D4a provenance learner. Training shows
individual actions only.

Search is supplied. It enumerates opaque action sequences shortest-first,
simulates each sequence with frozen learned transformations, and applies a
supplied structural criterion. The true environment is not called until after
one sequence has been selected.

## Planning Problems

Each temporary problem contains two competing route structures. They initially
produce the same result. One learned action advances a temporary occupant
through role positions, another reveals the staged occupant, and a third is
inert.

Fresh arrangements require shortest distinguishing sequences of exactly one,
two, three, four, or eight actions. Reversing advance and reveal fails.

## Results

```text
Required     Learned   Real      Oracle    Random    One-step  Mask
depth        planner   execution planner   order     selector  planner
1            8 / 8     8 / 8     8 / 8     7 / 8     8 / 8    0 / 8
2            8 / 8     8 / 8     8 / 8     7 / 8     0 / 8    0 / 8
3            8 / 8     8 / 8     8 / 8     6 / 8     0 / 8    0 / 8
4            8 / 8     8 / 8     8 / 8     4 / 8     0 / 8    0 / 8
8            8 / 8     8 / 8     8 / 8     6 / 8     0 / 8    0 / 8
```

The random-order baseline receives the same number of candidate evaluations as
the learned-model planner.

Search growth:

```text
Required depth    Candidates examined   Model applications   Role transfers
1                          1.9                    1.9                  24.4
2                          8.2                   13.5                 175.5
3                         27.4                   67.1                 872.6
4                         84.8                  285.0               3,705.0
8                      6,969.8               50,850.0             661,050.0
```

Additional controls:

```text
Unreachable problems reported       8 / 8
Order-sensitive problems             8 / 8
Permanent learned model entries        507
Training action sequences                 0
Frozen fingerprints changed              no
Planning completed before acting         yes
```

## Interpretation

The demonstrated claim is:

> Supplied bounded search used frozen learned transformations to select and
> execute unseen multi-step interventions from predicted consequences.

Role positions, identity equality, the epistemic criterion, and exhaustive
enumeration remain supplied. D4b demonstrates planning with learned models,
not learned planning or efficient search.

All 135,456 candidate predictions are recorded in
[`d4b_planning_traces.csv`](d4b_planning_traces.csv).
