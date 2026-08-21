# RP0b definitive outcome audit

Status: **technical negative**. The single definitive E2B run failed the
ordered RP0b.1 physical-runtime gate. RP0b.2 amortization was therefore not
evaluated.

## Result

- RP0a reconstruction parity: `8/8` exact frozen endpoints.
- Runtime behavior: all `144/144` arm/seed/depth rows were correct, explicit,
  naturally quiescent, fallback-free, routing-equivalent, deterministic, and
  fingerprint-preserving.
- Frozen learned state: ten roles and exactly four correct arrows per seed,
  `113648339` total acquisition work, zero additional installation work, and
  `52864` total permanent bytes.
- Technical comparison: REFLECTED exceeded CONCRETE in all `48/48`
  seed/depth cells. Per-cell overhead ranged from `2704` to `3088` work.
- Aggregate reflected overhead was `23296` work at every tested depth:

| Depth | Concrete runtime | Reflected runtime | Delta |
|---:|---:|---:|---:|
| 5 | 27648 | 50944 | +23296 |
| 8 | 44928 | 68224 | +23296 |
| 16 | 102272 | 125568 | +23296 |
| 32 | 266112 | 289408 | +23296 |
| 64 | 790400 | 813696 | +23296 |
| 128 | 2625408 | 2648704 | +23296 |

- ORACLE ENTRY equaled CONCRETE at every depth. The reflected program did not
  remove lower execution work; it preserved that work and added anonymous
  provenance processing, role recognition, temporary binding, learned-arrow
  evaluation, and target resolution.
- Economics rows: `0`, as required after the technical failure.
- Workspace lifecycle: `1150205/1150205` destroyed; maximum live `2`.

The CSV has `161` rows and `51` columns with a consistent shape and exactly one
row for each of the `144` runtime arm/seed/depth keys.

Artifact hashes:

- CSV: `10e52f45b24f595268112f027e14581655e3ec5d42363dca53f2a7bdc9edb0b3`
- generated report: `68261b1159e3989405d1a0cea18efb57f1829e803e32dfd8546ed4e52567e478`

## Interpretation

The reflected overhead is approximately fixed per 16-query batch, so its
fractional penalty shrinks with chain depth. That diagnostic scaling does not
constitute a runtime win: the absolute delta remains positive, and every
preregistered technical cell fails the strict `REFLECTED < CONCRETE` boundary.
No reuse horizon or carrying price can reverse a positive per-use runtime
delta, so stopping before RP0b.2 is substantive rather than procedural.

The result preserves the RP0a functional positive while rejecting economic
usefulness for this reflected level:

> The same learning physics can close over substrate-native internal
> computation, but invoking the resulting reflected program does not make the
> frozen computation cheaper. It reproduces the lower execution and adds
> reflected recognition and routing work.

## Boundary audit

The implementation commit was frozen before the definitive run. The run used
the preregistered eight RP0a seeds and depths `5, 8, 16, 32, 64, 128` exactly
once on the persistent E2B sandbox. The RP0a protocol, trajectory, generated
report, and audit are unchanged from
`rp0a-reflected-program-discovery-positive`. No learner capability, semantic
adapter, evaluator jump, index, shortcut, new route, or second reflection level
was introduced.

The full pre-run E2B validation passed `154` library tests with `2` ignored,
all target-specific suites, formatting, Clippy with warnings denied, focused
accounting tests, and explicit reconstruction parity. The sandbox was left
running after the definitive run.

## Frozen claim and limits

Supported:

> RP0a remains a one-level functional-fractality positive, but RP0b is a
> technical economics negative: the learned reflected program preserved
> behavior while adding runtime work at every tested depth.

This result does not evaluate RP0b.2 acquisition/carrying break-even, recursive
F1, arbitrary compiler discovery, or d2.6 retrieval substitution. F1 remains
blocked because the first reflected level did not establish a computational
reason to persist.
