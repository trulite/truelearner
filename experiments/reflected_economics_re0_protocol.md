# RE0 preregistration: reflected compaction economics

Protocol identifier: `reflected-compaction-economics-re0-v1`

Status: frozen before RE0 implementation, acquisition measurement, or any
definitive RE0 run.

## Question and boundary

RE0 asks only:

> Does the one shared learned asset frozen by RP0a, RC0a, and RC0b repay its
> acquisition and carrying cost through repeated mature use?

RE0 adds no capability. The RP0a learner, RC0a dispatch, RC0b motif,
consolidation laws, invocation path, temporary bindings, validation,
installation, fallback, lower executor, observable interface, and every runtime
counter remain frozen.

The ordered scientific facts entering RE0 are:

```text
RC0b-A  motif runtime < FULL RC0a runtime       positive
RC0b-B  balanced mature motif < concrete       positive
RE0     acquisition/carrying amortization       question here
```

No price or horizon may change a mature runtime sign. Depths whose motif
runtime is not below concrete are preregistered negative controls.

## Frozen ancestry and data

- RC0b positive tag: `rc0b-grounded-motif-substitution-positive`
- RC0b positive commit:
  `f6dcec833966347ce360f3ec126202b4843319d5`
- RC0b implementation tag: `rc0b-grounded-motif-substitution-implementation`
- RC0b implementation commit:
  `d50faa66a0c06258cbcde792103b3f9d7522e521`
- RC0b protocol tag: `rc0b-grounded-motif-substitution-protocol`
- RC0b protocol commit:
  `4ae4378eb5df95c9f1e829db58f3c7ba5c341b9b`
- frozen RC0b implementation source SHA-256:
  `6a21c62e406e68d47a442839938b457505cd195bbb891929f2b9b6c7f44e1987`
- RC0b definitive CSV SHA-256:
  `285ee87a7a77ea26b154cb728c63b1a53530891ae3ffd370e990dbd38e93f97e`
- RC0b definitive Markdown SHA-256:
  `fb3fa0bb7b351c4ddc77a8e83fe60f42ad29d57a64fc1edfbbbcc34f24018b72`
- RC0b outcome audit SHA-256:
  `fe4dcb18e8fe6c54778d61a743ed7d7e7f3090b4502dac618f207988435b80a2`

The frozen CSV's known extra unnamed gate-status field is irrelevant to RE0:
RE0 consumes only its 528 valid named result rows. It must validate all result
dimensions, claim flags, behavior fields, and totals before accounting.

## One shared learned asset

The primary accounting unit is one seed's complete learned one-level stack:

```text
RP0a learned roles and reflected program
    + RC0a compiled dispatch
    + RC0b one learned motif
```

The same persistent motif is acquired once and reused at every frozen depth.
It contains no depth field, query, answer, episode, concrete identity, or
horizon. RE0 must not charge a separate acquisition per depth.

Three acquisition views are reported without changing the primary gate:

1. **full stack from blank** — RP0a + RC0a + RC0b; primary;
2. **compiled-plus-motif incremental** — RC0a + RC0b when RP0a is already
   owned; secondary diagnostic;
3. **motif-only incremental** — RC0b when RC0a is already owned; secondary
   diagnostic.

The full-stack view determines the claim of economically useful one-level
fractality. Incremental views may not rescue a full-stack failure.

## Acquisition-only observer

The RC0b artifact did not serialize its acquisition rows. RE0 may therefore
append one read-only acquisition-only audit accessor to the RC0b source. That
accessor must:

- call the unchanged frozen seed reconstruction, RC0a acquisition, and RC0b
  motif-acquisition functions;
- use the exact frozen definitive seed and acquisition domains;
- return only parity, layer-separated work, persistent bytes, motif count,
  shortcut count, motif fingerprint, and workspace lifecycle;
- execute no held-out RC0b runtime cell and write no RC0b artifact;
- add no branch, feedback, candidate, route, executor, or counter;
- leave every pre-existing function body and type unchanged.

The definitive accessor is run twice per seed and must return identical values.
Only the first copy is charged; the duplicate is measurement validation, not
organism work. Seeds may run in parallel; each acquisition remains
single-threaded and deterministic.

Because this observer touches a frozen source file, one compatibility
regression is required before the RE0 implementation is frozen. No further
legacy regression is allowed unless shared/frozen code changes again.

## Physical quantities

For seed `s` and depth `d`:

- `A_rp0a(s)` — frozen observed RP0a acquisition work;
- `A_rc0a(s)` — observed work earning the compiled dispatch;
- `A_rc0b(s)` — observed work earning the one motif, including the ordinary
  RC0a executions through which it is learned;
- `A_full(s) = A_rp0a + A_rc0a + A_rc0b`;
- `I_persistent(s)` — additional persistent installation work after
  acquisition;
- `S_rp0a`, `S_rc0a`, `S_rc0b` — retained bytes by layer;
- `S_full = S_rp0a + S_rc0a + S_rc0b`;
- `M_use(d)` — observed permanent maintenance work per mature use;
- `W_concrete(s,d)` — frozen concrete runtime per invocation;
- `W_motif(s,d)` — frozen mature RC0b runtime per invocation.

Consolidation already performs persistent installation, so
`I_persistent = 0` unless an actually executed post-acquisition persistent
operation is observed. It may not be fabricated.

RC0b mature runtime already charges per-invocation topology validation,
binding reads, temporary compiled-route installation, motif compatibility,
temporary shortcut installation, shortcut evaluation/firing, residual effects,
and cleanup. None of those may be charged again as `I` or `M_use`.

Frozen RC0b mature evaluation is read-only and its permanent fingerprints do
not change. Therefore `M_use = 0` unless the unchanged acquisition observer or
frozen artifact supplies a real maintenance event. Storage bytes remain a
physical quantity; they are not themselves work.

## Exact break-even arithmetic

Let carrying price be `p = q / 1,000,000` work units per retained byte per
invocation, using integer `q`. Define per-use net saving in millionths:

```text
gain_µ(s,d,q) =
    (W_concrete(s,d) - W_motif(s,d) - M_use(d)) * 1,000,000
  - S_full(s) * q
```

For the full-stack primary view, finite break-even exists only when
`gain_µ > 0`. Then:

```text
H*(s,d,q) = ceil(
    (A_full(s) + I_persistent(s)) * 1,000,000
    / gain_µ(s,d,q)
)
```

All break-even calculations use exact integer arithmetic. Floating-point
rounding may be used only for display.

Frozen carrying-price numerators:

```text
q = 0, 1, 10, 100, 1000
p = 0, 0.000001, 0.00001, 0.0001, 0.001
```

Frozen reporting horizons:

```text
1, 2, 4, 8, 16, 32, 64, 128, 256, 1024,
10000, 100000, 1000000, 10000000
```

Prices and horizons are secondary scenarios. The primary physical result is
the zero-price break-even in equivalent work units.

## Exclusive-depth phase diagram

For each seed, charge one full acquisition to a deployment whose future uses
all have one depth.

- Depths `5, 8, 16` are negative controls. Since frozen mature motif work is
  above concrete, they must have no finite break-even at every nonnegative
  price.
- Depths `32, 64, 128` are finite-break-even candidates. RE0's primary
  exclusive-depth gate requires a finite zero-price full-stack `H*` for every
  seed at every one of these depths.

Report all seed/depth/price break-even values and all frozen-horizon delta
costs. No mean may hide a seed failure.

## Shared cross-depth reuse

The same motif is also evaluated as one asset serving a balanced recurring
workload. One workload cycle contains exactly one invocation at each frozen
depth:

```text
5, 8, 16, 32, 64, 128
```

Acquisition is charged once per seed, not six times. Carrying is charged once
per invocation, hence six times per cycle. Let cycle gain be the sum of the six
depth gains. Report exact break-even in cycles and in invocations
(`6 * cycles`).

The primary shared-asset gate requires finite zero-price full-stack break-even
for every seed. Incremental and priced results remain secondary diagnostics.

## Three-speed harness

### MICRO

- development only; never claim eligible;
- synthetic integer fixtures only;
- exercise positive, zero, and negative denominators, ceiling arithmetic,
  carrying-price conversion, and mixed-depth sharing;
- no frozen definitive acquisition seed is reconstructed.

### GATE

- development only; never claim eligible;
- one evaluator-constructed RC0b development fixture;
- one acquisition measurement and duplicate comparison;
- parse and validate the already-frozen RC0b runtime artifact;
- exercise every depth, price, horizon, acquisition view, and output row type;
- no frozen definitive acquisition seed is reconstructed.

### DEFINITIVE

- acquisition-only reconstruction for frozen seeds `0..7`, twice for
  determinism;
- consume the frozen RC0b runtime artifact without executing any runtime cell;
- compute the complete exact phase diagram once;
- serialize once after all accounting and gates complete;
- execute the definitive RE0 command exactly once.

Development modes write no result artifacts and establish no RE0 claim.

## Conjunctive gate

RE0 passes only if:

1. every frozen ancestry commit and artifact hash matches;
2. all 528 RC0b result rows validate, with eight seeds, six depths, eleven
   arms, 16 episodes per cell, and positive RC0b-A/RC0b-B flags;
3. all eight acquisition-only reconstructions reproduce frozen RP0a parity,
   one complete RC0a dispatch, exactly one RC0b motif with three shortcuts,
   stable role-relative fingerprints, and destroyed workspaces;
4. duplicate acquisition measurement is exact;
5. layer acquisition work and bytes are positive, installation is not double
   counted, and observed mature maintenance is zero;
6. the same motif fingerprint/structure for each seed is charged once and
   reused across all depths;
7. depths `5, 8, 16` have no finite break-even at any frozen nonnegative price;
8. depths `32, 64, 128` have finite zero-price full-stack break-even for every
   seed;
9. the balanced shared-depth workload has finite zero-price full-stack
   break-even for every seed;
10. exact break-even and horizon deltas reconcile independently;
11. no RC0b runtime is rerun and source inspection finds no learner, motif,
    route, executor, work-counter, or runtime-behavior change.

All gates are conjunctive. Prices above zero are reported but do not determine
the primary pass once the zero-price conditions are met.

## Outcome boundary

A positive RE0 supports:

> The depth-general one-level learned reflective asset repaid its complete
> observed acquisition cost after finite reuse under physical zero-carrying
> accounting, while preserving the preregistered depth phase boundary.

It may additionally be called economically useful one-level fractality. This
does not establish a second reflected level, indefinite recursive improvement,
or benefit at shallow depths.

If RE0 is negative, F1 remains blocked. If RE0 is positive, F1 becomes
scientifically permitted but is not implemented here.

Expected write-once artifacts:

- `results/re0_reflected_economics.csv`
- `results/re0_reflected_economics.md`

No RE0 result may modify the frozen RC0b artifacts.
