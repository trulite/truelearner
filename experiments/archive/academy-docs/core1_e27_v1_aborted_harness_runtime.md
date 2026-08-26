# CORE1-E27 v1 — Aborted Harness Run

## Status

**INVALID / ABORTED.** This run is neither a pass nor a falsification of E27.

The v1 evaluator started once and emitted its evidence-spent marker. Its
executability gate completed `8/8` with exact Reference replay and Production:

```text
teaching actions        1|4
Modulatory              0|1
PQLC                    0|2
E26 re-entry            0|1
E27 executable edges    0|2
autonomous probe        1
executable traversals   2
quiescent               true
```

The subsequent full matrix was manually stopped after more than eighteen
minutes at the user's direction. It had not produced `full.csv` or a terminal
result.

## Harness defect

Each execution constructed the complete 1,024-context spatial body although
the regimen used only five contexts. Reference mechanics then repeatedly
global-scanned that body. The evaluator scheduled 24 gate executions followed
by 24 full executions, turning a tiny causal test into a long CPU-bound harness
run.

The process was active at approximately 99% CPU and was not trapped in a
non-quiescent physical episode. The defect was unnecessary replicated fixture
surface and repeated global scanning.

## Disposition

- process terminated with exit code `130`;
- partial uncommitted result directory deleted;
- no v1 result commit or tag created;
- completed gate rows retained only in this diagnostic summary;
- v1 evidence is not rerun;
- successor protocol must use a minimal physically isomorphic context fixture,
  include the frozen E26 baseline in that same fixture, and complete in seconds.

