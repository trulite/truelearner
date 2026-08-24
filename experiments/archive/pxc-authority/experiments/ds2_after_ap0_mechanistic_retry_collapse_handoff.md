# Cumulative DS2 retry after AP0: collapse handoff

Outcome: **CUMULATIVE DS2 DEVELOPMENT COLLAPSE AT STAGE 6**:

> contrasting interactions differentiate supported and reverse/noncausal
> candidates

M1 remains authoritative. M2 does not exist. This retry is development-only,
claim-ineligible, and produced no definitive/result artifact.

## Ordered result

Stages 0--5 passed:

1. exact M1, prior-retry, AP0, readiness, and protocol fingerprints match;
2. learner-visible causal/source/target annotation remains absent;
3. the exact M1 interaction and learned boundary-role path remains intact;
4. frozen DS1 selection physically actuates its existing A1 route;
5. aftermath is contingent on the resulting physical route state;
6. that aftermath activates the existing A1 proposal/probation machinery.

Stage 6 collapsed. Across GATE seeds 100--104:

```text
selected-route proposals          20
selected-route support updates    20
alternate-route proposals         20
alternate-route support updates   20

downstream-consequence -> probation edges   0
consolidation/pruning edges                 0
semantic/causal adapters                    0
```

Thus AP0 has made plasticity physically reachable, but the existing mechanism
supports either executed alternative equally. It receives no ordinary physical
signal that distinguishes the downstream-supported direction from its reverse
or a noncausal neighbor. Consolidation, transfer, invalidation, and M2 remain
blocked.

## Frozen lineage

- Exact enabling parent: `830d80c3c925a3acf1be8026e9dd8cbe520c763e`
- Protocol: `f6d8116da9f331e79b2b28a92a0334318af4b2c3`
- Validated implementation: `5b54bad042c88c647f93190f0378b62aafab31f9`
- Authoritative M1: `16a1002b59bf0dbc23a6b6bf03572efca53b33ce`
- Frozen prior retry SHA-256: `da05e976dc43ceb5f14fdbb56928207d0fdc99fb52a5d8d630ced588c26d4224`
- Frozen AP0 SHA-256: `a33019958327b145bdb14f4386f628e2c4fd5fcca94e413736513f8b86cf78f5`
- Retry source SHA-256: `ce0e253ae43136ce7396bbbc237baf6490fa904067a007504fa26bfdbc87044a`
- Runner SHA-256: `9bc5b50ba4c3841caea3d709c4a361ee9138d8658b790252d527833942c76cd3`

## Validation

Local and exact-implementation-commit E2B validation passed:

- formatting;
- strict release Clippy;
- 64 focused release tests;
- release probe;
- definitive refusal with exit status 2.

Persistent E2B sandbox `ivwzv2hvyrfg6ep1r1usl` uses dedicated state
`/Users/satya/.cache/truelearner/ds2-after-ap0-retry-e2b.json` and was left
running.

## Scientific boundary

The next missing capability is not proposal formation. It is a physical,
nonsemantic path by which contrasting downstream consequences differentially
support or weaken already-probationary local structures. No such path was added
or tested here.
