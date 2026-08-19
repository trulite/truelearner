# D4a: Composable Role-Relative Transformations

## Question

Can individually learned action transformations compose into exact predictions
for action sequences never seen during training?

## Input

Training provides one opaque action at a time, a temporary role structure
before acting, and the resulting role structure afterward. Every temporary
occupant is a fresh opaque identity.

The learner uses identity equality only to determine which input role supplied
each output occupant. It retains no occupant identities.

## Learned Structure

For every opaque action and output role, generic proposals connect all possible
input roles. Repeated evidence retains the unique source role.

The environment includes:

- identity,
- a two-role swap,
- two opposite three-role rotations,
- copy with overwrite,
- another two-role swap.

Vacancy, deletion, and fresh-value creation are intentionally excluded.

## Results

```text
Exact held-out sequence predictions    848 / 848
Changed-role mask baseline              32 / 848
Matched-mask provenance pairs           16 / 16
Mask-distinguishable matched pairs       0 / 16
Swap twice returns original             16 / 16
Order-sensitive action pairs            16 / 16
Shuffled confident action models         0
Permanent identity cells                 0
Frozen fingerprints changed              no
```

Accuracy by unseen sequence length:

```text
Length 1      96 / 96
Length 2     576 / 576
Length 3      80 / 80
Length 4      48 / 48
Length 8      32 / 32
Length 16     16 / 16
```

The 848 predictions use 2,192 applications of the same generic model
application mechanism. Permanent model size stays fixed while work grows with
sequence length. Temporary simulation state remains six role occupants.

## Matched-Mask Control

Two actions change exactly the same three output roles, but rotate their
occupants in opposite directions. A changed/preserved mask gives both actions
the same representation and cannot distinguish their consequences.

The provenance model learns different source-role arrows and predicts both
actions and their later compositions exactly.

## Interpretation

The demonstrated claim is:

> Individually learned role-relative transformations composed into exact
> predictions for unseen action sequences using fixed permanent models and
> generic repeated application.

Role positions, identity equality, the supplied action sequence, and the
generic model-application mechanism remain priors. D4a does not choose action
sequences or learn how to search.

Complete step-by-step traces are in
[`d4a_composition_traces.csv`](d4a_composition_traces.csv).
