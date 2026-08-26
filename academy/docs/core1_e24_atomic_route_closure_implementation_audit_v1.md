# CORE1-E24 — Atomic Route Closure Implementation Audit v1

## Candidate surface

E24 adds one default-off CORE1 flag. When enabled, newly proposed generic
direct candidates are appended to the same source-firing event's existing
subdivision loop. The loop itself is unchanged: it creates one contact, one
positive stem, and one sign-preserving outgoing arrow with the existing
material, resistance, position, target, and delays.

The disabled path retains the prior candidate set. No proposal input, target
ordering, sign set, pressure, decay, propagation, credit, or PQLC code changes.

Academy adds one method that enables the flag. The E24 evaluator alone calls
it. Candidate-default CORE1 and the byte-identical E14/E16 evaluators do not.

## Static conformance

- E14 evaluator SHA-256 remains
  `1c2f144a3bd3b660bb3f213ce6d13bcc44aeaee13ff3de81378f95b9f2b32858`;
- E16 evaluator SHA-256 remains
  `08b50cacdcc05f2f5721de0267e8449fddaceafe844e6dbc6c5ea9b0077f2912`;
- E24 check mode passed without executing Gate 1;
- strict release Clippy passed;
- formatting and `git diff --check` passed.

SHA-256:

- E24 core candidate:
  `d52798ab3eb23aa2c3507b5fdf678fe31a81ee77c3ccfa966adb9fc1e0b7c449`;
- Academy enable surface:
  `4fb80ce765033cba3ce6e44615fc40834d20d0fcc0a3e222383dca1c61cd3693`;
- E24 evaluator:
  `9377feb73eff8f2150b9385317d3e232f6d7498a01c3b2a5bfd415605e706fd1`;
- protocol:
  `314df605a6f1395e1b99d4849d7f36730080055f5e934e7041dcfa03caa93482`.
