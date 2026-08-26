# CV0 bounded local contact genesis handoff v1

Status: Gate D static negative; stopped before organism implementation.

## Outcome

```text
SV1
separate contact compartments are required
→ accepted variation cannot create them

CV0 candidate
one fresh ordinary contact CELL per signed candidate relation

Gate D
unsupported candidate ARROWs can die and reuse slots
unsupported contact CELLs cannot die or reuse slots
→ repeated contact genesis would grow CELL occupancy monotonically
→ STOP NEGATIVE
```

This is not an inhibitory-learning or signed-selection negative. Those worlds
were never constructed. It is evidence that ordinary CELL lifetime is missing
from the accepted substrate.

## Frozen lineage

- SV1 parent/result: `a65c445`, tag
  `sv1-compartmentalized-signed-variation-gate-a-negative-v1`;
- CV0 protocol: `ac4fe20`, tag
  `cv0-bounded-local-contact-genesis-protocol-v1`;
- source-sentinel correction: `492f5e9`;
- successful fresh E2B evidence: `iv52x0ubcrh6htz58uztw`;
- executable organism changes: zero;
- Rust compilation, tests, evaluator, and runtime execution: none.

## Boundary

Do not implement contact genesis by adding garbage collection, orphan
detection, reference counts, degree checks, a special contact lifetime, or an
evaluator cleanup step. Those would supply the Gate D answer.

A successor may independently ask whether ordinary CELLs can have a general
physical lifetime and generation-safe reuse law analogous in discipline—but
not necessarily identical—to ordinary ARROW persistence. Only after that law
is independently earned may CV0 be rerun unchanged, followed by frozen SV1 and
then frozen RS2 Gate B.

FD2, ARC, RS2 Gate B, authority, the oracle, and `arch.md` remain unchanged.
