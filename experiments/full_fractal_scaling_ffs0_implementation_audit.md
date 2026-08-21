# FFS0 implementation and development audit

Protocol: `full-fractal-scaling-ffs0-v1`

Status: implementation and development gates frozen; definitive scientific
matrix not run.

## Frozen references

- parent outcome tag: `re0-reflected-compaction-economics-positive`;
- parent outcome commit:
  `f24fadbdd618a825f6f960df84106acc9a0bf806`;
- FFS0 protocol tag: `ffs0-full-fractal-scaling-protocol`;
- FFS0 protocol commit:
  `4c463e2b1d090fd1aefae08dca39fb2b42dbae83`;
- FFS0 protocol SHA-256:
  `303a00febf3377f6972a2473cf618d6e91510ab4344db7f1e39c0b82ce3f2025`;
- implementation source commit before this audit:
  `a73ffac3e6bfb30c1da988a484e3e61116c90141`;
- FFS0 kernel/harness source SHA-256:
  `12e7e06a7d95d79a1b8098f99982ea792ab060b180d39b4e6c7bb7ca8cf1dbda`;
- FFS0 binary SHA-256:
  `52cfe957af4d016936781a31da24f1a0ca93505c997cfd98b09dc2bc94578f69`;
- module registry SHA-256:
  `18e6ff373a0ff20aaa3e3659269c560285658f2f3b8b54a88aef42c45e51c3f1`;
- frozen RE0 CSV SHA-256:
  `93c02acd71fc8dd642839fd31f84e18af385858efdf731a7fbce758c89c8d36b`;
- frozen RE0 Markdown SHA-256:
  `a93fc8304782d2af112fa0cf9147b961e98a696d395a3ae9f02cad073c60e0b5`.

Neither `results/ffs0_full_fractal_scaling.csv` nor
`results/ffs0_full_fractal_scaling.md` exists. Development modes cannot write
them, and definitive mode refuses to overwrite either path.

## Additive implementation boundary

FFS0 adds one new library module, one new binary, and one module declaration.
Diff inspection against the positive RE0 tag confirms zero changes to every
frozen RP0a, RG0a, RC0a, RC0b, and RE0 source body and result artifact.

The implementation does not call an earlier definitive runner or mutate an
earlier fixture. It builds an additive level-blind execution substrate from the
already-frozen local principles, then subjects that substrate to the new sparse
scaling experiment.

The module is divided by explicit source markers into organism-visible kernel
and evaluator/harness sections. Source audit is restricted to the kernel
section, so generation, scale, process availability, economics, asset cost
ownership, and report labels may exist in the evaluator without entering
learning or execution.

## Persistent-state audit

The only executable persistent type is `Arrow`:

```text
integer identity
anonymous source CELL role
anonymous target CELL role
zero or two direct dependency-arrow identities
direct dependency content fingerprints
ordered role-relative residual effects
local compatibility site and marker
strength
```

Primitive and learned arrows have the same type. Primitive arrows have zero
dependencies; every learned arrow has exactly two locally coactive direct
dependencies. A learned arrow firing is appended to the same anonymous
occurrence stream observed during the next acquisition generation.

The kernel section contains no occurrence of `level`, `meta`, `process`,
`answer`, `reflection_depth`, `task_depth`, `total_depth`, `motif_level`,
`expected_boundary`, `break_even`, `economic`, `price`, or `horizon`. It stores
no seed, episode, workload depth, identity-population size, concrete filler,
correctness result, work counter, or evaluator asset identity.

The evaluator counts dependency distance and acquisition generations only
after execution. There is no conditional execution on a reported generation.

## One queue and recursive closure

All primitive roots, learned arrows, and invalidation expansions enter one
`VecDeque<Spike>`. The queue loop performs evaluation, dependency validation,
firing, effect delivery, and fallback. There is no second learned-arrow or
recursive executor.

When a learned arrow is invalid, the same queue receives its two direct
dependency spikes. A dependency may independently expand. Normal learned
execution, one-edge fallback, two-edge fallback, and primitive execution
therefore share one physical path.

Each successful firing emits its own ordinary arrow identity into the anonymous
occurrence stream. Acquisition sees only disjoint adjacent local occurrences,
direct adjacency, ordinary effects and compatibility, and terminal
success/failure. A pair must occur at least three times in an episode and
receive `+2` credit in three separately successful episodes before strength
reaches six. Failed evidence receives `-1`; strength at or below `-2` prunes.

No work, trace-equality, break-even, scale, or generation signal enters this
credit path.

## Binding and observable audit

Persistent residual effects contain only anonymous integer roles. Every held-
out episode constructs fresh concrete filler identities and a temporary role
binding. A binding permutation changes all concrete fillers while preserving
parent/child exact trace equality. With no bindings, no concrete effects are
delivered and the missing-binding signal is set.

The frozen observable trace contains final concrete state, ordered effect-role
and filler pairs, post-effect boundary states, quiescence/error signals, and the
context ledger. Parent and child branch from the same bindings and environment.
Internal queue contents, dependency validation, and arrow occurrences are not
observable.

The forced stale control removes one ordered effect while retaining the same
final endpoint. It remains endpoint-correct but trace-wrong and is never offered
to the learner or executor.

Temporary bindings, queues, validation caches, installation sets, effects, and
boundaries are invocation-owned values. The permanent-state control confirms
that the entire arrow store is unchanged after fresh, permuted, missing-binding,
and stale evaluations.

## Parent-relative evaluator

The harness freezes the parent root before each acquisition generation. The
child is compared only with that root, never with the primitive root after the
first edge. For each edge it independently records:

```text
parent mature runtime
child mature runtime
incremental acquisition work
incremental persistent bytes
installation and maintenance
physical runtime gain
exact finite marginal break-even
```

Acquisition charges all three parent evidence executions, recurrence
observation/comparison, credit, and consolidation work. Persistent installation
is zero because consolidation creates the arrow. Mature invocation charges
temporary installation, recursive dependency validation, queue operations,
bindings, and residual-effect delivery.

The learner never sees these fields. The evaluator derives the longest
consecutive structural, justified, and realized-useful prefixes. A deeper edge
after any failed intermediate edge would remain diagnostic and could not rescue
the prefix.

The CSV runner additionally computes all five frozen carrying prices from each
edge's incremental bytes. MICRO and GATE do not serialize CSV or Markdown.

## Asset identity and transfer

Every acquisition edge receives an evaluator-only asset instance identity from
its acquisition lineage and creation ordinal. Content fingerprints depend only
on actual persistent arrow content. The hierarchy instance used in a transfer
probe is the hash of the exact constituent asset instance identities, not its
content fingerprint.

Independent anchor/probe learning uses distinct acquisition lineages even when
content is identical. The two explicit transfer probes instead borrow the exact
immutable S1 store and constituent identity set:

```text
S1 asset -> depth 128 / population 64
S1 asset -> depth 32  / population 1024
```

Both charge zero new acquisition. No reconstructed lookalike is treated as
shared ownership.

## Process availability boundary

The development source audit classifies:

```text
execution   positive
learning    unavailable
retrieval   unavailable
decision    unavailable
```

Execution is available because its real arrows use the generic queue and a
learned child replaces their work. Current learning mutation remains Rust
control flow; current retrieval lacks a replaceable anonymous executor; current
decision procedures use semantic action tokens. No trace adapter or synthetic
process event was added to fill those cells.

Thus development D is `PARTIAL`, not failed and not fully positive. A/B/C/E are
independent of unavailable process rows exactly as preregistered.

## E2B clean-snapshot validation

Persistent sandbox: `iv7qfq154p7ffq4xpxw0o`

The clean implementation commit above was validated with:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -q --lib full_fractal_scaling
cargo test --release -q --bin full_fractal_scaling_ffs0
cargo run --release --bin full_fractal_scaling_ffs0 -- --micro
cargo run --release --bin full_fractal_scaling_ffs0 -- --gate
```

The remote chain exited `0`:

- kernel/harness tests: 5 passed, 0 failed;
- binary/schema/accounting tests: 2 passed, 0 failed;
- MICRO: PASS;
- GATE: PASS;
- all-target clippy: PASS;
- formatting: PASS.

No legacy regression was repeated. The only existing tracked file changed is
the module registry's one-line additive declaration; every frozen experimental
source body and artifact is byte-identical to the RE0-positive tag. Under the
frozen regression policy, another full historical suite is required only if
shared or frozen machinery later changes.

## MICRO result

MICRO is development-only and writes no artifact:

```text
depth 8     structural / justified / realized   0 / 0 / 0
depth 32    structural / justified / realized   3 / 3 / 3

edge        parent   child   acquisition   bytes   H*   removed firings
1              144     102           586     114   14       16
2              102      65           383      58   11        8
3               65      52           236      60   19        4
```

All seven controls passed. Duplicate acquisition/evaluation, source audit,
scaling arithmetic, and orthogonal-probe arithmetic passed.

## GATE scale law

GATE uses development seed `40_000` only:

| Cell | Depth | Population | Structural | Justified | Realized | Censored |
|---|---:|---:|---:|---:|---:|---|
| S0 | 8 | 16 | 0 | 0 | 0 | no |
| S1 | 32 | 64 | 3 | 3 | 3 | no |
| S2 | 128 | 256 | 5 | 5 | 5 | no |
| S3 | 512 | 1024 | 6 | 6 | 6 | yes, `>=6` |
| depth-only | 128 | 64 | 5 | 5 | 5 | no |
| population-only | 32 | 1024 | 3 | 3 | 3 | no |

Every observed child edge was trace-exact, physically cheaper than its
immediate parent, structurally retained, and finitely repayable. There were no
over-retained or under-retained mature child snapshots. Structural, justified,
and realized depths agree in every development cell.

Zero-price marginal break-even by anchor was:

```text
S1   14, 11, 19 uses
S2   10, 10, 13, 18, 31 uses
S3   10, 10, 12, 16, 23, 39 uses
```

S3 is right-censored: recurrence remains after the sixth promotion. No claim
about its uncensored stopping depth is made.

The depth-only probe reproduced S2 depth five despite the S1 population. The
population-only probe reproduced S1 depth three despite the S3 population.
Development therefore attributes the observed depth change to reusable program
length rather than identity-population size.

## Transfer and adaptation

Exact S1 instance reuse produced:

```text
depth-only transfer       576 -> 148 work   acquisition 0   trace exact
population-only transfer  144 ->  52 work   acquisition 0   trace exact
```

S2 adaptive arms produced:

| Arm | Trace | Fallback distance | Recovery work | Reacquisition |
|---|---|---:|---:|---:|
| stable | exact | 0 | 424 | 0 |
| child-own change | exact | 1 | 560 | 0 |
| direct-parent change | exact | 2 | 848 | 0 |
| return | exact | 0 | 424 | 0 |

Return reused the same historical asset instance and unchanged content
fingerprint. No hierarchy manager or ordinal fallback operation participated.

## Development claim statuses

Development is non-claim evidence only:

```text
A functional recursion       PASS
B computational recursion    PASS
C economic recursion         PASS
D cross-process closure      PARTIAL
E adaptive recursion         PASS
scaling trend                PASS
orthogonal depth signature   PASS
```

The definitive scientific statuses remain unspent.

## Frozen status

```text
RP0a -> RE0   positive ancestry                         frozen
FFS0 protocol                                             frozen
FFS0 implementation + development gates                  frozen
FFS0 definitive A/B/C/D/E outcomes                       pending
```

No FFS0 definitive command was executed while preparing this audit. The next
claim-eligible action is the single frozen `--definitive` run after this audit
and implementation are committed and tagged.
