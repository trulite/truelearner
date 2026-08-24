# S1.1: Compiled Search Value

## Question

Can the frozen 92-entry S0 value model become cheap enough to consult that
guided search costs less than rediscovery?

S1.1 changes neither learned values nor search order. It replaces repeated
structural reconstruction with an incrementally maintained compact signature.

## Equivalent Evaluators

All evaluators return identical high, medium, or low values:

1. Current evaluator: reconstruct the canonical structural key and consult the
   original table.
2. Compiled evaluator: update a compact signature and directly retrieve value.
3. Local activation: activate one of 92 signature cells, then follow its fixed
   arrow to one of three value cells.
4. Zero-cost bound: preserve the same ordering while charging no value cost.

The compiled table contains the same 92 learned entries. Local activation uses
95 cells and 92 arrows.

## Correctness

```text
Value comparisons                 46,944
Value mismatches                       0
Signature collisions                  0
Incremental signature mismatches      0
Convergent paths checked           9,795
Search-order mismatches                0
```

Different action paths reaching the same role-relative state produce the same
signature. States receiving different original values never alias.

## Reachable Planning Work

```text
Neutral exhaustive search       1,255,048
Current S0 evaluator            1,357,960
Compiled direct lookup          1,196,296
Local activation                1,236,712
Zero-cost ordering bound          630,472
```

The search expansions and chosen plans are identical for the current,
compiled, local, and zero-cost evaluators. Only value-evaluation cost changes.

Compiled direct lookup accounts for:

```text
Signature update work             525,408
Value retrieval work               40,416
```

Local activation accounts for:

```text
Signature update work             525,408
Activation spikes                  80,832
```

## Compilation Economics

```text
                              Direct      Local
One-time compilation work      1,932      2,116
Saving per reachable problem   1,468        458
Break-even problems                2          5
```

## Unreachable Planning

```text
Neutral exhaustive search       1,848,672
Current S0 evaluator            3,973,536
Compiled direct lookup          3,501,344
Local activation                3,619,392
```

Completeness requires exhausting every candidate when no plan exists.
Guidance therefore remains overhead on proof-of-absence problems.

## Interpretation

The previous S1 economic failure came primarily from repeatedly reconstructing
the structural situation in which learned value applied. Maintaining and
activating a compact signature makes reuse cheaper than neutral rediscovery on
reachable problems.

This is deterministic work accounting, not a measured hardware speedup. Role
structure, signature compilation, learned values, supplied search, and the
cost model remain engineered.

