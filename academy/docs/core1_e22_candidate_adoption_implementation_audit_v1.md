# CORE1-E22 — Candidate Adoption Implementation Audit v1

## Scope

The positive E22 primitive is promoted only through the ordinary CORE1 Academy
integration. Relative to positive result commit `7fa0c27`:

- `truelearner/crates/core/src/lib.rs` is byte-identical;
- the unchanged E14 evaluator is byte-identical;
- only `academy/crates/academy-arc3/src/sensorimotor.rs` changes in runtime
  source.

## Adoption seam

The Academy body now registers its already-existing returning CELL with the
unchanged atomic-return core mechanism during CORE1 construction and enables
that mechanism by default. Ordinary action admission brackets physical
propagation with the existing atomic capture flag. An admitted ordinary
consequence traverses the topology before the Academy boundary clears it.

No E22 contact predicate, edge shape, local retention rule, propagation rule,
PQLC rule, or topology cleanup implementation changed. E20 USED-PENDING
capture/protection is not enabled. No OPEN, completion, refractory, or
variation mechanism is added to ordinary observation.

## Static checks

- formatting: passed;
- `git diff --check`: passed;
- strict release Clippy for unchanged E14 evaluator: passed;
- E22 core diff from `7fa0c27`: empty;
- runtime-source diff: `21` insertions, `2` deletions in the Academy seam;
- E14 evaluator execution before evidence: none.

SHA-256:

- unchanged E22 core:
  `231d1ba35482bb88d8998448a9e1d631e30508d1329262105c408a098ab6892c`;
- Academy integration before adoption:
  `bc1c337cebe721fc225111db0e9febee1b917695e599e11c5ac8f6a62b7c1692`;
- candidate Academy integration:
  `59df0992839283263fdb7b5a1ecb5e3ed3d91ecf26524920dd46a4244d8fdf7c`;
- unchanged E14 evaluator:
  `1c2f144a3bd3b660bb3f213ce6d13bcc44aeaee13ff3de81378f95b9f2b32858`.
