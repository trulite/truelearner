# CORE1 E12 v2 fixture correction and continuation protocol v1

## Lineage and boundary

This evaluator-only continuation is rooted at CORE1-E12D result `47c7a4d`.
The physical runtime, profiles, learning law, and Academy integration remain
byte-identical to frozen CORE1 candidate
`2e19792067053ce4ff99ba424c7d94982e9c6260`.

Only CORE1-A/B/C are in scope. CORE1-D/E remain unchanged and unexecuted.

## E12 v2 correction

Training is byte-for-byte the frozen E12 training fixture:

- initial inhibitory coupling `-2`;
- both inhibitory links actually traverse;
- consequence arrives locally after exactly 12 ticks;
- exactly six teaching experiences;
- no adaptive stopping;
- unchanged continuous coupling/resistance law;
- unchanged no-consequence, wrong-contact, identity-permutation, unrelated
  negative, positive-link, and post-training-drift controls.

The sole fixture correction is the probe. Instead of cloning training cells
whose threshold is 100, construct the frozen RB0 executable geometry:

```text
A(threshold 2) -- +2,delay1 --> B(threshold 2)
B              -- +2,delay1 --> A
A -- +1,delay0 --> Ia(threshold 1) -- learned -H,delay0 --> A
B -- +1,delay0 --> Ib(threshold 1) -- learned -H,delay0 --> B
```

The learned Q32 coupling from each training inhibitory link is copied to its
corresponding ordinary probe link. One external Drive impulse `2` starts A.
No training state, probe result, or RB0 boundary value feeds back into learning.

Probe after fixed counts `0,1,2,3,4,5,6`. Record coupling, A/B and inhibitor
firings, negative incidence, work, ceiling, quiescence, and physical trace.

The unchanged stability predicate is:

```text
natural quiescence
AND A fires exactly once
AND B fires exactly once
```

Require probe 0 to remain active, and probes 2–6 to satisfy the stability
predicate. Probe 1 is recorded but not assigned a required class. Coupling must
mature strictly under consequence; frozen controls must remain exact.

Run Reference, exact Reference replay, and Production. Require exact complete
observations across replay and mechanics.

## Conditional continuation

Only if E12 v2 passes for all A/B/C, run the unchanged CORE1 gates:

1. E13 — four contextual action relations, expected `[1,4,2,3]`;
2. E14 — exact frozen ARC A2 teaching/closure regimen and predicates.

Each profile remains an independent row, but E13/E14 are globally blocked if
any E12 row fails. No repair or parameter change is permitted after execution.

## Execution discipline

Formatting, strict Clippy, release compilation, retained runtime checks, and a
byte comparison against frozen CORE1 physics precede candidate freeze. The
complete evaluator executes once and streams every row before advancing.
CORE1 authority and `arch.md` remain unchanged regardless of outcome.

