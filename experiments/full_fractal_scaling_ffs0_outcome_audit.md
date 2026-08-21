# FFS0 definitive outcome audit

Protocol: `full-fractal-scaling-ffs0-v1`

Outcome: A, B, C, and E positive; D partial. The definitive sparse matrix
supports functional, computational, economic, and adaptive recursion over
substrate-native execution. Cross-process closure remains unavailable for
learning, retrieval, and decision.

## Frozen execution boundary

- parent outcome tag: `re0-reflected-compaction-economics-positive`;
- parent outcome commit:
  `f24fadbdd618a825f6f960df84106acc9a0bf806`;
- FFS0 protocol tag: `ffs0-full-fractal-scaling-protocol`;
- FFS0 protocol commit:
  `4c463e2b1d090fd1aefae08dca39fb2b42dbae83`;
- FFS0 implementation tag: `ffs0-full-fractal-scaling-implementation`;
- frozen implementation commit:
  `647c404723cc31dc9623a3e99888ecddabf3e85f`;
- implementation source commit:
  `a73ffac3e6bfb30c1da988a484e3e61116c90141`;
- persistent E2B sandbox: `iv7qfq154p7ffq4xpxw0o`.

Before execution, HEAD exactly matched the implementation tag, the worktree was
clean, and neither FFS0 result path existed. The following claim-eligible
command was executed once:

```text
cargo run --release --bin full_fractal_scaling_ffs0 -- --definitive
```

The E2B runner compiled the frozen checkout, executed the complete matrix,
downloaded both write-once artifacts, and exited zero. No second definitive
FFS0 command was executed. No unavailable process cell was filled and no
source or frozen ancestor was changed before execution.

## Frozen artifacts and ancestry

- `results/ffs0_full_fractal_scaling.csv`
  SHA-256:
  `74f0a92442aa71a60cedf047feeadbebe389586210959b758a7d2cf6fd43db56`;
- `results/ffs0_full_fractal_scaling.md`
  SHA-256:
  `58d2c1efc124bdb481b43317ed2f373926d0da308b41778de0aacd1a79f3e0c2`.

The frozen inputs retained their preregistered hashes:

- FFS0 protocol:
  `303a00febf3377f6972a2473cf618d6e91510ab4344db7f1e39c0b82ce3f2025`;
- kernel/harness source:
  `12e7e06a7d95d79a1b8098f99982ea792ab060b180d39b4e6c7bb7ca8cf1dbda`;
- definitive binary source:
  `52cfe957af4d016936781a31da24f1a0ca93505c997cfd98b09dc2bc94578f69`;
- module registry:
  `18e6ff373a0ff20aaa3e3659269c560285658f2f3b8b54a88aef42c45e51c3f1`;
- parent RE0 CSV:
  `93c02acd71fc8dd642839fd31f84e18af385858efdf731a7fbce758c89c8d36b`;
- parent RE0 Markdown:
  `a93fc8304782d2af112fa0cf9147b961e98a696d395a3ae9f02cad073c60e0b5`.

Git ancestry from the positive RE0 outcome to the frozen FFS0 implementation
is intact. The implementation diff is additive: one kernel module, one binary,
one module declaration, the protocol, and the pre-definitive implementation
audit.

## Independent artifact audit

The CSV has one consistent 39-column schema and exactly 1,046 data rows:

```text
scale rows        48
edge rows        880
transfer rows     16
adaptive rows     32
process rows       4
control rows      56
claim rows          5
audit rows          5
```

An independent read-only audit parsed every row and recomputed all 880 priced
edge gains and exact ceiling break-even horizons from serialized inputs:

```text
gain_micros =
    (parent_work - child_work - maintenance_work) * 1,000,000
  - incremental_bytes * price_micros

H* = ceil(
    (acquisition_work + installation_work) * 1,000,000
    / gain_micros
)
```

All recomputed values match. The five carrying prices each contain exactly 176
rows. Every horizon is finite throughout the frozen price matrix and spans
`10..39` uses. At the highest price, 16 of 176 edges move upward by one use;
no edge changes qualitative status.

The 880 priced rows represent 176 distinct learned asset instances and six
repeated content fingerprints. Thus independently acquired structural
lookalikes remain separately owned while identical content is recognized as
such. Every five-price group preserves one instance identity and fingerprint.

## Endogenous scale law

Every one of the eight definitive seeds produced the same sparse scale result:

| Cell | Workload depth | Identities | Proposed | Functional / computational / economic / retained | Structural / justified / realized depth | Censored |
|---|---:|---:|---:|---:|---:|---|
| S0 | 8 | 16 | 0 | 0 | 0 | no |
| S1 | 32 | 64 | 4 | 3 | 3 | no |
| S2 | 128 | 256 | 6 | 5 | 5 | no |
| S3 | 512 | 1,024 | 7 | 6 | 6 | yes |
| depth-only | 128 | 64 | 6 | 5 | 5 | no |
| population-only | 32 | 1,024 | 4 | 3 | 3 | no |

Across all 48 seed/scale rows:

```text
structural depth = economically justified depth = realized useful depth
over-retained children = 0
under-retained useful children = 0
retention precision = recall = agreement = 1.0
```

The orthogonal probes distinguish computational depth from identity population:
depth 128 retains five levels at both 64 and 256 identities, while depth 32
retains three levels at both 64 and 1,024 identities. The supported trend is
therefore tied to reusable computational depth in this matrix, not merely to
population size.

S3 is right-censored. Its result is `>=6`, not an estimate that the natural
hierarchy terminates at six.

## Immediate-parent computational and economic tests

At zero carrying price, the per-seed parent-relative chains are:

```text
S1  144 -> 102 -> 65 -> 52
S2  576 -> 342 -> 209 -> 148 -> 119 -> 106
S3  2304 -> 1302 -> 785 -> 532 -> 407 -> 346 -> 317
```

Every arrow is an independently tested immediate-parent edge. No deeper level
is compared directly with L0 and no profitable deeper level subsidizes an
intermediate edge. Every edge preserves the complete observable trace, has
strictly lower mature work, removes ordinary arrow firings, and reaches finite
marginal break-even.

The end-to-end mature reductions are diagnostic consequences of those local
edges, not substitutes for them:

| Cell | Root work | Deepest observed mature work | Reduction |
|---|---:|---:|---:|
| S1 | 144 | 52 | 63.889% |
| S2 | 576 | 106 | 81.597% |
| S3 | 2,304 | 317 | 86.241% |

Across the primary S1/S2/S3 matrix, 5,248 ordinary arrow firings disappear.
The claimed computational result is therefore physical lower execution, not an
accounting-only change.

## Shared asset transfer

All 16 transfer rows pass. Within each seed, the depth-only and population-only
probes use the exact same frozen S1 hierarchy instance and content fingerprint:

```text
S1 asset -> depth 128 / population 64:  576 -> 148 work
S1 asset -> depth 32  / population 1024: 144 -> 52 work
```

Both preserve the observable trace and charge zero new acquisition. The result
demonstrates reuse of an actual persistent asset, not evaluator-side merging of
separately learned lookalikes.

## Adaptive recursion

All 32 adaptive rows preserve exact observable behavior. For every seed:

| Context | Fallback distance | Recovery work | Reacquisition |
|---|---:|---:|---:|
| stable | 0 | 1,696 | 0 |
| child-own change | 1 | 2,240 | 0 |
| direct-parent change | 2 | 3,392 | 0 |
| historical return | 0 | 1,696 | 0 |

The changed contexts expose the nearest compatible dependency path. Historical
return reuses the original asset without reacquisition. This supports local
graph-relative fallback rather than a level-numbered hierarchy manager.

## Process boundary, controls, and audits

The cross-process result remains deliberately partial:

```text
execution   positive
learning    unavailable
retrieval   unavailable
decision    unavailable
```

Execution is substrate-native and closes under the shared kernel. Current
learning mutation remains opaque Rust control flow, retrieval lacks a
replaceable anonymous executor, and decision uses semantic action tokens. No
adapter or synthetic event class was introduced to convert those unavailable
cells into positives.

All seven controls pass in all eight seeds, for 56/56 passes:

```text
subthreshold evidence does not consolidate
failed evidence prunes
shuffled adjacency does not consolidate
changed bindings remain exact
bindings remain necessary
same-endpoint stale effects fail trace equality
temporary state is erased
```

Frozen ancestry, duplicate determinism, the level-blind source audit, the
scaling trend, and the orthogonal depth signature all report PASS.

## Independent claim outcomes

```text
A  functional recursion       PASS
B  computational recursion    PASS
C  economic recursion         PASS
D  cross-process closure      PARTIAL
E  adaptive recursion         PASS
```

The supported narrow claim is:

> A level-blind developmental kernel recursively organized substrate-native
> execution into an endogenous hierarchy whose retained depth increased with
> reusable workload depth. Every claimed immediate-parent promotion preserved
> the preregistered observable behavior, reduced physical work, and repaid its
> marginal acquisition and carrying cost; local invalidation exposed the
> nearest valid dependency path and historical context return required no
> reacquisition.

Under the tested matrix, positive C plus exact retention agreement supports the
phrase that the organism selected its own useful execution-abstraction depth.
This does not establish cross-process closure, an uncensored natural depth at
S3, unbounded recursive improvement, or generality beyond the frozen synthetic
workloads and accounting.

## Ladder status

```text
RP0a   one-level functional fractality               positive, frozen
RG0a   abstract-to-concrete grounding                 positive, frozen
RC0a   reflected interpreter tax removed              positive, frozen
RC0b   lower computation eliminated; runtime wins     positive, frozen
RE0    full acquisition and persistence amortize      positive, frozen
FFS0   recursive execution scaling A/B/C/E             positive, frozen
       cross-process closure D                         partial
```

No process-specific learner, level-specific executor, or additional fractal
primitive was added.
