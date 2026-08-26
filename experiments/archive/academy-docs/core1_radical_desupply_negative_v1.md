# CORE1 radical de-supply — immutable negative v1

## Frozen lineage

- Protocol: `b798fa4`, tag `core1-radical-desupply-protocol-v1`
- Candidate: `2e19792067053ce4ff99ba424c7d94982e9c6260`, tag
  `core1-radical-desupply-frozen-v1`
- Branch: `research/core1-radical-desupply`

The sole evidence execution emitted
`CORE1_RADICAL_DESUPPLY_V1_EVIDENCE_SPENT` once and exited with status 101.
It was not rerun.

## Observed prefix

The execution log establishes the following prefix facts:

| Profile | Completed gates before stop | Next observed gate |
|---|---|---|
| CORE1-A | E0–E11 | E12 ran; E13 was not reached |
| CORE1-B | E0–E11 | E12 ran; E13 was not reached |
| CORE1-C | E0–E11 | E12 ran; E13 was not reached |
| CORE1-D | E0 | E1 panicked during the first Reference execution |
| CORE1-E | none | not reached because the process terminated in CORE1-D/E1 |

Because the frozen evaluator advances to the next gate only after a PASS,
absence of E13 for A–C proves that each first failed at E12. The exact failed
E12 subpredicate was not printed before the later process termination.

CORE1-D/E1 terminated at `PlasticSubstrate::require_cell` with:

```text
cell must be live in this substrate
```

The candidate therefore did not establish the complete five-profile matrix.
E13, E14, and the whole CORE1-E profile have no result.

## Classification

There are two distinct findings.

1. **Developmental negative:** CORE1-A, B, and C all clear propagation through
   stable recurrence (E0–E11) but do not clear fixed consequence-matured
   recurrence stabilization (E12). This is the first wall for all three.
2. **Runtime/evaluator integration negative:** CORE1-D's unbounded,
   distance-graded, every-firing variation can leave the fixed E1 source dead
   before the evaluator's next supplied pulse. A physical runtime must reject
   an arrival at a dead junction without process panic, and a destructive
   ablation evaluator must serialize each completed row before advancing.

Code inspection suggests the D failure geometry: unbounded proposals create
far-future arrivals, physical time advances through their long delays, weak
topology decays, and J0 orphan removal can reclaim the source before the next
fixed E1 pulse. This explanation is an inference, not a separately executed
diagnostic result.

## Publication limitation

The evaluator accumulated rows in memory and wrote `matrix.csv`/`report.md`
only after all 75 rows. Consequently the later panic prevented publication of
the completed A–C and D/E0 rows. No matrix artifact exists, and no values are
reconstructed or relabeled here.

## Scientific boundary

CORE1 does **not** establish that radical de-supply reaches contextual action
learning or ARC A2. It does establish a strong partial boundary:

```text
CORE1-A/B/C
E0 propagation through E11 stable recurrence   observed positive prefix
E12 learned recurrence stabilization           first negative

CORE1-D
E0 propagation                                 observed positive
E1 relation formation                          fatal negative

CORE1-E                                        unobserved
```

No authority, oracle, `arch.md`, FD2, or ARC claim advances from this run.

