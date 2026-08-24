# PX-C seam taxonomy protocol v2

Status: preregistered audit protocol. It supplements but never rewrites the
frozen v1 baseline of 368 occurrences across 295 source lines.

## Purpose

Functional gates answer whether behavior survives physicalization. PX-C
answers whether the typed scaffold actually disappeared. A lane is not ready
merely because old names vanished: the semantic supply must become ordinary
physical dynamics rather than move into an adapter, evaluator input, or new
control surface.

## Primary taxonomy

Every occurrence in the headline seam inventory receives exactly one primary
kind. The primary counts therefore sum exactly to `TOTAL_OCCURRENCES`.

| primary kind | source evidence |
|---|---|
| `typed_representation` | episode, history, or query representation |
| `explicit_mechanism_invocation` | direct call into a frozen cognitive mechanism namespace |
| `episode_reset_boundary` | explicit episode-start/reset boundary |
| `seed_history_synthesis` | seed-built event/history/route construction |
| `semantic_condition` | productive/contrast/outcome construction used as a condition |
| `manual_temporary_cleanup` | explicit temporary-state erase |
| `typed_handoff` | frozen or typed inter-layer transfer |
| `evaluator_derived_input` | fixture/session/episode constructor supplied by the evaluator |

This mapping is classification only. It does not delete or merge the frozen
v1 inventory rows.

## Independent relocation guards

Two high-specificity scans run over every manifested active source, including
lines that did not match the original v1 patterns:

- semantic-condition guard: outcome, correctness, productive/contrast,
  terminal/answer, reconstruction, and pass/fail vocabulary;
- evaluator-input guard: protocol/seed/fixture, held-out construction,
  harness mode, report/snapshot/control, and acquisition vocabulary.

Guard counts may overlap each other and do not alter the frozen headline seam
total. Their purpose is to detect a bad reduction in which an old seam is
renamed or relocated into a new semantic adapter.

## Lane-readiness procedure

After a PX lane reaches development readiness:

1. replace only that lane's predecessor entries in a new versioned
   active-surface manifest;
2. prove by source/dependency audit that the manifest covers the complete
   candidate mechanism surface and separately identify evaluator-only code;
3. run the taxonomy audit in E2B with the exact expected manifest hash;
4. apply ceilings from the previous serial checkpoint to headline total,
   semantic guard, and evaluator-input guard;
5. serialize total, primary-kind counts, guard counts, layer counts, and every
   source occurrence;
6. freeze the result without advancing authority.

A lane is rejected as a physicalization reduction if any ceiling increases,
even when its functional gate passes. Any justified surface expansion requires
a new protocol version before execution; it may not be explained after seeing
the result.

## Immutable reference

The following v1 artifacts remain byte-identical throughout the factory:

- `results/pxc_continuous_seam_baseline_v1.md`
- `results/pxc_continuous_seam_inventory_v1.csv`
- `results/pxc_continuous_seam_summary_v1.csv`
- tag `pxc-continuous-seam-baseline-v1`

PX-C success requires the serially authoritative active surface to reach:

```text
headline seams             0
semantic-condition guard   0
evaluator-input guard      0
```

while the functional authority chain remains intact.
