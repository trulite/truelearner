# CORE1-E23 — Differential Audit Implementation v1

## Scope audit

E23 adds observation only:

- `Arc3ContextDiagnostic` exposes the already-existing context-trace and
  babbler CELL IDs;
- `Arc3Sensorimotor` retains a clone of the `physical_trace` already returned
  by the most recent ordinary action admission and exposes it read-only;
- the E23 evaluator reconstructs the frozen E14 first turn and frozen E16
  seed-0 first participation, then reduces their diagnostics/traces to the five
  preregistered stage predicates.

The trace clone is outside the substrate and canonical body. It is not read by
propagation, selection, topology, learning, cleanup, snapshots, fingerprints,
or later action admissions.

## Conformance

- CORE1 runtime diff from `bc74c7c`: empty;
- Academy diff: `19` instrumentation-only insertions;
- E14 evaluator: byte-identical;
- E16 evaluator: byte-identical;
- E23 `--check`: passed without executing either compared action admission;
- strict release Clippy: passed;
- formatting and `git diff --check`: passed.

SHA-256:

- CORE1 runtime:
  `231d1ba35482bb88d8998448a9e1d631e30508d1329262105c408a098ab6892c`;
- Academy integration with trace exposure:
  `56613fa25667434655759524bba523bcfbb0ed3f01868f4d489f9edf2f07d23a`;
- unchanged E14 evaluator:
  `1c2f144a3bd3b660bb3f213ce6d13bcc44aeaee13ff3de81378f95b9f2b32858`;
- unchanged E16 evaluator:
  `08b50cacdcc05f2f5721de0267e8449fddaceafe844e6dbc6c5ea9b0077f2912`;
- E23 evaluator:
  `9b8d5edc9d76e1bfac8437d60db7f882d8cfb71d2c1b21b16d5a55aa688ccfbd`;
- protocol:
  `8302123f72a439a349e2b711459a1e297ffe59c9fd4845c0e1ef3accd12b6586`.
