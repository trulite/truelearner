# RB0 recurrence boundary result audit v1

## Disposition

RB0 is a development positive. A finite, ordinary inhibitory-efficacy region
preserves the intended first traversal and makes CORE-B naturally quiescent.
CORE0 E11 therefore exposed a supplied material value on the unstable side of
an existing stability boundary; it did not justify a new substrate law.

The sole evaluator execution emitted
`RB0_RECURRENCE_BOUNDARY_V1_EVIDENCE_SPENT` once and completed 158 canonical
cases / 254 profile rows. No rerun or rescue occurred.

## Exact boundary

For the CORE0 E11 geometry `(E=2, T=2, delays=1+1)` under either historical
phase pattern:

```text
H = 0, 1, 1.5, 2   -> periodic; A/B fire 48|48 in the observation
H = 2.5, 3, 4, 8, 16 -> naturally quiescent; A/B fire exactly 1|1
```

Thus the first frozen continuous point that settles is `H=2.5`. The failed
CORE0 E11 circuit used `H=2`. RS1's integer characterization had already found
`H1/H2` periodic and `H3+` quiescent; Q32 material resolves the same boundary
between two and three rather than changing it.

Across the complete CORE-B map:

- 96 / 158 cases were quiescent;
- 62 / 158 were periodic;
- zero were persistent-nonperiodic or growing;
- 58 cycle cases were both quiescent and preserved exactly one A/B firing;
- both one-way controls fired A/B exactly once and quiesced.

At threshold matched to excitation, the minimum frozen settling magnitude at
delay `1+1` was `H=2.5` for `(E,T)=(1,1)` and `(2,2)`, and `H=2.5` for
`(3,3)`. With threshold fixed at two it rose with excitation: `E=2.5 -> H=3`
and `E=3 -> H=4`.

Timing is ordinary physics rather than a representation artifact. At
`E=2,T=2`, the minimum sampled settling magnitude was:

```text
delay 0+1 -> H=2
delay 1+1 -> H=2.5
delay 2+2 -> H=16 among the frozen values
delay 3+3 -> H=16 among the frozen values
```

Both `core0_phase` and `rs1_phase` produced the same boundary throughout these
sections.

## Representation result

Exact replay and Reference/Production equality passed on all 254 rows. All
254 rows also passed the inert/static controls: no Modulation, plastic update,
proposal, QLP traversal, deallocation, or coupling change occurred; resistance
only decreased through the already-accepted local forgetting law.

The 96 parameter tuples representable by both whole-integer and Q32 bodies had:

- zero activity-class disagreements; and
- `first_divergence=none` for the complete compared physical histories.

CORE-B therefore reduces exactly to the RS1-style body at whole material
values in this discriminator. Its fractional representation exposes the
boundary more precisely; it does not introduce a different recurrence law.

Maximum PhysicalWork was 288, the frozen two-segment observation ceiling.

## Evidence limitation

The matrix serializes classifications, firing/incidence counts, work, clock,
pending count, quiescence, initial/final Q32 material state, and trace hashes.
The evaluator compared complete physical transitions internally, but it did
not serialize the transition bodies or the requested per-wave activation
series. Consequently the stability and equivalence claims are frozen, while
an independent artifact-only reconstruction of every intermediate impulse is
not available. RB0 remains development evidence and must not be relabeled as
authority.

## Frozen hashes

- core: `3594116d503630b87957a1b16f46cebca360e9c14441cc9f1589900f04788ca0`
- evaluator: `a364482a579d58d0daa6f2336a652830bdfb44f53525a33c4be7ac8c3fd23bdc`
- protocol v3: `f3d98489c723ab4a57a939eec87725c31f0e7bdc904b86a6fd0ec2c6289ba5fa`
- matrix: `4bb2fa78f85ba1d863f6279734dd2434171018ce20c7457bcea7222e6a731518`
- profile comparison: `e3fe815a04add8e4748bc9b8e08113dbef0c335cc68eb66dba327d2527fae56d`
- report: `f6a0739b23091e0052c0901978778b7759502c94d9851293a8bd0e7371a3fa63`

