# LR-C definitive v1 namespace-contamination collapse

Status: **V1 AUTHORITY INVALID; RESULT RETAINED AS DEVELOPMENT POSITIVE**.

Post-result audit found that `registered_matrix_passes_in_development` and
`definitive_serialization_has_exact_shape` executed the exact sixteen seeds,
31 worlds and 496 namespaces later used by `--definitive`.

The definitive command itself ran once from the frozen implementation and its
artifacts are internally exact. However, the registered authority cells were
not epistemically untouched at evidence time. This violates the intended fresh
authority boundary even though no state was persisted between executions.

Therefore:

- tag `lrc-qualified-modulatory-definitive-positive-v1` is historical only;
- v1 remains a strong implementation/development positive;
- v1 does **not** make LR-C authoritative;
- the v1 result and handoff remain immutable but are superseded by this
  collapse; and
- a v2 protocol must use disjoint development and authority namespaces.

V2 tests may execute only registered development seeds. Preflight may compute
authority namespace cardinality and hashes but may not call the physical cell
runner with any authority seed. Only the sole frozen `--definitive-v2` command
may execute v2 authority cells.
