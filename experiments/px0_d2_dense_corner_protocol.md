# PX0-D2 dense-corner diagnostic protocol

Status: **DEVELOPMENT PREREGISTRATION; NO D2 EVIDENCE SPENT; NO MECHANISM CHANGE AUTHORIZED**.

## Frozen starting point

PX0 definitive v1 and v2 remain immutable failures. PX0 authority remains
absent. D2 begins exactly at v2 negative handoff commit
`e3cdd9cc67f07149283ff82d39ce573e95eb31c3`.

The active PX0 law must remain byte-identical at SHA-256
`3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

D2 may add evaluator-side observation and serialization only. It may not add,
remove, strengthen, weaken, select, label, or otherwise change any active
CELL/ARROW/SPIKE mechanism or learned structure.

## Frozen v2 boundary

V2 failed only `P6` in cells 7 and 15. Both contemporary B routes nevertheless
executed exactly once. P6 bundled three clauses that were not separately
serialized:

```text
A. completed stable returns == scheduled stable-return contexts
B. stable resistance > sparse resistance
C. contemporary B held-out effect == 1
```

The v2 artifact establishes C and leaves A/B unresolved. D2 does not recreate
either spent cell or execute the v2 authority command.

## Research question

> Near the failed dense/slow corner, does physical scheduling merely prevent
> every offered return from completing, or does stable-return specificity itself
> lose its resistance and reusable-behavior separation?

## Fresh nearby matrix

D2 uses 256 fresh blank deterministic cells. Cell namespaces begin at
`0x14000000 + i * 0x80000`; none overlap v2 or development namespaces.

The matrix crosses:

- spacing: `13, 15, 17, 19`;
- route stride: `24, 25, 27, 28` (the spent value `26` is excluded);
- dense distractor load: `32, 36, 44, 48` (the spent value `40` is excluded);
- allocation/layout: all four normal/reverse × direct/mirrored combinations;
- incidental phase: a fixed balanced rotation over `0,1,2,3` derived from the
  other matrix coordinates;
- stable/sparse route identities rotated across all three routes;
- support-device order and arrival order permuted deterministically per cell.

No cell reproduces the full physical parameter vector of v2 cell 7 or 15.

## Exact physical history

Each blank cell executes the same causal prefix relevant to P6:

1. acquire route A through four ordinary return experiences;
2. confirm one held-out A execution;
3. preserve/reuse A across bounded absence;
4. fully deallocate original A under ordinary pressure;
5. attempt stale A once, producing fresh weak proposals but zero crossing;
6. enter the changed B world;
7. run eight interleaved four-context cycles where B has a scheduled support
   opportunity in every context and A has a genuine dense return opportunity
   in one phase per cycle;
8. run a three-context tail without A's dense driver;
9. evaluate final B and A from exact cloned physical state;
10. measure resistance and no-use deallocation delays from separate clones.

The learner receives no opportunity counter, context index, reliability label,
or diagnostic measurement.

## Required serialized observables

For every context, serialize separately:

- stable return opportunity (`0/1`);
- stable returns physically completed in that context and cumulatively;
- sparse return opportunity (`0/1`);
- sparse returns physically completed in that context and cumulatively;
- stable maximum live resistance;
- sparse maximum live resistance;
- stable and sparse live variable-arrow counts;
- cloned held-out B effect at that point;
- cloned held-out A effect at that point;
- first context where B becomes executable;
- cumulative contexts where B is executable;
- context deallocations and first deallocation context;
- queue comparisons and total work.

For every final cell, serialize:

- all physical parameters;
- scheduled and completed stable/sparse returns;
- final resistance separation;
- final B and A effects;
- first B execution and B-executable context count;
- stable and sparse no-use deallocation delays;
- total proposals, deallocations, queue work, and total work;
- complete and permanent fingerprints;
- deterministic duplicate equality and natural quiescence.

No independently meaningful clause may remain hidden inside one boolean.

## Classification

- **D2-A — accounting-only boundary:** at least one fresh cell completes fewer
  stable returns than opportunities, while every cell retains B executability,
  sparse-A silence, greater stable resistance, and longer stable no-use
  lifetime.
- **D2-B — resistance-separation boundary:** B executes in at least one cell
  where stable resistance is not greater than sparse resistance or stable
  no-use lifetime is not longer.
- **D2-C — specificity breakdown:** any adequately experienced cell ends with
  B non-executable, sparse A executable, or sparse structure surviving at least
  as robustly as stable structure in a way not explained solely by the measured
  accounting gap.
- **D2-D — no nearby boundary:** every stable opportunity completes and all
  resistance, lifetime, and behavior separations hold in the tested nearby
  matrix.
- **D2-E — scientific ambiguity:** the serialized observables do not uniquely
  separate accounting, resistance, and behavioral specificity.

Classification is descriptive development evidence. No outcome rescues v2 or
authorizes a v3 authority matrix automatically.

## Controls and integrity

- exact complete-state duplicate replay in every cell;
- exact fresh namespaces and parameter ledger;
- direct/mirrored and normal/reverse transfer;
- all three route identities;
- natural quiescence for every propagation;
- active-law, retained-physics, v1/v2 result, PX0-P1, and PX0-S hashes exact;
- zero normal dependencies and semantic-source isolation;
- atomic report, cell CSV, and trajectory CSV;
- development-only refusal of every definitive flag.

## Stopping rule

Freeze the first complete 256-cell diagnostic result exactly. Do not tune,
rerun, or reinterpret it. A failure of instrumentation before any cell may be
repaired only in a separately frozen implementation. Once cell zero begins,
the matrix is write-once development evidence.

PX1, PX-C, the continuous organism, Harness H1, and any PX0 v3 authority matrix
remain blocked after D2 pending explicit review of the classification.
