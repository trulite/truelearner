# PX4 physical learned-lifetime PROBE v2 implementation audit

Status: **MECHANICAL RETRY IMPLEMENTATION FROZEN BEFORE EXECUTION**.

## Frozen basis

- immutable v1 negative commit:
  `709d9ba86a961f8560928928b5a0ffeb6001a12a`;
- v2 protocol commit:
  `b555c65116b7a27e177c911d08ed1018b6341cd6`;
- v2 protocol SHA-256:
  `fb0e67c8c8e1e37789a4f51e081df975bcbb738856bc26f5acd866ade5490e98`;
- v2 harness SHA-256:
  `801944549876a6cc0a828cec5d9590b0df6d5700bd999621bfe4d2df231299cc`;
- unchanged active substrate law SHA-256:
  `3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d`.

## Exact delta

The harness advances ordinary substrate time to an already-fixed first-use
tick before introducing each fresh reserve-3 direction opportunity. The first
possible physical traversal remains at that same tick. All pressure after
introduction remains unchanged.

Both information-flow scan lists now assemble completed forbidden tokens from
split fragments, preventing the scan specification from matching itself.

PROBE uses fresh v2 namespaces and write-once v2 result paths. MICRO and GATE
namespaces remain fresh and unspent.

No substrate source, resistance increment, pressure decrement, eligibility
window, coupling update, deallocation condition, reproposal rule, threshold,
activity history, matched later gap, or outcome predicate changed.

## Pre-execution validation

The following passed without spending a PROBE v2 cell:

```text
cargo fmt --all -- --check
cargo check -p px0-physical-correspondence --example px4_physical_lifetime
cargo clippy -p px0-physical-correspondence --example px4_physical_lifetime -- -D warnings
git diff --check
```
