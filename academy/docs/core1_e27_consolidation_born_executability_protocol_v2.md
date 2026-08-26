# CORE1-E27 — Consolidation-Born Executability Protocol v2

## Status

Preregistered after the v1 harness was invalidated and before v2 runtime
evidence. Candidate physics from `8ec7ae7` is frozen unchanged.

## Harness correction

The ARC regimen requires five distinct spatial contexts. v1 nevertheless
constructed all 1,024 replicated context compartments for every execution.
v2 constructs exactly five context compartments and selects one raster for
each local context index `0..4`.

This is a fixture-size correction, not a physical shortcut:

- every context retains four candidate sources, traces, relays, babblers,
  motors, outputs, and all frozen scaffold arrows;
- cell thresholds, positions within each context/motor pair, arrow delays,
  materials, resistances, variation, E24, G+W, E22, PQLC, E26, recovery, and
  observation semantics are unchanged;
- context identifiers carry no semantics and the five compartments are
  structurally replicated;
- the candidate receives no selected route, action, consequence, or probe
  information.

The v2 evaluator must complete its whole matrix in seconds. Runtime is reported
as a harness property, not a biological acceptance predicate.

## Tournament

Run two arms in the same five-context fixture:

```text
B  frozen E26 stack, E27 disabled
X  identical stack, E27 enabled
```

Both arms use roots `95000000..95000007`, Reference, exact Reference replay,
and Production.

### Arm B — fixture-preserved baseline

Require the frozen causal chain:

```text
teaching actions       1|4|2|3
later Modulatory       >0
later PQLC             >0
E26 re-entry counts    0|1|2|3|4
E27 executable count   0 throughout
E22 returns            1|1|1|1|0
autonomous probes      none|none|none|none
natural quiescence     true
```

Arm B must reproduce E26 at `8/8`. Otherwise the compressed fixture is not
accepted and Arm X is not interpreted.

### Arm X — candidate solve

Require the same teaching/credit chain plus:

```text
E27 executable counts       0|2|4|6|8
autonomous probes           1|4|2|3
executable traversals       >=2 per probe
natural quiescence          true
```

Arm X must pass `8/8`, with exact replay and mechanics.

## Interpretation

- B fails: harness equivalence fails; v2 is invalid and X is uninterpretable.
- B passes, X fails: E27 candidate physics is falsified in the valid compact
  fixture.
- B passes, X passes: consolidation-born local threshold closure earns
  candidate CORE1 physics.

No post-evidence repair, predicate relaxation, or rerun is permitted.
