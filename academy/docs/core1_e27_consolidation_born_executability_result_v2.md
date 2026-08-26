# CORE1-E27 — Consolidation-Born Executability Result v2

## Status

**PASSED 8/8.** The compact fixture first reproduced frozen E26 at `8/8`, then
E27 produced the useful autonomous action sequence at `8/8`. Reference, exact
Reference replay, and Production agreed throughout.

The complete 48-execution matrix finished in `2055 ms` internally (`2.08 s`
wall), replacing the invalid v1 harness that was stopped after eighteen
minutes.

## Arm B — E26 equivalence

Every root and mechanics execution produced:

```text
teaching actions       1|4|2|3
Modulatory             0|1|1|1|1
PQLC updates           0|2|2|2|2
E26 re-entry topology  0|1|2|3|4
E27 executable edges   0|0|0|0|0
E22 returns            1|1|1|1|0
USED-PENDING           0|0|0|0|0
autonomous probes      none|none|none|none
executable traversals  0|0|0|0
natural quiescence     true
```

The five-context fixture therefore preserves the exact E26 causal result. Its
smaller replicated surface changes harness cost, not the tested behavior.

## Arm X — E27 solve

Every root and mechanics execution produced:

```text
teaching actions       1|4|2|3
Modulatory             0|1|1|1|1
PQLC updates           0|2|2|2|2
E26 re-entry topology  0|1|2|3|4
E27 executable edges   0|2|4|6|8
E22 returns            1|1|1|1|0
USED-PENDING           0|0|0|0|0
autonomous probes      1|4|2|3
executable traversals  2|2|2|2
natural quiescence     true
```

Thus later source incidence physically traversed exactly the two consolidated
route halves and produced the learned motor action with no babble, consequence,
recall operation, or policy lookup.

## Interpretation

E27 earns **candidate CORE1 physics**:

> Successful PQLC on a complete used route consolidates enough signed local
> efficacy for ordinary source activity to execute that route through each
> target's existing threshold.

The successful decomposition is now:

```text
route formation                 E24
motor competition/integration   E25 G+W
delayed consequence return      E22
credit                          existing PQLC
learned topology re-entry       E26
learned route executability     E27
```

The candidate stores no action or episode. Consolidation uses only the actual
participating endpoints, existing material sign, and each local target's
physical threshold. Later execution is the ordinary `SourceFires` propagation
path.

This result does not adopt E27 globally. Adoption still requires historical
controls and an explicit CORE1 promotion decision.

## Harness disposition

- E27 v1: invalid/aborted, no result commit or tag;
- v1 partial directory: deleted;
- v2 off-matrix two-arm smoke: `62 ms` internal;
- v2 frozen 48-execution matrix: `2055 ms` internal, `2.08 s` wall;
- v2 evidence marker emitted once; no repair or rerun.

## Evidence

- `experiments/results/core1_e27_consolidation_born_executability_v2/matrix.csv`
- `experiments/results/core1_e27_consolidation_born_executability_v2/report.md`

SHA-256:

- matrix:
  `a049b058523fb7ed4c800518d95724bdab4c62ba5aa25179bd1083de9997fb46`;
- generated report:
  `41e32e6d4520dafb8e671ad78ad86e1f95abcba7a9cd21979786931cf3bbc03c`.
