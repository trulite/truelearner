# FD0 phase-free local forgetting handoff v1

FD0 is development-positive.

The new forgetting candidate is local and age-based rather than globally
phased:

```text
elapsed physical time
    -> local fractional decay load
    -> resistance loss
    -> zero deallocates
```

Participation neither shields nor feeds forgetting. Traversal alone produces
no durable persistence. Absolute creation phase has no effect, stronger
resistance produces proportionally longer lifetime, host time partitioning is
exact, and local death invalidates an in-flight stale traversal.

Frozen lineage:

- protocol: `fd0-phase-free-local-forgetting-protocol-v1`;
- candidate: `fd0-phase-free-local-forgetting-candidate-v1`;
- implementation freeze: `fd0-phase-free-local-forgetting-frozen-v1`;
- positive evidence: `fd0-phase-free-local-forgetting-positive-v1`;
- development readiness: `fd0-phase-free-local-forgetting-ready-v1`.

Next allowed gate: FD1 consequence consolidation under local forgetting. RC0,
ARC, pressure-field retirement, authority, oracle, and `arch.md` remain blocked.
