# FFS-SAME1 compiled correspondence outcome audit

Protocol: `identity-desupply-ladder-v1/ffs-same1`

Outcome: **positive recursive compatibility and whole-stack compaction**.
Process availability remains independently **PARTIAL**.

## Frozen execution

The single definitive command was executed once from implementation tag
`ffs-same1-compiled-correspondence-implementation` at commit
`39e35b425c6907968a3427b7bd71a1b27222025e`:

```text
cargo run --release --bin ffs_same1_compiled_correspondence -- --definitive
```

Execution occurred in persistent E2B sandbox `iv7qfq154p7ffq4xpxw0o`.
The sandbox was left running. No source, threshold, matrix, reference, or
accounting field changed after the implementation freeze.

## Write-once artifacts

- definitive CSV:
  `results/ffs_same1_compiled_correspondence.csv`;
- CSV SHA-256:
  `7883f71918d48c4c622d7cd2d9dd7561f5954f7287f8bc6abb535f5a9f994a55`;
- definitive Markdown:
  `results/ffs_same1_compiled_correspondence.md`;
- Markdown SHA-256:
  `a788106462498dd7581fcbd324d6fbc71a1ca0a46c3390a4d289ae180731edad`.

The CSV contains 447 lines with a fixed 48-column schema and zero malformed
rows:

| Row type | Count |
|---|---:|
| scale | 48 |
| parent-relative edge | 176 |
| adaptive | 32 |
| control | 184 |
| independent claim | 6 |

The source runner computed transfer rows in memory as part of E1 but did not
serialize them as a separate CSV row type. Their exact equality, lower work,
zero acquisition charge, and same-instance reuse are conjuncts of the frozen
E1 result. The write-once artifacts are preserved unchanged; this audit does
not reconstruct or add post-hoc transfer measurements.

## A1 — functional hierarchy preservation: PASS

Every seed `0..7` reproduced the preregistered scale law:

| Scale | Depth | Population | Structural | Justified | Realized |
|---|---:|---:|---:|---:|---:|
| S0 | 8 | 16 | 0 | 0 | 0 |
| S1 | 32 | 64 | 3 | 3 | 3 |
| S2 | 128 | 256 | 5 | 5 | 5 |
| S3 | 512 | 1,024 | >=6 | >=6 | >=6 |
| depth-only | 128 | 64 | 5 | 5 | 5 |
| population-only | 32 | 1,024 | 3 | 3 | 3 |

S3 is right-censored at the frozen six-promotion ceiling. All 176 claimed
edges preserved their immediate parent's complete observable trace. There was
zero over-retention and zero under-retention in all 48 scale cells.

Therefore the CS0a path changed execution cost without changing the hierarchy
that emerged.

## B1 — parent-relative computational recursion: PASS

All 176 claimed consecutive edges were physically cheaper than their
immediate retained parent. Across the definitive matrix they removed 6,464
arrow firings. No deeper edge was compared directly with or subsidized by L0.

Relative to the frozen FFS-SAME0 edge artifact, the compiled correspondence
path reduced both parent and child mature evaluation by exactly 12 work per
invocation for every edge. It also reduced recursive acquisition work by
exactly 36 per edge because each frozen acquisition observes three executions:

```text
3 * (18 generic identity work - 6 compiled identity work) = 36
```

Across 176 measured recursive acquisitions this removed 6,336 work without
changing candidate formation, credit, consolidation, retained assets, or
observables.

## C1 — parent-relative economic recursion: PASS

Every claimed edge was exact, computationally useful, structurally retained,
and independently finitely repayable against its immediate parent. Physical
break-even ranged from 10 to 40 uses. No economic value or break-even result
was organism-visible.

The retained/justified/realized prefix was identical for every scale and seed.
Thus no profitable deeper edge rescued an unprofitable intermediate edge.

## D1 — mature identity-tax reduction: PASS

Every one of the 48 scale cells reproduced:

```text
FFS-SAME0 learned correspondence tax     18 work/use
FFS-SAME1 compiled correspondence tax     6 work/use
reduction                                12 work/use
                                        66.67%
```

The full scale runtimes were:

| Scale | FFS-SAME0 learned | FFS-SAME1 compiled | supplied SAME | SAME1 premium vs supplied |
|---|---:|---:|---:|---:|
| S0 | 54 | 42 | 36 | +6 |
| S1 | 70 | 58 | 52 | +6 |
| S2 | 124 | 112 | 106 | +6 |
| S3 | 335 | 323 | 317 | +6 |
| depth-only | 124 | 112 | 106 | +6 |
| population-only | 70 | 58 | 52 | +6 |

Across the 48 evaluator scale cells, mature compilation removed 576 measured
work relative to generic FFS-SAME0 and retained a 288-work aggregate premium
relative to supplied SAME. Per invocation, the premium was exactly +6 at
every depth, identity population, recursive depth, and seed. It did not grow
with population or hierarchy depth.

The residual remained the frozen CS0a attribution:

| Component | Work/use |
|---|---:|
| compiled local activation | 1 |
| context/support/dependency validation | 3 |
| ambiguity handling | 1 |
| temporary installation and binding | 1 |
| **total** | **6** |

This establishes whole-stack compaction but not parity with free supplied
SAME. IP0 remains responsible for the final prior-economics classification.

## E1 — adaptive invalidation and historical reuse: PASS

All 32 adaptive rows passed:

| Arm | Fallback distance | Reacquisition |
|---|---:|---:|
| stable | 0 | 0 |
| child-own change | 1 | 0 |
| direct-parent change | 2 | 0 |
| historical return | 0 | 0 |

Historical assets were reused on return. All 184 controls passed, including
fresh occurrence transfer, occurrence relabeling, allocation and memory-order
perturbation, changed bindings, subthreshold and shuffled evidence, stale
dependency invalidation, generic reopening, historical compatibility,
persistent-state stability, and the complete inherited SAME0 leak suite.

The source audit, deterministic duplicate audit, ancestry audit, scale trend,
and orthogonal depth/population signature all passed.

## P1 — process availability: PARTIAL

Execution is positive. Learning, retrieval, and decision remain `UNAVAILABLE`
because their actual computation does not yet naturally execute through the
frozen anonymous interface. No adapter or semantic event class was added.
This result is non-blocking and does not weaken A1-E1.

## Narrow claims

FFS-SAME1 establishes:

> A learned replacement for supplied filler equality underwent ordinary
> consolidation into role-relative local structure and remained compatible
> with recursive acquisition, execution, parent-relative compaction,
> economic retention, invalidation, and historical reuse throughout the
> level-blind fractal kernel.

It also establishes:

> Compilation reduced the mature learned-identity tax from 18 to 6 work per
> use at every tested scale without changing the endogenous hierarchy. The
> remaining six-work premium over supplied SAME was fixed rather than growing
> with identity population or recursive depth.

It does not establish that supplied SAME has zero economic value. The frozen
identity branch now opens IP0, which adds no capability and classifies that
value using the frozen FFS0, FFS-SAME0, CS0a, CS0b-skip, and FFS-SAME1
artifacts.
