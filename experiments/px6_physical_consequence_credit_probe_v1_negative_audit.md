# PX6 physical consequence-credit PROBE v1 negative audit

Status: **FROZEN `1/6` EVALUATOR NEGATIVE; PHYSICAL ARTIFACTS IMMUTABLE**.

The write-once PROBE ran once from implementation commit
`701e19013a9ef7e8da5d50cd928e528a71177b16`. Its artifacts are preserved
unchanged:

- CSV SHA-256: `cb07d2077b7ac0f8af6534339006be4e6887c211370eb2cc2039dbd74d69e6f9`;
- report SHA-256: `daf9a1681e9014678a7d224e16c7c13ac4734a5ba4a4d7a01a6138e426af3d6b`.

## Observed physical result

All six retained-arrow vectors and held-out outward vectors matched the frozen
hypothesis:

```text
left             live true|false   held-out 1|0
right            live false|true   held-out 0|1
both             live true|true    held-out 1|1
correlation      live false|false  held-out 0|0
crossed-return   live false|false  held-out 0|0
no-return        live false|false  held-out 0|0
```

Every duplicate was exact and every development quiesced. The physical
discriminator therefore did not expose a missing mechanism or scientific
ambiguity.

## Exact evaluator collapse

Five rows failed one or both of two evaluator predicates:

1. `WorkLedger.local_return_updates` is a substrate-wide work counter. An
   arrival at an active source changes every eligible outgoing arrow, including
   stable fixture arrows. It cannot equal the number of candidate-arrow
   resistance changes. Candidate resistance and liveness were already
   serialized directly and were exact.
2. In crossed-return and no-return, the reserve-3 participating candidate
   traversed twice and then lawfully deallocated under ordinary pressure.
   Downstream firing on that side consequently occurred twice, not eight
   times. Requiring continued downstream execution after physical
   deallocation contradicts the authoritative PX2 finite-opportunity boundary.

Neither defect is organism-visible. No physical input, topology, timing,
threshold, coupling, resistance, law, or expected retained vector needs to
change.

## Disposition

Freeze v1 without rescue or rerun. A fresh-namespace PROBE v2 may change only
the two invalid predicates: treat the global update counter as accounting
rather than candidate attribution, and require a positive measured number of
participating-side downstream firings before lawful deallocation in the two
no-own-return controls. Every other clause remains exact.
