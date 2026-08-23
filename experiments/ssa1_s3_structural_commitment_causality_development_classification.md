# SSA1-S3 structural-commitment causality — development handoff

Status: **Classification D — P8 readout, not established causal boundary**.

Frozen Organism v1 is unchanged. This successor does not amend SSA1-S2's
Classification B: P8 remains a perfect classifier of the final basin. SSA1-S3
shows that the two P8 structural observations are not individually sufficient
one-event commitment switches.

## Result

The hardened GATE used six fresh identity/layout cells crossed with three
frozen alternative-producing schedules, for 18 causal cells:

| descriptor (`B:A / stride / offset`) | B threshold | A deallocation |
|---|---:|---:|
| `1:2 / 7 / 1` | 1,529 | 12,509 |
| `1:2 / 13 / 43` | 429 | 3,509 |
| `1:2 / 17 / 1` | 432 | 3,533 |

All reference runs reproduced `[1,4] ALTERNATIVE`. Complete prefix clones,
incumbent/route mirrors, physical duplicate execution, schedules, and frozen
parents were exact.

The conjunctive result was:

```text
single threshold-event block       0/18 causal
single deallocation-event support  0/18 causal
post-transition basin inertia     18/18
all controls                      18/18
```

### Threshold node

At the reference threshold event, the alternative changed from one to four
live supporters. Withholding every alternative encounter for exactly that
event kept it at one, so the intervention reached and prevented the observed
transition. After the unchanged schedule resumed, the alternative formed the
missing structure later and every arm still ended `[1,4] ALTERNATIVE`.

Therefore the first threshold crossing is not a unique developmental closure
event. Preventing it once delays the physical transition without changing the
mature basin.

### Deallocation node

Immediately before reference incumbent deallocation, both routes were live at
`[4,4]`. Four ordinary incumbent encounter recurrences were physically
delivered before the pressure event through frozen `local_encounter`. The
frozen allocation/lifetime path nevertheless changed the incumbent from four
to one at that event, and every arm ended `[1,4] ALTERNATIVE`.

Therefore proposal recurrence at the last observed event is not independently
sufficient to preserve the incumbent basin. The observed deallocation is the
visible end of a coupled allocation/value/lifetime history, not an isolated
eraser that can be causally canceled by one ordinary recurrence.

### After the observed transitions

The same alternative absence immediately after threshold crossing left the
threshold transition intact. Incumbent recurrence immediately after
deallocation changed subthreshold structure from `[1,4]` to `[2,4]`, but did
not restore executable threshold or change the final alternative basin.

Thus post-transition experience is not literally state-inert. It can alter
subthreshold physical structure. It is basin-inert under the preregistered
single-event intervention.

## Interpretation

The supported statement is narrower than the proposed causal law:

> Developmental history becomes embodied in distributed, coupled M5/M6 and
> lifetime structure. P8 reads the resulting executable basin exactly, but no
> tested single threshold-formation or deallocation event is itself the causal
> commitment boundary.

P8 may first become available late because it detects when the accumulated
physical history has become externally legible as an executable basin. SSA1-S3
does not support describing that moment as a one-way closure analogous to the
fast SSA0.3 firing/inhibition boundary.

This leaves the program state unchanged:

```text
Frozen Organism v1  unchanged
SSA1                Classification C
SSA1-S2             Classification B (P8 exact late classifier)
SSA1-S3             Classification D (single-node causality rejected)
SSA2                blocked pending interpretation
```

No longer-duration block, stronger recurrence, direct record edit, learner
repair, or new mechanism was attempted. A future experiment about distributed
causal sufficiency would require a separately preregistered question; it must
not reinterpret this result.

## Operational record

The first PROBE artifact is preserved as an immutable negative. Its scientific
arms already showed the eventual D pattern, but its report was development-
invalid because the source audit inspected the composition wrapper instead of
the frozen lifetime source and an implementation-only control assumed the
pre-threshold count must be exactly three. The frozen reference actually moved
`1 -> 4`, which satisfied the preregistered `<4 -> >=4` definition.

Implementation v2 changed only those two audit conditions and result filenames.
It did not change the learner, physical interventions, schedules, identities,
consequences, or predictions. PROBE v2, MICRO, and GATE then classified D.

No definitive command, seed, cell, or artifact was executed. The definitive
surface refused with exit `2`.

## Commits and tags

| artifact | commit | tag |
|---|---|---|
| protocol v1 | `d28d064` | `ssa1-s3-structural-commitment-causality-protocol-v1` |
| implementation v1 | `6fd09ad` | `ssa1-s3-structural-commitment-causality-implementation-v1` |
| immutable PROBE v1 negative | `e023efa` | `ssa1-s3-structural-commitment-causality-probe-v1-negative` |
| mechanical implementation v2 | `cad0ac4` | `ssa1-s3-structural-commitment-causality-implementation-v2` |
| PROBE v2 D | `72157d0` | `ssa1-s3-structural-commitment-causality-probe-v2-classification-d` |
| MICRO v2 D | `0a195b2` | `ssa1-s3-structural-commitment-causality-micro-v2-classification-d` |
| GATE v2 D | `342293d` | `ssa1-s3-structural-commitment-causality-gate-v2-classification-d` |

## Frozen hashes

| artifact | SHA-256 |
|---|---|
| protocol | `55106241e70a6a0949980852209cb43767fdff2df096081be3f4d55bf2ebb06c` |
| physical adapter | `d6895c17bc4153ac899dfba227b87f2a1e4ebaa8266efe0e449d5d99a1d2872b` |
| S3 experiment v2 | `820c4b94a60e0fb733c4dd84d335608e2fcf7cb996eb26aa7fa80bad59d9be9e` |
| PROBE v1 CSV | `c08cd8335955973e2ea321f68c61f714a813f485529f29c0c4226ec8c68bf07f` |
| PROBE v2 CSV | `379ba0b3f1a5570d8e33e104bf6649bf99b977cf668c03be8834b5066e897de4` |
| MICRO v2 CSV | `0903cf9f70b36b65cb74bfd10cdef92a2ae51282b470da54909a39f846192a1a` |
| GATE v2 CSV | `3a4d2c8cc3818590c201e0483ae075e461b3b27cdd037f2380121c7fe2e5f7af` |
| GATE v2 report | `aa6a565ecfcbfaa9ed0bd25851bb83dda2e7d74c85da52d5239d95aa2189d941` |
| frozen S2 evaluator | `5e9f2055a4ec036f8adbe7c89de7028d2772826b2f5afea4bc97f99ca19d5c57` |
| frozen S2 GATE CSV | `164ea561cff5ba910dfb5cc9c2b781de05feb83761afc7582b9ce16cb74ad6cd` |

## Validation

- formatting check passed;
- focused S3 compilation passed;
- definitive-surface refusal passed with exit `2`;
- focused unit test passed (`1/1`);
- strict Clippy passed with only the six explicitly allowlisted pre-existing
  frozen/source lint classes;
- the frozen S2 evaluator, Frozen Organism v1 learning source, lifetime source,
  and S2 GATE result have no diff from parent `71d42fa`;
- final worktree must be clean before handoff.
