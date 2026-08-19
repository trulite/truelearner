# Discovery d2.3: Expectation-triggered reopening

## Question

Can a consolidated action policy reopen previously rejected alternatives when
its expected consequence is persistently absent?

D2.3 changes one generic mechanism. Historical action evidence is retained,
while a separate current-regime policy controls present action choice. Three
consecutive violations of the trusted action reopen only that local action
mapping.

## Setup

The hard d2.2 remap is reused:

- the old informative action becomes inert,
- an action previously tried and rejected becomes informative,
- no change notification is supplied.

Policies mature for 10, 50, or 100 problems before the remap. Tests use 16
and 64 choices. A full-reset policy receives the same new mapping as a
forgetting baseline.

Controls include:

- an unchanged environment,
- isolated noisy failures every tenth problem,
- a repeated first-regime to second-regime to first-regime to second-regime
  diagnostic,
- complete destruction of every topology workspace.

## Results

```text
choices  maturity  violations to reopen  problems to adapt  paid actions  full-reset problems
16       10        3                     8                  21            6
16       50        3                     8                  21            6
16       100       3                     8                  21            6
64       10        3                     8                  69            6
64       50        3                     8                  69            6
64       100       3                     8                  69            6
```

All 12 remapped mature policies adapt. The reopening and adaptation times are
independent of prior maturity.

The reset baseline is faster because it throws away all previous action
knowledge. D2.3 pays to preserve that history.

Repeated-switch diagnostic:

```text
switch                         violations  problems  paid actions
first regime -> second regime  3           8         21
second regime -> first regime  3           8          9
first regime -> second again   3           8          9
```

Retained history therefore reduces rediscovery cost after both regimes have
been experienced.

Isolated noise causes zero false reopenings. An unchanged environment also
causes zero. All 2,162 topology workspaces are destroyed.

The full per-problem trace is stored in `results/d2_3_plasticity.csv`.

## Conclusion

The precise result is:

> Repeated violations of a consolidated expectation locally reopen previously
> rejected alternatives. Recent evidence selects the current action policy,
> while evidence from earlier regimes remains available for later reuse.

The three-violation threshold and the usefulness of distinguishing evidence
are supplied. The learner does not yet discover its own change-detection
timescale or regime representation.
